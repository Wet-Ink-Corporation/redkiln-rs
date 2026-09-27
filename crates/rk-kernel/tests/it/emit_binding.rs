//! **`emit` binding** (`s8d` §8d.3; R2 = ADR-0173 decision 7, written `proposed`) and **§8d.8
//! defect 1**, a step after an `emit`.
//!
//! The `emit` step binds each key of its `artifacts` table — a reference, or a composing table as
//! `confirmation-test`'s `ConfirmationResult` is — to the provided kind, and its `evidence` list to
//! the evidence refs; that binding is what commits the activation, on its completion row, where a
//! later activation's `artifact:Kind[.field]` reads it. The walk's arm used to read
//! *"`StepVerdict::Emitted => break,`"*, dropping any later step in silence; a step after an `emit`
//! is now refused by name at load, and by the walk for a list transcribed some other way.
//!
//! *(Replaces `walk_semantics::an_emit_step_ends_the_walk_and_a_later_tool_step_never_runs`, which
//! asserted the drop as intended; `s8d` §8d.10.)*

use std::collections::BTreeMap;
use std::sync::Arc;

use rk_kernel::catalog::{Catalog, EmitArtifact, EmitBinding, InputValue, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::is_completion_row;
use rk_kernel::runtime::reference::{Env, Scope, UnwiredReadSeam, resolve_text};
use rk_kernel::runtime::step::ToolRouting;
use rk_kernel::workflow::memo::ToolchainValues;
use rk_ports::harness::Harness;
use rk_ports::toolchain::Toolchain;

use crate::reference_resolver::{activation, refusal, root, step};
use crate::runtime::{
    CAP, CountingHarness, CountingToolchain, ITEM, STAGE, at, counters, empty_state, gate_site,
    journal, ok_typed, ports, ports_with_toolchain, spec, store,
};

/// An `emit` step carrying `artifacts` and `evidence`.
fn emit_step(
    artifacts: BTreeMap<String, EmitArtifact>,
    evidence: &[&str],
) -> rk_kernel::catalog::Step {
    let mut s = step("emit", StepKind::Emit);
    s.emit = Some(EmitBinding {
        artifacts,
        evidence: evidence.iter().map(|e| (*e).to_owned()).collect(),
    });
    s
}

/// **Every provided kind binds, a composing table included, and the evidence refs with it.**
#[test]
fn emit_binds_every_provided_kind_including_a_composing_table() {
    let walk = |emit: rk_kernel::catalog::Step| {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
            serde_json::json!({
                "doc": "D", "status": "passing", "passed": 3
            }),
        )]));
        let rt = Runtime::new(ports(harness as Arc<dyn Harness>, &db), "run-emit", STAGE);
        let sp = spec();
        let mut skipped = step("skipped", StepKind::Agent);
        skipped.when = Some("${draft.outputs.doc} == 'not-D'".to_owned());
        let steps = vec![step("draft", StepKind::Agent), skipped, emit];
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
        (report, journal(&db))
    };
    let mut artifacts = BTreeMap::new();
    artifacts.insert(
        "Doc".to_owned(),
        EmitArtifact::Ref("${draft.outputs.doc}".to_owned()),
    );
    let mut members = BTreeMap::new();
    members.insert(
        "status".to_owned(),
        InputValue::Text("${draft.outputs.status}".to_owned()),
    );
    members.insert(
        "passes".to_owned(),
        InputValue::Text("${draft.outputs.passed}".to_owned()),
    );
    members.insert("runs".to_owned(), InputValue::Int(2));
    artifacts.insert("Result".to_owned(), EmitArtifact::Compose(members));
    let (report, rows) = walk(emit_step(artifacts.clone(), &["${draft}", "${skipped?}"]));
    assert!(
        report.completed() && report.step_error().is_none(),
        "the emit must bind and complete: {report:?}"
    );
    let completion: Vec<_> = rows.iter().filter(|e| is_completion_row(e)).collect();
    assert_eq!(completion.len(), 1, "one completion row: {rows:?}");
    assert_eq!(
        completion[0].payload["artifacts"],
        serde_json::json!({
            "Doc": "D",
            "Result": { "status": "passing", "passes": 3, "runs": 2 }
        }),
        "each provided kind — a reference and a composing table — is bound on the completion row"
    );
    assert_eq!(
        completion[0].payload["evidence"],
        serde_json::json!([{ "step": "draft", "kind": "agent" }]),
        "the evidence refs bind, and an absent `?` ref (a step its guard held off) is omitted"
    );
    // The binding is what a LATER activation reads: `artifact:Result.status` over this journal.
    let steps = vec![step("reader", StepKind::Agent)];
    let mut scope = Scope::new(&steps);
    scope.enter("reader");
    let values = ToolchainValues::default();
    let feed = |_: &str| -> Result<serde_json::Value, String> { Err("none".to_owned()) };
    let seam = UnwiredReadSeam;
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: "reader@1",
        events: &rows,
        values: &values,
        registry: None,
        feed_table: None,
        sources: None,
        seam: &seam,
        feed: &feed,
        run_branch: None,
    };
    assert_eq!(
        resolve_text(&env, &scope, "reader", "result", "artifact:Result.status").expect("resolves"),
        serde_json::json!("passing")
    );
    // A binding that does not resolve refuses by name, and the activation does NOT complete.
    let mut bad = artifacts;
    bad.insert(
        "Doc".to_owned(),
        EmitArtifact::Ref("${draft.outputs.missing}".to_owned()),
    );
    let (report, rows) = walk(emit_step(bad, &["${draft}"]));
    let e = refusal(&report);
    assert!(
        e.contains("`emit`") && e.contains("${draft.outputs.missing}"),
        "an unresolvable emit binding must refuse by name: {e}"
    );
    assert!(
        !rows.iter().any(is_completion_row) && !report.completed(),
        "an emit that did not bind must not complete the activation: {rows:?}"
    );
    assert!(
        report.committed(),
        "the agent step's commit stands (ADR-0144): {report:?}"
    );
}

/// **§8d.8 defect 1: a step after an `emit` is refused by name — at load, and by the walk — and is
/// never silently dropped** (M9: `Emitted => break` restored and the load refusal removed).
#[test]
fn a_step_after_emit_is_refused_by_name_and_never_silently_dropped() {
    // ---- (a) AT LOAD: the strict loader refuses the activity, naming the step and the `emit`. -----
    let src = |order: &str| {
        format!(
            "[activity]\nid = \"late\"\nversion = 1\nstage = \"implement\"\nprovides = [\"Doc\"]\n\
             method_rank = 10\n\n{order}"
        )
    };
    let emit =
        "[[step]]\nid = \"emit\"\nkind = \"emit\"\nartifacts = { Doc = \"${draft.outputs}\" }\n\n";
    let draft = "[[step]]\nid = \"draft\"\nkind = \"agent\"\nrole = \"implementer\"\n\n";
    let lint = "[[step]]\nid = \"lint\"\nkind = \"tool\"\nrun = \"toolchain:lint\"\n\n";
    // CONTROL: the same steps with the `emit` LAST load.
    Catalog::from_sources(&[("late.toml", &src(&format!("{draft}{lint}{emit}")))])
        .expect("CONTROL: an activity whose `emit` is last loads");
    let e = Catalog::from_sources(&[("late.toml", &src(&format!("{draft}{emit}{lint}")))])
        .expect_err("a step after the `emit` must be refused at load");
    let rendered = e.to_string();
    assert!(
        rendered.contains("late/lint")
            && rendered.contains("`emit`")
            && rendered.contains("terminal"),
        "the load refusal must name the activity, the later step and the `emit`: {rendered}"
    );

    // ---- (b) THE WALK: a list transcribed with a step after its `emit` is refused before anything
    // runs, and the later step is not dropped in silence. ------------------------------------------
    let walk = |steps: Vec<rk_kernel::catalog::Step>| {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
            serde_json::json!({"ok": 1}),
        )]));
        let tc = Arc::new(CountingToolchain::default());
        let rt = Runtime::new(
            ports_with_toolchain(
                Arc::clone(&harness) as Arc<dyn Harness>,
                &db,
                Arc::clone(&tc) as Arc<dyn Toolchain>,
            ),
            "run-emit-late",
            STAGE,
        )
        .with_tool_routing(ToolRouting::new(
            crate::tool_verb_routing::registry("", ""),
            ToolchainValues::default(),
        ));
        let sp = spec();
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
        (report, harness.dispatches(), tc.calls())
    };
    let tool = || {
        let mut s = step("lint", StepKind::Tool);
        s.run = Some("toolchain:lint".to_owned());
        s
    };
    // CONTROL: the tool step BEFORE the `emit` runs, and the walk completes on the `emit`.
    let (report, dispatches, calls) = walk(vec![
        step("draft", StepKind::Agent),
        tool(),
        step("emit", StepKind::Emit),
    ]);
    assert!(report.completed(), "CONTROL: {report:?}");
    assert_eq!(
        (dispatches, calls),
        (1, 1),
        "CONTROL: the agent dispatched and the tool reached the port"
    );
    // THE ASSERTION: after the `emit`, it is refused by name, before anything runs.
    let (report, dispatches, calls) = walk(vec![
        step("draft", StepKind::Agent),
        step("emit", StepKind::Emit),
        tool(),
    ]);
    let e = refusal(&report);
    assert!(
        e.contains("`lint`") && e.contains("`emit`") && e.contains("never dropped"),
        "a step after the `emit` must be refused by name: {e}"
    );
    assert_eq!(
        (dispatches, calls, report.completed()),
        (0, 0, false),
        "the refusal comes before anything runs, and nothing completes"
    );
}

/// **Measured over `.redkiln/catalog`: every activity carries exactly one `emit`, it is the LAST
/// step, and its `artifacts` keys equal `provides`** (§8d.3, *"102 of 102"*).
#[test]
fn every_shipped_activity_has_one_last_emit_whose_artifacts_equal_provides() {
    let catalog = Catalog::from_dir(&root().join(".redkiln").join("catalog"))
        .expect("the shipped catalog loads");
    let mut walked = 0_usize;
    let mut failures = Vec::new();
    for activity in catalog.activities() {
        walked += 1;
        let emits: Vec<usize> = activity
            .steps
            .iter()
            .enumerate()
            .filter(|(_, s)| s.kind == StepKind::Emit)
            .map(|(i, _)| i)
            .collect();
        if emits != vec![activity.steps.len() - 1] {
            failures.push(format!(
                "{}: `emit` at {emits:?} of {}",
                activity.id,
                activity.steps.len()
            ));
            continue;
        }
        let emit = &activity.steps[emits[0]];
        let keys: Vec<&str> = emit
            .emit
            .as_ref()
            .map(|b| b.artifacts.keys().map(String::as_str).collect())
            .unwrap_or_default();
        let mut provides: Vec<&str> = activity.provides.iter().map(String::as_str).collect();
        provides.sort_unstable();
        if keys != provides {
            failures.push(format!(
                "{}: artifacts {keys:?} != provides {provides:?}",
                activity.id
            ));
        }
    }
    println!(
        "EMIT_BINDING activities={walked} failures={}",
        failures.len()
    );
    assert_eq!(
        walked, 102,
        "the census walked {walked} activities, not the shipped 102"
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
