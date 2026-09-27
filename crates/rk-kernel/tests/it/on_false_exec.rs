//! **`on_false` as iteration** (`s8d` §8d.4; R3, written `proposed`; D21-42 4d, D21-48 X7).
//!
//! D21-42 4d: *"`on_false` is honoured as iteration. A `{to,max}` loop-back runs under a new
//! iteration path, keyword forms follow the schema, and exhaustion or no `on_false` refuses as
//! `StepFailedAfterCommit` (ADR-0144). Committed steps stand."* D21-48 X7: *"Refused by name, naming
//! ADR-0046's route OWED."*
//!
//! Every test here walks a real `Runtime` over the in-memory store `runtime.rs` counts through and
//! reads the JOURNAL a later process would fold, not only the report: a loop-back that reported a
//! new frame while re-using the first frame's commit key would pass a report-only test and serve
//! the stale value on the second pass.
//!
//! 🛑 **Every loop here is BOUNDED by its harness's script** ([`LoopHarness`]): a dispatch past the
//! script's end is refused with a message naming the bound (*"LOOP-HARNESS BOUND"*), so a mutant
//! that ignores `max` (M7) ends the walk on THAT refusal rather than hanging the suite, and the
//! failure message says the bound fired.
//!
//! The harness and the scripted toolchain are shared with `reference_resolver`'s iteration tests.

use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::{ActivityReport, commit_key_of, is_completion_row};
use rk_kernel::runtime::step::{ReviewRoutes, ToolRouting};
use rk_kernel::workflow::budget::{Charge, fold};
use rk_kernel::workflow::memo::ToolchainValues;
use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_ports::toolchain::{Selection, TestOutcome, TestRunner, Toolchain};
use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::retention::Retained;
use rk_types::typed::Typed;
use rk_types::usage::{CliVersion, Timings};

use crate::reference_resolver::{activation, check, step, with_inputs};
use crate::runtime::{
    CAP, CountingToolchain, ITEM, STAGE, TREE_HASH, at, counters, empty_state, gate_site, journal,
    ports_with_toolchain, spec, store,
};

type Json = serde_json::Value;

// ---- the shared doubles --------------------------------------------------------------------------

/// A harness that RECORDS the inputs of every spec it was handed and answers from a script under
/// that spec's own schema. A dispatch past the script's end is refused, naming the bound.
pub struct LoopHarness {
    script: Mutex<Vec<Json>>,
    seen: Mutex<Vec<Vec<(String, Json)>>>,
    /// The 1-based dispatch numbers answered with a TRANSIENT `Transport` failure (a class the
    /// kernel's retry loop re-dispatches) instead of the script's next value, which is kept for the
    /// following dispatch.
    transient: Mutex<Vec<usize>>,
}

impl LoopHarness {
    pub const fn scripted(script: Vec<Json>) -> Self {
        Self {
            script: Mutex::new(script),
            seen: Mutex::new(Vec::new()),
            transient: Mutex::new(Vec::new()),
        }
    }

    /// Answer dispatch `n` (1-based) with a transient `Transport` failure, so the kernel retries.
    pub fn fail_transiently_at(&self, n: usize) {
        self.transient.lock().expect("sound").push(n);
    }

    /// How many times the port was reached.
    pub fn dispatches(&self) -> usize {
        self.seen.lock().expect("sound").len()
    }

    /// The value of `name` in the spec of dispatch `n` (0-based).
    pub fn input(&self, n: usize, name: &str) -> Json {
        let inputs = {
            let seen = self.seen.lock().expect("sound");
            let made = seen.len();
            seen.get(n)
                .cloned()
                .unwrap_or_else(|| panic!("there is no dispatch {n}: {made} were made"))
        };
        inputs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| panic!("dispatch {n} carries no input `{name}`: {inputs:?}"))
    }
}

impl Harness for LoopHarness {
    fn dispatch(&self, spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        let n = {
            let mut seen = self.seen.lock().expect("sound");
            seen.push(spec.inputs.clone());
            seen.len()
        };
        let transient = self.transient.lock().expect("sound").contains(&n);
        let next = if transient {
            None
        } else {
            let mut script = self.script.lock().expect("sound");
            (!script.is_empty()).then(|| script.remove(0))
        };
        DispatchOutcome {
            record: DispatchRecord {
                argv_digest: rk_kernel::select::digest_of(b"argv"),
                env_digest: rk_kernel::select::digest_of(b"env"),
                instrument_version: "loop-harness".to_owned(),
                stdout: Retained::of(b""),
                stderr: Retained::of(b""),
                usage: None,
                timings: Timings {
                    wall_ms: 0,
                    api_ms: None,
                },
                total_cost_usd: None,
                headroom_before: None,
                headroom_after: None,
                cli_version: CliVersion::Pinned("loop-harness".to_owned()),
                api_retry_events: 0,
                prompt_cache_ttl: "1h".to_owned(),
                subagent_prompt_cache_ttl: "1h".to_owned(),
                cancelled_by: None,
                detected_by: None,
            },
            result: next.map_or_else(
                || {
                    if transient {
                        return Err(StepError::Transport {
                            kind: rk_types::error::TransportKind::Io,
                            detail: format!(
                                "LOOP-HARNESS TRANSIENT: dispatch {n} scripted to fail"
                            ),
                        });
                    }
                    Err(StepError::Unsupported {
                        capability: format!(
                            "LOOP-HARNESS BOUND: dispatch {n} is past the script's end, so the \
                             loop did not stop where it should have"
                        ),
                    })
                },
                |v| Ok(Typed::new(spec.schema.clone(), v)),
            ),
        }
    }
}

/// A toolchain whose `test` answers a SCRIPT of `(passed, failed)` counts, one per call, and records
/// the selection it was handed. Every other method is [`CountingToolchain`]'s. A call past the
/// script's end is refused, naming the bound.
pub struct ScriptedTests {
    script: Mutex<Vec<(u32, u32)>>,
    selections: Mutex<Vec<Selection>>,
    rest: CountingToolchain,
}

impl ScriptedTests {
    pub fn scripted(script: Vec<(u32, u32)>) -> Self {
        Self {
            script: Mutex::new(script),
            selections: Mutex::new(Vec::new()),
            rest: CountingToolchain::default(),
        }
    }

    /// Every selection `test` was handed, in call order.
    pub fn selections(&self) -> Vec<Selection> {
        self.selections.lock().expect("sound").clone()
    }
}

impl Toolchain for ScriptedTests {
    fn detect(&self) -> Result<bool, StepError> {
        self.rest.detect()
    }

    fn build(
        &self,
        bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_types::verb_output::BuildReport, StepError> {
        self.rest.build(bound)
    }

    fn test(
        &self,
        selection: &Selection,
        _bound: rk_ports::toolchain::StepBound,
    ) -> Result<TestOutcome, StepError> {
        self.selections
            .lock()
            .expect("sound")
            .push(selection.clone());
        let next = {
            let mut script = self.script.lock().expect("sound");
            (!script.is_empty()).then(|| script.remove(0))
        };
        let Some((passed, failed)) = next else {
            return Err(StepError::Unsupported {
                capability: "SCRIPTED-TESTS BOUND: a `test` call past the script's end".to_owned(),
            });
        };
        TestOutcome::new(
            rk_types::verify::VerifyReport {
                basis: rk_types::verify::Basis::WholeSuite,
                argv: vec!["scripted".to_owned(), "test".to_owned()],
                selected_test_count: passed + failed,
                total_test_count: passed + failed,
                total_test_count_source: rk_types::verify::TotalTestCountSource::OwnExecutedCount,
                wall_secs: 0,
                toolchain_version: "scripted".to_owned(),
                suite_sha: "scripted".to_owned(),
                basis_reason: Some(rk_types::verify::BasisReason::ProbeUnverified),
            },
            passed,
            failed,
            TestRunner::named("scripted", "0"),
        )
    }

    fn lint(&self, bound: rk_ports::toolchain::StepBound) -> Result<(), StepError> {
        self.rest.lint(bound)
    }

    fn classify(&self, path: &str) -> Result<rk_ports::toolchain::Classification, StepError> {
        self.rest.classify(path)
    }

    fn change_summary(
        &self,
        range: &str,
        bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_ports::toolchain::ChangeSummary, StepError> {
        self.rest.change_summary(range, bound)
    }

    fn run_verb(
        &self,
        call: &rk_ports::toolchain::VerbCall<'_>,
    ) -> Result<rk_ports::toolchain::VerbOutput, StepError> {
        self.rest.run_verb(call)
    }
}

/// One walk's world: the store, the harness, the toolchain and the runtime over them.
pub struct Rig {
    pub db: Arc<Mutex<dyn EventStore>>,
    pub harness: Arc<LoopHarness>,
    pub tests: Arc<ScriptedTests>,
    pub rt: Runtime,
}

impl Rig {
    /// A runtime over a fresh store, answering agent steps from `script` and `test` from `tests`,
    /// with the test registry's four `port` rows bound.
    pub fn new(script: Vec<Json>, tests: Vec<(u32, u32)>) -> Self {
        Self::routed(script, tests, "", None)
    }

    /// [`Rig::new`] with `extra` registry rows after the four `port` rows, and `routes` bound as the
    /// review routes when given *(dated 2026-09-27, `s8d` build, package R4FIX-8d)*.
    pub fn routed(
        script: Vec<Json>,
        tests: Vec<(u32, u32)>,
        extra: &str,
        routes: Option<Arc<dyn ReviewRoutes>>,
    ) -> Self {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(LoopHarness::scripted(script));
        let tests = Arc::new(ScriptedTests::scripted(tests));
        let routing = ToolRouting::new(
            crate::tool_verb_routing::registry(extra, ""),
            ToolchainValues::default(),
        );
        let routing = match routes {
            Some(routes) => routing.with_review_routes(routes),
            None => routing,
        };
        let rt = Runtime::new(
            ports_with_toolchain(
                Arc::clone(&harness) as Arc<dyn Harness>,
                &db,
                Arc::clone(&tests) as Arc<dyn Toolchain>,
            ),
            "run-loop",
            STAGE,
        )
        .with_tool_routing(routing);
        Self {
            db,
            harness,
            tests,
            rt,
        }
    }

    /// Append `event` to the item's journal (a seeded `emit` binding, or a gate's answer).
    pub fn seed(&self, event: Event) {
        self.db
            .lock()
            .expect("not poisoned")
            .append(event)
            .expect("seeded");
    }

    /// Walk `steps` once, over the journal as it stands (its gates folded).
    pub fn walk(&self, steps: &[Step]) -> ActivityReport {
        self.walk_in(steps, &rk_kernel::predicate::Context::new())
    }

    /// [`Rig::walk`] under the selection context `ctx` (a stage `rigor`, say).
    pub fn walk_in(&self, steps: &[Step], ctx: &rk_kernel::predicate::Context) -> ActivityReport {
        let state = rk_kernel::workflow::fsm::fold(ITEM, STAGE, &journal(&self.db))
            .unwrap_or_else(|_| empty_state());
        let sp = spec();
        self.rt
            .run_activity_steps(
                activation(&sp),
                Some(steps),
                ctx,
                gate_site(&state),
                rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
                at(100),
            )
            .expect("a step's own failure is a VALUE (AC-032)")
    }

    /// Answer the gate `report` opened with `option`, at the gate site's tree hash.
    pub fn answer(&self, report: &ActivityReport, option: &str) {
        let token = report
            .gate_opened()
            .unwrap_or_else(|| panic!("PREMISE: the walk opened a gate: {report:?}"))
            .clone();
        self.seed(rk_kernel::workflow::gate::gate_answered_event(
            &token, TREE_HASH, option,
        ));
    }

    /// Every committed step activation of `step_id` in the journal, as its iteration path's
    /// rendering (`root`, `loop_back:1`, `loop_back:1/repeat:2`), in journal order.
    pub fn paths(&self, step_id: &str) -> Vec<String> {
        journal(&self.db)
            .iter()
            .filter(|e| !is_completion_row(e))
            .filter_map(commit_key_of)
            .filter(|k| k.step_id == step_id)
            .map(|k| k.iteration_path.to_string())
            .collect()
    }

    /// Whether the journal holds this activation's completion row.
    pub fn completed(&self) -> bool {
        journal(&self.db).iter().any(is_completion_row)
    }
}

/// An `agent` step reading `inputs`.
pub fn agent(id: &str, inputs: &[(&str, &str)]) -> Step {
    with_inputs(step(id, StepKind::Agent), inputs)
}

/// A `tool` step naming `toolchain:<verb>`, reading `inputs`.
pub fn tool(id: &str, verb: &str, inputs: &[(&str, &str)]) -> Step {
    let mut s = with_inputs(step(id, StepKind::Tool), inputs);
    s.run = Some(format!("toolchain:{verb}"));
    s
}

/// A `gate` step asking `question`, guarded by `when`.
pub fn gate(id: &str, question: &str, when: Option<&str>) -> Step {
    let mut s = step(id, StepKind::Gate);
    s.question = Some(question.to_owned());
    s.when = when.map(ToOwned::to_owned);
    s
}

/// `check` with a declared `on_false`, set as the loader sets it: the whole value, and its `to`.
pub fn check_on_false(id: &str, inputs: &[(&str, &str)], expr: &str, on_false: Json) -> Step {
    let mut s = check(id, inputs, expr);
    s.on_false_to = on_false
        .get("to")
        .and_then(Json::as_str)
        .map(ToOwned::to_owned);
    s.on_false = Some(on_false);
    s
}

/// `s` with `repeat = { until, max }` and its exits, set as the loader sets them.
pub fn repeating(
    mut s: Step,
    until: &str,
    max: u32,
    on_exhausted: Option<&str>,
    escalate: Option<&str>,
) -> Step {
    s.repeat_until = Some(until.to_owned());
    s.repeat_max = Some(max);
    s.on_exhausted = on_exhausted.map(ToOwned::to_owned);
    s.escalate_question = escalate.map(ToOwned::to_owned);
    s
}

/// The walk's refusal, rendered: its `step_error` and its outcome.
pub fn refusal(report: &ActivityReport) -> String {
    format!("{:?} {:?}", report.step_error(), report.outcome())
}

/// The two-step loop every test below walks: `draft` (an agent step) and `verify` (a `check` over
/// its output, `ok == true`), `on_false` as given, then an `emit`.
fn draft_verify(on_false: Option<Json>) -> Vec<Step> {
    let verify = on_false.map_or_else(
        || check("verify", &[("d", "${draft.outputs}")], "d.ok == true"),
        |v| check_on_false("verify", &[("d", "${draft.outputs}")], "d.ok == true", v),
    );
    vec![agent("draft", &[]), verify, step("emit", StepKind::Emit)]
}

// =================================================================================================

/// **A `{ to, max }` loop-back re-enters its target under a NEW loop-back frame**, the committed
/// step activations of the earlier frame STAND, and each frame is a pass charged on its own (m14):
/// a re-entry that serves the committed frames charges nothing.
#[test]
fn a_loop_back_re_enters_its_target_under_a_new_iteration_path() {
    let mut steps = draft_verify(Some(serde_json::json!({"to": "draft", "max": 2})));
    // An approval gate after the loop, so a SECOND walk re-enters an incomplete activation and
    // serves both committed frames.
    steps.insert(2, gate("approve", "draft.approve", None));
    let rig = Rig::new(
        vec![
            serde_json::json!({"ok": false, "n": 1}),
            serde_json::json!({"ok": true, "n": 2}),
        ],
        Vec::new(),
    );
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "the loop-back must re-enter `draft` and the second pass hold, reaching the gate: {first:?} \
         {}",
        refusal(&first)
    );
    assert_eq!(
        rig.harness.dispatches(),
        2,
        "one dispatch per frame: the first pass and one loop-back"
    );
    assert_eq!(
        rig.paths("draft"),
        vec!["root".to_owned(), "loop_back:1".to_owned()],
        "each frame commits under its OWN iteration path, and the first frame's row STANDS"
    );
    // The per-frame charge (m14): one pass per frame, each charged for the step it executed.
    let charges = first.pass_charges();
    let passes: Vec<String> = charges.iter().map(|(p, _)| p.to_string()).collect();
    assert_eq!(
        passes,
        vec!["root".to_owned(), "loop_back:1".to_owned()],
        "each loop-back frame is a pass of its own (AC-021's per-pass count): {charges:?}"
    );
    let only: Vec<Charge> = charges.iter().map(|(_, c)| *c).collect();
    for c in &only {
        assert_eq!(
            *c,
            Charge::Attempts(rk_kernel::workflow::budget::Attempts::new(1)),
            "the pass's one dispatched step charges it once: {charges:?}"
        );
    }
    let envelope = rk_kernel::workflow::budget::BudgetEnvelope {
        attempts_total: rk_kernel::workflow::budget::Attempts::new(12),
        tokens: rk_kernel::workflow::budget::Tokens::new(1_000),
        wall_clock_secs: rk_kernel::workflow::budget::WallClockSecs::new(60),
    };
    let (ledger, measured) = fold(&only, &envelope);
    assert_eq!(
        (ledger.attempts().get(), measured.attempts_charges),
        (2, 2),
        "the draw-down accumulates across passes (ADR-0122 decision 3)"
    );
    // RE-ENTRY: the gate answered, a second walk SERVES both frames' commits, re-runs the check
    // (it does not commit) through the same loop-back, dispatches nothing and charges nothing.
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none(),
        "the re-entry must walk past the answered gate to the `emit`: {}",
        refusal(&second)
    );
    assert_eq!(
        (rig.harness.dispatches(), second.served()),
        (2, 2),
        "the re-entry serves both committed step activations and re-dispatches neither"
    );
    assert!(
        second
            .pass_charges()
            .iter()
            .all(|(_, c)| *c == Charge::Attempts(rk_kernel::workflow::budget::Attempts::new(0))),
        "a step served from its commit charges nothing: {:?}",
        second.pass_charges()
    );
    assert!(rig.completed(), "the completion row is journalled");
}

/// **m14, settled by accepted ADR-0122** *(dated 2026-09-27, `s8d` build, package M14-8d)*: a pass
/// is charged by the DISPATCHES its steps made, not a flat one per executed step. ADR-0122 decision
/// 1: *"The runtime attempt counter for the activity is debited by the harness's own attempts
/// observed on the stream of a dispatch, as well as by the dispatch itself."* The agent step `draft`
/// fails transiently on its first dispatch (`Transport`, a class the kernel's retry loop
/// re-dispatches) and commits on its second, so its pass is charged 2 — the flat-1 charge reads 1.
#[test]
fn a_kernel_retried_agent_step_charges_its_pass_once_per_dispatch() {
    let steps = draft_verify(None);
    let rig = Rig::new(vec![serde_json::json!({"ok": true, "n": 1})], Vec::new());
    rig.harness.fail_transiently_at(1);
    let report = rig.walk(&steps);
    assert!(
        report.completed() && report.step_error().is_none(),
        "PREMISE: the kernel retries the transient failure and the walk completes: {}",
        refusal(&report)
    );
    assert_eq!(
        (rig.harness.dispatches(), report.dispatches()),
        (2, 2),
        "PREMISE: the step was dispatched twice — one transient failure, one kernel retry"
    );
    assert_eq!(
        rig.paths("draft"),
        vec!["root".to_owned()],
        "PREMISE: one step activation committed, in the root pass"
    );
    let charges = report.pass_charges();
    let rendered: Vec<(String, Charge)> =
        charges.iter().map(|(p, c)| (p.to_string(), *c)).collect();
    assert_eq!(
        rendered,
        vec![(
            "root".to_owned(),
            Charge::Attempts(rk_kernel::workflow::budget::Attempts::new(2))
        )],
        "the pass is charged once per DISPATCH its step made (ADR-0122 decision 1), not once per \
         executed step: {charges:?}"
    );
}

/// The walk's pass charges, rendered as `(pass, attempts)`.
fn charged(report: &ActivityReport) -> Vec<(String, u32)> {
    report
        .pass_charges()
        .iter()
        .map(|(p, c)| {
            let Charge::Attempts(a) = c else {
                panic!("a walk charges attempts only: {c:?}")
            };
            (p.to_string(), a.get())
        })
        .collect()
}

/// **m14: a `tool` dispatch that REACHES the port and fails charges its pass** *(dated 2026-09-27,
/// `s8d` build, package R4FIX-8d; refuting round 4's spec-conformance major)*. §8d.3: *"Every agent or
/// tool dispatch in a frame charges that frame's pass."* A failed agent dispatch was charged and a
/// failed tool dispatch was not. Three arms over `draft` (agent) → `t` (`toolchain:test`) → `emit`:
/// the port answers (control, 2), the port is reached and errs (2, not 1), and the selection refuses
/// BEFORE the port (1: a pre-port refusal made no dispatch and stays uncharged).
#[test]
fn a_tool_dispatch_that_fails_at_the_port_charges_its_pass() {
    let steps = |selection: &str| {
        vec![
            agent("draft", &[]),
            tool("t", "test", &[("selection", selection)]),
            step("emit", StepKind::Emit),
        ]
    };
    let draft = || vec![serde_json::json!({"tests": ["t1"]})];
    let root = |n: u32| vec![("root".to_owned(), n)];

    // CONTROL: the port answers, and the walk completes.
    let rig = Rig::new(draft(), vec![(1, 0)]);
    let report = rig.walk(&steps("${draft.outputs.tests}"));
    assert!(
        report.completed() && report.step_error().is_none(),
        "PREMISE: the control walk completes: {}",
        refusal(&report)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.tests.selections().len()),
        (1, 1),
        "PREMISE: one agent dispatch and one port call"
    );
    assert_eq!(
        charged(&report),
        root(2),
        "the agent dispatch and the tool dispatch each charge the pass"
    );

    // THE PORT IS REACHED AND ERRS: the tool step still made its dispatch.
    let rig = Rig::new(draft(), Vec::new());
    let report = rig.walk(&steps("${draft.outputs.tests}"));
    let e = refusal(&report);
    assert!(
        e.contains("SCRIPTED-TESTS BOUND") && !report.completed(),
        "PREMISE: the port's own error ends the walk: {e}"
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.tests.selections().len()),
        (1, 1),
        "PREMISE: the port WAS reached, exactly once"
    );
    assert_eq!(
        charged(&report),
        root(2),
        "a tool dispatch that reached the port and failed charges its pass, as a failed agent \
         dispatch does (§8d.3 m14; ADR-0175 decision 7): {e}"
    );

    // A PRE-PORT REFUSAL: the selection does not resolve, the port is never reached.
    let rig = Rig::new(draft(), vec![(1, 0)]);
    let report = rig.walk(&steps("${draft.outputs.nope}"));
    let e = refusal(&report);
    assert!(
        e.contains("`t`") && !report.completed() && rig.tests.selections().is_empty(),
        "PREMISE: the selection refuses by name before the port: {e}"
    );
    assert_eq!(
        charged(&report),
        root(1),
        "a refusal before the port made no dispatch and charges nothing; only `draft` charges: {e}"
    );
}

/// **m14: a walk that halts before ANY commit keeps the charges its dispatched steps made**
/// *(dated 2026-09-27, `s8d` build, package R4FIX-8d)*. `Walk::halt` built a FRESH report for a
/// refusal or a gate before a commit, discarding every charge already noted. Two arms, each with no
/// commit (idempotent tool steps commit nothing): `t1` (`toolchain:lint`) dispatches, then `t2`
/// refuses before its port (a `test` with no selection); and `t1` dispatches, then a gate opens.
#[test]
fn a_walk_that_halts_before_any_commit_keeps_the_charges_already_made() {
    // A refusal before any commit.
    let rig = Rig::new(Vec::new(), Vec::new());
    let report = rig.walk(&[
        tool("t1", "lint", &[]),
        tool("t2", "test", &[]),
        step("emit", StepKind::Emit),
    ]);
    let e = refusal(&report);
    assert!(
        e.contains("`t2`")
            && !report.committed()
            && !report.completed()
            && rig.tests.selections().is_empty(),
        "PREMISE: `t2` refuses before its port and nothing committed: {e}"
    );
    assert_eq!(
        charged(&report),
        vec![("root".to_owned(), 1)],
        "`t1`'s dispatch stays charged when the walk refuses before any commit: {e}"
    );

    // A gate before any commit.
    let rig = Rig::new(Vec::new(), Vec::new());
    let report = rig.walk(&[
        tool("t1", "lint", &[]),
        gate("approve", "t1.approve", None),
        step("emit", StepKind::Emit),
    ]);
    assert!(
        report.gate_opened().is_some() && !report.committed() && report.dispatches() == 0,
        "PREMISE: the gate opens with no commit and no agent dispatch: {}",
        refusal(&report)
    );
    assert_eq!(
        charged(&report),
        vec![("root".to_owned(), 1)],
        "`t1`'s dispatch stays charged when the walk gates before any commit"
    );
}

/// **m14: a `for_each` that halts part-way charges its pass once for the copies it dispatched**
/// *(dated 2026-09-27, `s8d` build, package R4FIX-8d)*. `fan_out` returned a copy's halt before its
/// `note_charge`. `red` fans out over two elements; the first copy commits and the second's dispatch
/// is refused at the harness's bound. A second arm *(dated 2026-09-27, `s8d` build, package
/// R5FIX-8d)* refuses copy 0 itself, so the only dispatch is the halting copy's own (MR4q).
#[test]
fn a_fan_out_that_halts_part_way_charges_its_pass_for_the_copies_it_dispatched() {
    let mut red = agent("red", &[("ac", "${item}")]);
    red.for_each = Some("artifact:Spec.acs".to_owned());
    let rig = Rig::new(vec![serde_json::json!({"tests_added": ["a"]})], Vec::new());
    rig.seed(crate::reference_resolver::emitted_row(
        "spec-author@1",
        "Spec",
        serde_json::json!({"acs": ["AC-1", "AC-2"]}),
    ));
    let report = rig.walk(&[red, step("emit", StepKind::Emit)]);
    let e = refusal(&report);
    assert!(
        e.contains("LOOP-HARNESS BOUND") && report.committed() && !report.completed(),
        "PREMISE: the first copy commits and the second's dispatch is refused: {e}"
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.paths("red")),
        (2, vec!["for_each:0".to_owned()]),
        "PREMISE: both copies dispatched, one committed"
    );
    assert_eq!(
        charged(&report),
        vec![("root".to_owned(), 1)],
        "a fan-out that dispatched charges its pass ONCE (AC-022), halted or not: {e}"
    );

    // THE FIRST COPY FAILS *(dated 2026-09-27, `s8d` build, package R5FIX-8d; refuting round 5's
    // major)*: an empty harness script refuses copy 0 at the bound AFTER its dispatch, so nothing
    // commits. The arm above cannot see a `fan_out` that notes a copy's dispatch only when the copy
    // proceeds (MR4q): its copy 0 commits first. This one's only dispatch is the halting copy's own.
    let mut red = agent("red", &[("ac", "${item}")]);
    red.for_each = Some("artifact:Spec.acs".to_owned());
    let rig = Rig::new(Vec::new(), Vec::new());
    rig.seed(crate::reference_resolver::emitted_row(
        "spec-author@1",
        "Spec",
        serde_json::json!({"acs": ["AC-1", "AC-2"]}),
    ));
    let report = rig.walk(&[red, step("emit", StepKind::Emit)]);
    let e = refusal(&report);
    assert!(
        e.contains("LOOP-HARNESS BOUND") && !report.committed() && !report.completed(),
        "PREMISE: copy 0's dispatch is refused at the bound and nothing commits: {e}"
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.paths("red")),
        (1, Vec::<String>::new()),
        "PREMISE: copy 0 dispatched once, and no copy committed"
    );
    assert_eq!(
        charged(&report),
        vec![("root".to_owned(), 1)],
        "a fan-out whose FIRST copy dispatched and failed charges its pass once (AC-022; m14): {e}"
    );
}

/// **m14: a pass is charged only by the dispatches made IN it, across a fan-out and a `repeat`**
/// *(dated 2026-09-27, `s8d` build, package R6FIX-8d; refuting round 6's tests-mutants major)*.
/// §8d.3: *"each `repeat` frame is a pass for AC-021's per-pass count. Every agent or tool dispatch
/// in a frame charges that frame's pass. ... A step served from its commit on re-entry charges
/// nothing."*; R4FIX-8d: *"a refusal before the call charges nothing."* Three arms, one per mutant
/// round 6 left alive:
/// - (a) a `for_each` whose copy 0 refuses BEFORE its dispatch charges nothing (mutant A: the
///   halting arm's `if executed` forced true);
/// - (b) a `for_each` fully served from its commits on re-entry charges nothing (mutant B: the
///   completion arm's `if executed` forced true);
/// - (c) a `repeat` that holds on pass 2 charges `repeat:1` and `repeat:2` one attempt each
///   (mutant C: every pass keyed on the loop base, `root` 2).
#[test]
fn a_pass_is_charged_only_by_the_dispatches_made_in_it_across_a_fan_out_and_a_repeat() {
    let none: Vec<(String, u32)> = Vec::new();
    let acs = || {
        crate::reference_resolver::emitted_row(
            "spec-author@1",
            "Spec",
            serde_json::json!({"acs": ["AC-1", "AC-2"]}),
        )
    };

    // (a) Copy 0 of a `test` fan-out has no selection, so it refuses before the port.
    let mut t = tool("t", "test", &[]);
    t.for_each = Some("artifact:Spec.acs".to_owned());
    let rig = Rig::new(Vec::new(), vec![(1, 0), (1, 0)]);
    rig.seed(acs());
    let report = rig.walk(&[t, step("emit", StepKind::Emit)]);
    let e = refusal(&report);
    assert!(
        e.contains("`t`") && !report.completed() && rig.tests.selections().is_empty(),
        "PREMISE (a): copy 0 refuses by name before its port, and no copy reaches it: {e}"
    );
    assert_eq!(
        charged(&report),
        none,
        "(a) a fan-out whose copy refused BEFORE its dispatch dispatched nothing and charges \
         nothing: {e}"
    );

    // (b) Both copies commit, a gate opens; answered, the re-entry SERVES both copies.
    let mut red = agent("red", &[("ac", "${item}")]);
    red.for_each = Some("artifact:Spec.acs".to_owned());
    let steps = vec![
        red,
        gate("approve", "red.approve", None),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"tests_added": ["a"]}),
            serde_json::json!({"tests_added": ["b"]}),
        ],
        Vec::new(),
    );
    rig.seed(acs());
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "PREMISE (b): both copies commit and the gate opens: {}",
        refusal(&first)
    );
    assert_eq!(
        (charged(&first), rig.paths("red")),
        (
            vec![("root".to_owned(), 1)],
            vec!["for_each:0".to_owned(), "for_each:1".to_owned()]
        ),
        "PREMISE (b): the first walk dispatched both copies and charged its pass once (AC-022)"
    );
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none(),
        "PREMISE (b): the re-entry walks past the answered gate: {}",
        refusal(&second)
    );
    assert_eq!(
        (rig.harness.dispatches(), second.served()),
        (2, 2),
        "PREMISE (b): the re-entry serves both copies and dispatches neither"
    );
    assert_eq!(
        charged(&second),
        none,
        "(b) a fan-out served whole from its commits on re-entry charges nothing"
    );

    // (c) A `repeat` that holds on its second pass: two passes, one dispatch each.
    let rig = Rig::new(
        vec![
            serde_json::json!({"status": "no"}),
            serde_json::json!({"status": "done"}),
        ],
        Vec::new(),
    );
    let report = rig.walk(&[
        repeating(
            agent("attempt", &[]),
            "${attempt.outputs.status} == 'done'",
            3,
            Some("fail"),
            None,
        ),
        step("emit", StepKind::Emit),
    ]);
    assert!(
        report.completed() && rig.harness.dispatches() == 2,
        "PREMISE (c): the repeat holds on pass 2: {}",
        refusal(&report)
    );
    assert_eq!(
        rig.paths("attempt"),
        vec!["repeat:1".to_owned(), "repeat:2".to_owned()],
        "PREMISE (c): each pass commits under its own `repeat` frame"
    );
    assert_eq!(
        charged(&report),
        vec![("repeat:1".to_owned(), 1), ("repeat:2".to_owned(), 1)],
        "(c) each `repeat` frame is a pass of its own, charged by its own dispatch (AC-021)"
    );
}

/// Review routes whose clone answers or refuses, and whose re-executor always refuses: each call is
/// the route TAKEN, so it charges whether it answers or not.
struct R4Routes {
    clone_ok: bool,
}

impl ReviewRoutes for R4Routes {
    fn clone_at(&self, sha: &str) -> Result<Json, StepError> {
        if self.clone_ok {
            Ok(serde_json::json!({ "path": "r4-clone", "sha": sha, "working_tree": "declared" }))
        } else {
            Err(StepError::Unsupported {
                capability: "R4-ROUTES: the clone is refused".to_owned(),
            })
        }
    }

    fn reexecutor(
        &self,
        _workspace: &Json,
    ) -> Result<Box<dyn rk_kernel::review::reexec::ReExecutor + '_>, StepError> {
        Err(StepError::Unsupported {
            capability: "R4-ROUTES: no re-executor".to_owned(),
        })
    }
}

/// **m14 over every call a `tool` step can make** *(dated 2026-09-27, `s8d` build, package
/// R4FIX-8d)*: `build`, `change_summary`, a `toolchain` verb, `shallow_clone`'s `review_clone` route
/// and `verdict_record`'s `review_confirm` route each charge their pass once the call is MADE,
/// answered or not; the refusal before each call (no `range`, no `sha`, no stage `rigor`) charges
/// nothing. `lint` and `test` are pinned above. One fixture per call site, because one per function
/// left five sites unobserved.
#[test]
fn every_tool_call_site_charges_its_pass_once_the_call_is_made() {
    const ROWS: &str = concat!(
        "rebuild = { outputs = \"BuildReport\", live = false, timeout = \"build_timeout\", inputs = [], route = \"toolchain\", side_effecting = false }\n",
        "shallow_clone = { outputs = \"Workspace\", live = false, timeout = \"build_timeout\", inputs = [\"sha\"], route = \"review_clone\", side_effecting = true }\n",
        "verdict_record = { outputs = \"Verdict\", live = false, timeout = \"build_timeout\", inputs = [\"findings\", \"override\"], route = \"review_confirm\", side_effecting = true }\n",
    );
    let emit = || step("emit", StepKind::Emit);
    let rig = |script: Vec<Json>, clone_ok: bool| {
        Rig::routed(
            script,
            Vec::new(),
            ROWS,
            Some(Arc::new(R4Routes { clone_ok }) as Arc<dyn ReviewRoutes>),
        )
    };
    let root = |n: u32| vec![("root".to_owned(), n)];
    let none: Vec<(String, u32)> = Vec::new();

    // `build`: the port answers `passing`, and the walk completes.
    let r = rig(Vec::new(), true);
    let report = r.walk(&[tool("b", "build", &[]), emit()]);
    assert!(
        report.completed(),
        "PREMISE: `build` walks: {}",
        refusal(&report)
    );
    assert_eq!(
        charged(&report),
        root(1),
        "`build`'s port call charges its pass"
    );

    // `change_summary`: the port is reached and refuses (the counting fake's answer).
    let r = rig(Vec::new(), true);
    let report = r.walk(&[tool("c", "change_summary", &[("range", "a..b")]), emit()]);
    let e = refusal(&report);
    assert!(
        e.contains("change_summary") && !report.completed(),
        "PREMISE: the port refuses: {e}"
    );
    assert_eq!(
        charged(&report),
        root(1),
        "a reached `change_summary` charges: {e}"
    );
    let r = rig(Vec::new(), true);
    let report = r.walk(&[tool("c", "change_summary", &[]), emit()]);
    let e = refusal(&report);
    assert!(
        e.contains("inputs.range"),
        "PREMISE: refused before the port: {e}"
    );
    assert_eq!(
        charged(&report),
        none,
        "no `range`, no call, no charge: {e}"
    );

    // A `toolchain` verb: `run_verb` is reached and refuses by the verb's bare name.
    let r = rig(Vec::new(), true);
    let report = r.walk(&[tool("v", "rebuild", &[]), emit()]);
    let e = refusal(&report);
    assert!(
        e.contains("rebuild") && !report.completed(),
        "PREMISE: the verb call refuses: {e}"
    );
    assert_eq!(
        charged(&report),
        root(1),
        "a reached `run_verb` charges: {e}"
    );

    // `review_clone`: the root's clone refuses.
    let r = rig(Vec::new(), false);
    let report = r.walk(&[tool("clone", "shallow_clone", &[("sha", "abc")]), emit()]);
    let e = refusal(&report);
    assert!(
        e.contains("R4-ROUTES: the clone is refused"),
        "PREMISE: {e}"
    );
    assert_eq!(
        charged(&report),
        root(1),
        "a taken `review_clone` route charges: {e}"
    );
    // A `sha` that resolves to no string is refused INSIDE the route's arm, before the clone.
    let r = rig(vec![serde_json::json!({"not": "a sha"})], false);
    let report = r.walk(&[
        agent("a", &[]),
        tool("clone", "shallow_clone", &[("sha", "${a.outputs}")]),
        emit(),
    ]);
    let e = refusal(&report);
    assert!(
        e.contains("did not resolve to a string"),
        "PREMISE: refused before the route: {e}"
    );
    assert_eq!(
        charged(&report),
        root(1),
        "no string `sha`, no clone: only `a` charges: {e}"
    );

    // `review_confirm`: the clone commits a workspace, and the re-executor refuses. The route is
    // taken only under a stage `rigor`; without one it refuses first and charges nothing.
    let confirm = || {
        vec![
            agent("review", &[]),
            tool("clone", "shallow_clone", &[("sha", "abc")]),
            tool(
                "record",
                "verdict_record",
                &[("findings", "${review.outputs}"), ("override", "none")],
            ),
            emit(),
        ]
    };
    let facets = {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let src = std::fs::read_to_string(root.join(".redkiln").join("facets.toml"))
            .expect("the shipped facet registry is readable");
        rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry loads")
    };
    let ctx = rk_kernel::rigor::with_rigor(
        rk_kernel::predicate::Context::new(),
        rk_kernel::rigor::Rigor::Mandatory,
        &facets,
    );
    let r = rig(vec![serde_json::json!({"items": []})], true);
    let report = r.walk_in(&confirm(), &ctx);
    let e = refusal(&report);
    assert!(
        e.contains("R4-ROUTES: no re-executor") && report.committed(),
        "PREMISE: `review` and `clone` commit, and the re-executor refuses: {e}"
    );
    assert_eq!(
        charged(&report),
        root(3),
        "`review`, `clone` and a taken `review_confirm` route each charge: {e}"
    );
    let r = rig(vec![serde_json::json!({"items": []})], true);
    let report = r.walk(&confirm());
    let e = refusal(&report);
    assert!(
        e.contains("rigor"),
        "PREMISE: refused before the route: {e}"
    );
    assert_eq!(
        charged(&report),
        root(2),
        "no stage `rigor`, no route taken: only `review` and `clone` charge: {e}"
    );
}

/// **Exhaustion refuses as `StepFailedAfterCommit`, and the committed steps STAND** (ADR-0144; M7
/// ignores `max` and is BOUNDED by the harness script; M8 turns exhaustion into `Continue`).
#[test]
fn loop_back_exhaustion_refuses_as_step_failed_after_commit_and_committed_steps_stand() {
    let steps = draft_verify(Some(serde_json::json!({"to": "draft", "max": 2})));
    let rig = Rig::new(
        vec![
            serde_json::json!({"ok": false, "n": 1}),
            serde_json::json!({"ok": false, "n": 2}),
            serde_json::json!({"ok": false, "n": 3}),
        ],
        Vec::new(),
    );
    let report = rig.walk(&steps);
    let e = refusal(&report);
    assert!(
        e.contains("`verify`") && e.contains("did not hold") && e.contains("exhausted"),
        "after `max` loop-backs the check refuses by name, naming its exhaustion: {e}"
    );
    assert_eq!(
        rig.harness.dispatches(),
        3,
        "the first pass and exactly `max` = 2 loop-backs, and not one more: {e}"
    );
    assert!(
        report.step_error().is_some() && report.committed(),
        "a step committed, so the refusal travels on `step_error` with the commit kept — the \
         shape `dispatch_through` maps to `ReplayError::StepFailedAfterCommit` (ADR-0144): {e}"
    );
    assert!(
        matches!(report.step_error(), Some(StepError::Unsupported { .. })),
        "the class is `Unsupported`, so the exit code is that class's own row (ADR-0144 decision 6)"
    );
    assert_eq!(
        rig.paths("draft"),
        vec![
            "root".to_owned(),
            "loop_back:1".to_owned(),
            "loop_back:2".to_owned()
        ],
        "every frame's committed step activation STANDS in the journal"
    );
    assert!(
        !rig.completed() && !report.completed(),
        "an exhausted loop never completes the activation"
    );
}

/// **`"continue"` proceeds; `"fail"` refuses** — as `StepFailedAfterCommit` after a commit, and as
/// a plain refusal when nothing committed (ADR-0103 clause 6's keyword forms).
#[test]
fn continue_proceeds_and_fail_refuses() {
    let rig = Rig::new(vec![serde_json::json!({"ok": false})], Vec::new());
    let report = rig.walk(&draft_verify(Some(serde_json::json!("continue"))));
    assert!(
        report.completed() && report.step_error().is_none(),
        "`continue` proceeds to the next step: {}",
        refusal(&report)
    );
    assert_eq!(rig.harness.dispatches(), 1, "and loops back to nothing");

    let rig = Rig::new(vec![serde_json::json!({"ok": false})], Vec::new());
    let report = rig.walk(&draft_verify(Some(serde_json::json!("fail"))));
    let e = refusal(&report);
    assert!(
        e.contains("`verify`") && e.contains("did not hold") && e.contains("keyword `fail`"),
        "`fail` refuses by name: {e}"
    );
    assert!(
        report.step_error().is_some() && report.committed() && !report.completed(),
        "after a commit the refusal is `StepFailedAfterCommit`'s shape and the commit stands: {e}"
    );

    // Nothing committed: a check with `fail` over a literal input refuses outright.
    let rig = Rig::new(Vec::new(), Vec::new());
    let lone = vec![
        check_on_false(
            "verify",
            &[("d", "unready")],
            "d == 'ready'",
            serde_json::json!("fail"),
        ),
        step("emit", StepKind::Emit),
    ];
    let report = rig.walk(&lone);
    let e = refusal(&report);
    assert!(
        report.step_error().is_none()
            && !report.committed()
            && matches!(report.outcome(), Some(Err(_)))
            && e.contains("did not hold"),
        "with nothing committed, `fail` is a refusal and not `StepFailedAfterCommit`: {e}"
    );
}

/// **No `on_false` refuses as the exhausted case** (D21-42 4d: *"exhaustion or no `on_false`
/// refuses as `StepFailedAfterCommit`"*).
#[test]
fn a_false_check_with_no_on_false_refuses_as_step_failed_after_commit() {
    let rig = Rig::new(vec![serde_json::json!({"ok": false})], Vec::new());
    let report = rig.walk(&draft_verify(None));
    let e = refusal(&report);
    assert!(
        e.contains("`verify`") && e.contains("did not hold") && e.contains("no `on_false`"),
        "a false check with no `on_false` refuses by name: {e}"
    );
    assert!(
        report.step_error().is_some() && report.committed() && !report.completed(),
        "the draft committed, so the refusal travels on `step_error` and the commit stands: {e}"
    );
    assert_eq!(rig.harness.dispatches(), 1, "nothing loops back");
    assert_eq!(rig.paths("draft"), vec!["root".to_owned()]);
}

/// **`{ to = "deviate" }` refuses by name, naming ADR-0046's route OWED** (D21-48 X7: the narrowing
/// of D21-42 4d's *"keyword forms follow the schema"*, which R3 states).
#[test]
fn a_deviate_on_false_refuses_by_name() {
    let rig = Rig::new(vec![serde_json::json!({"ok": false})], Vec::new());
    let report = rig.walk(&draft_verify(Some(
        serde_json::json!({"to": "deviate", "kind": "skip", "target": "Spec"}),
    )));
    let e = refusal(&report);
    assert!(
        e.contains("`verify`")
            && e.contains("deviate")
            && e.contains("ADR-0046")
            && e.contains("OWED"),
        "a `deviate` on_false refuses by name, naming ADR-0046's route OWED: {e}"
    );
    assert!(
        report.step_error().is_some() && report.committed(),
        "the draft committed, so it is `StepFailedAfterCommit`'s shape: {e}"
    );
    assert_eq!(
        rig.harness.dispatches(),
        1,
        "`deviate` is never read as a loop-back to a step named `deviate`"
    );
}
