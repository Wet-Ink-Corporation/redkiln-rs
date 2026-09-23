//! **The `tool` arm's verb routing — ADR-0135's map, as ADR-0148 leaves it** (`PROGRESS.md` row
//! 37's finding (6)).
//!
//! `run_tool_step` read `toolchain.test(&[])`, and `rk-adapter-toolchain-rust` reads an EMPTY
//! selection as `cargo test --workspace`. So every one of the **28** shipped `toolchain:test` steps
//! was a whole-workspace run under the 3600s bound on a shared target dir — the invisible
//! non-event ADR-0135 exists to make impossible, one level down from the verb map it closed.
//!
//! 🛑 **AND THE TEMPTING FIX IS WORSE THAN THE DEFECT.** Parsing the `inputs.selection` key
//! `catalog::Step` drops and passing the string through as a test name would run
//! `cargo test affected` — a filter that matches **nothing**, which exits **0**. Measured over
//! `.redkiln/catalog/` on 2026-09-22: all 28 `toolchain:test` steps declare `inputs.selection`, and
//! it is `"affected"` (17), a `${…}` reference (6), `artifact:…` (4) or `"e2e"` (1). **Not one is a
//! test name.** A green from a toolchain that ran no tests is strictly worse than an over-broad run,
//! because the over-broad run at least fails when the tree is broken. So the verb refuses by name.
//!
//! The two readings below are independent, and the second is the one that cannot be faked: a
//! refusal that still reached the port would satisfy the message assertion and fail the counter.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::StepVerdict;
use rk_ports::harness::Harness;
use rk_ports::toolchain::Toolchain;

use crate::runtime::{
    CountingHarness, CountingToolchain, STAGE, counters, ports_with_toolchain, store,
};

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
    }
}

fn rt_with_counter(run_id: &str) -> (Runtime, Arc<CountingToolchain>) {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(Vec::new()));
    let tc = Arc::new(CountingToolchain::default());
    let rt = Runtime::new(
        ports_with_toolchain(
            harness as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        run_id,
        STAGE,
    );
    (rt, tc)
}

/// **ADR-0148: `toolchain:test` refuses by name and never reaches the port.**
#[test]
fn a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port() {
    let ctx = Context::new();

    // ---- THE CONTROL FIRST: a SIBLING verb from the same map DOES route. ------------------------
    //
    // `build` and `test` are both in `MAPPED_VERBS`, so without this the refusal below is
    // indistinguishable from ADR-0135's by-name refusal of an UNMAPPED verb, from an unbound port,
    // and from a `run` key this fixture spelled wrong.
    let (control_rt, control_tc) = rt_with_counter("run-verb-control");
    let control = control_rt
        .execute_step(&tool_step("build-it", "build"), &ctx)
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    assert_eq!(
        control,
        StepVerdict::Continue,
        "CONTROL: `toolchain:build` did not route through the map: {control:?}"
    );
    assert_eq!(
        control_tc.calls(),
        1,
        "CONTROL: `toolchain:build` did not reach the port, so the zero below would mean nothing"
    );

    // ---- THE ASSERTION. ------------------------------------------------------------------------
    let (rt, tc) = rt_with_counter("run-verb-test");
    let verdict = rt
        .execute_step(&tool_step("run-tests", "test"), &ctx)
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

    // (i) THE COUNTER. This is the half that sees the defect: the old code CALLED
    // `toolchain.test(&[])` and returned `Continue`, so a message-only assertion would pass on a
    // fix that still ran the whole workspace first.
    assert_eq!(
        tc.calls(),
        0,
        "`toolchain:test` reached the `Toolchain` port. An empty selection is \
         `cargo test --workspace` to the real Rust adapter, which is what ADR-0148 refuses: \
         {verdict:?}"
    );

    // (ii) AND IT REFUSED FOR ITS OWN REASON, not as an unmapped verb.
    match verdict {
        StepVerdict::Failed(e) => {
            let rendered = format!("{e:?}");
            assert!(
                rendered.contains("inputs.selection"),
                "the refusal does not name the unparsed selection, so it does not tell an \
                 operator why the step cannot run — and it is indistinguishable from ADR-0135's \
                 bare-verb refusal of an unmapped verb: {rendered}"
            );
            assert_ne!(
                rendered,
                format!(
                    "{:?}",
                    rk_types::error::StepError::Unsupported {
                        capability: "test".to_owned()
                    }
                ),
                "`test` was refused as though it were OUTSIDE the four-verb map. It is inside it \
                 (ADR-0135 clause 2) and refuses for a different reason; collapsing the two would \
                 make the map's own boundary unreadable from the journal"
            );
        }
        other => panic!(
            "`toolchain:test` did not refuse ({other:?}). Either a selection resolver landed — in \
             which case ADR-0148 is discharged and this test should be REPLACED rather than \
             deleted — or the arm went back to running the empty selection."
        ),
    }
}
