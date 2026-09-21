//! **The `Harness` the `doctor` path is handed** — it panics on `dispatch` and names the rule that
//! reached it. `s7d` §7d.1's `src` list, §7d.8, **D18-7**.
//!
//! # This file is complete, and it is not a skeleton
//!
//! Every other module this slice's skeleton cycle creates carries a signature whose body a later
//! cycle fills. This one does not: **panicking on `dispatch` IS its whole behaviour**, and a
//! version of it that did anything else would be the thing it exists to prevent.
//!
//! # Why a panic and not a returned error
//!
//! AC-414's THEN is *"zero harness dispatches are issued and zero model tokens are spent"*, and its
//! Verified-by names the instrument: *"the `doctor` integration test injects a `HarnessPort` fake
//! that fails the test on any dispatch"*. [`rk_ports::harness::Harness::dispatch`] returns
//! [`DispatchOutcome`] and **no `Result`** — by design, because ADR-0099 clause 1 makes the record
//! unconditional (AC-360: *"no dispatch path exists that can terminate without appending that
//! event"*). So there is no error arm to return, and a fabricated `DispatchOutcome` would be a
//! dispatch that SUCCEEDED as far as every caller could tell. The panic is the only outcome that
//! cannot be mistaken for one.
//!
//! # It names the rule, and the reason is a failure this crate has already paid for
//!
//! A bare `panic!("no dispatch")` would tell a reader that `doctor` dispatched and nothing about
//! **which** of the thirty-eight rules did it — and the rules run in table order over one `Ctx`, so
//! the id is the only thing that narrows it. [`NoDispatchHarness::during`] is set by the rule loop
//! before each `check` and read here.
//!
//! # 🛑 It is in `src/`, not behind `#[cfg(test)]`, and that is deliberate
//!
//! It is handed to the **shipped** binary's `doctor` path (`s7d` §7d.8: *"the root builds the
//! `doctor` path's `Ports` — `harness: NoDispatchHarness`"*), because AC-414's *"a CI assertion
//! that the `doctor` code path does not link a live adapter"* is about the real binary. A
//! test-only fake would leave the shipped `doctor` holding a real harness and would make the
//! assertion true of the tests alone.

use std::sync::Mutex;

use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;

/// A `Harness` that cannot dispatch, and says which rule tried.
///
/// `Mutex<Option<String>>` rather than a plain field: [`Harness`] takes `&self`, the trait is
/// `Send + Sync`, and the value is written by the rule loop between checks. `parking` is not
/// wanted here — the lock is held for one assignment and never across a call.
#[derive(Debug, Default)]
pub struct NoDispatchHarness {
    during: Mutex<Option<String>>,
}

impl NoDispatchHarness {
    /// Name the rule about to run, so a dispatch from inside it is attributable.
    ///
    /// A poisoned lock is **ignored** rather than propagated: this is a diagnostic aid, and a
    /// `doctor` run that failed because its panic-message bookkeeping could not take a lock would
    /// be a worse outcome than one whose panic says `<unknown rule>`. That is the opposite of
    /// [`crate::store::acquire`]'s rule and for the opposite reason — nothing here holds state a
    /// later read depends on.
    pub fn entering(&self, rule_id: &str) {
        if let Ok(mut slot) = self.during.lock() {
            *slot = Some(rule_id.to_owned());
        }
    }

    /// Which rule was last entered, or `None` before the loop starts.
    #[must_use]
    pub fn during(&self) -> Option<String> {
        self.during.lock().ok().and_then(|slot| slot.clone())
    }
}

impl Harness for NoDispatchHarness {
    fn dispatch(&self, spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        // The card digest identifies the activity: `ActivitySpec` carries no activity id
        // [measured 2026-09-20, `crates/rk-types/src/activity.rs`:47-59 — `card_digest`, `tools`,
        // `schema`, `timeout_secs`, `inputs`], and a digest is what the dispatch would have sent.
        panic!(
            "AC-414: `doctor` issued a harness dispatch. Every rule is configuration-time and \
             zero-dispatch (AC-098), so this is a rule reaching a model, not a port wired wrong. \
             Rule: {}. Card digest: {:?}.",
            self.during().unwrap_or_else(|| "<unknown rule>".to_owned()),
            spec.card_digest
        )
    }
}
