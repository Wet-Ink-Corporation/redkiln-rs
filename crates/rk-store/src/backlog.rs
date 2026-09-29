//! **The `toolchain:backlog` producer: a `store`-route read of the item records the store names**
//! (`s8f` §8f.7; D21-46 R7; the render record ADR-0189 decision 12, proposed; D21-106). *(Added
//! 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS.)*
//!
//! R7: *"s8f builds the `toolchain:backlog` producer (a store-route read of items)"*. The kernel
//! reaches it through `Runtime::call_feed`'s one `store` arm and its composition-root-bound seam
//! (D21-106); `rk-cli` binds the seam over this module.
//!
//! **What it reads: the item records the store names, and nothing else.** An id is named when a
//! tier-1 stage-transition row carries it as `item` (a `stage_transition` carrying `from`), or when
//! a journalled `ItemRefs` output of an `item_create` step lists it ([`crate::item_write`]). Each
//! named id is read at `.redkiln/items/<id>.toml` in the run-owned worktree **by its exact path**,
//! `doctor --rebuild`'s precedent. The producer **never walks** `.redkiln/` or `.redkiln/items/`, so
//! a file no row names is not in the feed, and D21-44's `worktrees/` exclusion holds by
//! construction.
//!
//! **A named id with no record is listed absent, never dropped.** An item minted on another lane's
//! run ref is absent here until the operator merges that ref.
//!
//! **What it costs, stated.** A journalled `ItemRefs` output is tier 2: after the deleted-database
//! test an item that was minted and never run drops out of the feed until it is run. Tier-1 names
//! survive, refolded from `Story:` trailers by `doctor --rebuild`.
//!
//! An item record is not an artifact kind, so the read appends no `artifact.read`. `rk-store`
//! takes no TOML dependency ([`crate::item_write`]'s header), so the caller hands in the record
//! parser; the read itself — which paths, by exact path, and what a missing file means — is here.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::verb_output::{BacklogAbsent, BacklogItem, BacklogSnapshot, NamedBy};
use serde_json::Value;

use crate::item_write::item_path;

/// One record, as the caller's parser reads it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedRecord {
    /// The record's top-level `id`, when it carries one.
    pub id: Option<String>,
    /// The record's top-level `kind`.
    pub kind: Option<String>,
    /// The record's top-level `stage`.
    pub stage: Option<String>,
    /// The record's `[facets]` table verbatim; an absent facet stays absent.
    pub facets: serde_json::Map<String, Value>,
    /// The record's `[provenance]` table, or `None`.
    pub provenance: Option<Value>,
}

/// Why the read refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BacklogError {
    /// A named id cannot name a file under `.redkiln/items/`.
    UnsafeId(String),
    /// A named record does not parse.
    Unparsable { id: String, detail: String },
    /// A named record's own `id` is not the id that named it.
    IdMismatch { named: String, carried: String },
    /// The filesystem refused (other than a missing file).
    Io { path: String, detail: String },
    /// The snapshot is not canonical JSON.
    Canon(String),
}

impl std::fmt::Display for BacklogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsafeId(id) => write!(
                f,
                "the store names the item `{id}`, which cannot name a file under `.redkiln/items/`"
            ),
            Self::Unparsable { id, detail } => {
                write!(
                    f,
                    "the item record `{}` does not parse: {detail}",
                    item_path(id)
                )
            }
            Self::IdMismatch { named, carried } => write!(
                f,
                "the record at `{}` carries `id = {carried:?}`, not the id the store names",
                item_path(named)
            ),
            Self::Io { path, detail } => write!(f, "`{path}`: {detail}"),
            Self::Canon(d) => write!(f, "the backlog snapshot is not canonical JSON: {d}"),
        }
    }
}

impl std::error::Error for BacklogError {}

/// **The ids the store names**, each with what named it: a tier-1 `stage_transition` (one
/// carrying `from`) by its `item`, and an `item_create` step's journalled `ItemRefs` output by its
/// `ids`.
#[must_use]
pub fn names(events: &[Event]) -> BTreeMap<String, BTreeSet<NamedBy>> {
    let mut out: BTreeMap<String, BTreeSet<NamedBy>> = BTreeMap::new();
    for e in events {
        match e.kind {
            EventKind::StageTransition => {
                let boundary = e.payload.get("from").is_some_and(|f| !f.is_null());
                // The interim key beside the payload (ADR-0182), or the payload's own `item`.
                let item = e
                    .item()
                    .or_else(|| e.payload.get("item").and_then(Value::as_str));
                if let (true, Some(item)) = (boundary, item) {
                    out.entry(item.to_owned())
                        .or_default()
                        .insert(NamedBy::StageTransition);
                }
            }
            EventKind::ActivityCommitted
                if e.payload.get("schema").and_then(Value::as_str) == Some("ItemRefs") =>
            {
                let ids = e
                    .payload
                    .get("result")
                    .and_then(|r| r.get("ids"))
                    .and_then(Value::as_array);
                for id in ids.into_iter().flatten().filter_map(Value::as_str) {
                    out.entry(id.to_owned())
                        .or_default()
                        .insert(NamedBy::ItemWrite);
                }
            }
            _ => {}
        }
    }
    out
}

/// Whether `id` can only name a file directly under `.redkiln/items/`.
fn safe_id(id: &str) -> bool {
    !id.is_empty() && !id.starts_with('.') && !id.contains(['/', '\\', ':']) && !id.contains("..")
}

/// **The snapshot**: each named id read at `worktree/.redkiln/items/<id>.toml` by its exact path.
///
/// # Errors
/// [`BacklogError`], naming the id.
pub fn snapshot(
    worktree: &Path,
    named: &BTreeMap<String, BTreeSet<NamedBy>>,
    parse: &dyn Fn(&[u8]) -> Result<ParsedRecord, String>,
) -> Result<BacklogSnapshot, BacklogError> {
    let mut items = Vec::new();
    let mut absent = Vec::new();
    for (id, by) in named {
        if !safe_id(id) {
            return Err(BacklogError::UnsafeId(id.clone()));
        }
        let named_by: Vec<NamedBy> = by.iter().copied().collect();
        let path = worktree.join(item_path(id));
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                absent.push(BacklogAbsent {
                    id: id.clone(),
                    named_by,
                });
                continue;
            }
            Err(e) => {
                return Err(BacklogError::Io {
                    path: item_path(id),
                    detail: e.to_string(),
                });
            }
        };
        let record = parse(&bytes).map_err(|detail| BacklogError::Unparsable {
            id: id.clone(),
            detail,
        })?;
        if let Some(carried) = record.id.as_ref().filter(|c| *c != id) {
            return Err(BacklogError::IdMismatch {
                named: id.clone(),
                carried: carried.clone(),
            });
        }
        items.push(BacklogItem {
            id: id.clone(),
            kind: record.kind,
            stage: record.stage,
            facets: record.facets,
            provenance: record.provenance,
            record_digest: rk_types::digest::sha256(&bytes).as_str().to_owned(),
            named_by,
        });
    }
    Ok(BacklogSnapshot { items, absent })
}

/// The snapshot as the value the verb outputs: its canonical bytes, parsed back, so the value's
/// member order is the canonical one.
///
/// # Errors
/// [`BacklogError::Canon`].
pub fn to_value(snapshot: &BacklogSnapshot) -> Result<Value, BacklogError> {
    let bytes = canonical_bytes(snapshot)?;
    serde_json::from_slice(&bytes).map_err(|e| BacklogError::Canon(e.to_string()))
}

/// The snapshot's `rk_store::canon::canonical` bytes.
///
/// # Errors
/// [`BacklogError::Canon`].
pub fn canonical_bytes(snapshot: &BacklogSnapshot) -> Result<Vec<u8>, BacklogError> {
    let value = serde_json::to_value(snapshot).map_err(|e| BacklogError::Canon(e.to_string()))?;
    crate::canon::canonical(&value).map_err(|e| BacklogError::Canon(format!("{e:?}")))
}
