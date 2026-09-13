//! The single enumeration of what `rk-types` exports.
//!
//! A rule that fires only on types someone remembered to register is satisfied by an empty
//! registry — the defect AC-133's completeness half was written to close. Everything exported
//! appears here, and the snapshot test asserts it covers the whole exported set.

use serde_json::Value;

/// Name → its exported schema.
pub type Exporter = (&'static str, fn() -> Value);

/// Every schema that crosses the closed-output boundary.
pub const EXPORTED: &[Exporter] = &[("Findings", || {
    crate::schema::export::<crate::artifact::findings::Findings>()
})];

/// Derive all of them, in registry order.
#[must_use]
pub fn export_all() -> Vec<(&'static str, Value)> {
    EXPORTED.iter().map(|(name, f)| (*name, f())).collect()
}
