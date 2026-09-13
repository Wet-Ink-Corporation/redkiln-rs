//! **S-GIX/a** — does gix discovery from a linked worktree return the main repository's common
//! dir at the pin `gix = "=0.87.1"`?
//!
//! Consumer: the read-side owner of `rev-parse`. ADR-0098 decision 1 fixes both outcomes in
//! advance — on a **go** gix resolves the common dir; on a **no-go** that one read moves into the
//! exec wrapper as `git rev-parse --git-common-dir`, invoked once at startup and cached for the
//! process. So this arm decides an owner, not a capability, and a no-go costs one wrapper call
//! rather than a rewrite (PIA-009: "a no-go costs nothing" was false for this arm).
//!
//! The oracle is deliberately the very command the no-go fallback would use, so the arm and its
//! fallback share one comparator.

use std::path::Path;

use crate::support::{Fixture, assert_agrees_with_record, normalise_path};

/// Control (i): two genuinely different paths must not compare equal.
#[test]
fn control_normalise_keeps_different_paths_different() {
    assert_ne!(normalise_path(r"D:\a\b"), normalise_path(r"D:\a\c"));
}

/// Control (ii): the extended-length prefix is a spelling, not an identity.
#[test]
fn control_normalise_strips_the_extended_length_prefix() {
    assert_eq!(
        normalise_path(r"\\?\D:\repos\x"),
        normalise_path(r"D:/repos/x")
    );
}

/// Control (iii) — the load-bearing one. `main/.git` and the worktree-private
/// `main/.git/worktrees/linked` are DIFFERENT answers, and this arm exists to tell them apart.
/// An over-eager normaliser that collapsed them would make arm a unable to fail.
#[test]
fn control_normalise_keeps_common_dir_and_worktree_dir_different() {
    assert_ne!(
        normalise_path(r"D:\t\main\.git"),
        normalise_path(r"D:\t\main\.git\worktrees\linked")
    );
}

/// Control (iv): a UNC extended-length path is NOT unwrapped. Stripping the prefix would yield
/// `UNC\server\share`, which is a different and broken path — the F20/2 shape, where a mangled
/// path was handed to git and read as a URL with host `?`.
#[test]
fn control_normalise_refuses_to_unwrap_unc() {
    assert_ne!(
        normalise_path(r"\\?\UNC\server\share"),
        normalise_path(r"//server/share")
    );
}

/// Control (v): a traversal is a spelling. `<main>/.git/worktrees/<n>/../..` and `<main>/.git`
/// name the same directory — confirmed independently of this comparator by `realpath` and inode
/// identity — so the comparator must absorb it. Control (iii) is what keeps this from becoming an
/// over-collapse: the two paths WITHOUT a traversal between them still compare unequal.
#[test]
fn control_normalise_resolves_a_traversal_but_not_the_distinction() {
    assert_eq!(
        normalise_path(r"D:\t\main\.git\worktrees\linked\..\.."),
        normalise_path(r"D:\t\main\.git")
    );
    assert_ne!(
        normalise_path(r"D:\t\main\.git\worktrees\linked"),
        normalise_path(r"D:\t\main\.git")
    );
}

#[test]
#[ignore = "S-GIX/a: a spike arm against the gix pin; run explicitly with --ignored"]
fn s_gix_a_common_dir_from_linked_worktree() {
    let fx = Fixture::new();
    let main = fx.repo_with_one_commit("main");
    let linked = fx.sub("linked");

    fx.git(&main, &["branch", "feature"]);
    fx.git(
        &main,
        &[
            "worktree",
            "add",
            linked.to_str().expect("utf-8"),
            "feature",
        ],
    );

    // Assert the fixture really IS a linked worktree BEFORE measuring anything. A `worktree add`
    // that silently did not produce one would otherwise yield a vacuous "go".
    let dotgit = linked.join(".git");
    assert!(
        dotgit.is_file(),
        "fixture is not a linked worktree: {} is not a .git FILE",
        dotgit.display()
    );

    // The oracle: the exact command ADR-0098 decision 1 names as this arm's no-go fallback.
    let oracle_raw = fx
        .git(&linked, &["rev-parse", "--git-common-dir"])
        .stdout_trimmed();
    let oracle_abs = absolutise(&linked, &oracle_raw);
    let oracle = normalise_path(&oracle_abs);

    // The candidate: gix discovery from inside the linked worktree.
    let candidate = match gix::discover(&linked) {
        Ok(repo) => normalise_path(&repo.common_dir().display().to_string()),
        Err(e) => {
            println!("S-GIX/a: NO-GO — gix::discover errored from a linked worktree: {e}");
            panic!("S-GIX/a no-go: gix::discover failed: {e}");
        }
    };

    let expected_main = normalise_path(&main.join(".git").display().to_string());
    let worktree_private =
        normalise_path(&main.join(".git/worktrees/linked").display().to_string());

    println!("S-GIX/a  oracle    = {oracle}");
    println!("S-GIX/a  candidate = {candidate}");
    println!("S-GIX/a  main/.git = {expected_main}");

    // Three conjuncts, and the second is not redundant with the first: equality alone would be
    // satisfied if BOTH implementations returned the worktree-private directory. The arm is about
    // resolving to the MAIN repository, so that is asserted in its own right.
    let agrees = candidate == oracle;
    let is_main = candidate == expected_main;
    let not_private = candidate != worktree_private;
    let measured = if agrees && is_main && not_private {
        "go"
    } else {
        println!("S-GIX/a  agrees={agrees} is_main={is_main} not_worktree_private={not_private}");
        "no-go"
    };

    assert_agrees_with_record('a', measured);
}

/// `git rev-parse --git-common-dir` may answer relatively (`.git`, `../main/.git`); resolve it
/// against the directory the command ran in before comparing.
fn absolutise(cwd: &Path, raw: &str) -> String {
    let p = Path::new(raw);
    let joined = if p.is_absolute() {
        p.to_path_buf()
    } else {
        cwd.join(p)
    };
    // `canonicalize` yields the extended-length form on Windows; `normalise_path` strips it. Fall
    // back to the unresolved join when the path cannot be canonicalised.
    std::fs::canonicalize(&joined)
        .unwrap_or(joined)
        .display()
        .to_string()
}
