//! **AC-285's positive control** for the ban *"`finding.evidence_label` has no reader inside the
//! policy module"*. NOT compiled; under `lint-controls/`, not `ui/` (a grep control is an ordinary
//! file carrying the banned token, never a compile-fail case — `s7b-ii` §7b-ii.10). Checkpoint
//! 7b-ii's grep and `module_boundary::policy_reads_no_model_label_rule` must both match the line
//! below when it is treated as a policy-module source.

pub fn classify(finding: &PersistedFinding, weight: ReviewWeight) -> Decision {
    // THE READ AC-285 FORBIDS: the model's label steering the block decision.
    if finding.evidence_label == EvidenceLabel::Reproduced {
        return Decision::Block(Block { class: EvidenceClass::Reproduced, weight });
    }
    Decision::Advise(NonBlockReason::AdvisoryClass)
}
