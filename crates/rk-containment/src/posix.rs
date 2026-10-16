//! The POSIX arm's safe surface: a [`Group`] over one process group.
//!
//! Also [`spawn_into_job`] (the name is the cross-platform primitive's; on POSIX the "job" is the
//! pgid) and the two `#[doc(hidden)]` test-support probes. **No `unsafe` in this file** — every signal call is `ffi::posix`'s (`s7c` §7c.10).
//!
//! *(Dated 2026-10-03: this arm has now RUN, on `ubuntu-latest` CI; what that run measured is
//! recorded at [`Group::kill`] and [`is_alive`]. The paragraph below is kept as it was written.)*
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
//! *(Dated 2026-10-16, issue #20: [`Group::is_empty`] now reads [`Group::pids`], because
//! `kill(-pgid, 0)` counts a zombie member; the probe is gone from `ffi`.)*
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

    /// `killpg(pgid, SIGKILL)`, then **wait until every member has terminated** — the Windows arm's
    /// contract (`TerminateJobObject` and a bounded wait per member), so a caller's first read after
    /// `kill` returns sees a dead tree on both arms. An already-empty group is success. The
    /// [`KillDropOrder`] probe records the kill only when it completed.
    ///
    /// "Terminated" is no longer listed by [`Group::pids`] (gone, or a zombie): a zombie has exited, and its reaper
    /// is the caller's own `child.wait()` (the direct child) or PID 1 (an orphan), so waiting for the
    /// reap could deadlock a caller that waits after the kill. The wait is [`KILL_WAIT_TICKS`] polls
    /// of [`KILL_WAIT_TICK`] (10 s, the Windows per-member bound), counted rather than clocked
    /// (`clippy.toml` bans the clock and `thread::sleep`, AC-001/AC-003).
    ///
    /// *(Dated 2026-10-03: the first `ubuntu-latest` run measured the old return-at-once `kill` —
    /// `run_bounded_kills_tree_at_timeout` read the orphaned holder alive on the first read after
    /// `TimedOut` on one run and dead on the next.)*
    ///
    /// # Errors
    /// [`ContainError::Kill`] if `killpg` fails for a reason other than `ESRCH`, or a member is
    /// still running when the wait is spent; [`ContainError::Enumerate`] if `/proc` cannot be read.
    pub fn kill(&self) -> Result<(), ContainError> {
        sys::killpg_all(self.pgid).map_err(ContainError::Kill)?;
        for _ in 0..KILL_WAIT_TICKS {
            if self.pids()?.is_empty() {
                self.order.record_kill();
                return Ok(());
            }
            std::thread::park_timeout(KILL_WAIT_TICK);
        }
        let left = self.pids()?;
        if left.is_empty() {
            self.order.record_kill();
            return Ok(());
        }
        Err(ContainError::Kill(std::io::Error::other(format!(
            "pid(s) {left:?} of group {} did not terminate within {} ms of killpg(SIGKILL)",
            self.pgid,
            KILL_WAIT_TICK.as_millis() * u128::from(KILL_WAIT_TICKS)
        ))))
    }

    /// Whether the group has no running member: [`Group::pids`] lists none. A zombie (exited, not
    /// yet reaped) is not a running member, so a group whose only member has exited reads empty
    /// before its reaper runs, as [`Group::kill`]'s wait and [`is_alive`] already read it.
    ///
    /// *(Dated 2026-10-16, issue #20: this was the portable probe `kill(-pgid, 0)`, which succeeds
    /// while any zombie member exists, so an exited, unreaped group read non-empty.)*
    ///
    /// # Errors
    /// [`ContainError::Enumerate`] if `/proc` cannot be read.
    pub fn is_empty(&self) -> Result<bool, ContainError> {
        Ok(self.pids()?.is_empty())
    }

    /// The pids in the group, read from `/proc` (Linux): every `/proc/<n>/stat` whose fifth
    /// field equals the pgid and whose state is not `Z`. A zombie has exited and is NOT listed —
    /// the Windows job's process list likewise holds only active processes, and [`is_alive`] reads
    /// a zombie dead. *(Dated 2026-10-03: a zombie used to be listed until reaped, so a group
    /// emptied by the kill still listed its orphans until PID 1 reaped them.)*
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
            if stat_pgrp(&stat) == Some(self.pgid) && stat_state(&stat) != Some("Z") {
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

/// One poll of [`Group::kill`]'s wait for its members.
const KILL_WAIT_TICK: std::time::Duration = std::time::Duration::from_millis(5);
/// How many polls [`Group::kill`] waits: 2,000 × 5 ms = 10 s, the Windows arm's per-member bound
/// (`windows.rs`'s `KILL_WAIT_EACH_MS`).
const KILL_WAIT_TICKS: u32 = 2_000;

/// Whether `pid` is a zombie (exited, not yet reaped): `/proc/<pid>/stat`'s state field is `Z`.
/// A pid with no readable `/proc` entry is not one — gone, or a platform with no `/proc`.
fn is_zombie(pid: u32) -> bool {
    fs::read_to_string(format!("/proc/{pid}/stat")).is_ok_and(|stat| stat_state(&stat) == Some("Z"))
}

/// The state field of a `/proc/<pid>/stat` line: the first field after the LAST `)`.
fn stat_state(stat: &str) -> Option<&str> {
    stat[stat.rfind(')')? + 1..].split_whitespace().next()
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

/// Test support: `kill(pid, 0)`, and **not a zombie** — an exited process reads dead whether or not
/// it has been reaped, as `GetExitCodeProcess` reads it on Windows.
///
/// *(Dated 2026-10-03, the first `ubuntu-latest` run: a zombie used to read alive until reaped, so
/// an orphan the kill had already ended read alive until PID 1 reaped it. `rk-cli`'s run-lock
/// predicate below already ANDs this with [`process_start_time`], which reads nothing for a zombie,
/// so its behaviour is unchanged.)*
///
/// *(dated 2026-09-27, `s8d-ii` build, P6; ADR-0180 decision 6, proposed.)* **Now also a
/// production call:** `rk-cli`'s run-tree lock builds its liveness predicate from this and
/// [`process_start_time`], which reads nothing for a zombie. The `#[doc(hidden)]` attribute is left
/// as `s7c` §7c.10 lists it until the record flips. ARMED to CI with the rest of this arm.
#[doc(hidden)]
#[must_use]
pub fn is_alive(pid: u32) -> bool {
    sys::is_alive(pid) && !is_zombie(pid)
}

/// The start time of the LIVE process `pid` names: `/proc/<pid>/stat`'s field 22 (`starttime`,
/// clock ticks since boot), or `None` when there is no such process or it is a zombie (state
/// `Z`, exited and not yet reaped). Read after the last `)`, because field 2 (`comm`) may hold
/// spaces and parentheses. **ARMED to CI with the rest of this arm**: written, not run here
/// (`s8d-ii` §8d-ii.2; ADR-0180 decision 6, proposed). *(dated 2026-09-27, s8d-ii build, P3.)*
#[must_use]
pub fn process_start_time(pid: u32) -> Option<u64> {
    let text = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let (_, after_comm) = text.rsplit_once(')')?;
    let fields: Vec<&str> = after_comm.split_whitespace().collect();
    // After `comm`, fields[0] is field 3 (state), so field 22 is fields[19].
    if fields.first().is_none_or(|state| *state == "Z") {
        return None;
    }
    fields.get(19)?.parse().ok()
}
