//! AC-481 — the fake-per-port parity assertion, as the register amended it on 2026-09-13.
//!
//! **The three clauses that bind** (register :6918-6923): the sets are derived from source and
//! never from a hand-maintained inventory; the equality is asserted **as a number**, not as a spot
//! check; and a port trait added without a fake **fails the suite naming the missing fake, by
//! name, from the same source parse**.
//!
//! *The registration attributes the 2026-09-11 GIVEN named were never built and cannot be:
//! `#[port]` and `#[fake_for(Trait)]` are not Rust attributes, an attribute macro needs a crate
//! with `proc-macro = true`, and design §9.1 -- the crate-name authority -- names no macro crate
//! among the ten. The register replaced the MECHANISM, not the criterion.*
//!
//! **What the amendment costs, restated rather than smoothed:** an attribute registration is a
//! DECLARED set and a source parse is an INFERRED one, so the parse can disagree with authorial
//! intent in two directions -- a `pub trait` the author did not mean to register is counted, and a
//! fake the parse cannot see is not -- and neither direction is visible in the count alone,
//! because both sides can move together. **The PIA-059 fixture is what holds the parse honest.**

use std::collections::BTreeSet;

use crate::source_parse::Source;

#[test]
fn port_fake_parity() {
    let source = Source::parse();
    let traits = source.traits.clone();
    let fakes = source.fakes();

    let missing: BTreeSet<_> = traits.difference(&fakes).collect();

    // TRAP 1: this literal spelling is load-bearing. Checkpoint 7a and `ci.yml` both grep for
    // `traits.len() == fakes.len()` (design:1783 makes it a CI structural assertion by that
    // name), and the idiomatic `assert_eq!(traits.len(), fakes.len())` does not contain that
    // string. One statement carries both the literal the grep needs and clause 3's by-name
    // message. Do NOT wrap the comparison across a line break to shorten it: that is the one edit
    // that satisfies `cargo fmt --all --check` and fails the grep.
    assert!(
        traits.len() == fakes.len(),
        "port traits with no fake: {missing:?}"
    );
}
