//! The confirm half — findings in, per-finding decisions and the rows that record them out
//! (ADR-0074 clauses 3–5, 8; AC-285, AC-295, AC-299; AC-307/AC-310/AC-313 land here in slice R3).
//!
//! Three things this function holds, in the order the tests red them:
//!
//! 1. **`ReadOnly` re-executes NOTHING** (AC-299). The re-executor is not called; every finding
//!    carries `NonBlockReason::ReadOnly`. An empty block set is satisfied by a tier that dispatched
//!    and then discarded — the process counter is the discriminating half, and it is zero because
//!    the call is never made, not because the result is dropped.
//! 2. **The computed class is used; the model label is retained and drift is recorded** (AC-285).
//!    Where `finding.evidence` disagrees with the classifier's answer, one `review_class_drift`
//!    row carries both; the block set follows the class.
//! 3. **One re-execution per finding per run, whatever the weight** (AC-295): changing the weight
//!    changes the lookup, never the number of dispatches or the classifier's output.
//!
//! Nothing here reads the model label for a decision: the label is copied onto the persisted
//! record and compared for drift, in this module — outside `policy` (AC-285's lint boundary).

use rk_types::artifact::findings::Findings;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use super::classify::{EvidenceClass, classify};
use super::policy::{Decision, NonBlockReason, ReviewWeight, classify as policy_classify};
use super::record::PersistedFinding;
use super::reexec::ReExecutor;

/// One confirm run's output.
#[derive(Debug, Clone, PartialEq)]
pub struct Confirmed {
    pub weight: ReviewWeight,
    pub sha: String,
    /// Every finding, in payload order, with its computed class and decision.
    pub records: Vec<PersistedFinding>,
    /// The rows the confirm half writes: `review_class_drift` (AC-285); slice R3 adds the block
    /// and sink rows (AC-307, AC-310).
    pub events: Vec<Event>,
}

impl Confirmed {
    /// The block set: the findings whose decision is `Block`, in payload order.
    #[must_use]
    pub fn block_set(&self) -> Vec<&PersistedFinding> {
        self.records
            .iter()
            .filter(|r| matches!(r.decision, Decision::Block(_)))
            .collect()
    }

    /// The computed classes, in payload order — what AC-295 byte-compares across weights.
    #[must_use]
    pub fn classes(&self) -> Vec<EvidenceClass> {
        self.records.iter().map(|r| r.computed_class).collect()
    }
}

/// The `review_class_drift` row (design §5:718): the finding, the model's label, the computed class.
#[must_use]
pub fn review_class_drift_event(
    ac: &str,
    model_label: &str,
    computed_class: EvidenceClass,
) -> Event {
    Event::new(
        EventKind::ReviewClassDrift,
        serde_json::json!({
            "ac": ac,
            "computed_class": computed_class.as_str(),
            "model_label": model_label,
        }),
    )
}

/// Run the confirm half over one `Findings` payload at one weight and one reviewed SHA.
#[must_use]
pub fn confirm(
    findings: &Findings,
    weight: ReviewWeight,
    sha: &str,
    reexecutor: &dyn ReExecutor,
) -> Confirmed {
    let mut records = Vec::with_capacity(findings.items.len());
    let mut events = Vec::new();

    for finding in &findings.items {
        // AC-299: the read-only tier dispatches no re-execution — the call is not made.
        if weight == ReviewWeight::ReadOnly {
            records.push(PersistedFinding {
                ac: finding.ac.clone(),
                evidence_label: finding.evidence,
                computed_class: EvidenceClass::Advisory,
                re_execution: None,
                decision: Decision::Advise(NonBlockReason::ReadOnly),
            });
            continue;
        }

        let rec = reexecutor.run(finding, sha);
        let class = classify(&finding.repro, &rec.observation);
        // AC-285: the model's label is compared for drift HERE, outside the policy module, and
        // routed nowhere else.
        if finding.evidence.as_str() != class.as_str() {
            events.push(review_class_drift_event(
                &finding.ac,
                finding.evidence.as_str(),
                class,
            ));
        }
        let decision = policy_classify(class, weight, Some(&rec.outcome));
        records.push(PersistedFinding {
            ac: finding.ac.clone(),
            evidence_label: finding.evidence,
            computed_class: class,
            re_execution: Some(rec.outcome),
            decision,
        });
    }

    Confirmed {
        weight,
        sha: sha.to_owned(),
        records,
        events,
    }
}
