// AC-335's POSITIVE CONTROL for `verify_reads_no_suite_verdict_rule` — NOT compiled, NOT a test.
// A status function that takes the item's suite verdict as an argument: the exact input the
// register forbids ("a static check that the AC-status function does not take the suite verdict
// as an argument"). Scanned by the rule at a path inside the verify module; must be reported.
// (dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT: the criterion type is the exported
// `AcceptanceCriterion`, R2 = ADR-0192 decision 4; the control's point, the `GreenAt` input, is unchanged.)
fn ac_status(ac: &AcceptanceCriterion, run: Option<&FlipOutcome>, suite: &GreenAt) -> LedgerStatus {
    if suite.is_green() {
        return LedgerStatus::Pinned;
    }
    LedgerStatus::Unpinned
}
