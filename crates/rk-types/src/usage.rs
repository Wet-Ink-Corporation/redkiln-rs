//! `Usage`, `CacheCreation` and the record's other usage-adjacent types — AC-362's *"the type that
//! carries it"*, homed here by ADR-0126 decision 2.
//!
//! **Lifted, with edits, from `runner/src/event.rs` in `redkiln-corpus` at
//! `61f590e51cbb00301c310e46aab615563a7787b8`** (the SHA in `CORPUS_PIN`; blob
//! `5ff3e4ae91a2f0ab99b08cfb3adcfab47880c27b`, 505 lines) — the second lift destination of that
//! file, beside the line classifier that lands in `rk-adapter-claude` (`s7c` §7c.2's lift table).
//! That file was the spike runner's parse of the CLI's `result` frame, **built and run over the
//! programme's recorded dispatches and exercised, not validated against an oracle** — so what
//! travels here is the field set, not a warrant (PIA-036).
//!
//! **Three edits, each forced by a rule the corpus version predates:**
//!
//! 1. **`Default` is gone from `Usage` and `CacheCreation`** (AC-362; PIA-062). The runner derived
//!    it so its tests could write `..Default::default()`; a `Default` on a usage type is a
//!    constructor whose every counter reads `None` by omission, and the derive-Default walk in
//!    Checkpoint 7c (`s7c` §7c.21) fails the build on one. The lifted tests became struct literals.
//! 2. **`ttl_split()`, `cache_read_share()` and `total_input()` propagate absence.** The runner
//!    read an absent counter as `0` (an `unwrap_or` over zero at the pin's `:124` and `:163`);
//!    AC-362's THEN forbids exactly that spelling at any call site, *"so a consumer cannot read
//!    absent as zero without a visible match"* — and Halt 11's shape is why: a `Some((0, 0))` from
//!    an absent split makes a TTL claim look verified. Each helper answers `None` when a counter
//!    it needs did not arrive — **one policy over all three**, `total_input()` included; its first
//!    form summed over the terms that had arrived and was corrected the same day, because a sum
//!    over present terms is the absent-as-zero read with the zero left implicit. (The banned
//!    spelling is not written out here even in prose: the CI rule that enforces it is a
//!    `git grep` over this file with no comment filter.)
//! 3. **The share is handed back as its two integers, not as an `f64`.** The runner returned
//!    `read / (read + created)` as a float; this crate carries no float today, `rk-store`'s
//!    canonical serialiser refuses one, and the reporter that divides — AC-382's helper half,
//!    with its `n` and its `n < 10` refusal — has an UNCONFIRMED home (`s7c` §7c.22). The division
//!    is the reporter's; the counters are the record's.
//!
//! **Why the types beyond the two lifted ones are here** (`s7c` §7c.11, the one-line home list the
//! third review asked for, *"so Slice 1 declares nothing by guess"*): ADR-0126 decision 1's payload
//! table names the completed [`crate::activity::DispatchRecord`]'s members, decision 2 says a field
//! of an `rk-types` struct has an `rk-types` type because `rk-types` depends on nothing internal,
//! and the register fixes three of the shapes — `usage` and every usage-adjacent measurement are
//! `Option` with no `Default` (AC-362), the window readings are `u16` ticks (AC-374) of exactly two
//! windows (AC-375), and `cli_version` is non-optional (AC-227). The rest is this crate's routing
//! of a named member onto a declared shape, each one line below, and the typed-payload record
//! (ADR-0126 decision 6) is the successor that may re-route it.
//!
//! **What none of these types derives:** `Default`. Not `Usage`, not `CacheCreation`, not
//! `WindowReading`, not `Timings` — the four the derive-Default walk names — and not the others
//! either, for the same reason: absent is `None`, and a value that can be summoned from nothing
//! is a zero with a different spelling.

use serde::{Deserialize, Serialize};

/// Token accounting from the `result` frame's `usage` block — *"every token counter the harness
/// reports verbatim"* (AC-360), each an `Option` (AC-362).
///
/// Every counter is optional because the docs mark most of them optional and the two cache
/// namings are mutually exclusive in practice; readers pick whichever arrived rather than
/// assuming a naming. **Unknown members are ignored, never refused** — AC-201's rule reaches every
/// struct a stream frame is parsed into, so there is no `deny_unknown_fields` here and may not be.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,

    /// The SDK-reference naming of the read counter, read only when the documented one is absent.
    pub ephemeral_cache_read_input_tokens: Option<u64>,
    /// The SDK-reference naming of the creation counter, read only when the documented one is
    /// absent.
    pub ephemeral_cache_creation_input_tokens: Option<u64>,

    /// The TTL split, which the platform docs place in a **nested** `usage.cache_creation` object
    /// — not as flat `ephemeral_*` siblings (ADR-0113 decision 2: present on 950 of 955 recorded
    /// runs under the nested naming; F3's *"absent"* was the flat naming).
    ///
    /// Getting this wrong is how an instrument reports "no TTL data" forever while the data is
    /// right there. Its *absence* is itself evidence — the five-state classifier (ADR-0113
    /// decision 1, `s7c` §7c.9) reads an absent object as `TTL-unverified`, never as a tier — and
    /// that is why it is `None` and not an object of zeros.
    pub cache_creation: Option<CacheCreation>,
}

/// `usage.cache_creation` — cache writes split by TTL tier (ADR-0113).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheCreation {
    pub ephemeral_1h_input_tokens: Option<u64>,
    pub ephemeral_5m_input_tokens: Option<u64>,
}

impl Usage {
    /// The cache-read share's two integers — `(read, created)` — from which the share is
    /// `read / (read + created)`.
    ///
    /// `None` when either counter did not arrive, and `None` when both read `0`: "no cache data",
    /// "a cold write every time" and "nothing measured" are three different findings, and E1's
    /// kill criterion turned on telling the first two apart. The division is the reporter's
    /// (AC-382's helper half), not this type's — see the module doc's third edit.
    #[must_use]
    pub fn cache_read_share(&self) -> Option<(u64, u64)> {
        let read = self
            .cache_read_input_tokens
            .or(self.ephemeral_cache_read_input_tokens)?;
        let created = self
            .cache_creation_input_tokens
            .or(self.ephemeral_cache_creation_input_tokens)?;
        if read + created == 0 {
            return None;
        }
        Some((read, created))
    }

    /// Which TTL tier the cache writes landed in, if the split was reported: `(five_minute,
    /// one_hour)`, the runner's order, kept.
    ///
    /// `None` when the nested object is absent **or when either tier is absent from it**: a
    /// partial split is not a split, and reading the missing tier as `0` would report "0 tokens on
    /// 1h", which reads as a measurement (Halt 11). The platform default is a 5-minute TTL and a
    /// 1h tier must be requested explicitly, so this is the field that decides whether a
    /// dispatch's TTL claim is verified at all (ADR-0078 clause (ii); ADR-0113).
    #[must_use]
    pub fn ttl_split(&self) -> Option<(u64, u64)> {
        let c = self.cache_creation.as_ref()?;
        Some((c.ephemeral_5m_input_tokens?, c.ephemeral_1h_input_tokens?))
    }

    /// Total input actually processed: `input_tokens + read + created`.
    ///
    /// The docs are explicit that `input_tokens` counts only tokens AFTER the last cache
    /// breakpoint, so it is not the denominator anyone expects; E1's headline metric stayed the
    /// charter's `read / (read + creation)` and this is recorded alongside so the other
    /// denominator can be computed later without re-running anything.
    ///
    /// `None` when **any** of the three terms did not arrive — the same policy as
    /// [`Self::cache_read_share`] and [`Self::ttl_split`], and there is one policy and not two.
    /// The first form of this helper (Step 7c Slice 1, corrected the same day) summed over the
    /// terms that had arrived, which is the runner's absent-as-zero read re-spelled so the CI grep
    /// cannot see it: a consumer of that total could not tell *"the cache counters were absent"*
    /// from *"the cache counters measured 0"*, which is exactly the coincidence AC-362's THEN
    /// forbids and ADR-0126 forbids in terms (*"any usage-adjacent absence written as `0`"*).
    #[must_use]
    pub fn total_input(&self) -> Option<u64> {
        let read = self
            .cache_read_input_tokens
            .or(self.ephemeral_cache_read_input_tokens)?;
        let created = self
            .cache_creation_input_tokens
            .or(self.ephemeral_cache_creation_input_tokens)?;
        Some(self.input_tokens? + read + created)
    }
}

/// A headroom reading: **exactly two windows** (AC-375) in **`u16` ticks** (AC-374).
///
/// The ticks are parsed from the decimal digits (ADR-0110) — `utilization` is quantised to `0.01`
/// [measured, F6], so a tick is a hundredth and `100` is the window spent.
///
/// Never an `f64` — `#[derive(Eq)]` on the record refuses one and so does `rk-store`'s canonical
/// serialiser — and never a model-family member (`seven_day_opus` / `seven_day_sonnet` are
/// tolerated and ignored by the adapter's parse, never named here). The record carries readings
/// and no per-dispatch delta (AC-373); `resets_at` per window is the typed-payload record's
/// (ADR-0126 decision 6).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowReading {
    pub five_hour: u16,
    pub seven_day: u16,
}

/// What the dispatch took: the adapter's wall clock around the child, and the API's own duration
/// when the result frame reported one.
///
/// `wall_ms` is required — the adapter's only two `Instant::now` sites are the ones that bound it,
/// each under `#[expect(clippy::disallowed_methods, reason = …)]` (`s7c` §7c.19) — and `api_ms` is
/// `Option` because it is a counter the harness reports, and AC-362's rule covers it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timings {
    pub wall_ms: u64,
    pub api_ms: Option<u64>,
}

/// The model id the child's own `system/init` reported (`s8h` §8h.3; Record A = ADR-0197, proposed).
/// *(Added 2026-10-02, `s8h` build, package P2-rk-types.)*
///
/// A distinct type, so a requested id cannot be written into the record's member by a plain string
/// assignment: the member is the child's REPORT, never the `--model` argument copied in (Halt 11's
/// shape; the reasoning that gives [`CliVersion`] its two arms). It travels as the bare string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObservedModel(String);

impl ObservedModel {
    /// Wrap the id exactly as the child's init printed it.
    #[must_use]
    pub fn new(reported: impl Into<String>) -> Self {
        Self(reported.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The CLI version on the row, **non-optional** (AC-227) and **two-armed so the provenance is
/// distinguishable** (ADR-0126 decision 1; D17-8's dated confirmation).
///
/// `Observed` is the version the child's own `system/init` reported (AC-385), on every path that
/// read one; `Pinned` is the `Config` pin — `instrument_version`'s value — on every path that
/// terminated before one was read: the throttle's `RateLimited` refusal (AC-376), the `Overage`
/// refusal (AC-221), a spawn failure, a kill before init. AC-385's mismatch warning attaches only
/// to `Observed`; a pre-spawn row that wrote the pin as `Observed` would make an unverified claim
/// look verified (Halt 11's shape) and is forbidden by the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliVersion {
    Observed(String),
    Pinned(String),
}

impl CliVersion {
    /// The version string under either arm. The arm is the provenance and is not erased by this;
    /// a reader that needs to know whether the value was measured matches the enum.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Observed(v) | Self::Pinned(v) => v,
        }
    }
}

/// `total_cost_usd` as the **decimal digits verbatim**, exactly as the CLI printed them (RT-19 as
/// amended, `s7c` §7c.1).
///
/// Never an `f64`, which `#[derive(Eq)]` refuses and `rk-store`'s canonical serialiser refuses;
/// never a fixed scale, which would truncate a digit count the pin does not record — the recorded
/// F20 value `2.337147` is the runner's `f64` re-serialisation, so the CLI's own digit count is
/// UNCONFIRMED at the pin. **A record naming the cost unit and any arithmetic form is OWED**
/// (ADR-0126 decision 6's typed-payload record); until it lands this is a string that is compared
/// as a string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CostUsd(pub String);

/// How a usage-limit result was recognised (AC-523).
///
/// By the CLI's own `subtype` on the result frame, or by the disclosed text-marker fallback — a
/// substring match over the result text that survives from the runner only as this labelled
/// second arm, never as the primary detection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectedBy {
    Subtype,
    TextMarker,
}
