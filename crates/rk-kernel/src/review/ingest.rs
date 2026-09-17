//! The review dispatch's result, ingested — a `Findings` payload or a typed error, never an empty
//! set standing in for an absent one (AC-282; Halt 3; F20).
//!
//! **The Halt 3 shape made mechanical.** F20 measured the platform returning `exit 0`,
//! `subtype: success`, a prose verdict in `result` and **no `structured_output`** when a card's
//! `tools` allowlist voided `--json-schema` — the E3 pilot dispatch, copied byte-identical into
//! `tests/fixtures/findings/ac282_f20_golden_transcript.jsonl`. A kernel that read that as "zero
//! findings" would approve by absence, which is `if (review && review.pass === false)` again
//! (`s3` §3a). So the boundary is one question: is `structured_output` a member of the result
//! event? If not, the step FAILS with `StepError::Schema` and no finding set is produced — empty or
//! otherwise.
//!
//! **No text-to-JSON extraction lives here or anywhere on the review path** (AC-318): the payload
//! is `structured_output` or it is an error, and the prose in `result` is never parsed. The lint
//! is `crate::module_boundary::review_path_no_prose_json_rule`, and
//! `tests/fixtures/lint-controls/ac318_review_parses_prose_as_json.rs` is the fixture that adds
//! exactly that call.

use rk_types::artifact::findings::{Findings, IngestError};
use rk_types::error::StepError;

/// How a review dispatch's result failed to yield findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewIngestError {
    /// The dispatch exited 0 with `subtype: success` and no `structured_output` block — F20's
    /// measured platform behaviour. A `StepError::Schema`, never a success with an empty set.
    MissingStructuredOutput(StepError),
    /// `structured_output` was present and is not a well-formed `Findings` (AC-281).
    Malformed(IngestError),
}

/// Ingest the `claude -p` stream-json `result` event of a review dispatch.
///
/// # Errors
///
/// [`ReviewIngestError::MissingStructuredOutput`] when the event carries no `structured_output`
/// member — whatever `subtype`, `is_error` or the prose `result` say;
/// [`ReviewIngestError::Malformed`] when it does and the payload fails `Findings::ingest`.
pub fn findings_from_result_event(
    result: &serde_json::Value,
) -> Result<Findings, ReviewIngestError> {
    let Some(payload) = result.get("structured_output") else {
        let subtype = result
            .get("subtype")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("?");
        return Err(ReviewIngestError::MissingStructuredOutput(
            StepError::Schema {
                attempts: 1,
                last_error: format!(
                    "missing structured_output: the review dispatch's result event carries no \
                 `structured_output` member (subtype `{subtype}`); a prose result is not a finding \
                 set and is not read as one (AC-282; F20)"
                ),
            },
        ));
    };
    Findings::ingest(payload.clone()).map_err(ReviewIngestError::Malformed)
}
