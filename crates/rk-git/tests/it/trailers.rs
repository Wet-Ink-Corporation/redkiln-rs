//! `trailer_roundtrip_matches_interpret_trailers` — the second red test s6 §6.3 names.
//!
//! Over ADR-0105 clause 3's **named fixture matrix**: an existing trailer block, no trailing
//! newline, a `---` divider, CRLF, and a duplicate token. The stage file is explicit about which
//! case carries the weight: *"Red on the duplicate-token case specifically: git's default
//! `addIfDifferentNeighbor` silently drops a repeated identical `Gate:` trailer, so a wrapper that
//! omits `--if-exists=add` must fail this test. That case is the discriminating one; the other four
//! pass either way."*
//!
//! The wrapper applies the flag itself, so the mutant is to strip it from `GitExec::config_prelude`
//! and from the `interpret-trailers` argv — and the recorded result of running that mutant is in
//! PROGRESS §6.

use std::path::Path;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;
use rk_git::trailer::grammar::{Trailer, TrailerKey};
use rk_git::trailer::write;
use rk_git::version;

/// ADR-0105 clause 3's five named cases, as exact bytes.
fn matrix() -> Vec<(&'static str, &'static [u8])> {
    vec![
        ("existing_block", b"subject\n\nbody\n\nStory: demo/s-1\n"),
        ("no_trailing_newline", b"subject\n\nbody\n\nStory: demo/s-1"),
        ("divider", b"subject\n\nbody\n---\nnot a trailer\n"),
        ("crlf", b"subject\r\n\r\nbody\r\n\r\nStory: demo/s-1\r\n"),
        ("duplicate_token", b"subject\n\nbody\n\nGate: pass\n"),
    ]
}

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// The wrapper's `interpret-trailers` must produce exactly what the oracle produces, over all five
/// cases — and the wrapper must be the thing supplying `--if-exists=add`, not the caller.
#[test]
fn trailer_roundtrip_matches_interpret_trailers() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let cwd = WorktreePath::new(fx.path()).expect("fixture path normalises");
    let trailer = Trailer {
        key: TrailerKey::Gate,
        value: "pass".into(),
    };

    for (case, input) in matrix() {
        // The oracle: git itself, with the flag, exactly as ADR-0105 clause 3 requires.
        let oracle_file = fx.sub(&format!("oracle_{case}"));
        std::fs::write(&oracle_file, input).expect("write oracle fixture");
        fx.git(
            fx.path(),
            &[
                "interpret-trailers",
                "--in-place",
                "--if-exists=add",
                "--trailer",
                "Gate: pass",
                oracle_file.to_str().expect("utf-8"),
            ],
        );
        let expected = std::fs::read(&oracle_file).expect("read oracle back");

        // The candidate: the same operation through the wrapper.
        let cand_file = fx.sub(&format!("cand_{case}"));
        std::fs::write(&cand_file, input).expect("write candidate fixture");
        write::interpret_trailers_in_place(&exec, &cwd, &cand_file, &trailer)
            .unwrap_or_else(|e| panic!("{case}: wrapper refused: {e:?}"));
        let got = std::fs::read(&cand_file).expect("read candidate back");

        assert_eq!(
            got,
            expected,
            "case `{case}` diverged.\n  wrapper: {:?}\n  oracle : {:?}",
            String::from_utf8_lossy(&got),
            String::from_utf8_lossy(&expected)
        );
    }
}

/// The discriminating case, stated on its own so a failure names it.
///
/// Without `--if-exists=add` git leaves ONE `Gate: pass` where the input already had one. Tier-1
/// state would be silently dropped and the commit would report success.
#[test]
fn a_repeated_identical_trailer_is_appended_not_dropped() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let cwd = WorktreePath::new(fx.path()).expect("normalises");
    let file = fx.sub("dup");
    std::fs::write(&file, b"subject\n\nbody\n\nGate: pass\n").expect("write");

    write::interpret_trailers_in_place(
        &exec,
        &cwd,
        &file,
        &Trailer {
            key: TrailerKey::Gate,
            value: "pass".into(),
        },
    )
    .expect("wrapper");

    let text = String::from_utf8(std::fs::read(&file).expect("read")).expect("utf-8");
    let n = text
        .lines()
        .filter(|l| l.trim_end() == "Gate: pass")
        .count();
    assert_eq!(
        n, 2,
        "a repeated identical Gate: trailer was DROPPED — the wrapper is not supplying \
         --if-exists=add (ADR-0105 clause 3).\n{text}"
    );
}

/// The prelude is the wrapper's job, asserted directly rather than only through its effect.
#[test]
fn the_wrapper_supplies_the_hooks_and_trailer_prelude() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);

    let commit: Vec<String> = exec
        .config_prelude(GitOp::Commit)
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();
    assert!(
        commit.iter().any(|a| a.starts_with("core.hooksPath=")),
        "commit carries no core.hooksPath (ADR-0105 clause 2): {commit:?}"
    );
    assert!(
        commit.iter().any(|a| a == "trailer.ifexists=add"),
        "commit carries no trailer.ifexists=add (clause 3): {commit:?}"
    );

    let it: Vec<String> = exec
        .config_prelude(GitOp::InterpretTrailers)
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();
    assert!(it.iter().any(|a| a == "trailer.ifexists=add"));

    // A read carries neither: the prelude is scoped to what it is for, not sprayed everywhere.
    assert!(
        exec.config_prelude(GitOp::RevParse).is_empty(),
        "a read operation is carrying a write-side prelude"
    );
}

/// AC-162's fault-injection fixture: a hook that rewrites the commit message **must have no
/// effect**, because every commit runs with `core.hooksPath` pointed at an empty directory.
#[test]
fn a_message_rewriting_hook_has_no_effect() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("hooked");
    let cwd = WorktreePath::new(&repo).expect("normalises");

    // Install a commit-msg hook that would rewrite every message.
    let hooks = repo.join(".git/hooks");
    std::fs::create_dir_all(&hooks).expect("mkdir hooks");
    let hook = hooks.join("commit-msg");
    std::fs::write(&hook, "#!/bin/sh\necho 'REWRITTEN BY HOOK' > \"$1\"\n").expect("write hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }

    let sha = write::commit_with_trailers(
        &exec,
        &cwd,
        "the real subject",
        &[Trailer {
            key: TrailerKey::Story,
            value: "demo/s-1".into(),
        }],
        true,
    )
    .expect("commit");

    let msg = exec
        .run(GitOp::Log, &cwd, ["log", "-1", "--format=%B", &sha])
        .expect("read the message back")
        .stdout_trimmed();
    assert!(
        msg.contains("the real subject"),
        "the hook rewrote a Redkiln commit — core.hooksPath isolation is not in force.\n{msg}"
    );
    assert!(
        !msg.contains("REWRITTEN BY HOOK"),
        "the hook ran on a Redkiln commit (ADR-0105 clause 2).\n{msg}"
    );
    assert!(
        msg.contains("Story: demo/s-1"),
        "the trailer did not survive into the commit message.\n{msg}"
    );
}

/// Control for the test above: the hook is genuinely capable of firing. Without this, a hook that
/// silently never ran (wrong name, not executable) would make the isolation test pass vacuously.
#[test]
fn control_the_fault_injection_hook_really_fires_without_isolation() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("unhooked");

    let hooks = repo.join(".git/hooks");
    std::fs::create_dir_all(&hooks).expect("mkdir hooks");
    let hook = hooks.join("commit-msg");
    std::fs::write(&hook, "#!/bin/sh\necho 'REWRITTEN BY HOOK' > \"$1\"\n").expect("write hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }

    // Deliberately NOT through the wrapper, and without the hooksPath prelude: a bare commit.
    let out = rk_git::git_exec::run_with_env(
        Path::new(&repo.display().to_string()),
        ["commit", "--quiet", "--allow-empty", "-m", "plain subject"],
        [
            ("GIT_CONFIG_NOSYSTEM", "1"),
            ("GIT_AUTHOR_NAME", "rk"),
            ("GIT_COMMITTER_NAME", "rk"),
            ("GIT_AUTHOR_EMAIL", "rk@example.invalid"),
            ("GIT_COMMITTER_EMAIL", "rk@example.invalid"),
        ],
    );
    assert!(out.is_ok(), "the control commit did not run: {out:?}");

    let msg = String::from_utf8_lossy(
        &rk_git::git_exec::run(
            Path::new(&repo.display().to_string()),
            ["log", "-1", "--format=%B"],
        )
        .expect("read back")
        .stdout,
    )
    .to_string();
    assert!(
        msg.contains("REWRITTEN BY HOOK"),
        "the fault-injection hook did NOT fire even without isolation, so the isolation test above \
         asserts nothing. Fix the fixture before trusting it.\n{msg}"
    );
}
