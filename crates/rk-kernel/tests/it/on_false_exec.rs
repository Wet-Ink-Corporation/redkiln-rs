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
use rk_kernel::runtime::step::ToolRouting;
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
}

impl LoopHarness {
    pub const fn scripted(script: Vec<Json>) -> Self {
        Self {
            script: Mutex::new(script),
            seen: Mutex::new(Vec::new()),
        }
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
        let next = {
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
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(LoopHarness::scripted(script));
        let tests = Arc::new(ScriptedTests::scripted(tests));
        let rt = Runtime::new(
            ports_with_toolchain(
                Arc::clone(&harness) as Arc<dyn Harness>,
                &db,
                Arc::clone(&tests) as Arc<dyn Toolchain>,
            ),
            "run-loop",
            STAGE,
        )
        .with_tool_routing(ToolRouting::new(
            crate::tool_verb_routing::registry("", ""),
            ToolchainValues::default(),
        ));
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
        let state = rk_kernel::workflow::fsm::fold(ITEM, STAGE, &journal(&self.db))
            .unwrap_or_else(|_| empty_state());
        let sp = spec();
        self.rt
            .run_activity_steps(
                activation(&sp),
                Some(steps),
                &rk_kernel::predicate::Context::new(),
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
