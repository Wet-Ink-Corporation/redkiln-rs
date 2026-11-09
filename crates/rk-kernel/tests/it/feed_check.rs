//! **The feed check in the activity loop** (ADR-0205, proposed; D21-177; plan
//! `spikes/plans/s8-repro-aware-implement-2026-11-08.md` section 2, Decisions D5 and D6).
//!
//! A fresh dispatch whose value fails its step's selection feeds is a `StepError::Schema`: one
//! `step_failed {class: "schema"}` row, a re-dispatch under AC-031's limit carrying the refusal as
//! the input `refused`, and nothing committed for the refused answer. A value served from a
//! committed row is never re-checked (Invariant 5).

use std::sync::{Arc, Mutex};

use rk_kernel::catalog::StepKind;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::StepAt;
use rk_kernel::runtime::reference::{SelectionFeed, selection_feeds};
use rk_kernel::workflow::envelope::AttemptCap;
use rk_kernel::workflow::idem::{IterationPath, checked_key};
use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::{SchemaId, Typed};

use crate::reference_resolver::{step, with_inputs};
use crate::runtime::{
    AID, CAP, CountingHarness, ITEM, SCHEMA, STAGE, STEP, at, counters, journal, ok_typed, ports,
    spec, store,
};

/// A harness that records each dispatched spec's inputs, then answers from a [`CountingHarness`]
/// script.
struct RecordingHarness {
    inner: CountingHarness,
    inputs: Mutex<Vec<Vec<(String, serde_json::Value)>>>,
}

impl RecordingHarness {
    fn scripted(script: Vec<Result<Typed, StepError>>) -> Self {
        Self {
            inner: CountingHarness::scripted(script),
            inputs: Mutex::new(Vec::new()),
        }
    }

    fn inputs(&self) -> Vec<Vec<(String, serde_json::Value)>> {
        self.inputs.lock().expect("not poisoned").clone()
    }
}

impl Harness for RecordingHarness {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        self.inputs
            .lock()
            .expect("not poisoned")
            .push(spec.inputs.clone());
        self.inner.dispatch(spec, store)
    }
}

/// The feeds of a `red` agent step whose `tests_added` a `red-verify` test step selects with.
fn red_feeds() -> Vec<SelectionFeed> {
    let mut verify = with_inputs(
        step("red-verify", StepKind::Tool),
        &[("selection", "${red.outputs.tests_added}")],
    );
    verify.run = Some("toolchain:test".to_owned());
    selection_feeds(&[step("red", StepKind::Agent), verify])
        .remove("red")
        .expect("red has a feed")
}

const BAD: &str = "doctor test";
const GOOD: &str = "doctor::margin_absent_window_is_an_error";

fn answer(name: &str) -> Result<Typed, StepError> {
    ok_typed(serde_json::json!({"summary": "s", "tests_added": [name]}))
}

/// Run `red` once at the step grain over `script`, with its feeds bound.
fn run_red(
    script: Vec<Result<Typed, StepError>>,
    db: &Arc<Mutex<dyn EventStore>>,
) -> (
    rk_kernel::runtime::activity::ActivityReport,
    Arc<RecordingHarness>,
) {
    let h = Arc::new(RecordingHarness::scripted(script));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, db),
        "run-feed",
        STAGE,
    );
    let feeds = red_feeds();
    let path = IterationPath::root();
    let at_red = StepAt {
        step_id: "red",
        iteration_path: &path,
        family: None,
        feeds: &feeds,
    };
    let report = rt
        .run_activity_at(
            (ITEM, AID, STEP),
            &spec(),
            Some(at_red),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the run resolves");
    (report, h)
}

fn schema_failed_rows(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| {
            e.kind == EventKind::StepFailed
                && e.payload.get("class").and_then(serde_json::Value::as_str) == Some("schema")
        })
        .count()
}

fn committed_rows(events: &[Event]) -> usize {
    events
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .count()
}

/// **Test 7.** `[bad, good]`: one `step_failed {class: "schema"}` row, two dispatches, and the good
/// value is the one committed.
#[test]
fn a_bad_feed_value_is_a_schema_failure_and_is_redispatched() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let (report, h) = run_red(vec![answer(BAD), answer(GOOD)], &db);
    assert_eq!(h.inner.dispatches(), 2);
    assert_eq!(report.schema_failures(), 1);
    let events = journal(&db);
    assert_eq!(schema_failed_rows(&events), 1, "{events:#?}");
    assert_eq!(committed_rows(&events), 1, "{events:#?}");
    match report.outcome() {
        Some(Ok(typed)) => assert_eq!(
            typed.value.get("tests_added"),
            Some(&serde_json::json!([GOOD]))
        ),
        other => panic!("the good value is the outcome: {other:?}"),
    }
}

/// **Test 8.** `[bad, bad]`: `Schema { attempts: 2 }` naming the field, the token and the variant,
/// and no `activity_committed` row.
#[test]
fn a_bad_feed_value_twice_refuses_as_schema_and_commits_nothing() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let (report, h) = run_red(vec![answer(BAD), answer(BAD), answer(GOOD)], &db);
    assert_eq!(h.inner.dispatches(), 2, "AC-031: exactly one re-prompt");
    match report.outcome() {
        Some(Err(StepError::Schema {
            attempts,
            last_error,
        })) => {
            assert_eq!(*attempts, 2);
            assert!(
                last_error.contains("`.tests_added`")
                    && last_error.contains("red-verify")
                    && last_error.contains("BadToken")
                    && last_error.contains(BAD),
                "{last_error}"
            );
        }
        other => panic!("expected Err(Schema {{ attempts: 2 }}), got {other:?}"),
    }
    let events = journal(&db);
    assert_eq!(committed_rows(&events), 0, "{events:#?}");
    assert_eq!(schema_failed_rows(&events), 2, "{events:#?}");
}

/// **Test 9 (D6).** The re-prompt's inputs are the first dispatch's plus `refused`, naming the
/// token; the first dispatch carries no `refused`.
#[test]
fn the_reprompt_carries_the_refusal_as_refused() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let (_, h) = run_red(vec![answer(BAD), answer(GOOD)], &db);
    let inputs = h.inputs();
    assert_eq!(inputs.len(), 2, "{inputs:?}");
    let refused = |i: usize| {
        inputs
            .get(i)
            .and_then(|ins| ins.iter().find(|(k, _)| k == "refused"))
            .map(|(_, v)| v.clone())
    };
    assert_eq!(refused(0), None, "the first prompt carries no refusal");
    let text = refused(1).expect("the re-prompt carries `refused`");
    let text = text.as_str().expect("`refused` is the refusal's text");
    assert!(text.contains(BAD) && text.contains("BadToken"), "{text}");
    let without: Vec<_> = inputs
        .get(1)
        .expect("a second dispatch")
        .iter()
        .filter(|(k, _)| k != "refused")
        .cloned()
        .collect();
    assert_eq!(Some(&without), inputs.first(), "only `refused` is added");
}

/// **Test 9b (D6, every `Schema` re-prompt).** A schema-id mismatch's re-prompt carries `refused`
/// too.
#[test]
fn a_schema_mismatch_reprompt_carries_refused_too() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let wrong = Ok(Typed::new(SchemaId::new("Other/v1"), serde_json::json!({})));
    let (_, h) = run_red(vec![wrong, answer(GOOD)], &db);
    let inputs = h.inputs();
    let refused = inputs
        .get(1)
        .and_then(|ins| ins.iter().find(|(k, _)| k == "refused"))
        .and_then(|(_, v)| v.as_str().map(str::to_owned))
        .expect("the re-prompt carries `refused`");
    assert!(
        refused.contains("Other/v1") && refused.contains(SCHEMA),
        "{refused}"
    );
}

/// **Test 10 (Invariant 5).** A committed `red` row holding `"doctor test"` is served on re-entry:
/// nothing is dispatched and nothing is re-checked.
#[test]
fn a_committed_bad_value_is_served_not_rechecked() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let path = IterationPath::root();
    let feeds = red_feeds();
    {
        let rt = Runtime::new(
            ports(
                Arc::new(CountingHarness::scripted(Vec::new())) as Arc<dyn Harness>,
                &db,
            ),
            "run-before",
            STAGE,
        );
        let key = checked_key(ITEM, AID, 1).expect("a valid key");
        let at_red = StepAt {
            step_id: "red",
            iteration_path: &path,
            family: None,
            feeds: &[],
        };
        let typed = Typed::new(
            SchemaId::new(SCHEMA),
            serde_json::json!({"summary": "s", "tests_added": [BAD]}),
        );
        rt.commit_at(&key, Some(at_red), &typed, at(50))
            .expect("the old row commits");
    }
    assert!(!feeds.is_empty(), "PREMISE: the step has a feed to check");
    let (report, h) = run_red(vec![answer(GOOD)], &db);
    assert_eq!(
        h.inner.dispatches(),
        0,
        "a committed step is served, never redone"
    );
    assert_eq!(report.schema_failures(), 0);
    assert_eq!(schema_failed_rows(&journal(&db)), 0);
}
