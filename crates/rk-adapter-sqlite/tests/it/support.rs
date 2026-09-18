//! Test support: an isolated temporary repository builder and the `GitExec` the adapter takes.
//!
//! **Every `git` call goes through `rk_git::git_exec`** (ADR-0077:70's one wrapper; the
//! ADR-0077 git-spawn grep in Checkpoint 7c polices `src/` and CI's polices every tracked `.rs`,
//! and the tests keep to the same door — the banned token is not spelt here even to deny it,
//! §7c.21's 7b-ii lesson). The fixture steps — `init`, `add`, `commit`, `branch` — run through
//! `run_with_env` with a stripped environment and an explicit `-c` prelude, the way
//! `rk-git`'s own harness does, so this box's `core.autocrlf=true` and `~/.gitconfig` never reach
//! a measurement; the adapter's own two reads (`rev-parse --git-common-dir`, `log
//! --max-parents=0`) and the worktree adds run through the `GitExec` the test mints, exactly as the
//! composition root will hand one in (s7c §7c.13).
//!
//! **`GitExec` is minted here, never constructed by the adapter**: `GitExec::new` demands a
//! `GitToolchain`, which only `rk_git::version::assert_at_startup` mints (it spawns
//! `git --version`), and the tests do that once per fixture over a temporary empty hooks dir.

use std::path::{Path, PathBuf};

use rk_adapter_sqlite::SqliteStore;
use rk_git::error::GitError;
use rk_git::git_exec::{self, GitExec, GitOutput};
use rk_git::path::WorktreePath;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// A temporary directory plus the isolated environment every fixture `git` call uses.
pub struct Fixture {
    dir: tempfile::TempDir,
}

impl Fixture {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("empty.gitconfig"), b"").expect("write empty gitconfig");
        std::fs::create_dir_all(dir.path().join("nohooks")).expect("mkdir nohooks");
        Self { dir }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn sub(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    /// The environment that makes a fixture step reproducible regardless of who runs it.
    fn env(&self) -> Vec<(String, String)> {
        vec![
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            (
                "GIT_CONFIG_GLOBAL".into(),
                self.dir
                    .path()
                    .join("empty.gitconfig")
                    .display()
                    .to_string(),
            ),
            ("GIT_AUTHOR_NAME".into(), "rk".into()),
            ("GIT_COMMITTER_NAME".into(), "rk".into()),
            ("GIT_AUTHOR_EMAIL".into(), "rk@example.invalid".into()),
            ("GIT_COMMITTER_EMAIL".into(), "rk@example.invalid".into()),
            ("GIT_AUTHOR_DATE".into(), "1700000000 +0000".into()),
            ("GIT_COMMITTER_DATE".into(), "1700000000 +0000".into()),
        ]
    }

    fn config_prelude(&self) -> Vec<String> {
        vec![
            "-c".into(),
            "core.autocrlf=false".into(),
            "-c".into(),
            format!(
                "core.hooksPath={}",
                self.dir.path().join("nohooks").display()
            ),
            "-c".into(),
            "commit.gpgsign=false".into(),
            "-c".into(),
            "init.defaultBranch=main".into(),
        ]
    }

    /// Run a fixture `git` step in `cwd`, isolated. Panics on failure — a fixture step that fails
    /// silently is a vacuous test, and these calls build the thing being measured.
    pub fn git(&self, cwd: &Path, args: &[&str]) -> GitOutput {
        let mut all = self.config_prelude();
        all.extend(args.iter().map(|s| (*s).to_string()));
        match git_exec::run_with_env(cwd, &all, self.env()) {
            Ok(o) => o,
            Err(GitError::NonZeroExit { op, code, stderr }) => panic!(
                "git {op} {args:?} in {} exited {code:?}: {}",
                cwd.display(),
                String::from_utf8_lossy(&stderr)
            ),
            Err(other) => panic!("git could not be run: {other:?}"),
        }
    }

    /// `git init` a repository at `<fixture>/<name>` with **one commit** — every `ac369_…` /
    /// `ac370_…` fixture carries at least one, or `log --max-parents=0` prints nothing and the
    /// open refuses for a fixture reason (s7c §7c.13).
    ///
    /// The first commit carries `name` in its content and message: with author, committer and
    /// both dates pinned by [`Fixture::env`], two repositories with byte-identical first commits
    /// would share one root SHA and an identity test over them would measure nothing [observed
    /// 2026-09-18: `ac369_…`'s first form built `a` and `b` from the same bytes, and B *was* A].
    pub fn repo_with_one_commit(&self, name: &str) -> PathBuf {
        let repo = self.sub(name);
        std::fs::create_dir_all(&repo).expect("mkdir repo");
        self.git(&repo, &["init", "--quiet"]);
        self.commit(
            &repo,
            "a.txt",
            format!("{name}: one\n").as_bytes(),
            &format!("{name}: one"),
        );
        repo
    }

    /// Write `file`, stage it and commit it.
    pub fn commit(&self, repo: &Path, file: &str, content: &[u8], message: &str) {
        std::fs::write(repo.join(file), content).expect("write file");
        self.git(repo, &["add", file]);
        self.git(repo, &["commit", "--quiet", "-m", message]);
    }

    /// The `GitExec` the adapter takes: the startup precondition run once, over this fixture's
    /// empty hooks directory.
    pub fn exec(&self) -> GitExec {
        let toolchain =
            rk_git::version::assert_at_startup(self.path()).expect("git is on PATH at the floor");
        GitExec::new(self.sub("nohooks"), toolchain).expect("GitExec::new")
    }
}

/// A worktree path in the form `git` accepts.
pub fn wt(p: &Path) -> WorktreePath {
    WorktreePath::new(p).expect("a drive-letter path")
}

/// Open the store at `repo` through the adapter's one constructor, or panic with the refusal.
pub fn open(exec: &GitExec, repo: &Path) -> SqliteStore {
    SqliteStore::open(exec, &wt(repo)).unwrap_or_else(|e| panic!("open {}: {e}", repo.display()))
}

/// A tier-2 event with a distinguishable payload. `reconcile_started` is the kind the shared
/// conformance body uses; the payload is what a test tells rows apart by.
pub fn event(n: u64) -> Event {
    Event::new(EventKind::ReconcileStarted, serde_json::json!({ "n": n }))
}

/// Every `store.sqlite3` under `dir`, recursively — the instrument for "one store file".
pub fn store_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in rd.filter_map(Result::ok) {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n == "store.sqlite3") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The store's on-disk footprint as bytes: the main file and, when present, its WAL — a write in
/// WAL mode lands in `store.sqlite3-wal` first, so a digest of the main file alone would miss it.
pub fn store_bytes(store_path: &Path) -> (Vec<u8>, Vec<u8>) {
    let main = std::fs::read(store_path).expect("read store");
    let wal = std::fs::read(format!("{}-wal", store_path.display())).unwrap_or_default();
    (main, wal)
}
