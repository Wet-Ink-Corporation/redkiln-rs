//! **ADR-0144: a step that fails AFTER a committed dispatch is not dropped on the floor.**
//!
//! `PROGRESS.md` row 37's finding (3). The walk's `Failed` arm read
//! `dispatched.unwrap_or_else(|| refused(now, e))`, which discards `e` outright whenever a dispatch
//! had already happened. `replay::dispatch_through`'s only question is `committed()`, the committed
//! report answers `true`, and the run therefore continued — or exited **0** — having executed
//! nothing after the dispatch and said so nowhere.
//!
//! That voids **ADR-0135** at the last step. 113 `tool` steps name 33 verbs and 29 of them are
//! refused **by name**, for one stated reason: *"a silent fallthrough would turn 66 of 113 steps
//! into invisible non-events, and a run that executed nothing would be indistinguishable from one
//! that worked."* The refusal was raised correctly and then thrown away one level up.
//!
//! # The population, re-measured at this repair
//!
//! Row 37 states **38 of 102**, which was measured **before ADR-0143** changed where a walk ends. A
//! second runnable `agent` step now ends it, so the reachable population is smaller and is
//! re-derived here rather than restated:
//!
//! | reading | count |
//! | --- | --- |
//! | any non-agent step after the first agent step | **82 of 102** |
//! | reaches a `tool`/`check` step before the walk ends | **33 of 102** |
//! | …and that step CANNOT succeed on this tree today | **29 of 102** |
//!
//! # 🛑 Three assertions, because each alone is satisfied by a wrong fix
//!
//! 1. **The failure is carried, by name.** A fix that dropped it is the defect.
//! 2. **The commit SURVIVES.** A fix that returned `refused(now, e)` unconditionally would also
//!    carry the failure — and would report `committed() == false` for an activation whose
//!    `activity_committed` row is in the journal, which is ADR-0143's lie wearing the other sign.
//! 3. **The dispatch's own outcome is UNTOUCHED.** A fix that wrote the step's error into
//!    `outcome` would satisfy (1) and (2) and destroy the dispatch's reading — ADR-0143's defect
//!    one field over. `outcome()` must still be the `Ok(typed)` the harness returned.
//!
//! # 🛑 What this does NOT cover, corrected 2026-09-22 — this header once claimed the opposite
//!
//! It read: *"The mapping from `ReplayError::StepFailedAfterCommit` to an exit row lives in
//! `rk-cli` and **is asserted there**."* **It was not.** An adversarial pass measured that
//! `StepFailedAfterCommit` appeared on four source lines and **zero test lines**, and that two of
//! ADR-0144's three edits were protected by nothing:
//!
//! | mutation | workspace | exit |
//! | --- | --- | --- |
//! | delete `dispatch_through`'s `step_error()` check | **GREEN** | **0** |
//! | the `run.rs` arm returns `Exit::ConfigError` | **GREEN** | **3** |
//! | the pre-ADR-0144 `Failed` one-liner | RED — *this* file | 0 |
//!
//! **This file tests `ActivityReport` and nothing else.** The report may carry `step_error` and
//! every caller may ignore it. A header sentence asserting coverage that does not exist is worse
//! than silence, because it is exactly what a reader checks instead of the test list.
//!
//! The mapping **is** asserted now, end to end, by
//! `rk-cli`'s `exit::adr0144_post_commit_step_failure_exits_the_class_row_not_three` — written in
//! the same change that corrected this sentence, which is the only honest order.
//!
//! The walk is still entered **directly** here, as `second_agent.rs` enters it; nothing in this
//! file spawns a binary.
//!
//! ⚠️ **And one assertion of the three below is weaker than it reads.** The claim that each is
//! satisfied by a wrong fix the other two reject is FALSE: an *attach-and-continue* fix — one that
//! calls `with_step_failure` but does not `return` — passes all three, the control, and the whole
//! workspace, because this walk is only two steps long and "return" and "fall through" are
//! observationally identical at its end. `StepVerdict::Failed`'s own docstring says *"The walk ends
//! and this is the activation's error"*; **nothing here asserts that it ends.**

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::Activation;
use rk_ports::harness::Harness;

use crate::runtime::{
    AID, CAP, ITEM, STAGE, STEP, at, counters, empty_state, gate_site, ok_typed, ports, spec, store,
};

/// An `agent` step: the one kind that dispatches.
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
    }
}

/// A `tool` step naming `verb`. **`publish` is one of ADR-0135's 29 refused verbs** — it is not in
/// `MAPPED_VERBS`, so `run_tool_step` refuses it by name without reaching any port.
/// *(dated 2026-09-26, s8c refuting round 6.)* `MAPPED_VERBS` is gone: routing is the verb
/// registry's rows (record V = ADR-0170, proposed). `publish` has no row, and a verb with no row,
/// or a runtime with no registry bound, refuses by its bare name before any port; nothing here
/// moves.
fn tool_step(id: &str, verb: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Tool,
        run: Some(format!("toolchain:{verb}")),
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

/// **The defect: the refusal of a post-commit step must reach the caller, and the commit must
/// stand.**
#[test]
fn a_step_failing_after_a_committed_dispatch_refuses_by_name_and_keeps_the_commit() {
    // ADR-0145: the walk now takes a `GateSite`. These tests reach no `gate` step, so the
    // state is an EMPTY fold and the site is never used — but it is real rather than stubbed,
    // so a test that later does reach a gate gets a true reach counter.
    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-post-commit-failure",
        STAGE,
    );

    // ---- THE CONTROL FIRST, so a failure of the assertion below cannot read as a pass. ---------
    //
    // The SAME walk with the failing step removed: it commits, and it carries NO step failure. If
    // this does not hold, every assertion below is about a walk that was broken anyway.
    let sp = spec();
    let clean = vec![agent_step("draft")];
    let control = rt
        .run_activity_steps(
            activation(&sp),
            Some(&clean),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    assert!(
        control.committed(),
        "CONTROL: the bare agent walk did not commit, so nothing below is about a POST-commit \
         failure: {control:?}"
    );
    assert!(
        control.step_error().is_none(),
        "CONTROL: a walk with no failing step reported one. `step_error` would then be satisfied \
         by a constant and the assertion below would prove nothing: {:?}",
        control.step_error()
    );

    // ---- THE DEFECT'S OWN SHAPE: dispatch, commit, THEN a refused verb. ------------------------
    let (q2, a2) = counters();
    let db2 = store(&q2, &a2);
    let harness2 = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&harness2) as Arc<dyn Harness>, &db2),
        "run-post-commit-failure-2",
        STAGE,
    );
    // 🛑 **THREE STEPS, NOT TWO, AND THE THIRD IS LOAD-BEARING.** With `[agent, tool]` the walk
    // ends immediately after the failing step anyway, so `return` and `fall through` are
    // OBSERVATIONALLY IDENTICAL and an attach-and-continue fix — one that calls
    // `with_step_failure` without returning — passes every assertion below. Measured: it did.
    // A second unmapped verb behind the first makes the difference visible, because a walk that
    // did not end would reach it and OVERWRITE the error with `deploy`.
    let steps = vec![
        agent_step("draft"),
        tool_step("publish-it", "publish"),
        tool_step("deploy-it", "deploy"),
    ];
    let report = rt2
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(200),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

    // (1) THE FAILURE IS CARRIED, BY NAME.
    let class = report.step_error().unwrap_or_else(|| {
        panic!(
            "the walk carried NO failure for a `tool` step naming one of ADR-0135's 29 refused \
             verbs, because a dispatch had already committed. `dispatch_through` asks only \
             `committed()`, so this run reports a SUCCESS having executed nothing after the \
             dispatch — the invisible non-event ADR-0135 exists to make impossible. Report: \
             {report:?}"
        )
    });
    let rendered = format!("{class:?}");
    assert!(
        rendered.contains("publish"),
        "the refusal does not NAME the verb it refused, so the journal cannot say which of the 29 \
         was reached (ADR-0135 clause 2): {rendered}"
    );
    // (1b) AND THE WALK ENDED THERE. `StepVerdict::Failed`'s own docstring is "The walk ends and
    // this is the activation's error"; without this the arm may attach the error and CONTINUE,
    // which passes every other assertion in this test and is what an adversarial pass exploited.
    assert!(
        !rendered.contains("deploy"),
        "the walk did NOT end at the failing step: it carried on to `deploy-it` and overwrote the \
         error. `StepVerdict::Failed` means the walk ends — an executor that attaches a failure \
         and keeps going reports the LAST failure instead of the first, and re-opens the gate-arm \
         masking that `dispatch_through`'s ordering now guards against: {rendered}"
    );

    // (2) AND THE COMMIT SURVIVES.
    assert!(
        report.committed(),
        "the activation's commit was discarded to report the later failure. The \
         `activity_committed` row is in the journal and AC-013's grain is per ACTIVATION \
         (ADR-0138), so reporting `committed() == false` here is false — and it is exactly the \
         shape ADR-0143 refused, with the sign reversed: {report:?}"
    );

    // (3) AND THE DISPATCH'S OWN OUTCOME IS UNTOUCHED.
    match report.outcome() {
        Some(Ok(_)) => {}
        other => panic!(
            "the dispatch's own outcome is no longer its returned value ({other:?}). Writing the \
             later step's error into `outcome` satisfies (1) and (2) and destroys the reading the \
             dispatch produced — ADR-0143's defect one field over, which is why `step_error` is a \
             second field."
        ),
    }

    assert_eq!(
        harness2.dispatches(),
        1,
        "exactly one dispatch: the `tool` step must not have reached the harness"
    );

    println!(
        "post_commit_failure: committed={} step_error={rendered} dispatches={}",
        report.committed(),
        harness2.dispatches()
    );
}
