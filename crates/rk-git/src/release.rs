//! **The release tag** — `s8g`'s write-set record (R3 = ADR-0193 decision 1, proposed), for the tag
//! record's `release_tag` handler (R5 = ADR-0195, proposed; D21-42 6b: *"The tag is local and never
//! pushed by the run."*). *(Added 2026-09-29, `s8g` build, package S8G-P3-KERNEL-GIT.)*
//!
//! One function, [`tag_release`], and the one code site that names [`GitOp::Tag`]:
//!
//! - **The tag** is annotated and local, `rk-release/<item>`, on a commit of the run-owned branch
//!   `rk-run/<item>`. A target that is not such a commit refuses by name before `git tag` runs
//!   ([`ReleaseRefusal::OffRunBranch`]).
//! - **Re-running** for the same item at the same commit is idempotent and writes nothing; at a
//!   different commit it refuses by name ([`ReleaseRefusal::TaggedElsewhere`]) and the tag stays.
//! - **Never pushed.** `GitOp` has no push variant, and this module adds none.
//! - Every call goes through the journalled seam, so a replay is served its reads and its write
//!   and reaches no `git`. Git's exit code is read (as `check-ignore`'s exit 1 is); its stderr never.
//!
//! The rollback-target read (`log --decorate-refs=refs/tags/rk-release/`) and the `ReleaseRecord`
//! composition are the tag record's `release_tag` handler's, in `rk-cli`, and not here.

use crate::error::GitError;
use crate::git_exec::GitOp;
use crate::gitops::GitOps;
use crate::path::WorktreePath;
use crate::worktree::{ItemId, RUN_BRANCH_PREFIX};

/// The release tag namespace: `rk-release/<item>` (R5; R3's forbids: no tag outside it).
pub const RELEASE_TAG_PREFIX: &str = "rk-release/";

/// Why a release tag was refused, by name. Nothing was tagged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseRefusal {
    /// The target is not a commit on the run-owned branch (`authorise_tier1_write`'s ownership
    /// pair's branch half), or does not name a commit at all, or the branch does not exist.
    OffRunBranch {
        item: String,
        commit: String,
        branch: String,
    },
    /// The item's tag already names another commit: re-tagging at a different commit refuses.
    TaggedElsewhere {
        tag: String,
        at: String,
        requested: String,
    },
}

impl std::fmt::Display for ReleaseRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OffRunBranch {
                item,
                commit,
                branch,
            } => write!(
                f,
                "off-run-branch: the release target `{commit}` for item `{item}` is not a commit on \
                 the run-owned branch `{branch}`, so no tag is made (R3 = ADR-0193 decision 1)"
            ),
            Self::TaggedElsewhere { tag, at, requested } => write!(
                f,
                "tagged-elsewhere: `{tag}` already names `{at}`, and a release at `{requested}` \
                 refuses rather than moving it (R3 = ADR-0193 decision 1)"
            ),
        }
    }
}

/// The tag a call made or found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseTag {
    /// `rk-release/<item>`.
    pub tag: String,
    /// The full commit id the tag names.
    pub commit: String,
    /// `false` when the tag already named this commit and nothing was written.
    pub created: bool,
}

/// `rk-release/<item>`.
#[must_use]
pub fn tag_name(item: &ItemId) -> String {
    format!("{RELEASE_TAG_PREFIX}{}", item.as_str())
}

fn off_branch(item: &ItemId, commit: &str, branch: &str) -> GitError {
    GitError::ReleaseRefused {
        refusal: ReleaseRefusal::OffRunBranch {
            item: item.as_str().to_owned(),
            commit: commit.to_owned(),
            branch: branch.to_owned(),
        },
    }
}

/// A commit-ish resolved to its full id, or `None` when git says it names no commit.
fn resolve_commit(
    ops: &mut GitOps<'_>,
    repo: &WorktreePath,
    rev: &str,
) -> Result<Option<String>, GitError> {
    let spec = format!("{rev}^{{commit}}");
    match ops.run(
        GitOp::RevParse,
        repo,
        &["rev-parse", "--verify", "--quiet", &spec],
    ) {
        Ok(out) => Ok(Some(out.stdout_trimmed())),
        Err(GitError::NonZeroExit { .. }) => Ok(None),
        Err(e) => Err(e),
    }
}

/// **Tag the release**: one annotated local tag `rk-release/<item>` on `commit`, which must be on
/// `rk-run/<item>`.
///
/// # Errors
/// [`GitError::ReleaseRefused`] naming the refusal; the seam's own [`GitError`] otherwise.
pub fn tag_release(
    ops: &mut GitOps<'_>,
    repo: &WorktreePath,
    item: &ItemId,
    commit: &str,
    message: &str,
) -> Result<ReleaseTag, GitError> {
    let branch = format!("{RUN_BRANCH_PREFIX}{}", item.as_str());
    let tag = tag_name(item);
    // (1) The target names a commit, and the run branch exists.
    let Some(full) = resolve_commit(ops, repo, commit)? else {
        return Err(off_branch(item, commit, &branch));
    };
    let branch_ref = format!("refs/heads/{branch}");
    if resolve_commit(ops, repo, &branch_ref)?.is_none() {
        return Err(off_branch(item, &full, &branch));
    }
    // (2) The target is on the run branch: `merge-base --is-ancestor` exits 1 for "not an
    // ancestor" (a read; D21-47: *"A READ is not a supersession"*).
    match ops.run(
        GitOp::MergeBase,
        repo,
        &["merge-base", "--is-ancestor", &full, &branch_ref],
    ) {
        Ok(_) => {}
        Err(GitError::NonZeroExit { code: Some(1), .. }) => {
            return Err(off_branch(item, &full, &branch));
        }
        Err(e) => return Err(e),
    }
    // (3) An existing tag: the same commit is idempotent, another refuses.
    let tag_ref = format!("refs/tags/{tag}");
    if let Some(at) = resolve_commit(ops, repo, &tag_ref)? {
        if at == full {
            return Ok(ReleaseTag {
                tag,
                commit: full,
                created: false,
            });
        }
        return Err(GitError::ReleaseRefused {
            refusal: ReleaseRefusal::TaggedElsewhere {
                tag,
                at,
                requested: full,
            },
        });
    }
    // (4) THE WRITE: annotated (`-a`, never lightweight), unsigned whatever the operator's config
    // says, under clause 2's empty `core.hooksPath` (the wrapper's prelude for `Tag`).
    ops.run(
        GitOp::Tag,
        repo,
        &[
            "-c",
            "tag.gpgSign=false",
            "tag",
            "-a",
            &tag,
            "-m",
            message,
            &full,
        ],
    )?;
    // (5) Re-read, never assumed (`s6` §6.2 constraint 4).
    match resolve_commit(ops, repo, &tag_ref)? {
        Some(at) if at == full => Ok(ReleaseTag {
            tag,
            commit: full,
            created: true,
        }),
        Some(at) => Err(GitError::ReleaseRefused {
            refusal: ReleaseRefusal::TaggedElsewhere {
                tag,
                at,
                requested: full,
            },
        }),
        None => Err(GitError::Spawn {
            detail: format!("`git tag` exited 0 and `{tag_ref}` does not re-read as a commit"),
        }),
    }
}
