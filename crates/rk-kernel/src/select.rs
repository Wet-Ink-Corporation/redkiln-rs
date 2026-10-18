//! The pure selection function — design §4.4, *"Selection is a pure function of four digests, in
//! seven steps"* (`references/2026-09-08-architecture-design.md`:503-515).
//!
//! ```text
//! select() is a function of (pack digest, catalog digest, context digest, artifact digests)
//! and of nothing else                              [documented, catalog §2.5; ADR-0024]
//! ```
//!
//! # WHAT IS BUILT HERE, AND WHAT EACH STEP OWES
//!
//! **All seven.** The entry point is [`select`] — steps 1 through 5 — because that is what a
//! function returning a validated graph can be. Step **6** is [`Selection::plan`], which produces
//! the topological order and does **not** run it: dispatch is the runtime's (Slice 4). Step **7** is
//! [`step7`], which the runtime calls with a `RequestDeviation` an activity returned.
//!
//! | step | this module | criteria |
//! | --- | --- | --- |
//! | 1 · rigor | [`step1`] | AC-050, AC-051 (stage half), AC-077 (with steps 2/4) |
//! | 2 · required set | [`step2`] | AC-049, AC-065, AC-079 |
//! | 3 · short-circuit | [`step3`] | AC-066 (armed), AC-067, AC-525's classification half |
//! | 4 · producer | [`step4`], [`rank_lint`] | AC-046, AC-047, AC-048 (ceiling), AC-052, AC-054 |
//! | 5 · validate | [`step5`], [`lints`] | AC-058, AC-078; AC-053/AC-057/AC-059/AC-060 at load |
//! | 6 · execute | [`step6`] | **none — the design records step 6 as uncovered**; see that module |
//! | 7 · deviation | [`step7`] | AC-061, AC-062, AC-063, AC-064; ADR-0046, ADR-0120 |
//!
//! **What is still not built, and is not stubbed**: the kernel runtime that *walks* the plan, the
//! memo, the budgets, retry and the breaker (§7b.10, Slice 4). Nothing here dispatches, and no
//! module in this tree stands in for one — *"a module that exists and returns nothing reads as
//! built to the next reader"* (`00-preamble.md` golden constraint 10).
//!
//! # Purity is enforced here, not asserted — AC-041
//!
//! This module carries `#![deny(clippy::disallowed_methods)]` and
//! `#![deny(clippy::disallowed_types)]`, exactly as [`crate::workflow`] does, so the wall clock, a
//! monotonic instant, randomness, a blocking sleep and an unordered map are refused inside it by
//! the root `clippy.toml`'s tables. **A declared attribute and a biting one look identical to a
//! grep**, so the bite is committed as a positive control behind a non-default feature:
//!
//! ```text
//! green:  cargo clippy --workspace --all-targets
//! RED:    cargo clippy --workspace --all-targets --features rk-kernel/select-clock-reintroduce
//! ```
//!
//! Nothing reachable from [`select`] opens a file, reads an environment variable, draws a random
//! number or reads a clock — **and the four halves of that sentence are not all enforced by the
//! same thing, which is stated here because for one day they were all four reported as if they
//! were.** Measured 2026-09-13 on the tree this note was written against:
//!
//! | ban | what enforces it | shown to bite by |
//! | --- | --- | --- |
//! | clock, monotonic instant, randomness, blocking sleep, unordered map | `clippy.toml`'s tables + the `#![deny]` above | `--features rk-kernel/select-clock-reintroduce` (clippy rc=101) |
//! | **a filesystem read** | `selection_source_reads_no_hidden_input` (`tests/it/selection.rs`) | its committed fixture `tests/fixtures/purity/banned-spellings.rs.txt` |
//! | **an environment read** | the same test | the same fixture |
//! | **an event-log / store read** (AC-525) | the same test | the same fixture |
//!
//! **`clippy.toml` does not and cannot cheaply cover the last three.** It has no path or module
//! scoping (clippy.toml:1-3), so `std::fs::read_to_string` or `std::env::var` in its
//! `disallowed-methods` table would be workspace-wide, and every legitimate read in `rk-git`,
//! `rk-store` and this crate's own [`crate::catalog::Catalog::from_dir`] would need an `#[expect]` —
//! editing two crates whose checkpoints have already passed. Measured before the structural test
//! existed: an `std::env::var` that CHANGED the selected stage, an `std::fs::read_to_string`, and an
//! `rk_store::conn::Reader::open` each clipped clean and left all 59 tests green. All three are red
//! now.
//!
//! In particular [`select`] **never** calls [`crate::catalog::Catalog::from_dir`], which does use
//! `std::fs`: the caller loads the catalog and hands it in, so the filesystem read happens at the
//! runtime boundary and not inside the pure function. **And it never reads the event log** —
//! AC-525's 2026-09-13 amendment clause 1 says so in terms. Two things enforce that: no function
//! here takes a store, a port or an `EventStore` (so there is no parameter one could arrive
//! through), **and** the source scan above, which is what covers the case the type signature cannot
//! — D3 put `rk-store` on this crate's dependency edge for `rk_store::canon`, so every other module
//! of that crate is one free function call away from inside here.
//!
//! # Canonical serialisation is `rk_store::canon` and nothing else — D3, §7b.9(2)
//!
//! *"Byte-identical"* in AC-040 is identity **under the canonical serialiser**, which already
//! exists at `crates/rk-store/src/canon.rs`: object keys sorted byte-wise, LF only, no trailing
//! newline, floats refused. This crate does **not** write a second one. `serde_json` carries
//! `preserve_order` workspace-wide, so a plain `to_vec` over an insertion-ordered map leaks
//! construction order into AC-040's assertion **and can still pass its 100 shuffled repetitions**
//! if the shuffle is of call order rather than of map construction.
//!
//! # The rank rule, stated once
//!
//! Among admissible producers of a kind the **lowest `method_rank`** wins. **There is no runtime
//! tie-break, of any kind, for any artifact kind**: an equal-rank pair simultaneously admissible in
//! any enumerated context is a **pack validation error at load** (ADR-0056; AC-047). **Specificity
//! is withdrawn** from producer-of-a-kind selection (ADR-0073; AC-047's *Cut* removes the absence
//! claim as a criterion, not the design). The mixed-toolchain case never contends for one slot: it
//! is resolved earlier, at step 2, by **keyed expansion** — one instance per element of the item's
//! `toolchain` set, one producer per instance (ADR-0073, AC-049).
//!
//! **The binder's specificity rule is a different rule and must not become this code path.** AC-087
//! ranks agent-card *selectors* by subsumption and survives; AC-088 withdraws specificity for
//! producers. Nothing in this module ranks by specificity, so there is no shared path to scope.
//!
//! # `RequiredSet` is constructible only inside this module
//!
//! AC-061's second AND: *"`RequiredSet` is constructible only inside the selection crate and
//! implements no deserialisation, so no agent-returned value can reach it."* ADR-0099 clause 5
//! makes *"inside the selection crate"* mean this module, so [`RequiredSet`]'s constructor is
//! `pub(in crate::select)` and the type derives no `Deserialize`. A `pub(crate)` constructor, or a
//! `#[cfg(test)]` public one, would silently convert AC-061 from a type fact into a convention.
//! The `R`-equality assertions therefore live in this module under `#[cfg(test)]` rather than in
//! `tests/it/`.

#![deny(clippy::disallowed_methods)]
#![deny(clippy::disallowed_types)]

pub mod artifacts;
pub mod canonical;
pub mod item;
pub mod lints;
pub mod pack;
pub mod step1;
pub mod step2;
pub mod step3;
pub mod step4;
pub mod step5;
pub mod step6;
pub mod step7;

use std::collections::BTreeMap;

pub use artifacts::{ArtifactKind, ArtifactRegistry, Instance, KindSource, RegistryError};
pub use canonical::{CanonError, Digest, digest_of};
pub use item::{ArtifactState, FacetValue, Item};
pub use lints::{
    DeadContentReport, dangling_requires_lint, dead_content_report, excludes_lint, load_lints,
    requires_cycle_lint,
};
pub use pack::{Pack, PackError, RequireRow, StageSpec};
pub use step1::{OverrideRow, RigorSource, SkipReason, SkipRow, Step1Outcome, admits, step1};
pub use step2::{
    GateOpened, GateReason, JudgementPredicate, RequiredSet, Step2Error, Step2Outcome,
    lint_selection_predicate, step2,
};
pub use step3::{Classification, DriftSignal, Step3Outcome, step3};
pub use step4::{
    Abstention, Candidate, FailedPredicate, LoadError, Picked, UnsatReason, Unsatisfiable,
    rank_lint, step4,
};
pub use step5::{
    AppetiteGate, Dag, DagEdge, DagNode, Step5Error, Step5Outcome, cost_points, step5,
};
pub use step6::{ExecutionPlan, step6};
pub use step7::{
    ConfigurationError, DagAmended, DeviationGate, DeviationKind, DeviationOutcome,
    RequestDeviation, step7,
};

/// Re-exported so a caller of this module does not have to reach into two of them for the one
/// value it must hand in. The catalog is **loaded by the caller**: [`crate::catalog::Catalog::from_dir`]
/// reads the filesystem and must not be reachable from the pure function (AC-041).
pub use crate::catalog::Catalog;

/// The AC-041 must-fail control: exactly one banned determinism call, behind a non-default feature.
///
/// If this function ever clips **clean** with `--features rk-kernel/select-clock-reintroduce`, the
/// module-level deny above is declared and does not bite, and AC-041's `rk-kernel` half is pinned
/// by nothing. It is committed rather than reverted on `tests/fixtures/rk-clock-read/`'s precedent
/// (DECISIONS-2026-09-12 E4): a reverted observation is not an assertion.
#[cfg(feature = "select-clock-reintroduce")]
#[must_use]
pub fn reads_the_wall_clock() -> std::time::SystemTime {
    std::time::SystemTime::now()
}

/// One producer chosen for one required instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedProducer {
    pub instance: Instance,
    pub activity_id: String,
    pub method_rank: i64,
}

/// What stopped selection before step 4 could gate anything.
///
/// **Unsatisfiability is not here.** §7b.9(4): step 4's unsatisfiability is **data in the DAG** — a
/// gate node — not an abort, because AC-058's gate *"carr[ies] the DAG"* and AC-054 forbids
/// dropping the kind from `R`. So [`select`] returns `Ok` with the kind gated. What
/// *is* here is the class of failure that is a **pack defect**: a rank tie, a required kind the
/// registry does not know, a predicate outside the closed vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectError {
    Step2(Step2Error),
    Load(LoadError),
    Canon(CanonError),
    /// A graph the assembler could not validate — a cycle (AC-078), an unbound edge, an
    /// unreachable member of `R`, or a mutually excluding pair. **Not the appetite**: an
    /// over-budget DAG is a gate carrying the graph (AC-058), not an error.
    Step5(Step5Error),
}

impl std::fmt::Display for SelectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Step2(e) => write!(f, "{e}"),
            Self::Load(e) => write!(f, "{e}"),
            Self::Canon(e) => write!(f, "{e}"),
            Self::Step5(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SelectError {}

impl From<Step5Error> for SelectError {
    fn from(e: Step5Error) -> Self {
        Self::Step5(e)
    }
}

impl From<Step2Error> for SelectError {
    fn from(e: Step2Error) -> Self {
        Self::Step2(e)
    }
}

impl From<LoadError> for SelectError {
    fn from(e: LoadError) -> Self {
        Self::Load(e)
    }
}

impl From<CanonError> for SelectError {
    fn from(e: CanonError) -> Self {
        Self::Canon(e)
    }
}

/// Everything the seven steps produced, for one item at one stage.
///
/// **Steps 1-5.** Step 6 is [`Selection::plan`] (the order, not the running of it) and step 7 is
/// [`step7`], which takes a `Selection` and returns a *second* one — it never writes to this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    stage: String,
    rigor: crate::rigor::Rigor,
    required: RequiredSet,
    classifications: BTreeMap<Instance, Classification>,
    producers: Vec<SelectedProducer>,
    gates: Vec<GateOpened>,
    skipped: Vec<SkipRow>,
    unsatisfiable: Vec<Unsatisfiable>,
    drift: Vec<DriftSignal>,
    rows_fired: usize,
    step5: Step5Outcome,
    inputs_digest: Digest,
    /// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT; R1 = ADR-0191 decision 1,
    /// proposed.)* The required instances whose kind is registered `source = "pass"` and is not
    /// `Present`: no catalog producer was selected for them, and the stage is not complete
    /// until the pass's `artifact.write` materialises each. Derived from `R`, the step-3
    /// classifications and the registry, so it is not part of the canonical bytes.
    pass_sourced: Vec<Instance>,
    /// *(dated 2026-10-18, s8j build, package S8J-C2-ROOT-QUESTIONS; ADR-0200 decision 2,
    /// proposed.)* The kinds of [`Self::unsatisfiable`] with `NoAdmissibleProducer` that the
    /// artifact registry declares `source = "human"`, sorted and once each: the roots only an
    /// answer can supply. Derived from the step-4 list and the registry, so it is not part of the
    /// canonical bytes.
    human_roots: Vec<String>,
}

impl Selection {
    /// The human-sourced kinds no activity can produce here (ADR-0200 decision 2): each is the
    /// `<Kind>` of a `human_root:<Kind>` gate the FSM's step 2 opens. A `ForbiddenStage`
    /// unsatisfiable and an activity-sourced kind are never in it.
    #[must_use]
    pub fn human_roots(&self) -> &[String] {
        &self.human_roots
    }

    /// The required, pass-sourced instances still `Missing` or `Stale` (R1 = ADR-0191 decision
    /// 1): what a composition-root pass owes before the stage is complete.
    #[must_use]
    pub fn pass_sourced(&self) -> &[Instance] {
        &self.pass_sourced
    }

    /// `R`, as step 2 built it and as no later step may edit.
    #[must_use]
    pub const fn required(&self) -> &RequiredSet {
        &self.required
    }

    /// `R` under the canonical serialiser — the comparison AC-058, AC-063 and §7b.4:431's
    /// `R`-equality assertions are all made under, rather than three different ones.
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the serialiser, which `rk-store` refuses by decision.
    pub fn required_canonical_bytes(&self) -> Result<Vec<u8>, CanonError> {
        canonical::canonical(&self.required.to_json())
    }

    /// The rigor in force at this stage (step 1).
    #[must_use]
    pub const fn rigor(&self) -> crate::rigor::Rigor {
        self.rigor
    }

    /// The producer chosen for each instance step 3 did not classify `Present`.
    pub fn producers(&self) -> impl Iterator<Item = &SelectedProducer> {
        self.producers.iter()
    }

    /// Step 3's classification per required instance.
    #[must_use]
    pub const fn classifications(&self) -> &BTreeMap<Instance, Classification> {
        &self.classifications
    }

    /// `gate_opened { reason: unknown_facet, axis }` rows, one per read facet (AC-079).
    #[must_use]
    pub fn gates(&self) -> &[GateOpened] {
        &self.gates
    }

    /// The rows step 5 carries as `dag_assembled.skipped`. **A forbidden stage is a logged skip,
    /// not an absence** (design §4.4 row 1; AC-077).
    #[must_use]
    pub fn skipped(&self) -> &[SkipRow] {
        &self.skipped
    }

    /// The kinds gated `Unsatisfiable`. Nothing was dropped from `R` to produce this list.
    #[must_use]
    pub fn unsatisfiable(&self) -> &[Unsatisfiable] {
        &self.unsatisfiable
    }

    /// Drift **signals** for vouched roots (AC-067). These are returned, never appended: the
    /// derivation of `drift_advisory`, the *"exactly one"* and the append are the runtime
    /// boundary's (AC-525's 2026-09-13 amendment, clauses 2 and 3).
    #[must_use]
    pub fn drift(&self) -> &[DriftSignal] {
        &self.drift
    }

    /// How many `require` rows fired at step 2 — `dag_assembled.rows_fired`, and the quantity
    /// [`DagAmended::rows_refired`] reads on a re-run.
    #[must_use]
    pub const fn rows_fired(&self) -> usize {
        self.rows_fired
    }

    /// Step 5's outcome, both variants of which carry a [`Dag`].
    #[must_use]
    pub const fn step5(&self) -> &Step5Outcome {
        &self.step5
    }

    /// The assembled DAG, whether or not the appetite gated.
    #[must_use]
    pub const fn dag(&self) -> &Dag {
        self.step5.dag()
    }

    /// **AC-058's gate**, when `Σ cost_class` exceeded the item's declared appetite. `None` when it
    /// did not, and `None` when the pack declared no appetite at all — which is **not** the same as
    /// an unbounded one, and [`Dag::appetite`] is where the difference is legible.
    #[must_use]
    pub fn appetite_gate(&self) -> Option<&AppetiteGate> {
        self.step5.gate()
    }

    /// **Step 6.** The topological order, as antichain layers.
    ///
    /// # Errors
    ///
    /// [`Step5Error::CyclicDag`]. An assembled DAG has already been validated acyclic by step 5, so
    /// this cannot fire on a `Selection` this module produced; it can on a hand-assembled one, and
    /// that is AC-078's case.
    pub fn plan(&self) -> Result<ExecutionPlan, Step5Error> {
        step6(self.dag())
    }

    /// `sha256_lp` over the four digests, in AC-041's order: pack, catalog, context, artifacts.
    #[must_use]
    pub const fn inputs_digest(&self) -> &Digest {
        &self.inputs_digest
    }

    /// **The event journal this selection owes**, in the order a run appends it.
    ///
    /// `dag_assembled` always; one `gate_opened` per abstaining read facet (AC-079); one per
    /// `Unsatisfiable` kind (AC-054's typed gate); one for the appetite when it gated (AC-058).
    ///
    /// **`drift_advisory` is NOT here** — AC-525's 2026-09-13 amendment clause 2 puts its
    /// derivation and its append at the runtime boundary, and [`Selection::drift`] returns the
    /// signals for that boundary to work from. **Returned, never appended:** nothing in this module
    /// takes a store or a port.
    #[must_use]
    pub fn journal(&self) -> Vec<rk_types::event::Event> {
        let mut events = vec![self.dag().assembled_event()];
        for gate in &self.gates {
            events.push(rk_types::event::Event::new(
                rk_types::event_kind::EventKind::GateOpened,
                gate.to_json(),
            ));
        }
        for u in &self.unsatisfiable {
            events.push(rk_types::event::Event::new(
                rk_types::event_kind::EventKind::GateOpened,
                serde_json::json!({ "reason": "unsatisfiable", "unsatisfiable": u.to_json() }),
            ));
        }
        if let Some(gate) = self.appetite_gate() {
            events.push(rk_types::event::Event::new(
                rk_types::event_kind::EventKind::GateOpened,
                gate.to_json(),
            ));
        }
        events
    }

    /// The selection under `rk_store::canon` — the one canonical form (§7b.9(2)).
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the serialiser, which `rk-store` refuses by decision.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CanonError> {
        canonical::canonical(&self.to_json())
    }

    fn to_json(&self) -> serde_json::Value {
        let mut o = serde_json::Map::new();
        o.insert("stage".into(), self.stage.clone().into());
        o.insert("rigor".into(), self.rigor.as_str().into());
        o.insert("required".into(), self.required.to_json());
        o.insert(
            "classifications".into(),
            serde_json::Value::Array(
                self.classifications
                    .iter()
                    .map(|(i, c)| {
                        serde_json::json!({ "instance": i.to_json(), "class": c.as_str() })
                    })
                    .collect(),
            ),
        );
        o.insert(
            "producers".into(),
            serde_json::Value::Array(
                self.producers
                    .iter()
                    .map(|p| {
                        serde_json::json!({
                            "instance": p.instance.to_json(),
                            "activity_id": p.activity_id,
                            "method_rank": p.method_rank,
                        })
                    })
                    .collect(),
            ),
        );
        o.insert(
            "gates".into(),
            serde_json::Value::Array(self.gates.iter().map(GateOpened::to_json).collect()),
        );
        o.insert(
            "skipped".into(),
            serde_json::Value::Array(self.skipped.iter().map(SkipRow::to_json).collect()),
        );
        o.insert(
            "unsatisfiable".into(),
            serde_json::Value::Array(
                self.unsatisfiable
                    .iter()
                    .map(Unsatisfiable::to_json)
                    .collect(),
            ),
        );
        o.insert(
            "drift".into(),
            serde_json::Value::Array(self.drift.iter().map(DriftSignal::to_json).collect()),
        );
        o.insert("dag".into(), self.dag().to_json());
        o.insert(
            "appetite_gate".into(),
            self.appetite_gate()
                .map_or(serde_json::Value::Null, AppetiteGate::to_json),
        );
        o.insert("rows_fired".into(), self.rows_fired.into());
        o.insert("inputs_digest".into(), self.inputs_digest.as_str().into());
        serde_json::Value::Object(o)
    }
}

/// **`select()` — steps 1 through 5**, for one item at one stage.
///
/// The four inputs AC-041 names are all arguments: the pack, the catalog, the registries and the
/// item (which carries the context vector **and** the artifact digests). Nothing else is read.
///
/// Step **6** is [`Selection::plan`] — the topological order, which the runtime walks — and step
/// **7** is [`step7`], which takes what this returned and returns a *second* `Selection` built by
/// re-running 2-5. Neither writes to what this produced, and that is the whole of *"the agent never
/// edits `R`"*.
///
/// # Errors
///
/// [`SelectError`] for a pack defect — a rank tie (AC-047), a required kind outside
/// `artifacts.toml`, a predicate outside the closed vocabulary (AC-065) — or a graph step 5 could
/// not validate (a cycle, an unbound edge, an unreachable requirement, a mutually excluding pair).
///
/// **Two things are NOT errors**, and both are data in the returned value: an unsatisfiable kind is
/// a gate in [`Selection::unsatisfiable`] with the rest of the graph intact (§7b.9(4); AC-054,
/// AC-058), and an over-budget DAG is [`Selection::appetite_gate`] carrying the whole graph
/// (AC-058).
pub fn select(
    pack: &Pack,
    catalog: &Catalog,
    artifacts: &ArtifactRegistry,
    facets: &crate::registry::Registry,
    stage: &str,
    item: &Item,
) -> Result<Selection, SelectError> {
    select_amended(pack, catalog, artifacts, facets, stage, item, None)
}

/// One live rigor level down, with `optional` as the floor.
///
/// **`discouraged` and `forbidden` are not below `optional` for this purpose** — the first is
/// RESERVED in Phase 1 (AC-050) and the second means nothing in that stage runs (ADR-0057), so a
/// `downgrade` that reached either would turn a scope request into a stage refusal. The floor is
/// stated here because no record states it and the alternative is a silent one.
const fn downgrade_one(rigor: crate::rigor::Rigor) -> crate::rigor::Rigor {
    use crate::rigor::Rigor;
    match rigor {
        Rigor::Mandatory => Rigor::Recommended,
        Rigor::Recommended | Rigor::Optional => Rigor::Optional,
        Rigor::Discouraged | Rigor::Forbidden => rigor,
    }
}

/// Steps 1-5 with an accepted deviation applied at the re-entry point design §4.4 row 7 names.
///
/// `amendment` is `None` for a first pass and `Some` only on step 7's accepted branch. It is
/// `pub(in crate::select)` because [`step7::Accepted`] is: an agent-supplied value cannot reach
/// this parameter, which is the same guarantee [`RequiredSet`]'s constructor carries.
pub(in crate::select) fn select_amended(
    pack: &Pack,
    catalog: &Catalog,
    artifacts: &ArtifactRegistry,
    facets: &crate::registry::Registry,
    stage: &str,
    item: &Item,
    amendment: Option<&step7::Accepted>,
) -> Result<Selection, SelectError> {
    // ---- the refusing load rules, BEFORE any step reads the catalog ---------------------------
    //
    // AC-053, AC-059, AC-060 and AC-047 are *load* rules, and until 2026-09-13 every one of them
    // was a function with no caller outside `tests/it/` — so a dangling `requires` reached step 4
    // and came back as an ordinary `Unsatisfiable`, which `tests/fixtures/catalog-dangling/`'s own
    // header forbids. This is the one place the pack, the catalog, the artifact registry and the
    // facet registry are all in hand, and it is the boundary past which a catalog defect would
    // become data in a `Dag`. See [`lints`]'s header for why it is here and not in the loader.
    lints::load_lints(catalog, artifacts, facets)?;

    // ---- step 1 -------------------------------------------------------------------------------
    let one = step1(pack, item);
    let rigor = one.rigor(stage).unwrap_or(crate::rigor::Rigor::Optional);
    let mut skipped = one.skipped().to_vec();

    // ---- step 2 -------------------------------------------------------------------------------
    let two = step2::step2_amended(pack, artifacts, facets, stage, item, amendment)?;
    let required = two.required().clone();
    let gates = two.gates().to_vec();
    let rows_fired = two.rows_fired();

    // ---- step 3 -------------------------------------------------------------------------------
    let mut classifications = BTreeMap::new();
    let mut drift = Vec::new();
    for instance in required.kinds() {
        let state = item.artifact_state(instance);
        let declared = step3::declared_inputs(catalog, artifacts, stage, &instance.kind);
        let declared_refs: Vec<&str> = declared.iter().map(String::as_str).collect();
        let upstream = item.upstream_digests(&declared_refs);
        let upstream_refs: Vec<(&str, Digest)> = upstream
            .iter()
            .map(|(k, d)| (k.as_str(), d.clone()))
            .collect();
        let out = state.as_ref().map_or_else(
            || Step3Outcome::missing(instance),
            |s| step3(artifacts, s, &declared_refs, &upstream_refs),
        );
        if let Some(d) = out.drift.clone() {
            drift.push(d);
        }
        classifications.insert(instance.clone(), out.classification);
    }

    // ---- step 4 -------------------------------------------------------------------------------
    let mut producers = Vec::new();
    let mut unsatisfiable = Vec::new();
    // **Identity is `(kind, key)`, not the whole `Instance`.** `required_by` is provenance — which
    // activity's `requires` pulled the instance in — and two activities requiring the same kind
    // require the SAME artifact, not two of them. Keying the seen-set on the whole struct resolved
    // `Changeset` twice for the fixture pack and emitted two producer rows for one node, which is
    // an over-count in `dag_assembled.nodes` and would put a spurious edge in the DAG.
    let mut seen: Vec<(String, Option<String>)> = Vec::new();
    let mut queue: Vec<Instance> = required
        .kinds()
        .filter(|i| classifications.get(*i) != Some(&Classification::Present))
        .cloned()
        .collect();

    while let Some(instance) = queue.pop() {
        let identity = (instance.kind.clone(), instance.key.clone());
        if seen.contains(&identity) {
            continue;
        }
        seen.push(identity);

        // *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT; R1 = ADR-0191 decision 1,
        // proposed; `s8g` section 8g.2.)* **A pass-sourced kind selects NO catalog producer**: a
        // composition-root pass the record names produces it, so no activity is judged for it,
        // none is planned, and it is not `Unsatisfiable` (its producer exists; it is the pass). The
        // instance stays in `R` and in `pass_sourced` below until its `artifact.write` lands.
        if artifacts.is_pass_sourced(&instance.kind) {
            continue;
        }

        // ADR-0057's Decision: `forbidden` means nothing in that stage runs, EVEN VIA `requires`.
        // The producing stage of a required kind is what decides this, and a kind whose only
        // producers live in a forbidden stage owes BOTH a skip row and a gate (§7b.9(7)).
        let producing_stages = step4::producing_stages(catalog, &instance.kind);
        let any_live = producing_stages.iter().any(|s| !one.is_forbidden(s));
        if !producing_stages.is_empty() && !any_live {
            for forbidden in &producing_stages {
                skipped.push(SkipRow {
                    stage: forbidden.clone(),
                    kind: Some(instance.kind.clone()),
                    requiring_activity: instance.required_by.clone(),
                    reason: SkipReason::ForbiddenStageViaRequires,
                });
            }
            unsatisfiable.push(Unsatisfiable {
                kind: instance.kind.clone(),
                failed_predicates: Vec::new(),
                rigor,
                reason: step4::UnsatReason::ForbiddenStage,
            });
            continue;
        }

        // An accepted `downgrade` moves the rigor THIS KIND's producer is resolved under, and
        // nothing else's. It is applied here rather than at step 2 because it changes candidacy,
        // not the required set (design §4.4 row 4's `min_rigor <= r <= max_rigor`).
        let effective = match amendment {
            Some(a) if a.kind == DeviationKind::Downgrade && a.target == instance.kind => {
                downgrade_one(rigor)
            }
            Some(_) | None => rigor,
        };

        let picked = step4(
            catalog, artifacts, facets, stage, effective, &instance, item,
        )?;
        if let Some(c) = picked.chosen() {
            producers.push(SelectedProducer {
                instance: instance.clone(),
                activity_id: c.activity_id.clone(),
                method_rank: c.method_rank,
            });
            // "recurse on the producer's `requires`" — design §4.4 row 4.
            //
            // **The keyed expansion applies on this path too, and that is not optional.** AC-049's
            // second AND is *"a mixed item cannot be satisfied by a single instance of a keyed
            // kind"*, and it says nothing about how the kind entered the graph. A requirement that
            // arrived un-keyed through a `requires` edge would give a mixed Node+UE item one
            // `Changeset` instead of two — the same defect F35 §1.3 measured one level up, arriving
            // by a different road.
            for req in step4::requires_of(catalog, &c.activity_id) {
                for mut child in step2::expand(artifacts, &req.kind, item) {
                    child.required_by = Some(c.activity_id.clone());
                    queue.push(child);
                }
            }
        } else if let Some(u) = picked.unsatisfiable() {
            unsatisfiable.push(u.clone());
        }
    }

    producers.sort_by(|a, b| (&a.instance, &a.activity_id).cmp(&(&b.instance, &b.activity_id)));
    skipped.sort();
    skipped.dedup();
    unsatisfiable.sort_by(|a, b| a.kind.cmp(&b.kind));
    // ADR-0200 decision 2: the step-4 gate opens on a human kind's `NoAdmissibleProducer` only.
    // Owned copies: the selection stores them beside `unsatisfiable`, which keeps its own.
    let mut human_roots: Vec<String> = unsatisfiable
        .iter()
        .filter(|u| u.reason == step4::UnsatReason::NoAdmissibleProducer)
        .filter(|u| artifacts.is_human_sourced(&u.kind))
        .map(|u| u.kind.clone())
        .collect();
    human_roots.dedup();
    drift.sort_by(|a, b| (&a.root, &a.upstream).cmp(&(&b.root, &b.upstream)));

    let inputs_digest = canonical::inputs_digest(pack, catalog, item)?;

    // R1 = ADR-0191 decision 1: stage completeness still needs a pass-sourced kind's
    // `artifact.write`, so every required pass-sourced instance not yet `Present` is named here.
    let pass_sourced: Vec<Instance> = required
        .kinds()
        .filter(|i| artifacts.is_pass_sourced(&i.kind))
        .filter(|i| classifications.get(*i) != Some(&Classification::Present))
        .cloned()
        .collect();

    // ---- step 5 -------------------------------------------------------------------------------
    let five = step5(
        pack,
        catalog,
        &producers,
        &required,
        &classifications,
        &unsatisfiable,
        &skipped,
        rows_fired,
        &inputs_digest,
        &pass_sourced,
    )?;

    Ok(Selection {
        stage: stage.to_owned(),
        rigor,
        required,
        classifications,
        producers,
        gates,
        skipped,
        unsatisfiable,
        drift,
        rows_fired,
        step5: five,
        inputs_digest,
        pass_sourced,
        human_roots,
    })
}

/// The `R`-equality assertions, **in this module rather than in `tests/it/`**.
///
/// §7b.4:431-436 fixes the placement and says why: `RequiredSet`'s constructor is
/// `pub(in crate::select)` and AC-058, AC-063 and AC-078 all name **unit** tests, so an
/// `R`-equality assertion written in `tests/it/` would need a public constructor — and *"a
/// `pub(crate)` constructor, or a `#[cfg(test)]` public one, silently converts AC-061 from a type
/// fact into a convention"*.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::rigor::Rigor;

    const ARTIFACTS: &str = include_str!("../tests/fixtures/registries/artifacts/artifacts.toml");
    const PACK: &str = include_str!("../tests/fixtures/packs/incident-forbidden.toml");

    fn fixtures() -> (Pack, Catalog, ArtifactRegistry, crate::registry::Registry) {
        let pack =
            Pack::from_toml("incident-forbidden.toml", PACK).expect("the fixture pack loads");
        let catalog = Catalog::from_sources(&[
            (
                "impl-report.toml",
                include_str!("../tests/fixtures/catalog-select/impl-report.toml"),
            ),
            (
                "change-summary.toml",
                include_str!("../tests/fixtures/catalog-select/change-summary.toml"),
            ),
            (
                "changeset.toml",
                include_str!("../tests/fixtures/catalog-select/changeset.toml"),
            ),
            (
                "spec-authoring.toml",
                include_str!("../tests/fixtures/catalog-select/spec-authoring.toml"),
            ),
        ])
        .expect("the fixture catalog loads");
        let artifacts =
            ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the registry loads");
        let facets_src = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("spikes/p9/facets.toml"),
        )
        .expect("the shipped facet registry is readable");
        let facets = crate::registry::Registry::from_toml(&facets_src).expect("it loads");
        (pack, catalog, artifacts, facets)
    }

    /// **AC-054's second AND, and the invariant AC-058 re-states for the appetite.** The engine
    /// performs *"no fallback: no next-best rank, no relaxation of `min_rigor`, no substitute kind,
    /// **no dropping the artifact from `R`**"*.
    ///
    /// The fixture reaches an `Unsatisfiable` gate — `impl-report` requires `Spec?`, whose only
    /// producer lives in the `forbidden` `specification` stage — and `R` is compared **byte for
    /// byte under the canonical serialiser** before and after, which is the same comparison AC-040
    /// is measured under rather than a second one.
    #[test]
    fn r_is_byte_identical_across_an_unsatisfiable_gate() {
        let (pack, catalog, artifacts, facets) = fixtures();
        let item = Item::new("INC-1", "incident").with_toolchain("node", ["text"]);

        let before = step2(&pack, &artifacts, &facets, "implement", &item)
            .expect("step 2 resolves")
            .required()
            .clone();
        let selection = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
            .expect("Ok with the kind gated: unsatisfiability is data, not an abort");

        assert!(
            !selection.unsatisfiable().is_empty(),
            "the fixture must actually reach a gate, or this asserts nothing"
        );
        assert_eq!(
            canonical::canonical(&before.to_json()).expect("canonical"),
            canonical::canonical(&selection.required().to_json()).expect("canonical"),
            "R is byte-identical across the gate"
        );
    }

    /// **AC-061's type fact, asserted from inside.** The constructor this test can reach is exactly
    /// the one `tests/ui/ac061_required_set_is_not_constructible_outside_select.rs` cannot, and
    /// naming both here is what keeps the two halves from drifting apart.
    #[test]
    fn required_set_is_constructible_here_and_only_here() {
        let mut kinds = std::collections::BTreeSet::new();
        kinds.insert(Instance::of("ImplReport"));
        let r = RequiredSet::new(kinds);
        assert_eq!(r.len(), 1);
        assert!(r.contains(&Instance::of("ImplReport")));
    }

    /// **Step 1's non-default.** A stage no pack policy mentions has NO rigor, and
    /// `select` does not invent one for the required set — it is the one place a
    /// default would be invisible, so it is asserted rather than assumed.
    #[test]
    fn an_unmentioned_stage_has_no_rigor() {
        let (pack, _, _, _) = fixtures();
        let out = step1(&pack, &Item::new("INC-1", "incident"));
        assert_eq!(out.rigor("no-such-stage"), None);
        assert_eq!(out.rigor("triage"), Some(Rigor::Mandatory));
    }
}
