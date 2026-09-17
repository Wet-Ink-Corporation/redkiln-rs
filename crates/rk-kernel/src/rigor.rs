//! Rigor monotonicity — **AC-105 / SM-035**, and the predicate is **ADR-0081's**.
//!
//! # It is AC-105, and the erratum that owes is about the record's completeness
//!
//! AC-104 is the family-**totality** criterion and SM-035's *Instrument* line names **AC-105** by
//! id. Where AC-105's THEN and ADR-0081 clause 1 differ, **ADR-0081 governs** (ADR-0092): AC-105
//! fails the check when the graph at *r+1* omits a *quality-apparatus step (adversary, policy, or
//! declared quality block)* present at *r*, while ADR-0081 requires the stronger superset relation
//! over the **closed evidence-variant set** and names the loss of a `check` step as a violation — a
//! step kind AC-105's three-way enumeration does not reach. **This module implements the superset
//! relation**, and AC-105's THEN is read as a non-exhaustive instance of it. **An erratum is OWED
//! and is not written here.**
//!
//! ```text
//! evidence(select(family, ctx, r2))  ⊇  evidence(select(family, ctx, r1))    for r1 < r2
//! ```
//!
//! # 🛑 The instrument moved, and that is recorded rather than called execution order
//!
//! ADR-0081 clause 2 is a normative must on a **named tool**: *"`lattice --validate` must check
//! block monotonicity, alongside the family-totality check it already performs"*. This stage leaves
//! `spikes/p9/lattice.py --validate` as the **totality** guard (AC-104) and puts monotonicity in a
//! Rust test. **The obligation is unchanged and nothing is withdrawn; what moves is where it
//! runs.** **Owed: either `lattice.py --validate` gains the check, or an erratum against ADR-0081
//! clause 2 records that its Phase-1 instrument is `rigor_monotonicity` in `rk-kernel`.** Neither
//! is written here. Until one lands this is clause 2's **Phase-1 stand-in**, named as such and not
//! as its home — and since `grep -ci monoton spikes/p9/lattice.py` is **0**, **neither tool has
//! ever measured monotonicity**, so this is its first measurement either way.
//!
//! # Scope: families, because clause 1 says "for every family"
//!
//! A member the engine "selects" is only defined inside a family — ADR-0056's rank rule picks the
//! lowest `method_rank` among the admissible members, and a tie is a **pack validation error at
//! load**, never a runtime choice (ADR-0056; ADR-0073 withdraws specificity for producer-of-a-kind
//! selection). Measured 2026-09-13 on `spikes/p9/catalog/`: **45 of 102** activities declare a
//! `family` and 57 do not, and the three that read the withdrawn `auth` facet
//! (`security-requirements`, `security-review`, `threat-model`) are among the 57 — so they are
//! outside this rule's scope by the record's own boundary and not by a convenience. Their seven
//! `auth` sites are **`s7e` §7e.2's**, and `spikes/p9/lint.py` — which *is* red on them by design
//! — is deliberately not run here.
//!
//! # The lattice, and the one reduction, which is sound rather than cheap
//!
//! `lattice.py --validate` enumerates kind × four ordinal bands × four toolchain instances × a
//! boolean covering array for **every** family. This module enumerates, per family, the
//! cross-product of **only the facets that family's members actually read** — the union of the
//! facets named in their `tags`, their `applicable_if` and their steps' `when` guards. That is not
//! a sample: two contexts that agree on every facet a family reads are **indistinguishable to that
//! family's admissibility filter and to its steps' guards**, so they select the same member and
//! promise the same evidence, and checking one checks both. The reduction is exact, and the survey
//! reports the context count so a lattice that enumerated nothing cannot read as a pass.
//!
//! Three domain rules, each taken from a record rather than chosen:
//!
//! - **`toolchain` and `change` are enumerated together**, over `lattice.py`'s own four instances
//!   (`node`/`{text}`, `unreal`/`{binary}`, `node+unreal`/`{text,binary}`, `dotnet`/`{text}`),
//!   because they are not independent axes — a toolchain determines the change kinds it produces.
//! - **Booleans are `true`/`false` and never `null`.** `null` is *"no evidence computed"* and is
//!   **not a member of the declared domain** (DECISIONS-2026-09-12 E1); a `null` read at selection
//!   step 2 emits `gate_opened { reason: unknown_facet, axis }` (AC-079) and is that path's
//!   business, not an admissibility sweep's.
//! - **Ordinals enumerate their declared values**, `unknown` included, which is one band more than
//!   `lattice.py`'s separate `unknown` sweep folds in.
//!
//! # `evidence(a)` is the **guaranteed** set, and a runtime-gated step is excluded from both sides
//!
//! ADR-0081's own warrant is that the comparison is *"computable from the catalog without running
//! anything"*. A step guarded by `when = "rigor >= mandatory"` is decidable from the context; a
//! step guarded by `when = "${bisect.outputs.status} != 'found'"` is not. So every `${…}` reference
//! is read as `null` and the guard is evaluated under ADR-0103's three-valued connectives: a step
//! counts when its guard is **`true`**, which makes `"rigor >= mandatory or ${x}.len() > 0"` count
//! at `mandatory` (`true or null` = `true`) and not below. A step that might or might not run is
//! excluded from **both** sides of every comparison, so it can neither manufacture a violation nor
//! hide one.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::catalog::{Activity, Catalog, Evidence, Step, StepKind};
use crate::predicate::{Context, EvalError, Expr, Literal, Tri, Value, evaluate, parse};
use crate::registry::Registry;
use crate::toml_subset::Value as TomlValue;

/// ADR-0020's ordinal deontic enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rigor {
    Forbidden,
    Discouraged,
    Optional,
    Recommended,
    Mandatory,
}

impl Rigor {
    /// The three levels the lattice sweeps. `forbidden` and `discouraged` are not swept:
    /// `forbidden` means nothing in that stage runs (ADR-0057) and `discouraged` is **reserved**,
    /// its semantics Phase 2's, with a pack setting it failing validation (AC-050).
    pub const LIVE: [Self; 3] = [Self::Optional, Self::Recommended, Self::Mandatory];

    /// All five, in order, which is the domain `rigor` compares against.
    pub const ALL: [Self; 5] = [
        Self::Forbidden,
        Self::Discouraged,
        Self::Optional,
        Self::Recommended,
        Self::Mandatory,
    ];

    /// The token as a pack and a catalog spell it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Forbidden => "forbidden",
            Self::Discouraged => "discouraged",
            Self::Optional => "optional",
            Self::Recommended => "recommended",
            Self::Mandatory => "mandatory",
        }
    }

    /// Parse a token. `None` for anything outside the five.
    #[must_use]
    pub fn from_token(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.as_str() == s)
    }
}

impl fmt::Display for Rigor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One violation of ADR-0081 clause 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonotonicityViolation {
    pub family: String,
    /// The context, rendered as its bindings in byte order, so a reader can reproduce it.
    pub context: String,
    pub lower: Rigor,
    pub upper: Rigor,
    /// The member selected at the lower rigor, and its file.
    pub lower_activity: String,
    pub lower_source: String,
    /// The member selected at the higher rigor, and its file.
    pub upper_activity: String,
    pub upper_source: String,
    /// The evidence variants the higher-rigor selection **stops** promising.
    pub lost: Vec<Evidence>,
}

impl fmt::Display for MonotonicityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lost: Vec<String> = self.lost.iter().map(ToString::to_string).collect();
        write!(
            f,
            "family {:?}: escalating {} -> {} in context [{}] moves the selection from {:?} ({}) \
             to {:?} ({}) and DROPS {} — escalating rigor must never drop the quality block \
             (ADR-0081 clause 1; AC-105)",
            self.family,
            self.lower,
            self.upper,
            self.context,
            self.lower_activity,
            self.lower_source,
            self.upper_activity,
            self.upper_source,
            lost.join(", ")
        )
    }
}

/// What the rule measured, whether or not it found anything.
///
/// The counters are not decoration. A rule that enumerated **no** family or **no** context returns
/// an empty violation list, which reads exactly like a clean catalog — Halt 8's *"a filter that
/// matches nothing exits 0"*, one level up — so the counts travel with the verdict and the tests
/// assert them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Survey {
    pub families: usize,
    pub contexts_checked: usize,
    pub comparisons: usize,
    pub violations: Vec<MonotonicityViolation>,
}

impl Survey {
    /// `true` when the catalog is monotone in internal quality.
    #[must_use]
    pub const fn is_monotone(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Why the rule could not run.
///
/// **Distinct from a violation, and never collapsed into one**: a catalog the rule cannot read is
/// not a monotone catalog, and returning "no violations" for it would be the silent pass this whole
/// stage exists to make impossible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RigorError {
    /// A predicate names something the registry does not declare, so no context can bind it.
    UnknownFacet {
        activity: String,
        predicate: String,
        facet: String,
    },
    /// A predicate does not parse under ADR-0103.
    BadPredicate {
        activity: String,
        predicate: String,
        detail: String,
    },
    /// A predicate could not be evaluated over a context that binds every facet it reads.
    Eval {
        activity: String,
        predicate: String,
        detail: String,
    },
    /// A facet whose domain this module cannot enumerate — a set-valued facet other than
    /// `toolchain`/`change`, or a facet with no declared `values`. **Refused rather than guessed**:
    /// a guessed domain is a lattice that is not the registered one.
    UnenumerableDomain { facet: String, why: String },
    /// A `min_rigor` or `max_rigor` outside ADR-0020's five.
    BadRigorToken { activity: String, token: String },
    /// The catalog declares no family at all, so the rule would pass vacuously.
    NoFamilies,
}

impl fmt::Display for RigorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownFacet {
                activity,
                predicate,
                facet,
            } => write!(
                f,
                "{activity}: predicate {predicate:?} reads {facet:?}, which the registry does not \
                 declare — an unregistered facet can feed no row (AC-072/AC-108), and this rule \
                 refuses rather than binding it to a default"
            ),
            Self::BadPredicate {
                activity,
                predicate,
                detail,
            } => write!(
                f,
                "{activity}: predicate {predicate:?} does not parse: {detail}"
            ),
            Self::Eval {
                activity,
                predicate,
                detail,
            } => write!(
                f,
                "{activity}: predicate {predicate:?} did not evaluate: {detail}"
            ),
            Self::UnenumerableDomain { facet, why } => write!(
                f,
                "facet {facet:?} has no enumerable domain for the lattice: {why}"
            ),
            Self::BadRigorToken { activity, token } => write!(
                f,
                "{activity}: rigor token {token:?} is outside ADR-0020's five"
            ),
            Self::NoFamilies => f.write_str(
                "the catalog declares no family — this rule is family-scoped (ADR-0081 clause 1) \
                 and would pass vacuously",
            ),
        }
    }
}

impl std::error::Error for RigorError {}

/// ADR-0081 clause 1, over every family and every context in the enumerated lattice.
///
/// # Errors
///
/// [`RigorError`] when the rule could not run at all. Violations are **not** errors: they come back
/// in the [`Survey`], so a caller can report how much lattice was walked beside what it found.
pub fn rigor_monotonicity_rule(
    catalog: &Catalog,
    registry: &Registry,
) -> Result<Survey, RigorError> {
    let mut families: BTreeMap<String, Vec<&Activity>> = BTreeMap::new();
    for activity in catalog.activities() {
        if let Some(family) = &activity.family {
            families.entry(family.clone()).or_default().push(activity);
        }
    }
    if families.is_empty() {
        return Err(RigorError::NoFamilies);
    }

    let mut survey = Survey {
        families: families.len(),
        ..Survey::default()
    };
    // Violations are deduplicated on (family, rigor pair, the two members, the lost set) and carry
    // the FIRST context that produced them, because one broken ceiling fires in every context the
    // family covers and a report of ten thousand copies of one finding is not a report. The context
    // count beside it is what says how much lattice was walked.
    let mut seen: BTreeSet<(String, Rigor, Rigor, String, String, Vec<Evidence>)> = BTreeSet::new();

    for (family, members) in &families {
        let prepared = members
            .iter()
            .map(|a| Prepared::build(a, registry))
            .collect::<Result<Vec<_>, _>>()?;
        let read = read_facets(&prepared);
        let axes = axes_for(&read, registry)?;

        for bindings in Odometer::new(&axes) {
            survey.contexts_checked += 1;
            let label = render_context(&bindings);

            // The selection per live rigor: the admissible member of lowest `method_rank`
            // (ADR-0056's rank rule; ADR-0073 withdraws specificity, so there is no tie-break, and
            // a tie is a pack validation error at load rather than a runtime choice).
            let mut selected: BTreeMap<Rigor, &Prepared> = BTreeMap::new();
            for rigor in Rigor::LIVE {
                let ctx = context_for(&bindings, rigor, registry);
                let mut best: Option<&Prepared> = None;
                for candidate in &prepared {
                    if !candidate.admissible(rigor, &ctx)? {
                        continue;
                    }
                    if best.is_none_or(|b| candidate.activity.method_rank < b.activity.method_rank)
                    {
                        best = Some(candidate);
                    }
                }
                if let Some(best) = best {
                    selected.insert(rigor, best);
                }
            }

            // EVERY ORDERED PAIR, not only adjacent ones. Superset is transitive, so adjacency
            // would suffice over an unbroken ladder — but a level at which the family selects
            // nothing is a TOTALITY hole (AC-104, a separate obligation), and adjacency alone would
            // let a hole hide a monotonicity break that straddles it.
            for (i, lower) in Rigor::LIVE.into_iter().enumerate() {
                for upper in Rigor::LIVE.into_iter().skip(i + 1) {
                    let (Some(low), Some(high)) = (selected.get(&lower), selected.get(&upper))
                    else {
                        continue;
                    };
                    survey.comparisons += 1;
                    let low_ev = low.evidence(lower, &context_for(&bindings, lower, registry))?;
                    let high_ev = high.evidence(upper, &context_for(&bindings, upper, registry))?;
                    let lost: Vec<Evidence> = low_ev.difference(&high_ev).copied().collect();
                    if lost.is_empty() {
                        continue;
                    }
                    let key = (
                        family.clone(),
                        lower,
                        upper,
                        low.activity.id.clone(),
                        high.activity.id.clone(),
                        lost.clone(),
                    );
                    if seen.insert(key) {
                        survey.violations.push(MonotonicityViolation {
                            family: family.clone(),
                            context: label.clone(),
                            lower,
                            upper,
                            lower_activity: low.activity.id.clone(),
                            lower_source: low.activity.source.clone(),
                            upper_activity: high.activity.id.clone(),
                            upper_source: high.activity.source.clone(),
                            lost,
                        });
                    }
                }
            }
        }
    }
    Ok(survey)
}

/// One family member with its predicates parsed once.
struct Prepared<'a> {
    activity: &'a Activity,
    min: Rigor,
    max: Option<Rigor>,
    /// The tag filter, compiled per §9b.20 — *same facet disjoins, distinct facets conjoin*.
    tags: Option<Expr>,
    applicable_if: Option<Expr>,
    /// Each step's guard, with every `${…}` reference already read as `null`.
    guards: Vec<Option<Expr>>,
}

impl<'a> Prepared<'a> {
    fn build(activity: &'a Activity, registry: &Registry) -> Result<Self, RigorError> {
        let rigor = |token: &str| {
            Rigor::from_token(token).ok_or_else(|| RigorError::BadRigorToken {
                activity: activity.id.clone(),
                token: token.to_owned(),
            })
        };
        let compile = |src: &str| -> Result<Expr, RigorError> {
            parse(src)
                .map(|p| null_out_refs(&p.expr))
                .map_err(|e| RigorError::BadPredicate {
                    activity: activity.id.clone(),
                    predicate: src.to_owned(),
                    detail: e.to_string(),
                })
        };
        let tags = tag_expr(activity, registry)
            .map(|src| compile(&src))
            .transpose()?;
        let applicable_if = activity.applicable_if.as_deref().map(compile).transpose()?;
        let guards = activity
            .steps
            .iter()
            .map(|s| s.when.as_deref().map(compile).transpose())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            activity,
            min: rigor(&activity.min_rigor)?,
            max: activity.max_rigor.as_deref().map(rigor).transpose()?,
            tags,
            applicable_if,
            guards,
        })
    }

    /// `lattice.py`'s `admissible()`, clause for clause: the rigor floor, the `max_rigor` ceiling,
    /// the tag filter, then `applicable_if`.
    fn admissible(&self, rigor: Rigor, ctx: &Context) -> Result<bool, RigorError> {
        if rigor < self.min || self.max.is_some_and(|m| rigor > m) {
            return Ok(false);
        }
        for (expr, src) in [
            (self.tags.as_ref(), "<tags>"),
            (self.applicable_if.as_ref(), "<applicable_if>"),
        ] {
            if let Some(expr) = expr
                && self.truth(expr, src, ctx)? != Tri::True
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// The **guaranteed** evidence set: what every run of this activity in this context mints.
    fn evidence(&self, _rigor: Rigor, ctx: &Context) -> Result<BTreeSet<Evidence>, RigorError> {
        let mut out = BTreeSet::new();
        for (step, guard) in self.activity.steps.iter().zip(&self.guards) {
            if let Some(guard) = guard
                && self.truth(guard, &step.id, ctx)? != Tri::True
            {
                continue;
            }
            out.extend(step.mints());
        }
        Ok(out)
    }

    fn truth(&self, expr: &Expr, what: &str, ctx: &Context) -> Result<Tri, RigorError> {
        evaluate(expr, ctx).map(|o| o.value).map_err(|e| match e {
            EvalError::UnknownField(facet) => RigorError::UnknownFacet {
                activity: self.activity.id.clone(),
                predicate: what.to_owned(),
                facet,
            },
            other => RigorError::Eval {
                activity: self.activity.id.clone(),
                predicate: what.to_owned(),
                detail: other.to_string(),
            },
        })
    }
}

/// The tag filter as a predicate string: *same facet disjoins, distinct facets conjoin*
/// (schema §9b.20), with a set-valued facet disjoining through ADR-0050's membership form.
///
/// It is built as source and handed to **this crate's own ADR-0103 parser** rather than evaluated
/// by a second code path, so the tag filter and every hand-written predicate in the catalog go
/// through one grammar.
fn tag_expr(activity: &Activity, registry: &Registry) -> Option<String> {
    let mut groups: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for tag in &activity.tags {
        let (facet, value) = tag.split_once(':')?;
        groups.entry(facet).or_default().push(value);
    }
    if groups.is_empty() {
        return None;
    }
    let parts: Vec<String> = groups
        .into_iter()
        .map(|(facet, values)| {
            let set = registry.facet(facet).is_some_and(|f| f.set);
            let terms: Vec<String> = values
                .into_iter()
                .map(|v| {
                    if set {
                        format!("'{v}' in {facet}")
                    } else {
                        format!("{facet} == '{v}'")
                    }
                })
                .collect();
            format!("({})", terms.join(" or "))
        })
        .collect();
    Some(parts.join(" and "))
}

/// Every facet a family reads, through any of its three routes.
fn read_facets(prepared: &[Prepared<'_>]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for member in prepared {
        for tag in &member.activity.tags {
            if let Some((facet, _)) = tag.split_once(':') {
                out.insert(facet.to_owned());
            }
        }
        for expr in std::iter::once(member.applicable_if.as_ref())
            .chain(member.guards.iter().map(Option::as_ref))
            .flatten()
        {
            paths(expr, &mut out);
        }
    }
    // `rigor` is the swept axis, not a context axis: it is bound per comparison and never
    // enumerated as a facet domain.
    out.remove("rigor");
    out
}

/// One axis of the lattice: the alternatives it offers, each a bundle of bindings.
struct Axis {
    alternatives: Vec<Vec<(String, Value)>>,
}

/// `lattice.py`'s four toolchain instances, with the change kinds each produces. `toolchain` and
/// `change` are **one axis**, because they are not independent: a toolchain determines the change
/// kinds it can produce, and sweeping them separately would enumerate contexts no item can be in.
const TOOLCHAIN_INSTANCES: [(&[&str], &[&str]); 4] = [
    (&["node"], &["text"]),
    (&["unreal"], &["binary"]),
    (&["node", "unreal"], &["text", "binary"]),
    (&["dotnet"], &["text"]),
];

fn axes_for(read: &BTreeSet<String>, registry: &Registry) -> Result<Vec<Axis>, RigorError> {
    let mut axes = Vec::new();
    if read.contains("toolchain") || read.contains("change") {
        axes.push(Axis {
            alternatives: TOOLCHAIN_INSTANCES
                .iter()
                .map(|(tools, changes)| {
                    vec![
                        ("toolchain".to_owned(), Value::set(tools.iter().copied())),
                        ("change".to_owned(), Value::set(changes.iter().copied())),
                    ]
                })
                .collect(),
        });
    }
    for name in read {
        if name == "toolchain" || name == "change" {
            continue;
        }
        let facet = registry
            .facet(name)
            .ok_or_else(|| RigorError::UnknownFacet {
                activity: "<lattice>".to_owned(),
                predicate: "<context axis>".to_owned(),
                facet: name.clone(),
            })?;
        if facet.set {
            return Err(RigorError::UnenumerableDomain {
                facet: name.clone(),
                why: "a set-valued facet other than toolchain/change has no enumeration this \
                      module may invent — a guessed domain is a lattice that is not the registered \
                      one"
                .to_owned(),
            });
        }
        if facet.values.is_empty() {
            return Err(RigorError::UnenumerableDomain {
                facet: name.clone(),
                why: "the registry declares no `values`".to_owned(),
            });
        }
        let domain: Vec<String> = facet
            .values
            .iter()
            .map(|v| match v {
                TomlValue::Str(s) => s.clone(),
                other => format!("{other:?}"),
            })
            .collect();
        let mut alternatives = Vec::with_capacity(facet.values.len());
        for value in &facet.values {
            // Booleans are `true`/`false` and never `null`: `null` is "no evidence computed" and is
            // NOT a member of the declared domain (DECISIONS-2026-09-12 E1). A null read at
            // selection step 2 emits gate_opened { reason: unknown_facet, axis } (AC-079) and is
            // that path's business, not an admissibility sweep's.
            let bound = match value {
                TomlValue::Bool(b) => Value::Bool(*b),
                TomlValue::Str(s) if facet.ordered => {
                    let index = domain.iter().position(|d| d == s).unwrap_or(0);
                    Value::Ordinal {
                        domain: domain.clone(),
                        index,
                    }
                }
                TomlValue::Str(s) => Value::Str(s.clone()),
                other => {
                    return Err(RigorError::UnenumerableDomain {
                        facet: name.clone(),
                        why: format!("value {other:?} is neither a string nor a boolean"),
                    });
                }
            };
            alternatives.push(vec![(name.clone(), bound)]);
        }
        axes.push(Axis { alternatives });
    }
    Ok(axes)
}

/// A plain odometer over the axes, so the enumeration order is deterministic and reproducible.
struct Odometer<'a> {
    axes: &'a [Axis],
    counters: Vec<usize>,
    done: bool,
}

impl<'a> Odometer<'a> {
    fn new(axes: &'a [Axis]) -> Self {
        let done = axes.iter().any(|a| a.alternatives.is_empty());
        Self {
            axes,
            counters: vec![0; axes.len()],
            done,
        }
    }
}

impl Iterator for Odometer<'_> {
    type Item = Vec<(String, Value)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut out = Vec::new();
        for (axis, i) in self.axes.iter().zip(&self.counters) {
            out.extend(axis.alternatives[*i].iter().cloned());
        }
        // Advance.
        let mut carry = true;
        for (axis, i) in self.axes.iter().zip(self.counters.iter_mut()).rev() {
            if !carry {
                break;
            }
            *i += 1;
            if *i < axis.alternatives.len() {
                carry = false;
            } else {
                *i = 0;
            }
        }
        if carry {
            self.done = true;
        }
        Some(out)
    }
}

/// The context a predicate sees: the axis bindings plus `rigor` as an ordinal over ADR-0020's five.
fn context_for(bindings: &[(String, Value)], rigor: Rigor, registry: &Registry) -> Context {
    let domain: Vec<String> = registry.facet("rigor").map_or_else(
        || Rigor::ALL.iter().map(|r| r.as_str().to_owned()).collect(),
        |f| {
            f.values
                .iter()
                .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                .collect()
        },
    );
    let index = domain
        .iter()
        .position(|d| d == rigor.as_str())
        .unwrap_or_default();
    let mut ctx = Context::new().with("rigor", Value::Ordinal { domain, index });
    for (name, value) in bindings {
        ctx.bind(name, value.clone());
    }
    ctx
}

fn render_context(bindings: &[(String, Value)]) -> String {
    let mut parts: Vec<String> = bindings
        .iter()
        .map(|(name, value)| format!("{name}={}", render_value(value)))
        .collect();
    parts.sort();
    parts.join(", ")
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(b) => b.to_string(),
        Value::Str(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Ordinal { domain, index } => domain[*index].clone(),
        Value::List(items) => {
            let inner: Vec<String> = items.iter().map(render_value).collect();
            format!("{{{}}}", inner.join("+"))
        }
        Value::Record(_) => "<record>".to_owned(),
    }
}

/// Every field-path root the expression reads.
fn paths(expr: &Expr, out: &mut BTreeSet<String>) {
    match expr {
        Expr::Path(p) => {
            out.insert(p.split('.').next().unwrap_or(p).to_owned());
        }
        Expr::Or(l, r) | Expr::And(l, r) => {
            paths(l, out);
            paths(r, out);
        }
        Expr::Not(inner) | Expr::Group(inner) | Expr::Len(inner) => paths(inner, out),
        Expr::Cmp { left, right, .. } => {
            paths(left, out);
            paths(right, out);
        }
        Expr::In { left, list } => {
            paths(left, out);
            paths(list, out);
        }
        Expr::Member { literal, set } => {
            paths(literal, out);
            paths(set, out);
        }
        Expr::List(items) => {
            for item in items {
                paths(item, out);
            }
        }
        Expr::Quant { source, body, .. } => {
            paths(source, out);
            paths(body, out);
        }
        Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

fn has_ref(expr: &Expr) -> bool {
    let mut found = false;
    walk(expr, &mut |e| {
        if matches!(e, Expr::Ref(_)) {
            found = true;
        }
    });
    found
}

fn walk(expr: &Expr, f: &mut impl FnMut(&Expr)) {
    f(expr);
    match expr {
        Expr::Or(l, r)
        | Expr::And(l, r)
        | Expr::Cmp {
            left: l, right: r, ..
        } => {
            walk(l, f);
            walk(r, f);
        }
        Expr::In { left, list } => {
            walk(left, f);
            walk(list, f);
        }
        Expr::Member { literal, set } => {
            walk(literal, f);
            walk(set, f);
        }
        Expr::Not(inner) | Expr::Group(inner) | Expr::Len(inner) => walk(inner, f),
        Expr::List(items) => {
            for item in items {
                walk(item, f);
            }
        }
        Expr::Quant { source, body, .. } => {
            walk(source, f);
            walk(body, f);
        }
        Expr::Path(_) | Expr::Ref(_) | Expr::Lit(_) => {}
    }
}

/// Read every `${…}` reference as `null`, by replacing the **smallest enclosing non-connective**
/// node that contains one.
///
/// Substituting the reference itself would put a `null` where a list or a status string belongs and
/// turn a decidable guard into an evaluator error (`len(null)`, an ordering against `null`).
/// Substituting the whole comparison says the only thing the catalog knows about it: *unknown*.
/// The connectives are then ADR-0103's three-valued ones, so `"rigor >= mandatory or ${x}.len() >
/// 0"` is `true` at `mandatory` (`true or null` = `true`) and `null` below it — and a step counts
/// only when its guard is **`true`**, so a step that might or might not run is excluded from both
/// sides of every comparison and can neither manufacture a violation nor hide one.
fn null_out_refs(expr: &Expr) -> Expr {
    match expr {
        Expr::Or(l, r) => Expr::Or(Box::new(null_out_refs(l)), Box::new(null_out_refs(r))),
        Expr::And(l, r) => Expr::And(Box::new(null_out_refs(l)), Box::new(null_out_refs(r))),
        Expr::Not(inner) => Expr::Not(Box::new(null_out_refs(inner))),
        Expr::Group(inner) => Expr::Group(Box::new(null_out_refs(inner))),
        other if has_ref(other) => Expr::Lit(Literal::Null),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::{RigorError, rigor_monotonicity_rule};
    use crate::catalog::Catalog;
    use crate::registry::Registry;

    const REGISTRY: &str = "[facet.has_repro]\nvalues = [true, false]\nsource = \"human\"\n";

    const NO_FAMILY: &str = "[activity]\nid = \"lone\"\nversion = 1\nstage = \"analysis\"\n\
                             provides = [\"RootCause\"]\nmethod_rank = 10\nmin_rigor = \"optional\"\n\
                             evidence = [\"ToolReport\"]\n\n[[step]]\nid = \"s\"\nkind = \"tool\"\n";

    /// The guard that exists to stop a vacuous pass, asserted — which nothing did until 2026-09-13.
    ///
    /// A catalog declaring no `family` enumerates no pair, finds no violation and would report
    /// itself monotone. `RigorError::NoFamilies` is why it does not, and deleting it changed
    /// nothing any test observed.
    #[test]
    fn a_catalog_with_no_family_is_an_error_and_not_a_vacuous_pass() {
        let registry = Registry::from_toml(REGISTRY).expect("the registry loads");
        let catalog =
            Catalog::from_sources(&[("lone.toml", NO_FAMILY)]).expect("the catalog loads");
        assert_eq!(
            rigor_monotonicity_rule(&catalog, &registry).err(),
            Some(RigorError::NoFamilies),
            "a family-less catalog must be refused, never surveyed and reported monotone"
        );

        // THE CONTROL: give the same activity a family and the rule runs and surveys it. Without
        // this the assertion above is satisfied by a rule that refuses every catalog.
        let with_family = NO_FAMILY.replace(
            "method_rank = 10\n",
            "method_rank = 10\nfamily = \"localisation\"\n",
        );
        assert!(with_family.contains("family"), "the anchor applied");
        let catalog =
            Catalog::from_sources(&[("lone.toml", &with_family)]).expect("the catalog loads");
        let survey = rigor_monotonicity_rule(&catalog, &registry).expect("the rule runs");
        assert_eq!(survey.families, 1);
        assert!(survey.contexts_checked > 0, "{survey:?}");
    }
}

// ------------------------------------------------------------ AC-303: the review-confirm step survives escalation

/// A review-confirm step the family lattice lost on escalation (AC-303).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmStepViolation {
    pub family: String,
    pub context: String,
    pub lower: Rigor,
    pub upper: Rigor,
    pub lower_activity: String,
    pub lower_source: String,
    pub upper_activity: String,
    pub upper_source: String,
    /// The review-confirm steps present in the lower selection's resolved graph and absent from
    /// the upper's, by step id.
    pub lost: Vec<String>,
}

impl fmt::Display for ConfirmStepViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "family {:?}: escalating {} -> {} in context [{}] moves the selection from {:?} ({}) to \
             {:?} ({}) and DROPS the review-confirm step(s) {:?}",
            self.family,
            self.lower.as_str(),
            self.upper.as_str(),
            self.context,
            self.lower_activity,
            self.lower_source,
            self.upper_activity,
            self.upper_source,
            self.lost
        )
    }
}

/// What the AC-303 rule walked.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfirmSurvey {
    pub families: usize,
    pub contexts_checked: usize,
    pub comparisons: usize,
    pub violations: Vec<ConfirmStepViolation>,
}

/// **The review-confirm block, identified structurally.** No catalog step is named `review-confirm`,
/// and the steps literally named `confirm` are `toolchain:test` tool steps that `bisect` KEEPS — so
/// a rule keyed on that id could never make AC-303's own regression row red [measured 2026-09-17].
/// The design's sentence introducing AC-303 names the F37 case as *"bisect, an activity with no
/// adversary and no policy step"*: the **review** half is the reviewer agent (`kind = agent`,
/// `role = reviewer`, `outputs = value:Findings`) and the **confirm** half is the `check` step that
/// consumes its findings (`quality_block::reads_findings` over its `expr`). A `check` mints no
/// evidence, which is exactly why AC-105's lint cannot see its loss and AC-303 must.
fn is_review_confirm_step(step: &Step) -> bool {
    match step.kind {
        StepKind::Agent => {
            step.role.as_deref() == Some("reviewer")
                && step.output_type.as_deref() == Some("Findings")
        }
        StepKind::Check => step
            .expr
            .as_deref()
            .and_then(|src| parse(src).ok())
            .is_some_and(|p| crate::quality_block::reads_findings(&p.expr)),
        StepKind::Tool | StepKind::Gate | StepKind::Emit => false,
    }
}

impl Prepared<'_> {
    /// The steps the activity's graph resolves to at `ctx`: every step whose `when` guard (with
    /// `${…}` references nulled out) is `Tri::True`, in declaration order. [`Self::evidence`] is
    /// this set projected to ADR-0036 evidence; AC-303 reads the steps themselves.
    fn resolved_steps(&self, ctx: &Context) -> Result<Vec<&Step>, RigorError> {
        let mut out = Vec::new();
        for (step, guard) in self.activity.steps.iter().zip(&self.guards) {
            if let Some(guard) = guard
                && self.truth(guard, &step.id, ctx)? != Tri::True
            {
                continue;
            }
            out.push(step);
        }
        Ok(out)
    }

    /// The review-confirm steps in the resolved graph, by id.
    fn review_confirm_steps(&self, ctx: &Context) -> Result<BTreeSet<String>, RigorError> {
        Ok(self
            .resolved_steps(ctx)?
            .into_iter()
            .filter(|s| is_review_confirm_step(s))
            .map(|s| s.id.clone())
            .collect())
    }
}

/// **AC-303 — no escalation removes the review-confirm step from the resolved graph.**
///
/// > GIVEN any pair of rigor levels (lower, higher) for a stage that has a review-confirm step /
/// > WHEN the lattice is linted / THEN no escalation removes the review-confirm step from the
/// > resolved graph.
///
/// The same lattice as [`rigor_monotonicity_rule`] — every family, every context the family's
/// members read, every live rigor, the lowest-`method_rank` admissible member selected at each,
/// **every ordered pair** compared — projected to a different thing: the set of review-confirm
/// step ids in the selected member's resolved graph, not the evidence it mints. **AC-105's twin,
/// not its credit** (`s7b-ii` §7b-ii.4): the two rules share the F37 `mandatory`+`has_repro`
/// regression row and nothing else, and a lost `check` step is visible only here.
///
/// The `(activity × rigor)` wording is read as *the activity selected at each rigor over the
/// family lattice*: an intra-activity reading is vacuously green on both the fixture and the
/// shipped catalog (every rigor-only step guard in the catalog is upward), and the F37 loss is
/// between two different selected activities. Both readings are stated so the one taken is
/// visible; the register amendment to AC-303's wording is raised in the tracker, not written here.
///
/// # Errors
///
/// As [`rigor_monotonicity_rule`].
pub fn confirm_step_monotonicity_rule(
    catalog: &Catalog,
    registry: &Registry,
) -> Result<ConfirmSurvey, RigorError> {
    let mut families: BTreeMap<String, Vec<&Activity>> = BTreeMap::new();
    for activity in catalog.activities() {
        if let Some(family) = &activity.family {
            families.entry(family.clone()).or_default().push(activity);
        }
    }
    if families.is_empty() {
        return Err(RigorError::NoFamilies);
    }

    let mut survey = ConfirmSurvey {
        families: families.len(),
        ..ConfirmSurvey::default()
    };
    let mut seen: BTreeSet<(String, Rigor, Rigor, String, String, Vec<String>)> = BTreeSet::new();

    for (family, members) in &families {
        let prepared = members
            .iter()
            .map(|a| Prepared::build(a, registry))
            .collect::<Result<Vec<_>, _>>()?;
        let read = read_facets(&prepared);
        let axes = axes_for(&read, registry)?;

        for bindings in Odometer::new(&axes) {
            survey.contexts_checked += 1;
            let label = render_context(&bindings);

            let mut selected: BTreeMap<Rigor, &Prepared> = BTreeMap::new();
            for rigor in Rigor::LIVE {
                let ctx = context_for(&bindings, rigor, registry);
                let mut best: Option<&Prepared> = None;
                for candidate in &prepared {
                    if !candidate.admissible(rigor, &ctx)? {
                        continue;
                    }
                    if best.is_none_or(|b| candidate.activity.method_rank < b.activity.method_rank)
                    {
                        best = Some(candidate);
                    }
                }
                if let Some(best) = best {
                    selected.insert(rigor, best);
                }
            }

            for (i, lower) in Rigor::LIVE.into_iter().enumerate() {
                for upper in Rigor::LIVE.into_iter().skip(i + 1) {
                    let (Some(low), Some(high)) = (selected.get(&lower), selected.get(&upper))
                    else {
                        continue;
                    };
                    survey.comparisons += 1;
                    let low_steps =
                        low.review_confirm_steps(&context_for(&bindings, lower, registry))?;
                    let high_steps =
                        high.review_confirm_steps(&context_for(&bindings, upper, registry))?;
                    // the CONFIRM step is compared by signature class, not by id, because ids
                    // differ across activities (`adversary` / `review`, `policy` / `verdict`):
                    // the loss is "the lower graph had a reviewer step and the upper has none",
                    // and likewise for the findings-consuming check. The ids reported are the
                    // lower graph's, so the row names what was dropped.
                    let has = |steps: &BTreeSet<String>, act: &Activity, kind: StepKind| {
                        act.steps
                            .iter()
                            .any(|s| s.kind == kind && steps.contains(&s.id))
                    };
                    let mut lost: Vec<String> = Vec::new();
                    for kind in [StepKind::Agent, StepKind::Check] {
                        if has(&low_steps, low.activity, kind)
                            && !has(&high_steps, high.activity, kind)
                        {
                            lost.extend(
                                low.activity
                                    .steps
                                    .iter()
                                    .filter(|s| s.kind == kind && low_steps.contains(&s.id))
                                    .map(|s| s.id.clone()),
                            );
                        }
                    }
                    if lost.is_empty() {
                        continue;
                    }
                    let key = (
                        family.clone(),
                        lower,
                        upper,
                        low.activity.id.clone(),
                        high.activity.id.clone(),
                        lost.clone(),
                    );
                    if seen.insert(key) {
                        survey.violations.push(ConfirmStepViolation {
                            family: family.clone(),
                            context: label.clone(),
                            lower,
                            upper,
                            lower_activity: low.activity.id.clone(),
                            lower_source: low.activity.source.clone(),
                            upper_activity: high.activity.id.clone(),
                            upper_source: high.activity.source.clone(),
                            lost,
                        });
                    }
                }
            }
        }
    }
    Ok(survey)
}
