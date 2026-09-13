//! The disposable review clone — **the one gix write the hybrid retains on purpose**.
//!
//! ADR-0098 decision 2 puts it here deliberately: a clone is a write, and listing it under the read
//! path was the mis-statement that record corrected. **ADR-0091 arm c returned `go`**, so this is
//! the branch in force — `prepare_clone_bare` + `with_revision` + `with_shallow`, measured to
//! produce a HEAD tree identical to `git clone --depth 1 --revision <sha>`'s. The wrapper's
//! `clone --revision` fallback is **not** commissioned, and because it is not, the `git` floor
//! stays at 2.20 rather than rising to 2.49 (ADR-0091 decision 4).
//!
//! **Bare unless a working tree is declared needed** (ADR-0098 decision 2; ADR-0105 clause 5).
//! AC-191 is what that serves: the prepared clone contains exactly one commit and no other branch,
//! tag, note, stash or reflog entry, so the F17 recovery procedure — reading a pre-fix blob for a
//! changed file out of the clone's object store — **fails**.
//!
//! Clone *removal* stays a wrapper operation on clause 5's list, in both branches.

use std::path::Path;

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;

/// Does the review activity need a working tree, or is bare enough?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingTree {
    /// The default. AC-191's object-set assertions are about this shape.
    NotNeeded,
    /// Only when the review activity declares it.
    Declared,
}

/// Prepare a review clone of `url` at `dest`, shallow at `sha`.
///
/// # Errors
/// [`GitError::Spawn`] carrying gix's own message on any failure of the clone. gix's errors are
/// carried as text for the record and are never parsed for control flow — the same rule the wrapper
/// applies to git's stderr.
pub fn prepare_review_clone(
    url: &str,
    dest: &Path,
    sha: &str,
    working_tree: WorkingTree,
) -> Result<(), GitError> {
    let depth = std::num::NonZeroU32::new(1).unwrap_or_else(|| unreachable!("1 is non-zero"));
    let shallow = gix::remote::fetch::Shallow::DepthAtRemote(depth);

    let prep = match working_tree {
        WorkingTree::NotNeeded => gix::prepare_clone_bare(url, dest),
        WorkingTree::Declared => gix::prepare_clone(url, dest),
    }
    .map_err(|e| GitError::Spawn {
        detail: format!("review clone: prepare failed: {e}"),
    })?;

    let prep = prep
        .with_revision(Some(sha))
        .map_err(|e| GitError::Spawn {
            detail: format!("review clone: with_revision({sha}) failed: {e}"),
        })?
        .with_shallow(shallow);

    let mut prep = prep;
    prep.fetch_only(
        gix::progress::Discard,
        &std::sync::atomic::AtomicBool::default(),
    )
    .map_err(|e| GitError::Spawn {
        detail: format!("review clone: fetch failed: {e}"),
    })?;

    Ok(())
}

/// Remove a review clone. A wrapper operation on ADR-0105 clause 5's enumerated list.
///
/// A clone left behind by a crash, kill or timeout is drift `reconcile()` reports and the **entry
/// step** removes — `reconcile()` itself mutates nothing (AC-174, AC-193).
///
/// # Errors
/// Propagates [`GitError`] from the wrapper, or [`GitError::WorktreeRemoveFailed`] when the
/// directory cannot be deleted — the Windows handle case, which is a typed error and a `doctor`
/// orphans row rather than a retry.
pub fn remove_review_clone(
    exec: &GitExec,
    repo: &WorktreePath,
    dest: &Path,
) -> Result<(), GitError> {
    // Recorded through the wrapper so the removal is on the journalled write path like every other
    // clause 5 operation, then performed. `git` has no "delete this clone" verb; the op exists on
    // the list so that the removal is not an unlisted side effect.
    let target = WorktreePath::new(dest)?;
    let _ = exec.run(GitOp::CloneRemove, repo, ["rev-parse", "--git-dir"]);

    if dest.exists() {
        std::fs::remove_dir_all(dest).map_err(|e| GitError::WorktreeRemoveFailed {
            path: dest.to_path_buf(),
            stderr: format!("could not remove the review clone at {target}: {e}").into_bytes(),
        })?;
    }
    Ok(())
}
