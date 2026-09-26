//! Shared support for the Node adapter's suite: the fixture project's path (compile-time
//! `env!("CARGO_MANIFEST_DIR")`, never `current_dir`), the wrapper bin's path (compile-time
//! `env!("CARGO_BIN_EXE_rk-argv-wrapper-node")`), the ONE process-environment read (the module
//! doc of `main.rs`), the `node_modules` preflight, and the `Config` builder every test goes
//! through.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rk_adapter_toolchain_node::{Config, NodeToolchain};

/// The fixture project (s7c §7c.14, §7c.17): `package.json`, `pnpm-lock.yaml`, one `*.test.ts`
/// with two passing tests (the decoy among them, one filterable by `-t`), a skip-marked failing
/// test and a skip-marked slow test reachable by `-t <name>` only; `node_modules/` git-ignored.
pub fn fixture_project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/project")
}

/// The fixture's one test file, as the selection names it.
pub const SUITE_FILE: &str = "test/suite.test.ts";

/// AC-325's wrapper fixture, this crate's own copy.
pub fn wrapper_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_rk-argv-wrapper-node"))
}

/// The preflight every spawning test runs first: the fixture's `node_modules/.bin/vitest` exists.
/// `npx` without it would fetch from the registry or prompt on a closed stdin until the bound
/// (§7c.14; the block's own preflight line says the same) — so the test refuses, naming the
/// install step, rather than spawning.
pub fn preflight() {
    let vitest = fixture_project().join("node_modules/.bin/vitest");
    assert!(
        vitest.is_file(),
        "run `pnpm install --frozen-lockfile --dir {}` ONCE before this suite (network; \
         local-only) — npx without node_modules prompts on a closed stdin (§7c.14)",
        fixture_project().display()
    );
}

/// The captured map, projected to the names the measured list carries — `SystemRoot` and `PATH`
/// (the crate's `env.rs`, measured 2026-09-18). Read once here, at the edge; nothing else in
/// this suite reads the environment.
pub fn minimal_env() -> BTreeMap<String, String> {
    std::env::vars()
        .filter(|(k, _)| k.eq_ignore_ascii_case("SystemRoot") || k.eq_ignore_ascii_case("PATH"))
        .collect()
}

/// A generous bound for runs that must complete: a fixture run is about two seconds on this
/// box, and a parallel suite of them shares the machine.
pub const GENEROUS: Duration = Duration::from_secs(300);

/// A `Config` over `root` with the given program, bound and map, and `scratch` as the output
/// directory.
pub fn config_with(
    program: PathBuf,
    timeout: Duration,
    root: &Path,
    env: BTreeMap<String, String>,
    scratch: &Path,
) -> Config {
    Config::new(program, timeout, root, env, scratch.to_path_buf())
        .expect("a Config over the fixture project constructs")
}

/// The adapter over the fixture project: default program, the given bound, the minimal map.
pub fn adapter(timeout: Duration, scratch: &Path) -> NodeToolchain {
    preflight();
    NodeToolchain::new(config_with(
        PathBuf::from(rk_adapter_toolchain_node::DEFAULT_PROGRAM),
        timeout,
        &fixture_project(),
        minimal_env(),
        scratch,
    ))
    .expect("the shipped registry loads")
}

/// The adapter over `root` with extra names in its map (the wrapper's `RK_*` names, the marker
/// dir).
pub fn adapter_with_env(
    program: PathBuf,
    timeout: Duration,
    root: &Path,
    extra: &[(&str, String)],
    scratch: &Path,
) -> NodeToolchain {
    preflight();
    let mut env = minimal_env();
    for (k, v) in extra {
        env.insert((*k).to_owned(), v.clone());
    }
    NodeToolchain::new(config_with(program, timeout, root, env, scratch))
        .expect("the shipped registry loads")
}

/// `YYYY-MM-DD`, digits in the right places — a dated entry, not a free string.
pub fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b.iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

/// The vectors an argv-log holds: blocks of lines separated by `---`, the first line of each the
/// wrapper's own path.
pub fn logged_vectors(log: &Path) -> Vec<Vec<String>> {
    let text = std::fs::read_to_string(log).expect("the argv log is readable");
    text.split("---\n")
        .map(|block| block.lines().map(str::to_owned).collect::<Vec<_>>())
        .filter(|v| !v.is_empty())
        .collect()
}

/// Two paths name the same file, compared as the wrapper prints its own path (backslashes,
/// `current_exe`) against the one the test handed the adapter.
pub fn same_path(a: &str, b: &str) -> bool {
    a.replace('\\', "/")
        .eq_ignore_ascii_case(&b.replace('\\', "/"))
}

/// The reporter file a recorded vector names (`--outputFile=<path>`), read back as JSON — the
/// re-execution instrument's count source, the adapter's own.
pub fn reporter_counts(argv: &[String]) -> (u64, u64) {
    let path = argv
        .iter()
        .find_map(|a| a.strip_prefix("--outputFile="))
        .expect("the vector names the reporter file");
    let text = std::fs::read_to_string(path).expect("the reporter file is readable");
    let v: serde_json::Value = serde_json::from_str(&text).expect("the reporter file is JSON");
    (
        v["numPassedTests"].as_u64().expect("numPassedTests"),
        v["numFailedTests"].as_u64().expect("numFailedTests"),
    )
}

/// `tc.test(Filter(selection))` with no step bound: the port's `test`, which the retired inherent
/// `verify_report` was (s8c §8c.2; record T = ADR-0169 decision 2, proposed). *(Added 2026-09-26.)*
pub fn run_test(
    tc: &rk_adapter_toolchain_node::NodeToolchain,
    selection: &[String],
) -> Result<rk_ports::toolchain::TestOutcome, rk_types::error::StepError> {
    use rk_ports::toolchain::Toolchain;
    tc.test(
        &rk_ports::toolchain::Selection::Filter(selection.to_vec()),
        rk_ports::toolchain::StepBound::Undeclared,
    )
}
