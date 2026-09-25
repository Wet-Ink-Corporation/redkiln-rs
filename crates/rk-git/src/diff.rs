//! `range_diff` — what changed over a commit range, read from the object store **through gix**
//! (ADR-0165; `s8a` §8a.2).
//!
//! `ChangeSummary`'s `Text` arm is built from this read: the changed paths, each path's change
//! kind, each path's text diff, and a mark on every path whose blob is a git-lfs pointer. It is a
//! **gix read**, and that is the point of its home here: ADR-0077's `## Decision` reads *"gix owns
//! the read path"*, and ADR-0091 decision 1 confirmed gix as the read owner at the pin. No
//! executable operation is added for it and nothing in this module spawns a process, so neither
//! ADR-0077 nor ADR-0105 clause 5 moves. The workspace pins `gix = "=0.87.1"` with default
//! features; its `basic` default carries `blob-diff`, which is what makes gix's tree diff and the
//! blob diff below available. *(Amended 2026-09-25, s8a fix round 7: this named
//! `Repository::diff_tree_to_tree`; step 4 says why the tree diff is now called directly.)*
//!
//! **How it reads.**
//!
//! 1. The range is `<base>..<head>`, two-dot, both ends non-empty. Anything else is refused by
//!    name ([`RangeError::Empty`], [`RangeError::Malformed`]): a single revision is not a range,
//!    and `...` (symmetric difference) means something else and is never read as `..`.
//! 2. The repository is opened AT the given directory ([`gix::open`]), never discovered upward:
//!    a project directory nested inside some other repository must not silently read that
//!    repository's history ([`RangeError::Open`]). Only a directory that is no repository is
//!    `Open`; a repository that is there but cannot be opened (a config that does not parse, an
//!    I/O fault, an unsafe or denied git dir) is [`RangeError::Read`] with gix's source chain.
//!    *(Amended 2026-09-25, s8a fix round 7.)*
//! 3. Each end is resolved to a commit and peeled to its tree ([`RangeError::Resolve`], naming the
//!    end that did not resolve). An end that resolves to an id whose commit or tree the store
//!    cannot read is [`RangeError::Read`] naming the id: a store fault, not an unresolvable end.
//!    *(Amended 2026-09-25, s8a fix round 4.)* An end reached by a `~N`/`^N` walk to a commit the
//!    store cannot read is the same `Read`, naming that commit; a walk past the root or a shallow
//!    boundary, or to a parent the commit does not have, stays `Resolve`. *(Amended 2026-09-25,
//!    s8a fix round 6: gix's ancestor walk failed before an id was returned, so it read `Resolve`.)*
//! 4. `gix::diff::tree` (gix-diff's tree diff) over the two trees, with a `Recorder` tracking full
//!    paths, gives the changed entries. Rename tracking is not done, so a rename is a deletion and
//!    an addition. Tree entries are dropped: a row is a FILE (a blob, a symlink, or a submodule
//!    commit).
//!    *(Amended 2026-09-25, s8a fix round 7: this was `Repository::diff_tree_to_tree`, which
//!    builds a blob resource cache from the checkout's `.git/index` even with `rewrites: None`, so
//!    an index under 20 bytes panicked inside gix-index and a damaged one failed a committed
//!    range. The read now touches the object store only, never the index.)*
//!    A changed path that is not UTF-8 is [`RangeError::Read`] naming its bytes: a row carries
//!    its path as a `String`, and a lossy conversion made two such paths one row naming neither.
//!    *(Amended 2026-09-25, s8a fix round 8.)*
//! 5. Each changed blob is read **by object id** at each end where it exists. That one read serves
//!    the LFS pointer check and the text diff.
//!
//! **The LFS pointer mark** (`s8a` §8a.2 item 2a). A path is an LFS pointer path when its blob at
//! EITHER end is under 1024 bytes and its first line is exactly
//! `version https://git-lfs.github.com/spec/v1` — the form git-lfs writes. It is decided on the
//! blob and not on `.gitattributes`, because an attribute can be absent at one end of a range or
//! unset locally, and because the object store holds the pointer whatever the checkout's smudge
//! configuration is: the diff of such a path is the pointer's `oid`/`size` delta and says nothing
//! about the content. This module only MARKS the path; what a consumer does with the mark (the
//! Rust adapter refuses, because the `Text` arm must never carry a pointer delta) is the
//! consumer's.
//!
//! **The text diff.** A unified diff per path with three lines of context, in git's header shape
//! (`diff --git a/<p> b/<p>`, `old mode`/`new mode` when the mode changed, `new file mode` /
//! `deleted file mode` for an added or deleted path, a type change as git's deletion-then-addition
//! pair, `--- a/<p>` or
//! `--- /dev/null`, `+++ b/<p>` or `+++ /dev/null` when there is a hunk, and git's own hunk
//! header: a one-line side's `,1` left out, a no-line side's start written as the line before it),
//! computed by gix's blob diff (histogram, with git's slider heuristics). A side whose bytes carry
//! a NUL in the first 8000 bytes, or are not UTF-8, is not diffed as text: the path's diff is
//! git's `Binary files … differ` line, so the mark of a binary change is visible and no bytes are
//! mangled into text. The same test sets the entry's `binary` mark, so a consumer never has to
//! decide it from a working tree that may not hold either side. A side whose last line has no
//! final newline carries git's `\ No newline at end of file` line after that line, as git writes
//! it. *(Amended 2026-09-25, s8a fix round 1: the `binary` mark and the no-final-newline marker.)*
//! *(Amended 2026-09-25, s8a fix round 2: the mode lines, the `---`/`+++` pair only over a hunk,
//! and git's hunk header in place of gix's `HunkHeader` display. No `index` line is written.)*
//! *(Amended 2026-09-25, s8a fix round 3: the `new file mode`/`deleted file mode` lines and the
//! type change as a pair, so the concatenated diffs are a patch `git apply --index` takes.)*
//! *(Amended 2026-09-25, s8a fix round 5: a path git quotes is quoted as git's `quote_c_style`
//! does, with `core.quotePath`'s default: a control byte, `"`, `\` or a byte >= 0x80 puts the
//! whole `a/<p>` or `b/<p>` inside `"…"` with C escapes, on the `diff --git`, `---`, `+++` and
//! `Binary files` lines, and a `---`/`+++` name holding a space is followed by a tab, as git writes
//! it. Round 3's `git apply` claim held only for paths git does not quote. And the hunk header
//! carries git's DEFAULT section heading, the one git writes with no diff driver: the nearest
//! base-side line before the hunk whose first byte is an ASCII letter, `_` or `$`, cut to 80 bytes
//! and stripped of trailing whitespace. A `diff=<driver>` attribute's heading is not read, and a
//! cut that would split a UTF-8 character backs off to the character boundary where git would
//! write the partial bytes.)*

use std::path::Path;

use gix::diff::blob::unified_diff::{ConsumeHunk, ContextSize, DiffLineKind, HunkHeader};
use gix::diff::blob::{Algorithm, InternedInput, UnifiedDiff, diff_with_slider_heuristics};
use gix::diff::tree::recorder::Change as Recorded;
use gix::object::tree::EntryKind;

/// The first line of a git-lfs pointer file (the git-lfs specification, v1).
pub const LFS_POINTER_FIRST_LINE: &str = "version https://git-lfs.github.com/spec/v1";

/// A pointer blob is under this many bytes (the git-lfs specification's bound).
pub const LFS_POINTER_MAX_BYTES: usize = 1024;

/// git's own binary heuristic reads this many leading bytes for a NUL (git's `FIRST_FEW_BYTES`).
///
/// Public so a working-tree byte test (the adapters' `classify`) reads the same window, and a path's
/// answer never moves with the checkout. *(Made public 2026-09-25, s8a fix round 12.)* *(Amended
/// 2026-09-25, s8a fix round 14: one window was not enough, since a dirty tree or a checkout at a
/// third commit still moved `change_summary`'s answer. The Rust adapter's `change_summary` now
/// takes `classify`'s path rules only and decides the bytes on this read's `binary` mark, so the
/// window this constant shares serves the `classify` verb alone.)*
pub const BINARY_SNIFF_BYTES: usize = 8000;

/// How a path changed over the range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeKind {
    /// The path exists at the head end only.
    Added,
    /// The path exists at the base end only.
    Deleted,
    /// The path exists at both ends, with different content or mode.
    Modified,
}

/// One changed path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeEntry {
    /// The path, relative to the repository root, `/`-separated.
    pub path: String,
    /// How it changed.
    pub change: ChangeKind,
    /// Its blob at either end has the git-lfs pointer form (the module doc).
    pub lfs_pointer: bool,
    /// Its blob at either end is binary: a NUL in the leading bytes, or not UTF-8 (the module
    /// doc). Decided on the range's blobs, never on a working tree, so a binary blob deleted in the
    /// range or present only at a head the checkout is not on is still marked. *(Added 2026-09-25,
    /// s8a fix round 1.)*
    pub binary: bool,
    /// Its unified diff, headers included, or git's `Binary files … differ` line.
    pub diff: String,
}

/// What changed over a range: the two resolved ends and one entry per changed path, sorted by
/// path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeDiff {
    /// The base end's commit id, 40 hex.
    pub base: String,
    /// The head end's commit id, 40 hex.
    pub head: String,
    /// One entry per changed file.
    pub entries: Vec<RangeEntry>,
}

/// Why a range was refused. Each variant names what it could not read; gix's own messages are
/// carried as text for the record and never parsed for control flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeError {
    /// The range is empty or blank.
    Empty,
    /// The range is not `<base>..<head>` with both ends present.
    Malformed { range: String },
    /// No repository opens at this directory: gix finds none there (it is never discovered
    /// upward). A repository that is there but fails to open is `Read` (s8a fix round 7).
    Open { path: String, detail: String },
    /// An end of the range does not resolve to a commit.
    Resolve { rev: String, detail: String },
    /// A tree, a diff or a blob could not be read, or a repository that is there could not be
    /// opened (s8a fix round 7), or a changed path is not UTF-8 (s8a fix round 8).
    Read { detail: String },
}

impl std::fmt::Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "the range is empty: there is nothing to read"),
            Self::Malformed { range } => write!(
                f,
                "`{range}` is not a `<base>..<head>` range with both ends present"
            ),
            Self::Open { path, detail } => {
                write!(f, "no repository opens at {path}: {detail}")
            }
            Self::Resolve { rev, detail } => {
                write!(f, "`{rev}` does not resolve to a commit: {detail}")
            }
            Self::Read { detail } => write!(f, "the range could not be read: {detail}"),
        }
    }
}

impl std::error::Error for RangeError {}

/// Read what changed over `range` in the repository at `repo` (the module doc).
///
/// # Errors
///
/// [`RangeError`] naming what was refused: an empty or malformed range, a directory that is not a
/// repository, an end that does not resolve to a commit, or an object that could not be read.
pub fn range_diff(repo: &Path, range: &str) -> Result<RangeDiff, RangeError> {
    let (base_rev, head_rev) = split_range(range)?;
    // Only a directory that is no repository is `Open`. A repository that is there but cannot be
    // opened (its config does not parse, an I/O fault, an unsafe or denied git dir) is a store
    // fault, `Read` carrying gix's whole source chain (amended 2026-09-25, s8a fix round 7: every
    // open error was `Open` with the top-level text only, so a consumer answered a broken config
    // as a directory that is no repository).
    let repository = gix::open(repo).map_err(|e| match e {
        gix::open::Error::NotARepository { .. } => RangeError::Open {
            path: repo.display().to_string(),
            detail: e.to_string(),
        },
        other => RangeError::Read {
            detail: format!("open {}: {}", repo.display(), source_chain(&other)),
        },
    })?;
    let (base, base_tree) = resolve(&repository, base_rev)?;
    let (head, head_tree) = resolve(&repository, head_rev)?;

    // gix's tree diff over the two trees' own bytes, recorded with full paths. Not
    // `Repository::diff_tree_to_tree`: that builds a blob resource cache from the checkout's
    // `.git/index` even with rename tracking off, so an index under 20 bytes panicked inside
    // gix-index and a damaged one failed a committed range (amended 2026-09-25, s8a fix round 7).
    let mut recorder = gix::diff::tree::Recorder::default();
    gix::diff::tree(
        gix::objs::TreeRefIter::from_bytes(&base_tree.data, base_tree.id.kind()),
        gix::objs::TreeRefIter::from_bytes(&head_tree.data, head_tree.id.kind()),
        gix::diff::tree::State::default(),
        &repository.objects,
        &mut recorder,
    )
    .map_err(|e| RangeError::Read {
        detail: format!("the tree diff {base}..{head}: {e}"),
    })?;

    let mut entries = Vec::with_capacity(recorder.records.len());
    for change in recorder.records {
        let (path, old, new) = sides(change)?;
        if let Some(entry) = read_entry(&repository, path, old, new)? {
            entries.push(entry);
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(RangeDiff {
        base: base.to_string(),
        head: head.to_string(),
        entries,
    })
}

/// An error and every source under it, joined with `: `, so a detail names the file and line gix's
/// top-level message leaves out. *(Added 2026-09-25, s8a fix round 7.)*
fn source_chain(e: &dyn std::error::Error) -> String {
    let mut out = e.to_string();
    let mut next = e.source();
    while let Some(source) = next {
        out.push_str(": ");
        out.push_str(&source.to_string());
        next = source.source();
    }
    out
}

/// `<base>..<head>`, both ends non-empty, and never `...`.
fn split_range(range: &str) -> Result<(&str, &str), RangeError> {
    let trimmed = range.trim();
    if trimmed.is_empty() {
        return Err(RangeError::Empty);
    }
    let malformed = || RangeError::Malformed {
        range: range.to_owned(),
    };
    if trimmed.contains("...") {
        return Err(malformed());
    }
    let (base, head) = trimmed.split_once("..").ok_or_else(malformed)?;
    if base.is_empty() || head.is_empty() || head.contains("..") {
        return Err(malformed());
    }
    Ok((base, head))
}

/// Resolve one end to its commit id and tree.
fn resolve<'repo>(
    repository: &'repo gix::Repository,
    rev: &str,
) -> Result<(gix::ObjectId, gix::Tree<'repo>), RangeError> {
    let refused = |detail: String| RangeError::Resolve {
        rev: rev.to_owned(),
        detail,
    };
    // Only a rev that names no object, or names one that peels to no commit, is `Resolve`. An id
    // that resolved but whose commit or tree cannot be read is a store fault, `Read` naming the
    // id (amended 2026-09-25, s8a fix round 4: all four steps were `Resolve`, so a consumer
    // answered a missing tree as an unresolvable range).
    let unreadable =
        |what: &str, id: &dyn std::fmt::Display, e: &dyn std::fmt::Display| RangeError::Read {
            detail: format!("{what} {id} for `{rev}`: {e}"),
        };
    // gix walks a `~N`/`^N` end itself and fails before returning an id when a commit on the walk
    // is gone from the store; that failure is re-read by hand, so a store fault is `Read` naming
    // the missing commit and only a real non-resolution stays `Resolve` (amended 2026-09-25, s8a
    // fix round 6).
    let id = repository.rev_parse_single(rev).map_err(|e| {
        unreadable_on_walk(repository, rev).unwrap_or_else(|| refused(e.to_string()))
    })?;
    let object = id.object().map_err(|e| unreadable("object", &id, &e))?;
    let commit = object.peel_to_commit().map_err(|e| match e {
        gix::object::peel::to_kind::Error::NotFound { .. } => refused(e.to_string()),
        gix::object::peel::to_kind::Error::FindExistingObject(_) => {
            unreadable("an object peeled from", &id, &e)
        }
    })?;
    let tree = commit.tree().map_err(|e| {
        let tree_id = commit
            .tree_id()
            .map_or_else(|_| "?".to_owned(), |t| t.to_string());
        unreadable("tree", &tree_id, &e)
    })?;
    Ok((commit.id, tree))
}

/// A `~N`/`^N` end that gix could not resolve, walked again by hand only to tell a store fault
/// from a name that does not resolve (s8a fix round 6). `Some(Read)` naming the first commit on the
/// walk that the store cannot read; `None` otherwise (a parent the commit does not have, a walk
/// past the root or a shallow boundary, a suffix this reader does not walk), so the caller keeps
/// gix's `Resolve`.
fn unreadable_on_walk(repository: &gix::Repository, rev: &str) -> Option<RangeError> {
    // `:<path>` and `:/<regex>` take the rest of the spec: a trailing `~1` there is not a walk.
    if rev.contains(':') {
        return None;
    }
    // The trailing walk, read right to left: `(~|^)` then optional digits, repeated.
    let bytes = rev.as_bytes();
    let mut end = bytes.len();
    let mut steps = Vec::new();
    loop {
        let digits_start = bytes[..end]
            .iter()
            .rposition(|b| !b.is_ascii_digit())
            .map_or(0, |i| i + 1);
        let Some(op) = digits_start.checked_sub(1).map(|i| bytes[i]) else {
            break;
        };
        if op != b'~' && op != b'^' {
            break;
        }
        let n = if digits_start == end {
            1
        } else {
            rev[digits_start..end].parse::<usize>().ok()?
        };
        steps.push((op, n));
        end = digits_start - 1;
    }
    if steps.is_empty() || end == 0 {
        return None;
    }
    steps.reverse();
    let shallow = repository.shallow_commits().ok().flatten();
    let is_boundary = |id: &gix::ObjectId| shallow.as_ref().is_some_and(|s| s.contains(id));
    let unreadable = |id: &gix::ObjectId, e: &dyn std::fmt::Display| RangeError::Read {
        detail: format!("commit {id} on the walk of `{rev}`: {e}"),
    };
    let mut current = repository.rev_parse_single(&rev[..end]).ok()?.detach();
    for (op, n) in steps {
        let hops = if op == b'~' { n } else { usize::from(n > 0) };
        for _ in 0..hops {
            let commit = match repository.find_object(current) {
                Ok(object) => object.peel_to_commit().ok()?,
                Err(e) => return Some(unreadable(&current, &e)),
            };
            if is_boundary(&commit.id) {
                return None;
            }
            let nth = if op == b'~' { 0 } else { n - 1 };
            current = commit.parent_ids().nth(nth)?.detach();
        }
    }
    repository
        .find_object(current)
        .err()
        .map(|e| unreadable(&current, &e))
}

/// One side of a change: its mode and object id, or `None` where the path does not exist (or is a
/// tree) at that end.
type Side = Option<(gix::object::tree::EntryMode, gix::ObjectId)>;

/// A change as a `(path, base side, head side)` row. The recorder does no rename tracking, so a
/// rename is a deletion and an addition. *(Amended 2026-09-25, s8a fix round 7: the rows came from
/// `diff_tree_to_tree`'s `ChangeDetached`, whose `Rewrite` arm could not occur.)* A path that is not
/// UTF-8 is [`RangeError::Read`] naming its bytes, never converted lossily *(amended 2026-09-25,
/// s8a fix round 8)*.
fn sides(change: Recorded) -> Result<(String, Side, Side), RangeError> {
    let file = |mode: gix::object::tree::EntryMode, id| (!mode.is_tree()).then_some((mode, id));
    let (path, old, new) = match change {
        Recorded::Addition {
            path,
            entry_mode,
            oid,
            ..
        } => (path, None, file(entry_mode, oid)),
        Recorded::Deletion {
            path,
            entry_mode,
            oid,
            ..
        } => (path, file(entry_mode, oid), None),
        Recorded::Modification {
            path,
            previous_entry_mode,
            previous_oid,
            entry_mode,
            oid,
        } => (
            path,
            file(previous_entry_mode, previous_oid),
            file(entry_mode, oid),
        ),
    };
    // A path is carried only as the UTF-8 it is: BString's `Display` would turn each invalid byte
    // into U+FFFD, so two paths differing in one such byte would share a row naming neither
    // (s8a fix round 8, 2026-09-25).
    let path = String::from_utf8(path.into()).map_err(|e| RangeError::Read {
        detail: format!(
            "the path {:?} is not UTF-8, so no row can carry it",
            gix::bstr::BStr::new(e.as_bytes())
        ),
    })?;
    Ok((path, old, new))
}

/// Read one path's two sides by object id, mark it, and diff it. `None` for a row that is a tree
/// at both ends.
fn read_entry(
    repository: &gix::Repository,
    path: String,
    old: Side,
    new: Side,
) -> Result<Option<RangeEntry>, RangeError> {
    let change = match (&old, &new) {
        (None, None) => return Ok(None),
        (None, Some(_)) => ChangeKind::Added,
        (Some(_), None) => ChangeKind::Deleted,
        (Some(_), Some(_)) => ChangeKind::Modified,
    };
    let old_bytes = old
        .map(|(mode, id)| side_bytes(repository, &path, mode, id))
        .transpose()?;
    let new_bytes = new
        .map(|(mode, id)| side_bytes(repository, &path, mode, id))
        .transpose()?;
    let lfs_pointer = [&old, &new]
        .iter()
        .zip([&old_bytes, &new_bytes])
        .any(|(side, bytes)| {
            side.is_some_and(|(mode, _)| mode.is_blob())
                && bytes.as_deref().is_some_and(is_lfs_pointer)
        });
    let binary = [&old_bytes, &new_bytes]
        .iter()
        .any(|bytes| bytes.as_deref().is_some_and(|b| as_text(b).is_none()));
    let old_kind = old.map(|(mode, _)| mode.kind());
    let new_kind = new.map(|(mode, _)| mode.kind());
    let diff = match (old_kind, new_kind) {
        // A TYPE change (a blob becoming a symlink or a submodule, or back) is git's deletion of
        // the old side followed by an addition of the new, each under its own header.
        (Some(o), Some(n)) if file_type(o) != file_type(n) => {
            let mut pair = unified(&path, Some(o), None, old_bytes.as_deref(), None)?;
            pair.push_str(&unified(&path, None, Some(n), None, new_bytes.as_deref())?);
            pair
        }
        _ => unified(
            &path,
            old_kind,
            new_kind,
            old_bytes.as_deref(),
            new_bytes.as_deref(),
        )?,
    };
    Ok(Some(RangeEntry {
        path,
        change,
        lfs_pointer,
        binary,
        diff,
    }))
}

/// A side's bytes: the blob read by id, or git's `Subproject commit <id>` line for a submodule.
fn side_bytes(
    repository: &gix::Repository,
    path: &str,
    mode: gix::object::tree::EntryMode,
    id: gix::ObjectId,
) -> Result<Vec<u8>, RangeError> {
    if mode.is_commit() {
        return Ok(format!("Subproject commit {id}\n").into_bytes());
    }
    let blob = repository.find_blob(id).map_err(|e| RangeError::Read {
        detail: format!("blob {id} for {path}: {e}"),
    })?;
    Ok(blob.data.clone())
}

/// The git-lfs pointer form: under [`LFS_POINTER_MAX_BYTES`], first line
/// [`LFS_POINTER_FIRST_LINE`].
#[must_use]
pub fn is_lfs_pointer(bytes: &[u8]) -> bool {
    if bytes.len() >= LFS_POINTER_MAX_BYTES {
        return false;
    }
    let first = bytes.split(|b| *b == b'\n').next().unwrap_or_default();
    let first = first.strip_suffix(b"\r").unwrap_or(first);
    first == LFS_POINTER_FIRST_LINE.as_bytes()
}

/// Text, as git decides it: no NUL in the leading bytes, and valid UTF-8.
fn as_text(bytes: &[u8]) -> Option<&str> {
    if bytes.iter().take(BINARY_SNIFF_BYTES).any(|b| *b == 0) {
        return None;
    }
    std::str::from_utf8(bytes).ok()
}

/// The file type git's diff compares for a type change: a blob (executable or not), a symlink or
/// a submodule. A blob gaining or losing its executable bit is a mode change, not a type change.
/// *(Added 2026-09-25, s8a fix round 3.)*
const fn file_type(kind: EntryKind) -> EntryKind {
    match kind {
        EntryKind::BlobExecutable => EntryKind::Blob,
        other => other,
    }
}

/// The per-path unified diff, git's header shape, three lines of context. A path whose mode
/// changed carries git's `old mode <o>` / `new mode <n>` lines after the `diff --git` line, and a
/// diff with no hunk (a mode-only change) carries no `---`/`+++` pair, as git writes neither.
/// *(Amended 2026-09-25, s8a fix round 2.)* An added path carries git's `new file mode <m>` line
/// and a deleted one `deleted file mode <m>`, so an added or deleted empty file still says what
/// happened and an added executable or symlink keeps its mode; `old_kind`/`new_kind` are `None`
/// at the end where the path does not exist, and the caller splits a type change into a deletion
/// and an addition before it gets here. *(Amended 2026-09-25, s8a fix round 3.)*
fn unified(
    path: &str,
    old_kind: Option<EntryKind>,
    new_kind: Option<EntryKind>,
    old: Option<&[u8]>,
    new: Option<&[u8]>,
) -> Result<String, RangeError> {
    let (a_name, b_name) = (git_quoted("a/", path), git_quoted("b/", path));
    let a = if old.is_some() {
        a_name.clone()
    } else {
        "/dev/null".to_owned()
    };
    let b = if new.is_some() {
        b_name.clone()
    } else {
        "/dev/null".to_owned()
    };
    let mut out = format!("diff --git {a_name} {b_name}\n");
    match (old_kind, new_kind) {
        (None, Some(n)) => out.push_str(&format!("new file mode {}\n", n.as_octal_str())),
        (Some(o), None) => out.push_str(&format!("deleted file mode {}\n", o.as_octal_str())),
        (Some(o), Some(n)) if o != n => out.push_str(&format!(
            "old mode {}\nnew mode {}\n",
            o.as_octal_str(),
            n.as_octal_str()
        )),
        _ => {}
    }
    let old_text = old.map(as_text);
    let new_text = new.map(as_text);
    if matches!(old_text, Some(None)) || matches!(new_text, Some(None)) {
        out.push_str(&format!("Binary files {a} and {b} differ\n"));
        return Ok(out);
    }
    let before = old_text.flatten().unwrap_or("");
    let after = new_text.flatten().unwrap_or("");
    let input = InternedInput::new(before, after);
    let computed = diff_with_slider_heuristics(Algorithm::Histogram, &input);
    let hunks = UnifiedDiff::new(
        &computed,
        &input,
        GitHunks {
            out: String::new(),
            before: before.split_inclusive('\n').collect(),
        },
        ContextSize::symmetrical(3),
    )
    .consume()
    .map_err(|e| RangeError::Read {
        detail: format!("the text diff of {path}: {e}"),
    })?;
    if !hunks.is_empty() {
        // git follows a `---`/`+++` name holding a space with a tab (`diff.c`).
        let tab = |name: &str| if name.contains(' ') { "\t" } else { "" };
        out.push_str(&format!("--- {a}{}\n+++ {b}{}\n", tab(&a), tab(&b)));
        out.push_str(&hunks);
    }
    Ok(out)
}

/// git's hunk body: each line prefixed, and a line that ends without a newline (a side's last
/// line when that side has no final newline) followed by `\ No newline at end of file`. `before`
/// is the base side's lines, each with its terminator, for the section heading. *(Amended
/// 2026-09-25, s8a fix round 5.)*
struct GitHunks<'a> {
    out: String,
    before: Vec<&'a str>,
}

/// `prefix` + `path` as git's `quote_c_style` writes it under `core.quotePath`'s default: when no
/// byte needs quoting, as is; otherwise inside `"…"`, with `\a \b \t \n \v \f \r \" \\` for those
/// bytes and a three-digit octal escape for any other control byte, DEL, or byte >= 0x80.
/// *(Added 2026-09-25, s8a fix round 5.)*
fn git_quoted(prefix: &str, path: &str) -> String {
    let joined = format!("{prefix}{path}");
    let needs = |b: u8| b < 0x20 || b == b'"' || b == b'\\' || b >= 0x7f;
    if !joined.bytes().any(needs) {
        return joined;
    }
    let mut out = String::from("\"");
    for b in joined.bytes() {
        match b {
            0x07 => out.push_str("\\a"),
            0x08 => out.push_str("\\b"),
            b'\t' => out.push_str("\\t"),
            b'\n' => out.push_str("\\n"),
            0x0b => out.push_str("\\v"),
            0x0c => out.push_str("\\f"),
            b'\r' => out.push_str("\\r"),
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            _ if needs(b) => out.push_str(&format!("\\{b:03o}")),
            _ => out.push(char::from(b)),
        }
    }
    out.push('"');
    out
}

/// git's default section heading for a hunk whose base side starts at 1-based line `start`: the
/// nearest earlier base-side line whose first byte is an ASCII letter, `_` or `$` (xdiff's
/// `def_ff`), cut to 80 bytes and stripped of trailing whitespace; `None` when there is none.
/// *(Added 2026-09-25, s8a fix round 5.)*
fn section_heading<'a>(before: &[&'a str], start: u32) -> Option<&'a str> {
    let first = usize::try_from(start).ok()?.checked_sub(1)?;
    before[..first.min(before.len())]
        .iter()
        .rev()
        .find_map(|line| {
            let lead = *line.as_bytes().first()?;
            if !(lead.is_ascii_alphabetic() || lead == b'_' || lead == b'$') {
                return None;
            }
            let mut cut = line.len().min(80);
            while !line.is_char_boundary(cut) {
                cut -= 1;
            }
            Some(line[..cut].trim_end_matches([' ', '\t', '\n', '\r']))
        })
}

/// git's hunk header, not gix's `HunkHeader` display (always `@@ -{},{} +{},{} @@`): a side of
/// one line leaves out its `,1`, and a side of no lines writes the line BEFORE it (0 for an added
/// or deleted file), as xdiff's `xdl_emit_hunk_hdr` does. *(Added 2026-09-25, s8a fix round 2.)*
fn git_hunk_header(header: &HunkHeader) -> String {
    let side = |start: u32, len: u32| match len {
        0 => format!("{},0", start.saturating_sub(1)),
        1 => format!("{start}"),
        _ => format!("{start},{len}"),
    };
    format!(
        "@@ -{} +{} @@",
        side(header.before_hunk_start, header.before_hunk_len),
        side(header.after_hunk_start, header.after_hunk_len)
    )
}

impl ConsumeHunk for GitHunks<'_> {
    type Out = String;

    fn consume_hunk(
        &mut self,
        header: HunkHeader,
        lines: &[(DiffLineKind, &[u8])],
    ) -> std::io::Result<()> {
        let head = git_hunk_header(&header);
        self.out.push_str(&head);
        if let Some(heading) =
            section_heading(&self.before, header.before_hunk_start).filter(|h| !h.is_empty())
        {
            // xdiff's header buffer is 128 bytes, its newline included.
            let mut cut = heading.len().min(126usize.saturating_sub(head.len()));
            while !heading.is_char_boundary(cut) {
                cut -= 1;
            }
            self.out.push(' ');
            self.out.push_str(&heading[..cut]);
        }
        self.out.push('\n');
        for &(kind, content) in lines {
            let text = std::str::from_utf8(content).map_err(std::io::Error::other)?;
            self.out.push(kind.to_prefix());
            self.out.push_str(text);
            if !text.ends_with('\n') {
                self.out.push_str("\n\\ No newline at end of file\n");
            }
        }
        Ok(())
    }

    fn finish(self) -> Self::Out {
        self.out
    }
}
