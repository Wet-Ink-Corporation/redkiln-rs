//! The classifier — the pure function over the repro record and the gate's observation of its
//! re-execution (ADR-0074 clause 2, as ADR-0096 decision 2 supersedes it; AC-284).
//!
//! **One total decision table and nothing else.** The class is computed here, from the typed
//! repro (`kind`, `expected`, `expected_when_met`) and what the deterministic gate observed; the
//! model's own `evidence` label is not an input and never will be (AC-285 keeps it for drift
//! monitoring on the persisted record). The match is exhaustive with no `_ =>` fallthrough to a
//! blocking class: every unrecognised shape is `advisory`, and `tests/it/review.rs` reads this
//! file to hold that.
//!
//! | `kind` | the gate observed | class |
//! | --- | --- | --- |
//! | `run` | ran, `observed ≠ expected_when_met` | `reproduced` |
//! | `run` | ran, outcome unchanged, only the discriminating assertion flipped | `narrowed-pin` |
//! | `run` | anything else (did not run; unchanged; wrong observation kind) | `advisory` |
//! | `mutate` | applied, suite green, reverted-fix patch | `mutation-survived` |
//! | `mutate` | applied, suite green, the named assertion flipped | `mutation-survived` |
//! | `mutate` | applied, suite green, an assertion that did NOT flip | `narrowed-pin` |
//! | `mutate` | applied, suite green, no assertion and not a reverted fix (an equivalent mutant) | `advisory` |
//! | `mutate` | not applied, or suite red (the mutant was killed), or wrong observation kind | `advisory` |
//! | either | `Unrecognised` | `advisory` |
//!
//! **Reads no clock, no environment, no file.** AC-286's source-level lint over this module's own
//! syntax is `crate::module_boundary::classifier_purity_rule` — on the SYNTAX, because *"a
//! crate-level dependency-edge check alone would not catch a transitive route"* — and
//! `tests/fixtures/lint-controls/ac286_classifier_reads_the_clock.rs` is its positive control.
//! F27 measured the model half's verdict flaky on 2/4 items over 21 draws; this half may not flip
//! at all, and a function of two values cannot.
//!
//! **AC-284 is PROVISIONAL on S-CLASS-REPLAY** — the table is built and tested, and its agreement
//! with the model's labels over the 253 recorded S-TYPED findings is unmeasured until that spike
//! runs. Nothing here claims it.

use rk_types::repro::{Repro, ReproKind};

/// The four evidence classes — **closed**; there is no fifth and no `Unverifiable` here
/// (`Unverifiable` is a re-execution OUTCOME carried beside the class, AC-290).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceClass {
    Reproduced,
    MutationSurvived,
    NarrowedPin,
    Advisory,
}

impl EvidenceClass {
    pub const ALL: [Self; 4] = [
        Self::Reproduced,
        Self::MutationSurvived,
        Self::NarrowedPin,
        Self::Advisory,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reproduced => "reproduced",
            Self::MutationSurvived => "mutation-survived",
            Self::NarrowedPin => "narrowed-pin",
            Self::Advisory => "advisory",
        }
    }
}

/// What the deterministic gate observed when it ran a `kind = run` repro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunObservation {
    /// The command ran to completion (an exit code was observed).
    pub ran: bool,
    /// What the run produced, in the same terms as `repro.observed` / `expected_when_met`.
    pub observed: String,
    /// The discriminating assertion (when `expected` names one) flipped while the command's
    /// overall outcome did not — the `narrowed-pin` shape for a run.
    pub only_assertion_flipped: bool,
}

/// What the gate observed when it applied a `kind = mutate` repro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutateObservation {
    /// `mutation.patch` applied cleanly at the reviewed SHA.
    pub applied: bool,
    /// `mutation.suite` stayed green with the patch applied.
    pub suite_green: bool,
    /// The patch is a reverted fix (ADR-0096 decision 2 (a)).
    pub reverted_fix: bool,
    /// For a finding carrying `expected.assertion`: did the gate observe the assertion flip?
    /// `None` when the finding carries no assertion.
    pub assertion_flipped: Option<bool>,
}

/// The gate's observation for one finding. `Unrecognised` is every input the gate could not
/// place — and it classifies as `advisory`, never a panic (AC-284's fuzz clause).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Observation {
    Run(RunObservation),
    Mutate(MutateObservation),
    Unrecognised,
}

/// The total decision table. A pure function of its two arguments.
#[must_use]
pub fn classify(repro: &Repro, observation: &Observation) -> EvidenceClass {
    match (repro.kind, observation) {
        (ReproKind::Run, Observation::Run(run)) => {
            if !run.ran {
                return EvidenceClass::Advisory;
            }
            if run.observed != repro.expected_when_met {
                return EvidenceClass::Reproduced;
            }
            if run.only_assertion_flipped {
                return EvidenceClass::NarrowedPin;
            }
            EvidenceClass::Advisory
        }
        (ReproKind::Mutate, Observation::Mutate(m)) => {
            // ADR-0096 decision 2: a surviving mutant blocks only when it DISCRIMINATES —
            // a reverted-fix patch, or an assertion the gate observed flip.
            if !m.applied || !m.suite_green {
                return EvidenceClass::Advisory;
            }
            if m.reverted_fix || m.assertion_flipped == Some(true) {
                return EvidenceClass::MutationSurvived;
            }
            if m.assertion_flipped == Some(false) {
                return EvidenceClass::NarrowedPin;
            }
            // green, no assertion, not a reverted fix: the equivalent mutant, which cannot block
            EvidenceClass::Advisory
        }
        // the kinds disagree, or the gate could not place what it saw
        (ReproKind::Run, Observation::Mutate(_))
        | (ReproKind::Mutate, Observation::Run(_))
        | (ReproKind::Run | ReproKind::Mutate, Observation::Unrecognised) => {
            EvidenceClass::Advisory
        }
    }
}
