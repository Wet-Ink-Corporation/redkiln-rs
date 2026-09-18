//! The POSIX arm's safe surface: a [`Group`] over one process group.
//!
//! Also [`spawn_into_job`] (the name is the cross-platform primitive's; on POSIX the "job" is the
//! pgid) and the two `#[doc(hidden)]` test-support probes. **No `unsafe` in this file** — every signal call is `ffi::posix`'s (`s7c` §7c.10).
//!
//! **This arm is written and not run.** The box this crate was written on is Windows
//! (`00-preamble.md`: every block runs on this box), the POSIX arm is **ARMED to a recorded
//! `ubuntu-latest` CI run** that has never executed (`s7c` §7c.10 AC-224; ADR-0127 §Consequences),
//! and nothing below is claimed measured. It compiles against `libc`'s signatures and `std`'s
//! `CommandExt::process_group`, which is what can be checked from here.
//!
//! **The shape.** `process_group(0)` at spawn makes the child the leader of a new process group
//! whose id is its own pid — so the group exists the instant the child does, and there is no
//! create-before-spawn step to order: the group IS the spawn. Kill is `killpg(pgid, SIGKILL)`.
//! Enumeration by pid has no syscall; [`Group::pids`] reads `/proc/<pid>/stat`'s fifth field (the
//! pgrp) over every numeric entry of `/proc`, which is Linux's and is what CI runs; the
//! emptiness probe `kill(-pgid, 0)` is the portable half and [`Group::is_empty`] exposes it.
//!
//! **What a process group does not promise, named rather than discovered on CI:** a member can
//! leave the group with `setpgid`/`setsid` — the POSIX analogue of Windows' `CREATE_BREAKAWAY_FROM_JOB`,
//! and one that no flag on the group can refuse. The Job Object refuses breakaway; the process
//! group cannot. The fixture child's `breakaway` mode on POSIX spawns its holder with
//! `process_group(0)` for exactly that reason, so a CI run of the POSIX arm measures the hole
//! rather than assumes its absence; `breakaway_attempt_fails_without_breakaway_ok` is
//! `cfg(windows)` and makes no claim here (`s7c` §7c.18).

use std::fs;
use std::sync::Arc;

use crate::error::ContainError;
use crate::ffi::posix as sys;
use crate::{Contained, KillDropOrder};

pub use crate::ffi::CANCEL_EXIT_CODE;

/// One process group, identified by its pgid — the direct child's pid.
#[derive(Debug)]
pub struct Group {
    pgid: u32,
    order: Arc<KillDropOrder>,
}

impl Group {
    /// The group id — the direct child's pid.
    #[must_use]
    pub const fn pgid(&self) -> u32 {
        self.pgid
    }

    /// `killpg(pgid, SIGKILL)`. An already-empty group is success. The [`KillDropOrder`] probe
    /// records the kill only when the call succeeded — never for a `killpg` that failed.
    ///
    /// # Errors
    /// [`ContainError::Kill`] if `killpg` fails for a reason other than `ESRCH`.
    pub fn kill(&self) -> Result<(), ContainError> {
        sys::killpg_all(self.pgid).map_err(ContainError::Kill)?;
        self.order.record_kill();
        Ok(())
    }

    /// Whether any member of the group still exists (`kill(-pgid, 0)`) — the portable probe.
    ///
    /// # Errors
    /// [`ContainError::Enumerate`] if the probe fails for a reason other than `ESRCH`/`EPERM`.
    pub fn is_empty(&self) -> Result<bool, ContainError> {
        sys::group_has_members(self.pgid)
            .map(|has| !has)
            .map_err(ContainError::Enumerate)
    }

    /// The pids in the group, read from `/proc` (Linux): every `/proc/<n>/stat` whose fifth
    /// field equals the pgid. A zombie is listed until reaped.
    ///
    /// # Errors
    /// [`ContainError::Enumerate`] if `/proc` cannot be read.
    pub fn pids(&self) -> Result<Vec<u32>, ContainError> {
        let entries = fs::read_dir("/proc").map_err(ContainError::Enumerate)?;
        let mut out = Vec::new();
        for entry in entries {
            let entry = entry.map_err(ContainError::Enumerate)?;
            let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() else {
                continue;
            };
            // A process may exit between the listing and the read; that is not an error.
            let Ok(stat) = fs::read_to_string(entry.path().join("stat")) else {
                continue;
            };
            if stat_pgrp(&stat) == Some(self.pgid) {
                out.push(pid);
            }
        }
        Ok(out)
    }

    /// The kill-then-drop order probe (test support): a clone that outlives this value.
    #[doc(hidden)]
    #[must_use]
    pub fn kill_drop_order(&self) -> Arc<KillDropOrder> {
        Arc::clone(&self.order)
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        self.order.record_drop();
        // A process group has no handle to close and no KILL_ON_JOB_CLOSE: what a caller did not
        // kill survives the drop. That asymmetry is the POSIX arm's and is recorded here, not hidden.
    }
}

/// The pgrp field of a `/proc/<pid>/stat` line: the fifth field, after `pid (comm) state ppid`,
/// where `comm` may contain spaces and parentheses and is delimited by the LAST `)`.
fn stat_pgrp(stat: &str) -> Option<u32> {
    let after_comm = &stat[stat.rfind(')')? + 1..];
    // fields after the comm: state ppid pgrp session ...
    after_comm.split_whitespace().nth(2)?.parse().ok()
}

/// Spawn the command as the leader of a new process group and hand back the child with the
/// group that names it.
///
/// **What this does to the `Command`:** its `process_group` setting is replaced by `0` (the
/// child leads a new group — `std`'s `process_group` sets, and `Command` has no getter); nothing
/// else on it is touched. A caller must not set a process group of its own.
///
/// # Errors
/// [`ContainError::Spawn`] if the child cannot be started.
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0127 decision 2: the public spawn-into-the-group primitive takes the std Command the \
              adapter built and hands it to the one spawn site in ffi.rs, which owns the process-group \
              setting (process_group(0) replaces whatever the caller set) and touches nothing else on \
              it. clippy.toml's disallowed-types has no path scoping, so the exemption is stated at the \
              narrowest item that needs it."
)]
pub fn spawn_into_job(cmd: std::process::Command) -> Result<Contained, ContainError> {
    let child = sys::spawn_in_new_group(cmd).map_err(ContainError::Spawn)?;
    let group = Group {
        pgid: child.id(),
        order: Arc::new(KillDropOrder::new()),
    };
    Ok(Contained { child, group })
}

/// Test support: `kill(pid, SIGKILL)`; `code` is Windows' and is accepted for signature symmetry.
///
/// # Errors
/// [`ContainError::Kill`] if the signal cannot be sent.
#[doc(hidden)]
pub fn terminate_pid(pid: u32, code: u32) -> Result<(), ContainError> {
    sys::terminate_pid(pid, code).map_err(ContainError::Kill)
}

/// Test support: `kill(pid, 0)` — alive (or a zombie not yet reaped).
#[doc(hidden)]
#[must_use]
pub fn is_alive(pid: u32) -> bool {
    sys::is_alive(pid)
}
