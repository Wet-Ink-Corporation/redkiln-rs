// AC-335's POSITIVE CONTROL for `verify_reads_no_suite_verdict_rule` — NOT compiled, NOT a test.
// A status function that takes the item's suite verdict as an argument: the exact input the
// register forbids ("a static check that the AC-status function does not take the suite verdict
// as an argument"). Scanned by the rule at a path inside the verify module; must be reported.
fn ac_status(ac: &SpecAc, run: Option<&FlipOutcome>, suite: &GreenAt) -> LedgerStatus {
    if suite.is_green() {
        return LedgerStatus::Pinned;
    }
    LedgerStatus::Unpinned
}
