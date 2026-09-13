//! `union_and_optional_caps` — AC-157 (≤ 16 union-typed) and AC-132 (≤ 24 optional).

use crate::fixtures::{Optional25, Union17Explicit, Union17Options};
use crate::support::{assert_only, exported};
use rk_types::registry;
use rk_types::schema::lint::{OPTIONAL_CAP, UNION_CAP, Violation, lint};

#[test]
fn union_and_optional_caps() {
    let all = registry::export_all();
    assert!(
        !all.is_empty(),
        "the exported set is empty — this test would assert nothing"
    );

    for (name, schema) in all {
        for v in lint(&schema) {
            assert!(
                !matches!(
                    v,
                    Violation::UnionCap { .. } | Violation::OptionalCap { .. }
                ),
                "exported schema `{name}` breaches a vendor cap: {v:?}"
            );
        }
    }
}

#[test]
fn seventeen_explicit_unions_is_caught() {
    let v = lint(&exported::<Union17Explicit>());
    assert_only(
        &v,
        &Violation::UnionCap {
            count: 17,
            cap: UNION_CAP,
        },
        "Union17Explicit",
    );
}

#[test]
fn seventeen_options_is_caught() {
    // The divergence AC-157's correction is about: zero under a per-node branch count.
    let v = lint(&exported::<Union17Options>());
    assert_only(
        &v,
        &Violation::UnionCap {
            count: 17,
            cap: UNION_CAP,
        },
        "Union17Options",
    );
}

#[test]
fn twenty_five_optionals_is_caught() {
    let v = lint(&exported::<Optional25>());
    assert_only(
        &v,
        &Violation::OptionalCap {
            count: 25,
            cap: OPTIONAL_CAP,
        },
        "Optional25",
    );
}
