//! **AC-529** — a tier-1 write resolves the run-owned worktree from the store, never from cwd.
//!
//! The criterion's Verified-by names three things, and all three are here. The third is the one
//! that matters most and is easiest to omit: *"an assertion that the resolution reads the store's
//! worktree row and not `std::env::current_dir`"*. A naive implementation that compares against
//! cwd passes the refusal test and the positive control by accident, because on the happy path cwd
//! and the owned worktree are the same directory. Only the third test separates them.
//!
//! No spike measured a foreign-worktree write, so this is **built and tested, not measured**.

use rk_git::error::GitError;
use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::version;
use rk_git::worktree::{self, RunOwnership};

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// Instrument 1: the refusal, naming the target and the owned path and branch.
#[test]
fn a_write_from_a_foreign_worktree_is_refused_naming_both() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = fx.repo_with_one_commit("main");
    let main_path = WorktreePath::new(&main).expect("normalises");
    let run_branch = worktree::current_branch(&exec, &main_path).expect("branch");

    // A second worktree of the same repository, on another branch — AC-181's GIVEN, "of two
    // different items".
    fx.git(&main, &["branch", "other"]);
    let foreign = fx.sub("foreign");
    fx.git(
        &main,
        &["worktree", "add", foreign.to_str().expect("utf-8"), "other"],
    );

    let ownership = RunOwnership {
        worktree: main.clone(),
        branch: run_branch.clone(),
    };

    let head_main_before = fx.git(&main, &["rev-parse", "HEAD"]).stdout_trimmed();
    let head_foreign_before = fx.git(&foreign, &["rev-parse", "HEAD"]).stdout_trimmed();

    match worktree::authorise_tier1_write(&exec, &foreign, &ownership) {
        Err(GitError::ForeignWorktree {
            target_path,
            target_branch,
            owned_path,
            owned_branch,
        }) => {
            assert!(
                target_path.display().to_string().contains("foreign"),
                "the refusal does not name the target path: {target_path:?}"
            );
            assert_eq!(target_branch, "other");
            assert!(owned_path.display().to_string().contains("main"));
            assert_eq!(owned_branch, run_branch);
        }
        Err(other) => panic!("refused, but with the wrong error: {other:?}"),
        Ok(_) => panic!("a tier-1 write from a FOREIGN worktree was authorised (AC-529)"),
    }

    // No commit, no ref update: HEAD unmoved on both branches.
    assert_eq!(
        head_main_before,
        fx.git(&main, &["rev-parse", "HEAD"]).stdout_trimmed(),
        "HEAD moved on the run-owned branch after a refused write"
    );
    assert_eq!(
        head_foreign_before,
        fx.git(&foreign, &["rev-parse", "HEAD"]).stdout_trimmed(),
        "HEAD moved on the foreign branch after a refused write"
    );
}

/// Instrument 2: the positive control. Without it, a function that refused everything would pass.
#[test]
fn a_write_from_the_run_owned_worktree_succeeds() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = fx.repo_with_one_commit("owned");
    let main_path = WorktreePath::new(&main).expect("normalises");
    let branch = worktree::current_branch(&exec, &main_path).expect("branch");

    let ownership = RunOwnership {
        worktree: main.clone(),
        branch: branch.clone(),
    };
    let owned = worktree::authorise_tier1_write(&exec, &main, &ownership).expect("authorised");
    assert_eq!(owned.branch(), branch);
}

/// **Instrument 3 — the discriminating one, and measured to be exactly that.**
///
/// The target is a real worktree on the run's branch; the *store's* record names a different one.
/// Only a resolution that actually consults the store row refuses this.
///
/// Two mutants were run against this file, and they divide the work — recorded because the split
/// is not what it looks like from the criterion's wording:
///
/// * **Resolve ownership from `std::env::current_dir`** → caught by the refusal tests and by
///   `the_crate_never_reads_the_current_directory`, but **NOT** by this test: the harness's cwd is
///   the crate root, which matches neither fixture, so the refusal still fires — for the wrong
///   reason. A behavioural test cannot pin cwd without mutating global state shared by every other
///   test in the binary, so the structural grep is the instrument for that half.
/// * **Ignore the store's worktree row and authorise on the branch alone** → caught by **this test
///   and nothing else**. Every other test here stayed green, including the foreign-worktree
///   refusal, which passed by accident because that fixture's worktree is also on another branch.
///
/// So AC-529's third instrument is genuinely the one a naive implementation passes by accident —
/// and it takes both this test and the structural one to cover it from both sides.
#[test]
fn the_resolution_reads_the_store_row_and_not_the_current_directory() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);

    let actually_owned = fx.repo_with_one_commit("store-says-this-one");
    let where_we_are_standing = fx.repo_with_one_commit("cwd-is-here");
    let standing_path = WorktreePath::new(&where_we_are_standing).expect("normalises");
    let branch = worktree::current_branch(&exec, &standing_path).expect("branch");

    // The store owns a DIFFERENT worktree than the one the target names.
    let ownership = RunOwnership {
        worktree: actually_owned.clone(),
        branch: branch.clone(),
    };

    let result = worktree::authorise_tier1_write(&exec, &where_we_are_standing, &ownership);
    assert!(
        matches!(result, Err(GitError::ForeignWorktree { .. })),
        "the write was authorised against the directory we happen to be standing in rather than \
         the worktree the STORE owns for the run. This is AC-529's third instrument and the one a \
         cwd-based implementation passes by accident: {result:?}"
    );
}

/// The same rule in its other direction: the right worktree on the WRONG branch is also refused.
/// ADR-0018's original sentence is about the branch; clause 8 adds the path. Both halves hold.
#[test]
fn the_owned_worktree_on_the_wrong_branch_is_refused() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = fx.repo_with_one_commit("branchy");

    let ownership = RunOwnership {
        worktree: main.clone(),
        branch: "a-branch-that-head-is-not-on".into(),
    };
    assert!(
        matches!(
            worktree::authorise_tier1_write(&exec, &main, &ownership),
            Err(GitError::ForeignWorktree { .. })
        ),
        "a write was authorised on the run-owned worktree whose HEAD is not the run's branch"
    );
}

/// **No call to `current_dir` exists in this crate.** The structural half of instrument 3: the test
/// above shows the behaviour, this shows there is no second path that could reintroduce it.
#[test]
fn the_crate_never_reads_the_current_directory() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    let mut stack = vec![src];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)
            .expect("read src")
            .filter_map(Result::ok)
        {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let text = std::fs::read_to_string(&p).expect("read");
                for (i, line) in text.lines().enumerate() {
                    let is_doc = line.trim_start().starts_with("//");
                    if line.contains("env::current_dir") && !is_doc {
                        hits.push(format!("{}:{}", p.display(), i + 1));
                    }
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "rk-git reads the current directory, which AC-529 forbids as the basis for resolving the \
         run-owned worktree: {hits:?}"
    );
}

/// Clause 6: a worktree is re-created per run, never reused. An existing path is refused rather
/// than adopted — a leftover tree is a `doctor` orphans row, not a resource.
#[test]
fn an_existing_worktree_path_is_refused_not_reused() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = fx.repo_with_one_commit("reuse");
    let main_path = WorktreePath::new(&main).expect("normalises");
    fx.git(&main, &["branch", "wt"]);

    let wt = fx.sub("wt-tree");
    let created = worktree::add(&exec, &main_path, &wt, "wt").expect("first add");
    assert!(wt.exists(), "worktree was not created at {created}");

    let again = worktree::add(&exec, &main_path, &wt, "wt");
    assert!(
        matches!(again, Err(GitError::WorktreeRemoveFailed { .. })),
        "an existing worktree path was reused instead of refused (ADR-0105 clause 6): {again:?}"
    );
}

/// Removal works on the happy path, so the refusal above is about reuse and not about `add` being
/// broken.
#[test]
fn control_a_worktree_can_be_created_and_removed() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = fx.repo_with_one_commit("lifecycle");
    let main_path = WorktreePath::new(&main).expect("normalises");
    fx.git(&main, &["branch", "lc"]);

    let wt = fx.sub("lc-tree");
    worktree::add(&exec, &main_path, &wt, "lc").expect("add");
    assert!(wt.exists());
    worktree::remove(&exec, &main_path, &wt).expect("remove");
    assert!(!wt.exists(), "the worktree survived removal");
}
