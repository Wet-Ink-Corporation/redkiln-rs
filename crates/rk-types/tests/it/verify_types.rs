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

/// A complete `verify_completed` payload (ADR-0191 decision 8): D21-42 10b's four members plus
/// AC-321's three, and `origin` and `stage` (D21-47 review, finding 4).
fn verify_completed_payload() -> serde_json::Value {
    serde_json::json!({
        "report_digest": "0".repeat(64),
        "selection": { "kind": "whole_suite" },
        "basis": "whole-suite",
        "basis_reason": "policy-forced",
        "passed": 3,
        "failed": 0,
        "selected_test_count": 3,
        "adapter": "rust",
        "range": { "base": "a".repeat(40), "head": "b".repeat(40) },
        "paths": ["crates/x/src/lib.rs"],
        "toolchain_version": "rustc 1.90.0",
        "origin": "pass",
        "stage": "verify"
    })
}

/// **`verify_completed`'s payload refuses an absent member** (`s8g` §8g.4; ADR-0191 decision 8,
/// proposed). AC-321 reads the range endpoints, the path list and the toolchain version back
/// *"from the run event verbatim"*, so each member is required at the boundary: an absent one is a
/// refusal naming it, never a default. `basis_reason` is required-present and may be `null` (an
/// `affected` run carries none). The interim `item` key rides beside the payload on ADR-0182's
/// carrier, and the runner rides on the report (D21-48 X15), so a payload carrying either member
/// is refused too. *(Added 2026-09-29, `s8g` build, package S8G-P2-RKTYPES.)*
#[test]
fn verify_completed_payload_refuses_an_absent_member() {
    use rk_types::verify::VerifyCompleted;

    let full = verify_completed_payload();
    let row = VerifyCompleted::from_payload(&full).expect("the complete payload parses");
    assert_eq!(
        row.to_payload(),
        full,
        "the payload round-trips byte for byte"
    );

    let members = full.as_object().expect("an object").clone();
    assert_eq!(
        members.len(),
        13,
        "ADR-0191 decision 8 names thirteen members"
    );
    for name in members.keys() {
        let mut spoiled = full.clone();
        spoiled.as_object_mut().unwrap().remove(name);
        let err = VerifyCompleted::from_payload(&spoiled)
            .expect_err(&format!("a payload without `{name}` must be refused"));
        assert!(
            err.to_string().contains(name.as_str()),
            "the refusal names the absent member `{name}`: {err}"
        );
    }

    // `null` is `basis_reason`'s affected form, and only that member's.
    let mut affected = full.clone();
    affected["basis"] = serde_json::json!("affected");
    affected["basis_reason"] = serde_json::Value::Null;
    let row =
        VerifyCompleted::from_payload(&affected).expect("an affected row carries a null reason");
    assert_eq!(row.basis_reason, None);

    // Beside the payload, never in it: the interim item key (ADR-0182) and the runner (X15).
    for extra in ["item", "runner"] {
        let mut spoiled = full.clone();
        spoiled[extra] = serde_json::json!("x");
        assert!(
            VerifyCompleted::from_payload(&spoiled).is_err(),
            "a `{extra}` member inside the payload must be refused"
        );
    }
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
