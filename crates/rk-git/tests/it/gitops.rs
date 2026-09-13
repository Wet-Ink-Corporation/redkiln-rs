//! The journalled `GitOps` seam — ADR-0105 clause 1, and the half of AC-006/AC-008 that reads
//! *"and zero rk-git writes"*.
//!
//! The full criteria are `rk-kernel`'s and cannot be discharged here: AC-006 replays a **golden
//! event log recorded by `rk-kernel` on the pinned corpus**, and no such log exists — its
//! generation is a Step 5/Step 6 checkpoint item and `rk-kernel` does not exist until 7b. What this
//! file pins is the seam those criteria depend on, so that when the kernel arrives the property is
//! already structural. **Neither AC-006 nor AC-008 is claimed by Step 6.**

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::gitops::{GitOps, Mode, VecJournal};
use rk_git::path::WorktreePath;
use rk_git::version;

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// Record live, then replay: the replay must produce the same answers and perform **zero** writes.
#[test]
fn replay_serves_the_journal_and_performs_zero_writes() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("j");
    let cwd = WorktreePath::new(&repo).expect("normalises");

    // --- live: three reads, journalled ---
    let mut journal = VecJournal::default();
    let live_answers = {
        let mut ops = GitOps::new(exec_for(&fx), &mut journal, Mode::Live);
        let a = ops
            .run(GitOp::RevParse, &cwd, &["rev-parse", "HEAD"])
            .expect("head")
            .stdout_trimmed();
        let b = ops
            .run(
                GitOp::RevParse,
                &cwd,
                &["rev-parse", "--abbrev-ref", "HEAD"],
            )
            .expect("branch")
            .stdout_trimmed();
        let c = ops
            .run(GitOp::Log, &cwd, &["log", "-1", "--format=%s"])
            .expect("subject")
            .stdout_trimmed();
        assert_eq!(
            ops.writes_performed(),
            3,
            "live should have reached git three times"
        );
        [a, b, c]
    };
    assert_eq!(
        journal.entries.len(),
        3,
        "three invocations were not journalled"
    );

    // --- replay: same answers, and the wrapper is never reached ---
    let mut replay_journal = VecJournal::from_entries(journal.entries.clone());
    let mut ops = GitOps::new(exec_for(&fx), &mut replay_journal, Mode::Replay);
    let replayed = [
        ops.run(GitOp::RevParse, &cwd, &["rev-parse", "HEAD"])
            .expect("head")
            .stdout_trimmed(),
        ops.run(
            GitOp::RevParse,
            &cwd,
            &["rev-parse", "--abbrev-ref", "HEAD"],
        )
        .expect("branch")
        .stdout_trimmed(),
        ops.run(GitOp::Log, &cwd, &["log", "-1", "--format=%s"])
            .expect("subject")
            .stdout_trimmed(),
    ];

    assert_eq!(
        replayed, live_answers,
        "replay did not reproduce the recorded answers"
    );
    assert_eq!(
        ops.writes_performed(),
        0,
        "rk-git performed {} invocation(s) during REPLAY — ADR-0105 clause 1 requires zero",
        ops.writes_performed()
    );
}

/// A replay that runs past the end of its journal must **fail loudly**, never fall through to a
/// live invocation. Falling through is the shape that would make "zero writes on replay" true in
/// the tests and false in production.
#[test]
fn an_exhausted_replay_refuses_rather_than_falling_through_to_git() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("exhausted");
    let cwd = WorktreePath::new(&repo).expect("normalises");

    let mut journal = VecJournal::default();
    let mut ops = GitOps::new(exec_for(&fx), &mut journal, Mode::Replay);

    let result = ops.run(GitOp::RevParse, &cwd, &["rev-parse", "HEAD"]);
    assert!(result.is_err(), "an exhausted replay returned a value");
    assert_eq!(
        ops.writes_performed(),
        0,
        "an exhausted replay fell through to a live git invocation"
    );
}

/// The journal records argv as a **digest**, and has nowhere to put the child environment
/// (ADR-0105 clause 7's retention rule).
#[test]
fn the_journal_keeps_argv_as_a_digest_and_has_no_field_for_the_environment() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("ret");
    let cwd = WorktreePath::new(&repo).expect("normalises");

    let mut journal = VecJournal::default();
    {
        let mut ops = GitOps::new(exec_for(&fx), &mut journal, Mode::Live);
        ops.run(GitOp::RevParse, &cwd, &["rev-parse", "HEAD"])
            .expect("run");
    }
    let inv = &journal.entries[0];
    assert_eq!(
        inv.argv_digest.as_str().len(),
        64,
        "argv digest is not a 64-hex digest"
    );
    assert!(
        !inv.argv_digest.as_str().contains("rev-parse"),
        "the argv was retained verbatim; clause 7 says digest-only"
    );
    assert_eq!(inv.stderr_digest.as_str().len(), 64);

    // Structural: there is no environment field to populate. If one is ever added this stops
    // compiling, which is the intent.
    let rk_git::gitops::GitInvocation {
        op: _,
        cwd: _,
        argv_digest: _,
        status: _,
        stdout: _,
        stderr_digest: _,
    } = inv.clone();
}

/// `GitOps` is a struct and git is not a port. Checkpoint 7a counts exactly five port traits in
/// `rk-ports`; a `GitOps` trait would make it six and put git on the port surface ADR-0003 keeps it
/// off (AC-483).
#[test]
fn gitops_is_a_struct_not_a_port_trait() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/gitops.rs");
    let text = std::fs::read_to_string(src).expect("read gitops.rs");
    assert!(
        text.contains("pub struct GitOps"),
        "GitOps is not declared as a struct"
    );
    assert!(
        !text.contains("pub trait GitOps"),
        "GitOps has become a trait — AC-483's port-inventory denylist and Checkpoint 7a's count of \
         five port traits both depend on it being a struct"
    );
}
