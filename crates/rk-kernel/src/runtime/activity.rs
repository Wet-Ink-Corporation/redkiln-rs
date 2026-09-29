//! **The activity execution loop** — the retry loop, the boundary's schema check, and the
//! late-duplicate discard. AC-009, AC-010, AC-011, AC-012, AC-013, AC-030, AC-031, AC-032.
//!
//! > **AC-011** — WHEN the kernel retries it / THEN the new dispatch carries `attempt + 1` and never
//! > reuses a previous attempt number. *Verified by:* a unit test over the retry loop asserting
//! > strictly increasing `attempt`; an event-log assertion that `(item, activity_id@version,
//! > attempt)` is **unique across the whole log**.
//!
//! > **AC-012** — GIVEN two attempts of the same `(item, activity_id@version)` step activation where
//! > the first has already committed its result / WHEN the second attempt returns late / THEN the
//! > late result is discarded as a duplicate and the committed value is unchanged.
//!
//! # The three functions, and why they are three and not one
//!
//! [`Runtime::run_activity`] is the loop. It is composed of [`Runtime::dispatch_once`] and
//! [`Runtime::commit`], and the split is **AC-012's instrument**, not a decomposition for its own
//! sake: the criterion's GIVEN is *"the second attempt **returns late**"*, which means the second
//! attempt has already dispatched when it finds the activation committed. A single function holding
//! the store lock from its pre-flight check through its append can never produce that state, so a
//! concurrency test written against it would assert a discard that no interleaving can reach. With
//! the two halves public, two threads can each pass the pre-flight, each dispatch, and then race on
//! the commit — which is the arrival AC-012 names.
//!
//! **The commit's read and its append are ONE lock acquisition**, and that is the whole of the
//! discard: the guard is taken once, the journal is re-read under it, and the append happens or does
//! not. Two acquisitions would leave a window in which both attempts see no commit and both append.
//!
//! # 🛑 The concurrency test's shape is constrained by ADR-0118, and the constraint is not obvious
//!
//! ADR-0118 clause 1: *"**A poisoned store lock halts the run.** The run stops at the poisoned
//! acquisition. No `StepError` is constructed, nothing crosses a port boundary, no retry, no next
//! activity, no gate answer."* Clause 4 makes that govern **both** mutexes. So a test thread that
//! panics — which is what a failed `assert!` is — **while holding the store lock** does not produce
//! a test failure under the decided semantics; it produces a halt, and clause 3's forbids bar
//! recovering from it. `ac012_late_duplicate_is_discarded` therefore makes **every** assertion on
//! the joining thread and none inside a worker.
//!
//! # The boundary's schema check — AC-031, and what it deliberately is not
//!
//! > **AC-031** — GIVEN a dispatch whose returned value fails schema validation **at the boundary** /
//! > WHEN the kernel handles it / THEN it re-prompts exactly once carrying the validation error, and
//! > on a second failure returns `Schema { attempts: 2 }`.
//!
//! The check is `typed.schema != spec.schema` and nothing more. `rk_types::typed::Typed` is
//! *"an erased, validated value: `serde_json::Value` **plus the schema id it validated against**"*,
//! so the identity of that id against the one the spec demanded is the whole of what this crate can
//! check without a schema registry — and a schema registry is `rk-types::schema`'s and the
//! adapter's, not the boundary's. **The ceiling of one re-prompt is not a constant declared here**:
//! it is `super::super::workflow::retry::SCHEMA_REPROMPT_LIMIT`, read through
//! `crate::workflow::retry::decide`, so the loop and the policy cannot disagree about how many.
//!
//! **AC-031's second instrument is [`boundary_prose_extraction_rule`]**, and it is a separate
//! instrument rather than a clause of the first: the register's own sub-clause says *"the boundary
//! never extracts structure from prose … verified separately by a grep-lint over the boundary module
//! rejecting any markdown/JSON-from-prose extraction path, with a must-fail fixture."* A build that
//! did only the scripted-responder half would have satisfied half a criterion. AC-031 is **compound
//! by the register's own admission** (*"the prose-extraction ban will be under-tracked until the
//! range widens"*) and is reported, not claimed whole.
//!
//! # AC-032 is a type fact here, and the door is `crate::workflow::retry::consult`
//!
//! A `Findings` value never reaches this loop's error channel: the loop's only `Err` source is
//! `rk_ports::harness::DispatchOutcome::result`, whose error type is `StepError`, and the only
//! function that turns an outcome into a retry decision is `retry::consult`. Slice 4c's
//! `disjoint_channels_ac032` measures that; nothing here widens it.
//!
//! # 🛑 WHAT IS MEASURED AND WHAT IS NOT
//!
//! | clause | state | why |
//! | --- | --- | --- |
//! | AC-009's *"recorded verbatim"* | **measured** — the `activity_committed` payload's `result` is the dispatch's `Typed.value` **unnormalised**, asserted against a vector whose keys are not in sorted order | a runtime that canonicalised on the way in would pass a canon-to-canon comparison and fail this one |
//! | AC-009's *"never re-elicited"* | **measured** — a second `Runtime` over the same store dispatches the classifier zero times | |
//! | AC-009's *"the vector is stable"* | **never asserted, and the register forbids asserting it** — *"it does not claim the vector is stable and must not be read as licensing selection on an unstable vector"* | |
//! | AC-013's *"process was killed"* | **ARMED** — the process boundary here is a **dropped `Runtime`**, not a `taskkill`. ADR-0087 clause 2: *"a reload that reads state back in-process is not a valid implementation of this decision and is not evidence for it"*. **ROUTED TO 7d, 2026-09-14 (D-8)**: a real re-exec harness now exists in `tests/it/cross_process.rs`, and it is deliberately not pointed here — AC-013's WHEN is *"WHEN `redkiln run <item>` re-enters"*, which is `rk-cli`'s | and AC-013 is **PROVISIONAL** (P8 PARTIAL, 3 of 27+3, one recorded step). **BUILT, NOT CLAIMED** |
//! | AC-030's *"exits 0"* | **ARMED** — **ROUTED TO 7d, 2026-09-14 (D-8)**. This cell read *"a library crate has no process exit; `rk-cli` at 7d"*; the first clause is now too wide — `tests/it/cross_process.rs` reads a **child's** exit for AC-024. AC-030 is not routed there because its own *Verified by* names *"the next **`redkiln run`** resumes from the mark (AC-013's re-entry test)"*: the binary and a PROVISIONAL criterion, neither of which is 7b's | residual 10d |
//! | AC-030's *"the next run resumes from the mark"* | **ARMED** — it rides on AC-013, which may not be counted | the register says so |
//! | AC-031's prose-extraction ban | **measured** by [`boundary_prose_extraction_rule`] over this file, with a committed must-fail fixture and a positive control per form | AC-031 stays **compound** and is not claimed whole |

use rk_store::memo::IdempotencyKey;
use rk_types::error::StepError;
use rk_types::event::{ErrorEvent, Event};
use rk_types::event_kind::EventKind;
use rk_types::typed::Typed;

use super::marks::RetryAtMark;
use super::{Runtime, RuntimeError};
use crate::workflow::breaker::{BreakerDecision, BreakerState, FamilyKey, JournaledNow};
use crate::workflow::envelope::AttemptCap;
use crate::workflow::idem::{CommitKey, IterationPath, checked_key};
use crate::workflow::retry::{RetryDecision, decide};

/// **R1: where a step activation commits** — its `[[step]]` id and its iteration path.
///
/// `s8d` §8d.2; record R1, proposed. With the item and `activity_id@version` it is the commit key
/// (D21-42 3b). `None` wherever this module takes an `Option<StepAt>` is the ACTIVATION grain,
/// unchanged: the kernel's machinery suites, which have no catalog activity behind them.
#[derive(Debug, Clone, Copy)]
pub struct StepAt<'a> {
    /// The `[[step]]` id — never the DAG node id.
    pub step_id: &'a str,
    /// The frames the step activation sits in; empty outside any loop.
    pub iteration_path: &'a IterationPath,
    /// **R4 (ADR-0176, proposed).** The model family the pack binds this step's role to, when a
    /// breaker is bound (D-7's key). `None` is a runtime with no breaker binding — the kernel's
    /// machinery suites — and a non-agent step.
    pub family: Option<&'a FamilyKey>,
}

impl StepAt<'_> {
    /// The commit key this step activation commits under.
    #[must_use]
    pub fn commit_key(&self, item: &str, activity_id_at_version: &str) -> CommitKey {
        CommitKey {
            item: item.to_owned(),
            activity_id_at_version: activity_id_at_version.to_owned(),
            step_id: self.step_id.to_owned(),
            iteration_path: self.iteration_path.clone(),
        }
    }
}

// ---- the values the loop produces ---------------------------------------------------------------

/// What one trip through [`Runtime::dispatch_once`] produced.
#[derive(Debug, Clone, PartialEq)]
pub struct DispatchAttempt {
    key: IdempotencyKey,
    dispatched: bool,
    result: Option<Result<Typed, StepError>>,
}

impl DispatchAttempt {
    /// The key this attempt carried. Validated before any port was touched.
    #[must_use]
    pub const fn key(&self) -> &IdempotencyKey {
        &self.key
    }

    /// Whether the `Harness` port was reached at all. `false` when the activation was already
    /// committed — AC-013's re-entry path.
    #[must_use]
    pub const fn dispatched(&self) -> bool {
        self.dispatched
    }

    /// The step's own outcome, and `None` when nothing was dispatched.
    #[must_use]
    pub const fn result(&self) -> Option<&Result<Typed, StepError>> {
        self.result.as_ref()
    }
}

/// **AC-012's verdict**, as a value rather than as a side effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitOutcome {
    /// One `activity_committed` was appended for this activation.
    Committed { key: IdempotencyKey },
    /// The activation was already committed. **The late result is discarded and nothing is
    /// appended**; the committed value is untouched.
    DiscardedLate {
        key: IdempotencyKey,
        /// The attempt that won, read off the journal — so the discard names what it deferred to.
        committed_attempt: u32,
    },
}

impl CommitOutcome {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Committed { .. } => "committed",
            Self::DiscardedLate { .. } => "discarded_late",
        }
    }

    #[must_use]
    pub const fn is_committed(&self) -> bool {
        matches!(self, Self::Committed { .. })
    }
}

/// House Shape C over one activity run: the counts travel with the outcome, so *"the loop did
/// nothing"* is distinguishable from *"the loop ran and reported nothing"*.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivityReport {
    attempts: Vec<u32>,
    dispatches: u32,
    decisions: Vec<RetryDecision>,
    outcome: Option<Result<Typed, StepError>>,
    commit: Option<CommitOutcome>,
    mark: Option<RetryAtMark>,
    appended: usize,
    schema_failures: u32,
    now: JournaledNow,
    /// **ADR-0144.** A step that failed AFTER this activation's dispatch had already committed.
    ///
    /// Separate from [`ActivityReport::outcome`] and deliberately so: `outcome` is the
    /// **dispatch's** result and for a committed activation it is `None` (the value went to the
    /// journal). Writing the later step's error there would overwrite the dispatch's own reading
    /// and reproduce ADR-0143's defect one field over. This is a *second* fact about the same
    /// activation — *it committed, AND a later step in its walk refused* — and both halves are
    /// needed: the commit stands (AC-013's grain is per activation, ADR-0138) and the refusal is
    /// the one thing the operator can act on.
    step_error: Option<StepError>,
    /// **ADR-0145.** The token of the gate this activation's walk OPENED, when it reached one.
    ///
    /// A gate is neither a success nor a failure: design §4.5 has the kernel write the mark and the
    /// run **exit 0**, resuming into the same step. So it can travel on neither `outcome` nor
    /// `step_error`, and `commit` is `None` for a walk that gated before dispatching. It is the
    /// token and not a `bool` because the token is what `terminal_read` trailers and what
    /// `RunState::awaiting_gate` answers with — a reader that had to go looking for it in the
    /// journal would be re-deriving a value the walk already held.
    gate: Option<rk_types::digest::Digest>,
    /// **R1.** Step activations this walk SERVED from the journal — committed by an earlier run and
    /// never re-dispatched (AC-013 at step grain). `0` on every report this module's loop builds.
    served: u32,
    /// **R1.** Whether this activation's COMPLETION row is journalled by this walk: the `emit`
    /// step's commit, the one `fsm::next_action` moves past a node on.
    completed: bool,
    /// **R1.** The per-step memo facts (`crate::workflow::memo::StepContext`) of every agent step
    /// this walk DISPATCHED, each assembled with that step's own card digest and schema id
    /// (§8d.2). The memo is still consulted by nothing on the run path; this is what it would be
    /// keyed on.
    memo_contexts: Vec<crate::workflow::memo::StepContext>,
    /// *(2026-09-26, `s8d` build, package P5; R3 (m14), written `proposed`.)* **The attempt charge
    /// per pass**, one entry per pass this walk charged, in the order each pass was first charged.
    /// A pass is an iteration path with its `for_each` frames removed: a `for_each` counts once
    /// (AC-022), while each loop-back frame and each `repeat` frame is a pass of its own (AC-021's
    /// per-pass count). Every agent or tool step the walk EXECUTED in a pass charges that pass
    /// through `budget::charge_for_each`; a step SERVED from its commit charges nothing.
    /// *(dated 2026-09-27, `s8d` build, package M14-8d.)* ~~through `budget::charge_for_each`~~ —
    /// a non-`for_each` step charges its pass once per DISPATCH it made, a kernel re-dispatch
    /// included (`budget::charge_dispatched`; accepted ADR-0122 decision 1); a `for_each` step keeps
    /// `budget::charge_for_each`'s once-whatever-N rule (AC-022).
    charges: Vec<(IterationPath, u32)>,
    /// **R4.** The breaker's refusal, when the executor refused the dispatch before the port.
    breaker: Option<BreakerDecision>,
}

impl ActivityReport {
    /// A report for a run that has not dispatched yet. **Private**: a report is produced by the
    /// loop and never minted beside it, so every count in one is a reading.
    /// **The step walk's refusal**: an activation that never reached a dispatch because a step
    /// refused before one (`super::step`).
    ///
    /// `pub(super)` rather than `pub`, and a *named* constructor rather than widening
    /// [`ActivityReport::opened`], because the invariant above still holds: every count in the
    /// report it returns is `0`, and that is a **reading** — nothing was dispatched, nothing
    /// committed, nothing was appended. The outcome it carries is the step's own error, which
    /// AC-032 makes a value rather than an error of the walk.
    pub(super) fn refused(now: JournaledNow, error: StepError) -> Self {
        let mut report = Self::opened(now);
        report.outcome = Some(Err(error));
        report
    }

    /// **ADR-0144: this activation COMMITTED, and a later step in its walk then failed.**
    ///
    /// Consumes the committed report and returns it carrying the step's error, so the commit is
    /// **not** discarded and the failure is **not** dropped. Before this, `super::step`'s `Failed`
    /// arm returned the committed report and threw `e` away, and `replay::dispatch_through` — whose
    /// only question is `committed()` — answered `Ok`. A `tool` step naming one of the 29 verbs
    /// ADR-0135 refuses by name therefore produced a run that exited **0** having executed nothing
    /// after the dispatch: the invisible non-event ADR-0135 exists to make impossible.
    ///
    /// `pub(super)` and consuming, on [`ActivityReport::refused`]'s precedent: a report is still
    /// produced by the loop and never minted beside it. Every count in the returned value is the
    /// committed run's own reading, unchanged.
    #[must_use]
    pub(super) fn with_step_failure(mut self, error: StepError) -> Self {
        self.step_error = Some(error);
        self
    }

    /// **ADR-0145: this activation's walk reached a `gate` step and OPENED it.**
    ///
    /// The row is appended by `super::step`'s walk, not by this module, so `appended` is **not**
    /// incremented — that counter's contract is *"events THIS MODULE appended"* and widening it
    /// silently would make an existing reading false. The gate row is in the journal delta
    /// `replay::dispatch_through` reads back, which is where every other boundary-written row is
    /// counted from.
    #[must_use]
    pub(super) fn with_gate(mut self, token: rk_types::digest::Digest) -> Self {
        self.gate = Some(token);
        self
    }

    /// **ADR-0145.** A walk that reached a `gate` step **before** any dispatch.
    ///
    /// 13 of the shipped pack's activities carry gates and no `agent` step at all, and 2 more put a
    /// gate before their first one. There is no dispatch report to attach the token to, and
    /// [`ActivityReport::refused`] would be wrong: a gate is not a failure. Every count in this
    /// report is `0` and that is a reading — nothing was dispatched and nothing committed, which is
    /// exactly what design §4.5 says a gate does.
    pub(super) fn gated(now: JournaledNow, token: rk_types::digest::Digest) -> Self {
        let mut report = Self::opened(now);
        report.gate = Some(token);
        report
    }

    const fn opened(now: JournaledNow) -> Self {
        Self {
            attempts: Vec::new(),
            dispatches: 0,
            decisions: Vec::new(),
            outcome: None,
            commit: None,
            mark: None,
            appended: 0,
            schema_failures: 0,
            now,
            step_error: None,
            gate: None,
            served: 0,
            completed: false,
            memo_contexts: Vec::new(),
            charges: Vec::new(),
            breaker: None,
        }
    }

    /// **R1.** The report a step walk accumulates into: opened, every count `0`.
    pub(super) const fn walk(now: JournaledNow) -> Self {
        Self::opened(now)
    }

    /// **R1.** Fold one step activation's dispatch report into the walk's.
    ///
    /// Counts add; the dispatch's own `outcome` and `mark` are the latest; and a COMMIT is never
    /// overwritten by a later dispatch that did not commit (ADR-0143 decision 1, at step grain):
    /// `commit` moves only when the absorbed report carries one.
    pub(super) fn absorb(&mut self, next: Self) {
        self.attempts.extend(next.attempts);
        self.dispatches = self.dispatches.saturating_add(next.dispatches);
        self.decisions.extend(next.decisions);
        self.outcome = next.outcome;
        if next.commit.is_some() {
            self.commit = next.commit;
        }
        self.mark = next.mark;
        if next.breaker.is_some() {
            self.breaker = next.breaker;
        }
        self.appended = self.appended.saturating_add(next.appended);
        self.schema_failures = self.schema_failures.saturating_add(next.schema_failures);
    }

    /// **R1.** One step activation served from the journal.
    pub(super) const fn note_served(&mut self) {
        self.served = self.served.saturating_add(1);
    }

    /// **R1.** One agent step's memo facts, recorded as it dispatches.
    pub(super) fn note_memo(&mut self, context: crate::workflow::memo::StepContext) {
        self.memo_contexts.push(context);
    }

    /// **R3 (m14).** Charge `n` attempts to `pass` (an iteration path with no `for_each` frame).
    pub(super) fn note_charge(&mut self, pass: IterationPath, n: u32) {
        if let Some((_, spent)) = self.charges.iter_mut().find(|(p, _)| *p == pass) {
            *spent = spent.saturating_add(n);
        } else {
            self.charges.push((pass, n));
        }
    }

    /// **R3 (m14).** The attempts this walk charged, one [`crate::workflow::budget::Charge`] per
    /// pass, keyed by the pass's iteration path; the draw-down `budget::fold` accumulates.
    #[must_use]
    pub fn pass_charges(&self) -> Vec<(IterationPath, crate::workflow::budget::Charge)> {
        self.charges
            .iter()
            .map(|(pass, n)| {
                (
                    pass.clone(),
                    crate::workflow::budget::Charge::Attempts(
                        crate::workflow::budget::Attempts::new(*n),
                    ),
                )
            })
            .collect()
    }

    /// **R1.** The completion row is journalled (or was, by a concurrent walk).
    #[must_use]
    pub(super) fn with_completion(mut self, outcome: CommitOutcome) -> Self {
        if outcome.is_committed() {
            self.appended = self.appended.saturating_add(1);
        }
        self.completed = true;
        self.commit = Some(outcome);
        self
    }

    /// **R1.** Record a step's own committed row appended by the walk (a `side_effecting` tool
    /// step), so `committed()` answers for it.
    pub(super) fn note_step_commit(&mut self, outcome: CommitOutcome) {
        if outcome.is_committed() {
            self.appended = self.appended.saturating_add(1);
        }
        self.commit = Some(outcome);
    }

    /// **R1.** Whether any step of this activation is committed — by this walk or an earlier run.
    #[must_use]
    pub(super) fn holds_a_commit(&self) -> bool {
        self.served > 0 || self.committed()
    }

    /// *(dated 2026-09-27, `s8d` build, package R4FIX-8d; refuting round 4's spec-conformance
    /// major.)* **m14.** A fresh report (a refusal or a gate before any commit) carrying the pass
    /// charges `walk` noted: a step dispatched before the halt stays charged (§8d.3; ADR-0175
    /// decision 7). Nothing else of `walk` is taken, so every other count stays the fresh reading.
    #[must_use]
    pub(super) fn with_charges_of(mut self, walk: Self) -> Self {
        self.charges = walk.charges;
        self
    }

    /// **R1.** The walk's own refusal, before any step committed.
    #[must_use]
    pub(super) fn refusing(mut self, error: StepError) -> Self {
        self.outcome = Some(Err(error));
        self
    }

    /// Every attempt number this run dispatched under, in order. **AC-011's first instrument reads
    /// this**, and it is a `Vec` rather than a count precisely so *"strictly increasing"* is
    /// assertable and not merely *"three of them"*.
    #[must_use]
    pub fn attempts(&self) -> &[u32] {
        &self.attempts
    }

    /// How many times the `Harness` port was reached.
    #[must_use]
    pub const fn dispatches(&self) -> u32 {
        self.dispatches
    }

    /// Every retry decision, in order.
    #[must_use]
    pub fn decisions(&self) -> &[RetryDecision] {
        &self.decisions
    }

    /// The step's own final outcome, and `None` when the activation was already committed.
    #[must_use]
    pub const fn outcome(&self) -> Option<&Result<Typed, StepError>> {
        self.outcome.as_ref()
    }

    /// The commit verdict, and `None` when the run never reached one.
    #[must_use]
    pub const fn commit(&self) -> Option<&CommitOutcome> {
        self.commit.as_ref()
    }

    /// Whether this run appended the activation's `activity_committed`.
    #[must_use]
    pub fn committed(&self) -> bool {
        self.commit
            .as_ref()
            .is_some_and(CommitOutcome::is_committed)
    }

    /// **AC-012.** Whether this run's result arrived late and was discarded.
    #[must_use]
    pub const fn discarded_late(&self) -> bool {
        matches!(self.commit, Some(CommitOutcome::DiscardedLate { .. }))
    }

    /// **AC-013's re-entry path**: the activation was already committed, so nothing was dispatched
    /// and nothing was appended.
    ///
    /// *(2026-09-26, `s8d` build, package P6; R4 = ADR-0176 decision 5, proposed.)* And the report
    /// carries no outcome. A dispatch the breaker refused before the port also reaches no port and
    /// commits nothing, and it is a refusal, not a re-entry: without this clause the walk's agent arm
    /// would serve it as a committed step.
    #[must_use]
    pub const fn skipped(&self) -> bool {
        self.dispatches == 0 && self.commit.is_none() && self.outcome.is_none()
    }

    /// **AC-030's durable mark**, when the run ended on one.
    #[must_use]
    pub const fn mark(&self) -> Option<&RetryAtMark> {
        self.mark.as_ref()
    }

    /// **R4 (ADR-0176, proposed; AC-033).** The breaker's refusal of this activation's dispatch:
    /// `Some` when the executor folded the store's family-scoped rows, found the family's mark open
    /// and unexpired, and never reached the port.
    #[must_use]
    pub const fn breaker_refusal(&self) -> Option<&BreakerDecision> {
        self.breaker.as_ref()
    }

    /// **ADR-0144.** The failure of a step that ran AFTER this activation's dispatch committed.
    ///
    /// `None` on every report this module's own loop produces — the loop walks no steps. It is
    /// `Some` only for a report `super::step`'s walk handed back after a later step refused, and it
    /// is the ONLY channel by which such a refusal reaches a caller.
    ///
    /// **It is a second field and not a reuse of [`ActivityReport::outcome`]**, because `outcome`
    /// is the DISPATCH's own reading and is already spoken for in both shapes a committed report
    /// takes: `Some(Ok(typed))` for a fresh commit (set beside `commit` on the success path) and
    /// `None` for AC-013's re-entry. Writing a later step's error into either would destroy the
    /// dispatch's reading to report the walk's — which is ADR-0143's defect one field over.
    #[must_use]
    pub const fn step_error(&self) -> Option<&StepError> {
        self.step_error.as_ref()
    }

    /// **ADR-0145.** The token of the gate this activation's walk opened, when it reached one.
    ///
    /// `Some` means the run **exits 0 and does not commit this activation** (design §4.5): the
    /// kernel wrote the mark, the fold will answer `awaiting_gate`, and `next_action` will return
    /// `AwaitGate` rather than the same dispatch. A caller that checked only `committed()` would
    /// read this as a failed dispatch, which is what `replay::dispatch_through` did before this
    /// record and why all 80 of the pack's `gate` steps were silent.
    #[must_use]
    pub const fn gate_opened(&self) -> Option<&rk_types::digest::Digest> {
        self.gate.as_ref()
    }

    /// How many events **this module** appended. The adapter's own journaling is not counted here
    /// and the test reads the store for that.
    #[must_use]
    pub const fn appended(&self) -> usize {
        self.appended
    }

    /// **R1.** How many step activations this walk served from the journal without dispatching.
    #[must_use]
    pub const fn served(&self) -> u32 {
        self.served
    }

    /// **R1.** Whether this walk journalled the activation's completion row.
    #[must_use]
    pub const fn completed(&self) -> bool {
        self.completed
    }

    /// **R1.** The per-step memo facts of every agent step this walk dispatched, in walk order.
    #[must_use]
    pub fn memo_contexts(&self) -> &[crate::workflow::memo::StepContext] {
        &self.memo_contexts
    }

    /// How many returned values failed the boundary's schema check. **AC-031's `attempts`.**
    #[must_use]
    pub const fn schema_failures(&self) -> u32 {
        self.schema_failures
    }

    /// The journaled instant the run was handed, returned **unchanged**.
    ///
    /// **AC-030's *"zero elapsed sleep on the injected clock"*, as far as a type can state it.** The
    /// equality a caller asserts against its own argument is a restatement of the fact that this
    /// crate has no way to move it; the instrument that would catch a real sleep is
    /// [`boundary_prose_extraction_rule`]'s sibling — the source lint over this module in
    /// `ac030_rate_limited_writes_a_mark_and_never_sleeps` — and the two are reported as two.
    #[must_use]
    pub const fn now(&self) -> JournaledNow {
        self.now
    }
}

// ---- the loop -----------------------------------------------------------------------------------

impl Runtime {
    /// One dispatch: validate the key, refuse a committed activation, append the `dispatch` row,
    /// and reach the `Harness` port.
    ///
    /// **The key is validated first and the port is reached last.** That order is AC-010's
    /// *"refused before any process is spawned"*, and it is why the malformed-key test can assert a
    /// dispatch count of **0** rather than merely an `Err`.
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Key`] for a malformed key; [`RuntimeError::Store`] from the port;
    /// [`RuntimeError::Fsm`] if the recorded journal does not fold.
    pub fn dispatch_once(
        &self,
        item: &str,
        activity_id_at_version: &str,
        step_id: &str,
        attempt: u32,
        spec: &rk_types::activity::ActivitySpec,
    ) -> Result<DispatchAttempt, RuntimeError> {
        self.dispatch_once_at(item, activity_id_at_version, step_id, attempt, spec, None)
    }

    /// [`Self::dispatch_once`], at the STEP grain when `at` is `Some` (R1): the pre-flight asks the
    /// commit key, so a step activation already committed is not dispatched again.
    ///
    /// # Errors
    ///
    /// As [`Self::dispatch_once`].
    pub fn dispatch_once_at(
        &self,
        item: &str,
        activity_id_at_version: &str,
        step_id: &str,
        attempt: u32,
        spec: &rk_types::activity::ActivitySpec,
        at: Option<StepAt<'_>>,
    ) -> Result<DispatchAttempt, RuntimeError> {
        // AC-010, FIRST: the refusal happens before the store is touched and long before the port
        // is reached, which is what makes "before any process is spawned" a dispatch count of 0.
        let key = checked_key(item, activity_id_at_version, attempt)?;

        // AC-012's and AC-013's shared pre-flight, read through the FSM's own fold so this module
        // and `next_action`'s step 3 cannot disagree about what "committed" means.
        let state = self.state(item)?;
        let already = at.map_or_else(
            || state.is_committed(item, activity_id_at_version),
            |at| state.is_step_committed(&at.commit_key(item, activity_id_at_version)),
        );
        if already {
            return Ok(DispatchAttempt {
                key,
                dispatched: false,
                result: None,
            });
        }

        self.append(dispatch_event(&key, step_id))?;
        let outcome = {
            // THE CALL BOUNDARY (ADR-0115 clause 4). The guard is taken here and dropped at the end
            // of this block; it is never held across a call to another port.
            let mut guard = self.store_guard();
            self.ports().harness.dispatch(spec, &mut *guard)
        };
        Ok(DispatchAttempt {
            key,
            dispatched: true,
            result: Some(outcome.result),
        })
    }

    /// **AC-012.** Commit a result, or discard it as a late duplicate.
    ///
    /// The journal re-read and the append happen under **one** lock acquisition; see the module
    /// header for why two would leave a window.
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Store`] from the port; [`RuntimeError::Fsm`] if the journal does not fold.
    pub fn commit(
        &self,
        key: &IdempotencyKey,
        typed: &Typed,
        now: JournaledNow,
    ) -> Result<CommitOutcome, RuntimeError> {
        self.commit_at(key, None, typed, now)
    }

    /// [`Self::commit`], at the STEP grain when `at` is `Some` (R1): the late-duplicate discard
    /// asks the commit key, and the appended row carries `step_id` and `iteration_path`.
    ///
    /// # Errors
    ///
    /// As [`Self::commit`].
    #[expect(
        clippy::significant_drop_tightening,
        reason = "AC-012: the journal re-read and the append are ONE critical section, and taking                   the lint's advice to narrow the guard is exactly the defect this function                   exists to prevent — two acquisitions leave a window in which both attempts                   observe no commit and both append. `#[expect]` and not `#[allow]` (ADR-0102                   decision 3): if the lint stops firing, the span has narrowed and the build says so"
    )]
    pub fn commit_at(
        &self,
        key: &IdempotencyKey,
        at: Option<StepAt<'_>>,
        typed: &Typed,
        now: JournaledNow,
    ) -> Result<CommitOutcome, RuntimeError> {
        // ONE ACQUISITION for the read AND the append. Two would leave a window in which both
        // attempts see no commit and both append, and AC-012's THEN would hold only by luck.
        //
        // `clippy::significant_drop_tightening` asks for the guard to be dropped earlier, and
        // taking that advice is exactly the defect this function exists to prevent — which is why
        // the exception is `#[expect]` and not `#[allow]` (ADR-0102 decision 3): if the lint ever
        // stops firing here, the span has narrowed and the build says so.
        //
        // *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; ADR-0188 decision 2.)* At
        // the STEP grain with a boundary bound, the landing is inside the same section: the
        // boundary is taken first (the order `tier_one_act` takes them in), then the store.
        let mut boundary = self.step_boundary(at.is_some());
        let mut guard = self.store_guard();
        let events =
            guard
                .query(super::journal_query(&key.item))
                .map_err(|e| RuntimeError::Store {
                    op: "query",
                    detail: format!("{e:?}"),
                })?;
        let prior = at.map_or_else(
            || committed_attempt(&events, &key.item, &key.activity_id_at_version),
            |at| {
                committed_step_attempt(
                    &events,
                    &at.commit_key(&key.item, &key.activity_id_at_version),
                )
            },
        );
        if let Some(committed) = prior {
            // **AC-012.** The late result is discarded and NOTHING is appended, so *"the committed
            // value is unchanged"* holds by construction rather than by a comparison.
            return Ok(CommitOutcome::DiscardedLate {
                key: key.clone(),
                committed_attempt: committed,
            });
        }
        let row = at.map_or_else(
            || activity_committed_event(key, typed, self.run_id(), now),
            |at| step_committed_event(key, at, typed, self.run_id(), now),
        );
        let commit_key = at.map(|at| at.commit_key(&key.item, &key.activity_id_at_version));
        land_step_row(
            &mut *guard,
            boundary.as_deref_mut(),
            commit_key.as_ref(),
            row,
        )?;
        Ok(CommitOutcome::Committed { key: key.clone() })
    }

    /// **R1: a row the WALK commits, not a dispatch** — a `side_effecting` `tool` step's output
    /// (`Some((kind, value))`), or the activation's COMPLETION (`None`, the `emit` step's row).
    ///
    /// Under one lock acquisition, as [`Self::commit`]: a row already journalled under the same
    /// commit key (or, for the completion, an activation already complete) is not appended again
    /// and is answered `DiscardedLate`. The row takes the next attempt number of the activation, so
    /// `(item, activity_id@version, attempt)` stays unique across the log (AC-011) and
    /// `IdempotencyKey` keeps three parts (D21-43 N15).
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Key`] for a malformed activation id; [`RuntimeError::Store`] from the port.
    pub fn commit_walk_row(
        &self,
        item: &str,
        activity_id_at_version: &str,
        at: StepAt<'_>,
        output: Option<(&str, &serde_json::Value)>,
        now: JournaledNow,
    ) -> Result<CommitOutcome, RuntimeError> {
        self.commit_walk_row_emitting(item, activity_id_at_version, at, output, None, now)
    }

    /// **R2 decision 7: the activation's COMPLETION, carrying its `emit` binding** — each provided
    /// kind and the evidence refs, as `runtime::reference::bind_emit` resolved them. `None` is an
    /// `emit` that declares no binding (a fixture's), whose row is [`completion_event`]'s alone.
    ///
    /// # Errors
    ///
    /// As [`Self::commit_walk_row`].
    pub fn commit_completion(
        &self,
        item: &str,
        activity_id_at_version: &str,
        at: StepAt<'_>,
        emitted: Option<&super::reference::Emitted>,
        now: JournaledNow,
    ) -> Result<CommitOutcome, RuntimeError> {
        self.commit_walk_row_emitting(item, activity_id_at_version, at, None, emitted, now)
    }

    /// **D21-76: journal one CONTROL DECISION of the walk** — a `check` verdict, a `repeat`'s
    /// `until` verdict (another pass, or exhaustion and escalation) or a `gate` step's `when` —
    /// under the commit key of the frame that took it, as [`decision_event`]'s row.
    ///
    /// *(dated 2026-09-27, `s8d` build, package D2176-8d; ADR-0174 decision 7 and ADR-0175, both
    /// `proposed`, amended.)* Under one lock acquisition, as [`Self::commit_walk_row`]: a decision
    /// of the same `kind` already journaled under the same commit key is not appended again (a
    /// re-entry REPLAYS it and never re-decides it). The row takes the activation's next attempt,
    /// so `(item, activity_id@version, attempt)` stays unique across the log (AC-011).
    ///
    /// *(dated 2026-09-27, `s8d` build, package D2177-8d; D21-77 rulings 1 and 2a.)* The walk calls
    /// this only for a decision whose route continues the walk: a `false` routed to a refusal ~~and a
    /// `null` are~~ is not journaled. *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92;
    /// ADR-0187, proposed.)* A `null` `until` or gate `when` follows the same rule: journaled (as
    /// `"holds": null`) when its route continues the walk, and not when it is routed to a refusal. The attempt the row takes draws nothing from AC-029's retry cap:
    /// [`Self::run_activity_at`] checks the cap against the activation's `dispatch` rows only.
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Key`] for a malformed activation id; [`RuntimeError::Store`] from the port.
    pub fn commit_decision(
        &self,
        item: &str,
        activity_id_at_version: &str,
        at: StepAt<'_>,
        decision: Decision,
        now: JournaledNow,
    ) -> Result<(), RuntimeError> {
        let mut guard = self.store_guard();
        let events = guard
            .query(super::journal_query(item))
            .map_err(|e| RuntimeError::Store {
                op: "query",
                detail: format!("{e:?}"),
            })?;
        let commit_key = at.commit_key(item, activity_id_at_version);
        if journaled_decision(&events, &commit_key, decision.kind).is_some() {
            return Ok(());
        }
        let attempt = next_attempt_of(&events, item, activity_id_at_version);
        let key = checked_key(item, activity_id_at_version, attempt)?;
        guard
            .append(decision_event(&key, at, decision, self.run_id(), now))
            .map_err(|e| RuntimeError::Store {
                op: "append",
                detail: format!("{e:?}"),
            })
    }

    #[expect(
        clippy::significant_drop_tightening,
        reason = "AC-012, at step grain: the journal re-read and the append are ONE critical \
                  section, for the reason `commit`'s own exception gives"
    )]
    fn commit_walk_row_emitting(
        &self,
        item: &str,
        activity_id_at_version: &str,
        at: StepAt<'_>,
        output: Option<(&str, &serde_json::Value)>,
        emitted: Option<&super::reference::Emitted>,
        now: JournaledNow,
    ) -> Result<CommitOutcome, RuntimeError> {
        // *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; ADR-0188 decision 2.)* A
        // side-effecting tool step's commit and an activation's `emit` completion are step-grain
        // acts too: boundary first, then the store, one section.
        let mut boundary = self.step_boundary(true);
        let mut guard = self.store_guard();
        let events = guard
            .query(super::journal_query(item))
            .map_err(|e| RuntimeError::Store {
                op: "query",
                detail: format!("{e:?}"),
            })?;
        let commit_key = at.commit_key(item, activity_id_at_version);
        let prior = if output.is_some() {
            committed_step_attempt(&events, &commit_key)
        } else {
            completion_attempt(&events, item, activity_id_at_version)
        };
        let attempt = next_attempt_of(&events, item, activity_id_at_version);
        let key = checked_key(item, activity_id_at_version, attempt)?;
        if let Some(committed) = prior {
            return Ok(CommitOutcome::DiscardedLate {
                key,
                committed_attempt: committed,
            });
        }
        let row = match output {
            Some((kind, value)) => step_committed_event(
                &key,
                at,
                &Typed::new(rk_types::typed::SchemaId::new(kind), value.clone()),
                self.run_id(),
                now,
            ),
            None => {
                let mut row = completion_event(&key, at, self.run_id(), now);
                if let (Some(emitted), Some(map)) = (emitted, row.payload.as_object_mut()) {
                    map.insert(
                        "artifacts".to_owned(),
                        serde_json::Value::Object(
                            emitted
                                .artifacts
                                .iter()
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect(),
                        ),
                    );
                    map.insert(
                        "evidence".to_owned(),
                        serde_json::Value::Array(emitted.evidence.clone()),
                    );
                }
                row
            }
        };
        land_step_row(&mut *guard, boundary.as_deref_mut(), Some(&commit_key), row)?;
        Ok(CommitOutcome::Committed { key })
    }

    /// The tier-1 boundary, locked, for a step-grain act: when one is bound and the commit is at
    /// the step grain. *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
    fn step_boundary(
        &self,
        step_grain: bool,
    ) -> Option<std::sync::MutexGuard<'_, super::tier_one::Boundary>> {
        if !step_grain {
            return None;
        }
        self.boundary
            .as_ref()
            .map(|b| b.lock().expect(super::LOCK_HALTS))
    }

    /// **The retry loop.** One activation, from its next attempt number to its terminal outcome.
    ///
    /// # Errors
    ///
    /// [`RuntimeError`] from the key validator, the store or the fold. A step's own failure is a
    /// **value** in the report (AC-032), never an error of this function.
    pub fn run_activity(
        &self,
        item: &str,
        activity_id_at_version: &str,
        step_id: &str,
        spec: &rk_types::activity::ActivitySpec,
        cap: AttemptCap,
        now: JournaledNow,
    ) -> Result<ActivityReport, RuntimeError> {
        self.run_activity_at(
            (item, activity_id_at_version, step_id),
            spec,
            None,
            cap,
            now,
        )
    }

    /// **The retry loop at the STEP grain (R1)** when `at` is `Some`: the re-entry question, the
    /// pre-flight and the late-duplicate discard all ask the commit key, and the committed row
    /// carries `step_id` and `iteration_path`. `None` is [`Self::run_activity`], unchanged.
    ///
    /// `activation` is `(item, activity_id@version, DAG node id)`.
    ///
    /// # Errors
    ///
    /// As [`Self::run_activity`].
    pub fn run_activity_at(
        &self,
        activation: (&str, &str, &str),
        spec: &rk_types::activity::ActivitySpec,
        at: Option<StepAt<'_>>,
        cap: AttemptCap,
        now: JournaledNow,
    ) -> Result<ActivityReport, RuntimeError> {
        let (item, activity_id_at_version, step_id) = activation;
        let mut report = ActivityReport::opened(now);
        let state = self.state(item)?;
        // **AC-013's re-entry path.** A committed activation is not re-executed, and the predicate
        // is the FSM's own rather than a second one written here. *(R1: at step grain, a committed
        // STEP activation is not re-executed; the question is the commit key's.)*
        let already = at.map_or_else(
            || state.is_committed(item, activity_id_at_version),
            |at| state.is_step_committed(&at.commit_key(item, activity_id_at_version)),
        );
        if already {
            return Ok(report);
        }
        // **AC-279's row (ADR-0123's `gate_resumed`).** The first dispatch after a gate on this
        // item was answered IS the resume, and the kernel's count of prompt-prefix rewrites for it
        // is one — this dispatch. One row per resumed gate: a token the fold already saw a
        // `gate_resumed` for is not written again, so a later dispatch in the same run is not a
        // second resume. The adapter's metric fields (`cache_creation_input_tokens`,
        // `total_cost_usd`; AC-267, SM-073) are ABSENT here, never zero — Step 7c fills them.
        for answered in state.answered_gates() {
            if !state.is_resumed(answered.token()) {
                self.append(crate::workflow::gate::gate_resumed_event(
                    answered.token(),
                    1,
                ))?;
                report.appended += 1;
            }
        }
        // **AC-011.** The first attempt number comes from the fold, so a re-entry after two
        // recorded attempts resumes at three rather than restarting at one and reusing a number.
        let mut attempt = state.next_attempt(item, activity_id_at_version);
        // 🛑 **D21-77 ruling 2a: THE RETRY CAP COUNTS DISPATCH ATTEMPTS ONLY.** *(dated 2026-09-27,
        // `s8d` build, package D2177-8d; ADR-0174 decisions 4 and 7, `proposed`, amended.)* `attempt`
        // above stays the activation's next number (AC-011), which every row the walk appends
        // takes: a decision row, a side-effecting tool step's commit, the completion. None of those
        // is a dispatch, so none may spend AC-029's `N`. `retry::decide` is handed the key at the
        // activation's DISPATCH ordinal — the `dispatch` rows the journal holds for it before this
        // loop, plus this loop's own — so its cap check and its backoff ceiling count dispatches.
        let prior_dispatches = dispatches_of(&self.journal(item)?, item, activity_id_at_version);

        loop {
            // **R4 (ADR-0176 decision 4; AC-033..AC-035).** Before an agent step's dispatch, the
            // store's family-scoped rows are folded with the pack's `K` and the family's mark is
            // decided against the journalled instant and the pack's expiry — from the STORE, never
            // from a value this process held. An open mark refuses the dispatch before the port; a
            // refusal is not a dispatch, so no error row is written for it.
            if let Some(family) = at.and_then(|at| at.family)
                && let Some((error, decision)) = self.breaker_refusal(item, family, now)?
            {
                report.outcome = Some(Err(error));
                report.breaker = decision;
                return Ok(report);
            }
            let dispatched =
                self.dispatch_once_at(item, activity_id_at_version, step_id, attempt, spec, at)?;
            let Some(result) = dispatched.result else {
                // Another attempt committed between the fold above and the pre-flight inside
                // `dispatch_once`. Nothing was dispatched and nothing was appended.
                return Ok(report);
            };
            report.appended += 1; // the `dispatch` cost row
            report.attempts.push(attempt);
            report.dispatches += 1;
            let key = dispatched.key;

            let error = match result {
                Ok(typed) => {
                    // **AC-031's boundary check, and the whole of it**: the value came back
                    // attributed to a schema and the spec demanded one. No text is parsed and no
                    // structure is recovered from prose — [`boundary_prose_extraction_rule`] is
                    // what says so about this file rather than about this line.
                    if typed.schema == spec.schema {
                        let commit = self.commit_at(&key, at, &typed, now)?;
                        if commit.is_committed() {
                            report.appended += 1;
                        }
                        report.commit = Some(commit);
                        report.outcome = Some(Ok(typed));
                        return Ok(report);
                    }
                    report.schema_failures += 1;
                    StepError::Schema {
                        attempts: report.schema_failures,
                        last_error: format!(
                            "the returned value validated against `{}` and the activity spec demands `{}`",
                            typed.schema.as_str(),
                            spec.schema.as_str()
                        ),
                    }
                }
                Err(error) => error,
            };

            // **R4 (ADR-0176 decisions 1-3; D21-42 20).** THE KERNEL WRITES THE ERROR ROW: exactly
            // one per failed dispatch, through `StepError::event()`, whatever the harness. A
            // `RateLimited` failure's row is AC-030's `retry_at` mark below, never a second row.
            let family = at.and_then(|at| at.family);
            if !matches!(error.event(), ErrorEvent::RateLimited) {
                self.append(error_row(&error, family, now))?;
                report.appended += 1;
            }

            let dispatch_key = IdempotencyKey {
                attempt: prior_dispatches.saturating_add(report.dispatches),
                ..key.clone()
            };
            let decision = decide(&error, &dispatch_key, cap);
            report.decisions.push(decision.clone());

            // **AC-030.** The mark, and then the run ends: no backoff (ADR-0095 forbids it on this
            // class), no sleep, no further dispatch. *(R4: the mark IS the dispatch's one
            // `rate_limited` row, and carries the members every error row carries.)*
            if let RetryDecision::WaitForReset { resets_at } = decision {
                let mark = RetryAtMark::carried(resets_at);
                let mut row = mark.to_event();
                stamp(&mut row, family, now);
                self.append(row)?;
                report.appended += 1;
                report.mark = Some(mark);
                report.outcome = Some(Err(error));
                return Ok(report);
            }
            if decision.authorises_redispatch() {
                // **AC-011.** `attempt + 1`, and never a number already used.
                attempt = attempt.saturating_add(1);
                continue;
            }
            report.outcome = Some(Err(error));
            return Ok(report);
        }
    }
}

impl Runtime {
    /// **R4 (ADR-0176 decision 4).** Whether the breaker refuses `family`'s next dispatch.
    ///
    /// `Ok(None)` proceeds. `Ok(Some((error, Some(decision))))` is AC-033's refusal: the store's
    /// family-scoped rows, folded by [`BreakerState::from_events`] with the pack's `K`
    /// ([`crate::select::Pack::require_breaker_threshold`]), hold an open mark for `family` that
    /// [`BreakerState::decide`] does not find expired at `now` under the pack's expiry.
    /// `Ok(Some((error, None)))` refuses by name without a decision: the pack declares no `K` or no
    /// expiry, or a family-scoped row does not fold. Nothing is defaulted.
    ///
    /// The fold reads the store's WHOLE journal: every `EventStore` in this tree ignores the query's
    /// selector (`rk-adapter-sqlite`'s `query` reads every `t2_event` row), and AC-034's partition is
    /// the family, not the item.
    fn breaker_refusal(
        &self,
        item: &str,
        family: &FamilyKey,
        now: JournaledNow,
    ) -> Result<Option<(StepError, Option<BreakerDecision>)>, RuntimeError> {
        let refuse = |why: String| {
            Some((
                StepError::Unsupported {
                    capability: format!(
                        "the breaker cannot decide the dispatch for family `{family}`: {why} \
                         (AC-033; R4 = ADR-0176 decision 4)"
                    ),
                },
                None,
            ))
        };
        let Some(binding) = self.breaker() else {
            return Ok(None);
        };
        let k = match binding.pack().require_breaker_threshold() {
            Ok(k) => k,
            Err(e) => return Ok(refuse(e.to_string())),
        };
        let expiry = match binding.pack().require_breaker_expiry() {
            Ok(expiry) => expiry,
            Err(e) => return Ok(refuse(e.to_string())),
        };
        let events = self.journal(item)?;
        let state = match BreakerState::from_events(&events, k) {
            Ok(state) => state,
            Err(e) => return Ok(refuse(e.to_string())),
        };
        let decision = state.decide(family, now, expiry);
        if decision.refuses() {
            return Ok(Some((
                StepError::Unsupported {
                    capability: format!("{decision} (R4 = ADR-0176 decision 4)"),
                },
                Some(decision),
            )));
        }
        Ok(None)
    }
}

/// **R4 (ADR-0176 decisions 1-3): the error row the KERNEL writes** — `StepError::event()`'s arm.
///
/// It carries the payload members the harness bridge wrote (`class` and `status` on `step_failed`,
/// `class` and `capability` on `step_refused`), plus `at` (the journalled instant) and, when the
/// dispatch's family is bound, `family` — the two members [`BreakerState::from_events`] reads.
#[must_use]
pub fn error_row(error: &StepError, family: Option<&FamilyKey>, now: JournaledNow) -> Event {
    let mut row = match error.event() {
        ErrorEvent::StepFailed { class, status } => Event::new(
            EventKind::StepFailed,
            serde_json::json!({ "class": class, "status": status }),
        ),
        ErrorEvent::StepCancelled => Event::new(EventKind::StepCancelled, serde_json::json!({})),
        ErrorEvent::RateLimited => Event::new(EventKind::RateLimited, serde_json::json!({})),
        ErrorEvent::StepRefused { class, capability } => Event::new(
            EventKind::StepRefused,
            serde_json::json!({ "class": class, "capability": capability }),
        ),
    };
    stamp(&mut row, family, now);
    row
}

/// The two members R4 adds to a row the breaker may fold: `at`, and `family` when bound.
fn stamp(row: &mut Event, family: Option<&FamilyKey>, now: JournaledNow) {
    if let Some(map) = row.payload.as_object_mut() {
        map.insert(
            "at".to_owned(),
            serde_json::json!(now.timestamp().unix_seconds()),
        );
        if let Some(family) = family {
            map.insert("family".to_owned(), serde_json::json!(family.as_str()));
        }
    }
}

/// The `dispatch` cost row, as this boundary writes it.
///
/// `{item, activity_id_at_version, step_id, attempt}` — the three parts `crate::workflow::fsm`'s
/// `activation_key` requires, plus the step id `crate::workflow::memo::recorded_residue` already
/// carries. **A READING**: design §5 gives `dispatch` as *"the dispatch cost row"* in prose only and
/// `crates/rk-types/src/event_kind.rs`:59 records the name itself as owed to design §5. This is the
/// ninth consumer of the typed-payload-per-kind record `crates/rk-types/src/event.rs`:46-58 says is
/// **OWED**.
#[must_use]
pub fn dispatch_event(key: &IdempotencyKey, step_id: &str) -> Event {
    Event::new(
        EventKind::Dispatch,
        serde_json::json!({
            "activity_id_at_version": key.activity_id_at_version,
            "attempt": key.attempt,
            "item": key.item,
            "step_id": step_id,
        }),
    )
}

/// The activation record — **ADR-0022's, per step activation and not per activity** (ADR-0097
/// decision 2).
///
/// `result` is the dispatch's `Typed.value` **verbatim** (AC-009) and `at` is the journaled instant
/// the caller supplied (AC-001: this crate has no clock). `run_id` is the value the composition root
/// handed [`Runtime::new`], and it is what makes `super::ledger::ExecutionStamp` constructible.
#[must_use]
pub fn activity_committed_event(
    key: &IdempotencyKey,
    typed: &Typed,
    run_id: &str,
    now: JournaledNow,
) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": key.activity_id_at_version,
            "at": now.timestamp().unix_seconds(),
            "attempt": key.attempt,
            "item": key.item,
            "result": typed.value,
            "run_id": run_id,
            "schema": typed.schema.as_str(),
        }),
    )
}

/// **N10: a committed step's row, landed first** (the landing record ADR-0188 decision 2,
/// proposed). *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
///
/// With a boundary bound and a step-grain key, the row rides ONE step-grain act of
/// `tier_one::commit_act`'s body: the run-owned tree is landed first (a content checkpoint commit
/// when the step changed it), and the row is appended anchored to that commit, or unanchored when
/// the step changed nothing. Unbound (the machinery suites and the golden-log recorder) or at the
/// activation grain, the row is appended as it was before `s8f`. The caller holds the store's
/// guard across this call, so the journal re-read, the landing and the append are one section.
fn land_step_row(
    store: &mut dyn rk_ports::event_store::EventStore,
    boundary: Option<&mut super::tier_one::Boundary>,
    commit_key: Option<&CommitKey>,
    row: Event,
) -> Result<(), RuntimeError> {
    match (boundary, commit_key) {
        (Some(boundary), Some(key)) => {
            let checkpoint = super::tier_one::StepCheckpoint::of(
                &key.item,
                &key.activity_id_at_version,
                &key.step_id,
                &key.iteration_path.to_json(),
                &row,
            )?;
            boundary.act_locked(store, super::tier_one::Act::step(checkpoint, vec![row]))?;
            Ok(())
        }
        _ => store.append(row).map_err(|e| RuntimeError::Store {
            op: "append",
            detail: format!("{e:?}"),
        }),
    }
}

/// **R1: one step activation's commit row** — [`activity_committed_event`]'s row plus the two
/// payload members the commit key needs, `step_id` and `iteration_path`. No `EventKind` is minted.
#[must_use]
pub fn step_committed_event(
    key: &IdempotencyKey,
    at: StepAt<'_>,
    typed: &Typed,
    run_id: &str,
    now: JournaledNow,
) -> Event {
    let mut row = activity_committed_event(key, typed, run_id, now);
    if let Some(map) = row.payload.as_object_mut() {
        map.insert("step_id".to_owned(), serde_json::json!(at.step_id));
        map.insert("iteration_path".to_owned(), at.iteration_path.to_json());
        // **R4 (ADR-0176 decision 4).** The success the breaker's reset arm reads: an agent step's
        // commit carries its family when one is bound, so K counts CONSECUTIVE failures.
        if let Some(family) = at.family {
            map.insert("family".to_owned(), serde_json::json!(family.as_str()));
        }
    }
    row
}

/// **R1: the activation's COMPLETION row** — one more `activity_committed`.
///
/// Its `step_id` is the activation's `emit` step's and it carries `completion: true`.
/// `fsm::next_action` moves past a node only on it. It carries no `result` and no `schema`: binding
/// the `emit` step's outputs is the resolver's (R2 decision 7), and this row says only that the
/// walk reached it. *(2026-09-26, `s8d` build, package P4.)* When the `emit` declares a binding,
/// [`Runtime::commit_completion`] adds two members to this row, `artifacts` (each provided kind's
/// bound value) and `evidence` (the evidence refs); `runtime::reference::read_artifact_input` reads
/// the former.
#[must_use]
pub fn completion_event(
    key: &IdempotencyKey,
    at: StepAt<'_>,
    run_id: &str,
    now: JournaledNow,
) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": key.activity_id_at_version,
            "at": now.timestamp().unix_seconds(),
            "attempt": key.attempt,
            "completion": true,
            "item": key.item,
            "iteration_path": at.iteration_path.to_json(),
            "run_id": run_id,
            "step_id": at.step_id,
        }),
    )
}

/// **D21-76: the kind of a journaled control decision.**
///
/// Closed: a `check`'s verdict, a `repeat`'s `until` verdict, and the `when` of a `gate` step
/// (whether the walk asks the human; for an escalate gate, whether it escalates).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionKind {
    /// A `check` step's `expr` held (`true`) or did not (`false`, routed through its `on_false`).
    Check,
    /// A `repeat`'s `until`, evaluated after a pass's body: `true` ends the repeat; `false` takes
    /// another pass, or at `max` the step's `on_exhausted`.
    Until,
    /// A `gate` step's `when`: `true` reaches the gate, `false` walks past it.
    When,
}

impl DecisionKind {
    /// The token the row carries.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::Until => "until",
            Self::When => "when",
        }
    }
}

/// **D21-76: one control decision** — its kind and whether its predicate held.
///
/// *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)* `holds` is
/// the predicate's three-valued outcome, not a `bool`: a `null` `until` or gate `when` whose route
/// continues the walk is journaled as `null` (the row's `"holds": null`), so its replay re-takes
/// the route the `null` took and the journal never records a `false` the predicate did not
/// answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision {
    /// What decided.
    pub kind: DecisionKind,
    /// Whether it held: `true`, `false`, or `null` (D21-92).
    pub holds: crate::predicate::Tri,
}

/// The row's `holds` member for a decision's outcome: a JSON `true`, `false` or `null`.
const fn holds_json(holds: crate::predicate::Tri) -> serde_json::Value {
    match holds {
        crate::predicate::Tri::True => serde_json::Value::Bool(true),
        crate::predicate::Tri::False => serde_json::Value::Bool(false),
        crate::predicate::Tri::Null => serde_json::Value::Null,
    }
}

/// **D21-76: a control decision's row** — one more `activity_committed`.
///
/// As R1's completion row is (ADR-0174 decision 5's precedent: no `EventKind` is minted), it is
/// keyed by the commit key of the frame that took it (`step_id`, `iteration_path`) and carrying a `decision` member,
/// `{"kind": "check" | "until" | "when", "holds": <bool>}` — *(dated 2026-09-28, `s8e` build,
/// package S8E-RULINGS; D21-92)* or `"holds": null` for a `null` `until` or gate `when` whose
/// route continued the walk. It carries no `result`, no `schema` and
/// no `family`: it is not a step activation's commit, so [`commit_key_of`] answers `None` for it,
/// the fold does not count it as a commit (`fsm::fold`), and the breaker never reads it.
#[must_use]
pub fn decision_event(
    key: &IdempotencyKey,
    at: StepAt<'_>,
    decision: Decision,
    run_id: &str,
    now: JournaledNow,
) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": key.activity_id_at_version,
            "at": now.timestamp().unix_seconds(),
            "attempt": key.attempt,
            "decision": { "kind": decision.kind.as_str(), "holds": holds_json(decision.holds) },
            "item": key.item,
            "iteration_path": at.iteration_path.to_json(),
            "run_id": run_id,
            "step_id": at.step_id,
        }),
    )
}

/// **D21-76.** Whether an `activity_committed` row is a control decision's row.
#[must_use]
pub fn is_decision_row(event: &Event) -> bool {
    event.kind == EventKind::ActivityCommitted && event.payload.get("decision").is_some()
}

/// **D21-76.** The journaled verdict of the `kind` decision taken under `key`, or `None` when the
/// walk never journaled one there — which is where a re-entry stops replaying and decides fresh.
///
/// *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)* The verdict
/// is three-valued: a row whose `holds` is JSON `null` is a journaled `null` ([`Tri::Null`]), which
/// is a decision the walk replays, never an absent one. A `holds` that is none of `true`, `false`
/// and `null` (or is missing) is not read as a decision, so the walk decides fresh there.
///
/// [`Tri::Null`]: crate::predicate::Tri::Null
#[must_use]
pub fn journaled_decision(
    events: &[Event],
    key: &CommitKey,
    kind: DecisionKind,
) -> Option<crate::predicate::Tri> {
    events
        .iter()
        .filter(|e| is_decision_row(e))
        .filter(|e| row_key_of(e).as_ref() == Some(key))
        .find_map(|e| {
            let d = e.payload.get("decision")?;
            if d.get("kind")?.as_str()? != kind.as_str() {
                return None;
            }
            match d.get("holds")? {
                serde_json::Value::Bool(b) => Some(crate::predicate::Tri::from_bool(*b)),
                serde_json::Value::Null => Some(crate::predicate::Tri::Null),
                _ => None,
            }
        })
}

/// The commit key a row carries in its `item`, `activity_id_at_version`, `step_id` and
/// `iteration_path` members — a step commit's, or the key a decision is journaled under.
fn row_key_of(event: &Event) -> Option<CommitKey> {
    let text = |m: &str| event.payload.get(m).and_then(serde_json::Value::as_str);
    Some(CommitKey {
        item: text("item")?.to_owned(),
        activity_id_at_version: text("activity_id_at_version")?.to_owned(),
        step_id: text("step_id")?.to_owned(),
        iteration_path: IterationPath::from_json(event.payload.get("iteration_path")?)?,
    })
}

/// **R1.** The commit key an `activity_committed` row carries, or `None` for a pre-R1 row (no
/// `step_id`) or one whose path does not read.
///
/// *(dated 2026-09-27, `s8d` build, package D2176-8d.)* `None` for a control decision's row too
/// (D21-76): a decision is not a step activation's commit.
#[must_use]
pub fn commit_key_of(event: &Event) -> Option<CommitKey> {
    if event.kind != EventKind::ActivityCommitted || is_decision_row(event) {
        return None;
    }
    row_key_of(event)
}

/// **R1.** Whether an `activity_committed` row is a completion row.
#[must_use]
pub fn is_completion_row(event: &Event) -> bool {
    event.kind == EventKind::ActivityCommitted
        && event
            .payload
            .get("completion")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
}

/// **R1.** The attempt that committed one STEP activation, and `None` when none has.
#[must_use]
pub fn committed_step_attempt(events: &[Event], key: &CommitKey) -> Option<u32> {
    events
        .iter()
        .filter(|e| commit_key_of(e).as_ref() == Some(key))
        .find_map(|e| e.payload.get("attempt").and_then(serde_json::Value::as_u64))
        .and_then(|n| u32::try_from(n).ok())
}

/// **R1.** The committed result of one STEP activation, read back out of the journal.
#[must_use]
pub fn committed_step_result<'a>(
    events: &'a [Event],
    key: &CommitKey,
) -> Option<&'a serde_json::Value> {
    events
        .iter()
        .filter(|e| !is_completion_row(e))
        .find(|e| commit_key_of(e).as_ref() == Some(key))
        .and_then(|e| e.payload.get("result"))
}

/// The attempt of an activation's completion — a completion row, or a pre-R1 row (which completed
/// its activation on its one commit) — and `None` when it is not complete.
fn completion_attempt(events: &[Event], item: &str, activity_id_at_version: &str) -> Option<u32> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .filter(|e| {
            e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
                && e.payload
                    .get("activity_id_at_version")
                    .and_then(serde_json::Value::as_str)
                    == Some(activity_id_at_version)
        })
        .find(|e| is_completion_row(e) || e.payload.get("step_id").is_none())
        .and_then(|e| e.payload.get("attempt"))
        .and_then(serde_json::Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
}

/// One above the highest attempt any `dispatch` or `activity_committed` row of this activation
/// carries — `crate::workflow::fsm::RunState::next_attempt`'s rule, read off the rows in hand.
fn next_attempt_of(events: &[Event], item: &str, activity_id_at_version: &str) -> u32 {
    events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Dispatch | EventKind::ActivityCommitted))
        .filter(|e| {
            e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
                && e.payload
                    .get("activity_id_at_version")
                    .and_then(serde_json::Value::as_str)
                    == Some(activity_id_at_version)
        })
        .filter_map(|e| e.payload.get("attempt").and_then(serde_json::Value::as_u64))
        .filter_map(|n| u32::try_from(n).ok())
        .max()
        .map_or(1, |a| a.saturating_add(1))
}

/// **D21-77 ruling 2a.** How many `dispatch` rows the journal holds for one activation — the count
/// AC-029's cap is checked against, which no row that is not a dispatch moves.
fn dispatches_of(events: &[Event], item: &str, activity_id_at_version: &str) -> u32 {
    let n = events
        .iter()
        .filter(|e| e.kind == EventKind::Dispatch)
        .filter(|e| {
            e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
                && e.payload
                    .get("activity_id_at_version")
                    .and_then(serde_json::Value::as_str)
                    == Some(activity_id_at_version)
        })
        .count();
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// **AC-009's re-read.** The committed result for one activation, read back out of the journal.
///
/// *"Every later read — replay, resume or a second stage — resolves from that journal entry rather
/// than a new elicitation."* This is that read, and it is a function so the resolution has one
/// definition rather than a convention at each call site.
#[must_use]
pub fn committed_result<'a>(
    events: &'a [Event],
    item: &str,
    activity_id_at_version: &str,
) -> Option<&'a serde_json::Value> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .find(|e| {
            e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
                && e.payload
                    .get("activity_id_at_version")
                    .and_then(serde_json::Value::as_str)
                    == Some(activity_id_at_version)
        })
        .and_then(|e| e.payload.get("result"))
}

/// The attempt that committed one activation, and `None` when none has.
#[must_use]
pub fn committed_attempt(
    events: &[Event],
    item: &str,
    activity_id_at_version: &str,
) -> Option<u32> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .find(|e| {
            e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
                && e.payload
                    .get("activity_id_at_version")
                    .and_then(serde_json::Value::as_str)
                    == Some(activity_id_at_version)
        })
        .and_then(|e| e.payload.get("attempt"))
        .and_then(serde_json::Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
}

// ---- AC-031's second instrument -----------------------------------------------------------------

/// Why the boundary lint refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryLintViolation {
    /// An anchor is missing: the file was emptied or renamed, and an absence over nothing is
    /// vacuous.
    AnchorMissing { anchor: &'static str },
    /// A path by which structure could be recovered from free text.
    ProseExtraction { form: &'static str, line: usize },
}

impl BoundaryLintViolation {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::AnchorMissing { .. } => "anchor_missing",
            Self::ProseExtraction { .. } => "prose_extraction",
        }
    }
}

impl core::fmt::Display for BoundaryLintViolation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AnchorMissing { anchor } => write!(
                f,
                "the boundary source carries no `{anchor}`, so every absence below it is vacuous"
            ),
            Self::ProseExtraction { form, line } => write!(
                f,
                "line {line} carries `{form}`: the boundary never extracts structure from prose \
                 (AC-031's sub-clause)"
            ),
        }
    }
}

/// The forms through which structure could be recovered from free text at the boundary.
///
/// Each is a **code-position** token: [`crate::workflow::breaker::code_lines`] strips comment lines
/// and string literals before the scan, so this very array — an array of the tokens it bans — is
/// invisible to the rule that reads it. That is the same property `MODULE_STATE_FORMS` relies on one
/// module over, and the reason is the same: a lint that matched its own vocabulary would be a
/// vacuous failure exactly as an unanchored absence is a vacuous pass.
///
/// **SCOPE, stated because a guard scoped wrong cuts both ways** (golden constraint 9). These are
/// banned **in this module**, not workspace-wide: `serde_json::from_str` is how every catalog,
/// registry and golden-log fixture in this crate is read, and `from_utf8` is how `crate::replay`
/// turns canonical bytes back into a manifest member. What makes them a defect *here* is that the
/// boundary's only value source is `rk_ports::harness::DispatchOutcome::result`, which is already an
/// erased, schema-attributed `Typed` — so any text-to-structure path in this file is a second,
/// unvalidated one.
pub const PROSE_EXTRACTION_FORMS: [&str; 6] = [
    "serde_json::from_str",
    "from_utf8",
    ".stdout",
    ".stderr",
    "find('{'",
    "rfind('}'",
];

/// The anchors the rule asserts **first**, so an emptied or renamed file fails rather than passing
/// six absences.
pub const BOUNDARY_ANCHORS: [&str; 3] = ["pub fn run_activity", "spec.schema", "StepError::Schema"];

/// **AC-031's sub-clause**, and the second of its two instruments.
///
/// *"The boundary never extracts structure from prose. Verified separately by a grep-lint over the
/// boundary module rejecting any markdown/JSON-from-prose extraction path, with a must-fail
/// fixture."*
///
/// The caller supplies the source with `include_str!`, so the rule performs no I/O.
///
/// # Errors
///
/// Every clause that failed, collected — never the first one only (house Shape B).
pub fn boundary_prose_extraction_rule(src: &str) -> Result<(), Vec<BoundaryLintViolation>> {
    // THE ANCHORS FIRST, against the raw source, so an emptied or renamed file fails on the
    // positive clause rather than passing six absences — `crate::workflow::breaker`'s two lints
    // take the same order for the same reason.
    let mut out: Vec<BoundaryLintViolation> = BOUNDARY_ANCHORS
        .into_iter()
        .filter(|anchor| !src.contains(anchor))
        .map(|anchor| BoundaryLintViolation::AnchorMissing { anchor })
        .collect();
    // EVERY FORM, EVERY LINE, AND NO EARLY RETURN (house Shape B): a rule that returned on the
    // first hit would report one form per file, and the six one-form positive controls would stop
    // discriminating between the forms they are named for.
    for (line, text) in crate::workflow::breaker::code_lines(src) {
        for form in PROSE_EXTRACTION_FORMS {
            if text.contains(form) {
                out.push(BoundaryLintViolation::ProseExtraction { form, line });
            }
        }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}
