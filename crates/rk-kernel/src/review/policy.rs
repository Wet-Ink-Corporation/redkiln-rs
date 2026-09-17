//! The policy table — the ONLY place a class becomes a block (ADR-0074 clause 5; AC-294) — and the
//! review-tier resolution (ADR-0015 as ADR-0020 renumbers it; AC-298, AC-299, AC-300, AC-302).
//!
//! **Two functions, neither of which takes a free-text field** (AC-283): [`classify`] here is the
//! policy-table lookup over an [`EvidenceClass`] the classifier computed, a [`ReviewWeight`] the
//! stage's rigor projected, and a [`ReExecution`] outcome the gate recorded. Its signature is
//! pinned by fn-pointer coercion in `tests/it/review.rs`; a `&str` parameter stops that compiling.
//!
//! | weight \ class | `reproduced` | `mutation-survived` | `narrowed-pin` | `advisory` |
//! | --- | --- | --- | --- | --- |
//! | `ReadOnly` (`optional`) | advise | advise | advise | advise |
//! | `Lenient` (`recommended`, the default) | block iff flipped | block iff flipped | advise | advise |
//! | `Strict` (`mandatory`) | block iff flipped | block iff flipped | block iff flipped | advise |
//!
//! *"A finding blocks only after the deterministic gate re-executes it and it flips"* (ADR-0074
//! clause 3): no re-execution record (AC-287) and every `Unverifiable` reason (AC-290) advise at
//! every weight. `advisory` never blocks (AC-296). `ReadOnly` blocks nothing (AC-299). The
//! ordering of the three blocking classes is S-VAR's (F29) and the row for `narrowed-pin` is the
//! operator's severity call, not a measurement (AC-298's own note).
//!
//! **This module reads no model label.** `finding.evidence_label` has no reader here — AC-285's
//! architecture lint (`crate::module_boundary::policy_reads_no_model_label_rule`) and Checkpoint
//! 7b-ii's grep over this path both hold it, and `tests/fixtures/lint-controls/
//! ac285_policy_reads_the_model_label.rs` is their positive control.
//!
//! **A block value is constructible only here** (AC-294): [`Block`]'s members are private and it has
//! no public constructor, so `tests/ui/ac294_block_constructor.rs` cannot compile and
//! `crate::module_boundary::block_construction_sites_rule` finds zero sites outside this file.

use crate::rigor::Rigor;

use super::classify::EvidenceClass;
use super::reexec::{ReExecution, Unverifiable};

/// The stage's review weight — the review stage's rigor, projected (ADR-0020: `optional →
/// ReadOnly` (the old 0.2), `recommended → Lenient`, `mandatory → Strict` (the old 1.0)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReviewWeight {
    ReadOnly,
    Lenient,
    Strict,
}

impl ReviewWeight {
    pub const ALL: [Self; 3] = [Self::ReadOnly, Self::Lenient, Self::Strict];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::Lenient => "lenient",
            Self::Strict => "strict",
        }
    }
}

/// **AC-302 / AC-298 — the tier derives from the authored stage rigor and nothing else.**
///
/// The one parameter is the review stage's declared rigor — a pack value (`[policy] stages = {
/// review = … }`), never a model output; there is no parameter a model-produced value could
/// arrive through, which is the whole of AC-302's signature test. Unset resolves to `Lenient`
/// (AC-298: *"`narrowed-pin` advises, it does not block"*). `forbidden` and `discouraged` project
/// to `ReadOnly`: a stage that does not run reviews nothing, and a weight below `optional` has no
/// row in the tier table (ADR-0015 names two tiers and ADR-0020 three levels above them).
#[must_use]
pub fn resolve_review_tier(stage_rigor: Option<Rigor>) -> ReviewWeight {
    match stage_rigor {
        None | Some(Rigor::Recommended) => ReviewWeight::Lenient,
        Some(Rigor::Forbidden | Rigor::Discouraged | Rigor::Optional) => ReviewWeight::ReadOnly,
        Some(Rigor::Mandatory) => ReviewWeight::Strict,
    }
}

/// A block value. Constructible ONLY in this module (AC-294): the fields are private and there is
/// no public constructor, so a block that did not originate in a `(class, weight)` lookup cannot
/// exist. Carries what the lookup saw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    class: EvidenceClass,
    weight: ReviewWeight,
}

impl Block {
    #[must_use]
    pub const fn class(&self) -> EvidenceClass {
        self.class
    }

    #[must_use]
    pub const fn weight(&self) -> ReviewWeight {
        self.weight
    }
}

/// Why a finding did NOT block — persisted with it in the advisory sink (AC-310).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonBlockReason {
    /// The class is `advisory`; it never blocks (AC-296).
    AdvisoryClass,
    /// `narrowed-pin` advises at this weight; it blocks only at `Strict` (ADR-0074 clause 5).
    NarrowedPinAdvisesAtWeight,
    /// The weight is `ReadOnly`; nothing blocks (AC-299).
    ReadOnly,
    /// No re-execution record, or one that did not flip (AC-287, AC-290).
    Unverifiable(Unverifiable),
}

impl NonBlockReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryClass => "advisory_class",
            Self::NarrowedPinAdvisesAtWeight => "narrowed_pin_advises_at_weight",
            Self::ReadOnly => "read_only",
            Self::Unverifiable(Unverifiable::BothFail) => "unverifiable:both_fail",
            Self::Unverifiable(Unverifiable::CannotRun) => "unverifiable:cannot_run",
            Self::Unverifiable(Unverifiable::Timeout) => "unverifiable:timeout",
        }
    }
}

/// The policy table's answer for one finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Block(Block),
    Advise(NonBlockReason),
}

/// **The policy-table lookup** — `(class, weight, re-execution outcome) → block or advise`.
///
/// The checks run in the order of the guarantees: the weight first (`ReadOnly` blocks nothing,
/// whatever follows), the class second (`advisory` never blocks; `narrowed-pin` only at `Strict`),
/// the re-execution last (only `Flipped` blocks; absent is `Unverifiable { cannot_run }`, AC-287).
/// The only `Block` construction in the crate is the one line below.
#[must_use]
pub fn classify(
    class: EvidenceClass,
    weight: ReviewWeight,
    reexec: Option<&ReExecution>,
) -> Decision {
    if weight == ReviewWeight::ReadOnly {
        return Decision::Advise(NonBlockReason::ReadOnly);
    }
    match class {
        EvidenceClass::Advisory => return Decision::Advise(NonBlockReason::AdvisoryClass),
        EvidenceClass::NarrowedPin if weight != ReviewWeight::Strict => {
            return Decision::Advise(NonBlockReason::NarrowedPinAdvisesAtWeight);
        }
        EvidenceClass::Reproduced
        | EvidenceClass::MutationSurvived
        | EvidenceClass::NarrowedPin => {}
    }
    match reexec {
        Some(ReExecution::Flipped) => Decision::Block(Block { class, weight }),
        Some(ReExecution::Unverifiable(reason)) => {
            Decision::Advise(NonBlockReason::Unverifiable(*reason))
        }
        None => Decision::Advise(NonBlockReason::Unverifiable(Unverifiable::CannotRun)),
    }
}
