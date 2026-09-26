//! **The `VerifyReport` from the real `cargo` over the fixture project** — AC-329's adapter half,
//! AC-325, AC-331, AC-344's adapter share and ADR-0094 decision 1's recorded nextest fallback
//! (s7c §7c.14, §7c.18).
//!
//! **Every count here comes from libtest's stable `--list` (RT-26)** — `selected_test_count` is
//! the `: test` lines of `cargo test <selection> -- --list` minus those of `-- --list --ignored`,
//! `total_test_count` that same count (the basis is WholeSuite, and the store's ADR-0104 d3 rule
//! requires the pair `(executed, OwnExecutedCount)` — `satisfies_store_whole_suite_rule`; RT-26's
//! unfiltered-listing total is refused there and is not read), pass/fail from the run's exit status
//! (RT-22) — and the fixture's decoys are what make a prose parse observable ON THE STREAM THE
//! ADAPTER READS: the fixture's `tests/zz_decoy.rs` (a `harness = false` test target) prints
//! `test result: ok. 99 passed; 0 failed` and `test phantom ... ok` to STDOUT on every `cargo
//! test`, the `--list` passes included — and `build.rs` prints the same two lines as
//! `cargo:warning=` lines, which cargo echoes on STDERR on every build. [Measured 2026-09-18: with
//! the stderr decoy alone, the run's stdout carried libtest's REAL summary — `test result: ok. 2
//! passed; …` last — so a stdout summary-line mutant reported 2 and passed `ac329_…`; with the
//! stdout decoy in place the same mutant (the last `test result:` line of the run's stdout) was
//! red: `selected_test_count is libtest's --list count (2), not the decoy's 99 … left: 99 /
//! right: 2`.] So a summary-line parser over either stream reports 99, an unanchored per-test
//! line-counter over-counts, and the structured source reports 2.
//!
//! **AC-325's wrapper** is the crate's `rk-argv-wrapper-rust` bin (`tests/fixtures/wrapper/`): put
//! in the adapter's `program` value, it logs the argv the OS handed it and runs the real `cargo`,
//! so the recorded `argv` is asserted against what the OS saw — the run vector AND both count
//! vectors — and the recorded count vectors are re-executed verbatim, with the adapter's own
//! subtraction, for the same count. **The register's argument-DROPPING arm is discharged on the
//! Node side only** (`rk-adapter-toolchain-node`'s `ac325_…`, `drop-tail:5`): on the Rust vector
//! the tail is the count route itself — `-- <selection> --list [--ignored]` — so a tail-drop
//! removes `--list` before it removes the filter and the wrapped `cargo` RUNS the suite where a
//! listing was asked for (zero `: test` lines, a count of 0 that says nothing about the drop);
//! the Node vector's tail is the filter (`-t <pattern>`) with the count source elsewhere (the
//! reporter file), which is where a dropped tail is visible in the record as a count.

use std::path::PathBuf;
use std::time::Duration;

use rk_containment::{Outcome, run_bounded};
use rk_ports::conformance::validate_report;
use rk_ports::toolchain::{TestStatus, Toolchain};
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

use rk_adapter_toolchain_rust::{ForcedRunner, Runner, RunnerPolicy, RustToolchain};

use crate::support::{
    GENEROUS, adapter, adapter_over, adapter_with_env, config_with, filter, fixture_project,
    fixture_workspace, logged_vectors, minimal_env, nextest_absent_env, run_test, same_path,
    toolchain_cargo, whole, wrapper_bin,
};

/// The nine members of `VerifyReport`, as the wire spells them (AC-329's list; `basis_reason`
/// last, the one member that may be `null`).
const MEMBERS: [&str; 9] = [
    "basis",
    "argv",
    "selected_test_count",
    "total_test_count",
    "total_test_count_source",
    "wall_secs",
    "toolchain_version",
    "suite_sha",
    "basis_reason",
];

/// The store's rule on a whole-suite report (ADR-0104 decision 3 as `rk-store` implements it —
/// `rk_store::artifacts::verify_report::total_test_count_fields(Basis::WholeSuite, executed, _)` is
/// `(executed, OwnExecutedCount)` and `validate_report` refuses any other pair): `total_test_count`
/// IS `selected_test_count`, from `OwnExecutedCount`. Restated here rather than imported because
/// §7c.19's manifest row for this crate names no `rk-store` edge.
fn satisfies_store_whole_suite_rule(report: &VerifyReport) {
    assert_eq!(report.basis, Basis::WholeSuite);
    assert_eq!(
        (
            report.total_test_count,
            report.total_test_count_source.clone()
        ),
        (
            report.selected_test_count,
            TotalTestCountSource::OwnExecutedCount
        ),
        "on a whole-suite run total_test_count must be the run's own executed count from          OwnExecutedCount, or rk-store's validate_report refuses the report (ADR-0104 d3)"
    );
}

/// Does a report with `member` removed fail at the boundary — a parse failure for eight of them,
/// and for `basis_reason` (defaulted so the refusal names the rule, AC-352) the shape check?
fn member_absent_is_refused(report: &VerifyReport, member: &str) {
    let mut value = serde_json::to_value(report).expect("serialises");
    let obj = value.as_object_mut().expect("an object");
    assert!(
        obj.remove(member).is_some(),
        "the serialised report carries `{member}` (PREMISE)"
    );
    let text = serde_json::to_string(&value).expect("serialises");
    match serde_json::from_str::<VerifyReport>(&text) {
        Err(e) => assert!(
            e.to_string().contains(member),
            "the refusal for the absent `{member}` names it: {e}"
        ),
        Ok(parsed) => {
            assert_eq!(
                member, "basis_reason",
                "only basis_reason may parse when absent"
            );
            assert!(
                parsed.validate_shape().is_err(),
                "a whole-suite report with no basis_reason fails the shape check (AC-352)"
            );
        }
    }
}

/// **AC-329 (adapter half).** The report's nine members come from structured sources — the
/// count from `--list`, never from the decoy's prose — and each member absent in turn is refused.
#[test]
fn ac329_report_conforms_no_prose_parse() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());

    // *(Amended 2026-09-26, s8c: the retired inherent `verify_report` is the port's `test`, and
    // the report is the outcome's.)*
    let report = run_test(&tc, &whole())
        .expect("the unselected fixture suite is green")
        .report;

    // The decoys: `tests/zz_decoy.rs` printed `test result: ok. 99 passed; 0 failed` and `test
    // phantom ... ok` on STDOUT (the run and both `--list` passes), and cargo echoed build.rs's
    // copy on STDERR on every build; the structured source says 2.
    assert_eq!(
        report.selected_test_count, 2,
        "selected_test_count is libtest's `--list` count (2), not the decoy's 99 nor a line count of 3"
    );
    report
        .validate_shape()
        .expect("the report's own invariants hold");
    assert_eq!(report.total_test_count, 2);
    assert_eq!(
        report.total_test_count_source,
        TotalTestCountSource::OwnExecutedCount
    );
    assert_eq!(report.basis, Basis::WholeSuite);
    assert_eq!(report.basis_reason, Some(BasisReason::ProbeUnverified));
    assert_eq!(
        report.argv,
        vec![
            rk_adapter_toolchain_rust::DEFAULT_PROGRAM.to_owned(),
            "test".to_owned(),
            "--workspace".to_owned(),
            "--no-fail-fast".to_owned()
        ],
        "argv is the run vector actually handed to the OS (ADR-0094 d1's `cargo test --workspace`, \
         with record T decision 6's --no-fail-fast)"
    );
    assert!(
        report.toolchain_version.starts_with("rustc "),
        "toolchain_version is `rustc -V`'s answer, not a hard-coded pin: {:?}",
        report.toolchain_version
    );
    assert!(
        !report.toolchain_version.contains("+1.98.1"),
        "no third copy of the toolchain pin (ADR-0102/ADR-0083)"
    );
    assert_eq!(
        report.suite_sha.len(),
        64,
        "suite_sha is a sha256 hex digest"
    );
    assert!(report.suite_sha.bytes().all(|b| b.is_ascii_hexdigit()));
    assert!(
        report.wall_secs < GENEROUS.as_secs(),
        "wall_secs is the run's wall time, inside the bound"
    );

    satisfies_store_whole_suite_rule(&report);

    // A selection narrows the selected count — and, the basis being WholeSuite (AC-328), the
    // total is that same executed count, or the store refuses the report (ADR-0104 d3; RT-26's
    // "total from the unfiltered --list" cannot coexist with a WholeSuite basis — the slice's
    // routing against ADR-0094 d1).
    let selected = run_test(&tc, &filter(&["passes_and_is_filterable"]))
        .expect("green")
        .report;
    assert_eq!(selected.selected_test_count, 1);
    assert_eq!(selected.total_test_count, 1);
    satisfies_store_whole_suite_rule(&selected);
    assert_eq!(
        selected.argv,
        vec![
            rk_adapter_toolchain_rust::DEFAULT_PROGRAM.to_owned(),
            "test".to_owned(),
            "--workspace".to_owned(),
            "--no-fail-fast".to_owned(),
            "--".to_owned(),
            "passes_and_is_filterable".to_owned()
        ]
    );

    // The ignored set, reached by selection: a selection of the ignored red test is a red suite.
    // *(Amended 2026-09-26, s8c: this read "RT-22's red suite — Err(NonZero), never Ok(count)".
    // Record T = ADR-0169 reverses RT-22: a red suite is Ok with its counts.)*
    match run_test(&tc, &filter(&["--ignored", "red_by_selection"])) {
        Ok(red) => assert_eq!(
            (red.passed, red.failed, red.report.selected_test_count),
            (0, 1, 1),
            "a red suite is Ok with its counts (record T decision 1)"
        ),
        other => {
            panic!("a red suite is Ok with failed = 1 (record T; RT-22 reversed); got {other:?}")
        }
    }

    // The nine negatives, one per member.
    for member in MEMBERS {
        member_absent_is_refused(&report, member);
    }
    // The suite digest is stable across two runs of the same tree.
    assert_eq!(report.suite_sha, selected.suite_sha);
}

/// **AC-325.** The recorded argv is the OS vector — through the wrapper it names the wrapper,
/// not the template's `cargo` — and the count vectors (the run vector plus `-- --list` /
/// `-- --list --ignored`) were handed to the OS as recorded; re-executing the recorded vector
/// reproduces the same count.
#[test]
fn ac325_recorded_argv_is_os_vector() {
    let scratch = tempfile::tempdir().expect("scratch");
    let log = scratch.path().join("argv.log");
    let wrapper = wrapper_bin();
    let tc = adapter_with_env(
        wrapper.clone(),
        GENEROUS,
        &[
            ("RK_ARGV_LOG", log.to_string_lossy().into_owned()),
            (
                "RK_WRAPPED_PROGRAM",
                rk_adapter_toolchain_rust::DEFAULT_PROGRAM.to_owned(),
            ),
            ("RK_WRAPPER_MODE", "forward".to_owned()),
        ],
        scratch.path(),
    );

    let report = run_test(&tc, &filter(&["passes_and_is_filterable"]))
        .expect("green through the wrapper")
        .report;
    assert_eq!(report.selected_test_count, 1);

    // (1) the recorded argv differs from the template: its program element is the wrapper the
    // OS was handed, and the rest is the vector, element by element.
    assert!(
        same_path(&report.argv[0], &wrapper.to_string_lossy()),
        "argv[0] is the resolved program the OS was handed ({}), not the template's `cargo`: {:?}",
        wrapper.display(),
        report.argv
    );
    assert_eq!(
        &report.argv[1..],
        [
            "test",
            "--workspace",
            "--no-fail-fast",
            "--",
            "passes_and_is_filterable"
        ],
        "the vector as elements, never a templated string"
    );

    // (2) what the OS actually saw: the wrapper logged every vector it received.
    let logged = logged_vectors(&log);
    let seen = |want: &[String]| {
        logged
            .iter()
            .any(|v| v.len() == want.len() && same_path(&v[0], &want[0]) && v[1..] == want[1..])
    };
    assert!(
        seen(&report.argv),
        "the recorded run vector was handed to the OS as recorded; logged: {logged:?}"
    );
    let mut list = report.argv.clone();
    list.push("--list".to_owned());
    assert!(
        seen(&list),
        "the count vector `<run> --list` was handed to the OS (RT-26); logged: {logged:?}"
    );
    let mut list_ignored = list.clone();
    list_ignored.push("--ignored".to_owned());
    assert!(
        seen(&list_ignored),
        "the count vector `<run> --list --ignored` was handed to the OS (RT-26); logged: {logged:?}"
    );

    // (3) re-execution of the recorded vectors, verbatim, reproduces the count: the run exits 0,
    // and the two count vectors — `<run> --list` minus `<run> --list --ignored`, the adapter's own
    // subtraction — give the same one test.
    let rerun = |argv: &[String], tag: &str| -> (Outcome, String) {
        let out_path = scratch.path().join(format!("{tag}.stdout"));
        let err_path = scratch.path().join(format!("{tag}.stderr"));
        let out = std::fs::File::create(&out_path).expect("stdout file");
        let err = std::fs::File::create(&err_path).expect("stderr file");
        let mut env = minimal_env();
        env.insert("RK_ARGV_LOG".to_owned(), log.to_string_lossy().into_owned());
        env.insert(
            "RK_WRAPPED_PROGRAM".to_owned(),
            rk_adapter_toolchain_rust::DEFAULT_PROGRAM.to_owned(),
        );
        env.insert("RK_WRAPPER_MODE".to_owned(), "forward".to_owned());
        let outcome = run_bounded(
            reexec_command(argv, &env, scratch.path(), out, err),
            GENEROUS,
        )
        .expect("the recorded vector spawns");
        (
            outcome,
            std::fs::read_to_string(&out_path).expect("stdout readable"),
        )
    };
    let (outcome, _) = rerun(&report.argv, "rerun");
    assert!(
        matches!(outcome, Outcome::Exited(s) if s.success()),
        "re-executing the recorded argv verbatim is green: {outcome:?}"
    );
    let (_, listing) = rerun(&list, "rerun-list");
    let (_, listing_ignored) = rerun(&list_ignored, "rerun-list-ignored");
    let relisted = rk_adapter_toolchain_rust::report::count_listed(&listing).saturating_sub(
        rk_adapter_toolchain_rust::report::count_listed(&listing_ignored),
    );
    assert_eq!(
        relisted, report.selected_test_count,
        "re-executing the recorded count vectors, with the adapter's own subtraction, reproduces \
         the recorded count (AC-325's AND)"
    );
}

/// The re-execution `Command`: program = the recorded `argv[0]`, args = the rest, the same map,
/// the same target dir; output to files (never a pipe — DECISIONS-2026-09-09 #31).
#[expect(
    clippy::disallowed_types,
    reason = "AC-325's re-execution instrument builds a std Command from the RECORDED vector and \
              hands it to rk-containment's run_bounded; a test site, outside every src/ ban."
)]
fn reexec_command(
    argv: &[String],
    env: &std::collections::BTreeMap<String, String>,
    scratch: &std::path::Path,
    out: std::fs::File,
    err: std::fs::File,
) -> std::process::Command {
    let mut cmd = std::process::Command::new(&argv[0]);
    cmd.args(&argv[1..])
        .current_dir(fixture_project())
        .env_clear()
        .envs(env)
        .env("CARGO_TARGET_DIR", scratch.join("target"))
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::from(err));
    cmd
}

/// **Two concurrent calls on one adapter share no output file.** `Toolchain: Send + Sync` with
/// `&self` methods, so the port permits two `verify_report` calls on one adapter at once; each
/// call's stdout / stderr files live under the call's OWN run directory (`<scratch>/run-<n>`),
/// never one fixed set of names rewritten per run — or the second run's `--list` stdout would
/// overwrite the first's before the first counted it. Two threads, a filtered and an unfiltered
/// run, each reads its own count (the target dir is shared; cargo's own lock serialises the
/// builds, not the reads).
#[test]
fn concurrent_verify_reports_share_no_output_file() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    assert_eq!(
        run_test(&tc, &whole()).expect("green").passed,
        2,
        "PREMISE: built and green"
    );
    let (filtered_count, whole_count) = std::thread::scope(|s| {
        let a = s.spawn(|| run_test(&tc, &filter(&["passes_and_is_filterable"])).map(|o| o.report));
        let b = s.spawn(|| run_test(&tc, &whole()).map(|o| o.report));
        (
            a.join().expect("thread a").expect("green"),
            b.join().expect("thread b").expect("green"),
        )
    });
    assert_eq!(
        filtered_count.selected_test_count, 1,
        "the filtered run read ITS count, not the concurrent unfiltered run's"
    );
    assert_eq!(
        whole_count.selected_test_count, 2,
        "the unfiltered run read ITS count, not the concurrent filtered run's"
    );
}

/// **AC-331.** A verify run killed by its bound is `Unverifiable`'s input — never `Ok` and
/// never a failing test attributable to the candidate: `Err(Timeout { after_secs })`, the
/// bound's own value, from `run_bounded`'s `TimedOut` (ADR-0127's forbids: never an exit status).
///
/// The fixture is warmed first under a generous bound over the SAME scratch (so the count passes
/// do not compile under the short bound and time out for the wrong reason); then a 5-second
/// adapter selects the `#[ignore]` slow test, which holds for 600 s.
#[test]
fn ac331_killed_verify_is_unverifiable() {
    let scratch = tempfile::tempdir().expect("scratch");
    let warm = adapter(GENEROUS, scratch.path());
    assert_eq!(
        run_test(&warm, &whole()).expect("green").passed,
        2,
        "PREMISE: the fixture is built and green"
    );

    let short = adapter(Duration::from_secs(5), scratch.path());
    let outcome = run_test(&short, &filter(&["--ignored", "slow_by_selection"]));
    match outcome {
        Err(StepError::Timeout { after_secs }) => assert_eq!(after_secs, 5),
        Err(StepError::NonZero { .. }) => {
            panic!("a run killed at its bound was read as a FAILING TEST (AC-331 forbids it)")
        }
        Ok(o) => panic!("a run killed at its bound was read as an outcome (AC-331): {o:?}"),
        Err(other) => panic!("a killed run is Timeout {{ after_secs }}, got {other:?}"),
    }
}

/// **AC-331's OS-killed arm has a shape 7d can key on without reading prose.** `rk_types::StepError`
/// carries no OS-kill variant (`Timeout { after_secs }` is the bound's, `Cancelled { by }` the
/// token's — AC-225), so an `OsKilled` run is `Transport { kind: Io }` like an unreadable output
/// file — and the two are told apart by a DOCUMENTED, EXPORTED detail prefix
/// (`report::OS_KILLED_DETAIL_PREFIX`), which the pure constructor `report::os_killed` puts first
/// and every OS-killed arm in the crate goes through. The variant itself is RT-3's OWED record's to
/// decide (ADR-0127 §7 leaves it open); until then the prefix is the contract, asserted here.
#[test]
fn os_killed_run_is_keyed_by_detail_prefix() {
    use rk_adapter_toolchain_rust::report::{OS_KILLED_DETAIL_PREFIX, os_killed};
    let argv = vec!["prog".to_owned(), "arg".to_owned()];
    match os_killed(&argv) {
        StepError::Transport {
            kind: TransportKind::Io,
            detail,
        } => {
            assert!(
                detail.starts_with(OS_KILLED_DETAIL_PREFIX),
                "the OS-killed detail starts with the exported prefix {OS_KILLED_DETAIL_PREFIX:?}: {detail:?}"
            );
            assert!(
                detail.contains("prog") && detail.contains("arg"),
                "the detail names the vector: {detail:?}"
            );
        }
        other => panic!(
            "an OS-killed run is Transport {{ Io }} (no OS-kill variant exists on StepError; RT-3); got {other:?}"
        ),
    }
    assert!(
        !OS_KILLED_DETAIL_PREFIX.is_empty() && !OS_KILLED_DETAIL_PREFIX.starts_with("could not"),
        "the prefix is distinguishable from the crate's own file-read failures"
    );
}

/// **AC-344 (adapter share).** An absent program — an ABSOLUTE nonexistent path, since a
/// PATH-less child map cannot induce the condition on Windows — is `Transport { kind: Spawn }`,
/// never a failing test and never `NonZero`; `detect` answers `false` for it.
#[test]
fn ac344_missing_binary_is_not_a_candidate_failure() {
    let scratch = tempfile::tempdir().expect("scratch");
    let missing = scratch.path().join("no-such-cargo.exe");
    assert!(!missing.exists(), "PREMISE: the path does not exist");
    let tc = RustToolchain::new(config_with(
        missing.clone(),
        GENEROUS,
        &fixture_project(),
        minimal_env(),
        scratch.path(),
    ))
    .expect("loads");

    match run_test(&tc, &whole()) {
        Err(StepError::Transport {
            kind: TransportKind::Spawn,
            detail,
        }) => assert!(
            !detail.is_empty(),
            "the spawn refusal carries the OS's reason"
        ),
        Err(StepError::NonZero { .. }) => panic!(
            "an absent {} was classified as a FAILING TEST / NonZero — AC-344's input to `Unverifiable {{ cannot_run }}` is Transport {{ Spawn }}",
            missing.display()
        ),
        other => panic!("an absent program is Transport {{ Spawn }}; got {other:?}"),
    }
    assert!(
        !tc.detect()
            .expect("detect never fails on an absent program"),
        "detect answers false: the toolchain is not present"
    );
    let build_dir: PathBuf = scratch.path().join("target");
    assert!(
        !build_dir.exists() || std::fs::read_dir(&build_dir).is_ok_and(|mut d| d.next().is_none()),
        "nothing was built"
    );
}

/// **ADR-0094 d1's fallback, recorded.** ~~`cargo nextest` is ABSENT on this box [measured
/// 2026-09-17], so the report's `argv` is the `cargo test --workspace` vector and names no
/// `nextest`. The presence arm is unobservable here and is ARMED (§7c.22).~~
///
/// *(Re-pointed 2026-09-26, s8c §8c.9, D21-48 X15; the NAME is kept, because Checkpoint 7c names it.)*
/// cargo-nextest is installed on this box from `s8c` on, so "absent on this box" is false. The body
/// now runs over the probe's ABSENT case -- the toolchain's own cargo under a `child_env` with no
/// `cargo-nextest` on `PATH` or under `CARGO_HOME` -- under `Detect`, and asserts that the fallback
/// is recorded twice: the run vector is `cargo test --workspace --no-fail-fast` naming no
/// `nextest`, and `runner.name` is `cargo-test`. The presence arm is live, and proved by
/// `runner::the_probe_finds_the_installed_nextest_and_names_its_version`.
#[test]
fn nextest_fallback_recorded_on_report() {
    let scratch = tempfile::tempdir().expect("scratch");
    let cargo_home = tempfile::tempdir().expect("an empty CARGO_HOME");
    let tc = RustToolchain::new(
        rk_adapter_toolchain_rust::Config::new(
            toolchain_cargo(),
            GENEROUS,
            &fixture_project(),
            nextest_absent_env(cargo_home.path()),
            scratch.path().to_path_buf(),
        )
        .expect("a Config constructs"),
    )
    .expect("loads");
    assert_eq!(
        tc.runner(),
        &Runner::CargoTest,
        "PREMISE: under Detect with no cargo-nextest reachable, the probe chose cargo test"
    );
    let outcome = run_test(&tc, &whole()).expect("green");
    assert!(
        outcome.report.argv.iter().all(|a| !a.contains("nextest")),
        "the fallback's recorded vector names no nextest: {:?}",
        outcome.report.argv
    );
    assert_eq!(
        &outcome.report.argv[1..],
        ["test", "--workspace", "--no-fail-fast"],
        "the fallback IS the recorded vector (ADR-0094 d1's `cargo test --workspace`, with record T \
         decision 6's --no-fail-fast)"
    );
    assert_eq!(
        outcome.runner.name, "cargo-test",
        "the fallback is recorded on the outcome's runner too (D21-48 X15)"
    );
    assert_eq!(outcome.passed, 2);
}

// ---------------------------------------------------------------- s8c: the outcome (record T)
//
// *(Added 2026-09-26, s8c §8c.1-§8c.2 and §8c.9; record T = ADR-0169, proposed.)* `test` returns a
// `TestOutcome`: a red suite is `Ok` with `failed ≥ 1`, the counts come from the pinned line
// parsers, and `passed + failed` must equal the `--list` count.

/// Every `<tag>.stdout` / `<tag>.stderr` file the adapter wrote under a scratch directory's run
/// directories, read back: the streams the parsers read, for a test's PREMISE about what was on them.
fn run_files(scratch: &std::path::Path, name: &str) -> Vec<String> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(scratch).expect("the scratch directory is readable") {
        let dir = entry.expect("an entry").path();
        let file = dir.join(name);
        if dir.is_dir() && file.is_file() {
            found.push(std::fs::read_to_string(&file).expect("a readable run file"));
        }
    }
    found
}

/// **A red suite is `Ok` with failed counts** (D21-42 1a: *"A red suite is `Ok` with counts;
/// `Err` is only cannot-run/timeout/killed (AC-331/AC-344)"*). Over the fixture project, the
/// ignored red test selected: `Ok`, 0 passed and 1 failed, `status: failing`, the report valid.
#[test]
fn red_suite_is_ok_with_failed_counts() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    let outcome = run_test(&tc, &filter(&["--ignored", "red_by_selection"])).expect(
        "a RED suite is Ok with counts, never Err(NonZero) (record T decision 1; RT-22 reversed)",
    );
    assert_eq!(
        (outcome.passed, outcome.failed),
        (0, 1),
        "the red test is tallied as failed: {outcome:?}"
    );
    assert_eq!(outcome.status, TestStatus::Failing);
    assert_eq!(outcome.report.selected_test_count, 1);
    assert_eq!(outcome.runner.name, "cargo-test");
    validate_report(&outcome).expect("the red outcome passes the conformance row");
    satisfies_store_whole_suite_rule(&outcome.report);

    // And the green side of the same adapter: the unselected suite is Ok and passing.
    let green = run_test(&tc, &whole()).expect("the unselected suite is green");
    assert_eq!((green.passed, green.failed), (2, 0));
    assert_eq!(green.status, TestStatus::Passing);
}

/// **A build failure is `Err`, never a red suite** (record T decision 1's class (a): *"the build
/// fails before any test executes (`NonZero` with no per-test record)"*). A crate that does not
/// compile has no test to fail: its `--list` pass fails, and that is `Err(NonZero)`, never
/// `Ok { failed ≥ 1 }` and never `Ok` with nothing executed.
#[test]
fn a_build_failure_is_err_never_a_red_suite() {
    let scratch = tempfile::tempdir().expect("scratch");
    let project = tempfile::tempdir().expect("a throwaway crate");
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"broken\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n",
    )
    .expect("write the manifest");
    std::fs::create_dir_all(project.path().join("src")).expect("mkdir src");
    std::fs::write(
        project.path().join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    \"not a number\"\n}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn would_pass() {}\n}\n",
    )
    .expect("write the broken source");
    let tc = RustToolchain::new(config_with(
        rk_adapter_toolchain_rust::DEFAULT_PROGRAM.into(),
        GENEROUS,
        project.path(),
        minimal_env(),
        scratch.path(),
    ))
    .expect("loads");
    match run_test(&tc, &whole()) {
        Err(StepError::NonZero { code, .. }) => {
            assert_ne!(code, Some(0), "a build that fails is a non-zero cargo exit");
        }
        Ok(outcome) => panic!(
            "a crate that does not COMPILE ran no test, so it is cannot-run (Err), never a red or \
             an empty suite (record T decision 1): {outcome:?}"
        ),
        Err(other) => panic!("a build failure before any test is NonZero; got {other:?}"),
    }
}

/// **`passed + failed` is the executed count, and it survives both decoys** (AC-329; record T
/// decision 3). The fixture's `zz_decoy` prints `test phantom ... ok` and `test result: ok. 99
/// passed` on the RUN child's stdout, the stream the libtest parser reads; `build.rs` prints the
/// same on stderr. The parser counts records only inside a `running N tests` block, so the tally is
/// 2 -- never 99, never 3 -- and it equals the `--list` count.
#[test]
fn passed_plus_failed_is_the_executed_count_and_survives_both_decoys() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    let outcome = run_test(&tc, &whole()).expect("the unselected fixture suite is green");
    assert_eq!(
        (outcome.passed, outcome.failed),
        (2, 0),
        "the tally is the two real tests: not the decoy's 99, not 2 + the phantom record"
    );
    assert_eq!(
        outcome.passed + outcome.failed,
        outcome.report.selected_test_count,
        "passed + failed is the --list count (record T decision 1's invariant)"
    );
    // PREMISE: the decoy WAS on the stream the parser read, or the 2 above proves nothing.
    let stdouts = run_files(scratch.path(), "run.stdout");
    assert!(
        stdouts
            .iter()
            .any(|s| s.contains("test phantom ... ok") && s.contains("99 passed")),
        "PREMISE: the run child's stdout carried zz_decoy's phantom record and summary"
    );
}

/// **A red test in an early binary still tallies every binary** (record T decision 6: the run
/// child passes `--no-fail-fast`). Over §8c.3's workspace with its red test selected in, the red
/// sits in package `a`'s unit-test binary, the FIRST binary cargo runs; every later block is still
/// run and tallied, so `passed + failed` equals the `--list` count (the golden's 4 / 1 over 6
/// blocks).
#[test]
fn a_red_test_in_an_early_binary_still_tallies_every_binary() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter_over(
        &fixture_workspace(),
        RunnerPolicy::Force(ForcedRunner::CargoTest),
        scratch.path(),
    );
    let outcome = run_test(&tc, &filter(&["--include-ignored"]))
        .expect("a red suite in an early binary is Ok, and every binary is tallied");
    assert_eq!(
        (outcome.passed, outcome.failed),
        (4, 1),
        "every block tallied: a's red plus the four passing tests (two unit, one integration, one \
         doctest): {outcome:?}"
    );
    assert_eq!(
        outcome.passed + outcome.failed,
        outcome.report.selected_test_count,
        "passed + failed is the --list count"
    );
    assert!(
        outcome.report.argv.iter().any(|a| a == "--no-fail-fast"),
        "the run child carries --no-fail-fast (record T decision 6): {:?}",
        outcome.report.argv
    );
    let stdouts = run_files(scratch.path(), "run.stdout");
    let headers = stdouts
        .iter()
        .map(|s| s.lines().filter(|l| l.starts_with("running ")).count())
        .max()
        .unwrap_or(0);
    assert_eq!(
        headers, 6,
        "PREMISE: all six binaries' blocks were printed after the first one failed"
    );
}

/// **The nextest path records its runner and tallies a red suite as `Ok`** (D21-48 X15; record T
/// decision 4). *(Re-pointed by D21-62 B: from "over the fixture project red" to §8c.3's workspace
/// red in package `a`, because the fixture project's `zz_decoy` does not answer nextest's
/// `--list`.)* Under `Force(nextest)`: `argv` names `nextest`, `runner` is cargo-nextest at the pin,
/// and the red suite is `Ok` with `failed ≥ 1`.
#[test]
fn the_nextest_path_records_its_runner_and_tallies_a_red_suite_as_ok() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter_over(
        &fixture_workspace(),
        RunnerPolicy::Force(ForcedRunner::Nextest),
        scratch.path(),
    );
    let outcome = run_test(&tc, &filter(&["--include-ignored"]))
        .expect("a red suite under nextest is Ok with counts");
    assert!(
        outcome.report.argv.iter().any(|a| a == "nextest"),
        "the recorded run vector names nextest exactly when nextest ran (AC-325): {:?}",
        outcome.report.argv
    );
    assert_eq!(outcome.runner.name, "cargo-nextest");
    assert!(
        rk_adapter_toolchain_rust::nextest_lines::NEXTEST_PINNED
            .contains(&outcome.runner.version.as_str()),
        "the runner names its pinned version: {:?}",
        outcome.runner
    );
    assert!(outcome.failed >= 1, "the red test is tallied: {outcome:?}");
    assert_eq!(outcome.status, TestStatus::Failing);
    validate_report(&outcome).expect("the nextest outcome passes the conformance row");
}

/// **The nextest path runs the doctests beside it, so the count matches `--list`** (record T
/// decision 5). `cargo nextest run` runs no doctests, and `--list` lists them: without the
/// `cargo test --doc` companion the invariant refuses. Over the green workspace: the companion's
/// vector carries `--doc`, the doctest is tallied, and `passed + failed` is the `--list` count.
#[test]
fn the_nextest_path_runs_doctests_beside_it_so_the_count_matches_list() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter_over(
        &fixture_workspace(),
        RunnerPolicy::Force(ForcedRunner::Nextest),
        scratch.path(),
    );
    let outcome = run_test(&tc, &whole()).expect("the green workspace under nextest");
    let companion = outcome
        .runner
        .companion_argv
        .clone()
        .expect("package a has a doctest-bearing library, so the companion ran");
    assert!(
        companion.iter().any(|a| a == "--doc"),
        "the companion is the doctest child: {companion:?}"
    );
    assert_eq!(
        (outcome.passed, outcome.failed),
        (4, 0),
        "three tests under nextest plus a's one doctest under the companion: {outcome:?}"
    );
    assert_eq!(
        outcome.passed + outcome.failed,
        outcome.report.selected_test_count,
        "passed + failed is the --list count, which lists the doctest"
    );
    assert_eq!(outcome.status, TestStatus::Passing);
}

/// **A nextest list failure is Unverifiable, never green** (D21-62 B, provisional; record T
/// decision 4). The fixture project's `zz_decoy` (`harness = false`) prints prose on `--list`, so
/// `cargo-nextest 0.9.146` exits 104 *"creating test list failed"* with no status line. Under
/// `Force(nextest)` that is `Transport { Decode }` naming `ListFailed` and the version -- never a
/// tally, and nothing falls back to `cargo test`.
#[test]
fn a_nextest_list_failure_is_unverifiable_never_green() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter_over(
        &fixture_project(),
        RunnerPolicy::Force(ForcedRunner::Nextest),
        scratch.path(),
    );
    match run_test(&tc, &whole()) {
        Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        }) => assert!(
            detail.contains("ListFailed") && detail.contains("0.9.146"),
            "the refusal names ListFailed and the version: {detail}"
        ),
        other => panic!(
            "a nextest list failure is Transport {{ Decode }} naming ListFailed, never a tally or \
             a fall back; got {other:?}"
        ),
    }
}
