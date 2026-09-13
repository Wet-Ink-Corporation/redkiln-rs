//! Design §5.4's trailer grammar, row by row (ADR-0105 clause 3; ADR-0109 decision 3; AC-165).

use rk_git::reconcile::{CommitTrailers, reconcile};
use rk_git::trailer::grammar::{MalformedReason, ParsedTrailer, TrailerKey, Verdict, parse_body};
use rk_types::drift::Drift;

fn commit(sha: &str, body: &str) -> CommitTrailers {
    CommitTrailers {
        sha: sha.into(),
        unix_time: 0,
        subject: "s".into(),
        trailers: parse_body(body),
    }
}

/// **Column 0, case-sensitive.** An indented or lower-cased mention inside prose is prose.
/// `strip_prefix`, never `trim_start` — the rule the corpus version carries and the one a
/// well-meaning `trim()` would silently break.
/// Two different rules, and they are deliberately not the same rule:
///
/// * **Indented** → not a trailer at all. Column-0 anchoring means an indented mention inside prose
///   never registers, in any form.
/// * **Lower-cased key** → a trailer, but not the `Story` **axis**. §5.4 is case-sensitive on the
///   key, so `story:` is simply a different key, and row 4 then applies: unknown keys are *ignored
///   and counted*, never folded into an axis and never reported as drift.
///
/// A first version of this test expected `story:` to vanish entirely. That was the test being
/// wrong, not the parser: silently dropping it would lose the count row 4 asks for.
#[test]
fn only_column_zero_and_exact_case_registers_as_a_trailer() {
    let t = parse_body("  Story: demo/s-1\n\tStory: demo/s-2\nstory: demo/s-3\nStory: demo/s-4\n");
    assert_eq!(
        t,
        vec![
            ParsedTrailer::UnknownKey {
                key: "story".into()
            },
            ParsedTrailer::Story {
                project: "demo".into(),
                slug: "s-4".into()
            }
        ],
        "indented mentions must not register at all, and a lower-cased key must not register as \
         the Story axis"
    );

    // The axis-level statement, said directly: only the column-0, exactly-cased line is a `Story`.
    let stories: Vec<_> = t
        .iter()
        .filter(|p| matches!(p, ParsedTrailer::Story { .. }))
        .collect();
    assert_eq!(stories.len(), 1);

    // And the lower-cased one is counted rather than dropped or reported as drift.
    let (state, drift) = reconcile(&[commit("aaa", "story: demo/s-3\n")]);
    assert!(drift.is_empty(), "a lower-cased key was reported as drift");
    assert_eq!(state.unknown_keys.0["story"], 1);
}

/// Row 1: a bare id carries no project, so it is kept apart rather than folded into one.
#[test]
fn a_bare_story_id_is_kept_separate_from_projects() {
    let (state, _) = reconcile(&[commit("aaa", "Story: RK-S0272\nStory: demo/s-1\n")]);
    assert!(state.bare_story_ids.contains("RK-S0272"));
    assert!(state.projects.contains_key("demo"));
    assert_eq!(
        state.projects.len(),
        1,
        "a bare id invented a project: {:?}",
        state.projects.keys().collect::<Vec<_>>()
    );
}

/// Row 1: a second identical `Story:` for one slug is drift, counted, and **not** a transition.
#[test]
fn a_duplicate_story_trailer_is_drift_and_not_a_transition() {
    let (state, drift) = reconcile(&[
        commit("aaa", "Story: demo/s-1\n"),
        commit("bbb", "Story: demo/s-1\n"),
    ]);
    assert_eq!(state.projects["demo"].stories_committed.len(), 1);
    assert_eq!(drift.len(), 1);
    assert!(matches!(drift[0], Drift::DuplicateStoryTrailer { .. }));
}

/// Row 2: exactly two whitespace-separated tokens; the vocabulary is closed; latest wins over a
/// fold that is oldest-first. This is the axis F12 exists to justify — 15 of 74 slices were ever
/// rejected, and `Story:` alone cannot see it.
#[test]
fn slice_verdict_latest_wins_and_history_is_retained() {
    let (state, _) = reconcile(&[
        commit("aaa", "Slice-Verdict: demo/sl-1 changes-requested\n"),
        commit("bbb", "Slice-Verdict: demo/sl-1 approved\n"),
    ]);
    let slice = &state.projects["demo"].slices["sl-1"];
    assert_eq!(slice.latest_verdict, Verdict::Approved);
    assert!(slice.is_done());
    assert!(slice.sealed_more_than_once);
    assert!(
        slice.ever_rejected(),
        "the rejection history was collapsed — a slice rejected then re-approved is a DIFFERENT \
         situation from one only ever approved, and that difference is what resume needs"
    );
}

/// Row 2: an unqualified value is drift, and a third token is drift under the declared grammar
/// even though the spike's parser silently ignored it.
#[test]
fn an_unqualified_or_over_long_slice_verdict_is_drift() {
    for body in [
        "Slice-Verdict: sl-1 approved\n",
        "Slice-Verdict: demo/sl-1 approved extra\n",
    ] {
        let t = parse_body(body);
        assert!(
            matches!(
                t[0],
                ParsedTrailer::Malformed {
                    why: MalformedReason::UnqualifiedSliceVerdict,
                    ..
                }
            ),
            "{body:?} did not register as drift: {t:?}"
        );
    }
}

/// Row 2: the verdict vocabulary is closed, compared case-insensitively and stored lower-case.
#[test]
fn the_verdict_vocabulary_is_closed_and_case_insensitive() {
    let t = parse_body("Slice-Verdict: demo/sl-1 APPROVED\n");
    assert!(matches!(
        t[0],
        ParsedTrailer::SliceVerdict {
            verdict: Verdict::Approved,
            ..
        }
    ));
    assert_eq!(Verdict::Approved.as_str(), "approved");

    let bad = parse_body("Slice-Verdict: demo/sl-1 rubber-stamped\n");
    assert!(matches!(
        bad[0],
        ParsedTrailer::Malformed {
            why: MalformedReason::UnknownVerdict,
            ..
        }
    ));
}

/// Row 3: the third axis. A well-formed `Gate:` is the `(token, tree_hash, deadline)` triple.
#[test]
fn a_gate_trailer_carries_the_triple() {
    let sha = "0123456789abcdef0123456789abcdef01234567";
    let t = parse_body(&format!("Gate: tok-1 {sha} 2026-09-13T00:00:00Z\n"));
    match &t[0] {
        ParsedTrailer::Gate {
            token,
            tree_hash,
            deadline,
        } => {
            assert_eq!(token, "tok-1");
            assert_eq!(tree_hash, sha);
            assert_eq!(deadline, "2026-09-13T00:00:00Z");
        }
        other => panic!("not parsed as a Gate triple: {other:?}"),
    }
}

/// Row 3: **a `Gate:` value that is not the triple is legacy drift** — counted, surfaced with its
/// offending text, never a transition. F12's corpus carries such lines, which is why this row
/// exists at all.
#[test]
fn a_legacy_gate_value_is_drift_and_never_a_transition() {
    let (state, drift) = reconcile(&[commit("aaa", "Gate: open\n")]);
    assert!(
        state.gates.is_empty(),
        "a legacy Gate: value was coerced into a gate record"
    );
    assert_eq!(drift.len(), 1);
    match &drift[0] {
        Drift::LegacyGateTrailer { offending, .. } => assert_eq!(offending.as_str(), "open"),
        other => panic!("wrong drift: {other:?}"),
    }
}

/// Row 4: **unknown keys are ignored and COUNTED — not drift.** `Fix:`, `Slice-Repair:` and
/// `Baseline-Repair:` are the old redkiln's keys, read by `redkiln-migrate` only and never emitted
/// by `rk-git`. Folding them into `Drift` would make a migration artefact look like a defect.
#[test]
fn unknown_keys_are_counted_not_drift() {
    let (state, drift) = reconcile(&[commit(
        "aaa",
        "Fix: RK-B0013\nSlice-Repair: demo/sl-1\nBaseline-Repair: x\nCo-authored-by: someone\n",
    )]);
    assert!(
        drift.is_empty(),
        "an unknown trailer key was reported as drift: {drift:?}"
    );
    assert_eq!(state.unknown_keys.total(), 4);
    assert_eq!(state.unknown_keys.0["Fix"], 1);
    assert_eq!(state.unknown_keys.0["Slice-Repair"], 1);
}

/// AC-165's core: a malformed trailer is surfaced **with its offending text** and yields zero state
/// transitions. Asserted with a count, not just "some drift exists", so a fixture that fired the
/// wrong rule cannot pass.
#[test]
fn a_malformed_trailer_is_one_drift_and_zero_transitions() {
    let (state, drift) = reconcile(&[commit("aaa", "Slice-Verdict: nonsense\n")]);
    assert_eq!(drift.len(), 1, "expected exactly one drift: {drift:?}");
    assert!(matches!(drift[0], Drift::UnqualifiedSliceVerdict { .. }));
    assert!(
        state.projects.is_empty(),
        "a malformed trailer created state"
    );
}

/// The five design drift values must not collapse into one another. `Template` reported as
/// `structured_drift` would make AC-128 — *"no template drift is reported as human drift"* — false
/// while every test above still passed.
#[test]
fn the_drift_kinds_are_all_distinct() {
    use rk_types::text::Text;
    let d = [
        Drift::Template {
            artifact: "a".into(),
            from: "1".into(),
            to: "2".into(),
        },
        Drift::UnparsableTrailer {
            commit: "a".into(),
            key: "Gate".into(),
            offending: Text::new("x"),
        },
        Drift::LegacyGateTrailer {
            commit: "a".into(),
            offending: Text::new("x"),
        },
        Drift::DuplicateStoryTrailer {
            project: "p".into(),
            slug: "s".into(),
            commit: "a".into(),
        },
        Drift::UnqualifiedSliceVerdict {
            commit: "a".into(),
            offending: Text::new("x"),
        },
        Drift::SliceLeftRejected {
            project: "p".into(),
            slice: "s".into(),
            latest: "changes-requested".into(),
        },
    ];
    let kinds: Vec<&str> = d.iter().map(rk_types::drift::Drift::kind).collect();
    let mut uniq = kinds.clone();
    uniq.sort_unstable();
    uniq.dedup();
    assert_eq!(
        uniq.len(),
        kinds.len(),
        "two drift kinds collapsed: {kinds:?}"
    );
    assert_ne!(
        Drift::Template {
            artifact: "a".into(),
            from: "1".into(),
            to: "2".into()
        }
        .kind(),
        "structured_drift",
        "template drift is being reported as human drift, which AC-128 forbids by name"
    );
}

/// A slice left at `changes-requested` is drift: unfinished work a naive resume would skip. This is
/// the finding the third axis exists for.
#[test]
fn a_slice_left_rejected_is_reported() {
    let (_, drift) = reconcile(&[commit(
        "aaa",
        "Slice-Verdict: demo/sl-1 changes-requested\n",
    )]);
    assert!(
        drift
            .iter()
            .any(|d| matches!(d, Drift::SliceLeftRejected { .. })),
        "a slice left rejected was not reported: {drift:?}"
    );
}

/// `rk-git` never emits the migration-only keys.
#[test]
fn rk_git_emits_no_legacy_keys() {
    for k in ["Fix", "Slice-Repair", "Baseline-Repair"] {
        assert!(
            matches!(TrailerKey::parse(k), TrailerKey::Other(_)),
            "{k} is being treated as a first-class axis; design §5.4 row 4 says redkiln-migrate \
             reads them and rk-git does not emit them"
        );
    }
}
