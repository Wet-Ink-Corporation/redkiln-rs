//! **The tier-1 boundary: every tier-1 act is git-first, through ONE function** (`s8e` §8e.3).
//!
//! *(dated 2026-09-28, `s8e` build, package S8E-P5; D21-42 8a: "The kernel owns it, through the
//! journalled `GitOps` seam. It commits (zero writes on replay), then appends the t1 and t2 rows with
//! `anchor_sha`. `reconcile` heals a commit without a row."; D21-43 N6, N7 and N10's seam; the tier-1
//! writer record ADR-0181 and the three-kinds record, both proposed.)*
//!
//! [`commit_act`] takes the journalled [`GitOps`] seam, the store, the [`Target`] (whose
//! [`RunOwnership`] the caller read from the store's `worktree_assigned` row, §8e.4) and one
//! [`Act`], and in this order:
//!
//! 1. authorises the target through `rk_git::worktree::authorise_tier1_write`;
//! 2. commits through the seam (`--allow-empty`, the trailer; the wrapper adds `-c
//!    core.hooksPath=<empty>` and `-c trailer.ifexists=add`, ADR-0105 clauses 2 and 3);
//! 3. re-reads `HEAD` and the tree, through the seam;
//! 4. calls `append_tier1` for every row of the act, each anchored to that one commit, and then one
//!    `git_op` row per invocation the seam made (N7), anchored the same way.
//!
//! **A failed commit returns before step 4, so no row exists** — AC-162's second clause by
//! construction, which `tier_one::a_failed_commit_leaves_zero_store_rows` watches. A failed act
//! writes no `git_op` row either: its invocations are held until the commit exists, and the failure
//! travels on the error (the record states why AC-162's *"zero store rows"* binds harder than
//! journalling a failed invocation).
//!
//! **Git is not a port** (ADR-0003's bullet *"Git may not be abstracted behind a port"*), so the seam
//! travels beside `Ports`, never inside it: [`Boundary`] owns one and the runtime binds it
//! ([`super::Runtime::with_boundary`]).
//!
//! # Zero writes on replay, and the stated rule for golden logs (N7)
//!
//! In [`Mode::Replay`] the act takes no authorisation read and the seam serves the commit and its
//! re-reads from the act's recorded `git_op` rows ([`ActJournal::replaying`]); no row is appended.
//! **A journal holding no `git_op` row is a run that performed no git write**: the replay arm then
//! serves no commit at all ([`Committed::NoGitWrite`]), which is why the committed golden logs are
//! read as they are and never re-recorded.
//!
//! # The heal — the converse of AC-163
//!
//! [`heal`] appends the rows of every trailer mark reachable from `rk-run/<item>` whose sha anchors
//! no row. It is git → store, the direction ADR-0109 decision 1 permits, and it writes nothing to
//! git (AC-174): every git call it makes is a read.
//!
//! # What this module does NOT decide
//!
//! The `closed` mark's rows (§8e.6) and the seal's value (§8e.8) are their sections'; [`Mark`]
//! carries both so their producers ride this function, and [`heal`] skips ~~a `closed` mark and~~ ~~a
//! `Slice-Verdict:` mark, naming ~~them~~ it, until ~~those sections fix their rows~~ §8e.8 fixes its
//! row~~. *(dated 2026-09-28, `s8e` build, package S8E-P6: §8e.6 fixed the `closed` act's row,
//! [`close_act`], and the heal re-derives it.)* *(dated 2026-09-28, `s8e` build, package S8E-P7:
//! §8e.8 fixed the seal's act, [`seal_act`], with its refusals at the mint, and the heal re-derives
//! a `Slice-Verdict:` mark's `slice_sealed` row too — stage-less, because the trailer carries no
//! stage; the slice record states how the fold scopes such a row, D21-93.)*

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_git::error::GitError;
use rk_git::git_exec::{GitExec, GitOp};
use rk_git::gitops::{GitInvocation, GitJournal, GitOps, Mode};
use rk_git::landing::{self, Baseline, Landed, Refusal, Request, Staged};
use rk_git::path::WorktreePath;
use rk_git::reconcile::{CommitTrailers, read_commits};
use rk_git::trailer::grammar::{GateExpiry, GateState, ParsedTrailer, StoryStage, Verdict};
use rk_git::worktree::{RunOwnership, authorise_tier1_write};
use rk_ports::event_store::{EventStore, Query, TierOneAnchor};
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::{EventKind, ItemKey};

use crate::workflow::fsm::GateRecord;

use super::LOCK_HALTS;

/// Where an act commits and what it may write: the tree, the ownership the STORE records for the
/// run, and the two values a `Story:` value names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    /// The tree the act commits in.
    pub worktree: PathBuf,
    /// What the store's `worktree_assigned` row says the run owns (§8e.4). Never `cwd`, never a
    /// path convention.
    pub ownership: RunOwnership,
    /// `.redkiln/config.toml`'s `[project] name`. `None` refuses a `Story:` or `Slice-Verdict:`
    /// mark by name (ADR-0159).
    pub project: Option<String>,
    /// The item every row of the act is keyed to (ADR-0182).
    pub item: String,
}

/// The trailer an act carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mark {
    /// `Story: <project>/<item> <stage>` — the lane entered `<stage>` (ADR-0183).
    Stage(String),
    /// `Story: <project>/<item> closed` — the lane's end (D21-43 N9; §8e.6).
    Closed,
    /// `Gate: <token> <tree_hash> <deadline> <kind> <reach> <state> [<outcome>…]` (ADR-0184).
    Gate {
        token: String,
        tree_hash: String,
        deadline: i64,
        kind: String,
        reach: u32,
        state: GateState,
    },
    /// `Slice-Verdict: <project>/<slice> <verdict>` (§8e.8).
    Seal { slice: String, verdict: Verdict },
}

/// What the act's commit holds besides its trailer. ~~**One arm at Checkpoint 8e**: the empty
/// `--allow-empty` trailer commit.~~ `s8f`'s landed arm changes what the commit holds and not the
/// order (N10: *"The commit and the content checkpoint are one boundary act"*).
///
/// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; the landing record ADR-0188
/// decisions 2, 3, 7 and 14, proposed.)* Three arms. Every one commits through `rk-git`'s landing
/// primitive (`commit-tree` + `update-ref`), never through porcelain `commit`.
/// *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-114.)* ~~Three arms~~ Four: the entry
/// landing ([`Content::Entry`]) is the fourth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    /// No staging: the trailer rides an empty commit over `HEAD`'s tree. An answer's transition
    /// (`answer` does not land, decision 14) and a refused landing's gate (decision 7). With no
    /// trailer it is refused ([`TierOneError::EmptyAct`]).
    Empty,
    /// **A tier-1 boundary** (decision 2): the run-owned tree is staged; the trailer rides a
    /// content commit when a checkpoint is due, and an empty commit otherwise.
    Landed,
    /// **A step checkpoint** (decisions 2 and 3): trailer-less; a content commit when the step
    /// changed the tree, and NO commit when it did not. Its rows are anchored to the commit it
    /// made, or appended unanchored when it made none.
    Step(StepCheckpoint),
    /// **The entry landing** (ADR-0188 decision 2's dated note, proposed; D21-114, TAKEN
    /// PROVISIONALLY under D21-105). *(Added 2026-09-29, s8f build, package R5FIX2-8f.)* The
    /// entry pass's own writes (an accepted prose edit, an applied `move-to-prose`, a stored
    /// vouched root), landed before the first seam's read when no drift gate's `Gate:` commit
    /// lands them. Trailer-less and body-less: a content commit when the tree changed, NO commit
    /// when it did not, staged exactly as [`Content::Step`] is, and its only rows are its `git_op`
    /// rows. `heal`, [`marked_commits`] and the step-checkpoint read ignore it by construction: it
    /// has no trailer and its subject does not begin [`STEP_SUBJECT_PREFIX`].
    Entry(EntryCheckpoint),
}

/// The message an entry landing carries (D21-114): a subject only, `rk entry <item> <stage>
/// <run_id>`, with no body and no trailer. *(Added 2026-09-29, s8f build, package R5FIX2-8f.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryCheckpoint {
    /// `rk entry <item> <stage> <run_id>`.
    pub subject: String,
}

/// The entry landing's subject prefix (D21-114).
///
/// It never begins [`STEP_SUBJECT_PREFIX`], so the
/// heal's step-checkpoint read cannot take an entry landing for a step (asserted at compile time
/// below). *(Added 2026-09-29, s8f build, package R5FIX2-8f.)*
pub const ENTRY_SUBJECT_PREFIX: &str = "rk entry ";

/// `true` when `s` begins with `prefix`, in a `const` context.
const fn begins_with(s: &str, prefix: &str) -> bool {
    let (s, prefix) = (s.as_bytes(), prefix.as_bytes());
    if prefix.len() > s.len() {
        return false;
    }
    let mut i = 0;
    while i < prefix.len() {
        if s[i] != prefix[i] {
            return false;
        }
        i += 1;
    }
    true
}

const _: () = assert!(
    !begins_with(ENTRY_SUBJECT_PREFIX, STEP_SUBJECT_PREFIX),
    "an entry landing's subject must never begin the step checkpoint's prefix"
);

impl EntryCheckpoint {
    /// The entry landing of `item`'s stage run `(stage, run_id)`.
    #[must_use]
    pub fn of(item: &str, stage: &str, run_id: &str) -> Self {
        Self {
            subject: format!("{ENTRY_SUBJECT_PREFIX}{item} {stage} {run_id}"),
        }
    }
}

/// The message a step checkpoint carries (ADR-0188 decision 3): the subject names the step-grain
/// key, and the body carries the exact row the act appends, so `reconcile`'s heal can re-append a
/// row a crash lost. Neither line is in trailer form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepCheckpoint {
    /// `rk step <item> <activity_id@version> <step_id> <iteration-path>`.
    pub subject: String,
    /// `rk-step-row <canonical {kind, payload}>`.
    pub body: String,
}

/// The subject line's prefix, which the heal reads.
pub const STEP_SUBJECT_PREFIX: &str = "rk step ";
/// The body line's prefix, which the heal reads.
pub const STEP_ROW_PREFIX: &str = "rk-step-row ";

impl StepCheckpoint {
    /// The checkpoint for the step-grain key `(item, activity_id@version, step_id,
    /// iteration_path)` and the row the act appends.
    ///
    /// # Errors
    /// [`TierOneError::Journal`] when the row or the path does not serialise canonically.
    pub fn of(
        item: &str,
        activity_id_at_version: &str,
        step_id: &str,
        iteration_path: &serde_json::Value,
        row: &Event,
    ) -> Result<Self, TierOneError> {
        let canon = |v: &serde_json::Value| {
            rk_store::canon::canonical(v)
                .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
                .map_err(|e| TierOneError::Journal {
                    detail: format!("a step checkpoint's value is not canonical: {e:?}"),
                })
        };
        let path = canon(iteration_path)?;
        let body = canon(&serde_json::json!({
            "kind": row.kind.wire_name(),
            "payload": row.payload,
        }))?;
        Ok(Self {
            subject: format!(
                "{STEP_SUBJECT_PREFIX}{item} {activity_id_at_version} {step_id} {path}"
            ),
            body: format!("{STEP_ROW_PREFIX}{body}"),
        })
    }
}

/// One boundary act: an optional trailer, the act's content, and the rows the act anchors.
#[derive(Debug, Clone, PartialEq)]
pub struct Act {
    pub mark: Option<Mark>,
    pub content: Content,
    pub rows: Vec<Event>,
    /// For a gate's `opened` transition only: the opening stage run's `(stage, run_id)`, the
    /// `t1_gate_transition` columns `answer`'s `artifact.write` reads (`s8f` §8f.2 item 14).
    pub opening_run: Option<(String, String)>,
}

impl Act {
    /// An act carrying `mark`, ~~with the empty content arm~~ with the **landed** arm: the
    /// boundary stages the run-owned tree (ADR-0188 decision 2). *(dated 2026-09-28, s8f build,
    /// package S8F-P2-LANDING-CORE.)*
    #[must_use]
    pub const fn marked(mark: Mark, rows: Vec<Event>) -> Self {
        Self {
            mark: Some(mark),
            content: Content::Landed,
            rows,
            opening_run: None,
        }
    }

    /// An act carrying `mark` over an EMPTY commit, staging nothing: an answer's transition and a
    /// refused landing's gate (ADR-0188 decisions 7 and 14). *(dated 2026-09-28, s8f build,
    /// package S8F-P2-LANDING-CORE.)*
    #[must_use]
    pub const fn marked_empty(mark: Mark, rows: Vec<Event>) -> Self {
        Self {
            mark: Some(mark),
            content: Content::Empty,
            rows,
            opening_run: None,
        }
    }

    /// **The step-grain act** (N10; ADR-0188 decision 2): no trailer, the step's checkpoint, and
    /// the step's commit row. *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
    #[must_use]
    pub const fn step(checkpoint: StepCheckpoint, rows: Vec<Event>) -> Self {
        Self {
            mark: None,
            content: Content::Step(checkpoint),
            rows,
            opening_run: None,
        }
    }

    /// **The entry landing** (D21-114; ADR-0188 decision 2's dated note, proposed): no trailer, no
    /// row of its own, and no opening run. Its only rows are the `git_op` rows the act journals.
    /// *(Added 2026-09-29, s8f build, package R5FIX2-8f.)*
    #[must_use]
    pub const fn entry(checkpoint: EntryCheckpoint) -> Self {
        Self {
            mark: None,
            content: Content::Entry(checkpoint),
            rows: Vec::new(),
            opening_run: None,
        }
    }

    /// An act with no trailer. With the empty content arm it is refused ([`TierOneError::EmptyAct`]);
    /// the trailer-less act with content is `s8f`'s per-step checkpoint.
    #[must_use]
    pub const fn unmarked(rows: Vec<Event>) -> Self {
        Self {
            mark: None,
            content: Content::Empty,
            rows,
            opening_run: None,
        }
    }

    /// The same act, carrying the opening stage run for a gate's `opened` row.
    #[must_use]
    pub fn opened_by(mut self, stage: &str, run_id: &str) -> Self {
        self.opening_run = Some((stage.to_owned(), run_id.to_owned()));
        self
    }

    /// The act a gate transition row takes: its `Gate:` mark carries the OPENING record's token,
    /// tree hash, deadline, kind and reach, and the state the row's kind names (N6).
    ///
    /// # Errors
    /// [`TierOneError::NotAGateRow`] for a row of any other kind, or an `answered` row with no
    /// `verdict`, or an `expired` row with no `action`.
    pub fn gate(row: Event, opening: &GateRecord) -> Result<Self, TierOneError> {
        let state = gate_state_of(&row)?;
        // *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; ADR-0188 decisions 12 and
        // 14.)* An opening lands (its tree is the landing's T); an answer, an expiry and a void do
        // not: their mark rides an empty commit and carries the OPENING record's tree.
        let build = if state == GateState::Opened {
            Self::marked
        } else {
            Self::marked_empty
        };
        Ok(build(
            Mark::Gate {
                token: opening.token().as_str().to_owned(),
                tree_hash: opening.tree_hash().to_owned(),
                deadline: opening.deadline().unix_seconds(),
                kind: opening.kind().to_owned(),
                reach: opening.reach(),
                state,
            },
            vec![row],
        ))
    }

    /// The act a runtime-shaped `gate_opened` row takes, its mark read off the row itself.
    ///
    /// # Errors
    /// [`TierOneError::NotAGateRow`] for any other row, or one missing a member of the mark.
    pub fn gate_opened(row: Event) -> Result<Self, TierOneError> {
        if row.kind != EventKind::GateOpened {
            return Err(TierOneError::NotAGateRow {
                detail: format!("a `{}` row is not a gate opening", row.kind.wire_name()),
            });
        }
        let text = |m: &str| row.payload.get(m).and_then(serde_json::Value::as_str);
        let missing = |m: &str| TierOneError::NotAGateRow {
            detail: format!("the `gate_opened` row carries no `{m}`: {}", row.payload),
        };
        let token = text("token").ok_or_else(|| missing("token"))?.to_owned();
        let tree_hash = text("tree_hash")
            .ok_or_else(|| missing("tree_hash"))?
            .to_owned();
        let kind = text("kind").ok_or_else(|| missing("kind"))?.to_owned();
        let deadline = row
            .payload
            .get("deadline")
            .and_then(serde_json::Value::as_i64)
            .ok_or_else(|| missing("deadline"))?;
        let reach = row
            .payload
            .get("reach")
            .and_then(serde_json::Value::as_u64)
            .and_then(|r| u32::try_from(r).ok())
            .unwrap_or(1);
        Ok(Self::marked(
            Mark::Gate {
                token,
                tree_hash,
                deadline,
                kind,
                reach,
                state: GateState::Opened,
            },
            vec![row],
        ))
    }

    /// The act a runtime-shaped `gate_opened` row takes when its caller must NOT land: the same
    /// mark as [`Act::gate_opened`], over an EMPTY commit that stages nothing, so the mark keeps
    /// the row's own tree hash. `answer`'s expiry pass opens an escalation this way (ADR-0188
    /// decision 14: *"`answer` does not land"*). *(Added 2026-09-29, s8f build, package R4FIX-8f;
    /// refuting round 4's blocker.)*
    ///
    /// # Errors
    /// As [`Act::gate_opened`].
    pub fn gate_opened_empty(row: Event) -> Result<Self, TierOneError> {
        Self::gate_opened(row).map(|act| Self {
            content: Content::Empty,
            ..act
        })
    }
}

/// The state a gate transition row's kind names.
fn gate_state_of(row: &Event) -> Result<GateState, TierOneError> {
    let text = |m: &str| row.payload.get(m).and_then(serde_json::Value::as_str);
    let refuse = |detail: String| TierOneError::NotAGateRow { detail };
    match row.kind {
        EventKind::GateOpened => Ok(GateState::Opened),
        EventKind::GateAnswered => Ok(GateState::Answered {
            option: text("verdict")
                .ok_or_else(|| {
                    refuse(format!(
                        "`gate_answered` carries no verdict: {}",
                        row.payload
                    ))
                })?
                .to_owned(),
        }),
        EventKind::GateExpired => {
            let action = match text("action") {
                Some("fail") => GateExpiry::Fail,
                Some("escalate") => GateExpiry::Escalate,
                Some("auto-decide") => GateExpiry::AutoDecide {
                    verdict: text("verdict")
                        .ok_or_else(|| refuse("an `auto-decide` expiry carries no verdict".into()))?
                        .to_owned(),
                },
                other => {
                    return Err(refuse(format!(
                        "`gate_expired` carries no terminal action the grammar names: {other:?}"
                    )));
                }
            };
            Ok(GateState::Expired { action })
        }
        EventKind::GateAnswerVoid => Ok(GateState::Voided),
        other => Err(refuse(format!(
            "a `{}` row is not a gate transition",
            other.wire_name()
        ))),
    }
}

/// What one act did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Committed {
    /// Live: the commit exists and every row of the act is anchored to it.
    Live { sha: String, tree: String },
    /// Replay: the commit was SERVED from the act's `git_op` rows; nothing was written.
    Replayed { sha: String, tree: String },
    /// Replay over a journal holding no `git_op` row: the recorded run made no git write (N7's
    /// stated rule), so none is served.
    NoGitWrite,
    /// *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.5; ADR-0183 decision 6,
    /// proposed.)* Live: the act's `Story:` mark — this `(item, stage)`, or the item's `closed` —
    /// is ALREADY on `rk-run/<item>` at `sha`, so the act committed nothing and wrote no row: the
    /// mint is idempotent per `(item, stage)`, and a mark whose row is missing is the heal's.
    AlreadyMarked { sha: String },
    /// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; ADR-0188 decision 2.)* Live, a
    /// step act over a tree the step did not change: no commit was made, and the act's rows were
    /// appended unanchored.
    Unchanged,
}

impl Committed {
    /// The commit the act made or was served, if any.
    #[must_use]
    pub fn sha(&self) -> Option<&str> {
        match self {
            Self::Live { sha, .. } | Self::Replayed { sha, .. } | Self::AlreadyMarked { sha } => {
                Some(sha)
            }
            Self::NoGitWrite | Self::Unchanged => None,
        }
    }
}

/// Why an act or a heal refused. A stable tag per variant (House Shape A).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TierOneError {
    /// Neither a trailer nor content: an empty commit that records nothing is noise on the run ref.
    EmptyAct,
    /// A `Story:` or `Slice-Verdict:` mark and no `[project] name` (ADR-0159).
    NoProject,
    /// A project name containing `/`: every mark would parse under a different project.
    ProjectHasSlash { project: String },
    /// A mark the widened grammar cannot carry — a token with whitespace, a stage with `/`, the
    /// reserved `closed` as a stage — refused at the mint rather than committed as something else.
    Unrenderable { mark: String },
    /// A gate act built from a row that is not a gate transition, or lacks a member of its mark.
    NotAGateRow { detail: String },
    /// Git refused: the authorisation (a typed `ForeignWorktree`), the commit or a re-read.
    Git(GitError),
    /// The store refused a row AFTER the commit — the case [`heal`] repairs at the next entry.
    Store { detail: String },
    /// A recorded `git_op` row that does not read back as an invocation.
    Journal { detail: String },
    /// *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8; the slice record, proposed.)*
    /// The seal's `<slice>` is not a declared slice: refused at the mint and by name (ADR-0159
    /// decision 2's siting; ADR-0160's first bullet), before any git call.
    SliceRefused {
        item: String,
        slice: Option<String>,
        reason: SliceRefusal,
    },
    /// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; ADR-0188 decisions 5-7.)* The
    /// landing refused at least one path, by name: **nothing landed**, no ref moved and no row of
    /// the act was written (the step did not commit). `tree` is ADR-0105 clause 4's
    /// no-checkpoint-due derivation over the run-owned tree, the `landing.refused-path` gate's.
    LandingRefused {
        refusals: Vec<Refusal>,
        tree: String,
    },
    /// *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E; ADR-0188 decision 3.)* A step
    /// checkpoint on the run ref whose row the store lacks, and whose body cannot be healed: the
    /// body is absent, is not canonical JSON, or names a key that disagrees with the subject. The
    /// commit is named; the step is never re-dispatched over its own landed edit.
    StepCheckpointUnreadable { sha: String, detail: String },
}

/// **Why a seal's `<slice>` is refused.**
///
/// ADR-0160's first bullet names five values a `Slice-Verdict:` `<slice>` may never be: *"the item id, the project, the stage, the run id, or
/// any value not declared as a slice"*. Four are refused here by name; the fifth, the run id, cannot
/// be known at the item record and carries a digest, so a declared slice can never equal it (the
/// slice record says so). Whitespace is refused because the value is *"exactly two
/// whitespace-separated tokens"*. *(dated 2026-09-28, `s8e` build, package S8E-P7.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SliceRefusal {
    /// The item record declares no `slice` (or an empty one): *"any value not declared as a slice"*.
    Absent,
    /// The declared slice is the item id (D21-41 e).
    ItemId,
    /// The declared slice is `[project] name`.
    Project,
    /// The declared slice is a stage the pack's `stage_order` names.
    Stage(String),
    /// The declared slice carries whitespace.
    Whitespace,
}

impl TierOneError {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAct => "empty_act",
            Self::NoProject => "no_project",
            Self::ProjectHasSlash { .. } => "project_has_slash",
            Self::Unrenderable { .. } => "unrenderable",
            Self::NotAGateRow { .. } => "not_a_gate_row",
            Self::Git(_) => "git",
            Self::Store { .. } => "store",
            Self::Journal { .. } => "journal",
            Self::SliceRefused { .. } => "slice_refused",
            Self::LandingRefused { .. } => "landing_refused",
            Self::StepCheckpointUnreadable { .. } => "step_checkpoint_unreadable",
        }
    }
}

impl core::fmt::Display for TierOneError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyAct => write!(
                f,
                "a tier-1 act with neither a trailer nor content is refused: an empty commit that \
                 records nothing is noise on the run ref"
            ),
            Self::NoProject => write!(
                f,
                "`.redkiln/config.toml` declares no `[project] name`, and a `Story:` or \
                 `Slice-Verdict:` value is `<project>/<…>`; an empty project degrades it to a bare \
                 id that records no transition, so the mark is refused (ADR-0159)"
            ),
            Self::ProjectHasSlash { project } => write!(
                f,
                "`[project] name` is {project:?}, which contains `/`; the reader splits on the \
                 FIRST slash, so every mark would parse under a different project (ADR-0159)"
            ),
            Self::Unrenderable { mark } => write!(
                f,
                "the mark {mark} cannot be carried by the trailer grammar (a token with \
                 whitespace, a stage with `/`, or the reserved `closed` as a stage), so it is \
                 refused rather than committed as something else (ADR-0183, ADR-0184)"
            ),
            Self::NotAGateRow { detail } => write!(f, "not a gate transition: {detail}"),
            Self::Git(e) => write!(f, "the tier-1 git write was refused: {e:?}"),
            Self::Store { detail } => write!(
                f,
                "the store refused a tier-1 row after its commit (the next entry heals it): \
                 {detail}"
            ),
            Self::Journal { detail } => write!(f, "a recorded `git_op` row: {detail}"),
            Self::LandingRefused { refusals, .. } => {
                let named: Vec<String> = refusals.iter().map(ToString::to_string).collect();
                write!(
                    f,
                    "the landing refused {} path(s), and nothing landed: {} (ADR-0188)",
                    refusals.len(),
                    named.join(", ")
                )
            }
            Self::StepCheckpointUnreadable { sha, detail } => write!(
                f,
                "the step checkpoint {sha} on the run ref anchors no row, and its row cannot be \
                 read back from it: {detail}. The step is not re-dispatched over its own landed \
                 edit (ADR-0188 decision 3)"
            ),
            Self::SliceRefused {
                item,
                slice,
                reason,
            } => {
                let declared = slice.as_deref().unwrap_or_default();
                match reason {
                    SliceRefusal::Absent => write!(
                        f,
                        "the item `{item}` declares no `slice`, so its review outcome cannot be \
                         sealed: a `Slice-Verdict:` `<slice>` is a declared slice and nothing \
                         else, and no default is taken (ADR-0160; `s8e` §8e.8)"
                    ),
                    SliceRefusal::ItemId => write!(
                        f,
                        "the item `{item}` declares `slice = {declared:?}`, which is the item id; \
                         a slice is declared, never the item (D21-41 e; ADR-0160)"
                    ),
                    SliceRefusal::Project => write!(
                        f,
                        "the item `{item}` declares `slice = {declared:?}`, which is the project \
                         name; a slice is never the project (ADR-0160)"
                    ),
                    SliceRefusal::Stage(stage) => write!(
                        f,
                        "the item `{item}` declares `slice = {declared:?}`, which is the stage \
                         `{stage}` the pack's `stage_order` names; a slice is never a stage \
                         (ADR-0160)"
                    ),
                    SliceRefusal::Whitespace => write!(
                        f,
                        "the item `{item}` declares `slice = {declared:?}`, which carries \
                         whitespace; a `Slice-Verdict:` value is exactly two \
                         whitespace-separated tokens (design §5.4)"
                    ),
                }
            }
        }
    }
}

impl std::error::Error for TierOneError {}

impl From<GitError> for TierOneError {
    fn from(e: GitError) -> Self {
        Self::Git(e)
    }
}

// ------------------------------------------------------------------------ the act's git journal

/// The invocations the seam recorded during the act, shared with [`commit_act`] so they become the
/// act's `git_op` rows once — and only once — the commit exists.
#[derive(Debug, Clone, Default)]
pub struct GitOpRecorder {
    recorded: Arc<Mutex<Vec<GitInvocation>>>,
    /// How many recorded invocations a replaying journal holds; `0` is N7's "no git writes".
    served: usize,
}

impl GitOpRecorder {
    fn take(&self) -> Vec<GitInvocation> {
        std::mem::take(&mut *self.recorded.lock().expect(LOCK_HALTS))
    }
}

/// The `GitJournal` the boundary's seam runs over (ADR-0105 clause 1's journal, N7's `git_op`).
#[derive(Debug, Default)]
pub struct ActJournal {
    recorder: GitOpRecorder,
    served: VecDeque<GitInvocation>,
}

impl ActJournal {
    /// A live journal: every invocation is held for the act's `git_op` rows.
    #[must_use]
    pub fn live() -> Self {
        Self::default()
    }

    /// A replaying journal, served from the `git_op` rows of a recorded journal, in order.
    ///
    /// # Errors
    /// [`TierOneError::Journal`] for a `git_op` row that does not read back as an invocation.
    pub fn replaying(events: &[Event]) -> Result<Self, TierOneError> {
        let served: VecDeque<GitInvocation> = events
            .iter()
            .filter(|e| e.kind == EventKind::GitOp)
            .map(invocation_of)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            recorder: GitOpRecorder {
                recorded: Arc::default(),
                served: served.len(),
            },
            served,
        })
    }

    /// The handle [`commit_act`] reads the act's invocations through.
    #[must_use]
    pub fn recorder(&self) -> GitOpRecorder {
        self.recorder.clone()
    }
}

impl GitJournal for ActJournal {
    fn record(&mut self, inv: GitInvocation) {
        self.recorder.recorded.lock().expect(LOCK_HALTS).push(inv);
    }

    fn next_recorded(&mut self) -> Option<GitInvocation> {
        self.served.pop_front()
    }
}

/// The tier-2 `git_op` row for one invocation: `GitInvocation`'s own members, keyed to the item.
#[must_use]
pub fn git_op_event(inv: &GitInvocation, item: &str) -> Event {
    Event::new(
        EventKind::GitOp,
        serde_json::json!({
            "argv_digest": inv.argv_digest.as_str(),
            "cwd": inv.cwd,
            "op": inv.op,
            "status": inv.status,
            "stderr_digest": inv.stderr_digest.as_str(),
            "stdout": String::from_utf8_lossy(&inv.stdout),
        }),
    )
    .with_item(item)
}

fn invocation_of(row: &Event) -> Result<GitInvocation, TierOneError> {
    let bad = |m: &str| TierOneError::Journal {
        detail: format!("a `git_op` row's `{m}` does not read back: {}", row.payload),
    };
    let text = |m: &str| row.payload.get(m).and_then(serde_json::Value::as_str);
    let op = text("op")
        .and_then(GitOp::from_name)
        .ok_or_else(|| bad("op"))?;
    let digest = |m: &str| {
        text(m)
            .and_then(|h| Digest::from_hex(h).ok())
            .ok_or_else(|| bad(m))
    };
    Ok(GitInvocation {
        op: op.as_str(),
        cwd: text("cwd").ok_or_else(|| bad("cwd"))?.to_owned(),
        argv_digest: digest("argv_digest")?,
        status: row
            .payload
            .get("status")
            .and_then(serde_json::Value::as_i64)
            .and_then(|s| i32::try_from(s).ok())
            .ok_or_else(|| bad("status"))?,
        stdout: text("stdout")
            .ok_or_else(|| bad("stdout"))?
            .as_bytes()
            .to_vec(),
        stderr_digest: digest("stderr_digest")?,
    })
}

// -------------------------------------------------------------------------------- the one act

/// The trailer an act's mark renders to, through `rk-git`'s round-trip-checked renderer.
fn render(mark: &Mark, target: &Target) -> Result<rk_git::trailer::grammar::Trailer, TierOneError> {
    let project = || -> Result<String, TierOneError> {
        let project = target.project.clone().ok_or(TierOneError::NoProject)?;
        if project.contains('/') {
            return Err(TierOneError::ProjectHasSlash { project });
        }
        Ok(project)
    };
    let parsed = match mark {
        Mark::Stage(stage) => ParsedTrailer::Story {
            project: project()?,
            slug: target.item.clone(),
            stage: Some(StoryStage::Stage(stage.clone())),
        },
        Mark::Closed => ParsedTrailer::Story {
            project: project()?,
            slug: target.item.clone(),
            stage: Some(StoryStage::Closed),
        },
        Mark::Gate {
            token,
            tree_hash,
            deadline,
            kind,
            reach,
            state,
        } => ParsedTrailer::Gate {
            token: token.clone(),
            tree_hash: tree_hash.clone(),
            deadline: *deadline,
            kind: kind.clone(),
            reach: *reach,
            state: state.clone(),
        },
        Mark::Seal { slice, verdict } => ParsedTrailer::SliceVerdict {
            project: project()?,
            slice: slice.clone(),
            verdict: *verdict,
        },
    };
    parsed
        .to_trailer()
        .ok_or_else(|| TierOneError::Unrenderable {
            mark: format!("{mark:?}"),
        })
}

/// A colon-free subject: git's own `%(trailers)` reads a one-paragraph message's last paragraph,
/// and a subject carrying `": "` would read as a key (§8e.13 finding 9).
fn subject(mark: &Mark, item: &str) -> String {
    match mark {
        Mark::Stage(stage) => format!("{item} enters {stage}"),
        Mark::Closed => format!("{item} lane closed"),
        Mark::Gate { state, .. } => format!("{item} gate {}", state.as_str()),
        Mark::Seal { .. } => format!("{item} slice sealed"),
    }
}

/// **The one boundary function** — see the module header for its four steps and why their order
/// is the criterion.
///
/// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; the landing record ADR-0188,
/// proposed.)* The store is locked for the whole act, so an act is one critical section; the body
/// is [`land_act`], which a caller that already holds the store's guard calls through
/// [`Boundary::act_locked`] (the step-grain act, AC-012).
///
/// # Errors
/// [`TierOneError::EmptyAct`] for an act with neither trailer nor content; a render refusal; the
/// typed [`TierOneError::Git`] for a foreign target or a failed commit (**no row is written**);
/// [`TierOneError::LandingRefused`] when the landing refuses a path (nothing lands, no row); or
/// [`TierOneError::Store`] when the store refuses a row after the commit (the heal's case).
pub fn commit_act(
    git: &mut GitOps<'_>,
    recorder: &GitOpRecorder,
    store: &Mutex<dyn EventStore>,
    target: &Target,
    act: Act,
) -> Result<Committed, TierOneError> {
    let mut guard = store.lock().expect(LOCK_HALTS);
    land_act(
        git,
        recorder,
        &mut *guard,
        target,
        &BTreeSet::new(),
        None,
        act,
    )
}

/// Where the act's baseline comes from (ADR-0188 decision 4).
#[derive(Debug, Clone, PartialEq, Eq)]
enum BaselineRead {
    /// The item's journal holds it: the first `status` `git_op` row after the latest
    /// `worktree_assigned` row whose `reused` is `false`.
    Journalled(BTreeSet<String>),
    /// The tree was created and no act has read it since: THIS act takes the baseline, as its
    /// first journalled invocation.
    ThisAct,
    /// No baseline can be read (no creation row, or a reused tree whose creation's read is not in
    /// the journal — a deleted database): fail-closed.
    Unknown,
}

/// **The baseline, read from the item's journal** (ADR-0188 decision 4).
fn baseline_of(events: &[Event], item: &str) -> BaselineRead {
    let assigned = |e: &Event| {
        e.kind == EventKind::WorktreeAssigned
            && e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
    };
    let latest = events.iter().rposition(assigned);
    let created = events.iter().rposition(|e| {
        assigned(e) && e.payload.get("reused").and_then(serde_json::Value::as_bool) == Some(false)
    });
    let Some(created) = created else {
        return BaselineRead::Unknown;
    };
    let read = events[created + 1..].iter().find(|e| {
        e.kind == EventKind::GitOp
            && e.payload.get("op").and_then(serde_json::Value::as_str)
                == Some(GitOp::Status.as_str())
            && e.payload.get("status").and_then(serde_json::Value::as_i64) == Some(0)
    });
    match read {
        Some(row) => {
            let stdout = row
                .payload
                .get("stdout")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();
            BaselineRead::Journalled(
                landing::parse_status(stdout.as_bytes())
                    .into_iter()
                    .filter(landing::StatusEntry::is_untracked)
                    .map(|e| e.path)
                    .collect(),
            )
        }
        None if latest == Some(created) => BaselineRead::ThisAct,
        None => BaselineRead::Unknown,
    }
}

/// **The journalled baseline, for the re-entry discard** (ADR-0188 decisions 4 and 13): the set
/// the item's journal records, or `None` when none is recorded (no creation row, a creation no act
/// has read yet, or a deleted database). The discard keeps a recorded baseline's paths, since the
/// run did not create them, and with `None` it discards as it did before `s8f`.
/// *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E.)*
#[must_use]
pub fn journalled_baseline(events: &[Event], item: &str) -> Option<BTreeSet<String>> {
    match baseline_of(events, item) {
        BaselineRead::Journalled(set) => Some(set),
        BaselineRead::ThisAct | BaselineRead::Unknown => None,
    }
}

/// **The act, over a store the caller holds** — the body of [`commit_act`].
///
/// Live: authorise; the `Story:` idempotence read; the baseline; the landing's staging (a refusal
/// returns here, before any commit and any row); a gate opening's tree becomes the landing's T and
/// its token is re-minted over it (defect (b), ADR-0188 decision 12); the commit (content when due;
/// empty for a boundary otherwise; none for a step otherwise); then every row and every `git_op`
/// row, anchored to the commit, or unanchored when a step made none.
fn land_act(
    git: &mut GitOps<'_>,
    recorder: &GitOpRecorder,
    store: &mut dyn EventStore,
    target: &Target,
    excluded: &BTreeSet<String>,
    // *(dated 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED; AC-525's second AND;
    // ADR-0025's *"the next gate the item reaches carries an advisory note"*; ADR-0189 decision
    // 10.)* The run's advisories not yet carried: a gate OPENED by this act carries them
    // (`carried_by_gate`), and they are taken once its rows are appended.
    pending: Option<&super::advisory::PendingAdvisories>,
    mut act: Act,
) -> Result<Committed, TierOneError> {
    // One arm may carry no trailer: the step checkpoint (N10). *(dated 2026-09-29, s8f build,
    // package R5FIX2-8f; D21-114.)* ~~One arm~~ Two: the entry landing too.
    if act.mark.is_none() && !matches!(act.content, Content::Step(_) | Content::Entry(_)) {
        return Err(TierOneError::EmptyAct);
    }
    let trailer = act.mark.as_ref().map(|m| render(m, target)).transpose()?;
    let replaying = git.mode() == Mode::Replay;

    // 1. LIVE: the target is the run-owned tree the STORE records, on its branch — or the write is
    //    refused, typed, naming all four values (AC-529). REPLAY: no authorisation read at all (it
    //    would reach git); N7's rule first — a journal with no `git_op` row made no git write, so
    //    none is served.
    let cwd = if replaying {
        if recorder.served == 0 {
            return Ok(Committed::NoGitWrite);
        }
        WorktreePath::new(&target.worktree)?
    } else {
        // Nothing an earlier act left behind may be anchored to this one.
        drop(recorder.take());
        authorise_tier1_write(git.exec(), &target.worktree, &target.ownership)?
            .path()
            .clone()
    };

    // The baseline (ADR-0188 decision 4), read from the item's journal before any git call.
    let events = store
        .query(Query::ByItem(target.item.clone()))
        .map_err(|e| TierOneError::Store {
            detail: format!("{e:?}"),
        })?;
    let baseline_read = baseline_of(&events, &target.item);

    // 1b. *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.5; ADR-0183 decision 6,
    //     proposed.)* LIVE, a `Story:` mark: the idempotence read is *"this `(item, stage)` already
    //     committed"* (not ADR-0155's *"item already committed"*), read off git itself. A mark
    //     already on the run ref commits nothing and writes no row.
    if !replaying
        && let Some(stage) = act.mark.as_ref().and_then(story_stage_of)
        && let Some(sha) = stage_already_marked(git.exec(), &cwd, target, &stage)?
    {
        // *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)* An already-marked entry
        // that is the first act after creation still takes the baseline, so a later step's
        // creations are never read as the baseline; its one `status` row is journalled
        // unanchored, and nothing else is written.
        if baseline_read == BaselineRead::ThisAct {
            landing::staging_set(
                git,
                &cwd,
                &Baseline::Known(BTreeSet::new()),
                &BTreeSet::new(),
            )?;
            for inv in recorder.take() {
                store
                    .append(git_op_event(&inv, &target.item))
                    .map_err(|e| TierOneError::Store {
                        detail: format!("{e:?}"),
                    })?;
            }
        }
        return Ok(Committed::AlreadyMarked { sha });
    }

    let baseline = match baseline_read {
        BaselineRead::Journalled(set) => Baseline::Known(set),
        BaselineRead::Unknown => Baseline::Unknown,
        BaselineRead::ThisAct => {
            // The first act after creation: its first invocation IS the baseline's read.
            let first = landing::staging_set(
                git,
                &cwd,
                &Baseline::Known(BTreeSet::new()),
                &BTreeSet::new(),
            )?;
            Baseline::Known(first.untracked.into_iter().collect())
        }
    };

    // 2. Stage (or, for the empty arm, take HEAD's tree), then the commit, through the journalled
    //    seam (on replay, SERVED from it). A refusal or a failure returns BEFORE any row: AC-162
    //    by construction, and a refused landing lands nothing.
    let staged = match &act.content {
        Content::Empty => landing::prepare_empty(git, &cwd, &target.item)?,
        Content::Landed | Content::Step(_) | Content::Entry(_) => landing::stage(
            git,
            &Request {
                cwd: &cwd,
                item: &target.item,
                baseline: &baseline,
                excluded,
            },
        )?,
    };
    let prepared = match staged {
        Staged::Ready(prepared) => prepared,
        Staged::Refused { refusals, tree, .. } => {
            drop(recorder.take());
            return Err(TierOneError::LandingRefused { refusals, tree });
        }
    };

    // Defect (b): a gate OPENED by a landing act binds the landing's T over the run-owned tree,
    // and its token is re-minted over that tree (the token is derived from it, so the fold reads
    // the row back as this record).
    let mut trailer = trailer;
    if matches!(act.content, Content::Landed)
        && let Some(Mark::Gate {
            token,
            tree_hash,
            deadline,
            kind,
            reach,
            state: GateState::Opened,
        }) = act.mark.as_mut()
    {
        let landed_tree = prepared.tree().to_owned();
        let record = GateRecord::derive_reach(
            kind,
            &landed_tree,
            rk_types::time::Timestamp::from_unix_seconds(*deadline),
            *reach,
        )
        .map_err(|e| TierOneError::NotAGateRow {
            detail: format!("the gate's token cannot be minted over the landed tree: {e:?}"),
        })?;
        *token = record.token().as_str().to_owned();
        *tree_hash = landed_tree.clone();
        let minted = token.clone();
        for row in &mut act.rows {
            if row.kind == EventKind::GateOpened
                && let Some(map) = row.payload.as_object_mut()
            {
                map.insert("token".to_owned(), serde_json::Value::from(minted.clone()));
                map.insert(
                    "tree_hash".to_owned(),
                    serde_json::Value::from(landed_tree.clone()),
                );
            }
        }
        trailer = act.mark.as_ref().map(|m| render(m, target)).transpose()?;
    }

    let (message, allow_empty) = match (&act.mark, &act.content) {
        (Some(mark), _) => (
            landing::Message {
                subject: subject(mark, &target.item),
                body: None,
                trailer,
            },
            true,
        ),
        (None, Content::Step(checkpoint)) => (
            landing::Message {
                subject: checkpoint.subject.clone(),
                body: Some(checkpoint.body.clone()),
                trailer: None,
            },
            false,
        ),
        // *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-114.)* The entry landing: the
        // subject only, and no commit when the tree is unchanged.
        (None, Content::Entry(entry)) => (
            landing::Message {
                subject: entry.subject.clone(),
                body: None,
                trailer: None,
            },
            false,
        ),
        (None, _) => return Err(TierOneError::EmptyAct),
    };
    let landed = landing::commit(git, &prepared, &message, allow_empty);

    if replaying {
        return match landed? {
            Landed::Commit { sha, tree, .. } => Ok(Committed::Replayed { sha, tree }),
            Landed::Nothing { .. } => Ok(Committed::NoGitWrite),
        };
    }
    let invocations = recorder.take();
    let landed = landed?;

    // The next gate the item reaches carries the run's advisories not yet carried (AC-525's
    // second AND): beside the members the token binds, so the token does not move.
    let carried: Vec<super::advisory::Advisory> = match (pending, &act.mark) {
        (
            Some(pending),
            Some(Mark::Gate {
                state: GateState::Opened,
                ..
            }),
        ) => pending.lock().expect(LOCK_HALTS).clone(),
        _ => Vec::new(),
    };
    if !carried.is_empty() {
        act.rows = act
            .rows
            .into_iter()
            .map(|row| {
                if row.kind == EventKind::GateOpened {
                    super::advisory::carried_by_gate(&row, &carried)
                } else {
                    row
                }
            })
            .collect();
    }

    // 4. Every row of the act, then its `git_op` rows, each anchored to the one commit — or, for a
    //    step that changed nothing, appended unanchored as a tier-2 row is.
    let rows = act
        .rows
        .into_iter()
        .map(|row| keyed(row, &target.item))
        .chain(
            invocations
                .iter()
                .map(|inv| git_op_event(inv, &target.item)),
        );
    match landed {
        Landed::Commit { sha, tree, .. } => {
            let mut anchor = TierOneAnchor::new(&sha, &tree);
            if let Some((stage, run_id)) = &act.opening_run {
                anchor = anchor.with_opening_run(stage, run_id);
            }
            for row in rows {
                store
                    .append_tier1(row, &anchor)
                    .map_err(|e| TierOneError::Store {
                        detail: format!("{e:?}"),
                    })?;
            }
            if let Some(pending) = pending.filter(|_| !carried.is_empty()) {
                let mut left = pending.lock().expect(LOCK_HALTS);
                let n = carried.len().min(left.len());
                left.drain(..n);
            }
            Ok(Committed::Live { sha, tree })
        }
        Landed::Nothing { .. } => {
            for row in rows {
                store.append(row).map_err(|e| TierOneError::Store {
                    detail: format!("{e:?}"),
                })?;
            }
            Ok(Committed::Unchanged)
        }
    }
}

/// The `Story:` stage a mark carries, if it is a `Story:` mark.
fn story_stage_of(mark: &Mark) -> Option<StoryStage> {
    match mark {
        Mark::Stage(stage) => Some(StoryStage::Stage(stage.clone())),
        Mark::Closed => Some(StoryStage::Closed),
        Mark::Gate { .. } | Mark::Seal { .. } => None,
    }
}

/// **The mint's idempotence read**: the commit already carrying this `(item, stage)` mark.
///
/// (`s8e` §8e.5; ADR-0183 decision 6, proposed.) The commit on `rk-run/<item>` that already carries `Story: <project>/<item> <stage>` for THIS `(item, stage)`,
/// if any. ~~ADR-0155's read was *"item already committed"*~~ — the first boundary only. Every
/// git call here is a read. *(dated 2026-09-28, `s8e` build, package S8E-P6.)*
///
/// # Errors
/// [`TierOneError::Git`] from a read.
pub fn stage_already_marked(
    exec: &GitExec,
    repo: &WorktreePath,
    target: &Target,
    stage: &StoryStage,
) -> Result<Option<String>, TierOneError> {
    let reachable = reachable_from_run_ref(exec, repo, &target.item)?;
    if reachable.is_empty() {
        return Ok(None);
    }
    let found = read_commits(exec, repo)?
        .into_iter()
        .filter(|c| reachable.contains(&c.sha))
        .find(|c| {
            c.trailers.iter().any(|t| {
                matches!(t, ParsedTrailer::Story { project, slug, stage: Some(s) }
                    if Some(project) == target.project.as_ref()
                        && *slug == target.item
                        && s == stage)
            })
        })
        .map(|c| c.sha);
    Ok(found)
}

/// **The lane's end, as one act**: the `closed` mark and its one `stage_transition` row.
///
/// (`s8e` §8e.6; D21-43 N9; ADR-0183 decision 2 and its dated decision 10, proposed.) The terminal `Story: <project>/<item> closed` mark, with the empty
/// content arm at Checkpoint 8e, anchoring ONE tier-1 row — `stage_transition { from: <the stage
/// the lane ended in>, to: "closed" }`. `closed` is reserved (no pack may declare a stage of that
/// name), so the row names no stage, and a deleted store rebuilds it from the mark alone.
/// *(dated 2026-09-28, `s8e` build, package S8E-P6.)*
#[must_use]
pub fn close_act(last_stage: &str) -> Act {
    Act::marked(
        Mark::Closed,
        vec![super::stage_transition_event(
            last_stage,
            rk_git::trailer::grammar::CLOSED_STAGE_TOKEN,
        )],
    )
}

// ------------------------------------------------------------ the refused landing's gate

/// The gate class a refused landing opens (ADR-0188 decision 6). *(dated 2026-09-28, s8f build,
/// package S8F-P3-LANDING-E2E.)*
pub const LANDING_REFUSED_KIND: &str = "landing.refused-path";
/// Its option that leaves exactly the refused paths unstaged at every later landing of the lane.
pub const LANDING_EXCLUDE: &str = "exclude";
/// Its option that stops the lane.
pub const LANDING_ABORT: &str = "abort";

/// **The refused landing's question** (ADR-0129 decisions 3-4; ADR-0188 decision 6): `exclude` or
/// `abort`, neither attaching anything. The kernel owns it, as it owns the default question; the
/// paths and rules it asks about are on the gate's `gate_opened` row (`refused`).
#[must_use]
pub fn landing_question() -> crate::workflow::question::Question {
    crate::workflow::question::Question::new([LANDING_EXCLUDE, LANDING_ABORT]).with_text(
        "The landing refused paths by name, and nothing landed. `exclude`: every later landing of \
         this lane leaves exactly those paths unstaged, naming them each time. `abort`: the lane \
         stops.",
    )
}

/// **The refused landing's gate record** — class [`LANDING_REFUSED_KIND`], bound to `tree`, the
/// refusal's no-checkpoint-due derivation over the run-owned tree (ADR-0188 decision 7), never
/// the empty commit's tree.
///
/// # Errors
/// [`TierOneError::NotAGateRow`] when the token cannot be minted.
pub fn landing_gate_record(
    tree: &str,
    deadline: rk_types::time::Timestamp,
    reach: u32,
) -> Result<GateRecord, TierOneError> {
    GateRecord::derive_reach(LANDING_REFUSED_KIND, tree, deadline, reach).map_err(|e| {
        TierOneError::NotAGateRow {
            detail: format!("the refused landing's gate token cannot be minted: {e:?}"),
        }
    })
}

/// **The refused landing's gate, as one act** (ADR-0188 decisions 6-7): its `Gate: … opened` mark
/// rides an EMPTY commit (no checkpoint is due at a refused landing, so nothing is staged), and its
/// `gate_opened` row names every refused path with its rule under `refused`, in the spellings the
/// record fixes (`denylist:<glob>`, `size:<bytes>`, `path-length:<bound>`, `baseline-unknown`).
#[must_use]
pub fn refused_landing_act(record: &GateRecord, refusals: &[Refusal]) -> Act {
    let mut row = super::gate_opened_event(record);
    if let Some(map) = row.payload.as_object_mut() {
        map.insert(
            "refused".to_owned(),
            serde_json::Value::Array(
                refusals
                    .iter()
                    .map(|r| serde_json::json!({ "path": r.path, "rule": r.rule.to_string() }))
                    .collect(),
            ),
        );
    }
    Act::marked_empty(
        Mark::Gate {
            token: record.token().as_str().to_owned(),
            tree_hash: record.tree_hash().to_owned(),
            deadline: record.deadline().unix_seconds(),
            kind: record.kind().to_owned(),
            reach: record.reach(),
            state: GateState::Opened,
        },
        vec![row],
    )
}

/// What the lane's answered refused-landing gates say (ADR-0188 decision 6).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LandingAnswers {
    /// Every path an applied `exclude` answer leaves unstaged.
    pub excluded: BTreeSet<String>,
    /// The token of an applied `abort` answer, if one was given: the lane is stopped.
    pub aborted: Option<String>,
}

/// **Read the lane's refused-landing answers** off the fold and the journal: an APPLIED answer
/// (the fold's answered set, which excludes a void) to a gate of [`LANDING_REFUSED_KIND`]. An
/// `exclude` contributes the paths its gate's `gate_opened` row names; an `abort` stops the lane.
/// A gate healed from git carries no `refused` list, so its `exclude` excludes nothing and the
/// next landing refuses again, by name (fail-closed).
#[must_use]
pub fn landing_answers(state: &crate::workflow::fsm::RunState, events: &[Event]) -> LandingAnswers {
    let mut out = LandingAnswers::default();
    for answer in state.answered_gates() {
        let token = answer.token();
        if state.gate_record(token).map(GateRecord::kind) != Some(LANDING_REFUSED_KIND) {
            continue;
        }
        match answer.verdict() {
            LANDING_EXCLUDE => {
                for row in events.iter().filter(|e| {
                    e.kind == EventKind::GateOpened
                        && e.payload.get("token").and_then(serde_json::Value::as_str)
                            == Some(token.as_str())
                }) {
                    out.excluded.extend(
                        row.payload
                            .get("refused")
                            .and_then(serde_json::Value::as_array)
                            .into_iter()
                            .flatten()
                            .filter_map(|r| r.get("path").and_then(serde_json::Value::as_str))
                            .map(str::to_owned),
                    );
                }
            }
            LANDING_ABORT => out.aborted = Some(token.as_str().to_owned()),
            _ => {}
        }
    }
    out
}

// ------------------------------------------------------------------------- the drift gate

/// The drift gate's class (`crate::workflow::fsm::DRIFT_GATE_KIND`), re-exported beside the
/// landing gate's. *(Added 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED.)*
pub const DRIFT_KIND: &str = crate::workflow::fsm::DRIFT_GATE_KIND;
/// Its option that lets the restore stand (ADR-0189 decision 5).
pub const DRIFT_CONFIRM_RESTORE: &str = "confirm-restore";
/// The prefix of its per-section option: `move-to-prose:<section>` moves the drifted bytes into
/// the named prose section at the next entry (ADR-0189 decision 5). One option per prose section
/// `rk-store`'s templates declare, since an option carries no argument.
pub const DRIFT_MOVE_TO_PROSE: &str = "move-to-prose:";

/// **The drift gate's question** (ADR-0129 decisions 3-4; ADR-0189 decision 5): `confirm-restore`,
/// or `move-to-prose:<section>` for each prose section of `rk-store`'s human-facing templates.
/// Neither attaches anything. The kernel owns it, as it owns the refused landing's.
#[must_use]
pub fn drift_question() -> crate::workflow::question::Question {
    let sections: BTreeSet<String> = rk_store::render::TEMPLATES
        .iter()
        .flat_map(|t| t.prose.iter())
        .map(|p| format!("{DRIFT_MOVE_TO_PROSE}{p}"))
        .collect();
    crate::workflow::question::Question::new(
        std::iter::once(DRIFT_CONFIRM_RESTORE.to_owned()).chain(sections),
    )
    .with_text(
        "A structured section of a store-rendered artifact was edited, recorded as \
         `structured_drift` and restored from the type. `confirm-restore`: the restore stands. \
         `move-to-prose:<section>`: at the next entry the drifted text, read from the causing \
         commit, is appended to that prose section. Until answered, the stage does not exit.",
    )
}

/// **The drift gate's record** — class [`DRIFT_KIND`], over `tree` (re-minted over the landing's
/// T when the act lands, defect (b)).
///
/// # Errors
/// [`TierOneError::NotAGateRow`] when the token cannot be minted.
pub fn drift_gate_record(
    tree: &str,
    deadline: rk_types::time::Timestamp,
    reach: u32,
) -> Result<GateRecord, TierOneError> {
    GateRecord::derive_reach(DRIFT_KIND, tree, deadline, reach).map_err(|e| {
        TierOneError::NotAGateRow {
            detail: format!("the drift gate's token cannot be minted: {e:?}"),
        }
    })
}

/// **The drift gate, as one act** (ADR-0189 decision 4): its `Gate: … opened` mark rides the
/// LANDED arm, so its commit is the checkpoint that lands the restore; its `gate_opened` row
/// names the stage it holds (`stage`) and every drift it was opened over (`drifts`, each a
/// `structured_drift` payload), beside the members the token binds.
///
/// # Errors
/// As [`Act::gate_opened`].
pub fn drift_gate_act(
    record: &GateRecord,
    stage: &str,
    drifts: Vec<serde_json::Value>,
) -> Result<Act, TierOneError> {
    let mut row = super::gate_opened_event(record);
    if let Some(map) = row.payload.as_object_mut() {
        map.insert("stage".to_owned(), serde_json::Value::from(stage));
        map.insert("drifts".to_owned(), serde_json::Value::Array(drifts));
    }
    Act::gate_opened(row)
}

/// An open drift gate that holds a stage's exit: its token and the drifted artifacts it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenDriftGate {
    pub token: String,
    pub stage: String,
    /// `<kind>` at `<anchor>`, one per drift the gate was opened over.
    pub artifacts: Vec<String>,
}

impl std::fmt::Display for OpenDriftGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the stage `{}` does not exit: its `{DRIFT_KIND}` gate {} is unanswered, over the \
             drifted artifact(s) {}; answer it with `{DRIFT_CONFIRM_RESTORE}` or \
             `{DRIFT_MOVE_TO_PROSE}<section>` (ADR-0189 decision 4; N11). No boundary commit is \
             made and no row is appended",
            self.stage,
            self.token,
            self.artifacts.join(", ")
        )
    }
}

/// **The stage advance's drift check** (`s8f` §8f.5; D21-47, cross #16; ADR-0189 decision 4): the
/// first gate of class [`DRIFT_KIND`] for the item at `stage` whose latest state is OPENED (not
/// answered, void or expired), read off the fold and the journal's `gate_opened` rows. A gate
/// whose row carries no `stage` (one healed from git after a deleted database) holds every stage,
/// fail-closed. `None`: the stage may exit.
#[must_use]
pub fn open_drift_gate(
    state: &crate::workflow::fsm::RunState,
    events: &[Event],
    stage: &str,
) -> Option<OpenDriftGate> {
    state.open_gates().find_map(|record| {
        let token = record.token();
        if record.kind() != DRIFT_KIND
            || state.is_answered(token)
            || state.is_void(token)
            || state.is_expired(token)
        {
            return None;
        }
        let row = events.iter().rev().find(|e| {
            e.kind == EventKind::GateOpened
                && e.payload.get("token").and_then(serde_json::Value::as_str)
                    == Some(token.as_str())
        });
        let gate_stage = row
            .and_then(|r| r.payload.get("stage"))
            .and_then(serde_json::Value::as_str);
        if gate_stage.is_some_and(|s| s != stage) {
            return None;
        }
        let artifacts = row
            .and_then(|r| r.payload.get("drifts"))
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .map(|d| {
                let text = |m: &str| {
                    d.get(m)
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("?")
                        .to_owned()
                };
                format!("`{}` at `{}`", text("kind"), text("anchor"))
            })
            .collect::<Vec<_>>();
        Some(OpenDriftGate {
            token: token.as_str().to_owned(),
            stage: gate_stage.unwrap_or(stage).to_owned(),
            artifacts: if artifacts.is_empty() {
                vec!["(not named: the gate's row was healed from git)".to_owned()]
            } else {
                artifacts
            },
        })
    })
}

/// One applied `move-to-prose:<section>` answer (ADR-0189 decision 5): the gate's token, the
/// answer row's position in the journal, the prose section, and every drift the gate named.
#[derive(Debug, Clone, PartialEq)]
pub struct MoveToProse {
    pub token: String,
    pub answered_at: usize,
    pub section: String,
    pub drifts: Vec<serde_json::Value>,
}

/// **The drift gates answered `move-to-prose:<section>`**, read off the fold's answered set (an
/// APPLIED answer; a void is not one) and the journal. The composition root applies each at the
/// next entry through `rk-store`'s writer, once.
#[must_use]
pub fn drift_moves(state: &crate::workflow::fsm::RunState, events: &[Event]) -> Vec<MoveToProse> {
    let mut out = Vec::new();
    for answer in state.answered_gates() {
        let token = answer.token();
        if state.gate_record(token).map(GateRecord::kind) != Some(DRIFT_KIND) {
            continue;
        }
        let Some(section) = answer.verdict().strip_prefix(DRIFT_MOVE_TO_PROSE) else {
            continue;
        };
        let names = |e: &Event, kind: EventKind| {
            e.kind == kind
                && e.payload.get("token").and_then(serde_json::Value::as_str)
                    == Some(token.as_str())
        };
        let Some(answered_at) = events
            .iter()
            .rposition(|e| names(e, EventKind::GateAnswered))
        else {
            continue;
        };
        let drifts = events
            .iter()
            .rev()
            .find(|e| names(e, EventKind::GateOpened))
            .and_then(|e| e.payload.get("drifts"))
            .and_then(serde_json::Value::as_array)
            .cloned()
            .unwrap_or_default();
        out.push(MoveToProse {
            token: token.as_str().to_owned(),
            answered_at,
            section: section.to_owned(),
            drifts,
        });
    }
    out
}

/// **The one slice validator.**
///
/// Every refusal ADR-0160's first bullet names, by name, shared by the mint ([`seal_act`]) and the
/// lane's entry check (`rk-cli`'s `run`, D21-94 / D21-95), so the two sitings cannot disagree.
/// *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; the slice record as amended,
/// proposed.)*
///
/// `declared` is the item record's `slice`, `project` its `[project] name`, `stages` the pack's
/// `stage_order`. The order of the checks is the mint's: absent (or empty), whitespace, the item
/// id, the project, a stage. An ABSENT project skips only the project comparison: that absence is
/// ADR-0159's `NoProject`, refused at the mint and never at entry (*"a run that mints no trailer is
/// not affected by the key's absence"*). Returns the declared slice.
///
/// # Errors
/// [`TierOneError::SliceRefused`] for each value ADR-0160 names.
pub fn validate_slice<'a>(
    declared: Option<&'a str>,
    item: &str,
    project: Option<&str>,
    stages: &[String],
) -> Result<&'a str, TierOneError> {
    let refuse = |reason: SliceRefusal| TierOneError::SliceRefused {
        item: item.to_owned(),
        slice: declared.map(ToOwned::to_owned),
        reason,
    };
    let Some(slice) = declared.filter(|s| !s.trim().is_empty()) else {
        return Err(refuse(SliceRefusal::Absent));
    };
    if slice.chars().any(char::is_whitespace) {
        return Err(refuse(SliceRefusal::Whitespace));
    }
    if slice == item {
        return Err(refuse(SliceRefusal::ItemId));
    }
    if project == Some(slice) {
        return Err(refuse(SliceRefusal::Project));
    }
    if let Some(named) = stages.iter().find(|s| s.as_str() == slice) {
        return Err(refuse(SliceRefusal::Stage(named.clone())));
    }
    Ok(slice)
}

/// **A review outcome's seal, as one act.**
///
/// `Slice-Verdict: <project>/<slice> <verdict>` and its one `slice_sealed` row, or a refusal at the mint (`s8e` §8e.8; D21-41 e; the slice record,
/// proposed). *(dated 2026-09-28, `s8e` build, package S8E-P7.)*
///
/// `declared` is the item record's `slice`; `stages` is the pack's `stage_order`; `stage` is the
/// stage whose exit outcome is sealed — the review stage the pass ran in, which the row carries so
/// the fold scopes the seal to it (D21-93: `terminal()` reads `Sealed` only while the fold is in
/// that stage). Every refusal is typed ([`TierOneError::SliceRefused`]) and taken BEFORE any git
/// call: the boundary is never reached with an undeclared slice.
///
/// # Errors
/// [`TierOneError::SliceRefused`] for each value ADR-0160 names; [`TierOneError::NoProject`] when
/// `[project] name` is absent (ADR-0159).
pub fn seal_act(
    declared: Option<&str>,
    item: &str,
    project: Option<&str>,
    stages: &[String],
    stage: &str,
    verdict: Verdict,
) -> Result<Act, TierOneError> {
    // *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* The refusals
    // are the ONE validator the lane's entry check calls too, so the entry and this backstop
    // cannot disagree. `[project] name`'s absence stays ADR-0159's, sited here at the mint.
    let slice = validate_slice(declared, item, project, stages)?;
    let project = project.ok_or(TierOneError::NoProject)?;
    Ok(Act::marked(
        Mark::Seal {
            slice: slice.to_owned(),
            verdict,
        },
        vec![Event::new(
            EventKind::SliceSealed,
            serde_json::json!({
                "project": project,
                "slice": slice,
                "stage": stage,
                "verdict": verdict.as_str(),
            }),
        )],
    ))
}

/// The interim item key (ADR-0182) on a row whose kind is item-scoped and carries none yet.
fn keyed(row: Event, item: &str) -> Event {
    if row.item().is_none() && row.kind.item_key() == ItemKey::Carried {
        row.with_item(item)
    } else {
        row
    }
}

/// **The seam the runtime binds**: the wrapper, the act journal, the target, and the mode.
///
/// It owns what [`commit_act`] borrows, so a `Runtime` can hold it for a whole stage and hand each
/// act through the one function.
#[derive(Debug)]
pub struct Boundary {
    exec: GitExec,
    journal: ActJournal,
    target: Target,
    mode: Mode,
    writes: usize,
    /// The paths the lane's `landing.refused-path` `exclude` answers leave unstaged (ADR-0188
    /// decision 6). *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
    excluded: BTreeSet<String>,
    /// The run's advisories the next gate opened through this boundary (or a sibling) carries
    /// (AC-525's second AND; ADR-0189 decision 10). *(dated 2026-09-28, s8f build, package
    /// S8F-P5-DRIFT-GATE-VOUCHED.)*
    advisories: Option<super::advisory::PendingAdvisories>,
}

impl Boundary {
    /// A live boundary over `target`.
    #[must_use]
    pub fn live(exec: GitExec, target: Target) -> Self {
        Self {
            exec,
            journal: ActJournal::live(),
            target,
            mode: Mode::Live,
            writes: 0,
            excluded: BTreeSet::new(),
            advisories: None,
        }
    }

    /// The same boundary, whose gate openings carry the run's `pending` advisories
    /// (`carried_by_gate`). *(Added 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED.)*
    #[must_use]
    pub fn with_advisories(mut self, pending: super::advisory::PendingAdvisories) -> Self {
        self.advisories = Some(pending);
        self
    }

    /// The same boundary, leaving `excluded` unstaged at every landing (ADR-0188 decision 6).
    #[must_use]
    pub fn with_excluded(mut self, excluded: BTreeSet<String>) -> Self {
        self.excluded = excluded;
        self
    }

    /// What this boundary writes against.
    #[must_use]
    pub const fn target(&self) -> &Target {
        &self.target
    }

    /// A second boundary over the same wrapper, target and mode, with its own act journal — for an
    /// act made after the first boundary was handed to `resume` (the review pass's seal, `s8e`
    /// §8e.8). *(dated 2026-09-28, `s8e` build, package S8E-P7.)*
    #[must_use]
    pub fn sibling(&self) -> Self {
        Self {
            exec: self.exec.clone(),
            journal: ActJournal::live(),
            target: self.target.clone(),
            mode: self.mode,
            writes: 0,
            excluded: self.excluded.clone(),
            advisories: self.advisories.clone(),
        }
    }

    /// Every invocation that reached the wrapper through this boundary's seam, across its acts.
    #[must_use]
    pub const fn writes_performed(&self) -> usize {
        self.writes
    }

    /// One act, through [`commit_act`].
    ///
    /// # Errors
    /// As [`commit_act`].
    pub fn act(
        &mut self,
        store: &Mutex<dyn EventStore>,
        act: Act,
    ) -> Result<Committed, TierOneError> {
        let mut guard = store.lock().expect(LOCK_HALTS);
        self.act_locked(&mut *guard, act)
    }

    /// One act over a store the CALLER holds locked: the step-grain act, whose journal re-read,
    /// landing and append are one critical section (AC-012; ADR-0188 decision 2).
    /// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
    ///
    /// # Errors
    /// As [`commit_act`].
    pub fn act_locked(
        &mut self,
        store: &mut dyn EventStore,
        act: Act,
    ) -> Result<Committed, TierOneError> {
        let recorder = self.journal.recorder();
        let mut ops = GitOps::new(self.exec.clone(), &mut self.journal, self.mode);
        let out = land_act(
            &mut ops,
            &recorder,
            store,
            &self.target,
            &self.excluded,
            self.advisories.as_ref(),
            act,
        );
        self.writes += ops.writes_performed();
        out
    }

    /// **The baseline, read at worktree creation** (ADR-0188 decision 4; `s8f` §8f.4: *"The
    /// baseline is the untracked set read at worktree creation, as `status --porcelain=v1 -z
    /// --untracked-files=all` through the seam, and journalled"*). When the item's journal holds a
    /// creation (`worktree_assigned { reused: false }`) that no `status` read has followed, this
    /// reads the tree's untracked set through the seam, authorised as any act is, and journals the
    /// read unanchored, so it is the baseline every later act reads. `run` calls it right after the
    /// creation's row and BEFORE the entry pass writes into the managed directory: on a tree
    /// re-created on an existing run branch the entry pass's writes (a stored vouched root, a
    /// restored rendering) are then files the run created, and the entry's boundary lands them.
    /// Anything else — a reused tree, a journalled baseline, replay — reads nothing and returns
    /// `false`. *(Added 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2's
    /// spec-conformance major: the first ACT took the baseline, after the entry pass had written.)*
    ///
    /// # Errors
    /// [`TierOneError::Store`] from the query or an append; the authorisation's and the read's own.
    pub fn take_baseline(&mut self, store: &Mutex<dyn EventStore>) -> Result<bool, TierOneError> {
        if self.mode == Mode::Replay {
            return Ok(false);
        }
        let mut guard = store.lock().expect(LOCK_HALTS);
        let events = guard
            .query(Query::ByItem(self.target.item.clone()))
            .map_err(|e| TierOneError::Store {
                detail: format!("{e:?}"),
            })?;
        if baseline_of(&events, &self.target.item) != BaselineRead::ThisAct {
            return Ok(false);
        }
        let recorder = self.journal.recorder();
        drop(recorder.take());
        let mut ops = GitOps::new(self.exec.clone(), &mut self.journal, self.mode);
        let cwd = authorise_tier1_write(ops.exec(), &self.target.worktree, &self.target.ownership)?
            .path()
            .clone();
        let read = landing::staging_set(
            &mut ops,
            &cwd,
            &Baseline::Known(BTreeSet::new()),
            &BTreeSet::new(),
        );
        self.writes += ops.writes_performed();
        read?;
        for inv in recorder.take() {
            guard
                .append(git_op_event(&inv, &self.target.item))
                .map_err(|e| TierOneError::Store {
                    detail: format!("{e:?}"),
                })?;
        }
        drop(guard);
        Ok(true)
    }
}

// ---------------------------------------------------------------------------- run ownership

/// The `worktree_assigned` row a `run` writes at every start (§8e.4; the three-kinds record).
#[must_use]
pub fn worktree_assigned_event(
    item: &str,
    worktree: &Path,
    branch: &str,
    reused: bool,
    discarded: &[String],
) -> Event {
    Event::new(
        EventKind::WorktreeAssigned,
        serde_json::json!({
            "branch": branch,
            "discarded": discarded,
            "item": item,
            "reused": reused,
            "worktree": worktree.display().to_string(),
        }),
    )
    .with_item(item)
}

/// **The one reader of run ownership**: the latest `worktree_assigned` row FOR `item`. `None` when
/// the store holds none — and a caller then refuses, fail-closed; nothing is resolved from `cwd` or
/// from a path convention (ADR-0105 clause 8; AC-529).
#[must_use]
pub fn assigned_ownership(events: &[Event], item: &str) -> Option<RunOwnership> {
    events
        .iter()
        .rev()
        .filter(|e| e.kind == EventKind::WorktreeAssigned)
        .filter(|e| e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item))
        .find_map(|e| {
            let worktree = e.payload.get("worktree")?.as_str()?;
            let branch = e.payload.get("branch")?.as_str()?;
            Some(RunOwnership {
                worktree: PathBuf::from(worktree),
                branch: branch.to_owned(),
            })
        })
}

/// The `stage_empty` row (§8e.6; the three-kinds record decision 3's closed reason set).
#[must_use]
pub fn stage_empty_event(stage: &str, reason: &str) -> Event {
    Event::new(
        EventKind::StageEmpty,
        serde_json::json!({ "reason": reason, "stage": stage }),
    )
}

// ------------------------------------------------------------------------------------ the heal

/// **The heal**: for every trailer mark reachable from `rk-run/<item>` whose sha anchors no row,
/// append the rows the mark carries, anchored to it. Git → store; every git call here is a read.
///
/// Healed: a stage-bearing `Story:` mark for `item` (its `stage_transition`, `from` the previous
/// stage mark on the ref when there is one) and every `Gate:` mark on the ref (its transition row,
/// built from the mark's own tokens). Skipped, by name: ~~a `closed` mark and~~ ~~a `Slice-Verdict:`
/// mark, whose rows are ~~§8e.6's and~~ §8e.8's;~~ a legacy stage-less mark, which records no stage.
/// *(dated 2026-09-28, `s8e` build, package S8E-P7: a `Slice-Verdict:` mark heals as its
/// `slice_sealed` row, `{project, slice, verdict}` — no `stage`, which the trailer does not carry.)*
/// *(dated 2026-09-28, `s8e` build, package S8E-P6: a `closed` mark now heals as [`close_act`]'s
/// row, `stage_transition { from: <the last stage mark>, to: "closed" }`.)*
///
/// Returns the shas it healed, oldest first.
///
/// # Errors
/// [`TierOneError::Git`] from a read; [`TierOneError::Store`] from a query or an append.
pub fn heal(
    exec: &GitExec,
    repo: &WorktreePath,
    store: &Mutex<dyn EventStore>,
    item: &str,
) -> Result<Vec<String>, TierOneError> {
    let mut healed = Vec::new();
    for commit in marked_commits(exec, repo, item)? {
        let rows = commit.rows;
        if rows.is_empty() {
            continue;
        }
        let present = store
            .lock()
            .expect(LOCK_HALTS)
            .query(Query::BySha(commit.sha.clone()))
            .map_err(|e| TierOneError::Store {
                detail: format!("{e:?}"),
            })?;
        if !present.is_empty() {
            continue;
        }
        let spec = format!("{}^{{tree}}", commit.sha);
        let tree = exec
            .run(GitOp::RevParse, repo, ["rev-parse", spec.as_str()])?
            .stdout_trimmed();
        let anchor = TierOneAnchor::new(&commit.sha, &tree);
        let mut guard = store.lock().expect(LOCK_HALTS);
        for row in rows {
            guard
                .append_tier1(row, &anchor)
                .map_err(|e| TierOneError::Store {
                    detail: format!("{e:?}"),
                })?;
        }
        drop(guard);
        healed.push(commit.sha.clone());
    }
    // *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E; ADR-0188 decision 3; finding
    // 11.)* The step-checkpoint arm: a commit whose subject begins `rk step ` and whose sha anchors
    // no row gets the row its body carries, anchored to it. A crash between a step's landing and
    // its row is D21-42 8a's window, and this is how it closes for a step: the step is served, not
    // re-dispatched over its own landed edit.
    for checkpoint in step_checkpoints(exec, repo, item)? {
        let present = store
            .lock()
            .expect(LOCK_HALTS)
            .query(Query::BySha(checkpoint.sha.clone()))
            .map_err(|e| TierOneError::Store {
                detail: format!("{e:?}"),
            })?;
        if !present.is_empty() {
            continue;
        }
        let row = step_row_of(&checkpoint, item)?;
        let tree = commit_tree(exec, repo, &checkpoint.sha)?;
        store
            .lock()
            .expect(LOCK_HALTS)
            .append_tier1(
                keyed(row, item),
                &TierOneAnchor::new(&checkpoint.sha, &tree),
            )
            .map_err(|e| TierOneError::Store {
                detail: format!("{e:?}"),
            })?;
        healed.push(checkpoint.sha);
    }
    Ok(healed)
}

/// One step checkpoint on the run ref: its sha, subject and body (ADR-0188 decision 3).
#[derive(Debug, Clone, PartialEq, Eq)]
struct StepCommit {
    sha: String,
    subject: String,
    body: String,
}

/// Every commit reachable from `rk-run/<item>` whose subject begins [`STEP_SUBJECT_PREFIX`] and
/// names `item`, oldest first. A read.
fn step_checkpoints(
    exec: &GitExec,
    repo: &WorktreePath,
    item: &str,
) -> Result<Vec<StepCommit>, TierOneError> {
    let run_ref = format!("refs/heads/rk-run/{item}");
    if exec
        .run(
            GitOp::RevParse,
            repo,
            ["rev-parse", "--verify", "--quiet", run_ref.as_str()],
        )
        .is_err()
    {
        return Ok(Vec::new());
    }
    let out = exec.run(
        GitOp::Log,
        repo,
        [
            "log",
            "--reverse",
            "--format=%H%x1f%s%x1f%b%x1e",
            run_ref.as_str(),
        ],
    )?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let prefix = format!("{STEP_SUBJECT_PREFIX}{item} ");
    Ok(text
        .split('\u{1e}')
        .filter_map(|record| {
            let mut fields = record.trim_start_matches('\n').splitn(3, '\u{1f}');
            let sha = fields.next()?.trim().to_owned();
            let subject = fields.next()?.to_owned();
            let body = fields.next().unwrap_or_default().to_owned();
            (!sha.is_empty() && subject.starts_with(&prefix)).then_some(StepCommit {
                sha,
                subject,
                body,
            })
        })
        .collect())
}

/// **The row a step checkpoint carries**, read from its body and checked against its subject: the
/// key the subject names (`<item> <activity_id@version> <step_id> <iteration-path>`) must be the
/// key inside the row. Anything else is refused by name, naming the commit.
fn step_row_of(checkpoint: &StepCommit, item: &str) -> Result<Event, TierOneError> {
    let refuse = |detail: String| TierOneError::StepCheckpointUnreadable {
        sha: checkpoint.sha.clone(),
        detail,
    };
    let key: Vec<&str> = checkpoint
        .subject
        .strip_prefix(STEP_SUBJECT_PREFIX)
        .unwrap_or_default()
        .split_whitespace()
        .collect();
    let [subject_item, activity, step_id, path] = key.as_slice() else {
        return Err(refuse(format!(
            "its subject {:?} does not name `<item> <activity_id@version> <step_id> \
             <iteration-path>`",
            checkpoint.subject
        )));
    };
    let line = checkpoint
        .body
        .lines()
        .find_map(|l| l.strip_prefix(STEP_ROW_PREFIX))
        .ok_or_else(|| refuse(format!("its body carries no `{STEP_ROW_PREFIX}` line")))?;
    let value: serde_json::Value = serde_json::from_str(line)
        .map_err(|e| refuse(format!("its `{STEP_ROW_PREFIX}` line is not JSON: {e}")))?;
    let canonical = rk_store::canon::canonical(&value)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .map_err(|e| refuse(format!("its row is not canonical JSON: {e:?}")))?;
    if canonical != line {
        return Err(refuse("its row is not in canonical form".to_owned()));
    }
    let kind = value
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .and_then(EventKind::from_wire)
        .ok_or_else(|| refuse("its row names no event kind this binary knows".to_owned()))?;
    let payload = value
        .get("payload")
        .cloned()
        .ok_or_else(|| refuse("its row carries no payload".to_owned()))?;
    let text = |m: &str| payload.get(m).and_then(serde_json::Value::as_str);
    let row_path = payload
        .get("iteration_path")
        .map(rk_store::canon::canonical)
        .transpose()
        .map_err(|e| refuse(format!("its row's iteration path is not canonical: {e:?}")))?
        .map(|b| String::from_utf8_lossy(&b).into_owned());
    let agrees = *subject_item == item
        && text("item").is_none_or(|i| i == item)
        && text("activity_id_at_version") == Some(*activity)
        && text("step_id") == Some(*step_id)
        && row_path.as_deref() == Some(*path);
    if !agrees {
        return Err(refuse(format!(
            "the key its subject names ({:?}) disagrees with the key inside its row",
            checkpoint.subject
        )));
    }
    Ok(Event::new(kind, payload))
}

/// One trailer commit reachable from `rk-run/<item>`, and the rows its marks carry for the item.
/// *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.9.)*
#[derive(Debug, Clone, PartialEq)]
pub struct MarkedCommit {
    /// The commit.
    pub sha: String,
    /// The rows the commit's marks carry, keyed to the item (ADR-0182), in mark order — the rows
    /// the live act that made the commit anchored to it.
    pub rows: Vec<Event>,
    /// The commit carries a legacy stage-less `Story: <project>/<item>` mark, which records no
    /// stage and so carries no row here (the rebuild's item-record fallback reads it).
    pub legacy_story: bool,
}

/// **The mark-to-row derivation, shared by the heal and the rebuild** (`s8e` §8e.3, §8e.9;
/// ADR-0181 decisions 9 and 11). *(dated 2026-09-28, `s8e` build, package S8E-P7.)*
///
/// Every commit reachable from `refs/heads/rk-run/<item>`, oldest first, with the rows its marks
/// carry — the rows the live act anchored to it. Empty when the ref does not exist. Every git call
/// is a read (AC-174).
///
/// # Errors
/// [`TierOneError::Git`] from a read.
pub fn marked_commits(
    exec: &GitExec,
    repo: &WorktreePath,
    item: &str,
) -> Result<Vec<MarkedCommit>, TierOneError> {
    let reachable = reachable_from_run_ref(exec, repo, item)?;
    if reachable.is_empty() {
        return Ok(Vec::new());
    }
    let mut last_stage: Option<String> = None;
    let mut out = Vec::new();
    for commit in read_commits(exec, repo)?
        .iter()
        .filter(|c| reachable.contains(&c.sha))
    {
        let rows: Vec<Event> = rows_of_marks(commit, item, &mut last_stage)
            .into_iter()
            .map(|row| keyed(row, item))
            .collect();
        let legacy_story = commit
            .trailers
            .iter()
            .any(|t| matches!(t, ParsedTrailer::Story { slug, stage: None, .. } if slug == item));
        if rows.is_empty() && !legacy_story {
            continue;
        }
        out.push(MarkedCommit {
            sha: commit.sha.clone(),
            rows,
            legacy_story,
        });
    }
    Ok(out)
}

/// The tree of commit `sha` (`<sha>^{tree}`), a read.
///
/// # Errors
/// [`TierOneError::Git`] from the read.
pub fn commit_tree(exec: &GitExec, repo: &WorktreePath, sha: &str) -> Result<String, TierOneError> {
    let spec = format!("{sha}^{{tree}}");
    Ok(exec
        .run(GitOp::RevParse, repo, ["rev-parse", spec.as_str()])?
        .stdout_trimmed())
}

/// Every commit `refs/heads/rk-run/<item>` reaches; empty when the ref does not exist.
fn reachable_from_run_ref(
    exec: &GitExec,
    repo: &WorktreePath,
    item: &str,
) -> Result<BTreeSet<String>, TierOneError> {
    let run_ref = format!("refs/heads/rk-run/{item}");
    if exec
        .run(
            GitOp::RevParse,
            repo,
            ["rev-parse", "--verify", "--quiet", run_ref.as_str()],
        )
        .is_err()
    {
        return Ok(BTreeSet::new());
    }
    let out = exec.run(GitOp::Log, repo, ["log", "--format=%H", run_ref.as_str()])?;
    Ok(out
        .stdout_trimmed()
        .lines()
        .map(|l| l.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect())
}

/// The rows one commit's marks carry for `item`, advancing the stage the ref last entered.
fn rows_of_marks(
    commit: &CommitTrailers,
    item: &str,
    last_stage: &mut Option<String>,
) -> Vec<Event> {
    let mut rows = Vec::new();
    for trailer in &commit.trailers {
        match trailer {
            ParsedTrailer::Story {
                slug,
                stage: Some(StoryStage::Stage(stage)),
                ..
            } if slug == item => {
                let mut payload = serde_json::Map::new();
                payload.insert("to".to_owned(), serde_json::Value::from(stage.clone()));
                if let Some(from) = last_stage.replace(stage.clone()) {
                    payload.insert("from".to_owned(), serde_json::Value::from(from));
                }
                rows.push(Event::new(
                    EventKind::StageTransition,
                    serde_json::Value::Object(payload),
                ));
            }
            // *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.6.)* The `closed` mark's
            // row is [`close_act`]'s: `stage_transition { from: <the last stage>, to: "closed" }`.
            ParsedTrailer::Story {
                slug,
                stage: Some(StoryStage::Closed),
                ..
            } if slug == item => {
                let mut payload = serde_json::Map::new();
                payload.insert(
                    "to".to_owned(),
                    serde_json::Value::from(rk_git::trailer::grammar::CLOSED_STAGE_TOKEN),
                );
                if let Some(from) =
                    last_stage.replace(rk_git::trailer::grammar::CLOSED_STAGE_TOKEN.to_owned())
                {
                    payload.insert("from".to_owned(), serde_json::Value::from(from));
                }
                rows.push(Event::new(
                    EventKind::StageTransition,
                    serde_json::Value::Object(payload),
                ));
            }
            ParsedTrailer::Gate {
                token,
                tree_hash,
                deadline,
                kind,
                reach,
                state,
            } => rows.push(gate_row_of_mark(
                token, tree_hash, *deadline, kind, *reach, state,
            )),
            // *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8.)* The seal's row, from
            // the mark's own tokens. No `stage`: the trailer carries none, and the fold scopes a
            // stage-less seal to the stage it folds in (D21-93; the slice record's stated residual).
            ParsedTrailer::SliceVerdict {
                project,
                slice,
                verdict,
            } => rows.push(Event::new(
                EventKind::SliceSealed,
                serde_json::json!({
                    "project": project,
                    "slice": slice,
                    "verdict": verdict.as_str(),
                }),
            )),
            _ => {}
        }
    }
    rows
}

/// The transition row a `Gate:` mark carries, from the mark's own tokens.
fn gate_row_of_mark(
    token: &str,
    tree_hash: &str,
    deadline: i64,
    kind: &str,
    reach: u32,
    state: &GateState,
) -> Event {
    let mut payload: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    payload.insert("token", serde_json::Value::from(token));
    let event_kind = match state {
        GateState::Opened => {
            payload.insert("deadline", serde_json::Value::from(deadline));
            payload.insert("kind", serde_json::Value::from(kind));
            payload.insert("reach", serde_json::Value::from(reach));
            payload.insert("tree_hash", serde_json::Value::from(tree_hash));
            EventKind::GateOpened
        }
        GateState::Answered { option } => {
            payload.insert("tree_hash", serde_json::Value::from(tree_hash));
            payload.insert("verdict", serde_json::Value::from(option.as_str()));
            EventKind::GateAnswered
        }
        GateState::Expired { action } => {
            let (name, verdict) = match action {
                GateExpiry::Fail => ("fail", None),
                GateExpiry::Escalate => ("escalate", None),
                GateExpiry::AutoDecide { verdict } => ("auto-decide", Some(verdict.as_str())),
            };
            payload.insert("action", serde_json::Value::from(name));
            if let Some(v) = verdict {
                payload.insert("verdict", serde_json::Value::from(v));
            }
            EventKind::GateExpired
        }
        GateState::Voided => {
            payload.insert("bound_tree_hash", serde_json::Value::from(tree_hash));
            EventKind::GateAnswerVoid
        }
    };
    Event::new(
        event_kind,
        serde_json::Value::Object(
            payload
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v))
                .collect(),
        ),
    )
}
