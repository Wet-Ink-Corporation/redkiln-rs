//! The Windows arm's safe surface: a [`Group`] over one Job Object, [`spawn_into_job`] and the two
//! `#[doc(hidden)]` test-support probes.
//!
//! **No `unsafe` in this file** — every OS call is `ffi::win`'s (`s7c` §7c.10: *"`windows.rs` and
//! `posix.rs` are safe wrappers that call into `ffi.rs` and carry no `unsafe`"*).
//!
//! **The spawn shape, in order, and why each step is where it is** (RT-13 as amended twice;
//! ADR-0127 decision 1; DECISIONS-2026-09-17 O-1 as amended):
//!
//! 1. **The job is created first** — `KILL_ON_JOB_CLOSE`, no `BREAKAWAY_OK` (decision 7). A job
//!    created after the spawn is what ADR-0127's forbids names first. **This order is held by
//!    construction and is not observable from outside**: with the child suspended a job created
//!    between the spawn and the assign would be functionally identical, and no reading a test can
//!    take distinguishes the two (the test-discrimination review of Slice 2, its M-B mutant);
//!    `ac224_job_object_created_before_spawn`'s discriminated property is step 3's, below.
//! 2. **The child is spawned `CREATE_SUSPENDED`** through `std`. It has a pid and a process handle
//!    and has executed nothing — it cannot have forked. It comes back as `ffi::win::Suspended`,
//!    which nothing but step 3 accepts. **The creation-flag word is this crate's**: `std`'s
//!    `creation_flags` *sets* the word, so any flags the caller put on the `Command` are replaced
//!    by `CREATE_SUSPENDED` (the `Command` has no getter to preserve them through); a caller must
//!    not set creation flags, and an adapter that needs one later gets an explicit parameter here.
//! 3. **`AssignProcessToJobObject`.** Its return is `ffi::win::Assigned`, the only type step 5
//!    accepts — so *assign before resume* is a compile-time fact of the types, not a runtime reading
//!    (the hygiene review of Slice 2: the two readings below are each true under a mutant that
//!    resumes first and assigns second, since `ResumeThread`'s count is 1 for any never-run thread
//!    whenever it is read and `IsProcessInJob` is true whenever it is read after the assign). If
//!    the assign fails the child is terminated before the error is returned: a child running
//!    outside a job is the state ADR-0099's forbids exists to prevent, and *"assigned before it
//!    can fork"* has to hold on the failure path too.
//! 4. **The two parent-side readings** (`s7c` §7c.10, both taken here and only here, before the
//!    resume): `IsProcessInJob(child, OUR job)` — true only for a member of this handle's job, so
//!    cargo's outer `KILL_ON_JOB_CLOSE` job (which every test process is inside) cannot satisfy
//!    it — and, from step 5, `ResumeThread`'s previous suspend count. **And the hook**: between the
//!    assign and the resume [`spawn_into_job_with`] hands the test the [`Group`] and the still-
//!    suspended `Child`, so the test can take its OWN readings while the child provably has not
//!    run — the job's enumeration lists the pid, and the `hold` fixture has written no marker after
//!    a second inside the job (a suspended process writes nothing; a resumed one writes within
//!    tens of milliseconds). That is the observation that goes red under the resume-then-assign
//!    mutant the type also refuses (the test-discrimination review's M-A).
//! 5. **`ResumeThread`** over a Toolhelp32 enumeration of the pid's threads, because `std` closed
//!    the primary-thread handle after `CreateProcessW`. A previous suspend count of **exactly 1**
//!    is the reading that the thread had never run; a mutant that spawns un-suspended reads 0
//!    here or never calls it (`ac224_job_object_created_before_spawn`).
//!
//! The readings are kept on the [`Group`] behind [`Group::spawn_observation`], `#[doc(hidden)]`,
//! so `Contained { child, group }` keeps the two-field shape `s7c` §7c.10 names. **No child-side
//! line exists** (§7c.10's reason: under `cargo test` a child inherits cargo's job and reads
//! `KILL_ON_JOB_CLOSE` true whatever this crate did, so a child-side reading discriminates
//! nothing).
//!
//! **Kill, then drop, in that order.** [`Group::kill`] is `TerminateJobObject` plus a bounded wait
//! on every member the job lists, so that when it returns every member is dead — not merely
//! terminating — and a caller that reads `is_alive` afterwards reads the truth. **It blocks**, up
//! to `KILL_WAIT_EACH_MS` per member, so an async caller (Slice 5's adapter) calls it from
//! `spawn_blocking`. The handle is closed on drop, which is `KILL_ON_JOB_CLOSE`'s sweep of
//! anything a kill did not reach; a caller that relied on the drop *instead of* the kill would be
//! depending on a sweep that runs after its own return, which is why the [`KillDropOrder`] probe
//! records whether a kill **completed** before the drop and `s7c` §7c.10's cancel test asserts it.
//!
//! **Enumeration is via the job** — `QueryInformationJobObject(JobObjectBasicProcessIdList)` —
//! never a PID walk from the child (ADR-0127's forbids; F22's survivor is what a walk misses).

use std::sync::Arc;

use crate::error::ContainError;
use crate::ffi::win;
use crate::{Contained, KillDropOrder};

pub use crate::ffi::CANCEL_EXIT_CODE;

/// How long [`Group::kill`] waits for each member the job lists after `TerminateJobObject` before
/// reporting it as a survivor. Ten seconds is far above anything a terminated process needs and
/// far below any test's own bound; a member still alive after it is an error, never a silence.
const KILL_WAIT_EACH_MS: u32 = 10_000;

/// The two parent-side readings `spawn_into_job` takes before it resumes the child
/// (`s7c` §7c.10; `ac224_job_object_created_before_spawn`). Test support — hidden.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpawnObservation {
    /// `IsProcessInJob(child, our_job)` read after the assign and before the resume.
    pub in_our_job_before_resume: bool,
    /// `ResumeThread`'s return for the child's primary thread — the previous suspend count, 1 for a
    /// thread that has never run.
    pub previous_suspend_count: u32,
    /// How many threads owned by the pid the resume site found and resumed — 1 for a fresh process.
    pub threads_resumed: u32,
}

/// One Job Object, held for the child's lifetime. Dropping it closes the handle, which is the
/// `KILL_ON_JOB_CLOSE` sweep.
pub struct Group {
    job: win::JobHandle,
    order: Arc<KillDropOrder>,
    observation: SpawnObservation,
}

impl std::fmt::Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("observation", &self.observation)
            .field("order", &self.order)
            .finish_non_exhaustive()
    }
}

impl Group {
    /// Kill every member through the job with [`CANCEL_EXIT_CODE`] and wait for each to be gone.
    /// **Blocks** up to `KILL_WAIT_EACH_MS` per member; an async caller runs it on a blocking
    /// thread. The [`KillDropOrder`] probe records the kill only when it completed — on an `Err`
    /// nothing is recorded, so `killed_before_drop()` never reads true for a kill that did not
    /// finish.
    ///
    /// # Errors
    /// [`ContainError::Kill`] if `TerminateJobObject` fails, a member could not be opened for a
    /// reason other than being gone, a wait fails, or a member outlives the bounded wait;
    /// [`ContainError::Enumerate`] if the members could not be listed for the wait.
    pub fn kill(&self) -> Result<(), ContainError> {
        win::terminate_job(&self.job, CANCEL_EXIT_CODE, KILL_WAIT_EACH_MS).map_err(
            |e| match e {
                win::TerminateFailure::Kill(e) => ContainError::Kill(e),
                win::TerminateFailure::Enumerate(e) => ContainError::Enumerate(e),
            },
        )?;
        self.order.record_kill();
        Ok(())
    }

    /// The pids the job currently holds — enumeration **via the job**, which is how an orphaned
    /// grandchild (parent exited) is still found (AC-224, AC-278).
    ///
    /// # Errors
    /// [`ContainError::Enumerate`] if the query fails or the job holds more members than one
    /// enumeration can carry.
    pub fn pids(&self) -> Result<Vec<u32>, ContainError> {
        win::job_pids(&self.job).map_err(ContainError::Enumerate)
    }

    /// The readings taken at spawn (test support).
    #[doc(hidden)]
    #[must_use]
    pub const fn spawn_observation(&self) -> SpawnObservation {
        self.observation
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
        // `self.job` drops after this body and closes the handle — the KILL_ON_JOB_CLOSE sweep.
    }
}

/// Create the job, spawn the command suspended into it, assign, read, resume — the module doc's
/// five steps — and hand back the child with the group that holds it.
///
/// **What this does to the `Command`:** its creation flags are replaced by `CREATE_SUSPENDED`
/// (`std`'s `creation_flags` sets the word and `Command` has no getter — the module doc, step 2);
/// nothing else on it is touched. A caller must not set creation flags.
///
/// # Errors
/// [`ContainError::Create`], [`ContainError::Spawn`], [`ContainError::Assign`] or
/// [`ContainError::Resume`]; on the last two the child has already been terminated.
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0127 decision 2: the public spawn-into-the-group primitive takes the std Command the \
              adapter built and hands it to the one spawn site in ffi.rs, which owns the creation-flag \
              word (CREATE_SUSPENDED replaces whatever the caller set) and touches nothing else on it. \
              clippy.toml's disallowed-types has no path scoping, so the exemption is stated at the \
              narrowest item that needs it."
)]
pub fn spawn_into_job(cmd: std::process::Command) -> Result<Contained, ContainError> {
    spawn_into_job_with(cmd, |_, _| {})
}

/// Test support: [`spawn_into_job`] with a hook that runs **after the assign and before the
/// resume**, handed the [`Group`] and the still-suspended child — so a test can take readings of
/// its own while the child provably has not executed an instruction (the module doc, step 4).
/// `spawn_into_job` is this with an empty hook.
///
/// # Errors
/// As [`spawn_into_job`].
#[doc(hidden)]
#[expect(
    clippy::disallowed_types,
    reason = "the hidden test-support form of spawn_into_job: same Command, same one spawn site in \
              ffi.rs, same creation-flag ownership; the hook is the only difference."
)]
pub fn spawn_into_job_with(
    cmd: std::process::Command,
    between_assign_and_resume: impl FnOnce(&Group, &std::process::Child),
) -> Result<Contained, ContainError> {
    // 1. the job, before the child exists
    let job = win::create_job_kill_on_close().map_err(ContainError::Create)?;
    // 2. the child, suspended — a `Suspended`, which only `assign` accepts
    let suspended = win::spawn_suspended(cmd).map_err(ContainError::Spawn)?;
    // 3. assigned before it can fork — an `Assigned`, which only `resume_threads` accepts; on
    //    failure the child has been terminated inside `assign`
    let assigned = win::assign(&job, suspended).map_err(ContainError::Assign)?;
    // 4. reading (1), before the resume; on failure `assigned` drops and terminates the child
    let in_our_job_before_resume = win::is_in_job(&job, &assigned).map_err(ContainError::Assign)?;
    let mut group = Group {
        job,
        order: Arc::new(KillDropOrder::new()),
        observation: SpawnObservation {
            in_our_job_before_resume,
            previous_suspend_count: 0,
            threads_resumed: 0,
        },
    };
    between_assign_and_resume(&group, assigned.child());
    // 5. the resume, and reading (2); on failure the child has been terminated inside
    let (child, resumed) = win::resume_threads(assigned).map_err(ContainError::Resume)?;
    group.observation.previous_suspend_count = resumed.previous_suspend_count;
    group.observation.threads_resumed = resumed.threads;
    Ok(Contained { child, group })
}

/// Test support: terminate one process by pid with `code` — AC-225's external kill, from outside
/// the adapter, with the same [`CANCEL_EXIT_CODE`]. Spawns nothing, creates no containment.
///
/// # Errors
/// [`ContainError::Kill`] if the process cannot be opened or terminated.
#[doc(hidden)]
pub fn terminate_pid(pid: u32, code: u32) -> Result<(), ContainError> {
    win::terminate_pid(pid, code).map_err(ContainError::Kill)
}

/// Test support: is the process with this pid still running — read from the process table, never
/// from any job, so a survivor of a mutant containment is visible to it.
#[doc(hidden)]
#[must_use]
pub fn is_alive(pid: u32) -> bool {
    win::is_alive(pid)
}
