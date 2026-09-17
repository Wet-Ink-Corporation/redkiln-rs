//! **`rk-store-artifacts`, the verify gate's half** — Step 7b-ii, `s7b-ii` §7b-ii.9. Two ids whose
//! register subject tag is `rk-store-artifacts`, filtered here by Checkpoint 7b-ii's third loop.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac329_total_test_count_source`] | **AC-329** (the field-semantics half) | the eight members are required — a fixture missing each in turn fails to deserialise; on an `affected` run `total_test_count` is the most recent recorded whole-suite count at that toolchain version and `total_test_count_source` names the run it came from; on the bootstrap run it is the run's own executed count; a report whose fields disagree with the record fails validation | the per-adapter schema sweep and the markdown-parsing lint over the toolchain adapters — `s7c`'s (the split is stated at both ends: `s7b-ii` §7b-ii.9; the `s7c` note is OWED) |
//! | [`ac333_ac_evidence_ledger`] | **AC-333** | the `ledger` companion schema's `kind` constants are EXACTLY the closed five and carry no wildcard; a `pinned` row naming only a file fails validation; every spec AC id is materialised — a ledger missing one fails the completeness check with the id named, an `unpinned` row is the correct output; the six companion kinds include `ledger` | the ledger's computation from mutations and answers — `rk-kernel`'s (AC-334, AC-335) |
//!
//! Red first, type-level where the register reads it so (AC-244's and AC-283's precedent): both
//! tests were written against an `AcLedgerRow` with no locator member and a workspace with no
//! `VerifyReport` type — neither compiled (the absent behaviour IS the absent type). Once the
//! types existed, each was shown red on its own claim against a reverted mutant: AC-333's
//! completeness check with the `missing` test dropped (*"a ledger missing a spec AC must fail the
//! completeness check naming it: Ok(())"*); AC-329's affected rule returning the run's own count
//! (`left: 9 right: 236`). Outputs in the commit message.

use rk_store::artifacts::ledger::{LedgerError, validate_ledger};
use rk_store::artifacts::verify_report::{
    WholeSuiteRecord, latest_whole_suite, total_test_count_fields, validate_report,
};
use rk_types::artifact::ledger::{AcLedger, AcLedgerRow, LedgerRowError, LedgerStatus};
use rk_types::companion::CompanionKind;
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};
use serde_json::{Value, json};

const CLOSED_STATUS_ENUM: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../rk-kernel/tests/fixtures/ledger/ac333_closed_status_enum.json"
));
const FILE_PATH_ONLY_ROW: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../rk-kernel/tests/fixtures/ledger/ac333_file_path_only_row.json"
));
const SPEC_AC_IDS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../rk-kernel/tests/fixtures/ledger/ac333_spec_ac_ids.json"
));
const LEDGER_COMPLETE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../rk-kernel/tests/fixtures/ledger/ac333_ledger_complete.json"
));

const NODE_PIN: &str = "node 22.4.1 / vitest 2.1.0";

fn report(
    basis: Basis,
    reason: Option<BasisReason>,
    executed: u32,
    total: u32,
    source: TotalTestCountSource,
) -> VerifyReport {
    VerifyReport {
        basis,
        argv: vec![
            "pnpm".to_owned(),
            "test".to_owned(),
            "--".to_owned(),
            "--run".to_owned(),
        ],
        selected_test_count: executed,
        total_test_count: total,
        total_test_count_source: source,
        wall_secs: 42,
        toolchain_version: NODE_PIN.to_owned(),
        suite_sha: "0123456789abcdef0123456789abcdef01234567".to_owned(),
        basis_reason: reason,
    }
}

fn valid_json() -> Value {
    json!({
        "basis": "whole-suite",
        "argv": ["pnpm", "test"],
        "selected_test_count": 236,
        "total_test_count": 236,
        "total_test_count_source": { "kind": "own_executed_count" },
        "wall_secs": 251,
        "toolchain_version": NODE_PIN,
        "suite_sha": "0123456789abcdef0123456789abcdef01234567",
        "basis_reason": "probe-unverified"
    })
}

/// **AC-329 — the eight members, and `total_test_count`'s two semantics with its named source.**
#[test]
fn ac329_total_test_count_source() {
    // the control round-trips
    let control: VerifyReport =
        serde_json::from_value(valid_json()).expect("the control deserialises");
    assert_eq!(control.basis, Basis::WholeSuite);
    assert_eq!(control.basis_reason, Some(BasisReason::ProbeUnverified));

    // a negative fixture missing EACH field in turn — every one is required at the boundary
    let members = [
        "basis",
        "argv",
        "selected_test_count",
        "total_test_count",
        "total_test_count_source",
        "wall_secs",
        "toolchain_version",
        "suite_sha",
    ];
    for m in members {
        let mut v = valid_json();
        v.as_object_mut().unwrap().remove(m);
        assert!(
            serde_json::from_value::<VerifyReport>(v).is_err(),
            "a report missing `{m}` must fail to deserialise (AC-329: validation fails if any is absent)"
        );
    }
    // `basis_reason` is the ninth: required-present for a whole-suite fallback (AC-352), refused on an
    // affected run — its absence is a VALIDATION failure, not a parse failure, because `null` is legal
    let mut no_reason = valid_json();
    no_reason.as_object_mut().unwrap().remove("basis_reason");
    let parsed: VerifyReport = serde_json::from_value(no_reason).expect("null reason parses");
    assert!(
        validate_report(&parsed, None).is_err(),
        "a whole-suite report with no reason fails validation (AC-352: a fallback with no reason fails)"
    );
    // an unknown member is refused too (typed in, typed out — §3 principle 6)
    let mut extra = valid_json();
    extra["pass"] = json!(true);
    assert!(
        serde_json::from_value::<VerifyReport>(extra).is_err(),
        "an unknown member is refused"
    );

    // the comparator record: the most recent WHOLE-SUITE report at this toolchain version seeds it
    let runs: Vec<(&str, VerifyReport)> = vec![
        (
            "run-1",
            report(
                Basis::WholeSuite,
                Some(BasisReason::ProbeUnverified),
                230,
                230,
                TotalTestCountSource::OwnExecutedCount,
            ),
        ),
        (
            "run-2",
            report(
                Basis::Affected,
                None,
                12,
                230,
                TotalTestCountSource::RecordedRun {
                    run_id: "run-1".to_owned(),
                },
            ),
        ),
        (
            "run-3",
            report(
                Basis::WholeSuite,
                Some(BasisReason::PolicyForced),
                236,
                236,
                TotalTestCountSource::OwnExecutedCount,
            ),
        ),
    ];
    let recorded = latest_whole_suite(runs.iter().map(|(id, r)| (*id, r)), NODE_PIN)
        .expect("two whole-suite runs are recorded at this pin");
    assert_eq!(
        recorded,
        WholeSuiteRecord {
            toolchain_version: NODE_PIN.to_owned(),
            count: 236,
            run_id: "run-3".to_owned()
        },
        "the MOST RECENT whole-suite count, and the run it came from"
    );
    assert!(
        latest_whole_suite(runs.iter().map(|(id, r)| (*id, r)), "rust 1.98.1").is_none(),
        "another pin has no record — the comparator is per (toolchain, version pin)"
    );

    // on an `affected` run: total = the recorded whole-suite count, source = that run
    let (total, source) = total_test_count_fields(Basis::Affected, 9, Some(&recorded))
        .expect("affected with a record");
    assert_eq!(total, 236);
    assert_eq!(
        source,
        TotalTestCountSource::RecordedRun {
            run_id: "run-3".to_owned()
        },
        "the report names the run the count came from"
    );
    // on the bootstrap run: total = the run's own executed count (ADR-0104 decision 3)
    let (total, source) = total_test_count_fields(Basis::WholeSuite, 241, None).expect("bootstrap");
    assert_eq!(
        (total, source),
        (241, TotalTestCountSource::OwnExecutedCount)
    );
    // `affected` with NO record is refused here — ADR-0104 forbids reporting it (the kernel reclassifies first)
    assert!(
        total_test_count_fields(Basis::Affected, 9, None).is_err(),
        "affected with no recorded whole-suite count is refused"
    );

    // field semantics validated against the record: a report whose total is its own executed count on an
    // affected run, or whose source names a different run, fails
    let good = report(
        Basis::Affected,
        None,
        9,
        236,
        TotalTestCountSource::RecordedRun {
            run_id: "run-3".to_owned(),
        },
    );
    validate_report(&good, Some(&recorded)).expect("consistent");
    let own_count = report(
        Basis::Affected,
        None,
        9,
        9,
        TotalTestCountSource::RecordedRun {
            run_id: "run-3".to_owned(),
        },
    );
    assert!(
        validate_report(&own_count, Some(&recorded)).is_err(),
        "an affected run's total is the RECORDED count, never its own"
    );
    let wrong_run = report(
        Basis::Affected,
        None,
        9,
        236,
        TotalTestCountSource::RecordedRun {
            run_id: "run-1".to_owned(),
        },
    );
    assert!(
        validate_report(&wrong_run, Some(&recorded)).is_err(),
        "the source names the run the count came from"
    );
    let affected_with_reason = report(
        Basis::Affected,
        Some(BasisReason::PolicyForced),
        9,
        236,
        TotalTestCountSource::RecordedRun {
            run_id: "run-3".to_owned(),
        },
    );
    assert!(
        validate_report(&affected_with_reason, Some(&recorded)).is_err(),
        "an affected run carries no fallback reason"
    );
    let bootstrap = report(
        Basis::WholeSuite,
        Some(BasisReason::ProbeUnverified),
        241,
        241,
        TotalTestCountSource::OwnExecutedCount,
    );
    validate_report(&bootstrap, None)
        .expect("the bootstrap run is consistent with an empty comparator");
    let bootstrap_borrowed = report(
        Basis::WholeSuite,
        Some(BasisReason::ProbeUnverified),
        241,
        236,
        TotalTestCountSource::OwnExecutedCount,
    );
    assert!(
        validate_report(&bootstrap_borrowed, None).is_err(),
        "the bootstrap run's total is its OWN executed count"
    );
}

fn kind_constants(schema: &Value) -> Vec<String> {
    // the `status` member is a union of tagged objects; each branch's `kind` is a const
    let branches = schema
        .pointer("/properties/rows/items/properties/status/anyOf")
        .or_else(|| schema.pointer("/properties/rows/items/properties/status/oneOf"))
        .and_then(Value::as_array)
        .expect("status is a union of tagged branches");
    let mut kinds = Vec::new();
    for b in branches {
        let k = b
            .pointer("/properties/kind")
            .expect("each branch carries `kind`");
        if let Some(c) = k.get("const").and_then(Value::as_str) {
            kinds.push(c.to_owned());
        } else if let Some(e) = k.get("enum").and_then(Value::as_array) {
            assert_eq!(e.len(), 1, "a branch names ONE kind: {b}");
            kinds.push(e[0].as_str().unwrap().to_owned());
        } else {
            panic!(
                "a branch's `kind` is neither a const nor a one-member enum — that is the wildcard: {b}"
            );
        }
        assert_eq!(
            b.get("additionalProperties"),
            Some(&json!(false)),
            "every branch is closed: {b}"
        );
    }
    kinds
}

/// **AC-333 — the closed enum with no wildcard, the file-path-only negative, the completeness check.**
#[test]
fn ac333_ac_evidence_ledger() {
    // the schema fixture: EXACTLY the five, from the `ledger` companion kind's registered schema
    let schema = CompanionKind::Ledger
        .registered_schema()
        .expect("the ledger kind registers AcLedger's schema (AC-143)");
    let mut kinds = kind_constants(&schema);
    kinds.sort();
    let mut expected: Vec<String> = serde_json::from_str(CLOSED_STATUS_ENUM).unwrap();
    expected.sort();
    assert_eq!(
        kinds, expected,
        "the ledger status is ONE closed enum per AC, five members, no wildcard"
    );
    // and no free-form string anywhere on the `status` member — `type: string` without an enum/const
    // would be the wildcard wearing a different shape
    let status = schema
        .pointer("/properties/rows/items/properties/status")
        .unwrap();
    assert!(
        status.get("type").is_none_or(|t| t != "string"),
        "status is not a free string: {status}"
    );

    // the six companion kinds include `ledger` (AC-143), and the row shape exposes the locator
    assert!(CompanionKind::ALL.contains(&CompanionKind::Ledger));
    let row_props = schema
        .pointer("/properties/rows/items/properties")
        .and_then(Value::as_object)
        .unwrap();
    for m in ["file", "test", "assertion"] {
        assert!(
            schema
                .pointer("/properties/rows/items/properties/evidence")
                .is_some_and(|e| e.to_string().contains(&format!("\"{m}\""))),
            "the row's locator names `{m}`: {row_props:?}"
        );
    }

    // the negative fixture: a pinned row that names a file and nothing else
    let row: AcLedgerRow =
        serde_json::from_str(FILE_PATH_ONLY_ROW).expect("it parses — the refusal is validation's");
    assert!(
        matches!(row.validate(), Err(LedgerRowError::FilePathOnly { .. })),
        "a row naming only a test file fails validation (an AC is a claim about ONE assertion, F25): {:?}",
        row.validate()
    );
    // ...and a pinned row with NO locator at all is not a pinned row
    let bare = AcLedgerRow {
        ac: "AC-9".to_owned(),
        status: LedgerStatus::Pinned,
        evidence: None,
    };
    assert!(matches!(
        bare.validate(),
        Err(LedgerRowError::EvidenceRequired { .. })
    ));
    // an unpinned row carries no locator — "no evidence" is what the status MEANS
    let unpinned = AcLedgerRow {
        ac: "AC-9".to_owned(),
        status: LedgerStatus::Unpinned,
        evidence: None,
    };
    unpinned
        .validate()
        .expect("unpinned with no evidence is the reserved shape");

    // the completeness check: the ledger's AC id set equals the spec's
    let spec_ids: Vec<String> = serde_json::from_str(SPEC_AC_IDS).unwrap();
    let ledger: AcLedger = serde_json::from_str(LEDGER_COMPLETE).unwrap();
    validate_ledger(&ledger, &spec_ids)
        .expect("every spec AC is materialised; the unpinned one as a ROW");
    assert!(
        ledger
            .rows
            .iter()
            .any(|r| r.ac == "AC-903" && r.status == LedgerStatus::Unpinned),
        "the AC with no evidence is materialised as `unpinned`, not omitted"
    );
    // RED on its own claim: omit one AC — the check names the missing id
    let mut short = ledger.clone();
    short.rows.retain(|r| r.ac != "AC-903");
    match validate_ledger(&short, &spec_ids) {
        Err(LedgerError::Missing { acs }) => {
            assert_eq!(acs, vec!["AC-903".to_owned()], "the missing id is NAMED");
        }
        other => panic!(
            "a ledger missing a spec AC must fail the completeness check naming it: {other:?}"
        ),
    }
    // an id the spec does not have is also a mismatch — the sets are EQUAL, not a superset
    let mut extra = ledger.clone();
    extra.rows.push(AcLedgerRow {
        ac: "AC-999".to_owned(),
        status: LedgerStatus::Unpinned,
        evidence: None,
    });
    assert!(matches!(
        validate_ledger(&extra, &spec_ids),
        Err(LedgerError::Unknown { .. })
    ));
    // and a duplicated id is one row too many
    let mut dup = ledger;
    dup.rows.push(dup.rows[0].clone());
    assert!(matches!(
        validate_ledger(&dup, &spec_ids),
        Err(LedgerError::Duplicate { .. })
    ));
}
