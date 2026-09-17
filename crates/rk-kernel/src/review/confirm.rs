//! The confirm half — findings in, per-finding decisions and the rows that record them out
//! (ADR-0074 clauses 3–6, 8; AC-285, AC-287, AC-295, AC-299, AC-304, AC-306, AC-307, AC-310,
//! AC-313, AC-530).
//!
//! **One pass, one row per fact, one code path per row.** For each finding, in payload order:
//!
//! 1. `ReadOnly` re-executes NOTHING (AC-299) — the re-executor is not called.
//! 2. The re-executor is asked; **no record** is `Unverifiable { cannot_run }` beside an `advisory`
//!    class (AC-287) — an outcome, never a change of class.
//! 3. The classifier computes the class from the repro and the observation; the model's label is
//!    compared for drift HERE, outside `policy`, and a `review_class_drift` row written on
//!    disagreement (AC-285).
//! 4. An **active quarantine** on the finding's identity (`until` after `now`) makes the decision
//!    `Advise(Quarantined)` — absent from the block set, persisted with its reason (AC-304); an
//!    **expired** one (`until` at or before `now`) writes `quarantine_expired` and the finding is
//!    re-raised, decided exactly as a new finding of its class at this weight (AC-306).
//! 5. Otherwise `policy::classify` decides. **A `Block` and its `review_block` row are the same
//!    arm** (AC-307: *"the block path and the event append are the same code path"*); every
//!    non-block is an `advisory_created` row with its class and reason (AC-310's intake).
//!
//! `now` is a journaled instant on [`ConfirmInput`] — this module reads no clock — and the whole
//! function is a pure map over its inputs, which is what AC-313's k=10 replay measures: the same
//! recorded payload and re-execution record give the same block set ten times, and that set is
//! non-empty for a fixture that has something to block.

use std::collections::{BTreeMap, BTreeSet};

use rk_store::artifacts::finding_identity::finding_identity;
use rk_types::artifact::findings::{Finding, Findings};
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::ReproKind;
use rk_types::review::Quarantine;
use rk_types::time::Timestamp;

use super::classify::{EvidenceClass, classify};
use super::policy::{Decision, NonBlockReason, ReviewWeight, classify as policy_classify};
use super::record::PersistedFinding;
use super::reexec::{ReExecution, ReExecutor, Unverifiable};

/// One confirm run's output.
#[derive(Debug, Clone, PartialEq)]
pub struct Confirmed {
    pub weight: ReviewWeight,
    pub sha: String,
    pub review_id: String,
    /// Every finding, in payload order, with its identity, computed class and decision.
    pub records: Vec<PersistedFinding>,
    /// The rows the confirm half writes, in the order it decided them: `review_class_drift`
    /// (AC-285), `quarantine_expired` (AC-306), `review_block` (AC-307), `advisory_created`
    /// (AC-310).
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

    /// The identities that landed in the advisory sink this run (every non-block).
    #[must_use]
    pub fn sink_identities(&self) -> BTreeSet<Digest> {
        self.records
            .iter()
            .filter(|r| matches!(r.decision, Decision::Advise(_)))
            .map(|r| r.finding_id.clone())
            .collect()
    }
}

/// Everything one confirm run reads.
#[derive(Debug, Clone)]
pub struct ConfirmInput<'a> {
    pub findings: &'a Findings,
    pub weight: ReviewWeight,
    /// The reviewed SHA.
    pub sha: &'a str,
    pub review_id: &'a str,
    /// A journaled instant, never a clock: the comparison AC-306's `until ± 1s` test injects.
    pub now: Timestamp,
    /// Active quarantines by finding identity (AC-305), as the store holds them.
    pub quarantines: &'a BTreeMap<Digest, Quarantine>,
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

/// AC-307's members, shared by the block row and the two override rows (ADR-0124).
pub(super) fn ac307_members(
    record: &PersistedFinding,
    weight: ReviewWeight,
    sha: &str,
    actor: &str,
) -> serde_json::Value {
    serde_json::json!({
        "actor": actor,
        "adjudication": serde_json::Value::Null,
        "ac": record.ac,
        "computed_class": record.computed_class.as_str(),
        "finding_id": record.finding_id.as_str(),
        "model_label": record.evidence_label.as_str(),
        "re_execution_id": record.re_execution_id,
        "sha": sha,
        "stage_weight": weight.as_str(),
    })
}

/// The `review_block` row (ADR-0124; AC-307's payload), plus — for a `kind = mutate` finding — the
/// suite name and the mutant patch's digest AC-289 wants on the block event.
#[must_use]
pub fn review_block_event(
    record: &PersistedFinding,
    finding: &Finding,
    weight: ReviewWeight,
    sha: &str,
) -> Event {
    let mut payload = ac307_members(record, weight, sha, "kernel");
    if finding.repro.kind == ReproKind::Mutate
        && let Some(m) = &finding.repro.mutation
        && let Some(obj) = payload.as_object_mut()
    {
        obj.insert(
            "mutation_suite".to_owned(),
            serde_json::Value::from(m.suite.as_str()),
        );
        obj.insert(
            "mutation_patch_digest".to_owned(),
            serde_json::Value::from(rk_types::digest::sha256(m.patch.as_bytes()).as_str()),
        );
    }
    Event::new(EventKind::ReviewBlock, payload)
}

/// The `advisory_created` row (design §5:722): the finding, verbatim, with its class and the reason
/// it did not block — ADR-0082 decision 1's *"same typed `Findings` shape as a blocking finding"*
/// carried on the tier-2 row until the durable table exists (`rk_store::artifacts`'s header).
#[must_use]
pub fn advisory_created_event(
    record: &PersistedFinding,
    finding: &Finding,
    reason: NonBlockReason,
    review_id: &str,
) -> Event {
    Event::new(
        EventKind::AdvisoryCreated,
        serde_json::json!({
            "ac": record.ac,
            "class": record.computed_class.as_str(),
            "finding": serde_json::to_value(finding).unwrap_or(serde_json::Value::Null),
            "finding_id": record.finding_id.as_str(),
            "reason": reason.as_str(),
            "review_id": review_id,
        }),
    )
}

/// The `quarantine_expired` row (ADR-0124; AC-306).
#[must_use]
pub fn quarantine_expired_event(
    finding_id: &Digest,
    ac: &str,
    until: Timestamp,
    review_id: &str,
) -> Event {
    Event::new(
        EventKind::QuarantineExpired,
        serde_json::json!({
            "ac": ac,
            "finding_id": finding_id.as_str(),
            "review_id": review_id,
            "until": until.unix_seconds(),
        }),
    )
}

/// Run the confirm half with no quarantines and no clock — the R2 signature, which AC-316 pins:
/// findings, a weight, a sha, a re-executor, and NO suite result.
#[must_use]
pub fn confirm(
    findings: &Findings,
    weight: ReviewWeight,
    sha: &str,
    reexecutor: &dyn ReExecutor,
) -> Confirmed {
    let none = BTreeMap::new();
    confirm_with(
        &ConfirmInput {
            findings,
            weight,
            sha,
            review_id: "review",
            now: Timestamp::from_unix_seconds(0),
            quarantines: &none,
        },
        reexecutor,
    )
}

/// Run the confirm half over one payload at one weight, one SHA and one journaled instant.
#[must_use]
pub fn confirm_with(input: &ConfirmInput<'_>, reexecutor: &dyn ReExecutor) -> Confirmed {
    let mut records = Vec::with_capacity(input.findings.items.len());
    let mut events = Vec::new();

    for finding in &input.findings.items {
        let finding_id = finding_identity(finding);

        // 1. AC-299: the read-only tier dispatches no re-execution — the call is not made.
        if input.weight == ReviewWeight::ReadOnly {
            let record = PersistedFinding {
                finding_id,
                ac: finding.ac.clone(),
                evidence_label: finding.evidence,
                computed_class: EvidenceClass::Advisory,
                re_execution: None,
                re_execution_id: None,
                sha: input.sha.to_owned(),
                weight: input.weight,
                decision: Decision::Advise(NonBlockReason::ReadOnly),
                quarantine: None,
            };
            events.push(advisory_created_event(
                &record,
                finding,
                NonBlockReason::ReadOnly,
                input.review_id,
            ));
            records.push(record);
            continue;
        }

        // 2. the re-execution, or its absence (AC-287)
        let rec = reexecutor.run(finding, input.sha);
        let (class, outcome, re_execution_id) = match &rec {
            Some(r) => (
                classify(&finding.repro, &r.observation),
                r.outcome,
                Some(r.id.clone()),
            ),
            None => (
                EvidenceClass::Advisory,
                ReExecution::Unverifiable(Unverifiable::CannotRun),
                None,
            ),
        };

        // 3. AC-285: drift is compared HERE, outside the policy module, and routed nowhere else
        if finding.evidence.as_str() != class.as_str() {
            events.push(review_class_drift_event(
                &finding.ac,
                finding.evidence.as_str(),
                class,
            ));
        }

        // 4. quarantine: active holds (AC-304); expired re-raises and is recorded (AC-306)
        let mut quarantine = None;
        let mut decision = None;
        if let Some(q) = input.quarantines.get(&finding_id) {
            if q.until.unix_seconds() > input.now.unix_seconds() {
                quarantine = Some(q.clone());
                decision = Some(Decision::Advise(NonBlockReason::Quarantined));
            } else {
                events.push(quarantine_expired_event(
                    &finding_id,
                    &finding.ac,
                    q.until,
                    input.review_id,
                ));
            }
        }

        // 5. the policy table, and the row for what it decided — one arm each
        let decision =
            decision.unwrap_or_else(|| policy_classify(class, input.weight, Some(&outcome)));
        let record = PersistedFinding {
            finding_id,
            ac: finding.ac.clone(),
            evidence_label: finding.evidence,
            computed_class: class,
            re_execution: Some(outcome),
            re_execution_id,
            sha: input.sha.to_owned(),
            weight: input.weight,
            decision: decision.clone(),
            quarantine,
        };
        match &decision {
            // AC-307: the block and its row are the SAME code path — this arm.
            Decision::Block(_) => {
                events.push(review_block_event(
                    &record,
                    finding,
                    input.weight,
                    input.sha,
                ));
            }
            // AC-310: every non-block is persisted to the sink with its class and reason.
            Decision::Advise(reason) => {
                events.push(advisory_created_event(
                    &record,
                    finding,
                    *reason,
                    input.review_id,
                ));
            }
        }
        records.push(record);
    }

    Confirmed {
        weight: input.weight,
        sha: input.sha.to_owned(),
        review_id: input.review_id.to_owned(),
        records,
        events,
    }
}

/// **AC-530 — a record leaves the sink only by a recorded disposition.** Every identity live in the
/// sink after the previous review that review `current` did not raise again gets one
/// `advisory_retired { finding_id, reason: not_re_raised, review_id }` row — *"never silently,
/// never by deletion"*. A re-raised finding carries the same identity (AC-305) and is not retired.
#[must_use]
pub fn retire_not_re_raised(
    previous_live: &BTreeSet<Digest>,
    current: &Confirmed,
    review_id: &str,
) -> Vec<Event> {
    let raised_now: BTreeSet<&Digest> = current.records.iter().map(|r| &r.finding_id).collect();
    previous_live
        .iter()
        .filter(|id| !raised_now.contains(id))
        .map(|id| {
            Event::new(
                EventKind::AdvisoryRetired,
                serde_json::json!({
                    "finding_id": id.as_str(),
                    "reason": "not_re_raised",
                    "review_id": review_id,
                }),
            )
        })
        .collect()
}
