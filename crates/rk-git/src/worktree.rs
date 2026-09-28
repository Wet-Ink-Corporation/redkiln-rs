//! Worktree lifecycle and ownership (ADR-0105 clauses 6 and 8; AC-529).
//!
//! **Ownership is resolved from the store, never from cwd.** ADR-0018 said the kernel *"refuses to
//! write when the current worktree's HEAD is not the run's branch"*; ADR-0105 clause 8 narrows
//! "current worktree" to *"the worktree the store owns for the run"*. The difference is the whole
//! criterion: a process can be `cd`'d anywhere, and a rule keyed on cwd would authorise a write
//! from whichever directory the operator happened to be standing in.
//!
//! That is why [`RunOwnership`] is a **parameter** and not something this module looks up. There is
//! no call to `std::env::current_dir` in this crate, and AC-529's third instrument is the test that
//! says so — the one a naive implementation passes by accident, because on the happy path cwd and
//! the owned worktree are usually the same directory.
//!
//! ~~**Worktrees are re-created per run, never reused** (clause 6).~~ *(dated 2026-09-27, s8d-ii
//! build, P3: ADR-0180 decision 9, proposed, supersedes clause 6 in part **for the run-owned
//! worktree only**. The run tree is reused on re-entry by [`enter`] and removed only when clean,
//! landed, marked and unlocked by [`remove_landed`], unforced; `redkiln-migrate`'s worktree keeps
//! clause 6 whole through [`add`] and [`remove`], as AC-522 verifies it.)* A failed `worktree
//! remove` — on Windows typically a handle no `Stop-Process` releases, Halt 6 — is a typed error
//! and a `doctor` orphans row, never a retry into a reused tree, because reusing the tree corrupts
//! the next arm. A run tree that will not delete is kept and named, and never handed to another
//! item.

use std::path::{Path, PathBuf};

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp};
use crate::path::WorktreePath;

/// Which worktree and branch the **store** says this run owns.
///
/// Supplied by `rk-store` (§6.3). Passed in rather than looked up, so that no code path here can
/// fall back to cwd.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOwnership {
    pub worktree: PathBuf,
    pub branch: String,
}

/// A worktree a tier-1 write has been authorised against.
///
/// Like `Reconciled<State>`, the private field is the mechanism: it can only be produced by
/// [`authorise_tier1_write`], so "was this write checked?" is answered by the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedWorktree {
    path: WorktreePath,
    branch: String,
}

impl OwnedWorktree {
    #[must_use]
    pub const fn path(&self) -> &WorktreePath {
        &self.path
    }
    #[must_use]
    pub fn branch(&self) -> &str {
        &self.branch
    }
}

/// Authorise a tier-1 write against `target`, given what the store says the run owns.
///
/// Refuses when the target is not the run-owned worktree, **or** when its `HEAD` is not the run's
/// branch. The refusal names all four values, so the operator sees what was attempted and what was
/// expected rather than a bare denial (AC-529).
///
/// # Errors
/// [`GitError::ForeignWorktree`] on either refusal; propagates wrapper errors from reading `HEAD`.
pub fn authorise_tier1_write(
    exec: &GitExec,
    target: &Path,
    ownership: &RunOwnership,
) -> Result<OwnedWorktree, GitError> {
    let target_path = WorktreePath::new(target)?;
    let owned_path = WorktreePath::new(&ownership.worktree)?;

    let head = current_branch(exec, &target_path)?;

    // Compare on the normalised form: `D:\x` and `D:/x` are the same worktree, and a comparator
    // that said otherwise would refuse valid work.
    if target_path != owned_path || head != ownership.branch {
        return Err(GitError::ForeignWorktree {
            target_path: PathBuf::from(target_path.as_str()),
            target_branch: head,
            owned_path: PathBuf::from(owned_path.as_str()),
            owned_branch: ownership.branch.clone(),
        });
    }

    Ok(OwnedWorktree {
        path: target_path,
        branch: head,
    })
}

/// The branch `HEAD` currently points at, or `HEAD` when detached.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn current_branch(exec: &GitExec, at: &WorktreePath) -> Result<String, GitError> {
    let out = exec.run(GitOp::RevParse, at, ["rev-parse", "--abbrev-ref", "HEAD"])?;
    Ok(out.stdout_trimmed())
}

/// Create a worktree. **Re-created per run, never reused** (clause 6). *(dated 2026-09-27, s8d-ii
/// build, P3: this is `redkiln-migrate`'s, which keeps clause 6 whole; the run-owned worktree is
/// [`enter`]'s, ADR-0180 decision 9, proposed.)*
///
/// If `path` already exists this refuses rather than adopting it: an existing tree is either a
/// leftover from a killed run — which is a `doctor` orphans row, not a resource — or another run's,
/// and reusing either is what Halt 6 is about.
///
/// # Errors
/// Propagates [`GitError`] from the wrapper.
pub fn add(
    exec: &GitExec,
    repo: &WorktreePath,
    path: &Path,
    branch: &str,
) -> Result<WorktreePath, GitError> {
    let target = WorktreePath::new(path)?;
    if path.exists() {
        return Err(GitError::WorktreeRemoveFailed {
            path: path.to_path_buf(),
            stderr: b"a worktree already exists at this path; worktrees are re-created per run, \
                      never reused (ADR-0105 clause 6)"
                .to_vec(),
        });
    }
    exec.run(
        GitOp::WorktreeAdd,
        repo,
        ["worktree", "add", target.as_str(), branch],
    )?;
    Ok(target)
}

/// Remove a run-owned worktree.
///
/// A failure here is **typed and surfaced**, never swallowed and never retried into a reuse. The
/// caller records a `doctor` orphans row; S-CANCEL-WIN's fixture is the regression for the Windows
/// handle case, and it runs at 7c.
///
/// # Errors
/// [`GitError::WorktreeRemoveFailed`] carrying the path and git's stderr.
pub fn remove(exec: &GitExec, repo: &WorktreePath, path: &Path) -> Result<(), GitError> {
    let target = WorktreePath::new(path)?;
    match exec.run(
        GitOp::WorktreeRemove,
        repo,
        ["worktree", "remove", target.as_str()],
    ) {
        Ok(_) => Ok(()),
        Err(GitError::NonZeroExit { stderr, .. }) => Err(GitError::WorktreeRemoveFailed {
            path: path.to_path_buf(),
            stderr,
        }),
        Err(other) => Err(other),
    }
}

// ---- The one derivation, the item-id validator and the creation guards ----
//
// s8d-ii §8d-ii.1, §8d-ii.2 and §8d-ii.6 (D21-44; D21-45 W1; D21-47; D21-48 X1; ADR-0180
// decisions 1 to 4 and 16, proposed). *(dated 2026-09-27, s8d-ii build, P2.)*
//
// - **The root is the main worktree's directory**, the first record of `git worktree list
//   --porcelain`, whichever checkout the call starts in; a first record carrying `bare` is refused
//   by name. The run tree is `<main>/.redkiln/worktrees/<item>` on `rk-run/<item>`, and the review
//   reference tree `<item>.review-ref` beside it: [`derive`] is the ONE derivation, and every call
//   site and every test that plants a tree under the root calls it (§8d-ii.1).
// - **The item id is validated once** ([`ItemId::parse`]) as a path component and a branch
//   component; the case-fold rule reads the registry ([`refuse_case_fold_collision`]).
// - **Creation's guards** ([`prepare_creation`]): no symlinked or junctioned `.redkiln`,
//   `.redkiln/worktrees` or `<item>`; the root git-ignored in the main worktree (D21-48 X1,
//   *"Refuse by name."*); the worst-case path, computed on the actual item, within 259 or
//   `core.longpaths=true`. After `git worktree add`, [`verify_created`] compares canonical paths
//   only: a canonical `\\?\` form is never handed to git (ADR-0105's forbids bullet).
// - Every git call here is a READ (`worktree list`, `rev-parse`, `check-ignore`, `ls-tree`,
//   `config`); D21-47: *"A READ is not a supersession"*.

/// The longest item id the validator admits (W1: *"at most 64 characters"*).
pub const ITEM_ID_MAX: usize = 64;

/// The run branch's namespace: `rk-run/<item>`.
pub const RUN_BRANCH_PREFIX: &str = "rk-run/";

/// The review reference tree's suffix, `<item>.review-ref`, in the same directory as the run tree.
pub const REVIEW_REF_SUFFIX: &str = ".review-ref";

/// Windows' 260-character `MAX_PATH` leaves 259 usable (§8d-ii.0 item 2).
pub const PATH_LIMIT: usize = 259;

/// `/.redkiln/worktrees/`, the root's segments between the main worktree and the item: 20.
const ROOT_SEGMENTS_LEN: usize = 20;

/// The run-tree root as one anchored ignore row (D21-45 W4; `s8d-ii` §8d-ii.5).
///
/// It is the line `redkiln init` writes inside its `# redkiln-runtime` block in
/// `<commondir>/info/exclude`. It is declared here, beside [`derive`], so the row and the derivation have one home (D21-44), and
/// `init` and the test builders take it from here rather than spelling the root a second time.
pub const RUN_ROOT_IGNORE_ROW: &str = "/.redkiln/worktrees/";

// The row is the root's segments, so the path bound's 20 and the row cannot drift apart.
const _: () = assert!(RUN_ROOT_IGNORE_ROW.len() == ROOT_SEGMENTS_LEN);

/// The run-tree root as one pathspec that excludes it.
///
/// For a listing that must never walk another run's tree: `s8c`'s `.worktreeinclude` reader
/// (`worktree_include::RUN_WORKTREES_EXCLUDE` names this constant; D21-44). Declared here,
/// beside [`derive`] and [`RUN_ROOT_IGNORE_ROW`], so the
/// root is spelled in one module (`s8d-ii` §8d-ii.1; Checkpoint 8d-ii (3)'s root floor).
/// *(dated 2026-09-27, `s8d-ii` build, P6; ADR-0180 decision 1, proposed.)*
pub const RUN_ROOT_EXCLUDE_PATHSPEC: &str = ":(exclude).redkiln/worktrees";

/// The pathspec's root is the ignore row's, less its anchoring `/` and its trailing `/`: the two
/// spellings of the root cannot drift apart.
const fn pathspec_names_the_row_root() -> bool {
    const MAGIC: &[u8] = b":(exclude)";
    let spec = RUN_ROOT_EXCLUDE_PATHSPEC.as_bytes();
    let row = RUN_ROOT_IGNORE_ROW.as_bytes();
    if spec.len() != MAGIC.len() + row.len() - 2 {
        return false;
    }
    let mut i = 0;
    while i < MAGIC.len() {
        if spec[i] != MAGIC[i] {
            return false;
        }
        i += 1;
    }
    let mut j = 0;
    while j < row.len() - 2 {
        if spec[MAGIC.len() + j] != row[j + 1] {
            return false;
        }
        j += 1;
    }
    true
}
const _: () = assert!(pathspec_names_the_row_root());

/// Why an item id is refused as a path component and a branch component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemIdRefusal {
    Empty,
    TooLong,
    UnsafeCharacter,
    Reserved,
    LeadingDot,
    TrailingDot,
    DoubleDot,
    LockSuffix,
    DeviceName,
    ReviewRefSuffix,
}

impl ItemIdRefusal {
    /// The rule the id broke, stable for reports.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::TooLong => "longer-than-64",
            Self::UnsafeCharacter => "unsafe-character",
            Self::Reserved => "reserved-name",
            Self::LeadingDot => "leading-dot",
            Self::TrailingDot => "trailing-dot",
            Self::DoubleDot => "double-dot",
            Self::LockSuffix => "lock-suffix",
            Self::DeviceName => "windows-device-name",
            Self::ReviewRefSuffix => "review-ref-suffix",
        }
    }
}

/// A refusal of run-tree creation, **by name**: every arm names the path or the id it refused and
/// why, so the operator is never handed a bare denial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeRefusal {
    /// The item id is not a safe path component and branch component.
    UnsafeItemId { item: String, reason: ItemIdRefusal },
    /// The id case-folds onto a registered run tree or an existing `rk-run/` branch without
    /// equalling it: Windows' filesystem folds case and git's refs do not.
    CaseFoldCollision { item: String, existing: String },
    /// The first `worktree list --porcelain` record carries `bare`: there is no main worktree.
    BareRepository { repo: PathBuf },
    /// `.redkiln`, `.redkiln/worktrees` or `<item>` is a symlink or a junction.
    LinkedSegment { path: PathBuf, kind: &'static str },
    /// The created tree's canonical path does not lie under the canonical run-tree root.
    RealpathOutsideRoot {
        created: PathBuf,
        resolved: PathBuf,
        root: PathBuf,
    },
    /// `git check-ignore`, run in the main worktree, does not report the run tree as ignored.
    RootNotIgnored { main: PathBuf, path: String },
    /// The worst-case path, computed on this item, passes the limit and `core.longpaths` is not
    /// `true`.
    PathPastLimit {
        item: String,
        bound: usize,
        limit: usize,
        longest: usize,
        longest_path: String,
    },
    /// `git worktree list --porcelain` printed nothing this module can read (no record, a
    /// non-UTF-8 or quoted path), so the main worktree cannot be named.
    UnreadableWorktreeList { detail: String },
    /// A segment of the run tree's path could not be inspected (`symlink_metadata` or
    /// `canonicalize` failed for a reason other than absence).
    SegmentUnreadable { path: PathBuf, detail: String },
    // ---- the lifecycle's refusals (s8d-ii §8d-ii.2, §8d-ii.3; W1, W2). Each KEEPS the tree.
    /// A tree at the derived path carries no `RK_BASE` in its gitdir: redkiln did not create it.
    NoRkBase { path: PathBuf },
    /// `RK_BASE` names another item than the one whose path it sits at.
    RkBaseMismatch { path: PathBuf, recorded: String },
    /// Something exists at the derived path that is not a registered worktree of this
    /// repository: a checkout of another repository (its common dir named), or a plain directory.
    ForeignRepository {
        path: PathBuf,
        found: String,
        expected: String,
    },
    /// The tree's `HEAD` has commits the run ref lacks.
    CommitsRunRefLacks {
        path: PathBuf,
        branch: String,
        count: usize,
    },
    /// The tree's `HEAD` is detached: it can equal the tip and still not be on the run ref.
    DetachedHead { path: PathBuf },
    /// The tree's `HEAD` is symbolic on a ref other than the run ref (and adds no commit to it).
    OffRunRef {
        path: PathBuf,
        head: String,
        branch: String,
    },
    /// A live lock: its pid is alive and that pid's start time equals the one recorded.
    LockHeld { path: PathBuf, pid: u32, start: u64 },
    /// A lock redkiln did not write (no `redkiln run <item> (pid N start T)` reason): never
    /// released by redkiln.
    ForeignLock { path: PathBuf, reason: String },
    /// The discard left a status that is not empty.
    DiscardIncomplete {
        path: PathBuf,
        remaining: Vec<String>,
    },
    // ---- W4's refusals (s8d-ii §8d-ii.5; D21-47 V1; D21-48 X2). *(dated 2026-09-27, P4.)*
    /// V1 (a): a path at the fork commit selects `driver`, and no `filter.<driver>.process` or
    /// `.smudge` value resolves at any scope read (`scopes`), so git would check out the
    /// committed content, which for LFS is the pointer.
    FilterDriverNotEffective {
        driver: String,
        path: String,
        scopes: String,
    },
    /// V1 (b): smudge is skipped for `driver`, by a resolved `smudge` value carrying `--skip`
    /// (its `scope` and `origin` file) or by `GIT_LFS_SKIP_SMUDGE` (scope `environment`).
    SmudgeSkipped {
        driver: String,
        path: String,
        scope: String,
        origin: String,
    },
    /// D21-48 X2: after checkout, paths a selected driver names hold a Git LFS pointer. Boxed,
    /// so `GitError` stays small.
    PointerFilesAfterCheckout(Box<PointerHits>),
}

/// [`WorktreeRefusal::PointerFilesAfterCheckout`]'s facts.
///
/// `hits` paths `driver` selects hold a Git LFS pointer (`first` the first), although the
/// driver's effective value (`scope`, `origin`) was read. The fresh `tree` was removed unforced
/// when `removed`, else kept and named (`kept_because`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointerHits {
    pub tree: PathBuf,
    pub driver: String,
    pub hits: usize,
    pub first: String,
    pub scope: String,
    pub origin: String,
    pub removed: bool,
    pub kept_because: String,
}

impl WorktreeRefusal {
    /// The refusal's name, stable for reports.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::UnsafeItemId { .. } => "run-tree-item-id-unsafe",
            Self::CaseFoldCollision { .. } => "run-tree-item-id-case-fold-collision",
            Self::BareRepository { .. } => "run-tree-bare-repository",
            Self::LinkedSegment { .. } => "run-tree-linked-segment",
            Self::RealpathOutsideRoot { .. } => "run-tree-realpath-outside-root",
            Self::RootNotIgnored { .. } => "run-tree-root-not-ignored",
            Self::PathPastLimit { .. } => "run-tree-path-past-limit",
            Self::UnreadableWorktreeList { .. } => "run-tree-worktree-list-unreadable",
            Self::SegmentUnreadable { .. } => "run-tree-segment-unreadable",
            Self::NoRkBase { .. } => "run-tree-no-rk-base",
            Self::RkBaseMismatch { .. } => "run-tree-rk-base-mismatch",
            Self::ForeignRepository { .. } => "run-tree-foreign-repository",
            Self::CommitsRunRefLacks { .. } => "run-tree-commits-the-run-ref-lacks",
            Self::DetachedHead { .. } => "run-tree-detached-head",
            Self::OffRunRef { .. } => "run-tree-off-the-run-ref",
            Self::LockHeld { .. } => "run-tree-lock-held",
            Self::ForeignLock { .. } => "run-tree-foreign-lock",
            Self::DiscardIncomplete { .. } => "run-tree-discard-incomplete",
            Self::FilterDriverNotEffective { .. } => "run-tree-filter-driver-not-effective",
            Self::SmudgeSkipped { .. } => "run-tree-smudge-skipped",
            Self::PointerFilesAfterCheckout(_) => "run-tree-pointer-files-after-checkout",
        }
    }
}

impl std::fmt::Display for WorktreeRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsafeItemId { item, reason } => write!(
                f,
                "{}: the item id `{item}` is not a safe path and branch component ({})",
                self.name(),
                reason.name()
            ),
            Self::CaseFoldCollision { item, existing } => write!(
                f,
                "{}: the item id `{item}` case-folds onto `{existing}`, which is registered or \
                 has an rk-run/ branch; on a case-insensitive filesystem they would share a \
                 directory and not a branch",
                self.name()
            ),
            Self::BareRepository { repo } => write!(
                f,
                "{}: `{}` is a bare repository; it has no main worktree to root run trees in",
                self.name(),
                repo.display()
            ),
            Self::LinkedSegment { path, kind } => write!(
                f,
                "{}: `{}` is a {kind}; a linked segment could redirect run-tree creation \
                 outside the repository",
                self.name(),
                path.display()
            ),
            Self::RealpathOutsideRoot {
                created,
                resolved,
                root,
            } => write!(
                f,
                "{}: the created tree `{}` resolves to `{}`, which is not under the run-tree \
                 root `{}`",
                self.name(),
                created.display(),
                resolved.display(),
                root.display()
            ),
            Self::RootNotIgnored { main, path } => write!(
                f,
                "{}: `{path}` is not git-ignored in the main worktree `{}`; a run tree there \
                 would dirty the checkout (add `.redkiln/worktrees/` to .gitignore or run \
                 `redkiln init`)",
                self.name(),
                main.display()
            ),
            Self::PathPastLimit {
                item,
                bound,
                limit,
                longest,
                longest_path,
            } => write!(
                f,
                "{}: the worst-case path for `{item}` is {bound} characters, past the {limit} \
                 limit, and core.longpaths is not true (longest tracked path {longest}: \
                 `{longest_path}`)",
                self.name()
            ),
            Self::UnreadableWorktreeList { detail } => write!(
                f,
                "{}: `git worktree list --porcelain` could not be read ({detail}); the main \
                 worktree cannot be named",
                self.name()
            ),
            Self::SegmentUnreadable { path, detail } => write!(
                f,
                "{}: `{}` could not be inspected ({detail})",
                self.name(),
                path.display()
            ),
            Self::NoRkBase { path } => write!(
                f,
                "{}: the tree `{}` carries no RK_BASE marker in its gitdir; redkiln did not \
                 create it, so it is refused and kept, never removed",
                self.name(),
                path.display()
            ),
            Self::RkBaseMismatch { path, recorded } => write!(
                f,
                "{}: the tree `{}` carries an RK_BASE recording item `{recorded}`, not the item \
                 its path derives from; it is refused and kept",
                self.name(),
                path.display()
            ),
            Self::ForeignRepository {
                path,
                found,
                expected,
            } => write!(
                f,
                "{}: `{}` is not a registered worktree of this repository (its git common dir \
                 is `{found}`, this repository's is `{expected}`); it is refused and kept",
                self.name(),
                path.display()
            ),
            Self::CommitsRunRefLacks {
                path,
                branch,
                count,
            } => write!(
                f,
                "{}: the tree `{}` has {count} commit(s) that {branch} lacks; it is refused and \
                 kept, never removed",
                self.name(),
                path.display()
            ),
            Self::DetachedHead { path } => write!(
                f,
                "{}: the tree `{}` is on a detached HEAD, not on its run ref; it is refused at \
                 re-entry and kept",
                self.name(),
                path.display()
            ),
            Self::OffRunRef { path, head, branch } => write!(
                f,
                "{}: the tree `{}` has HEAD on `{head}`, not on refs/heads/{branch}; it is \
                 refused and kept",
                self.name(),
                path.display()
            ),
            Self::LockHeld { path, pid, start } => write!(
                f,
                "{}: the tree `{}` is locked by a live run (pid {pid} start {start}); this run \
                 is refused",
                self.name(),
                path.display()
            ),
            Self::ForeignLock { path, reason } => write!(
                f,
                "{}: the tree `{}` carries a lock redkiln did not write (`{reason}`); it is \
                 never released by redkiln, and this run is refused",
                self.name(),
                path.display()
            ),
            Self::DiscardIncomplete { path, remaining } => write!(
                f,
                "{}: after the discard the tree `{}` still reports {} path(s): {}",
                self.name(),
                path.display(),
                remaining.len(),
                remaining.join(", ")
            ),
            Self::FilterDriverNotEffective {
                driver,
                path,
                scopes,
            } => write!(
                f,
                "{}: `{path}` selects the filter driver `{driver}` at the fork commit, and no \
                 filter.{driver}.process or filter.{driver}.smudge value is effective at any \
                 scope read ({scopes}); the checkout would keep the committed content, which for \
                 Git LFS is a pointer file",
                self.name()
            ),
            Self::SmudgeSkipped {
                driver,
                path,
                scope,
                origin,
            } => write!(
                f,
                "{}: the smudge of the filter driver `{driver}`, which `{path}` selects at the \
                 fork commit, is skipped at {scope} scope ({origin}); the checkout would yield \
                 pointer files",
                self.name()
            ),
            Self::PointerFilesAfterCheckout(h) => {
                let PointerHits {
                    tree,
                    driver,
                    hits,
                    first,
                    scope,
                    origin,
                    removed,
                    kept_because,
                } = h.as_ref();
                write!(
                    f,
                    "{}: after checkout, {hits} path(s) the filter driver `{driver}` selects hold \
                     a Git LFS pointer (first: `{first}`), although its effective value at \
                     {scope} scope ({origin}) was read; the driver did not run. ",
                    self.name()
                )?;
                if *removed {
                    write!(
                        f,
                        "The fresh tree `{}` was removed (unforced); its branch is kept",
                        tree.display()
                    )
                } else {
                    write!(
                        f,
                        "The fresh tree `{}` is KEPT, never forced: {kept_because}",
                        tree.display()
                    )
                }
            }
        }
    }
}

/// An item id validated as a path component and a branch component. The private field is the
/// mechanism: [`ItemId::parse`] is the only way to make one, so [`derive`] cannot be handed an
/// unchecked id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(String);

impl ItemId {
    /// Validate `raw` (W1, as §8d-ii.2 lists the rules): at most 64 characters; one segment of
    /// `[a-zA-Z0-9._-]`; not `.`, `..` or `.git`, case-folded; no leading or trailing `.`, no `..`,
    /// no `.lock` suffix (git's ref rules for `rk-run/<item>`, case-folded because Windows' loose
    /// refs are files); not a Windows device name with or without an extension; and no
    /// `.review-ref` suffix, case-folded, which would collide with another item's reference tree.
    /// The first rule broken is the one named.
    ///
    /// # Errors
    /// [`GitError::WorktreeRefused`] carrying [`WorktreeRefusal::UnsafeItemId`].
    pub fn parse(raw: &str) -> Result<Self, GitError> {
        item_id_refusal(raw).map_or_else(
            || Ok(Self(raw.to_string())),
            |reason| {
                Err(refuse(WorktreeRefusal::UnsafeItemId {
                    item: raw.to_string(),
                    reason,
                }))
            },
        )
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The rule `raw` breaks first, or `None` when it is a safe id.
fn item_id_refusal(raw: &str) -> Option<ItemIdRefusal> {
    let folded = raw.to_ascii_lowercase();
    if raw.is_empty() {
        return Some(ItemIdRefusal::Empty);
    }
    if raw.chars().count() > ITEM_ID_MAX {
        return Some(ItemIdRefusal::TooLong);
    }
    if !raw
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
    {
        return Some(ItemIdRefusal::UnsafeCharacter);
    }
    if matches!(folded.as_str(), "." | ".." | ".git") {
        return Some(ItemIdRefusal::Reserved);
    }
    if raw.starts_with('.') {
        return Some(ItemIdRefusal::LeadingDot);
    }
    if raw.ends_with('.') {
        return Some(ItemIdRefusal::TrailingDot);
    }
    if raw.contains("..") {
        return Some(ItemIdRefusal::DoubleDot);
    }
    if folded.ends_with(".lock") {
        return Some(ItemIdRefusal::LockSuffix);
    }
    if is_device_name(folded.split('.').next().unwrap_or("")) {
        return Some(ItemIdRefusal::DeviceName);
    }
    if folded.ends_with(REVIEW_REF_SUFFIX) {
        return Some(ItemIdRefusal::ReviewRefSuffix);
    }
    None
}

/// `CON`, `PRN`, `AUX`, `NUL`, `COM1`..`COM9`, `LPT1`..`LPT9`, given the lower-cased stem before
/// the first `.` (so `nul.txt` is a device name too).
fn is_device_name(stem: &str) -> bool {
    if matches!(stem, "con" | "prn" | "aux" | "nul") {
        return true;
    }
    let b = stem.as_bytes();
    b.len() == 4
        && (stem.starts_with("com") || stem.starts_with("lpt"))
        && matches!(b[3], b'1'..=b'9')
}

/// The run tree an item derives to, and its branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunTree {
    pub item: ItemId,
    /// `<main>/.redkiln/worktrees/<item>`.
    pub path: PathBuf,
    /// `<main>/.redkiln/worktrees/<item>.review-ref`, the review reference tree.
    pub reference: PathBuf,
    /// `rk-run/<item>`.
    pub branch: String,
}

/// The run-tree root under the main worktree's directory (D21-44).
fn run_tree_root(main: &Path) -> PathBuf {
    main.join(".redkiln").join("worktrees")
}

/// **The one derivation** (§8d-ii.1): the main worktree's directory and a validated item to the
/// run tree's path, the review reference tree's path and the run branch. `main` is what
/// [`main_worktree`] returns; nothing else derives a path under the root.
#[must_use]
pub fn derive(main: &Path, item: &ItemId) -> RunTree {
    let root = run_tree_root(main);
    RunTree {
        item: item.clone(),
        path: root.join(item.as_str()),
        reference: root.join(format!("{}{REVIEW_REF_SUFFIX}", item.as_str())),
        branch: format!("{RUN_BRANCH_PREFIX}{}", item.as_str()),
    }
}

const fn refuse(refusal: WorktreeRefusal) -> GitError {
    GitError::WorktreeRefused { refusal }
}

/// One `worktree list --porcelain` record.
struct WorktreeRecord {
    path: String,
    bare: bool,
    /// The `locked` line: `None` unlocked, `Some("")` locked with no reason, else the reason.
    locked: Option<String>,
}

/// Read every `worktree list --porcelain` record, in git's order (the main worktree first).
fn worktree_records(exec: &GitExec, at: &WorktreePath) -> Result<Vec<WorktreeRecord>, GitError> {
    let out = exec.run(GitOp::WorktreeList, at, ["worktree", "list", "--porcelain"])?;
    let text = String::from_utf8(out.stdout).map_err(|_| {
        refuse(WorktreeRefusal::UnreadableWorktreeList {
            detail: "the listing is not UTF-8".into(),
        })
    })?;
    let mut records: Vec<WorktreeRecord> = Vec::new();
    for raw in text.split('\n') {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if let Some(path) = line.strip_prefix("worktree ") {
            // git quotes a path it cannot print on one line; such a path is not read as another.
            if path.starts_with('"') || path.is_empty() {
                return Err(refuse(WorktreeRefusal::UnreadableWorktreeList {
                    detail: format!("a quoted or empty worktree path: {path}"),
                }));
            }
            records.push(WorktreeRecord {
                path: path.to_string(),
                bare: false,
                locked: None,
            });
        } else if let (Some(reason), Some(r)) = (
            (line == "locked")
                .then(String::new)
                .or_else(|| line.strip_prefix("locked ").map(str::to_string)),
            records.last_mut(),
        ) {
            r.locked = Some(reason);
        } else if line == "bare" {
            match records.last_mut() {
                Some(r) => r.bare = true,
                None => {
                    return Err(refuse(WorktreeRefusal::UnreadableWorktreeList {
                        detail: "`bare` before any worktree record".into(),
                    }));
                }
            }
        }
    }
    if records.is_empty() {
        return Err(refuse(WorktreeRefusal::UnreadableWorktreeList {
            detail: "no worktree record".into(),
        }));
    }
    Ok(records)
}

/// **The main worktree's directory**, whichever checkout `start` is (§8d-ii.1): the first
/// `worktree` record of `git worktree list --porcelain`, which git lists first. Not
/// `--show-toplevel`, which names the checkout a command runs in, so a run started in a linked
/// worktree or inside a run tree would root its tree there.
///
/// # Errors
/// [`WorktreeRefusal::BareRepository`] when the first record carries `bare`;
/// [`WorktreeRefusal::UnreadableWorktreeList`]; wrapper errors.
pub fn main_worktree(exec: &GitExec, start: &Path) -> Result<PathBuf, GitError> {
    let at = WorktreePath::new(start)?;
    let records = worktree_records(exec, &at)?;
    let first = &records[0];
    if first.bare {
        return Err(refuse(WorktreeRefusal::BareRepository {
            repo: PathBuf::from(&first.path),
        }));
    }
    Ok(PathBuf::from(&first.path))
}

/// Every registered worktree's path, as `git worktree list --porcelain` prints it.
///
/// # Errors
/// As [`main_worktree`], less the bare refusal.
pub fn registered_worktrees(exec: &GitExec, at: &Path) -> Result<Vec<PathBuf>, GitError> {
    let at = WorktreePath::new(at)?;
    Ok(worktree_records(exec, &at)?
        .into_iter()
        .map(|r| PathBuf::from(r.path))
        .collect())
}

/// Is this metadata a Windows reparse point (`FILE_ATTRIBUTE_REPARSE_POINT`)? A junction reads
/// as one whether or not `is_symlink` reports it.
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

/// W1's symlink guards: refuse when `.redkiln`, `.redkiln/worktrees` or `<item>` under `main` is
/// a symlink or, on Windows, a junction. Nothing is followed: each segment is read with
/// `symlink_metadata`. An absent segment is not a link (creation makes it).
///
/// # Errors
/// [`WorktreeRefusal::LinkedSegment`] naming the segment; [`WorktreeRefusal::SegmentUnreadable`].
pub fn refuse_linked_segments(main: &Path, item: &ItemId) -> Result<(), GitError> {
    let dot = main.join(".redkiln");
    let root = run_tree_root(main);
    let tree = root.join(item.as_str());
    for seg in [dot, root, tree] {
        let md = match std::fs::symlink_metadata(&seg) {
            Ok(md) => md,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => {
                return Err(refuse(WorktreeRefusal::SegmentUnreadable {
                    path: seg,
                    detail: e.to_string(),
                }));
            }
        };
        let kind = if md.file_type().is_symlink() {
            if cfg!(windows) {
                "symlink or junction"
            } else {
                "symlink"
            }
        } else if is_reparse_point(&md) {
            "junction (a reparse point)"
        } else {
            continue;
        };
        return Err(refuse(WorktreeRefusal::LinkedSegment { path: seg, kind }));
    }
    Ok(())
}

/// Normalise a path for a registry comparison only: forward slashes, ASCII case folded (Windows
/// folds case), no trailing slash. Never handed to git.
fn fold_path(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
    s.trim_end_matches('/').to_string()
}

/// W1's case-fold rule, which reads the registry: refuse an id that case-folds onto a registered
/// run tree (or its reference tree) or an existing `rk-run/` branch **without equalling it**.
/// Windows' filesystem folds case and git's refs do not, so `Fix-1` and `fix-1` would share a
/// directory and not a branch.
///
/// # Errors
/// [`WorktreeRefusal::CaseFoldCollision`] naming the existing id; wrapper errors.
pub fn refuse_case_fold_collision(
    exec: &GitExec,
    main: &Path,
    item: &ItemId,
) -> Result<(), GitError> {
    let at = WorktreePath::new(main)?;
    let root = fold_path(&run_tree_root(main));
    let mut existing: Vec<String> = Vec::new();
    for rec in worktree_records(exec, &at)? {
        let p = Path::new(&rec.path);
        let under_root = p.parent().is_some_and(|parent| fold_path(parent) == root);
        if let (true, Some(name)) = (under_root, p.file_name()) {
            existing.push(name.to_string_lossy().into_owned());
        }
    }
    let branches = exec.run(
        GitOp::RevParse,
        &at,
        [
            "rev-parse",
            "--symbolic-full-name",
            &format!("--branches={RUN_BRANCH_PREFIX}*"),
        ],
    )?;
    let full_prefix = format!("refs/heads/{RUN_BRANCH_PREFIX}");
    for line in branches.stdout_trimmed().lines() {
        if let Some(name) = line.trim_end_matches('\r').strip_prefix(&full_prefix) {
            existing.push(name.to_string());
        }
    }
    for name in existing {
        let base = if name.to_ascii_lowercase().ends_with(REVIEW_REF_SUFFIX) {
            name[..name.len() - REVIEW_REF_SUFFIX.len()].to_string()
        } else {
            name
        };
        if base != item.as_str() && base.eq_ignore_ascii_case(item.as_str()) {
            return Err(refuse(WorktreeRefusal::CaseFoldCollision {
                item: item.as_str().to_string(),
                existing: base,
            }));
        }
    }
    Ok(())
}

/// D21-48 X1, *"**Refuse by name.**"*: creation refuses when `git check-ignore`, run in the main
/// worktree, does not report `.redkiln/worktrees/<item>` as ignored. Without the row, `git status
/// --porcelain` in the user's checkout lists the run tree and AC-518's instrument fails on a
/// correct run. Exit 1 means *not ignored*; stderr is never read.
///
/// # Errors
/// [`WorktreeRefusal::RootNotIgnored`] naming the path; wrapper errors.
pub fn refuse_unignored_root(exec: &GitExec, main: &Path, item: &ItemId) -> Result<(), GitError> {
    let at = WorktreePath::new(main)?;
    let rel = format!(".redkiln/worktrees/{}", item.as_str());
    match exec.run(
        GitOp::CheckIgnore,
        &at,
        ["check-ignore", "-q", "--", rel.as_str()],
    ) {
        Ok(_) => Ok(()),
        Err(GitError::NonZeroExit { code: Some(1), .. }) => {
            Err(refuse(WorktreeRefusal::RootNotIgnored {
                main: main.to_path_buf(),
                path: rel,
            }))
        }
        Err(other) => Err(other),
    }
}

/// The worst-case path bound (§8d-ii.6): `len(root) + 20 + len(item) + 11 + 1 + longest`, where
/// `root` is the main worktree's directory, 20 is `/.redkiln/worktrees/`, 11 is `.review-ref` (the
/// longer of the two trees' names) and `longest` the longest path tracked at the fork commit.
/// **`item_len` is the actual item's**, never the 64-character maximum (D21-47).
#[must_use]
pub const fn path_bound(root_len: usize, item_len: usize, longest: usize) -> usize {
    root_len + ROOT_SEGMENTS_LEN + item_len + REVIEW_REF_SUFFIX.len() + 1 + longest
}

/// Lengths in UTF-16 code units, the unit Windows' `MAX_PATH` counts.
fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// Is `core.longpaths` `true` at any scope, as git resolves it? Exit 1 means *not set*.
fn longpaths_enabled(exec: &GitExec, at: &WorktreePath) -> Result<bool, GitError> {
    match exec.run(
        GitOp::Config,
        at,
        ["config", "--type=bool", "--get", "core.longpaths"],
    ) {
        Ok(out) => Ok(out.stdout_trimmed() == "true"),
        Err(GitError::NonZeroExit { code: Some(1), .. }) => Ok(false),
        Err(other) => Err(other),
    }
}

/// §8d-ii.6: refuse by name when [`path_bound`], computed on **this** item and the longest path
/// tracked at `fork`, exceeds [`PATH_LIMIT`] and `core.longpaths` is not `true`, rather than
/// check out a partial tree.
///
/// # Errors
/// [`WorktreeRefusal::PathPastLimit`]; wrapper errors.
pub fn refuse_past_limit(
    exec: &GitExec,
    main: &Path,
    item: &ItemId,
    fork: &str,
) -> Result<(), GitError> {
    let at = WorktreePath::new(main)?;
    let out = exec.run(
        GitOp::LsTree,
        &at,
        ["ls-tree", "-r", "-z", "--name-only", fork],
    )?;
    let mut longest = 0usize;
    let mut longest_path = String::new();
    for raw in out.stdout.split(|b| *b == 0).filter(|s| !s.is_empty()) {
        let p = String::from_utf8_lossy(raw);
        let n = utf16_len(&p);
        if n > longest {
            longest = n;
            longest_path = p.into_owned();
        }
    }
    let bound = path_bound(
        utf16_len(&main.to_string_lossy()),
        utf16_len(item.as_str()),
        longest,
    );
    if bound <= PATH_LIMIT || longpaths_enabled(exec, &at)? {
        return Ok(());
    }
    Err(refuse(WorktreeRefusal::PathPastLimit {
        item: item.as_str().to_string(),
        bound,
        limit: PATH_LIMIT,
        longest,
        longest_path,
    }))
}

/// A canonical path in the drive-letter form, for a message only. A `\\?\UNC\` form, which
/// [`WorktreePath`] refuses, is shown as it is.
fn shown(p: &Path) -> PathBuf {
    WorktreePath::new(p).map_or_else(|_| p.to_path_buf(), |w| PathBuf::from(w.as_str()))
}

/// W1's post-create realpath check: after `git worktree add`, the created tree's canonical path
/// must lie under the canonical run-tree root, or it is refused naming both paths. The
/// canonical (`\\?\`) forms are **compared only** and never handed to git; the refusal carries
/// them in drive-letter form.
///
/// # Errors
/// [`WorktreeRefusal::RealpathOutsideRoot`]; [`WorktreeRefusal::SegmentUnreadable`] when a path
/// cannot be canonicalised.
pub fn verify_created(main: &Path, created: &Path) -> Result<(), GitError> {
    let canon = |p: &Path| {
        std::fs::canonicalize(p).map_err(|e| {
            refuse(WorktreeRefusal::SegmentUnreadable {
                path: p.to_path_buf(),
                detail: e.to_string(),
            })
        })
    };
    let root = canon(&run_tree_root(main))?;
    let tree = canon(created)?;
    if tree != root && tree.starts_with(&root) {
        return Ok(());
    }
    Err(refuse(WorktreeRefusal::RealpathOutsideRoot {
        created: created.to_path_buf(),
        resolved: shown(&tree),
        root: shown(&root),
    }))
}

/// **Creation's guards, before `git worktree add`**, in order: the main worktree (a bare
/// repository refused), the item id, the symlink and junction guards, the case-fold rule, the
/// ignore refusal (D21-48 X1) and the length rule on the actual item at `fork`. Returns the one
/// derivation. `start` may be any checkout of the repository.
///
/// # Errors
/// [`GitError::WorktreeRefused`] naming the first guard that refused; wrapper errors.
pub fn prepare_creation(
    exec: &GitExec,
    start: &Path,
    item: &str,
    fork: &str,
) -> Result<RunTree, GitError> {
    let main = main_worktree(exec, start)?;
    let item = ItemId::parse(item)?;
    refuse_linked_segments(&main, &item)?;
    refuse_case_fold_collision(exec, &main, &item)?;
    refuse_unignored_root(exec, &main, &item)?;
    refuse_past_limit(exec, &main, &item, fork)?;
    Ok(derive(&main, &item))
}

// ---- The lifecycle: create, re-enter, refuse and remove ----
//
// s8d-ii §8d-ii.2 and §8d-ii.3 (D21-45 W1 and W2; D21-47; ADR-0180 decisions 5, 6, 7 and 9,
// proposed). *(dated 2026-09-27, s8d-ii build, P3.)*

/// The ownership marker's file name, in the tree's OWN gitdir (W1; ADR-0109 decision 4).
pub const RK_BASE: &str = "RK_BASE";

/// What `RK_BASE` records: the fork sha read once before the add, and the item id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RkBase {
    pub fork: String,
    pub item: String,
}

/// The run that takes a tree's lock: its pid and its start time as the OS reports it (T in
/// `redkiln run <item> (pid N start T)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockOwner {
    pub pid: u32,
    pub start: u64,
}

/// **The liveness predicate, injected** (§8d-ii.2; ADR-0180 decision 6).
///
/// `rk-cli` builds it from
/// `rk_containment::is_alive` and `rk_containment::process_start_time`, so `rk-git` gains no
/// edge to `rk-containment`. `start_time(pid)` is the start time of the live process `pid`
/// names, or `None` when no live process has that pid. A lock is **live** when this returns
/// exactly the start time the lock records.
pub trait ProcessProbe {
    fn start_time(&self, pid: u32) -> Option<u64>;
}

impl<F: Fn(u32) -> Option<u64>> ProcessProbe for F {
    fn start_time(&self, pid: u32) -> Option<u64> {
        self(pid)
    }
}

/// How [`enter`] came by the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Creation {
    /// First creation: `worktree add --no-track -b rk-run/<item> <path> <fork>`.
    New,
    /// The branch existed and no tree was registered: `worktree add <path> rk-run/<item>`,
    /// never `-B`.
    OnExistingBranch,
    /// The registered tree, re-entered and reused after an explicit discard.
    Reused,
}

/// [`enter`]'s typed result.
///
/// The tree, the fork `RK_BASE` records, how the tree was come by, and
/// **every discarded path** (N10's semantics; `s8e`'s `worktree_assigned.discarded` from `s8e`
/// on, and one stderr line per path until then).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entered {
    pub tree: RunTree,
    pub fork: String,
    pub created: Creation,
    pub discarded: Vec<String>,
    /// The filter read's and the scan's facts, for the **debug** level only (§8d-ii.5: an
    /// admitted driver prints nothing at the default level). Empty on re-entry, which reads and
    /// scans nothing.
    pub filter_notes: Vec<String>,
}

/// Why a removal kept the tree. The `Dirty` reason is Claude Code's form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeepReason {
    /// No `RK_BASE`: redkiln never removes a tree it did not create.
    Unmarked,
    /// The tree is locked (the lock's reason, as `worktree list` prints it).
    Locked { reason: String },
    /// `HEAD` is not symbolic on `rk-run/<item>` at its tip (detached, another branch, or
    /// commits the run ref lacks).
    NotOnRunRef { head: String },
    /// The status is not empty.
    Dirty { changed: usize },
    /// The tip's `Story:` trailer is not the lane's terminal `closed` mark (§8d-ii.3's condition
    /// 3; the trailer values the tip carries, if any).
    Unlanded { story: Vec<String> },
    /// Nothing at the derived path is a registered worktree of this repository.
    NotRegistered,
}

impl std::fmt::Display for KeepReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unmarked => f.write_str(
                "it carries no RK_BASE marker; redkiln never removes a tree it did not create",
            ),
            Self::Locked { reason } => write!(f, "it is locked (`{reason}`)"),
            Self::NotOnRunRef { head } => {
                write!(f, "its HEAD (`{head}`) is not its run ref's tip")
            }
            Self::Dirty { changed } => write!(f, "{changed} changed file(s) would be lost"),
            Self::NotRegistered => {
                f.write_str("it is not a registered worktree of this repository")
            }
            Self::Unlanded { story } => write!(
                f,
                "it has not landed: its tip carries no `Story: <project>/<item> closed` mark \
                 (Story: {})",
                if story.is_empty() {
                    "none".to_string()
                } else {
                    story.join(", ")
                }
            ),
        }
    }
}

/// [`remove_landed`]'s decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Removal {
    Removed,
    Kept { path: PathBuf, reason: KeepReason },
}

/// The tree's own gitdir, absolute: what `git rev-parse --git-dir` prints from inside it (the
/// `.git/worktrees/<name>` directory of a linked tree).
fn tree_gitdir(exec: &GitExec, tree: &WorktreePath) -> Result<PathBuf, GitError> {
    let out = exec.run(
        GitOp::RevParse,
        tree,
        ["rev-parse", "--path-format=absolute", "--git-dir"],
    )?;
    Ok(PathBuf::from(out.stdout_trimmed()))
}

/// The repository's common dir as seen from `at`, absolute.
fn common_dir_of(exec: &GitExec, at: &WorktreePath) -> Result<String, GitError> {
    let out = exec.run(
        GitOp::RevParse,
        at,
        ["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    Ok(out.stdout_trimmed())
}

/// Read `RK_BASE` from the tree's gitdir. `None` when it is absent **or does not parse**: a
/// marker redkiln cannot read is not proof redkiln made the tree (fail closed).
///
/// # Errors
/// [`WorktreeRefusal::SegmentUnreadable`] when the file exists and cannot be read; wrapper
/// errors from the gitdir read.
pub fn read_rk_base(exec: &GitExec, tree: &Path) -> Result<Option<RkBase>, GitError> {
    let at = WorktreePath::new(tree)?;
    let marker = tree_gitdir(exec, &at)?.join(RK_BASE);
    let text = match std::fs::read_to_string(&marker) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => {
            return Err(refuse(WorktreeRefusal::SegmentUnreadable {
                path: marker,
                detail: e.to_string(),
            }));
        }
    };
    let mut fork = None;
    let mut item = None;
    for raw in text.lines() {
        let line = raw.trim_end_matches('\r');
        if let Some(v) = line.strip_prefix("fork ") {
            fork = Some(v.to_string());
        } else if let Some(v) = line.strip_prefix("item ") {
            item = Some(v.to_string());
        }
    }
    Ok(fork
        .zip(item)
        .filter(|(f, i)| !f.is_empty() && !i.is_empty())
        .map(|(fork, item)| RkBase { fork, item }))
}

/// Write `RK_BASE` into the tree's own gitdir (W1).
///
/// `fork <sha>` and `item <id>`, one per line, through a temporary file and a rename. A file `rk-git` writes, not a git invocation, so it
/// has no `GitOp` variant; it is still a write into git's admin area, and ADR-0180 names it
/// among the writes that supersede ADR-0105 clause 5 in part (D21-47).
///
/// # Errors
/// [`GitError::Spawn`] when the OS refuses the write (the precedent is `GitExec::new`'s hooks
/// directory); wrapper errors from the gitdir read.
pub fn write_rk_base(exec: &GitExec, tree: &Path, base: &RkBase) -> Result<(), GitError> {
    let at = WorktreePath::new(tree)?;
    let gitdir = tree_gitdir(exec, &at)?;
    let marker = gitdir.join(RK_BASE);
    let tmp = gitdir.join(format!("{RK_BASE}.tmp"));
    let body = format!("fork {}\nitem {}\n", base.fork, base.item);
    let fail = |e: std::io::Error| GitError::Spawn {
        detail: format!("could not write {}: {e}", marker.display()),
    };
    std::fs::write(&tmp, body.as_bytes()).map_err(fail)?;
    std::fs::rename(&tmp, &marker).map_err(fail)
}

/// The lock's reason: `redkiln run <item> (pid N start T)` (W1).
#[must_use]
pub fn lock_reason(item: &ItemId, owner: &LockOwner) -> String {
    format!(
        "redkiln run {} (pid {} start {})",
        item.as_str(),
        owner.pid,
        owner.start
    )
}

/// Parse a lock reason redkiln wrote back into `(item, pid, start)`; anything else is `None`.
fn parse_lock_reason(reason: &str) -> Option<(String, u32, u64)> {
    let rest = reason.strip_prefix("redkiln run ")?;
    let (item, tail) = rest.split_once(" (pid ")?;
    let (pid, start) = tail.strip_suffix(')')?.split_once(" start ")?;
    Some((item.to_string(), pid.parse().ok()?, start.parse().ok()?))
}

/// The record registered at `path`, compared on the folded form (never handed to git).
fn record_at<'r>(records: &'r [WorktreeRecord], path: &Path) -> Option<&'r WorktreeRecord> {
    let want = fold_path(path);
    records
        .iter()
        .find(|r| fold_path(Path::new(&r.path)) == want)
}

/// Take the run's lock (W1). An unlocked tree is locked. A lock redkiln wrote for this item is
/// **live** when `probe` reports its pid alive with exactly the recorded start time: the run is
/// refused by name. Otherwise it is stale (a dead pid, or a pid that now names another process)
/// and is released, then taken. A lock redkiln did not write is never released: refused by name.
fn take_lock(
    exec: &GitExec,
    main: &WorktreePath,
    tree: &RunTree,
    current: Option<&str>,
    owner: &LockOwner,
    probe: &dyn ProcessProbe,
) -> Result<(), GitError> {
    let path = WorktreePath::new(&tree.path)?;
    if let Some(reason) = current {
        match parse_lock_reason(reason) {
            Some((item, pid, start)) if item == tree.item.as_str() => {
                if probe.start_time(pid) == Some(start) {
                    return Err(refuse(WorktreeRefusal::LockHeld {
                        path: tree.path.clone(),
                        pid,
                        start,
                    }));
                }
                exec.run(
                    GitOp::WorktreeUnlock,
                    main,
                    ["worktree", "unlock", path.as_str()],
                )?;
            }
            _ => {
                return Err(refuse(WorktreeRefusal::ForeignLock {
                    path: tree.path.clone(),
                    reason: reason.to_string(),
                }));
            }
        }
    }
    let reason = lock_reason(&tree.item, owner);
    exec.run(
        GitOp::WorktreeLock,
        main,
        [
            "worktree",
            "lock",
            "--reason",
            reason.as_str(),
            path.as_str(),
        ],
    )?;
    Ok(())
}

/// Release the run's own lock at its end (W1). Returns whether a lock was released: a tree that
/// is not registered, not locked, or locked by anyone else is left exactly as it is.
///
/// # Errors
/// [`GitError::WorktreeRefused`] for an unsafe item id; wrapper errors.
pub fn release(
    exec: &GitExec,
    start: &Path,
    item: &str,
    owner: &LockOwner,
) -> Result<bool, GitError> {
    let main = main_worktree(exec, start)?;
    let id = ItemId::parse(item)?;
    let tree = derive(&main, &id);
    let main_at = WorktreePath::new(&main)?;
    let records = worktree_records(exec, &main_at)?;
    let ours = lock_reason(&id, owner);
    match record_at(&records, &tree.path).and_then(|r| r.locked.as_deref()) {
        Some(reason) if reason == ours => {
            let path = WorktreePath::new(&tree.path)?;
            exec.run(
                GitOp::WorktreeUnlock,
                &main_at,
                ["worktree", "unlock", path.as_str()],
            )?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// One `status --porcelain=v1 -z --untracked-files=all` read: the number of entries, and every
/// path they name (a rename or copy names two).
fn in_flight(exec: &GitExec, at: &WorktreePath) -> Result<(usize, Vec<String>), GitError> {
    let out = exec.run(
        GitOp::Status,
        at,
        ["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    let mut fields = out.stdout.split(|b| *b == 0).filter(|f| !f.is_empty());
    let mut entries = 0usize;
    let mut paths = Vec::new();
    while let Some(f) = fields.next() {
        if f.len() < 4 {
            continue;
        }
        entries += 1;
        let (xy, path) = f.split_at(3);
        paths.push(String::from_utf8_lossy(path).into_owned());
        if (matches!(xy[0], b'R' | b'C') || matches!(xy[1], b'R' | b'C'))
            && let Some(orig) = fields.next()
        {
            paths.push(String::from_utf8_lossy(orig).into_owned());
        }
    }
    Ok((entries, paths))
}

/// The discard (W2; N10's semantics): read every in-flight path, then `reset --hard HEAD` and
/// `clean -fd` **without `-x`**, so ignored files survive; return the paths. The status must be
/// empty afterwards, or the re-entry is refused by name.
fn discard(exec: &GitExec, at: &WorktreePath, path: &Path) -> Result<Vec<String>, GitError> {
    let (_, paths) = in_flight(exec, at)?;
    if !paths.is_empty() {
        exec.run(GitOp::ResetHard, at, ["reset", "--hard", "--quiet", "HEAD"])?;
        exec.run(GitOp::Clean, at, ["clean", "-fd", "--quiet"])?;
    }
    let (_, remaining) = in_flight(exec, at)?;
    if !remaining.is_empty() {
        return Err(refuse(WorktreeRefusal::DiscardIncomplete {
            path: path.to_path_buf(),
            remaining,
        }));
    }
    Ok(paths)
}

/// The commit `refs/heads/<branch>` names, or `None` when the branch does not exist. Exit 1 of
/// `rev-parse --verify --quiet` means *no*. *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii:
/// it was `branch_exists`, a bool; D21-89 needs the tip itself.)*
fn branch_tip(
    exec: &GitExec,
    main: &WorktreePath,
    branch: &str,
) -> Result<Option<String>, GitError> {
    let full = format!("refs/heads/{branch}^{{commit}}");
    match exec.run(
        GitOp::RevParse,
        main,
        ["rev-parse", "--verify", "--quiet", full.as_str()],
    ) {
        Ok(out) => Ok(Some(out.stdout_trimmed())),
        Err(GitError::NonZeroExit { code: Some(1), .. }) => Ok(None),
        Err(other) => Err(other),
    }
}

/// `git merge-base <head> <tip>`: where an existing run branch forked from start's `HEAD` (D21-89
/// ruling 2). A read (`GitOp::MergeBase`). Histories with no common ancestor exit 1, which is
/// returned as the wrapper's error before any add: nothing is created.
fn merge_base(
    exec: &GitExec,
    main: &WorktreePath,
    head: &str,
    tip: &str,
) -> Result<String, GitError> {
    Ok(exec
        .run(GitOp::MergeBase, main, ["merge-base", head, tip])?
        .stdout_trimmed())
}

/// **Enter the run tree for `item`** (s8d-ii §8d-ii.3; W1, W2): create it, or re-enter and reuse
/// it, or refuse and keep it. Never re-creates a registered tree, never removes one, never
/// `-B`.
///
/// - **Registered at the derived path:** re-entry. It must carry `RK_BASE` for this item, be a
///   checkout of this repository, have no commits the run ref lacks, and have a **symbolic**
///   `HEAD` on `refs/heads/rk-run/<item>` at its tip. Then the lock is taken (a live one refuses)
///   and the in-flight changes are discarded and returned.
/// - **Something unregistered at the derived path:** refused and kept (another repository's
///   checkout, or a plain directory).
/// - **Nothing there:** creation's guards ([`prepare_creation`]), then `git worktree add
///   --no-track -b rk-run/<item> <path> <fork>` with the fork read once from `start`'s `HEAD`,
///   or `git worktree add <path> rk-run/<item>` when the branch exists; the realpath check;
///   `RK_BASE`; D21-48 X2's post-checkout pointer scan; and the lock. Before the add,
///   [`check_filters`] reads the drivers and refuses when pointer files would result (D21-47 V1,
///   §8d-ii.5), with `lfs_skip_smudge` the `GIT_LFS_SKIP_SMUDGE` value `rk-cli` read. Re-entry
///   reads and scans nothing: a reused tree was scanned when it was created.
/// - **D21-89, when the branch exists** *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii)*:
///   the length rule, the filter read and so the scan's path set read the **commit the tree
///   checks out**, the branch's tip (read before the add), never start's `HEAD`; and `RK_BASE`
///   records `merge-base(start HEAD, tip)`, the point the branch forked. A first creation is
///   unchanged: it checks out, reads and records start's `HEAD`.
///
/// # Errors
/// [`GitError::WorktreeRefused`] naming the path and the reason; wrapper errors.
pub fn enter(
    exec: &GitExec,
    start: &Path,
    item: &str,
    owner: &LockOwner,
    probe: &dyn ProcessProbe,
    lfs_skip_smudge: Option<&str>,
) -> Result<Entered, GitError> {
    let main = main_worktree(exec, start)?;
    let id = ItemId::parse(item)?;
    refuse_linked_segments(&main, &id)?;
    let tree = derive(&main, &id);
    let main_at = WorktreePath::new(&main)?;
    let records = worktree_records(exec, &main_at)?;
    if let Some(rec) = record_at(&records, &tree.path) {
        let current = rec.locked.clone();
        return reenter(exec, &main_at, tree, current.as_deref(), owner, probe);
    }
    if std::fs::symlink_metadata(&tree.path).is_ok() {
        return Err(refuse(unregistered(exec, &main_at, &tree.path)?));
    }
    create(exec, start, &main_at, &id, owner, probe, lfs_skip_smudge)
}

/// What is at an unregistered derived path: another repository's checkout, or a directory of
/// this one. Both are refused and kept.
fn unregistered(
    exec: &GitExec,
    main: &WorktreePath,
    path: &Path,
) -> Result<WorktreeRefusal, GitError> {
    let expected = common_dir_of(exec, main)?;
    let at = WorktreePath::new(path)?;
    let found = match common_dir_of(exec, &at) {
        Ok(found) if fold_path(Path::new(&found)) == fold_path(Path::new(&expected)) => {
            format!("{found} (a directory of this checkout, not a registered worktree)")
        }
        Ok(found) => found,
        Err(GitError::NonZeroExit { .. }) => "none (not a git checkout)".to_string(),
        Err(other) => return Err(other),
    };
    Ok(WorktreeRefusal::ForeignRepository {
        path: path.to_path_buf(),
        found,
        expected,
    })
}

fn reenter(
    exec: &GitExec,
    main: &WorktreePath,
    tree: RunTree,
    current_lock: Option<&str>,
    owner: &LockOwner,
    probe: &dyn ProcessProbe,
) -> Result<Entered, GitError> {
    let at = WorktreePath::new(&tree.path)?;
    let Some(base) = read_rk_base(exec, &tree.path)? else {
        return Err(refuse(WorktreeRefusal::NoRkBase { path: tree.path }));
    };
    if base.item != tree.item.as_str() {
        return Err(refuse(WorktreeRefusal::RkBaseMismatch {
            path: tree.path,
            recorded: base.item,
        }));
    }
    let expected = common_dir_of(exec, main)?;
    let found = common_dir_of(exec, &at)?;
    if fold_path(Path::new(&found)) != fold_path(Path::new(&expected)) {
        return Err(refuse(WorktreeRefusal::ForeignRepository {
            path: tree.path,
            found,
            expected,
        }));
    }
    let full = format!("refs/heads/{}", tree.branch);
    let ahead = exec
        .run(
            GitOp::Log,
            &at,
            ["log", "--format=%H", &format!("{full}..HEAD")],
        )?
        .stdout_trimmed()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .count();
    if ahead > 0 {
        return Err(refuse(WorktreeRefusal::CommitsRunRefLacks {
            path: tree.path,
            branch: tree.branch,
            count: ahead,
        }));
    }
    let head_ref = exec
        .run(
            GitOp::RevParse,
            &at,
            ["rev-parse", "--symbolic-full-name", "HEAD"],
        )?
        .stdout_trimmed();
    if head_ref == "HEAD" {
        return Err(refuse(WorktreeRefusal::DetachedHead { path: tree.path }));
    }
    let head = exec
        .run(GitOp::RevParse, &at, ["rev-parse", "--verify", "HEAD"])?
        .stdout_trimmed();
    let tip = exec
        .run(
            GitOp::RevParse,
            main,
            ["rev-parse", "--verify", full.as_str()],
        )?
        .stdout_trimmed();
    if head_ref != full || head != tip {
        return Err(refuse(WorktreeRefusal::OffRunRef {
            path: tree.path,
            head: head_ref,
            branch: tree.branch,
        }));
    }
    take_lock(exec, main, &tree, current_lock, owner, probe)?;
    // A refused discard releases the lock this call just took, so no refusal leaves the run's own
    // lock behind (ADR-0180 decision 6, *"released at the run's end"*; round-1 minor). The tree
    // is kept. *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)*
    let discarded = match discard(exec, &at, &tree.path) {
        Ok(paths) => paths,
        Err(refused) => {
            exec.run(
                GitOp::WorktreeUnlock,
                main,
                ["worktree", "unlock", at.as_str()],
            )?;
            return Err(refused);
        }
    };
    Ok(Entered {
        tree,
        fork: base.fork,
        created: Creation::Reused,
        discarded,
        filter_notes: Vec::new(),
    })
}

fn create(
    exec: &GitExec,
    start: &Path,
    main: &WorktreePath,
    id: &ItemId,
    owner: &LockOwner,
    probe: &dyn ProcessProbe,
    lfs_skip_smudge: Option<&str>,
) -> Result<Entered, GitError> {
    let start_at = WorktreePath::new(start)?;
    // start's frozen `HEAD`, read ONCE, before the add (D21-45). A first creation forks there.
    let head = exec
        .run(
            GitOp::RevParse,
            &start_at,
            ["rev-parse", "--verify", "HEAD^{commit}"],
        )?
        .stdout_trimmed();
    // D21-89 ruling 1: when `rk-run/<item>` exists, the tree checks out its TIP, so the length
    // rule, the filter read and the scan's path set read that commit, read here before the add.
    // *(dated 2026-09-28, s8d-ii build, package R1FIX-8dii.)*
    let tip = branch_tip(exec, main, &derive(Path::new(main.as_str()), id).branch)?;
    let checked = tip.as_deref().unwrap_or(head.as_str());
    let tree = prepare_creation(exec, start, id.as_str(), checked)?;
    let filters = check_filters(exec, Path::new(main.as_str()), id, checked, lfs_skip_smudge)?;
    // D21-89 ruling 2: RK_BASE records where the branch forked, `merge-base(start HEAD, tip)`; a
    // first creation's fork is start's `HEAD`, unchanged.
    let fork = match tip.as_deref() {
        Some(t) => merge_base(exec, main, &head, t)?,
        None => head.clone(),
    };
    let path = WorktreePath::new(&tree.path)?;
    let created = if tip.is_some() {
        // The branch exists and no tree is registered: check it out as it stands. Never `-B`.
        exec.run(
            GitOp::WorktreeAdd,
            main,
            [
                "worktree",
                "add",
                "--quiet",
                path.as_str(),
                tree.branch.as_str(),
            ],
        )?;
        Creation::OnExistingBranch
    } else {
        exec.run(
            GitOp::WorktreeAdd,
            main,
            [
                "worktree",
                "add",
                "--quiet",
                "--no-track",
                "-b",
                tree.branch.as_str(),
                path.as_str(),
                fork.as_str(),
            ],
        )?;
        Creation::New
    };
    verify_created(Path::new(main.as_str()), &tree.path)?;
    write_rk_base(
        exec,
        &tree.path,
        &RkBase {
            fork: fork.clone(),
            item: id.as_str().to_string(),
        },
    )?;
    // D21-48 X2: the scan, on the tree THIS call created, after RK_BASE and before the lock.
    let hits = match scan_for_pointers(&tree.path, &filters) {
        Ok(hits) => hits,
        Err(e) => {
            // An unreadable selected file is not admitted: the fresh tree goes the rollback's way.
            remove_fresh(exec, main, &tree)?;
            return Err(e);
        }
    };
    if !hits.is_empty() {
        return Err(pointer_refusal(exec, main, &tree, &filters, &hits)?);
    }
    take_lock(exec, main, &tree, None, owner, probe)?;
    Ok(Entered {
        tree,
        fork,
        created,
        discarded: Vec::new(),
        filter_notes: filters.notes,
    })
}

/// Is `value` the lane's terminal mark for `item`, `<project>/<item> closed` (§8d-ii.3's
/// condition 3)? A stage-less legacy value is not.
fn is_closed_mark(value: &str, item: &str) -> bool {
    let tokens: Vec<&str> = value.split_whitespace().collect();
    tokens.len() == 2
        && tokens[1] == "closed"
        && tokens[0]
            .rsplit_once('/')
            .is_some_and(|(project, slug)| !project.is_empty() && slug == item)
}

/// Remove every link (a symlink, or on Windows a junction) inside the tree that git does not
/// track, **without following it**: the link is removed, never its target. Git for Windows' own
/// unforced `worktree remove` exits 0 and leaves every junction and its parent directories on
/// disk (s8c §8c.12 item 3's skeleton, measured 2026-09-27 on git 2.55.0.windows.3), so no link
/// reaches it. Nothing else is touched: git removes the rest.
fn unlink_untracked_links(exec: &GitExec, at: &WorktreePath, root: &Path) -> Result<(), GitError> {
    let out = exec.run(GitOp::LsFiles, at, ["ls-files", "-z"])?;
    let tracked: std::collections::BTreeSet<String> = out
        .stdout
        .split(|b| *b == 0)
        .filter(|f| !f.is_empty())
        .map(|f| String::from_utf8_lossy(f).into_owned())
        .collect();
    let failed = |p: &Path, e: &std::io::Error| GitError::WorktreeRemoveFailed {
        path: p.to_path_buf(),
        stderr: format!("could not unlink `{}` before removal: {e}", p.display()).into_bytes(),
    };
    let mut stack: Vec<(PathBuf, String)> = vec![(root.to_path_buf(), String::new())];
    while let Some((dir, rel)) = stack.pop() {
        let entries = std::fs::read_dir(&dir).map_err(|e| failed(&dir, &e))?;
        for entry in entries {
            let entry = entry.map_err(|e| failed(&dir, &e))?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if rel.is_empty() && name == ".git" {
                continue;
            }
            let p = entry.path();
            let r = if rel.is_empty() {
                name
            } else {
                format!("{rel}/{name}")
            };
            let md = std::fs::symlink_metadata(&p).map_err(|e| failed(&p, &e))?;
            if md.file_type().is_symlink() || is_reparse_point(&md) {
                if !tracked.contains(&r) {
                    remove_link(&p, &md).map_err(|e| failed(&p, &e))?;
                }
            } else if md.is_dir() {
                stack.push((p, r));
            }
        }
    }
    Ok(())
}

/// Remove one link itself. A directory link (a junction, or a directory symlink on Windows) is
/// removed with `remove_dir`, which removes the link and never its target; anything else with
/// `remove_file`.
fn remove_link(p: &Path, md: &std::fs::Metadata) -> std::io::Result<()> {
    if is_directory_link(md) {
        std::fs::remove_dir(p)
    } else {
        std::fs::remove_file(p)
    }
}

#[cfg(windows)]
fn is_directory_link(md: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt as _;
    const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
    md.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0
}

#[cfg(not(windows))]
const fn is_directory_link(_md: &std::fs::Metadata) -> bool {
    false
}

/// **Remove the run tree only when it is clean, landed, marked and unlocked** (s8d-ii §8d-ii.3).
///
/// W2: by `git worktree remove` **without `--force`**, never following a link (W1). Anything
/// else is kept, with its reason. The trigger is `s8e`'s (the lane's `closed` mark); this is the
/// rule. The branch is kept: it is the ref the operator merges.
///
/// **Landed** (Phase 1 has no integration ref): `HEAD` symbolic on `rk-run/<item>` at its tip,
/// an empty status, and the tip's `Story:` trailer the lane's terminal `<project>/<item> closed`
/// mark.
///
/// # Errors
/// [`GitError::WorktreeRemoveFailed`] when git refuses the unforced removal or a residue is left
/// (Halt 6: typed, never retried, never forced); wrapper errors.
pub fn remove_landed(exec: &GitExec, start: &Path, item: &str) -> Result<Removal, GitError> {
    let main = main_worktree(exec, start)?;
    let id = ItemId::parse(item)?;
    let tree = derive(&main, &id);
    let main_at = WorktreePath::new(&main)?;
    let kept = |reason: KeepReason| {
        Ok(Removal::Kept {
            path: tree.path.clone(),
            reason,
        })
    };
    let records = worktree_records(exec, &main_at)?;
    let Some(rec) = record_at(&records, &tree.path) else {
        return kept(KeepReason::NotRegistered);
    };
    let at = WorktreePath::new(&tree.path)?;
    match read_rk_base(exec, &tree.path)? {
        Some(base) if base.item == id.as_str() => {}
        _ => return kept(KeepReason::Unmarked),
    }
    if let Some(reason) = &rec.locked {
        return kept(KeepReason::Locked {
            reason: reason.clone(),
        });
    }
    let full = format!("refs/heads/{}", tree.branch);
    let head_ref = exec
        .run(
            GitOp::RevParse,
            &at,
            ["rev-parse", "--symbolic-full-name", "HEAD"],
        )?
        .stdout_trimmed();
    if head_ref != full {
        return kept(KeepReason::NotOnRunRef { head: head_ref });
    }
    let (changed, _) = in_flight(exec, &at)?;
    if changed > 0 {
        return kept(KeepReason::Dirty { changed });
    }
    let story: Vec<String> = exec
        .run(
            GitOp::Log,
            &main_at,
            [
                "log",
                "-1",
                "--format=%(trailers:key=Story,valueonly=true)",
                full.as_str(),
            ],
        )?
        .stdout_trimmed()
        .lines()
        .map(|l| l.trim_end_matches('\r').to_string())
        .filter(|l| !l.is_empty())
        .collect();
    if !story.iter().any(|v| is_closed_mark(v, id.as_str())) {
        return kept(KeepReason::Unlanded { story });
    }
    unlink_untracked_links(exec, &at, &tree.path)?;
    match exec.run(
        GitOp::WorktreeRemove,
        &main_at,
        ["worktree", "remove", at.as_str()],
    ) {
        Ok(_) => {}
        Err(GitError::NonZeroExit { stderr, .. }) => {
            return Err(GitError::WorktreeRemoveFailed {
                path: tree.path.clone(),
                stderr,
            });
        }
        Err(other) => return Err(other),
    }
    if std::fs::symlink_metadata(&tree.path).is_ok() {
        return Err(GitError::WorktreeRemoveFailed {
            path: tree.path.clone(),
            stderr: b"git reported the removal done and a residue of the tree is still on disk"
                .to_vec(),
        });
    }
    Ok(Removal::Removed)
}

/// [`roll_back_fresh`]'s outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rollback {
    /// The fresh tree was deregistered and nothing of it is left on disk. The branch is kept.
    Removed,
    /// The tree is kept, and why: it was not this run's fresh tree, or git refused the unforced
    /// removal. It is never forced.
    Kept { path: PathBuf, reason: String },
}

/// **Roll back a FRESH run tree whose carry or `setup` failed** (D21-88 ruling 2; ADR-0180
/// decisions 9 and 11, proposed). *(dated 2026-09-27, s8d-ii build, package D2188.)*
///
/// D21-88: *"when a FRESH tree's carry or `setup` fails, the tree is removed (unforced, through
/// the same pre-unlink path) before the run refuses by name, as D21-48 X2's pointer scan already
/// does, so the next run creates it anew and re-carries. A reused tree is still not re-carried."*
///
/// Only the tree [`enter`] has just created for this run is touched: `entered.created` is not
/// [`Creation::Reused`], the tree is registered at its derived path, its `RK_BASE` names this
/// item and `entered.fork`, and it holds **this** run's own lock. Anything else is kept, and its
/// lock is left as it is. Then the lock is released, every untracked link is unlinked without
/// being followed (`remove_landed`'s pre-unlink, D21-88 ruling 1), and git's `worktree remove`
/// runs **without `--force`**. The branch is kept, so the next run checks it out anew
/// ([`Creation::OnExistingBranch`]) and re-carries. If git refuses, or a residue is left, the
/// tree is kept and named, unlocked, never forced.
///
/// # Errors
/// [`GitError::WorktreeRemoveFailed`] when a link cannot be unlinked; wrapper errors.
pub fn roll_back_fresh(
    exec: &GitExec,
    start: &Path,
    entered: &Entered,
    owner: &LockOwner,
) -> Result<Rollback, GitError> {
    let path = entered.tree.path.clone();
    let kept = |reason: String| {
        Ok(Rollback::Kept {
            path: path.clone(),
            reason,
        })
    };
    if entered.created == Creation::Reused {
        return kept("it is a reused tree, which is never rolled back or re-carried".to_string());
    }
    let main = main_worktree(exec, start)?;
    let main_at = WorktreePath::new(&main)?;
    let tree = derive(&main, &entered.tree.item);
    if fold_path(&tree.path) != fold_path(&entered.tree.path) {
        return kept("it is not the item's derived run-tree path".to_string());
    }
    let records = worktree_records(exec, &main_at)?;
    let Some(rec) = record_at(&records, &tree.path) else {
        return kept(KeepReason::NotRegistered.to_string());
    };
    match read_rk_base(exec, &tree.path)? {
        Some(base) if base.item == tree.item.as_str() && base.fork == entered.fork => {}
        _ => return kept(KeepReason::Unmarked.to_string()),
    }
    let ours = lock_reason(&tree.item, owner);
    if rec.locked.as_deref() != Some(ours.as_str()) {
        return kept(format!(
            "its lock is not this run's ({}), so this run did not just create it",
            rec.locked.as_deref().unwrap_or("unlocked")
        ));
    }
    let at = WorktreePath::new(&tree.path)?;
    exec.run(
        GitOp::WorktreeUnlock,
        &main_at,
        ["worktree", "unlock", at.as_str()],
    )?;
    unlink_untracked_links(exec, &at, &tree.path)?;
    let (removed, why) = remove_fresh(exec, &main_at, &tree)?;
    if removed {
        Ok(Rollback::Removed)
    } else {
        kept(why)
    }
}

// ---- W4: the filter-driver refusal and the post-checkout pointer scan ----
//
// s8d-ii §8d-ii.5 and §8d-ii.6 (D21-45 W4; D21-46 R8; D21-47 V1; D21-48 X2; ADR-0180 decisions
// 12 and 13, proposed). *(dated 2026-09-27, s8d-ii build, P4.)*

/// The Git LFS pointer header, the first line of every pointer file.
pub const LFS_POINTER_HEADER: &str = "version https://git-lfs.github.com/spec/v1";

/// One configuration value as `git config --show-scope --show-origin` prints it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigEntry {
    pub scope: String,
    pub origin: String,
    pub key: String,
    pub value: String,
}

/// A filter driver that at least one path selects at the fork commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedDriver {
    pub driver: String,
    /// Every path that selects it at the fork commit, in `ls-tree` order.
    pub paths: Vec<String>,
    /// Its effective value: the resolved `process`, else the resolved `smudge`.
    pub effective: Option<ConfigEntry>,
}

/// What the filter read found when it admitted creation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FilterCheck {
    pub selected: Vec<SelectedDriver>,
    /// Facts for the **debug** level only, never the default level (§8d-ii.5).
    pub notes: Vec<String>,
}

/// The scopes the driver read covers: `git config` without a scope flag reads the resolved
/// configuration, and `--show-scope` names each value's (R8: *"The effective config at every
/// scope."*). Never narrowed to one file.
const SCOPES_READ: &str = "system, global, local, worktree and command";

/// §8d-ii.5's reads (1) to (4), before `git worktree add`, and D21-47 V1's refusal.
///
/// V1: *"**Refuse only when pointer files would result.** That is, when a tracked path at the
/// fork commit selects a filter whose driver is NOT effective at any scope, or smudge is
/// skipped."*
///
/// 1. The selecting paths **at the fork commit**: `check-attr -z --source=<fork> --stdin
///    filter` over `ls-tree -r -z --name-only <fork>`, never the working tree's attributes.
/// 2. The drivers at every scope: `config -z --show-scope --show-origin --includes
///    --get-regexp '^(filter|includeif)\.'`, run in the main worktree.
/// 3. Every `includeIf` whose condition matches the **run tree** (its gitdir
///    `<common-dir>/worktrees/<name>`, where `<name>` is the item, suffixed as git suffixes a
///    basename collision: `admin_dir_name`, dated 2026-09-28, s8d-ii build, package R1FIX-8dii;
///    or its branch `rk-run/<item>`) is read from the file it
///    names and folded in at its own position, with its includer's scope and the included file
///    as origin. A condition this module cannot evaluate (`~`, `hasconfig:`) is not folded; it
///    goes to the debug notes, and D21-48 X2's scan is the backstop.
/// 4. `lfs_skip_smudge`, `GIT_LFS_SKIP_SMUDGE` as `rk-cli` reads it through `env.rs`.
///
/// A selected driver is **effective** when a non-empty `filter.<name>.process` or `.smudge`
/// value resolves (the last value read). Refuses (a) when one is not; (b) when
/// `lfs_skip_smudge` is set to a value other than `0` (empty reads as unset), or a selected
/// driver's resolved `smudge` value carries `--skip`. Admits otherwise, **silently**: the facts
/// go to [`FilterCheck::notes`] for the debug level only. Every call is a READ.
///
/// # Errors
/// [`WorktreeRefusal::FilterDriverNotEffective`] and [`WorktreeRefusal::SmudgeSkipped`], each
/// naming the driver, one selecting path and the scope and origin; wrapper errors.
pub fn check_filters(
    exec: &GitExec,
    main: &Path,
    item: &ItemId,
    fork: &str,
    lfs_skip_smudge: Option<&str>,
) -> Result<FilterCheck, GitError> {
    let at = WorktreePath::new(main)?;
    let selected_paths = selecting_paths(exec, &at, fork)?;
    let (entries, mut notes) = driver_entries(exec, &at, item)?;
    let resolved = |driver: &str, var: &str| -> Option<ConfigEntry> {
        let key = format!("filter.{driver}.{var}");
        entries.iter().rev().find(|e| e.key == key).cloned()
    };
    let mut selected: Vec<SelectedDriver> = Vec::new();
    for (driver, paths) in selected_paths {
        let effective = resolved(&driver, "process")
            .filter(|e| !e.value.trim().is_empty())
            .or_else(|| resolved(&driver, "smudge").filter(|e| !e.value.trim().is_empty()));
        selected.push(SelectedDriver {
            driver,
            paths,
            effective,
        });
    }
    // (a) a selected driver effective at no scope.
    for d in &selected {
        if d.effective.is_none() {
            return Err(refuse(WorktreeRefusal::FilterDriverNotEffective {
                driver: d.driver.clone(),
                path: d.paths[0].clone(),
                scopes: scopes_read(&notes),
            }));
        }
    }
    // (b) smudge skipped by the environment ...
    if let (Some(v), Some(d)) = (lfs_skip_smudge, selected.first())
        && !v.is_empty()
        && v != "0"
    {
        return Err(refuse(WorktreeRefusal::SmudgeSkipped {
            driver: d.driver.clone(),
            path: d.paths[0].clone(),
            scope: "environment".into(),
            origin: format!("GIT_LFS_SKIP_SMUDGE={v}"),
        }));
    }
    // ... or by a resolved `smudge` value carrying `--skip`.
    for d in &selected {
        if let Some(e) = resolved(&d.driver, "smudge")
            && e.value.split_whitespace().any(|t| t == "--skip")
        {
            return Err(refuse(WorktreeRefusal::SmudgeSkipped {
                driver: d.driver.clone(),
                path: d.paths[0].clone(),
                scope: e.scope,
                origin: e.origin,
            }));
        }
    }
    for d in &selected {
        if let Some(e) = &d.effective {
            notes.push(format!(
                "filter driver `{}` selected by {} path(s) at {fork} (first `{}`) is effective \
                 at {} scope ({}: `{}`); admitted",
                d.driver,
                d.paths.len(),
                d.paths[0],
                e.scope,
                e.origin,
                e.key
            ));
        }
    }
    let mut declared: Vec<&str> = entries
        .iter()
        .filter_map(|e| e.key.strip_prefix("filter."))
        .filter_map(|rest| rest.rsplit_once('.').map(|(name, _)| name))
        .filter(|name| !selected.iter().any(|d| d.driver == *name))
        .collect();
    declared.sort_unstable();
    declared.dedup();
    for name in declared {
        notes.push(format!(
            "filter driver `{name}` is declared and selected by no path at {fork}; admitted"
        ));
    }
    Ok(FilterCheck { selected, notes })
}

/// "every scope read", with any run-tree `includeIf` file folded in (from the notes).
fn scopes_read(notes: &[String]) -> String {
    let folded: Vec<&str> = notes
        .iter()
        .filter_map(|n| n.strip_prefix(FOLDED_NOTE))
        .collect();
    if folded.is_empty() {
        format!("{SCOPES_READ}, every include followed")
    } else {
        format!(
            "{SCOPES_READ}, every include followed, and the run tree's includeIf file(s) {}",
            folded.join(", ")
        )
    }
}

const FOLDED_NOTE: &str = "folded the run tree's includeIf file ";

/// Read (1): `driver -> every path selecting it at `fork``, drivers in name order. `unspecified`,
/// `unset` and a bare `set` name no driver.
fn selecting_paths(
    exec: &GitExec,
    at: &WorktreePath,
    fork: &str,
) -> Result<std::collections::BTreeMap<String, Vec<String>>, GitError> {
    let listed = exec.run(
        GitOp::LsTree,
        at,
        ["ls-tree", "-r", "-z", "--name-only", fork],
    )?;
    let mut out: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    if listed.stdout.iter().all(|b| *b == 0) {
        return Ok(out);
    }
    let source = format!("--source={fork}");
    let attrs = exec.run_with_stdin(
        GitOp::CheckAttr,
        at,
        ["check-attr", "-z", source.as_str(), "--stdin", "filter"],
        &listed.stdout,
    )?;
    let fields: Vec<&[u8]> = attrs.stdout.split(|b| *b == 0).collect();
    for rec in fields.chunks(3) {
        if let [path, _attr, value] = rec {
            let value = String::from_utf8_lossy(value);
            if matches!(value.as_ref(), "unspecified" | "unset" | "set" | "") {
                continue;
            }
            out.entry(value.into_owned())
                .or_default()
                .push(String::from_utf8_lossy(path).into_owned());
        }
    }
    Ok(out)
}

/// Parse `config -z` output: `[scope NUL] origin NUL key LF value NUL`, repeated.
fn parse_config_z(stdout: &[u8], with_scope: Option<&str>) -> Vec<ConfigEntry> {
    let fields: Vec<String> = stdout
        .split(|b| *b == 0)
        .map(|f| String::from_utf8_lossy(f).into_owned())
        .collect();
    let width = if with_scope.is_some() { 2 } else { 3 };
    let mut out = Vec::new();
    for rec in fields.chunks(width) {
        let (scope, origin, kv) = match (with_scope, rec) {
            (Some(s), [origin, kv]) => (s.to_string(), origin.clone(), kv.clone()),
            (None, [scope, origin, kv]) => (scope.clone(), origin.clone(), kv.clone()),
            _ => continue,
        };
        let (key, value) = kv.split_once('\n').unwrap_or((kv.as_str(), ""));
        out.push(ConfigEntry {
            scope,
            origin,
            key: key.to_string(),
            value: value.to_string(),
        });
    }
    out
}

/// A config read whose exit 1 means *nothing matched*.
fn config_read(exec: &GitExec, at: &WorktreePath, args: &[&str]) -> Result<Vec<u8>, GitError> {
    match exec.run(GitOp::Config, at, args) {
        Ok(out) => Ok(out.stdout),
        Err(GitError::NonZeroExit { code: Some(1), .. }) => Ok(Vec::new()),
        Err(other) => Err(other),
    }
}

/// Reads (2) and (3): every `filter.*` value at every scope, in git's order, with each
/// run-tree `includeIf` file folded in at its own position. Returns the values and the notes.
fn driver_entries(
    exec: &GitExec,
    at: &WorktreePath,
    item: &ItemId,
) -> Result<(Vec<ConfigEntry>, Vec<String>), GitError> {
    let raw = config_read(
        exec,
        at,
        &[
            "config",
            "-z",
            "--show-scope",
            "--show-origin",
            "--includes",
            "--get-regexp",
            r"^(filter|includeif)\.",
        ],
    )?;
    let mut notes = Vec::new();
    let common = common_dir_of(exec, at)?;
    let gitdirs = run_tree_gitdirs(&common, item);
    let branch = format!("{RUN_BRANCH_PREFIX}{}", item.as_str());
    let mut out = Vec::new();
    for e in parse_config_z(&raw, None) {
        let include = e
            .key
            .strip_prefix("includeif.")
            .and_then(|rest| rest.strip_suffix(".path"))
            .map(str::to_string);
        if e.key.starts_with("filter.") {
            out.push(e.clone());
        }
        let Some(condition) = include else {
            continue;
        };
        let here = Path::new(at.as_str());
        match include_matches_run_tree(&condition, &e.origin, here, &gitdirs, &branch) {
            Some(true) => {}
            Some(false) => continue,
            None => {
                notes.push(format!(
                    "includeIf `{condition}` ({}) was not evaluated for the run tree; the \
                     post-checkout scan is the backstop",
                    e.origin
                ));
                continue;
            }
        }
        let Some(file) = include_file(&e.origin, here, &e.value) else {
            notes.push(format!(
                "includeIf `{condition}` path `{}` ({}) could not be resolved; the \
                 post-checkout scan is the backstop",
                e.value, e.origin
            ));
            continue;
        };
        if !file.is_file() {
            // git ignores a missing include file, and so does this read.
            continue;
        }
        let file_s = WorktreePath::new(&file)?;
        let folded = config_read(
            exec,
            at,
            &[
                "config",
                "-z",
                "--show-origin",
                "--includes",
                "--file",
                file_s.as_str(),
                "--get-regexp",
                r"^filter\.",
            ],
        )?;
        notes.push(format!(
            "{FOLDED_NOTE}`{}` (`{condition}`)",
            file_s.as_str()
        ));
        out.extend(parse_config_z(&folded, Some(&e.scope)));
    }
    Ok((out, notes))
}

/// The name `git worktree add` will give the run tree's admin directory under
/// `<common>/worktrees/`: the tree path's basename (the item), with a number appended while that
/// name exists (git's `add_worktree`: `mkdir` of `worktrees/<name>`, then `<name>1`, `<name>2`, …
/// on `EEXIST`). Round 1 measured the suffix on git 2.55.0.windows.3: `../a/item`, then
/// `../b/item`, left `item` and `item1`, and in this repository the legacy tree already holds
/// `worktrees/rk-fix-doctor-test-count`. Read before the add, as read (3) is; an admin directory
/// created between this read and the add (a concurrent `worktree add`) is the one case it can
/// miss, and D21-48 X2's scan is the backstop. *(dated 2026-09-28, s8d-ii build, package
/// R1FIX-8dii: read (3) assumed `worktrees/<item>`.)*
fn admin_dir_name(common: &str, item: &ItemId) -> String {
    let admin = Path::new(common).join("worktrees");
    let mut name = item.as_str().to_string();
    let mut n: u32 = 0;
    while std::fs::symlink_metadata(admin.join(&name)).is_ok() {
        n = n.saturating_add(1);
        name = format!("{}{n}", item.as_str());
    }
    name
}

/// The run tree's gitdir as git would match an `includeIf "gitdir:"` against it: the canonical
/// common dir's `worktrees/<name>` first (git tries the realpath first), then the absolute form,
/// where `<name>` is [`admin_dir_name`]'s, the directory git will assign.
fn run_tree_gitdirs(common: &str, item: &ItemId) -> Vec<String> {
    let name = admin_dir_name(common, item);
    let absolute = format!("{}/worktrees/{name}", common.trim_end_matches('/'));
    let mut out = Vec::new();
    if let Ok(canon) = std::fs::canonicalize(common) {
        out.push(format!(
            "{}/worktrees/{name}",
            shown(&canon).to_string_lossy().trim_end_matches('/'),
        ));
    }
    if !out.contains(&absolute) {
        out.push(absolute);
    }
    out
}

/// Does `condition` (an `includeIf` condition) match the run tree? `None` when this module does
/// not evaluate it: `~` patterns (no environment is read in this crate; ADR-0083), `hasconfig:`
/// and anything unknown.
fn include_matches_run_tree(
    condition: &str,
    origin: &str,
    here: &Path,
    gitdirs: &[String],
    branch: &str,
) -> Option<bool> {
    if let Some(p) = condition.strip_prefix("onbranch:") {
        let mut pat = p.to_string();
        if pat.ends_with('/') {
            pat.push_str("**");
        }
        return Some(wildmatch(pat.as_bytes(), branch.as_bytes(), false));
    }
    let (pattern, icase) = if let Some(p) = condition.strip_prefix("gitdir/i:") {
        (p, true)
    } else {
        (condition.strip_prefix("gitdir:")?, false)
    };
    if pattern.starts_with('~') {
        return None;
    }
    // git's prepare_include_condition_pattern: `./` is the including file's directory, a
    // relative pattern gets `**/`, and a trailing `/` gets `**`.
    let (mut pat, prefix) = if let Some(rest) = pattern
        .strip_prefix("./")
        .or_else(|| pattern.strip_prefix(".\\"))
    {
        let file = origin_file(origin, here)?;
        let dir = std::fs::canonicalize(file)
            .ok()?
            .parent()
            .map(|d| shown(d).to_string_lossy().trim_end_matches('/').to_string())?;
        let prefix = dir.len() + 1;
        (format!("{dir}/{rest}"), prefix)
    } else if is_absolute_pattern(pattern) {
        (pattern.to_string(), 0)
    } else {
        (format!("**/{pattern}"), 0)
    };
    if pat.ends_with('/') {
        pat.push_str("**");
    }
    Some(gitdirs.iter().any(|text| {
        let (p, t) = (pat.as_bytes(), text.as_bytes());
        if prefix > 0 {
            if t.len() < prefix {
                return false;
            }
            let same = if icase {
                p[..prefix].eq_ignore_ascii_case(&t[..prefix])
            } else {
                p[..prefix] == t[..prefix]
            };
            same && wildmatch(&p[prefix..], &t[prefix..], icase)
        } else {
            wildmatch(p, t, icase)
        }
    }))
}

fn is_absolute_pattern(p: &str) -> bool {
    let b = p.as_bytes();
    p.starts_with('/')
        || p.starts_with('\\')
        || (b.len() >= 3
            && b[0].is_ascii_alphabetic()
            && b[1] == b':'
            && matches!(b[2], b'/' | b'\\'))
}

/// The config file an origin names (`file:<path>`), absolute: git prints a repository's own
/// files relative to the checkout it ran in (`file:.git/config`), so a relative one is joined
/// to `here`, that checkout. Never resolved against the process's current directory (AC-529).
fn origin_file(origin: &str, here: &Path) -> Option<PathBuf> {
    let file = origin.strip_prefix("file:")?;
    if is_absolute_pattern(file) {
        Some(PathBuf::from(file))
    } else {
        Some(here.join(file))
    }
}

/// The file an `include.path` / `includeIf.<c>.path` value names: absolute as it stands, else
/// relative to the directory of the config file that holds it. `None` for `~` (no environment is
/// read here) or a relative path in a value that came from no file.
fn include_file(origin: &str, here: &Path, value: &str) -> Option<PathBuf> {
    if value.starts_with('~') || value.is_empty() {
        return None;
    }
    if is_absolute_pattern(value) {
        return Some(PathBuf::from(value));
    }
    Some(origin_file(origin, here)?.parent()?.join(value))
}

/// git's `wildmatch` with `WM_PATHNAME`: `*` and `?` never match `/`; `**` between slashes (or
/// at an end) matches across them; `[...]` classes; `\` escapes.
fn wildmatch(p: &[u8], t: &[u8], icase: bool) -> bool {
    wm(p, 0, t, 0, icase)
}

fn wm(p: &[u8], mut pi: usize, t: &[u8], mut ti: usize, icase: bool) -> bool {
    let eq = |a: u8, b: u8| {
        if icase {
            a.eq_ignore_ascii_case(&b)
        } else {
            a == b
        }
    };
    while pi < p.len() {
        match p[pi] {
            b'*' => {
                let mut end = pi;
                while end < p.len() && p[end] == b'*' {
                    end += 1;
                }
                let double = end - pi >= 2;
                let at_start = pi == 0 || p[pi - 1] == b'/';
                let at_end = end == p.len() || p[end] == b'/';
                if double && at_start && at_end {
                    if end == p.len() {
                        return true;
                    }
                    let rest = end + 1;
                    if wm(p, rest, t, ti, icase) {
                        return true;
                    }
                    return (ti..t.len()).any(|k| t[k] == b'/' && wm(p, rest, t, k + 1, icase));
                }
                let mut k = ti;
                loop {
                    if wm(p, end, t, k, icase) {
                        return true;
                    }
                    if k >= t.len() || t[k] == b'/' {
                        return false;
                    }
                    k += 1;
                }
            }
            b'?' => {
                if ti >= t.len() || t[ti] == b'/' {
                    return false;
                }
                pi += 1;
                ti += 1;
            }
            b'[' => {
                let Some((matched, next)) = class_match(p, pi, t.get(ti).copied(), icase) else {
                    if ti >= t.len() || !eq(b'[', t[ti]) {
                        return false;
                    }
                    pi += 1;
                    ti += 1;
                    continue;
                };
                if ti >= t.len() || t[ti] == b'/' || !matched {
                    return false;
                }
                pi = next;
                ti += 1;
            }
            b'\\' if pi + 1 < p.len() => {
                if ti >= t.len() || !eq(p[pi + 1], t[ti]) {
                    return false;
                }
                pi += 2;
                ti += 1;
            }
            c => {
                if ti >= t.len() || !eq(c, t[ti]) {
                    return false;
                }
                pi += 1;
                ti += 1;
            }
        }
    }
    ti == t.len()
}

/// A `[...]` class at `p[start]`: `Some((matched, index after ']'))`, or `None` when unclosed.
fn class_match(p: &[u8], start: usize, c: Option<u8>, icase: bool) -> Option<(bool, usize)> {
    let mut i = start + 1;
    let negate = matches!(p.get(i), Some(b'!' | b'^'));
    if negate {
        i += 1;
    }
    let c = c.map(|c| if icase { c.to_ascii_lowercase() } else { c });
    let mut matched = false;
    let mut first = true;
    while i < p.len() {
        if p[i] == b']' && !first {
            return Some((matched != negate, i + 1));
        }
        first = false;
        let lo = p[i];
        let (lo, hi, step) = if i + 2 < p.len() && p[i + 1] == b'-' && p[i + 2] != b']' {
            (lo, p[i + 2], 3)
        } else {
            (lo, lo, 1)
        };
        if let Some(c) = c {
            let (lo, hi) = if icase {
                (lo.to_ascii_lowercase(), hi.to_ascii_lowercase())
            } else {
                (lo, hi)
            };
            if lo <= c && c <= hi {
                matched = true;
            }
        }
        i += step;
    }
    None
}

/// D21-48 X2's post-checkout pointer scan.
///
/// Every path read (1) found selecting a driver is read in the FRESH tree (never the blob at
/// the fork commit). A hit is a regular file whose first line, one trailing CR
/// dropped, is [`LFS_POINTER_HEADER`]; only that line's bytes are read. A selected path the tree
/// does not hold as a regular file is skipped. Returns `(path, driver)` per hit, in read order.
///
/// # Errors
/// [`WorktreeRefusal::SegmentUnreadable`] when a selected regular file cannot be read.
pub fn scan_for_pointers(
    tree: &Path,
    check: &FilterCheck,
) -> Result<Vec<(String, String)>, GitError> {
    let mut hits = Vec::new();
    for d in &check.selected {
        for rel in &d.paths {
            let p = tree.join(rel);
            match std::fs::symlink_metadata(&p) {
                Ok(md) if md.is_file() => {}
                _ => continue,
            }
            if first_line_is_pointer_header(&p).map_err(|e| {
                refuse(WorktreeRefusal::SegmentUnreadable {
                    path: p.clone(),
                    detail: e.to_string(),
                })
            })? {
                hits.push((rel.clone(), d.driver.clone()));
            }
        }
    }
    Ok(hits)
}

/// Read at most the header's length plus a CR and an LF, and compare the first line.
fn first_line_is_pointer_header(p: &Path) -> std::io::Result<bool> {
    use std::io::Read as _;
    let cap = LFS_POINTER_HEADER.len() + 2;
    let mut buf = Vec::with_capacity(cap);
    std::fs::File::open(p)?
        .take(cap as u64)
        .read_to_end(&mut buf)?;
    let line = buf
        .iter()
        .position(|b| *b == b'\n')
        .map_or(buf.as_slice(), |n| &buf[..n]);
    let line = line.strip_suffix(b"\r").unwrap_or(line);
    Ok(line == LFS_POINTER_HEADER.as_bytes())
}

/// The scan's rollback (D21-48 X2): the tree this call created, carrying this call's `RK_BASE`
/// and not yet locked, is removed by `git worktree remove` WITHOUT `--force` (on ADR-0105 clause
/// 5's list), and the branch is kept. If git refuses, the tree is kept and named, never forced.
fn pointer_refusal(
    exec: &GitExec,
    main: &WorktreePath,
    tree: &RunTree,
    check: &FilterCheck,
    hits: &[(String, String)],
) -> Result<GitError, GitError> {
    let (first, driver) = hits[0].clone();
    let effective = check
        .selected
        .iter()
        .find(|d| d.driver == driver)
        .and_then(|d| d.effective.clone());
    let (scope, origin) = effective.map_or_else(
        || ("none".to_string(), "no effective value".to_string()),
        |e| (e.scope, e.origin),
    );
    let (removed, kept_because) = remove_fresh(exec, main, tree)?;
    Ok(refuse(WorktreeRefusal::PointerFilesAfterCheckout(
        Box::new(PointerHits {
            tree: tree.path.clone(),
            driver,
            hits: hits.len(),
            first,
            scope,
            origin,
            removed,
            kept_because,
        }),
    )))
}

/// Remove the tree this call created, unforced: `(true, "")` when git removed it and nothing is
/// left on disk, else `(false, why it is kept)`.
fn remove_fresh(
    exec: &GitExec,
    main: &WorktreePath,
    tree: &RunTree,
) -> Result<(bool, String), GitError> {
    let at = WorktreePath::new(&tree.path)?;
    match exec.run(
        GitOp::WorktreeRemove,
        main,
        ["worktree", "remove", at.as_str()],
    ) {
        Ok(_) if std::fs::symlink_metadata(&tree.path).is_err() => Ok((true, String::new())),
        Ok(_) => Ok((
            false,
            "git reported the removal done and the tree is still on disk".to_string(),
        )),
        Err(GitError::NonZeroExit { stderr, .. }) => Ok((
            false,
            format!(
                "git refused the unforced removal: {}",
                String::from_utf8_lossy(&stderr).trim()
            ),
        )),
        Err(other) => Err(other),
    }
}
