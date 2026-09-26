//! The `.worktreeinclude` reader (s8c §8c.4; D21-45 W3): *"the same file with the same semantics
//! (ignored ∩ matched, symlinks skipped)"*.
//!
//! Three tests run over a temporary repository, so both git reads are real: the intersection
//! (mutant M17 drops it), the absent file, and the planted run-worktree sibling (mutant M24 drops
//! the `:(exclude).redkiln/worktrees` pathspec). Two run over `select_carry`, the pure core, so
//! the symlink and escape cases need no OS symlink. **The OS-level symlink case is not asserted
//! here** (s8c §8c.11): creating one on this Windows box needs a privilege it may not grant.
//!
//! *(Added 2026-09-26, s8c build, P6.)*

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_git::error::GitError;
use rk_git::git_exec::GitExec;
use rk_git::version;
use rk_git::worktree_include::{
    CarryRefusal, CarrySet, EntryKind, Kinds, SkipReason, Skipped, carry_set, select_carry,
};

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

fn write(root: &Path, rel: &str, body: &str) {
    let p = root.join(rel);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).expect("mkdir");
    }
    std::fs::write(p, body).expect("write");
}

/// A repository with `.gitignore` committed and, when given, `.worktreeinclude` committed too.
fn repo(
    fx: &crate::support::Fixture,
    name: &str,
    gitignore: &str,
    include: Option<&str>,
) -> PathBuf {
    let r = fx.repo_with_one_commit(name);
    write(&r, ".gitignore", gitignore);
    fx.git(&r, &["add", ".gitignore"]);
    if let Some(inc) = include {
        write(&r, ".worktreeinclude", inc);
        fx.git(&r, &["add", ".worktreeinclude"]);
    }
    fx.git(&r, &["commit", "--quiet", "-m", "ignore rules"]);
    r
}

fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

#[test]
fn carries_only_files_both_ignored_and_matched() {
    let fx = crate::support::Fixture::new();
    let r = repo(
        &fx,
        "inter",
        ".env\nsecret.local\nforced.local\n*.log\n",
        Some(".env\n*.local\n"),
    );
    // Tracked and matched (one of them ignored too, force-added): the checkout holds both.
    write(&r, "tracked.local", "t\n");
    write(&r, "forced.local", "f\n");
    fx.git(&r, &["add", "tracked.local"]);
    fx.git(&r, &["add", "-f", "forced.local"]);
    fx.git(&r, &["commit", "--quiet", "-m", "tracked"]);
    // Matched and ignored: carried, at the root and nested.
    write(&r, ".env", "A=1\n");
    write(&r, "config/.env", "B=2\n");
    write(&r, "secret.local", "s\n");
    // Matched, untracked, NOT ignored: never carried (M17's discriminating file).
    write(&r, "other.local", "o\n");
    // Ignored, NOT matched: never carried.
    write(&r, "build.log", "l\n");

    let got = carry_set(&exec_for(&fx), &r).expect("the carry reads");
    assert_eq!(
        got,
        CarrySet {
            carried: strings(&[".env", "config/.env", "secret.local"]),
            skipped: Vec::new(),
        },
        "the carry is exactly matched ∩ ignored, untracked"
    );
}

#[test]
fn an_absent_file_carries_nothing() {
    let fx = crate::support::Fixture::new();
    let r = repo(&fx, "absent", ".env\n", None);
    write(&r, ".env", "A=1\n");
    assert!(
        !r.join(".worktreeinclude").exists(),
        "the fixture has no .worktreeinclude"
    );
    let got = carry_set(&exec_for(&fx), &r);
    assert_eq!(
        got,
        Ok(CarrySet::default()),
        "an absent .worktreeinclude is the empty set, not an error"
    );
}

#[test]
fn a_file_under_a_run_worktree_is_never_carried() {
    let fx = crate::support::Fixture::new();
    let r = repo(&fx, "runwt", ".env\n", Some(".env\n"));
    write(&r, ".env", "A=1\n");
    // A planted sibling: matched and ignored, but under another run's worktree (D21-44).
    write(&r, ".redkiln/worktrees/x/.env", "X=1\n");
    let got = carry_set(&exec_for(&fx), &r).expect("the carry reads");
    assert_eq!(
        got.carried,
        strings(&[".env"]),
        "the root .env is carried and the run worktree's is not"
    );
    assert!(got.skipped.is_empty(), "nothing is skipped: {got:?}");
}

fn root() -> PathBuf {
    PathBuf::from("canonical-root")
}

fn file(rel: &str) -> EntryKind {
    EntryKind::File {
        canonical: root().join(rel),
    }
}

#[test]
fn a_symlink_is_skipped_and_named() {
    let matched = strings(&[".env", "link.env", "junction.env"]);
    let kinds = Kinds {
        canonical_root: root(),
        entries: BTreeMap::from([
            (".env".to_string(), file(".env")),
            ("link.env".to_string(), EntryKind::Symlink),
            ("junction.env".to_string(), EntryKind::ReparsePoint),
        ]),
    };
    let got = select_carry(&matched, &matched, &kinds).expect("no escape");
    assert_eq!(
        got.carried,
        strings(&[".env"]),
        "only the regular file is carried"
    );
    assert_eq!(
        got.skipped,
        vec![
            Skipped {
                path: "junction.env".into(),
                reason: SkipReason::ReparsePoint,
            },
            Skipped {
                path: "link.env".into(),
                reason: SkipReason::Symlink,
            },
        ],
        "each skipped entry is named with its reason"
    );
    let names: Vec<&str> = got.skipped.iter().map(|s| s.reason.name()).collect();
    assert_eq!(names, ["reparse-point", "symlink"]);
}

#[test]
fn a_path_that_escapes_the_root_is_refused_by_name() {
    let mut cases: Vec<(&str, &str)> = vec![
        ("../outside.env", "carry-path-parent-component"),
        ("a/../../outside.env", "carry-path-parent-component"),
        ("/etc/outside.env", "carry-path-absolute"),
    ];
    if cfg!(windows) {
        cases.push(("C:/outside.env", "carry-path-prefixed"));
        cases.push(("C:outside.env", "carry-path-prefixed"));
    }
    for (bad, name) in cases {
        // One good entry beside the bad one: the refusal is WHOLE, never per path.
        let matched = strings(&[".env", bad]);
        let kinds = Kinds {
            canonical_root: root(),
            entries: BTreeMap::from([
                (".env".to_string(), file(".env")),
                (bad.to_string(), file(bad)),
            ]),
        };
        let got = select_carry(&matched, &matched, &kinds);
        let Err(refusal) = got else {
            panic!("{bad} escaped the root and was not refused: {got:?}");
        };
        assert_eq!(refusal.name(), name, "{bad} is refused by name");
        assert!(refusal.to_string().contains(bad), "the refusal names {bad}");
    }

    // Lexically inside, canonically outside (a junction or symlink in a parent, resolved).
    let matched = strings(&[".env", "inside/.env"]);
    let kinds = Kinds {
        canonical_root: root(),
        entries: BTreeMap::from([
            (".env".to_string(), file(".env")),
            (
                "inside/.env".to_string(),
                EntryKind::File {
                    canonical: PathBuf::from("elsewhere").join(".env"),
                },
            ),
        ]),
    };
    let got = select_carry(&matched, &matched, &kinds);
    assert!(
        matches!(&got, Err(CarryRefusal::OutsideRoot { path, .. }) if path == "inside/.env"),
        "a path canonicalising outside the root refuses the whole carry: {got:?}"
    );
    assert_eq!(got.unwrap_err().name(), "carry-path-outside-root");

    // Control: the same shape with every entry inside the root carries both.
    let kinds_ok = Kinds {
        canonical_root: root(),
        entries: BTreeMap::from([
            (".env".to_string(), file(".env")),
            ("inside/.env".to_string(), file("inside/.env")),
        ]),
    };
    assert_eq!(
        select_carry(&matched, &matched, &kinds_ok).map(|c| c.carried),
        Ok(strings(&[".env", "inside/.env"])),
        "control: an in-root listing carries"
    );

    // And the refusal reaches carry_set's caller as a GitError that names it.
    let e = GitError::CarryRefused {
        refusal: CarryRefusal::ParentComponent {
            path: "../x".into(),
        },
    };
    assert!(
        e.is_precondition(),
        "a refused carry is a precondition, not a transport fault"
    );
}
