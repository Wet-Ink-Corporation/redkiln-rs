//! **`check` evaluation** (`s8d` §8d.3; R2 = ADR-0173 decision 6, written `proposed`).
//!
//! A `check` step's `expr` is evaluated against a `Context` built from its own `inputs` table,
//! through the one `predicate::evaluate` (ADR-0136 decision 1). `Tri::True` continues; `Tri::False`
//! goes to the step's `on_false` (§8d.4 routes it, R3 = ADR-0175); a `check` that declares NO
//! `on_false`, as every one here does, refuses by name as R3's exhausted case, *"did not hold"*;
//! `Tri::Null` abstains (ADR-0103, AC-079); an `EvalError` or an unresolvable input refuses
//! by name and is **never** read as false (§8d.8 defect 3's distinction). ADR-0142 clause 4 is
//! discharged here: a `check` reading a prior step's output now decides.

use std::sync::Arc;

use rk_kernel::catalog::{Catalog, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::ToolRouting;
use rk_kernel::workflow::memo::ToolchainValues;
use rk_ports::harness::Harness;
use rk_ports::toolchain::Toolchain;

use crate::reference_resolver::{activation, check, refusal, root, step};
use crate::runtime::{
    CAP, CountingHarness, CountingToolchain, STAGE, at, counters, empty_state, gate_site, ok_typed,
    ports, ports_with_toolchain, spec, store,
};

fn walk_over(
    rt: &Runtime,
    steps: &[rk_kernel::catalog::Step],
) -> rk_kernel::runtime::activity::ActivityReport {
    let sp = spec();
    rt.run_activity_steps(
        activation(&sp),
        Some(steps),
        &rk_kernel::predicate::Context::new(),
        gate_site(&empty_state()),
        rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
        at(100),
    )
    .expect("a step's own failure is a VALUE (AC-032)")
}

/// A runtime whose one agent dispatch returns `value`.
fn rt_with(value: serde_json::Value) -> Runtime {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(value)]));
    Runtime::new(
        ports(harness as Arc<dyn Harness>, &db),
        "run-check-eval",
        STAGE,
    )
}

/// **The three outcomes, from the step's OWN inputs table.**
#[test]
fn a_check_binds_its_inputs_table_and_decides_true_false_and_refuses() {
    let report = serde_json::json!({"status": "passing", "failed": 0, "passed": 3});
    let steps = |expr: &str| {
        vec![
            step("verify", StepKind::Agent),
            check("verified", &[("report", "${verify.outputs}")], expr),
        ]
    };
    // TRUE: the walk continues past the check (and completes: the list has no `emit`).
    let r = walk_over(
        &rt_with(report.clone()),
        &steps("report.status == 'passing' and report.passed > 0"),
    );
    assert!(
        r.step_error().is_none() && r.completed(),
        "a check reading its bound input must DECIDE true and continue: {r:?}"
    );
    // FALSE with NO `on_false` (the `check` helper declares none): R3's exhausted case, which
    // refuses by name (§8d.4; ADR-0175). The routed arms are `on_false_exec`'s.
    let r = walk_over(&rt_with(report.clone()), &steps("report.failed > 0"));
    let e = refusal(&r);
    assert!(
        e.contains("`verified`") && e.contains("did not hold"),
        "a check that does not hold and declares no `on_false` refuses by name: {e}"
    );
    assert!(
        r.committed(),
        "the agent step's commit stands (ADR-0144): {r:?}"
    );
    // AN `EvalError`: an ordering against a string is not orderable; refused by name, NOT false.
    let r = walk_over(&rt_with(report.clone()), &steps("report.status > 3"));
    let e = refusal(&r);
    assert!(
        e.contains("the `expr` of check step `verified` cannot be evaluated")
            && !e.contains("did not hold"),
        "an evaluation error refuses by name and is never read as false: {e}"
    );
    // NULL: an absent optional input's field abstains; neither true nor false.
    let steps_null = vec![
        step("verify", StepKind::Agent),
        check(
            "verified",
            &[("report", "${nothing.outputs?}")],
            "report.status == 'passing'",
        ),
        step("nothing", StepKind::Agent),
    ];
    let r = walk_over(&rt_with(report), &steps_null);
    let e = refusal(&r);
    assert!(
        e.contains("abstained") && !e.contains("did not hold"),
        "a `null` verdict abstains (ADR-0103; AC-079), never false: {e}"
    );
}

/// **An unresolvable input refuses by name and is NEVER read as false** (§8d.8 defect 3; M5: an
/// `UnresolvedRef` read as `Tri::Null`; M15: the `Err` routed to `on_false`).
#[test]
fn an_unresolvable_check_input_refuses_and_is_never_read_as_false() {
    // `built` in `build-package`'s shape, but reading a step this activation never declared.
    let steps = vec![
        step("verify", StepKind::Agent),
        check(
            "built",
            &[("report", "${build.outputs}")],
            "report.status == 'passing'",
        ),
    ];
    let r = walk_over(&rt_with(serde_json::json!({"status": "passing"})), &steps);
    let e = refusal(&r);
    assert!(
        e.contains("`built`") && e.contains("${build.outputs}"),
        "the refusal must name the step id AND the reference (ADR-0136's forbids): {e}"
    );
    assert!(
        e.contains("cannot resolve"),
        "the refusal is the resolver's, by name: {e}"
    );
    for never in ["did not hold", "abstained", "evaluated `null`"] {
        assert!(
            !e.contains(never),
            "an unresolvable input must never read as `{never}` (ADR-0136; §8d.8 defect 3): {e}"
        );
    }
    // And a prior step that is declared but has not run refuses the same way.
    let steps = vec![
        check(
            "built",
            &[("report", "${build.outputs}")],
            "report.status == 'passing'",
        ),
        step("build", StepKind::Agent),
    ];
    let r = walk_over(&rt_with(serde_json::json!({})), &steps);
    let e = refusal(&r);
    assert!(
        e.contains("${build.outputs}") && !e.contains("did not hold") && !e.contains("abstained"),
        "a mandatory ref with no value is refused, never false or null: {e}"
    );
}

/// **N25: `build-package`'s `built` check reads `s8c`'s widened `build` report**, so a red build is
/// a REPORT whose status fails the check by name (M23: `built` reads the call's `is_ok()`). The
/// steps are the SHIPPED activity's `build` and `built`, read off `.redkiln/catalog`.
#[test]
fn a_red_build_report_fails_the_built_check_by_name() {
    let catalog = Catalog::from_dir(&root().join(".redkiln").join("catalog"))
        .expect("the shipped catalog loads");
    let activity = catalog
        .activities()
        .iter()
        .find(|a| a.id == "build-package")
        .expect("build-package ships");
    // `build` reads `artifact:Changeset.sha`, whose resolution is not this test's: the first two
    // steps are taken with the `sha` input dropped, and `built`'s `inputs`/`expr` are verbatim.
    let mut build = activity.steps[0].clone();
    assert_eq!(
        (build.id.as_str(), build.run.as_deref()),
        ("build", Some("toolchain:build"))
    );
    build.inputs.clear();
    build.timeout_secs = None;
    let built = activity.steps[1].clone();
    assert_eq!(
        built.expr.as_deref(),
        Some("report.status == 'passing'"),
        "PREMISE: the shipped `built`"
    );
    let steps = vec![build, built];
    let walk = |tc: CountingToolchain| {
        let (q, a) = counters();
        let db = store(&q, &a);
        let tc = Arc::new(tc);
        let rt = Runtime::new(
            ports_with_toolchain(
                Arc::new(CountingHarness::scripted(Vec::new())) as Arc<dyn Harness>,
                &db,
                Arc::clone(&tc) as Arc<dyn Toolchain>,
            ),
            "run-built",
            STAGE,
        )
        .with_tool_routing(ToolRouting::new(
            crate::tool_verb_routing::registry("", ""),
            ToolchainValues::default(),
        ));
        (walk_over(&rt, &steps), tc.calls())
    };
    // CONTROL: a green build passes `built`.
    let (r, calls) = walk(CountingToolchain::default());
    assert_eq!(calls, 1, "the port's `build` was reached once");
    assert!(
        r.step_error().is_none() && r.completed(),
        "CONTROL: a passing build walks: {r:?}"
    );
    // A red build is `Ok(BuildReport { status: failing })` at the port (N25): the STEP walks, its
    // report binds, and the `built` check fails BY NAME — never the build step's own `Err`.
    let (r, calls) = walk(CountingToolchain::with_failing_build());
    assert_eq!(calls, 1);
    let e = refusal(&r);
    assert!(
        e.contains("`built`") && e.contains("did not hold"),
        "a red build must fail the `built` check by name (N25): {e}"
    );
    assert!(
        !e.contains("NonZero"),
        "the red build was refused at the tool step, not by `built`: {e}"
    );
}
