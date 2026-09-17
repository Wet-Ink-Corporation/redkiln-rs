//! **AC-329's field semantics** — the whole-suite comparator record and the two meanings of
//! `total_test_count` (ADR-0104 decision 3).
//!
//! The record is a pure fold over recorded reports: the **most recent** `whole-suite` report at a
//! toolchain version seeds [`WholeSuiteRecord`] — its count and the run it came from. On an
//! `affected` run the report's `total_test_count` is that count and `total_test_count_source`
//! names that run; on the bootstrap run (no record yet) it is the run's own executed count. A pin
//! bump is a different `toolchain_version` and therefore a different — empty — record: the reset
//! AC-326 requires is a property of the key, not a step.
//!
//! What this module does NOT do: reclassify a claim (that emits `selection_ineffective` and is
//! `rk-kernel`'s, AC-326), or read anything but `rk_types::verify::VerifyReport` values.

use rk_types::verify::{Basis, TotalTestCountSource, VerifyReport, VerifyReportShapeError};

/// The comparator's record at one toolchain version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WholeSuiteRecord {
    pub toolchain_version: String,
    pub count: u32,
    /// The run the count came from — what an `affected` report's `total_test_count_source` names.
    pub run_id: String,
}

/// The most recent recorded whole-suite count at `toolchain_version`, in the order given (the
/// caller supplies reports oldest first). `None` is the bootstrap condition.
pub fn latest_whole_suite<'a>(
    reports: impl Iterator<Item = (&'a str, &'a VerifyReport)>,
    toolchain_version: &str,
) -> Option<WholeSuiteRecord> {
    reports
        .filter(|(_, r)| r.basis == Basis::WholeSuite && r.toolchain_version == toolchain_version)
        .last()
        .map(|(run_id, r)| WholeSuiteRecord {
            toolchain_version: r.toolchain_version.clone(),
            count: r.selected_test_count,
            run_id: run_id.to_owned(),
        })
}

/// Why `total_test_count` could not be filled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TotalTestCountError {
    /// An `affected` claim with no recorded whole-suite count at the pin. ADR-0104's forbids:
    /// *"reporting `basis: affected` with no recorded whole-suite count at the current pin"* — the
    /// kernel reclassifies such a claim to `whole-suite / probe-unverified` before a report exists.
    AffectedWithoutRecord,
}

impl std::fmt::Display for TotalTestCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AffectedWithoutRecord => write!(
                f,
                "an affected run with no recorded whole-suite count at this pin — reclassify to \
                 whole-suite / probe-unverified first (ADR-0104 decision 3)"
            ),
        }
    }
}

impl std::error::Error for TotalTestCountError {}

/// `total_test_count` and its source for a run of `basis` that executed `executed` tests.
///
/// # Errors
///
/// [`TotalTestCountError::AffectedWithoutRecord`].
pub fn total_test_count_fields(
    basis: Basis,
    executed: u32,
    recorded: Option<&WholeSuiteRecord>,
) -> Result<(u32, TotalTestCountSource), TotalTestCountError> {
    match (basis, recorded) {
        (Basis::Affected, Some(rec)) => Ok((
            rec.count,
            TotalTestCountSource::RecordedRun {
                run_id: rec.run_id.clone(),
            },
        )),
        (Basis::Affected, None) => Err(TotalTestCountError::AffectedWithoutRecord),
        (Basis::WholeSuite, _) => Ok((executed, TotalTestCountSource::OwnExecutedCount)),
    }
}

/// How a report disagreed with its own shape or with the record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyReportError {
    Shape(VerifyReportShapeError),
    /// The report's `total_test_count` / source pair is not what the record and basis dictate.
    TotalTestCount {
        basis: Basis,
        want: (u32, TotalTestCountSource),
        got: (u32, TotalTestCountSource),
    },
    Comparator(TotalTestCountError),
}

impl std::fmt::Display for VerifyReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape(e) => write!(f, "{e}"),
            Self::TotalTestCount { basis, want, got } => write!(
                f,
                "on a `{}` run total_test_count must be {} from {:?}; the report carries {} from {:?} \
                 (AC-329; ADR-0104 decision 3)",
                basis.as_str(),
                want.0,
                want.1,
                got.0,
                got.1
            ),
            Self::Comparator(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for VerifyReportError {}

/// Validate a report's shape and its `total_test_count` semantics against the comparator record.
///
/// # Errors
///
/// [`VerifyReportError`] naming the member and both values.
pub fn validate_report(
    report: &VerifyReport,
    recorded: Option<&WholeSuiteRecord>,
) -> Result<(), VerifyReportError> {
    report.validate_shape().map_err(VerifyReportError::Shape)?;
    let want = total_test_count_fields(report.basis, report.selected_test_count, recorded)
        .map_err(VerifyReportError::Comparator)?;
    let got = (
        report.total_test_count,
        report.total_test_count_source.clone(),
    );
    if want == got {
        Ok(())
    } else {
        Err(VerifyReportError::TotalTestCount {
            basis: report.basis,
            want,
            got,
        })
    }
}
