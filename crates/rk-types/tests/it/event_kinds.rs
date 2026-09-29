//! The closed `EventKind` table (ADR-0105 clause 7 as ADR-0116 supersedes it, and as ADR-0120,
//! ADR-0123, ADR-0124, ADR-0125 and ADR-0126 extend it; design §5).

use rk_types::event_kind::{EventKind, Tier};

/// `ALL` really is all of them. The enum is closed, so a variant added without updating `ALL` is a
/// silent hole in every completeness check that follows.
#[test]
fn all_lists_every_variant_exactly_once() {
    // 29 until ADR-0123 (2026-09-17) minted `gate_answer_spent` and `gate_resumed`; 31 until
    // ADR-0124 (the same day) minted the review and verify gates' five; 36 until ADR-0125 (the same
    // day again) minted `stage_exit_refused`; 37 until ADR-0126 (the same day, Step 7c Slice 1)
    // minted `dispatch_completed`; 38 until `s8e`'s three-kinds record (2026-09-28, package S8E-P5,
    // proposed) minted `worktree_assigned`, `stage_empty` and `git_op`; 41 until `s8g`'s verify record
    // (ADR-0191 decision 8, proposed; dated 2026-09-29, `s8g` build, package S8G-P2-RKTYPES) minted
    // `verify_completed`. Was: `assert_eq!(EventKind::ALL.len(), 41);`.
    assert_eq!(EventKind::ALL.len(), 42);
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

/// **ADR-0125's addition** — the verify gate's refusal to let a stage exit over unpinned ACs
/// (AC-346: *"the refusal is an event, not only a console line"*). Tier 2: the refusal is DERIVED
/// from the AC ledger and recomputed at every attempt (AC-336, AC-359); losing the row loses the
/// record of an attempt, never the refusal. Not a halt and not a step refusal — the item stays in
/// the stage and AC-357's gate is the next action.
#[test]
fn stage_exit_refused_exists_and_is_tier_two() {
    assert_eq!(
        EventKind::StageExitRefused.wire_name(),
        "stage_exit_refused"
    );
    assert_eq!(EventKind::StageExitRefused.tier(), Tier::Two);
    assert_eq!(
        EventKind::from_wire("stage_exit_refused"),
        Some(EventKind::StageExitRefused)
    );
    assert_eq!(EventKind::StageExitRefused.trailer_axis(), None);
    // a refused exit is not a refused step (ADR-0095's closed classes) and not a review block
    assert_ne!(EventKind::StageExitRefused, EventKind::StepRefused);
    assert_ne!(EventKind::StageExitRefused, EventKind::ReviewBlock);
}

/// **ADR-0126's addition** — the usage-bearing row AC-360 requires on every termination path
/// (*"exactly one dispatch event is appended carrying every token counter the harness reports
/// verbatim, `total_cost_usd`, the `five_hour` and `seven_day` headroom readings taken before and
/// after, and the pinned harness/CLI version"*), written by the Harness adapter through the store
/// it is handed. Tier 2 by the deleted-database test (decision 4): the next action is a function
/// of the tier-1 commit state; losing this row loses the cost, the usage, the headroom pair and the
/// version — never the decision. It pairs with the kernel's pre-dispatch `dispatch` row and does
/// not replace it (decision 3): two kinds, two producers, two moments.
#[test]
fn adr0126_dispatch_completed_is_tier_two() {
    // Written first against the wire name and the count, so the red is the ABSENT kind
    // (`from_wire` answering `None`, `ALL` at 37) and not a compile error naming a variant.
    // *(dated 2026-09-28, `s8e` build, package S8E-P5.)* 38 -> 41: `s8e`'s three-kinds record
    // (proposed) adds three tier-2 kinds after ADR-0126's one. *(dated 2026-09-29, `s8g` build,
    // package S8G-P2-RKTYPES.)* 41 -> 42: ADR-0191 decision 8 (proposed) adds `verify_completed`.
    assert_eq!(
        EventKind::ALL.len(),
        42,
        "ADR-0126 adds exactly one kind to 7b-ii's 37, s8e's three-kinds record three more, and          s8g's verify record one"
    );
    let kind = EventKind::from_wire("dispatch_completed")
        .expect("`dispatch_completed` is in the closed table (ADR-0126 decision 1)");
    assert_eq!(kind, EventKind::DispatchCompleted);
    assert_eq!(kind.wire_name(), "dispatch_completed");
    assert_eq!(
        kind.tier(),
        Tier::Two,
        "tier 2, not a tier-1 tag (decision 4)"
    );
    assert_eq!(
        kind.trailer_axis(),
        None,
        "no fourth trailer axis is minted (decision 4)"
    );
    // Two rows per dispatch is the honest count (decision 3): the kernel's pre-dispatch row and
    // the adapter's completion row are two kinds, and nothing may count them as one.
    assert_ne!(kind, EventKind::Dispatch);
    // The row is not one of ADR-0095's failure rows: it is written on success too.
    assert_ne!(kind, EventKind::StepFailed);
}

/// **`s8g`'s verify record (R1 = ADR-0191 decision 8, proposed) mints `verify_completed`** — AC-321's
/// replay event and the toolchain identity carrier (D21-42 10b): one row per **completed**
/// `Toolchain::test` port call, written by the kernel at one site. Tier 2 by ADR-0109 decision 2's
/// deleted-database test (the report is recomputable by re-running the suite), no trailer, and
/// item-scoped on ADR-0182's carrier, beside the payload and never in it (D21-46 R3).
/// *(Added 2026-09-29, `s8g` build, package S8G-P2-RKTYPES; `s8g` §8g.4.)*
///
/// Written against the wire name and the count, so the red is the ABSENT kind (`from_wire`
/// answering `None`, `ALL` at 41) and not a compile error naming a variant.
#[test]
fn verify_completed_is_tier_two_and_wired() {
    let kind = EventKind::from_wire("verify_completed")
        .expect("`verify_completed` is in the closed table (ADR-0191 decision 8)");
    assert_eq!(kind.wire_name(), "verify_completed");
    assert_eq!(
        kind.tier(),
        Tier::Two,
        "tier 2 by the deleted-database test"
    );
    assert_eq!(kind.trailer_axis(), None, "tier 2 rides no trailer");
    assert_eq!(
        kind.item_key(),
        rk_types::event_kind::ItemKey::Carried,
        "item-scoped, its interim key carried beside the payload (ADR-0182; D21-46 R3)"
    );
    assert_eq!(
        EventKind::ALL.len(),
        42,
        "ADR-0191 adds exactly one kind to the 41 s8e left"
    );
    assert_eq!(
        EventKind::ALL.iter().filter(|k| **k == kind).count(),
        1,
        "listed in ALL exactly once"
    );
    // A completed test call is not a failure row and not the stage-exit refusal the ledger writes.
    assert_ne!(kind, EventKind::StepFailed);
    assert_ne!(kind, EventKind::StageExitRefused);
}

/// The two dotted names are dotted. A blanket `rename_all = "snake_case"` would quietly turn them
/// into `artifact_read`/`artifact_write` and every consumer reading design §5's table would miss.
#[test]
fn the_dotted_names_are_not_normalised_to_snake_case() {
    assert_eq!(EventKind::ArtifactRead.wire_name(), "artifact.read");
    assert_eq!(EventKind::ArtifactWrite.wire_name(), "artifact.write");
}

/// Exactly ~~five~~ six kinds are tier 1, and they are the ~~five~~ six design §5's table marks as
/// git-first: stage transitions, gate opened/answered/expired/voided, and slice seals.
///
/// *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.7; D21-43 N6; ADR-0184,
/// proposed.)* `gate_answer_void` moves to tier 1 with design §5's row: N6 makes the voiding a
/// git-first transition (*"Every gate transition (opened, answered, expired, voided) is a git-first
/// tier-1 event"*). The name is kept so the re-point is visible.
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
            "slice_sealed",
            "gate_answer_void"
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
///
/// *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.7; ADR-0184, proposed.)* The
/// exception ~~`|| k == EventKind::GateExpired`~~ is gone: `gate_expired` is a `Gate: … expired`
/// mark (N6), so it names its axis like every other tier-1 kind, and so does `gate_answer_void`.
#[test]
fn the_trailer_axes_belong_to_the_tier_one_kinds() {
    assert_eq!(EventKind::GateExpired.trailer_axis(), Some("Gate"));
    assert_eq!(EventKind::GateAnswerVoid.trailer_axis(), Some("Gate"));
    for k in EventKind::ALL {
        match k.tier() {
            Tier::One => assert!(
                k.trailer_axis().is_some(),
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
