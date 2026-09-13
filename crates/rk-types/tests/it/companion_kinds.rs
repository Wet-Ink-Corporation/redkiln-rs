//! `companion_kind_has_registered_schema` — AC-143, on decision #3 of 2026-09-11.
//!
//! The cases run in a deliberate order. `the_six_kinds_are_exactly_these` asserts the SET first,
//! by name and in order; only once that is green does a failure in
//! `companion_kind_has_registered_schema` mean what it says. A `#[serde(rename)]` typo would fail
//! the first case with a different message, so the two are never confusable.

use rk_types::companion::{CompanionError, CompanionKind, PHASE_2_KINDS, write_companion};
use rk_types::schema::lint::lint;

/// Case 1 — the set. AC-143's six, in AC-143's order.
#[test]
fn the_six_kinds_are_exactly_these() {
    let names: Vec<&str> = CompanionKind::ALL.iter().map(|k| k.wire_name()).collect();
    assert_eq!(
        names,
        vec![
            "_integration",
            "_grounding",
            "_decomposition",
            "_review",
            "_ledger",
            "ledger"
        ],
        "exactly six: AC-143's five plus the census's bare `ledger` kind. `_ledger` and `ledger` \
         are different kinds — the preamble's decision-7 row lists only the five underscore names, \
         and the register owns the wording (ADR-0092)."
    );
    assert_eq!(CompanionKind::ALL.len(), 6);
}

/// Case 2 — the discriminator. Every kind has its own registered sidecar schema, and each one
/// passes the closed-output rules (AC-129..AC-133, AC-157, AC-158 apply to companions too).
#[test]
fn companion_kind_has_registered_schema() {
    for kind in CompanionKind::ALL {
        let name = kind.wire_name();
        let schema = kind
            .registered_schema()
            .unwrap_or_else(|| panic!("companion kind `{name}` has no registered sidecar schema"));
        let violations = lint(&schema);
        assert!(
            violations.is_empty(),
            "companion `{name}`'s sidecar schema breaches the closed-output rules: {violations:#?}"
        );
    }
}

/// Case 3 — a companion kind with no registered schema cannot be written at all.
#[test]
fn every_registered_kind_is_writable() {
    for kind in CompanionKind::ALL {
        assert!(
            write_companion(kind.wire_name()).is_ok(),
            "`{}` is registered, so a write must resolve",
            kind.wire_name()
        );
    }
}

/// Case 4 — the eight Phase-2 census kinds are REFUSED, with their own reason. `redkiln-migrate`
/// lifts them as opaque prose (ADR-0075 clause 6); a write against the Phase-1 registry is never
/// accepted as untyped.
#[test]
fn the_eight_phase_two_kinds_are_refused() {
    assert_eq!(
        PHASE_2_KINDS.len(),
        8,
        "closure, design, design-review, slices, storymap, implementation, plan, retrospective"
    );
    for kind in PHASE_2_KINDS {
        assert_eq!(
            write_companion(kind),
            Err(CompanionError::Phase2Kind {
                kind: (*kind).to_owned()
            }),
            "`{kind}` is a Phase-2 kind and must be refused as one, not parsed as unknown"
        );
    }
}

/// Case 5 — a kind that is neither is refused as unknown, never written untyped.
#[test]
fn an_unregistered_kind_is_refused() {
    assert_eq!(
        write_companion("_invented"),
        Err(CompanionError::UnknownKind {
            kind: "_invented".to_owned()
        })
    );
}
