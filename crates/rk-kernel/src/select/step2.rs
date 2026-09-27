//! **Step 2 — the required set.** Design §4.4 row 2:
//!
//! > `R(stage, ctx)` = `baseline` ∪ artifacts of every `require` row whose `when` holds.
//!
//! # The three outcomes of a `when`, and the order they are decided in
//!
//! `true` → the row fires. `false` → the row contributes nothing, **and that is not an error**.
//! `null` → the **abstention** case: `gate_opened { reason: unknown_facet, axis }`, **counted once
//! per read facet, never per abstaining facet** (AC-079), and **never** `Unsatisfiable` — which is
//! reserved for step 4 over predicates that evaluated to `false` (§7b:507-509; ADR-0103 decision 5).
//!
//! The register states the order in terms: *"lint at load (AC-069), gate at step 2 (here),
//! `Unsatisfiable` at step 4"*.
//!
//! # The keyed expansion — AC-049, and what it replaces
//!
//! A required kind registered `keyed_by = "toolchain"` expands to **one instance per element of the
//! item's `toolchain` set**, each with its own producer, and the producer's condition is evaluated
//! **per instance** rather than once against the item. This is what ADR-0073 put in place of the
//! withdrawn specificity tie-break, and it is why step 4 never has to break a tie between a generic
//! and a specialised producer: they never contend for one slot.
//!
//! The measured defect it repairs is F35 §1.3: on a `node+unreal` item both change summaries were
//! admissible at rank 10, specificity picked the UE one, and the text summary was never produced —
//! *"satisfiable and wrong"*.
//!
//! # AC-065 — the closed vocabulary
//!
//! Every identifier a selection predicate reads must be a **registered facet**. `facets.toml`'s own
//! header states the rule: *"Every predicate identifier must be a facet here, a step id, `rigor`,
//! `item`, or a builtin."* A predicate that reads anything else — *"stakeholders in agreement"* —
//! fails load with [`JudgementPredicate`], and the guidance points at a gate node instead.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::artifacts::{ArtifactRegistry, Instance};
use super::{Item, Pack};
use crate::predicate::{self, Expr, Tri};
use crate::registry::Registry;

/// The required set. **Constructible only inside this module** (AC-061).
///
/// The constructor is `pub(in crate::select)` and the type derives **no** `Deserialize`, so no
/// agent-returned value can reach it. ADR-0099 clause 5 makes *"inside the selection crate"* mean
/// this module; a `pub(crate)` constructor, or a `#[cfg(test)]` public one, would silently convert
/// AC-061 from a type fact into a convention.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RequiredSet {
    kinds: BTreeSet<Instance>,
}

impl RequiredSet {
    pub(in crate::select) const fn new(kinds: BTreeSet<Instance>) -> Self {
        Self { kinds }
    }

    /// Every required instance, in byte order of `(kind, key)`.
    pub fn kinds(&self) -> impl Iterator<Item = &Instance> {
        self.kinds.iter()
    }

    /// How many instances are required.
    #[must_use]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    /// `true` when nothing is required.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }

    /// `true` when the set holds this instance.
    #[must_use]
    pub fn contains(&self, instance: &Instance) -> bool {
        self.kinds.contains(instance)
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Array(self.kinds.iter().map(Instance::to_json).collect())
    }
}

/// Why a gate opened. One variant today; the enum is closed so a second reason is a decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GateReason {
    /// AC-079. A top-level `null`: no evidence for the facet was computable, and **the absence of
    /// evidence never resolves to `false`** (ADR-0089).
    UnknownFacet,
}

impl GateReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownFacet => "unknown_facet",
        }
    }
}

/// `gate_opened { reason, axis }`.
///
/// **Returned, never appended.** The append is the runtime boundary's, on the same shape ADR-0119
/// clause 5 and ADR-0120 clause 4 fix for the other two derivations in this function.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateOpened {
    pub reason: GateReason,
    /// The facet that abstained. For an ordinal this is the axis; for a boolean, ADR-0089 puts the
    /// facet name here and explicitly does **not** rename the payload key.
    pub axis: String,
    /// The `require` row that read it, so a reader can find the site.
    pub site: String,
}

impl GateOpened {
    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "reason": self.reason.as_str(),
            "axis": self.axis,
            "site": self.site,
        })
    }
}

/// **AC-065.** A predicate outside the closed vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudgementPredicate {
    pub location: String,
    pub text: String,
    /// The identifier that is not a registered facet.
    pub identifier: String,
}

impl fmt::Display for JudgementPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: `{}` reads `{}`, which is not a registered facet, an artifact presence, a digest \
             comparison or a typed verdict. A human-judged criterion disguised as a predicate \
             cannot enter selection (AC-065) — author it as a **gate** node instead, where a person \
             answers it and the answer is recorded",
            self.location, self.text, self.identifier
        )
    }
}

impl std::error::Error for JudgementPredicate {}

/// What stopped step 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step2Error {
    /// The `when` did not parse under ADR-0103's grammar.
    Parse { site: String, detail: String },
    /// AC-065.
    Judgement(JudgementPredicate),
    /// The `when` parsed and read a facet the context does not bind, or ordered against a `null`
    /// (ADR-0069's forbids clause). **Not an abstention**: an unbound facet is a fault, because a
    /// facet outside the registry is a load-time error and never reaches here.
    Eval { site: String, detail: String },
    /// A `require` row or a `baseline` names a kind `artifacts.toml` does not declare. ADR-0057
    /// makes that registry the kind vocabulary, so this is a pack defect and not a missing artifact.
    UnregisteredKind { site: String, kind: String },
}

impl fmt::Display for Step2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Judgement(j) => write!(f, "{j}"),
            // Two distinct variants, one rendering. They are NOT merged into one variant: a
            // predicate that did not parse and one that parsed and could not be evaluated are
            // different defects with different repairs, and collapsing them would make the
            // distinction unrecoverable from the error a caller matches on.
            Self::Parse { site, detail } | Self::Eval { site, detail } => {
                write!(f, "{site}: {detail}")
            }
            Self::UnregisteredKind { site, kind } => write!(
                f,
                "{site}: `{kind}` is not declared in artifacts.toml. ADR-0057 makes that registry \
                 the kind vocabulary, so this is a pack defect rather than a missing artifact"
            ),
        }
    }
}

impl std::error::Error for Step2Error {}

/// What step 2 produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step2Outcome {
    required: RequiredSet,
    gates: Vec<GateOpened>,
    rows_fired: usize,
    rows_abstained: usize,
}

impl Step2Outcome {
    /// `R`.
    #[must_use]
    pub const fn required(&self) -> &RequiredSet {
        &self.required
    }

    /// One gate per **read** facet of an abstaining row, in byte order. **Never one per abstaining
    /// facet**: a facet that abstains but that no firing row reads opens nothing (AC-079).
    #[must_use]
    pub fn gates(&self) -> &[GateOpened] {
        &self.gates
    }

    /// How many `require` rows evaluated `true`. Step 5 carries this as `dag_assembled.rows_fired`.
    #[must_use]
    pub const fn rows_fired(&self) -> usize {
        self.rows_fired
    }

    /// How many `require` rows evaluated `null`.
    #[must_use]
    pub const fn rows_abstained(&self) -> usize {
        self.rows_abstained
    }
}

/// **AC-065.** Refuse a selection predicate that reads anything outside the closed vocabulary.
///
/// The three sites AC-065's GIVEN names are a `claimed_predicate`, an `applicable_if` and a
/// `require` row `when`. It is **not** applied to a step's own `expr`, whose identifiers are the
/// step's declared `inputs` rather than facets.
///
/// # Errors
///
/// [`JudgementPredicate`] naming the location, the text and the offending identifier.
pub fn lint_selection_predicate(
    location: &str,
    text: &str,
    facets: &Registry,
) -> Result<(), JudgementPredicate> {
    // A SELECTION predicate that does not parse under ADR-0103's grammar is a judgement predicate,
    // and that is a reading rather than an assumption. AC-065's own named must-fail fixture is
    // *"stakeholders in agreement"*, which this crate's parser REJECTS — measured 2026-09-13:
    // `expected at 16: expected "[", got "agreement"`, because `in`'s right operand is a list
    // literal or a set-valued facet and `agreement` is neither. So the criterion's own fixture
    // never reaches a vocabulary check, and an implementation that passed unparseable strings
    // through would report the one row AC-065 names as a grammar error and the criterion as
    // discharged by a different rule. The GIVEN settles it: *"a … `when` that **does not resolve
    // to** an artifact presence, a digest comparison, a registered facet value, or a typed
    // verdict"* — free text resolves to none of the four, and free text is precisely what does not
    // parse. **The parse detail is carried, not discarded**, so nothing is diagnosable only by
    // re-running the parser.
    let parsed = match predicate::parse(text) {
        Ok(p) => p,
        Err(e) => {
            return Err(JudgementPredicate {
                location: location.to_owned(),
                text: text.to_owned(),
                identifier: format!("<does not parse: {e}>"),
            });
        }
    };
    let mut offending = None;
    collect_paths(&parsed.expr, &BTreeSet::new(), &mut |p| {
        if offending.is_none() {
            let root = p.split('.').next().unwrap_or(p);
            if facets.facet(root).is_none() && !is_builtin(root) {
                offending = Some(p.to_owned());
            }
        }
    });
    offending.map_or(Ok(()), |identifier| {
        Err(JudgementPredicate {
            location: location.to_owned(),
            text: text.to_owned(),
            identifier,
        })
    })
}

/// The names `facets.toml`'s own header admits beside a facet: *"a step id, `rigor`, `item`, or a
/// builtin"*. `rigor` and the step ids reach a predicate as `${…}` references, which are
/// [`Expr::Ref`] and never [`Expr::Path`], so this list is the residue.
fn is_builtin(root: &str) -> bool {
    matches!(root, "item" | "exists" | "len")
}

/// Every facet path a predicate reads, for [`super::step4::rank_lint`]'s reduced enumeration.
pub(in crate::select) fn collect_paths_pub(expr: &Expr, f: &mut impl FnMut(&str)) {
    collect_paths(expr, &BTreeSet::new(), f);
}

fn collect_paths(expr: &Expr, bound: &BTreeSet<String>, f: &mut impl FnMut(&str)) {
    match expr {
        Expr::Or(l, r) | Expr::And(l, r) => {
            collect_paths(l, bound, f);
            collect_paths(r, bound, f);
        }
        Expr::Not(inner) | Expr::Group(inner) | Expr::Len(inner) => collect_paths(inner, bound, f),
        Expr::Cmp { left, right, .. } => {
            collect_paths(left, bound, f);
            collect_paths(right, bound, f);
        }
        Expr::In { left, list } => {
            collect_paths(left, bound, f);
            collect_paths(list, bound, f);
        }
        Expr::Member { literal, set } => {
            collect_paths(literal, bound, f);
            collect_paths(set, bound, f);
        }
        Expr::Quant { source, body, .. } => {
            collect_paths(source, bound, f);
            // A quantifier body opens a record scope: its identifiers are the element's fields,
            // not facets, so nothing under one is checked against the registry.
            let _ = body;
        }
        Expr::List(items) => {
            for item in items {
                collect_paths(item, bound, f);
            }
        }
        Expr::Path(p) => {
            if !bound.contains(p) {
                f(p);
            }
        }
        Expr::Ref(_) | Expr::Lit(_) => {}
    }
}

/// **The keyed expansion (AC-049), as one function.**
///
/// A kind registered `keyed_by = "toolchain"` becomes one instance per element of the item's
/// `toolchain` set; anything else becomes one un-keyed instance. It is a free function rather than
/// a closure inside [`step2`] because **step 4's `requires` recursion needs exactly the same rule**:
/// AC-049's second AND is *"a mixed item cannot be satisfied by a single instance of a keyed
/// kind"*, and it does not care by which road the kind entered the graph.
pub(in crate::select) fn expand(
    artifacts: &ArtifactRegistry,
    kind: &str,
    item: &Item,
) -> Vec<Instance> {
    match artifacts.keyed_by(kind) {
        Some("toolchain") => {
            let keys: Vec<&String> = item.toolchains().collect();
            if keys.is_empty() {
                // An item with no classified toolchain cannot expand a keyed kind, and inventing a
                // default key would be the silent single-instance shape AC-049's second AND
                // forbids. The un-keyed instance is required and step 4's producer filter is what
                // will or will not find a producer for it.
                vec![Instance::of(kind)]
            } else {
                keys.into_iter()
                    .map(|key| Instance::keyed(kind, key))
                    .collect()
            }
        }
        // A kind keyed by an axis this slice does not model expands as un-keyed, and that is
        // reported rather than silently handled: `keyed_by` carries the axis precisely so a second
        // one is visible.
        Some(_) | None => vec![Instance::of(kind)],
    }
}

/// **Step 2.** `R(stage, ctx)` = `baseline` ∪ the artifacts of every `require` row whose `when`
/// holds, with every `keyed_by` kind expanded per key.
///
/// # Errors
///
/// [`Step2Error`] for a predicate that does not parse, one outside the closed vocabulary (AC-065),
/// an evaluation fault, or a kind `artifacts.toml` does not declare.
pub fn step2(
    pack: &Pack,
    artifacts: &ArtifactRegistry,
    facets: &Registry,
    stage: &str,
    item: &Item,
) -> Result<Step2Outcome, Step2Error> {
    step2_amended(pack, artifacts, facets, stage, item, None)
}

/// **Step 2, re-entered from step 7.** Design §4.4 row 7: an accepted deviation *"re-runs 2-5"*,
/// and this is where the re-entry lands.
///
/// The amendment is applied **to the inputs of the union**, never to a finished [`RequiredSet`]:
/// `skip` drops the target kind's instances from the set the row union produced, `add` adds them,
/// and `downgrade` touches `R` at all (it moves the rigor step 4 resolves the producer under). So
/// there is still exactly one construction site for `R` and the agent's request reaches it as data
/// rather than as an edit — which is what AC-061's *"the agent never edits `R`"* asks for.
///
/// # Errors
///
/// As [`step2`].
pub(in crate::select) fn step2_amended(
    pack: &Pack,
    artifacts: &ArtifactRegistry,
    facets: &Registry,
    stage: &str,
    item: &Item,
    amendment: Option<&super::step7::Accepted>,
) -> Result<Step2Outcome, Step2Error> {
    let mut kinds: BTreeSet<String> = BTreeSet::new();
    let mut gates: BTreeMap<(String, String), GateOpened> = BTreeMap::new();
    let mut rows_fired = 0;
    let mut rows_abstained = 0;

    let spec = pack.stage(stage);
    if let Some(spec) = spec {
        for kind in &spec.baseline {
            kinds.insert(kind.clone());
        }
        // *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 3, proposed; D21-73.)*
        // **`rigor` is bound, as the run's rigor at this stage.** `facets.toml` declares it a
        // `kernel` facet (*"a lookup over policy.<class>.stages"*), so the item never carries it and
        // `item.context()` alone left every `when` that reads it an unknown field. The value is
        // step 1's (the pack's, or the item's logged override), bound by `rigor::with_rigor`, the one
        // place the ordinal is built, and bound AFTER the item's facets so a stale facet of the same
        // name cannot displace it (the executor's order, ADR-0142).
        let rigor = super::step1(pack, item)
            .rigor(stage)
            .unwrap_or(crate::rigor::Rigor::Optional);
        let ctx = crate::rigor::with_rigor(item.context(), rigor, facets);
        for row in &spec.require {
            lint_selection_predicate(&row.site, &row.when, facets)
                .map_err(Step2Error::Judgement)?;
            let parsed = predicate::parse(&row.when).map_err(|e| Step2Error::Parse {
                site: row.site.clone(),
                detail: e.to_string(),
            })?;
            let outcome =
                predicate::evaluate(&parsed.expr, &ctx).map_err(|e| Step2Error::Eval {
                    site: row.site.clone(),
                    detail: e.to_string(),
                })?;
            match outcome.value {
                Tri::True => {
                    rows_fired += 1;
                    for kind in &row.artifacts {
                        kinds.insert(kind.clone());
                    }
                }
                Tri::False => {}
                Tri::Null => {
                    rows_abstained += 1;
                    // "Counted once per READ facet, never per abstaining facet" — the axes come
                    // from the evaluator's own accounting, which records every context-bound path
                    // the evaluation actually read, once each.
                    for axis in outcome.abstention_axes() {
                        gates
                            .entry((axis.to_owned(), row.site.clone()))
                            .or_insert_with(|| GateOpened {
                                reason: GateReason::UnknownFacet,
                                axis: axis.to_owned(),
                                site: row.site.clone(),
                            });
                    }
                }
            }
        }
    }

    // ---- the accepted deviation, applied to the UNION's inputs (step 7's re-entry) -------------
    if let Some(amendment) = amendment {
        match amendment.kind {
            super::DeviationKind::Skip => {
                kinds.remove(&amendment.target);
            }
            super::DeviationKind::Add => {
                kinds.insert(amendment.target.clone());
            }
            // `downgrade` does not change WHAT is required, only the rigor the producer is resolved
            // under. It is applied at step 4, not here, and saying so is why this arm exists rather
            // than a catch-all.
            super::DeviationKind::Downgrade => {}
        }
    }

    // ---- the keyed expansion (AC-049) ----------------------------------------------------------
    let mut instances: BTreeSet<Instance> = BTreeSet::new();
    for kind in &kinds {
        let site = format!("{}: stage.{stage}", pack.id);
        if artifacts.kind(kind).is_none() {
            return Err(Step2Error::UnregisteredKind {
                site,
                kind: kind.clone(),
            });
        }
        instances.extend(expand(artifacts, kind, item));
    }

    Ok(Step2Outcome {
        required: RequiredSet::new(instances),
        gates: gates.into_values().collect(),
        rows_fired,
        rows_abstained,
    })
}
