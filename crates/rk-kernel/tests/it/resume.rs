//! **`rk_kernel::replay::resume`** — §7d.1 edit (6), §7d.5 step (9), RT-55. Step 7d Slice 3.
//!
//! # 🛑 Why this module is not in [`crate::replay`], which is where the run-record tests live
//!
//! `crates/rk-kernel/tests/it/replay.rs` carries `#![cfg(feature = "corpus")]` over the **whole
//! module**, under its own header explaining why. Checkpoint 7d's `rk-kernel` loop invokes
//! `rk_test rk-kernel "$t" "…"` and that helper is `cargo test -p "$1" ${4:-} -- "$2"` with **no
//! fourth argument** on the kernel loop — so a test placed there is not compiled in the run the
//! checkpoint makes, and the block fails with *"no test named … ran and passed"*. Nothing here
//! needs the corpus or a real git, so the coupling is avoided rather than worked around.
//!
//! # What the fixture is, and why it is a new one
//!
//! `tests/fixtures/packs/resume-gate.toml`, whose header carries the reasoning: the one selection
//! on this tree that is **both gated and dispatchable**, which is what makes the store-seeded fold
//! observable as a *different action* rather than as the same action twice.
//!
//! # 🛑 RAISED, NOT REPAIRED — the spec supplier cannot make a node dispatchable on its own
//!
//! §7d.5 step (9) gives `resume` a `supply_spec` seam "consulted where `record()` calls
//! `spec_for`", and this module proves it is consulted. It does **not** prove a supplier-served node
//! reaches the harness, and it cannot, because of a coupling no record names:
//! `RunInputs::activations()` is derived from the **same** `specs` list `spec_for` reads, so a node
//! absent from `specs` also has no activation; `fsm::next_action` then falls back to the DAG node's
//! bare `activity_id` (its own comment calls this "the ARMED state, not a default"), and
//! `workflow::idem::checked_key` refuses a bare id **for want of an `@version`** one layer down.
//! The `activity_id@version` join is recorded as a decision this stage may not take, so a fixture
//! whose catalog ids embedded `@1` would be this test taking it. The seam's own red is measured
//! here — `record` refuses on `inputs/specs.json` where `resume` does not — and the coupling is
//! reported rather than designed around.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_kernel::replay::{self, RecordedSpec, ReplayError, RunInputs, RuntimeContext};
use rk_kernel::runtime::journal_query;
use rk_kernel::select::Item;
use rk_kernel::workflow::fsm::{self, FsmContext, NextAction};
use rk_ports::event_store::EventStore;
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::root::Ports;
use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::typed::{SchemaId, Typed};

const PACK: &str = include_str!("../fixtures/packs/resume-gate.toml");
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

const CATALOG: &[(&str, &str)] = &[
    (
        "impl-report.toml",
        include_str!("../fixtures/catalog-select/impl-report.toml"),
    ),
    (
        "change-summary.toml",
        include_str!("../fixtures/catalog-select/change-summary.toml"),
    ),
    (
        "change-summary-ue.toml",
        include_str!("../fixtures/catalog-select/change-summary-ue.toml"),
    ),
    (
        "changeset.toml",
        include_str!("../fixtures/catalog-select/changeset.toml"),
    ),
    (
        "spec-authoring.toml",
        include_str!("../fixtures/catalog-select/spec-authoring.toml"),
    ),
];

const ITEM: &str = "INC-1";
const STAGE: &str = "implement";

/// Supplied and recorded, never read from a clock or from git — [`crate::replay`]'s rule, and the
/// reason is the same here: a value this module derived would make two runs of identical inputs
/// produce different bytes.
const TREE_HASH: &str = "9f2c1ab4d6e80357bb41c0d9e2f7a85643c1908df5b2e7461ac30d98f5b1e2c7";
const DEADLINE: i64 = 1_760_000_000;
const RESULTS_AT: i64 = 1_759_900_000;
const RECORDED_SCHEMA: &str = "RecordedResult/v1";

/// The gate class the fixture's abstaining `require` row mints. **The full kind, axis and all** —
/// `fsm::next_action` spells it `format!("{}:{}", gate.reason.as_str(), gate.axis)`, and a policy
/// keyed on the bare `unknown_facet` would match no gate this system opens.
const GATE_KIND: &str = "unknown_facet:external_dep";

/// The two DAG nodes the fixture's baseline assembles, in the plan's own order.
const BOUND: [(&str, &str); 2] = [
    ("implement-change[node]", "implement-change@1"),
    ("impl-report", "impl-report@1"),
];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// A distinct, derived card digest per node — three bindings sharing one digest would let a spec
/// mixed up between two nodes reach the port unnoticed ([`crate::replay`]'s reason, re-used).
fn spec_for(node: &str) -> ActivitySpec {
    ActivitySpec {
        card_digest: Digest::from_hex(rk_types::digest::sha256(node.as_bytes()).as_str())
            .expect("64 lowercase hex characters"),
        tools: vec!["Read".to_owned()],
        schema: SchemaId::new(RECORDED_SCHEMA),
        timeout_secs: 900,
        inputs: vec![("item".to_owned(), serde_json::json!(ITEM))],
    }
}

/// The input set, bound for exactly the nodes named. `bind_all` is the ordinary case; the partial
/// case is conjunct (C)'s, where the absent binding is what routes a node to the seam.
fn inputs(bind: &[(&str, &str)]) -> RunInputs {
    built(bind, Abstains::Yes)
}

/// The same input set with the `require` row DECIDED rather than abstaining, so no gate opens and
/// the very first action is a dispatch.
///
/// It exists for one job: (C)'s control needs a run that actually REACHES `spec_for`, and a gated
/// run breaks at the gate before it reaches any binding at all — so `record` over the gated inputs
/// refuses nothing and the conjunct would measure nothing. The two differ in exactly one facet
/// value, which is what keeps the control a control.
fn ungated(bind: &[(&str, &str)]) -> RunInputs {
    built(bind, Abstains::No)
}

/// `Yes` reads the facet as `null` and opens the gate; `No` decides it `false` and opens none.
#[derive(Clone, Copy)]
enum Abstains {
    Yes,
    No,
}

fn built(bind: &[(&str, &str)], abstains: Abstains) -> RunInputs {
    let facets_src = std::fs::read_to_string(workspace_root().join("spikes/p9/facets.toml"))
        .expect("the shipped facet registry is readable");
    let item = match abstains {
        // ADR-0089: the absence of evidence never resolves to `false`, and a null read at step 2 is
        // what mints the gate.
        Abstains::Yes => Item::new(ITEM, "incident")
            .with_toolchain("node", ["text"])
            .with_facet_null("external_dep"),
        Abstains::No => Item::new(ITEM, "incident")
            .with_toolchain("node", ["text"])
            .with_facet_bool("external_dep", false),
    };
    let mut inputs = RunInputs::new(STAGE)
        .with_runtime(
            RuntimeContext::supplied(TREE_HASH, DEADLINE)
                .entered_from("triage")
                .advances_to("verify")
                // §7d.5's one clock read, as a RECORDED value. `derive` and `resume` both call
                // `require_results_at`, which refuses rather than defaulting — "a zero `at` is a
                // commit stamped at the epoch".
                .results_at(RESULTS_AT),
        )
        .with_pack("resume-gate.toml", PACK)
        .with_catalog(CATALOG)
        .with_artifacts("artifacts.toml", ARTIFACTS)
        .with_facets(&facets_src)
        .with_item(&item);
    for (node, activation) in bind {
        inputs = inputs.with_spec(node, RecordedSpec::supplied(activation, spec_for(node)));
    }
    inputs
}

/// A store pre-loaded with `prior`, as the CLI's is: §7d.5 step (9) hands `resume` the store's own
/// journal, so the rows are in the store **and** in the argument. Seeding only the argument would
/// let a `resume` that re-appended `prior` pass unnoticed.
fn seeded_store(prior: &[Event]) -> Arc<Mutex<dyn EventStore>> {
    let mut store = VecStore::default();
    for event in prior {
        store.append(event.clone()).expect("the fake store appends");
    }
    Arc::new(Mutex::new(store))
}

fn ports(store: &Arc<Mutex<dyn EventStore>>, script: usize) -> Ports {
    let ok = || {
        Ok(Typed::new(
            SchemaId::new(RECORDED_SCHEMA),
            serde_json::json!({ "ok": true }),
        ))
    };
    Ports {
        store: Arc::clone(store),
        harness: Arc::new(FakeHarness::scripted((0..script).map(|_| ok()).collect())),
        tracker: Arc::new(BoardTracker::default()),
        docs: Arc::new(RecordingDocs::default()),
        toolchain: Arc::new(FixtureToolchain::default()),
    }
}

fn rows(store: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    store
        .lock()
        .expect("the fake store's lock is sound")
        .query(journal_query(ITEM))
        .expect("the fake store queries")
}

/// A `gate_opened` row the **runtime** wrote, told apart from the two
/// [`rk_kernel::select::Selection::journal`] emits by the member only the runtime's carries.
///
/// This distinction is the whole instrument: the selection's rows are `{reason, axis, site}` and
/// `{reason, unsatisfiable}`, and a test that counted bare `gate_opened` kinds would read **two**
/// before `resume` had stepped anything and could never go red.
fn stepped_gates(events: &[Event]) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened && e.payload.get("token").is_some())
        .collect()
}

fn count(events: &[Event], kind: EventKind) -> usize {
    events.iter().filter(|e| e.kind == kind).count()
}

/// **§7d.1 edit (6) / §7d.5 step (9) / RT-55 — four conjuncts, one per red §7d.14 declares.**
///
/// §7d.14, verbatim: red *"by a `resume` over a journal holding an applied answer re-deriving
/// `OpenGate` at reach 1 (a fresh local fold), by no `gate_opened` row appended when `next_action`
/// returns `OpenGate` (the `record()` break), by a dispatched node whose binding the supplier served
/// being refused on `inputs/specs.json` (the seam is not consulted), or by `record()`'s own recorded
/// output changing (its 7b fixtures are the control)."*
///
/// - **(D)** `record` still **breaks** on `OpenGate`: it appends no runtime `gate_opened` and its
///   residue names the open-gate action. This is also (A)'s positive control — it is what proves
///   the observation point can read zero, so (A)'s one is a difference and not a constant.
/// - **(A)** `resume` over an empty prior **steps** it: exactly one runtime `gate_opened`, carrying
///   the full gate class, reach 1, and the supplied tree hash and deadline.
/// - **(B)** `resume` over a journal holding the applied answer does **not** re-open it — zero new
///   runtime `gate_opened` rows, against (A)'s one on the same instrument — and reaches the
///   dispatch the answer unblocks. It also writes no second `dag_assembled`, which is the rule the
///   function's header states: the opening rows are per item, and `prior.is_empty()` is not the
///   discriminator.
/// - **(C)** the spec-supplier seam is consulted: over an input set missing one node's binding,
///   `record` refuses on `inputs/specs.json` and `resume` does not, and the supplier's own log
///   names that node and **only** that node — the bound one is served from `inputs`. The log's
///   non-emptiness is the positive control for that second, absence-shaped half.
#[test]
fn replay_resume_seeds_fold_from_journal_and_steps_gate_open() {
    // ---- PREMISE. Both halves of the fixture, asserted before anything is claimed over them. ---
    let all = inputs(&BOUND);
    let (pack, catalog, artifacts, facets, item) =
        all.build().expect("the inputs rebuild the item");
    let selection = rk_kernel::select::select(&pack, &catalog, &artifacts, &facets, STAGE, &item)
        .expect("the fixture selection resolves");
    assert_eq!(
        selection.gates().len(),
        1,
        "PREMISE: the fixture's abstaining `require` row carries exactly one gate — without it \
         `next_action` never returns `OpenGate` and every conjunct below is vacuous"
    );
    assert_eq!(
        selection.dag().nodes().len(),
        2,
        "PREMISE: and the plan is not empty, so (B)'s dispatch is a real action rather than the \
         absence of one"
    );
    let ctx = FsmContext::supplied(
        item.id(),
        TREE_HASH,
        Timestamp::from_unix_seconds(DEADLINE),
        Some("verify"),
    );
    let fresh = fsm::fold(item.id(), STAGE, &selection.journal()).expect("the journal folds");
    let NextAction::OpenGate(record) =
        fsm::next_action(&selection, &fresh, &ctx).expect("computes")
    else {
        panic!("PREMISE: a fresh fold of this fixture must reach `OpenGate`");
    };
    assert_eq!(
        record.kind(),
        GATE_KIND,
        "PREMISE: the gate class, axis and all"
    );

    // ---- (D) `record` BREAKS on the gate. The control for (A), and §7d.14's fourth red. --------
    let control_store = seeded_store(&[]);
    let recorded = replay::record(&all, ports(&control_store, 0)).expect("the recording completes");
    assert!(
        stepped_gates(&rows(&control_store)).is_empty(),
        "(D) `record` appends NO runtime `gate_opened` — it breaks on `OpenGate` without \
         appending, which is exactly the behaviour `resume` exists to differ from. A non-empty \
         reading here means `record`'s own recorded output moved, which §7d.15 forbids"
    );
    assert!(
        recorded.next_action().contains("open_gate"),
        "(D) and its residue names the open-gate action rather than stepping past it: {}",
        recorded.next_action()
    );

    // ---- (A) `resume` STEPS it. -----------------------------------------------------------------
    let opened_store = seeded_store(&[]);
    let mut never = |node: &str| -> Result<RecordedSpec, ReplayError> {
        panic!("(A) reaches no dispatch, so the supplier must not be called — it was, for {node}")
    };
    let opened = replay::resume(&all, ports(&opened_store, 0), &[], &mut never)
        .expect("the resumed run completes");
    let written = rows(&opened_store);
    let stepped = stepped_gates(&written);
    assert_eq!(
        stepped.len(),
        1,
        "(A) exactly one runtime `gate_opened` reaches the store, against (D)'s zero on the same \
         instrument. Rows written: {:?}",
        written
            .iter()
            .map(|e| e.kind.wire_name())
            .collect::<Vec<_>>()
    );
    let payload = &stepped[0].payload;
    assert_eq!(payload["kind"], serde_json::json!(GATE_KIND));
    assert_eq!(payload["reach"], serde_json::json!(1));
    assert_eq!(payload["tree_hash"], serde_json::json!(TREE_HASH));
    assert_eq!(payload["deadline"], serde_json::json!(DEADLINE));
    assert_eq!(
        payload["token"],
        serde_json::json!(record.token().as_str()),
        "(A) and the token is the FSM's own derived one, not a value `resume` minted"
    );
    assert_eq!(
        count(&opened.events, EventKind::Dispatch),
        0,
        "(A) a run that opens a gate dispatches nothing — it returns at the gate"
    );

    // ---- (B) the fold is SEEDED FROM `prior`. ---------------------------------------------------
    // The journal the store now holds, plus the operator's answer to the gate `resume` just opened.
    let mut prior = opened.events.clone();
    prior.push(Event::new(
        EventKind::GateAnswered,
        serde_json::json!({
            "token": record.token().as_str(),
            "tree_hash": TREE_HASH,
            "verdict": "proceed",
        }),
    ));
    let resumed_store = seeded_store(&prior);
    let resumed = replay::resume(&all, ports(&resumed_store, 2), &prior, &mut never)
        .expect("the resumed run completes");
    let after = rows(&resumed_store);
    assert_eq!(
        stepped_gates(&after).len(),
        1,
        "(B) the gate is NOT re-opened: the store still holds the ONE runtime `gate_opened` it was \
         seeded with. A `resume` folding a fresh local vector reads `reaches == 0` and mints a \
         second at reach 1 — that is §7d.14's first red, and (A)'s reading is the control proving \
         this observation point moves"
    );
    assert_eq!(
        count(&after, EventKind::DagAssembled),
        1,
        "(B) and no second `dag_assembled`: the opening rows belong to the run that assembled the \
         plan. `prior.is_empty()` is not the discriminator — a first `run` reaches the kernel with \
         a `reconcile_started` row already in the store"
    );
    assert_eq!(
        count(&after, EventKind::Dispatch),
        2,
        "(B) and the answer unblocks the plan: both baseline nodes dispatch. Zero here means the \
         fold never saw the answer; one means it stopped early"
    );
    assert_eq!(count(&after, EventKind::ActivityCommitted), 2);
    assert_eq!(
        resumed.events.len(),
        after.len(),
        "(B) the returned record carries `prior` FOLLOWED BY this run's rows — the whole journal \
         the item has, which is what `replay` and `explain` read back"
    );

    // ---- (C) the spec-supplier seam is CONSULTED. ----------------------------------------------
    let partial = inputs(&BOUND[1..]);
    let unbound = BOUND[0].0;
    let refused = replay::record(&ungated(&BOUND[1..]), ports(&seeded_store(&[]), 2));
    match refused {
        Err(ReplayError::Malformed { ref path, .. }) if path == "inputs/specs.json" => {}
        other => panic!(
            "(C) PREMISE: a run that REACHES a node `inputs` does not bind is refused on \
             `inputs/specs.json` — that refusal is what the seam exists to prevent, and without it \
             the conjunct measures nothing. Measured over the UNGATED twin, because a gated run \
             breaks at the gate before it reaches any binding. Got: {}",
            match other {
                Ok(run) => run.next_action().to_owned(),
                Err(e) => e.to_string(),
            }
        ),
    }
    let asked: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let log = Arc::clone(&asked);
    let mut supplier = move |node: &str| -> Result<RecordedSpec, ReplayError> {
        log.lock()
            .expect("the log's lock is sound")
            .push(node.to_owned());
        Ok(RecordedSpec::supplied(node, spec_for(node)))
    };
    let served = replay::resume(
        &partial,
        ports(&seeded_store(&prior), 2),
        &prior,
        &mut supplier,
    );
    let asked = asked.lock().expect("the log's lock is sound").clone();
    assert_eq!(
        asked,
        vec![unbound.to_owned()],
        "(C) the supplier is consulted for the node `inputs` does not bind, and for that node \
         ONLY — `impl-report` is bound and is served from `inputs`. The first half is this \
         assertion's own positive control for the second"
    );
    match served {
        Err(ReplayError::Malformed { ref path, .. }) if path == "inputs/specs.json" => panic!(
            "(C) `resume` refused on `inputs/specs.json` — the seam was not consulted. That is \
             §7d.14's third red"
        ),
        // RAISED in this module's header, and asserted rather than hidden: the supplier serves the
        // SPEC, and `RunInputs::activations()` reads the same `specs` list, so a node absent from
        // it also has no activation and `checked_key` refuses its bare id one layer down. The seam
        // is consulted — which is what this conjunct measures — and the run still cannot complete.
        Err(other) => assert!(
            format!("{other}").contains("@version"),
            "(C) past the seam, the only remaining refusal is the recorded `activity_id@version` \
             join this stage may not take. Got: {other}"
        ),
        Ok(_) => panic!(
            "(C) this run cannot complete on today's tree — see the module header. A green here \
             means the activation coupling moved and the RAISED note is stale"
        ),
    }
}
