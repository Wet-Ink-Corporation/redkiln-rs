//! The review pass — `Exit::FindingsBlock`'s producer (ADR-0162; `s7h` §7h.3's call site).
//!
//! 🛑 **What this module does NOT assert, said here so nobody reads it as more than it is.** These
//! are the pass's PURE halves: the journal read, the selection rule, and the re-executor's mapping
//! onto AC-288's four cells. They do not run the binary, and a green module here is **not**
//! evidence that a real `redkiln run` ever returns `4` — Checkpoint 7h guard (10)'s behavioural
//! form is what asserts that, and this programme has twice shipped a source-level green over code
//! no run reached (both `s7g` lints; ADR-0158's three writerless tables).
//!
//! **ADR-0101 clause 5 bounds what any of this may be read as.** S-SUBTLE-CB is registered and
//! unrun; nothing here claims anything about the gate's calibration.

use rk_cli::review_pass::{
    FINDINGS_SCHEMA, PassSkipped, REVIEW_VERDICT_KIND, ToolchainReExecutor, findings_in,
    is_review_activity, seal_verdict, selection_of,
};
use rk_git::trailer::grammar::Verdict;
use rk_kernel::catalog::{Activity, Catalog};
use rk_kernel::review::reexec::{ReExecution, ReExecutor, Unverifiable};
use rk_ports::toolchain::{
    ChangeSummary, Classification, Selection, StepBound, TestOutcome, TestRunner, Toolchain,
    VerbCall, VerbOutput,
};
use rk_types::artifact::findings::Findings;
use rk_types::artifact::findings::{EvidenceLabel, Finding};
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::{Expected, Repro, ReproKind};
use rk_types::text::Text;

/// How one arm's `test` answers (s8c §8c.2, record T = ADR-0169, proposed). *(Amended 2026-09-26,
/// s8c: this was a `green: bool`, with red as `Err(NonZero)` under RT-22's *"`Ok(count)` only on a
/// green suite"*. RT-22 is reversed: **red is no longer `Err`**, so the fake has three modes.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arm {
    /// `Ok`, every selected test passed.
    Green,
    /// `Ok`, one selected test failed: a red suite.
    Red,
    /// `Err(Transport { Spawn })`: the program cannot start (AC-344's cannot-run).
    CannotRun,
    /// `Ok` with nothing executed, `status: Empty` (the filter matched no test). *(dated
    /// 2026-09-26, s8c refuting round 4: without it the re-executor's EMPTY-arm guard was
    /// unobservable, since every other mode executes one test per selected name.)*
    Empty,
}

/// A toolchain whose `test` answer is scripted per arm. Every other method refuses: this fake
/// stands in for exactly one capability and says so by failing on the rest.
struct ScriptedToolchain {
    arm: Arm,
}

impl ScriptedToolchain {
    const fn green(green: bool) -> Self {
        Self {
            arm: if green { Arm::Green } else { Arm::Red },
        }
    }
}

impl Toolchain for ScriptedToolchain {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self, _bound: StepBound) -> Result<rk_types::verb_output::BuildReport, StepError> {
        Err(refusal("build"))
    }
    /// Record T's contract: `Ok` with counts, green OR red; `Err` only when the suite cannot run.
    fn test(&self, selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        let Selection::Filter(args) = selection else {
            return Err(refusal("a selection other than a filter"));
        };
        let executed = if self.arm == Arm::Empty {
            0
        } else {
            u32::try_from(args.len()).unwrap_or(0)
        };
        let failed = match self.arm {
            Arm::Green | Arm::Empty => 0,
            Arm::Red => 1.min(executed),
            Arm::CannotRun => {
                return Err(StepError::Transport {
                    kind: TransportKind::Spawn,
                    detail: "the scripted arm cannot start its program".to_owned(),
                });
            }
        };
        TestOutcome::new(
            rk_types::verify::VerifyReport {
                basis: rk_types::verify::Basis::WholeSuite,
                argv: vec!["scripted".to_owned(), "test".to_owned()],
                selected_test_count: executed,
                total_test_count: executed,
                total_test_count_source: rk_types::verify::TotalTestCountSource::OwnExecutedCount,
                wall_secs: 0,
                toolchain_version: "scripted".to_owned(),
                suite_sha: "scripted".to_owned(),
                basis_reason: Some(rk_types::verify::BasisReason::ProbeUnverified),
            },
            executed - failed,
            failed,
            TestRunner::named("scripted", "0"),
        )
    }
    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        Err(refusal("lint"))
    }
    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        Err(refusal("classify"))
    }
    fn change_summary(&self, _range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        Err(refusal("change_summary"))
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        Err(refusal(call.verb))
    }
}

fn refusal(what: &str) -> StepError {
    StepError::Unsupported {
        capability: format!("the scripted fixture toolchain does not implement {what}"),
    }
}

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

fn finding(ac: &str, repro: Repro) -> Finding {
    Finding {
        ac: ac.to_owned(),
        claim: Text::new(format!("{ac}: the claim")),
        evidence: EvidenceLabel::Reproduced,
        rationale: Text::new("the rationale"),
        repro,
        files: vec![],
    }
}

/// A committed row of the shipped `adversarial-review`'s `review` step.
///
/// *(dated 2026-09-29, `s8g` build, package R-D2119-8g; D21-119 item 2.)* ~~`"activity_id_at_version":
/// "review@1"`~~ — the seal now resolves a row's activation against the catalog, so this helper's
/// rows name the shipped review activity at its shipped version, read off `.redkiln/catalog/`.
fn committed(schema: &str, result: serde_json::Value) -> Event {
    committed_by(&at_version(ADVERSARIAL_REVIEW), "review", schema, result)
}

/// A committed step row, as `activity::step_committed_event` writes one.
fn committed_by(
    activity_id_at_version: &str,
    step_id: &str,
    schema: &str,
    result: serde_json::Value,
) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": activity_id_at_version,
            "at": 0,
            "attempt": 1,
            "item": "an-item",
            "iteration_path": [],
            "result": result,
            "run_id": "review-0000000000000000",
            "schema": schema,
            "step_id": step_id,
        }),
    )
}

// ---------------------------------------------------------------- the shipped catalog (D21-119)

/// The shipped review activity both lanes select (D21-119 finding 2).
const ADVERSARIAL_REVIEW: &str = "adversarial-review";
/// The shipped review stage's name, as the shipped packs spell it. A TEST literal: `crates/rk-cli/
/// src/` compares no stage to a string (ADR-0162 decision 5).
const REVIEW: &str = "review";

fn workspace() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// The shipped catalog, composed by `run`'s own `catalog_sources` over this repository (no trial).
fn shipped() -> &'static Catalog {
    static CATALOG: std::sync::OnceLock<Catalog> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| {
        let sources = rk_cli::verbs::run::catalog_sources(&workspace(), None)
            .unwrap_or_else(|e| panic!("the shipped catalog's sources read: {e:?}"));
        let borrowed: Vec<(&str, &str)> = sources
            .iter()
            .map(|(n, s)| (n.as_str(), s.as_str()))
            .collect();
        Catalog::from_sources(&borrowed)
            .unwrap_or_else(|e| panic!("the shipped catalog loads: {e}"))
    })
}

fn shipped_activity(id: &str) -> &'static Activity {
    shipped()
        .activities()
        .iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("PREMISE: the shipped catalog carries `{id}`"))
}

/// `<id>@<version>` of the shipped activity `id`, as the committed row's key carries it.
fn at_version(id: &str) -> String {
    shipped_activity(id)
        .id_at_version()
        .unwrap_or_else(|| panic!("PREMISE: the shipped `{id}` declares a version"))
}

/// PREMISE: the shipped `id` sits at `stage`, provides `first` first, and its step `step`
/// commits under the `Findings` schema — the shape each D21-119 test is about.
fn premise(id: &str, stage: &str, first: &str, step: &str) {
    let a = shipped_activity(id);
    assert_eq!(a.stage, stage, "PREMISE: `{id}` sits at `{stage}`");
    assert_eq!(
        a.provides.first().map(String::as_str),
        Some(first),
        "PREMISE: `{id}` provides `{first}` first"
    );
    assert!(
        a.steps
            .iter()
            .any(|s| s.id == step && s.output_type.as_deref() == Some(FINDINGS_SCHEMA)),
        "PREMISE: `{id}`'s step `{step}` commits under `{FINDINGS_SCHEMA}`"
    );
}

/// The seal's read at `stage` over the shipped catalog.
fn read_at(events: &[Event], stage: &str) -> Result<Findings, PassSkipped> {
    findings_in(events, shipped(), stage)
}

/// The seal's read at the shipped review stage — what the pre-D21-119 tests above call.
fn read(events: &[Event]) -> Result<Findings, PassSkipped> {
    read_at(events, REVIEW)
}

fn one_finding_payload(ac: &str, command: &str) -> serde_json::Value {
    serde_json::json!({
        "items": [{
            "ac": ac,
            "claim": "the claim",
            "evidence": "reproduced",
            "rationale": "the rationale",
            "repro": {
                "kind": "run",
                "command": command,
                "expected": { "kind": "exit", "exit": 0 },
                "observed": "exit 101",
                "expected_when_met": "exit 0",
            },
        }]
    })
}

// ---------------------------------------------------------------- the selection rule

/// **The `--` rule, and its two refusals.** `Repro.command` is a shell string and `Toolchain::test`
/// takes a selection; no record maps one to the other, so the rule is explicit and narrow, and
/// what it cannot serve it refuses rather than guesses (ADR-0162 decision 4).
#[test]
fn selection_is_the_double_dash_tail_or_nothing() {
    assert_eq!(
        selection_of("cargo test -- ac601"),
        Some(vec!["ac601".to_owned()]),
        "the tokens after `--` are the selection"
    );
    assert_eq!(
        selection_of("cargo test -- ac601 ac602"),
        Some(vec!["ac601".to_owned(), "ac602".to_owned()]),
        "several filters are several selection entries"
    );
    assert_eq!(
        selection_of("cargo test"),
        None,
        "a command naming no selection is REFUSED, not run whole: a bare `cargo test` is the \
         entire suite, and attributing the entire suite to one finding is the attribution error \
         AC-373 exists for"
    );
    assert_eq!(
        selection_of("cargo test -- "),
        None,
        "an EMPTY selection is refused: `ac332_empty_selection_is_never_green` is the standing \
         finding that an empty suite is not a pass, and `Ok(0)` from it would read as green"
    );
}

// ---------------------------------------------------------------- the journal read

/// The finding set is read from the journal, never invented — and an absence is an ABSENCE.
#[test]
fn findings_come_from_the_journal_and_absence_is_not_an_empty_set() {
    assert_eq!(
        read(&[]),
        Err(PassSkipped::NoFindingsRow),
        "an empty journal is no review, not a review with nothing to say"
    );

    // A committed activation under ANOTHER schema is not a review: this is the path every
    // non-review stage takes, and it must not produce a verdict.
    let other = [committed("ImplReport", serde_json::json!({ "ok": true }))];
    assert_eq!(
        read(&other),
        Err(PassSkipped::NoFindingsRow),
        "a row under another schema is not a finding set"
    );

    let good = [committed(
        "Findings",
        one_finding_payload("AC-601", "cargo test -- ac601"),
    )];
    let findings = read(&good).expect("a well-formed payload ingests");
    assert_eq!(findings.items.len(), 1, "the one finding is read");
    assert_eq!(findings.items[0].ac, "AC-601");

    // 🛑 THE LAST ROW, NOT THE FIRST: a re-dispatched activation appends a second
    // `activity_committed`, and the verdict is about what the run FINALLY committed.
    let twice = [
        committed(
            "Findings",
            one_finding_payload("AC-601", "cargo test -- ac601"),
        ),
        committed(
            "Findings",
            one_finding_payload("AC-999", "cargo test -- ac999"),
        ),
    ];
    let findings = read(&twice).expect("the later payload ingests");
    assert_eq!(
        findings.items[0].ac, "AC-999",
        "the LAST committed finding set is the run's, not the first"
    );
}

/// **AC-282's shape.** A payload that is not a finding set is a refusal naming it, never an empty
/// set — approving by absence is the defect F20 measured and `review::ingest` exists to prevent.
#[test]
fn a_malformed_payload_refuses_rather_than_reading_as_zero_findings() {
    let bad = [committed(
        "Findings",
        serde_json::json!({ "items": "not an array" }),
    )];
    match read(&bad) {
        Err(PassSkipped::Malformed(_)) => {}
        other => panic!("a malformed payload is a typed refusal, got {other:?}"),
    }
}

// ---------------------------------------------------------------- the four cells

/// **AC-288's four cells, over two arms** — and the cell a flip-only implementation gets wrong.
#[test]
fn four_cells_over_two_arms_and_both_fail_is_not_a_flip() {
    let f = finding("AC-601", run_repro("cargo test -- ac601"));

    let cases: [(bool, bool, ReExecution, &str); 4] = [
        (
            false,
            true,
            ReExecution::Flipped,
            "candidate fails, reference passes: the change is what broke it",
        ),
        (
            false,
            false,
            ReExecution::Unverifiable(Unverifiable::BothFail),
            "fails at BOTH ends: the command separates nothing. A flip-only implementation \
             reports this as a block, which is the defect the four-cell table exists for",
        ),
        (
            true,
            true,
            ReExecution::DidNotFlip,
            "candidate passes: the claim did not reproduce",
        ),
        (
            true,
            false,
            ReExecution::DidNotFlip,
            "candidate passes and reference fails: not the candidate's fault",
        ),
    ];

    for (candidate_green, reference_green, expected, why) in cases {
        let candidate = ScriptedToolchain::green(candidate_green);
        let reference = ScriptedToolchain::green(reference_green);
        let reexec = ToolchainReExecutor {
            candidate: &candidate,
            reference: &reference,
        };
        let record = reexec
            .run(&f, "0123456789abcdef0123456789abcdef01234567")
            .expect("a `run` repro naming a selection produces a record");
        assert_eq!(record.outcome, expected, "{why}");
    }
}

/// Two refusals that must never reach a block, each for its own recorded reason.
#[test]
fn mutate_and_selectionless_repros_produce_no_record() {
    let mutate = Finding {
        repro: Repro {
            kind: ReproKind::Mutate,
            command: "cargo test -- ac601".to_owned(),
            expected: Expected::Exit { exit: 0 },
            observed: "exit 101".to_owned(),
            expected_when_met: "exit 0".to_owned(),
            mutation: Some(rk_types::repro::Mutation {
                patch: "--- a\n+++ b\n-x\n".to_owned(),
                suite: "cargo test".to_owned(),
            }),
        },
        ..finding("AC-602", run_repro("cargo test -- ac602"))
    };
    let selectionless = finding("AC-603", run_repro("cargo test"));

    let candidate = ScriptedToolchain::green(false);
    let reference = ScriptedToolchain::green(true);
    let reexec = ToolchainReExecutor {
        candidate: &candidate,
        reference: &reference,
    };

    assert!(
        reexec.run(&mutate, "sha").is_none(),
        "a `mutate` repro is refused BY NAME: no `Toolchain` method applies a patch (ADR-0162's \
         remaining reason; record V's `run_verb` is not a patch applier)"
    );
    assert!(
        reexec.run(&selectionless, "sha").is_none(),
        "a command naming no selection is refused rather than run whole"
    );

    // PREMISE, or the two assertions above are satisfied by a re-executor that never records
    // anything at all: the same arms DO produce a record for a repro that names a selection.
    let ok = finding("AC-604", run_repro("cargo test -- ac604"));
    assert!(
        reexec.run(&ok, "sha").is_some(),
        "the fixture arms are live — a selection-naming `run` repro still produces a record"
    );
}

/// **A candidate that cannot run is `Unverifiable { cannot_run }`, never a flip** (AC-344:
/// *"both-fail or cannot-run resolves to `Unverifiable { both_fail | cannot_run }` … never to a
/// block"*; record T = ADR-0169 decision 2, proposed, superseding ADR-0162 decision 4's
/// *"`is_err()` **is** "fails""* in part).
///
/// Before s8c the re-executor computed `four_cell(candidate.is_err(), reference.is_err())`, so a
/// candidate whose program cannot start (`Err(Transport { Spawn })`) beside a green reference was
/// `(true, false)` -- `Flipped`, a BLOCK. Now either arm's `Err` returns no record, which
/// `confirm_with` reads as `Unverifiable { cannot_run }`; and a red candidate's `observed` names its
/// counts, so it can never read as the repro's `expected_when_met`.
///
/// *(Added 2026-09-26, s8c §8c.9.)*
#[test]
fn a_candidate_that_cannot_run_is_unverifiable_never_a_flip() {
    let f = finding("AC-601", run_repro("cargo test -- ac601"));
    let sha = "0123456789abcdef0123456789abcdef01234567";
    let cannot = ScriptedToolchain {
        arm: Arm::CannotRun,
    };
    let green = ScriptedToolchain { arm: Arm::Green };
    let red = ScriptedToolchain { arm: Arm::Red };

    for (candidate, reference, why) in [
        (
            &cannot,
            &green,
            "a candidate that cannot start beside a green reference",
        ),
        (
            &cannot,
            &red,
            "a candidate that cannot start beside a red reference",
        ),
        (
            &red,
            &cannot,
            "a red candidate beside a reference that cannot start",
        ),
    ] {
        let reexec = ToolchainReExecutor {
            candidate,
            reference,
        };
        assert!(
            reexec.run(&f, sha).is_none(),
            "{why} produces NO record (Unverifiable {{ cannot_run }}), never a flip or a cell: {:?}",
            reexec.run(&f, sha).map(|r| r.outcome)
        );
    }

    // PREMISE, or the assertions above are satisfied by a re-executor that never records at all:
    // a red candidate beside a green reference IS the flip, and its observation names its counts.
    let reexec = ToolchainReExecutor {
        candidate: &red,
        reference: &green,
    };
    let record = reexec
        .run(&f, sha)
        .expect("two runnable arms produce a record");
    assert_eq!(record.outcome, ReExecution::Flipped);
    let rk_kernel::review::classify::Observation::Run(observed) = &record.observation else {
        panic!(
            "a run repro's observation is a Run: {:?}",
            record.observation
        );
    };
    assert_ne!(
        observed.observed, f.repro.expected_when_met,
        "a red candidate never reads as the repro's expected_when_met"
    );
    assert!(
        observed.observed.contains("1 failed"),
        "a red candidate's observation names its counts: {:?}",
        observed.observed
    );
}

/// **An arm that executed nothing is no record, never a cell** (AC-332: an empty run is never a
/// pass; s8c §8c.2's P4 note, *"The re-executor reads an EMPTY arm as no record, like an `Err`
/// arm"*). A candidate whose filter matches nothing (say the change deleted the repro's test)
/// beside a green reference would otherwise be `four_cell(false, false)`, and its `observed` the
/// repro's `expected_when_met`: an empty run recorded as green.
///
/// *(Added 2026-09-26, s8c refuting round 4; M50 deletes the guard.)*
#[test]
fn an_empty_arm_is_no_record_never_a_flip() {
    let f = finding("AC-601", run_repro("cargo test -- ac601"));
    let sha = "0123456789abcdef0123456789abcdef01234567";
    let empty = ScriptedToolchain { arm: Arm::Empty };
    let green = ScriptedToolchain { arm: Arm::Green };
    let red = ScriptedToolchain { arm: Arm::Red };

    // PREMISE: the empty arm's outcome IS `Empty`, so the cases below reach the guard.
    let outcome = empty
        .test(
            &Selection::Filter(vec!["ac601".to_owned()]),
            StepBound::Undeclared,
        )
        .expect("an empty arm is Ok");
    assert_eq!(outcome.status, rk_ports::toolchain::TestStatus::Empty);

    for (candidate, reference, why) in [
        (
            &empty,
            &green,
            "an empty candidate beside a green reference",
        ),
        (&empty, &red, "an empty candidate beside a red reference"),
        (
            &green,
            &empty,
            "a green candidate beside an empty reference",
        ),
        (&red, &empty, "a red candidate beside an empty reference"),
    ] {
        let reexec = ToolchainReExecutor {
            candidate,
            reference,
        };
        let got = reexec.run(&f, sha);
        assert!(
            got.is_none(),
            "{why} produces NO record, never a cell: {:?}",
            got.map(|r| r.outcome)
        );
    }

    // CONTROL: the same two runnable non-empty arms DO produce a record.
    let reexec = ToolchainReExecutor {
        candidate: &red,
        reference: &green,
    };
    assert!(
        reexec.run(&f, sha).is_some(),
        "CONTROL: red beside green is a record"
    );
}

// ================================================================ D21-119: the review key
//
// *(Added 2026-09-29, `s8g` build, package R-D2119-8g; D21-119, TAKEN PROVISIONALLY under D21-105
// items 3 and 4; the Findings-key record, ADR-0196, proposed.)* Every test below reads the SHIPPED
// catalog (`.redkiln/catalog/`, composed by `run`'s own `catalog_sources`), with journal rows in
// the shape `activity::step_committed_event` writes: each names its activation at the shipped
// version and commits a finding set under the `Findings` schema, as the shipped steps'
// `outputs = "value:Findings"` do. None dispatches, runs a binary or touches a repository.

/// The shipped review stage's authored rigor on the fix lane, as `review_pass` reads it (AC-302).
fn shipped_review_weight() -> rk_kernel::review::policy::ReviewWeight {
    let src = std::fs::read_to_string(workspace().join(".redkiln/packs/bug-standard.toml"))
        .expect("the shipped fix-lane pack reads");
    let pack = rk_kernel::select::Pack::from_toml("bug-standard.toml", &src)
        .expect("the shipped fix-lane pack loads");
    rk_kernel::review::policy::resolve_review_tier(pack.rigor(REVIEW))
}

/// The pass's verdict over `findings`, re-executed on two scripted arms, mapped as the seal maps it.
fn sealed_verdict(findings: &Findings, candidate_green: bool) -> Verdict {
    let candidate = ScriptedToolchain::green(candidate_green);
    let reference = ScriptedToolchain::green(true);
    let reexec = ToolchainReExecutor {
        candidate: &candidate,
        reference: &reference,
    };
    let confirmed = rk_cli::review_pass::confirm(
        findings,
        shipped_review_weight(),
        "0123456789abcdef0123456789abcdef01234567",
        "an-item",
        rk_types::time::Timestamp::from_unix_seconds(0),
        &reexec,
    );
    seal_verdict(confirmed.block_set().is_empty())
}

/// **(b1) An `adversarial-review`/`review` activation seals `approved`** — at its own stage, and
/// ONLY there (D21-93: the seal names its stage; D21-119 item 2).
///
/// An empty finding set, committed by the shipped review activity's `review` step, is selected at
/// the shipped review stage and maps to `approved`. The same journal read at `release` (the stage a
/// lane reaches next) is no review row there: without the stage clause the review stage's verdict
/// would be sealed again under every later stage the lane enters.
#[test]
fn adversarial_review_seals_approved_at_its_own_stage() {
    premise(ADVERSARIAL_REVIEW, REVIEW, REVIEW_VERDICT_KIND, "review");
    let journal = [committed_by(
        &at_version(ADVERSARIAL_REVIEW),
        "review",
        FINDINGS_SCHEMA,
        serde_json::json!({ "items": [] }),
    )];
    let findings = read(&journal).expect("the shipped review activation's row is the verdict row");
    assert!(findings.items.is_empty(), "the empty finding set is read");
    assert_eq!(
        sealed_verdict(&findings, true),
        Verdict::Approved,
        "nothing blocked seals `approved`"
    );
    assert_eq!(
        read_at(&journal, "release"),
        Err(PassSkipped::NoFindingsRow),
        "the review stage's row was selected again at `release`: the seal is scoped to the stage \
         it is written under"
    );
}

/// **(b2) An `adversarial-review`/`review` activation seals `changes-requested`** — a finding the
/// re-execution flips (candidate red, reference green) blocks at the shipped review stage's weight,
/// and a later stage does not re-read it.
#[test]
fn adversarial_review_seals_changes_requested_at_its_own_stage() {
    premise(ADVERSARIAL_REVIEW, REVIEW, REVIEW_VERDICT_KIND, "review");
    let journal = [committed_by(
        &at_version(ADVERSARIAL_REVIEW),
        "review",
        FINDINGS_SCHEMA,
        one_finding_payload("AC-601", "cargo test -- ac601"),
    )];
    let findings = read(&journal).expect("the shipped review activation's row is the verdict row");
    assert_eq!(findings.items.len(), 1, "the one finding is read");
    assert_eq!(
        sealed_verdict(&findings, false),
        Verdict::ChangesRequested,
        "a flipped, reproduced finding at the shipped review stage's weight blocks, and a block \
         seals `changes-requested`"
    );
    assert_eq!(
        sealed_verdict(&findings, true),
        Verdict::Approved,
        "CONTROL: the same finding that does not reproduce blocks nothing"
    );
    assert_eq!(
        read_at(&journal, "release"),
        Err(PassSkipped::NoFindingsRow),
        "the review stage's row was selected again at `release`"
    );
}

/// **(c) A `fix-spec`/`adversary` `Findings` row does not trigger the review pass or a seal.**
///
/// `fix-spec` (specification, provides `Spec`) is one of the ~40 shipped non-review activities
/// whose adversary step commits under `Findings` (D21-119 finding 4). Before D21-119 the seal took
/// ANY `Findings` row, so a fix lane's specification adversary was read as a review verdict.
#[test]
fn a_fix_spec_adversary_findings_row_does_not_seal() {
    premise("fix-spec", "specification", "Spec", "adversary");
    let journal = [committed_by(
        &at_version("fix-spec"),
        "adversary",
        FINDINGS_SCHEMA,
        one_finding_payload("AC-601", "cargo test -- ac601"),
    )];
    for stage in ["specification", REVIEW] {
        assert_eq!(
            read_at(&journal, stage),
            Err(PassSkipped::NoFindingsRow),
            "a fix-spec/adversary `Findings` row was read as a review verdict at `{stage}`"
        );
    }
}

/// **(d) A later non-review `Findings` row does not displace the review verdict.**
///
/// The review activation commits first; a later adversary row commits after it. The ruling names
/// `equivalence-boundary`/`adversary` as the verify-stage case; *(measured at build)* S8G-P4
/// re-homed `equivalence-boundary` to `implement`, so the post-review case is also taken at a stage
/// the lane reaches AFTER review: `knowledge-harvest`/`adversary` (closeout). Under "the last
/// `Findings` row" either would displace the verdict.
#[test]
fn a_later_non_review_findings_row_does_not_displace_the_review_verdict() {
    premise(ADVERSARIAL_REVIEW, REVIEW, REVIEW_VERDICT_KIND, "review");
    premise(
        "equivalence-boundary",
        "implement",
        "AuthoredTests",
        "adversary",
    );
    premise("knowledge-harvest", "closeout", "KBAtoms", "adversary");
    let journal = [
        committed_by(
            &at_version(ADVERSARIAL_REVIEW),
            "review",
            FINDINGS_SCHEMA,
            one_finding_payload("AC-601", "cargo test -- ac601"),
        ),
        committed_by(
            &at_version("equivalence-boundary"),
            "adversary",
            FINDINGS_SCHEMA,
            one_finding_payload("AC-998", "cargo test -- ac998"),
        ),
        committed_by(
            &at_version("knowledge-harvest"),
            "adversary",
            FINDINGS_SCHEMA,
            one_finding_payload("AC-999", "cargo test -- ac999"),
        ),
    ];
    let findings = read(&journal).expect("the review activation's row is still the verdict row");
    assert_eq!(
        findings
            .items
            .iter()
            .map(|f| f.ac.as_str())
            .collect::<Vec<_>>(),
        vec!["AC-601"],
        "a later non-review `Findings` row displaced the review verdict"
    );
    assert_eq!(
        read_at(&journal, "closeout"),
        Err(PassSkipped::NoFindingsRow),
        "the closeout adversary's row was read as a review verdict at `closeout`"
    );
}

/// **(e) `security-review` and `design-review` do not seal.** Both sit at the shipped review stage
/// and both have a `review` step committing under `Findings`, but neither provides the review
/// verdict kind first: `SecurityReviewVerdict` is the near miss an `ends_with`/`contains` match
/// would take, and `DesignVerdict` is not a review verdict at all.
#[test]
fn security_review_and_design_review_do_not_seal() {
    premise("security-review", REVIEW, "SecurityReviewVerdict", "review");
    premise("design-review", REVIEW, "DesignVerdict", "review");
    for id in ["security-review", "design-review"] {
        assert!(
            !is_review_activity(shipped_activity(id)),
            "`{id}` was read as a review activity"
        );
        let journal = [committed_by(
            &at_version(id),
            "review",
            FINDINGS_SCHEMA,
            one_finding_payload("AC-601", "cargo test -- ac601"),
        )];
        assert_eq!(
            read(&journal),
            Err(PassSkipped::NoFindingsRow),
            "`{id}`'s `Findings` row was read as the review verdict"
        );
    }
    assert!(
        is_review_activity(shipped_activity(ADVERSARIAL_REVIEW)),
        "CONTROL: the shipped adversarial-review is a review activity"
    );
}

/// **(f) An unresolvable `activity_id_at_version` is refused by name, never skipped.** A skip
/// would read as `NoFindingsRow` — no review — which is the vacuous green D21-119 item 2 forbids.
#[test]
fn an_unresolvable_activity_id_at_version_is_refused_by_name() {
    let review = committed_by(
        &at_version(ADVERSARIAL_REVIEW),
        "review",
        FINDINGS_SCHEMA,
        serde_json::json!({ "items": [] }),
    );
    let stale = committed_by(
        "adversarial-review@999",
        "review",
        FINDINGS_SCHEMA,
        one_finding_payload("AC-601", "cargo test -- ac601"),
    );
    assert_eq!(
        read(&[review.clone(), stale]),
        Err(PassSkipped::Unresolved {
            activity_id_at_version: Some("adversarial-review@999".to_owned())
        }),
        "an activation the loaded catalog does not carry was not refused by name"
    );
    let mut absent = committed("Findings", serde_json::json!({ "items": [] }));
    absent
        .payload
        .as_object_mut()
        .expect("a row is an object")
        .remove("activity_id_at_version");
    assert_eq!(
        read(&[review.clone(), absent]),
        Err(PassSkipped::Unresolved {
            activity_id_at_version: None
        }),
        "a `Findings` row naming no activation was not refused"
    );
    assert!(
        read(&[review]).is_ok(),
        "CONTROL: the resolvable row alone is the verdict row"
    );
}

/// **(g) `formal-inspection`'s verdict row is pinned: the LAST of its two `Findings` steps.** The
/// shipped `formal-inspection` commits `reader` then `second-reader`, both under `Findings`, from
/// one review activation; by the existing re-dispatch rule the verdict is the last committed row,
/// the second reader's. Green on arrival by construction (the rule predates D21-119); its red is
/// the mutant that takes the first row (§8g.1's D21-119 note, M6).
#[test]
fn formal_inspections_second_reader_row_is_the_verdict() {
    premise("formal-inspection", REVIEW, REVIEW_VERDICT_KIND, "reader");
    premise(
        "formal-inspection",
        REVIEW,
        REVIEW_VERDICT_KIND,
        "second-reader",
    );
    let fi = at_version("formal-inspection");
    let journal = [
        committed_by(
            &fi,
            "reader",
            FINDINGS_SCHEMA,
            one_finding_payload("AC-701", "cargo test -- ac701"),
        ),
        committed_by(
            &fi,
            "second-reader",
            FINDINGS_SCHEMA,
            one_finding_payload("AC-702", "cargo test -- ac702"),
        ),
    ];
    let findings = read(&journal).expect("formal-inspection is a review activation");
    assert_eq!(
        findings
            .items
            .iter()
            .map(|f| f.ac.as_str())
            .collect::<Vec<_>>(),
        vec!["AC-702"],
        "formal-inspection's verdict row is the second reader's, the last committed"
    );
}
