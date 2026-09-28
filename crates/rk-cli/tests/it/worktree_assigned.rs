//! **`s8e` §8e.4 — the run-owned worktree is read from the store** (D21-42 8d; AC-529; R-63).
//!
//! *(dated 2026-09-28, `s8e` build, package S8E-P5; the three-kinds record, proposed.)*
//!
//! D21-42 8d: *"A `worktree_assigned` row is written at run start. Every tier-1 write is checked
//! against it and refused with a typed rk-git error on mismatch."* `run` writes the row at every
//! start, on a created tree and on a reused one; every tier-1 act resolves its `RunOwnership` from
//! the latest one for its item through `rk_cli::verbs::run::assigned_ownership`, and from nothing
//! else — never `cwd`, never the `.redkiln/worktrees/<item>` convention.
//!
//! AC-529's three instruments are the first three tests: the foreign worktree's typed refusal with
//! `HEAD` unchanged on both branches and zero new rows, the positive control, and a resolution test
//! whose store row names a path that DIFFERS from the convention (with equal paths the convention
//! mutant, S8E-M9, would be unobservable — a fixture per branch).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_git::worktree::RunOwnership;
use rk_kernel::runtime::stage_transition_event;
use rk_kernel::runtime::tier_one::{Act, Boundary, Committed, Mark, Target, TierOneError};
use rk_ports::event_store::{EventStore, Query};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

const ITEM: &str = "gate-1";

fn git_out(cwd: &Path, args: &[&str]) -> String {
    match rk_git::git_exec::run(cwd, args) {
        Ok(out) => out.stdout_trimmed(),
        Err(e) => panic!("git {args:?} in {}: {e:?}", cwd.display()),
    }
}

/// The repository's real store, opened as the verbs open it.
fn open_store(root: &Path) -> Arc<Mutex<dyn EventStore>> {
    let repo = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let store =
        rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree).expect("the store opens");
    Arc::new(Mutex::new(store))
}

/// Every row, whatever its item key (`ByRun` is inert and returns the whole store).
fn every_row(root: &Path) -> Vec<Event> {
    open_store(root)
        .lock()
        .expect("the store's lock is sound")
        .query(Query::ByRun(String::new()))
        .expect("the store reads")
}

fn assigned_rows(root: &Path) -> Vec<Event> {
    every_row(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::WorktreeAssigned)
        .collect()
}

/// `gate-1` over `pack-min`'s Base variant: `has_repro` is absent, so the abstention gate opens and
/// the run exits 0 with its tree entered at the convention path.
fn repo_with_a_run() -> tempfile::TempDir {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let out = crate::run::spawn_run(repo.path(), ITEM, None);
    assert_eq!(
        out.status.code(),
        Some(0),
        "PREMISE: the run opens its gate and exits 0. stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    repo
}

/// The ownership the store records for `gate-1`, through the one resolver.
fn recorded(root: &Path) -> RunOwnership {
    let store = open_store(root);
    rk_cli::verbs::run::assigned_ownership(&store, ITEM)
        .expect("a sound lock")
        .unwrap_or_else(|| {
            panic!(
                "PREMISE: the run wrote no `worktree_assigned` row for {ITEM}; rows: {:?}",
                every_row(root)
                    .iter()
                    .map(|e| e.kind.wire_name())
                    .collect::<Vec<_>>()
            )
        })
}

fn exec(root: &Path) -> rk_git::git_exec::GitExec {
    rk_cli::repo::discover(Some(root))
        .expect("the repository discovers")
        .exec
}

fn advance_act() -> Act {
    Act::marked(
        Mark::Stage("review".to_owned()),
        vec![stage_transition_event("implement", "review")],
    )
}

/// **AC-529 (1): a second worktree of the same repository, on another branch, attempts the tier-1
/// write and gets the typed refusal** naming all four values — with `HEAD` unchanged on both
/// branches and zero new rows.
#[test]
fn a_tier1_write_from_a_foreign_worktree_is_refused_typed() {
    let repo = repo_with_a_run();
    let root = repo.path();
    let ownership = recorded(root);
    let elsewhere = tempfile::tempdir().expect("tempdir");
    let foreign = elsewhere.path().join("foreign");
    git_out(
        root,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "other",
            &foreign.display().to_string(),
        ],
    );
    let run_head = git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]);
    let other_head = git_out(root, &["rev-parse", "refs/heads/other"]);
    let rows_before = every_row(root).len();

    let store = open_store(root);
    let refused = Boundary::live(
        exec(root),
        Target {
            worktree: foreign.clone(),
            ownership: ownership.clone(),
            project: Some("rk-fixture".to_owned()),
            item: ITEM.to_owned(),
        },
    )
    .act(&store, advance_act());
    drop(store);

    let Err(TierOneError::Git(rk_git::error::GitError::ForeignWorktree {
        target_path,
        target_branch,
        owned_path,
        owned_branch,
    })) = refused
    else {
        panic!("the foreign write was not refused with the typed ForeignWorktree: {refused:?}");
    };
    assert_eq!(
        target_branch, "other",
        "the refusal names the attempted branch"
    );
    assert_eq!(
        owned_branch,
        format!("rk-run/{ITEM}"),
        "and the owned branch"
    );
    assert!(
        target_path.ends_with("foreign"),
        "the refusal names the attempted path: {}",
        target_path.display()
    );
    let owned = rk_git::path::WorktreePath::new(&ownership.worktree).expect("normalises");
    assert_eq!(
        rk_git::path::WorktreePath::new(&owned_path).expect("normalises"),
        owned,
        "the refusal names the path the STORE records"
    );
    assert_eq!(
        git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]),
        run_head,
        "the run branch moved"
    );
    assert_eq!(
        git_out(root, &["rev-parse", "refs/heads/other"]),
        other_head,
        "the foreign branch moved"
    );
    assert_eq!(
        every_row(root).len(),
        rows_before,
        "the refused write left a row"
    );
}

/// **AC-529 (2), the positive control: the run-owned worktree the store records succeeds**, and the
/// act's row is anchored to the commit it made on `rk-run/<item>`.
#[test]
fn the_positive_control_from_the_run_owned_worktree_succeeds() {
    let repo = repo_with_a_run();
    let root = repo.path();
    let ownership = recorded(root);
    let store = open_store(root);
    let done = Boundary::live(
        exec(root),
        Target {
            worktree: ownership.worktree.clone(),
            ownership: ownership.clone(),
            project: Some("rk-fixture".to_owned()),
            item: ITEM.to_owned(),
        },
    )
    .act(&store, advance_act())
    .expect("the run-owned write is authorised");
    let Committed::Live { sha, .. } = done else {
        panic!("a live act reported {done:?}");
    };
    assert_eq!(
        git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]),
        sha,
        "the act's commit is the run branch's tip"
    );
    let anchored = store
        .lock()
        .expect("the store's lock is sound")
        .query(Query::BySha(sha.clone()))
        .expect("the store reads");
    assert!(
        anchored
            .iter()
            .any(|e| e.kind == EventKind::StageTransition),
        "the act's row is not anchored to {sha}"
    );
}

/// **AC-529 (3): the resolution reads the STORE, not `cwd` and not the convention.** The run tree
/// is moved away from `.redkiln/worktrees/<item>` and a `worktree_assigned` row names where it now
/// is — a path that DIFFERS from the convention — and `answer`, spawned from the main checkout,
/// commits in the moved tree. Red under S8E-M9.
#[test]
fn ownership_is_read_from_the_store_not_from_cwd_or_convention() {
    let repo = repo_with_a_run();
    let root = repo.path();
    let convention = crate::run_tree(root, ITEM).path;
    assert!(
        convention.is_dir(),
        "PREMISE: the run entered the convention path"
    );
    let moved_parent = tempfile::tempdir().expect("tempdir");
    let moved: PathBuf = moved_parent.path().join("moved-run-tree");
    git_out(
        root,
        &[
            "worktree",
            "move",
            &convention.display().to_string(),
            &moved.display().to_string(),
        ],
    );
    assert!(!convention.exists(), "PREMISE: the convention path is gone");
    {
        let store = open_store(root);
        let mut guard = store.lock().expect("the store's lock is sound");
        guard
            .append(rk_kernel::runtime::tier_one::worktree_assigned_event(
                ITEM,
                &moved,
                &format!("rk-run/{ITEM}"),
                true,
                &[],
            ))
            .expect("the assignment row appends");
    }
    let run_head = git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]);

    let out = crate::answer::spawn_answer(root, ITEM, "approve", &crate::answer::BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the answer did not apply in the tree the store records. stderr:\n{stderr}"
    );
    let tip = git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]);
    assert_ne!(tip, run_head, "the answer made no commit on the run branch");
    assert_eq!(
        git_out(&moved, &["rev-parse", "HEAD"]),
        tip,
        "the answer's commit is not the moved tree's HEAD"
    );
    assert!(
        git_out(&moved, &["log", "-1", "--format=%B"]).contains("answered approve"),
        "the moved tree's HEAD is not the answer's `Gate:` mark"
    );
}

/// **The discard list moves from stderr into the row.** Two invocations over one item, with a
/// tracked file dirtied in the run tree between them: the first row reads `reused: false` with an
/// empty list, the second `reused: true` listing that path. Red under S8E-M41.
#[test]
fn a_reentry_records_reused_and_its_discard_list() {
    let repo = repo_with_a_run();
    let root = repo.path();
    let tree = crate::run_tree(root, ITEM).path;
    let tracked = git_out(&tree, &["ls-files"]);
    let file = tracked
        .lines()
        .find(|l| !l.trim().is_empty())
        .expect("PREMISE: the run tree tracks a file")
        .to_owned();
    std::fs::write(tree.join(&file), b"dirtied between invocations\n").expect("dirty");

    let out = crate::run::spawn_run(root, ITEM, None);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the second invocation did not re-enter. stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rows = assigned_rows(root);
    assert_eq!(
        rows.len(),
        2,
        "one `worktree_assigned` row per run start: {rows:?}"
    );
    let reused = |e: &Event| e.payload.get("reused").and_then(serde_json::Value::as_bool);
    let discarded = |e: &Event| -> Vec<String> {
        e.payload
            .get("discarded")
            .and_then(serde_json::Value::as_array)
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    };
    assert_eq!(
        reused(&rows[0]),
        Some(false),
        "the first start created the tree"
    );
    assert!(
        discarded(&rows[0]).is_empty(),
        "the first start discarded nothing"
    );
    assert_eq!(
        reused(&rows[1]),
        Some(true),
        "the second start reused the tree"
    );
    let list = discarded(&rows[1]);
    assert!(
        list.iter().any(|p| p.ends_with(&file)),
        "the re-entry's discard list does not name `{file}`: {list:?}"
    );
    for row in &rows {
        assert_eq!(row.item(), Some(ITEM), "the row carries its item key");
        assert_eq!(
            row.payload.get("item").and_then(serde_json::Value::as_str),
            Some(ITEM)
        );
    }
}

/// **The deleted-database reading: `answer` finds no row and refuses, fail-closed**, until a `run`
/// writes one. A gate is opened, the store is deleted, and `answer` must refuse by name, write no
/// row but its invocation's `reconcile_started` (AC-172's baseline), make no commit, and resolve
/// nothing from the convention path. Red under S8E-M42.
#[test]
fn answer_refuses_fail_closed_without_an_assignment_row() {
    let repo = repo_with_a_run();
    let root = repo.path();
    assert!(
        crate::run_tree(root, ITEM).path.is_dir(),
        "PREMISE: the convention path exists, so a fallback to it would find a tree"
    );
    let redkiln = crate::common_dir(root).join("redkiln");
    for name in ["store.sqlite3", "store.sqlite3-wal", "store.sqlite3-shm"] {
        let path = redkiln.join(name);
        if path.exists() {
            std::fs::remove_file(&path).expect("delete the store");
        }
    }
    let run_head = git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]);

    let out = crate::answer::spawn_answer(root, ITEM, "approve", &crate::answer::BOTH_FLAGS);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(3),
        "an answer with no assignment row must refuse. stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("worktree_assigned"),
        "the refusal does not name the missing `worktree_assigned` row. stderr:\n{stderr}"
    );
    let rows = every_row(root);
    assert_eq!(
        rows.iter().map(|e| e.kind).collect::<Vec<_>>(),
        vec![EventKind::ReconcileStarted],
        "the refusal wrote more than its invocation's `reconcile_started` row"
    );
    assert_eq!(
        git_out(root, &["rev-parse", &format!("refs/heads/rk-run/{ITEM}")]),
        run_head,
        "the refused answer committed"
    );
}
