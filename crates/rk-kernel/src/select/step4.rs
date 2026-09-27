//! **Step 4 — producer.** Design §4.4 row 4:
//!
//! > Candidates = activities with the kind in `provides`, `applicable_if` true (evaluated over the
//! > item's context vector at selection, never re-evaluated inside the activity — ADR-0103
//! > decision 6), `min_rigor ≤ r ≤ max_rigor` where an absent ceiling is no upper bound (ADR-0056),
//! > `stage` equal to the requiring stage; **lowest `method_rank` wins**; recurse on the producer's
//! > `requires`.
//!
//! # The rank rule is FORCED, and there is no tie-break
//!
//! **A tie is a pack validation error, never a runtime choice** (SHOP2 ordered methods; ADR-0056).
//! [`rank_lint`] is the load-time instrument: it enumerates the contexts each kind's producer set
//! can distinguish and fails with [`LoadError::RankTie`] on the first equal-rank pair that is
//! simultaneously admissible. **Specificity is withdrawn** from producer-of-a-kind selection
//! (ADR-0073:24; design row 1c, *"no specificity tie-break is consulted for the mixed case"*).
//! AC-047's *Cut* removes the absence claim as a **criterion**, not as the design, and is not
//! licence to build a tie-break.
//!
//! **The mixed-toolchain case never reaches this rule.** It is resolved earlier, at step 2, by
//! keyed expansion: one instance per element of the item's `toolchain` set, one producer per
//! instance, with the context **narrowed per instance** so a generic and a specialised producer are
//! never simultaneously admissible for one slot.
//!
//! **Two specificity rules exist in this crate's problem space and must not become one code path**
//! (§7b.9(1)). The binder's subsumption rule over agent-card selectors survives (AC-087); producer
//! specificity does not (AC-088). Nothing here ranks by specificity at all, so there is no shared
//! path to scope — the anti-tie-break is structural rather than a lint that must be kept narrow.
//!
//! # Unsatisfiability is DATA, not an abort — §7b.9(4)
//!
//! [`step4`] returns `Ok` with the kind gated. AC-058's gate *"carr[ies] the DAG"* and its
//! byte-identity test needs an assembled `R`, so nothing is dropped from `R` (AC-054). **No
//! fallback**: no next-best rank, no relaxation of `min_rigor`, no substitute kind. There is no
//! default arm in the candidate filter — every rejection has a named reason.
//!
//! And the **operands are typed into the variant**. AC-055 requires *"the evaluated value of each
//! facet that predicate read"*, AC-063 requires *"its evaluated operands attached"*, and
//! re-deriving them at gate-write time is a **second** evaluation of a three-valued grammar over a
//! context that may have been re-expanded — which AC-055's *"rejected by the store's single write
//! path"* would then turn into a runtime write failure rather than a test failure.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::Item;
use super::artifacts::{ArtifactRegistry, Instance};
use super::item::FacetValue;
use super::step1::admits;
use crate::catalog::{Activity, Catalog, Requirement};
use crate::predicate::{self, Tri};
use crate::registry::Registry;
use crate::rigor::Rigor;

/// One admissible producer.
///
/// 🛑 **The derived `Ord` is NOT the rank rule.** It orders by `activity_id` first (field order), so
/// sorting candidates by it picks the alphabetically first producer. Step 4 sorts by
/// `(method_rank, activity_id)` explicitly (R8 = ADR-0179 decision 1, proposed; D21-73).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate {
    pub activity_id: String,
    pub method_rank: i64,
}

/// A producer whose `applicable_if` evaluated `null`.
///
/// **Not a candidate and not a failure.** AC-079's third AND fixes the order: lint at load
/// (AC-069), gate at step 2, `Unsatisfiable` at step 4 — *"never raised for a `null` read"*. An
/// abstention here is routed the way step 2 routes one and is never folded into
/// [`Unsatisfiable::failed_predicates`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstention {
    pub activity_id: String,
    pub text: String,
    pub axes: Vec<String>,
}

/// One predicate that evaluated **false**, with the values it read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedPredicate {
    pub activity_id: String,
    /// The predicate, verbatim.
    pub text: String,
    /// Always [`Tri::False`] — [`Unsatisfiable`] is reserved for predicates that evaluated false.
    /// Carried explicitly so a reader of a written gate record does not have to take it on trust.
    pub value: Tri,
    /// **The evaluated value of each facet the predicate read**, typed in rather than re-derived
    /// (AC-055; §7b.9(4)). A `null` here is recorded as an abstention with the rule that produced
    /// it and is never coerced to `false` (ADR-0089).
    pub operands: Vec<(String, FacetValue)>,
}

/// Why a kind could not be produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnsatReason {
    /// Every candidate's `applicable_if` evaluated false, or the kind has no producer at all.
    NoAdmissibleProducer,
    /// The kind's only producers live in a stage the pack sets to `forbidden` (ADR-0057's
    /// Decision; AC-077). This one is reached through a `requires` edge and owes a skip row too —
    /// **both, not either** (§7b.9(7)).
    ForbiddenStage,
}

impl UnsatReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAdmissibleProducer => "no_admissible_producer",
            Self::ForbiddenStage => "forbidden_stage",
        }
    }
}

/// **AC-054.** A required kind with no admissible producer, gated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unsatisfiable {
    pub kind: String,
    pub failed_predicates: Vec<FailedPredicate>,
    /// The rigor in force, which AC-055 requires the gate record to name.
    pub rigor: Rigor,
    pub reason: UnsatReason,
}

impl Unsatisfiable {
    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "kind": self.kind,
            "rigor": self.rigor.as_str(),
            "reason": self.reason.as_str(),
            "failed_predicates": self.failed_predicates.iter().map(|p| serde_json::json!({
                "activity_id": p.activity_id,
                "text": p.text,
                "value": match p.value { Tri::True => "true", Tri::False => "false", Tri::Null => "null" },
                "operands": p.operands.iter().map(|(n, v)| serde_json::json!({
                    "facet": n,
                    "value": v.to_json(),
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
        })
    }
}

/// What step 4 produced for one instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picked {
    chosen: Option<Candidate>,
    beaten: Vec<Candidate>,
    unsatisfiable: Option<Unsatisfiable>,
    abstained: Vec<Abstention>,
}

impl Picked {
    /// The producer with the lowest `method_rank`, if one was admissible.
    #[must_use]
    pub const fn chosen(&self) -> Option<&Candidate> {
        self.chosen.as_ref()
    }

    /// **AC-046's second clause**: every candidate the winner beat, with its rank. A selection
    /// record that named only the winner would not be checkable.
    #[must_use]
    pub fn beaten(&self) -> &[Candidate] {
        &self.beaten
    }

    /// The gate, when nothing was admissible.
    #[must_use]
    pub const fn unsatisfiable(&self) -> Option<&Unsatisfiable> {
        self.unsatisfiable.as_ref()
    }

    /// The producers that abstained. AC-079's route, not AC-054's.
    #[must_use]
    pub fn abstained(&self) -> &[Abstention] {
        &self.abstained
    }
}

/// A pack defect found at load, or re-raised at the moment a runtime path would have had to guess.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// **AC-047.** Two producers of one kind hold the same `method_rank` and are simultaneously
    /// admissible in some enumerated context.
    RankTie {
        kind: String,
        /// The context the tie was found in, rendered as its bindings in byte order so a reader can
        /// reproduce it.
        context: String,
        /// Both activity ids, in byte order.
        activities: Vec<String>,
    },
    /// A predicate on an activity that did not parse or could not be evaluated.
    Predicate { activity_id: String, detail: String },
    /// **AC-059.** The catalog's `requires` relation contains a cycle, named as a path of activity
    /// ids. [`super::requires_cycle_lint`] is the instrument; **AC-078 is the separate RUNTIME
    /// assertion** over an assembled graph and lives in [`super::step5`].
    RequiresCycle { path: Vec<String> },
    /// **AC-060.** `A` declares `excludes = [B]` while `B` does not declare `excludes = [A]`.
    AsymmetricExcludes { a: String, b: String },
    /// **AC-053's load half.** A `requires` naming a kind that is neither in `artifacts.toml` nor
    /// provided by any activity.
    DanglingRequires { activity_id: String, kind: String },
    /// **The forward-ref licence** (D21-48 X8; R2 = ADR-0173, written `proposed`; `s8d` §8d.3). A
    /// step reads a later-declared step under none of the three licences, or without `?`. Every
    /// refused ref is carried, each naming its activity, step and reference
    /// ([`crate::catalog::forward_ref_lint`]).
    UnlicensedForwardRef {
        refs: Vec<crate::catalog::ForwardRef>,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RankTie {
                kind,
                context,
                activities,
            } => write!(
                f,
                "RankTie {{ kind: {kind:?}, context: {context:?}, activities: {activities:?} }} — \
                 two producers of one kind hold the same `method_rank` and are simultaneously \
                 admissible. There is NO runtime tie-break: not specificity (withdrawn for \
                 producer-of-a-kind selection, ADR-0073), not declaration order, not file order, \
                 not name. Change one rank, or narrow one activity's tags or `applicable_if` so \
                 they are never both admissible (ADR-0056; AC-047)"
            ),
            Self::Predicate {
                activity_id,
                detail,
            } => write!(f, "{activity_id}: {detail}"),
            Self::RequiresCycle { path } => write!(
                f,
                "the catalog's `requires` relation contains a cycle: {} — La Rosa's                  contradiction/cycle check, and F35 measured hard cycles 0 across the enumerated                  lattice, so this is an authored defect rather than a found one (AC-059)",
                path.join(" -> ")
            ),
            Self::AsymmetricExcludes { a, b } => write!(
                f,
                "AsymmetricExcludes {{ a: {a:?}, b: {b:?} }} — `{a}` declares `excludes = [{b}]`                  and `{b}` does not declare `excludes = [{a}]`. `excludes` are SYMMETRIC (catalog                  §2.6); a one-way exclusion means the pair's behaviour depends on which of the two                  the assembler reaches first (AC-060)"
            ),
            Self::UnlicensedForwardRef { refs } => {
                write!(
                    f,
                    "{} forward ref(s) to a later-declared step are licensed by none of ADR-0042's \
                     letter, the repeat body (D21-48 X6) and the escalate re-entry (D21-48 X8), or \
                     carry no `?`; each is a lint error, fixed in the catalog (D21-48 X8):",
                    refs.len()
                )?;
                for r in refs {
                    write!(f, "\n  {r}")?;
                }
                Ok(())
            }
            Self::DanglingRequires { activity_id, kind } => write!(
                f,
                "{activity_id}: `requires` names `{kind}`, which no activity provides and                  `artifacts.toml` does not declare. ADR-0057 makes that registry the kind                  vocabulary, so the edge binds to nothing and this is a load failure (AC-053)"
            ),
        }
    }
}

impl std::error::Error for LoadError {}

/// Every stage in which some activity `provides` this kind.
#[must_use]
pub fn producing_stages(catalog: &Catalog, kind: &str) -> Vec<String> {
    let mut out: Vec<String> = catalog
        .activities()
        .iter()
        .filter(|a| a.provides.iter().any(|p| p == kind))
        .map(|a| a.stage.clone())
        .collect();
    out.sort();
    out.dedup();
    out
}

/// One activity's `requires`, for design §4.4 row 4's *"recurse on the producer's `requires`"*.
#[must_use]
pub fn requires_of(catalog: &Catalog, activity_id: &str) -> Vec<Requirement> {
    catalog
        .activities()
        .iter()
        .find(|a| a.id == activity_id)
        .map(|a| a.requires.clone())
        .unwrap_or_default()
}

/// Why one activity was not a candidate. **There is no `_ =>` arm anywhere below**: every rejection
/// is one of these, and AC-054's exhaustiveness half is what that is for.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Rejection {
    WrongStage,
    DoesNotProvide,
    OutsideRigorBand,
    TagsDoNotMatch,
    PredicateFalse(FailedPredicate),
    PredicateNull(Abstention),
}

/// Decide one activity's candidacy for one instance.
fn judge(
    activity: &Activity,
    stage: &str,
    rigor: Rigor,
    instance: &Instance,
    item: &Item,
) -> Result<Result<Candidate, Rejection>, LoadError> {
    if activity.stage != stage {
        return Ok(Err(Rejection::WrongStage));
    }
    if !activity.provides.contains(&instance.kind) {
        return Ok(Err(Rejection::DoesNotProvide));
    }
    let min = Rigor::from_token(&activity.min_rigor).unwrap_or(Rigor::Optional);
    let max = activity.max_rigor.as_deref().and_then(Rigor::from_token);
    if !admits(rigor, min, max) {
        return Ok(Err(Rejection::OutsideRigorBand));
    }

    let facets = item.narrowed_facets(instance);
    if !tags_match(activity, &facets) {
        return Ok(Err(Rejection::TagsDoNotMatch));
    }

    let Some(text) = activity.applicable_if.as_deref() else {
        return Ok(Ok(Candidate {
            activity_id: activity.id.clone(),
            method_rank: activity.method_rank,
        }));
    };
    let parsed = predicate::parse(text).map_err(|e| LoadError::Predicate {
        activity_id: activity.id.clone(),
        detail: e.to_string(),
    })?;
    // ADR-0103 decision 6: evaluated over the item's context vector AT SELECTION, and for a keyed
    // instance over the NARROWED one (AC-049's "evaluated per instance rather than once against
    // the item").
    let ctx = item.narrowed_context(instance);
    let outcome = predicate::evaluate(&parsed.expr, &ctx).map_err(|e| LoadError::Predicate {
        activity_id: activity.id.clone(),
        detail: e.to_string(),
    })?;
    match outcome.value {
        Tri::True => Ok(Ok(Candidate {
            activity_id: activity.id.clone(),
            method_rank: activity.method_rank,
        })),
        Tri::False => Ok(Err(Rejection::PredicateFalse(FailedPredicate {
            activity_id: activity.id.clone(),
            text: text.to_owned(),
            value: Tri::False,
            operands: outcome
                .read_facets
                .iter()
                .map(|name| {
                    (
                        name.clone(),
                        facets.get(name).cloned().unwrap_or(FacetValue::Null),
                    )
                })
                .collect(),
        }))),
        Tri::Null => Ok(Err(Rejection::PredicateNull(Abstention {
            activity_id: activity.id.clone(),
            text: text.to_owned(),
            axes: outcome
                .abstention_axes()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        }))),
    }
}

/// An activity's `tags` are `axis:value` constraints, grouped by axis.
///
/// `spikes/p9/catalog/change-summary-ue.toml`:9 is the shipped instance — `tags = [… ,
/// "toolchain:unreal"]` with no `applicable_if` at all, while the generic `change-summary.toml`
/// carries no `toolchain:` tag and discriminates on `change`. An axis the activity does not tag is
/// unconstrained; an axis it does tag must intersect the item's value for that axis.
fn tags_match(activity: &Activity, facets: &BTreeMap<String, FacetValue>) -> bool {
    let mut by_axis: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for tag in &activity.tags {
        if let Some((axis, value)) = tag.split_once(':') {
            by_axis.entry(axis).or_default().insert(value);
        }
    }
    by_axis.iter().all(|(axis, allowed)| {
        match facets.get(*axis) {
            Some(FacetValue::Str(s)) => allowed.contains(s.as_str()),
            Some(FacetValue::Set(members)) => members.iter().any(|m| allowed.contains(m.as_str())),
            // An axis the item does not bind cannot satisfy a constraint on it, and defaulting to
            // "matches" would let a `toolchain:unreal` activity produce for an item with no
            // classified toolchain at all.
            Some(FacetValue::Null | FacetValue::Bool(_) | FacetValue::Int(_))
            | Some(FacetValue::Ordinal { .. })
            | None => false,
        }
    })
}

/// **Step 4** for one required instance.
///
/// # Errors
///
/// [`LoadError::RankTie`] if two admissible producers share the lowest rank — **the runtime path
/// re-raises the load error rather than tie-breaking**, so a catalog that somehow reached here
/// without [`rank_lint`] having run still cannot be resolved by guessing.
/// [`LoadError::Predicate`] for an `applicable_if` that does not parse or evaluate.
pub fn step4(
    catalog: &Catalog,
    _artifacts: &ArtifactRegistry,
    _facets: &Registry,
    stage: &str,
    rigor: Rigor,
    instance: &Instance,
    item: &Item,
) -> Result<Picked, LoadError> {
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut failed: Vec<FailedPredicate> = Vec::new();
    let mut abstained: Vec<Abstention> = Vec::new();

    for activity in catalog.activities() {
        match judge(activity, stage, rigor, instance, item)? {
            Ok(c) => candidates.push(c),
            Err(Rejection::PredicateFalse(p)) => failed.push(p),
            Err(Rejection::PredicateNull(a)) => abstained.push(a),
            Err(
                Rejection::WrongStage
                | Rejection::DoesNotProvide
                | Rejection::OutsideRigorBand
                | Rejection::TagsDoNotMatch,
            ) => {}
        }
    }

    // *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 1, proposed; D21-73.)*
    // **By `method_rank`, never by the derived `Ord`.** `Candidate` derives `Ord` over its fields in
    // declaration order, `(activity_id, method_rank)`, so a bare `sort()` put the first id
    // ALPHABETICALLY at `candidates[0]` and step 4 chose it (the fix lane's `bisect`, rank 30, over
    // `root-cause`, rank 10). The id is the second key only so the `beaten` list is deterministic;
    // it never decides a winner, because an equal lowest rank refuses below as `RankTie`.
    candidates
        .sort_by(|a, b| (a.method_rank, &a.activity_id).cmp(&(b.method_rank, &b.activity_id)));
    if candidates.is_empty() {
        // An abstention is NOT unsatisfiability. AC-079: `Unsatisfiable` is reserved for predicates
        // that evaluated to `false`, never raised for a `null` read.
        let unsatisfiable = if abstained.is_empty() {
            Some(Unsatisfiable {
                kind: instance.kind.clone(),
                failed_predicates: failed,
                rigor,
                reason: UnsatReason::NoAdmissibleProducer,
            })
        } else {
            None
        };
        return Ok(Picked {
            chosen: None,
            beaten: Vec::new(),
            unsatisfiable,
            abstained,
        });
    }

    let lowest = candidates[0].method_rank;
    let tied: Vec<String> = candidates
        .iter()
        .filter(|c| c.method_rank == lowest)
        .map(|c| c.activity_id.clone())
        .collect();
    if tied.len() > 1 {
        return Err(LoadError::RankTie {
            kind: instance.kind.clone(),
            context: render_context(item, instance),
            activities: tied,
        });
    }

    let chosen = candidates.remove(0);
    Ok(Picked {
        chosen: Some(chosen),
        beaten: candidates,
        unsatisfiable: None,
        abstained,
    })
}

/// **AC-047's load-time instrument.**
///
/// > GIVEN a catalog in which two producers of one kind … hold the same `method_rank` and are
/// > simultaneously admissible **in any enumerated context**
/// > WHEN the pack is loaded, or `redkiln lint` runs in CI
/// > THEN load fails with `RankTie { kind, context, activities }` **before any activity is
/// > dispatched**.
///
/// # The enumeration, and why the reduction is exact
///
/// Per `(stage, kind)`, this enumerates the cross-product of **only the facets that kind's producer
/// set actually reads** — the union of the axes named in their `tags` and their `applicable_if`.
/// That is not a sample: two contexts that agree on every facet the producer set reads are
/// indistinguishable to its admissibility filter, so they admit the same producers and checking one
/// checks both. [`crate::rigor`] justifies the same reduction for ADR-0081 clause 1 and this is the
/// same argument over the same filter.
///
/// `toolchain` and `change` are enumerated **together**, over `spikes/p9/lattice.py`'s own four
/// instances, because they are not independent axes — a toolchain determines the change kinds it
/// produces.
///
/// **A keyed kind needs no special case here, and that was measured rather than assumed.** An
/// earlier revision skipped the mixed `node+unreal` instance for a `keyed_by = "toolchain"` kind,
/// on the argument that step 2 never presents a mixed context to a keyed kind's producer filter.
/// The argument is right and the branch was dead: `rank_lint` judges a keyed kind against
/// `Instance::keyed(kind, key)`, and [`Item::narrowed_facets`] then narrows `toolchain` to that
/// singleton and `change` to its own kinds — so the mixed instance collapses onto the `node`
/// singleton before any producer is judged. Deleting the branch changed no verdict on the fixture
/// pair that exercises it (`change-summary` / `change-summary-ue`, both rank 10, both providing
/// `ChangeSummary`), which is why it is deleted rather than kept as belt-and-braces: a branch that
/// reads as load-bearing and is not is worse than no branch.
///
/// # What this lint does NOT cover
///
/// AC-047's GIVEN also names *"two members of one declared family"*. Family membership is
/// ADR-0058's declared `family` field and its totality check is **AC-104's**, whose instrument is
/// `spikes/p9/lattice.py --validate`'s own output (register :825-827). This lint is the
/// producer-of-a-kind half only, and the family half is **not discharged here**.
///
/// # Errors
///
/// [`LoadError::RankTie`] on the first tie found, in a deterministic order: stage, then kind, then
/// rigor, then context.
pub fn rank_lint(
    catalog: &Catalog,
    artifacts: &ArtifactRegistry,
    facets: &Registry,
) -> Result<(), LoadError> {
    let mut pairs: BTreeSet<(String, String)> = BTreeSet::new();
    for a in catalog.activities() {
        for kind in &a.provides {
            pairs.insert((a.stage.clone(), kind.clone()));
        }
    }

    for (stage, kind) in pairs {
        let producers: Vec<&Activity> = catalog
            .activities()
            .iter()
            .filter(|a| a.stage == stage && a.provides.contains(&kind))
            .collect();
        if producers.len() < 2 {
            continue;
        }
        let keyed = artifacts.keyed_by(&kind) == Some("toolchain");
        for item in enumerate_contexts(&producers, facets) {
            for rigor in Rigor::LIVE {
                let instance = if keyed {
                    let key = item
                        .toolchains()
                        .next()
                        .cloned()
                        .unwrap_or_else(|| "node".to_owned());
                    Instance::keyed(&kind, &key)
                } else {
                    Instance::of(&kind)
                };
                let mut admitted: Vec<Candidate> = Vec::new();
                for a in &producers {
                    if let Ok(c) = judge(a, &stage, rigor, &instance, &item)? {
                        admitted.push(c);
                    }
                }
                admitted.sort();
                let mut by_rank: BTreeMap<i64, Vec<String>> = BTreeMap::new();
                for c in admitted {
                    by_rank
                        .entry(c.method_rank)
                        .or_default()
                        .push(c.activity_id);
                }
                if let Some((_, tied)) = by_rank.iter().find(|(_, ids)| ids.len() > 1) {
                    return Err(LoadError::RankTie {
                        kind: kind.clone(),
                        context: format!("{}, rigor={rigor}", render_context(&item, &instance)),
                        activities: tied.clone(),
                    });
                }
            }
        }
    }
    Ok(())
}

/// `lattice.py`'s four toolchain instances, with the change kinds each produces.
const TOOLCHAIN_INSTANCES: [(&str, &[&str]); 4] = [
    ("node", &["text"]),
    ("unreal", &["binary"]),
    ("dotnet", &["text"]),
    ("node+unreal", &["text", "binary"]),
];

/// The reduced context enumeration for one producer set.
fn enumerate_contexts(producers: &[&Activity], facets: &Registry) -> Vec<Item> {
    let mut axes: BTreeSet<String> = BTreeSet::new();
    for a in producers {
        for tag in &a.tags {
            if let Some((axis, _)) = tag.split_once(':') {
                axes.insert(axis.to_owned());
            }
        }
        if let Some(text) = &a.applicable_if
            && let Ok(parsed) = predicate::parse(text)
        {
            super::step2::collect_paths_pub(&parsed.expr, &mut |p| {
                axes.insert(p.split('.').next().unwrap_or(p).to_owned());
            });
        }
    }

    // `toolchain` and `change` are one axis pair, swept together over the four instances.
    let sweep_toolchain = axes.remove("toolchain") | axes.remove("change");
    let mut items: Vec<Item> = vec![Item::new("lint", "story")];

    if sweep_toolchain {
        let mut next = Vec::new();
        for base in &items {
            for (name, kinds) in TOOLCHAIN_INSTANCES {
                let mut it = base.clone();
                for part in name.split('+') {
                    let own: Vec<&str> = if name.contains('+') {
                        match part {
                            "unreal" => vec!["binary"],
                            _ => vec!["text"],
                        }
                    } else {
                        kinds.to_vec()
                    };
                    it = it.with_toolchain(part, own);
                }
                next.push(it);
            }
        }
        items = next;
    }

    for axis in &axes {
        let Some(facet) = facets.facet(axis) else {
            continue;
        };
        let mut next = Vec::new();
        for base in &items {
            for value in &facet.values {
                let mut it = base.clone();
                let Some(token) = value.as_str() else {
                    continue;
                };
                if facet.set {
                    it = it.with_facet_set(axis, [token]);
                } else if facet.ordered {
                    let domain: Vec<&str> =
                        facet.values.iter().filter_map(|v| v.as_str()).collect();
                    it = it.with_facet_ordinal(axis, &domain, token);
                } else if token == "true" || token == "false" {
                    it = it.with_facet_bool(axis, token == "true");
                } else {
                    it = it.with_facet_str(axis, token);
                }
                next.push(it);
            }
            if facet.nullable {
                next.push(base.clone().with_facet_null(axis));
            }
        }
        if !next.is_empty() {
            items = next;
        }
    }
    items
}

/// The context, rendered as its bindings in byte order, so a `RankTie` names something a reader can
/// reproduce rather than an opaque index.
fn render_context(item: &Item, instance: &Instance) -> String {
    let facets = item.narrowed_facets(instance);
    let mut parts: Vec<String> = Vec::new();
    for (name, value) in &facets {
        parts.push(format!("{name}={}", render_value(value)));
    }
    parts.join(", ")
}

fn render_value(value: &FacetValue) -> String {
    match value {
        FacetValue::Null => "null".to_owned(),
        FacetValue::Bool(b) => b.to_string(),
        FacetValue::Str(s) => s.clone(),
        FacetValue::Int(i) => i.to_string(),
        FacetValue::Ordinal { domain, index } => domain
            .get(*index)
            .cloned()
            .unwrap_or_else(|| index.to_string()),
        FacetValue::Set(items) => format!(
            "{{{}}}",
            items.iter().cloned().collect::<Vec<_>>().join("|")
        ),
    }
}
