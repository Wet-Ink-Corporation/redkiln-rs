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
            let mut run = |op: GitOp, args: &[&str], env: &[(OsString, OsString)]| {
                run_with_index(exec, cwd, env, op, args)
            };
            derive_working_state(common_dir, true, &mut run)
        }
    }
}

/// One wrapper operation run with a temporary index's environment: the wrapper's own, or the
/// journalled seam's. *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
pub(crate) type IndexRun<'a> = dyn FnMut(GitOp, &[&str], &[(OsString, OsString)]) -> Result<crate::git_exec::GitOutput, GitError>
    + 'a;

/// **The one no-checkpoint-due derivation**, written once and reached two ways: over the wrapper
/// ([`tree_hash`]) and through the journalled seam ([`crate::gitops::GitOps::tree_hash`]), so a
/// replay is served it rather than re-running it. `fs` is `false` on replay: the temporary index's
/// directory is neither created nor cleaned, because nothing reaches git to write it.
/// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE: the body moved here from
/// `tree_hash`'s `WorkingState` arm, byte for byte in what it runs.)*
fn derive_working_state(
    common_dir: &Path,
    fs: bool,
    run: &mut IndexRun<'_>,
) -> Result<TreeHash, GitError> {
    let tmp_dir = common_dir.join("redkiln").join("tmp");
    if fs {
        std::fs::create_dir_all(&tmp_dir).map_err(|e| GitError::Spawn {
            detail: format!("could not create the temporary index dir: {e}"),
        })?;
    }
    // The name is derived, never random: `clippy.toml` bans RNG on this side, and a
    // content-free unique name would be a determinism hole (AC-002).
    let index = tmp_dir.join("index-tree-hash");
    if fs {
        let _ = std::fs::remove_file(&index);
    }

    let env: Vec<(OsString, OsString)> =
        vec![("GIT_INDEX_FILE".into(), index.clone().into_os_string())];

    // Seed the temporary index from HEAD, then take tracked modifications only.
    run(GitOp::Add, &["read-tree", "HEAD"], &env)?;
    run(GitOp::Add, &["add", "-u"], &env)?;
    let tree = write_index_tree(run, &env)?;

    if fs {
        let _ = std::fs::remove_file(&index);
    }
    Ok(tree)
}

/// **The one `write-tree` invocation in this crate**, over the temporary index `env` names: this
/// derivation's, and the landing's T (ADR-0188 decision 9 step 2), which is `write-tree` over the
/// landing's own per-item temporary index. Two callers, one argv site
/// (`tree::there_is_exactly_one_write_tree_call_site`). *(dated 2026-09-28, s8f build, package
/// S8F-P2-LANDING-CORE.)*
///
/// # Errors
/// Propagates [`GitError`] from `run`.
pub(crate) fn write_index_tree(
    run: &mut IndexRun<'_>,
    env: &[(OsString, OsString)],
) -> Result<TreeHash, GitError> {
    let out = run(GitOp::WriteTree, &["write-tree"], env)?;
    TreeHash::from_hex(&out.stdout_trimmed())
}

impl crate::gitops::GitOps<'_> {
    /// [`tree_hash`], **through the journalled seam**: each invocation is recorded, and on replay
    /// served, never run (ADR-0105 clause 1). The derivation is [`tree_hash`]'s own, not a second
    /// one (clause 4). *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; the landing's
    /// re-read (`CheckpointCommit`) and a refused landing's gate tree (`WorkingState`), ADR-0188
    /// decisions 7 and 9.)*
    ///
    /// # Errors
    /// Propagates [`GitError`] from the seam.
    pub fn tree_hash(
        &mut self,
        cwd: &WorktreePath,
        common_dir: &Path,
        basis: &TreeBasis,
    ) -> Result<TreeHash, GitError> {
        match basis {
            TreeBasis::CheckpointCommit(sha) => {
                let spec = format!("{sha}^{{tree}}");
                let out = self.run(GitOp::RevParse, cwd, &["rev-parse", spec.as_str()])?;
                TreeHash::from_hex(&out.stdout_trimmed())
            }
            TreeBasis::WorkingState => {
                let live = self.mode() == crate::gitops::Mode::Live;
                let mut run = |op: GitOp, args: &[&str], env: &[(OsString, OsString)]| {
                    self.run_with_env(op, cwd, args, env)
                };
                derive_working_state(common_dir, live, &mut run)
            }
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
