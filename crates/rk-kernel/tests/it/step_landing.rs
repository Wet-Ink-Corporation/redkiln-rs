//! **N10's step-grain boundary act** — `s8f` §8f.4, the landing record ADR-0188 decision 2
//! (proposed). *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
//!
//! D21-43 N10: *"Land at every committed step that changed the worktree. The commit and the content
//! checkpoint are one boundary act."* A committed step's row is appended AFTER its content
//! checkpoint commit and anchored to it; a step that changed nothing makes no commit and its row is
//! appended unanchored. Each fixture is a real repository on `rk-run/<item>`, with the store's
//! `worktree_assigned` row (a created tree) and a first boundary act (the lane's entry mark), which
//! journals the baseline — so a file the step writes afterwards is one the run created.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_git::git_exec::GitExec;
use rk_git::worktree::RunOwnership;
use rk_kernel::runtime::tier_one::{Act, Boundary, Mark, Target, worktree_assigned_event};
use rk_kernel::runtime::{Runtime, activity::StepAt, stage_transition_event};
use rk_kernel::workflow::breaker::JournaledNow;
use rk_kernel::workflow::idem::{IterationPath, checked_key};
use rk_ports::event_store::{EventStore, Query, Snapshot, TierOneAnchor};
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::root::Ports;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::typed::{SchemaId, Typed};

const ITEM: &str = "step-1";
const PROJECT: &str = "rk-fixture";
const BRANCH: &str = "rk-run/step-1";
const ACTIVITY: &str = "implement-change@1";

fn git(root: &Path, args: &[&str]) -> String {
    match rk_git::git_exec::run(root, args) {
        Ok(out) => out.stdout_trimmed(),
        Err(e) => panic!("git {args:?} in {}: {e:?}", root.display()),
    }
}

/// A store that records, at every append, which commit `HEAD` was on and whether the row was
/// anchored — so "lands before its commit row" is read at the moment of the append.
struct Witness {
    inner: VecStore,
    root: PathBuf,
    seen: Seen,
}

/// `(kind, HEAD at the append, the anchor sha if anchored)`.
type Seen = Arc<Mutex<Vec<(EventKind, String, Option<String>)>>>;

impl EventStore for Witness {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        let at = git(&self.root, &["rev-parse", "HEAD"]);
        self.seen.lock().expect("lock").push((event.kind, at, None));
        self.inner.append(event)
    }
    fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError> {
        let at = git(&self.root, &["rev-parse", "HEAD"]);
        let sha = anchor_sha(anchor);
        self.seen
            .lock()
            .expect("lock")
            .push((event.kind, at, Some(sha)));
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

/// The anchor's sha.
fn anchor_sha(anchor: &TierOneAnchor) -> String {
    anchor.sha().to_owned()
}

struct Fx {
    _dir: tempfile::TempDir,
    root: PathBuf,
    store: Arc<Mutex<dyn EventStore>>,
    seen: Seen,
    rt: Runtime,
}

/// A repository on `rk-run/step-1`, a store holding the tree's `worktree_assigned` row, a
/// runtime bound to the boundary, and the lane's entry mark already committed (the baseline act).
fn fixture() -> Fx {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("repo");
    std::fs::create_dir_all(&root).expect("mkdir");
    git(&root, &["init", "--quiet", "-b", "main"]);
    for (k, v) in [
        ("user.name", "rk"),
        ("user.email", "rk@example.invalid"),
        ("commit.gpgsign", "false"),
        ("core.autocrlf", "false"),
    ] {
        git(&root, &["config", k, v]);
    }
    std::fs::write(root.join(".gitattributes"), b"* text=auto eol=lf\n").expect("attrs");
    std::fs::write(root.join("README.md"), b"step landing\n").expect("readme");
    git(&root, &["add", "."]);
    git(&root, &["commit", "--quiet", "-m", "base"]);
    git(&root, &["checkout", "--quiet", "-b", BRANCH]);
    let tc = rk_git::version::assert_at_startup(&root).expect("git at or above the floor");
    let exec = GitExec::new(dir.path().join("hooks-empty"), tc).expect("the wrapper");

    let seen: Seen = Arc::new(Mutex::new(Vec::new()));
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(Witness {
        inner: VecStore::default(),
        root: root.clone(),
        seen: Arc::clone(&seen),
    }));
    store
        .lock()
        .expect("lock")
        .append(worktree_assigned_event(ITEM, &root, BRANCH, false, &[]))
        .expect("the assignment row");
    let target = Target {
        worktree: root.clone(),
        ownership: RunOwnership {
            worktree: root.clone(),
            branch: BRANCH.to_owned(),
        },
        project: Some(PROJECT.to_owned()),
        item: ITEM.to_owned(),
    };
    let mut entry = Boundary::live(exec.clone(), target.clone());
    entry
        .act(
            &store,
            Act::marked(
                Mark::Stage("implement".to_owned()),
                vec![stage_transition_event("triage", "implement")],
            ),
        )
        .expect("the entry mark");
    let ports = Ports {
        store: Arc::clone(&store),
        harness: Arc::new(FakeHarness::scripted(Vec::new())),
        tracker: Arc::new(BoardTracker::default()),
        docs: Arc::new(RecordingDocs::default()),
        toolchain: Arc::new(FixtureToolchain::default()),
    };
    let rt = Runtime::new(ports, "implement-0123456789abcdef", "implement")
        .with_item(ITEM)
        .with_boundary(Boundary::live(exec, target));
    seen.lock().expect("lock").clear();
    Fx {
        _dir: dir,
        root,
        store,
        seen,
        rt,
    }
}

fn now() -> JournaledNow {
    JournaledNow::from_journal(Timestamp::from_unix_seconds(1_800_000_000))
}

/// Commit one agent step's result at the step grain, as the walk does.
fn commit_step(fx: &Fx) {
    let key = checked_key(ITEM, ACTIVITY, 1).expect("a valid key");
    let path = IterationPath::root();
    let at = StepAt {
        step_id: "edit",
        iteration_path: &path,
        family: None,
    };
    let typed = Typed::new(
        SchemaId::new("ImplReport"),
        serde_json::json!({"summary": "the step's result"}),
    );
    fx.rt
        .commit_at(&key, Some(at), &typed, now())
        .expect("the step commits");
}

fn head(root: &Path) -> String {
    git(root, &["rev-parse", "HEAD"])
}

/// **A committed step that changed the tree lands BEFORE its commit row**, in one act: the content
/// checkpoint commit exists (carrying the step's file, its step-grain key on its subject, its row
/// in its body and no tier-1 trailer), and the `activity_committed` row is appended while `HEAD`
/// already IS that commit, anchored to it. Red at entry because the row is appended with no
/// landing (no commit is made at all). Red under M8.
#[test]
fn a_committed_step_that_changed_the_tree_lands_before_its_commit_row() {
    let fx = fixture();
    let before = head(&fx.root);
    std::fs::write(fx.root.join("change.rs"), b"fn the_step_wrote_this() {}\n").expect("edit");

    commit_step(&fx);

    let after = head(&fx.root);
    assert_ne!(
        after, before,
        "the step changed the tree and no content checkpoint commit was made"
    );
    assert_eq!(
        git(
            &fx.root,
            &["rev-list", "--count", &format!("{before}..HEAD")]
        ),
        "1",
        "exactly one checkpoint commit"
    );
    assert!(
        git(&fx.root, &["ls-tree", "-r", "--name-only", "HEAD"])
            .lines()
            .any(|p| p == "change.rs"),
        "the step's file is not in its checkpoint"
    );
    let subject = git(&fx.root, &["log", "-1", "--format=%s", "HEAD"]);
    assert_eq!(
        subject,
        format!("rk step {ITEM} {ACTIVITY} edit []"),
        "the checkpoint names its step-grain key on its subject"
    );
    let body = git(&fx.root, &["log", "-1", "--format=%b", "HEAD"]);
    assert!(
        body.starts_with("rk-step-row {\"kind\":\"activity_committed\""),
        "the checkpoint's body carries the row the heal reads: {body}"
    );
    let trailers = git(
        &fx.root,
        &["log", "-1", "--format=%(trailers:only)", "HEAD"],
    );
    assert_eq!(trailers, "", "a step checkpoint carries no tier-1 trailer");
    assert!(
        rk_git::trailer::grammar::parse_body(&git(&fx.root, &["log", "-1", "--format=%B"]))
            .is_empty(),
        "the grammar reads a trailer off a step checkpoint"
    );

    let seen = fx.seen.lock().expect("lock").clone();
    let committed: Vec<_> = seen
        .iter()
        .filter(|(k, _, _)| *k == EventKind::ActivityCommitted)
        .collect();
    assert_eq!(committed.len(), 1, "exactly one commit row: {seen:?}");
    let (_, at_append, anchor) = committed[0];
    assert_eq!(
        at_append, &after,
        "the commit row was appended while HEAD was not yet the checkpoint — the row came first"
    );
    assert_eq!(
        anchor.as_deref(),
        Some(after.as_str()),
        "the commit row is not anchored to the step's checkpoint"
    );
    let anchored = fx
        .store
        .lock()
        .expect("lock")
        .query(Query::BySha(after.clone()))
        .expect("reads");
    assert!(
        anchored
            .iter()
            .any(|e| e.kind == EventKind::ActivityCommitted),
        "the store holds no commit row at the checkpoint {after}"
    );
}

/// **A committed step that changed nothing makes no commit**: `HEAD` does not move, the row is
/// appended unanchored, and the act READ the tree (its `status` read is journalled), so the
/// absence of a commit is the act's decision and not an act that never ran. Red at entry because
/// no act runs (no `status` read is journalled). Red under M30 (an empty commit is made).
#[test]
fn a_committed_step_that_changed_nothing_makes_no_commit() {
    let fx = fixture();
    let before = head(&fx.root);

    commit_step(&fx);

    assert_eq!(
        head(&fx.root),
        before,
        "a step that changed nothing made a commit"
    );
    let seen = fx.seen.lock().expect("lock").clone();
    let committed: Vec<_> = seen
        .iter()
        .filter(|(k, _, _)| *k == EventKind::ActivityCommitted)
        .collect();
    assert_eq!(committed.len(), 1, "exactly one commit row: {seen:?}");
    assert_eq!(
        committed[0].2, None,
        "a step that made no commit has no commit to anchor its row to"
    );
    let rows = fx
        .store
        .lock()
        .expect("lock")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("reads");
    let read_after_row = rows
        .iter()
        .skip_while(|e| e.kind != EventKind::ActivityCommitted)
        .any(|e| {
            e.kind == EventKind::GitOp
                && e.payload.get("op").and_then(serde_json::Value::as_str) == Some("status")
        });
    assert!(
        read_after_row,
        "the step act journalled no read of the tree, so nothing decided that it was unchanged"
    );
}
