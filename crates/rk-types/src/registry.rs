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

/// Both lists from ONE table, so a type cannot enter one without the other. *(dated 2026-10-02,
/// `s8h` build, package P2-rk-types.)* The two hand-written lists `s8g` left were kept in step by
/// `judgement_registry::ac133_every_exported_enum_is_registered`; that test still asserts it.
macro_rules! exported {
    ($($name:literal => $ty:path,)+) => {
        /// Every schema that crosses the closed-output boundary.
        pub const EXPORTED: &[Exporter] = &[
            $(($name, || crate::schema::export::<$ty>()),)+
        ];

        /// [`EXPORTED`]'s types, in its order, as NAMED schemas (`$defs` kept) — AC-133's registry
        /// reads enum names from these. Never sent across the boundary.
        pub const EXPORTED_NAMED: &[Exporter] = &[
            $(($name, || crate::schema::judgement::export_named::<$ty>()),)+
        ];
    };
}

// *(dated 2026-10-02, `s8h` build, package P2-rk-types; Record A = ADR-0197 decision 6, proposed.)*
// `s8h` §8h.4's universe of 24 agent-step output kinds, each under its kind name — the name the
// binder looks a step's `value:<Kind>` up by. `Findings` was exported already and is registered
// once, never redeclared; the other 23 are `artifact::lane`'s, in alphabetical order.
// `AcceptanceCriterion` is `s8g`'s (ADR-0192), embedded by `Spec` and `GherkinScenarios`.
exported! {
    "Findings" => crate::artifact::findings::Findings,
    "AcceptanceCriterion" => crate::acceptance::AcceptanceCriterion,
    "ADR" => crate::artifact::lane::Adr,
    "APIContract" => crate::artifact::lane::ApiContract,
    "ArchitectureDescription" => crate::artifact::lane::ArchitectureDescription,
    "CausalChain" => crate::artifact::lane::CausalChain,
    "ChangeRecord" => crate::artifact::lane::ChangeRecord,
    "DebtRegisterEntry" => crate::artifact::lane::DebtRegisterEntry,
    "DesignDoc" => crate::artifact::lane::DesignDoc,
    "Docs" => crate::artifact::lane::Docs,
    "Estimate" => crate::artifact::lane::Estimate,
    "FixDesign" => crate::artifact::lane::FixDesign,
    "GherkinScenarios" => crate::artifact::lane::GherkinScenarios,
    "HypothesisLog" => crate::artifact::lane::HypothesisLog,
    "ImplReport" => crate::artifact::lane::ImplReport,
    "Postmortem" => crate::artifact::lane::Postmortem,
    "ReproAttempt" => crate::artifact::lane::ReproAttempt,
    "ReproCase" => crate::artifact::lane::ReproCase,
    "Retrospective" => crate::artifact::lane::Retrospective,
    "RolloutPlan" => crate::artifact::lane::RolloutPlan,
    "RootCause" => crate::artifact::lane::RootCause,
    "SelfCheck" => crate::artifact::lane::SelfCheck,
    "Spec" => crate::artifact::lane::Spec,
    "TestDesign" => crate::artifact::lane::TestDesign,
    "ThreatModel" => crate::artifact::lane::ThreatModel,
}

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
