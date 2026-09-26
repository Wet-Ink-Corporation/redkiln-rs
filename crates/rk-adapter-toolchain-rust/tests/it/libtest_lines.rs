//! `libtest_lines::` — the strict libtest-line parser, pinned per toolchain version (s8c §8c.1,
//! D21-43 N4; record T = ADR-0169 decision 3; D21-62 A's foreign-line skip, provisional).
//!
//! Every test here is PURE: the parser reads a string and an exit code, so no child runs. The
//! goldens under `tests/fixtures/libtest/1.98.1/` are captured run-child stdout files, each with its
//! expected tally beside it (`<name>.tally.json`); they are PARSED, never compared byte for byte
//! with a fresh run (`finished in` varies). The synthetic streams below each isolate one rule, so a
//! mutant that removes that rule turns exactly the test that names it red (§8c.9's M11–M14, M25 and
//! D21-62 A's three).

use std::path::{Path, PathBuf};

use rk_adapter_toolchain_rust::libtest_lines::{ForeignLine, PINNED, ParseRefusal, Tally, parse};

const PIN: &str = "1.98.1";

/// The six goldens §8c.1 lists, by the name `captures.json` records.
const GOLDENS: [&str; 6] = [
    "project-green",
    "project-red",
    "workspace-green",
    "workspace-red-in-a",
    "doctest-a",
    "this-repository",
];

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/libtest/1.98.1")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The expected tally beside a golden, and the exit code its capture recorded.
fn expected(name: &str) -> (serde_json::Value, i32) {
    let v: serde_json::Value =
        serde_json::from_str(&read(&golden_dir().join(format!("{name}.tally.json"))))
            .unwrap_or_else(|e| panic!("{name}.tally.json: {e}"));
    let exit =
        i32::try_from(v["exit_code"].as_i64().expect("exit_code")).expect("exit_code fits i32");
    (v, exit)
}

fn n(v: &serde_json::Value, key: &str) -> u32 {
    u32::try_from(
        v[key]
            .as_u64()
            .unwrap_or_else(|| panic!("tally member `{key}`")),
    )
    .expect("fits u32")
}

/// A synthetic block: a header, its record region, the empty line, and a summary.
const SUMMARY_OK_1: &str =
    "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s";
const SUMMARY_OK_2: &str =
    "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s";

fn lines(ls: &[&str]) -> String {
    let mut s = ls.join("\n");
    s.push('\n');
    s
}

#[test]
fn the_pinned_goldens_parse_to_their_recorded_counts() {
    // The pin: rust-toolchain.toml's channel is a release PINNED lists, so a pin bump that forgets
    // its golden turns this red.
    let toolchain = read(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rust-toolchain.toml"));
    let channel = toolchain
        .lines()
        .find_map(|l| {
            let l = l.trim();
            l.strip_prefix("channel = \"")
                .and_then(|r| r.strip_suffix('"'))
        })
        .expect("rust-toolchain.toml carries a channel line");
    assert_eq!(channel, PIN, "the channel this suite is written against");
    assert!(
        PINNED.contains(&channel),
        "rust-toolchain.toml's channel {channel} is not in PINNED {PINNED:?}"
    );
    assert_eq!(PINNED, [PIN]);

    // captures.json names exactly the six goldens, and each parses to its recorded tally.
    let captures: serde_json::Value =
        serde_json::from_str(&read(&golden_dir().join("captures.json"))).expect("captures.json");
    let mut recorded: Vec<&str> = captures["files"]
        .as_object()
        .expect("captures.json files")
        .keys()
        .map(|k| k.strip_suffix(".stdout").expect("a .stdout golden"))
        .collect();
    recorded.sort_unstable();
    let mut want = GOLDENS.to_vec();
    want.sort_unstable();
    assert_eq!(recorded, want, "captures.json and §8c.1's six goldens");

    let mut parsed = 0;
    for name in GOLDENS {
        let stdout = read(&golden_dir().join(format!("{name}.stdout")));
        let (v, exit) = expected(name);
        let t = parse(&stdout, exit, PIN).unwrap_or_else(|e| panic!("golden {name} refused: {e}"));
        assert_eq!(
            (t.passed, t.failed, t.ignored, t.blocks),
            (
                n(&v, "passed"),
                n(&v, "failed"),
                n(&v, "ignored"),
                n(&v, "blocks")
            ),
            "golden {name}"
        );
        let foreign = v.get("foreign_lines").map_or(0, |_| n(&v, "foreign_lines"));
        assert_eq!(t.foreign_lines, foreign, "golden {name}: foreign lines");
        parsed += 1;
    }
    assert_eq!(parsed, 6);
}

#[test]
fn a_record_line_outside_a_block_is_never_counted() {
    // s7c's stdout decoy prints a per-test record and a SHORT summary with no `running` header; a
    // record before any block, and a FAILED record after the last, must move no count.
    let stdout = lines(&[
        "test phantom_before ... ok",
        "",
        "running 1 test",
        "test real ... ok",
        "",
        SUMMARY_OK_1,
        "",
        "test result: ok. 99 passed; 0 failed",
        "test phantom ... ok",
        "test ghost ... FAILED",
    ]);
    let t = parse(&stdout, 0, PIN).expect("a green block beside decoys parses");
    assert_eq!(
        t,
        Tally {
            passed: 1,
            failed: 0,
            ignored: 0,
            blocks: 1,
            foreign_lines: 0,
            first_foreign: None
        }
    );

    // The captured decoy itself: project-green's stdout carries `test phantom ... ok` after its
    // last summary; the tally is the two real passes.
    let t =
        parse(&read(&golden_dir().join("project-green.stdout")), 0, PIN).expect("project-green");
    assert_eq!((t.passed, t.failed, t.ignored, t.blocks), (2, 0, 2, 2));
}

#[test]
fn a_block_whose_record_count_differs_from_its_header_is_unverifiable() {
    // (a) the header states three records and the region holds two.
    let short = lines(&[
        "running 3 tests",
        "test a ... ok",
        "test b ... ok",
        "",
        SUMMARY_OK_2,
    ]);
    assert_eq!(
        parse(&short, 0, PIN),
        Err(ParseRefusal::RecordCountMismatch {
            line: 1,
            header: 3,
            records: 2
        })
    );

    // (b) D21-62 A test 2: a leaked record-shaped line over-counts against the header.
    let leaked = lines(&[
        "running 2 tests",
        "test a ... ok",
        "test x ... ok",
        "test b ... ok",
        "",
        SUMMARY_OK_2,
    ]);
    assert_eq!(
        parse(&leaked, 0, PIN),
        Err(ParseRefusal::RecordCountMismatch {
            line: 1,
            header: 2,
            records: 3
        })
    );

    // (c) D21-62 A test 4: a leaked EMPTY line closes the region early, so the count falls short.
    // A pinned fail-closed false refusal.
    let early = lines(&[
        "running 2 tests",
        "test a ... ok",
        "",
        "test b ... ok",
        "",
        SUMMARY_OK_2,
    ]);
    assert_eq!(
        parse(&early, 0, PIN),
        Err(ParseRefusal::RecordCountMismatch {
            line: 1,
            header: 2,
            records: 1
        })
    );

    // (d) the control: a FOREIGN line in the region is skipped, never counted, and tallied with its
    // position, so a header that agrees with the records still parses.
    let foreign = lines(&[
        "running 1 test",
        "RK-D-X ok",
        "test a ... ok",
        "",
        SUMMARY_OK_1,
    ]);
    assert_eq!(
        parse(&foreign, 0, PIN),
        Ok(Tally {
            passed: 1,
            failed: 0,
            ignored: 0,
            blocks: 1,
            foreign_lines: 1,
            first_foreign: Some(ForeignLine {
                line: 2,
                text: "RK-D-X ok".to_owned()
            }),
        })
    );
}

#[test]
fn a_summary_that_disagrees_with_its_records_is_unverifiable() {
    let failed_1 = "test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s";

    // (a) two ok records under a summary claiming one failure.
    let s = lines(&[
        "running 2 tests",
        "test a ... ok",
        "test b ... ok",
        "",
        failed_1,
    ]);
    assert_eq!(
        parse(&s, 101, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 5 })
    );

    // (b) D21-62 A test 3: a leaked fragment corrupts the FAILED record into a foreign line, and a
    // leaked ok record balances the header. The summary still names the failure: rule 3 refuses.
    let s = lines(&[
        "running 2 tests",
        "test a ... ok",
        "test b ... FAILRK-D-X okED",
        "test x ... ok",
        "",
        "failures:",
        "    b",
        "",
        failed_1,
    ]);
    assert_eq!(
        parse(&s, 101, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 9 })
    );

    // (c) the ignored member is compared too.
    let s = lines(&[
        "running 1 test",
        "test a ... ignored",
        "",
        "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s",
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 4 })
    );

    // (d) the verdict word is compared: `ok` over a failed record.
    let s = lines(&[
        "running 1 test",
        "test a ... FAILED",
        "",
        "test result: ok. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s",
    ]);
    assert_eq!(
        parse(&s, 101, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 4 })
    );

    // (f) *(added 2026-09-26, s8c refuting round 1)* `passed` ALONE disagrees: the verdict, `failed`
    // and `ignored` all agree, so only the `passed` conjunct can refuse. (a) moved three members at
    // once and could not tell a deleted conjunct from a kept one.
    let s = lines(&[
        "running 2 tests",
        "test a ... ok",
        "test b ... ok",
        "",
        "test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s",
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 5 })
    );

    // (g) *(added 2026-09-26, s8c refuting round 1)* `failed` ALONE disagrees: one failed record, a
    // FAILED summary claiming two, `passed` and `ignored` equal, so only the `failed` conjunct refuses.
    let s = lines(&[
        "running 2 tests",
        "test a ... ok",
        "test b ... FAILED",
        "",
        "test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s",
    ]);
    assert_eq!(
        parse(&s, 101, PIN),
        Err(ParseRefusal::SummaryDisagrees { line: 5 })
    );

    // (e) a summary without a header, and a header without a summary.
    assert_eq!(
        parse(&lines(&["", SUMMARY_OK_1]), 0, PIN),
        Err(ParseRefusal::SummaryWithoutHeader { line: 2 })
    );
    assert_eq!(
        parse(&lines(&["running 1 test", "test a ... ok", ""]), 0, PIN),
        Err(ParseRefusal::HeaderWithoutSummary { line: 1 })
    );
    assert_eq!(
        parse(
            &lines(&[
                "running 1 test",
                "test a ... ok",
                "",
                "running 1 test",
                "test b ... ok",
                "",
                SUMMARY_OK_1
            ]),
            0,
            PIN
        ),
        Err(ParseRefusal::HeaderWithoutSummary { line: 1 })
    );
}

#[test]
fn a_nonzero_exit_with_no_failed_record_is_unverifiable_never_green() {
    let green = lines(&["running 1 test", "test a ... ok", "", SUMMARY_OK_1]);
    // (a) a green block under a non-zero exit: something failed that no record names (a
    // `harness = false` target has no block, only its exit code speaks).
    assert_eq!(
        parse(&green, 101, PIN),
        Err(ParseRefusal::UnattributedFailure { exit_code: 101 })
    );
    // (b) no block at all under a non-zero exit.
    assert_eq!(
        parse("", 101, PIN),
        Err(ParseRefusal::UnattributedFailure { exit_code: 101 })
    );
    // (c) a zero exit over a failed record.
    let red = lines(&[
        "running 1 test",
        "test a ... FAILED",
        "",
        "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s",
    ]);
    assert_eq!(
        parse(&red, 0, PIN),
        Err(ParseRefusal::ExitDisagrees {
            exit_code: 0,
            failed: 1
        })
    );
    // (d) the control: the same red block under the exit libtest gives it is a red suite, Ok.
    assert_eq!(
        parse(&red, 101, PIN).map(|t| (t.passed, t.failed)),
        Ok((0, 1))
    );
    let t = parse(&read(&golden_dir().join("project-red.stdout")), 101, PIN).expect("project-red");
    assert_eq!((t.passed, t.failed), (0, 1));
}

#[test]
fn an_unpinned_toolchain_version_is_refused_by_name() {
    let green = read(&golden_dir().join("project-green.stdout"));
    for v in ["1.99.0", "1.98", "1.98.0", ""] {
        let r = parse(&green, 0, v);
        assert_eq!(
            r,
            Err(ParseRefusal::Unsupported {
                capability: format!("libtest-lines@{v}")
            }),
            "version {v:?}"
        );
        let msg = r.expect_err("refused").to_string();
        assert!(
            msg.contains(&format!("libtest-lines@{v}")),
            "the refusal names the version: {msg}"
        );
    }
    // The control: the pinned version parses the same stream.
    assert!(parse(&green, 0, PIN).is_ok());
}

#[test]
fn this_repositorys_whole_suite_stdout_parses_to_its_recorded_tally() {
    let path = golden_dir().join("this-repository.stdout");
    let stdout = read(&path);
    let (v, exit) = expected("this-repository");
    assert_eq!(exit, 0, "captured on a green run");

    // An independent byte-level count of the leaked lines (D21-62 A): whole lines starting `RK-D-`.
    let bytes = std::fs::read(&path).expect("golden bytes");
    assert_eq!(
        bytes.iter().filter(|b| **b == b'\r').count(),
        0,
        "committed LF (ADR-0164)"
    );
    let leaked: Vec<usize> = stdout
        .split('\n')
        .enumerate()
        .filter(|(_, l)| l.starts_with("RK-D-"))
        .map(|(i, _)| i + 1)
        .collect();
    assert_eq!(leaked.len(), 39);
    assert_eq!(leaked.first(), Some(&287));

    let t = parse(&stdout, exit, PIN)
        .unwrap_or_else(|e| panic!("§8c.1: the parser is wrong, not the golden: {e}"));
    assert_eq!((t.passed, t.failed, t.ignored, t.blocks), (784, 0, 9, 44));
    assert_eq!(
        (t.passed, t.failed, t.ignored, t.blocks),
        (
            n(&v, "passed"),
            n(&v, "failed"),
            n(&v, "ignored"),
            n(&v, "blocks")
        )
    );
    assert_eq!(t.foreign_lines, 39);
    assert_eq!(t.foreign_lines, n(&v, "foreign_lines"));
    assert_eq!(n(&v, "first_foreign_line"), 287);
    assert_eq!(
        t.first_foreign,
        Some(ForeignLine {
            line: 287,
            text: "RK-D-GATE-OUTPUTS ok".to_owned()
        })
    );
}

#[test]
fn a_slow_test_notice_inside_a_block_is_skipped_never_refused() {
    // (a) libtest's own notice for a test past 60 s, inside the region, naming a test the block
    // records: skipped, and neither counted nor tallied as foreign.
    let s = lines(&[
        "running 2 tests",
        "test slow_one has been running for over 60 seconds",
        "test fast ... ok",
        "test slow_one ... ok",
        "",
        SUMMARY_OK_2,
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Ok(Tally {
            passed: 2,
            failed: 0,
            ignored: 0,
            blocks: 1,
            foreign_lines: 0,
            first_foreign: None
        })
    );

    // (b) D21-62 A test 5: a notice naming a test the block never records refuses.
    let s = lines(&[
        "running 1 test",
        "test ghost has been running for over 60 seconds",
        "test fast ... ok",
        "",
        SUMMARY_OK_1,
    ]);
    assert_eq!(
        parse(&s, 0, PIN),
        Err(ParseRefusal::SlowNoticeUnrecorded {
            line: 2,
            name: "ghost".to_owned()
        })
    );
}
