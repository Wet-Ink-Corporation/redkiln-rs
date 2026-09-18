//! Where the store is — `<git rev-parse --git-common-dir>/redkiln/store.sqlite3` (AC-370), the
//! common dir taken through `rk-git`'s one wrapper (RT-17; ADR-0077:70).
//!
//! **One store per repository, whichever worktree opens it** (AC-178, AC-370). `rk-store` fixes
//! the layout under the common dir (`rk_store::conn::store_path`) and takes `common_dir: &Path`;
//! it never asks git. Nothing on the tree before this crate resolved a *linked* worktree to its
//! common dir — Step 6's AC-178 tests handed a temporary directory in as the common dir and
//! created no linked worktree (`s7c` §7c.13, critic gap 8) — so the derivation is this crate's,
//! and it is one `git` read: `git rev-parse --git-common-dir`. Not `--git-dir`: from a linked
//! worktree that answers the tree's own `.git/worktrees/<name>`, and a store keyed on it is a
//! store per worktree, which is exactly the stray file AC-370's `doctor` row is armed against.
//!
//! **The join rule** (`s7c` §7c.13). From a **main** worktree `--git-common-dir` prints the
//! cwd-relative `.git`; from a linked one it prints an absolute path. `--path-format=absolute`
//! would settle it, but it exists only from git 2.31 and `rk-git`'s floor is `GIT_FLOOR = (2, 20)`
//! (ADR-0091 decision 4), so a relative answer is joined onto the worktree path **here**, before
//! anything opens. Left unjoined, `Writer::open(".git")` would resolve against the process's
//! current directory — whatever directory that happens to be — and create a `.git/redkiln/`
//! there. The joined path is normalised through `WorktreePath` (forward slashes, no `\\?\`
//! prefix — ADR-0105 clause 6) so the same repository derives the same spelling from every
//! worktree.
//!
//! **What this module does not do.** It does not canonicalise (a `\\?\`-prefixed answer is the
//! F20/2 defect `rk_git::path` exists to refuse), does not read `.git` files itself (git is the
//! one reader of its own layout), and does not consult the process environment or `current_dir`
//! (the block's AC-217 ban; CI's `env::current_dir` rule).

use std::path::{Path, PathBuf};

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;

use crate::StoreOpenError;

/// The repository's common dir, as `git rev-parse --git-common-dir` answers it from `worktree`,
/// joined onto `worktree` when the answer is relative.
///
/// # Errors
/// [`StoreOpenError::Git`] when `rev-parse` cannot run or exits non-zero (`worktree` is not
/// inside a repository); [`StoreOpenError::Path`] when the joined path is one `rk-git` refuses.
pub fn common_dir(exec: &GitExec, worktree: &WorktreePath) -> Result<PathBuf, StoreOpenError> {
    let out = exec
        .run(GitOp::RevParse, worktree, ["rev-parse", "--git-common-dir"])
        .map_err(|error| StoreOpenError::Git {
            op: "rev-parse --git-common-dir",
            worktree: worktree.to_string(),
            error: Box::new(error),
        })?;
    let answer = PathBuf::from(out.stdout_trimmed());
    let joined = if answer.is_absolute() {
        answer
    } else {
        Path::new(worktree.as_str()).join(answer)
    };
    let normalised = WorktreePath::new(&joined).map_err(|error| StoreOpenError::Path {
        worktree: worktree.to_string(),
        error: Box::new(error),
    })?;
    Ok(PathBuf::from(normalised.as_str()))
}

/// The store file under `common_dir` — `rk-store`'s layout, restated here so the adapter has one
/// name for it.
#[must_use]
pub fn store_path(common_dir: &Path) -> PathBuf {
    rk_store::conn::store_path(common_dir)
}
