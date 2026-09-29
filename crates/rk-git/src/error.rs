//! `GitError` — the closed error set `rk-git` returns.
//!
//! Same shape as `rk_types::StepError`: matched exhaustively with no catch-all, so a new variant is
//! `E0004` at every call site rather than a silent fallthrough. `rk-git` carries no `anyhow` —
//! that is confined to `rk-cli` and `redkiln-migrate` (ADR-0102 decision 6), and CI asserts it over
//! `cargo metadata`.
//!
//! RK-B0013 is the failure this shape exists to stop, one level up: a transport fault that reads as
//! a verdict. Here the equivalent is a non-zero `git` exit read as a state transition, which is why
//! `NonZeroExit` carries stderr for the record and nothing in this crate branches on its text.

use std::path::PathBuf;

/// What went wrong. Closed; extended only by a successor record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitError {
    /// `git` is not on `PATH` at all. Raised at startup, never at first use (ADR-0077).
    GitNotOnPath { detail: String },

    /// `git` is present but older than the floor ADR-0091 decision 4 fixes.
    VersionBelowFloor { found: String, floor: &'static str },

    /// `git --version` produced something this crate cannot parse into a version.
    VersionUnparsable { output: String },

    /// The child could not be spawned: the OS refused.
    Spawn { detail: String },

    /// `git` ran and exited non-zero. `stderr` is carried verbatim for retention, never parsed.
    NonZeroExit {
        op: &'static str,
        code: Option<i32>,
        stderr: Vec<u8>,
    },

    /// An extended-length (`\\?\`) path reached the boundary. Handing one to `git` is the F20/2
    /// defect — `git fetch` read `//?/D:/repos/redkiln` as a URL with host `?` and every call
    /// failed. This crate refuses such a path rather than mangling it into a different one.
    ExtendedLengthPath { given: String },

    /// A tier-1 write was attempted against a worktree the store does not own for this run
    /// (AC-529; ADR-0105 clause 8 narrowing ADR-0018's "current worktree").
    ForeignWorktree {
        target_path: PathBuf,
        target_branch: String,
        owned_path: PathBuf,
        owned_branch: String,
    },

    /// `git worktree remove` failed — on Windows, typically a handle no `Stop-Process` releases
    /// (Halt 6). A typed error and a `doctor` orphans row, never a retry into a reused tree.
    WorktreeRemoveFailed { path: PathBuf, stderr: Vec<u8> },

    /// The `.worktreeinclude` reader refused the whole carry (s8c §8c.4, D21-45 W3): a listed
    /// path was absolute, prefixed, carried a `..` component, or canonicalised outside the
    /// canonical repository root. Whole, not per path, because an escaping entry means the listing
    /// is not what the reader thinks it is. *(Added 2026-09-26, s8c build, P6.)*
    CarryRefused {
        refusal: crate::worktree_include::CarryRefusal,
    },

    /// Run-tree creation refused by name before (or, for the realpath check, just after) `git
    /// worktree add`: an unsafe item id, a case-fold collision, a bare repository, a symlinked or
    /// junctioned segment, a created tree whose canonical path leaves the root, a root git does
    /// not report as ignored, or a worst-case path past the platform limit (s8d-ii §8d-ii.1,
    /// §8d-ii.2 and §8d-ii.6; D21-45 W1; D21-48 X1). *(dated 2026-09-27, s8d-ii build, P2.)*
    WorktreeRefused {
        refusal: crate::worktree::WorktreeRefusal,
    },

    /// The landing's `commit-tree` object does not re-read as the tree the landing derived (the
    /// landing record, ADR-0188 decision 9 step 5; `s6` §6.2 constraint 4: *"anything written by the
    /// executable and then read is RE-READ, never assumed"*). No ref moved.
    /// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
    LandingReread { expected: String, found: String },
}

impl GitError {
    /// Does this fault mean the operator said no, or that the tool never spoke? The distinction
    /// RK-B0013 was written about, kept available to callers rather than inferred from a message.
    #[must_use]
    pub const fn is_precondition(&self) -> bool {
        match self {
            Self::GitNotOnPath { .. }
            | Self::VersionBelowFloor { .. }
            | Self::VersionUnparsable { .. }
            | Self::ExtendedLengthPath { .. }
            | Self::ForeignWorktree { .. }
            | Self::CarryRefused { .. }
            | Self::WorktreeRefused { .. } => true,
            Self::Spawn { .. }
            | Self::NonZeroExit { .. }
            | Self::WorktreeRemoveFailed { .. }
            | Self::LandingReread { .. } => false,
        }
    }
}
