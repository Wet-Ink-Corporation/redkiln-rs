//! The per-AC ledger — the `ledger` companion kind's sidecar (AC-333; 220 records at the pin).
//!
//! **An AC is a claim about ONE ASSERTION** (F25): a row that pins an AC names the file, the test
//! and the assertion within that test — [`AssertionLocator`] — and a row naming only a file fails
//! validation ([`AcLedgerRow::validate`]). The status is one closed enum per AC ([`LedgerStatus`]),
//! and `unpinned` is the reserved name for *no evidence*: an AC with nothing behind it is a row
//! that says so, never an omission (the completeness check against the spec's id set is
//! `rk-store`'s, beside the persisted ledger).
//!
//! What a row may NOT carry, on purpose: the item's suite verdict (AC-335 — a green suite changes
//! nothing about a survivor), a review finding (AC-342), a quarantine (AC-345 — a review-gate
//! state; ADR-0104 decision 4). The ledger is computed from declared mutations, flip outcomes and
//! human answers, in `rk-kernel`'s verify module, and this type is what it writes.

use crate::text::Text;
use serde::{Deserialize, Serialize};

/// One closed status per AC. `Unpinned` is RESERVED and explicit: an AC with no discriminating
/// test is *unpinned*, which is a state to be named, not an absence to be inferred
/// (DECISIONS-2026-09-11, PIA-032/033).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LedgerStatus {
    /// The AC's named assertion flips under its declared mutation (AC-334's predicate).
    Pinned,
    /// The suite stayed green under the mutation. A survivor blocks exit even when quarantined
    /// (ADR-0104 decision 4).
    MutationSurvived,
    /// No discriminating test. Reserved, and never a default.
    Unpinned,
    /// The toolchain pack declares mutation infeasible (AC-337).
    NotFeasible { reason: Text },
    /// A human gate answered THAT AC explicitly (AC-357).
    HumanAnswered { reason: Text },
}

impl LedgerStatus {
    /// The wire spelling — the `kind` tag as serialised.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Pinned => "pinned",
            Self::MutationSurvived => "mutation_survived",
            Self::Unpinned => "unpinned",
            Self::NotFeasible { .. } => "not_feasible",
            Self::HumanAnswered { .. } => "human_answered",
        }
    }

    /// Does this status let the stage exit for its AC (AC-346)? Only a flip-verified pin or a
    /// human answer for THAT AC does; a survivor, an unpinned AC and an infeasible declaration all
    /// refuse (AC-337: `not-feasible` *"is not `pinned`"*; AC-357 routes it to a human).
    #[must_use]
    pub const fn permits_exit(&self) -> bool {
        matches!(self, Self::Pinned | Self::HumanAnswered { .. })
    }
}

/// Where the evidence for a row lives: **file, test identifier and the assertion within that
/// test** (AC-333). Three members, all required, none empty — a file alone is the shape F25
/// measured as vacuous (36/36 green at file granularity over an inverted ordering).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AssertionLocator {
    pub file: String,
    pub test: String,
    pub assertion: String,
}

/// Why a row failed validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerRowError {
    /// The locator names a file and nothing else (or an empty test / assertion).
    FilePathOnly { ac: String, missing: &'static str },
    /// A `pinned` or `mutation_survived` row carries no locator at all.
    EvidenceRequired { ac: String, status: &'static str },
    /// An `unpinned` row carries a locator — *no evidence* is what the status means.
    UnpinnedWithEvidence { ac: String },
    /// The `ac` member is empty.
    EmptyAc,
}

impl std::fmt::Display for LedgerRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FilePathOnly { ac, missing } => write!(
                f,
                "{ac}: the evidence names a file but no {missing} — an AC is a claim about one \
                 assertion, not one file (AC-333; F25)"
            ),
            Self::EvidenceRequired { ac, status } => {
                write!(
                    f,
                    "{ac}: a `{status}` row must name the assertion it rests on"
                )
            }
            Self::UnpinnedWithEvidence { ac } => write!(
                f,
                "{ac}: `unpinned` means no evidence; a row with a locator is not unpinned"
            ),
            Self::EmptyAc => write!(f, "a ledger row with no AC id"),
        }
    }
}

impl std::error::Error for LedgerRowError {}

/// One AC's row. `evidence` is declared last because it is optional (AC-158's ordering rule) and
/// optional only because `unpinned`, `not_feasible` and `human_answered` rows have none to give.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcLedgerRow {
    pub ac: String,
    pub status: LedgerStatus,
    #[serde(default)]
    pub evidence: Option<AssertionLocator>,
}

impl AcLedgerRow {
    /// The row's own invariants: a pin or a survivor names its assertion in full; `unpinned`
    /// carries nothing.
    ///
    /// # Errors
    ///
    /// [`LedgerRowError`] naming the row and what is missing or surplus.
    pub fn validate(&self) -> Result<(), LedgerRowError> {
        if self.ac.trim().is_empty() {
            return Err(LedgerRowError::EmptyAc);
        }
        match (&self.status, &self.evidence) {
            (LedgerStatus::Pinned | LedgerStatus::MutationSurvived, None) => {
                Err(LedgerRowError::EvidenceRequired {
                    ac: self.ac.clone(),
                    status: self.status.as_str(),
                })
            }
            (LedgerStatus::Unpinned, Some(_)) => Err(LedgerRowError::UnpinnedWithEvidence {
                ac: self.ac.clone(),
            }),
            (_, Some(locator)) => {
                let missing = if locator.file.trim().is_empty() {
                    Some("file")
                } else if locator.test.trim().is_empty() {
                    Some("test identifier")
                } else if locator.assertion.trim().is_empty() {
                    Some("assertion")
                } else {
                    None
                };
                missing.map_or(Ok(()), |missing| {
                    Err(LedgerRowError::FilePathOnly {
                        ac: self.ac.clone(),
                        missing,
                    })
                })
            }
            (_, None) => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcLedger {
    pub rows: Vec<AcLedgerRow>,
}
