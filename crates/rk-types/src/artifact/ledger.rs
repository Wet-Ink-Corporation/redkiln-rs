//! The per-AC ledger — the `ledger` companion kind's sidecar (AC-333; 220 records at the pin).

use crate::text::Text;
use serde::{Deserialize, Serialize};

/// One closed status per AC. `Unpinned` is RESERVED and explicit: an AC with no discriminating
/// test is *unpinned*, which is a state to be named, not an absence to be inferred
/// (DECISIONS-2026-09-11, PIA-032/033).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LedgerStatus {
    /// The AC's named assertion flips under its declared mutation (AC-334's predicate).
    Pinned,
    /// The suite stayed green under the mutation. A survivor blocks exit even when quarantined
    /// (ADR-0104 decision 4).
    MutationSurvived,
    /// No discriminating test. Reserved, and never a default.
    Unpinned,
    /// The toolchain pack declares mutation infeasible (AC-337).
    NotFeasible { reason: Text },
    /// A human gate answered THAT AC explicitly (AC-357).
    HumanAnswered { reason: Text },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcLedgerRow {
    pub ac: String,
    pub status: LedgerStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcLedger {
    pub rows: Vec<AcLedgerRow>,
}
