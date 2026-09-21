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

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;
use rk_git::tree::TreeBasis;

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
    // (1) The git floor, before anything else reads a repository.
    let toolchain = rk_git::version::assert_at_startup(target).map_err(other)?;

    // (2) H-3's bootstrap. The hooks directory must exist before the common dir is known, so it is
    // a `tempfile::TempDir` held for exactly as long as the two reads below need it.
    let boot_hooks = tempfile::tempdir()
        .map_err(|e| other_msg(format!("the bootstrap hooks directory: {e}")))?;
    let boot = GitExec::new(boot_hooks.path().to_path_buf(), toolchain).map_err(other)?;
    let target_wt = WorktreePath::new(target).map_err(other)?;

    // `rev-parse --git-common-dir` and not `rk-adapter-sqlite`'s resolver: AC-466 forbids that
    // edge, which is the ADR-0091 divergence §7d.18 records as RAISED.
    let common_dir = PathBuf::from(
        boot.run(
            GitOp::RevParse,
            &target_wt,
            ["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )
        .map_err(other)?
        .stdout_trimmed(),
    );

    let exec =
        GitExec::new(common_dir.join("redkiln").join("hooks-empty"), toolchain).map_err(other)?;
    drop(boot_hooks);

    // (3) The porcelain read — THE refusal, before any write. There is no porcelain helper in
    // `rk-git` [measured 2026-09-21: `GitOp::Status` has no other call site in the workspace];
    // this is the one call site, and it is a call site rather than a new helper because §7d.15's
    // floor forbids editing `rk-git`.
    let porcelain = exec
        .run(GitOp::Status, &target_wt, ["status", "--porcelain"])
        .map_err(other)?
        .stdout_trimmed();
    if !porcelain.is_empty() {
        return Err(MigrateError::Refused(Refusal::DirtyTarget {
            paths: porcelain.lines().map(|l| l.trim().to_owned()).collect(),
        }));
    }

    let head_short = exec
        .run(
            GitOp::RevParse,
            &target_wt,
            ["rev-parse", "--short", "HEAD"],
        )
        .map_err(other)?
        .stdout_trimmed();
    let branch = format!("{MIGRATION_BRANCH_PREFIX}{head_short}");
    let wt_path = common_dir.join("redkiln").join("migrate-wt");

    // The prior migration commit, read BEFORE the worktree moves anything (AC-450 step (8)).
    let prior = exec
        .run(
            GitOp::RevParse,
            &target_wt,
            ["rev-parse", "--verify", "--quiet", &branch],
        )
        .ok()
        .map(|o| o.stdout_trimmed())
        .filter(|s| !s.is_empty());

    // (4) The worktree — NOT on a dry run: §7d.11 is explicit that "the dry run runs steps 1, 2
    // and 4 only", so no worktree and no branch may exist after one.
    let worktree = if dry_run {
        None
    } else {
        Some(open_worktree(
            &exec,
            &target_wt,
            &wt_path,
            &branch,
            prior.is_some(),
        )?)
    };

    // (5) The read.
    let lifted = crate::lift::run(target, &mut crate::lift::Natural)
        .map_err(|e| other_msg(format!("the lift failed: {e:?}")))?;
    let report_bytes = rk_store::canon::canonical(
        &serde_json::to_value(&lifted.report)
            .map_err(|e| other_msg(format!("the report will not serialise: {e}")))?,
    )
    .map_err(|e| other_msg(format!("the report will not canonicalise: {e}")))?;

    // (6) `--dry-run` stops here: the JSON document on stdout and nothing else.
    if dry_run {
        print_report(&report_bytes);
        return Ok(Completed::DryRun);
    }
    let worktree = worktree.expect("a non-dry run opened its worktree");

    // (7) The write phase. AC-447's AND aborts BEFORE writing, not after.
    if hooks.request_second_write_phase {
        return Err(MigrateError::Refused(Refusal::SecondWritePhase));
    }
    write_phase(&wt_path, &lifted, &report_bytes, hooks.disable_report_step)?;

    // (8) D18-12's kill seam: a print, never a flag or an environment toggle.
    println!("{PHASE_WRITTEN_MARKER}");

    exec.run(GitOp::Add, &worktree, ["add", "-A"])
        .map_err(other)?;

    // (9) AC-450's no-op, defined as tree-digest equality with the prior migration commit's tree.
    if let Some(prior) = prior {
        let now = rk_git::tree::tree_hash(&exec, &worktree, &common_dir, &TreeBasis::WorkingState)
            .map_err(other)?;
        let then = rk_git::tree::tree_hash(
            &exec,
            &worktree,
            &common_dir,
            &TreeBasis::CheckpointCommit(prior),
        )
        .map_err(other)?;
        if now == then {
            print_report(&report_bytes);
            return Ok(Completed::NoOp);
        }
    }

    // AC-448's AND: the layout without a report is refused, and the refusal precedes the commit.
    if hooks.disable_report_step {
        return Err(MigrateError::Refused(Refusal::NoReport));
    }

    // (10) ONE commit, on the migration branch only.
    let message = format!(
        "redkiln-migrate: {} {}",
        lifted.report.corpus_sha, lifted.report.tool_version
    );
    let commit =
        rk_git::trailer::write::commit_with_trailers(&exec, &worktree, &message, &[], false)
            .map_err(other)?;

    Ok(Completed::Committed { commit, branch })
}

/// Open the migration worktree, **re-creating it and never reusing it** (ADR-0105 clause 6).
fn open_worktree(
    exec: &GitExec,
    target: &WorktreePath,
    wt_path: &Path,
    branch: &str,
    branch_exists: bool,
) -> Result<WorktreePath, MigrateError> {
    // A previous run's worktree is torn down first. `rk_git::worktree::remove` passes no `--force`
    // and a killed run leaves written, uncommitted files git refuses to remove without it
    // ("contains modified or untracked files"), so the teardown takes the same raw-`run` form this
    // step already uses for `worktree add -b`. NO `rk-git` EDIT — §7d.15's floor holds — and the
    // `--force` is licensed only because this tree is a run-owned worktree, never the user's.
    if wt_path.exists() {
        exec.run(
            GitOp::WorktreeRemove,
            target,
            [
                "worktree",
                "remove",
                "--force",
                &wt_path.to_string_lossy().replace('\\', "/"),
            ],
        )
        .map_err(other)?;
        if wt_path.exists() {
            return Err(MigrateError::Refused(Refusal::WorktreePathExists {
                path: wt_path.to_path_buf(),
            }));
        }
    }

    if branch_exists {
        // The helper checks out the EXISTING branch — whose HEAD already carries the migrated
        // layout, which is what makes step (9)'s `add -u` digest meaningful.
        rk_git::worktree::add(exec, target, wt_path, branch).map_err(other)
    } else {
        // A first run: the branch does not exist yet, and `worktree::add` shells
        // `git worktree add <path> <branch>`, which needs an existing commit-ish. `GitOp` has no
        // branch or update-ref op, so the raw form carries the `-b`.
        exec.run(
            GitOp::WorktreeAdd,
            target,
            [
                "worktree",
                "add",
                "-b",
                branch,
                &wt_path.to_string_lossy().replace('\\', "/"),
                "HEAD",
            ],
        )
        .map_err(other)?;
        WorktreePath::new(wt_path).map_err(other)
    }
}

/// Write the new layout into the worktree, and `report.json` beside it.
///
/// Each legacy file is REPLACED by its canonical `.json` artifact: the migration is a clean break
/// (AC-474), so leaving the old bytes beside the new ones would leave `template_sig` in the
/// migration commit's tree and make `migrated_artifacts_carry_schema_version_and_no_sig` a claim
/// about a subset rather than about the tree.
fn write_phase(
    wt_path: &Path,
    lifted: &crate::lift::Lifted,
    report_bytes: &[u8],
    disable_report_step: bool,
) -> Result<(), MigrateError> {
    for (dest, bytes) in &lifted.artifacts {
        let path = wt_path.join(dest);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| other_msg(format!("{}: {e}", parent.display())))?;
        }
        std::fs::write(&path, bytes).map_err(|e| other_msg(format!("{}: {e}", path.display())))?;
    }
    for file in &lifted.files {
        let legacy = wt_path.join(&file.source);
        if legacy.exists() {
            std::fs::remove_file(&legacy)
                .map_err(|e| other_msg(format!("{}: {e}", legacy.display())))?;
        }
    }
    if !disable_report_step {
        std::fs::write(wt_path.join("report.json"), report_bytes)
            .map_err(|e| other_msg(format!("report.json: {e}")))?;
    }
    Ok(())
}

/// The report, on stdout, as one JSON document and nothing else.
fn print_report(report_bytes: &[u8]) {
    println!("{}", String::from_utf8_lossy(report_bytes));
}

fn other(e: rk_git::error::GitError) -> MigrateError {
    MigrateError::Other(anyhow::anyhow!("{e:?}"))
}

fn other_msg(detail: String) -> MigrateError {
    MigrateError::Other(anyhow::anyhow!(detail))
}
