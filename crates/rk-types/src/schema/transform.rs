//! The post-derive transform ADR-0095 decision 1 requires.
//!
//! `schemars` 1.2.2 renders `#[serde(tag = "kind")]` as a **`oneOf`** when the variants are unique
//! (PIA-005; ERRATA E-0084-2). The vendor's structured-outputs page lists `anyOf`/`allOf` and does
//! **not** list `oneOf`, and a schema outside the documented contract fails OPEN — exit 0 with
//! `subtype: success` and no `structured_output` at all (F20). So the derive is corrected on
//! export rather than the enum being flattened: flattening would spend the ≤ 24 optional budget on
//! payload fields that are mutually exclusive by construction, and would turn `StepError` into a
//! struct the kernel cannot match exhaustively — the property AC-229 exists for.

use schemars::Schema;
use schemars::transform::{Transform, transform_subschemas};
use serde_json::Value;

/// Rewrites a `kind`-discriminated `oneOf` into an equivalent `anyOf`.
///
/// SCOPE, stated because a guard scoped wrong cuts both ways (golden constraint 9). This rewrites
/// **only** a `oneOf` whose every branch is an object with a required `kind` property — the shape
/// `#[serde(tag = "kind")]` produces. Any other `oneOf` — an untagged union, a hand-written one —
/// is left exactly where it is, so `lint::Violation::NoOneOf` still catches it. A transform that
/// rewrote every `oneOf` would not fix the schemas; it would launder the lint.
#[derive(Debug, Clone, Copy)]
pub struct KindOneOfToAnyOf;

/// Is every branch of this array an object discriminated by a required `kind`?
fn all_branches_kind_discriminated(branches: &[Value]) -> bool {
    !branches.is_empty()
        && branches.iter().all(|b| {
            b.get("properties").and_then(|p| p.get("kind")).is_some()
                && b.get("required")
                    .and_then(Value::as_array)
                    .is_some_and(|r| r.iter().any(|v| v.as_str() == Some("kind")))
        })
}

impl Transform for KindOneOfToAnyOf {
    fn transform(&mut self, schema: &mut Schema) {
        // Recurse FIRST, so a nested tagged enum is reached wherever it sits.
        transform_subschemas(self, schema);

        let Some(map) = schema.as_object_mut() else {
            return;
        };
        let Some(branches) = map.get("oneOf").and_then(Value::as_array).cloned() else {
            return;
        };
        if !all_branches_kind_discriminated(&branches) {
            return;
        }
        map.remove("oneOf");
        map.insert("anyOf".to_owned(), Value::Array(branches));
    }
}
