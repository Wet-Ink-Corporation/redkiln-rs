//! The closed `EventKind` table (ADR-0105 clause 7 as ADR-0116 supersedes it, and as ADR-0120,
//! ADR-0123 and ADR-0124 extend it; design §5).

use rk_types::event_kind::{EventKind, Tier};

/// `ALL` really is all of them. The enum is closed, so a variant added without updating `ALL` is a
/// silent hole in every completeness check that follows.
#[test]
fn all_lists_every_variant_exactly_once() {
    // 29 until ADR-0123 (2026-09-17) minted `gate_answer_spent` and `gate_resumed`; 31 until
    // ADR-0124 (the same day) minted the review and verify gates' five.
    assert_eq!(EventKind::ALL.len(), 36);
    let mut names: Vec<&str> = EventKind::ALL.iter().map(|k| k.wire_name()).collect();
    let before = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), before, "two kinds share a wire name");
}

/// **ADR-0116's addition.** Without the write side, `doctor` cannot tell a produced-and-never-read
/// artifact from one that was never produced — which is the whole reason the record exists.
#[test]
fn artifact_write_exists_and_is_tier_two() {
    assert_eq!(EventKind::ArtifactWrite.wire_name(), "artifact.write");
    assert_eq!(EventKind::ArtifactWrite.tier(), Tier::Two);
    assert_eq!(
        EventKind::from_wire("artifact.write"),
        Some(EventKind::ArtifactWrite)
    );
}

/// **ADR-0120's addition.** AC-064's THEN names `dag_amended` and nothing minted it, so the
/// criterion was unimplementable and Step 7b carried it ARMED. Tier 2 for `dag_assembled`'s reason:
/// a delta may not outrank the assembly it amends, and the deviation record is already in the
/// replay input set, so the amended DAG is reproduced by replay rather than read back from here.
#[test]
fn dag_amended_exists_and_is_tier_two() {
    assert_eq!(EventKind::DagAmended.wire_name(), "dag_amended");
    assert_eq!(EventKind::DagAmended.tier(), Tier::Two);
    assert_eq!(
        EventKind::from_wire("dag_amended"),
        Some(EventKind::DagAmended)
    );
    // The delta and the assembly are two kinds, not one — a second `dag_assembled` is forbidden as
    // the re-run's record (ADR-0120 decision 5's forbids).
    assert_ne!(EventKind::DagAmended, EventKind::DagAssembled);
    assert_eq!(EventKind::DagAmended.trailer_axis(), None);
}

/// **ADR-0123's two additions.** AC-246's THEN says a second answer on a spent token *"records a
/// distinct event"* and nothing minted one; AC-279 and AC-267 assert on a `gate_resumed` event
/// that no table defined. Both tier 2 by the deleted-database test: the spent refusal is derived
/// from the tier-1 `gate_answered` rows and this row only records that it happened; the resume row
/// is telemetry about a dispatch that already happened.
#[test]
fn gate_answer_spent_and_gate_resumed_exist_and_are_tier_two() {
    assert_eq!(EventKind::GateAnswerSpent.wire_name(), "gate_answer_spent");
    assert_eq!(EventKind::GateAnswerSpent.tier(), Tier::Two);
    assert_eq!(
        EventKind::from_wire("gate_answer_spent"),
        Some(EventKind::GateAnswerSpent)
    );
    assert_eq!(EventKind::GateResumed.wire_name(), "gate_resumed");
    assert_eq!(EventKind::GateResumed.tier(), Tier::Two);
    assert_eq!(
        EventKind::from_wire("gate_resumed"),
        Some(EventKind::GateResumed)
    );
    // Spent and void are two facts with two fold effects (ADR-0123's forbids): a void un-answers
    // the gate, a spent refusal leaves the first answer standing.
    assert_ne!(EventKind::GateAnswerSpent, EventKind::GateAnswerVoid);
    // Neither rides a trailer; no fourth axis was minted.
    assert_eq!(EventKind::GateAnswerSpent.trailer_axis(), None);
    assert_eq!(EventKind::GateResumed.trailer_axis(), None);
}

/// **ADR-0124's five additions** — the review and verify gates' rows, each named by a criterion or a
/// success measure and minted by nothing until now. All tier 2: a block is DERIVED from the recorded
/// payload and re-execution (AC-313 recomputes it), a quarantine's effect rides the finding's
/// persisted state, and `selection_ineffective` is a reclassification the verify report itself
/// carries — losing any of these rows loses a count, never a decision (ADR-0109 decision 2).
#[test]
fn the_review_and_verify_gates_five_kinds_exist_and_are_tier_two() {
    let five = [
        (EventKind::ReviewBlock, "review_block"),
        (EventKind::QuarantineGranted, "quarantine_granted"),
        (EventKind::QuarantineRemoved, "quarantine_removed"),
        (EventKind::QuarantineExpired, "quarantine_expired"),
        (EventKind::SelectionIneffective, "selection_ineffective"),
    ];
    for (kind, wire) in five {
        assert_eq!(kind.wire_name(), wire);
        assert_eq!(kind.tier(), Tier::Two, "{wire} is tier 2");
        assert_eq!(EventKind::from_wire(wire), Some(kind));
        assert_eq!(kind.trailer_axis(), None, "{wire} rides no trailer");
    }
    // a block is not drift and not sink intake — three kinds, three counts (AC-307, AC-310)
    assert_ne!(EventKind::ReviewBlock, EventKind::ReviewClassDrift);
    assert_ne!(EventKind::ReviewBlock, EventKind::AdvisoryCreated);
}

/// The two dotted names are dotted. A blanket `rename_all = "snake_case"` would quietly turn them
/// into `artifact_read`/`artifact_write` and every consumer reading design §5's table would miss.
#[test]
fn the_dotted_names_are_not_normalised_to_snake_case() {
    assert_eq!(EventKind::ArtifactRead.wire_name(), "artifact.read");
    assert_eq!(EventKind::ArtifactWrite.wire_name(), "artifact.write");
}

/// Exactly five kinds are tier 1, and they are the five design §5's table marks as git-first:
/// stage transitions, gate opened/answered/expired, and slice seals.
#[test]
fn exactly_five_kinds_are_tier_one() {
    let t1: Vec<&str> = EventKind::ALL
        .iter()
        .filter(|k| k.tier() == Tier::One)
        .map(|k| k.wire_name())
        .collect();
    assert_eq!(
        t1,
        [
            "stage_transition",
            "gate_opened",
            "gate_answered",
            "gate_expired",
            "slice_sealed"
        ],
        "the tier-1 set is not the one the deleted-database test selects"
    );
}

/// The memo is **tier 2** — ADR-0097 decision 1, and the amendment AC-160's fixture depends on:
/// *"the step memo is tier 2 — rebuildable by re-execution, never by rebuild"*.
#[test]
fn the_memo_kinds_are_tier_two() {
    for k in [
        EventKind::MemoHit,
        EventKind::MemoMiss,
        EventKind::MemoEvicted,
    ] {
        assert_eq!(k.tier(), Tier::Two, "{} is not tier 2", k.wire_name());
    }
}

/// `rewrite` is tier-2-derived and sits **outside** the AC-184 rebuild differential (ADR-0105
/// clause 9). Marking it tier 1 would put it inside, and the differential would then fail on any
/// repository where someone had rebased.
#[test]
fn rewrite_is_not_tier_one() {
    assert_eq!(EventKind::Rewrite.tier(), Tier::Two);
}

/// `slice_sealed` is ONE kind carrying a verdict, not two. Design §5 writes the row as
/// "`slice_sealed` / verdict", which reads like two names for one thing — and it is one.
#[test]
fn slice_sealed_is_a_single_kind() {
    assert_eq!(
        EventKind::ALL
            .iter()
            .filter(|k| k.wire_name().contains("slice"))
            .count(),
        1
    );
}

/// Every tier-1 kind names the trailer axis it rides on, and no tier-2 kind does — tier 1 is
/// exactly the set that is written to git first.
#[test]
fn the_trailer_axes_belong_to_the_tier_one_kinds() {
    for k in EventKind::ALL {
        match k.tier() {
            Tier::One => assert!(
                k.trailer_axis().is_some() || k == EventKind::GateExpired,
                "{} is tier 1 but names no trailer axis",
                k.wire_name()
            ),
            Tier::Two => assert!(
                k.trailer_axis().is_none(),
                "{} is tier 2 but claims a trailer axis",
                k.wire_name()
            ),
        }
    }
}

/// **`ReplayDivergence` is not a kind, by name.** Design §5's tier column for that row reads `—`
/// and calls it "a halt, not a durable record". The exclusion is asserted explicitly, because an
/// unexplained absence is indistinguishable from an omission.
#[test]
fn replay_divergence_is_deliberately_not_an_event_kind() {
    assert!(
        EventKind::from_wire("replay_divergence").is_none(),
        "ReplayDivergence became a durable event kind; design §5 marks it a halt, and recording it \
         would let a run continue past the divergence"
    );
    // It exists as its own type, so the exclusion is a decision rather than an oversight.
    let _ = rk_types::event_kind::ReplayDivergence {
        activity_id: "a".into(),
        recorded: "x".into(),
        recomputed: "y".into(),
    };
}

/// Round-trip: every kind's wire name resolves back to it.
#[test]
fn every_wire_name_round_trips() {
    for k in EventKind::ALL {
        assert_eq!(EventKind::from_wire(k.wire_name()), Some(k));
    }
    assert_eq!(EventKind::from_wire("not_a_kind"), None);
}
