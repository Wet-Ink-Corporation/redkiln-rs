//! The closed `EventKind` table (ADR-0105 clause 7 as ADR-0116 supersedes it; design §5).

use rk_types::event_kind::{EventKind, Tier};

/// `ALL` really is all of them. The enum is closed, so a variant added without updating `ALL` is a
/// silent hole in every completeness check that follows.
#[test]
fn all_lists_every_variant_exactly_once() {
    assert_eq!(EventKind::ALL.len(), 28);
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
