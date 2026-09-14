//! The item: its context vector, its recorded artifact state, and its rigor overrides.
//!
//! **This is two of AC-041's four digests**, not one. The *context* digest is the facet vector; the
//! *artifact* digests are the recorded bodies and their `derived_from` maps. They are carried on
//! one type because they are one item's, and they are digested separately because AC-041 names
//! them separately.
//!
//! # Why the toolchain is not just a set-valued facet
//!
//! ADR-0022 (i) has a technology pack *"add **facts** to the context vector (`toolchains`,
//! `change_kinds`, `binary_change`), computed **per item from affected paths** via
//! `Toolchain::classify`"*. Classification is therefore **per path**, so each affected path yields
//! a toolchain *and* a change kind, and the item knows which change kinds each toolchain
//! contributed. That is what makes AC-049's *"the generic producer's condition (e.g. `'text' in
//! change`) is evaluated **per instance** rather than once against the item"* mean something: the
//! per-instance context narrows `toolchain` to the singleton **and** `change` to that toolchain's
//! own kinds. [`Item::with_toolchain`] is that per-path classification's result, and the
//! item-level `toolchain` and `change` facets are its unions.

use std::collections::{BTreeMap, BTreeSet};

use super::artifacts::Instance;
use super::canonical::Digest;
use crate::predicate::{Context, Value as PredValue};
use crate::rigor::Rigor;

/// A facet value, as the item records it.
///
/// A separate type from [`crate::predicate::Value`] on purpose: this one is `Eq`, `Ord` and
/// serialisable, because §7b.9(4) requires the evaluated operands to be **typed into**
/// [`super::Unsatisfiable`] rather than re-derived at gate-write time, and a variant carrying a
/// value that cannot be compared or written is not carrying it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FacetValue {
    /// Abstention. ADR-0089: the absence of evidence never resolves to `false`.
    Null,
    Bool(bool),
    Str(String),
    Int(i64),
    /// Carried **with** its domain, so an ordering never has to guess a rank.
    Ordinal {
        domain: Vec<String>,
        index: usize,
    },
    /// A set-valued facet — `toolchain`, `change`, `surfaces`.
    Set(BTreeSet<String>),
}

impl FacetValue {
    pub(in crate::select) fn to_predicate(&self) -> PredValue {
        match self {
            Self::Null => PredValue::Null,
            Self::Bool(b) => PredValue::Bool(*b),
            Self::Str(s) => PredValue::Str(s.clone()),
            Self::Int(i) => PredValue::Int(*i),
            Self::Ordinal { domain, index } => PredValue::Ordinal {
                domain: domain.clone(),
                index: *index,
            },
            Self::Set(items) => PredValue::set(items.iter().cloned()),
        }
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Null => serde_json::Value::Null,
            Self::Bool(b) => (*b).into(),
            Self::Str(s) => s.clone().into(),
            Self::Int(i) => (*i).into(),
            Self::Ordinal { domain, index } => serde_json::json!({
                "domain": domain.clone(),
                "index": index,
            }),
            Self::Set(items) => {
                serde_json::Value::Array(items.iter().cloned().map(Into::into).collect())
            }
        }
    }

    /// The members, if this is a set. Used by the tag filter and the keyed expansion.
    #[must_use]
    pub const fn as_set(&self) -> Option<&BTreeSet<String>> {
        match self {
            Self::Set(s) => Some(s),
            _ => None,
        }
    }
}

/// One recorded artifact: its body digest, if it exists, and its `derived_from` map.
///
/// **A vouched root is `derived_from = []`** — an empty map, which is what
/// [`derived_from_entries`](Self::derived_from_entries) reports. It is *not* how step 3 decides
/// the vouched-root path: the registered kind's `source` is (§7b.9(6)).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactState {
    instance: Instance,
    body: Option<Digest>,
    derived_from: BTreeMap<String, Digest>,
}

impl ArtifactState {
    /// A not-yet-materialised artifact: it does not exist and records nothing.
    #[must_use]
    pub fn new(instance: &Instance) -> Self {
        Self {
            instance: instance.clone(),
            body: None,
            derived_from: BTreeMap::new(),
        }
    }

    /// The artifact exists on disk with this body digest.
    #[must_use]
    pub fn exists(mut self, body: Digest) -> Self {
        self.body = Some(body);
        self
    }

    /// Record one `derived_from` entry: the digest of `input` **at the time this artifact was
    /// materialised**.
    #[must_use]
    pub fn derived_from(mut self, input: &str, digest: Digest) -> Self {
        self.derived_from.insert(input.to_owned(), digest);
        self
    }

    /// The recorded `derived_from` map. Empty for a vouched root.
    #[must_use]
    pub const fn derived_from_entries(&self) -> &BTreeMap<String, Digest> {
        &self.derived_from
    }

    /// The instance this state is for.
    #[must_use]
    pub const fn instance(&self) -> &Instance {
        &self.instance
    }

    /// `true` when the artifact exists.
    #[must_use]
    pub const fn is_materialised(&self) -> bool {
        self.body.is_some()
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "instance": self.instance.to_json(),
            "body": self.body.as_ref().map(|d| d.as_str().to_owned()),
            "derived_from": self
                .derived_from
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::from(v.as_str())))
                .collect::<serde_json::Map<_, _>>(),
        })
    }
}

/// The item selection runs over.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Item {
    id: String,
    kind: String,
    facets: BTreeMap<String, FacetValue>,
    /// toolchain key → the change kinds `Toolchain::classify` attributed to it.
    toolchains: BTreeMap<String, BTreeSet<String>>,
    rigor_overrides: BTreeMap<String, Rigor>,
    artifacts: BTreeMap<Instance, ArtifactState>,
    /// The current digest of each upstream artifact kind, as the world holds it now — the
    /// right-hand side of step 3's `derived_from` comparison.
    upstream: BTreeMap<String, Digest>,
}

impl Item {
    /// A bare item. `kind` is bound as the `kind` facet, because that is where every pack's
    /// `applies` and every activity's `kind:` tag reads it from.
    #[must_use]
    pub fn new(id: &str, kind: &str) -> Self {
        let mut facets = BTreeMap::new();
        facets.insert("kind".to_owned(), FacetValue::Str(kind.to_owned()));
        Self {
            id: id.to_owned(),
            kind: kind.to_owned(),
            facets,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// An explicit item-frontmatter rigor override for one stage. Design §4.4 row 1 puts "(logged)"
    /// inside the rule, and [`super::step1`] is what logs it.
    #[must_use]
    pub fn with_rigor_override(mut self, stage: &str, rigor: Rigor) -> Self {
        self.rigor_overrides.insert(stage.to_owned(), rigor);
        self
    }

    /// Bind a boolean facet whose evidence **was** computable.
    #[must_use]
    pub fn with_facet_bool(mut self, name: &str, value: bool) -> Self {
        self.facets.insert(name.to_owned(), FacetValue::Bool(value));
        self
    }

    /// Bind a facet to `null` — *"no evidence computed"*. ADR-0089: this never resolves to `false`.
    #[must_use]
    pub fn with_facet_null(mut self, name: &str) -> Self {
        self.facets.insert(name.to_owned(), FacetValue::Null);
        self
    }

    /// Bind a plain string facet — `kind`, `cos`, `stage`, `role`.
    #[must_use]
    pub fn with_facet_str(mut self, name: &str, value: &str) -> Self {
        self.facets
            .insert(name.to_owned(), FacetValue::Str(value.to_owned()));
        self
    }

    /// Bind an ordinal facet, carrying its declared domain.
    ///
    /// # Panics
    ///
    /// If `member` is not in `domain` — a value outside its declared domain is a fault, not a
    /// silent `false`.
    #[must_use]
    pub fn with_facet_ordinal(mut self, name: &str, domain: &[&str], member: &str) -> Self {
        let index = domain
            .iter()
            .position(|d| *d == member)
            .unwrap_or_else(|| panic!("{member:?} is not in the declared domain {domain:?}"));
        self.facets.insert(
            name.to_owned(),
            FacetValue::Ordinal {
                domain: domain.iter().map(|s| (*s).to_owned()).collect(),
                index,
            },
        );
        self
    }

    /// Bind a set-valued facet directly.
    ///
    /// For `toolchain` prefer [`Self::with_toolchain`], which also records the per-toolchain change
    /// kinds the keyed expansion narrows on.
    #[must_use]
    pub fn with_facet_set<I: IntoIterator<Item = S>, S: Into<String>>(
        mut self,
        name: &str,
        items: I,
    ) -> Self {
        self.facets.insert(
            name.to_owned(),
            FacetValue::Set(items.into_iter().map(Into::into).collect()),
        );
        self
    }

    /// Record one toolchain and the change kinds `Toolchain::classify` attributed to it.
    ///
    /// The item-level `toolchain` and `change` facets are recomputed as the unions, so an item
    /// built this way is also a correct un-narrowed context.
    #[must_use]
    pub fn with_toolchain<I: IntoIterator<Item = S>, S: Into<String>>(
        mut self,
        toolchain: &str,
        change_kinds: I,
    ) -> Self {
        self.toolchains.insert(
            toolchain.to_owned(),
            change_kinds.into_iter().map(Into::into).collect(),
        );
        let all_toolchains: BTreeSet<String> = self.toolchains.keys().cloned().collect();
        let all_changes: BTreeSet<String> = self
            .toolchains
            .values()
            .flat_map(|s| s.iter().cloned())
            .collect();
        self.facets
            .insert("toolchain".to_owned(), FacetValue::Set(all_toolchains));
        self.facets
            .insert("change".to_owned(), FacetValue::Set(all_changes));
        self
    }

    /// Record an artifact's state — its body digest and its `derived_from` map.
    #[must_use]
    pub fn with_artifact(mut self, state: ArtifactState) -> Self {
        self.artifacts.insert(state.instance.clone(), state);
        self
    }

    /// Record the **current** digest of an upstream artifact kind.
    #[must_use]
    pub fn with_upstream(mut self, kind: &str, digest: Digest) -> Self {
        self.upstream.insert(kind.to_owned(), digest);
        self
    }

    /// The toolchain keys, in byte order. The keyed expansion's domain (AC-049).
    pub fn toolchains(&self) -> impl Iterator<Item = &String> {
        self.toolchains.keys()
    }

    /// The change kinds attributed to one toolchain.
    #[must_use]
    pub fn change_kinds_of(&self, toolchain: &str) -> Option<&BTreeSet<String>> {
        self.toolchains.get(toolchain)
    }

    /// The item's own value for one facet.
    #[must_use]
    pub fn facet(&self, name: &str) -> Option<&FacetValue> {
        self.facets.get(name)
    }

    /// The rigor override for one stage, if the frontmatter set one.
    #[must_use]
    pub fn rigor_override(&self, stage: &str) -> Option<Rigor> {
        self.rigor_overrides.get(stage).copied()
    }

    #[must_use]
    pub(in crate::select) const fn rigor_overrides(&self) -> &BTreeMap<String, Rigor> {
        &self.rigor_overrides
    }

    /// The recorded state of one instance.
    #[must_use]
    pub fn artifact_state(&self, instance: &Instance) -> Option<ArtifactState> {
        self.artifacts.get(instance).cloned()
    }

    pub(in crate::select) fn upstream_digests(&self, kinds: &[&str]) -> Vec<(String, Digest)> {
        kinds
            .iter()
            .filter_map(|k| self.upstream.get(*k).map(|d| ((*k).to_owned(), d.clone())))
            .collect()
    }

    /// The context vector as the evaluator sees it, with no narrowing.
    #[must_use]
    pub fn context(&self) -> Context {
        let mut ctx = Context::new();
        for (name, value) in &self.facets {
            ctx.bind(name, value.to_predicate());
        }
        ctx
    }

    /// The context vector **narrowed to one instance of a keyed kind** (AC-049).
    ///
    /// `toolchain` becomes the singleton `{key}` and `change` becomes that toolchain's own change
    /// kinds. Every other facet is unchanged. For an un-keyed instance this is [`Self::context`].
    #[must_use]
    pub fn narrowed_context(&self, instance: &Instance) -> Context {
        let mut ctx = self.context();
        if let Some(key) = &instance.key {
            ctx.bind("toolchain", PredValue::set(std::iter::once(key.clone())));
            if let Some(kinds) = self.toolchains.get(key) {
                ctx.bind("change", PredValue::set(kinds.iter().cloned()));
            }
        }
        ctx
    }

    /// The facet vector narrowed the same way, for the tag filter and for the operands typed into
    /// [`super::Unsatisfiable`].
    #[must_use]
    pub fn narrowed_facets(&self, instance: &Instance) -> BTreeMap<String, FacetValue> {
        let mut out = self.facets.clone();
        if let Some(key) = &instance.key {
            out.insert(
                "toolchain".to_owned(),
                FacetValue::Set(std::iter::once(key.clone()).collect()),
            );
            if let Some(kinds) = self.toolchains.get(key) {
                out.insert("change".to_owned(), FacetValue::Set(kinds.clone()));
            }
        }
        out
    }

    /// The item as the golden log records it: the two halves of AC-041's tuple that live on the
    /// item, side by side.
    ///
    /// **Public because AC-042 needs it to be.** *"`redkiln replay` re-derives the DAG from the
    /// **logged inputs alone**"* — so the logged inputs must be sufficient to rebuild the item, and
    /// [`Item::from_recorded`] is the other half of that round trip. Nothing inside
    /// [`super::select`] calls either.
    #[must_use]
    pub fn recorded_json(&self) -> (serde_json::Value, serde_json::Value) {
        (self.context_json(), self.artifacts_json())
    }

    /// Rebuild an item from what [`Self::recorded_json`] wrote.
    ///
    /// # Errors
    ///
    /// A `String` naming the field that did not have the recorded shape. **Never a partial item:**
    /// a replay that silently dropped a facet would re-derive a different DAG and report the
    /// divergence as the run's fault rather than the reader's.
    pub fn from_recorded(
        context: &serde_json::Value,
        artifacts: &serde_json::Value,
    ) -> Result<Self, String> {
        let id = context["id"].as_str().ok_or("context.id is not a string")?;
        let kind = context["kind"]
            .as_str()
            .ok_or("context.kind is not a string")?;
        let mut item = Self::new(id, kind);

        if let Some(map) = context["facets"].as_object() {
            for (name, value) in map {
                item.facets.insert(name.clone(), facet_from_json(value)?);
            }
        }
        if let Some(map) = context["toolchains"].as_object() {
            for (name, value) in map {
                let kinds: BTreeSet<String> = value
                    .as_array()
                    .ok_or("context.toolchains.* is not an array")?
                    .iter()
                    .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                    .collect();
                item.toolchains.insert(name.clone(), kinds);
            }
        }
        if let Some(map) = context["rigor_overrides"].as_object() {
            for (stage, token) in map {
                let token = token
                    .as_str()
                    .ok_or("context.rigor_overrides.* is not a string")?;
                let rigor =
                    Rigor::from_token(token).ok_or_else(|| format!("{token:?} is not a rigor"))?;
                item.rigor_overrides.insert(stage.clone(), rigor);
            }
        }
        if let Some(rows) = artifacts["recorded"].as_array() {
            for row in rows {
                let instance = instance_from_json(&row["instance"])?;
                let mut state = ArtifactState::new(&instance);
                if let Some(body) = row["body"].as_str() {
                    state = state.exists(Digest::from_hex(body).map_err(|e| format!("{e:?}"))?);
                }
                if let Some(map) = row["derived_from"].as_object() {
                    for (input, digest) in map {
                        let digest = digest
                            .as_str()
                            .ok_or("recorded.derived_from.* is not a string")?;
                        state = state.derived_from(
                            input,
                            Digest::from_hex(digest).map_err(|e| format!("{e:?}"))?,
                        );
                    }
                }
                item = item.with_artifact(state);
            }
        }
        if let Some(map) = artifacts["upstream"].as_object() {
            for (kind, digest) in map {
                let digest = digest.as_str().ok_or("upstream.* is not a string")?;
                item = item.with_upstream(
                    kind,
                    Digest::from_hex(digest).map_err(|e| format!("{e:?}"))?,
                );
            }
        }
        Ok(item)
    }

    /// The **context digest** of AC-041's tuple: the facet vector alone.
    pub(in crate::select) fn context_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "kind": self.kind,
            "facets": self
                .facets
                .iter()
                .map(|(k, v)| (k.clone(), v.to_json()))
                .collect::<serde_json::Map<_, _>>(),
            "toolchains": self
                .toolchains
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        serde_json::Value::Array(
                            v.iter().cloned().map(serde_json::Value::from).collect(),
                        ),
                    )
                })
                .collect::<serde_json::Map<_, _>>(),
            "rigor_overrides": self
                .rigor_overrides
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::from(v.as_str())))
                .collect::<serde_json::Map<_, _>>(),
        })
    }

    /// The **artifact digests** of AC-041's tuple, kept separate from the context digest because
    /// the criterion names them separately.
    pub(in crate::select) fn artifacts_json(&self) -> serde_json::Value {
        serde_json::json!({
            "recorded": self
                .artifacts
                .values()
                .map(ArtifactState::to_json)
                .collect::<Vec<_>>(),
            "upstream": self
                .upstream
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::from(v.as_str())))
                .collect::<serde_json::Map<_, _>>(),
        })
    }
}

/// One [`FacetValue`] from what [`FacetValue::to_json`] wrote.
///
/// **The ordinal's `{ domain, index }` object is checked before the plain-object fallback**, and
/// there is no fallback: an object that is not an ordinal is an error rather than a `Null`, because
/// a silently-nulled facet is exactly the hidden input AC-042 exists to catch.
fn facet_from_json(value: &serde_json::Value) -> Result<FacetValue, String> {
    match value {
        serde_json::Value::Null => Ok(FacetValue::Null),
        serde_json::Value::Bool(b) => Ok(FacetValue::Bool(*b)),
        serde_json::Value::String(s) => Ok(FacetValue::Str(s.clone())),
        serde_json::Value::Number(n) => n
            .as_i64()
            .map(FacetValue::Int)
            .ok_or_else(|| format!("{n} is not an integer; the canonical form refuses floats")),
        serde_json::Value::Array(items) => Ok(FacetValue::Set(
            items
                .iter()
                .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                .collect(),
        )),
        serde_json::Value::Object(map) => {
            let domain: Vec<String> = map
                .get("domain")
                .and_then(serde_json::Value::as_array)
                .ok_or("an object facet must be an ordinal with a `domain`")?
                .iter()
                .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                .collect();
            let index = usize::try_from(
                map.get("index")
                    .and_then(serde_json::Value::as_u64)
                    .ok_or("an ordinal facet must carry an `index`")?,
            )
            .map_err(|e| format!("{e:?}"))?;
            Ok(FacetValue::Ordinal { domain, index })
        }
    }
}

/// One [`Instance`] from what [`Instance::to_json`] wrote.
///
/// `required_by` is **not** recorded and is not rebuilt: it is provenance about how an instance
/// entered the graph, the re-run derives it again, and a recorded one would be an input the
/// re-derivation does not need.
fn instance_from_json(value: &serde_json::Value) -> Result<Instance, String> {
    let kind = value["kind"]
        .as_str()
        .ok_or("instance.kind is not a string")?;
    Ok(match value["key"].as_str() {
        Some(key) => Instance::keyed(kind, key),
        None => Instance::of(kind),
    })
}
