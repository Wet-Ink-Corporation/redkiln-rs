//! The five named tests of `s7c` §7c.18's `-p rk-containment` paragraph, one per checkpoint line,
//! each red first (the failure text is quoted in Slice 2's commit body):
//!
//! - `ac224_job_object_created_before_spawn` (`cfg(windows)`) — the two parent-side readings,
//!   both taken before `ResumeThread` inside `spawn_into_job`'s hidden reading:
//!   `IsProcessInJob(child, OUR job)` TRUE and a previous suspend count of exactly 1 — **and the
//!   test's own readings, taken through `spawn_into_job_with`'s hook between the assign and the
//!   resume**: the job's enumeration lists the child, and after a second inside the job the
//!   `hold` fixture has written no marker (a suspended process writes nothing; a resumed `hold`
//!   writes within tens of milliseconds). The two crate-reported values are each true under a
//!   mutant that resumes first and assigns second (the test-discrimination review's M-A — the
//!   count is 1 whenever a never-run thread is first resumed, the membership is true whenever it
//!   is read after the assign), which is why the hook exists: under M-A the marker appears inside
//!   the hook, red; under no-assign the enumeration is empty, red; under an un-suspended spawn
//!   both fire and the count reads 0. The crate makes the same order a type (`ffi::win::Suspended`
//!   → `Assigned` → `Child`), so the mutant is also a compile error in the crate itself. **What
//!   this test's name promises and cannot see**: that the job was created before the spawn. With
//!   the child suspended a job created between the spawn and the assign is indistinguishable from
//!   outside (M-B passes every assertion here), so create-order is held by construction (step 1 of
//!   `spawn_into_job`) and the discriminated property is *assigned before the first instruction*.
//!   **No child-side line** (§7c.10: cargo wraps every test process in a `KILL_ON_JOB_CLOSE`
//!   job, so a child reading it discriminates nothing).
//! - `orphan_grandchild_reaped_via_job` (`cfg(windows)`) — the fixture in three-level `orphan` mode
//!   with the child holding; the holder's liveness read after `group.kill()` and BEFORE the
//!   handle drops. Red by a PID walk that misses the holder. Slice 2's measurement of O-1/O-2
//!   (positive control), and S-CANCEL-WIN's Job Object arm (the tree deletes after the kill).
//! - `breakaway_attempt_fails_without_breakaway_ok` (`cfg(windows)`) — the fixture in `breakaway`
//!   mode; red by a breakaway that succeeds (a pid outside our job). O-2's negative control.
//! - `ac278_no_descendant_survives_exit` — the primitive over a fixture child, never the gate path:
//!   the child exits leaving **two** orphans (`orphan-mid:2`); enumeration via the job / pgid lists
//!   both from outside; the kill reaps both. Red by a survivor after exit, read by `is_alive` on
//!   each marker's pid — two, so a kill that reaps only the first pid the job lists leaves one
//!   (AC-278's THEN is plural).
//! - `run_bounded_kills_tree_at_timeout` — red by a survivor after the bound, by output through a
//!   pipe (`spew:<bytes>` with N > 64 KiB), by `TimedOut` reported as an exit status, **or by a
//!   timeout arm that does not kill through the group and lets the drop's `KILL_ON_JOB_CLOSE`
//!   sweep do its work** — read through `run_bounded_with`'s hook as the group's kill-then-drop
//!   probe and as the holder's liveness on the first read after the call returns (a kill through
//!   the group has waited for every member; a drop's sweep is asynchronous and would not satisfy
//!   an immediate read).
//!
//! **Every liveness read is `rk_containment::is_alive` on a marker's pid** — the process table,
//! never the job — so a mutant with no job cannot hide a survivor (§7c.17). Every test spawns its
//! own fixture tree under its own temp dir (Halt 6; `run_bounded_…`'s two arms each have theirs)
//! and reports a tree that will not delete rather than reusing it. **Every pid a test learns is
//! registered on a [`Reaper`]** so a red's survivor is terminated during the unwind and a piped
//! harness reads EOF at once, not at the fixture's 600 s cap (`support`'s module doc); the
//! liveness assertions read first, the reaper runs after.

use std::time::Duration;

use rk_containment::{Contained, Outcome, is_alive, run_bounded, run_bounded_with, spawn_into_job};

use crate::support::{Reaper, fixture, read_markers, wait_dead, wait_for_marker};

/// 20 s of 10 ms polls: the bound on every "the fixture is up" wait.
const UP_POLLS: u32 = 2_000;
/// 5 s of 10 ms polls: the bound on every "it is dead now" wait (a kill through the job has
/// already waited for its members; on POSIX a reaped orphan can lag by a scheduler tick).
const DEAD_POLLS: u32 = 500;
/// 1 s of 10 ms polls: how long `ac224_…`'s hook watches for a marker that must NOT appear while
/// the child is suspended inside the job — a resumed `hold` child writes its marker in tens of
/// milliseconds, so a second of silence is not a near miss.
const SUSPENDED_POLLS: u32 = 100;

#[cfg(windows)]
#[test]
fn ac224_job_object_created_before_spawn() {
    let dir = tempfile::tempdir().expect("temp dir");
    let mut reaper = Reaper::new();
    // The test's OWN readings, between the assign and the resume (the module doc): the child is
    // in the job by the job's own enumeration, and it has executed nothing.
    let mut in_job_while_suspended: Option<(u32, Vec<u32>)> = None;
    let mut marker_while_suspended = None;
    let Contained { mut child, group } =
        rk_containment::spawn_into_job_with(fixture("hold", dir.path()), |group, child| {
            in_job_while_suspended = Some((
                child.id(),
                group.pids().expect("enumerate via the job, suspended"),
            ));
            marker_while_suspended = wait_for_marker(dir.path(), "hold", SUSPENDED_POLLS);
        })
        .expect("spawn_into_job");
    reaper.track(child.id());
    let obs = group.spawn_observation();
    println!(
        "MEASURED O-1: IsProcessInJob(child, our_job) before resume = {}; ResumeThread previous suspend count = {}; threads resumed = {}; in the job's own list while suspended = {:?}; marker while suspended = {:?}",
        obs.in_our_job_before_resume,
        obs.previous_suspend_count,
        obs.threads_resumed,
        in_job_while_suspended,
        marker_while_suspended.as_ref().map(|m| m.pid)
    );

    // (0) the hook's readings: assigned, and not yet run — the observation the crate cannot
    // report about itself
    let (pid, pids) = in_job_while_suspended.expect("the hook ran between assign and resume");
    assert_eq!(pid, child.id(), "the hook saw the child that came back");
    assert!(
        pids.contains(&pid),
        "between the assign and the resume the job's own list {pids:?} does not hold the child \
         {pid}: the child was not in THIS job before it could run (ADR-0127 d1: assigned before \
         it can fork)"
    );
    assert!(
        marker_while_suspended.is_none(),
        "the `hold` child (pid {}) wrote its marker while it was supposed to be suspended inside \
         the job: it was resumed before the assign (ADR-0127 d1: assigned before it can fork — \
         the resume-then-assign mutant)",
        marker_while_suspended.as_ref().map_or(0, |m| m.pid)
    );
    // (1) the child was in OUR job before it was resumed — cargo's outer job cannot satisfy this
    assert!(
        obs.in_our_job_before_resume,
        "IsProcessInJob(child, our_job) read FALSE before the resume: the child was not assigned \
         to THIS job before it could run (ADR-0099 d3 / ADR-0127 d1: assigned before it can fork)"
    );
    // (2) the primary thread had never run: ResumeThread's previous suspend count is exactly 1
    assert_eq!(
        obs.previous_suspend_count, 1,
        "ResumeThread's previous suspend count was {} — exactly 1 is a primary thread that has \
         never run; 0 is a child spawned un-suspended and assigned afterwards (RT-13 as amended)",
        obs.previous_suspend_count
    );
    assert_eq!(
        obs.threads_resumed, 1,
        "a freshly created suspended process has exactly one thread; {} were resumed",
        obs.threads_resumed
    );

    // and the resume took: the child ran after it and wrote its marker
    let marker = wait_for_marker(dir.path(), "hold", UP_POLLS).expect(
        "the resumed child never wrote its marker: the re-opened primary thread did not run",
    );
    assert_eq!(marker.pid, child.id(), "the marker's pid is the child's");
    assert_eq!(
        marker.args.len(),
        1,
        "the fixture received argv[0] and nothing else: {:?}",
        marker.args
    );
    let pids = group.pids().expect("enumerate via the job");
    assert!(
        pids.contains(&child.id()),
        "the job's own process list {pids:?} does not hold the child {}",
        child.id()
    );

    group.kill().expect("kill through the job");
    let status = child.wait().expect("wait");
    assert_eq!(
        status.code(),
        Some(i32::try_from(rk_containment::CANCEL_EXIT_CODE).expect("fits")),
        "the child's exit status is the adapter's own CANCEL_EXIT_CODE (AC-225 / PIA-059)"
    );
    assert!(
        !is_alive(child.id()),
        "the direct child (pid {}) is still alive after the kill through the job",
        child.id()
    );
    dir.close()
        .expect("the fixture tree deletes after the kill");
}

#[cfg(windows)]
#[test]
fn orphan_grandchild_reaped_via_job() {
    let dir = tempfile::tempdir().expect("temp dir");
    let mut reaper = Reaper::new();
    let Contained { mut child, group } =
        spawn_into_job(fixture("orphan", dir.path())).expect("spawn_into_job");
    reaper.track(child.id());

    // The fixture announces through its own marker once the holder is up and the intermediate has
    // exited — the three-level shape: child (alive, holding) → intermediate (dead) → holder (alive).
    let orphan = wait_for_marker(dir.path(), "orphan", UP_POLLS)
        .expect("the orphan fixture never reached READY (its holder never came up)");
    let holder_pid: u32 = orphan
        .extra_value("holder_pid")
        .and_then(|v| v.parse().ok())
        .expect("the orphan marker carries holder_pid=");
    reaper.track(holder_pid);
    let mid = read_markers(dir.path())
        .into_iter()
        .find(|m| m.mode == "orphan-mid")
        .expect("the intermediate wrote its marker");
    reaper.track(mid.pid);
    assert!(
        wait_dead(mid.pid, DEAD_POLLS),
        "precondition: the intermediate (pid {}) must have exited so the holder is an orphan",
        mid.pid
    );
    assert!(
        is_alive(child.id()),
        "precondition: the direct child must be ALIVE when the kill fires (the WHEN of AC-224)"
    );
    assert!(
        is_alive(holder_pid),
        "precondition: the orphaned holder (pid {holder_pid}) must be alive before the kill"
    );

    // Enumeration VIA THE JOB lists the holder although its parent is dead — the reading a PID walk
    // from the live child cannot make (F22's shape).
    let pids = group.pids().expect("enumerate via the job");
    assert!(
        pids.contains(&holder_pid),
        "enumeration via the job {pids:?} does not list the orphaned holder {holder_pid}"
    );
    assert!(
        pids.contains(&child.id()),
        "enumeration via the job {pids:?} does not list the direct child {}",
        child.id()
    );
    println!(
        "MEASURED O-2 positive control: child {} alive, intermediate {} dead, orphaned holder {holder_pid} alive and listed by the job {pids:?}",
        child.id(),
        mid.pid
    );

    // The kill, through the group — and the holder's liveness read AFTER it and BEFORE the drop,
    // so KILL_ON_JOB_CLOSE at drop cannot do the kill's work.
    group.kill().expect("kill through the job");
    assert!(
        wait_dead(holder_pid, DEAD_POLLS),
        "the orphaned holder (pid {holder_pid}) SURVIVED group.kill(): its parent is dead, so a \
         PID walk from the live child cannot reach it — only a kill through the job can (AC-224)"
    );
    assert!(
        wait_dead(child.id(), DEAD_POLLS),
        "the direct child survived group.kill()"
    );
    println!(
        "MEASURED O-2 positive control: after group.kill() and before the drop, holder {holder_pid} is_alive = {}, child {} is_alive = {}",
        is_alive(holder_pid),
        child.id(),
        is_alive(child.id())
    );
    let status = child.wait().expect("wait");
    assert_eq!(
        status.code(),
        Some(i32::try_from(rk_containment::CANCEL_EXIT_CODE).expect("fits")),
        "the child died with the adapter's own code, through the job"
    );
    let after = group.pids().expect("enumerate after the kill");
    assert!(
        after.is_empty(),
        "the job still lists {after:?} after the kill"
    );

    // kill-then-drop order, recorded by the group's hidden hook (§7c.10) — "killed" meaning a kill
    // that COMPLETED, since the probe records only on Ok
    let order = group.kill_drop_order();
    drop(group);
    assert!(
        order.killed_before_drop(),
        "the group was dropped without a completed kill() first — a caller relying on \
         KILL_ON_JOB_CLOSE at drop instead of killing through the job"
    );

    // S-CANCEL-WIN's Job Object arm (Halt 6): the fixture tree deletes after the kill.
    dir.close().expect(
        "S-CANCEL-WIN: the fixture tree could not be removed after the kill — a handle survived",
    );
}

#[cfg(windows)]
#[test]
fn breakaway_attempt_fails_without_breakaway_ok() {
    let dir = tempfile::tempdir().expect("temp dir");
    let mut reaper = Reaper::new();
    let Contained { mut child, group } =
        spawn_into_job(fixture("breakaway", dir.path())).expect("spawn_into_job");
    reaper.track(child.id());
    let marker = wait_for_marker(dir.path(), "breakaway", UP_POLLS)
        .expect("the breakaway fixture never recorded its attempt");

    // The attempt's surface — O-2: a spawn error, or a child that landed inside our job. Either is
    // the attempt FAILING; a child outside our job is the containment hole, and is red. The
    // reading is taken first and the pid registered for the reaper before the assertion, so a red
    // here (a breakaway that succeeded) leaves no survivor holding the harness's pipe.
    let mut breakaway_pid: Option<u32> = None;
    if let Some(v) = marker.extra_value("breakaway") {
        if let Some(pid) = v.strip_prefix("ok pid=") {
            let pid: u32 = pid.parse().expect("pid");
            breakaway_pid = Some(pid);
            reaper.track(pid);
            let pids = group.pids().expect("enumerate via the job");
            assert!(
                pids.contains(&pid),
                "the CREATE_BREAKAWAY_FROM_JOB child (pid {pid}) spawned and is NOT in our job \
                 {pids:?}: the breakaway SUCCEEDED — the job granted BREAKAWAY_OK, which ADR-0127 \
                 decision 7 forbids"
            );
            println!(
                "MEASURED breakaway surface: spawn succeeded, pid {pid} landed INSIDE our job"
            );
        } else if let Some(os) = v.strip_prefix("err os=") {
            let os: i32 = os.parse().expect("os code");
            assert_eq!(
                os, 5,
                "the breakaway spawn failed with OS error {os}, not ERROR_ACCESS_DENIED (5) — a \
                 failure for some other reason is not the negative control"
            );
            println!(
                "MEASURED breakaway surface: spawn error, ERROR_ACCESS_DENIED (os error {os})"
            );
        } else {
            panic!("unrecognised breakaway line {v:?}");
        }
    } else {
        panic!("the breakaway marker carries no breakaway= line: {marker:?}");
    }

    group.kill().expect("kill through the job");
    assert!(
        wait_dead(child.id(), DEAD_POLLS),
        "the breakaway fixture's direct child (pid {}) survived group.kill(): it was never in our \
         job (an `err os=5` reading above can come from the OUTER job cargo wraps the test in, \
         not ours — §7c.10)",
        child.id()
    );
    child.wait().expect("wait");
    if let Some(pid) = breakaway_pid {
        assert!(
            wait_dead(pid, DEAD_POLLS),
            "the breakaway child (pid {pid}) survived the kill through the job"
        );
    }
    dir.close()
        .expect("the fixture tree deletes after the kill");
}

#[test]
fn ac278_no_descendant_survives_exit() {
    let dir = tempfile::tempdir().expect("temp dir");
    let mut reaper = Reaper::new();
    // `orphan-mid:2`: spawn TWO holders and exit at once — the shape of a run that leaves daemons
    // behind, plural, as AC-278's THEN is.
    let Contained { mut child, group } =
        spawn_into_job(fixture("orphan-mid:2", dir.path())).expect("spawn_into_job");
    reaper.track(child.id());
    let status = child.wait().expect("wait");
    assert!(status.success(), "the intermediate exited {status}");
    let mid = read_markers(dir.path())
        .into_iter()
        .find(|m| m.mode == "orphan-mid")
        .expect("the exited child wrote its marker");
    assert_eq!(mid.pid, child.id(), "the marker is the direct child's");
    assert_eq!(
        mid.args.len(),
        1,
        "the fixture received argv[0] and nothing else: {:?}",
        mid.args
    );

    // Both holders' markers, then both alive after their parent's exit.
    let mut holders: Vec<u32> = Vec::new();
    for _ in 0..UP_POLLS {
        holders = read_markers(dir.path())
            .into_iter()
            .filter(|m| m.mode == "orphan-holder")
            .map(|m| m.pid)
            .collect();
        if holders.len() >= 2 {
            break;
        }
        std::thread::park_timeout(crate::support::POLL);
    }
    holders.sort_unstable();
    for &pid in &holders {
        reaper.track(pid);
    }
    assert_eq!(
        holders.len(),
        2,
        "orphan-mid:2 left {} holder markers, not 2: {holders:?}",
        holders.len()
    );
    for &pid in &holders {
        assert!(
            is_alive(pid),
            "precondition: the holder (pid {pid}) must outlive its exited parent, or nothing is \
             measured"
        );
    }

    // AC-278's instrument: enumerate the descendants of the run's job object / process group
    // AFTER the child's exit, from outside — every survivor, not one.
    let pids = group.pids().expect("enumerate after exit");
    for &pid in &holders {
        assert!(
            pids.contains(&pid),
            "after the child exited, enumeration via the job/pgid {pids:?} does not list the \
             surviving descendant {pid}"
        );
    }
    println!(
        "MEASURED AC-278 primitive: child {} exited, holders {holders:?} alive and listed by the job/pgid {pids:?}",
        child.id()
    );

    group.kill().expect("kill through the group");
    let survivors: Vec<u32> = holders
        .iter()
        .copied()
        .filter(|&pid| !wait_dead(pid, DEAD_POLLS))
        .collect();
    assert!(
        survivors.is_empty(),
        "descendants of the run {survivors:?} (of {holders:?}) survived the kill after the \
         child's exit — AC-278: no child, background, detached or daemon process belonging to the \
         run remains alive; a kill that reaches only the first pid the group lists leaves one"
    );
    let after = group.pids().expect("enumerate after the kill");
    assert!(
        after.is_empty(),
        "the group still lists {after:?} after the kill"
    );
    dir.close()
        .expect("the fixture tree deletes after the kill");
}

#[test]
fn run_bounded_kills_tree_at_timeout() {
    // Arm A — output to a FILE survives a child that writes more than a pipe holds. Its own tree.
    let dir_a = tempfile::tempdir().expect("temp dir");
    let out_path = dir_a.path().join("spew.out");
    let out = std::fs::File::create(&out_path).expect("stdout file");
    let mut cmd = fixture("spew:100000", dir_a.path());
    cmd.stdout(std::process::Stdio::from(out));
    let outcome = run_bounded(cmd, Duration::from_secs(20)).expect("run_bounded");
    match outcome {
        Outcome::Exited(status) => assert!(status.success(), "spew exited {status}"),
        other => panic!(
            "spew:100000 (> 64 KiB) ended as {other:?}, not Exited: output went through a pipe \
             that filled and deadlocked (DECISIONS-2026-09-09 #31) — run_bounded must leave the \
             caller's file stdio in place"
        ),
    }
    let len = std::fs::metadata(&out_path).expect("stdout file").len();
    assert_eq!(len, 100_000, "the file holds every byte the child wrote");
    dir_a
        .close()
        .expect("arm A's fixture tree deletes after the run");

    // Arm B — the bound fires on a holding child whose grandchild is orphaned; the whole tree
    // dies THROUGH THE GROUP before run_bounded returns, and the outcome is TimedOut, never an
    // exit status. Its own tree (Halt 6: parallel arms share nothing).
    let dir_b = tempfile::tempdir().expect("temp dir");
    let mut reaper = Reaper::new();
    let mut cmd = fixture("orphan", dir_b.path());
    cmd.stdout(std::process::Stdio::null());
    let mut order = None;
    let outcome = run_bounded_with(cmd, Duration::from_secs(6), |group| {
        order = Some(group.kill_drop_order());
    })
    .expect("run_bounded");
    let order = order.expect("the hook ran after the spawn");
    let orphan = wait_for_marker(dir_b.path(), "orphan", 1).expect(
        "precondition: the orphan fixture did not reach READY before the 6 s bound fired, so the \
         kill had no orphan to reap — a longer bound is needed, not a pass",
    );
    reaper.track(orphan.pid);
    let holder_pid: u32 = orphan
        .extra_value("holder_pid")
        .and_then(|v| v.parse().ok())
        .expect("holder_pid=");
    reaper.track(holder_pid);
    // The readings, all taken before any assertion: the outcome, the probe, and liveness on the
    // FIRST read — a kill through the group has waited for every member before returning, where a
    // drop's KILL_ON_JOB_CLOSE sweep is asynchronous and could not satisfy an immediate read.
    let holder_alive_now = is_alive(holder_pid);
    let child_alive_now = is_alive(orphan.pid);
    println!(
        "MEASURED run_bounded timeout arm: outcome {outcome:?}; killed-before-drop = {}; on the first read after return holder {holder_pid} is_alive = {holder_alive_now}, child {} is_alive = {child_alive_now}",
        order.killed_before_drop(),
        orphan.pid
    );
    assert!(
        matches!(outcome, Outcome::TimedOut),
        "the bound fired but run_bounded returned {outcome:?} — a timed-out run reported as an \
         exit status is what ADR-0127's forbids names"
    );
    assert!(
        order.killed_before_drop(),
        "run_bounded's timeout arm returned TimedOut without a completed kill through the group \
         before the group dropped — the tree was left to KILL_ON_JOB_CLOSE at drop, the mutant \
         §7c.10's kill-then-drop probe exists to make red"
    );
    assert!(
        !holder_alive_now,
        "the orphaned holder (pid {holder_pid}) was alive on the first read after run_bounded \
         returned TimedOut — the timeout arm did not kill the tree through the group and wait for \
         its members (a drop's sweep would be asynchronous)"
    );
    assert!(
        !child_alive_now,
        "the direct child (pid {}) was alive on the first read after run_bounded returned",
        orphan.pid
    );
    dir_b
        .close()
        .expect("arm B's fixture tree deletes after the kill");
}
