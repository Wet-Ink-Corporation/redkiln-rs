//! **ADR-0143: a second runnable `agent` step refuses, and the commit is never overwritten.**
//!
//! `ActivitySpec` is one spec per node (ADR-0099:61), so a second `agent` step has no card, schema
//! or inputs of its own. AC-013's grain is per **activation**, so `run_activity` answers a second
//! call with its re-entry report — `dispatches: 0`, `commit: None`, `outcome: None`. The walk was
//! storing that **over the committed one**, and `dispatch_through` then answered
//! `NotCommitted { class: None }` for an activation that had committed: the exact hole Mechanism B
//! exists to close, on a report for which `skipped()` is also `true`, so a failed walk was
//! indistinguishable from a legitimate AC-013 skip.
//!
//! Measured over the shipped pack: **63 of 102** activities declare more than one `agent` step, and
//! **43 of those 63 use DIFFERENT roles** — `[spec-author, reviewer]`,
//! `[architect, reviewer, architect]`. So "the first dispatch already served them" is not available
//! as an answer; it would silently drop a review step in 43 activities.
//!
//! # 🛑 Both halves are asserted, because either alone is satisfiable by the wrong fix
//!
//! A walk that simply stopped at the first agent step would also never overwrite the report. A walk
//! that refused every activity declaring two agent steps would also never produce
//! `NotCommitted { class: None }`. So:
//!
//! 1. the refusal **names the activation and the step**, and says the first dispatch **COMMITTED** —
//!    proving the committed report was read, not discarded;
//! 2. a second agent step whose **guard does not hold** refuses *nothing* and the walk completes —
//!    proving the check is dynamic rather than a pre-walk count of `agent` steps.
//!
//! The second is only a real distinction because ADR-0142 made that guard evaluable at all.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::registry::Registry;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::Activation;
use rk_ports::harness::Harness;

use crate::runtime::{
    AID, CAP, ITEM, STAGE, STEP, at, counters, empty_state, gate_site, ok_typed, ports, spec, store,
};

fn agent_step(id: &str, when: Option<&str>) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Agent,
        run: None,
        output_type: None,
        role: Some("implementer".to_owned()),
        when: when.map(ToOwned::to_owned),
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

fn activation<'a>(spec: &'a rk_types::activity::ActivitySpec) -> Activation<'a> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: Some(spec),
        // (dated 2026-09-26, s8d build, P3) R1: no per-step specs, so the first runnable agent
        // step dispatches under the activation's spec (ADR-0143 decision 2 as R1 narrows it).
        step_specs: None,
        rigor: rk_kernel::rigor::Rigor::Optional,
    }
}

/// **The defect: a second runnable agent step must refuse, naming the commit it did NOT discard.**
#[test]
fn a_second_runnable_agent_step_refuses_and_the_commit_survives() {
    // ADR-0145: the walk now takes a `GateSite`. These tests reach no `gate` step, so the
    // state is an EMPTY fold and the site is never used — but it is real rather than stubbed,
    // so a test that later does reach a gate gets a true reach counter.
    let state = empty_state();
    let (q, a) = counters();
    let db = store(&q, &a);
    // ONE scripted result: the second dispatch must never happen, and a second script entry would
    // hide that by making one available.
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-second-agent",
        STAGE,
    );

    let steps = vec![agent_step("draft", None), agent_step("review", None)];
    let sp = spec();
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

    // ---- (0) THE REPORT STAYS COMMITTED. (dated 2026-09-26, s8d build, P3; section 8d.8 defect 2) --
    //
    // This test checked the refusal STRING for "COMMITTED" and never the report, and the report was
    // `ActivityReport::refused`'s: `committed() == false` and `skipped() == true` for an activation
    // whose row is in the journal, so `dispatch_through` answered `NotCommitted` (ADR-0144's named
    // lie). The repair is ADR-0144's shape, `done.with_step_failure(cause)`: the refusal travels on
    // `step_error()`, and the report answers what the journal holds.
    assert!(
        report.committed() && !report.skipped(),
        "the first dispatch COMMITTED, so the report must answer committed() == true and \
         skipped() == false (section 8d.8 defect 2): committed = {}, skipped = {}",
        report.committed(),
        report.skipped()
    );

    // ---- (1) THE OUTCOME IS A NAMED REFUSAL, NOT A SILENT NON-COMMIT. --------------------------
    // (dated 2026-09-26, s8d build, P3) Read off `step_error()`, the channel ADR-0144 gives a
    // failure after a committed dispatch; `outcome()` is the dispatch's own reading.
    let err = report
        .step_error()
        .or_else(|| report.outcome().and_then(|o| o.as_ref().err()))
        .unwrap_or_else(|| {
            panic!(
                "the walk carried NO error for a second agent step. Before ADR-0143 this returned \
                 AC-013's empty re-entry report, whose `outcome()` is `None` — which is what made \
                 `dispatch_through` answer `NotCommitted {{ class: None }}` for an activation that \
                 had committed. An absent error here IS the defect."
            )
        });
    let rendered = format!("{err:?}");
    for expected in [AID, "review", "one spec per node"] {
        assert!(
            rendered.contains(expected),
            "the refusal does not name {expected:?}, so it does not tell an operator which \
             activation, which step, or why: {rendered}"
        );
    }

    // ---- (2) AND IT READ THE COMMITTED REPORT RATHER THAN DISCARDING IT. -----------------------
    //
    // This is the half that distinguishes the fix from "stop at the first agent step". The refusal
    // states the first dispatch's fate, which it can only do by having the committed report in hand.
    assert!(
        rendered.contains("COMMITTED"),
        "the refusal does not say the first dispatch COMMITTED, so the committed report was not \
         read — which is the overwrite this record exists to end: {rendered}"
    );

    // ---- (3) EXACTLY ONE DISPATCH. ------------------------------------------------------------
    assert_eq!(
        harness.dispatches(),
        1,
        "AC-013's grain is per ACTIVATION: the second agent step must not reach the `Harness` port"
    );
}

/// **The control: a second agent step whose guard does NOT hold refuses nothing.**
///
/// The refusal is keyed on a second *runnable* step, not on the declared count — so an activity
/// that declares two agent steps and guards one off still completes. Only ADR-0142's context makes
/// this distinguishable: before it, the guard itself would have failed on `UnknownField`.
#[test]
fn a_guarded_off_second_agent_step_refuses_nothing() {
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
        "run-guarded-second",
        STAGE,
    );

    // `rigor >= mandatory` evaluated at `optional` is FALSE, so the second step is `Skipped`.
    let steps = vec![
        agent_step("draft", None),
        agent_step("review", Some("rigor >= mandatory")),
    ];
    let ctx = rk_kernel::rigor::with_rigor(Context::new(), Rigor::Optional, &shipped_registry());
    let sp = spec();
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
        report.committed(),
        "the guarded-off second agent step refused a walk that should have completed on its first \
         dispatch. ADR-0143 clause 3 keys the refusal on a RUNNABLE second step, not on the \
         declared count: outcome = {:?}",
        report.outcome()
    );
    assert_eq!(
        harness.dispatches(),
        1,
        "one dispatch, for the one agent step whose guard held"
    );
}
