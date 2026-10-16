//! `ffi` — the ONE module in the workspace under a commented `#![allow(unsafe_code)]`, holding
//! **both** containment arms: the Windows Job Object and the POSIX process group.
//!
//! **Why one module holds two platforms** (`s7c` §7c.10): `libc::killpg` and `libc::kill`
//! are `unsafe extern "C"` calls too, so a `posix.rs` carrying them would need its own `allow` —
//! which ADR-0102 decision 2 forbids (*"the only `allow` permitted in source"*) and Checkpoint 7c's
//! exactly-one count would catch. So the POSIX arm lives here under `#[cfg(unix)]`, beside the
//! Windows arm under `#[cfg(windows)]`, and `windows.rs` / `posix.rs` are **safe wrappers** that
//! call in and carry no `unsafe` of their own (the block greps the token over `src/` and permits
//! it in this file only).
//!
//! **Every `unsafe` site is a call into the binding layer ADR-0127 decision 5 names** — `windows-sys`
//! 0.61.2 on Windows, `libc` 0.2.189 on Unix, both exact-pinned — or an `impl Send`/`Sync` on the
//! kernel-object handle those calls return; nothing here dereferences a pointer the OS did not
//! hand back through a documented out-parameter. The site list, read against the count Slice 2's
//! commit body carries (`s7c` §7c.20 item 2 — *"spawn, assign, resume, kill, enumerate,
//! `terminate_pid`, `is_alive`"*):
//!
//! | site | Windows call | what it is for |
//! | --- | --- | --- |
//! | create | `CreateJobObjectW` + `SetInformationJobObject(JobObjectExtendedLimitInformation)` with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` | the group, **created before the spawn** (ADR-0099 d3 as 0127 keeps it) |
//! | spawn | none — `std`'s `CommandExt::creation_flags(CREATE_SUSPENDED)` | the child exists but has not run an instruction (RT-13 as amended); it comes back as a `win::Suspended`, a type only `assign` accepts |
//! | assign | `AssignProcessToJobObject` | the suspended child joins the job **before it can fork**; the return is a `win::Assigned`, the only type `resume_threads` accepts — so *resume before assign* is a compile error, not a reading |
//! | reading | `IsProcessInJob(child, our_job)` | the parent-side discriminator (1) of `ac224_job_object_created_before_spawn`, taken against OUR handle so cargo's outer job cannot satisfy it (§7c.10) |
//! | resume | `CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD)` / `Thread32First` / `Thread32Next` / `OpenThread(THREAD_SUSPEND_RESUME)` / `ResumeThread` | `std` closes the primary-thread handle after `CreateProcessW`, so it is re-opened by pid; `ResumeThread`'s return is discriminator (2), the previous suspend count, exactly 1; a `Thread32First`/`Thread32Next` FALSE is read against `ERROR_NO_MORE_FILES`, so a failed walk is its own error and never *"nothing to resume"* |
//! | kill | `TerminateJobObject` with [`CANCEL_EXIT_CODE`], then `OpenProcess(PROCESS_SYNCHRONIZE)` + `IsProcessInJob` + `WaitForSingleObject` per member | the tree dies **through the job**, never a PID walk, and `kill` returns only when every member has; a listed pid that cannot be opened is *gone* only on `ERROR_INVALID_PARAMETER`, a pid the OS re-used is not waited on (the membership read), and `WAIT_FAILED` carries its own OS error |
//! | enumerate | `QueryInformationJobObject(JobObjectBasicProcessIdList)` | AC-224's *"enumerating the group via the job"*; AC-278's external enumeration |
//! | drop | `CloseHandle` | the last handle closes and `KILL_ON_JOB_CLOSE` sweeps whatever a kill did not reach |
//! | `terminate_pid` | `OpenProcess(PROCESS_TERMINATE)` + `TerminateProcess` | test support, `#[doc(hidden)]`: AC-225's external kill with the SAME code, from outside the adapter |
//! | `is_alive` | `OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION)` + `GetExitCodeProcess == STILL_ACTIVE` | test support, `#[doc(hidden)]`: the **job-independent** liveness probe the orphan tests read, so a mutant with no job cannot hide a survivor from it |
//! | `process_start_time` | `OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION)` + `GetExitCodeProcess == STILL_ACTIVE` + `GetProcessTimes` | the start-time half of `s8d-ii`'s run-lock liveness predicate (ADR-0180 decision 6, proposed): a lock is live only when its pid is alive AND that pid's start time equals the recorded one. A production read, spawns nothing *(dated 2026-09-27, s8d-ii build, P3)* |
//!
//! The POSIX arm's sites: spawn (`std`'s `CommandExt::process_group(0)`), kill (`killpg(pgid,
//! SIGKILL)`), `terminate_pid` (`kill(pid, SIGKILL)`) and `is_alive` (`kill(pid, 0)`). *(Dated
//! 2026-10-16, issue #20: the emptiness probe `kill(-pgid, 0)` was removed; it counted a zombie
//! member, and `posix.rs`'s `Group::is_empty` reads its zombie-filtered `/proc` listing instead.)*
//! Enumeration by pid on POSIX has no syscall; `posix.rs` reads `/proc` for it and this module carries only the signal calls. **The POSIX arm is written, not
//! run: this box is Windows and the arm is ARMED to a recorded CI run** (`s7c` §7c.10; `ci.yml`
//! has never run).
//!
//! [`CANCEL_EXIT_CODE`] is declared here because ADR-0127 decision 1 puts *"the adapter's own
//! `TerminateProcess` exit code"* as *"a named constant in this module"*; `windows.rs` re-exports it
//! at the path `s7c` §7c.7 names (`rk_containment::windows::CANCEL_EXIT_CODE`). Its value is
//! chosen at Slice 2 (§7c.7: *"UNCONFIRMED until then — no record fixes it"*) and recorded in the
//! commit body; the classifier is keyed on the token, never on the code (AC-225), so the value
//! carries no decision — it only has to be the adapter's own and shared by AC-225's positive and
//! negative controls (PIA-059).
//!
//! **What `std` does and does not keep on Windows, measured, not assumed** (`s7c` §7c.10, RT-13):
//! `Command::spawn` calls `CreateProcessW` and closes `PROCESS_INFORMATION.hThread` right after,
//! keeping only the process handle a `Child` wraps — so a suspended child cannot be resumed
//! through `std`, and the resume site re-opens the primary thread by walking a Toolhelp32 thread
//! snapshot for `th32OwnerProcessID == pid`. A freshly created suspended process has exactly one
//! thread; the resume site resumes every thread it finds and reports the count, so a reading of
//! more than one is visible rather than silently folded.

// ADR-0102 decision 2's one permitted `allow` — the commented `#![allow(unsafe_code)]` exception to the
// workspace's `unsafe_code = "deny"` — relocated from `rk-adapter-claude` to this crate by ADR-0127
// decision 4 (*"the one commented `#![allow(unsafe_code)]` moves crate"*): this module, and no other file
// in `crates/`, may carry `unsafe`. Checkpoint 7c counts exactly one such attribute workspace-wide and
// asserts it is this one, in the module-level form, with this comment on the line above it.
#![allow(unsafe_code)]

/// The exit code the adapter's own kill hands `TerminateJobObject` / `TerminateProcess` on Windows
/// (ADR-0127 decision 1; AC-225; PIA-059).
///
/// `0x524B` is the ASCII pair `R K`, 21067 in decimal — small enough that `ExitStatus::code()`
/// reports it as a positive `i32`, and nobody else's: not an `NTSTATUS`, not a `taskkill` code
/// (1), not `STATUS_CONTROL_C_EXIT`. On POSIX the group is killed with `SIGKILL` and there is no
/// code to choose; the constant is declared unconditionally so a cross-platform caller can name
/// it.
pub const CANCEL_EXIT_CODE: u32 = 0x524B;

#[cfg(windows)]
pub mod win {
    //! The Windows arm. Every function here is a safe `fn` whose body holds the `unsafe` call.

    use std::io;
    use std::os::windows::io::AsRawHandle;
    use std::os::windows::process::CommandExt;

    use windows_sys::Win32::Foundation::{
        CloseHandle, ERROR_INVALID_PARAMETER, ERROR_MORE_DATA, ERROR_NO_MORE_FILES, FILETIME,
        HANDLE, INVALID_HANDLE_VALUE, STILL_ACTIVE, WAIT_FAILED, WAIT_OBJECT_0,
    };
    use windows_sys::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First, Thread32Next,
    };
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, IsProcessInJob,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOBOBJECT_BASIC_LIMIT_INFORMATION,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectBasicProcessIdList,
        JobObjectExtendedLimitInformation, QueryInformationJobObject, SetInformationJobObject,
        TerminateJobObject,
    };
    use windows_sys::Win32::System::Threading::{
        CREATE_SUSPENDED, GetExitCodeProcess, GetProcessTimes, IO_COUNTERS, OpenProcess,
        OpenThread, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE,
        ResumeThread, THREAD_SUSPEND_RESUME, TerminateProcess, WaitForSingleObject,
    };

    /// An owned Job Object handle. Closing it is the `KILL_ON_JOB_CLOSE` sweep, so the
    /// [`Drop`] here IS ADR-0099 decision 3's *"the handle held for the child's lifetime"* — the
    /// value that owns it must outlive the child, and `windows::Group` does.
    ///
    /// A kernel-object handle is a process-wide token, valid from any thread; the `Send`/`Sync`
    /// impls below say so, and they are the only non-call `unsafe` in the crate.
    pub struct JobHandle(HANDLE);

    // SAFETY: a Job Object HANDLE is a process-wide kernel handle with no thread affinity; every call
    // on it here is a documented kernel32 entry point that takes the handle by value.
    unsafe impl Send for JobHandle {}
    // SAFETY: as above — the handle is a token, not a pointer to memory this crate reads.
    unsafe impl Sync for JobHandle {}

    impl JobHandle {
        pub const fn raw(&self) -> HANDLE {
            self.0
        }
    }

    impl Drop for JobHandle {
        fn drop(&mut self) {
            // SAFETY: the handle was returned by CreateJobObjectW and is closed exactly once, here.
            unsafe {
                CloseHandle(self.0);
            }
        }
    }

    /// A process handle opened by pid, closed on drop.
    struct ProcessHandle(HANDLE);

    impl Drop for ProcessHandle {
        fn drop(&mut self) {
            // SAFETY: the handle was returned by OpenProcess and is closed exactly once, here.
            unsafe {
                CloseHandle(self.0);
            }
        }
    }

    fn open_process(access: u32, pid: u32) -> Option<ProcessHandle> {
        // SAFETY: OpenProcess takes plain integers and returns a handle or null; no pointer crosses.
        let h = unsafe { OpenProcess(access, 0, pid) };
        if h.is_null() {
            None
        } else {
            Some(ProcessHandle(h))
        }
    }

    /// **create**: a Job Object whose extended limit carries `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`
    /// and — deliberately — **not** `JOB_OBJECT_LIMIT_BREAKAWAY_OK` (ADR-0127 decision 7: the job
    /// never grants breakaway, which is what makes the fixture's `CREATE_BREAKAWAY_FROM_JOB`
    /// attempt the negative control that must fail).
    pub fn create_job_kill_on_close() -> io::Result<JobHandle> {
        // SAFETY: null attributes (default security, not inheritable) and a null name (anonymous
        // job) are the documented arguments; the return is a handle or null.
        let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if job.is_null() {
            return Err(io::Error::last_os_error());
        }
        let job = JobHandle(job);
        let info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
            BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION {
                PerProcessUserTimeLimit: 0,
                PerJobUserTimeLimit: 0,
                LimitFlags: JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
                MinimumWorkingSetSize: 0,
                MaximumWorkingSetSize: 0,
                ActiveProcessLimit: 0,
                Affinity: 0,
                PriorityClass: 0,
                SchedulingClass: 0,
            },
            IoInfo: IO_COUNTERS {
                ReadOperationCount: 0,
                WriteOperationCount: 0,
                OtherOperationCount: 0,
                ReadTransferCount: 0,
                WriteTransferCount: 0,
                OtherTransferCount: 0,
            },
            ProcessMemoryLimit: 0,
            JobMemoryLimit: 0,
            PeakProcessMemoryUsed: 0,
            PeakJobMemoryUsed: 0,
        };
        let len = u32::try_from(std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>())
            .map_err(|_| io::Error::other("JOBOBJECT_EXTENDED_LIMIT_INFORMATION exceeds u32"))?;
        // SAFETY: `info` is a fully initialised #[repr(C)] value of exactly the class's documented
        // type and `len` is its size; the pointer is valid for the duration of the call.
        let ok = unsafe {
            SetInformationJobObject(
                job.raw(),
                JobObjectExtendedLimitInformation,
                std::ptr::from_ref(&info).cast(),
                len,
            )
        };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(job)
    }

    /// A child that exists — pid, process handle — and has executed nothing: the value
    /// [`spawn_suspended`] returns and the only value [`assign`] accepts. It is not a `Child` on
    /// purpose: nothing can `wait()` on it, and nothing can resume it, until it has been assigned.
    ///
    /// Dropping it with the child still inside (an `assign` never reached) terminates the child: a
    /// suspended process outside a job is not a state this crate leaves behind. No `unsafe`: the
    /// `Option` is how the one owner moves the `Child` on without running that `Drop`.
    #[derive(Debug)]
    pub struct Suspended(Option<std::process::Child>);

    impl Drop for Suspended {
        fn drop(&mut self) {
            if let Some(mut child) = self.0.take() {
                // A suspended child nobody assigned: terminate and reap. Either call failing means
                // the process is already gone, which is the state wanted.
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    /// A child that is **in the job and still suspended** — the value [`assign`] returns and the
    /// only value [`resume_threads`] accepts. The order *assign, then resume* is therefore a type,
    /// not a reading: a caller cannot resume what it has not assigned (ADR-0127 decision 1's
    /// *"assigned before it can fork"*; the hygiene review of Slice 2).
    ///
    /// Dropping it with the child still inside (a resume never reached) terminates the child.
    #[derive(Debug)]
    pub struct Assigned(Option<std::process::Child>);

    impl Assigned {
        /// The child, borrowed — a test-support hook reads its pid while it is still suspended.
        pub const fn child(&self) -> &std::process::Child {
            self.0
                .as_ref()
                .expect("an Assigned holds its child until resume_threads takes it")
        }
    }

    impl Drop for Assigned {
        fn drop(&mut self) {
            if let Some(mut child) = self.0.take() {
                // An assigned child nobody resumed: terminate and reap (see `Suspended`).
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    /// **spawn**: the child is created `CREATE_SUSPENDED` through `std`, so it exists — and has a
    /// pid and a process handle — before it has run an instruction. No `unsafe` here: the flag is a
    /// plain `u32` `std` takes.
    ///
    /// **The creation-flag word is this crate's.** `CommandExt::creation_flags` *sets* the word
    /// (it does not OR into it, and `Command` has no getter), so whatever flags the caller set on
    /// the `Command` — `CREATE_NO_WINDOW`, `CREATE_NEW_PROCESS_GROUP`, anything — are **replaced**
    /// by `CREATE_SUSPENDED` here. A caller must not set creation flags; an adapter that needs one
    /// later gets an explicit parameter on this crate's surface, never a `Command` it is trusted to
    /// have pre-flagged (the spec-and-ffi review of Slice 2).
    ///
    /// The `Command` type is clippy-banned workspace-wide (`clippy.toml`, Checkpoint 6's type ban)
    /// and this is one of the crate's stated `#[expect]` sites (`s7c` §7c.19, §7c.20 item 2).
    #[expect(
        clippy::disallowed_types,
        reason = "ADR-0127 decision 2/3: this is the ONE spawn site through which every child of every \
                  spawning adapter is created on Windows — suspended, so it can be assigned to the job \
                  before it can fork (RT-13 as amended). clippy.toml's disallowed-types has no path \
                  scoping, so the exemption is stated at the narrowest item that needs it."
    )]
    pub fn spawn_suspended(mut cmd: std::process::Command) -> io::Result<Suspended> {
        cmd.creation_flags(CREATE_SUSPENDED)
            .spawn()
            .map(|child| Suspended(Some(child)))
    }

    /// **assign**: the suspended child joins the job. Under `cargo test` this is a nested-job
    /// assign (the process is already in cargo's `KILL_ON_JOB_CLOSE` job — `s7c` §7c.10), which
    /// Windows 8+ permits.
    ///
    /// On failure the child is terminated before the error is returned (`Suspended`'s drop): a
    /// child running outside a job is the state ADR-0099's forbids exists to prevent, and
    /// *"assigned before it can fork"* has to hold on the failure path too.
    pub fn assign(job: &JobHandle, mut child: Suspended) -> io::Result<Assigned> {
        let handle = child
            .0
            .as_ref()
            .expect("a Suspended holds its child until assign takes it")
            .as_raw_handle();
        // SAFETY: both handles are live — the job is owned by `job`, the process by `child`.
        let ok = unsafe { AssignProcessToJobObject(job.raw(), handle) };
        if ok == 0 {
            let err = io::Error::last_os_error();
            drop(child);
            return Err(err);
        }
        Ok(Assigned(child.0.take()))
    }

    /// **reading (1)**: `IsProcessInJob(child, OUR job)` — true only for a process assigned to
    /// exactly this job, so cargo's outer job cannot satisfy it (`s7c` §7c.10). Takes an
    /// [`Assigned`], so the reading is by construction after the assign and before the resume.
    pub fn is_in_job(job: &JobHandle, child: &Assigned) -> io::Result<bool> {
        process_in_job(child.child().as_raw_handle(), job)
    }

    /// `IsProcessInJob(handle, job)` over any live process handle.
    fn process_in_job(process: HANDLE, job: &JobHandle) -> io::Result<bool> {
        let mut result: i32 = 0;
        // SAFETY: both handles are live and `result` is a valid out-pointer for the call's duration.
        let ok = unsafe { IsProcessInJob(process, job.raw(), &raw mut result) };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(result != 0)
    }

    /// What **resume** observed: how many threads owned by the pid it resumed, and the previous
    /// suspend count `ResumeThread` returned for the first of them — reading (2) of
    /// `ac224_job_object_created_before_spawn`, exactly 1 for a primary thread that has never run.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Resumed {
        pub threads: u32,
        pub previous_suspend_count: u32,
    }

    /// **resume**: re-open the child's threads by pid through a Toolhelp32 snapshot and resume each.
    /// Only an [`Assigned`] child can be resumed; the resumed child comes back as the ordinary
    /// `std::process::Child` the adapter waits on. On failure the child is terminated before the
    /// error is returned (`Assigned`'s drop).
    pub fn resume_threads(mut child: Assigned) -> io::Result<(std::process::Child, Resumed)> {
        let resumed = resume_threads_of(child.child().id())?;
        let child = child
            .0
            .take()
            .expect("an Assigned holds its child until resume_threads takes it");
        Ok((child, resumed))
    }

    /// The resume by pid — the walk `resume_threads` performs once the type has said the child is
    /// assigned. Private: nothing outside this module may resume by pid.
    fn resume_threads_of(pid: u32) -> io::Result<Resumed> {
        // SAFETY: TH32CS_SNAPTHREAD ignores the pid argument and returns a snapshot handle or
        // INVALID_HANDLE_VALUE.
        let snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
        if snap == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let snap = ProcessHandle(snap);
        let size = u32::try_from(std::mem::size_of::<THREADENTRY32>())
            .map_err(|_| io::Error::other("THREADENTRY32 exceeds u32"))?;
        let mut entry = THREADENTRY32 {
            dwSize: size,
            cntUsage: 0,
            th32ThreadID: 0,
            th32OwnerProcessID: 0,
            tpBasePri: 0,
            tpDeltaPri: 0,
            dwFlags: 0,
        };
        // SAFETY: `entry.dwSize` is set as the API requires and the pointer is valid for the call.
        let mut more = walk_step(unsafe { Thread32First(snap.0, &raw mut entry) })?;
        let mut threads = 0u32;
        let mut first_count: Option<u32> = None;
        while more {
            if entry.th32OwnerProcessID == pid {
                // SAFETY: plain-integer arguments; returns a thread handle or null.
                let th = unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, entry.th32ThreadID) };
                if th.is_null() {
                    return Err(io::Error::last_os_error());
                }
                let th = ProcessHandle(th);
                // SAFETY: a live thread handle opened with THREAD_SUSPEND_RESUME.
                let previous = unsafe { ResumeThread(th.0) };
                if previous == u32::MAX {
                    return Err(io::Error::last_os_error());
                }
                threads += 1;
                first_count.get_or_insert(previous);
            }
            // SAFETY: as for Thread32First; the entry is reused as the API documents.
            more = walk_step(unsafe { Thread32Next(snap.0, &raw mut entry) })?;
        }
        first_count.map_or_else(
            || {
                Err(io::Error::other(format!(
                    "no thread owned by pid {pid} in the Toolhelp32 snapshot — nothing to resume"
                )))
            },
            |previous_suspend_count| {
                Ok(Resumed {
                    threads,
                    previous_suspend_count,
                })
            },
        )
    }

    /// One `Thread32First` / `Thread32Next` return, read: TRUE is *an entry*; FALSE is *the end*
    /// only when `GetLastError` is `ERROR_NO_MORE_FILES`, and any other code is the walk's own
    /// failure — never folded into the *"no thread owned by pid"* message.
    fn walk_step(ok: i32) -> io::Result<bool> {
        if ok != 0 {
            return Ok(true);
        }
        let err = io::Error::last_os_error();
        if err.raw_os_error() == i32::try_from(ERROR_NO_MORE_FILES).ok() {
            Ok(false)
        } else {
            Err(err)
        }
    }

    /// Why a kill did not complete — so the safe wrapper can say which primitive failed
    /// (`ContainError::Kill` for the terminate and the wait, `ContainError::Enumerate` for the
    /// member list the wait needs).
    #[derive(Debug)]
    pub enum TerminateFailure {
        /// `TerminateJobObject` failed, a member could not be opened for a reason other than
        /// *gone*, a wait failed, or a member outlived the bounded wait.
        Kill(io::Error),
        /// The members could not be listed for the wait.
        Enumerate(io::Error),
    }

    /// **kill**: `TerminateJobObject` — every member dies with `code`, never a PID walk — and then
    /// a bounded `WaitForSingleObject` on each member the job still lists, so that when this
    /// returns the members are terminated, not merely terminating (`TerminateProcess` is
    /// asynchronous by contract). The wait bound is generous and per member; a member that
    /// outlives it is reported, never ignored. **This blocks**, up to `wait_each_ms` per member.
    ///
    /// Three readings on the way, each named so nothing is folded: a listed pid whose `OpenProcess`
    /// fails is *already gone* only on `ERROR_INVALID_PARAMETER` (the code for a pid that no longer
    /// names a process) — `ERROR_ACCESS_DENIED` or anything else is returned; a pid that opens is
    /// waited on only if `IsProcessInJob` still says it is OUR job's, so a pid the OS re-used
    /// between the list and the open is never waited on as if it were a member; and
    /// `WaitForSingleObject`'s `WAIT_FAILED` carries `GetLastError`, where `WAIT_TIMEOUT` is the
    /// survivor message.
    pub fn terminate_job(
        job: &JobHandle,
        code: u32,
        wait_each_ms: u32,
    ) -> Result<(), TerminateFailure> {
        // SAFETY: a live job handle and a plain exit code.
        let ok = unsafe { TerminateJobObject(job.raw(), code) };
        if ok == 0 {
            return Err(TerminateFailure::Kill(io::Error::last_os_error()));
        }
        for pid in job_pids(job).map_err(TerminateFailure::Enumerate)? {
            let Some(h) =
                open_process(PROCESS_SYNCHRONIZE | PROCESS_QUERY_LIMITED_INFORMATION, pid)
            else {
                let err = io::Error::last_os_error();
                if err.raw_os_error() == i32::try_from(ERROR_INVALID_PARAMETER).ok() {
                    // Already gone: the pid no longer names a process, the outcome wanted.
                    continue;
                }
                return Err(TerminateFailure::Kill(err));
            };
            if !process_in_job(h.0, job).map_err(TerminateFailure::Kill)? {
                // The pid was re-used by a process outside our job between the list and the open;
                // the member it named is gone.
                continue;
            }
            // SAFETY: a live process handle opened with SYNCHRONIZE; the wait is bounded.
            let waited = unsafe { WaitForSingleObject(h.0, wait_each_ms) };
            if waited == WAIT_FAILED {
                return Err(TerminateFailure::Kill(io::Error::last_os_error()));
            }
            if waited != WAIT_OBJECT_0 {
                return Err(TerminateFailure::Kill(io::Error::other(format!(
                    "pid {pid} did not terminate within {wait_each_ms} ms of TerminateJobObject \
                     (WaitForSingleObject returned {waited:#x})"
                ))));
            }
        }
        Ok(())
    }

    /// The fixed capacity of one enumeration. A job this crate creates holds one child and its
    /// descendants; a job listing more processes than this is reported as an error, never
    /// truncated to a clean-looking list (Halt 8).
    const PID_LIST_CAP: usize = 1024;

    /// `JOBOBJECT_BASIC_PROCESS_ID_LIST` with room for [`PID_LIST_CAP`] entries — the header's two
    /// `u32`s followed by the flexible `usize` array, laid out as the OS writes it.
    #[repr(C)]
    struct PidList {
        number_of_assigned_processes: u32,
        number_of_process_ids_in_list: u32,
        process_id_list: [usize; PID_LIST_CAP],
    }

    /// **enumerate**: the pids currently in the job, read from the job itself
    /// (`QueryInformationJobObject(JobObjectBasicProcessIdList)`).
    pub fn job_pids(job: &JobHandle) -> io::Result<Vec<u32>> {
        let mut list = PidList {
            number_of_assigned_processes: 0,
            number_of_process_ids_in_list: 0,
            process_id_list: [0; PID_LIST_CAP],
        };
        let len = u32::try_from(std::mem::size_of::<PidList>())
            .map_err(|_| io::Error::other("PidList exceeds u32"))?;
        let mut returned: u32 = 0;
        // SAFETY: `list` is a #[repr(C)] buffer whose leading fields match the documented class
        // layout, `len` is its size, and both pointers are valid for the call's duration.
        let ok = unsafe {
            QueryInformationJobObject(
                job.raw(),
                JobObjectBasicProcessIdList,
                std::ptr::from_mut(&mut list).cast(),
                len,
                &raw mut returned,
            )
        };
        if ok == 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == i32::try_from(ERROR_MORE_DATA).ok() {
                return Err(io::Error::other(format!(
                    "the job lists {} processes, more than the {PID_LIST_CAP} this enumeration holds",
                    list.number_of_assigned_processes
                )));
            }
            return Err(err);
        }
        let n = usize::try_from(list.number_of_process_ids_in_list)
            .map_err(|_| io::Error::other("process id count exceeds usize"))?;
        list.process_id_list
            .iter()
            .take(n.min(PID_LIST_CAP))
            .map(|&p| {
                u32::try_from(p).map_err(|_| io::Error::other(format!("pid {p} exceeds u32")))
            })
            .collect()
    }

    /// **`terminate_pid`** (test support): an external kill of one process by pid with the given
    /// code — AC-225's negative control, the same code from outside the adapter.
    pub fn terminate_pid(pid: u32, code: u32) -> io::Result<()> {
        let h = open_process(PROCESS_TERMINATE, pid).ok_or_else(io::Error::last_os_error)?;
        // SAFETY: a live process handle opened with PROCESS_TERMINATE and a plain exit code.
        let ok = unsafe { TerminateProcess(h.0, code) };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// **`is_alive`** (test support): `OpenProcess` + `GetExitCodeProcess == STILL_ACTIVE`, reading
    /// the process table and never any job — the probe a mutant with no job cannot fool. A pid
    /// that cannot be opened is reported dead; a process that exits with code 259 (`STILL_ACTIVE`'s
    /// value) would read alive, the documented ambiguity of this API, and no fixture exits 259.
    pub fn is_alive(pid: u32) -> bool {
        let Some(h) = open_process(PROCESS_QUERY_LIMITED_INFORMATION, pid) else {
            return false;
        };
        let mut code: u32 = 0;
        // SAFETY: a live process handle with query access and a valid out-pointer.
        let ok = unsafe { GetExitCodeProcess(h.0, &raw mut code) };
        ok != 0 && code == STILL_ACTIVE.unsigned_abs()
    }

    /// **`process_start_time`** (`s8d-ii` §8d-ii.2; ADR-0180 decision 6, proposed): the creation
    /// time `GetProcessTimes` reports for a LIVE process, in 100 ns units since 1601 (the
    /// `FILETIME` read as one `u64`). `None` when the pid cannot be opened or its process has
    /// exited: an exited process whose handle someone still holds reads nothing, as it does for
    /// [`is_alive`], and so does `STILL_ACTIVE`'s documented ambiguity (an exit code of 259 reads
    /// live, and its start time is returned, which keeps a stale lock held: fail-closed). The
    /// start time is what tells a reused pid from the process that took a lock.
    /// *(dated 2026-09-27, s8d-ii build, P3.)*
    pub fn process_start_time(pid: u32) -> Option<u64> {
        let h = open_process(PROCESS_QUERY_LIMITED_INFORMATION, pid)?;
        let mut code: u32 = 0;
        // SAFETY: a live process handle with query access and a valid out-pointer.
        let ok = unsafe { GetExitCodeProcess(h.0, &raw mut code) };
        if ok == 0 || code != STILL_ACTIVE.unsigned_abs() {
            return None;
        }
        let zero = || FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let (mut created, mut exited, mut kernel, mut user) = (zero(), zero(), zero(), zero());
        // SAFETY: a live process handle with PROCESS_QUERY_LIMITED_INFORMATION (which
        // GetProcessTimes accepts) and four valid out-pointers to FILETIMEs this frame owns.
        let ok = unsafe {
            GetProcessTimes(
                h.0,
                &raw mut created,
                &raw mut exited,
                &raw mut kernel,
                &raw mut user,
            )
        };
        (ok != 0)
            .then(|| (u64::from(created.dwHighDateTime) << 32) | u64::from(created.dwLowDateTime))
    }
}

#[cfg(unix)]
pub mod posix {
    //! The POSIX arm — **ARMED to CI, never run on the box this crate was written on** (`s7c`
    //! §7c.10; ADR-0127 §Consequences). Every function is a safe `fn` around one `libc` call.

    use std::io;
    use std::os::unix::process::CommandExt;

    fn pid_t(pid: u32) -> io::Result<libc::pid_t> {
        libc::pid_t::try_from(pid)
            .map_err(|_| io::Error::other(format!("pid {pid} does not fit a pid_t")))
    }

    /// **spawn**: `process_group(0)` makes the child the leader of a new process group whose id is
    /// its own pid — the group exists the instant the child does, before it can fork. No `unsafe`.
    #[expect(
        clippy::disallowed_types,
        reason = "ADR-0127 decision 2/3: this is the ONE spawn site through which every child of every \
                  spawning adapter is created on POSIX — into its own process group, the group killpg \
                  later reaps. clippy.toml's disallowed-types has no path scoping, so the exemption is \
                  stated at the narrowest item that needs it."
    )]
    pub fn spawn_in_new_group(mut cmd: std::process::Command) -> io::Result<std::process::Child> {
        cmd.process_group(0).spawn()
    }

    /// **kill**: `killpg(pgid, SIGKILL)`. `SIGKILL`, not `SIGTERM`: AC-224's THEN is *"no descendant
    /// of the dispatch survives the call"*, and a catchable signal cannot promise that. (AC-225's
    /// *"exit 143 on POSIX"* is a shell's rendering of `SIGTERM`; the classifier is keyed on the
    /// token, so the signal number decides nothing — recorded in Slice 2's body as a reading.)
    /// An already-empty group (`ESRCH`) is success: the outcome wanted is the outcome found.
    pub fn killpg_all(pgid: u32) -> io::Result<()> {
        let pgid = pid_t(pgid)?;
        // SAFETY: plain-integer arguments to a documented syscall wrapper.
        let rc = unsafe { libc::killpg(pgid, libc::SIGKILL) };
        if rc == 0 {
            return Ok(());
        }
        let err = io::Error::last_os_error();
        if err.raw_os_error() == Some(libc::ESRCH) {
            return Ok(());
        }
        Err(err)
    }

    /// **`terminate_pid`** (test support): `kill(pid, SIGKILL)`; the code is Windows' and is
    /// accepted for signature symmetry.
    pub fn terminate_pid(pid: u32, _code: u32) -> io::Result<()> {
        let pid = pid_t(pid)?;
        // SAFETY: plain-integer arguments to a documented syscall wrapper.
        let rc = unsafe { libc::kill(pid, libc::SIGKILL) };
        if rc == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    /// **`is_alive`** (test support): `kill(pid, 0)` — `0` or `EPERM` is alive. A zombie reads alive
    /// until reaped, which is why the tests `wait()` the direct child before they read it.
    pub fn is_alive(pid: u32) -> bool {
        let Ok(pid) = pid_t(pid) else {
            return false;
        };
        // SAFETY: plain-integer arguments; signal 0 delivers nothing.
        let rc = unsafe { libc::kill(pid, 0) };
        rc == 0 || io::Error::last_os_error().raw_os_error() == Some(libc::EPERM)
    }
}
