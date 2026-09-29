//! The capability registry — the SQLite row's two dated entries (D17-9).
//!
//! AC-238's *"dated registry entry `{capability, verified_on, probe}`"*, AC-489's required
//! members, AC-492's closed vocabulary. `s7c` §7c.7, RT-24.
//!
//! **One shape, declared in each adapter crate** (RT-24): four `registry.rs` files, each with its
//! own [`Row`], [`Capability`] and [`Probe`], `#[derive(serde::Deserialize)]` +
//! `#[serde(deny_unknown_fields)]`; the four `pub enum Capability` **bodies are byte-equal** —
//! Checkpoint 7c digests the range from `pub enum Capability {` to the closing brace in each
//! crate and requires one digest — so four copies stay one vocabulary, and Step 7d's bridge is
//! `CapabilityClaim::new(capability, verified_on, probe.as_str())`. A shared type in `rk-types`
//! or `rk-ports` would re-open Checkpoint 5 or 7a for a criterion no record homes; the body
//! below carries **no doc comment inside the braces** for the same reason — a comment in one
//! copy would be a fourth digest.
//!
//! **Every member is required** (AC-489): a row missing `capability`, `verified_on` or `probe`
//! fails to load naming the member, a row carrying a member the type does not declare fails
//! naming it (`deny_unknown_fields`, on the row and on the probe), and no `Default`, no
//! `#[serde(default)]` and no `Option` member exists to let one through. **`probe` is a closed
//! enum**, `Probe::{Recorded { source }, None { reason }}` — so a row is present, dated and
//! *explicitly* probe-less, never absent-member (`s7c` §7c.7). **The vocabulary is closed**
//! (AC-492): a token outside [`Capability`] fails to load naming the token, and the one mapper
//! from variant to token, [`Capability::as_str`], is an exhaustive `match` with no catch-all arm
//! (`ac492_…` reads this file for one), so no code path maps an unknown token to a default, an
//! empty set or a no-op.
//!
//! *(dated 2026-09-28, `s8e` build, package S8E-P7; ADR-0181 decision 11, proposed; `s8e` §8e.9:
//! **both rows flip together**, each to `Probe::Recorded { source }` naming the test that exercises
//! it, `verified_on: "2026-09-28"`. `SqliteStore::snapshot` and `rebuild` are implemented, and
//! they still answer THROUGH their rows ([`answer`]): a probe-less row refuses `Unsupported`, and a
//! registry carrying no row is a decode failure. [`unsupported_tokens`] now returns nothing, so the
//! real conformance row's capability arm is empty and the body's round trip runs over the real
//! store. The paragraph below is left as written at `s7c`.)*
//!
//! **The SQLite entries, named**: `snapshot` and `rebuild`, each `verified_on: "2026-09-18"`
//! and each `Probe::None { reason }` — exactly the rows D17-9 says answer `Unsupported`.
//! `rk_ports::Snapshot` is an opaque newtype whose contents no record has decided, so no `Ok`
//! is constructible and no probe exists; the row says so rather than being absent, and
//! [`refusal`] — pure over the rows `SqliteStore::open` loaded through [`rows`] — is where
//! `SqliteStore::snapshot` / `rebuild` take their error from: the token in
//! `StepError::Unsupported { capability }` is the **row's**, and a registry that carries no such
//! row is a typed decode failure (`Transport { Decode }`) naming the missing row, never a
//! silent `Unsupported` (`ac489_…` opens a store over an empty registry and watches the
//! difference; a shipped registry that does not load fails the open, `StoreOpenError::Registry`).
//! The Harness capability (`dispatch`) and the toolchain ones
//! (`change_summary`, `affected-selection`) are **absent** from this registry, not `Probe::None`:
//! a store claims no dispatch or toolchain capability, as the Claude registry claims no store
//! capability (`s7c` §7c.7).
//!
//! **Wire form.** The registry is JSON (the crate carries `serde_json` for `t2_event.payload`
//! already; a TOML reader would be a dependency for one file): a list of rows, `probe`
//! externally tagged — `{"none":{"reason": …}}` / `{"recorded":{"source": …}}`. *(dated
//! 2026-09-28, `s8e` build, package S8E-P7: spelled without the space after the tag's colon, so
//! Checkpoint 8e (5)'s count of probe-less rows — `grep -cF` on the tag, colon, space, brace — reads
//! rows and not this sentence.)* *(dated 2026-09-28, `s8e` build, package R3FIX-8e: that count is
//! now whitespace-robust and reads code lines only, so a doc line such as this one is dropped by
//! the filter rather than by its spelling.)*

use serde::Deserialize;

use rk_types::error::{StepError, TransportKind};

/// The canonical capability vocabulary (AC-492) — one body, four crates, byte-equal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Capability {
    #[serde(rename = "dispatch")]
    Dispatch,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "rebuild")]
    Rebuild,
    #[serde(rename = "change_summary")]
    ChangeSummary,
    #[serde(rename = "affected-selection")]
    AffectedSelection,
}

impl Capability {
    /// The token, exactly as the register spells it (the one hyphenated token included). The
    /// token mapper AC-492's lint reads: exhaustive, no catch-all arm.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dispatch => "dispatch",
            Self::Snapshot => "snapshot",
            Self::Rebuild => "rebuild",
            Self::ChangeSummary => "change_summary",
            Self::AffectedSelection => "affected-selection",
        }
    }
}

/// How a row was verified: a recorded probe, or explicitly none with the reason.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Probe {
    /// A probe ran and its record is at `source` (a corpus SHA and path, a fixture on the tree).
    Recorded { source: String },
    /// No probe exists, and `reason` says why — never an absent member.
    None { reason: String },
}

impl Probe {
    /// The tag 7d's bridge carries.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Recorded { .. } => "recorded",
            Self::None { .. } => "none",
        }
    }
}

/// One registry row: `{capability, verified_on, probe}`, every member required.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    pub capability: Capability,
    pub verified_on: String,
    pub probe: Probe,
}

impl Row {
    /// The capability token.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.capability.as_str()
    }
}

/// The SQLite registry: the two rows D17-9 names. ~~dated the day this crate landed, each
/// explicitly probe-less.~~ *(dated 2026-09-28, `s8e` build, package S8E-P7: re-dated the day
/// `s8e` §8e.9 implemented both, each with the recorded probe that exercises it.)*
const REGISTRY: &str = r#"[
  {
    "capability": "snapshot",
    "verified_on": "2026-09-28",
    "probe": { "recorded": { "source": "crates/rk-adapter-sqlite/tests/it/store.rs store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows; crates/rk-adapter-sqlite/tests/it/conformance.rs conformance::port_conformance_real_row_sqlite" } }
  },
  {
    "capability": "rebuild",
    "verified_on": "2026-09-28",
    "probe": { "recorded": { "source": "crates/rk-adapter-sqlite/tests/it/store.rs store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows; crates/rk-cli/tests/it/doctor.rs doctor::ac184_rebuild_differential_over_anchored_tier_one_rows" } }
  }
]"#;

/// Load a registry from its JSON text.
///
/// # Errors
/// A `serde_json::Error` naming the missing member, the unknown member or the unknown token.
pub fn load(text: &str) -> Result<Vec<Row>, serde_json::Error> {
    serde_json::from_str(text)
}

/// The shipped registry.
///
/// # Errors
/// As [`load`] — a registry that does not load is a build defect `ac489_…` reports.
pub fn rows() -> Result<Vec<Row>, serde_json::Error> {
    load(REGISTRY)
}

/// The tokens whose row carries `Probe::None` — what the adapter answers `Unsupported` for, and
/// what the real conformance row hands `conform_event_store` as its capability arm.
///
/// # Errors
/// As [`load`].
pub fn unsupported_tokens() -> Result<Vec<String>, serde_json::Error> {
    Ok(rows()?
        .iter()
        .filter(|r| matches!(r.probe, Probe::None { .. }))
        .map(|r| r.token().to_owned())
        .collect())
}

/// **Whether a port method may answer, derived from its row in `rows`.**
///
/// `Ok(())` for a row
/// carrying a recorded probe (the method runs), `Err` as [`refusal`] otherwise. *(dated
/// 2026-09-28, `s8e` build, package S8E-P7; ADR-0181 decision 11: `snapshot` and `rebuild` are
/// implemented, so a recorded row is no longer "the row is wrong".)*
///
/// # Errors
/// `Unsupported { capability }` for a probe-less row; a typed decode failure for no row.
pub fn answer(rows: &[Row], capability: Capability) -> Result<(), StepError> {
    match rows.iter().find(|r| r.capability == capability) {
        Some(Row {
            probe: Probe::Recorded { .. },
            ..
        }) => Ok(()),
        Some(Row {
            probe: Probe::None { .. },
            ..
        })
        | None => Err(refusal(rows, capability)),
    }
}

/// The error a port method answers for `capability`, **derived from its row in `rows`**.
///
/// *(dated 2026-09-28, `s8e` build, package S8E-P7.)* Reached through [`answer`] for a probe-less
/// row or a missing one. Its recorded-probe arm below is kept for a direct caller and is now only a
/// statement about such a caller: this crate implements both methods.
///
/// AC-238; D17-9: `Unsupported { capability }` naming the row's token when the row is
/// probe-less; a typed decode failure when `rows` carries no such row, or carries a row claiming
/// a recorded probe for a method this crate has no implementation of — each a defect named,
/// never a default answered. Pure over the slice, so `ac489_…` hands it controlled rows and
/// `SqliteStore` hands it the rows it was opened over ([`rows`] through `SqliteStore::open`).
#[must_use]
pub fn refusal(rows: &[Row], capability: Capability) -> StepError {
    match rows.iter().find(|r| r.capability == capability) {
        Some(Row {
            probe: Probe::None { .. },
            ..
        }) => StepError::Unsupported {
            capability: capability.as_str().to_owned(),
        },
        Some(Row {
            probe: Probe::Recorded { source },
            verified_on,
            ..
        }) => StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "the registry row for `{}` claims a recorded probe ({source}, verified {verified_on})                  but this crate implements no `{}` — the row is wrong, not the method",
                capability.as_str(),
                capability.as_str()
            ),
        },
        None => StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "the registry carries no row for `{}`; a capability with no dated entry is not                  answered as Unsupported by default (AC-238, AC-492)",
                capability.as_str()
            ),
        },
    }
}
