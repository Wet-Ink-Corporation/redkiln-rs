//! **Two ways the step walk ENDS, both asserted by nothing until now** — `PROGRESS.md` row 37's
//! finding (21).
//!
//! | test | the semantic | the mutant it kills | what killed that mutant before |
//! | --- | --- | --- | --- |
//! | [`an_emit_step_ends_the_walk_and_a_later_tool_step_never_runs`] | `emit` is terminal (design §4.5) | `StepVerdict::Emitted => {}` | **nothing in the workspace** |
//! | [`an_empty_transcribed_step_list_is_refused_by_name_and_none_is_not_the_empty_case`] | ADR-0138's empty-list forbid | `if false && steps.is_empty()` | **nothing in the workspace** |
//!
//! 🛑 **WHY `emit` TERMINALITY NEEDS A COUNTING TOOLCHAIN AND CANNOT BE READ OFF THE REPORT.**
//! `StepVerdict::Emitted => break` leaves the loop and falls through to the same
//! `dispatched.unwrap_or_else(...)` the loop reaches when it runs off the end. Replace the `break`
//! with `{}` and the walk continues into the following step; if that step is a `tool` step naming a
//! MAPPED verb, `run_tool_step` returns `Ok(())`, the verdict is `Continue`, and the loop then falls
//! out at the very same place carrying the very same committed report. **Every field of the report
//! is identical under the mutant.** The only channel that distinguishes them is whether the port was
//! reached — which is why [`crate::runtime::CountingToolchain`] exists and why
//! `rk_ports::fakes::FixtureToolchain`, whose `build` is an unconditional `Ok(())` that counts
//! nothing, cannot serve here.
//!
//! 🛑 **WHY THE EMPTY-LIST HALF ASSERTS A SUBSTRING AND NOT `is_err()`.** Under
//! `if false && steps.is_empty()`, a `Some(&[])` walk runs a ZERO-iteration loop and falls to the
//! trailing refusal — *"walked its steps with no `agent` step producing a value"*. That is also an
//! `Err`. An assertion of the form *"the outcome is an error"* is therefore **GREEN on the mutant**
//! and proves nothing. Only the refusal's own text discriminates the two, so the test asserts the
//! ADR-0138 message is present **and** that the fall-through message is absent. Measured before the
//! assertion was written, not assumed.
//!
//! ⚠️ **`None` is not the empty case, and that is asserted rather than remembered.** `steps: None`
//! means *"this caller has no catalog activity behind it"* and falls through to
//! `Runtime::run_activity` one line before the walk — the fall-through that made guard (7b)'s
//! original `ac432` line green against a walk body that panics (finding (19)). A "fix" that refused
//! `None` alongside `Some(&[])` would pass the empty-list half below and break every machinery
//! suite in the crate, so the `None` half runs FIRST and is a control for the other.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::Activation;
use rk_ports::harness::Harness;
use rk_ports::toolchain::Toolchain;

use crate::runtime::{
    AID, CAP, CountingHarness, CountingToolchain, ITEM, STAGE, STEP, at, counters, empty_state,
    gate_site, ok_typed, ports_with_toolchain, spec, store,
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
    }
}

/// A `tool` step naming `toolchain:build`.
///
/// `build` is in ADR-0135's four-verb `MAPPED_VERBS` map and is the cheapest of the four — it takes
/// no selection and parses no report — so it is the step that DOES reach the port. That is the whole
/// of its job here: it is the instrument, placed after the step under test, and a verb outside the
/// map would be refused by name before the port and would move no counter.
fn build_step(id: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Tool,
        run: Some("toolchain:build".to_owned()),
        output_type: None,
        role: None,
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    }
}

/// An `emit` step.
///
/// Every field but `id` and `kind` is `None`: `catalog::Step` parses neither `artifacts` nor
/// `evidence`, and an `emit` binds its outputs to `provides`, which this loader does not read
/// either. So there is nothing else to set, and a fixture that set something would be asserting
/// about a key the parser drops.
fn emit_step(id: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Emit,
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
    }
}

const fn activation<'a>(sp: &'a rk_types::activity::ActivitySpec) -> Activation<'a> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: sp,
    }
}

/// One `Runtime` over a fresh store, a scripted one-dispatch harness and a COUNTING toolchain.
///
/// Returned alongside the two fakes because the assertions are readings taken OFF the fakes, and a
/// helper that dropped them would leave the caller asserting about the report alone — which is
/// exactly what cannot see the `emit` mutant.
fn rt_with_counters(run_id: &str) -> (Runtime, Arc<CountingHarness>, Arc<CountingToolchain>) {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let tc = Arc::new(CountingToolchain::default());
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::clone(&harness) as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        run_id,
        STAGE,
    );
    (rt, harness, tc)
}

/// **design §4.5: an `emit` is terminal. The walk ends there and a later step does not run.**
#[test]
fn an_emit_step_ends_the_walk_and_a_later_tool_step_never_runs() {
    let state = empty_state();
    let sp = spec();

    // ---- THE CONTROL FIRST. ---------------------------------------------------------------------
    //
    // The SAME trailing `tool` step at the SAME position, with no `emit` in front of it: it runs,
    // and it moves the counter to 1. Without this, the `0` asserted below is satisfied by a broken
    // `tool` arm, an unmapped verb, a port that was never bound, and a typo in `run` alike.
    let (rt, harness, tc) = rt_with_counters("run-emit-control");
    let control_steps = vec![agent_step("draft"), build_step("verify")];
    let control = rt
        .run_activity_steps(
            activation(&sp),
            Some(&control_steps),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    assert_eq!(
        tc.calls(),
        1,
        "CONTROL: a `tool` step at the END of a walk did not reach the toolchain port. The `0` \
         asserted below would then hold for a reason that has nothing to do with `emit` being \
         terminal: {control:?}"
    );
    assert!(
        control.committed(),
        "CONTROL: the agent step did not commit, so the walk below is not about a step AFTER a \
         commit: {control:?}"
    );
    assert_eq!(harness.dispatches(), 1, "CONTROL: one dispatch, not more");

    // ---- THE ASSERTION: the same walk with an `emit` inserted before the `tool` step. -----------
    let (rt2, harness2, tc2) = rt_with_counters("run-emit-terminal");
    let steps = vec![
        agent_step("draft"),
        emit_step("report"),
        build_step("verify"),
    ];
    let report = rt2
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

    assert_eq!(
        tc2.calls(),
        0,
        "the `tool` step AFTER the `emit` reached the toolchain port, so the walk did not end at \
         the `emit`. design §4.5 makes an `emit` terminal and `StepVerdict::Emitted` is handled by \
         a bare `break`; every field of this report is IDENTICAL whether that `break` is there or \
         not, so this counter is the only thing that can see it: {report:?}"
    );
    assert!(
        report.committed(),
        "the `emit` discarded the activation's commit. It is terminal, not a refusal — the walk \
         ends and the activation's provided kind stands: {report:?}"
    );
    assert!(
        report.step_error().is_none(),
        "the `emit` reported a step failure. Ending a walk is not failing it: {:?}",
        report.step_error()
    );
    assert_eq!(
        harness2.dispatches(),
        1,
        "the `emit` walk dispatched {} times rather than once",
        harness2.dispatches()
    );
}

/// **ADR-0138's forbid: an activation whose transcribed step list is EMPTY is refused, not walked
/// past — and `None` is a different thing that must keep working.**
#[test]
fn an_empty_transcribed_step_list_is_refused_by_name_and_none_is_not_the_empty_case() {
    let state = empty_state();
    let sp = spec();

    // ---- `None` FIRST, AND IT IS A CONTROL. ----------------------------------------------------
    //
    // `steps: None` is NOT the empty case: it falls through to `Runtime::run_activity` one line
    // before the walk, and that fall-through is the pre-`s7g` dispatch path every machinery suite
    // in this crate still takes. A "repair" that refused `None` alongside `Some(&[])` would satisfy
    // the half below and break all of them, so the distinction is asserted here rather than left to
    // be remembered.
    let (rt, harness, _tc) = rt_with_counters("run-empty-none");
    let none = rt
        .run_activity_steps(
            activation(&sp),
            None,
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    assert!(
        none.committed(),
        "CONTROL: `steps: None` did not reach the pre-`s7g` dispatch. It is the fall-through, not \
         the empty case, and every machinery suite in this crate depends on it: {none:?}"
    );
    assert_eq!(
        harness.dispatches(),
        1,
        "CONTROL: `steps: None` dispatched {} times rather than once",
        harness.dispatches()
    );

    // ---- `Some(&[])`: the forbid. ---------------------------------------------------------------
    let (rt2, harness2, tc2) = rt_with_counters("run-empty-list");
    let empty: Vec<Step> = Vec::new();
    let report = rt2
        .run_activity_steps(
            activation(&sp),
            Some(&empty),
            &Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

    assert_eq!(
        harness2.dispatches(),
        0,
        "an EMPTY step list dispatched. A zero-step walk is not a success and must not reach the \
         harness at all: {report:?}"
    );
    assert_eq!(
        tc2.calls(),
        0,
        "an EMPTY step list reached the toolchain port"
    );
    assert!(
        !report.committed(),
        "an EMPTY step list committed the activation — the vacuity ADR-0138 exists to end: \
         {report:?}"
    );

    let rendered = match report.outcome() {
        Some(Err(e)) => format!("{e:?}"),
        other => panic!(
            "an EMPTY step list did not refuse. ADR-0138 forbids it and `ActivityReport::refused` \
             carries the cause on `outcome` as `Some(Err(..))`: {other:?}"
        ),
    };

    // 🛑 **THE TEXT, NOT `is_err()` — see the module header.** Under `if false && steps.is_empty()`
    // the zero-iteration loop falls to the trailing refusal, which is ALSO an `Err`. These two
    // assertions are what separate the forbid from the fall-through; an `is_err()` in their place
    // is green on that mutant.
    assert!(
        rendered.contains("EMPTY step list"),
        "the refusal does not name ADR-0138's forbid, so it is not this refusal. A zero-iteration \
         walk falling through to \"walked its steps with no `agent` step producing a value\" is \
         also an error, and would read as a pass here: {rendered}"
    );
    assert!(
        !rendered.contains("with no `agent` step producing a value"),
        "the EMPTY list reached the loop's FALL-THROUGH refusal rather than ADR-0138's guard, so \
         the guard did not fire — the list was walked past, not refused: {rendered}"
    );
}
