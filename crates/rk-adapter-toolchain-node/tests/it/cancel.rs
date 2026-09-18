//! **S-CANCEL-WIN's vitest arm** (design §10.4/§10.5; DECISIONS-2026-09-17 D17-6, O-10; s7c §7c.16):
//! *"a mid-`vitest` victim under `rk-containment`'s job, killed by `run_bounded`'s timeout path
//! (the Node toolchain's), with Halt 6's own fixture tree"* — and the handle-leak question
//! ADR-0127 §7 leaves to this spike: *"whether a killed `vitest` under a job still leaves an
//! unremovable fixture tree"* (DECISIONS-2026-09-09 #30: a kill mid-`confirm` left a Windows
//! handle on the fixture tree and `build_silent_fixture` could not clear it — `os error 32`;
//! design §9.1's counter-evidence: *"a tree kill reaps processes; it does not release an OS
//! handle the dead process left behind"*).
//!
//! **The victim's tree is its own** (Halt 6): the fixture's suite is copied into a temporary
//! directory INSIDE the fixture project (so `npx` resolves the project's `node_modules/.bin/vitest`
//! by walking up and fetches nothing, and vitest's root is the temp tree), the adapter's project
//! root is that tree, the bound is 6 s, and the selection is the skip-marked slow test. **That the
//! victim was alive mid-run when the bound killed it is proved by the slow test's own two side
//! effects** (the fixture's `suite.test.ts`): with `RK_SPAWN_MARKER_DIR` in the child map it writes
//! `slow-started-<pid>.marker` before its 600 s wait and `slow-finished-<pid>.marker` after — so
//! the arm asserts the first exists and the second does not, and a `Timeout` from an adapter that
//! never spawned, or a run that ended before the kill, is red on the markers, not on timing alone.
//! After the adapter reports `Timeout`, `TempDir::close()` is called — the removal that #30 could
//! not perform — and its result IS the spike's reading, printed as `S_CANCEL_WIN_VITEST_ARM=<…>`
//! for the slice body and design §10.5's row: `ok-immediately`, `ok-after-<n>ms` (the first
//! `close()` failed and the root became removable after `n` ms of 25-ms polls), or `err` (held
//! past 10 s — #30's leak, and the test fails).
//!
//! **MEASURED 2026-09-18, this box.** *With the fixture's vitest pool uncapped* (a worker per
//! core pre-spawned — ~20 `node.exe` under each `vitest run`): `close()` the instant `run_bounded`
//! returned — every job member already waited dead — failed with `os error 32` (*"being used by
//! another process"*) **on the tree's root directory only**, its contents already removed; the
//! root became removable after **38 / 37 / 37 ms** alone (32–41 ms across a reviewer's and later
//! runs), and once, inside the parallel suite, the parent's listing still carried the entry for
//! one read after `remove_dir` had succeeded (delete-pending: `exists()` false, the name not yet
//! gone). *With the pool capped to one fork* (`vitest.config.ts`, the same commit): `close()`
//! succeeds **immediately** and the listing is clear within **1–5 ms**, in thirteen runs alone,
//! inside the parallel suite, and under two deliberate process-enumeration / process-spawn storms
//! — so the tens-of-milliseconds hold WAS the dead workers' current-directory handles being torn
//! down, and the killed vitest leaves no handle that survives its processes: not #30's leak
//! (which outlived `Stop-Process` on every `node`). **One observation that did not reproduce**,
//! recorded rather than smoothed: four consecutive runs in one two-minute window (a runaway
//! shell loop was spawning a failing `tasklist.exe` continuously on the box at the time) read
//! `ok-immediately` and then found the EMPTY root still listed after the 10 s bound; each was
//! removed by hand with a plain `rmdir` minutes later — an entry the OS cleared, never an
//! unremovable handle. Hence the shape below: the removal is the fatal reading (`close()` Ok, or
//! the root gone within 10 s of polls); the listing lag is polled to the same bound and PRINTED
//! (`S_CANCEL_WIN_VITEST_ARM_LISTING=…`), and the stable predicate after it — the root does not
//! exist — is what is asserted, so a delete-pending name the OS is still clearing cannot turn the
//! workspace gate red under load. A caller that removes a victim's tree right after the kill
//! should expect both lags; nothing in the adapter depends on them. The Job Object arm ran at
//! Slice 2 (`orphan_grandchild_reaped_via_job`, `breakaway_attempt_fails_without_breakaway_ok`).

use std::path::Path;
use std::time::{Duration, Instant};

use rk_ports::toolchain::Toolchain;
use rk_types::error::StepError;

use crate::support::{SUITE_FILE, adapter_with_env, fixture_project, preflight};

/// How long the arm waits for a lagging OS teardown before calling it #30's leak.
const HOLD_BOUND: Duration = Duration::from_secs(10);
/// The poll interval — a channel wait, never `sleep` (the house rule).
const POLL: Duration = Duration::from_millis(25);

/// Poll `ready` every [`POLL`] until it answers `Ok(())` or [`HOLD_BOUND`] passes since `started`;
/// the answer is the last `ready` result and the elapsed time. A channel nobody sends on is the
/// wait: `recv_timeout` returns after the interval without a `sleep`.
fn poll_until(
    started: Instant,
    mut ready: impl FnMut() -> Result<(), std::io::Error>,
) -> (Result<(), std::io::Error>, Duration) {
    let (_tx, rx) = std::sync::mpsc::channel::<()>();
    let mut result = ready();
    while result.is_err() && started.elapsed() <= HOLD_BOUND {
        let _ = rx.recv_timeout(POLL);
        result = ready();
    }
    (result, started.elapsed())
}

/// The `.s-cancel-win-*` entries the fixture project's listing carries right now.
fn leftover_entries(project: &Path) -> Vec<String> {
    std::fs::read_dir(project)
        .expect("readable")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".s-cancel-win-"))
        .collect()
}

/// The `<prefix>*.marker` files in the marker directory.
fn markers(dir: &Path, prefix: &str) -> Vec<String> {
    std::fs::read_dir(dir)
        .expect("the marker dir is readable")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(prefix) && n.ends_with(".marker"))
        .collect()
}

/// S-CANCEL-WIN, the vitest arm: a mid-vitest victim under the job is killed at the bound, the
/// outcome is `Timeout` (never a status), the victim had started its wait and never finished it,
/// and the victim's own tree is removable afterwards.
#[test]
fn s_cancel_win_vitest_arm() {
    preflight();
    let victim_tree = tempfile::Builder::new()
        .prefix(".s-cancel-win-")
        .tempdir_in(fixture_project())
        .expect("a temp tree inside the fixture project (git-ignored there)");
    std::fs::create_dir_all(victim_tree.path().join("test")).expect("test dir");
    std::fs::copy(
        fixture_project().join(SUITE_FILE),
        victim_tree.path().join(SUITE_FILE),
    )
    .expect("the suite is copied into the victim's own tree");
    std::fs::copy(
        fixture_project().join("vitest.config.ts"),
        victim_tree.path().join("vitest.config.ts"),
    )
    .expect("the config is copied into the victim's own tree");
    let scratch = tempfile::tempdir().expect("scratch");
    let marker_dir = tempfile::tempdir().expect("marker dir");

    let tc = adapter_with_env(
        rk_adapter_toolchain_node::DEFAULT_PROGRAM.into(),
        Duration::from_secs(6),
        victim_tree.path(),
        &[(
            "RK_SPAWN_MARKER_DIR",
            marker_dir.path().to_string_lossy().into_owned(),
        )],
        scratch.path(),
    );
    let outcome = tc.test(&[
        SUITE_FILE.to_owned(),
        "-t".to_owned(),
        "slow_by_selection".to_owned(),
    ]);
    match outcome {
        Err(StepError::Timeout { after_secs }) => assert_eq!(after_secs, 6),
        other => {
            panic!("the mid-vitest victim was killed at the bound: expected Timeout, got {other:?}")
        }
    }

    // The victim was alive MID-RUN when the bound killed it: the slow test had written its
    // started marker and never reached its finished marker (a Timeout without a spawn, or a run
    // that ended before the kill, is red here).
    let started_markers = markers(marker_dir.path(), "slow-started-");
    let finished_markers = markers(marker_dir.path(), "slow-finished-");
    assert_eq!(
        started_markers.len(),
        1,
        "the victim's slow test started its wait before the kill (a `slow-started-<pid>.marker` in \
         the marker dir); found {started_markers:?}"
    );
    assert!(
        finished_markers.is_empty(),
        "the kill interrupted the victim's 600 s wait: no `slow-finished-*.marker` may exist; found \
         {finished_markers:?}"
    );
    let victim_pid = started_markers[0]
        .trim_start_matches("slow-started-")
        .trim_end_matches(".marker")
        .to_owned();

    // Halt 6's question, answered by the removal itself: does the killed vitest's tree close?
    // FIRST reading: `TempDir::close()` the instant `run_bounded` returns — every job member is
    // already waited dead (`Group::kill`). SECOND reading, when the first fails: how long until
    // the root directory is removable, polled every 25 ms through a channel wait (no `sleep`)
    // for at most 10 s — a transient hold is the OS's handle teardown lagging the process
    // signal; a hold that outlives the bound is the leak #30 saw.
    let root = victim_tree.path().to_path_buf();
    #[expect(
        clippy::disallowed_methods,
        reason = "S-CANCEL-WIN's own measurement, the test's ONE clock site: how long the killed \
                  victim's tree root (and its entry in the parent's listing) stays held after the \
                  kill (design §10.5's row); a test site, feeding no workflow decision."
    )]
    let started = Instant::now();
    let first = victim_tree.close();
    let reading = match first {
        Ok(()) => "ok-immediately".to_owned(),
        Err(first_err) => {
            let (result, elapsed) = poll_until(started, || {
                std::fs::remove_dir(&root).or_else(|e| if root.exists() { Err(e) } else { Ok(()) })
            });
            match result {
                Ok(()) => format!(
                    "ok-after-{}ms (close() failed first: os error {} on the root)",
                    elapsed.as_millis(),
                    first_err
                        .raw_os_error()
                        .map_or_else(|| "?".to_owned(), |c| c.to_string())
                ),
                Err(e) => format!("err: the root is still held after 10 s: {e}"),
            }
        }
    };
    println!("S_CANCEL_WIN_VITEST_ARM={reading} victim_pid={victim_pid}");
    assert!(
        !reading.starts_with("err"),
        "S-CANCEL-WIN vitest arm: the victim's fixture tree could not be removed after the kill \
         through the job — the handle leak DECISIONS-2026-09-09 #30 saw (os error 32) is still \
         live: {reading}"
    );

    // The parent's listing, polled to the same bound and PRINTED (the module doc: a
    // delete-pending name can outlive `remove_dir`'s success, and once outlived the bound); the
    // stable predicate — the root does not exist — is what is asserted.
    let (listed, elapsed) = poll_until(started, || {
        let left = leftover_entries(&fixture_project());
        if left.is_empty() {
            Ok(())
        } else {
            Err(std::io::Error::other(format!("still listed: {left:?}")))
        }
    });
    match listed {
        Ok(()) => println!(
            "S_CANCEL_WIN_VITEST_ARM_LISTING=clear-after-{}ms",
            elapsed.as_millis()
        ),
        Err(e) => println!("S_CANCEL_WIN_VITEST_ARM_LISTING=still-listed-after-10s ({e})"),
    }
    assert!(
        !root.exists(),
        "the victim's tree root still exists after the kill and the removal (S-CANCEL-WIN): {}",
        root.display()
    );
}
