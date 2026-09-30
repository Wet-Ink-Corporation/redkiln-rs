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
use rk_kernel::verify::ledger::{HumanAnswer, ac_status, compute_ledger};
use rk_kernel::verify::verify_activity_has_no_agent_step;
use rk_kernel::workflow::memo::{CardOrTool, StepContext};
use rk_ports::toolchain::MutationFeasibility;
use rk_store::artifacts::verify_report::{WholeSuiteRecord, latest_whole_suite};
use rk_types::acceptance::{AcceptanceCriterion, MutationDeclaration};
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
// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT; R2 = ADR-0192 decision 4.)* The two
// fixture specs moved from the retired TOML reader's form to `AcceptanceCriterion`'s serde form,
// each criterion through `AcceptanceCriterion::ingest`, with the assertions its pins name beside.
const TWO_AC_SPEC: &str = include_str!("../fixtures/ledger/ac346_two_ac_spec.json");
const AC455_SPEC: &str = include_str!("../fixtures/ledger/ac455_spec.json");
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

fn declared(patch: &str) -> Option<MutationDeclaration> {
    Some(MutationDeclaration::Declared {
        patch: patch.to_owned(),
        suite: "npx vitest run".to_owned(),
    })
}

/// One criterion, built as a value (the ledger reads the exported type however it was made; the
/// one door is exercised by the fixture specs and `the_ledger_reads_rk_types_acceptance_criterion`).
fn spec_ac(id: &str, mutation: Option<MutationDeclaration>) -> AcceptanceCriterion {
    AcceptanceCriterion {
        id: id.to_owned(),
        criterion: Text::new(format!("{id}'s criterion")),
        gwt: None,
        verification: Some(Text::new(format!("{id}::pins it"))),
        mutation,
    }
}

/// A fixture spec in `AcceptanceCriterion`'s serde form: `{ "acceptance_criteria": [...],
/// "assertions": { "<id>": <locator> } }`, each criterion through the one door.
fn load_spec(src: &str) -> (Vec<AcceptanceCriterion>, BTreeMap<String, AssertionLocator>) {
    let doc: Value = serde_json::from_str(src).expect("the fixture spec is JSON");
    let spec = doc["acceptance_criteria"]
        .as_array()
        .expect("an acceptance_criteria array")
        .iter()
        .map(|v| AcceptanceCriterion::ingest(v.clone()).expect("each criterion ingests"))
        .collect();
    let pins = serde_json::from_value(doc["assertions"].clone()).expect("the assertions map");
    (spec, pins)
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
    let ac = spec_ac("AC-1", declared("--- a\n+++ b\n-fix\n"));
    let pin = locator("pins it");
    let status = ac_status(
        &ac,
        Some(&FlipOutcome::MutationSurvived),
        Some(&pin),
        &feasible(),
        None,
    );
    assert_eq!(status, LedgerStatus::MutationSurvived);
    let ledger = AcLedger {
        rows: vec![rk_types::artifact::ledger::AcLedgerRow {
            ac: "AC-1".to_owned(),
            status,
            evidence: Some(pin.clone()),
        }],
    };
    assert!(
        stage_exit("item-1", "implement", &ledger).is_err(),
        "a survivor blocks exit"
    );
    // pinned requires the declared assertion locator — a flip with no named assertion pins nothing
    let no_locator = spec_ac("AC-2", declared("--- a\n+++ b\n-fix\n"));
    assert_eq!(
        ac_status(
            &no_locator,
            Some(&FlipOutcome::Pinned),
            None,
            &feasible(),
            None
        ),
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
    let ac = spec_ac("AC-015", declared(AC015_PATCH));
    assert_eq!(
        ac_status(
            &ac,
            Some(&outcome),
            Some(&locator(test_name)),
            &feasible(),
            None
        ),
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
    let ac = spec_ac("AC-F17", declared("--- a\n+++ b\n-fix\n"));
    assert_eq!(
        ac_status(
            &ac,
            Some(&flip_predicate(&restored)),
            Some(&locator("pins it")),
            &feasible(),
            None
        ),
        LedgerStatus::Unpinned,
        "unverifiable is no evidence"
    );
}

// ------------------------------------------------------------------ AC-335 / AC-342

/// **AC-335 — the item's green suite changes nothing about a survivor.**
#[test]
fn ac335_green_suite_does_not_promote_a_survivor() {
    let ac = spec_ac("AC-1", declared("--- a\n+++ b\n-fix\n"));
    let pin = locator("pins it");
    // the static check: the status function's signature has NO suite-verdict parameter — adding one
    // stops this line compiling (AC-283's pattern)
    // *(dated 2026-09-29, s8g build: the criterion is the exported type, and the assertion a pin
    // names travels beside the flip outcome; still no suite-verdict parameter.)*
    let _status: StatusFn = ac_status;
    // the fixture: a green suite and a surviving mutation — the suite verdict is not even an input
    let status = ac_status(
        &ac,
        Some(&FlipOutcome::MutationSurvived),
        Some(&pin),
        &feasible(),
        None,
    );
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
        "fn status(ac: &AcceptanceCriterion, review: &Findings) -> LedgerStatus { LedgerStatus::Pinned }\n",
    ));
    assert!(
        hit.is_err(),
        "a status function taking Findings is reported"
    );
    // the fixture: the reviewer reports "all criteria met" over an unpinned AC — it still blocks exit
    let ac = spec_ac("AC-7", None);
    let status = ac_status(&ac, None, Some(&locator("pins it")), &feasible(), None);
    assert_eq!(status, LedgerStatus::Unpinned);
    let all_met = Findings { items: vec![] }; // an approving reviewer raises nothing
    assert!(all_met.items.is_empty());
    let ledger = compute_ledger(&[ac], &BTreeMap::new(), &BTreeMap::new(), &feasible(), &[]);
    let refused = stage_exit("item", "implement", &ledger)
        .expect_err("an unpinned AC refuses exit whatever the reviewer said");
    assert_eq!(refused_ids(&refused), vec!["AC-7".to_owned()]);
}

// ------------------------------------------------------------------ AC-345 / AC-346

type Pins = BTreeMap<String, AssertionLocator>;

/// AC-335's static check, as a type: the status function over the exported criterion, the flip
/// outcome and the assertion its pin names, the feasibility and the answer, and NO suite verdict.
/// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT.)*
type StatusFn = fn(
    &AcceptanceCriterion,
    Option<&FlipOutcome>,
    Option<&AssertionLocator>,
    &MutationFeasibility,
    Option<&HumanAnswer>,
) -> LedgerStatus;

/// The ledger over the exported criterion (R2 = ADR-0192 decision 4).
type LedgerFn = fn(
    &[AcceptanceCriterion],
    &BTreeMap<String, FlipOutcome>,
    &Pins,
    &MutationFeasibility,
    &[HumanAnswer],
) -> AcLedger;

fn two_ac_fixture() -> (
    Vec<AcceptanceCriterion>,
    BTreeMap<String, FlipOutcome>,
    Pins,
) {
    let (spec, pins) = load_spec(TWO_AC_SPEC);
    assert_eq!(spec.len(), 2, "PREMISE: two ACs");
    assert_eq!(pins.len(), 2, "PREMISE: each AC names its assertion");
    let mut runs = BTreeMap::new();
    runs.insert(spec[0].id.clone(), FlipOutcome::Pinned);
    runs.insert(spec[1].id.clone(), FlipOutcome::MutationSurvived);
    (spec, runs, pins)
}

/// **AC-345 — a quarantine is a review-gate state: it stops the finding blocking review and does
/// not change the AC's ledger status; stage exit is still refused for that AC.**
#[test]
fn ac345_quarantine_changes_no_ledger_status() {
    let (spec, runs, pins) = two_ac_fixture();
    let survivor = spec[1].id.clone();
    let before = compute_ledger(&spec, &runs, &pins, &feasible(), &[]);
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
    let after = compute_ledger(&spec, &runs, &pins, &feasible(), &[]);
    assert_eq!(after, before, "quarantine changes no ledger status");
    assert_eq!(after.rows[1].status, LedgerStatus::MutationSurvived);
    // and stage exit is STILL refused for that AC (AC-346; ADR-0104 decision 4)
    let refused = stage_exit("item", "implement", &after).expect_err("exit refused");
    assert_eq!(refused_ids(&refused), vec![survivor]);
}

/// **AC-346 — exit refused, the unpinned ids named, the refusal is an event (ADR-0125).**
#[test]
fn ac346_quarantined_survivor_still_refuses_exit() {
    let (spec, runs, pins) = two_ac_fixture();
    let ledger = compute_ledger(&spec, &runs, &pins, &feasible(), &[]);
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
    let still = compute_ledger(&spec, &runs, &pins, &feasible(), &[answer_other]);
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
    let lifted = compute_ledger(&spec, &runs, &pins, &feasible(), &[answer]);
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
        &compute_ledger(&spec, &all_pinned, &pins, &feasible(), &[]),
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
    let (spec, runs, pins) = two_ac_fixture();
    let mut events: Vec<Event> = Vec::new();
    let out = reclassify(&affected_claim(241, &["pnpm", "test"]), None);
    events.extend(out.events);
    let ledger = compute_ledger(&spec, &runs, &pins, &feasible(), &[]);
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
    let (spec, _pins) = load_spec(AC455_SPEC);
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
    let (two, runs, pins) = two_ac_fixture();
    let ledger = compute_ledger(&two, &runs, &pins, &feas, &[]);
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

// ================================================================================================
// `s8g` section 8g.4 / 8g.5: the one `Toolchain::test` port-call site and its `verify_completed`
// row (R1 = ADR-0191 decisions 2 and 8, proposed), AC-321's replay, and AC-349's claims binder.
// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT.)*
// ================================================================================================

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::StepKind;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::{Activation, ToolRouting};
use rk_kernel::verify::claims::{self, PathClaim};
use rk_kernel::verify::runner::{self, TestCall, TestOrigin, VerifyIdentity};
use rk_kernel::workflow::memo::{ToolchainValues, VerbRegistry};
use rk_ports::harness::Harness;
use rk_ports::toolchain::{
    Selection, StepBound, TestOutcome, TestRunner, Toolchain, VerbCall, VerbOutput,
};
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{VerifyCompleted, VerifyRange};

use crate::reference_resolver::{step, with_inputs};
use crate::runtime::{
    CAP, CountingHarness, ITEM, STAGE, at as journaled_at, counters, empty_state, gate_site,
    journal, ports_with_toolchain, store,
};

/// An outcome of `passed` + `failed` tests at `version`.
fn s8g_outcome(passed: u32, failed: u32, version: &str) -> TestOutcome {
    TestOutcome::new(
        VerifyReport {
            basis: Basis::WholeSuite,
            argv: vec!["cargo".to_owned(), "nextest".to_owned(), "run".to_owned()],
            selected_test_count: passed + failed,
            total_test_count: passed + failed,
            total_test_count_source: TotalTestCountSource::OwnExecutedCount,
            wall_secs: 3,
            toolchain_version: version.to_owned(),
            suite_sha: "suite-sha".to_owned(),
            basis_reason: Some(BasisReason::ProbeUnverified),
        },
        passed,
        failed,
        TestRunner::named("fixture", "0"),
    )
    .expect("the counts agree with the report")
}

/// A `Toolchain` whose `test` answers a script, in order, and counts every call.
#[derive(Default)]
struct ScriptedTests {
    answers: Mutex<VecDeque<Result<TestOutcome, StepError>>>,
    tests: AtomicU32,
    others: AtomicU32,
}

impl ScriptedTests {
    fn new(answers: Vec<Result<TestOutcome, StepError>>) -> Self {
        Self {
            answers: Mutex::new(answers.into()),
            ..Self::default()
        }
    }
    fn tests(&self) -> u32 {
        self.tests.load(Ordering::SeqCst)
    }
    fn others(&self) -> u32 {
        self.others.load(Ordering::SeqCst)
    }
}

impl Toolchain for ScriptedTests {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self, _b: StepBound) -> Result<rk_types::verb_output::BuildReport, StepError> {
        self.others.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: "build".to_owned(),
        })
    }
    fn test(&self, _s: &Selection, _b: StepBound) -> Result<TestOutcome, StepError> {
        self.tests.fetch_add(1, Ordering::SeqCst);
        self.answers
            .lock()
            .expect("not poisoned")
            .pop_front()
            .unwrap_or_else(|| Ok(s8g_outcome(1, 0, "fixture 1")))
    }
    fn lint(&self, _b: StepBound) -> Result<(), StepError> {
        self.others.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn classify(&self, _p: &str) -> Result<rk_ports::toolchain::Classification, StepError> {
        Ok(rk_ports::toolchain::Classification::Text)
    }
    fn change_summary(
        &self,
        _r: &str,
        _b: StepBound,
    ) -> Result<rk_ports::toolchain::ChangeSummary, StepError> {
        self.others.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: "change_summary".to_owned(),
        })
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.others.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: call.verb.to_owned(),
        })
    }
}

fn s8g_identity(adapter: &str, base: &str, head: &str) -> VerifyIdentity {
    VerifyIdentity {
        adapter: adapter.to_owned(),
        range: VerifyRange {
            base: base.to_owned(),
            head: head.to_owned(),
        },
    }
}

/// One runner call over `tc`, the rows it appended collected.
fn s8g_call(
    tc: &ScriptedTests,
    origin: TestOrigin<'_>,
    stage: &str,
    id: &VerifyIdentity,
    selection: &Selection,
    paths: &[String],
) -> (Result<TestOutcome, StepError>, Vec<Event>) {
    let mut rows: Vec<Event> = Vec::new();
    let result = runner::run_test(
        tc,
        &TestCall {
            item: ITEM,
            selection,
            bound: StepBound::Undeclared,
            origin,
            stage,
            identity: id,
            paths,
        },
        &mut |e| {
            rows.push(e);
            Ok(())
        },
    );
    (result, rows)
}

fn s8g_whole() -> Selection {
    Selection::WholeSuite {
        reason: BasisReason::ProbeUnverified,
    }
}

fn s8g_rows(events: &[Event]) -> Vec<VerifyCompleted> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::VerifyCompleted)
        .map(|e| VerifyCompleted::from_payload(&e.payload).expect("the row reads back"))
        .collect()
}

/// A `toolchain:test` step named `id` whose selection is the declared `e2e`.
fn s8g_test_step(id: &str) -> rk_kernel::catalog::Step {
    let mut s = with_inputs(step(id, StepKind::Tool), &[("selection", "e2e")]);
    s.run = Some("toolchain:test".to_owned());
    s
}

fn s8g_lint_step(id: &str) -> rk_kernel::catalog::Step {
    let mut s = step(id, StepKind::Tool);
    s.run = Some("toolchain:lint".to_owned());
    s
}

/// Walk `steps` as the activation `aid` over `tc`, with `id` bound on the routing when given.
fn s8g_walk(
    aid: &str,
    steps: &[rk_kernel::catalog::Step],
    tc: Arc<ScriptedTests>,
    id: Option<VerifyIdentity>,
) -> (rk_kernel::runtime::activity::ActivityReport, Vec<Event>) {
    let (q, a) = counters();
    let db = store(&q, &a);
    let mut routing = ToolRouting::new(
        crate::tool_verb_routing::registry("", ""),
        ToolchainValues::default(),
    );
    if let Some(id) = id {
        routing = routing.with_verify_identity(id);
    }
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::new(CountingHarness::scripted(Vec::new())) as Arc<dyn Harness>,
            &db,
            tc as Arc<dyn Toolchain>,
        ),
        "run-s8g-verify",
        STAGE,
    )
    .with_tool_routing(routing);
    let activation = Activation {
        item: ITEM,
        activity_id_at_version: aid,
        step_id: aid,
        spec: None,
        step_specs: None,
        rigor: rk_kernel::rigor::Rigor::Optional,
    };
    let report = rt
        .run_activity_steps(
            activation,
            Some(steps),
            &rk_kernel::predicate::Context::new(),
            gate_site(&empty_state()),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            journaled_at(100),
        )
        .expect("a value");
    (report, journal(&db))
}

/// **R1 decision 8, *One site*:** one `verify_completed` row per COMPLETED `Toolchain::test`
/// port call, whatever made it — the runner called directly (the pass's route), and the
/// executor's `toolchain:test` arm in a walk. A red suite is a completed call and journals its
/// row; a `lint` call journals none.
#[test]
fn verify_completed_is_journalled_once_per_completed_test_port_call() {
    // (a) The runner directly: green, red, green -- three completed calls, three rows.
    let tc = ScriptedTests::new(vec![
        Ok(s8g_outcome(5, 0, "fixture 1")),
        Ok(s8g_outcome(3, 2, "fixture 1")),
        Ok(s8g_outcome(7, 0, "fixture 1")),
    ]);
    let id = s8g_identity("adapter-under-test", "b0", "h1");
    let mut all = Vec::new();
    for _ in 0..3 {
        let (result, rows) = s8g_call(&tc, TestOrigin::Pass, "check", &id, &s8g_whole(), &[]);
        result.expect("a completed call is Ok, green or red");
        assert_eq!(rows.len(), 1, "one row per completed call");
        all.extend(rows);
    }
    assert_eq!(tc.tests(), 3, "PREMISE: three port calls");
    let rows = s8g_rows(&all);
    assert_eq!(rows.len(), 3);
    let counts: Vec<(u32, u32)> = rows.iter().map(|r| (r.passed, r.failed)).collect();
    assert_eq!(
        counts,
        vec![(5, 0), (3, 2), (7, 0)],
        "each row carries its own call's counts, the red one included"
    );
    assert!(
        rows.iter()
            .all(|r| r.selected_test_count == r.passed + r.failed)
    );
    assert!(
        all.iter().all(|e| e.item() == Some(ITEM)),
        "scoped by the item key"
    );

    // (b) Through the executor: `lint` then two `toolchain:test` steps -- two rows, not three.
    let tc = Arc::new(ScriptedTests::default());
    let (report, events) = s8g_walk(
        "suite-run@1",
        &[
            s8g_lint_step("lint-it"),
            s8g_test_step("run-a"),
            s8g_test_step("run-b"),
        ],
        Arc::clone(&tc),
        Some(s8g_identity("adapter-under-test", "b0", "h1")),
    );
    assert_eq!(tc.tests(), 2, "PREMISE: two test port calls: {report:?}");
    assert_eq!(tc.others(), 1, "PREMISE: one lint call");
    let rows = s8g_rows(&events);
    assert_eq!(
        rows.len(),
        2,
        "the executor's `toolchain:test` arm does not reach the port through the one \
         journalling site: {} rows for 2 completed calls",
        rows.len()
    );
}

/// **R1 decision 8, *Completed only*:** a timed-out, killed, unspawnable or unreadable call
/// writes NO `verify_completed`, and its `StepError` comes back unchanged. The control: a
/// completed call over the same function writes one.
#[test]
fn no_verify_completed_for_a_run_that_did_not_complete() {
    let failures = vec![
        StepError::Timeout { after_secs: 360 },
        StepError::Transport {
            kind: TransportKind::Spawn,
            detail: "cannot run: no such binary".to_owned(),
        },
        StepError::Transport {
            kind: TransportKind::Io,
            detail: "os-killed: signal 9".to_owned(),
        },
        StepError::Transport {
            kind: TransportKind::Decode,
            detail: "unreadable".to_owned(),
        },
    ];
    let tc = ScriptedTests::new(failures.iter().cloned().map(Err).collect());
    let id = s8g_identity("adapter-under-test", "b0", "h1");
    for want in &failures {
        let (result, rows) = s8g_call(&tc, TestOrigin::Pass, "check", &id, &s8g_whole(), &[]);
        assert_eq!(
            result.as_ref().err(),
            Some(want),
            "the refusal travels back unchanged"
        );
        assert!(
            rows.is_empty(),
            "a call that did not complete ({want:?}) journalled {rows:?}"
        );
    }
    assert_eq!(tc.tests(), 4, "PREMISE: every failure reached the port");
    // CONTROL: the same function over a completed call writes its row.
    let (ok, rows) = s8g_call(&tc, TestOrigin::Pass, "check", &id, &s8g_whole(), &[]);
    ok.expect("completed");
    assert_eq!(rows.len(), 1, "CONTROL: a completed call writes one row");

    // And through the executor: a timed-out test step writes no row.
    let tc = Arc::new(ScriptedTests::new(vec![Err(StepError::Timeout {
        after_secs: 360,
    })]));
    let (report, events) = s8g_walk(
        "suite-run@1",
        &[s8g_test_step("run-a")],
        Arc::clone(&tc),
        Some(s8g_identity("adapter-under-test", "b0", "h1")),
    );
    assert_eq!(tc.tests(), 1, "PREMISE: the port was reached: {report:?}");
    assert!(
        s8g_rows(&events).is_empty(),
        "a killed walk step journalled a verify_completed row"
    );
}

/// **AC-321:** *"the range endpoints, the resolved path list and the toolchain version are read
/// back from the run event verbatim AND the replay reproduces the identical path list without
/// re-reading the working tree."* The paths name files that exist nowhere; `replay` takes the
/// rows and nothing else; a row whose `paths` member is gone does not read back at all.
#[test]
fn ac321_replay_reads_the_path_list_from_verify_completed_alone() {
    let _replay_takes_rows_only: fn(&[Event]) -> Result<Vec<VerifyCompleted>, runner::ReplayError> =
        runner::replay;
    let paths: Vec<String> = vec![
        "crates/nowhere/src/lib.rs".to_owned(),
        "docs/a file with spaces.md".to_owned(),
        "Cargo.lock".to_owned(),
    ];
    let base = "1111111111111111111111111111111111111111";
    let head = "2222222222222222222222222222222222222222";
    let tc = ScriptedTests::new(vec![Ok(s8g_outcome(
        4,
        0,
        "rustc 1.95.0 / nextest 0.9.146",
    ))]);
    let id = s8g_identity("adapter-under-test", base, head);
    let selection = Selection::Affected {
        changed: paths.clone(),
        recorded: None,
    };
    let (result, rows) = s8g_call(&tc, TestOrigin::Pass, "check", &id, &selection, &paths);
    result.expect("completed");
    // Other kinds around it are skipped, not read.
    let mut journal = vec![Event::new(
        EventKind::StageTransition,
        serde_json::json!({}),
    )];
    journal.extend(rows);
    let replayed = runner::replay(&journal).expect("the rows read back");
    assert_eq!(replayed.len(), 1);
    let row = &replayed[0];
    assert_eq!(
        serde_json::to_vec(&row.paths).expect("paths serialise"),
        serde_json::to_vec(&paths).expect("paths serialise"),
        "the path list is not byte-identical to the one the call was made over"
    );
    assert_eq!(row.range.base, base);
    assert_eq!(row.range.head, head);
    assert_eq!(row.toolchain_version, "rustc 1.95.0 / nextest 0.9.146");
    // The row is the only source: without its `paths` member it does not read back.
    let mut stripped = journal.clone();
    let payload = stripped[1]
        .payload
        .as_object_mut()
        .expect("the payload is an object");
    payload.remove("paths");
    assert!(
        runner::replay(&stripped).is_err(),
        "a row with no `paths` read back, so the list came from somewhere else"
    );
}

/// **AC-321's property half:** *"a property test over 100 synthetic ranges"*. One hundred
/// deterministic (base, head, path list, version) cases, each through the one site and back
/// through `replay`, each reproduced verbatim.
#[test]
fn ac321_replay_property_over_100_synthetic_ranges() {
    let mut state: u64 = 0x5eed_0000_0000_0321;
    let mut next = move || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        state >> 33
    };
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789-_. /é"
        .chars()
        .collect();
    let mut checked = 0u32;
    for case in 0..100u32 {
        let hex = |n: u64| format!("{n:040x}");
        let base = hex(next());
        let head = hex(next());
        let count = usize::try_from(next() % 7).expect("small");
        let paths: Vec<String> = (0..count)
            .map(|i| {
                let len = 1 + usize::try_from(next() % 24).expect("small");
                let name: String = (0..len)
                    .map(|_| alphabet[usize::try_from(next()).expect("small") % alphabet.len()])
                    .collect();
                format!("d{case}/{i}/{name}")
            })
            .collect();
        let version = format!("toolchain {}.{}", next() % 3, next() % 100);
        let tc = ScriptedTests::new(vec![Ok(s8g_outcome(1 + case, 0, &version))]);
        let id = s8g_identity("adapter-under-test", &base, &head);
        let selection = Selection::Affected {
            changed: paths.clone(),
            recorded: None,
        };
        let (result, rows) = s8g_call(&tc, TestOrigin::Pass, "check", &id, &selection, &paths);
        result.expect("completed");
        let replayed = runner::replay(&rows).expect("reads back");
        assert_eq!(replayed.len(), 1, "case {case}");
        assert_eq!(replayed[0].paths, paths, "case {case}: the path list moved");
        assert_eq!(replayed[0].range.base, base, "case {case}");
        assert_eq!(replayed[0].range.head, head, "case {case}");
        assert_eq!(replayed[0].toolchain_version, version, "case {case}");
        checked += 1;
    }
    assert_eq!(
        checked, 100,
        "the property ran over exactly 100 synthetic ranges"
    );
}

fn s8g_claim(toolchain: &str, pattern: &str) -> PathClaim {
    PathClaim {
        toolchain: toolchain.to_owned(),
        pattern: pattern.to_owned(),
    }
}

/// **AC-349:** *"GIVEN two … toolchain claims matching the same changed path at equal
/// specificity WHEN the binder resolves selection THEN it is a configuration error … AND the
/// error names both rules and the path."* Its *Verified by*: *"binder unit test with two
/// equal-specificity fixtures; … an assertion that the error message contains both rule ids."*
#[test]
fn ac349_equal_specificity_claims_are_a_configuration_error_naming_both() {
    // Two different toolchains, two different patterns, one literal directory and three literal
    // characters each: equal specificity, both matching `tools/gen.rs`.
    let tied = vec![
        s8g_claim("rust", "tools/*.rs"),
        s8g_claim("node", "tools/gen*"),
    ];
    assert!(claims::matches("tools/*.rs", "tools/gen.rs"));
    assert!(claims::matches("tools/gen*", "tools/gen.rs"));
    let err = claims::bind(&tied, "tools/gen.rs")
        .expect_err("an equal-specificity tie is a configuration error, never a guess");
    let msg = err.to_string();
    for needle in ["rust:tools/*.rs", "node:tools/gen*", "tools/gen.rs"] {
        assert!(
            msg.contains(needle),
            "the error does not name `{needle}`: {msg}"
        );
    }
    // Declaration order does not decide it: the reversed list refuses the same way.
    let reversed: Vec<PathClaim> = tied.iter().rev().cloned().collect();
    assert!(
        claims::bind(&reversed, "tools/gen.rs").is_err(),
        "the reversed list resolved: a tie was broken by order"
    );
    // CONTROLS: a strictly more specific claim wins; a path only one claim matches binds to it;
    // a path none matches binds to nothing; two claims of ONE toolchain are no tie.
    let ranked = vec![
        s8g_claim("node", "tools/*"),
        s8g_claim("rust", "tools/gen.rs"),
    ];
    assert_eq!(
        claims::bind(&ranked, "tools/gen.rs")
            .expect("no tie")
            .map(|c| c.toolchain.as_str()),
        Some("rust")
    );
    assert_eq!(
        claims::bind(&tied, "tools/other.rs")
            .expect("no tie")
            .map(|c| c.toolchain.as_str()),
        Some("rust")
    );
    assert_eq!(claims::bind(&tied, "README.md").expect("no tie"), None);
    let same = vec![
        s8g_claim("rust", "tools/*.rs"),
        s8g_claim("rust", "tools/gen*"),
    ];
    assert_eq!(
        claims::bind(&same, "tools/gen.rs")
            .expect("one toolchain's claims never tie with each other")
            .map(|c| c.toolchain.as_str()),
        Some("rust")
    );
    // The shipped claims column (`.redkiln/registry.toml`, R1 decision 10): both tables carry
    // one, and this repository's own paths bind without a tie.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let registry = VerbRegistry::load_file(&root.join(".redkiln/registry.toml"))
        .expect("the shipped registry loads");
    let shipped = claims::claims_from(&registry).expect("the claims rows read");
    let toolchains: std::collections::BTreeSet<&str> =
        shipped.iter().map(|c| c.toolchain.as_str()).collect();
    assert_eq!(
        toolchains,
        ["node", "rust"].into_iter().collect(),
        "the shipped claims column is not on both per-toolchain tables: {shipped:?}"
    );
    for (path, want) in [
        ("crates/rk-kernel/src/lib.rs", "rust"),
        ("Cargo.lock", "rust"),
        ("crates/rk-cli/Cargo.toml", "rust"),
        (
            "crates/rk-adapter-toolchain-node/tests/fixtures/project/package.json",
            "node",
        ),
        (
            "crates/rk-adapter-toolchain-node/tests/fixtures/project/pnpm-lock.yaml",
            "node",
        ),
    ] {
        let bound = claims::bind(&shipped, path)
            .unwrap_or_else(|e| panic!("the shipped claims tie on `{path}`: {e}"));
        assert_eq!(bound.map(|c| c.toolchain.as_str()), Some(want), "{path}");
    }
}

/// **R1 decision 8** (D21-47 review, finding 4): *"`origin` is `\"pass\"` for the verify pass's
/// call, and `\"<activity>@<version>/<step>\"` for a catalog step's call"*; `stage` is the node's
/// stage name, as data. A pass call and a `tdd-red-green-refactor` `red-verify` call each
/// journal one row with their own `origin` and `stage`, and the executor's row carries the
/// adapter the composition root bound, opaque.
#[test]
fn verify_completed_carries_its_origin_and_stage() {
    // The pass's arm: the runner called with `TestOrigin::Pass`, at a stage NOT named verify.
    let tc = ScriptedTests::new(vec![Ok(s8g_outcome(2, 0, "fixture 1"))]);
    let id = s8g_identity("adapter-under-test", "b0", "h1");
    let (result, rows) = s8g_call(&tc, TestOrigin::Pass, "check", &id, &s8g_whole(), &[]);
    result.expect("completed");
    let pass = s8g_rows(&rows);
    assert_eq!(pass.len(), 1);
    assert_eq!(pass[0].origin, "pass");
    assert_eq!(pass[0].stage, "check");
    assert_eq!(pass[0].adapter, "adapter-under-test");

    // The catalog step's arm: `red-verify` in `tdd-red-green-refactor@2`, through the executor.
    let tc = Arc::new(ScriptedTests::default());
    let (report, events) = s8g_walk(
        "tdd-red-green-refactor@2",
        &[s8g_test_step("red-verify")],
        Arc::clone(&tc),
        Some(s8g_identity("adapter-from-the-root", "b0", "h1")),
    );
    assert_eq!(tc.tests(), 1, "PREMISE: one port call: {report:?}");
    let step_rows = s8g_rows(&events);
    assert_eq!(
        step_rows.len(),
        1,
        "the red-verify call journalled {step_rows:?}"
    );
    assert_eq!(
        step_rows[0].origin, "tdd-red-green-refactor@2/red-verify",
        "a catalog step's row is not told apart from the pass's (M24's shape)"
    );
    assert_eq!(step_rows[0].stage, STAGE);
    assert_eq!(step_rows[0].adapter, "adapter-from-the-root");
    let row_event = events
        .iter()
        .find(|e| e.kind == EventKind::VerifyCompleted)
        .expect("the row");
    assert_eq!(
        row_event.item(),
        Some(ITEM),
        "the interim item key rides beside the payload"
    );
    assert!(
        row_event.payload.get("item").is_none(),
        "the item key is in the payload"
    );
}

/// **R2 = ADR-0192 decision 4** (`s8g` section 8g.3): *"`SpecAc` is retired. It and its TOML reader
/// leave `rk-kernel`; the ledger, the exit check and `admissibility::admissible` read
/// `&[rk_types::AcceptanceCriterion]`."* The signatures are the static half (a fn-pointer coercion
/// stops compiling if either moves off the exported type); the behavioural half runs three criteria
/// through the ONE door, `AcceptanceCriterion::ingest`, one per declaration arm, into the ledger,
/// the exit check and the admissibility row.
///
/// The assertion a pin names (AC-333) is not a member of the exported type (decision 3's field
/// list has none): it is the harness's evidence, handed to the ledger beside the flip outcomes,
/// keyed by criterion id. *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT.)*
#[test]
fn the_ledger_reads_rk_types_acceptance_criterion() {
    use rk_types::acceptance::MutationDeclaration as Declared;
    let _ledger: LedgerFn = compute_ledger;
    let _status: StatusFn = ac_status;
    let _admit: fn(&str, &[AcceptanceCriterion], &Pack) -> Result<(), Refused> = admissible;

    let spec: Vec<AcceptanceCriterion> = [
        json!({
            "id": "AC-A",
            "criterion": "the plan steps are ordered",
            "verification": "test/implement-contract.test.ts::orders the plan steps",
            "mutation": {
                "kind": "declared",
                "patch": "--- a/w.js\n+++ b/w.js\n@@ -1 +1 @@\n-const A = 1;\n+const A = 2;\n",
                "suite": "npx vitest run test/implement-contract.test.ts"
            }
        }),
        json!({
            "id": "AC-B",
            "criterion": "the render is legible",
            "verification": "judged at review",
            "mutation": { "kind": "not_feasible", "reason": "no seam to mutate a judgement" }
        }),
        json!({ "id": "AC-C", "criterion": "declares nothing" }),
    ]
    .into_iter()
    .map(|v| AcceptanceCriterion::ingest(v).expect("each criterion comes through the one door"))
    .collect();
    assert!(matches!(spec[0].mutation, Some(Declared::Declared { .. })));
    assert!(matches!(
        spec[1].mutation,
        Some(Declared::NotFeasible { .. })
    ));
    assert!(spec[2].mutation.is_none() && spec[2].verification.is_none());

    let mut runs = BTreeMap::new();
    runs.insert("AC-A".to_owned(), FlipOutcome::Pinned);
    runs.insert("AC-B".to_owned(), FlipOutcome::Pinned);
    let mut assertions = BTreeMap::new();
    assertions.insert("AC-A".to_owned(), locator("orders the plan steps"));
    let ledger = compute_ledger(&spec, &runs, &assertions, &feasible(), &[]);
    let statuses: Vec<(&str, &LedgerStatus)> = ledger
        .rows
        .iter()
        .map(|r| (r.ac.as_str(), &r.status))
        .collect();
    assert_eq!(
        statuses,
        vec![
            ("AC-A", &LedgerStatus::Pinned),
            (
                "AC-B",
                &LedgerStatus::NotFeasible {
                    reason: Text::new("no seam to mutate a judgement")
                }
            ),
            ("AC-C", &LedgerStatus::Unpinned),
        ],
        "each declaration arm of the exported type is read"
    );
    assert_eq!(
        ledger.rows[0].evidence,
        Some(locator("orders the plan steps"))
    );
    // A flip with no named assertion pins nothing (AC-333), from the exported type too.
    let unnamed = compute_ledger(&spec[..1], &runs, &BTreeMap::new(), &feasible(), &[]);
    assert_eq!(unnamed.rows[0].status, LedgerStatus::Unpinned);
    // The exit check over that ledger, and the admissibility row over the same criteria.
    let refused = stage_exit("item", "verify", &ledger).expect_err("two ACs are not pinned");
    assert_eq!(
        refused_ids(&refused),
        vec!["AC-B".to_owned(), "AC-C".to_owned()]
    );
    let requires = Pack::from_toml("ac455-requires", AC455_REQUIRES).expect("loads");
    let refusal = admissible("implement", &spec, &requires).expect_err("AC-C has no verification");
    assert_eq!(refusal.acs, vec!["AC-C".to_owned()]);
}

/// **AC-347's lint over the SHIPPED catalog** (`s8g` section 8g.5; D21-46 R5; R1 = ADR-0191
/// decision 7, proposed). *(dated 2026-09-29, s8g build, package S8G-P4-PACK-REHOME.)* Every
/// `.redkiln/catalog` activity with `stage = "verify"` is walked through the kernel's own lint,
/// [`verify_activity_has_no_agent_step`], file by file; the count walked is non-zero (D21-42 13a),
/// and the set that carries an agent step is **exactly** `{migration-dry-run}`, the single residual
/// D21-46 R5 names (*"`migration-dry-run` stays a named residual, because no Phase-1 lane reaches
/// it"*). A new agent step at verify turns this red (M19); so does `integration-tests` back at
/// verify (M20), and so does removing the residual without updating this test. The test states the
/// residual; it does not hide it. Reachability is `selection::migration_dry_run_is_selected_on_no_shipped_lane`'s.
#[test]
fn ac347_verify_stage_agent_steps_are_exactly_the_named_residual() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.redkiln/catalog")
        .canonicalize()
        .expect("the shipped catalog directory resolves");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
        .expect("the shipped catalog reads")
        .map(|e| e.expect("an entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    let mut walked = Vec::new();
    let mut agent_bearing = std::collections::BTreeSet::new();
    for path in &files {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a UTF-8 file name");
        let src = std::fs::read_to_string(path).expect("the activity reads");
        let catalog = rk_kernel::catalog::Catalog::from_sources(&[(name, src.as_str())])
            .unwrap_or_else(|e| panic!("{name} loads: {e:?}"));
        for a in catalog.activities() {
            if a.stage == "verify" {
                walked.push(a.id.clone());
            }
        }
        match verify_activity_has_no_agent_step(name, &src) {
            Ok(()) => {}
            Err(rk_kernel::verify::VerifyActivityError::AgentStep { activity, .. }) => {
                agent_bearing.insert(activity);
            }
            Err(other) => panic!("{name}: the lint did not run: {other}"),
        }
    }
    assert!(
        files.len() >= 100,
        "the shipped catalog read {} files",
        files.len()
    );
    assert!(
        !walked.is_empty(),
        "zero verify-stage activities walked — a green over nothing (D21-42 13a)"
    );
    println!(
        "ac347-shipped: walked {} verify-stage activities {:?}; agent-bearing {:?}",
        walked.len(),
        walked,
        agent_bearing
    );
    assert_eq!(
        agent_bearing,
        std::collections::BTreeSet::from(["migration-dry-run".to_owned()]),
        "the verify stage's agent-bearing activities are not exactly AC-347's named residual \
         (D21-46 R5)"
    );
}
