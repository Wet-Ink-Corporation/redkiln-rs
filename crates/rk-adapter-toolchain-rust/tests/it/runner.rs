//! **The runner probe** (s8c §8c.1; D21-48 X15; record T = ADR-0169 decision 4, proposed). At
//! construction, under `Detect`, the adapter probes `<program> nextest --version` under the
//! `Config`'s `child_env`, and runs nextest when the probe exits 0 with a `cargo-nextest <version>`
//! line, `cargo test` otherwise.
//!
//! **The absent case is the REAL cargo, not a stub** (record T's evidence: *"the toolchain's own
//! `cargo` under a `child_env` with no `cargo-nextest` on `PATH` and an empty `CARGO_HOME` answers
//! `cargo nextest --version` with rc 101, empty stdout and "error: no such command: `nextest`" on
//! stderr"*). The toolchain's own cargo is `CARGO` (the binary rustup resolved, beside its `rustc`),
//! never the `~/.cargo/bin` proxy directory `cargo-nextest` is installed into.
//!
//! *(Added 2026-09-26, s8c.)*

use rk_adapter_toolchain_rust::nextest_lines::NEXTEST_PINNED;
use rk_adapter_toolchain_rust::{Config, Runner, RunnerPolicy, RustToolchain};

use crate::support::{GENEROUS, fixture_project, minimal_env, nextest_absent_env, toolchain_cargo};

/// **The probe finds the installed nextest and names its version** (the presence arm s7c §7c.22
/// held ARMED). Under `Detect` over the minimal map, whose `PATH` reaches `cargo-nextest` (installed
/// at stage open, §8c.8 step 0), the adapter chooses nextest at the pinned release.
#[test]
fn the_probe_finds_the_installed_nextest_and_names_its_version() {
    let scratch = tempfile::tempdir().expect("scratch");
    let config = Config::new(
        rk_adapter_toolchain_rust::DEFAULT_PROGRAM.into(),
        GENEROUS,
        &fixture_project(),
        minimal_env(),
        scratch.path().to_path_buf(),
    )
    .expect("a Config constructs");
    assert_eq!(
        config.runner(),
        RunnerPolicy::Detect,
        "PREMISE: Detect is the default policy, the one real_toolchain uses"
    );
    let tc = RustToolchain::new(config).expect("the shipped registry loads");
    match tc.runner() {
        Runner::Nextest { version } => assert!(
            NEXTEST_PINNED.contains(&version.as_str()),
            "the probe named cargo-nextest {version}, which NEXTEST_PINNED ({NEXTEST_PINNED:?}) \
             must list: the stage-open install is the pin (D21-48 X15)"
        ),
        other => panic!(
            "cargo-nextest is installed on this box (s8c §8c.8 step 0), and the probe under the \
             minimal map chose {other:?}: the adapter always runs cargo test"
        ),
    }
}

/// **The probe falls back to `cargo test` where nextest is absent** -- over the toolchain's own
/// cargo, under a `child_env` with no `cargo-nextest` on `PATH` or under `CARGO_HOME`. The control
/// is the SAME program under the minimal map, which does reach nextest, so the fallback below is
/// the child map's doing and nothing else's.
#[test]
fn the_probe_falls_back_to_cargo_test_where_nextest_is_absent() {
    let scratch = tempfile::tempdir().expect("scratch");
    let control = RustToolchain::new(
        Config::new(
            toolchain_cargo(),
            GENEROUS,
            &fixture_project(),
            minimal_env(),
            scratch.path().join("control"),
        )
        .expect("a Config constructs"),
    )
    .expect("loads");
    assert!(
        matches!(control.runner(), Runner::Nextest { .. }),
        "CONTROL: the toolchain's own cargo under the minimal map reaches cargo-nextest, so the \
         fallback below is the child map's: {:?}",
        control.runner()
    );

    let cargo_home = tempfile::tempdir().expect("an empty CARGO_HOME");
    let tc = RustToolchain::new(
        Config::new(
            toolchain_cargo(),
            GENEROUS,
            &fixture_project(),
            nextest_absent_env(cargo_home.path()),
            scratch.path().join("absent"),
        )
        .expect("a Config constructs"),
    )
    .expect("loads");
    assert_eq!(
        tc.runner(),
        &Runner::CargoTest,
        "with no cargo-nextest on PATH or under CARGO_HOME, Detect chooses cargo test"
    );
}
