//! `reconcile()` — deterministic resume from git alone.
//!
//! **Lifted, with edits, from `runner/src/reconcile.rs` in `redkiln-corpus` at
//! `27d97e9b164d09dc755fe38e353de922cb97c482`** (the SHA in `CORPUS_PIN`; blob
//! `2acb9c3639f571f3c19562fa6164ca4a11858dc4`, 420 lines). That file is itself the spike runner's
//! re-implementation (`330d46c`, 2026-09-02) of the old redkiln's `src/store/verify.ts` at the
//! frozen predecessor `47faec9f`. It was **built and run in S0.2 over 360 commits / 74 slices (F12)
//! and exercised, not validated against an oracle** — so what travels here is the design, not a
//! warrant (PIA-036, PIA-078).
//!
//! The design rule it encodes is shipmates L2.1: **membership may union several sources; completion
//! trusts exactly one class of verified signal.** `Story:` trailers establish that work was
//! *committed*. They do not establish that it was *accepted* — RFC §3.4: *"a slice REJECTED at
//! review looks identical, in commit history, to one that passed … the state that most needs
//! recovery was precisely the state resume could not see."*
//!
//! **Four edits, each forced by a rule the corpus version predates:**
//!
//! 1. `anyhow` is gone. It is confined to `rk-cli` and `redkiln-migrate` (ADR-0102 decision 6) and
//!    CI asserts that over `cargo metadata`; this crate returns [`GitError`].
//! 2. `read_commits` no longer calls `std::process::Command` — it goes through the one wrapper
//!    (ADR-0077:70, and Checkpoint 6's type ban).
//! 3. **The third axis.** The corpus version modelled `Story:` and `Slice-Verdict:` only. ADR-0109
//!    decision 3 requires three, so `Gate:` is parsed here, a non-triple `Gate:` value is legacy
//!    drift, and unknown keys are *counted* rather than dropped (design §5.4's last row).
//! 4. `Drift` is the typed `rk_types::drift::Drift`, not a `kind: &'static str` record. The corpus
//!    shape would let template drift and human drift carry the same string; AC-128 forbids exactly
//!    that, so the distinction is now a variant the compiler keeps apart.

use std::collections::{BTreeMap, BTreeSet};

use rk_types::drift::Drift;
use rk_types::text::Text;

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;
use crate::trailer::grammar::{
    GateState, MalformedReason, ParsedTrailer, StoryStage, TrailerKey, UnknownKeyCounts, Verdict,
    parse_body,
};

/// One commit's resume-relevant trailers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitTrailers {
    pub sha: String,
    pub unix_time: i64,
    pub subject: String,
    pub trailers: Vec<ParsedTrailer>,
}

/// A slice's sealing history. The **latest** verdict wins, which is why the history is retained
/// rather than collapsed on the way in: a slice rejected and then re-approved is a different
/// situation from one only ever approved, and the difference is exactly what resume needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceState {
    pub slice: String,
    pub latest_verdict: Verdict,
    pub history: Vec<(String, Verdict)>,
    pub sealed_more_than_once: bool,
}

impl SliceState {
    #[must_use]
    pub const fn is_done(&self) -> bool {
        matches!(self.latest_verdict, Verdict::Approved)
    }

    /// Did this slice ever pass through a rejected state? The figure the RFC's axis-2 claim is
    /// actually about — **not** the final-snapshot figure, because resume does not run at the end
    /// of history.
    #[must_use]
    pub fn ever_rejected(&self) -> bool {
        self.history
            .iter()
            .any(|(_, v)| matches!(v, Verdict::ChangesRequested))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectState {
    pub project: String,
    /// Union of every `Story:` trailer seen. Membership, not completion.
    pub stories_committed: BTreeSet<String>,
    /// Every distinct `(slug, stage)` mark seen, the legacy stage-less one as `None`. A repeat of
    /// one of these is `DuplicateStoryTrailer` drift. *(Added 2026-09-28, `s8e` build, package
    /// S8E-P4; ADR-0183, proposed.)*
    pub story_marks: BTreeSet<(String, Option<StoryStage>)>,
    /// The latest stage-bearing (or `closed`) mark per slug, folded oldest-first. A legacy
    /// stage-less mark never sets or resets it. *(Added 2026-09-28, `s8e` build, package S8E-P4;
    /// ADR-0183, proposed.)*
    pub stages: BTreeMap<String, StageMark>,
    pub slices: BTreeMap<String, SliceState>,
}

/// An item's latest stage as git carries it, and the commit that carries it.
/// *(Added 2026-09-28, `s8e` build, package S8E-P4; ADR-0183, proposed.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageMark {
    pub stage: StoryStage,
    pub commit: String,
}

/// A gate as the trailers carry it: its identity, its latest state, and every transition.
///
/// *(Widened 2026-09-28, `s8e` build, package S8E-P4; ADR-0184, proposed.)* `state` and `commit`
/// are the latest mark's, folded oldest-first; `history` is every mark for the token, in fold
/// order, so a rebuild can emit every transition and not just the last.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateRecord {
    pub token: String,
    pub tree_hash: String,
    pub deadline: i64,
    pub kind: String,
    pub reach: u32,
    pub state: GateState,
    pub commit: String,
    pub history: Vec<(String, GateState)>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct State {
    pub projects: BTreeMap<String, ProjectState>,
    /// Bare-ID `Story:` trailers (`RK-S0272`), used by the fix lane rather than the `project/slug`
    /// form. Kept separate: they carry no project, so folding them into one would invent it.
    pub bare_story_ids: BTreeSet<String>,
    /// The third axis. ~~Latest mark per token.~~ *(2026-09-28, `s8e` build, package S8E-P4;
    /// ADR-0184, proposed.)* The latest STATE per token, with every transition in its history.
    pub gates: BTreeMap<String, GateRecord>,
    /// Unknown trailer keys: **counted, not drift** (design §5.4's last row).
    pub unknown_keys: UnknownKeyCounts,
}

/// Fold `commits` (oldest first) into state plus the drift it could not explain.
#[must_use]
pub fn reconcile(commits: &[CommitTrailers]) -> (State, Vec<Drift>) {
    let mut state = State::default();
    let mut drift = Vec::new();

    for c in commits {
        for t in &c.trailers {
            match t {
                ParsedTrailer::Story {
                    project,
                    slug,
                    stage,
                } => {
                    // *(Widened 2026-09-28, `s8e` build, package S8E-P4; ADR-0183, proposed.)*
                    // Membership is kept; a repeat of an IDENTICAL `(project, slug, stage)` is the
                    // drift it always was, counted and not a transition; and the latest
                    // stage-bearing mark wins, oldest-first, as `Slice-Verdict:` already does. A
                    // stage-less mark never sets or resets the stage.
                    let p = project_entry(&mut state, project);
                    p.stories_committed.insert(slug.clone());
                    if !p.story_marks.insert((slug.clone(), stage.clone())) {
                        drift.push(Drift::DuplicateStoryTrailer {
                            project: project.clone(),
                            slug: slug.clone(),
                            commit: c.sha.clone(),
                        });
                        continue;
                    }
                    if let Some(stage) = stage {
                        // Latest wins: commits are folded oldest-first.
                        p.stages.insert(
                            slug.clone(),
                            StageMark {
                                stage: stage.clone(),
                                commit: c.sha.clone(),
                            },
                        );
                    }
                }
                ParsedTrailer::BareStory { id } => {
                    state.bare_story_ids.insert(id.clone());
                }
                ParsedTrailer::SliceVerdict {
                    project,
                    slice,
                    verdict,
                } => {
                    let p = project_entry(&mut state, project);
                    let entry = p.slices.entry(slice.clone()).or_insert_with(|| SliceState {
                        slice: slice.clone(),
                        latest_verdict: *verdict,
                        history: vec![],
                        sealed_more_than_once: false,
                    });
                    entry.history.push((c.sha.clone(), *verdict));
                    entry.sealed_more_than_once = entry.history.len() > 1;
                    // Latest wins: commits are folded oldest-first.
                    entry.latest_verdict = *verdict;
                }
                ParsedTrailer::Gate {
                    token,
                    tree_hash,
                    deadline,
                    kind,
                    reach,
                    state: gate_state,
                } => {
                    // *(Widened 2026-09-28, `s8e` build, package S8E-P4; ADR-0184, proposed.)*
                    // The latest state per token, oldest-first, with every transition kept.
                    let record = state
                        .gates
                        .entry(token.clone())
                        .or_insert_with(|| GateRecord {
                            token: token.clone(),
                            tree_hash: tree_hash.clone(),
                            deadline: *deadline,
                            kind: kind.clone(),
                            reach: *reach,
                            state: gate_state.clone(),
                            commit: c.sha.clone(),
                            history: Vec::new(),
                        });
                    record.history.push((c.sha.clone(), gate_state.clone()));
                    // Latest wins: commits are folded oldest-first.
                    record.state = gate_state.clone();
                    record.commit = c.sha.clone();
                }
                ParsedTrailer::UnknownKey { key } => state.unknown_keys.record(key),
                ParsedTrailer::Malformed {
                    key,
                    offending,
                    why,
                } => drift.push(malformed_to_drift(&c.sha, key, offending, *why)),
            }
        }
    }

    // A slice whose last word was `changes-requested` is unfinished work a naive resume would skip.
    for p in state.projects.values() {
        for s in p.slices.values() {
            if !s.is_done() {
                drift.push(Drift::SliceLeftRejected {
                    project: p.project.clone(),
                    slice: s.slice.clone(),
                    latest: s.latest_verdict.as_str().to_string(),
                });
            }
        }
    }

    (state, drift)
}

fn project_entry<'a>(state: &'a mut State, project: &str) -> &'a mut ProjectState {
    state
        .projects
        .entry(project.to_string())
        .or_insert_with(|| ProjectState {
            project: project.to_string(),
            ..Default::default()
        })
}

fn malformed_to_drift(sha: &str, key: &TrailerKey, offending: &str, why: MalformedReason) -> Drift {
    match why {
        MalformedReason::LegacyGateValue => Drift::LegacyGateTrailer {
            commit: sha.to_string(),
            offending: Text::new(offending),
        },
        MalformedReason::UnqualifiedSliceVerdict => Drift::UnqualifiedSliceVerdict {
            commit: sha.to_string(),
            offending: Text::new(offending),
        },
        MalformedReason::UnknownVerdict
        | MalformedReason::MalformedStoryValue
        | MalformedReason::MalformedGateValue => Drift::UnparsableTrailer {
            commit: sha.to_string(),
            key: key.as_str().to_string(),
            offending: Text::new(offending),
        },
    }
}

/// Read every trailer-bearing commit reachable from any ref, oldest first.
///
/// 🛑 **THE ORDER ASSUMPTION, STATED** *(2026-09-28, `s8e` build, package S8E-P4; ADR-0183,
/// proposed)*. The per-item stage fold and the per-token gate fold are both *latest wins*, so they
/// rest on this read's order. It is `log --all --reverse` **without `--topo-order`**. Git's default
/// walk emits a commit only after every child that reached it, so over ONE linear run ref a child
/// always precedes its parent, whatever the commit dates, and `--reverse` puts the parent first.
/// That is the case the lane writes: every `Story:` and `Gate:` mark for an item is a commit on
/// `rk-run/<item>` alone. Across refs the default walk interleaves by commit date, so two refs'
/// marks for ONE item would be ordered by clock; nothing writes that shape, and the record names it
/// rather than paying `--topo-order`'s change to the F12 corpus read.
///
/// Unit separators keep parsing off the shell and out of regex escaping, matching what
/// `verify.ts::trailerCommits` does. **This is edit 2**: the corpus version spawned
/// `std::process::Command` here; it now goes through the one wrapper.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn read_commits(exec: &GitExec, repo: &WorktreePath) -> Result<Vec<CommitTrailers>, GitError> {
    let out = exec.run(
        GitOp::Log,
        repo,
        [
            "log",
            "--all",
            "--reverse",
            "--format=%H\x1f%ct\x1f%s\x1f%b\x1e",
        ],
    )?;

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
        let trailers = parse_body(body);
        if !trailers.is_empty() {
            commits.push(CommitTrailers {
                sha: sha.to_string(),
                unix_time: ct.parse().unwrap_or(0),
                subject: subject.to_string(),
                trailers,
            });
        }
    }
    Ok(commits)
}
