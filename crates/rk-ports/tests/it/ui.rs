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
//!
//! *(Amended 2026-10-15, s8i, ADR-0199, proposed; s8i refuting round 21's minor: the first sentence
//! counts the FIRST harness test. The file now holds two: the one above, with its four cases
//! (Checkpoint 7a's count, unchanged), and `trybuild_pins_the_binary_change_summary_construction_rule`,
//! with `s8i`'s two construction-rule cases (`change_summary_binary_construction_rule.rs` and
//! `change_summary_binary_struct_literal.rs`; Checkpoint 8i (8)). Six cases under two harness tests.)*

#[test]
fn trybuild_pins_the_reason_each_port_fixture_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/arc_dyn_port_rejects_rc.rs");
    t.compile_fail("tests/ui/bare_arc_store_has_no_mut.rs");
    t.compile_fail("tests/ui/git_port_is_not_a_port.rs");
    t.compile_fail("tests/ui/async_fn_in_a_port_trait.rs");
}

/// `s8i`, ADR-0199 decision 2 (proposed): the `Binary` payload of `ChangeSummary` is built only
/// through its one recorded checked constructor. The two cases' committed `.stderr` files name
/// each other route's reason (private fields for a struct literal and for a field read; no
/// `Default`, no `From`, no `Deserialize`), so a `pub` field or a second route goes red. The
/// struct literal is its own case because rustc reports its privacy error (E0451) only when type
/// checking succeeded. Its own harness test, so `trybuild_pins_the_reason_each_port_fixture_fails`
/// keeps its four cases (Checkpoint 7a). It supersedes, by name, `s8a`'s
/// `change_summary::binary_arm_is_not_constructible_outside_rk_ports` and its case
/// `tests/ui/change_summary_binary_not_constructible.rs`.
#[test]
fn trybuild_pins_the_binary_change_summary_construction_rule() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/change_summary_binary_construction_rule.rs");
    t.compile_fail("tests/ui/change_summary_binary_struct_literal.rs");
}
