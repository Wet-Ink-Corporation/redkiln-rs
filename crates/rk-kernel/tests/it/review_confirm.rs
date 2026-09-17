//! **The review gate, R3 — re-execution, quarantine, the events, the sink, replay** — Step 7b-ii,
//! `s7b-ii` §7b-ii.5–.8.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac287_no_record_no_block`] | **AC-287** | no re-execution record → `Unverifiable { cannot_run }` beside the class, and an empty block set | the corpus replay `blocks ⊆ re-executed` (Step 8) |
//! | [`ac288_four_cell_truth_table`] | **AC-288** | the four cells: only fail-on-candidate ∧ pass-on-reference blocks; fails-on-both is `Unverifiable { both_fail }` | — |
//! | [`ac289_equivalent_mutant_does_not_block`] | **AC-289** | the S0.1 set: the one survivor blocks, the nine killed do not; the equivalent mutant (no-op, green, no assertion) does not; the block row carries suite and patch digest | the workflow-only fixture (S0.1-SCOPE) |
//! | [`ac290_unverifiable_never_blocks`] | **AC-290** | three negative fixtures, one per reason; no `Unverifiable` in any block set; the reason is on the record beside the class | the `doctor` rule (7d) |
//! | [`ac304_quarantined_is_absent_but_persisted`] | **AC-304** | a quarantined finding is absent from the block set AND still persisted and rendered with its reason (golden); quarantining an advisory is refused | — |
//! | [`ac306_quarantine_expiry_re_raises`] | **AC-306** | at `until − 1s` the quarantine holds; at `until + 1s` the finding is re-raised, `quarantine_expired` is written, and it blocks iff its class blocks at that weight — at BOTH Lenient and Strict | the `doctor` row |
//! | [`ac307_every_block_has_an_event`] | **AC-307** | `count(blocks) == count(review_block rows)` with the payload the register names; the block path and the append are one code path (source rule) | the corpus replay (Step 8) |
//! | [`ac530_sink_leaves_only_by_recorded_disposition`] | **AC-530** | a two-review run: the record not re-raised gets `advisory_retired { not_re_raised, review_id }` and leaves; `live == created − retired − dismissed` over created, retired and dismissed | the `doctor` row |
//! | [`ac313_replay_k10`] | **AC-313** | ten runs of the confirm half over one recorded payload + record: identical block sets AND equal to the pinned NON-EMPTY set | — |
//! | [`ac314_corpus_sha_is_the_corpus_repos`] | **AC-314** | the regression record refuses a per-item checkpoint sha in `corpus_sha` and names the two fields apart | the `doctor` history walk (7d) |
//! | [`ac315_review_never_marks_an_ac_verified`] | **AC-315** | the review module names no ledger type (source rule); a clean review leaves every AC unpinned | — |
//! | [`ac316_green_suite_does_not_clear_a_block`] | **AC-316** | `confirm` takes no suite result (signature); a flipped block stands; nothing in the review module reads a verify result | — |
//!
//! Red first, each on its own claim, against a confirm half that wrote no block rows, no sink rows,
//! knew no quarantine and had no four-cell rule — the outputs are in the slice's commit.

use std::collections::{BTreeMap, BTreeSet};

use rk_kernel::module_boundary::{
    self, SourceFile, block_path_appends_its_own_event_rule, review_touches_no_ledger_rule,
};
use rk_kernel::review::classify::{EvidenceClass, MutateObservation, Observation, RunObservation};
use rk_kernel::review::confirm::{
    ConfirmInput, Confirmed, confirm, confirm_with, retire_not_re_raised,
};
use rk_kernel::review::policy::{Decision, NonBlockReason, ReviewWeight};
use rk_kernel::review::quarantine::{QuarantineError, grant};
use rk_kernel::review::reexec::{
    ReExecution, ReExecutionRecord, ReExecutor, Unverifiable, four_cell, mutation_outcome,
};
use rk_kernel::review::regression::{RegressionRecord, RegressionRecordError};
use rk_kernel::review::render::render_findings;
use rk_store::artifacts::finding_identity::finding_identity;
use rk_store::artifacts::sink::sink_accounting;
use rk_types::artifact::findings::{EvidenceLabel, Finding, Findings};
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::{Expected, Locator, Mutation, Repro, ReproKind};
use rk_types::review::Quarantine;
use rk_types::text::Text;
use rk_types::time::Timestamp;

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";
const S01: &str = include_str!("../fixtures/findings/ac289_s01/mutants.json");
const AC304_GOLDEN: &str = include_str!("../fixtures/findings/ac304_rendering.golden.md");

fn run_repro(command: &str) -> Repro {
    Repro {
        kind: ReproKind::Run,
        command: command.to_owned(),
        expected: Expected::Exit { exit: 0 },
        observed: "exit 101".to_owned(),
        expected_when_met: "exit 0".to_owned(),
        mutation: None,
    }
}

fn mutate_repro(patch: &str, suite: &str, with_assertion: bool) -> Repro {
    Repro {
        kind: ReproKind::Mutate,
        command: suite.to_owned(),
        expected: if with_assertion {
            Expected::Assertion {
                assertion: Locator {
                    file: "test/x.test.ts".to_owned(),
                    test: "pins it".to_owned(),
                },
            }
        } else {
            Expected::Exit { exit: 0 }
        },
        observed: "suite green".to_owned(),
        expected_when_met: "suite red".to_owned(),
        mutation: Some(Mutation {
            patch: patch.to_owned(),
            suite: suite.to_owned(),
        }),
    }
}

fn finding(ac: &str, label: EvidenceLabel, repro: Repro) -> Finding {
    Finding {
        ac: ac.to_owned(),
        claim: Text::new(format!("{ac}: the claim")),
        evidence: label,
        rationale: Text::new("the rationale"),
        repro,
        files: vec![],
    }
}

fn ran(observed: &str) -> Observation {
    Observation::Run(RunObservation {
        ran: true,
        observed: observed.to_owned(),
        only_assertion_flipped: false,
    })
}

fn survived(reverted_fix: bool, assertion_flipped: Option<bool>) -> Observation {
    mutated(true, reverted_fix, assertion_flipped)
}

/// An applied mutant the gate saw with the suite `green` or not (`killed` is `green == false`).
const fn mutated(green: bool, reverted_fix: bool, assertion_flipped: Option<bool>) -> Observation {
    Observation::Mutate(MutateObservation {
        applied: true,
        suite_green: green,
        reverted_fix,
        assertion_flipped,
    })
}

/// Scripted re-executions keyed by AC; an AC with no row is "no record" (AC-287).
struct Scripted(BTreeMap<String, (Observation, ReExecution)>);

impl Scripted {
    fn new(rows: Vec<(&str, Observation, ReExecution)>) -> Self {
        Self(
            rows.into_iter()
                .map(|(a, o, r)| (a.to_owned(), (o, r)))
                .collect(),
        )
    }
}

impl ReExecutor for Scripted {
    fn run(&self, finding: &Finding, sha: &str) -> Option<ReExecutionRecord> {
        self.0
            .get(&finding.ac)
            .cloned()
            .map(|(observation, outcome)| ReExecutionRecord {
                id: format!("reexec:{}:{sha}", finding.ac),
                observation,
                outcome,
            })
    }
}

const fn at(secs: i64) -> Timestamp {
    Timestamp::from_unix_seconds(secs)
}

fn input<'a>(
    findings: &'a Findings,
    weight: ReviewWeight,
    quarantines: &'a BTreeMap<Digest, Quarantine>,
    now: i64,
) -> ConfirmInput<'a> {
    ConfirmInput {
        findings,
        weight,
        sha: SHA,
        review_id: "review-1",
        now: at(now),
        quarantines,
    }
}

const fn no_quarantines() -> BTreeMap<Digest, Quarantine> {
    BTreeMap::new()
}

fn blocks(c: &Confirmed) -> Vec<String> {
    c.block_set().iter().map(|r| r.ac.clone()).collect()
}

fn count(c: &Confirmed, kind: EventKind) -> usize {
    c.events.iter().filter(|e| e.kind == kind).count()
}

// ------------------------------------------------------------------ AC-287 / AC-288 / AC-290

/// **AC-287 — no re-execution record, no block; the outcome is `Unverifiable { cannot_run }` beside the class.**
///
/// RED (observed 2026-09-17): the skeleton executor returned a record for everything; a finding with
/// none had no `cannot_run` outcome to carry.
#[test]
fn ac287_no_record_no_block() {
    let findings = Findings {
        items: vec![
            finding(
                "AC-101",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac101"),
            ),
            finding(
                "AC-102",
                EvidenceLabel::MutationSurvived,
                mutate_repro("--- a\n+++ b\n-fix\n", "cargo test", false),
            ),
        ],
    };
    // an EMPTY re-execution record: the gate produced nothing for either finding
    let out = confirm(&findings, ReviewWeight::Strict, SHA, &Scripted::new(vec![]));
    assert!(
        out.block_set().is_empty(),
        "no record → no block: {:?}",
        blocks(&out)
    );
    for r in &out.records {
        assert_eq!(
            r.re_execution,
            Some(ReExecution::Unverifiable(Unverifiable::CannotRun)),
            "{}: the outcome is cannot_run, carried beside the class",
            r.ac
        );
        assert_eq!(
            r.decision,
            Decision::Advise(NonBlockReason::Unverifiable(Unverifiable::CannotRun))
        );
    }
    // the class is NOT changed by the absence: with no observation the classifier says advisory,
    // and the outcome enum — not the class enum — is where `Unverifiable` lives
    assert!(
        out.records
            .iter()
            .all(|r| EvidenceClass::ALL.contains(&r.computed_class))
    );
    assert_eq!(count(&out, EventKind::ReviewBlock), 0);
}

/// **AC-288 — the four cells; the "fails on both" cell is the one a flip-only implementation gets wrong.**
///
/// RED (observed 2026-09-17): `four_cell` did not exist; against a stub returning `Flipped` for
/// every fail-on-candidate cell, "fails on both must NOT block" failed.
#[test]
fn ac288_four_cell_truth_table() {
    // (candidate fails, reference fails) → outcome
    assert_eq!(
        four_cell(true, false),
        ReExecution::Flipped,
        "fails on the candidate, passes on the reference: the flip"
    );
    assert_eq!(
        four_cell(true, true),
        ReExecution::Unverifiable(Unverifiable::BothFail),
        "fails on both: cannot separate"
    );
    assert_eq!(
        four_cell(false, false),
        ReExecution::DidNotFlip,
        "passes on both: the claim did not reproduce"
    );
    assert_eq!(
        four_cell(false, true),
        ReExecution::DidNotFlip,
        "passes on the candidate, fails on the reference: not the candidate's fault"
    );

    // through the whole path: the flipping fixture blocks, its both-fail pair does not
    let findings = Findings {
        items: vec![
            finding(
                "AC-201",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac201"),
            ),
            finding(
                "AC-202",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac202"),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        ("AC-201", ran("exit 101"), four_cell(true, false)),
        ("AC-202", ran("exit 101"), four_cell(true, true)),
    ]);
    let out = confirm(&findings, ReviewWeight::Lenient, SHA, &exec);
    assert_eq!(
        blocks(&out),
        vec!["AC-201"],
        "only the flipping cell blocks"
    );
    assert_eq!(
        out.records[1].re_execution,
        Some(ReExecution::Unverifiable(Unverifiable::BothFail))
    );
    assert_eq!(
        out.records[1].computed_class,
        EvidenceClass::Reproduced,
        "…and its CLASS is unchanged by the outcome"
    );
}

/// **AC-290 — `Unverifiable` never blocks and is never silently recorded as passing.**
///
/// RED (observed 2026-09-17): the `timeout` fixture had no reason to carry.
#[test]
fn ac290_unverifiable_never_blocks() {
    let findings = Findings {
        items: vec![
            finding(
                "AC-301",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- both-fail"),
            ),
            finding(
                "AC-302",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- cannot-run"),
            ),
            finding(
                "AC-303",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- timeout"),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        (
            "AC-301",
            ran("exit 101"),
            ReExecution::Unverifiable(Unverifiable::BothFail),
        ),
        (
            "AC-302",
            Observation::Unrecognised,
            ReExecution::Unverifiable(Unverifiable::CannotRun),
        ),
        (
            "AC-303",
            Observation::Unrecognised,
            ReExecution::Unverifiable(Unverifiable::Timeout),
        ),
    ]);
    for weight in ReviewWeight::ALL {
        let out = confirm(&findings, weight, SHA, &exec);
        assert!(
            out.block_set().is_empty(),
            "{weight:?}: an Unverifiable finding blocked: {:?}",
            blocks(&out)
        );
        // never silently recorded as passing: every record carries its reason, and every
        // non-block is a sink record with that reason (AC-310's intake)
        if weight != ReviewWeight::ReadOnly {
            for (r, want) in out.records.iter().zip([
                Unverifiable::BothFail,
                Unverifiable::CannotRun,
                Unverifiable::Timeout,
            ]) {
                assert_eq!(
                    r.re_execution,
                    Some(ReExecution::Unverifiable(want)),
                    "{}: the reason is on the record",
                    r.ac
                );
            }
            assert_eq!(
                count(&out, EventKind::AdvisoryCreated),
                3,
                "{weight:?}: three sink rows, none dropped"
            );
        }
    }
    // the class enum has four variants and no `Unverifiable`
    assert_eq!(EvidenceClass::ALL.len(), 4);
}

// ------------------------------------------------------------------ AC-289

/// **AC-289 — a surviving mutant blocks only when it discriminates; the S0.1 set and the equivalent mutant.**
///
/// The ten S0.1 mutants are reverted-fix patches (`gate.rs::apply_mutation` reverted the fix's
/// production code), so each DISCRIMINATES by construction; whether it blocks is whether the suite
/// stayed green — one did (`issue-75`), nine did not. The patches were derived from the bundle at
/// the pin (`fixtures/findings/README.md`) and the gate's run is SCRIPTED from the recorded verdict:
/// the Toolchain fake stands in for a real suite, which is why AC-334 is ARMED at this stage.
///
/// RED (observed 2026-09-17): `mutation_outcome` did not exist; against a stub that called every
/// applied-and-green mutant `Flipped`, the equivalent mutant blocked.
#[test]
fn ac289_equivalent_mutant_does_not_block() {
    let set: Vec<serde_json::Value> = serde_json::from_str(S01).expect("the S0.1 fixture parses");
    assert_eq!(set.len(), 10, "PREMISE: the ten test-reachable mutants");
    let survivors: Vec<&str> = set
        .iter()
        .filter(|m| m["recorded"]["survived"] == true)
        .map(|m| m["slug"].as_str().unwrap())
        .collect();
    assert_eq!(
        survivors,
        vec!["issue-75-item-frontmatter-strict-drift"],
        "PREMISE: F11's one survivor"
    );

    let mut items = Vec::new();
    let mut rows = Vec::new();
    for (i, m) in set.iter().enumerate() {
        let ac = format!("S01-{i:02}");
        let patch = m["mutation"]["patch"].as_str().unwrap();
        let suite = m["mutation"]["suite"].as_str().unwrap();
        assert!(
            !patch.is_empty(),
            "{}: the derived patch is non-empty",
            m["slug"]
        );
        items.push(finding(
            &ac,
            EvidenceLabel::MutationSurvived,
            mutate_repro(patch, suite, false),
        ));
        let green = m["recorded"]["survived"] == true;
        // the observation and the outcome describe the SAME run: a killed mutant's suite went red
        rows.push((
            ac,
            mutated(green, true, None),
            mutation_outcome(true, green, true),
        ));
    }
    // the equivalent mutant: a no-op patch, a green suite, no assertion, NOT a reverted fix
    items.push(finding(
        "S01-EQ",
        EvidenceLabel::MutationSurvived,
        mutate_repro("--- a\n+++ b\n", "pnpm test", false),
    ));
    rows.push((
        "S01-EQ".to_owned(),
        survived(false, None),
        mutation_outcome(true, true, false),
    ));
    let findings = Findings { items };
    let exec = Scripted::new(
        rows.iter()
            .map(|(a, o, r)| (a.as_str(), o.clone(), *r))
            .collect(),
    );
    let out = confirm(&findings, ReviewWeight::Lenient, SHA, &exec);

    let survivor_ac = format!(
        "S01-{:02}",
        set.iter()
            .position(|m| m["recorded"]["survived"] == true)
            .unwrap()
    );
    assert_eq!(
        blocks(&out),
        vec![survivor_ac.clone()],
        "the ONE surviving mutant blocks and the nine killed do not"
    );
    let eq = out.records.iter().find(|r| r.ac == "S01-EQ").unwrap();
    assert_ne!(
        eq.computed_class,
        EvidenceClass::MutationSurvived,
        "an equivalent mutant collapses out of mutation-survived"
    );
    assert!(
        matches!(eq.decision, Decision::Advise(_)),
        "…and cannot block: {:?}",
        eq.decision
    );
    for r in out
        .records
        .iter()
        .filter(|r| r.ac != survivor_ac && r.ac != "S01-EQ")
    {
        assert_eq!(
            r.computed_class,
            EvidenceClass::Advisory,
            "{}: a killed mutant is advisory",
            r.ac
        );
    }
    // the suite name and the mutant patch digest are recorded on the block event
    let block_rows: Vec<&Event> = out
        .events
        .iter()
        .filter(|e| e.kind == EventKind::ReviewBlock)
        .collect();
    assert_eq!(block_rows.len(), 1);
    let survivor = &set
        .iter()
        .find(|m| m["recorded"]["survived"] == true)
        .unwrap();
    assert_eq!(
        block_rows[0].payload["mutation_suite"],
        survivor["mutation"]["suite"]
    );
    assert_eq!(
        block_rows[0].payload["mutation_patch_digest"], survivor["patch_sha256"],
        "the patch digest on the row is the fixture's recorded sha256"
    );
}

// ------------------------------------------------------------------ AC-304 / AC-306

fn quarantine(until: i64) -> Quarantine {
    Quarantine {
        reason: Text::new("disputed: baseline already red"),
        by: "ryan".to_owned(),
        until: at(until),
    }
}

/// **AC-304 — a quarantined finding is absent from the block set and still persisted and rendered.**
///
/// RED (observed 2026-09-17): the confirm half knew no quarantine and blocked the finding.
#[test]
fn ac304_quarantined_is_absent_but_persisted() {
    let findings = Findings {
        items: vec![
            finding(
                "AC-401",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac401"),
            ),
            finding(
                "AC-402",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac402"),
            ),
            finding(
                "AC-403",
                EvidenceLabel::Advisory,
                run_repro("cargo test -- ac403"),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        ("AC-401", ran("exit 101"), ReExecution::Flipped),
        ("AC-402", ran("exit 101"), ReExecution::Flipped),
        ("AC-403", ran("exit 0"), ReExecution::DidNotFlip),
    ]);
    // PREMISE: without the quarantine both reproduced findings block
    let before = confirm(&findings, ReviewWeight::Strict, SHA, &exec);
    assert_eq!(blocks(&before), vec!["AC-401", "AC-402"]);

    // quarantine AC-402; the grant is an event carrying AC-307's members plus reason/by/until
    let id_402 = finding_identity(&findings.items[1]);
    let rec_402 = before.records.iter().find(|r| r.ac == "AC-402").unwrap();
    let granted = grant(rec_402, &quarantine(20_000), "ryan")
        .expect("a reproduced finding may be quarantined");
    assert_eq!(granted.kind, EventKind::QuarantineGranted);
    assert_eq!(granted.payload["finding_id"], id_402.as_str());
    assert_eq!(granted.payload["computed_class"], "reproduced");
    assert_eq!(granted.payload["actor"], "ryan");
    assert_eq!(granted.payload["adjudication"], serde_json::Value::Null);
    let mut qs = no_quarantines();
    qs.insert(id_402, quarantine(20_000));

    let out = confirm_with(&input(&findings, ReviewWeight::Strict, &qs, 10_000), &exec);
    assert_eq!(
        blocks(&out),
        vec!["AC-401"],
        "the quarantined finding is ABSENT from the block set"
    );
    let q = out
        .records
        .iter()
        .find(|r| r.ac == "AC-402")
        .expect("…and still PERSISTED");
    assert_eq!(q.decision, Decision::Advise(NonBlockReason::Quarantined));
    assert!(
        q.quarantine
            .as_ref()
            .is_some_and(|q| q.reason.as_str() == "disputed: baseline already red"),
        "…with its reason"
    );
    assert_eq!(
        q.computed_class,
        EvidenceClass::Reproduced,
        "its class is untouched by the quarantine"
    );
    // rendered, not deleted: the golden
    let rendered = render_findings(&out);
    assert!(
        rendered.contains("AC-402")
            && rendered.contains("quarantined")
            && rendered.contains("disputed: baseline already red"),
        "the rendering shows the quarantined finding with its reason:\n{rendered}"
    );
    assert_eq!(
        rendered.replace("\r\n", "\n"),
        AC304_GOLDEN.replace("\r\n", "\n"),
        "the rendering golden"
    );
    // intake still holds: 3 in = 1 block + 2 sink records (the quarantined one is a sink record)
    assert_eq!(count(&out, EventKind::AdvisoryCreated), 2);

    // the negative fixture: a quarantine for an ADVISORY record is refused — disjoint states
    let rec_403 = before.records.iter().find(|r| r.ac == "AC-403").unwrap();
    assert_eq!(rec_403.computed_class, EvidenceClass::Advisory);
    assert!(
        matches!(
            grant(rec_403, &quarantine(20_000), "ryan"),
            Err(QuarantineError::AdvisoryIsDisjoint { .. })
        ),
        "advisory and Quarantined are disjoint (ADR-0082 clause 4)"
    );
}

/// **AC-306 — an expired quarantine re-raises the finding, at `until ± 1s`, at both weights.**
///
/// RED (observed 2026-09-17): the confirm half took no `now` and could not expire anything.
#[test]
fn ac306_quarantine_expiry_re_raises() {
    let findings = Findings {
        items: vec![
            finding(
                "AC-501",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac501"),
            ),
            finding(
                "AC-502",
                EvidenceLabel::NarrowedPin,
                mutate_repro("--- a\n+++ b\n-x\n", "cargo test", true),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        ("AC-501", ran("exit 101"), ReExecution::Flipped),
        ("AC-502", survived(false, Some(false)), ReExecution::Flipped),
    ]);
    let until = 20_000;
    let mut qs = no_quarantines();
    qs.insert(finding_identity(&findings.items[0]), quarantine(until));
    qs.insert(finding_identity(&findings.items[1]), quarantine(until));

    for weight in [ReviewWeight::Lenient, ReviewWeight::Strict] {
        // until − 1s: the quarantine HOLDS
        let held = confirm_with(&input(&findings, weight, &qs, until - 1), &exec);
        assert!(
            held.block_set().is_empty(),
            "{weight:?} at until−1s: nothing blocks"
        );
        assert_eq!(count(&held, EventKind::QuarantineExpired), 0);
        // until + 1s: both are re-raised, each with a quarantine_expired row, and each blocks IFF
        // its class blocks at this weight — exactly as a newly raised finding
        let raised = confirm_with(&input(&findings, weight, &qs, until + 1), &exec);
        assert_eq!(
            count(&raised, EventKind::QuarantineExpired),
            2,
            "{weight:?}: two quarantine_expired rows"
        );
        let expired: Vec<&Event> = raised
            .events
            .iter()
            .filter(|e| e.kind == EventKind::QuarantineExpired)
            .collect();
        assert_eq!(expired[0].payload["until"], until);
        assert_eq!(expired[0].payload["review_id"], "review-1");
        let want: Vec<&str> = match weight {
            ReviewWeight::Lenient => vec!["AC-501"],
            ReviewWeight::Strict => vec!["AC-501", "AC-502"],
            ReviewWeight::ReadOnly => unreachable!(),
        };
        assert_eq!(
            blocks(&raised),
            want,
            "{weight:?}: the re-raised findings block iff their class blocks at this weight"
        );
        assert!(
            raised.records.iter().all(|r| r.quarantine.is_none()),
            "an expired quarantine is not carried as active"
        );
    }
}

// ------------------------------------------------------------------ AC-307 / AC-530 / AC-313

/// **AC-307 — every block has an event, from the same code path.**
///
/// RED (observed 2026-09-17): the confirm half wrote no `review_block` rows — `count(blocks) == 2`,
/// `count(block_events) == 0`.
#[test]
fn ac307_every_block_has_an_event() {
    let findings = Findings {
        items: vec![
            finding(
                "AC-601",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac601"),
            ),
            finding(
                "AC-602",
                EvidenceLabel::NarrowedPin,
                mutate_repro("--- a\n+++ b\n-x\n", "cargo test", true),
            ),
            finding(
                "AC-603",
                EvidenceLabel::Advisory,
                run_repro("cargo test -- ac603"),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        ("AC-601", ran("exit 101"), ReExecution::Flipped),
        ("AC-602", survived(false, Some(false)), ReExecution::Flipped),
        ("AC-603", ran("exit 0"), ReExecution::DidNotFlip),
    ]);
    for weight in [ReviewWeight::Lenient, ReviewWeight::Strict] {
        let out = confirm(&findings, weight, SHA, &exec);
        let n_blocks = out.block_set().len();
        let rows: Vec<&Event> = out
            .events
            .iter()
            .filter(|e| e.kind == EventKind::ReviewBlock)
            .collect();
        assert_eq!(
            rows.len(),
            n_blocks,
            "{weight:?}: count(blocks) == count(block_events)"
        );
        assert!(n_blocks >= 1, "PREMISE: something blocks at {weight:?}");
        for (row, rec) in rows.iter().zip(out.block_set()) {
            // the payload the register names, every member present
            for member in [
                "finding_id",
                "ac",
                "computed_class",
                "model_label",
                "stage_weight",
                "sha",
                "re_execution_id",
                "actor",
                "adjudication",
            ] {
                assert!(
                    row.payload.get(member).is_some(),
                    "{weight:?}: the block row carries `{member}`: {}",
                    row.payload
                );
            }
            assert_eq!(row.payload["ac"], rec.ac);
            assert_eq!(row.payload["finding_id"], rec.finding_id.as_str());
            assert_eq!(row.payload["computed_class"], rec.computed_class.as_str());
            assert_eq!(row.payload["model_label"], rec.evidence_label.as_str());
            assert_eq!(row.payload["stage_weight"], weight.as_str());
            assert_eq!(row.payload["sha"], SHA);
            assert_eq!(
                row.payload["adjudication"],
                serde_json::Value::Null,
                "adjudication is null at write (AC-308 attaches later)"
            );
        }
    }
    // the architecture lint: the block decision and the review_block append are ONE code path
    let files = module_boundary::kernel_sources().expect("src/ is readable");
    let report = block_path_appends_its_own_event_rule(&files).unwrap_or_else(|v| {
        panic!("a second block-append site, or a block path without its append:\n{v:#?}")
    });
    assert_eq!(
        report.append_sites, 1,
        "exactly one site appends review_block, and it is the block path's"
    );
    // POSITIVE CONTROL: a second append site is a violation
    let second = vec![SourceFile { path: "crates/rk-kernel/src/review/sink.rs".to_owned(), text: "fn late(events: &mut Vec<Event>) {\n    events.push(review_block_event(&rec, weight, sha, \"kernel\"));\n}\n".to_owned() }];
    let mut both = files;
    both.extend(second);
    assert!(
        block_path_appends_its_own_event_rule(&both).is_err(),
        "a second append site is reported"
    );
}

/// **AC-530 — a record leaves the sink only by a recorded disposition.**
///
/// RED (observed 2026-09-17): `retire_not_re_raised` did not exist; a record absent from review 2
/// simply vanished from the fold, which is the deletion the invariant forbids.
#[test]
fn ac530_sink_leaves_only_by_recorded_disposition() {
    let review1 = Findings {
        items: vec![
            finding(
                "AC-701",
                EvidenceLabel::NarrowedPin,
                mutate_repro("--- a\n+++ b\n-x\n", "cargo test", true),
            ),
            finding(
                "AC-702",
                EvidenceLabel::Advisory,
                run_repro("cargo test -- ac702"),
            ),
            finding(
                "AC-703",
                EvidenceLabel::Advisory,
                run_repro("cargo test -- ac703"),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        ("AC-701", survived(false, Some(false)), ReExecution::Flipped),
        ("AC-702", ran("exit 0"), ReExecution::DidNotFlip),
        ("AC-703", ran("exit 0"), ReExecution::DidNotFlip),
    ]);
    let r1 = confirm_with(
        &ConfirmInput {
            findings: &review1,
            weight: ReviewWeight::Lenient,
            sha: SHA,
            review_id: "review-1",
            now: at(1),
            quarantines: &no_quarantines(),
        },
        &exec,
    );
    assert_eq!(
        count(&r1, EventKind::AdvisoryCreated),
        3,
        "review 1: three sink records"
    );
    let mut journal: Vec<Event> = r1.events.clone();
    let live_after_1: BTreeSet<Digest> = r1
        .records
        .iter()
        .filter(|r| matches!(r.decision, Decision::Advise(_)))
        .map(|r| r.finding_id.clone())
        .collect();
    assert_eq!(live_after_1.len(), 3);

    // review 2 re-raises AC-701 (same identity) and AC-702; AC-703 is NOT re-raised
    let review2 = Findings {
        items: vec![review1.items[0].clone(), review1.items[1].clone()],
    };
    let r2 = confirm_with(
        &ConfirmInput {
            findings: &review2,
            weight: ReviewWeight::Lenient,
            sha: SHA,
            review_id: "review-2",
            now: at(2),
            quarantines: &no_quarantines(),
        },
        &exec,
    );
    let retired = retire_not_re_raised(&live_after_1, &r2, "review-2");
    assert_eq!(
        retired.len(),
        1,
        "exactly the record not re-raised is retired"
    );
    assert_eq!(retired[0].kind, EventKind::AdvisoryRetired);
    assert_eq!(
        retired[0].payload["finding_id"],
        finding_identity(&review1.items[2]).as_str()
    );
    assert_eq!(retired[0].payload["reason"], "not_re_raised");
    assert_eq!(retired[0].payload["review_id"], "review-2");
    // a re-raised record with the SAME identity is not created again as new (AC-305)
    let created_in_2: Vec<&Event> = r2
        .events
        .iter()
        .filter(|e| e.kind == EventKind::AdvisoryCreated)
        .collect();
    assert_eq!(created_in_2.len(), 2, "review 2 writes its own intake rows");
    for e in &created_in_2 {
        assert!(
            live_after_1
                .iter()
                .any(|id| id.as_str() == e.payload["finding_id"]),
            "a re-raised finding carries the SAME identity as its review-1 record"
        );
    }
    journal.extend(retired.iter().cloned());
    // a human dismissal (AC-311's disposition) is the third way out
    journal.push(Event::new(EventKind::AdvisoryDismissed, serde_json::json!({ "finding_id": finding_identity(&review1.items[1]).as_str(), "by": "ryan" })));
    let counts = sink_accounting(&journal).expect("folds");
    assert_eq!(
        (
            counts.created,
            counts.retired,
            counts.dismissed,
            counts.live
        ),
        (3, 1, 1, 1),
        "live == created − retired − dismissed over a run carrying all three"
    );
}

/// **AC-313 — k=10 replay: identical block sets AND equal to the pinned NON-EMPTY set.**
///
/// The two halves are red separately (`s7b-ii` §7b-ii.8): a gate that blocks nothing scores 10/10
/// on the first. RED (observed 2026-09-17): the non-empty half, against a skeleton whose confirm
/// half blocked nothing.
#[test]
fn ac313_replay_k10() {
    let recorded = Findings {
        items: vec![
            finding(
                "AC-801",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac801"),
            ),
            finding(
                "AC-802",
                EvidenceLabel::Advisory,
                run_repro("cargo test -- ac802"),
            ),
            finding(
                "AC-803",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -- ac803"),
            ),
        ],
    };
    // the recorded re-execution record, held fixed (re-execution determinism is AC-288/289's job)
    let exec = Scripted::new(vec![
        ("AC-801", ran("exit 101"), ReExecution::Flipped),
        ("AC-802", ran("exit 0"), ReExecution::DidNotFlip),
        (
            "AC-803",
            ran("exit 101"),
            ReExecution::Unverifiable(Unverifiable::BothFail),
        ),
    ]);
    let expected_non_empty = vec!["AC-801".to_owned()];
    let mut sets: Vec<Vec<String>> = Vec::new();
    for _ in 0..10 {
        sets.push(blocks(&confirm(
            &recorded,
            ReviewWeight::Lenient,
            SHA,
            &exec,
        )));
    }
    assert!(
        sets.iter().all(|s| *s == sets[0]),
        "identical on all 10 runs: {sets:?}"
    );
    assert_eq!(
        sets[0], expected_non_empty,
        "…AND equal to the pinned expected set"
    );
    assert!(
        !expected_non_empty.is_empty(),
        "the pinned set is NON-EMPTY, or a gate that blocks nothing scores 10/10"
    );
}

// ------------------------------------------------------------------ AC-314 / AC-315 / AC-316

/// **AC-314 — the regression record carries the CORPUS repository's sha; a per-item checkpoint sha is a different field.**
///
/// RED (observed 2026-09-17): `RegressionRecord` did not exist; the measured F37 defect — a
/// per-item checkpoint sha in `corpus_sha` — had nothing to refuse it.
#[test]
fn ac314_corpus_sha_is_the_corpus_repos() {
    let pin = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../CORPUS_PIN"),
    )
    .expect("CORPUS_PIN");
    let corpus_sha = pin
        .lines()
        .find_map(|l| l.strip_prefix("CORPUS_SHA="))
        .expect("CORPUS_SHA=")
        .trim()
        .to_owned();
    assert_eq!(corpus_sha.len(), 40);
    let item_sha = "582a3e9694f8c4d6bae02f8889d806d500178771"; // issue-75's checkpoint, an ITEM sha
    let rec = RegressionRecord::at_pin(&corpus_sha, &corpus_sha, item_sha)
        .expect("the corpus sha at the pin, the item sha beside it");
    assert_eq!(rec.corpus_sha, corpus_sha);
    assert_eq!(rec.item_checkpoint_sha, item_sha);
    let v = serde_json::to_value(&rec).expect("serialises");
    assert!(
        v.get("corpus_sha").is_some() && v.get("item_checkpoint_sha").is_some(),
        "two differently named fields"
    );
    // F37's measured defect: a per-item checkpoint sha stored as the corpus sha
    assert!(
        matches!(
            RegressionRecord::at_pin(&corpus_sha, item_sha, item_sha),
            Err(RegressionRecordError::NotTheCorpusSha { .. })
        ),
        "an item checkpoint sha in corpus_sha is refused"
    );
    assert!(matches!(
        RegressionRecord::at_pin(&corpus_sha, "not-a-sha", item_sha),
        Err(RegressionRecordError::NotTheCorpusSha { .. })
    ));
}

/// **AC-315 — a review can only add findings, never mark an AC verified.**
///
/// RED (observed 2026-09-17): the source rule did not exist.
#[test]
fn ac315_review_never_marks_an_ac_verified() {
    let files = module_boundary::kernel_sources().expect("src/ is readable");
    let report = review_touches_no_ledger_rule(&files)
        .unwrap_or_else(|v| panic!("the review module names a ledger type:\n{v:#?}"));
    assert!(
        report.files_scanned >= 8,
        "the review module's files were walked: {report:?}"
    );
    let scratch = vec![SourceFile {
        path: "crates/rk-kernel/src/review/confirm.rs".to_owned(),
        text: "fn mark(ledger: &mut AcLedger) { ledger.rows[0].status = LedgerStatus::Pinned; }\n"
            .to_owned(),
    }];
    assert!(
        review_touches_no_ledger_rule(&scratch).is_err(),
        "a review that writes a ledger status is reported"
    );
    // a clean review leaves every AC unpinned: the review's output carries no ledger at all
    let findings = Findings {
        items: vec![finding(
            "AC-901",
            EvidenceLabel::Advisory,
            run_repro("cargo test -- ac901"),
        )],
    };
    let out = confirm(
        &findings,
        ReviewWeight::Strict,
        SHA,
        &Scripted::new(vec![("AC-901", ran("exit 0"), ReExecution::DidNotFlip)]),
    );
    assert!(out.block_set().is_empty());
    let as_json = out
        .events
        .iter()
        .map(|e| format!("{}:{}", e.kind.wire_name(), e.payload))
        .collect::<Vec<_>>()
        .join(
            "
",
        );
    assert!(
        !as_json.contains("pinned") && !as_json.contains("ledger"),
        "a clean review's rows say nothing about verification: {as_json}"
    );
}

/// **AC-316 — a green suite is not a dismissal path: the block stands.**
///
/// The signature is the instrument: `confirm` takes findings, a weight, a sha and a re-executor —
/// no suite result — and `Confirmed` has no method that clears a block. RED (observed 2026-09-17):
/// the source half of the rule did not exist.
#[test]
fn ac316_green_suite_does_not_clear_a_block() {
    let _confirm: fn(&Findings, ReviewWeight, &str, &dyn ReExecutor) -> Confirmed = confirm;
    let findings = Findings {
        items: vec![finding(
            "AC-951",
            EvidenceLabel::Reproduced,
            run_repro("cargo test -- ac951"),
        )],
    };
    let exec = Scripted::new(vec![("AC-951", ran("exit 101"), ReExecution::Flipped)]);
    let out = confirm(&findings, ReviewWeight::Lenient, SHA, &exec);
    assert_eq!(blocks(&out), vec!["AC-951"], "the flipped finding blocks…");
    // …and a whole-suite green result has no route in: nothing in the review module reads one
    let files = module_boundary::kernel_sources().expect("src/ is readable");
    let review: Vec<&SourceFile> = files
        .iter()
        .filter(|f| f.path.contains("/review/") || f.path.ends_with("/review.rs"))
        .collect();
    assert!(review.len() >= 8);
    for f in review {
        for (i, line) in f.text.lines().enumerate() {
            let code = line.trim();
            if code.starts_with("//") {
                continue;
            }
            assert!(
                !code.contains("suite_green")
                    || f.path.ends_with("classify.rs")
                    || f.path.ends_with("reexec.rs"),
                "{}:{}: a suite result read outside the classifier/re-execution — a dismissal path: {code}",
                f.path,
                i + 1
            );
            assert!(
                !code.contains("VerifyReport") && !code.contains("whole_suite"),
                "{}:{}: the review module reads a verify result: {code}",
                f.path,
                i + 1
            );
        }
    }
}
