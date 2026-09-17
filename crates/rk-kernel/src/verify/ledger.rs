//! **The AC evidence ledger's computation** (AC-333, AC-335, AC-337, AC-342, AC-345): one closed
//! status per spec AC, from four inputs and no fifth.
//!
//! | input | who supplies it |
//! | --- | --- |
//! | the spec's ACs — id, the assertion the author claims pins it, the author-declared mutation or its `not-feasible(reason)` (ADR-0104 decision 1) | [`SpecAc`], read from the fixture spec (a kernel-local shape, decided 2026-09-17 by the operator: the full `rk-types` spec type is the migration's, 7d) |
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
use rk_types::artifact::ledger::{AcLedger, AcLedgerRow, AssertionLocator, LedgerStatus};
use rk_types::repro::Mutation;
use rk_types::text::Text;
use rk_types::time::Timestamp;

use super::flip::FlipOutcome;
use crate::toml_subset::{self, Value};

/// What the author declared about an AC's mutation (ADR-0104 decision 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationDeclaration {
    /// `mutation: { patch, suite }` — a reverted-fix patch or one carrying a discriminating
    /// assertion locator (ADR-0096).
    Declared(Mutation),
    /// `mutation: not-feasible(reason)` (AC-337).
    NotFeasible { reason: String },
    /// The author declared nothing.
    None,
}

/// One AC of the fixture spec, as the verify gate reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecAc {
    pub id: String,
    /// The verification the author points at, or `None` — which the TYPE accepts (AC-455: the
    /// policy decides whether that blocks a stage).
    pub verification: Option<String>,
    pub mutation: MutationDeclaration,
    /// The assertion the author claims pins this AC — file, test, assertion (AC-333).
    pub assertion: Option<AssertionLocator>,
}

/// Why a fixture spec did not load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecError {
    Toml { source: String, error: String },
    Shape { source: String, detail: String },
}

impl std::fmt::Display for SpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml { source, error } => write!(f, "{source}: {error}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
        }
    }
}

impl std::error::Error for SpecError {}

impl SpecAc {
    /// Read a fixture spec: `[[ac]] id, verification?, [ac.assertion]?, [ac.mutation]?` where the
    /// mutation table is `{ patch, suite }` or `{ not_feasible = "<reason>" }`.
    ///
    /// # Errors
    ///
    /// [`SpecError`] for a file outside the TOML subset or a malformed AC.
    pub fn from_toml(name: &str, src: &str) -> Result<Vec<Self>, SpecError> {
        let doc = toml_subset::parse(src).map_err(|e| SpecError::Toml {
            source: name.to_owned(),
            error: e.to_string(),
        })?;
        let shape = |detail: String| SpecError::Shape {
            source: name.to_owned(),
            detail,
        };
        let Some(Value::Array(acs)) = doc.get("ac") else {
            return Err(shape("no [[ac]] array".to_owned()));
        };
        let mut out = Vec::new();
        for (i, ac) in acs.iter().enumerate() {
            let t = ac
                .as_table()
                .ok_or_else(|| shape(format!("[[ac]] {i} is not a table")))?;
            let id = t
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| shape(format!("[[ac]] {i} has no id")))?
                .to_owned();
            let verification = t
                .get("verification")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned);
            let assertion = match t.get("assertion").and_then(Value::as_table) {
                None => None,
                Some(a) => {
                    let member = |k: &str| {
                        a.get(k)
                            .and_then(Value::as_str)
                            .map(ToOwned::to_owned)
                            .ok_or_else(|| shape(format!("{id}: assertion has no `{k}`")))
                    };
                    Some(AssertionLocator {
                        file: member("file")?,
                        test: member("test")?,
                        assertion: member("assertion")?,
                    })
                }
            };
            let mutation = match t.get("mutation").and_then(Value::as_table) {
                None => MutationDeclaration::None,
                Some(m) => {
                    if let Some(reason) = m.get("not_feasible").and_then(Value::as_str) {
                        MutationDeclaration::NotFeasible {
                            reason: reason.to_owned(),
                        }
                    } else {
                        let member = |k: &str| {
                            m.get(k)
                                .and_then(Value::as_str)
                                .map(ToOwned::to_owned)
                                .ok_or_else(|| shape(format!("{id}: mutation has no `{k}`")))
                        };
                        MutationDeclaration::Declared(Mutation {
                            patch: member("patch")?,
                            suite: member("suite")?,
                        })
                    }
                }
            };
            out.push(Self {
                id,
                verification,
                mutation,
                assertion,
            });
        }
        Ok(out)
    }
}

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
    ac: &SpecAc,
    run: Option<&FlipOutcome>,
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
        (MutationDeclaration::NotFeasible { reason }, _) => LedgerStatus::NotFeasible {
            reason: Text::new(reason.clone()),
        },
        (MutationDeclaration::Declared(_), Some(FlipOutcome::Pinned)) if ac.assertion.is_some() => {
            LedgerStatus::Pinned
        }
        (MutationDeclaration::Declared(_), Some(FlipOutcome::MutationSurvived))
            if ac.assertion.is_some() =>
        {
            LedgerStatus::MutationSurvived
        }
        (MutationDeclaration::Declared(_) | MutationDeclaration::None, _) => LedgerStatus::Unpinned,
    }
}

/// Every spec AC materialised, in spec order (AC-333: an AC with no evidence is an `unpinned` row).
#[must_use]
pub fn compute_ledger(
    spec: &[SpecAc],
    runs: &BTreeMap<String, FlipOutcome>,
    feasibility: &MutationFeasibility,
    answers: &[HumanAnswer],
) -> AcLedger {
    let rows = spec
        .iter()
        .map(|ac| {
            let answer = answers.iter().find(|a| a.ac == ac.id);
            let status = ac_status(ac, runs.get(&ac.id), feasibility, answer);
            let evidence = match status {
                LedgerStatus::Pinned | LedgerStatus::MutationSurvived => ac.assertion.clone(),
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
