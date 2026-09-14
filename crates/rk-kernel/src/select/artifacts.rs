//! `artifacts.toml` — the artifact-kind registry ADR-0057's Decision mints, and the **authority**
//! that breaks AC-066's and AC-067's tie.
//!
//! ADR-0057: *"A producer's unconditional `requires` may resolve from any stage the pack does not
//! forbid, or from an artifact kind registered `source = "human"` in a new registry,
//! `artifacts.toml` (which also holds the §2.1 kind vocabulary)."*
//!
//! Three fields are read here and no more: `source`, `keyed_by` and `schema`.
//!
//! * **`source`** is what decides the vouched-root path. §7b.9(6): *"`derived_from = []` is **not**
//!   the vouched-root test — the **registered kind's `source`** is: AC-067's GIVEN conjoins
//!   `source = "human"`, and AC-053 with ADR-0057's Decision make `artifacts.toml` the
//!   authority."*
//! * **`keyed_by = "toolchain"`** expands a required kind to one instance per element of the
//!   item's `toolchain` set (ADR-0073, AC-049). This is what replaces the withdrawn specificity
//!   tie-break.
//! * **`schema`** is carried but not interpreted: typing an artifact's body against `schemas.toml`
//!   is `rk-types`' and is not steps 1-4's.

use std::collections::BTreeMap;
use std::fmt;

use crate::toml_subset::{self, Value};

/// The `source` vocabulary `spikes/p9/artifacts.toml` uses, closed and refusing a third token.
///
/// This is **not** `facets.toml`'s five-token provenance set (`human | llm | derived | kernel |
/// declared`, ADR-0093 decision 1): that one classifies where a *facet value* came from, this one
/// classifies whether an artifact *kind* has a producing activity at all. They are different
/// vocabularies over different registries and are deliberately not fused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KindSource {
    /// A **root**: no producer anywhere, supplied at intake or pasted (PC3, `derived_from = []`).
    Human,
    /// Some catalog activity `provides` it.
    Activity,
}

impl KindSource {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Activity => "activity",
        }
    }

    fn from_token(s: &str) -> Option<Self> {
        match s {
            "human" => Some(Self::Human),
            "activity" => Some(Self::Activity),
            _ => None,
        }
    }
}

/// One `[artifact.<Name>]` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactKind {
    pub name: String,
    pub source: KindSource,
    /// `keyed_by = "toolchain"` today; the axis is carried rather than assumed so a second keyed
    /// axis does not need a new field.
    pub keyed_by: Option<String>,
    pub schema: Option<String>,
}

/// One required artifact: a kind, plus the key its expansion gave it.
///
/// `key` is `None` for a kind that is not `keyed_by` anything; `Some("node")` for the `node`
/// instance of a `keyed_by = "toolchain"` kind. **A mixed item cannot be satisfied by a single
/// instance of a keyed kind** (AC-049's second AND), and that is why the key is part of the
/// identity rather than a decoration on it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instance {
    pub kind: String,
    pub key: Option<String>,
    /// The activity whose `requires` pulled this instance in, when it was not a member of `R`
    /// itself. Carried so AC-077's skip row can name the requiring activity rather than only the
    /// stage.
    pub required_by: Option<String>,
}

impl Instance {
    /// An un-keyed instance of `kind`.
    #[must_use]
    pub fn of(kind: &str) -> Self {
        Self {
            kind: kind.to_owned(),
            key: None,
            required_by: None,
        }
    }

    /// The `key`-th instance of a keyed kind.
    #[must_use]
    pub fn keyed(kind: &str, key: &str) -> Self {
        Self {
            kind: kind.to_owned(),
            key: Some(key.to_owned()),
            required_by: None,
        }
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "kind": self.kind,
            "key": self.key,
        })
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.key {
            Some(k) => write!(f, "{}[{k}]", self.kind),
            None => f.write_str(&self.kind),
        }
    }
}

/// What stopped an `artifacts.toml` load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    Toml {
        source: String,
        error: toml_subset::TomlError,
    },
    Shape {
        source: String,
        detail: String,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml { source, error } => write!(f, "{source}: {error}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
        }
    }
}

impl std::error::Error for RegistryError {}

/// The whole registry, by kind name.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArtifactRegistry {
    kinds: BTreeMap<String, ArtifactKind>,
}

impl ArtifactRegistry {
    /// Read one `artifacts.toml`.
    ///
    /// # Errors
    ///
    /// [`RegistryError`] for a file outside the TOML subset, a missing or unrecognised `source`, or
    /// an empty registry — **an empty registry is an error, not an empty success**: every rule
    /// below enumerates it, and a rule that enumerates nothing passes.
    pub fn from_toml(name: &str, src: &str) -> Result<Self, RegistryError> {
        let doc = toml_subset::parse(src).map_err(|error| RegistryError::Toml {
            source: name.to_owned(),
            error,
        })?;
        let shape = |detail: String| RegistryError::Shape {
            source: name.to_owned(),
            detail,
        };
        let table = doc
            .get("artifact")
            .and_then(Value::as_table)
            .ok_or_else(|| shape("no [artifact.*] tables".to_owned()))?;
        let mut kinds = BTreeMap::new();
        for (name, row) in table {
            let row = row
                .as_table()
                .ok_or_else(|| shape(format!("[artifact.{name}] is not a table")))?;
            let token = row
                .get("source")
                .and_then(Value::as_str)
                .ok_or_else(|| shape(format!("[artifact.{name}] has no `source`")))?;
            let source = KindSource::from_token(token).ok_or_else(|| {
                shape(format!(
                    "[artifact.{name}]: `source = {token:?}` is outside the closed set \
                     (human, activity). ADR-0057 mints this registry with two sources and a third \
                     is a decision with a record, not a default."
                ))
            })?;
            kinds.insert(
                name.clone(),
                ArtifactKind {
                    name: name.clone(),
                    source,
                    keyed_by: row
                        .get("keyed_by")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    schema: row
                        .get("schema")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                },
            );
        }
        if kinds.is_empty() {
            return Err(shape(
                "holds no artifact kinds — an empty registry is an error, not an empty success"
                    .to_owned(),
            ));
        }
        Ok(Self { kinds })
    }

    /// The row for `name`, if the registry declares it.
    #[must_use]
    pub fn kind(&self, name: &str) -> Option<&ArtifactKind> {
        self.kinds.get(name)
    }

    /// `true` when the kind is a vouched root's kind — registered `source = "human"`.
    ///
    /// **This, and not `derived_from = []`, is AC-067's test** (§7b.9(6)).
    #[must_use]
    pub fn is_human_sourced(&self, name: &str) -> bool {
        self.kinds
            .get(name)
            .is_some_and(|k| k.source == KindSource::Human)
    }

    /// The axis a kind is keyed by, if any.
    #[must_use]
    pub fn keyed_by(&self, name: &str) -> Option<&str> {
        self.kinds.get(name).and_then(|k| k.keyed_by.as_deref())
    }

    /// How many kinds were read.
    #[must_use]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    /// `true` when nothing was read. Never true of a successfully loaded registry.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }
}
