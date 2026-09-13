//! The must-fail fixtures for AC-129 / AC-131 / AC-132 / AC-157 / AC-158.
//!
//! Every one lives HERE, inline, and never in `src/`. Two reasons, both measured failures one
//! level up:
//!   * a fixture read from a data file panics with `No such file or directory` and reads as a
//!     behavioural red — so there are no data files, only Rust types and `json!` literals;
//!   * a fixture type that reached `registry::EXPORTED` would make the positive half unable to go
//!     green and would commit a violating schema into the golden snapshot.
//!
//! Each fixture is built so it fires EXACTLY ONE rule. They all carry `deny_unknown_fields` except
//! `OpenObject`, and all declare required-before-optional except `OptionalBeforeRequired` — so no
//! fixture co-fires and a red always names its own cause.

#![allow(dead_code)]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Fires `OpenObject`: no `deny_unknown_fields`, so `additionalProperties: false` is absent.
/// `additionalProperties: false` is NOT automatic in schemars — it comes from that attribute.
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OpenObject {
    pub a_req: String,
}

/// An untagged union, used only as a component of `NestedUnion` below.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(untagged, deny_unknown_fields)]
pub enum StringOrInt {
    S(String),
    I(i64),
}

/// Fires `NestedUnion`: an `Option<T>` (one union) whose inner schema is itself a union.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NestedUnion {
    pub f: Option<StringOrInt>,
}

/// Fires `OptionalBeforeRequired`.
///
/// The field NAMES are chosen so that alphabetical order would be COMPLIANT and declaration order
/// is VIOLATING. If `preserve_order` were ever silently dropped, the properties would sort to
/// `[a_req, z_opt]`, this fixture would quietly go green, and AC-158 would be pinned by nothing.
/// `preserve_order_is_in_force` below is the guard that makes that impossible to miss.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct OptionalBeforeRequired {
    #[serde(default)]
    pub z_opt: String,
    pub a_req: String,
}

macro_rules! seventeen_union_fields {
    ($name:ident, $ty:ty) => {
        /// Fires `UnionCap { count: 17, cap: 16 }`.
        #[derive(Serialize, Deserialize, JsonSchema)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            pub f01: $ty,
            pub f02: $ty,
            pub f03: $ty,
            pub f04: $ty,
            pub f05: $ty,
            pub f06: $ty,
            pub f07: $ty,
            pub f08: $ty,
            pub f09: $ty,
            pub f10: $ty,
            pub f11: $ty,
            pub f12: $ty,
            pub f13: $ty,
            pub f14: $ty,
            pub f15: $ty,
            pub f16: $ty,
            pub f17: $ty,
        }
    };
}

// 17 explicit unions.
seventeen_union_fields!(Union17Explicit, StringOrInt);
// 17 nullable `Option<T>`s and no explicit union anywhere. Under the vendor's count these are
// seventeen union-typed parameters; under a per-node branch count they are ZERO. AC-157's
// 2026-09-11 correction is exactly this divergence, and this fixture is what pins it.
seventeen_union_fields!(Union17Options, Option<String>);

/// Fires `OptionalCap { count: 25, cap: 24 }` — and ONLY that.
///
/// The fields are `#[serde(default)] String`, deliberately NOT `Option<String>`: twenty-five
/// `Option`s would breach the union cap as well, both violations would fire, and the test could
/// not say which rule caught it.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, default)]
#[derive(Default)]
pub struct Optional25 {
    pub f01: String,
    pub f02: String,
    pub f03: String,
    pub f04: String,
    pub f05: String,
    pub f06: String,
    pub f07: String,
    pub f08: String,
    pub f09: String,
    pub f10: String,
    pub f11: String,
    pub f12: String,
    pub f13: String,
    pub f14: String,
    pub f15: String,
    pub f16: String,
    pub f17: String,
    pub f18: String,
    pub f19: String,
    pub f20: String,
    pub f21: String,
    pub f22: String,
    pub f23: String,
    pub f24: String,
    pub f25: String,
}

/// A `kind`-discriminated enum. `schemars` 1.2.2 renders `#[serde(tag = "kind")]` as `oneOf`
/// (PIA-005, ERRATA E-0084-2), so an UNTRANSFORMED derive of this is the mutant for
/// `no_oneof_at_any_node`.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TaggedKind {
    Alpha { a: i32 },
    Beta { b: String },
}

/// An EXTERNALLY tagged enum. `schemars` renders this as a `oneOf` whose branches are objects
/// keyed by the VARIANT NAME — `{"Ok": ...}` / `{"Err": ...}` — not by a required `kind`.
///
/// This fixture exists because a mutant survived without it. Making `KindOneOfToAnyOf` over-broad
/// (rewriting EVERY `oneOf`, not only the kind-discriminated shape) left all sixteen tests green:
/// nothing in the set carried a `oneOf` that the transform was supposed to LEAVE ALONE, so a
/// transform that laundered `NoOneOf` violations instead of fixing two schemas was indistinguishable
/// from the correct one. ADR-0104 decision 4: a survivor blocks.
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ExternallyTagged {
    Alpha(i32),
    Beta(String),
}

/// The guard against a silent `preserve_order` regression (see `OptionalBeforeRequired`).
#[test]
fn preserve_order_is_in_force() {
    let s = serde_json::json!({ "b": 1, "a": 2 }).to_string();
    assert_eq!(
        s, r#"{"b":1,"a":2}"#,
        "serde_json must carry `preserve_order`, or AC-158 is unobservable and the \
         OptionalBeforeRequired fixture silently passes"
    );
}
