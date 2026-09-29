//! **The circuit breaker** — the counting rule, the per-key isolation and the store-derived state
//! of AC-033–AC-035, keyed on the model family the **pack** declares for a step's role.
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
//! # THE KEY IS THE PACK'S `[role.<name>] family`, RESOLVED FROM THE STEP'S ROLE — DECIDED
//! 2026-09-14
//!
//! Eight occurrences of *"family"* across those three criteria, five of them the key the breaker is
//! partitioned by. **Operator decision D-7, 2026-09-14:**
//!
//! > *"The pack declares the family per role (extending design :480's 'tier policy per role'); the
//! > kernel resolves role → family at plan time and keys the breaker on the resolved family."*
//!
//! The producer is [`crate::select::Pack::require_family_for_role`] and its input is
//! `crate::catalog::Step::role`, which `catalog.rs` began parsing on the same day and had parsed
//! **zero** times before it while the shipped catalog declared one **185 times in 8 distinct
//! values**. D-7 was chosen over three alternatives *"because it is the only one consistent with
//! records already written: the pack owns roles and tier policy (design :480-481, ADR-0014), `K` is
//! **already** a pack value asserted by pack validation (AC-033's own Verified by), and the breaker
//! expiry's home falls out in the same place."* It needs no ADR supersession, re-words no
//! criterion's GIVEN, and touches neither `ActivitySpec` nor `rk-types` — `ActivitySpec` is still
//! *"ADR-0099:61's five fields and no sixth"* and no sixth was added. *(dated 2026-09-29, s8f build, package S8F-P7; ADR-0190 decision 2, proposed; D21-107.)*
//! A sixth was added LATER, for another reason: `activity_id_at_version`, for the adapter's
//! `dispatch_completed` row; the breaker still reads nothing off the spec.
//!
//! **CORRECTED HERE, NOT QUIETLY REPLACED.** The section that stood here until 2026-09-14 was
//! headed *"🛑 THE KEY HAS NO CARRIER"* and said *"**nothing in this tree represents a model
//! family**"* [measured: `grep -rn "family\|ModelFamily\|model_tier" crates/rk-types/src/
//! crates/rk-ports/src/` → 0 hits], concluding that the binding *"is **ARMED, NOT DISCHARGED** and
//! is reported against Step 7c. A record is OWED."* The measurement it rested on is **still true and
//! was re-taken today** — there is no model family in `rk-types` or `rk-ports`, and none was added:
//! the carrier D-7 chose is a **pack declaration**, which is a TOML table and not a type, so the
//! type-level absence the old sentence measured was never the thing that blocked it. What is
//! withdrawn is the conclusion, not the measurement.
//!
//! **What is still OWED, and it is narrower than what stood here.** The key *spellings* —
//! `[role.<name>] family` and `[breaker] expiry_secs` — are a **reading**, exactly as
//! [`super::envelope`]'s 🛑 already records for `[retry] attempts_cap`,
//! `[breaker] consecutive_platform_failures` and `appetite.tokens`; a record naming all five is
//! OWED. And **whether `claude-opus-4` and `claude-opus-4-5` are one family or two is still settled
//! nowhere** — it is now a question about what a *pack author* writes in the table rather than about
//! what this crate can represent, and the shipped packs' own tables are **Step 7e's** (§7b.15's
//! AC-080–AC-118 shape: rules and fixtures here, shipped artifacts there). [measured 2026-09-14:
//! `grep -rn role spikes/p9/packs/` → **0 hits across all twelve shipped packs**.] The pinning test
//! `breaker_key_is_the_packs_role_family_never_the_activity_family` carries **no AC id** for that
//! reason, on `appetite_raise_from_an_activity_result_is_refused`'s precedent.
//!
//! # 🛑 AND THE CATALOG'S `family` IS STILL A DIFFERENT FAMILY — THE DANGER GREW RATHER THAN SHRANK
//!
//! **`crates/rk-kernel/src/catalog.rs`'s `family` is ADR-0058's ACTIVITY family and may never be
//! read here.** ADR-0058: *"An activity declares its family membership with a `family = "<name>"`
//! field. Lint checks three things for every declared family: that its members share `provides`,
//! that they hold distinct `method_rank` values, and that the family satisfies rule 26's
//! totality-over-rigor property."* That is a rank-escalation ladder — the shipped fixture family is
//! `root-cause(10) < differential-debugging(20) < bisect(30) < instrumented-trace-analysis(40)` —
//! and every member of it would typically run on the **same** model. A breaker keyed on it would
//! trip globally with respect to the model and locally with respect to the method, which is the
//! exact inverse of AC-034 on both axes. **The distance between the right answer and the worst one
//! is a single `.family`.**
//!
//! Until 2026-09-14 that risk was theoretical here, because nothing produced a key at all and the
//! paragraph above said so. **D-7 made it live**: the resolution now runs at plan time off an
//! activity's steps, so a value carrying the *other* `family` is in scope at the moment the key is
//! built. Two instruments answer it, and neither existed before today:
//!
//! 1. **Behavioural** — `breaker_key_is_the_packs_role_family_never_the_activity_family` resolves
//!    two roles of **one** fixture activity that declares `family = "localisation"`, and asserts the
//!    two keys DIFFER, that neither is `localisation`, that `localisation` is *refused* as a role,
//!    and that a journal tripping one role's family leaves the other proceeding. The old
//!    instrument could not do this: every case in `tests/it/breaker.rs` supplied its own arbitrary
//!    token, so a fold reading the activity family passed all of them, and that file's own header
//!    said so in terms.
//! 2. **Structural** — [`family_key_never_from_the_activity_catalog_rule`], run over **both** this
//!    file and `src/select/pack.rs` (the two files the substitution could be written in), banning
//!    `::catalog` on their code lines.
//!
//! [`FamilyKey`] still has one constructor, and it is still [`FamilyKey::supplied`] — the resolution
//! **calls** it rather than replacing it, because the fold must mint a key from a token it reads off
//! a journaled payload and the two keys have to compare equal. What changed is that there is now a
//! lawful *producer* of the token, and [`FamilyKey::supplied`]'s own doc says which call is which.
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
//! # THE EXPIRY'S HOME IS PACK-DECLARED, BESIDE `K` — DECIDED 2026-09-14
//!
//! AC-035's *"until the mark is read and found expired"* names a duration. **D-7 gave it a home in
//! the same motion as the key**, and that co-location is one of the decision's own stated reasons
//! for choosing the pack table: *"the breaker expiry's home falls out in the same place."* It is
//! read from `[breaker] expiry_secs`, beside `[breaker] consecutive_platform_failures`, and the
//! refusal is [`crate::select::Pack::require_breaker_expiry`]. The two `[breaker]` members are
//! **independent absences**: a pack may declare `K` and no expiry, and
//! `breaker_key_is_the_packs_role_family_never_the_activity_family` asserts that pair against
//! `runtime-envelope.toml`, which declares exactly it.
//!
//! **CORRECTED HERE, NOT QUIETLY REPLACED.** The section that stood here until 2026-09-14 was headed
//! *"🛑 THE EXPIRY HAS NO VALUE AND NO HOME, AND THIS MODULE DOES NOT GIVE IT ONE"* and listed four
//! structurally available homes — *"pack-declared beside `K`; a kernel policy constant beside the
//! backoff pair, which contradicts PIA-055's own exhaustive split; derived from the backoff cap,
//! which is a default with arithmetic in front of it; or no expiry at all, which re-words a THEN a
//! stage may not touch"* — and said **"None is chosen here."** The operator chose the **first** of
//! those four, and the module's own reasons for ranking it first stand unedited above it. The
//! measurement the section rested on is still true and is not withdrawn: §7b.10:852-856's two lists
//! contain no expiry [re-measured 2026-09-14: `grep -n "expir" runbook/phase-1/s7b-kernel.md`
//! returns nothing in §7b.10], which is why the choice was the operator's to make and not this
//! stage's.
//!
//! **What is still true of the type, and is load-bearing.** [`BreakerExpiry`] still has one
//! constructor, [`BreakerExpiry::declared`], **no `Default`**, no `const` fallback and no
//! `unwrap_or`, and [`BreakerState::decide`] still takes it **as an argument**. Nothing in this file
//! names a number; the value enters from a pack. The *value* is still sized by no corpus figure —
//! only its home is settled — and **a record naming the key spelling `expiry_secs` is OWED** with
//! the other four.
//!
//! **And the `elapsed >= expiry` boundary is unchanged**, for the reason reading 2 below states.
//!
//! # 🛑 THE DURABLE MARK HAS NO CARRIER — AC-033's SECOND CONJUNCT IS ARMED
//!
//! `EventKind` is closed at ~~38~~ 41 variants *(29 until ADR-0123, 31 until ADR-0124, 36 until ADR-0125 and 37 until ADR-0126, all 2026-09-17; 38 until ADR-0185, proposed)* *(dated 2026-09-28, `s8e` build, package R3FIX-8e: ADR-0185's three kinds made `ALL` 41)* (`event_kind.rs`'s `ALL`, *"Built from an exhaustive match so a
//! new variant cannot be added without appearing here"* — cited by its sentence and not its line:
//! this header said `:87` across three mint commits while the constant sat at `:118` and now
//! `:145` [re-measured 2026-09-17, Step 7c Slice 1]) and none is a breaker mark — ADR-0126's
//! `dispatch_completed` (Step 7c Slice 1) is the adapter's usage-bearing telemetry row and is folded
//! below in the no-state-change arm; `RefusalClass`
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
//!    *"found expired"* falls on. The sibling reading is already on disk, and it is the **wall
//!    clock's**: `budget.rs`'s `exhausted_counter` takes `spent >= declared` for
//!    `Counter::WallClock`, on AC-024's verb *"reaches"* — an ELAPSED-TIME quantity is reached when
//!    the elapsed value equals it, which is exactly this expiry's kind. (Its attempts and token
//!    counters take `>` on AC-025's *"ran out"* as of 2026-09-14; those are draw-down quantities
//!    and are not this one's sibling.) Both sides are asserted by `breaker_ac033_ac035`.
//!
//! # What IS discharged here
//!
//! AC-034's two-family property over two arbitrary distinct keys, and its lint
//! ([`breaker_no_module_state_rule`]); AC-035's own named instrument, *"a lint that breaker state is
//! constructed only from the store"* ([`breaker_state_from_store_rule`]), plus the survival of a
//! dropped-and-rebuilt state over the same events; and AC-033's counting rule, its refusal, and
//! `K` read from a pack and refused as a code default.
//!
//! **Added 2026-09-14 (D-8)**: that survival across a **real** process boundary, in
//! `tests/it/cross_process.rs` rather than here — the dropped-and-rebuilt form above is the
//! in-process one and ADR-0087 clause 2 does not admit it as evidence. **No production code in this
//! module moved for it.**
//!
//! **Added 2026-09-14 (D-7)**: the **key's own binding** — the pack's role → family table resolved
//! through [`crate::select::Pack::require_family_for_role`] and keyed on here — the **expiry's
//! home**, pack-declared and refused as a code default beside `K`, and a third lint,
//! [`family_key_never_from_the_activity_catalog_rule`]. AC-034's isolation is now additionally
//! asserted over **resolved** keys (two roles of one activity) rather than over two arbitrary tokens
//! only. All three lints are declared as `fn`s in `crates/rk-kernel/src/` on the facet-lint pattern,
//! so the declaration is grep-visible the way §7b.14:1592-1594 makes the three facet rules visible.
//!
//! # What is deliberately NOT here
//!
//! No clock: the expiry compares against a [`JournaledNow`] **argument**, and the parent module's
//! `#![deny(clippy::disallowed_methods)]` reaches this child, so a `SystemTime::now()` here is a
//! non-zero clippy exit and not a review comment. No `static`, no `lazy_static`, no `OnceLock`, no
//! `thread_local`, no `Mutex`, no `RefCell` — AC-034's lint runs over this file's own source. No
//! dispatch and no append: the refusal is a **decision value**. No widening of the trip condition:
//! the breaker keys on `Platform` **only** (AC-033's GIVEN), and no record licenses `Transport` or
//! `Timeout` tripping it.
//!
//! 🛑 **AND NOTHING CALLS [`BreakerState::decide`] ON A PRODUCTION PATH — CORRECTED 2026-09-14.**
//! The clause above read *"…and the runtime boundary that would act on it is **Slice 4h's**"*, which
//! was a forward reference when it was written and is a **stale** one now: Slice 4h shipped,
//! [`crate::runtime`] dispatches at `runtime/activity.rs`:311, and it does **not** consult this
//! module [measured 2026-09-14: `grep -rn "\.decide(" crates/rk-kernel/src` → **0 hits**;
//! `runtime/activity.rs` imports [`JournaledNow`] and [`code_lines`] from here and nothing else].
//! So the gap is not *"a later slice will do it"* — it is that AC-033's WHEN (*"the next dispatch
//! for that family is requested"*) has a live dispatch beside it and no wire between them, which
//! `runtime.rs`:53 already reports for AC-033's mark half.
//!
//! *(Corrected 2026-09-26, `s8d` build, package P6; R4 = ADR-0176, `proposed`.)* **The call site exists
//! now, and this module did not move for it.** `runtime/activity.rs`'s `Runtime::breaker_refusal` calls
//! [`BreakerState::from_events`] and [`BreakerState::decide`] before every agent step's dispatch attempt
//! whose family is bound, over the store's journal, and refuses an open family before the port. The
//! rows it folds are the kernel's own (`activity::error_row`, and an agent step's commit row carrying
//! `family`). The paragraph above and the one below stand as the record of the state before R4.
//!
//! **D-7 did not change that and did not claim to.** Its work is the **key**: a producer for the
//! family token, its expiry's home, and the ban on the other `family`. The consequence, stated
//! rather than left for a reader to infer: the role → family resolution is **pinned by
//! `breaker_key_is_the_packs_role_family_never_the_activity_family` rather than exercised by a
//! runtime path**, exactly as `StepContext.output_schema`'s binding is pinned by
//! `component_eight_is_the_whole_schema_id_and_never_a_parsed_version` and by no production call
//! site. Wiring the refusal into the dispatch is a separate piece of work that this decision neither
//! performs nor authorises.
//!
//! **On concurrency (residual 10m).** [`BreakerState`] is a fold over values with no cell of any
//! kind, so it holds nothing across a call and nothing to interleave. ADR-0076 — *"the kernel never
//! observes interleaving"* — and ADR-0115 clause 1's `Arc<Mutex<dyn EventStore>>` put AC-012's
//! concurrency instrument at the composition root, and a second process lifetime is simply a second
//! fold over the same store. That is what makes AC-035's survival assertion a property of this
//! function rather than of a process.
//!
//! **Amended 2026-09-14 (D-8), and the amendment is that the claim is now MEASURED rather than
//! argued.** *"A second process lifetime is simply a second fold over the same store"* was an
//! argument from this function's shape, and ADR-0087 clause 2 says an in-process reload *"is not
//! evidence"* for the proposition it is being offered for. `tests/it/cross_process.rs`'s
//! `ac035_breaker_refusal_survives_a_real_process_death` re-execs the test binary, has the CHILD
//! open the breaker and die, and folds what the child left on disk in a process that never saw the
//! failures — AC-035's own *Verified by*, *"a kill-and-re-enter test"*. Nothing in **this module**
//! changed for it: the property is still the fold's, and that is exactly why the boundary could be
//! crossed without a line of production code moving.

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

/// The breaker's partition key — **a token this crate transcribes and never computes from its own
/// data.**
///
/// There is exactly one constructor and it takes the token from the caller. As of **2026-09-14**
/// there are exactly two lawful callers, and naming them is the point of keeping the constructor
/// single:
///
/// | caller | what it transcribes |
/// | --- | --- |
/// | [`crate::select::Pack::require_family_for_role`] | the **pack's** `[role.<name>] family` — the PLAN-TIME producer (D-7), and the only route by which a dispatch decision gets a key |
/// | [`BreakerState::from_events`] | the `family` member of a **journaled event payload** — the token the fold reads back, which must compare equal to the one above or the mark is for a different partition |
///
/// **What is still forbidden, and the type is what forbids it**: a
/// `FamilyKey::from_activity(&Activity)` would have to read `catalog.rs`'s `family` — ADR-0058's
/// **activity** family — and would be the substitution the module header's second 🛑 describes. No
/// such constructor exists, [`family_key_never_from_the_activity_catalog_rule`] bans the module path
/// that would be needed to write one in either file, and
/// `breaker_key_is_the_packs_role_family_never_the_activity_family` catches it behaviourally.
///
/// **The name stays `supplied` and was not renamed with the decision.** It is still accurate — every
/// call site transcribes something this crate did not compute — and it is `Pack`'s `require_…` call
/// that carries the provenance, so a reader looking for *"where does a key come from"* finds a
/// producer with a name rather than a constructor with an adjective.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FamilyKey(String);

impl FamilyKey {
    /// From a token the caller supplied. **The only constructor**, and named `supplied` rather than
    /// `new` because every call site is transcribing something this crate did not compute. The two
    /// lawful callers are named in the type's own doc.
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

/// How long an open breaker mark stands. **Declared, never defaulted, and homed in the PACK as of
/// 2026-09-14.**
///
/// The home is `[breaker] expiry_secs` and the refusal is
/// [`crate::select::Pack::require_breaker_expiry`] — operator decision D-7, which chose the first of
/// the four homes the module header listed. No `Default`, no `const` fallback, no `unwrap_or` and no
/// derivation from the backoff cap: the value reaches [`BreakerState::decide`] as an argument, from
/// a pack.
///
/// *(This doc said "**homed by no record**" and "four homes are available and none is chosen here"
/// until 2026-09-14. The home is chosen; the **value** is still sized by no corpus figure and the
/// key spelling is still a reading with a record OWED.)*
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
    /// Total over the closed set (`EventKind::ALL`) with **no catch-all arm**, so a new kind is an
    /// `E0004` (~~the closed 38 kinds … a thirty-ninth kind~~ *(dated 2026-09-28, `s8e` build, package R3FIX-8e: ADR-0185's three kinds made `ALL` 41)*)
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

            // TOTAL OVER THE CLOSED 36, WITH NO CATCH-ALL ARM. A thirty-seventh kind is an `E0004`
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
                // (`event.rs`:20-29) — and including `dispatch_completed` (ADR-0126,
                // 2026-09-17), the adapter's usage-bearing row: it carries the cost, the usage
                // and the readings of a dispatch whose outcome the `step_*` rows already
                // classified, and it is not a breaker mark (decision 4: tier-2 telemetry;
                // decision 5: the kernel folds it and moves nothing).
                EventKind::StageTransition
                | EventKind::GateOpened
                | EventKind::GateAnswered
                | EventKind::GateExpired
                | EventKind::SliceSealed
                | EventKind::ReconcileStarted
                | EventKind::GateAnswerVoid
                | EventKind::GateAnswerSpent
                | EventKind::GateResumed
                | EventKind::ReviewBlock
                | EventKind::QuarantineGranted
                | EventKind::QuarantineRemoved
                | EventKind::QuarantineExpired
                | EventKind::SelectionIneffective
                | EventKind::StageExitRefused
                | EventKind::DagAssembled
                | EventKind::DagAmended
                | EventKind::MemoHit
                | EventKind::MemoMiss
                | EventKind::MemoEvicted
                | EventKind::Dispatch
                | EventKind::DispatchCompleted
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
                | EventKind::AdvisoryDismissed
                | EventKind::WorktreeAssigned
                | EventKind::StageEmpty
                | EventKind::GitOp => {}
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
        // is `>=`, `budget.rs`'s `exhausted_counter` reading for the WALL-CLOCK counter (AC-024's
        // *"reaches"*): reaching the declared number is reaching it. Its attempts and token
        // counters take `>` instead — a different verb over a different kind of quantity.
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
    /// **AC-033/AC-034, added 2026-09-14 (D-7).** A form through which the breaker's partition key
    /// could become ADR-0058's **activity** family instead of the pack's model family.
    ReadsTheActivityCatalog {
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
            Self::ReadsTheActivityCatalog { .. } => "reads-the-activity-catalog",
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
            Self::ReadsTheActivityCatalog { form, line } => write!(
                f,
                "line {line} reads `{form}`: the activity catalog carries ADR-0058's ACTIVITY \
                 family, and a breaker keyed on it trips globally with respect to the model and \
                 locally with respect to the method — the exact inverse of AC-034 on both axes. The \
                 key is the PACK's `[role.<name>] family`, resolved from the step's role \
                 (`Pack::require_family_for_role`, 2026-09-14)"
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

/// The one form through which the breaker's key could become ADR-0058's **activity** family.
///
/// **One entry, and the single entry is the point.** The activity family lives on
/// `crate::catalog::Activity` and nowhere else in this tree — `crate::catalog` has **no crate-root
/// re-export under another name** [measured 2026-09-14: the only `pub use`s in
/// `crates/rk-kernel/src/lib.rs` are `budget`, `retry`, `memo` and `activity`] — so the *module
/// path* is the whole surface. Every route to it, a `use` or a fully-qualified expression or an
/// alias, has to write `::catalog` on a code line of the file that takes it. A longer list of
/// spellings would read as more rigour and cover strictly less.
///
/// **The separator is load-bearing and is not decoration.** The bare token `catalog` matches this
/// module's own `family_key_never_from_the_activity_catalog_rule`, so a ban on it would be red
/// against the correct file — a vacuous *failure*, exactly as an unanchored absence is a vacuous
/// *pass*. And it is **not** `.family`: [`OpenMark`] has a `family` field, `self.family` and
/// `mark.family` appear on code lines of this module, and that ban would be red against the correct
/// file too.
pub const NOT_THE_ACTIVITY_FAMILY_FORMS: [&str; 1] = ["::catalog"];

/// The anchors the three breaker lints assert before any absence, so a renamed or emptied file is
/// not a pass.
pub const BREAKER_ANCHORS: [&str; 3] =
    ["pub struct BreakerState", "pub fn from_events", "&[Event]"];

/// The anchors [`family_key_never_from_the_activity_catalog_rule`] asserts when it is run over
/// `crates/rk-kernel/src/select/pack.rs` — **the second file the ban has to cover**, because the
/// role → family resolution lives there and a lint that scanned only `breaker.rs` would leave the
/// one expression that could read the wrong `family` unguarded.
pub const ROLE_RESOLUTION_ANCHORS: [&str; 3] = [
    "pub fn require_family_for_role",
    "role_families",
    "RoleFamilyNotDeclared",
];

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
    let mut out = missing_anchors(src, &BREAKER_ANCHORS);
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
    let mut out = missing_anchors(src, &BREAKER_ANCHORS);
    for (n, line) in code_lines(src) {
        for form in NOT_FROM_STORE_FORMS {
            if carries(&line, form) {
                out.push(BreakerLintViolation::StateNotFromStore { form, line: n });
            }
        }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}

/// **The key's own lint, added 2026-09-14 (operator decision D-7).**
///
/// D-7 gave the breaker a real partition key — the pack's `[role.<name>] family`, resolved from
/// `crate::catalog::Step::role` — and in the same motion made a second `family` reachable from the
/// same plan-time code path: `crate::catalog::Activity::family`, which is **ADR-0058's activity
/// family**. The two are one field access apart and the consequence of confusing them is not a
/// smaller breaker but an inverted one: the activity family is a rank-escalation ladder
/// (`root-cause(10) < differential-debugging(20) < bisect(30) <
/// instrumented-trace-analysis(40)`) whose members would typically all run on the **same** model, so
/// a breaker keyed on it trips globally with respect to the model and locally with respect to the
/// method — AC-034 inverted on both axes.
///
/// **This rule exists because the behavioural instrument alone could not see the substitution.**
/// Before today every case in `tests/it/breaker.rs` supplied its own arbitrary token, so a fold that
/// read the activity family would have passed all of them; the test file's own header said so. The
/// pin `breaker_key_is_the_packs_role_family_never_the_activity_family` now catches it
/// behaviourally over a fixture activity that carries both, and this rule catches it structurally
/// in the two files where it would be written.
///
/// **It is run over two sources, and the anchors say which**: [`BREAKER_ANCHORS`] for
/// `src/workflow/breaker.rs`, [`ROLE_RESOLUTION_ANCHORS`] for `src/select/pack.rs`. Comment lines
/// and string literals are stripped by [`code_lines`], so this module's own header — which names
/// `crate::catalog` repeatedly, on purpose — is not what the rule measures.
///
/// # Errors
///
/// Every clause that failed, collected — never the first one only.
pub fn family_key_never_from_the_activity_catalog_rule(
    src: &str,
    anchors: &[&'static str],
) -> Result<(), Vec<BreakerLintViolation>> {
    let mut out = missing_anchors(src, anchors);
    for (n, line) in code_lines(src) {
        for form in NOT_THE_ACTIVITY_FAMILY_FORMS {
            if carries(&line, form) {
                out.push(BreakerLintViolation::ReadsTheActivityCatalog { form, line: n });
            }
        }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}

/// Every anchor the source does not carry — the positive clause all three lints run **first**, so an
/// emptied or renamed file fails on the anchor rather than passing every absence.
fn missing_anchors(src: &str, anchors: &[&'static str]) -> Vec<BreakerLintViolation> {
    anchors
        .iter()
        .copied()
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
