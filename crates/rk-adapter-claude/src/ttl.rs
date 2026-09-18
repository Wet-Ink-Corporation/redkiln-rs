//! The TTL classifier — FIVE states over `usage.cache_creation`, read per run, never awaited
//! (`s7c` §7c.9; ADR-0113 decision 1; ADR-0078; AC-362).
//!
//! **New code over the lifted `Usage::ttl_split()`** (`rk-types`' since ADR-0126): the 211-line
//! stage file listed three states (*verified 1 h*, *TTL-unverified*, *no-write*) and ADR-0113
//! decision 1 fixes **five**, quoted:
//!
//! > *A dispatch's TTL is **verified 1 h** when its `usage.cache_creation.ephemeral_1h_input_tokens
//! > > 0` and `ephemeral_5m_input_tokens == 0`; **verified 5 m** when the converse holds;
//! > **mixed** when both are positive (reported as such, with both counts); and **`TTL-unverified`**
//! > only when the object is absent or both tiers are zero and `cache_creation_input_tokens > 0`.
//! > A warm re-read (all three zero) carries no TTL claim and is reported as `no-write`.*
//!
//! [`classify_ttl`] is that sentence as a `match`, and [`TtlState`] has one variant per clause —
//! `Verified5m` and `Mixed` are distinct outcomes, which is `adr0113_ttl_classifier_five_states`'s
//! red against the three-state set. **Absence stays absence** (AC-362; Halt 11): a `usage` with no
//! `cache_creation` object, or one whose split lacks a tier, or a run that reported no `usage` at
//! all, is `Unverified` — never a tier, never `Some(0)`. A `cache_creation_input_tokens` that did
//! not arrive is not zero either: with both tiers zero and the total absent, no claim of a write is
//! made and none of a no-write, so the state is `Unverified` (nothing measured), not `NoWrite`.
//!
//! **What the state is for.** It is derived from the `dispatch_completed` row's `usage` (ADR-0126
//! decision 1: whether it is also materialised as a member is the typed-payload record's, decision
//! 6); ADR-0078 clause (ii) forbids reporting a run as 1 h without the per-run evidence, and this
//! is the per-run evidence. The reporter that prints it is 7d's `redkiln cost`; billing mode
//! (`isUsingOverage`) is recorded beside every reading on the same row (`headroom_after`'s source
//! event).

use rk_types::usage::Usage;

/// ADR-0113 decision 1's five states, one variant per clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtlState {
    /// `ephemeral_1h_input_tokens > 0 && ephemeral_5m_input_tokens == 0`.
    Verified1h { one_hour: u64 },
    /// The converse: `ephemeral_5m_input_tokens > 0 && ephemeral_1h_input_tokens == 0`.
    Verified5m { five_minute: u64 },
    /// Both tiers positive — reported with both counts.
    Mixed { one_hour: u64, five_minute: u64 },
    /// The object absent (or a tier absent, or no usage at all), or both tiers zero while
    /// `cache_creation_input_tokens > 0` — a write happened and the tier is unknown.
    Unverified,
    /// All three zero: a warm re-read, no TTL claim.
    NoWrite,
}

impl TtlState {
    /// The register's spelling of each state, for a reporter.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified1h { .. } => "verified 1h",
            Self::Verified5m { .. } => "verified 5m",
            Self::Mixed { .. } => "mixed",
            Self::Unverified => "TTL-unverified",
            Self::NoWrite => "no-write",
        }
    }
}

/// Classify one run's `usage` (the `dispatch_completed` row's) into its TTL state.
#[must_use]
pub fn classify_ttl(usage: Option<&Usage>) -> TtlState {
    let Some(usage) = usage else {
        return TtlState::Unverified;
    };
    // `ttl_split` is `(five_minute, one_hour)`, the runner's order, and `None` when the object or
    // either tier is absent (rk-types: a partial split is not a split).
    let Some((five_minute, one_hour)) = usage.ttl_split() else {
        return TtlState::Unverified;
    };
    match (one_hour, five_minute) {
        (h, 0) if h > 0 => TtlState::Verified1h { one_hour: h },
        (0, m) if m > 0 => TtlState::Verified5m { five_minute: m },
        (h, m) if h > 0 && m > 0 => TtlState::Mixed {
            one_hour: h,
            five_minute: m,
        },
        // both zero: a write with no tier is unverified; no write at all is no claim; an absent
        // total is nothing measured
        _ => {
            let total = usage
                .cache_creation_input_tokens
                .or(usage.ephemeral_cache_creation_input_tokens);
            if total == Some(0) {
                TtlState::NoWrite
            } else {
                TtlState::Unverified
            }
        }
    }
}
