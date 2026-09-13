//! The canonical serialiser and the three keys (ADR-0097; AC-014/AC-016/AC-019).

use rk_store::canon::{self, CanonError};
use rk_store::memo::{IdempotencyKey, StepKeyComponents, activity_key};
use rk_types::digest::Digest;
use serde_json::json;

fn d(n: u8) -> Digest {
    Digest::from_hex(&format!("{n:02x}").repeat(32)).expect("32 bytes of hex")
}

fn components() -> StepKeyComponents {
    StepKeyComponents {
        activity_id_at_version: "implement@3".into(),
        rigor: "recommended".into(),
        step_id: "green".into(),
        input_digests: vec![d(1), d(2)],
        card_or_tool: "card:abc".into(),
        kb_atom_digests: vec![d(3), d(4)],
        loop_iteration: 0,
        schema_version: "1".into(),
    }
}

// ---------------------------------------------------------------- the canonical serialiser

/// Key order is a property of the value, not of who wrote it. `serde_json` is pulled with
/// `preserve_order`, so without the sort the same value authored in two orders would compare
/// unequal under four criteria that call themselves equality tests.
#[test]
fn object_keys_are_sorted_so_authoring_order_does_not_leak() {
    let a = canon::canonical(&json!({"b": 1, "a": 2})).expect("canon");
    let b = canon::canonical(&json!({"a": 2, "b": 1})).expect("canon");
    assert_eq!(a, b);
    assert_eq!(String::from_utf8(a).expect("utf8"), r#"{"a":2,"b":1}"#);
}

/// Array order IS a property of the value and must survive.
#[test]
fn array_order_is_preserved() {
    let a = canon::canonical(&json!([1, 2, 3])).expect("canon");
    let b = canon::canonical(&json!([3, 2, 1])).expect("canon");
    assert_ne!(a, b, "array order was normalised away");
}

/// Floats are refused rather than formatted. See the module header: a refusal cannot disagree
/// across platforms the way any chosen float format eventually does.
#[test]
fn floats_are_refused_with_a_pointer_to_the_offending_node() {
    match canon::canonical(&json!({"outer": {"ratio": 1.5}})) {
        Err(CanonError::FloatNotRepresentable { pointer }) => {
            assert_eq!(
                pointer, "/outer/ratio",
                "the refusal does not locate the float"
            );
        }
        other => panic!("a float was serialised rather than refused: {other:?}"),
    }
}

/// Integers are not floats, and must not be caught by the refusal.
#[test]
fn control_integers_pass_through() {
    let out = canon::canonical(&json!({"n": -7, "m": 42})).expect("canon");
    assert_eq!(String::from_utf8(out).expect("utf8"), r#"{"m":42,"n":-7}"#);
}

/// LF only, and no trailing newline — a CRLF host must not serialise differently from an LF one.
#[test]
fn control_strings_escape_to_lf_and_there_is_no_trailing_newline() {
    let out = canon::canonical(&json!({"s": "a\r\nb"})).expect("canon");
    let text = String::from_utf8(out).expect("utf8");
    assert_eq!(text, r#"{"s":"a\r\nb"}"#);
    assert!(!text.ends_with('\n'));
}

// ---------------------------------------------------------------- the step memo key

/// **"Exactly these components, in this order."** The count is asserted against the record so the
/// struct cannot quietly grow a ninth field.
#[test]
fn the_step_key_has_exactly_eight_components() {
    assert_eq!(StepKeyComponents::COMPONENT_COUNT, 8);
}

/// One property case per component: changing any single one changes the key. This is AC-014 and
/// AC-019's enumeration, and it is what makes the key a key rather than a hash of some of it.
#[test]
fn every_component_participates_in_the_key() {
    let base = components().digest();

    let mut c = components();
    c.activity_id_at_version = "implement@4".into();
    assert_ne!(
        base,
        c.digest(),
        "component 1 (activity_id@version) is inert"
    );

    let mut c = components();
    c.rigor = "strict".into();
    assert_ne!(base, c.digest(), "component 2 (rigor) is inert");

    let mut c = components();
    c.step_id = "red".into();
    assert_ne!(base, c.digest(), "component 3 (step id) is inert");

    let mut c = components();
    c.input_digests = vec![d(1), d(9)];
    assert_ne!(base, c.digest(), "component 4 (input digests) is inert");

    let mut c = components();
    c.card_or_tool = "card:def".into();
    assert_ne!(base, c.digest(), "component 5 (card or tool) is inert");

    let mut c = components();
    c.kb_atom_digests = vec![d(3), d(9)];
    assert_ne!(base, c.digest(), "component 6 (kb atoms) is inert");

    let mut c = components();
    c.loop_iteration = 1;
    assert_ne!(base, c.digest(), "component 7 (loop iteration) is inert");

    let mut c = components();
    c.schema_version = "2".into();
    assert_ne!(base, c.digest(), "component 8 (schema_version) is inert");
}

/// Component 4's order is **declaration order** and matters; component 6's is **sorted** and does
/// not. Both are the record's words, and they differ.
#[test]
fn input_order_matters_and_kb_atom_order_does_not() {
    let mut swapped_inputs = components();
    swapped_inputs.input_digests = vec![d(2), d(1)];
    assert_ne!(
        components().digest(),
        swapped_inputs.digest(),
        "input digests are in DECLARATION order; swapping them must change the key"
    );

    let mut swapped_atoms = components();
    swapped_atoms.kb_atom_digests = vec![d(4), d(3)];
    assert_eq!(
        components().digest(),
        swapped_atoms.digest(),
        "KB atom digests are SORTED; the caller's order must not reach the key"
    );
}

/// The framing again, at the key's own level: two different component splits must not collide.
#[test]
fn components_cannot_absorb_each_others_boundaries() {
    let mut a = components();
    a.rigor = "ab".into();
    a.step_id = "c".into();
    let mut b = components();
    b.rigor = "a".into();
    b.step_id = "bc".into();
    assert_ne!(a.digest(), b.digest());
}

/// Stable across calls, or nothing above means anything.
#[test]
fn the_key_is_stable() {
    assert_eq!(components().digest(), components().digest());
}

// ---------------------------------------------------------------- the other two keys

/// **A different formula.** ADR-0097 decision 2 says the activity-level key is "separately
/// derived", and design §3.5 was corrected so its two paragraphs agree. If one were implemented in
/// terms of the other they would eventually coincide, and AC-016 would be discharged by accident.
#[test]
fn the_activity_key_is_not_the_step_key() {
    let cards = vec![d(1), d(2)];
    assert_ne!(
        activity_key(&cards),
        components().digest(),
        "the activity key and the step key produced the same value"
    );
}

/// The activity key sorts its cards: reachability order is not a property of the activity.
#[test]
fn the_activity_key_sorts_its_cards() {
    assert_eq!(activity_key(&[d(1), d(2)]), activity_key(&[d(2), d(1)]));
}

/// **Three parts, never a fourth** — ADR-0097's forbids clause names the failure by name.
#[test]
fn the_idempotency_key_has_three_parts() {
    assert_eq!(IdempotencyKey::PART_COUNT, 3);
    let k = IdempotencyKey {
        item: "RK-S1".into(),
        activity_id_at_version: "implement@3".into(),
        attempt: 1,
    };
    let mut other = k.clone();
    other.attempt = 2;
    assert_ne!(k.digest(), other.digest(), "attempt does not participate");

    // And it is not the step key either.
    assert_ne!(k.digest(), components().digest());
}
