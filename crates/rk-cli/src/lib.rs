//! **`rk-cli` — the composition root**, and the library the integration tests link.
//!
//! The `redkiln` binary is `src/main.rs`; it installs the handler and the panic hook and exits
//! through [`code_for`]. Everything it calls lives here, because an integration test cannot link a
//! `[[bin]]` or name a `pub(crate)` item (ADR-0083) — so the surface the tests assert on and the
//! surface the binary runs are the same surface, rather than two that agree by inspection.
//!
//! **What is here is Slice 1** (`s7d` §7d.16 item 1): [`cli`], the router and its resume-shaped
//! denylist; [`exit`], the closed exit table and the one halt; and [`verbs`], eight stubs, one per
//! subcommand, each refusing as a configuration error. There is no `store`, `signal`, `env`,
//! `clock`, `config`, `adapters`, `repo` or `doctor` module: those are Slices 2–4's, and a module
//! declared here before its file exists is what an earlier draft of this slice did and could not
//! have compiled.
//!
//! **The two `std::process::exit` sites are both in `main.rs`, and at this slice there is one.**
//! The table exit is `main`'s; the panic hook's is Slice 2's, when `LOCK_HALTS` becomes reachable.
//! Checkpoint 7d counts exactly two, so the count is first satisfiable at Slice 2 and is
//! `[WILL EXIST]`-shaped by absence here.

pub mod cli;
pub mod exit;
pub mod verbs;

use crate::exit::{Exit, Halt};

/// The ONE mapping from what a verb returned to what the process exits with.
///
/// The table exit and the halt arm are **one call**, never two: an implementer who reaches for a
/// second `std::process::exit` to render the halt fails Checkpoint 7d's site count on an otherwise
/// correct binary (§7d.4).
#[must_use]
pub const fn code_for(result: &Result<Exit, Halt>) -> i32 {
    match result {
        Ok(exit) => exit.code(),
        Err(Halt::PoisonedLock { .. }) => Exit::PoisonedLockHalt.code(),
    }
}

/// The diagnostic a halt prints to stderr before the process exits.
///
/// A `std::sync::PoisonError` carries only the guard — `into_inner` / `get_ref` — and no panic
/// payload, so none is printed: the message says what stopped and what did not run, and nothing it
/// cannot know.
#[must_use]
pub fn render_halt(halt: &Halt) -> String {
    match halt {
        Halt::PoisonedLock { lock, verb } => format!(
            "redkiln: halt — the store lock is poisoned ({lock}): a writer panicked while holding \
             it; nothing after {verb} ran."
        ),
    }
}
