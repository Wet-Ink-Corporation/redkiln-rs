//! **Two ways the step walk ENDS, both asserted by nothing until now** — `PROGRESS.md` row 37's
//! finding (21).
//!
//! | test | the semantic | the mutant it kills | what killed that mutant before |
//! | --- | --- | --- | --- |
//! | ~~`an_emit_step_ends_the_walk_and_a_later_tool_step_never_runs`~~ | ~~`emit` is terminal (design §4.5)~~ | ~~`StepVerdict::Emitted => {}`~~ | ~~**nothing in the workspace**~~ |
//! | [`an_empty_transcribed_step_list_is_refused_by_name_and_none_is_not_the_empty_case`] | ADR-0138's empty-list forbid | `if false && steps.is_empty()` | **nothing in the workspace** |
//!
//! *(dated 2026-09-26, `s8d` build, package P4; R2 = ADR-0173 decision 7, proposed; `s8d` §8d.8 defect
//! 1 and §8d.10.)* **The first row is struck and its test REMOVED, not renamed: it asserted the silent
//! drop as intended.** A step after an `emit` is now refused by name — at load by the strict walker,
//! and by `run_activity_steps` for a list transcribed some other way — and the walk's
//! `StepVerdict::Emitted` arm appends the completion row that carries the `emit` binding. It is
//! replaced by `emit_binding::a_step_after_emit_is_refused_by_name_and_never_silently_dropped`, whose
//! walk half keeps this module's counting-toolchain instrument (a CONTROL walk with the tool step
//! BEFORE the `emit` moves the counter to 1; the refused walk leaves it at 0). The paragraph below is
//! kept as the history of why the instrument is a counter. Its three step helpers left with it.
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

use rk_kernel::catalog::Step;
use rk_kernel::predicate::Context;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::Activation;
use rk_ports::harness::Harness;
use rk_ports::toolchain::Toolchain;

use crate::runtime::{
    AID, CAP, CountingHarness, CountingToolchain, ITEM, STAGE, STEP, at, counters, empty_state,
    gate_site, ok_typed, ports_with_toolchain, spec, store,
};

const fn activation<'a>(sp: &'a rk_types::activity::ActivitySpec) -> Activation<'a> {
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
    )
    // *(2026-09-26, s8c §8c.4, record V, proposed.)* `MAPPED_VERBS` is replaced by the verb
    // registry, and a runtime with none bound refuses every verb by its bare name; this walk's
    // `toolchain:build` reaches the port through ADR-0135's four verbs as test-only `port` rows.
    .with_tool_routing(rk_kernel::runtime::step::ToolRouting::new(
        crate::tool_verb_routing::registry("", ""),
        rk_kernel::workflow::memo::ToolchainValues::default(),
    ));
    (rt, harness, tc)
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
