//! The review regression run's record (AC-314) — the CORPUS repository's sha, and a per-item
//! checkpoint sha under a differently named field.
//!
//! F37 measured the defect this exists for: `emit.corpus_sha` held a per-item checkpoint sha, so the
//! key was misnamed and every regression record pointed at an item's commit rather than the corpus
//! commit the run was measured against. Here the two are two fields, and [`RegressionRecord::at_pin`]
//! refuses a `corpus_sha` that is not the pinned corpus commit — which a per-item checkpoint sha
//! never is. The `doctor` rule that walks the corpus remote's history for an unknown sha is 7d's.

use serde::{Deserialize, Serialize};

/// One review regression run's record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegressionRecord {
    /// The corpus REPOSITORY's commit sha — `CORPUS_PIN`'s `CORPUS_SHA` for a run at the pin.
    pub corpus_sha: String,
    /// The reviewed ITEM's checkpoint sha — a different repository, a different field.
    pub item_checkpoint_sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegressionRecordError {
    /// `corpus_sha` is not the pinned corpus commit: a per-item checkpoint sha, or not a sha.
    NotTheCorpusSha { given: String, pin: String },
}

impl std::fmt::Display for RegressionRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotTheCorpusSha { given, pin } => write!(
                f,
                "corpus_sha `{given}` is not the pinned corpus commit `{pin}` — a per-item checkpoint \
                 sha belongs under item_checkpoint_sha (AC-314; F37)"
            ),
        }
    }
}

impl std::error::Error for RegressionRecordError {}

impl RegressionRecord {
    /// Build a record for a run at the pin.
    ///
    /// # Errors
    ///
    /// [`RegressionRecordError::NotTheCorpusSha`] when `corpus_sha` is not `pin`.
    pub fn at_pin(
        pin: &str,
        corpus_sha: &str,
        item_checkpoint_sha: &str,
    ) -> Result<Self, RegressionRecordError> {
        if corpus_sha != pin {
            return Err(RegressionRecordError::NotTheCorpusSha {
                given: corpus_sha.to_owned(),
                pin: pin.to_owned(),
            });
        }
        Ok(Self {
            corpus_sha: corpus_sha.to_owned(),
            item_checkpoint_sha: item_checkpoint_sha.to_owned(),
        })
    }
}
