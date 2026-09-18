//! The fixture suite (s7c §7c.14, §7c.17): TWO passing tests -- one of them the AC-329 decoy, which
//! prints the prose summary line and a phantom per-test line to its own stdout (libtest captures a
//! passing test's stdout, so this decoy reaches the adapter only under `--nocapture`, which nothing
//! runs; the decoys that DO reach it are tests/zz_decoy.rs's on stdout and build.rs's on stderr)
//! -- plus a failing test and a slow test, both
//! `#[ignore]` and reachable ONLY by selection (`-- --ignored <name>`), so the conformance core's
//! unselected `cargo test --workspace` is green at count 2 (RT-22).

use std::sync::{Condvar, Mutex};
use std::time::Duration;

#[test]
fn passes_and_prints_the_decoy() {
    println!("test result: ok. 99 passed; 0 failed");
    println!("test phantom ... ok");
    assert_eq!(rk_toolchain_rust_fixture::add(1, 1), 2);
}

/// Also the SPAWN MARKER (s7c §7c.18: *"`Ok` without a spawn marker — the fixture project's tests
/// write one"*): when the adapter's constructed child map carries `RK_SPAWN_MARKER_DIR`, this test
/// writes one file there per run, so a conformance row can tell a real `cargo test` from an
/// adapter that answered `Ok(2)` without spawning. Read at the fixture's own edge (outside every
/// `src/` ban).
#[test]
fn passes_and_is_filterable() {
    if let Some(dir) = std::env::var_os("RK_SPAWN_MARKER_DIR") {
        let path = std::path::Path::new(&dir).join(format!("rust-{}.marker", std::process::id()));
        std::fs::write(path, b"spawned\n").expect("the marker dir is writable");
    }
    assert_eq!(rk_toolchain_rust_fixture::add(2, 2), 4);
}

/// Reachable only by `-- --ignored red_by_selection`: the RT-22 red suite.
#[test]
#[ignore = "reachable only by selection: the red suite (RT-22)"]
fn red_by_selection() {
    assert_eq!(
        rk_toolchain_rust_fixture::add(2, 2),
        5,
        "this test is the fixture's red"
    );
}

/// Reachable only by `-- --ignored slow_by_selection`: AC-331's victim, which outlives any bound the
/// adapter tests construct. Holds on a Condvar nobody signals (no `sleep`, the house rule).
#[test]
#[ignore = "reachable only by selection: AC-331's slow victim"]
fn slow_by_selection() {
    let pair = (Mutex::new(false), Condvar::new());
    let mut done = pair.0.lock().expect("unpoisoned");
    let mut remaining = Duration::from_secs(600);
    while !*done && !remaining.is_zero() {
        let (guard, timed_out) = pair.1.wait_timeout(done, remaining).expect("unpoisoned");
        done = guard;
        if timed_out.timed_out() {
            remaining = Duration::ZERO;
        }
    }
}
