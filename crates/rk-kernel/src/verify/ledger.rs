//! **The AC evidence ledger's computation** (AC-333, AC-335, AC-337, AC-342, AC-345): one closed
//! status per spec AC, from four inputs and no fifth. *(dated 2026-09-29, s8g build, package
//! S8G-P3-KERNEL-GIT: the first input is now carried as two values, the criterion and the assertion
//! its pin names, because the exported criterion type has no locator member; the rows below list
//! them apart. No new kind of input is read.)*
//!
//! | input | who supplies it |
//! | --- | --- |
//! | the spec's ACs — id and the author-declared mutation or its `not_feasible { reason }` (ADR-0104 decision 1) | `rk_types::acceptance::AcceptanceCriterion` *(dated 2026-09-29, s8g build: R2 = ADR-0192 decision 4 retires the kernel-local shape the operator chose on 2026-09-17)* |
//! | the assertion each pin names (AC-333) | the harness's evidence, keyed by criterion id beside the flip outcomes |
//! | the flip predicate's recorded outcome per AC | [`crate::verify::flip`] |
//! | the toolchain's mutation feasibility | `rk_ports::toolchain::MutationFeasibility` (AC-337) |
//! | a human's recorded answer FOR THAT AC (AC-357's machine-checkable part: `by`, `at`) | [`HumanAnswer`] |
//!
//! **Not an input**: the item's suite verdict (AC-335 — its absence from [`ac_status`]'s signature is
//! the static check), a review's findings (AC-342), a quarantine (AC-345; ADR-0104 decision 4), a
//! CI status (AC-341). `crate::module_boundary::verify_reads_no_suite_verdict_rule` bans their
//! names on every code line of this module.
//!
//! `unpinned` is what an AC with no evidence GETS, as a row (AC-333): a declared mutation whose run
//! is missing or unverifiable, an AC declaring no mutation, a flip with no named assertion.

use std::collections::BTreeMap;

use rk_ports::toolchain::MutationFeasibility;
use rk_types::acceptance::{AcceptanceCriterion, MutationDeclaration};
use rk_types::artifact::ledger::{AcLedger, AcLedgerRow, AssertionLocator, LedgerStatus};
use rk_types::text::Text;
use rk_types::time::Timestamp;

use super::flip::FlipOutcome;

// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT; R2 = ADR-0192 decision 4, proposed.)*
// The kernel-local criterion type, its mutation enum, its error and its TOML fixture reader are
// RETIRED: the ledger reads the exported `rk_types::acceptance::AcceptanceCriterion`, which comes
// through its one door (`AcceptanceCriterion::ingest`), and a spec's declaration arms are that
// type's `MutationDeclaration` (absent = declared nothing). The locator a pin names (AC-333) is not
// a member of the exported type (decision 3's field list has none): it is the harness's evidence,
// handed in beside the flip outcomes, keyed by criterion id.

/// A human gate's recorded answer for ONE AC (AC-357's machine-checkable part; AC-346's other exit).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanAnswer {
    pub ac: String,
    pub by: String,
    pub at: Timestamp,
    pub reason: Text,
}

/// **One AC's status**, from the four inputs and no fifth. The signature IS AC-335's static check:
/// there is no parameter for the item's suite verdict, none for a review's findings (AC-342),
/// none for a quarantine (AC-345).
///
/// Order: a human answer for THIS AC (AC-357's machine-checkable part) settles it; an infeasible
/// toolchain, or an AC declared `not-feasible`, is `not_feasible` (AC-337 — never `pinned`); a
/// declared mutation's recorded run gives `pinned` only on a flip AND a named assertion to pin
/// (AC-333: a pin names its assertion), `mutation_survived` on a green suite; everything else —
/// no declaration, no run, an unverifiable run, a flip with nothing named — is `unpinned`, the
/// reserved name for *no evidence*.
#[must_use]
pub fn ac_status(
    ac: &AcceptanceCriterion,
    run: Option<&FlipOutcome>,
    assertion: Option<&AssertionLocator>,
    feasibility: &MutationFeasibility,
    answer: Option<&HumanAnswer>,
) -> LedgerStatus {
    if let Some(a) = answer.filter(|a| a.ac == ac.id) {
        return LedgerStatus::HumanAnswered {
            reason: a.reason.clone(),
        };
    }
    if let MutationFeasibility::NotFeasible { reason } = feasibility {
        return LedgerStatus::NotFeasible {
            reason: Text::new(reason.clone()),
        };
    }
    match (&ac.mutation, run) {
        (Some(MutationDeclaration::NotFeasible { reason }), _) => LedgerStatus::NotFeasible {
            reason: reason.clone(),
        },
        (Some(MutationDeclaration::Declared { .. }), Some(FlipOutcome::Pinned))
            if assertion.is_some() =>
        {
            LedgerStatus::Pinned
        }
        (Some(MutationDeclaration::Declared { .. }), Some(FlipOutcome::MutationSurvived))
            if assertion.is_some() =>
        {
            LedgerStatus::MutationSurvived
        }
        (Some(MutationDeclaration::Declared { .. }) | None, _) => LedgerStatus::Unpinned,
    }
}

/// Every spec AC materialised, in spec order (AC-333: an AC with no evidence is an `unpinned` row).
#[must_use]
pub fn compute_ledger(
    spec: &[AcceptanceCriterion],
    runs: &BTreeMap<String, FlipOutcome>,
    assertions: &BTreeMap<String, AssertionLocator>,
    feasibility: &MutationFeasibility,
    answers: &[HumanAnswer],
) -> AcLedger {
    let rows = spec
        .iter()
        .map(|ac| {
            let answer = answers.iter().find(|a| a.ac == ac.id);
            let assertion = assertions.get(&ac.id);
            let status = ac_status(ac, runs.get(&ac.id), assertion, feasibility, answer);
            let evidence = match status {
                LedgerStatus::Pinned | LedgerStatus::MutationSurvived => assertion.cloned(),
                LedgerStatus::Unpinned
                | LedgerStatus::NotFeasible { .. }
                | LedgerStatus::HumanAnswered { .. } => None,
            };
            AcLedgerRow {
                ac: ac.id.clone(),
                status,
                evidence,
            }
        })
        .collect();
    AcLedger { rows }
}
