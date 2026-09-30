//! **`rk-cli` — the composition root**, and the library the integration tests link.
//!
//! The `redkiln` binary is `src/main.rs`; it installs the handler and the panic hook and exits
//! through [`code_for`]. Everything it calls lives here, because an integration test cannot link a
//! `[[bin]]` or name a `pub(crate)` item (ADR-0083) — so the surface the tests assert on and the
//! surface the binary runs are the same surface, rather than two that agree by inspection.
//!
//! **What is here is Slices 1 and 2** (`s7d` §7d.16 items 1 and 2). Slice 1: [`cli`], the router
//! and its resume-shaped denylist; [`exit`], the closed exit table and the one halt; [`verbs`], the
//! router's arms. Slice 2: [`env`] the one environment read, [`clock`] the one wall-clock read,
//! [`config`] the closed `ConfigFile`, [`adapters`] the resolver, [`repo`] H-3's discovery,
//! [`store`] the one acquisition, [`signal`] the handler, and `verbs::init`.
//!
//! **Amended 2026-09-20, Step 7d Slice 4 cycle 2.** The sentence that stood here said *"There is
//! **no `doctor` module** — that is Slice 4's, and a module declared here before its file exists is
//! what an early draft of Slice 1 did and could not have compiled."* Slice 4 is that slice, and the
//! two modules it adds are declared below: [`doctor`] — the `Ctx`, the rule table, and the
//! rebuild / probe / roles signatures — and [`no_dispatch`], the `Harness` the `doctor` path is
//! handed, which panics on dispatch and names the rule that reached it. The rule the old sentence
//! carried outlives the list it was written about and is restated: **a module is declared when its
//! file exists**, and `doctor`'s exists now.
//!
//! 🚧 **`doctor` is a SKELETON at this cycle and claims no test.** `doctor::rules::RULES` is empty,
//! and §7d.14 records that over an empty rule table AC-414's per-fixture join, AC-415's per-id join
//! and SM-121's dispatch count are all **vacuous** — so none of the three is claimed here. Its
//! module header carries the cycle-by-cycle table of what fills each unfilled body.
//!
//! **The two `std::process::exit` sites are both in `main.rs`, and as of Slice 2 there are two** —
//! the table exit and the panic hook's `70`. Checkpoint 7d counts exactly two and requires both to
//! be in that file.

pub mod adapters;
pub mod change_deriver;
pub mod cli;
pub mod clock;
pub mod config;
pub mod doctor;
pub mod env;
pub mod exit;
pub mod no_dispatch;
pub mod processes;
pub mod release_tag;
pub mod repo;
pub mod review_pass;
pub mod signal;
pub mod store;
pub mod verbs;
pub mod verify_pass;

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

/// Is this panic payload the kernel's poisoned-store-lock halt?
///
/// **Takes the payload, not the `PanicHookInfo`.** §7d.4 and §7d.10 both spell the parameter as
/// `&PanicHookInfo`, but §7d.10's own test description hands this function `catch_unwind`'s payload
/// — a `Box<dyn Any + Send>` — and `PanicHookInfo` has no public constructor, so no test could build
/// one. The two sentences are incompatible; the payload form is the one that can be tested, and the
/// hook adapts with `info.payload()`. RAISED as a stage-file defect, not edited.
///
/// **It matches `rk_kernel::runtime::LOCK_HALTS` and never a second copy of that string.** That is
/// the whole reason §7d.1's eighth kernel edit exports the constant: two copies can drift, and the
/// drift would be invisible — every test green and the hook silently leaving the process at `101`.
///
/// The payload of `.expect(LOCK_HALTS)` is a `String` (`"{msg}: {PoisonError:?}"`), and a bare
/// `panic!("…")` with a literal is a `&'static str`; both are downcast, because which one arrives
/// depends on how the panic was raised and not on what it means.
#[must_use]
pub fn is_lock_halt(payload: &(dyn std::any::Any + Send)) -> bool {
    let needle = rk_kernel::runtime::LOCK_HALTS;
    payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&'static str>().copied())
        .is_some_and(|text| text.contains(needle))
}
