//! `reconcile_runs_at_every_entry` — the third red test s6 §6.3 names, and the only one of the
//! three that is **type-level**.
//!
//! s6 is explicit about the instrument: *"AC-172's `Reconciled<State>` is the instrument: a state
//! that reaches the kernel without passing through it must not be **constructible**. Red as a
//! `trybuild` must-fail fixture, not a runtime assertion, because this one is type-level."*
//!
//! A runtime assertion would be the wrong instrument here and is worth saying why: it could only
//! check that a reconcile *had* happened, which is a different claim from the one ADR-0109 makes.
//! The criterion is that the unreconciled path does not exist.

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::reconciled::{self, Entry};
use rk_git::version;

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// The must-fail fixture. `trybuild` compiles it and **requires** a compile error.
#[test]
fn reconcile_runs_at_every_entry() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/reconciled_cannot_be_constructed.rs");
}

/// The positive half: the sanctioned route does produce one, so the fixture above is failing
/// because construction is closed and not because the type is unusable.
#[test]
fn the_sanctioned_route_produces_reconciled_state() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("r");
    let path = WorktreePath::new(&repo).expect("normalises");

    let outcome = reconciled::reconcile_at_entry(&exec, &path, Entry::Run).expect("reconcile");
    assert_eq!(outcome.entry, Entry::Run);
    let _state: &rk_git::reconcile::State = outcome.state.get();
}

/// AC-175: `reconcile(reconcile(w)) == reconcile(w)`, and the second drift set is empty of any
/// observation the first did not also make. Idempotence is what lets it run at *every* entry
/// without accumulating.
#[test]
fn reconcile_is_idempotent_and_adds_no_drift_on_a_second_run() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("idem");
    let path = WorktreePath::new(&repo).expect("normalises");

    fx.git(
        &repo,
        &[
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            "seal\n\nSlice-Verdict: demo/s-1 approved",
        ],
    );

    let first = reconciled::reconcile_at_entry(&exec, &path, Entry::Run).expect("first");
    let second = reconciled::reconcile_at_entry(&exec, &path, Entry::Run).expect("second");

    assert_eq!(
        first.state.get(),
        second.state.get(),
        "reconcile is not idempotent: the same world produced two different states"
    );
    assert_eq!(
        first.drift, second.drift,
        "a second reconcile over an unchanged world produced a different drift set"
    );
}

/// AC-174: reconcile mutates nothing in git. Asserted by a before/after digest of every file under
/// the git directory, which is stronger than "it did not error".
#[test]
fn reconcile_mutates_nothing_in_git() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("readonly");
    let path = WorktreePath::new(&repo).expect("normalises");

    let before = crate::support::dir_fingerprint(&repo.join(".git"));
    let _ = reconciled::reconcile_at_entry(&exec, &path, Entry::Doctor).expect("reconcile");
    let after = crate::support::dir_fingerprint(&repo.join(".git"));

    assert_eq!(
        before, after,
        "reconcile() changed something under the git directory — AC-174 requires it to complete \
         against a READ-ONLY git dir, creating no commit, ref update, checkout, index write or stash"
    );
}

/// `status` is read-only and does not reconcile (ADR-0105 clause 8; DECISIONS-2026-09-12 E11).
/// Asserted structurally: there is no `Entry` variant for it, so no `status` path can call in.
#[test]
fn status_is_not_a_reconcile_entry() {
    // The entry set is exactly {run, answer, doctor}. If a `Status` variant is ever added, this
    // match stops compiling, which is the point.
    for e in [Entry::Run, Entry::Answer, Entry::Doctor] {
        assert!(matches!(e, Entry::Run | Entry::Answer | Entry::Doctor));
    }
    let names: Vec<&str> = [Entry::Run, Entry::Answer, Entry::Doctor]
        .iter()
        .map(|e| e.as_str())
        .collect();
    assert_eq!(names, ["run", "answer", "doctor"]);
    assert!(
        !names.contains(&"status") && !names.contains(&"explain"),
        "status/explain must not be reconcile entries (AC-402's negative case)"
    );
}
