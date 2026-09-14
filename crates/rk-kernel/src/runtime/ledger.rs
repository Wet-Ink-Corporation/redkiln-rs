//! **The executed-once ledger** — ADR-0114 decision 2, and the criterion that does not carry it.
//!
//! > **ADR-0114 `## Decision` clause 2**, accepted 2026-09-11: *"The catalog record gains
//! > `executed: { first: Option<{ run_id, item, at }>, last: Option<{ run_id, item, at }> }`,
//! > written by the kernel on the first and every subsequent `activity_committed` for that
//! > activity."*
//!
//! # 🛑 AC-110 DOES NOT CARRY THIS, AND THE COVERAGE IS REPORTED **POWERLESS**
//!
//! `s7b-kernel.md` §7b.10 says *"The register routes this through **AC-110**"*. It does not.
//! AC-110's GIVEN/WHEN/THEN is *"GIVEN an activity that lives only in the unshipped extended catalog
//! file / WHEN any enabled pack references it … / THEN the reference fails as a configuration error
//! at load time"*, and its *Verified by* names a loader unit test, a `doctor` rule-table row and an
//! integration test over the resolved catalog. **None of the three is a ledger.** Under ADR-0092
//! clause 2.2 the register owns each criterion's GIVEN/WHEN/THEN and its instrument, so a runbook
//! sentence that adds a THEN is the runbook being wrong.
//!
//! Residual 6 is FORCED and its Read line is followed exactly: **build it, do not stop**; an
//! accepted ADR is authority enough to *execute* and `00-preamble.md`:64 forbids a stage from
//! *deciding*. The test is named `ac110_executed_ledger_first_and_last` because Checkpoint 7b
//! :1498-1507 asserts that name and the block halts without it — and it is **not** named after
//! AC-114, which the register marks **HUMAN GATE**. What is reported is: the ledger is **built to
//! ADR-0114 decision 2**, and **AC-110's ledger coverage is POWERLESS** (golden constraint 10,
//! `00-preamble.md`:154). A register amendment — widen AC-110's THEN, or allocate a new id — is
//! **OWED**. AC-110's own `doctor` half is 7d's per §7b.15's ARMED table and is not claimed here.
//!
//! # 🛑 THE LEDGER IS A PROJECTION OVER THE STORE, NEVER A FIELD IN THE SHIPPED TOML
//!
//! Residual 6b is FREE and its Read line is followed. ADR-0114 says *"the catalog record gains"*;
//! writing `executed` into the committed catalog TOML would trip **AC-111** — a shipped-activity
//! content-digest change with no `version` bump — on **every run**, and a per-run `version` bump
//! would invalidate **every** memo entry keyed on `activity_id@version` (ADR-0097 component 1). So
//! *"the catalog record"* is read as the **resolved, in-memory** record, which is consistent with
//! §7b.10's own *"Step 7e authors TOML only"*. **That is a reading and a decision is OWED.**
//!
//! # 🛑 `at` ARRIVES ON THE PAYLOAD, AND A ROW WITHOUT ONE IS REFUSED
//!
//! ADR-0114 requires `at`; AC-001 bans the clock; `crate::replay`'s header bans a timestamp in the
//! golden-log layout (*"a recorded `at` would make two runs of identical inputs produce different
//! bytes"*). The only reading consistent with all three is that `at` arrives **on the
//! `activity_committed` payload the activity returned**, and `super::activity` writes it from the
//! `JournaledNow` its caller supplied. A payload without `at`, `run_id` or `item` is
//! [`LedgerError::MalformedPayload`] — **never a zero, never an epoch**. Stated because no record
//! states it (residual 6b); `src/select.rs`:494-499 is the house model for a reading written into the
//! code that takes it.
//!
//! # 🛑 A SECOND READING: *"first"* AND *"last"* ARE JOURNAL ORDER, NOT `at` ORDER
//!
//! ADR-0114 says *"on the first and every subsequent `activity_committed`"* — which is a statement
//! about the **sequence of events**, not about their timestamps. `at` is a value the activity
//! returned and this crate does not mint, two commits can carry the same `at`, and an adapter whose
//! clock stepped backwards would otherwise silently re-write `first`. So [`fold_executed`] takes the
//! **first `activity_committed` in append order** as `first` and the **most recent** as `last`, and
//! `ac110_executed_ledger_first_and_last` asserts both readings against a case in which journal
//! order and `at` order **disagree**. On a well-behaved journal the two rules agree, which is why
//! the disagreeing case is in the test rather than left to be discovered. **No record settles it; a
//! record is OWED.**
//!
//! # The granularity is the one AC-012 uses, and the two must agree
//!
//! `activity_committed` is emitted **per step activation**, not per activity (ADR-0097 decision 2;
//! PIA-008; AC-012's own *Verified by*). The ledger is keyed on `activity_id@version` and therefore
//! **aggregates** activations of one activity — which is exactly what ADR-0114's *"for that
//! activity"* asks for, and it is worth stating because the two grains differ by one level and the
//! key names only one of them.

use std::collections::BTreeMap;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

/// One execution, stamped. **ADR-0114 decision 2's three fields and no fourth.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionStamp {
    pub run_id: String,
    pub item: String,
    /// **Journaled, never minted.** See the module header.
    pub at: Timestamp,
}

/// The ledger entry for one activity.
///
/// `Default` is derived and it is the *empty* entry — *"at v0 every activity but `root-cause`'s P7
/// members reads `executed: never`"* (§7b.10). An empty entry is a fact about the store, not a
/// defaulted policy value, which is the same distinction `crate::workflow::budget`'s counters draw
/// against `BudgetEnvelope`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Executed {
    pub first: Option<ExecutionStamp>,
    pub last: Option<ExecutionStamp>,
    /// **House Shape C**, private so only [`fold_executed`] can move it.
    executions: usize,
}

impl Executed {
    /// How many `activity_committed` rows this entry folded. **House Shape C**: without it,
    /// *"`first` and `last` are the same stamp"* cannot be told from *"the fold saw one row"*.
    #[must_use]
    pub const fn executions(&self) -> usize {
        self.executions
    }

    /// Whether this activity has never executed — §7b.10's `executed: never`.
    #[must_use]
    pub const fn never(&self) -> bool {
        self.first.is_none()
    }
}

/// Why the ledger refused a row. **Refused, never defaulted.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    MalformedPayload {
        /// Which member was missing or the wrong type.
        member: &'static str,
        detail: String,
    },
}

impl LedgerError {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::MalformedPayload { .. } => "malformed_payload",
        }
    }
}

impl core::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MalformedPayload { member, detail } => write!(
                f,
                "an `activity_committed` row carries no usable `{member}`: {detail}. ADR-0114 \
                 decision 2's stamp is `{{ run_id, item, at }}` and none of the three has a lawful \
                 default here"
            ),
        }
    }
}

impl std::error::Error for LedgerError {}

/// **ADR-0114 decision 2.** Fold every `activity_committed` into the per-activity ledger.
///
/// Keyed on `activity_id@version`, in a `BTreeMap` so the iteration order is the key order and not
/// the insertion order — `HashMap` is in `clippy.toml`'s `disallowed-types` and a ledger printed in
/// hash order would make two runs of one store print differently.
///
/// # Errors
///
/// [`LedgerError::MalformedPayload`] for a row missing `activity_id_at_version`, `item`, `run_id`
/// or `at`.
pub fn fold_executed(events: &[Event]) -> Result<BTreeMap<String, Executed>, LedgerError> {
    let mut out: BTreeMap<String, Executed> = BTreeMap::new();
    for event in events {
        // A row of another kind is skipped, never refused: the ledger's subject is the activation
        // record and a journal holds twenty-eight other kinds beside it.
        if event.kind != EventKind::ActivityCommitted {
            continue;
        }
        let activity = required_str(event, "activity_id_at_version")?;
        let item = required_str(event, "item")?;
        let run_id = required_str(event, "run_id")?;
        let at = event
            .payload
            .get("at")
            .and_then(serde_json::Value::as_i64)
            .ok_or_else(|| LedgerError::MalformedPayload {
                member: "at",
                detail: "absent, or not an integer count of seconds since the epoch".to_owned(),
            })?;
        let stamp = ExecutionStamp {
            run_id,
            item,
            at: Timestamp::from_unix_seconds(at),
        };
        let entry = out.entry(activity).or_default();
        // **"the first and every subsequent"** — `first` is written once and `last` on every row.
        // The order is APPEND order; see the module header's second reading.
        if entry.first.is_none() {
            entry.first = Some(stamp.clone());
        }
        entry.last = Some(stamp);
        entry.executions += 1;
    }
    Ok(out)
}

/// One required string member, refused rather than defaulted.
fn required_str(event: &Event, member: &'static str) -> Result<String, LedgerError> {
    match event
        .payload
        .get(member)
        .and_then(serde_json::Value::as_str)
    {
        Some(value) if !value.is_empty() => Ok(value.to_owned()),
        Some(_) => Err(LedgerError::MalformedPayload {
            member,
            detail: "present and empty, which names nothing".to_owned(),
        }),
        None => Err(LedgerError::MalformedPayload {
            member,
            detail: "absent, or not a string".to_owned(),
        }),
    }
}

/// Every `activity_committed` row in the journal, unfiltered — the input the fold reads.
///
/// Exposed so a caller can assert *"the fold looked at N rows"* against the journal rather than
/// against the fold's own report.
#[must_use]
pub fn committed_rows(events: &[Event]) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .collect()
}
