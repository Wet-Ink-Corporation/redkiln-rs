//! **ADR-0145: a catalog `gate` step opens the SAME gate the selection-level path opens.**
//!
//! `PROGRESS.md` row 37's finding (4). The walk's `Gate` arm `break`ed beside `Emitted` and wrote
//! **nothing**, and the comment beside it said the mark was *"written by `Runtime::step`'s
//! `OpenGate` arm"*. That was false for a catalog gate step: `OpenGate` is only ever reached from
//! `selection.gates()` — the abstention, unsatisfiable and appetite gates — and nothing in the tree
//! connected the pack's **80** `[[step]] kind = "gate"` to it.
//!
//! Measured over `.redkiln/catalog/` at this repair:
//!
//! | reading | count |
//! | --- | --- |
//! | `gate` steps in the pack | **80** (all declaring a `question`; **37** distinct step ids) |
//! | activities with a gate AFTER the first agent step — silently skipped | **55** |
//! | activities with a gate BEFORE it — given the `emit` refusal, which is the wrong message | **2** |
//! | activities with gates and NO agent step at all | **13** |
//!
//! # 🛑 The two halves the pack actually contains, and both are asserted
//!
//! A gate **after** a committed dispatch (55 activities) and a gate **before** any dispatch (15).
//! They take different branches — `with_gate` on the committed report, `gated` on a fresh one — and
//! a fix that handled only the first would leave 15 activities exactly as broken as before.
//!
//! # What makes each assertion non-vacuous
//!
//! - The row must be the **token-bearing** one. `select()`'s announcement rows share the
//!   `gate_opened` kind and carry `{reason, axis, site}` with no token; `fsm::apply` tells the two
//!   producers apart by `tree_hash`/`deadline`, and only the runtime-shaped row moves state. A
//!   walk that appended the announcement shape would look like it worked and gate nothing.
//! - The token on the report must be the token in the journal. Two independently-derived values
//!   that happen to agree is the two-written-to-agree defect; these are asserted equal.
//! - A gate whose guard does **not** hold must write **nothing**. `StepVerdict::Skipped` is only
//!   reachable at all because ADR-0142 landed, and without this control the arm would be satisfied
//!   by one that opens a gate unconditionally.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::registry::Registry;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::Activation;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::harness::Harness;
use rk_types::event_kind::EventKind;

use crate::runtime::{
    AID, CAP, ITEM, STAGE, STEP, at, counters, empty_state, gate_site, ok_typed, ports, spec, store,
};

fn agent_step(id: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Agent,
        run: None,
        output_type: None,
        role: Some("implementer".to_owned()),
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: None,
        inputs: std::collections::BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    }
}

/// A `gate` step. `question` is the gate's KIND — the key the loader dropped until ADR-0145.
fn gate_step(id: &str, question: &str, when: Option<&str>) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Gate,
        run: None,
        output_type: None,
        role: None,
        when: when.map(ToOwned::to_owned),
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: Some(question.to_owned()),
        timeout_secs: None,
        inputs: std::collections::BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    }
}

fn activation<'a>(sp: &'a rk_types::activity::ActivitySpec) -> Activation<'a> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: Some(sp),
        // (dated 2026-09-26, s8d build, P3) R1: no per-step specs, so the first runnable agent
        // step dispatches under the activation's spec (ADR-0143 decision 2 as R1 narrows it).
        step_specs: None,
        rigor: rk_kernel::rigor::Rigor::Optional,
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

/// Every `gate_opened` row in the item's journal that carries a token — the RUNTIME's shape.
fn token_rows(db: &std::sync::Arc<std::sync::Mutex<dyn EventStore>>) -> Vec<String> {
    db.lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("the journal is readable")
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter_map(|e| {
            e.payload
                .get("token")
                .and_then(serde_json::Value::as_str)
                .map(ToOwned::to_owned)
        })
        .collect()
}

/// **A gate AFTER a committed dispatch — 55 of the shipped pack's activities.**
#[test]
fn a_gate_step_after_a_dispatch_writes_the_runtime_mark_and_keeps_the_commit() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-gate-after",
        STAGE,
    );
    let state = empty_state();
    let sp = spec();

    // PREMISE: nothing has opened a gate yet, so the assertion below counts only this walk's row.
    assert!(
        token_rows(&db).is_empty(),
        "PREMISE: the journal already carries a token-bearing gate row before the walk ran"
    );

    let steps = vec![
        agent_step("draft"),
        gate_step("accept", "draft.accept", None),
    ];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

    // (1) THE REPORT SAYS A GATE OPENED.
    let token = report.gate_opened().unwrap_or_else(|| {
        panic!(
            "the walk reached a `gate` step and reported no gate. Before ADR-0145 this arm \
             `break`ed and wrote nothing, so `dispatch_through` saw an ordinary report, the fold \
             saw no mark, and the gate was SILENTLY SKIPPED — 55 of 102 shipped activities. \
             Report: {report:?}"
        )
    });

    // (2) AND THE COMMIT SURVIVES. A gate is not a failure and does not undo a dispatch.
    assert!(
        report.committed(),
        "the dispatch's commit was lost when the gate opened: {report:?}"
    );

    // (3) AND THE ROW IN THE JOURNAL IS THE RUNTIME'S, CARRYING THIS EXACT TOKEN.
    let rows = token_rows(&db);
    assert_eq!(
        rows.len(),
        1,
        "expected exactly one token-bearing `gate_opened` row, found {}: {rows:?}. \
         `select()`'s announcement rows share this kind and carry NO token; a walk that wrote one \
         of those would gate nothing while looking like it worked",
        rows.len()
    );
    assert_eq!(
        rows[0],
        token.as_str(),
        "the token in the journal is not the token the report carries. Two independently derived \
         values that happen to agree is the two-written-to-agree defect; they must be one value"
    );

    println!(
        "gate_after_dispatch: token={} committed={}",
        rows[0],
        report.committed()
    );
}

/// **A gate BEFORE any dispatch — 15 of the shipped pack's activities (13 with no agent step at
/// all, 2 with the gate ahead of one).**
#[test]
fn a_gate_step_before_any_dispatch_opens_the_gate_and_does_not_refuse() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-gate-before",
        STAGE,
    );
    let state = empty_state();
    let sp = spec();

    let steps = vec![
        gate_step("accept", "draft.accept", None),
        agent_step("draft"),
    ];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the walk completes");

    assert!(
        report.gate_opened().is_some(),
        "a gate reached BEFORE any dispatch opened nothing: {report:?}"
    );
    // The `emit` refusal is what this used to get, and it named the wrong mechanism entirely.
    let refusal = report.outcome().and_then(|o| o.as_ref().err());
    assert!(
        refusal.is_none(),
        "a gate before any dispatch was REFUSED. Before ADR-0145 the walk fell through to the \
         \"no `agent` step produced a value\" arm, whose message is about `emit` binding outputs \
         and the reference resolver — a refusal naming a mechanism that has nothing to do with \
         gates: {refusal:?}"
    );
    assert!(
        !report.committed(),
        "a gate does not commit its activation. design §4.5: the kernel writes the mark and the \
         run EXITS 0, resuming into this same step: {report:?}"
    );
    assert_eq!(
        harness.dispatches(),
        0,
        "the gate ends the walk, so the agent step after it must NOT have dispatched"
    );
    assert_eq!(token_rows(&db).len(), 1, "exactly one runtime gate row");
}

/// **THE CONTROL: a gate whose guard does not hold opens NOTHING.**
///
/// Without it, every assertion above is satisfied by an arm that opens a gate unconditionally.
/// `StepVerdict::Skipped` is reachable at all only because ADR-0142 gave the walk a real context —
/// an adversarial pass found it unreachable by construction before that.
#[test]
fn a_guarded_off_gate_step_opens_nothing() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-gate-guarded-off",
        STAGE,
    );
    let state = empty_state();
    let sp = spec();

    // `rigor` is bound as an ordinal, exactly as `replay` binds it. At `optional`, a guard reading
    // `rigor >= mandatory` is FALSE — not unresolvable, which would be `Failed` and prove nothing.
    let registry = shipped_registry();
    let ctx = rk_kernel::rigor::with_rigor(
        rk_kernel::predicate::Context::new(),
        Rigor::Optional,
        &registry,
    );

    let steps = vec![
        agent_step("draft"),
        gate_step("accept", "draft.accept", Some("rigor >= mandatory")),
    ];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &ctx,
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the walk completes");

    assert!(
        report.gate_opened().is_none(),
        "CONTROL: a gate whose `when` does not hold OPENED. The arm is firing unconditionally, so \
         every assertion in this module is satisfied by a constant: {report:?}"
    );
    assert!(
        token_rows(&db).is_empty(),
        "CONTROL: a guarded-off gate appended a runtime gate row: {:?}",
        token_rows(&db)
    );
    assert!(
        report.committed(),
        "CONTROL: the walk should have run to completion past the skipped gate: {report:?}"
    );
}

/// **A `gate` step with no `question` has no KIND, and is refused BY NAME.**
///
/// All 80 shipped gate steps declare one, so this guards a catalog that changes rather than a state
/// the pack is in. Without it, a missing `question` would have to be defaulted — and a token
/// derived over an invented kind binds an answer to a gate nobody declared.
#[test]
fn a_gate_step_without_a_question_is_refused_by_name() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-gate-no-question",
        STAGE,
    );
    let state = empty_state();
    let sp = spec();

    let mut bad = gate_step("accept", "unused", None);
    bad.question = None;
    let steps = vec![bad];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the walk completes");

    let err = report
        .outcome()
        .and_then(|o| o.as_ref().err())
        .unwrap_or_else(|| panic!("a `gate` step with no `question` was not refused: {report:?}"));
    let rendered = format!("{err:?}");
    for expected in ["accept", "question", "ADR-0145"] {
        assert!(
            rendered.contains(expected),
            "the refusal does not name {expected:?}: {rendered}"
        );
    }
    assert!(
        report.gate_opened().is_none(),
        "a gate with no kind still opened one: {report:?}"
    );
    assert!(
        token_rows(&db).is_empty(),
        "a gate with no kind appended a row"
    );
}

/// Fold a journal into a `RunState`, so a test can put a gate step's own history behind it.
///
/// `RunState::initial` is `pub(in crate::workflow)` and unreachable from an integration test, so
/// the state is obtained the way production obtains it — by folding. That is also what makes these
/// tests real: they assert over the same fold `replay`'s loop hands the walk.
fn state_after(events: &[rk_types::event::Event]) -> rk_kernel::workflow::fsm::RunState {
    rk_kernel::workflow::fsm::fold(ITEM, STAGE, events).expect("the journal folds")
}

/// The journal a gate leaves behind once it has been opened and ANSWERED.
fn opened_and_answered(kind: &str) -> Vec<rk_types::event::Event> {
    let record = rk_kernel::workflow::gate::reach(
        &state_after(&[]),
        kind,
        &rk_kernel::workflow::fsm::FsmContext::supplied(
            ITEM,
            crate::runtime::TREE_HASH,
            rk_types::time::Timestamp::from_unix_seconds(9_000),
            None,
        ),
    )
    .expect("the record derives");
    vec![
        rk_kernel::runtime::gate_opened_event(&record),
        rk_kernel::workflow::gate::gate_answered_event(
            record.token(),
            crate::runtime::TREE_HASH,
            "approve",
        ),
    ]
}

/// 🛑 **AN ANSWERED GATE STEP IS WALKED PAST, NOT RE-OPENED — ADR-0146 clause 2.**
///
/// The first version of this arm called `gate::reach` unconditionally, and `gate::reach` mints at
/// `reaches(kind) + 1`. So an answered gate re-opened at reach 2, then 3, for ever. Measured
/// through the real binary over three run→answer rounds: three `gate_opened` rows, three distinct
/// tokens, and **never a `Dispatch`**.
///
/// It was invisible because every other test in this file passes `empty_state()`, where
/// `reaches(kind)` is `0` and the arm's only reachable branch is the one that opens. **A guard whose
/// fixtures all sit at reach 1 cannot see a re-reach defect at all.**
#[test]
fn an_answered_gate_step_is_walked_past_and_not_re_opened() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-gate-answered",
        STAGE,
    );
    let sp = spec();
    let history = opened_and_answered("draft.accept");
    let state = state_after(&history);

    // PREMISE: the fold really did record a reach, or the assertion below is about reach 0 again.
    assert_eq!(
        state.reaches("draft.accept"),
        1,
        "PREMISE: the history did not fold to one reach, so this test is still at reach 0 and \
         asserts nothing about a RE-reach"
    );

    // The gate comes FIRST, so if it re-opened the agent step could never run. That is exactly the
    // shape of the 15 shipped activities that gate before dispatching.
    let steps = vec![
        gate_step("accept", "draft.accept", None),
        agent_step("draft"),
    ];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            rk_kernel::runtime::step::GateSite {
                state: &state,
                tree_hash: crate::runtime::TREE_HASH,
                deadline: rk_types::time::Timestamp::from_unix_seconds(9_000),
            },
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the walk completes");

    assert!(
        report.gate_opened().is_none(),
        "an ANSWERED gate step opened AGAIN. `gate::reach` mints at `reaches + 1`, so this is a \
         fresh token at reach 2 and the next run will mint reach 3: the activation never commits, \
         `is_committed` never skips its node, and the walk returns to this step for ever. 15 \
         shipped activities gate before any dispatch and would be permanently unexecutable: \
         {report:?}"
    );
    assert!(
        token_rows(&db).is_empty(),
        "an answered gate step appended a SECOND runtime gate row: {:?}",
        token_rows(&db)
    );
    assert!(
        report.committed(),
        "the walk did not proceed past the answered gate to the agent step behind it: {report:?}"
    );
    assert_eq!(
        harness.dispatches(),
        1,
        "the step behind an answered gate must actually run — that is the whole point of \
         answering it"
    );
}

/// **A VOIDED gate step RE-OPENS, at reach n+1 — AC-252, mirrored from `fsm::next_action`.**
///
/// The control for the test above: without it, "an answered gate is walked past" is satisfied by an
/// arm that never opens a gate a second time under any circumstance, which would break AC-252's
/// void-then-re-open contract instead.
#[test]
fn a_voided_gate_step_re_opens_at_the_next_reach() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-gate-void",
        STAGE,
    );
    let sp = spec();

    let record = rk_kernel::workflow::gate::reach(
        &state_after(&[]),
        "draft.accept",
        &rk_kernel::workflow::fsm::FsmContext::supplied(
            ITEM,
            crate::runtime::TREE_HASH,
            rk_types::time::Timestamp::from_unix_seconds(9_000),
            None,
        ),
    )
    .expect("the record derives");
    let history = vec![
        rk_kernel::runtime::gate_opened_event(&record),
        rk_kernel::workflow::gate::gate_answer_void_event(
            record.token(),
            crate::runtime::TREE_HASH,
            "ffffffffffffffffffffffffffffffffffffffff",
        ),
    ];
    let state = state_after(&history);
    assert!(
        state.is_void(record.token()),
        "PREMISE: the fold did not record the void, so the arm below is being asked the wrong \
         question"
    );

    let steps = vec![gate_step("accept", "draft.accept", None)];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            rk_kernel::runtime::step::GateSite {
                state: &state,
                tree_hash: crate::runtime::TREE_HASH,
                deadline: rk_types::time::Timestamp::from_unix_seconds(9_000),
            },
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the walk completes");

    let token = report.gate_opened().unwrap_or_else(|| {
        panic!(
            "a VOIDED gate step did not re-open. AC-252 is `a void is followed by a re-open, fresh \
             in all three fields`, and `fsm::next_action` clause 2 executes it; this arm must \
             mirror it or a voided step gate can never be answered again: {report:?}"
        )
    });
    assert_ne!(
        token,
        record.token(),
        "the re-open reused the VOIDED token. The reach is a member of the digest, so a re-open \
         must be fresh (AC-524, AC-246)"
    );
    assert_eq!(token_rows(&db).len(), 1, "exactly one new runtime gate row");
}
/// A `tool` step naming `verb`, guarded by `when`.
fn guarded_tool(id: &str, verb: &str, when: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Tool,
        run: Some(format!("toolchain:{verb}")),
        output_type: None,
        role: None,
        when: Some(when.to_owned()),
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: None,
        inputs: std::collections::BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    }
}

/// 🛑 **THE WALK CONTINUES PAST A NON-AGENT STEP — asserted here because nothing else asserts it.**
///
/// An adversarial pass measured that adding
///
/// ```ignore
/// if step.kind != StepKind::Agent { break; }
/// ```
///
/// to the end of the walk body survives the **whole** Checkpoint 7g block *and* the whole
/// `rk-kernel` integration suite. Every other fixture in this file and in `post_commit_failure.rs`
/// puts its non-agent step **last**, so "stop after it" and "run to the end" are observationally
/// identical — the same blindness as a gate fixture that only ever sits at reach 1, and as six
/// checkpoint filters that never enter the walk. **Where a fixture puts the interesting step is
/// part of what it asserts.**
///
/// The middle step is SKIPPED rather than run, so this stays a test about the walk's control flow
/// and not about any port: `rigor` is bound at `optional`, the guard reads `rigor >= mandatory`,
/// and ADR-0142 is what makes that evaluate to `false` instead of erroring.
#[test]
fn the_walk_continues_past_a_skipped_step_to_the_gate_behind_it() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-walk-continues",
        STAGE,
    );
    let state = empty_state();
    let sp = spec();
    let registry = shipped_registry();
    let ctx = rk_kernel::rigor::with_rigor(
        rk_kernel::predicate::Context::new(),
        Rigor::Optional,
        &registry,
    );

    // agent -> a SKIPPED tool -> gate. The gate is third, so a walk that stopped at the first
    // non-agent step would never reach it.
    let steps = vec![
        agent_step("draft"),
        guarded_tool("publish-it", "publish", "rigor >= mandatory"),
        gate_step("accept", "draft.accept", None),
    ];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &ctx,
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the walk completes");

    // PREMISE: the middle step really was SKIPPED, not run. `publish` is one of ADR-0135's 29
    // refused verbs, so if the guard had not held it off the walk would have ended with its name.
    assert!(
        report.step_error().is_none(),
        "PREMISE: the middle step was EXECUTED, not skipped — the guard did not hold it off, so          this test is about a failed walk rather than about continuing past a skipped step: {:?}",
        report.step_error()
    );

    assert!(
        report.gate_opened().is_some(),
        "the walk did not reach the gate BEHIND the skipped step. A walk that stops at the first          non-agent step passes every other test in this file, because they all put their          non-agent step last: {report:?}"
    );
    assert!(
        report.committed(),
        "the dispatch's commit was lost: {report:?}"
    );
    assert_eq!(token_rows(&db).len(), 1, "exactly one runtime gate row");
}
