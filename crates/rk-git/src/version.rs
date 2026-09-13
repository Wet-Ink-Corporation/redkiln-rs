//! The startup precondition: `git` is on `PATH`, at or above the floor.
//!
//! ADR-0077 requires this **at startup, not at first use**, and left the number open —
//! *"That minimum version is to be established and written into the assertion when the wrapper is
//! built; it is not asserted here, because this pass did not verify it."* **ADR-0091 decision 4**
//! establishes it: `git >= 2.20`, which is what `interpret-trailers --if-exists=add` needs, with
//! every other operation in ADR-0105 clause 5's write set older than that. `git clone --revision`
//! (git 2.49) would have raised it, but it is reached only on an S-GIX/c **no-go**, and /c is a
//! **go** — gix performs the review clone, so the wrapper never invokes it.
//!
//! "At startup, not at first use" is made a **type fact** rather than a convention: [`GitToolchain`]
//! is a capability token that only [`assert_at_startup`] can mint, and every constructor that can
//! reach `git` demands one. A caller cannot skip the check by calling an operation directly,
//! because there is no operation to call without the token.

use crate::error::GitError;
use crate::git_exec;

/// The floor, as a `(major, minor)` pair. ADR-0091 decision 4.
pub const GIT_FLOOR: (u32, u32) = (2, 20);

/// The floor as it is written in ADR-0091, for the record-vs-code cross-check.
pub const GIT_FLOOR_STR: &str = "2.20";

/// Proof that the startup precondition ran and passed.
///
/// Un-constructible outside this crate, which is what makes it a capability rather than a comment:
/// `GitExec::new` demands one, and only [`assert_at_startup`] can produce one, so "was the
/// precondition checked?" is answered by the type system rather than by a call-order convention.
///
/// `#[non_exhaustive]` rather than a `_private: ()` field — clippy's `manual_non_exhaustive` names
/// the hand-rolled form, and the attribute says the same thing in the language's own words while
/// still letting callers read `found`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct GitToolchain {
    /// The version actually found, kept for the record (Halt 2: the instrument is part of every
    /// record, and "at or above the floor" is a floor check, not a match).
    pub found: VersionTriple,
}

/// A parsed `git --version`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionTriple {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl std::fmt::Display for VersionTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Assert the precondition and mint the token.
///
/// # Errors
/// [`GitError::GitNotOnPath`] if `git` cannot be spawned, [`GitError::VersionUnparsable`] if
/// `git --version` does not yield a version, [`GitError::VersionBelowFloor`] if it is below
/// [`GIT_FLOOR`].
pub fn assert_at_startup(cwd: &std::path::Path) -> Result<GitToolchain, GitError> {
    let out = git_exec::run(cwd, ["--version"]).map_err(|e| match e {
        GitError::Spawn { detail } => GitError::GitNotOnPath { detail },
        other => other,
    })?;
    let text = out.stdout_trimmed();
    let found = parse_version(&text).ok_or_else(|| GitError::VersionUnparsable {
        output: text.clone(),
    })?;

    if (found.major, found.minor) < GIT_FLOOR {
        return Err(GitError::VersionBelowFloor {
            found: text,
            floor: GIT_FLOOR_STR,
        });
    }
    Ok(GitToolchain { found })
}

/// Parse `git version 2.55.0.windows.3` — and anything else git emits — into a triple.
///
/// Deliberately tolerant about what follows the patch component (`.windows.3`, `-rc1`, a git-for-
/// Windows suffix) and strict about what precedes it: a string with no `N.N` at all is unparsable
/// and must not silently read as `0.0.0`, which would compare below any floor and refuse a healthy
/// toolchain (golden constraint 9 — a guard scoped wrong refuses valid work).
#[must_use]
pub fn parse_version(s: &str) -> Option<VersionTriple> {
    let tail = s.split_whitespace().find(|t| {
        t.split('.')
            .next()
            .is_some_and(|h| !h.is_empty() && h.chars().all(|c| c.is_ascii_digit()))
    })?;
    let mut parts = tail.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    // A missing patch is not a parse failure: `git version 2.20` is a legitimate answer.
    let patch = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    Some(VersionTriple {
        major,
        minor,
        patch,
    })
}
