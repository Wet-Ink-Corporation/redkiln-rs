//! **The review gate, R2 — the classifier and the policy table** — Step 7b-ii, `s7b-ii` §7b-ii.3
//! and §7b-ii.4. *Reviewer proposes, machine confirms* (ADR-0074; ADR-0096).
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac284_class_decision_table`] | **AC-284** — PROVISIONAL (S-CLASS-REPLAY), built not claimed | one row per branch of the total table plus the `otherwise` rows; the source carries no `_ =>` arm that yields a blocking class; a seeded fuzz never panics and yields only the four | agreement with the model's labels over the 253 S-TYPED findings — S-CLASS-REPLAY's |
//! | [`ac285_class_drift_event`] | **AC-285** | the computed class is used and the model label retained; `review_class_drift` carries both; the block set follows the class; the policy module has no reader of `evidence_label` (source rule, with its committed control) | — |
//! | [`ac286_classifier_is_deterministic`] | **AC-286** | 1,000 in-process draws twice and once in a FRESH PROCESS give one digest; the classifier module names no clock/env/fs (source rule, with its committed control) | — |
//! | [`ac283_no_free_text_reaches_policy`] | **AC-283** | the two functions' signatures, pinned by fn-pointer coercion — a free-text parameter stops the coercion compiling; the rationale property as the corollary | — |
//! | [`ac282_structured_output_required`] | **AC-282** (`rk-kernel` half) | the F20 golden transcript (exit 0, `subtype: success`, no `structured_output`) is an ERROR with no finding set; a present payload ingests | the adapter half (7c) |
//! | [`ac318_no_json_from_prose_in_the_review_path`] | **AC-318** | the review path calls no text-to-JSON extraction; the committed lint-fail fixture is reported | — |
//! | [`ac294_block_constructor_is_private_outside_policy`] | **AC-294** | zero block-value construction sites outside `review/policy.rs` (source rule); the compile-fail case exists, constructs a block and commits a `.stderr` | the `--stay` half, which is `module_boundary.rs`'s AC-294 test from 7b |
//! | [`ac295_weight_change_adds_only_narrowed_pin`] | **AC-295** (fixture half; the recorded-corpus replay is ARMED to Step 8) | Lenient → Strict changes the block set by exactly the `narrowed-pin` items; classes byte-identical; no re-dispatch | a recorded corpus run |
//! | [`ac296_advisory_never_blocks`] | **AC-296** | `proptest` over every weight × every class × every outcome: `advisory` is never a block | — |
//! | [`ac298_unset_weight_is_lenient`] | **AC-298** | unset → `Lenient`; the config golden file over every rigor | — |
//! | [`ac299_read_only_spawns_nothing`] | **AC-299** | empty block set at `ReadOnly` regardless of class AND the re-executor was called zero times | — |
//! | [`ac300_fix_lane_is_strict`] | **AC-300** (pack-validation half) | shipped `bug-standard` passes; the lowered copy fails naming the declared weight | the `doctor` row (7d) |
//! | [`ac301_cheap_first_cascade_is_refused`] | **AC-301** (the refusal; the lift is ARMED) | the cascade fixture is refused with the exact reason; every shipped pack passes | — |
//! | [`ac302_review_tier_takes_no_model_value`] | **AC-302** | `resolve_review_tier`'s signature; the call-graph rule with a scratch violation | — |
//! | [`ac303_confirm_step_survives_escalation`] | **AC-303** | the F37 row RED on the pre-ceiling fixture (root-cause → bisect loses the reviewer AND the check); the twin discrimination (a lost check alone is AC-303's, not AC-105's); GREEN on the shipped catalog with the ceiling asserted | — |
//! | [`ac303_multi_hop_confirm_is_seen`], [`ac303_twin_broken_chain_is_lost`], [`ac303_read_binding_is_strict`], [`ac303_back_edge_counts_and_is_reported`] | **AC-303** (D21-57's identification rule) | a check consuming the Findings one step removed is a confirm step (RED first under the name rule); an `artifact:` input breaks the chain; a declared-but-unread binding does not count; a back-edge-only dependence counts and is reported `back-edge` | that a `back-edge` dependence is as strong as a `forward` one |
//!
//! The Step 7b-ii rows above were red first, every one, against a skeleton `review` module that
//! carried the ABSENT behaviour (everything `advisory`; blocks at every weight; unset → `Strict`;
//! an absent `structured_output` read as zero findings; lints that reported nothing) — the outputs
//! are in the slice's commit. *(dated 2026-09-25, s8b refuting round 1)* Of the four D21-57 rows,
//! only [`ac303_multi_hop_confirm_is_seen`] was shown red first, under the old name rule, on
//! 2026-09-25; the other three were authored with the rule and carry no skeleton output.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use proptest::prelude::*;
use rk_kernel::module_boundary::{
    self, SourceFile, block_construction_sites_rule, classifier_purity_rule,
    policy_reads_no_model_label_rule, review_path_no_prose_json_rule, review_tier_call_graph_rule,
};
use rk_kernel::review::classify::{
    EvidenceClass, MutateObservation, Observation, RunObservation, classify,
};
use rk_kernel::review::confirm::{Confirmed, confirm};
use rk_kernel::review::ingest::{ReviewIngestError, findings_from_result_event};
use rk_kernel::review::pack_rules::{
    ReviewPackError, cascade_is_refused, fix_lane_review_is_strict,
};
use rk_kernel::review::policy::{
    self, Decision, NonBlockReason, ReviewWeight, resolve_review_tier,
};
use rk_kernel::review::reexec::{ReExecution, ReExecutionRecord, ReExecutor, Unverifiable};
use rk_kernel::rigor::{
    ConfirmSurvey, Dependence, Rigor, confirm_step_monotonicity_rule, rigor_monotonicity_rule,
};
use rk_kernel::select::{Catalog, Pack};
use rk_kernel::toml_subset;
use rk_types::artifact::findings::{EvidenceLabel, Finding, Findings};
use rk_types::error::StepError;
use rk_types::event_kind::EventKind;
use rk_types::repro::{Expected, Locator, Mutation, Repro, ReproKind};
use rk_types::text::Text;

const SHA: &str = "0123456789abcdef0123456789abcdef01234567";
const F20_GOLDEN: &str = include_str!("../fixtures/findings/ac282_f20_golden_transcript.jsonl");
const AC285_CONTROL: &str =
    include_str!("../fixtures/lint-controls/ac285_policy_reads_the_model_label.rs");
const AC286_CONTROL: &str =
    include_str!("../fixtures/lint-controls/ac286_classifier_reads_the_clock.rs");
const AC318_CONTROL: &str =
    include_str!("../fixtures/lint-controls/ac318_review_parses_prose_as_json.rs");
const AC294_CASE: &str = include_str!("../ui/ac294_block_constructor.rs");
const AC300_LOWERED: &str = include_str!("../fixtures/packs/ac300-fix-lane-lowered.toml");
const AC301_CASCADE: &str = include_str!("../fixtures/packs/ac301-cheap-first-cascade.toml");
const AC298_GOLDEN: &str =
    include_str!("../fixtures/packs/ac298-review-weight-defaults.golden.json");
const CLASSIFY_SRC: &str = include_str!("../../src/review/classify.rs");

// ------------------------------------------------------------------ fixtures

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

fn mutate_repro(patch: &str, with_assertion: bool) -> Repro {
    Repro {
        kind: ReproKind::Mutate,
        command: "cargo test -p rk-kernel --test it".to_owned(),
        expected: if with_assertion {
            Expected::Assertion {
                assertion: Locator {
                    file: "tests/it/review.rs".to_owned(),
                    test: "ac284_class_decision_table".to_owned(),
                },
            }
        } else {
            Expected::Exit { exit: 0 }
        },
        observed: "suite green".to_owned(),
        expected_when_met: "suite red".to_owned(),
        mutation: Some(Mutation {
            patch: patch.to_owned(),
            suite: "cargo test -p rk-kernel".to_owned(),
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
        files: vec!["src/lib.rs".to_owned()],
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
    Observation::Mutate(MutateObservation {
        applied: true,
        suite_green: true,
        reverted_fix,
        assertion_flipped,
    })
}

/// A re-executor that returns scripted records and COUNTS its calls — AC-299's process counter.
struct Scripted {
    script: BTreeMap<String, (Observation, ReExecution)>,
    calls: AtomicUsize,
}

impl Scripted {
    fn new(rows: Vec<(&str, Observation, ReExecution)>) -> Self {
        Self {
            script: rows
                .into_iter()
                .map(|(ac, o, r)| (ac.to_owned(), (o, r)))
                .collect(),
            calls: AtomicUsize::new(0),
        }
    }

    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl ReExecutor for Scripted {
    fn run(&self, finding: &Finding, sha: &str) -> Option<ReExecutionRecord> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let (observation, outcome) = self.script.get(&finding.ac).cloned().unwrap_or((
            Observation::Unrecognised,
            ReExecution::Unverifiable(Unverifiable::CannotRun),
        ));
        Some(ReExecutionRecord {
            id: format!("reexec:{}:{sha}", finding.ac),
            observation,
            outcome,
        })
    }
}

/// The three-finding payload most tests share: a reproduced regression, a narrowed pin, an advisory.
fn payload() -> (Findings, Scripted) {
    let findings = Findings {
        items: vec![
            finding(
                "AC-001",
                EvidenceLabel::Reproduced,
                run_repro("cargo test -p rk-kernel -- ac001"),
            ),
            finding(
                "AC-002",
                EvidenceLabel::NarrowedPin,
                mutate_repro("--- a\n+++ b\n", true),
            ),
            finding(
                "AC-003",
                EvidenceLabel::Advisory,
                run_repro("cargo test -p rk-kernel -- ac003"),
            ),
        ],
    };
    let exec = Scripted::new(vec![
        ("AC-001", ran("exit 101"), ReExecution::Flipped),
        ("AC-002", survived(false, Some(false)), ReExecution::Flipped),
        (
            "AC-003",
            ran("exit 0"),
            ReExecution::Unverifiable(Unverifiable::BothFail),
        ),
    ]);
    (findings, exec)
}

fn kernel_sources() -> Vec<SourceFile> {
    module_boundary::kernel_sources().expect("src/ is readable")
}

fn control(path: &str, text: &str) -> Vec<SourceFile> {
    vec![SourceFile {
        path: path.to_owned(),
        text: text.to_owned(),
    }]
}

// ------------------------------------------------------------------ AC-284

/// **AC-284 — the class is the total decision table and nothing else.** PROVISIONAL
/// (S-CLASS-REPLAY): built and tested, not claimed as evidenced.
///
/// > `reproduced` iff `kind = run`, the command ran and `observed ≠ expected_when_met`;
/// > `mutation-survived` iff `kind = mutate`, the patch was applied, the named suite stayed green
/// > and the mutant discriminates; `narrowed-pin` iff only the discriminating assertion flips, or a
/// > surviving mutant has an assertion that did not flip; `advisory` in every remaining case,
/// > including a surviving mutant with no assertion and every unrecognised input.
///
/// RED (observed 2026-09-17) against a classifier that answered `advisory` for every row.
#[test]
fn ac284_class_decision_table() {
    let run = run_repro("cargo test -p rk-kernel -- ac284");
    let mut_with = mutate_repro("--- a\n+++ b\n-fix\n", true);
    let mut_without = mutate_repro("--- a\n+++ b\n-fix\n", false);
    let table: Vec<(&str, &Repro, Observation, EvidenceClass)> = vec![
        // ---- kind = run ----
        (
            "run, ran, observed != expected_when_met",
            &run,
            ran("exit 101"),
            EvidenceClass::Reproduced,
        ),
        (
            "run, ran, observed == expected_when_met",
            &run,
            ran("exit 0"),
            EvidenceClass::Advisory,
        ),
        (
            "run, did not run",
            &run,
            Observation::Run(RunObservation {
                ran: false,
                observed: String::new(),
                only_assertion_flipped: false,
            }),
            EvidenceClass::Advisory,
        ),
        (
            "run, only the discriminating assertion flipped",
            &run,
            Observation::Run(RunObservation {
                ran: true,
                observed: "exit 0".to_owned(),
                only_assertion_flipped: true,
            }),
            EvidenceClass::NarrowedPin,
        ),
        // ---- kind = mutate ----
        (
            "mutate, applied, green, reverted fix",
            &mut_without,
            survived(true, None),
            EvidenceClass::MutationSurvived,
        ),
        (
            "mutate, applied, green, assertion flipped",
            &mut_with,
            survived(false, Some(true)),
            EvidenceClass::MutationSurvived,
        ),
        (
            "mutate, applied, green, assertion did NOT flip",
            &mut_with,
            survived(false, Some(false)),
            EvidenceClass::NarrowedPin,
        ),
        (
            "mutate, applied, green, no assertion, not a reverted fix (equivalent mutant)",
            &mut_without,
            survived(false, None),
            EvidenceClass::Advisory,
        ),
        (
            "mutate, not applied",
            &mut_with,
            Observation::Mutate(MutateObservation {
                applied: false,
                suite_green: false,
                reverted_fix: true,
                assertion_flipped: None,
            }),
            EvidenceClass::Advisory,
        ),
        (
            "mutate, applied, suite RED (the mutant was killed)",
            &mut_with,
            Observation::Mutate(MutateObservation {
                applied: true,
                suite_green: false,
                reverted_fix: true,
                assertion_flipped: Some(true),
            }),
            EvidenceClass::Advisory,
        ),
        // ---- the `otherwise` rows ----
        (
            "run repro, mutate observation (kinds disagree)",
            &run,
            survived(true, Some(true)),
            EvidenceClass::Advisory,
        ),
        (
            "mutate repro, run observation (kinds disagree)",
            &mut_with,
            ran("exit 101"),
            EvidenceClass::Advisory,
        ),
        (
            "run repro, unrecognised",
            &run,
            Observation::Unrecognised,
            EvidenceClass::Advisory,
        ),
        (
            "mutate repro, unrecognised",
            &mut_with,
            Observation::Unrecognised,
            EvidenceClass::Advisory,
        ),
    ];
    for (name, repro, obs, want) in &table {
        assert_eq!(classify(repro, obs), *want, "row `{name}`");
    }
    assert!(
        table.iter().any(|r| r.3 == EvidenceClass::Reproduced)
            && table.iter().any(|r| r.3 == EvidenceClass::MutationSurvived)
            && table.iter().any(|r| r.3 == EvidenceClass::NarrowedPin),
        "PREMISE: the table reaches every blocking class, or `advisory-everything` passes it"
    );

    // the exhaustive match has no `_ =>` fallthrough to a blocking class
    for line in CLASSIFY_SRC.lines() {
        let code = line.trim();
        if code.starts_with("//") {
            continue;
        }
        if code.starts_with("_ =>") {
            assert!(
                !code.contains("Reproduced")
                    && !code.contains("MutationSurvived")
                    && !code.contains("NarrowedPin"),
                "a `_ =>` arm yields a BLOCKING class: {code}"
            );
        }
    }

    // the fuzz: a seeded generator over shapes never panics and only ever yields the four
    let mut x = 7u64;
    let mut next = move || {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        x >> 33
    };
    for _ in 0..2_000 {
        let repro = if next() % 2 == 0 {
            run.clone()
        } else {
            mut_with.clone()
        };
        let obs = match next() % 3 {
            0 => Observation::Run(RunObservation {
                ran: next() % 2 == 0,
                observed: if next() % 2 == 0 {
                    "exit 0"
                } else {
                    "exit 101"
                }
                .to_owned(),
                only_assertion_flipped: next() % 2 == 0,
            }),
            1 => Observation::Mutate(MutateObservation {
                applied: next() % 2 == 0,
                suite_green: next() % 2 == 0,
                reverted_fix: next() % 2 == 0,
                assertion_flipped: match next() % 3 {
                    0 => None,
                    1 => Some(false),
                    _ => Some(true),
                },
            }),
            _ => Observation::Unrecognised,
        };
        let class = classify(&repro, &obs);
        assert!(EvidenceClass::ALL.contains(&class));
    }
}

// ------------------------------------------------------------------ AC-285

/// **AC-285 — the computed class is used; the model label is retained and drift is recorded.**
///
/// > THEN the computed class is used, the model label is retained only on the persisted finding,
/// > and a `review_class_drift` event carries both. *Verified by:* a unit test on a disagreeing
/// > fixture asserting class, event emission and block set, plus an architecture lint that
/// > `finding.evidence_label` has no reader inside the policy module.
///
/// RED (observed 2026-09-17): the skeleton emitted no drift event.
#[test]
fn ac285_class_drift_event() {
    // the model says `reproduced`; the gate saw the command pass → advisory
    let optimistic = Findings {
        items: vec![finding(
            "AC-010",
            EvidenceLabel::Reproduced,
            run_repro("cargo test -- ac010"),
        )],
    };
    let exec = Scripted::new(vec![(
        "AC-010",
        ran("exit 0"),
        ReExecution::Unverifiable(Unverifiable::BothFail),
    )]);
    let out = confirm(&optimistic, ReviewWeight::Strict, SHA, &exec);
    assert_eq!(
        out.records[0].computed_class,
        EvidenceClass::Advisory,
        "the computed class is used"
    );
    assert_eq!(
        out.records[0].evidence_label,
        EvidenceLabel::Reproduced,
        "the model label is retained on the record"
    );
    assert!(
        out.block_set().is_empty(),
        "the block set follows the CLASS, not the label"
    );
    let drift: Vec<&rk_types::event::Event> = out
        .events
        .iter()
        .filter(|e| e.kind == EventKind::ReviewClassDrift)
        .collect();
    assert_eq!(
        drift.len(),
        1,
        "one review_class_drift row for one disagreement"
    );
    assert_eq!(drift[0].payload["ac"], "AC-010");
    assert_eq!(drift[0].payload["model_label"], "reproduced");
    assert_eq!(drift[0].payload["computed_class"], "advisory");

    // the mirror: the model says `advisory`; the gate saw it flip → reproduced, and it BLOCKS
    let pessimistic = Findings {
        items: vec![finding(
            "AC-011",
            EvidenceLabel::Advisory,
            run_repro("cargo test -- ac011"),
        )],
    };
    let exec = Scripted::new(vec![("AC-011", ran("exit 101"), ReExecution::Flipped)]);
    let out = confirm(&pessimistic, ReviewWeight::Lenient, SHA, &exec);
    assert_eq!(out.records[0].computed_class, EvidenceClass::Reproduced);
    assert_eq!(
        out.block_set().len(),
        1,
        "a reproduced, flipped finding blocks whatever the model called it"
    );
    assert_eq!(
        out.events
            .iter()
            .filter(|e| e.kind == EventKind::ReviewClassDrift)
            .count(),
        1
    );

    // agreement writes no drift row
    let agreed = Findings {
        items: vec![finding(
            "AC-012",
            EvidenceLabel::Reproduced,
            run_repro("cargo test -- ac012"),
        )],
    };
    let exec = Scripted::new(vec![("AC-012", ran("exit 101"), ReExecution::Flipped)]);
    let out = confirm(&agreed, ReviewWeight::Lenient, SHA, &exec);
    assert_eq!(
        out.events
            .iter()
            .filter(|e| e.kind == EventKind::ReviewClassDrift)
            .count(),
        0
    );

    // ---- the architecture lint: no reader of evidence_label inside the policy module ----------
    let report = policy_reads_no_model_label_rule(&kernel_sources())
        .unwrap_or_else(|v| panic!("the policy module reads the model label:\n{v:#?}"));
    assert!(
        report.files_scanned >= 1,
        "the rule found the policy module"
    );
    let violations = policy_reads_no_model_label_rule(&control(
        "crates/rk-kernel/src/review/policy.rs",
        AC285_CONTROL,
    ))
    .expect_err("the committed control reads evidence_label inside a policy-module path and must be reported");
    assert!(
        violations
            .iter()
            .any(|v| v.text().contains("evidence_label"))
    );
}

// ------------------------------------------------------------------ AC-286

const AC286_MARKER: &str = "RK_AC286_CHILD";

fn classifier_digest() -> String {
    // 1,000 draws over a seeded generator, folded into one digest
    let run = run_repro("cargo test -p rk-kernel -- ac286");
    let mutate = mutate_repro("--- a\n+++ b\n", true);
    let mut x = 42u64;
    let mut next = move || {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        x >> 33
    };
    let mut classes = String::new();
    for _ in 0..1_000 {
        let repro = if next() % 2 == 0 { &run } else { &mutate };
        let obs = match next() % 3 {
            0 => ran(if next() % 2 == 0 {
                "exit 0"
            } else {
                "exit 101"
            }),
            1 => survived(
                next() % 2 == 0,
                match next() % 3 {
                    0 => None,
                    1 => Some(false),
                    _ => Some(true),
                },
            ),
            _ => Observation::Unrecognised,
        };
        classes.push_str(classify(repro, &obs).as_str());
        classes.push(',');
    }
    rk_types::digest::sha256(classes.as_bytes())
        .as_str()
        .to_owned()
}

/// The child half of AC-286's fresh-process draw. Refuses without the marker.
#[test]
#[ignore = "child entry point: run by ac286_classifier_is_deterministic in a fresh process"]
fn ac286_child_prints_the_classifier_digest() {
    assert!(
        std::env::var(AC286_MARKER).is_ok(),
        "child entry point run without the marker"
    );
    println!("AC286-DIGEST={}", classifier_digest());
}

/// **AC-286 — the classifier is deterministic and reads nothing ambient.**
///
/// > THEN 1,000 draws in one process and again in a fresh process give identical results, and the
/// > classifier reads no clock, no environment variable and no file. *Verified by:* a `proptest`
/// > determinism property plus a source-level lint over the classifier module forbidding
/// > `std::fs`, `std::time`, `std::env` and `SystemTime`/`Instant` call sites.
///
/// RED (observed 2026-09-17): the source rule reported nothing over the committed control that
/// reads `SystemTime::now()` inside a classifier-module path — the register's own red for this id.
#[test]
#[expect(
    clippy::disallowed_types,
    reason = "AC-286's instrument is a FRESH PROCESS; the child is this test binary re-exec'd, the \
              pattern tests/it/cross_process.rs:195-232 established. clippy.toml's Command entry \
              is Checkpoint 6's git ban and this is not a git child."
)]
fn ac286_classifier_is_deterministic() {
    let once = classifier_digest();
    let twice = classifier_digest();
    assert_eq!(
        once, twice,
        "two passes of 1,000 draws in one process agree"
    );

    // the proptest property: the same (repro, observation) always classifies the same
    proptest!(ProptestConfig::with_cases(256), |(k in 0u8..2, ran in any::<bool>(), obs in 0u8..3, a in any::<bool>(), b in any::<bool>(), c in 0u8..3)| {
        let repro = if k == 0 { run_repro("cargo test -- p") } else { mutate_repro("--- a\n+++ b\n", true) };
        let observation = match obs {
            0 => Observation::Run(RunObservation { ran, observed: if a { "exit 0" } else { "exit 101" }.to_owned(), only_assertion_flipped: b }),
            1 => Observation::Mutate(MutateObservation { applied: ran, suite_green: a, reverted_fix: b, assertion_flipped: match c { 0 => None, 1 => Some(false), _ => Some(true) } }),
            _ => Observation::Unrecognised,
        };
        prop_assert_eq!(classify(&repro, &observation), classify(&repro, &observation));
    });

    // the fresh process
    let exe = std::env::current_exe().expect("the test binary's own path");
    let out = std::process::Command::new(&exe)
        .args([
            "review::ac286_child_prints_the_classifier_digest",
            "--exact",
            "--ignored",
            "--nocapture",
        ])
        .env(AC286_MARKER, "1")
        .output()
        .expect("re-exec");
    assert!(
        out.status.success(),
        "the child ran: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let child = stdout
        .lines()
        .find_map(|l| l.strip_prefix("AC286-DIGEST="))
        .expect("the child printed its digest");
    assert_eq!(child, once, "a fresh process draws the same 1,000 classes");

    // ---- the source-level lint over the classifier module's own syntax ----------------------
    let report = classifier_purity_rule(&kernel_sources()).unwrap_or_else(|v| {
        panic!("the classifier module names a clock, an env read or the filesystem:\n{v:#?}")
    });
    assert!(
        report.files_scanned >= 1 && report.code_lines_scanned > 10,
        "the rule read the classifier: {report:?}"
    );
    let violations = classifier_purity_rule(&control(
        "crates/rk-kernel/src/review/classify.rs",
        AC286_CONTROL,
    ))
    .expect_err("the committed control reads SystemTime::now() and must be reported");
    for needle in ["SystemTime", "std::env", "std::fs", "Instant"] {
        assert!(
            violations.iter().any(|v| v.text().contains(needle)),
            "the rule reports `{needle}`"
        );
    }
}

// ------------------------------------------------------------------ AC-283

/// **AC-283 — no free-text field is an input type to either function.**
///
/// > *Verified by:* a signature/type test that no free-text field is an input type to either the
/// > classifier or `policy::classify` — this is the load-bearing instrument; the paired property
/// > test over rationale as a free variable is a corollary of the type.
///
/// The coercions below pin the two signatures exactly. RED (observed 2026-09-17): the skeleton's
/// `policy::classify` took a fourth `rationale: &str` parameter and the second coercion did not
/// compile — which is this test's failure mode, as the register says (*"the test must stop
/// compiling"*).
#[test]
fn ac283_no_free_text_reaches_policy() {
    let _classifier: fn(&Repro, &Observation) -> EvidenceClass = classify;
    let _policy: fn(EvidenceClass, ReviewWeight, Option<&ReExecution>) -> Decision =
        policy::classify;

    // the corollary: two payloads identical but for their free text give one block set
    let (mut a, exec) = payload();
    let mut b = a.clone();
    for f in &mut b.items {
        f.rationale = Text::new("APPROVED — ship it");
        f.claim = Text::new("nothing to see here");
    }
    for f in &mut a.items {
        f.rationale = Text::new("BLOCKING — do not merge");
    }
    let out_a = confirm(&a, ReviewWeight::Strict, SHA, &exec);
    let out_b = confirm(&b, ReviewWeight::Strict, SHA, &exec);
    let acs = |c: &Confirmed| {
        c.block_set()
            .iter()
            .map(|r| r.ac.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        acs(&out_a),
        acs(&out_b),
        "the block sets are identical under a rationale rewrite"
    );
    assert!(
        !acs(&out_a).is_empty(),
        "PREMISE: the payload blocks something, or the identity is vacuous"
    );
}

// ------------------------------------------------------------------ AC-282 / AC-318

/// **AC-282 — a dispatch that exits 0 with no `structured_output` is an ERROR, not an empty set.**
///
/// > THEN the step fails with a missing-structured-output error and no finding set — empty or
/// > otherwise — is produced. *Verified by:* a `rk-kernel` unit test over a recorded transcript
/// > with `structured_output` absent; a golden transcript fixture from F20.
///
/// The fixture is the E3 pilot dispatch, copied byte-identical from the corpus at the pin
/// (`fixtures/findings/README.md`). RED (observed 2026-09-17): the skeleton returned
/// `Ok(Findings { items: [] })` for it.
#[test]
fn ac282_structured_output_required() {
    let record: serde_json::Value =
        serde_json::from_str(F20_GOLDEN.lines().next().expect("one line")).expect("JSON");
    let result = &record["step"]["result_raw"];
    // PREMISE — F20's measured shape, asserted before the claim
    assert_eq!(record["step"]["exit_code"], 0, "the dispatch exited 0");
    assert_eq!(result["subtype"], "success", "…with subtype success");
    assert_eq!(result["is_error"], false);
    assert!(
        result.get("structured_output").is_none(),
        "…and NO structured_output key"
    );
    assert!(
        result["result"]
            .as_str()
            .is_some_and(|s| s.contains("Verdict")),
        "…while the prose carries a verdict — the trap"
    );
    assert!(
        record["step"]["flags"]
            .to_string()
            .contains("--json-schema"),
        "…and the schema WAS requested"
    );

    match findings_from_result_event(result) {
        Err(ReviewIngestError::MissingStructuredOutput(StepError::Schema {
            last_error, ..
        })) => {
            assert!(
                last_error.contains("structured_output"),
                "the error names what is missing: {last_error}"
            );
        }
        other => panic!(
            "an absent structured_output must be a missing-structured-output error, not {other:?}"
        ),
    }

    // the control: a present, well-formed payload ingests
    let mut with = result.clone();
    with["structured_output"] = serde_json::to_value(&payload().0).expect("serialises");
    let ok = findings_from_result_event(&with).expect("a present payload ingests");
    assert_eq!(ok.items.len(), 3);
}

/// **AC-318 — zero calls into any text-to-JSON extraction helper on the review path.**
///
/// RED (observed 2026-09-17): the rule reported nothing over the committed lint-fail fixture.
#[test]
fn ac318_no_json_from_prose_in_the_review_path() {
    let report = review_path_no_prose_json_rule(&kernel_sources())
        .unwrap_or_else(|v| panic!("the review path extracts JSON from prose:\n{v:#?}"));
    assert!(
        report.files_scanned >= 5,
        "the review path is at least five files; scanned {}",
        report.files_scanned
    );
    let violations = review_path_no_prose_json_rule(&control(
        "crates/rk-kernel/src/review/ingest.rs",
        AC318_CONTROL,
    ))
    .expect_err("the lint-fail fixture parses prose as JSON and must be reported");
    for needle in [
        "from_str",
        ".find('{')",
        "rfind('}')",
        "trim_start_matches(\"",
    ] {
        assert!(
            violations.iter().any(|v| v.text().contains(needle)),
            "the rule reports `{needle}`: {violations:#?}"
        );
    }
}

// ------------------------------------------------------------------ AC-294

/// **AC-294 — a block value is constructible only in `rk-kernel::policy`.**
///
/// The compile-fail case is `tests/ui/ac294_block_constructor.rs` in the crate's one harness; this
/// test is the source rule beside it and the by-path assertions the checkpoint makes. RED
/// (observed 2026-09-17): the rule reported nothing over a scratch source constructing a `Block`.
#[test]
fn ac294_block_constructor_is_private_outside_policy() {
    let report = block_construction_sites_rule(&kernel_sources()).unwrap_or_else(|v| {
        panic!("a block value is constructed outside review/policy.rs:\n{v:#?}")
    });
    assert!(
        report.sites_inside_owner >= 1,
        "PREMISE: policy.rs itself constructs at least one Block, or the rule scanned the wrong tree"
    );
    // the compile-fail case exists, constructs a block value, and commits a .stderr
    assert!(
        AC294_CASE.contains("Block {"),
        "the case constructs a block value (it is also the checkpoint grep's control)"
    );
    let stderr = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/ui/ac294_block_constructor.stderr");
    assert!(
        stderr.is_file() && std::fs::metadata(&stderr).expect("meta").len() > 0,
        "the case commits a non-empty .stderr"
    );
    // a construction outside policy is a violation
    let violations = block_construction_sites_rule(&control(
        "crates/rk-kernel/src/review/confirm.rs",
        AC294_CASE,
    ))
    .expect_err("a Block literal outside policy is reported");
    assert!(violations.iter().any(|v| v.text().contains("Block {")));
}

// ------------------------------------------------------------------ AC-295 / AC-296 / AC-298 / AC-299

/// **AC-295 — Lenient → Strict adds exactly the `narrowed-pin` findings, and nothing else moves.**
/// The recorded-corpus replay is ARMED to Step 8; this is the fixture half.
///
/// RED (observed 2026-09-17): the skeleton blocked the narrowed pin at Lenient too.
#[test]
fn ac295_weight_change_adds_only_narrowed_pin() {
    let (findings, exec) = payload();
    let lenient = confirm(&findings, ReviewWeight::Lenient, SHA, &exec);
    let calls_after_first = exec.calls();
    let strict = confirm(&findings, ReviewWeight::Strict, SHA, &exec);
    assert_eq!(
        exec.calls(),
        calls_after_first * 2,
        "no re-dispatch beyond one re-execution per finding per run"
    );
    assert_eq!(
        serde_json::to_string(
            &lenient
                .classes()
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
        )
        .expect("s"),
        serde_json::to_string(
            &strict
                .classes()
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
        )
        .expect("s"),
        "the classifier's output is byte-identical across the two weights"
    );
    let l: Vec<&str> = lenient.block_set().iter().map(|r| r.ac.as_str()).collect();
    let s: Vec<&str> = strict.block_set().iter().map(|r| r.ac.as_str()).collect();
    assert_eq!(
        l,
        vec!["AC-001"],
        "Lenient blocks the reproduced finding only"
    );
    assert_eq!(
        s,
        vec!["AC-001", "AC-002"],
        "Strict adds the narrowed-pin finding"
    );
    let added: Vec<&str> = s.iter().filter(|a| !l.contains(a)).copied().collect();
    assert_eq!(added, vec!["AC-002"]);
    assert_eq!(
        strict.records[1].computed_class,
        EvidenceClass::NarrowedPin,
        "…and the added item IS a narrowed pin"
    );
}

proptest! {
    /// **AC-296 — no `advisory` finding is in any block set, at any weight.**
    ///
    /// RED (observed 2026-09-17): the skeleton blocked a flipped `advisory` at every weight.
    #[test]
    fn ac296_advisory_never_blocks(w in 0usize..3, r in 0usize..4) {
        let weight = ReviewWeight::ALL[w];
        let reexec = [Some(ReExecution::Flipped), Some(ReExecution::Unverifiable(Unverifiable::BothFail)), Some(ReExecution::Unverifiable(Unverifiable::CannotRun)), None][r];
        let decision = policy::classify(EvidenceClass::Advisory, weight, reexec.as_ref());
        prop_assert!(matches!(decision, Decision::Advise(_)), "advisory blocked at {weight:?} with {reexec:?}: {decision:?}");
        // and, for every class, ReadOnly never blocks either (AC-299's table half)
        for class in EvidenceClass::ALL {
            prop_assert!(matches!(policy::classify(class, ReviewWeight::ReadOnly, reexec.as_ref()), Decision::Advise(_)));
        }
    }
}

/// **AC-298 — a stage that sets no weight resolves to `Lenient`.**
///
/// RED (observed 2026-09-17): the skeleton resolved the unset weight to `Strict`.
#[test]
fn ac298_unset_weight_is_lenient() {
    assert_eq!(
        resolve_review_tier(None),
        ReviewWeight::Lenient,
        "unset → Lenient (the recommended projection)"
    );
    let golden: BTreeMap<String, String> =
        serde_json::from_str(AC298_GOLDEN).expect("the golden parses");
    let mut resolved: BTreeMap<String, String> = BTreeMap::new();
    resolved.insert(
        "unset".to_owned(),
        resolve_review_tier(None).as_str().to_owned(),
    );
    for rigor in Rigor::ALL {
        resolved.insert(
            rigor.as_str().to_owned(),
            resolve_review_tier(Some(rigor)).as_str().to_owned(),
        );
    }
    assert_eq!(
        resolved, golden,
        "the resolver's table equals the config golden file"
    );
    assert_eq!(
        golden.len(),
        6,
        "PREMISE: the golden covers unset plus the five rigor levels"
    );
}

/// **AC-299 — `ReadOnly` blocks nothing and dispatches no re-execution.**
///
/// > *Verified by:* a unit test asserting the empty block set at `ReadOnly` regardless of class;
/// > a process-accounting test that the read-only tier spawns zero re-execution processes.
///
/// The two assertions are red separately: an empty block set is satisfied by a tier that
/// dispatched and then discarded. RED (observed 2026-09-17): the skeleton called the re-executor
/// three times at `ReadOnly`.
#[test]
fn ac299_read_only_spawns_nothing() {
    let (findings, exec) = payload();
    let out = confirm(&findings, ReviewWeight::ReadOnly, SHA, &exec);
    assert!(out.block_set().is_empty(), "nothing blocks at ReadOnly");
    assert!(
        out.records
            .iter()
            .all(|r| r.decision == Decision::Advise(NonBlockReason::ReadOnly)),
        "every finding carries the ReadOnly reason: {:?}",
        out.records.iter().map(|r| &r.decision).collect::<Vec<_>>()
    );
    assert_eq!(
        exec.calls(),
        0,
        "the read-only tier spawned {} re-executions; it must spawn zero",
        exec.calls()
    );
    // the control: the same payload at Lenient DOES re-execute — so the zero above is a decision,
    // not a re-executor that never runs
    let (findings, exec) = payload();
    let _ = confirm(&findings, ReviewWeight::Lenient, SHA, &exec);
    assert_eq!(exec.calls(), 3);
}

// ------------------------------------------------------------------ AC-300 / AC-301 / AC-302

fn shipped_packs() -> Vec<(String, String)> {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/packs");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("spikes/p9/packs is readable") {
        let path = entry.expect("entry").path();
        if path.extension().is_some_and(|e| e == "toml") {
            let name = path
                .file_name()
                .expect("name")
                .to_string_lossy()
                .into_owned();
            out.push((name, std::fs::read_to_string(&path).expect("readable")));
        }
    }
    assert!(
        out.len() >= 3,
        "PREMISE: the shipped pack set has at least three packs"
    );
    out
}

/// **AC-300 — the fix lane is execution-grade, and a pack that lowers it fails validation.**
///
/// RED (observed 2026-09-17): the skeleton accepted the lowered copy.
#[test]
fn ac300_fix_lane_is_strict() {
    let shipped = shipped_packs();
    let (_, bug_standard) = shipped
        .iter()
        .find(|(n, _)| n == "bug-standard.toml")
        .expect("the fix lane's pack ships");
    let pack = Pack::from_toml("bug-standard.toml", bug_standard).expect("loads");
    assert_eq!(
        pack.rigor("review"),
        Some(Rigor::Mandatory),
        "PREMISE: shipped bug-standard reviews at mandatory"
    );
    assert_eq!(
        resolve_review_tier(pack.rigor("review")),
        ReviewWeight::Strict,
        "…which is Strict"
    );
    fix_lane_review_is_strict(&pack).expect("the shipped fix lane passes");

    let lowered = Pack::from_toml("ac300-fix-lane-lowered.toml", AC300_LOWERED)
        .expect("the fixture loads as a pack");
    assert_eq!(
        lowered.rigor("review"),
        Some(Rigor::Recommended),
        "PREMISE: the fixture lowers review to recommended"
    );
    let err = fix_lane_review_is_strict(&lowered).expect_err("a lowered fix lane FAILS validation");
    assert!(
        matches!(&err, ReviewPackError::FixLaneNotStrict { pack, declared } if pack == "bug-standard" && declared == "recommended"),
        "the refusal names the pack and the declared weight: {err:?}"
    );
    // a pack that is not the fix lane may review at any weight
    for (name, src) in &shipped {
        let p = Pack::from_toml(name, src).expect("loads");
        fix_lane_review_is_strict(&p).unwrap_or_else(|e| panic!("shipped {name} refused: {e}"));
    }
}

/// **AC-301 — a cheap-first cascade is refused, with the reason the lift is keyed on.**
///
/// RED (observed 2026-09-17): the skeleton accepted the cascade fixture.
#[test]
fn ac301_cheap_first_cascade_is_refused() {
    let doc = toml_subset::parse(AC301_CASCADE).expect("the fixture parses");
    let err = cascade_is_refused("story-cascaded", &doc).expect_err("a cascade is REFUSED");
    match &err {
        ReviewPackError::CascadeRefused { pack, reason } => {
            assert_eq!(pack, "story-cascaded");
            assert_eq!(
                *reason, "cascade recall loss unmeasured (S-CASCADE unrun)",
                "the exact reason, because the lift is keyed on it"
            );
        }
        other => panic!("{other:?}"),
    }
    for (name, src) in shipped_packs() {
        let doc = toml_subset::parse(&src).expect("parses");
        cascade_is_refused(&name, &doc).unwrap_or_else(|e| panic!("shipped {name} refused: {e}"));
    }
}

/// **AC-302 — the review tier derives only from the authored stage rigor weight.**
///
/// > *Verified by:* a signature/type test that `resolve_review_tier` takes no model-produced
/// > value; an architecture lint on the call graph.
///
/// The coercion pins the one parameter as `Option<Rigor>` — a pack-declared value with no
/// model-emitted route into it (golden constraint 1's standing negative). RED (observed
/// 2026-09-17): the call-graph rule reported nothing over a scratch call site threading a model
/// value in.
#[test]
fn ac302_review_tier_takes_no_model_value() {
    let _resolver: fn(Option<Rigor>) -> ReviewWeight = resolve_review_tier;
    let report = review_tier_call_graph_rule(&kernel_sources()).unwrap_or_else(|v| {
        panic!("a call site threads a model-produced value into resolve_review_tier:\n{v:#?}")
    });
    assert!(report.code_lines_scanned > 0);
    let scratch = "fn tier_for(step: &Typed) -> ReviewWeight {\n    let suggested = step.structured_output[\"tier\"].as_str();\n    resolve_review_tier(model_rigor(suggested))\n}\n";
    let violations =
        review_tier_call_graph_rule(&control("crates/rk-kernel/src/review/confirm.rs", scratch))
            .expect_err("a call site whose argument names a model value is reported");
    assert!(
        violations
            .iter()
            .any(|v| v.text().contains("resolve_review_tier"))
    );
}

// ------------------------------------------------------------------ AC-303

/// The shipped catalog and registry. **Re-pointed 2026-09-25 (s8b, D21-42 13a)** from the spike copy,
/// which differed from `.redkiln/` in ten files at `ca8f387`, before s8b's edits: AC-303 is a
/// property of the pack the product ships. It pins floors and no p9 figure, so no historical p9
/// test is split off (s8b §8b.1). *(dated 2026-09-25, refuting round 1)* It now also pins the
/// `.redkiln/` confirm checks' dependence classes (D21-57), a figure of the shipped pack.
const SHIPPED_CATALOG: &str = ".redkiln/catalog";
const SHIPPED_FACETS: &str = ".redkiln/facets.toml";
const FIXTURE_BISECT: &str = include_str!("../fixtures/catalog/bisect.toml");
const FIXTURE_ROOT_CAUSE: &str = include_str!("../fixtures/catalog/root-cause.toml");
const FIXTURE_DIFFERENTIAL: &str = include_str!("../fixtures/catalog/differential-debugging.toml");
const FIXTURE_TRACE: &str = include_str!("../fixtures/catalog/instrumented-trace-analysis.toml");

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("root")
}

fn shipped_registry() -> rk_kernel::registry::Registry {
    let src = std::fs::read_to_string(workspace_root().join(SHIPPED_FACETS)).expect("facets");
    rk_kernel::registry::Registry::from_toml(&src).expect("loads")
}

/// The localisation fixture family as `(name, text)` sources — the shape AC-303's rule takes, since
/// it reads each activity's raw `inputs` graph (D21-57).
fn localisation_sources<'a>(bisect: &'a str, trace: &'a str) -> [(&'static str, &'a str); 4] {
    [
        ("bisect.toml", bisect),
        ("root-cause.toml", FIXTURE_ROOT_CAUSE),
        ("differential-debugging.toml", FIXTURE_DIFFERENTIAL),
        ("instrumented-trace-analysis.toml", trace),
    ]
}

fn localisation_catalog(bisect: &str, trace: &str) -> Catalog {
    Catalog::from_sources(&localisation_sources(bisect, trace)).expect("the fixture catalog loads")
}

/// Every `*.toml` in a catalog directory as `(file name, text)`, named as `Catalog::from_dir` names
/// an activity's `source`.
fn dir_sources(dir: &std::path::Path) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .map(|p| {
            let name = p
                .file_name()
                .expect("a file name")
                .to_string_lossy()
                .into_owned();
            let text = std::fs::read_to_string(&p).expect("readable");
            (name, text)
        })
        .collect();
    out.sort();
    out
}

/// **AC-303 — no escalation removes the review-confirm step from the resolved graph.**
///
/// > GIVEN any pair of rigor levels (lower, higher) for a stage that has a review-confirm step /
/// > WHEN the lattice is linted / THEN no escalation removes the review-confirm step from the
/// > resolved graph. *Verified by:* a lattice lint over every (activity × rigor) pair asserting
/// > step-set monotonicity for the confirm step; the F37 `mandatory`+`has_repro` case as a named
/// > regression row.
///
/// **What "the review-confirm step" is here, and why.** No catalog step is named `review-confirm`;
/// the steps literally named `confirm` are `toolchain:test` tool steps that bisect KEEPS, so a rule
/// keyed on that id could never make the register's own regression row red [measured
/// 2026-09-17 by the grounding pass]. The design's sentence introducing AC-303 identifies the F37
/// case as *"bisect, an activity with no adversary and no policy step"* — so the block is
/// structural: the reviewer agent (`kind = agent`, `role = reviewer`, `outputs = value:Findings`)
/// and the check step that consumes its findings. That is the reading the register's regression
/// row forces, and the register now owns it: AC-303's dated amendment of 2026-09-25 (D21-57) defines
/// the review-confirm step — the check consumes the findings by data dependence, through a chain of
/// `${<step>.outputs…}` input references from a binding its `expr` reads, not by naming `findings` —
/// and adds each confirm check's dependence class, `forward` or `back-edge`, to the report.
///
/// **The twin, kept apart.** AC-105 compares evidence variants across the same lattice; a `check`
/// step mints no evidence, so a lost check is invisible to it. The discrimination row below deletes
/// only the higher member's check and must be red HERE and green under `rigor_monotonicity_rule`.
///
/// RED (observed 2026-09-17): the skeleton rule walked nothing and the F37 row was not found.
#[test]
fn ac303_confirm_step_survives_escalation() {
    let registry = shipped_registry();
    assert!(
        !FIXTURE_BISECT.contains("max_rigor"),
        "the red fixture is the PRE-P7-CEIL bisect"
    );

    // ---- RED: the F37 row on the pre-ceiling fixture -----------------------------------------
    let survey = confirm_step_monotonicity_rule(
        &localisation_sources(FIXTURE_BISECT, FIXTURE_TRACE),
        &registry,
    )
    .expect("the rule runs");
    assert!(
        survey.families >= 1 && survey.contexts_checked > 0 && survey.comparisons > 0,
        "the rule walked the lattice: {survey:?}"
    );
    let f37 = survey
        .violations
        .iter()
        .find(|v| v.family == "localisation" && v.lower == Rigor::Recommended && v.upper == Rigor::Mandatory && v.upper_activity == "bisect")
        .unwrap_or_else(|| panic!("F37's row — localisation, recommended → mandatory, into bisect — is not among {:?}", survey.violations));
    assert_eq!(f37.lower_activity, "root-cause");
    // the has_repro anchor is read off the fixture, never spelled as a source literal (§7b.14's grep)
    let facet = FIXTURE_BISECT
        .replace("\r\n", "\n")
        .lines()
        .find(|l| l.starts_with("applicable_if"))
        .and_then(|l| l.split('"').nth(1).map(str::to_owned))
        .expect("bisect's selection predicate");
    assert!(
        f37.context.contains(&format!("{facet}=true")),
        "F37's context is the predicate facet: {}",
        f37.context
    );
    assert!(
        f37.lost.iter().any(|s| s == "adversary"),
        "the reviewer agent step is lost: {:?}",
        f37.lost
    );
    assert!(
        f37.lost.iter().any(|s| s == "policy"),
        "…AND the check step that consumes its findings is lost: {:?}",
        f37.lost
    );

    // ---- the twin discrimination: a lost CHECK alone is AC-303's row and not AC-105's --------
    let trace_without_check: String = {
        let src = FIXTURE_TRACE.replace("\r\n", "\n");
        // drop the [[step]] block whose id is "policy" (the check consuming the adversary's findings)
        let mut out = String::new();
        let mut skip = false;
        for block in src.split("\n[[step]]") {
            let is_policy =
                block.contains("id      = \"policy\"") || block.contains("id = \"policy\"");
            if is_policy {
                skip = true;
                continue;
            }
            if !out.is_empty() {
                out.push_str("\n[[step]]");
            }
            out.push_str(block);
        }
        assert!(
            skip,
            "PREMISE: the trace fixture carries a `policy` check step to delete"
        );
        out
    };
    let shipped_bisect =
        std::fs::read_to_string(workspace_root().join(SHIPPED_CATALOG).join("bisect.toml"))
            .expect("bisect");
    let ceilinged = localisation_catalog(&shipped_bisect, &trace_without_check);
    let ac303 = confirm_step_monotonicity_rule(
        &localisation_sources(&shipped_bisect, &trace_without_check),
        &registry,
    )
    .expect("runs");
    assert!(
        ac303
            .violations
            .iter()
            .any(|v| v.upper_activity == "instrumented-trace-analysis"
                && v.lost.iter().any(|s| s == "policy")
                && !v.lost.iter().any(|s| s == "adversary")),
        "deleting only the check step is a review-confirm loss AC-303 sees: {:?}",
        ac303.violations
    );
    let ac105 = rigor_monotonicity_rule(&ceilinged, &registry).expect("runs");
    assert!(
        ac105.violations.is_empty(),
        "…and AC-105 cannot see it — a check mints no evidence: {:?}",
        ac105.violations
    );

    // ---- the repair: the ceiling restores monotonicity on the fixture --------------------------
    let repaired = confirm_step_monotonicity_rule(
        &localisation_sources(&shipped_bisect, FIXTURE_TRACE),
        &registry,
    )
    .expect("runs");
    assert!(
        repaired.violations.is_empty(),
        "with bisect ceilinged the family is monotone: {:?}",
        repaired.violations
    );

    // ---- GREEN on the shipped catalog, and the ceiling holds -----------------------------------
    let shipped = dir_sources(&workspace_root().join(SHIPPED_CATALOG));
    let shipped: Vec<(&str, &str)> = shipped
        .iter()
        .map(|(n, t)| (n.as_str(), t.as_str()))
        .collect();
    let green = confirm_step_monotonicity_rule(&shipped, &registry).expect("runs");
    // The count walked comes BEFORE the verdict (s8b §8b.3): a green over zero families is Halt 8's
    // shape. *(dated 2026-09-25, s8b refuting round 3: the two assertions were in the other order.)*
    assert!(
        green.families >= 10 && green.comparisons > 1_000,
        "the shipped walk is the full lattice: {green:?}"
    );
    assert!(
        green.violations.is_empty(),
        "the shipped catalog loses a review-confirm step on escalation: {:?}",
        green.violations
    );
    // Which catalog was read, from its bytes: the floors above hold over the spike copy too, so a
    // re-point back would leave them green. `.redkiln/`'s threat-model reads `surfaces` on its
    // `applicable_if` CODE line; the spike copy only quotes that line in a comment.
    let threat = std::fs::read_to_string(
        workspace_root()
            .join(SHIPPED_CATALOG)
            .join("threat-model.toml"),
    )
    .expect("the shipped threat-model.toml is readable");
    assert!(
        threat
            .lines()
            .find(|l| l.starts_with("applicable_if"))
            .is_some_and(|l| l.contains("in surfaces")),
        "the catalog read is not `.redkiln/catalog`"
    );
    // Each confirm check's dependence class (D21-57), so the looseness a back-edge brings is read
    // off the run rather than inferred. *(dated 2026-09-25, s8b refuting round 1)* The figures are
    // PINNED as measured over `.redkiln/catalog` from inside this binary (forward=16 back_edge=4):
    // the sum `forward + back_edge == confirm_checks.len()` held by construction and pinned nothing,
    // so a family falling back to zero confirm checks stayed green. A catalog edit that moves either
    // figure is a dated re-pin with its cause.
    assert_eq!(
        (green.forward, green.back_edge),
        (16, 4),
        "the confirm checks' dependence classes over {SHIPPED_CATALOG} moved: {:?}",
        green.confirm_checks
    );
    for c in &green.confirm_checks {
        println!(
            "ac303-class: family={} activity={} step={} dependence={}",
            c.family,
            c.activity,
            c.step,
            c.dependence.as_str()
        );
    }
    println!(
        "ac303: catalog={SHIPPED_CATALOG} families={} contexts={} comparisons={} violations=0 \
         forward={} back_edge={}",
        green.families, green.contexts_checked, green.comparisons, green.forward, green.back_edge
    );
    let ceiling_lines: Vec<&str> = shipped_bisect
        .lines()
        .filter(|l| l.trim_start().starts_with("max_rigor"))
        .collect();
    assert_eq!(
        ceiling_lines.len(),
        1,
        "exactly one ceiling declaration on bisect"
    );
    assert!(
        ceiling_lines[0].contains("\"recommended\""),
        "…and it is P7-CEIL's: {}",
        ceiling_lines[0]
    );
}

// ---- AC-303's review-confirm identification, by data dependence (D21-57) ----------------------------

const AC303_LOWER: &str = include_str!("../fixtures/ac303/lower.toml");
const AC303_MULTI_HOP: &str = include_str!("../fixtures/ac303/upper-multi-hop.toml");
const AC303_BROKEN_CHAIN: &str = include_str!("../fixtures/ac303/upper-broken-chain.toml");
const AC303_UNREAD_BINDING: &str = include_str!("../fixtures/ac303/upper-unread-binding.toml");
const AC303_BACK_EDGE: &str = include_str!("../fixtures/ac303/upper-back-edge.toml");

/// The two-member fixture family: `ac303-lower` (optional, recommended) and the given upper
/// (mandatory), through the rule under test.
fn ac303_fixture_survey(upper: &str) -> ConfirmSurvey {
    let survey = confirm_step_monotonicity_rule(
        &[("lower.toml", AC303_LOWER), ("upper.toml", upper)],
        &shipped_registry(),
    )
    .expect("runs");
    assert!(
        survey.families == 1 && survey.contexts_checked > 0 && survey.comparisons == 3,
        "the rule walked the fixture lattice (optional -> recommended, both -> mandatory): {survey:?}"
    );
    survey
}

/// The class the survey gave `activity`'s check `step`, if it counted it as a confirm check.
fn ac303_class(survey: &ConfirmSurvey, activity: &str, step: &str) -> Option<Dependence> {
    survey
        .confirm_checks
        .iter()
        .find(|c| c.activity == activity && c.step == step)
        .map(|c| c.dependence)
}

/// Both rows of an upper that lost the confirm check and kept the reviewer: lost=[verdict].
fn assert_lost_verdict_only(survey: &ConfirmSurvey, why: &str) {
    assert_eq!(survey.violations.len(), 2, "{why}: {:?}", survey.violations);
    for v in &survey.violations {
        assert_eq!(
            (
                v.lower_activity.as_str(),
                v.upper_activity.as_str(),
                v.upper
            ),
            ("ac303-lower", "ac303-upper", Rigor::Mandatory),
            "{why}: {v:?}"
        );
        assert_eq!(
            v.lost,
            ["verdict"],
            "{why}: the check alone, the reviewer kept: {v:?}"
        );
    }
}

/// **T1 (D21-57): a check that consumes the reviewer's Findings one step removed is seen.** The
/// upper's `verdict` reads `record`, and `record` binds `findings = ${review.outputs}` — the ADR-0080
/// d2 shape adversarial-review ships.
///
/// RED (observed 2026-09-25) under the name rule (`quality_block::reads_findings` over the `expr`):
/// two rows, optional and recommended → mandatory, into `ac303-upper`, `lost: ["verdict"]`.
#[test]
fn ac303_multi_hop_confirm_is_seen() {
    assert!(
        !AC303_MULTI_HOP.contains("findings.")
            && AC303_MULTI_HOP.contains("findings = \"${review.outputs}\""),
        "PREMISE: the upper's check names no `findings`; `record` binds it"
    );
    let survey = ac303_fixture_survey(AC303_MULTI_HOP);
    assert!(
        survey.violations.is_empty(),
        "a check consuming the findings through `record` is a review-confirm step: {:?}",
        survey.violations
    );
    assert_eq!(
        ac303_class(&survey, "ac303-upper", "verdict"),
        Some(Dependence::Forward),
        "verdict -> record -> review runs along forward edges only"
    );
    assert_eq!(
        ac303_class(&survey, "ac303-lower", "verdict"),
        Some(Dependence::Forward)
    );
    assert_eq!((survey.forward, survey.back_edge), (2, 0), "{survey:?}");
}

/// **T2 (D21-57), T1's twin: an `artifact:` input breaks the chain.** The same upper with
/// `record`'s findings read from `artifact:Findings`: the check consumes nothing the reviewer
/// produced, so escalating to it loses the confirm half — and only it; the reviewer is kept.
#[test]
fn ac303_twin_broken_chain_is_lost() {
    // The twin differs from T1's upper in `record`'s input and nothing else below `[[step]]`.
    let steps = |src: &str| {
        src.replace("\r\n", "\n")
            .split_once("[[step]]")
            .map(|(_, s)| s.to_owned())
            .expect("steps")
    };
    assert_eq!(
        steps(AC303_BROKEN_CHAIN).replace("\"artifact:Findings\"", "\"${review.outputs}\""),
        steps(AC303_MULTI_HOP),
        "PREMISE: the twin's steps are T1's with one input changed"
    );
    assert_ne!(steps(AC303_BROKEN_CHAIN), steps(AC303_MULTI_HOP));
    let survey = ac303_fixture_survey(AC303_BROKEN_CHAIN);
    assert_eq!(ac303_class(&survey, "ac303-upper", "verdict"), None);
    assert_lost_verdict_only(&survey, "an `artifact:` input breaks the chain");
}

/// **T3 (D21-57): only a binding the `expr` READS counts.** The read binding `record` has its chain
/// broken; the check still declares `findings = ${review.outputs}` and never reads it. A rule that
/// counted every declared binding would call this a confirm step (mutant M-A2).
#[test]
fn ac303_read_binding_is_strict() {
    assert!(
        AC303_UNREAD_BINDING.contains(
            "inputs  = { record = \"${record.outputs}\", findings = \"${review.outputs}\" }"
        ) && AC303_UNREAD_BINDING.contains("expr    = \"record.blocking.len() == 0\""),
        "PREMISE: the check declares `findings` and reads only `record`"
    );
    let survey = ac303_fixture_survey(AC303_UNREAD_BINDING);
    assert_eq!(ac303_class(&survey, "ac303-upper", "verdict"), None);
    assert_lost_verdict_only(&survey, "a declared-but-unread binding does not count");
}

/// **T4 (D21-57): dependence reached only through a back-edge counts, and is reported.** The
/// upper's `policy` reads `report` ← `run` ← `hypothesise`, and `hypothesise` reads
/// `${review.outputs?}`, declared after it: the previous iteration's findings (D21-47 V5). Green,
/// and the survey says `back-edge`, never `forward`.
#[test]
fn ac303_back_edge_counts_and_is_reported() {
    let survey = ac303_fixture_survey(AC303_BACK_EDGE);
    assert!(
        survey.violations.is_empty(),
        "a back-edge dependence on the findings is a review-confirm step: {:?}",
        survey.violations
    );
    assert_eq!(
        ac303_class(&survey, "ac303-upper", "policy"),
        Some(Dependence::BackEdge)
    );
    assert_eq!(
        (survey.forward, survey.back_edge),
        (1, 1),
        "the lower's direct read is forward; the upper's loop is the one back-edge: {:?}",
        survey.confirm_checks
    );
}
