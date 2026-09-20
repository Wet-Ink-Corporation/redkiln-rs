//! **The one store acquisition** — ADR-0118 clauses 1, 3, 4, 5, 6; `s7d` §7d.4.
//!
//! Clause 1: *"A poisoned store lock halts the run. The run stops at the poisoned acquisition. No
//! `StepError` is constructed, nothing crosses a port boundary, no retry, no next activity, no gate
//! answer."* Clause 5 puts the acquisition at the composition root, and this is it: **every
//! `Mutex::lock` in `rk-cli` goes through the function below**, which is why Checkpoint 7d's
//! `.lock()` ban excludes exactly this module and nothing else (the two stdio handles aside — they
//! are not `Mutex`es, and a ban scoped wider than its warrant blocks valid work).
//!
//! **A poison is matched, never recovered.** There is no `into_inner`, no `catch_unwind`, no
//! `unwrap_or_else`. A poisoned lock means a writer panicked while holding it, so the store's
//! in-memory state is of unknown shape; the one safe thing to do with it is stop. The alternative
//! ADR-0118 explicitly rejected was appending a row on the way out — a write through a lock whose
//! invariants just failed.
//!
//! **Why an `Err`, and not a panic.** Clause 6 makes every verb return `Result<Exit, Halt>` so an
//! in-process caller — a test — can observe a returned value rather than a dead process, and so the
//! diagnostic and the exit are one call through `code_for` rather than a second `process::exit`.

use std::sync::{Arc, Mutex, MutexGuard};

use rk_ports::event_store::EventStore;

use crate::exit::Halt;

/// The name this lock carries in the halt diagnostic.
///
/// ADR-0118 clause 4 scopes the rule to two locks — this one, the `Arc<Mutex<dyn EventStore>>`, and
/// `rk-store`'s writer mutex. Only the first is observable here: the second recovers its own poison
/// today, which is RAISED as RT-9/R-7 and is not repaired by this stage.
const LOCK: &str = "store";

/// Acquire the event store, or halt.
///
/// **The `+ 'static` is not decoration.** `Arc<T: ?Sized>` carries no lifetime bound, so
/// `Ports.store` is an `Arc<Mutex<dyn EventStore + 'static>>`, while `MutexGuard<'a, T: ?Sized + 'a>`
/// would elaborate an elided form to `dyn EventStore + 'a` — and `MutexGuard` is invariant in `T`,
/// so the two do not unify and the elided spelling does not compile. `rk-kernel`'s own shipped
/// acquisition writes it out for the same reason (`Runtime::store_guard`). §7d.4 gives the signature
/// without it; that is a stage-file defect, RAISED and not edited.
///
/// # Errors
/// [`Halt::PoisonedLock`] naming this lock and the verb that was running. Never a guard over a
/// poisoned lock, and never a [`rk_types::error::StepError`] — a halt is not an error class.
pub fn acquire<'a>(
    store: &'a Arc<Mutex<dyn EventStore>>,
    verb: &'static str,
) -> Result<MutexGuard<'a, dyn EventStore + 'static>, Halt> {
    store.lock().map_or(
        Err(Halt::PoisonedLock { lock: LOCK, verb }),
        // The `Ok` arm binds the guard and hands it on. The `Err` arm does not bind the
        // `PoisonError` at all: binding it is the first step towards `into_inner`, and the type is
        // matched here precisely so that nothing can be taken out of it.
        Ok,
    )
}
