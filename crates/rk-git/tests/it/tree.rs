//! `tree_hash` — ADR-0105 clause 4's single derivation.

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::tree::{self, TreeBasis};
use rk_git::version;

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// **The clause's sharpest requirement.** Untracked files are excluded from the working-state
/// derivation. If they were not, a gate's tree hash would move when someone dropped a scratch file
/// in the worktree, and every open gate would be spuriously void under AC-176.
#[test]
fn untracked_files_are_excluded_from_the_working_state_tree() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("t");
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let common = repo.join(".git");

    let before = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("before");

    std::fs::write(repo.join("scratch.tmp"), b"not tracked\n").expect("write untracked");
    let after = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("after");

    assert_eq!(
        before, after,
        "an UNTRACKED file changed the tree hash — ADR-0105 clause 4 excludes them, and any open \
         gate would otherwise go void whenever someone left a scratch file lying about"
    );
}

/// Control: the derivation is not simply inert. A change to a TRACKED file must move it, or the
/// test above would pass on a function that always returned the same value.
#[test]
fn control_a_tracked_modification_does_change_the_tree() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("t2");
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let common = repo.join(".git");

    let before = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("before");
    std::fs::write(repo.join("a.txt"), b"modified\n").expect("modify tracked file");
    let after = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("after");

    assert_ne!(
        before, after,
        "a tracked modification did not move the tree hash — the derivation is inert"
    );
}

/// On a clean tree the two bases agree. They are two routes to one value, not two values.
#[test]
fn checkpoint_and_working_basis_agree_on_a_clean_tree() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("t3");
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let common = repo.join(".git");

    let head = fx.git(&repo, &["rev-parse", "HEAD"]).stdout_trimmed();
    let by_commit = tree::tree_hash(
        &exec,
        &cwd,
        &common,
        &TreeBasis::CheckpointCommit(head.clone()),
    )
    .expect("by commit");
    let by_working = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("wt");

    assert_eq!(
        by_commit, by_working,
        "the two bases disagree on a clean tree — there is more than one derivation"
    );
}

/// The derivation leaves no temporary index behind, and does not disturb the real one.
#[test]
fn the_temporary_index_is_not_the_real_one_and_does_not_survive() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("t4");
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let common = repo.join(".git");

    // Stage nothing; leave a tracked modification unstaged so the real index is distinguishable.
    std::fs::write(repo.join("a.txt"), b"unstaged change\n").expect("modify");
    let real_index_before = std::fs::read(common.join("index")).expect("read real index");

    let _ = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("derive");

    let real_index_after = std::fs::read(common.join("index")).expect("read real index");
    assert_eq!(
        real_index_before, real_index_after,
        "the derivation wrote to the REAL index; it must use a temporary one"
    );
    assert!(
        !common.join("redkiln/tmp/index-tree-hash").exists(),
        "the temporary index survived the derivation"
    );
}

/// **The working state LESS a lane's excluded paths drops exactly those paths** (`s8f` D21-112;
/// ADR-0188 decisions 6-7). Three tracked files are modified: `a.txt`, `id_1.rs` and `id_[1].rs`.
/// Excluding `id_[1].rs` must give the tree the plain derivation gives once that one file is back
/// as `HEAD` has it — a path, never a pattern (`[1]` as a glob would also drop `id_1.rs`).
/// Controls: an empty exclusion is the plain derivation, and the exclusion does move the tree.
/// *(Added 2026-09-29, s8f build, package R3FIX-8f.)*
#[test]
fn working_state_less_drops_exactly_the_excluded_paths() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("t5");
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let common = repo.join(".git");
    for name in ["id_1.rs", "id_[1].rs"] {
        std::fs::write(repo.join(name), b"committed\n").expect("write");
    }
    fx.git(&repo, &["add", "--", "id_1.rs", "id_[1].rs"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "two more"]);
    for name in ["a.txt", "id_1.rs", "id_[1].rs"] {
        std::fs::write(repo.join(name), b"modified\n").expect("modify");
    }
    let less = |paths: &[&str]| {
        tree::tree_hash(
            &exec,
            &cwd,
            &common,
            &TreeBasis::WorkingStateLess(paths.iter().map(|p| (*p).to_owned()).collect()),
        )
        .expect("the derivation less the paths")
    };
    let all = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("plain");
    assert_eq!(
        less(&[]),
        all,
        "CONTROL: an empty exclusion is the plain derivation"
    );
    let excluded = less(&["id_[1].rs", "not-a-path.txt"]);
    assert_ne!(excluded, all, "CONTROL: the exclusion moved nothing");

    std::fs::write(repo.join("id_[1].rs"), b"committed\n").expect("restore");
    let reference = tree::tree_hash(&exec, &cwd, &common, &TreeBasis::WorkingState).expect("ref");
    assert_eq!(
        excluded, reference,
        "the derivation less `id_[1].rs` is not the working state with exactly that path as HEAD \
         has it"
    );
}

/// There is exactly one `write-tree` call site in the crate. A second derivation is the thing
/// clause 4 forbids, and a grep is the only instrument that can see it.
#[test]
fn there_is_exactly_one_write_tree_call_site() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    let mut stack = vec![src.clone()];
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
                    // The argv literal only. Two things are deliberately NOT counted, because
                    // neither invokes anything: `GitOp::WriteTree => "write-tree"`, which is the
                    // op's display name, and doc-comment prose. A first version of this test
                    // counted the name table and reported two sites — a guard scoped too wide
                    // (golden constraint 9), flagging a label as a second derivation.
                    let is_name_table = line.contains("=>");
                    let is_doc = line.trim_start().starts_with("//");
                    if line.contains("\"write-tree\"") && !is_name_table && !is_doc {
                        hits.push(format!("{}:{}", p.display(), i + 1));
                    }
                }
            }
        }
    }
    assert_eq!(
        hits.len(),
        1,
        "expected exactly one `write-tree` argv site (ADR-0105 clause 4: one algorithm, written \
         once); found: {hits:?}"
    );
    assert!(
        hits[0].contains("tree.rs"),
        "the single write-tree site is not in tree.rs: {hits:?}"
    );
}
