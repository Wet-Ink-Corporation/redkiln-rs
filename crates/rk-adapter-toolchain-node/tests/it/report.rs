//! **The `VerifyReport` from the real `npx.cmd vitest` over the fixture project** — AC-329's
//! adapter half, AC-325, AC-331, AC-344's adapter share and DECISIONS-2026-09-09 #34's direct
//! invocation (s7c §7c.14, §7c.18).
//!
//! **Every count here comes from vitest's JSON reporter FILE** (`--reporter=json
//! --outputFile=<f>`, the two flags on the vector because the counts come from that file):
//! `selected_test_count` = `numPassedTests + numFailedTests` (what the run executed; skip-marked
//! and `-t`-unmatched tests are `numPendingTests` and not executed), pass/fail from the run's
//! exit status (RT-22). **The fixture's decoy is inert under the real vector** [measured
//! 2026-09-18: under `--reporter=json --outputFile=<f>` vitest echoes no console output and prints
//! no summary — stdout is the one line `JSON report written to <f>`, stderr is empty — so the
//! `Tests  99 passed` the `passes and prints the decoy` test prints reaches nothing]; a stdout
//! prose parser is therefore red by the ABSENCE of prose, not by the decoy: the mutant that parsed
//! `Tests  N passed` from the run's stdout file and ignored the reporter file was red as
//! `selected_test_count is the reporter file's executed count (2), not the decoy's 99 … left: 0 /
//! right: 2`. The decoy would be echoed only under the default reporter, which the adapter never
//! runs — it is kept so a vector that drops the reporter flag becomes observable.
//!
//! **AC-325's wrapper** is the crate's `rk-argv-wrapper-node` bin (the Rust crate's fixture
//! source under this crate's own target name): put in the adapter's `program` value in
//! `drop-tail:5` mode, it logs the argv the OS handed it and runs the real `npx.cmd` with the
//! trailing `-t <pattern>` DROPPED — so the recorded `argv` names the wrapper (not the template's
//! `npx.cmd`) and still carries the filter, the count is 2 where the filter would have made it
//! 1 (the drop is visible in the record), and re-executing the recorded vector verbatim
//! reproduces that count.

use std::time::Duration;

use rk_containment::{Outcome, run_bounded};
use rk_ports::toolchain::Toolchain;
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

use rk_adapter_toolchain_node::NodeToolchain;

use crate::support::{
    GENEROUS, SUITE_FILE, adapter, adapter_with_env, config_with, fixture_project, logged_vectors,
    minimal_env, reporter_counts, same_path, wrapper_bin,
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

/// The selection every filtered test uses: the one file, the `-t` pattern as its own element
/// (DECISIONS-2026-09-09 #20).
fn filtered(pattern: &str) -> Vec<String> {
    vec![SUITE_FILE.to_owned(), "-t".to_owned(), pattern.to_owned()]
}

/// The suite digest is over the suite's SOURCES and nothing vite leaves beside them. Vite compiles
/// `vitest.config.ts` to a transient `vitest.config.ts.timestamp-<ms>-<hash>.mjs` next to it for the
/// life of one run and deletes it after; a walk that admits it (a) reads a file that can vanish
/// between the listing and the read -- measured 2026-09-18 as `could not read ...timestamp-...mjs
/// for the suite digest: The system cannot find the file specified (os error 2)` under the crate's
/// parallel tests -- and (b) makes `suite_sha` differ while a run is in progress, which is a
/// digest that names no suite. RED before the exclusion: the two digests below differed.
#[test]
fn suite_sha_ignores_vites_transient_config_bundle() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::write(root.join("package.json"), b"{}\n").expect("write");
    std::fs::write(root.join("vitest.config.ts"), b"export default {}\n").expect("write");
    std::fs::create_dir(root.join("test")).expect("mkdir");
    std::fs::write(root.join("test/a.test.ts"), b"// a\n").expect("write");
    let before =
        rk_adapter_toolchain_node::report::suite_sha(root).expect("digest over the sources");
    std::fs::write(
        root.join("vitest.config.ts.timestamp-1789755454505-3bb7d96c12f0d.mjs"),
        b"// vite's transient bundle of the config\n",
    )
    .expect("write");
    let during = rk_adapter_toolchain_node::report::suite_sha(root)
        .expect("digest with the transient present");
    assert_eq!(
        before, during,
        "vite's transient config bundle must not enter the suite digest: it exists only while a run is \
         in progress and is not a source"
    );
    dir.close().expect("the walk holds no handle");
}

/// **AC-329 (adapter half).** The report's nine members come from structured sources — the
/// count from the reporter file, never from the decoy's prose — and each member absent in turn
/// is refused.
#[test]
fn ac329_report_conforms_no_prose_parse() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());

    let report = tc
        .verify_report(&[])
        .expect("the unselected fixture suite is green");

    // Under `--reporter=json` no prose reaches the adapter (the decoy is inert; the module doc):
    // a stdout parser reports 0, the reporter file says 2.
    assert_eq!(
        report.selected_test_count, 2,
        "selected_test_count is the reporter file's executed count (2), not the decoy's 99"
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
        report.argv[0],
        rk_adapter_toolchain_node::DEFAULT_PROGRAM,
        "argv[0] is the per-OS program element (npx.cmd on Windows, npx elsewhere)"
    );
    assert_eq!(&report.argv[1..3], ["vitest", "run"]);
    assert_eq!(report.argv[3], "--reporter=json");
    assert!(
        report.argv[4].starts_with("--outputFile="),
        "the reporter file is on the vector: {:?}",
        report.argv
    );
    assert_eq!(
        report.argv.len(),
        5,
        "an unselected run adds nothing after the reporter flags"
    );
    assert!(
        report.toolchain_version.starts_with("vitest/"),
        "toolchain_version is `vitest --version`'s answer: {:?}",
        report.toolchain_version
    );
    assert_eq!(
        report.suite_sha.len(),
        64,
        "suite_sha is a sha256 hex digest"
    );
    assert!(report.suite_sha.bytes().all(|b| b.is_ascii_hexdigit()));
    assert!(report.wall_secs < GENEROUS.as_secs());

    satisfies_store_whole_suite_rule(&report);

    // A `-t` selection narrows the executed count; the total follows it (the store's rule).
    let selected = tc.verify_report(&filtered("filterable")).expect("green");
    assert_eq!(selected.selected_test_count, 1);
    assert_eq!(selected.total_test_count, 1);
    satisfies_store_whole_suite_rule(&selected);
    assert_eq!(&selected.argv[5..], [SUITE_FILE, "-t", "filterable"]);

    // The skip-marked red test, reached by `-t`: RT-22's red suite is Err(NonZero), never Ok.
    match tc.verify_report(&filtered("red_by_selection")) {
        Err(StepError::NonZero { code, .. }) => assert_ne!(code, Some(0)),
        other => panic!("a red suite is Err(NonZero) (RT-22); got {other:?}"),
    }

    // The nine negatives, one per member.
    for member in MEMBERS {
        member_absent_is_refused(&report, member);
    }
    // The suite digest is stable across two runs of the same tree.
    assert_eq!(report.suite_sha, selected.suite_sha);
}

/// **Two concurrent calls on one adapter share no output file.** `Toolchain: Send + Sync` with
/// `&self` methods, so the port permits two `verify_report` calls on one adapter at once; each
/// call's stdout / stderr / reporter files live under the call's OWN run directory
/// (`<scratch>/run-<n>`), never one fixed set of names rewritten per run — or the second run's
/// `--outputFile` would overwrite the first's before the first read its counts. Two threads, a
/// filtered and an unfiltered run, each reads its own count.
#[test]
fn concurrent_verify_reports_share_no_output_file() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    let (filtered_count, whole_count) = std::thread::scope(|s| {
        let a = s.spawn(|| tc.verify_report(&filtered("filterable")));
        let b = s.spawn(|| tc.verify_report(&[]));
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
    assert_ne!(
        filtered_count.argv[4], whole_count.argv[4],
        "the two runs named two different reporter files: {:?} / {:?}",
        filtered_count.argv[4], whole_count.argv[4]
    );
}

/// **DECISIONS-2026-09-09 #34.** The ordered vector minus the program element is exactly
/// `vitest run --reporter=json --outputFile=<tmp> <file> -t <pattern>` — separate elements, the
/// reporter flags in place — and never `pnpm`, `test` or `--` anywhere on it.
#[test]
fn d34_vitest_invoked_directly_never_pnpm_forwarded() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    let report = tc.verify_report(&filtered("filterable")).expect("green");
    assert!(
        report
            .argv
            .iter()
            .all(|a| a != "pnpm" && a != "test" && a != "--" && !a.contains("pnpm test")),
        "never `pnpm test --` forwarding (#34): {:?}",
        report.argv
    );
    // The per-OS program element, pinned BY NAME (§7c.14: `npx.cmd` on Windows — the launcher
    // `CreateProcess` can start — and `npx` elsewhere); comparing against `DEFAULT_PROGRAM` would
    // be tautological.
    #[cfg(windows)]
    assert_eq!(
        report.argv[0], "npx.cmd",
        "the per-OS program element on Windows is the `.cmd` launcher (§7c.14)"
    );
    #[cfg(not(windows))]
    assert_eq!(
        report.argv[0], "npx",
        "the per-OS program element off Windows is the shell script (§7c.14)"
    );
    let tail = &report.argv[1..];
    assert_eq!(
        tail.len(),
        7,
        "the vector minus the program element: {tail:?}"
    );
    assert_eq!(tail[0], "vitest");
    assert_eq!(tail[1], "run");
    assert_eq!(tail[2], "--reporter=json");
    assert!(tail[3].starts_with("--outputFile="), "{tail:?}");
    assert_eq!(tail[4], SUITE_FILE);
    assert_eq!(tail[5], "-t");
    assert_eq!(tail[6], "filterable");
    assert_eq!(
        report.selected_test_count, 1,
        "the filter survived: one test ran"
    );
}

/// **AC-325.** Through the argument-dropping wrapper: the recorded argv is the OS vector (it
/// names the wrapper, and carries the filter the wrapper dropped), the drop is visible in the
/// record as a count of 2 where the real program gives 1, the wrapper logged the full vector,
/// and re-executing the recorded vector verbatim reproduces the count.
#[test]
fn ac325_recorded_argv_is_os_vector() {
    let scratch = tempfile::tempdir().expect("scratch");
    let log = scratch.path().join("argv.log");
    let wrapper = wrapper_bin();
    let extra = [
        ("RK_ARGV_LOG", log.to_string_lossy().into_owned()),
        (
            "RK_WRAPPED_PROGRAM",
            rk_adapter_toolchain_node::DEFAULT_PROGRAM.to_owned(),
        ),
        ("RK_WRAPPER_MODE", "drop-tail:5".to_owned()),
    ];
    let through_wrapper = adapter_with_env(
        wrapper.clone(),
        GENEROUS,
        &fixture_project(),
        &extra,
        scratch.path(),
    );

    let report = through_wrapper
        .verify_report(&filtered("filterable"))
        .expect("green through the wrapper");

    // (1) the recorded argv differs from the template: its program element is the wrapper.
    assert!(
        same_path(&report.argv[0], &wrapper.to_string_lossy()),
        "argv[0] is the resolved program the OS was handed ({}), not the template's npx: {:?}",
        wrapper.display(),
        report.argv
    );
    assert_eq!(
        &report.argv[5..],
        [SUITE_FILE, "-t", "filterable"],
        "the vector as elements, filter included — what was HANDED, not what survived"
    );
    // (2) the drop is visible in the record: the wrapper dropped `-t filterable`, so the whole
    // file ran and the count is 2; the real program under the same selection gives 1.
    assert_eq!(
        report.selected_test_count, 2,
        "the wrapper dropped the trailing `-t <pattern>`: the whole file's two tests ran"
    );
    let control = adapter(GENEROUS, scratch.path());
    assert_eq!(
        control
            .verify_report(&filtered("filterable"))
            .expect("green")
            .selected_test_count,
        1,
        "the control: the real program honours the filter"
    );
    // (3) what the OS actually saw: the wrapper logged the full vector it received.
    let logged = logged_vectors(&log);
    assert!(
        logged.iter().any(|v| {
            v.len() == report.argv.len()
                && same_path(&v[0], &report.argv[0])
                && v[1..] == report.argv[1..]
        }),
        "the recorded run vector was handed to the OS as recorded; logged: {logged:?}"
    );
    // (4) re-execution of the recorded vector, verbatim, reproduces the count.
    let out_path = scratch.path().join("rerun.stdout");
    let err_path = scratch.path().join("rerun.stderr");
    let out = std::fs::File::create(&out_path).expect("stdout file");
    let err = std::fs::File::create(&err_path).expect("stderr file");
    let mut env = minimal_env();
    for (k, v) in &extra {
        env.insert((*k).to_owned(), v.clone());
    }
    let outcome = run_bounded(reexec_command(&report.argv, &env, out, err), GENEROUS)
        .expect("the recorded vector spawns");
    assert!(
        matches!(outcome, Outcome::Exited(s) if s.success()),
        "re-executing the recorded argv verbatim is green: {outcome:?}"
    );
    let (passed, failed) = reporter_counts(&report.argv);
    assert_eq!(
        passed + failed,
        u64::from(report.selected_test_count),
        "re-executing the recorded vector reproduces the recorded count (AC-325's AND)"
    );
}

/// The re-execution `Command`: program = the recorded `argv[0]`, args = the rest, the same map;
/// output to files (never a pipe — DECISIONS-2026-09-09 #31).
#[expect(
    clippy::disallowed_types,
    reason = "AC-325's re-execution instrument builds a std Command from the RECORDED vector and \
              hands it to rk-containment's run_bounded; a test site, outside every src/ ban."
)]
fn reexec_command(
    argv: &[String],
    env: &std::collections::BTreeMap<String, String>,
    out: std::fs::File,
    err: std::fs::File,
) -> std::process::Command {
    let mut cmd = std::process::Command::new(&argv[0]);
    cmd.args(&argv[1..])
        .current_dir(fixture_project())
        .env_clear()
        .envs(env)
        .stdout(std::process::Stdio::from(out))
        .stderr(std::process::Stdio::from(err));
    cmd
}

/// **AC-331.** A verify run killed by its bound is `Unverifiable`'s input — never `Ok` and
/// never a failing test attributable to the candidate: `Err(Timeout { after_secs })`, the
/// bound's own value, from `run_bounded`'s `TimedOut` (ADR-0127's forbids: never an exit status).
/// The 8-second adapter selects the skip-marked slow test, which holds for 600 s.
#[test]
fn ac331_killed_verify_is_unverifiable() {
    let scratch = tempfile::tempdir().expect("scratch");
    let short = adapter(Duration::from_secs(8), scratch.path());
    match short.test(&filtered("slow_by_selection")) {
        Err(StepError::Timeout { after_secs }) => assert_eq!(after_secs, 8),
        Err(StepError::NonZero { .. }) => {
            panic!("a run killed at its bound was read as a FAILING TEST (AC-331 forbids it)")
        }
        Ok(n) => panic!("a run killed at its bound was read as a PASS of {n} tests (AC-331)"),
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
    use rk_adapter_toolchain_node::report::{OS_KILLED_DETAIL_PREFIX, os_killed};
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

/// **AC-344 (adapter share).** An absent program — an ABSOLUTE nonexistent `.exe` path — is
/// `Transport { kind: Spawn }`, never a failing test and never `NonZero`; `detect` answers
/// `false` for it. (`.exe`, not `.cmd`: `std` runs a `.cmd` through `cmd.exe /c`, which starts
/// and exits 1 on a missing script — an exit status, not a spawn refusal.)
#[test]
fn ac344_missing_binary_is_not_a_candidate_failure() {
    let scratch = tempfile::tempdir().expect("scratch");
    let missing = scratch.path().join("no-such-npx.exe");
    assert!(!missing.exists(), "PREMISE: the path does not exist");
    let tc = NodeToolchain::new(config_with(
        missing.clone(),
        GENEROUS,
        &fixture_project(),
        minimal_env(),
        scratch.path(),
    ))
    .expect("loads");

    match tc.test(&[]) {
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
}
