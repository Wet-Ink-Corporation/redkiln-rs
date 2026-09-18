//! `rk-fixture-child` — the spawnable fixture child of `s7c` §7c.17: the process every containment
//! and dispatch test puts under the mechanism it tests. **`std` only** — it depends on no crate,
//! names no `windows_sys::` symbol and no `unsafe`, because `rk-adapter-claude` compiles this same
//! source as a second `[[bin]]` (`rk-fixture-child-claude`) against that crate's dependencies,
//! which carry no `windows-sys` row, and under the workspace's `unsafe_code = "deny"`.
//!
//! **How it learns its mode**: `RK_FIXTURE_MODE=<mode>[:<arg>]` and `RK_FIXTURE_MARKER_DIR=<dir>`
//! in the environment its parent constructed — the argv is the golden vector and carries no mode
//! flag (§7c.12) — read at this bin's own edge with `std::env::var`, which is outside every
//! `std::env::var` ban's scope (the bans read adapter `src/`; this file is under `tests/`).
//! `set_var` is never called: a child's environment is built on its `Command`.
//!
//! **One marker file per invocation** into `RK_FIXTURE_MARKER_DIR` (§7c.17), named `<pid>.marker`,
//! written to a `.tmp` and renamed so a reader never sees a partial file, carrying `pid=<own pid>`,
//! `mode=<mode name, without its `:<arg>`>` and the argv it received one element per line as
//! `arg=<element>` — so *"exactly
//! one child spawn"* and *"dispatch count 0"* are file counts, `ac225_…`'s external kill has a pid
//! to `terminate_pid`, and the liveness probe the cancel tests read (`is_alive` on the marker's
//! pid) is independent of whatever job the implementation under test did or did not create.
//! Without the variable no marker is written and nothing fails: the marker is an instrument, not a
//! precondition.
//!
//! **It asserts nothing about the job it is in** (§7c.10): under `cargo test` every child inherits
//! cargo's own `KILL_ON_JOB_CLOSE` job, so a child-side reading discriminates nothing; the
//! assign-order discriminators are the parent's, in `spawn_into_job`'s hidden reading.
//!
//! **It holds itself with `std::thread::park_timeout`, never `sleep`** (ADR-0127 decision 2;
//! `clippy.toml` bans `std::thread::sleep` and `tokio::time::sleep` workspace-wide) — a loop of
//! bounded parks, because a blocking `stdin().read_line` would end the hold the moment stdin is
//! null or its writer exits, and an orphaned holder MUST outlive its parent unless the job kills
//! it. The hold is capped ([`HOLD_PARKS`] × [`HOLD_PARK`], 600 s) so a survivor of a mutant
//! containment does not sit on a developer's box for ever; no test's bound is within an order of
//! magnitude of it.
//!
//! **The modes** (§7c.17), each by name:
//!
//! | mode | what it does |
//! | --- | --- |
//! | `replay:<file>` | cat a SCRIPTED transcript file to stdout, exit 0 |
//! | `exit:<code>` | exit with that code |
//! | `hold` | write the marker, stay alive until killed (or the 600 s cap) |
//! | `close-stdin-early` | exit 0 without reading stdin — `std` cannot close an inherited stdin handle, and the exit closes it for the parent's ≥ 1 MiB write to fail on (`Transport { Io }`) |
//! | `wait-stdin-eof:<file>` | read stdin to EOF, then replay `<file>` and exit 0 — a never-shut stdin hangs to the bound |
//! | `spew:<bytes>` | write N bytes to stdout then exit 0 — N > 64 KiB deadlocks a pipe and only a file survives |
//! | `orphan` | **three-level** (§7c.10): spawn an intermediate (`orphan-mid`) that spawns the **holder** (`orphan-holder`, which writes its marker and holds) and exits; wait for the intermediate to exit and for the holder's marker to appear; print `READY holder_pid=<n>` and **hold** — so the direct child is alive when the token fires and the holder's parent is dead: a PID walk from the live child finds nothing, only the job holds the holder (AC-224's positive control) |
//! | `breakaway` | the bin's OWN attempt to leave containment: spawn a holder with `creation_flags(0x0100_0000)` (`CREATE_BREAKAWAY_FROM_JOB`, a plain `u32` `std` takes); print `BREAKAWAY spawn=ok pid=<n>` or `BREAKAWAY spawn=err os=<code>`, record the same in the marker, and hold. Under a job that never grants `BREAKAWAY_OK` the attempt must fail at spawn or land inside the job (AC-224's negative control, O-2). On POSIX the analogue is `process_group(0)` — a new group `killpg` cannot reach — which the group cannot refuse; the POSIX line is a measurement of that, not a control |
//! | `echo-env` | print every environment variable as `KEY=VALUE` in sorted order, then `CWD=<dir>` (AC-509; RT-21) |
//!
//! The intermediate and holder modes (`orphan-mid[:<n>]`, `orphan-holder`) are this bin's own;
//! `orphan` spawns `orphan-mid` (one holder), and `ac278_no_descendant_survives_exit` spawns
//! `orphan-mid:2` directly — **two** holders left behind by one exited child, so that a kill
//! which reaps only the first pid the job lists leaves a survivor and AC-278's plural THEN
//! (*"no child, background, detached or daemon process … remains alive"*) has a red of its own
//! (the test-discrimination review of Slice 2, its M-E mutant).
//!
//! **These spawns are the instrument, not a violation** (ADR-0127's forbids, first bullet, worded
//! so): a test fixture binary that spawns its own descendant to exercise containment sits outside
//! `src/`, and each spawn carries its per-site `#[expect(clippy::disallowed_types, reason = …)]`
//! naming what it is — one site, [`spawn_self`], through which every mode spawns.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

/// The hold is [`HOLD_PARKS`] bounded parks of [`HOLD_PARK`] each — 600 s absent spurious wakeups,
/// and a spurious wakeup only shortens a cap. No clock is read: `Instant::now` is clippy-banned
/// workspace-wide (`clippy.toml`, AC-001) and a counted loop of parks needs none.
const HOLD_PARK: Duration = Duration::from_secs(1);
const HOLD_PARKS: u32 = 600;

/// `orphan` waits for its holder's marker in [`MARKER_POLLS`] parks of [`MARKER_POLL`] — 30 s.
const MARKER_POLL: Duration = Duration::from_millis(5);
const MARKER_POLLS: u32 = 6_000;

/// `CREATE_BREAKAWAY_FROM_JOB`, as `std::os::windows::process::CommandExt::creation_flags` takes it.
#[cfg(windows)]
const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;

fn main() -> ExitCode {
    let mode = env::var("RK_FIXTURE_MODE").unwrap_or_else(|_| "hold".to_owned());
    let (name, arg) = match mode.split_once(':') {
        Some((n, a)) => (n.to_owned(), Some(a.to_owned())),
        None => (mode.clone(), None),
    };
    let marker_dir = env::var_os("RK_FIXTURE_MARKER_DIR").map(PathBuf::from);
    let argv: Vec<String> = env::args().collect();

    let mut extra: Vec<String> = Vec::new();
    let run = match name.as_str() {
        "replay" => replay(arg.as_deref()),
        "exit" => Ok(exit_code(arg.as_deref())),
        "hold" | "orphan-holder" => {
            write_marker(marker_dir.as_deref(), &name, &argv, &extra);
            hold();
            Ok(ExitCode::SUCCESS)
        }
        "close-stdin-early" => Ok(ExitCode::SUCCESS),
        "wait-stdin-eof" => wait_stdin_eof(arg.as_deref()),
        "spew" => spew(arg.as_deref()),
        "orphan" => orphan(marker_dir.as_deref(), &argv),
        "orphan-mid" => orphan_mid(arg.as_deref()),
        "breakaway" => breakaway(&mut extra).map(|()| {
            write_marker(marker_dir.as_deref(), &name, &argv, &extra);
            hold();
            ExitCode::SUCCESS
        }),
        "echo-env" => echo_env(),
        other => {
            eprintln!("rk-fixture-child: unknown RK_FIXTURE_MODE {other:?}");
            Ok(ExitCode::from(2))
        }
    };
    // Every mode above that did not write its marker on the way writes it here, so one invocation
    // is always one file (a holder writes it before holding; the rest write it before exiting).
    if !matches!(
        name.as_str(),
        "hold" | "orphan-holder" | "orphan" | "breakaway"
    ) {
        write_marker(marker_dir.as_deref(), &name, &argv, &extra);
    }
    match run {
        Ok(code) => code,
        Err(e) => {
            eprintln!("rk-fixture-child: {name}: {e}");
            ExitCode::from(3)
        }
    }
}

/// The marker: `pid=`, `mode=`, one `arg=` per argv element, then any mode-specific lines.
fn write_marker(dir: Option<&Path>, mode: &str, argv: &[String], extra: &[String]) {
    let Some(dir) = dir else {
        return;
    };
    let pid = std::process::id();
    let mut body = format!("pid={pid}\nmode={mode}\n");
    for a in argv {
        body.push_str("arg=");
        body.push_str(a);
        body.push('\n');
    }
    for line in extra {
        body.push_str(line);
        body.push('\n');
    }
    let tmp = dir.join(format!("{pid}.marker.tmp"));
    let done = dir.join(format!("{pid}.marker"));
    if let Err(e) = fs::write(&tmp, body).and_then(|()| fs::rename(&tmp, &done)) {
        eprintln!(
            "rk-fixture-child: marker {} not written: {e}",
            done.display()
        );
    }
}

/// Stay alive: a counted loop of bounded parks. A spurious unpark returns early from one park
/// and the loop parks again; nothing but a kill ends the hold before the cap.
fn hold() {
    for _ in 0..HOLD_PARKS {
        std::thread::park_timeout(HOLD_PARK);
    }
}

fn replay(file: Option<&str>) -> io::Result<ExitCode> {
    let file = file.ok_or_else(|| io::Error::other("replay needs a file argument"))?;
    let bytes = fs::read(file)?;
    let mut out = io::stdout().lock();
    out.write_all(&bytes)?;
    out.flush()?;
    Ok(ExitCode::SUCCESS)
}

fn exit_code(arg: Option<&str>) -> ExitCode {
    let code = arg.and_then(|a| a.parse::<u8>().ok()).unwrap_or(2);
    ExitCode::from(code)
}

fn wait_stdin_eof(file: Option<&str>) -> io::Result<ExitCode> {
    let mut sink = Vec::new();
    io::stdin().lock().read_to_end(&mut sink)?;
    replay(file)
}

fn spew(arg: Option<&str>) -> io::Result<ExitCode> {
    let n: usize = arg
        .and_then(|a| a.parse().ok())
        .ok_or_else(|| io::Error::other("spew needs a byte count"))?;
    let mut out = io::stdout().lock();
    let line = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n";
    let mut written = 0usize;
    while written < n {
        let take = line.len().min(n - written);
        out.write_all(&line[..take])?;
        written += take;
    }
    out.flush()?;
    Ok(ExitCode::SUCCESS)
}

/// `orphan`: spawn the intermediate, wait for it to exit (so the holder's parent is dead), wait
/// for the holder's marker, announce, hold.
fn orphan(marker_dir: Option<&Path>, argv: &[String]) -> io::Result<ExitCode> {
    let Some(dir) = marker_dir else {
        return Err(io::Error::other(
            "orphan mode needs RK_FIXTURE_MARKER_DIR: the holder's pid travels by marker",
        ));
    };
    let mut mid = spawn_self("orphan-mid", false)?;
    let mid_status = mid.wait()?;
    if !mid_status.success() {
        return Err(io::Error::other(format!(
            "the intermediate exited {mid_status} before the holder was up"
        )));
    }
    let holder_pid = wait_for_marker(dir, "orphan-holder")?;
    let mut extra = vec![format!("holder_pid={holder_pid}")];
    extra.push("intermediate=exited".to_owned());
    write_marker(Some(dir), "orphan", argv, &extra);
    {
        let mut out = io::stdout().lock();
        writeln!(out, "READY holder_pid={holder_pid}")?;
        out.flush()?;
    }
    hold();
    Ok(ExitCode::SUCCESS)
}

/// `orphan-mid[:<n>]`: spawn `n` holders (default 1) and exit at once — its children are now
/// orphans.
fn orphan_mid(arg: Option<&str>) -> io::Result<ExitCode> {
    let n: u32 = match arg {
        None => 1,
        Some(a) => a
            .parse()
            .map_err(|_| io::Error::other(format!("orphan-mid:{a}: not a holder count")))?,
    };
    for _ in 0..n {
        let _holder = spawn_self("orphan-holder", false)?;
    }
    Ok(ExitCode::SUCCESS)
}

/// `breakaway`: the bin's own attempt to leave containment. Records the surface it met.
fn breakaway(extra: &mut Vec<String>) -> io::Result<()> {
    match spawn_self("orphan-holder", true) {
        Ok(child) => {
            let pid = child.id();
            extra.push(format!("breakaway=ok pid={pid}"));
            let mut out = io::stdout().lock();
            writeln!(out, "BREAKAWAY spawn=ok pid={pid}")?;
            out.flush()?;
        }
        Err(e) => {
            let os = e.raw_os_error().unwrap_or(-1);
            extra.push(format!("breakaway=err os={os}"));
            let mut out = io::stdout().lock();
            writeln!(out, "BREAKAWAY spawn=err os={os}")?;
            out.flush()?;
        }
    }
    Ok(())
}

fn echo_env() -> io::Result<ExitCode> {
    let mut vars: Vec<(String, String)> = env::vars().collect();
    vars.sort();
    let mut out = io::stdout().lock();
    for (k, v) in vars {
        writeln!(out, "{k}={v}")?;
    }
    let cwd = env::current_dir()?;
    writeln!(out, "CWD={}", cwd.display())?;
    out.flush()?;
    Ok(ExitCode::SUCCESS)
}

/// Poll the marker directory (bounded parks, no sleep) until a marker with `mode=<mode>` appears,
/// and return its pid.
fn wait_for_marker(dir: &Path, mode: &str) -> io::Result<u32> {
    for _ in 0..MARKER_POLLS {
        if let Some(pid) = find_marker_pid(dir, mode)? {
            return Ok(pid);
        }
        std::thread::park_timeout(MARKER_POLL);
    }
    Err(io::Error::other(format!(
        "no marker with mode={mode} appeared in {} within {MARKER_POLLS} polls of {MARKER_POLL:?}",
        dir.display()
    )))
}

fn find_marker_pid(dir: &Path, mode: &str) -> io::Result<Option<u32>> {
    let wanted = format!("mode={mode}");
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_none_or(|e| e != "marker") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        if text.lines().any(|l| l == wanted) {
            let pid = text
                .lines()
                .find_map(|l| l.strip_prefix("pid=")?.parse::<u32>().ok());
            if let Some(pid) = pid {
                return Ok(Some(pid));
            }
        }
    }
    Ok(None)
}

/// The one spawn site: this same executable in another mode, inheriting the environment (so
/// `RK_FIXTURE_MARKER_DIR` travels) with `RK_FIXTURE_MODE` overridden, stdio inherited. With
/// `breakaway` the Windows creation flag `CREATE_BREAKAWAY_FROM_JOB` is set — the fixture's own
/// attempt, which the job under test must refuse; on POSIX the analogue is a new process group.
#[expect(
    clippy::disallowed_types,
    reason = "s7c §7c.17 / ADR-0127 forbids (first bullet): a test fixture binary that spawns its own \
              descendant in order to exercise containment is the instrument of AC-224 / AC-278, not a \
              violation of the rule that adapters spawn only through rk-containment. This bin is \
              outside src/ and is the child under test, and this is its one spawn site."
)]
fn spawn_self(mode: &str, breakaway: bool) -> io::Result<std::process::Child> {
    let me = env::current_exe()?;
    let mut cmd = std::process::Command::new(me);
    cmd.env("RK_FIXTURE_MODE", mode);
    if breakaway {
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(CREATE_BREAKAWAY_FROM_JOB);
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }
    }
    cmd.spawn()
}
