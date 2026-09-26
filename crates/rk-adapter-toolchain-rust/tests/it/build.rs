//! **`build` returns an outcome** (s8c §8c.2; D21-43 N25; record T = ADR-0169 decision 8,
//! proposed). The Rust adapter runs `cargo build --workspace --message-format=json` and reads
//! cargo's structured stream: `status` is the `build-finished` message's `success`, and `errors`
//! counts the `compiler-message` records at level `error`. So a compile error is
//! `Ok(BuildReport { status: failing, errors >= 1 })`. A non-zero exit with no `build-finished`
//! message is `Err(Transport { Decode })`, and so is a stream that disagrees with its exit code.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::path::Path;

use rk_adapter_toolchain_rust::build::read_build_stream;
use rk_adapter_toolchain_rust::{DEFAULT_PROGRAM, RustToolchain};
use rk_ports::toolchain::{StepBound, Toolchain};
use rk_types::error::{StepError, TransportKind};
use rk_types::verb_output::BuildStatus;

use crate::support::{GENEROUS, adapter, config_with, minimal_env};

/// A one-crate project of its own (its own `[workspace]` root, no dependency, so cargo needs no
/// registry) whose library is `lib`.
fn one_crate(dir: &Path, manifest: &str, lib: &str) {
    std::fs::create_dir_all(dir.join("src")).expect("mkdir src");
    std::fs::write(dir.join("Cargo.toml"), manifest).expect("write the manifest");
    std::fs::write(dir.join("src/lib.rs"), lib).expect("write the library");
}

const MANIFEST: &str = "[package]\nname = \"broken\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n";

/// **A compile error is `Ok` with `status: failing`**, never `Err` (record T decision 8; M15's
/// kill). The control is the fixture project, which builds `passing` with zero errors through the
/// same method.
#[test]
fn a_compile_error_is_ok_with_status_failing() {
    let tmp = crate::support::repo_tempdir();
    let project = tmp.path().join("broken");
    one_crate(
        &project,
        MANIFEST,
        "pub fn one() -> u32 {\n    \"one\"\n}\n\npub fn two() -> u32 {\n    \"two\"\n}\n",
    );
    let scratch = tmp.path().join("scratch");
    let tc = RustToolchain::new(config_with(
        DEFAULT_PROGRAM.into(),
        GENEROUS,
        &project,
        minimal_env(),
        &scratch,
    ))
    .expect("the shipped registry loads");
    let report = tc.build(StepBound::Undeclared).expect(
        "a compile error is Ok(BuildReport { status: failing }), never Err (record T decision 8)",
    );
    assert_eq!(
        report.status,
        BuildStatus::Failing,
        "the build-finished message's success is false: {report:?}"
    );
    assert_eq!(
        report.errors, 2,
        "errors counts the compiler-message records at level error (two mismatched types): {report:?}"
    );
    assert_eq!(
        &report.argv[1..],
        ["build", "--workspace", "--message-format=json"],
        "the recorded vector is ADR-0094 d1's build with the structured stream's flag"
    );
    assert!(
        report.toolchain_version.starts_with("rustc "),
        "the version is rustc -V's answer: {:?}",
        report.toolchain_version
    );

    // The control: the fixture project builds passing, zero errors, through the same method.
    let control_scratch = tempfile::tempdir().expect("scratch");
    let green = adapter(GENEROUS, control_scratch.path())
        .build(StepBound::Undeclared)
        .expect("the fixture project builds");
    assert_eq!(green.status, BuildStatus::Passing, "CONTROL: {green:?}");
    assert_eq!(green.errors, 0, "CONTROL: {green:?}");
}

/// **A stream with no `build-finished` message is `Err(Transport { Decode })`** (record T
/// decision 8). Over the port: a manifest cargo cannot parse exits non-zero before any JSON message.
/// Over the pure reader: a stream that disagrees with its exit code is refused both ways, and so
/// is a zero exit with no `build-finished`.
#[test]
fn a_build_stream_without_build_finished_is_err() {
    let tmp = crate::support::repo_tempdir();
    let project = tmp.path().join("unparseable");
    one_crate(
        &project,
        "[package\nname = \"unparseable\"\n",
        "pub fn one() -> u32 {\n    1\n}\n",
    );
    let scratch = tmp.path().join("scratch");
    let tc = RustToolchain::new(config_with(
        DEFAULT_PROGRAM.into(),
        GENEROUS,
        &project,
        minimal_env(),
        &scratch,
    ))
    .expect("the shipped registry loads");
    match tc.build(StepBound::Undeclared) {
        Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        }) => assert!(
            detail.contains("build-finished"),
            "the refusal names the missing build-finished message: {detail}"
        ),
        other => panic!(
            "a non-zero exit with no build-finished message is Transport {{ Decode }} (record T \
             decision 8), never NonZero and never a report; got {other:?}"
        ),
    }

    let finished =
        |success: bool| format!("{{\"reason\":\"build-finished\",\"success\":{success}}}\n");
    let error =
        "{\"reason\":\"compiler-message\",\"message\":{\"level\":\"error\",\"message\":\"x\"}}\n";
    let warning =
        "{\"reason\":\"compiler-message\",\"message\":{\"level\":\"warning\",\"message\":\"y\"}}\n";

    // The reader's own arms.
    assert_eq!(
        read_build_stream(&finished(true), 0).expect("a green stream reads"),
        (BuildStatus::Passing, 0)
    );
    assert_eq!(
        read_build_stream(&format!("{error}{warning}{error}{}", finished(false)), 101)
            .expect("a red stream reads"),
        (BuildStatus::Failing, 2),
        "two error-level messages, the warning not counted"
    );
    for (stream, code, why) in [
        (finished(true), 101, "success true under a non-zero exit"),
        (finished(false), 0, "success false under exit 0"),
        (String::new(), 0, "exit 0 with no build-finished message"),
        (
            error.to_owned(),
            101,
            "a non-zero exit with no build-finished message",
        ),
        (
            "not json\n".to_owned(),
            101,
            "a line that is not cargo's JSON",
        ),
    ] {
        let refusal = read_build_stream(&stream, code)
            .expect_err(&format!("{why} is unreadable, never a report"));
        assert!(!refusal.is_empty(), "{why}: the refusal says why");
    }
}
