//! **AC-007's third instrument.** *"An exhaustive `match` on the divergence outcome with no
//! wildcard arm, and a **`trybuild` test proving the wildcard cannot be reintroduced**."*
//!
//! This file must FAIL to compile.
//!
//! # What this case pins, and what it cannot
//!
//! *"Proving the wildcard cannot be reintroduced"* has **no literal construction**, and that is
//! stated here rather than papered over: adding `_ => …` to a `match` compiles, here as anywhere,
//! and `clippy::wildcard_enum_match_arm` is a `restriction` lint that is not in
//! `[workspace.lints.clippy]`. `tests/ui/ac028_steperror_wildcard.rs` records the same gap for the
//! retry policy's eleven variants and takes the same two readings; this file follows it.
//!
//! What IS constructible, and is what the two statements below assert:
//!
//! 1. **a `match` over `Divergence` that does not cover every variant does not compile** (`E0004`) —
//!    which is the property that makes AC-007's THEN true. A fourth divergence outcome cannot be
//!    added without every site deciding what it means, and `Divergence::halt` is such a site;
//! 2. **a wildcard placed after an exhaustive match is DEAD**, under this fixture's own
//!    `#[deny(unreachable_patterns)]`. That proves something about Rust and **nothing about
//!    `rk-kernel`**, and it is labelled as such: `rk-kernel`'s own shipped `halt` carrying three
//!    arms and no `_` is asserted by the sweep in `tests/it/divergence.rs` over
//!    `Divergence::ALL_TAGS` and by `Divergence::halt`'s own exhaustiveness.
//!
//! # 🛑 ASSERTED BY NO CHECKPOINT LINE
//!
//! §7b.4's case table has six rows and none is this file; §7b.14's by-path loop (:1580-1589)
//! enumerates the same six. `ac007_replay_divergence_halts` therefore asserts this file and its
//! `.stderr` **by path**, which catches a deletion but not `.stderr` drift. The gap is recorded in
//! `tests/it/ui.rs`'s header and is owed to `PROGRESS.md` §2.

#![deny(unreachable_patterns)]

use rk_kernel::workflow::fsm::Divergence;

fn classify(d: &Divergence) -> &'static str {
    // 1. `TerminalState` is not covered: `E0004`.
    match d {
        Divergence::Decision { .. } => "decision",
        Divergence::NextAction { .. } => "next_action",
    }
}

fn reintroduced(d: &Divergence) -> &'static str {
    // 2. Exhaustive already, so the wildcard is dead under this file's own deny.
    match d {
        Divergence::Decision { .. } => "decision",
        Divergence::NextAction { .. } => "next_action",
        Divergence::TerminalState { .. } => "terminal_state",
        _ => "something new, and nobody decided what it means",
    }
}

fn main() {
    let d = Divergence::NextAction {
        recorded: String::new(),
        recomputed: String::new(),
    };
    let _ = classify(&d);
    let _ = reintroduced(&d);
}
