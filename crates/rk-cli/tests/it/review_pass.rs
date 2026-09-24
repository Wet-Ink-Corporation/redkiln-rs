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

use rk_cli::review_pass::{PassSkipped, ToolchainReExecutor, findings_in, selection_of};
use rk_kernel::review::reexec::{ReExecution, ReExecutor, Unverifiable};
use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};
use rk_types::artifact::findings::{EvidenceLabel, Finding};
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::{Expected, Repro, ReproKind};
use rk_types::text::Text;

/// A toolchain whose `test` answer is scripted per arm. Every other method refuses: this fake
/// stands in for exactly one capability and says so by failing on the rest.
struct ScriptedToolchain {
    green: bool,
}

impl Toolchain for ScriptedToolchain {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self) -> Result<(), StepError> {
        Err(refusal("build"))
    }
    /// RT-22's contract: `Ok(count)` ONLY on a green suite, so `Err` **is** "fails".
    fn test(&self, selection: &[String]) -> Result<u32, StepError> {
        if self.green {
            Ok(u32::try_from(selection.len()).unwrap_or(0))
        } else {
            Err(StepError::NonZero {
                code: Some(101),
                stderr_digest: rk_types::digest::Digest::from_hex(&"a".repeat(64))
                    .expect("a 64-hex digest"),
            })
        }
    }
    fn lint(&self) -> Result<(), StepError> {
        Err(refusal("lint"))
    }
    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        Err(refusal("classify"))
    }
    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {
        Err(refusal("change_summary"))
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

fn committed(schema: &str, result: serde_json::Value) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": "review@1",
            "at": 0,
            "attempt": 1,
            "item": "an-item",
            "result": result,
            "run_id": "review-0000000000000000",
            "schema": schema,
        }),
    )
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
        findings_in(&[]),
        Err(PassSkipped::NoFindingsRow),
        "an empty journal is no review, not a review with nothing to say"
    );

    // A committed activation under ANOTHER schema is not a review: this is the path every
    // non-review stage takes, and it must not produce a verdict.
    let other = [committed("ImplReport", serde_json::json!({ "ok": true }))];
    assert_eq!(
        findings_in(&other),
        Err(PassSkipped::NoFindingsRow),
        "a row under another schema is not a finding set"
    );

    let good = [committed(
        "Findings",
        one_finding_payload("AC-601", "cargo test -- ac601"),
    )];
    let findings = findings_in(&good).expect("a well-formed payload ingests");
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
    let findings = findings_in(&twice).expect("the later payload ingests");
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
    match findings_in(&bad) {
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
        let candidate = ScriptedToolchain {
            green: candidate_green,
        };
        let reference = ScriptedToolchain {
            green: reference_green,
        };
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

    let candidate = ScriptedToolchain { green: false };
    let reference = ScriptedToolchain { green: true };
    let reexec = ToolchainReExecutor {
        candidate: &candidate,
        reference: &reference,
    };

    assert!(
        reexec.run(&mutate, "sha").is_none(),
        "a `mutate` repro is refused BY NAME: no `Toolchain` method applies a patch, Checkpoint \
         7a counts exactly six methods on that port, and a seventh for a fixture is the cost s7d \
         refused for `Pack`"
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
