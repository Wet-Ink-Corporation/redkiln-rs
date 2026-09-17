//! The review outcome, rendered — a markdown table, **a rendering and never a source** (ADR-0006).
//!
//! AC-304's golden: a quarantined finding is *"still persisted and rendered with its quarantine
//! reason"* — visible, not deleted. Every record renders, in payload order, with its identity,
//! class, decision and (when quarantined) the reason, `by` and `until`. Deterministic by
//! construction: it reads the `Confirmed` value and nothing else.

use super::confirm::Confirmed;
use super::policy::Decision;

/// Render one confirm run's records as a markdown table.
#[must_use]
pub fn render_findings(confirmed: &Confirmed) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# Review {} at `{}` — weight `{}`\n\n",
        confirmed.review_id,
        confirmed.sha,
        confirmed.weight.as_str()
    ));
    out.push_str("| ac | finding_id | model label | class | decision | quarantine |\n");
    out.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for r in &confirmed.records {
        let decision = match &r.decision {
            Decision::Block(_) => "**BLOCK**".to_owned(),
            Decision::Advise(reason) => format!("advise ({})", reason.as_str()),
        };
        let quarantine = r.quarantine.as_ref().map_or_else(
            || "—".to_owned(),
            |q| {
                format!(
                    "quarantined until {} by {}: {}",
                    q.until.unix_seconds(),
                    q.by,
                    q.reason.as_str()
                )
            },
        );
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} |\n",
            r.ac,
            &r.finding_id.as_str()[..12],
            r.evidence_label.as_str(),
            r.computed_class.as_str(),
            decision,
            quarantine
        ));
    }
    out.push_str(&format!(
        "\n{} finding(s): {} block, {} in the advisory sink.\n",
        confirmed.records.len(),
        confirmed.block_set().len(),
        confirmed.records.len() - confirmed.block_set().len()
    ));
    out
}
