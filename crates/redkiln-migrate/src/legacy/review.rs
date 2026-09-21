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
    let fenced = super::fence::split(path, src)?;
    let block: serde_json::Map<String, serde_json::Value> = yaml_serde::from_str(&fenced.block)
        .map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("the fenced review block is not valid YAML: {e}"),
        })?;

    let need = |key: &str| -> Result<String, LegacyError> {
        block
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| LegacyError::MissingKey {
                path: path.to_path_buf(),
                key: key.to_owned(),
            })
    };

    // `verdict` and `rubric` have no field on ReviewCompanion { item, stage, summary }, which
    // carries deny_unknown_fields. They are stripped into markers HERE rather than dropped, which
    // is why this reader exists instead of a bare `fence::read::<ReviewCompanion>`.
    let mut losses = Vec::new();
    for key in [VERDICT_KEY, RUBRIC_KEY] {
        if let Some(value) = block.get(key) {
            losses.push(LossMarker {
                what: format!("key `{key}`"),
                why: "rk_types::companion::ReviewCompanion declares { item, stage, summary } and \
                      nothing else; §7d.15's floor forbids widening rk-types, so the key is \
                      carried opaque beside the typed value rather than dropped"
                    .to_owned(),
                opaque: yaml_serde::to_string(value)
                    .unwrap_or_default()
                    .trim_end()
                    .to_owned(),
            });
        }
    }

    Ok(LegacyReview {
        item: need("item")?,
        stage: need("stage")?,
        summary: need("summary")?,
        losses,
    })
}

/// Read a legacy free-form gate result (`report.md`) as opaque prose with a loss marker.
///
/// Never returns a typed gate result — see the module header.
///
/// # Errors
/// [`LegacyError::Unreadable`] if the file cannot be read.
pub fn parse_free_form_gate_result(path: &Path, src: &str) -> Result<LossMarker, LegacyError> {
    if src.trim().is_empty() {
        return Err(LegacyError::Unreadable {
            path: path.to_path_buf(),
            detail: "the file is empty".to_owned(),
        });
    }
    // Everything after the frontmatter fence, when there is one; otherwise the whole file. The
    // prose is carried VERBATIM — summarising it would be the reconstruction S-MIG decision 2 and
    // AC-478's second conjunct both forbid.
    let prose = super::frontmatter::parse(path, src)
        .map(|fm| fm.body)
        .unwrap_or_else(|_| src.to_owned());

    Ok(LossMarker {
        what: "free-form gate result".to_owned(),
        why: "a gate result is emitted by the toolchain and rendered, never parsed (S-MIG \
              decision 2); this one was written as prose with no structured verdict, so it is \
              carried opaque and NO typed gate result is synthesised from it"
            .to_owned(),
        opaque: prose.trim().to_owned(),
    })
}
