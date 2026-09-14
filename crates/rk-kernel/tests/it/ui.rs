//! The one compile-fail harness, named after its rule and not after the tool (`s7b` §7b.4).
//!
//! `tests/ui/` is this glob's root and therefore holds **compile-FAIL cases only** — a file that
//! must compile placed there makes `trybuild` report an unexpected success and turns the harness
//! red. Each case commits a `.stderr` snapshot, so a case that starts failing for a **different**
//! reason goes red rather than silently continuing to pass.
//!
//! # What is here, and what §7b.4's table still owes
//!
//! Four of the six cases §7b.4:426-433 names are here: `ac041_selection_reads_no_clock.rs`,
//! `ac050_rigor_is_not_arithmetic.rs`, `ac061_required_set_is_not_constructible_outside_select.rs`
//! and `ac061_required_set_has_no_deserialize.rs`. **`ac480_generics_leak.rs` and
//! `stay_class_state_is_unconstructible.rs` are NOT written**: §7b.13 assigns both to **Slice 1**,
//! neither landed there, and neither is steps 1-4's. Checkpoint 7b asserts all six **by path** with
//! their `.stderr`, so the two absences are live and are reported rather than left to be discovered
//! by that assertion.

#[test]
fn trybuild_pins_the_reason_each_kernel_fixture_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/ac041_selection_reads_no_clock.rs");
    t.compile_fail("tests/ui/ac050_rigor_is_not_arithmetic.rs");
    t.compile_fail("tests/ui/ac061_required_set_is_not_constructible_outside_select.rs");
    t.compile_fail("tests/ui/ac061_required_set_has_no_deserialize.rs");
}
