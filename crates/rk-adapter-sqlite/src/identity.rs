//! The store's identity — AC-369's sqlite half, over Step 6's `rk_identity` table.
//!
//! *"GIVEN a store whose identity rows (root commit + common-dir, schema version, harness
//! versions) do not match the repository it is opened against / WHEN the store is opened / THEN
//! the open is refused with a typed error and nothing is written / AND no "adopt this store"
//! fallback path exists."* The `doctor` rule-table row is ARMED to 7d (`s7c` §7c.22).
//!
//! **The surface is `rk-store`'s** (`s7c` §7c.13): `rk_store::identity::{initialise, read,
//! check}` over `Identity { root_commit, common_dir, schema_version }`, and this module adds
//! only what those three cannot know — **the two git answers** the row needs, taken through
//! `GitExec` (RT-17): the common dir (`crate::path`) and the **root commit**, from `git log
//! --max-parents=0 --format=%H`, the first line. `GitOp` has no `rev-list`, and `run` injects
//! the verb only for `InterpretTrailers`, so the verb is spelt in the args as every caller
//! spells it [measured 2026-09-17]. The root, never `HEAD`: an identity keyed on `HEAD` refuses a
//! repository its own store after its next commit (`ac369_…` watches that mutant red).
//!
//! **The limit of that root, stated.** `git log` walks from `HEAD`, so the root is **the root
//! reachable from the checked-out branch** — and a repository with more than one root (an
//! orphan branch such as `gh-pages`; `checkout --orphan` in the main worktree) derives a
//! different root from a worktree on the other branch, at which point AC-369's refusal fires
//! against AC-370's "any worktree opens the store". A shallow clone's boundary commits are
//! parentless too and would answer as roots. This module implements §7c.13 / RT-17's command as
//! written; **multi-root and shallow repositories are UNDECIDED** — the choice (every root under
//! `--all` with a deterministic pick, or a typed refusal naming the roots) is a dated record's,
//! not this crate's, and is reported against the stage file rather than taken here.
//!
//! **What is compared, honestly.** `rk_store::identity::check` compares the root commit and the
//! schema version and names both common dirs in its refusal; it does not compare the
//! `common_dir` string, and this module adds no comparison of its own — a repository that has
//! been moved keeps its root commit and loses its path, and refusing it on the path is a
//! decision no record takes (AC-179's Step 6 form is what `check` implements). The register's
//! *"harness versions"* are not in Step 6's row (`boot-id` was cut for having no source, and no
//! harness version column exists in `0001_initial.sql`); an identity member that has no column
//! is not invented here.
//!
//! **Nothing is written on a refusal, and the one transaction is what makes that true**: the
//! open reads the stamp with `rk_store::identity::read` and [`establish`] checks it with `check`
//! inside the same `Writer::transaction` as the schema statements; a mismatch returns
//! `StoreError::Identity` out of the closure and the transaction rolls back whole (the crate doc
//! names the mutant that was watched red: a schema committed on its own before the check). The
//! stamp is read before the schema runs because that is the readable order, not because the
//! bytes depend on it. A fresh store — no `rk_identity` table, or no row — is reported as fresh so
//! the caller stamps it after the schema; an existing stamp that matches is left as it is. There
//! is no third path: nothing here re-stamps, and nothing re-reads after a refusal.

use std::path::Path;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;
use rk_store::conn::StoreError;
use rk_store::identity::{Identity, SCHEMA_VERSION};

use crate::StoreOpenError;

/// The repository's root commit: the first line of `git log --max-parents=0 --format=%H` from
/// `worktree`.
///
/// # Errors
/// [`StoreOpenError::Git`] when `git log` cannot run or exits non-zero (a repository with no
/// commit at all exits non-zero here — git 2.55 refuses to log an unborn branch);
/// [`StoreOpenError::NoRootCommit`] when it runs and prints nothing.
pub fn root_commit(exec: &GitExec, worktree: &WorktreePath) -> Result<String, StoreOpenError> {
    let out = exec
        .run(
            GitOp::Log,
            worktree,
            ["log", "--max-parents=0", "--format=%H"],
        )
        .map_err(|error| StoreOpenError::Git {
            op: "log --max-parents=0",
            worktree: worktree.to_string(),
            error: Box::new(error),
        })?;
    let text = out.stdout_trimmed();
    let first = text.lines().next().map(str::trim).unwrap_or_default();
    if first.is_empty() {
        return Err(StoreOpenError::NoRootCommit {
            worktree: worktree.to_string(),
        });
    }
    Ok(first.to_owned())
}

/// The identity this binary opens `worktree`'s store under: its root commit, the common dir the
/// store lives under, and the schema version this binary writes.
///
/// # Errors
/// As [`root_commit`].
pub fn opening(
    exec: &GitExec,
    worktree: &WorktreePath,
    common_dir: &Path,
) -> Result<Identity, StoreOpenError> {
    Ok(Identity {
        root_commit: root_commit(exec, worktree)?,
        common_dir: common_dir.display().to_string(),
        schema_version: SCHEMA_VERSION,
    })
}

/// Decide what the stamp `read` returned means against `opening`, **before anything else
/// touches the store**.
///
/// Takes the stamp rather than the connection, so that the one call site (`SqliteStore::open`'s
/// transaction) is where the read happens and this crate names no connection type of its own —
/// `rusqlite` is not on its manifest (`s7c` §7c.19). Returns `Ok(true)` when the store is fresh
/// (no stamp) and the caller must stamp it after applying the schema; `Ok(false)` when the stamp
/// matches and is left as it is.
///
/// # Errors
/// [`StoreError::Identity`] naming both identities when the stamp is another repository's or
/// another schema version's — the caller maps it to [`StoreOpenError::Refused`].
pub fn establish(stamped: Option<&Identity>, opening: &Identity) -> Result<bool, StoreError> {
    match stamped {
        Some(stamped) => {
            rk_store::identity::check(stamped, opening)?;
            Ok(false)
        }
        None => Ok(true),
    }
}
