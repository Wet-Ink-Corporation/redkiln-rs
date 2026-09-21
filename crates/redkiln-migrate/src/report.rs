//! The differential report (AC-448, AC-467, AC-468; `s7d` §7d.11 step (4), §7d.13).
//!
//! # Why there is exactly one row per kind, and why `slot` reads `body`
//!
//! AC-448 asks for *"per-(kind, slot) counts of lifted / lifted-with-loss / failed"*. **What a
//! `slot` is, v0 does not decide** — the predecessor's S-MIG census carries real slot names
//! (`frontmatter`, `gate_table`, `boundary`, `merge_gate`, `gates`, `tdd`), and a record fixing the
//! schema as the migration's contract is OWED (DECISIONS RT-28).
//!
//! What v0 *is* pinned to is arithmetic. Checkpoint 7d asserts, summed over `slot`, that the
//! `report` rows read `(lifted 0, with-loss 1, failed 0)` and the `spec` rows `(1, 1, 0)`, over a
//! fixture carrying **one** `report.md` and **two** `spec.md` (§7d.13). Each kind's counts therefore
//! sum to its file count, so **one file yields exactly one outcome** and a kind cannot be split
//! across several slots: `spec` across two slots could sum to `lifted 2`, which AC-468's
//! folded-form check reads as a fold and refuses.
//!
//! So v0 emits **one row per kind** with `slot = "body"` — an explicit statement that this version
//! does not decompose a file, rather than a duplicate of `kind` that says nothing. Phase 2 widens
//! it when the corpus differential needs real slots.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The one slot v0 emits. See the module header: v0 does not decompose a file.
pub const SLOT_BODY: &str = "body";

/// The committed schema the report is validated against before it is written (AC-448).
///
/// Compiled in rather than read at runtime: the report is validated inside the migration run,
/// whose working directory is the *target* repository, which carries no copy of this file.
pub const REPORT_SCHEMA: &str = include_str!("../schema/report.schema.json");

/// The workspace's corpus pin, compiled in.
///
/// **A file read at build time, never an environment read** — `env!` is a compile-time macro and is
/// not `std::env::var`, which AC-474 forbids this crate entirely. The target repository has no
/// `CORPUS_PIN` of its own, so a runtime read is not merely discouraged but impossible; the pin the
/// report names is the pin *this tool* was built against, which is what AC-467 asks for.
const CORPUS_PIN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../CORPUS_PIN"));

/// One `(kind, slot)` row. Three distinct counts, and **no `total`** (AC-468).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    pub kind: String,
    pub slot: String,
    pub lifted: u64,
    pub lifted_with_loss: u64,
    pub failed: u64,
}

/// The report document. Its serialised form is the whole of `--dry-run`'s stdout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub corpus_sha: String,
    pub tool_version: String,
    pub per_kind_slot: Vec<Row>,
}

/// How one file came through the lift. Exactly one of these per file read (see the module header).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Carried whole, with nothing dropped.
    Lifted,
    /// Carried, with a migration-loss marker naming what had no home (AC-475's carry, AC-477's).
    LiftedWithLoss,
    /// Not carried.
    Failed,
}

/// Why a report could not be built or did not conform.
#[derive(Debug)]
pub enum ReportError {
    /// `CORPUS_PIN` carries no `CORPUS_SHA=` line, or one that is not 40 hex characters.
    CorpusPinUnreadable { detail: String },
    /// `schema/report.schema.json` is not a compilable JSON Schema.
    SchemaUnusable { detail: String },
    /// The report does not validate against the committed schema.
    DoesNotValidate { errors: Vec<String> },
    /// The report could not be serialised.
    NotSerialisable { detail: String },
}

/// The value of `CORPUS_PIN`'s `CORPUS_SHA=` line.
///
/// A `KEY=value` text file; the precedent parser is `rk-cli`'s `verbs/lint.rs`.
///
/// # Errors
/// [`ReportError::CorpusPinUnreadable`] if no `CORPUS_SHA=` line is present or its value is not
/// forty lowercase hex characters — a report naming an unreadable pin compares every future record
/// against nothing.
pub fn corpus_sha() -> Result<&'static str, ReportError> {
    let found = CORPUS_PIN
        .lines()
        .filter_map(|l| l.strip_prefix("CORPUS_SHA="))
        .map(str::trim)
        .find(|v| !v.is_empty())
        .ok_or_else(|| ReportError::CorpusPinUnreadable {
            detail: "CORPUS_PIN carries no non-empty CORPUS_SHA= line".to_owned(),
        })?;
    if found.len() != 40
        || !found
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err(ReportError::CorpusPinUnreadable {
            detail: format!("CORPUS_SHA={found} is not forty lowercase hex characters"),
        });
    }
    Ok(found)
}

/// This tool's version, for AC-467's *"the lifter/tool version"*.
#[must_use]
pub const fn tool_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Build the report from the per-file outcomes, one row per kind, `slot = "body"`.
///
/// Rows are ordered by kind so the document is byte-stable across two runs — AC-449 compares report
/// digests, and a `BTreeMap` is what makes the hash-seed arm of its sweep moot.
///
/// # Errors
/// [`ReportError::CorpusPinUnreadable`] by way of [`corpus_sha`].
pub fn build(outcomes: &BTreeMap<String, Vec<Outcome>>) -> Result<Report, ReportError> {
    // A BTreeMap, so the row order is the kinds' own order and two runs produce byte-identical
    // documents whatever order the filesystem listed them in (AC-449's report-digest half).
    let per_kind_slot = outcomes
        .iter()
        .map(|(kind, outcomes)| {
            let mut row = Row {
                kind: kind.clone(),
                slot: SLOT_BODY.to_owned(),
                lifted: 0,
                lifted_with_loss: 0,
                failed: 0,
            };
            for outcome in outcomes {
                match outcome {
                    Outcome::Lifted => row.lifted += 1,
                    Outcome::LiftedWithLoss => row.lifted_with_loss += 1,
                    Outcome::Failed => row.failed += 1,
                }
            }
            row
        })
        .collect();

    Ok(Report {
        corpus_sha: corpus_sha()?.to_owned(),
        tool_version: tool_version().to_owned(),
        per_kind_slot,
    })
}

/// Validate a report against the committed schema, in process, through `jsonschema` (AC-448).
///
/// # Errors
/// [`ReportError::SchemaUnusable`] if the committed schema will not compile;
/// [`ReportError::DoesNotValidate`] with every error the validator reported.
pub fn validate(report: &Report) -> Result<(), ReportError> {
    let schema: serde_json::Value =
        serde_json::from_str(REPORT_SCHEMA).map_err(|e| ReportError::SchemaUnusable {
            detail: format!("schema/report.schema.json is not JSON: {e}"),
        })?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|e| ReportError::SchemaUnusable {
            detail: format!("schema/report.schema.json will not compile: {e}"),
        })?;
    let instance = serde_json::to_value(report).map_err(|e| ReportError::NotSerialisable {
        detail: e.to_string(),
    })?;
    let errors: Vec<String> = validator
        .iter_errors(&instance)
        .map(|e| e.to_string())
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ReportError::DoesNotValidate { errors })
    }
}
