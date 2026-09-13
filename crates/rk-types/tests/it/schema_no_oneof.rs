//! `no_oneof_at_any_node` — AC-131, ADR-0095 decision 1.
//!
//! Two halves. The POSITIVE half asserts every schema in `registry::EXPORTED` is conformant. The
//! NEGATIVE half asserts the fixtures that should breach each rule actually do — without it, a
//! `lint()` that returned an empty vec would make the positive half pass having checked nothing.

use crate::fixtures::{
    ExternallyTagged, NestedUnion, OpenObject, OptionalBeforeRequired, TaggedKind,
};
use crate::support::{assert_only, exported};
use rk_types::registry;
use rk_types::schema::lint::{Violation, lint};

#[test]
fn no_oneof_at_any_node() {
    let all = registry::export_all();
    assert!(
        !all.is_empty(),
        "the exported set is empty — this test would assert nothing"
    );

    for (name, schema) in all {
        let violations = lint(&schema);
        assert!(
            violations.is_empty(),
            "exported schema `{name}` carries violations: {violations:#?}"
        );
    }
}

#[test]
fn tagged_enum_without_the_transform_renders_oneof() {
    // The PIA-005 mutant, pinned as a standing fact rather than a memory: schemars 1.2.2 renders
    // `#[serde(tag = "kind")]` as `oneOf`. If a future schemars stops doing so, this fails and the
    // transform's reason for existing must be re-read — it does not silently become dead code.
    let raw = serde_json::to_value(
        schemars::generate::SchemaSettings::draft2020_12()
            .into_generator()
            .root_schema_for::<TaggedKind>(),
    )
    .expect("schema serialises");
    assert!(
        raw.to_string().contains("oneOf"),
        "schemars 1.2.2 is expected to render a kind-discriminated enum as oneOf (PIA-005)"
    );
}

#[test]
fn open_object_is_caught() {
    assert_only(
        &lint(&exported::<OpenObject>()),
        &Violation::OpenObject {
            pointer: String::new(),
        },
        "OpenObject",
    );
}

#[test]
fn nested_union_is_caught() {
    let v = lint(&exported::<NestedUnion>());
    assert_only(
        &v,
        &Violation::NestedUnion {
            pointer: "/properties/f/anyOf/0".to_owned(),
        },
        "NestedUnion",
    );
}

#[test]
fn optional_before_required_is_caught() {
    let v = lint(&exported::<OptionalBeforeRequired>());
    assert_only(
        &v,
        &Violation::OptionalBeforeRequired {
            pointer: String::new(),
            property: "z_opt".to_owned(),
        },
        "OptionalBeforeRequired",
    );
}

#[test]
fn a_non_kind_discriminated_oneof_is_still_flagged() {
    // The transform's SCOPE, pinned. It must rewrite the kind-discriminated shape and leave every
    // other `oneOf` exactly where it is, so the lint still catches it. Without this assertion an
    // over-broad transform is green — measured 2026-09-13, the mutant survived.
    let v = lint(&exported::<ExternallyTagged>());
    assert!(
        v.iter().any(|x| matches!(x, Violation::NoOneOf { .. })),
        "an externally tagged enum's oneOf must survive the transform and be flagged, got {v:#?}"
    );
}
