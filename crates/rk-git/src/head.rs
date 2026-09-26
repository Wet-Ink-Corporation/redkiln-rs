//! The one shared read-only `HEAD` read (s8c §8c.4; record V = ADR-0170 decisions 5 and 6,
//! proposed; D21-62 C, provisional).
//!
//! Two handlers in the Rust toolchain adapter read a repository's `HEAD` and nothing else: the
//! `sca_scan` handler reads each advisory database's commit before and after its tool runs (so the
//! recorded commit is provably the one the tool read, and a database that moved mid-scan is
//! refused), and the `package` handler reads the project root's `HEAD` to check it against the
//! input `sha` it was asked to build. D21-62 C: *"For each database, the adapter reads HEAD through
//! a single shared read-only rk_git head-read function built on gix."* This module is that
//! function, and Checkpoint 8a guard (6)'s `rk_git` allowlist admits it by a dated amendment
//! (`s8a`, 2026-09-26, s8c build).
//!
//! **A gix read, never a spawn** (ADR-0077: *"gix owns the read path"*). The repository is opened
//! AT the given directory ([`gix::open`]), never discovered upward: a directory that is not itself
//! a repository is refused, not answered with the commit of whatever repository encloses it. That
//! matters for both readers: a scan database path that is a plain directory inside some other
//! checkout must not borrow that checkout's commit, and neither may a project root.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::path::Path;

/// Why a `HEAD` could not be read. Each arm names the path it was read at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadError {
    /// The directory is not itself a git repository (gix's `NotARepository`).
    NotARepository { path: String, detail: String },
    /// The repository opened, but it could not be opened cleanly or its `HEAD` does not resolve
    /// to a commit (an unborn branch, a dangling ref, a store fault).
    Unresolved { path: String, detail: String },
}

impl std::fmt::Display for HeadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotARepository { path, detail } => {
                write!(f, "{path} is not a git repository: {detail}")
            }
            Self::Unresolved { path, detail } => {
                write!(f, "the HEAD of {path} does not resolve: {detail}")
            }
        }
    }
}

impl std::error::Error for HeadError {}

/// The 40-hex commit `HEAD` names in the repository opened AT `repo` (the module doc).
///
/// # Errors
///
/// [`HeadError::NotARepository`] when `repo` is not itself a repository;
/// [`HeadError::Unresolved`] when it opens with a fault or its `HEAD` names no commit.
pub fn head_commit(repo: &Path) -> Result<String, HeadError> {
    let path = repo.display().to_string();
    let repository = gix::open(repo).map_err(|e| match e {
        gix::open::Error::NotARepository { .. } => HeadError::NotARepository {
            path: path.clone(),
            detail: e.to_string(),
        },
        other => HeadError::Unresolved {
            path: path.clone(),
            detail: other.to_string(),
        },
    })?;
    let id = repository.head_id().map_err(|e| HeadError::Unresolved {
        path: path.clone(),
        detail: e.to_string(),
    })?;
    Ok(id.detach().to_hex().to_string())
}
