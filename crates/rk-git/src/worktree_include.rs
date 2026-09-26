//! The `.worktreeinclude` reader (s8c §8c.4; D21-45 W3).
//!
//! W3 reads: *"**Honour the project's existing `.worktreeinclude`**, the same file with the same
//! semantics (ignored ∩ matched, symlinks skipped), for files like `.env`."* This module is the
//! reader. The copy at run-worktree creation is `s8d-ii`'s (D21-46 R1); nothing here copies.
//!
//! - **An absent file carries nothing.** Claude Code carries *"nothing by default"* (D21-45), so
//!   an absent `.worktreeinclude` is the empty set, never an error.
//! - **Matched ∩ ignored, by git's own matcher.** Two reads, each `-z`, through [`GitExec`]:
//!   [`GitOp::LsFiles`] lists the untracked paths the file matches, with the
//!   `:(exclude).redkiln/worktrees` pathspec so another run's worktree under the root (D21-44) is
//!   never walked or carried; [`GitOp::CheckIgnore`] keeps the ones the standard rules ignore, and
//!   its exit 1 means *none*. A tracked file is never listed, so never carried. **No second
//!   gitignore matcher exists here**: two matchers would give one pattern file two meanings.
//! - **Symlinks are skipped and named**, and on Windows so is any other reparse point (a
//!   junction). They are never followed.
//! - **Escapes refuse the whole carry by name**: a listed path that is absolute, prefixed, carries
//!   a `..` component, or canonicalises outside the canonical repository root.
//! - **A pure core.** [`select_carry`] decides over lists already read, so the symlink and escape
//!   cases are tested without creating an OS symlink (a privilege this Windows box may not grant).
//!   The OS-level symlink case is not asserted by any test (s8c §8c.11).
//!
//! `LC_ALL=C` on these calls is D21-45 W1's rule and `s8d-ii`'s; the reads inherit whatever
//! [`GitExec`] sets.
//!
//! *(Added 2026-09-26, s8c build, P6.)*

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;

/// The pattern file, at the repository root.
pub const INCLUDE_FILE: &str = ".worktreeinclude";

/// The pathspec that keeps every run worktree under the root out of the listing (D21-44).
pub const RUN_WORKTREES_EXCLUDE: &str = ":(exclude).redkiln/worktrees";

/// What a candidate path is on disk, read with `symlink_metadata` (never followed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    /// A regular file, with its canonical path for the outside-the-root check.
    File { canonical: PathBuf },
    /// A symbolic link.
    Symlink,
    /// A Windows reparse point that is not reported as a symlink (a junction, say).
    ReparsePoint,
    /// Anything else, or a path that could not be read: a directory, a vanished file.
    NotAFile,
}

/// The on-disk facts [`select_carry`] decides over.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Kinds {
    /// The repository root, canonicalised.
    pub canonical_root: PathBuf,
    /// One entry per candidate path (matched ∩ ignored), keyed by the listed path.
    pub entries: BTreeMap<String, EntryKind>,
}

/// Why a candidate was skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkipReason {
    Symlink,
    ReparsePoint,
    NotAFile,
}

impl SkipReason {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Symlink => "symlink",
            Self::ReparsePoint => "reparse-point",
            Self::NotAFile => "not-a-regular-file",
        }
    }
}

/// A candidate that is not carried, named with its reason.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Skipped {
    pub path: String,
    pub reason: SkipReason,
}

/// What the carry copies, and what it skipped. Paths are repository-relative, `/`-separated as
/// git lists them, in byte order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CarrySet {
    pub carried: Vec<String>,
    pub skipped: Vec<Skipped>,
}

/// Why the whole carry is refused. Each arm names the offending path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarryRefusal {
    /// The path is absolute, or begins at a root directory.
    Absolute { path: String },
    /// The path carries a drive or UNC prefix.
    Prefixed { path: String },
    /// The path carries a `..` component.
    ParentComponent { path: String },
    /// The path canonicalises outside the canonical repository root.
    OutsideRoot { path: String, canonical: PathBuf },
    /// A candidate has no on-disk kind: the reads and the stat disagree.
    Unstatted { path: String },
    /// git listed a path that is not UTF-8.
    NonUtf8 { lossy: String },
}

impl CarryRefusal {
    /// The refusal's name, stable for reports.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Absolute { .. } => "carry-path-absolute",
            Self::Prefixed { .. } => "carry-path-prefixed",
            Self::ParentComponent { .. } => "carry-path-parent-component",
            Self::OutsideRoot { .. } => "carry-path-outside-root",
            Self::Unstatted { .. } => "carry-path-unstatted",
            Self::NonUtf8 { .. } => "carry-path-non-utf8",
        }
    }
}

impl std::fmt::Display for CarryRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute { path }
            | Self::Prefixed { path }
            | Self::ParentComponent { path }
            | Self::Unstatted { path } => write!(f, "{}: {path}", self.name()),
            Self::OutsideRoot { path, canonical } => {
                write!(f, "{}: {path} -> {}", self.name(), canonical.display())
            }
            Self::NonUtf8 { lossy } => write!(f, "{}: {lossy}", self.name()),
        }
    }
}

/// Refuse a listed path whose shape could name something outside the root.
fn lexical_escape(path: &str) -> Option<CarryRefusal> {
    let p = Path::new(path);
    // Components first, so a drive- or UNC-prefixed path is named as prefixed (on Windows
    // `C:/x` is also absolute), then the platform's own absoluteness as a backstop.
    for c in p.components() {
        match c {
            Component::Prefix(_) => return Some(CarryRefusal::Prefixed { path: path.into() }),
            Component::RootDir => return Some(CarryRefusal::Absolute { path: path.into() }),
            Component::ParentDir => {
                return Some(CarryRefusal::ParentComponent { path: path.into() });
            }
            Component::CurDir | Component::Normal(_) => {}
        }
    }
    if p.is_absolute() {
        return Some(CarryRefusal::Absolute { path: path.into() });
    }
    None
}

/// The pure core: from the matched list, the ignored list and the on-disk kinds, the carry.
///
/// Every matched path is checked for an escape first, whether or not it is ignored: one escaping
/// entry refuses the whole carry. Then each path that is both matched and ignored is carried if it
/// is a regular file inside the canonical root, skipped and named if it is a symlink, another
/// reparse point or not a regular file, and refused if it canonicalises outside the root.
///
/// # Errors
/// A [`CarryRefusal`] naming the first offending path.
pub fn select_carry(
    matched: &[String],
    ignored: &[String],
    kinds: &Kinds,
) -> Result<CarrySet, CarryRefusal> {
    for m in matched {
        if let Some(r) = lexical_escape(m) {
            return Err(r);
        }
    }
    let ignored: BTreeSet<&str> = ignored.iter().map(String::as_str).collect();
    let candidates: BTreeSet<&str> = matched
        .iter()
        .map(String::as_str)
        .filter(|m| ignored.contains(m))
        .collect();

    let mut out = CarrySet::default();
    for c in candidates {
        match kinds.entries.get(c) {
            None => return Err(CarryRefusal::Unstatted { path: c.into() }),
            Some(EntryKind::File { canonical }) => {
                if !canonical.starts_with(&kinds.canonical_root) {
                    return Err(CarryRefusal::OutsideRoot {
                        path: c.into(),
                        canonical: canonical.clone(),
                    });
                }
                out.carried.push(c.into());
            }
            Some(EntryKind::Symlink) => out.skipped.push(Skipped {
                path: c.into(),
                reason: SkipReason::Symlink,
            }),
            Some(EntryKind::ReparsePoint) => out.skipped.push(Skipped {
                path: c.into(),
                reason: SkipReason::ReparsePoint,
            }),
            Some(EntryKind::NotAFile) => out.skipped.push(Skipped {
                path: c.into(),
                reason: SkipReason::NotAFile,
            }),
        }
    }
    Ok(out)
}

/// Split git's `-z` output into paths, refusing a non-UTF-8 one by name.
fn split_z(bytes: &[u8]) -> Result<Vec<String>, CarryRefusal> {
    bytes
        .split(|b| *b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| {
            String::from_utf8(s.to_vec()).map_err(|_| CarryRefusal::NonUtf8 {
                lossy: String::from_utf8_lossy(s).into_owned(),
            })
        })
        .collect()
}

/// Is this metadata a Windows reparse point (`FILE_ATTRIBUTE_REPARSE_POINT`)?
#[cfg(windows)]
fn is_reparse_point(md: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt as _;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    md.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
const fn is_reparse_point(_md: &std::fs::Metadata) -> bool {
    false
}

/// Read one candidate's kind without following it.
fn kind_of(root: &Path, rel: &str) -> EntryKind {
    let full = root.join(rel);
    let Ok(md) = std::fs::symlink_metadata(&full) else {
        return EntryKind::NotAFile;
    };
    let ft = md.file_type();
    if ft.is_symlink() {
        return EntryKind::Symlink;
    }
    if is_reparse_point(&md) {
        return EntryKind::ReparsePoint;
    }
    if !ft.is_file() {
        return EntryKind::NotAFile;
    }
    std::fs::canonicalize(&full).map_or(EntryKind::NotAFile, |canonical| EntryKind::File {
        canonical,
    })
}

const fn refused(refusal: CarryRefusal) -> GitError {
    GitError::CarryRefused { refusal }
}

/// The carry set for the repository at `repo_root` (the module doc).
///
/// # Errors
/// [`GitError::CarryRefused`] naming the refusal; the wrapper's [`GitError`] if a read fails
/// (`check-ignore`'s exit 1 is *none ignored*, not a failure); [`GitError::Spawn`] if the root
/// cannot be canonicalised.
pub fn carry_set(exec: &GitExec, repo_root: &Path) -> Result<CarrySet, GitError> {
    match std::fs::symlink_metadata(repo_root.join(INCLUDE_FILE)) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(CarrySet::default()),
        Err(_) | Ok(_) => {}
    }
    let cwd = WorktreePath::new(repo_root)?;
    let exclude_from = format!("--exclude-from={INCLUDE_FILE}");
    let listed = exec.run(
        GitOp::LsFiles,
        &cwd,
        [
            "ls-files",
            "-z",
            "--others",
            "--ignored",
            exclude_from.as_str(),
            "--",
            ".",
            RUN_WORKTREES_EXCLUDE,
        ],
    )?;
    let matched = split_z(&listed.stdout).map_err(refused)?;
    if matched.is_empty() {
        return Ok(CarrySet::default());
    }

    let mut stdin: Vec<u8> = Vec::new();
    for m in &matched {
        stdin.extend_from_slice(m.as_bytes());
        stdin.push(0);
    }
    let ignored = match exec.run_with_stdin(
        GitOp::CheckIgnore,
        &cwd,
        ["check-ignore", "-z", "--stdin", "--no-index"],
        &stdin,
    ) {
        Ok(out) => split_z(&out.stdout).map_err(refused)?,
        // Exit 1: no path is ignored. An exit code, never stderr, decides this (ADR-0077:70).
        Err(GitError::NonZeroExit { code: Some(1), .. }) => Vec::new(),
        Err(e) => return Err(e),
    };

    let canonical_root = std::fs::canonicalize(repo_root).map_err(|e| GitError::Spawn {
        detail: format!("could not canonicalise {}: {e}", repo_root.display()),
    })?;
    let ignored_set: BTreeSet<&str> = ignored.iter().map(String::as_str).collect();
    let mut entries = BTreeMap::new();
    for m in &matched {
        // Only a lexically safe candidate is ever joined onto the root and stat'ed; an escaping
        // one is refused by `select_carry` before any entry is read.
        if ignored_set.contains(m.as_str()) && lexical_escape(m).is_none() {
            entries.insert(m.clone(), kind_of(repo_root, m));
        }
    }
    let kinds = Kinds {
        canonical_root,
        entries,
    };
    select_carry(&matched, &ignored, &kinds).map_err(refused)
}
