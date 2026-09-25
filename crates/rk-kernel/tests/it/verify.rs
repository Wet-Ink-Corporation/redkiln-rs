//! **The verify gate** — Step 7b-ii, `s7b-ii` §7b-ii.9. The flip predicate, the AC ledger, the
//! stage-exit refusal, the bootstrap comparator, the gate-eligible list and the admissibility
//! policy — every `rk-kernel-verify` id Checkpoint 7b-ii's second loop names, plus the remainder
//! ids §7b-ii.14 says must be named or ARMED before the step is run.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac334_flip_predicate`] | **AC-334** | `pinned` iff the named assertion flips; a green suite under the mutant is `mutation-survived`; a survivor is `mutation-survived` whether or not a quarantine exists (the predicate has no quarantine input); the predicate is exercised over a SCRIPTED run | the harness against a real toolchain — **ARMED to 7c**: `Toolchain::test` returns a count and the fake cannot script a flip, so this is evidence about the predicate, not about a suite (`s7b-ii` §7b-ii.9) |
//! | [`ac015_inversion_regression`] | **AC-334** (F25's fixture) | the recorded ordering inversion — 36/36 green, assertion not flipped — yields `mutation-survived`, never `pinned`; the copied patch's digest is the record's | executing `npx vitest` against the predecessor — the run is the RECORD's (`record.json`), and the fixture says so |
//! | [`ac338_restored_mutation_is_unverifiable`] | **AC-338** (the predicate half) | a run whose mutation was not present at completion is `Unverifiable`, never `mutation-survived` and never `pinned` | the post-run content hash of the mutated file (the harness's; 7c) |
//! | [`ac335_green_suite_does_not_promote_a_survivor`] | **AC-335** | a survivor stays `mutation-survived` with the item's suite green; the status function's signature has no suite-verdict parameter (fn-pointer coercion); the verify module names no suite verdict / CI / review type (`module_boundary` rule, positive control red) | — |
//! | [`ac342_review_findings_are_not_a_ledger_input`] | **AC-342** | the status function does not take `Findings`; a reviewer reporting "all criteria met" over an unpinned AC leaves it unpinned and exit refused | — |
//! | [`ac345_quarantine_changes_no_ledger_status`] | **AC-345** | quarantining a survivor's finding stops the review gate blocking, writes `by`/`until` on the row, leaves the AC `mutation-survived`, and stage exit is still refused for that AC | the `doctor` age report (7d) |
//! | [`ac346_quarantined_survivor_still_refuses_exit`] | **AC-346** | the two-AC fixture: exit refused, the unpinned id named, the refusal is a `stage_exit_refused` row (ADR-0125) written from ONE site; quarantining the survivor's finding changes nothing | AC-357's gate, which follows the refusal (human-gated; excluded) |
//! | [`ac326_bootstrap_comparator`] | **AC-326** | the reclassifier at the boundary (`executed == recorded` reclassifies); the inert `pnpm test -- --run <file>` shape; the empty comparator → `probe-unverified` and the seeded count; a pin bump resets | a partially inert filter (the register's own *Known gap*, carried not smoothed) |
//! | [`ac322_inert_filter_records_whole_suite`] | **AC-322** | a claim whose selection did not narrow records `whole-suite`, never `affected` | the fixture toolchain (7c) |
//! | [`ac332_empty_selection_is_never_green`] | **AC-332** | a non-empty change set selecting zero tests escalates to `whole-suite / selection-empty` or fails, per policy; never `affected` | the glob fixture against a real suite (7c) |
//! | [`ac352_basis_reason_is_exhaustive`] | **AC-352** | both enums matched with no wildcard (a seventh reason is a compile error); a fallback with no reason fails validation; the capture golden records the pair for one run of each variant | — |
//! | [`ac324_absent_basis_is_err`] | **AC-324** | three fixtures (missing basis, digest, argv) each `Err`; no `Default`; no `unwrap_or_default` on the verify path (lint) | — |
//! | [`ac343_model_authored_pass_is_a_schema_violation`] | **AC-343** | a model response carrying `pass: true` fails ingest rather than being ignored; a report carrying it fails too | — |
//! | [`ac344_verify_findings_use_the_review_predicate`] | **AC-344** | the verify gate's own `reproduced` finding blocks iff it fails on the candidate and passes on the reference; both-fail / cannot-run route to the sink; the predicate is `review::reexec`'s — the verify module defines none | the cannot-run fixture against a missing binary (7c) |
//! | [`ac340_survivor_finding_blocks_under_default_weight`] | **AC-340** | the S0.1-shaped survivor emits a `mutation-survived` finding that blocks under the default weight and is still emitted, not blocking, under `ReadOnly`; a green suite does not drop it | — |
//! | [`ac347_verify_dispatches_nothing`] | **AC-347** | the verify path emits zero `dispatch` events over a full fixture run; a verify activity with an agent step fails the lint, one without passes | the derived-vector accuracy (F39, AC-070's) |
//! | [`ac336_toolchain_version_bump_moves_the_verify_memo_key`] | **AC-336** (the unit half) | the verify step's memo key moves on a toolchain-version bump and on a changed suite sha, so evidence at N does not count at N+1 | the integration recompute over a store (Step 8) |
//! | [`ac239_gate_eligible_list_is_data`] | **AC-239** (list-is-data half; PROVISIONAL) | the list is read from a fixture, holds exactly one entry in Phase 1, and a result from any other adapter cannot close a gate; the kernel source names no adapter (AC-487's denylist) | the Codex-present half (Phase 2) *(noted 2026-09-25, s8a §8a.6 fix round 12, ADR-0167: the Codex adapter is Phase 4, after S-CODEX, as ADR-0007 places it; the exclusion stands, its phase reason is corrected)* |
//! | [`ac455_policy_decides_not_the_type`] | **AC-455** | one fixture spec with `verification: None`; the requiring pack refuses the stage naming the AC id, at zero model cost; the permitting pack passes (the must-pass control) | the `doctor` rule (7d; orphaned — RAISED) |
//!
//! Red first. The whole file was written against a kernel with no `verify` module (did not
//! compile), then run against a SKELETON carrying the absent behaviour — a predicate that pinned
//! whenever the patch applied, a status function that promoted a survivor when the suite was
//! green, an exit check that never refused, a comparator that never reclassified, a list that
//! accepted any adapter, an admissibility that always passed — so every test failed on its own
//! claim rather than on a missing symbol: 16 of 19 red. The three green on arrival pin behaviour
//! that already existed and are kept as pins: `ac343` (`deny_unknown_fields` at both ingest
//! boundaries, Step 5 / V1), `ac336` (the memo key's tool branch, Step 7b), `ac340` (the review
//! policy table, R2). AC-239's own red — the adapter name hard-coded at the evaluator — was taken
//! against a reverted mutant and fired both this test and AC-487's denylist. The outputs are in
//! the commit message.

use std::collections::BTreeMap;

use rk_kernel::module_boundary::{
    self, SourceFile, stage_exit_refused_single_site_rule, verify_defines_no_second_predicate_rule,
    verify_path_no_default_rule, verify_reads_no_suite_verdict_rule,
};
use rk_kernel::review::classify::{EvidenceClass, MutateObservation, Observation, RunObservation};
use rk_kernel::review::confirm::{ConfirmInput, confirm, confirm_with};
use rk_kernel::review::policy::{Decision, ReviewWeight};
use rk_kernel::review::quarantine::grant;
use rk_kernel::review::reexec::{
    ReExecution, ReExecutionRecord, ReExecutor, Unverifiable, four_cell,
};
use rk_kernel::rigor::Rigor;
use rk_kernel::select::pack::Pack;
use rk_kernel::verify::admissibility::{Refused, admissible};
use rk_kernel::verify::comparator::{Claim, SelectionPolicy, reclassify};
use rk_kernel::verify::eligible::{EligibleError, GateEligible};
use rk_kernel::verify::exit::stage_exit;
use rk_kernel::verify::flip::{FlipOutcome, FlipUnverifiable, MutationRun, flip_predicate};
use rk_kernel::verify::ledger::{
    HumanAnswer, MutationDeclaration, SpecAc, ac_status, compute_ledger,
};
use rk_kernel::verify::verify_activity_has_no_agent_step;
use rk_kernel::workflow::memo::{CardOrTool, StepContext};
use rk_ports::toolchain::MutationFeasibility;
use rk_store::artifacts::verify_report::{WholeSuiteRecord, latest_whole_suite};
use rk_types::artifact::findings::{EvidenceLabel, Finding, Findings};
use rk_types::artifact::ledger::{AcLedger, AssertionLocator, LedgerStatus};
use rk_types::digest::{Digest, sha256};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::{Expected, Locator, Mutation, Repro, ReproKind};
use rk_types::review::Quarantine;
use rk_types::text::Text;
use rk_types::time::Timestamp;
use rk_types::typed::SchemaId;
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};
use serde_json::{Value, json};

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";
const NODE_PIN: &str = "node 22.4.1 / vitest 2.1.0";

const AC015_RECORD: &str = include_str!("../fixtures/findings/ac015_inversion/record.json");
const AC015_PATCH: &str = include_str!("../fixtures/findings/ac015_inversion/inversion.patch");
const TWO_AC_SPEC: &str = include_str!("../fixtures/ledger/ac346_two_ac_spec.toml");
const AC455_SPEC: &str = include_str!("../fixtures/ledger/ac455_spec.toml");
const AC455_REQUIRES: &str = include_str!("../fixtures/packs/ac455-requires-verification.toml");
const AC455_PERMITS: &str = include_str!("../fixtures/packs/ac455-permits-unverified.toml");
const AC337_INFEASIBLE: &str = include_str!("../fixtures/packs/ac337-mutation-infeasible.toml");
const AC239_ELIGIBLE: &str = include_str!("../fixtures/gates/ac239_gate_eligible_adapters.toml");
const AC352_GOLDEN: &str = include_str!("../fixtures/ledger/ac352_basis_pairs.golden.json");
const AC335_CONTROL: &str =
    include_str!("../fixtures/lint-controls/ac335_status_reads_the_suite_verdict.rs");
const AC347_AGENT_STEP: &str =
    include_str!("../fixtures/activities/ac347_verify_with_agent_step.toml");
const AC347_CLEAN: &str = include_str!("../fixtures/activities/ac347_verify_clean.toml");

// ------------------------------------------------------------------ helpers

fn run(
    applied: bool,
    suite_green: bool,
    assertion_flipped: Option<bool>,
    still_present: bool,
) -> MutationRun {
    MutationRun {
        applied,
        suite_green,
        assertion_flipped,
        still_present,
    }
}

fn locator(test: &str) -> AssertionLocator {
    AssertionLocator {
        file: "test/implement-contract.test.ts".to_owned(),
        test: test.to_owned(),
        assertion: "expect(order).toEqual(expected)".to_owned(),
    }
}

fn declared(patch: &str) -> MutationDeclaration {
    MutationDeclaration::Declared(Mutation {
        patch: patch.to_owned(),
        suite: "npx vitest run".to_owned(),
    })
}

fn spec_ac(id: &str, mutation: MutationDeclaration, assertion: Option<AssertionLocator>) -> SpecAc {
    SpecAc {
        id: id.to_owned(),
        verification: assertion
            .as_ref()
            .map(|l| format!("{}::{}", l.file, l.test)),
        mutation,
        assertion,
    }
}

fn at(secs: i64) -> Timestamp {
    Timestamp::from_unix_seconds(secs)
}

fn feasible() -> MutationFeasibility {
    MutationFeasibility::Feasible
}

fn mutate_finding(ac: &str, patch: &str) -> Finding {
    Finding {
        ac: ac.to_owned(),
        claim: Text::new(format!("{ac}: the mutant survived")),
        evidence: EvidenceLabel::MutationSurvived,
        rationale: Text::new("the suite stayed green under a reverted fix"),
        repro: Repro {
            kind: ReproKind::Mutate,
            command: "npx vitest run".to_owned(),
            expected: Expected::Assertion {
                assertion: Locator {
                    file: "test/x.test.ts".to_owned(),
                    test: "pins it".to_owned(),
                },
            },
            observed: "suite green".to_owned(),
            expected_when_met: "suite red".to_owned(),
            mutation: Some(Mutation {
                patch: patch.to_owned(),
                suite: "npx vitest run".to_owned(),
            }),
        },
        files: vec![],
    }
}

fn run_finding(ac: &str, command: &str) -> Finding {
    Finding {
        ac: ac.to_owned(),
        claim: Text::new(format!("{ac}: reproduced")),
        evidence: EvidenceLabel::Reproduced,
        rationale: Text::new("the command fails on the candidate"),
        repro: Repro {
            kind: ReproKind::Run,
            command: command.to_owned(),
            expected: Expected::Exit { exit: 0 },
            observed: "exit 101".to_owned(),
            expected_when_met: "exit 0".to_owned(),
            mutation: None,
        },
        files: vec![],
    }
}

/// Scripted re-executions keyed by AC.
struct Scripted(BTreeMap<String, (Observation, ReExecution)>);

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

fn survived_observation() -> Observation {
    Observation::Mutate(MutateObservation {
        applied: true,
        suite_green: true,
        reverted_fix: true,
        assertion_flipped: None,
    })
}

fn blocks(out: &rk_kernel::review::confirm::Confirmed) -> Vec<String> {
    out.block_set().iter().map(|r| r.ac.clone()).collect()
}

fn refused_ids(err: &rk_kernel::verify::exit::ExitRefused) -> Vec<String> {
    err.unpinned.clone()
}

fn count(events: &[Event], kind: EventKind) -> usize {
    events.iter().filter(|e| e.kind == kind).count()
}

fn control(path: &str, text: &str) -> Vec<SourceFile> {
    vec![SourceFile {
        path: path.to_owned(),
        text: text.to_owned(),
    }]
}

fn report(
    basis: Basis,
    reason: Option<BasisReason>,
    executed: u32,
    total: u32,
    source: TotalTestCountSource,
) -> VerifyReport {
    VerifyReport {
        basis,
        argv: vec!["pnpm".to_owned(), "test".to_owned()],
        selected_test_count: executed,
        total_test_count: total,
        total_test_count_source: source,
        wall_secs: 1,
        toolchain_version: NODE_PIN.to_owned(),
        suite_sha: SHA.to_owned(),
        basis_reason: reason,
    }
}

// ------------------------------------------------------------------ AC-334 / AC-015 / AC-338

/// **AC-334 — `pinned` only if the named assertion flips; a green suite is `mutation-survived`.**
///
/// ARMED against the Toolchain fake: `FixtureToolchain::test` returns the selection's length and
/// cannot script a flip, so the run here is a scripted [`MutationRun`] — evidence about the
/// predicate, not about a suite (`s7b-ii` §7b-ii.9, §7b-ii.14).
#[test]
fn ac334_flip_predicate() {
    // the table
    assert_eq!(
        flip_predicate(&run(true, false, Some(true), true)),
        FlipOutcome::Pinned,
        "the named assertion flipped pass → fail"
    );
    assert_eq!(
        flip_predicate(&run(true, true, Some(false), true)),
        FlipOutcome::MutationSurvived,
        "the suite stayed green: survived"
    );
    assert_eq!(
        flip_predicate(&run(true, true, None, true)),
        FlipOutcome::MutationSurvived,
        "green with no observed assertion is still a survivor"
    );
    assert_eq!(
        flip_predicate(&run(true, false, Some(false), true)),
        FlipOutcome::Unverifiable(FlipUnverifiable::RedElsewhere),
        "the suite went red but NOT at the named assertion: not pinned, not survived"
    );
    assert_eq!(
        flip_predicate(&run(false, true, None, true)),
        FlipOutcome::Unverifiable(FlipUnverifiable::PatchDidNotApply)
    );
    // a survivor is `mutation-survived` and blocks exit, quarantined or not: the predicate and the
    // status function have no quarantine input at all (the signature is the assertion)
    let _predicate: fn(&MutationRun) -> FlipOutcome = flip_predicate;
    let ac = spec_ac(
        "AC-1",
        declared("--- a\n+++ b\n-fix\n"),
        Some(locator("pins it")),
    );
    let status = ac_status(&ac, Some(&FlipOutcome::MutationSurvived), &feasible(), None);
    assert_eq!(status, LedgerStatus::MutationSurvived);
    let ledger = AcLedger {
        rows: vec![rk_types::artifact::ledger::AcLedgerRow {
            ac: "AC-1".to_owned(),
            status,
            evidence: ac.assertion.clone(),
        }],
    };
    assert!(
        stage_exit("item-1", "implement", &ledger).is_err(),
        "a survivor blocks exit"
    );
    // pinned requires the declared assertion locator — a flip with no named assertion pins nothing
    let no_locator = spec_ac("AC-2", declared("--- a\n+++ b\n-fix\n"), None);
    assert_eq!(
        ac_status(&no_locator, Some(&FlipOutcome::Pinned), &feasible(), None),
        LedgerStatus::Unpinned
    );
}

/// **AC-334's F25 regression — the recorded ordering inversion yields `mutation-survived`, never `pinned`.**
#[test]
fn ac015_inversion_regression() {
    let record: Value = serde_json::from_str(AC015_RECORD).expect("the record parses");
    // the copied patch is the recorded one
    let digest = sha256(AC015_PATCH.replace("\r\n", "\n").as_bytes());
    assert_eq!(
        digest.as_str(),
        record["patch_sha256"].as_str().unwrap(),
        "the patch on disk is the one the record digests"
    );
    assert!(
        AC015_PATCH.contains("as the FIRST thing you do"),
        "the inversion the fixture forbids is the patch's + side"
    );
    // the recorded run: 36/36 green, the assertion did not flip
    let recorded = &record["recorded"];
    assert_eq!(recorded["suite_green_under_mutant"], true);
    assert_eq!(recorded["assertion_flipped"], false);
    assert_eq!(recorded["cases_in_file"], 36);
    let run = MutationRun {
        applied: true,
        suite_green: recorded["suite_green_under_mutant"].as_bool().unwrap(),
        assertion_flipped: Some(recorded["assertion_flipped"].as_bool().unwrap()),
        still_present: true,
    };
    let outcome = flip_predicate(&run);
    assert_eq!(
        outcome,
        FlipOutcome::MutationSurvived,
        "F25: invert the ordering → 36/36 GREEN is a survivor"
    );
    assert_ne!(outcome, FlipOutcome::Pinned);
    // through the ledger, with the assertion the record names
    let test_name = record["expected"]["assertion"]["test"].as_str().unwrap();
    let ac = spec_ac("AC-015", declared(AC015_PATCH), Some(locator(test_name)));
    assert_eq!(
        ac_status(&ac, Some(&outcome), &feasible(), None),
        LedgerStatus::MutationSurvived
    );
}

/// **AC-338 (predicate half) — a mutation not present at completion is `Unverifiable`.**
#[test]
fn ac338_restored_mutation_is_unverifiable() {
    // F17's shape: the mutation was applied, the suite ran green — and the region had been restored
    let restored = run(true, true, None, false);
    assert_eq!(
        flip_predicate(&restored),
        FlipOutcome::Unverifiable(FlipUnverifiable::NotPresentAtCompletion)
    );
    let restored_red = run(true, false, Some(true), false);
    assert_eq!(
        flip_predicate(&restored_red),
        FlipOutcome::Unverifiable(FlipUnverifiable::NotPresentAtCompletion),
        "even a flip does not pin when the mutant was gone"
    );
    let ac = spec_ac(
        "AC-F17",
        declared("--- a\n+++ b\n-fix\n"),
        Some(locator("pins it")),
    );
    assert_eq!(
        ac_status(&ac, Some(&flip_predicate(&restored)), &feasible(), None),
        LedgerStatus::Unpinned,
        "unverifiable is no evidence"
    );
}

// ------------------------------------------------------------------ AC-335 / AC-342

/// **AC-335 — the item's green suite changes nothing about a survivor.**
#[test]
fn ac335_green_suite_does_not_promote_a_survivor() {
    let ac = spec_ac(
        "AC-1",
        declared("--- a\n+++ b\n-fix\n"),
        Some(locator("pins it")),
    );
    // the static check: the status function's signature has NO suite-verdict parameter — adding one
    // stops this line compiling (AC-283's pattern)
    let _status: fn(
        &SpecAc,
        Option<&FlipOutcome>,
        &MutationFeasibility,
        Option<&HumanAnswer>,
    ) -> LedgerStatus = ac_status;
    // the fixture: a green suite and a surviving mutation — the suite verdict is not even an input
    let status = ac_status(&ac, Some(&FlipOutcome::MutationSurvived), &feasible(), None);
    assert_eq!(
        status,
        LedgerStatus::MutationSurvived,
        "remains mutation-survived, never pinned"
    );
    // `unpinned` is reserved for NO evidence — a survivor has evidence of the wrong kind
    assert_ne!(status, LedgerStatus::Unpinned);
    // the architecture lint: the verify module names no suite verdict, CI status, tracker or review type
    let files = module_boundary::kernel_sources().expect("src/ is readable");
    let report = verify_reads_no_suite_verdict_rule(&files)
        .unwrap_or_else(|v| panic!("the verify module reads a forbidden input:\n{v:#?}"));
    assert!(
        report.files_scanned >= 6,
        "the verify module's files were walked: {report:?}"
    );
    assert!(
        report.code_lines_scanned > 100,
        "and it read code lines: {report:?}"
    );
    // POSITIVE CONTROL: a status function taking the suite verdict is reported
    let hit = verify_reads_no_suite_verdict_rule(&control(
        "crates/rk-kernel/src/verify/ledger.rs",
        AC335_CONTROL,
    ));
    assert!(
        hit.is_err(),
        "the control reads `GreenAt` and must be reported"
    );
}

/// **AC-342 — a review finding can add blocking state, never move an AC to `pinned`.**
#[test]
fn ac342_review_findings_are_not_a_ledger_input() {
    // type-level: the status function does not accept `Findings` (the coercion above is the pin;
    // here the lint's ban on the type name in the verify module is the second instrument)
    let files = module_boundary::kernel_sources().unwrap();
    assert!(verify_reads_no_suite_verdict_rule(&files).is_ok());
    let hit = verify_reads_no_suite_verdict_rule(&control(
        "crates/rk-kernel/src/verify/ledger.rs",
        "fn status(ac: &SpecAc, review: &Findings) -> LedgerStatus { LedgerStatus::Pinned }\n",
    ));
    assert!(
        hit.is_err(),
        "a status function taking Findings is reported"
    );
    // the fixture: the reviewer reports "all criteria met" over an unpinned AC — it still blocks exit
    let ac = spec_ac("AC-7", MutationDeclaration::None, Some(locator("pins it")));
    let status = ac_status(&ac, None, &feasible(), None);
    assert_eq!(status, LedgerStatus::Unpinned);
    let all_met = Findings { items: vec![] }; // an approving reviewer raises nothing
    assert!(all_met.items.is_empty());
    let ledger = compute_ledger(&[ac], &BTreeMap::new(), &feasible(), &[]);
    let refused = stage_exit("item", "implement", &ledger)
        .expect_err("an unpinned AC refuses exit whatever the reviewer said");
    assert_eq!(refused_ids(&refused), vec!["AC-7".to_owned()]);
}

// ------------------------------------------------------------------ AC-345 / AC-346

fn two_ac_fixture() -> (Vec<SpecAc>, BTreeMap<String, FlipOutcome>) {
    let spec = SpecAc::from_toml("ac346_two_ac_spec", TWO_AC_SPEC).expect("the two-AC spec parses");
    assert_eq!(spec.len(), 2, "PREMISE: two ACs");
    let mut runs = BTreeMap::new();
    runs.insert(spec[0].id.clone(), FlipOutcome::Pinned);
    runs.insert(spec[1].id.clone(), FlipOutcome::MutationSurvived);
    (spec, runs)
}

/// **AC-345 — a quarantine is a review-gate state: it stops the finding blocking review and does
/// not change the AC's ledger status; stage exit is still refused for that AC.**
#[test]
fn ac345_quarantine_changes_no_ledger_status() {
    let (spec, runs) = two_ac_fixture();
    let survivor = spec[1].id.clone();
    let before = compute_ledger(&spec, &runs, &feasible(), &[]);
    assert_eq!(before.rows[1].status, LedgerStatus::MutationSurvived);

    // the survivor's finding, blocking review
    let findings = Findings {
        items: vec![mutate_finding(&survivor, "--- a\n+++ b\n-fix\n")],
    };
    let exec = Scripted(
        [(
            survivor.clone(),
            (survived_observation(), ReExecution::Flipped),
        )]
        .into_iter()
        .collect(),
    );
    let blocking = confirm(&findings, ReviewWeight::Lenient, SHA, &exec);
    assert_eq!(
        blocks(&blocking),
        vec![survivor.clone()],
        "PREMISE: the survivor's finding blocks review"
    );

    // quarantine it: the row carries who and until when
    let record = blocking.records.iter().find(|r| r.ac == survivor).unwrap();
    let q = Quarantine {
        reason: Text::new("disputed: the fix is in a later slice"),
        by: "ryan".to_owned(),
        until: at(20_000),
    };
    let granted = grant(record, &q, "ryan").expect("a survivor's finding may be quarantined");
    assert_eq!(granted.kind, EventKind::QuarantineGranted);
    assert_eq!(granted.payload["by"], "ryan");
    assert_eq!(granted.payload["until"], 20_000);
    let mut qs: BTreeMap<Digest, Quarantine> = BTreeMap::new();
    qs.insert(record.finding_id.clone(), q);
    let quarantined = confirm_with(
        &ConfirmInput {
            findings: &findings,
            weight: ReviewWeight::Lenient,
            sha: SHA,
            review_id: "r2",
            now: at(10_000),
            quarantines: &qs,
        },
        &exec,
    );
    assert!(
        blocks(&quarantined).is_empty(),
        "the review gate no longer blocks on it"
    );

    // the ledger is untouched: the AC remains `mutation-survived` — the ledger has no quarantine input
    let after = compute_ledger(&spec, &runs, &feasible(), &[]);
    assert_eq!(after, before, "quarantine changes no ledger status");
    assert_eq!(after.rows[1].status, LedgerStatus::MutationSurvived);
    // and stage exit is STILL refused for that AC (AC-346; ADR-0104 decision 4)
    let refused = stage_exit("item", "implement", &after).expect_err("exit refused");
    assert_eq!(refused_ids(&refused), vec![survivor]);
}

/// **AC-346 — exit refused, the unpinned ids named, the refusal is an event (ADR-0125).**
#[test]
fn ac346_quarantined_survivor_still_refuses_exit() {
    let (spec, runs) = two_ac_fixture();
    let ledger = compute_ledger(&spec, &runs, &feasible(), &[]);
    assert_eq!(
        ledger.rows[0].status,
        LedgerStatus::Pinned,
        "PREMISE: one AC is pinned"
    );
    let refused =
        stage_exit("item-46", "implement", &ledger).expect_err("one unpinned AC refuses the exit");
    assert_eq!(
        refused_ids(&refused),
        vec![spec[1].id.clone()],
        "the unpinned AC id is named"
    );
    // the refusal is an EVENT, not only a console line, and it names the id
    assert_eq!(refused.event.kind, EventKind::StageExitRefused);
    assert_eq!(
        refused.event.payload["unpinned"],
        json!([spec[1].id.clone()])
    );
    assert_eq!(refused.event.payload["item"], "item-46");
    assert_eq!(refused.event.payload["stage"], "implement");
    assert_eq!(
        refused.event.payload["statuses"][&spec[1].id],
        "mutation_survived"
    );
    assert_eq!(refused.event.payload["statuses"][&spec[0].id], "pinned");
    // the row is written from ONE site (ADR-0125 decision 3)
    let files = module_boundary::kernel_sources().unwrap();
    let sites = stage_exit_refused_single_site_rule(&files).unwrap_or_else(|v| panic!("{v:#?}"));
    assert_eq!(sites.construction_sites, 1);
    // a human answer for THAT AC lifts it; a human answer for the OTHER AC does not
    let answer_other = HumanAnswer {
        ac: spec[0].id.clone(),
        by: "ryan".to_owned(),
        at: at(1),
        reason: Text::new("meaning judged"),
    };
    let still = compute_ledger(&spec, &runs, &feasible(), &[answer_other]);
    assert!(
        stage_exit("item-46", "implement", &still).is_err(),
        "an answer for a different AC is not an answer"
    );
    let answer = HumanAnswer {
        ac: spec[1].id.clone(),
        by: "ryan".to_owned(),
        at: at(2),
        reason: Text::new("the assertion discriminates by inspection"),
    };
    let lifted = compute_ledger(&spec, &runs, &feasible(), &[answer]);
    assert!(matches!(
        lifted.rows[1].status,
        LedgerStatus::HumanAnswered { .. }
    ));
    stage_exit("item-46", "implement", &lifted)
        .expect("every AC pinned or answered: exit permitted");
    // both pinned: permitted, and no row is produced
    let mut all_pinned = runs.clone();
    all_pinned.insert(spec[1].id.clone(), FlipOutcome::Pinned);
    stage_exit(
        "item-46",
        "implement",
        &compute_ledger(&spec, &all_pinned, &feasible(), &[]),
    )
    .expect("permitted");
}

// ------------------------------------------------------------------ AC-326 / AC-322 / AC-332 / AC-352

fn affected_claim(executed: u32, argv: &[&str]) -> Claim {
    Claim {
        run_id: "run-9".to_owned(),
        basis: Basis::Affected,
        reason: None,
        executed,
        changed_files: 3,
        toolchain: "node".to_owned(),
        toolchain_version: NODE_PIN.to_owned(),
        argv: argv.iter().map(|s| (*s).to_owned()).collect(),
        policy: SelectionPolicy::EscalateToWholeSuite,
    }
}

fn recorded(count: u32) -> WholeSuiteRecord {
    WholeSuiteRecord {
        toolchain_version: NODE_PIN.to_owned(),
        count,
        run_id: "run-1".to_owned(),
    }
}

/// **AC-326 — the bootstrap comparator: the boundary, the inert shape, the seed, the pin bump.**
#[test]
fn ac326_bootstrap_comparator() {
    // 1. the reclassifier over (selected, total) pairs INCLUDING the boundary
    let rec = recorded(236);
    // (executed == 0 over a non-empty change set is AC-332's case, tested there, not the boundary's)
    for (executed, reclassified) in [
        (1u32, false),
        (2, false),
        (235, false),
        (236, true),
        (237, true),
        (1_000, true),
    ] {
        let out = reclassify(&affected_claim(executed, &["pnpm", "test"]), Some(&rec));
        assert_eq!(
            out.basis == Basis::WholeSuite,
            reclassified,
            "executed {executed} against recorded 236"
        );
        if reclassified {
            assert_eq!(out.reason, Some(BasisReason::SelectionIneffective));
            assert_eq!(count(&out.events, EventKind::SelectionIneffective), 1);
            let e = &out.events[0];
            assert_eq!(e.payload["reason"], "selection-ineffective");
            assert_eq!(e.payload["executed"], executed);
            assert_eq!(e.payload["recorded_whole_suite"], 236);
            assert_eq!(e.payload["toolchain"], "node");
        } else {
            assert_eq!(out.reason, None);
            assert!(
                out.events.is_empty(),
                "an effective selection emits nothing"
            );
        }
        assert!(out.seed.is_none(), "a run against a record seeds nothing");
    }
    // 2. the historical `pnpm test -- --run <file>` shape: the filter did not survive forwarding, so
    //    the whole suite ran under an `affected` claim — it must reclassify
    let inert = reclassify(
        &affected_claim(236, &["pnpm", "test", "--", "--run", "test/x.test.ts"]),
        Some(&rec),
    );
    assert_eq!(
        (inert.basis, inert.reason),
        (Basis::WholeSuite, Some(BasisReason::SelectionIneffective)),
        "the 26-of-68 shape"
    );
    // 3. the bootstrap: an empty comparator → probe-unverified, and the executed count seeds it
    let boot = reclassify(&affected_claim(241, &["pnpm", "test"]), None);
    assert_eq!(
        (boot.basis, boot.reason),
        (Basis::WholeSuite, Some(BasisReason::ProbeUnverified))
    );
    assert_eq!(
        boot.seed,
        Some(WholeSuiteRecord {
            toolchain_version: NODE_PIN.to_owned(),
            count: 241,
            run_id: "run-9".to_owned()
        }),
        "the executed count seeds the comparator"
    );
    assert_eq!(count(&boot.events, EventKind::SelectionIneffective), 1);
    assert_eq!(boot.events[0].payload["reason"], "probe-unverified");
    assert!(boot.events[0].payload["recorded_whole_suite"].is_null());
    // 4. a pin bump resets: the record is keyed by toolchain version, so the bumped pin has none
    let runs = [(
        "run-1",
        report(
            Basis::WholeSuite,
            Some(BasisReason::ProbeUnverified),
            236,
            236,
            TotalTestCountSource::OwnExecutedCount,
        ),
    )];
    assert!(latest_whole_suite(runs.iter().map(|(i, r)| (*i, r)), NODE_PIN).is_some());
    let bumped = latest_whole_suite(
        runs.iter().map(|(i, r)| (*i, r)),
        "node 22.5.0 / vitest 2.1.0",
    );
    assert!(bumped.is_none(), "a pin bump resets the comparator");
    let mut claim = affected_claim(240, &["pnpm", "test"]);
    claim.toolchain_version = "node 22.5.0 / vitest 2.1.0".to_owned();
    let after_bump = reclassify(&claim, bumped.as_ref());
    assert_eq!(
        after_bump.reason,
        Some(BasisReason::ProbeUnverified),
        "after the bump the first run bootstraps again"
    );
    // a whole-suite claim passes through unchanged and emits nothing
    let mut ws = affected_claim(236, &["pnpm", "test"]);
    ws.basis = Basis::WholeSuite;
    ws.reason = Some(BasisReason::PolicyForced);
    let out = reclassify(&ws, Some(&rec));
    assert_eq!(
        (out.basis, out.reason),
        (Basis::WholeSuite, Some(BasisReason::PolicyForced))
    );
    assert!(out.events.is_empty());
}

/// **AC-322 — a selection that did not narrow records `whole-suite`, never `affected`.**
#[test]
fn ac322_inert_filter_records_whole_suite() {
    let rec = recorded(68);
    let out = reclassify(
        &affected_claim(68, &["dotnet", "test", "--filter", "FullyQualifiedName~X"]),
        Some(&rec),
    );
    assert_eq!(
        out.basis,
        Basis::WholeSuite,
        "recorded basis is whole-suite for an inert filter"
    );
    assert_ne!(out.basis, Basis::Affected);
    let narrowed = reclassify(
        &affected_claim(26, &["dotnet", "test", "--filter", "FullyQualifiedName~X"]),
        Some(&rec),
    );
    assert_eq!(narrowed.basis, Basis::Affected, "26 of 68 DID narrow");
}

/// **AC-332 — a non-empty change set selecting zero tests is never green.**
#[test]
fn ac332_empty_selection_is_never_green() {
    let rec = recorded(236);
    let mut escalate = affected_claim(0, &["pnpm", "test", "--", "--run", "no/such/glob"]);
    escalate.policy = SelectionPolicy::EscalateToWholeSuite;
    let out = reclassify(&escalate, Some(&rec));
    assert_eq!(
        (out.basis, out.reason),
        (Basis::WholeSuite, Some(BasisReason::SelectionEmpty)),
        "escalates with reason selection-empty"
    );
    let mut fail = escalate.clone();
    fail.policy = SelectionPolicy::Fail;
    let out = reclassify(&fail, Some(&rec));
    assert!(out.failed, "under the failing policy the run does not pass");
    assert_ne!(
        out.basis,
        Basis::Affected,
        "zero tests executed is never rendered as an affected pass"
    );
    // zero selected with NO changed files is a different question (nothing to verify) and is not this rule
    let mut nothing_changed = escalate.clone();
    nothing_changed.changed_files = 0;
    let out = reclassify(&nothing_changed, Some(&rec));
    assert_ne!(out.reason, Some(BasisReason::SelectionEmpty));
}

/// **AC-352 — both enums closed and matched without a wildcard; a fallback with no reason fails;
/// the capture golden.**
#[test]
fn ac352_basis_reason_is_exhaustive() {
    // exhaustive match: `describe` matches every reason with no wildcard arm — a seventh variant is
    // E0004 there; here we pin that every member has a description and they differ
    let described: Vec<&str> = BasisReason::ALL
        .iter()
        .map(|r| rk_kernel::verify::comparator::describe(*r))
        .collect();
    assert_eq!(described.len(), 6);
    for (i, d) in described.iter().enumerate() {
        assert!(
            !d.is_empty() && described.iter().filter(|x| *x == d).count() == 1,
            "reason {i} described once"
        );
    }
    // a fallback with no reason fails validation
    let no_reason = report(
        Basis::WholeSuite,
        None,
        10,
        10,
        TotalTestCountSource::OwnExecutedCount,
    );
    assert!(no_reason.validate_shape().is_err());
    // the capture golden: one run per basis variant, the (basis, reason) pair recorded
    let rec = recorded(100);
    let captured = json!({
        "affected":               { "basis": "affected", "reason": null },
        "selection-ineffective":  reclassify(&affected_claim(100, &["t"]), Some(&rec)).pair(),
        "probe-unverified":       reclassify(&affected_claim(50, &["t"]), None).pair(),
        "selection-empty":        reclassify(&affected_claim(0, &["t"]), Some(&rec)).pair(),
    });
    let golden: Value = serde_json::from_str(AC352_GOLDEN).unwrap();
    assert_eq!(
        captured, golden,
        "a kernel change that moves a run from one variant to another fails here"
    );
}

// ------------------------------------------------------------------ AC-324 / AC-343 / AC-344 / AC-340

/// **AC-324 — an absent basis, digest or command record is `Err`, never a pass.**
#[test]
fn ac324_absent_basis_is_err() {
    let base = json!({
        "basis": "whole-suite", "argv": ["pnpm", "test"], "selected_test_count": 3, "total_test_count": 3,
        "total_test_count_source": { "kind": "own_executed_count" }, "wall_secs": 1,
        "toolchain_version": NODE_PIN, "suite_sha": SHA, "basis_reason": "policy-forced"
    });
    for missing in ["basis", "suite_sha", "argv"] {
        let mut v = base.clone();
        v.as_object_mut().unwrap().remove(missing);
        let parsed = rk_kernel::verify::report_at_boundary(v);
        assert!(
            parsed.is_err(),
            "missing `{missing}` resolves to Err, never to a pass"
        );
    }
    // an empty digest or command record parses but fails the boundary check
    let mut empty = base.clone();
    empty["argv"] = json!([]);
    assert!(rk_kernel::verify::report_at_boundary(empty).is_err());
    let mut empty = base.clone();
    empty["suite_sha"] = json!("");
    assert!(rk_kernel::verify::report_at_boundary(empty).is_err());
    rk_kernel::verify::report_at_boundary(base).expect("the control passes the boundary");
    // the lint: no `unwrap_or_default` / `Default` on the verify path
    let files = module_boundary::kernel_sources().unwrap();
    verify_path_no_default_rule(&files).unwrap_or_else(|v| panic!("{v:#?}"));
    assert!(
        verify_path_no_default_rule(&control(
            "crates/rk-kernel/src/verify/exit.rs",
            "let r = report.unwrap_or_default();\n"
        ))
        .is_err()
    );
}

/// **AC-343 — a model-authored `pass` is a schema violation, not an ignored field.**
#[test]
fn ac343_model_authored_pass_is_a_schema_violation() {
    let mut card = json!({ "items": [] });
    card["pass"] = json!(true);
    assert!(
        Findings::ingest(card).is_err(),
        "a findings card carrying `pass` fails ingest"
    );
    let mut item = json!({ "items": [{
        "ac": "AC-1", "claim": "c", "evidence": "advisory", "rationale": "r", "files": [],
        "repro": { "kind": "run", "command": "cargo test", "mutation": null, "expected": { "kind": "exit", "exit": 0 }, "observed": "exit 1", "expected_when_met": "exit 0" },
        "verdict": "approved"
    }]});
    assert!(
        Findings::ingest(item.clone()).is_err(),
        "a `verdict` on a finding fails ingest"
    );
    item["items"][0].as_object_mut().unwrap().remove("verdict");
    Findings::ingest(item).expect("the control ingests");
    let mut rep = json!({
        "basis": "whole-suite", "argv": ["x"], "selected_test_count": 1, "total_test_count": 1,
        "total_test_count_source": { "kind": "own_executed_count" }, "wall_secs": 1,
        "toolchain_version": NODE_PIN, "suite_sha": SHA, "basis_reason": "policy-forced", "approved": true
    });
    assert!(
        rk_kernel::verify::report_at_boundary(rep.clone()).is_err(),
        "a report carrying `approved` is refused"
    );
    rep.as_object_mut().unwrap().remove("approved");
    rk_kernel::verify::report_at_boundary(rep).unwrap();
}

/// **AC-344 — the verify gate's own `reproduced` finding uses AC-288's predicate; unverifiable
/// routes to the sink; the verify module defines no second predicate.**
#[test]
fn ac344_verify_findings_use_the_review_predicate() {
    assert_eq!(four_cell(true, false), ReExecution::Flipped);
    assert_eq!(
        four_cell(true, true),
        ReExecution::Unverifiable(Unverifiable::BothFail)
    );
    assert_eq!(four_cell(false, false), ReExecution::DidNotFlip);
    assert_eq!(four_cell(false, true), ReExecution::DidNotFlip);
    // the verify gate's finding, through the confirm half: both-fail and cannot-run land in the sink
    let findings = Findings {
        items: vec![
            run_finding("AC-V1", "cargo test -- v1"),
            run_finding("AC-V2", "cargo test -- v2"),
            run_finding("AC-V3", "no-such-binary --check"),
        ],
    };
    let ran = |o: &str| {
        Observation::Run(RunObservation {
            ran: true,
            observed: o.to_owned(),
            only_assertion_flipped: false,
        })
    };
    let exec = Scripted(
        [
            (
                "AC-V1".to_owned(),
                (ran("exit 101"), four_cell(true, false)),
            ),
            ("AC-V2".to_owned(), (ran("exit 101"), four_cell(true, true))),
            (
                "AC-V3".to_owned(),
                (
                    Observation::Run(RunObservation {
                        ran: false,
                        observed: String::new(),
                        only_assertion_flipped: false,
                    }),
                    ReExecution::Unverifiable(Unverifiable::CannotRun),
                ),
            ),
        ]
        .into_iter()
        .collect(),
    );
    let out = confirm(&findings, ReviewWeight::Strict, SHA, &exec);
    assert_eq!(
        blocks(&out),
        vec!["AC-V1".to_owned()],
        "blocks only if it fails on the candidate and passes on the reference"
    );
    assert_eq!(
        count(&out.events, EventKind::AdvisoryCreated),
        2,
        "both-fail and cannot-run are durably routed to the sink"
    );
    assert!(
        out.records
            .iter()
            .all(|r| !matches!(r.decision, Decision::Block(_)) || r.ac == "AC-V1")
    );
    // architecture: the predicate is `review::reexec`'s — no `fn four_cell` / `fn mutation_outcome` in verify
    let files = module_boundary::kernel_sources().unwrap();
    verify_defines_no_second_predicate_rule(&files).unwrap_or_else(|v| panic!("{v:#?}"));
    assert!(
        verify_defines_no_second_predicate_rule(&control(
            "crates/rk-kernel/src/verify/flip.rs",
            "pub const fn four_cell(a: bool, b: bool) -> bool { a && !b }\n"
        ))
        .is_err()
    );
}

/// **AC-340 — the survivor's finding is emitted and blocks under the default weight; under a
/// relaxed weight it is still emitted, not blocking; a green suite never drops it.**
#[test]
fn ac340_survivor_finding_blocks_under_default_weight() {
    let findings = Findings {
        items: vec![mutate_finding("S01-04", "--- a\n+++ b\n-fix\n")],
    };
    let exec = Scripted(
        [(
            "S01-04".to_owned(),
            (survived_observation(), ReExecution::Flipped),
        )]
        .into_iter()
        .collect(),
    );
    let default_weight = rk_kernel::review::policy::resolve_review_tier(None);
    let out = confirm(&findings, default_weight, SHA, &exec);
    assert_eq!(
        out.records[0].computed_class,
        EvidenceClass::MutationSurvived,
        "the finding is of class mutation-survived"
    );
    assert_eq!(
        blocks(&out),
        vec!["S01-04".to_owned()],
        "…and blocks under the pack's default weight"
    );
    assert_eq!(
        count(&out.events, EventKind::ReviewBlock),
        1,
        "written as an event"
    );
    let relaxed = confirm(&findings, ReviewWeight::ReadOnly, SHA, &exec);
    assert!(blocks(&relaxed).is_empty(), "relaxed: not blocking");
    assert_eq!(relaxed.records.len(), 1, "…but still emitted");
    assert_eq!(count(&relaxed.events, EventKind::AdvisoryCreated), 1);
    // a green suite is not an input anywhere on this path: the survivor is in the block set regardless
    assert!(
        out.records.iter().any(|r| r.ac == "S01-04"),
        "never dropped because the rest of the suite is green"
    );
}

// ------------------------------------------------------------------ AC-347 / AC-336

/// **AC-347 — no harness dispatch participates in verify; a verify activity with an agent step fails lint.**
#[test]
fn ac347_verify_dispatches_nothing() {
    // a full fixture run of the verify path: comparator, ledger, exit — collect every event it writes
    let (spec, runs) = two_ac_fixture();
    let mut events: Vec<Event> = Vec::new();
    let out = reclassify(&affected_claim(241, &["pnpm", "test"]), None);
    events.extend(out.events);
    let ledger = compute_ledger(&spec, &runs, &feasible(), &[]);
    if let Err(refused) = stage_exit("item", "implement", &ledger) {
        events.push(refused.event);
    }
    assert!(!events.is_empty(), "PREMISE: the run wrote rows");
    assert!(
        events.iter().all(|e| e.kind != EventKind::Dispatch),
        "zero dispatch events on the verify path"
    );
    // the lint over the verify activity's step kinds
    verify_activity_has_no_agent_step("ac347_verify_clean", AC347_CLEAN)
        .expect("a verify activity of tool/check/emit steps passes");
    let hit = verify_activity_has_no_agent_step("ac347_verify_with_agent_step", AC347_AGENT_STEP)
        .expect_err("an agent step at verify.pre-test fails lint");
    assert!(
        hit.to_string().contains("pre-test"),
        "the refusal names the step: {hit}"
    );
}

/// **AC-336 (unit half) — the verify step's memo key moves on a toolchain-version bump and on a
/// changed suite sha, so evidence at N cannot count at N+1 through the memo.**
#[test]
fn ac336_toolchain_version_bump_moves_the_verify_memo_key() {
    let d = |s: &str| sha256(s.as_bytes());
    let base = StepContext {
        activity_id_at_version: "verify@1".to_owned(),
        rigor: Rigor::Mandatory,
        step_id: "run-suite".to_owned(),
        resolved_inputs: vec![json!({"changed": ["src/a.rs"], "suite_sha": SHA})],
        card_or_tool: CardOrTool::Tool {
            detected_version: NODE_PIN.to_owned(),
            command: "pnpm test".to_owned(),
        },
        kb_atoms: vec![],
        loop_iteration: 0,
        output_schema: SchemaId::new("VerifyReport/v1"),
    };
    let key = base.key().unwrap();
    let mut bumped = base.clone();
    bumped.card_or_tool = CardOrTool::Tool {
        detected_version: "node 22.5.0 / vitest 2.1.0".to_owned(),
        command: "pnpm test".to_owned(),
    };
    assert_ne!(
        bumped.key().unwrap(),
        key,
        "a toolchain-version bump moves the key"
    );
    let mut resha = base.clone();
    resha.resolved_inputs = vec![
        json!({"changed": ["src/a.rs"], "suite_sha": "ffffffffffffffffffffffffffffffffffffffff"}),
    ];
    assert_ne!(
        resha.key().unwrap(),
        key,
        "a changed suite sha moves the key"
    );
    assert_eq!(
        base.key().unwrap(),
        key,
        "and the same inputs give the same key"
    );
    let _ = d("unused");
}

// ------------------------------------------------------------------ AC-239 / AC-455

/// **AC-239 (list-is-data half) — the gate-eligible adapter list is data with exactly one entry.**
#[test]
fn ac239_gate_eligible_list_is_data() {
    let list = GateEligible::from_toml("ac239", AC239_ELIGIBLE).expect("the fixture list parses");
    assert_eq!(list.phase(), 1);
    assert_eq!(list.len(), 1, "exactly one entry in Phase 1");
    let only = list.adapters().next().unwrap().to_owned();
    assert!(list.may_close(&only));
    assert!(
        !list.may_close("rk-adapter-codex"),
        "a Codex-sourced result cannot close a gate"
    );
    assert!(!list.may_close(""), "and neither can an unnamed one");
    // a Phase-1 list with two entries, or none, is refused
    assert!(matches!(
        GateEligible::from_toml("two", "phase = 1\nadapters = [\"a\", \"b\"]\n"),
        Err(EligibleError::NotExactlyOne { .. })
    ));
    assert!(matches!(
        GateEligible::from_toml("none", "phase = 1\nadapters = []\n"),
        Err(EligibleError::NotExactlyOne { .. })
    ));
    // the list is DATA: the kernel source names no adapter identity (AC-487's denylist is the lint),
    // and the evaluator takes the list as an argument rather than reading a constant
    let _evaluator: fn(&GateEligible, &str) -> bool = GateEligible::may_close_fn;
    let files = module_boundary::kernel_sources().unwrap();
    let hits: Vec<_> = files
        .iter()
        .flat_map(|f| rk_kernel::kernel_denylist::scan_source(&f.path, &f.text))
        .collect();
    assert!(hits.is_empty(), "the kernel names no adapter: {hits:#?}");
}

/// **AC-455 — the policy, not the type, decides whether a missing verification blocks a stage.**
#[test]
fn ac455_policy_decides_not_the_type() {
    let spec = SpecAc::from_toml("ac455_spec", AC455_SPEC).expect("the fixture spec parses");
    let unverified: Vec<&str> = spec
        .iter()
        .filter(|a| a.verification.is_none())
        .map(|a| a.id.as_str())
        .collect();
    assert_eq!(
        unverified,
        vec!["AC-455-B"],
        "PREMISE: one AC at verification: None"
    );
    let requires = Pack::from_toml("ac455-requires", AC455_REQUIRES).unwrap();
    let permits = Pack::from_toml("ac455-permits", AC455_PERMITS).unwrap();
    // row 1: the requiring pack refuses the stage, naming the AC id, at zero model cost (a pure fn)
    let refused = admissible("implement", &spec, &requires).expect_err("refused");
    assert_eq!(
        refused,
        Refused {
            stage: "implement".to_owned(),
            acs: vec!["AC-455-B".to_owned()]
        }
    );
    assert!(refused.to_string().contains("AC-455-B"));
    // row 2 (the must-pass control): the same spec passes under the pack whose policy does not require it
    admissible("implement", &spec, &permits).expect("the permitting pack admits the stage");
    // and the requiring pack admits a stage its policy does not name
    admissible("triage", &spec, &requires).expect("only the named stage requires a verification");
    // the type carries `verification: None` without complaint — the TYPE did not decide
    assert!(spec.iter().any(|a| a.verification.is_none()));
    // AC-337's fixture pack: an infeasible toolchain fixes every AC at not-feasible, never pinned by the mutation path
    let infeasible = Pack::from_toml("ac337", AC337_INFEASIBLE).unwrap();
    let feas = rk_kernel::verify::mutation_feasibility(&infeasible).expect("declared");
    assert!(matches!(feas, MutationFeasibility::NotFeasible { .. }));
    let (two, runs) = two_ac_fixture();
    let ledger = compute_ledger(&two, &runs, &feas, &[]);
    assert!(
        ledger
            .rows
            .iter()
            .all(|r| matches!(r.status, LedgerStatus::NotFeasible { .. })),
        "{ledger:?}"
    );
    assert!(
        stage_exit("item", "implement", &ledger).is_err(),
        "not-feasible is not pinned"
    );
    assert!(
        rk_kernel::verify::mutation_feasibility(&permits).is_err(),
        "a pack declaring nothing is a configuration error when read"
    );
}
