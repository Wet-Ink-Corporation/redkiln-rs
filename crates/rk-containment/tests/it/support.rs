//! Test support: the one `Command` builder for the fixture child, the marker reader, and the
//! bounded waits — counted `park_timeout` loops, never `sleep` (clippy-banned) and never a clock
//! (`Instant::now` is clippy-banned too; a counted loop of bounded parks needs neither).
//!
//! Every test reaches the fixture through `env!("CARGO_BIN_EXE_rk-fixture-child")` — the
//! compile-time path cargo sets for this package's own `[[bin]]` (`s7c` §7c.17: never
//! `std::env::var`, never `current_dir`).
//!
//! **A red must not hang the harness** ([`Reaper`]). The fixture's descendants are spawned with
//! `std`'s default handle inheritance, so a survivor of a failed containment holds a copy of the
//! test process's stdout — and when that stdout is a pipe (`cargo test | …`, or Checkpoint 7c's
//! `$(cargo test -p rk-containment -- <name>)` capture in `rk_test`), the reader waits for EOF
//! until the survivor's 600 s hold cap expires: the test panicked in a quarter of a second and
//! nobody sees it for ten minutes, which reads as a Halt 5 hang rather than a red [measured
//! 2026-09-18 by the test-discrimination review: a BREAKAWAY_OK mutant's red at 0.26 s, the
//! pipeline returning only when the survivor was stopped]. So every pid a test learns is
//! registered on a [`Reaper`], whose `Drop` — which runs during the unwind of a failed assertion
//! as much as at a green exit — terminates every registered process still alive with
//! `terminate_pid`. The assertions read liveness FIRST and the reaper runs AFTER, so cleanup
//! never turns a survivor into a pass; it only makes the red prompt.

use std::fs;
use std::path::Path;
use std::time::Duration;

/// One poll of a bounded wait.
pub const POLL: Duration = Duration::from_millis(10);

/// The one `std::process::Command` site in the tests: the fixture child in `mode`, its marker dir
/// set, stdout and stderr to null so no pipe is ever held by a descendant (a holder inherits its
/// parent's handles, and a pipe it held open would outlive the test's reader).
#[expect(
    clippy::disallowed_types,
    reason = "s7c §7c.17: the tests build the fixture child's Command here, once, and hand it to the \
              crate's spawn primitives — the type is banned workspace-wide by clippy.toml and this \
              is the tests' single site."
)]
pub fn fixture(mode: &str, marker_dir: &Path) -> std::process::Command {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_rk-fixture-child"));
    cmd.env("RK_FIXTURE_MODE", mode)
        .env("RK_FIXTURE_MARKER_DIR", marker_dir)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    cmd
}

/// One marker file, parsed.
#[derive(Debug, Clone)]
pub struct Marker {
    pub pid: u32,
    pub mode: String,
    pub args: Vec<String>,
    pub extra: Vec<String>,
}

impl Marker {
    /// The value of an `extra` line `key=<value>`, if present.
    pub fn extra_value(&self, key: &str) -> Option<&str> {
        let prefix = format!("{key}=");
        self.extra
            .iter()
            .find_map(|l| l.strip_prefix(prefix.as_str()))
    }
}

/// Every `*.marker` in `dir`, parsed; a file that is not yet complete is skipped (the bin writes
/// to `.tmp` and hard-links it to a fresh name, so a `.marker` is always whole and never replaced).
pub fn read_markers(dir: &Path) -> Vec<Marker> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "marker") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let mut pid = None;
        let mut mode = None;
        let mut args = Vec::new();
        let mut extra = Vec::new();
        for line in text.lines() {
            if let Some(v) = line.strip_prefix("pid=") {
                pid = v.parse().ok();
            } else if let Some(v) = line.strip_prefix("mode=") {
                mode = Some(v.to_owned());
            } else if let Some(v) = line.strip_prefix("arg=") {
                args.push(v.to_owned());
            } else {
                extra.push(line.to_owned());
            }
        }
        if let (Some(pid), Some(mode)) = (pid, mode) {
            out.push(Marker {
                pid,
                mode,
                args,
                extra,
            });
        }
    }
    out
}

/// Wait, in at most `polls` parks of [`POLL`], for a marker with `mode=<mode>` to appear.
pub fn wait_for_marker(dir: &Path, mode: &str, polls: u32) -> Option<Marker> {
    for _ in 0..polls {
        if let Some(m) = read_markers(dir).into_iter().find(|m| m.mode == mode) {
            return Some(m);
        }
        std::thread::park_timeout(POLL);
    }
    None
}

/// Wait, in at most `polls` parks of [`POLL`], for `is_alive(pid)` to read false. Returns at once
/// when it does; a survivor holds for 600 s, so a false here is never a near miss.
pub fn wait_dead(pid: u32, polls: u32) -> bool {
    for _ in 0..polls {
        if !rk_containment::is_alive(pid) {
            return true;
        }
        std::thread::park_timeout(POLL);
    }
    false
}

/// Every pid a test has learned, terminated on drop if still alive — so a red's survivor dies
/// during the unwind and a piped harness reads EOF at once instead of at the fixture's 600 s cap
/// (the module doc). Registration is a reading the test has already taken; the reaper never
/// enumerates anything itself, and a pid it kills was a survivor the assertion has already
/// judged.
#[derive(Debug, Default)]
pub struct Reaper {
    pids: Vec<u32>,
}

impl Reaper {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a pid to reap on drop.
    pub fn track(&mut self, pid: u32) {
        if !self.pids.contains(&pid) {
            self.pids.push(pid);
        }
    }
}

impl Drop for Reaper {
    fn drop(&mut self) {
        for &pid in &self.pids {
            if rk_containment::is_alive(pid) {
                // A pid that cannot be terminated is one that died between the read and the call;
                // there is nothing further to do with the error during an unwind.
                let _ = rk_containment::terminate_pid(pid, rk_containment::CANCEL_EXIT_CODE);
            }
        }
    }
}
