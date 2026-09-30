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

/// D21-45 W1: *"`LC_ALL=C` on every parsed git call"*, set at the one process-exec site so no
/// parse depends on the operator's locale (s8d-ii §8d-ii.2). Observed from INSIDE the child: a
/// shell alias prints the `LC_ALL` git's child environment carries, through every route to the
/// exec site — `GitExec::run`, `GitExec::run_with_stdin`, the unlabelled `run`, `run_with_env`
/// with a caller's own `LC_ALL` (the pin wins), and a live `GitOps::run`.
/// *(dated 2026-09-27, s8d-ii build, P2.)*
#[test]
fn every_git_child_runs_with_lc_all_c() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("lc");
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let alias = "alias.rklc=!printf 'LC_ALL=[%s]' \"$LC_ALL\"";
    let args = ["-c", alias, "rklc"];
    let want = "LC_ALL=[C]";
    let exec = exec_for(&fx);

    let mut journal = VecJournal::default();
    let mut ops = GitOps::new(exec_for(&fx), &mut journal, Mode::Live);
    let seen = [
        (
            "GitExec::run",
            exec.run(GitOp::Log, &cwd, args)
                .expect("run")
                .stdout_trimmed(),
        ),
        (
            "GitExec::run_with_stdin",
            exec.run_with_stdin(GitOp::CheckIgnore, &cwd, args, b"x\n")
                .expect("run_with_stdin")
                .stdout_trimmed(),
        ),
        (
            "git_exec::run",
            rk_git::git_exec::run(&repo, args)
                .expect("run")
                .stdout_trimmed(),
        ),
        (
            "git_exec::run_with_env (caller sets LC_ALL=de_DE.UTF-8)",
            rk_git::git_exec::run_with_env(&repo, args, [("LC_ALL", "de_DE.UTF-8")])
                .expect("run_with_env")
                .stdout_trimmed(),
        ),
        (
            "GitOps::run (live)",
            ops.run(GitOp::Log, &cwd, &args)
                .expect("gitops")
                .stdout_trimmed(),
        ),
    ];

    let wrong: Vec<String> = seen
        .iter()
        .filter(|(_, got)| got != want)
        .map(|(route, got)| format!("{route}: the child saw `{got}`"))
        .collect();
    assert!(
        wrong.is_empty(),
        "{} of {} routes ran git without LC_ALL=C (W1):\n{}",
        wrong.len(),
        seen.len(),
        wrong.join("\n")
    );
    // CONTROL: the alias reports the variable it is given, so a `C` above is the pin and not the
    // alias printing a constant.
    let ctl = rk_git::git_exec::run_with_env(
        &repo,
        ["-c", "alias.rkx=!printf 'X=[%s]' \"$RK_S8DII_X\"", "rkx"],
        [("RK_S8DII_X", "seen")],
    )
    .expect("control")
    .stdout_trimmed();
    assert_eq!(
        ctl, "X=[seen]",
        "CONTROL: the alias does not report the child's environment"
    );
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

// ---- `s8g`'s write-set record (R3 = ADR-0193, proposed): `GitOp::Tag` and `GitOp::Apply`, each
// scoped, and no push variant. *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT.)*

/// An isolated wrapper whose children carry a fixed identity, so an annotated tag has a tagger on
/// any box, and read no operator configuration (this box sets `core.autocrlf=true` globally, which
/// would rewrite an applied patch's line endings); the fixture's own `git` calls are isolated the
/// same way.
fn isolated_exec(fx: &crate::support::Fixture) -> GitExec {
    exec_for(fx)
        .with_env("GIT_CONFIG_NOSYSTEM", "1")
        .with_env("GIT_CONFIG_GLOBAL", fx.sub("empty.gitconfig"))
        .with_env("GIT_COMMITTER_NAME", "rk")
        .with_env("GIT_COMMITTER_EMAIL", "rk@example.invalid")
        .with_env("GIT_COMMITTER_DATE", "1700000000 +0000")
}

/// A repository whose `rk-run/fix-1` branch carries one commit past `main`, and `main` one commit
/// the run branch does not carry. Returns (repo, the run branch's tip, main's own commit).
fn run_branch_repo(fx: &crate::support::Fixture) -> (std::path::PathBuf, String, String) {
    let repo = fx.repo_with_one_commit("rel");
    fx.git(&repo, &["checkout", "--quiet", "-b", "rk-run/fix-1"]);
    std::fs::write(repo.join("b.txt"), b"run\n").expect("write b.txt");
    fx.git(&repo, &["add", "b.txt"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "landed"]);
    let landed = fx.git(&repo, &["rev-parse", "HEAD"]).stdout_trimmed();
    fx.git(&repo, &["checkout", "--quiet", "main"]);
    std::fs::write(repo.join("c.txt"), b"main\n").expect("write c.txt");
    fx.git(&repo, &["add", "c.txt"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "off the run branch"]);
    let off = fx.git(&repo, &["rev-parse", "HEAD"]).stdout_trimmed();
    (repo, landed, off)
}

/// R3 decision 1: `tag` is a WRITE on the enumerated set, read back by its journalled name, and it
/// runs under clause 2's empty `core.hooksPath` (a tag moves a ref, which runs the target's
/// `reference-transaction` hook). The one tag it makes is annotated, local, `rk-release/<item>`, on
/// the run branch's commit; the same commit again is idempotent and writes nothing.
#[test]
fn tag_is_on_the_enumerated_write_set() {
    assert_eq!(GitOp::from_name("tag"), Some(GitOp::Tag));
    assert_eq!(GitOp::Tag.as_str(), "tag");
    assert!(GitOp::Tag.is_write(), "a tag is a git write (R3)");
    assert!(
        GitOp::Tag.creates_a_commit(),
        "a tag moves a ref, so the target's reference-transaction hook must not run (clause 2)"
    );
    assert!(GitOp::ALL.contains(&GitOp::Tag), "Tag is not in GitOp::ALL");

    let fx = crate::support::Fixture::new();
    let (repo, landed, _off) = run_branch_repo(&fx);
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let item = rk_git::worktree::ItemId::parse("fix-1").expect("a safe id");
    let mut journal = VecJournal::default();
    let first = {
        let mut ops = GitOps::new(isolated_exec(&fx), &mut journal, Mode::Live);
        rk_git::release::tag_release(&mut ops, &cwd, &item, &landed, "release fix-1")
            .expect("the run branch's commit is tagged")
    };
    assert_eq!(first.tag, "rk-release/fix-1");
    assert_eq!(first.commit, landed);
    assert!(first.created, "the first tag was not reported as created");
    assert!(
        journal.entries.iter().any(|inv| inv.op == "tag"),
        "the tag write was not journalled under its op name: {:?}",
        journal.entries.iter().map(|i| i.op).collect::<Vec<_>>()
    );
    let kind = fx
        .git(&repo, &["cat-file", "-t", "refs/tags/rk-release/fix-1"])
        .stdout_trimmed();
    assert_eq!(
        kind, "tag",
        "the release tag is not annotated (R3 forbids a lightweight tag)"
    );
    let target = fx
        .git(&repo, &["rev-parse", "refs/tags/rk-release/fix-1^{commit}"])
        .stdout_trimmed();
    assert_eq!(target, landed);

    // Idempotent at the same commit: no second write, the same answer.
    let mut journal2 = VecJournal::default();
    let again = {
        let mut ops = GitOps::new(isolated_exec(&fx), &mut journal2, Mode::Live);
        rk_git::release::tag_release(&mut ops, &cwd, &item, &landed, "release fix-1")
            .expect("re-tagging the same commit is idempotent")
    };
    assert_eq!(again.commit, landed);
    assert!(!again.created, "an idempotent re-run reported a new tag");
    assert!(
        !journal2.entries.iter().any(|inv| inv.op == "tag"),
        "an idempotent re-run wrote a tag"
    );
}

/// R3 decision 2: `apply` is a WRITE on the enumerated set, and its one caller applies only in the
/// item's `<item>.mut` tree, beside `.review-ref` under the run-tree root (`s8g` §8g.5).
#[test]
fn apply_is_on_the_enumerated_write_set() {
    assert_eq!(GitOp::from_name("apply"), Some(GitOp::Apply));
    assert_eq!(GitOp::Apply.as_str(), "apply");
    assert!(GitOp::Apply.is_write(), "an apply is a git write (R3)");
    assert!(!GitOp::Apply.creates_a_commit());
    assert!(
        GitOp::ALL.contains(&GitOp::Apply),
        "Apply is not in GitOp::ALL"
    );

    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("mut");
    let item = rk_git::worktree::ItemId::parse("fix-2").expect("a safe id");
    let tree = rk_git::worktree::derive(&repo, &item);
    let root = repo.join(".redkiln").join("worktrees");
    assert_eq!(tree.mutation, root.join("fix-2.mut"));
    assert_eq!(
        tree.mutation.parent(),
        tree.reference.parent(),
        "the mutation tree does not sit beside the review reference tree"
    );
    assert_ne!(
        tree.mutation, tree.path,
        "the mutation tree IS the run-owned tree"
    );
    fx.git(
        &repo,
        &[
            "worktree",
            "add",
            "--quiet",
            "--detach",
            tree.mutation.to_str().expect("utf-8"),
            "HEAD",
        ],
    );
    let patch = b"--- a/a.txt\n+++ b/a.txt\n@@ -1 +1 @@\n-one\n+mutated\n";
    let mut journal = VecJournal::default();
    {
        let mut ops = GitOps::new(isolated_exec(&fx), &mut journal, Mode::Live);
        rk_git::mutation::apply(&mut ops, &tree, patch)
            .expect("the patch applies in the .mut tree");
    }
    assert_eq!(
        std::fs::read(tree.mutation.join("a.txt")).expect("read"),
        b"mutated\n"
    );
    assert_eq!(
        std::fs::read(repo.join("a.txt")).expect("read"),
        b"one\n",
        "the patch reached the main worktree"
    );
    assert!(
        journal.entries.iter().any(|inv| inv.op == "apply"),
        "the apply was not journalled under its op name"
    );
}

/// R3's forbids bullet: *"A push of any ref, and a `GitOp` push variant."* Over the enum's names and
/// over `git_exec.rs`'s code lines, with a planted control so an empty sweep proves something.
#[test]
fn the_write_set_has_no_push() {
    let named: Vec<&str> = GitOp::ALL
        .iter()
        .map(|op| op.as_str())
        .filter(|name| name.contains("push"))
        .collect();
    assert!(named.is_empty(), "GitOp names a push: {named:?}");
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/git_exec.rs");
    let text = std::fs::read_to_string(src).expect("read git_exec.rs");
    let push_code_lines = |text: &str| -> Vec<String> {
        text.lines()
            .map(str::trim_start)
            .filter(|l| !l.starts_with("//"))
            .filter(|l| l.contains("Push") || l.contains("\"push"))
            .map(ToOwned::to_owned)
            .collect()
    };
    let hits = push_code_lines(&text);
    assert!(
        hits.is_empty(),
        "git_exec.rs carries a push on a code line: {hits:?}"
    );
    // CONTROL: the sweep keeps a planted variant line and drops a comment naming it.
    let planted = "pub enum GitOp {\n    Push,\n    // Push is forbidden\n}\n";
    assert_eq!(push_code_lines(planted), vec!["Push,".to_owned()]);
    // The enum has variants at all, so the name sweep above was over something.
    assert!(GitOp::ALL.len() > 20);
}

/// R3 decision 1: *"The target must be a commit on the run-owned branch `rk-run/<item>` … any other
/// target refuses by name."* Nothing is tagged on the refusal; the control tags the branch's commit,
/// and once it is tagged a second target refuses by name.
#[test]
fn tag_refuses_a_target_off_the_run_owned_branch() {
    let fx = crate::support::Fixture::new();
    let (repo, landed, off) = run_branch_repo(&fx);
    let cwd = WorktreePath::new(&repo).expect("normalises");
    let item = rk_git::worktree::ItemId::parse("fix-1").expect("a safe id");
    let mut journal = VecJournal::default();
    let refused = {
        let mut ops = GitOps::new(isolated_exec(&fx), &mut journal, Mode::Live);
        rk_git::release::tag_release(&mut ops, &cwd, &item, &off, "release fix-1")
    };
    match refused {
        Err(rk_git::error::GitError::ReleaseRefused {
            refusal:
                rk_git::release::ReleaseRefusal::OffRunBranch {
                    item,
                    commit,
                    branch,
                },
        }) => {
            assert_eq!(item, "fix-1");
            assert_eq!(commit, off);
            assert_eq!(branch, "rk-run/fix-1");
        }
        other => panic!("a commit off rk-run/fix-1 was not refused by name: {other:?}"),
    }
    assert!(
        !journal.entries.iter().any(|inv| inv.op == "tag"),
        "a refused target still reached `git tag`"
    );
    assert!(
        fx.try_git(
            &repo,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                "refs/tags/rk-release/fix-1"
            ]
        )
        .is_err(),
        "a tag exists after the refusal"
    );
    // CONTROL: the run branch's own commit is tagged, so the refusal above is about the target.
    let mut journal2 = VecJournal::default();
    let mut ops = GitOps::new(isolated_exec(&fx), &mut journal2, Mode::Live);
    let ok = rk_git::release::tag_release(&mut ops, &cwd, &item, &landed, "release fix-1")
        .expect("CONTROL: the run branch's commit is tagged");
    assert_eq!(ok.commit, landed);
    // Once tagged, a different commit ON the run branch refuses by name (re-running at a different
    // commit, R3 decision 1), and the tag stays where it was.
    fx.git(&repo, &["checkout", "--quiet", "rk-run/fix-1"]);
    std::fs::write(repo.join("d.txt"), b"later\n").expect("write d.txt");
    fx.git(&repo, &["add", "d.txt"]);
    fx.git(&repo, &["commit", "--quiet", "-m", "a later landing"]);
    let later = fx.git(&repo, &["rev-parse", "HEAD"]).stdout_trimmed();
    fx.git(&repo, &["checkout", "--quiet", "main"]);
    let moved = rk_git::release::tag_release(&mut ops, &cwd, &item, &later, "release fix-1");
    assert!(
        matches!(
            moved,
            Err(rk_git::error::GitError::ReleaseRefused {
                refusal: rk_git::release::ReleaseRefusal::TaggedElsewhere { .. }
            })
        ),
        "a second target was not refused by name: {moved:?}"
    );
    let target = fx
        .git(&repo, &["rev-parse", "refs/tags/rk-release/fix-1^{commit}"])
        .stdout_trimmed();
    assert_eq!(target, landed, "a refused re-tag moved the tag");
}
