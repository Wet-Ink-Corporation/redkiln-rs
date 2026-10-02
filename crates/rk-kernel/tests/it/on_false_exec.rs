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
                activity_id_at_version: None,
                // *(dated 2026-10-02, s8h build, package P2-rk-types; ADR-0197, proposed.)* No init read here.
                observed_model: None,
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
///
/// *(dated 2026-09-27, `s8d` build, package PRECP-8d; refuting round 9's tests-mutants and
/// checkpoint minors.)* A `continue` route continues the walk, so its `false` is JOURNALED (D21-77
/// ruling 1; ADR-0174 decision 7's D2177-8d paragraph): a `check`'s `on_false = "continue"`, and a
/// `repeat`'s `until` exhausted at `max` into `on_exhausted = "continue"`. Routing alone cannot see
/// either row (`true` and a `continue` `false` both run on), so the journal is read directly.
/// Mutants MC1 and MC2 (§8d.10) die here.
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
    assert_eq!(
        rig.decisions(),
        vec![decided("verify", "root", "check", false)],
        "a `check`'s `false` routed to `continue` continues the walk and is journaled (MC1)"
    );

    // A `repeat` exhausted at `max` = 1 into `on_exhausted = "continue"`: `g` answers `failing`,
    // the walk proceeds to `emit`, and the exhausted `until`'s `false` is journaled.
    let rig = Rig::new(vec![serde_json::json!({"status": "failing"})], Vec::new());
    let steps = vec![
        repeating(
            agent("g", &[]),
            "${g.outputs.status} == 'passing'",
            1,
            Some("continue"),
            None,
        ),
        step("emit", StepKind::Emit),
    ];
    let report = rig.walk(&steps);
    assert!(
        report.completed() && report.step_error().is_none() && rig.harness.dispatches() == 1,
        "PREMISE (repeat): the one pass fails and `on_exhausted = \"continue\"` proceeds: {}",
        refusal(&report)
    );
    assert_eq!(
        rig.decisions(),
        vec![decided("g", "repeat:1", "until", false)],
        "an `until` exhausted into `continue` continues the walk and is journaled (MC2)"
    );

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

// ================================================================================================
// D21-76: A RE-ENTERED WALK REPLAYS ITS JOURNALED CONTROL DECISIONS
// ================================================================================================
//
// *(dated 2026-09-27, `s8d` build, package D2176-8d; D21-76, option (a); ADR-0174 decision 7 and
// ADR-0175, both `proposed`, amended by dated text.)* D21-76: *"The walk **journals its control
// decisions** — each `check` verdict and each loop-back or escalation decision — under the commit
// key of the frame that took it, and a re-entered walk **replays** them: up to the last journaled
// decision it neither re-executes the deciding `check` nor re-decides a loop; from there, only work
// the prior walk did not reach executes fresh (a crash re-entry therefore still re-verifies
// whatever it had not yet decided). An answered escalate gate thus resumes into its escalating
// step."* Refuting round 7 measured the defect: a re-walk past an answered gate re-ran `test`
// against a tree a later step had changed, re-decided `red-check` and looped back into `red`.

/// One per-step spec per agent step id, each marked with its own id as the input `who`, so the
/// harness's record says WHICH step a dispatch was.
pub fn per_step_specs(ids: &[&str]) -> std::collections::BTreeMap<String, ActivitySpec> {
    ids.iter()
        .map(|id| {
            let mut sp = spec();
            sp.inputs.push(("who".to_owned(), serde_json::json!(id)));
            ((*id).to_owned(), sp)
        })
        .collect()
}

impl Rig {
    /// [`Rig::walk`] with a per-step spec for each agent step (R1), so a walk may dispatch more
    /// than one agent step.
    pub fn walk_specs(
        &self,
        steps: &[Step],
        specs: &std::collections::BTreeMap<String, ActivitySpec>,
    ) -> ActivityReport {
        let state = rk_kernel::workflow::fsm::fold(ITEM, STAGE, &journal(&self.db))
            .unwrap_or_else(|_| empty_state());
        let sp = spec();
        self.rt
            .run_activity_steps(
                rk_kernel::runtime::step::Activation {
                    step_specs: Some(specs),
                    ..activation(&sp)
                },
                Some(steps),
                &rk_kernel::predicate::Context::new(),
                gate_site(&state),
                rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
                at(100),
            )
            .expect("a step's own failure is a VALUE (AC-032)")
    }

    /// Which agent step each dispatch was, in dispatch order (the spec's `who`).
    pub fn who(&self) -> Vec<String> {
        (0..self.harness.dispatches())
            .map(|n| {
                self.harness
                    .input(n, "who")
                    .as_str()
                    .unwrap_or("?")
                    .to_owned()
            })
            .collect()
    }

    /// Every journaled control decision, in journal order: `(step id, iteration path, decision)`,
    /// read off the `decision` member of the rows that carry one.
    pub fn decisions(&self) -> Vec<(String, String, Json)> {
        journal(&self.db)
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
            .filter_map(|e| {
                let decision = e.payload.get("decision")?.clone();
                let step = e.payload.get("step_id")?.as_str()?.to_owned();
                let path = rk_kernel::workflow::idem::IterationPath::from_json(
                    e.payload.get("iteration_path")?,
                )?
                .to_string();
                Some((step, path, decision))
            })
            .collect()
    }
}

/// `(step, path, {"kind": kind, "holds": holds})`, as [`Rig::decisions`] renders one.
fn decided(step: &str, path: &str, kind: &str, holds: bool) -> (String, String, Json) {
    (
        step.to_owned(),
        path.to_owned(),
        serde_json::json!({"kind": kind, "holds": holds}),
    )
}

/// The red loop of `tdd-red-green-refactor`: `red` (agent), `red-verify` (`toolchain:test` over
/// `red`'s tests) and `red-check` (the catalog's own expr and `on_false`).
fn red_loop() -> Vec<Step> {
    vec![
        agent("red", &[("last", "${red-verify.outputs?}")]),
        tool(
            "red-verify",
            "test",
            &[("selection", "${red.outputs.tests}")],
        ),
        check_on_false(
            "red-check",
            &[("report", "${red-verify.outputs}")],
            "report.failed > 0 and report.passed == 0",
            serde_json::json!({"to": "red", "max": 2}),
        ),
    ]
}

/// **D21-76 test (1): a re-entry past an answered gate, with a CHANGED `test` answer, re-dispatches
/// no committed-frame step and resumes after the gate.** Refuting round 7's probe, adapted: the
/// first walk's `red-verify` reports `(0 passed, 1 failed)`, `red-check` holds and the gate opens.
/// Between the walks a later step fixed the test, so the re-executed `red-verify` reports `(1, 0)`.
/// Before D21-76 the re-walk re-decided `red-check` over that answer and looped back into `red`
/// twice (*"R7PROBE dispatches=3 red_paths=[root, loop_back:1, loop_back:2]"*).
#[test]
fn a_re_entry_past_an_answered_gate_replays_the_check_verdict_and_re_dispatches_nothing() {
    let mut steps = red_loop();
    // The gate is guarded over `red-verify`'s output, as ~~17 shipped gates are~~ 15 shipped gates
    // are guarded over a tool step's, through 17 `when` references (a census over
    // `.redkiln/catalog/*.toml`; corrected by dated note 2026-09-27, `s8d` build, package
    // D2177-8d), and a `check` after it reads its answer:
    // re-evaluated over the changed answer, the guard would skip the gate and leave the answer
    // unbound. Its `when` is a journaled decision too, and is replayed.
    steps.push(gate(
        "approve",
        "red.approve",
        Some("${red-verify.outputs.failed} > 0"),
    ));
    steps.push(check(
        "approved",
        &[("a", "${approve.answer}")],
        "a == 'approve'",
    ));
    steps.push(step("emit", StepKind::Emit));
    let tests_red = serde_json::json!({"tests": ["t1"]});
    let rig = Rig::new(
        vec![tests_red.clone(), tests_red.clone(), tests_red],
        vec![(0, 1), (1, 0), (1, 0), (1, 0)],
    );
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "PREMISE: `red-check` holds on (0, 1) and the gate opens: {}",
        refusal(&first)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.tests.selections().len()),
        (1, 1),
        "PREMISE: one `red` dispatch and one `test` call before the gate"
    );
    assert_eq!(
        rig.decisions(),
        vec![
            decided("red-check", "root", "check", true),
            decided("approve", "root", "when", true),
        ],
        "the first walk journals `red-check`'s verdict and the gate's `when` under their commit keys"
    );
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert_eq!(
        rig.tests.selections().len(),
        2,
        "PREMISE: the re-entry re-executed the idempotent `red-verify` (ADR-0174 decision 7), and \
         its answer is now (1, 0)"
    );
    assert_eq!(
        (rig.harness.dispatches(), second.served()),
        (1, 1),
        "the re-entry serves `red` and re-dispatches no committed-frame step: the journaled verdict \
         of `red-check` is replayed, not re-decided over the changed answer: {}",
        refusal(&second)
    );
    assert_eq!(
        rig.paths("red"),
        vec!["root".to_owned()],
        "no loop-back frame was opened on re-entry"
    );
    assert!(
        second.completed() && second.step_error().is_none(),
        "the walk resumes after the answered gate and completes: {}",
        refusal(&second)
    );
    assert_eq!(
        rig.decisions(),
        vec![
            decided("red-check", "root", "check", true),
            decided("approve", "root", "when", true),
            decided("approved", "root", "check", true),
        ],
        "a replayed decision is not journaled again; the check the prior walk never reached is \
         decided fresh"
    );
    let state = rk_kernel::workflow::fsm::fold(ITEM, STAGE, &journal(&rig.db)).expect("folds");
    assert_eq!(
        state.committed().count(),
        2,
        "the fold counts `red`'s commit and the completion, and never the decision's row"
    );
}

/// **D21-76 test (2): a tdd-shaped escalate re-entry dispatches `green`, not `red`, after the
/// answer.** `tdd-red-green-refactor`'s shape: the red loop, then `green` (repeat until
/// `green-verify` passes, max 2, escalating to `tdd.green-exhausted`), `green-verify`, the guarded
/// `green-gate`, and the `emit`. `green` changes the tree before the gate opens, so on the answer's
/// re-entry every re-executed `test` reports `(1, 0)`: re-decided, `red-check` would loop back into
/// `red`, the last pass's `until` would hold (no escalation), and `green-gate`'s `when` would skip
/// the gate. Replayed, the walk reaches the answered gate and resumes INTO `green` (schema §3).
#[test]
fn a_tdd_shaped_escalate_re_entry_dispatches_green_not_red_after_the_answer() {
    let mut steps = red_loop();
    steps.push(repeating(
        agent(
            "green",
            &[
                ("red", "${red.outputs}"),
                ("last", "${green-verify.outputs?}"),
                ("hint", "${green-gate.answer?}"),
            ],
        ),
        "${green-verify.outputs.status} == 'passing'",
        2,
        Some("escalate"),
        Some("tdd.green-exhausted"),
    ));
    steps.push(tool(
        "green-verify",
        "test",
        &[("selection", "${red.outputs.tests}")],
    ));
    steps.push(gate(
        "green-gate",
        "tdd.green-exhausted",
        Some("${green-verify.outputs.status} != 'passing'"),
    ));
    steps.push(step("emit", StepKind::Emit));
    let specs = per_step_specs(&["red", "green"]);
    let rig = Rig::new(
        vec![
            serde_json::json!({"tests": ["t1"]}),
            serde_json::json!({"n": 1}),
            serde_json::json!({"n": 2}),
            serde_json::json!({"n": 3}),
            serde_json::json!({"n": 4}),
        ],
        vec![
            (0, 1),
            (0, 1),
            (0, 1),
            (1, 0),
            (1, 0),
            (1, 0),
            (1, 0),
            (1, 0),
        ],
    );
    let first = rig.walk_specs(&steps, &specs);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "PREMISE: `green` exhausts its two passes and escalates to `green-gate`: {}",
        refusal(&first)
    );
    assert_eq!(
        rig.who(),
        vec!["red", "green", "green"],
        "PREMISE: one `red`, two `green` passes"
    );
    rig.answer(&first, "try-the-other-branch");
    let second = rig.walk_specs(&steps, &specs);
    assert_eq!(
        rig.who(),
        vec!["red", "green", "green", "green"],
        "the answer resumes INTO `green` (schema §3) and never loops back into `red`: {}",
        refusal(&second)
    );
    assert_eq!(
        rig.harness.input(3, "hint"),
        serde_json::json!("try-the-other-branch"),
        "the resumed `green` carries the gate's answer"
    );
    assert_eq!(
        rig.paths("green"),
        vec!["repeat:1", "repeat:2", "loop_back:1/repeat:1"],
        "the escalate re-entry opens a new frame of `green`, and a journaled decision is never read \
         as a step commit"
    );
    assert_eq!(rig.paths("red"), vec!["root".to_owned()]);
    assert!(
        second.completed() && second.step_error().is_none(),
        "the resumed pass passes and the walk completes: {}",
        refusal(&second)
    );
}

/// **D21-76 test (3), the control: a CRASH re-entry replays only the decisions it journaled and
/// re-decides the rest.** `a` → `ta` → `ca` (loop to `a`) → `b` → `tb` → `cb` (loop to `b`) →
/// `emit`. A complete first walk decides `ca` true, `cb` false (a loop-back) and `cb` true at
/// `loop_back:1`. The process is taken to have died after `b`'s `loop_back:1` commit, so the
/// journal is cut there: `cb`'s `loop_back:1` verdict was never journaled. On re-entry `ta` answers
/// differently, and `ca` and `cb@root` are replayed; `cb@loop_back:1` is decided FRESH over the
/// re-executed `tb`, in two arms: false (a new loop-back dispatches `b` once) and true (nothing is
/// dispatched).
#[test]
fn a_crash_re_entry_replays_only_the_decisions_it_journaled_and_re_decides_the_rest() {
    let steps = vec![
        agent("a", &[]),
        tool("ta", "test", &[("selection", "${a.outputs.tests}")]),
        check_on_false(
            "ca",
            &[("report", "${ta.outputs}")],
            "report.failed == 0",
            serde_json::json!({"to": "a", "max": 2}),
        ),
        agent("b", &[]),
        tool("tb", "test", &[("selection", "${a.outputs.tests}")]),
        check_on_false(
            "cb",
            &[("report", "${tb.outputs}")],
            "report.failed == 0",
            serde_json::json!({"to": "b", "max": 2}),
        ),
        step("emit", StepKind::Emit),
    ];
    let specs = per_step_specs(&["a", "b"]);
    let tests = serde_json::json!({"tests": ["t1"]});
    let full = Rig::new(
        vec![tests.clone(), tests.clone(), tests.clone()],
        vec![(1, 0), (0, 1), (1, 0)],
    );
    let report = full.walk_specs(&steps, &specs);
    assert!(
        report.completed() && full.who() == vec!["a", "b", "b"],
        "PREMISE: the complete walk loops back into `b` once: {:?} {}",
        full.who(),
        refusal(&report)
    );
    // The crash: the journal up to and including `b`'s `loop_back:1` commit.
    let rows = journal(&full.db);
    let cut = rows
        .iter()
        .position(|e| {
            !is_completion_row(e)
                && commit_key_of(e).is_some_and(|k| {
                    k.step_id == "b" && k.iteration_path.to_string() == "loop_back:1"
                })
        })
        .expect("PREMISE: `b` committed at loop_back:1");
    let prefix = &rows[..=cut];

    for (fresh_tb, want_who, want_b) in [
        (
            (0, 1),
            vec!["b"],
            vec!["root", "loop_back:1", "loop_back:2"],
        ),
        ((1, 0), Vec::new(), vec!["root", "loop_back:1"]),
    ] {
        let tail = if fresh_tb == (0, 1) {
            vec![(0, 1), (0, 1), fresh_tb, (1, 0)]
        } else {
            vec![(0, 1), (0, 1), fresh_tb]
        };
        let rig = Rig::new(vec![tests.clone()], tail);
        for row in prefix {
            rig.seed(row.clone());
        }
        let re = rig.walk_specs(&steps, &specs);
        assert_eq!(
            rig.who(),
            want_who,
            "fresh `tb` {fresh_tb:?}: `ca` and `cb@root` are replayed (the changed `ta` answer does \
             not loop back into `a`), and `cb@loop_back:1`, never journaled, is decided FRESH: {}",
            refusal(&re)
        );
        assert_eq!(
            rig.paths("b"),
            want_b,
            "fresh `tb` {fresh_tb:?}: `b`'s committed frames"
        );
        assert!(
            re.completed() && re.step_error().is_none(),
            "fresh `tb` {fresh_tb:?}: the re-entry completes: {}",
            refusal(&re)
        );
        assert!(
            rig.decisions()
                .contains(&decided("cb", "loop_back:1", "check", fresh_tb == (1, 0))),
            "fresh `tb` {fresh_tb:?}: the re-entry journals the verdict it decided: {:?}",
            rig.decisions()
        );
    }
}

// ================================================================================================
// D21-77: A JOURNALED REFUSAL IS NOT REPLAYED; DECISION ROWS DO NOT DRAW ON THE RETRY CAP
// ================================================================================================
//
// *(dated 2026-09-27, `s8d` build, package D2177-8d; D21-77 rulings 1 and 2; ADR-0174 decision 7
// and ADR-0175, both `proposed`, amended by dated text.)* D21-77: *"Replay only decisions whose
// route continues the walk — a `true`, a loop-back, a `repeat` pass, an escalation. A `false` routed
// to a refusal is **not replayed**: the next run re-verifies it"*, and *"decision rows draw no
// attempt from AC-029's cap (the retry cap counts dispatch attempts only), and a gate `when`
// deciding `false` is journaled and replayed under a killing test."* Refuting round 8 measured both
// defects: *"R8PROBE second completed=false tests_called=2"* and *"R8CAP probe(3 checks)
// dispatches=2 completed=false"*.

/// **D21-77 ruling 1: a `false` routed to a refusal is not journaled, so a refused-then-fixed
/// re-run completes.** Refuting round 8's probe shape, in four arms, each refused on its first run
/// and fixed before its second: (a) `on_false = "fail"`, (b) no `on_false`, (c) a `{ to, max }`
/// loop-back exhausted after one loop-back, and (d) a `repeat` exhausted into `on_exhausted =
/// "fail"`. Built as NOT JOURNALED (not journaled-and-skipped): the refusing decision leaves no row,
/// so the next run decides it fresh over the re-executed tool answer. A decision whose route
/// continued (arm (c)'s first loop-back) is still journaled and replayed.
#[test]
fn a_refused_decision_is_not_replayed_so_a_refused_then_fixed_re_run_completes() {
    let tests = serde_json::json!({"tests": ["t1"]});
    let emit = || step("emit", StepKind::Emit);
    let rv = |on: &str| {
        tool(
            "rv",
            "test",
            &[("selection", &format!("${{{on}.outputs.tests}}"))],
        )
    };
    let c = |on_false: Option<Json>| {
        let inputs = [("report", "${rv.outputs}")];
        on_false.map_or_else(
            || check("c", &inputs, "report.failed == 0"),
            |v| check_on_false("c", &inputs, "report.failed == 0", v),
        )
    };

    // (a) and (b): the probe's walk, `red` → `rv` → `c` → `emit`, answers (0, 1) then (1, 0).
    for (label, on_false) in [
        ("fail", Some(serde_json::json!("fail"))),
        ("no on_false", None),
    ] {
        let steps = vec![agent("red", &[]), rv("red"), c(on_false), emit()];
        let rig = Rig::new(vec![tests.clone()], vec![(0, 1), (1, 0)]);
        let first = rig.walk(&steps);
        assert!(
            first.step_error().is_some() && !first.completed(),
            "PREMISE ({label}): `c` does not hold on (0, 1) and the walk refuses: {}",
            refusal(&first)
        );
        let second = rig.walk(&steps);
        assert!(
            second.completed() && second.step_error().is_none(),
            "({label}) the re-run re-verifies the refused `c` over the fixed tree and completes: {}",
            refusal(&second)
        );
        assert_eq!(
            (rig.harness.dispatches(), rig.tests.selections().len()),
            (1, 2),
            "({label}) `red` is served, `rv` re-executes once per run"
        );
        assert_eq!(
            rig.decisions(),
            vec![decided("c", "root", "check", true)],
            "({label}) the refusing `false` left no row; the re-run journals the verdict it decided"
        );
    }

    // (c) an exhausted loop-back: `a` → `rv` → `c` (`{ to = "a", max = 1 }`) → `emit`.
    let steps = vec![
        agent("a", &[]),
        rv("a"),
        c(Some(serde_json::json!({"to": "a", "max": 1}))),
        emit(),
    ];
    let rig = Rig::new(
        vec![tests.clone(), tests.clone()],
        vec![(0, 1), (0, 1), (0, 1), (1, 0)],
    );
    let first = rig.walk(&steps);
    assert!(
        first.step_error().is_some() && !first.completed() && refusal(&first).contains("exhausted"),
        "PREMISE (exhausted): one loop-back, then `c` refuses as exhausted: {}",
        refusal(&first)
    );
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none(),
        "(exhausted) the re-run replays the loop-back, re-verifies the refused `c@loop_back:1` and \
         completes: {}",
        refusal(&second)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.paths("a")),
        (2, vec!["root".to_owned(), "loop_back:1".to_owned()]),
        "(exhausted) both of `a`'s frames are served and no third frame is opened"
    );
    assert_eq!(
        rig.decisions(),
        vec![
            decided("c", "root", "check", false),
            decided("c", "loop_back:1", "check", true),
        ],
        "(exhausted) the loop-back's `false` continued the walk and is journaled; the exhausted \
         one was not"
    );

    // (d) a `repeat` exhausted into `fail`: `g` (until `rv` passes, max 1) → `rv` → `emit`.
    let steps = vec![
        repeating(
            agent("g", &[]),
            "${rv.outputs.status} == 'passing'",
            1,
            Some("fail"),
            None,
        ),
        rv("g"),
        emit(),
    ];
    let rig = Rig::new(vec![tests], vec![(0, 1), (1, 0)]);
    let first = rig.walk(&steps);
    assert!(
        first.step_error().is_some() && refusal(&first).contains("exhausted"),
        "PREMISE (repeat): the one pass fails and `on_exhausted = \"fail\"` refuses: {}",
        refusal(&first)
    );
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none(),
        "(repeat) the re-run re-evaluates the refused `until` over the fixed tree and completes: {}",
        refusal(&second)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.decisions()),
        (1, vec![decided("g", "repeat:1", "until", true)]),
        "(repeat) `g` is served; the refusing `until` left no row"
    );
}

/// **D21-77 ruling 2a: journaled decisions draw nothing from AC-029's retry cap; the cap counts
/// dispatch attempts only.** Refuting round 8's two probe shapes at the rig's `CAP` = 4:
/// (i) `a` → `ta` → N `check`s over `ta` → `b` → `emit`, `b`'s first dispatch failing transiently,
/// at N = 0 (the control) and N = 3; (ii) `draft` → `verify` (`{ to = "draft", max = 3 }`) →
/// `emit`, answering `ok: false` twice, with its third dispatch failing transiently. Each retries
/// and completes. The decision rows keep attempts of their own: no two `activity_committed` rows of
/// the activation share one. CONTROL: three transient failures of `b` still reach the cap at the
/// fourth dispatch, so the cap is counted, not dropped.
#[test]
fn journaled_decisions_draw_nothing_from_the_retry_cap() {
    let tests = serde_json::json!({"tests": ["t1"]});
    let shape = |n: usize| {
        let mut steps = vec![
            agent("a", &[]),
            tool("ta", "test", &[("selection", "${a.outputs.tests}")]),
        ];
        for k in 0..n {
            steps.push(check(
                &format!("c{k}"),
                &[("report", "${ta.outputs}")],
                "report.failed == 0",
            ));
        }
        steps.push(agent("b", &[]));
        steps.push(step("emit", StepKind::Emit));
        steps
    };
    let specs = per_step_specs(&["a", "b"]);
    let attempts_unique = |rig: &Rig| {
        let mut seen: Vec<u64> = journal(&rig.db)
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
            .filter_map(|e| e.payload.get("attempt").and_then(Json::as_u64))
            .collect();
        let all = seen.len();
        seen.sort_unstable();
        seen.dedup();
        all == seen.len()
    };
    for n in [0, 3] {
        let rig = Rig::new(
            vec![tests.clone(), serde_json::json!({"n": 1})],
            vec![(1, 0)],
        );
        rig.harness.fail_transiently_at(2);
        let report = rig.walk_specs(&shape(n), &specs);
        assert!(
            report.completed() && report.step_error().is_none(),
            "({n} checks) `b`'s transient failure is retried under the cap and the walk completes: \
             {}",
            refusal(&report)
        );
        assert_eq!(
            rig.who(),
            vec!["a", "b", "b"],
            "({n} checks) one retry of `b`, whatever the decisions journaled before it"
        );
        assert_eq!(
            rig.decisions().len(),
            n,
            "PREMISE ({n} checks): each check journaled its verdict"
        );
        assert!(
            attempts_unique(&rig),
            "({n} checks) no two rows share an attempt"
        );
    }

    // (ii) the loop-back shape: the third `draft` dispatch fails transiently and is retried.
    let rig = Rig::new(
        vec![
            serde_json::json!({"ok": false}),
            serde_json::json!({"ok": false}),
            serde_json::json!({"ok": true}),
        ],
        Vec::new(),
    );
    rig.harness.fail_transiently_at(3);
    let report = rig.walk(&draft_verify(Some(
        serde_json::json!({"to": "draft", "max": 3}),
    )));
    assert!(
        report.completed() && report.step_error().is_none(),
        "(loop-back) the third frame's transient failure is retried and the walk completes: {}",
        refusal(&report)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.decisions().len()),
        (4, 3),
        "(loop-back) four dispatches, one of them the retry; three verdicts journaled"
    );
    assert!(
        attempts_unique(&rig),
        "(loop-back) no two rows share an attempt"
    );

    // CONTROL: the cap still counts dispatches. `b` fails at dispatches 2, 3 and 4; the fourth is
    // the cap's (`attempt >= CAP`), so it is not retried.
    let rig = Rig::new(vec![tests, serde_json::json!({"n": 1})], vec![(1, 0)]);
    for d in [2, 3, 4] {
        rig.harness.fail_transiently_at(d);
    }
    let report = rig.walk_specs(&shape(3), &specs);
    assert!(
        !report.completed() && refusal(&report).contains("LOOP-HARNESS TRANSIENT: dispatch 4"),
        "CONTROL: the fourth dispatch is the cap's, and its failure is not retried: {}",
        refusal(&report)
    );
    assert_eq!(
        rig.harness.dispatches(),
        4,
        "CONTROL: exactly CAP = 4 dispatches"
    );
}

/// **D21-77 ruling 2b: a gate `when` that decides `false` is journaled and replayed.** `a` → `t`
/// (`toolchain:test`) → the gate `g`, guarded on `${t.outputs.failed} > 0` → an unguarded
/// `approve` gate that cuts the walk → `emit`. The first walk's `t` reports (1, 0), so `g` is walked
/// past and `approve` opens. The re-entry's `t` reports (0, 1): re-decided, `g`'s guard would now
/// hold and open a gate the first walk walked past. Replayed, the walk passes `g` and completes.
#[test]
fn a_gate_when_that_decides_false_is_journaled_and_replayed() {
    let steps = vec![
        agent("a", &[]),
        tool("t", "test", &[("selection", "${a.outputs.tests}")]),
        gate("g", "a.escalate", Some("${t.outputs.failed} > 0")),
        gate("approve", "a.approve", None),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![serde_json::json!({"tests": ["t1"]})],
        vec![(1, 0), (0, 1)],
    );
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "PREMISE: `g` is walked past and `approve` opens: {}",
        refusal(&first)
    );
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert!(
        second.gate_opened().is_none() && second.completed() && second.step_error().is_none(),
        "the re-entry replays `g`'s journaled `false` over the changed `t` answer, opens no gate and \
         completes: {}",
        refusal(&second)
    );
    assert_eq!(
        (rig.tests.selections().len(), rig.harness.dispatches()),
        (2, 1),
        "PREMISE: `t` re-executed on the re-entry (ADR-0174 decision 7); `a` was served"
    );
    assert_eq!(
        rig.decisions(),
        vec![decided("g", "root", "when", false)],
        "`g`'s `false` is journaled once; `approve`, with no `when`, journals nothing"
    );
}

/// `(step, path, {"kind": kind, "holds": null})`: a `null` decision, as [`Rig::decisions`] renders
/// one *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92)*.
fn decided_null(step: &str, path: &str, kind: &str) -> (String, String, Json) {
    (
        step.to_owned(),
        path.to_owned(),
        serde_json::json!({"kind": kind, "holds": null}),
    )
}

// ================================================================================================
// D21-92: A ROUTING `null` IS JOURNALED WHEN ITS ROUTE CONTINUES THE WALK; ONE ROUTED TO A REFUSAL
// IS NOT
// ================================================================================================
//
// *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed, superseding
// ADR-0174 decision 7 in part — its null sentence only.)* D21-92: *"A `null` `until` or gate `when`
// is journaled on the same rule as a `true` or `false`: **when its route continues the walk**. A
// `null` routed to a refusal leaves no row, so a refused-then-fixed re-run still completes
// (D21-77)."* These two tests replace `a_null_until_or_gate_when_is_not_journaled`, which pinned the
// rule the ruling reverses. Each `null` below is a Kleene disjunction over an agent step's `null`
// member and a tool step's count: `null or false` is `null`, and `null or true` is `true`, so a
// re-executed tool answer that changed between walks would decide the disjunction differently if it
// were re-evaluated.

/// **D21-92, the continuing arm: a `null` `until` or gate `when` whose route continues the walk is
/// journaled, and a re-entry replays it over a changed tool answer.**
///
/// (a) `until`: `g` (agent, `repeat` max 2) → `rv` (`toolchain:test`), `until = "${g.outputs.status}
/// == 'passing' or ${rv.outputs.failed} == 0"`, then an unguarded `approve` gate that cuts the walk.
/// Pass 1 answers `status: null` over `(0, 1)`, so the `until` is `null` and takes pass 2, which
/// holds. Between the walks the tree was fixed, so the re-entry's `rv` answers `(1, 0)`: re-evaluated,
/// pass 1's `until` would now hold and end the repeat a pass early. Replayed, the walk takes pass 2
/// again (served), replays its `true`, and completes past the answered gate.
///
/// (b) gate `when`: `a` → `t` (`toolchain:test`) → the gate `g`, `when = "${a.outputs.status} ==
/// 'stuck' or ${t.outputs.failed} > 0"` → `approve` → `emit`. The first walk's `t` reports `(1, 0)`,
/// so `g`'s `when` is `null` and `g` is walked past (a skipped gate always continues the walk).
/// The re-entry's `t` reports `(0, 1)`: re-evaluated, `g` would open a gate the first walk never
/// asked. Replayed, the walk passes `g` and completes.
#[test]
fn a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed() {
    // (a) the `until`.
    let steps = vec![
        repeating(
            agent("g", &[]),
            "${g.outputs.status} == 'passing' or ${rv.outputs.failed} == 0",
            2,
            Some("fail"),
            None,
        ),
        tool("rv", "test", &[("selection", "${g.outputs.tests}")]),
        gate("approve", "a.approve", None),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"status": null, "tests": ["t1"]}),
            serde_json::json!({"status": "passing", "tests": ["t1"]}),
        ],
        vec![(0, 1), (0, 1), (1, 0), (1, 0)],
    );
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "PREMISE (until): pass 1's `null` takes pass 2, which holds, and `approve` opens: {}",
        refusal(&first)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.tests.selections().len()),
        (2, 2),
        "PREMISE (until): two passes, each dispatching `g` and running `rv` once"
    );
    assert_eq!(
        rig.decisions(),
        vec![
            decided_null("g", "repeat:1", "until"),
            decided("g", "repeat:2", "until", true),
        ],
        "(until) pass 1's `null` continued the walk (another pass), so it is journaled as `null` \
         beside pass 2's `true` (D21-92)"
    );
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none() && second.gate_opened().is_none(),
        "(until) the re-entry replays pass 1's `null` and pass 2's `true` and completes: {}",
        refusal(&second)
    );
    assert_eq!(
        (
            rig.harness.dispatches(),
            rig.tests.selections().len(),
            rig.paths("g")
        ),
        (2, 4, vec!["repeat:1".to_owned(), "repeat:2".to_owned()]),
        "(until) both of `g`'s passes are served and `rv` re-executes in each: pass 1's `until` was \
         NOT re-decided over the fixed `(1, 0)`, which would have ended the repeat at pass 1 and run \
         `rv` once"
    );
    assert_eq!(
        rig.decisions(),
        vec![
            decided_null("g", "repeat:1", "until"),
            decided("g", "repeat:2", "until", true),
        ],
        "(until) the re-entry journals nothing new: both decisions were replayed"
    );

    // (b) the gate `when`.
    let steps = vec![
        agent("a", &[]),
        tool("t", "test", &[("selection", "${a.outputs.tests}")]),
        gate(
            "g",
            "a.escalate",
            Some("${a.outputs.status} == 'stuck' or ${t.outputs.failed} > 0"),
        ),
        gate("approve", "a.approve", None),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![serde_json::json!({"status": null, "tests": ["t1"]})],
        vec![(1, 0), (0, 1)],
    );
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some() && first.step_error().is_none(),
        "PREMISE (when): `g`'s `null` walks past it and `approve` opens: {}",
        refusal(&first)
    );
    assert_eq!(
        rig.decisions(),
        vec![decided_null("g", "root", "when")],
        "(when) `g`'s `null` continued the walk (the gate is walked past), so it is journaled as \
         `null` (D21-92); `approve`, with no `when`, journals nothing"
    );
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert!(
        second.gate_opened().is_none() && second.completed() && second.step_error().is_none(),
        "(when) the re-entry replays `g`'s journaled `null` over the changed `t` answer, opens no \
         gate and completes: {}",
        refusal(&second)
    );
    assert_eq!(
        (rig.tests.selections().len(), rig.harness.dispatches()),
        (2, 1),
        "PREMISE (when): `t` re-executed on the re-entry (ADR-0174 decision 7); `a` was served"
    );
    assert_eq!(
        rig.decisions(),
        vec![decided_null("g", "root", "when")],
        "(when) the re-entry journals nothing new"
    );
}

/// **D21-92, the refusal arm: a `null` `until` routed to a refusal leaves no row, so a
/// refused-then-fixed re-run completes** (D21-77's rule, kept for a `null`). `g` (agent, `repeat`
/// max 1, `on_exhausted = "fail"`) → `rv` → `emit`, `until = "${g.outputs.status} == 'passing' or
/// ${rv.outputs.failed} == 0"`. The first walk's pass answers `status: null` over `(0, 1)`: the
/// `until` is `null` at `max`, and `"fail"` refuses. The tree is fixed before the re-run, whose `rv`
/// answers `(1, 0)`: the `until` is decided fresh, holds, and the walk completes. A gate `when` has
/// no refusal arm — a skipped gate always continues the walk — so this arm is the `until`'s alone.
#[test]
fn a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes() {
    let steps = vec![
        repeating(
            agent("g", &[]),
            "${g.outputs.status} == 'passing' or ${rv.outputs.failed} == 0",
            1,
            Some("fail"),
            None,
        ),
        tool("rv", "test", &[("selection", "${g.outputs.tests}")]),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![serde_json::json!({"status": null, "tests": ["t1"]})],
        vec![(0, 1), (1, 0)],
    );
    let first = rig.walk(&steps);
    assert!(
        first.step_error().is_some() && !first.completed() && refusal(&first).contains("exhausted"),
        "PREMISE: the one pass's `until` is `null` at `max`, and `on_exhausted = \"fail\"` refuses: \
         {}",
        refusal(&first)
    );
    assert_eq!(
        rig.decisions(),
        Vec::new(),
        "the `null` routed to a refusal leaves no row (D21-92; D21-77)"
    );
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none(),
        "the re-run re-evaluates the refused `until` over the fixed tree and completes: {}",
        refusal(&second)
    );
    assert_eq!(
        (
            rig.harness.dispatches(),
            rig.tests.selections().len(),
            rig.decisions()
        ),
        (1, 2, vec![decided("g", "repeat:1", "until", true)]),
        "`g` is served, `rv` re-executes once, and the re-run journals the `true` it decided"
    );
}

/// **A decision is appended at most once per commit key AND kind** (ADR-0174 decision 7:
/// *"appended at most once per commit key and kind (`Runtime::commit_decision`)"*). The same
/// `check` decision twice under one key appends one row; a `when` under the same key is a different
/// kind and appends its own.
#[test]
fn a_decision_is_journaled_at_most_once_per_commit_key_and_kind() {
    use rk_kernel::runtime::activity::{Decision, DecisionKind, StepAt};
    let rig = Rig::new(Vec::new(), Vec::new());
    let path = rk_kernel::workflow::idem::IterationPath::root();
    let at_c = StepAt {
        step_id: "c",
        iteration_path: &path,
        family: None,
    };
    let check_true = Decision {
        kind: DecisionKind::Check,
        holds: rk_kernel::predicate::Tri::True,
    };
    for _ in 0..2 {
        rig.rt
            .commit_decision(ITEM, crate::runtime::AID, at_c, check_true, at(100))
            .expect("appends");
    }
    assert_eq!(
        rig.decisions(),
        vec![decided("c", "root", "check", true)],
        "the second `check` decision under the same key appends nothing"
    );
    rig.rt
        .commit_decision(
            ITEM,
            crate::runtime::AID,
            at_c,
            Decision {
                kind: DecisionKind::When,
                holds: rk_kernel::predicate::Tri::False,
            },
            at(100),
        )
        .expect("appends");
    assert_eq!(
        rig.decisions(),
        vec![
            decided("c", "root", "check", true),
            decided("c", "root", "when", false),
        ],
        "a decision of another kind under the same key is its own row"
    );
}

/// **A replayed `true` `check` is still marked run**, so a bare `${c}` evidence pointer on the
/// `emit` resolves on the re-entry (ADR-0173 decision 1, as R6FIX-8d built it: the pointer
/// *"resolves only once the step has run in this activation"*). `a` → `c` (holds) → `approve` (cuts
/// the walk) → `emit` with evidence `${c}`. The re-entry replays `c` and completes with `c`'s
/// pointer on the completion row.
#[test]
fn a_replayed_true_check_is_marked_run_for_the_emit_evidence() {
    let mut emit = step("emit", StepKind::Emit);
    emit.emit = Some(rk_kernel::catalog::EmitBinding {
        artifacts: std::collections::BTreeMap::new(),
        evidence: vec!["${c}".to_owned()],
    });
    let steps = vec![
        agent("a", &[]),
        check("c", &[("d", "${a.outputs}")], "d.ok == true"),
        gate("approve", "a.approve", None),
        emit,
    ];
    let rig = Rig::new(vec![serde_json::json!({"ok": true})], Vec::new());
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some(),
        "PREMISE: `c` holds and `approve` opens: {}",
        refusal(&first)
    );
    rig.answer(&first, "approve");
    let second = rig.walk(&steps);
    assert!(
        second.completed() && second.step_error().is_none(),
        "the re-entry replays `c`, marks it run, and the `emit`'s `${{c}}` resolves: {}",
        refusal(&second)
    );
    let completion: Vec<Event> = journal(&rig.db)
        .into_iter()
        .filter(is_completion_row)
        .collect();
    assert_eq!(
        completion.first().map(|e| e.payload["evidence"].clone()),
        Some(serde_json::json!([{ "step": "c", "kind": "check" }])),
        "the completion row carries `c`'s evidence pointer"
    );
}
