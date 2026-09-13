//! The one wrapper. Every `git` invocation in this workspace goes through here (ADR-0077:70,
//! restated as ADR-0105 clause 5 and as the type ban in `clippy.toml`).
//!
//! At §6.1 this is deliberately minimal: spawn, capture, map a non-zero exit to a typed error.
//! The `-c` preludes ADR-0105 clauses 2 and 3 require are §6.2's, and each arrives with the test
//! that discriminates it.
//!
//! **Git's stderr is diagnostic payload and is never parsed for control flow** (ADR-0077:70). The
//! only thing this module ever reads out of stderr is its bytes, handed back verbatim to the
//! caller for retention; nothing here branches on its content.

use std::ffi::OsStr;
use std::path::Path;

/// What went wrong invoking `git`. Closed, and matched without a catch-all so that a new variant
/// is `E0004` at every call site rather than a silent fallthrough — the shape
/// `rk_types::StepError` establishes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitError {
    /// The child could not be spawned at all: `git` is not on `PATH`, or the OS refused.
    Spawn { detail: String },
    /// `git` ran and exited non-zero. `stderr` is carried verbatim for retention, never parsed.
    NonZeroExit { code: Option<i32>, stderr: Vec<u8> },
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

/// Run `git` with `args` in `cwd`, returning its captured output.
pub fn run<I, S>(cwd: &Path, args: I) -> Result<GitOutput, GitError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_with_env(cwd, args, std::iter::empty::<(&OsStr, &OsStr)>())
}

/// As [`run`], with additional environment variables set on the child.
///
/// Env reaches the child through `Command::env`, never `std::env::set_var` — that became an
/// `unsafe fn` in edition 2024, races a concurrent getenv, and is on `clippy.toml`'s ban list.
/// Fixture isolation (`GIT_CONFIG_NOSYSTEM`, an empty `GIT_CONFIG_GLOBAL`, a fixed identity and
/// fixed dates) has to come through here, because this is the only site in the workspace where a
/// child process is spawned at all.
///
/// `Command::new("git")` is written as a LITERAL and resolved through `PATH` on purpose. Binding
/// it to a stored, already-resolved path would make Checkpoint 6's and CI's `Command::new("git")`
/// greps match nothing forever — a guard that cannot fail (Halt 8).
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0077:70 and ADR-0105 clause 5: this is the single process-exec site in the \
              workspace through which git is invoked. clippy.toml's disallowed-types has no path \
              scoping, so the exemption is stated at the narrowest item that needs it."
)]
pub fn run_with_env<I, S, E, K, V>(cwd: &Path, args: I, env: E) -> Result<GitOutput, GitError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    E: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let out = std::process::Command::new("git")
        .args(args)
        .envs(env)
        .current_dir(cwd)
        .output()
        .map_err(|e| GitError::Spawn {
            detail: e.to_string(),
        })?;

    if out.status.success() {
        Ok(GitOutput {
            stdout: out.stdout,
            stderr: out.stderr,
        })
    } else {
        Err(GitError::NonZeroExit {
            code: out.status.code(),
            stderr: out.stderr,
        })
    }
}
