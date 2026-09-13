//! Shared harness for the three S-GIX arms.
//!
//! **Isolation is part of the instrument, not housekeeping.** This box has `core.autocrlf=true`
//! set globally and carries no `.gitattributes` [measured 2026-09-13]. Without isolation, arm b's
//! CRLF case measures the operator's `autocrlf` rather than trailer handling, and arm b's
//! duplicate-token case measures the operator's `trailer.ifexists` rather than git's default —
//! i.e. an arm whose result is a property of `~/.gitconfig` and not of gix-versus-git.
//!
//! Every invocation therefore goes out with a stripped environment and an explicit `-c` prelude,
//! through `rk_git::git_exec`, because that is the only site in the workspace allowed to spawn a
//! child (ADR-0077:70).

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use rk_git::git_exec::{self, GitError, GitOutput};

/// A temporary directory plus the isolated environment every `git` call in these tests uses.
pub struct Fixture {
    dir: tempfile::TempDir,
}

impl Fixture {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        // An empty global config file, so `GIT_CONFIG_GLOBAL` points at something that exists and
        // is definitively empty rather than at the operator's ~/.gitconfig.
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

    /// The environment that makes a run reproducible regardless of who is running it.
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

    /// The `-c` prelude pinned on every invocation. `autocrlf=false` and `hooksPath` are the two
    /// that would otherwise let this box's configuration reach into a measurement.
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

    /// Run `git` in `cwd`, isolated. Panics on failure — a fixture step that fails silently is a
    /// vacuous arm, and these calls build the thing being measured.
    pub fn git(&self, cwd: &Path, args: &[&str]) -> GitOutput {
        match self.try_git(cwd, args) {
            Ok(o) => o,
            Err(GitError::NonZeroExit { code, stderr }) => panic!(
                "git {:?} in {} exited {:?}: {}",
                args,
                cwd.display(),
                code,
                String::from_utf8_lossy(&stderr)
            ),
            Err(GitError::Spawn { detail }) => panic!("git could not be spawned: {detail}"),
        }
    }

    /// As [`Fixture::git`], returning the error instead of panicking.
    pub fn try_git(&self, cwd: &Path, args: &[&str]) -> Result<GitOutput, GitError> {
        let mut all = self.config_prelude();
        all.extend(args.iter().map(|s| (*s).to_string()));
        git_exec::run_with_env(cwd, &all, self.env())
    }

    /// `git init` a repository at `<fixture>/<name>` with one commit, and return its path.
    pub fn repo_with_one_commit(&self, name: &str) -> PathBuf {
        let repo = self.sub(name);
        std::fs::create_dir_all(&repo).expect("mkdir repo");
        self.git(&repo, &["init", "--quiet"]);
        std::fs::write(repo.join("a.txt"), b"one\n").expect("write a.txt");
        self.git(&repo, &["add", "a.txt"]);
        self.git(&repo, &["commit", "--quiet", "-m", "one"]);
        repo
    }
}

/// The verdict ADR-0091 records for one S-GIX arm, read out of the record itself.
///
/// This exists because Checkpoint 6's closing block claims the arms' verdicts "match the record"
/// while comparing nothing — it echoes on both branches and cannot fail (Halt 8 shape (f)). Each
/// arm therefore computes its own verdict and asserts it equals this, so a green run *means*
/// agreement, go or no-go alike, and flipping a line in the record turns a test red.
///
/// Returns `None` only when the record does not exist yet — the bootstrap ordering, where the arms
/// run first precisely so that their results can be written down.
pub fn recorded_verdict(arm: char) -> Option<String> {
    let adr_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../references/adr");
    let entry = std::fs::read_dir(&adr_dir)
        .ok()?
        .filter_map(Result::ok)
        .find(|e| e.file_name().to_string_lossy().starts_with("0091-"))?;
    let text = std::fs::read_to_string(entry.path()).ok()?;
    let needle = format!("S-GIX/{arm}: ");
    text.lines()
        .find_map(|l| l.strip_prefix(&needle))
        .map(|v| v.trim().to_string())
}

/// Assert the verdict this arm just measured is the verdict the record carries.
pub fn assert_agrees_with_record(arm: char, measured: &str) {
    println!("S-GIX/{arm}: {measured}");
    let recorded = recorded_verdict(arm).unwrap_or_else(|| {
        panic!(
            "S-GIX/{arm} measured `{measured}`, but no references/adr/0091-*.md carries an \
             `S-GIX/{arm}: <verdict>` line. Write the record; a measured arm with nowhere to be \
             recorded is not a result."
        )
    });
    assert_eq!(
        measured, recorded,
        "S-GIX/{arm}: measured `{measured}` but ADR-0091 records `{recorded}`"
    );
}

/// Normalise a filesystem path for comparison between gix and the `git` executable.
///
/// The two disagree about spelling, not about identity: `std::fs::canonicalize` on Windows yields
/// the extended-length `\\?\D:\...` form, git yields a drive-letter path, and either may use
/// forward or back slashes. Comparing the raw strings would report a difference that is not one.
///
/// What it must NOT do is collapse a real distinction — `main/.git` and
/// `main/.git/worktrees/linked` are different answers, and an over-eager normaliser that made them
/// compare equal would turn arm a into a test that cannot fail.
pub fn normalise_path(p: &str) -> String {
    let mut s = p.trim();
    // The extended-length prefix, in both spellings. UNC (`\\?\UNC\server\share`) is deliberately
    // NOT unwrapped: stripping it yields `UNC\server\share`, which is a different, broken path —
    // the F20/2 shape. Leave it alone so it compares unequal and the arm reports a difference.
    for pfx in [r"\\?\", "//?/"] {
        if let Some(rest) = s.strip_prefix(pfx)
            && !rest.starts_with("UNC\\")
            && !rest.starts_with("UNC/")
        {
            s = rest;
            break;
        }
    }
    let slashed = s.replace('\\', "/");

    // Resolve `.` and `..` LEXICALLY. gix answers the common-dir question with an unresolved
    // traversal (`<main>/.git/worktrees/<name>/../..`) where the `git` executable answers with the
    // resolved path; `realpath` and inode identity confirm the two name the same directory
    // [measured 2026-09-13]. That is a difference of spelling, which is exactly what this function
    // exists to absorb — refusing it would be a comparator reporting a difference that is not one.
    //
    // It does not collapse the distinction the arm is about: `<main>/.git` and
    // `<main>/.git/worktrees/<name>` have no traversal between them and stay different, which is
    // what the third control pins.
    let leading_slashes: String = slashed.chars().take_while(|c| *c == '/').collect();
    let mut parts: Vec<&str> = Vec::new();
    for seg in slashed.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                // Only pop a segment that can be popped; never climb above a drive or a root.
                let poppable = parts
                    .last()
                    .is_some_and(|p: &&str| *p != ".." && !p.ends_with(':'));
                if poppable {
                    parts.pop();
                } else {
                    parts.push("..");
                }
            }
            other => parts.push(other),
        }
    }
    let mut out = format!("{leading_slashes}{}", parts.join("/"));
    while out.len() > 1 && out.ends_with('/') {
        out.pop();
    }
    // Windows paths are case-insensitive; the drive letter in particular varies between `git`
    // and gix. Case-folding is safe here and nowhere near a real distinction — `.git` and
    // `.git/worktrees/<name>` differ in structure, not in case.
    if cfg!(windows) {
        out = out.to_lowercase();
    }
    out
}
