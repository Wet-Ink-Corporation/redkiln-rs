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
build          = { outputs = "BuildReport", live = false, timeout = "build_timeout", inputs = [], route = "port", side_effecting = false }
test           = { outputs = "TestReport", live = false, timeout = "test_timeout", inputs = ["selection"], route = "port", side_effecting = false }
lint           = { outputs = "LintReport", live = false, timeout = "scan_timeout", inputs = [], route = "port", side_effecting = false }
change_summary = { outputs = "ChangeSummary", live = false, timeout = "build_timeout", inputs = ["range"], route = "port", side_effecting = false }
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
        inputs: std::collections::BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
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
const SMOKE_ROW: &str = r#"smoke = { outputs = "SmokeResult", live = true, timeout = "deploy_timeout", inputs = [], route = "toolchain", side_effecting = false }"#;
/// **ADR-0148: `toolchain:test` refuses by name and never reaches the port.**
///
/// *(Narrowed 2026-09-26, `s8d` build, package P4; R2 = ADR-0173 decision 9, proposed; `s8d`
/// §8d.11.)* ADR-0148's refusal is REVERSED by the resolver (its decision 4: *"reversed by a
/// resolver, not by a revert"*): a `toolchain:test` step whose selection resolves reaches the port
/// (`test_selection::`). What stays TRUE of this name is ADR-0148's forbids — a selection the
/// resolver refuses by name never reaches the port as an empty one — so the fixture is narrowed to
/// that case: `inputs.selection = "${nope.outputs}"`, a reference to a step the activation does not
/// declare. The name is kept, and Checkpoint 8c's dated amendment names it.
#[test]
fn a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port() {
    let ctx = Context::new();

    // ---- THE CONTROL FIRST: a SIBLING verb from the same map DOES route. ------------------------
    //
    // `build` and `test` are both in `MAPPED_VERBS`, so without this the refusal below is
    // indistinguishable from ADR-0135's by-name refusal of an UNMAPPED verb, from an unbound port,
    // and from a `run` key this fixture spelled wrong.
    // *(dated 2026-09-26, s8c refuting round 6.)* `MAPPED_VERBS` is gone: routing is the verb
    // registry's rows (record V = ADR-0170, proposed), and both verbs are rows of [`PORT_ROWS`]
    // here; a verb with no row refuses by its bare name. The control's reasoning is unchanged.
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
        .execute_step(
            &{
                // (narrowed 2026-09-26, s8d build, P4) a selection the resolver refuses by name.
                let mut s = tool_step("run-tests", "test");
                s.inputs.insert(
                    "selection".to_owned(),
                    rk_kernel::catalog::InputValue::Text("${nope.outputs}".to_owned()),
                );
                s
            },
            &ctx,
        )
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
                rendered.contains("${nope.outputs}"),
                "the refusal does not name the reference the resolver could not resolve: {rendered}"
            );
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
        // (dated 2026-09-26, s8d build, P4) The resolver landed, and this test was NARROWED rather
        // than replaced: its selection is one the resolver refuses by name.
        other => panic!(
            "`toolchain:test` with an unresolvable selection did not refuse ({other:?}): the arm \
             ran an empty or unresolved selection, which ADR-0148's forbids refuse."
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

    // *(Added 2026-09-26, s8c refuting round 6: before it, no double handed the kernel an `empty`
    // SmokeResult, so M51 (`Empty` moved to the `Ok` arm) survived the whole workspace suite, 875
    // passed.)* An EMPTY smoke result — no rows run — is not a walked step either (§8c.4; AC-332's
    // "`Empty` is never passing"): `NonZero`, no code, the digest of the report's JSON.
    let empty = serde_json::json!({ "rows": [], "passed": 0, "failed": 0, "status": "empty" });
    let digest = rk_types::digest::sha256(
        serde_json::to_string(
            &serde_json::from_value::<rk_types::verb_output::SmokeResult>(empty.clone())
                .expect("the empty value is a decodable SmokeResult"),
        )
        .expect("a report serialises")
        .as_bytes(),
    );
    let (rt, tc) = rt_routed(
        "reaches-run-verb-empty",
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: empty,
        }),
        ToolRouting::new(registry(SMOKE_ROW, ""), ToolchainValues::default()),
    );
    let e = failed(
        rt.execute_step(&tool_step("smoke", "smoke"), &ctx)
            .expect("a value"),
    );
    assert_eq!(
        e,
        StepError::NonZero {
            code: None,
            stderr_digest: digest,
        },
        "an empty SmokeResult must fail the step as NonZero, no code, the report's digest"
    );
    assert_eq!(tc.calls(), 1, "`run_verb` was not reached exactly once");
}

/// **Every kind in the kernel's closed decode table refuses a value that does not decode, naming
/// the kind; a kind with no entry refuses naming it as having none** (§8c.4; record V = ADR-0170
/// decision 2, proposed). Each kind is reached through a test-only `toolchain` row with no inputs,
/// as `smoke` and `rebuild` are above: the shipped `sca_scan`, `package` and `setup` rows declare
/// inputs and refuse before the port until `s8d`'s resolver, and `item_create` / `release_tag` take
/// the `store` / `git` routes, which refuse naming `s8f` / `s8g`. The registry loader checks
/// `outputs` only on `port` rows, so a `toolchain` row may declare any of these kinds, or none of
/// them.
///
/// *(Added 2026-09-26, s8c refuting round 6: before it, only the `SmokeResult` and `BuildReport`
/// arms were observed, so M52 (the no-entry arm answers `Ok`) and M53a–e (one typed arm accepts any
/// value undecoded) each survived the whole workspace suite, 875 passed.)*
#[test]
fn every_decode_table_kind_refuses_an_undecodable_value_naming_it() {
    let ctx = Context::new();
    let run = |kind: &str, value: serde_json::Value| {
        let row = format!(
            "probe = {{ outputs = \"{kind}\", live = false, timeout = \"build_timeout\", \
             inputs = [], route = \"toolchain\", side_effecting = false }}"
        );
        let (rt, tc) = rt_routed(
            "decode-table",
            CountingToolchain::with_verb_output(VerbOutput {
                kind: kind.to_owned(),
                value,
            }),
            ToolRouting::new(registry(&row, ""), ToolchainValues::default()),
        );
        let v = rt
            .execute_step(&tool_step("probe", "probe"), &ctx)
            .expect("a value");
        assert_eq!(
            tc.calls(),
            1,
            "`{kind}`: `run_verb` was not reached exactly once"
        );
        v
    };
    let kinds = [
        (
            "ScanReport",
            serde_json::json!({ "critical": 0, "high": 0, "medium": 0, "low": 0, "unrated": 0,
                                "advisories": [], "tools": [] }),
        ),
        (
            "BuildArtifact",
            serde_json::json!({ "profile": "release", "binaries": [], "argv": ["cargo"],
                                "toolchain_version": "t", "sha": "s", "head": "s" }),
        ),
        (
            "SetupResult",
            serde_json::json!({ "toolchain": "rust", "projects": [] }),
        ),
        ("ItemRefs", serde_json::json!({ "ids": [] })),
        (
            "ReleaseTag",
            serde_json::json!({ "tag": "v0", "target": "s", "rollback_target": "none" }),
        ),
    ];
    for (kind, valid) in kinds {
        // CONTROL: a value that decodes as the row's kind walks, so the refusal below is the
        // decode's alone.
        assert_eq!(
            run(kind, valid),
            StepVerdict::Continue,
            "CONTROL: a decodable `{kind}` did not walk"
        );
        let e = failed(run(kind, serde_json::json!({})));
        assert!(
            matches!(&e, StepError::Transport { kind: TransportKind::Decode, detail }
                if detail.contains(&format!("`{kind}`")) && detail.contains("does not decode")),
            "an undecodable `{kind}` must be refused as a decode failure naming the kind: {e:?}"
        );
    }

    // A kind the table has no entry for, declared and returned alike, so the kind check passes and
    // only the table can refuse.
    let e = failed(run("Nope", serde_json::json!({})));
    assert!(
        matches!(&e, StepError::Transport { kind: TransportKind::Decode, detail }
            if detail.contains("`Nope`") && detail.contains("no entry")),
        "a kind with no entry must be refused naming it as having none: {e:?}"
    );
}

/// **A `toolchain` row whose input the resolver refuses is refused BEFORE the port**, by name.
///
/// *(Renamed 2026-09-26, `s8d` build, package P4; R2 = ADR-0173, proposed; `s8d` §8d.11. This was
/// `a_registry_verb_whose_inputs_need_the_resolver_refuses_before_the_port`, whose premise — *"the
/// input resolver is `s8d`'s, so the step cannot supply it"* — the resolver made false: a row's
/// declared inputs are now supplied from the step's RESOLVED `inputs`. What stays true, and what
/// this name now says, is that an input which does not resolve — a reference the resolver refuses,
/// or a row input the step does not declare — refuses by name and the port is not called.
/// Checkpoint 8c's dated amendment names the new test.)*
#[test]
fn a_registry_verb_whose_input_the_resolver_refuses_is_refused_before_the_port() {
    let ctx = Context::new();
    let rows = format!(
        "{SMOKE_ROW}\nsca_scan = {{ outputs = \"ScanReport\", live = false, timeout = \
         \"scan_timeout\", inputs = [\"audit_db\", \"deny_db\"], route = \"toolchain\", side_effecting = false }}"
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

    let scan = |inputs: &[(&str, &str)]| {
        let mut s = tool_step("scan", "sca_scan");
        for (k, v) in inputs {
            s.inputs.insert(
                (*k).to_owned(),
                rk_kernel::catalog::InputValue::Text((*v).to_owned()),
            );
        }
        s
    };

    // (i) A row input the step does NOT declare: refused naming the verb and that input, and the
    // FIRST such input, never a later one.
    let (rt, tc) = rt_routed("inputs", out(), routing());
    let e = failed(rt.execute_step(&scan(&[]), &ctx).expect("a value"));
    let rendered = format!("{e:?}");
    assert!(
        rendered.contains("toolchain:sca_scan") && rendered.contains("`audit_db`"),
        "the refusal must name the verb and its FIRST undeclared input: {rendered}"
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
        "a row whose input the step does not declare reached the port"
    );

    // (ii) A declared input the RESOLVER refuses: by name — the step and the reference.
    let (rt, tc) = rt_routed("inputs-unresolved", out(), routing());
    let e = failed(
        rt.execute_step(
            &scan(&[("audit_db", "${nope.outputs}"), ("deny_db", "d")]),
            &ctx,
        )
        .expect("a value"),
    );
    let rendered = format!("{e:?}");
    assert!(
        rendered.contains("`scan`") && rendered.contains("${nope.outputs}"),
        "the refusal must name the step and the unresolvable reference: {rendered}"
    );
    assert_eq!(
        tc.calls(),
        0,
        "an input the resolver refused reached the port"
    );

    // (iii) CONTROL: both inputs declared and resolvable, and the port IS reached — so the two
    // zeros above are the refusals', not a row that can never be called.
    let (rt, tc) = rt_routed(
        "inputs-resolved",
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "ScanReport".to_owned(),
            value: serde_json::json!({ "critical": 0, "high": 0, "medium": 0, "low": 0,
                                       "unrated": 0, "advisories": [], "tools": [] }),
        }),
        routing(),
    );
    let v = rt
        .execute_step(&scan(&[("audit_db", "a.db"), ("deny_db", "d.toml")]), &ctx)
        .expect("a value");
    assert_eq!(
        v,
        StepVerdict::Continue,
        "CONTROL: resolved inputs reach the port: {v:?}"
    );
    assert_eq!(tc.calls(), 1, "CONTROL: the port was reached once");
}

/// **A `store` / `git` row refuses naming the route and the stage that builds it** (§8c.4).
///
/// *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-106 ruling 3.)* The
/// `store` route has an executor now: the composition-root-bound store-route seam. With none bound,
/// as here, the kernel's default refuses by name, naming the route and the unbound handler, so the
/// `store` row's needle moves from ~~`s8f`~~ to `no store-route handler is bound`. The `git` row is
/// unchanged (`s8g`'s).
#[test]
fn a_row_whose_route_has_no_executor_refuses_naming_the_route() {
    let ctx = Context::new();
    let rows = "item_create = { outputs = \"ItemRefs\", live = false, timeout = \"build_timeout\", \
                inputs = [], route = \"store\", side_effecting = false }\nrelease_tag = { outputs = \"ReleaseTag\", live = \
                false, timeout = \"build_timeout\", inputs = [], route = \"git\", side_effecting = false }";
    for (verb, route, stage) in [
        ("item_create", "store", "no store-route handler is bound"),
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
        let _ = route;
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

/// A `toolchain` row whose output kind is `BuildReport`, the verb path's twin of the `build` port row.
const BUILD_VERB_ROW: &str = r#"rebuild = { outputs = "BuildReport", live = false, timeout = "build_timeout", inputs = [], route = "toolchain", side_effecting = false }"#;

/// The report [`CountingToolchain`] answers for `build`, passing or failing.
fn build_report(failing: bool) -> rk_types::verb_output::BuildReport {
    rk_types::verb_output::BuildReport {
        argv: vec!["counting".to_owned(), "build".to_owned()],
        toolchain_version: "counting".to_owned(),
        wall_secs: 0,
        errors: u32::from(failing),
        status: if failing {
            rk_types::verb_output::BuildStatus::Failing
        } else {
            rk_types::verb_output::BuildStatus::Passing
        },
    }
}

/// **A `failing` `BuildReport` is not a walked step, on either path** (s8c §8c.2's P4 note; record
/// T = ADR-0169 decision 8, proposed). A red build is `Ok` with `status: failing` at the port, so the
/// kernel's `build_verdict` is the ONLY place a red build stops a step. It is read twice: after a
/// rowed `build` port call, and in `decode_verb_output`'s `BuildReport` arm for a `run_verb` output.
/// Each reads a failing report as `NonZero` with no exit code and the digest of the report's JSON.
///
/// *(Added 2026-09-26, s8c refuting round 4: before it, every double handing the kernel a build was
/// `passing`, so M46 (the `Failing` arm read as `Ok`), M47 (the port call's verdict dropped) and M48
/// (the verb arm's verdict dropped) each survived the whole workspace suite, 873 passed.)*
#[test]
fn a_failing_build_report_is_not_a_walked_step() {
    let ctx = Context::new();
    let digest_of = |failing: bool| {
        rk_types::digest::sha256(
            serde_json::to_string(&build_report(failing))
                .expect("a report serialises")
                .as_bytes(),
        )
    };

    // ---- The port path: ADR-0135's `build` row.
    let port = |tc: CountingToolchain| {
        let (rt, tc) = rt_routed(
            "build-port",
            tc,
            ToolRouting::new(registry("", ""), ToolchainValues::default()),
        );
        let v = rt
            .execute_step(&tool_step("b", "build"), &ctx)
            .expect("a value");
        assert_eq!(
            tc.calls(),
            1,
            "the port's `build` was not reached exactly once"
        );
        v
    };
    // CONTROL: a passing report walks, so the red below is about the status alone.
    assert_eq!(
        port(CountingToolchain::default()),
        StepVerdict::Continue,
        "CONTROL: a passing build must walk"
    );
    let e = failed(port(CountingToolchain::with_failing_build()));
    assert_eq!(
        e,
        StepError::NonZero {
            code: None,
            stderr_digest: digest_of(true),
        },
        "a failing build at the port must fail the step as NonZero, no code, the report's digest"
    );

    // ---- The verb path: a `toolchain` row whose `outputs` is `BuildReport`.
    let verb = |failing: bool| {
        let (rt, tc) = rt_routed(
            "build-verb",
            CountingToolchain::with_verb_output(VerbOutput {
                kind: "BuildReport".to_owned(),
                value: serde_json::to_value(build_report(failing)).expect("a report serialises"),
            }),
            ToolRouting::new(registry(BUILD_VERB_ROW, ""), ToolchainValues::default()),
        );
        let v = rt
            .execute_step(&tool_step("rb", "rebuild"), &ctx)
            .expect("a value");
        assert_eq!(tc.calls(), 1, "`run_verb` was not reached exactly once");
        v
    };
    assert_eq!(
        verb(false),
        StepVerdict::Continue,
        "CONTROL: a passing BuildReport verb output must walk"
    );
    let e = failed(verb(true));
    assert_eq!(
        e,
        StepError::NonZero {
            code: None,
            stderr_digest: digest_of(true),
        },
        "a failing BuildReport verb output must fail the step as NonZero, no code, the report's digest"
    );
}

/// **D21-73 item 5 (R8 = ADR-0179 decision 4, proposed; `s8d` build, package F2-D2173).** A
/// `toolchain` row's input the step leaves undeclared is supplied from the routing's ROW INPUTS —
/// the values the composition root binds (a declaration row, or a tool's default location) — and
/// reaches `run_verb`; a step that declares the input keeps its own value; and with no row input
/// bound the refusal by name stands. ADR-0170 assigns `sca_scan`'s `audit_db`/`deny_db` defaults to
/// *"the resolver's to supply (`s8d`)"*, and no step in the shipped catalog declares them.
#[test]
fn a_row_input_the_step_leaves_undeclared_is_supplied_by_the_routing() {
    let ctx = Context::new();
    let row = "sca_scan = { outputs = \"ScanReport\", live = false, timeout = \"scan_timeout\", \
               inputs = [\"audit_db\", \"deny_db\"], route = \"toolchain\", side_effecting = false }";
    let report = || {
        CountingToolchain::with_verb_output(VerbOutput {
            kind: "ScanReport".to_owned(),
            value: serde_json::json!({ "critical": 0, "high": 0, "medium": 0, "low": 0,
                                       "unrated": 0, "advisories": [], "tools": [] }),
        })
    };
    let supplied = || {
        ToolRouting::new(registry(row, ""), ToolchainValues::default())
            .with_row_input(
                "sca_scan",
                "audit_db",
                serde_json::json!("/cargo/advisory-db"),
            )
            .with_row_input(
                "sca_scan",
                "deny_db",
                serde_json::json!("/cargo/advisory-dbs"),
            )
    };

    // (i) Neither input declared by the step: both are supplied from the routing, and the port is
    // reached with exactly them.
    let (rt, tc) = rt_routed("row-inputs", report(), supplied());
    let v = rt
        .execute_step(&tool_step("scan", "sca_scan"), &ctx)
        .expect("a value");
    assert_eq!(
        v,
        StepVerdict::Continue,
        "a row whose inputs the routing supplies must reach the port: {v:?}"
    );
    assert_eq!(tc.calls(), 1, "`run_verb` was not reached exactly once");
    let seen = tc.verb_inputs();
    assert_eq!(
        seen.first()
            .map(|m| (m.get("audit_db").cloned(), m.get("deny_db").cloned())),
        Some((
            Some(serde_json::json!("/cargo/advisory-db")),
            Some(serde_json::json!("/cargo/advisory-dbs"))
        )),
        "the port must be handed the routing's row inputs: {seen:?}"
    );

    // (ii) A step that DECLARES the input keeps its own value; the routing fills only the other.
    let (rt, tc) = rt_routed("row-inputs-declared", report(), supplied());
    let mut scan = tool_step("scan", "sca_scan");
    scan.inputs.insert(
        "audit_db".to_owned(),
        rk_kernel::catalog::InputValue::Text("own.db".to_owned()),
    );
    rt.execute_step(&scan, &ctx).expect("a value");
    let seen = tc.verb_inputs();
    assert_eq!(
        seen.first()
            .map(|m| (m.get("audit_db").cloned(), m.get("deny_db").cloned())),
        Some((
            Some(serde_json::json!("own.db")),
            Some(serde_json::json!("/cargo/advisory-dbs"))
        )),
        "the step's own declared input wins over the routing's: {seen:?}"
    );

    // (iii) CONTROL: with no row input bound, the refusal by name stands and the port is untouched.
    let (rt, tc) = rt_routed(
        "row-inputs-none",
        report(),
        ToolRouting::new(registry(row, ""), ToolchainValues::default()),
    );
    let e = failed(
        rt.execute_step(&tool_step("scan", "sca_scan"), &ctx)
            .expect("a value"),
    );
    assert!(
        format!("{e:?}").contains("declares the input `audit_db`"),
        "CONTROL: an unsupplied row input still refuses by name: {e:?}"
    );
    assert_eq!(
        tc.calls(),
        0,
        "CONTROL: the unsupplied row reached the port"
    );
}
