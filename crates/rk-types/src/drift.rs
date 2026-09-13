//! `Drift` — something the world says that the record cannot explain. Reported, never silently
//! reconciled.
//!
//! This type exists here because `rk-types` is the owning crate for the state vocabulary, and
//! because **the design fixes five distinct drift values that must not collapse into one another**:
//! a structured-section edit blocks until resolved (AC-124/AC-125), a vouched-root change is
//! advisory and does **not** block (AC-127), a template-version bump is template drift and is
//! *"never reported as human drift"* (AC-128), an unparsable trailer is surfaced with its offending
//! text and is never coerced into a transition (AC-165), and a gate answer bound to a stale tree is
//! void (AC-176). Four more come from `reconcile()`'s own fold.
//!
//! Collapsing any two of them is the most likely way to get this wrong — a single `kind: String`
//! would compile and would quietly let template drift be reported as a human edit — so
//! [`Drift::kind`] is total and its distinctness is asserted by a test rather than assumed.

use crate::digest::Digest;
use crate::text::Text;

/// One observation that the world and the record disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drift {
    /// A human edited text inside a digest-stamped structured section. Blocks until resolved
    /// through a typed write (AC-124, AC-125).
    StructuredSection {
        artifact: String,
        anchor: String,
        stored: Digest,
        observed: Digest,
    },

    /// A human-supplied (vouched-root) artifact changed since vouching. **Advisory: does not
    /// block** (AC-127). Collapsing this into `StructuredSection` would block work the design
    /// deliberately lets through.
    Advisory {
        root: String,
        vouched: Digest,
        observed: Digest,
    },

    /// Templates moved from V to V+1 without re-rendering. Reported with **both** template
    /// versions, and *never* as human drift (AC-128).
    Template {
        artifact: String,
        from: String,
        to: String,
    },

    /// A trailer that does not parse against the declared grammar (design §5.4). Surfaced with the
    /// offending text attached; never skipped, never coerced into a state transition (AC-165).
    UnparsableTrailer {
        commit: String,
        key: String,
        offending: Text,
    },

    /// A `Gate:` value that is not the `(token, tree_hash, deadline)` triple. Legacy drift —
    /// counted and surfaced, never a transition. F12's corpus carries such lines.
    LegacyGateTrailer { commit: String, offending: Text },

    /// A gate answer bound to a tree hash that no longer matches. The answer is void and the gate
    /// re-opens with this attached (AC-176).
    GateAnswerVoid {
        token: String,
        bound: String,
        observed: String,
    },

    /// The same `Story:` committed twice for one slug. Counted, not a transition (design §5.4).
    DuplicateStoryTrailer {
        project: String,
        slug: String,
        commit: String,
    },

    /// A `Slice-Verdict:` whose value is not `<project>/<slice> <verdict>`.
    UnqualifiedSliceVerdict { commit: String, offending: Text },

    /// A slice whose last word was `changes-requested`: unfinished work a naive resume would skip.
    /// This is the finding the third trailer axis exists for — 15 of 74 slices were ever rejected
    /// [measured, F12].
    SliceLeftRejected {
        project: String,
        slice: String,
        latest: String,
    },

    /// A review clone left behind by a crash, kill or timeout. Reported by `reconcile()`; removed
    /// by the entry step's action, never by `reconcile()` itself (AC-193).
    StaleReviewClone { path: String },

    /// Rows anchored to a commit a rebase/reset/amend/force-push made unreachable. Reported with
    /// their anchor; no row is deleted or re-anchored as a side effect (AC-167).
    OrphanedAnchor { anchor: String, rows: u32 },
}

impl Drift {
    /// A stable discriminator, total over the set with no catch-all.
    ///
    /// Written as one arm per variant deliberately: `clippy::match_same_arms` is denied
    /// workspace-wide, which is exactly the lint that would fire if two variants were given the
    /// same kind — so the lint enforces the distinctness this type exists to preserve.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::StructuredSection { .. } => "structured_drift",
            Self::Advisory { .. } => "drift_advisory",
            Self::Template { .. } => "template_drift",
            Self::UnparsableTrailer { .. } => "unparsable_trailer",
            Self::LegacyGateTrailer { .. } => "legacy_gate_trailer",
            Self::GateAnswerVoid { .. } => "gate_answer_void",
            Self::DuplicateStoryTrailer { .. } => "duplicate_story_trailer",
            Self::UnqualifiedSliceVerdict { .. } => "unqualified_slice_verdict",
            Self::SliceLeftRejected { .. } => "slice_left_rejected",
            Self::StaleReviewClone { .. } => "stale_review_clone",
            Self::OrphanedAnchor { .. } => "orphaned_anchor",
        }
    }

    /// Does this drift block the next action, or is it advisory?
    ///
    /// AC-127 is the criterion this encodes: the same change on a vouched root is advisory, and on
    /// a store-rendered structured section it blocks. Getting this wrong in either direction is a
    /// criterion failure, not a policy preference.
    #[must_use]
    pub const fn blocks(&self) -> bool {
        match self {
            Self::StructuredSection { .. } | Self::GateAnswerVoid { .. } => true,
            Self::Advisory { .. }
            | Self::Template { .. }
            | Self::UnparsableTrailer { .. }
            | Self::LegacyGateTrailer { .. }
            | Self::DuplicateStoryTrailer { .. }
            | Self::UnqualifiedSliceVerdict { .. }
            | Self::SliceLeftRejected { .. }
            | Self::StaleReviewClone { .. }
            | Self::OrphanedAnchor { .. } => false,
        }
    }
}
