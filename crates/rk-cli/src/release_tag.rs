//! **The `release_tag` git-route handler.**
//!
//! `s8g` §8g.7; R5 = ADR-0195 decisions 1 and 2, proposed; R3 = ADR-0193 decision 1, proposed;
//! D21-42 6b. *(Added 2026-09-29, `s8g` build, package S8G-P8-TAG-RELEASE-AND-MOVES.)*
//!
//! `s8c`'s registry row `release_tag` (route `git`, output `ReleaseTag`) is served HERE, in the
//! composition root, through `rk-git`'s journalled seam, and never in a toolchain adapter shelling
//! git (ADR-0105: *"Forbids a wrapper write not on clause 5's list"*). One call:
//!
//! 1. **The rollback target** is read first, through the existing `GitOp::Log`
//!    (`--decorate-refs=refs/tags/rk-release/ --simplify-by-decoration`, so no new read op is owed):
//!    the nearest `rk-release/*` tag on an ancestor of the landed commit, the landed commit itself
//!    excluded. **A first release names it explicitly as `none`**; it is never absent and never
//!    defaulted to the landed sha (R5's forbids).
//! 2. **The tag** is `rk_git::release::tag_release`'s: annotated, local, `rk-release/<item>`, on a
//!    commit of `rk-run/<item>`, idempotent at the same commit and refused by name at another
//!    (`tagged-elsewhere`) or off the run branch (`off-run-branch`). Never pushed: `GitOp` has no
//!    push variant.
//!
//! The output is `rk_types::verb_output::ReleaseTag`; the activity's `emit` composes the
//! `ReleaseRecord` from it and the `BuildArtifact`'s binary digests.

use std::sync::Arc;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::gitops::{GitOps, Mode, VecJournal};
use rk_git::path::WorktreePath;
use rk_git::release::RELEASE_TAG_PREFIX;
use rk_git::worktree::ItemId;
use rk_kernel::runtime::step::{GitRoute, StoreCall};
use rk_ports::toolchain::VerbOutput;
use rk_types::error::StepError;
use rk_types::verb_output::ReleaseTag;

/// What a first release names as its rollback target (R5 decision 1).
pub const NO_ROLLBACK_TARGET: &str = "none";

/// The handler, over one repository.
struct ReleaseTagRoute {
    exec: GitExec,
    repo: WorktreePath,
}

const fn refuse(capability: String) -> StepError {
    StepError::Unsupported { capability }
}

/// The landed commit as its full id, or `None` when git says it names no commit.
fn full_commit(ops: &mut GitOps<'_>, repo: &WorktreePath, rev: &str) -> Option<String> {
    let spec = format!("{rev}^{{commit}}");
    ops.run(
        GitOp::RevParse,
        repo,
        &["rev-parse", "--verify", "--quiet", &spec],
    )
    .ok()
    .map(|out| out.stdout_trimmed())
}

/// **The nearest `rk-release/*` tag on a strict ancestor of `landed`**, or `none`.
///
/// `git log --decorate-refs=refs/tags/rk-release/ --simplify-by-decoration` walks `landed`'s
/// ancestry and keeps the commits a release tag decorates, nearest first; the landed commit itself
/// is skipped (its own tag, on a re-run, is not the previous release). Of two tags on one commit the
/// first by name is taken, so the answer is a function of the repository.
fn rollback_target(
    ops: &mut GitOps<'_>,
    repo: &WorktreePath,
    landed: &str,
) -> Result<String, StepError> {
    let decorate = format!("--decorate-refs=refs/tags/{RELEASE_TAG_PREFIX}");
    let out = ops
        .run(
            GitOp::Log,
            repo,
            &[
                "log",
                &decorate,
                "--simplify-by-decoration",
                "--decorate=full",
                "--format=%H%x09%D",
                landed,
            ],
        )
        .map_err(|e| {
            refuse(format!(
                "`release_tag`: the rollback target could not be read from `{landed}`'s history: \
                 {e:?}"
            ))
        })?;
    let text = out.stdout_trimmed();
    for line in text.lines() {
        let Some((sha, decorations)) = line.split_once('\t') else {
            continue;
        };
        if sha == landed {
            continue;
        }
        let mut tags: Vec<&str> = decorations
            .split(", ")
            .filter_map(|d| d.trim().strip_prefix("tag: refs/tags/"))
            .filter(|t| t.starts_with(RELEASE_TAG_PREFIX))
            .collect();
        tags.sort_unstable();
        if let Some(first) = tags.first() {
            return Ok((*first).to_owned());
        }
    }
    Ok(NO_ROLLBACK_TARGET.to_owned())
}

impl GitRoute for ReleaseTagRoute {
    fn serve(&self, call: &StoreCall<'_>) -> Result<VerbOutput, StepError> {
        if call.verb != "release_tag" {
            return Err(refuse(format!(
                "`{}` takes the `git` route, which serves `release_tag` only (R5 = ADR-0195 \
                 decision 2)",
                call.verb
            )));
        }
        let item = ItemId::parse(call.item).map_err(|e| {
            refuse(format!(
                "`release_tag`: `{}` is not an item id a tag may carry: {e:?}",
                call.item
            ))
        })?;
        let sha = call
            .inputs
            .get("sha")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                refuse(
                    "`release_tag` was handed no `inputs.sha` (the landed commit) to tag (R5 = \
                     ADR-0195 decision 3)"
                        .to_owned(),
                )
            })?;
        let mut journal = VecJournal::default();
        let mut ops = GitOps::new(self.exec.clone(), &mut journal, Mode::Live);
        // (1) The rollback target, read before the write. A sha that names no commit reads no
        // history; `tag_release` then refuses it by name.
        let rollback = match full_commit(&mut ops, &self.repo, sha) {
            Some(full) => rollback_target(&mut ops, &self.repo, &full)?,
            None => NO_ROLLBACK_TARGET.to_owned(),
        };
        // (2) The tag, through the one write site of `GitOp::Tag`.
        let message = format!("redkiln release of {} at {sha}", item.as_str());
        let tag = rk_git::release::tag_release(&mut ops, &self.repo, &item, sha, &message)
            .map_err(|e| match e {
                rk_git::error::GitError::ReleaseRefused { refusal } => {
                    refuse(format!("`release_tag` refused: {refusal}"))
                }
                other => refuse(format!("`release_tag`: {other:?}")),
            })?;
        let value = ReleaseTag {
            tag: tag.tag,
            target: tag.commit,
            rollback_target: rollback,
        };
        Ok(VerbOutput {
            kind: "ReleaseTag".to_owned(),
            value: serde_json::to_value(value).map_err(|e| {
                refuse(format!(
                    "`release_tag`: the ReleaseTag does not serialise: {e}"
                ))
            })?,
        })
    }
}

/// **The `git` route's one seam, as the composition root binds it**: `release_tag` over `repo`.
#[must_use]
pub fn release_tag_route(exec: GitExec, repo: WorktreePath) -> Arc<dyn GitRoute> {
    Arc::new(ReleaseTagRoute { exec, repo })
}
