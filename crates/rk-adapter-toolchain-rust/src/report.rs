//! The `VerifyReport`, produced by an inherent method from structured sources only.
//!
//! The `VerifyReport` — `rk_types::verify::VerifyReport`, produced by an INHERENT method
//! (`RustToolchain::verify_report`, RT-6: the port stays six methods) from **structured sources
//! only** (AC-329's adapter half, `s7c` §7c.14).
//!
//! **How the counts are read on the pinned STABLE toolchain — RT-26.** `libtest --format json`
//! is nightly-only on 1.98.1 and `--format junit` is gated the same way [measured 2026-09-17];
//! nextest is ABSENT on this box [measured 2026-09-17]; `RUSTC_BOOTSTRAP=1` is a workaround
//! Halt 7 forbids. So the counts come from **libtest's stable `--list`**: every line ending in
//! `: test` is one test that would run under the given filter and flags ([`count_listed`] — the
//! per-test list, never the summary line AC-329 bans, which this module never reads; the block's
//! `rk_absent` polices the spellings over this crate's `src/`). The two `N tests, M benchmarks`
//! trailer lines `--list` prints are not `: test` lines and are not counted.
//!
//! - `selected_test_count` = the `: test` lines of `cargo test --workspace -- <selection> --list`
//!   **minus** those of `… --list --ignored` (plain `--list` lists the ignored tests too;
//!   `--list --ignored` prints only the ignored set [measured 2026-09-17 and again 2026-09-18 on
//!   the fixture: 4 and 2]) — **unless the selection itself carries `--ignored`** (then the run
//!   executes only the ignored set and the listing already is that set, and a second `--ignored`
//!   would be getopts' *"Option 'ignored' given more than once"*) **or `--include-ignored`** (then
//!   the whole listing runs). [`count_vectors`] is that rule, and `ac325_…` asserts each vector it
//!   returns was handed to the OS.
//! - `total_test_count` = **`selected_test_count`**, with `total_test_count_source:
//!   OwnExecutedCount` — because `basis` is always `WholeSuite` here (AC-328, below) and on a
//!   whole-suite report the store's boundary validator (`rk_store::artifacts::verify_report::
//!   validate_report`, ADR-0104 decision 3's implementation) requires exactly `(executed,
//!   OwnExecutedCount)` and refuses any other pair. **RT-26's *"`total_test_count` from the
//!   unfiltered `--list`"* cannot coexist with a `WholeSuite` basis** — a report carrying it
//!   (`selected: 1, total: 2`) is refused at the boundary AC-329 says it is validated at
//!   [measured 2026-09-18 against `total_test_count_fields`]; an accepted ADR's implementation
//!   outranks a routing (ADR-0092), so the unfiltered listing is NOT read and that reading is
//!   the input to RT-26's OWED note against ADR-0094 decision 1. The whole-suite comparison a
//!   selected run needs (AC-326's `selection_ineffective`) is the kernel's, against the recorded
//!   whole-suite run — not a second number on this report.
//! - pass / fail = **the run's exit status** (RT-22: `Err(NonZero { code, stderr_digest })` on a
//!   red suite, the digest over the retained stderr bytes; `Ok(report)` only on a green one).
//!
//! **What a killed run is** (AC-331; ADR-0127's forbids): `run_bounded`'s `TimedOut` becomes
//! `StepError::Timeout { after_secs }` — the construction-time bound's own value, since the
//! toolchain has no token and `Cancelled { by }` is keyed on one (AC-225) — and `OsKilled`
//! becomes `Transport { kind: Io }` whose `detail` starts with [`OS_KILLED_DETAIL_PREFIX`]
//! ([`os_killed`]: the shape 7d keys on, since the same variant and kind also carry an unreadable
//! output file — the variant is RT-3's OWED record's); **neither is ever `NonZero` and neither
//! is ever `Ok`** — the input 7d's verify runner maps to `Unverifiable { cannot_run }`. A program that
//! cannot start is `Transport { kind: Spawn }` (AC-344's adapter share, [`crate::run`]).
//!
//! **The other members.** `argv` is the RUN vector as handed to the OS (AC-325) — the program as
//! the `Config` names it, then `test --workspace [-- <selection>]` — never a template and never a
//! `nextest` element on this box (ADR-0094 decision 1's *"or `cargo nextest run` where present,
//! recorded on the report"*: the fallback is what is recorded; the presence arm is unobservable
//! here and ARMED). `toolchain_version` is `rustc -V`'s answer, read by running the `rustc` beside
//! the configured program when one is there and a bare `rustc` (the parent's `PATH`) otherwise
//! — **never a hard-coded `+1.98.1`** (ADR-0102/ADR-0083: two copies of the pin, not three).
//! `suite_sha` is a sha256 over the suite's SOURCES — every `Cargo.toml`, `Cargo.lock`, `build.rs`
//! and `*.rs` under the project root, `target/` excluded, as length-prefixed `(relative path,
//! bytes)` pairs in path order (`rk_types::digest::sha256_lp`) — the same tree digests the same,
//! and a changed test file changes it. `wall_secs` is the run's own wall time (the two
//! `Instant::now` sites in [`crate::run`]). `basis` is **always `WholeSuite`** with
//! `basis_reason: ProbeUnverified` — AC-328: no `affected-selection` probe is recorded at any
//! pin, so no `basis: affected` record can be produced by this adapter; `total_test_count_source`
//! is `OwnExecutedCount`, the one variant the closed enum admits without inventing a run id
//! (`RecordedRun` is an affected run's), and its count is literally this run's own (the rule
//! above).

use rk_containment::Outcome;
use rk_types::digest::{Sha256Stream, sha256};
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

use crate::RustToolchain;
use crate::run::{self, Run};

/// The cargo subcommand vector ADR-0094 decision 1 fixes, before any selection.
const RUN_ARGS: [&str; 2] = ["test", "--workspace"];

/// The detail prefix every OS-killed run carries.
///
/// The first bytes of an OS-killed run's `Transport { kind: Io, detail }` — the one thing that
/// tells it from the crate's other `Transport { Io }` errors (an unreadable output file, a
/// containment primitive failing around the run) without reading prose: 7d's verify runner keys
/// `Unverifiable { cannot_run }` on `detail.starts_with(OS_KILLED_DETAIL_PREFIX)`. A routing:
/// `rk_types::StepError` carries no OS-kill variant (`Timeout` is the bound's, `Cancelled { by }`
/// the token's — AC-225), and which variant a killed toolchain run should be is RT-3's OWED
/// record's to decide (ADR-0127 §7); until it does, this prefix is the contract.
pub const OS_KILLED_DETAIL_PREFIX: &str = "os-killed:";

/// The error an OS-killed run is (AC-331: unverifiable, never a status) — every `OsKilled` arm in
/// the crate goes through here, so the prefix above is on all of them.
#[must_use]
pub fn os_killed(argv: &[String]) -> StepError {
    StepError::Transport {
        kind: TransportKind::Io,
        detail: format!(
            "{OS_KILLED_DETAIL_PREFIX} the OS ended the toolchain child (an error-severity \
             termination) while running {argv:?}: unverifiable, not a failing test (AC-331)"
        ),
    }
}

/// The number of `: test` lines in a `--list` stdout — libtest's one-line-per-test list.
#[must_use]
pub fn count_listed(stdout: &str) -> u32 {
    let n = stdout
        .lines()
        .filter(|l| l.trim_end().ends_with(": test"))
        .count();
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// The run vector's arguments (after the program): `test --workspace`, then `--` and the
/// selection when there is one.
#[must_use]
pub fn run_args(selection: &[String]) -> Vec<String> {
    let mut args: Vec<String> = RUN_ARGS.iter().map(|s| (*s).to_owned()).collect();
    if !selection.is_empty() {
        args.push("--".to_owned());
        args.extend(selection.iter().cloned());
    }
    args
}

/// The count vectors for a selection: the listing, and the `--ignored` listing to subtract.
///
/// The count vectors for a selection (RT-26; the module doc): `(listing, subtract)` — the
/// listing vector, and the `--ignored` listing to subtract when the selection carries neither
/// `--ignored` nor `--include-ignored`.
#[must_use]
pub fn count_vectors(selection: &[String]) -> (Vec<String>, Option<Vec<String>>) {
    let mut list = run_args(selection);
    if selection.is_empty() {
        list.push("--".to_owned());
    }
    list.push("--list".to_owned());
    let ignored_only = selection.iter().any(|s| s == "--ignored");
    let include_ignored = selection.iter().any(|s| s == "--include-ignored");
    if ignored_only || include_ignored {
        return (list, None);
    }
    let mut subtract = list.clone();
    subtract.push("--ignored".to_owned());
    (list, Some(subtract))
}

/// A green run's stdout, or the typed error a non-green outcome is.
fn expect_green(run: &Run, timeout_secs: u64) -> Result<Vec<u8>, StepError> {
    match run.outcome {
        Outcome::Exited(status) if status.success() => run.stdout_bytes(),
        Outcome::Exited(status) => Err(StepError::NonZero {
            code: status.code(),
            stderr_digest: sha256(&run.stderr_bytes()?),
        }),
        // AC-331: a run killed at the bound or by the OS is never a status (ADR-0127's forbids).
        Outcome::TimedOut => Err(StepError::Timeout {
            after_secs: timeout_secs,
        }),
        Outcome::OsKilled => Err(os_killed(&run.argv)),
    }
}

/// The listed count of one `--list` vector, run green.
fn listed(
    tc: &RustToolchain,
    dir: &std::path::Path,
    tag: &str,
    args: &[String],
) -> Result<u32, StepError> {
    let run = run::run(tc.config(), dir, tag, args)?;
    let stdout = expect_green(&run, tc.config().timeout().as_secs())?;
    Ok(count_listed(&String::from_utf8_lossy(&stdout)))
}

/// The count under a selection: the listing minus the ignored listing (the module doc).
fn counted(
    tc: &RustToolchain,
    dir: &std::path::Path,
    tag: &str,
    selection: &[String],
) -> Result<u32, StepError> {
    let (list, subtract) = count_vectors(selection);
    let all = listed(tc, dir, &format!("{tag}-list"), &list)?;
    let ignored = match subtract {
        Some(v) => listed(tc, dir, &format!("{tag}-list-ignored"), &v)?,
        None => 0,
    };
    Ok(all.saturating_sub(ignored))
}

/// `rustc -V`: the `rustc` beside the configured program when one is there, else a bare `rustc`.
fn toolchain_version(tc: &RustToolchain, dir: &std::path::Path) -> Result<String, StepError> {
    let program = tc.config().program();
    let sibling = program.parent().and_then(|dir| {
        let exe = if cfg!(windows) { "rustc.exe" } else { "rustc" };
        let candidate = dir.join(exe);
        candidate.is_file().then_some(candidate)
    });
    let rustc = sibling.unwrap_or_else(|| std::path::PathBuf::from("rustc"));
    let run = run::run_program(
        tc.config(),
        dir,
        "rustc-version",
        &rustc,
        &["-V".to_owned()],
    )?;
    let stdout = expect_green(&run, tc.config().timeout().as_secs())?;
    let version = String::from_utf8_lossy(&stdout).trim().to_owned();
    if version.is_empty() {
        return Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail: "`rustc -V` printed nothing; the report's toolchain_version has no source"
                .to_owned(),
        });
    }
    Ok(version)
}

/// sha256 over the suite's sources (the module doc).
fn suite_sha(tc: &RustToolchain) -> Result<String, StepError> {
    let root = std::path::Path::new(tc.config().project_root().as_str());
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    collect_sources(root, &mut files)?;
    files.sort();
    let mut h = Sha256Stream::start();
    for f in &files {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(f).map_err(|e| StepError::Transport {
            kind: TransportKind::Io,
            detail: format!("could not read {} for the suite digest: {e}", f.display()),
        })?;
        let len = u32::try_from(rel.len()).unwrap_or(u32::MAX);
        h.update(&len.to_be_bytes());
        h.update(rel.as_bytes());
        let len = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
        h.update(&len.to_be_bytes());
        h.update(&bytes);
    }
    Ok(h.finish().as_str().to_owned())
}

fn collect_sources(
    dir: &std::path::Path,
    out: &mut Vec<std::path::PathBuf>,
) -> Result<(), StepError> {
    let entries = std::fs::read_dir(dir).map_err(|e| StepError::Transport {
        kind: TransportKind::Io,
        detail: format!("could not list {} for the suite digest: {e}", dir.display()),
    })?;
    for entry in entries {
        let entry = entry.map_err(|e| StepError::Transport {
            kind: TransportKind::Io,
            detail: format!("could not list {} for the suite digest: {e}", dir.display()),
        })?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if name == "target" || name == ".git" || name == "node_modules" {
                continue;
            }
            collect_sources(&path, out)?;
        } else if name == "Cargo.toml"
            || name == "Cargo.lock"
            || name == "build.rs"
            || name.ends_with(".rs")
        {
            out.push(path);
        }
    }
    Ok(())
}

/// The report for one selection (the module doc).
///
/// # Errors
/// `NonZero` on a red suite (RT-22); `Timeout { after_secs }` on a run killed at the bound and
/// `Transport { Io }` on one the OS ended (AC-331); `Transport { Spawn }` when the program cannot
/// start (AC-344); `Transport { Io | Decode }` around the files and the version.
pub fn verify_report(tc: &RustToolchain, selection: &[String]) -> Result<VerifyReport, StepError> {
    let dir = tc.run_dir()?;
    let toolchain_version = toolchain_version(tc, &dir)?;
    let selected_test_count = counted(tc, &dir, "selected", selection)?;
    // The basis is WholeSuite (AC-328), so the total IS the run's own executed count — the
    // store's rule (ADR-0104 d3; the module doc) — and no unfiltered listing is spawned.
    let total_test_count = selected_test_count;

    let args = run_args(selection);
    let run = run::run(tc.config(), &dir, "run", &args)?;
    expect_green(&run, tc.config().timeout().as_secs())?;

    Ok(VerifyReport {
        basis: Basis::WholeSuite,
        argv: run.argv,
        selected_test_count,
        total_test_count,
        total_test_count_source: TotalTestCountSource::OwnExecutedCount,
        wall_secs: run.wall.as_secs(),
        toolchain_version,
        suite_sha: suite_sha(tc)?,
        basis_reason: Some(BasisReason::ProbeUnverified),
    })
}
