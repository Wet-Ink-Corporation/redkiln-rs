//! The persisted finding record — what the review gate keeps about one finding once the confirm
//! half has run.
//!
//! `evidence_label` is the model's own label, kept here **for drift monitoring only** (ADR-0074
//! clause 2; AC-285). The policy module has no reader of it — that is an architecture lint, and the
//! reason the member is named exactly as the register spells it.

use rk_types::artifact::findings::EvidenceLabel;
use rk_types::digest::Digest;
use rk_types::review::Quarantine;

use super::classify::EvidenceClass;
use super::policy::{Decision, ReviewWeight};
use super::reexec::ReExecution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedFinding {
    /// AC-305's stable identity (`rk_store::artifacts::finding_identity`).
    pub finding_id: Digest,
    pub ac: String,
    /// The model's label — retained on the record, routed nowhere (AC-285).
    pub evidence_label: EvidenceLabel,
    /// The class the classifier computed from the repro record and the observation.
    pub computed_class: EvidenceClass,
    pub re_execution: Option<ReExecution>,
    /// The re-execution record's id, when the gate produced one (AC-307's `re_execution_id`).
    pub re_execution_id: Option<String>,
    /// The reviewed sha and the weight the decision was taken at — so every override row written
    /// later from this record (quarantine grant/removal) carries AC-307's `sha` and `stage_weight`
    /// as the block row did, from the record and not from a caller's memory.
    pub sha: String,
    pub weight: ReviewWeight,
    pub decision: Decision,
    /// The active quarantine, when the decision is `Advise(Quarantined)` (AC-304): persisted and
    /// rendered with its reason, never deleted.
    pub quarantine: Option<Quarantine>,
}
