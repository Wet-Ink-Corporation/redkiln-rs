//! **The verify gate** — Step 7b-ii, `s7b-ii` §7b-ii.9; the register's `rk-kernel-verify`, a module
//! of `rk-kernel` (design §9.1). *"Reviewed and green" is not evidence that a criterion is pinned*
//! (ADR-0019): this module is where that sentence has teeth.
//!
//! | module | what it owns | criteria |
//! | --- | --- | --- |
//! | [`flip`] | the flip predicate over a declared mutation's run — `pinned` iff the named assertion flips, `mutation-survived` if the suite stays green | AC-334 (ARMED against the Toolchain fake), AC-338's predicate half |
//! | [`ledger`] | the AC evidence ledger: every spec AC materialised, one closed status each, computed from declared mutations, flip outcomes, the toolchain's feasibility and human answers — and from nothing else | AC-333 (the computation), AC-335, AC-337's kernel half, AC-342, AC-345 |
//! | [`exit`] | the stage-exit check: refused, the unpinned ids named, the `stage_exit_refused` row written from one site | AC-346 (ADR-0125) |
//! | [`comparator`] | the bootstrap comparator's reclassifier: `selection-ineffective` at the boundary, `probe-unverified` and the seed on an empty record, `selection-empty` per policy | AC-326, AC-322, AC-332, AC-352 |
//! | [`eligible`] | the gate-eligible adapter list, as data with exactly one entry in Phase 1 | AC-239's list-is-data half (PROVISIONAL) |
//! | [`admissibility`] | the policy, not the type, decides whether an AC at `verification: None` blocks a stage | AC-455 |
//! | [`ac_gate`] | AC-357's gate, minted when the stage exit is refused (its three payload facts), the human answers the ledger reads, and AC-359's re-open with the drift attached *(dated 2026-09-29, s8g build, package S8G-P6-MUTATION-RESUME-DRIFT; R1 = ADR-0191 decision 10, proposed)* | AC-357's machine part, AC-359 |
//!
//! **What none of this reads, by construction and by lint** (`crate::module_boundary`, outside
//! this module — `s7b-ii` §7b-ii.12's self-reference rule): the item's own suite verdict (AC-335),
//! a review's findings or approval (AC-342), a quarantine (AC-345; ADR-0104 decision 4), a CI or
//! tracker status (AC-341). The ledger's inputs are the author's declarations, the harness's
//! recorded runs and a human's recorded answers; the review gate's word about a finding and the
//! verify gate's word about an AC stay two words.
//!
//! **What is powerless here and reported so** (golden constraint 10): the predicate runs over a
//! SCRIPTED [`flip::MutationRun`] — `Toolchain::test` returns a count and the Phase-1 fake cannot
//! script a flip, so nothing in this module applies a patch or runs a suite; the harness against a
//! real adapter is 7c's and AC-334 is ARMED for that reason (`s7b-ii` §7b-ii.9, §7b-ii.14).
//!
//! **ADR-0101 clause 5 bounds what any of this may be read as**: the gate exists and fails closed.

pub mod ac_gate;
pub mod admissibility;
pub mod claims;
pub mod comparator;
pub mod eligible;
pub mod exit;
pub mod flip;
pub mod ledger;
pub mod runner;

use rk_ports::toolchain::{FeasibilityError, MutationFeasibility};
use rk_types::verify::VerifyReport;

use crate::catalog::{Catalog, CatalogError, StepKind};
use crate::select::pack::Pack;

/// **AC-337 — the pack's mutation feasibility**, built from the pack's raw `[mutation]` members.
/// An absent table is the configuration error, raised HERE where the verify gate reads it — so a
/// process pack that declares nothing still loads (every shipped pack does), and the pure
/// selection function never names the port type (AC-525).
///
/// # Errors
///
/// [`FeasibilityError`] — absent, infeasible without a reason, feasible with one.
pub fn mutation_feasibility(pack: &Pack) -> Result<MutationFeasibility, FeasibilityError> {
    let (feasible, reason) = pack.mutation_declaration().unwrap_or((None, None));
    MutationFeasibility::declared(feasible, reason)
}

/// How a report failed the boundary (AC-324: `Result<VerifyReport, StepError>`-shaped — an
/// absent basis, digest or command record is `Err`, never a pass).
#[derive(Debug)]
pub enum BoundaryError {
    /// The value does not parse as a report: a member absent, unknown, or of the wrong type.
    Parse(serde_json::Error),
    /// The members parsed but violate the report's own invariants.
    Shape(rk_types::verify::VerifyReportShapeError),
}

impl std::fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "the verify report does not parse: {e}"),
            Self::Shape(e) => write!(f, "the verify report is malformed: {e}"),
        }
    }
}

impl std::error::Error for BoundaryError {}

/// The verify result at the boundary: parse (every member required, none unknown), then the
/// report's own invariants (a whole-suite fallback names its reason; argv, digest and version are
/// non-empty). Nothing defaults.
///
/// # Errors
///
/// [`BoundaryError`].
pub fn report_at_boundary(value: serde_json::Value) -> Result<VerifyReport, BoundaryError> {
    let report: VerifyReport = serde_json::from_value(value).map_err(BoundaryError::Parse)?;
    report.validate_shape().map_err(BoundaryError::Shape)?;
    Ok(report)
}

/// How a verify activity failed AC-347's lint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyActivityError {
    /// The activity does not load.
    Catalog(String),
    /// A step of kind `agent` — a harness dispatch — inside a verify activity.
    AgentStep { activity: String, step: String },
}

impl std::fmt::Display for VerifyActivityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Catalog(e) => write!(f, "the activity does not load: {e}"),
            Self::AgentStep { activity, step } => write!(
                f,
                "{activity}: step `{step}` is an agent step inside a verify activity — no harness \
                 dispatch participates in verify (AC-347)"
            ),
        }
    }
}

impl std::error::Error for VerifyActivityError {}

impl From<CatalogError> for VerifyActivityError {
    fn from(e: CatalogError) -> Self {
        Self::Catalog(e.to_string())
    }
}

/// **AC-347's lint**: a verify activity's step kinds contain no agent step — the selection is a
/// pure function of the changed-file set, the toolchain config and `facets.toml`, and a harness
/// dispatch inside the activity is the shape F36 measured as unstable (5/20, 3/20, 2/20, 2/20).
///
/// # Errors
///
/// [`VerifyActivityError::AgentStep`] naming the activity and the step; `Catalog` when the source
/// does not load.
pub fn verify_activity_has_no_agent_step(name: &str, src: &str) -> Result<(), VerifyActivityError> {
    let catalog = Catalog::from_sources(&[(name, src)])?;
    for activity in catalog.activities() {
        if activity.stage != "verify" {
            continue;
        }
        if let Some(step) = activity.steps.iter().find(|s| s.kind == StepKind::Agent) {
            return Err(VerifyActivityError::AgentStep {
                activity: activity.id.clone(),
                step: step.id.clone(),
            });
        }
    }
    Ok(())
}
