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
    }
}

fn activation<'a>(sp: &'a rk_types::activity::ActivitySpec) -> Activation<'a> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: sp,
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
