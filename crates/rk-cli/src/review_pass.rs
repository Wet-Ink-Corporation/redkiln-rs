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
use rk_ports::toolchain::{Selection, StepBound, TestStatus, Toolchain};
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
///
/// 🛑 **MATCHED BY EXACT EQUALITY, AND THE NEAR MISS IS ALREADY ON THE TREE.** The shipped catalog
/// provides **`PrototypeFindings`** (two activities, measured 2026-09-23), which a `contains`,
/// `starts_with` or `ends_with` rule would match — firing the review pass over a prototype's
/// output and confirming it as a review verdict. `==` is the whole guard against that, and
/// relaxing it is not a tidy-up.
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
/// *(Moved 2026-09-26, `s8d` build, package P4; R2 = ADR-0173 decision 9, proposed.)* The ONE
/// selection parser now lives in `rk_kernel::runtime::reference`, beside the resolver's selection
/// mapping, because `rk-kernel` cannot depend on `rk-cli` and a second parser is forbidden. It is
/// re-exported here so `rk_cli::review_pass::selection_of` keeps its path and its tests their
/// import; its rule and its doc (`None` rather than a guess or an empty selection) moved with it.
pub use rk_kernel::runtime::reference::selection_of;

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
    /// ~~`Toolchain::test`'s contract is *"`Ok(count)` only on a green suite"* (RT-22), so `is_err()`
    /// **is** "fails" and no output parsing is needed or done.~~ *(Amended 2026-09-26, s8c §8c.2:
    /// record T = ADR-0169 decision 2, proposed, reverses RT-22 and supersedes ADR-0162 decision 4's
    /// *"`is_err()` **is** "fails""* in part. `test` now returns `Ok(TestOutcome)` for a red suite
    /// too, so a cell reads `failed > 0` on each arm when BOTH are `Ok`. **If either arm is `Err`
    /// -- cannot run, killed, unreadable -- there is no record**, which `confirm_with` reads as
    /// `Unverifiable { cannot_run }` (AC-344: *"both-fail or cannot-run resolves to `Unverifiable {
    /// both_fail | cannot_run }` … never to a block"*). Before this, a candidate whose program could
    /// not start beside a green reference was `four_cell(true, false)`, a BLOCK. A run that
    /// executed NOTHING on either arm (`status: empty`) is read the same way: a selection that ran no
    /// test separates nothing, and an empty suite is never read as a pass (AC-332) -- the stricter
    /// of the two readings record T leaves open, and it can never produce a block.)*
    ///
    /// Returns `None` — which `confirm_with` reads as `Unverifiable { cannot_run }` (AC-287) — for
    /// a `mutate` repro and for a command naming no selection. **`mutate` is refused by name**: no
    /// `Toolchain` method applies a patch, ~~Checkpoint 7a counts exactly six methods on that port,
    /// and inventing a seventh for a fixture is the cost `s7d` refused for `Pack`~~ *(corrected
    /// 2026-09-26, s8c §8c.7: Checkpoint 7a counts no method (E-0162-2), and the port's seventh
    /// method, record V's `run_verb`, is not a patch applier; ADR-0162's refusal stands on its
    /// remaining reason, "no port method applies a patch")*.
    fn run(&self, finding: &Finding, sha: &str) -> Option<ReExecutionRecord> {
        if finding.repro.kind != ReproKind::Run {
            return None;
        }
        let selection = selection_of(&finding.repro.command)?;

        let selection = Selection::Filter(selection);
        let candidate = self.candidate.test(&selection, StepBound::Undeclared);
        let reference = self.reference.test(&selection, StepBound::Undeclared);
        // Either arm that could not run, was killed or was unreadable: no cell (AC-344).
        let (Ok(candidate), Ok(reference)) = (candidate, reference) else {
            return None;
        };
        // Either arm that executed nothing: no cell either (AC-332; the doc above).
        if candidate.status == TestStatus::Empty || reference.status == TestStatus::Empty {
            return None;
        }
        let outcome = four_cell(candidate.failed > 0, reference.failed > 0);

        // What the gate SAW, which the classifier reads against the repro's own two strings. A
        // green candidate is reported AS the repro's `expected_when_met`, which is what
        // `review::classify` compares for equality — so a suite that passes classifies `advisory`
        // and cannot block. A RED candidate names its counts, so it can never equal
        // `expected_when_met` (record T decision 2); a repro that happened to spell those exact
        // words is told apart by the marker appended below rather than read as green.
        let observed = if candidate.failed == 0 {
            finding.repro.expected_when_met.clone()
        } else {
            let counts = format!(
                "red suite: {} passed, {} failed ({} {})",
                candidate.passed, candidate.failed, candidate.runner.name, candidate.runner.version
            );
            if counts == finding.repro.expected_when_met {
                format!("{counts} [observed, not the expected-when-met string]")
            } else {
                counts
            }
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

// ------------------------------------------------------------------------------------------------
// R5 (ADR-0177, proposed): THE REVIEW ROUTES THE COMPOSITION ROOT PERFORMS. *(Added 2026-09-26,
// `s8d` build, package P7.)*
// ------------------------------------------------------------------------------------------------

/// A toolchain rooted at a project directory: how the root builds each re-execution arm.
pub type ToolchainAt =
    Box<dyn Fn(&std::path::Path) -> Result<std::sync::Arc<dyn Toolchain>, String> + Send + Sync>;

/// **`shallow_clone` and `verdict_record`'s routes, as `rk-cli` performs them** (R5 = ADR-0177
/// decisions 2-5).
///
/// `rk-kernel` opens no files and `rk-git` is a concrete crate, not a port, so the kernel's two
/// review routes reach the world through this value, bound on the run's `ToolRouting`. The clone is
/// ADR-0098 decision 2's retained gix write, `rk_git::clone::prepare_review_clone`, with a working
/// tree checked out (`WorkingTree::Declared`): `review::confirm` re-executes each finding's test in
/// it. The re-executor is [`ToolchainReExecutor`]'s two arms: the candidate rooted in the clone and
/// the reference rooted at `reference_root`.
pub struct ReviewCloneRoutes {
    /// The clone's source, a `file://` URL naming the repository.
    pub source_url: String,
    /// Where clones are written, one directory per reviewed sha: `<clones_dir>/<sha>`.
    pub clones_dir: std::path::PathBuf,
    /// The reference arm's project root.
    pub reference_root: std::path::PathBuf,
    /// Builds each arm's toolchain.
    pub toolchain_at: ToolchainAt,
}

/// The two arms, owned, so the kernel can hold the re-executor for the length of one confirm.
struct OwnedReExecutor {
    candidate: std::sync::Arc<dyn Toolchain>,
    reference: std::sync::Arc<dyn Toolchain>,
}

impl ReExecutor for OwnedReExecutor {
    fn run(&self, finding: &Finding, sha: &str) -> Option<ReExecutionRecord> {
        ToolchainReExecutor {
            candidate: self.candidate.as_ref(),
            reference: self.reference.as_ref(),
        }
        .run(finding, sha)
    }
}

impl rk_kernel::runtime::step::ReviewRoutes for ReviewCloneRoutes {
    fn clone_at(&self, sha: &str) -> Result<serde_json::Value, rk_types::error::StepError> {
        let io = |detail: String| rk_types::error::StepError::Transport {
            kind: rk_types::error::TransportKind::Io,
            detail,
        };
        // A reviewed sha is a full object id. Anything else is refused before it can name a path.
        if sha.len() != 40 || !sha.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(rk_types::error::StepError::Unsupported {
                capability: format!(
                    "`shallow_clone` was handed `{sha}`, which is not a 40-hex commit id; a review \
                     clone is made at a full sha (R5 = ADR-0177 decision 3)"
                ),
            });
        }
        let dest = self.clones_dir.join(sha);
        // A leftover from a killed run is not this run's clone: its commit was never journalled,
        // or the step would have been served and this call never made (R1).
        if dest.exists() {
            std::fs::remove_dir_all(&dest).map_err(|e| {
                io(format!(
                    "the stale review clone {} could not be removed: {e}",
                    dest.display()
                ))
            })?;
        }
        std::fs::create_dir_all(&self.clones_dir)
            .map_err(|e| io(format!("{}: {e}", self.clones_dir.display())))?;
        rk_git::clone::prepare_review_clone(
            &self.source_url,
            &dest,
            sha,
            rk_git::clone::WorkingTree::Declared,
        )
        .map_err(|e| io(format!("the review clone at {sha} failed: {e:?}")))?;
        Ok(serde_json::json!({
            "path": dest.display().to_string(),
            "sha": sha,
            "working_tree": "declared",
        }))
    }

    fn reexecutor(
        &self,
        workspace: &serde_json::Value,
    ) -> Result<Box<dyn ReExecutor + '_>, rk_types::error::StepError> {
        let refuse = |why: String| rk_types::error::StepError::Unsupported {
            capability: format!("`verdict_record` cannot re-execute: {why} (R5 = ADR-0177)"),
        };
        let path = workspace
            .get("path")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| refuse("the committed `Workspace` carries no `path`".to_owned()))?;
        let candidate = (self.toolchain_at)(std::path::Path::new(path))
            .map_err(|e| refuse(format!("no toolchain at the review clone {path}: {e}")))?;
        let reference = (self.toolchain_at)(&self.reference_root).map_err(|e| {
            refuse(format!(
                "no toolchain at the reference root {}: {e}",
                self.reference_root.display()
            ))
        })?;
        Ok(Box::new(OwnedReExecutor {
            candidate,
            reference,
        }))
    }
}
