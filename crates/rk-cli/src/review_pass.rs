//! The review pass — **`Exit::FindingsBlock`'s producer** (ADR-0162).
//!
//! `exit.rs` has promised this since 7b-ii: *"The review gate's block verdict surfaced by `run`.
//! The value exists so the row is distinct; its producer is the kernel's, built at 7b-ii."* The
//! verdict machine — `rk_kernel::review` (10 files) and `rk_kernel::verify` (6) — has been built
//! and tested since then with **zero `src/` callers**. This module is the caller.
//!
//! **What it does NOT do, and the omissions are the point.** It constructs no verdict: `Block`'s
//! members are private and it has no public constructor (AC-294), so the only way to a block is to
//! read one back out of a [`Confirmed`] that `rk_kernel::review::confirm` produced. It invents no
//! finding set: the set is read from the **journal**, off `activity_committed`'s `result` member,
//! which is *"the dispatch's `Typed.value` **verbatim** (AC-009)"*. And it reads no prose — the
//! payload is the committed structured value or there is no pass (AC-318, AC-282).
//!
//! 🛑 **A producer nothing reaches is the defect this module exists to avoid.** Checkpoint 7h
//! guard (10) as authored is a `grep` over `crates/rk-cli/src/` that a **doc comment** satisfies,
//! and this programme has paid for that shape twice — both `s7g` source lints went green while
//! `execute_step` had no caller, and `0158` measured three declared tier-1 tables with no writer.
//! So the assertion behind this module is a **binary run that exits 4**, not the presence of the
//! identifier below.
//!
//! **`0101` clause 5 bounds what any of this may be read as.** S-SUBTLE-CB is registered and unrun;
//! the gate exists and fails closed, and nothing here claims more than that.

use rk_kernel::review::classify::{Observation, RunObservation};
use rk_kernel::review::confirm::{ConfirmInput, Confirmed, confirm_with};
use rk_kernel::review::policy::ReviewWeight;
use rk_kernel::review::reexec::{ReExecutionRecord, ReExecutor, four_cell};
use rk_ports::toolchain::Toolchain;
use rk_types::artifact::findings::{Finding, Findings, IngestError};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::ReproKind;
use rk_types::time::Timestamp;

/// The schema id the review activity's `agent` step commits its finding set under.
///
/// The row is selected by **schema**, not by stage name: `resolved.stage` is compared to no string
/// literal anywhere in `crates/rk-cli/src/`, and ADR-0162 decision 5 does not introduce the first
/// one — a stage-name match would bind the wiring to one pack's spelling of one stage.
pub const FINDINGS_SCHEMA: &str = "Findings";

/// How the pass declined to produce a verdict. **Never a block**, and never an empty finding set
/// standing in for an absent one (AC-282).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassSkipped {
    /// The run committed no activation under [`FINDINGS_SCHEMA`] — this activation is not a review,
    /// which is the ordinary case for every other stage.
    NoFindingsRow,
    /// A row was found and its payload is not a well-formed `Findings` (AC-281).
    Malformed(IngestError),
}

/// The finding set this run committed, read out of its own journal.
///
/// 🛑 **The LAST matching row, not the first.** A run that re-dispatched an activation appends a
/// second `activity_committed`, and the verdict is about what the run finally committed. `explain`
/// resolves its run id the same way, off the same kind, for the same reason.
///
/// # Errors
/// [`PassSkipped::NoFindingsRow`] when no `activity_committed` row carries the review schema;
/// [`PassSkipped::Malformed`] when one does and its `result` fails `Findings::ingest`.
pub fn findings_in(events: &[Event]) -> Result<Findings, PassSkipped> {
    let row = events
        .iter()
        .rev()
        .find(|e| {
            e.kind == EventKind::ActivityCommitted
                && e.payload.get("schema").and_then(serde_json::Value::as_str)
                    == Some(FINDINGS_SCHEMA)
        })
        .ok_or(PassSkipped::NoFindingsRow)?;

    // `result` is `Typed.value` verbatim (AC-009). An absent member is NOT an empty set: it is a
    // row that cannot be a review, and it is reported as an absence rather than approved by one.
    let payload = row
        .payload
        .get("result")
        .ok_or(PassSkipped::NoFindingsRow)?
        .clone();

    Findings::ingest(payload).map_err(PassSkipped::Malformed)
}

/// The selection a `run` repro names: the tokens of `command` after a `--` separator.
///
/// 🛑 **`None` rather than a guess, and `None` rather than an empty selection.** `Repro.command` is
/// a shell string and `Toolchain::test` takes a selection, and no record maps one to the other; the
/// narrowest rule that makes AC-288's four cells computable from what a `Repro` actually carries is
/// the explicit `--` filter. A command without one is refused into `Unverifiable { cannot_run }`,
/// which AC-287 forbids from blocking — and an empty selection is refused outright, because
/// `ac332_empty_selection_is_never_green` is the standing finding that an empty suite is not a pass.
#[must_use]
pub fn selection_of(command: &str) -> Option<Vec<String>> {
    let (_, tail) = command.split_once(" -- ")?;
    let selection: Vec<String> = tail
        .split_whitespace()
        .map(str::to_owned)
        .filter(|t| !t.is_empty())
        .collect();
    (!selection.is_empty()).then_some(selection)
}

/// The re-executor: **two arms, two trees, AC-288's four cells** (ADR-0162 decision 4).
///
/// 🛑 **Both arms are run, and that is not belt-and-braces.** `four_cell`'s only `Flipped` cell is
/// `(true, false)`; a command that fails at *both* ends separates nothing and is
/// `Unverifiable { both_fail }` — the cell its own doc calls the one *"a flip-only implementation
/// gets wrong"*. A single-arm re-executor would report every broken command as a block.
pub struct ToolchainReExecutor<'a> {
    /// Rooted in the run-owned worktree: the change under review.
    pub candidate: &'a dyn Toolchain,
    /// Rooted in an ephemeral worktree at the merge base, which the pass REMOVES (decision 10).
    pub reference: &'a dyn Toolchain,
}

impl ReExecutor for ToolchainReExecutor<'_> {
    /// Re-run the repro's selection at both ends.
    ///
    /// `Toolchain::test`'s contract is *"`Ok(count)` only on a green suite"* (RT-22), so `is_err()`
    /// **is** "fails" and no output parsing is needed or done.
    ///
    /// Returns `None` — which `confirm_with` reads as `Unverifiable { cannot_run }` (AC-287) — for
    /// a `mutate` repro and for a command naming no selection. **`mutate` is refused by name**: no
    /// `Toolchain` method applies a patch, Checkpoint 7a counts exactly six methods on that port,
    /// and inventing a seventh for a fixture is the cost `s7d` refused for `Pack`.
    fn run(&self, finding: &Finding, sha: &str) -> Option<ReExecutionRecord> {
        if finding.repro.kind != ReproKind::Run {
            return None;
        }
        let selection = selection_of(&finding.repro.command)?;

        let candidate = self.candidate.test(&selection);
        let reference = self.reference.test(&selection);
        let outcome = four_cell(candidate.is_err(), reference.is_err());

        // What the gate SAW, which the classifier reads against the repro's own two strings. A
        // green candidate is reported AS the repro's `expected_when_met`, which is what
        // `review::classify` compares for equality — so a suite that passes classifies `advisory`
        // and cannot block, and a suite that fails carries the refusal verbatim.
        let observed = match &candidate {
            Ok(_) => finding.repro.expected_when_met.clone(),
            // `StepError` carries no `Display` — deliberately, so a refusal is never rendered as
            // prose a reader could mistake for a message. The debug form is the whole value.
            Err(e) => format!("{e:?}"),
        };

        Some(ReExecutionRecord {
            id: format!("reexec:{}:{sha}", finding.ac),
            observation: Observation::Run(RunObservation {
                ran: true,
                observed,
                // This re-executor watches a SUITE, never a single assertion, so it never reports
                // an assertion flip and never produces `narrowed-pin` from a `run` repro. Saying so
                // here rather than leaving a `false` to be read as an observation.
                only_assertion_flipped: false,
            }),
            outcome,
        })
    }
}

/// Confirm one review's finding set: the rows to append, and the block set to exit on.
///
/// `now` is a journaled instant supplied by the caller — this module reads no clock (AC-001).
#[must_use]
pub fn confirm(
    findings: &Findings,
    weight: ReviewWeight,
    sha: &str,
    review_id: &str,
    now: Timestamp,
    reexecutor: &dyn ReExecutor,
) -> Confirmed {
    // No quarantines are read at this stage: nothing persists them yet, and an empty map is the
    // honest reading of "none recorded" rather than a claim that none exist.
    let quarantines = std::collections::BTreeMap::new();
    confirm_with(
        &ConfirmInput {
            findings,
            weight,
            sha,
            review_id,
            now,
            quarantines: &quarantines,
        },
        reexecutor,
    )
}
