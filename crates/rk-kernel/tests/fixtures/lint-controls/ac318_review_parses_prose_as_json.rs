//! **AC-318's lint-fail fixture**: *"an architecture lint over the review path's call graph asserting
//! zero calls into any text-to-JSON extraction helper, plus a lint-fail fixture that adds one"*.
//! NOT compiled. This is the Halt 3 shape made mechanical: a review path that, finding no
//! `structured_output`, goes looking for JSON inside the prose `result` — F20 measured the platform
//! returning exit 0 / `subtype: success` with the schema voided, and prose extraction is how that
//! silently becomes a verdict.

pub fn findings_from_result_event(result: &serde_json::Value) -> Result<Findings, ReviewIngestError> {
    if let Some(payload) = result.get("structured_output") {
        return Findings::ingest(payload.clone()).map_err(ReviewIngestError::Malformed);
    }
    let prose = result["result"].as_str().unwrap_or_default();
    let start = prose.find('{').unwrap_or(0);
    let end = prose.rfind('}').map_or(prose.len(), |e| e + 1);
    let fenced = prose.trim_start_matches("```json").trim_end_matches("```");
    let extracted: serde_json::Value = serde_json::from_str(&prose[start..end]).or_else(|_| serde_json::from_str(fenced))?;
    Findings::ingest(extracted).map_err(ReviewIngestError::Malformed)
}
