//! **AC-025.** *"GIVEN a gate whose evidence-producing activity ran out of any budget / WHEN the
//! gate is evaluated / THEN the result is `Err(BudgetExhausted)` and the gate does not close."*
//!
//! *Verified by:* a unit test per budget asserting the gate does not close; **a negative fixture
//! proving no code path maps an exhausted budget to `pass`.**
//!
//! This file must FAIL to compile. If it ever compiles, there is a path from an exhausted budget to
//! a closed gate — either a conversion (`BudgetExhausted` into a verdict) or a hand-written verdict
//! minted outside the module that owns `evaluate_gate` — and AC-025's THEN becomes a property of
//! one function's body rather than of the type. The three per-counter cases in `tests/it/budget.rs`
//! and the nine-row negative table beside them are the runtime half; this is the half they cannot
//! reach, because a `From` impl is a *compilable* program that passes all of them.
//!
//! # This is taken IN ADDITION TO the register's instrument, never in substitution for it
//!
//! AC-025's *Verified by* says **"a negative fixture"** and does not say `trybuild`. §7b.9:829-830
//! is explicit in the other direction — *"assigning `trybuild` to a criterion whose register
//! instrument is a unit test changes an instrument, which is the one thing a stage file may not
//! do"* — so the register's own negative fixture is built as a **runtime table** in
//! `tests/it/budget.rs` (nine ledgers × two evidence sets, asserting that no combination produces a
//! closed gate while any counter has run out), and this file is an additional, stronger reading of
//! the same clause. Neither replaces the other.
//!
//! # What this case does NOT pin, named rather than assumed
//!
//! Nothing here is asserted by any Checkpoint 7b line. §7b.4's case table has six rows and none of
//! them is this file; §7b.14's by-path loop (:1580-1589) enumerates the same six. So this file's
//! absence, and any drift in its `.stderr`, would be caught by nothing in the checkpoint — the gap
//! is recorded here and in `tests/it/ui.rs`, on `ac020_counters_never_share.rs`'s precedent.
//!
//! It also does not pin the ORDER of the check inside `evaluate_gate`. That the exhaustion branch
//! runs first and returns is a property of that function's body, asserted by the runtime table.

use rk_kernel::budget::{BudgetExhausted, Counter, GateVerdict};

fn main() {
    let exhausted = BudgetExhausted {
        counter: Counter::Attempts,
    };
    // No `From<BudgetExhausted>`, so an exhausted budget cannot become a verdict of any kind — a
    // `?` in a later gate evaluator cannot quietly turn this refusal into one.
    let _passed: GateVerdict = exhausted.into();
    // `Closed` is `#[non_exhaustive]`, so the closing verdict cannot be minted outside the crate
    // that owns `evaluate_gate`. `GateVerdict::closed()` is `pub(in crate::workflow)`.
    let _minted = GateVerdict::Closed;
    // And neither can the other one: a caller cannot hand itself a verdict at all.
    let _open = GateVerdict::Open {
        reason: String::new(),
    };
}
