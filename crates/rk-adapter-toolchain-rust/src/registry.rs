//! The capability registry — the Rust toolchain row's two dated entries (D17-9; AC-328).
//!
//! AC-238's *"dated registry entry `{capability, verified_on, probe}`"*, AC-489's required
//! members, AC-492's closed vocabulary. `s7c` §7c.7, §7c.14, RT-24.
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
//! **The Rust entries, named**: `change_summary` — `Probe::None`, S-BIN unrun, the opaque
//! `rk_ports::ChangeSummary` has no decided contents and no `Ok` is constructible (D17-9;
//! ADR-0099 decision 5's forbids) — and `affected-selection` — `Probe::None`, because **no
//! affected-selection probe result exists at any pin**, on the tree or in any record [measured
//! 2026-09-17: `grep -rn` over `references/`, `runbook/`, `spikes/*.md` finds none; `s7c`
//! §7c.14], and a row with an invented `verified_on`/`probe` is exactly what AC-489/AC-492
//! refuse. Each `verified_on: "2026-09-18"`, the day this crate landed. [`refusal`] — pure over
//! the rows the adapter was constructed over — is where `RustToolchain::change_summary` and
//! `RustToolchain::select_affected` take their error from: the token in `StepError::Unsupported
//! { capability }` is the **row's**; a registry carrying no such row, or one claiming a
//! recorded probe for a method this crate has no implementation of, is a typed decode failure
//! (`Transport { Decode }`) naming the row, never a silent `Unsupported` and never a selection
//! basis (`ac489_…` / `ac328_…` hand it controlled rows and watch the difference). The Harness
//! capability (`dispatch`) and the store ones (`snapshot`, `rebuild`) are **absent** from this
//! registry, not `Probe::None`: a toolchain claims no dispatch or store capability (§7c.7).
//!
//! **What `verified_on` is compared against — nothing yet, and that is stated, not hidden.**
//! AC-328's *"absent or stale"* and its Verified-by's *"a fixture adapter with an expired
//! `verified_on` that must produce `Unsupported`"* need a pin the date is compared to, and no
//! record fixes one: `verified_on` is a required, date-shaped member (AC-489) that [`refusal`]
//! prints and never reads, so today *"absent or stale"* collapses to *"absent"* — a row dated
//! `1970-01-01` with `Probe::Recorded` would be honoured. The comparison is the
//! `CapabilityClaim` bridge's at 7d (`CapabilityClaim::new(capability, verified_on, probe)`,
//! where the pin is in hand), and the expired-date fixture is OWED there with it; the two
//! `ac328_…` tests say the same. The instrument is unmet at 7c and named, never claimed.
//!
//! **Wire form.** The registry is JSON (the crate carries `serde_json` on §7c.19's row, and the
//! `cargo metadata` read S-BIN's `change_summary` will make through it is not built yet — the
//! crate doc; a TOML reader would be a dependency for one file): a list of rows,
//! `probe` externally tagged — `{"none": {"reason": …}}` / `{"recorded": {"source": …}}`.

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

/// The Rust registry: the two rows, dated the day this crate landed, each explicitly probe-less.
const REGISTRY: &str = r#"[
  {
    "capability": "change_summary",
    "verified_on": "2026-09-18",
    "probe": { "none": { "reason": "S-BIN is unrun and rk_ports::ChangeSummary is an opaque newtype whose contents no record has decided (D17-9; ADR-0099 decision 5); no Ok is constructible and no probe exists" } }
  },
  {
    "capability": "affected-selection",
    "verified_on": "2026-09-18",
    "probe": { "none": { "reason": "AC-328: no affected-selection probe result exists at any pin, on the tree or in any record (measured 2026-09-17); selection is refused and no basis: affected record is produced until one is recorded" } }
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
/// what the real conformance row hands `conform_toolchain` as its capability arm.
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

/// The error a method answers for `capability`, **derived from its row in `rows`**.
///
/// AC-238; D17-9; AC-328: `Unsupported { capability }` naming the row's token when the row is
/// probe-less; a typed decode failure when `rows` carries no such row, or carries a row claiming
/// a recorded probe for a capability this crate has no implementation of — each a defect named,
/// never a default answered and never a selection basis. Pure over the slice, so `ac489_…` /
/// `ac328_…` hand it controlled rows and `RustToolchain` hands it the rows it was constructed
/// over.
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
                "the registry row for `{}` claims a recorded probe ({source}, verified \
                 {verified_on}) but this crate implements no `{}` — the row is wrong, not the \
                 method (AC-489/AC-492; for affected-selection, AC-328: no probe exists at any pin)",
                capability.as_str(),
                capability.as_str()
            ),
        },
        None => StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "the registry carries no row for `{}`; a capability with no dated entry is not \
                 answered as Unsupported by default (AC-238, AC-492)",
                capability.as_str()
            ),
        },
    }
}
