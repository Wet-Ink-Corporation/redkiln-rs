//! **`s8e` §8e.3 — every tier-1 act is git-first, through one kernel function.**
//!
//! *(dated 2026-09-28, `s8e` build, package S8E-P5; D21-42 8a; D21-43 N6, N7 and N10's seam; the
//! tier-1 writer record ADR-0181 and the three-kinds record, both proposed.)*
//!
//! `rk_kernel::runtime::tier_one::commit_act` authorises its target through
//! `rk_git::worktree::authorise_tier1_write`, commits through the journalled `GitOps` seam, re-reads
//! `HEAD` and the tree, and only then calls `append_tier1` for every row of the act, each anchored to
//! that one commit. So a failed commit leaves no row by construction (AC-162's second clause), and
//! the tests below watch it rather than trust it.
//!
//! Every fixture is a real repository whose main worktree sits on `rk-run/<item>`, with a
//! `worktree_assigned`-shaped ownership naming it: the positive half of AC-529 is the precondition
//! of every act here, and the foreign half is `rk-cli`'s `worktree_assigned::` module.
//!
//! **The two source lints read CODE lines only** (a guard satisfied by its own prose is no guard):
//! a line whose first non-blank characters are `//`, `/*` or a `*` doc continuation is dropped, and
//! each lint carries a synthetic control proving it can go red.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_git::git_exec::GitExec;
use rk_git::gitops::{GitOps, Mode};
use rk_git::worktree::RunOwnership;
use rk_kernel::runtime::stage_transition_event;
use rk_kernel::runtime::tier_one::{
    Act, ActJournal, Boundary, Committed, Mark, Target, TierOneError, commit_act, heal,
};
use rk_kernel::workflow::fsm::GateRecord;
use rk_kernel::workflow::gate::{
    ExpiryAction, gate_answer_void_event, gate_answered_event, gate_expired_event,
};
use rk_ports::event_store::{EventStore, Query, Snapshot, TierOneAnchor};
use rk_ports::fakes::VecStore;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

const ITEM: &str = "tier-1";
const PROJECT: &str = "rk-fixture";
const BRANCH: &str = "rk-run/tier-1";
const TREE_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const DEADLINE: i64 = 1_900_000_000;

// ------------------------------------------------------------------------------------ fixtures

struct Fx {
    _dir: tempfile::TempDir,
    root: PathBuf,
    exec: GitExec,
}

fn git(root: &Path, args: &[&str]) -> String {
    match rk_git::git_exec::run(root, args) {
        Ok(out) => out.stdout_trimmed(),
        Err(e) => panic!("git {args:?} in {}: {e:?}", root.display()),
    }
}

/// A repository whose main worktree is on `rk-run/tier-1`, one commit deep, with the hooks
/// directory pointed at `.git/hooks` so the hook fixture below can bite a plain commit.
fn fixture() -> Fx {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("repo");
    std::fs::create_dir_all(&root).expect("mkdir repo");
    git(&root, &["init", "--quiet", "-b", "main"]);
    git(&root, &["config", "user.name", "rk"]);
    git(&root, &["config", "user.email", "rk@example.invalid"]);
    git(&root, &["config", "commit.gpgsign", "false"]);
    git(&root, &["config", "core.autocrlf", "false"]);
    let hooks = root.join(".git").join("hooks");
    std::fs::create_dir_all(&hooks).expect("mkdir hooks");
    git(
        &root,
        &["config", "core.hooksPath", &hooks.display().to_string()],
    );
    std::fs::write(root.join("README.md"), b"tier one\n").expect("README");
    git(&root, &["add", "README.md"]);
    git(&root, &["commit", "--quiet", "-m", "base"]);
    git(&root, &["checkout", "--quiet", "-b", BRANCH]);
    let tc = rk_git::version::assert_at_startup(&root).expect("git at or above the floor");
    let exec = GitExec::new(dir.path().join("hooks-empty"), tc).expect("the wrapper");
    Fx {
        _dir: dir,
        root,
        exec,
    }
}

fn target(fx: &Fx) -> Target {
    Target {
        worktree: fx.root.clone(),
        ownership: RunOwnership {
            worktree: fx.root.clone(),
            branch: BRANCH.to_owned(),
        },
        project: Some(PROJECT.to_owned()),
        item: ITEM.to_owned(),
    }
}

fn store() -> Arc<Mutex<dyn EventStore>> {
    Arc::new(Mutex::new(VecStore::default()))
}

/// Every row, whatever its key: `ByRun` is inert and returns the whole store (ADR-0182 d5).
fn all_rows(store: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(Query::ByRun(String::new()))
        .expect("the store reads")
}

fn rows_at(store: &Arc<Mutex<dyn EventStore>>, sha: &str) -> Vec<Event> {
    store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(Query::BySha(sha.to_owned()))
        .expect("the store reads")
}

fn head(root: &Path) -> String {
    git(root, &["rev-parse", "HEAD"])
}

fn message(root: &Path, rev: &str) -> String {
    git(root, &["log", "-1", "--format=%B", rev])
}

fn commit_count(root: &Path) -> usize {
    git(root, &["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a count")
}

fn advance(from: &str, to: &str) -> Act {
    Act::marked(
        Mark::Stage(to.to_owned()),
        vec![stage_transition_event(from, to)],
    )
}

fn record() -> GateRecord {
    GateRecord::derive(
        "unknown_facet:has_repro",
        TREE_A,
        Timestamp::from_unix_seconds(DEADLINE),
    )
    .expect("derive")
}

/// The one boundary, live, over a fresh journal.
fn act_live(
    fx: &Fx,
    store: &Arc<Mutex<dyn EventStore>>,
    act: Act,
) -> Result<Committed, TierOneError> {
    Boundary::live(fx.exec.clone(), target(fx)).act(store, act)
}

/// Make the NEXT commit in `root` fail: git takes `index.lock` for every commit, `--allow-empty`
/// included, and refuses when it exists (measured 2026-09-28: `fatal: Unable to create
/// '…/.git/index.lock': File exists.`, rc 128).
fn jam_the_commit(root: &Path) -> PathBuf {
    let lock = root.join(".git").join("index.lock");
    std::fs::write(&lock, b"").expect("index.lock");
    lock
}

/// A store that records, at every append, which commit `HEAD` was on — so "the commit exists
/// before any row is readable" is read at the moment of the append rather than inferred after it.
struct Witness {
    inner: VecStore,
    root: PathBuf,
    seen: Seen,
}

/// Every append the witness saw: the row's kind, and the commit `HEAD` was on at that moment.
type Seen = Arc<Mutex<Vec<(EventKind, String)>>>;

impl Witness {
    fn note(&self, kind: EventKind) {
        let at = head(&self.root);
        self.seen.lock().expect("witness lock").push((kind, at));
    }
}

impl EventStore for Witness {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        self.note(event.kind);
        self.inner.append(event)
    }
    fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError> {
        self.note(event.kind);
        self.inner.append_tier1(event, anchor)
    }
    fn query(&self, by: Query) -> Result<Vec<Event>, StepError> {
        self.inner.query(by)
    }
    fn snapshot(&self) -> Result<Snapshot, StepError> {
        self.inner.snapshot()
    }
    fn rebuild(&mut self, snapshot: &Snapshot) -> Result<(), StepError> {
        self.inner.rebuild(snapshot)
    }
}

fn witness(fx: &Fx) -> (Arc<Mutex<dyn EventStore>>, Seen) {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(Witness {
        inner: VecStore::default(),
        root: fx.root.clone(),
        seen: Arc::clone(&seen),
    }));
    (store, seen)
}

/// The git-first property for one act: the act's commit exists and carries `expect_trailer`, and
/// every one of the act's `kind` rows was appended while `HEAD` already WAS that commit.
fn assert_committed_before_rows(fx: &Fx, act: Act, kind: EventKind, expect_trailer: &str) {
    let (store, seen) = witness(fx);
    let before = head(&fx.root);
    let done = act_live(fx, &store, act).expect("the act commits");
    let Committed::Live { sha, .. } = done else {
        panic!("a live act reported {done:?}");
    };
    assert_ne!(sha, before, "the act made no commit");
    assert!(
        message(&fx.root, &sha).contains(expect_trailer),
        "the act's commit does not carry `{expect_trailer}`:\n{}",
        message(&fx.root, &sha)
    );
    let seen = seen.lock().expect("witness lock").clone();
    let of_kind: Vec<&(EventKind, String)> = seen.iter().filter(|(k, _)| *k == kind).collect();
    assert_eq!(
        of_kind.len(),
        1,
        "exactly one `{}` row reaches the store: {seen:?}",
        kind.wire_name()
    );
    assert!(
        seen.iter().all(|(_, at)| *at == sha),
        "a row was appended while HEAD was not yet the act's commit {sha} — the store was written \
         first: {seen:?}"
    );
    assert!(
        rows_at(&store, &sha).iter().any(|e| e.kind == kind),
        "the `{}` row is not anchored to its commit {sha}",
        kind.wire_name()
    );
}

/// The failure half for one act: the commit fails, and NOTHING reaches the store.
fn assert_failed_commit_leaves_nothing(fx: &Fx, act: Act) {
    let store = store();
    let before = head(&fx.root);
    let lock = jam_the_commit(&fx.root);
    let refused = act_live(fx, &store, act);
    std::fs::remove_file(&lock).expect("unjam");
    assert!(
        matches!(refused, Err(TierOneError::Git(_))),
        "a failed commit must be the act's git error, got {refused:?}"
    );
    assert_eq!(
        head(&fx.root),
        before,
        "HEAD moved although the commit failed"
    );
    let rows = all_rows(&store);
    assert!(
        rows.is_empty(),
        "a failed commit left {} store row(s): {:?}",
        rows.len(),
        rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
    );
}

// --------------------------------------------------------------------------- AC-162, AC-163

/// **AC-162's first fault injection: a git backend that fails the commit, and ZERO store rows.**
/// Red at entry because the row is appended first. Red under S8E-M3.
#[test]
fn a_failed_commit_leaves_zero_store_rows() {
    let fx = fixture();
    assert_failed_commit_leaves_nothing(&fx, advance("implement", "review"));
}

/// **AC-162's second fault injection: a hook that rewrites the commit message has no effect.**
///
/// The fixture installs a `commit-msg` hook that replaces the whole message. The CONTROL commits
/// through git with no prelude and shows the hook bites; the act commits through the boundary,
/// whose wrapper points `core.hooksPath` at an empty directory (ADR-0105 clause 2).
#[test]
fn a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit() {
    let fx = fixture();
    let hook = fx.root.join(".git").join("hooks").join("commit-msg");
    std::fs::write(&hook, b"#!/bin/sh\necho rewritten-by-hook > \"$1\"\n").expect("hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }
    // CONTROL: the hook bites a commit that bypasses the wrapper.
    git(
        &fx.root,
        &["commit", "--quiet", "--allow-empty", "-m", "control"],
    );
    assert_eq!(
        message(&fx.root, "HEAD").trim(),
        "rewritten-by-hook",
        "CONTROL: the fixture's hook does not rewrite a plain commit, so the assertion below \
         would be about nothing"
    );

    let store = store();
    let done = act_live(&fx, &store, advance("implement", "review")).expect("the act commits");
    let sha = done.sha().expect("a live act has a sha").to_owned();
    let body = message(&fx.root, &sha);
    assert!(
        !body.contains("rewritten-by-hook"),
        "the hook rewrote the trailer commit:\n{body}"
    );
    assert!(
        body.contains(&format!("Story: {PROJECT}/{ITEM} review")),
        "the trailer commit lost its `Story:` mark:\n{body}"
    );
}

/// **The commit exists before any row is readable.** Red at entry because the store is written
/// first. Red under S8E-M3.
#[test]
fn the_commit_exists_before_any_row_is_readable() {
    let fx = fixture();
    assert_committed_before_rows(
        &fx,
        advance("implement", "review"),
        EventKind::StageTransition,
        &format!("Story: {PROJECT}/{ITEM} review"),
    );
}

/// **AC-163's unit test: `HEAD` advanced, the trailer parses, and every row of the act — tier 1
/// and tier 2 — is anchored to the one commit.** A `stage_transition` and its `stage_empty` ride
/// one act, the shape §8e.6's empty stage takes.
#[test]
fn a_boundary_anchors_every_row_of_the_act_to_its_commit() {
    let fx = fixture();
    let store = store();
    let count_before = commit_count(&fx.root);
    let empty = Event::new(
        EventKind::StageEmpty,
        serde_json::json!({ "reason": "no_applicable_activity", "stage": "triage" }),
    );
    let act = Act::marked(
        Mark::Stage("triage".to_owned()),
        vec![stage_transition_event("intake", "triage"), empty],
    );
    let done = act_live(&fx, &store, act).expect("the act commits");
    let sha = done.sha().expect("live").to_owned();
    assert_eq!(
        commit_count(&fx.root),
        count_before + 1,
        "HEAD advanced by one commit"
    );
    assert_eq!(head(&fx.root), sha);

    let parsed = rk_git::trailer::grammar::parse_body(&message(&fx.root, &sha));
    assert!(
        parsed.iter().any(|t| matches!(
            t,
            rk_git::trailer::grammar::ParsedTrailer::Story { project, slug, stage: Some(rk_git::trailer::grammar::StoryStage::Stage(s)) }
                if project == PROJECT && slug == ITEM && s == "triage"
        )),
        "the commit's trailer does not parse as the stage-bearing mark: {parsed:?}"
    );

    let anchored = rows_at(&store, &sha);
    for kind in [EventKind::StageTransition, EventKind::StageEmpty] {
        assert_eq!(
            anchored.iter().filter(|e| e.kind == kind).count(),
            1,
            "the act's `{}` row is not anchored to {sha}: {:?}",
            kind.wire_name(),
            anchored
                .iter()
                .map(|e| e.kind.wire_name())
                .collect::<Vec<_>>()
        );
    }
    let unanchored = all_rows(&store).len() - anchored.len();
    assert_eq!(unanchored, 0, "a row of the act was written unanchored");
    assert!(
        anchored.iter().all(|e| e.item() == Some(ITEM)),
        "every row of the act carries the interim item key (ADR-0182)"
    );
}

/// **An act with neither a trailer nor content is refused** — an empty commit recording nothing is
/// noise on the run ref. No commit and no row.
#[test]
fn an_act_with_neither_trailer_nor_content_is_refused() {
    let fx = fixture();
    let store = store();
    let before = head(&fx.root);
    let act = Act::unmarked(vec![Event::new(
        EventKind::StageEmpty,
        serde_json::json!({ "reason": "no_applicable_activity", "stage": "triage" }),
    )]);
    let refused = act_live(&fx, &store, act);
    assert!(
        matches!(refused, Err(TierOneError::EmptyAct)),
        "an act with no trailer and no content was not refused as empty: {refused:?}"
    );
    assert_eq!(head(&fx.root), before, "the refused act committed");
    assert!(all_rows(&store).is_empty(), "the refused act wrote a row");
}

// ------------------------------------------------------------------ N6: every gate transition

/// **The answer is committed before its row** (N6 fixes `answer`'s append-then-commit). Red at
/// entry because `answer` appends first. ~~Red under S8E-M26 (its `rk-cli` half is
/// `answer::a_failed_answer_commit_leaves_zero_rows`).~~
///
/// *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's tests-mutants finding.)*
/// **What reddens this test is the KERNEL boundary's order**, never `rk-cli`'s: it builds
/// `Act::gate(gate_answered_event(..), rec)` and calls `Boundary::live(..).act(..)` itself, so it
/// never reaches `answer.rs` or `run::gate_rows_through`. Measured: **S8E-M26b** (the boundary
/// appends the act's `gate_answered` row before its commit) is red here (*"a row was appended while
/// HEAD was not yet the act's commit"*); **S8E-M26a** (`answer` appends `gate_answered` before its
/// commit, in `rk-cli`) leaves this test GREEN and is killed by
/// `answer::a_failed_answer_commit_leaves_zero_rows` alone. S8E-M3 (every row before the commit)
/// reddens it too.
#[test]
fn a_gate_answer_is_committed_before_its_row() {
    let fx = fixture();
    let rec = record();
    let act = |rec: &GateRecord| {
        Act::gate(gate_answered_event(rec.token(), TREE_A, "approve"), rec).expect("an answer act")
    };
    assert_committed_before_rows(
        &fx,
        act(&rec),
        EventKind::GateAnswered,
        &format!(
            "Gate: {} {TREE_A} {DEADLINE} unknown_facet:has_repro 1 answered approve",
            rec.token().as_str()
        ),
    );
    assert_failed_commit_leaves_nothing(&fx, act(&rec));
}

/// **The expiry is committed before its row** (N6: today it is store-only). ~~Red under S8E-M43.~~
///
/// *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's tests-mutants finding.)*
/// **What reddens this test is the KERNEL boundary** (it calls `Boundary::live(..).act(..)` over
/// `Act::gate(gate_expired_event(..), rec)` itself). Measured: **S8E-M43b** (the boundary appends
/// an expiry's row with no commit and reports `NoGitWrite`) is red here (*"a live act reported
/// NoGitWrite"*); **S8E-M43a** (`run`'s expiry rows handed to `run::gate_rows_through`'s
/// store-only arm, in `rk-cli`) leaves this test GREEN and is killed by
/// `lane_e2e::every_tier_one_row_has_exactly_one_trailer_commit`'s expiry arm alone.
#[test]
fn a_gate_expiry_is_committed_before_its_row() {
    let fx = fixture();
    let rec = record();
    let act = |rec: &GateRecord| {
        Act::gate(
            gate_expired_event(rec.token(), &ExpiryAction::Fail, None),
            rec,
        )
        .expect("an expiry act")
    };
    assert_committed_before_rows(
        &fx,
        act(&rec),
        EventKind::GateExpired,
        &format!(
            "Gate: {} {TREE_A} {DEADLINE} unknown_facet:has_repro 1 expired fail",
            rec.token().as_str()
        ),
    );
    assert_failed_commit_leaves_nothing(&fx, act(&rec));
}

/// **The void is committed before its row** (N6: today it is a row with no trailer).
#[test]
fn a_gate_void_is_committed_before_its_row() {
    let fx = fixture();
    let rec = record();
    let observed = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    let act = |rec: &GateRecord| {
        Act::gate(gate_answer_void_event(rec.token(), TREE_A, observed), rec).expect("a void act")
    };
    assert_committed_before_rows(
        &fx,
        act(&rec),
        EventKind::GateAnswerVoid,
        &format!(
            "Gate: {} {TREE_A} {DEADLINE} unknown_facet:has_repro 1 voided",
            rec.token().as_str()
        ),
    );
    assert_failed_commit_leaves_nothing(&fx, act(&rec));
}

/// **A gate kind carrying whitespace is refused at the mint** — the widened value is
/// whitespace-separated, so such a kind would shift every later token (ADR-0184). Nothing commits.
#[test]
fn a_gate_kind_with_whitespace_is_refused_at_the_mint() {
    let fx = fixture();
    let store = store();
    let before = head(&fx.root);
    let rec = GateRecord::derive(
        "unknown facet",
        TREE_A,
        Timestamp::from_unix_seconds(DEADLINE),
    )
    .expect("derive");
    let row = rk_kernel::runtime::gate_opened_event(&rec);
    let refused = Act::gate_opened(row).and_then(|act| act_live(&fx, &store, act));
    assert!(
        matches!(refused, Err(TierOneError::Unrenderable { .. })),
        "a whitespace-bearing kind was not refused at the mint: {refused:?}"
    );
    assert_eq!(head(&fx.root), before, "a refused mark committed");
    assert!(all_rows(&store).is_empty(), "a refused mark wrote a row");
}

// ------------------------------------------------------------ N7: the git journal and replay

/// **One `git_op` row per invocation the boundary made**, anchored to the act's commit, carrying
/// `GitInvocation`'s six members. Red under S8E-M29.
#[test]
fn the_boundary_journals_a_git_op_row_per_invocation() {
    let fx = fixture();
    let store = store();
    let mut boundary = Boundary::live(fx.exec.clone(), target(&fx));
    let done = boundary
        .act(&store, advance("implement", "review"))
        .expect("the act commits");
    let sha = done.sha().expect("live").to_owned();
    let invocations = boundary.writes_performed();
    assert!(
        invocations >= 2,
        "PREMISE: the boundary made {invocations} invocation(s) through its seam; a commit and \
         its re-read are two at least"
    );
    let ops: Vec<Event> = rows_at(&store, &sha)
        .into_iter()
        .filter(|e| e.kind == EventKind::GitOp)
        .collect();
    assert_eq!(
        ops.len(),
        invocations,
        "the seam made {invocations} invocation(s) and {} `git_op` row(s) were anchored",
        ops.len()
    );
    assert_eq!(
        ops[0].payload.get("op").and_then(serde_json::Value::as_str),
        Some("commit"),
        "the first journalled invocation is the commit: {:?}",
        ops[0].payload
    );
    for op in &ops {
        let mut members: Vec<&str> = op
            .payload
            .as_object()
            .expect("an object")
            .keys()
            .map(String::as_str)
            .collect();
        members.sort_unstable();
        assert_eq!(
            members,
            [
                "argv_digest",
                "cwd",
                "op",
                "status",
                "stderr_digest",
                "stdout"
            ],
            "a `git_op` row carries GitInvocation's own members and nothing else"
        );
        assert_eq!(op.item(), Some(ITEM), "a `git_op` row carries the item key");
    }
}

/// **Replay performs zero git writes** — read off `rk-git`'s own counter before and after, with
/// `HEAD` and the store unchanged. Red under S8E-M12.
#[test]
fn replay_of_a_boundary_performs_zero_git_writes() {
    let fx = fixture();
    let store = store();
    act_live(&fx, &store, advance("implement", "review")).expect("the live act");
    let recorded = all_rows(&store);
    let head_before = head(&fx.root);
    let rows_before = recorded.len();

    let mut journal = ActJournal::replaying(&recorded).expect("the recorded rows read back");
    let recorder = journal.recorder();
    let mut ops = GitOps::new(fx.exec.clone(), &mut journal, Mode::Replay);
    assert_eq!(
        ops.writes_performed(),
        0,
        "PREMISE: a fresh seam has written nothing"
    );
    let replayed = commit_act(
        &mut ops,
        &recorder,
        &store,
        &target(&fx),
        advance("implement", "review"),
    );
    assert!(replayed.is_ok(), "the replay refused: {replayed:?}");
    assert_eq!(
        ops.writes_performed(),
        0,
        "the replay reached the git wrapper"
    );
    assert_eq!(head(&fx.root), head_before, "the replay moved HEAD");
    assert_eq!(
        all_rows(&store).len(),
        rows_before,
        "the replay wrote store rows"
    );
}

/// **Replay is served from the act's `git_op` rows**: the replayed act reports the recorded commit.
/// Red under S8E-M29.
#[test]
fn replay_serves_the_boundary_from_its_git_op_rows() {
    let fx = fixture();
    let store = store();
    let live = act_live(&fx, &store, advance("implement", "review")).expect("the live act");
    let live_sha = live.sha().expect("live").to_owned();

    let recorded = all_rows(&store);
    let mut journal = ActJournal::replaying(&recorded).expect("the recorded rows read back");
    let recorder = journal.recorder();
    let mut ops = GitOps::new(fx.exec.clone(), &mut journal, Mode::Replay);
    let replayed = commit_act(
        &mut ops,
        &recorder,
        &store,
        &target(&fx),
        advance("implement", "review"),
    )
    .expect("the replay is served");
    assert!(
        matches!(&replayed, Committed::Replayed { sha, .. } if *sha == live_sha),
        "the replay was not served the recorded commit {live_sha}: {replayed:?}"
    );
}

/// **The stated rule for golden logs: a journal holding no `git_op` row is a run that made no git
/// write.** The committed golden log is read as it is — not re-recorded — and the boundary's
/// replay arm over it serves no commit and reaches no git.
#[test]
fn a_journal_without_git_op_rows_replays_as_no_git_writes() {
    let fx = fixture();
    let golden = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("logs")
        .join("implement-bf1d8cec3b22d288");
    let run = rk_kernel::replay::read_run(&golden).expect("the committed golden log reads");
    assert!(
        !run.events.is_empty(),
        "PREMISE: the golden log holds rows, so the rule is read over a real journal"
    );
    assert_eq!(
        run.events
            .iter()
            .filter(|e| e.kind == EventKind::GitOp)
            .count(),
        0,
        "PREMISE: the committed golden log carries no `git_op` row"
    );

    let store = store();
    let head_before = head(&fx.root);
    let mut journal = ActJournal::replaying(&run.events).expect("the golden rows read");
    let recorder = journal.recorder();
    let mut ops = GitOps::new(fx.exec.clone(), &mut journal, Mode::Replay);
    let replayed = commit_act(
        &mut ops,
        &recorder,
        &store,
        &target(&fx),
        advance("implement", "review"),
    );
    assert!(
        matches!(replayed, Ok(Committed::NoGitWrite)),
        "a journal with no `git_op` row must replay as no git write: {replayed:?}"
    );
    assert_eq!(ops.writes_performed(), 0, "the replay reached git");
    assert_eq!(head(&fx.root), head_before, "the replay moved HEAD");
    assert!(all_rows(&store).is_empty(), "the replay wrote a row");
}

// ------------------------------------------------------------------------------------ the heal

/// A mark committed on `rev` by git itself — the state a crash between the boundary's commit and
/// its rows leaves behind.
fn commit_mark(root: &Path, trailer: &str) -> String {
    git(
        root,
        &[
            "-c",
            "core.hooksPath=",
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            "a mark with no row",
            "--trailer",
            trailer,
        ],
    );
    head(root)
}

/// **A trailer commit on `rk-run/<item>` whose sha anchors no row is healed at entry** — git →
/// store, the direction ADR-0109 decision 1 permits, writing nothing to git (AC-174). Red under
/// S8E-M5.
#[test]
fn a_commit_without_a_row_is_healed_at_entry() {
    let fx = fixture();
    let store = store();
    let sha = commit_mark(&fx.root, &format!("Story: {PROJECT}/{ITEM} implement"));
    assert!(
        rows_at(&store, &sha).is_empty(),
        "PREMISE: the mark anchors no row before the heal"
    );
    let head_before = head(&fx.root);
    let repo = rk_git::path::WorktreePath::new(&fx.root).expect("normalises");

    let healed = heal(&fx.exec, &repo, &store, ITEM).expect("the heal runs");
    assert_eq!(
        healed,
        vec![sha.clone()],
        "the one rowless mark was not healed"
    );
    let rows = rows_at(&store, &sha);
    assert_eq!(
        rows.iter()
            .filter(|e| e.kind == EventKind::StageTransition)
            .count(),
        1,
        "the healed mark's `stage_transition` row is not anchored to {sha}: {rows:?}"
    );
    assert_eq!(
        rows[0]
            .payload
            .get("to")
            .and_then(serde_json::Value::as_str),
        Some("implement")
    );
    assert_eq!(head(&fx.root), head_before, "the heal wrote to git");

    let again = heal(&fx.exec, &repo, &store, ITEM).expect("a second heal runs");
    assert!(
        again.is_empty(),
        "a healed mark was healed twice: {again:?}"
    );
}

/// **A mark whose anchor is NOT reachable from `rk-run/<item>` heals nothing** — this branch's own
/// fixture. The control arm: the same repository's reachable mark IS healed. Red under S8E-M6.
#[test]
fn a_heal_writes_no_row_for_an_unreachable_anchor() {
    let fx = fixture();
    let store = store();
    let reachable = commit_mark(&fx.root, &format!("Story: {PROJECT}/{ITEM} implement"));
    git(
        &fx.root,
        &["checkout", "--quiet", "-b", "elsewhere", "main"],
    );
    let unreachable = commit_mark(&fx.root, &format!("Story: {PROJECT}/{ITEM} review"));
    git(&fx.root, &["checkout", "--quiet", BRANCH]);
    let repo = rk_git::path::WorktreePath::new(&fx.root).expect("normalises");

    let healed = heal(&fx.exec, &repo, &store, ITEM).expect("the heal runs");
    assert_eq!(
        healed,
        vec![reachable.clone()],
        "CONTROL arm: the reachable mark {reachable} must be healed, and only it"
    );
    assert!(
        rows_at(&store, &unreachable).is_empty(),
        "the heal wrote a row for {unreachable}, which rk-run/{ITEM} does not reach"
    );
}

/// **A `closed` mark heals as the lane's end** (`s8e` §8e.6; D21-43 N9): its row is `close_act`'s,
/// `stage_transition { from: <the last stage mark>, to: "closed" }`, anchored to the `closed`
/// commit — the heal's own branch for the mark, which no `lane_e2e` fixture reaches (a closed
/// item's `run` returns before the heal). *(Added 2026-09-28, `s8e` build, package S8E-P6.)*
#[test]
fn a_closed_mark_heals_as_the_lanes_end_transition() {
    let fx = fixture();
    let store = store();
    let entered = commit_mark(&fx.root, &format!("Story: {PROJECT}/{ITEM} review"));
    let closed = commit_mark(&fx.root, &format!("Story: {PROJECT}/{ITEM} closed"));
    let repo = rk_git::path::WorktreePath::new(&fx.root).expect("normalises");

    let healed = heal(&fx.exec, &repo, &store, ITEM).expect("the heal runs");
    assert_eq!(
        healed,
        vec![entered, closed.clone()],
        "the `closed` mark was not healed after its stage mark"
    );
    let rows = rows_at(&store, &closed);
    assert_eq!(
        rows.iter()
            .filter(|e| e.kind == EventKind::StageTransition)
            .map(|e| (
                e.payload
                    .get("from")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned),
                e.payload
                    .get("to")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned),
            ))
            .collect::<Vec<_>>(),
        vec![(Some("review".to_owned()), Some("closed".to_owned()))],
        "the `closed` commit {closed} does not anchor exactly `review -> closed`: {rows:?}"
    );
}

// ------------------------------------------------------------------------------ the source lints

/// The workspace root, from this crate's manifest.
fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root")
}

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|x| x == "rs") {
            out.push(path);
        }
    }
}

/// Every `crates/*/src/**.rs`, as `(workspace-relative path with '/', text)`.
fn src_files() -> Vec<(String, String)> {
    let root = workspace();
    let mut crates: Vec<PathBuf> = std::fs::read_dir(root.join("crates"))
        .expect("crates/")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    crates.sort();
    let mut files = Vec::new();
    for krate in crates {
        rs_files(&krate.join("src"), &mut files);
    }
    files
        .into_iter()
        .map(|p| {
            let rel = p
                .strip_prefix(&root)
                .expect("under the root")
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&p).expect("a readable source file");
            (rel, text)
        })
        .collect()
}

/// A CODE line: its first non-blank characters are not `//`, `/*`, or a `*` doc continuation.
fn is_code(line: &str) -> bool {
    let t = line.trim_start();
    !(t.starts_with("//")
        || t.starts_with("/*")
        || t == "*"
        || t.starts_with("* ")
        || t.starts_with("*/"))
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Does `line` name `word` as a whole identifier?
fn names(line: &str, word: &str) -> bool {
    line.match_indices(word).any(|(i, _)| {
        let before = line[..i].chars().next_back();
        let after = line[i + word.len()..].chars().next();
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })
}

/// `(path, line_no, line)` for every code line naming `word`.
fn code_hits(files: &[(String, String)], word: &str) -> Vec<(String, usize, String)> {
    let mut hits = Vec::new();
    for (path, text) in files {
        for (i, line) in text.lines().enumerate() {
            if is_code(line) && names(line, word) {
                hits.push((path.clone(), i + 1, line.to_owned()));
            }
        }
    }
    hits
}

const BOUNDARY: &str = "crates/rk-kernel/src/runtime/tier_one.rs";

/// The writer's uses, definitions dropped, outside the two allowed files.
fn stray_writer_uses(hits: &[(String, usize, String)]) -> Vec<(String, usize, String)> {
    hits.iter()
        .filter(|(_, _, line)| !line.contains("fn append_tier1"))
        .filter(|(path, _, _)| path != BOUNDARY && path != "crates/rk-ports/src/conformance.rs")
        .cloned()
        .collect()
}

/// **AC-162's reachability lint: the tier-1 writer is called only from the boundary**, in every
/// call form — a method call, a path call such as `EventStore::append_tier1(..)`, a bare path handed
/// on as a function value. Definitions are dropped and nothing else. The one allow-listed caller is
/// `rk-ports`' conformance arm, behind the `conformance` feature, which exercises the port rather
/// than applying a transition. PREMISE: the boundary itself calls it (a writer nothing calls is the
/// s7g lesson). Red under S8E-M46.
#[test]
fn the_tier_one_writer_is_called_only_from_the_boundary() {
    let files = src_files();
    assert!(
        files.len() > 100,
        "PREMISE: the walk read {} source files, which is not the workspace",
        files.len()
    );
    let hits = code_hits(&files, "append_tier1");
    let in_boundary = hits
        .iter()
        .filter(|(p, _, l)| p == BOUNDARY && !l.contains("fn append_tier1"))
        .count();
    assert!(
        in_boundary >= 1,
        "PREMISE: no code line of {BOUNDARY} calls `append_tier1`, so the boundary never writes"
    );
    let stray = stray_writer_uses(&hits);
    assert!(
        stray.is_empty(),
        "`append_tier1` is used outside the boundary and the conformance arm: {stray:#?}"
    );

    // CONTROL: a path-form call and a function-value use outside the boundary are KEPT; a
    // definition and a comment are DROPPED.
    let synthetic = vec![(
        "crates/rk-cli/src/x.rs".to_owned(),
        "    EventStore::append_tier1(&mut *s, e, &a)?;\n    acts.map(SqliteStore::append_tier1);\n    fn append_tier1(&mut self, e: Event, a: &TierOneAnchor) -> R;\n    // append_tier1 belongs to the kernel\n".to_owned(),
    )];
    let kept = stray_writer_uses(&code_hits(&synthetic, "append_tier1"));
    assert_eq!(
        kept.len(),
        2,
        "CONTROL: the filter kept {} of two stray uses, one definition and one comment: {kept:?}",
        kept.len()
    );

    // rk-store's tier_one statements are named only inside rk-store and rk-adapter-sqlite: the
    // rebuild's exemption from the lint above is not a second route in from rk-cli.
    let fenced: Vec<(String, usize, String)> = files
        .iter()
        .flat_map(|(path, text)| {
            text.lines()
                .enumerate()
                .filter(|(_, l)| is_code(l) && l.contains("rk_store::") && l.contains("tier_one"))
                .map(|(i, l)| (path.clone(), i + 1, l.to_owned()))
                .collect::<Vec<_>>()
        })
        .filter(|(p, _, _)| {
            !p.starts_with("crates/rk-store/src/")
                && !p.starts_with("crates/rk-adapter-sqlite/src/")
        })
        .collect();
    assert!(
        fenced.is_empty(),
        "a code line outside rk-store and rk-adapter-sqlite names rk-store's tier_one: {fenced:#?}"
    );
}

/// The `commit_with_trailers` code lines under `rk-cli/src` and `rk-kernel/src` outside the
/// boundary.
fn stray_commits(hits: &[(String, usize, String)]) -> Vec<(String, usize, String)> {
    hits.iter()
        .filter(|(p, _, _)| {
            p.starts_with("crates/rk-cli/src/") || p.starts_with("crates/rk-kernel/src/")
        })
        .filter(|(p, _, _)| p != BOUNDARY)
        .cloned()
        .collect()
}

/// **AC-518's write-path lint, at this stage's scope: the commit write.** Every code-line call of
/// `commit_with_trailers` under `crates/rk-cli/src/` and `crates/rk-kernel/src/` is in the
/// boundary, where `authorise_tier1_write` precedes it. Workspace-wide, the only other callers are
/// `rk-git`'s own definitions and `redkiln-migrate`'s one call, named with its reason: it commits a
/// migration branch, not a run (`s8e` §8e.14). The file-write and index-change halves are `s8f`'s
/// landing. Red at entry because `run.rs` and `answer.rs` call it directly. Red under S8E-M44.
#[test]
fn every_run_commit_resolves_through_the_worktree_resolver() {
    let files = src_files();
    let hits = code_hits(&files, "commit_with_trailers");
    let stray = stray_commits(&hits);
    assert!(
        stray.is_empty(),
        "a run commit outside the boundary, past no worktree resolver: {stray:#?}"
    );

    let boundary = files
        .iter()
        .find(|(p, _)| p == BOUNDARY)
        .map(|(_, t)| t.clone())
        .unwrap_or_default();
    let code: Vec<(usize, &str)> = boundary
        .lines()
        .enumerate()
        .filter(|(_, l)| is_code(l))
        .collect();
    let first = |needle: &str| {
        code.iter()
            .find(|(_, l)| l.contains(needle))
            .map(|(i, _)| *i)
    };
    let (Some(authorise), Some(commit)) = (
        first("authorise_tier1_write("),
        first("commit_with_trailers("),
    ) else {
        panic!(
            "PREMISE: {BOUNDARY} must authorise and then commit; authorise at {:?}, commit at {:?}",
            first("authorise_tier1_write("),
            first("commit_with_trailers(")
        );
    };
    assert!(
        authorise < commit,
        "the boundary commits (line {}) before it authorises (line {})",
        commit + 1,
        authorise + 1
    );

    // Workspace-wide: rk-git's definitions and write path, the boundary, and redkiln-migrate's one
    // call. Nothing else.
    let elsewhere: Vec<&(String, usize, String)> = hits
        .iter()
        .filter(|(p, _, _)| !p.starts_with("crates/rk-git/src/") && p != BOUNDARY)
        .collect();
    let migrate: Vec<&&(String, usize, String)> = elsewhere
        .iter()
        .filter(|(p, _, _)| p == "crates/redkiln-migrate/src/run_contract.rs")
        .collect();
    assert_eq!(
        migrate.len(),
        1,
        "redkiln-migrate's one migration-branch commit is the named exception: {elsewhere:#?}"
    );
    assert_eq!(
        elsewhere.len(),
        1,
        "a `commit_with_trailers` caller outside rk-git, the boundary and redkiln-migrate: \
         {elsewhere:#?}"
    );

    // CONTROL: an rk-cli call is kept, a comment naming it is dropped, and the boundary's own is
    // allowed.
    let synthetic = vec![
        (
            "crates/rk-cli/src/verbs/x.rs".to_owned(),
            "    rk_git::trailer::write::commit_with_trailers(&e, &p, m, &t, true)?;\n    // commit_with_trailers belongs to the boundary\n".to_owned(),
        ),
        (
            BOUNDARY.to_owned(),
            "    git.commit_with_trailers(owned.path(), &s, &[t], true)?;\n".to_owned(),
        ),
    ];
    let kept = stray_commits(&code_hits(&synthetic, "commit_with_trailers"));
    assert_eq!(
        kept.len(),
        1,
        "CONTROL: the filter kept {} of one stray call, one comment and one boundary call",
        kept.len()
    );
}
