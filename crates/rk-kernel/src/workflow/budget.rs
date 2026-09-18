//! **The three budget counters and the exhaustion gate** — AC-020–AC-025, the two quantities
//! `attempts_total` names, and the mark a run that reaches its wall clock owes.
//!
//! | criterion | what this module carries | state |
//! | --- | --- | --- |
//! | **AC-020** | [`Attempts`], [`Tokens`], [`WallClockSecs`] — three newtypes with no `From`, no `Add`, no `Sum` and no `Into<f64>` crossing them; [`fold`] touches one counter per [`Charge`] | **built**; the separation half is `tests/ui/ac020_counters_never_share.rs` |
//! | **AC-021** | [`required_attempts_total`] — lint rule 8's **declared** minimum | **built** |
//! | **AC-022** | [`charge_for_each`] / [`charge_pass`] — the **runtime** charge, constant in the fan-out cardinality | **built**; see the 🛑 on ADR-0053 below |
//! | **AC-023** | [`appetite_raise_from_activity_result`] — the refusal, and **only** the refusal | **built**; C3 is **RESOLVED** in favour of AC-389 (DECISIONS-2026-09-14 D-13) — an in-run raise is refused **outright**, AC-023's *"unless"* clause is **CUT** by dated register amendment and its second instrument is withdrawn **with** the clause, so the escape hatch is not unbuilt but **withdrawn**. What is still unbuilt is **AC-389's** second THEN half, *"the attempt is appended as an event"* — no `EventKind` exists for it and it is residual **R-1** in `spikes/DECISIONS-2026-09-15.md` |
//! | **AC-024** | [`wall_clock_expiry_mark`] — the durable mark's **value**, on [`EventKind::GateOpened`] with kind `budget_extension` | **partial**; the *write* has **no call site anywhere in `src/`** [re-measured 2026-09-14, after Slice 4h shipped without one: it has zero call sites; the bare grep returns four lines, the definition plus three doc mentions, so the count is stated as call sites and not as grep hits]. The sentence here read *"the *write* is Slice 4h's"*; 4h is committed and did not write it, so the forward reference is replaced by the measurement. `crate::runtime`'s header carries the reason the kernel is powerless to add one — no ledger, no envelope and no elapsed-seconds carrier — and the record routing an elapsed value to the kernel is **OWED**. Both halves ARMED below. **Amended 2026-09-14 (D-8):** this cell began *"*\"exits 0\"* is 7d's and the *write* has no call site"*; the first clause is withdrawn, because [`BudgetExtensionMark::to_event`] gives the mark an event form and `tests/it/cross_process.rs` re-execs the test binary, writes the mark in a **child** and reads that child's **exit code** — the register's own *"test asserting process exit and a written mark"*. The *write on a live `src/` path* is untouched by that and stays ARMED, measured as stated. AC-024 is a **twin** (owner: AC-240 + AC-278) |
//! | **AC-025** | [`BudgetExhausted`], [`GateVerdict`], [`evaluate_gate`] — and no route from the first to a closed second | **built**; the negative fixture is `tests/it/budget.rs`'s nine-row table, with `tests/ui/ac025_exhaustion_is_not_a_pass.rs` in addition |
//!
//! # The two quantities `attempts_total` names, and why they are two functions
//!
//! **ADR-0122 decision 3** (accepted 2026-09-14) is the load-bearing clause of this module, and it
//! is quoted rather than paraphrased because the whole shape of the file follows from it:
//!
//! > **AC-021 and AC-026 both hold, because `attempts_total` names two quantities and only one of
//! > them is AC-021's.** … **AC-021's quantity is the declared budget field.** Its `evidence:` line
//! > is *"activity-graph schema §9a"*, whose item 7 (:236) is lint rule 8; its only `decisions:`
//! > citation is **ADR-0037** … What AC-021 forbids is **summing the same steps across passes to
//! > inflate the declared minimum**: a two-pass graph does not require `2A`. … **AC-026's quantity
//! > is the runtime draw-down.** Schema §4 :83 puts both in one sentence and keeps them apart: the
//! > activity *declares* `budget`, **steps draw from it**, and inner harness retries **count**.
//!
//! So [`required_attempts_total`] is the **declaration** side — ADR-0037's `## Decision` entire:
//! *"Lint rule 8 becomes: `budget.attempts_total ≥ number of agent/tool steps on one pass`"* — and
//! [`fold`] over [`Charge`]s is the **draw-down** side, which **accumulates across passes**. The
//! alternative reading, in which AC-021 governs the runtime counter, is the one ADR-0122 decision 3
//! rejects on four grounds, and it is rejected here too: *"a counter invariant across passes can
//! never be exhausted by repeated passes, AC-025's `Err(BudgetExhausted)` becomes unreachable on the
//! retry path"*. The inner-attempt charge itself is **AC-026's and Slice 4c's**; nothing here counts
//! a `system/api_retry` event, and no function in this file may be read as having done so.
//!
//! # 🛑 AC-022 IS BUILT AGAINST ADR-0053's `Negative`, AND THE CONTRADICTION IS REPORTED, NOT FIXED
//!
//! Both texts, in full, on both sides. **AC-022's THEN** (`references/2026-09-08-acceptance-criteria.md`:491-495):
//!
//! > GIVEN a `for_each` step projecting over N items / WHEN the attempts budget is charged / THEN
//! > the step counts once, independent of N.
//! > *Verified by:* a parameterised unit test at N = 1, 2, 50 asserting a constant charge.
//!
//! **ADR-0053's `## Decision`**, verbatim and entire: *"Rule 8's step count treats a `for_each` step
//! as counting once, regardless of its runtime fan-out cardinality"* — a clause about **lint rule
//! 8**, a catalog-load-time count. Its `## Consequences` **Negative** says the opposite about the
//! runtime: *"a `for_each` over many elements can still exhaust `budget.attempts_total` at runtime
//! even though it passed the static lint check."*
//!
//! Under ADR-0092 the register owns AC-022's wording and instrument, so [`charge_for_each`] is built
//! to AC-022's THEN and **ADR-0053's `Negative` is contradicted by this build**. That is stated and
//! left: a stage may not resolve a record's ambiguity in code (`00-preamble.md`:55). **This is the
//! single most likely place for a later implementer to "fix" the test to match the ADR**, because
//! the ADR reads authoritative and its `Negative` reads like a rule; changing
//! `for_each_charges_once_independent_of_n` to accept a charge that scales with N is exactly the red
//! §7b.13 names, turned green against a `Consequences` paragraph.
//!
//! # 🛑 AC-023 IS BUILT TO THE REFUSAL BOTH CRITERIA SHARE, AND C3 IS NOW SETTLED
//!
//! **C3 is RESOLVED, in favour of AC-389: an in-run appetite raise is refused OUTRIGHT, and
//! AC-023's *"unless it arrives as a typed budget-change event with an author"* clause is CUT by
//! dated register amendment** (`DECISIONS-2026-09-14 D-13`). The paragraph that stood here read
//! *"C3 is open (`references/2026-09-08-acceptance-criteria.md`:117)"* — quoted verbatim; `:117`
//! was C3's row when that sentence was written and is **C1**'s row now, because this pass's own
//! register amendments moved it, so C3 is named here by its `| C3 |` cell and not by line
//! [re-derived 2026-09-15: `grep -n '^| C3 ' references/2026-09-08-acceptance-criteria.md`]. The
//! paragraph took the two refusals as irreconcilable — *"A build cannot implement both refusals."*
//! It is corrected rather than deleted, because **what this module builds did not move**: a raise
//! arriving **from an activity result** is refused, which is what
//! [`appetite_raise_from_activity_result`] is and all of it.
//!
//! The resolution's warrant is that the hatch was **not buildable at all**: it needs a typed
//! budget-change `EventKind`, `rk_types::event_kind::EventKind` is closed at 38 variants *(29 until ADR-0123, 31 until ADR-0124, 36 until ADR-0125 and 37 until ADR-0126, all 2026-09-17)* and
//! carries no budget or appetite change, and minting one takes a record (ADR-0116 clause 5;
//! ADR-0120 is the precedent). **AC-396 still routes the appetite VALUE to a human gate**, so
//! *"refused outright"* refuses the in-run mechanism and not the quantity.
//!
//! **AC-023's second instrument — *"an event-schema test that every appetite change carries an
//! author field"* — is WITHDRAWN WITH THE CLAUSE** rather than left ARMED against no stage: D-13
//! cuts the clause and its instrument together. AC-023's **only** surviving instrument is therefore
//! *"a unit test asserting refusal of an in-run raise from an activity result"*, which is exactly
//! `tests/it/budget.rs`'s `appetite_raise_from_an_activity_result_is_refused` — **so the taboo on
//! naming that test `ac023_…` is LIFTED.** Nothing is renamed here: Checkpoint 7b asserts literal
//! test names and a rename is a separate, checked act. Recorded, not done.
//!
//! 🛑 **AC-389's *"the attempt is appended as an event"* IS STILL NOT BUILT AND STILL OWED A
//! RECORD.** It was the half where the two criteria differed and D-13 did not build it: there is no
//! `EventKind` for a refused appetite raise, the enum is closed at 29, ADR-0112's eleven `StepError`
//! variants carry no member for one, and design §5's table has no row. It is residual **R-1** in
//! `spikes/DECISIONS-2026-09-15.md`, and C3's resolution must not be read as discharging it.
//!
//! # 🛑 THE EXHAUSTION BOUNDARY IS PER COUNTER AS OF 2026-09-14, AND IT IS A READING NO RECORD STATES
//!
//! **The true history first, because the sentence that stood here until 2026-09-14 got it wrong.**
//! That sentence read *"the rule that stood in this file until 2026-09-14 was `spent > declared`,
//! and it is now `spent >= declared`"*. **That is false.** The comparison is byte-identical at
//! `71b20c1` and at `6d1a8fa` — `envelope.X.get() > 0 && ledger.X >= envelope.X` at both, on all
//! three counters, and those are the only two commits that have ever touched this file. `spent >
//! declared` was **never committed**: it existed in an uncommitted working copy for minutes. The
//! same sentence also recast what the stage actually did — *"the existing rule was kept and the
//! contradiction recorded as OWED"*, which is what [`exhausted_counter`]'s own body said — into
//! *"the boundary was moved on a fail-closed reading"*, claiming a decision the stage did not take
//! and erasing the OWED record along with it. Both sites are corrected with the measurement.
//!
//! **What actually changed on 2026-09-14 is this**, and it is a change to attempts and tokens only:
//! the comparison is now **one rule per counter**, each following its own criterion's verb —
//! `spent > declared` for [`Attempts`] and [`Tokens`] (AC-025: *"a gate whose evidence-producing
//! activity **ran out** of any budget"*), `spent >= declared` for [`WallClockSecs`] (AC-024: *"a run
//! that **reaches** its wall-clock budget"*), with the declared-`0` guard kept on all three. The
//! wall clock's comparison is therefore unchanged from both commits above.
//!
//! **The defect the change repairs was live** [measured 2026-09-14]: **9 of the 102** shipped
//! activities declare exactly their lawful ADR-0037 rule-8 minimum, so under the uniform `>=` each
//! was exhausted by its own **successful** pass and [`evaluate_gate`] could never close a gate on
//! its evidence. The in-crate reproduction is `tests/fixtures/catalog/two-pass.toml`, which has the
//! same shape.
//!
//! **The dated note fixing the comparison per counter has LANDED, 2026-09-15** — a dated register
//! note against **AC-024/AC-025**, on operator decision `DECISIONS-2026-09-14 D-6`, which states
//! the per-counter split in the register's own voice. This paragraph opened *"A dated note fixing
//! the comparison per counter is still OWED"*; it is corrected rather than deleted, because the
//! warrant it carried is still the warrant — neither criterion states a comparison outright, those
//! two verbs are all either offers, and D-6 is what turns this file's reading of them into a
//! record's text. The reading is still pinned by `tests/it/budget.rs`'s
//! `minimum_declaration_is_not_exhausted_by_its_own_clean_pass`, whose name carries no AC id, now
//! for the narrower reason that D-6's instrument is a dated amendment against **two** criteria and
//! not a criterion of its own, so there is no id for the test to take.
//!
//! 🛑 **AND THE COMPARISON IS NOT EXERCISED END-TO-END AT 7b** [measured 2026-09-15, code lines
//! only]. [`evaluate_gate`] has **zero callers under any `crates/*/src/`** — the only code-line hit
//! is its own definition — and [`wall_clock_expiry_mark`]'s **write on a live `src/` path is still
//! ARMED**, as the AC-024 row above states. So D-6's repair is pinned by unit cases and by nothing
//! that dispatches. See [`exhausted_counter`], which stays the **single** definition —
//! [`fold`]'s observation and [`evaluate_gate`]'s verdict both call it, so the two cannot disagree.
//!
//! # What is deliberately NOT here
//!
//! **The sentence that stood here until 2026-09-14 said `BudgetExhausted`, `GateVerdict`,
//! `evaluate_gate` and the pack's accessors were *"Slice 4b's and 4c's and are not stubbed"*, and
//! the first three are now built.** The pack's `K`, `N` and per-item token appetite live in
//! [`super::envelope`] and [`crate::select::Pack`] — the declared numbers are the pack's, not this
//! module's. Still **not built and not stubbed**: the inner-attempt charge (AC-026), the retry
//! policy (AC-027–AC-032) and the breaker (AC-033–AC-035), all Slices 4c and 4d's — *"a module that
//! exists and returns nothing reads as built to the next reader"* (`00-preamble.md` golden
//! constraint 10). [`BudgetReport::exhausted`] stays an **observation**: it reports which counter
//! has run out and decides nothing, and the verdict is [`evaluate_gate`]'s alone.
//!
//! **What AC-024 does NOT cover here, named so a green is not misread** *(amended 2026-09-14)*: the
//! *"exits 0"* half is a **process** exit, and this sentence continued *"and this is a library crate
//! — there is no `rk-cli` before Step 7d"*. The premise is still true of this MODULE, which exits
//! nothing; the conclusion that followed from it elsewhere — that no test in this crate can measure
//! a process exit — is withdrawn under **D-8**, because `tests/it/cross_process.rs` re-execs the
//! test binary and reads a **child's** exit code. What is 7d's is that the exiting process is
//! `redkiln run`. Nothing in THIS module is that instrument. The mark's
//! *write* is outside the pure function (ADR-0119 clause 5, ADR-0120 clause 4) and is Slice 4h's,
//! and so is *"no further dispatch"*, which needs a dispatch counter. The gate record itself —
//! ADR-0017's `(token, tree_hash, deadline)` — is the FSM's (Slice 4e), because ADR-0105 clause 4
//! forbids a second `tree_hash` derivation anywhere but `rk-git`.
//!
//! **Nothing here defaults a policy number.** [`BudgetEnvelope`] derives no `Default`, on `Margins`'
//! exact model (AC-377: *"`Margins` implements no `Default`"*), and every envelope in this crate's
//! tests is read off a fixture's declared `budget = { attempts_total, tokens, wall_clock_secs }`
//! table — the shape all 102 shipped activities carry [measured 2026-09-14:
//! `grep -lE '^budget' spikes/p9/catalog/*.toml | wc -l` → 102 of 102 files].

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use super::retry::InnerAttempts;
use crate::catalog::{Activity, Evidence, Step, StepKind};

/// Attempts spent, per activity. **AC-020's first counter.**
///
/// The private field and the absent arithmetic are the criterion: *"the three are distinct newtypes
/// and no `From`/`Add` impl crosses them, so a shared counter does not compile."* The discipline is
/// `rk_types::digest::Digest`'s, verbatim — *"the type could only be constructed from data it was
/// given, so it could not silently mint an id."*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Attempts(u32);

impl Attempts {
    /// From a count the caller already has.
    #[must_use]
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    /// The count.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Not `Add`. **`pub(in crate::workflow)` deliberately**: an `impl Add` is what AC-020's
    /// `trybuild` case exists to forbid, and a public `plus` would let a caller assemble the shared
    /// counter by hand out of two public calls.
    ///
    /// Saturating, not wrapping: a budget counter that wraps to zero is a free budget, and the
    /// release profile's wrapping `+` would deliver one silently.
    #[must_use]
    pub(in crate::workflow) const fn plus(self, n: u32) -> Self {
        Self(self.0.saturating_add(n))
    }
}

impl fmt::Display for Attempts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} attempts", self.0)
    }
}

/// Tokens spent, per item. **AC-020's second counter**, and the one AC-023's appetite bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Tokens(u64);

impl Tokens {
    /// From a count the caller already has.
    #[must_use]
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    /// The count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Not `Add`; see [`Attempts::plus`].
    #[must_use]
    pub(in crate::workflow) const fn plus(self, n: u64) -> Self {
        Self(self.0.saturating_add(n))
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} tokens", self.0)
    }
}

/// Wall-clock seconds spent, per run. **AC-020's third counter.**
///
/// Seconds, and never an `Instant`: there is no clock on this side of the split (AC-001), and the
/// elapsed value reaches the workflow only as a journaled activity result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WallClockSecs(u64);

impl WallClockSecs {
    /// From a count the caller already has.
    #[must_use]
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    /// The count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Not `Add`; see [`Attempts::plus`].
    #[must_use]
    pub(in crate::workflow) const fn plus(self, n: u64) -> Self {
        Self(self.0.saturating_add(n))
    }
}

impl fmt::Display for WallClockSecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} wall-clock seconds", self.0)
    }
}

/// Which counter, and nothing else. A closed vocabulary with `ALL` and `as_str`, so a fourth
/// counter is a compile error at every sweep rather than a row a test forgot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Counter {
    Attempts,
    Tokens,
    WallClock,
}

impl Counter {
    /// The three, in the order a sweep reports them.
    pub const ALL: [Self; 3] = [Self::Attempts, Self::Tokens, Self::WallClock];

    /// A stable tag, so a test names the counter it expects without matching message text.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attempts => "attempts",
            Self::Tokens => "tokens",
            Self::WallClock => "wall_clock",
        }
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The declared envelope. **No `Default`, and that is the criterion.**
///
/// `K`, `N` and the per-item token appetite are *"declared in the pack and refused as code
/// defaults"* (§7b.10's 🛑; AC-033, AC-023), and an activity's `attempts_total`, `tokens` and
/// `wall_clock_secs` are declared in its catalog row (schema §4). A `Default` here would be the code
/// default the record forbids, on `Margins`' exact model (AC-377: *"`Margins` implements no
/// `Default`"*) and on `Pack::appetite`'s (`select/pack.rs`:249-256: *"**`None` is not
/// 'unbounded'**"*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetEnvelope {
    /// The **declared** field ADR-0037's lint rule 8 bounds from below. It is not the runtime
    /// counter; see [`required_attempts_total`] and ADR-0122 decision 3.
    pub attempts_total: Attempts,
    /// The item's token appetite, fixed at triage (AC-023, AC-389).
    pub tokens: Tokens,
    /// The run's wall clock.
    pub wall_clock_secs: WallClockSecs,
}

/// One charge against one counter. **Exactly one**, and that is AC-020's THEN as a type fact:
/// *"WHEN one of them is consumed or exhausted THEN neither of the other two changes."*
///
/// A charge carrying two magnitudes would be the shared counter the criterion forbids, one level up
/// from the missing `impl Add`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charge {
    Attempts(Attempts),
    Tokens(Tokens),
    WallClock(WallClockSecs),
}

impl Charge {
    /// Which counter this charge lands on.
    #[must_use]
    pub const fn counter(self) -> Counter {
        match self {
            Self::Attempts(_) => Counter::Attempts,
            Self::Tokens(_) => Counter::Tokens,
            Self::WallClock(_) => Counter::WallClock,
        }
    }
}

/// What has been spent.
///
/// **A fold over a charge sequence, never a mutable accumulator held across a process**: durable
/// state is read back out of the journal, and a ledger that lived in a process-lifetime cell would
/// answer differently in a second process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BudgetLedger {
    attempts: Attempts,
    tokens: Tokens,
    wall_clock: WallClockSecs,
}

impl BudgetLedger {
    /// Attempts spent.
    #[must_use]
    pub const fn attempts(self) -> Attempts {
        self.attempts
    }

    /// Tokens spent.
    #[must_use]
    pub const fn tokens(self) -> Tokens {
        self.tokens
    }

    /// Wall-clock seconds spent.
    #[must_use]
    pub const fn wall_clock(self) -> WallClockSecs {
        self.wall_clock
    }
}

/// What the fold measured, whether or not anything was charged.
///
/// House Shape C (`src/rigor.rs`:173-193): *"a rule that enumerated **no** family or **no** context
/// returns an empty violation list, which reads exactly like a clean catalog"* — so the per-counter
/// charge **counts** travel beside the totals, and AC-020's three pair-cases assert them. A ledger
/// reporting `attempts = 0` because nothing was charged is indistinguishable from one reporting it
/// because the charge was lost; `attempts_charges` tells the two apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BudgetReport {
    /// How many [`Charge::Attempts`] landed.
    pub attempts_charges: usize,
    /// How many [`Charge::Tokens`] landed.
    pub token_charges: usize,
    /// How many [`Charge::WallClock`] landed.
    pub wall_clock_charges: usize,
    /// The first counter of [`Counter::ALL`] whose spend has passed its declared limit.
    ///
    /// **An observation and never a verdict.** AC-024/AC-025's gate — `Err(BudgetExhausted)`, and
    /// the rule that no code path maps an exhausted budget to a pass — is Slice 4b's and is not
    /// here. Nothing in this module reads this field to decide anything.
    pub exhausted: Option<Counter>,
}

/// Fold a charge sequence into a ledger, and report what it measured.
///
/// **The fold accumulates across passes**, because the runtime draw-down is AC-026's quantity and
/// ADR-0122 decision 3 keeps it apart from AC-021's declared field: *"a counter invariant across
/// passes can never be exhausted by repeated passes, AC-025's `Err(BudgetExhausted)` becomes
/// unreachable on the retry path."* The declared minimum is [`required_attempts_total`] and it is a
/// different function over a different input.
///
/// `envelope` is read for one purpose only — [`BudgetReport::exhausted`], an observation. No charge
/// is clamped, dropped or converted; AC-390's *"no counter can be replenished, borrowed against or
/// converted into another"* is a type fact of [`Charge`] and is not re-checked at run time.
#[must_use]
pub fn fold(charges: &[Charge], envelope: &BudgetEnvelope) -> (BudgetLedger, BudgetReport) {
    let mut ledger = BudgetLedger::default();
    let mut report = BudgetReport::default();
    for charge in charges {
        // ONE COUNTER PER CHARGE, AND NO ARM TOUCHES A SECOND. This match is AC-020's THEN: an arm
        // that also bumped a sibling is the shared counter, and the three pair-cases in
        // `tests/it/budget.rs` are what would catch it.
        match *charge {
            Charge::Attempts(n) => {
                ledger.attempts = ledger.attempts.plus(n.get());
                report.attempts_charges += 1;
            }
            Charge::Tokens(n) => {
                ledger.tokens = ledger.tokens.plus(n.get());
                report.token_charges += 1;
            }
            Charge::WallClock(n) => {
                ledger.wall_clock = ledger.wall_clock.plus(n.get());
                report.wall_clock_charges += 1;
            }
        }
    }
    // ONE DEFINITION OF EXHAUSTION, and the observation here calls the same function the verdict
    // does, so the two cannot disagree (`exhausted_counter`, and AC-025's gate in `evaluate_gate`).
    report.exhausted = exhausted_counter(&ledger, envelope);
    (ledger, report)
}

/// A run's **observed** fan-out cardinalities, per step id.
///
/// Values only: this arrives from a journaled activity result and nothing here computes it. A step
/// with no entry ran once, which is a structural fact about a step that does not fan out and **not**
/// a defaulted policy value — the distinction `select/pack.rs`:249-256 draws for `appetite`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FanOut(BTreeMap<String, usize>);

impl FanOut {
    /// An empty observation: every step ran once.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that `step_id` fanned out over `items` items.
    #[must_use]
    pub fn with_observed(mut self, step_id: &str, items: usize) -> Self {
        self.0.insert(step_id.to_owned(), items);
        self
    }

    /// How many items `step_id` fanned out over. `1` for a step that does not fan out.
    #[must_use]
    pub fn observed(&self, step_id: &str) -> usize {
        self.0.get(step_id).copied().unwrap_or(1)
    }
}

/// **AC-022's runtime charge for one step**, at the fan-out cardinality the run actually produced.
///
/// > GIVEN a `for_each` step projecting over N items / WHEN the attempts budget is charged / THEN
/// > the step counts once, independent of N.
///
/// `_fan_out` is **taken and never read**, and the underscore is that THEN written into the
/// signature rather than into a comment: there is no expression in this body in which the observed
/// cardinality appears, so a charge that scales with N cannot be reached by a mistake — only by
/// someone deliberately wiring the parameter in, which is what
/// `for_each_charges_once_independent_of_n` goes red on. The parameter is kept rather than dropped
/// because dropping it would also delete the evidence that the cardinality was seen and refused.
///
/// Only `agent` and `tool` steps charge: they are the dispatchable ones, and `check`, `gate` and
/// `emit` mint no dispatch (ADR-0036; schema §9a item 7 counts *"agent/tool steps"*).
#[must_use]
pub const fn charge_for_each(step: &Step, _fan_out: usize) -> Attempts {
    match step.kind {
        StepKind::Agent | StepKind::Tool => Attempts::new(1),
        StepKind::Check | StepKind::Gate | StepKind::Emit => Attempts::new(0),
    }
}

/// **The runtime attempts charge for one pass** through an activity's graph.
///
/// Every step is charged through [`charge_for_each`], the fan-out step and the plain step alike, so
/// there is exactly **one** charge site and AC-022's constancy cannot be true of one path and false
/// of another.
///
/// This is the **draw-down**, AC-026's quantity, and it is charged again on every pass — see
/// [`fold`] and ADR-0122 decision 3. It is deliberately **not** what AC-021 asserts.
#[must_use]
pub fn charge_pass(activity: &Activity, fan_out: &FanOut) -> Attempts {
    let mut total = Attempts::new(0);
    for step in &activity.steps {
        total = total.plus(charge_for_each(step, fan_out.observed(&step.id)).get());
    }
    total
}

/// **AC-026's runtime charge, at ADR-0122 decision 1's unit — the ONE site that charges an inner
/// harness attempt.**
///
/// > An inner harness attempt charges the activity's attempt budget, **one charge per recorded
/// > `system/api_retry` event**, in the pass that dispatched it. … The unit is the recorded event,
/// > **not the terminal value of its `attempt` field**.
///
/// The charge lands on [`Counter::Attempts`] and on nothing else — one counter per [`Charge`], so
/// AC-020's separation holds through AC-026's accounting as it does through [`charge_pass`]'s.
///
/// **It is confined to one function called from one site deliberately.** Conflict C1 (AC-026/AC-232
/// vs AC-391's *Cut*) was open until 2026-09-14 and is **resolved by ADR-0122 for AC-026**; the
/// other reading — that harness-internal retries do not charge at all — is a one-line change here
/// and nowhere else. ADR-0122 decision 5 records what is still owed above this record: AC-391's
/// *Cut:* paragraph must be withdrawn by a dated register amendment, §0.3's C1 row gains a
/// resolution line, and AC-391's surviving class-scoped clause needs a dated clarification against
/// AC-026's *"the same failure"*. **Until (a) and (b) land, a stage claiming AC-391 reports it
/// armed, not discharged.**
///
/// The counting itself is [`super::retry::InnerAttempts::from_recorded`]; this function only turns
/// the count into a charge, so there is no second place where the unit could be chosen differently.
#[must_use]
pub const fn charge_observed_inner_attempts(observed: InnerAttempts) -> Charge {
    Charge::Attempts(Attempts::new(observed.get()))
}

/// **AC-021 / ADR-0037's lint rule 8**: the minimum an activity's `budget.attempts_total` may
/// declare.
///
/// > Lint rule 8 becomes: `budget.attempts_total ≥ number of agent/tool steps on one pass`.
///
/// *"One pass"* is the whole of it. The losing formula ADR-0037 replaced was `Σ (step retry.max ×
/// 1)` — its own `## Context` calls it *"a formula stated in terms of summed per-step retry
/// allowances"* — and a graph whose steps declare retries does **not** thereby require `2A`. That is
/// what ADR-0122 decision 3 calls AC-021's forbidden quantity: *"summing the same steps across
/// passes to inflate the declared minimum."*
///
/// A `for_each` step counts **once** (ADR-0053's `## Decision`), which is why this function and
/// [`charge_pass`] agree on a single pass while measuring different things.
///
/// **A reading, stated because no record states it:** the count is over the **declared** steps, a
/// `when`-guarded step included. Rule 8 runs at catalog-load time, before any run exists to evaluate
/// a guard, and ADR-0053's own driver is that the check *"runs at catalog-load time, before any run
/// exists to compute a `for_each` step's actual cardinality"*. Counting only unguarded steps would
/// need a runtime value at load, which is the shape that record rejects.
#[must_use]
pub fn required_attempts_total(activity: &Activity) -> Attempts {
    let mut total = Attempts::new(0);
    for step in &activity.steps {
        match step.kind {
            StepKind::Agent | StepKind::Tool => total = total.plus(1),
            StepKind::Check | StepKind::Gate | StepKind::Emit => {}
        }
    }
    total
}

/// The refusal AC-023 and AC-389 share.
///
/// It carries both magnitudes because both are the **same** counter — the item's token appetite,
/// before and after — so no arithmetic crosses a counter boundary here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppetiteRaiseRefused {
    /// The appetite as triage fixed it.
    pub fixed_at_triage: Tokens,
    /// What the activity result asked for.
    pub proposed: Tokens,
}

impl fmt::Display for AppetiteRaiseRefused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "an activity result asked to raise the item's token appetite from {} to {}, and the \
             raise is refused outright: the appetite is fixed at triage (AC-023, AC-389). C3 is \
             settled in favour of AC-389 (DECISIONS-2026-09-14 D-13) — the only route to a higher \
             appetite is returning the item to triage, and AC-396 routes the value to a human gate",
            self.fixed_at_triage, self.proposed
        )
    }
}

impl std::error::Error for AppetiteRaiseRefused {}

/// **AC-023 ∩ AC-389**: a mid-run appetite raise arriving on an activity result is refused.
///
/// `envelope` is taken by **shared** reference and a new envelope is never written back on this
/// path, which is AC-389's second instrument restated as a signature — *"a type-level check that the
/// appetite field is not mutable outside the triage transition."*
///
/// # Errors
///
/// [`AppetiteRaiseRefused`] — **always, for every input**, and that is the criterion rather than an
/// unfinished implementation. The `Ok` arm **was** where C3 split the two criteria: AC-023 admitted
/// a raise *"if it arrives as a typed budget-change event with an author"* and AC-389 admits none
/// at all. **C3 is RESOLVED in favour of AC-389 — refused outright — by DECISIONS-2026-09-14 D-13,
/// and AC-023's *"unless"* clause is CUT by dated register amendment**, so there is no hatch left
/// for the `Ok` arm to hold open. It is kept regardless: narrowing the signature to a non-`Result`
/// is a change at every call site that the resolution does not ask for, and **AC-389's own *"the
/// attempt is appended as an event"* is STILL NOT BUILT** — there is no `EventKind` for it, the
/// enum is closed at 29, and that half is residual **R-1** in `spikes/DECISIONS-2026-09-15.md`.
pub const fn appetite_raise_from_activity_result(
    envelope: &BudgetEnvelope,
    proposed: Tokens,
) -> Result<BudgetEnvelope, AppetiteRaiseRefused> {
    Err(AppetiteRaiseRefused {
        fixed_at_triage: envelope.tokens,
        proposed,
    })
}

// ---- AC-024 / AC-025 — the exhaustion gate (Slice 4b) ------------------------------------------

/// **AC-025's `Err(BudgetExhausted)`.** A `rk-kernel::budget` gate-evaluation result.
///
/// It is **not** a `StepError` and may not become one. Design §3.6:346 states the positive side,
/// verbatim: *"`BudgetExhausted` in the last row is **not** a `StepError`: it is
/// `rk-kernel::budget`'s gate-evaluation result (AC-024, AC-025), raised by the kernel after a
/// dispatch, never by an adapter."* ADR-0112's `**What this forbids**` states the negative:
/// *"Forbids a twelfth variant without a successor to this record"*, and ADR-0118 clause 2 declines
/// to be that successor — *"ADR-0112 is NOT superseded and the closed set stays at eleven … this
/// record deliberately does not become that successor."* A stage may not write the successor
/// (`00-preamble.md`:64), so the kernel-local error is the only route that takes no decision. It
/// never crosses a port: it is returned to the kernel's own gate evaluator.
///
/// It carries **only** the counter — no `spent`, no `limit`. Carrying both numbers would put two
/// counters' magnitudes into one struct and hand a caller the arithmetic AC-020 exists to forbid;
/// the numbers travel in [`BudgetReport`] instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetExhausted {
    /// Which of the three ran out. The first of [`Counter::ALL`] to have done so.
    pub counter: Counter,
}

impl fmt::Display for BudgetExhausted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the {} budget is exhausted, so the gate does not close (AC-025)",
            self.counter
        )
    }
}

impl std::error::Error for BudgetExhausted {}

/// A gate's verdict. **There is no route from an exhausted budget to a closed gate**, and the
/// restricted constructors are what make that a type fact rather than a convention.
///
/// Both variants are `#[non_exhaustive]`, so neither can be constructed outside this crate
/// (`E0639`), and the two constructors are `pub(in crate::workflow)` — `RequiredSet`'s exact
/// doctrine (`src/select.rs`:97-105): *"a `pub(crate)` constructor, or a `#[cfg(test)]` public one,
/// silently converts a type fact into a convention."* A caller outside the workflow reads a verdict
/// through [`GateVerdict::is_closed`] and [`GateVerdict::reason`] and cannot mint one.
///
/// `Closed` means **the gate closes** — the evidence is in and the run proceeds. `Open` means it
/// does not, and the reason says which evidence is missing. AC-038's degradation reason lands on
/// this same type at Slice 4d; nothing here reads a model tier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateVerdict {
    /// The gate closes.
    #[non_exhaustive]
    Closed,
    /// The gate does not close, and the reason names what is missing.
    #[non_exhaustive]
    Open {
        /// What is missing, named so a reader does not have to re-derive it.
        reason: String,
    },
}

impl GateVerdict {
    /// The closing verdict. **`pub(in crate::workflow)`**: AC-025's *"no code path maps an
    /// exhausted budget to `pass`"* is enforced by there being no path to this constructor from
    /// outside the module that also owns [`evaluate_gate`].
    #[must_use]
    pub(in crate::workflow) const fn closed() -> Self {
        Self::Closed
    }

    /// The non-closing verdict, carrying its reason.
    #[must_use]
    pub(in crate::workflow) const fn open(reason: String) -> Self {
        Self::Open { reason }
    }

    /// Whether the gate closes.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }

    /// The reason the gate did not close, or `None` when it closed.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Closed => None,
            Self::Open { reason } => Some(reason),
        }
    }
}

/// What a gate demanded and what the evidence-producing activity actually minted.
///
/// The vocabulary is the catalog's closed [`Evidence`] enum (activity-graph schema §2, ADR-0036),
/// not a string set: a gate that demanded a kind no step can mint would otherwise be a typo that
/// never closes. Both sides are `BTreeSet`s, so the missing list is in byte order and the same on
/// every platform — `#![deny(clippy::disallowed_types)]` on the parent module bans the unordered
/// pair outright (AC-004).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GateEvidence {
    demanded: BTreeSet<Evidence>,
    produced: BTreeSet<Evidence>,
}

impl GateEvidence {
    /// From what the gate demands and what the activity minted.
    #[must_use]
    pub fn new(
        demanded: impl IntoIterator<Item = Evidence>,
        produced: impl IntoIterator<Item = Evidence>,
    ) -> Self {
        Self {
            demanded: demanded.into_iter().collect(),
            produced: produced.into_iter().collect(),
        }
    }

    /// What the gate demanded and did not get, in byte order.
    #[must_use]
    pub fn missing(&self) -> Vec<Evidence> {
        self.demanded.difference(&self.produced).copied().collect()
    }

    /// Whether every demanded kind was minted.
    #[must_use]
    pub fn is_satisfied(&self) -> bool {
        self.missing().is_empty()
    }
}

/// Which counter, if any, has run out — **the one definition of exhaustion in this crate**.
///
/// [`fold`] reports it as an observation and [`evaluate_gate`] refuses on it, and both call this, so
/// the observation and the verdict cannot disagree. The sweep is over [`Counter::ALL`], so a fourth
/// counter is a compile error here and not a row someone forgot.
///
/// **The true history first, because the sentence that stood here until 2026-09-14 got it wrong.**
/// That sentence read *"the rule that stood in this file on 2026-09-14 was `spent > declared`; it
/// is now `spent >= declared`"*. **That is false.** The comparison is byte-identical at `71b20c1`
/// and at `6d1a8fa` — `envelope.X.get() > 0 && ledger.X >= envelope.X` on all three counters, and
/// those are the only two commits that have ever touched this file. `spent > declared` was **never
/// committed**. What this function's body actually said on 2026-09-14 is what the stage actually
/// did: *"`>=` … is kept"*, with the contradiction recorded as **OWED**. The module header carries
/// the measurement in full.
///
/// **A reading, stated because no record states it, and it is PER COUNTER as of 2026-09-14.** What
/// changed is attempts and tokens only; the wall clock's comparison is unchanged from both commits
/// above. Each counter follows its own criterion's verb:
///
/// * **AC-025** — *"a gate whose evidence-producing activity **ran out** of any budget"* → `spent >
///   declared` for [`Counter::Attempts`] and [`Counter::Tokens`]. An activity that spent exactly
///   what it declared spent its budget; it did not run out of it.
/// * **AC-024** — *"a run that **reaches** its wall-clock budget"* → `spent >= declared` for
///   [`Counter::WallClock`]. *"Reaches"* is the criterion's own word and it names the equality.
///
/// The declared-`0` guard is kept on all three: nothing runs out of a budget it never drew on.
///
/// **This stays ONE function** with a per-counter rule rather than becoming three, so [`fold`]'s
/// observation and [`evaluate_gate`]'s verdict still cannot disagree.
///
/// **The dated note fixing the comparison per counter has LANDED, 2026-09-15**, against
/// **AC-024/AC-025**, on operator decision `DECISIONS-2026-09-14 D-6`. This paragraph read *"A
/// dated note fixing the comparison per counter is OWED … Until that note lands, a reader must not
/// take the per-counter split as settled"*, and it is corrected in place: neither criterion states a
/// comparison outright — those two verbs are all either offers — so the rule above followed each
/// criterion's own verb, and D-6 is what makes that a record's text rather than this file's reading.
/// It is still pinned by `tests/it/budget.rs`'s
/// `minimum_declaration_is_not_exhausted_by_its_own_clean_pass`, whose name carries no AC id —
/// because D-6's instrument is an amendment against two criteria, not an id a test could take.
///
/// 🛑 **Not exercised end-to-end at 7b** [measured 2026-09-15, code lines only]: [`evaluate_gate`]
/// has **zero callers under any `crates/*/src/`** and [`wall_clock_expiry_mark`]'s write on a live
/// `src/` path is still **ARMED**, so nothing that dispatches reaches this comparison.
#[must_use]
pub fn exhausted_counter(ledger: &BudgetLedger, envelope: &BudgetEnvelope) -> Option<Counter> {
    // 🛑 THE CONTRADICTION WITH ADR-0037's RULE 8 THAT LIVED IN THIS EXPRESSION IS REPAIRED HERE,
    // AND THE READING IT TAKES IS NOW A RECORD'S. A dated register note against AC-024/AC-025
    // landed 2026-09-15 on operator decision DECISIONS-2026-09-14 D-6. This line read "AND THE
    // READING IT TAKES IS STILL OWED A RECORD" until then.
    //
    // *(Found 2026-09-14 by an adversarial pass, which reproduced it against the in-crate fixture
    // `tests/fixtures/catalog/two-pass.toml`: `attempts_total = 3`, `required_attempts_total` = 3,
    // a clean pass charges 3 -> under a uniform `>=`, `exhausted = Some(Attempts)` and
    // `evaluate_gate` returns `Err(BudgetExhausted)`.)*
    //
    // Rule 8, verbatim: *"`budget.attempts_total >= number of agent/tool steps on one pass`"* — so
    // declaring EXACTLY the lawful minimum is lawful, and one clean pass then spends exactly the
    // declared amount. **The defect was live, not theoretical** [measured 2026-09-14]: 9 of the 102
    // shipped activities declare exactly their lawful minimum (`change-summary` 2/2,
    // `change-summary-ue` 3/3, `checklist-review` 2/2, `debt-logging` 2/2, `dependency-update` 6/6,
    // `rolling-deploy` 4/4, `smoke-test` 2/2, `t-shirt-sizing` 2/2, `unit-tests` 4/4), so under a
    // uniform `>=` each was exhausted by its own SUCCESSFUL pass and `evaluate_gate` could never
    // close a gate on its evidence.
    //
    // The comparison now follows each criterion's own verb — `>` for attempts and tokens (AC-025,
    // *"ran out"*), `>=` for the wall clock (AC-024, *"reaches"*) — and the doc comment above
    // carries the warrant and the dated AC-024/AC-025 note that now records it (D-6); that clause
    // read "the OWED record" until 2026-09-15. Design §3.6:344 and schema §4:83 both name the
    // OUTCOME (`Err(BudgetExhausted)`, `Blocked { reason }`) and neither fixes the COMPARISON.
    //
    // The declared-`0` guard is unchanged and is load-bearing for the wall clock's `>=`: without
    // it, a declared 0 against an EMPTY ledger reads as exhausted and mints a budget-extension mark
    // for a run that has done nothing.
    Counter::ALL.into_iter().find(|counter| match counter {
        // AC-025: *"ran out"* — spending exactly the declared number is not running out of it.
        Counter::Attempts => {
            envelope.attempts_total.get() > 0 && ledger.attempts > envelope.attempts_total
        }
        // AC-025 again, the same verb over the same kind of quantity.
        Counter::Tokens => envelope.tokens.get() > 0 && ledger.tokens > envelope.tokens,
        // AC-024: *"reaches"* — reaching the declared wall clock IS reaching it.
        Counter::WallClock => {
            envelope.wall_clock_secs.get() > 0 && ledger.wall_clock >= envelope.wall_clock_secs
        }
    })
}

/// **AC-025.** Evaluate a gate against what the evidence-producing activity spent and minted.
///
/// > GIVEN a gate whose evidence-producing activity ran out of any budget / WHEN the gate is
/// > evaluated / THEN the result is `Err(BudgetExhausted)` and the gate does not close.
///
/// The exhaustion check is **first**, and it returns rather than falling through: there is no
/// expression in this body in which an exhausted ledger reaches a [`GateVerdict`], which is the
/// *"no code path maps an exhausted budget to `pass`"* half written into the control flow. The
/// type-level half is `tests/ui/ac025_exhaustion_is_not_a_pass.rs` — no `From<BudgetExhausted>` for
/// `GateVerdict`, and `GateVerdict::Closed` is not constructible outside `crate::workflow`.
///
/// # Errors
///
/// [`BudgetExhausted`] naming the first counter of [`Counter::ALL`] that has run out.
pub fn evaluate_gate(
    ledger: &BudgetLedger,
    envelope: &BudgetEnvelope,
    evidence: &GateEvidence,
) -> Result<GateVerdict, BudgetExhausted> {
    // FIRST, AND IT RETURNS. Every later line in this function is unreachable for an exhausted
    // ledger, which is AC-025's "no code path maps an exhausted budget to `pass`" written into the
    // control flow rather than into a comment.
    if let Some(counter) = exhausted_counter(ledger, envelope) {
        return Err(BudgetExhausted { counter });
    }
    let missing = evidence.missing();
    if missing.is_empty() {
        return Ok(GateVerdict::closed());
    }
    let named = missing
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    Ok(GateVerdict::open(format!(
        "the gate demands evidence the activity did not mint: {named}"
    )))
}

/// **AC-024's durable mark**, as a value. The *write* is the runtime's and is not here.
///
/// > GIVEN a run that reaches its wall-clock budget / WHEN the budget expires / THEN the kernel
/// > writes a durable mark and exits 0; no thread sleeps waiting for the budget to be extended.
///
/// The **carrier** is [`EventKind::GateOpened`] with `kind = "budget_extension"`, and that is a
/// reading of design §2.9's table rather than a record's word: `EventKind` is closed at 38 variants *(29 until ADR-0123, 31 until ADR-0124, 36 until ADR-0125, 37 until ADR-0126)*
/// (ADR-0116's *"a kind in the design table and not in the enum … is schema drift"*) and none of
/// them is a budget mark, so the choice is between existing kinds. `GateOpened` is the one AC-024's
/// own `decisions:` line points at — ADR-0017 defines the gate record `(token, tree_hash, deadline)`
/// and is cited by AC-024, AC-030, AC-035 and AC-039 alike — and a budget's expiry opens exactly the
/// thing a human is being asked to answer: an extension. **`GateExpired` was considered and
/// rejected**: it is a *gate's own* deadline passing, not a run's budget, and using it would make
/// the two indistinguishable in a replay.
///
/// **What this struct is not.** It is not the gate record. ADR-0017's triple is `(token, tree_hash,
/// deadline)`; the `tree_hash` is `rk-git`'s and ADR-0105 clause 4 forbids a second derivation, and
/// the `token` is a content digest over the whole record. Both are the FSM's (Slice 4e). This is
/// what the **budget** hands the gate writer, and the append itself is outside the pure function
/// (ADR-0119 clause 5, ADR-0120 clause 4) and is Slice 4h's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetExtensionMark {
    spent: WallClockSecs,
    declared: WallClockSecs,
}

impl BudgetExtensionMark {
    /// The gate kind this mark opens. A stable tag, matched by tests rather than message text.
    pub const KIND: &'static str = "budget_extension";

    /// The event kind that carries it — one of the closed 38 (37 until ADR-0126, 2026-09-17), minted
    /// by nobody here.
    #[must_use]
    pub const fn event_kind() -> EventKind {
        EventKind::GateOpened
    }

    /// The gate kind, as the record's payload spells it.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        Self::KIND
    }

    /// Wall-clock seconds spent, as the journal reported them.
    #[must_use]
    pub const fn spent(&self) -> WallClockSecs {
        self.spent
    }

    /// The declared wall clock this run reached.
    #[must_use]
    pub const fn declared(&self) -> WallClockSecs {
        self.declared
    }

    /// The mark **as the event that carries it** — added 2026-09-14 for D-8.
    ///
    /// `crate::runtime::marks::RetryAtMark::to_event` is the shape, one kind over, so the journal
    /// carries **one** convention for a mark rather than two: a `kind` tag that tells the mark
    /// apart from the other producer of the same [`EventKind`], and the mark's own members beside
    /// it. `gate_opened` has two producers here exactly as `rate_limited` does — the FSM's gate
    /// record and this — and a reader that guessed between them would be silently wrong, which is
    /// what [`Self::KIND`] exists to prevent.
    ///
    /// **This is the mark's FORM and not its WRITE.** The append is outside the pure function
    /// (ADR-0119 clause 5, ADR-0120 clause 4) and nothing in `src/` calls
    /// [`wall_clock_expiry_mark`] even now — see that function's own doc for the re-measured count.
    /// What this method changes is that a caller which *has* decided to write the mark no longer
    /// has to invent a payload shape of its own.
    ///
    /// **The payload's field set is a reading, and so is `RetryAtMark`'s**: `rk_types::event::Event`
    /// carries `payload: serde_json::Value` and the typed-payload record, per kind, is **OWED**
    /// (`rk-types/src/event.rs`:53-59). Nothing here mints an `EventKind` — the carrier is
    /// [`Self::event_kind`], one of the closed 38 (37 until ADR-0126, 2026-09-17).
    #[must_use]
    pub fn to_event(self) -> Event {
        Event::new(
            Self::event_kind(),
            serde_json::json!({
                "kind": Self::KIND,
                "spent": self.spent.get(),
                "declared": self.declared.get(),
            }),
        )
    }

    /// Rebuild a mark the journal already carries.
    ///
    /// **The readback's constructor, and it decides nothing.** [`wall_clock_expiry_mark`] stays the
    /// only place that decides *whether* a mark is owed; this only re-reads one that was already
    /// written. `crate::runtime::marks::RetryAtMark::carried` is the precedent — a public
    /// constructor used by the producer and the reader alike.
    #[must_use]
    pub const fn from_journal(spent: WallClockSecs, declared: WallClockSecs) -> Self {
        Self { spent, declared }
    }
}

/// Every budget-extension mark the journal holds, in append order — added 2026-09-14 for D-8.
///
/// **A `gate_opened` row that is not this mark is skipped rather than defaulted**, and a row that
/// claims to be one but is missing a member is skipped too. Design §5 gives `gate_opened` to both
/// the FSM's gate record and this mark; `kind = "budget_extension"` is what tells them apart, and
/// the shape is `crate::runtime::marks::read_retry_at_marks`' verbatim — one kind, two
/// propositions, and a fold that guesses between them is a fold that can be wrong silently.
#[must_use]
pub fn read_budget_extension_marks(events: &[Event]) -> Vec<BudgetExtensionMark> {
    events
        .iter()
        .filter(|e| e.kind == BudgetExtensionMark::event_kind())
        .filter(|e| {
            e.payload.get("kind").and_then(serde_json::Value::as_str)
                == Some(BudgetExtensionMark::KIND)
        })
        .filter_map(|e| {
            let spent = e.payload.get("spent").and_then(serde_json::Value::as_u64)?;
            let declared = e
                .payload
                .get("declared")
                .and_then(serde_json::Value::as_u64)?;
            Some(BudgetExtensionMark::from_journal(
                WallClockSecs::new(spent),
                WallClockSecs::new(declared),
            ))
        })
        .collect()
}

/// **AC-024, half 1.** The mark a run that has reached its wall-clock budget owes, or `None`.
///
/// `ledger` is a fold over journaled charges and `envelope` is a declaration: **there is no clock in
/// this function and none in this module**, so *"no thread sleeps waiting for the budget to be
/// extended"* is a property of the signature and not of a timer that happens to be absent. The
/// elapsed value reaches the workflow only as a journaled activity result (AC-001), and
/// `#![deny(clippy::disallowed_methods)]` on the parent module is the structural half (AC-003),
/// re-run here rather than re-claimed.
///
/// **ARMED, NOT DISCHARGED, and named rather than assumed.** This paragraph opened *"AC-024's
/// *\"exits 0\"* is a **process** exit and `rk-kernel` is a library — there is no `rk-cli` before Step
/// 7d, and a test that called `std::process::exit` would kill the harness rather than assert
/// anything"* until 2026-09-14. The clause about `std::process::exit` is still true and is still
/// why no test asserts its OWN exit; the conclusion drawn from it is withdrawn under **D-8**, which
/// built `tests/it/cross_process.rs`'s `ac024_wall_clock_mark_outlives_the_process_that_wrote_it` —
/// a child process that reaches its wall-clock budget, writes [`BudgetExtensionMark::to_event`]'s
/// row and exits, with the PARENT reading both. What is still 7d's is that the exiting process is
/// `redkiln run`. **What is still ARMED here and is not moved by any of that:** the *write* of this
/// mark on a live `src/` path, and *"no further dispatch"*, which needs a dispatch counter — both
/// the runtime's. AC-024 is also a **twin**, not the owner: the owner is the human-gate pair
/// AC-240 + AC-278, and this half may not be double-claimed against that area.
#[must_use]
pub fn wall_clock_expiry_mark(
    ledger: &BudgetLedger,
    envelope: &BudgetEnvelope,
) -> Option<BudgetExtensionMark> {
    // THE SAME BOUNDARY `exhausted_counter` TAKES FOR `Counter::WallClock`, and stated per counter
    // because as of 2026-09-14 the three counters no longer share one comparison: AC-024's verb is
    // *"reaches"*, so this is `>=` — while attempts and tokens take AC-025's *"ran out"* and are
    // `>`. This is a second expression rather than a call, and the two are kept in step by
    // `exhaustion_ac024_ac025`'s obligation 4, which asserts the mark at `wall` and its absence at
    // `wall - 1`.
    (ledger.wall_clock >= envelope.wall_clock_secs).then_some(BudgetExtensionMark {
        spent: ledger.wall_clock,
        declared: envelope.wall_clock_secs,
    })
}
