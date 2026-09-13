//! AC-160's compile-fail fixture.
//!
//! The criterion's Verified-by is *"a `trybuild` compile-fail fixture with an unannotated field,
//! failing with the tier-missing diagnostic"* — so the expected stderr is pinned as a file and the
//! **diagnostic itself** is asserted, not merely the failure. A fixture that failed for a typo
//! would otherwise read as the criterion being met.

#[test]
fn a_state_field_without_a_tier_is_a_compile_error() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/tier_missing.rs");
}
