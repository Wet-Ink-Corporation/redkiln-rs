//! **The 2026-11-08 failure, reproduced under the scripted harness, and its two hardenings**
//! (ADR-0205, proposed; D21-177; plan `spikes/plans/s8-repro-aware-implement-2026-11-08.md`
//! tests 18 and 19).
//!
//! The operator's run of `rk-fix-doctor-margin-2` went: `tdd-red-green-refactor@3`'s `red`
//! for_each:0 committed the FIX and named no test, for_each:1 named `"doctor test"`, `red-verify`
//! refused it as `Unsupported`, and the run exited 19. The scripts here model that implementer: it
//! "makes the change", and names tests in prose.

use rk_types::artifact::lane;
use rk_types::event_kind::EventKind;

use super::rig::{self, FIX_ITEM, Key, Response, STORY_ITEM};
use super::{
    FIXED_LIB, STORY_RED_NAME, STORY_RED_PATH, STORY_RED_TEST, fix_script, impl_report, key,
    story_script, t,
};

const LIB: &str = "crates/rk-lane-fixture/src/lib.rs";

/// What the real implementer card did at `red`: the fix, with a prose test name.
fn improvising_red() -> Response {
    Response::of(&lane::ImplReport {
        status: t("implemented"),
        files: vec![LIB.to_owned()],
        summary: t("made the change the step describes"),
        tests_added: vec!["word count test".to_owned()],
    })
    .writing(LIB, FIXED_LIB)
}

/// The `step_failed {class: "schema"}` rows `item` journalled.
fn schema_failures(root: &std::path::Path, item: &str) -> usize {
    crate::support::journal_rows(root, item)
        .iter()
        .filter(|e| {
            e.kind == EventKind::StepFailed
                && e.payload.get("class").and_then(serde_json::Value::as_str) == Some("schema")
        })
        .count()
}

/// **Test 18: the fix lane with an improvising implementer lands through the repro.** `tdd`'s
/// `red` (at `@3` and `@4`) is scripted as the operator's run had it. Under the plan it is never
/// selected for a bug with a placed repro: implement is `repro-green-refactor@1`, whose
/// `red-proof` runs the committed repro red (one failed, none passed) before any agent, and whose
/// `green` lands the fix with the criterion's named test. Verify confirms the repro passing and
/// opens no `verify.ac-discriminates` gate (survey 12). Red before the plan: the run exited 19 at
/// `red-verify` (`Unsupported`, the prose token), with `lib.rs` committed by a `red` step.
#[test]
fn fix_lane_with_an_improvising_implementer_lands_through_the_repro() {
    let repo = rig::fixture();
    let root = repo.path();
    let tdd: Vec<Key> = ["tdd-red-green-refactor@3", "tdd-red-green-refactor@4"]
        .iter()
        .map(|a| key(a, "red", 0))
        .collect();
    let mut script = fix_script();
    for k in &tdd {
        script.insert(k.clone(), improvising_red());
    }
    let harness = rig::ScriptedLane::new(root, FIX_ITEM, script);
    let walk = rig::walk_to_close(root, FIX_ITEM, &harness, 16);

    assert!(
        harness.unscripted().is_empty(),
        "unscripted dispatches: {:?}",
        harness.unscripted()
    );
    assert_eq!(
        harness.unconsumed(),
        tdd,
        "only the improvising `tdd` keys are left unconsumed"
    );
    assert!(
        harness
            .visited()
            .iter()
            .all(|(a, _, _)| !a.starts_with("tdd-red-green-refactor")),
        "no `tdd` key is visited: {:?}",
        harness.visited()
    );

    // `red-proof` ran the committed repro red, as `green` read it.
    let red = harness.input_of("repro-green-refactor@1", "green", "red");
    let [report] = red.as_slice() else {
        panic!("`green` was dispatched with one red-proof report, not {red:?}");
    };
    let count = |name: &str| report.get(name).and_then(serde_json::Value::as_u64);
    assert_eq!(
        (count("failed"), count("passed")),
        (Some(1), Some(0)),
        "implement's red-proof: {report}"
    );

    let confirmation = super::landed_artifact(root, FIX_ITEM, "ConfirmationResult");
    assert_eq!(
        confirmation
            .get("status")
            .and_then(serde_json::Value::as_str),
        Some("passing"),
        "verify's ConfirmationResult: {confirmation}"
    );
    assert!(
        walk.gates
            .iter()
            .all(|(kind, _)| kind != "verify.ac-discriminates"),
        "no AC-discrimination gate opens: {:?}",
        walk.gates
    );
    let run_ref = crate::lane_e2e::run_ref(FIX_ITEM);
    assert_eq!(
        rig::blob_at(root, &run_ref, LIB).as_deref(),
        Some(FIXED_LIB.as_bytes()),
        "the landed lib.rs is `green`'s fix"
    );
}

/// **Test 19: the story lane's `red` with a bad test name is re-prompted as `Schema`.** `tdd@4`'s
/// `red` first answers a prose name (writing its test), then, on the re-prompt, the bare name. The
/// lane lands; there is exactly one `step_failed {class: "schema"}` row, and the second dispatch's
/// inputs carry `refused` naming the prose token. Red before the plan: exit 19 at `red-verify`.
#[test]
fn story_lane_red_with_a_bad_test_name_is_reprompted_as_schema() {
    let repo = rig::fixture();
    let root = repo.path();
    let red = key("tdd-red-green-refactor@4", "red", 0);
    let mut script = story_script();
    let good = script
        .remove(&red)
        .expect("the story script keys tdd@4's red");
    script.insert(
        red,
        Response::of(&impl_report(&["story red test"], &[STORY_RED_PATH]))
            .writing(STORY_RED_PATH, STORY_RED_TEST)
            .then_on_retry(good),
    );
    let harness = rig::ScriptedLane::new(root, STORY_ITEM, script);
    rig::walk_to_close(root, STORY_ITEM, &harness, 16);

    assert!(
        harness.unscripted().is_empty() && harness.unconsumed().is_empty(),
        "unscripted {:?}, unconsumed {:?}",
        harness.unscripted(),
        harness.unconsumed()
    );
    assert_eq!(
        rig::story_stages(root, STORY_ITEM)
            .last()
            .map(String::as_str),
        Some("closed"),
        "the story lane lands"
    );
    assert_eq!(schema_failures(root, STORY_ITEM), 1, "one schema failure");
    let refused = harness.input_of("tdd-red-green-refactor@4", "red", "refused");
    let [text] = refused.as_slice() else {
        panic!("exactly the re-prompt carries `refused`: {refused:?}");
    };
    let text = text.as_str().unwrap_or_default();
    assert!(
        text.contains("story red test") && text.contains("BadToken"),
        "`refused` names the token and the refusal: {text}"
    );
    let tests_added = harness.input_of("tdd-red-green-refactor@4", "green", "red");
    assert!(
        tests_added
            .iter()
            .any(|red| red.to_string().contains(STORY_RED_NAME)),
        "`green` read the re-prompt's good answer: {tests_added:?}"
    );
}
