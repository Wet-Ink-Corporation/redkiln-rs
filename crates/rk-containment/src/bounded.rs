//! `run_bounded` — the token-less bounded run the two Toolchain adapters use (ADR-0127 decision 2;
//! ADR-0099 decision 2's *"the two Toolchain adapters run bounded commands under the same timeout
//! without a token"*, which stands as worded because this takes none).
//!
//! **What it returns is a value with three cases, not a status with a convention** — [`Outcome`]:
//! `Exited(status)` when the child ended on its own, `TimedOut` when THIS function's bound fired
//! and killed the group, `OsKilled` when the OS ended the child. The distinction is what AC-331's
//! `Unverifiable` is built from (*"a verify run killed by its timeout or by the OS … the result is
//! `Unverifiable` … never recorded as a pass or as a test failure attributable to the candidate"*),
//! and ADR-0127's forbids names the failure this shape prevents: *"reporting a timed-out or
//! OS-killed run as an exit status"*. Each adapter maps the value onto its own `StepError`; this
//! crate depends on no `rk-types` (decision 2).
//!
//! **The wait has no sleep and no tokio** (`s7c` §7c.10): a `std::thread` calls `child.wait()` and
//! sends the status down an `mpsc` channel; the caller `recv_timeout`s on it; on `Timeout` it
//! kills the group through the handle and reaps. `std::thread::sleep` and `tokio::time::sleep`
//! are clippy-banned (`clippy.toml`, AC-003) and neither is called; nothing polls.
//!
//! **Output goes to a file the caller configured on the `Command`, never a pipe this function
//! opens.** A pipe fills at 64 KiB and a child that writes more blocks forever on a reader nobody
//! runs — DECISIONS-2026-09-09 #31's deadlock, and `run_bounded_kills_tree_at_timeout`'s pipe red
//! (the fixture child in `spew:<bytes>` mode with N > 64 KiB). This function leaves the
//! `Command`'s stdout/stderr exactly as the caller set them — the toolchain adapters hand in
//! `Stdio::from(File)` for both, because they must read the bytes back by path anyway — and sets
//! only stdin, to null: a bounded toolchain run reads nothing. It cannot inspect a `Command`'s
//! stdio (there is no getter), so a caller that hands in `Stdio::piped()` gets the deadlock the
//! bound then converts to `TimedOut`; the rule is stated here and enforced by the adapters'
//! tests, not by a type. **What is replaced on the `Command`, in full:** stdin (null, here) and,
//! on Windows, the creation-flag word (`CREATE_SUSPENDED`, in [`spawn_into_job`] — `std`'s
//! `creation_flags` sets rather than ORs, so a caller's flags do not survive); on POSIX the
//! process group (`0`). A caller sets neither.
//!
//! **The timeout path kills through the group, and the test can see that it did.** A `Group`
//! that is merely dropped on return would also end the tree — `KILL_ON_JOB_CLOSE` — but
//! asynchronously and after this function has returned `TimedOut`, which is the mutant `s7c`
//! §7c.10's kill-then-drop probe exists to make red. [`run_bounded_with`] hands the test the
//! group right after the spawn so it can hold the probe (`Group::kill_drop_order`) across the
//! call; `run_bounded` is that with an empty hook.
//!
//! **After the child exits, the group is killed anyway.** A verify run that leaves a daemon behind
//! (a watcher, a stuck worker) has exited with a status and a survivor; AC-278's *"no child,
//! background, detached or daemon process belonging to that run remains alive"* is a property of
//! the run, so the group is swept through the handle before the outcome is returned, and the
//! handle's drop sweeps whatever that did not reach. The status the child itself returned is
//! what `Exited` carries.

use std::process::ExitStatus;
use std::sync::mpsc;
use std::time::Duration;

use crate::error::ContainError;
use crate::{Contained, spawn_into_job};

/// How a bounded run ended. `TimedOut` and `OsKilled` are values, never an `Err` and never an
/// `Exited` with a special status (ADR-0127's forbids).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The child ended on its own with this status — a pass or a failure attributable to it.
    Exited(ExitStatus),
    /// This function's bound fired: the group was killed through the handle and reaped.
    TimedOut,
    /// The OS ended the child: on POSIX a signal this crate did not send; on Windows an
    /// error-severity `NTSTATUS` exit (`0xC000_0000` set — an access violation, a stack overflow,
    /// `STATUS_CONTROL_C_EXIT`). A termination by another party with an ordinary code is
    /// indistinguishable from an exit and is reported as `Exited`, which is the reading AC-225
    /// keys on the token for.
    OsKilled,
}

/// How long the caller waits, after a kill on the timeout path, for the waiting thread to report
/// that `wait()` returned. The kill has already completed by then (the group's `kill` waits for
/// every member), so this is a bound on a thread hand-off, not on the child.
const REAP_GRACE: Duration = Duration::from_secs(10);

/// Run `cmd` inside a fresh group to completion or to `timeout`, and say which.
///
/// **What this does to the `Command`:** stdin becomes null; on Windows the creation flags become
/// `CREATE_SUSPENDED` and on POSIX the process group becomes `0` (the module doc). stdout and
/// stderr are the caller's, and must be files.
///
/// # Errors
/// Whatever [`spawn_into_job`] returns; [`ContainError::Kill`] if the group could not be killed on
/// the timeout path; [`ContainError::Wait`] if the child could not be waited on.
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0127 decision 2: run_bounded takes the std Command the toolchain adapter built and \
              runs it inside a group, replacing its stdin (null) and, through spawn_into_job, its \
              creation flags / process group; this signature is one of the crate's stated #[expect] \
              sites (s7c §7c.19). clippy.toml's disallowed-types has no path scoping, so the exemption \
              is stated at the narrowest item that needs it."
)]
pub fn run_bounded(cmd: std::process::Command, timeout: Duration) -> Result<Outcome, ContainError> {
    run_bounded_with(cmd, timeout, |_| {})
}

/// Test support: [`run_bounded`] with a hook that runs right after the spawn, handed the group —
/// so a test can clone its kill-then-drop probe and, after the call returns, assert that the
/// timeout path killed through the group before the group dropped (the module doc).
///
/// # Errors
/// As [`run_bounded`].
#[doc(hidden)]
#[expect(
    clippy::disallowed_types,
    reason = "the hidden test-support form of run_bounded: same Command, same stdin replacement, same \
              spawn through spawn_into_job; the hook is the only difference."
)]
pub fn run_bounded_with(
    mut cmd: std::process::Command,
    timeout: Duration,
    after_spawn: impl FnOnce(&crate::Group),
) -> Result<Outcome, ContainError> {
    cmd.stdin(std::process::Stdio::null());
    let Contained { mut child, group } = spawn_into_job(cmd)?;
    after_spawn(&group);

    let (tx, rx) = mpsc::channel::<std::io::Result<ExitStatus>>();
    std::thread::spawn(move || {
        let status = child.wait();
        // A receiver that has gone away is the timeout path having given up on the hand-off; the
        // status has nowhere to go and nothing depends on it.
        let _ = tx.send(status);
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(status)) => {
            // The child is done; anything it left behind is the run's and dies with the group.
            group.kill()?;
            Ok(classify(status))
        }
        Ok(Err(e)) => {
            group.kill()?;
            Err(ContainError::Wait(e))
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            group.kill()?;
            // The kill waited for every member, so this is a thread hand-off, bounded for form.
            let _ = rx.recv_timeout(REAP_GRACE);
            Ok(Outcome::TimedOut)
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            group.kill()?;
            Err(ContainError::Wait(std::io::Error::other(
                "the waiting thread ended without reporting the child's status",
            )))
        }
    }
}

#[cfg(unix)]
fn classify(status: ExitStatus) -> Outcome {
    use std::os::unix::process::ExitStatusExt;
    if status.signal().is_some() {
        Outcome::OsKilled
    } else {
        Outcome::Exited(status)
    }
}

#[cfg(windows)]
fn classify(status: ExitStatus) -> Outcome {
    // `ExitStatus::code()` is the raw DWORD read as an i32; an NTSTATUS with severity bits `11`
    // (0xC000_0000) is an error-severity termination the OS applied, never a code a process returns.
    let raw = status.code().map(i32::cast_unsigned);
    match raw {
        Some(code) if code & 0xC000_0000 == 0xC000_0000 => Outcome::OsKilled,
        _ => Outcome::Exited(status),
    }
}
