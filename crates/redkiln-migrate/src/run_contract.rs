//! The run contract, in `s7d` §7d.11's order — and RT-53's exit values.
//!
//! 1. [`rk_git::version::assert_at_startup`] — ADR-0091 decision 4's `>= 2.20` floor.
//! 2. H-3's bootstrap: a [`rk_git::git_exec::GitExec`] whose `hooks_empty` is a `tempfile::TempDir`,
//!    through which `rev-parse --show-toplevel` and the one `rev-parse --git-common-dir` this
//!    crate's worktree path needs are read — **because it may not depend on `rk-adapter-sqlite`**
//!    (AC-466), which is the ADR-0091 divergence §7d.18 records as RAISED — then the working
//!    `GitExec` under `<git-common-dir>/redkiln/hooks-empty/`.
//! 3. `GitExec::run(GitOp::Status, target, ["status", "--porcelain"])` — non-empty is a refusal
//!    **before any write**, naming every dirty path (AC-522). No porcelain helper exists in
//!    `rk-git` [measured 2026-09-21: `GitOp::Status` has no call site in the workspace]; this is the
//!    one call site.
//! 4. The branch `rk-migrate/<target-HEAD-short>` and the worktree
//!    `<git-common-dir>/redkiln/migrate-wt/` — **re-created, never reused** (ADR-0105 clause 6).
//! 5. The read: [`crate::lift::run`].
//! 6. `--dry-run` stops here, printing the report and nothing else to stdout, exit `0`.
//! 7. The write phase into the worktree, then `report.json` beside it.
//! 8. `phase: written` on stdout (**D18-12**, the AC-450 kill seam — a print, never a flag).
//! 9. The no-op check, then **one** commit (AC-447, AC-448).
//!
//! # Why the worktree is re-created and never reused
//!
//! A killed run (`idempotent_across_kill`) leaves written, uncommitted files that `git worktree
//! remove` refuses — *"contains modified or untracked files, use --force to delete it"*, rc=128 —
//! and `rk_git::worktree::remove` passes no `--force`. `remove_dir_all` + `add` fails too
//! (*"missing but already registered worktree"*) and `GitOp` has no prune op. So the teardown is
//! the raw `GitExec::run(GitOp::WorktreeRemove, …, ["worktree", "remove", "--force", …])` form this
//! step already uses for `worktree add -b`. **No `rk-git` edit**: §7d.15's floor holds. The
//! `--force` is licensed only because the tree is a run-owned worktree, never the user's checkout.
//!
//! # Why the first run takes the raw `add -b` form
//!
//! `rk_git::worktree::add` runs `git worktree add <path> <branch>`, which needs an existing
//! commit-ish; on a first run `rk-migrate/<short>` does not exist yet and `GitOp` has no branch or
//! update-ref op. So the first run is
//! `GitExec::run(GitOp::WorktreeAdd, target, ["worktree", "add", "-b", &branch, wt, "HEAD"])`,
//! after the same `path.exists()` refusal `worktree::add` performs; later runs go through the
//! helper, which checks out the existing branch.
//!
//! **That is also why AC-450's tree-digest comparison can mean anything.**
//! [`rk_git::tree::TreeBasis::WorkingState`] seeds a temporary index from `read-tree HEAD` and then
//! takes `add -u` — **tracked modifications only** [measured 2026-09-21, `rk-git/src/tree.rs`]. On a
//! later run the worktree is checked out from the migration branch, whose HEAD already carries the
//! migrated layout, so every migrated file is tracked and the rewrite is exactly what `add -u`
//! sees. On a first run there is no prior migration commit and §7d.11 step (8) skips the comparison
//! by its own *"when one exists"*.
//!
//! **Disclosed limit**: a later run that would ADD a file absent from the prior migration commit is
//! untracked, so `add -u` never stages it and the run reads as a no-op. `legacy-mini` does not
//! exercise it and no test here claims otherwise.

use std::path::{Path, PathBuf};

/// The branch prefix every migration commit lands under. Never the target's own branch (AC-476's
/// AND clause, which AC-522 instruments).
pub const MIGRATION_BRANCH_PREFIX: &str = "rk-migrate/";

/// D18-12's kill seam: printed on stdout after the write phase and **before** the commit.
pub const PHASE_WRITTEN_MARKER: &str = "phase: written";

/// A refusal, before anything was written. RT-53 maps every one of these to exit `3`.
#[derive(Debug)]
pub enum Refusal {
    /// AC-522: the target's `git status --porcelain` was not empty. Names every dirty path.
    DirtyTarget { paths: Vec<String> },
    /// AC-447's AND: a run that would produce more than one commit aborts **before writing**.
    SecondWritePhase,
    /// AC-448's AND: a run that would write the new layout without emitting a report is refused.
    NoReport,
    /// ADR-0105 clause 6 / D18-11: `worktree::add` refuses an existing path.
    WorktreePathExists { path: PathBuf },
}

/// Anything that is not a [`Refusal`]. RT-53 maps these to exit `1`.
#[derive(Debug)]
pub enum MigrateError {
    /// The refusals above, before any write.
    Refused(Refusal),
    /// Any other error.
    Other(anyhow::Error),
}

/// What a completed run did, for the caller that maps it onto an exit value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Completed {
    /// `--dry-run`: the report went to stdout and nothing was written. Exit `0`.
    DryRun,
    /// AC-450's no-op — tree-digest equality with the prior migration commit's tree. The re-run's
    /// report went to stdout; **no second commit**. Exit `0`.
    NoOp,
    /// One commit, on the migration branch only, carrying the layout and the report. Exit `0`.
    Committed { commit: String, branch: String },
}

/// A seam the AC-447 test drives to request a second write phase, and the AC-448 test to disable
/// the report step.
///
/// It is a **parameter**, never a flag or an environment variable: AC-474 forbids any configuration
/// key, flag or environment variable that changes the write path, and a test seam that travelled
/// through argv or the environment would be exactly that. The binary constructs
/// [`Hooks::default`] and nothing else.
#[derive(Debug, Default, Clone, Copy)]
pub struct Hooks {
    /// AC-447's instrument: request a second write phase, which must abort before writing.
    pub request_second_write_phase: bool,
    /// AC-448's instrument: disable the report step, which must refuse before the commit.
    pub disable_report_step: bool,
}

/// Run the contract against `target`.
///
/// # Errors
/// [`MigrateError::Refused`] for every refusal that happens before a write (exit `3`);
/// [`MigrateError::Other`] for anything else (exit `1`).
pub fn run(target: &Path, dry_run: bool, hooks: Hooks) -> Result<Completed, MigrateError> {
    let _ = (target, dry_run, hooks);
    todo!("cycles 3-5: the bootstrap, the porcelain refusal, the worktree, the write, the commit")
}
