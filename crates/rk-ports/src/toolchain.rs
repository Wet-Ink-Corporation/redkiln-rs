//! The `Toolchain` port — *"detect; `build`; `test(selection)`; `lint`; `classify(path) ->
//! Text/Binary/Generated`; `change_summary(range) -> ChangeSummary`"* (design:269).
//!
//! The spike has build/test/classify/change_summary, **no `detect` and no `lint`**, and spells
//! `classify(&self, paths: &[String]) -> Vec<String>` where design:269 and AC-497's header both
//! fix `Text`/`Binary`/`Generated`. The design's column is the authority, not the spike.
//!
//! **What no record fixes and this crate does not pretend it does:** `test`'s selection type and
//! `change_summary`'s range type — taken here as `&[String]` and `&str` rather than as two further
//! opaque newtypes, because `s7a` §7a.3 fixes the cost at **four** opaque newtypes and a fifth and
//! sixth would be two more undecided things to reverse — and whether `detect` reports a bool or a
//! version. Each is a routing and Checkpoint 7a asserts none of them.

use rk_types::error::StepError;

/// What `classify` says about one path (design:269; AC-497's port shape turns on `Binary`,
/// register :7109).
///
/// **Routed to `rk-ports`, not decided — no record assigns it.** It is named and homed here
/// because `toolchain_fake_classifies_binary` has nothing to assert otherwise, and because its
/// only consumer before 7c is this port's own fake while `rk-types`' *"typed artifacts"* row
/// (design:1699) does not name it. **A record naming the home is OWED**, and Checkpoint 7a's
/// re-declaration ban deliberately does not cover it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Text,
    Binary,
    Generated,
}

/// `change_summary` returns an opaque, non-finalised newtype until S-BIN reports (**ADR-0099
/// clause 5**, :90-91).
///
/// **This is the one site in `crates/` the name may appear** (ADR-0099's forbids bullet at :131),
/// and Checkpoint 5's grep for its absence covers `crates/` minus exactly this declaration. The
/// rule is about where the **type** lives, not about one textual occurrence: the shape necessarily
/// carries the name on `change_summary`'s return type and on the fake's impl too.
///
/// **Its body is not written here.** That is 7c's, behind S-BIN (ADR-0093 decision 4): the
/// Phase-1 fake returns `ChangeSummary(())` and no real adapter implements it at this stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeSummary(pub(crate) ());

/// The build toolchain. The kernel knows no language.
pub trait Toolchain: Send + Sync {
    /// Is this toolchain present for the project under test?
    fn detect(&self) -> Result<bool, StepError>;

    /// Build.
    fn build(&self) -> Result<(), StepError>;

    /// Run the selected tests; the count that ran comes back.
    fn test(&self, selection: &[String]) -> Result<u32, StepError>;

    /// Lint.
    fn lint(&self) -> Result<(), StepError>;

    /// Text, binary or generated — **per path**, as design:269 spells it.
    fn classify(&self, path: &str) -> Result<Classification, StepError>;

    /// What changed over a range, as the typed value a reviewer receives instead of a diff when a
    /// path classifies as `Binary` (AC-497).
    fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError>;
}

/// **AC-337 — a toolchain pack declares mutation feasibility explicitly.** *"Per-AC mutation
/// evidence is required only where the toolchain declares mutation feasible"* (design §4.2;
/// ADR-0008's Unreal fixture is the canonical infeasible toolchain). The declaration is a type,
/// not a trait method: no record fixes a sixth port method, Checkpoint 7a counts exactly six on
/// this port, and a declaration is a fact about the PACK, read when the pack is loaded.
///
/// An absent declaration is a **configuration error** ([`FeasibilityError::Undeclared`]), never a
/// default in either direction — a pack that says nothing has said nothing. Where mutation is
/// infeasible, every AC's mutation-path status is `not-feasible(reason)`
/// ([`Self::ledger_status`]), which is **not `pinned`** and does not let a stage exit (AC-346);
/// AC-357 routes such an AC to a human gate.
///
/// The `[mutation] feasible = …, reason = …` table this is read from is the pack loader's
/// (`rk-kernel`); the key name is the runbook's fixture wording, not a record's, and is raised as
/// such.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationFeasibility {
    Feasible,
    NotFeasible { reason: String },
}

/// How a feasibility declaration was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeasibilityError {
    /// The pack declares nothing — a configuration error, not a default (AC-337).
    Undeclared,
    /// `feasible = false` with no reason: the register's shape is `not-feasible(reason)`.
    InfeasibleWithoutReason,
    /// `feasible = true` with a reason, which means nothing and is refused rather than dropped.
    FeasibleWithReason,
}

impl std::fmt::Display for FeasibilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undeclared => write!(
                f,
                "the toolchain pack declares no mutation feasibility — `[mutation] feasible` is \
                 required, and its absence is a configuration error, not a default (AC-337)"
            ),
            Self::InfeasibleWithoutReason => write!(
                f,
                "`feasible = false` needs a `reason`: an AC records `not-feasible(reason)` (AC-337)"
            ),
            Self::FeasibleWithReason => {
                write!(
                    f,
                    "`feasible = true` carries a `reason`, which means nothing here"
                )
            }
        }
    }
}

impl std::error::Error for FeasibilityError {}

impl MutationFeasibility {
    /// Build the declaration from the pack's two members as the loader read them.
    ///
    /// # Errors
    ///
    /// [`FeasibilityError`] — absent, infeasible without a reason, or feasible with one.
    pub fn declared(
        feasible: Option<bool>,
        reason: Option<&str>,
    ) -> Result<Self, FeasibilityError> {
        let reason = reason.map(str::trim).filter(|r| !r.is_empty());
        match (feasible, reason) {
            (None, _) => Err(FeasibilityError::Undeclared),
            (Some(false), None) => Err(FeasibilityError::InfeasibleWithoutReason),
            (Some(false), Some(reason)) => Ok(Self::NotFeasible {
                reason: reason.to_owned(),
            }),
            (Some(true), None) => Ok(Self::Feasible),
            (Some(true), Some(_)) => Err(FeasibilityError::FeasibleWithReason),
        }
    }

    /// The status an infeasible toolchain fixes for every AC's mutation path — `not-feasible`,
    /// which is not `pinned`. A feasible toolchain fixes nothing: the flip predicate decides per
    /// AC (AC-334).
    #[must_use]
    pub fn ledger_status(&self) -> Option<rk_types::artifact::ledger::LedgerStatus> {
        match self {
            Self::Feasible => None,
            Self::NotFeasible { reason } => {
                Some(rk_types::artifact::ledger::LedgerStatus::NotFeasible {
                    reason: rk_types::text::Text::new(reason.clone()),
                })
            }
        }
    }
}
