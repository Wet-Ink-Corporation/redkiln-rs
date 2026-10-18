//! **`baseline` in a tool step's `range` is the run's fork point** (the operator's ruling A on the
//! S8J-C4 lane-proof defect D2, 2026-10-18). *(Added 2026-10-18, `s8j` build, package S8J-C4-FIX
//! D2.)*
//!
//! Twelve shipped activities declare `range = "baseline..HEAD"`, and `baseline` is no git ref. The
//! kernel rewrites a `baseline` endpoint of a tool step's resolved `inputs.range` to the fork point
//! the composition root binds (`ToolRouting::with_fork_point`, the commit the run tree was cut from,
//! as its `RK_BASE` records it, ADR-0180), before the port or the verb is called. With no fork
//! point bound, such a range is refused by name and the port is not called.

use std::sync::{Arc, Mutex};

use rk_kernel::catalog::InputValue;
use rk_kernel::predicate::Context;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::{StepVerdict, ToolRouting};
use rk_kernel::workflow::memo::ToolchainValues;
use rk_ports::harness::Harness;
use rk_ports::toolchain::{StepBound, Toolchain, VerbOutput};
use rk_types::error::StepError;

use crate::runtime::{
    CountingHarness, CountingToolchain, STAGE, counters, ports_with_toolchain, store,
};
use crate::tool_verb_routing::{registry, rt_routed, tool_step};

/// The fork point the tests bind: 40 hex, a commit id's shape.
const FORK: &str = "f00df00df00df00df00df00df00df00df00df00d";

/// A `toolchain` row taking a `range`, as `implement-ue`'s `asset_manifest` does.
const RANGE_ROW: &str = r#"manifest = { outputs = "SmokeResult", live = false, timeout = "build_timeout", inputs = ["range"], route = "toolchain", side_effecting = false }"#;

/// Records every range `change_summary` is handed, then answers as the counting fake does.
#[derive(Default)]
struct RangeRecorder {
    ranges: Mutex<Vec<String>>,
    rest: CountingToolchain,
}

impl RangeRecorder {
    fn ranges(&self) -> Vec<String> {
        self.ranges.lock().expect("not poisoned").clone()
    }
}

impl Toolchain for RangeRecorder {
    fn detect(&self) -> Result<bool, StepError> {
        self.rest.detect()
    }

    fn build(&self, bound: StepBound) -> Result<rk_types::verb_output::BuildReport, StepError> {
        self.rest.build(bound)
    }

    fn test(
        &self,
        selection: &rk_ports::toolchain::Selection,
        bound: StepBound,
    ) -> Result<rk_ports::toolchain::TestOutcome, StepError> {
        self.rest.test(selection, bound)
    }

    fn lint(&self, bound: StepBound) -> Result<(), StepError> {
        self.rest.lint(bound)
    }

    fn classify(&self, path: &str) -> Result<rk_ports::toolchain::Classification, StepError> {
        self.rest.classify(path)
    }

    fn change_summary(
        &self,
        range: &str,
        bound: StepBound,
    ) -> Result<rk_ports::toolchain::ChangeSummary, StepError> {
        self.ranges
            .lock()
            .map_err(|e| StepError::Unsupported {
                capability: format!("the range recorder's lock is poisoned: {e}"),
            })?
            .push(range.to_owned());
        self.rest.change_summary(range, bound)
    }

    fn run_verb(&self, call: &rk_ports::toolchain::VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.rest.run_verb(call)
    }
}

/// A runtime over a [`RangeRecorder`], with `routing` bound.
fn recording(run_id: &str, routing: ToolRouting) -> (Runtime, Arc<RangeRecorder>) {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness: Arc<dyn Harness> = Arc::new(CountingHarness::scripted(Vec::new()));
    let tc = Arc::new(RangeRecorder::default());
    let recorder = Arc::clone(&tc);
    let port: Arc<dyn Toolchain> = recorder;
    let rt = Runtime::new(ports_with_toolchain(harness, &db, port), run_id, STAGE)
        .with_tool_routing(routing);
    (rt, tc)
}

/// A tool step `summary` naming `verb`, with `range` declared.
fn ranged(verb: &str, range: &str) -> rk_kernel::catalog::Step {
    let mut step = tool_step("summary", verb);
    step.inputs
        .insert("range".to_owned(), InputValue::Text(range.to_owned()));
    step
}

fn routing() -> ToolRouting {
    ToolRouting::new(registry(RANGE_ROW, ""), ToolchainValues::default())
}

#[test]
fn a_baseline_range_reaches_change_summary_from_the_runs_fork_point() {
    let (rt, tc) = recording("baseline-fork", routing().with_fork_point(FORK));

    rt.execute_step(&ranged("change_summary", "baseline..HEAD"), &Context::new())
        .expect("a value");

    assert_eq!(tc.ranges(), vec![format!("{FORK}..HEAD")]);
}

#[test]
fn a_baseline_range_is_refused_by_name_when_no_fork_point_is_bound() {
    let (rt, tc) = recording("baseline-unbound", routing());

    let verdict = rt
        .execute_step(&ranged("change_summary", "baseline..HEAD"), &Context::new())
        .expect("a value");

    let StepVerdict::Failed(StepError::Unsupported { capability }) = verdict else {
        panic!("an unbound fork point must refuse the step by name: {verdict:?}");
    };
    assert!(
        capability.contains("`summary`")
            && capability.contains("baseline..HEAD")
            && capability.contains("fork point"),
        "the refusal names the step, the range and the missing fork point: {capability}"
    );
    assert_eq!(tc.ranges(), Vec::<String>::new(), "the port is not called");
}

#[test]
fn a_range_without_baseline_reaches_the_port_unchanged() {
    let (rt, tc) = recording("baseline-absent", routing());

    rt.execute_step(&ranged("change_summary", "abc123..HEAD"), &Context::new())
        .expect("a value");

    assert_eq!(tc.ranges(), vec!["abc123..HEAD".to_owned()]);
}

#[test]
fn a_toolchain_verbs_baseline_range_input_is_handed_the_fork_point() {
    let (rt, tc) = rt_routed(
        "baseline-verb",
        CountingToolchain::default(),
        routing().with_fork_point(FORK),
    );

    rt.execute_step(&ranged("manifest", "baseline..HEAD"), &Context::new())
        .expect("a value");

    let seen = tc.verb_inputs();
    assert_eq!(
        seen.first().and_then(|inputs| inputs.get("range")).cloned(),
        Some(serde_json::json!(format!("{FORK}..HEAD"))),
        "the verb is handed the rewritten range: {seen:?}"
    );
}
