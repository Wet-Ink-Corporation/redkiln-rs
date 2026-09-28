//! **The `Story:` axis** — Step 7h, `s7h` §7h.2, ADR-0155 and ADR-0159.
//!
//! `grep '^Story:'` — which is what Checkpoint 7h asserted as authored — passes on a value that
//! records nothing. These tests assert the value the producer actually emits **parses**, and that
//! the two ways of getting it wrong are caught.
//!
//! *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.5 and §8e.12; ADR-0183,
//! proposed.)* The grammar widened to `<project> "/" <slug> SP <stage>`, so `story_value` gains the
//! stage and each test's single-mark assertions become the **legacy-mark fixture** beside the
//! stage-bearing case: the one stage-less mark on the tree (`rk-run/rk-fix-doctor-test-count` at
//! `73c087a`) must keep reading as it always did. No test is deleted; the names are kept, although
//! ADR-0155's one-mark-per-item cardinality is what ADR-0183 supersedes in part.

use rk_git::reconcile::{self, CommitTrailers};
use rk_git::trailer::grammar::{
    self, MalformedReason, ParsedTrailer, StoryStage, Trailer, TrailerKey,
};

/// The one legacy stage-less mark on the tree, as `s8e` §8e.0 item 1 recorded it at `73c087a`.
const LEGACY: &str = "redkiln-rs/rk-fix-doctor-test-count";

/// One commit carrying `value` on the `Story:` axis, parsed the way `read_commits` parses a real one.
fn parsed(value: &str) -> Vec<ParsedTrailer> {
    let line = Trailer {
        key: TrailerKey::Story,
        value: value.to_owned(),
    }
    .to_line();
    // The producer's own rendering, in a body shaped like a real commit message: subject, blank
    // line, trailer block. `parse_body` anchors at column 0, so the shape matters.
    grammar::parse_body(&format!("story x entered the lane\n\n{line}"))
}

fn commit(value: &str) -> CommitTrailers {
    CommitTrailers {
        sha: format!("sha-{value}"),
        unix_time: 1_760_000_000,
        subject: "story x entered the lane".to_owned(),
        trailers: parsed(value),
    }
}

/// **The value the producer emits round-trips to the transition it intends.**
///
/// *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4.)* ~~`story_value("redkiln-rs", item)`
/// == `"redkiln-rs/rk-fix-doctor-test-count"`~~: the value now carries the stage, and parses as a
/// stage-bearing mark folded to that stage. The stage-less value is kept as the legacy fixture.
#[test]
fn adr0155_the_emitted_story_value_parses_as_a_qualified_story() {
    let value =
        rk_cli::verbs::run::story_value("redkiln-rs", "rk-fix-doctor-test-count", "analysis");
    assert_eq!(value, "redkiln-rs/rk-fix-doctor-test-count analysis");

    assert_eq!(
        parsed(&value),
        vec![ParsedTrailer::Story {
            project: "redkiln-rs".to_owned(),
            slug: "rk-fix-doctor-test-count".to_owned(),
            stage: Some(StoryStage::Stage("analysis".to_owned())),
        }],
        "the emitted value must parse as a QUALIFIED, stage-bearing story — a `BareStory` is \
         counted into `bare_story_ids` and is never a transition, and a slug that absorbed the \
         stage names no item record"
    );

    // And `reconcile` folds it as a committed story of that project, AT that stage.
    let (state, drift) = reconcile::reconcile(&[commit(&value)]);
    assert!(
        drift.is_empty(),
        "a single well-formed mark is not drift: {drift:?}"
    );
    let project = state.projects.get("redkiln-rs");
    assert!(
        project.is_some_and(|p| p.stories_committed.contains("rk-fix-doctor-test-count")),
        "the mark must reach `stories_committed`, which is what the producer's own idempotence \
         check reads"
    );
    assert_eq!(
        project
            .and_then(|p| p.stages.get("rk-fix-doctor-test-count"))
            .map(|m| m.stage.clone()),
        Some(StoryStage::Stage("analysis".to_owned())),
        "the fold must carry the mark's stage"
    );
    assert!(
        state.bare_story_ids.is_empty(),
        "nothing may land in `bare_story_ids`"
    );

    // ---- THE LEGACY-MARK FIXTURE: the old single-mark assertions, over the stage-less value.
    assert_eq!(
        parsed(LEGACY),
        vec![ParsedTrailer::Story {
            project: "redkiln-rs".to_owned(),
            slug: "rk-fix-doctor-test-count".to_owned(),
            stage: None,
        }],
        "the legacy stage-less mark must still parse as a qualified story, with no stage"
    );
    let (legacy_state, legacy_drift) = reconcile::reconcile(&[commit(LEGACY)]);
    assert!(legacy_drift.is_empty(), "{legacy_drift:?}");
    let legacy_project = legacy_state.projects.get("redkiln-rs");
    assert!(
        legacy_project.is_some_and(|p| p.stories_committed.contains("rk-fix-doctor-test-count"))
    );
    assert!(
        legacy_project.is_some_and(|p| p.stages.is_empty()),
        "a stage-less mark folds to NO stage, so the seed falls back to the item record"
    );
}

/// **ADR-0159's failure mode: an empty project still COMMITS and records nothing.**
///
/// This is why the producer refuses instead of emitting. The trailer is well-formed to git and to a
/// `grep`; it is inert to `reconcile`.
///
/// *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4.)* With the stage, an empty project's
/// value `/<item> <stage>` is not a bare id any more: the widened grammar has no bare-id-with-stage
/// form, so it is **malformed drift** (AC-183) — surfaced, where the stage-less `/<item>` was
/// silent. Still no transition, and still exactly what a `grep '^Story:'` cannot tell apart. The
/// stage-less arm is kept as the legacy fixture.
#[test]
fn adr0159_an_empty_project_degrades_to_a_bare_story_and_records_no_transition() {
    let value = rk_cli::verbs::run::story_value("", "rk-fix-doctor-test-count", "analysis");
    assert_eq!(value, "/rk-fix-doctor-test-count analysis");

    assert!(
        matches!(
            parsed(&value).as_slice(),
            [ParsedTrailer::Malformed {
                why: MalformedReason::MalformedStoryValue,
                ..
            }]
        ),
        "an empty project makes `split_qualified` return `None`; with a stage the value is drift: \
         {:?}",
        parsed(&value)
    );
    let (state, drift) = reconcile::reconcile(&[commit(&value)]);
    assert!(
        state.projects.is_empty(),
        "an unqualified mark belongs to no project, so it can never be a transition"
    );
    assert!(state.bare_story_ids.is_empty());
    assert_eq!(drift.len(), 1, "it is surfaced as drift now: {drift:?}");

    // ---- THE LEGACY-MARK FIXTURE: the stage-less empty-project value is still a silent bare id.
    let legacy = "/rk-fix-doctor-test-count";
    assert_eq!(
        parsed(legacy),
        vec![ParsedTrailer::BareStory {
            id: legacy.to_owned(),
        }],
        "an empty project makes `split_qualified` return `None`, so the stage-less mark is BARE"
    );
    let (legacy_state, legacy_drift) = reconcile::reconcile(&[commit(legacy)]);
    assert!(legacy_state.projects.is_empty());
    assert_eq!(legacy_state.bare_story_ids.len(), 1);
    assert!(
        legacy_drift.is_empty(),
        "and it is not even reported as drift: {legacy_drift:?}"
    );

    // 🛑 THE POINT: the guard Checkpoint 7h was authored with cannot tell these apart.
    let good = Trailer {
        key: TrailerKey::Story,
        value: rk_cli::verbs::run::story_value(
            "redkiln-rs",
            "rk-fix-doctor-test-count",
            "analysis",
        ),
    }
    .to_line();
    let inert = Trailer {
        key: TrailerKey::Story,
        value,
    }
    .to_line();
    assert!(
        good.starts_with("Story:") && inert.starts_with("Story:"),
        "both satisfy `grep '^Story:'` — which is why the checkpoint now asserts the PARSE"
    );
}

/// **ADR-0155's cardinality: a second identical mark is drift, so the producer must be idempotent.**
///
/// *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4.)* Identity is now `(project, slug,
/// stage)`: an identical stage-bearing mark is still the duplicate, and a mark for ANOTHER stage of
/// the same item is not — that is the widening, and why ADR-0183 supersedes ADR-0155's
/// one-mark-per-item in part. Two identical legacy marks are kept as the legacy fixture.
#[test]
fn adr0155_a_second_identical_story_mark_is_drift_not_a_transition() {
    let value =
        rk_cli::verbs::run::story_value("redkiln-rs", "rk-fix-doctor-test-count", "analysis");
    let (state, drift) = reconcile::reconcile(&[commit(&value), commit(&value)]);

    assert_eq!(
        drift.len(),
        1,
        "the second identical mark must be reported as drift: {drift:?}"
    );
    assert!(
        format!("{drift:?}").contains("DuplicateStoryTrailer"),
        "and specifically as a duplicate: {drift:?}"
    );
    // The state is unchanged by the second mark — which is what makes minting it pointless as well
    // as harmful.
    assert_eq!(
        state
            .projects
            .get("redkiln-rs")
            .map(|p| p.stories_committed.len()),
        Some(1)
    );

    // A DIFFERENT stage of the same item is a transition, not a duplicate.
    let next =
        rk_cli::verbs::run::story_value("redkiln-rs", "rk-fix-doctor-test-count", "specification");
    let (moved, moved_drift) = reconcile::reconcile(&[commit(&value), commit(&next)]);
    assert!(moved_drift.is_empty(), "{moved_drift:?}");
    assert_eq!(
        moved
            .projects
            .get("redkiln-rs")
            .and_then(|p| p.stages.get("rk-fix-doctor-test-count"))
            .map(|m| m.stage.clone()),
        Some(StoryStage::Stage("specification".to_owned()))
    );

    // ---- THE LEGACY-MARK FIXTURE: two identical stage-less marks are the duplicate they were.
    let (_, legacy_drift) = reconcile::reconcile(&[commit(LEGACY), commit(LEGACY)]);
    assert_eq!(legacy_drift.len(), 1, "{legacy_drift:?}");
}
