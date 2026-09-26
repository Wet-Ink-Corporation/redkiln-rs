//! One bounded child run through `rk_containment::run_bounded`, its output to files.
//!
//! One bounded child run — every `cargo` this crate spawns goes through [`run`], and [`run`] goes
//! through `rk_containment::run_bounded` (ADR-0127 decision 3: *"every child any of them spawns
//! is spawned through it"*; decision 2's token-less bounded run, whose bound is the adapter's
//! **construction-time** timeout — RT-5). No `.spawn()` / `.output()` / `.status()` is called in
//! this crate's `src/` (the block's ADR-0127 ban); the `Command` is BUILT here and HANDED over.
//!
//! **stdout and stderr go to FILES, never a pipe** (DECISIONS-2026-09-09 #31: *"Output goes to a
//! file, not a pipe, because polling `try_wait` while a pipe fills is a deadlock"*; `run_bounded`'s
//! module doc says the same and cannot check it — there is no getter on `Command` — so the rule
//! is kept here, at the one site that sets stdio). The files live under the CALL's own run
//! directory beneath the adapter's scratch (`<scratch>/run-<n>`, allocated per port-method call
//! by the adapter — two concurrent calls on one `Send + Sync` adapter share no file) and are read
//! back by path after the child ends: the stderr bytes are digested for
//! `NonZero { stderr_digest }` (retained, never parsed), and libtest's `--list` stdout is read by
//! [`crate::report`] for its `: test` lines and nothing else.
//!
//! **The environment is the constructed map, and nothing else** (`s7c` §7c.14): `env_clear()`,
//! then the `Config`'s captured map, then `CARGO_TARGET_DIR` = the scratch target dir. The process
//! environment is never read here (the AC-217 ban).
//!
//! **What comes back is a value with the child's fate decided by `rk-containment`, not by this
//! module**: [`Run::outcome`] is `run_bounded`'s `Outcome` — `Exited(status)`, `TimedOut`,
//! `OsKilled` — and the mapping onto `StepError` is [`crate::report`]'s, in one place, so a
//! killed run can never be read as a status (ADR-0127's forbids; AC-331). A spawn that fails is
//! `Transport { kind: Spawn }` from [`spawn_error`] — AC-344's adapter share: an absent program is
//! never a failing test and never `NonZero`.
//!
//! **The two `Instant::now` sites** bound `wall_secs` (AC-329's member) and feed no decision; each
//! carries its `#[expect]` (ADR-0102 decision 3; `s7c` §7c.19's per-site rule).

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use rk_containment::{ContainError, Outcome, run_bounded};
use rk_types::error::{StepError, TransportKind};

use crate::Config;

/// One completed (or bounded, or OS-ended) child run.
#[derive(Debug)]
pub struct Run {
    /// The vector handed to `std::process::Command` — the program as the `Config` names it, then
    /// every argument in order — AC-325's *"the argv actually handed to the OS process API"* as
    /// `std` receives it, never a template. (For a `.cmd` launcher such as the default `npx.cmd`,
    /// `std` on Windows hands `CreateProcessW` a `cmd.exe /c "<program>" …` line built from this
    /// vector; the recorded vector is the one `std` was given, which is what re-executing it
    /// verbatim reproduces.)
    pub argv: Vec<String>,
    /// How it ended, as `rk-containment` decided it.
    pub outcome: Outcome,
    /// Where the child's stdout landed.
    pub stdout: PathBuf,
    /// Where the child's stderr landed.
    pub stderr: PathBuf,
    /// Wall time between spawn and the outcome.
    pub wall: Duration,
}

impl Run {
    /// The stdout bytes, read back by path.
    ///
    /// # Errors
    /// `Transport { Io }` if the file cannot be read.
    pub fn stdout_bytes(&self) -> Result<Vec<u8>, StepError> {
        read(&self.stdout)
    }

    /// The stderr bytes, read back by path — retained for the digest, never parsed.
    ///
    /// # Errors
    /// `Transport { Io }` if the file cannot be read.
    pub fn stderr_bytes(&self) -> Result<Vec<u8>, StepError> {
        read(&self.stderr)
    }
}

fn read(path: &Path) -> Result<Vec<u8>, StepError> {
    std::fs::read(path).map_err(|e| StepError::Transport {
        kind: TransportKind::Io,
        detail: format!(
            "could not read the child's output file {}: {e}",
            path.display()
        ),
    })
}

/// A `ContainError` from the spawn path, as the `StepError` the port carries: `Spawn` is
/// `Transport { kind: Spawn }` (AC-344's adapter share), everything else `Transport { kind: Io }`.
#[must_use]
pub fn spawn_error(e: &ContainError) -> StepError {
    match e {
        ContainError::Spawn(io) => StepError::Transport {
            kind: TransportKind::Spawn,
            detail: format!("the toolchain program could not be started: {io}"),
        },
        ContainError::Create(_)
        | ContainError::Assign(_)
        | ContainError::Resume(_)
        | ContainError::Kill(_)
        | ContainError::Enumerate(_)
        | ContainError::Wait(_) => StepError::Transport {
            kind: TransportKind::Io,
            detail: e.to_string(),
        },
    }
}

/// Run the configured program bounded, its output to files under the call's run directory.
///
/// Run `program args…` in the project root, bounded by the `Config`'s timeout, output to files
/// named `<tag>.stdout` / `<tag>.stderr` under `dir` — the CALL's own run directory
/// (`<scratch>/run-<n>`, one per port-method call, never a fixed name rewritten per run: the port
/// is `Send + Sync` over `&self`, so two calls on one adapter may run at once and must share no
/// file).
///
/// # Errors
/// `Transport { Spawn }` if the program could not be started; `Transport { Io }` if the output
/// files could not be created or the containment primitives failed around the run.
pub fn run(config: &Config, dir: &Path, tag: &str, args: &[String]) -> Result<Run, StepError> {
    run_program(config, dir, tag, config.program(), args)
}

/// As [`run`], over an explicit program — the `rustc -V` version read, which is not the
/// configured `cargo`; everything else (cwd, environment, files, bound) is the same.
///
/// # Errors
/// As [`run`].
pub fn run_program(
    config: &Config,
    dir: &Path,
    tag: &str,
    program: &Path,
    args: &[String],
) -> Result<Run, StepError> {
    run_in(
        config,
        dir,
        tag,
        program,
        args,
        Path::new(config.project_root().as_str()),
    )
}

/// As [`run_program`], in an explicit working directory.
///
/// The `smoke` handler runs a checklist row in its scratch directory, never the project root (record V = ADR-0170 decision 4, proposed).
/// Everything else (environment, files, bound) is the same. *(Added 2026-09-26, s8c build, P5.)*
///
/// # Errors
/// As [`run`].
pub fn run_in(
    config: &Config,
    dir: &Path,
    tag: &str,
    program: &Path,
    args: &[String],
    cwd: &Path,
) -> Result<Run, StepError> {
    let stdout = dir.join(format!("{tag}.stdout"));
    let stderr = dir.join(format!("{tag}.stderr"));
    let out = std::fs::File::create(&stdout).map_err(|e| io_err("stdout", &stdout, &e))?;
    let err = std::fs::File::create(&stderr).map_err(|e| io_err("stderr", &stderr, &e))?;

    let mut argv = Vec::with_capacity(args.len() + 1);
    argv.push(program.to_string_lossy().into_owned());
    argv.extend(args.iter().cloned());

    let cmd = build_command(config, program, args, cwd, out, err);
    #[expect(
        clippy::disallowed_methods,
        reason = "AC-001 bans the clock on the WORKFLOW side; this is the FIRST of the adapter's two \
                  Instant::now sites, the pair that bounds VerifyReport.wall_secs (s7c §7c.19); \
                  feeds no decision."
    )]
    let started = Instant::now();
    let outcome = run_bounded(cmd, config.timeout()).map_err(|e| spawn_error(&e))?;
    #[expect(
        clippy::disallowed_methods,
        reason = "the SECOND of the adapter's two Instant::now sites: the end of the pair that \
                  bounds VerifyReport.wall_secs (s7c §7c.19); feeds no decision."
    )]
    let ended = Instant::now();
    Ok(Run {
        argv,
        outcome,
        stdout,
        stderr,
        wall: ended.duration_since(started),
    })
}

fn io_err(which: &str, path: &Path, e: &std::io::Error) -> StepError {
    StepError::Transport {
        kind: TransportKind::Io,
        detail: format!(
            "could not create the child's {which} file {}: {e}",
            path.display()
        ),
    }
}

/// The one `Command` construction site: program, args, cwd (the project root, or the smoke
/// handler's scratch directory: [`run_in`]), `env_clear()` +
/// the captured map + `CARGO_TARGET_DIR`, stdout/stderr = the files. stdin is `run_bounded`'s
/// (null); the creation flags / process group are `spawn_into_job`'s.
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0127 decision 3: the adapter BUILDS the std Command and HANDS it to rk-containment's \
              run_bounded, which spawns it inside a group; this is the crate's one Command site and \
              clippy.toml's type ban has no path scoping, so the exemption is stated here."
)]
fn build_command(
    config: &Config,
    program: &Path,
    args: &[String],
    cwd: &Path,
    out: std::fs::File,
    err: std::fs::File,
) -> std::process::Command {
    let mut cmd = std::process::Command::new(program);
    cmd.args(args)
        .current_dir(cwd)
        .env_clear()
        .envs(config.child_env())
        .env(crate::env::CARGO_TARGET_DIR, config.target_dir())
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::from(err));
    cmd
}
