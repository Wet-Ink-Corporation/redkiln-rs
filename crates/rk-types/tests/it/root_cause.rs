//! `RootCause.discriminating_test` — `s8h` §8h.4 (D21-47 cross #5). *(Added 2026-10-02, `s8h`
//! build, package P2-rk-types; Record A = ADR-0197 decision 6, proposed.)*
//!
//! `s8b` §8b.5's (C) rows bind `${compare.outputs.discriminating_test}` and
//! `${analyse.outputs.discriminating_test}`; both steps output `RootCause`, and the stand-in's row
//! lacked the field. The member is the test selection `confirm` re-runs, so it is a REQUIRED
//! string. Read through `registry::EXPORTED` by kind name — the binder's door (§8h.4) — and checked
//! against the exported schema by a validator, so the test reads what crosses the boundary.

use rk_types::registry;
use serde_json::{Value, json};

fn exported(kind: &str) -> Value {
    registry::export_all()
        .into_iter()
        .find(|(n, _)| *n == kind)
        .map(|(_, s)| s)
        .unwrap_or_else(|| panic!("`{kind}` is not registered in registry::EXPORTED"))
}

fn root_cause(discriminating_test: Option<Value>) -> Value {
    let mut v = json!({
        "cause": "the cache key omits the locale",
        "contributing_factors": ["no test covers a non-default locale"],
        "confidence": "high",
        "summary": "a stale entry is served across locales",
        "rationale": "the discriminating test fails before the fix and passes after it"
    });
    if let Some(t) = discriminating_test {
        v["discriminating_test"] = t;
    }
    v
}

#[test]
fn discriminating_test_is_a_required_member() {
    let schema = exported("RootCause");

    let required: Vec<&str> = schema["required"]
        .as_array()
        .expect("RootCause lists its required members")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        required.contains(&"discriminating_test"),
        "`discriminating_test` is required, never optional: {required:?}"
    );
    assert_eq!(
        schema.pointer("/properties/discriminating_test/type"),
        Some(&json!("string")),
        "`discriminating_test` is a string: the test selection `confirm` re-runs"
    );

    let validator = jsonschema::validator_for(&schema).expect("the exported schema compiles");
    // Control: the same value WITH the member validates, so the refusal below is the member's.
    let with = root_cause(Some(json!("cache::locale_is_part_of_the_key")));
    assert!(
        validator.is_valid(&with),
        "control: a RootCause carrying the member validates: {:?}",
        validator
            .iter_errors(&with)
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
    );
    assert!(
        !validator.is_valid(&root_cause(None)),
        "a RootCause without `discriminating_test` must not validate"
    );
    assert!(
        !validator.is_valid(&root_cause(Some(json!(["a", "b"])))),
        "`discriminating_test` is one string, not a list"
    );
}
