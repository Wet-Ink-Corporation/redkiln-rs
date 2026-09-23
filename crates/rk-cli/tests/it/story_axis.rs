//! **The `Story:` axis** — Step 7h, `s7h` §7h.2, ADR-0155 and ADR-0159.
//!
//! `grep '^Story:'` — which is what Checkpoint 7h asserted as authored — passes on a value that
//! records nothing. These tests assert the value the producer actually emits **parses**, and that
//! the two ways of getting it wrong are caught.

use rk_git::reconcile::{self, CommitTrailers};
use rk_git::trailer::grammar::{self, ParsedTrailer, Trailer, TrailerKey};

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
#[test]
fn adr0155_the_emitted_story_value_parses_as_a_qualified_story() {
    let value = rk_cli::verbs::run::story_value("redkiln-rs", "rk-fix-doctor-test-count");
    assert_eq!(value, "redkiln-rs/rk-fix-doctor-test-count");

    assert_eq!(
        parsed(&value),
        vec![ParsedTrailer::Story {
            project: "redkiln-rs".to_owned(),
            slug: "rk-fix-doctor-test-count".to_owned(),
        }],
        "the emitted value must parse as a QUALIFIED story — a `BareStory` is counted into \
         `bare_story_ids` and is never a transition"
    );

    // And `reconcile` folds it as a committed story of that project.
    let (state, drift) = reconcile::reconcile(&[commit(&value)]);
    assert!(
        drift.is_empty(),
        "a single well-formed mark is not drift: {drift:?}"
    );
    assert!(
        state
            .projects
            .get("redkiln-rs")
            .is_some_and(|p| p.stories_committed.contains("rk-fix-doctor-test-count")),
        "the mark must reach `stories_committed`, which is what the producer's own idempotence \
         check reads"
    );
    assert!(
        state.bare_story_ids.is_empty(),
        "nothing may land in `bare_story_ids`"
    );
}

/// **ADR-0159's failure mode: an empty project still COMMITS and records nothing.**
///
/// This is why the producer refuses instead of emitting. The trailer is well-formed to git and to a
/// `grep`; it is inert to `reconcile`.
#[test]
fn adr0159_an_empty_project_degrades_to_a_bare_story_and_records_no_transition() {
    let value = rk_cli::verbs::run::story_value("", "rk-fix-doctor-test-count");
    assert_eq!(value, "/rk-fix-doctor-test-count");

    assert_eq!(
        parsed(&value),
        vec![ParsedTrailer::BareStory {
            id: "/rk-fix-doctor-test-count".to_owned(),
        }],
        "an empty project makes `split_qualified` return `None`, so the mark is BARE"
    );

    let (state, drift) = reconcile::reconcile(&[commit(&value)]);
    assert!(
        state.projects.is_empty(),
        "a bare story belongs to no project, so it can never be a transition"
    );
    assert_eq!(state.bare_story_ids.len(), 1);
    assert!(
        drift.is_empty(),
        "and it is not even reported as drift: {drift:?}"
    );

    // 🛑 THE POINT: the guard Checkpoint 7h was authored with cannot tell these apart.
    let good = Trailer {
        key: TrailerKey::Story,
        value: rk_cli::verbs::run::story_value("redkiln-rs", "rk-fix-doctor-test-count"),
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
#[test]
fn adr0155_a_second_identical_story_mark_is_drift_not_a_transition() {
    let value = rk_cli::verbs::run::story_value("redkiln-rs", "rk-fix-doctor-test-count");
    let (state, drift) = reconcile::reconcile(&[commit(&value), commit(&value)]);

    assert_eq!(
        drift.len(),
        1,
        "the second mark for one slug must be reported as drift: {drift:?}"
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
}
