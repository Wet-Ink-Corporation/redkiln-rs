//! The one integration binary for `redkiln-migrate` (ADR-0083: one per crate, a `mod` per case).
//!
//! # What exists here, and what does not
//!
//! **Slice 5 cycle 1 landed the crate skeleton and the fixture, and claimed NO test.** Golden
//! constraint 12 is the reason: every Slice 3 and Slice 4 cycle that authored a test before the
//! signatures it describes was redone, because the red it watched was a compile error rather than
//! the absent behaviour. The module list below grows one cycle at a time, and **no empty module
//! stands in for an unbuilt one** — a `mod` that exists and asserts nothing reads as coverage.
//!
//! The case modules arrive: `lift` and `report` at cycle 2; `contract` at cycle 3; `commit` at
//! cycle 4; `idempotence` and `lints` at cycle 5.
//!
//! # The helpers are this crate's own copies, deliberately
//!
//! `rk-cli`'s `tests/it/main.rs` carries the same `rs_files` / `code_lines` / `temp_repo` shapes,
//! and `rk-kernel` carries a third copy. A test binary cannot link another crate's test binary, so
//! the alternative to duplication is a shared dev-dependency crate that no record names. The copies
//! are kept faithful to `rk-cli`'s, which is the version the block's own shell helpers mirror.

mod contract;
mod graph;
mod lift;
mod lints;
mod output;
mod provenance;
mod report;

use std::path::{Path, PathBuf};

// ---- locations -------------------------------------------------------------------------------

/// This crate's `src/`, the scope every source-scan test reads.
pub fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// This crate's `tests/fixtures/`.
pub fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// The synthetic old-layout tree (§7d.13). **Never run against directly** — every test copies it
/// into a temp git repository first, as the checkpoint block does.
pub fn legacy_mini_dir() -> PathBuf {
    fixtures_dir().join("legacy-mini")
}

/// The positive controls for every absence ban — **`rk-cli`'s directory, not this crate's**.
///
/// §7d.13 fixes **one** directory for both crates' bans ("the helper takes one `LC`; a control is a
/// file carrying a token, and which crate's ban it controls is in its name"), and Checkpoint 7d
/// sets `LC=crates/rk-cli/tests/fixtures/lint-controls`. A second directory here would leave the
/// block's `rk_control` lines pointing at the wrong place.
pub fn lint_controls_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("rk-cli")
        .join("tests")
        .join("fixtures")
        .join("lint-controls")
}

/// The `redkiln-migrate` binary, located as cargo hands it to this package's own tests. Never
/// `assert_cmd`'s `cargo_bin`, which guesses.
pub const fn migrate_bin() -> &'static str {
    env!("CARGO_BIN_EXE_redkiln-migrate")
}

// ---- source scanning -------------------------------------------------------------------------

/// Every `.rs` file under `root`, sorted.
///
/// Panics on a missing root **and** on an empty result: an absence assertion over a directory that
/// does not exist, or that holds nothing, reads as a pass having read nothing (Halt 8 shape (a)).
pub fn rs_files(root: &Path) -> Vec<PathBuf> {
    assert!(
        root.is_dir(),
        "the scan root {} does not exist — an absence assertion over a missing directory reads as \
         a pass having read nothing (Halt 8 shape (a))",
        root.display()
    );
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries =
            std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out.sort();
    assert!(
        !out.is_empty(),
        "no .rs file under {} — the scan would read nothing and report a pass",
        root.display()
    );
    out
}

/// The non-comment lines of `src`, as `(1-indexed line number, content)`.
///
/// The same filter the block's `rk_absent` applies (`^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)`), plus
/// a trailing-comment cut the shell form cannot do. Three guards on this tree have passed on their
/// own prose; a scan that counts comments is one of them waiting to happen.
pub fn code_lines(src: &str) -> Vec<(usize, String)> {
    src.lines()
        .enumerate()
        .map(|(i, line)| (i + 1, line))
        .filter(|(_, line)| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .map(|(n, line)| (n, strip_trailing_comment(line)))
        .filter(|(_, line)| !line.trim().is_empty())
        .collect()
}

/// Cut `line` at the first `//` that lies outside a double-quoted string.
fn strip_trailing_comment(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if c == b'\\' {
                escaped = true;
            } else if c == b'"' {
                in_string = false;
            }
        } else if c == b'"' {
            in_string = true;
        } else if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            return line[..i].to_owned();
        }
        i += 1;
    }
    line.to_owned()
}

/// Refuse a region carrying a `/* … */` block comment — [`code_lines`] cannot read one reliably.
pub fn assert_no_block_comment(region: &str, what: &str) {
    assert!(
        !region.contains("/*"),
        "{what} carries a /* block comment — the line-leading comment filter cannot read one \
         reliably, and a scan that silently under-reads its own region is worth nothing. Write the \
         explanation as // lines."
    );
}

// ---- temp repositories (D18-9) -----------------------------------------------------------------

/// **Git in `cwd`, through `rk-git`'s one process-exec site, asserting it succeeded** — the one
/// body `git` and `git_out` share.
///
/// ADR-0077's git rule is workspace-wide, tests included: *"No `std::process::Command` for git
/// exists anywhere else in the workspace"* (Checkpoint 6's workspace-wide git-spawn line), and
/// `git_exec::run` is the route its own doc names, *"Used by the startup probe ... and by
/// fixtures"*. *(dated 2026-09-30, `s8g` build, package D21128-8g; D21-128 A1. The reason this
/// helper's `#[expect(clippy::disallowed_types)]` gave — "the block's src/ ban enforces" —
/// misread the rule; the attribute is gone with the type.)*
fn git_ok(cwd: &Path, args: &[&str]) -> rk_git::git_exec::GitOutput {
    match rk_git::git_exec::run(cwd, args.iter().copied()) {
        Ok(out) => out,
        Err(rk_git::error::GitError::NonZeroExit { code, stderr, .. }) => panic!(
            "git {args:?} failed (exit {code:?}): {}",
            String::from_utf8_lossy(&stderr)
        ),
        Err(e) => panic!("git {args:?} did not run: {e:?}"),
    }
}

/// Run git in `cwd`, asserting it succeeded ([`git_ok`]).
///
/// The **product**'s git goes through the same wrapper, which is what
/// `ac522_worktree_recreated_never_reused` reads as its instrument.
pub fn git(cwd: &Path, args: &[&str]) {
    git_ok(cwd, args);
}

/// Capture git's stdout in `cwd`, trimmed, asserting it succeeded ([`git_ok`]).
pub fn git_out(cwd: &Path, args: &[&str]) -> String {
    String::from_utf8_lossy(&git_ok(cwd, args).stdout)
        .trim()
        .to_owned()
}

/// A temp repository with one commit, and nothing else.
///
/// **Every repository gets a distinct root commit.** `rk-cli`'s copy of this helper carries the
/// measurement that forced it: the same `README`, author and message produce the same commit SHA,
/// so two repositories built in the same second shared one — and `rk_store::identity::check`
/// compares `root_commit` first. Here the reason is narrower but real: a migration branch is named
/// `rk-migrate/<target-HEAD-short>`, so two fixtures with one root commit would collide on the
/// branch name as well.
///
/// The nonce is a process-wide counter, not a clock: `Instant::now` and `SystemTime::now` are on
/// `clippy.toml`'s `disallowed-methods`, and a counter is both sufficient and deterministic.
pub fn temp_repo() -> tempfile::TempDir {
    static NONCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nonce = NONCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = tempfile::tempdir().expect("a temp dir");
    git(dir.path(), &["init", "--quiet", "-b", "main"]);
    git(
        dir.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(dir.path(), &["config", "user.name", "Test"]);
    std::fs::write(
        dir.path().join("README"),
        format!("root {}-{nonce}\n", std::process::id()).as_bytes(),
    )
    .expect("the root file writes");
    git(dir.path(), &["add", "README"]);
    git(dir.path(), &["commit", "--quiet", "-m", "root"]);
    dir
}

/// A temp repository carrying `legacy-mini/` as its second commit — the shape the checkpoint
/// block's `TMP2` builds, and the one every migrator test runs against.
pub fn legacy_mini_repo() -> tempfile::TempDir {
    let dir = temp_repo();
    copy_tree(&legacy_mini_dir(), dir.path());
    git(dir.path(), &["add", "-A"]);
    git(dir.path(), &["commit", "--quiet", "-m", "legacy-mini"]);
    dir
}

/// Copy a directory tree, recursively.
pub fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the destination directory");
    for entry in std::fs::read_dir(from).expect("the source directory") {
        let entry = entry.expect("a directory entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a fixture file copies");
        }
    }
}

/// `git status --porcelain`, sorted.
pub fn porcelain(repo: &Path) -> Vec<String> {
    let mut lines: Vec<String> = git_out(repo, &["status", "--porcelain"])
        .lines()
        .map(str::to_owned)
        .filter(|l| !l.trim().is_empty())
        .collect();
    lines.sort();
    lines
}

/// Every ref name in `repo`, sorted.
pub fn refs(repo: &Path) -> Vec<String> {
    let mut lines: Vec<String> = git_out(repo, &["for-each-ref", "--format=%(refname)"])
        .lines()
        .map(str::to_owned)
        .filter(|l| !l.trim().is_empty())
        .collect();
    lines.sort();
    lines
}
