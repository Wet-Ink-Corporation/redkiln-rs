//! **The circuit breaker** — the counting rule, the per-key isolation and the store-derived state
//! of AC-033–AC-035, keyed on a family token this crate does not construct.
//!
//! > **AC-033** — `rk-kernel::retry` · decisions: ADR-0022
//! > GIVEN K consecutive `Platform` failures for one model family
//! > WHEN the next dispatch for that family is requested
//! > THEN it is refused and a durable breaker mark for the family is written.
//! > *Verified by:* a unit test driving K consecutive failures and asserting refusal plus the mark;
//! > `K` is a pack value, asserted present by pack validation.
//! > *Note:* no corpus figure sizes `K`, and no spike measured a `Platform` failure rate at all
//! > (gap 3).
//! >
//! > **AC-034** — `rk-kernel::retry` · decisions: ADR-0022
//! > GIVEN an open breaker on one model family
//! > WHEN a dispatch is requested for a different family
//! > THEN it proceeds; the breaker never trips globally.
//! > *Verified by:* a two-family unit test asserting one refusal and one success; a lint banning
//! > module-level `static`/`lazy_static`/`OnceLock` breaker state, so process-wide breaker state
//! > cannot be introduced.
//! >
//! > **AC-035** — `rk-kernel::retry` · evidence: **F22** · decisions: ADR-0022, ADR-0017
//! > GIVEN an open breaker
//! > WHEN the process dies and `redkiln run` is invoked again
//! > THEN dispatches to that family are still refused until the mark is read and found expired;
//! > breaker state is never held only in memory.
//! > *Verified by:* a kill-and-re-enter test asserting the refusal survives the restart; a lint that
//! > breaker state is constructed only from the store.
//! > *Note:* the expiry comparison reads the current time as a **journaled activity result**
//! > (AC-001), never from the workflow's own clock.
//!
//! # 🛑 THE KEY HAS NO CARRIER, AND THE CATALOG'S `family` IS A DIFFERENT FAMILY
//!
//! Eight occurrences of *"family"* across those three criteria, five of them the key the breaker is
//! partitioned by — and **nothing in this tree represents a model family**.
//! [measured 2026-09-14: `grep -rn "family\|ModelFamily\|model_tier" crates/rk-types/src/
//! crates/rk-ports/src/` → **0 hits, exit 1**.] `ActivitySpec` is *"ADR-0099:61's five fields and no
//! sixth"* and carries the card as a **digest**, so even a bound card's tier is behind a hash at the
//! moment AC-033's WHEN fires.
//!
//! **`crates/rk-kernel/src/catalog.rs`'s `family` is ADR-0058's ACTIVITY family and may never be
//! read here.** ADR-0058: *"An activity declares its family membership with a `family = "<name>"`
//! field. Lint checks three things for every declared family: that its members share `provides`,
//! that they hold distinct `method_rank` values, and that the family satisfies rule 26's
//! totality-over-rigor property."* That is a rank-escalation ladder — the shipped fixture family is
//! `root-cause(10) < differential-debugging(20) < bisect(30) < instrumented-trace-analysis(40)` —
//! and every member of it would typically run on the **same** model. A breaker keyed on it would
//! trip globally with respect to the model and locally with respect to the method, which is the
//! exact inverse of AC-034 on both axes; and a two-"family" unit test written from the catalog would
//! pass. **The distance between the right answer and the worst one is a single `.family`**, and
//! nothing in Checkpoint 7b's instrument set would catch it — which is why [`FamilyKey`] has exactly
//! one constructor, [`FamilyKey::supplied`], and why no expression in this file reads
//! [`crate::catalog`].
//!
//! So the **binding** — what produces a family token, and whether `claude-opus-4` and
//! `claude-opus-4-5` are one family or two — is **ARMED, NOT DISCHARGED** and is reported against
//! Step 7c. A record is OWED; the operator's instruction for a half-buildable criterion is to build
//! the buildable half and record the other. What is built here is the half that does not depend on
//! what the key *means*: the counting rule, the reset rule, the per-key isolation, the store
//! derivation and the refusal.
//!
//! # 🛑 `K` HAS NO MEASURED VALUE AND IS PACK-DECLARED
//!
//! §7b.10: *"`K`, `N`, the backoff base and cap, the two-tick headroom margin and the per-item token
//! appetite **HAVE NO MEASURED VALUES** … `K`, `N` and the appetite are declared in the pack and
//! refused as code defaults … **do not default a constant here**."* [`BreakerThreshold`] is
//! [`super::envelope`]'s and has no `Default`; the pack accessor is
//! [`crate::select::Pack::breaker_threshold`] and the refusal is
//! [`crate::select::Pack::require_breaker_threshold`]. Nothing in this file names a number.
//!
//! # 🛑 THE EXPIRY HAS NO VALUE AND NO HOME, AND THIS MODULE DOES NOT GIVE IT ONE
//!
//! AC-035's *"until the mark is read and found expired"* names a duration. §7b.10:852-856 sorts
//! every Slice-4 number with no measured value into exactly two homes — pack-declared (`K`, `N`, the
//! appetite) and kernel policy constant (backoff base 2 s, cap 5 min) — **and the breaker's expiry
//! is in neither** [measured 2026-09-14: `grep -n "expir" runbook/phase-1/s7b-kernel.md` returns
//! nothing in §7b.10]. Four homes are structurally available (pack-declared beside `K`; a kernel
//! policy constant beside the backoff pair, which contradicts PIA-055's own exhaustive split;
//! derived from the backoff cap, which is a default with arithmetic in front of it; or no expiry at
//! all, which re-words a THEN a stage may not touch). **None is chosen here.** [`BreakerExpiry`]
//! has one constructor, [`BreakerExpiry::declared`], no `Default` and no `const` fallback, and
//! [`BreakerState::decide`] takes it **as an argument** — so the value enters from outside this
//! crate and the choice of home remains the operator's. A record is OWED.
//!
//! # 🛑 THE DURABLE MARK HAS NO CARRIER — AC-033's SECOND CONJUNCT IS ARMED
//!
//! `EventKind` is closed at 29 variants (`event_kind.rs`:87, *"Built from an exhaustive match so a
//! new variant cannot be added without appearing here"*) and none is a breaker mark; `RefusalClass`
//! is closed at four (`NotAdopted`, `NotIsolated`, `Unsupported`, `Overage`) and none is a breaker;
//! `StepError` is closed at eleven (ADR-0112, *"Forbids a twelfth variant without a successor to
//! this record"*). ADR-0116's driver fixes the route — *"the kind is minted by record or not at
//! all"* — and ADR-0120 is the precedent for the lawful one. **No kind is minted here.**
//!
//! Residual 10c's reading is followed instead: the breaker's durable state is a **projection over
//! `step_failed { class: platform }` events the store already holds**, which is exactly what
//! AC-035's *"breaker state is constructed only from the store"* and AC-034's `static` ban describe.
//! That is an **inference and is stated as one**, not a record's word. **AC-033's
//! *"a durable breaker mark for the family is written"* is therefore ARMED, NOT DISCHARGED** — this
//! module derives the mark and never writes one, and the `write` has no lawful carrier.
//!
//! # The payload shape is a READING, against an already-OWED record
//!
//! The fold reads three members and no fourth:
//!
//! | kind | members read | effect |
//! | --- | --- | --- |
//! | `step_failed` | `family` (string), `class` (string), `at` (integer seconds) | `class == "platform"` increments the family's consecutive count; any other class resets it to zero |
//! | `activity_committed` | `family` (string) | resets the family's consecutive count to zero |
//! | every other kind | none | no state change, in one grouped arm |
//!
//! `Event.payload` is `serde_json::Value` *"provisionally and visibly"* and `crates/rk-types/src/
//! event.rs`:46-58 records that **the typed-payload record, per kind, is OWED**. This module does
//! not invent it: an event that carries no `family` is not family-scoped and is skipped, and an
//! event that carries a `family` but not the members the table names is a defect
//! ([`BreakerError`]) rather than a silent zero.
//!
//! # Two readings this file takes because no record states them
//!
//! 1. **A success resets the count and does NOT clear an open mark.** *"K consecutive"* needs a
//!    reset rule and no record gives one; `activity_committed` is ADR-0022's activation record and
//!    is the only success this crate can see. Clearing an open mark on it would be a route around
//!    the expiry AC-035 names, and a half-open probe policy is in no record either — so the mark
//!    stands until [`BreakerState::decide`] finds it expired.
//! 2. **The expiry boundary is `elapsed >= expiry`.** Neither AC-035 nor ADR-0017 says which side
//!    *"found expired"* falls on. The sibling reading is already on disk — `budget.rs`'s
//!    `exhausted_counter` takes `spent >= declared`, i.e. a declared quantity is **reached** when
//!    the spend equals it — and taking the opposite side here would mean two declared quantities in
//!    one crate with two boundary rules. Both sides are asserted by `breaker_ac033_ac035`.
//!
//! # What IS discharged here
//!
//! AC-034's two-family property over two arbitrary distinct keys, and its lint
//! ([`breaker_no_module_state_rule`]); AC-035's own named instrument, *"a lint that breaker state is
//! constructed only from the store"* ([`breaker_state_from_store_rule`]), plus the survival of a
//! dropped-and-rebuilt state over the same events; and AC-033's counting rule, its refusal, and
//! `K` read from a pack and refused as a code default. Both lints are declared as `fn`s in
//! `crates/rk-kernel/src/` on the facet-lint pattern, so the declaration is grep-visible the way
//! §7b.14:1592-1594 makes the three facet rules visible.
//!
//! # What is deliberately NOT here
//!
//! No clock: the expiry compares against a [`JournaledNow`] **argument**, and the parent module's
//! `#![deny(clippy::disallowed_methods)]` reaches this child, so a `SystemTime::now()` here is a
//! non-zero clippy exit and not a review comment. No `static`, no `lazy_static`, no `OnceLock`, no
//! `thread_local`, no `Mutex`, no `RefCell` — AC-034's lint runs over this file's own source. No
//! dispatch and no append: the refusal is a **decision value**, and the runtime boundary that would
//! act on it is Slice 4h's. No widening of the trip condition: the breaker keys on `Platform`
//! **only** (AC-033's GIVEN), and no record licenses `Transport` or `Timeout` tripping it.
//!
//! **On concurrency (residual 10m).** [`BreakerState`] is a fold over values with no cell of any
//! kind, so it holds nothing across a call and nothing to interleave. ADR-0076 — *"the kernel never
//! observes interleaving"* — and ADR-0115 clause 1's `Arc<Mutex<dyn EventStore>>` put AC-012's
//! concurrency instrument at the composition root, and a second process lifetime is simply a second
//! fold over the same store. That is what makes AC-035's survival assertion a property of this
//! function rather than of a process.

use std::collections::BTreeMap;
use std::fmt;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

use super::envelope::BreakerThreshold;

/// The one `step_failed` class that counts toward the threshold.
///
/// AC-033's GIVEN is *"K consecutive **`Platform`** failures"*, and no record licenses widening it
/// to `Transport` or `Timeout` — both of which `retry_class()` already sends to `Backoff`, i.e. the
/// outer retry handles them and the breaker does not. The token is `FailureClass`'s own
/// `snake_case` serialisation (`event.rs`:9-19).
pub const PLATFORM_CLASS: &str = "platform";

/// The breaker's partition key — **an opaque token this crate never constructs from its own data.**
///
/// There is exactly one constructor and it takes the token from the caller. That is the whole of
/// the header's first 🛑 written into the type: no model family exists in `rk-types`, `rk-ports` or
/// this crate, so a `FamilyKey::from_activity(&Activity)` would have to read
/// `catalog.rs`'s `family` — ADR-0058's **activity** family — and the named instrument would not
/// catch the substitution.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FamilyKey(String);

impl FamilyKey {
    /// From a token the caller supplied. **The only constructor**, and named `supplied` rather than
    /// `new` because every call site is transcribing something this crate did not compute.
    #[must_use]
    pub fn supplied(token: &str) -> Self {
        Self(token.to_owned())
    }

    /// The token, as supplied.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FamilyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Time, as a value the workflow was handed. **There is no constructor that reads a clock.**
///
/// AC-035's *Note*: *"the expiry comparison reads the current time as a **journaled activity
/// result** (AC-001), never from the workflow's own clock."* `rk_types::time::Timestamp` has no
/// `now()` *"deliberately"*, and this wrapper is what makes the provenance a type fact at the one
/// call site that compares times: a `Timestamp` could have come from anywhere a `Timestamp` can be
/// built, and a [`JournaledNow`] could only have come from a journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JournaledNow(Timestamp);

impl JournaledNow {
    /// The only constructor: from a timestamp that arrived **on a journaled activity result**.
    #[must_use]
    pub const fn from_journal(ts: Timestamp) -> Self {
        Self(ts)
    }

    /// The journaled instant.
    #[must_use]
    pub const fn timestamp(self) -> Timestamp {
        self.0
    }
}

/// How long an open breaker mark stands. **Declared, never defaulted, and homed by no record.**
///
/// See the header's third 🛑: the expiry is in neither of §7b.10's two lists, four homes are
/// available and none is chosen here. No `Default`, no `const` fallback, no `unwrap_or` — the value
/// reaches [`BreakerState::decide`] as an argument from outside this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BreakerExpiry(u64);

impl BreakerExpiry {
    /// From a value something outside this crate declared. **The only constructor.**
    #[must_use]
    pub const fn declared(secs: u64) -> Self {
        Self(secs)
    }

    /// The declared expiry, in seconds.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for BreakerExpiry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} s", self.0)
    }
}

/// An open breaker, as the fold derived it. **Derived, never written** — see the header's fourth 🛑.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OpenMark {
    family: FamilyKey,
    consecutive: u32,
    opened_at: Timestamp,
}

impl OpenMark {
    /// The family the mark is for.
    #[must_use]
    pub const fn family(&self) -> &FamilyKey {
        &self.family
    }

    /// How many consecutive `Platform` failures stood behind it when it opened. **`>= K`**, never a
    /// number this crate chose.
    #[must_use]
    pub const fn consecutive(&self) -> u32 {
        self.consecutive
    }

    /// The journaled instant of the failure that opened it — read off the event payload, never
    /// minted here.
    #[must_use]
    pub const fn opened_at(&self) -> Timestamp {
        self.opened_at
    }
}

/// Why the fold could not run — **Shape A** (`kernel-style.md` §2.2).
///
/// Distinct from a refusal, and never collapsed into one: a journal this rule cannot read is not a
/// journal with no failures in it, and returning an empty state for it would be the silent pass the
/// module exists to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakerError {
    /// A family-scoped event is missing a member the fold's reading names.
    MalformedPayload {
        /// The event kind's wire name.
        kind: &'static str,
        /// Which member, and what was wrong with it.
        detail: String,
    },
}

impl BreakerError {
    /// A stable tag, so a test names the failure it expects without matching message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::MalformedPayload { .. } => "malformed_payload",
        }
    }
}

impl fmt::Display for BreakerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedPayload { kind, detail } => write!(
                f,
                "the breaker fold cannot read a family-scoped `{kind}` event: {detail}. The \
                 per-kind typed-payload record is OWED (`rk-types/src/event.rs`:46-58) and this \
                 fold reads a stated shape rather than guessing one (AC-035)"
            ),
        }
    }
}

impl std::error::Error for BreakerError {}

/// What the fold measured, whether or not it opened anything — **House Shape C**.
///
/// `kernel-style.md` §2.2: a state with no open mark reads exactly like a journal the fold never
/// looked at, so the counts travel beside the verdict and the tests assert them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BreakerSurvey {
    /// Every event handed to the fold, including the ones it ignored.
    pub events_folded: usize,
    /// Family-scoped events the fold actually read.
    pub family_scoped: usize,
    /// `step_failed { class: platform }` rows counted toward a threshold.
    pub platform_failures: usize,
    /// Rows that reset a family's consecutive count to zero.
    pub resets: usize,
}

/// The breaker's state. **Constructed only from the store**, which is AC-035's own second
/// instrument verbatim.
///
/// [`BreakerState::from_events`] is the only constructor besides `Default`, and `Default` is the
/// empty state a fresh fold starts from — not a remembered one. There is no cell of any kind in
/// this struct and no `static` in this file, so a second process lifetime is a second fold over the
/// same events and decides the same way. That is the whole of AC-035's *"breaker state is never
/// held only in memory"*, and [`breaker_state_from_store_rule`] is the lint that keeps it true.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BreakerState {
    open: BTreeMap<FamilyKey, OpenMark>,
    consecutive: BTreeMap<FamilyKey, u32>,
    survey: BreakerSurvey,
}

impl BreakerState {
    /// Fold a recorded journal into the breaker's state, against a pack-declared `K`.
    ///
    /// Total over the closed 29 kinds with **no catch-all arm**, so a thirtieth kind is an `E0004`
    /// here exactly as it is in `EventKind::tier()`: a kind cannot enter the system without someone
    /// deciding whether it moves the breaker. Grouped or-patterns, because
    /// `clippy::match_same_arms` is denied workspace-wide.
    ///
    /// # Errors
    ///
    /// [`BreakerError::MalformedPayload`] when an event carries a `family` but not the other
    /// members the header's reading table names. An unreadable row is a defect, never a zero.
    pub fn from_events(events: &[Event], k: BreakerThreshold) -> Result<Self, BreakerError> {
        let mut state = Self::default();
        state.survey.events_folded = events.len();
        for event in events {
            // Not family-scoped: the fold ignores it. Checked BEFORE the kind, so an unrelated
            // `step_failed` from some other subsystem is not a defect here.
            let Some(family) = event.payload.get("family") else {
                continue;
            };
            let family = family
                .as_str()
                .ok_or_else(|| BreakerError::MalformedPayload {
                    kind: "any",
                    detail: "`family` is present but is not a string".to_owned(),
                })?;
            let family = FamilyKey::supplied(family);
            state.survey.family_scoped += 1;

            // TOTAL OVER THE CLOSED 29, WITH NO CATCH-ALL ARM. A thirtieth kind is an `E0004`
            // here, exactly as it is in `EventKind::tier()`: a kind cannot enter the system
            // without someone deciding whether it moves the breaker. Grouped or-patterns, because
            // `clippy::match_same_arms` is denied workspace-wide.
            match event.kind {
                EventKind::StepFailed => {
                    let class = event
                        .payload
                        .get("class")
                        .and_then(serde_json::Value::as_str)
                        .ok_or_else(|| BreakerError::MalformedPayload {
                            kind: "step_failed",
                            detail: "`family` is present but `class` is absent or not a string, \
                                     so the fold cannot tell a platform failure from any other"
                                .to_owned(),
                        })?;
                    let at = event
                        .payload
                        .get("at")
                        .and_then(serde_json::Value::as_i64)
                        .ok_or_else(|| BreakerError::MalformedPayload {
                            kind: "step_failed",
                            detail: "`family` is present but `at` is absent or not an integer, so \
                                     an opened mark would have no journaled instant to expire \
                                     against (AC-035)"
                                .to_owned(),
                        })?;
                    if class == PLATFORM_CLASS {
                        state.survey.platform_failures += 1;
                        let run = state.consecutive.entry(family.clone()).or_insert(0);
                        *run = run.saturating_add(1);
                        let run = *run;
                        if run >= k.get() {
                            // INSERT OR REFRESH, and the direction is deliberate. The mark
                            // records the MOST RECENT failure that completed a run of at least
                            // `K`, so a family that trips, recovers and trips again starts its
                            // expiry from the second trip. Keeping the first mark would let an
                            // old instant expire a breaker that has just re-opened, which is the
                            // fail-OPEN side of a boundary no record settles. Neither side is
                            // stated anywhere; this one is the one that refuses for longer.
                            state.open.insert(
                                family.clone(),
                                OpenMark {
                                    family,
                                    consecutive: run,
                                    opened_at: Timestamp::from_unix_seconds(at),
                                },
                            );
                        }
                    } else {
                        // A failure of another class BREAKS the run of `Platform` failures. The
                        // breaker keys on `Platform` ONLY (AC-033's GIVEN) and no record licenses
                        // widening it, so this row neither counts toward the threshold nor leaves
                        // the previous run intact.
                        state.survey.resets += 1;
                        state.consecutive.insert(family, 0);
                    }
                }
                // ADR-0022's activation record — the only success this crate can see. It resets
                // the count and does NOT clear an open mark: clearing one here would be a route
                // around the expiry AC-035 names, and a half-open probe policy is in no record.
                EventKind::ActivityCommitted => {
                    state.survey.resets += 1;
                    state.consecutive.insert(family, 0);
                }
                // Everything else: NO STATE CHANGE, in one grouped arm. None of these is a
                // `Platform` failure and none is the activation record, so none can move a
                // consecutive run — including `step_refused` and `rate_limited`, which are
                // configuration and billing states and never platform failures
                // (`event.rs`:20-29).
                EventKind::StageTransition
                | EventKind::GateOpened
                | EventKind::GateAnswered
                | EventKind::GateExpired
                | EventKind::SliceSealed
                | EventKind::ReconcileStarted
                | EventKind::GateAnswerVoid
                | EventKind::DagAssembled
                | EventKind::DagAmended
                | EventKind::MemoHit
                | EventKind::MemoMiss
                | EventKind::MemoEvicted
                | EventKind::Dispatch
                | EventKind::StepCancelled
                | EventKind::StepRefused
                | EventKind::RateLimited
                | EventKind::RateLimitReading
                | EventKind::Degraded
                | EventKind::StructuredDrift
                | EventKind::DriftAdvisory
                | EventKind::ReviewClassDrift
                | EventKind::Rewrite
                | EventKind::ArtifactRead
                | EventKind::ArtifactWrite
                | EventKind::AdvisoryCreated
                | EventKind::AdvisoryRetired
                | EventKind::AdvisoryDismissed => {}
            }
        }
        Ok(state)
    }

    /// What the fold measured.
    #[must_use]
    pub const fn survey(&self) -> BreakerSurvey {
        self.survey
    }

    /// The open mark for a family, if the fold derived one.
    #[must_use]
    pub fn mark(&self, family: &FamilyKey) -> Option<&OpenMark> {
        self.open.get(family)
    }

    /// Every family with an open mark, in byte order.
    #[must_use]
    pub fn open_families(&self) -> Vec<&FamilyKey> {
        self.open.keys().collect()
    }

    /// A family's current consecutive-`Platform`-failure count.
    #[must_use]
    pub fn consecutive(&self, family: &FamilyKey) -> u32 {
        self.consecutive.get(family).copied().unwrap_or(0)
    }

    /// **AC-033's WHEN, AC-034's isolation and AC-035's expiry comparison, in one function.**
    ///
    /// The decision is a function of (this family's mark, a journaled instant, a declared expiry)
    /// and of nothing else — so an open mark on one family is invisible to every other, which is
    /// AC-034's *"the breaker never trips globally"* written as the absence of any expression that
    /// could read another key.
    #[must_use]
    pub fn decide(
        &self,
        family: &FamilyKey,
        now: JournaledNow,
        expiry: BreakerExpiry,
    ) -> BreakerDecision {
        let Some(mark) = self.open.get(family) else {
            return BreakerDecision::Proceed;
        };
        // The comparison is against a JOURNALED instant and a DECLARED expiry, and neither is this
        // crate's to produce. A `now` BEFORE the mark yields a negative elapsed, which `u64`
        // conversion refuses — so a stale journaled instant cannot reopen a family. The boundary
        // is `>=`, `budget.rs`'s `exhausted_counter` reading for a declared quantity: reaching the
        // declared number is reaching it.
        let elapsed = now
            .timestamp()
            .unix_seconds()
            .saturating_sub(mark.opened_at.unix_seconds());
        if u64::try_from(elapsed).is_ok_and(|e| e >= expiry.get()) {
            return BreakerDecision::Proceed;
        }
        BreakerDecision::Refused {
            family: mark.family.clone(),
            consecutive: mark.consecutive,
        }
    }
}

/// Whether the next dispatch for a family may be requested.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakerDecision {
    /// No open mark, or an open mark the journaled instant has passed the expiry of.
    Proceed,
    /// **AC-033's THEN, first conjunct.** The dispatch is refused.
    Refused {
        /// The family refused — never *"the breaker"*, which has no global state to name.
        family: FamilyKey,
        /// The consecutive count behind the mark.
        consecutive: u32,
    },
}

impl BreakerDecision {
    /// A stable tag, so a test names the outcome it expects without matching message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Refused { .. } => "refused",
        }
    }

    /// Whether this decision refuses the dispatch.
    #[must_use]
    pub const fn refuses(&self) -> bool {
        matches!(self, Self::Refused { .. })
    }
}

impl fmt::Display for BreakerDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Proceed => f.write_str("proceed: no open breaker mark for this family"),
            Self::Refused {
                family,
                consecutive,
            } => write!(
                f,
                "the dispatch for family `{family}` is refused: {consecutive} consecutive \
                 `Platform` failures reached the pack's declared `K` and the mark has not expired \
                 (AC-033)"
            ),
        }
    }
}

/// A clause of one of the two breaker lints. **Shape B** — every violation collected, never the
/// first only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakerLintViolation {
    /// **AC-034.** A module-level state form appears in the breaker's source.
    ModuleLevelState {
        /// The banned form, as a stable token.
        form: &'static str,
        /// The 1-indexed source line it was found on.
        line: usize,
    },
    /// **AC-035.** A form through which breaker state could come from somewhere other than a fold
    /// over the store.
    StateNotFromStore {
        /// The banned form, as a stable token.
        form: &'static str,
        /// The 1-indexed source line it was found on.
        line: usize,
    },
    /// The anchor the lint exists to check is absent: the file is empty, renamed or gutted, and an
    /// absence-based ban over it would otherwise be a vacuous pass.
    AnchorMissing {
        /// Which anchor.
        anchor: &'static str,
    },
}

impl BreakerLintViolation {
    /// A stable tag for the clause.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::ModuleLevelState { .. } => "module-level-state",
            Self::StateNotFromStore { .. } => "state-not-from-store",
            Self::AnchorMissing { .. } => "anchor-missing",
        }
    }
}

impl fmt::Display for BreakerLintViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModuleLevelState { form, line } => write!(
                f,
                "line {line} introduces `{form}` into the breaker's source: process-wide breaker \
                 state cannot be introduced (AC-034)"
            ),
            Self::StateNotFromStore { form, line } => write!(
                f,
                "line {line} introduces `{form}`, through which breaker state could come from \
                 somewhere other than a fold over the store (AC-035)"
            ),
            Self::AnchorMissing { anchor } => write!(
                f,
                "the breaker source carries no `{anchor}`, so an absence-based ban over it asserts \
                 nothing — a renamed or emptied file would otherwise pass every clause"
            ),
        }
    }
}

/// The four forms AC-034 bans, as stable tokens.
///
/// The criterion names three; `thread_local!` is the fourth and is **added rather than found** — it
/// is the same defect spelled differently, and saying so here is cheaper than a reader discovering
/// the gap.
pub const MODULE_STATE_FORMS: [&str; 4] = [STATIC_ITEM, "lazy_static", "OnceLock", "thread_local"];

/// The first of [`MODULE_STATE_FORMS`], named because it is the one that needs a syntactic match
/// rather than a substring one: `&'static str` contains it and is a lifetime, not state.
pub const STATIC_ITEM: &str = "static ";

/// The forms through which breaker state could arrive from somewhere other than the store.
///
/// An interior-mutability cell would hold it across a call, and a deserialiser would let an
/// agent-returned value become one. Neither is a fold.
pub const NOT_FROM_STORE_FORMS: [&str; 5] =
    ["Mutex<", "RefCell<", "Cell<", "Deserialize", "OnceCell"];

/// The anchors both lints assert before any absence, so a renamed or emptied file is not a pass.
pub const BREAKER_ANCHORS: [&str; 3] =
    ["pub struct BreakerState", "pub fn from_events", "&[Event]"];

/// **AC-034's lint half**: *"a lint banning module-level `static`/`lazy_static`/`OnceLock` breaker
/// state, so process-wide breaker state cannot be introduced."*
///
/// A source lint over the breaker module's own text, on the facet-lint pattern — the rule is a `fn`
/// in `crates/rk-kernel/src/` so its declaration is grep-visible the way §7b.14:1592-1594 makes the
/// three facet rules visible, and the caller supplies the source with `include_str!` so the rule
/// itself performs no I/O and stays inside the determinism gate.
///
/// **Comment lines are stripped before the scan.** This module's own header quotes the criterion,
/// which names all three banned forms; a lint that matched prose would be measuring its own
/// documentation. The [`BREAKER_ANCHORS`] are asserted **first**, against the stripped source, so an
/// emptied file fails rather than passing four absences.
///
/// # Errors
///
/// Every clause that failed, collected — never the first one only.
pub fn breaker_no_module_state_rule(src: &str) -> Result<(), Vec<BreakerLintViolation>> {
    let mut out = missing_anchors(src);
    // EVERY FORM, EVERY LINE, AND NO EARLY RETURN. A rule that returned on the first hit would
    // report one form per file and the four one-form controls would stop discriminating between
    // the forms they are named for (`facet_stability.rs`:203-205 is the shape).
    for (n, line) in code_lines(src) {
        for form in MODULE_STATE_FORMS {
            if carries(&line, form) {
                out.push(BreakerLintViolation::ModuleLevelState { form, line: n });
            }
        }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}

/// **AC-035's lint half**: *"a lint that breaker state is constructed only from the store."*
///
/// The positive clause is the anchor set — [`BreakerState::from_events`] exists and takes
/// `&[Event]` — and the negative clauses are the cells through which a state could survive a call
/// without being re-derived, plus the `Deserialize` that would let one arrive as data.
///
/// # Errors
///
/// Every clause that failed, collected — never the first one only.
pub fn breaker_state_from_store_rule(src: &str) -> Result<(), Vec<BreakerLintViolation>> {
    let mut out = missing_anchors(src);
    for (n, line) in code_lines(src) {
        for form in NOT_FROM_STORE_FORMS {
            if carries(&line, form) {
                out.push(BreakerLintViolation::StateNotFromStore { form, line: n });
            }
        }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}

/// Every [`BREAKER_ANCHORS`] entry the source does not carry — the positive clause both lints run
/// **first**, so an emptied or renamed file fails on the anchor rather than passing every absence.
fn missing_anchors(src: &str) -> Vec<BreakerLintViolation> {
    BREAKER_ANCHORS
        .into_iter()
        .filter(|anchor| !src.contains(anchor))
        .map(|anchor| BreakerLintViolation::AnchorMissing { anchor })
        .collect()
}

/// The source's non-comment lines with their string literals emptied, 1-indexed.
///
/// **Two strippings, and each one is the difference between a rule and a rule that fails on its own
/// source.**
///
/// *Comment lines* go because this module's header quotes AC-034, which **names all three banned
/// forms**: a lint that matched prose would be measuring its own documentation and would go red on
/// the file carrying the criterion it enforces. The caller strips them too; doing it here as well
/// makes the rule correct whichever source it is handed.
///
/// *String literals* go because [`MODULE_STATE_FORMS`] and [`NOT_FROM_STORE_FORMS`] are declared in
/// this very file, as arrays of the tokens they ban. A `contains` over the raw text would report the
/// lint's own vocabulary as four violations — which is a vacuous failure exactly as an unanchored
/// absence is a vacuous pass. The stripping is deliberately crude (everything between double quotes,
/// pairwise): a `static` item cannot hide inside a string literal, so nothing a real violation could
/// take is lost.
///
/// **`pub(crate)` as of 2026-09-14, Slice 4h.** AC-031's prose-extraction ban
/// ([`crate::runtime::activity::boundary_prose_extraction_rule`]) is a fourth source lint with the
/// same two stripping problems and the same answer to them. It reuses this function rather than
/// copying it, because two copies are two places for the string-literal stripping to be forgotten
/// and a source lint that reports its own vocabulary is a vacuous failure exactly as an unanchored
/// absence is a vacuous pass. The visibility is `pub(crate)` and not `pub`: it is part of no
/// criterion's surface, and no criterion names it.
pub(crate) fn code_lines(src: &str) -> impl Iterator<Item = (usize, String)> {
    src.lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l))
        .filter(|(_, l)| !l.trim_start().starts_with("//"))
        .map(|(n, l)| {
            let outside = l.split('"').step_by(2).collect::<Vec<_>>().join(" ");
            (n, outside)
        })
}

/// Whether one sanitised line carries one banned form.
///
/// [`STATIC_ITEM`] is matched as an **item declaration** and not as a substring, because
/// `&'static str` contains `static ` and appears on every `as_str` in this crate. AC-034 bans
/// *module-level `static` state*, which is a `static NAME: T = …;` item — and a lifetime is not
/// state. The other three forms have no such collision and are matched by containment.
fn carries(line: &str, form: &str) -> bool {
    if form == STATIC_ITEM {
        let trimmed = line.trim_start();
        // Drop a leading visibility — `pub`, `pub(crate)`, `pub(in crate::workflow)` — so
        // every spelling of a module-level `static` is caught by the one clause. The `") "` split
        // is what makes the parenthesised forms work; the plain-space split is the bare `pub`.
        let item = trimmed.strip_prefix("pub").map_or(trimmed, |rest| {
            rest.rsplit_once(") ")
                .map_or_else(
                    || rest.split_once(' ').map_or(rest, |(_, after)| after),
                    |(_, after)| after,
                )
                .trim_start()
        });
        return item.starts_with(form);
    }
    line.contains(form)
}
