//! The `VerifyReport`, produced by an inherent method from structured sources only.
//!
//! The `VerifyReport` — `rk_types::verify::VerifyReport`, produced by an INHERENT method
//! (`RustToolchain::verify_report`, ~~RT-6: the port stays six methods~~ *(struck 2026-09-26, s8c §8c.7: RT-6 is discharged and the port has seven methods under record V; the dated paragraph below)*) from **structured sources
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
//!
//! *(Amended 2026-09-26, s8c §8c.1-§8c.2; record T = ADR-0169, proposed. Much of the above is
//! superseded, and stays as the record of what was; read this paragraph as the current contract.)*
//!
//! - **One path, the port's.** The INHERENT `RustToolchain::verify_report` is retired (RT-6 is
//!   discharged by record T decision 2); [`verify_report`] here is `Toolchain::test`'s body and
//!   returns a `rk_ports::toolchain::TestOutcome`, the unchanged `VerifyReport` plus `passed`,
//!   `failed`, the derived `status` and the `runner`. *"RT-6: the port stays six methods"* is
//!   false twice over: RT-6 is discharged, and under record V = ADR-0170 the port has SEVEN
//!   methods (Checkpoint 7a counts none; E-0162-2).
//! - **Pass / fail is no longer the exit status alone (RT-22 is REVERSED).** The per-test counts
//!   come from the strict line parsers pinned per version: [`crate::libtest_lines::parse`] over
//!   the `cargo test` run child's STDOUT file, and [`crate::nextest_lines::parse`] over the
//!   `cargo nextest run` child's STDERR file (D21-43 N4; D21-48 X15). **A red suite is `Ok`** with
//!   `failed ≥ 1`. `Err` is exactly three things: cannot run (the program cannot start --
//!   `Transport { Spawn }`; or a `--list` pass that fails, which is the build failing before any
//!   test executes -- `NonZero`), killed (`Timeout`, the `os-killed:` `Transport { Io }`), or
//!   unreadable (`Transport { Io }`, and every parser refusal: `Transport { Decode }` naming the
//!   refusal and the version; an unpinned version is refused by name as `Unsupported {
//!   capability: "libtest-lines@<v>" | "nextest-lines@<v>" }`, rule 1's spelling).
//! - **`--list` stays the source of `selected_test_count`** (RT-26, unchanged), and record T
//!   decision 1's invariant `passed + failed == selected_test_count` is checked at construction:
//!   a breach is `Transport { Decode }` naming both numbers, never a corrected count. That is what
//!   makes the stdout decoy's phantom `... ok` over-count visible, and what makes a nextest run
//!   whose doctests were dropped refuse rather than read as a smaller green suite.
//! - **`--no-fail-fast` on every run child** (record T decision 6; §8c.1 rule 4): `RUN_ARGS` is
//!   `test --workspace --no-fail-fast`, so a red test in an early binary still leaves every later
//!   block to be tallied.
//! - **The runner the adapter selected at construction** ([`crate::Runner`]; ADR-0094 decision 1's
//!   *"(or `cargo nextest run` where present, recorded on the report)"*, executed by record T
//!   decision 4). On the nextest path the run child is `cargo nextest run` with [`NEXTEST_ARGS`],
//!   and, because nextest runs no doctests, a **doctest companion** child, `cargo test --workspace
//!   --doc --no-fail-fast [-- <args>]`, runs beside it under the same bound when a workspace
//!   member has a doctest-bearing library target (read from `cargo metadata --format-version 1
//!   --no-deps`: a target of a `lib`-kind with `doctest: true`). The outcome sums both tallies and
//!   both wall times; `runner.companion_argv` records the companion's vector, and is `None` when
//!   none ran. `VerifyReport.argv` is the RUN vector as handed to the OS (AC-325), so it names
//!   `nextest` exactly when nextest ran. A nextest list failure (exit 104, D21-62 B) refuses as
//!   `ListFailed`; nothing falls back to `cargo test`.
//! - **The selection is a closed `Selection`** (record T decision 7): `WholeSuite { reason }` runs
//!   the whole suite and records `reason`; `Filter(args)` passes `-- <args>` as before, with
//!   `ProbeUnverified`; `Affected` is refused from the `affected-selection` registry row until the
//!   Affected rules land (§8c.3), and a `Named` selection is refused by name (the adapter declares
//!   none; `"e2e"` included).
//!   *(Dated 2026-09-26, s8c build, P5: the Affected rules have landed.)* `Affected` runs record T
//!   decision 7's rules in order: the row must be `Recorded` and the change set non-empty, both
//!   before any spawn ([`affected_gate`]); `cargo metadata --format-version 1` and the pure
//!   [`crate::affected::affected_packages`] give the set, a path in no package running the whole
//!   suite with `PolicyForced`; no recorded whole-suite count, or one at another `rustc -V`, runs
//!   it with `ProbeUnverified`; a `-p` set that lists no test escalates with `SelectionEmpty`;
//!   otherwise the `-p` set ([`Scope::Packages`], in place of `--workspace`) runs with `basis:
//!   Affected`, no reason, and the recorded run's count as `total_test_count` with
//!   `RecordedRun { run_id }`. Every child runs under `min(step, construction)` (record B).

use rk_containment::Outcome;
use std::path::Path;

use rk_ports::toolchain::{Selection, StepBound, TestOutcome, TestRunner};
use rk_types::digest::{Sha256Stream, sha256};
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

use crate::affected::{self, AFFECTED_POLICY, AffectedSet, Metadata};
use crate::libtest_lines::{self, ParseRefusal, Tally};
use crate::nextest_lines;
use crate::run::{self, Run};
use crate::{Config, Runner, RustToolchain, registry};

/// The cargo subcommand vector ADR-0094 decision 1 fixes, before any selection -- with
/// `--no-fail-fast`, record T = ADR-0169 decision 6's deviation (§8c.1 rule 4). *(Amended
/// 2026-09-26, s8c: this was `["test", "--workspace"]`.)*
const RUN_ARGS: [&str; 3] = ["test", "--workspace", "--no-fail-fast"];

/// The nextest run vector's arguments, before any selection (record T decision 4, measured on
/// `0.9.146`): each executed test prints exactly one status line on STDERR.
pub const NEXTEST_ARGS: [&str; 12] = [
    "nextest",
    "run",
    "--workspace",
    "--no-fail-fast",
    "--retries",
    "0",
    "--color",
    "never",
    "--status-level",
    "pass",
    "--final-status-level",
    "none",
];

/// The doctest companion's vector, before any selection (record T decision 5): `cargo nextest
/// run` runs no doctests, so this child runs them under libtest's grammar.
const DOC_ARGS: [&str; 4] = ["test", "--workspace", "--doc", "--no-fail-fast"];

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

/// Which packages a vector names.
///
/// The whole workspace (`--workspace`), or an `Affected` run's `-p` set (record T = ADR-0169 decision 7 rule 4, proposed; its supersession in part of ADR-0094
/// decision 1's *"`test` = `cargo test --workspace`"*). *(Added 2026-09-26, s8c build, P5.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    /// `--workspace`.
    Workspace,
    /// `-p <name>` for each, in order.
    Packages(Vec<String>),
}

/// `head` with its `--workspace` element replaced by `scope`'s flags.
fn scoped(head: &[&str], scope: &Scope) -> Vec<String> {
    let mut args = Vec::with_capacity(head.len() + 2);
    for element in head {
        match (*element, scope) {
            ("--workspace", Scope::Packages(names)) => {
                for name in names {
                    args.push("-p".to_owned());
                    args.push(name.clone());
                }
            }
            (other, _) => args.push(other.to_owned()),
        }
    }
    args
}

/// The run vector's arguments (after the program): `test --workspace --no-fail-fast`, then `--`
/// and the selection when there is one.
#[must_use]
pub fn run_args(selection: &[String]) -> Vec<String> {
    run_args_in(&Scope::Workspace, selection)
}

/// [`run_args`] over a scope.
#[must_use]
pub fn run_args_in(scope: &Scope, selection: &[String]) -> Vec<String> {
    with_selection(scoped(&RUN_ARGS, scope), selection)
}

/// The nextest run vector's arguments (after the program), then `--` and the selection.
#[must_use]
pub fn nextest_args(selection: &[String]) -> Vec<String> {
    nextest_args_in(&Scope::Workspace, selection)
}

/// [`nextest_args`] over a scope.
#[must_use]
pub fn nextest_args_in(scope: &Scope, selection: &[String]) -> Vec<String> {
    with_selection(scoped(&NEXTEST_ARGS, scope), selection)
}

/// The doctest companion's arguments (after the program), then `--` and the selection.
#[must_use]
pub fn doc_args(selection: &[String]) -> Vec<String> {
    doc_args_in(&Scope::Workspace, selection)
}

/// [`doc_args`] over a scope.
#[must_use]
pub fn doc_args_in(scope: &Scope, selection: &[String]) -> Vec<String> {
    with_selection(scoped(&DOC_ARGS, scope), selection)
}

fn with_selection(mut args: Vec<String>, selection: &[String]) -> Vec<String> {
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
    count_vectors_in(&Scope::Workspace, selection)
}

/// [`count_vectors`] over a scope.
#[must_use]
pub fn count_vectors_in(scope: &Scope, selection: &[String]) -> (Vec<String>, Option<Vec<String>>) {
    let mut list = run_args_in(scope, selection);
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

/// A run that EXITED -- green or red -- and its exit code; the typed error a killed run is.
/// (A run child's non-zero exit is a red suite's or an unreadable one's, and the parser decides
/// which: record T decision 1.)
fn exited(run: &Run, timeout_secs: u64) -> Result<i32, StepError> {
    match run.outcome {
        Outcome::Exited(status) => status.code().ok_or_else(|| StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "the run child {:?} exited with no exit code (a signal): no verdict is read",
                run.argv
            ),
        }),
        Outcome::TimedOut => Err(StepError::Timeout {
            after_secs: timeout_secs,
        }),
        Outcome::OsKilled => Err(os_killed(&run.argv)),
    }
}

/// A parser refusal as the port carries it (§8c.1: *"The adapter maps every `ParseRefusal` to
/// `Err(Transport { Decode })`, naming the refusal and the version"*). An UNPINNED version is
/// refused by name in rule 1's own spelling, `Unsupported { capability: "<parser>@<version>" }`:
/// the same cannot-run class for every reader (the review re-executor, the verify pass), and
/// never a green.
fn refused(refusal: &ParseRefusal, parser: &str, version: &str, argv: &[String]) -> StepError {
    match refusal {
        ParseRefusal::Unsupported { capability } => StepError::Unsupported {
            capability: capability.clone(),
        },
        other => StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "{parser}@{version} refused the output of {argv:?} as {}: {other} -- Unverifiable, \
                 never green (D21-43 N4)",
                other.name()
            ),
        },
    }
}

/// The listed count of one `--list` vector, run green.
///
/// *(Amended 2026-09-26, s8c build, P5: every helper below takes the call's bounded `Config`
/// (record B = ADR-0171 decision 4, proposed), so each child runs under `min(step,
/// construction)` and a `Timeout` names the bound that fired.)*
fn listed(cfg: &Config, dir: &Path, tag: &str, args: &[String]) -> Result<u32, StepError> {
    let run = run::run(cfg, dir, tag, args)?;
    let stdout = expect_green(&run, cfg.timeout().as_secs())?;
    Ok(count_listed(&String::from_utf8_lossy(&stdout)))
}

/// The count under a scope and a selection: the listing minus the ignored listing (the module
/// doc).
fn counted(
    cfg: &Config,
    dir: &Path,
    tag: &str,
    scope: &Scope,
    selection: &[String],
) -> Result<u32, StepError> {
    let (list, subtract) = count_vectors_in(scope, selection);
    let all = listed(cfg, dir, &format!("{tag}-list"), &list)?;
    let ignored = match subtract {
        Some(v) => listed(cfg, dir, &format!("{tag}-list-ignored"), &v)?,
        None => 0,
    };
    Ok(all.saturating_sub(ignored))
}

/// `rustc -V`: the `rustc` beside the configured program when one is there, else a bare `rustc`.
///
/// # Errors
/// As a green run of `rustc -V`; `Transport { Decode }` when it prints nothing.
pub(crate) fn toolchain_version(cfg: &Config, dir: &Path) -> Result<String, StepError> {
    let program = cfg.program();
    let sibling = program.parent().and_then(|dir| {
        let exe = if cfg!(windows) { "rustc.exe" } else { "rustc" };
        let candidate = dir.join(exe);
        candidate.is_file().then_some(candidate)
    });
    let rustc = sibling.unwrap_or_else(|| std::path::PathBuf::from("rustc"));
    let run = run::run_program(cfg, dir, "rustc-version", &rustc, &["-V".to_owned()])?;
    let stdout = expect_green(&run, cfg.timeout().as_secs())?;
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

/// The release a `rustc -V` line names: `rustc 1.98.1 (48a229cea 2026-09-01)` is `1.98.1`, the
/// key `libtest_lines::PINNED` is listed by.
///
/// # Errors
/// `Transport { Decode }` naming the line when it names no release.
pub fn rustc_release(version: &str) -> Result<&str, StepError> {
    let mut words = version.split_whitespace();
    match (words.next(), words.next()) {
        (Some("rustc"), Some(release)) => Ok(release),
        _ => Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "`rustc -V` answered {version:?}, which names no release; the libtest parser is \
                 pinned per release"
            ),
        }),
    }
}

/// sha256 over the suite's sources (the module doc).
fn suite_sha(cfg: &Config) -> Result<String, StepError> {
    let root = Path::new(cfg.project_root().as_str());
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

fn collect_sources(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<(), StepError> {
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

/// The `cargo metadata` target kinds that are a library, whose doctests `cargo test --doc` runs.
const LIB_KINDS: [&str; 4] = ["lib", "rlib", "dylib", "proc-macro"];

/// `cargo metadata --format-version 1 [--no-deps]`, parsed: a structured read of cargo's stable
/// JSON, never prose.
fn metadata(
    cfg: &Config,
    dir: &Path,
    tag: &str,
    no_deps: bool,
) -> Result<serde_json::Value, StepError> {
    let mut args: Vec<String> = ["metadata", "--format-version", "1"]
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    if no_deps {
        args.push("--no-deps".to_owned());
    }
    let run = run::run(cfg, dir, tag, &args)?;
    let stdout = expect_green(&run, cfg.timeout().as_secs())?;
    serde_json::from_slice(&stdout).map_err(|e| StepError::Transport {
        kind: TransportKind::Decode,
        detail: format!("`cargo metadata --format-version 1` did not parse: {e}"),
    })
}

/// The doctest companion's scope, when one runs at all. Read from `cargo metadata
/// --format-version 1 --no-deps` (record T decision 5: *"When the selected packages have no
/// library target (read from `cargo metadata`), no companion runs"*): a package carrying a target
/// whose `kind` names a library (`lib`, `rlib`, `dylib`, `proc-macro`) and whose `doctest` is
/// `true`. Over the workspace it is `--workspace` when any member carries one; over an `Affected`
/// `-p` set it is those of the set that carry one, since `cargo test --doc -p <name>` refuses a
/// package with no library. `None` when no selected package carries one.
fn doc_scope(cfg: &Config, dir: &Path, scope: &Scope) -> Result<Option<Scope>, StepError> {
    let metadata = metadata(cfg, dir, "metadata", true)?;
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| StepError::Transport {
            kind: TransportKind::Decode,
            detail: "`cargo metadata --format-version 1` carries no `packages` array".to_owned(),
        })?;
    let bearing: Vec<String> = packages
        .iter()
        .filter(|p| {
            p.get("targets")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|targets| {
                    targets.iter().any(|t| {
                        let lib = t
                            .get("kind")
                            .and_then(serde_json::Value::as_array)
                            .is_some_and(|kinds| {
                                kinds
                                    .iter()
                                    .filter_map(serde_json::Value::as_str)
                                    .any(|k| LIB_KINDS.contains(&k))
                            });
                        lib && t.get("doctest").and_then(serde_json::Value::as_bool) == Some(true)
                    })
                })
        })
        .filter_map(|p| p.get("name").and_then(serde_json::Value::as_str))
        .map(str::to_owned)
        .collect();
    Ok(match scope {
        Scope::Workspace => (!bearing.is_empty()).then_some(Scope::Workspace),
        Scope::Packages(names) => {
            let kept: Vec<String> = names
                .iter()
                .filter(|n| bearing.contains(n))
                .cloned()
                .collect();
            (!kept.is_empty()).then_some(Scope::Packages(kept))
        }
    })
}

/// **Rules 1 and 2 of the Rust `Affected` rules** (record T = ADR-0169 decision 7, proposed), before
/// anything is spawned: the row must be `Recorded` ([`registry::gate`]), and an empty change set
/// is refused by name. *(Added 2026-09-26, s8c build, P5.)*
///
/// # Errors
/// The row's refusal, or `Unsupported` naming the empty change set.
pub(crate) fn affected_gate(tc: &RustToolchain, changed: &[String]) -> Result<(), StepError> {
    registry::gate(tc.registry(), registry::Capability::AffectedSelection)?;
    if changed.is_empty() {
        return Err(StepError::Unsupported {
            capability: "affected-selection: an empty change set is refused by name; an affected \
                         run over nothing is a caller error, not a green and not a whole suite \
                         (record T decision 7 rule 2)"
                .to_owned(),
        });
    }
    Ok(())
}

/// **Rules 3 and 4**: `cargo metadata --format-version 1` (with the resolve graph), parsed, and the
/// pure [`affected::affected_packages`] over it. *(Added 2026-09-26, s8c build, P5.)*
///
/// # Errors
/// The metadata read's errors; `Transport { Decode }` naming a malformed metadata member.
pub(crate) fn affected_set(
    cfg: &Config,
    dir: &Path,
    changed: &[String],
) -> Result<AffectedSet, StepError> {
    let value = metadata(cfg, dir, "affected-metadata", false)?;
    let parsed =
        Metadata::parse(&value, Path::new(cfg.project_root().as_str())).map_err(|detail| {
            StepError::Transport {
                kind: TransportKind::Decode,
                detail: format!("the affected set's `cargo metadata` read: {detail}"),
            }
        })?;
    Ok(affected::affected_packages(
        changed,
        &parsed,
        AFFECTED_POLICY,
    ))
}

/// What one `test` call runs, and the report fields that follow from the selection.
struct Plan {
    scope: Scope,
    args: Vec<String>,
    basis: Basis,
    reason: Option<BasisReason>,
    /// `(recorded count, run id)` for an `Affected` run (rule 7).
    recorded_total: Option<(u32, String)>,
    /// The selected count when a rule already listed it (rule 6).
    selected: Option<u32>,
}

impl Plan {
    const fn whole(args: Vec<String>, reason: BasisReason) -> Self {
        Self {
            scope: Scope::Workspace,
            args,
            basis: Basis::WholeSuite,
            reason: Some(reason),
            recorded_total: None,
            selected: None,
        }
    }
}

/// The selection's plan (record T decision 7). `Affected` runs rules 3-7 here; rules 1 and 2 ran
/// before any spawn ([`affected_gate`]).
fn plan(
    cfg: &Config,
    dir: &Path,
    selection: &Selection,
    toolchain_version: &str,
) -> Result<Plan, StepError> {
    match selection {
        Selection::WholeSuite { reason } => Ok(Plan::whole(Vec::new(), *reason)),
        Selection::Filter(args) => Ok(Plan::whole(args.clone(), BasisReason::ProbeUnverified)),
        Selection::Affected { changed, recorded } => {
            let packages = match affected_set(cfg, dir, changed)? {
                // Rule 3: a path in no package forces the whole suite (record T's reading of
                // PolicyForced: a declared policy -- this rule, the adapter's own -- demanded it).
                AffectedSet::WholeSuite { .. } => {
                    return Ok(Plan::whole(Vec::new(), BasisReason::PolicyForced));
                }
                AffectedSet::Packages { packages, .. } => packages,
            };
            // Rule 5: no recorded whole-suite count, or one at another toolchain version.
            let Some(recorded) = recorded
                .as_ref()
                .filter(|r| r.toolchain_version == toolchain_version)
            else {
                return Ok(Plan::whole(Vec::new(), BasisReason::ProbeUnverified));
            };
            // Rule 6: a -p set that lists no test escalates (AC-332's declared policy).
            let scope = Scope::Packages(packages);
            let selected = counted(cfg, dir, "affected", &scope, &[])?;
            if selected == 0 {
                return Ok(Plan::whole(Vec::new(), BasisReason::SelectionEmpty));
            }
            // Rule 7.
            Ok(Plan {
                scope,
                args: Vec::new(),
                basis: Basis::Affected,
                reason: None,
                recorded_total: Some((recorded.count, recorded.run_id.clone())),
                selected: Some(selected),
            })
        }
        Selection::Named(name) => Err(named_refusal(name)),
    }
}

/// A `Named` selection: the Rust adapter declares none, so every name is refused by name,
/// `"e2e"` included (record T decision 7).
fn named_refusal(name: &str) -> StepError {
    StepError::Unsupported {
        capability: format!(
            "the named selection `{name}`: the Rust adapter declares no named selection, so \
             it is refused by name rather than given a meaning no record fixes (record T \
             decision 7)"
        ),
    }
}

/// One tallied child: its tally and its run.
struct Tallied {
    tally: Tally,
    run: Run,
}

/// Run the libtest (`cargo test`) child and parse its stdout.
fn libtest_child(
    cfg: &Config,
    dir: &Path,
    tag: &str,
    args: &[String],
    release: &str,
) -> Result<Tallied, StepError> {
    let run = run::run(cfg, dir, tag, args)?;
    let code = exited(&run, cfg.timeout().as_secs())?;
    let stdout = run.stdout_bytes()?;
    let tally = libtest_lines::parse(&String::from_utf8_lossy(&stdout), code, release)
        .map_err(|r| refused(&r, "libtest-lines", release, &run.argv))?;
    Ok(Tallied { tally, run })
}

/// Run the `cargo nextest run` child and parse its stderr (the status stream on `0.9.146`).
fn nextest_child(
    cfg: &Config,
    dir: &Path,
    args: &[String],
    version: &str,
) -> Result<Tallied, StepError> {
    let run = run::run(cfg, dir, "run", args)?;
    let code = exited(&run, cfg.timeout().as_secs())?;
    let stderr = run.stderr_bytes()?;
    let tally = nextest_lines::parse(&String::from_utf8_lossy(&stderr), code, version)
        .map_err(|r| refused(&r, "nextest-lines", version, &run.argv))?;
    Ok(Tallied { tally, run })
}

/// `Toolchain::test`'s body (the module doc): the outcome for one selection, every child under
/// `min(step, construction)` (record B = ADR-0171 decision 4, proposed).
///
/// # Errors
/// `Transport { Spawn }` when the program cannot start (AC-344) and `NonZero` when a `--list` pass
/// fails (the build failed before any test executed); `Timeout { after_secs }` (the bound that
/// fired) on a child killed at the bound and `Transport { Io }` on one the OS ended (AC-331);
/// `Transport { Io | Decode }` around the files, the version and every parser refusal;
/// `Unsupported` for an unpinned runner version, an `Affected` selection refused by its row or
/// over an empty change set, and a `Named` one. **Never `Err` for a red suite.**
pub fn verify_report(
    tc: &RustToolchain,
    selection: &Selection,
    bound: StepBound,
) -> Result<TestOutcome, StepError> {
    // Refusals that spawn nothing come first: a Named selection, and Affected rules 1 and 2.
    match selection {
        Selection::Named(name) => return Err(named_refusal(name)),
        Selection::Affected { changed, .. } => affected_gate(tc, changed)?,
        Selection::WholeSuite { .. } | Selection::Filter(_) => {}
    }
    let cfg = tc.config().bounded(bound);
    let dir = tc.run_dir()?;
    let toolchain_version = toolchain_version(&cfg, &dir)?;
    let release = rustc_release(&toolchain_version)?.to_owned();
    let plan = plan(&cfg, &dir, selection, &toolchain_version)?;
    let (scope, args) = (&plan.scope, &plan.args);
    let selected_test_count = match plan.selected {
        Some(n) => n,
        None => counted(&cfg, &dir, "selected", scope, args)?,
    };

    let (run_argv, wall, passed, failed, runner) = match tc.runner() {
        Runner::CargoTest => {
            let child = libtest_child(&cfg, &dir, "run", &run_args_in(scope, args), &release)?;
            let mut runner = TestRunner::named("cargo-test", &release);
            runner.foreign_lines = child.tally.foreign_lines;
            (
                child.run.argv,
                child.run.wall,
                child.tally.passed,
                child.tally.failed,
                runner,
            )
        }
        Runner::Nextest { version } => {
            let run = nextest_child(&cfg, &dir, &nextest_args_in(scope, args), version)?;
            let mut runner = TestRunner::named("cargo-nextest", version);
            let (mut passed, mut failed, mut wall) =
                (run.tally.passed, run.tally.failed, run.run.wall);
            if let Some(doc_scope) = doc_scope(&cfg, &dir, scope)? {
                let doc =
                    libtest_child(&cfg, &dir, "doc", &doc_args_in(&doc_scope, args), &release)?;
                passed = passed.saturating_add(doc.tally.passed);
                failed = failed.saturating_add(doc.tally.failed);
                wall += doc.run.wall;
                runner.foreign_lines = doc.tally.foreign_lines;
                runner.companion_argv = Some(doc.run.argv);
            }
            (run.run.argv, wall, passed, failed, runner)
        }
        Runner::NextestUnavailable { detail } => {
            return Err(StepError::Transport {
                kind: TransportKind::Spawn,
                detail: format!(
                    "the runner policy forces cargo-nextest, and the probe found none: {detail}"
                ),
            });
        }
    };

    // A whole-suite report's total IS the run's own executed count (the store's rule, ADR-0104
    // d3; the module doc), and no unfiltered listing is spawned. An Affected report's total is the
    // recorded whole-suite count, named by its run (rule 7; the store's affected rule).
    let (total_test_count, total_test_count_source) = match plan.recorded_total {
        Some((count, run_id)) => (count, TotalTestCountSource::RecordedRun { run_id }),
        None => (selected_test_count, TotalTestCountSource::OwnExecutedCount),
    };
    let report = VerifyReport {
        basis: plan.basis,
        argv: run_argv,
        selected_test_count,
        total_test_count,
        total_test_count_source,
        wall_secs: wall.as_secs(),
        toolchain_version,
        suite_sha: suite_sha(&cfg)?,
        basis_reason: plan.reason,
    };
    TestOutcome::new(report, passed, failed, runner)
}
