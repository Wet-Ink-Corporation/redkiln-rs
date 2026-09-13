//! `tree_hash` — **one algorithm, written once** (ADR-0105 clause 4).
//!
//! The clause is specific, and the specificity is the point: the same derivation serves AC-245's
//! `gate_opened`, the answer record, and AC-254's diff. A second derivation anywhere would let a
//! gate open against one tree and be answered against another, which is precisely the condition
//! AC-176 declares void.
//!
//! Two cases, and only two:
//!
//! * **A checkpoint commit is due** — the recorded tree is that commit's tree.
//! * **Otherwise** — `git write-tree` on a **temporary index** built from the current index plus
//!   tracked modifications. **Untracked files are excluded**, by construction rather than by
//!   filtering: the temporary index is seeded from `HEAD` and updated with `add -u`, and `-u`
//!   touches only paths git already tracks. There is no `add -A` in this module, and that absence
//!   is what makes the exclusion true.
//!
//! The temporary index lives under the common dir, never `std::env::temp_dir()`: a run-owned
//! location keeps it inside the tree the store owns and out of a shared world-writable directory.

use std::ffi::OsString;
use std::path::Path;

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;

/// A 40-hex tree object id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeHash(String);

impl TreeHash {
    /// # Errors
    /// [`GitError::VersionUnparsable`] is reused as the shape error when `s` is not 40 hex
    /// characters — a tree hash that is not one is not a tree hash.
    pub fn from_hex(s: &str) -> Result<Self, GitError> {
        if s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(GitError::VersionUnparsable {
                output: format!("not a 40-hex tree id: {s}"),
            })
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TreeHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Which tree is being recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeBasis {
    /// A checkpoint commit is due: the recorded tree is that commit's.
    CheckpointCommit(String),
    /// No checkpoint is due: the current index plus tracked modifications.
    WorkingState,
}

/// Derive the tree hash. **The only derivation in this workspace.**
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn tree_hash(
    exec: &GitExec,
    cwd: &WorktreePath,
    common_dir: &Path,
    basis: &TreeBasis,
) -> Result<TreeHash, GitError> {
    match basis {
        TreeBasis::CheckpointCommit(sha) => {
            let out = exec.run(
                GitOp::RevParse,
                cwd,
                ["rev-parse", &format!("{sha}^{{tree}}")],
            )?;
            TreeHash::from_hex(&out.stdout_trimmed())
        }
        TreeBasis::WorkingState => {
            let tmp_dir = common_dir.join("redkiln").join("tmp");
            std::fs::create_dir_all(&tmp_dir).map_err(|e| GitError::Spawn {
                detail: format!("could not create the temporary index dir: {e}"),
            })?;
            // The name is derived, never random: `clippy.toml` bans RNG on this side, and a
            // content-free unique name would be a determinism hole (AC-002).
            let index = tmp_dir.join("index-tree-hash");
            let _ = std::fs::remove_file(&index);

            let env: Vec<(OsString, OsString)> =
                vec![("GIT_INDEX_FILE".into(), index.clone().into_os_string())];

            // Seed the temporary index from HEAD, then take tracked modifications only.
            run_with_index(exec, cwd, &env, GitOp::Add, &["read-tree", "HEAD"])?;
            run_with_index(exec, cwd, &env, GitOp::Add, &["add", "-u"])?;
            let out = run_with_index(exec, cwd, &env, GitOp::WriteTree, &["write-tree"])?;

            let _ = std::fs::remove_file(&index);
            TreeHash::from_hex(&out.stdout_trimmed())
        }
    }
}

/// Run one wrapper operation with `GIT_INDEX_FILE` pointed at the temporary index.
fn run_with_index(
    exec: &GitExec,
    cwd: &WorktreePath,
    env: &[(OsString, OsString)],
    op: GitOp,
    args: &[&str],
) -> Result<crate::git_exec::GitOutput, GitError> {
    let mut all = exec.config_prelude(op);
    all.extend(args.iter().map(|a| OsString::from(*a)));
    crate::git_exec::run_with_env(Path::new(cwd.as_str()), &all, env.iter().cloned()).map_err(|e| {
        match e {
            GitError::NonZeroExit { code, stderr, .. } => GitError::NonZeroExit {
                op: op.as_str(),
                code,
                stderr,
            },
            other => other,
        }
    })
}
