//! The four `trybuild` must-fail cases, under **one** harness test.
//!
//! Two of them **pin the reason** of the two `compile_fail` doctest fences in `src/lib.rs`, and
//! that is the job the fences measurably cannot do: a bare `compile_fail` is green whenever
//! compilation fails for ANY reason, and an expected error code on a fence is **inert at this
//! pin** -- a three-case probe crate on rustc 1.97.1 and on 1.98.1 reported `ok` for
//! `compile_fail,E0277` failing with E0308, for `compile_fail,E9999` (a code that does not exist)
//! failing with E0308, and for `compile_fail,E0277` failing with E0433 [measured 2026-09-13]. Each
//! case here commits a `.stderr` snapshot, so a case that starts failing for a DIFFERENT reason
//! goes red.
//!
//! The other two are criteria's own named fixtures: AC-483's `trait GitPort` (register :6958) and
//! AC-528's *"a compile-fail fixture adding one"* async method (register :7162-7163).
//!
//! **This test's NAME is load-bearing.** Checkpoint 7a filters on it, and cargo's `--` filter is a
//! substring match over TEST names -- not the crate, the target or the file. The house convention
//! names a `trybuild` harness after its RULE, not after the tool: `rk-git`'s is
//! `reconcile_runs_at_every_entry` and `rk-store`'s is
//! `a_state_field_without_a_tier_is_a_compile_error`, and neither name nor its module contains the
//! substring `trybuild` -- so a filter spelled `-- trybuild` selects ZERO tests and cargo still
//! exits 0 (`0 passed; N filtered out`). This name carries the token the checkpoint filters on.

#[test]
fn trybuild_pins_the_reason_each_port_fixture_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/arc_dyn_port_rejects_rc.rs");
    t.compile_fail("tests/ui/bare_arc_store_has_no_mut.rs");
    t.compile_fail("tests/ui/git_port_is_not_a_port.rs");
    t.compile_fail("tests/ui/async_fn_in_a_port_trait.rs");
}
