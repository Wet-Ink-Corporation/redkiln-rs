//! **The verify pass — `TestReport`'s one producer on a lane** (`s8g` §8g.2; R1 = ADR-0191
//! decisions 1 and 2, proposed; D21-41 (c) as superseded by D21-42 1a; D21-120, TAKEN PROVISIONALLY
//! under D21-105). *(Added 2026-09-29, `s8g` build, package S8G-P5-VERIFY-PASS.)*
//!
//! Beside `review_pass.rs`, on its shape: a composition-root pass the kernel cannot make, because
//! the kernel opens no files and names no adapter.
//!
//! - **Selected by the required kind, never by a stage name.** The pass runs for each instance the
//!   kernel reports OWED on a stage's advance (`RecordedRun::pass_owed()`, D21-120) whose kind is
//!   [`REQUIRED_KIND`]. `crates/rk-cli/src/` compares no stage to a string literal (ADR-0162
//!   decision 5; Checkpoint 8g (6)), and the fixture that proves it names its stage `check`.
//! - **The order is §8g.2's steps 1-5 and 7.** (1) It meets a set-up tree: [`set_up`] runs the
//!   `setup` verb on a tree this run CREATED, and the pass takes the [`SetUp`] it returns, so no
//!   `test` call can precede `setup` and a failed `setup` leaves zero. (2) It refuses before any
//!   port call when the changed-file set or the selection policy is unresolved (AC-320). (3)
//!   `rk_kernel::verify::runner::run_test` makes the one `Toolchain::test` call, whole-suite, and
//!   journals `verify_completed` with `origin = "pass"`. (4) The outcome maps: green only when the
//!   suite passed; `failed > 0` (or an empty run, AC-332) is a failed verify; a timeout or an OS
//!   kill is `Unverifiable` (AC-331); a spawn failure is `Unverifiable { cannot_run }` (AC-344).
//!   (5) A green report is validated through `report_at_boundary` (AC-324), passed through ADR-0104
//!   decision 3's comparator, and written through `rk-store`'s writer: ONE `artifact.write`, keyed
//!   by the instance's `toolchain` element when it has one (ADR-0073), never the adapter name. (7)
//!   The caller re-enters `resume`, which re-reads the journal from the store.
//! - ~~**Step 6 is not here.** The AC ledger, the stage-exit check and the mutation harness over the
//!   item's criteria are §8g.2 step 6 and §8g.5's (AC-338, AC-339), a later package's.~~
//!   *(dated 2026-09-29, `s8g` build, package S8G-P6-MUTATION-RESUME-DRIFT.)* **Step 6 is
//!   [`run_gated`]'s gate, decided before the write**: the composition root reads the item's
//!   criteria ([`criteria_of`]), runs [`mutate`] for each declared mutation, computes the ledger and
//!   the stage-exit check, and on a refusal writes no `TestReport` and has the kernel mint AC-357's
//!   gate (`rk_kernel::verify::ac_gate`).
//! - **A failed or unverifiable verify writes no `TestReport`.** The kernel's pass-owed guard then
//!   keeps holding the stage, and `run` exits by name: a red suite is never a report of green.
//!
//! **What it never touches**: a `Harness` (AC-347: no dispatch participates in verify) and the
//! `Tracker` (AC-341: no verify result writes a status). Neither is a parameter here.

use rk_kernel::select::Instance;
use rk_kernel::verify::runner::VerifyIdentity;
use rk_ports::toolchain::{Selection, StepBound, TestStatus, Toolchain};
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::verify::BasisReason;

/// The kind this pass produces, and the one it is selected by.
pub const REQUIRED_KIND: &str = "TestReport";

/// The whole-suite reason the pass requests while no affected-selection probe is recorded at the
/// pin (AC-328's explicit whole-suite policy; AC-352's closed enum).
pub const PASS_POLICY: BasisReason = BasisReason::ProbeUnverified;

/// The owed instances this pass is selected for: those whose kind is [`REQUIRED_KIND`], by exact
/// equality, never by the stage they were owed at.
#[must_use]
pub fn selected(owed: &[Instance]) -> Vec<&Instance> {
    owed.iter().filter(|i| i.kind == REQUIRED_KIND).collect()
}

/// Whether the run-owned tree was created by this invocation or re-entered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tree {
    Created,
    Reused,
}

/// The `setup` verb's call, as the per-toolchain table declares it (`setup_projects`, the row's
/// timeout key as its bound).
#[derive(Debug, Clone)]
pub struct SetupRequest {
    pub projects: Vec<serde_json::Value>,
    pub bound: StepBound,
}

/// **The witness that the tree the pass runs in is set up** (§8g.2 step 1). Two constructors make
/// one. [`set_up`] runs `setup` on a CREATED tree, and returns no witness when it fails, so the pass
/// cannot be called on a created tree whose `setup` failed. [`reused`] makes one with no `setup`
/// call, and it is only for a tree the run-worktree entry reports as RE-ENTERED
/// (`Creation::Reused`), whose `setup` ran at its creation (N25). *(Corrected 2026-09-30, `s8g`
/// build, package R2FIX-8g: this doc read "Only [`set_up`] makes one".)*
#[derive(Debug)]
pub struct SetUp {
    _private: (),
}

/// **Step 1.** On a tree this invocation CREATED with a declared `setup_projects` row, run the
/// `setup` verb through the chosen toolchain; a refusal is returned and no witness is made. A
/// re-entered tree was set up when it was created (N25), and a tree with nothing declared has
/// nothing to set up.
///
/// # Errors
/// The `setup` verb's own [`StepError`].
pub fn set_up(
    tree: Tree,
    setup: Option<(&dyn Toolchain, &SetupRequest)>,
) -> Result<SetUp, StepError> {
    if let (Tree::Created, Some((toolchain, request))) = (tree, setup) {
        let inputs = std::collections::BTreeMap::from([(
            "projects".to_owned(),
            serde_json::Value::Array(request.projects.clone()),
        )]);
        toolchain.run_verb(&rk_ports::toolchain::VerbCall {
            verb: "setup",
            inputs: &inputs,
            bound: request.bound,
        })?;
    }
    Ok(SetUp { _private: () })
}

/// The witness for a RE-ENTERED tree: its `setup` ran when it was created (D21-43 N25: creation
/// refuses by name on a failed `setup`, so a tree that exists was set up).
#[must_use]
pub const fn reused() -> SetUp {
    SetUp { _private: () }
}

/// The adapter name `verify_completed` carries when no adapter detected (the named absence).
pub const UNDETECTED: &str = "undetected";

/// **The identity the composition root binds**: the adapter `detect()` chose (by the name `run`'s
/// selection loop returned, never the configured list's order) and the range the calls are made
/// over. Opaque to the kernel (AC-487).
#[must_use]
pub fn identity(adapter: Option<&str>, range: rk_types::verify::VerifyRange) -> VerifyIdentity {
    VerifyIdentity {
        adapter: adapter.unwrap_or(UNDETECTED).to_owned(),
        range,
    }
}

/// The whole-suite selection the pass requests.
#[must_use]
pub const fn whole_suite(reason: BasisReason) -> Selection {
    Selection::WholeSuite { reason }
}

/// One pass call's inputs.
#[derive(Debug, Clone, Copy)]
pub struct PassInputs<'a> {
    /// The item the rows are keyed to.
    pub item: &'a str,
    /// The stage the pass ran at, as data (the row's `stage` member); never compared.
    pub stage: &'a str,
    /// The owed instance: `TestReport`, keyed or not.
    pub instance: &'a Instance,
    /// The resolved changed-file set; `None` is unresolved (AC-320).
    pub changed: Option<&'a [String]>,
    /// The resolved selection policy; `None` is unresolved (AC-320).
    pub policy: Option<BasisReason>,
    /// The adapter `detect()` chose and the range, carried onto `verify_completed`.
    pub identity: &'a VerifyIdentity,
    /// The pass's bound.
    pub bound: StepBound,
}

/// Why a verify could not be decided (AC-331, AC-344).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unverifiable {
    /// The adapter's bound fired.
    Timeout,
    /// The OS killed the run, or it was cancelled.
    Killed,
    /// The run could not start (a missing binary), or produced no per-test record.
    CannotRun,
    /// The run's stream could not be read.
    Unreadable,
}

/// What the pass decided.
#[derive(Debug)]
pub enum Verdict {
    /// The suite passed; the `TestReport` was written at `path`.
    Green { passed: u32, path: String },
    /// The suite ran and did not pass: never a report of green.
    Failed { passed: u32, failed: u32 },
    /// The suite passed and step 6 refused the stage exit (AC-346): no report was written.
    /// *(dated 2026-09-29, `s8g` build, package S8G-P6-MUTATION-RESUME-DRIFT.)*
    ExitHeld { passed: u32, why: String },
    /// No outcome could be recorded.
    Unverifiable { why: Unverifiable, error: StepError },
}

/// Why the pass refused before, or instead of, a verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refused {
    /// AC-320: an input the pass needs is unresolved; no port call was made.
    Unresolved { what: &'static str },
    /// The owed instance is not this pass's kind.
    NotThePassKind { kind: String },
    /// The report failed the boundary (AC-324).
    Boundary(String),
    /// The `verify_completed` row or the `TestReport` could not be journalled or written.
    Write(String),
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unresolved { what } => write!(
                f,
                "the verify pass refused before any toolchain call: {what} is unresolved (AC-320)"
            ),
            Self::NotThePassKind { kind } => write!(
                f,
                "`{kind}` is owed by a pass, and the verify pass produces `{REQUIRED_KIND}` only"
            ),
            Self::Boundary(e) => write!(f, "the verify report failed the boundary: {e} (AC-324)"),
            Self::Write(e) => write!(f, "the verify pass could not record its result: {e}"),
        }
    }
}

impl Refused {
    /// The refusal as the `StepError` AC-320 names, for the exit table's one mapping.
    #[must_use]
    pub fn step_error(&self) -> StepError {
        StepError::Unsupported {
            capability: self.to_string(),
        }
    }
}

/// Where the pass writes its `TestReport`: `rk-store`'s writer on the product path.
pub trait ReportWriter {
    /// Write one kind's value, keyed when `key` is `Some`; the written path.
    ///
    /// # Errors
    /// The writer's refusal, rendered.
    fn write(
        &self,
        kind: &str,
        key: Option<&str>,
        value: &serde_json::Value,
    ) -> Result<String, String>;
}

impl ReportWriter for rk_store::render::ArtifactWriter {
    fn write(
        &self,
        kind: &str,
        key: Option<&str>,
        value: &serde_json::Value,
    ) -> Result<String, String> {
        let written = match key {
            Some(key) => self.write_keyed(kind, key, value),
            None => Self::write(self, kind, value),
        };
        written.map(|w| w.path).map_err(|e| format!("{e:?}"))
    }
}

/// **AC-331 / AC-344's mapping** from the port's `Err` to why the verify is undecided.
#[must_use]
pub fn unverifiable(error: &StepError) -> Unverifiable {
    match error {
        StepError::Timeout { .. } => Unverifiable::Timeout,
        StepError::Cancelled { .. } => Unverifiable::Killed,
        StepError::Transport {
            kind: TransportKind::Io,
            detail,
        } if detail.starts_with(OS_KILLED) => Unverifiable::Killed,
        StepError::Transport {
            kind: TransportKind::Io | TransportKind::Decode,
            ..
        } => Unverifiable::Unreadable,
        // A spawn failure (a missing binary), a non-zero exit with no per-test record, and every
        // refusal by name: the run could not be made (AC-344's `cannot_run`).
        StepError::Transport {
            kind: TransportKind::Spawn,
            ..
        }
        | StepError::NonZero { .. }
        | StepError::Schema { .. }
        | StepError::RateLimited { .. }
        | StepError::NotAdopted { .. }
        | StepError::NotIsolated { .. }
        | StepError::Platform { .. }
        | StepError::Unsupported { .. }
        | StepError::Overage { .. } => Unverifiable::CannotRun,
    }
}

/// The detail prefix both toolchain adapters put on an OS kill's `Transport { Io }` (record T).
pub const OS_KILLED: &str = "os-killed:";

/// **The pass, steps 2-5** (step 1 is [`set_up`]'s witness; step 7 is the caller's re-entry), with
/// no step 6: every green run is written.
///
/// # Errors
/// As [`run_gated`].
pub fn run(
    toolchain: &dyn Toolchain,
    tree: &SetUp,
    inputs: &PassInputs<'_>,
    append: &mut dyn FnMut(Event) -> Result<(), String>,
    writer: &dyn ReportWriter,
) -> Result<Verdict, Refused> {
    run_gated(toolchain, tree, inputs, append, writer, &mut |_| {
        StepSix::Permitted
    })
}

/// What §8g.2 step 6 (the AC ledger, the stage-exit check and the mutation harness) decided for a
/// green whole suite, before the `TestReport` is written. *(dated 2026-09-29, `s8g` build, package
/// S8G-P6-MUTATION-RESUME-DRIFT.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepSix {
    /// Every criterion permits the exit (AC-346): the report is written.
    Permitted,
    /// The exit is refused, named: no report is written, so the stage stays held (D21-120).
    Held(String),
}

/// **The pass, steps 2-6** (step 1 is [`set_up`]'s witness; step 7 is the caller's re-entry).
/// Step 6 runs after the report passed the boundary and before the ONE write: a refused stage
/// exit writes no `TestReport` (§8g.2 step 6; AC-346). *(dated 2026-09-29, `s8g` build, package
/// S8G-P6-MUTATION-RESUME-DRIFT.)*
///
/// # Errors
/// [`Refused`]: an unresolved input (before any port call), another pass kind, a report that fails
/// the boundary, or a row or write the store refused.
pub fn run_gated(
    toolchain: &dyn Toolchain,
    tree: &SetUp,
    inputs: &PassInputs<'_>,
    append: &mut dyn FnMut(Event) -> Result<(), String>,
    writer: &dyn ReportWriter,
    step_six: &mut dyn FnMut(&rk_ports::toolchain::TestOutcome) -> StepSix,
) -> Result<Verdict, Refused> {
    let _set_up: &SetUp = tree;
    if inputs.instance.kind != REQUIRED_KIND {
        return Err(Refused::NotThePassKind {
            kind: inputs.instance.kind.clone(),
        });
    }
    // ---- (2) AC-320: refused before any port call, with no row.
    let Some(changed) = inputs.changed else {
        return Err(Refused::Unresolved {
            what: "the changed-file set",
        });
    };
    let Some(policy) = inputs.policy else {
        return Err(Refused::Unresolved {
            what: "the selection policy",
        });
    };

    // ---- (3) the kernel makes the one port call and journals `verify_completed`.
    let selection = whole_suite(policy);
    let called = rk_kernel::verify::runner::run_test(
        toolchain,
        &rk_kernel::verify::runner::TestCall {
            item: inputs.item,
            selection: &selection,
            bound: inputs.bound,
            origin: rk_kernel::verify::runner::TestOrigin::Pass,
            stage: inputs.stage,
            identity: inputs.identity,
            paths: changed,
        },
        append,
    );

    // ---- (4) the outcome.
    let outcome = match called {
        Ok(outcome) => outcome,
        // The runner's own refusal to journal a completed call is a write failure, not a verdict.
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail,
        }) if detail.starts_with(ROW_REFUSED) => {
            return Err(Refused::Write(detail));
        }
        Err(error) => {
            return Ok(Verdict::Unverifiable {
                why: unverifiable(&error),
                error,
            });
        }
    };
    if outcome.status != TestStatus::Passing {
        // `failed > 0`, or nothing executed (AC-332): never a report of green, and no write.
        return Ok(Verdict::Failed {
            passed: outcome.passed,
            failed: outcome.failed,
        });
    }

    // ---- (5) the boundary, the comparator, and the ONE write.
    let report_json = serde_json::to_value(&outcome.report)
        .map_err(|e| Refused::Boundary(format!("the report does not serialise: {e}")))?;
    let report = rk_kernel::verify::report_at_boundary(report_json)
        .map_err(|e| Refused::Boundary(e.to_string()))?;
    let reclassified = rk_kernel::verify::comparator::reclassify(
        &rk_kernel::verify::comparator::Claim {
            run_id: format!("{}:{}", inputs.item, inputs.stage),
            basis: report.basis,
            reason: report.basis_reason,
            executed: report.selected_test_count,
            changed_files: u32::try_from(changed.len()).unwrap_or(u32::MAX),
            // The instance's toolchain element, or none: never the adapter name (ADR-0073).
            toolchain: inputs.instance.key.clone().unwrap_or_default(),
            toolchain_version: report.toolchain_version.clone(),
            argv: report.argv.clone(),
            policy: rk_kernel::verify::comparator::SelectionPolicy::EscalateToWholeSuite,
        },
        None,
    );
    let mut value = serde_json::to_value(&outcome)
        .map_err(|e| Refused::Boundary(format!("the outcome does not serialise: {e}")))?;
    if let Some(members) = value.as_object_mut() {
        members.insert("comparator".to_owned(), reclassified.pair());
        if let Some(key) = &inputs.instance.key {
            members.insert(
                "toolchain".to_owned(),
                serde_json::Value::String(key.clone()),
            );
        }
    }
    // ---- (6) the AC ledger and the stage-exit check, before the write.
    if let StepSix::Held(why) = step_six(&outcome) {
        return Ok(Verdict::ExitHeld {
            passed: outcome.passed,
            why,
        });
    }
    let path = writer
        .write(REQUIRED_KIND, inputs.instance.key.as_deref(), &value)
        .map_err(Refused::Write)?;
    Ok(Verdict::Green {
        passed: outcome.passed,
        path,
    })
}

/// The detail prefix `run_test` puts on a completed call whose row the store refused.
const ROW_REFUSED: &str = "the verify_completed row could not be appended";

// ------------------------------------------------------------------------ step 6: the harness
//
// *(dated 2026-09-29, `s8g` build, package S8G-P6-MUTATION-RESUME-DRIFT; `s8g` §8g.2 step 6 and
// §8g.5; R1 = ADR-0191 decision 11, proposed; ADR-0104 decision 1.)*

/// Where the mutation harness runs: the wrapper, a worktree of the repository to run `git
/// worktree` from, the item's derived trees (the harness uses `tree.mutation` and never
/// `tree.path`), and the candidate sha the ephemeral tree is created at.
#[derive(Debug, Clone, Copy)]
pub struct MutationSite<'a> {
    pub exec: &'a rk_git::git_exec::GitExec,
    pub repo: &'a rk_git::path::WorktreePath,
    pub tree: &'a rk_git::worktree::RunTree,
    pub candidate: &'a str,
}

/// One harness run's row context: the item, the stage (as data), the identity and the bound.
#[derive(Debug, Clone, Copy)]
pub struct MutationCall<'a> {
    pub item: &'a str,
    pub stage: &'a str,
    pub identity: &'a VerifyIdentity,
    pub bound: StepBound,
}

/// A `Toolchain` rooted at a path: how the harness gets one over its ephemeral tree.
pub type ToolchainAt<'a> =
    &'a dyn Fn(&std::path::Path) -> Result<std::sync::Arc<dyn Toolchain>, String>;

/// What the harness recorded for one criterion's declared mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationEvidence {
    /// The ephemeral tree the patch was applied in, and removed from.
    pub tree: std::path::PathBuf,
    /// What the harness observed, when the suite completed.
    pub run: Option<rk_kernel::verify::flip::MutationRun>,
    /// The flip predicate's answer over `run`; `None` when there is no run to judge.
    pub outcome: Option<rk_kernel::verify::flip::FlipOutcome>,
    /// Why there is no run, or what refused, named.
    pub note: Option<String>,
}

/// The repo-relative paths a unified diff touches: each `+++ b/<path>`, or the `--- a/<path>` of a
/// deletion (`+++ /dev/null`). The mutated REGION the harness re-reads (AC-338).
#[must_use]
pub fn patched_paths(patch: &str) -> Vec<String> {
    let mut out: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut last_old: Option<String> = None;
    for line in patch.lines() {
        if let Some(old) = line.strip_prefix("--- ") {
            last_old = old.trim().strip_prefix("a/").map(str::to_owned);
        } else if let Some(new) = line.strip_prefix("+++ ") {
            match new.trim().strip_prefix("b/") {
                Some(path) => {
                    out.insert(path.to_owned());
                }
                None => {
                    if let Some(old) = last_old.take() {
                        out.insert(old);
                    }
                }
            }
        }
    }
    out.into_iter().collect()
}

/// The content hash of the mutated region in `tree`: each path's bytes' digest, or `absent`.
fn region_hash(tree: &std::path::Path, region: &[String]) -> Vec<(String, String)> {
    region
        .iter()
        .map(|p| {
            let digest = std::fs::read(tree.join(p)).map_or_else(
                |_| "absent".to_owned(),
                |bytes| rk_types::digest::sha256(&bytes).as_str().to_owned(),
            );
            (p.clone(), digest)
        })
        .collect()
}

/// Remove the ephemeral tree: `git worktree remove --force`, then the directory. **Called on every
/// path** (ADR-0162 decision 10's precedent): the tree is scratch, and the revert ADR-0104 decision
/// 1 names is its removal. A failure is ignored here and read by the caller's absence check.
fn remove_mutation_tree(site: &MutationSite<'_>) {
    let path = &site.tree.mutation;
    let _ = site.exec.run(
        rk_git::git_exec::GitOp::WorktreeRemove,
        site.repo,
        [
            std::ffi::OsStr::new("worktree"),
            std::ffi::OsStr::new("remove"),
            std::ffi::OsStr::new("--force"),
            path.as_os_str(),
        ],
    );
    let _ = std::fs::remove_dir_all(path);
}

/// Whether `path` is a worktree git has registered.
fn registered(site: &MutationSite<'_>, path: &std::path::Path) -> bool {
    let fold = |p: &std::path::Path| {
        p.display()
            .to_string()
            .replace('\\', "/")
            .trim_end_matches('/')
            .to_lowercase()
    };
    rk_git::worktree::registered_worktrees(site.exec, std::path::Path::new(site.repo.as_str()))
        .map(|all| all.iter().any(|p| fold(p) == fold(path)))
        .unwrap_or(false)
}

/// **ADR-0104 decision 1's harness** (`s8g` §8g.5; R1 = ADR-0191 decision 11, proposed): *"The
/// harness applies the patch at the candidate SHA, runs the suite, reverts, and records the
/// outcome. It is deterministic and dispatches nothing."*
///
/// 1. The ephemeral tree is `site.tree.mutation` — `<main>/.redkiln/worktrees/<item>.mut`, the one
///    derivation's (`rk_git::worktree::derive`) — created detached at `site.candidate`, never the
///    run-owned tree (`site.tree.path`), which the harness does not touch (AC-339). A leftover of it
///    is removed only when it is a registered tree carrying the `RK_BASE` this harness writes;
///    anything else there is kept and the run refused by name.
/// 2. The patch is applied there through `rk_git::mutation::apply`, the one `GitOp::Apply` site
///    (R3 = ADR-0193), from this composition-root pass — the capability R1 decision 11 moves ADR-0162's
///    patch clause for, and no `-p` child. A patch that does not apply is
///    `Unverifiable { patch_did_not_apply }`, and no suite runs.
/// 3. The declared `suite` runs through `rk_kernel::verify::runner::run_test` (the one
///    `Toolchain::test` site, journalling `verify_completed` with `origin = "mutation/<ac>"`), over
///    a `Toolchain` rooted in the ephemeral tree.
/// 4. **AC-338**: the mutated region (every path the patch touches) is re-read after the suite and
///    its content hash compared with the one the applied patch left; a mutation reverted or
///    restored before the suite finished is `still_present = false`, which the flip predicate
///    reads as `Unverifiable`, never `mutation-survived` and never `pinned`.
/// 5. The tree is removed on every path, a refusal included.
///
/// The observation is at the declared suite's granularity: the port reports counts, not which
/// assertion failed, so `assertion_flipped` is `None` and a red suite is `RedElsewhere` — `pinned`
/// is not reachable through this harness until a port reports the named assertion (AC-334 stays
/// ARMED, `rk_kernel::verify::flip`'s own note).
#[must_use]
pub fn mutate(
    site: &MutationSite<'_>,
    ac: &str,
    patch: &str,
    suite: &str,
    toolchain_at: ToolchainAt<'_>,
    call: &MutationCall<'_>,
    append: &mut dyn FnMut(Event) -> Result<(), String>,
) -> MutationEvidence {
    use rk_kernel::verify::flip::{FlipOutcome, FlipUnverifiable, MutationRun, flip_predicate};

    let path = site.tree.mutation.clone();
    let refused = |note: String| MutationEvidence {
        tree: path.clone(),
        run: None,
        outcome: None,
        note: Some(note),
    };
    if path == site.tree.path {
        return refused("the mutation tree derived to the run-owned tree; refused".to_owned());
    }

    // ---- (1) the ephemeral tree, a leftover removed only when this harness made it.
    if std::fs::symlink_metadata(&path).is_ok() {
        let ours = registered(site, &path)
            && matches!(
                rk_git::worktree::read_rk_base(site.exec, &path),
                Ok(Some(base)) if base.item == call.item
            );
        if !ours {
            return refused(format!(
                "a leftover at {} carries no RK_BASE this harness wrote for `{}`; it is kept and \
                 the mutation run refused",
                path.display(),
                call.item
            ));
        }
        remove_mutation_tree(site);
    }
    if let Some(parent) = path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        return refused(format!("{}: {e}", parent.display()));
    }
    let added = site.exec.run(
        rk_git::git_exec::GitOp::WorktreeAdd,
        site.repo,
        [
            std::ffi::OsStr::new("worktree"),
            std::ffi::OsStr::new("add"),
            std::ffi::OsStr::new("--detach"),
            path.as_os_str(),
            std::ffi::OsStr::new(site.candidate),
        ],
    );
    if let Err(e) = added {
        remove_mutation_tree(site);
        return refused(format!("the mutation tree could not be created: {e:?}"));
    }

    // ---- (2)-(4), each fallible; the removal below runs whatever they returned.
    let evidence = (|| -> MutationEvidence {
        if let Err(e) = rk_git::worktree::write_rk_base(
            site.exec,
            &path,
            &rk_git::worktree::RkBase {
                fork: site.candidate.to_owned(),
                item: call.item.to_owned(),
            },
        ) {
            return refused(format!(
                "the mutation tree's RK_BASE could not be written: {e:?}"
            ));
        }
        let mut journal = rk_git::gitops::VecJournal::default();
        let applied = {
            let mut ops = rk_git::gitops::GitOps::new(
                site.exec.clone(),
                &mut journal,
                rk_git::gitops::Mode::Live,
            );
            rk_git::mutation::apply(&mut ops, site.tree, patch.as_bytes())
        };
        if let Err(e) = applied {
            return MutationEvidence {
                tree: path.clone(),
                run: Some(MutationRun {
                    applied: false,
                    suite_green: false,
                    assertion_flipped: None,
                    still_present: false,
                }),
                outcome: Some(FlipOutcome::Unverifiable(
                    FlipUnverifiable::PatchDidNotApply,
                )),
                note: Some(format!("the declared patch did not apply: {e:?}")),
            };
        }
        let region = patched_paths(patch);
        let applied_hash = region_hash(&path, &region);

        let toolchain = match toolchain_at(&path) {
            Ok(toolchain) => toolchain,
            Err(e) => return refused(format!("no toolchain over the mutation tree: {e}")),
        };
        let selection = Selection::Filter(suite.split_whitespace().map(str::to_owned).collect());
        let outcome = rk_kernel::verify::runner::run_test(
            toolchain.as_ref(),
            &rk_kernel::verify::runner::TestCall {
                item: call.item,
                selection: &selection,
                bound: call.bound,
                origin: rk_kernel::verify::runner::TestOrigin::Mutation { ac },
                stage: call.stage,
                identity: call.identity,
                paths: &region,
            },
            append,
        );
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(e) => return refused(format!("the declared suite did not complete: {e:?}")),
        };

        // ---- (4) AC-338: the region re-read after the suite.
        let still_present = region_hash(&path, &region) == applied_hash;
        let run = MutationRun {
            applied: true,
            suite_green: outcome.status == TestStatus::Passing,
            assertion_flipped: None,
            still_present,
        };
        MutationEvidence {
            tree: path.clone(),
            outcome: Some(flip_predicate(&run)),
            run: Some(run),
            note: None,
        }
    })();

    // ---- (5) the revert is the removal, on every path.
    remove_mutation_tree(site);
    if std::fs::symlink_metadata(&path).is_ok() || registered(site, &path) {
        return MutationEvidence {
            note: Some(format!(
                "the mutation tree {} outlived its removal",
                path.display()
            )),
            ..evidence
        };
    }
    evidence
}

/// The kind whose sidecar carries the item's criteria.
pub const SPEC_KIND: &str = "Spec";

/// **The item's acceptance criteria**, off the committed `Spec` sidecar in the run-owned tree
/// (`<tree>/.redkiln/artifacts/<item>/Spec.json`; `s8g` §8g.3: *"The pass reads the item's ACs off
/// the committed `Spec` artifact"*), under the member `acceptance_criteria`, each through
/// `AcceptanceCriterion::ingest`, the type's one door. *(dated 2026-09-29, `s8g` build, package
/// S8G-P6-MUTATION-RESUME-DRIFT.)*
///
/// No sidecar is no criteria: the ledger is then empty and permits the exit, which is what every
/// lane without a specifying stage reads (a build choice, stated and reversible). A sidecar that
/// does not parse, lacks the member, or carries a criterion the door refuses is an error.
///
/// # Errors
/// The reason, rendered.
pub fn criteria_of(
    tree: &std::path::Path,
    item: &str,
) -> Result<Vec<rk_types::acceptance::AcceptanceCriterion>, String> {
    let dir = rk_store::render::ManagedDir::of(tree, item);
    let name = rk_store::render::sidecar_name(SPEC_KIND);
    let Some(bytes) = dir.read(&name) else {
        return Ok(Vec::new());
    };
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("{} does not parse: {e}", dir.rel(&name)))?;
    let Some(list) = value
        .get("acceptance_criteria")
        .and_then(serde_json::Value::as_array)
    else {
        return Err(format!(
            "{} carries no `acceptance_criteria` list",
            dir.rel(&name)
        ));
    };
    list.iter()
        .map(|c| {
            rk_types::acceptance::AcceptanceCriterion::ingest(c.clone())
                .map_err(|e| format!("{}: {e}", dir.rel(&name)))
        })
        .collect()
}
