//! **Step 7 — deviation.** Design §4.4 row 7:
//!
//! > An activity may return `RequestDeviation { target, kind, claimed_predicate, why }`. The engine
//! > re-evaluates the predicate against evidence: true ⇒ **re-run 2–5 and log the delta**; false ⇒
//! > gate with the request and its evaluated operands attached.
//! >
//! > *Invariant:* **the agent never edits `R`.** `RequiredSet` is constructible only inside the
//! > selection crate and implements no deserialisation, so no agent-returned value can reach it; a
//! > `kind` outside the activity's own `deviation_ok` is refused as a configuration error and never
//! > reaches predicate evaluation.
//!
//! # The three branches, and the one that writes
//!
//! | branch | criterion | what it produces | event |
//! | --- | --- | --- | --- |
//! | `kind` outside `deviation_ok` | **AC-062** | [`DeviationOutcome::Refused`] — a configuration error, **before** predicate evaluation | **none** |
//! | `claimed_predicate` → `false` | **AC-063** | [`DeviationOutcome::Gated`] with the request, the predicate and its evaluated operands; `R` unchanged | **none** |
//! | `claimed_predicate` → `true` | **AC-064** | [`DeviationOutcome::Accepted`] — **steps 2–5 re-run** and the delta is named | **one `dag_amended`** |
//!
//! ADR-0120 decision 5 states the two silences in terms, and this module's shape is what enforces
//! them: only [`DeviationOutcome::Accepted`] carries a [`DagAmended`], and
//! [`DeviationOutcome::event`] reads it off that variant alone.
//!
//! # `null` is a fourth outcome and it is NOT the accepted branch
//!
//! AC-064's GIVEN is *"evaluates **true**"* and AC-063's is *"evaluates **false**"*. Neither names
//! the abstention, and **ADR-0089 forbids coercing it to `false`** — the absence of evidence never
//! resolves to `false`. An abstaining `claimed_predicate` therefore **gates**, carrying
//! [`crate::predicate::Tri::Null`] so a reader of the gate record can tell an abstention from a
//! refutation, and writes no event. That reading is stated here because no record states it, and it
//! is the conservative one: the alternative, treating `null` as true, would amend `R` on no
//! evidence.
//!
//! # `deviation_id` — the register's 2026-09-13 amendment to AC-064, executed
//!
//! > `deviation_id = sha256_lp([target, kind, claimed_predicate])` — the SHA-256 of the
//! > **length-prefixed tuple** of exactly those three components, **in that order**, which is the
//! > declaration order AC-061's THEN and design §4.4 row 7 both give. … **The bytes of each
//! > component are that field's value under the canonical serialiser.** … **`why` is EXCLUDED, and
//! > that is a decision rather than an omission.**
//!
//! The stated cost travels with it: *"two deviations that differ **only** in `why` are
//! indistinguishable **by id**"*. That is asserted as a property in
//! `tests/it/deviation.rs::deviation_id_is_the_content_hash_of_the_request`, not left as prose.
//!
//! **No shared key helper is written** — ADR-0119 clause 6 governs the reuse and says why: *"sharing
//! the length-prefixed **primitive** is not sharing a key derivation, and a helper that computed
//! 'the key' is the thing that would make two of them accidentally equal."*
//!
//! # The append is OUTSIDE the pure function
//!
//! ADR-0120 decision 4: *"the delta is **returned** by the re-run and the kernel's step-7 handler
//! appends the event through the `EventStore` port."* [`DeviationOutcome::event`] **constructs** the
//! event and hands it back; nothing in this module takes a store, a port or an `EventStore`, and
//! there is no parameter one could arrive through.

use std::fmt;

use super::artifacts::{ArtifactRegistry, Instance};
use super::canonical::{CanonError, Digest};
use super::item::FacetValue;
use super::{Catalog, Item, Pack, SelectError, Selection};
use crate::catalog::Activity;
use crate::predicate::{self, Tri};
use crate::registry::Registry;

/// ADR-0046's closed vocabulary: *"a `deviate` declaration whose `kind` falls outside the closed
/// `skip | downgrade | add` vocabulary"* is forbidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeviationKind {
    /// The target kind is not required in this run.
    Skip,
    /// The target kind's producer is resolved one live rigor level lower.
    Downgrade,
    /// The target kind is required in addition to what the pack's rows produced.
    Add,
}

impl DeviationKind {
    /// The three, for the per-kind enumeration AC-062's *Verified by* names.
    pub const ALL: [Self; 3] = [Self::Skip, Self::Downgrade, Self::Add];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Skip => "skip",
            Self::Downgrade => "downgrade",
            Self::Add => "add",
        }
    }

    /// Parse one `deviation_ok` token. `None` for anything outside the three — the catalog loader
    /// already refuses a fourth, so this is the second reading of the same closed set.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "skip" => Some(Self::Skip),
            "downgrade" => Some(Self::Downgrade),
            "add" => Some(Self::Add),
            _ => None,
        }
    }
}

impl fmt::Display for DeviationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// **The one selection-affecting variant an activity may return** (AC-061's THEN).
///
/// It carries no required-artifact set, no rigor override and no producer choice, and it derives no
/// `Deserialize` route into [`super::RequiredSet`] — the agent hands over a *request*, and every
/// change to `R` is made by step 2 re-running over it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestDeviation {
    /// The artifact kind (ADR-0046: `target = <ArtifactKind>`).
    pub target: String,
    pub kind: DeviationKind,
    pub claimed_predicate: String,
    /// The justification. **Not part of [`Self::deviation_id`]** — AC-064's 2026-09-13 amendment
    /// clause 2, with its cost stated there.
    pub why: String,
}

impl RequestDeviation {
    #[must_use]
    pub fn new(target: &str, kind: DeviationKind, claimed_predicate: &str, why: &str) -> Self {
        Self {
            target: target.to_owned(),
            kind,
            claimed_predicate: claimed_predicate.to_owned(),
            why: why.to_owned(),
        }
    }

    /// `sha256_lp([target, kind, claimed_predicate])` under `rk_store::canon`'s bytes.
    ///
    /// **Derived, never stored.** A counter, a UUID or a timestamp would need a clock, a random
    /// source or the store, all three of which AC-041 forbids at compile time — and replay could
    /// then only read the id back rather than recompute it, which is what would make an amended DAG
    /// a hidden input.
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the canonical serialiser. All three components are strings
    /// today; naming the canonical form anyway is what stops a component that later becomes
    /// structured from acquiring a serialisation chosen at whichever call site reaches it first.
    pub fn deviation_id(&self) -> Result<Digest, CanonError> {
        let target = super::canonical::canonical(&serde_json::Value::from(self.target.clone()))?;
        let kind = super::canonical::canonical(&serde_json::Value::from(self.kind.as_str()))?;
        let claimed =
            super::canonical::canonical(&serde_json::Value::from(self.claimed_predicate.clone()))?;
        Ok(rk_types::digest::sha256_lp(&[&target, &kind, &claimed]))
    }

    /// The request as the golden log records it. **Public because AC-064's second AND requires it
    /// to be**: *"the deviation record is part of the replay input set"*, so the log must be able
    /// to write one and read it back.
    #[must_use]
    pub fn to_json_public(&self) -> serde_json::Value {
        self.to_json()
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "target": self.target,
            "kind": self.kind.as_str(),
            "claimed_predicate": self.claimed_predicate,
            "why": self.why,
        })
    }
}

/// **AC-062.** A `kind` outside the requesting activity's own `deviation_ok`.
///
/// *"The request is refused as a configuration error and logged; it does not reach predicate
/// evaluation and does not open a deviation gate."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationError {
    pub activity_id: String,
    pub requested: DeviationKind,
    /// What the activity **does** permit, so the repair is readable from the refusal.
    pub deviation_ok: Vec<DeviationKind>,
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let allowed: Vec<&str> = self.deviation_ok.iter().map(|k| k.as_str()).collect();
        write!(
            f,
            "{}: `{}` is not in its own `deviation_ok` {:?}. This is a CONFIGURATION error, not a \
             judgement: the request does not reach predicate evaluation and opens no gate \
             (AC-062). The deviation channel becomes control flow by the back door exactly when \
             this check is skipped",
            self.activity_id,
            self.requested.as_str(),
            allowed
        )
    }
}

/// **AC-063's gate.** *"A gate opens with the request, the predicate and its evaluated operands
/// attached."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviationGate {
    pub request: RequestDeviation,
    /// [`Tri::False`] for AC-063's own case, [`Tri::Null`] for an abstention. **Never coerced.**
    pub value: Tri,
    /// The evaluated value of each facet the predicate read, typed in rather than re-derived — the
    /// same rule §7b.9(4) applies to [`super::Unsatisfiable`], for the same reason.
    pub operands: Vec<(String, FacetValue)>,
}

impl DeviationGate {
    /// The `gate_opened` payload. **Returned, never appended** — the append is the runtime
    /// boundary's.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "reason": "deviation_refused",
            "request": self.request.to_json(),
            "value": match self.value {
                Tri::True => "true",
                Tri::False => "false",
                Tri::Null => "null",
            },
            "operands": self.operands.iter().map(|(n, v)| serde_json::json!({
                "facet": n,
                "value": v.to_json(),
            })).collect::<Vec<_>>(),
        })
    }
}

/// **AC-064's payload**, as ADR-0120 decision 1 fixes its four names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DagAmended {
    pub deviation_id: Digest,
    /// Instances in the re-run's `R` that were not in the base's.
    pub added: Vec<Instance>,
    /// Instances in the base's `R` that are not in the re-run's.
    pub removed: Vec<Instance>,
    /// **A reading, stated because no record fixes it.** The register names the field and not its
    /// meaning; ADR-0120 decision 1 says *"the payload is the register's and is taken as given …
    /// This record fixes the four names and their obligation; it does not fix their Rust types."*
    /// Taken here as **the number of `require` rows that fired in the re-run** — the only quantity
    /// under that name the re-run produces — so a reader can see how much of the amended `R` came
    /// from rows rather than from the deviation itself.
    pub rows_refired: usize,
}

impl DagAmended {
    /// The event, **constructed here and appended outside** (ADR-0120 decision 4).
    #[must_use]
    pub fn event(&self) -> rk_types::event::Event {
        rk_types::event::Event::new(rk_types::event_kind::EventKind::DagAmended, self.to_json())
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "deviation_id": self.deviation_id.as_str(),
            "added": self.added.iter().map(Instance::to_json).collect::<Vec<_>>(),
            "removed": self.removed.iter().map(Instance::to_json).collect::<Vec<_>>(),
            "rows_refired": self.rows_refired,
        })
    }
}

/// What step 7 did with the request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviationOutcome {
    /// AC-062. No gate, no predicate evaluation, no event.
    Refused(ConfigurationError),
    /// AC-063 (and the abstention). A gate, `R` unchanged, no event.
    Gated(Box<DeviationGate>),
    /// AC-064. Steps 2–5 re-ran; the delta is named and **one** `dag_amended` is owed.
    Accepted {
        amended: DagAmended,
        selection: Box<Selection>,
    },
}

impl DeviationOutcome {
    /// **The one event, on the one branch that writes it.**
    ///
    /// ADR-0120 decision 5: a `false` predicate is AC-063's gate and a refused `kind` is AC-062's
    /// configuration error, and *"only the true branch amends `R`, and only the true branch
    /// writes"*. This method is where that is a shape rather than a rule — the other two variants
    /// carry no [`DagAmended`] to read one off.
    #[must_use]
    pub fn event(&self) -> Option<rk_types::event::Event> {
        match self {
            Self::Refused(_) | Self::Gated(_) => None,
            Self::Accepted { amended, .. } => Some(amended.event()),
        }
    }

    /// The delta, on the accepted branch.
    #[must_use]
    pub const fn amended(&self) -> Option<&DagAmended> {
        match self {
            Self::Refused(_) | Self::Gated(_) => None,
            Self::Accepted { amended, .. } => Some(amended),
        }
    }

    /// The re-run's selection, on the accepted branch. **The base selection is never mutated** —
    /// this is a second value, which is what *"deviation never edits `R`"* means operationally.
    #[must_use]
    pub fn selection(&self) -> Option<&Selection> {
        match self {
            Self::Refused(_) | Self::Gated(_) => None,
            Self::Accepted { selection, .. } => Some(selection),
        }
    }
}

/// The amendment the re-run of steps 2–5 is given. **Not constructible from a deserialised value**:
/// it is built here from a `RequestDeviation` that step 7 has already accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::select) struct Accepted {
    pub kind: DeviationKind,
    pub target: String,
}

/// **Step 7.**
///
/// `base` is the selection the request arrived against; it is **read and never written**, and the
/// accepted branch returns a *second* [`Selection`] produced by re-running steps 2–5.
///
/// # Errors
///
/// [`SelectError`] when the `claimed_predicate` is outside the closed vocabulary (AC-065 names a
/// `claimed_predicate` as one of its three sites) or when the re-run itself hits a pack defect.
/// **A false predicate and a refused `kind` are not errors** — they are outcomes.
#[expect(
    clippy::too_many_arguments,
    reason = "the four digests AC-041 names reach the pure function as arguments and nothing else \
              does; collapsing them into a context struct would put a constructor between the \
              criterion and the call"
)]
pub fn step7(
    pack: &Pack,
    catalog: &Catalog,
    artifacts: &ArtifactRegistry,
    facets: &Registry,
    stage: &str,
    item: &Item,
    requesting_activity: &str,
    request: &RequestDeviation,
    base: &Selection,
) -> Result<DeviationOutcome, SelectError> {
    // ---- AC-062, FIRST, and before anything reads the predicate --------------------------------
    let activity: Option<&Activity> = catalog
        .activities()
        .iter()
        .find(|a| a.id == requesting_activity);
    let allowed: Vec<DeviationKind> = activity
        .map(|a| {
            a.deviation_ok
                .iter()
                .filter_map(|t| DeviationKind::from_token(t))
                .collect()
        })
        .unwrap_or_default();
    if !allowed.contains(&request.kind) {
        return Ok(DeviationOutcome::Refused(ConfigurationError {
            activity_id: requesting_activity.to_owned(),
            requested: request.kind,
            deviation_ok: allowed,
        }));
    }

    // ---- the predicate, over the closed vocabulary (AC-065's third site) ------------------------
    let site = format!("{requesting_activity}: claimed_predicate");
    super::step2::lint_selection_predicate(&site, &request.claimed_predicate, facets)
        .map_err(super::Step2Error::Judgement)?;
    let parsed =
        predicate::parse(&request.claimed_predicate).map_err(|e| super::Step2Error::Parse {
            site: site.clone(),
            detail: e.to_string(),
        })?;
    let ctx = item.context();
    let outcome = predicate::evaluate(&parsed.expr, &ctx).map_err(|e| super::Step2Error::Eval {
        site: site.clone(),
        detail: e.to_string(),
    })?;

    // ---- AC-063, and the abstention ------------------------------------------------------------
    if outcome.value != Tri::True {
        let facet_values = item.narrowed_facets(&Instance::of(&request.target));
        return Ok(DeviationOutcome::Gated(Box::new(DeviationGate {
            request: request.clone(),
            value: outcome.value,
            operands: outcome
                .read_facets
                .iter()
                .map(|name| {
                    (
                        name.clone(),
                        facet_values.get(name).cloned().unwrap_or(FacetValue::Null),
                    )
                })
                .collect(),
        })));
    }

    // ---- AC-064: steps 2-5 RE-RUN. `R` is rebuilt, never edited --------------------------------
    let accepted = Accepted {
        kind: request.kind,
        target: request.target.clone(),
    };
    let rerun = super::select_amended(
        pack,
        catalog,
        artifacts,
        facets,
        stage,
        item,
        Some(&accepted),
    )?;

    let before: Vec<Instance> = base.required().kinds().cloned().collect();
    let after: Vec<Instance> = rerun.required().kinds().cloned().collect();
    let added: Vec<Instance> = after
        .iter()
        .filter(|i| !before.contains(i))
        .cloned()
        .collect();
    let removed: Vec<Instance> = before
        .iter()
        .filter(|i| !after.contains(i))
        .cloned()
        .collect();

    Ok(DeviationOutcome::Accepted {
        amended: DagAmended {
            deviation_id: request.deviation_id()?,
            added,
            removed,
            rows_refired: rerun.rows_fired(),
        },
        selection: Box::new(rerun),
    })
}
