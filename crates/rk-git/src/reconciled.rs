//! `Reconciled<T>` — the type that makes AC-172 a compile error rather than a review convention.
//!
//! ADR-0109 decision 1: *"Every CLI entry that can cause an action obtains decision-driving state
//! only as the output of `reconcile()` for that invocation. The planner accepts only a
//! `Reconciled<State>` value (AC-172: the compile failure is the instrument)."* Its forbids clause
//! is the same sentence from the other side: *"Forbids any code path that reaches the planner
//! without a `Reconciled<State>` produced in that invocation."*
//!
//! The mechanism is a private field. `Reconciled` can be *named* anywhere — the planner's signature
//! has to name it — but it can only be **constructed** here, by the function that actually performs
//! the reconcile. A caller outside this crate holding a `State` has no way to wrap it, so "did this
//! state come from a reconcile?" stops being a question anyone has to remember to ask.
//!
//! Why the entry set is exactly `{run, answer, doctor}` and not `status`: ADR-0105 clause 8 and
//! DECISIONS-2026-09-12 E11. `status` is read-only and does **not** reconcile.

use crate::error::GitError;
use crate::git_exec::GitExec;
use crate::path::WorktreePath;
use crate::reconcile::{self, State};
use rk_types::drift::Drift;

/// State that provably came out of a `reconcile()` in this invocation.
///
/// The field is private and there is no public constructor, no `From`, no `Default`, and no
/// `DerefMut`. Read access is deliberate and total; write access does not exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciled<T> {
    inner: T,
}

impl<T> Reconciled<T> {
    #[must_use]
    pub const fn get(&self) -> &T {
        &self.inner
    }

    /// Consume the wrapper. Taking the value out is allowed — what is not allowed is putting one
    /// *in* from outside.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// Which CLI entry is reconciling. Every one of these can cause an action; `status` cannot, and is
/// deliberately absent (ADR-0105 clause 8; AC-172; AC-402's negative case).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    Run,
    Answer,
    Doctor,
}

impl Entry {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Answer => "answer",
            Self::Doctor => "doctor",
        }
    }
}

/// What one reconcile produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub state: Reconciled<State>,
    pub drift: Vec<Drift>,
    pub entry: Entry,
}

/// Reconcile at a CLI entry. **The only way a `Reconciled<State>` comes into existence.**
///
/// This performs no git write of any kind — AC-174 requires it to complete against a read-only git
/// directory, creating no commit, ref update, checkout, index write or stash. Every call here is a
/// read (`GitOp::Log`), and there is no cache, TTL or "unchanged since" flag that can skip it
/// (AC-173): the function has no parameter that could carry one.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn reconcile_at_entry(
    exec: &GitExec,
    repo: &WorktreePath,
    entry: Entry,
) -> Result<Outcome, GitError> {
    let commits = reconcile::read_commits(exec, repo)?;
    let (state, drift) = reconcile::reconcile(&commits);
    Ok(Outcome {
        state: Reconciled { inner: state },
        drift,
        entry,
    })
}
