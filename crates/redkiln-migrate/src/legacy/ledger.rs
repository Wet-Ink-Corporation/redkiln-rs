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
    let fenced = super::fence::split(path, src)?;
    let block: serde_json::Value =
        yaml_serde::from_str(&fenced.block).map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("the fenced ledger block is not valid YAML: {e}"),
        })?;
    let rows = block
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(LegacyLedger {
        rows,
        prose: fenced.prose,
    })
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
    // THE ONE REAL TRANSLATION IN THIS READER, and the reason it is not a bare `from_value`.
    //
    // The predecessor writes a row's status as a BARE STRING (`status: pinned`).
    // `rk_types::artifact::ledger::LedgerStatus` is an internally tagged enum
    // (`#[serde(tag = "kind", rename_all = "snake_case")]`) [measured 2026-09-21], so its wire form
    // is `status: { kind: pinned }`, and two of its five variants carry a `reason` beside the tag.
    // Converting between those two spellings is exactly what this tool exists to do; leaving the
    // legacy form alone would put every ledger row in the report's `failed` column.
    let rows: Vec<serde_json::Value> = legacy
        .rows
        .iter()
        .map(|row| {
            let mut row = row.clone();
            let Some(obj) = row.as_object_mut() else {
                return row;
            };
            if let Some(status) = obj.get("status").and_then(serde_json::Value::as_str) {
                let mut tagged = serde_json::Map::new();
                tagged.insert(
                    "kind".to_owned(),
                    serde_json::Value::String(status.to_owned()),
                );
                // `not_feasible` and `human_answered` carry a reason; the predecessor keeps it as a
                // sibling key, the type wants it inside the tagged object.
                if let Some(reason) = obj.remove("reason") {
                    tagged.insert("reason".to_owned(), reason);
                }
                obj.insert("status".to_owned(), serde_json::Value::Object(tagged));
            }
            row
        })
        .collect();

    let value = serde_json::json!({ "rows": rows });
    let ledger: rk_types::artifact::ledger::AcLedger =
        serde_json::from_value(value).map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("the ledger rows do not read as AcLedger: {e}"),
        })?;
    for row in &ledger.rows {
        row.validate().map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("ledger row {}: {e}", row.ac),
        })?;
    }
    Ok(ledger)
}
