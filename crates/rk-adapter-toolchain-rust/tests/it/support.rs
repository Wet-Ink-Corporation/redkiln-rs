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
