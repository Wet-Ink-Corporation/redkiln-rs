//! Pack-validation rules for the review tier (AC-300, AC-301; ADR-0015).
//!
//! - **AC-300** — *"`bug-standard` is execution-grade because the fix lane is where the 52% lives"*
//!   (ADR-0015). The fix lane IS the `bug-standard` pack; its review stage must declare
//!   `mandatory` (→ `Strict`), and a pack that lowers it fails validation. The `doctor` rule-table
//!   row is 7d's.
//! - **AC-301** — *"Cascades are refused, not priced"* (design §9.3). A pack configuring a
//!   cheap-first review cascade — a `[review] cascade = […]` table, a read-only pass gating whether
//!   the execution-grade review runs — is refused with the one reason the lift is keyed on:
//!   [`CASCADE_REASON`]. S-CASCADE is registered and unrun; the refusal lifts only when it reports,
//!   and that lift is not this stage's.

use crate::review::policy::{ReviewWeight, resolve_review_tier};
use crate::select::Pack;
use crate::toml_subset::{Table, Value};

/// The pack that is the fix lane (ADR-0015; AC-300).
pub const FIX_LANE_PACK: &str = "bug-standard";

/// AC-301's refusal reason — the exact string, because the lift is keyed on it.
pub const CASCADE_REASON: &str = "cascade recall loss unmeasured (S-CASCADE unrun)";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewPackError {
    /// The fix lane's review weight is below execution-grade (AC-300).
    FixLaneNotStrict { pack: String, declared: String },
    /// A cheap-first cascade is configured (AC-301).
    CascadeRefused { pack: String, reason: &'static str },
}

impl std::fmt::Display for ReviewPackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FixLaneNotStrict { pack, declared } => write!(
                f,
                "{pack}: the fix lane's review weight is `{declared}`; it must be execution-grade \
                 (`mandatory` → Strict, ADR-0015)"
            ),
            Self::CascadeRefused { pack, reason } => write!(f, "{pack}: {reason}"),
        }
    }
}

impl std::error::Error for ReviewPackError {}

/// **AC-300.** The fix lane reviews at `Strict`, or the pack fails validation.
///
/// # Errors
///
/// [`ReviewPackError::FixLaneNotStrict`] naming the declared weight (`unset` when the stage
/// declares none — which resolves to `Lenient`, AC-298, and is therefore also a lowering).
pub fn fix_lane_review_is_strict(pack: &Pack) -> Result<(), ReviewPackError> {
    if pack.id != FIX_LANE_PACK {
        return Ok(());
    }
    let declared = pack.rigor("review");
    if resolve_review_tier(declared) == ReviewWeight::Strict {
        return Ok(());
    }
    Err(ReviewPackError::FixLaneNotStrict {
        pack: pack.id.clone(),
        declared: declared.map_or_else(|| "unset".to_owned(), |r| r.as_str().to_owned()),
    })
}

/// **AC-301.** A pack whose `[review]` table carries a `cascade` key (or whose `[policy]` carries
/// `review_cascade`) is refused.
///
/// # Errors
///
/// [`ReviewPackError::CascadeRefused`] with [`CASCADE_REASON`].
pub fn cascade_is_refused(pack_id: &str, doc: &Table) -> Result<(), ReviewPackError> {
    let configures_cascade = doc
        .get("review")
        .and_then(Value::as_table)
        .is_some_and(|review| review.contains_key("cascade"))
        || doc
            .get("policy")
            .and_then(Value::as_table)
            .is_some_and(|policy| policy.contains_key("review_cascade"));
    if configures_cascade {
        return Err(ReviewPackError::CascadeRefused {
            pack: pack_id.to_owned(),
            reason: CASCADE_REASON,
        });
    }
    Ok(())
}
