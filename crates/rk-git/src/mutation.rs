//! **The mutation patch's one write** — `s8g`'s write-set record (R3 = ADR-0193 decision 2,
//! proposed), for the verify pass's mutation harness (R1 = ADR-0191 decision 11, proposed; ADR-0104
//! decision 1: *"The harness applies the patch at the candidate SHA, runs the suite, reverts, and
//! records the outcome."*). *(Added 2026-09-29, `s8g` build, package S8G-P3-KERNEL-GIT.)*
//!
//! [`apply`] is the one code site that names [`GitOp::Apply`]. Its working directory is not a
//! parameter: it is the [`RunTree::mutation`] path the one derivation (`worktree::derive`) gives,
//! `<main>/.redkiln/worktrees/<item>.mut`, beside the review reference tree and never the run-owned
//! tree or the main worktree. The revert is the tree's removal, which is the harness's, on every
//! path. The patch goes in on stdin, so no file is written into any tree to carry it.

use crate::error::GitError;
use crate::git_exec::GitOp;
use crate::gitops::GitOps;
use crate::path::WorktreePath;
use crate::worktree::RunTree;

/// Apply a criterion's declared `mutation` patch in `tree`'s mutation tree, through the journalled
/// seam (`git apply --whitespace=nowarn -`).
///
/// # Errors
/// The seam's [`GitError`]: [`GitError::NonZeroExit`] when git does not apply the patch there, and
/// [`GitError::ExtendedLengthPath`] for a path the boundary refuses.
pub fn apply(ops: &mut GitOps<'_>, tree: &RunTree, patch: &[u8]) -> Result<(), GitError> {
    let cwd = WorktreePath::new(&tree.mutation)?;
    ops.run_with_stdin(
        GitOp::Apply,
        &cwd,
        &["apply", "--whitespace=nowarn", "-"],
        patch,
    )?;
    Ok(())
}
