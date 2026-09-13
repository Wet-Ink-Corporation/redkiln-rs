//! The one door a schema leaves this crate through.
//!
//! Every schema exported across the closed-output boundary passes through `export_settings()`, so
//! the snapshot, the CI lint and the must-fail fixtures all see the same EXPORTED shape — not the
//! raw derive. ADR-0095 decision 1 is a rule about the exported shape, and `schemars` does not
//! enforce it.

pub mod lint;
pub mod transform;

use schemars::JsonSchema;
use schemars::generate::SchemaSettings;
use serde_json::Value;

/// The export contract.
///
/// `inline_subschemas` is load-bearing, not a style choice: "per exported schema" must mean per
/// self-contained document for the vendor's per-request caps to be the quantity being counted. A
/// `$ref` to a `$defs` entry would hide union-typed and optional parameters below the count.
/// `lint::Violation::UnresolvedRef` catches the recursive-type case inlining cannot reach.
#[must_use]
pub fn export_settings() -> SchemaSettings {
    let mut settings = SchemaSettings::draft2020_12();
    settings.inline_subschemas = true;
    // ADR-0095 decision 1: the transform runs INSIDE the export, so the snapshot, the CI lint
    // and every must-fail fixture see the exported shape rather than the raw derive.
    settings.with_transform(transform::KindOneOfToAnyOf)
}

/// Derive one exported schema.
#[must_use]
pub fn export<T: JsonSchema>() -> Value {
    let schema = export_settings().into_generator().root_schema_for::<T>();
    serde_json::to_value(schema).expect("a derived schema serialises to JSON")
}
