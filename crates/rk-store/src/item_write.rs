//! **The in-repo item write: `item_create`'s `store`-route handler** (`s8f` §8f.7; the render
//! record ADR-0189 decision 11, proposed; D21-42 6a; D21-106). *(Added 2026-09-28, s8f build,
//! package S8F-P6-ITEMS-BACKLOG-FEEDS.)*
//!
//! D21-42 6a: *"DebtRegisterEntry is an rk-store item write with provenance; human-sourced facets
//! are null and go to `unknown_facet`."* The kernel serves `item_create`'s registry row into this
//! function through its composition-root-bound store-route seam, **never** a `Toolchain` adapter:
//! an adapter that wrote `.redkiln/items/` would be a second write path.
//!
//! Each extracted entry becomes `.redkiln/items/<id>.toml` in the run-owned worktree, in
//! `ItemRecord`'s key set (`id`, `kind`, `stage = "triage"`, `[facets]`) plus a `[provenance]`
//! table naming the producing activity at its version, the run, the source `ImplReport` digest,
//! the digest of the extracted entry and every withheld facet.
//!
//! - **Facets.** A facet is written only when the registry declares it with `source =
//!   "declared"`. A `human` or `llm` facet is **never written**, even when the extracted value
//!   carries one (golden constraint 1; DECISIONS-2026-09-12 #15), and a `derived` or `kernel` facet
//!   is not the record's to carry (`run`'s `build_item` refuses the first and never reads the
//!   second). Every withheld facet is **named** in `[provenance] withheld`, with its source, never
//!   dropped silently. `run`'s `build_item` nulls a declared facet the record omits, so a minted
//!   item, when run, abstains and opens an `unknown_facet` gate.
//! - **The kind.** `item_create` mints tech-debt items (D21-42 6a's `DebtRegisterEntry`), so the
//!   record's `kind` is the handler's own [`MINTED_KIND`]. An extracted `kind` is a `human` facet
//!   and is withheld and named like any other.
//! - **An existing id.** A file byte-identical to the rendering is **served**: nothing is written.
//!   A file that differs is **refused by name, never overwritten**, and nothing of the call is
//!   written (§8f.7: *"An existing id — found in revision"*; the `on_false` loop-back is the case).
//! - **No artifact kind is minted.** An item record is not an artifact kind, so its write appends
//!   no `artifact.write` (ADR-0189 decision 11).
//!
//! The rendering is this module's own, and it is not a TOML library's: `rk-store` takes no TOML
//! dependency (it would move `rk-kernel`'s resolved graph, a dependency decision `00-preamble.md`
//! reserves). The subset written is closed: basic strings, booleans, integers, arrays and inline
//! tables of those, every key bare, LF only. A value outside it (a float, a `null` inside an
//! array) is refused by name.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

/// The `kind` every minted item takes: `item_create` mints tech-debt items (D21-42 6a).
pub const MINTED_KIND: &str = "tech-debt";

/// The `stage` every minted item enters at (§8f.7).
pub const MINTED_STAGE: &str = "triage";

/// The managed directory, relative to the run-owned worktree's root (ADR-0189 decision 6).
pub const ITEMS_DIR: &str = ".redkiln/items";

/// What the producing step stamps into `[provenance]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    /// The producing activity at its version (`debt-logging@1`).
    pub activity: String,
    /// The stage run.
    pub run: String,
    /// The digest of the source `ImplReport` the extraction read.
    pub source_impl_report: String,
    /// The item the entries were extracted from; a minted id without its own is
    /// `<source_item>-debt-<n>`.
    pub source_item: String,
}

/// What one call did, id by id.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemWrite {
    /// Every id the call names, in entry order: the `ItemRefs` value.
    pub ids: Vec<String>,
    /// The ids whose file this call wrote.
    pub written: Vec<String>,
    /// The ids whose byte-identical file was already there, and was served.
    pub served: Vec<String>,
}

/// Why the write refused. Nothing is written when any entry refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemWriteError {
    /// The `items` input is not an array of objects.
    Shape(String),
    /// An id is not `[A-Za-z0-9][A-Za-z0-9._-]*` without `..`.
    BadId(String),
    /// Two entries of one call name the same id.
    DuplicateId(String),
    /// A value is outside the written subset.
    Unrenderable { id: String, detail: String },
    /// A differing file already holds the id: refused, never overwritten.
    Exists { id: String, path: String },
    /// The filesystem refused.
    Io { path: String, detail: String },
}

impl std::fmt::Display for ItemWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shape(d) => write!(f, "the `items` input is not a list of item entries: {d}"),
            Self::BadId(id) => write!(
                f,
                "the item id `{id}` is not `[A-Za-z0-9][A-Za-z0-9._-]*` without `..`, so it names \
                 no file under `{ITEMS_DIR}/`"
            ),
            Self::DuplicateId(id) => write!(f, "two entries of one call name the item id `{id}`"),
            Self::Unrenderable { id, detail } => {
                write!(
                    f,
                    "the item `{id}` carries a value the record cannot hold: {detail}"
                )
            }
            Self::Exists { id, path } => write!(
                f,
                "the item `{id}` already exists at `{path}` with different bytes; an existing item \
                 is refused by name and never overwritten (ADR-0189 decision 11)"
            ),
            Self::Io { path, detail } => write!(f, "`{path}`: {detail}"),
        }
    }
}

impl std::error::Error for ItemWriteError {}

/// `path`, relative to the worktree, of item `id`'s record.
#[must_use]
pub fn item_path(id: &str) -> String {
    format!("{ITEMS_DIR}/{id}.toml")
}

/// Whether `id` is a record id this module writes: `[A-Za-z0-9][A-Za-z0-9._-]*` with no `..`, so
/// it can only name a file directly under `.redkiln/items/`.
#[must_use]
pub fn valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphanumeric())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && !id.contains("..")
}

/// **The source digest**: the `rk_store::canon` digest of the latest `emit` binding of `kind` in
/// `events` (an `activity_committed` row's `artifacts.<kind>`), the value an `artifact:<kind>` input
/// reads and whose canonical bytes the writer lands. `None` when no row binds it.
#[must_use]
pub fn emitted_digest(events: &[rk_types::event::Event], kind: &str) -> Option<String> {
    events
        .iter()
        .rev()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
        .find_map(|e| e.payload.get("artifacts").and_then(|a| a.get(kind)))
        .and_then(|v| crate::canon::canonical_digest(v).ok())
        .map(|d| d.as_str().to_owned())
}

/// **Write each extracted entry as an item record** under `worktree/.redkiln/items/`.
///
/// `items` is the step's resolved `items` input: a list of objects, each with an optional `id`
/// (else `<source_item>-debt-<n>`, 1-based in entry order), an optional `facets` object and any
/// other members. `facet_sources` maps each registry facet to its `source` token.
///
/// # Errors
/// [`ItemWriteError`]; when any entry refuses, nothing is written.
pub fn write_items(
    worktree: &Path,
    items: &Value,
    facet_sources: &BTreeMap<String, String>,
    provenance: &Provenance,
) -> Result<ItemWrite, ItemWriteError> {
    let entries = items
        .as_array()
        .ok_or_else(|| ItemWriteError::Shape(format!("{items}")))?;
    let mut planned: Vec<(String, PathBuf, Vec<u8>)> = Vec::new();
    let mut seen = BTreeSet::new();
    for (n, entry) in entries.iter().enumerate() {
        let obj = entry
            .as_object()
            .ok_or_else(|| ItemWriteError::Shape(format!("entry {} is {entry}", n + 1)))?;
        let id = match obj.get("id") {
            Some(Value::String(id)) => id.clone(),
            Some(other) => return Err(ItemWriteError::BadId(other.to_string())),
            None => format!("{}-debt-{}", provenance.source_item, n + 1),
        };
        if !valid_id(&id) {
            return Err(ItemWriteError::BadId(id));
        }
        if !seen.insert(id.clone()) {
            return Err(ItemWriteError::DuplicateId(id));
        }
        let bytes = render_record(&id, obj, facet_sources, provenance)?;
        planned.push((id.clone(), worktree.join(item_path(&id)), bytes));
    }
    // Every existing file is compared BEFORE anything is written, so a refusal writes nothing.
    let mut out = ItemWrite::default();
    let mut to_write = Vec::new();
    for (id, path, bytes) in planned {
        out.ids.push(id.clone());
        match std::fs::read(&path) {
            Ok(existing) if existing == bytes => out.served.push(id),
            Ok(_) => {
                return Err(ItemWriteError::Exists {
                    path: item_path(&id),
                    id,
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => to_write.push((id, path, bytes)),
            Err(e) => {
                return Err(ItemWriteError::Io {
                    path: path.display().to_string(),
                    detail: e.to_string(),
                });
            }
        }
    }
    for (id, path, bytes) in to_write {
        let io = |e: std::io::Error| ItemWriteError::Io {
            path: path.display().to_string(),
            detail: e.to_string(),
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io)?;
        }
        std::fs::write(&path, &bytes).map_err(io)?;
        out.written.push(id);
    }
    Ok(out)
}

/// The record's bytes: the key set, `[facets]` (the `declared` facets only) and `[provenance]`.
fn render_record(
    id: &str,
    entry: &serde_json::Map<String, Value>,
    facet_sources: &BTreeMap<String, String>,
    provenance: &Provenance,
) -> Result<Vec<u8>, ItemWriteError> {
    let unrenderable = |detail: String| ItemWriteError::Unrenderable {
        id: id.to_owned(),
        detail,
    };
    // The candidate facets: the entry's `facets` object, and an extracted top-level `kind` (a
    // `human` facet, withheld like the rest).
    let mut candidates: BTreeMap<String, Value> = BTreeMap::new();
    match entry.get("facets") {
        None | Some(Value::Null) => {}
        Some(Value::Object(map)) => {
            for (k, v) in map {
                candidates.insert(k.clone(), v.clone());
            }
        }
        Some(other) => return Err(unrenderable(format!("`facets` is {other}, not a table"))),
    }
    if let Some(kind) = entry.get("kind") {
        candidates.insert("kind".to_owned(), kind.clone());
    }
    let mut written = BTreeMap::new();
    let mut withheld = Vec::new();
    for (name, value) in candidates {
        let source = facet_sources
            .get(&name)
            .map_or("undeclared", String::as_str);
        if source == "declared" && name != "kind" && !value.is_null() {
            if !bare_key(&name) {
                return Err(unrenderable(format!(
                    "the facet name `{name}` is not a bare key"
                )));
            }
            written.insert(name, inline(&value).map_err(unrenderable)?);
        } else {
            withheld.push((name, source.to_owned()));
        }
    }
    let entry_digest = crate::canon::canonical_digest(&Value::Object(entry.clone()))
        .map_err(|e| unrenderable(format!("the extracted entry is not canonical: {e:?}")))?;

    let mut s = String::new();
    s.push_str(&format!("id = {}\n", basic(id)));
    s.push_str(&format!("kind = {}\n", basic(MINTED_KIND)));
    s.push_str(&format!("stage = {}\n", basic(MINTED_STAGE)));
    s.push_str("\n[facets]\n");
    for (name, value) in &written {
        s.push_str(&format!("{name} = {value}\n"));
    }
    s.push_str("\n[provenance]\n");
    s.push_str(&format!("activity = {}\n", basic(&provenance.activity)));
    s.push_str(&format!("entry = {}\n", basic(entry_digest.as_str())));
    s.push_str(&format!("run = {}\n", basic(&provenance.run)));
    s.push_str(&format!(
        "source_impl_report = {}\n",
        basic(&provenance.source_impl_report)
    ));
    let withheld = withheld
        .iter()
        .map(|(facet, source)| {
            format!("{{ facet = {}, source = {} }}", basic(facet), basic(source))
        })
        .collect::<Vec<_>>()
        .join(", ");
    s.push_str(&format!("withheld = [{withheld}]\n"));
    Ok(s.into_bytes())
}

/// Whether `k` is a bare TOML key.
fn bare_key(k: &str) -> bool {
    !k.is_empty()
        && k.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// A TOML basic string.
fn basic(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if u32::from(c) < 0x20 || c == '\u{7f}' => {
                out.push_str(&format!("\\u{:04X}", u32::from(c)));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// One value, inline, in the written subset.
fn inline(v: &Value) -> Result<String, String> {
    match v {
        Value::String(s) => Ok(basic(s)),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) if n.is_i64() || n.is_u64() => Ok(n.to_string()),
        Value::Number(n) => Err(format!("the number {n} is not an integer")),
        Value::Array(items) => items
            .iter()
            .map(inline)
            .collect::<Result<Vec<_>, _>>()
            .map(|parts| format!("[{}]", parts.join(", "))),
        // Keys in byte order: `serde_json` preserves insertion order workspace-wide, and a rendering
        // that followed it would not be a function of the value.
        Value::Object(map) => map
            .iter()
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .map(|(k, v)| {
                if bare_key(k) {
                    inline(v).map(|v| format!("{k} = {v}"))
                } else {
                    Err(format!("the key `{k}` is not a bare key"))
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|parts| format!("{{ {} }}", parts.join(", "))),
        Value::Null => Err("a `null` has no TOML form".to_owned()),
    }
}
