//! The activity catalog, and the **closed** evidence set each step kind mints.
//!
//! This is the half of the catalog the rigor-monotonicity rule needs and no more: the `[activity]`
//! header the selection filter reads, and the `[[step]]` list ADR-0036 turns into an evidence
//! promise. Nothing here dispatches, and nothing here is the selection function — that is Slice 3's
//! seven-step pure function and it is **not built**.
//!
//! # Minting is ADR-0036's, and it is closed
//!
//! *"a `tool` step mints `ToolReport` (plus `GreenAt` when `run = toolchain:test` and status is
//! passing; plus `Mutation` when the output is a `MutationReport`); an `agent` step mints
//! `Findings` or `SelfCheck` depending on its output type; a `gate` step mints `HumanAnswer`;
//! `check` and `emit` steps mint no evidence."* That is the whole rule, and it is what makes
//! ADR-0081 clause 1 *"computable from the catalog with nothing run"*.
//!
//! **One half of it is not catalog-computable, and the reading is stated rather than taken
//! silently.** `GreenAt`'s condition includes *"and status is passing"*, which is a run-time fact.
//! [`Step::mints`] mints `GreenAt` for `run = "toolchain:test"` on the catalog half alone, because
//! ADR-0081's own premise is that `evidence(a)` is computable without running anything, and because
//! the alternative — dropping `GreenAt` entirely — would drop it from **both** sides of every
//! superset comparison and change no verdict while making the promise wrong. The activity-level
//! `evidence = […]` promises in the shipped catalog list `GreenAt` unconditionally beside a
//! `toolchain:test` step, which is the same reading.

use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

use crate::toml_subset::{self, Table, Value};

/// The five step kinds of the activity-graph schema §1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StepKind {
    Agent,
    Tool,
    Check,
    Gate,
    Emit,
}

impl StepKind {
    fn from_token(s: &str) -> Option<Self> {
        match s {
            "agent" => Some(Self::Agent),
            "tool" => Some(Self::Tool),
            "check" => Some(Self::Check),
            "gate" => Some(Self::Gate),
            "emit" => Some(Self::Emit),
            _ => None,
        }
    }
}

/// The closed `Evidence` enum of the activity-graph schema §2.
///
/// `Degraded` and `Attempts` are members of the closed set and are minted by the **runtime**, not
/// by a step kind — ADR-0036's mapping names neither — so nothing in this module produces them.
/// They are listed because the set is closed and a reader must be able to see that they were not
/// forgotten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evidence {
    SelfCheck,
    Findings,
    ToolReport,
    GreenAt,
    Mutation,
    HumanAnswer,
    Degraded,
    Attempts,
}

impl fmt::Display for Evidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::SelfCheck => "SelfCheck",
            Self::Findings => "Findings",
            Self::ToolReport => "ToolReport",
            Self::GreenAt => "GreenAt",
            Self::Mutation => "Mutation",
            Self::HumanAnswer => "HumanAnswer",
            Self::Degraded => "Degraded",
            Self::Attempts => "Attempts",
        };
        f.write_str(name)
    }
}

/// One `[[step]]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub id: String,
    pub kind: StepKind,
    /// `run = "toolchain:…"` for a `tool` step.
    pub run: Option<String>,
    /// `outputs = "value:…"`, with the `value:` prefix stripped.
    pub output_type: Option<String>,
    /// The step's own `when` guard, verbatim. `None` means the step always runs.
    pub when: Option<String>,
}

impl Step {
    /// The evidence variants this step mints, per ADR-0036.
    #[must_use]
    pub fn mints(&self) -> BTreeSet<Evidence> {
        let mut out = BTreeSet::new();
        match self.kind {
            StepKind::Tool => {
                out.insert(Evidence::ToolReport);
                if self.run.as_deref() == Some("toolchain:test") {
                    out.insert(Evidence::GreenAt);
                }
                if self.output_type.as_deref() == Some("MutationReport") {
                    out.insert(Evidence::Mutation);
                }
            }
            StepKind::Agent => {
                if self.output_type.as_deref() == Some("Findings") {
                    out.insert(Evidence::Findings);
                } else {
                    out.insert(Evidence::SelfCheck);
                }
            }
            StepKind::Gate => {
                out.insert(Evidence::HumanAnswer);
            }
            StepKind::Check | StepKind::Emit => {}
        }
        out
    }
}

/// One `[activity]` plus its steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    /// The file this activity was read from — carried so a violation names a path a reader can
    /// open, rather than an id they then have to find.
    pub source: String,
    pub id: String,
    pub stage: String,
    pub provides: Vec<String>,
    pub tags: Vec<String>,
    pub applicable_if: Option<String>,
    pub method_rank: i64,
    pub min_rigor: String,
    pub max_rigor: Option<String>,
    pub family: Option<String>,
    pub steps: Vec<Step>,
}

/// A directory of activities.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Catalog {
    activities: Vec<Activity>,
}

/// What stopped a catalog load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    /// The directory could not be read, or held no `.toml` at all. **An empty catalog is an
    /// error, not an empty success**: a rule that enumerates nothing passes, which is Halt 8's
    /// *"a filter that matches nothing exits 0"* one level up.
    Io {
        path: String,
        detail: String,
    },
    Toml {
        source: String,
        error: toml_subset::TomlError,
    },
    Shape {
        source: String,
        detail: String,
    },
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, detail } => write!(f, "{path}: {detail}"),
            Self::Toml { source, error } => write!(f, "{source}: {error}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
        }
    }
}

impl std::error::Error for CatalogError {}

impl Catalog {
    /// Build from named sources, so a fixture catalog can be `include_str!`'d rather than opened.
    ///
    /// # Errors
    ///
    /// [`CatalogError`] for an empty set, a file outside the TOML subset, or an activity missing a
    /// key the selection filter needs.
    pub fn from_sources(sources: &[(&str, &str)]) -> Result<Self, CatalogError> {
        if sources.is_empty() {
            return Err(CatalogError::Io {
                path: "<sources>".to_owned(),
                detail: "no activity sources were given — an empty catalog is an error, because a \
                         rule that enumerates nothing passes"
                    .to_owned(),
            });
        }
        let mut activities = Vec::with_capacity(sources.len());
        for (name, src) in sources {
            activities.push(activity_from(name, src)?);
        }
        activities.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Self { activities })
    }

    /// Read every `*.toml` in a directory.
    ///
    /// # Errors
    ///
    /// [`CatalogError`] as [`Self::from_sources`], plus an I/O failure naming the directory. A
    /// directory that exists and holds no `*.toml` is an error.
    pub fn from_dir(dir: &Path) -> Result<Self, CatalogError> {
        let io = |detail: String| CatalogError::Io {
            path: dir.display().to_string(),
            detail,
        };
        let entries = std::fs::read_dir(dir).map_err(|e| io(e.to_string()))?;
        let mut paths: Vec<_> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        paths.sort();
        if paths.is_empty() {
            return Err(io(
                "holds no *.toml — an empty catalog is an error, not an empty success".to_owned(),
            ));
        }
        let mut activities = Vec::with_capacity(paths.len());
        for path in &paths {
            let name = path.file_name().map_or_else(
                || path.display().to_string(),
                |n| n.to_string_lossy().into(),
            );
            let src = std::fs::read_to_string(path).map_err(|e| CatalogError::Io {
                path: path.display().to_string(),
                detail: e.to_string(),
            })?;
            activities.push(activity_from(&name, &src)?);
        }
        activities.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Self { activities })
    }

    /// Every activity, by id.
    #[must_use]
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    /// How many activities were read. A rule asserts this is non-zero before it reports green.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.activities.len()
    }

    /// `true` when nothing was read.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.activities.is_empty()
    }
}

fn activity_from(name: &str, src: &str) -> Result<Activity, CatalogError> {
    let doc = toml_subset::parse(src).map_err(|error| CatalogError::Toml {
        source: name.to_owned(),
        error,
    })?;
    let shape = |detail: String| CatalogError::Shape {
        source: name.to_owned(),
        detail,
    };
    let head = doc
        .get("activity")
        .and_then(Value::as_table)
        .ok_or_else(|| shape("no [activity] table".to_owned()))?;
    let text = |t: &Table, key: &str| t.get(key).and_then(Value::as_str).map(ToOwned::to_owned);
    let strings = |t: &Table, key: &str| -> Vec<String> {
        t.get(key)
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let id = text(head, "id").ok_or_else(|| shape("[activity] has no id".to_owned()))?;
    let stage = text(head, "stage").ok_or_else(|| shape(format!("{id}: no stage")))?;
    let method_rank = head
        .get("method_rank")
        .and_then(Value::as_int)
        .ok_or_else(|| shape(format!("{id}: no method_rank")))?;
    let mut steps = Vec::new();
    if let Some(Value::Array(rows)) = doc.get("step") {
        for row in rows {
            let table = row
                .as_table()
                .ok_or_else(|| shape(format!("{id}: a [[step]] is not a table")))?;
            let step_id =
                text(table, "id").ok_or_else(|| shape(format!("{id}: a step has no id")))?;
            let kind_token =
                text(table, "kind").ok_or_else(|| shape(format!("{id}/{step_id}: no kind")))?;
            let kind = StepKind::from_token(&kind_token).ok_or_else(|| {
                shape(format!(
                    "{id}/{step_id}: kind {kind_token:?} is outside the schema's five \
                     (agent, tool, check, gate, emit)"
                ))
            })?;
            steps.push(Step {
                id: step_id,
                kind,
                run: text(table, "run"),
                output_type: text(table, "outputs").map(|o| {
                    o.strip_prefix("value:")
                        .map_or_else(|| o.clone(), ToOwned::to_owned)
                }),
                when: text(table, "when"),
            });
        }
    }
    Ok(Activity {
        source: name.to_owned(),
        id,
        stage,
        provides: strings(head, "provides"),
        tags: strings(head, "tags"),
        applicable_if: text(head, "applicable_if"),
        method_rank,
        min_rigor: text(head, "min_rigor").unwrap_or_else(|| "optional".to_owned()),
        max_rigor: text(head, "max_rigor"),
        family: text(head, "family"),
        steps,
    })
}
