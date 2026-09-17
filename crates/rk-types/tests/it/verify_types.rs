//! The verify gate's `rk-types` half (Step 7b-ii, `s7b-ii` §7b-ii.9): the closed `basis` /
//! `basis_reason` pair (AC-352), the report with no `Default` (AC-324), and the ledger row's
//! locator (AC-333). The named tests for AC-329 and AC-333 run in `rk-store`, the register's
//! subject crate; these pin the type-level facts they stand on.

use rk_types::artifact::ledger::{AcLedgerRow, AssertionLocator, LedgerStatus};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

/// AC-352: six reasons, kebab-case on the wire, matched exhaustively wherever read.
#[test]
fn basis_reason_is_the_closed_six() {
    assert_eq!(BasisReason::ALL.len(), 6);
    let wire: Vec<&str> = BasisReason::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        wire,
        [
            "no-selection-support",
            "probe-unverified",
            "selection-empty",
            "selection-ineffective",
            "policy-forced",
            "toolchain-unsupported",
        ]
    );
    for r in BasisReason::ALL {
        let json = serde_json::to_value(r).unwrap();
        assert_eq!(json, serde_json::Value::from(r.as_str()));
        assert_eq!(serde_json::from_value::<BasisReason>(json).unwrap(), r);
    }
    assert!(
        serde_json::from_value::<BasisReason>(serde_json::json!("selection_ineffective")).is_err(),
        "the wire spelling is kebab-case, and nothing else parses"
    );
    assert_eq!(Basis::WholeSuite.as_str(), "whole-suite");
    assert_eq!(serde_json::to_value(Basis::Affected).unwrap(), "affected");
}

/// AC-324: `Result<VerifyReport, StepError>` at the boundary **with no `Default` impl** — a report
/// cannot be conjured. Asserted over the source, since Rust has no negative trait bound to write.
#[test]
fn verify_report_has_no_default() {
    let src = include_str!("../../src/verify.rs");
    for (i, line) in src.lines().enumerate() {
        let code = line.split("//").next().unwrap_or("");
        assert!(
            !code.contains("Default"),
            "verify.rs:{}: a `Default` on the verify path is the pass-by-absence AC-324 forbids: {line}",
            i + 1
        );
    }
    // and the shape check refuses the empty members AC-324 names
    let r = VerifyReport {
        basis: Basis::WholeSuite,
        argv: vec![],
        selected_test_count: 0,
        total_test_count: 0,
        total_test_count_source: TotalTestCountSource::OwnExecutedCount,
        wall_secs: 0,
        toolchain_version: "x".to_owned(),
        suite_sha: String::new(),
        basis_reason: Some(BasisReason::PolicyForced),
    };
    assert!(
        r.validate_shape().is_err(),
        "an empty argv is a missing command record"
    );
}

/// AC-333: the row's locator round-trips with its three members and `evidence: null` is the
/// unpinned shape.
#[test]
fn ledger_row_locator_round_trips() {
    let row = AcLedgerRow {
        ac: "AC-1".to_owned(),
        status: LedgerStatus::Pinned,
        evidence: Some(AssertionLocator {
            file: "tests/it/x.rs".to_owned(),
            test: "pins_it".to_owned(),
            assertion: "assert_eq!(a, b)".to_owned(),
        }),
    };
    row.validate().unwrap();
    let json = serde_json::to_value(&row).unwrap();
    assert_eq!(json["evidence"]["assertion"], "assert_eq!(a, b)");
    assert_eq!(serde_json::from_value::<AcLedgerRow>(json).unwrap(), row);
    let unpinned: AcLedgerRow =
        serde_json::from_str(r#"{"ac":"AC-2","status":{"kind":"unpinned"}}"#).unwrap();
    assert_eq!(
        unpinned.evidence, None,
        "an absent `evidence` member is `None`"
    );
    unpinned.validate().unwrap();
    assert!(!LedgerStatus::MutationSurvived.permits_exit() && LedgerStatus::Pinned.permits_exit());
}
