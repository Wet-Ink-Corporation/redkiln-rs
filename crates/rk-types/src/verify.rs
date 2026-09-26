//! The verify gate's record type — Step 7b-ii, `s7b-ii` §7b-ii.9.
//!
//! [`VerifyReport`] is what a toolchain adapter hands the kernel (AC-329): *"`basis`,
//! `basis_reason`, `argv`, `selected_test_count`, `total_test_count`, `wall_secs`,
//! `toolchain_version` and `suite_sha`, and validation fails if any is absent"*. Every member is
//! required at the boundary (`deny_unknown_fields`, no `Option` but the fallback reason, and that
//! one is validated rather than optional), and **there is no `Default`** (AC-324: a verify result
//! whose basis, digest or command record is absent resolves to `Err`, never to a pass).
//!
//! `basis` and `basis_reason` are **closed enums** (AC-352): the six reasons are the register's,
//! matched exhaustively wherever they are read so a seventh is a compile error. The two semantics
//! of `total_test_count` (ADR-0104 decision 3) are named by [`TotalTestCountSource`]: on an
//! `affected` run it is the most recent recorded whole-suite count at that toolchain version and
//! the report names the run it came from; on the bootstrap run it is the run's own executed count.
//! The comparison against the record is `rk-store`'s (`artifacts::verify_report`); the
//! reclassification that emits `selection_ineffective` is `rk-kernel`'s (AC-326).
//!
//! No member of this record is obtained by parsing prose or markdown (brief §C.5); the adapter
//! lint that enforces that over the toolchain adapters is `s7c`'s.

use serde::{Deserialize, Serialize};

/// What the run claims it ran. `whole-suite` carries a reason (AC-352); `affected` carries none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Basis {
    Affected,
    WholeSuite,
}

impl Basis {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Affected => "affected",
            Self::WholeSuite => "whole-suite",
        }
    }
}

/// Why a run fell back to the whole suite — AC-352's six, closed, no wildcard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum BasisReason {
    /// The pack declares no affected-selection support.
    NoSelectionSupport,
    /// AC-328's registry entry is absent or stale at the pin, or AC-326's bootstrap: no whole-suite
    /// count recorded yet.
    ProbeUnverified,
    /// A non-empty change set selected zero tests (AC-332).
    SelectionEmpty,
    /// The filter was inert: the executed count met the recorded whole-suite count (AC-326).
    SelectionIneffective,
    /// The pack's policy demanded the whole suite regardless.
    // *(Noted 2026-09-26, s8c build, P5; record T = ADR-0169 decision 7 rule 3, proposed.)* Record T
    // reads this reason as "a declared policy demanded the whole suite regardless", and declares
    // the Rust adapter's rule 3 (a changed path in no workspace package -- the root Cargo.toml,
    // Cargo.lock, rust-toolchain*, .cargo/ or any non-crate path -- forces the whole suite) as
    // the adapter's own policy, because of the six it is the only reason meaning "a rule, not a
    // probe or a count, forced the whole suite". The reading is flagged for the operator's flip;
    // rejecting it mints nothing (AC-352) and picks another of the six by record. This note is a
    // plain comment, not a doc line, so the exported schema's description is unchanged.
    PolicyForced,
    /// The claiming toolchain has no verify adapter.
    ToolchainUnsupported,
}

impl BasisReason {
    pub const ALL: [Self; 6] = [
        Self::NoSelectionSupport,
        Self::ProbeUnverified,
        Self::SelectionEmpty,
        Self::SelectionIneffective,
        Self::PolicyForced,
        Self::ToolchainUnsupported,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSelectionSupport => "no-selection-support",
            Self::ProbeUnverified => "probe-unverified",
            Self::SelectionEmpty => "selection-empty",
            Self::SelectionIneffective => "selection-ineffective",
            Self::PolicyForced => "policy-forced",
            Self::ToolchainUnsupported => "toolchain-unsupported",
        }
    }
}

/// Where `total_test_count` came from (ADR-0104 decision 3; AC-329's `total_test_count_source`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TotalTestCountSource {
    /// The bootstrap run, or any whole-suite run: the count is this run's own executed count.
    OwnExecutedCount,
    /// An `affected` run: the most recent recorded whole-suite count, from the run named here.
    RecordedRun { run_id: String },
}

/// The adapter's report of one verify run. No `Default`; every member required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct VerifyReport {
    pub basis: Basis,
    /// The executed command record — the argv as run, never a rendering of it.
    pub argv: Vec<String>,
    /// The count this run executed under its selection.
    pub selected_test_count: u32,
    /// The whole-suite count this run is measured against; see [`TotalTestCountSource`].
    pub total_test_count: u32,
    pub total_test_count_source: TotalTestCountSource,
    pub wall_secs: u64,
    /// The toolchain's detected version string — the comparator's pin.
    pub toolchain_version: String,
    /// The suite digest.
    pub suite_sha: String,
    /// The fallback reason. Required-present on a `whole-suite` run and refused on an `affected`
    /// one — `null` parses so that the refusal is a validation error naming the rule (AC-352), not
    /// a parse error. Declared last because it is the one member that may be `null` (AC-158).
    #[serde(default)]
    pub basis_reason: Option<BasisReason>,
}

/// What a report's own members got wrong, before any comparison against a record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyReportShapeError {
    /// A `whole-suite` run with no reason (AC-352).
    FallbackWithoutReason,
    /// An `affected` run carrying a fallback reason.
    AffectedWithReason { reason: BasisReason },
    /// An empty executed-command record (AC-324's "missing argv").
    EmptyArgv,
    /// An empty toolchain version, so the comparator has no pin.
    EmptyToolchainVersion,
    /// An empty suite digest (AC-324's "missing digest").
    EmptySuiteSha,
}

impl std::fmt::Display for VerifyReportShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FallbackWithoutReason => {
                write!(f, "a whole-suite fallback with no basis_reason (AC-352)")
            }
            Self::AffectedWithReason { reason } => write!(
                f,
                "an affected run carries the fallback reason `{}`",
                reason.as_str()
            ),
            Self::EmptyArgv => write!(f, "the executed-command record is empty (AC-324)"),
            Self::EmptyToolchainVersion => write!(f, "the toolchain version is empty"),
            Self::EmptySuiteSha => write!(f, "the suite digest is empty (AC-324)"),
        }
    }
}

impl std::error::Error for VerifyReportShapeError {}

impl VerifyReport {
    /// The members' own invariants — what can be checked without a comparator record.
    ///
    /// # Errors
    ///
    /// [`VerifyReportShapeError`] naming the rule.
    pub fn validate_shape(&self) -> Result<(), VerifyReportShapeError> {
        match (self.basis, self.basis_reason) {
            (Basis::WholeSuite, None) => return Err(VerifyReportShapeError::FallbackWithoutReason),
            (Basis::Affected, Some(reason)) => {
                return Err(VerifyReportShapeError::AffectedWithReason { reason });
            }
            (Basis::WholeSuite, Some(_)) | (Basis::Affected, None) => {}
        }
        if self.argv.is_empty() {
            return Err(VerifyReportShapeError::EmptyArgv);
        }
        if self.toolchain_version.trim().is_empty() {
            return Err(VerifyReportShapeError::EmptyToolchainVersion);
        }
        if self.suite_sha.trim().is_empty() {
            return Err(VerifyReportShapeError::EmptySuiteSha);
        }
        Ok(())
    }
}
