//! The capability registry — the Claude row's ONE dated entry, `dispatch` (D17-9; `s7c` §7c.7).
//!
//! AC-238's *"dated registry entry `{capability, verified_on, probe}`"*, AC-489's required
//! members, AC-492's closed vocabulary. RT-24.
//!
//! **One shape, declared in each adapter crate** (RT-24): four `registry.rs` files, each with its
//! own [`Row`], [`Capability`] and [`Probe`], `#[derive(serde::Deserialize)]` +
//! `#[serde(deny_unknown_fields)]`; the four `pub enum Capability` **bodies are byte-equal** —
//! Checkpoint 7c digests the range from `pub enum Capability {` to the closing brace in each
//! crate and requires one digest; this copy is `rk-adapter-sqlite`'s, byte for byte — so four
//! copies stay one vocabulary, and Step 7d's bridge is
//! `CapabilityClaim::new(capability, verified_on, probe.as_str())`. A shared type in `rk-types`
//! or `rk-ports` would re-open Checkpoint 5 or 7a for a criterion no record homes; the body
//! below carries **no doc comment inside the braces** for the same reason — a comment in one
//! copy would be a second digest.
//!
//! **Every member is required** (AC-489): a row missing `capability`, `verified_on` or `probe`
//! fails to load naming the member, a row carrying a member the type does not declare fails
//! naming it (`deny_unknown_fields`, on the row and on the probe), and no `Default`, no
//! `#[serde(default)]` and no `Option` member exists to let one through. **`probe` is a closed
//! enum**, `Probe::{Recorded { source }, None { reason }}` — so a row is present, dated and
//! *explicitly* probe-less, never absent-member (§7c.7). **The vocabulary is closed** (AC-492): a
//! token outside [`Capability`] fails to load naming the token, and the one mapper from variant to
//! token, [`Capability::as_str`], is an exhaustive `match` with no catch-all arm (`ac492_…` reads
//! this file for one), so no code path maps an unknown token to a default, an empty set or a no-op.
//!
//! **The Claude entry, named** (§7c.7): `dispatch` — `verified_on: "2026-09-17"`,
//! `Probe::Recorded { source }` naming the corpus pin's recorded dispatches (`results/e3.jsonl` at
//! `61f590e5…`, 101 runner-typed rows; and the F20 transcript on the tree,
//! `crates/rk-kernel/tests/fixtures/findings/ac282_f20_golden_transcript.jsonl`) at Slice 4 —
//! **re-dated in the close commit to the probe's corpus commit** (§7c.15), the first dispatch at
//! the pinned 2.1.277. **Every capability the Harness port does not implement** — `snapshot`,
//! `rebuild` (the store's), `change_summary`, `affected-selection` (the toolchains') — is
//! **ABSENT from this registry, not `Probe::None`**: a Harness claims no store or toolchain
//! capability, as the SQLite registry claims no dispatch capability. [`claims`] is what the kernel's
//! resolver will read through 7d's bridge; nothing here answers `Unsupported` for a row it does
//! not hold, because the Harness port has no method that would.
//!
//! **Wire form.** JSON (the crate carries `serde_json` for the `--agents` element already): a
//! list of rows, `probe` externally tagged — `{"recorded": {"source": …}}` / `{"none": {"reason": …}}`.

use serde::Deserialize;

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

/// The Claude registry: the one row §7c.7 names — `dispatch`, dated the day the stage opened and
/// the corpus pin's recorded dispatches were read for it, its probe the record of those dispatches.
/// Re-dated in the close commit to the live probe's corpus commit (§7c.15).
const REGISTRY: &str = r#"[
  {
    "capability": "dispatch",
    "verified_on": "2026-09-17",
    "probe": { "recorded": { "source": "redkiln-corpus 61f590e51cbb00301c310e46aab615563a7787b8:results/e3.jsonl (101 recorded dispatch rows at the pin; the seven is_error frames and line 6's api_retries are this crate's fixtures at Slice 5) and crates/rk-kernel/tests/fixtures/findings/ac282_f20_golden_transcript.jsonl (the F20 row on the tree); the first dispatch at the pinned 2.1.277 is the live probe's, whose corpus commit re-dates this row at close (s7c section 7c.15)" } }
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

/// Does `rows` carry a row for `capability`?
///
/// What 7d's resolver reads: a capability with no row is not claimed, and a row's claim is exactly
/// its three members (AC-238's *"derived from the registry entry"*). Pure over the slice, so a test
/// hands it controlled rows.
#[must_use]
pub fn claims(rows: &[Row], capability: Capability) -> bool {
    rows.iter().any(|r| r.capability == capability)
}
