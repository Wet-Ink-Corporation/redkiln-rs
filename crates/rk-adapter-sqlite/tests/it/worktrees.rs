//! **AC-370 (three-worktree half)** — *"GIVEN a repository with one or more linked worktrees /
//! WHEN any worktree opens the store / THEN it opens exactly one store located at `git rev-parse
//! --git-common-dir`/redkiln/"*. *Verified by:* *"an integration test creating three worktrees
//! and asserting one store path and one set of rows"*; the `doctor` stray-store row is ARMED to
//! 7d.
//!
//! The derivation is 7c's own (RT-17): nothing on the tree before this crate resolved a linked
//! worktree to its common dir — `rk-store` takes `common_dir: &Path` and Step 6's AC-178 tests
//! handed a temp dir in (critic gap 8). Two things this test pins that the prose names (s7c
//! §7c.13): the answer is `--git-common-dir`, not `--git-dir` (which is the linked tree's own
//! `.git/worktrees/<name>` and would yield a store per worktree); and a main-worktree answer is
//! the cwd-relative `.git`, which the adapter joins onto the worktree path before anything opens.

use rk_adapter_sqlite::path;
use rk_ports::event_store::{EventStore, Query};

use crate::support::{Fixture, event, open, store_files_under, wt};
use crate::three_worktrees;

fn canonical(p: &std::path::Path) -> std::path::PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|e| panic!("canonicalize {}: {e}", p.display()))
}

/// RED (observed 2026-09-18), twice, against the skeleton: first on the join rule — the derived
/// common dir from the main worktree was the bare relative `.git`; then, with the join in place
/// but `--git-dir` as the answer, on the second store file — three `store.sqlite3` under the
/// repository, one per worktree. Both outputs are quoted in the slice.
#[test]
fn ac370_one_store_per_common_dir() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = three_worktrees::build(&fx, &exec);
    let main_git = canonical(&repo.main.join(".git"));

    // ---- the derivation, pure: every worktree resolves to the main repository's `.git`, and the
    // main worktree's cwd-relative answer is joined to an absolute path before anything opens.
    for worktree in repo.all() {
        let common = path::common_dir(&exec, &wt(worktree))
            .unwrap_or_else(|e| panic!("common_dir from {}: {e}", worktree.display()));
        assert!(
            common.is_absolute(),
            "the common dir derived from {} is not absolute: {} (the join rule, s7c §7c.13)",
            worktree.display(),
            common.display()
        );
        assert_eq!(
            canonical(&common),
            main_git,
            "the common dir derived from {} is not the main repository's .git",
            worktree.display()
        );
    }

    // ---- the store, sequentially from each worktree: one path, one growing set of rows.
    let mut paths = Vec::new();
    for (i, worktree) in repo.all().into_iter().enumerate() {
        let mut store = open(&exec, worktree);
        store
            .append(event(u64::try_from(i).expect("small")))
            .unwrap_or_else(|e| panic!("append from {}: {e:?}", worktree.display()));
        let rows = store.query(Query::ByItem("x".to_owned())).expect("query");
        assert_eq!(
            rows.len(),
            i + 1,
            "opened from {}, the store holds the rows every earlier worktree appended",
            worktree.display()
        );
        paths.push(canonical(store.path()));
    }
    assert!(
        paths.iter().all(|p| p == &paths[0]),
        "the three worktrees opened different store files: {paths:?}"
    );
    assert_eq!(
        paths[0],
        canonical(&rk_store::conn::store_path(&main_git)),
        "the one store is at <git-common-dir>/redkiln/store.sqlite3"
    );

    // ---- one store FILE under the whole fixture — under the main `.git` (its `worktrees/<name>`
    // subtrees included, which is where a `--git-dir` derivation would have put two more) and
    // under each linked tree.
    let files = store_files_under(fx.path());
    assert_eq!(
        files.len(),
        1,
        "exactly one store.sqlite3 under the repository and its worktrees; found {files:?}"
    );
    assert_eq!(canonical(&files[0]), paths[0]);
}
