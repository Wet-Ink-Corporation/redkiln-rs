//! S0.1 — gate false-positive rate. Zero model cost.
//!
//! This is mutation testing (PRIOR-ART §5) applied to Redkiln's own gate. For
//! each fix-lane item: revert the fix's **code** while keeping its **tests**,
//! then run the gate. A mutant the gate passes is a survivor — a known-bad
//! change the oracle waved through.
//!
//! It matters because that oracle is what E4 and E5 both rest on. If the gate
//! passes known-bad code at a material rate, E4's decision rule is unsound and
//! its result is uninterpretable, so this runs before either of them.
//!
//! **Why revert-the-fix-keep-the-test is the preferred mutation source:** the
//! retained test proves the mutant is not semantically equivalent to correct
//! behaviour. Equivalent mutants cannot be detected and must be excluded from
//! the denominator; this construction avoids manufacturing them at all.

use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use serde::Serialize;

/// What a file is, for mutation purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FileRole {
    /// Production code. Reverting this is the mutation.
    Code,
    /// A test. Kept at the fixed version — it is the proof of non-equivalence.
    Test,
    /// Docs, changelog, backlog artifacts. Reverting these creates no defect,
    /// so touching them would only add noise.
    Other,
}

pub fn classify(path: &str) -> FileRole {
    let p = path.replace('\\', "/");
    if p.starts_with("test/") || p.contains(".test.") || p.contains(".dod.") {
        return FileRole::Test;
    }
    if p.starts_with("src/") || p.starts_with("scripts/") || p.starts_with("bin/") {
        return FileRole::Code;
    }
    FileRole::Other
}

#[derive(Debug, Clone, Serialize)]
pub struct GateRecord {
    pub experiment: &'static str,
    pub slug: String,
    pub condition: &'static str,
    pub checkpoint_sha: String,

    pub code_files: Vec<String>,
    pub test_files: Vec<String>,
    /// Files actually reverted. Empty on CLEAN.
    pub reverted: Vec<String>,

    /// Exit code of the gate command.
    pub exit_code: Option<i32>,
    /// Which stage failed first, if any: typecheck / lint / test.
    ///
    /// A mutant killed at `typecheck` says much less than one killed at `test`:
    /// a partial revert can leave code that simply does not compile against the
    /// retained test, which is an incoherent mutant rather than evidence about
    /// the suite's power. Reported separately for that reason.
    pub failed_stage: Option<String>,
    /// Stages that actually spawned. Empty means the gate never ran, and the
    /// verdict is void — see the note on `run_gate`.
    pub stages_run: Vec<String>,
    /// **The finding.** On a mutant, `true` means the gate passed known-bad
    /// code — a survivor.
    pub gate_passed: bool,
    pub survived: bool,

    pub duration_ms: u128,
    pub stdout_tail: String,
    pub stderr_tail: String,
    pub outcome: String,
    pub skipped_reason: Option<String>,
}

fn git(dir: &Path, args: &[&str]) -> Result<std::process::Output> {
    std::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .with_context(|| format!("git {args:?} in {}", dir.display()))
}

/// Return the worktree to the fix commit, discarding any mutation.
///
/// `node_modules` is excluded from the clean: it is a junction into the main
/// repo's install, and removing it would cost a 125 MB reinstall per item.
pub fn reset_worktree(wt: &Path, sha: &str) -> Result<()> {
    git(wt, &["checkout", "--detach", "--force", sha])?;
    git(wt, &["reset", "--hard", sha])?;
    git(wt, &["clean", "-fd", "-e", "node_modules"])?;
    Ok(())
}

/// Apply the mutation: restore each code file to its pre-fix state, leaving
/// tests at the fixed version.
///
/// A file the fix *added* has no parent version, so it is deleted rather than
/// checked out — otherwise the mutation would silently be a no-op for exactly
/// the fixes that introduced new modules.
pub fn apply_mutation(wt: &Path, sha: &str, code_files: &[String]) -> Result<Vec<String>> {
    let parent = format!("{sha}^");
    let mut reverted = Vec::new();
    for f in code_files {
        let existed = git(wt, &["cat-file", "-e", &format!("{parent}:{f}")])
            .map(|o| o.status.success())
            .unwrap_or(false);
        if existed {
            let out = git(wt, &["checkout", &parent, "--", f])?;
            if out.status.success() {
                reverted.push(f.clone());
            }
        } else {
            let path = wt.join(f);
            if path.exists() && std::fs::remove_file(&path).is_ok() {
                reverted.push(format!("{f} (deleted; added by the fix)"));
            }
        }
    }
    Ok(reverted)
}

/// The stages of `affected_gate`, run in order and short-circuiting like `&&`.
///
/// This is `.redkiln/config.yaml`'s declared gate verbatim. It carries no
/// `{{changed}}` placeholder, so the gate is the WHOLE suite every time — which
/// is why S0.1 costs wall clock rather than headroom.
///
/// **Not** dispatched through a shell. The first version of this ran
/// `bash -lc "pnpm typecheck && pnpm lint && pnpm test"`, which on Windows
/// resolved to WSL's bash and failed instantly with
/// `execvpe(/bin/bash) failed`. Every mutant then "died" in 4 seconds having
/// never been tested, producing a **100% mutation score that was entirely
/// fake**. Running the stages directly removes the shell dependency and gives
/// exact attribution instead of regex-guessing which link broke.
pub const GATE_STAGES: [&str; 3] = ["typecheck", "lint", "test"];

#[derive(Debug)]
pub struct GateRun {
    pub exit_code: Option<i32>,
    pub failed_stage: Option<String>,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
    /// Every stage that actually spawned. If this is empty the gate did not
    /// run at all, and any verdict from it is meaningless.
    pub stages_run: Vec<String>,
}

pub fn run_gate(wt: &Path) -> Result<GateRun> {
    let clock = Instant::now();
    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut stages_run = Vec::new();

    for stage in GATE_STAGES {
        // `cmd /c` so `pnpm` resolves to pnpm.CMD on Windows.
        let out = std::process::Command::new("cmd")
            .args(["/c", "pnpm", stage])
            .current_dir(wt)
            // A cached verdict from a previous condition would report the wrong
            // tree's result.
            .env("REDKILN_NO_GATE_CACHE", "1")
            .env("CI", "1")
            .output()
            .with_context(|| format!("spawning `pnpm {stage}`"))?;

        stages_run.push(stage.to_string());
        stdout.push_str(&format!("\n===== pnpm {stage} =====\n"));
        stdout.push_str(&String::from_utf8_lossy(&out.stdout));
        stderr.push_str(&String::from_utf8_lossy(&out.stderr));

        if !out.status.success() {
            return Ok(GateRun {
                exit_code: out.status.code(),
                failed_stage: Some(stage.to_string()),
                stdout: tail(&stdout, 6000),
                stderr: tail(&stderr, 3000),
                duration_ms: clock.elapsed().as_millis(),
                stages_run,
            });
        }
    }

    Ok(GateRun {
        exit_code: Some(0),
        failed_stage: None,
        stdout: tail(&stdout, 6000),
        stderr: tail(&stderr, 3000),
        duration_ms: clock.elapsed().as_millis(),
        stages_run,
    })
}

fn tail(s: &str, n: usize) -> String {
    let s = s.trim_end();
    if s.len() <= n {
        return s.to_string();
    }
    s.char_indices()
        .nth(s.chars().count().saturating_sub(n))
        .map(|(i, _)| s[i..].to_string())
        .unwrap_or_else(|| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_are_classified_before_code() {
        // A test living under src/ must still count as a test, or the mutation
        // would revert the very file that proves non-equivalence.
        assert_eq!(classify("test/advance-git-commit.test.ts"), FileRole::Test);
        assert_eq!(classify("src/store/verify.test.ts"), FileRole::Test);
        assert_eq!(classify("test/dod/scenarios.dod.test.ts"), FileRole::Test);
    }

    #[test]
    fn production_code_is_code() {
        assert_eq!(classify("src/cli/advance.ts"), FileRole::Code);
        assert_eq!(classify("src/process/engine.ts"), FileRole::Code);
        assert_eq!(classify("scripts/build.mjs"), FileRole::Code);
    }

    #[test]
    fn docs_and_artifacts_are_never_mutated() {
        // Reverting a changelog or a backlog artifact creates no defect; it
        // would only add noise to the survival rate.
        for p in [
            "CHANGELOG.md",
            ".bklg/support/x/implementation-report.md",
            "docs/src/content/docs/reference/cli.mdx",
        ] {
            assert_eq!(classify(p), FileRole::Other, "{p}");
        }
    }

    #[test]
    fn windows_separators_are_handled() {
        assert_eq!(classify("src\\cli\\advance.ts"), FileRole::Code);
        assert_eq!(classify("test\\a.test.ts"), FileRole::Test);
    }

    #[test]
    fn tail_keeps_the_end_not_the_start() {
        let s = "a".repeat(50) + "MARKER";
        let t = tail(&s, 10);
        assert!(t.ends_with("MARKER"), "got {t}");
        assert!(t.len() <= 12);
    }
}
