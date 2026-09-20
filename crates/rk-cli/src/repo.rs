//! **Finding the repository** — H-3's discovery order, and the one `"."` that makes it work
//! (`s7d` §7d.3's common-dir bullet, **D-18**).
//!
//! Every verb resolves the repository the same way, in this order:
//!
//! 1. `rk_git::version::assert_at_startup` — ADR-0091 decision 4's `>= 2.20` floor, read BEFORE any
//!    `GitExec` exists (it is the one unlabelled `git --version` call). Its refusal is a
//!    configuration error, exit `3`.
//! 2. a **bootstrap** `GitExec` whose empty-hooks directory is under the OS temp directory —
//!    because `GitExec::new` creates that directory at construction and the working one belongs
//!    under a common dir nobody has found yet. The first wrapper cannot be the working one, which is
//!    why `tempfile` is a normal dependency of this binary and not a dev one.
//! 3. `rev-parse --show-toplevel` **through** that wrapper, with the cwd argument being `-C <path>`
//!    when given and the literal `"."` otherwise.
//! 4. `rk_adapter_sqlite::path::common_dir` — the `pub` reader 7c shipped, which joins git's
//!    relative answer onto the worktree it is handed. Never this crate's own `--git-common-dir`.
//! 5. the working `GitExec`, under `<git-common-dir>/redkiln/hooks-empty/`.
//!
//! **Why the `"."` and why only once.** The child git resolves `"."` against the *inherited* working
//! directory, so no Rust code path here names `std::env::current_dir` — which is banned in both new
//! crates and which `ci.yml`'s AC-529 rule watches. But the `"."` is used for the `--show-toplevel`
//! read ALONE. Every later operation takes the **absolute** answer, because
//! `rk_store::identity` stamps the common dir as a string and compares it byte-wise: a `"."` handed
//! on would stamp `./.git` at `init` and then refuse a `status` run from a subdirectory as another
//! repository's store — an AC-413 false positive, and a nasty one, because it would look like the
//! guard working.

use std::fmt;
use std::path::{Path, PathBuf};

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;

/// A resolved repository: where its working tree is, where its git common directory is, and a git
/// wrapper configured for it.
pub struct Repo {
    /// The **absolute** working-tree root, from `rev-parse --show-toplevel`.
    pub worktree: WorktreePath,
    /// The absolute git common directory — not `<root>/.git` in a linked worktree.
    pub common_dir: PathBuf,
    /// The working wrapper, hooks disabled.
    pub exec: GitExec,
}

/// Why the repository could not be resolved. Every arm is exit `3`.
///
/// Each carries `rk_git::error::GitError`'s **`Debug`** rendering, not its `Display` one, because
/// that type has no `Display` — it is a closed enum matched exhaustively by its own crate and never
/// formatted there. Carrying the `Debug` form keeps every field (the op, the exit code, the
/// verbatim stderr) in the diagnostic, which is what makes a refusal here actionable; rendering it
/// prettily is a job for whoever gives `GitError` a `Display`, and doing that would edit a closed
/// crate (§7d.15).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepoError {
    /// git is absent, too old, or unparsable — ADR-0091 decision 4's floor.
    Git(String),
    /// The bootstrap wrapper's temporary hooks directory could not be made.
    Bootstrap(String),
    /// The store adapter's common-dir reader refused.
    CommonDir(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Git(detail) => write!(f, "git is not usable here: {detail}"),
            Self::Bootstrap(detail) => {
                write!(f, "the bootstrap git wrapper could not be built: {detail}")
            }
            Self::CommonDir(detail) => {
                write!(
                    f,
                    "the repository's git common directory is not readable: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for RepoError {}

/// Resolve the repository, in H-3's order.
///
/// # Errors
/// [`RepoError`] naming which step refused. All three are configuration errors.
pub fn discover(cwd_arg: Option<&Path>) -> Result<Repo, RepoError> {
    // (1) The version floor, before any wrapper exists.
    let cwd = cwd_arg.unwrap_or_else(|| Path::new("."));
    let toolchain =
        rk_git::version::assert_at_startup(cwd).map_err(|e| RepoError::Git(format!("{e:?}")))?;

    // (2) The bootstrap wrapper. Its hooks directory lives under the OS temp directory and is kept
    // alive for exactly as long as the bootstrap is used.
    let boot = tempfile::tempdir().map_err(|e| RepoError::Bootstrap(format!("{e:?}")))?;
    let bootstrap = GitExec::new(boot.path().join("hooks-empty"), toolchain)
        .map_err(|e| RepoError::Bootstrap(format!("{e:?}")))?;

    // (3) The one `"."` read, through the wrapper.
    let here = WorktreePath::new(cwd).map_err(|e| RepoError::Git(format!("{e:?}")))?;
    let toplevel = bootstrap
        .run(GitOp::RevParse, &here, ["rev-parse", "--show-toplevel"])
        .map_err(|e| RepoError::Git(format!("{e:?}")))?
        .stdout_trimmed();
    let worktree =
        WorktreePath::new(Path::new(&toplevel)).map_err(|e| RepoError::Git(format!("{e:?}")))?;

    // (4) The common dir, through the reader the store adapter already ships.
    let common_dir = rk_adapter_sqlite::path::common_dir(&bootstrap, &worktree)
        .map_err(|e| RepoError::CommonDir(format!("{e:?}")))?;

    // (5) The working wrapper. From here the bootstrap and its temp directory are done.
    let exec = GitExec::new(common_dir.join("redkiln").join("hooks-empty"), toolchain)
        .map_err(|e| RepoError::Bootstrap(format!("{e:?}")))?;

    Ok(Repo {
        worktree,
        common_dir,
        exec,
    })
}
