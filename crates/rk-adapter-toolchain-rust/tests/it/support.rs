//! Shared support for the Rust adapter's suite: the fixture project's path (compile-time
//! `env!("CARGO_MANIFEST_DIR")`, never `current_dir`), the wrapper bin's path (compile-time
//! `env!("CARGO_BIN_EXE_rk-argv-wrapper-rust")`), the ONE process-environment read (the module
//! doc of `main.rs`), and the `Config` builder every test goes through.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rk_adapter_toolchain_rust::{Config, RustToolchain};

/// The fixture project (s7c §7c.14, §7c.17): its own workspace root, two passing tests, the
/// `build.rs` decoy, the two `#[ignore]` tests reachable by selection only.
pub fn fixture_project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/project")
}

/// The Rust technology pack fixture (§7c.14).
pub fn pack_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pack/pack.toml")
}

/// AC-325's wrapper fixture, this crate's own copy.
pub fn wrapper_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_rk-argv-wrapper-rust"))
}

/// The captured map, projected to the names the measured lists carry — `SystemDrive` and the
/// temp trio for `cargo` (the crate's `env.rs`, measured 2026-09-18), `SystemRoot` and `PATH`
/// for the Node adapter's launcher (harmless here; `PATH` is what lets the wrapper fixture
/// resolve `cargo` by name). Read once here, at the edge; nothing else in this suite reads the
/// environment.
pub fn minimal_env() -> BTreeMap<String, String> {
    const NAMES: [&str; 6] = [
        "SystemRoot",
        "SystemDrive",
        "PATH",
        "TMP",
        "TEMP",
        "USERPROFILE",
    ];
    std::env::vars()
        .filter(|(k, _)| NAMES.iter().any(|n| n.eq_ignore_ascii_case(k)))
        .collect()
}

/// A generous bound for runs that must complete: the fixture compiles in about two seconds on
/// this box, and a parallel suite of them shares the machine.
pub const GENEROUS: Duration = Duration::from_secs(300);

/// A `Config` over the fixture project with the default program, the given bound and map, and
/// `scratch` as the output / target directory.
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

/// The adapter over the fixture project: default program, the given bound, the minimal map, the
/// given scratch directory.
pub fn adapter(timeout: Duration, scratch: &Path) -> RustToolchain {
    RustToolchain::new(config_with(
        PathBuf::from(rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
        timeout,
        &fixture_project(),
        minimal_env(),
        scratch,
    ))
    .expect("the shipped registry loads")
}

/// The adapter with extra names in its map (the wrapper's `RK_*` names, the marker dir).
pub fn adapter_with_env(
    program: PathBuf,
    timeout: Duration,
    extra: &[(&str, String)],
    scratch: &Path,
) -> RustToolchain {
    let mut env = minimal_env();
    for (k, v) in extra {
        env.insert((*k).to_owned(), v.clone());
    }
    RustToolchain::new(config_with(
        program,
        timeout,
        &fixture_project(),
        env,
        scratch,
    ))
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
