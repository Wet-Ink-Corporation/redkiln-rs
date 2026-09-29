//! `AcceptanceCriterion` — one criterion of an item's spec, as the verify gate reads it.
//!
//! `s8g`'s schema record (R2 = ADR-0192 decision 3, proposed; D21-42 10a: *"One spec/pack schema
//! record fixes the three spellings and adds `rk_types::AcceptanceCriterion` (with ADR-0104's
//! mutation field) to `EXPORTED`"*). *(Added 2026-09-29, `s8g` build, package S8G-P2-RKTYPES.)*
//!
//! - **AC-137's GIVEN/THEN.** `gwt` and `verification` are optional in the TYPE: AC-137 accepts a
//!   criterion with neither, and whether an empty `verification` blocks a stage is AC-455's policy
//!   row (`verification_required`), never a constant here (AC-138; ADR-0192 decision 5).
//! - **ADR-0104 decision 1's `mutation`**, in ADR-0084's discriminated form: a `kind`-tagged
//!   declaration, `declared { patch, suite }` or `not_feasible { reason }`. It supersedes the
//!   fixture reader's `[ac.mutation] not_feasible = "<reason>"` (ADR-0192 decision 1).
//! - **One door.** [`AcceptanceCriterion::ingest`] deserialises and then validates, so a prose-only
//!   mutation is refused at ingest with the criterion named (ADR-0096 decision 1's rule, applied to
//!   the author's declaration as `Findings::ingest` applies it to a repro).
//!
//! `s8h`'s `Spec` embeds this type unchanged under the member `acceptance_criteria` (ADR-0192
//! decision 7) and redeclares none of its fields.

use crate::text::Text;
use serde::{Deserialize, Serialize};

/// A criterion's GIVEN/WHEN/THEN, with any number of AND clauses (AC-137).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Gwt {
    pub given: Text,
    pub when: Text,
    pub then: Text,
    /// The AND clauses. Optional, so declared LAST (AC-158).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<Text>,
}

/// What the author declared about a criterion's mutation (ADR-0104 decision 1), in ADR-0084's
/// `kind`-discriminated form. The export transform renders it as an `anyOf` (ADR-0095 decision 1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum MutationDeclaration {
    /// A unified diff the harness applies in an ephemeral tree, and the suite it runs.
    Declared { patch: String, suite: String },
    /// The mutation cannot be written, and why (AC-337).
    NotFeasible { reason: Text },
}

/// One acceptance criterion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceCriterion {
    pub id: String,
    pub criterion: Text,
    /// AC-137's GIVEN/WHEN/THEN, when the author wrote one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gwt: Option<Gwt>,
    /// The verification the author points at. Absent is admitted by the type (AC-137); the policy
    /// row decides whether it blocks a stage (AC-455, AC-138).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Text>,
    /// The author's mutation declaration (ADR-0104 decision 1). Absent means none was declared,
    /// which the ledger reads as unpinned (AC-333).
    ///
    /// Exported as the declaration's own two-branch union and left out of `required`, with no
    /// `null` branch: a nullable union would be a union nested inside a union, which AC-131
    /// forbids. The field is omitted when absent and never written as `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "MutationDeclaration")]
    pub mutation: Option<MutationDeclaration>,
}

/// Why a criterion was refused at ingest. A refused criterion never reaches the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CriterionIngestError {
    /// The value did not deserialise into the typed shape at all — a sentence where the
    /// declaration belongs is this.
    Malformed { detail: String },
    /// A mutation declaration whose content is prose: a `declared` patch that is not a unified
    /// diff, a `declared` arm naming no suite, or a `not_feasible` arm with no reason.
    ProseOnlyMutation { ac: String, detail: String },
}

impl std::fmt::Display for CriterionIngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Malformed { detail } => write!(f, "malformed acceptance criterion: {detail}"),
            Self::ProseOnlyMutation { ac, detail } => {
                write!(
                    f,
                    "{ac}: a prose-only mutation is refused at ingest: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for CriterionIngestError {}

/// Is this a unified diff: a `---`/`+++` header pair and at least one `@@` hunk header?
///
/// SCOPE: presence and SHAPE only. Whether the patch APPLIES is the harness's to find out, in its
/// ephemeral tree (ADR-0191 decision 11); this touches no filesystem.
fn is_unified_diff(patch: &str) -> bool {
    let mut minus = false;
    let mut plus = false;
    let mut hunk = false;
    for line in patch.lines() {
        if line.starts_with("--- ") {
            minus = true;
        } else if line.starts_with("+++ ") {
            plus = true;
        } else if line.starts_with("@@ ") {
            hunk = true;
        }
    }
    minus && plus && hunk
}

impl AcceptanceCriterion {
    /// The one door a criterion comes through: deserialise, then validate the mutation.
    ///
    /// # Errors
    ///
    /// [`CriterionIngestError`] naming the criterion.
    pub fn ingest(value: serde_json::Value) -> Result<Self, CriterionIngestError> {
        let ac: Self =
            serde_json::from_value(value).map_err(|e| CriterionIngestError::Malformed {
                detail: e.to_string(),
            })?;
        let prose = match &ac.mutation {
            Some(MutationDeclaration::Declared { patch, .. }) if !is_unified_diff(patch) => {
                Some("the declared patch is not a unified diff")
            }
            Some(MutationDeclaration::Declared { suite, .. }) if suite.trim().is_empty() => {
                Some("the declared mutation names no suite")
            }
            Some(MutationDeclaration::NotFeasible { reason })
                if reason.as_str().trim().is_empty() =>
            {
                Some("not_feasible carries no reason (AC-337)")
            }
            None
            | Some(
                MutationDeclaration::Declared { .. } | MutationDeclaration::NotFeasible { .. },
            ) => None,
        };
        match prose {
            Some(detail) => Err(CriterionIngestError::ProseOnlyMutation {
                ac: ac.id,
                detail: detail.to_owned(),
            }),
            None => Ok(ac),
        }
    }
}
