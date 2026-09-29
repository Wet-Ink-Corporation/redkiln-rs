//! **`rk-store`'s render, write and read path** (`s8f` §8f.5 and §8f.6; the render record ADR-0189,
//! proposed; D21-42 5a and 5b; D21-43 N11 and N12; D21-47). *(Added 2026-09-28, s8f build, package
//! S8F-P4-RENDER-WRITE-READ.)*
//!
//! Five things live here, and each is the ONE of its kind:
//!
//! - **The renderer** ([`render`], [`section_slice`], [`frontmatter_block`]): a human-facing kind is
//!   layout C (ADR-0006, as refined by ADR-0075) — typed frontmatter, digest-stamped structured
//!   sections and opaque prose sections — and each piece is a pure function of *(typed value,
//!   template version)* (AC-121, AC-123). A machine-only kind is its canonical JSON sidecar alone,
//!   rendered to `.md` on demand ([`render_on_demand`]) and never stored (AC-144).
//! - **The writer** ([`ArtifactWriter`]): the single write path into an item's managed directory,
//!   `.redkiln/artifacts/<item>/` in the run-owned worktree (AC-122; decision 6). A struct, never a
//!   trait. It appends exactly one `artifact.write` per write, carrying `kind`, `item`, `stage`,
//!   `run`, `schema_version`, `digest` and `path` (decision 8), and on replay it performs zero
//!   writes and serves the recorded digest (decision 7). No filesystem-write handle into a managed
//!   directory is exported: [`ManagedDir`]'s write is private to this module.
//! - **The entry pass** ([`reconcile_entry`]): C2's surface-and-restore half (decision 4). A
//!   structured section or frontmatter that differs from its rendering appends one
//!   `structured_drift`, and THEN the file is restored from the type, a write with its one
//!   `artifact.write`. A prose section edited on disk is accepted whole as the field's new value,
//!   with no drift record (AC-152). The drift GATE is the kernel's, opened over what this pass
//!   returns ([`Reconciled::drifts`]).
//! - **`move-to-prose`** ([`move_to_prose`]): the drift gate's second answer. The drifted bytes
//!   are read from the causing commit the event names, never from the event (which carries digests
//!   only), and are appended to the named prose section through the writer (decision 5).
//! - **The reader** ([`ArtifactReader`]): the ONLY emitter of `artifact.read` (D21-47; decision
//!   9). It serves a kind's value from its sidecar on the run ref, checks the digest against the
//!   committed `emit` binding's (a mismatch is a halt), appends one `artifact.read` carrying
//!   `kind`, `item`, `stage`, `run`, `schema_version`, `tree` and `digest`, and refuses a read
//!   when the current entry has no reconcile record (AC-126).
//!
//! **Nothing here parses structure out of markdown** (AC-120). Every byte range of a rendering is
//! [`crate::locator`]'s answer; no typed field is populated from inside one. The one deserializer
//! call in this module reads a sidecar ([`value_of_sidecar`]), never a rendering and never drift
//! text (AC-125).
//!
//! **What this module does not touch.** Git: `rk-store` does not depend on `rk-git`, so the causing
//! commit, the run ref's bytes and its tree are READ by the composition root and handed in as
//! functions. The journal: `rk-store` does not depend on `rk-ports`, so the event store is handed in
//! as a [`Journal`] of two functions (append, and the item's rows in append order).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value as Json;

use rk_types::digest::{Digest, sha256};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::canon::{self, CanonError};
use crate::locator::{self, Anchor};

/// The managed tree an item's rendered artifacts live under, repo-relative inside the run-owned
/// worktree (decision 6): `.redkiln/artifacts/<item>/`.
pub const ARTIFACTS_DIR: &str = ".redkiln/artifacts";

/// The heading level every layout-C section anchor is declared at.
pub const SECTION_LEVEL: usize = 2;

/// The anchor a `structured_drift` names for the frontmatter (decision 4).
pub const FRONTMATTER_ANCHOR: &str = "frontmatter";

/// The anchor a `structured_drift` names when the rendering is absent or unreadable as a whole.
pub const DOCUMENT_ANCHOR: &str = "document";

/// **A human-facing kind's template.** Its version is the stamp's `<N>` (decision 2), which is what
/// makes AC-128's template-drift kind buildable later; its frontmatter and prose fields are
/// declared by name, and every other top-level field of the value is a structured section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Template {
    pub kind: &'static str,
    pub version: u32,
    /// Top-level fields rendered as typed frontmatter, in this order.
    pub frontmatter: &'static [&'static str],
    /// Top-level fields that are opaque prose sections (AC-152).
    pub prose: &'static [&'static str],
}

/// **The human-facing template set.** *(dated 2026-09-28, s8f build, package
/// S8F-P4-RENDER-WRITE-READ; a reading ADR-0189 does not settle, stated in `s8f` §8f.5's dated
/// note.)* ADR-0006 names `spec.md`, `plan.md` and `story.md` as the human-facing artifacts;
/// ADR-0075 decision 4 removes `plan.md`, and `.redkiln/artifacts.toml` registers no `Story` kind,
/// so `Spec` is the one human-facing kind. Every other registered kind is machine-only (ADR-0006:
/// *"`_review`, `_ledger`, digests, run reports"*).
pub const TEMPLATES: &[Template] = &[Template {
    kind: "Spec",
    version: 1,
    frontmatter: &["id", "status", "title"],
    prose: &["context", "notes"],
}];

/// The template of a human-facing `kind`, or `None` for a machine-only one.
#[must_use]
pub fn template_of(kind: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.kind == kind)
}

/// Why the render, write or read path refused. Every variant names what it refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// A human-facing value that is not a JSON object.
    NotAnObject { kind: String },
    /// A prose field whose value is not text.
    ProseNotText { kind: String, field: String },
    /// A float reached the canonical serialiser.
    Canon { kind: String, detail: String },
    /// A filesystem operation inside the managed directory failed.
    Io { path: String, detail: String },
    /// The journal refused an append or a read.
    Journal { detail: String },
    /// Replay found no recorded row for this write or read.
    ReplayUnrecorded { kind: String, path: String },
    /// Replay found a recorded row whose digest is not the one this call derives.
    ReplayDiverged {
        kind: String,
        recorded: String,
        derived: String,
    },
    /// `move-to-prose` named a kind with no template, a section that is not prose, or a drift row
    /// that does not carry what it needs.
    MoveToProse { detail: String },
    /// The causing commit's bytes could not be read, or the drifted section is not in them.
    CausingCommit { detail: String },
    /// **AC-126**: the read was asked for when the current entry has no reconcile record.
    NotReconciled {
        kind: String,
        item: String,
        bound_entry: usize,
        journal_entries: usize,
    },
    /// The sidecar is not on the run ref, or is not JSON.
    Sidecar {
        kind: String,
        path: String,
        detail: String,
    },
    /// **Halt 4**: the bytes served are not the committed `emit` binding's.
    DigestMismatch {
        kind: String,
        served: String,
        bound: String,
    },
    /// **A vouched root that cannot be stored** (ADR-0189 decision 10): the attached file is
    /// absent or unreadable at its recorded `attached_path`. Nothing is stored. *(Added
    /// 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED.)*
    VouchedUnreadable {
        kind: String,
        path: String,
        vouched: String,
        detail: String,
    },
    /// **A vouched root whose bytes are not the ones vouched** (ADR-0189 decision 10): the digest
    /// of the file at `attached_path` differs from the attach row's. Nothing is stored.
    VouchedMismatch {
        kind: String,
        path: String,
        vouched: String,
        observed: String,
    },
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAnObject { kind } => write!(
                f,
                "the `{kind}` value is not an object, so layout C has no sections to render"
            ),
            Self::ProseNotText { kind, field } => write!(
                f,
                "the `{kind}` prose field `{field}` is not text; a prose section is opaque text \
                 (AC-152)"
            ),
            Self::Canon { kind, detail } => write!(f, "the `{kind}` value: {detail}"),
            Self::Io { path, detail } => write!(f, "{path}: {detail}"),
            Self::Journal { detail } => write!(f, "the journal refused: {detail}"),
            Self::ReplayUnrecorded { kind, path } => write!(
                f,
                "replay found no recorded row for `{kind}` at `{path}`; a replay performs no write \
                 and serves only what was recorded"
            ),
            Self::ReplayDiverged {
                kind,
                recorded,
                derived,
            } => write!(
                f,
                "replay of `{kind}` diverged: the journal recorded digest {recorded} and this call \
                 derives {derived}"
            ),
            Self::MoveToProse { detail } => write!(f, "`move-to-prose` refused: {detail}"),
            Self::CausingCommit { detail } => {
                write!(f, "the causing commit could not be read: {detail}")
            }
            Self::NotReconciled {
                kind,
                item,
                bound_entry,
                journal_entries,
            } => write!(
                f,
                "a read of `{kind}` for `{item}` was refused: the current entry has no reconcile \
                 record (the reader is bound to entry {bound_entry}, the journal holds \
                 {journal_entries}); `reconcile()` runs before any read is served, and nothing \
                 skips it (AC-126)"
            ),
            Self::Sidecar { kind, path, detail } => {
                write!(f, "the `{kind}` sidecar `{path}`: {detail}")
            }
            Self::DigestMismatch {
                kind,
                served,
                bound,
            } => write!(
                f,
                "HALT: the `{kind}` bytes on the run ref digest to {served}, but the committed \
                 `emit` binding digests to {bound}; neither is chosen silently (Halt 4)"
            ),
            Self::VouchedUnreadable {
                kind,
                path,
                vouched,
                detail,
            } => write!(
                f,
                "the vouched root `{kind}` was not stored: its attached file `{path}` (vouched \
                 digest {vouched}) could not be read: {detail}; nothing is stored (AC-127, ADR-0129 \
                 decision 4)"
            ),
            Self::VouchedMismatch {
                kind,
                path,
                vouched,
                observed,
            } => write!(
                f,
                "the vouched root `{kind}` was not stored: its attached file `{path}` digests to \
                 {observed}, but the attach vouched {vouched}; nothing is stored, and only a new \
                 attach through `redkiln answer` moves the vouched digest (AC-127)"
            ),
        }
    }
}

impl std::error::Error for RenderError {}

fn canon_err(kind: &str, e: &CanonError) -> RenderError {
    RenderError::Canon {
        kind: kind.to_owned(),
        detail: e.to_string(),
    }
}

// =================================================================================================
// The renderer — a pure function of (typed value, template version).
// =================================================================================================

fn object<'v>(
    template: &Template,
    value: &'v Json,
) -> Result<&'v serde_json::Map<String, Json>, RenderError> {
    value.as_object().ok_or_else(|| RenderError::NotAnObject {
        kind: template.kind.to_owned(),
    })
}

/// `<!-- rk-stamp template=<kind>@<N> sha256=<64 hex> -->` over `body` (decision 2).
fn html_stamp(template: &Template, body: &[u8]) -> String {
    format!(
        "<!-- rk-stamp template={}@{} sha256={} -->\n",
        template.kind,
        template.version,
        sha256(body).as_str()
    )
}

/// `# rk-stamp template=<kind>@<N> sha256=<64 hex>` over `body` (decision 2).
fn yaml_stamp(template: &Template, body: &[u8]) -> String {
    format!(
        "# rk-stamp template={}@{} sha256={}\n",
        template.kind,
        template.version,
        sha256(body).as_str()
    )
}

/// Opaque prose, as its section carries it: LF only, one trailing blank line. The CONTENT is never
/// inspected (AC-152): only line endings and the trailing newlines are normalised, so two
/// renderings of one value are byte-identical on every host (AC-121).
fn prose_normal(text: &str) -> String {
    let lf = text.replace("\r\n", "\n").replace('\r', "\n");
    let body = lf.trim_end_matches('\n');
    if body.is_empty() {
        "\n".to_owned()
    } else {
        format!("{body}\n\n")
    }
}

fn prose_of(template: &Template, field: &str, value: Option<&Json>) -> Result<String, RenderError> {
    match value {
        None | Some(Json::Null) => Ok(String::new()),
        Some(Json::String(s)) => Ok(s.clone()),
        Some(_) => Err(RenderError::ProseNotText {
            kind: template.kind.to_owned(),
            field: field.to_owned(),
        }),
    }
}

/// The frontmatter's inner bytes, as [`crate::locator::frontmatter`] returns their range: `kind`,
/// each declared frontmatter field the value carries, then the stamp line over those bytes.
///
/// # Errors
/// [`RenderError`] for a non-object value or a float.
pub fn frontmatter_block(template: &Template, value: &Json) -> Result<Vec<u8>, RenderError> {
    let map = object(template, value)?;
    let mut body = format!("kind: \"{}\"\n", template.kind).into_bytes();
    for field in template.frontmatter {
        if let Some(v) = map.get(*field) {
            body.extend_from_slice(field.as_bytes());
            body.extend_from_slice(b": ");
            body.extend(canon::canonical(v).map_err(|e| canon_err(template.kind, &e))?);
            body.push(b'\n');
        }
    }
    let stamp = yaml_stamp(template, &body);
    body.extend_from_slice(stamp.as_bytes());
    Ok(body)
}

/// The section keys of a value, in byte-wise order: every top-level field that is not frontmatter,
/// plus every declared prose field (an absent prose field is an empty section a human may fill).
fn section_keys(template: &Template, value: &Json) -> Result<Vec<String>, RenderError> {
    let map = object(template, value)?;
    let mut keys: BTreeSet<String> = map
        .keys()
        .filter(|k| !template.frontmatter.contains(&k.as_str()))
        .cloned()
        .collect();
    keys.extend(template.prose.iter().map(|p| (*p).to_owned()));
    Ok(keys.into_iter().collect())
}

fn is_prose(template: &Template, key: &str) -> bool {
    template.prose.contains(&key)
}

/// **One section's slice** — the bytes the locator returns under `## <key>`. A structured section
/// is its stamp line then a fenced block of the field's canonical JSON; a prose section is its
/// text. A pure function of the value and the template: no prior content is an argument.
///
/// # Errors
/// [`RenderError`] for a non-object value, a non-text prose field or a float.
pub fn section_slice(template: &Template, key: &str, value: &Json) -> Result<Vec<u8>, RenderError> {
    let map = object(template, value)?;
    if is_prose(template, key) {
        return Ok(prose_normal(&prose_of(template, key, map.get(key))?).into_bytes());
    }
    let field = map.get(key).unwrap_or(&Json::Null);
    let mut rest = b"```json\n".to_vec();
    rest.extend(canon::canonical(field).map_err(|e| canon_err(template.kind, &e))?);
    rest.extend_from_slice(b"\n```\n\n");
    let mut out = html_stamp(template, &rest).into_bytes();
    out.extend(rest);
    Ok(out)
}

/// **Layout C** (ADR-0006; decision 1): `---`, the frontmatter block, `---`, a blank line, then one
/// `## <key>` section per key in byte-wise order. LF only.
///
/// # Errors
/// As [`section_slice`].
pub fn render(template: &Template, value: &Json) -> Result<Vec<u8>, RenderError> {
    let mut out = b"---\n".to_vec();
    out.extend(frontmatter_block(template, value)?);
    out.extend_from_slice(b"---\n\n");
    for key in section_keys(template, value)? {
        out.extend_from_slice(format!("## {key}\n").as_bytes());
        out.extend(section_slice(template, &key, value)?);
    }
    Ok(out)
}

/// **A machine-only kind's `.md`, on demand** (AC-144): a heading and the canonical JSON, never
/// stored by the store and never read back.
///
/// # Errors
/// [`RenderError::Canon`] for a float.
pub fn render_on_demand(kind: &str, value: &Json) -> Result<Vec<u8>, RenderError> {
    let mut out = format!("# {kind}\n\n```json\n").into_bytes();
    out.extend(canon::canonical(value).map_err(|e| canon_err(kind, &e))?);
    out.extend_from_slice(b"\n```\n");
    Ok(out)
}

/// The sidecar's file name, `<Kind>.json` (decision 3).
#[must_use]
pub fn sidecar_name(kind: &str) -> String {
    format!("{kind}.json")
}

/// The rendering's file name, `<Kind>.md` (decision 3).
#[must_use]
pub fn rendering_name(kind: &str) -> String {
    format!("{kind}.md")
}

/// `.redkiln/artifacts/<item>/<name>`, the repo-relative path inside the run-owned worktree.
#[must_use]
pub fn managed_path(item: &str, name: &str) -> String {
    format!("{ARTIFACTS_DIR}/{item}/{name}")
}

/// **The one deserializer call in this module**: a sidecar's bytes, into its value. Never called on
/// a rendering or on drift text (AC-125's architecture test reads this module's code lines).
fn value_of_sidecar(sidecar: &[u8]) -> Result<Json, String> {
    serde_json::from_slice(sidecar).map_err(|e| e.to_string())
}

// =================================================================================================
// The managed directory, the journal, the stage run.
// =================================================================================================

/// **An item's managed directory in the run-owned worktree** (decision 6). Reads are public; the
/// write is private to this module, so no sibling crate holds a write handle into it (AC-122).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedDir {
    tree: PathBuf,
    item: String,
}

impl ManagedDir {
    /// The directory of `item` inside the run-owned worktree rooted at `tree`.
    #[must_use]
    pub fn of(tree: &Path, item: &str) -> Self {
        Self {
            tree: tree.to_path_buf(),
            item: item.to_owned(),
        }
    }

    /// `name`'s repo-relative path.
    #[must_use]
    pub fn rel(&self, name: &str) -> String {
        managed_path(&self.item, name)
    }

    /// `name`'s absolute path.
    #[must_use]
    pub fn abs(&self, name: &str) -> PathBuf {
        self.tree.join(self.rel(name))
    }

    /// `name`'s bytes, or `None` when the file is absent.
    #[must_use]
    pub fn read(&self, name: &str) -> Option<Vec<u8>> {
        std::fs::read(self.abs(name)).ok()
    }

    /// **The one filesystem write into a managed directory**: a temporary file beside the target,
    /// then a rename over it. Private: only this module's writer reaches it.
    fn write(&self, name: &str, bytes: &[u8]) -> Result<(), RenderError> {
        let path = self.abs(name);
        let io = |e: std::io::Error| RenderError::Io {
            path: self.rel(name),
            detail: e.to_string(),
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io)?;
        }
        let tmp = self.abs(&format!(".{name}.rk-tmp"));
        std::fs::write(&tmp, bytes).map_err(io)?;
        std::fs::rename(&tmp, &path).map_err(io)
    }
}

type AppendFn = dyn Fn(Event) -> Result<(), String> + Send + Sync;

/// The composition root's read of git for the entry pass: the last commit on `rk-run/<item>` that
/// touched a repo-relative path, or `None`.
pub type CausingCommit = dyn Fn(&str) -> Result<Option<String>, String>;

/// The composition root's read of git for `move-to-prose`: `(commit, path)` to the path's bytes at
/// that commit.
pub type ReadAt = dyn Fn(&str, &str) -> Result<Vec<u8>, String>;
type RowsFn = dyn Fn() -> Result<Vec<Event>, String> + Send + Sync;

/// **The event store, as this crate is handed it**: append one row, and read the item's rows in
/// append order. Two functions, not a trait: `rk-store` depends on no port crate, and a new trait
/// would move no boundary this crate owns.
pub struct Journal {
    append: Box<AppendFn>,
    rows: Box<RowsFn>,
}

impl Journal {
    /// A journal over `append` and `rows`.
    pub fn new(
        append: impl Fn(Event) -> Result<(), String> + Send + Sync + 'static,
        rows: impl Fn() -> Result<Vec<Event>, String> + Send + Sync + 'static,
    ) -> Self {
        Self {
            append: Box::new(append),
            rows: Box::new(rows),
        }
    }

    fn append(&self, event: Event) -> Result<(), RenderError> {
        (self.append)(event).map_err(|detail| RenderError::Journal { detail })
    }

    fn rows(&self) -> Result<Vec<Event>, RenderError> {
        (self.rows)().map_err(|detail| RenderError::Journal { detail })
    }
}

impl std::fmt::Debug for Journal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Journal")
    }
}

/// **The stage run a writer or reader is bound to** (decision 8): the item, the stage whose step
/// writes or reads, and that stage run's `run_id` (ADR-0152's `format!("{}-{}", stage,
/// &digest[..16])`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageRun {
    pub item: String,
    pub stage: String,
    pub run: String,
}

fn str_of<'a>(e: &'a Event, key: &str) -> Option<&'a str> {
    e.payload.get(key).and_then(Json::as_str)
}

fn is_row_of(e: &Event, kind: EventKind, at: &StageRun) -> bool {
    e.kind == kind
        && str_of(e, "item") == Some(at.item.as_str())
        && str_of(e, "stage") == Some(at.stage.as_str())
        && str_of(e, "run") == Some(at.run.as_str())
}

/// The number of `reconcile_started` rows naming `rk-run/<item>`: an entry's ordinal.
fn entries_of(rows: &[Event], item: &str) -> usize {
    let branch = format!("rk-run/{item}");
    rows.iter()
        .filter(|e| e.kind == EventKind::ReconcileStarted)
        .filter(|e| str_of(e, "branch") == Some(branch.as_str()))
        .count()
}

// =================================================================================================
// The writer.
// =================================================================================================

/// What one write did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    /// The repo-relative path the row names.
    pub path: String,
    /// The digest the row carries: the `rk-types` digest of the bytes at `path`.
    pub digest: Digest,
    /// `true` when served from the journal on replay: zero writes, no row.
    pub replayed: bool,
}

/// **The single write path** (AC-122; decisions 7 and 8). A struct, never a trait.
#[derive(Debug)]
pub struct ArtifactWriter {
    dir: ManagedDir,
    at: StageRun,
    schemas: BTreeMap<String, String>,
    journal: Journal,
    replay: bool,
}

impl ArtifactWriter {
    /// A live writer over `tree` (the run-owned worktree), bound to one stage run. `schemas` maps a
    /// kind to its registered `schema` string (`artifacts.toml`); a kind it does not name writes
    /// `schema_version: null`.
    #[must_use]
    pub fn new(
        tree: &Path,
        at: StageRun,
        schemas: BTreeMap<String, String>,
        journal: Journal,
    ) -> Self {
        Self {
            dir: ManagedDir::of(tree, &at.item),
            at,
            schemas,
            journal,
            replay: false,
        }
    }

    /// A REPLAYING writer: every write is served from the journal's recorded row, and the
    /// filesystem is never touched (decision 7).
    #[must_use]
    pub fn replaying(
        tree: &Path,
        at: StageRun,
        schemas: BTreeMap<String, String>,
        journal: Journal,
    ) -> Self {
        Self {
            replay: true,
            ..Self::new(tree, at, schemas, journal)
        }
    }

    /// The managed directory this writer writes.
    #[must_use]
    pub const fn dir(&self) -> &ManagedDir {
        &self.dir
    }

    /// The stage run this writer is bound to.
    #[must_use]
    pub const fn stage_run(&self) -> &StageRun {
        &self.at
    }

    fn schema_version(&self, kind: &str) -> Json {
        self.schemas
            .get(kind)
            .map_or(Json::Null, |s| Json::String(s.clone()))
    }

    /// The one `artifact.write` row (decision 8).
    fn row(&self, kind: &str, path: &str, digest: &Digest) -> Event {
        Event::new(
            EventKind::ArtifactWrite,
            serde_json::json!({
                "digest": digest.as_str(),
                "item": self.at.item,
                "kind": kind,
                "path": path,
                "run": self.at.run,
                "schema_version": self.schema_version(kind),
                "stage": self.at.stage,
            }),
        )
        .with_item(self.at.item.clone())
    }

    /// Replay: the recorded row of this stage run for `path`, or a refusal.
    fn served(&self, kind: &str, path: &str, derived: &Digest) -> Result<Written, RenderError> {
        let rows = self.journal.rows()?;
        let recorded = rows
            .iter()
            .rev()
            .filter(|e| is_row_of(e, EventKind::ArtifactWrite, &self.at))
            .find(|e| str_of(e, "kind") == Some(kind) && str_of(e, "path") == Some(path))
            .and_then(|e| str_of(e, "digest"))
            .ok_or_else(|| RenderError::ReplayUnrecorded {
                kind: kind.to_owned(),
                path: path.to_owned(),
            })?;
        if recorded != derived.as_str() {
            return Err(RenderError::ReplayDiverged {
                kind: kind.to_owned(),
                recorded: recorded.to_owned(),
                derived: derived.as_str().to_owned(),
            });
        }
        Ok(Written {
            path: path.to_owned(),
            digest: derived.clone(),
            replayed: true,
        })
    }

    /// Put `bytes` at `name` unless the file already holds exactly them.
    fn put(&self, name: &str, bytes: &[u8]) -> Result<(), RenderError> {
        let prior = self.dir.read(name);
        if prior.as_deref() == Some(bytes) {
            return Ok(());
        }
        self.dir.write(name, bytes)
    }

    /// **Write a kind's typed value**: its canonical sidecar and, for a human-facing kind, its
    /// layout-C rendering, with ONE `artifact.write` naming the sidecar and its digest.
    ///
    /// # Errors
    /// [`RenderError`] from the renderer, the filesystem, the journal, or replay.
    pub fn write(&self, kind: &str, value: &Json) -> Result<Written, RenderError> {
        let sidecar = canon::canonical(value).map_err(|e| canon_err(kind, &e))?;
        let rendering = template_of(kind).map(|t| render(t, value)).transpose()?;
        let path = self.dir.rel(&sidecar_name(kind));
        let digest = sha256(&sidecar);
        if self.replay {
            return self.served(kind, &path, &digest);
        }
        self.put(&sidecar_name(kind), &sidecar)?;
        if let Some(md) = rendering {
            self.put(&rendering_name(kind), &md)?;
        }
        self.journal.append(self.row(kind, &path, &digest))?;
        Ok(Written {
            path,
            digest,
            replayed: false,
        })
    }

    /// **The restore** (AC-411): re-render a human-facing kind's `.md` from its typed value, a
    /// write with its ONE `artifact.write` naming the rendering and its digest.
    ///
    /// # Errors
    /// As [`Self::write`].
    pub fn restore(&self, template: &Template, value: &Json) -> Result<Written, RenderError> {
        let md = render(template, value)?;
        let path = self.dir.rel(&rendering_name(template.kind));
        let digest = sha256(&md);
        if self.replay {
            return self.served(template.kind, &path, &digest);
        }
        self.dir.write(&rendering_name(template.kind), &md)?;
        self.journal
            .append(self.row(template.kind, &path, &digest))?;
        Ok(Written {
            path,
            digest,
            replayed: false,
        })
    }

    /// **A vouched root's store** (AC-127's first half; ADR-0189 decision 10, proposed): `bytes`
    /// at `name` in the managed directory, with ONE `artifact.write` naming the path and the
    /// `rk-types` digest of the bytes. The caller has checked the digest against the attach's;
    /// this is the single write path that puts them there. On replay: zero writes, the recorded
    /// digest served. *(Added 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED.)*
    pub(crate) fn write_root(
        &self,
        kind: &str,
        name: &str,
        bytes: &[u8],
    ) -> Result<Written, RenderError> {
        let path = self.dir.rel(name);
        let digest = sha256(bytes);
        if self.replay {
            return self.served(kind, &path, &digest);
        }
        self.put(name, bytes)?;
        self.journal.append(self.row(kind, &path, &digest))?;
        Ok(Written {
            path,
            digest,
            replayed: false,
        })
    }

    /// The item's rows, in append order (the vouched pass reads the attach rows off them).
    pub(crate) fn rows(&self) -> Result<Vec<Event>, RenderError> {
        self.journal.rows()
    }

    /// Append one row through the writer's journal (the vouched pass's admitted advisory).
    pub(crate) fn append(&self, event: Event) -> Result<(), RenderError> {
        self.journal.append(event)
    }

    /// Is this a replaying writer?
    pub(crate) const fn is_replaying(&self) -> bool {
        self.replay
    }

    /// A kind's typed value, read from its sidecar in the managed directory — never from the
    /// markdown (AC-477).
    ///
    /// # Errors
    /// [`RenderError::Sidecar`] when it is absent or not JSON.
    pub fn value(&self, kind: &str) -> Result<Json, RenderError> {
        let name = sidecar_name(kind);
        let sidecar = |detail: String| RenderError::Sidecar {
            kind: kind.to_owned(),
            path: self.dir.rel(&name),
            detail,
        };
        let bytes = self
            .dir
            .read(&name)
            .ok_or_else(|| sidecar("absent from the managed directory".to_owned()))?;
        value_of_sidecar(&bytes).map_err(sidecar)
    }

    /// The kinds this writer's item has a sidecar for, named by the store's own `artifact.write`
    /// rows — never by a walk of the directory (decision 6).
    fn written_kinds(&self, rows: &[Event]) -> BTreeSet<String> {
        rows.iter()
            .filter(|e| e.kind == EventKind::ArtifactWrite)
            .filter(|e| str_of(e, "item") == Some(self.at.item.as_str()))
            .filter_map(|e| {
                let kind = str_of(e, "kind")?;
                (str_of(e, "path") == Some(self.dir.rel(&sidecar_name(kind)).as_str()))
                    .then(|| kind.to_owned())
            })
            .collect()
    }
}

// =================================================================================================
// The entry pass — C2's surface-and-restore half.
// =================================================================================================

/// One structured drift the entry pass surfaced (decision 4): the `structured_drift` row's members.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Drift {
    pub kind: String,
    pub anchor: String,
    pub old_digest: Digest,
    pub new_digest: Digest,
    pub causing_commit: Option<String>,
}

/// **What the entry pass did** — and the proof a reader needs that it ran (AC-126).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciled {
    entry: usize,
    drifts: Vec<Drift>,
    accepted: Vec<(String, String)>,
}

impl Reconciled {
    /// Every structured drift surfaced, in the order appended. The kernel opens the drift gate
    /// over these (decision 4); an empty list opens none.
    #[must_use]
    pub fn drifts(&self) -> &[Drift] {
        &self.drifts
    }

    /// Every `(kind, prose field)` accepted whole as the field's new value (AC-152).
    #[must_use]
    pub fn accepted(&self) -> &[(String, String)] {
        &self.accepted
    }

    /// The entry this pass reconciled: the ordinal of the `reconcile_started` row naming
    /// `rk-run/<item>` that it ran after, `0` when there was none.
    #[must_use]
    pub const fn entry(&self) -> usize {
        self.entry
    }
}

fn drift_row(item: &str, d: &Drift) -> Event {
    Event::new(
        EventKind::StructuredDrift,
        serde_json::json!({
            "anchor": d.anchor,
            "causing_commit": d.causing_commit,
            "item": item,
            "kind": d.kind,
            "new_digest": d.new_digest.as_str(),
            "old_digest": d.old_digest.as_str(),
        }),
    )
    .with_item(item.to_owned())
}

/// The drifts of one rendering against its value: frontmatter first, then each structured section
/// in byte-wise order. Prose sections are never compared here.
fn drifts_of(
    template: &Template,
    value: &Json,
    disk: Option<&str>,
) -> Result<Vec<(String, Digest, Digest)>, RenderError> {
    let Some(disk) = disk else {
        return Ok(vec![(
            DOCUMENT_ANCHOR.to_owned(),
            sha256(&render(template, value)?),
            sha256(b""),
        )]);
    };
    let mut out = Vec::new();
    let expected = frontmatter_block(template, value)?;
    let observed = locator::frontmatter(disk).map_or(&b""[..], |r| &disk.as_bytes()[r]);
    if observed != expected.as_slice() {
        out.push((
            FRONTMATTER_ANCHOR.to_owned(),
            sha256(&expected),
            sha256(observed),
        ));
    }
    for key in section_keys(template, value)? {
        if is_prose(template, &key) {
            continue;
        }
        let expected = section_slice(template, &key, value)?;
        let anchor = Anchor {
            text: key.clone(),
            level: SECTION_LEVEL,
        };
        let observed = locator::resolve(disk, &anchor).map_or(&b""[..], |r| &disk.as_bytes()[r]);
        if observed != expected.as_slice() {
            out.push((key, sha256(&expected), sha256(observed)));
        }
    }
    Ok(out)
}

/// One structured section, or the frontmatter, whose bytes on disk differ from the rendering of the
/// typed value: `doctor`'s `RK-D-ARTIFACT-MUTATION` reads these (AC-159).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    /// The section's anchor text, [`FRONTMATTER_ANCHOR`], or [`DOCUMENT_ANCHOR`] when the rendering
    /// is absent.
    pub anchor: String,
    /// The digest of the rendering's bytes for that anchor.
    pub rendered: Digest,
    /// The digest of the bytes on disk for that anchor.
    pub observed: Digest,
}

/// **The per-section comparison `doctor` asks for** (AC-159; ADR-0189 decision 13, proposed): the
/// frontmatter and every structured section of `rendering` (the `.md` on disk, `None` when absent)
/// compared with the rendering of the value in `sidecar`, located by [`crate::locator`] — `doctor`
/// parses no markdown itself (AC-120). **Prose sections are never compared** (AC-152), and **no
/// whole-file digest is taken** (PIA-043's amendment). `Ok(None)` for a machine-only kind, which has
/// no rendering to compare. *(Added 2026-09-29, s8f build, package S8F-P8-DOCTOR-RULES-AND-NOTES.)*
///
/// # Errors
/// [`RenderError::Sidecar`] when `sidecar` is not JSON; the renderer's own.
pub fn differences(
    kind: &str,
    sidecar: &[u8],
    rendering: Option<&[u8]>,
) -> Result<Option<Vec<Difference>>, RenderError> {
    let Some(template) = template_of(kind) else {
        return Ok(None);
    };
    let value = value_of_sidecar(sidecar).map_err(|detail| RenderError::Sidecar {
        kind: kind.to_owned(),
        path: sidecar_name(kind),
        detail,
    })?;
    let disk = rendering.map(|b| String::from_utf8_lossy(b).into_owned());
    Ok(Some(
        drifts_of(template, &value, disk.as_deref())?
            .into_iter()
            .map(|(anchor, rendered, observed)| Difference {
                anchor,
                rendered,
                observed,
            })
            .collect(),
    ))
}

/// The value with every prose section edited on disk accepted whole (AC-152): the field's new
/// value is the section's bytes, uninspected. `None` when nothing changed.
fn accepted_prose(
    template: &Template,
    value: &Json,
    disk: &str,
) -> Result<Option<(Json, Vec<String>)>, RenderError> {
    let mut next = value.clone();
    let mut fields = Vec::new();
    for field in template.prose {
        let anchor = Anchor {
            text: (*field).to_owned(),
            level: SECTION_LEVEL,
        };
        let Ok(range) = locator::resolve(disk, &anchor) else {
            continue;
        };
        let on_disk = &disk[range];
        let current = prose_of(template, field, object(template, value)?.get(*field))?;
        if prose_normal(on_disk) != prose_normal(&current) {
            if let Some(map) = next.as_object_mut() {
                map.insert((*field).to_owned(), Json::String(on_disk.to_owned()));
            }
            fields.push((*field).to_owned());
        }
    }
    Ok((!fields.is_empty()).then_some((next, fields)))
}

/// **The entry pass** (decision 4; AC-124, AC-411, AC-152), run after the re-entry discard and the
/// entry's reconcile. For each human-facing kind the store has written for the item:
///
/// 1. a prose section edited on disk is accepted whole into the value;
/// 2. each structured section, and the frontmatter, that differs from its rendering appends one
///    `structured_drift` `{ kind, item, anchor, old_digest, new_digest, causing_commit }`;
/// 3. THEN the file is restored from the type — a write with its one `artifact.write`.
///
/// `causing` answers the last commit on `rk-run/<item>` that touched a repo-relative path (the
/// composition root's `log -1 --format=%H -- <path>`, `GitOp::Log`), or `None`.
///
/// # Errors
/// [`RenderError`] from the journal, the renderer, the filesystem or a sidecar.
pub fn reconcile_entry(
    writer: &ArtifactWriter,
    causing: &CausingCommit,
) -> Result<Reconciled, RenderError> {
    let rows = writer.journal.rows()?;
    let item = writer.at.item.clone();
    let mut done = Reconciled {
        entry: entries_of(&rows, &item),
        drifts: Vec::new(),
        accepted: Vec::new(),
    };
    for kind in writer.written_kinds(&rows) {
        let Some(template) = template_of(&kind) else {
            continue;
        };
        let value = writer.value(&kind)?;
        let md_name = rendering_name(&kind);
        let disk = writer
            .dir
            .read(&md_name)
            .map(|b| String::from_utf8_lossy(&b).into_owned());
        let accepted = match &disk {
            Some(disk) => accepted_prose(template, &value, disk)?,
            None => None,
        };
        let drifts = drifts_of(template, &value, disk.as_deref())?;
        if !drifts.is_empty() {
            let md_path = writer.dir.rel(&md_name);
            let commit =
                causing(&md_path).map_err(|detail| RenderError::CausingCommit { detail })?;
            for (anchor, old_digest, new_digest) in drifts {
                let d = Drift {
                    kind: kind.clone(),
                    anchor,
                    old_digest,
                    new_digest,
                    causing_commit: commit.clone(),
                };
                // *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE.)* On replay the drift
                // row is RECORDED: it is served, never appended again (decision 7; ADR-0105
                // clause 1), and a drift the journal does not record is refused by name.
                let row = drift_row(&item, &d);
                if writer.replay {
                    if !rows
                        .iter()
                        .any(|e| e.kind == row.kind && e.payload == row.payload)
                    {
                        return Err(RenderError::ReplayUnrecorded {
                            kind: kind.clone(),
                            path: md_path.clone(),
                        });
                    }
                } else {
                    writer.journal.append(row)?;
                }
                done.drifts.push(d);
            }
        }
        match accepted {
            Some((next, fields)) => {
                writer.write(&kind, &next)?;
                done.accepted
                    .extend(fields.into_iter().map(|f| (kind.clone(), f)));
            }
            None if done.drifts.iter().any(|d| d.kind == kind) => {
                writer.restore(template, &value)?;
            }
            None => {}
        }
    }
    Ok(done)
}

/// **`move-to-prose:<section>`** (decision 5): the drifted bytes of `drift` (a `structured_drift`
/// row's payload) are read from its `causing_commit` through `read_at(commit, path)` — never from
/// the event, which carries digests only — located there by the drift's anchor, and appended to
/// the prose section `section` through the writer, as one `artifact.write`. No drift text reaches a
/// deserializer: it lands in opaque prose.
///
/// # Errors
/// [`RenderError::MoveToProse`] for a kind with no template, a section that is not prose, or a
/// drift row without a causing commit; [`RenderError::CausingCommit`] when the bytes cannot be
/// read or the anchor is not in them; and the writer's own.
pub fn move_to_prose(
    writer: &ArtifactWriter,
    drift: &Json,
    section: &str,
    read_at: &ReadAt,
) -> Result<Written, RenderError> {
    let refuse = |detail: String| RenderError::MoveToProse { detail };
    let field = |k: &str| drift.get(k).and_then(Json::as_str);
    let kind = field("kind").ok_or_else(|| refuse("the drift row names no `kind`".to_owned()))?;
    let anchor =
        field("anchor").ok_or_else(|| refuse("the drift row names no `anchor`".to_owned()))?;
    let template = template_of(kind)
        .ok_or_else(|| refuse(format!("`{kind}` is machine-only and has no prose section")))?;
    if !is_prose(template, section) {
        return Err(refuse(format!(
            "`{section}` is not a prose section of `{kind}` (its prose sections: {:?})",
            template.prose
        )));
    }
    // *(dated 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2.)* A `document`
    // drift is a MISSING rendering (`drifts_of`): the drifted state is the absent file, whose
    // bytes are none (its `new_digest` is the empty digest), and its causing commit is the one
    // that removed the file, where it cannot be read. So the move reads no commit and appends
    // nothing; it is the answer's one typed write through the writer (its `artifact.write` is the
    // answer's applied mark). A `document` row naming any other digest is refused by name.
    if anchor == DOCUMENT_ANCHOR {
        if field("new_digest") != Some(sha256(b"").as_str()) {
            return Err(refuse(format!(
                "the `{kind}` drift at `{DOCUMENT_ANCHOR}` names bytes ({}) where a missing \
                 rendering has none, so there is nothing this move can find",
                field("new_digest").unwrap_or("no digest")
            )));
        }
        return writer.write(kind, &writer.value(kind)?);
    }
    let commit = field("causing_commit").ok_or_else(|| {
        refuse(format!(
            "the `{kind}` drift at `{anchor}` names no causing commit, so there are no drifted \
             bytes to move"
        ))
    })?;
    let path = writer.dir.rel(&rendering_name(kind));
    let bytes = read_at(commit, &path).map_err(|detail| RenderError::CausingCommit { detail })?;
    let text = String::from_utf8_lossy(&bytes);
    let range = if anchor == FRONTMATTER_ANCHOR {
        locator::frontmatter(&text)
    } else {
        locator::resolve(
            &text,
            &Anchor {
                text: anchor.to_owned(),
                level: SECTION_LEVEL,
            },
        )
        .ok()
    }
    .ok_or_else(|| RenderError::CausingCommit {
        detail: format!("the anchor `{anchor}` is not in `{path}` at {commit}"),
    })?;
    // *(dated 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2's minor.)* The
    // drifted bytes are appended FENCED, so no line of them is a heading the locator reads in the
    // prose section (a fenced block is one of its decoys): the frontmatter's `# rk-stamp …` line
    // would otherwise end the section, and the same entry's pass would accept the store's own
    // rendering as a human's prose edit. The bytes inside the fence are the commit's, unchanged.
    let drifted = fenced(&text[range]);
    let mut value = writer.value(kind)?;
    let prior = prose_of(template, section, object(template, &value)?.get(section))?;
    let body = prior.replace("\r\n", "\n");
    let body = body.trim_end_matches('\n');
    let joined = if body.is_empty() {
        drifted
    } else {
        format!("{body}\n\n{drifted}")
    };
    if let Some(map) = value.as_object_mut() {
        map.insert(section.to_owned(), Json::String(joined));
    }
    writer.write(kind, &value)
}

/// **Opaque bytes, fenced** for a prose section: a backtick fence one longer than the longest
/// backtick run in `text` (and at least three), so no line inside is a heading the locator reads
/// and no line inside closes the fence. The bytes between the fences are `text`'s, with one
/// trailing newline. *(Added 2026-09-29, s8f build, package R2FIX-8f-A-CODE.)*
fn fenced(text: &str) -> String {
    let longest = text.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest.max(2) + 1);
    let body = text.strip_suffix('\n').unwrap_or(text);
    format!("{fence}\n{body}\n{fence}\n")
}

// =================================================================================================
// The reader — the only emitter of `artifact.read`.
// =================================================================================================

/// A blob on the run ref, and the tree it was read from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRefBlob {
    pub bytes: Vec<u8>,
    pub tree: String,
}

type SourceFn = dyn Fn(&str) -> Result<Option<RunRefBlob>, String> + Send + Sync;

/// `(tree, path)` to the blob at `path` in `tree`, `None` when the path is not in it: the
/// composition root's `cat-file blob <tree>:<path>`. *(Added 2026-09-29, s8f build, package
/// R5FIX-8f; D21-113 step 4: a replayed read serves the blob at the RECORDED tree.)*
type AtTreeFn = dyn Fn(&str, &str) -> Result<Option<Vec<u8>>, String> + Send + Sync;

/// What one read served.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Served {
    pub value: Json,
    pub digest: Digest,
    pub tree: String,
    /// `true` when served from the journal on replay: no row appended.
    pub replayed: bool,
}

/// **The read seam** (N12; D21-47; decision 9). A struct, bound per stage run.
pub struct ArtifactReader {
    at: StageRun,
    schemas: BTreeMap<String, String>,
    journal: Journal,
    source: Box<SourceFn>,
    /// The replaying reader's read of a recorded tree (D21-113 step 4); the live reader's never
    /// answers.
    at_tree: Box<AtTreeFn>,
    entry: usize,
    replay: bool,
}

impl std::fmt::Debug for ArtifactReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArtifactReader")
            .field("at", &self.at)
            .field("entry", &self.entry)
            .field("replay", &self.replay)
            .finish_non_exhaustive()
    }
}

impl ArtifactReader {
    /// A live reader, bound to the entry `reconciled` names. `source(path)` reads `path` on
    /// `rk-run/<item>` and the tree it was read from (the composition root's `cat-file` and
    /// `rev-parse`), `None` when the path is not on the ref.
    #[must_use]
    pub fn new(
        reconciled: &Reconciled,
        at: StageRun,
        schemas: BTreeMap<String, String>,
        journal: Journal,
        source: impl Fn(&str) -> Result<Option<RunRefBlob>, String> + Send + Sync + 'static,
    ) -> Self {
        Self {
            at,
            schemas,
            journal,
            source: Box::new(source),
            at_tree: Box::new(|_, _| Ok(None)),
            entry: reconciled.entry,
            replay: false,
        }
    }

    /// A REPLAYING reader: a read is served from the journal's recorded row and appends nothing.
    /// *(dated 2026-09-29, s8f build, package R5FIX-8f; D21-113 step 4.)* `at_tree(tree, path)`
    /// reads `path` in a recorded `tree` (the composition root's `cat-file`), so the value served
    /// is the blob the recorded read served, checked against its recorded digest.
    #[must_use]
    pub fn replaying(
        reconciled: &Reconciled,
        at: StageRun,
        schemas: BTreeMap<String, String>,
        journal: Journal,
        at_tree: impl Fn(&str, &str) -> Result<Option<Vec<u8>>, String> + Send + Sync + 'static,
    ) -> Self {
        Self {
            replay: true,
            at_tree: Box::new(at_tree),
            ..Self::new(reconciled, at, schemas, journal, |_| Ok(None))
        }
    }

    /// The stage run this reader is bound to.
    #[must_use]
    pub const fn stage_run(&self) -> &StageRun {
        &self.at
    }

    /// **Serve one read of `kind`** — the value from its sidecar on the run ref and its digest —
    /// and append its ONE `artifact.read`. `bound` is the committed `emit` binding the resolver
    /// read, whose digest the served bytes must equal (`None` for a root, which has no binding).
    /// *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-113 step 3, D21-114.)* ~~whose digest
    /// the served bytes must equal~~: the served bytes must equal the LATEST `artifact.write` of
    /// the sidecar's path, and the binding's digest only when there is none.
    ///
    /// # Errors
    /// [`RenderError::NotReconciled`] before the entry's reconcile; [`RenderError::DigestMismatch`]
    /// (Halt 4); [`RenderError::Sidecar`]; the journal's; on replay, an unrecorded read.
    pub fn serve(&self, kind: &str, bound: Option<&Json>) -> Result<Served, RenderError> {
        let rows = self.journal.rows()?;
        // 🛑 AC-126: `reconcile()` runs before any read is served, and nothing skips it.
        let current = entries_of(&rows, &self.at.item);
        if self.entry == 0 || current != self.entry {
            return Err(RenderError::NotReconciled {
                kind: kind.to_owned(),
                item: self.at.item.clone(),
                bound_entry: self.entry,
                journal_entries: current,
            });
        }
        let path = managed_path(&self.at.item, &sidecar_name(kind));
        if self.replay {
            let recorded = rows
                .iter()
                .rev()
                .filter(|e| is_row_of(e, EventKind::ArtifactRead, &self.at))
                .find(|e| str_of(e, "kind") == Some(kind))
                .ok_or_else(|| RenderError::ReplayUnrecorded {
                    kind: kind.to_owned(),
                    path: path.clone(),
                })?;
            let digest = str_of(recorded, "digest")
                .and_then(|d| Digest::from_hex(d).ok())
                .ok_or_else(|| RenderError::ReplayUnrecorded {
                    kind: kind.to_owned(),
                    path: path.clone(),
                })?;
            // *(dated 2026-09-29, s8f build, package R5FIX-8f; D21-113 step 4, TAKEN
            // PROVISIONALLY under D21-105; ADR-0189 decision 9's dated note, proposed.)*
            // ~~`value: bound.cloned().unwrap_or(Json::Null)`~~: replay serves the sidecar blob
            // at the recorded read's `tree`, checked against its recorded `digest`, and appends
            // nothing. A tree without the blob is unrecorded; a blob that is not the recorded
            // digest is a halt (Halt 4), never a silent choice.
            let tree = str_of(recorded, "tree").unwrap_or_default().to_owned();
            let bytes = (self.at_tree)(&tree, &path)
                .map_err(|detail| RenderError::Sidecar {
                    kind: kind.to_owned(),
                    path: path.clone(),
                    detail,
                })?
                .ok_or_else(|| RenderError::ReplayUnrecorded {
                    kind: kind.to_owned(),
                    path: path.clone(),
                })?;
            let served = sha256(&bytes);
            if served != digest {
                return Err(RenderError::DigestMismatch {
                    kind: kind.to_owned(),
                    served: served.as_str().to_owned(),
                    bound: digest.as_str().to_owned(),
                });
            }
            let value = value_of_sidecar(&bytes).map_err(|detail| RenderError::Sidecar {
                kind: kind.to_owned(),
                path: path.clone(),
                detail,
            })?;
            return Ok(Served {
                value,
                digest,
                tree,
                replayed: true,
            });
        }
        let sidecar = |detail: String| RenderError::Sidecar {
            kind: kind.to_owned(),
            path: path.clone(),
            detail,
        };
        let blob = (self.source)(&path)
            .map_err(sidecar)?
            .ok_or_else(|| sidecar(format!("not on rk-run/{}", self.at.item)))?;
        let digest = sha256(&blob.bytes);
        // *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-113 step 3 as amended by D21-114,
        // both TAKEN PROVISIONALLY under D21-105; ADR-0189 decision 9's dated note, proposed.)*
        // ~~`if let Some(bound) = bound { … canonical_digest(bound) … }`~~: the run ref's sidecar is
        // checked against the LATEST `artifact.write` of this item whose `path` is the sidecar's
        // own, `<Kind>.json`, never selected by kind (a restore names `<Kind>.md`). With no such
        // row the check falls back to the binding's canonical digest, byte-identical to before;
        // anything else is Halt 4. The entry pass's own writes are on the run ref by now, landed
        // at entry (D21-114; ADR-0188 decision 2's dated note), so this cannot livelock.
        let latest = rows
            .iter()
            .rev()
            .filter(|e| e.kind == EventKind::ArtifactWrite)
            .filter(|e| str_of(e, "item") == Some(self.at.item.as_str()))
            .find(|e| str_of(e, "path") == Some(path.as_str()))
            .map(|e| str_of(e, "digest").unwrap_or_default().to_owned());
        let expected = match (latest, bound) {
            (Some(written), _) => Some(written),
            (None, Some(bound)) => Some(
                canon::canonical_digest(bound)
                    .map_err(|e| canon_err(kind, &e))?
                    .as_str()
                    .to_owned(),
            ),
            (None, None) => None,
        };
        if let Some(expected) = expected
            && expected != digest.as_str()
        {
            return Err(RenderError::DigestMismatch {
                kind: kind.to_owned(),
                served: digest.as_str().to_owned(),
                bound: expected,
            });
        }
        let value = value_of_sidecar(&blob.bytes).map_err(sidecar)?;
        self.journal.append(
            Event::new(
                EventKind::ArtifactRead,
                serde_json::json!({
                    "digest": digest.as_str(),
                    "item": self.at.item,
                    "kind": kind,
                    "run": self.at.run,
                    "schema_version": self
                        .schemas
                        .get(kind)
                        .map_or(Json::Null, |s| Json::String(s.clone())),
                    "stage": self.at.stage,
                    "tree": blob.tree,
                }),
            )
            .with_item(self.at.item.clone()),
        )?;
        Ok(Served {
            value,
            digest,
            tree: blob.tree,
            replayed: false,
        })
    }
}
