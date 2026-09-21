//! The differential report and its committed schema (AC-448, AC-467, AC-468).

use redkiln_migrate::lift::{self, Natural};
use redkiln_migrate::report;
use serde_json::json;

use crate::legacy_mini_repo;

/// Compile the committed schema the same way the crate does, and validate one instance.
fn validates(instance: &serde_json::Value) -> bool {
    let schema: serde_json::Value =
        serde_json::from_str(report::REPORT_SCHEMA).expect("the committed schema is JSON");
    let validator = jsonschema::validator_for(&schema).expect("the committed schema compiles");
    validator.is_valid(instance)
}

fn a_valid_report() -> serde_json::Value {
    json!({
        "corpus_sha": "61f590e51cbb00301c310e46aab615563a7787b8",
        "tool_version": "0.0.0",
        "per_kind_slot": [
            { "kind": "spec", "slot": "body", "lifted": 1, "lifted_with_loss": 1, "failed": 0 }
        ]
    })
}

/// AC-468's schema half (BINDING, E13).
///
/// *"lifted, lifted-with-loss and failed are three distinct counts AND a report or consumer that
/// presents only a combined total, or folds with-loss into lifted, fails validation."*
///
/// # The two halves are instrumented differently, and the second is not a schema check
///
/// A `total`-only row and a row carrying `total` beside the three are refused **by the schema** —
/// `additionalProperties: false` and the three `required` integers. A **folded** form is not
/// detectable by any JSON Schema: `lifted` and `lifted_with_loss` are both integers, and a schema
/// cannot see that one absorbed the other. So the folded conjunct is instrumented by the
/// **fixture-count check** below, over `legacy-mini`'s known composition. No consumer exists in v0.
#[test]
fn ac468_schema_rejects_total_only_and_folded_forms() {
    // Positive control FIRST: a scanner that refuses everything proves nothing.
    assert!(
        validates(&a_valid_report()),
        "a well-formed report does NOT validate against the committed schema — every refusal \
         below would then be vacuous"
    );

    // (a) a total-only row
    let mut total_only = a_valid_report();
    total_only["per_kind_slot"] = json!([{ "kind": "spec", "slot": "body", "total": 2 }]);
    assert!(
        !validates(&total_only),
        "a row presenting only a combined total VALIDATED (AC-468)"
    );

    // (b) a row carrying `total` beside the three
    let mut total_beside = a_valid_report();
    total_beside["per_kind_slot"] = json!([{
        "kind": "spec", "slot": "body",
        "lifted": 1, "lifted_with_loss": 1, "failed": 0, "total": 2
    }]);
    assert!(
        !validates(&total_beside),
        "a row carrying `total` beside the three counts VALIDATED — additionalProperties is not \
         false on the row (AC-468)"
    );

    // (c) a top-level total
    let mut top_total = a_valid_report();
    top_total["total"] = json!(2);
    assert!(
        !validates(&top_total),
        "a top-level `total` VALIDATED — additionalProperties is not false at the top (AC-468)"
    );

    // (d) each of the three counts is individually required
    for missing in ["lifted", "lifted_with_loss", "failed"] {
        let mut r = a_valid_report();
        let row = r["per_kind_slot"][0].as_object_mut().expect("the row");
        row.remove(missing);
        assert!(
            !validates(&r),
            "a row missing `{missing}` VALIDATED — the three counts are not three DISTINCT \
             required fields (AC-468)"
        );
    }

    // (e) THE FOLDED FORM, which no schema can catch: the exact counts over legacy-mini.
    //
    // §7d.13's composition: one clean spec.md, one diff-prose spec.md, one free-form report.md.
    // Summed over slot that is spec (1, 1, 0) and report (0, 1, 0). `lifted 2` is a fold;
    // `lifted_with_loss 0` is a drop. Checkpoint 7d asserts the same two triples.
    let repo = legacy_mini_repo();
    let lifted = lift::run(repo.path(), &mut Natural).expect("the fixture lifts");
    report::validate(&lifted.report).expect("the built report validates against its own schema");

    for (kind, want) in [("report", (0, 1, 0)), ("spec", (1, 1, 0))] {
        let got = lifted
            .report
            .per_kind_slot
            .iter()
            .filter(|r| r.kind == kind)
            .fold((0, 0, 0), |a, r| {
                (a.0 + r.lifted, a.1 + r.lifted_with_loss, a.2 + r.failed)
            });
        assert_eq!(
            got, want,
            "kind `{kind}` summed over slot reads {got:?}, not {want:?} — `lifted` too high is a \
             FOLD (with-loss added into lifted) and `lifted_with_loss` at zero is a DROP; \
             §7d.13's fixture carries one clean spec, one diff-prose spec and one free-form report"
        );
    }
}

/// The report's own shape, as the block's dry-run python reads it.
#[test]
fn report_carries_three_counts_per_kind_and_no_total() {
    let repo = legacy_mini_repo();
    let lifted = lift::run(repo.path(), &mut Natural).expect("the fixture lifts");
    let value = serde_json::to_value(&lifted.report).expect("the report serialises");

    let obj = value.as_object().expect("the report is an object");
    let keys: std::collections::BTreeSet<&str> = obj.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        ["corpus_sha", "per_kind_slot", "tool_version"]
            .into_iter()
            .collect(),
        "the report's top-level keys are not AC-467's three"
    );

    assert_eq!(
        lifted.report.corpus_sha.len(),
        40,
        "corpus_sha is not a 40-hex sha: {}",
        lifted.report.corpus_sha
    );
    assert_eq!(lifted.report.tool_version, env!("CARGO_PKG_VERSION"));

    // One row per kind, slot = "body" — see src/report.rs for why the arithmetic forbids more.
    let kinds: Vec<&str> = lifted
        .report
        .per_kind_slot
        .iter()
        .map(|r| r.kind.as_str())
        .collect();
    let unique: std::collections::BTreeSet<&str> = kinds.iter().copied().collect();
    assert_eq!(
        kinds.len(),
        unique.len(),
        "a kind appears in more than one row — v0 emits one row per kind"
    );
    assert_eq!(
        unique.len(),
        21,
        "the report carries {} kinds, not the twenty-one legacy-mini yields",
        unique.len()
    );
    for row in &lifted.report.per_kind_slot {
        assert_eq!(
            row.slot,
            report::SLOT_BODY,
            "row {} has a non-body slot",
            row.kind
        );
    }

    // The fourteen companion kinds, which the block asserts as a subset.
    let companion: std::collections::BTreeSet<&str> = rk_types::companion::CompanionKind::ALL
        .iter()
        .map(|k| k.wire_name())
        .chain(rk_types::companion::PHASE_2_KINDS.iter().copied())
        .collect();
    let missing: Vec<&&str> = companion.iter().filter(|k| !unique.contains(**k)).collect();
    assert!(
        missing.is_empty(),
        "the report is missing companion kinds: {missing:?}"
    );
}
