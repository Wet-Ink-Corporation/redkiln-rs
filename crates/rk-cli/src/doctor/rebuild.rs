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

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;
use rk_git::reconcile::{CommitTrailers, State};
use rk_git::trailer::grammar::ParsedTrailer;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// The item records' stages, by the slug the `Story:` trailer carries.
///
/// A slug with no entry here has no recoverable `to` — see the header.
pub type ItemStages = BTreeMap<String, String>;

/// What the trailers could and could not be turned into.
///
/// **`unresolvable` is not a diagnostic afterthought.** It is the list AC-165 requires: a mark that
/// cannot be rebuilt is surfaced, never skipped and never coerced into a state transition.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rebuilt {
    /// Each rebuilt event beside the trailer commit's sha, which is its `anchor_sha`.
    pub events: Vec<(Event, String)>,
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

/// Read `.redkiln/items/<slug>.toml`'s `stage` for every slug the trailers committed.
///
/// A record that is absent, unparsable or carries no `stage` yields **no entry** — the mark is
/// listed unresolvable rather than given a default (RT-50: *"the stage is read once, from the
/// record, and is not defaulted"*).
#[must_use]
pub fn item_stages(root: &Path, state: &State) -> ItemStages {
    let mut out = ItemStages::new();
    let items = root.join(".redkiln").join("items");
    for project in state.projects.values() {
        for slug in &project.stories_committed {
            let Ok(src) = std::fs::read_to_string(items.join(format!("{slug}.toml"))) else {
                continue;
            };
            let Ok(doc) = toml::from_str::<toml::Value>(&src) else {
                continue;
            };
            if let Some(stage) = doc.get("stage").and_then(toml::Value::as_str) {
                out.insert(slug.clone(), stage.to_owned());
            }
        }
    }
    out
}

/// Turn the git-side reconciled state into the tier-1 events a rebuilt store holds.
///
/// **Stated so two implementers build one shape** (§7d.8, as amended 2026-09-21): `Story:` marks →
/// **one** `stage_transition` per item, `to` from `stages` and `anchor_sha` that item's last
/// `Story:` commit; `Slice-Verdict:` → `slice_sealed` per history entry, anchored on its own
/// commit; **`Gate:` marks → nothing** (the header). Tier-1 kinds only; **no tier-2 row**, which is
/// exactly what `RK-D-REBUILT-TIER2` reports as `ok`.
///
/// `commits` is carried beside `state` because the prescribed `&State` alone supplies neither the
/// `to` nor the commit sha the payload names — the header states why.
#[must_use]
pub fn events_from_trailers(
    commits: &[CommitTrailers],
    state: &State,
    stages: &ItemStages,
) -> Rebuilt {
    let mut out = Rebuilt::default();

    // ---- `Story:` → ONE `stage_transition` per item, anchored on that item's LAST mark.
    //
    // One per item and not one per mark, because `to` is read from the item record and the record
    // holds ONE stage: a second row would claim a transition into a stage no source names. The
    // limit is stated in the header and in `ac427_…`'s doc comment.
    let mut last_mark: BTreeMap<(&str, &str), &str> = BTreeMap::new();
    for commit in commits {
        for trailer in &commit.trailers {
            if let ParsedTrailer::Story { project, slug } = trailer {
                last_mark.insert((project.as_str(), slug.as_str()), commit.sha.as_str());
            }
        }
    }
    for project in state.projects.values() {
        for slug in &project.stories_committed {
            let item = format!("{}/{}", project.project, slug);
            let Some(sha) = last_mark.get(&(project.project.as_str(), slug.as_str())) else {
                out.unresolvable
                    .push(format!("Story: {item} — no commit carries this mark"));
                continue;
            };
            let Some(to) = stages.get(slug) else {
                out.unresolvable.push(format!(
                    "Story: {item} — `.redkiln/items/{slug}.toml` declares no `stage`, and the \
                     fold makes `to` mandatory, so no transition is written rather than one being \
                     invented (AC-165)"
                ));
                continue;
            };
            out.events.push((
                Event::new(
                    EventKind::StageTransition,
                    serde_json::json!({ "item": item, "to": to }),
                ),
                (*sha).to_owned(),
            ));
        }
    }

    // ---- `Slice-Verdict:` → `slice_sealed` PER HISTORY ENTRY, each on its own commit.
    //
    // The history and not just the latest verdict: F12 measured 20.3% of slices passing through a
    // rejected state, which is the whole reason the second axis exists (design §5.4). A rebuild
    // that wrote only `latest_verdict` would fold to the same `sealed` and throw that away.
    for project in state.projects.values() {
        for slice in project.slices.values() {
            for (sha, verdict) in &slice.history {
                out.events.push((
                    Event::new(
                        EventKind::SliceSealed,
                        serde_json::json!({
                            "slice": format!("{}/{}", project.project, slice.slice),
                            "verdict": verdict.as_str(),
                        }),
                    ),
                    sha.clone(),
                ));
            }
        }
    }

    // ---- `Gate:` → NOTHING, and every one of them said out loud. See the header.
    for (token, record) in &state.gates {
        out.unresolvable.push(format!(
            "Gate: {token} at {} — the fold derives a `gate_opened` token from `(kind, tree_hash, \
             deadline, reach)` and the trailer triple carries neither `kind` nor `reach`, so a \
             rebuilt row would invent a gate identity (§7d.18 RAISES the widening record)",
            record.commit
        ));
    }

    // ---- Bare-id `Story:` trailers carry no project, so they name no item record either.
    for id in &state.bare_story_ids {
        out.unresolvable.push(format!(
            "Story: {id} — a bare id with no `project/` form, which names no `.redkiln/items/` \
             record; folding it into a project would invent one (design §5.4 row 1)"
        ));
    }

    out
}

/// The staging directory `run` builds into: `<common-dir>/redkiln/rebuild/`.
///
/// `Writer::open` places the file at `<staging>/redkiln/store.sqlite3`, which is why the rename
/// below reaches two levels in rather than one.
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
    // ---- (1) THE ONE GIT READ. `read_commits` issues the single `GitOp::Log`; nothing below
    // ---- writes, which is AC-429's whole clause.
    let commits = rk_git::reconcile::read_commits(exec, worktree)
        .map_err(|e| RebuildError::Git(format!("{e:?}")))?;
    let (state, _drift) = rk_git::reconcile::reconcile(&commits);
    let head = exec
        .run(GitOp::RevParse, worktree, ["rev-parse", "HEAD"])
        .map_err(|e| RebuildError::Git(format!("{e:?}")))?
        .stdout_trimmed();

    // ---- (2) THE ITEM RECORDS, the third payload source (§7d.8 as amended 2026-09-21).
    let stages = item_stages(root, &state);
    let rebuilt = events_from_trailers(&commits, &state, &stages);

    // ---- (3) THE FOLD, PER ITEM, AS VALIDATION — and it runs BEFORE anything is renamed.
    //
    // §7d.8 names `fsm::fold` per item → `fsm::stage_action`. `fsm::next_action` is NOT called: it
    // needs a `Selection` a git-only rebuild does not have. What this buys is that a payload the
    // kernel would refuse is refused HERE, while the old journal is still in place, rather than
    // after a new store has been renamed over it.
    //
    // The filter is the one `ac427_…` applies on both sides: `fsm::fold` partitions nothing, so a
    // whole-journal fold would give every item the last row's transition.
    for project in state.projects.values() {
        for slug in &project.stories_committed {
            let item = format!("{}/{}", project.project, slug);
            let stage = stages.get(slug).map_or("", String::as_str);
            let mine: Vec<Event> = rebuilt
                .events
                .iter()
                .filter(|(e, _)| {
                    e.payload
                        .get("item")
                        .and_then(serde_json::Value::as_str)
                        .is_none_or(|i| i == item)
                })
                .map(|(e, _)| e.clone())
                .collect();
            rk_kernel::workflow::fsm::stage_action(&item, stage, &mine)
                .map_err(|e| RebuildError::Fold(format!("{item}: {e:?}")))?;
        }
    }

    // ---- (4) THE STAGING STORE. ADR-0105 clause 7: a rebuild writes to a separate file and
    // ---- renames. `SqliteStore::open` derives its path from the common dir and cannot target a
    // ---- second file, which is why this goes through `rk_store`'s own `Writer`.
    let staging = staging_dir(common_dir);
    if staging.exists() {
        std::fs::remove_dir_all(&staging)
            .map_err(|e| RebuildError::Store(format!("the staging directory is not clean: {e}")))?;
    }
    std::fs::create_dir_all(&staging).map_err(|e| RebuildError::Store(e.to_string()))?;
    let identity = rk_adapter_sqlite::identity::opening(exec, worktree, common_dir)
        .map_err(|e| RebuildError::Store(format!("{e:?}")))?;

    // Serialised out here, so the transaction below deals in one error type and a malformed
    // payload cannot leave a half-written store behind.
    let mut rows: Vec<(&'static str, String, &str)> = Vec::with_capacity(rebuilt.events.len());
    for (event, anchor) in &rebuilt.events {
        let payload = serde_json::to_string(&event.payload).map_err(|e| {
            RebuildError::Store(format!("a rebuilt payload will not serialise: {e}"))
        })?;
        rows.push((event.kind.wire_name(), payload, anchor.as_str()));
    }

    let staged_file = {
        let writer = rk_store::conn::Writer::open(&staging)
            .map_err(|e| RebuildError::Store(e.to_string()))?;
        writer
            .transaction(|tx| {
                for (_name, sql, _keys) in rk_store::schema_lint::all() {
                    tx.execute_batch(sql)?;
                }
                rk_store::identity::initialise(tx, &identity)?;
                for (kind, payload, anchor) in &rows {
                    // 🛑 THE SECOND COPY OF `SqliteStore::append`'s STATEMENT. Disclosed, not
                    // fixed — see the header. `append` hard-codes `anchor_sha` NULL and cannot
                    // target a staging file; `EventStore::rebuild` on it is a typed refusal.
                    tx.execute(
                        "INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2, ?3)",
                        (kind, payload, anchor),
                    )?;
                }
                Ok(())
            })
            .map_err(|e| RebuildError::Store(e.to_string()))?;
        let path = writer.path().to_path_buf();
        // 🛑 DROPPED BEFORE THE RENAME, AND ON WINDOWS THAT IS CORRECTNESS. `Writer` holds a
        // `Mutex<Connection>` in WAL mode; a rename over a file another handle holds open without
        // `FILE_SHARE_DELETE` fails with a sharing violation. `ac427_…` runs here and is red on it.
        drop(writer);
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
        events_written: rebuilt.events.len(),
        head,
        unresolvable: rebuilt.unresolvable,
    })
}
