//! **AC-127's vouched-root half** (`s8f` §8f.5; the render record ADR-0189 decision 10, proposed;
//! D21-46 R7). *(Added 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED.)*
//!
//! AC-127: *"GIVEN a human-supplied (vouched-root) artifact whose content has changed since it was
//! vouched / WHEN the store reconciles it / THEN the change is recorded as advisory drift and does
//! not block the stage / AND the same change on a store-rendered structured section does block
//! until resolved."* The second clause is C2's ([`crate::render::reconcile_entry`] and the kernel's
//! `render.structured-drift` gate). This module is the first.
//!
//! Two things happen here, in the same entry pass as C2 and after it:
//!
//! - **The store.** `answer` does not land (ADR-0188 decision 14), so an attached root is stored at
//!   the next `run`'s entry: the bytes at the attach row's `attached_path` are read, their digest
//!   checked against the attach row's, and written as `<Kind>.root<ext>` in the item's managed
//!   directory through the writer, with one `artifact.write`. A missing file or a mismatch is
//!   refused by name ([`RenderError::VouchedUnreadable`], [`RenderError::VouchedMismatch`]) and
//!   nothing is stored. The **vouched digest** is the attach row's, and only a new attach moves it.
//! - **The pass.** For each stored root, the whole file's digest is compared with the vouched
//!   digest (a root is not layout C: it has no section, and the locator never reads it). Changed:
//!   one [`Drift::Advisory`], handed to the run's advise function, whose admission is the run's
//!   `AdvisorySink` (the composition root binds it; `rk-store` depends on no kernel crate); the
//!   admitted row is appended once per `(root, null, new_digest)` per run. **Nothing else
//!   happens**: the root is never restored or rewritten, no gate opens, and no step re-runs.
//!   Unchanged: nothing is appended.
//!
//! Every root is named by the store's own rows (the attach rows and the writer's rows), never by
//! a walk of the managed directory (ADR-0189 decision 6).

use std::path::{Path, PathBuf};

use rk_types::digest::{Digest, sha256};
use rk_types::drift::Drift;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::render::{ArtifactWriter, RenderError, Written};

/// The member an attach row carries its attachment's path in (ADR-0189 decision 8). Not `path`,
/// which on a writer row names a managed destination.
pub const ATTACHED_PATH: &str = "attached_path";

/// One vouched root, as the item's latest attach row for its kind names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attached {
    /// The artifact kind the root was attached as; the advisory's `root`.
    pub kind: String,
    /// The attach row's `attached_path`: relative to the checkout's root in `/` form, or absolute.
    pub attached_path: String,
    /// The attach row's digest: the vouched digest.
    pub vouched: Digest,
    /// The attach row's position in the item's rows.
    pub at: usize,
}

impl Attached {
    /// The root's file name in the managed directory: `<Kind>.root<ext>`, `<ext>` the
    /// attachment's own extension (with its dot), or none.
    #[must_use]
    pub fn name(&self) -> String {
        let ext = Path::new(&self.attached_path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{e}"))
            .unwrap_or_default();
        format!("{}.root{ext}", self.kind)
    }

    /// The attachment's path on this host: `attached_path` itself when absolute, and otherwise
    /// joined to the checkout's root.
    #[must_use]
    pub fn source(&self, checkout: &Path) -> PathBuf {
        let p = Path::new(&self.attached_path);
        if p.is_absolute() || self.attached_path.starts_with('/') {
            p.to_path_buf()
        } else {
            checkout.join(p)
        }
    }
}

fn str_of<'a>(e: &'a Event, key: &str) -> Option<&'a str> {
    e.payload.get(key).and_then(serde_json::Value::as_str)
}

/// **The vouched roots the store names for `item`**: the latest attach row per kind (an
/// `artifact.write` carrying [`ATTACHED_PATH`]), ordered by kind. A row whose digest does not read
/// back is not a root.
#[must_use]
pub fn attached(rows: &[Event], item: &str) -> Vec<Attached> {
    let mut latest: std::collections::BTreeMap<String, Attached> =
        std::collections::BTreeMap::new();
    for (at, e) in rows.iter().enumerate() {
        if e.kind != EventKind::ArtifactWrite || str_of(e, "item") != Some(item) {
            continue;
        }
        let (Some(kind), Some(path), Some(digest)) = (
            str_of(e, "kind"),
            str_of(e, ATTACHED_PATH),
            str_of(e, "digest").and_then(|d| Digest::from_hex(d).ok()),
        ) else {
            continue;
        };
        latest.insert(
            kind.to_owned(),
            Attached {
                kind: kind.to_owned(),
                attached_path: path.to_owned(),
                vouched: digest,
                at,
            },
        );
    }
    latest.into_values().collect()
}

/// **One vouched root as the store holds it**: the stored copy in the item's managed directory.
/// *(Added 2026-10-18, `s8j` build, package S8J-C4-FIX D1; ADR-0202, proposed.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stored {
    /// The artifact kind the root was attached as.
    pub kind: String,
    /// The stored copy's repo-relative path, `<ARTIFACTS_DIR>/<item>/<Kind>.root<ext>`.
    pub path: String,
    /// The vouched digest: what the bytes at `path` hashed to when the pass stored them. A later
    /// edit of the stored copy is advisory drift (AC-127), so the bytes there may no longer hash
    /// to it.
    pub digest: Digest,
}

/// Is `e` the writer row that stored a root at `path` under `digest` for `item`? The one test of
/// "stored" the vouched pass and [`stored`] share.
fn is_stored_copy(e: &Event, item: &str, path: &str, digest: &Digest) -> bool {
    e.kind == EventKind::ArtifactWrite
        && str_of(e, "item") == Some(item)
        && str_of(e, ATTACHED_PATH).is_none()
        && str_of(e, "path") == Some(path)
        && str_of(e, "digest") == Some(digest.as_str())
}

/// **The stored copy of `item`'s vouched root of `kind`** (ADR-0202, proposed).
///
/// It is the latest attach row of the kind ([`attached`]), when a writer row AFTER it stored its bytes at
/// `<Kind>.root<ext>` in the item's managed directory under its vouched digest. `None` when there
/// is no such attach row, or it is not stored yet (the vouched pass has not run since it), so a
/// caller never reads the operator's `attached_path`.
#[must_use]
pub fn stored(rows: &[Event], item: &str, kind: &str) -> Option<Stored> {
    let root = attached(rows, item).into_iter().find(|r| r.kind == kind)?;
    let path = crate::render::managed_path(item, &root.name());
    rows.iter()
        .skip(root.at.saturating_add(1))
        .any(|e| is_stored_copy(e, item, &path, &root.vouched))
        .then_some(Stored {
            kind: root.kind,
            path,
            digest: root.vouched,
        })
}

/// What the run's advise function answers for one own-change advisory: the row to append (the
/// kernel's `Advisory::own_change(..).to_event()`), and whether the run's `AdvisorySink` admitted
/// it as the FIRST of its `(root, null, new_digest)` in this run.
#[derive(Debug, Clone, PartialEq)]
pub struct Admitted {
    pub row: Event,
    pub first_in_run: bool,
}

/// The run's advise function: bound per run by the composition root over the kernel's
/// `AdvisorySink`, so the de-duplication memory lives exactly as long as the run.
pub type Advise<'a> = dyn FnMut(&Drift) -> Result<Admitted, String> + 'a;

/// What one vouched pass did.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VouchedPass {
    stored: Vec<Written>,
    changed: Vec<Drift>,
    advised: usize,
    suppressed: usize,
}

impl VouchedPass {
    /// Every root stored at this entry, one `artifact.write` each.
    #[must_use]
    pub fn stored(&self) -> &[Written] {
        &self.stored
    }

    /// Every changed root's drift, in kind order.
    #[must_use]
    pub fn changed(&self) -> &[Drift] {
        &self.changed
    }

    /// The drifts this pass classes as BLOCKING (`Drift::blocks()`): the kernel opens a
    /// `render.structured-drift` gate over these. A vouched root's change is advisory, so for a
    /// correct pass this is always empty (AC-127).
    #[must_use]
    pub fn blocking(&self) -> Vec<&Drift> {
        self.changed.iter().filter(|d| d.blocks()).collect()
    }

    /// How many advisories were appended (the run's sink admitted them first).
    #[must_use]
    pub const fn advised(&self) -> usize {
        self.advised
    }

    /// How many were suppressed as already appended in this run.
    #[must_use]
    pub const fn suppressed(&self) -> usize {
        self.suppressed
    }
}

/// **The vouched pass** (ADR-0189 decision 10): store every attached root not yet stored under its
/// vouched digest, then compare every stored root's file with its vouched digest. `checkout` is
/// the invoking checkout's root, which a relative `attached_path` is read against.
///
/// # Errors
/// [`RenderError::VouchedUnreadable`] / [`RenderError::VouchedMismatch`] naming the kind, the path
/// and the digests (nothing is stored); the writer's and the journal's own; the advise function's
/// refusal, as [`RenderError::Journal`].
pub fn vouched_pass(
    writer: &ArtifactWriter,
    checkout: &Path,
    advise: &mut Advise<'_>,
) -> Result<VouchedPass, RenderError> {
    let rows = writer.rows()?;
    let item = writer.stage_run().item.clone();
    let mut done = VouchedPass::default();
    for root in attached(&rows, &item) {
        let name = root.name();
        let path = writer.dir().rel(&name);
        // Stored = a writer row naming the root's managed path under the vouched digest, after
        // the attach row. A newer attach (a re-vouch) is stored again.
        let is_stored = rows
            .iter()
            .skip(root.at + 1)
            .any(|e| is_stored_copy(e, &item, &path, &root.vouched));
        if !is_stored {
            done.stored.push(store(writer, &root, &name, checkout)?);
            continue;
        }
        // THE PASS: the whole file is the instrument (a root is not layout C). An absent file
        // reads as zero bytes, which is a change like any other.
        let observed = sha256(&writer.dir().read(&name).unwrap_or_default());
        if observed == root.vouched {
            continue;
        }
        let drift = Drift::Advisory {
            root: root.kind.clone(),
            vouched: root.vouched.clone(),
            observed,
        };
        if !drift.blocks() && !writer.is_replaying() {
            let admitted = advise(&drift).map_err(|detail| RenderError::Journal { detail })?;
            if admitted.first_in_run {
                writer.append(admitted.row)?;
                done.advised += 1;
            } else {
                done.suppressed += 1;
            }
        }
        done.changed.push(drift);
    }
    Ok(done)
}

/// **The store** of one attached root: the bytes at `attached_path`, checked against the vouched
/// digest, written through the writer's single write path. A missing file or a mismatch is
/// refused by name and nothing is stored.
fn store(
    writer: &ArtifactWriter,
    root: &Attached,
    name: &str,
    checkout: &Path,
) -> Result<Written, RenderError> {
    let bytes =
        std::fs::read(root.source(checkout)).map_err(|e| RenderError::VouchedUnreadable {
            kind: root.kind.clone(),
            path: root.attached_path.clone(),
            vouched: root.vouched.as_str().to_owned(),
            detail: e.to_string(),
        })?;
    let observed = sha256(&bytes);
    if observed != root.vouched {
        return Err(RenderError::VouchedMismatch {
            kind: root.kind.clone(),
            path: root.attached_path.clone(),
            vouched: root.vouched.as_str().to_owned(),
            observed: observed.as_str().to_owned(),
        });
    }
    writer.write_root(&root.kind, name, &bytes)
}
