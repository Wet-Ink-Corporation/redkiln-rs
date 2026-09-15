//! **AC-495** — pack validation refuses a stage whose only route needs a capability nothing claims,
//! with a **span** at the requiring row.
//!
//! > **AC-495** — GIVEN a process pack whose only route through a stage requires a capability no
//! > configured adapter claims / WHEN the pack is validated at startup / THEN validation fails with
//! > a span pointing at the requiring row, before any activity runs / AND a pack that declares an
//! > alternative route for that capability validates instead of failing.
//! > *Verified by:* a pack-validation test with spans over two fixture packs (one unroutable, one
//! > routed), asserting the span location on the failing one and clean validation on the routed one.
//!
//! # 🛑 THE NOTE IS LOAD-BEARING AND THIS MODULE HONOURS IT BY DOING LESS
//!
//! AC-495's own *Note*: *"the earlier draft's second clause also asserted that the alternative
//! branch is **taken at runtime**. That is a different behaviour with a different instrument, and it
//! is already AC-500's subject; asserting it here made one row cover both startup validation and
//! dispatch-time routing. **Trimmed to the validation-time twin.**"*
//!
//! So this module decides **one** thing: does a route exist. It does not choose one, does not
//! order them, does not evaluate a row's `when`, and returns no selected route — there is no value
//! in [`RoutingReport`] a caller could mistake for a dispatch decision. **AC-500 is ARMED at this
//! stage and is not claimed** (`s7b`:1791: *"AC-500 is ARMED, not claimed … the path is built here
//! and the pack half is 7e's"*).
//!
//! # What a *route* is here, stated because the register does not define one
//!
//! A stage's requirement set is `crate::select::step2`'s `R(stage, ctx) = baseline ∪ artifacts of
//! every require row whose `when` holds`. This rule reads that structure through the **shipped**
//! loader ([`crate::select::Pack`]) rather than re-reading the file, so a pack the selection engine
//! refuses is refused here too and the two cannot disagree about which rows exist.
//!
//! A **route to an artifact** is a declaration that produces it:
//!
//! - every entry of `[stage.<name>] baseline` — a route with **no** capability;
//! - every artifact of every `[[stage.<name>.require]]` row — a route carrying that row's
//!   `capability`, when it declares one.
//!
//! An artifact is **routed** when at least one of its routes needs no capability, or needs one the
//! set claims. It is **unroutable** when every one of its routes needs a capability nothing claims —
//! *"the only route through a stage requires a capability no configured adapter claims"* — and that
//! is the violation, reported at the **first** such row in file order.
//!
//! **`when` is not evaluated, and that is forced rather than chosen.** The WHEN is *"the pack is
//! validated **at startup**"*, and at startup there is no item, so there is no context vector to
//! evaluate a row's predicate against. A row that *can* fire is therefore a route. The consequence
//! is stated: a row whose `when` is unsatisfiable still counts as an alternative here. Narrowing
//! that needs a satisfiability decision over the predicate grammar, which is nobody's criterion at
//! this pin.
//!
//! **A row that declares a capability and names no artifacts is a route to nothing**, so it cannot
//! be a stage's only route and is not refused. It is counted — [`RoutingReport::rows_without_artifacts`]
//! — so that reading is a number a test can read rather than a sentence a reader has to trust.
//!
//! # The `capability` key is read HERE and not by the pack loader, and that is deliberate
//!
//! `crate::select::pack`'s `RequireRow` carries `when`, `artifacts`, `rigor` and `site`. A
//! `capability` key is not among them and is not added: `Pack::to_json` is the `dag_assembled`
//! payload's shape, AC-040's byte-identity assertions and the committed golden log are measured over
//! those bytes, and a routing concern is not an input to the selection function. So the key is read
//! from the **same parse** the spans come from, keyed by the same `stage.<name>.require[<i>]` path
//! the loader builds its `site` from — one parser, one path vocabulary, no second reader to drift.
//!
//! # Spans are real, at line grain, and they come from the loader rather than a second scan
//!
//! `crate::toml_subset` is line-oriented over a **measured** subset (its header: no multi-line value
//! anywhere in 102 catalog files, 12 packs, `facets.toml` and `artifacts.toml`), and it already
//! reports a line on every refusal. [`crate::toml_subset::parse_spans`] returns the same `Table`
//! `parse` returns **plus** the line each header and each key was read at — `parse` is now defined as
//! `parse_spans(…).map(|(t, _)| t)`, so there is exactly one parser and a span cannot point
//! somewhere the value did not come from.
//!
//! A violation carries **two** spans, because *"the requiring row"* and *"the requirement"* are
//! different lines and both are useful: [`RoutingViolation::row`] is the `[[stage.<name>.require]]`
//! header — the row — and [`RoutingViolation::capability_span`] is its `capability = "…"` key.
//!
//! # 🛑 *"before any activity runs"* — what is discharged and what is ARMED
//!
//! | clause | state |
//! | --- | --- |
//! | *"validation fails with a span pointing at the requiring row"* | **measured** — the span's line is asserted as a literal **and** cross-read against the fixture's own text at that line, so a rule that reported a plausible-but-wrong line is red |
//! | *"a pack that declares an alternative route … validates instead of failing"* | **measured** — over a second fixture that differs from the first by that row alone, and the test deletes the row to show the green depends on it |
//! | *"before any activity runs"* | **a TYPE fact here, and an ARMED half.** [`pack_capability_routing_rule`] takes `(&str, &str, &ClaimedCapabilities)`. It binds no `rk_ports::root::Ports`, no [`crate::runtime::Runtime`] and no `Harness`, so no path from it reaches a dispatch and no activity *can* run during it. What is **not** discharged is the temporal ordering of a live run — that validation is called *before* the runtime is stepped — which needs a composition root that calls both. That is `rk-cli`'s, **Step 7d**, and it is named rather than claimed |

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::resolver::ClaimedCapabilities;
use crate::select::{Pack, PackError};
use crate::toml_subset::{self, Value};

/// A location in a pack file: the file the caller named, and a 1-based line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    source: String,
    line: usize,
}

impl Span {
    /// One location.
    #[must_use]
    pub fn new(source: &str, line: usize) -> Self {
        Self {
            source: source.to_owned(),
            line,
        }
    }

    /// The file.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// The 1-based line.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.source, self.line)
    }
}

/// What stopped a pack from validating.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingViolation {
    /// The pack is not a pack. Carried whole, with the line the reader stopped at when it has one,
    /// because a rule that reported *"no violations"* for a file it could not read would be the
    /// silent pass `crate::toml_subset`'s header exists to prevent.
    PackDoesNotLoad {
        source: String,
        detail: String,
        span: Option<Span>,
    },
    /// A `capability` key that is not a non-empty string. Refused rather than ignored: an ignored
    /// malformed key is a requirement that silently stops being one.
    CapabilityNotAToken {
        site: String,
        row: Span,
        detail: String,
    },
    /// **AC-495's THEN.** Every route to `artifact` through `stage` needs a capability the set does
    /// not claim.
    StageUnroutable {
        stage: String,
        artifact: String,
        /// The token the reported row needs — the one the run would fail `Unsupported` on.
        capability: String,
        /// `<pack>: stage.<name>.require[<i>]`, the loader's own site vocabulary.
        site: String,
        /// The `[[stage.<name>.require]]` header — **the requiring row**.
        row: Span,
        /// Its `capability = "…"` key.
        capability_span: Span,
        /// How many routes to this artifact were considered before the stage was called unroutable.
        /// **1** means there was only ever one; a number above 1 means every alternative was
        /// checked and every one of them needed an unclaimed token.
        routes_considered: usize,
    },
}

impl RoutingViolation {
    /// The row this violation points at, when it points at one.
    #[must_use]
    pub const fn row(&self) -> Option<&Span> {
        match self {
            Self::PackDoesNotLoad { .. } => None,
            Self::CapabilityNotAToken { row, .. } | Self::StageUnroutable { row, .. } => Some(row),
        }
    }

    /// The `capability = "…"` key this violation points at, when it points at one.
    #[must_use]
    pub const fn capability_span(&self) -> Option<&Span> {
        match self {
            Self::PackDoesNotLoad { .. } | Self::CapabilityNotAToken { .. } => None,
            Self::StageUnroutable {
                capability_span, ..
            } => Some(capability_span),
        }
    }

    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::PackDoesNotLoad { .. } => "pack_does_not_load",
            Self::CapabilityNotAToken { .. } => "capability_not_a_token",
            Self::StageUnroutable { .. } => "stage_unroutable",
        }
    }
}

impl fmt::Display for RoutingViolation {
    /// **Leads with the span**, so the location is the first thing on the line rather than the last.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackDoesNotLoad {
                source,
                detail,
                span,
            } => match span {
                Some(span) => write!(f, "{span}: this pack does not load: {detail}"),
                None => write!(f, "{source}: this pack does not load: {detail}"),
            },
            Self::CapabilityNotAToken { site, row, detail } => write!(
                f,
                "{row}: {site} declares a `capability` that is not a non-empty string ({detail})"
            ),
            Self::StageUnroutable {
                stage,
                artifact,
                capability,
                row,
                capability_span,
                routes_considered,
                ..
            } => write!(
                f,
                "{row}: stage `{stage}` has no route to `{artifact}` — every one of its \
                 {routes_considered} route(s) requires a capability no configured adapter claims, \
                 and this row requires `{capability}` ({capability_span}). A pack that declares an \
                 alternative route for that capability validates instead (AC-495)"
            ),
        }
    }
}

impl std::error::Error for RoutingViolation {}

/// House Shape C over one pack: the counts travel with the verdict, so *"validated nothing"* is
/// distinguishable from *"validated everything and found nothing wrong"*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingReport {
    stages_walked: usize,
    rows_walked: usize,
    rows_without_artifacts: usize,
    artifacts_checked: usize,
    routes_checked: usize,
    capabilities_required: Vec<String>,
    claims_walked: usize,
}

impl RoutingReport {
    /// Every `[stage.<name>]` table the pack declares.
    #[must_use]
    pub const fn stages_walked(&self) -> usize {
        self.stages_walked
    }

    /// Every `[[stage.<name>.require]]` row, across every stage.
    #[must_use]
    pub const fn rows_walked(&self) -> usize {
        self.rows_walked
    }

    /// Rows that name no artifact, and so route nothing. See the module header.
    #[must_use]
    pub const fn rows_without_artifacts(&self) -> usize {
        self.rows_without_artifacts
    }

    /// Distinct `(stage, artifact)` pairs put to the routing question.
    #[must_use]
    pub const fn artifacts_checked(&self) -> usize {
        self.artifacts_checked
    }

    /// Every route considered, baseline entries included.
    #[must_use]
    pub const fn routes_checked(&self) -> usize {
        self.routes_checked
    }

    /// Every distinct capability token the pack's rows require, in byte order.
    #[must_use]
    pub fn capabilities_required(&self) -> &[String] {
        &self.capabilities_required
    }

    /// How many rows the claimed set held while the pack was validated.
    #[must_use]
    pub const fn claims_walked(&self) -> usize {
        self.claims_walked
    }
}

/// One declaration that produces an artifact.
#[derive(Debug, Clone)]
struct Route {
    site: String,
    capability: Option<String>,
    row: Span,
    capability_span: Option<Span>,
}

/// **AC-495's rule.** Every stage of one pack, against one adapter's claimed set.
///
/// `source` is the name that appears in every span and in the loader's own `site` strings; `src` is
/// the pack's text. Nothing here opens a file — the caller supplies the bytes, which is what keeps
/// this rule callable from `select`'s side of the purity line as well as from a composition root.
///
/// # Errors
///
/// Every [`RoutingViolation`] the pack carries, in a deterministic order: stages in byte order, and
/// within a stage, artifacts in byte order. A pack that does not load produces exactly one.
pub fn pack_capability_routing_rule(
    source: &str,
    src: &str,
    claimed: &ClaimedCapabilities,
) -> Result<RoutingReport, Vec<RoutingViolation>> {
    // ONE PARSE for the spans and the `capability` keys, and the SHIPPED loader for the structure.
    // A pack the selection engine refuses is refused here too, so the two cannot disagree about
    // which rows exist — and the row indices below are the loader's own.
    let (doc, spans) = toml_subset::parse_spans(src).map_err(|error| {
        vec![RoutingViolation::PackDoesNotLoad {
            source: source.to_owned(),
            detail: error.detail.clone(),
            span: Some(Span::new(source, error.line)),
        }]
    })?;
    let pack = Pack::from_toml(source, src).map_err(|error| {
        let span = match &error {
            PackError::Toml { error, .. } => Some(Span::new(source, error.line)),
            PackError::Shape { .. }
            | PackError::DiscouragedIsReserved { .. }
            | PackError::NotARigor { .. } => None,
        };
        vec![RoutingViolation::PackDoesNotLoad {
            source: source.to_owned(),
            detail: error.to_string(),
            span,
        }]
    })?;

    let mut violations: Vec<RoutingViolation> = Vec::new();
    let mut required: BTreeSet<String> = BTreeSet::new();
    let mut stages_walked = 0_usize;
    let mut rows_walked = 0_usize;
    let mut rows_without_artifacts = 0_usize;
    let mut artifacts_checked = 0_usize;
    let mut routes_checked = 0_usize;

    for (stage, spec) in pack.stage_specs() {
        stages_walked += 1;
        let mut routes: BTreeMap<String, Vec<Route>> = BTreeMap::new();

        // THE BASELINE IS A ROUTE WITH NO CAPABILITY. An artifact the stage lists unconditionally
        // is never reachable through *only* a capability-bearing row, which is what makes
        // AC-495's *"the ONLY route"* false for it.
        let baseline_line = line_of(&spans, &format!("stage.{stage}.baseline"))
            .or_else(|| line_of(&spans, &format!("stage.{stage}")))
            .unwrap_or(1);
        for artifact in &spec.baseline {
            routes.entry(artifact.clone()).or_default().push(Route {
                site: format!("{source}: stage.{stage}.baseline"),
                capability: None,
                row: Span::new(source, baseline_line),
                capability_span: None,
            });
            routes_checked += 1;
        }

        for (i, row) in spec.require.iter().enumerate() {
            rows_walked += 1;
            if row.artifacts.is_empty() {
                rows_without_artifacts += 1;
            }
            let path = format!("stage.{stage}.require[{i}]");
            let row_span = Span::new(source, line_of(&spans, &path).unwrap_or(baseline_line));
            let capability_span =
                line_of(&spans, &format!("{path}.capability")).map(|l| Span::new(source, l));

            let capability = match require_row_capability(&doc, stage, i) {
                RowCapability::Absent => None,
                RowCapability::Token(token) => {
                    required.insert(token.clone());
                    Some(token)
                }
                // REFUSED, NEVER IGNORED. An ignored malformed key is a requirement that silently
                // stops being one, which is the failure `crate::toml_subset`'s header exists to
                // prevent one layer down.
                RowCapability::Malformed(detail) => {
                    violations.push(RoutingViolation::CapabilityNotAToken {
                        site: row.site.clone(),
                        row: row_span,
                        detail,
                    });
                    continue;
                }
            };

            for artifact in &row.artifacts {
                routes.entry(artifact.clone()).or_default().push(Route {
                    site: row.site.clone(),
                    capability: capability.clone(),
                    row: row_span.clone(),
                    capability_span: capability_span.clone(),
                });
                routes_checked += 1;
            }
        }

        // Artifacts in byte order, so the violation list is a function of the pack and not of the
        // order a `HashMap` happened to iterate in.
        for (artifact, routes) in &routes {
            artifacts_checked += 1;
            let routed = routes.iter().any(|route| {
                route
                    .capability
                    .as_ref()
                    .is_none_or(|token| claimed.claims_token(token))
            });
            if routed {
                continue;
            }
            // Every route needs a token nothing claims. The FIRST in file order is the one
            // reported, so the span is stable against a pack that gains a second dead alternative.
            let first = &routes[0];
            let Some(capability) = first.capability.clone() else {
                // Unreachable: a route with no capability is routed by the `any` above.
                continue;
            };
            violations.push(RoutingViolation::StageUnroutable {
                stage: stage.clone(),
                artifact: artifact.clone(),
                capability,
                site: first.site.clone(),
                row: first.row.clone(),
                capability_span: first
                    .capability_span
                    .clone()
                    .unwrap_or_else(|| first.row.clone()),
                routes_considered: routes.len(),
            });
        }
    }

    if violations.is_empty() {
        Ok(RoutingReport {
            stages_walked,
            rows_walked,
            rows_without_artifacts,
            artifacts_checked,
            routes_checked,
            capabilities_required: required.into_iter().collect(),
            claims_walked: claimed.len(),
        })
    } else {
        Err(violations)
    }
}

/// What a row's `capability` key is.
enum RowCapability {
    Absent,
    Token(String),
    Malformed(String),
}

/// The 1-based line a path was read at.
fn line_of(spans: &toml_subset::Spans, path: &str) -> Option<usize> {
    spans.get(path).copied()
}

/// Read one row's `capability` out of the **same** parse the spans came from.
///
/// Indexed by the loader's own `(stage, i)`, so this cannot address a row `crate::select::Pack`
/// does not hold.
fn require_row_capability(doc: &toml_subset::Table, stage: &str, i: usize) -> RowCapability {
    let row = doc
        .get("stage")
        .and_then(Value::as_table)
        .and_then(|stages| stages.get(stage))
        .and_then(Value::as_table)
        .and_then(|body| body.get("require"))
        .and_then(Value::as_array)
        .and_then(|rows| rows.get(i))
        .and_then(Value::as_table);
    let Some(row) = row else {
        return RowCapability::Absent;
    };
    match row.get("capability") {
        None => RowCapability::Absent,
        Some(Value::Str(token)) if !token.trim().is_empty() => RowCapability::Token(token.clone()),
        Some(value) => RowCapability::Malformed(format!("{value:?}")),
    }
}
