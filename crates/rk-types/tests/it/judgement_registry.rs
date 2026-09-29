//! **AC-133's judgement-enum registry**, built once, here (`s8g` §8g.3; R2 = ADR-0192 decision 8,
//! proposed; D21-43's integration call: *"AC-133's judgement-enum registry is built once, by
//! **s8g**. s8h consumes it."*). *(Added 2026-09-29, `s8g` build, package S8G-P2-RKTYPES.)*
//!
//! AC-133's *Verified by*, three instruments: *"a registry-completeness test (every shipped enum is
//! registered, or the lint fails); a schema lint rule for the rationale sibling; a fixture judgement
//! enum with no rationale sibling that must fail."* A third test keeps the registry from outliving
//! its types. The **value** of each yes/no is not this file's: AC-133's limit reads *"that
//! classification is the human gate AC-156"*, which is `s8h`'s.
//!
//! The walk reads the NAMED schema (`$defs` kept, not inlined), because the exported shape inlines
//! every subschema and an inlined enum carries no name to register.

use rk_types::registry;
use rk_types::schema::judgement::{
    JudgementEntry, REGISTRY, enums_in, export_named, rationale_violations_with,
};
use rk_types::text::Text;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

// ---- fixtures: never in `src/`, never in EXPORTED (`fixtures.rs`' rule). ----

/// A judgement enum, registered `yes` in the fixture registry below.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum FixtureVerdict {
    Holds,
    Breaks,
}

/// Must fail: a judgement enum with no sibling `rationale`.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct JudgedWithoutRationale {
    ac: String,
    verdict: FixtureVerdict,
}

/// The control: the same enum beside a free-text `rationale`.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct JudgedWithRationale {
    ac: String,
    verdict: FixtureVerdict,
    rationale: Text,
}

/// The same enum nested in a list, one object down, with no `rationale` there.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct NestedJudgementWithoutRationale {
    items: Vec<JudgedWithoutRationale>,
}

const FIXTURE_REGISTRY: &[JudgementEntry] = &[JudgementEntry {
    name: "FixtureVerdict",
    judgement_bearing: true,
    reason: "fixture",
}];

fn exported_enums() -> BTreeSet<String> {
    registry::export_all_named()
        .iter()
        .flat_map(|(_, named)| enums_in(named))
        .collect()
}

/// **Completeness** (AC-133's repair: *"as written before, the criterion fired only on enums
/// someone had already registered — a system with an empty registry passed it"*). Every enum
/// reachable from `registry::EXPORTED` is registered with an explicit yes/no, `Findings`' and
/// `AcceptanceCriterion`'s included.
#[test]
fn ac133_every_exported_enum_is_registered() {
    // The walk's control: it SEES an enum, or the completeness check below proves nothing.
    let seen = enums_in(&export_named::<JudgedWithoutRationale>());
    assert_eq!(
        seen,
        BTreeSet::from(["FixtureVerdict".to_owned()]),
        "the enum walk must find the fixture's one enum"
    );

    // The walk reads EXPORTED_NAMED; it covers exactly EXPORTED, in order, or a type escapes it.
    let exported: Vec<&str> = registry::EXPORTED.iter().map(|(n, _)| *n).collect();
    let named: Vec<&str> = registry::EXPORTED_NAMED.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        named, exported,
        "EXPORTED_NAMED must name EXPORTED's types, in order"
    );
    for ((n, exp), (_, nam)) in registry::export_all()
        .iter()
        .zip(registry::export_all_named())
    {
        assert_eq!(
            exp["title"], nam["title"],
            "{n}: the named schema derives a different type"
        );
    }

    let reachable = exported_enums();
    assert!(
        !reachable.is_empty(),
        "no enum is reachable from EXPORTED — this check would pass over nothing"
    );
    let registered: BTreeSet<&str> = REGISTRY.iter().map(|e| e.name).collect();
    assert_eq!(
        registered.len(),
        REGISTRY.len(),
        "an enum is registered twice"
    );
    let missing: Vec<&String> = reachable
        .iter()
        .filter(|n| !registered.contains(n.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "enums reachable from registry::EXPORTED with no judgement-bearing yes/no: {missing:?}"
    );
    for e in REGISTRY {
        assert!(
            !e.reason.trim().is_empty(),
            "{}'s yes/no carries no stated reason",
            e.name
        );
    }
    // Findings' label is the model's judgement; it is registered `yes`.
    assert!(
        REGISTRY
            .iter()
            .any(|e| e.name == "EvidenceLabel" && e.judgement_bearing),
        "Findings' EvidenceLabel is the model's label, a judgement (ADR-0074 clause 1)"
    );
}

/// **The rationale-sibling rule**, with its must-fail fixture: an object holding a member of a
/// judgement-bearing enum carries a free-text `rationale` on the same object.
#[test]
fn ac133_a_judgement_enum_without_a_rationale_sibling_fails() {
    let bad =
        rationale_violations_with(&export_named::<JudgedWithoutRationale>(), FIXTURE_REGISTRY);
    assert_eq!(
        bad.len(),
        1,
        "the must-fail fixture must fail exactly once: {bad:?}"
    );
    assert_eq!(bad[0].property, "verdict");
    assert_eq!(bad[0].judgement_enum, "FixtureVerdict");

    let nested = rationale_violations_with(
        &export_named::<NestedJudgementWithoutRationale>(),
        FIXTURE_REGISTRY,
    );
    assert_eq!(
        nested.len(),
        1,
        "the rule reaches an object one list down: {nested:?}"
    );

    let good = rationale_violations_with(&export_named::<JudgedWithRationale>(), FIXTURE_REGISTRY);
    assert!(good.is_empty(), "the control has its rationale: {good:?}");

    // The same enum marked `no` is not held to the rule.
    let no = [JudgementEntry {
        name: "FixtureVerdict",
        judgement_bearing: false,
        reason: "fixture",
    }];
    assert!(rationale_violations_with(&export_named::<JudgedWithoutRationale>(), &no).is_empty());

    // Every exported schema meets the rule under the real registry.
    for (name, named) in registry::export_all_named() {
        let v = rationale_violations_with(&named, REGISTRY);
        assert!(
            v.is_empty(),
            "exported `{name}` breaks the rationale rule: {v:?}"
        );
    }
}

/// **The registry does not outlive its types**: every registered name is an enum reachable from
/// `registry::EXPORTED` today. A type `s8h` adds registers into this one registry (D21-43), and a
/// type removed from `EXPORTED` takes its row with it.
#[test]
fn the_registry_names_no_enum_that_is_not_exported() {
    assert!(
        !REGISTRY.is_empty(),
        "the registry is empty — this check would pass over nothing"
    );
    let reachable = exported_enums();
    let stale: Vec<&str> = REGISTRY
        .iter()
        .map(|e| e.name)
        .filter(|n| !reachable.contains(*n))
        .collect();
    assert!(
        stale.is_empty(),
        "registered enums no exported schema reaches: {stale:?}"
    );
}
