//! **Test selection, the resolver side** (D21-42 4c; `s8d` §8d.3; R2 = ADR-0173 decision 9,
//! written `proposed`).
//!
//! ADR-0148 made `toolchain:test` refuse until a selection resolver exists, and its decision 4 says
//! how that ends: *"This is reversed by a resolver, not by a revert."* The resolver resolves a
//! `${…}` selection and an `artifact:ReproCase.command` selection, parses a command through the ONE
//! `selection_of` (moved into `rk-kernel`; `rk_cli::review_pass` re-exports it), and hands
//! `"affected"` the item's committed `Changeset` paths for `s8c`'s package-level mapping. What
//! stands of ADR-0148 is its forbids: an unresolvable selection refuses by name and NEVER reaches
//! the port as an empty one — *"`cargo test affected`, which matches no test and exits 0"*.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::StepKind;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::reference::{SelectionRefusal, check_selection_token, parse_selection};
use rk_kernel::runtime::step::ToolRouting;
use rk_kernel::workflow::memo::ToolchainValues;
use rk_ports::harness::Harness;
use rk_ports::toolchain::{Selection, StepBound, TestOutcome, Toolchain, VerbCall, VerbOutput};
use rk_types::error::StepError;

use crate::reference_resolver::{activation, emitted_row, refusal, step, with_inputs};
use crate::runtime::{
    CAP, CountingHarness, STAGE, at, counters, empty_state, gate_site, journal, ok_typed,
    ports_with_toolchain, spec, store,
};

/// A toolchain that records every `Selection` it is handed, and answers a one-test green outcome.
#[derive(Default)]
struct SelectionToolchain {
    calls: AtomicU32,
    selections: Mutex<Vec<Selection>>,
}

impl SelectionToolchain {
    fn selections(&self) -> Vec<Selection> {
        self.selections.lock().expect("not poisoned").clone()
    }
}

impl Toolchain for SelectionToolchain {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self, _b: StepBound) -> Result<rk_types::verb_output::BuildReport, StepError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: "build".to_owned(),
        })
    }
    fn test(&self, selection: &Selection, _b: StepBound) -> Result<TestOutcome, StepError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.selections
            .lock()
            .expect("not poisoned")
            .push(selection.clone());
        TestOutcome::new(
            rk_types::verify::VerifyReport {
                basis: rk_types::verify::Basis::WholeSuite,
                argv: vec!["selection".to_owned(), "test".to_owned()],
                selected_test_count: 1,
                total_test_count: 1,
                total_test_count_source: rk_types::verify::TotalTestCountSource::OwnExecutedCount,
                wall_secs: 0,
                toolchain_version: "selection".to_owned(),
                suite_sha: "selection".to_owned(),
                basis_reason: Some(rk_types::verify::BasisReason::ProbeUnverified),
            },
            1,
            0,
            rk_ports::toolchain::TestRunner::named("selection", "0"),
        )
    }
    fn lint(&self, _b: StepBound) -> Result<(), StepError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
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
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: "change_summary".to_owned(),
        })
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: call.verb.to_owned(),
        })
    }
}

/// A `toolchain:test` step whose `inputs.selection` is `selection`.
fn test_step(selection: &str) -> rk_kernel::catalog::Step {
    let mut s = with_inputs(
        step("run-tests", StepKind::Tool),
        &[("selection", selection)],
    );
    s.run = Some("toolchain:test".to_owned());
    s
}

/// Walk `[write, run-tests]` over a journal seeded with `seed`; `write` returns `wrote`.
fn walk(
    seed: Vec<rk_types::event::Event>,
    wrote: serde_json::Value,
    selection: &str,
) -> (
    rk_kernel::runtime::activity::ActivityReport,
    Arc<SelectionToolchain>,
    Vec<rk_types::event::Event>,
) {
    walk_answering(seed, vec![ok_typed(wrote)], selection)
}

/// [`walk`], with `write`'s whole script supplied.
fn walk_answering(
    seed: Vec<rk_types::event::Event>,
    script: Vec<Result<rk_types::typed::Typed, StepError>>,
    selection: &str,
) -> (
    rk_kernel::runtime::activity::ActivityReport,
    Arc<SelectionToolchain>,
    Vec<rk_types::event::Event>,
) {
    let (q, a) = counters();
    let db = store(&q, &a);
    for e in seed {
        db.lock().expect("not poisoned").append(e).expect("seeded");
    }
    let tc = Arc::new(SelectionToolchain::default());
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::new(CountingHarness::scripted(script)),
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        "run-selection",
        STAGE,
    )
    .with_tool_routing(ToolRouting::new(
        crate::tool_verb_routing::registry("", ""),
        ToolchainValues::default(),
    ));
    let sp = spec();
    let steps = vec![step("write", StepKind::Agent), test_step(selection)];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &rk_kernel::predicate::Context::new(),
            gate_site(&empty_state()),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a value");
    (report, tc, journal(&db))
}

/// `write`'s committed step row holding `value`, as an earlier run left it.
///
/// *(dated 2026-11-08, s8 repro-aware implement; ADR-0205, proposed; plan Decision D5.)* A FRESH
/// `write` value that is not a selection is refused at `write` itself as `Schema` (`feed_check::`).
/// A SERVED value is never re-checked (Invariant 5), so the tool step's own refusal stays the belt
/// for it; the cases that pin that belt walk over this row.
fn served_write(value: serde_json::Value) -> rk_types::event::Event {
    rk_types::event::Event::new(
        rk_types::event_kind::EventKind::ActivityCommitted,
        serde_json::json!({
            "item": crate::runtime::ITEM, "activity_id_at_version": crate::runtime::AID,
            "attempt": 1, "step_id": "write", "iteration_path": [], "at": 50, "run_id": "r",
            "result": value, "schema": crate::runtime::SCHEMA,
        }),
    )
}

/// **A step-output selection resolves and reaches the port as NAMED TESTS** — and `"affected"`
/// reaches it as the item's committed `Changeset` paths.
#[test]
fn a_step_output_selection_resolves_and_reaches_the_port_as_named_tests() {
    // `${write.outputs.tests_added}`, the shape of `test-after`'s `${write.outputs.tests_added}`.
    let (report, tc, _) = walk(
        Vec::new(),
        serde_json::json!({"tests_added": ["ac601_fails_before", "ac601_passes_after"]}),
        "${write.outputs.tests_added}",
    );
    assert!(
        report.completed() && report.step_error().is_none(),
        "{report:?}"
    );
    assert_eq!(
        tc.selections(),
        vec![Selection::Filter(vec![
            "ac601_fails_before".to_owned(),
            "ac601_passes_after".to_owned()
        ])],
        "the resolved list reaches the port as those named tests, and nothing else"
    );
    // A `${…}` that resolves to a COMMAND is parsed by the one `selection_of`.
    let (report, tc, _) = walk(
        Vec::new(),
        serde_json::json!({"command": "cargo test -p rk-cli -- ac601"}),
        "${write.outputs.command}",
    );
    assert!(report.completed(), "{report:?}");
    assert_eq!(
        tc.selections(),
        vec![Selection::Filter(vec!["ac601".to_owned()])]
    );
    // `"affected"`: the item's committed `Changeset` paths (its `files` member, ADR-0165).
    let changeset = emitted_row(
        "tdd-red-green-refactor@2",
        "Changeset",
        serde_json::json!({"sha": "abc", "files": ["crates/rk-cli/src/lib.rs", "README.md"]}),
    );
    let (report, tc, _) = walk(vec![changeset], serde_json::json!({}), "affected");
    assert!(report.completed(), "{report:?}");
    assert_eq!(
        tc.selections(),
        vec![Selection::Affected {
            changed: vec![
                "crates/rk-cli/src/lib.rs".to_owned(),
                "README.md".to_owned()
            ],
            recorded: None,
        }],
        "`affected` reaches `s8c`'s mapping as the Changeset's paths"
    );
}

/// **`artifact:ReproCase.command` resolves through the ONE `selection_of`** (`confirmation-test`'s
/// and `bisect`'s shape).
#[test]
fn an_artifact_command_selection_resolves_through_selection_of() {
    let repro = emitted_row(
        "minimal-repro@2",
        "ReproCase",
        serde_json::json!({"command": "cargo test -p rk-kernel -- ac601 ac602"}),
    );
    let (report, tc, _) = walk(
        vec![repro],
        serde_json::json!({}),
        "artifact:ReproCase.command",
    );
    assert!(
        report.completed() && report.step_error().is_none(),
        "{report:?}"
    );
    assert_eq!(
        tc.selections(),
        vec![Selection::Filter(vec![
            "ac601".to_owned(),
            "ac602".to_owned()
        ])],
        "the repro's `--` tail, parsed by `selection_of`, reaches the port"
    );
    // It is the ONE parser: the kernel's function, the one `rk_cli::review_pass` re-exports.
    assert_eq!(
        rk_kernel::runtime::reference::selection_of("cargo test -- ac601 ac602"),
        Some(vec!["ac601".to_owned(), "ac602".to_owned()])
    );
    assert_eq!(
        rk_kernel::runtime::reference::selection_of("cargo test"),
        None
    );
    assert_eq!(
        rk_kernel::runtime::reference::selection_of("cargo test -- "),
        None
    );
}

/// **An unresolvable selection refuses by name and NEVER reaches the port** — not as an empty
/// selection, and not as the whole suite (ADR-0148's forbids; M14).
#[test]
fn an_unresolvable_selection_refuses_and_never_runs_the_whole_suite() {
    let repro_no_filter = || {
        vec![emitted_row(
            "minimal-repro@2",
            "ReproCase",
            serde_json::json!({"command": "cargo test"}),
        )]
    };
    let cases: Vec<(&str, Vec<rk_types::event::Event>, serde_json::Value, &str)> = vec![
        // *(dated 2026-11-08, ADR-0205, proposed.)* Served: a fresh `{}` is refused at `write`.
        (
            "a step output that has no such field",
            vec![served_write(serde_json::json!({}))],
            serde_json::json!({}),
            "${write.outputs.tests_added}",
        ),
        (
            "a step output that is an EMPTY list",
            Vec::new(),
            serde_json::json!({"tests_added": []}),
            "${write.outputs.tests_added}",
        ),
        (
            "a step that is not in the activation",
            Vec::new(),
            serde_json::json!({}),
            "${nope.outputs}",
        ),
        (
            "a repro command with no `--` filter",
            repro_no_filter(),
            serde_json::json!({}),
            "artifact:ReproCase.command",
        ),
        (
            "no repro binding at all",
            Vec::new(),
            serde_json::json!({}),
            "artifact:ReproCase.command",
        ),
        (
            "`affected` with no Changeset binding",
            Vec::new(),
            serde_json::json!({}),
            "affected",
        ),
        (
            "a literal that is not a selection",
            Vec::new(),
            serde_json::json!({}),
            "all",
        ),
    ];
    let walked = cases.len();
    for (name, seed, wrote, selection) in cases {
        let served = seed
            .iter()
            .any(|e| e.payload.get("step_id") == Some(&serde_json::json!("write")));
        let (report, tc, _) = walk(seed, wrote, selection);
        assert_eq!(
            tc.calls.load(Ordering::SeqCst),
            0,
            "{name}: the port was reached with {:?} — an unresolvable selection must never reach it \
             (ADR-0148: an empty selection is the whole suite, and a literal matches nothing)",
            tc.selections()
        );
        let e = refusal(&report);
        assert!(
            e.contains("`run-tests`")
                && e.contains("inputs.selection")
                && e.contains("never reaches"),
            "{name}: the refusal must name the step and its selection: {e}"
        );
        // A served `write` is a commit that stands as surely as a fresh one (`holds_a_commit`).
        assert_eq!(
            (report.committed(), report.served()),
            if served { (false, 1) } else { (true, 0) },
            "{name}: the agent step's commit stands: {report:?}"
        );
    }
    assert_eq!(walked, 7, "the case table walked");
}

// ---- D21-74 ruling 2 (R2 = ADR-0173 decision 9 as amended, proposed; `s8d` build, package
// F3-D2174) --------------------------------------------------------------------------------------
//
// `tdd-red-green-refactor`'s `green-verify` and `refactor-verify` declare `selection = "affected"`,
// and `tdd` PRODUCES the item's `Changeset`, so none is committed while it runs: the lane walk read
// *"no committed `emit` binding of `Changeset` for the item `LANE-WALK-1` is in the journal"*.
// D21-74: *"inside the producer of `Changeset`, `"affected"` resolves over the run branch's diff
// (R6's `HEAD..rk-run/<item>` range, read through `rk_git`), and elsewhere as before, from the
// committed `Changeset`."*

/// A run-branch reader that answers one fixed value and records every item it is asked for.
struct BranchDouble {
    answer: Result<Option<Vec<String>>, String>,
    asked: Mutex<Vec<String>>,
}

impl BranchDouble {
    fn answering(answer: Result<Option<Vec<String>>, String>) -> Arc<Self> {
        Arc::new(Self {
            answer,
            asked: Mutex::new(Vec::new()),
        })
    }
    fn asked(&self) -> Vec<String> {
        self.asked.lock().expect("not poisoned").clone()
    }
}

impl rk_kernel::runtime::reference::RunBranch for BranchDouble {
    fn changed_paths(&self, item: &str) -> Result<Option<Vec<String>>, String> {
        self.asked
            .lock()
            .expect("not poisoned")
            .push(item.to_owned());
        self.answer.clone()
    }
}

/// An `emit` step binding `kind` to `write`'s output: what makes the activation `kind`'s producer.
fn emit_of(kind: &str) -> rk_kernel::catalog::Step {
    let mut s = step("emit", StepKind::Emit);
    s.emit = Some(rk_kernel::catalog::EmitBinding {
        artifacts: std::iter::once((
            kind.to_owned(),
            rk_kernel::catalog::EmitArtifact::Ref("${write.outputs}".to_owned()),
        ))
        .collect(),
        evidence: Vec::new(),
    });
    s
}

/// Walk `[write, run-tests(affected), emit(<emits>)]` over a journal seeded with `seed`, with the
/// run-branch reader `branch` bound (or none).
fn walk_affected(
    seed: Vec<rk_types::event::Event>,
    emits: &str,
    branch: Option<Arc<BranchDouble>>,
) -> (
    rk_kernel::runtime::activity::ActivityReport,
    Arc<SelectionToolchain>,
) {
    let (q, a) = counters();
    let db = store(&q, &a);
    for e in seed {
        db.lock().expect("not poisoned").append(e).expect("seeded");
    }
    let tc = Arc::new(SelectionToolchain::default());
    let mut routing = ToolRouting::new(
        crate::tool_verb_routing::registry("", ""),
        ToolchainValues::default(),
    );
    if let Some(b) = branch {
        routing = routing.with_run_branch(b as Arc<dyn rk_kernel::runtime::reference::RunBranch>);
    }
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::new(CountingHarness::scripted(vec![ok_typed(
                serde_json::json!({"sha": "abc", "files": ["src/on_branch.rs"]}),
            )])) as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        "run-affected",
        STAGE,
    )
    .with_tool_routing(routing);
    let sp = spec();
    let steps = vec![
        step("write", StepKind::Agent),
        test_step("affected"),
        emit_of(emits),
    ];
    let report = rt
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &rk_kernel::predicate::Context::new(),
            gate_site(&empty_state()),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a value");
    (report, tc)
}

/// **Inside the producer of `Changeset`, `"affected"` resolves over the run branch's diff** — with
/// no `Changeset` committed, and ahead of one an earlier activation committed; **outside it, the
/// committed `Changeset` is read and the branch is never asked.**
#[test]
fn inside_the_changeset_producer_affected_resolves_over_the_run_branch_diff() {
    let affected = |paths: &[&str]| Selection::Affected {
        changed: paths.iter().map(|p| (*p).to_owned()).collect(),
        recorded: None,
    };
    let committed = || {
        vec![emitted_row(
            "an-earlier-producer@1",
            "Changeset",
            serde_json::json!({"sha": "old", "files": ["src/committed.rs"]}),
        )]
    };

    // ---- (a) the producer, no `Changeset` committed: the branch's paths reach the port -----------
    let branch = BranchDouble::answering(Ok(Some(vec![
        "src/on_branch.rs".to_owned(),
        "tests/it/on_branch.rs".to_owned(),
    ])));
    let (report, tc) = walk_affected(Vec::new(), "Changeset", Some(Arc::clone(&branch)));
    assert!(
        report.completed() && report.step_error().is_none(),
        "inside `Changeset`'s producer, `affected` must resolve with no committed Changeset: {}",
        refusal(&report)
    );
    assert_eq!(
        tc.selections(),
        vec![affected(&["src/on_branch.rs", "tests/it/on_branch.rs"])],
        "the run branch's paths reach `s8c`'s mapping"
    );
    assert_eq!(
        branch.asked(),
        vec![crate::runtime::ITEM.to_owned()],
        "the branch is read once, for this item"
    );

    // ---- (b) the producer, with an earlier `Changeset` committed: the BRANCH still wins ----------
    let branch = BranchDouble::answering(Ok(Some(vec!["src/on_branch.rs".to_owned()])));
    let (report, tc) = walk_affected(committed(), "Changeset", Some(Arc::clone(&branch)));
    assert!(report.completed(), "{}", refusal(&report));
    assert_eq!(
        tc.selections(),
        vec![affected(&["src/on_branch.rs"])],
        "inside the producer the committed Changeset is an EARLIER one, never this activation's"
    );

    // ---- (c) NOT the producer: the committed `Changeset`, and the branch is never asked ---------
    let branch = BranchDouble::answering(Ok(Some(vec!["src/on_branch.rs".to_owned()])));
    let (report, tc) = walk_affected(committed(), "ImplReport", Some(Arc::clone(&branch)));
    assert!(report.completed(), "{}", refusal(&report));
    assert_eq!(
        tc.selections(),
        vec![affected(&["src/committed.rs"])],
        "outside the producer, `affected` is the committed Changeset, as before"
    );
    assert!(
        branch.asked().is_empty(),
        "outside the producer the run branch is never read: {:?}",
        branch.asked()
    );

    // ---- (d)-(f) the producer's refusals: each by name, and the port never reached ---------------
    let cases: Vec<(&str, Option<Arc<BranchDouble>>, &str)> = vec![
        ("no reader bound", None, "no run-branch reader is bound"),
        (
            "an empty range",
            Some(BranchDouble::answering(Ok(None))),
            "carries no change",
        ),
        (
            "an unreadable range",
            Some(BranchDouble::answering(Err("rev-list exploded".to_owned()))),
            "rev-list exploded",
        ),
    ];
    let walked = cases.len();
    for (name, branch, needle) in cases {
        let (report, tc) = walk_affected(committed(), "Changeset", branch);
        assert_eq!(
            tc.calls.load(Ordering::SeqCst),
            0,
            "{name}: the port was reached with {:?}",
            tc.selections()
        );
        let e = refusal(&report);
        assert!(
            e.contains("`run-tests`") && e.contains("inputs.selection") && e.contains(needle),
            "{name}: the refusal must name the step, its selection and `{needle}`: {e}"
        );
    }
    assert_eq!(walked, 3, "the refusal table walked");
}

// ---- ADR-0204 decision 6 (proposed; dated 2026-11-08, s8 repro-pair route): `parse_selection` is
// the ONE parser, and it refuses more. ADR-0173 stands. -------------------------------------------
//
// The 2026-11-08 relaunch 3 stopped at exit 14: `minimal-repro@1`'s `confirm` read an investigator's
// prose as a command, and `selection_of` split it after ` -- ` into libtest arguments, so the
// prose's closing backtick reached `--list` as part of `--exact`. A selection is now refused by
// name, before the port, unless it is a plain command.

/// The shape of the investigator's `ReproAttempt.command` on 2026-11-08 (relaunch 3, rc 14). The
/// full string lives in the operator's store; what is kept here are its failing features: prose
/// with a parenthesis before the separator, and a closing backtick glued to `--exact`.
const PROSE_2026_11_08: &str = "Append the repro (crates/rk-cli/tests/it/doctor.rs) and run \
     `cargo test -p rk-cli --test it -- \
     doctor::s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error --exact`";

#[test]
fn parse_selection_reads_the_plain_tail_after_the_separator() {
    assert_eq!(
        parse_selection("cargo test -p rk-cli --test it -- doctor::x --exact"),
        Ok(vec!["doctor::x".to_owned(), "--exact".to_owned()])
    );
}

#[test]
fn parse_selection_refuses_the_2026_11_08_prose_by_name() {
    let refused = parse_selection(PROSE_2026_11_08);
    assert_eq!(
        refused,
        Err(SelectionRefusal::ShellSyntax {
            found: '(',
            at: PROSE_2026_11_08
                .find('(')
                .expect("the prose holds a parenthesis"),
        })
    );
    assert_eq!(
        rk_kernel::runtime::reference::selection_of(PROSE_2026_11_08),
        None
    );
}

#[test]
fn parse_selection_refuses_a_backtick_a_dollar_and_a_semicolon() {
    for (command, found, at) in [
        ("cargo test -- `x`", '`', 14),
        ("cargo test -- $HOME", '$', 14),
        ("cargo test -- a; rm -rf b", ';', 15),
    ] {
        assert_eq!(
            parse_selection(command),
            Err(SelectionRefusal::ShellSyntax { found, at }),
            "{command}"
        );
    }
}

#[test]
fn parse_selection_refuses_a_newline_as_multiline() {
    assert_eq!(
        parse_selection("cargo test -- a\ncargo test -- b"),
        Err(SelectionRefusal::Multiline)
    );
    assert_eq!(
        parse_selection("cargo test -- a\r"),
        Err(SelectionRefusal::Multiline)
    );
}

#[test]
fn parse_selection_refuses_a_control_byte() {
    assert_eq!(
        parse_selection("cargo test -- a\tb"),
        Err(SelectionRefusal::Control { at: 15 })
    );
    assert_eq!(
        parse_selection("cargo test -- a\u{1b}[0m"),
        Err(SelectionRefusal::Control { at: 15 })
    );
}

#[test]
fn parse_selection_refuses_no_filter_and_an_empty_filter() {
    assert_eq!(
        parse_selection("cargo test"),
        Err(SelectionRefusal::NoFilter)
    );
    assert_eq!(
        parse_selection("cargo test -- "),
        Err(SelectionRefusal::EmptyFilter)
    );
    assert_eq!(
        parse_selection("cargo test --    "),
        Err(SelectionRefusal::EmptyFilter)
    );
}

#[test]
fn each_selection_refusal_renders_its_variant_name_first() {
    for (refusal, name) in [
        (SelectionRefusal::Multiline, "Multiline"),
        (SelectionRefusal::Control { at: 3 }, "Control"),
        (
            SelectionRefusal::ShellSyntax { found: '|', at: 4 },
            "ShellSyntax",
        ),
        (SelectionRefusal::NoFilter, "NoFilter"),
        (SelectionRefusal::EmptyFilter, "EmptyFilter"),
        (
            SelectionRefusal::BadToken {
                token: "a b".to_owned(),
            },
            "BadToken",
        ),
    ] {
        assert!(refusal.to_string().starts_with(name), "{refusal}");
    }
}

#[test]
fn check_selection_token_accepts_a_name_and_refuses_whitespace_shell_and_empty() {
    assert_eq!(check_selection_token("doctor::x"), Ok(()));
    // A `-`-led token is a libtest flag, not a name: `tests_added` must not inject one.
    for token in [
        "",
        "a b",
        "a;b",
        "`a`",
        "a\u{7}",
        "--include-ignored",
        "--ignored",
        "--list",
        "-q",
    ] {
        assert_eq!(
            check_selection_token(token),
            Err(SelectionRefusal::BadToken {
                token: token.to_owned()
            }),
            "{token:?}"
        );
    }
}

mod parse_selection_properties {
    use proptest::prelude::*;
    use rk_kernel::runtime::reference::{parse_selection, selection_of};

    const SHELL: &[char] = &[
        '`', '$', ';', '|', '&', '<', '>', '(', ')', '{', '}', '"', '\'', '\\',
    ];

    proptest! {
        #[test]
        fn selection_of_is_parse_selection_ok(command in "\\PC{0,40}( -- \\PC{0,20})?") {
            prop_assert_eq!(selection_of(&command), parse_selection(&command).ok());
        }

        #[test]
        fn any_shell_character_is_refused(
            head in "[a-z :-]{0,20}",
            shell in proptest::sample::select(SHELL),
            tail in "[a-z :-]{0,20}",
        ) {
            let command = format!("{head} -- x{shell}{tail}");
            prop_assert!(parse_selection(&command).is_err(), "{}", command);
        }
    }
}

/// **A prose `${…}` command refuses by name, naming the refusal, and never reaches the port.**
#[test]
fn selection_plan_refuses_a_prose_string_naming_the_refusal() {
    // Served (ADR-0205, proposed): a fresh prose `command` is refused at `write` as `Schema`.
    let (report, tc, _) = walk(
        vec![served_write(
            serde_json::json!({"command": PROSE_2026_11_08}),
        )],
        serde_json::json!({}),
        "${write.outputs.command}",
    );
    assert_eq!(tc.calls.load(Ordering::SeqCst), 0, "{:?}", tc.selections());
    let e = refusal(&report);
    assert!(
        e.contains("`run-tests`") && e.contains("inputs.selection") && e.contains("ShellSyntax"),
        "the refusal names the step, its selection and the refusal: {e}"
    );
}

/// **A list element with whitespace refuses by name (D5), naming the element.**
#[test]
fn selection_plan_refuses_a_list_element_with_whitespace_naming_it() {
    // Served (ADR-0205, proposed): a fresh spaced name is refused at `write` as `Schema`.
    let (report, tc, _) = walk(
        vec![served_write(
            serde_json::json!({"tests_added": ["ac601_fails_before", "run ac601 --exact"]}),
        )],
        serde_json::json!({}),
        "${write.outputs.tests_added}",
    );
    assert_eq!(tc.calls.load(Ordering::SeqCst), 0, "{:?}", tc.selections());
    let e = refusal(&report);
    assert!(
        e.contains("BadToken") && e.contains("run ac601 --exact"),
        "the refusal names the element: {e}"
    );
}

/// The error rows (`step_refused`, `step_failed`) a journal holds.
fn error_rows(events: &[rk_types::event::Event]) -> Vec<&rk_types::event::Event> {
    use rk_types::event_kind::EventKind;
    events
        .iter()
        .filter(|e| matches!(e.kind, EventKind::StepRefused | EventKind::StepFailed))
        .collect()
}

/// **Test 11 (ADR-0205, proposed; plan D7 option J1).** A tool step's refusal that ends the walk
/// after an agent commit appends exactly one `step_refused` row, carrying the step, the activation
/// and the capability text; an agent dispatch's failure keeps its one row from the dispatch loop
/// and gains no second.
#[test]
fn a_tool_step_failure_after_a_commit_appends_one_error_row() {
    use rk_types::event_kind::EventKind;
    // An empty list passes the feed check (an element may name no test) and the tool step refuses.
    let (report, tc, events) = walk(
        Vec::new(),
        serde_json::json!({"tests_added": []}),
        "${write.outputs.tests_added}",
    );
    assert!(
        report.committed() && tc.selections().is_empty(),
        "{report:?}"
    );
    let rows = error_rows(&events);
    assert_eq!(rows.len(), 1, "{events:#?}");
    let row = rows.first().expect("one row");
    assert_eq!(row.kind, EventKind::StepRefused);
    assert_eq!(
        row.payload.get("step_id"),
        Some(&serde_json::json!("run-tests"))
    );
    assert_eq!(
        row.payload.get("activity_id_at_version"),
        Some(&serde_json::json!(crate::runtime::AID))
    );
    let capability = row
        .payload
        .get("capability")
        .and_then(serde_json::Value::as_str)
        .expect("a step_refused row carries its capability");
    assert!(capability.contains("`run-tests`"), "{capability}");

    // An agent dispatch's own failure: its one row, and no second from the walk.
    let (report, _, events) = walk_answering(
        Vec::new(),
        vec![Err(StepError::Unsupported {
            capability: "the scripted refusal".to_owned(),
        })],
        "${write.outputs.tests_added}",
    );
    assert!(!report.committed(), "{report:?}");
    assert_eq!(error_rows(&events).len(), 1, "{events:#?}");
}
