//! The legacy readers — **the only `yaml_serde` users in this workspace** (ADR-0100 decision 1)
//! and the only permitted site of the literals `template_sig` and `rendered_sig` (AC-442).
//!
//! Everything under this module **reads**. Nothing here emits an old format: AC-474's clean break
//! means there is no compatibility path out, and `ci.yml`'s AC-442 lint plus
//! `ac442_legacy_readers_only_under_legacy_module` both assert that every hit for those literals
//! across `crates/*/src/**/*.rs` lies under this directory.
//!
//! The lint's scope is **`src/` only, never `tests/`** — `rk-cli`'s
//! `tests/fixtures/lint-controls/ac442_template_sig_control.rs` carries `template_sig` by design,
//! and this crate's own `tests/it/` names both literals in order to assert their absence. A scan
//! over all of `crates/` is red on a correct tree.

pub mod config_yaml;
pub mod frontmatter;
pub mod ledger;
pub mod review;

use std::path::PathBuf;

/// Why one legacy file could not be read.
///
/// A [`LegacyError`] is a **per-file** outcome — it becomes a `failed` count in the report, never a
/// reason to abandon the run. A migration that stopped at the first unreadable file could not
/// produce the differential report whose whole purpose is to say what did not come through.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegacyError {
    /// The file is not valid YAML, or its frontmatter fence is unterminated.
    Malformed { path: PathBuf, detail: String },
    /// A required key of the predecessor's own key set is absent.
    MissingKey { path: PathBuf, key: String },
    /// The file could not be read from disk.
    Unreadable { path: PathBuf, detail: String },
}

/// A part of a legacy file that has no home in the Phase-1 schema and is carried as opaque prose
/// beside a marker naming it (AC-475's carry, AC-477's).
///
/// **The two signature literals never appear here.** Dropping `template_sig` and `rendered_sig` is
/// the clean break AC-471 and AC-474 describe, not a migration loss — see [`crate::lift`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LossMarker {
    /// The legacy key or section that had no home.
    pub what: String,
    /// Why it had none, in the record's voice.
    pub why: String,
    /// The bytes as read, carried verbatim rather than parsed.
    pub opaque: String,
}
