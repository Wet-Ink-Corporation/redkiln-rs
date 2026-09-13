//! `git_paths_are_drive_letter_form` — the red test s6 §6.3 names, pinning the F20/2 normaliser.
//!
//! Golden constraint 4's warning is attached to this file and must not be lost: **this defect was
//! not reachable from a unit test when it happened**, because nothing in the spike's unit suite
//! touched the filesystem or the CLI. So the unit test below pins the normaliser, and the wrapper
//! still needs one real end-to-end `git` invocation before Checkpoint 6 is claimed — that is
//! `wrapper_accepts_a_canonicalised_path_end_to_end`, at the bottom.

use std::path::Path;

use rk_git::error::GitError;
use rk_git::path::WorktreePath;

/// The case the corpus fix already handles: a verbatim drive path is stripped, not re-slashed.
#[test]
fn git_paths_are_drive_letter_form() {
    let p = WorktreePath::new(Path::new(r"\\?\D:\repos\redkiln")).expect("drive path is accepted");
    assert_eq!(p.as_str(), "D:/repos/redkiln");
    assert!(
        !p.as_str().starts_with("//?/") && !p.as_str().contains('\\'),
        "an extended-length prefix survived into a path bound for git: {p}"
    );
}

/// The discriminating case, and the edit this port makes to the corpus version.
///
/// `\\?\UNC\server\share` denotes `\\server\share`. Stripping the prefix yields
/// `UNC/server/share`, which denotes nothing at all — the corpus normaliser turns one broken path
/// into a *different* broken path, moving the failure away from its cause. The port refuses.
#[test]
fn a_unc_extended_length_path_is_refused_not_mangled() {
    let given = r"\\?\UNC\server\share";
    match WorktreePath::new(Path::new(given)) {
        Err(GitError::ExtendedLengthPath { given: g }) => assert_eq!(g, given),
        Err(other) => panic!("refused, but with the wrong error: {other:?}"),
        Ok(p) => panic!(
            "a UNC extended-length path was MANGLED into `{p}` instead of refused — this is the \
             corpus normaliser's behaviour, and it is what this port exists to change"
        ),
    }
}

/// Control: an ordinary path is passed through unharmed. Without this, a normaliser that refused
/// everything would satisfy the test above.
#[test]
fn control_an_ordinary_path_is_accepted_unchanged() {
    let p = WorktreePath::new(Path::new(r"D:\repos\redkiln-rs")).expect("ordinary path");
    assert_eq!(p.as_str(), "D:/repos/redkiln-rs");
}

/// Control: a genuine UNC path with no device prefix is fine — it is only the `\\?\UNC\` device
/// form that cannot be unwrapped.
#[test]
fn control_a_plain_unc_path_is_accepted() {
    let p = WorktreePath::new(Path::new(r"\\server\share\x")).expect("plain UNC");
    assert_eq!(p.as_str(), "//server/share/x");
}

/// Golden constraint 4: the unit tests above pin the normaliser, and pinning the normaliser is not
/// evidence that a path ever reached `git`. This runs one real invocation through the wrapper with
/// a path that has been through `std::fs::canonicalize` — the exact call that produced the
/// extended-length form in F20/2 — and asserts `git` accepted it.
#[test]
fn wrapper_accepts_a_canonicalised_path_end_to_end() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("canon");

    let canonical = std::fs::canonicalize(&repo).expect("canonicalize");
    // The shape F20/2 hit: on Windows this really is `\\?\D:\...`.
    let normalised = WorktreePath::new(&canonical).expect("canonicalised path normalises");

    let out = fx.git(
        fx.path(),
        &[
            "-C",
            normalised.as_str(),
            "rev-parse",
            "--is-inside-work-tree",
        ],
    );
    assert_eq!(
        out.stdout_trimmed(),
        "true",
        "git did not accept the normalised canonical path `{normalised}`"
    );
}
