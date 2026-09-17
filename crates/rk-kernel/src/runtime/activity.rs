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
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::Typed;

use super::marks::RetryAtMark;
use super::{Runtime, RuntimeError};
use crate::workflow::breaker::JournaledNow;
use crate::workflow::envelope::AttemptCap;
use crate::workflow::idem::checked_key;
use crate::workflow::retry::{RetryDecision, decide};

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
}

impl ActivityReport {
    /// A report for a run that has not dispatched yet. **Private**: a report is produced by the
    /// loop and never minted beside it, so every count in one is a reading.
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
        }
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
    #[must_use]
    pub const fn skipped(&self) -> bool {
        self.dispatches == 0 && self.commit.is_none()
    }

    /// **AC-030's durable mark**, when the run ended on one.
    #[must_use]
    pub const fn mark(&self) -> Option<&RetryAtMark> {
        self.mark.as_ref()
    }

    /// How many events **this module** appended. The adapter's own journaling is not counted here
    /// and the test reads the store for that.
    #[must_use]
    pub const fn appended(&self) -> usize {
        self.appended
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
        // AC-010, FIRST: the refusal happens before the store is touched and long before the port
        // is reached, which is what makes "before any process is spawned" a dispatch count of 0.
        let key = checked_key(item, activity_id_at_version, attempt)?;

        // AC-012's and AC-013's shared pre-flight, read through the FSM's own fold so this module
        // and `next_action`'s step 3 cannot disagree about what "committed" means.
        if self.state(item)?.is_committed(item, activity_id_at_version) {
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
    #[expect(
        clippy::significant_drop_tightening,
        reason = "AC-012: the journal re-read and the append are ONE critical section, and taking                   the lint's advice to narrow the guard is exactly the defect this function                   exists to prevent — two acquisitions leave a window in which both attempts                   observe no commit and both append. `#[expect]` and not `#[allow]` (ADR-0102                   decision 3): if the lint stops firing, the span has narrowed and the build says so"
    )]
    pub fn commit(
        &self,
        key: &IdempotencyKey,
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
        let mut guard = self.store_guard();
        let events =
            guard
                .query(super::journal_query(&key.item))
                .map_err(|e| RuntimeError::Store {
                    op: "query",
                    detail: format!("{e:?}"),
                })?;
        if let Some(committed) = committed_attempt(&events, &key.item, &key.activity_id_at_version)
        {
            // **AC-012.** The late result is discarded and NOTHING is appended, so *"the committed
            // value is unchanged"* holds by construction rather than by a comparison.
            return Ok(CommitOutcome::DiscardedLate {
                key: key.clone(),
                committed_attempt: committed,
            });
        }
        guard
            .append(activity_committed_event(key, typed, self.run_id(), now))
            .map_err(|e| RuntimeError::Store {
                op: "append",
                detail: format!("{e:?}"),
            })?;
        Ok(CommitOutcome::Committed { key: key.clone() })
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
        let mut report = ActivityReport::opened(now);
        let state = self.state(item)?;
        // **AC-013's re-entry path.** A committed activation is not re-executed, and the predicate
        // is the FSM's own rather than a second one written here.
        if state.is_committed(item, activity_id_at_version) {
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

        loop {
            let dispatched =
                self.dispatch_once(item, activity_id_at_version, step_id, attempt, spec)?;
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
                        let commit = self.commit(&key, &typed, now)?;
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

            let decision = decide(&error, &key, cap);
            report.decisions.push(decision.clone());

            // **AC-030.** The mark, and then the run ends: no backoff (ADR-0095 forbids it on this
            // class), no sleep, no further dispatch.
            if let RetryDecision::WaitForReset { resets_at } = decision {
                let mark = RetryAtMark::carried(resets_at);
                self.append(mark.to_event())?;
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
