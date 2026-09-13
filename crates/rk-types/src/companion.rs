//! The Phase-1 companion kinds (AC-143; ADR-0075 clause 3, decision #3 of 2026-09-11).
//!
//! **Exactly six**, and the naming is genuinely confusing, so it is spelled out: AC-143's own five
//! are `_integration`, `_grounding`, `_decomposition`, `_review` and `_ledger`, and the sixth is
//! the S-MIG census's `ledger` kind — 220 records at the pin. **`_ledger` and `ledger` are
//! different kinds.** `00-preamble.md`'s decision-7 row and `DECISIONS-2026-09-11` #3 list only the
//! five underscore-prefixed names; that is an incomplete restatement of a six-kind list, and under
//! ADR-0092 the AC register owns the wording.
//!
//! The three-source disagreement this settles — ADR-0075 clause 3's four kinds, AC-143's five, the
//! census's eleven plus `ledger` — is why the set is enumerated here once and read everywhere else.

use crate::artifact::ledger::AcLedger;
use crate::text::Text;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A Phase-1 companion kind. Closed at six; a seventh is a schema change, reviewed as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanionKind {
    Integration,
    Grounding,
    Decomposition,
    Review,
    /// The `_ledger` sidecar of an item.
    LedgerCompanion,
    /// The census's bare `ledger` kind. NOT `_ledger`.
    Ledger,
}

/// The eight kinds the S-MIG census holds that are **Phase 2**. `redkiln-migrate` lifts them as
/// opaque prose per ADR-0075 clause 6; a write of one against the Phase-1 registry is REFUSED,
/// never accepted as untyped.
pub const PHASE_2_KINDS: &[&str] = &[
    "closure",
    "design",
    "design-review",
    "slices",
    "storymap",
    "implementation",
    "plan",
    "retrospective",
];

/// Why a companion write was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompanionError {
    /// A Phase-2 census kind. Lifted as opaque prose, never written against this registry.
    Phase2Kind { kind: String },
    /// Not a companion kind at all.
    UnknownKind { kind: String },
    /// A registered kind with no sidecar schema. Unwritable by construction (AC-143).
    NoRegisteredSchema { kind: &'static str },
}

impl CompanionKind {
    /// All six, in AC-143's order: the criterion's five, then the census's `ledger`.
    pub const ALL: [Self; 6] = [
        Self::Integration,
        Self::Grounding,
        Self::Decomposition,
        Self::Review,
        Self::LedgerCompanion,
        Self::Ledger,
    ];

    /// The on-disk name.
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Integration => "_integration",
            Self::Grounding => "_grounding",
            Self::Decomposition => "_decomposition",
            Self::Review => "_review",
            Self::LedgerCompanion => "_ledger",
            Self::Ledger => "ledger",
        }
    }

    /// Resolve a wire name through the REGISTRY, not through `#[derive(Deserialize)]`, so a Phase-2
    /// kind is refused with its own reason rather than as a generic parse error.
    pub fn from_wire(name: &str) -> Result<Self, CompanionError> {
        if let Some(k) = Self::ALL.into_iter().find(|k| k.wire_name() == name) {
            return Ok(k);
        }
        if PHASE_2_KINDS.contains(&name) {
            return Err(CompanionError::Phase2Kind {
                kind: name.to_owned(),
            });
        }
        Err(CompanionError::UnknownKind {
            kind: name.to_owned(),
        })
    }

    /// This kind's own registered sidecar schema.
    ///
    /// A companion kind with no registered schema **cannot be written at all** (AC-143). The
    /// completeness test checks that every kind has one; it cannot judge whether the CONTENTS are
    /// right — that classification is a human gate, and AC-143's residual-risk line says so.
    #[must_use]
    pub fn registered_schema(self) -> Option<Value> {
        match self {
            Self::Integration => Some(crate::schema::export::<IntegrationCompanion>()),
            Self::Grounding => Some(crate::schema::export::<GroundingCompanion>()),
            Self::Decomposition => Some(crate::schema::export::<DecompositionCompanion>()),
            Self::Review => Some(crate::schema::export::<ReviewCompanion>()),
            Self::LedgerCompanion => Some(crate::schema::export::<LedgerCompanionSidecar>()),
            Self::Ledger => Some(crate::schema::export::<AcLedger>()),
        }
    }
}

/// Write a companion, or refuse. There is no untyped path.
pub fn write_companion(kind: &str) -> Result<Value, CompanionError> {
    let k = CompanionKind::from_wire(kind)?;
    k.registered_schema()
        .ok_or(CompanionError::NoRegisteredSchema {
            kind: k.wire_name(),
        })
}

macro_rules! sidecar {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        ///
        /// Phase-1 minimum. No record fixes this companion's field set, so it carries the
        /// stage-level facts the process already names and nothing invented beside them. AC-143's
        /// completeness test cannot judge the contents, only that a schema is registered — its own
        /// residual-risk line records that as the limit.
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            pub item: String,
            pub stage: String,
            pub summary: Text,
        }
    };
}

sidecar!(IntegrationCompanion, "The `_integration` sidecar.");
sidecar!(GroundingCompanion, "The `_grounding` sidecar.");
sidecar!(DecompositionCompanion, "The `_decomposition` sidecar.");
sidecar!(ReviewCompanion, "The `_review` sidecar.");
sidecar!(
    LedgerCompanionSidecar,
    "The `_ledger` sidecar — distinct from the census's bare `ledger` kind."
);
