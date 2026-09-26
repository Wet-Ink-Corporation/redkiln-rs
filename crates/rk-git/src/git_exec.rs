//! The one wrapper. Every `git` invocation in this workspace goes through here (ADR-0077:70,
//! restated as ADR-0105 clause 5 and as the type ban in `clippy.toml`).
//!
//! Three properties are structural rather than remembered:
//!
//! * **The write set is closed.** [`GitExec::run`] takes a [`GitOp`], never a free `&str`, so
//!   ADR-0105 clause 5's *"anything not listed is a successor record"* is a compile error rather
//!   than a review convention.
//! * **Every commit and every trailer write carries its `-c` prelude.** `core.hooksPath` pointing
//!   at an empty directory (clause 2) and `trailer.ifexists=add` (clause 3) are applied by the
//!   wrapper, not by callers, so there is no call site that can forget them.
//! * **`git` is asserted at startup.** [`GitExec::new`] demands a [`GitToolchain`], which only
//!   `version::assert_at_startup` can mint (ADR-0077; ADR-0091 decision 4).
//!
//! **Git's stderr is diagnostic payload and is never parsed for control flow** (ADR-0077:70). The
//! only thing this module does with stderr is carry its bytes to the caller for retention; nothing
//! here branches on its content. That is the RK-B0013 shape one level down: a transport fault must
//! never be readable as a verdict.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::error::GitError;
use crate::path::WorktreePath;
use crate::version::GitToolchain;

/// The enumerated operation set of ADR-0105 clause 5, plus the reads the hybrid keeps on the
/// executable. Anything not here is a successor record, and adding a variant is that decision made
/// visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitOp {
    // --- ADR-0105 clause 5's write set ---
    WorktreeAdd,
    WorktreeRemove,
    Add,
    /// Includes `--allow-empty` and `--trailer`; always gets the hooks and trailer prelude.
    Commit,
    /// Always gets `--if-exists=add` as well as the config prelude.
    InterpretTrailers,
    /// `write-tree` against a temporary index — the one `tree_hash` derivation (clause 4).
    WriteTree,
    PatchId,
    /// Fast-forward or merge on the integration ref.
    Integrate,
    /// Removal of a disposable review clone.
    CloneRemove,

    // --- reads ---
    RevParse,
    Status,
    Log,
    /// `ls-files -z --others --ignored --exclude-from=.worktreeinclude` — the `.worktreeinclude`
    /// reader's first read (s8c §8c.4, D21-45 W3). A read: ADR-0105 clause 5's write set is
    /// untouched. *(Added 2026-09-26, s8c build, P6.)*
    LsFiles,
    /// `check-ignore -z --stdin --no-index` — the reader's second read, over the first's list.
    /// Exit 1 means *none ignored*, not failure. *(Added 2026-09-26, s8c build, P6.)*
    CheckIgnore,
}

impl GitOp {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorktreeAdd => "worktree add",
            Self::WorktreeRemove => "worktree remove",
            Self::Add => "add",
            Self::Commit => "commit",
            Self::InterpretTrailers => "interpret-trailers",
            Self::WriteTree => "write-tree",
            Self::PatchId => "patch-id",
            Self::Integrate => "integrate",
            Self::CloneRemove => "clone-remove",
            Self::RevParse => "rev-parse",
            Self::Status => "status",
            Self::Log => "log",
            Self::LsFiles => "ls-files",
            Self::CheckIgnore => "check-ignore",
        }
    }

    /// Does this operation write? Used to decide whether the hooks/trailer prelude applies, and
    /// asserted by the replay seam, which must perform none of these.
    #[must_use]
    pub const fn is_write(self) -> bool {
        match self {
            Self::WorktreeAdd
            | Self::WorktreeRemove
            | Self::Add
            | Self::Commit
            | Self::InterpretTrailers
            | Self::WriteTree
            | Self::PatchId
            | Self::Integrate
            | Self::CloneRemove => true,
            Self::RevParse | Self::Status | Self::Log | Self::LsFiles | Self::CheckIgnore => false,
        }
    }

    /// Does this operation create a commit, so that the target repository's hooks must not run?
    #[must_use]
    pub const fn creates_a_commit(self) -> bool {
        matches!(self, Self::Commit)
    }

    /// Does this operation touch trailers, so `trailer.ifexists=add` must be in force?
    #[must_use]
    pub const fn writes_a_trailer(self) -> bool {
        matches!(self, Self::Commit | Self::InterpretTrailers)
    }
}

/// One completed `git` invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl GitOutput {
    /// `stdout` as UTF-8 with trailing newline trimmed — the shape every `rev-parse` caller wants.
    #[must_use]
    pub fn stdout_trimmed(&self) -> String {
        String::from_utf8_lossy(&self.stdout).trim_end().to_string()
    }
}

/// The wrapper. Holding one is proof the startup precondition ran.
#[derive(Debug, Clone)]
pub struct GitExec {
    hooks_empty: PathBuf,
    toolchain: GitToolchain,
}

impl GitExec {
    /// Build the wrapper. `hooks_empty` is created if absent — a filesystem write, never a git
    /// write, so `reconcile()`'s "mutates nothing in git" (AC-174) is untouched.
    ///
    /// # Errors
    /// [`GitError::Spawn`] if the empty hooks directory cannot be created.
    pub fn new(hooks_empty: PathBuf, toolchain: GitToolchain) -> Result<Self, GitError> {
        std::fs::create_dir_all(&hooks_empty).map_err(|e| GitError::Spawn {
            detail: format!("could not create the empty hooks dir: {e}"),
        })?;
        Ok(Self {
            hooks_empty,
            toolchain,
        })
    }

    #[must_use]
    pub const fn toolchain(&self) -> GitToolchain {
        self.toolchain
    }

    /// The `-c` prelude for `op`, applied by the wrapper so no call site can omit it.
    ///
    /// `core.hooksPath` on anything that creates a commit (ADR-0105 clause 2), and
    /// `trailer.ifexists=add` on anything that writes a trailer (clause 3) — because git defaults
    /// `ifExists` to `addIfDifferentNeighbor` and would silently drop a repeated identical `Gate:`
    /// or `Slice-Verdict:` trailer. That default is measured, not assumed: see
    /// `control_oracle_drops_a_duplicate_without_if_exists_add`.
    #[must_use]
    pub fn config_prelude(&self, op: GitOp) -> Vec<OsString> {
        let mut out: Vec<OsString> = Vec::new();
        if op.creates_a_commit() {
            out.push("-c".into());
            let mut v = OsString::from("core.hooksPath=");
            v.push(self.hooks_empty.as_os_str());
            out.push(v);
        }
        if op.writes_a_trailer() {
            out.push("-c".into());
            out.push("trailer.ifexists=add".into());
        }
        out
    }

    /// Run `op` in `cwd`.
    ///
    /// `--if-exists=add` is appended for `interpret-trailers` in addition to the config prelude:
    /// ADR-0105 clause 3 requires both spellings, because the command-line form wins over config
    /// and a caller that supplied its own `--if-exists` would otherwise silently take precedence.
    ///
    /// # Errors
    /// [`GitError::Spawn`] if the child cannot start, [`GitError::NonZeroExit`] otherwise.
    pub fn run<I, S>(&self, op: GitOp, cwd: &WorktreePath, args: I) -> Result<GitOutput, GitError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut all = self.config_prelude(op);
        if op == GitOp::InterpretTrailers {
            all.push("interpret-trailers".into());
            all.push("--if-exists=add".into());
        }
        all.extend(args.into_iter().map(|a| a.as_ref().to_os_string()));
        run_labelled(op, Path::new(cwd.as_str()), all)
    }

    /// Run `op` in `cwd` with `stdin` written to the child's standard input — the form
    /// `check-ignore --stdin` needs (s8c §8c.4, the `.worktreeinclude` reader). The same `-c`
    /// prelude as [`GitExec::run`] applies; no op that writes a trailer is routed here.
    /// *(Added 2026-09-26, s8c build, P6.)*
    ///
    /// # Errors
    /// [`GitError::Spawn`] if the child cannot start or its stdin cannot be written,
    /// [`GitError::NonZeroExit`] otherwise.
    pub fn run_with_stdin<I, S>(
        &self,
        op: GitOp,
        cwd: &WorktreePath,
        args: I,
        stdin: &[u8],
    ) -> Result<GitOutput, GitError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut all = self.config_prelude(op);
        all.extend(args.into_iter().map(|a| a.as_ref().to_os_string()));
        relabel(
            op,
            spawn(
                Path::new(cwd.as_str()),
                all,
                std::iter::empty::<(&OsStr, &OsStr)>(),
                Some(stdin),
            ),
        )
    }
}

/// Run `git` with `args` in `cwd`, unlabelled. Used by the startup probe, which necessarily runs
/// before a [`GitExec`] can exist, and by fixtures.
///
/// # Errors
/// [`GitError::Spawn`] if the child cannot start, [`GitError::NonZeroExit`] otherwise.
pub fn run<I, S>(cwd: &Path, args: I) -> Result<GitOutput, GitError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_with_env(cwd, args, std::iter::empty::<(&OsStr, &OsStr)>())
}

fn run_labelled<I, S>(op: GitOp, cwd: &Path, args: I) -> Result<GitOutput, GitError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    relabel(
        op,
        run_with_env(cwd, args, std::iter::empty::<(&OsStr, &OsStr)>()),
    )
}

fn relabel(op: GitOp, r: Result<GitOutput, GitError>) -> Result<GitOutput, GitError> {
    match r {
        Err(GitError::NonZeroExit { code, stderr, .. }) => Err(GitError::NonZeroExit {
            op: op.as_str(),
            code,
            stderr,
        }),
        other => other,
    }
}

/// As [`run`], with additional environment variables set on the child.
///
/// Env reaches the child through `Command::env`, never `std::env::set_var` — that became an
/// `unsafe fn` in edition 2024, races a concurrent getenv, and is on `clippy.toml`'s ban list.
///
/// `Command::new("git")` is written as a LITERAL and resolved through `PATH` on purpose. Binding
/// it to a stored, already-resolved path would make Checkpoint 6's and CI's `Command::new("git")`
/// greps match nothing forever — a guard that cannot fail (Halt 8).
///
/// # Errors
/// [`GitError::Spawn`] if the child cannot start, [`GitError::NonZeroExit`] otherwise.
pub fn run_with_env<I, S, E, K, V>(cwd: &Path, args: I, env: E) -> Result<GitOutput, GitError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    E: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    spawn(cwd, args, env, None)
}

/// The one process-exec site (the doc on [`run_with_env`] says why it is a literal). `stdin`,
/// when given, is written to the child from a scoped thread while its output is read, so neither
/// pipe can fill and deadlock the other. *(The site moved here from `run_with_env`'s body on
/// 2026-09-26, s8c build, P6, when `check-ignore --stdin` needed a stdin; `run_with_env` calls it
/// with `None`, and the `Command` literal is still the only one in the workspace.)*
///
/// # Errors
/// [`GitError::Spawn`] if the child cannot start or its stdin cannot be written,
/// [`GitError::NonZeroExit`] otherwise.
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0077:70 and ADR-0105 clause 5: this is the single process-exec site in the \
              workspace through which git is invoked. clippy.toml's disallowed-types has no path \
              scoping, so the exemption is stated at the narrowest item that needs it."
)]
fn spawn<I, S, E, K, V>(
    cwd: &Path,
    args: I,
    env: E,
    stdin: Option<&[u8]>,
) -> Result<GitOutput, GitError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    E: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut cmd = std::process::Command::new("git");
    cmd.args(args).envs(env).current_dir(cwd);
    let spawn_err = |e: std::io::Error| GitError::Spawn {
        detail: e.to_string(),
    };
    let out = match stdin {
        None => cmd.output().map_err(spawn_err)?,
        Some(input) => {
            let mut child = cmd
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(spawn_err)?;
            let mut pipe = child.stdin.take().ok_or_else(|| GitError::Spawn {
                detail: "the child's stdin was not piped".into(),
            })?;
            std::thread::scope(|s| {
                let writer = s.spawn(move || {
                    use std::io::Write as _;
                    // Dropping `pipe` at the end of this closure closes the child's stdin.
                    pipe.write_all(input)
                });
                let out = child.wait_with_output().map_err(spawn_err)?;
                // A child that exits non-zero may close its stdin early, so the writer's broken
                // pipe is not the fault then: the exit is, and it is reported below as
                // `NonZeroExit`. Only a child that SUCCEEDED over a short write is refused here.
                match writer.join() {
                    Ok(Ok(())) => Ok(out),
                    _ if !out.status.success() => Ok(out),
                    Ok(Err(e)) => Err(GitError::Spawn {
                        detail: format!("writing the child's stdin failed: {e}"),
                    }),
                    Err(_) => Err(GitError::Spawn {
                        detail: "the stdin writer thread panicked".into(),
                    }),
                }
            })?
        }
    };

    if out.status.success() {
        Ok(GitOutput {
            stdout: out.stdout,
            stderr: out.stderr,
        })
    } else {
        Err(GitError::NonZeroExit {
            op: "git",
            code: out.status.code(),
            stderr: out.stderr,
        })
    }
}
