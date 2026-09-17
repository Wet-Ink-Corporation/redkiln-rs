//! `EventKind` — design §5's event-kind table as a **closed** enum.
//!
//! ADR-0105 clause 7 puts the table in the design with `rk-types` as the owning crate; **ADR-0116**
//! supersedes that clause in part, adds `artifact.write`, and fixes the rule that makes the enum
//! worth having: *"the `EventKind` enum is closed, a kind in the design table and not in the enum
//! (or the reverse) is schema drift."* **ADR-0120** mints `dag_amended` under that same rule and
//! supersedes nothing — ADR-0116 had already withdrawn the completeness reading of clause 7, and
//! what survives of it is the rule this crate obeys by carrying the variant. **ADR-0123** mints two
//! more under the same rule for Step 7b-ii's human gate — `gate_answer_spent`, the row AC-246's
//! *"records a distinct event"* names, and `gate_resumed`, the event AC-267/AC-279 and SM-073 assert
//! on — and likewise supersedes nothing. **ADR-0124** mints the review and verify gates' five —
//! `review_block`, `quarantine_granted`, `quarantine_removed`, `quarantine_expired` and
//! `selection_ineffective` — each named by a criterion or a success measure, under the same rule.
//!
//! The **tier** column is the deleted-database test applied per kind (ADR-0109 decision 2): *if the
//! database were deleted, would `redkiln run` take a different next action?* Five kinds are tier 1
//! and are written to git first; everything else is tier 2 and is rebuildable or disposable.
//! [`EventKind::tier`] is total with no catch-all, so a new kind cannot be added without answering
//! that question.
//!
//! **Three names are owed to design §5, and they are named here rather than invented silently.**
//! The table gives three rows in prose only — *"dispatch record (cost row)"*, *"`rate_limit_event`
//! (as read) / headroom reading"* and the advisory sink's *"dismissed"*. This enum calls them
//! `Dispatch`, `RateLimitReading` and `AdvisoryDismissed`; a dated correction to design §5 naming
//! them is **owed**, and until it lands those three are this crate's proposal rather than the
//! design's word. A fourth row, *"`slice_sealed` / verdict"*, is **one** kind carrying a verdict
//! field, not two.
//!
//! **`ReplayDivergence` is deliberately not here.** Its tier column reads `—` and the row says
//! *"a halt, not a durable record"*. It lives beside the enum as its own type, and the completeness
//! test encodes that exclusion **by name** — otherwise it would read as a missing kind.

/// Durability tier. ADR-0109 decision 2's split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    /// Decision-bearing: written to git first, as a trailer on a checkpoint or empty commit.
    One,
    /// Telemetry: event store only, rebuildable or disposable.
    Two,
}

/// Every event kind the system may record. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventKind {
    // --- tier 1: decision-bearing, git first ---
    StageTransition,
    GateOpened,
    GateAnswered,
    GateExpired,
    SliceSealed,

    // --- tier 2: telemetry ---
    ReconcileStarted,
    GateAnswerVoid,
    /// Added by **ADR-0123**: a second answer presenting a token the answered set already holds is
    /// refused as spent (ADR-0017's single-use token; AC-246), and this row is the record that it
    /// arrived. Tier 2: the refusal is derived from the tier-1 `gate_answered` rows, so losing this
    /// row loses the count of refusals and never the refusal.
    GateAnswerSpent,
    /// Added by **ADR-0123**: the first kernel action on an item after one of its gates reached a
    /// terminal state. Carries the kernel's `prefix_rewrites` count (AC-279: ≤ 1) and, once an
    /// adapter fills them at Step 7c, `cache_creation_input_tokens` and `total_cost_usd` (AC-267,
    /// SM-073) — **absent, not zero**, until then.
    GateResumed,
    /// Added by **ADR-0124**: AC-307's block event, written by the path that decided the block and
    /// nowhere else, carrying `adjudication: null` for a later adjudication to attach to.
    ReviewBlock,
    /// Added by **ADR-0124**: the human override moving a finding to `Quarantined` (ADR-0074
    /// clause 6). Carries AC-307's members plus `reason, by, until`.
    QuarantineGranted,
    /// Added by **ADR-0124**: the human override lifting a quarantine before `until`.
    QuarantineRemoved,
    /// Added by **ADR-0124**: `until` passed and the finding is re-raised (AC-306).
    QuarantineExpired,
    /// Added by **ADR-0124**: the verify gate's bootstrap comparator reclassified an `affected`
    /// claim (AC-326; ADR-0104 decision 3) — `selection-ineffective` or `probe-unverified`.
    SelectionIneffective,
    DagAssembled,
    /// Added by **ADR-0120**: the delta an accepted deviation writes when selection steps 2–5
    /// re-run (AC-064). Tier 2 for the same reason `DagAssembled` is — a delta may not outrank the
    /// assembly it amends, and the deviation record is already in the replay input set.
    DagAmended,
    ActivityCommitted,
    MemoHit,
    MemoMiss,
    MemoEvicted,
    /// The dispatch cost row. Name owed to design §5 (given there in prose only).
    Dispatch,
    StepFailed,
    StepCancelled,
    StepRefused,
    RateLimited,
    /// The headroom reading as the adapter's stream reports it. Name owed to design §5.
    RateLimitReading,
    Degraded,
    StructuredDrift,
    DriftAdvisory,
    ReviewClassDrift,
    /// `rewrite(old_sha -> new_sha, mechanism)`. Tier-2-derived, and **outside** the AC-184 rebuild
    /// differential (ADR-0105 clause 9).
    Rewrite,
    ArtifactRead,
    /// Added by **ADR-0116**: the producing half of `artifact.read`, without which `doctor` cannot
    /// tell a produced-and-never-read artifact from one never produced.
    ArtifactWrite,
    AdvisoryCreated,
    AdvisoryRetired,
    /// Name owed to design §5.
    AdvisoryDismissed,
}

impl EventKind {
    /// Every kind, for the completeness test. Built from an exhaustive match so a new variant
    /// cannot be added without appearing here.
    pub const ALL: [Self; 36] = [
        Self::StageTransition,
        Self::GateOpened,
        Self::GateAnswered,
        Self::GateExpired,
        Self::SliceSealed,
        Self::ReconcileStarted,
        Self::GateAnswerVoid,
        Self::GateAnswerSpent,
        Self::GateResumed,
        Self::ReviewBlock,
        Self::QuarantineGranted,
        Self::QuarantineRemoved,
        Self::QuarantineExpired,
        Self::SelectionIneffective,
        Self::DagAssembled,
        Self::DagAmended,
        Self::ActivityCommitted,
        Self::MemoHit,
        Self::MemoMiss,
        Self::MemoEvicted,
        Self::Dispatch,
        Self::StepFailed,
        Self::StepCancelled,
        Self::StepRefused,
        Self::RateLimited,
        Self::RateLimitReading,
        Self::Degraded,
        Self::StructuredDrift,
        Self::DriftAdvisory,
        Self::ReviewClassDrift,
        Self::Rewrite,
        Self::ArtifactRead,
        Self::ArtifactWrite,
        Self::AdvisoryCreated,
        Self::AdvisoryRetired,
        Self::AdvisoryDismissed,
    ];

    /// The wire name, exactly as design §5's table spells it — including the two **dotted** names,
    /// which are not snake_case and must not be normalised into it.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::StageTransition => "stage_transition",
            Self::GateOpened => "gate_opened",
            Self::GateAnswered => "gate_answered",
            Self::GateExpired => "gate_expired",
            Self::SliceSealed => "slice_sealed",
            Self::ReconcileStarted => "reconcile_started",
            Self::GateAnswerVoid => "gate_answer_void",
            Self::GateAnswerSpent => "gate_answer_spent",
            Self::GateResumed => "gate_resumed",
            Self::ReviewBlock => "review_block",
            Self::QuarantineGranted => "quarantine_granted",
            Self::QuarantineRemoved => "quarantine_removed",
            Self::QuarantineExpired => "quarantine_expired",
            Self::SelectionIneffective => "selection_ineffective",
            Self::DagAssembled => "dag_assembled",
            Self::DagAmended => "dag_amended",
            Self::ActivityCommitted => "activity_committed",
            Self::MemoHit => "memo_hit",
            Self::MemoMiss => "memo_miss",
            Self::MemoEvicted => "memo_evicted",
            Self::Dispatch => "dispatch",
            Self::StepFailed => "step_failed",
            Self::StepCancelled => "step_cancelled",
            Self::StepRefused => "step_refused",
            Self::RateLimited => "rate_limited",
            Self::RateLimitReading => "rate_limit_reading",
            Self::Degraded => "degraded",
            Self::StructuredDrift => "structured_drift",
            Self::DriftAdvisory => "drift_advisory",
            Self::ReviewClassDrift => "review_class_drift",
            Self::Rewrite => "rewrite",
            Self::ArtifactRead => "artifact.read",
            Self::ArtifactWrite => "artifact.write",
            Self::AdvisoryCreated => "advisory_created",
            Self::AdvisoryRetired => "advisory_retired",
            Self::AdvisoryDismissed => "advisory_dismissed",
        }
    }

    /// The deleted-database test, per kind.
    ///
    /// Written as two or-pattern arms rather than one arm per variant: `clippy::match_same_arms` is
    /// denied workspace-wide, and grouping is what the lint asks for. Exhaustiveness still makes a
    /// new variant an `E0004` here, which is the property that matters — a kind cannot enter the
    /// system without someone deciding whether losing it would change the next action.
    #[must_use]
    pub const fn tier(self) -> Tier {
        match self {
            Self::StageTransition
            | Self::GateOpened
            | Self::GateAnswered
            | Self::GateExpired
            | Self::SliceSealed => Tier::One,

            Self::ReconcileStarted
            | Self::GateAnswerVoid
            | Self::GateAnswerSpent
            | Self::GateResumed
            | Self::ReviewBlock
            | Self::QuarantineGranted
            | Self::QuarantineRemoved
            | Self::QuarantineExpired
            | Self::SelectionIneffective
            | Self::DagAssembled
            | Self::DagAmended
            | Self::ActivityCommitted
            | Self::MemoHit
            | Self::MemoMiss
            | Self::MemoEvicted
            | Self::Dispatch
            | Self::StepFailed
            | Self::StepCancelled
            | Self::StepRefused
            | Self::RateLimited
            | Self::RateLimitReading
            | Self::Degraded
            | Self::StructuredDrift
            | Self::DriftAdvisory
            | Self::ReviewClassDrift
            | Self::Rewrite
            | Self::ArtifactRead
            | Self::ArtifactWrite
            | Self::AdvisoryCreated
            | Self::AdvisoryRetired
            | Self::AdvisoryDismissed => Tier::Two,
        }
    }

    /// Is this kind carried on a git trailer? The tier-1 kinds are, and design §5 names which
    /// trailer axis each uses.
    #[must_use]
    pub const fn trailer_axis(self) -> Option<&'static str> {
        match self {
            Self::StageTransition => Some("Story"),
            Self::GateOpened | Self::GateAnswered => Some("Gate"),
            Self::SliceSealed => Some("Slice-Verdict"),
            _ => None,
        }
    }

    #[must_use]
    pub fn from_wire(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.wire_name() == name)
    }
}

/// A replay divergence. **Not an `EventKind`** — design §5's own tier column reads `—` and calls it
/// *"a halt, not a durable record"*. Recording it as an event would make the halt durable and let a
/// run continue past it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayDivergence {
    pub activity_id: String,
    pub recorded: String,
    pub recomputed: String,
}
