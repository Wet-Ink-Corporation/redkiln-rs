//! Shared harness for the integration cases.

use rk_types::schema::lint::Violation;
use serde_json::Value;

/// Derive a fixture type's EXPORTED schema — the same door `registry::EXPORTED` uses, so a fixture
/// is judged on exactly the shape a real artifact would be.
#[must_use]
pub fn exported<T: schemars::JsonSchema>() -> Value {
    rk_types::schema::export::<T>()
}

/// Assert a fixture fires EXACTLY the expected violation and nothing else.
///
/// `assert!(!violations.is_empty())` would be satisfied by a fixture firing the WRONG rule, which
/// is how a caps test passes while the caps are unchecked.
pub fn assert_only(got: &[Violation], want: &Violation, fixture: &str) {
    assert_eq!(
        got,
        std::slice::from_ref(want),
        "{fixture}: expected exactly one violation, {want:?}"
    );
}
