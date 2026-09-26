//! The `VerifyReport`, produced by an inherent method from structured sources only.
//!
//! The `VerifyReport` — `rk_types::verify::VerifyReport`, produced by an INHERENT method
//! (`NodeToolchain::verify_report`, ~~RT-6: the port stays six methods~~ *(struck 2026-09-26, s8c §8c.7: RT-6 is discharged and the port has seven methods under record V; the dated paragraph below)*) from **structured sources
//! only** (AC-329's adapter half, `s7c` §7c.14).
//!
//! **The count source is vitest's JSON reporter FILE.** The test argv is *"the full vector
//! `<npx.cmd|npx> vitest run --reporter=json --outputFile=<tmp> <file> [-t <pattern>]`, stated
//! once"* (§7c.14) — the reporter's two flags are on the vector **because the counts come from
//! its output file**: after the run, [`Counts`] is read from `<scratch>/run-<n>/vitest-report.json` by
//! path, through `serde_json` — `numPassedTests`, `numFailedTests`, `numPendingTests`,
//! `numTotalTests` [measured 2026-09-18 on the fixture: unfiltered `4 / 2 / 0 / 2`; `-t
//! filterable` `4 / 1 / 0 / 3`; `-t red_by_selection` exit 1, `4 / 0 / 1 / 3`; and on the
//! predecessor's `-t AC-015` run, `36 / 2 / 0 / 34`]. **Stdout is never read** — and under the
//! JSON reporter it carries no prose to read: vitest echoes no console output and prints no
//! summary, only `JSON report written to <f>` [measured 2026-09-18 on the fixture], so the
//! fixture's decoy `Tests  99 passed` reaches nothing and a stdout parser would find nothing (the
//! block's `rk_absent` polices the spellings over this crate's `src/`). A reporter file that is
//! absent or does not parse after the run is `Transport { kind: Decode }` naming it — never a
//! count from prose.
//!
//! - `selected_test_count` = `numPassedTests + numFailedTests`: what the run **executed** under
//!   its selection. Skip-marked tests and tests a `-t` pattern did not match are
//!   `numPendingTests` and did not execute — vitest marks the unmatched as skipped, so the
//!   reporter cannot tell a skip-mark from a filter, and neither is an executed test.
//! - `total_test_count` = the same executed count, and `total_test_count_source` =
//!   `OwnExecutedCount` — literally what the closed enum's variant says. **The whole-suite
//!   listing has no structured source that survives the predecessor**: `vitest list --json` over
//!   the whole predecessor tree fails to collect (`Failed to load url opentype.js … in
//!   docs/scripts/lockup-outliner.mjs`, `… sharp … gen-og-card.mjs` [measured 2026-09-18, 13 s,
//!   no file written]), so a selected run's total is the run's own and the whole-suite
//!   comparison (AC-326's `selection_ineffective`) is the kernel's, against the recorded
//!   whole-suite run — the residual is named in the slice, not smoothed here. This is also the
//!   store's rule: on a whole-suite report `rk_store::artifacts::verify_report::validate_report`
//!   (ADR-0104 decision 3) requires exactly `(executed, OwnExecutedCount)`, and the Rust crate
//!   reports the same pair for the same reason — one report shape across the two toolchains.
//! - pass / fail = **the run's exit status** (RT-22: `Err(NonZero { code, stderr_digest })` on a
//!   red suite, the digest over the retained stderr bytes; `Ok(report)` only on a green one).
//!   The reporter file's `success` is read and must agree — a green exit over a `success: false`
//!   file, or the reverse, is a `Transport { Decode }` naming the disagreement, never a verdict.
//!
//! **What a killed run is** (AC-331; ADR-0127's forbids): `run_bounded`'s `TimedOut` becomes
//! `StepError::Timeout { after_secs }` — the construction-time bound's own value, since the
//! toolchain has no token and `Cancelled { by }` is keyed on one (AC-225) — and `OsKilled`
//! becomes `Transport { kind: Io }` whose `detail` starts with [`OS_KILLED_DETAIL_PREFIX`]
//! ([`os_killed`]: the shape 7d keys on, since the same variant and kind also carry an unreadable
//! output file — the variant is RT-3's OWED record's); **neither is ever `NonZero` and neither
//! is ever `Ok`** — the input 7d's verify runner maps to `Unverifiable { cannot_run }`. A program that
//! cannot start is `Transport { kind: Spawn }` (AC-344's adapter share, [`crate::run`]); so is a
//! project with no locally installed `vitest` ([`crate::env::local_bin`]: `npx` would fetch or
//! prompt, and this adapter never lets it).
//!
//! **The other members.** `argv` is the RUN vector as handed to the OS (AC-325) — the program as
//! the `Config` names it (`npx.cmd` / `npx` / the wrapper a test pointed it at), then the vector
//! above, the selection's elements appended verbatim (DECISIONS-2026-09-09 #20: the file and the
//! `-t` pattern are separate elements; #34: never `pnpm test --`). `toolchain_version` is
//! `<program> vitest --version`'s answer (`vitest/2.1.9 win32-x64 node-v24.18.0` here) — never
//! a hard-coded pin. `suite_sha` is a sha256 over the suite's SOURCES — `package.json`, the
//! lockfile, `vitest.config.*`, `tsconfig.json` and every `*.test.*` / `*.spec.*` file under the
//! project root, `node_modules/`, `.git/`, `dist/`, `build/` and `target/` excluded, as
//! length-prefixed `(relative path, bytes)` pairs in path order. `wall_secs` is the run's own
//! wall time (the two `Instant::now` sites in [`crate::run`]). `basis` is **always `WholeSuite`**
//! with `basis_reason: ProbeUnverified` — AC-328: no `affected-selection` probe is recorded at
//! any pin (S-SEL-NODE is owed), so no `basis: affected` record can be produced by this adapter.
//!
//! *(Amended 2026-09-26, s8c §8c.2; record T = ADR-0169, proposed. Read this paragraph as the
//! current contract; the text above stays as the record of what was.)* The INHERENT
//! `NodeToolchain::verify_report` is RETIRED (RT-6 discharged): [`verify_report`] here is
//! `Toolchain::test`'s body, and returns a `rk_ports::toolchain::TestOutcome` -- the unchanged
//! report plus `passed` / `failed` from the SAME reporter file (`numPassedTests`,
//! `numFailedTests`), the derived `status`, and `runner` = `vitest` at `vitest --version`'s
//! answer. *"RT-6: the port stays six methods"* is false twice over: RT-6 is discharged, and
//! under record V = ADR-0170 the port has SEVEN methods (Checkpoint 7a counts none; E-0162-2).
//! **RT-22 is REVERSED: a red suite is `Ok`** with `failed ≥ 1`. A non-zero exit whose reporter
//! file names NO failed test is still `Err(NonZero)`: nothing the file can attribute failed, so
//! the suite could not run (a collection or import error), record T decision 1's class (a). The
//! exit and the file's `success` must still agree, and a zero exit whose file names a failure
//! is `Transport { Decode }` too. The selection is the closed `Selection`: `WholeSuite` and
//! `Filter` run as before (the reason travels with the first), `Affected` is refused from the
//! `Probe::None` registry row (AC-328), and a `Named` selection by name.

use std::path::{Path, PathBuf};

use rk_containment::Outcome;
use rk_ports::toolchain::{Selection, StepBound, TestOutcome, TestRunner};
use rk_types::digest::{Sha256Stream, sha256};
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

use crate::run::{self, Run};
use crate::{Config, NodeToolchain, registry};

/// The reporter file's name under the call's run directory.
///
/// The reporter file's name under the call's run directory (`<scratch>/run-<n>/`): one per
/// `verify_report` call, in a directory that call alone writes — never one file per adapter
/// rewritten per run, because two concurrent calls on this `Send + Sync` adapter would then
/// overwrite each other's before either read its counts.
pub const REPORTER_FILE: &str = "vitest-report.json";

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

/// What the JSON reporter file carries that this module reads — the four counts and the
/// verdict flag; every other member of the file is ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub struct Counts {
    #[serde(rename = "numTotalTests")]
    pub total: u32,
    #[serde(rename = "numPassedTests")]
    pub passed: u32,
    #[serde(rename = "numFailedTests")]
    pub failed: u32,
    #[serde(rename = "numPendingTests")]
    pub pending: u32,
    pub success: bool,
}

impl Counts {
    /// What the run executed: passed plus failed.
    #[must_use]
    pub const fn executed(self) -> u32 {
        self.passed.saturating_add(self.failed)
    }
}

/// Read the reporter file (the module doc).
///
/// # Errors
/// `Transport { Decode }` naming the file when it is absent or does not parse.
pub fn read_counts(path: &Path) -> Result<Counts, StepError> {
    let text = std::fs::read_to_string(path).map_err(|e| StepError::Transport {
        kind: TransportKind::Decode,
        detail: format!(
            "the vitest JSON reporter file {} is not readable after the run: {e} — the counts \
             have no structured source and no prose is read (AC-329)",
            path.display()
        ),
    })?;
    serde_json::from_str(&text).map_err(|e| StepError::Transport {
        kind: TransportKind::Decode,
        detail: format!(
            "the vitest JSON reporter file {} does not parse: {e}",
            path.display()
        ),
    })
}

/// The run vector's arguments (after the program): `vitest run --reporter=json
/// --outputFile=<file>`, then the selection's elements verbatim.
#[must_use]
pub fn run_args(reporter: &Path, selection: &[String]) -> Vec<String> {
    let mut args = vec![
        "vitest".to_owned(),
        "run".to_owned(),
        "--reporter=json".to_owned(),
        format!("--outputFile={}", reporter.display()),
    ];
    args.extend(selection.iter().cloned());
    args
}

/// A green run, or the typed error a non-green outcome is.
/// How the run ended: green, a red suite, or killed — the three the caller keeps apart.
enum Verdict {
    Green,
    Red(StepError),
    Killed(StepError),
}

/// The run's fate as a value, the errors typed.
fn verdict(run: &Run, timeout_secs: u64) -> Result<Verdict, StepError> {
    Ok(match run.outcome {
        Outcome::Exited(status) if status.success() => Verdict::Green,
        Outcome::Exited(status) => Verdict::Red(StepError::NonZero {
            code: status.code(),
            stderr_digest: sha256(&run.stderr_bytes()?),
        }),
        // AC-331: a run killed at the bound or by the OS is never a status (ADR-0127's forbids).
        Outcome::TimedOut => Verdict::Killed(StepError::Timeout {
            after_secs: timeout_secs,
        }),
        Outcome::OsKilled => Verdict::Killed(os_killed(&run.argv)),
    })
}

/// A green run, or the typed error a non-green outcome is — the version read's shape.
fn expect_green(run: &Run, timeout_secs: u64) -> Result<(), StepError> {
    match verdict(run, timeout_secs)? {
        Verdict::Green => Ok(()),
        Verdict::Red(e) | Verdict::Killed(e) => Err(e),
    }
}

/// `<program> vitest --version`, read from the run's stdout file.
fn toolchain_version(tc: &NodeToolchain, cfg: &Config, dir: &Path) -> Result<String, StepError> {
    tc.require_local_bin("vitest")?;
    let run = run::run(
        cfg,
        dir,
        "vitest-version",
        &["vitest".to_owned(), "--version".to_owned()],
    )?;
    expect_green(&run, cfg.timeout().as_secs())?;
    let version = String::from_utf8_lossy(&run.stdout_bytes()?)
        .trim()
        .to_owned();
    if version.is_empty() {
        return Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail:
                "`vitest --version` printed nothing; the report's toolchain_version has no source"
                    .to_owned(),
        });
    }
    Ok(version)
}

/// sha256 over the suite's sources (the module doc). `pub` so the digest's own test can drive
/// it over a temp tree without a vitest run.
///
/// # Errors
/// `Transport { Io }` when a source cannot be listed or read.
pub fn suite_sha(root: &Path) -> Result<String, StepError> {
    let mut files: Vec<PathBuf> = Vec::new();
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

fn is_suite_source(name: &str) -> bool {
    const FIXED: [&str; 5] = [
        "package.json",
        "pnpm-lock.yaml",
        "package-lock.json",
        "yarn.lock",
        "tsconfig.json",
    ];
    // Vite compiles the config to a transient `vitest.config.ts.timestamp-<ms>-<hash>.mjs` beside
    // it for the life of one run and deletes it after: not a source, and a file that can vanish
    // between the listing and the read [measured 2026-09-18 under the crate's parallel tests].
    if name.contains(".timestamp-") {
        return false;
    }
    FIXED.contains(&name)
        || name.starts_with("vitest.config.")
        || name.starts_with("vite.config.")
        || [".test.", ".spec."].iter().any(|m| name.contains(m))
}

fn collect_sources(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), StepError> {
    let listing = |e: std::io::Error| StepError::Transport {
        kind: TransportKind::Io,
        detail: format!("could not list {} for the suite digest: {e}", dir.display()),
    };
    for entry in std::fs::read_dir(dir).map_err(listing)? {
        let entry = entry.map_err(listing)?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if matches!(
                name.as_str(),
                "node_modules" | ".git" | "dist" | "build" | "target"
            ) || name.starts_with(".s-cancel-win-")
            {
                continue;
            }
            collect_sources(&path, out)?;
        } else if is_suite_source(&name) {
            out.push(path);
        }
    }
    Ok(())
}

/// The selection's vitest arguments and the basis reason it records (record T decision 7).
///
/// # Errors
/// `Affected` refused from the `affected-selection` registry row (AC-328; its `Probe::None` row);
/// `Named` refused by name.
fn arguments(
    tc: &NodeToolchain,
    selection: &Selection,
) -> Result<(Vec<String>, BasisReason), StepError> {
    match selection {
        Selection::WholeSuite { reason } => Ok((Vec::new(), *reason)),
        Selection::Filter(args) => Ok((args.clone(), BasisReason::ProbeUnverified)),
        Selection::Affected { .. } => Err(registry::refusal(
            tc.registry(),
            registry::Capability::AffectedSelection,
        )),
        Selection::Named(name) => Err(StepError::Unsupported {
            capability: format!(
                "the named selection `{name}`: the Node adapter declares no named selection, so it \
                 is refused by name (record T decision 7)"
            ),
        }),
    }
}

/// `Toolchain::test`'s body (the module doc): the outcome for one selection.
///
/// # Errors
/// `Timeout { after_secs }` on a run killed at the bound and `Transport { Io }` on one the OS
/// ended (AC-331); `Transport { Spawn }` when the program cannot start or `vitest` is not installed
/// locally (AC-344); `NonZero` when the run exited non-zero and the reporter file names no failed
/// test (it could not run); `Transport { Decode }` when the reporter file is absent, unparseable or
/// disagrees with the exit status; `Unsupported` for an `Affected` or `Named` selection. **Never
/// `Err` for a red suite.**
///
/// *(Amended 2026-09-26, s8c build, P5: every child runs under `min(step, construction)`, record B
/// = ADR-0171 decision 4, proposed, and a `Timeout` names the bound that fired.)*
pub fn verify_report(
    tc: &NodeToolchain,
    selection: &Selection,
    bound: StepBound,
) -> Result<TestOutcome, StepError> {
    let (args, reason) = arguments(tc, selection)?;
    tc.require_local_bin("vitest")?;
    let cfg = tc.config().bounded(bound);
    let dir = tc.run_dir()?;
    let toolchain_version = toolchain_version(tc, &cfg, &dir)?;
    let root = Path::new(tc.config().project_root().as_str());

    // The reporter file lives in this call's fresh run directory, so no stale file from a
    // previous run — this adapter's or a concurrent call's — can be read as this run's.
    let reporter = dir.join(REPORTER_FILE);
    let run_argv = run_args(&reporter, &args);
    let run = run::run(&cfg, &dir, "run", &run_argv)?;
    // A killed run has no report to read — the reporter file was never written — and is
    // returned before any structured source is consulted (AC-331).
    let verdict = match verdict(&run, cfg.timeout().as_secs())? {
        Verdict::Killed(e) => return Err(e),
        other => other,
    };
    let counts = read_counts(&reporter)?;
    match (&verdict, counts.success) {
        // A killed run returned above; the arm exists for exhaustiveness and nothing reaches it.
        (Verdict::Green, true) | (Verdict::Red(_), false) | (Verdict::Killed(_), _) => {}
        (Verdict::Green, false) => {
            return Err(StepError::Transport {
                kind: TransportKind::Decode,
                detail: format!(
                    "the run exited 0 but the reporter file {} says success: false — no verdict \
                     is read from a disagreement",
                    reporter.display()
                ),
            });
        }
        (Verdict::Red(_), true) => {
            return Err(StepError::Transport {
                kind: TransportKind::Decode,
                detail: format!(
                    "the run exited non-zero but the reporter file {} says success: true — no \
                     verdict is read from a disagreement",
                    reporter.display()
                ),
            });
        }
    }
    match verdict {
        // A non-zero exit whose file names no failed test: nothing ran that could fail, so the
        // suite could not run (record T decision 1, class (a)) -- never a red suite, never green.
        Verdict::Red(e) if counts.failed == 0 => return Err(e),
        Verdict::Green if counts.failed > 0 => {
            return Err(StepError::Transport {
                kind: TransportKind::Decode,
                detail: format!(
                    "the run exited 0 but the reporter file {} names {} failed tests — no \
                     verdict is read from a disagreement",
                    reporter.display(),
                    counts.failed
                ),
            });
        }
        Verdict::Green | Verdict::Red(_) | Verdict::Killed(_) => {}
    }
    let executed = counts.executed();

    let report = VerifyReport {
        basis: Basis::WholeSuite,
        argv: run.argv,
        selected_test_count: executed,
        total_test_count: executed,
        total_test_count_source: TotalTestCountSource::OwnExecutedCount,
        wall_secs: run.wall.as_secs(),
        toolchain_version: toolchain_version.clone(),
        suite_sha: suite_sha(root)?,
        basis_reason: Some(reason),
    };
    TestOutcome::new(
        report,
        counts.passed,
        counts.failed,
        TestRunner::named("vitest", &toolchain_version),
    )
}
