//! Shared support for the Rust adapter's suite: the fixture project's path (compile-time
//! `env!("CARGO_MANIFEST_DIR")`, never `current_dir`), the wrapper bin's path (compile-time
//! `env!("CARGO_BIN_EXE_rk-argv-wrapper-rust")`), the ONE process-environment read (the module
//! doc of `main.rs`), and the `Config` builder every test goes through.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rk_adapter_toolchain_rust::{Config, ForcedRunner, RunnerPolicy, RustToolchain};
use rk_ports::toolchain::{Selection, StepBound, TestOutcome, Toolchain};
use rk_types::error::StepError;
use rk_types::verify::BasisReason;

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
///
/// *(Amended 2026-09-26, s8c §8c.1, D21-62 B E: every Config built here pins
/// `Force(cargo test)`. cargo-nextest is installed on this box from `s8c` on, and the fixture
/// project's `zz_decoy` (`harness = false`) does not answer nextest's `--list`, so under `Detect`
/// every count test over it would read as Unverifiable (`ListFailed`). The nextest path and the
/// probe are proved by `runner::` and the `report::the_nextest_path_…` tests, which build their own
/// `Config`.)*
pub fn config_with(
    program: PathBuf,
    timeout: Duration,
    root: &Path,
    env: BTreeMap<String, String>,
    scratch: &Path,
) -> Config {
    Config::new(program, timeout, root, env, scratch.to_path_buf())
        .expect("a Config over the fixture project constructs")
        .with_runner(RunnerPolicy::Force(ForcedRunner::CargoTest))
}

/// The workspace fixture (s8c §8c.3): `a` <- `b` <- `c`, one doctest in `a`, one red test in `a`'s
/// unit-test binary reachable only by selection.
pub fn fixture_workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/workspace")
}

/// The adapter over `root` with the default program, the minimal map and the given runner policy.
pub fn adapter_over(root: &Path, policy: RunnerPolicy, scratch: &Path) -> RustToolchain {
    RustToolchain::new(
        Config::new(
            PathBuf::from(rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
            GENEROUS,
            root,
            minimal_env(),
            scratch.to_path_buf(),
        )
        .expect("a Config over the fixture constructs")
        .with_runner(policy),
    )
    .expect("the shipped registry loads")
}

/// The whole suite, as the conformance body and the unselected tests ask for it.
pub const fn whole() -> Selection {
    Selection::WholeSuite {
        reason: BasisReason::ProbeUnverified,
    }
}

/// A libtest-argument selection.
pub fn filter(args: &[&str]) -> Selection {
    Selection::Filter(args.iter().map(|s| (*s).to_owned()).collect())
}

/// `tc.test(selection)` with no step bound.
pub fn run_test(tc: &RustToolchain, selection: &Selection) -> Result<TestOutcome, StepError> {
    tc.test(selection, StepBound::Undeclared)
}

/// The toolchain's OWN cargo -- `CARGO`, which cargo sets for every crate it compiles, is the real
/// toolchain binary rustup resolved, beside its `rustc` -- never the `~/.cargo/bin` proxy directory
/// where `cargo-nextest` is installed. Read at COMPILE time, so no process environment is read.
pub fn toolchain_cargo() -> PathBuf {
    PathBuf::from(env!("CARGO"))
}

/// A child map under which `cargo nextest` does not exist (record T decision 4's measured case):
/// the minimal map's `SystemRoot`, `SystemDrive` and temp names, with `PATH` narrowed to the
/// toolchain's own bin directory and the system directory, and `CARGO_HOME` an empty directory.
/// The system directory is `%SystemRoot%\System32` on Windows and `/usr/bin:/bin` on POSIX, where
/// there is no `SystemRoot` and the linker the fixture's build script needs (`cc`) is found only
/// through `PATH`. Neither holds cargo-nextest: `cargo install` writes to `$CARGO_HOME/bin`, and
/// that is the empty directory. *(Amended 2026-10-03: the first `ubuntu-latest` run failed the
/// fallback test with `linker 'cc' not found`, since the narrowed `PATH` had no system directory.)*
pub fn nextest_absent_env(empty_cargo_home: &Path) -> BTreeMap<String, String> {
    let mut env: BTreeMap<String, String> = minimal_env()
        .into_iter()
        .filter(|(k, _)| !k.eq_ignore_ascii_case("PATH"))
        .collect();
    let bin = toolchain_cargo()
        .parent()
        .expect("the toolchain cargo has a parent directory")
        .to_path_buf();
    let system = env
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("SystemRoot"))
        .map(|(_, v)| Path::new(v).join("System32"));
    let mut path = bin.to_string_lossy().into_owned();
    if let Some(system) = system {
        path.push(if cfg!(windows) { ';' } else { ':' });
        path.push_str(&system.to_string_lossy());
    }
    if cfg!(unix) {
        path.push_str(":/usr/bin:/bin");
    }
    env.insert("PATH".to_owned(), path);
    env.insert(
        "CARGO_HOME".to_owned(),
        empty_cargo_home.to_string_lossy().into_owned(),
    );
    env
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

/// A throwaway git repository for `change_summary` (ADR-0165; `s8a` §8a.9): built through
/// `rk-git`'s one wrapper (ADR-0077 -- no `Command` for git exists in this suite), isolated from
/// the operator's configuration the way `rk-git`'s own fixtures are (an empty global config, no
/// system config, `core.autocrlf=false`, no hooks, pinned identity and dates), so the range read
/// under test is a property of the fixture and never of `~/.gitconfig`.
pub struct FixtureRepo {
    dir: tempfile::TempDir,
}

impl FixtureRepo {
    /// `git init` a fresh repository.
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("repo")).expect("mkdir repo");
        std::fs::create_dir_all(dir.path().join("nohooks")).expect("mkdir nohooks");
        std::fs::write(dir.path().join("empty.gitconfig"), b"").expect("write empty gitconfig");
        let repo = Self { dir };
        repo.git(&["init", "--quiet"]);
        repo
    }

    /// The repository's root: the project root the adapter is constructed over.
    pub fn root(&self) -> PathBuf {
        self.dir.path().join("repo")
    }

    /// Run git in the repository, isolated; panics on failure (a fixture step that fails
    /// silently builds a vacuous fixture).
    pub fn git(&self, args: &[&str]) -> String {
        let home = self.dir.path();
        let mut all: Vec<String> = vec![
            "-c".into(),
            "core.autocrlf=false".into(),
            "-c".into(),
            format!("core.hooksPath={}", home.join("nohooks").display()),
            "-c".into(),
            "commit.gpgsign=false".into(),
            "-c".into(),
            "init.defaultBranch=main".into(),
        ];
        all.extend(args.iter().map(|s| (*s).to_owned()));
        let env = [
            ("GIT_CONFIG_NOSYSTEM", "1".to_owned()),
            (
                "GIT_CONFIG_GLOBAL",
                home.join("empty.gitconfig").display().to_string(),
            ),
            ("GIT_AUTHOR_NAME", "rk".to_owned()),
            ("GIT_COMMITTER_NAME", "rk".to_owned()),
            ("GIT_AUTHOR_EMAIL", "rk@example.invalid".to_owned()),
            ("GIT_COMMITTER_EMAIL", "rk@example.invalid".to_owned()),
            ("GIT_AUTHOR_DATE", "1700000000 +0000".to_owned()),
            ("GIT_COMMITTER_DATE", "1700000000 +0000".to_owned()),
        ];
        match rk_git::git_exec::run_with_env(&self.root(), &all, env) {
            Ok(out) => out.stdout_trimmed(),
            Err(e) => panic!("fixture git {args:?} failed: {e:?}"),
        }
    }

    /// Write `bytes` at `path` (relative), creating parents.
    pub fn write(&self, path: &str, bytes: &[u8]) {
        let full = self.root().join(path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).expect("mkdir parents");
        }
        std::fs::write(full, bytes).expect("write fixture file");
    }

    /// Remove `path` (relative).
    pub fn remove(&self, path: &str) {
        std::fs::remove_file(self.root().join(path)).expect("remove fixture file");
    }

    /// Stage everything and commit.
    pub fn commit(&self, message: &str) {
        self.git(&["add", "-A"]);
        self.git(&["commit", "--quiet", "-m", message]);
    }

    /// The adapter over THIS repository as its project root, with the shipped registry.
    pub fn adapter(&self, scratch: &Path) -> RustToolchain {
        RustToolchain::new(self.config(scratch)).expect("the shipped registry loads")
    }

    /// A `Config` over this repository.
    pub fn config(&self, scratch: &Path) -> Config {
        config_with(
            PathBuf::from(rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
            GENEROUS,
            &self.root(),
            minimal_env(),
            scratch,
        )
    }
}

/// A git-lfs pointer blob, as git-lfs writes it.
pub const LFS_POINTER: &[u8] = b"version https://git-lfs.github.com/spec/v1\noid sha256:4d7a214614ab2935c943f9e0ff69d22eadbb8f32b1258daaa5e2ca24d17e2393\nsize 12345\n";

/// A two-commit repository whose second commit touches only text and generated paths: modifies
/// `src/lib.rs` and `Cargo.lock`, adds `notes.md`, deletes `README.md`. *(Amended 2026-09-25, s8a,
/// D21-51: the first commit also carries a `Cargo.toml` the range does not change, so the `Text`
/// arm's git-as-authority case can mutate and delete it in the working tree.)*
pub fn text_range_repo() -> FixtureRepo {
    let repo = FixtureRepo::new();
    repo.write(
        "Cargo.toml",
        b"[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    repo.write("src/lib.rs", b"pub fn one() -> u32 {\n    1\n}\n");
    repo.write("README.md", b"# fixture\n");
    repo.write("Cargo.lock", b"version = 3\n");
    repo.commit("one");
    repo.write("src/lib.rs", b"pub fn one() -> u32 {\n    2\n}\n");
    repo.write("Cargo.lock", b"version = 4\n");
    repo.write("notes.md", b"a note\n");
    repo.remove("README.md");
    repo.commit("two");
    repo
}

/// A two-commit repository whose second commit adds a `.uasset` (bytes, with NULs) beside a text
/// change.
pub fn binary_range_repo() -> FixtureRepo {
    let repo = FixtureRepo::new();
    repo.write("src/lib.rs", b"pub fn one() -> u32 {\n    1\n}\n");
    repo.commit("one");
    repo.write("src/lib.rs", b"pub fn one() -> u32 {\n    2\n}\n");
    repo.write(
        "Content/Hero.uasset",
        b"\xC1\x83\x2A\x9E\x00\x00\x00\x01uasset\x00bytes",
    );
    repo.commit("two");
    repo
}

/// A two-commit repository whose second commit adds `data/table.json` as a git-lfs pointer blob
/// beside an ordinary `data/plain.json`.
pub fn lfs_range_repo() -> FixtureRepo {
    let repo = FixtureRepo::new();
    repo.write("README.md", b"# fixture\n");
    repo.commit("one");
    repo.write("data/table.json", LFS_POINTER);
    repo.write("data/plain.json", b"{\"rows\": [1, 2, 3]}\n");
    repo.commit("two");
    repo
}

/// A two-commit repository whose second commit DELETES an extensionless NUL-bearing blob
/// (`assets/blob`) beside a text change: the working tree no longer holds the binary, so only the
/// range's blobs can say it is one. *(Added 2026-09-25, s8a fix round 1.)*
pub fn deleted_binary_range_repo() -> FixtureRepo {
    let repo = FixtureRepo::new();
    repo.write("a.txt", b"one\n");
    repo.write("assets/blob", b"\x00\x01\x02binary\x00payload");
    repo.commit("one");
    repo.remove("assets/blob");
    repo.write("a.txt", b"two\n");
    repo.commit("two");
    repo
}

/// A two-commit repository whose second commit ADDS an extensionless NUL-bearing `tool`, then
/// checks the BASE commit out: the working tree is not on the head end. Returns the repository and
/// the `<base>..<head>` range. *(Added 2026-09-25, s8a fix round 1.)*
pub fn unchecked_out_binary_range_repo() -> (FixtureRepo, String) {
    let repo = FixtureRepo::new();
    repo.write("a.txt", b"one\n");
    repo.commit("one");
    let base = repo.git(&["rev-parse", "HEAD"]);
    repo.write("tool", b"\x7fELF\x02\x01\x01\x00\x00\x00tool");
    repo.commit("two");
    let head = repo.git(&["rev-parse", "HEAD"]);
    repo.git(&["checkout", "--quiet", &base]);
    (repo, format!("{base}..{head}"))
}

// ---------------------------------------------------------------- s8c P5: verbs, affected, bound
//
// *(Added 2026-09-26, s8c build, P5.)* Helpers for the `build::`, `bound::`, `affected::` and
// `verbs::` modules (s8c §8c.2-§8c.4, §8c.6).

/// The repository this crate sits in (two levels above the crate's manifest).
pub fn repo_root() -> PathBuf {
    // The manifest directory's grandparent, never `canonicalize` (which gives the `\\?\` form git
    // refuses on Windows).
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the crate sits two levels under the repository root")
        .to_path_buf()
}

/// Copy the tree at `src` to `dst` (files and directories; `target/` skipped).
pub fn copy_tree(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("mkdir the copy");
    for entry in std::fs::read_dir(src).expect("the source tree lists") {
        let entry = entry.expect("a readable entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            if entry.file_name() == "target" {
                continue;
            }
            copy_tree(&from, &to);
        } else {
            std::fs::copy(&from, &to).expect("the file copies");
        }
    }
}

/// Run the git CLI in `dir` through `rk-git`'s one exec wrapper, isolated from the operator's
/// configuration the way [`FixtureRepo`] is (its `home` holds the empty config and hooks dir);
/// panics on failure. This is the independent reader the scan and package tests compare the
/// adapter's gix `HEAD` read against (D21-62 C: *"The expected hashes come from an independent
/// `git rev-parse HEAD` via the git CLI"*).
pub fn git_cli(home: &Path, dir: &Path, args: &[&str]) -> String {
    std::fs::create_dir_all(home.join("nohooks")).expect("mkdir nohooks");
    if !home.join("empty.gitconfig").exists() {
        std::fs::write(home.join("empty.gitconfig"), b"").expect("write empty gitconfig");
    }
    let mut all: Vec<String> = vec![
        "-c".into(),
        "core.autocrlf=false".into(),
        "-c".into(),
        format!("core.hooksPath={}", home.join("nohooks").display()),
        "-c".into(),
        "commit.gpgsign=false".into(),
        "-c".into(),
        "init.defaultBranch=main".into(),
    ];
    all.extend(args.iter().map(|s| (*s).to_owned()));
    let env = [
        ("GIT_CONFIG_NOSYSTEM", "1".to_owned()),
        (
            "GIT_CONFIG_GLOBAL",
            home.join("empty.gitconfig").display().to_string(),
        ),
        ("GIT_AUTHOR_NAME", "rk".to_owned()),
        ("GIT_COMMITTER_NAME", "rk".to_owned()),
        ("GIT_AUTHOR_EMAIL", "rk@example.invalid".to_owned()),
        ("GIT_COMMITTER_EMAIL", "rk@example.invalid".to_owned()),
        ("GIT_AUTHOR_DATE", "1700000000 +0000".to_owned()),
        ("GIT_COMMITTER_DATE", "1700000000 +0000".to_owned()),
    ];
    match rk_git::git_exec::run_with_env(dir, &all, env) {
        Ok(out) => out.stdout_trimmed(),
        Err(e) => panic!("git {args:?} in {} failed: {e:?}", dir.display()),
    }
}

/// `git init`, add everything and commit in `dir`, returning `git rev-parse HEAD`. `message`
/// distinguishes two commits of the same tree (the scan's two databases get DIFFERENT commits).
pub fn git_commit_all(home: &Path, dir: &Path, message: &str) -> String {
    git_cli(home, dir, &["init", "--quiet"]);
    git_cli(home, dir, &["add", "-A"]);
    git_cli(
        home,
        dir,
        &["commit", "--quiet", "--allow-empty", "-m", message],
    );
    git_cli(home, dir, &["rev-parse", "HEAD"])
}

/// The scan fixture (s8c §8c.4): the advisory database and the project whose lockfile it matches.
pub fn advisory_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/advisory-db")
}

/// The child directory name `cargo deny 0.20.2` opens under its `db-path` for the default
/// RustSec URL (measured at stage open: `advisory-db-3157b0e258782691`). deny computes the name;
/// the adapter never does (D21-62 C), and the test only has to lay the database out where deny
/// will look.
pub const DENY_DB_CHILD: &str = "advisory-db-3157b0e258782691";

/// Two git-initialised copies of the fixture database with DIFFERENT commits: `audit` (the
/// directory `cargo audit --db` is handed) and `deny_parent`, whose ONLY child is the database
/// `cargo deny` opens. Returns `(audit_db, deny_parent, audit_commit, deny_commit)`.
pub fn scan_databases(tmp: &Path) -> (PathBuf, PathBuf, String, String) {
    let home = tmp.join("gh");
    let audit = tmp.join("audit-db");
    copy_tree(&advisory_fixture().join("db"), &audit);
    let audit_commit = git_commit_all(&home, &audit, "the audit copy");
    let deny_parent = tmp.join("deny-dbs");
    let deny_child = deny_parent.join(DENY_DB_CHILD);
    copy_tree(&advisory_fixture().join("db"), &deny_child);
    let deny_commit = git_commit_all(&home, &deny_child, "the deny copy, a different commit");
    assert_ne!(
        audit_commit, deny_commit,
        "PREMISE: the two databases carry different commits, so a swap is visible"
    );
    (audit, deny_parent, audit_commit, deny_commit)
}

/// `run_verb` with inputs given as one JSON object, and the given bound.
pub fn run_verb_with(
    tc: &RustToolchain,
    verb: &str,
    inputs: &serde_json::Value,
    bound: StepBound,
) -> Result<rk_ports::toolchain::VerbOutput, StepError> {
    let map: BTreeMap<String, serde_json::Value> = inputs
        .as_object()
        .expect("the inputs are one JSON object")
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    tc.run_verb(&rk_ports::toolchain::VerbCall {
        verb,
        inputs: &map,
        bound,
    })
}

/// `run_verb` with no step bound.
pub fn run_verb(
    tc: &RustToolchain,
    verb: &str,
    inputs: &serde_json::Value,
) -> Result<rk_ports::toolchain::VerbOutput, StepError> {
    run_verb_with(tc, verb, inputs, StepBound::Undeclared)
}

/// The `smoke-*` directories left under `scratch` (record V decision 4: removed on every path).
pub fn smoke_dirs(scratch: &Path) -> Vec<String> {
    std::fs::read_dir(scratch)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.starts_with("smoke-"))
                .collect()
        })
        .unwrap_or_default()
}

/// A temporary directory under this repository's git-ignored `target/`, so a cargo run inside it
/// resolves the repository's `rust-toolchain.toml` (the pinned channel the libtest parser lists),
/// never rustup's default: over a crate outside this repository rustup's default here is 1.97.1,
/// which the parser refuses by name (measured at s8c P4).
pub fn repo_tempdir() -> tempfile::TempDir {
    let target = repo_root().join("target");
    std::fs::create_dir_all(&target).expect("the repository target dir");
    tempfile::Builder::new()
        .prefix("s8c-p5-")
        .tempdir_in(&target)
        .expect("a temp dir under target/")
}
