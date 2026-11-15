//! `GitError`'s operator-facing rendering (#16).
//!
//! The exit-3 paths in `rk-cli` printed `GitError` with `{e:?}`, which renders `stderr` as a
//! `Vec<u8>` of integers. `Display` renders the op, the exit code and the stderr as text.

use std::path::PathBuf;

use rk_git::error::GitError;

/// A non-zero exit renders its op, its code and its stderr as text, never as byte integers.
#[test]
fn a_non_zero_exit_renders_the_op_the_code_and_the_stderr_text() {
    let e = GitError::NonZeroExit {
        op: "rev-parse",
        code: Some(128),
        stderr: b"fatal: not a git repository\n".to_vec(),
    };

    let shown = e.to_string();

    assert_eq!(
        shown, "git rev-parse exited with code 128: fatal: not a git repository",
        "the rendering names the op, the code and the stderr text"
    );
}

/// A child killed by a signal has no exit code, and the rendering says so instead of inventing one.
#[test]
fn a_non_zero_exit_without_a_code_says_it_was_terminated() {
    let e = GitError::NonZeroExit {
        op: "fetch",
        code: None,
        stderr: Vec::new(),
    };

    let shown = e.to_string();

    assert_eq!(
        shown, "git fetch was terminated without an exit code (no stderr)",
        "no code and no stderr are both stated"
    );
}

/// Stderr that is not UTF-8 is rendered lossily, and stderr past the bound is truncated with a
/// marker, so a runaway child cannot flood the operator's terminal.
#[test]
fn stderr_is_lossy_and_bounded() {
    let mut stderr = vec![0xff_u8];
    stderr.extend(std::iter::repeat_n(b'x', 10_000));
    let e = GitError::WorktreeRemoveFailed {
        path: PathBuf::from("/tmp/wt"),
        stderr,
    };

    let shown = e.to_string();

    assert!(
        shown.starts_with("git worktree remove failed for /tmp/wt: \u{fffd}x"),
        "lossy UTF-8 replaces the invalid byte: {shown:.80}"
    );
    assert!(
        shown.ends_with("… (stderr truncated)"),
        "a long stderr is truncated with a marker: {:?}",
        shown.chars().rev().take(40).collect::<String>()
    );
    assert!(
        shown.chars().count() < 2_200,
        "the rendering is bounded: {} chars",
        shown.chars().count()
    );
}
