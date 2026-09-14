//! **AC-020.** *"GIVEN the three budgets — attempts per activity, tokens per item, wall clock per
//! run / WHEN one of them is consumed or exhausted / THEN neither of the other two changes."*
//!
//! *Verified by:* a unit test per pair (three cases) asserting independence; **a `trybuild`
//! separation test — the three are distinct newtypes and no `From`/`Add` impl crosses them, so a
//! shared counter does not compile.**
//!
//! This file must FAIL to compile. If it ever compiles, the three counters have become one: a
//! caller can add attempts to attempts, convert attempts into tokens, fold a column of them, or
//! average them as floats — and AC-020's THEN is then a convention rather than a type fact. The
//! three pair-cases in `tests/it/budget.rs` are the runtime half; this is the half they cannot
//! reach, because a shared counter is a *compilable* program that passes them only by accident.
//!
//! # What this case does NOT pin, named rather than assumed
//!
//! Nothing here is asserted by any Checkpoint 7b line. §7b.9:824-830 names AC-020's separation test
//! among *"four `trybuild` compile-fail obligations"*, but §7b.4's case table has no AC-020 row and
//! §7b.14's by-path loop (:1580-1589) enumerates exactly six case files, none of them this one. So
//! **this file's absence, and its `.stderr`'s drift, would go undetected by the checkpoint** — the
//! gap is recorded here and in `tests/it/ui.rs` rather than left for a later reader to find.
//!
//! Built on `ac050_rigor_is_not_arithmetic.rs`'s exact model: four statements, four distinct
//! compile errors, one committed `.stderr` naming all four.

use rk_kernel::budget::{Attempts, Tokens};

fn main() {
    // No `Add`. A counter that can be added to is a counter that can be added to ANOTHER counter's
    // total by a later refactor; `Attempts::plus` is `pub(in crate::workflow)` for this reason.
    let _sum = Attempts::new(1) + Attempts::new(1);
    // No `From` crossing them, so attempts cannot become tokens.
    let _crossed: Tokens = Attempts::new(1).into();
    // No `Sum`, so no folding a column of them into a shared total.
    let _total: Attempts = [Attempts::new(1), Attempts::new(2)].into_iter().sum();
    // No `Into<f64>`, so no averaging and no conversion into a magnitude with no counter attached.
    let _avg: f64 = Attempts::new(1).into();
}
