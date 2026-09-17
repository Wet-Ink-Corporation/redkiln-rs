//! **AC-326's bootstrap comparator** (ADR-0104 decision 3) — the reclassifier over a run's
//! `basis: affected` claim against the most recent recorded whole-suite count at the toolchain's
//! version pin.
//!
//! - executed **≥** recorded → `basis: whole-suite`, reason `selection-ineffective`, and a
//!   `selection_ineffective` row (ADR-0124). The boundary is inclusive: a filter that ran exactly
//!   the whole suite selected nothing.
//! - no record at the pin → `whole-suite / probe-unverified`, the same row with the reason, and
//!   the run's executed count SEEDS the comparator. A pin bump is a different key and so an empty
//!   record: the reset is a property of `rk_store::artifacts::verify_report`'s key, not a step.
//! - a non-empty change set that selected zero tests (AC-332) → `whole-suite / selection-empty`
//!   or a failed run, per the declared [`SelectionPolicy`]; never an `affected` pass.
//! - `basis` and `basis_reason` are AC-352's closed enums, matched here with no wildcard.
//!
//! *Known gap, carried not smoothed* (the register's own): this detects only a filter that is
//! fully inert. 200 of 236 passes. AC-327's probe and SM-097 are the compensating instruments.

use rk_store::artifacts::verify_report::WholeSuiteRecord;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::verify::{Basis, BasisReason};

/// What the pack declared for a selection that resolves to zero tests (AC-332).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionPolicy {
    EscalateToWholeSuite,
    Fail,
}

/// A run's claim, before the comparator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    pub run_id: String,
    pub basis: Basis,
    pub reason: Option<BasisReason>,
    /// The count this run executed.
    pub executed: u32,
    /// The size of the changed-file set the selection was computed from.
    pub changed_files: u32,
    pub toolchain: String,
    pub toolchain_version: String,
    pub argv: Vec<String>,
    pub policy: SelectionPolicy,
}

/// The comparator's answer.
#[derive(Debug, Clone, PartialEq)]
pub struct Reclassified {
    pub basis: Basis,
    pub reason: Option<BasisReason>,
    /// Under `SelectionPolicy::Fail`, an empty selection fails the run rather than escalating.
    pub failed: bool,
    /// The `selection_ineffective` row, when the claim was reclassified.
    pub events: Vec<Event>,
    /// The record this run seeds, on the bootstrap.
    pub seed: Option<WholeSuiteRecord>,
}

impl Reclassified {
    /// The `(basis, reason)` pair as JSON — AC-352's capture shape.
    #[must_use]
    pub fn pair(&self) -> serde_json::Value {
        serde_json::json!({
            "basis": self.basis.as_str(),
            "reason": self.reason.map(BasisReason::as_str),
        })
    }
}

/// **The reclassifier.** A `whole-suite` claim passes through. An `affected` claim is tested in
/// order: an empty selection over a non-empty change set (AC-332) escalates or fails per policy;
/// no record at the pin bootstraps (`probe-unverified`, seeded); executed ≥ recorded is
/// `selection-ineffective`; otherwise the claim stands.
#[must_use]
pub fn reclassify(claim: &Claim, recorded: Option<&WholeSuiteRecord>) -> Reclassified {
    let pass_through = Reclassified {
        basis: claim.basis,
        reason: claim.reason,
        failed: false,
        events: Vec::new(),
        seed: None,
    };
    if claim.basis == Basis::WholeSuite {
        return pass_through;
    }
    if claim.executed == 0 && claim.changed_files > 0 {
        return match claim.policy {
            SelectionPolicy::EscalateToWholeSuite => Reclassified {
                basis: Basis::WholeSuite,
                reason: Some(BasisReason::SelectionEmpty),
                failed: false,
                events: vec![ineffective_event(
                    claim,
                    BasisReason::SelectionEmpty,
                    recorded,
                )],
                seed: None,
            },
            SelectionPolicy::Fail => Reclassified {
                basis: Basis::WholeSuite,
                reason: Some(BasisReason::SelectionEmpty),
                failed: true,
                events: vec![ineffective_event(
                    claim,
                    BasisReason::SelectionEmpty,
                    recorded,
                )],
                seed: None,
            },
        };
    }
    match recorded {
        None => Reclassified {
            basis: Basis::WholeSuite,
            reason: Some(BasisReason::ProbeUnverified),
            failed: false,
            events: vec![ineffective_event(claim, BasisReason::ProbeUnverified, None)],
            seed: Some(WholeSuiteRecord {
                toolchain_version: claim.toolchain_version.clone(),
                count: claim.executed,
                run_id: claim.run_id.clone(),
            }),
        },
        Some(rec) if claim.executed >= rec.count => Reclassified {
            basis: Basis::WholeSuite,
            reason: Some(BasisReason::SelectionIneffective),
            failed: false,
            events: vec![ineffective_event(
                claim,
                BasisReason::SelectionIneffective,
                Some(rec),
            )],
            seed: None,
        },
        Some(_) => pass_through,
    }
}

/// The `selection_ineffective` row (ADR-0124's payload: `run_id, toolchain, reason, executed,
/// recorded_whole_suite`).
fn ineffective_event(
    claim: &Claim,
    reason: BasisReason,
    recorded: Option<&WholeSuiteRecord>,
) -> Event {
    Event::new(
        EventKind::SelectionIneffective,
        serde_json::json!({
            "run_id": claim.run_id,
            "toolchain": claim.toolchain,
            "reason": reason.as_str(),
            "executed": claim.executed,
            "recorded_whole_suite": recorded.map(|r| r.count),
        }),
    )
}

/// One line per reason, matched exhaustively — a seventh variant is an `E0004` here (AC-352).
#[must_use]
pub const fn describe(reason: BasisReason) -> &'static str {
    match reason {
        BasisReason::NoSelectionSupport => "the pack declares no affected-selection support",
        BasisReason::ProbeUnverified => {
            "no whole-suite count is recorded at this pin, or the probe entry is absent or stale"
        }
        BasisReason::SelectionEmpty => "a non-empty change set selected zero tests",
        BasisReason::SelectionIneffective => {
            "the executed count met the recorded whole-suite count: the filter was inert"
        }
        BasisReason::PolicyForced => "the pack's policy demanded the whole suite",
        BasisReason::ToolchainUnsupported => "the claiming toolchain has no verify adapter",
    }
}
