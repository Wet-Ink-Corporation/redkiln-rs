//! The single enumeration of what `rk-types` exports.
//!
//! A rule that fires only on types someone remembered to register is satisfied by an empty
//! registry — the defect AC-133's completeness half was written to close. Everything exported
//! appears here, and the snapshot test asserts it covers the whole exported set.
//!
//! *(dated 2026-09-29, `s8g` build, package S8G-P2-RKTYPES; R2 = ADR-0192 decisions 3 and 8,
//! proposed.)* `AcceptanceCriterion` enters (D21-42 10a), and [`EXPORTED_NAMED`] carries the same
//! types, in the same order, derived with `$defs` kept for AC-133's judgement-enum registry
//! (`schema::judgement`). `judgement_registry::ac133_every_exported_enum_is_registered` asserts the
//! two lists name the same types in the same order, so a type cannot enter one alone.

use serde_json::Value;

/// Name → its exported schema.
pub type Exporter = (&'static str, fn() -> Value);

/// Every schema that crosses the closed-output boundary.
pub const EXPORTED: &[Exporter] = &[
    ("Findings", || {
        crate::schema::export::<crate::artifact::findings::Findings>()
    }),
    ("AcceptanceCriterion", || {
        crate::schema::export::<crate::acceptance::AcceptanceCriterion>()
    }),
];

/// [`EXPORTED`]'s types, in its order, as NAMED schemas (`$defs` kept) — AC-133's registry reads
/// enum names from these. Never sent across the boundary.
pub const EXPORTED_NAMED: &[Exporter] = &[
    ("Findings", || {
        crate::schema::judgement::export_named::<crate::artifact::findings::Findings>()
    }),
    ("AcceptanceCriterion", || {
        crate::schema::judgement::export_named::<crate::acceptance::AcceptanceCriterion>()
    }),
];

/// Derive all of them, in registry order.
#[must_use]
pub fn export_all() -> Vec<(&'static str, Value)> {
    EXPORTED.iter().map(|(name, f)| (*name, f())).collect()
}

/// Derive every exported type's NAMED schema, in registry order (AC-133's walk).
#[must_use]
pub fn export_all_named() -> Vec<(&'static str, Value)> {
    EXPORTED_NAMED
        .iter()
        .map(|(name, f)| (*name, f()))
        .collect()
}
