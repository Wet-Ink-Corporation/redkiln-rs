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
//! **ADR-0126** mints `dispatch_completed` — the usage-bearing row AC-360's THEN requires on every
//! termination path and design §5's table carried as a description with no wire name — under the
//! same rule, and likewise supersedes nothing; the kind and the completed `DispatchRecord` it
//! carries land in one change (its decision 5), which is Step 7c's Slice 1.
//!
//! The **tier** column is the deleted-database test applied per kind (ADR-0109 decision 2): *if the
//! database were deleted, would `redkiln run` take a different next action?* Five kinds are tier 1
//! and are written to git first; everything else is tier 2 and is rebuildable or disposable.
//! [`EventKind::tier`] is total with no catch-all, so a new kind cannot be added without answering
//! that question.
//!
//! **Two names are owed to design §5, and they are named here rather than invented silently.**
//! The table gives two rows in prose only — *"`rate_limit_event` (as read) / headroom reading"*
//! and the advisory sink's *"dismissed"*. This enum calls them `RateLimitReading` and
//! `AdvisoryDismissed`; a dated correction to design §5 naming them is **owed**, and until it
//! lands those two are this crate's proposal rather than the design's word. A third row,
//! *"`slice_sealed` / verdict"*, is **one** kind carrying a verdict field, not two. **Until
//! ADR-0126 (2026-09-17) there were three owed names**: the table's *"dispatch record (cost row)"*
//! was the third and this enum's `Dispatch` was its name. ADR-0126 decision 1 replaced that row
//! with `dispatch_completed`, which `DispatchCompleted` carries, and decision 3 keeps `Dispatch` —
//! the kernel's pre-dispatch row, the idempotency key — **without giving it a row of its own**. So
//! after ADR-0126 `Dispatch` is a kind in this enum with no row in design §5's table, which is
//! ADR-0116's *"schema drift"* shape by its own definition; the row it needs is one line ADR-0126
//! decision 3 already spells (tier 2; the kernel, before the child exists; payload the activation
//! key) and it is RAISED for `s7c` §7c.22's ledger rather than added here, because a design row is
//! minted by record and not by a doc comment.
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

/// Where a kind's owning item is recorded — ADR-0182 decision 3 (`s8e` §8e.10; D21-43 N5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKey {
    /// Item-scoped: the producer sets the interim key beside the event (`Event::with_item`).
    Carried,
    /// Item-scoped, keyed in the payload by an earlier record (ADR-0163's `rate_limit_reading.item`).
    InPayload,
    /// Not item-scoped; ADR-0182 gives the reason per kind.
    NotItemScoped,
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
    /// Added by **ADR-0125**: the verify gate refused a stage exit over unpinned ACs and named them
    /// (AC-346). Derived from the AC ledger, recomputed at every attempt — tier 2.
    StageExitRefused,
    DagAssembled,
    /// Added by **ADR-0120**: the delta an accepted deviation writes when selection steps 2–5
    /// re-run (AC-064). Tier 2 for the same reason `DagAssembled` is — a delta may not outrank the
    /// assembly it amends, and the deviation record is already in the replay input set.
    DagAmended,
    ActivityCommitted,
    MemoHit,
    MemoMiss,
    MemoEvicted,
    /// **The kernel's pre-dispatch row** — the idempotency key, written before the child exists,
    /// for AC-012/AC-013's commit protocol — and it carries no counter. Until ADR-0126
    /// (2026-09-17) this was this crate's name for design §5's *"dispatch record (cost row)"*,
    /// given there in prose only; that row is now `dispatch_completed`, which
    /// [`Self::DispatchCompleted`] carries, and ADR-0126 decision 3 keeps this kind beside it —
    /// two kinds, two producers, two moments, and *"nothing counts them as one, and nothing
    /// may"* — with **no design §5 row of its own**: RAISED, not settled (the module doc).
    Dispatch,
    /// Added by **ADR-0126**: the usage-bearing row — the completed `rk_types::DispatchRecord`
    /// (usage, timings, `total_cost_usd`, the two headroom readings, the two-armed CLI version,
    /// the `api_retry_events` count, the two TTL values, and the cancel cause / detection arm on
    /// the paths where they apply) — appended by the Harness adapter, once per dispatch, on
    /// **every** termination path, through the store `dispatch` receives (AC-360). Tier 2 by the
    /// deleted-database test (decision 4): the next action is a function of the tier-1 commit
    /// state; losing this row loses the cost, the usage, the headroom pair and the version, and
    /// never the decision. The kernel folds it and moves nothing (decision 5).
    DispatchCompleted,
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
    /// *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.4; D21-42 8d; the three-kinds
    /// record, proposed.)* The run-owned worktree a `run` entered: `{ item, worktree, branch,
    /// reused, discarded }`, written at **every** run start, on a created tree and on a reused one.
    /// Every tier-1 write resolves its `RunOwnership` from the latest one for its item, and from
    /// nothing else. Tier 2 by the deleted-database test: `run` re-enters the registered tree and
    /// writes it again without re-creating anything, and `answer` refuses fail-closed until it
    /// does, so losing it changes no next action of `run`.
    WorktreeAssigned,
    /// *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.6; D21-42 7; the three-kinds
    /// record, proposed.)* A stage the lane entered and passed with an empty plan: `{ stage,
    /// reason }`, the reason one of [`STAGE_EMPTY_REASONS`]. Tier 2: the `Story:` mark carries the
    /// entry either way, and the row is anchored to that mark's commit as a row of the same act.
    StageEmpty,
    /// *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3; D21-43 N7; the three-kinds
    /// record, proposed.)* One journalled git invocation of the kernel's tier-1 boundary —
    /// `GitInvocation`'s own members `{ op, cwd, argv_digest, status, stdout, stderr_digest }` —
    /// anchored to the act's commit, so replay serves the boundary's git writes from the journal.
    /// Tier 2: the commit it describes is the durable fact.
    GitOp,
}

/// *(dated 2026-09-28, `s8e` build, package S8E-P5; the three-kinds record decision 3, proposed.)*
/// The closed reason set a `stage_empty` row carries. One member at Checkpoint 8e:
/// `no_applicable_activity` — the stage's assembled plan holds zero nodes, so nothing is
/// dispatchable there at the effective rigor. A new reason is a record, never a producer's choice.
pub const STAGE_EMPTY_REASONS: [&str; 1] = ["no_applicable_activity"];

impl EventKind {
    /// Every kind, for the completeness test. Built from an exhaustive match so a new variant
    /// cannot be added without appearing here.
    pub const ALL: [Self; 41] = [
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
        Self::StageExitRefused,
        Self::DagAssembled,
        Self::DagAmended,
        Self::ActivityCommitted,
        Self::MemoHit,
        Self::MemoMiss,
        Self::MemoEvicted,
        Self::Dispatch,
        Self::DispatchCompleted,
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
        Self::WorktreeAssigned,
        Self::StageEmpty,
        Self::GitOp,
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
            Self::StageExitRefused => "stage_exit_refused",
            Self::DagAssembled => "dag_assembled",
            Self::DagAmended => "dag_amended",
            Self::ActivityCommitted => "activity_committed",
            Self::MemoHit => "memo_hit",
            Self::MemoMiss => "memo_miss",
            Self::MemoEvicted => "memo_evicted",
            Self::Dispatch => "dispatch",
            Self::DispatchCompleted => "dispatch_completed",
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
            Self::WorktreeAssigned => "worktree_assigned",
            Self::StageEmpty => "stage_empty",
            Self::GitOp => "git_op",
        }
    }

    /// The deleted-database test, per kind.
    ///
    /// Written as two or-pattern arms rather than one arm per variant: `clippy::match_same_arms` is
    /// denied workspace-wide, and grouping is what the lint asks for. Exhaustiveness still makes a
    /// new variant an `E0004` here, which is the property that matters — a kind cannot enter the
    /// system without someone deciding whether losing it would change the next action.
    #[must_use]
    ///
    /// *(Dated 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.7; D21-43 N6; ADR-0184,
    /// proposed.)* `gate_answer_void` moves from tier 2 to tier 1 with design §5's row: N6 makes
    /// every gate transition, the void included, a git-first tier-1 event.
    pub const fn tier(self) -> Tier {
        match self {
            Self::StageTransition
            | Self::GateOpened
            | Self::GateAnswered
            | Self::GateExpired
            | Self::GateAnswerVoid
            | Self::SliceSealed => Tier::One,

            Self::ReconcileStarted
            | Self::GateAnswerSpent
            | Self::GateResumed
            | Self::ReviewBlock
            | Self::QuarantineGranted
            | Self::QuarantineRemoved
            | Self::QuarantineExpired
            | Self::SelectionIneffective
            | Self::StageExitRefused
            | Self::DagAssembled
            | Self::DagAmended
            | Self::ActivityCommitted
            | Self::MemoHit
            | Self::MemoMiss
            | Self::MemoEvicted
            | Self::Dispatch
            | Self::DispatchCompleted
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
            | Self::AdvisoryDismissed
            | Self::WorktreeAssigned
            | Self::StageEmpty
            | Self::GitOp => Tier::Two,
        }
    }

    /// Where this kind's item lives — ADR-0182 decision 3's list (`s8e` §8e.10; D21-43 N5).
    ///
    /// Total with no catch-all, so a kind cannot be added without deciding it, and
    /// `fsm::every_item_scoped_kind_carries_the_interim_item_key` pins the list. Grouped into
    /// three or-pattern arms for `clippy::match_same_arms`, as [`Self::tier`] is.
    #[must_use]
    pub const fn item_key(self) -> ItemKey {
        match self {
            Self::StageTransition
            | Self::GateOpened
            | Self::GateAnswered
            | Self::GateExpired
            | Self::SliceSealed
            | Self::ReconcileStarted
            | Self::GateAnswerVoid
            | Self::GateAnswerSpent
            | Self::GateResumed
            | Self::ReviewBlock
            | Self::QuarantineGranted
            | Self::QuarantineRemoved
            | Self::QuarantineExpired
            | Self::SelectionIneffective
            | Self::StageExitRefused
            | Self::DagAssembled
            | Self::DagAmended
            | Self::ActivityCommitted
            | Self::Dispatch
            | Self::DispatchCompleted
            | Self::StepFailed
            | Self::StepCancelled
            | Self::StepRefused
            | Self::RateLimited
            | Self::Degraded
            | Self::StructuredDrift
            | Self::DriftAdvisory
            | Self::ReviewClassDrift
            | Self::ArtifactRead
            | Self::ArtifactWrite
            | Self::WorktreeAssigned
            | Self::StageEmpty
            | Self::GitOp => ItemKey::Carried,

            // ADR-0163 keys it in its payload; a second source beside it could disagree.
            Self::RateLimitReading => ItemKey::InPayload,

            // The memo is content-keyed and shared across items (ADR-0097 d1); a rewrite is a fact
            // about the repository; the advisory sink is store-wide (AC-310's invariant).
            Self::MemoHit
            | Self::MemoMiss
            | Self::MemoEvicted
            | Self::Rewrite
            | Self::AdvisoryCreated
            | Self::AdvisoryRetired
            | Self::AdvisoryDismissed => ItemKey::NotItemScoped,
        }
    }

    /// Is this kind carried on a git trailer? The tier-1 kinds are, and design §5 names which
    /// trailer axis each uses.
    ///
    /// *(Dated 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.7; ADR-0184, proposed.)*
    /// `gate_expired` ~~maps to `None`~~ is a `Gate: … expired` mark, and `gate_answer_void` a
    /// `Gate: … voided` mark (D21-43 N6), so both ride the `Gate` axis.
    #[must_use]
    pub const fn trailer_axis(self) -> Option<&'static str> {
        match self {
            Self::StageTransition => Some("Story"),
            Self::GateOpened | Self::GateAnswered | Self::GateExpired | Self::GateAnswerVoid => {
                Some("Gate")
            }
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
