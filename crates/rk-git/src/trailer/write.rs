//! The trailer **write** path — on the executable, where ADR-0077 put it and where S-GIX/b's
//! no-go leaves it.
//!
//! ADR-0091 measured that `gix 0.87.1` exposes no trailer-writing API at all: over the resolved
//! source of `gix`, `gix-object` and `gix-actor`, every public item naming `trailer` is read-side.
//! So this path is `git commit --trailer` and `git interpret-trailers`, both through the one
//! wrapper, both carrying `--if-exists=add` / `-c trailer.ifexists=add`.
//!
//! **Why the flag is not optional.** `git interpret-trailers` defaults `ifExists` to
//! `addIfDifferentNeighbor`, which silently does *not* append a repeated identical trailer. A
//! repeated identical `Gate:` or `Slice-Verdict:` is exactly what the three-axis design produces —
//! a slice sealed twice with the same verdict, a gate re-opened at the same tree — so a wrapper
//! that omitted the flag would drop tier-1 state and report success. That default is **measured**,
//! not assumed: `control_oracle_drops_a_duplicate_without_if_exists_add` runs git both ways.

use std::path::Path;

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;
use crate::trailer::grammar::Trailer;

/// Append `trailer` to the message file at `file`, in place.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn interpret_trailers_in_place(
    exec: &GitExec,
    cwd: &WorktreePath,
    file: &Path,
    trailer: &Trailer,
) -> Result<(), GitError> {
    let target = WorktreePath::new(file)?;
    exec.run(
        GitOp::InterpretTrailers,
        cwd,
        [
            "--in-place",
            "--trailer",
            &trailer.to_line(),
            target.as_str(),
        ],
    )?;
    Ok(())
}

/// Create a commit carrying `trailers`, optionally empty.
///
/// Tier-1 transitions with no file changes to check point still get a commit — ADR-0109 decision 2
/// and AC-163: *"an empty commit carrying the trailer is created"*, and the transition is never
/// recorded in the store alone.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn commit_with_trailers(
    exec: &GitExec,
    cwd: &WorktreePath,
    message: &str,
    trailers: &[Trailer],
    allow_empty: bool,
) -> Result<String, GitError> {
    let mut args: Vec<String> = vec!["commit".into(), "--quiet".into()];
    if allow_empty {
        args.push("--allow-empty".into());
    }
    args.push("-m".into());
    args.push(message.to_string());
    for t in trailers {
        args.push("--trailer".into());
        args.push(t.to_line());
    }
    exec.run(GitOp::Commit, cwd, &args)?;

    // ADR-0105 clause 4 / §6.2 constraint 4: anything written by the executable and then read is
    // RE-READ, never assumed. Two git implementations in one process can disagree, so the commit
    // this function reports is the one git says HEAD now is.
    let head = exec.run(GitOp::RevParse, cwd, ["rev-parse", "HEAD"])?;
    Ok(head.stdout_trimmed())
}
