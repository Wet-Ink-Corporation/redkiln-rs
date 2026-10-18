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
//! **One marker file per invocation** into `RK_FIXTURE_MARKER_DIR` (§7c.17), named `<pid>.marker`
//! or, when that name is taken, the first free `<pid>-<n>.marker` (Windows reuses a pid across
//! sequential spawns, and a replaced marker undercounts them), written to a `.tmp` and hard-linked
//! into place so a reader never sees a partial file and no marker is ever overwritten. Readers
//! select on the `.marker` extension and take the pid from the body, never from the name. It
//! carries `pid=<own pid>`,
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
//! | `replay-then-orphan:<file>` | replay `<file>`, then spawn an `orphan-holder` with stdio INHERITED (it holds the parent's stdout/stderr pipes open), wait for its marker, and exit 0 — the direct child has EXITED while a descendant holds its pipes: the adapter's race must be decided by the child's exit and the frame it read, and the leftover reaped through the group, never a wait to the timer for an EOF the holder withholds (`s7c` §7c.10's race; AC-278's shape) |
//! | `replay-then-hold:<file>` | replay `<file>` (flushed), then hold with stdout OPEN — a frame is read and the child lives on to the token or the timer: the arm that fired classifies, and what was read before the kill is retained (RT-3 as amended; Halt 4) |
//! | `exit:<code>` | exit with that code |
//! | `hold` | write the marker, stay alive until killed (or the 600 s cap) |
//! | `close-stdin-early` | exit 0 without reading stdin — `std` cannot close an inherited stdin handle, and the exit closes it for the parent's ≥ 1 MiB write to fail on (`Transport { Io }`) |
//! | `wait-stdin-eof:<file>` | read stdin to EOF, then replay `<file>` and exit 0 — a never-shut stdin hangs to the bound |
//! | `spew:<bytes>` | write N bytes to stdout then exit 0 — N > 64 KiB deadlocks a pipe and only a file survives |
//! | `spew-after-stdin-eof:<bytes>` | read stdin to EOF, then `spew:<bytes>` — for a parent that WRITES the prompt: exit 0 with no frame is `Transport { Decode }` only when the write completed, and a child that exits unread races the write into `Transport { Io }` (added 2026-09-25, s8a: `dispatch::ac360_one_event_per_termination_path` failed that race under the whole suite's load) |
//! | `orphan` | **three-level** (§7c.10): spawn an intermediate (`orphan-mid`) that spawns the **holder** (`orphan-holder`, which writes its marker and holds) and exits; wait for the intermediate to exit and for the holder's marker to appear; print `READY holder_pid=<n>` and **hold** — so the direct child is alive when the token fires and the holder's parent is dead: a PID walk from the live child finds nothing, only the job holds the holder (AC-224's positive control) |
//! | `breakaway` | the bin's OWN attempt to leave containment: spawn a holder with `creation_flags(0x0100_0000)` (`CREATE_BREAKAWAY_FROM_JOB`, a plain `u32` `std` takes); print `BREAKAWAY spawn=ok pid=<n>` or `BREAKAWAY spawn=err os=<code>`, record the same in the marker, and hold. Under a job that never grants `BREAKAWAY_OK` the attempt must fail at spawn or land inside the job (AC-224's negative control, O-2). On POSIX the analogue is `process_group(0)` — a new group `killpg` cannot reach — which the group cannot refuse; the POSIX line is a measurement of that, not a control |
//! | `echo-env` | print every environment variable as `KEY=VALUE` in sorted order, then `CWD=<dir>` (AC-509; RT-21) |
//! | `marker-twice` | write the marker a second time and exit 0 — one pid's two markers, the collision a reused Windows pid makes, on every OS (`two_markers_with_one_pid_both_survive`, added 2026-10-18) |
//! | `script:<dir>` | ONE SCRIPTED DISPATCH PER INVOCATION (`s8j` §8j.4's binary arm, added 2026-10-18): drain stdin to EOF; take the next ordinal `n` from `<dir>/next` (absent reads `0`) and write `n + 1` back; write the tree `<dir>/<n>.files/`, if present, fresh into the current directory (a write, not a copy, so each file carries the current mtime and cargo rebuilds it) (the run-owned worktree the adapter spawns in); then print an init frame built from the child's OWN argv (`--agent`, `--model`, and the `--agents` JSON's `tools` and `permissionMode`, so the adapter's adoption and isolation checks read what it asked for) and a success result whose `structured_output` is `<dir>/<n>.json` verbatim (one line), and exit 0. A missing `<n>.json` exits 4, so a run past its script fails by name |
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

/// How many names one pid's markers may take (`<pid>.marker`, then `<pid>-1.marker` …): far more
/// spawns than any test makes reuse one pid, and a bound so a full directory fails by name.
const MARKER_NAMES: u32 = 1_000;

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
    // The modes that carry no mode-specific marker lines write their marker FIRST, before any
    // output or exit: the marker is the spawn's evidence, and a parent that kills the child on
    // its first stream line (the Harness adapter's init-time refusals, s7c §7c.4) must still find
    // it. The holders write theirs before holding; `orphan` and `breakaway` write theirs after the
    // lines their attempts add. (Moved here at Slice 5, 2026-09-18: `replay` wrote its marker after
    // the replay, and an init-time kill raced it — the marker count read 0 for a child that ran.)
    if !matches!(
        name.as_str(),
        "hold" | "orphan-holder" | "orphan" | "breakaway"
    ) {
        write_marker(marker_dir.as_deref(), &name, &argv, &extra);
    }
    let run = match name.as_str() {
        "replay" => replay(arg.as_deref()),
        "replay-then-orphan" => replay(arg.as_deref()).and_then(|code| {
            let _holder = spawn_self("orphan-holder", false)?;
            // exit only once the holder is up (its marker is the test's evidence that a
            // descendant held the pipes when the child exited)
            if let Some(dir) = marker_dir.as_deref() {
                wait_for_marker(dir, "orphan-holder")?;
            }
            Ok(code)
        }),
        "replay-then-hold" => replay(arg.as_deref()).inspect(|_| hold()),
        "exit" => Ok(exit_code(arg.as_deref())),
        "hold" | "orphan-holder" => {
            write_marker(marker_dir.as_deref(), &name, &argv, &extra);
            hold();
            Ok(ExitCode::SUCCESS)
        }
        "close-stdin-early" => Ok(ExitCode::SUCCESS),
        "wait-stdin-eof" => wait_stdin_eof(arg.as_deref()),
        "spew" => spew(arg.as_deref()),
        "spew-after-stdin-eof" => spew_after_stdin_eof(arg.as_deref()),
        "orphan" => orphan(marker_dir.as_deref(), &argv),
        "orphan-mid" => orphan_mid(arg.as_deref()),
        "breakaway" => breakaway(&mut extra).map(|()| {
            write_marker(marker_dir.as_deref(), &name, &argv, &extra);
            hold();
            ExitCode::SUCCESS
        }),
        "echo-env" => echo_env(),
        "marker-twice" => {
            write_marker(marker_dir.as_deref(), &name, &argv, &extra);
            Ok(ExitCode::SUCCESS)
        }
        "script" => script(arg.as_deref(), &argv),
        other => {
            eprintln!("rk-fixture-child: unknown RK_FIXTURE_MODE {other:?}");
            Ok(ExitCode::from(2))
        }
    };
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
    if let Err(e) = publish_marker(dir, pid, &body) {
        let warning = writeln!(
            io::stderr().lock(),
            "rk-fixture-child: marker for pid {pid} in {}: {e}",
            dir.display()
        );
        if warning.is_err() {
            // stderr is closed, so the warning has no channel left; the missing marker is itself
            // the report a reader's count sees, and the marker never decides this child's exit.
        }
    }
}

/// Publish `body` as a marker that replaces no other: staged whole in `<pid>.marker.tmp`, then
/// hard-linked to the first free name of `<pid>.marker`, `<pid>-1.marker`, … `<pid>-<n>.marker`.
/// A hard link fails on an existing name where `rename` replaces it, and Windows reuses a pid
/// across sequential spawns quickly, so a renamed marker cost the earlier child its file and a
/// spawn count read one short. The link is atomic, so a reader still never sees a partial file.
/// The staging file is removed whether or not the link succeeded; a pid has one live process, so
/// no other writer shares it.
fn publish_marker(dir: &Path, pid: u32, body: &str) -> io::Result<()> {
    let tmp = dir.join(format!("{pid}.marker.tmp"));
    fs::write(&tmp, body)?;
    let linked = link_first_free(&tmp, dir, pid);
    let removed = fs::remove_file(&tmp);
    linked.map_err(|e| io::Error::new(e.kind(), format!("not written: {e}")))?;
    removed.map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("published, but staging {} not removed: {e}", tmp.display()),
        )
    })
}

/// Hard-link `tmp` to the first of [`MARKER_NAMES`] candidate names that does not exist yet.
fn link_first_free(tmp: &Path, dir: &Path, pid: u32) -> io::Result<()> {
    for n in 0..MARKER_NAMES {
        let name = if n == 0 {
            format!("{pid}.marker")
        } else {
            format!("{pid}-{n}.marker")
        };
        match fs::hard_link(tmp, dir.join(name)) {
            Ok(()) => return Ok(()),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!("all {MARKER_NAMES} marker names for pid {pid} are taken"),
    ))
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

/// `spew-after-stdin-eof:<bytes>`: drain stdin to EOF, then [`spew`] — so a parent's prompt write always
/// completes before the exit (added 2026-09-25, s8a).
fn spew_after_stdin_eof(arg: Option<&str>) -> io::Result<ExitCode> {
    let mut sink = Vec::new();
    io::stdin().lock().read_to_end(&mut sink)?;
    spew(arg)
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

/// `script:<dir>`: one scripted dispatch (the module table's row).
fn script(dir: Option<&str>, argv: &[String]) -> io::Result<ExitCode> {
    let dir = PathBuf::from(dir.ok_or_else(|| io::Error::other("script needs a directory"))?);
    let mut sink = Vec::new();
    io::stdin().lock().read_to_end(&mut sink)?;
    let counter = dir.join("next");
    let n: u32 = match fs::read_to_string(&counter) {
        Ok(text) => text
            .trim()
            .parse()
            .map_err(|e| io::Error::other(format!("{}: {e}", counter.display())))?,
        Err(e) if e.kind() == io::ErrorKind::NotFound => 0,
        Err(e) => return Err(e),
    };
    fs::write(&counter, format!("{}\n", n.saturating_add(1)))?;
    let Ok(output) = fs::read_to_string(dir.join(format!("{n}.json"))) else {
        writeln!(
            io::stderr().lock(),
            "rk-fixture-child: script: no dispatch {n} in {}",
            dir.display()
        )?;
        return Ok(ExitCode::from(4));
    };
    let files = dir.join(format!("{n}.files"));
    if files.is_dir() {
        copy_tree(&files, &env::current_dir()?)?;
    }
    let flag = |name: &str| {
        argv.iter()
            .position(|a| a == name)
            .and_then(|i| argv.get(i + 1))
            .map_or("", String::as_str)
    };
    let agents = flag("--agents");
    let tools = json_member(agents, "\"tools\":", b']').unwrap_or("[]");
    let mode = json_member(agents, "\"permissionMode\":", b'"').unwrap_or("\"\"");
    let mut out = io::stdout().lock();
    writeln!(
        out,
        "{{\"agent\":{agent},\"apiKeySource\":\"none\",\"claude_code_version\":\"2.1.278\",\
         \"cwd\":\"<worktree>\",\"mcp_servers\":[],\"model\":{model},\"permissionMode\":{mode},\
         \"session_id\":\"script\",\"slash_commands\":[],\"subtype\":\"init\",\"tools\":{tools},\
         \"type\":\"system\",\"uuid\":\"script-init-{n}\"}}",
        agent = json_string(flag("--agent")),
        model = json_string(flag("--model")),
    )?;
    writeln!(
        out,
        "{{\"duration_api_ms\":1000,\"duration_ms\":1234,\"is_error\":false,\"num_turns\":1,\
         \"result\":\"scripted\",\"session_id\":\"script\",\"structured_output\":{},\
         \"subtype\":\"success\",\"total_cost_usd\":0.0031,\"type\":\"result\",\"usage\":\
         {{\"cache_creation_input_tokens\":0,\"cache_read_input_tokens\":0,\"input_tokens\":10,\
         \"output_tokens\":20}},\"uuid\":\"script-result-{n}\"}}",
        output.trim()
    )?;
    out.flush()?;
    Ok(ExitCode::SUCCESS)
}

/// Write every file under `from` to the same relative path under `to`, creating directories.
fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    for entry in fs::read_dir(from)? {
        let path = entry?.path();
        let Some(name) = path.file_name() else {
            continue;
        };
        let target = to.join(name);
        if path.is_dir() {
            fs::create_dir_all(&target)?;
            copy_tree(&path, &target)?;
        } else {
            // Not `fs::copy`: on Windows (`CopyFileExW`) it keeps the source's mtime, older than
            // the fixture's last build, so cargo would not rebuild the scripted edit. A fresh write
            // stamps the current time, as the in-process double's `fs::write` does.
            fs::write(&target, fs::read(&path)?)?;
        }
    }
    Ok(())
}

/// The raw JSON text of the value after `key` in `json`, through its closing `close` byte: an
/// array of plain strings (`]`) or a string (`"`, the value's own opening quote skipped). A
/// `key` cannot occur inside a JSON string, whose quotes are escaped.
fn json_member<'a>(json: &'a str, key: &str, close: u8) -> Option<&'a str> {
    let start = json.find(key)? + key.len();
    let rest = json.get(start..)?;
    let from = usize::from(close == b'"');
    let end = rest.bytes().skip(from).position(|b| b == close)? + from + 1;
    rest.get(..end)
}

/// `s` as a JSON string literal (quotes, backslashes and control characters escaped).
fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if u32::from(c) < 0x20 => {
                out.push_str("\\u00");
                for nibble in [u32::from(c) >> 4, u32::from(c) & 0xf] {
                    out.extend(char::from_digit(nibble, 16));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
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
