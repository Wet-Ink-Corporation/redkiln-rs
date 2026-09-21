//! The legacy `_review.md` sidecar, and the free-form gate result beside it.
//!
//! # The two keys with no home, stated rather than discovered
//!
//! [`rk_types::companion::ReviewCompanion`] is `{ item, stage, summary }` with
//! `#[serde(deny_unknown_fields)]` — all five sidecar types expand identically from one macro
//! [measured 2026-09-21, `rk-types/src/companion.rs`]. The predecessor's `_review.md` carries
//! **`verdict` and `rubric`**, and neither has a field to land in. §7d.15's floor forbids widening
//! `rk-types`, so:
//!
//! * `_review` lifts **typed**, as `adr0075_six_typed_kinds_eight_opaque` requires of all six; and
//! * it counts **`lifted_with_loss`**, with a marker naming `verdict` and `rubric` as carried
//!   opaque beside the typed value.
//!
//! Checkpoint 7d pins the exact counts of `report` and `spec` only, and requires `loss >= 1`
//! overall, so this is consistent with the block and with the criterion. It is recorded here
//! because a test may not claim more than it asserts, and "lifted clean" would have been a claim.
//!
//! # A free-form gate result is never parsed into a typed one
//!
//! S-MIG decision 2: a gate result is *"emitted by the toolchain and rendered, never parsed"*. The
//! legacy `report.md`'s free-form result is therefore carried as opaque prose with a loss marker
//! (AC-477's carry) — reconstructing a typed verdict from prose is exactly what AC-478's second
//! conjunct forbids.

use std::path::Path;

use super::{LegacyError, LossMarker};

/// The legacy `verdict` key — carried opaque, never mapped onto a typed verdict.
pub const VERDICT_KEY: &str = "verdict";

/// The legacy `rubric` key — carried opaque.
pub const RUBRIC_KEY: &str = "rubric";

/// A legacy `_review.md`, as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyReview {
    /// The item the review belongs to.
    pub item: String,
    /// The stage it was taken at.
    pub stage: String,
    /// The prose that becomes [`rk_types::companion::ReviewCompanion`]'s `summary`.
    pub summary: String,
    /// `verdict` and `rubric`, carried opaque beside the typed value.
    pub losses: Vec<LossMarker>,
}

/// Read a legacy `_review.md`.
///
/// # Errors
/// [`LegacyError::Malformed`] if the fence is absent or its block is not valid YAML;
/// [`LegacyError::MissingKey`] if `item` or `stage` is absent.
pub fn parse(path: &Path, src: &str) -> Result<LegacyReview, LegacyError> {
    let _ = (path, src);
    todo!("cycle 2: read the block, keep verdict/rubric as opaque loss markers")
}

/// Read a legacy free-form gate result (`report.md`) as opaque prose with a loss marker.
///
/// Never returns a typed gate result — see the module header.
///
/// # Errors
/// [`LegacyError::Unreadable`] if the file cannot be read.
pub fn parse_free_form_gate_result(path: &Path, src: &str) -> Result<LossMarker, LegacyError> {
    let _ = (path, src);
    todo!("cycle 2: carry the free-form gate result opaque with its marker")
}
