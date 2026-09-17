//! The persisted finding record — what the review gate keeps about one finding once the confirm
//! half has run.
//!
//! `evidence_label` is the model's own label, kept here **for drift monitoring only** (ADR-0074
//! clause 2; AC-285). The policy module has no reader of it — that is an architecture lint, and the
//! reason the member is named exactly as the register spells it.

use rk_types::artifact::findings::EvidenceLabel;

use super::classify::EvidenceClass;
use super::policy::Decision;
use super::reexec::ReExecution;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedFinding {
    pub ac: String,
    /// The model's label — retained on the record, routed nowhere (AC-285).
    pub evidence_label: EvidenceLabel,
    /// The class the classifier computed from the repro record and the observation.
    pub computed_class: EvidenceClass,
    pub re_execution: Option<ReExecution>,
    pub decision: Decision,
}
