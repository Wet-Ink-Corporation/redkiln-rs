//! S0.2 — deterministic reconcile replay. Zero model cost.
//!
//! RFC §3.4 calls the Preflight agent's git archaeology "the single
//! highest-value thing to port", because it removes an LLM from the resume
//! decision; shipmates L2.2 reaches the same design independently. Nothing had
//! tested it.
//!
//! The design rule this encodes is shipmates L2.1: **membership may union
//! several sources; completion trusts exactly one class of verified signal.**
//! Story trailers establish that work was *committed*. They do not establish
//! that it was *accepted* — RFC §3.4: "a slice REJECTED at review looks
//! identical, in commit history, to one that passed … the state that most needs
//! recovery was precisely the state resume could not see." Only a
//! `Slice-Verdict` says accepted.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result};
use serde::Serialize;

/// One commit's resume-relevant trailers.
#[derive(Debug, Clone, Serialize)]
pub struct CommitTrailers {
    pub sha: String,
    pub unix_time: i64,
    pub subject: String,
    pub stories: Vec<String>,
    /// `(slice, verdict)` in the order they appear.
    pub slice_verdicts: Vec<(String, String)>,
    pub fixes: Vec<String>,
    pub repairs: Vec<String>,
}

/// A slice's sealing history. The **latest** verdict wins, which is why the
/// history is retained rather than collapsed on the way in: a slice rejected and
/// then re-approved is a different situation from one only ever approved, and
/// the difference is exactly what resume needs.
#[derive(Debug, Clone, Serialize)]
pub struct SliceState {
    pub slice: String,
    pub latest_verdict: String,
    pub history: Vec<(String, String)>,
    pub sealed_more_than_once: bool,
}

impl SliceState {
    pub fn is_done(&self) -> bool {
        self.latest_verdict == "approved"
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProjectState {
    pub project: String,
    /// Union of every `Story:` trailer seen. Membership, not completion.
    pub stories_committed: BTreeSet<String>,
    pub slices: BTreeMap<String, SliceState>,
}

/// Something the world says that the record cannot explain. Reported, never
/// silently reconciled (shipmates L2.1).
#[derive(Debug, Clone, Serialize)]
pub struct Drift {
    pub kind: &'static str,
    pub project: String,
    pub subject: String,
    pub detail: String,
}

#[derive(Debug, Default, Serialize)]
pub struct State {
    pub projects: BTreeMap<String, ProjectState>,
    /// Bare-ID `Story:` trailers (`RK-S0272`), used by the fix lane rather than
    /// the `project/slug` form. Kept separate: they carry no project, so
    /// folding them into a project would invent one.
    pub bare_story_ids: BTreeSet<String>,
    pub fixes: BTreeSet<String>,
}

/// Split `project/slug`, or `None` for a bare id.
fn split_project(value: &str) -> Option<(String, String)> {
    let v = value.trim();
    let (p, s) = v.split_once('/')?;
    (!p.is_empty() && !s.is_empty()).then(|| (p.to_string(), s.to_string()))
}

/// Parse the resume-relevant trailers out of one commit body.
///
/// Anchored at column 0 and case-sensitive on the key, matching
/// `src/store/verify.ts`'s `STORY_TRAILER`. An indented or lower-cased mention
/// inside prose must not register as a trailer.
pub fn parse_body(sha: &str, unix_time: i64, subject: &str, body: &str) -> CommitTrailers {
    let mut t = CommitTrailers {
        sha: sha.to_string(),
        unix_time,
        subject: subject.to_string(),
        stories: vec![],
        slice_verdicts: vec![],
        fixes: vec![],
        repairs: vec![],
    };
    for line in body.lines() {
        // Column-0 only: no trim_start.
        if let Some(v) = line.strip_prefix("Story:") {
            let v = v.trim();
            if !v.is_empty() {
                t.stories.push(v.to_string());
            }
        } else if let Some(v) = line.strip_prefix("Slice-Verdict:") {
            let mut it = v.split_whitespace();
            if let (Some(slice), Some(verdict)) = (it.next(), it.next()) {
                t.slice_verdicts
                    .push((slice.to_string(), verdict.to_ascii_lowercase()));
            }
        } else if let Some(v) = line.strip_prefix("Fix:") {
            let v = v.trim();
            if !v.is_empty() {
                t.fixes.push(v.to_string());
            }
        } else if let Some(v) = line
            .strip_prefix("Slice-Repair:")
            .or_else(|| line.strip_prefix("Baseline-Repair:"))
        {
            let v = v.trim();
            if !v.is_empty() {
                t.repairs.push(v.to_string());
            }
        }
    }
    t
}

/// Fold commits (oldest first) into state, reporting drift rather than fixing it.
pub fn reconcile(commits: &[CommitTrailers]) -> (State, Vec<Drift>) {
    let mut state = State::default();
    let mut drift = Vec::new();

    for c in commits {
        for raw in &c.stories {
            match split_project(raw) {
                Some((project, slug)) => {
                    let p = state.projects.entry(project.clone()).or_insert_with(|| {
                        ProjectState {
                            project: project.clone(),
                            ..Default::default()
                        }
                    });
                    if !p.stories_committed.insert(slug.clone()) {
                        drift.push(Drift {
                            kind: "duplicate-story-trailer",
                            project,
                            subject: slug,
                            detail: format!("committed again at {}", &c.sha[..8.min(c.sha.len())]),
                        });
                    }
                }
                None => {
                    state.bare_story_ids.insert(raw.clone());
                }
            }
        }

        for (raw, verdict) in &c.slice_verdicts {
            let Some((project, slice)) = split_project(raw) else {
                drift.push(Drift {
                    kind: "unqualified-slice-verdict",
                    project: String::new(),
                    subject: raw.clone(),
                    detail: "no project/slice form".into(),
                });
                continue;
            };
            let p = state
                .projects
                .entry(project.clone())
                .or_insert_with(|| ProjectState {
                    project: project.clone(),
                    ..Default::default()
                });
            let entry = p.slices.entry(slice.clone()).or_insert_with(|| SliceState {
                slice: slice.clone(),
                latest_verdict: verdict.clone(),
                history: vec![],
                sealed_more_than_once: false,
            });
            entry.history.push((c.sha.clone(), verdict.clone()));
            entry.sealed_more_than_once = entry.history.len() > 1;
            // Latest wins: commits are folded oldest-first.
            entry.latest_verdict = verdict.clone();
        }

        for f in &c.fixes {
            state.fixes.insert(f.clone());
        }
    }

    // A slice whose last word was `changes-requested` is unfinished work that a
    // naive resume would skip.
    for p in state.projects.values() {
        for s in p.slices.values() {
            if !s.is_done() {
                drift.push(Drift {
                    kind: "slice-left-rejected",
                    project: p.project.clone(),
                    subject: s.slice.clone(),
                    detail: format!("latest verdict `{}`", s.latest_verdict),
                });
            }
        }
    }

    (state, drift)
}

/// **The RFC's axis-2 claim, made measurable.**
///
/// If resume trusted `Story:` trailers alone, how much rejected work would it
/// mistake for finished? Every slice whose latest verdict is not `approved` is
/// a case where "committed" and "accepted" disagree.
#[derive(Debug, Serialize)]
pub struct AxisComparison {
    pub slices_total: usize,
    pub slices_approved: usize,
    pub slices_rejected: usize,
    pub slices_resealed: usize,
    /// Rejected slices that were later re-sealed approved — the rework cycle
    /// axis 2 exists to make visible.
    pub rejected_then_approved: usize,
    /// Slices whose last word is a rejection. A Story-trailer-only resume would
    /// treat these as done **at the final snapshot**.
    pub would_be_misread_as_done: usize,
    /// Slices that were rejected at ANY point — the mid-flight exposure.
    ///
    /// This, not the final-snapshot figure, is what the RFC's axis-2 claim is
    /// about. Resume does not run at the end of history; it runs when a
    /// workflow is re-entered, which is precisely when some slice is sitting in
    /// a rejected state. Every slice counted here was, for some window, work
    /// that a `Story:`-trailer-only resume would have skipped as finished.
    pub ever_rejected: usize,
}

pub fn compare_axes(state: &State) -> AxisComparison {
    let mut c = AxisComparison {
        slices_total: 0,
        slices_approved: 0,
        slices_rejected: 0,
        slices_resealed: 0,
        rejected_then_approved: 0,
        would_be_misread_as_done: 0,
        ever_rejected: 0,
    };
    for p in state.projects.values() {
        for s in p.slices.values() {
            c.slices_total += 1;
            if s.sealed_more_than_once {
                c.slices_resealed += 1;
            }
            let had_rejection = s.history.iter().any(|(_, v)| v != "approved");
            if had_rejection {
                c.ever_rejected += 1;
            }
            if s.is_done() {
                c.slices_approved += 1;
                if had_rejection {
                    c.rejected_then_approved += 1;
                }
            } else {
                c.slices_rejected += 1;
                c.would_be_misread_as_done += 1;
            }
        }
    }
    c
}

/// Read commits oldest-first across all refs.
pub fn read_commits(repo: &std::path::Path) -> Result<Vec<CommitTrailers>> {
    // Unit separators keep parsing off the shell and out of regex escaping,
    // matching what `verify.ts::trailerCommits` does.
    let out = std::process::Command::new("git")
        .args([
            "log",
            "--all",
            "--reverse",
            "--format=%H\x1f%ct\x1f%s\x1f%b\x1e",
        ])
        .current_dir(repo)
        .output()
        .context("git log")?;
    anyhow::ensure!(out.status.success(), "git log failed");

    let text = String::from_utf8_lossy(&out.stdout);
    let mut commits = Vec::new();
    for chunk in text.split('\x1e') {
        let chunk = chunk.trim_start_matches(['\n', '\r']);
        if chunk.trim().is_empty() {
            continue;
        }
        let mut parts = chunk.splitn(4, '\x1f');
        let (Some(sha), Some(ct), Some(subject), Some(body)) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            continue;
        };
        let t = parse_body(sha, ct.parse().unwrap_or(0), subject, body);
        if !t.stories.is_empty()
            || !t.slice_verdicts.is_empty()
            || !t.fixes.is_empty()
            || !t.repairs.is_empty()
        {
            commits.push(t);
        }
    }
    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(sha: &str, body: &str) -> CommitTrailers {
        parse_body(sha, 0, "subject", body)
    }

    #[test]
    fn trailers_are_column_anchored_and_case_sensitive() {
        // An indented or lower-cased mention inside prose is not a trailer.
        let t = c(
            "aaa",
            "Body text.\n  Story: indented/nope\nstory: lower/nope\nStory: proj/yes\n",
        );
        assert_eq!(t.stories, vec!["proj/yes"]);
    }

    #[test]
    fn slice_verdicts_capture_slice_and_verdict() {
        let t = c("aaa", "Slice-Verdict: proj/slice-one approved\n");
        assert_eq!(
            t.slice_verdicts,
            vec![("proj/slice-one".to_string(), "approved".to_string())]
        );
    }

    #[test]
    fn latest_verdict_wins_and_history_is_kept() {
        // The resume-critical sequence: rejected, then re-approved.
        let commits = vec![
            c("a1", "Slice-Verdict: p/s changes-requested\n"),
            c("a2", "Slice-Verdict: p/s approved\n"),
        ];
        let (state, _) = reconcile(&commits);
        let s = &state.projects["p"].slices["s"];
        assert_eq!(s.latest_verdict, "approved");
        assert!(s.is_done());
        assert!(s.sealed_more_than_once);
        assert_eq!(s.history.len(), 2);
    }

    #[test]
    fn a_slice_left_rejected_is_not_done_and_is_reported_as_drift() {
        let commits = vec![
            c("a1", "Slice-Verdict: p/s approved\n"),
            c("a2", "Slice-Verdict: p/s changes-requested\n"),
        ];
        let (state, drift) = reconcile(&commits);
        assert!(!state.projects["p"].slices["s"].is_done());
        assert!(drift.iter().any(|d| d.kind == "slice-left-rejected"));
    }

    #[test]
    fn story_membership_does_not_imply_completion() {
        // The RFC's whole point: a committed story whose slice was rejected
        // must not read as finished.
        let commits = vec![c(
            "a1",
            "Story: p/story-one\nSlice-Verdict: p/slice-one changes-requested\n",
        )];
        let (state, _) = reconcile(&commits);
        let p = &state.projects["p"];
        assert!(p.stories_committed.contains("story-one"));
        assert!(!p.slices["slice-one"].is_done());

        let cmp = compare_axes(&state);
        assert_eq!(cmp.would_be_misread_as_done, 1);
    }

    #[test]
    fn bare_ids_are_kept_apart_from_projects() {
        // The fix lane writes `Story: RK-S0272`. Folding that into a project
        // would invent one.
        let (state, _) = reconcile(&[c("a1", "Story: RK-S0272\n")]);
        assert!(state.bare_story_ids.contains("RK-S0272"));
        assert!(state.projects.is_empty());
    }

    #[test]
    fn duplicate_story_trailers_are_drift_not_silently_merged() {
        let commits = vec![c("a1", "Story: p/s\n"), c("a2", "Story: p/s\n")];
        let (_, drift) = reconcile(&commits);
        assert!(drift.iter().any(|d| d.kind == "duplicate-story-trailer"));
    }

    #[test]
    fn rejected_then_approved_is_counted_separately_from_never_rejected() {
        let commits = vec![
            c("a1", "Slice-Verdict: p/a changes-requested\n"),
            c("a2", "Slice-Verdict: p/a approved\n"),
            c("a3", "Slice-Verdict: p/b approved\n"),
        ];
        let (state, _) = reconcile(&commits);
        let cmp = compare_axes(&state);
        assert_eq!(cmp.slices_total, 2);
        assert_eq!(cmp.slices_approved, 2);
        assert_eq!(cmp.rejected_then_approved, 1);
        assert_eq!(cmp.would_be_misread_as_done, 0);
        // The final snapshot is clean, but one slice WAS rejected mid-flight —
        // which is when resume actually runs.
        assert_eq!(cmp.ever_rejected, 1);
    }
}
