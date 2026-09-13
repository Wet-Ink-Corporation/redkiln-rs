//! The disposable review clone (ADR-0098 decision 2; AC-191, AC-193).

use rk_git::clone::{self, WorkingTree};
use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::version;

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// Build a source repo with three commits plus a tag, and return `(path, file_url, review_sha)`.
/// The tag matters: AC-191 asserts the prepared clone carries **no other branch, tag, note, stash
/// or reflog entry**, which a source with nothing to leave behind could not demonstrate.
fn source(fx: &crate::support::Fixture) -> (std::path::PathBuf, String, String) {
    let repo = fx.sub("origin");
    std::fs::create_dir_all(&repo).expect("mkdir");
    fx.git(&repo, &["init", "--quiet"]);
    for (n, body) in [("one", "1\n"), ("two", "2\n"), ("three", "3\n")] {
        std::fs::write(repo.join("f.txt"), body).expect("write");
        fx.git(&repo, &["add", "f.txt"]);
        fx.git(&repo, &["commit", "--quiet", "-m", n]);
    }
    fx.git(&repo, &["tag", "v1"]);
    fx.git(&repo, &["branch", "sidebranch"]);
    fx.git(&repo, &["config", "uploadpack.allowAnySHA1InWant", "true"]);
    let sha = fx.git(&repo, &["rev-parse", "HEAD~1"]).stdout_trimmed();
    let url = format!("file:///{}", repo.display().to_string().replace('\\', "/"));
    (repo, url, sha)
}

/// AC-191: exactly one commit, and none of the source's other refs.
#[test]
fn the_prepared_clone_has_one_commit_and_no_other_ref() {
    let fx = crate::support::Fixture::new();
    let (_repo, url, sha) = source(&fx);
    let dest = fx.sub("review");

    clone::prepare_review_clone(&url, &dest, &sha, WorkingTree::NotNeeded).expect("clone");

    let count = fx
        .git(&dest, &["rev-list", "--count", "--all"])
        .stdout_trimmed();
    assert_eq!(
        count, "1",
        "the review clone carries {count} commits, not 1"
    );

    let tags = fx.git(&dest, &["tag", "-l"]).stdout_trimmed();
    assert!(tags.is_empty(), "the review clone carries tags: {tags}");

    let branches = fx
        .git(
            &dest,
            &["for-each-ref", "--format=%(refname)", "refs/heads"],
        )
        .stdout_trimmed();
    assert!(
        !branches.contains("sidebranch"),
        "the review clone carries the source's other branch: {branches}"
    );
}

/// The F17 regression, stated as the criterion states it: the recovery procedure — reading a
/// pre-fix blob for a changed file out of the clone's object store — **must now fail**.
#[test]
fn the_f17_recovery_procedure_fails_against_a_review_clone() {
    let fx = crate::support::Fixture::new();
    let (repo, url, sha) = source(&fx);
    let dest = fx.sub("review-f17");

    // The blob of `f.txt` at an EARLIER commit than the reviewed one: exactly what F17 read out.
    let older = fx.git(&repo, &["rev-parse", "HEAD~2"]).stdout_trimmed();
    let pre_fix_blob = fx
        .git(&repo, &["rev-parse", &format!("{older}:f.txt")])
        .stdout_trimmed();

    clone::prepare_review_clone(&url, &dest, &sha, WorkingTree::NotNeeded).expect("clone");

    let recovered = fx.try_git(&dest, &["cat-file", "-e", &pre_fix_blob]);
    assert!(
        recovered.is_err(),
        "the pre-fix blob {pre_fix_blob} is READABLE from the review clone — the F17 recovery \
         procedure still works and AC-191's shallow isolation is not in force"
    );
}

/// Control: the same blob IS readable from the source. Without this, the test above would pass
/// against a clone that simply failed to contain anything.
#[test]
fn control_the_pre_fix_blob_is_readable_from_the_source() {
    let fx = crate::support::Fixture::new();
    let (repo, _url, _sha) = source(&fx);
    let older = fx.git(&repo, &["rev-parse", "HEAD~2"]).stdout_trimmed();
    let blob = fx
        .git(&repo, &["rev-parse", &format!("{older}:f.txt")])
        .stdout_trimmed();
    assert!(
        fx.try_git(&repo, &["cat-file", "-e", &blob]).is_ok(),
        "the blob is not readable from the SOURCE either, so the F17 test asserts nothing"
    );
}

/// Bare unless declared: the default carries no working tree.
#[test]
fn the_review_clone_is_bare_unless_a_working_tree_is_declared() {
    let fx = crate::support::Fixture::new();
    let (_repo, url, sha) = source(&fx);

    let bare = fx.sub("bare-clone");
    clone::prepare_review_clone(&url, &bare, &sha, WorkingTree::NotNeeded).expect("bare");
    assert!(
        !bare.join("f.txt").exists(),
        "the default review clone has a working tree; ADR-0098 decision 2 makes it bare unless the \
         review activity declares otherwise"
    );

    let with_tree = fx.sub("wt-clone");
    clone::prepare_review_clone(&url, &with_tree, &sha, WorkingTree::Declared).expect("declared");
    assert!(
        with_tree.join(".git").exists(),
        "a clone declared as needing a working tree is not a non-bare repository"
    );
}

/// AC-193: removal is the entry step's action, and it leaves nothing behind.
#[test]
fn a_review_clone_is_removed_completely() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let (repo, url, sha) = source(&fx);
    let repo_path = WorktreePath::new(&repo).expect("normalises");
    let dest = fx.sub("removable");

    clone::prepare_review_clone(&url, &dest, &sha, WorkingTree::NotNeeded).expect("clone");
    assert!(dest.exists());

    clone::remove_review_clone(&exec, &repo_path, &dest).expect("remove");
    assert!(!dest.exists(), "the review clone survived removal");

    // Removing an absent clone is not an error: the entry step runs it whether or not a stale clone
    // was found, and a spurious failure there would block the run it is meant to clean up after.
    clone::remove_review_clone(&exec, &repo_path, &dest).expect("idempotent removal");
}
