//! The results store: append-only JSONL, always outside the fixture repo.
//!
//! Charter principle 3: nothing the experiment writes may touch the tree under
//! test. Claude Code's system prompt captures git state, and sequential runs
//! share a cache prefix only when that snapshot matches — so a results file
//! written inside a fixture would perturb the very thing E1 measures. That is
//! not a hypothetical: it is exactly how the current system would corrupt a
//! cache measurement, by committing telemetry mid-run.

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

#[derive(Debug)]
pub struct Journal {
    path: PathBuf,
    /// Canonicalised roots this journal must never be written inside.
    forbidden: Vec<PathBuf>,
}

/// The header written once per results file, so a record can never be read
/// without the provenance needed to interpret it (charter §14.2).
#[derive(Debug, Clone, Serialize)]
pub struct RunHeader<'a> {
    pub record: &'static str,
    pub experiment: &'a str,
    pub arm: &'a str,
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// The pre-registration commit. Charter §8.2 — any change after the first
    /// run is a new pre-registration and is reported as such.
    pub preregistration_sha: Option<String>,
    /// The frozen redkiln corpus pin.
    pub corpus_sha: Option<String>,
    pub claude_version: String,
    pub platform: String,
}

impl Journal {
    /// Open (creating if needed) an append-only journal, refusing any path
    /// inside a fixture tree.
    pub fn open(path: impl AsRef<Path>, forbidden: &[PathBuf]) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("could not create {}", parent.display()))?;
        }

        let forbidden: Vec<PathBuf> = forbidden
            .iter()
            .filter_map(|p| std::fs::canonicalize(p).ok())
            .collect();

        let journal = Self { path, forbidden };
        journal.assert_outside_fixtures()?;
        Ok(journal)
    }

    fn assert_outside_fixtures(&self) -> Result<()> {
        // Canonicalise the parent: the file itself may not exist yet.
        let anchor = self
            .path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let resolved = std::fs::canonicalize(anchor)
            .with_context(|| format!("could not resolve {}", anchor.display()))?;

        for root in &self.forbidden {
            if resolved.starts_with(root) {
                bail!(
                    "results path {} is inside the fixture tree {} — writing there would \
                     perturb the git snapshot the cache measurement depends on \
                     (charter principle 3)",
                    self.path.display(),
                    root.display()
                );
            }
        }
        Ok(())
    }

    pub fn append<T: Serialize>(&self, record: &T) -> Result<()> {
        let line = serde_json::to_string(record).context("record is not serialisable")?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| format!("could not open {}", self.path.display()))?;
        writeln!(file, "{line}").with_context(|| format!("could not write {}", self.path.display()))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Resume keys for every step that already SUCCEEDED in this results file.
///
/// The Opus family sub-limit is enforced but not readable (charter §6.1, and
/// FINDINGS F1 confirms `unifiedWindows` carries no family key), so an
/// Opus-heavy arm *will* be interrupted at an unpredictable point. The answer
/// is not better forecasting — it is making the interruption cost one dispatch
/// instead of an arm.
///
/// Only `outcome == "ok"` counts. A step that failed is re-run, because the
/// dispatch that hit the limit produced no judgement.
///
/// Note this is deliberately not shipmates' L2.3 ("no `--resume` flag, one
/// entry point that reconciles"). This is measurement apparatus reading its own
/// append-only journal, not an orchestrator reconciling against the world.
pub fn completed_keys(path: impl AsRef<Path>) -> std::collections::HashSet<String> {
    let mut done = std::collections::HashSet::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return done;
    };
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if v.get("outcome").and_then(|o| o.as_str()) != Some("ok") {
            continue;
        }
        let (Some(exp), Some(arm), Some(id)) = (
            v.get("experiment").and_then(|x| x.as_str()),
            v.get("arm").and_then(|x| x.as_str()),
            v.get("identity").and_then(|x| x.as_str()),
        ) else {
            continue;
        };
        done.insert(format!("{exp}/{arm}/{id}"));
    }
    done
}

/// Read the pinned corpus SHA written by the scaffolding.
pub fn read_pinned_sha(pin_file: impl AsRef<Path>) -> Option<String> {
    let text = std::fs::read_to_string(pin_file).ok()?;
    text.lines()
        .find_map(|l| l.trim().strip_prefix("REDKILN_SHA=").map(str::to_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_a_results_path_inside_a_fixture() {
        let tmp = std::env::temp_dir().join(format!("rk-journal-{}", std::process::id()));
        let fixture = tmp.join("fixtures").join("e1");
        std::fs::create_dir_all(&fixture).unwrap();

        let err = Journal::open(fixture.join("results.jsonl"), &[tmp.join("fixtures")])
            .expect_err("must refuse a path inside the fixture tree");
        assert!(err.to_string().contains("inside the fixture tree"), "{err}");

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn accepts_a_sibling_results_dir_and_appends() {
        let tmp = std::env::temp_dir().join(format!("rk-journal-ok-{}", std::process::id()));
        let fixture = tmp.join("fixtures");
        std::fs::create_dir_all(&fixture).unwrap();

        let journal = Journal::open(tmp.join("results").join("e1.jsonl"), &[fixture]).unwrap();
        journal.append(&serde_json::json!({"a": 1})).unwrap();
        journal.append(&serde_json::json!({"a": 2})).unwrap();

        let body = std::fs::read_to_string(journal.path()).unwrap();
        assert_eq!(body.lines().count(), 2, "appends, never truncates");

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn resume_counts_only_successful_steps() {
        let tmp = std::env::temp_dir().join(format!("rk-resume-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let f = tmp.join("r.jsonl");
        std::fs::write(
            &f,
            [
                r#"{"record":"header","experiment":"E3"}"#,
                r#"{"experiment":"E3","arm":"b","identity":"review#1:1:opus","outcome":"ok"}"#,
                // Interrupted by the invisible Opus sub-limit: must re-run.
                r#"{"experiment":"E3","arm":"b","identity":"review#2:1:opus","outcome":"error","error_kind":"usage-limit"}"#,
                r#"{"experiment":"E3","arm":"c","identity":"review#1:1:opus","outcome":"ok"}"#,
                "not json at all",
            ]
            .join("\n"),
        )
        .unwrap();

        let done = completed_keys(&f);
        assert!(done.contains("E3/b/review#1:1:opus"));
        assert!(
            !done.contains("E3/b/review#2:1:opus"),
            "a usage-limit failure produced no judgement and must be re-run"
        );
        // Same identity, different arm — must not be treated as done.
        assert!(done.contains("E3/c/review#1:1:opus"));
        assert_eq!(done.len(), 2);

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn resume_on_a_missing_file_is_empty_not_an_error() {
        assert!(completed_keys("does/not/exist.jsonl").is_empty());
    }

    #[test]
    fn reads_the_pin_file_past_its_comments() {
        let tmp = std::env::temp_dir().join(format!("rk-pin-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let pin = tmp.join("PINNED_SHA");
        std::fs::write(&pin, "# a comment\nREDKILN_REPO=D:/x\nREDKILN_SHA=47faec9f\n").unwrap();
        assert_eq!(read_pinned_sha(&pin).as_deref(), Some("47faec9f"));
        std::fs::remove_dir_all(&tmp).ok();
    }
}
