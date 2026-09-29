//! `rk_types::AcceptanceCriterion` — `s8g`'s schema record (R2 = ADR-0192 decision 3, proposed;
//! D21-42 10a; `s8g` §8g.3). AC-137's GIVEN/THEN, ADR-0104 decision 1's `mutation` field in
//! ADR-0084's discriminated form, one ingest door that refuses a prose-only mutation (ADR-0096),
//! and the type in `registry::EXPORTED`. *(Added 2026-09-29, `s8g` build, package S8G-P2-RKTYPES.)*
//!
//! Every fixture is a `json!` literal handed to the one door, so each red is the door's answer and
//! never a compile error naming a field.

use rk_types::acceptance::{AcceptanceCriterion, CriterionIngestError, MutationDeclaration};
use rk_types::registry;
use rk_types::schema::lint::lint;
use serde_json::json;

const PATCH: &str = "--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-    a < b\n+    a <= b\n";

/// AC-137: *"`verification` is an authored field from the first CLI write"* — and the TYPE accepts a
/// criterion with none, and with no GWT, because AC-455's policy row (not the type) decides whether
/// an empty `verification` blocks a stage (ADR-0192 decision 6).
#[test]
fn ac137_criterion_without_gwt_or_verification_is_accepted() {
    let bare = json!({ "id": "AC-1", "criterion": "the gate refuses an empty answer" });
    let ac = AcceptanceCriterion::ingest(bare.clone())
        .expect("a criterion with no gwt, no verification and no mutation is accepted (AC-137)");
    assert_eq!(ac.id, "AC-1");
    assert_eq!(ac.criterion.as_str(), "the gate refuses an empty answer");
    assert!(ac.gwt.is_none(), "an absent gwt is None");
    assert!(ac.verification.is_none(), "an absent verification is None");
    assert!(ac.mutation.is_none(), "an absent mutation is None");
    // Absent members stay absent on the way out, so the row round-trips byte for byte.
    assert_eq!(serde_json::to_value(&ac).unwrap(), bare);

    // The full form, for contrast: GWT with an `and` list, and an authored verification.
    let full = json!({
        "id": "AC-2",
        "criterion": "c",
        "gwt": { "given": "g", "when": "w", "then": "t", "and": ["a1"] },
        "verification": "cargo test -p x --test it -- m::t --exact",
    });
    let ac = AcceptanceCriterion::ingest(full).expect("the full form is accepted");
    let gwt = ac.gwt.expect("gwt present");
    assert_eq!(gwt.and.len(), 1);
    assert_eq!(gwt.then.as_str(), "t");

    // A GWT's `and` is optional; its three clauses are not.
    let no_and = json!({ "id": "AC-3", "criterion": "c",
        "gwt": { "given": "g", "when": "w", "then": "t" } });
    assert!(
        AcceptanceCriterion::ingest(no_and)
            .unwrap()
            .gwt
            .unwrap()
            .and
            .is_empty()
    );
    let no_then = json!({ "id": "AC-4", "criterion": "c",
        "gwt": { "given": "g", "when": "w" } });
    assert!(
        AcceptanceCriterion::ingest(no_then).is_err(),
        "a GWT without its THEN is malformed"
    );
    // An unknown member is refused, not ignored.
    let extra = json!({ "id": "AC-5", "criterion": "c", "status": "pass" });
    assert!(AcceptanceCriterion::ingest(extra).is_err());
}

/// ADR-0084's discriminated form: `#[serde(tag = "kind")]` over `declared { patch, suite }` and
/// `not_feasible { reason }` — the serde form that supersedes the fixture reader's
/// `[ac.mutation] not_feasible = "<reason>"` (ADR-0192 decision 1's table, row 3).
#[test]
fn mutation_declaration_round_trips_declared_and_not_feasible() {
    let declared = json!({ "kind": "declared", "patch": PATCH, "suite": "rk-types::it" });
    let not_feasible =
        json!({ "kind": "not_feasible", "reason": "the behaviour is a timing window" });

    let d: MutationDeclaration = serde_json::from_value(declared.clone())
        .expect("the declared arm parses in the tagged form");
    match &d {
        MutationDeclaration::Declared { patch, suite } => {
            assert_eq!(patch, PATCH);
            assert_eq!(suite, "rk-types::it");
        }
        MutationDeclaration::NotFeasible { .. } => panic!("parsed as the wrong arm"),
    }
    assert_eq!(
        serde_json::to_value(&d).unwrap(),
        declared,
        "declared round-trips"
    );

    let n: MutationDeclaration = serde_json::from_value(not_feasible.clone())
        .expect("the not_feasible arm parses in the tagged form");
    match &n {
        MutationDeclaration::NotFeasible { reason } => {
            assert_eq!(reason.as_str(), "the behaviour is a timing window");
        }
        MutationDeclaration::Declared { .. } => panic!("parsed as the wrong arm"),
    }
    assert_eq!(
        serde_json::to_value(&n).unwrap(),
        not_feasible,
        "not_feasible round-trips"
    );

    // Through the criterion, both arms survive a write and a read unchanged.
    for m in [declared, not_feasible] {
        let row = json!({ "id": "AC-9", "criterion": "c", "mutation": m });
        let ac = AcceptanceCriterion::ingest(row.clone()).expect("a criterion carrying the arm");
        assert_eq!(serde_json::to_value(&ac).unwrap(), row);
    }

    // The fixture reader's old spelling and an unknown tag are not the form.
    for old in [
        json!({ "not_feasible": "reason" }),
        json!({ "kind": "maybe", "reason": "r" }),
        json!({ "kind": "declared", "patch": PATCH }),
    ] {
        assert!(
            serde_json::from_value::<MutationDeclaration>(old.clone()).is_err(),
            "{old} is not ADR-0084's discriminated form"
        );
    }
}

/// A criterion with every authored member present, so the mutation is the only thing a refusal
/// can be about.
fn authored(id: &str, mutation: serde_json::Value) -> serde_json::Value {
    json!({
        "id": id,
        "criterion": "c",
        "gwt": { "given": "g", "when": "w", "then": "t" },
        "verification": "cargo test -p x --test it -- m::t --exact",
        "mutation": mutation,
    })
}

/// ADR-0096 via golden constraint 12: a mutation that is prose — a sentence where the declaration
/// belongs, or a `declared` arm whose patch is not a diff — is refused at ingest, naming the AC,
/// and never reaches the ledger (where it would read as a mutation that ran).
#[test]
fn a_prose_only_mutation_is_refused_at_ingest() {
    // The control: a real diff with a suite is admitted.
    let ok = authored(
        "AC-7",
        json!({ "kind": "declared", "patch": PATCH, "suite": "rk-types::it" }),
    );
    AcceptanceCriterion::ingest(ok).expect("a declared unified diff is admitted");

    // A sentence in place of the declaration.
    let sentence = authored(
        "AC-7",
        json!("flip the comparison in lib.rs and watch the test fail"),
    );
    assert!(
        matches!(
            AcceptanceCriterion::ingest(sentence),
            Err(CriterionIngestError::Malformed { .. })
        ),
        "a bare sentence is not a declaration"
    );

    // A declared arm whose `patch` is prose.
    let prose_patch = authored(
        "AC-7",
        json!({ "kind": "declared",
                "patch": "change the < to <= in the comparison",
                "suite": "rk-types::it" }),
    );
    match AcceptanceCriterion::ingest(prose_patch) {
        Err(CriterionIngestError::ProseOnlyMutation { ac, .. }) => assert_eq!(ac, "AC-7"),
        other => panic!("a prose patch must be refused naming the AC, got {other:?}"),
    }

    // A declared arm with an empty suite names nothing to run.
    let no_suite = authored(
        "AC-8",
        json!({ "kind": "declared", "patch": PATCH, "suite": "  " }),
    );
    assert!(
        matches!(
            AcceptanceCriterion::ingest(no_suite),
            Err(CriterionIngestError::ProseOnlyMutation { .. })
        ),
        "a declared mutation with no suite is refused"
    );

    // `not_feasible` with no reason is a bare claim (AC-337: the reason is the declaration).
    let no_reason = authored("AC-9", json!({ "kind": "not_feasible", "reason": "" }));
    assert!(
        matches!(
            AcceptanceCriterion::ingest(no_reason),
            Err(CriterionIngestError::ProseOnlyMutation { .. })
        ),
        "a not_feasible declaration with no reason is refused"
    );
}

/// D21-42 10a: `AcceptanceCriterion` enters `registry::EXPORTED` (*"Every schema that crosses the
/// closed-output boundary"*), and its exported shape passes the closed-output lint. The snapshot
/// `exported__AcceptanceCriterion.snap` is `schema_snapshot`'s, accepted once.
#[test]
fn acceptance_criterion_is_in_the_exported_registry() {
    let all = registry::export_all();
    let found: Vec<_> = all
        .iter()
        .filter(|(name, _)| *name == "AcceptanceCriterion")
        .collect();
    assert_eq!(
        found.len(),
        1,
        "registry::EXPORTED carries AcceptanceCriterion exactly once"
    );
    let schema = &found[0].1;
    assert_eq!(schema["title"], "AcceptanceCriterion");
    let violations = lint(schema);
    assert!(
        violations.is_empty(),
        "the exported criterion breaches the closed-output rules: {violations:#?}"
    );
    // The exported schema admits the bare AC-137 form and refuses a prose mutation's shape.
    let v = jsonschema::validator_for(schema).expect("the exported schema compiles");
    assert!(v.is_valid(&json!({ "id": "AC-1", "criterion": "c" })));
    assert!(!v.is_valid(&json!({ "id": "AC-1", "criterion": "c", "mutation": "prose" })));
    assert!(v.is_valid(&json!({ "id": "AC-1", "criterion": "c",
        "mutation": { "kind": "not_feasible", "reason": "r" } })));
}
