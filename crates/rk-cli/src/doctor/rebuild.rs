//! **`doctor --rebuild` — git → store, never the reverse** (AC-427, AC-429; ADR-0105 clauses 1 and
//! 7; `s7d` §7d.8).
//!
//! The direction is the whole criterion. `GitOp::Log` over the trailer commits →
//! `rk_git::reconcile` → [`events_from_trailers`] → `rk_kernel::workflow::fsm::fold` per item →
//! `fsm::stage_action`, written to a **new store file and renamed over the old** (ADR-0105 clause
//! 7: *"a rebuild writes to a separate file and renames"*). AC-429's THEN is that **no git object,
//! ref or working-tree file** was written on the way.
//!
//! # 🛑 `Gate:` MARKS ARE NOT REBUILT AS `gate_opened` ROWS, and the reason is measured
//!
//! The fold's `gate_opened` arm REQUIRES `kind` and `deadline` and **derives** the token from
//! `(kind, tree_hash, deadline, reach)`, ignoring any `token` in the payload
//! [`crates/rk-kernel/src/workflow/fsm.rs`]. The `Gate:` trailer carries `(token, tree_hash,
//! deadline)` and no `kind` or `reach`. So a row built from a trailer is either `MalformedPayload`
//! or derives a token that is **not** the trailer's — the rebuild would invent a gate identity.
//! `rk_git::reconcile::GateRecord` is recorded as the fact it is and listed as *unresolvable from
//! git alone* until a record widens the triple or makes the token the fold's key (RAISED, §7d.18).
//! `ac427_…`'s synthetic repository therefore carries **no gate**.
//!
//! # 🛑 A `Story:` MARK'S `to` IS THE ITEM RECORD'S, AND THE REASON IS MEASURED
//!
//! *Operator decision of 2026-09-21; §7d.8 amended that date. Do not re-derive it.*
//!
//! §7d.8 prescribed payloads *"built from the trailer values and the commit sha"*. That cannot be
//! built: the fold's `StageTransition` arm makes **`to` mandatory** (`from` is optional and
//! defaults to the fold's running stage), while `ParsedTrailer::Story` is `{ project, slug }` and
//! `ProjectState.stories_committed` is a `BTreeSet<String>` of slugs — **no stage anywhere**
//! [measured 2026-09-21]. Reading the item record *at the trailer commit's tree* is foreclosed
//! too: `GitOp`'s read set is `RevParse | Status | Log` with no blob read, and adding a variant is
//! *"a successor record"* over a crate §7d.15 declares not edited.
//!
//! So the third payload source is **`.redkiln/items/<slug>.toml`'s `stage` key in the checkout** —
//! the only in-repo source of a stage at this pin, and the same value RT-50 makes authoritative
//! for the item everywhere else. **Its limit travels with it: only the LAST transition per item is
//! recoverable**, so this module emits **one** `stage_transition` per committed item, anchored on
//! that item's last `Story:` commit. A mark whose record is absent or carries no `stage` is
//! **listed unresolvable, never coerced** (AC-165), exactly as a `Gate:` mark is.
//!
//! *(Dated 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.5; ADR-0183, proposed.)* **The rule
//! above is now the LEGACY rule.** The `Story:` grammar widened to `<project>/<slug> <stage>`, so
//! a stage-bearing mark gives its own `to`, anchored on its own commit (`reconcile`'s latest mark
//! per item, `ProjectState::stages`). Only a stage-less mark — the one legacy mark on the tree,
//! `rk-run/rk-fix-doctor-test-count` at `73c087a` — still falls back to the item record. The
//! module's move into `rk-adapter-sqlite` and a row per boundary are `s8e` §8e.9's.
//!
//! # 🛑 The module-path ban: nothing here re-opens the store
//!
//! Checkpoint 7d's D-10 ban admits both `src/doctor/*.rs` and `src/verbs/doctor.rs`, and
//! `SqliteStore::open` is on its pattern. That is not incidental to this file — it is the one that
//! would most naturally reach for it. **`Ctx.store` is `None` on this path** and the rebuilt store
//! is re-opened for its identity check **by the root**, in `adapters.rs`, after `verbs::doctor` has
//! returned. On Windows this is also a correctness rule and not only a layering one: `Writer` holds
//! a `Mutex<Connection>` in WAL mode, and `std::fs::rename` over a file another handle holds open
//! without `FILE_SHARE_DELETE` fails with a sharing violation. `ac427_…` runs on Windows and is red
//! on exactly that.
//!
//! # 🛑 ONE STATEMENT, WRITTEN TWICE, AND IT IS DISCLOSED RATHER THAN FIXED
//!
//! [`run`] inserts each rebuilt event through `Writer::transaction` with this crate's own
//! `INSERT INTO t2_event (kind, payload, anchor_sha)`, duplicating `SqliteStore::append`'s
//! statement — which hard-codes `anchor_sha` NULL and is the only event `INSERT` on the tree, and
//! `EventStore::rebuild` on `SqliteStore` is a typed `Capability::Rebuild` refusal. §7d.8 discloses
//! the second copy and §7d.18 RAISES an `rk-adapter-sqlite` `open_at(path)` / `Capability::Rebuild`
//! implementation for a record. It is a closed-crate edit this stage does not make.
//!
//! # *(dated 2026-09-28, `s8e` build, package S8E-P7)* THE REBUILD MOVED INTO `rk-adapter-sqlite`
//!
//! `s8e` §8e.9; D21-42 8c: *"Moves into rk-adapter-sqlite (`open_at` / `Capability::Rebuild`, the
//! [s7d] §7d.18 RAISED item). rk-cli's duplicated INSERT is retired, and criterion (2) is
//! re-measured against ADR-0105 cl.9's real projection."* ADR-0181 decision 11 (proposed); D21-48
//! X10. **Three of the sections above are superseded, and each stays as the record of what was:**
//!
//! - ~~`Gate:` MARKS ARE NOT REBUILT~~ — the value is widened (ADR-0184), and every gate
//!   transition is rebuilt as its `t1_gate_transition` row, anchored to its own mark. A rebuilt
//!   `opened` row carries no opening run (D21-48 X11): the value carries none, and nothing is
//!   invented.
//! - ~~A `Story:` MARK'S `to` IS THE ITEM RECORD'S~~ — for a stage-bearing mark, `to` is the
//!   mark's own (ADR-0183), and every boundary is a row, `from` the previous mark on the ref. The
//!   item-record rule survives for the LEGACY stage-less mark only, as it did after S8E-P4, and
//!   only for an item whose ref carries no stage-bearing mark.
//! - ~~ONE STATEMENT, WRITTEN TWICE~~ — **retired.** This module names no statement: it builds an
//!   `rk_ports::event_store::Snapshot` from git and hands it to the adapter's
//!   `EventStore::rebuild` over a store opened with `SqliteStore::open_at` on the staging file
//!   (through `crate::adapters::open_rebuild_target`, outside the D-10 ban's module filter). The
//!   adapter owns the SQL (`rk_store::tier_one::rebuild`), which is X10's reading of ADR-0158's
//!   bullet: *""From `rk-cli`" names where the write runs. The adapter's `Capability::Rebuild`
//!   owns the SQL, so there is no supersession."*
//!
//! **The rows are the heal's rows.** Each item's rows come from
//! `rk_kernel::runtime::tier_one::marked_commits` — every mark reachable from `rk-run/<item>`,
//! through the same mark-to-row derivation the entry heal uses — so a rebuilt row is the row the
//! live act anchored to that commit, and AC-184's projection compares equal
//! (`doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`). A mark no `rk-run/<item>`
//! ref reaches is listed unresolvable, never coerced (AC-165). The git read, the item-record
//! fallback and the order *"git → store, never the reverse"* stay here.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;
use rk_git::reconcile::{CommitTrailers, State};
use rk_git::trailer::grammar::ParsedTrailer;
use rk_ports::event_store::{EventStore, Snapshot, SnapshotRow};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// The item records' stages, by the slug the `Story:` trailer carries.
///
/// A slug with no entry here has no recoverable `to` for a LEGACY mark — see the header.
pub type ItemStages = BTreeMap<String, String>;

/// What the trailers could and could not be turned into.
///
/// **`unresolvable` is not a diagnostic afterthought.** It is the list AC-165 requires: a mark that
/// cannot be rebuilt is surfaced, never skipped and never coerced into a state transition.
///
/// *(dated 2026-09-28, `s8e` build, package S8E-P7.)* ~~`events: Vec<(Event, String)>`~~ — the
/// rows are a `Snapshot`'s, each anchored to its own trailer commit with that commit's tree.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rebuilt {
    /// Every rebuilt row, anchored, in ref order per item.
    pub rows: Vec<SnapshotRow>,
    /// Marks this rebuild could not resolve from git alone, each with the reason.
    pub unresolvable: Vec<String>,
}

/// What one rebuild did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// Rows written into the rebuilt store.
    pub events_written: usize,
    /// The source `HEAD`, which is also the `rebuilt` sidecar's content.
    pub head: String,
    /// As [`Rebuilt::unresolvable`].
    pub unresolvable: Vec<String>,
}

/// Why a rebuild refused. Every arm is exit `3` and every arm refuses **before** the rename.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RebuildError {
    /// A read through the wrapper failed. Carries `GitError`'s `Debug` form — it has no `Display`.
    Git(String),
    /// The rebuilt events do not fold: a payload the kernel refuses.
    Fold(String),
    /// The staging store could not be built or written.
    Store(String),
    /// The staging store could not be moved into place.
    Rename(String),
}

impl std::fmt::Display for RebuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git(d) => write!(f, "git could not be read: {d}"),
            Self::Fold(d) => write!(f, "the rebuilt events do not fold: {d}"),
            Self::Store(d) => write!(f, "the staging store could not be written: {d}"),
            Self::Rename(d) => write!(f, "the rebuilt store could not be moved into place: {d}"),
        }
    }
}

/// Every item a `Story:` mark names, across projects: the items whose `rk-run/<item>` ref the
/// rebuild walks.
fn marked_items(state: &State) -> BTreeSet<String> {
    state
        .projects
        .values()
        .flat_map(|p| p.stories_committed.iter().chain(p.stages.keys()))
        .cloned()
        .collect()
}

/// Read `.redkiln/items/<slug>.toml`'s `stage` for every item a `Story:` mark names.
///
/// A record that is absent, unparsable or carries no `stage` yields **no entry** — a legacy mark
/// is then listed unresolvable rather than given a default (RT-50: *"the stage is read once, from
/// the record, and is not defaulted"*).
#[must_use]
pub fn item_stages(root: &Path, state: &State) -> ItemStages {
    let mut out = ItemStages::new();
    let items = root.join(".redkiln").join("items");
    for slug in marked_items(state) {
        let Ok(src) = std::fs::read_to_string(items.join(format!("{slug}.toml"))) else {
            continue;
        };
        let Ok(doc) = toml::from_str::<toml::Value>(&src) else {
            continue;
        };
        if let Some(stage) = doc.get("stage").and_then(toml::Value::as_str) {
            out.insert(slug, stage.to_owned());
        }
    }
    out
}

/// **The snapshot git determines** (`s8e` §8e.9; ADR-0181 decision 11). *(dated 2026-09-28, `s8e`
/// build, package S8E-P7; this replaces `events_from_trailers`.)*
///
/// Per item a `Story:` mark names, the rows of every mark reachable from `rk-run/<item>`
/// (`tier_one::marked_commits`, the heal's derivation), each anchored to its commit and that
/// commit's tree: every boundary, every gate transition, every seal, the `closed` mark. For an item
/// whose ref carries only the legacy stage-less mark, one `stage_transition` whose `to` is the item
/// record's `stage`, anchored to the last such mark (the legacy rule; listed unresolvable when the
/// record has no stage). Every mark on a commit no item's run ref reaches, and every bare-id
/// `Story:`, is listed unresolvable (AC-165).
///
/// # Errors
/// [`RebuildError::Git`] from a read.
pub fn snapshot_from_git(
    exec: &GitExec,
    worktree: &WorktreePath,
    commits: &[CommitTrailers],
    state: &State,
    stages: &ItemStages,
) -> Result<Rebuilt, RebuildError> {
    let git = |e: rk_kernel::runtime::tier_one::TierOneError| RebuildError::Git(e.to_string());
    let mut out = Rebuilt::default();
    let mut reached: BTreeSet<String> = BTreeSet::new();
    for item in marked_items(state) {
        let marked =
            rk_kernel::runtime::tier_one::marked_commits(exec, worktree, &item).map_err(git)?;
        if marked.is_empty() {
            out.unresolvable.push(format!(
                "Story: {item} — no `rk-run/{item}` ref reaches its mark, and the rebuild derives \
                 rows only along an item's run ref"
            ));
            continue;
        }
        let stage_bearing = marked
            .iter()
            .any(|m| m.rows.iter().any(|r| r.kind == EventKind::StageTransition));
        let last_legacy = marked
            .iter()
            .rev()
            .find(|m| m.legacy_story)
            .map(|m| m.sha.clone());
        for commit in &marked {
            reached.insert(commit.sha.clone());
            let legacy_row =
                if !stage_bearing && last_legacy.as_deref() == Some(commit.sha.as_str()) {
                    match stages.get(&item) {
                        Some(to) => Some(
                            Event::new(EventKind::StageTransition, serde_json::json!({ "to": to }))
                                .with_item(&item),
                        ),
                        None => {
                            out.unresolvable.push(format!(
                                "Story: {item} at {} — a legacy stage-less mark, and \
                             `.redkiln/items/{item}.toml` declares no `stage`; the fold makes `to` \
                             mandatory, so no transition is written rather than one being invented \
                             (AC-165)",
                                commit.sha
                            ));
                            None
                        }
                    }
                } else {
                    None
                };
            if commit.rows.is_empty() && legacy_row.is_none() {
                continue;
            }
            let tree = rk_kernel::runtime::tier_one::commit_tree(exec, worktree, &commit.sha)
                .map_err(git)?;
            for row in legacy_row.into_iter().chain(commit.rows.iter().cloned()) {
                out.rows.push(SnapshotRow {
                    event: row,
                    anchor_sha: Some(commit.sha.clone()),
                    tree: Some(tree.clone()),
                    opening_run: None,
                });
            }
        }
    }

    // ---- a mark on a commit no item's run ref reaches: listed, never coerced.
    for commit in commits {
        if reached.contains(&commit.sha) {
            continue;
        }
        for trailer in &commit.trailers {
            let named = match trailer {
                ParsedTrailer::Story { project, slug, .. } => format!("Story: {project}/{slug}"),
                ParsedTrailer::Gate { token, state, .. } => {
                    format!("Gate: {token} {}", state.as_str())
                }
                ParsedTrailer::SliceVerdict {
                    project,
                    slice,
                    verdict,
                } => format!("Slice-Verdict: {project}/{slice} {}", verdict.as_str()),
                _ => continue,
            };
            out.unresolvable.push(format!(
                "{named} at {} — no `rk-run/<item>` ref reaches this commit, and the rebuild \
                 derives rows only along an item's run ref",
                commit.sha
            ));
        }
    }

    // ---- Bare-id `Story:` trailers carry no project, so they name no item record either.
    for id in &state.bare_story_ids {
        out.unresolvable.push(format!(
            "Story: {id} — a bare id with no `project/` form, which names no `.redkiln/items/` \
             record; folding it into a project would invent one (design §5.4 row 1)"
        ));
    }

    Ok(out)
}

/// The staging directory `run` builds into: `<common-dir>/redkiln/rebuild/`.
///
/// `SqliteStore::open_at` places the file at `<staging>/redkiln/store.sqlite3`, which is why the
/// rename below reaches two levels in rather than one.
#[must_use]
pub fn staging_dir(common_dir: &Path) -> PathBuf {
    common_dir.join("redkiln").join("rebuild")
}

/// Rebuild the store from git alone, into a staging file renamed over the old one.
///
/// # Errors
/// [`RebuildError`] — every arm refuses before anything is renamed, so a refused rebuild leaves the
/// journal it exists to reconstruct exactly where it was.
pub fn run(
    exec: &GitExec,
    worktree: &WorktreePath,
    common_dir: &Path,
    root: &Path,
) -> Result<Outcome, RebuildError> {
    // ---- (1) THE GIT READS. Every call below is a read, which is AC-429's whole clause.
    let commits = rk_git::reconcile::read_commits(exec, worktree)
        .map_err(|e| RebuildError::Git(format!("{e:?}")))?;
    let (state, _drift) = rk_git::reconcile::reconcile(&commits);
    let head = exec
        .run(GitOp::RevParse, worktree, ["rev-parse", "HEAD"])
        .map_err(|e| RebuildError::Git(format!("{e:?}")))?
        .stdout_trimmed();

    // ---- (2) THE SNAPSHOT git determines, the item records the legacy rule's third source.
    let stages = item_stages(root, &state);
    let rebuilt = snapshot_from_git(exec, worktree, &commits, &state, &stages)?;

    // ---- (3) THE FOLD, PER ITEM, AS VALIDATION — and it runs BEFORE anything is renamed.
    //
    // §7d.8 names `fsm::fold` per item → `fsm::stage_action`. What this buys is that a payload the
    // kernel would refuse is refused HERE, while the old journal is still in place. The rows are
    // partitioned by their interim `item` key (ADR-0182), the key the kernel's own reads select by.
    for item in marked_items(&state) {
        let stage = stages.get(&item).map_or("", String::as_str);
        let mine: Vec<Event> = rebuilt
            .rows
            .iter()
            .filter(|r| r.event.item().is_none_or(|i| i == item))
            .map(|r| r.event.clone())
            .collect();
        rk_kernel::workflow::fsm::stage_action(&item, stage, &mine)
            .map_err(|e| RebuildError::Fold(format!("{item}: {e:?}")))?;
    }

    // ---- (4) THE STAGING STORE, through the port. ADR-0105 clause 7: a rebuild writes to a
    // ---- separate file and renames. *(dated 2026-09-28, `s8e` build, package S8E-P7.)*
    // ---- ~~`rk_store`'s own `Writer` and a second copy of the INSERT~~ — the adapter's
    // ---- `open_at` and `EventStore::rebuild`; the identity is derived from git, and the
    // ---- repository's old store is never opened on the way (a version-1 store is rebuilt).
    let staging = staging_dir(common_dir);
    if staging.exists() {
        std::fs::remove_dir_all(&staging)
            .map_err(|e| RebuildError::Store(format!("the staging directory is not clean: {e}")))?;
    }
    std::fs::create_dir_all(&staging).map_err(|e| RebuildError::Store(e.to_string()))?;
    let identity = rk_adapter_sqlite::identity::opening(exec, worktree, common_dir)
        .map_err(|e| RebuildError::Store(format!("{e:?}")))?;
    let written = rebuilt.rows.len();
    let staged_file = {
        let mut staged = crate::adapters::open_rebuild_target(&staging, &identity)
            .map_err(|e| RebuildError::Store(e.to_string()))?;
        staged
            .rebuild(&Snapshot::new(rebuilt.rows))
            .map_err(|e| RebuildError::Store(format!("{e:?}")))?;
        let path = staged.path().to_path_buf();
        // 🛑 DROPPED BEFORE THE RENAME, AND ON WINDOWS THAT IS CORRECTNESS. The store holds a
        // `Mutex<Connection>` in WAL mode; a rename over a file another handle holds open without
        // `FILE_SHARE_DELETE` fails with a sharing violation. `ac427_…` runs here and is red on it.
        drop(staged);
        path
    };

    // ---- (5) THE RENAME, sidecars included.
    let target_dir = common_dir.join("redkiln");
    std::fs::create_dir_all(&target_dir).map_err(|e| RebuildError::Rename(e.to_string()))?;
    for suffix in ["", "-wal", "-shm"] {
        let from = staged_file.with_file_name(format!("store.sqlite3{suffix}"));
        let to = target_dir.join(format!("store.sqlite3{suffix}"));
        if from.is_file() {
            std::fs::rename(&from, &to).map_err(|e| {
                RebuildError::Rename(format!("{} → {}: {e}", from.display(), to.display()))
            })?;
        } else if to.is_file() {
            // A sidecar of the OLD database. Left in place it would be read as this one's, which
            // is how a WAL from another file corrupts the one renamed over it.
            std::fs::remove_file(&to).map_err(|e| {
                RebuildError::Rename(format!("a stale {} survived: {e}", to.display()))
            })?;
        }
    }
    let _ = std::fs::remove_dir_all(&staging);

    // ---- (6) THE MARKER `RK-D-REBUILT-TIER2` READS. RT-56 — ROUTED, not decided; the design
    // ---- §5 / §9.2 note naming how `doctor` learns a store was rebuilt is still OWED (§7d.18).
    std::fs::write(target_dir.join("rebuilt"), head.as_bytes())
        .map_err(|e| RebuildError::Store(format!("the `rebuilt` sidecar did not write: {e}")))?;

    Ok(Outcome {
        events_written: written,
        head,
        unresolvable: rebuilt.unresolvable,
    })
}
