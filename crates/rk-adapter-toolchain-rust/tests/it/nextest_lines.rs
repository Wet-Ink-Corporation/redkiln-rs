//! `nextest_lines::` — the strict cargo-nextest status-line parser, pinned per nextest version
//! (s8c §8c.1's nextest bullet, D21-48 X15; record T = ADR-0169 decision 4; D21-62 B's `ListFailed`
//! refusal, provisional).
//!
//! PURE, like `libtest_lines::`: the parser reads nextest's status stream (STDERR, measured on
//! `0.9.146`) and an exit code. The one child this module runs is `cargo nextest --version`, to
//! prove the INSTALLED version is a pinned one. The goldens under `tests/fixtures/nextest/0.9.146/`
//! are parsed, never compared byte for byte; each has its expected result beside it.

use std::path::{Path, PathBuf};

use rk_adapter_toolchain_rust::libtest_lines::{self, ParseRefusal, Tally};
use rk_adapter_toolchain_rust::nextest_lines::{NEXTEST_PINNED, parse};

const PIN: &str = "0.9.146";

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nextest/0.9.146")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn json(name: &str) -> serde_json::Value {
    serde_json::from_str(&read(&golden_dir().join(name))).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn n(v: &serde_json::Value, key: &str) -> u32 {
    u32::try_from(v[key].as_u64().unwrap_or_else(|| panic!("member `{key}`"))).expect("fits u32")
}

fn exit_of(v: &serde_json::Value) -> i32 {
    i32::try_from(v["exit_code"].as_i64().expect("exit_code")).expect("fits i32")
}

const SEP: &str = "────────────";

fn stream(body: &[&str]) -> String {
    let mut v = vec![
        "    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s",
        SEP,
        " Nextest run ID 00000000-0000-0000-0000-000000000000 with nextest profile: default",
    ];
    v.extend_from_slice(body);
    let mut s = v.join("\n");
    s.push('\n');
    s
}

/// `cargo nextest --version`'s stdout, from the cargo on this suite's `PATH`.
#[expect(
    clippy::disallowed_types,
    reason = "a test reading the INSTALLED nextest's version line to prove it is a pinned one; the \
              adapter's own children go through rk-containment (ADR-0127 decision 3)"
)]
fn installed_nextest_version_line() -> String {
    let out = std::process::Command::new("cargo")
        .args(["nextest", "--version"])
        .output()
        .expect("cargo nextest --version runs");
    assert!(
        out.status.success(),
        "cargo nextest --version: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8")
}

#[test]
fn the_pinned_nextest_goldens_parse_to_their_recorded_counts() {
    // The installed nextest is a pinned one (D21-48 X15: the stage-open install IS the pin).
    let text = installed_nextest_version_line();
    let installed = text
        .lines()
        .find_map(|l| l.strip_prefix("cargo-nextest "))
        .and_then(|r| r.split_whitespace().next())
        .unwrap_or_else(|| panic!("no `cargo-nextest <version>` line in {text:?}"));
    assert!(
        NEXTEST_PINNED.contains(&installed),
        "installed {installed} is not in NEXTEST_PINNED {NEXTEST_PINNED:?}"
    );
    assert_eq!(NEXTEST_PINNED, [PIN]);

    let captures = json("captures.json");
    let files = captures["files"].as_object().expect("captures.json files");
    let mut parsed = 0;

    // The workspace red in package `a`, with its doctest companion parsed by the libtest parser.
    let want = json("workspace-red-in-a.tally.json");
    let nx = parse(
        &read(&golden_dir().join("workspace-red-in-a.stderr")),
        exit_of(&want["nextest"]),
        PIN,
    )
    .expect("workspace-red-in-a.stderr parses");
    assert_eq!(
        (nx.passed, nx.failed),
        (n(&want["nextest"], "passed"), n(&want["nextest"], "failed"))
    );
    assert_eq!(nx.passed + nx.failed, n(&want["nextest"], "starting"));
    parsed += 1;
    let doc = libtest_lines::parse(
        &read(&golden_dir().join("workspace-red-in-a.doc.stdout")),
        exit_of(&want["doc_companion"]),
        libtest_lines::PINNED[0],
    )
    .expect("the doctest companion parses");
    assert_eq!(
        (doc.passed, doc.failed),
        (
            n(&want["doc_companion"], "passed"),
            n(&want["doc_companion"], "failed")
        )
    );
    parsed += 1;
    assert_eq!(
        (nx.passed + doc.passed, nx.failed + doc.failed),
        (n(&want["total"], "passed"), n(&want["total"], "failed"))
    );
    assert_eq!(
        (nx.passed + doc.passed, nx.failed + doc.failed),
        (4, 1),
        "= the libtest workspace-red golden's 4/1"
    );

    // D21-62 B: the -E-filtered project streams, parse-only. They carry build.rs's decoy lines on
    // the status stream, and neither decoy is counted.
    for name in ["project-green.filtered", "project-red.filtered"] {
        let body = read(&golden_dir().join(format!("{name}.stderr")));
        assert!(
            body.contains("test phantom ... ok"),
            "{name} carries the decoy record"
        );
        let want = json(&format!("{name}.tally.json"));
        let t = parse(&body, exit_of(&want), PIN).unwrap_or_else(|e| panic!("{name} refused: {e}"));
        assert_eq!(
            (t.passed, t.failed, t.ignored),
            (n(&want, "passed"), n(&want, "failed"), n(&want, "ignored")),
            "{name}"
        );
        assert!(
            files.contains_key(&format!("{name}.stderr")),
            "captures.json records {name}"
        );
        parsed += 1;
    }

    // D21-62 B: the unfiltered project streams are list failures (exit 104), named refusals.
    for name in ["project-green.list-failed", "project-red.list-failed"] {
        let body = read(&golden_dir().join(format!("{name}.stderr")));
        let want = json(&format!("{name}.expected.json"));
        assert_eq!(want["refusal"], "ListFailed");
        let exit = exit_of(&want);
        assert_eq!(exit, 104);
        assert_eq!(
            parse(&body, exit, PIN),
            Err(ParseRefusal::ListFailed {
                version: PIN.to_owned(),
                exit: 104
            }),
            "{name}"
        );
        assert!(
            files.contains_key(&format!("{name}.stderr")),
            "captures.json records {name}"
        );
        parsed += 1;
    }
    assert_eq!(parsed, 6);
}

#[test]
fn an_unpinned_nextest_version_is_refused_by_name() {
    let body = read(&golden_dir().join("workspace-red-in-a.stderr"));
    for v in ["0.9.145", "0.9.147", "0.9", ""] {
        let r = parse(&body, 100, v);
        assert_eq!(
            r,
            Err(ParseRefusal::Unsupported {
                capability: format!("nextest-lines@{v}")
            }),
            "version {v:?}"
        );
        let msg = r.expect_err("refused").to_string();
        assert!(
            msg.contains(&format!("nextest-lines@{v}")),
            "the refusal names the version: {msg}"
        );
    }
    assert!(
        parse(&body, 100, PIN).is_ok(),
        "the control: the pinned version parses the same stream"
    );
}

#[test]
fn a_summary_that_disagrees_with_its_status_lines_is_unverifiable() {
    // (a) two PASS records under a Summary claiming one failure (exit 100 agrees with the Summary,
    // so only the Summary-against-records comparison can refuse).
    let s = stream(&[
        "    Starting 2 tests across 1 binary",
        "        PASS [   0.001s] (1/2) x a",
        "        PASS [   0.001s] (2/2) x b",
        SEP,
        "     Summary [   0.002s] 2 tests run: 1 passed, 1 failed, 0 skipped",
        "error: test run failed",
    ]);
    assert_eq!(
        parse(&s, 100, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 8 })
    );

    // (b) a PASS and a FAIL under a Summary claiming two passes at exit 0.
    let s = stream(&[
        "    Starting 2 tests across 1 binary",
        "        PASS [   0.001s] (1/2) x a",
        "        FAIL [   0.001s] (2/2) x b",
        SEP,
        "     Summary [   0.002s] 2 tests run: 2 passed, 0 skipped",
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 8 })
    );

    // (c) the Starting count against the records.
    let s = stream(&[
        "    Starting 3 tests across 1 binary",
        "        PASS [   0.001s] (1/3) x a",
        "        PASS [   0.001s] (2/3) x b",
        SEP,
        "     Summary [   0.002s] 2 tests run: 2 passed, 0 skipped",
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Err(ParseRefusal::StartingMismatch {
            line: 4,
            starting: 3,
            records: 2
        })
    );

    // (d) no Summary at all, and a Summary line outside the pinned grammar.
    let s = stream(&[
        "    Starting 1 test across 1 binary",
        "        PASS [   0.001s] (1/1) x a",
    ]);
    assert_eq!(parse(&s, 0, PIN), Err(ParseRefusal::SummaryMissing));
    let s = stream(&[
        "    Starting 1 test across 1 binary",
        "        PASS [   0.001s] (1/1) x a",
        "     Summary [   0.002s] 1 test run: 1 passed (1 flaky), 0 skipped",
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Err(ParseRefusal::MalformedSummary { line: 6 })
    );

    // The control: the same records under an agreeing Summary parse.
    let s = stream(&[
        "    Starting 2 tests across 1 binary (1 test skipped)",
        "        PASS [   0.001s] (1/2) x a",
        "        FAIL [   0.001s] (2/2) x b",
        SEP,
        "     Summary [   0.002s] 2 tests run: 1 passed, 1 failed, 1 skipped",
        "error: test run failed",
    ]);
    assert_eq!(
        parse(&s, 100, PIN),
        Ok(Tally {
            passed: 1,
            failed: 1,
            ignored: 1,
            blocks: 1,
            foreign_lines: 0,
            first_foreign: None
        })
    );
}

#[test]
fn a_nonzero_exit_with_no_fail_line_is_unverifiable_never_green() {
    let green = stream(&[
        "    Starting 1 test across 1 binary",
        "        PASS [   0.001s] (1/1) x a",
        SEP,
        "     Summary [   0.001s] 1 test run: 1 passed, 0 skipped",
    ]);
    // (a) the test-failure exit with no FAIL line: something failed that no record names.
    assert_eq!(
        parse(&green, 100, PIN),
        Err(ParseRefusal::UnattributedFailure { exit_code: 100 })
    );
    // (b) an exit that is neither 0 nor 100 is never tallied, with status lines or without.
    assert_eq!(
        parse(&green, 101, PIN),
        Err(ParseRefusal::UnexpectedExit { exit_code: 101 })
    );
    assert_eq!(
        parse(
            "error: no tests to run\n(hint: use `--no-tests` to customize)\n",
            4,
            PIN
        ),
        Err(ParseRefusal::UnexpectedExit { exit_code: 4 })
    );
    // (c) a zero exit over a FAIL line.
    let red = stream(&[
        "    Starting 1 test across 1 binary",
        "        FAIL [   0.001s] (1/1) x a",
        SEP,
        "     Summary [   0.001s] 1 test run: 0 passed, 1 failed, 0 skipped",
    ]);
    assert_eq!(
        parse(&red, 0, PIN),
        Err(ParseRefusal::ExitDisagrees {
            exit_code: 0,
            failed: 1
        })
    );
    // The control: each under the exit nextest gives it.
    assert_eq!(
        parse(&green, 0, PIN).map(|t| (t.passed, t.failed)),
        Ok((1, 0))
    );
    assert_eq!(
        parse(&red, 100, PIN).map(|t| (t.passed, t.failed)),
        Ok((0, 1))
    );
}

/// **A selection argument the installed nextest does not accept is refused NAMING it** (§8c.1's
/// nextest bullet; record T = ADR-0169 decision 4, proposed: *"An argument the installed nextest
/// does not accept refuses by name; it is never dropped"*). Measured on `0.9.146` through the
/// no-dispatch wrapper: nextest prints the pinned line *"error: failed to parse test binary
/// arguments `<args>`: arguments are unsupported"* and exits 96 before running any test.
///
/// *(Added 2026-09-26, s8c refuting round 6: before it, exit 96 refused as `UnexpectedExit`, which
/// names only the exit, and no test covered the case.)*
#[test]
fn an_argument_nextest_rejects_is_refused_naming_it() {
    let files = json("captures.json");
    assert!(
        files["files"]
            .as_object()
            .expect("captures.json files")
            .contains_key("workspace.args-rejected.stderr"),
        "captures.json records the golden"
    );
    let body = read(&golden_dir().join("workspace.args-rejected.stderr"));
    let want = json("workspace.args-rejected.expected.json");
    assert_eq!(want["refusal"], "ArgumentsRejected");
    let exit = exit_of(&want);
    assert_eq!(exit, 96);
    let args = want["args"].as_str().expect("args");
    let r = parse(&body, exit, PIN);
    assert_eq!(
        r,
        Err(ParseRefusal::ArgumentsRejected {
            version: PIN.to_owned(),
            args: args.to_owned(),
        }),
        "the golden"
    );
    let msg = r.expect_err("refused").to_string();
    assert!(
        msg.contains("`--frobnicate, --show-output`"),
        "the refusal names the rejected arguments: {msg}"
    );

    // One argument, among cargo's build lines on the same stream.
    let one = "   Compiling a v0.1.0\nerror: failed to parse test binary arguments `--test-threads=1`: \
               arguments are unsupported\n";
    assert_eq!(
        parse(one, 96, PIN),
        Err(ParseRefusal::ArgumentsRejected {
            version: PIN.to_owned(),
            args: "--test-threads=1".to_owned(),
        })
    );

    // CONTROLS: exit 96 WITHOUT the pinned line is still `UnexpectedExit`, and the pinned line
    // under another non-green exit is not read as a rejection.
    assert_eq!(
        parse("error: something else\n", 96, PIN),
        Err(ParseRefusal::UnexpectedExit { exit_code: 96 })
    );
    assert_eq!(
        parse(&body, 101, PIN),
        Err(ParseRefusal::UnexpectedExit { exit_code: 101 })
    );
    // A line that only starts like the pinned one (no closing text) is not a rejection either.
    assert_eq!(
        parse(
            "error: failed to parse test binary arguments `--x`\n",
            96,
            PIN
        ),
        Err(ParseRefusal::UnexpectedExit { exit_code: 96 })
    );
}

#[test]
fn a_retry_or_timeout_line_is_unverifiable_never_counted() {
    // (a) a retried test: two runs of one test. Counted as records, TRY 1 FAIL + TRY 2 PASS would
    // balance a Starting of 2 and a 1/1 Summary, so only the refusal can catch it.
    let tried = stream(&[
        "    Starting 2 tests across 1 binary",
        "  TRY 1 FAIL [   0.007s] (───) x flaky",
        "  TRY 2 PASS [   0.005s] (1/2) x flaky",
        SEP,
        "     Summary [   0.017s] 2 tests run: 1 passed, 1 failed, 0 skipped",
    ]);
    assert_eq!(
        parse(&tried, 100, PIN),
        Err(ParseRefusal::RetryOrKill {
            line: 5,
            text: "  TRY 1 FAIL [   0.007s] (───) x flaky".to_owned()
        })
    );

    // (b) a timed-out test is a kill, not a failure (AC-331). Counted as a FAIL record, the
    // TIMEOUT line would balance Starting, Summary and the exit.
    let timed_out = stream(&[
        "    Starting 2 tests across 1 binary",
        "        PASS [   0.001s] (1/2) x a",
        " TERMINATING [>  2.000s] (───) x hang",
        "     TIMEOUT [   2.020s] (2/2) x hang",
        SEP,
        "     Summary [   2.021s] 2 tests run: 1 passed, 1 failed, 0 skipped",
    ]);
    assert_eq!(
        parse(&timed_out, 100, PIN),
        Err(ParseRefusal::RetryOrKill {
            line: 6,
            text: " TERMINATING [>  2.000s] (───) x hang".to_owned()
        })
    );
    let timeout_only = stream(&[
        "    Starting 2 tests across 1 binary",
        "        PASS [   0.001s] (1/2) x a",
        "     TIMEOUT [   2.020s] (2/2) x hang",
        SEP,
        "     Summary [   2.021s] 2 tests run: 1 passed, 1 failed, 0 skipped",
    ]);
    assert_eq!(
        parse(&timeout_only, 100, PIN),
        Err(ParseRefusal::RetryOrKill {
            line: 6,
            text: "     TIMEOUT [   2.020s] (2/2) x hang".to_owned()
        })
    );

    // (c) every other kill or retry word refuses the same way.
    for word in [
        "RETRY", "FLAKY", "ABORT", "SIGSEGV", "SIGABRT", "SIGKILL", "SIGTERM", "LEAK",
    ] {
        let line = format!("{word:>12} [   0.212s] (1/1) x a");
        let s = stream(&["    Starting 1 test across 1 binary", &line]);
        assert_eq!(
            parse(&s, 100, PIN),
            Err(ParseRefusal::RetryOrKill {
                line: 5,
                text: line.clone()
            }),
            "{word}"
        );
    }

    // (d) the control: a SLOW notice is not a retry. It is skipped when it names a recorded test,
    // and refuses when it names none.
    let slow = stream(&[
        "    Starting 1 test across 1 binary",
        "        SLOW [> 60.000s] (───────) x lattice::p6",
        "        PASS [  61.000s] (1/1) x lattice::p6",
        SEP,
        "     Summary [  61.001s] 1 test run: 1 passed (1 slow), 0 skipped",
    ]);
    assert_eq!(
        parse(&slow, 0, PIN).map(|t| (t.passed, t.failed)),
        Ok((1, 0))
    );
    let ghost = slow.replace(
        "SLOW [> 60.000s] (───────) x lattice::p6",
        "SLOW [> 60.000s] (───────) x ghost",
    );
    assert_eq!(
        parse(&ghost, 0, PIN),
        Err(ParseRefusal::SlowNoticeUnrecorded {
            line: 5,
            name: "x ghost".to_owned()
        })
    );
}
