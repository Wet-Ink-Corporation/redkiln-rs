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

/// W2 (s8d-ii §8d-ii.3; ADR-0180 decision 9): **a tree without `RK_BASE` is refused, never
/// removed.** Renamed from `an_existing_worktree_path_is_refused_not_reused`, whose refusal is kept
/// below for the worktree that still carries ADR-0105 clause 6 whole (`redkiln-migrate`'s, AC-522).
/// *(dated 2026-09-27, s8d-ii build, P3.)*
#[test]
fn a_tree_without_rk_base_is_refused_and_never_removed() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "nobase", &[]);
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");
    let rt = worktree::derive(&root, &ItemId::parse(ITEM).expect("valid id"));
    // A tree at the derived path that redkiln did not create: registered, on the run branch at
    // its tip, and carrying no RK_BASE.
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &rt.branch,
            rt.path.to_str().expect("utf-8"),
        ],
    );
    std::fs::write(rt.path.join("work.txt"), b"not redkiln's\n").expect("write");

    let r = refused(
        "re-entry into a tree without RK_BASE",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::NoRkBase { path } if same_dir(path, &rt.path)),
        "refused for the wrong reason: {r:?}"
    );
    assert!(r.to_string().contains("RK_BASE"), "not named: {r}");
    assert_kept(&fx, &main, &rt.path, "after the refused re-entry");
    assert!(
        rt.path.join("work.txt").exists(),
        "the refused tree's untracked file was discarded"
    );

    match worktree::remove_landed(&exec, &main, ITEM).expect("removal decision") {
        Removal::Kept { reason, .. } => assert!(
            matches!(reason, KeepReason::Unmarked),
            "kept for the wrong reason: {reason:?}"
        ),
        Removal::Removed => {
            panic!("a tree without RK_BASE was REMOVED (W2: refused, never removed)")
        }
    }
    assert_kept(&fx, &main, &rt.path, "after the removal decision");

    // The kept refusal: ADR-0105 clause 6 still governs `redkiln-migrate`'s worktree (AC-522), and
    // `worktree::add` still refuses an existing path rather than adopting it.
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

/// Removal works on the happy path, so the refusals are about ownership and state and not about
/// creation or removal being broken. **Re-pointed** (s8d-ii §8d-ii.8): removal now needs a clean,
/// landed, marked, unlocked tree, so the tree is made by the lifecycle, closed, released and
/// removed by it. *(dated 2026-09-27, s8d-ii build, P3.)*
#[test]
fn control_a_worktree_can_be_created_and_removed() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "lifecycle", &[]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(7), &|_| None, None).expect("created");
    assert!(e.tree.path.exists());
    close_the_lane(&fx, &e.tree.path);
    assert!(worktree::release(&exec, &main, ITEM, &owner(7)).expect("released"));
    let removal = worktree::remove_landed(&exec, &main, ITEM).expect("removal decision");
    assert!(matches!(removal, Removal::Removed), "{removal:?}");
    assert!(!e.tree.path.exists(), "the worktree survived removal");
}

// ---------------------------------------------------------------------------------------------
// s8d-ii §8d-ii.1, §8d-ii.2 and §8d-ii.6 (D21-44; D21-45 W1; D21-47; D21-48 X1): the one
// derivation from the main worktree, the item-id validator, the case-fold rule, the symlink and
// junction guards, the post-create realpath check, the ignore refusal and the length rule.
// *(dated 2026-09-27, s8d-ii build, P2.)*
// ---------------------------------------------------------------------------------------------

use rk_git::worktree::{ItemId, ItemIdRefusal, WorktreeRefusal};

/// The refusal inside a `GitError::WorktreeRefused`, or a panic naming what came back instead.
fn refused<T: std::fmt::Debug>(what: &str, r: Result<T, GitError>) -> WorktreeRefusal {
    match r {
        Err(GitError::WorktreeRefused { refusal }) => {
            println!("{what}: refused: {refusal}");
            refusal
        }
        Err(other) => panic!("{what}: refused, but not by a WorktreeRefused: {other:?}"),
        Ok(v) => panic!("{what}: ADMITTED ({v:?}), and it must be refused by name"),
    }
}

/// A repository whose tracked `.gitignore` carries the run-tree root's row (D21-44), with this
/// box's global excludes and `core.longpaths` pinned locally so no operator config reaches the
/// guards (the production reads run with the inherited environment).
fn ignored_repo(fx: &crate::support::Fixture, name: &str) -> std::path::PathBuf {
    let repo = pinned_repo(fx, name);
    std::fs::write(repo.join(".gitignore"), b".redkiln/worktrees/\n").expect("write .gitignore");
    fx.git(&repo, &["add", ".gitignore"]);
    fx.git(
        &repo,
        &["commit", "--quiet", "-m", "ignore the run-tree root"],
    );
    repo
}

/// One commit, NO ignore row, and the operator's config pinned out locally.
fn pinned_repo(fx: &crate::support::Fixture, name: &str) -> std::path::PathBuf {
    let repo = fx.repo_with_one_commit(name);
    let empty = fx.sub(&format!("{name}-empty-excludes"));
    std::fs::write(&empty, b"").expect("write empty excludes");
    fx.git(
        &repo,
        &[
            "config",
            "core.excludesFile",
            empty.to_str().expect("utf-8"),
        ],
    );
    fx.git(&repo, &["config", "core.longpaths", "false"]);
    repo
}

fn utf16_len(p: &std::path::Path) -> usize {
    p.to_string_lossy().encode_utf16().count()
}

fn same_dir(a: &std::path::Path, b: &std::path::Path) -> bool {
    crate::support::normalise_path(&a.to_string_lossy())
        == crate::support::normalise_path(&b.to_string_lossy())
}

/// W1's validator, row by row: each row is refused for ITS OWN reason, and the admitted controls
/// show the table is not refusing everything.
#[test]
fn an_item_id_that_is_not_a_safe_path_and_branch_component_is_refused() {
    use ItemIdRefusal as R;
    let rows: Vec<(String, R)> = vec![
        (String::new(), R::Empty),
        ("a".repeat(65), R::TooLong),
        ("a/b".into(), R::UnsafeCharacter),
        ("a\\b".into(), R::UnsafeCharacter),
        ("a b".into(), R::UnsafeCharacter),
        ("a:b".into(), R::UnsafeCharacter),
        ("caf\u{e9}".into(), R::UnsafeCharacter),
        (".".into(), R::Reserved),
        ("..".into(), R::Reserved),
        (".git".into(), R::Reserved),
        (".GIT".into(), R::Reserved),
        (".hidden".into(), R::LeadingDot),
        ("a.".into(), R::TrailingDot),
        ("a..b".into(), R::DoubleDot),
        ("a.lock".into(), R::LockSuffix),
        ("A.LOCK".into(), R::LockSuffix),
        ("CON".into(), R::DeviceName),
        ("nul.txt".into(), R::DeviceName),
        ("com1".into(), R::DeviceName),
        ("Lpt9.log".into(), R::DeviceName),
        ("aux".into(), R::DeviceName),
        ("prn".into(), R::DeviceName),
        ("x.review-ref".into(), R::ReviewRefSuffix),
        ("x.Review-Ref".into(), R::ReviewRefSuffix),
    ];
    let mut wrong = Vec::new();
    for (raw, want) in &rows {
        match ItemId::parse(raw) {
            Err(GitError::WorktreeRefused {
                refusal: WorktreeRefusal::UnsafeItemId { item, reason },
            }) => {
                if reason != *want || item != *raw {
                    wrong.push(format!(
                        "`{raw}`: refused as {} (`{item}`), want {}",
                        reason.name(),
                        want.name()
                    ));
                }
                let msg = WorktreeRefusal::UnsafeItemId {
                    item: item.clone(),
                    reason,
                }
                .to_string();
                assert!(
                    msg.contains(want.name()) && msg.contains(raw.as_str()),
                    "the refusal does not name the id and its rule: {msg}"
                );
            }
            other => wrong.push(format!(
                "`{raw}`: {other:?}, want a refusal as {}",
                want.name()
            )),
        }
    }
    assert!(
        wrong.is_empty(),
        "{} of {} rows were not refused for their own reason (no validator?):\n{}",
        wrong.len(),
        rows.len(),
        wrong.join("\n")
    );
    // CONTROLS: safe ids are admitted, including the 64-character maximum and names that only
    // look like a device name or a reserved one.
    for ok in [
        "fix-1",
        "rk-fix-doctor-test-count",
        "a.b",
        "A_b-9",
        "com0",
        "lpt10",
        "console",
        "con-1",
        "gitx",
        "x.review-refs",
        &"a".repeat(64),
    ] {
        let id = ItemId::parse(ok).unwrap_or_else(|e| panic!("CONTROL: `{ok}` was refused: {e:?}"));
        assert_eq!(id.as_str(), ok);
    }
}

/// W1's case-fold rule, which the table cannot hold because it reads the registry: `fix-1` is
/// refused beside a registered `Fix-1`, and `other-2` beside a bare `rk-run/Other-2` branch.
#[test]
fn an_item_id_that_case_folds_onto_a_registered_tree_is_refused() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = ignored_repo(&fx, "fold");
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");
    let registered = worktree::derive(&root, &ItemId::parse("Fix-1").expect("valid id"));
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &registered.branch,
            registered.path.to_str().expect("utf-8"),
        ],
    );
    fx.git(&main, &["branch", "rk-run/Other-2"]);

    let r = refused(
        "fix-1 beside a registered Fix-1",
        worktree::prepare_creation(&exec, &main, "fix-1", "HEAD"),
    );
    assert!(
        matches!(&r, WorktreeRefusal::CaseFoldCollision { item, existing } if item == "fix-1" && existing == "Fix-1"),
        "refused for the wrong reason: {r:?}"
    );
    let r = refused(
        "other-2 beside an rk-run/Other-2 branch with no tree",
        worktree::prepare_creation(&exec, &main, "other-2", "HEAD"),
    );
    assert!(
        matches!(&r, WorktreeRefusal::CaseFoldCollision { item, existing } if item == "other-2" && existing == "Other-2"),
        "refused for the wrong reason: {r:?}"
    );
    // CONTROLS: the id that EQUALS the registered one is not a collision, and an unrelated id is
    // admitted.
    let same = worktree::prepare_creation(&exec, &main, "Fix-1", "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: the registered id itself was refused: {e:?}"));
    assert!(same_dir(&same.path, &registered.path));
    worktree::prepare_creation(&exec, &main, "fix-3", "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: an unrelated id was refused: {e:?}"));
}

/// The root is the MAIN worktree (the first `worktree list --porcelain` record), whichever
/// checkout the call starts in: a linked worktree, or a run tree inside the root.
#[test]
fn the_root_is_the_main_worktree_when_run_from_a_linked_worktree() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = ignored_repo(&fx, "rootmain");
    let linked = fx.sub("rootmain.swt");
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "side",
            linked.to_str().expect("utf-8"),
        ],
    );

    let from_linked = worktree::main_worktree(&exec, &linked).expect("main worktree");
    assert!(
        same_dir(&from_linked, &main),
        "read from the linked worktree, the root is `{}`, not the main worktree `{}`",
        from_linked.display(),
        main.display()
    );

    let tree = worktree::prepare_creation(&exec, &linked, "item-1", "HEAD").expect("admitted");
    let want = main.join(".redkiln").join("worktrees").join("item-1");
    assert!(
        same_dir(&tree.path, &want),
        "a run started in the linked worktree derives `{}`, not `{}`",
        tree.path.display(),
        want.display()
    );
    assert_eq!(tree.branch, "rk-run/item-1");
    assert!(same_dir(
        &tree.reference,
        &main
            .join(".redkiln")
            .join("worktrees")
            .join("item-1.review-ref")
    ));

    // A run started INSIDE a run tree does not nest one inside it.
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &tree.branch,
            tree.path.to_str().expect("utf-8"),
        ],
    );
    let from_run_tree = worktree::main_worktree(&exec, &tree.path).expect("main worktree");
    assert!(
        same_dir(&from_run_tree, &main),
        "read from inside a run tree, the root is `{}`, not `{}`",
        from_run_tree.display(),
        main.display()
    );
}

/// A repository whose first record carries `bare` has no main worktree: refused by name.
#[test]
fn a_bare_repository_is_refused_by_name() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let bare = fx.sub("bare.git");
    fx.git(
        fx.path(),
        &["init", "--quiet", "--bare", bare.to_str().expect("utf-8")],
    );
    let r = refused(
        "the main worktree of a bare repository",
        worktree::main_worktree(&exec, &bare),
    );
    assert!(
        matches!(&r, WorktreeRefusal::BareRepository { repo } if same_dir(repo, &bare)),
        "refused for the wrong reason: {r:?}"
    );
    assert!(r.to_string().contains("bare repository"), "not named: {r}");
    let r = refused(
        "creation in a bare repository",
        worktree::prepare_creation(&exec, &bare, "item-1", "HEAD"),
    );
    assert!(matches!(r, WorktreeRefusal::BareRepository { .. }), "{r:?}");
    // CONTROL: a repository with a main worktree is admitted.
    let main = ignored_repo(&fx, "notbare");
    worktree::main_worktree(&exec, &main).expect("CONTROL: a non-bare repository was refused");
}

/// Make `link` a directory link to `target`: a junction on Windows (no Developer Mode needed), a
/// symlink elsewhere. Returns which it made, so the test can say which it ran.
#[cfg(windows)]
#[expect(
    clippy::disallowed_types,
    reason = "a test building a junction with cmd's mklink /J; the PRODUCT spawns nothing but git, \
              through rk-git's one GitExec wrapper"
)]
fn dir_link(link: &std::path::Path, target: &std::path::Path) -> &'static str {
    // cmd reads a forward slash as a switch (`C:/Users` is `/Users`), and a derived path carries
    // the porcelain's forward slashes, so both go over in backslash form.
    let back = |p: &std::path::Path| p.to_string_lossy().replace('/', "\\");
    let out = std::process::Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(back(link))
        .arg(back(target))
        .output()
        .expect("cmd could be spawned");
    assert!(
        out.status.success(),
        "mklink /J failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    "junction"
}

#[cfg(not(windows))]
fn dir_link(link: &std::path::Path, target: &std::path::Path) -> &'static str {
    std::os::unix::fs::symlink(target, link).expect("symlink");
    "symlink"
}

/// W1: creation refuses when `.redkiln`, `.redkiln/worktrees` or `<item>` is a symlink or a
/// junction; each case in its own repository, and a plain-directory control.
#[test]
fn a_symlinked_redkiln_or_worktrees_directory_is_refused() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    for (n, segment) in [
        ".redkiln",
        ".redkiln/worktrees",
        ".redkiln/worktrees/item-1",
    ]
    .into_iter()
    .enumerate()
    {
        let main = ignored_repo(&fx, &format!("link{n}"));
        let elsewhere = fx.sub(&format!("elsewhere{n}"));
        std::fs::create_dir_all(elsewhere.join("worktrees")).expect("mkdir elsewhere");
        let link = main.join(segment);
        std::fs::create_dir_all(link.parent().expect("parent")).expect("mkdir parent");
        let kind = dir_link(&link, &elsewhere);
        println!("built a {kind} at {segment}");
        let r = refused(
            &format!("a {kind} at {segment}"),
            worktree::prepare_creation(&exec, &main, "item-1", "HEAD"),
        );
        match &r {
            WorktreeRefusal::LinkedSegment { path, .. } => assert!(
                same_dir(path, &link),
                "the refusal names `{}`, not the linked segment `{}`",
                path.display(),
                link.display()
            ),
            other => panic!("a {kind} at {segment} was refused for the wrong reason: {other:?}"),
        }
        assert!(
            r.to_string()
                .contains(segment.rsplit('/').next().expect("seg"))
        );
    }
    // CONTROL: the same three segments as plain directories are admitted.
    let main = ignored_repo(&fx, "plain");
    std::fs::create_dir_all(main.join(".redkiln").join("worktrees").join("item-1")).expect("mkdir");
    worktree::prepare_creation(&exec, &main, "item-1", "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: plain directories were refused: {e:?}"));
}

/// W1's post-create realpath check: a tree at the derived path whose canonical path lies outside
/// the canonical root is refused, naming both paths, and neither in the `\\?\` form.
#[test]
fn a_created_worktree_whose_realpath_leaves_the_root_is_refused() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = ignored_repo(&fx, "real");
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");

    // The escape: a real worktree OUTSIDE the root, reached through a link at the derived path.
    let escape = worktree::derive(&root, &ItemId::parse("escape").expect("valid id"));
    let outside = fx.sub("outside-tree");
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &escape.branch,
            outside.to_str().expect("utf-8"),
        ],
    );
    std::fs::create_dir_all(escape.path.parent().expect("parent")).expect("mkdir root");
    let kind = dir_link(&escape.path, &outside);
    println!("the escape is a {kind}");
    let r = refused(
        "a tree resolving outside the root",
        worktree::verify_created(&root, &escape.path),
    );
    match &r {
        WorktreeRefusal::RealpathOutsideRoot {
            created,
            resolved,
            root: named_root,
        } => {
            assert!(
                same_dir(created, &escape.path),
                "created: {}",
                created.display()
            );
            assert!(
                same_dir(resolved, &outside),
                "resolved: {}",
                resolved.display()
            );
            for p in [created, resolved, named_root] {
                let s = p.to_string_lossy();
                assert!(
                    !s.starts_with(r"\\?\") && !s.starts_with("//?/"),
                    "the refusal carries an extended-length path: {s}"
                );
            }
        }
        other => panic!("refused for the wrong reason: {other:?}"),
    }
    let msg = r.to_string();
    assert!(
        msg.contains("outside-tree") && msg.contains("escape"),
        "both paths not named: {msg}"
    );

    // CONTROL: a tree created at its derived path is admitted.
    let inside = worktree::derive(&root, &ItemId::parse("inside").expect("valid id"));
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &inside.branch,
            inside.path.to_str().expect("utf-8"),
        ],
    );
    worktree::verify_created(&root, &inside.path)
        .unwrap_or_else(|e| panic!("CONTROL: a tree under the root was refused: {e:?}"));
}

/// D21-48 X1: *"Refuse by name."* Creation refuses when `git check-ignore`, run in the main
/// worktree, does not report `.redkiln/worktrees/<item>` as ignored; the tracked row and `init`'s
/// `info/exclude` form are both admitted.
#[test]
fn creation_refuses_when_the_worktree_root_is_not_ignored() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = pinned_repo(&fx, "unignored");
    let r = refused(
        "creation under an unignored root",
        worktree::prepare_creation(&exec, &main, "item-1", "HEAD"),
    );
    assert!(
        matches!(&r, WorktreeRefusal::RootNotIgnored { path, .. } if path == ".redkiln/worktrees/item-1"),
        "refused for the wrong reason: {r:?}"
    );
    assert!(
        r.to_string().contains(".redkiln/worktrees/item-1"),
        "not named: {r}"
    );
    // CONTROL 1: `init`'s form, a row in info/exclude.
    let excl = main.join(".git").join("info").join("exclude");
    std::fs::create_dir_all(excl.parent().expect("parent")).expect("mkdir info");
    std::fs::write(&excl, b"# redkiln-runtime\n/.redkiln/worktrees/\n").expect("write exclude");
    worktree::prepare_creation(&exec, &main, "item-1", "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: the info/exclude row was not honoured: {e:?}"));
    // CONTROL 2: the tracked .gitignore row.
    let tracked = ignored_repo(&fx, "tracked");
    worktree::prepare_creation(&exec, &tracked, "item-1", "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: the tracked row was not honoured: {e:?}"));
}

/// Commit a path of exactly `len` characters to `repo`'s index and `HEAD` WITHOUT writing it to
/// the working tree (a 200-character path is past MAX_PATH on disk, and the rule reads the fork
/// commit, not the checkout).
fn commit_path_of_len(fx: &crate::support::Fixture, repo: &std::path::Path, len: usize) -> String {
    assert!(len >= 4, "a {len}-character path cannot be built");
    let mut p = String::new();
    while p.len() + 2 < len.saturating_sub(4) {
        p.push_str("d/");
    }
    while p.len() < len {
        p.push('f');
    }
    assert_eq!(p.len(), len);
    std::fs::write(fx.sub("blob.txt"), b"long\n").expect("write blob");
    let sha = fx
        .git(
            repo,
            &[
                "hash-object",
                "-w",
                fx.sub("blob.txt").to_str().expect("utf-8"),
            ],
        )
        .stdout_trimmed();
    fx.git(
        repo,
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("100644,{sha},{p}"),
        ],
    );
    fx.git(repo, &["commit", "--quiet", "-m", "a long path"]);
    p
}

/// The worst-case bound's fixed terms for `item_len`, spelled out as §8d-ii.6 states them and
/// NEVER read from `worktree::path_bound`, the function under test: `/.redkiln/worktrees/` is 20,
/// `.review-ref` is 11, and the `/` before the tracked path is 1. A fixture sized from `path_bound`
/// itself moves with any term a mutant drops (round-1 finding, `R1FIX-PB-SUFFIX` and
/// `R1FIX-PB-PLUS1`). *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)*
fn fixed_terms(root: &std::path::Path, item_len: usize) -> usize {
    utf16_len(root) + 20 + item_len + 11 + 1
}

/// §8d-ii.6: creation refuses by name when root + 20 + item + 11 + 1 + the longest path at the
/// fork commit exceeds 259 and `core.longpaths` is not `true`; at exactly 259 it is admitted, and
/// with `core.longpaths=true` it is admitted. The fixture and the expected bound are computed from
/// the formula's literal terms (`fixed_terms`), never from `path_bound`. *(dated 2026-09-28, s8d-ii
/// build, package R1FIX-8dii: it read `worktree::path_bound(utf16_len(&root), item.len(), 0)`.)*
#[test]
fn creation_refuses_a_path_past_the_platform_limit_by_name() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let item = "item-1";

    let over = ignored_repo(&fx, "over");
    let root = worktree::main_worktree(&exec, &over).expect("main worktree");
    let fixed = fixed_terms(&root, item.len());
    let long = commit_path_of_len(&fx, &over, 260 - fixed);
    assert_eq!(
        fixed + long.len(),
        260,
        "the fixture is not a 260 worst case"
    );
    let r = refused(
        "a 260-character worst case",
        worktree::prepare_creation(&exec, &over, item, "HEAD"),
    );
    assert!(
        matches!(&r, WorktreeRefusal::PathPastLimit { bound: 260, limit: 259, longest, longest_path, .. } if *longest == long.len() && *longest_path == long),
        "refused for the wrong reason: {r:?}"
    );
    assert!(r.to_string().contains("core.longpaths"), "not named: {r}");

    // CONTROL: core.longpaths=true admits the same tree.
    fx.git(&over, &["config", "core.longpaths", "true"]);
    worktree::prepare_creation(&exec, &over, item, "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: core.longpaths=true did not admit: {e:?}"));

    // CONTROL: exactly 259 is within the limit.
    let edge = ignored_repo(&fx, "edge");
    let root = worktree::main_worktree(&exec, &edge).expect("main worktree");
    let fixed = fixed_terms(&root, item.len());
    commit_path_of_len(&fx, &edge, 259 - fixed);
    worktree::prepare_creation(&exec, &edge, item, "HEAD")
        .unwrap_or_else(|e| panic!("CONTROL: a 259-character worst case was refused: {e:?}"));

    // The formula's terms, pinned on literal inputs: 10 + 20 + 6 + 11 + 1 + 100.
    assert_eq!(
        worktree::path_bound(10, 6, 100),
        148,
        "path_bound's terms moved"
    );
}

/// The control for S8D-II-M18 (D21-47): the bound is computed on the ACTUAL item. A short item
/// under a root too long for a 64-character one is admitted. Green at entry by design (§8d-ii.8's
/// seven pins); M18 shows it can fail, and the refusal test above shows the rule is live.
#[test]
fn the_length_rule_reads_the_actual_item_length() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = ignored_repo(&fx, "actual");
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");
    // *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii)* sized from the literal terms, not from
    // `path_bound` (it read `worktree::path_bound(utf16_len(&root), 64, 0)`).
    let fixed64 = fixed_terms(&root, 64);
    let long = commit_path_of_len(&fx, &main, 260 - fixed64);
    assert!(
        fixed64 + long.len() > 259,
        "the fixture root is not too long for a 64-character item"
    );
    worktree::prepare_creation(&exec, &main, "a", "HEAD").unwrap_or_else(|e| {
        panic!("a 1-character item was refused on a 64-character bound: {e:?}")
    });
}

// ---------------------------------------------------------------------------------------------
// s8d-ii §8d-ii.2 and §8d-ii.3 (D21-45 W1 and W2; D21-47; ADR-0180 decisions 5, 6, 7 and 9,
// proposed): the `RK_BASE` marker, the lock and its injected liveness predicate, re-entry with
// an explicit, reported discard, the refusals that keep a tree, and removal of a clean, landed,
// marked, unlocked tree only, unforced and never through a junction.
// *(dated 2026-09-27, s8d-ii build, P3.)*
// ---------------------------------------------------------------------------------------------

use rk_git::worktree::{Creation, KeepReason, LockOwner, Removal};

/// The item every lifecycle test runs.
const ITEM: &str = "item-1";

/// A lock owner with a fixed start time per pid, so a test can name a live, a dead or a reused
/// pid through its injected probe.
fn owner(pid: u32) -> LockOwner {
    LockOwner {
        pid,
        start: u64::from(pid) * 1000,
    }
}

/// `ignored_repo`'s shape plus extra ignore rows, `core.autocrlf` pinned off (the production
/// reads run in the inherited environment, and this box sets it globally), one commit.
fn lifecycle_repo(fx: &crate::support::Fixture, name: &str, extra: &[&str]) -> std::path::PathBuf {
    let repo = pinned_repo(fx, name);
    fx.git(&repo, &["config", "core.autocrlf", "false"]);
    let mut rows = String::from(".redkiln/worktrees/\n");
    for r in extra {
        rows.push_str(r);
        rows.push('\n');
    }
    std::fs::write(repo.join(".gitignore"), rows.as_bytes()).expect("write .gitignore");
    fx.git(&repo, &["add", ".gitignore"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "ignore rows"]);
    repo
}

/// The tree's `worktree list --porcelain` record, or `None` when it is not registered.
fn record_of(
    fx: &crate::support::Fixture,
    main: &std::path::Path,
    tree: &std::path::Path,
) -> Option<Vec<String>> {
    let out = fx
        .git(main, &["worktree", "list", "--porcelain"])
        .stdout_trimmed();
    out.split("\n\n")
        .map(|rec| {
            rec.lines()
                .map(|l| l.trim_end_matches('\r').to_string())
                .collect::<Vec<_>>()
        })
        .find(|rec| {
            rec.first()
                .and_then(|l| l.strip_prefix("worktree "))
                .is_some_and(|p| same_dir(std::path::Path::new(p), tree))
        })
}

/// The tree's `locked` line's reason: `None` unlocked, `Some("")` locked with no reason.
fn lock_of(
    fx: &crate::support::Fixture,
    main: &std::path::Path,
    tree: &std::path::Path,
) -> Option<String> {
    let rec = record_of(fx, main, tree).expect("the tree is registered");
    rec.iter().find_map(|l| {
        if l == "locked" {
            Some(String::new())
        } else {
            l.strip_prefix("locked ").map(str::to_string)
        }
    })
}

/// The tree is still registered AND still on disk.
fn assert_kept(
    fx: &crate::support::Fixture,
    main: &std::path::Path,
    tree: &std::path::Path,
    when: &str,
) {
    assert!(
        tree.exists(),
        "{when}: the tree `{}` was removed from disk (refused trees are never removed)",
        tree.display()
    );
    assert!(
        record_of(fx, main, tree).is_some(),
        "{when}: the tree `{}` is no longer registered (refused trees are never removed)",
        tree.display()
    );
}

/// The tree's own gitdir, as `git rev-parse --git-dir` prints it from inside the tree.
fn gitdir_of(fx: &crate::support::Fixture, tree: &std::path::Path) -> std::path::PathBuf {
    std::path::PathBuf::from(
        fx.git(tree, &["rev-parse", "--path-format=absolute", "--git-dir"])
            .stdout_trimmed(),
    )
}

fn sha(fx: &crate::support::Fixture, at: &std::path::Path, rev: &str) -> String {
    fx.git(at, &["rev-parse", "--verify", rev]).stdout_trimmed()
}

/// Commit the lane's terminal `closed` mark on the tree's run branch (s8d-ii §8d-ii.3: before
/// `s8e` mints it, *"a `Story:` trailer whose stage value is `closed`, which the rk-git fixture
/// writes directly"*).
fn close_the_lane(fx: &crate::support::Fixture, tree: &std::path::Path) {
    std::fs::write(tree.join("closeout.txt"), b"closed\n").expect("write closeout");
    fx.git(tree, &["add", "closeout.txt"]);
    fx.git(
        tree,
        &[
            "commit",
            "--quiet",
            "-m",
            "closeout",
            "-m",
            &format!("Story: redkiln-rs/{ITEM} closed"),
        ],
    );
}

/// Create the run tree through the lifecycle with `owner(1)`, and release its lock.
fn created_and_released(
    fx: &crate::support::Fixture,
    exec: &GitExec,
    main: &std::path::Path,
) -> rk_git::worktree::Entered {
    let _ = fx;
    let e = worktree::enter(exec, main, ITEM, &owner(1), &|_| None, None)
        .unwrap_or_else(|err| panic!("the first creation was refused: {err:?}"));
    assert!(
        worktree::release(exec, main, ITEM, &owner(1)).expect("release"),
        "the creating run's own lock was not released"
    );
    e
}

/// W1's marker: `RK_BASE` is written into the tree's OWN gitdir (not its working tree) and holds
/// the fork sha and the item id, so ownership is recoverable from git alone (ADR-0109 d4).
#[test]
fn creation_writes_rk_base_into_the_worktree_gitdir() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "base", &[]);
    let head = sha(&fx, &main, "HEAD");
    let e = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None).expect("created");
    assert!(matches!(e.created, Creation::New), "{:?}", e.created);
    assert_eq!(e.fork, head, "the fork is not the HEAD read before the add");

    let gitdir = gitdir_of(&fx, &e.tree.path);
    let marker = gitdir.join("RK_BASE");
    let text = std::fs::read_to_string(&marker).unwrap_or_else(|err| {
        panic!(
            "no RK_BASE in the tree's gitdir `{}` ({err}) — W1's ownership marker is absent",
            gitdir.display()
        )
    });
    assert!(
        text.lines().any(|l| l == format!("fork {head}")),
        "RK_BASE does not record the fork sha {head}: {text:?}"
    );
    assert!(
        text.lines().any(|l| l == format!("item {ITEM}")),
        "RK_BASE does not record the item: {text:?}"
    );
    assert!(
        !e.tree.path.join("RK_BASE").exists(),
        "RK_BASE was written into the working tree, where git would list it"
    );
    let read = worktree::read_rk_base(&exec, &e.tree.path)
        .expect("read")
        .expect("the marker reads back");
    assert_eq!(
        (read.fork.as_str(), read.item.as_str()),
        (head.as_str(), ITEM)
    );
    // The branch forked at the fork sha, with no upstream (`--no-track`).
    assert_eq!(sha(&fx, &main, &e.tree.branch), head);
    assert!(
        fx.try_git(
            &main,
            &[
                "config",
                "--get",
                &format!("branch.{}.merge", e.tree.branch)
            ]
        )
        .is_err(),
        "the run branch tracks an upstream"
    );
}

/// W1's lock: `git worktree lock --reason "redkiln run <item> (pid N start T)"` at every run
/// start, and released at the run's end.
#[test]
fn creation_locks_the_worktree_with_the_pid_and_start_reason() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "lock", &[]);
    let me = LockOwner {
        pid: 4242,
        start: 133_000_000_777,
    };
    let e = worktree::enter(&exec, &main, ITEM, &me, &|_| None, None).expect("created");
    let want = format!("redkiln run {ITEM} (pid 4242 start 133000000777)");
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path).as_deref(),
        Some(want.as_str()),
        "the created tree is not locked with the pid and start reason (no lock?)"
    );
    // CONTROL: another owner's release leaves the lock; the owner's own release removes it.
    assert!(!worktree::release(&exec, &main, ITEM, &owner(9)).expect("release by another"));
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path).as_deref(),
        Some(want.as_str())
    );
    assert!(worktree::release(&exec, &main, ITEM, &me).expect("release"));
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path),
        None,
        "the lock survived its release"
    );
}

/// A lock whose pid is dead is released, and the new run's lock replaces it.
#[test]
fn a_lock_held_by_a_dead_pid_is_released() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "dead", &[]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(111), &|_| None, None).expect("created");
    // pid 111 is dead: the probe reports no start time for it.
    let dead = |pid: u32| {
        if pid == 111 {
            None
        } else {
            Some(u64::from(pid) * 1000)
        }
    };
    let again = worktree::enter(&exec, &main, ITEM, &owner(222), &dead, None)
        .unwrap_or_else(|err| panic!("a dead pid's lock was not released: {err:?}"));
    assert!(
        matches!(again.created, Creation::Reused),
        "{:?}",
        again.created
    );
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path).as_deref(),
        Some(format!("redkiln run {ITEM} (pid 222 start 222000)").as_str()),
        "the dead pid's lock was not replaced by the new run's"
    );
}

/// Pid reuse: a lock whose pid now names a process with a DIFFERENT start time is stale.
#[test]
fn a_lock_whose_pid_now_names_another_process_is_released() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "reused", &[]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(111), &|_| None, None).expect("created");
    // pid 111 is alive, but it started at 5, not at the 111000 the lock records.
    let reused = |pid: u32| {
        if pid == 111 {
            Some(5)
        } else {
            Some(u64::from(pid) * 1000)
        }
    };
    let again =
        worktree::enter(&exec, &main, ITEM, &owner(222), &reused, None).unwrap_or_else(|err| {
            panic!("a lock whose pid now names another process was not released: {err:?}")
        });
    assert!(
        matches!(again.created, Creation::Reused),
        "{:?}",
        again.created
    );
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path).as_deref(),
        Some(format!("redkiln run {ITEM} (pid 222 start 222000)").as_str())
    );
}

/// A live lock (pid alive AND start time equal) refuses the run by name and is left in place; so
/// is a lock redkiln did not write.
#[test]
fn a_lock_held_by_a_live_pid_refuses_by_name() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "live", &[]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(111), &|_| None, None).expect("created");
    std::fs::write(e.tree.path.join("in-flight.txt"), b"the live run's work\n").expect("write");
    let live = |pid: u32| Some(u64::from(pid) * 1000);
    let r = refused(
        "re-entry under a live lock",
        worktree::enter(&exec, &main, ITEM, &owner(222), &live, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::LockHeld { path, pid: 111, start: 111_000 } if same_dir(path, &e.tree.path)),
        "refused for the wrong reason: {r:?}"
    );
    let msg = r.to_string();
    assert!(
        msg.contains("111") && msg.contains(ITEM),
        "not named: {msg}"
    );
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path).as_deref(),
        Some(format!("redkiln run {ITEM} (pid 111 start 111000)").as_str()),
        "a live lock was touched"
    );
    assert!(
        e.tree.path.join("in-flight.txt").exists(),
        "the live run's in-flight work was discarded under its lock"
    );
    // A lock redkiln did not write is never released: refused by name.
    assert!(worktree::release(&exec, &main, ITEM, &owner(111)).expect("release"));
    fx.git(
        &main,
        &[
            "worktree",
            "lock",
            "--reason",
            "operator hold",
            e.tree.path.to_str().expect("utf-8"),
        ],
    );
    let r = refused(
        "re-entry under the operator's own lock",
        worktree::enter(&exec, &main, ITEM, &owner(222), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::ForeignLock { reason, .. } if reason == "operator hold"),
        "refused for the wrong reason: {r:?}"
    );
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path).as_deref(),
        Some("operator hold")
    );
}

/// W2: re-entry REUSES the registered tree when its `HEAD` is symbolic on `rk-run/<item>` at the
/// tip: committed work, ignored files and the marker all survive, and nothing is re-created.
#[test]
fn reentry_reuses_the_registered_tree_when_head_equals_the_run_ref_tip() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "reuse", &["keep.ignored"]);
    let e = created_and_released(&fx, &exec, &main);
    std::fs::write(e.tree.path.join("step.txt"), b"a committed step\n").expect("write");
    fx.git(&e.tree.path, &["add", "step.txt"]);
    fx.git(&e.tree.path, &["commit", "--quiet", "-m", "a step"]);
    std::fs::write(e.tree.path.join("keep.ignored"), b"survives\n").expect("write");
    let tip = sha(&fx, &main, &e.tree.branch);
    let gitdir = gitdir_of(&fx, &e.tree.path);
    let marker = std::fs::read(gitdir.join("RK_BASE")).expect("marker");

    let again = worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None)
        .unwrap_or_else(|err| panic!("the registered tree was not reused: {err:?}"));
    assert!(
        matches!(again.created, Creation::Reused),
        "the registered tree was re-created, not reused: {:?}",
        again.created
    );
    assert!(same_dir(&again.tree.path, &e.tree.path));
    assert!(again.discarded.is_empty(), "{:?}", again.discarded);
    assert_eq!(
        again.fork, e.fork,
        "re-entry read a new fork instead of RK_BASE's"
    );
    assert_eq!(sha(&fx, &main, &e.tree.branch), tip, "the run ref moved");
    assert_eq!(sha(&fx, &e.tree.path, "HEAD"), tip, "the tree's HEAD moved");
    assert!(
        e.tree.path.join("step.txt").exists(),
        "committed work is gone"
    );
    assert!(
        e.tree.path.join("keep.ignored").exists(),
        "an ignored file is gone — the tree was re-created"
    );
    assert!(
        same_dir(&gitdir_of(&fx, &e.tree.path), &gitdir),
        "a new gitdir"
    );
    assert_eq!(
        std::fs::read(gitdir.join("RK_BASE")).expect("marker"),
        marker
    );
}

/// N10's semantics: re-entry reads the in-flight changes path by path, discards them explicitly
/// (`reset --hard HEAD`, then `clean -fd`), and RETURNS every discarded path.
#[test]
fn reentry_discards_in_flight_changes_and_reports_each_path() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "discard", &[]);
    let e = created_and_released(&fx, &exec, &main);
    let t = &e.tree.path;
    std::fs::write(t.join("a.txt"), b"edited in flight\n").expect("modify tracked");
    std::fs::write(t.join("new.txt"), b"untracked\n").expect("untracked");
    std::fs::create_dir_all(t.join("dir")).expect("mkdir");
    std::fs::write(t.join("dir").join("n.txt"), b"untracked, nested\n").expect("nested");
    std::fs::write(t.join("staged.txt"), b"staged\n").expect("staged");
    fx.git(t, &["add", "staged.txt"]);

    let again = worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None).expect("reused");
    let mut got = again.discarded;
    got.sort();
    assert_eq!(
        got,
        vec!["a.txt", "dir/n.txt", "new.txt", "staged.txt"],
        "the discard did not report each path"
    );
    assert_eq!(std::fs::read(t.join("a.txt")).expect("a.txt"), b"one\n");
    for gone in ["new.txt", "dir/n.txt", "staged.txt"] {
        assert!(!t.join(gone).exists(), "{gone} survived the discard");
    }
    assert!(
        fx.git(t, &["status", "--porcelain"])
            .stdout_trimmed()
            .is_empty(),
        "the status is not empty after the discard"
    );
}

/// `clean` runs WITHOUT `-x`: ignored files (W3's carried files, `setup`'s `node_modules`)
/// survive re-entry; untracked ones do not.
#[test]
fn reentry_keeps_ignored_files() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "ignored", &[".env", "node_modules/"]);
    let e = created_and_released(&fx, &exec, &main);
    let t = &e.tree.path;
    std::fs::write(t.join(".env"), b"CARRIED=1\n").expect("carried");
    std::fs::create_dir_all(t.join("node_modules").join("pkg")).expect("mkdir");
    std::fs::write(t.join("node_modules").join("pkg").join("index.js"), b"x\n").expect("dep");
    std::fs::write(t.join("scratch.txt"), b"untracked\n").expect("untracked");

    let again = worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None).expect("reused");
    assert_eq!(again.discarded, vec!["scratch.txt"]);
    assert!(
        t.join(".env").exists(),
        "an ignored carried file did not survive re-entry"
    );
    assert!(
        t.join("node_modules").join("pkg").join("index.js").exists(),
        "setup's ignored node_modules did not survive re-entry"
    );
    assert!(
        !t.join("scratch.txt").exists(),
        "an untracked file survived the discard"
    );
}

/// A tree whose `HEAD` has commits the run ref lacks is refused and kept, never removed.
#[test]
fn a_tree_with_commits_the_run_ref_lacks_is_refused_and_never_removed() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "ahead", &[]);
    let e = created_and_released(&fx, &exec, &main);
    let t = &e.tree.path;
    fx.git(t, &["switch", "--quiet", "-c", "side"]);
    std::fs::write(t.join("side.txt"), b"not on the run ref\n").expect("write");
    fx.git(t, &["add", "side.txt"]);
    fx.git(t, &["commit", "--quiet", "-m", "side work"]);
    let tip_before = sha(&fx, &main, &e.tree.branch);

    let r = refused(
        "re-entry into a tree ahead of the run ref",
        worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::CommitsRunRefLacks { path, count: 1, .. } if same_dir(path, t)),
        "refused for the wrong reason: {r:?}"
    );
    assert!(r.to_string().contains(&e.tree.branch), "not named: {r}");
    assert_kept(&fx, &main, t, "after the refused re-entry");
    assert!(
        t.join("side.txt").exists(),
        "the unlanded commit's file is gone"
    );
    assert_eq!(
        sha(&fx, &main, &e.tree.branch),
        tip_before,
        "the run ref moved"
    );
    match worktree::remove_landed(&exec, &main, ITEM).expect("removal decision") {
        Removal::Kept { .. } => {}
        Removal::Removed => panic!("a tree with commits the run ref lacks was REMOVED"),
    }
    assert_kept(&fx, &main, t, "after the removal decision");
}

/// The re-entry check: a tree at the derived path that is a checkout of ANOTHER repository is
/// refused by name, and kept.
#[test]
fn a_tree_registered_to_another_repository_is_refused() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "ours", &[]);
    let other = lifecycle_repo(&fx, "theirs", &[]);
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");
    let rt = worktree::derive(&root, &ItemId::parse(ITEM).expect("valid id"));
    fx.git(
        &other,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &rt.branch,
            rt.path.to_str().expect("utf-8"),
        ],
    );
    let r = refused(
        "re-entry into another repository's tree",
        worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::ForeignRepository { path, .. } if same_dir(path, &rt.path)),
        "refused for the wrong reason: {r:?}"
    );
    assert!(
        r.to_string().contains("theirs"),
        "the other repository is not named: {r}"
    );
    assert!(rt.path.exists(), "the other repository's tree was removed");
    assert!(
        record_of(&fx, &other, &rt.path).is_some(),
        "the other repository's registration was touched"
    );
    assert!(
        fx.try_git(&main, &["rev-parse", "--verify", "--quiet", &rt.branch])
            .is_err(),
        "a run branch was created in this repository for a tree it refused"
    );
}

/// A detached `HEAD` can equal the tip and still not be on the run ref: refused and kept.
#[test]
fn a_tree_on_a_detached_head_is_refused_at_reentry_and_kept() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "detached", &[]);
    let e = created_and_released(&fx, &exec, &main);
    let t = &e.tree.path;
    fx.git(t, &["switch", "--quiet", "--detach"]);
    assert_eq!(
        sha(&fx, t, "HEAD"),
        sha(&fx, &main, &e.tree.branch),
        "fixture: HEAD is the tip"
    );
    std::fs::write(t.join("in-flight.txt"), b"x\n").expect("write");

    let r = refused(
        "re-entry on a detached HEAD at the tip",
        worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::DetachedHead { path } if same_dir(path, t)),
        "refused for the wrong reason: {r:?}"
    );
    assert!(r.to_string().contains("detached"), "not named: {r}");
    assert_kept(&fx, &main, t, "after the refused re-entry");
    assert!(
        t.join("in-flight.txt").exists(),
        "a refused tree was discarded"
    );
    assert!(
        matches!(
            worktree::remove_landed(&exec, &main, ITEM).expect("decision"),
            Removal::Kept { .. }
        ),
        "a detached tree was removed"
    );
    assert_kept(&fx, &main, t, "after the removal decision");
}

/// The branch exists and no tree is registered: `git worktree add <path> rk-run/<item>`. Never
/// `-B`, which would reset the branch to `HEAD` and destroy a resumable run.
#[test]
fn an_existing_run_branch_is_never_reset() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "branch", &[]);
    let branch = format!("rk-run/{ITEM}");
    fx.git(&main, &["switch", "--quiet", "-c", &branch]);
    std::fs::write(main.join("resumable.txt"), b"a landed step\n").expect("write");
    fx.git(&main, &["add", "resumable.txt"]);
    fx.git(&main, &["commit", "--quiet", "-m", "run work"]);
    let run_tip = sha(&fx, &main, "HEAD");
    fx.git(&main, &["switch", "--quiet", "-"]);
    std::fs::write(main.join("moved.txt"), b"HEAD moved on\n").expect("write");
    fx.git(&main, &["add", "moved.txt"]);
    fx.git(&main, &["commit", "--quiet", "-m", "main moves"]);
    assert_ne!(sha(&fx, &main, "HEAD"), run_tip);

    let e = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None)
        .unwrap_or_else(|err| panic!("no reuse path for an existing run branch: {err:?}"));
    assert!(
        matches!(e.created, Creation::OnExistingBranch),
        "{:?}",
        e.created
    );
    assert_eq!(
        sha(&fx, &main, &branch),
        run_tip,
        "the existing run branch was RESET"
    );
    assert_eq!(sha(&fx, &e.tree.path, "HEAD"), run_tip);
    assert!(e.tree.path.join("resumable.txt").exists());
    assert!(
        !e.tree.path.join("moved.txt").exists(),
        "the tree holds HEAD's content"
    );
    assert_eq!(
        fx.git(&e.tree.path, &["rev-parse", "--symbolic-full-name", "HEAD"])
            .stdout_trimmed(),
        format!("refs/heads/{branch}")
    );
}

/// Removal: a dirty tree is kept, with Claude Code's reason, *"N changed file(s) would be lost"*.
#[test]
fn a_dirty_tree_is_kept_with_its_reason() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "dirty", &[]);
    let e = created_and_released(&fx, &exec, &main);
    close_the_lane(&fx, &e.tree.path);
    std::fs::write(e.tree.path.join("a.txt"), b"changed\n").expect("modify");
    std::fs::write(e.tree.path.join("u.txt"), b"untracked\n").expect("untracked");
    match worktree::remove_landed(&exec, &main, ITEM).expect("removal decision") {
        Removal::Kept { path, reason } => {
            assert!(same_dir(&path, &e.tree.path));
            assert!(
                matches!(reason, KeepReason::Dirty { changed: 2 }),
                "kept for the wrong reason: {reason:?}"
            );
            let msg = reason.to_string();
            assert!(
                msg.contains("2 changed file(s) would be lost"),
                "not Claude Code's form: {msg}"
            );
        }
        Removal::Removed => panic!("a DIRTY tree was removed"),
    }
    assert_kept(&fx, &main, &e.tree.path, "after the removal decision");
    assert_eq!(
        std::fs::read(e.tree.path.join("a.txt")).expect("a.txt"),
        b"changed\n"
    );
}

/// "Landed" is not "`HEAD` equals the tip" (re-entry's own condition, D21-47): a clean, marked,
/// unlocked tree whose tip carries no `closed` mark is kept.
#[test]
fn an_unlanded_tree_is_kept_with_its_reason() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "unlanded", &[]);
    let e = created_and_released(&fx, &exec, &main);
    std::fs::write(e.tree.path.join("step.txt"), b"a step\n").expect("write");
    fx.git(&e.tree.path, &["add", "step.txt"]);
    fx.git(
        &e.tree.path,
        &[
            "commit",
            "--quiet",
            "-m",
            "a step",
            "-m",
            &format!("Story: redkiln-rs/{ITEM} implement"),
        ],
    );
    match worktree::remove_landed(&exec, &main, ITEM).expect("removal decision") {
        Removal::Kept { reason, .. } => {
            assert!(
                matches!(reason, KeepReason::Unlanded { .. }),
                "kept for the wrong reason: {reason:?}"
            );
            assert!(reason.to_string().contains("closed"), "not named: {reason}");
        }
        Removal::Removed => {
            panic!("an UNLANDED tree (its tip carries no closed mark) was removed")
        }
    }
    assert_kept(&fx, &main, &e.tree.path, "after the removal decision");
}

/// A locked tree is kept, whatever its other state.
#[test]
fn a_locked_tree_is_not_removed() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "locked", &[]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None).expect("created");
    close_the_lane(&fx, &e.tree.path);
    match worktree::remove_landed(&exec, &main, ITEM).expect("removal decision") {
        Removal::Kept { reason, .. } => assert!(
            matches!(&reason, KeepReason::Locked { reason } if reason.contains("pid 1 ")),
            "kept for the wrong reason: {reason:?}"
        ),
        Removal::Removed => panic!("a LOCKED tree was removed"),
    }
    assert_kept(&fx, &main, &e.tree.path, "after the removal decision");
    assert!(
        lock_of(&fx, &main, &e.tree.path).is_some(),
        "the lock was released"
    );
}

/// The one removal: clean, landed, marked, unlocked — by `git worktree remove` without
/// `--force`. The branch stays; it is the ref the operator merges.
#[test]
fn a_clean_landed_marked_unlocked_tree_is_removed() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "landed", &["node_modules/"]);
    let e = created_and_released(&fx, &exec, &main);
    close_the_lane(&fx, &e.tree.path);
    std::fs::create_dir_all(e.tree.path.join("node_modules")).expect("mkdir");
    std::fs::write(e.tree.path.join("node_modules").join("dep.js"), b"x\n").expect("ignored");
    let tip = sha(&fx, &main, &e.tree.branch);
    let removal = worktree::remove_landed(&exec, &main, ITEM).expect("removal decision");
    assert!(
        matches!(removal, Removal::Removed),
        "a landed tree was kept: {removal:?}"
    );
    assert!(!e.tree.path.exists(), "the removed tree is still on disk");
    assert!(
        record_of(&fx, &main, &e.tree.path).is_none(),
        "still registered"
    );
    assert_eq!(
        sha(&fx, &main, &e.tree.branch),
        tip,
        "the run branch moved or went"
    );
}

/// W1: Windows removal never follows a junction. An ignored junction inside the tree points at a
/// sentinel directory OUTSIDE it (and a pnpm-shaped one at a directory inside it); the tree is
/// removed, the sentinel's contents survive, and no junction skeleton is left behind (s8c
/// §8c.12 item 3). Measured 2026-09-27 on git 2.55.0.windows.3: git's own unforced `worktree
/// remove` of such a tree exits 0 and deregisters it, but leaves every junction and its parent
/// directories on disk (a pnpm-installed `node_modules`: 114 links, 0 files, 60 directories
/// left). The removal therefore unlinks every untracked link first, never walking through one.
#[test]
fn removal_never_follows_a_junction() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "junction", &["linked", "nm/"]);
    let e = created_and_released(&fx, &exec, &main);
    close_the_lane(&fx, &e.tree.path);
    let sentinel = fx.sub("sentinel");
    std::fs::create_dir_all(&sentinel).expect("mkdir sentinel");
    std::fs::write(sentinel.join("keep.txt"), b"precious\n").expect("write sentinel");
    let kind = dir_link(&e.tree.path.join("linked"), &sentinel);
    let pnpm = e.tree.path.join("nm").join(".pnpm").join("pkg");
    std::fs::create_dir_all(&pnpm).expect("mkdir pnpm store");
    std::fs::write(pnpm.join("index.js"), b"x\n").expect("write dep");
    dir_link(&e.tree.path.join("nm").join("pkg"), &pnpm);
    println!("built a {kind} to a sentinel outside the tree, and one inside it");
    assert!(
        fx.git(&e.tree.path, &["status", "--porcelain"])
            .stdout_trimmed()
            .is_empty(),
        "fixture: the links are not ignored"
    );

    let removal = worktree::remove_landed(&exec, &main, ITEM).expect("removal decision");
    assert_eq!(
        std::fs::read(sentinel.join("keep.txt")).ok().as_deref(),
        Some(&b"precious\n"[..]),
        "the removal FOLLOWED the {kind} and deleted the sentinel's contents outside the tree"
    );
    assert!(matches!(removal, Removal::Removed), "{removal:?}");
    assert!(
        std::fs::symlink_metadata(&e.tree.path).is_err(),
        "a skeleton of the removed tree is left on disk (s8c §8c.12 item 3)"
    );
    assert!(
        record_of(&fx, &main, &e.tree.path).is_none(),
        "still registered"
    );
}

/// **D21-88 ruling 2 (ADR-0180 decisions 9 and 11, proposed): a FRESH tree whose carry or
/// `setup` failed is rolled back** — unlocked from this run's own lock, every untracked link
/// unlinked without being followed (ruling 1's pre-unlink path), then git's unforced `worktree
/// remove`; nothing is left on disk, the registration is gone, and the branch is kept.
/// *(dated 2026-09-27, s8d-ii build, package D2188.)*
///
/// Red while `roll_back_fresh` is a stub that keeps the tree. Red under `S8D-II-M28` (the
/// pre-unlink skipped): git's unforced removal exits 0 and leaves the junctions on disk (D21-88's
/// finding-17 measurement), so the rollback reports the residue and keeps.
#[test]
fn a_fresh_tree_whose_carry_failed_is_rolled_back_with_its_links_unlinked() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "rollback", &["linked", "nm/"]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None).expect("created");
    assert!(
        matches!(e.created, Creation::New),
        "PREMISE: a fresh tree: {:?}",
        e.created
    );
    let sentinel = fx.sub("rb-sentinel");
    std::fs::create_dir_all(&sentinel).expect("mkdir sentinel");
    std::fs::write(sentinel.join("keep.txt"), b"precious\n").expect("write sentinel");
    let kind = dir_link(&e.tree.path.join("linked"), &sentinel);
    let pnpm = e.tree.path.join("nm").join(".pnpm").join("pkg");
    std::fs::create_dir_all(&pnpm).expect("mkdir pnpm store");
    std::fs::write(pnpm.join("index.js"), b"x\n").expect("write dep");
    dir_link(&e.tree.path.join("nm").join("pkg"), &pnpm);
    let tip = sha(&fx, &main, &e.tree.branch);

    let rolled = worktree::roll_back_fresh(&exec, &main, &e, &owner(1)).expect("rollback");
    assert_eq!(
        std::fs::read(sentinel.join("keep.txt")).ok().as_deref(),
        Some(&b"precious\n"[..]),
        "the rollback FOLLOWED the {kind} and deleted the sentinel's contents outside the tree"
    );
    assert!(
        matches!(rolled, worktree::Rollback::Removed),
        "the fresh tree was not rolled back: {rolled:?}"
    );
    assert!(
        std::fs::symlink_metadata(&e.tree.path).is_err(),
        "a residue of the rolled-back tree is left on disk"
    );
    assert!(
        record_of(&fx, &main, &e.tree.path).is_none(),
        "the rolled-back tree is still registered"
    );
    assert_eq!(
        sha(&fx, &main, &e.tree.branch),
        tip,
        "the rollback moved or deleted the run branch; it is kept"
    );
    // The next entry creates it anew, on the existing branch.
    let again = worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None).expect("again");
    assert!(
        matches!(again.created, Creation::OnExistingBranch),
        "after the rollback the next entry did not create the tree anew: {:?}",
        again.created
    );
}

/// **D21-88 ruling 2's other half: a reused tree is never rolled back, nor a fresh tree whose
/// lock is not this run's.** Both are kept, registered, on disk, with their lock as it was.
/// *(dated 2026-09-27, s8d-ii build, package D2188.)*
///
/// Red under `S8D-II-M29` (the `Reused` guard dropped): the reused tree is removed.
#[test]
fn a_reused_tree_or_one_this_run_did_not_lock_is_never_rolled_back() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "noroll", &[]);
    created_and_released(&fx, &exec, &main);
    let reused = worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None).expect("reused");
    assert!(
        matches!(reused.created, Creation::Reused),
        "PREMISE: a reused tree: {:?}",
        reused.created
    );
    match worktree::roll_back_fresh(&exec, &main, &reused, &owner(2)).expect("rollback") {
        worktree::Rollback::Kept { reason, .. } => assert!(
            reason.contains("reused"),
            "a reused tree was kept for the wrong reason: {reason}"
        ),
        worktree::Rollback::Removed => panic!("a REUSED tree was rolled back"),
    }
    assert_kept(&fx, &main, &reused.tree.path, "a reused tree");
    assert!(
        lock_of(&fx, &main, &reused.tree.path).is_some_and(|r| r.contains("pid 2 ")),
        "the reused tree's lock was touched"
    );

    // A fresh-looking Entered whose lock is someone else's: kept, lock untouched.
    let pretend = worktree::Entered {
        created: Creation::New,
        ..reused.clone()
    };
    match worktree::roll_back_fresh(&exec, &main, &pretend, &owner(3)).expect("rollback") {
        worktree::Rollback::Kept { reason, .. } => assert!(
            reason.contains("lock"),
            "a tree this run did not lock was kept for the wrong reason: {reason}"
        ),
        worktree::Rollback::Removed => panic!("a tree locked by ANOTHER run was rolled back"),
    }
    assert_kept(&fx, &main, &reused.tree.path, "a tree another run locked");
    assert!(
        lock_of(&fx, &main, &reused.tree.path).is_some_and(|r| r.contains("pid 2 ")),
        "another run's lock was released"
    );
}

// ---------------------------------------------------------------------------------------------
// s8d-ii §8d-ii.5 and §8d-ii.6 (D21-45 W4; D21-46 R8; D21-47 V1; D21-48 X2; ADR-0180 decisions
// 12 and 13, proposed): the filter-driver refusal, read from the fork commit at every scope, and
// the post-checkout pointer scan. EVERY test here isolates the machine's own configuration:
// `GIT_CONFIG_SYSTEM` and `GIT_CONFIG_GLOBAL` point at fixture files on the wrapper's children,
// and the drivers are portable fixture commands (`tr`), so no test reads this box's `system`
// `filter.lfs` or needs `git-lfs`. *(dated 2026-09-27, s8d-ii build, P4.)*
// ---------------------------------------------------------------------------------------------

/// Pointer bytes: what a Git LFS path's committed blob holds.
const POINTER: &[u8] = b"version https://git-lfs.github.com/spec/v1\noid sha256:0000000000000000000000000000000000000000000000000000000000000000\nsize 1\n";

/// A working fixture driver: smudge upper-cases, clean lower-cases, so a (lower-case) pointer
/// round-trips and its checked-out bytes are no pointer.
const WORKING_FX: &str = "[filter \"fx\"]\n\tsmudge = tr a-z A-Z\n\tclean = tr A-Z a-z\n";

/// Finding 15's driver: effective (a `smudge` value resolves) but its command is not installed,
/// and not `required`, so git keeps the committed content.
const BROKEN_FX: &str = "[filter \"fx\"]\n\tsmudge = rk-s8d-ii-no-such-smudge-command %f\n";

/// A smudge that skips (`--skip`), which yields pointer files.
const SKIPPING_FX: &str = "[filter \"fx\"]\n\tsmudge = git-lfs smudge --skip -- %f\n";

/// The wrapper with `GIT_CONFIG_SYSTEM` and `GIT_CONFIG_GLOBAL` pointed at fixture files holding
/// `system` and `global`, and `GIT_CONFIG_NOSYSTEM` forced off.
fn filter_exec(fx: &crate::support::Fixture, name: &str, system: &str, global: &str) -> GitExec {
    let sys = fx.path().join(format!("{name}-system.cfg"));
    let glo = fx.path().join(format!("{name}-global.cfg"));
    std::fs::write(&sys, system.as_bytes()).expect("write the system fixture");
    std::fs::write(&glo, global.as_bytes()).expect("write the global fixture");
    exec_for(fx)
        .with_env("GIT_CONFIG_SYSTEM", sys.as_os_str())
        .with_env("GIT_CONFIG_GLOBAL", glo.as_os_str())
        .with_env("GIT_CONFIG_NOSYSTEM", "0")
}

/// The same isolation for a raw `git` child: `filter_exec(fx, name, …)`'s fixture files.
fn fixture_env(
    fx: &crate::support::Fixture,
    name: &str,
) -> Vec<(std::ffi::OsString, std::ffi::OsString)> {
    vec![
        (
            "GIT_CONFIG_SYSTEM".into(),
            fx.path()
                .join(format!("{name}-system.cfg"))
                .into_os_string(),
        ),
        (
            "GIT_CONFIG_GLOBAL".into(),
            fx.path()
                .join(format!("{name}-global.cfg"))
                .into_os_string(),
        ),
        ("GIT_CONFIG_NOSYSTEM".into(), "0".into()),
    ]
}

/// `lifecycle_repo`'s shape plus a committed `.gitattributes` and files (committed with no
/// filter configured, so each blob holds the bytes given).
fn filter_repo(
    fx: &crate::support::Fixture,
    name: &str,
    attributes: &str,
    files: &[(&str, &[u8])],
) -> std::path::PathBuf {
    let repo = lifecycle_repo(fx, name, &[]);
    std::fs::write(repo.join(".gitattributes"), attributes.as_bytes()).expect("write attributes");
    fx.git(&repo, &["add", ".gitattributes"]);
    for (path, bytes) in files {
        let p = repo.join(path);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mkdir");
        }
        std::fs::write(&p, bytes).expect("write a fixture file");
        fx.git(&repo, &["add", path]);
    }
    fx.git(&repo, &["commit", "--quiet", "-m", "filter fixture"]);
    repo
}

/// The derived run tree for `ITEM` under `main`.
fn run_tree_of(main: &std::path::Path) -> std::path::PathBuf {
    main.join(".redkiln").join("worktrees").join(ITEM)
}

/// Refused BEFORE `git worktree add`: nothing on disk, nothing registered.
fn assert_not_created(fx: &crate::support::Fixture, main: &std::path::Path, when: &str) {
    let t = run_tree_of(main);
    assert!(
        !t.exists(),
        "{when}: a tree was created at `{}`",
        t.display()
    );
    assert!(
        record_of(fx, main, &t).is_none(),
        "{when}: a tree is registered at `{}`",
        t.display()
    );
}

/// Created, registered, and its `a.bin` (if any) is no pointer.
fn assert_created(
    fx: &crate::support::Fixture,
    main: &std::path::Path,
    r: Result<rk_git::worktree::Entered, GitError>,
    when: &str,
) -> rk_git::worktree::Entered {
    let e = r.unwrap_or_else(|err| panic!("{when}: creation was REFUSED and must not be: {err:?}"));
    assert!(e.tree.path.exists(), "{when}: no tree on disk");
    assert!(
        record_of(fx, main, &e.tree.path).is_some(),
        "{when}: the tree is not registered"
    );
    e
}

/// (a): a selected driver that no scope declares. Git checks out the committed content, which
/// for LFS is the pointer. The diagnosis names the driver, a selecting path and every scope read.
#[test]
fn a_selected_filter_with_no_driver_at_any_scope_refuses_naming_the_path() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "nodrv", "", "");
    let main = filter_repo(
        &fx,
        "nodrv",
        "*.bin filter=nodrv\n",
        &[("assets/a.bin", POINTER)],
    );
    let refusal = refused(
        "a driver selected at the fork commit and declared at no scope",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    let text = refusal.to_string();
    assert!(
        matches!(&refusal, WorktreeRefusal::FilterDriverNotEffective { driver, path, .. }
            if driver == "nodrv" && path == "assets/a.bin"),
        "not the no-effective-driver refusal naming `nodrv` and `assets/a.bin`: {refusal:?}"
    );
    for scope in ["system", "global", "local", "worktree", "command"] {
        assert!(
            text.contains(scope),
            "the diagnosis does not name the `{scope}` scope it read: {text}"
        );
    }
    assert_not_created(&fx, &main, "after the refusal");
}

/// (b) at `local` scope: the resolved `smudge` carries `--skip`, set in the repository's own
/// config. The diagnosis names the scope and its origin.
#[test]
fn a_smudge_skipped_at_local_scope_refuses_naming_the_scope() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "skiplocal", "", "");
    let main = filter_repo(&fx, "skiplocal", "*.bin filter=fx\n", &[("a.bin", POINTER)]);
    fx.git(
        &main,
        &["config", "filter.fx.smudge", "git-lfs smudge --skip -- %f"],
    );
    assert_smudge_skipped(&fx, &exec, &main, "local", ".git/config", None);
}

/// (b) at `global` scope (`GIT_CONFIG_GLOBAL` at a fixture file).
#[test]
fn a_smudge_skipped_at_global_scope_refuses_naming_the_scope() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "skipglobal", "", SKIPPING_FX);
    let main = filter_repo(
        &fx,
        "skipglobal",
        "*.bin filter=fx\n",
        &[("a.bin", POINTER)],
    );
    assert_smudge_skipped(&fx, &exec, &main, "global", "skipglobal-global.cfg", None);
}

/// (b) at `system` scope (`GIT_CONFIG_SYSTEM` at a fixture file). A `--local` read would not see
/// the driver at all (`S8D-II-M5`).
#[test]
fn a_smudge_skipped_at_system_scope_refuses_naming_the_scope() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "skipsystem", SKIPPING_FX, "");
    let main = filter_repo(
        &fx,
        "skipsystem",
        "*.bin filter=fx\n",
        &[("a.bin", POINTER)],
    );
    assert_smudge_skipped(&fx, &exec, &main, "system", "skipsystem-system.cfg", None);
}

/// Asserts the refusal is `SmudgeSkipped` for driver `fx` and path `a.bin` at `scope`, whose
/// origin ends with `origin_tail`, and that nothing was created.
fn assert_smudge_skipped(
    fx: &crate::support::Fixture,
    exec: &GitExec,
    main: &std::path::Path,
    scope: &str,
    origin_tail: &str,
    env: Option<&str>,
) {
    let refusal = refused(
        &format!("a smudge skipped at {scope} scope"),
        worktree::enter(exec, main, ITEM, &owner(1), &|_| None, env),
    );
    let text = refusal.to_string();
    match &refusal {
        // `..`: the refusal now also names the commit read (R2FIX-8dii, dated 2026-09-28).
        WorktreeRefusal::SmudgeSkipped {
            driver,
            path,
            scope: s,
            origin,
            ..
        } => {
            assert_eq!((driver.as_str(), path.as_str()), ("fx", "a.bin"), "{text}");
            assert_eq!(s, scope, "the refusal names the wrong scope: {text}");
            assert!(
                origin.replace('\\', "/").ends_with(origin_tail),
                "the refusal's origin `{origin}` is not `…{origin_tail}`: {text}"
            );
        }
        other => panic!("not the skipped-smudge refusal at {scope} scope: {other:?} ({text})"),
    }
    assert!(
        text.contains(scope) && text.contains("fx") && text.contains("a.bin"),
        "the diagnosis does not name the driver, the path and the scope: {text}"
    );
    assert_not_created(fx, main, "after the refusal");
}

/// (b) by the environment: `GIT_LFS_SKIP_SMUDGE`, which `rk-cli` reads through `env.rs` and passes
/// in, set to a value other than `0`. The same working driver with `0` creates.
#[test]
fn git_lfs_skip_smudge_in_the_environment_refuses_by_name() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "skipenv", WORKING_FX, "");
    let main = filter_repo(&fx, "skipenv", "*.bin filter=fx\n", &[("a.bin", POINTER)]);
    let refusal = refused(
        "GIT_LFS_SKIP_SMUDGE=1",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, Some("1")),
    );
    let text = refusal.to_string();
    assert!(
        matches!(&refusal, WorktreeRefusal::SmudgeSkipped { driver, origin, .. }
            if driver == "fx" && origin.contains("GIT_LFS_SKIP_SMUDGE")),
        "not the skipped-smudge refusal naming GIT_LFS_SKIP_SMUDGE: {refusal:?}"
    );
    assert!(text.contains("GIT_LFS_SKIP_SMUDGE"), "{text}");
    assert_not_created(&fx, &main, "after the refusal");
    // `0` skips nothing: the same repository and driver create.
    let e = assert_created(
        &fx,
        &main,
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, Some("0")),
        "GIT_LFS_SKIP_SMUDGE=0",
    );
    let bytes = std::fs::read(e.tree.path.join("a.bin")).expect("read a.bin");
    assert!(
        !bytes.starts_with(b"version https://git-lfs"),
        "the working driver did not smudge"
    );
    // The admission is a fact for the DEBUG level only (§8d-ii.5), and it names the scope.
    assert!(
        e.filter_notes
            .iter()
            .any(|n| n.contains("`fx`") && n.contains("system")),
        "the admission is not recorded for the debug level: {:?}",
        e.filter_notes
    );
}

/// Read (3): an `includeIf` whose condition matches the RUN TREE (its gitdir,
/// `<common>/worktrees/<item>`, or its branch `rk-run/<item>`) and not the user's checkout,
/// folded into the driver read. Three cases: a `gitdir:` and an `onbranch:` condition at
/// `system` scope, and an `onbranch:` condition at `local` scope whose relative path resolves
/// against `.git/config`'s directory (git prints that origin relative to the checkout).
#[test]
fn an_include_if_that_skips_the_smudge_for_the_run_tree_refuses_naming_the_included_file() {
    let fx = crate::support::Fixture::new();
    for (name, condition, scope) in [
        ("incgitdir", "gitdir:**/.git/worktrees/**", "system"),
        ("inconbranch", "onbranch:rk-run/**", "system"),
        ("inclocal", "onbranch:rk-run/**", "local"),
    ] {
        let included = format!("{name}-skip.cfg");
        let system = if scope == "system" {
            std::fs::write(fx.path().join(&included), SKIPPING_FX.as_bytes()).expect("write");
            format!("{WORKING_FX}[includeIf \"{condition}\"]\n\tpath = {included}\n")
        } else {
            WORKING_FX.to_string()
        };
        let exec = filter_exec(&fx, name, &system, "");
        let main = filter_repo(&fx, name, "*.bin filter=fx\n", &[("a.bin", POINTER)]);
        if scope == "local" {
            std::fs::write(main.join(".git").join(&included), SKIPPING_FX.as_bytes())
                .expect("write");
            fx.git(
                &main,
                &["config", &format!("includeIf.{condition}.path"), &included],
            );
        }
        // The user's checkout does NOT see the skip: the condition matches the run tree only.
        let here = rk_git::git_exec::run_with_env(
            &main,
            ["config", "--get", "filter.fx.smudge"],
            fixture_env(&fx, name),
        )
        .expect("read the smudge in the user's checkout")
        .stdout_trimmed();
        assert_eq!(
            here, "tr a-z A-Z",
            "`{condition}` matches the user's checkout too, so this is not a run-tree-only include"
        );
        let refusal = refused(
            &format!("an includeIf `{condition}` skipping the smudge for the run tree"),
            worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
        );
        let text = refusal.to_string();
        match &refusal {
            WorktreeRefusal::SmudgeSkipped {
                driver,
                scope: s,
                origin,
                ..
            } => {
                assert_eq!(driver, "fx", "{text}");
                assert_eq!(s, scope, "{text}");
                assert!(
                    origin.replace('\\', "/").ends_with(&included),
                    "`{condition}`: the refusal does not name the included file `{included}`: {text}"
                );
            }
            other => panic!("`{condition}`: not the skipped-smudge refusal: {other:?} ({text})"),
        }
        assert_not_created(&fx, &main, condition);
    }
}

/// V1: *"Attributes are read from the fork commit."* The fork selects a driver no scope
/// declares; the working tree's `.gitattributes` has been edited to select nothing.
#[test]
fn the_filter_attribute_is_read_from_the_fork_commit() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "fork", "", "");
    let main = filter_repo(&fx, "fork", "*.bin filter=nodrv\n", &[("a.bin", POINTER)]);
    std::fs::write(main.join(".gitattributes"), b"# nothing selected here\n").expect("write");
    let refusal = refused(
        "the fork commit selects `nodrv`; the working tree selects nothing",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    assert!(
        matches!(&refusal, WorktreeRefusal::FilterDriverNotEffective { driver, path, .. }
            if driver == "nodrv" && path == "a.bin"),
        "the attribute was not read from the fork commit: {refusal:?}"
    );
    assert_not_created(&fx, &main, "after the refusal");
}

/// BrittonGames' shape (V1; `s8i` §8i.0 item 12): a tracked path selects `lfs`, and `lfs` is
/// effective at `system` scope, `required`. Admitted silently: the fact goes to the debug notes.
#[test]
fn a_selected_filter_with_a_driver_effective_at_system_scope_creates_silently() {
    let fx = crate::support::Fixture::new();
    let system =
        "[filter \"lfs\"]\n\tsmudge = tr a-z A-Z\n\tclean = tr A-Z a-z\n\trequired = true\n";
    let exec = filter_exec(&fx, "britton", system, "");
    let main = filter_repo(&fx, "britton", "*.bin filter=lfs\n", &[("a.bin", POINTER)]);
    let e = assert_created(
        &fx,
        &main,
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
        "a driver effective at system scope",
    );
    println!("debug-level notes: {:?}", e.filter_notes);
}

/// This box's own shape: `filter.lfs.*` declared at `system` scope and selected by no path.
#[test]
fn a_filter_declared_but_selected_by_no_path_does_not_refuse() {
    let fx = crate::support::Fixture::new();
    let system = "[filter \"lfs\"]\n\tclean = git-lfs clean -- %f\n\tsmudge = git-lfs smudge -- %f\n\tprocess = git-lfs filter-process\n\trequired = true\n";
    let exec = filter_exec(&fx, "declared", system, "");
    let main = filter_repo(&fx, "declared", "*.txt text\n", &[("b.txt", b"plain\n")]);
    assert_created(
        &fx,
        &main,
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
        "a declared driver no path selects",
    );
}

/// The empty case: no driver at any scope, no `includeIf`, no attribute.
#[test]
fn a_repo_with_no_filter_or_include_if_creates() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "empty", "", "");
    let main = lifecycle_repo(&fx, "empty", &[]);
    assert_created(
        &fx,
        &main,
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
        "no filter anywhere",
    );
}

/// D21-48 X2, finding 15's shape: the driver is effective (so the read admits it) but its command
/// is not installed and it is not `required`, so the checkout keeps the pointer. The scan refuses
/// by name, removes the fresh tree unforced, and keeps `rk-run/<item>` at the fork sha.
#[test]
fn a_pointer_file_at_a_selected_path_after_checkout_refuses_by_name_and_removes_the_fresh_tree() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "pointer", BROKEN_FX, "");
    let main = filter_repo(&fx, "pointer", "*.bin filter=fx\n", &[("a.bin", POINTER)]);
    let fork = sha(&fx, &main, "HEAD");
    // First: the configuration read ADMITS the driver ...
    let check =
        worktree::check_filters(&exec, &main, &ItemId::parse(ITEM).expect("id"), &fork, None)
            .unwrap_or_else(|err| {
                panic!("the configuration read refused, so the scan is not what refuses: {err:?}")
            });
    assert!(
        check
            .selected
            .iter()
            .any(|d| d.driver == "fx" && d.effective.is_some()),
        "the read does not see `fx` as selected and effective: {check:?}"
    );
    // ... and `git worktree add` with this configuration exits 0 and leaves a pointer.
    let probe = fx.sub("pointer-probe-tree");
    let probe_s = probe.to_str().expect("utf-8").to_string();
    let added = rk_git::git_exec::run_with_env(
        &main,
        [
            "worktree",
            "add",
            "--quiet",
            "--detach",
            probe_s.as_str(),
            fork.as_str(),
        ],
        fixture_env(&fx, "pointer"),
    );
    assert!(added.is_ok(), "git worktree add did not exit 0: {added:?}");
    assert!(
        std::fs::read(probe.join("a.bin"))
            .expect("read")
            .starts_with(b"version https://git-lfs.github.com/spec/v1"),
        "the probe checkout holds no pointer, so the fixture is not finding 15's shape"
    );
    fx.git(&main, &["worktree", "remove", probe_s.as_str()]);

    let refusal = refused(
        "a pointer file at a selected path after checkout",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    let text = refusal.to_string();
    match &refusal {
        WorktreeRefusal::PointerFilesAfterCheckout(h) => {
            assert_eq!(
                (h.driver.as_str(), h.hits, h.first.as_str()),
                ("fx", 1, "a.bin"),
                "{text}"
            );
            assert_eq!(h.scope, "system", "{text}");
            assert!(
                h.origin.replace('\\', "/").ends_with("pointer-system.cfg"),
                "{text}"
            );
            assert!(
                h.removed,
                "the refusal says the fresh tree was kept: {text}"
            );
        }
        other => panic!("not the post-checkout pointer refusal: {other:?} ({text})"),
    }
    assert!(text.contains("a.bin") && text.contains("fx"), "{text}");
    let t = run_tree_of(&main);
    assert!(
        !t.exists(),
        "the fresh tree `{}` is still on disk after the refusal",
        t.display()
    );
    assert!(
        record_of(&fx, &main, &t).is_none(),
        "the fresh tree is still registered after the refusal"
    );
    assert_eq!(
        sha(&fx, &main, &format!("rk-run/{ITEM}")),
        fork,
        "rk-run/{ITEM} does not stand at the fork sha"
    );
}

/// D21-48 X2: the scan never judges a tree this call did not create. A registered, marked tree
/// whose selected path holds pointer bytes is re-entered: reused and kept, nothing removed.
#[test]
fn the_pointer_scan_never_removes_a_tree_this_call_did_not_create() {
    let fx = crate::support::Fixture::new();
    let working = filter_exec(&fx, "reuse-ok", WORKING_FX, "");
    let main = filter_repo(&fx, "reuse", "*.bin filter=fx\n", &[("a.bin", POINTER)]);
    let e = worktree::enter(&working, &main, ITEM, &owner(1), &|_| None, None)
        .unwrap_or_else(|err| panic!("the first creation was refused: {err:?}"));
    assert!(worktree::release(&working, &main, ITEM, &owner(1)).expect("release"));
    // The selected path now holds pointer bytes, which clean to the committed blob: not dirty.
    std::fs::write(e.tree.path.join("a.bin"), POINTER).expect("write pointer bytes");
    let broken = filter_exec(&fx, "reuse-broken", BROKEN_FX, "");
    let again = worktree::enter(&broken, &main, ITEM, &owner(2), &|_| None, None)
        .unwrap_or_else(|err| panic!("re-entry was refused: {err:?}"));
    assert!(
        matches!(again.created, Creation::Reused),
        "the registered tree was not reused: {:?}",
        again.created
    );
    assert_kept(&fx, &main, &e.tree.path, "after re-entry");
    assert_eq!(
        std::fs::read(e.tree.path.join("a.bin")).expect("read"),
        POINTER,
        "the re-entered tree's pointer bytes were touched"
    );
}

/// D21-48 X2: a pointer header at a path NO filter selects is not a hit (a document quoting the
/// spec is no pointer file).
#[test]
fn a_pointer_header_at_a_path_no_filter_selects_does_not_refuse() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "unselected", WORKING_FX, "");
    let main = filter_repo(
        &fx,
        "unselected",
        "*.bin filter=fx\n",
        &[("a.bin", POINTER), ("docs/lfs-spec.txt", POINTER)],
    );
    let e = assert_created(
        &fx,
        &main,
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
        "a pointer header at an unselected path",
    );
    assert!(
        std::fs::read(e.tree.path.join("docs/lfs-spec.txt"))
            .expect("read")
            .starts_with(b"version https://git-lfs.github.com/spec/v1"),
        "the unselected document does not hold the header, so this control observes nothing"
    );
}

/// D21-48 X2: the scan reads the CHECKED-OUT file, not the committed blob. A committed pointer a
/// working driver smudges is no hit.
#[test]
fn a_committed_pointer_that_the_driver_smudges_creates_silently() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "smudged", WORKING_FX, "");
    let main = filter_repo(&fx, "smudged", "*.bin filter=fx\n", &[("a.bin", POINTER)]);
    assert_eq!(
        fx.git(&main, &["cat-file", "-p", "HEAD:a.bin"]).stdout,
        POINTER,
        "the committed blob is not the pointer"
    );
    let e = assert_created(
        &fx,
        &main,
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
        "a committed pointer the driver smudges",
    );
    assert!(
        !std::fs::read(e.tree.path.join("a.bin"))
            .expect("read")
            .starts_with(b"version https://git-lfs"),
        "the driver did not smudge the checkout"
    );
}

// ---------------------------------------------------------------------------------------------
// s8d-ii round 1, D21-89 (the operator, 2026-09-28): *"On the existing-branch path: (1) the filter
// read, the pointer scan's path set and the length rule read the **commit being checked out** (the
// run branch's tip, read before the add); (2) `RK_BASE` records **`merge-base(start HEAD,
// run-branch tip)`** — the point the branch forked. First creation is unchanged (fork = start's
// HEAD)."* And two round-1 minors: a re-entry whose discard is incomplete releases the run's own
// lock, and read (3) matches `gitdir:` against the admin dir git will assign (git suffixes a
// basename collision). ADR-0180, proposed. *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)*
// ---------------------------------------------------------------------------------------------

/// A repository whose `rk-run/<ITEM>` branch forks at `fork`, carries ~~one commit (`tip`)~~ two
/// commits, the first adding `attributes` and `files` (committed with no filter configured, so
/// each blob holds the bytes given) and the second (`tip`) a plain file, and whose checkout then
/// moves on ~~to a third commit~~ by two commits (`head`) that select nothing. So the fork is
/// neither `tip^` nor `head^` *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii; round-2
/// minor)*. No tree is registered: the existing-branch arm (`Creation::OnExistingBranch`) is the
/// one taken. Returns `(main, fork, tip, head)`.
fn run_branch_ahead(
    fx: &crate::support::Fixture,
    name: &str,
    attributes: &str,
    files: &[(&str, &[u8])],
) -> (std::path::PathBuf, String, String, String) {
    let main = lifecycle_repo(fx, name, &[]);
    let fork = sha(fx, &main, "HEAD");
    fx.git(
        &main,
        &["switch", "--quiet", "-c", &format!("rk-run/{ITEM}")],
    );
    if !attributes.is_empty() {
        std::fs::write(main.join(".gitattributes"), attributes.as_bytes())
            .expect("write attributes");
        fx.git(&main, &["add", ".gitattributes"]);
    }
    for (path, bytes) in files {
        let p = main.join(path);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("mkdir");
        }
        std::fs::write(&p, bytes).expect("write a fixture file");
        fx.git(&main, &["add", path]);
    }
    fx.git(
        &main,
        &[
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            "run work on the branch",
        ],
    );
    // A second commit on each side, so the fork is neither `tip^` nor `head^` (round-2 minor:
    // with one commit a side, a fork read as either parent passed as the merge-base).
    // *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)*
    std::fs::write(main.join("run-2.txt"), b"more run work\n").expect("write");
    fx.git(&main, &["add", "run-2.txt"]);
    fx.git(
        &main,
        &["commit", "--quiet", "-m", "more run work on the branch"],
    );
    let tip = sha(fx, &main, "HEAD");
    fx.git(&main, &["switch", "--quiet", "-"]);
    for (n, file) in ["moved.txt", "moved-2.txt"].iter().enumerate() {
        std::fs::write(main.join(file), b"HEAD moved on\n").expect("write");
        fx.git(&main, &["add", file]);
        fx.git(
            &main,
            &[
                "commit",
                "--quiet",
                "-m",
                &format!("the checkout moves on ({})", n + 1),
            ],
        );
    }
    let head = sha(fx, &main, "HEAD");
    assert!(
        fork != tip && tip != head && fork != head,
        "PREMISE: fork, tip and HEAD are three commits"
    );
    let tip_parent = sha(fx, &main, &format!("{tip}^"));
    let head_parent = sha(fx, &main, &format!("{head}^"));
    assert!(
        fork != tip_parent && fork != head_parent,
        "PREMISE: the fork is `tip^` or `head^`, so a parent read passes as the merge-base"
    );
    (main, fork, tip, head)
}

/// D21-89 ruling 1, V1 on the existing-branch arm: the run branch's TIP selects a driver no scope
/// declares, and start's `HEAD` selects nothing. Creation refuses before the add, naming the tip's
/// path, and the branch is untouched.
///
/// Red while the read runs over start's `HEAD`: the tree is created on the branch with a pointer
/// file in it and no refusal (round 1's probe, `R1SPEC tree holds a.bin pointer: true`).
#[test]
fn an_existing_run_branch_whose_tip_selects_an_ineffective_driver_refuses() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "tipnodrv", "", "");
    let (main, _fork, tip, head) = run_branch_ahead(
        &fx,
        "tipnodrv",
        "*.bin filter=nodrv\n",
        &[("assets/a.bin", POINTER)],
    );
    let id = ItemId::parse(ITEM).expect("id");
    // PREMISE: start's HEAD selects nothing, so only a read of the tip can refuse.
    let at_head = worktree::check_filters(&exec, &main, &id, &head, None)
        .unwrap_or_else(|e| panic!("PREMISE: the read over start's HEAD refused: {e:?}"));
    assert!(
        at_head.selected.is_empty(),
        "PREMISE: start's HEAD selects a driver: {at_head:?}"
    );
    let refusal = refused(
        "the tip selects `nodrv`, declared at no scope; start's HEAD selects nothing",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    assert!(
        matches!(&refusal, WorktreeRefusal::FilterDriverNotEffective { driver, path, .. }
            if driver == "nodrv" && path == "assets/a.bin"),
        "not the no-effective-driver refusal naming the tip's `nodrv` and `assets/a.bin`: {refusal:?}"
    );
    assert_not_created(&fx, &main, "after the refusal");
    assert_eq!(
        sha(&fx, &main, &format!("rk-run/{ITEM}")),
        tip,
        "the run branch moved"
    );
}

/// D21-89 ruling 1, X2 on the existing-branch arm: the TIP selects a driver that is effective (so
/// the read admits it) whose command does not exist, and `a.bin` at the tip holds pointer bytes.
/// The scan reads the tip's path set, refuses by name and rolls the fresh tree back, keeping the
/// branch at its tip.
///
/// Red while the path set is start's `HEAD`'s (which selects nothing): the tree is created holding
/// the pointer file.
#[test]
fn an_existing_run_branch_whose_tip_leaves_a_pointer_is_scanned_refused_and_rolled_back() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "tipptr", BROKEN_FX, "");
    let (main, _fork, tip, head) =
        run_branch_ahead(&fx, "tipptr", "*.bin filter=fx\n", &[("a.bin", POINTER)]);
    let id = ItemId::parse(ITEM).expect("id");
    let at_head = worktree::check_filters(&exec, &main, &id, &head, None)
        .unwrap_or_else(|e| panic!("PREMISE: the read over start's HEAD refused: {e:?}"));
    assert!(
        at_head.selected.is_empty(),
        "PREMISE: start's HEAD selects a driver: {at_head:?}"
    );
    // PREMISE: at the tip the read ADMITS `fx` (effective), so the scan, not the read, refuses.
    let at_tip = worktree::check_filters(&exec, &main, &id, &tip, None).unwrap_or_else(|e| {
        panic!("PREMISE: the read over the tip refused, so the scan is not what refuses: {e:?}")
    });
    assert!(
        at_tip
            .selected
            .iter()
            .any(|d| d.driver == "fx" && d.effective.is_some()),
        "PREMISE: the read does not see `fx` selected and effective at the tip: {at_tip:?}"
    );
    let refusal = refused(
        "a pointer file at a path the tip selects",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    let text = refusal.to_string();
    match &refusal {
        WorktreeRefusal::PointerFilesAfterCheckout(h) => {
            assert_eq!(
                (h.driver.as_str(), h.hits, h.first.as_str()),
                ("fx", 1, "a.bin"),
                "{text}"
            );
            assert!(
                h.removed,
                "the refusal says the fresh tree was kept: {text}"
            );
        }
        other => {
            panic!("not the post-checkout pointer refusal over the tip's paths: {other:?} ({text})")
        }
    }
    let t = run_tree_of(&main);
    assert!(
        !t.exists(),
        "the fresh tree `{}` is still on disk",
        t.display()
    );
    assert!(
        record_of(&fx, &main, &t).is_none(),
        "the fresh tree is still registered after the refusal"
    );
    assert_eq!(
        sha(&fx, &main, &format!("rk-run/{ITEM}")),
        tip,
        "the run branch does not stand at its tip"
    );
}

/// D21-89 ruling 1, the length rule on the existing-branch arm: the TIP tracks a path that makes
/// the worst case exactly 260 (sized from the formula's literal terms), and start's `HEAD` tracks
/// nothing long. Creation refuses by name before the add, `bound: 260`.
///
/// Red while the rule reads start's `HEAD`: admitted (the checkout itself fits, 11 + 1 short of
/// the bound, but its review reference tree would not).
#[test]
fn an_existing_run_branch_whose_tip_is_past_the_path_limit_refuses() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "tiplong", &[]);
    let branch = format!("rk-run/{ITEM}");
    fx.git(&main, &["switch", "--quiet", "-c", &branch]);
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");
    let long = commit_path_of_len(&fx, &main, 260 - fixed_terms(&root, ITEM.len()));
    let tip = sha(&fx, &main, "HEAD");
    fx.git(&main, &["checkout", "--quiet", "-f", "-"]);
    std::fs::write(main.join("moved.txt"), b"HEAD moved on\n").expect("write");
    fx.git(&main, &["add", "moved.txt"]);
    fx.git(&main, &["commit", "--quiet", "-m", "the checkout moves on"]);
    assert!(
        fx.git(&main, &["ls-tree", "-r", "--name-only", "HEAD"])
            .stdout_trimmed()
            .lines()
            .all(|p| p.len() < 20),
        "PREMISE: start's HEAD tracks a long path"
    );
    let r = refused(
        "the tip's worst case is 260; start's HEAD's is short",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::PathPastLimit { bound: 260, limit: 259, longest, longest_path, .. }
            if *longest == long.len() && *longest_path == long),
        "not the length refusal over the tip's longest path: {r:?}"
    );
    assert_not_created(&fx, &main, "after the refusal");
    assert_eq!(sha(&fx, &main, &branch), tip, "the run branch moved");
}

/// D21-89 ruling 2: a tree created on an existing run branch records `merge-base(start HEAD, tip)`
/// in `RK_BASE`, the point the branch forked, and `Entered::fork` agrees: neither start's moved
/// `HEAD` nor the tip. A first creation still records start's `HEAD` (the control).
///
/// Red while `RK_BASE` records start's `HEAD` (the pre-ruling form, §8d-ii.8's P3 note).
#[test]
fn a_tree_created_on_an_existing_run_branch_records_the_merge_base_as_its_fork() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let (main, fork, tip, head) =
        run_branch_ahead(&fx, "mbase", "", &[("run.txt", &b"run work\n"[..])]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None)
        .unwrap_or_else(|err| panic!("creation on the existing branch was refused: {err:?}"));
    assert!(
        matches!(e.created, Creation::OnExistingBranch),
        "PREMISE: the existing-branch arm: {:?}",
        e.created
    );
    let base = worktree::read_rk_base(&exec, &e.tree.path)
        .expect("read RK_BASE")
        .expect("RK_BASE present");
    assert_eq!(
        base.fork, fork,
        "RK_BASE records {} and not the merge-base {fork} of start's HEAD {head} and the tip {tip} \
         (D21-89 ruling 2)",
        base.fork
    );
    assert_eq!(e.fork, fork, "Entered::fork is not the merge-base");
    // RK_BASE is `git merge-base` as the fixture reads it, and neither parent (round-2 minor).
    // *(dated 2026-09-28, s8d-ii build, package R2FIX-8dii.)*
    let read = fx.git(&main, &["merge-base", &head, &tip]).stdout_trimmed();
    assert_eq!(
        base.fork, read,
        "RK_BASE is not the fixture's own `git merge-base {head} {tip}`"
    );
    for parent in [format!("{tip}^"), format!("{head}^")] {
        assert_ne!(
            base.fork,
            sha(&fx, &main, &parent),
            "RK_BASE records `{parent}`, not the merge-base"
        );
    }
    assert_eq!(
        sha(&fx, &e.tree.path, "HEAD"),
        tip,
        "the tree is not at the tip"
    );
    // CONTROL: a first creation (no branch) still records start's HEAD.
    let first = lifecycle_repo(&fx, "mbase-first", &[]);
    let first_head = sha(&fx, &first, "HEAD");
    let f = worktree::enter(&exec, &first, ITEM, &owner(1), &|_| None, None).expect("first");
    assert!(matches!(f.created, Creation::New), "{:?}", f.created);
    assert_eq!(
        f.fork, first_head,
        "a first creation's fork is not start's HEAD"
    );
}

// ---------------------------------------------------------------------------------------------
// s8d-ii refuting round 2 (package R2FIX-8dii): on the existing-branch arm the run tree checks out
// the tip, but the review's reference tree (`<item>.review-ref`, `rk-cli`'s `reference_worktree`)
// checks out `RK_BASE`'s fork, the merge-base. The length rule's longest path is the longer of the
// two commits', and V1's filter read runs over the merge-base too, a refusal naming that commit.
// ADR-0180 decisions 9, 12 and 16, proposed. *(dated 2026-09-28, s8d-ii build.)*
// ---------------------------------------------------------------------------------------------

/// A repository state in which `main`'s commit `fork` tracks the paths `at_fork` commits, and
/// both sides then drop them: start's checkout commits the removal (`head`), and `rk-run/<ITEM>`
/// carries a separate removal commit (`tip`, built with `commit-tree`, so the working tree never
/// checks the fork's paths out again). Returns `(fork, tip, head)`; `merge-base(head, tip)` is
/// `fork`, asserted.
fn merge_base_only(
    fx: &crate::support::Fixture,
    main: &std::path::Path,
    at_fork: impl FnOnce() -> Vec<String>,
) -> (String, String, String) {
    let paths = at_fork();
    let fork = sha(fx, main, "HEAD");
    for p in &paths {
        fx.git(main, &["rm", "--cached", "--quiet", "--", p]);
        let _ = std::fs::remove_file(main.join(p));
    }
    fx.git(
        main,
        &[
            "commit",
            "--quiet",
            "-m",
            "the checkout drops the fork's paths",
        ],
    );
    let head = sha(fx, main, "HEAD");
    let tree = sha(fx, main, "HEAD:");
    let tip = fx
        .git(
            main,
            &[
                "commit-tree",
                &tree,
                "-p",
                &fork,
                "-m",
                "the run branch drops them too",
            ],
        )
        .stdout_trimmed();
    fx.git(
        main,
        &["update-ref", &format!("refs/heads/rk-run/{ITEM}"), &tip],
    );
    assert_eq!(
        fx.git(main, &["merge-base", &head, &tip]).stdout_trimmed(),
        fork,
        "PREMISE: the merge-base of start's HEAD and the tip is not the fork"
    );
    assert!(tip != head && tip != fork, "PREMISE: three commits");
    (fork, tip, head)
}

/// Round-2 minor, the length rule: the MERGE-BASE (the reference tree's commit) tracks a path that
/// makes the worst case exactly 260 (sized from the formula's literal terms); the tip and start's
/// `HEAD` track nothing long. Creation on the existing branch refuses by name before the add,
/// naming the merge-base.
///
/// Red while the rule reads the tip only: admitted, and the reference tree at the merge-base would
/// pass the limit.
#[test]
fn an_existing_run_branch_whose_merge_base_is_past_the_path_limit_refuses() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "mblong", &[]);
    let root = worktree::main_worktree(&exec, &main).expect("main worktree");
    let mut long = String::new();
    let (fork, tip, head) = merge_base_only(&fx, &main, || {
        long = commit_path_of_len(&fx, &main, 260 - fixed_terms(&root, ITEM.len()));
        vec![long.clone()]
    });
    for at in [&tip, &head] {
        assert!(
            fx.git(&main, &["ls-tree", "-r", "--name-only", at])
                .stdout_trimmed()
                .lines()
                .all(|p| p.len() < 20),
            "PREMISE: {at} tracks a long path"
        );
    }
    let r = refused(
        "the merge-base's worst case is 260; the tip's and start's HEAD's are short",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::PathPastLimit { bound: 260, limit: 259, longest, longest_path, commit, .. }
            if *longest == long.len() && *longest_path == long && *commit == fork),
        "not the length refusal over the merge-base {fork}'s longest path: {r:?}"
    );
    assert!(
        r.to_string().contains(&fork),
        "the refusal does not name {fork}: {r}"
    );
    assert_not_created(&fx, &main, "after the refusal");
    assert_eq!(
        sha(&fx, &main, &format!("rk-run/{ITEM}")),
        tip,
        "the run branch moved"
    );

    // CONTROL: core.longpaths=true admits the same topology, and the fork is the merge-base.
    fx.git(&main, &["config", "core.longpaths", "true"]);
    let e = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None)
        .unwrap_or_else(|err| panic!("CONTROL: core.longpaths=true did not admit: {err:?}"));
    assert_eq!(e.fork, fork, "CONTROL: the fork is not the merge-base");
}

/// Round-2 minor, V1 over the reference tree's commit: the MERGE-BASE selects a driver no scope
/// declares; the tip and start's `HEAD` select nothing. Creation on the existing branch refuses
/// before the add, naming the driver, the path and the merge-base.
///
/// Red while the read runs over the tip only: admitted, and the reference tree at the merge-base
/// would hold the pointer.
#[test]
fn an_existing_run_branch_whose_merge_base_selects_an_ineffective_driver_refuses() {
    let fx = crate::support::Fixture::new();
    let exec = filter_exec(&fx, "mbnodrv", "", "");
    let main = lifecycle_repo(&fx, "mbnodrv", &[]);
    let (fork, tip, head) = merge_base_only(&fx, &main, || {
        std::fs::write(main.join(".gitattributes"), b"*.bin filter=nodrv\n").expect("attributes");
        std::fs::write(main.join("a.bin"), POINTER).expect("pointer");
        fx.git(&main, &["add", ".gitattributes", "a.bin"]);
        fx.git(
            &main,
            &["commit", "--quiet", "-m", "a filtered path at the fork"],
        );
        vec![".gitattributes".to_string(), "a.bin".to_string()]
    });
    let id = ItemId::parse(ITEM).expect("id");
    for at in [&tip, &head] {
        let c = worktree::check_filters(&exec, &main, &id, at, None)
            .unwrap_or_else(|e| panic!("PREMISE: the read over {at} refused: {e:?}"));
        assert!(
            c.selected.is_empty(),
            "PREMISE: {at} selects a driver: {c:?}"
        );
    }
    let refusal = refused(
        "the merge-base selects `nodrv`, declared at no scope; the tip selects nothing",
        worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None),
    );
    assert!(
        matches!(&refusal, WorktreeRefusal::FilterDriverNotEffective { driver, path, commit, .. }
            if driver == "nodrv" && path == "a.bin" && *commit == fork),
        "not the no-effective-driver refusal naming `nodrv`, `a.bin` and the merge-base {fork}: \
         {refusal:?}"
    );
    assert!(
        refusal.to_string().contains(&fork),
        "the refusal does not name {fork}: {refusal}"
    );
    assert_not_created(&fx, &main, "after the refusal");
    assert_eq!(
        sha(&fx, &main, &format!("rk-run/{ITEM}")),
        tip,
        "the run branch moved"
    );
}

/// Round-1 minor (ADR-0180 decision 6, *"taken at every run start and released at the run's
/// end"*): a re-entry whose discard leaves a residue refuses `DiscardIncomplete` by name, keeps
/// the tree, and releases the lock it took, so no refusal leaves the run's own lock behind. The
/// residue is a nested repository, which `clean -fd` (one `-f`) never removes.
///
/// Red while the refusal returns with the lock still held.
#[test]
fn a_reentry_whose_discard_is_incomplete_releases_its_own_lock_and_keeps_the_tree() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let main = lifecycle_repo(&fx, "residue", &[]);
    let e = created_and_released(&fx, &exec, &main);
    let nested = e.tree.path.join("nested");
    std::fs::create_dir_all(&nested).expect("mkdir nested");
    fx.git(&nested, &["init", "--quiet"]);
    std::fs::write(nested.join("n.txt"), b"nested\n").expect("write nested file");
    let r = refused(
        "a re-entry whose discard leaves a nested repository",
        worktree::enter(&exec, &main, ITEM, &owner(2), &|_| None, None),
    );
    assert!(
        matches!(&r, WorktreeRefusal::DiscardIncomplete { remaining, .. }
            if remaining.iter().any(|p| p.starts_with("nested"))),
        "not the incomplete-discard refusal naming `nested`: {r:?}"
    );
    assert_kept(&fx, &main, &e.tree.path, "after the refused re-entry");
    assert_eq!(
        lock_of(&fx, &main, &e.tree.path),
        None,
        "the refused re-entry left the run's own lock on the tree"
    );
    // CONTROL: the reading sees a lock when one is held.
    let held = worktree::lock_reason(&ItemId::parse(ITEM).expect("id"), &owner(3));
    fx.git(
        &main,
        &[
            "worktree",
            "lock",
            "--reason",
            &held,
            e.tree.path.to_str().expect("utf-8"),
        ],
    );
    assert_eq!(lock_of(&fx, &main, &e.tree.path), Some(held));
}

/// Round-1 minor: read (3) matches a `gitdir:` condition against the admin directory git WILL
/// assign the run tree. Git names it after the path's basename and adds a number on a collision
/// (`item-1`, then `item-11`), so a tree registered elsewhere with basename `item-1` pushes the run
/// tree's gitdir to `…/worktrees/item-11`. Two repositories: an include on `item-11` (the run
/// tree's) skipping the smudge refuses naming the included file; an include on `item-1` (the
/// other tree's) is not folded, and creation proceeds. Git's own `rev-parse --git-dir` and config
/// read inside the created tree confirm both premises.
///
/// Red while the read assumes `worktrees/<item>`: the first repository is refused by the scan
/// (the read missed the include), the second by the read (it folded the other tree's include).
#[test]
fn an_include_if_is_matched_against_the_suffixed_gitdir_git_assigns() {
    let fx = crate::support::Fixture::new();
    for (name, pattern, refuses) in [
        ("sfxown", format!("gitdir:**/.git/worktrees/{ITEM}1"), true),
        (
            "sfxother",
            format!("gitdir:**/.git/worktrees/{ITEM}"),
            false,
        ),
    ] {
        let included = format!("{name}-skip.cfg");
        std::fs::write(fx.path().join(&included), SKIPPING_FX.as_bytes()).expect("write");
        let system = format!("{WORKING_FX}[includeIf \"{pattern}\"]\n\tpath = {included}\n");
        let exec = filter_exec(&fx, name, &system, "");
        let main = filter_repo(&fx, name, "*.bin filter=fx\n", &[("a.bin", POINTER)]);
        // Another tree whose basename is the item takes `worktrees/<item>` first.
        let other = fx.sub(&format!("{name}-elsewhere")).join(ITEM);
        let other_s = other.to_str().expect("utf-8").to_string();
        fx.git(
            &main,
            &["worktree", "add", "--quiet", "--detach", &other_s, "HEAD"],
        );
        assert!(
            main.join(".git").join("worktrees").join(ITEM).is_dir(),
            "PREMISE: the other tree does not hold `worktrees/{ITEM}`"
        );
        let r = worktree::enter(&exec, &main, ITEM, &owner(1), &|_| None, None);
        if refuses {
            let refusal = refused(
                &format!("`{pattern}` skipping the smudge for the run tree"),
                r,
            );
            let text = refusal.to_string();
            match &refusal {
                WorktreeRefusal::SmudgeSkipped { driver, origin, .. } => {
                    assert_eq!(driver, "fx", "{text}");
                    assert!(
                        origin.replace('\\', "/").ends_with(&included),
                        "`{pattern}`: the refusal does not name `{included}`: {text}"
                    );
                }
                other => panic!(
                    "`{pattern}` (the run tree's suffixed gitdir): not the skipped-smudge \
                     refusal: {other:?} ({text})"
                ),
            }
            assert_not_created(&fx, &main, &pattern);
        } else {
            let e = assert_created(
                &fx,
                &main,
                r,
                &format!("`{pattern}` (another tree's gitdir)"),
            );
            let gitdir = gitdir_of(&fx, &e.tree.path);
            assert!(
                gitdir
                    .to_string_lossy()
                    .replace('\\', "/")
                    .ends_with(&format!("worktrees/{ITEM}1")),
                "PREMISE: git did not suffix the run tree's gitdir: {}",
                gitdir.display()
            );
            let smudge = rk_git::git_exec::run_with_env(
                &e.tree.path,
                ["config", "--get", "filter.fx.smudge"],
                fixture_env(&fx, name),
            )
            .expect("read the smudge in the run tree")
            .stdout_trimmed();
            assert_eq!(
                smudge, "tr a-z A-Z",
                "PREMISE: git applied `{pattern}` to the run tree"
            );
        }
    }
}
