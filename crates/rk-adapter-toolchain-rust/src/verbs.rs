//! The Rust verb handlers behind `run_verb` (s8c §8c.4; record V = ADR-0170 decisions 2 and 4-7,
//! proposed; D21-62 C, provisional).
//!
//! [`run_verb`] routes four verbs and refuses every other one by its **bare** name
//! (`Unsupported { capability: "<verb>" }`, ADR-0135 clause 2's shape): `item_create` and
//! `release_tag` (the registry's `store` and `git` rows, which the kernel refuses naming the route
//! before any port call), the verbs with no row, and `mutate` (`run_verb` is not a patch applier;
//! ADR-0162's refusal of `ReproKind::Mutate` stands). Each handler answers an ERASED
//! `VerbOutput { kind, value }` whose `kind` is the registry row's `outputs`; the kernel types it at
//! the boundary. Every child a handler spawns runs under `min(step, construction)` (record B =
//! ADR-0171 decision 4), and a `Timeout` names the bound that fired.
//!
//! **`smoke`** (decision 4). `cargo build --workspace --message-format=json`, then each checklist
//! row against the executable THAT build produced (named by its target, read from the stream's
//! `compiler-artifact` messages), never an installed binary. The rows run in ONE directory per
//! call, `<scratch>/smoke-<pid>-<n>`, where `<scratch>` is the adapter's scratch: in production
//! ADR-0137 clause 6's `<git-common-dir>/redkiln/tmp`, outside every `.redkiln/worktrees/<item>`
//! and outside the checkout. Each row's `cwd = "scratch"` resolves to it (no other `cwd` is
//! admitted), and it is removed on every path; a removal that fails is `Err` naming the path. The
//! handler runs no git. **The item form this handler reads**: `{ id, argv, cwd, expect_exit,
//! stdout_contains }`, the one stdout predicate being *the row's stdout contains this text*
//! (`s8g` §8g.6 authors the items; record V fixes that form, and the predicate's spelling is this
//! handler's, stated here). A row that fails is `Ok` with `failed >= 1`; `Err` means the build did
//! not compile or could not run, a row's program could not start (a binary the build did not
//! produce included), or a row was killed.
//!
//! **`sca_scan`** (decision 5; D21-62 C). Both tools run offline against an EXPLICIT database:
//! `cargo audit --json --no-fetch --db <audit_db> --file <root>/Cargo.lock`, and `cargo deny
//! --offline -L debug --format json --config <generated> check advisories --audit-compatible-output`
//! under a config whose `[advisories] db-path` is the `deny_db` input, which must hold exactly one
//! child directory. The adapter reads each database's `HEAD` itself, through the one shared
//! `rk_git::head::head_commit` read, before its tool runs and after it exits, and refuses by name,
//! recording no `ScanReport`, on: (a) `database-not-a-checkout`; (b) `database-moved-mid-scan`;
//! (c) `database-member-disagrees` (audit's non-null `database.last-commit` against the read; it is
//! cross-checked, never allowed to overwrite it); (d) `deny-db-path-children`; (e)
//! `deny-db-path-line` (deny's pinned DEBUG line *"Opening advisory database at '<path>'"* missing or
//! naming another path; it pins the PATH only, never the commit). **Refusal (b) is asserted over
//! the comparator [`db_unmoved`] only**: the adapter carries no seam that moves `HEAD` between its
//! two reads, so the end-to-end case is not claimed. Advisories are the tools' `vulnerabilities`,
//! merged by `(id, package, version)`; the tools' informational `warnings` (unmaintained, yanked)
//! are not advisories with a severity and are not counted here. **Severity**: a v3.x vector's CVSS
//! v3 base score is computed in-tree by [`cvss3_tenths`] (the CVSS v3.1 specification's base
//! equations and `Roundup`), pinned against `cvss::v3` by a differential test; a `CVSS:4.0` vector
//! is scored by the RustSec `cvss` crate's v4 scorer (`cvss::v4`, pinned `=2.2.0`: D21-64 ruling 7,
//! which moves D21-63 from option (1) to (3'); 63 of the RustSec database's vectors are v4.0 at
//! stage open). Both are banded by [`severity_of`] on record V's one set of thresholds (FIRST CVSS
//! v4.0 §6 uses the v3 qualitative scale): `critical` >= 9.0, `high` 7.0-8.9, `medium` 4.0-6.9,
//! `low` 0.0-3.9. **An advisory with no CVSS is `unrated`**, never `low`, never dropped. A vector
//! neither scorer accepts (malformed, or a version other than 3.0, 3.1 and 4.0) refuses the scan
//! by name, `cvss-unscorable`, rather than be counted as unrated or scored as something else:
//! fail-closed, naming the vector and its remedies — record V decision 5.
//!
//! **`package`** (decision 6). The input `sha` must equal the project root's `HEAD`, read through
//! the same shared `rk_git::head` read (Checkpoint 8a guard (6)'s allowlist admits it by a dated
//! amendment in `s8a`). Then `cargo build --release --locked --message-format=json`, and the plain
//! `rk_types::digest::sha256` of each DECLARED binary (`binaries`, a non-empty list of target
//! names), never a glob. Refused by name: `undeclared-binary-set`, `sha-head-mismatch`,
//! `head-unreadable`, `binary-not-built`. Each binary's path is recorded relative to the target
//! directory, and a binary outside it is refused, `package-binary-outside-target`, never recorded
//! as an absolute path.
//!
//! **`setup`** (decision 7). For each declared project (`projects`, paths relative to the project
//! root), in order, `pnpm install --frozen-lockfile --prefer-offline --dir <project>` in the project
//! root, with `pnpm` from [`crate::Config::tool_program`]. A non-zero install is refused naming
//! `setup`, the project and the exit code, never `Ok` with a failed row; an empty set is `Ok` with
//! no projects and spawns nothing.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_containment::Outcome;
use rk_ports::toolchain::{VerbCall, VerbOutput};
use rk_types::digest::sha256;
use rk_types::error::{StepError, TransportKind};
use rk_types::verb_output::{
    Advisory, ArtifactBinary, BuildArtifact, BuildStatus, ScanReport, ScanTool, SetupProject,
    SetupResult, Severity, SmokeResult, SmokeRow, SmokeStatus,
};
use serde::Deserialize;

use crate::report::os_killed;
use crate::run::{self, Run};
use crate::{Config, DEFAULT_PNPM, RustToolchain, build};

/// The build a `smoke` call makes before its rows.
const SMOKE_BUILD: [&str; 3] = ["build", "--workspace", "--message-format=json"];

/// Record V decision 6's locked release build.
const PACKAGE_BUILD: [&str; 4] = ["build", "--release", "--locked", "--message-format=json"];

/// The `setup` install vector's arguments before the project.
const SETUP_ARGS: [&str; 4] = ["install", "--frozen-lockfile", "--prefer-offline", "--dir"];

/// Route one call (the module doc).
///
/// # Errors
/// `Unsupported { capability: "<verb>" }` for every verb but the four; each handler's own errors.
pub fn run_verb(tc: &RustToolchain, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
    let cfg = tc.config().bounded(call.bound);
    match call.verb {
        "smoke" => smoke(tc, &cfg, call.inputs),
        "sca_scan" => sca_scan(tc, &cfg, call.inputs),
        "package" => package(tc, &cfg, call.inputs),
        "setup" => setup(tc, &cfg, call.inputs),
        other => Err(StepError::Unsupported {
            capability: other.to_owned(),
        }),
    }
}

/// A refusal by name: `Transport { Decode }` whose detail leads with the verb and the refusal.
fn refuse(verb: &str, name: &str, detail: &str) -> StepError {
    StepError::Transport {
        kind: TransportKind::Decode,
        detail: format!("{verb} refused ({name}): {detail}"),
    }
}

fn output<T: serde::Serialize>(kind: &str, value: &T) -> Result<VerbOutput, StepError> {
    Ok(VerbOutput {
        kind: kind.to_owned(),
        value: serde_json::to_value(value).map_err(|e| StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!("the {kind} output did not serialise: {e}"),
        })?,
    })
}

/// A required input, decoded; a missing or malformed one is refused naming it.
fn input<T: serde::de::DeserializeOwned>(
    inputs: &BTreeMap<String, serde_json::Value>,
    verb: &str,
    name: &str,
) -> Result<T, StepError> {
    let value = inputs.get(name).ok_or_else(|| {
        refuse(
            verb,
            "input-missing",
            &format!("the call carries no `{name}` input"),
        )
    })?;
    serde_json::from_value(value.clone())
        .map_err(|e| refuse(verb, "input-malformed", &format!("the `{name}` input: {e}")))
}

/// A run's exit code, or the error a killed run is (the bound that fired, or the OS kill).
fn exit_code(run: &Run, cfg: &Config) -> Result<i32, StepError> {
    match run.outcome {
        Outcome::Exited(status) => status.code().ok_or_else(|| StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!("{:?} exited with no exit code (a signal)", run.argv),
        }),
        Outcome::TimedOut => Err(StepError::Timeout {
            after_secs: cfg.timeout().as_secs(),
        }),
        Outcome::OsKilled => Err(os_killed(&run.argv)),
    }
}

/// The build a handler needs, which must compile: a failing build is `NonZero`, cannot-run, with
/// the digest of the report's JSON (a build that did not compile runs no row and packages nothing).
fn compiled(cfg: &Config, dir: &Path, tag: &str, args: &[&str]) -> Result<String, StepError> {
    let (report, stdout) = RustToolchain::json_build(cfg, dir, tag, args)?;
    if report.status == BuildStatus::Failing {
        return Err(StepError::NonZero {
            code: None,
            stderr_digest: sha256(&serde_json::to_vec(&report).unwrap_or_default()),
        });
    }
    Ok(stdout)
}

const fn io(detail: String) -> StepError {
    StepError::Transport {
        kind: TransportKind::Io,
        detail,
    }
}

// ------------------------------------------------------------------------------------ smoke

/// One checklist item, the form this handler reads (the module doc).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SmokeItem {
    id: String,
    argv: Vec<String>,
    cwd: String,
    expect_exit: i32,
    stdout_contains: String,
}

fn smoke(
    tc: &RustToolchain,
    cfg: &Config,
    inputs: &BTreeMap<String, serde_json::Value>,
) -> Result<VerbOutput, StepError> {
    let items: Vec<SmokeItem> = input(inputs, "smoke", "checklist")?;
    for item in &items {
        if item.argv.is_empty() {
            return Err(refuse(
                "smoke",
                "input-malformed",
                &format!("the item `{}` has an empty argv", item.id),
            ));
        }
        if item.cwd != "scratch" {
            return Err(refuse(
                "smoke",
                "input-malformed",
                &format!(
                    "the item `{}` names cwd `{}`; only `scratch` is admitted (record V decision 4)",
                    item.id, item.cwd
                ),
            ));
        }
    }
    let dir = tc.run_dir()?;
    let stream = compiled(cfg, &dir, "smoke-build", &SMOKE_BUILD)?;
    let exes = build::executables(&stream).map_err(|d| refuse("smoke", "build-stream", &d))?;
    let scratch = smoke_dir(tc, cfg)?;
    let ran = smoke_rows(cfg, &dir, &scratch, &exes, &items);
    match (ran, std::fs::remove_dir_all(&scratch)) {
        (ran, Ok(())) => ran,
        (Ok(_), Err(e)) => Err(io(format!(
            "smoke could not remove its scratch directory {}: {e} (a removal that fails is never a silent leak)",
            scratch.display()
        ))),
        (Err(first), Err(e)) => Err(io(format!(
            "smoke could not remove its scratch directory {}: {e}; the call had already failed: {first:?}",
            scratch.display()
        ))),
    }
}

/// One empty directory for this call: `<scratch>/smoke-<pid>-<n>`, created fresh (never reused).
fn smoke_dir(tc: &RustToolchain, cfg: &Config) -> Result<PathBuf, StepError> {
    for _ in 0..64 {
        let path = cfg
            .scratch()
            .join(format!("smoke-{}-{}", std::process::id(), tc.unique()));
        match std::fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => {
                return Err(io(format!(
                    "smoke could not create its scratch directory {}: {e}",
                    path.display()
                )));
            }
        }
    }
    Err(io(format!(
        "smoke found no free smoke-<unique> name under {}",
        cfg.scratch().display()
    )))
}

fn smoke_rows(
    cfg: &Config,
    dir: &Path,
    scratch: &Path,
    exes: &[(String, PathBuf)],
    items: &[SmokeItem],
) -> Result<VerbOutput, StepError> {
    let mut rows = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        let program = exes
            .iter()
            .find(|(name, _)| *name == item.argv[0])
            .map(|(_, path)| path.clone())
            .ok_or_else(|| StepError::Transport {
                kind: TransportKind::Spawn,
                detail: format!(
                    "smoke: the item `{}` runs `{}`, which is not a binary the call's build produced ({:?}); an installed binary is never run",
                    item.id,
                    item.argv[0],
                    exes.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>()
                ),
            })?;
        let run = run::run_in(
            cfg,
            dir,
            &format!("smoke-row-{i}"),
            &program,
            &item.argv[1..],
            scratch,
        )?;
        let exit = exit_code(&run, cfg)?;
        let stdout = String::from_utf8_lossy(&run.stdout_bytes()?).into_owned();
        let predicate_held = stdout.contains(&item.stdout_contains);
        rows.push(SmokeRow {
            id: item.id.clone(),
            argv: run.argv,
            cwd: scratch.display().to_string(),
            expect_exit: item.expect_exit,
            exit,
            predicate: format!("stdout contains {:?}", item.stdout_contains),
            predicate_held,
            passed: exit == item.expect_exit && predicate_held,
        });
    }
    let passed = u32::try_from(rows.iter().filter(|r| r.passed).count()).unwrap_or(u32::MAX);
    let failed = u32::try_from(rows.len())
        .unwrap_or(u32::MAX)
        .saturating_sub(passed);
    let status = if failed > 0 {
        SmokeStatus::Failing
    } else if passed > 0 {
        SmokeStatus::Passing
    } else {
        SmokeStatus::Empty
    };
    output(
        "SmokeResult",
        &SmokeResult {
            rows,
            passed,
            failed,
            status,
        },
    )
}

// ------------------------------------------------------------------------------------ sca_scan

/// Refusal (b)'s comparator: the database a tool read did not move while it ran (D21-62 C).
///
/// # Errors
/// `database-moved-mid-scan` naming the tool, the database and both commits.
pub fn db_unmoved(tool: &str, db: &Path, before: &str, after: &str) -> Result<(), StepError> {
    if before == after {
        return Ok(());
    }
    Err(refuse(
        "sca_scan",
        "database-moved-mid-scan",
        &format!(
            "{tool}'s database {} read HEAD {before} before the scan and {after} after it: a scan \
             whose database moved is two instruments",
            db.display()
        ),
    ))
}

/// A database's `HEAD` through the one shared `rk_git` read (refusal (a)).
fn db_head(tool: &str, db: &Path) -> Result<String, StepError> {
    rk_git::head::head_commit(db).map_err(|e| {
        refuse(
            "sca_scan",
            "database-not-a-checkout",
            &format!("{tool}'s database {}: {e}", db.display()),
        )
    })
}

/// The CVSS v3 base score in tenths.
///
/// The CVSS v3.1 specification's base equations and its `Roundup`, which v3.0 vectors are scored
/// by too. Only the eight base metrics are read; any other metric (temporal, environmental) is
/// ignored, and a base metric missing, repeated or out of its set is refused.
///
/// `pub` so the differential test (`verbs::the_in_tree_v3_scorer_agrees_with_cvss_v3_over_the_database_sample`)
/// can pin it against `cvss::v3` over real advisory vectors (D21-64 ruling 7).
///
/// # Errors
/// A description of why the vector is not a scorable CVSS v3.0 or v3.1 base vector.
pub fn cvss3_tenths(vector: &str) -> Result<u32, String> {
    let rest = vector
        .strip_prefix("CVSS:3.1/")
        .or_else(|| vector.strip_prefix("CVSS:3.0/"))
        .ok_or("not a CVSS v3.0 or v3.1 vector; this scorer scores v3 base vectors only")?;
    let mut m: BTreeMap<&str, &str> = BTreeMap::new();
    for part in rest.split('/') {
        let (k, v) = part
            .split_once(':')
            .ok_or_else(|| format!("the metric `{part}` has no value"))?;
        if ["AV", "AC", "PR", "UI", "S", "C", "I", "A"].contains(&k) && m.insert(k, v).is_some() {
            return Err(format!("the base metric `{k}` is repeated"));
        }
    }
    let get = |k: &str| {
        m.get(k)
            .copied()
            .ok_or_else(|| format!("the base metric `{k}` is missing"))
    };
    let bad = |k: &str, v: &str| format!("`{k}:{v}` is not a CVSS v3 value");
    let changed = match get("S")? {
        "U" => false,
        "C" => true,
        v => return Err(bad("S", v)),
    };
    let av = match get("AV")? {
        "N" => 0.85,
        "A" => 0.62,
        "L" => 0.55,
        "P" => 0.2,
        v => return Err(bad("AV", v)),
    };
    let ac = match get("AC")? {
        "L" => 0.77,
        "H" => 0.44,
        v => return Err(bad("AC", v)),
    };
    let pr = match (get("PR")?, changed) {
        ("N", _) => 0.85,
        ("L", false) => 0.62,
        ("L", true) => 0.68,
        ("H", false) => 0.27,
        ("H", true) => 0.5,
        (v, _) => return Err(bad("PR", v)),
    };
    let ui = match get("UI")? {
        "N" => 0.85,
        "R" => 0.62,
        v => return Err(bad("UI", v)),
    };
    let cia = |k: &str| -> Result<f64, String> {
        match get(k)? {
            "H" => Ok(0.56),
            "L" => Ok(0.22),
            "N" => Ok(0.0),
            v => Err(bad(k, v)),
        }
    };
    let iss = 1.0 - (1.0 - cia("C")?) * (1.0 - cia("I")?) * (1.0 - cia("A")?);
    let impact = if changed {
        // The specification's `7.52 × (ISS − 0.029) − 3.25 × (ISS − 0.02)^15`.
        7.52f64.mul_add(iss - 0.029, -3.25 * (iss - 0.02).powi(15))
    } else {
        6.42 * iss
    };
    let exploitability = 8.22 * av * ac * pr * ui;
    if impact <= 0.0 {
        return Ok(0);
    }
    let raw = if changed {
        (1.08 * (impact + exploitability)).min(10.0)
    } else {
        (impact + exploitability).min(10.0)
    };
    // CVSS v3.1 Roundup: the smallest one-decimal number >= the input, computed on an integer so
    // floating-point noise cannot move a band.
    let int_input = (raw * 100_000.0).round() as i64;
    let tenths = if int_input % 10_000 == 0 {
        int_input / 10_000
    } else {
        int_input / 10_000 + 1
    };
    u32::try_from(tenths).map_err(|_| format!("the score {tenths} is out of range"))
}

/// A `CVSS:4.0` vector's score in tenths, from the RustSec `cvss` crate's v4 scorer
/// (`cvss::v4::Vector::from_str(..).score().value()`; D21-64 ruling 7, D21-63 option (3')). The
/// crate scores the vector as written: a vector that carries threat metrics (`E:`) is scored as
/// CVSS-BT, as FIRST CVSS v4.0 §2 names it, which is how CVE-2025-47273's vector reaches 7.7.
fn cvss4_tenths(vector: &str) -> Result<u32, String> {
    let parsed: cvss::v4::Vector = vector
        .parse()
        .map_err(|e: cvss::Error| format!("the `cvss` crate's v4 scorer rejects it: {e}"))?;
    let tenths = (parsed.score().value() * 10.0).round();
    if !(0.0..=100.0).contains(&tenths) {
        return Err(format!("the score {tenths} tenths is out of range"));
    }
    // The range check above makes the cast exact: a one-decimal score in [0.0, 10.0].
    Ok(tenths as u32)
}

/// An advisory's band, or `None` (unrated) when it carries no CVSS (record V decision 5).
///
/// A `CVSS:4.0` vector is scored by the `cvss` crate's v4 scorer and every other vector by the
/// in-tree v3 scorer; both are banded on the same thresholds, a score of 0.0 counting `low`.
///
/// # Errors
/// `cvss-unscorable` naming a vector neither scorer accepts (malformed, or an unknown version),
/// and its remedies, so the halt is one operator decision away (record V decision 5).
pub fn severity_of(vector: Option<&str>) -> Result<Option<Severity>, StepError> {
    let Some(vector) = vector else {
        return Ok(None);
    };
    let scored = if vector.starts_with("CVSS:4.0/") {
        cvss4_tenths(vector)
    } else {
        cvss3_tenths(vector)
    };
    let tenths = scored.map_err(|d| {
        refuse(
            "sca_scan",
            "cvss-unscorable",
            &format!(
                "`{vector}`: {d}; neither the in-tree CVSS v3 scorer nor the RustSec `cvss` crate's v4 scorer can score it (record V decision 5): correct the advisory's vector, or rule it `unrated` so sca-scan routes to scan.accept-findings"
            ),
        )
    })?;
    Ok(Some(match tenths {
        90.. => Severity::Critical,
        70..=89 => Severity::High,
        40..=69 => Severity::Medium,
        _ => Severity::Low,
    }))
}

/// A tool's version: the last word of `cargo <sub> --version` (`cargo-audit-audit 0.22.2`,
/// `cargo-deny 0.20.2`).
fn tool_version(cfg: &Config, dir: &Path, sub: &str) -> Result<String, StepError> {
    let run = run::run(
        cfg,
        dir,
        &format!("{sub}-version"),
        &[sub.to_owned(), "--version".to_owned()],
    )?;
    let code = exit_code(&run, cfg)?;
    let stdout = String::from_utf8_lossy(&run.stdout_bytes()?)
        .trim()
        .to_owned();
    match (code, stdout.split_whitespace().last()) {
        (0, Some(v)) => Ok(v.to_owned()),
        _ => Err(refuse(
            "sca_scan",
            "tool-version",
            &format!("`cargo {sub} --version` exited {code} with {stdout:?}"),
        )),
    }
}

/// One tool's findings: `(id, package, version) -> cvss`.
type Findings = BTreeMap<(String, String, String), Option<String>>;

/// The `vulnerabilities` entries of one tool's JSON object (audit: `vulnerabilities.list`; deny's
/// audit-compatible output: `vulnerabilities`, an array).
fn findings_of(
    tool: &str,
    value: &serde_json::Value,
    into: &mut Findings,
) -> Result<(), StepError> {
    let v = value.get("vulnerabilities").ok_or_else(|| {
        refuse(
            "sca_scan",
            "report-malformed",
            &format!("{tool}'s report carries no `vulnerabilities`"),
        )
    })?;
    let list = v.get("list").unwrap_or(v).as_array().ok_or_else(|| {
        refuse(
            "sca_scan",
            "report-malformed",
            &format!("{tool}'s `vulnerabilities` is not a list"),
        )
    })?;
    for entry in list {
        let s = |path: &[&str]| {
            path.iter()
                .try_fold(entry, |v, k| v.get(k))
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        };
        let (Some(id), Some(package), Some(version)) = (
            s(&["advisory", "id"]),
            s(&["package", "name"]),
            s(&["package", "version"]),
        ) else {
            return Err(refuse(
                "sca_scan",
                "report-malformed",
                &format!("{tool} reported an advisory without an id, package or version: {entry}"),
            ));
        };
        let cvss = s(&["advisory", "cvss"]);
        into.entry((id, package, version)).or_insert(cvss);
    }
    Ok(())
}

/// A scan tool's run, which must exit 0 (nothing found) or 1 (findings).
fn scan_run(cfg: &Config, dir: &Path, tag: &str, args: &[String]) -> Result<Run, StepError> {
    let run = run::run(cfg, dir, tag, args)?;
    match exit_code(&run, cfg)? {
        0 | 1 => Ok(run),
        code => Err(StepError::NonZero {
            code: Some(code),
            stderr_digest: sha256(&run.stderr_bytes()?),
        }),
    }
}

/// The path deny's pinned DEBUG line names (refusal (e)): a `--format json` log line whose
/// `fields.message` is `Opening advisory database at '<path>'`.
fn deny_opened(stderr: &str) -> Option<String> {
    stderr.lines().find_map(|line| {
        let value: serde_json::Value = serde_json::from_str(line).ok()?;
        let message = value.get("fields")?.get("message")?.as_str()?;
        message
            .strip_prefix("Opening advisory database at '")?
            .strip_suffix('\'')
            .map(str::to_owned)
    })
}

fn sca_scan(
    tc: &RustToolchain,
    cfg: &Config,
    inputs: &BTreeMap<String, serde_json::Value>,
) -> Result<VerbOutput, StepError> {
    let audit_db = PathBuf::from(input::<String>(inputs, "sca_scan", "audit_db")?);
    let deny_parent = PathBuf::from(input::<String>(inputs, "sca_scan", "deny_db")?);

    // (d) deny's db-path holds exactly one database.
    let mut children: Vec<PathBuf> = std::fs::read_dir(&deny_parent)
        .map_err(|e| {
            refuse(
                "sca_scan",
                "deny-db-path-children",
                &format!(
                    "deny's db-path {} cannot be listed: {e}",
                    deny_parent.display()
                ),
            )
        })?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    children.sort();
    let [deny_db] = children.as_slice() else {
        return Err(refuse(
            "sca_scan",
            "deny-db-path-children",
            &format!(
                "deny's db-path {} holds {} databases, not exactly one: {children:?}",
                deny_parent.display(),
                children.len()
            ),
        ));
    };
    let deny_db = deny_db.clone();

    let dir = tc.run_dir()?;
    let root = Path::new(cfg.project_root().as_str());
    let mut findings = Findings::new();

    // ---- cargo audit, over copy A.
    let audit_before = db_head("cargo-audit", &audit_db)?;
    let audit_version = tool_version(cfg, &dir, "audit")?;
    let audit_args: Vec<String> = [
        "audit",
        "--json",
        "--no-fetch",
        "--db",
        &audit_db.to_string_lossy(),
        "--file",
        &root.join("Cargo.lock").to_string_lossy(),
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect();
    let audit = scan_run(cfg, &dir, "audit", &audit_args)?;
    let audit_after = db_head("cargo-audit", &audit_db)?;
    db_unmoved("cargo-audit", &audit_db, &audit_before, &audit_after)?;
    let audit_json: serde_json::Value =
        serde_json::from_slice(&audit.stdout_bytes()?).map_err(|e| {
            refuse(
                "sca_scan",
                "report-malformed",
                &format!("cargo audit's JSON: {e}"),
            )
        })?;
    // (c) a non-null report member is cross-checked, never trusted over the read.
    if let Some(member) = audit_json
        .get("database")
        .and_then(|d| d.get("last-commit"))
        .and_then(serde_json::Value::as_str)
        && member != audit_before
    {
        return Err(refuse(
            "sca_scan",
            "database-member-disagrees",
            &format!(
                "cargo audit's database.last-commit {member} is not the HEAD {audit_before} the adapter read"
            ),
        ));
    }
    findings_of("cargo audit", &audit_json, &mut findings)?;

    // ---- cargo deny, over copy B, under a generated config naming the db-path.
    let deny_before = db_head("cargo-deny", &deny_db)?;
    let deny_version = tool_version(cfg, &dir, "deny")?;
    let config = dir.join("deny.toml");
    std::fs::write(
        &config,
        format!(
            "[advisories]\ndb-path = \"{}\"\n",
            deny_parent.to_string_lossy().replace('\\', "/")
        ),
    )
    .map_err(|e| {
        io(format!(
            "could not write the deny config {}: {e}",
            config.display()
        ))
    })?;
    let deny_args: Vec<String> = [
        "deny",
        "--offline",
        "-L",
        "debug",
        "--format",
        "json",
        "--config",
        &config.to_string_lossy(),
        "check",
        "advisories",
        "--audit-compatible-output",
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect();
    let deny = scan_run(cfg, &dir, "deny", &deny_args)?;
    let deny_after = db_head("cargo-deny", &deny_db)?;
    db_unmoved("cargo-deny", &deny_db, &deny_before, &deny_after)?;
    // (e) deny's pinned DEBUG line names the child the adapter read, and only the PATH is read.
    let opened = deny_opened(&String::from_utf8_lossy(&deny.stderr_bytes()?));
    let same = opened.as_deref().is_some_and(|p| {
        match (std::fs::canonicalize(p), std::fs::canonicalize(&deny_db)) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        }
    });
    if !same {
        return Err(refuse(
            "sca_scan",
            "deny-db-path-line",
            &format!(
                "cargo deny's \"Opening advisory database at\" line names {opened:?}, not the database {} the adapter read",
                deny_db.display()
            ),
        ));
    }
    for line in String::from_utf8_lossy(&deny.stdout_bytes()?)
        .lines()
        .filter(|l| !l.trim().is_empty())
    {
        let value: serde_json::Value = serde_json::from_str(line).map_err(|e| {
            refuse(
                "sca_scan",
                "report-malformed",
                &format!("cargo deny's JSON: {e}"),
            )
        })?;
        findings_of("cargo deny", &value, &mut findings)?;
    }

    // ---- the report.
    let mut report = ScanReport {
        critical: 0,
        high: 0,
        medium: 0,
        low: 0,
        unrated: 0,
        advisories: Vec::with_capacity(findings.len()),
        tools: vec![
            ScanTool {
                name: "cargo-audit".to_owned(),
                version: audit_version,
                db_path: audit_db.display().to_string(),
                db_commit: audit_before,
            },
            ScanTool {
                name: "cargo-deny".to_owned(),
                version: deny_version,
                db_path: deny_db.display().to_string(),
                db_commit: deny_before,
            },
        ],
    };
    for ((id, package, version), cvss) in findings {
        let severity = severity_of(cvss.as_deref())?;
        let slot = match severity {
            Some(Severity::Critical) => &mut report.critical,
            Some(Severity::High) => &mut report.high,
            Some(Severity::Medium) => &mut report.medium,
            Some(Severity::Low) => &mut report.low,
            None => &mut report.unrated,
        };
        *slot = slot.saturating_add(1);
        report.advisories.push(Advisory {
            id,
            package,
            version,
            severity,
        });
    }
    output("ScanReport", &report)
}

// ------------------------------------------------------------------------------------ package

/// A built binary's `BuildArtifact` path: relative to the adapter's cargo target directory,
/// forward-slashed (record V decision 6).
///
/// # Errors
/// `package-binary-outside-target` naming both paths, for a binary outside the target directory:
/// it is never recorded as an absolute path (the fail-open record V flagged; s8c build, D63).
pub fn binary_path(target: &Path, exe: &Path) -> Result<String, StepError> {
    let relative = exe.strip_prefix(target).map_err(|_| {
        refuse(
            "package",
            "package-binary-outside-target",
            &format!(
                "the built binary {} is not under the adapter's target directory {}, so it has no path relative to it",
                exe.display(),
                target.display()
            ),
        )
    })?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn package(
    tc: &RustToolchain,
    cfg: &Config,
    inputs: &BTreeMap<String, serde_json::Value>,
) -> Result<VerbOutput, StepError> {
    let sha: String = input(inputs, "package", "sha")?;
    let declared: Vec<String> = match inputs.get("binaries") {
        Some(_) => input(inputs, "package", "binaries")?,
        None => Vec::new(),
    };
    if declared.is_empty() {
        return Err(refuse(
            "package",
            "undeclared-binary-set",
            "the call declares no shipped binary set, and nothing is digested by glob (a release build also links the fixture [[bin]]s; record V decision 6)",
        ));
    }
    let root = Path::new(cfg.project_root().as_str());
    let head = rk_git::head::head_commit(root)
        .map_err(|e| refuse("package", "head-unreadable", &e.to_string()))?;
    if head != sha {
        return Err(refuse(
            "package",
            "sha-head-mismatch",
            &format!(
                "the input sha {sha} is not the project root's HEAD {head}: a package built from another tree is not that sha's artifact"
            ),
        ));
    }
    let dir = tc.run_dir()?;
    let (report, stream) = RustToolchain::json_build(cfg, &dir, "package", &PACKAGE_BUILD)?;
    if report.status == BuildStatus::Failing {
        return Err(StepError::NonZero {
            code: None,
            stderr_digest: sha256(&serde_json::to_vec(&report).unwrap_or_default()),
        });
    }
    let exes = build::executables(&stream).map_err(|d| refuse("package", "build-stream", &d))?;
    let target = cfg.target_dir();
    let mut binaries = Vec::with_capacity(declared.len());
    for name in &declared {
        let Some((_, exe)) = exes.iter().find(|(n, _)| n == name) else {
            return Err(refuse(
                "package",
                "binary-not-built",
                &format!(
                    "the declared binary `{name}` is not one the build produced ({:?})",
                    exes.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>()
                ),
            ));
        };
        let bytes = std::fs::read(exe).map_err(|e| {
            io(format!(
                "could not read the built binary {}: {e}",
                exe.display()
            ))
        })?;
        let path = binary_path(&target, exe)?;
        binaries.push(ArtifactBinary {
            name: name.clone(),
            path,
            sha256: sha256(&bytes).as_str().to_owned(),
        });
    }
    output(
        "BuildArtifact",
        &BuildArtifact {
            profile: "release".to_owned(),
            binaries,
            argv: report.argv,
            toolchain_version: report.toolchain_version,
            sha,
            head,
        },
    )
}

// ------------------------------------------------------------------------------------ setup

fn setup(
    tc: &RustToolchain,
    cfg: &Config,
    inputs: &BTreeMap<String, serde_json::Value>,
) -> Result<VerbOutput, StepError> {
    let projects: Vec<String> = input(inputs, "setup", "projects")?;
    let root = Path::new(cfg.project_root().as_str());
    let pnpm = cfg.tool_program("pnpm", DEFAULT_PNPM);
    let mut rows = Vec::with_capacity(projects.len());
    let dir = if projects.is_empty() {
        None
    } else {
        Some(tc.run_dir()?)
    };
    for (i, project) in projects.iter().enumerate() {
        let rel = Path::new(project);
        if rel.is_absolute()
            || project.contains(':')
            || rel
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(refuse(
                "setup",
                "input-malformed",
                &format!("the declared project `{project}` is not a path inside the project root"),
            ));
        }
        let mut args: Vec<String> = SETUP_ARGS.iter().map(|s| (*s).to_owned()).collect();
        args.push(project.clone());
        let dir = dir.as_deref().unwrap_or(root);
        let run = run::run_program(cfg, dir, &format!("setup-{i}"), &pnpm, &args)?;
        let code = exit_code(&run, cfg)?;
        if code != 0 {
            return Err(StepError::Unsupported {
                capability: format!(
                    "setup: `{}` over the declared project `{project}` exited {code}; a worktree with half its dependencies is no place for a later verb, so setup refuses (record V decision 7)",
                    run.argv.join(" ")
                ),
            });
        }
        let lock = root.join(project).join("pnpm-lock.yaml");
        let bytes = std::fs::read(&lock).map_err(|e| {
            io(format!(
                "setup installed `{project}` but its lockfile {} cannot be read: {e}",
                lock.display()
            ))
        })?;
        rows.push(SetupProject {
            path: project.clone(),
            argv: run.argv,
            lockfile_sha256: sha256(&bytes).as_str().to_owned(),
        });
    }
    output(
        "SetupResult",
        &SetupResult {
            toolchain: "rust".to_owned(),
            projects: rows,
        },
    )
}
