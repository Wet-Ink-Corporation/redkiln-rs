//! `rk-containment` — process containment for every child the workspace spawns: a Windows Job
//! Object, a POSIX process group, and a token-less bounded run. ADR-0127's sixth crate.
//!
//! **What this crate is.** ADR-0127 decision 1: *"a library crate under `crates/` beside the
//! five on the tree, and it implements no port"* — it is not an adapter (ADR-0094 decision 1's
//! four are unchanged), it owns **the one** commented `#![allow(unsafe_code)]` module in the
//! workspace (`ffi`, ADR-0102 decision 2's exception relocated by 0127 decision 4), and all three
//! spawning adapters depend on it while `rk-kernel` never does (decision 3; Checkpoint 7c asserts
//! both on `cargo tree`). Its dependencies are `std`, `windows-sys` on Windows and `libc` on Unix
//! (decision 2, decision 5) — no tokio, no `rk-types`, no `rk-ports`, no `rk-adapter-*` (the
//! forbids; RT-15).
//!
//! **Two surfaces, one mechanism** (decision 2):
//!
//! - the primitives the Harness adapter composes with its own `CancelToken` — [`spawn_into_job`]
//!   (create the group, spawn into it) returning a [`Contained`] `{ child, group }`, and on the
//!   [`Group`]: `kill` (through the handle), `pids` (via the job / the pgid), drop (the handle's
//!   close); and
//! - [`run_bounded`] `(cmd, timeout)`, which runs a command inside a group to completion or to the
//!   bound and returns an [`Outcome`] in which *timed out* and *killed by the OS* are values
//!   distinct from *exited with a status* — the value AC-331's `Unverifiable` is built from. It
//!   takes no token, so ADR-0099 decision 2's *"without a token"* stands as worded.
//!
//! **The mechanism, unchanged from ADR-0099 decision 3** (0127 keeps every mechanism sentence
//! and withdraws only the location clause): on **Windows** a Job Object with
//! `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, **created before the spawn**, the child **assigned before
//! it can fork** (spawned `CREATE_SUSPENDED`, assigned, then resumed — RT-13 as amended;
//! [`windows`]'s module doc has the five steps), the **handle held** for the child's lifetime,
//! enumeration and kill **via the job**, never a PID walk (F22's survivor is what a walk misses),
//! and the adapter's own [`CANCEL_EXIT_CODE`] handed to `TerminateJobObject`; on **POSIX**
//! `CommandExt::process_group(0)` at spawn and `killpg` at kill, enumeration via the pgid
//! (`posix`). The job never sets `JOB_OBJECT_LIMIT_BREAKAWAY_OK` (decision 7).
//!
//! **Which arm this box runs.** The Windows arm is the live arm here and is what Slice 2 measured
//! (`s7c` §7c.10; the readings are in the commit body: `ResumeThread`'s previous suspend count
//! and `IsProcessInJob(child, our_job)` before the resume, the orphaned holder reaped via the
//! job, the breakaway attempt refused, the cargo-`KILL_ON_JOB_CLOSE`-job fact beside them). The
//! POSIX arm is written and **ARMED to a recorded CI run** that has never executed.
//!
//! **Two `#[doc(hidden)]` test-support functions** sit outside decision 2's enumerated surface and
//! are named as such (`s7c` §7c.10): [`terminate_pid`] — AC-225's external kill with the same
//! code — and [`is_alive`] — the job-independent liveness probe the orphan tests read, so a mutant
//! with no job cannot hide a survivor from it. Both spawn nothing and leave no containment. **No
//! public function of this crate spawns a child outside a group**; the breakaway attempt the
//! fixture child makes (`tests/fixtures/bin/rk-fixture-child.rs`) is the fixture's own, the
//! instrument of AC-224's negative control, and the record's forbids says in terms that it is
//! not a violation.
//!
//! *(dated 2026-09-27, s8d-ii build, P3.)* **One production read joins them: [`process_start_time`]**,
//! the start time of a live process by pid (`GetProcessTimes` on Windows, `/proc/<pid>/stat` field
//! 22 on POSIX, ARMED to CI with the rest of that arm). It is the start-time half of `s8d-ii`'s
//! run-lock liveness predicate, which `rk-cli` composes with [`is_alive`] and injects into
//! `rk-git` (ADR-0180 decision 6, proposed), so `rk-git` gains no edge to this crate. It spawns
//! nothing and creates no containment; its Windows arm is one more `ffi` site. ADR-0180 makes
//! `is_alive` a production call too; its `#[doc(hidden)]` attribute is left as it stands until the
//! `rk-cli` wiring that makes the call.
//!
//! **Lint posture.** Every override in this crate is `#[expect(clippy::…, reason = "…")]` per site
//! (ADR-0102 decision 3) — the `std::process::Command` sites, since `clippy.toml`'s
//! `disallowed-types` bans the type workspace-wide: `ffi.rs`'s spawn (one per arm), the platform
//! `spawn_into_job` (one per arm) and its Windows `#[doc(hidden)]` hook form `spawn_into_job_with`,
//! `run_bounded`'s signature and its hidden hook form `run_bounded_with`, the fixture bin's one
//! spawn helper and the test support's one builder — nine on a Windows build, listed in Slice 2's
//! commit body against the count of sites. The one `allow` is `ffi`'s commented `#![allow(unsafe_code)]`, and nothing else in
//! this crate says `unsafe`. No `HashMap`, no clock, no sleep: the only wait is an `mpsc`
//! `recv_timeout`.
//!
//! **What the primitives replace on the `Command` they are handed, so no adapter is surprised:**
//! on Windows the creation-flag word (`CREATE_SUSPENDED` — `std`'s `creation_flags` sets, it does
//! not OR, and `Command` has no getter to preserve a caller's flags through); on POSIX the process
//! group (`0`); and `run_bounded` additionally stdin (null). Nothing else. A caller sets none of
//! those; an adapter that needs `CREATE_NO_WINDOW` later gets an explicit parameter on this
//! crate's surface, never a pre-flagged `Command`.

use std::sync::atomic::{AtomicBool, Ordering};

mod bounded;
mod error;
mod ffi;

#[cfg(unix)]
pub mod posix;
#[cfg(windows)]
pub mod windows;

#[doc(hidden)]
pub use bounded::run_bounded_with;
pub use bounded::{Outcome, run_bounded};
pub use error::ContainError;
pub use ffi::CANCEL_EXIT_CODE;

#[cfg(windows)]
pub use windows::{Group, process_start_time, spawn_into_job};
#[cfg(windows)]
#[doc(hidden)]
pub use windows::{SpawnObservation, is_alive, spawn_into_job_with, terminate_pid};

#[cfg(unix)]
pub use posix::{Group, process_start_time, spawn_into_job};
#[cfg(unix)]
#[doc(hidden)]
pub use posix::{is_alive, terminate_pid};

/// A child and the group that holds it — what [`spawn_into_job`] returns (`s7c` §7c.10, RT-13).
///
/// The adapter takes the three pipes with `child.stdin/stdout/stderr.take()`, waits on `child`
/// from a blocking thread, and on its cancel arm kills **through `group`** — never
/// `Child::kill`, which would reach the direct child alone — so that `wait()` returns and no
/// descendant survives. `group` must outlive `child`'s use: dropping it is the
/// `KILL_ON_JOB_CLOSE` sweep on Windows and nothing at all on POSIX.
#[derive(Debug)]
pub struct Contained {
    pub child: std::process::Child,
    pub group: Group,
}

/// Test support: records whether a [`Group`] was killed before it was dropped.
///
/// `s7c` §7c.10: *"the kill must complete before the handle drops: `spawn_into_job`'s test-only
/// hook records kill-then-drop order, and the test asserts it, so a mutant that relies on
/// `KILL_ON_JOB_CLOSE` at drop rather than killing through the job is red too."* Obtained from
/// `Group::kill_drop_order()` as an `Arc` that outlives the group. **"Killed" means a kill that
/// completed**: `Group::kill` records it only on `Ok` — after `TerminateJobObject` succeeded and
/// every listed member was waited dead (Windows), or after `killpg` succeeded (POSIX) — so a kill
/// that failed, or a member that outlived the bounded wait, leaves `killed()` false and
/// `killed_before_drop()` false.
#[doc(hidden)]
#[derive(Debug, Default)]
pub struct KillDropOrder {
    killed: AtomicBool,
    dropped: AtomicBool,
    killed_before_drop: AtomicBool,
}

impl KillDropOrder {
    fn new() -> Self {
        Self::default()
    }

    fn record_kill(&self) {
        self.killed.store(true, Ordering::SeqCst);
    }

    fn record_drop(&self) {
        let killed = self.killed.load(Ordering::SeqCst);
        self.killed_before_drop.store(killed, Ordering::SeqCst);
        self.dropped.store(true, Ordering::SeqCst);
    }

    /// `Group::kill` has been called.
    #[must_use]
    pub fn killed(&self) -> bool {
        self.killed.load(Ordering::SeqCst)
    }

    /// The group has been dropped.
    #[must_use]
    pub fn dropped(&self) -> bool {
        self.dropped.load(Ordering::SeqCst)
    }

    /// The group was dropped AND `kill` had been called before that drop — the order AC-224's
    /// cancel path must show.
    #[must_use]
    pub fn killed_before_drop(&self) -> bool {
        self.dropped() && self.killed_before_drop.load(Ordering::SeqCst)
    }
}
