//! **The landing primitive** — how a kernel-made content checkpoint commit reaches
//! `rk-run/<item>` (the landing record, ADR-0188, proposed; `s8f` §8f.4; D21-41 (b); D21-42 9;
//! D21-43 N10). *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)*
//!
//! Two calls, because a `Gate:` value carries its tree hash and the tree must therefore be known
//! before the commit exists, and be **that commit's** tree (ADR-0105 clause 4):
//!
//! 1. [`stage`] reads the **staging set** ([`staging_set`], the ONE function, which `s8d-ii`'s
//!    re-entry discard also names its paths through), checks every path against the refusals
//!    ([`DENYLIST`], [`MAX_LANDED_BLOB_BYTES`], the worst-case path bound), and — when nothing is
//!    refused — stages onto a per-item **temporary index** under the common dir (`read-tree HEAD`,
//!    `add -u`, `add -- <run-created>`) and derives **T** with `write-tree`. A refusal writes
//!    nothing that any ref reaches, and carries clause 4's no-checkpoint-due tree for the gate.
//! 2. [`commit`] composes the message through `interpret-trailers --in-place`, runs `commit-tree T
//!    -p <tip>`, **re-reads** the new object's tree (it must equal T, or no ref moves), moves
//!    `refs/heads/rk-run/<item>` with its old value, and re-syncs the worktree's own index with
//!    `read-tree HEAD`.
//!
//! **Every invocation goes through the journalled seam** ([`GitOps`]), reads included, so on replay
//! T and the commit are served from the journal and `rk-git` performs **zero** writes (clause 1).
//! The filesystem writes this module makes itself (the temporary index's directory, the message
//! file) are skipped on replay for the same reason.
//!
//! `GitOp::CommitTree`, `GitOp::UpdateRef` and `GitOp::ResyncIndex` are named on code lines only
//! here (AC-518's index-change and ref halves; the widened lint
//! `tier_one::every_run_commit_resolves_through_the_worktree_resolver`).

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::error::GitError;
use crate::git_exec::GitOp;
use crate::gitops::{GitOps, Mode};
use crate::path::WorktreePath;
use crate::trailer::grammar::Trailer;
use crate::tree::TreeBasis;

/// **The denylist** (ADR-0188 decision 5), matched on a path's FINAL component, ASCII
/// case-insensitively, `*` matching any run of characters. Halt 7: never narrowed or widened to
/// make a run pass; the `exclude` answer is the remedy for a false hit such as `id_map.rs`.
pub const DENYLIST: [&str; 17] = [
    ".env",
    ".env.*",
    "*.pem",
    "*.key",
    "id_*",
    "credentials*",
    "*.p12",
    "*.pfx",
    ".netrc",
    ".pgpass",
    ".git-credentials",
    ".npmrc",
    ".pypirc",
    "*.ppk",
    "*.jks",
    "*.keystore",
    "*.kdbx",
];

/// **The size limit** (ADR-0188 decision 5): 2 MiB. A staged blob LARGER than this is refused; a
/// blob of exactly this size lands. Warrant: the largest tracked blob is 1,439,568 bytes (`s8f`
/// §8f.2 item 7, measured at `37bba5c`), so the limit admits every blob the repository tracks with
/// a 1.46x margin.
pub const MAX_LANDED_BLOB_BYTES: u64 = 2_097_152;

/// What the run's worktree held untracked at creation (ADR-0188 decision 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Baseline {
    /// The journalled set: an untracked path in it was not created by the run.
    Known(BTreeSet<String>),
    /// No baseline is recorded (a reused tree after a deleted database): fail-closed, every
    /// untracked non-ignored path is refused by name as `baseline-unknown`.
    Unknown,
}

/// One entry of `status --porcelain=v1 -z`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusEntry {
    /// The two status letters.
    pub xy: [u8; 2],
    /// The path (the NEW path of a rename or copy).
    pub path: String,
    /// A rename's or copy's original path.
    pub orig: Option<String>,
}

impl StatusEntry {
    /// Untracked (`??`).
    #[must_use]
    pub const fn is_untracked(&self) -> bool {
        self.xy[0] == b'?' && self.xy[1] == b'?'
    }

    /// Ignored (`!!`): listed only when the read asks for ignored files.
    #[must_use]
    pub const fn is_ignored(&self) -> bool {
        self.xy[0] == b'!' && self.xy[1] == b'!'
    }

    /// A deletion in the index or the worktree.
    #[must_use]
    pub const fn is_deletion(&self) -> bool {
        self.xy[0] == b'D' || self.xy[1] == b'D'
    }
}

/// Parse `status --porcelain=v1 -z` output. A rename or copy names two paths.
#[must_use]
pub fn parse_status(stdout: &[u8]) -> Vec<StatusEntry> {
    let mut fields = stdout.split(|b| *b == 0).filter(|f| !f.is_empty());
    let mut out = Vec::new();
    while let Some(f) = fields.next() {
        if f.len() < 4 {
            continue;
        }
        let (xy, path) = f.split_at(3);
        let xy = [xy[0], xy[1]];
        let orig = if matches!(xy[0], b'R' | b'C') || matches!(xy[1], b'R' | b'C') {
            fields
                .next()
                .map(|o| String::from_utf8_lossy(o).into_owned())
        } else {
            None
        };
        out.push(StatusEntry {
            xy,
            path: String::from_utf8_lossy(path).into_owned(),
            orig,
        });
    }
    out
}

/// **The staging set** (ADR-0188 decision 4), and what it was computed from.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StagingSet {
    /// Tracked modifications and deletions (a rename names both paths), sorted.
    pub tracked: Vec<String>,
    /// The tracked paths among them that are deletions (exempt from the size check only).
    pub deleted: BTreeSet<String>,
    /// The files the run created: untracked, not ignored, not in the baseline. Sorted.
    pub created: Vec<String>,
    /// Every untracked, non-ignored path the read saw — the baseline's own read at creation.
    pub untracked: Vec<String>,
    /// Paths an `exclude` answer leaves unstaged, named every time (ADR-0188 decision 6).
    pub excluded: Vec<String>,
}

impl StagingSet {
    /// Every path a landing would stage: the tracked paths and the run-created ones, sorted.
    #[must_use]
    pub fn paths(&self) -> Vec<String> {
        let all: BTreeSet<&String> = self.tracked.iter().chain(&self.created).collect();
        all.into_iter().cloned().collect()
    }

    /// Nothing to stage.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tracked.is_empty() && self.created.is_empty()
    }
}

/// **The staging set, read through the seam** (ADR-0188 decisions 4 and 13): the `status` read,
/// then [`classify`], the ONE function that computes the set. *(dated 2026-09-29, s8f build,
/// package R1FIX-8f-A-CODE: the classification moved into [`classify`], which the re-entry
/// discard in `worktree.rs` calls over its own `status` read — the read is not shared, since the
/// landing reads through `GitOps` and the discard through `GitExec`; the set is.)*
///
/// # Errors
/// [`GitError`] from the seam's `status` read.
pub fn staging_set(
    git: &mut GitOps<'_>,
    cwd: &WorktreePath,
    baseline: &Baseline,
    excluded: &BTreeSet<String>,
) -> Result<StagingSet, GitError> {
    let out = git.run(
        GitOp::Status,
        cwd,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    Ok(classify(parse_status(&out.stdout), baseline, excluded))
}

/// **The ONE staging-set function** (ADR-0188 decisions 4 and 13), pure over parsed `status`
/// entries: tracked modifications and deletions (a rename names both paths), plus the untracked
/// files that are not in `baseline`, honouring `.gitignore`, `info/exclude` and the global
/// excludes (an ignored entry is never a candidate), less `excluded`, which it names. With
/// [`Baseline::Unknown`] every untracked path is a candidate, and [`stage`] refuses it. The
/// landing stages from it ([`staging_set`]) and `s8d-ii`'s re-entry discards exactly it
/// (`worktree.rs`'s `discard`). *(Added 2026-09-29, s8f build, package R1FIX-8f-A-CODE.)*
#[must_use]
pub fn classify(
    entries: Vec<StatusEntry>,
    baseline: &Baseline,
    excluded: &BTreeSet<String>,
) -> StagingSet {
    let mut tracked = BTreeSet::new();
    let mut deleted = BTreeSet::new();
    let mut created = BTreeSet::new();
    let mut untracked = BTreeSet::new();
    let mut named = BTreeSet::new();
    for entry in entries {
        if entry.is_ignored() {
            continue;
        }
        if entry.is_untracked() {
            untracked.insert(entry.path.clone());
            let in_baseline = match baseline {
                Baseline::Known(set) => set.contains(&entry.path),
                Baseline::Unknown => false,
            };
            if in_baseline {
                continue;
            }
            if excluded.contains(&entry.path) {
                named.insert(entry.path);
            } else {
                created.insert(entry.path);
            }
            continue;
        }
        let mut paths = vec![(entry.path.clone(), entry.is_deletion())];
        if let Some(orig) = entry.orig.clone() {
            paths.push((orig, true));
        }
        for (path, gone) in paths {
            if excluded.contains(&path) {
                named.insert(path);
                continue;
            }
            if gone {
                deleted.insert(path.clone());
            }
            tracked.insert(path);
        }
    }
    StagingSet {
        tracked: tracked.into_iter().collect(),
        deleted,
        created: created.into_iter().collect(),
        untracked: untracked.into_iter().collect(),
        excluded: named.into_iter().collect(),
    }
}

/// Why a path is refused (ADR-0188 decision 6's spellings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// The final component matches this [`DENYLIST`] glob.
    Denylist(&'static str),
    /// The staged blob is this many bytes, over [`MAX_LANDED_BLOB_BYTES`].
    Size(u64),
    /// ADR-0180's worst-case bound, computed with this path, is this many characters, over 259,
    /// and `core.longpaths` is not `true`.
    PathLength(usize),
    /// No baseline is journalled, so nothing can say whether the run created this path.
    BaselineUnknown,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Denylist(glob) => write!(f, "denylist:{glob}"),
            Self::Size(bytes) => write!(f, "size:{bytes}"),
            Self::PathLength(bound) => write!(f, "path-length:{bound}"),
            Self::BaselineUnknown => f.write_str("baseline-unknown"),
        }
    }
}

/// One refused path and the rule it broke — a refusal is by name, never a silent drop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub path: String,
    pub rule: Rule,
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.path, self.rule)
    }
}

/// The [`DENYLIST`] glob `path`'s final component matches, if any.
#[must_use]
pub fn denylisted(path: &str) -> Option<&'static str> {
    let name = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
    DENYLIST
        .iter()
        .copied()
        .find(|glob| glob_matches(glob, &name))
}

/// `*`-only glob match over ASCII-lowercased text.
fn glob_matches(glob: &str, name: &str) -> bool {
    let parts: Vec<&str> = glob.split('*').collect();
    if parts.len() == 1 {
        return glob == name;
    }
    let (first, last) = (parts[0], parts[parts.len() - 1]);
    if !name.starts_with(first) || !name.ends_with(last) || name.len() < first.len() + last.len() {
        return false;
    }
    let mut rest = &name[first.len()..name.len() - last.len()];
    for mid in &parts[1..parts.len() - 1] {
        match rest.find(mid) {
            Some(i) => rest = &rest[i + mid.len()..],
            None => return false,
        }
    }
    true
}

/// What one landing is asked to land.
#[derive(Debug, Clone, Copy)]
pub struct Request<'a> {
    /// The run-owned worktree.
    pub cwd: &'a WorktreePath,
    /// The item, which names the run ref and the temporary index.
    pub item: &'a str,
    /// The journalled baseline.
    pub baseline: &'a Baseline,
    /// The paths the lane's `exclude` answers leave unstaged.
    pub excluded: &'a BTreeSet<String>,
}

/// A landing that passed every refusal: T is derived and nothing is committed yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prepared {
    cwd: WorktreePath,
    common_dir: PathBuf,
    item: String,
    tip: String,
    head_tree: String,
    tree: String,
    set: StagingSet,
}

impl Prepared {
    /// **T**: `write-tree` over the temporary index the staging set was staged onto.
    #[must_use]
    pub fn tree(&self) -> &str {
        &self.tree
    }

    /// `HEAD^{tree}` when the landing was staged.
    #[must_use]
    pub fn head_tree(&self) -> &str {
        &self.head_tree
    }

    /// The run ref's tip the commit will take as its parent.
    #[must_use]
    pub fn tip(&self) -> &str {
        &self.tip
    }

    /// **Due** (ADR-0188 decision 7): the landing passed every refusal AND T differs from
    /// `HEAD^{tree}`.
    #[must_use]
    pub fn due(&self) -> bool {
        self.tree != self.head_tree
    }

    /// What was staged.
    #[must_use]
    pub const fn set(&self) -> &StagingSet {
        &self.set
    }
}

/// What [`stage`] found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Staged {
    /// Every path passed; T is derived.
    Ready(Prepared),
    /// At least one path was refused: nothing is committed, and `tree` is clause 4's
    /// no-checkpoint-due derivation over the worktree, the refused landing's gate tree.
    Refused {
        refusals: Vec<Refusal>,
        tree: String,
        set: StagingSet,
    },
}

/// The run ref a landing moves.
#[must_use]
pub fn run_ref(item: &str) -> String {
    format!("refs/heads/rk-run/{item}")
}

/// The common dir, as an absolute path (git prints it relative to `cwd` for a main worktree).
fn common_dir(git: &mut GitOps<'_>, cwd: &WorktreePath) -> Result<PathBuf, GitError> {
    let out = git.run(GitOp::RevParse, cwd, &["rev-parse", "--git-common-dir"])?;
    let raw = PathBuf::from(out.stdout_trimmed());
    Ok(if raw.is_absolute() {
        raw
    } else {
        Path::new(cwd.as_str()).join(raw)
    })
}

/// The main worktree's directory, as `worktree list --porcelain` prints it (ADR-0180's `root`).
fn main_root(git: &mut GitOps<'_>, cwd: &WorktreePath) -> Result<String, GitError> {
    let out = git.run(
        GitOp::WorktreeList,
        cwd,
        &["worktree", "list", "--porcelain"],
    )?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    Ok(text
        .lines()
        .find_map(|l| l.strip_prefix("worktree "))
        .unwrap_or_default()
        .to_owned())
}

/// Is `core.longpaths` `true`? Exit 1 means *not set*.
fn longpaths(git: &mut GitOps<'_>, cwd: &WorktreePath) -> Result<bool, GitError> {
    match git.run(
        GitOp::Config,
        cwd,
        &["config", "--type=bool", "--get", "core.longpaths"],
    ) {
        Ok(out) => Ok(out.stdout_trimmed() == "true"),
        Err(GitError::NonZeroExit { code: Some(1), .. }) => Ok(false),
        Err(other) => Err(other),
    }
}

/// Lengths in UTF-16 code units, the unit Windows' `MAX_PATH` counts (ADR-0180's).
fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// The refusals on path NAMES, checked before any `add`: the denylist, the baseline, and the
/// worst-case path bound (ADR-0180 decision 16's formula and constant, the landed path in place of
/// the fork's longest).
fn name_refusals(
    git: &mut GitOps<'_>,
    req: &Request<'_>,
    set: &StagingSet,
) -> Result<Vec<Refusal>, GitError> {
    let mut refusals = Vec::new();
    let paths = set.paths();
    for path in &paths {
        if let Some(glob) = denylisted(path) {
            refusals.push(Refusal {
                path: path.clone(),
                rule: Rule::Denylist(glob),
            });
        }
    }
    if matches!(req.baseline, Baseline::Unknown) {
        for path in &set.created {
            refusals.push(Refusal {
                path: path.clone(),
                rule: Rule::BaselineUnknown,
            });
        }
    }
    if !paths.is_empty() {
        let root = utf16_len(&main_root(git, req.cwd)?);
        let item = utf16_len(req.item);
        let over: Vec<(String, usize)> = paths
            .iter()
            .map(|p| {
                (
                    p.clone(),
                    crate::worktree::path_bound(root, item, utf16_len(p)),
                )
            })
            .filter(|(_, bound)| *bound > crate::worktree::PATH_LIMIT)
            .collect();
        if !over.is_empty() && !longpaths(git, req.cwd)? {
            for (path, bound) in over {
                refusals.push(Refusal {
                    path,
                    rule: Rule::PathLength(bound),
                });
            }
        }
    }
    refusals.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(refusals)
}

/// **Stage a landing** (ADR-0188 decisions 4, 5 and 9 steps 1–2).
///
/// # Errors
/// [`GitError`] from the seam.
pub fn stage(git: &mut GitOps<'_>, req: &Request<'_>) -> Result<Staged, GitError> {
    let live = git.mode() == Mode::Live;
    let tip = git
        .run(GitOp::RevParse, req.cwd, &["rev-parse", "HEAD"])?
        .stdout_trimmed();
    let head_tree = git
        .run(GitOp::RevParse, req.cwd, &["rev-parse", "HEAD^{tree}"])?
        .stdout_trimmed();
    let common = common_dir(git, req.cwd)?;
    let set = staging_set(git, req.cwd, req.baseline, req.excluded)?;

    let refused = |git: &mut GitOps<'_>, refusals: Vec<Refusal>, set: StagingSet| {
        let tree = git.tree_hash(req.cwd, &common, &TreeBasis::WorkingState)?;
        Ok(Staged::Refused {
            refusals,
            tree: tree.as_str().to_owned(),
            set,
        })
    };

    // *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 minor.)* A name refusal no
    // longer returns before the size check: every path is checked (decision 5) and the gate names
    // every refused path (decision 6). A path refused by name is never staged, so its bytes never
    // enter the object store; the size check reads the rest.
    let by_name = name_refusals(git, req, &set)?;
    let named: BTreeSet<String> = by_name.iter().map(|r| r.path.clone()).collect();

    // Step 1: the temporary index, derived per item and never random (`tree.rs`: "The name is
    // derived, never random"); one worktree per concurrent item (Halt 6) makes it collision-free.
    let tmp_dir = common.join("redkiln").join("tmp");
    let index = tmp_dir.join(format!("landing-{}.index", req.item));
    if live {
        std::fs::create_dir_all(&tmp_dir).map_err(|e| GitError::Spawn {
            detail: format!("could not create the landing's temporary index dir: {e}"),
        })?;
        let _ = std::fs::remove_file(&index);
    }
    let env: Vec<(OsString, OsString)> = vec![
        ("GIT_INDEX_FILE".into(), index.clone().into_os_string()),
        // A path is a path, never a pathspec pattern: a file named `*.rs` stages itself only.
        ("GIT_LITERAL_PATHSPECS".into(), "1".into()),
    ];
    git.run_with_env(GitOp::Add, req.cwd, &["read-tree", "HEAD"], &env)?;
    let unnamed = |paths: &[String]| -> Vec<String> {
        paths
            .iter()
            .filter(|p| !named.contains(*p))
            .cloned()
            .collect()
    };
    let (tracked, created) = (unnamed(&set.tracked), unnamed(&set.created));
    if !tracked.is_empty() {
        let mut args = vec!["add", "-u", "--"];
        args.extend(tracked.iter().map(String::as_str));
        git.run_with_env(GitOp::Add, req.cwd, &args, &env)?;
    }
    if !created.is_empty() {
        let mut args = vec!["add", "--"];
        args.extend(created.iter().map(String::as_str));
        git.run_with_env(GitOp::Add, req.cwd, &args, &env)?;
    }

    // The size limit, on the STAGED blob (after `eol=lf` normalisation), before `write-tree`.
    let sized: Vec<String> = set
        .paths()
        .into_iter()
        .filter(|p| !set.deleted.contains(p) && !named.contains(p))
        .collect();
    let mut over = Vec::new();
    if !sized.is_empty() {
        let mut args = vec!["ls-files", "-s", "-z", "--"];
        args.extend(sized.iter().map(String::as_str));
        let listed = git.run_with_env(GitOp::LsFiles, req.cwd, &args, &env)?;
        for raw in listed.stdout.split(|b| *b == 0).filter(|f| !f.is_empty()) {
            let line = String::from_utf8_lossy(raw).into_owned();
            let Some((meta, path)) = line.split_once('\t') else {
                continue;
            };
            let Some(oid) = meta.split_whitespace().nth(1) else {
                continue;
            };
            let size: u64 = git
                .run(GitOp::CatFile, req.cwd, &["cat-file", "-s", oid])?
                .stdout_trimmed()
                .parse()
                .map_err(|_| GitError::VersionUnparsable {
                    output: format!("`cat-file -s {oid}` is not a size"),
                })?;
            if size > MAX_LANDED_BLOB_BYTES {
                over.push(Refusal {
                    path: path.to_owned(),
                    rule: Rule::Size(size),
                });
            }
        }
    }
    if !by_name.is_empty() || !over.is_empty() {
        if live {
            let _ = std::fs::remove_file(&index);
        }
        let mut all = by_name;
        all.extend(over);
        all.sort_by(|a, b| a.path.cmp(&b.path));
        return refused(git, all, set);
    }

    // Step 2: T.
    let tree = {
        let mut run = |op: GitOp, args: &[&str], env: &[(OsString, OsString)]| {
            git.run_with_env(op, req.cwd, args, env)
        };
        crate::tree::write_index_tree(&mut run, &env)?
            .as_str()
            .to_owned()
    };
    if live {
        let _ = std::fs::remove_file(&index);
    }
    Ok(Staged::Ready(Prepared {
        cwd: req.cwd.clone(),
        common_dir: common,
        item: req.item.to_owned(),
        tip,
        head_tree,
        tree,
        set,
    }))
}

/// **Prepare an empty landing**: stage nothing, and take `HEAD`'s tree as T, so [`commit`] with
/// `allow_empty` makes the trailer ride an empty commit. An answer's transition does not land
/// (ADR-0188 decision 14), and a refused landing's gate is empty (decision 7).
///
/// # Errors
/// [`GitError`] from the seam's reads.
pub fn prepare_empty(
    git: &mut GitOps<'_>,
    cwd: &WorktreePath,
    item: &str,
) -> Result<Staged, GitError> {
    let tip = git
        .run(GitOp::RevParse, cwd, &["rev-parse", "HEAD"])?
        .stdout_trimmed();
    let head_tree = git
        .run(GitOp::RevParse, cwd, &["rev-parse", "HEAD^{tree}"])?
        .stdout_trimmed();
    let common = common_dir(git, cwd)?;
    Ok(Staged::Ready(Prepared {
        cwd: cwd.clone(),
        common_dir: common,
        item: item.to_owned(),
        tip,
        tree: head_tree.clone(),
        head_tree,
        set: StagingSet::default(),
    }))
}

/// The commit message a landing writes: a subject, an optional body paragraph, and an optional
/// tier-1 trailer appended through `interpret-trailers` (`--if-exists=add`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub subject: String,
    pub body: Option<String>,
    pub trailer: Option<Trailer>,
}

/// What [`commit`] did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Landed {
    /// A commit exists on the run ref; `content` is whether its tree differs from its parent's.
    Commit {
        sha: String,
        tree: String,
        content: bool,
    },
    /// Not due and no empty commit asked for: nothing was committed and no ref moved.
    Nothing { tree: String },
}

/// **Commit a prepared landing** (ADR-0188 decision 9 steps 3–7). `allow_empty` is a boundary's:
/// a trailer rides an empty commit when no checkpoint is due. A step passes `false`, so a step
/// that changed nothing makes no commit.
///
/// # Errors
/// [`GitError`] from the seam; [`GitError::LandingReread`] when the new object's tree is not T,
/// in which case no ref has moved.
pub fn commit(
    git: &mut GitOps<'_>,
    prepared: &Prepared,
    message: &Message,
    allow_empty: bool,
) -> Result<Landed, GitError> {
    let due = prepared.due();
    if !due && !allow_empty {
        return Ok(Landed::Nothing {
            tree: prepared.head_tree.clone(),
        });
    }
    let tree = if due {
        prepared.tree.clone()
    } else {
        prepared.head_tree.clone()
    };
    let live = git.mode() == Mode::Live;
    let cwd = &prepared.cwd;

    // Step 3: the message, through `interpret-trailers --in-place` for the trailer.
    let msg_path = prepared
        .common_dir
        .join("redkiln")
        .join("tmp")
        .join(format!("landing-{}.msg", prepared.item));
    if live {
        let mut text = message.subject.clone();
        text.push('\n');
        if let Some(body) = &message.body {
            text.push('\n');
            text.push_str(body);
            text.push('\n');
        }
        if let Some(parent) = msg_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| GitError::Spawn {
                detail: format!("could not create the landing's message dir: {e}"),
            })?;
        }
        std::fs::write(&msg_path, text.as_bytes()).map_err(|e| GitError::Spawn {
            detail: format!("could not write the landing's message: {e}"),
        })?;
    }
    let msg = WorktreePath::new(&msg_path)?;
    if let Some(trailer) = &message.trailer {
        git.interpret_trailers_in_place(cwd, &msg, trailer)?;
    }

    // Step 4: `commit-tree T -p <tip>`, under the wrapper's empty `core.hooksPath` (clause 2).
    let sha = git
        .run(
            GitOp::CommitTree,
            cwd,
            &[
                "commit-tree",
                tree.as_str(),
                "-p",
                prepared.tip.as_str(),
                "-F",
                msg.as_str(),
            ],
        )?
        .stdout_trimmed();
    if live {
        let _ = std::fs::remove_file(&msg_path);
    }

    // Step 5: RE-READ the new object's tree; it must equal T, or no ref moves.
    let reread = git.tree_hash(
        cwd,
        &prepared.common_dir,
        &TreeBasis::CheckpointCommit(sha.clone()),
    )?;
    if reread.as_str() != tree {
        return Err(GitError::LandingReread {
            expected: tree,
            found: reread.as_str().to_owned(),
        });
    }

    // Step 6: the run ref, its old value guarding a race.
    let reference = run_ref(&prepared.item);
    git.run(
        GitOp::UpdateRef,
        cwd,
        &[
            "update-ref",
            reference.as_str(),
            sha.as_str(),
            prepared.tip.as_str(),
        ],
    )?;

    // Step 7: the worktree's OWN index agrees with the new tip.
    git.run(GitOp::ResyncIndex, cwd, &["read-tree", "HEAD"])?;

    Ok(Landed::Commit {
        sha,
        tree,
        content: due,
    })
}
