//! The two ledger kinds, which the predecessor spells differently and the type keeps distinct.
//!
//! `_ledger.md` is the **sidecar** ([`rk_types::companion::CompanionKind::LedgerCompanion`], wire
//! name `_ledger`), a fenced-yaml block beside an item. `ledger.md` is the census's **bare** kind
//! ([`rk_types::companion::CompanionKind::Ledger`], wire name `ledger`), whose typed form lives at
//! [`rk_types::artifact::ledger::AcLedger`] — the census kind's type lives under `artifact::ledger`
//! and is imported privately by `companion.rs` [measured 2026-09-21].
//!
//! DECISIONS-2026-09-11 #3 words the set as *"five plus `ledger`"*; the type spells it as six
//! variants because `_ledger` and `ledger` are distinct wire names. Noted, not relitigated.

use std::path::Path;

use super::LegacyError;

/// The fenced-yaml block of a legacy `_ledger.md` or `ledger.md`, as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyLedger {
    /// The rows of the fenced block, each an unparsed key/value map.
    pub rows: Vec<serde_json::Value>,
    /// Prose outside the fence, if any.
    pub prose: String,
}

/// Read a legacy ledger file of either kind.
///
/// # Errors
/// [`LegacyError::Malformed`] if the fence is absent or unterminated or its block is not valid
/// YAML.
pub fn parse(path: &Path, src: &str) -> Result<LegacyLedger, LegacyError> {
    let _ = (path, src);
    todo!("cycle 2: read the fenced yaml block and keep the prose beside it")
}

/// Lift a bare-`ledger` census file into its typed form.
///
/// # Errors
/// [`LegacyError::Malformed`] if a row does not satisfy
/// [`rk_types::artifact::ledger::AcLedgerRow::validate`].
pub fn to_ac_ledger(
    path: &Path,
    legacy: &LegacyLedger,
) -> Result<rk_types::artifact::ledger::AcLedger, LegacyError> {
    let _ = (path, legacy);
    todo!("cycle 2: map the legacy rows onto AcLedgerRow and validate each")
}
