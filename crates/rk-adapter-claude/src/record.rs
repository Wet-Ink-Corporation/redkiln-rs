//! The record and `dispatch_completed` — the one row the adapter appends on EVERY termination path —
//! and `rate_limit_reading`, the second row it appends on SOME.
//!
//! Through the store it was handed, and the warning derived from it on read
//! (`s7c` §7c.11; ADR-0126; ADR-0163 clauses 2 and 3; AC-360, AC-368, AC-385, AC-227).
//!
//! **The two kinds.** `dispatch_completed` is unconditional: exactly one per return of
//! `dispatch`. `rate_limit_reading` ([`append_reading`]) is conditional: at most one per
//! dispatch, only on the path that went **through the child**, only when the stream carried a
//! representable reading (`Run::last_reading` is `Some`), and always **before** that dispatch's
//! `dispatch_completed` row — never on a pre-spawn refusal (the guard holds only the *previous*
//! reading, and re-journalling it would double-count a sample) and never on the panic path
//! (ADR-0163 clause 2, superseding `0133` clause 2's *"once per reading"* in part).
//!
//! **Two principles from the runner's journal, as prose — no bytes lifted.** `runner/src/journal.rs`
//! in `redkiln-corpus` at `61f590e51cbb00301c310e46aab615563a7787b8` (cited by the commit SHA
//! only; it has no blob check because nothing here is its text, and there is no `journal.rs` in
//! this adapter — §7c.2's `journal.rs` row) held two rules this module keeps and one shape it
//! does not: **the forbidden-root refusal** — *never write inside the tree under test* — and
//! **append-only discipline** — a record is appended, never rewritten. Here both are satisfied by
//! construction: the adapter writes no file at all (ADR-0099 decision 1's `&mut dyn EventStore` is
//! the only sink; design §9.1's `rk-store` row is the single write path), and the store's port has
//! `append` and no update or delete (`rk-ports`, AC-364). What is NOT taken is the runner's
//! file-writing journal itself (a second write path), its `completed_keys` / `last_warm_mark`
//! resume bookkeeping and its `read_pinned_sha` (`REDKILN_SHA=`), and `anyhow` (ADR-0102 d6).
//! Design §9.1's own adapter row names `journal.rs` in the lift list while its `rk-store` row makes
//! the store the single write path — an intra-design tension; the principles-only reading is
//! taken and a dated design note is OWED (§7c.22).
//!
//! **The row** ([`append_completed`]): the completed `rk_types::DispatchRecord`, serialised, as
//! the payload of `EventKind::DispatchCompleted` (ADR-0126 decision 1's table; `Event.payload` is
//! a `serde_json::Value`). Argv and env travel as digests only (the record has no member for
//! either — ADR-0105 clause 7), stdout/stderr verbatim to the cap then digest + head/tail
//! (`Retained`), the child environment never. **No `result` member** — the row is the completed
//! record and nothing else (§7c.11); the step's outcome is `DispatchOutcome.result`. The kernel
//! drops `outcome.record` (`runtime/activity.rs`), so this append is the only persistence of
//! everything AC-360 names, and `lib.rs` calls it on every path: `Ok`, each `StepError` the
//! adapter produces, the pre-spawn refusals with `cli_version: Pinned` (AC-221, AC-376), a spawn
//! failure, a kill before init.
//!
//! **AC-368 — a tier-2 append failure is a warning with a counter, never a `StepError`.** The
//! `dispatch_completed` append is ATTEMPTED on every path, and the `rate_limit_reading` append on
//! every path that owes one; when the store refuses either, the failure is counted and
//! typed on the adapter's [`AppendWarnings`] (the counter AC-368 names) and `dispatch` returns the
//! outcome it had. That is exactly what both records permit today and no more: ADR-0126's forbids
//! reads against a path that returns *"without appending"*, decision 6 says AC-368 and AC-360
//! *"cannot both hold as worded"* and leaves the reconciliation to a dated register amendment, so
//! `ac368_tier2_append_failure_is_warning` asserts the attempt, the typed count and no panic, and
//! labels the outcome-after-failed-append as R-16's reading under test, not a claim (§7c.13;
//! PROVISIONAL).
//!
//! **AC-385's warning is derived on read** ([`mismatch_warning`]): the row carries `cli_version`
//! and `instrument_version` and **no warning member** (ADR-0126 decision 1's table names none),
//! so the warning is `Some(VersionMismatch { observed, pinned })` exactly when `cli_version` is
//! `Observed(v)` and `v != instrument_version`, and `None` on `Pinned` (no measurement to warn
//! about — a pre-spawn row written `Observed` would be Halt 11's shape and is forbidden by the
//! record). *"Attached to every cache metric taken in that run"* is the reader's join over this
//! accessor (7d's `redkiln cost`).

use std::sync::Mutex;

use rk_ports::event_store::EventStore;
use rk_types::activity::DispatchRecord;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::usage::CliVersion;

use crate::headroom::{self, RateLimitReading};

/// AC-385's typed warning: the child's own version disagreed with the pin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionMismatch {
    /// What the child's `system/init` reported (`CliVersion::Observed`).
    pub observed: String,
    /// The `Config` pin (`instrument_version`).
    pub pinned: String,
}

/// The mismatch warning derived from a row: `Some` exactly when the version was OBSERVED and
/// differs from the pin; `None` on `Pinned` (nothing measured) and on an observed match.
#[must_use]
pub fn mismatch_warning(record: &DispatchRecord) -> Option<VersionMismatch> {
    match &record.cli_version {
        CliVersion::Observed(v) if *v != record.instrument_version => Some(VersionMismatch {
            observed: v.clone(),
            pinned: record.instrument_version.clone(),
        }),
        CliVersion::Observed(_) | CliVersion::Pinned(_) => None,
    }
}

/// One failed tier-2 append — a `dispatch_completed` row or a `rate_limit_reading` row — typed
/// (AC-368's *"surfaced as a warning with a counter"*).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendWarning {
    /// The error the store answered.
    pub error: StepError,
}

/// The adapter's warning counter: every tier-2 append the store refused, in order.
///
/// Both kinds sit on one counter, so a dispatch through the child that carried a reading can
/// count two (the reading's, then the completion's; ADR-0163 clause 2). Held by the
/// harness behind a `Mutex` (the harness is `Sync`; a poisoned lock is a halt, never recovered —
/// ADR-0118).
#[derive(Debug, Default)]
pub struct AppendWarnings(Mutex<Vec<AppendWarning>>);

impl AppendWarnings {
    /// Record one failure.
    ///
    /// # Panics
    /// If the lock is poisoned (a thread panicked while holding it): ADR-0118 — no recovery.
    pub fn record(&self, error: StepError) {
        self.0
            .lock()
            .expect("the append-warning counter's lock is poisoned: a halt (ADR-0118)")
            .push(AppendWarning { error });
    }

    /// How many appends failed so far.
    ///
    /// # Panics
    /// As [`Self::record`].
    #[must_use]
    pub fn count(&self) -> usize {
        self.0
            .lock()
            .expect("the append-warning counter's lock is poisoned: a halt (ADR-0118)")
            .len()
    }

    /// Every failure so far, typed, in order.
    ///
    /// # Panics
    /// As [`Self::record`].
    #[must_use]
    pub fn warnings(&self) -> Vec<AppendWarning> {
        self.0
            .lock()
            .expect("the append-warning counter's lock is poisoned: a halt (ADR-0118)")
            .clone()
    }
}

/// The `dispatch_completed` event for a record: the record serialised as the payload.
///
/// # Panics
/// Never in practice: every member derives `Serialize` and none can fail to serialise; the
/// `expect` names the invariant.
#[must_use]
pub fn completed_event(record: &DispatchRecord) -> Event {
    Event::new(
        EventKind::DispatchCompleted,
        serde_json::to_value(record)
            .expect("a DispatchRecord serialises (every member derives Serialize)"),
    )
}

/// Append the `dispatch_completed` row for `record` through `store`.
///
/// On failure, count and type it on `warnings` and return the error for the caller's own
/// accounting — never a `StepError` that fails the activity (AC-368).
pub fn append_completed(
    store: &mut dyn EventStore,
    record: &DispatchRecord,
    warnings: &AppendWarnings,
) -> Result<(), StepError> {
    append_counted(store, completed_event(record), warnings)
}

/// The `rate_limit_reading` event for a reading and the dispatch's item: the payload is
/// [`headroom::reading_payload`]'s fixed key set (ADR-0163 decision clause 3).
///
/// It carries `cli_version` — the dispatch record's own, so the row satisfies AC-227 as its
/// `dispatch_completed` sibling does.
#[must_use]
pub fn reading_event(
    reading: &RateLimitReading,
    item: Option<&str>,
    cli_version: &CliVersion,
) -> Event {
    Event::new(
        EventKind::RateLimitReading,
        headroom::reading_payload(reading, item, cli_version),
    )
}

/// Append the `rate_limit_reading` row through `store`.
///
/// ONCE per dispatch that went through the child, from the stream's last representable reading,
/// before that dispatch's `dispatch_completed` row (ADR-0163 decision clause 2). The caller
/// decides WHETHER; this only appends.
///
/// A refused append is counted and typed on the SAME `warnings` counter a refused
/// `dispatch_completed` append is (AC-368; ADR-0163 clause 2's *"exactly as"*) and returned for
/// the caller's accounting — never a `StepError` that fails the activity.
pub fn append_reading(
    store: &mut dyn EventStore,
    reading: &RateLimitReading,
    item: Option<&str>,
    cli_version: &CliVersion,
    warnings: &AppendWarnings,
) -> Result<(), StepError> {
    append_counted(store, reading_event(reading, item, cli_version), warnings)
}

/// One tier-2 append with AC-368's accounting: a refusal is recorded on `warnings` and returned.
fn append_counted(
    store: &mut dyn EventStore,
    event: Event,
    warnings: &AppendWarnings,
) -> Result<(), StepError> {
    match store.append(event) {
        Ok(()) => Ok(()),
        Err(e) => {
            warnings.record(e.clone());
            Err(e)
        }
    }
}
