//! **AC-333's completeness check** over the persisted AC evidence ledger: every AC id in the spec
//! is materialised, none twice, none the spec does not have — and every row valid on its own
//! terms (`rk_types::artifact::ledger::AcLedgerRow::validate`: a file-path-only locator fails).
//!
//! An AC with no evidence is an `unpinned` ROW here, never an omission: the check that fails on a
//! shorter ledger names the missing id, so a ledger that dropped an AC cannot pass by being
//! shorter. The ledger's computation is `rk-kernel`'s verify module; this reads the result.

use std::collections::BTreeSet;

use rk_types::artifact::ledger::{AcLedger, LedgerRowError};

/// How the ledger failed against the spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    Row(LedgerRowError),
    /// Spec AC ids with no row. Named, so the failure says which AC vanished.
    Missing {
        acs: Vec<String>,
    },
    /// Rows for ids the spec does not carry.
    Unknown {
        acs: Vec<String>,
    },
    /// The same id twice.
    Duplicate {
        ac: String,
    },
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(e) => write!(f, "{e}"),
            Self::Missing { acs } => write!(
                f,
                "the ledger materialises no row for {} — an AC with no evidence is an `unpinned` \
                 row, never an omission (AC-333)",
                acs.join(", ")
            ),
            Self::Unknown { acs } => {
                write!(
                    f,
                    "the ledger carries rows the spec has no AC for: {}",
                    acs.join(", ")
                )
            }
            Self::Duplicate { ac } => write!(f, "{ac} appears twice in the ledger"),
        }
    }
}

impl std::error::Error for LedgerError {}

/// Validate every row and assert the ledger's AC id set EQUALS the spec's.
///
/// # Errors
///
/// [`LedgerError`], the first found in the order: row invariants, duplicates, missing, unknown.
pub fn validate_ledger(ledger: &AcLedger, spec_ac_ids: &[String]) -> Result<(), LedgerError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for row in &ledger.rows {
        row.validate().map_err(LedgerError::Row)?;
        if !seen.insert(row.ac.as_str()) {
            return Err(LedgerError::Duplicate { ac: row.ac.clone() });
        }
    }
    let spec: BTreeSet<&str> = spec_ac_ids.iter().map(String::as_str).collect();
    let missing: Vec<String> = spec.difference(&seen).map(|s| (*s).to_owned()).collect();
    if !missing.is_empty() {
        return Err(LedgerError::Missing { acs: missing });
    }
    let unknown: Vec<String> = seen.difference(&spec).map(|s| (*s).to_owned()).collect();
    if !unknown.is_empty() {
        return Err(LedgerError::Unknown { acs: unknown });
    }
    Ok(())
}
