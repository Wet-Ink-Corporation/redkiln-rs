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

//!
//! *(dated 2026-09-26, s8c §8c.4; records V and B = ADR-0170/0171, proposed.)* `MAPPED_VERBS` is
//! replaced by the verb registry, looked up FIRST, then ADR-0148's `test` refusal, then the route
//! and input checks, then the step's bound, then the port. Every runtime here binds a TEST-ONLY
//! registry ([`PORT_ROWS`] and the rows each case adds); the shipped `.redkiln/registry.toml` is
//! proved by `verb_registry::…`. A runtime with no registry bound refuses every verb by its bare
//! name, which `a_verb_with_no_registry_row_is_refused_by_its_bare_name` also pins.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind, StepTimeout};
use rk_kernel::predicate::Context;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::{StepVerdict, ToolRouting};
use rk_kernel::workflow::memo::{ToolchainValues, VerbRegistry};
use rk_ports::harness::Harness;
use rk_ports::toolchain::{StepBound, Toolchain, VerbOutput};
use rk_types::error::{StepError, TransportKind};

use crate::runtime::{
    CountingHarness, CountingToolchain, STAGE, counters, ports_with_toolchain, store,
};

/// ADR-0135's four verbs as the registry's `port` rows, in the spike's own `outputs` spelling.
pub const PORT_ROWS: &str = r#"[verb]
build          = { outputs = "BuildReport", live = false, timeout = "build_timeout", inputs = [], route = "port" }
test           = { outputs = "TestReport", live = false, timeout = "test_timeout", inputs = ["selection"], route = "port" }
lint           = { outputs = "LintReport", live = false, timeout = "scan_timeout", inputs = [], route = "port" }
change_summary = { outputs = "ChangeSummary", live = false, timeout = "build_timeout", inputs = ["range"], route = "port" }
"#;

/// A test-only registry: [`PORT_ROWS`] plus `extra` rows, and `tables` after them.
pub fn registry(extra: &str, tables: &str) -> VerbRegistry {
    let src = format!("{PORT_ROWS}{extra}\n{tables}");
    VerbRegistry::from_toml(&src).unwrap_or_else(|e| panic!("the test registry loads: {e}\n{src}"))
}

pub fn tool_step(id: &str, verb: &str) -> Step {
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

/// `tool_step` with a declared `timeout_secs`.
pub fn bounded_step(id: &str, verb: &str, timeout: StepTimeout) -> Step {
    Step {
        timeout_secs: Some(timeout),
        ..tool_step(id, verb)
    }
}

/// A runtime over `tc`, with `routing` bound.
pub fn rt_routed(
    run_id: &str,
    tc: CountingToolchain,
    routing: ToolRouting,
) -> (Runtime, Arc<CountingToolchain>) {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(Vec::new()));
    let tc = Arc::new(tc);
    let rt = Runtime::new(
        ports_with_toolchain(
            harness as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        run_id,
        STAGE,
    )
    .with_tool_routing(routing);
    (rt, tc)
}

/// A runtime over a counting fake and ADR-0135's four `port` rows, with an empty value table.
fn rt_with_counter(run_id: &str) -> (Runtime, Arc<CountingToolchain>) {
    rt_routed(
        run_id,
        CountingToolchain::default(),
        ToolRouting::new(registry("", ""), ToolchainValues::default()),
    )
}

/// A passing `SmokeResult` value: one row, passed.
pub fn smoke_value(passed: bool) -> serde_json::Value {
    serde_json::json!({
        "rows": [{
            "id": "version", "argv": ["redkiln", "--version"], "cwd": "scratch",
            "expect_exit": 0, "exit": if passed { 0 } else { 1 },
            "predicate": "redkiln", "predicate_held": passed, "passed": passed
        }],
        "passed": u32::from(passed), "failed": u32::from(!passed),
        "status": if passed { "passing" } else { "failing" }
    })
}

/// A `toolchain` row with no inputs, the positive control's verb.
const SMOKE_ROW: &str = r#"smoke = { outputs = "SmokeResult", live = true, timeout = "deploy_timeout", inputs = [], route = "toolchain" }"#;
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
    // *(Added 2026-09-26, s8c §8c.6, record B = ADR-0171, proposed.)* The step declares no
    // `timeout_secs`, so the port was handed an UNDECLARED bound: the construction bound stands.
    assert_eq!(
        control_tc.bounds(),
        vec![rk_ports::toolchain::StepBound::Undeclared],
        "CONTROL: a step with no timeout_secs hands the port an undeclared bound"
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

// ================================================================================================
// s8c §8c.4 / §8c.9: the registry-routed `tool` arm (records V and B = ADR-0170/0171, proposed).
// ================================================================================================

/// The one refusal every "nothing reached the port" assertion below is kept apart from.
fn bare(verb: &str) -> StepError {
    StepError::Unsupported {
        capability: verb.to_owned(),
    }
}

fn failed(verdict: StepVerdict) -> StepError {
    match verdict {
        StepVerdict::Failed(e) => e,
        other => panic!("the step did not fail: {other:?}"),
    }
}

/// **ADR-0135 clause 2, unchanged: a verb with no registry row refuses by its BARE name, and
/// never reaches the port** (M10: a wildcard arm routing an unknown verb to `run_verb`).
#[test]
fn a_verb_with_no_registry_row_is_refused_by_its_bare_name() {
    let ctx = Context::new();
    let routing = || ToolRouting::new(registry(SMOKE_ROW, ""), ToolchainValues::default());

    // CONTROL: a ROWED `toolchain` verb through the identical registry reaches the port.
    let (rt, tc) = rt_routed(
        "no-row-control",
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: smoke_value(true),
        }),
        routing(),
    );
    let v = rt
        .execute_step(&tool_step("s", "smoke"), &ctx)
        .expect("a value");
    assert_eq!(
        v,
        StepVerdict::Continue,
        "CONTROL: the rowed verb did not route: {v:?}"
    );
    assert_eq!(
        tc.calls(),
        1,
        "CONTROL: the rowed verb did not reach the port"
    );

    // THE ASSERTION: an unrowed verb, over a fake that WOULD answer any `run_verb`.
    let (rt, tc) = rt_routed(
        "no-row",
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: smoke_value(true),
        }),
        routing(),
    );
    let e = failed(
        rt.execute_step(&tool_step("d", "deploy"), &ctx)
            .expect("a value"),
    );
    assert_eq!(
        e,
        bare("deploy"),
        "a verb with no row must refuse by its BARE name (ADR-0135 clause 2)"
    );
    assert_eq!(
        tc.calls(),
        0,
        "a verb with no registry row reached the port: an unknown verb was routed, which is the \
         wildcard record V decision 1 forbids"
    );

    // AND A RUNTIME WITH NO REGISTRY BOUND refuses even a port verb by its bare name: there is no
    // built-in table behind the registry (§8c.4, "One parser, not two").
    let (q, a) = counters();
    let db = store(&q, &a);
    let tc = Arc::new(CountingToolchain::default());
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::new(CountingHarness::scripted(Vec::new())) as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        "no-registry",
        STAGE,
    );
    let e = failed(
        rt.execute_step(&tool_step("b", "build"), &ctx)
            .expect("a value"),
    );
    assert_eq!(e, bare("build"), "no registry bound: every verb has no row");
    assert_eq!(tc.calls(), 0, "no registry bound, and the port was reached");
}

/// **The positive control that the executor reaches `run_verb`** (§8c.4): a test-only `toolchain`
/// row that declares no inputs.
#[test]
fn a_registry_verb_with_no_inputs_reaches_run_verb() {
    let ctx = Context::new();
    let (rt, tc) = rt_routed(
        "reaches-run-verb",
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: smoke_value(true),
        }),
        ToolRouting::new(registry(SMOKE_ROW, ""), ToolchainValues::default()),
    );
    let v = rt
        .execute_step(&tool_step("smoke", "smoke"), &ctx)
        .expect("a value");
    assert_eq!(
        v,
        StepVerdict::Continue,
        "a `toolchain` row with no inputs did not walk: {v:?}"
    );
    assert_eq!(tc.calls(), 1, "`run_verb` was not reached exactly once");
    assert_eq!(
        tc.bounds(),
        vec![StepBound::Undeclared],
        "the step declares no timeout_secs, so the call carries an undeclared bound"
    );

    // The typed output is READ: a failing smoke result is not a walked step.
    let (rt, tc) = rt_routed(
        "reaches-run-verb-red",
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: smoke_value(false),
        }),
        ToolRouting::new(registry(SMOKE_ROW, ""), ToolchainValues::default()),
    );
    let e = failed(
        rt.execute_step(&tool_step("smoke", "smoke"), &ctx)
            .expect("a value"),
    );
    assert!(
        matches!(e, StepError::NonZero { code: None, .. }),
        "a failing SmokeResult must fail the step: {e:?}"
    );
    assert_eq!(tc.calls(), 1);
}

/// **A `toolchain` row whose inputs need the resolver refuses BEFORE the port**, naming the verb
/// and its FIRST unresolved input (§8c.4; the resolver is `s8d`'s).
#[test]
fn a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port() {
    let ctx = Context::new();
    let rows = format!(
        "{SMOKE_ROW}\nsca_scan = {{ outputs = \"ScanReport\", live = false, timeout = \
         \"scan_timeout\", inputs = [\"audit_db\", \"deny_db\"], route = \"toolchain\" }}"
    );
    let routing = || ToolRouting::new(registry(&rows, ""), ToolchainValues::default());
    let out = || {
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: smoke_value(true),
        })
    };

    // CONTROL: the no-input row in the same registry reaches the port.
    let (rt, tc) = rt_routed("inputs-control", out(), routing());
    rt.execute_step(&tool_step("s", "smoke"), &ctx)
        .expect("a value");
    assert_eq!(
        tc.calls(),
        1,
        "CONTROL: the no-input row did not reach the port"
    );

    let (rt, tc) = rt_routed("inputs", out(), routing());
    let e = failed(
        rt.execute_step(&tool_step("scan", "sca_scan"), &ctx)
            .expect("a value"),
    );
    let rendered = format!("{e:?}");
    assert!(
        rendered.contains("toolchain:sca_scan") && rendered.contains("`audit_db`"),
        "the refusal must name the verb and its FIRST unresolved input: {rendered}"
    );
    assert!(
        !rendered.contains("deny_db"),
        "the refusal names the FIRST input, not a later one: {rendered}"
    );
    assert_ne!(
        e,
        bare("sca_scan"),
        "a rowed verb refused as though it had no row"
    );
    assert_eq!(
        tc.calls(),
        0,
        "a row whose inputs need the resolver reached the port"
    );
}

/// **A `store` / `git` row refuses naming the route and the stage that builds it** (§8c.4).
#[test]
fn a_row_whose_route_has_no_executor_refuses_naming_the_route() {
    let ctx = Context::new();
    let rows = "item_create = { outputs = \"ItemRefs\", live = false, timeout = \"build_timeout\", \
                inputs = [], route = \"store\" }\nrelease_tag = { outputs = \"ReleaseTag\", live = \
                false, timeout = \"build_timeout\", inputs = [], route = \"git\" }";
    for (verb, route, stage) in [
        ("item_create", "store", "s8f"),
        ("release_tag", "git", "s8g"),
    ] {
        let (rt, tc) = rt_routed(
            "route",
            CountingToolchain::default(),
            ToolRouting::new(registry(rows, ""), ToolchainValues::default()),
        );
        let e = failed(
            rt.execute_step(&tool_step(verb, verb), &ctx)
                .expect("a value"),
        );
        let rendered = format!("{e:?}");
        assert!(
            rendered.contains(&format!("`{route}` route")) && rendered.contains(stage),
            "`{verb}` must refuse naming the `{route}` route and `{stage}`: {rendered}"
        );
        assert_ne!(
            e,
            bare(verb),
            "a rowed verb refused as though it had no row"
        );
        assert_eq!(
            tc.calls(),
            0,
            "`{verb}`'s route has no executor and reached the port"
        );
    }
}

/// **An output whose kind is not the row's `outputs` is refused, naming BOTH kinds** (record V
/// decision 2; M9 skips the check). The value is a VALID `SmokeResult`, so only the kind label
/// differs and nothing but the kind check can refuse it.
#[test]
fn an_output_of_the_wrong_kind_is_refused_naming_both_kinds() {
    let ctx = Context::new();
    let run = |kind: &str, value: serde_json::Value| {
        let (rt, tc) = rt_routed(
            "kind",
            CountingToolchain::with_verb_output(VerbOutput {
                kind: kind.to_owned(),
                value,
            }),
            ToolRouting::new(registry(SMOKE_ROW, ""), ToolchainValues::default()),
        );
        let v = rt
            .execute_step(&tool_step("smoke", "smoke"), &ctx)
            .expect("a value");
        assert_eq!(
            tc.calls(),
            1,
            "the call reached the port in every case here"
        );
        v
    };

    // CONTROL: the same value under the row's own kind walks.
    assert_eq!(run("SmokeResult", smoke_value(true)), StepVerdict::Continue);

    let e = failed(run("ScanReport", smoke_value(true)));
    match &e {
        StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        } => assert!(
            detail.contains("SmokeResult") && detail.contains("ScanReport"),
            "the refusal must name the row's kind AND the returned kind: {detail}"
        ),
        other => panic!("a wrong-kind output was not refused as a decode failure: {other:?}"),
    }

    // A value that does not decode as the row's kind is refused naming the kind.
    let e = failed(run("SmokeResult", serde_json::json!({ "rows": [] })));
    assert!(
        matches!(&e, StepError::Transport { kind: TransportKind::Decode, detail }
            if detail.contains("SmokeResult") && detail.contains("does not decode")),
        "an undecodable value must be refused naming its kind: {e:?}"
    );
}

/// **The order is fixed: the registry lookup runs BEFORE the step's bound is resolved** (§8c.4;
/// M23). `rolling-deploy`'s `deploy` has no row and an `unmeasured` `${toolchain.deploy_timeout}`,
/// and it refuses by its bare verb name, never by its key.
#[test]
fn a_verb_with_no_row_refuses_by_its_bare_name_before_its_bound_is_resolved() {
    let ctx = Context::new();
    let tables = "[toolchain.t]\ndeploy_timeout = { unmeasured = \"no Phase-1 verb deploys\" }\n";
    let routing = || {
        let reg = registry("", tables);
        let values = reg.values_for("t").expect("the test table loads");
        ToolRouting::new(reg, values)
    };
    let key = StepTimeout::Toolchain("deploy_timeout".to_owned());

    // CONTROL: the SAME bound on a ROWED verb does refuse, by its key, so the bound below is one
    // that would refuse if it were reached.
    let (rt, tc) = rt_routed("order-control", CountingToolchain::default(), routing());
    let e = failed(
        rt.execute_step(&bounded_step("b", "build", key.clone()), &ctx)
            .expect("a value"),
    );
    assert!(
        format!("{e:?}").contains("${toolchain.deploy_timeout}"),
        "CONTROL: the unmeasured key did not refuse on a rowed verb: {e:?}"
    );
    assert_eq!(tc.calls(), 0);

    let (rt, tc) = rt_routed("order", CountingToolchain::default(), routing());
    let e = failed(
        rt.execute_step(&bounded_step("deploy", "deploy", key), &ctx)
            .expect("a value"),
    );
    assert_eq!(
        e,
        bare("deploy"),
        "the unrowed verb must refuse by its bare name; a refusal naming its key means the bound \
         was resolved before the registry lookup"
    );
    assert_eq!(tc.calls(), 0);
}
