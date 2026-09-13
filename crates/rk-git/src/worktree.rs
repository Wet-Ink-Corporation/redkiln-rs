//! Worktree lifecycle and ownership (ADR-0105 clauses 6 and 8; AC-529).
//!
//! **Ownership is resolved from the store, never from cwd.** ADR-0018 said the kernel *"refuses to
//! write when the current worktree's HEAD is not the run's branch"*; ADR-0105 clause 8 narrows
//! "current worktree" to *"the worktree the store owns for the run"*. The difference is the whole
//! criterion: a process can be `cd`'d anywhere, and a rule keyed on cwd would authorise a write
//! from whichever directory the operator happened to be standing in.
//!
//! That is why [`RunOwnership`] is a **parameter** and not something this module looks up. There is
//! no call to `std::env::current_dir` in this crate, and AC-529's third instrument is the test that
//! says so — the one a naive implementation passes by accident, because on the happy path cwd and
//! the owned worktree are usually the same directory.
//!
//! **Worktrees are re-created per run, never reused** (clause 6). A failed `worktree remove` — on
//! Windows typically a handle no `Stop-Process` releases, Halt 6 — is a typed error and a `doctor`
//! orphans row, never a retry into a reused tree, because reusing the tree corrupts the next arm.

use std::path::{Path, PathBuf};

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;

/// Which worktree and branch the **store** says this run owns.
///
/// Supplied by `rk-store` (§6.3). Passed in rather than looked up, so that no code path here can
/// fall back to cwd.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOwnership {
    pub worktree: PathBuf,
    pub branch: String,
}

/// A worktree a tier-1 write has been authorised against.
///
/// Like `Reconciled<State>`, the private field is the mechanism: it can only be produced by
/// [`authorise_tier1_write`], so "was this write checked?" is answered by the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedWorktree {
    path: WorktreePath,
    branch: String,
}

impl OwnedWorktree {
    #[must_use]
    pub const fn path(&self) -> &WorktreePath {
        &self.path
    }
    #[must_use]
    pub fn branch(&self) -> &str {
        &self.branch
    }
}

/// Authorise a tier-1 write against `target`, given what the store says the run owns.
///
/// Refuses when the target is not the run-owned worktree, **or** when its `HEAD` is not the run's
/// branch. The refusal names all four values, so the operator sees what was attempted and what was
/// expected rather than a bare denial (AC-529).
///
/// # Errors
/// [`GitError::ForeignWorktree`] on either refusal; propagates wrapper errors from reading `HEAD`.
pub fn authorise_tier1_write(
    exec: &GitExec,
    target: &Path,
    ownership: &RunOwnership,
) -> Result<OwnedWorktree, GitError> {
    let target_path = WorktreePath::new(target)?;
    let owned_path = WorktreePath::new(&ownership.worktree)?;

    let head = current_branch(exec, &target_path)?;

    // Compare on the normalised form: `D:\x` and `D:/x` are the same worktree, and a comparator
    // that said otherwise would refuse valid work.
    if target_path != owned_path || head != ownership.branch {
        return Err(GitError::ForeignWorktree {
            target_path: PathBuf::from(target_path.as_str()),
            target_branch: head,
            owned_path: PathBuf::from(owned_path.as_str()),
            owned_branch: ownership.branch.clone(),
        });
    }

    Ok(OwnedWorktree {
        path: target_path,
        branch: head,
    })
}

/// The branch `HEAD` currently points at, or `HEAD` when detached.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn current_branch(exec: &GitExec, at: &WorktreePath) -> Result<String, GitError> {
    let out = exec.run(GitOp::RevParse, at, ["rev-parse", "--abbrev-ref", "HEAD"])?;
    Ok(out.stdout_trimmed())
}

/// Create a run-owned worktree. **Re-created per run, never reused** (clause 6).
///
/// If `path` already exists this refuses rather than adopting it: an existing tree is either a
/// leftover from a killed run — which is a `doctor` orphans row, not a resource — or another run's,
/// and reusing either is what Halt 6 is about.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn add(
    exec: &GitExec,
    repo: &WorktreePath,
    path: &Path,
    branch: &str,
) -> Result<WorktreePath, GitError> {
    let target = WorktreePath::new(path)?;
    if path.exists() {
        return Err(GitError::WorktreeRemoveFailed {
            path: path.to_path_buf(),
            stderr: b"a worktree already exists at this path; worktrees are re-created per run, \
                      never reused (ADR-0105 clause 6)"
                .to_vec(),
        });
    }
    exec.run(
        GitOp::WorktreeAdd,
        repo,
        ["worktree", "add", target.as_str(), branch],
    )?;
    Ok(target)
}

/// Remove a run-owned worktree.
///
/// A failure here is **typed and surfaced**, never swallowed and never retried into a reuse. The
/// caller records a `doctor` orphans row; S-CANCEL-WIN's fixture is the regression for the Windows
/// handle case, and it runs at 7c.
///
/// # Errors
/// [`GitError::WorktreeRemoveFailed`] carrying the path and git's stderr.
pub fn remove(exec: &GitExec, repo: &WorktreePath, path: &Path) -> Result<(), GitError> {
    let target = WorktreePath::new(path)?;
    match exec.run(
        GitOp::WorktreeRemove,
        repo,
        ["worktree", "remove", target.as_str()],
    ) {
        Ok(_) => Ok(()),
        Err(GitError::NonZeroExit { stderr, .. }) => Err(GitError::WorktreeRemoveFailed {
            path: path.to_path_buf(),
            stderr,
        }),
        Err(other) => Err(other),
    }
}
