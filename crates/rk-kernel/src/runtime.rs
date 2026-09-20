//! **The runtime boundary** — where `rk-ports` is first CALLED, and the only module in this crate
//! that appends.
//!
//! > `crates/rk-kernel/Cargo.toml`, until 2026-09-14: *"Where `rk-ports` will first be USED: the
//! > kernel runtime boundary that appends what `select` RETURNS (Slice 4, §7b.10 — ADR-0119
//! > clause 5, ADR-0120 clause 4 and AC-525's 2026-09-13 amendment all put the append OUTSIDE the
//! > pure function). **Delete that line when its call site lands.**"*
//!
//! This is that call site, and the line is deleted. Every `rk-ports` trait method this crate ever
//! calls is called from this module or its children, and nowhere else.
//!
//! # WHY THIS MODULE IS NOT UNDER `crate::workflow`
//!
//! `src/replay.rs`:10-15's exact model, one sentence of which is the whole argument: *"Putting them
//! inside the pure function would make the criterion false by construction."* This module
//! dispatches, it appends, it binds ports and it takes a `Mutex`. `crate::workflow` carries
//! `#![deny(clippy::disallowed_methods)]` and `#![deny(clippy::disallowed_types)]`, which reach
//! every child module — so a `runtime` module underneath it would either fail to compile or force
//! an `#[expect]` per call site, and an `#[expect]` per I/O call in the module whose *purpose* is
//! I/O is a gate that has stopped meaning anything.
//!
//! **And it is not under `crate::select` either**, for the same reason `replay` is not: residual 7
//! is FORCED on this point and its Read line is followed verbatim — *"Put the FSM/runtime in its
//! own top-level module beside `replay` … and have it take an already-loaded `&Catalog`/`&Pack` by
//! value; never call `Catalog::from_dir` from inside `workflow` or `select`."* Nothing reachable
//! from this module reaches `Catalog::from_dir`, so **Slice 4 builds the workflow/activity split
//! without assigning the catalog loader a side** — the open question `src/workflow.rs`'s header
//! records (gap G-4) is routed around rather than answered, which is what `00-preamble.md`:55
//! requires of a stage.
//!
//! # The five things that happen OUTSIDE the pure function, and the records that put them here
//!
//! | act | the record that puts it outside `select()` |
//! | --- | --- |
//! | the `SelectionMemoKey` lookup | ADR-0119 clause 5 — *"the caller at the runtime boundary computes the key … and either serves the node's memoised result or calls `select()`"*. **Not built**: the key itself is AC-045's and stays ARMED under clause 8 |
//! | the `dag_amended` append | ADR-0120 clause 4 — returned by the re-run, appended by the handler. Already on disk at `crate::replay`'s `derive` |
//! | the `drift_advisory` derivation and append | AC-525's 2026-09-13 amendment clauses 2 and 3 — [`advisory`] |
//! | the `memo_hit` / `memo_miss` append | AC-019; `crate::workflow::memo::recorded_residue` is the **projection** and this module is what would write it |
//! | the executed-once ledger | ADR-0114 decision 2 — [`ledger`] |
//!
//! # The durable marks, and which `EventKind` carries each
//!
//! `EventKind` is closed at 29 (`crates/rk-types/src/event_kind.rs`:87) and **this slice mints
//! none**. Residual 10c is FREE and its Read line is followed: project the marks onto kinds that
//! exist, and say which projection is a reading.
//!
//! | the record's words | the kind used | lawful because |
//! | --- | --- | --- |
//! | AC-030 *"writes a `retry_at(resets_at)` mark"* | `EventKind::RateLimited` | design §5 row 18 puts *"`resets_at` on `rate_limited`"* — **not** a reading |
//! | AC-024 *"the kernel writes a durable mark"* (wall clock) | `EventKind::GateOpened`, kind `budget_extension` | AC-039's own mechanism half. **The mark's VALUE is on disk at `crate::workflow::budget::wall_clock_expiry_mark`; its WRITER is ARMED and has no call site — see the split table below** |
//! | AC-012's activation | `EventKind::ActivityCommitted` | design §5 row 10, *"at most once per **step activation**"* (ADR-0097 decision 2) |
//! | AC-525's advisory | `EventKind::DriftAdvisory` | design §5 row 22 |
//! | 🛑 AC-033's breaker mark | **none exists** | `RefusalClass` is closed at four and `FailureClass` at five; **ARMED, NOT DISCHARGED** since Slice 4d and unchanged here |
//!
//! # 🛑 *"exits 0"* IS NOT THIS **MODULE'S** TO ASSERT — AMENDED 2026-09-14 (D-8)
//!
//! **This section was headed *"*\"exits 0\"* IS NOT THIS CRATE'S TO ASSERT"* and said:**
//!
//! > AC-024's and AC-030's THEN each end *"and exits 0"*. `rk-kernel` is a **library**: a
//! > `cargo test` binary cannot assert its own process exit, and `std::process::exit` inside one
//! > aborts the harness. Residual 10d is FORCED and its Read line is followed — the kernel returns
//! > a terminal outcome and the process exit is `rk-cli`'s, **Step 7d**. The exit half is **ARMED,
//! > NOT DISCHARGED** and its home is named.
//!
//! **Every sentence there is true of a test asserting its OWN exit, and the conclusion drawn from
//! them was wider than they support.** A `cargo test` binary can assert a **child's** exit, and
//! operator decision **D-8** built the harness that does: `tests/it/cross_process.rs` re-execs this
//! binary through `std::env::current_exe()` with an env marker, and
//! `ac024_wall_clock_mark_outlives_the_process_that_wrote_it` reads the child's exit code beside
//! the mark the child left on disk — which is AC-024's own *Verified by*, *"a test asserting
//! **process exit** and a **written mark**"*, verbatim.
//!
//! **What is still 7d's, stated narrowly so the green is not misread:** that the process which
//! exits 0 is **`redkiln run`**. The child is a libtest binary. Residual 10d's Read line still
//! holds for the CLI half, and AC-024 is additionally a **twin** (AC-240 + AC-278 own it) and is
//! not double-claimed.
//!
//! **AC-030 is ROUTED to 7d and the harness is deliberately NOT pointed at it (D-8).** Its own
//! *Verified by* ends *"the next **`redkiln run`** resumes from the mark (AC-013's re-entry
//! test)"* — it names the binary, and AC-013's re-entry test is **PROVISIONAL** and may not be
//! counted. **AC-013 is routed the same way**: its WHEN is *"WHEN `redkiln run <item>` re-enters"*.
//! Both stay **ARMED, NOT DISCHARGED** with that reason, and `crate::runtime::activity`'s table
//! carries it per row.
//!
//! 🛑 **AND THE TWO MARKS ARE NOT JOINTLY DISCHARGED — corrected 2026-09-14 on an adversarial
//! verification pass.** The sentence that stood here said *"what is discharged here is **writes a
//! durable mark** and **no thread sleeps**"* of AC-024 **and** AC-030 together. It was false of
//! AC-024, and the split is now stated per criterion:
//!
//! | criterion | its mark's **value** | its mark's **writer**, on a live path |
//! | --- | --- | --- |
//! | **AC-030** *"writes a `retry_at(resets_at)` mark"* | [`crate::runtime::marks::RetryAtMark`] | **WRITTEN** — `activity.rs`'s `run_activity` appends it on the `WaitForReset` branch. `grep -rn RetryAtMark crates/rk-kernel/src/` names that call site |
//! | **AC-024** *"the kernel writes a durable mark"* | [`crate::workflow::budget::wall_clock_expiry_mark`] | **ARMED, NOT DISCHARGED** — it has **ZERO call sites**: `grep -rn 'wall_clock_expiry_mark(' crates/*/src/ | grep -v '^.*//'` finds only the definition at `workflow/budget.rs` [measured 2026-09-14]. *(Corrected the same day: this cell read "returns two lines, both doc text, and ZERO callers". The bare grep returns **four** lines — the definition plus three doc mentions, two of which are this cell and its twin — so the figure was falsified by the edit that wrote it. The claim that survives is about call sites, and it is stated as one.)* |
//!
//! **Why AC-024's writer is not simply added here, rather than why it would be nice to add it.**
//! The mark is a function of a [`crate::workflow::budget::BudgetLedger`] and a
//! [`crate::workflow::budget::BudgetEnvelope`], and **this module holds neither**: `grep -rn
//! 'BudgetLedger\|WallClockSecs\|charge' crates/rk-kernel/src/runtime*` returns no ledger, no
//! envelope and no charge site [measured 2026-09-14]. The ledger's `wall_clock` counter is a fold
//! over **journaled charges**, and nothing in this tree produces one: `rk_types::time::Timestamp`
//! has no `now()` *deliberately*, so the kernel cannot derive an elapsed value itself, and the one
//! measurement that exists is dropped before it could — *(corrected 2026-09-17, Step 7c Slice 1:
//! this sentence said `DispatchOutcome` / `DispatchRecord` carry **no elapsed-seconds channel**;
//! since ADR-0126 the record carries `timings.wall_ms`, the adapter's wall clock around the child,
//! but `runtime/activity.rs` keeps `outcome.result` and drops `outcome.record`, so the carrier
//! exists and the routing from it to a journaled charge is still the record OWED below)*. Deciding
//! what charges the wall clock — and from which journaled instants — is a decision
//! `00-preamble.md`:55 reserves; a record routing an elapsed value from the adapter to the kernel
//! is **OWED**, and it is named here rather than written here. The shape is AC-026's
//! exactly (`crate::workflow::retry::decide_after_inner_retries`): the **rule** is on disk and
//! pinned, the **wiring** needs a carrier no record has minted.
//!
//! # 🛑 THE STORE LOCK IS TAKEN HERE, AND THAT IS A READING WITH A NAMED TENSION
//!
//! ADR-0115 clause 1 holds the event store as `Arc<Mutex<dyn EventStore>>`, and clause 4 says *"the
//! lock is acquired by the composition root at the call boundary"*. `crates/rk-ports/src/lib.rs`
//! :20-22 restates it and adds *"Step 7a writes no `lock()` call site at all; **Step 7d owns the
//! acquisition** and owes the discriminating test."*
//!
//! **This module acquires it**, and the reason is mechanical rather than preferential:
//! `Harness::dispatch` takes `&mut dyn EventStore`, an `Arc<Mutex<_>>` is the only holding shape
//! that can produce one (ADR-0115:130-132 is the measurement), and the dispatch happens **here**.
//! *"At the call boundary"* is satisfied literally — the guard is taken immediately before the
//! store touch, released when it returns, and **never held across a call to another port**. What is
//! owed is the reconciliation of `rk-ports`' sentence with the fact that Step 7d constructs
//! [`rk_ports::root::Ports`] while Step 7b is the first stage that *calls through* one. A poisoned
//! acquisition **halts the run**: `.expect` propagates and nothing here recovers (ADR-0118 clauses
//! 1, 3 and 4; no `PoisonError::into_inner`, no resuming `catch_unwind`).
//!
//! # 🛑 THE FSM'S OWN `Dispatch` KEY IS REFUSED HERE TODAY, AND IT IS PINNED RATHER THAN PATCHED
//!
//! [`crate::workflow::fsm::next_action`] builds `NextAction::Dispatch { key }` from the DAG node's
//! `activity_id`, and a `crate::select::DagNode` carries the **bare** catalog id. AC-010's third
//! malformation is *"missing `@version`"*, so [`crate::workflow::idem::checked_key`] refuses it and
//! the dispatch never reaches the port.
//!
//! [measured 2026-09-14: `grep -rhoE '^id *= *"[^"]*"' spikes/p9/catalog/*.toml | grep -c '@'` →
//! **0** of **676**; `grep -l "^version " spikes/p9/catalog/*.toml | wc -l` → **102** of 102.] The
//! version exists on every shipped activity and `crate::catalog::Activity::id_at_version` reads it
//! as of Slice 4g. **Nothing joins the two**, and the join has three candidate homes — step 5's
//! `DagNode` construction, `next_action`'s node lookup, or the composition root. Choosing one is a
//! decision `00-preamble.md`:55 reserves, so this slice takes none and
//! `the_fsm_dispatch_key_has_no_version_and_the_boundary_refuses_it` asserts the gap instead: if a
//! later slice makes the join, that test goes red and whoever made it has to say so. **Widening
//! `checked_key` to accept a bare id is the one move that is definitely wrong** — it would make
//! AC-010's third refusal untrue. A record is OWED.
//!
//! # What is deliberately NOT here
//!
//! **No `CancelToken`** — ADR-0099 clause 1: the token is constructed at the composition root,
//! cloned into the adapter at construction, and *"the kernel never calls it and never holds it"*.
//! `crates/rk-ports/src/cancel.rs`:11-13 records that nothing in the instrument set would catch a
//! violation, so this is a read-the-record ban and not a lint. **No `pub trait` of any shape** —
//! `src/replay.rs`:200-203 records that AC-483's denylist parses `rk-ports`' source only and would
//! not see one declared here. **No async runtime, no timer crate, no `std::thread::sleep`**, and no
//! clock: every timestamp this module writes arrives on a value the caller handed it
//! ([`rk_types::time::Timestamp`] has no `now()`, *deliberately*).
//!
//! **And no stage ORDERING.** [`crate::workflow::fsm::NextAction::AdvanceStage`] carries the
//! successor the caller supplied; this module appends the transition it is told to append and
//! invents no next stage (`spikes/p9/packs/incident.toml`:1-2, its own first comment).

use std::sync::MutexGuard;

use rk_ports::event_store::{EventStore, Query};
use rk_ports::root::Ports;
use rk_types::activity::ActivitySpec;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::select::Digest;
use crate::workflow::breaker::JournaledNow;
use crate::workflow::envelope::AttemptCap;
use crate::workflow::fsm::{FsmError, NextAction};
use crate::workflow::idem::KeyError;

pub mod activity;
pub mod advisory;
pub mod ledger;
pub mod marks;

/// What a poisoned store lock means. **ADR-0118 clause 1**: *"the run stops at the poisoned
/// acquisition. No `StepError` is constructed, nothing crosses a port boundary, no retry, no next
/// activity, no gate answer."* `.expect` propagates; it does not resume.
/// **`pub` since Step 7d Slice 2 — a visibility change and nothing else** (§7d.1 edit (8)).
/// `rk-cli`'s panic hook matches THIS constant, so that a poison first observed inside the
/// kernel — which unwinds out through `.expect` below — is turned into an explicit exit `70`
/// rather than the unhandled panic's `101`. The hook substring-matches the payload against this
/// string and never against a second copy of it, which is the whole reason the constant is
/// exported rather than duplicated: two copies can drift, and the drift would be invisible
/// (every test green, the hook silently at 101).
///
/// **The text is UNCHANGED.** Moving a byte of it would break the match it exists for.
pub const LOCK_HALTS: &str =
    "a poisoned store lock halts the run; it is never recovered (ADR-0118)";

/// Why the runtime boundary refused.
///
/// House Shape A: a stable tag per variant, hand-written `Display`, no `thiserror` — the crate's
/// eleven-error precedent (`catalog.rs`, `select.rs`, `replay.rs`, `rigor.rs`, `registry.rs`,
/// `select/pack.rs`, `select/step2.rs`, `select/step4.rs`, `select/step5.rs`,
/// `select/artifacts.rs`, `workflow/memo.rs`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// AC-010: the key was malformed. **Raised before the `Harness` port is reached.**
    Key(KeyError),
    /// The `EventStore` port refused. Carried as text, because `rk_types::error::StepError` is the
    /// port's error type and re-wrapping it whole would put a dispatch's error channel into the
    /// boundary's own (AC-032).
    Store {
        /// `append` or `query`.
        op: &'static str,
        detail: String,
    },
    /// A `NextAction::Dispatch` arrived with no [`ActivitySpec`]. **Refused, never defaulted**: an
    /// empty spec is a dispatch of a card nobody wrote.
    SpecNotSupplied { step_id: String },
    /// The FSM refused to fold the journal this boundary read back.
    Fsm(FsmError),
}

impl RuntimeError {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Key(_) => "key",
            Self::Store { .. } => "store",
            Self::SpecNotSupplied { .. } => "spec_not_supplied",
            Self::Fsm(_) => "fsm",
        }
    }
}

impl core::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Key(e) => write!(f, "the dispatch was refused before any spawn: {e}"),
            Self::Store { op, detail } => write!(f, "the event store refused `{op}`: {detail}"),
            Self::SpecNotSupplied { step_id } => write!(
                f,
                "step `{step_id}` is dispatchable but no ActivitySpec was supplied; the boundary \
                 refuses rather than dispatching an empty one"
            ),
            Self::Fsm(e) => write!(f, "the recorded journal did not fold: {e}"),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<KeyError> for RuntimeError {
    fn from(e: KeyError) -> Self {
        Self::Key(e)
    }
}

impl From<FsmError> for RuntimeError {
    fn from(e: FsmError) -> Self {
        Self::Fsm(e)
    }
}

/// What one [`Runtime::step`] did.
///
/// House Shape C: the outcome carries what was appended, so *"the kernel took no action"* is
/// distinguishable from *"the kernel acted and reported nothing"*. Three of the six variants append
/// **nothing at all**, and that is the criterion in each case rather than an omission —
/// `AwaitGate` because ADR-0022 says the kernel takes no action until a gate is answered, `Halt`
/// because the refusal is already in the journal, and `Done` because there is nothing left.
#[derive(Debug, Clone, PartialEq)]
pub enum StepOutcome {
    /// `NextAction::Dispatch` — the activity ran, with whatever retries the policy authorised.
    Activity(Box<activity::ActivityReport>),
    /// `NextAction::OpenGate` — one tier-1 `gate_opened` appended, carrying the derived token.
    GateOpened { token: Digest },
    /// `NextAction::AwaitGate` — **nothing appended** (ADR-0022).
    Awaiting { token: Digest },
    /// `NextAction::AdvanceStage` — one tier-1 `stage_transition` appended.
    StageAdvanced { from: String, to: String },
    /// `NextAction::Halt` — **nothing appended**.
    Halted { reason: &'static str },
    /// `NextAction::Done` — **nothing appended**.
    Done,
}

impl StepOutcome {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Activity(_) => "activity",
            Self::GateOpened { .. } => "gate_opened",
            Self::Awaiting { .. } => "awaiting",
            Self::StageAdvanced { .. } => "stage_advanced",
            Self::Halted { .. } => "halted",
            Self::Done => "done",
        }
    }

    /// How many events this step appended. **Read off the store by the test, and stated here so the
    /// two cannot disagree**: three of the six are zero by the records quoted on the variants.
    #[must_use]
    pub fn appended(&self) -> usize {
        match self {
            Self::Activity(report) => report.appended(),
            Self::GateOpened { .. } | Self::StageAdvanced { .. } => 1,
            Self::Awaiting { .. } | Self::Halted { .. } | Self::Done => 0,
        }
    }
}

/// The impure shell: the five ports, the run id, and the one method that turns a decision into an
/// effect.
///
/// **Configuration reaches it as a value** (ADR-0083's Configuration clause: *"a `Config` struct
/// constructed at the composition root and passed in"*, with its forbids *"Forbids the kernel
/// reading the process environment"*). `Ports` is constructed by the caller; the run id is a string
/// the caller supplies; nothing here reads an environment variable, a path or a clock.
pub struct Runtime {
    ports: Ports,
    run_id: String,
    stage: String,
}

impl Runtime {
    /// Bind the ports, the run id and the stage. **Nothing is read here** — no file, no environment
    /// variable, no clock.
    ///
    /// The stage is an argument because [`crate::workflow::fsm::fold`] takes one, and this module
    /// reads *"has this activation committed?"* through that fold rather than through a second
    /// predicate of its own — so AC-013's *"no committed step re-executes"* and
    /// `next_action`'s step 3 share one definition and cannot disagree.
    #[must_use]
    pub fn new(ports: Ports, run_id: &str, stage: &str) -> Self {
        Self {
            ports,
            run_id: run_id.to_owned(),
            stage: stage.to_owned(),
        }
    }

    /// The stage this runtime is running.
    #[must_use]
    pub fn stage(&self) -> &str {
        &self.stage
    }

    /// The bound ports, for a caller that needs one this module does not call.
    #[must_use]
    pub const fn ports(&self) -> &Ports {
        &self.ports
    }

    /// The run id every [`ledger::ExecutionStamp`] this runtime writes carries.
    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// The store, locked. **The guard's lifetime is the call boundary** (ADR-0115 clause 4) and a
    /// poisoned acquisition halts the run (ADR-0118 clause 1).
    fn store_guard(&self) -> MutexGuard<'_, dyn EventStore + 'static> {
        self.ports.store.lock().expect(LOCK_HALTS)
    }

    /// Everything the store holds for one item, in append order.
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Store`] when the port refuses.
    pub fn journal(&self, item: &str) -> Result<Vec<Event>, RuntimeError> {
        self.store_guard()
            .query(journal_query(item))
            .map_err(|e| RuntimeError::Store {
                op: "query",
                // `rk_types::error::StepError` carries no `Display` impl [measured 2026-09-14:
                // `grep -n Display crates/rk-types/src/error.rs` -> 0 hits], so the port's refusal
                // is carried as its `Debug` form rather than re-wrapped whole: a `StepError` inside
                // a `RuntimeError` would put a dispatch's error channel into the boundary's own,
                // which is the collapse AC-032 exists to prevent one level up.
                detail: format!("{e:?}"),
            })
    }

    /// Append one event through the port.
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Store`] when the port refuses.
    pub fn append(&self, event: Event) -> Result<(), RuntimeError> {
        self.store_guard()
            .append(event)
            .map_err(|e| RuntimeError::Store {
                op: "append",
                detail: format!("{e:?}"),
            })
    }

    /// The FSM's view of this item's journal.
    ///
    /// **One definition of *"has this activation committed?"***: `next_action`'s step 3 skips a
    /// committed node and [`activity::Runtime::run_activity`] skips one too, and both read it off
    /// this fold. A second predicate here would be a second answer to AC-013's question.
    ///
    /// # Errors
    ///
    /// [`RuntimeError::Store`] from the port, or [`RuntimeError::Fsm`] if the journal does not fold.
    pub fn state(&self, item: &str) -> Result<crate::workflow::fsm::RunState, RuntimeError> {
        let events = self.journal(item)?;
        Ok(crate::workflow::fsm::fold(item, &self.stage, &events)?)
    }

    /// **Turn one decision into one effect.** The only method in this crate that acts.
    ///
    /// `spec` is `Some` only for `NextAction::Dispatch`, and a `Dispatch` without one is
    /// [`RuntimeError::SpecNotSupplied`] rather than a defaulted spec. `cap` is the pack's `N`,
    /// which this crate refuses to default (AC-029). `now` is a journaled instant — the only
    /// shape time enters in.
    ///
    /// # Errors
    ///
    /// [`RuntimeError`] from the store, the key validator or the fold.
    pub fn step(
        &self,
        action: &NextAction,
        spec: Option<&ActivitySpec>,
        cap: AttemptCap,
        now: JournaledNow,
    ) -> Result<StepOutcome, RuntimeError> {
        match action {
            NextAction::OpenGate(record) => {
                self.append(gate_opened_event(record))?;
                Ok(StepOutcome::GateOpened {
                    token: record.token().clone(),
                })
            }
            // ADR-0022: escalation is a kernel-owned durable `Gate:` mark and **the kernel takes no
            // action until it is answered**. Appending anything here — even a "still waiting" row —
            // would be an action.
            NextAction::AwaitGate { token } => Ok(StepOutcome::Awaiting {
                token: token.clone(),
            }),
            NextAction::Dispatch { key, step_id } => {
                let Some(spec) = spec else {
                    return Err(RuntimeError::SpecNotSupplied {
                        step_id: step_id.clone(),
                    });
                };
                let report = self.run_activity(
                    &key.item,
                    &key.activity_id_at_version,
                    step_id,
                    spec,
                    cap,
                    now,
                )?;
                Ok(StepOutcome::Activity(Box::new(report)))
            }
            NextAction::AdvanceStage { from, to } => {
                self.append(stage_transition_event(from, to))?;
                Ok(StepOutcome::StageAdvanced {
                    from: from.clone(),
                    to: to.clone(),
                })
            }
            // The refusal is ALREADY in the journal — `fold` reads `state.halt` off a recorded
            // `step_refused` — so writing a second row would record the same fact twice and move
            // the byte-identity every replay comparison is measured under.
            NextAction::Halt { reason } => Ok(StepOutcome::Halted {
                reason: reason.as_str(),
            }),
            NextAction::Done => Ok(StepOutcome::Done),
        }
    }
}

/// The `gate_opened` a runtime mark carries: `kind`, `tree_hash`, `deadline` and the derived token.
///
/// Shaped so `crate::workflow::fsm::fold`'s `gate_opened` arm reads it as **the runtime's** producer
/// and not as the selection's announcement — the two share one kind and the fold tells them apart by
/// the presence of `tree_hash`/`deadline` (that module's header, reading 3).
#[must_use]
pub fn gate_opened_event(record: &crate::workflow::fsm::GateRecord) -> Event {
    Event::new(
        EventKind::GateOpened,
        serde_json::json!({
            "deadline": record.deadline().unix_seconds(),
            "kind": record.kind(),
            "reach": record.reach(),
            "token": record.token().as_str(),
            "tree_hash": record.tree_hash(),
        }),
    )
}

/// The tier-1 `stage_transition` a stage advance writes.
///
/// `from` is always present here, because an advance has a predecessor by construction — which is
/// the one case `crate::replay`'s `stage_entry` cannot say (a run's first stage has none).
#[must_use]
pub fn stage_transition_event(from: &str, to: &str) -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": from, "to": to }),
    )
}

/// The selector every journal read in this module uses.
///
/// `Query::ByItem` and not `ByRun`: AC-011's uniqueness assertion is over *"the whole event log"*
/// for an item, AC-012's discard is per step activation of `(item, activity_id@version)`, and
/// AC-013's re-entry reuses the item deliberately (AC-010's own *Note*). A run-scoped read would
/// make every one of those three false across a re-entry, which is the failure AC-013 exists to
/// find.
#[must_use]
pub fn journal_query(item: &str) -> Query {
    Query::ByItem(item.to_owned())
}
