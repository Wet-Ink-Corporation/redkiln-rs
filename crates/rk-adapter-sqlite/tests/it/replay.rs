//! **AC-243 (real-store replay)** — CROSS-LISTED; the kernel half was discharged at 7b-ii
//! (`rk-kernel`'s `ac243_no_dispatch_across_open_gate`). 7c claims **no new instrument**: this
//! re-runs the kernel's open-gate fixture over the real store (s7c §7c.13) — the same selection,
//! the same five ticks, the same replay checker — with `rk-kernel` as a **dev** dependency (the
//! allowed direction; AC-484 forbids the reverse and Checkpoint 7c walks the kernel's normal
//! graph).
//!
//! What the real store adds to the kernel's own run: the journal the checker reads is a
//! **read-back** through `t2_event` — every row a kernel step wrote went to disk as
//! `(kind, payload)` text and came back ordered by id — so a store that lost a row, re-ordered
//! two, or mangled a payload member (`token`, `tree_hash`) would move the checker's answer. The
//! positive control is appended through the real store too, and the checker must still find it.
//!
//! The fixture files are the kernel's, read by path from its `tests/fixtures/` (the pack, the
//! catalog, the artifact registry) and `spikes/p9/facets.toml`, exactly as its `fsm.rs` reads
//! them — a copy would be a second fixture.

use std::sync::{Arc, Mutex};

use rk_adapter_sqlite::SqliteStore;
use rk_kernel::runtime::{Runtime, StepOutcome, gate_opened_event};
use rk_kernel::select::{ArtifactRegistry, Catalog, Item, Pack, select};
use rk_kernel::workflow::breaker::JournaledNow;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_kernel::workflow::fsm::{FsmContext, NextAction, next_action};
use rk_kernel::workflow::gate;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs};
use rk_ports::root::Ports;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

use crate::support::{Fixture, open};

const ITEM: &str = "INC-1";
const TREE_A: &str = "aaaa111122223333444455556666777788889999";

const INCIDENT: &str =
    include_str!("../../../rk-kernel/tests/fixtures/packs/incident-forbidden.toml");
const ARTIFACTS: &str =
    include_str!("../../../rk-kernel/tests/fixtures/registries/artifacts/artifacts.toml");
const CATALOG: &[(&str, &str)] = &[
    (
        "impl-report.toml",
        include_str!("../../../rk-kernel/tests/fixtures/catalog-select/impl-report.toml"),
    ),
    (
        "change-summary.toml",
        include_str!("../../../rk-kernel/tests/fixtures/catalog-select/change-summary.toml"),
    ),
    (
        "change-summary-ue.toml",
        include_str!("../../../rk-kernel/tests/fixtures/catalog-select/change-summary-ue.toml"),
    ),
    (
        "changeset.toml",
        include_str!("../../../rk-kernel/tests/fixtures/catalog-select/changeset.toml"),
    ),
    (
        "spec-authoring.toml",
        include_str!("../../../rk-kernel/tests/fixtures/catalog-select/spec-authoring.toml"),
    ),
];

fn facets() -> rk_kernel::registry::Registry {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml");
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry must load")
}

/// The kernel's `gated()` selection: exactly one `gate_opened { reason: unknown_facet, axis }`.
fn gated() -> rk_kernel::select::Selection {
    let pack =
        Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("the fixture pack loads");
    let catalog = Catalog::from_sources(CATALOG).expect("the fixture catalog loads");
    let artifacts =
        ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the registry loads");
    let item = Item::new(ITEM, "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_null("external_dep");
    select(&pack, &catalog, &artifacts, &facets(), "analysis", &item).expect("selection resolves")
}

const fn ctx(tree: &str, deadline: i64) -> FsmContext<'_> {
    FsmContext::supplied(ITEM, tree, Timestamp::from_unix_seconds(deadline), None)
}

const fn at(secs: i64) -> JournaledNow {
    JournaledNow::from_journal(Timestamp::from_unix_seconds(secs))
}

fn ports(store: SqliteStore) -> (Ports, Arc<Mutex<dyn EventStore>>) {
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(store));
    let ports = Ports {
        store: Arc::clone(&store),
        // An empty script: any dispatch the kernel issued would exhaust it and the fake would
        // append a `step_failed` of its own — a row the assertions below would see.
        harness: Arc::new(FakeHarness::scripted(Vec::new())),
        tracker: Arc::new(BoardTracker::default()),
        docs: Arc::new(RecordingDocs::default()),
        toolchain: Arc::new(FixtureToolchain::default()),
    };
    (ports, store)
}

/// RED (observed 2026-09-18), twice: against the skeleton that applied no schema — the runtime's
/// first append, the `gate_opened` row, failed with `no such table: t2_event`; and via a
/// temporary mutant of `query` that returned rows `ORDER BY id DESC` — the forged dispatch inside
/// the open window read back before its `gate_opened` and the checker reported nothing (s7c
/// §7c.18: *"red by a `dispatch` row inside an open gate"*). Run and reverted; both quoted.
#[test]
fn ac243_gate_open_replay_over_sqlite_store() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let (ports, store) = ports(open(&exec, &repo));

    // ---- the runtime over the REAL store: a gate opens, then five ticks of AwaitGate ----------
    let gated = gated();
    let rt = Runtime::new(ports, "run-ac243", "analysis");
    let state0 = rt.state(ITEM).expect("folds");
    let open_gate = match next_action(&gated, &state0, &ctx(TREE_A, 9_000)).expect("computes") {
        NextAction::OpenGate(record) => record,
        other => panic!("{other:?}"),
    };
    let outcome = rt
        .step(
            &NextAction::OpenGate(open_gate.clone()),
            None,
            AttemptCap::declared(4),
            at(9_000),
        )
        .expect("steps");
    assert!(matches!(outcome, StepOutcome::GateOpened { .. }));
    for tick in 1..=5 {
        let state = rt.state(ITEM).expect("folds — the read-back journal folds");
        let action = next_action(&gated, &state, &ctx(TREE_A, 9_000 + tick)).expect("computes");
        assert!(
            matches!(action, NextAction::AwaitGate { .. }),
            "tick {tick}: with a gate open the next action is AwaitGate, not {action:?}"
        );
        rt.step(&action, None, AttemptCap::declared(4), at(9_000 + tick))
            .expect("awaiting is a step");
    }

    // ---- the journal, READ BACK from t2_event through the port ------------------------------
    let journal = rt.journal(ITEM).expect("query");
    assert_eq!(
        journal.len(),
        1,
        "one gate_opened and nothing else: no dispatch row, no step_failed from the fake, no \
         AwaitGate row (ADR-0022) — {journal:?}"
    );
    assert!(
        journal.iter().all(|e| e.kind != EventKind::Dispatch),
        "no dispatch row was written while the gate was open"
    );
    assert_eq!(
        journal[0],
        gate_opened_event(&open_gate),
        "the row came back as it was written"
    );
    assert_eq!(
        gate::dispatches_across_open_gates(&journal),
        Vec::<gate::DispatchAcrossGate>::new(),
        "the runtime's own journal, read back from the store, is clean"
    );

    // ---- POSITIVE CONTROL through the same store: a dispatch between gate_opened and the gate's
    // terminal event is reported by the checker over the read-back journal. Appended through the
    // port, in that order, so the store's ordering is what the checker sees.
    {
        let mut guard = store.lock().expect("the store lock is not poisoned");
        guard
            .append(Event::new(
                EventKind::Dispatch,
                serde_json::json!({ "item": ITEM, "activity_id_at_version": "impl-report@1", "attempt": 1 }),
            ))
            .expect("append the forged dispatch");
        guard
            .append(gate::gate_answered_event(
                open_gate.token(),
                TREE_A,
                "proceed",
            ))
            .expect("append the answer");
    }
    let forged = store
        .lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("query");
    assert_eq!(
        forged.len(),
        3,
        "gate_opened, dispatch, gate_answered — in append order"
    );
    let found = gate::dispatches_across_open_gates(&forged);
    assert_eq!(
        found.len(),
        1,
        "the dispatch inside the open window is reported over the read-back journal: {found:?}"
    );
    assert_eq!(found[0].gate, *open_gate.token());
    assert_eq!(found[0].dispatch_index, 1, "the dispatch is the second row");
}
