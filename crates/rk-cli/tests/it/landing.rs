//! **`s8f` §8f.4 — the landing, end to end through `verbs::run::run`** (D21-41 b; D21-42 9; D21-43
//! N10; the landing record ADR-0188, proposed). *(Added 2026-09-28, s8f build, package
//! S8F-P3-LANDING-E2E.)*
//!
//! Every test here drives `rk_cli::verbs::run::run(` over `s8e`'s `lane_e2e` harness: the fixture
//! lane (`pack-min`'s `Lane` variant), the `ScriptedFactory` seam, and a scripted `FakeHarness`. The
//! scripted step is wrapped by [`Writing`], which edits the RUN-OWNED tree while it "works" — a
//! tracked modification, a created file, a denylisted file — so a landing has something to land.
//! Nothing here builds `ClaudeHarness`, and nothing can reach a real `claude`.
//!
//! What is read: git itself (`git show`, `git log`, `%(trailers)`, `for-each-ref`), the store's
//! rows, and the filesystem of the checkout and the run tree. No assertion reads a value the
//! product computed for the test.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rk_cli::exit::Exit;
use rk_ports::event_store::EventStore;
use rk_ports::fakes::FakeHarness;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::Typed;

use crate::lane_e2e::{self, GATE_ITEM, PROJECT, ScriptedFactory, THROUGH_ITEM, git_rc, git_read};
use crate::support;

/// The gate class a refused landing opens (ADR-0188 decision 6). Spelled here, not imported: the
/// test pins the product's spelling.
const LANDING_KIND: &str = "landing.refused-path";

/// What the scripted implement step writes as a RUN-CREATED file.
pub(crate) const LANDED: &[u8] = b"landed by the implement step\n";
/// What it appends to the one TRACKED file the fixture repository has.
const EDIT: &[u8] = b"edited by the implement step\n";

// ------------------------------------------------------------------------------- the harness

type Before = Box<dyn Fn(usize, &Path) + Send + Sync>;
type After = Box<dyn Fn(usize) + Send + Sync>;

/// A harness that edits the run-owned tree before each scripted dispatch answers (`before`, with
/// the 0-based dispatch index), then lets `after` act once the answer exists.
struct Writing {
    inner: FakeHarness,
    tree: PathBuf,
    n: AtomicUsize,
    before: Before,
    after: After,
}

impl Harness for Writing {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        let n = self.n.fetch_add(1, Ordering::SeqCst);
        (self.before)(n, &self.tree);
        let out = self.inner.dispatch(spec, store);
        (self.after)(n);
        out
    }
}

/// A factory over [`Writing`], whose script is `script` and whose edits are `before`.
pub(crate) fn writing(
    tree: &Path,
    script: Vec<Result<Typed, StepError>>,
    before: impl Fn(usize, &Path) + Send + Sync + 'static,
) -> ScriptedFactory {
    writing_then(tree, script, before, |_| {})
}

fn writing_then(
    tree: &Path,
    script: Vec<Result<Typed, StepError>>,
    before: impl Fn(usize, &Path) + Send + Sync + 'static,
    after: impl Fn(usize) + Send + Sync + 'static,
) -> ScriptedFactory {
    ScriptedFactory::over(Box::new(Writing {
        inner: FakeHarness::scripted(script),
        tree: tree.to_path_buf(),
        n: AtomicUsize::new(0),
        before: Box::new(before),
        after: Box::new(after),
    }))
}

pub(crate) fn write(tree: &Path, rel: &str, bytes: &[u8]) {
    let path = tree.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("the parent directory is created");
    }
    std::fs::write(&path, bytes).unwrap_or_else(|e| panic!("{} writes: {e}", path.display()));
}

fn append(tree: &Path, rel: &str, bytes: &[u8]) {
    let path = tree.join(rel);
    let mut all = std::fs::read(&path).unwrap_or_default();
    all.extend_from_slice(bytes);
    std::fs::write(&path, all).unwrap_or_else(|e| panic!("{} writes: {e}", path.display()));
}

/// `run <item>` in process through the seam; a halt fails the test by name.
pub(crate) fn run(root: &Path, item: &str, factory: &ScriptedFactory) -> Exit {
    lane_e2e::ran(
        rk_cli::verbs::run::run(
            &lane_e2e::cli(root, item),
            &lane_e2e::args(item),
            &support::clean_env(),
            lane_e2e::seam(factory),
        ),
        item,
    )
}

// ------------------------------------------------------------------------------- git, read

pub(crate) fn rev(root: &Path, spec: &str) -> Option<String> {
    git_read(root, &["rev-parse", "--verify", "--quiet", spec]).map(|s| s.trim().to_owned())
}

fn tree_of(root: &Path, rev_spec: &str) -> String {
    rev(root, &format!("{rev_spec}^{{tree}}")).unwrap_or_else(|| panic!("`{rev_spec}` has no tree"))
}

/// The blob at `<rev>:<path>`, or `None` when the path is not in that tree.
fn blob(root: &Path, rev_spec: &str, path: &str) -> Option<Vec<u8>> {
    git_read(root, &["cat-file", "-e", &format!("{rev_spec}:{path}")])?;
    git_read(root, &["show", &format!("{rev_spec}:{path}")]).map(String::into_bytes)
}

/// `(sha, subject)` of every commit on `rk-run/<item>`, oldest first.
fn subjects(root: &Path, item: &str) -> Vec<(String, String)> {
    git_read(
        root,
        &[
            "log",
            "--reverse",
            "--format=%H%x1f%s",
            &lane_e2e::run_ref(item),
        ],
    )
    .unwrap_or_default()
    .lines()
    .filter_map(|l| l.split_once('\u{1f}'))
    .map(|(s, m)| (s.to_owned(), m.to_owned()))
    .collect()
}

/// The step checkpoints on the ref: commits whose subject is `rk step <item> …` (ADR-0188
/// decision 3).
fn step_checkpoints(root: &Path, item: &str) -> Vec<(String, String)> {
    let prefix = format!("rk step {item} ");
    subjects(root, item)
        .into_iter()
        .filter(|(_, s)| s.starts_with(&prefix))
        .collect()
}

/// Every `(sha, tokens)` whose commit carries a `Gate:` value, oldest first.
fn gate_commits(root: &Path, item: &str) -> Vec<(String, Vec<String>)> {
    git_read(
        root,
        &[
            "log",
            "--reverse",
            "--format=%H%x1f%(trailers:key=Gate,valueonly,separator=%x1e)",
            &lane_e2e::run_ref(item),
        ],
    )
    .unwrap_or_default()
    .lines()
    .filter_map(|l| l.split_once('\u{1f}'))
    .flat_map(|(sha, values)| {
        values
            .split('\u{1e}')
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(|v| {
                (
                    sha.to_owned(),
                    v.split_whitespace().map(str::to_owned).collect(),
                )
            })
            .collect::<Vec<_>>()
    })
    .collect()
}

/// A commit is EMPTY when its tree is its first parent's.
pub(crate) fn is_empty_commit(root: &Path, sha: &str) -> bool {
    tree_of(root, sha) == tree_of(root, &format!("{sha}^1"))
}

// ----------------------------------------------------------------------------- store, read

/// The runtime-shaped `gate_opened` rows of the refused-landing class.
fn landing_gates(root: &Path) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some(LANDING_KIND))
        .collect()
}

/// `(path, rule)` of every refusal a landing gate's row names.
fn refused_of(row: &Event) -> Vec<(String, String)> {
    row.payload
        .get("refused")
        .and_then(serde_json::Value::as_array)
        .map(|a| {
            a.iter()
                .map(|r| {
                    (
                        r.get("path")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or_default()
                            .to_owned(),
                        r.get("rule")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or_default()
                            .to_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn count_kind(root: &Path, kind: EventKind) -> usize {
    support::store_rows(root)
        .iter()
        .filter(|e| e.kind == kind)
        .count()
}

/// The `activity_committed` rows carrying `step_id` — the step-grain commit rows (R1).
fn step_rows(root: &Path) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .filter(|e| e.payload.get("step_id").is_some() && e.payload.get("result").is_some())
        .collect()
}

/// The latest `worktree_assigned` row for `item`.
fn latest_assigned(root: &Path, item: &str) -> Event {
    support::store_rows(root)
        .into_iter()
        .rev()
        .find(|e| {
            e.kind == EventKind::WorktreeAssigned
                && e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
        })
        .unwrap_or_else(|| panic!("no `worktree_assigned` row for {item}"))
}

// ------------------------------------------------------------------------------- the tests

/// **A scripted step's edit lands on `rk-run/<item>`, and a second `run` keeps it.**
///
/// Invocation 1: `implement`'s step writes a new file and edits the tracked `README` in the run
/// tree; the script holds no `review` element, so the lane stops in `review`. Both edits are on
/// the run ref, in the step checkpoint's OWN commit (N10: `rk step <item> …`, whose parent lacks
/// the file). Invocation 2 re-enters (W2) and walks `review` to the `closed` mark with exactly one
/// dispatch (`implement` is not re-run), and the edit is still on the ref, its commit an ancestor
/// of the tip. Red under M1 (the run-created `add` dropped).
#[test]
fn fake_harness_edit_lands_on_the_run_ref_and_survives_a_second_run() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let tree = crate::run_tree(root, item).path;

    let first = writing(&tree, vec![lane_e2e::implement()], |n, t| {
        if n == 0 {
            write(t, "landed.txt", LANDED);
            append(t, "README", EDIT);
        }
    });
    let exit = run(root, item, &first);
    let stages = lane_e2e::story_stages(root, item);
    assert!(
        stages.iter().any(|s| s == "review") && !stages.iter().any(|s| s == "closed"),
        "PREMISE: invocation 1 crosses into `review` and stops there (exit {exit:?}): {stages:?}"
    );

    let run_ref = lane_e2e::run_ref(item);
    assert_eq!(
        blob(root, &run_ref, "landed.txt").as_deref(),
        Some(LANDED),
        "the run-created file is not on {run_ref} after invocation 1"
    );
    assert!(
        blob(root, &run_ref, "README").is_some_and(|b| b.ends_with(EDIT)),
        "the tracked edit is not on {run_ref} after invocation 1"
    );
    let checkpoints = step_checkpoints(root, item);
    let landing: Vec<&(String, String)> = checkpoints
        .iter()
        .filter(|(sha, _)| {
            blob(root, sha, "landed.txt").is_some()
                && blob(root, &format!("{sha}^1"), "landed.txt").is_none()
        })
        .collect();
    assert_eq!(
        landing.len(),
        1,
        "the edit must land in exactly one step checkpoint's own commit (N10); step \
         checkpoints: {checkpoints:?}"
    );
    let landed_at = landing[0].0.clone();

    let second = ScriptedFactory::new(vec![lane_e2e::review()]);
    assert_eq!(
        run(root, item, &second),
        Exit::Success,
        "invocation 2 walks `review` to the lane's end"
    );
    assert_eq!(
        (
            second.dispatches(),
            lane_e2e::story_stages(root, item).last().cloned()
        ),
        (1, Some("closed".to_owned())),
        "invocation 2 must dispatch `review` alone (the landed `implement` step is not re-run) and \
         reach `closed`"
    );
    assert_eq!(
        blob(root, &run_ref, "landed.txt").as_deref(),
        Some(LANDED),
        "the landed file did not survive the second run"
    );
    assert_eq!(
        git_rc(root, &["merge-base", "--is-ancestor", &landed_at, &run_ref]),
        Some(0),
        "the step checkpoint {landed_at} is no longer on {run_ref}"
    );
}

/// **The gate's `tree_hash` is its own commit's tree, and `answer` applies** (defect (b); ADR-0188
/// decision 12).
///
/// The operator's checkout carries an uncommitted edit, so the CHECKOUT's working state is not the
/// run tree's. `lane-1` opens its abstention gate: the `Gate: … opened` value's `tree_hash` equals
/// `<its commit>^{tree}` and the store row's, and `answer approve` hashes the run-owned tree and is
/// APPLIED (a `gate_answered` row, no `gate_answer_void`). Red under M7 (the gate tree from the
/// stage-start `WorkingState` of the checkout).
#[test]
fn gate_tree_hash_is_its_own_commits_tree_and_answer_does_not_void_it() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    append(root, "README", b"the operator's own uncommitted edit\n");
    assert_eq!(
        git_rc(root, &["diff", "--quiet"]),
        Some(1),
        "PREMISE: the checkout's working state differs from the run tree's"
    );

    assert_eq!(
        run(root, GATE_ITEM, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "invocation 1 exits 0 at the gate"
    );
    let opened: Vec<(String, Vec<String>)> = gate_commits(root, GATE_ITEM)
        .into_iter()
        .filter(|(_, m)| m.get(5).map(String::as_str) == Some("opened"))
        .collect();
    assert_eq!(opened.len(), 1, "PREMISE: one `opened` mark: {opened:?}");
    let (sha, mark) = &opened[0];
    let token = mark[0].clone();
    let tree_hash = mark[1].clone();
    assert_eq!(
        tree_hash,
        tree_of(root, sha),
        "the gate's tree_hash is not its own commit's tree"
    );
    let rows = support::store_rows(root);
    let row_tree = rows
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .find(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(&token))
        .and_then(|e| {
            e.payload
                .get("tree_hash")
                .and_then(serde_json::Value::as_str)
        })
        .map(str::to_owned);
    assert_eq!(
        row_tree.as_deref(),
        Some(tree_hash.as_str()),
        "the store's `gate_opened` row does not carry the mark's tree"
    );

    assert_eq!(
        lane_e2e::answer_item(root, GATE_ITEM, "approve"),
        Exit::Success
    );
    let rows = support::store_rows(root);
    assert_eq!(
        rows.iter()
            .filter(|e| e.kind == EventKind::GateAnswerVoid)
            .count(),
        0,
        "the answer was VOID: the gate bound a tree `answer` does not derive"
    );
    assert_eq!(
        rows.iter()
            .filter(|e| e.kind == EventKind::GateAnswered)
            .filter(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(&token))
            .count(),
        1,
        "the answer was not applied to {token}"
    );
}

/// **The operator's checkout is unchanged across a lane that LANDS** (AC-518's file-write and
/// index-change halves, ADR-0188 decision 11): its `HEAD`, its index (`ls-files -s`), its
/// `status`, and its bytes — while the lane lands a created file and a tracked edit on the run ref.
#[test]
fn the_user_checkout_is_unchanged_across_a_landing_lane() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    crate::git(root, &["add", "-A"]);
    crate::git(
        root,
        &["commit", "--quiet", "-m", "the user's committed checkout"],
    );
    let head = rev(root, "HEAD");
    let index = git_read(root, &["ls-files", "-s"]);
    let readme = std::fs::read(root.join("README")).expect("the checkout's README reads");
    let clean = |when: &str| {
        assert!(
            crate::porcelain(root).is_empty(),
            "{when}: the checkout's `status --porcelain` is not empty: {:?}",
            crate::porcelain(root)
        );
        assert_eq!(rev(root, "HEAD"), head, "{when}: the checkout's HEAD moved");
        assert_eq!(
            git_read(root, &["ls-files", "-s"]),
            index,
            "{when}: the checkout's index changed"
        );
        assert_eq!(
            std::fs::read(root.join("README")).ok().as_deref(),
            Some(readme.as_slice()),
            "{when}: the checkout's README changed"
        );
        assert!(
            !root.join("landed.txt").exists(),
            "{when}: the landed file appeared in the checkout"
        );
    };
    clean("PREMISE, before the lane");

    let tree = crate::run_tree(root, GATE_ITEM).path;
    assert_eq!(
        run(root, GATE_ITEM, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    clean("after invocation 1");
    assert_eq!(
        lane_e2e::answer_item(root, GATE_ITEM, "approve"),
        Exit::Success
    );
    clean("after the answer");
    let walked = writing(&tree, lane_e2e::through(), |n, t| {
        if n == 0 {
            write(t, "landed.txt", LANDED);
            append(t, "README", EDIT);
        }
    });
    assert_eq!(run(root, GATE_ITEM, &walked), Exit::Success);
    assert_eq!(
        blob(root, &lane_e2e::run_ref(GATE_ITEM), "landed.txt").as_deref(),
        Some(LANDED),
        "PREMISE: the lane really landed an edit on its run ref"
    );
    clean("after the lane's end");
}

/// **Only `rk-run/<item>` moves across a lane.** Every ref and its sha, before the lane and after
/// it (open, answer, walk with edits, `closed`): the one new ref is the run ref, and no other ref
/// is created, moved or deleted; the checkout's `HEAD` names the same branch.
#[test]
fn only_the_run_ref_moves_across_a_lane() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let refs = |root: &Path| -> std::collections::BTreeMap<String, String> {
        git_read(root, &["for-each-ref", "--format=%(refname) %(objectname)"])
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(r, s)| (r.to_owned(), s.to_owned()))
            .collect()
    };
    let before = refs(root);
    let symbolic = git_read(root, &["symbolic-ref", "HEAD"]);
    let tree = crate::run_tree(root, GATE_ITEM).path;

    assert_eq!(
        run(root, GATE_ITEM, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    assert_eq!(
        lane_e2e::answer_item(root, GATE_ITEM, "approve"),
        Exit::Success
    );
    let walked = writing(&tree, lane_e2e::through(), |n, t| {
        if n == 0 {
            write(t, "landed.txt", LANDED);
        }
    });
    assert_eq!(run(root, GATE_ITEM, &walked), Exit::Success);
    assert_eq!(
        lane_e2e::story_stages(root, GATE_ITEM)
            .last()
            .map(String::as_str),
        Some("closed"),
        "PREMISE: the lane walked to its end"
    );

    let after = refs(root);
    let run_ref = lane_e2e::run_ref(GATE_ITEM);
    let mut moved: Vec<String> = after
        .iter()
        .filter(|(r, s)| before.get(*r) != Some(*s))
        .map(|(r, _)| r.clone())
        .collect();
    moved.extend(
        before
            .keys()
            .filter(|r| !after.contains_key(*r))
            .map(|r| format!("{r} (deleted)")),
    );
    assert_eq!(
        moved,
        vec![run_ref.clone()],
        "only {run_ref} may be created or moved by a lane; before {before:?}, after {after:?}"
    );
    assert_eq!(
        git_read(root, &["symbolic-ref", "HEAD"]),
        symbolic,
        "the checkout's HEAD names another branch"
    );
}

/// **A refused path stops the lane at a `landing.refused-path` gate, and nothing lands**
/// (ADR-0188 decisions 5-7).
///
/// `implement`'s step writes `.env` (the denylist), a created file and a tracked edit. `run`
/// exits 0; one gate of the class opens, naming `.env` by its rule and nothing else; its `Gate:`
/// commit is empty and is the ref's tip; no step checkpoint exists, no step row was appended, and
/// none of the three edits is on the ref. `approve` is not an option of the gate's question (exit
/// 2); `abort` applies, and a re-run then stops the lane: no dispatch, no commit, no stage.
#[test]
fn a_refused_path_stops_the_lane_at_a_landing_gate_with_nothing_landed() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let tree = crate::run_tree(root, item).path;
    let fork = rev(root, "HEAD").expect("the checkout's HEAD");

    let first = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        write(t, ".env", b"API_TOKEN=not-a-real-secret\n");
        write(t, "notes.txt", b"a created file\n");
        append(t, "README", EDIT);
    });
    assert_eq!(
        run(root, item, &first),
        Exit::Success,
        "a refused landing is a gate, and `run` exits 0 at a gate"
    );
    assert_eq!(
        first.dispatches(),
        1,
        "PREMISE: `implement` dispatched once"
    );

    let gates = landing_gates(root);
    assert_eq!(
        gates.len(),
        1,
        "exactly one `{LANDING_KIND}` gate opens: {gates:?}"
    );
    assert_eq!(
        refused_of(&gates[0]),
        vec![(".env".to_owned(), "denylist:.env".to_owned())],
        "the gate names every refused path with its rule, and nothing else: {}",
        gates[0].payload
    );
    let marks = gate_commits(root, item);
    let tip = lane_e2e::ref_sha(root, item).expect("the run ref");
    let at_tip: Vec<&(String, Vec<String>)> = marks.iter().filter(|(s, _)| *s == tip).collect();
    assert!(
        at_tip.len() == 1
            && at_tip[0].1.get(3).map(String::as_str) == Some(LANDING_KIND)
            && at_tip[0].1.get(5).map(String::as_str) == Some("opened"),
        "the run ref's tip is not the `{LANDING_KIND}` gate's `opened` mark: {marks:?}"
    );
    assert!(
        is_empty_commit(root, &tip),
        "the refused landing's `Gate:` commit is not empty: something landed"
    );
    assert!(
        step_checkpoints(root, item).is_empty(),
        "a step checkpoint was committed for a refused landing: {:?}",
        step_checkpoints(root, item)
    );
    assert!(
        step_rows(root).is_empty(),
        "the refused step's commit row was appended: {:?}",
        step_rows(root)
    );
    for path in [".env", "notes.txt"] {
        assert!(
            blob(root, &tip, path).is_none(),
            "`{path}` landed on the run ref"
        );
    }
    assert_eq!(
        blob(root, &tip, "README"),
        blob(root, &fork, "README"),
        "the tracked edit landed on the run ref"
    );

    // The answer comes before the next entry: the refused step's edits are still in the run tree,
    // so `answer`'s derivation is the gate's tree (ADR-0188 decision 7). A re-entry first would
    // discard them, and the answer would be void (the re-entry test reads the awaiting re-run).
    assert_eq!(
        lane_e2e::answer_item(root, item, "approve"),
        Exit::Usage,
        "`approve` is not an option of the landing gate's question"
    );
    assert_eq!(
        lane_e2e::answer_item(root, item, "abort"),
        Exit::Success,
        "`abort` applies"
    );
    let rows = support::store_rows(root);
    assert!(
        rows.iter().any(|e| e.kind == EventKind::GateAnswered
            && e.payload.get("verdict").and_then(serde_json::Value::as_str) == Some("abort"))
            && !rows.iter().any(|e| e.kind == EventKind::GateAnswerVoid),
        "PREMISE: `abort` was APPLIED, not voided"
    );
    let answered_tip = lane_e2e::ref_sha(root, item);
    let after_abort = ScriptedFactory::new(Vec::new());
    assert_eq!(run(root, item, &after_abort), Exit::Success);
    let stages = lane_e2e::story_stages(root, item);
    assert!(
        after_abort.dispatches() == 0
            && lane_e2e::ref_sha(root, item) == answered_tip
            && !stages.iter().any(|s| s == "review" || s == "closed"),
        "an aborted lane moved: {} dispatches, stages {stages:?}",
        after_abort.dispatches()
    );
}

/// **A kill after a committed writer step resumes over its landed edit** (N10; finding 11;
/// ADR-0188 decision 3).
///
/// The kill is real: once `implement`'s scripted answer exists, the test takes the store's write
/// lock, so the step's landing commits (git first) and its row's append then fails — the window
/// between the commit and the row. Invocation 2 (lock released) heals the row from the step
/// checkpoint's body, anchored to its commit, and does NOT re-dispatch the step: it dispatches
/// `review` alone and reaches `closed`, the landed edit still on the ref. Red under M8 (the row
/// appended with no step landing).
#[test]
fn a_kill_after_a_committed_writer_step_resumes_over_its_landed_edit() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let tree = crate::run_tree(root, item).path;
    let common = crate::common_dir(root);

    let lock: Arc<Mutex<Option<rk_store::conn::Reader>>> = Arc::new(Mutex::new(None));
    let held = Arc::clone(&lock);
    let lock_dir = common.clone();
    let first = writing_then(
        &tree,
        vec![lane_e2e::implement()],
        |n, t| {
            if n == 0 {
                write(t, "landed.txt", LANDED);
            }
        },
        move |n| {
            if n == 0 {
                let reader = rk_store::conn::Reader::open(&lock_dir).expect("a second connection");
                reader
                    .conn()
                    .execute_batch("BEGIN IMMEDIATE")
                    .expect("the store's write lock is taken");
                *held.lock().expect("the lock slot") = Some(reader);
            }
        },
    );
    let killed = run(root, item, &first);
    drop(lock.lock().expect("the lock slot").take());

    let checkpoints = step_checkpoints(root, item);
    assert_eq!(
        checkpoints.len(),
        1,
        "PREMISE: the writer step's checkpoint is committed before the kill (exit {killed:?}): \
         {:?}",
        subjects(root, item)
    );
    let (landed_at, subject) = checkpoints[0].clone();
    assert_eq!(
        blob(root, &landed_at, "landed.txt").as_deref(),
        Some(LANDED),
        "PREMISE: the checkpoint carries the step's edit"
    );
    assert!(
        step_rows(root).is_empty(),
        "PREMISE: the kill left the step's row unwritten (the crash window): {:?}",
        step_rows(root)
    );

    let second = ScriptedFactory::new(vec![lane_e2e::review()]);
    assert_eq!(run(root, item, &second), Exit::Success);
    assert_eq!(
        (
            second.dispatches(),
            lane_e2e::story_stages(root, item).last().cloned()
        ),
        (1, Some("closed".to_owned())),
        "invocation 2 must NOT re-dispatch the landed step ({subject}): it dispatches `review` \
         alone and reaches `closed`"
    );
    let anchors = lane_e2e::sql_rows(
        &common,
        "SELECT CAST(anchor_sha AS TEXT) FROM t2_event WHERE kind = 'activity_committed' AND \
         json_extract(payload, '$.result') IS NOT NULL AND \
         json_extract(payload, '$.activity_id_at_version') LIKE 'emit-report@%'",
    )
    .unwrap_or_else(|e| panic!("the store reads: {e}"));
    assert_eq!(
        anchors,
        vec![vec![Some(landed_at.clone())]],
        "the step's row was not healed exactly once, anchored to its checkpoint {landed_at}"
    );
    assert_eq!(
        blob(root, &lane_e2e::run_ref(item), "landed.txt").as_deref(),
        Some(LANDED),
        "the landed edit is not on the run ref at the lane's end"
    );
}

/// **Re-entry names every discarded path, and the discarded set is the staging set** (ADR-0188
/// decision 13).
///
/// The checkout ignores `local.cfg` through its own UNTRACKED `.gitignore` and names it in
/// `.worktreeinclude`, so the carry copies it into the new run tree, where nothing ignores it: an
/// untracked file present at creation — the **baseline**. `implement`'s step writes `.env` (so the
/// landing refuses and nothing lands), a created file, a nested created file, an ignored
/// `build.log`, and a tracked edit. At the next entry the `worktree_assigned` row's `discarded` is
/// exactly the staging set — the tracked edit and the run-created files, computed here from git's
/// own `status` less the baseline — and the baseline file and the ignored file survive.
#[test]
fn reentry_names_every_discarded_path() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let tree = crate::run_tree(root, item).path;
    write(root, ".gitignore", b"local.cfg\n");
    write(root, ".worktreeinclude", b"local.cfg\n");
    write(root, "local.cfg", b"the operator's local setting\n");
    let exclude = crate::common_dir(root).join("info").join("exclude");
    let mut rows = std::fs::read(&exclude).unwrap_or_default();
    rows.extend_from_slice(b"\nbuild.log\n");
    std::fs::write(&exclude, rows).expect("the exclude row writes");

    let first = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        write(t, ".env", b"API_TOKEN=not-a-real-secret\n");
        write(t, "notes.txt", b"a created file\n");
        write(t, "sub/dir/created.txt", b"a nested created file\n");
        write(t, "build.log", b"an ignored file\n");
        append(t, "README", EDIT);
    });
    assert_eq!(run(root, item, &first), Exit::Success);
    assert_eq!(landing_gates(root).len(), 1, "PREMISE: the landing refused");
    assert_eq!(
        std::fs::read(tree.join("local.cfg")).ok().as_deref(),
        Some(&b"the operator's local setting\n"[..]),
        "PREMISE: the carry copied the baseline file into the run tree"
    );

    // The staging set, from git's own `status` in the run tree, less the baseline.
    let status = git_read(
        &tree,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .expect("the run tree's status reads");
    let listed: Vec<String> = status
        .split('\0')
        .filter(|f| f.len() > 3)
        .map(|f| f[3..].to_owned())
        .collect();
    assert!(
        listed.iter().any(|p| p == "local.cfg"),
        "PREMISE: the baseline file is untracked and not ignored in the run tree: {listed:?}"
    );
    // *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 spec-conformance finding.)*
    // The expected set is the ONE staging-set function's output over the same status bytes and
    // the journalled baseline, not a recomputation here (struck: `listed` less `local.cfg`,
    // sorted). The literal below stays as an independent pin of what that function names.
    let baseline =
        rk_kernel::runtime::tier_one::journalled_baseline(&support::store_rows(root), item)
            .expect("PREMISE: a baseline is journalled for the run tree");
    assert!(
        baseline.contains("local.cfg"),
        "PREMISE: the journalled baseline holds the carried file: {baseline:?}"
    );
    let expected = rk_git::landing::classify(
        rk_git::landing::parse_status(status.as_bytes()),
        &rk_git::landing::Baseline::Known(baseline),
        &std::collections::BTreeSet::new(),
    )
    .paths();
    assert_eq!(
        expected,
        [".env", "README", "notes.txt", "sub/dir/created.txt"],
        "PREMISE: the staging set the one function names"
    );

    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "invocation 2 re-enters and awaits the landing gate"
    );
    let assigned = latest_assigned(root, item);
    assert_eq!(
        assigned
            .payload
            .get("reused")
            .and_then(serde_json::Value::as_bool),
        Some(true),
        "PREMISE: invocation 2 re-entered the tree"
    );
    let mut discarded: Vec<String> = assigned
        .payload
        .get("discarded")
        .and_then(serde_json::Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    discarded.sort();
    assert_eq!(
        discarded, expected,
        "the discarded set is not the staging set (tracked edits and run-created files, less the \
         baseline, honouring ignores)"
    );
    assert_eq!(
        std::fs::read(tree.join("local.cfg")).ok().as_deref(),
        Some(&b"the operator's local setting\n"[..]),
        "the baseline file was discarded"
    );
    assert!(
        tree.join("build.log").is_file(),
        "the ignored file was discarded"
    );
    for gone in [".env", "notes.txt", "sub/dir/created.txt"] {
        assert!(!tree.join(gone).exists(), "`{gone}` survived the discard");
    }
    assert_eq!(
        git_read(&tree, &["status", "--porcelain", "--untracked-files=all"])
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        vec!["?? local.cfg".to_owned()],
        "after the discard the run tree holds the baseline file and nothing else in flight"
    );
}

/// **The lane's end lands the final stage's work** (N9's `closed` mark; ADR-0188 decisions 1-2).
///
/// `implement` writes one file and `review` another. At the lane's end the run ref's tip is the
/// `Story: <project>/<item> closed` commit, its tree carries BOTH stages' work, `<fork>..<tip>` is
/// not empty, and nothing is left in flight: the clean, landed tree is removed at the mark.
#[test]
fn the_lane_end_lands_the_final_stages_work() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let tree = crate::run_tree(root, item).path;
    let fork = rev(root, "HEAD").expect("the checkout's HEAD");
    let walked = writing(&tree, lane_e2e::through(), |n, t| match n {
        0 => write(t, "impl.txt", b"implement's work\n"),
        1 => write(t, "review.txt", b"review's work\n"),
        _ => {}
    });
    assert_eq!(run(root, item, &walked), Exit::Success);
    assert_eq!(walked.dispatches(), 2, "PREMISE: both stages dispatched");

    let tip = lane_e2e::ref_sha(root, item).expect("the run ref");
    let closed = format!("{PROJECT}/{item} closed");
    let tip_story = git_read(
        root,
        &[
            "log",
            "-1",
            "--format=%(trailers:key=Story,valueonly)",
            &tip,
        ],
    )
    .unwrap_or_default();
    assert_eq!(
        tip_story.trim(),
        closed,
        "the run ref's tip is not the lane's `closed` mark"
    );
    assert_eq!(
        blob(root, &tip, "review.txt").as_deref(),
        Some(&b"review's work\n"[..]),
        "the final stage's work is not in the `closed` commit's tree"
    );
    assert_eq!(
        blob(root, &tip, "impl.txt").as_deref(),
        Some(&b"implement's work\n"[..]),
        "the earlier stage's work is not in the `closed` commit's tree"
    );
    let diff = git_read(root, &["diff", "--name-only", &fork, &tip]).unwrap_or_default();
    let mut names: Vec<&str> = diff.lines().collect();
    names.sort_unstable();
    assert_eq!(
        names,
        ["impl.txt", "review.txt"],
        "`<fork>..rk-run/{item}` is not the lane's work"
    );
    assert!(
        !tree.exists(),
        "the run tree is kept at the `closed` mark, so work was left unlanded"
    );
}

/// **A refused landing is NOT due, and `answer` derives the gate's tree** (D21-47, review M6;
/// ADR-0188 decision 7).
///
/// `implement`'s step writes `.env` and edits the tracked `README`. The landing gate's `Gate:`
/// commit is empty (no checkpoint was due), and its `tree_hash` is clause 4's no-checkpoint-due
/// derivation over the run tree — NOT the empty commit's tree, which differs because a tracked file
/// is modified. `answer`'s identical derivation (`rk_git::tree::tree_hash`, `WorkingState`, over the
/// run tree) equals it, and `exclude` is APPLIED, never voided. Red under M48 (the empty commit's
/// tree recorded).
#[test]
fn a_refused_landing_is_not_due_and_answer_derives_the_gates_tree() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let tree = crate::run_tree(root, item).path;
    let first = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        write(t, ".env", b"API_TOKEN=not-a-real-secret\n");
        append(t, "README", EDIT);
    });
    assert_eq!(run(root, item, &first), Exit::Success);

    let tip = lane_e2e::ref_sha(root, item).expect("the run ref");
    let marks: Vec<(String, Vec<String>)> = gate_commits(root, item)
        .into_iter()
        .filter(|(_, m)| m.get(3).map(String::as_str) == Some(LANDING_KIND))
        .collect();
    assert_eq!(
        marks.len(),
        1,
        "PREMISE: one `{LANDING_KIND}` mark: {marks:?}"
    );
    let (sha, mark) = &marks[0];
    assert_eq!(*sha, tip, "PREMISE: the landing gate's mark is the tip");
    assert!(
        is_empty_commit(root, sha),
        "PREMISE: no checkpoint was due, so the gate's commit is empty"
    );
    let (token, gate_tree) = (mark[0].clone(), mark[1].clone());
    assert_ne!(
        gate_tree,
        tree_of(root, sha),
        "the gate recorded the EMPTY COMMIT's tree, with a tracked file modified (M48)"
    );

    let discovered = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let owned = rk_git::path::WorktreePath::new(&tree).expect("the run tree's path");
    let derived = rk_git::tree::tree_hash(
        &discovered.exec,
        &owned,
        &discovered.common_dir,
        &rk_git::tree::TreeBasis::WorkingState,
    )
    .expect("the no-checkpoint-due derivation");
    assert_eq!(
        derived.as_str(),
        gate_tree,
        "`answer`'s derivation over the run tree is not the gate's tree"
    );

    assert_eq!(lane_e2e::answer_item(root, item, "exclude"), Exit::Success);
    let rows = support::store_rows(root);
    assert_eq!(
        (
            rows.iter()
                .filter(|e| e.kind == EventKind::GateAnswered)
                .filter(
                    |e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(&token)
                )
                .count(),
            count_kind(root, EventKind::GateAnswerVoid)
        ),
        (1, 0),
        "`exclude` was not APPLIED to the landing gate {token}"
    );
}

// ------------------------------------------------------ the render path (§8f.5, §8f.6)
//
// *(Added 2026-09-28, s8f build, package S8F-P4-RENDER-WRITE-READ.)* The fixture lane's two
// activities are given an `emit` and an `artifact:` input, in the fixture repository's own catalog:
// `emit-report`'s agent step's output is emitted as `ImplReport`, and `review-min`'s agent step
// reads it back as `artifact:ImplReport`. So `implement` writes through `rk-store`'s writer (one
// `artifact.write`, landed by the step's N10 act) and `review` reads through `rk-store`'s reader
// (one `artifact.read`).

/// `emit-report` with an `emit` of its agent step's output.
const EMIT_REPORT: &str = r#"[activity]
id           = "emit-report"
version      = 1
stage        = "implement"
provides     = ["ImplReport"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "check"
kind    = "agent"
role    = "implementer"
outputs = "value:ImplReport"

[[step]]
id        = "emit"
kind      = "emit"
artifacts = { ImplReport = "${check.outputs}" }
"#;

/// `review-min` whose agent step reads `artifact:ImplReport`.
const REVIEW_MIN: &str = r#"[activity]
id           = "review-min"
version      = 1
stage        = "review"
provides     = ["Findings"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "review"
kind    = "agent"
role    = "implementer"
outputs = "value:Findings"
inputs  = { report = "artifact:ImplReport" }
"#;

/// The fixture lane, its catalog given the `emit` and the `artifact:` input above.
fn render_lane() -> tempfile::TempDir {
    let repo = lane_e2e::lane_repo();
    let catalog = repo.path().join(".redkiln").join("catalog");
    std::fs::write(catalog.join("emit-report.toml"), EMIT_REPORT).expect("the catalog is written");
    std::fs::write(catalog.join("review-min.toml"), REVIEW_MIN).expect("the catalog is written");
    repo
}

/// **D21-42 5b's two halves, over a fixture** (`s8f` §8f.6): the emitted artifact's
/// `artifact.write` digest equals the `rk-types` digest of the bytes at `rk-run/<item>:<path>`,
/// and those bytes are the committed `emit` binding's canonical bytes. The lane runs through
/// `verbs::run::run` on the `FakeHarness` seam to its `closed` mark.
#[test]
fn emitted_artifact_digest_matches_its_bytes_on_the_run_ref() {
    let repo = render_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let lane = ScriptedFactory::new(lane_e2e::through());
    assert_eq!(run(root, item, &lane), Exit::Success);
    assert_eq!(
        lane_e2e::story_stages(root, item)
            .last()
            .map(String::as_str),
        Some("closed"),
        "PREMISE: the lane walks to its end"
    );

    let writes: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some("ImplReport"))
        .collect();
    assert_eq!(
        writes.len(),
        1,
        "one emitted output, one `artifact.write`: {writes:?}"
    );
    let row = &writes[0].payload;
    let path = row["path"].as_str().expect("the row names its path");
    assert_eq!(path, format!(".redkiln/artifacts/{item}/ImplReport.json"));
    let run_ref = lane_e2e::run_ref(item);
    let bytes = blob(root, &run_ref, path)
        .unwrap_or_else(|| panic!("`{path}` is not on {run_ref}: the emit did not land"));
    assert_eq!(
        row["digest"],
        serde_json::json!(rk_types::digest::sha256(&bytes).as_str()),
        "the `artifact.write` digest is not the digest of the bytes on {run_ref}"
    );
    assert_eq!(
        bytes,
        rk_store::canon::canonical(&support::impl_report().value).expect("canonical"),
        "the landed sidecar is not the emitted value's canonical bytes"
    );
    assert_eq!(
        (
            row["stage"].as_str(),
            row["run"].as_str().map(|r| r.starts_with("implement-"))
        ),
        (Some("implement"), Some(true)),
        "the row does not name the stage run that wrote it"
    );
}

/// **AC-126's replay assertion**: in a lane's journal every `artifact.read` is preceded by its
/// entry's reconcile record — the `reconcile_started` row of the invocation that served it — and
/// the read is the reading stage run's. Invocation 1 walks `implement` (which emits) and enters
/// `review`, whose step reads `artifact:ImplReport` and then finds no scripted answer; invocation 2
/// re-enters, reconciles again, and `review` reads it again. Each entry's reads follow THAT entry's
/// record: none precedes the first, and the second entry's span holds reads of its own.
#[test]
fn every_artifact_read_in_a_lane_follows_its_entrys_reconcile() {
    let repo = render_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let first = ScriptedFactory::new(vec![lane_e2e::implement()]);
    run(root, item, &first);
    assert!(
        !lane_e2e::story_stages(root, item)
            .iter()
            .any(|s| s == "closed"),
        "PREMISE: invocation 1 stops before the lane's end"
    );
    let second = ScriptedFactory::new(vec![lane_e2e::review()]);
    assert_eq!(run(root, item, &second), Exit::Success);

    let rows = support::store_rows(root);
    let branch = format!("rk-run/{item}");
    let entries: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::ReconcileStarted)
        .filter(|(_, e)| {
            e.payload.get("branch").and_then(serde_json::Value::as_str) == Some(&branch)
        })
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        entries.len(),
        2,
        "PREMISE: two entries, two reconcile records"
    );
    let reads: Vec<(usize, &Event)> = rows
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::ArtifactRead)
        .collect();
    assert!(
        !reads.is_empty(),
        "PREMISE: the lane read an artifact (no vacuous green)"
    );
    let mut spans = std::collections::BTreeSet::new();
    for (at, read) in &reads {
        let entry = entries.iter().rposition(|&e| e < *at);
        assert!(
            entry.is_some(),
            "the `artifact.read` at row {at} precedes every reconcile record: {read:?}"
        );
        spans.extend(entry);
        assert_eq!(
            (
                read.payload["kind"].as_str(),
                read.payload["stage"].as_str()
            ),
            (Some("ImplReport"), Some("review")),
            "the read is not the reading stage's"
        );
        assert!(
            read.payload["run"]
                .as_str()
                .is_some_and(|r| r.starts_with("review-")),
            "the read names no stage run: {read:?}"
        );
    }
    assert_eq!(
        spans.into_iter().collect::<Vec<_>>(),
        vec![0, 1],
        "each entry's reads follow its own reconcile record: the re-entry read again after its own"
    );
}

// --------------------------------------------- the drift gate and the vouched root (§8f.5)
//
// *(Added 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED; the render record ADR-0189
// decisions 4, 5 and 10, proposed.)* The drift lane: `implement`'s `emit-report` emits its agent
// step's output as `ImplReport` AND as `Spec`, the one human-facing kind (layout C), and the lane
// gains a last stage, `wrapup` (optional, no activity), so `review` — where the drift is found —
// has a successor and its exit is an ADVANCE, the act the stage advance's drift check refuses. A
// human's edit of `Spec.md`'s structured section is a COMMIT on `rk-run/<item>` (§8f.4 re-entry
// item 3), made here with git itself in the run-owned tree.

/// The gate class the entry pass opens over a structured drift. Spelled here, not imported.
const DRIFT_KIND: &str = "render.structured-drift";

/// `emit-report`, emitting its agent step's output as `ImplReport` and as `Spec`.
const EMIT_SPEC: &str = r#"[activity]
id           = "emit-report"
version      = 1
stage        = "implement"
provides     = ["ImplReport"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "check"
kind    = "agent"
role    = "implementer"
outputs = "value:ImplReport"

[[step]]
id        = "emit"
kind      = "emit"
artifacts = { ImplReport = "${check.outputs}", Spec = "${check.outputs}" }
"#;

/// Replace `from` with `to` in `path`, asserting it was there exactly once.
fn patch(path: &Path, from: &str, to: &str) {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{} reads: {e}", path.display()));
    assert_eq!(
        text.matches(from).count(),
        1,
        "PREMISE: `{from}` occurs once in {}",
        path.display()
    );
    std::fs::write(path, text.replace(from, to))
        .unwrap_or_else(|e| panic!("{} writes: {e}", path.display()));
}

/// The fixture lane, with `Spec` emitted at `implement` and a last stage `wrapup` after `review`.
pub(crate) fn drift_lane() -> tempfile::TempDir {
    let repo = lane_e2e::lane_repo();
    let rk = repo.path().join(".redkiln");
    std::fs::write(rk.join("catalog").join("emit-report.toml"), EMIT_SPEC)
        .expect("the catalog is written");
    for pack in ["bug-standard.toml", "story.toml"] {
        let path = rk.join("packs").join(pack);
        patch(
            &path,
            r#"stage_order = ["triage", "discovery", "implement", "review"]"#,
            r#"stage_order = ["triage", "discovery", "implement", "review", "wrapup"]"#,
        );
        patch(
            &path,
            r#"review = "mandatory" }"#,
            r#"review = "mandatory", wrapup = "optional" }"#,
        );
    }
    patch(
        &rk.join("facets.toml"),
        r#"values = ["triage", "implement", "review"]"#,
        r#"values = ["triage", "implement", "review", "wrapup"]"#,
    );
    repo
}

/// `Spec.md`'s repo-relative path in the run-owned tree.
fn spec_md(item: &str) -> String {
    format!(".redkiln/artifacts/{item}/Spec.md")
}

/// **A human's edit of a structured section, committed on `rk-run/<item>`**: the canonical JSON
/// of `summary` in `Spec.md` is rewritten, and the run-owned tree commits it.
pub(crate) fn human_edits_the_spec(root: &Path, item: &str) -> String {
    let tree = crate::run_tree(root, item).path;
    let md = tree.join(spec_md(item));
    let text = std::fs::read_to_string(&md).expect("PREMISE: the rendering is in the run tree");
    let edited = text.replace("\"pack-min\"", "\"a human edited this\"");
    assert_ne!(
        edited, text,
        "PREMISE: the structured `summary` section is edited"
    );
    std::fs::write(&md, edited).expect("the rendering is edited");
    assert_eq!(git_rc(&tree, &["add", "--", &spec_md(item)]), Some(0));
    assert_eq!(
        git_rc(
            &tree,
            &["commit", "--quiet", "-m", "a human edits the spec"]
        ),
        Some(0),
        "PREMISE: the human's edit is a commit on the run ref"
    );
    lane_e2e::ref_sha(root, item).expect("the run ref names the human's commit")
}

/// Every `gate_opened` row of the drift gate's class.
fn drift_gates(root: &Path) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some(DRIFT_KIND))
        .collect()
}

/// Every `stage_transition` row INTO `stage`.
fn transitions_into(root: &Path, stage: &str) -> usize {
    support::store_rows(root)
        .iter()
        .filter(|e| e.kind == EventKind::StageTransition)
        .filter(|e| e.payload.get("to").and_then(serde_json::Value::as_str) == Some(stage))
        .count()
}

/// `answer <item> <option> [--attach <kind>=<path>]`, in process, as the operator types it.
pub(crate) fn answer(
    root: &Path,
    item: &str,
    option: &str,
    attach: Option<(&str, PathBuf)>,
) -> Exit {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::AnswerArgs {
        item: item.to_owned(),
        option: option.to_owned(),
        attach: attach
            .into_iter()
            .map(|(kind, path)| rk_cli::cli::Attachment {
                kind: kind.to_owned(),
                path,
            })
            .collect(),
        explain_sufficed: rk_cli::cli::YesNo::Yes,
        toml_opened: rk_cli::cli::YesNo::No,
    };
    rk_cli::verbs::answer::run(
        &cli,
        &args,
        // *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; s8h §8h.7.)* The fixture
        // launch record, explicitly: `answer::run` is bound by the launch check.
        &support::launched_env(),
        rk_cli::verbs::Startup::real(),
    )
    .unwrap_or_else(|halt| panic!("`answer {item}` halted: {halt:?}"))
}

/// Invocation 1 emits the spec and stops in `review`; the human's edit is committed; invocation 2
/// finds the drift at its entry. Returns the rendering's bytes as the store wrote them, and the
/// human's commit.
fn walk_into_a_drift_gate(root: &Path, item: &str) -> (Vec<u8>, String) {
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    let run_ref = lane_e2e::run_ref(item);
    let rendered = blob(root, &run_ref, &spec_md(item))
        .expect("PREMISE: invocation 1 landed the spec's rendering on the run ref");
    assert!(
        !lane_e2e::story_stages(root, item)
            .iter()
            .any(|s| s == "wrapup" || s == "closed"),
        "PREMISE: invocation 1 stops before the lane's end"
    );
    let human = human_edits_the_spec(root, item);
    let second = ScriptedFactory::new(vec![lane_e2e::review()]);
    assert_eq!(
        run(root, item, &second),
        Exit::Success,
        "`run` exits 0 at the drift gate, like any gate"
    );
    assert_eq!(
        second.dispatches(),
        1,
        "the stage's steps continue to dispatch behind the drift gate: `review` ran"
    );
    (rendered, human)
}

/// **N11: a `structured_drift` opens a drift gate that blocks the stage's exit until it is
/// answered** (`s8f` §8f.5; ADR-0189 decision 4). Invocation 2's entry pass appends one
/// `structured_drift` (the causing commit is the human's), restores the section from the type,
/// and opens ONE gate of class `render.structured-drift`, git first, whose `Gate:` commit lands
/// the restore. `review`'s step still runs; the advance into `wrapup` is refused — no boundary
/// commit, no row. Invocation 3, with the gate unanswered, is refused again and moves nothing on
/// the run ref. Once `answer … confirm-restore` applies, invocation 4 advances and closes the
/// lane. Red under M23 (no gate opens) and M47 (the advance's check removed).
#[test]
fn a_drift_gate_blocks_the_stage_exit_until_it_is_answered() {
    let repo = drift_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let (rendered, human) = walk_into_a_drift_gate(root, item);
    let run_ref = lane_e2e::run_ref(item);

    let drifts: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::StructuredDrift)
        .collect();
    assert_eq!(drifts.len(), 1, "one `structured_drift`: {drifts:?}");
    assert_eq!(
        drifts[0].payload["causing_commit"],
        serde_json::json!(human),
        "the causing commit is the human's"
    );
    assert_eq!(
        drifts[0].payload["anchor"],
        serde_json::json!("summary"),
        "the drifted section is named"
    );
    assert_eq!(
        blob(root, &run_ref, &spec_md(item)).as_deref(),
        Some(rendered.as_slice()),
        "the section is RESTORED from the type, and the restore is on the run ref"
    );

    let gates = drift_gates(root);
    assert_eq!(gates.len(), 1, "exactly one drift gate: {gates:?}");
    let token = gates[0].payload["token"]
        .as_str()
        .expect("the gate carries its token")
        .to_owned();
    assert_eq!(
        gates[0].payload["stage"],
        serde_json::json!("review"),
        "the gate is the stage's"
    );
    assert!(
        gate_commits(root, item)
            .iter()
            .any(|(_, v)| v.first() == Some(&token)
                && v.get(3).map(String::as_str) == Some(DRIFT_KIND)
                && v.get(5).map(String::as_str) == Some("opened")),
        "the gate is opened git-first: a `Gate: {token} … {DRIFT_KIND} … opened` commit"
    );
    assert!(
        !lane_e2e::story_stages(root, item)
            .iter()
            .any(|s| s == "wrapup"),
        "the stage's exit is BLOCKED: no `Story: … wrapup` mark"
    );
    assert_eq!(
        transitions_into(root, "wrapup"),
        0,
        "and no row into `wrapup`"
    );

    // ---- unanswered, a third invocation is refused again and moves nothing ---------------------
    let tip = lane_e2e::ref_sha(root, item);
    let transitions = count_kind(root, EventKind::StageTransition);
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "`run` exits 0 at the gate"
    );
    assert_eq!(
        lane_e2e::ref_sha(root, item),
        tip,
        "no boundary commit while the gate is unanswered: {:?}",
        subjects(root, item)
    );
    assert_eq!(
        count_kind(root, EventKind::StageTransition),
        transitions,
        "and no row"
    );
    assert_eq!(drift_gates(root).len(), 1, "and no second gate");

    // ---- answered, the exit is released ---------------------------------------------------------
    assert_eq!(answer(root, item, "confirm-restore", None), Exit::Success);
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    let stages = lane_e2e::story_stages(root, item);
    assert!(
        stages.iter().any(|s| s == "wrapup"),
        "the answered gate releases the exit: {stages:?}"
    );
    assert_eq!(stages.last().map(String::as_str), Some("closed"));
}

/// **The drift gate's two answers** (ADR-0189 decision 5), each releasing the stage's exit: a
/// table over `confirm-restore` (the restore stands; the value is unchanged) and
/// `move-to-prose:notes` (at the next entry the drifted bytes, read from the CAUSING COMMIT, are
/// appended to the `notes` prose section through the writer as one `artifact.write`, and the
/// structured section stays restored). In both arms the next `run` walks the lane to its `closed`
/// mark.
#[test]
fn answering_the_drift_gate_either_way_releases_the_stage_exit() {
    for option in ["confirm-restore", "move-to-prose:notes"] {
        let repo = drift_lane();
        let root = repo.path();
        let item = THROUGH_ITEM;
        walk_into_a_drift_gate(root, item);
        let run_ref = lane_e2e::run_ref(item);
        assert_eq!(
            drift_gates(root).len(),
            1,
            "{option}: PREMISE: the gate is open"
        );
        let sidecar = format!(".redkiln/artifacts/{item}/Spec.json");
        let before = blob(root, &run_ref, &sidecar).expect("PREMISE: the sidecar is landed");

        assert_eq!(
            answer(root, item, option, None),
            Exit::Success,
            "{option}: the answer applies"
        );
        let writes_before = count_kind(root, EventKind::ArtifactWrite);
        assert_eq!(
            run(root, item, &ScriptedFactory::new(Vec::new())),
            Exit::Success
        );
        let stages = lane_e2e::story_stages(root, item);
        assert_eq!(
            stages.last().map(String::as_str),
            Some("closed"),
            "{option}: the answer releases the stage's exit: {stages:?}"
        );
        let after = blob(root, &run_ref, &sidecar).expect("the sidecar is on the run ref");
        let notes = serde_json::from_slice::<serde_json::Value>(&after).expect("json")["notes"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if option == "confirm-restore" {
            assert_eq!(after, before, "{option}: the typed value is unchanged");
            assert!(notes.is_empty(), "{option}: nothing moved into prose");
        } else {
            assert!(
                notes.contains("a human edited this"),
                "{option}: the drifted bytes are in the `notes` prose section: {notes:?}"
            );
            assert_eq!(
                count_kind(root, EventKind::ArtifactWrite) - writes_before,
                1,
                "{option}: one `artifact.write` for the move"
            );
            let md = String::from_utf8(
                blob(root, &run_ref, &spec_md(item)).expect("the rendering is landed"),
            )
            .expect("utf-8");
            assert!(
                md.contains("\"pack-min\""),
                "{option}: the structured section stays restored from the type"
            );
        }
    }
}

/// The attached root's bytes, as vouched.
const VOUCHED_ROOT: &[u8] = b"{\"items\":[]}\n";
/// The root after a step edited it on the run ref.
const EDITED_ROOT: &[u8] = b"{\"items\":[\"a finding added after vouching\"]}\n";

/// **AC-127's first half, through the lane** (ADR-0189 decision 10). A `Findings` root is attached
/// at `lane-1`'s abstention gate (a pack question whose `vouch` option attaches it), stored at the
/// next entry under its vouched digest, and then edited by that invocation's `implement` step (so
/// the edit lands on the run ref). The NEXT invocation's entry appends ONE `drift_advisory`
/// `{ root, upstream: null, old_digest: vouched, new_digest }`; no `render.structured-drift` gate
/// opens; the root is not restored; and the next gate the item reaches — a refused landing's,
/// opened later in that invocation — carries the advisory. Answered `exclude`, the lane walks to
/// its `closed` mark: the changed root never blocked the stage's exit, and the new run records its
/// own advisory (AC-525's amendment (3)). Red under M36 (the change classed blocking).
#[test]
fn a_changed_vouched_root_does_not_block_the_stage_exit_and_the_next_gate_carries_it() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = GATE_ITEM;
    let processes = root.join(".redkiln").join("processes");
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("vouch.toml"),
        r#"[question."unknown_facet:has_repro"]
text = "Does the item reproduce? `vouch` attaches the human's findings."
options = ["approve", { id = "vouch", attaches = "Findings" }]
"#,
    )
    .expect("the question is written");

    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "PREMISE: the abstention gate opens"
    );
    write(root, "notes/brief.json", VOUCHED_ROOT);
    assert_eq!(
        answer(
            root,
            item,
            "vouch",
            Some(("Findings", root.join("notes/brief.json")))
        ),
        Exit::Success,
        "PREMISE: the root is attached"
    );
    let attach: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .filter(|e| e.payload.get("attached_path").is_some())
        .collect();
    assert_eq!(attach.len(), 1, "one attach row: {attach:?}");
    assert_eq!(
        attach[0].payload["attached_path"],
        serde_json::json!("notes/brief.json")
    );
    assert_eq!(attach[0].payload["stage"], serde_json::json!("implement"));
    assert!(
        attach[0].payload["run"]
            .as_str()
            .is_some_and(|r| r.starts_with("implement-")),
        "the attach row names the run that opened the gate: {attach:?}"
    );

    // ---- invocation 2: the root is stored at the entry, then `implement`'s step edits it --------
    let run_tree = crate::run_tree(root, item).path;
    let root_rel = format!(".redkiln/artifacts/{item}/Findings.root.json");
    let edit_rel = root_rel.clone();
    let second = writing(&run_tree, vec![lane_e2e::implement()], move |n, tree| {
        if n == 0 {
            write(tree, &edit_rel, EDITED_ROOT);
        }
    });
    run(root, item, &second);
    let run_ref = lane_e2e::run_ref(item);
    assert_eq!(
        blob(root, &run_ref, &root_rel).as_deref(),
        Some(EDITED_ROOT),
        "PREMISE: the root was stored and the step's edit of it landed"
    );
    assert_eq!(
        count_kind(root, EventKind::DriftAdvisory),
        0,
        "PREMISE: invocation 2's entry saw the root as vouched"
    );

    // ---- invocation 3: the change is advisory, and the next gate carries it --------------------
    let third = writing(&run_tree, vec![lane_e2e::review()], |n, tree| {
        if n == 0 {
            write(tree, ".env", b"SECRET=1\n");
        }
    });
    assert_eq!(
        run(root, item, &third),
        Exit::Success,
        "exit 0 at the landing's gate"
    );
    let advisories: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::DriftAdvisory)
        .collect();
    assert_eq!(advisories.len(), 1, "one advisory: {advisories:?}");
    let advisory = &advisories[0].payload;
    assert_eq!(
        advisory["upstream"],
        serde_json::Value::Null,
        "the own-change mark"
    );
    assert_eq!(advisory["root"], serde_json::json!("Findings"));
    assert_eq!(
        advisory["old_digest"],
        serde_json::json!(rk_types::digest::sha256(VOUCHED_ROOT).as_str())
    );
    assert_eq!(
        advisory["new_digest"],
        serde_json::json!(rk_types::digest::sha256(EDITED_ROOT).as_str())
    );
    assert!(
        drift_gates(root).is_empty(),
        "a vouched root's change opens NO drift gate"
    );
    assert_eq!(
        count_kind(root, EventKind::StructuredDrift),
        0,
        "and is not a structured drift"
    );
    assert_eq!(
        blob(root, &run_ref, &root_rel).as_deref(),
        Some(EDITED_ROOT),
        "the changed root is never restored"
    );
    let gates = landing_gates(root);
    assert_eq!(
        gates.len(),
        1,
        "PREMISE: the refused landing's gate opened: {gates:?}"
    );
    assert_eq!(
        gates[0].payload.get("advisories"),
        Some(&serde_json::Value::Array(vec![advisory.clone()])),
        "the NEXT gate the item reaches carries the advisory (ADR-0025; `carried_by_gate`)"
    );

    // ---- answered, the lane walks to its end past the changed root -----------------------------
    assert_eq!(answer(root, item, "exclude", None), Exit::Success);
    assert_eq!(
        run(root, item, &ScriptedFactory::new(vec![lane_e2e::review()])),
        Exit::Success
    );
    let stages = lane_e2e::story_stages(root, item);
    assert_eq!(
        stages.last().map(String::as_str),
        Some("closed"),
        "the changed root does not block the stage's exit: {stages:?}"
    );
    assert_eq!(
        count_kind(root, EventKind::DriftAdvisory),
        2,
        "the new run records its own advisory (AC-525's amendment (3): one per run)"
    );
    assert!(drift_gates(root).is_empty(), "and never a drift gate");
}

/// **A tree re-created on an existing run branch lands its entry's writes** (`s8f` §8f.4: *"The
/// baseline is the untracked set read at worktree creation"*; §8f.5: *"The entry's boundary lands
/// the file"*; ADR-0188 decision 4). Invocation 1 opens `lane-1`'s abstention gate, and `answer …
/// vouch` attaches a `Findings` root. The run tree is then removed and its branch kept, so
/// invocation 2 creates the tree anew ON THE EXISTING BRANCH (`Creation::OnExistingBranch`,
/// `reused: false`). Its entry pass stores the root in the managed directory before any act; the
/// baseline must be the tree as created, not as the entry pass left it, so the stored root is a
/// file the run created and lands on `rk-run/<item>`. *(Added 2026-09-29, s8f build, package
/// R2FIX-8f-A-CODE; refuting round 2's spec-conformance major.)* Red under M-BASE (the baseline
/// read at creation dropped).
#[test]
fn a_tree_recreated_on_an_existing_branch_lands_the_entrys_stored_root() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = GATE_ITEM;
    let processes = root.join(".redkiln").join("processes");
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("vouch.toml"),
        r#"[question."unknown_facet:has_repro"]
text = "Does the item reproduce? `vouch` attaches the human's findings."
options = ["approve", { id = "vouch", attaches = "Findings" }]
"#,
    )
    .expect("the question is written");
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "PREMISE: the abstention gate opens"
    );
    write(root, "notes/brief.json", VOUCHED_ROOT);
    assert_eq!(
        answer(
            root,
            item,
            "vouch",
            Some(("Findings", root.join("notes/brief.json")))
        ),
        Exit::Success,
        "PREMISE: the root is attached"
    );

    // The run tree is removed; `rk-run/<item>` is kept.
    let run_tree = crate::run_tree(root, item).path;
    let tree_arg = run_tree.to_string_lossy().into_owned();
    assert_eq!(
        git_rc(root, &["worktree", "remove", "--force", &tree_arg]),
        Some(0),
        "PREMISE: the run tree is removed"
    );
    assert!(!run_tree.exists(), "PREMISE: the run tree is gone");
    let run_ref = lane_e2e::run_ref(item);
    assert!(rev(root, &run_ref).is_some(), "PREMISE: its branch is kept");

    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    assert_eq!(
        latest_assigned(root, item).payload["reused"],
        serde_json::json!(false),
        "PREMISE: invocation 2 created the tree anew on the existing branch"
    );
    let root_rel = format!(".redkiln/artifacts/{item}/Findings.root.json");
    assert_eq!(
        std::fs::read(run_tree.join(&root_rel)).ok().as_deref(),
        Some(VOUCHED_ROOT),
        "PREMISE: the entry pass stored the root in the re-created tree"
    );
    assert_eq!(
        blob(root, &run_ref, &root_rel).as_deref(),
        Some(VOUCHED_ROOT),
        "the root the entry pass stored in a re-created tree never landed on {run_ref}: it was \
         read as baseline"
    );
}

/// **A `document` drift answered `move-to-prose` applies, and the lane walks on** (ADR-0189
/// decision 5; `s8f` §8f.6 reading 6). Invocation 1 emits `Spec`; a human DELETES its rendering, a
/// commit on `rk-run/<item>`; invocation 2's entry records one `structured_drift` at anchor
/// `document`, restores the file and opens the drift gate. `answer … move-to-prose:notes` is one of
/// the gate's offered options, so it must be answerable: invocation 3 applies it (nothing moves,
/// since the missing rendering had no bytes) and walks the lane to its `closed` mark, where before
/// it refused at every entry. *(Added 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting
/// round 2's spec-conformance major.)* Red under M-DOC.
#[test]
fn a_document_drift_answered_move_to_prose_applies_and_the_lane_walks_on() {
    let repo = drift_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    let run_ref = lane_e2e::run_ref(item);
    let rendered = blob(root, &run_ref, &spec_md(item))
        .expect("PREMISE: invocation 1 landed the spec's rendering on the run ref");
    let tree = crate::run_tree(root, item).path;
    assert_eq!(
        git_rc(&tree, &["rm", "--quiet", "--", &spec_md(item)]),
        Some(0),
        "PREMISE: the human deletes the rendering"
    );
    assert_eq!(
        git_rc(
            &tree,
            &[
                "commit",
                "--quiet",
                "-m",
                "a human deletes the spec's rendering"
            ]
        ),
        Some(0),
        "PREMISE: the deletion is a commit on the run ref"
    );
    assert_eq!(
        run(root, item, &ScriptedFactory::new(vec![lane_e2e::review()])),
        Exit::Success,
        "`run` exits 0 at the drift gate"
    );
    let drifts: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::StructuredDrift)
        .collect();
    assert_eq!(drifts.len(), 1, "PREMISE: one drift: {drifts:?}");
    assert_eq!(
        drifts[0].payload["anchor"],
        serde_json::json!("document"),
        "PREMISE: a missing rendering is a `document` drift"
    );
    assert_eq!(
        drift_gates(root).len(),
        1,
        "PREMISE: the drift gate is open"
    );

    assert_eq!(
        answer(root, item, "move-to-prose:notes", None),
        Exit::Success,
        "PREMISE: the offered answer is accepted"
    );
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "a `move-to-prose` answer to a `document` drift could not be applied at the next entry"
    );
    let stages = lane_e2e::story_stages(root, item);
    assert_eq!(
        stages.last().map(String::as_str),
        Some("closed"),
        "the answered gate releases the exit: {stages:?}"
    );
    assert_eq!(
        blob(root, &run_ref, &spec_md(item)).as_deref(),
        Some(rendered.as_slice()),
        "the rendering is restored from the type and landed"
    );
}

// ------------------------------- a later stage reads the ACCEPTED value (D21-113, R5FIX-8f)
//
// *(Added 2026-09-29, s8f build, package R5FIX-8f; refuting round 5's spec-conformance major;
// D21-113, TAKEN PROVISIONALLY under D21-105; ADR-0189 decision 9's dated note, proposed.)* An
// accepted prose edit (AC-152) and an applied `move-to-prose:<section>` each rewrite `Spec.json`
// through the writer, with its own `artifact.write`, and append NO new `emit` binding. A later
// `artifact:Spec` reader must be served that value: the reader checks the served bytes against the
// LATEST `artifact.write` of the sidecar's path (the binding only when there is none), and the
// resolver serves what the reader served. Before D21-113 every such read was a Halt 4
// (`DigestMismatch`), and, with the check gone, the resolver would still have served the stale
// binding. The drift lane's `review-min` is given a mandatory `artifact:Spec` input here, and the
// harness records each dispatch's assembled inputs, so the assertion is the served TEXT.

/// `review-min` whose agent step reads the whole `Spec`, mandatory.
const REVIEW_SPEC: &str = r#"[activity]
id           = "review-min"
version      = 1
stage        = "review"
provides     = ["Findings"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "review"
kind    = "agent"
role    = "implementer"
outputs = "value:Findings"
inputs  = { spec = "artifact:Spec" }
"#;

/// The drift lane, `review-min` reading `artifact:Spec`.
fn spec_read_lane() -> tempfile::TempDir {
    let repo = drift_lane();
    let catalog = repo.path().join(".redkiln").join("catalog");
    std::fs::write(catalog.join("review-min.toml"), REVIEW_SPEC).expect("the catalog is written");
    repo
}

/// Each dispatch's assembled inputs, recorded BEFORE the scripted answer (so a dispatch whose
/// script is exhausted is still recorded: its inputs were served).
type Seen = Arc<Mutex<Vec<Vec<(String, serde_json::Value)>>>>;

struct InputsSeen {
    inner: FakeHarness,
    seen: Seen,
}

impl Harness for InputsSeen {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        self.seen
            .lock()
            .expect("not poisoned")
            .push(spec.inputs.clone());
        self.inner.dispatch(spec, store)
    }
}

fn capturing(script: Vec<Result<Typed, StepError>>) -> (ScriptedFactory, Seen) {
    let seen: Seen = Arc::new(Mutex::new(Vec::new()));
    (
        ScriptedFactory::over(Box::new(InputsSeen {
            inner: FakeHarness::scripted(script),
            seen: Arc::clone(&seen),
        })),
        seen,
    )
}

/// The `spec` input the LAST recorded dispatch was served, or a failure naming why there is none
/// (with every stored row that names Halt 4, so a refused read is named by its refusal).
fn served_spec(root: &Path, seen: &Seen) -> serde_json::Value {
    let halts: Vec<String> = support::store_rows(root)
        .iter()
        .map(|e| e.payload.to_string())
        .filter(|p| p.contains("Halt 4"))
        .collect();
    let last = seen.lock().expect("not poisoned").last().cloned();
    let last = last.unwrap_or_else(|| {
        panic!(
            "the later stage never dispatched: its `artifact:Spec` read was refused; rows naming \
             Halt 4: {halts:?}"
        )
    });
    last.iter()
        .find(|(name, _)| name == "spec")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| panic!("the dispatch carried no `spec` input: {last:?}"))
}

/// What the human writes into `Spec.md`'s `context` prose section.
const HUMAN_CONTEXT: &str = "A human wrote this context after the spec was emitted.";

/// **T1 (D21-113): an accepted prose edit is SERVED to a later stage's mandatory `artifact:Spec`**
/// (AC-152: *"the edit is accepted as the field's new value"*; AC-477: *"the typed value is read back
/// from the store"*). Invocation 1 emits `Spec` at `implement` and enters `review`, whose read is
/// served and whose dispatch finds no scripted answer. A human writes prose into the `context`
/// section of `Spec.md`, a commit on `rk-run/<item>`. Invocation 2's entry accepts it through the
/// writer (a new `Spec.json`, its own `artifact.write`, no new `emit` binding), and `review`'s read
/// is served THAT value: the assertion is the text. Red before D21-113 with `DigestMismatch` (Halt
/// 4); red under M2 (the resolver serves the binding) and M5 (the check always compares against the
/// binding).
/// *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-114.)* ~~`#[ignore]`d, R5FIX-8f BLOCKED~~:
/// un-ignored as written, once the entry's accepted write lands at entry (D21-114) and D21-113 step 3
/// is built. Red before step 3 (the stale `{"summary":"pack-min"}` served), then red at step 3 alone
/// (`Unsupported`, the livelock), then green with the entry landing.
#[test]
fn an_accepted_prose_edit_is_served_to_a_later_stages_artifact_read() {
    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let (first, seen_first) = capturing(vec![lane_e2e::implement()]);
    run(root, item, &first);
    assert!(
        !served_spec(root, &seen_first)["context"]
            .as_str()
            .is_some_and(|c| c.contains(HUMAN_CONTEXT)),
        "PREMISE: invocation 1's `review` was served the emitted value, without the human's text"
    );

    let tree = crate::run_tree(root, item).path;
    let md = tree.join(spec_md(item));
    let text = std::fs::read_to_string(&md).expect("PREMISE: the rendering is in the run tree");
    assert_eq!(
        text.matches("## context\n").count(),
        1,
        "PREMISE: one `context` prose section"
    );
    let edited = text.replace("## context\n", &format!("## context\n{HUMAN_CONTEXT}\n"));
    std::fs::write(&md, edited).expect("the rendering is edited");
    assert_eq!(git_rc(&tree, &["add", "--", &spec_md(item)]), Some(0));
    assert_eq!(
        git_rc(
            &tree,
            &[
                "commit",
                "--quiet",
                "-m",
                "a human writes the spec's context"
            ]
        ),
        Some(0),
        "PREMISE: the human's prose edit is a commit on the run ref"
    );

    let (second, seen) = capturing(vec![lane_e2e::review()]);
    assert_eq!(run(root, item, &second), Exit::Success);
    let served = served_spec(root, &seen);
    assert!(
        served["context"]
            .as_str()
            .is_some_and(|c| c.contains(HUMAN_CONTEXT)),
        "the later stage's `artifact:Spec` was not served the accepted prose: {served}"
    );
    assert_eq!(
        served["summary"],
        serde_json::json!("pack-min"),
        "the served value lost the emitted structured field: {served}"
    );
}

/// **T2 (D21-113): an applied `move-to-prose:<section>` is SERVED to a later stage's mandatory
/// `artifact:Spec`, and a read after the entry's RESTORE is served too** (ADR-0189 decision 5).
/// Invocation 1 emits `Spec`; a human edits its structured `summary` section, a commit on the run
/// ref. Invocation 2's entry records the drift, RESTORES `Spec.md` (an `artifact.write` naming the
/// `.md`, not the sidecar) and opens the drift gate; `review`'s read is served (its dispatch finds
/// no scripted answer). `answer … move-to-prose:notes` applies at invocation 3's entry, a new
/// `Spec.json` through the writer, and `review`'s read is served THAT value: the drifted bytes in
/// `notes`. Red before D21-113 with `DigestMismatch` at invocation 3; red under M1 (the latest write
/// selected by KIND, which after the restore is the `.md` row) at invocation 2's read.
/// *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-114.)* ~~`#[ignore]`d, R5FIX-8f BLOCKED~~:
/// un-ignored as written, once the entry's accepted write lands at entry (D21-114) and D21-113 step 3
/// is built. Red before step 3 (the stale `{"summary":"pack-min"}` served), then red at step 3 alone
/// (`Unsupported`, the livelock), then green with the entry landing.
#[test]
fn a_move_to_prose_answer_is_served_to_a_later_stages_artifact_read() {
    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    human_edits_the_spec(root, item);

    // Its dispatch finds no scripted answer, so the exit is the dispatch's, not the gate's: only
    // the served read and the open gate are asserted.
    let (second, seen_second) = capturing(Vec::new());
    run(root, item, &second);
    assert_eq!(
        drift_gates(root).len(),
        1,
        "PREMISE: the drift gate is open"
    );
    let restored_writes = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .filter(|e| {
            e.payload.get("path").and_then(serde_json::Value::as_str)
                == Some(spec_md(item).as_str())
        })
        .count();
    assert_eq!(
        restored_writes, 1,
        "PREMISE: the entry's restore appended one `artifact.write` naming the `.md`"
    );
    let after_restore = served_spec(root, &seen_second);
    assert_eq!(
        after_restore["summary"],
        serde_json::json!("pack-min"),
        "the read after the restore was not served the typed value: {after_restore}"
    );

    assert_eq!(
        answer(root, item, "move-to-prose:notes", None),
        Exit::Success,
        "PREMISE: the offered answer is accepted"
    );
    let (third, seen) = capturing(vec![lane_e2e::review()]);
    assert_eq!(run(root, item, &third), Exit::Success);
    let served = served_spec(root, &seen);
    assert!(
        served["notes"]
            .as_str()
            .is_some_and(|n| n.contains("a human edited this")),
        "the later stage's `artifact:Spec` was not served the moved prose: {served}"
    );
    assert_eq!(
        served["summary"],
        serde_json::json!("pack-min"),
        "the structured field is the type's, restored: {served}"
    );
}

// --------------------------------------- the entry's own writes land AT ENTRY (D21-114, R5FIX2-8f)
//
// *(Added 2026-09-29, s8f build, package R5FIX2-8f; D21-114, TAKEN PROVISIONALLY under D21-105,
// amending D21-113 step 3; ADR-0188 decision 2's and ADR-0189 decision 9's dated notes, proposed.)*
// D21-113 step 3's reader checks the run ref's sidecar against the LATEST `artifact.write` of its
// path. The entry pass's own writes (an accepted prose edit, an applied `move-to-prose`, a stored
// vouched root) used to land only at the stage's first committed step, after that step's read, so
// the check livelocked (measured by R5FIX-8f: `Unsupported`, 0 dispatches, `Spec.json` writes
// 2 -> 3 -> 4). They now land AT ENTRY, through `commit_act`'s `Content::Entry` arm: a trailerless,
// body-less `rk entry <item> <stage> <run_id>` content commit carrying only its `git_op` rows, made
// only when the entry wrote and no drift gate's `Gate:` commit lands the write.

/// `Spec.json`'s repo-relative path in the run-owned tree.
fn spec_json(item: &str) -> String {
    format!(".redkiln/artifacts/{item}/Spec.json")
}

/// The `rk entry <item> ` commits on the run ref, `(sha, subject)`, oldest first.
fn entry_landings(root: &Path, item: &str) -> Vec<(String, String)> {
    let prefix = format!("rk entry {item} ");
    subjects(root, item)
        .into_iter()
        .filter(|(_, s)| s.starts_with(&prefix))
        .collect()
}

/// Every `artifact.write` naming `Spec.json`, in journal order.
fn spec_json_writes(root: &Path, item: &str) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .filter(|e| {
            e.payload.get("path").and_then(serde_json::Value::as_str)
                == Some(spec_json(item).as_str())
        })
        .collect()
}

/// The number of `git_op` rows in the store.
fn git_op_rows(root: &Path) -> usize {
    count_kind(root, EventKind::GitOp)
}

/// Invocation 1 emits `Spec` at `implement` and stops in `review`; a human then writes prose into
/// `Spec.md`'s `context` section, a commit on `rk-run/<item>` (T1's premise, taken again).
fn a_human_writes_the_context(root: &Path, item: &str) {
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    let tree = crate::run_tree(root, item).path;
    let md = tree.join(spec_md(item));
    let text = std::fs::read_to_string(&md).expect("PREMISE: the rendering is in the run tree");
    assert_eq!(
        text.matches("## context\n").count(),
        1,
        "PREMISE: one `context` prose section"
    );
    let edited = text.replace("## context\n", &format!("## context\n{HUMAN_CONTEXT}\n"));
    std::fs::write(&md, edited).expect("the rendering is edited");
    assert_eq!(git_rc(&tree, &["add", "--", &spec_md(item)]), Some(0));
    assert_eq!(
        git_rc(
            &tree,
            &[
                "commit",
                "--quiet",
                "-m",
                "a human writes the spec's context"
            ]
        ),
        Some(0),
        "PREMISE: the human's prose edit is a commit on the run ref"
    );
}

/// **D21-114: an accepted prose edit lands AT ENTRY, before the stage's first read** — so D21-113
/// step 3's latest-write check is met at once and the lane cannot livelock. Invocation 2's entry
/// accepts the human's `context` prose (one new `Spec.json` write) and lands it; the stage's
/// `artifact:Spec` read is served and `review` DISPATCHES (its script is empty, so it fails after
/// the read and the stage re-enters). The run ref's `Spec.json` blob IS the latest write. Invocations
/// 3 and 4 re-enter and write `Spec.json` no more: nothing was discarded, so nothing is accepted
/// again. Red before the entry landing: invocation 2 dispatches 0 times (`Unsupported`, the read's
/// Halt 4), and each re-entry discards and re-accepts (writes 2 -> 3 -> 4). Red under M-ENTRY-LAND.
#[test]
fn an_accepted_prose_edit_lands_at_entry_before_the_first_read() {
    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    a_human_writes_the_context(root, item);

    let (second, seen) = capturing(Vec::new());
    run(root, item, &second);
    let dispatches = seen.lock().expect("not poisoned").len();
    let halts = support::store_rows(root)
        .iter()
        .map(|e| e.payload.to_string())
        .filter(|p| p.contains("Halt 4"))
        .count();
    assert!(
        dispatches > 0,
        "invocation 2 dispatched {dispatches} times: its `artifact:Spec` read was refused before \
         any dispatch (rows naming Halt 4: {halts}), the entry's accepted write was not on the run \
         ref"
    );
    let writes = spec_json_writes(root, item);
    let latest = writes
        .last()
        .and_then(|e| e.payload.get("digest"))
        .and_then(serde_json::Value::as_str)
        .expect("PREMISE: `Spec.json` has a recorded write")
        .to_owned();
    let on_ref = blob(root, &lane_e2e::run_ref(item), &spec_json(item))
        .expect("`Spec.json` is on the run ref");
    assert_eq!(
        rk_types::digest::sha256(&on_ref).as_str(),
        latest,
        "the run ref's `Spec.json` is not the latest recorded write after invocation 2's entry"
    );

    let after_two = writes.len();
    for n in 3..=4 {
        let (again, _) = capturing(Vec::new());
        run(root, item, &again);
        assert_eq!(
            spec_json_writes(root, item).len(),
            after_two,
            "invocation {n} wrote `Spec.json` again: the entry's accepted write was discarded and \
             re-accepted"
        );
    }
}

/// **D21-114: the entry landing is IGNORED by the heal and the tier-1 folds, by construction.**
/// Its commit is subject-only (no body, no trailer), its subject begins `rk entry `, never `rk
/// step `, and with every row deleted (a fresh, empty store) the heal gives it none while it heals
/// the lane's trailer commits (the control arm: the heal ran and could heal), and
/// `marked_commits` does not list it. Red before the entry landing (no `rk entry` commit), and
/// under M-ENTRY-HEAL (the step read widened to `rk `: the heal refuses the entry commit as an
/// unreadable step checkpoint).
#[test]
fn an_entry_landing_is_ignored_by_heal_and_the_tier1_folds() {
    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    a_human_writes_the_context(root, item);
    let (second, _) = capturing(Vec::new());
    run(root, item, &second);

    let landings = entry_landings(root, item);
    assert_eq!(
        landings.len(),
        1,
        "invocation 2's accepted write made no single `rk entry {item} ` commit: {:?}",
        subjects(root, item)
    );
    let (sha, subject) = landings[0].clone();
    assert!(
        !subject.starts_with("rk step "),
        "an entry landing's subject begins the step checkpoint's prefix: {subject}"
    );
    let message =
        git_read(root, &["log", "-1", "--format=%B", &sha]).expect("the commit's message reads");
    assert_eq!(
        message.trim_end(),
        subject,
        "the entry landing carries more than its subject (a body or a trailer)"
    );
    assert!(
        !is_empty_commit(root, &sha),
        "the entry landing is not a content commit"
    );

    let found = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let empty: Arc<Mutex<dyn EventStore>> =
        Arc::new(Mutex::new(rk_ports::fakes::VecStore::default()));
    let healed = rk_kernel::runtime::tier_one::heal(&found.exec, &found.worktree, &empty, item)
        .expect("the heal runs over a store with every row deleted");
    assert!(
        !healed.is_empty(),
        "CONTROL: the heal healed nothing, so it did not read this lane's trailer commits"
    );
    assert!(
        !healed.contains(&sha),
        "the heal healed the entry landing {sha}"
    );
    let anchored = empty
        .lock()
        .expect("not poisoned")
        .query(rk_ports::event_store::Query::BySha(sha.clone()))
        .expect("the store reads");
    assert!(
        anchored.is_empty(),
        "the heal anchored rows to the entry landing: {anchored:?}"
    );
    let marked = rk_kernel::runtime::tier_one::marked_commits(&found.exec, &found.worktree, item)
        .expect("the fold reads");
    assert!(
        !marked.is_empty(),
        "CONTROL: `marked_commits` lists none of the lane's trailer commits"
    );
    assert!(
        marked.iter().all(|c| c.sha != sha),
        "`marked_commits` lists the entry landing {sha}"
    );
}

/// **D21-114: an entry that writes nothing makes no entry act** — no `rk entry` commit and no
/// `git_op` row. The control arm first: invocation 2's entry accepts the human's prose and makes
/// ONE entry landing (so the call site is live). Invocation 3's entry writes nothing: the landing
/// is not invoked, so the run ref gains no `rk entry` commit and the store no `git_op` row (an
/// invocation on an unchanged tree would still journal its reads, unanchored). Red before the entry
/// landing (the control arm: no `rk entry` commit), and red if the landing ran on every entry.
#[test]
fn an_entry_that_writes_nothing_makes_no_entry_act() {
    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    a_human_writes_the_context(root, item);
    let (second, _) = capturing(Vec::new());
    run(root, item, &second);
    assert_eq!(
        entry_landings(root, item).len(),
        1,
        "CONTROL: an entry that accepted a write made no single entry landing"
    );

    let ops_before = git_op_rows(root);
    let ref_before = lane_e2e::ref_sha(root, item);
    let (third, _) = capturing(Vec::new());
    run(root, item, &third);
    assert_eq!(
        entry_landings(root, item).len(),
        1,
        "an entry that wrote nothing made an entry landing"
    );
    assert_eq!(
        lane_e2e::ref_sha(root, item),
        ref_before,
        "an entry that wrote nothing moved the run ref"
    );
    assert_eq!(
        git_op_rows(root),
        ops_before,
        "an entry that wrote nothing journalled `git_op` rows"
    );
}

/// **D21-114: a replayed entry landing makes no git write.** Its act's rows are ONLY `git_op` rows
/// (no new kind, no completion-shaped row: D21-113 step 5), and replaying the act through
/// `commit_act` over them is SERVED the recorded commit: `rk-git`'s own counter reads 0, the run
/// ref does not move, and no row is appended. Red before the entry landing (no `rk entry` commit).
#[test]
fn a_replayed_entry_landing_makes_no_git_write() {
    use rk_git::gitops::{GitOps, Mode};
    use rk_kernel::runtime::tier_one::{
        Act, ActJournal, Committed, EntryCheckpoint, Target, commit_act,
    };

    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    a_human_writes_the_context(root, item);
    let (second, _) = capturing(Vec::new());
    run(root, item, &second);
    let landings = entry_landings(root, item);
    assert_eq!(
        landings.len(),
        1,
        "PREMISE: invocation 2 made one entry landing: {:?}",
        subjects(root, item)
    );
    let (sha, subject) = landings[0].clone();

    let found = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let sqlite = rk_adapter_sqlite::SqliteStore::open(&found.exec, &found.worktree)
        .expect("the store opens");
    let anchored = sqlite
        .query(rk_ports::event_store::Query::BySha(sha.clone()))
        .expect("the store reads");
    assert!(
        !anchored.is_empty() && anchored.iter().all(|e| e.kind == EventKind::GitOp),
        "the entry landing anchors rows other than its `git_op` rows, or none: {:?}",
        anchored.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    let journal = support::journal_rows(root, item);
    drop(sqlite);

    let store: Arc<Mutex<dyn EventStore>> =
        Arc::new(Mutex::new(rk_ports::fakes::VecStore::default()));
    for row in &journal {
        store
            .lock()
            .expect("not poisoned")
            .append(row.clone())
            .expect("the journal is seeded");
    }
    let tree = crate::run_tree(root, item);
    let target = Target {
        worktree: tree.path.clone(),
        ownership: rk_git::worktree::RunOwnership {
            worktree: tree.path,
            branch: tree.branch,
        },
        project: Some(PROJECT.to_owned()),
        item: item.to_owned(),
    };
    let ref_before = lane_e2e::ref_sha(root, item);
    let rows_before = journal.len();

    let mut replaying = ActJournal::replaying(&anchored).expect("the recorded rows read back");
    let recorder = replaying.recorder();
    let mut ops = GitOps::new(found.exec, &mut replaying, Mode::Replay);
    let replayed = commit_act(
        &mut ops,
        &recorder,
        &store,
        &target,
        Act::entry(EntryCheckpoint { subject }),
    )
    .expect("the replayed entry landing is served");
    assert!(
        matches!(&replayed, Committed::Replayed { sha: served, .. } if *served == sha),
        "the replay was not served the recorded entry landing {sha}: {replayed:?}"
    );
    assert_eq!(
        ops.writes_performed(),
        0,
        "the replay reached the git wrapper"
    );
    assert_eq!(
        lane_e2e::ref_sha(root, item),
        ref_before,
        "the replay moved the run ref"
    );
    assert_eq!(
        store
            .lock()
            .expect("not poisoned")
            .query(rk_ports::event_store::Query::ByRun(String::new()))
            .expect("reads")
            .len(),
        rows_before,
        "the replay appended a row"
    );
}

/// **D21-114, M-ENTRY-GATE: a drift entry lands its accepted write ONCE, through its drift gate.**
/// The human's one commit edits the structured `summary` (a drift) AND the prose `context` (an
/// accepted write). The entry pass records the drift, accepts the prose and restores; the drift
/// gate's `Gate:` commit (the landed arm) lands all of it (ADR-0189 decision 4), so the entry
/// landing is NOT invoked: invocation 2 adds no `rk entry` commit, and exactly ONE content commit
/// (the gate's) carries the entry's writes, and every `git_op` row invocation 2 journals is the
/// gate's, anchored to it. Red under M-ENTRY-GATE (the entry landing runs with the gate pending:
/// before the gate, an `rk entry` content commit and the gate's commit then empty; after it, an
/// unchanged tree's reads journalled as `git_op` rows the gate does not anchor).
#[test]
fn a_drift_entry_lands_its_accepted_write_once_through_its_gate() {
    let repo = spec_read_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    let tree = crate::run_tree(root, item).path;
    let md = tree.join(spec_md(item));
    let text = std::fs::read_to_string(&md).expect("PREMISE: the rendering is in the run tree");
    let edited = text
        .replace("\"pack-min\"", "\"a human edited this\"")
        .replace("## context\n", &format!("## context\n{HUMAN_CONTEXT}\n"));
    assert_ne!(edited, text, "PREMISE: the rendering is edited");
    std::fs::write(&md, edited).expect("the rendering is edited");
    assert_eq!(git_rc(&tree, &["add", "--", &spec_md(item)]), Some(0));
    assert_eq!(
        git_rc(
            &tree,
            &["commit", "--quiet", "-m", "a human edits the spec twice"]
        ),
        Some(0)
    );
    let human = lane_e2e::ref_sha(root, item).expect("the run ref names the human's commit");
    let writes_before = spec_json_writes(root, item).len();
    let ops_before = git_op_rows(root);

    let (second, _) = capturing(Vec::new());
    run(root, item, &second);
    assert_eq!(
        drift_gates(root).len(),
        1,
        "PREMISE: the drift gate is open"
    );
    let ops_journalled = git_op_rows(root) - ops_before;
    assert!(
        spec_json_writes(root, item).len() > writes_before,
        "PREMISE: the entry accepted the prose edit (a new `Spec.json` write) beside the drift"
    );
    assert!(
        entry_landings(root, item).is_empty(),
        "a drift entry made an entry landing: the gate's `Gate:` commit lands its writes"
    );
    let after: Vec<String> = git_read(
        root,
        &[
            "rev-list",
            "--reverse",
            &format!("{human}..{}", lane_e2e::run_ref(item)),
        ],
    )
    .unwrap_or_default()
    .lines()
    .map(str::to_owned)
    .collect();
    let content: Vec<&String> = after.iter().filter(|c| !is_empty_commit(root, c)).collect();
    assert_eq!(
        content.len(),
        1,
        "the drift entry landed its writes in {} content commits, not one: {after:?}",
        content.len()
    );
    assert!(
        gate_commits(root, item)
            .iter()
            .any(|(sha, _)| sha == content[0]),
        "the one content commit {} is not the drift gate's `Gate:` commit",
        content[0]
    );
    let found = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let sqlite = rk_adapter_sqlite::SqliteStore::open(&found.exec, &found.worktree)
        .expect("the store opens");
    let anchored_to_gate = sqlite
        .query(rk_ports::event_store::Query::BySha(content[0].clone()))
        .expect("the store reads")
        .into_iter()
        .filter(|e| e.kind == EventKind::GitOp)
        .count();
    assert!(
        anchored_to_gate > 0,
        "PREMISE: the gate's act anchors its `git_op` rows"
    );
    assert_eq!(
        ops_journalled, anchored_to_gate,
        "invocation 2 journalled {ops_journalled} `git_op` rows and the gate's commit anchors \
         {anchored_to_gate}: another act ran at the drift entry"
    );
}

// --------------------------- a refusal on the drift gate's landing, and an unchanged entry tree
//
// *(Added 2026-09-29, s8f build, package R7FIX-8f; refuting round 7's two majors; D21-114, TAKEN
// PROVISIONALLY under D21-105; ADR-0188 decisions 2, 6 and 7, proposed.)* When the entry also
// drifted, the entry's writes land through the drift gate's `Gate:` commit, not `land_entry`, so a
// refusal there is the FOURTH refusal site (a step's act, the entry landing, `answer`'s gate rows,
// and this one). And the entry landing's `allow_empty` false arm: an entry whose writes leave the
// run tree unchanged makes no `rk entry` commit.

/// Invocation 1 emits `Spec` and stops in `review`; a human's ONE commit then edits the structured
/// `summary` (a drift) AND writes prose over the blob limit into `context` (an accepted write), so
/// invocation 2's entry both drifts and writes, and both managed files go over the limit.
/// Returns the human's commit.
fn a_human_drifts_the_spec_over_the_blob_limit(root: &Path, item: &str) -> String {
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    let tree = crate::run_tree(root, item).path;
    let md = tree.join(spec_md(item));
    let text = std::fs::read_to_string(&md).expect("PREMISE: the rendering is in the run tree");
    let over = usize::try_from(rk_git::landing::MAX_LANDED_BLOB_BYTES).expect("fits") + 10;
    let prose = "x".repeat(over);
    let edited = text
        .replace("\"pack-min\"", "\"a human edited this\"")
        .replace("## context\n", &format!("## context\n{prose}\n"));
    assert_ne!(edited, text, "PREMISE: the rendering is edited");
    std::fs::write(&md, edited).expect("the rendering is edited");
    assert_eq!(git_rc(&tree, &["add", "--", &spec_md(item)]), Some(0));
    assert_eq!(
        git_rc(
            &tree,
            &[
                "commit",
                "--quiet",
                "-m",
                "a human drifts the spec and writes prose over the limit"
            ]
        ),
        Some(0),
        "PREMISE: the human's edit is a commit on the run ref"
    );
    lane_e2e::ref_sha(root, item).expect("the run ref names the human's commit")
}

/// **A refusal on the drift gate's landing opens the `landing.refused-path` gate, and the lane is
/// not stuck** (ADR-0188 decisions 6-7; `s8f` §8f.4's R7FIX-8f note). The entry both drifts and
/// accepts prose over the blob limit, so the drift gate's `Gate:` commit would land both managed
/// files and refuses them by size. Invocation 2 then opens ONE `landing.refused-path` gate over an
/// EMPTY commit naming both paths with `size:<bytes>`, opens no drift gate, dispatches nothing,
/// lands nothing and exits `0`. The follow-up proceeds for each answer: ~~after `exclude`, invocation
/// 3 opens the drift gate (the stated reading: it opens at the next entry once the exclusion is in
/// force), refuses nothing again, lands neither path and exits `0`;~~ *(dated 2026-09-29, s8f build,
/// package D21115-8f; D21-115)* `exclude` is refused (both paths are the item's managed artifacts)
/// and `abort` then releases the lane; after `abort`, invocation 3
/// stops the lane, exit `0`, and the run ref does not move. Red before the fix: invocation 2 exits
/// `3` with no gate, and so does every later invocation. Red under M-DRIFT-REFUSED.
#[test]
fn a_refused_drift_gate_landing_opens_its_gate_and_the_lane_is_not_stuck() {
    for option in ["exclude", "abort"] {
        let repo = drift_lane();
        let root = repo.path();
        let item = THROUGH_ITEM;
        let human = a_human_drifts_the_spec_over_the_blob_limit(root, item);
        let run_ref = lane_e2e::run_ref(item);
        let json_before = blob(root, &human, &spec_json(item));
        assert!(
            json_before.is_some(),
            "PREMISE: `Spec.json` is on the run ref"
        );

        let second = ScriptedFactory::new(vec![lane_e2e::review()]);
        assert_eq!(
            run(root, item, &second),
            Exit::Success,
            "[{option}] `run` does not exit 0 at the drift gate landing's refusal"
        );
        let tree = crate::run_tree(root, item).path;
        let size_of = |rel: &str| {
            std::fs::metadata(tree.join(rel))
                .unwrap_or_else(|e| panic!("PREMISE: {rel} is in the run tree: {e}"))
                .len()
        };
        let mut expected = vec![
            (
                spec_json(item),
                format!("size:{}", size_of(&spec_json(item))),
            ),
            (spec_md(item), format!("size:{}", size_of(&spec_md(item)))),
        ];
        expected.sort();
        let gates = landing_gates(root);
        assert_eq!(
            gates.len(),
            1,
            "[{option}] the refused drift-gate landing opened no single `{LANDING_KIND}` gate: \
             {gates:?}"
        );
        let mut refused = refused_of(&gates[0]);
        refused.sort();
        assert_eq!(
            refused, expected,
            "[{option}] the gate does not name both refused paths with `size:<bytes>`"
        );
        assert!(
            drift_gates(root).is_empty(),
            "[{option}] a drift gate opened over a refused landing"
        );
        assert_eq!(
            second.dispatches(),
            0,
            "[{option}] a step dispatched past the refused landing"
        );
        let tip = lane_e2e::ref_sha(root, item).expect("the run ref exists");
        assert!(
            gate_commits(root, item).iter().any(|(sha, _)| *sha == tip),
            "[{option}] the run ref's tip is not the refused landing's `Gate:` commit"
        );
        let after: Vec<String> = git_read(root, &["rev-list", &format!("{human}..{run_ref}")])
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect();
        assert!(
            !after.is_empty() && after.iter().all(|c| is_empty_commit(root, c)),
            "[{option}] a content commit followed the refused drift-gate landing: {after:?}"
        );
        assert_eq!(
            blob(root, &run_ref, &spec_json(item)),
            json_before,
            "[{option}] the refused `Spec.json` landed"
        );
        let token = gates[0].payload["token"]
            .as_str()
            .expect("the gate's token")
            .to_owned();
        // *(dated 2026-09-29, s8f build, package D21115-8f; D21-115, TAKEN PROVISIONALLY under
        // D21-105; ADR-0188 decision 6's D21-115 note, proposed.)* ~~After `exclude`, invocation 3
        // opens the drift gate~~: the refused paths are the item's managed `Spec.json` and
        // `Spec.md`, so `exclude` is now REFUSED (exit `2`, nothing answered, the gate open) and
        // the lane is released by `abort`, as the `abort` arm is. The livelock the R7FIX-8f reading
        // left (a drift gate re-opened at every entry) is what D21-115 rules out.
        if option == "exclude" {
            assert_eq!(
                lane_e2e::answer_item(root, item, "exclude"),
                Exit::Usage,
                "`exclude` over the item's managed artifacts was not refused (D21-115)"
            );
            assert_eq!(
                answered_and_void(root, &token),
                (0, 0),
                "the refused `exclude` answered the gate"
            );
        }
        assert_eq!(
            lane_e2e::answer_item(root, item, "abort"),
            Exit::Success,
            "[{option}] the refused landing's gate is not answerable by `abort`"
        );
        assert_eq!(
            answered_and_void(root, &token),
            (1, 0),
            "[{option}] the `abort` answer was not APPLIED"
        );

        let third = ScriptedFactory::new(vec![lane_e2e::review()]);
        let tip_before = lane_e2e::ref_sha(root, item);
        assert_eq!(
            run(root, item, &third),
            Exit::Success,
            "[{option}] the lane is stuck: the follow-up invocation does not exit 0"
        );
        assert_eq!(
            landing_gates(root).len(),
            1,
            "[{option}] the follow-up refused again"
        );
        assert_eq!(
            blob(root, &run_ref, &spec_json(item)),
            json_before,
            "[{option}] a refused path landed on the follow-up"
        );
        assert!(
            drift_gates(root).is_empty() && third.dispatches() == 0,
            "[{option}] the aborted lane walked on"
        );
        assert_eq!(
            lane_e2e::ref_sha(root, item),
            tip_before,
            "[{option}] the aborted lane moved the run ref"
        );
    }
}

/// **D21-115: `exclude` over a refused path under the item's managed artifact directory is refused
/// BY NAME, appends nothing, and `abort` releases the lane** (ADR-0188 decision 6's D21-115 note,
/// proposed; `s8f` §8f.4). Invocation 2's drift-gate landing refuses `Spec.json` and `Spec.md` by
/// size, both under `.redkiln/artifacts/<item>/`. `answer … exclude`, driven AS A PROCESS so its
/// stderr is read, exits `2` (the usage exit `answer` gives an option the gate cannot take),
/// naming each managed path and `abort` as the remedy; it appends no row to the item's journal
/// (beyond the process's own startup `reconcile_started`), moves no ref, and the gate stays open (no `gate_answered` for its token). `abort` is then APPLIED, and invocation 3 stops the
/// lane: exit `0`, no dispatch, no drift gate, the run ref unmoved. Red before the refusal: `exclude`
/// is APPLIED, exit `0`. Red under M-EXCL-MANAGED (the refusal dropped). *(Added 2026-09-29, s8f
/// build, package D21115-8f; D21-115, TAKEN PROVISIONALLY under D21-105.)*
#[test]
fn an_exclude_over_a_refused_managed_path_is_refused_by_name_and_abort_stops_the_lane() {
    let repo = drift_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    let human = a_human_drifts_the_spec_over_the_blob_limit(root, item);
    let run_ref = lane_e2e::run_ref(item);
    let json_before = blob(root, &human, &spec_json(item));
    assert_eq!(
        run(root, item, &ScriptedFactory::new(vec![lane_e2e::review()])),
        Exit::Success,
        "PREMISE: `run` exits 0 at the drift gate landing's refusal"
    );
    let gates = landing_gates(root);
    assert_eq!(gates.len(), 1, "PREMISE: one landing gate: {gates:?}");
    let refused: Vec<String> = refused_of(&gates[0]).into_iter().map(|(p, _)| p).collect();
    let managed = rk_store::render::managed_path(item, "");
    assert!(
        refused.contains(&spec_json(item))
            && refused.contains(&spec_md(item))
            && refused.iter().all(|p| p.starts_with(&managed)),
        "PREMISE: the gate refuses the item's managed `Spec.json` and `Spec.md`: {refused:?}"
    );
    let token = gates[0].payload["token"]
        .as_str()
        .expect("the gate's token")
        .to_owned();

    // ---- `exclude` is refused, by name, and appends nothing ------------------------------------
    // Every row but a process's own startup `reconcile_started` (item `None`, written by every
    // `redkiln` invocation before any verb runs): that one is not the answer's.
    let journal = || -> Vec<Event> {
        support::journal_rows(root, item)
            .into_iter()
            .filter(|e| !(e.kind == EventKind::ReconcileStarted && e.item().is_none()))
            .collect()
    };
    let rows_before = journal().len();
    assert!(rows_before > 0, "PREMISE: the item's journal is read");
    let tip_before = lane_e2e::ref_sha(root, item);
    let out = crate::answer::spawn_answer(root, item, "exclude", &crate::answer::BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(2),
        "`exclude` over a managed path was not refused with the usage exit: stderr {stderr}"
    );
    for path in &refused {
        assert!(
            stderr.contains(path.as_str()),
            "the refusal does not name the managed path {path}: {stderr}"
        );
    }
    assert!(
        stderr.contains("`abort`"),
        "the refusal does not name `abort` as the remedy: {stderr}"
    );
    let rows_after = journal();
    assert_eq!(
        rows_after.len(),
        rows_before,
        "the refused `exclude` appended a row to the item's journal: {:?}",
        rows_after.get(rows_before..)
    );
    assert_eq!(
        lane_e2e::ref_sha(root, item),
        tip_before,
        "the refused `exclude` moved the run ref"
    );
    assert_eq!(
        answered_and_void(root, &token),
        (0, 0),
        "the gate was answered: it does not stay open"
    );

    // ---- `abort` still works, and stops the lane ------------------------------------------------
    assert_eq!(
        lane_e2e::answer_item(root, item, "abort"),
        Exit::Success,
        "`abort` is not answerable after the refused `exclude`"
    );
    assert_eq!(
        answered_and_void(root, &token),
        (1, 0),
        "`abort` was not APPLIED"
    );
    let third = ScriptedFactory::new(vec![lane_e2e::review()]);
    let tip_after_abort = lane_e2e::ref_sha(root, item);
    assert_eq!(run(root, item, &third), Exit::Success);
    assert!(
        drift_gates(root).is_empty() && third.dispatches() == 0,
        "the aborted lane walked on"
    );
    assert_eq!(
        lane_e2e::ref_sha(root, item),
        tip_after_abort,
        "the aborted lane moved the run ref"
    );
    assert_eq!(
        blob(root, &run_ref, &spec_json(item)),
        json_before,
        "a refused managed path landed"
    );
}

/// **An entry whose writes leave the run tree unchanged makes NO `rk entry` commit** (D21-114;
/// `s8f` §8f.4: *"with `allow_empty` false: an unchanged tree makes no commit"*). Invocation 2
/// finds a `document` drift (the human deleted the rendering), restores it and lands the restore
/// through the drift gate's `Gate:` commit. `answer … move-to-prose:notes` applies at invocation 3's
/// entry: a typed write of `Spec.json` (so the entry WROTE, and the entry landing runs), whose bytes
/// the gate's commit already landed. The tree is unchanged, so no `rk entry <item> ` commit is made,
/// empty or not. The control: the move's `Spec.json` write follows the answer. Red under
/// M-ENTRY-EMPTY (the Entry arm's `allow_empty` true: an EMPTY `rk entry` commit lands).
#[test]
fn an_entry_whose_writes_leave_the_tree_unchanged_makes_no_entry_commit() {
    let repo = drift_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    run(
        root,
        item,
        &ScriptedFactory::new(vec![lane_e2e::implement()]),
    );
    let tree = crate::run_tree(root, item).path;
    assert_eq!(
        git_rc(&tree, &["rm", "--quiet", "--", &spec_md(item)]),
        Some(0),
        "PREMISE: the human deletes the rendering"
    );
    assert_eq!(
        git_rc(
            &tree,
            &[
                "commit",
                "--quiet",
                "-m",
                "a human deletes the spec's rendering"
            ]
        ),
        Some(0),
        "PREMISE: the deletion is a commit on the run ref"
    );
    assert_eq!(
        run(root, item, &ScriptedFactory::new(vec![lane_e2e::review()])),
        Exit::Success,
        "PREMISE: `run` exits 0 at the drift gate"
    );
    assert_eq!(
        drift_gates(root).len(),
        1,
        "PREMISE: the drift gate is open"
    );
    assert_eq!(
        answer(root, item, "move-to-prose:notes", None),
        Exit::Success,
        "PREMISE: the offered answer is accepted"
    );
    let writes_before = spec_json_writes(root, item).len();
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "PREMISE: invocation 3 applies the move and walks on"
    );
    assert!(
        spec_json_writes(root, item).len() > writes_before,
        "CONTROL: the applied move wrote no `Spec.json`, so the entry landing never ran"
    );
    assert_eq!(
        entry_landings(root, item),
        Vec::<(String, String)>::new(),
        "an entry whose writes left the tree unchanged made an `rk entry` commit"
    );
}

// ------------------------- the entry landing's vouched-root arm and its refusal arm (D21-114)
//
// *(Added 2026-09-29, s8f build, package R6FIX-8f; refuting round 6's tests-mutants major and its
// crossfile-safety minor; D21-114, TAKEN PROVISIONALLY under D21-105; ADR-0188 decision 2's dated
// note, proposed.)* §8f.4 names three entry writes the entry landing lands: an accepted prose
// edit, an applied `move-to-prose` and a STORED VOUCHED ROOT. The tests above drive the first two;
// these drive the third, and the landing's refusal arm (`landing.refused-path` over an empty
// commit), which a stored root over the blob limit reaches.

/// Invocation 1 opens `lane-1`'s abstention gate (a pack question whose `vouch` option attaches a
/// `Findings` root), and `answer … vouch` attaches `bytes` from the checkout. The root is stored at
/// the NEXT entry. Returns the root's repo-relative path in the run tree.
fn vouch_a_findings_root(root: &Path, item: &str, bytes: &[u8]) -> String {
    let processes = root.join(".redkiln").join("processes");
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("vouch.toml"),
        r#"[question."unknown_facet:has_repro"]
text = "Does the item reproduce? `vouch` attaches the human's findings."
options = ["approve", { id = "vouch", attaches = "Findings" }]
"#,
    )
    .expect("the question is written");
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "PREMISE: the abstention gate opens"
    );
    write(root, "notes/brief.json", bytes);
    assert_eq!(
        answer(
            root,
            item,
            "vouch",
            Some(("Findings", root.join("notes/brief.json")))
        ),
        Exit::Success,
        "PREMISE: the root is attached"
    );
    format!(".redkiln/artifacts/{item}/Findings.root.json")
}

/// Every writer `artifact.write` (not an attach row) naming `path`, in journal order.
fn root_writes(root: &Path, path: &str) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .filter(|e| e.payload.get("attached_path").is_none())
        .filter(|e| e.payload.get("path").and_then(serde_json::Value::as_str) == Some(path))
        .collect()
}

/// **D21-114: a vouched root stored at entry lands AT ENTRY, in one `rk entry <item> ` commit,
/// before any step checkpoint** (§8f.4's R5FIX2-8f note: *"a stored vouched root"*). `answer …
/// vouch` attaches a `Findings` root; invocation 2's entry stores it (one writer `artifact.write`)
/// and, with no step committing (an empty script), lands it: exactly one `rk entry lane-1 ` content
/// commit carries `.redkiln/artifacts/lane-1/Findings.root.json`, whose parent lacks it. Invocation
/// 3 re-enters, again with no committed step: the stored root is NOT discarded (still in the run
/// tree with its vouched bytes), NOT stored again (no second writer row) and NOT seen as changed
/// (no `drift_advisory`), and no second entry landing is made. Invocation 4 dispatches `implement`
/// and its step writes a file, so it commits a step checkpoint: every step checkpoint follows the
/// entry landing, and no commit before the entry landing carries the root. Red under
/// M-ENTRY-VOUCHED (the vouched arm of the entry pass's `wrote` dropped): no `rk entry` commit, the
/// root rides no commit at invocation 2, and invocation 3's re-entry discards it.
#[test]
fn a_vouched_root_stored_at_entry_lands_at_entry_before_any_step_checkpoint() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = GATE_ITEM;
    let root_rel = vouch_a_findings_root(root, item, VOUCHED_ROOT);
    let run_ref = lane_e2e::run_ref(item);
    assert!(
        root_writes(root, &root_rel).is_empty(),
        "PREMISE: nothing stored the root before the next entry"
    );
    assert!(
        entry_landings(root, item).is_empty(),
        "PREMISE: no entry landing before the root is stored"
    );

    // ---- invocation 2: the entry stores the root; no step commits ------------------------------
    run(root, item, &ScriptedFactory::new(Vec::new()));
    assert_eq!(
        root_writes(root, &root_rel).len(),
        1,
        "PREMISE: invocation 2's entry stored the root once"
    );
    let landings = entry_landings(root, item);
    assert_eq!(
        landings.len(),
        1,
        "invocation 2's stored root made no single `rk entry {item} ` commit: {:?}",
        subjects(root, item)
    );
    let (entry_sha, _) = landings[0].clone();
    assert_eq!(
        blob(root, &entry_sha, &root_rel).as_deref(),
        Some(VOUCHED_ROOT),
        "the entry landing {entry_sha} does not carry the stored root"
    );
    assert_eq!(
        blob(root, &format!("{entry_sha}^1"), &root_rel),
        None,
        "the root was on the run ref before the entry landing"
    );
    assert!(
        step_checkpoints(root, item).is_empty(),
        "PREMISE: invocation 2 committed no step"
    );

    // ---- invocation 3: re-entry with no committed step -------------------------------------------
    let run_tree = crate::run_tree(root, item).path;
    run(root, item, &ScriptedFactory::new(Vec::new()));
    assert_eq!(
        std::fs::read(run_tree.join(&root_rel)).ok().as_deref(),
        Some(VOUCHED_ROOT),
        "invocation 3's re-entry discarded the stored root"
    );
    assert_eq!(
        root_writes(root, &root_rel).len(),
        1,
        "the root was stored again at invocation 3"
    );
    assert_eq!(
        count_kind(root, EventKind::DriftAdvisory),
        0,
        "invocation 3 saw the stored root as changed"
    );
    assert_eq!(
        entry_landings(root, item).len(),
        1,
        "invocation 3 made a second entry landing"
    );

    // ---- invocation 4: a committed step (it writes a file) follows the entry landing -----------
    let fourth = writing(&run_tree, vec![lane_e2e::implement()], |n, tree| {
        if n == 0 {
            write(tree, "landed.txt", LANDED);
        }
    });
    run(root, item, &fourth);
    let all = subjects(root, item);
    let entry_at = all
        .iter()
        .position(|(sha, _)| *sha == entry_sha)
        .expect("the entry landing is on the run ref");
    let steps: Vec<usize> = all
        .iter()
        .enumerate()
        .filter(|(_, (_, s))| s.starts_with(&format!("rk step {item} ")))
        .map(|(n, _)| n)
        .collect();
    assert!(
        !steps.is_empty(),
        "PREMISE: invocation 4 committed a step: {all:?}"
    );
    assert!(
        steps.iter().all(|n| *n > entry_at),
        "a step checkpoint precedes the entry landing: {all:?}"
    );
    assert!(
        all[..entry_at]
            .iter()
            .all(|(sha, _)| blob(root, sha, &root_rel).is_none()),
        "a commit before the entry landing carries the root: {all:?}"
    );
    assert_eq!(
        blob(root, &run_ref, &root_rel).as_deref(),
        Some(VOUCHED_ROOT),
        "the stored root is not on {run_ref}"
    );
}

/// **D21-114 item 2: a refused entry landing opens the `landing.refused-path` gate over an EMPTY
/// commit, `run` exits `0` at it, and nothing lands** (§8f.4's R5FIX2-8f note; ADR-0188 decision
/// 7). The attached root is one byte over `MAX_LANDED_BLOB_BYTES`, so the entry pass stores it and
/// the entry landing refuses it by size, before any seam reaches the kernel: no step dispatches, no
/// `rk entry` or `rk step` commit is made, the run ref's tip is the gate's empty `Gate:` commit, the
/// gate's row names the root's path and `size:<bytes>`, and the root is not on the run ref. Red
/// under M-ENTRY-REFUSED (the `LandingRefused` arm of `land_entry` removed: the catch-all exits
/// `3`) and M-ENTRY-SWALLOW (the refusal swallowed: the lane walks on and `implement` dispatches).
#[test]
fn a_refused_entry_landing_opens_its_gate_over_an_empty_commit_and_lands_nothing() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = GATE_ITEM;
    let over = usize::try_from(rk_git::landing::MAX_LANDED_BLOB_BYTES).expect("fits") + 1;
    let mut big = b"{\"items\":[]}".to_vec();
    big.resize(over - 1, b' ');
    big.push(b'\n');
    assert_eq!(
        big.len(),
        over,
        "PREMISE: the root is one byte over the limit"
    );
    let root_rel = vouch_a_findings_root(root, item, &big);
    let run_tree = crate::run_tree(root, item).path;
    let dispatched = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&dispatched);
    let second = writing(&run_tree, vec![lane_e2e::implement()], move |_, _| {
        counted.fetch_add(1, Ordering::SeqCst);
    });
    let before = subjects(root, item).len();

    assert_eq!(
        run(root, item, &second),
        Exit::Success,
        "`run` does not exit 0 at the refused entry landing's gate"
    );
    assert_eq!(
        root_writes(root, &root_rel).len(),
        1,
        "PREMISE: the entry pass stored the root"
    );
    assert_eq!(
        dispatched.load(Ordering::SeqCst),
        0,
        "a step dispatched past the refused entry landing"
    );
    let gates = landing_gates(root);
    assert_eq!(
        gates.len(),
        1,
        "the refused entry landing opened no single `{LANDING_KIND}` gate: {gates:?}"
    );
    assert_eq!(
        refused_of(&gates[0]),
        vec![(root_rel.clone(), format!("size:{over}"))],
        "the gate does not name the refused root and its rule"
    );
    assert!(
        entry_landings(root, item).is_empty() && step_checkpoints(root, item).is_empty(),
        "a content commit was made past a refused entry landing: {:?}",
        subjects(root, item)
    );
    let tip = lane_e2e::ref_sha(root, item).expect("the run ref exists");
    assert!(
        subjects(root, item).len() > before,
        "PREMISE: the gate's commit is on the run ref"
    );
    assert!(
        gate_commits(root, item).iter().any(|(sha, _)| *sha == tip),
        "the run ref's tip is not the gate's `Gate:` commit"
    );
    assert!(
        is_empty_commit(root, &tip),
        "the refused entry landing's gate commit is not empty"
    );
    assert_eq!(
        blob(root, &lane_e2e::run_ref(item), &root_rel),
        None,
        "the refused root landed"
    );
}

// ------------------------------------------ an `exclude` over a TRACKED path (D21-112)
//
// *(Added 2026-09-29, s8f build, package R3FIX-8f; refuting round 3's spec-conformance major;
// D21-112, TAKEN PROVISIONALLY under D21-105; ADR-0188 decisions 6 and 7, proposed.)* A tracked
// path the denylist refuses (`id_map.rs`, the false hit `rk_git::landing::DENYLIST`'s own doc names)
// is answered `exclude`. Every later landing leaves it unstaged, so the owned tree keeps its
// modification after the commit, and a gate a landed act opens binds the landing's T, which lacks
// it. `answer` must derive the working state LESS the lane's excluded paths, read as `run` reads
// them, or its tree is not the gate's and the answer is void (AC-176).

/// The tracked file the denylist refuses by `id_*`: committed at the fork, so the run tree tracks it.
const ID_MAP: &str = "id_map.rs";

/// Commit [`ID_MAP`] in the checkout, so the run tree forked from it tracks the file.
fn track_id_map(root: &Path) {
    write(root, ID_MAP, b"pub const MAP: &[u8] = b\"\";\n");
    assert_eq!(git_rc(root, &["add", "--", ID_MAP]), Some(0));
    assert_eq!(
        git_rc(root, &["commit", "--quiet", "-m", "track id_map.rs"]),
        Some(0),
        "PREMISE: `{ID_MAP}` is committed at the fork"
    );
}

/// The `gate_answered` rows and the `gate_answer_void` rows for `token` (a void answer writes both).
fn answered_and_void(root: &Path, token: &str) -> (usize, usize) {
    let rows = support::store_rows(root);
    (
        rows.iter()
            .filter(|e| e.kind == EventKind::GateAnswered)
            .filter(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token))
            .count(),
        rows.iter()
            .filter(|e| e.kind == EventKind::GateAnswerVoid)
            .filter(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token))
            .count(),
    )
}

/// Invocation 1's `implement` modifies the tracked [`ID_MAP`]; the landing refuses it by `id_*`,
/// and `answer … exclude` applies. PREMISES only.
fn refuse_and_exclude_id_map(root: &Path, item: &str, tree: &Path) {
    let first = writing(tree, vec![lane_e2e::implement()], |_, t| {
        append(t, ID_MAP, EDIT);
    });
    assert_eq!(
        run(root, item, &first),
        Exit::Success,
        "PREMISE: exit 0 at the landing gate"
    );
    let gates = landing_gates(root);
    assert_eq!(gates.len(), 1, "PREMISE: one landing gate: {gates:?}");
    assert_eq!(
        refused_of(&gates[0]),
        vec![(ID_MAP.to_owned(), "denylist:id_*".to_owned())],
        "PREMISE: the TRACKED `{ID_MAP}` is refused by `id_*`"
    );
    let token = gates[0].payload["token"]
        .as_str()
        .expect("the gate's token")
        .to_owned();
    assert_eq!(lane_e2e::answer_item(root, item, "exclude"), Exit::Success);
    assert_eq!(
        answered_and_void(root, &token),
        (1, 0),
        "PREMISE: `exclude` was APPLIED to the landing gate"
    );
}

/// **After an `exclude` of a TRACKED path, the next gate's answer is APPLIED, not void** (D21-112
/// ruling 2; ADR-0188 decisions 6-7). `implement`'s activity is given a catalog `gate` step AFTER
/// its agent step. Invocation 1's agent step modifies the tracked `id_map.rs`: refused, answered
/// `exclude`. Invocation 2's agent step modifies it again and edits `README`: the step's landing
/// lands `README` and leaves `id_map.rs` unstaged (still modified in the run tree), and the gate
/// step then opens its gate over the landed T. `answer … accept` is APPLIED. Red while `answer`
/// derives the plain working state (the excluded modification in it), and under M-EXCL (the
/// subtraction dropped).
#[test]
fn a_tracked_exclusion_leaves_the_next_gates_answer_applied() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    track_id_map(root);
    let catalog = root
        .join(".redkiln")
        .join("catalog")
        .join("emit-report.toml");
    let src = std::fs::read_to_string(&catalog).expect("emit-report.toml reads");
    assert_eq!(
        src.matches("[[step]]").count(),
        1,
        "PREMISE: emit-report.toml has its one agent step"
    );
    std::fs::write(
        &catalog,
        format!(
            "{src}\n[[step]]\nid       = \"probe\"\nkind     = \"gate\"\nquestion = \"probe.accept\"\n"
        ),
    )
    .expect("the gate step is written");
    let processes = rk_cli::processes::processes_dir(root);
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("questions.toml"),
        "[question.\"probe.accept\"]\ntext = \"Accept the probe?\"\noptions = [\"accept\", \"reject\"]\n",
    )
    .expect("the question is declared");
    // *(dated 2026-10-02, s8h build, package P7; `s8h` §8h.10.)* The catalog now reaches
    // `probe.accept`, so the packs carry its fail-closed row (ADR-0146's load-time lint).
    support::add_gate_expiry_row(root, "probe.accept");
    let tree = crate::run_tree(root, item).path;
    let fork = rev(root, "HEAD").expect("the checkout's HEAD");

    refuse_and_exclude_id_map(root, item, &tree);

    let second = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        append(t, ID_MAP, EDIT);
        append(t, "README", EDIT);
    });
    assert_eq!(
        run(root, item, &second),
        Exit::Success,
        "exit 0 at the probe gate"
    );
    let probes: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| {
            e.payload.get("kind").and_then(serde_json::Value::as_str) == Some("probe.accept")
        })
        .collect();
    assert_eq!(
        probes.len(),
        1,
        "PREMISE: the probe gate opened: {probes:?}"
    );
    let token = probes[0].payload["token"]
        .as_str()
        .expect("the probe gate's token")
        .to_owned();
    let run_ref = lane_e2e::run_ref(item);
    assert!(
        blob(root, &run_ref, "README").is_some_and(|b| b.ends_with(EDIT)),
        "PREMISE: the step's landing landed the README edit"
    );
    assert_eq!(
        blob(root, &run_ref, ID_MAP),
        blob(root, &fork, ID_MAP),
        "PREMISE: the excluded `{ID_MAP}` did not land"
    );
    assert!(
        git_read(&tree, &["status", "--porcelain", "--", ID_MAP])
            .is_some_and(|s| s.contains(ID_MAP)),
        "PREMISE: the excluded modification is still in the run tree"
    );
    assert_eq!(
        landing_gates(root).len(),
        1,
        "PREMISE: the exclusion held, so no second landing gate opened"
    );

    assert_eq!(lane_e2e::answer_item(root, item, "accept"), Exit::Success);
    assert_eq!(
        answered_and_void(root, &token),
        (1, 0),
        "the probe gate's answer was VOID: `answer` derived a tree with the excluded tracked \
         modification in it, which the gate's T lacks (D21-112)"
    );
}

/// **A refused landing under a TRACKED exclusion is answerable** (D21-112; ADR-0188 decision 7).
/// `id_map.rs` is refused and excluded; invocation 2's step modifies it again and writes `.env`, so
/// its landing refuses `.env` and opens a second landing gate over the no-checkpoint-due derivation.
/// That derivation is LESS the lane's excluded paths, as `answer`'s is, so `exclude` is APPLIED.
/// Red under M-EXCL-R (the refusal's tree taken as the plain working state while `answer`
/// subtracts). *(Added 2026-09-29, s8f build, package R3FIX-8f.)*
#[test]
fn a_refused_landing_under_a_tracked_exclusion_is_answerable() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    track_id_map(root);
    let tree = crate::run_tree(root, item).path;

    refuse_and_exclude_id_map(root, item, &tree);

    let second = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        append(t, ID_MAP, EDIT);
        write(t, ".env", b"API_TOKEN=not-a-real-secret\n");
    });
    assert_eq!(
        run(root, item, &second),
        Exit::Success,
        "exit 0 at the second landing gate"
    );
    let gates = landing_gates(root);
    assert_eq!(
        gates.len(),
        2,
        "PREMISE: a second landing gate opened: {gates:?}"
    );
    assert_eq!(
        refused_of(&gates[1]),
        vec![(".env".to_owned(), "denylist:.env".to_owned())],
        "PREMISE: only `.env` is refused; `{ID_MAP}` is excluded, not refused"
    );
    assert!(
        git_read(&tree, &["status", "--porcelain", "--", ID_MAP])
            .is_some_and(|s| s.contains(ID_MAP)),
        "PREMISE: the excluded modification is in the run tree"
    );
    let token = gates[1].payload["token"]
        .as_str()
        .expect("the second gate's token")
        .to_owned();
    assert_eq!(lane_e2e::answer_item(root, item, "exclude"), Exit::Success);
    assert_eq!(
        answered_and_void(root, &token),
        (1, 0),
        "the second landing gate's `exclude` was VOID under a tracked exclusion (D21-112)"
    );
}

/// **A drift gate answered `move-to-prose` under a TRACKED exclusion is applied** (D21-112 ruling
/// 2; ADR-0189 decision 5). `id_map.rs` is refused and excluded; invocation 2 lands the spec; the
/// human's edit of a structured section is committed; invocation 3's entry opens the drift gate
/// over the landed T, and `review`'s step then modifies `id_map.rs`, which its landing leaves
/// unstaged. `answer … move-to-prose:notes` is APPLIED, and invocation 4 moves the drifted bytes
/// into `notes` and walks to `closed`. Red while `answer` derives the plain working state: the
/// answer is void and the move is dropped without a word.
#[test]
fn a_drift_gate_answered_move_to_prose_under_a_tracked_exclusion_is_applied() {
    let repo = drift_lane();
    let root = repo.path();
    let item = THROUGH_ITEM;
    track_id_map(root);
    let tree = crate::run_tree(root, item).path;

    refuse_and_exclude_id_map(root, item, &tree);

    let second = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        append(t, ID_MAP, EDIT);
    });
    run(root, item, &second);
    let run_ref = lane_e2e::run_ref(item);
    assert!(
        blob(root, &run_ref, &spec_md(item)).is_some(),
        "PREMISE: invocation 2 landed the spec's rendering"
    );
    human_edits_the_spec(root, item);

    let third = writing(&tree, vec![lane_e2e::review()], |_, t| {
        append(t, ID_MAP, EDIT);
    });
    assert_eq!(
        run(root, item, &third),
        Exit::Success,
        "exit 0 at the drift gate"
    );
    let drifts = drift_gates(root);
    assert_eq!(
        drifts.len(),
        1,
        "PREMISE: the drift gate is open: {drifts:?}"
    );
    let token = drifts[0].payload["token"]
        .as_str()
        .expect("the drift gate's token")
        .to_owned();
    assert!(
        git_read(&tree, &["status", "--porcelain", "--", ID_MAP])
            .is_some_and(|s| s.contains(ID_MAP)),
        "PREMISE: `review`'s modification of the excluded path is in the run tree"
    );
    assert_eq!(
        landing_gates(root).len(),
        1,
        "PREMISE: the exclusion held, so no second landing gate opened"
    );

    assert_eq!(
        answer(root, item, "move-to-prose:notes", None),
        Exit::Success
    );
    assert_eq!(
        answered_and_void(root, &token),
        (1, 0),
        "the drift gate's `move-to-prose` was VOID under a tracked exclusion (D21-112)"
    );
    assert_eq!(
        run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    let sidecar = format!(".redkiln/artifacts/{item}/Spec.json");
    let after = blob(root, &run_ref, &sidecar).expect("the sidecar is on the run ref");
    let notes = serde_json::from_slice::<serde_json::Value>(&after).expect("json")["notes"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        notes.contains("a human edited this"),
        "the drifted bytes are in the `notes` prose section: {notes:?}"
    );
    let stages = lane_e2e::story_stages(root, item);
    assert_eq!(
        stages.last().map(String::as_str),
        Some("closed"),
        "the lane walks to its end: {stages:?}"
    );
}

// ------------------------------------------ the item write and the backlog feed (§8f.7)
//
// *(Added 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-46 R7; D21-47 cross #6;
// D21-106; the render record ADR-0189 decisions 11 and 12, proposed.)* The render lane gains a last
// stage, `closeout`, holding a trimmed `debt-logging` or `closure-record` (the shipped activities'
// steps, with `pack-min`'s one role). The temp registry's `[verb]` rows and `[feed.*.backlog]`
// tables are copied VERBATIM from the shipped `.redkiln/registry.toml`, so the shipped rows are what
// the lane runs on: M34 (the shipped `[feed.rust.backlog]` left `unprovided`) and M46 (the shipped
// `item_create` row left refusing) reach these tests.

/// The workspace root, where the shipped `.redkiln/` lives.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// A harness that records every spec it is handed, then answers from its script.
struct Capturing {
    inner: FakeHarness,
    specs: Arc<Mutex<Vec<ActivitySpec>>>,
}

impl Harness for Capturing {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        self.specs.lock().expect("not poisoned").push(spec.clone());
        self.inner.dispatch(spec, store)
    }
}

/// The shipped registry's line for `[verb]` row `verb`, verbatim.
fn shipped_verb_row(verb: &str) -> String {
    let src = std::fs::read_to_string(workspace_root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry reads");
    let rows: Vec<&str> = src
        .lines()
        .filter(|l| l.split('=').next().map(str::trim) == Some(verb))
        .collect();
    assert_eq!(
        rows.len(),
        1,
        "PREMISE: the shipped registry has one `{verb}` row: {rows:?}"
    );
    rows[0].to_owned()
}

/// The shipped registry's `[feed.<toolchain>.backlog]` table, header and body, verbatim.
fn shipped_feed_table(toolchain: &str) -> String {
    let src = std::fs::read_to_string(workspace_root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry reads");
    let header = format!("[feed.{toolchain}.backlog]");
    let mut lines = src.lines().skip_while(|l| l.trim() != header);
    let head = lines
        .next()
        .expect("PREMISE: the shipped registry declares the backlog feed");
    let body: Vec<&str> = lines
        .take_while(|l| !l.trim().is_empty() && !l.starts_with('['))
        .filter(|l| !l.starts_with('#'))
        .collect();
    assert_eq!(body.len(), 1, "PREMISE: one key under {header}: {body:?}");
    format!("{head}\n{}\n", body[0])
}

/// `debt-logging`, the shipped activity's three steps with `pack-min`'s one role.
const DEBT_LOGGING: &str = r#"[activity]
id           = "debt-logging"
version      = 1
stage        = "closeout"
provides     = ["DebtRegisterEntry"]
requires     = ["ImplReport"]
derived_from = ["ImplReport"]
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 2, tokens = 50000, wall_clock_secs = 600 }

[[step]]
id      = "extract"
kind    = "agent"
role    = "implementer"
inputs  = { report = "artifact:ImplReport" }
outputs = "value:DebtRegisterEntry"

[[step]]
id      = "file"
kind    = "tool"
run     = "toolchain:item_create"
inputs  = { items = "${extract.outputs.items}" }
outputs = "value:ItemRefs"

[[step]]
id        = "emit"
kind      = "emit"
artifacts = { DebtRegisterEntry = "${extract.outputs}" }
evidence  = ["${file}"]
"#;

/// `closure-record`, the shipped activity's `audit` (reading the backlog feed as a MANDATORY input),
/// `clean` and `emit`, with `pack-min`'s one role.
const CLOSURE_RECORD: &str = r#"[activity]
id           = "closure-record"
version      = 1
stage        = "closeout"
provides     = ["ClosureRecord"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 3, tokens = 100000, wall_clock_secs = 1200 }

[[step]]
id      = "audit"
kind    = "agent"
role    = "implementer"
inputs  = { backlog = "toolchain:backlog" }
outputs = "value:SelfCheck"

[[step]]
id      = "clean"
kind    = "check"
inputs  = { audit = "${audit.outputs}" }
expr    = "audit.failed == 0"

[[step]]
id        = "emit"
kind      = "emit"
artifacts = { ClosureRecord = "${audit.outputs}" }
"#;

/// The render lane with a last stage `closeout` whose baseline is `kind`, holding `activity`
/// (`id`, TOML), and a registry of the shipped `item_create` and `backlog` rows and backlog feeds.
fn closeout_lane(kind: &str, id: &str, activity: &str) -> tempfile::TempDir {
    let repo = render_lane();
    let root = repo.path();
    let rk = root.join(".redkiln");
    std::fs::write(rk.join("catalog").join(format!("{id}.toml")), activity)
        .expect("the closeout activity is written");
    for pack in ["bug-standard.toml", "story.toml"] {
        let path = rk.join("packs").join(pack);
        patch(
            &path,
            r#"stage_order = ["triage", "discovery", "implement", "review"]"#,
            r#"stage_order = ["triage", "discovery", "implement", "review", "closeout"]"#,
        );
        patch(
            &path,
            r#"review = "mandatory" }"#,
            r#"review = "mandatory", closeout = "recommended" }"#,
        );
        let mut src = std::fs::read_to_string(&path).expect("the pack reads");
        src.push_str(&format!(
            "\n[stage.closeout]\nbaseline = [\"{kind}\"]\n\n[activity.{id}]\ntimeout_secs = 900\n"
        ));
        std::fs::write(&path, src).expect("the pack writes");
    }
    patch(
        &rk.join("facets.toml"),
        r#"values = ["triage", "implement", "review"]"#,
        r#"values = ["triage", "implement", "review", "closeout"]"#,
    );
    let mut artifacts = std::fs::read_to_string(rk.join("artifacts.toml")).expect("artifacts");
    for k in ["DebtRegisterEntry", "ClosureRecord"] {
        artifacts.push_str(&format!(
            "\n[artifact.{k}]\nsource = \"activity\"\nschema = \"{k}\"\n"
        ));
    }
    std::fs::write(rk.join("artifacts.toml"), artifacts).expect("artifacts writes");
    // *(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments; `s8h` §8h.4.)* The
    // three `.redkiln/schemas/<kind>.json` files this wrote (`DebtRegisterEntry`, `ClosureRecord`,
    // `SelfCheck`) are no longer written: the binder reads each step's schema from
    // `rk_types::registry::EXPORTED` by name, and no `.redkiln/schemas/` exists to write into.
    let pin = "0123456789abcdef0123456789abcdef01234567";
    std::fs::write(
        rk.join("registry.toml"),
        format!(
            "[verb]\n{}\n{}\n\n[toolchain.rust]\nbuild_timeout = {{ secs = 60, measured = [30], \
             rule = \"r\", pin = \"{pin}\", command = \"c\" }}\n\n{}\n{}",
            shipped_verb_row("item_create"),
            shipped_verb_row("backlog"),
            shipped_feed_table("rust"),
            shipped_feed_table("node"),
        ),
    )
    .expect("the registry writes");
    repo
}

/// The environment a lane with a detected Rust toolchain runs under: `clean_env` plus the
/// process-start keys the toolchain adapters refuse to run without (as `lane_e2e`'s own).
fn toolchain_env() -> rk_cli::env::Env {
    let mut map = std::collections::BTreeMap::new();
    for key in [
        "PATH",
        "Path",
        "PATHEXT",
        "SystemRoot",
        "SystemDrive",
        "TMP",
        "TEMP",
    ] {
        if let Some(v) = std::env::var_os(key) {
            map.insert(key.to_owned(), v.to_string_lossy().into_owned());
        }
    }
    rk_cli::env::Env::from_map(map)
}

/// The `activity_committed` rows of `activity`'s step `step`.
fn step_committed(root: &Path, activity: &str, step: &str) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .filter(|e| {
            e.payload["activity_id_at_version"].as_str() == Some(activity)
                && e.payload["step_id"].as_str() == Some(step)
        })
        .collect()
}

/// **D21-47 cross #6: `debt-logging` walks past `file` through `item_create`'s `store`-route
/// handler** (`s8f` §8f.7). A `FakeHarness` `extract` outputs two items; the activity walks past
/// `file` to its `emit`; the `file` step commits the `ItemRefs` value naming both ids; and the run
/// ref holds the two item files, each carrying its `[provenance]` (the activity at its version and
/// the run). M46 leaves the row refusing.
#[test]
fn debt_logging_walks_past_file_through_item_create() {
    let repo = closeout_lane("DebtRegisterEntry", "debt-logging", DEBT_LOGGING);
    let root = repo.path();
    let item = THROUGH_ITEM;
    let extract = Ok(Typed::new(
        rk_types::typed::SchemaId::new("DebtRegisterEntry"),
        serde_json::json!({ "items": [{ "title": "a shortcut" }, { "title": "a second" }] }),
    ));
    let lane = ScriptedFactory::new(vec![lane_e2e::implement(), lane_e2e::review(), extract]);
    assert_eq!(run(root, item, &lane), Exit::Success);
    assert_eq!(
        lane.dispatches(),
        3,
        "PREMISE: implement, review and extract each dispatched once"
    );

    let files = step_committed(root, "debt-logging@1", "file");
    let ids: Vec<String> = [1, 2].iter().map(|n| format!("{item}-debt-{n}")).collect();
    assert_eq!(
        files.len(),
        1,
        "`file` did not commit: `debt-logging` stopped at `item_create` (rows: {files:?})"
    );
    assert_eq!(
        (
            files[0].payload["schema"].as_str(),
            files[0].payload["result"].clone()
        ),
        (Some("ItemRefs"), serde_json::json!({ "ids": ids })),
        "the committed `ItemRefs` names both minted ids"
    );
    let completion: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| {
            e.kind == EventKind::ActivityCommitted
                && e.payload["activity_id_at_version"].as_str() == Some("debt-logging@1")
                && e.payload["completion"] == serde_json::json!(true)
        })
        .collect();
    assert_eq!(
        completion.len(),
        1,
        "the activity walked past `file` to its `emit`"
    );
    let run_ref = lane_e2e::run_ref(item);
    for id in &ids {
        let path = format!(".redkiln/items/{id}.toml");
        let bytes = blob(root, &run_ref, &path)
            .unwrap_or_else(|| panic!("`{path}` is not on {run_ref}: the item write did not land"));
        let text = String::from_utf8(bytes).expect("UTF-8");
        assert!(
            text.contains(&format!(
                "id = \"{id}\"\nkind = \"tech-debt\"\nstage = \"triage\"\n"
            )) && text.contains("activity = \"debt-logging@1\"")
                && text.contains("run = \"closeout-"),
            "`{path}` is not an item record with its provenance:\n{text}"
        );
    }
    assert_eq!(
        lane_e2e::story_stages(root, item)
            .last()
            .map(String::as_str),
        Some("closed"),
        "the lane walks to its end past `debt-logging`"
    );
}

/// **D21-46 R7: `closure-record` walks past `audit` on the `backlog` feed** (`s8f` §8f.7). `audit`
/// reads `toolchain:backlog` as a MANDATORY input; the feed resolves through the shipped `backlog`
/// verb's `store` route (the run's store-route seam), and the snapshot the audit is handed names the
/// item itself, a tier-1 boundary's `item`. The activity completes and the lane closes. M34 leaves
/// the shipped `[feed.rust.backlog]` `unprovided`.
#[test]
fn closure_record_walks_past_audit_on_the_backlog_feed() {
    let repo = closeout_lane("ClosureRecord", "closure-record", CLOSURE_RECORD);
    let root = repo.path();
    support::install_review_toolchain(root);
    let item = THROUGH_ITEM;
    let audit = Ok(Typed::new(
        rk_types::typed::SchemaId::new("SelfCheck"),
        serde_json::json!({ "passed": 1, "failed": 0, "notes": "all hold" }),
    ));
    let specs = Arc::new(Mutex::new(Vec::new()));
    let lane = ScriptedFactory::over(Box::new(Capturing {
        inner: FakeHarness::scripted(vec![lane_e2e::implement(), lane_e2e::review(), audit]),
        specs: Arc::clone(&specs),
    }));
    let exit = lane_e2e::ran(
        rk_cli::verbs::run::run(
            &lane_e2e::cli(root, item),
            &lane_e2e::args(item),
            &toolchain_env(),
            lane_e2e::seam(&lane),
        ),
        item,
    );
    assert_eq!(exit, Exit::Success, "the lane refused");
    assert_eq!(
        lane.dispatches(),
        3,
        "`audit` was not dispatched: `closure-record` stopped on its `toolchain:backlog` input"
    );
    let completion: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| {
            e.kind == EventKind::ActivityCommitted
                && e.payload["activity_id_at_version"].as_str() == Some("closure-record@1")
                && e.payload["completion"] == serde_json::json!(true)
        })
        .collect();
    assert_eq!(
        completion.len(),
        1,
        "`closure-record` walked past `audit` to its `emit`"
    );
    // What `audit` was handed: the snapshot of the ids the STORE names. The item itself is named by
    // its tier-1 boundaries; its record lives in the checkout, not the run-owned tree, so it is
    // listed ABSENT there, never dropped.
    let specs = specs.lock().expect("not poisoned").clone();
    let audit_spec = specs.last().expect("PREMISE: three dispatches");
    let backlog = audit_spec
        .inputs
        .iter()
        .find(|(k, _)| k == "backlog")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| panic!("`audit` was handed no `backlog`: {:?}", audit_spec.inputs));
    assert_eq!(
        backlog["absent"],
        serde_json::json!([{ "id": item, "named_by": ["stage_transition"] }]),
        "the backlog is not the snapshot of the ids the store names: {backlog}"
    );
    assert_eq!(backlog["items"], serde_json::json!([]), "{backlog}");
    assert_eq!(
        lane_e2e::story_stages(root, item)
            .last()
            .map(String::as_str),
        Some("closed"),
        "the lane walks to its end past `closure-record`"
    );
}

// ------------------------------------------ `answer`'s expiry pass does not land (§8f.4)
//
// *(Added 2026-09-29, s8f build, package R4FIX-8f; refuting round 4's blocker; ADR-0188 decision
// 14, proposed.)* §8f.4: *"`answer` does not land."* A gate whose class declares `escalate` and
// whose deadline has passed is expired by `answer`'s expiry pass, which writes a `gate_expired`
// row and an escalation's `gate_opened` row. Both ride EMPTY commits: the escalation binds the
// no-checkpoint-due derivation `answer` took, so the run tree's in-flight dirt is not committed and
// a later answer to the escalation derives the same tree.

/// What the tests below leave uncommitted in the run tree before `answer` runs.
const DIRT: &[u8] = b"IN-FLIGHT DIRT, NEVER COMMITTED BY A STEP\n";

/// Give `emit-report` a catalog `gate` step after its agent step, asking `probe.accept`.
fn add_probe_gate(root: &Path) {
    let catalog = root
        .join(".redkiln")
        .join("catalog")
        .join("emit-report.toml");
    let src = std::fs::read_to_string(&catalog).expect("emit-report.toml reads");
    assert_eq!(
        src.matches("[[step]]").count(),
        1,
        "PREMISE: emit-report.toml has its one agent step"
    );
    std::fs::write(
        &catalog,
        format!(
            "{src}\n[[step]]\nid       = \"probe\"\nkind     = \"gate\"\nquestion = \"probe.accept\"\n"
        ),
    )
    .expect("the gate step is written");
    let processes = rk_cli::processes::processes_dir(root);
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("questions.toml"),
        "[question.\"probe.accept\"]\ntext = \"Accept the probe?\"\noptions = [\"accept\", \"reject\"]\n",
    )
    .expect("the question is declared");
    // *(dated 2026-10-02, s8h build, package P7; `s8h` §8h.10.)* The catalog now reaches
    // `probe.accept`, so the packs carry its fail-closed row (ADR-0146's load-time lint).
    support::add_gate_expiry_row(root, "probe.accept");
}

/// Declare `escalate` as the expiry action of gate class `class`, in both installed pack files.
///
/// *(dated 2026-10-02, s8h build, package P7-cli-binding-data-and-amendments; `s8h` §8h.10.)* Every
/// reachable class now carries a `fail` row (ADR-0146's load-time lint refuses one without), so the
/// class's ONE `fail` row is re-declared `escalate`, where this appended a first row before. It
/// read: assert no row for `class`, then append `class` / `action = "escalate"`.
fn escalate_on_expiry(root: &Path, class: &str) {
    for name in ["bug-standard.toml", "story.toml"] {
        let path = root.join(".redkiln").join("packs").join(name);
        let src = std::fs::read_to_string(&path).expect("the pack reads");
        let row = format!("class = \"{class}\"\naction = \"fail\"\n");
        assert_eq!(
            src.matches(&row).count(),
            1,
            "PREMISE: {name} declares exactly one `fail` expiry row for `{class}`"
        );
        std::fs::write(
            &path,
            src.replace(
                &row,
                &format!("class = \"{class}\"\naction = \"escalate\"\n"),
            ),
        )
        .expect("the expiry row is written");
    }
}

/// `answer <item> <option>` in process, exactly as the verb's outer half builds it, with the one
/// wall-clock read replaced by `now` (so a deadline is passed without a sleep).
fn answer_at(root: &Path, item: &str, option: &str, now: i64) -> Exit {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let run_args = rk_cli::cli::RunArgs {
        item: item.to_owned(),
        lane: None,
    };
    let resolved = rk_cli::verbs::run::through_step_seven(
        &cli,
        &run_args,
        &support::clean_env(),
        rk_cli::verbs::Startup::real(),
        rk_git::reconciled::Entry::Answer,
    )
    .unwrap_or_else(|e| panic!("`answer {item}`'s steps (1)-(7) resolve: {e:?}"));
    let opened = rk_adapter_sqlite::SqliteStore::open(&resolved.exec, &resolved.worktree)
        .unwrap_or_else(|e| panic!("the store opens: {e:?}"));
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));
    let at = Path::new(resolved.worktree.as_str());
    let registry = rk_cli::processes::load(at)
        .and_then(|loaded| rk_cli::processes::with_kernel_questions(at, loaded))
        .unwrap_or_else(|e| panic!("the question registry loads: {e:?}"));
    let args = rk_cli::cli::AnswerArgs {
        item: item.to_owned(),
        option: option.to_owned(),
        attach: Vec::new(),
        explain_sufficed: rk_cli::cli::YesNo::Yes,
        toml_opened: rk_cli::cli::YesNo::No,
    };
    rk_cli::verbs::answer::run_with(
        rk_cli::verbs::answer::Channel::Cli,
        &registry,
        &resolved,
        &args,
        &store,
        now,
    )
    .unwrap_or_else(|halt| panic!("`answer {item}` halted: {halt:?}"))
}

/// `answer`'s derivation over the run tree with no path excluded: the working state LESS nothing.
fn answers_derivation(root: &Path, tree: &Path) -> String {
    let discovered = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let owned = rk_git::path::WorktreePath::new(tree).expect("the run tree's path");
    rk_git::tree::tree_hash(
        &discovered.exec,
        &owned,
        &discovered.common_dir,
        &rk_git::tree::TreeBasis::WorkingStateLess(std::collections::BTreeSet::new()),
    )
    .expect("the no-checkpoint-due derivation")
    .as_str()
    .to_owned()
}

/// The escalation gates the journal opened (`gate_opened` rows of class `escalation`).
fn escalations(root: &Path) -> Vec<Event> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some("escalation"))
        .collect()
}

/// The `gate_expired` rows for `token` whose action is `escalate`.
fn escalated(root: &Path, token: &str) -> usize {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateExpired)
        .filter(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token))
        .filter(|e| e.payload.get("action").and_then(serde_json::Value::as_str) == Some("escalate"))
        .count()
}

/// The token and deadline of a runtime-shaped `gate_opened` row.
fn token_and_deadline(row: &Event) -> (String, i64) {
    (
        row.payload["token"]
            .as_str()
            .expect("the gate's token")
            .to_owned(),
        row.payload["deadline"]
            .as_i64()
            .expect("the gate's deadline"),
    )
}

/// **A gate that expires into an escalation during `answer` lands nothing** (§8f.4 *"`answer` does
/// not land"*; ADR-0188 decision 14). `implement` lands a README edit and its catalog gate step
/// opens `probe.accept`, whose class declares `escalate`. Dirt is then left uncommitted in the run
/// tree, and `answer` runs past the gate's deadline: the gate expires, the escalation opens, and
/// the run ref's TREE does not move, the dirt stays uncommitted, and the escalation binds `answer`'s
/// own derivation. A later `answer … approve` to the escalation is APPLIED. Red while the
/// escalation's `gate_opened` rides the landed arm (the dirt is committed onto `rk-run/<item>`),
/// and under M-ESC-K / M-ESC-C.
#[test]
fn a_gate_escalating_during_answer_lands_nothing() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    add_probe_gate(root);
    escalate_on_expiry(root, "probe.accept");
    let tree = crate::run_tree(root, item).path;

    let first = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        append(t, "README", EDIT);
    });
    assert_eq!(
        run(root, item, &first),
        Exit::Success,
        "PREMISE: exit 0 at the probe gate"
    );
    let probes: Vec<Event> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| {
            e.payload.get("kind").and_then(serde_json::Value::as_str) == Some("probe.accept")
        })
        .collect();
    assert_eq!(
        probes.len(),
        1,
        "PREMISE: the probe gate opened: {probes:?}"
    );
    let (token, deadline) = token_and_deadline(&probes[0]);
    let run_ref = lane_e2e::run_ref(item);
    let tip = lane_e2e::ref_sha(root, item).expect("the run ref");
    let landed = tree_of(root, &tip);
    assert!(
        blob(root, &run_ref, "README").is_some_and(|b| b.ends_with(EDIT)),
        "PREMISE: the step's landing landed the README edit"
    );

    append(&tree, "README", DIRT);
    let derived = answers_derivation(root, &tree);
    assert_ne!(
        derived, landed,
        "PREMISE: the dirt moves `answer`'s derivation off the landed tree"
    );

    let exit = answer_at(root, item, "accept", deadline + 1);
    assert_ne!(
        exit,
        Exit::ConfigError,
        "`answer`'s expiry pass was refused at the escalation"
    );
    assert_eq!(
        escalated(root, &token),
        1,
        "PREMISE: the probe gate expired into an escalation"
    );
    let opened = escalations(root);
    assert_eq!(
        opened.len(),
        1,
        "PREMISE: one escalation opened: {opened:?}"
    );

    let tip_after = lane_e2e::ref_sha(root, item).expect("the run ref");
    assert_eq!(
        tree_of(root, &tip_after),
        landed,
        "`answer` LANDED the run tree on `{run_ref}`: its tree moved from the probe gate's"
    );
    assert!(
        blob(root, &run_ref, "README").is_some_and(|b| !b.ends_with(DIRT)),
        "`answer` committed the run tree's in-flight dirt onto `{run_ref}`"
    );
    assert!(
        git_read(&tree, &["status", "--porcelain", "--", "README"])
            .is_some_and(|s| s.contains("README")),
        "the dirt is no longer uncommitted in the run tree"
    );
    let (escalation, _) = token_and_deadline(&opened[0]);
    assert_eq!(
        opened[0].payload["tree_hash"].as_str(),
        Some(derived.as_str()),
        "the escalation does not bind `answer`'s own derivation"
    );

    assert_eq!(lane_e2e::answer_item(root, item, "approve"), Exit::Success);
    assert_eq!(
        answered_and_void(root, &escalation),
        (1, 0),
        "a later answer to the escalation was not APPLIED"
    );
}

/// **A refused landing's own gate escalating during `answer` is not refused** (§8f.4; ADR-0188
/// decisions 7 and 14). `implement` writes `.env` and edits README; the landing refuses `.env` and
/// opens its gate, whose class declares `escalate`. `answer` past the deadline expires it into an
/// escalation that rides an EMPTY commit, so the refused paths are not restaged: no
/// `LandingRefused`, no exit 3, nothing lands, and the escalation binds `answer`'s derivation. Red
/// while the escalation rides the landed arm (the restage refuses `.env` again and the escalation's
/// row is never written), and under M-ESC-K / M-ESC-C.
#[test]
fn a_refused_landings_gate_escalating_during_answer_is_not_refused() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let item = THROUGH_ITEM;
    escalate_on_expiry(root, LANDING_KIND);
    let tree = crate::run_tree(root, item).path;

    let first = writing(&tree, vec![lane_e2e::implement()], |_, t| {
        write(t, ".env", b"API_TOKEN=not-a-real-secret\n");
        append(t, "README", EDIT);
    });
    assert_eq!(
        run(root, item, &first),
        Exit::Success,
        "PREMISE: exit 0 at the landing gate"
    );
    let gates = landing_gates(root);
    assert_eq!(gates.len(), 1, "PREMISE: one landing gate: {gates:?}");
    let (token, deadline) = token_and_deadline(&gates[0]);
    let run_ref = lane_e2e::run_ref(item);
    let tip = lane_e2e::ref_sha(root, item).expect("the run ref");
    let landed = tree_of(root, &tip);
    let derived = answers_derivation(root, &tree);

    let exit = answer_at(root, item, "exclude", deadline + 1);
    assert_ne!(
        exit,
        Exit::ConfigError,
        "`answer`'s expiry pass restaged the refused paths: a gate transition was refused"
    );
    assert_eq!(
        escalated(root, &token),
        1,
        "PREMISE: the landing gate expired into an escalation"
    );
    let opened = escalations(root);
    assert_eq!(
        opened.len(),
        1,
        "the escalation's row was never written: {opened:?}"
    );
    assert_eq!(
        opened[0].payload["tree_hash"].as_str(),
        Some(derived.as_str()),
        "the escalation does not bind `answer`'s own derivation"
    );
    let tip_after = lane_e2e::ref_sha(root, item).expect("the run ref");
    assert_eq!(
        tree_of(root, &tip_after),
        landed,
        "`answer` LANDED the run tree on `{run_ref}`"
    );
    assert!(
        blob(root, &run_ref, ".env").is_none(),
        "the refused `.env` reached `{run_ref}`"
    );

    let (escalation, _) = token_and_deadline(&opened[0]);
    assert_eq!(lane_e2e::answer_item(root, item, "approve"), Exit::Success);
    assert_eq!(
        answered_and_void(root, &escalation),
        (1, 0),
        "a later answer to the escalation was not APPLIED"
    );
}
