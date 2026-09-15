//! **The durable marks and their readback** — AC-030's `retry_at(resets_at)`.
//!
//! > **AC-030** — GIVEN a `RateLimited { resets_at }` failure / WHEN the kernel handles it / THEN it
//! > writes a `retry_at(resets_at)` mark and exits 0; **it never sleeps until the reset**.
//! > *Verified by:* a unit test asserting exit plus a written mark and **zero elapsed sleep with the
//! > injected clock**; the next `redkiln run` resumes from the mark (AC-013's re-entry test).
//!
//! # 🛑 THE CARRIER IS NOT A READING HERE, AND ONE MODULE OVER IT IS
//!
//! Residual 10c is FREE, and its own forcing note separates the three marks: *"Two of the three ARE
//! forced: AC-039's machine residue is 'a test asserting the halt writes a **gate record**', which
//! makes AC-024's mark an `EventKind::GateOpened`; and design §5's row 18 puts '`resets_at` on
//! `rate_limited`', which makes AC-030's mark an `EventKind::RateLimited`."* This module builds the
//! second of those two, and its carrier is design §5's own row rather than an inference. **The
//! breaker's mark is the third and it is still ARMED**: no `EventKind` and no `RefusalClass` can
//! carry it, minting one is a record's act under ADR-0116 clause 5, and Slice 4d reported it.
//!
//! # 🛑 `resets_at` IS CARRIED, NEVER DERIVED, AND `None` IS NOT A DEFAULT
//!
//! AC-030's own *Note* is part of the record: *"`resets_at` is observed in the headroom recordings
//! … but it arrives on `rate_limit_event`, **not** on a failure result — the adapter must derive
//! `RateLimited { resets_at }` from the stream, and if it cannot, the mark degrades to a bare
//! `retry_after` with a **pack-declared** delay."*
//!
//! So: this crate never computes a reset instant — it has no clock, and
//! `rk_types::time::Timestamp` offers no constructor that could read one. A mark whose `resets_at`
//! is `None` records the **absence**, and the `retry_after` degradation the *Note* describes is the
//! **adapter's** with a **pack-declared** delay. Writing a delay here would be the code default
//! §7b.10's 🛑 forbids, one variant removed. The absence is written into the payload as JSON `null`
//! rather than dropped, so a reader of the journal can tell *"the platform named no reset"* from
//! *"the mark carries no such member"*.
//!
//! # 🛑 *"exits 0"* is not this module's, and *"resumes from the mark"* is not either
//!
//! The exit is a **process** claim and this module exits nothing. *(This sentence read *"and
//! `rk-kernel` is a library (residual 10d; `rk-cli` is Step 7d)"* until 2026-09-14. The parenthesis
//! stays for the CLI half; the library clause is no longer a reason no test in this crate can
//! measure a process exit, because operator decision **D-8** built a re-exec harness that reads a
//! **child's** — `tests/it/cross_process.rs`. It is pointed at AC-024 and AC-035 and **NOT** at
//! AC-030, and the reason is AC-030's own words below.)* The resume rides on AC-013's re-entry
//! test, which is **PROVISIONAL** and may not be counted — and AC-030's *Verified by* names *"the
//! next **`redkiln run`** resumes from the mark"*, a binary that does not exist before Step 7d, so
//! **D-8 routes both of AC-030's remaining halves to 7d**. Both are ARMED and both are named in
//! `super::activity`'s WHAT-IS-MEASURED table. What this module discharges is the mark's **write**
//! and its **readback**, and [`read_retry_at_marks`] exists so the second is a function rather than
//! a test scanning a `Vec` by hand.

use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

/// **AC-030's durable mark**, as a value.
///
/// One field, deliberately: the criterion names `retry_at(resets_at)` and nothing else, and a second
/// field would be a number this crate had to source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryAtMark {
    resets_at: Option<Timestamp>,
}

impl RetryAtMark {
    /// Build the mark from the reset the **failure carried**. There is deliberately no constructor
    /// that takes a delay, a duration or a clock.
    #[must_use]
    pub const fn carried(resets_at: Option<Timestamp>) -> Self {
        Self { resets_at }
    }

    /// The reset as the platform named it, and `None` when it named none.
    #[must_use]
    pub const fn resets_at(self) -> Option<Timestamp> {
        self.resets_at
    }

    /// The kind that carries this mark — **design §5 row 18**, not an inference.
    #[must_use]
    pub const fn event_kind() -> EventKind {
        EventKind::RateLimited
    }

    /// A stable tag for the mark itself, so a journal reader sees `retry_at` and not a bare
    /// `rate_limited` row that could be a reading rather than a mark.
    #[must_use]
    pub const fn kind(self) -> &'static str {
        "retry_at"
    }

    /// The mark as the event that carries it.
    #[must_use]
    pub fn to_event(self) -> Event {
        Event::new(
            Self::event_kind(),
            serde_json::json!({
                "kind": self.kind(),
                "resets_at": self.resets_at.map(Timestamp::unix_seconds),
            }),
        )
    }
}

/// Every `retry_at` mark the journal holds, in append order.
///
/// **A `rate_limited` row that is not a mark is skipped rather than defaulted.** Design §5 gives
/// `rate_limited` to both the failure event and the mark; `kind = "retry_at"` is what tells them
/// apart, and a row without it is the failure's, not this module's. That is the same shape
/// `crate::workflow::fsm`'s `gate_opened` arm takes for its two producers, and the reason is the
/// same: one kind, two propositions, and a fold that guesses between them is a fold that can be
/// wrong silently.
#[must_use]
pub fn read_retry_at_marks(events: &[Event]) -> Vec<RetryAtMark> {
    events
        .iter()
        .filter(|e| e.kind == RetryAtMark::event_kind())
        .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some("retry_at"))
        .map(|e| {
            RetryAtMark::carried(
                e.payload
                    .get("resets_at")
                    .and_then(serde_json::Value::as_i64)
                    .map(Timestamp::from_unix_seconds),
            )
        })
        .collect()
}
