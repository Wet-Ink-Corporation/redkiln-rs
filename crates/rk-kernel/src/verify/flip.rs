//! **AC-334's flip predicate** — the one predicate (ADR-0104 decision 2): given an author-declared
//! `mutation: { patch, suite }` applied at the candidate SHA, the named suite re-run and the tree
//! reverted, the AC is `pinned` **only if the named assertion flips from pass to fail**, and
//! `mutation-survived` if the suite stays green. F25 is the warrant: remove step 7 → RED, invert
//! the ordering → **36/36 GREEN** — green at file granularity is not evidence at assertion
//! granularity.
//!
//! The run arrives as a [`MutationRun`] — what the harness RECORDED — and the predicate is a pure
//! function of it. At this stage the run is scripted: `Toolchain::test` returns a count and the
//! Phase-1 fake cannot script a flip, so nothing here applies a patch or runs a suite. That is
//! why AC-334 is ARMED against the fake and the F25 fixture is a record replayed, not a suite run.
//!
//! AC-338's predicate half lives here too: a mutation that was not present at completion (F17's
//! agent-restored fix) is `Unverifiable`, never `mutation-survived` and never `pinned`. The post-run
//! content hash that detects it is the harness's (7c).

/// What the harness recorded about one declared mutation's run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationRun {
    /// The patch applied cleanly at the candidate SHA.
    pub applied: bool,
    /// The named suite stayed green under the mutant.
    pub suite_green: bool,
    /// The named assertion was observed to flip pass → fail (`Some(true)`), observed and did not
    /// (`Some(false)`), or was not observed at all (`None`).
    pub assertion_flipped: Option<bool>,
    /// The mutation was still present when the suite finished (AC-338).
    pub still_present: bool,
}

/// Why a run could not say pinned or survived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipUnverifiable {
    /// The declared patch did not apply at the candidate SHA.
    PatchDidNotApply,
    /// The mutation was reverted or restored before the suite finished (AC-338; F17).
    NotPresentAtCompletion,
    /// The suite went red, but not at the named assertion — the mutant broke something else.
    RedElsewhere,
}

/// The predicate's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipOutcome {
    /// The named assertion flipped pass → fail: the AC is pinned by it.
    Pinned,
    /// The suite stayed green under the mutant: the assertion does not discriminate.
    MutationSurvived,
    Unverifiable(FlipUnverifiable),
}

/// **The predicate.** In order: a patch that did not apply verifies nothing; a mutation gone by
/// completion verifies nothing (AC-338); the named assertion observed to flip pins; a green suite
/// is a survivor; a red suite that did not flip the named assertion is neither — the mutant broke
/// something else, and that is no evidence about THIS assertion.
#[must_use]
pub const fn flip_predicate(run: &MutationRun) -> FlipOutcome {
    if !run.applied {
        return FlipOutcome::Unverifiable(FlipUnverifiable::PatchDidNotApply);
    }
    if !run.still_present {
        return FlipOutcome::Unverifiable(FlipUnverifiable::NotPresentAtCompletion);
    }
    match (run.assertion_flipped, run.suite_green) {
        (Some(true), _) => FlipOutcome::Pinned,
        (Some(false) | None, true) => FlipOutcome::MutationSurvived,
        (Some(false) | None, false) => FlipOutcome::Unverifiable(FlipUnverifiable::RedElsewhere),
    }
}
