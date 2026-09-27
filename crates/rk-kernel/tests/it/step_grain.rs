//! **Step-grain commits (`s8d` §8d.2; record R1, proposed).**
//!
//! D21-42 3a: *"Step grain in the runtime. Per-step specs are carried on `RecordedSpec` (ADR-0138's
//! warrant), each agent step commits under its own key, and re-entry resumes at the first
//! uncommitted step."* D21-42 3b: *"`(item, activity@version, step_id, iteration path)`."* D21-43
//! N15: *"A separate commit key. ADR-0097's memo key is untouched."*
//!
//! Each test below is over a real `Runtime` and the in-memory store `runtime.rs` counts through,
//! and each reads the JOURNAL (the rows a later process would fold), not only the report the walk
//! handed back: a walk that reported step-grain commits while appending activation-grain rows would
//! pass a report-only test and lose every step on re-entry.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::registry::Registry;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::{commit_key_of, committed_step_result, is_completion_row};
use rk_kernel::runtime::step::{Activation, ToolRouting};
use rk_kernel::select::digest_of;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_kernel::workflow::fsm::{FsmContext, NextAction, fold, next_action};
use rk_kernel::workflow::idem::{CommitKey, Frame, IterationPath};
use rk_kernel::workflow::memo::{CardOrTool, ToolchainValues, VerbRegistry};
use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_ports::toolchain::{Toolchain, VerbOutput};
use rk_store::memo::IdempotencyKey;
use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::digest::Digest;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::retention::Retained;
use rk_types::typed::{SchemaId, Typed};
use rk_types::usage::{CliVersion, Timings};

use crate::runtime::{
    AID, CAP, CountingToolchain, ITEM, STAGE, STEP, at, counters, empty_state, gate_site, journal,
    ports, ports_with_toolchain, spec, store,
};

// ---- the fixtures --------------------------------------------------------------------------------

/// A step of `kind`, every optional field absent.
fn step(id: &str, kind: StepKind) -> Step {
    Step {
        id: id.to_owned(),
        kind,
        run: None,
        output_type: None,
        role: None,
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: None,
        inputs: BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    }
}

fn agent(id: &str, role: &str) -> Step {
    Step {
        role: Some(role.to_owned()),
        ..step(id, StepKind::Agent)
    }
}

fn emit() -> Step {
    step("emit", StepKind::Emit)
}

/// One per-step spec: its own card digest (`hex` repeated) and its own schema id.
fn step_spec(hex: &str, schema: &str) -> ActivitySpec {
    ActivitySpec {
        card_digest: Digest::from_hex(&hex.repeat(32)).expect("64 lowercase hex characters"),
        tools: vec!["Read".to_owned()],
        schema: SchemaId::new(schema),
        timeout_secs: 60,
        inputs: vec![("item".to_owned(), serde_json::json!(ITEM))],
    }
}

fn shipped_registry() -> Registry {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves");
    let src = std::fs::read_to_string(root.join(".redkiln").join("facets.toml"))
        .expect("the shipped facet registry is readable");
    Registry::from_toml(&src).expect("the shipped facet registry loads")
}

const fn activation<'a>(
    node_spec: Option<&'a ActivitySpec>,
    step_specs: Option<&'a BTreeMap<String, ActivitySpec>>,
) -> Activation<'a> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: node_spec,
        step_specs,
        rigor: Rigor::Recommended,
    }
}

/// A harness that RECORDS the spec each dispatch was handed and answers under THAT spec's schema,
/// from a script: `Some(value)` is a well-formed result, `None` a non-retryable refusal.
pub struct SpecEchoHarness {
    script: Mutex<Vec<Option<serde_json::Value>>>,
    seen: Mutex<Vec<(String, String)>>,
}

impl SpecEchoHarness {
    pub const fn scripted(script: Vec<Option<serde_json::Value>>) -> Self {
        Self {
            script: Mutex::new(script),
            seen: Mutex::new(Vec::new()),
        }
    }

    /// `(card_digest, schema)` of every spec the port was handed, in dispatch order.
    pub fn seen(&self) -> Vec<(String, String)> {
        self.seen.lock().expect("the lock is sound").clone()
    }

    pub fn dispatches(&self) -> usize {
        self.seen().len()
    }
}

impl Harness for SpecEchoHarness {
    fn dispatch(&self, spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        self.seen.lock().expect("the lock is sound").push((
            spec.card_digest.as_str().to_owned(),
            spec.schema.as_str().to_owned(),
        ));
        let next = {
            let mut script = self.script.lock().expect("the lock is sound");
            if script.is_empty() {
                None
            } else {
                script.remove(0)
            }
        };
        DispatchOutcome {
            record: DispatchRecord {
                argv_digest: digest_of(b"argv"),
                env_digest: digest_of(b"env"),
                instrument_version: "spec-echo-fake".to_owned(),
                stdout: Retained::of(b""),
                stderr: Retained::of(b""),
                usage: None,
                timings: Timings {
                    wall_ms: 0,
                    api_ms: None,
                },
                total_cost_usd: None,
                headroom_before: None,
                headroom_after: None,
                cli_version: CliVersion::Pinned("spec-echo-fake".to_owned()),
                api_retry_events: 0,
                prompt_cache_ttl: "1h".to_owned(),
                subagent_prompt_cache_ttl: "1h".to_owned(),
                cancelled_by: None,
                detected_by: None,
            },
            result: next.map_or_else(
                || {
                    Err(StepError::Unsupported {
                        capability: "the scripted step refuses".to_owned(),
                    })
                },
                |value| Ok(Typed::new(spec.schema.clone(), value)),
            ),
        }
    }
}

fn committed_rows(events: &[Event]) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .collect()
}

fn key(step_id: &str, path: &IterationPath) -> CommitKey {
    CommitKey {
        item: ITEM.to_owned(),
        activity_id_at_version: AID.to_owned(),
        step_id: step_id.to_owned(),
        iteration_path: path.clone(),
    }
}

// ---- the tests ------------------------------------------------------------------------------------

/// **R1: each agent step dispatches under its OWN spec and commits under its OWN key**, as one
/// `activity_committed` row carrying `step_id` and `iteration_path`; the activation completes on
/// one more row, the `emit` step's.
#[test]
fn each_agent_step_dispatches_under_its_own_spec_and_commits_under_its_own_key() {
    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(SpecEchoHarness::scripted(vec![
        Some(serde_json::json!({ "draft": 1 })),
        Some(serde_json::json!({ "review": 2 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-step-grain",
        STAGE,
    );
    let node = spec();
    let specs: BTreeMap<String, ActivitySpec> = [
        ("draft".to_owned(), step_spec("aa", "ImplReport/v1")),
        ("review".to_owned(), step_spec("bb", "Findings/v1")),
    ]
    .into_iter()
    .collect();
    let steps = vec![
        agent("draft", "implementer"),
        agent("review", "reviewer"),
        emit(),
    ];
    let report = rt
        .run_activity_steps(
            activation(Some(&node), Some(&specs)),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a value, not an error of the walk (AC-032)");

    // ---- (1) EACH DISPATCH WAS HANDED ITS OWN STEP'S SPEC, never the node's. -------------------
    assert_eq!(
        harness.seen(),
        vec![
            ("aa".repeat(32), "ImplReport/v1".to_owned()),
            ("bb".repeat(32), "Findings/v1".to_owned()),
        ],
        "each agent step must reach the Harness under its own card and schema (R1); a walk that \
         handed both the node's spec, or refused the second, fails here. outcome = {:?}, \
         step_error = {:?}",
        report.outcome(),
        report.step_error()
    );
    assert!(
        report.committed() && report.completed(),
        "the walk reached its emit, so it commits and completes: {report:?}"
    );

    // ---- (2) THE JOURNAL: one row per step activation, under its own commit key. ---------------
    let events = journal(&db);
    let rows = committed_rows(&events);
    let steps_committed: Vec<CommitKey> = rows
        .iter()
        .filter(|e| !is_completion_row(e))
        .filter_map(|e| commit_key_of(e))
        .collect();
    let root = IterationPath::root();
    assert_eq!(
        steps_committed,
        vec![key("draft", &root), key("review", &root)],
        "each agent step's commit is ONE `activity_committed` row carrying `step_id` and \
         `iteration_path` (R1; M5): {rows:?}"
    );
    for row in rows.iter().filter(|e| !is_completion_row(e)) {
        assert!(
            row.payload.get("step_id").is_some() && row.payload.get("iteration_path").is_some(),
            "a step's commit row carries both payload members: {row:?}"
        );
    }
    let completion: Vec<&&Event> = rows.iter().filter(|e| is_completion_row(e)).collect();
    assert_eq!(
        completion.len(),
        1,
        "exactly one completion row, appended when the emit binding commits: {rows:?}"
    );
    assert_eq!(
        completion[0]
            .payload
            .get("step_id")
            .and_then(serde_json::Value::as_str),
        Some("emit"),
        "the completion row's `step_id` is the activation's `emit` step's"
    );
    assert_eq!(
        rows.len(),
        3,
        "two step rows and the completion, no more: {rows:?}"
    );
    assert_eq!(
        events
            .iter()
            .filter(|e| e.kind == EventKind::Dispatch)
            .count(),
        2,
        "one `dispatch` cost row per agent step"
    );
    assert_eq!(
        committed_step_result(&events, &key("review", &root)),
        Some(&serde_json::json!({ "review": 2 })),
        "the review step's own value is its own row's `result`"
    );

    // ---- (3) THE FOLD READS BOTH GRAINS BACK. --------------------------------------------------
    let folded = fold(ITEM, STAGE, &events).expect("the journal folds");
    assert!(folded.is_step_committed(&key("draft", &root)));
    assert!(folded.is_step_committed(&key("review", &root)));
    assert!(
        folded.is_committed(ITEM, AID),
        "the completion row completes the activation"
    );
}

/// **Defect 2 (§8d.8), and ADR-0143 decision 2 as R1 narrows it**: a runnable agent step with NO
/// per-step spec refuses by name, and the report STAYS committed — `done.with_step_failure(cause)`,
/// ADR-0144's shape, never `ActivityReport::refused`, whose report answers `committed() == false`
/// for an activation whose row is in the journal.
#[test]
fn a_runnable_agent_step_with_no_per_step_spec_refuses_and_the_report_stays_committed() {
    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(SpecEchoHarness::scripted(vec![
        Some(serde_json::json!({ "draft": 1 })),
        Some(serde_json::json!({ "never": 2 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-no-step-spec",
        STAGE,
    );
    let node = spec();
    // `review` has NO per-step spec.
    let specs: BTreeMap<String, ActivitySpec> =
        BTreeMap::from([("draft".to_owned(), step_spec("aa", "ImplReport/v1"))]);
    let steps = vec![
        agent("draft", "implementer"),
        agent("review", "reviewer"),
        emit(),
    ];
    let report = rt
        .run_activity_steps(
            activation(Some(&node), Some(&specs)),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a value (AC-032)");

    assert_eq!(
        harness.dispatches(),
        1,
        "the step with no per-step spec never reaches the Harness"
    );
    assert!(
        report.committed() && !report.skipped(),
        "the first step's commit is in the journal, so the report must answer committed() == true \
         and skipped() == false (§8d.8 defect 2; ADR-0144). committed = {}, skipped = {}",
        report.committed(),
        report.skipped()
    );
    let refusal = format!(
        "{:?}",
        report.step_error().unwrap_or_else(|| panic!(
            "the refusal must travel on step_error (ADR-0144's channel): outcome = {:?}",
            report.outcome()
        ))
    );
    for expected in [AID, "review", "no per-step spec"] {
        assert!(
            refusal.contains(expected),
            "the refusal does not name {expected:?}: {refusal}"
        );
    }
    assert!(
        !report.completed(),
        "a refused walk does not complete: no completion row is appended"
    );
    let events = journal(&db);
    assert_eq!(
        committed_rows(&events).len(),
        1,
        "only the draft step's row"
    );
}

/// **Re-entry resumes at the FIRST UNCOMMITTED step and re-dispatches no committed one** (AC-013 at
/// step grain). A first walk commits two of three agent steps and stops on the third; a second
/// `Runtime` over the same journal serves the two and dispatches the third once.
#[test]
fn re_entry_resumes_at_the_first_uncommitted_step_and_re_dispatches_no_committed_one() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let node = spec();
    let specs: BTreeMap<String, ActivitySpec> = [
        ("one".to_owned(), step_spec("a1", "ImplReport/v1")),
        ("two".to_owned(), step_spec("b2", "ImplReport/v1")),
        ("three".to_owned(), step_spec("c3", "ImplReport/v1")),
    ]
    .into_iter()
    .collect();
    let steps = vec![
        agent("one", "implementer"),
        agent("two", "implementer"),
        agent("three", "implementer"),
        emit(),
    ];

    // ---- the first walk: one and two commit, three refuses. ------------------------------------
    let first = Arc::new(SpecEchoHarness::scripted(vec![
        Some(serde_json::json!({ "n": 1 })),
        Some(serde_json::json!({ "n": 2 })),
        None,
    ]));
    let rt1 = Runtime::new(
        ports(Arc::clone(&first) as Arc<dyn Harness>, &db),
        "run-first",
        STAGE,
    );
    let state = empty_state();
    let r1 = rt1
        .run_activity_steps(
            activation(Some(&node), Some(&specs)),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the first walk resolves");
    assert_eq!(
        first.dispatches(),
        3,
        "PREMISE: the first walk reached all three"
    );
    assert!(
        !r1.completed(),
        "PREMISE: the first walk did not complete: {r1:?}"
    );
    let before = journal(&db);
    let digests_before: Vec<String> = ["one", "two"]
        .iter()
        .map(|s| {
            rk_store::canon::canonical_digest(
                committed_step_result(&before, &key(s, &IterationPath::root()))
                    .expect("committed before the re-entry"),
            )
            .expect("no floats")
            .as_str()
            .to_owned()
        })
        .collect();

    // ---- the re-entry: a different Runtime, the same journal. ----------------------------------
    let second = Arc::new(SpecEchoHarness::scripted(vec![Some(
        serde_json::json!({ "n": 3 }),
    )]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&second) as Arc<dyn Harness>, &db),
        "run-re-entry",
        STAGE,
    );
    let r2 = rt2
        .run_activity_steps(
            activation(Some(&node), Some(&specs)),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            AttemptCap::declared(CAP),
            at(200),
        )
        .expect("the re-entry resolves");
    assert_eq!(
        second.seen(),
        vec![("c3".repeat(32), "ImplReport/v1".to_owned())],
        "the re-entry dispatches ONLY the first uncommitted step, under its own spec — zero \
         re-dispatch of a committed one"
    );
    assert_eq!(
        r2.served(),
        2,
        "the two committed steps are SERVED from the journal"
    );
    assert!(r2.completed(), "and the walk reaches its emit: {r2:?}");
    let after = journal(&db);
    let digests_after: Vec<String> = ["one", "two"]
        .iter()
        .map(|s| {
            rk_store::canon::canonical_digest(
                committed_step_result(&after, &key(s, &IterationPath::root()))
                    .expect("still committed"),
            )
            .expect("no floats")
            .as_str()
            .to_owned()
        })
        .collect();
    assert_eq!(
        digests_after, digests_before,
        "AC-013: every committed step's digest is byte-identical after the re-entry"
    );
    for s in ["one", "two"] {
        assert_eq!(
            committed_rows(&after)
                .iter()
                .filter(|e| commit_key_of(e) == Some(key(s, &IterationPath::root())))
                .count(),
            1,
            "`{s}` is committed exactly once across both walks"
        );
    }
}

/// **Two iterations of one step never share a commit key** (D21-42 3b; AC-017 carried to the
/// commit): the same step under two iteration paths dispatches and commits twice, and the same
/// path a second time is served.
#[test]
fn two_iterations_of_one_step_never_share_a_commit_key() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(SpecEchoHarness::scripted(vec![
        Some(serde_json::json!({ "pass": 1 })),
        Some(serde_json::json!({ "pass": 2 })),
        Some(serde_json::json!({ "pass": 99 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-iterations",
        STAGE,
    );
    let node = spec();
    let specs: BTreeMap<String, ActivitySpec> =
        BTreeMap::from([("green".to_owned(), step_spec("ee", "ImplReport/v1"))]);
    let green = agent("green", "implementer");
    let first = IterationPath::root().pushed(Frame::LoopBack(1));
    let second = IterationPath::root().pushed(Frame::LoopBack(2));
    for path in [&first, &second] {
        let report = rt
            .run_agent_step(
                activation(Some(&node), Some(&specs)),
                &green,
                path,
                AttemptCap::declared(CAP),
                at(100),
            )
            .expect("the step activation resolves");
        assert!(
            report.committed(),
            "the step under iteration path {path} must dispatch and commit under its own key: \
             {report:?}"
        );
    }
    assert_eq!(harness.dispatches(), 2, "two iterations, two dispatches");
    let events = journal(&db);
    let keys: Vec<CommitKey> = committed_rows(&events)
        .iter()
        .filter_map(|e| commit_key_of(e))
        .collect();
    assert_eq!(
        keys,
        vec![key("green", &first), key("green", &second)],
        "one row per iteration, each carrying its own iteration path"
    );
    assert_ne!(key("green", &first), key("green", &second));

    // The SAME path again is served — it is the same step activation.
    let again = rt
        .run_agent_step(
            activation(Some(&node), Some(&specs)),
            &green,
            &first,
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("resolves");
    assert!(
        again.skipped(),
        "a committed step activation is not re-executed"
    );
    assert_eq!(
        harness.dispatches(),
        2,
        "the same iteration path shares the key and is served, never re-dispatched"
    );
}

/// **The memo key of an agent step carries THAT step's card digest and schema id** (§8d.2), never
/// the node's. ADR-0097's tuple is untouched; this is what it is assembled from.
#[test]
fn the_memo_key_of_an_agent_step_carries_that_steps_own_card_digest() {
    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(SpecEchoHarness::scripted(vec![
        Some(serde_json::json!({ "draft": 1 })),
        Some(serde_json::json!({ "review": 2 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-memo",
        STAGE,
    );
    let node = spec();
    let draft = step_spec("aa", "ImplReport/v1");
    let review = step_spec("bb", "Findings/v1");
    let specs: BTreeMap<String, ActivitySpec> = [
        ("draft".to_owned(), draft.clone()),
        ("review".to_owned(), review.clone()),
    ]
    .into_iter()
    .collect();
    let steps = vec![
        agent("draft", "implementer"),
        agent("review", "reviewer"),
        emit(),
    ];
    let report = rt
        .run_activity_steps(
            activation(Some(&node), Some(&specs)),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("resolves");
    let contexts = report.memo_contexts();
    assert_eq!(
        contexts.len(),
        2,
        "one memo context per DISPATCHED agent step: {contexts:?}"
    );
    for (ctx, (id, own)) in contexts
        .iter()
        .zip([("draft", &draft), ("review", &review)])
    {
        assert_eq!(ctx.step_id, id);
        assert_eq!(
            ctx.card_or_tool,
            CardOrTool::Card(own.card_digest.clone()),
            "`{id}`'s memo component 5 must be its OWN card digest, never the node's"
        );
        assert_ne!(ctx.card_or_tool, CardOrTool::Card(node.card_digest.clone()));
        assert_eq!(
            ctx.output_schema, own.schema,
            "`{id}`'s memo component 8 must be its OWN schema id"
        );
        assert_eq!(ctx.activity_id_at_version, AID);
        assert_eq!(ctx.rigor, Rigor::Recommended);
        assert_eq!(ctx.loop_iteration, 0, "component 7 is 0 outside a loop");
    }
    let k0 = contexts[0].key().expect("assembles");
    let k1 = contexts[1].key().expect("assembles");
    assert_ne!(k0, k1, "two steps' memo keys differ");
}

/// **An activation with no agent step walks to its emit and completes** (§8d.8 defect 4's kernel
/// half): bound as an explicit absence of agent specs, with no activation spec at all, and
/// completed by its one completion row.
#[test]
fn an_activation_with_no_agent_step_walks_to_its_emit_and_completes() {
    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(SpecEchoHarness::scripted(Vec::new()));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-no-agent",
        STAGE,
    );
    let none: BTreeMap<String, ActivitySpec> = BTreeMap::new();
    let check = Step {
        expr: Some("rigor >= optional".to_owned()),
        ..step("verify", StepKind::Check)
    };
    let steps = vec![check, emit()];
    let report = rt
        .run_activity_steps(
            activation(None, Some(&none)),
            Some(&steps),
            &rk_kernel::rigor::with_rigor(Context::new(), Rigor::Recommended, &shipped_registry()),
            gate_site(&state),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("resolves");
    assert!(
        report.completed() && report.committed(),
        "a zero-agent activation walks to its emit and completes: outcome = {:?}",
        report.outcome()
    );
    assert_eq!(harness.dispatches(), 0, "nothing reaches the Harness");
    let events = journal(&db);
    let rows = committed_rows(&events);
    assert_eq!(rows.len(), 1, "exactly the completion row: {rows:?}");
    assert!(is_completion_row(rows[0]));
    assert_eq!(
        events
            .iter()
            .filter(|e| e.kind == EventKind::Dispatch)
            .count(),
        0,
        "and no dispatch row"
    );
    assert!(
        fold(ITEM, STAGE, &events)
            .expect("folds")
            .is_committed(ITEM, AID)
    );
}

/// **The commit key carries the step, and `IdempotencyKey` keeps three parts** (D21-43 N15).
///
/// The three-part literal and the exhaustive destructure below fail to COMPILE if the memo key
/// grows a fourth field, which is ADR-0097's forbids as a type fact.
#[test]
fn the_commit_key_carries_the_step_and_the_idempotency_key_keeps_three_parts() {
    let three = IdempotencyKey {
        item: ITEM.to_owned(),
        activity_id_at_version: AID.to_owned(),
        attempt: 1,
    };
    let IdempotencyKey {
        item: _,
        activity_id_at_version: _,
        attempt: _,
    } = three;

    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(SpecEchoHarness::scripted(vec![
        Some(serde_json::json!({ "draft": 1 })),
        Some(serde_json::json!({ "review": 2 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-keys",
        STAGE,
    );
    let node = spec();
    let specs: BTreeMap<String, ActivitySpec> = [
        ("draft".to_owned(), step_spec("aa", "ImplReport/v1")),
        ("review".to_owned(), step_spec("bb", "ImplReport/v1")),
    ]
    .into_iter()
    .collect();
    let steps = vec![
        agent("draft", "implementer"),
        agent("review", "reviewer"),
        emit(),
    ];
    rt.run_activity_steps(
        activation(Some(&node), Some(&specs)),
        Some(&steps),
        &Context::new(),
        gate_site(&state),
        AttemptCap::declared(CAP),
        at(100),
    )
    .expect("resolves");
    let events = journal(&db);
    let rows = committed_rows(&events);
    let first = commit_key_of(rows[0]).unwrap_or_else(|| {
        panic!(
            "the first agent step's commit row carries no commit key (step_id + iteration_path): \
             {:?}",
            rows[0]
        )
    });
    assert_eq!(first, key("draft", &IterationPath::root()));
    // Every row still carries exactly the memo key's three parts beside the two new members, and
    // `(item, activity_id@version, attempt)` stays unique across the log (AC-011): the completion
    // row takes the next attempt, never a reused one.
    let mut triples: Vec<(String, String, u64)> = rows
        .iter()
        .map(|e| {
            (
                e.payload["item"].as_str().expect("item").to_owned(),
                e.payload["activity_id_at_version"]
                    .as_str()
                    .expect("activation")
                    .to_owned(),
                e.payload["attempt"].as_u64().expect("attempt"),
            )
        })
        .collect();
    let n = triples.len();
    triples.sort();
    triples.dedup();
    assert_eq!(
        triples.len(),
        n,
        "(item, activity_id@version, attempt) is unique across the committed rows"
    );
    assert_eq!(n, 3);
}

/// **The plan walk moves past a node only once its EMIT row is journalled** (R1; M5): a node whose
/// first agent step committed and whose completion is absent is dispatched again, and the
/// completion row moves the plan on.
#[test]
fn the_plan_walk_moves_past_a_node_only_once_its_emit_row_is_journalled() {
    let planned = crate::fsm::planned();
    let order: Vec<String> = planned
        .plan()
        .expect("the assembled DAG orders")
        .order()
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    assert!(
        order.len() >= 2,
        "PREMISE: a second node to move on to: {order:?}"
    );
    let first = planned
        .dag()
        .nodes()
        .iter()
        .find(|n| n.id == order[0])
        .expect("in the DAG")
        .activity_id
        .clone();
    let ctx = FsmContext::supplied(
        ITEM,
        "aaaa111122223333444455556666777788889999",
        rk_types::time::Timestamp::from_unix_seconds(9_000),
        None,
    );
    let step_row = Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "item": ITEM, "activity_id_at_version": first, "attempt": 1,
            "step_id": "draft", "iteration_path": [], "at": 100, "run_id": "r", "result": {},
            "schema": "ImplReport/v1",
        }),
    );
    let part = fold(ITEM, STAGE, std::slice::from_ref(&step_row)).expect("folds");
    match next_action(&planned, &part, &ctx).expect("computes") {
        NextAction::Dispatch { step_id, .. } => assert_eq!(
            step_id, order[0],
            "a node whose first agent step committed and whose emit row is absent is NOT moved \
             past: the plan dispatches it again"
        ),
        other => panic!("expected the same node's Dispatch, got {other:?}"),
    }
    let completion = Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "item": ITEM, "activity_id_at_version": first, "attempt": 2,
            "step_id": "emit", "iteration_path": [], "completion": true, "at": 100,
            "run_id": "r",
        }),
    );
    let whole = fold(ITEM, STAGE, &[step_row, completion]).expect("folds");
    match next_action(&planned, &whole, &ctx).expect("computes") {
        NextAction::Dispatch { step_id, .. } => assert_eq!(
            step_id, order[1],
            "the completion row moves the plan past the node"
        ),
        other => panic!("expected the next node's Dispatch, got {other:?}"),
    }
}

/// **A `side_effecting` tool step commits under the commit key and is SERVED on re-entry, never
/// re-executed** (R1; M8). The control is the same shape over an idempotent verb, which DOES
/// re-execute and commits nothing.
#[test]
fn a_side_effecting_tool_step_commits_and_is_served_on_re_entry_never_re_executed() {
    let registry = VerbRegistry::from_toml(
        "[verb]\n\
         mint = { outputs = \"ItemRefs\", live = false, timeout = \"build_timeout\", inputs = [], \
         route = \"toolchain\", side_effecting = true }\n\
         look = { outputs = \"ItemRefs\", live = false, timeout = \"build_timeout\", inputs = [], \
         route = \"toolchain\", side_effecting = false }\n",
    )
    .expect("the test registry loads");
    let facets = shipped_registry();
    // The `stop` check runs (and fails) under `optional` and is guarded off under `mandatory`, so
    // the first walk dies AFTER the tool step and the second walks to its emit.
    let stop = Step {
        expr: Some("false".to_owned()),
        when: Some("rigor < mandatory".to_owned()),
        ..step("stop", StepKind::Check)
    };
    for (verb, side_effecting) in [("mint", true), ("look", false)] {
        let tool = Step {
            run: Some(format!("toolchain:{verb}")),
            ..step("record", StepKind::Tool)
        };
        let steps = vec![tool, stop.clone(), emit()];
        let none: BTreeMap<String, ActivitySpec> = BTreeMap::new();
        let (q, a) = counters();
        let db = store(&q, &a);
        let output = VerbOutput {
            kind: "ItemRefs".to_owned(),
            value: serde_json::json!({ "ids": ["STORY-9"] }),
        };
        let walk = |optional: bool| {
            let tc = Arc::new(CountingToolchain::with_verb_output(output.clone()));
            let rt = Runtime::new(
                ports_with_toolchain(
                    Arc::new(SpecEchoHarness::scripted(Vec::new())) as Arc<dyn Harness>,
                    &db,
                    Arc::clone(&tc) as Arc<dyn Toolchain>,
                ),
                "run-side-effecting",
                STAGE,
            )
            .with_tool_routing(ToolRouting::new(
                registry.clone(),
                ToolchainValues::unselected("no table in this test"),
            ));
            let rigor = if optional {
                Rigor::Optional
            } else {
                Rigor::Mandatory
            };
            let ctx = rk_kernel::rigor::with_rigor(Context::new(), rigor, &facets);
            let state = empty_state();
            let report = rt
                .run_activity_steps(
                    activation(None, Some(&none)),
                    Some(&steps),
                    &ctx,
                    gate_site(&state),
                    AttemptCap::declared(CAP),
                    at(100),
                )
                .expect("resolves");
            (report, tc.calls())
        };
        let (r1, calls1) = walk(true);
        assert_eq!(
            calls1, 1,
            "PREMISE ({verb}): the first walk runs the tool step once"
        );
        assert!(
            !r1.completed(),
            "PREMISE ({verb}): the first walk dies at `stop`: {r1:?}"
        );
        let (r2, calls2) = walk(false);
        assert!(
            r2.completed(),
            "({verb}) the second walk reaches its emit: {r2:?}"
        );
        let events = journal(&db);
        let tool_rows = committed_rows(&events)
            .iter()
            .filter(|e| commit_key_of(e) == Some(key("record", &IterationPath::root())))
            .count();
        if side_effecting {
            assert_eq!(
                calls2, 0,
                "a `side_effecting` tool step is SERVED on re-entry and NEVER re-executed — a \
                 second run of it would repeat a write (R1; M8)"
            );
            assert_eq!(r2.served(), 1, "the tool step is served from the journal");
            assert_eq!(
                tool_rows, 1,
                "the side-effecting tool step committed ONE row under its commit key"
            );
            assert_eq!(
                committed_step_result(&events, &key("record", &IterationPath::root())),
                Some(&serde_json::json!({ "ids": ["STORY-9"] })),
                "the committed row carries the verb's output"
            );
        } else {
            assert_eq!(
                calls2, 1,
                "CONTROL: an idempotent tool step re-executes on re-entry (ADR-0150 decision 2's \
                 residual, closed by re-execution)"
            );
            assert_eq!(
                tool_rows, 0,
                "CONTROL: an idempotent tool step commits nothing"
            );
        }
    }
}
