//! **The complete reference resolver** — `rk_kernel::runtime::reference` (`s8d` §8d.3; R2 =
//! ADR-0173, written `proposed`).
//!
//! D21-11: *"It does **not** license a partial resolver — a lookup for `${x.answer}` but not
//! `${x.outputs.y}`"*. So the first test here is table-driven over EVERY form of ADR-0040's closed
//! set, each with both outcomes (it resolves; it refuses by name), and the others pin the forms one
//! at a time through the resolver and through the walk. The iteration forms (`for_each`, `repeat`,
//! a loop-back's previous frame) are pinned by this module's iteration tests, which the loop
//! executor's package adds; the table below exercises their resolver half only, over a scope the
//! test binds by hand ([`Scope::bind_previous`], [`Scope::set_element`]).
//!
//! 🛑 **Every refusal is asserted to name the step id AND the reference** (ADR-0136's forbids), and
//! never to read as `null`, `false` or a skip.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Catalog, InputValue, Step, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::reference::{
    ArtifactRead, ArtifactReadSeam, Env, GateAnswer, Scope, UnwiredReadSeam, parse_source, resolve,
    resolve_text, step_names,
};
use rk_kernel::runtime::step::{Activation, StepVerdict, ToolRouting};
use rk_kernel::workflow::memo::{ToolchainValues, VerbRegistry};
use rk_ports::harness::Harness;
use rk_ports::toolchain::{Toolchain, VerbOutput};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::runtime::{
    AID, CAP, CountingHarness, CountingToolchain, ITEM, STAGE, STEP, at, counters, empty_state,
    gate_site, journal, ok_typed, ports, ports_with_toolchain, spec, store,
};

type Json = serde_json::Value;

// ---- shared fixtures (also used by `check_eval`, `emit_binding` and `test_selection`) ----------

/// A step of `kind` with nothing declared.
pub fn step(id: &str, kind: StepKind) -> Step {
    Step {
        id: id.to_owned(),
        kind,
        run: None,
        output_type: None,
        role: None,
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: None,
        inputs: BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    }
}

/// `step` with `inputs`, each a source string.
pub fn with_inputs(mut s: Step, inputs: &[(&str, &str)]) -> Step {
    for (k, v) in inputs {
        s.inputs
            .insert((*k).to_owned(), InputValue::Text((*v).to_owned()));
    }
    s
}

/// A `check` step reading `inputs` under `expr`.
pub fn check(id: &str, inputs: &[(&str, &str)], expr: &str) -> Step {
    let mut s = with_inputs(step(id, StepKind::Check), inputs);
    s.expr = Some(expr.to_owned());
    s
}

/// A completion row carrying an `emit` binding of `kind` for [`ITEM`], as a producing activation
/// leaves it (R2 decision 7): what `artifact:Kind[.field]` reads.
pub fn emitted_row(producer: &str, kind: &str, value: Json) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": producer,
            "attempt": 1,
            "completion": true,
            "item": ITEM,
            "iteration_path": [],
            "run_id": "run-producer",
            "step_id": "emit",
            "at": 50,
            "artifacts": { kind: value },
            "evidence": [],
        }),
    )
}

/// A read seam that COUNTS its calls and records what each read.
#[derive(Default)]
pub struct CountingSeam {
    calls: AtomicU32,
    reads: Mutex<Vec<(String, String, String)>>,
}

impl CountingSeam {
    pub fn calls(&self) -> u32 {
        self.calls.load(Ordering::SeqCst)
    }

    pub fn reads(&self) -> Vec<(String, String, String)> {
        self.reads.lock().expect("not poisoned").clone()
    }
}

impl ArtifactReadSeam for CountingSeam {
    fn read(&self, read: &ArtifactRead<'_>) -> Result<(), String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.reads.lock().expect("not poisoned").push((
            read.kind.to_owned(),
            read.reader_step.to_owned(),
            read.binding.to_owned(),
        ));
        Ok(())
    }
}

/// The workspace root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

fn no_feed(verb: &str) -> Result<Json, String> {
    Err(format!("no feed is bound in this test (`{verb}`)"))
}

/// The value table every table-driven case here reads.
fn values() -> ToolchainValues {
    let reg = VerbRegistry::from_toml(
        "[verb]\nlint = { outputs = \"LintReport\", live = false, timeout = \"scan_timeout\", \
         inputs = [], route = \"port\", side_effecting = false }\n\n[toolchain.t]\ntest_timeout = \
         { secs = 600, measured = [300], rule = \"r\", pin = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", source = \"test\" }\n",
    )
    .expect("the test registry loads");
    reg.values_for("t").expect("the table loads")
}

/// The refusal's rendering, asserted to name the step and the reference.
fn refused(
    result: Result<Json, rk_kernel::runtime::reference::RefError>,
    step: &str,
    reference: &str,
) -> String {
    let e = result.expect_err("the reference must REFUSE, not resolve");
    let rendered = e.to_string();
    assert!(
        rendered.contains(&format!("`{step}`")) && rendered.contains(reference),
        "a refusal must name the step id AND the reference (ADR-0136's forbids): {rendered}"
    );
    rendered
}

// =================================================================================================

/// **Every ADR-0040 form, both outcomes** — the completeness guard D21-11 asks for.
#[test]
fn every_input_source_form_resolves_or_refuses_by_name() {
    let steps = vec![
        step("draft", StepKind::Agent),
        step("fan", StepKind::Agent),
        step("g", StepKind::Gate),
        step("size", StepKind::Agent),
        step("resize", StepKind::Agent),
        step("me", StepKind::Agent),
        step("later", StepKind::Agent),
    ];
    let mut scope = Scope::new(&steps);
    scope.bind_output("draft", serde_json::json!({"title": "T", "items": [1, 2]}));
    scope.bind_output(
        "fan",
        serde_json::json!([{"tests_added": ["a"]}, {"tests_added": ["b", "c"]}]),
    );
    scope.bind_output("size", serde_json::json!("M"));
    let mut attachments = BTreeMap::new();
    attachments.insert("Spec".to_owned(), serde_json::json!({"digest": "d"}));
    scope.bind_answer(
        "g",
        GateAnswer {
            option: "proceed".to_owned(),
            attachments,
        },
    );
    scope.enter("me");

    // `kb:` / `checklist:` / `toolchain:<feed>` through the SHIPPED registry and its tables.
    let shipped = VerbRegistry::load_file(&root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry loads");
    let sources = root().join(".redkiln");
    let events = vec![emitted_row(
        "spec-author@1",
        "Spec",
        serde_json::json!({"title": "the spec"}),
    )];
    let seam = CountingSeam::default();
    let values = values();
    let feed = |verb: &str| -> Result<Json, String> {
        if verb == "logfeed" {
            Ok(serde_json::json!({"lines": 3}))
        } else {
            no_feed(verb)
        }
    };
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: AID,
        events: &events,
        values: &values,
        registry: Some(&shipped),
        feed_table: Some("rust"),
        sources: Some(&sources),
        seam: &seam,
        feed: &feed,
        run_branch: None,
    };
    let r = |text: &str| resolve_text(&env, &scope, "me", "x", text);

    // ---- `${<step>.outputs[.path]}`, `${<step>}` -------------------------------------------------
    assert_eq!(
        r("${draft.outputs}").unwrap(),
        serde_json::json!({"title": "T", "items": [1, 2]})
    );
    assert_eq!(r("${draft.outputs.title}").unwrap(), serde_json::json!("T"));
    assert_eq!(
        r("${draft.outputs.items[1]}").unwrap(),
        serde_json::json!(2)
    );
    assert_eq!(
        r("${draft}").unwrap(),
        serde_json::json!({"step": "draft", "kind": "agent"})
    );
    refused(r("${nope.outputs}"), "me", "${nope.outputs}");
    refused(
        r("${draft.outputs.missing}"),
        "me",
        "${draft.outputs.missing}",
    );
    assert_eq!(r("${draft.outputs.missing?}").unwrap(), Json::Null);
    // ---- a self / later-step ref reads the PREVIOUS frame, `null` first (ADR-0055, ADR-0042) -----
    assert_eq!(r("${later.outputs?}").unwrap(), Json::Null);
    refused(r("${later.outputs}"), "me", "${later.outputs}");
    assert_eq!(r("${me.outputs?}").unwrap(), Json::Null);
    let mut prev = scope.clone();
    prev.bind_previous("me", serde_json::json!("last pass"));
    assert_eq!(
        resolve_text(&env, &prev, "me", "x", "${me.outputs?}").unwrap(),
        serde_json::json!("last pass")
    );
    // ---- `??` (ADR-0034) --------------------------------------------------------------------------
    assert_eq!(
        r("${resize.outputs?} ?? ${size.outputs}").unwrap(),
        serde_json::json!("M")
    );
    refused(r("${resize.outputs?} ?? ${resize.outputs}"), "me", "??");
    // ---- a gate's answer and attachments; `.outputs` on a gate refuses (ADR-0045) -----------------
    assert_eq!(r("${g.answer}").unwrap(), serde_json::json!("proceed"));
    assert_eq!(
        r("${g.attachments.Spec}").unwrap(),
        serde_json::json!({"digest": "d"})
    );
    refused(r("${g.attachments.Plan}"), "me", "${g.attachments.Plan}");
    let gate_outputs = refused(r("${g.outputs}"), "me", "${g.outputs}");
    assert!(gate_outputs.contains("ADR-0045"), "{gate_outputs}");
    refused(r("${draft.answer}"), "me", "${draft.answer}");
    // ---- `${item}` ---------------------------------------------------------------------------------
    refused(r("${item}"), "me", "${item}");
    let mut element = scope.clone();
    element.set_element(Some(serde_json::json!("AC-1")));
    assert_eq!(
        resolve_text(&env, &element, "me", "x", "${item}").unwrap(),
        serde_json::json!("AC-1")
    );
    // ---- `[*]` (ADR-0035; D21-48 X5: concatenation in element order) ------------------------------
    assert_eq!(
        r("${fan.outputs[*].tests_added}").unwrap(),
        serde_json::json!(["a", "b", "c"])
    );
    refused(
        r("${draft.outputs[*].title}"),
        "me",
        "${draft.outputs[*].title}",
    );
    // ---- `${toolchain.<key>}` (D21-42 4a) ----------------------------------------------------------
    assert_eq!(
        r("${toolchain.test_timeout}").unwrap(),
        serde_json::json!(600)
    );
    refused(
        r("${toolchain.no_such_key}"),
        "me",
        "${toolchain.no_such_key}",
    );
    // ---- `artifact:Kind[.field]` -------------------------------------------------------------------
    assert_eq!(
        r("artifact:Spec.title").unwrap(),
        serde_json::json!("the spec")
    );
    refused(r("artifact:Plan"), "me", "artifact:Plan");
    assert_eq!(r("artifact:Plan?").unwrap(), Json::Null);
    // ---- `kb:`, `checklist:`, `toolchain:<feed>` (N14) --------------------------------------------
    let kb = r("kb:spec-conventions").unwrap();
    assert_eq!(kb["path"], serde_json::json!("kb/spec-conventions.md"));
    refused(r("kb:no-such-atom?"), "me", "kb:no-such-atom?");
    assert_eq!(r("checklist:smoke?").unwrap(), Json::Null);
    refused(r("checklist:smoke"), "me", "checklist:smoke");
    assert_eq!(r("toolchain:logs?").unwrap(), Json::Null);
    refused(r("toolchain:backlog"), "me", "toolchain:backlog");
    // The RESOLVING outcome of `toolchain:<feed>` (R3FIX-8d, refuting round 3's minor): every shipped
    // `[feed.rust.*]` row is `unprovided`, so the shipped registry cannot reach it. A registry that
    // declares a feed WITH a producing verb resolves the key to what `env.feed` returns for that verb,
    // which is the `logfeed` arm of `feed` above, and to nothing else.
    let feed_registry = VerbRegistry::from_toml(
        "[verb]\nlogfeed = { outputs = \"SmokeResult\", live = true, timeout = \"deploy_timeout\", \
         inputs = [], route = \"toolchain\", side_effecting = false }\n\n[feed.t.logs]\nverb = \
         \"logfeed\"\n\n[toolchain.t]\ndeploy_timeout = { secs = 60, measured = [30], rule = \"r\", pin = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", source = \"test\" }\n",
    )
    .expect("the feed registry loads");
    let feed_env = Env {
        registry: Some(&feed_registry),
        feed_table: Some("t"),
        ..env
    };
    assert_eq!(
        resolve_text(&feed_env, &scope, "me", "x", "toolchain:logs").unwrap(),
        serde_json::json!({"lines": 3}),
        "a declared feed with a producing verb resolves to that verb's value"
    );
    // ---- literals ----------------------------------------------------------------------------------
    assert_eq!(r("affected").unwrap(), serde_json::json!("affected"));
    assert_eq!(
        resolve(&env, &scope, "me", "n", &InputValue::Int(3)).unwrap(),
        serde_json::json!(3)
    );
    // ---- outside the grammar: refused by name -----------------------------------------------------
    refused(
        r("baseline..${draft.outputs}"),
        "me",
        "baseline..${draft.outputs}",
    );
    refused(r("${draft.bogus}"), "me", "${draft.bogus}");
}

/// **A prior step's output binds by step name and field path**, through the walk: the agent step's
/// committed value is read back from its journalled row and a later `check` binds it.
#[test]
fn a_prior_step_output_binds_by_step_name_and_field_path() {
    let walk = |expr: &str, field: &str| {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
            serde_json::json!({"title": "T", "items": ["a", "b"]}),
        )]));
        let rt = Runtime::new(ports(harness as Arc<dyn Harness>, &db), "run-prior", STAGE);
        let sp = spec();
        let steps = vec![
            step("draft", StepKind::Agent),
            check(
                "reads",
                &[
                    ("t", &format!("${{draft.outputs.{field}}}")),
                    ("n", "${draft.outputs.items}"),
                ],
                expr,
            ),
        ];
        rt.run_activity_steps(
            activation(&sp),
            Some(&steps),
            &rk_kernel::predicate::Context::new(),
            gate_site(&empty_state()),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE (AC-032)")
    };
    // The value is the dispatch's, by step name and field path.
    let report = walk("t == 'T' and n.len() == 2", "title");
    assert!(
        report.step_error().is_none() && report.completed(),
        "`${{draft.outputs.title}}` and `${{draft.outputs.items}}` must bind the committed value: \
         {report:?}"
    );
    // CONTROL: the same check against another value does not hold, so the green above DECIDED.
    let report = walk("t == 'X'", "title");
    assert!(
        refusal(&report).contains("did not hold"),
        "CONTROL: a check reading another value must not hold: {report:?}"
    );
    // A field the output does not carry refuses by name, naming the step and the reference.
    let report = walk("t == 'T'", "subtitle");
    let e = refusal(&report);
    assert!(
        e.contains("`reads`") && e.contains("${draft.outputs.subtitle}") && e.contains("no field"),
        "a missing field must refuse by name: {e}"
    );
}

/// A walk's refusal, rendered whichever channel carries it: `step_error` after a commit (ADR-0144),
/// the outcome's `Err` before one.
pub fn refusal(report: &rk_kernel::runtime::activity::ActivityReport) -> String {
    format!("{:?} {:?}", report.step_error(), report.outcome())
}

/// The activation the walk tests here share.
pub const fn activation(sp: &rk_types::activity::ActivitySpec) -> Activation<'_> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: Some(sp),
        step_specs: None,
        rigor: rk_kernel::rigor::Rigor::Optional,
    }
}

/// **ADR-0032: an absent `${…?}` is `null`; a mandatory ref to a step that has not run refuses by
/// name** — here a step the guard held off.
#[test]
fn an_optional_reference_is_null_and_a_mandatory_one_refuses_when_its_step_has_not_run() {
    let walk = |input: &str, expr: &str| {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(CountingHarness::scripted(Vec::new()));
        let rt = Runtime::new(
            ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
            "run-optional",
            STAGE,
        );
        let sp = spec();
        let mut guarded = step("a", StepKind::Agent);
        guarded.when = Some("rigor >= mandatory".to_owned());
        let steps = vec![guarded, check("c", &[("x", input)], expr)];
        let registry = rk_kernel::registry::Registry::from_toml(
            &std::fs::read_to_string(root().join(".redkiln").join("facets.toml"))
                .expect("the shipped facets read"),
        )
        .expect("the shipped facets load");
        let ctx = rk_kernel::rigor::with_rigor(
            rk_kernel::predicate::Context::new(),
            rk_kernel::rigor::Rigor::Optional,
            &registry,
        );
        let report = rt
            .run_activity_steps(
                activation(&sp),
                Some(&steps),
                &ctx,
                gate_site(&empty_state()),
                rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
                at(100),
            )
            .expect("a value");
        assert_eq!(harness.dispatches(), 0, "PREMISE: the guard held `a` off");
        report
    };
    // `${a.outputs?}` over a step that did not run is `null`, and `x == null` reads it as absent.
    let report = walk("${a.outputs?}", "x == null");
    assert!(
        report.step_error().is_none() && report.completed(),
        "an optional ref to a step that has not run must bind `null` (ADR-0032): {report:?}"
    );
    // The same field read off it is `null`, never an unknown field.
    let report = walk("${a.outputs?}", "x.status == null");
    assert!(
        report.completed(),
        "a field of an absent optional input must read `null`: {report:?}"
    );
    // The mandatory spelling refuses by name: the step id and the reference.
    let report = walk("${a.outputs}", "x == null");
    let e = refusal(&report);
    assert!(
        e.contains("`c`") && e.contains("${a.outputs}") && e.contains("has not run"),
        "a mandatory ref to a step that has not run must refuse by name: {e}"
    );
    assert!(
        !e.contains("did not hold") && !e.contains("abstained"),
        "an unresolvable ref is never read as false or null: {e}"
    );
}

/// **ADR-0034: `${a?} ?? ${b}` is the left operand unless it is `null`** (M25).
#[test]
fn a_coalesce_takes_its_left_operand_unless_it_is_null() {
    let steps = vec![
        step("size", StepKind::Agent),
        step("resize", StepKind::Agent),
        step("emit", StepKind::Emit),
    ];
    let values = ToolchainValues::default();
    let feed = no_feed;
    let seam = UnwiredReadSeam;
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: AID,
        events: &[],
        values: &values,
        registry: None,
        feed_table: None,
        sources: None,
        seam: &seam,
        feed: &feed,
        run_branch: None,
    };
    // `appetite-check`'s shipped spelling, verbatim.
    let text = "${resize.outputs?} ?? ${size.outputs}";
    let mut scope = Scope::new(&steps);
    scope.enter("emit");
    scope.bind_output("size", serde_json::json!({"size": "M"}));
    // The left operand is absent: the right one.
    assert_eq!(
        resolve_text(&env, &scope, "emit", "AppetiteEstimate", text).unwrap(),
        serde_json::json!({"size": "M"}),
        "an absent left operand takes the right one"
    );
    // The left operand is present: IT, not the right one (M25 takes the right one always).
    scope.bind_output("resize", serde_json::json!({"size": "S"}));
    assert_eq!(
        resolve_text(&env, &scope, "emit", "AppetiteEstimate", text).unwrap(),
        serde_json::json!({"size": "S"}),
        "a present left operand must be taken over the right one (ADR-0034)"
    );
    // The right operand resolves like any other: absent and mandatory, it refuses by name.
    let empty = {
        let mut s = Scope::new(&steps);
        s.enter("emit");
        s
    };
    let e = resolve_text(&env, &empty, "emit", "AppetiteEstimate", text)
        .expect_err("an absent mandatory right operand refuses");
    assert!(
        e.to_string().contains("`emit`") && e.to_string().contains("has not run"),
        "{e}"
    );
}

/// **ADR-0045: a gate step binds `answer` and never `outputs`**, through the walk: the answered
/// gate is walked past and a later check reads its answer; `.outputs` on it refuses by name.
#[test]
fn a_gate_step_binds_answer_and_never_outputs() {
    let history = {
        let record = rk_kernel::workflow::gate::reach(
            &empty_state(),
            "draft.accept",
            &rk_kernel::workflow::fsm::FsmContext::supplied(
                ITEM,
                crate::runtime::TREE_HASH,
                rk_types::time::Timestamp::from_unix_seconds(9_000),
                None,
            ),
        )
        .expect("the record derives");
        vec![
            rk_kernel::runtime::gate_opened_event(&record),
            rk_kernel::workflow::gate::gate_answered_event(
                record.token(),
                crate::runtime::TREE_HASH,
                "proceed",
            ),
        ]
    };
    let state = rk_kernel::workflow::fsm::fold(ITEM, STAGE, &history).expect("folds");
    let walk = |input: &str, expr: &str| {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(CountingHarness::scripted(Vec::new()));
        let rt = Runtime::new(
            ports(harness as Arc<dyn Harness>, &db),
            "run-gate-answer",
            STAGE,
        );
        let sp = spec();
        let mut gate = step("accept", StepKind::Gate);
        gate.question = Some("draft.accept".to_owned());
        let steps = vec![gate, check("reads", &[("a", input)], expr)];
        rt.run_activity_steps(
            activation(&sp),
            Some(&steps),
            &rk_kernel::predicate::Context::new(),
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a value")
    };
    let report = walk("${accept.answer}", "a == 'proceed'");
    assert!(
        report.completed() && report.gate_opened().is_none(),
        "the answered gate's `${{accept.answer}}` must bind its option: {report:?}"
    );
    // CONTROL: another option does not hold, so the green above read the real answer.
    let report = walk("${accept.answer}", "a == 'reject'");
    assert!(
        refusal(&report).contains("did not hold"),
        "CONTROL: {report:?}"
    );
    // `.outputs` on a gate refuses by name, naming ADR-0045.
    let report = walk("${accept.outputs}", "a == 'proceed'");
    let e = refusal(&report);
    assert!(
        e.contains("`reads`") && e.contains("${accept.outputs}") && e.contains("ADR-0045"),
        "`.outputs` on a gate must refuse by name: {e}"
    );
}

/// **Every reference the shipped catalog declares is in the resolver's grammar**, over a non-zero
/// count walked (D21-42 13a's rule), and every step it names is a step of its activity.
#[test]
fn every_shipped_step_reference_is_in_the_resolver_grammar() {
    let catalog = Catalog::from_dir(&root().join(".redkiln").join("catalog"))
        .expect("the shipped catalog loads");
    let mut walked = 0_usize;
    let mut failures = Vec::new();
    for activity in catalog.activities() {
        let ids: Vec<&str> = activity.steps.iter().map(|s| s.id.as_str()).collect();
        for s in &activity.steps {
            let mut texts: Vec<String> = s
                .inputs
                .values()
                .filter_map(InputValue::as_text)
                .map(str::to_owned)
                .collect();
            texts.extend(s.for_each.iter().cloned());
            if let Some(emit) = &s.emit {
                for a in emit.artifacts.values() {
                    match a {
                        rk_kernel::catalog::EmitArtifact::Ref(t) => texts.push(t.clone()),
                        rk_kernel::catalog::EmitArtifact::Compose(m) => texts.extend(
                            m.values()
                                .filter_map(InputValue::as_text)
                                .map(str::to_owned),
                        ),
                    }
                }
                texts.extend(emit.evidence.iter().cloned());
            }
            for pred in [
                s.when.as_deref(),
                s.repeat_until.as_deref(),
                s.expr.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                let parsed = rk_kernel::predicate::parse(pred).unwrap_or_else(|e| {
                    panic!("{}/{}: {pred} does not parse: {e:?}", activity.id, s.id)
                });
                collect_refs(&parsed.expr, &mut texts);
            }
            for text in texts {
                walked += 1;
                match parse_source(&text) {
                    Ok(src) => {
                        for name in step_names(&src) {
                            if !ids.contains(&name) {
                                failures.push(format!(
                                    "{}/{}: `{text}` names `{name}`, no step of the activity",
                                    activity.id, s.id
                                ));
                            }
                        }
                    }
                    Err(e) => failures.push(format!("{}/{}: {e}", activity.id, s.id)),
                }
            }
        }
    }
    println!(
        "RESOLVER_GRAMMAR refs={walked} activities={}",
        catalog.len()
    );
    assert!(
        walked > 1_000 && catalog.len() == 102,
        "the census walked {walked} references over {} activities — a census over (nearly) nothing \
         is Halt 8",
        catalog.len()
    );
    assert!(
        failures.is_empty(),
        "{} references outside the grammar:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

fn collect_refs(expr: &rk_kernel::predicate::Expr, out: &mut Vec<String>) {
    use rk_kernel::predicate::Expr;
    match expr {
        Expr::Ref(r) => out.push(r.clone()),
        Expr::Or(a, b) | Expr::And(a, b) => {
            collect_refs(a, out);
            collect_refs(b, out);
        }
        Expr::Cmp { left, right, .. } => {
            collect_refs(left, out);
            collect_refs(right, out);
        }
        Expr::In { left, list } => {
            collect_refs(left, out);
            collect_refs(list, out);
        }
        Expr::Member { literal, set } => {
            collect_refs(literal, out);
            collect_refs(set, out);
        }
        Expr::Not(a) | Expr::Group(a) | Expr::Len(a) => collect_refs(a, out),
        Expr::List(items) => items.iter().for_each(|i| collect_refs(i, out)),
        Expr::Quant { source, body, .. } => {
            collect_refs(source, out);
            collect_refs(body, out);
        }
        Expr::Path(_) | Expr::Lit(_) => {}
    }
}

/// The CODE lines of a Rust source naming `needle`: `//`, `/* … */` and doc lines dropped, string
/// literals KEPT (a literal naming it fails the floor, which is the safe direction).
fn code_lines_naming(text: &str, needle: &str) -> Vec<String> {
    let mut in_block = false;
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if in_block {
            if trimmed.contains("*/") {
                in_block = false;
            }
            continue;
        }
        if trimmed.starts_with("/*") {
            in_block = !trimmed.contains("*/");
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        let code = line.split(" //").next().unwrap_or(line);
        if code.contains(needle) {
            out.push(line.to_owned());
        }
    }
    out
}

/// **ADR-0136's forbids: `null_out_refs` is never called on the execution path.** A code-lines-only
/// lint over all of `crates/rk-kernel/src` except `rigor.rs`, whose static lint's use is correct
/// (m11), with a LIVE control: the same reading sees a planted call, and sees `rigor.rs`'s own.
#[test]
fn null_out_refs_has_no_execution_path_caller() {
    // LIVE CONTROL 1: a planted call is seen, a comment naming it is not.
    let planted =
        "fn f() {\n    // null_out_refs(e) is not a call\n    let v = null_out_refs(&e);\n}\n";
    assert_eq!(
        code_lines_naming(planted, "null_out_refs").len(),
        1,
        "CONTROL: the lint must see exactly the planted code line"
    );
    // LIVE CONTROL 2: the reading keeps `rigor.rs`'s real code lines, so an empty result below is
    // not a reader that sees nothing.
    let src = root().join("crates").join("rk-kernel").join("src");
    let rigor = std::fs::read_to_string(src.join("rigor.rs")).expect("rigor.rs reads");
    assert!(
        !code_lines_naming(&rigor, "null_out_refs").is_empty(),
        "CONTROL: rigor.rs's own `null_out_refs` code lines are not seen"
    );
    let mut files = 0_usize;
    let mut hits = Vec::new();
    let mut stack = vec![src.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the source dir reads") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|x| x == "rs")
                && path.file_name().is_some_and(|n| n != "rigor.rs")
            {
                files += 1;
                let text = std::fs::read_to_string(&path).expect("a source file reads");
                for line in code_lines_naming(&text, "null_out_refs") {
                    hits.push(format!("{}: {line}", path.display()));
                }
            }
        }
    }
    assert!(
        files > 50,
        "the lint read {files} files — the walk is vacuous"
    );
    assert!(
        src.join("runtime").join("reference.rs").is_file(),
        "the resolver's home, runtime/reference.rs, is not where this lint reads (m11)"
    );
    assert!(
        hits.is_empty(),
        "`null_out_refs` on an execution-path CODE line (ADR-0136's forbids):\n{}",
        hits.join("\n")
    );
}

/// **N12: every resolved `artifact:` input calls the read seam ONCE; a `null` optional and a refusal
/// call it not at all; and this stage's stub appends no `artifact.read` row** (D21-47) — M19.
#[test]
fn every_resolved_artifact_input_calls_the_read_seam_once_and_the_stub_emits_no_row() {
    // ---- (a) through the resolver, over a counting double. ----------------------------------------
    let events = vec![emitted_row(
        "spec-author@1",
        "Spec",
        serde_json::json!({"title": "the spec", "acs": ["AC-1"]}),
    )];
    let steps = vec![step("draft", StepKind::Agent)];
    let mut scope = Scope::new(&steps);
    scope.enter("draft");
    let values = ToolchainValues::default();
    let feed = no_feed;
    let seam = CountingSeam::default();
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: AID,
        events: &events,
        values: &values,
        registry: None,
        feed_table: None,
        sources: None,
        seam: &seam,
        feed: &feed,
        run_branch: None,
    };
    let r = |text: &str| resolve_text(&env, &scope, "draft", "spec", text);
    assert_eq!(
        r("artifact:Spec").unwrap()["title"],
        serde_json::json!("the spec")
    );
    assert_eq!(seam.calls(), 1, "one resolved input, one seam call");
    assert_eq!(r("artifact:Spec.acs").unwrap(), serde_json::json!(["AC-1"]));
    assert_eq!(
        seam.calls(),
        2,
        "a second resolved input, a second call — never two per input"
    );
    assert_eq!(r("artifact:Plan?").unwrap(), Json::Null);
    assert_eq!(r("artifact:Spec.nope?").unwrap(), Json::Null);
    assert_eq!(
        seam.calls(),
        2,
        "an optional input that resolves `null` makes no call"
    );
    assert!(r("artifact:Plan").is_err());
    assert_eq!(seam.calls(), 2, "a refused input makes no call");
    assert_eq!(
        seam.reads(),
        vec![
            ("Spec".to_owned(), "draft".to_owned(), "spec".to_owned()),
            ("Spec".to_owned(), "draft".to_owned(), "spec".to_owned()),
        ],
        "each call names the kind, the reading step and its input"
    );

    // ---- (b) through a walk, over the runtime's DEFAULT seam, the non-emitting stub. ---------------
    let walk = |seam: Option<Arc<dyn ArtifactReadSeam>>| {
        let (q, a) = counters();
        let db = store(&q, &a);
        db.lock()
            .expect("not poisoned")
            .append(events[0].clone())
            .expect("seeded");
        let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
            serde_json::json!({"ok": true}),
        )]));
        let mut rt = Runtime::new(
            ports(harness as Arc<dyn Harness>, &db),
            "run-read-seam",
            STAGE,
        );
        if let Some(seam) = seam {
            rt = rt.with_read_seam(seam);
        }
        let sp = spec();
        let steps = vec![with_inputs(
            step("draft", StepKind::Agent),
            &[("spec", "artifact:Spec")],
        )];
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
    let counting = Arc::new(CountingSeam::default());
    let (report, _) = walk(Some(Arc::clone(&counting) as Arc<dyn ArtifactReadSeam>));
    assert!(
        report.committed(),
        "the walk dispatched and committed: {report:?}"
    );
    assert_eq!(
        counting.calls(),
        1,
        "the walk's one `artifact:` input made one seam call"
    );
    let (report, rows) = walk(None);
    assert!(report.committed(), "{report:?}");
    assert_eq!(
        rows.iter()
            .filter(|e| e.kind == EventKind::ArtifactRead)
            .count(),
        0,
        "this stage's stub, `UnwiredReadSeam`, appends NO `artifact.read` row: `s8f`'s rk-store seam \
         is the only emitter (D21-47)"
    );
}

/// **N14: `kb:`, `checklist:` and `toolchain:<feed>` resolve through `s8c`'s declared tables, and an
/// undeclared key refuses by name EVEN WITH `?`** (m12) — M21, M29.
#[test]
fn kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name()
 {
    let shipped = VerbRegistry::load_file(&root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry loads");
    let sources = root().join(".redkiln");
    let values = ToolchainValues::default();
    let feed = no_feed;
    let seam = UnwiredReadSeam;
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: AID,
        events: &[],
        values: &values,
        registry: Some(&shipped),
        feed_table: Some("rust"),
        sources: Some(&sources),
        seam: &seam,
        feed: &feed,
        run_branch: None,
    };
    let steps = vec![step("draft", StepKind::Agent)];
    let scope = Scope::new(&steps);
    let r = |text: &str| resolve_text(&env, &scope, "draft", "x", text);

    // A declared atom and a declared checklist resolve to their CONTENT and its digest.
    for (key, path) in [
        ("kb:spec-conventions", "kb/spec-conventions.md"),
        ("checklist:fix-scope", "checklists/fix-scope.toml"),
    ] {
        let bytes = std::fs::read(sources.join(path)).expect("the declared file reads");
        let v = r(key).unwrap_or_else(|e| panic!("{key} must resolve: {e}"));
        assert_eq!(
            v["content"],
            serde_json::json!(String::from_utf8(bytes.clone()).unwrap()),
            "{key}"
        );
        assert!(!bytes.is_empty(), "PREMISE: {path} carries content");
        assert_eq!(
            v["digest"],
            serde_json::json!(rk_types::digest::sha256(&bytes).as_str()),
            "{key}'s content digest"
        );
    }
    // An UNDECLARED key refuses by name — mandatory, and with `?` (m12; M21 read empty content).
    for key in [
        "kb:no-such-atom",
        "kb:no-such-atom?",
        "checklist:no-such-list?",
        "toolchain:no-such-feed?",
    ] {
        let e = refused(r(key), "draft", key.trim_end_matches('?'));
        assert!(e.contains("not declared"), "{key}: {e}");
    }
    // A DECLARED key with no content: `?` makes it `null`; mandatory refuses by name.
    assert_eq!(
        r("checklist:smoke?").unwrap(),
        Json::Null,
        "an `unauthored` optional is null"
    );
    assert!(refused(r("checklist:smoke"), "draft", "checklist:smoke").contains("unauthored"));
    assert_eq!(
        r("toolchain:backlog?").unwrap(),
        Json::Null,
        "an `unprovided` optional is null"
    );
    assert!(refused(r("toolchain:backlog"), "draft", "toolchain:backlog").contains("unprovided"));

    // A feed WITH a producing verb resolves by calling it through the port, via the runtime.
    let registry = VerbRegistry::from_toml(
        "[verb]\nlogfeed = { outputs = \"SmokeResult\", live = true, timeout = \"deploy_timeout\", \
         inputs = [], route = \"toolchain\", side_effecting = false }\n\n[feed.t.logs]\nverb = \
         \"logfeed\"\n\n[toolchain.t]\ndeploy_timeout = { secs = 60, measured = [30], rule = \"r\", pin = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", source = \"test\" }\n",
    )
    .expect("the test registry loads");
    let values = registry.values_for("t").expect("the table");
    let (q, a) = counters();
    let db = store(&q, &a);
    let tc = Arc::new(CountingToolchain::with_verb_output(VerbOutput {
        kind: "SmokeResult".to_owned(),
        value: crate::tool_verb_routing::smoke_value(true),
    }));
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::new(CountingHarness::scripted(Vec::new())) as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        "run-feed",
        STAGE,
    )
    .with_tool_routing(ToolRouting::new(registry, values).with_sources(Some("t".to_owned()), None));
    let c = check(
        "reads-logs",
        &[("logs", "toolchain:logs")],
        "logs.status == 'passing'",
    );
    let steps = vec![c.clone()];
    let v = rt
        .execute_step_in(
            &c,
            &rk_kernel::predicate::Context::new(),
            &Scope::new(&steps),
        )
        .expect("a value");
    assert_eq!(
        v,
        StepVerdict::Continue,
        "the feed's producing verb's value must bind: {v:?}"
    );
    assert_eq!(
        tc.calls(),
        1,
        "the feed was produced by ONE call through the port"
    );
}

// =================================================================================================
// Iteration (`s8d` §8d.3's *Iteration*, the repeat body, the attempt charge per frame and the
// escalate re-entry; R2 = ADR-0173 decisions 1, 4 and 5; R3; package P5). Each walks a real
// `Runtime` over `on_false_exec`'s bounded doubles and reads the frames off the JOURNAL.
// =================================================================================================

use crate::on_false_exec::{Rig, agent, check_on_false, gate, repeating, tool};

/// The step's committed frames as the journal holds them, e.g. `["repeat:1", "repeat:2"]`.
fn frames(rig: &Rig, step_id: &str) -> Vec<String> {
    rig.paths(step_id)
}

/// **ADR-0055: a self-reference inside a loop binds the PREVIOUS iteration's output, and is `null`
/// on the first pass** (M6 binds the current iteration's, which the new pass has not produced).
#[test]
fn a_self_reference_in_a_loop_binds_the_previous_iteration_and_is_null_first() {
    let steps = vec![
        repeating(
            agent("attempt", &[("prev", "${attempt.outputs?}")]),
            "${attempt.outputs.status} == 'done'",
            3,
            Some("fail"),
            None,
        ),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"status": "no", "n": 1}),
            serde_json::json!({"status": "no", "n": 2}),
            serde_json::json!({"status": "done", "n": 3}),
        ],
        Vec::new(),
    );
    let report = rig.walk(&steps);
    assert!(
        report.completed() && report.step_error().is_none(),
        "the repeat must run until `until` holds: {}",
        crate::on_false_exec::refusal(&report)
    );
    assert_eq!(rig.harness.dispatches(), 3, "one dispatch per pass");
    assert_eq!(
        rig.harness.input(0, "prev"),
        Json::Null,
        "the first pass has no previous iteration, so the self-reference is `null` (ADR-0055)"
    );
    assert_eq!(
        rig.harness.input(1, "prev")["n"],
        serde_json::json!(1),
        "the second pass binds the FIRST pass's output, never the current one"
    );
    assert_eq!(rig.harness.input(2, "prev")["n"], serde_json::json!(2));
    assert_eq!(
        frames(&rig, "attempt"),
        vec!["repeat:1", "repeat:2", "repeat:3"],
        "each pass is a `repeat` frame of its own"
    );
}

/// **ADR-0042: a feedback ref binds the later step's value from the PREVIOUS frame of the cycle that
/// re-enters the referencing step, and is `null` first** — over `tdd`'s `red` / `red-verify` /
/// `red-check` loop-back, and over `minimal-repro`'s escalate-gate answer (B1; M24 binds the current
/// frame's value, which is absent on every pass).
#[test]
fn a_feedback_ref_binds_the_previous_loop_back_frame_and_is_null_first() {
    // ---- (a) `tdd`'s red loop: `red` reads `${red-verify.outputs?}` across `red-check`'s loop-back.
    let steps = vec![
        agent("red", &[("last", "${red-verify.outputs?}")]),
        tool(
            "red-verify",
            "test",
            &[("selection", "${red.outputs.tests}")],
        ),
        check_on_false(
            "red-check",
            &[("report", "${red-verify.outputs}")],
            "report.failed > 0 and report.passed == 0",
            serde_json::json!({"to": "red", "max": 2}),
        ),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"tests": ["t1"]}),
            serde_json::json!({"tests": ["t1"]}),
        ],
        // The first run PASSES (so `red-check` does not hold and loops back); the second fails.
        vec![(1, 0), (0, 1)],
    );
    let report = rig.walk(&steps);
    assert!(
        report.completed() && report.step_error().is_none(),
        "the loop-back must re-enter `red`, whose second run is red: {}",
        crate::on_false_exec::refusal(&report)
    );
    assert_eq!(
        rig.harness.input(0, "last"),
        Json::Null,
        "the first pass has no previous frame: the feedback ref is `null` (ADR-0042)"
    );
    let last = rig.harness.input(1, "last");
    assert_eq!(
        (last["passed"].clone(), last["failed"].clone()),
        (serde_json::json!(1), serde_json::json!(0)),
        "the loop-back frame binds `red-verify`'s value from the PREVIOUS frame: {last}"
    );
    assert_eq!(frames(&rig, "red"), vec!["root", "loop_back:1"]);

    // ---- (b) `minimal-repro`'s escalate gate: `attempt` reads `${triage-gate.answer?}`.
    let steps = vec![
        repeating(
            agent("attempt", &[("hint", "${triage-gate.answer?}")]),
            "${attempt.outputs.status} == 'reproduced'",
            1,
            Some("escalate"),
            Some("repro.cannot-reproduce"),
        ),
        gate(
            "triage-gate",
            "repro.cannot-reproduce",
            Some("${attempt.outputs.status} != 'reproduced'"),
        ),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"status": "no"}),
            serde_json::json!({"status": "reproduced"}),
        ],
        Vec::new(),
    );
    let first = rig.walk(&steps);
    assert_eq!(
        rig.harness.input(0, "hint"),
        Json::Null,
        "before the gate is answered the answer is `null`"
    );
    rig.answer(&first, "look-at-the-logs");
    let second = rig.walk(&steps);
    assert!(
        second.completed(),
        "the answer re-enters `attempt`, which reproduces: {}",
        crate::on_false_exec::refusal(&second)
    );
    assert_eq!(
        rig.harness.input(1, "hint"),
        serde_json::json!("look-at-the-logs"),
        "the re-entered pass binds the gate's answer from the previous frame"
    );
}

/// **D21-48 X6: a `repeat.until` over a later step runs the body THROUGH that step each pass**, in
/// declaration order, one `repeat` frame per pass, `until` evaluated after the body's last step —
/// over `tdd`'s `green` and `green-verify` (M26 ends the body at the repeating step).
#[test]
fn a_repeat_until_over_a_later_step_runs_the_body_through_that_step_each_pass() {
    let steps = vec![
        repeating(
            agent("green", &[("last", "${green-verify.outputs?}")]),
            "${green-verify.outputs.status} == 'passing'",
            3,
            Some("escalate"),
            Some("tdd.green-exhausted"),
        ),
        tool(
            "green-verify",
            "test",
            &[("selection", "${green.outputs.tests}")],
        ),
        gate(
            "green-gate",
            "tdd.green-exhausted",
            Some("${green-verify.outputs.status} != 'passing'"),
        ),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"tests": ["g1"]}),
            serde_json::json!({"tests": ["g1"]}),
        ],
        vec![(0, 1), (1, 0)],
    );
    let report = rig.walk(&steps);
    assert!(
        report.completed() && report.gate_opened().is_none() && report.step_error().is_none(),
        "the body must run through `green-verify` each pass and stop when it passes: {}",
        crate::on_false_exec::refusal(&report)
    );
    assert_eq!(
        (rig.harness.dispatches(), rig.tests.selections().len()),
        (2, 2),
        "each pass runs the WHOLE body: `green` and then `green-verify`"
    );
    assert_eq!(frames(&rig, "green"), vec!["repeat:1", "repeat:2"]);
    assert_eq!(
        rig.harness.input(0, "last"),
        Json::Null,
        "the first pass reads no `green-verify` yet"
    );
    assert_eq!(
        rig.harness.input(1, "last")["status"],
        serde_json::json!("failing"),
        "the second pass binds the first pass's `green-verify`, the body's feedback"
    );
}

/// **`for_each` fans out, one frame per element, and `[*].field` projects** — over a list-valued
/// field, the concatenation in element order (D21-48 X5), while a bare fan-out is never flattened
/// into a selection (N13). AC-022: the fan-out charges its pass ONCE.
#[test]
fn for_each_fans_out_over_a_list_and_star_projects_its_outputs() {
    let fan_out = |selection: &str| {
        let mut red = agent("red", &[("ac", "${item}")]);
        red.for_each = Some("artifact:Spec.acs".to_owned());
        vec![
            red,
            tool("red-verify", "test", &[("selection", selection)]),
            step("emit", StepKind::Emit),
        ]
    };
    let rig = Rig::new(
        vec![
            serde_json::json!({"tests_added": ["a", "b"]}),
            serde_json::json!({"tests_added": ["c"]}),
        ],
        vec![(3, 0)],
    );
    rig.seed(emitted_row(
        "spec-author@1",
        "Spec",
        serde_json::json!({"acs": ["AC-1", "AC-2"]}),
    ));
    let report = rig.walk(&fan_out("${red.outputs[*].tests_added}"));
    assert!(
        report.completed() && report.step_error().is_none(),
        "the fan-out and its projection must walk: {}",
        crate::on_false_exec::refusal(&report)
    );
    assert_eq!(
        (rig.harness.input(0, "ac"), rig.harness.input(1, "ac")),
        (serde_json::json!("AC-1"), serde_json::json!("AC-2")),
        "one copy per element, with the element bound, in element order"
    );
    assert_eq!(
        frames(&rig, "red"),
        vec!["for_each:0", "for_each:1"],
        "each element commits under its own `for_each` frame"
    );
    assert_eq!(
        rig.tests.selections(),
        vec![rk_ports::toolchain::Selection::Filter(vec![
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned()
        ])],
        "`[*].tests_added` over lists of DIFFERENT lengths is their concatenation in element order"
    );
    let charges = report.pass_charges();
    assert_eq!(
        charges.len(),
        1,
        "a fan-out is not a pass of its own: {charges:?}"
    );
    assert_eq!(
        charges[0].1,
        rk_kernel::workflow::budget::Charge::Attempts(rk_kernel::workflow::budget::Attempts::new(
            2
        )),
        "`red` counts ONCE whatever its element count (AC-022), and `red-verify` once: {charges:?}"
    );

    // A bare `${red.outputs}` of a fan-out is a list of reports, never flattened into a selection.
    let rig = Rig::new(
        vec![
            serde_json::json!({"tests_added": ["a"]}),
            serde_json::json!({"tests_added": ["c"]}),
        ],
        vec![(1, 0)],
    );
    rig.seed(emitted_row(
        "spec-author@1",
        "Spec",
        serde_json::json!({"acs": ["AC-1", "AC-2"]}),
    ));
    let report = rig.walk(&fan_out("${red.outputs}"));
    let e = crate::on_false_exec::refusal(&report);
    assert!(
        e.contains("`red-verify`")
            && e.contains("${red.outputs}")
            && e.contains("not all test names"),
        "a bare fan-out is refused as a selection by name, never flattened (N13): {e}"
    );
    assert!(
        rig.tests.selections().is_empty(),
        "and the port is never reached"
    );
}

/// **`repeat = { until, max }` re-runs until `until` holds, and at `max` takes `on_exhausted`**:
/// `escalate` proceeds to the explicit gate step asking its question (schema item 15) and refuses
/// by name when there is none; `fail` refuses; `continue` proceeds.
#[test]
fn repeat_until_reruns_until_the_predicate_holds_and_takes_on_exhausted_at_max() {
    let attempt = |max: u32, exit: &str| {
        repeating(
            agent("attempt", &[]),
            "${attempt.outputs.status} == 'done'",
            max,
            Some(exit),
            Some("loop.exhausted"),
        )
    };
    let no = || serde_json::json!({"status": "no"});
    // HOLDS on the third pass.
    let rig = Rig::new(
        vec![no(), no(), serde_json::json!({"status": "done"})],
        Vec::new(),
    );
    let report = rig.walk(&[attempt(3, "fail"), step("emit", StepKind::Emit)]);
    assert!(
        report.completed() && rig.harness.dispatches() == 3,
        "the step re-runs until `until` holds: {}",
        crate::on_false_exec::refusal(&report)
    );
    // `escalate`, WITH the gate step: exhaustion proceeds to it and opens it.
    let rig = Rig::new(vec![no(), no()], Vec::new());
    let report = rig.walk(&[
        attempt(2, "escalate"),
        gate(
            "stuck",
            "loop.exhausted",
            Some("${attempt.outputs.status} != 'done'"),
        ),
        step("emit", StepKind::Emit),
    ]);
    assert!(
        report.gate_opened().is_some() && report.step_error().is_none(),
        "at `max`, `escalate` proceeds to the gate asking its question: {}",
        crate::on_false_exec::refusal(&report)
    );
    assert_eq!(
        rig.harness.dispatches(),
        2,
        "`max` passes, and not one more (the harness's script is the bound)"
    );
    // `escalate` with NO gate step asking the question: refused by name.
    let rig = Rig::new(vec![no(), no()], Vec::new());
    let report = rig.walk(&[
        attempt(2, "escalate"),
        gate("other", "another.question", None),
        step("emit", StepKind::Emit),
    ]);
    let e = crate::on_false_exec::refusal(&report);
    assert!(
        e.contains("`attempt`") && e.contains("loop.exhausted") && e.contains("no `gate` step"),
        "an `escalate` with no gate step asking its question refuses by name: {e}"
    );
    assert!(report.gate_opened().is_none(), "and opens no gate: {e}");
    // `fail`: refused, and the commits stand.
    let rig = Rig::new(vec![no(), no()], Vec::new());
    let report = rig.walk(&[attempt(2, "fail"), step("emit", StepKind::Emit)]);
    let e = crate::on_false_exec::refusal(&report);
    assert!(
        e.contains("`attempt`") && e.contains("exhausted") && report.committed(),
        "`fail` refuses by name at `max`, the commits standing: {e}"
    );
    // `continue`: proceeds.
    let rig = Rig::new(vec![no(), no()], Vec::new());
    let report = rig.walk(&[attempt(2, "continue"), step("emit", StepKind::Emit)]);
    assert!(
        report.completed() && rig.harness.dispatches() == 2,
        "`continue` proceeds past an exhausted loop: {}",
        crate::on_false_exec::refusal(&report)
    );
}

/// **D21-48 X8: an escalate re-entry is a new frame of the referencing step**, so a later TOOL
/// step's output from the previous frame binds across it as the gate's answer does, and is `null`
/// on the first pass — over `minimal-repro-ue`'s `attempt` reading `${cook.outputs?}` and
/// `${run.outputs?}` (M35 binds `null` for them on the resumed pass). `cook` and `run` route to
/// `toolchain:build` here, since `CookReport` has no entry in the closed decode table.
#[test]
fn an_escalate_re_entry_binds_a_later_steps_previous_frame_value_and_is_null_first() {
    let steps = vec![
        repeating(
            agent(
                "attempt",
                &[
                    ("hint", "${triage-gate.answer?}"),
                    ("cook", "${cook.outputs?}"),
                    ("automation", "${run.outputs?}"),
                ],
            ),
            "${attempt.outputs.status} == 'reproduced'",
            1,
            Some("escalate"),
            Some("repro.cannot-reproduce"),
        ),
        tool(
            "cook",
            "build",
            &[("command", "${attempt.outputs.command}")],
        ),
        tool("run", "build", &[("cook", "${cook.outputs}")]),
        gate(
            "triage-gate",
            "repro.cannot-reproduce",
            Some("${attempt.outputs.status} != 'reproduced'"),
        ),
        step("emit", StepKind::Emit),
    ];
    let rig = Rig::new(
        vec![
            serde_json::json!({"status": "no", "command": "c1"}),
            serde_json::json!({"status": "reproduced", "command": "c2"}),
        ],
        Vec::new(),
    );
    let first = rig.walk(&steps);
    assert!(
        first.gate_opened().is_some(),
        "PREMISE: the exhausted `attempt` escalates to `triage-gate`: {}",
        crate::on_false_exec::refusal(&first)
    );
    for name in ["hint", "cook", "automation"] {
        assert_eq!(
            rig.harness.input(0, name),
            Json::Null,
            "the first pass has no previous frame: `{name}` is `null`"
        );
    }
    rig.answer(&first, "try-the-editor");
    let second = rig.walk(&steps);
    assert_eq!(
        rig.harness.dispatches(),
        2,
        "the answer resumes INTO `attempt` (schema §3): the escalate re-entry opens a new frame and \
         dispatches it: {}",
        crate::on_false_exec::refusal(&second)
    );
    assert_eq!(
        rig.harness.input(1, "cook")["status"],
        serde_json::json!("passing"),
        "the resumed pass binds `cook`'s value from the PREVIOUS frame, a later tool step's"
    );
    assert!(
        !rig.harness.input(1, "automation").is_null(),
        "and `run`'s, the same way"
    );
    assert_eq!(
        rig.harness.input(1, "hint"),
        serde_json::json!("try-the-editor")
    );
    assert!(
        second.completed() && second.step_error().is_none(),
        "the re-entered pass reproduces and the walk completes: {}",
        crate::on_false_exec::refusal(&second)
    );
    assert_eq!(
        frames(&rig, "attempt"),
        vec!["repeat:1", "loop_back:1/repeat:1"],
        "the re-entry is a new frame of `attempt`; its fresh repeat runs inside it"
    );
}

/// **A bare `${<step>}` evidence pointer resolves once the step has run IN THIS ACTIVATION, not
/// only in the current frame** *(dated 2026-09-27, `s8d` build, package R6FIX-8d; refuting round
/// 6's spec-conformance major)*. §8d.3's dated R2FIX-8d row and ADR-0173 decision 1: the pointer
/// *"resolves only once the step has run in this activation"*. Over `minimal-repro`'s shape: the
/// exhausted `attempt` escalates to `triage-gate`, the gate is answered, the re-entry opens a new
/// frame from `attempt` to the end, `attempt` reproduces and `triage-gate` is skipped by its own
/// `when`. The gate WAS answered in this activation, so the completion row's `evidence` carries its
/// `{step, kind}` pointer (ADR-0036: *"a `gate` step mints `HumanAnswer`"*). CONTROLS: a gate never
/// reached is `null` (omitted) to `${triage-gate?}` and refuses by name to a mandatory
/// `${triage-gate}`.
#[test]
fn an_answered_escalate_gate_is_evidence_on_the_completion_row_after_its_re_entry() {
    let steps = |evidence: &[&str]| {
        let mut emit = step("emit", StepKind::Emit);
        emit.emit = Some(rk_kernel::catalog::EmitBinding {
            artifacts: BTreeMap::new(),
            evidence: evidence.iter().map(|e| (*e).to_owned()).collect(),
        });
        vec![
            repeating(
                agent("attempt", &[("hint", "${triage-gate.answer?}")]),
                "${attempt.outputs.status} == 'reproduced'",
                1,
                Some("escalate"),
                Some("repro.cannot-reproduce"),
            ),
            gate(
                "triage-gate",
                "repro.cannot-reproduce",
                Some("${attempt.outputs.status} != 'reproduced'"),
            ),
            emit,
        ]
    };
    let evidence_of = |rig: &Rig| -> Vec<Json> {
        let rows = journal(&rig.db);
        let completion: Vec<_> = rows
            .iter()
            .filter(|e| rk_kernel::runtime::activity::is_completion_row(e))
            .collect();
        assert_eq!(completion.len(), 1, "PREMISE: one completion row: {rows:?}");
        completion[0].payload["evidence"]
            .as_array()
            .cloned()
            .unwrap_or_default()
    };
    let attempt_ptr = serde_json::json!({ "step": "attempt", "kind": "agent" });
    let gate_ptr = serde_json::json!({ "step": "triage-gate", "kind": "gate" });

    // THE ESCALATE RE-ENTRY, answered, then a pass that succeeds.
    for (evidence, label) in [
        (&["${attempt}", "${triage-gate?}"][..], "optional"),
        (&["${attempt}", "${triage-gate}"][..], "mandatory"),
    ] {
        let rig = Rig::new(
            vec![
                serde_json::json!({"status": "no"}),
                serde_json::json!({"status": "reproduced"}),
            ],
            Vec::new(),
        );
        let first = rig.walk(&steps(evidence));
        assert!(
            first.gate_opened().is_some(),
            "PREMISE ({label}): the exhausted `attempt` escalates to `triage-gate`: {}",
            crate::on_false_exec::refusal(&first)
        );
        rig.answer(&first, "look-at-the-logs");
        let second = rig.walk(&steps(evidence));
        assert!(
            second.completed() && second.step_error().is_none(),
            "the answered re-entry reproduces, skips the gate by its `when` and completes \
             ({label}): {}",
            crate::on_false_exec::refusal(&second)
        );
        assert_eq!(
            (rig.harness.dispatches(), rig.harness.input(1, "hint")),
            (2, serde_json::json!("look-at-the-logs")),
            "PREMISE ({label}): the resumed pass was dispatched with the gate's answer"
        );
        assert_eq!(
            evidence_of(&rig),
            vec![attempt_ptr.clone(), gate_ptr.clone()],
            "the gate answered in THIS activation is evidence on the completion row, although the \
             re-entry's frame skipped it ({label})"
        );
    }

    // CONTROL: `attempt` reproduces on its first pass, so `triage-gate` is never reached.
    let rig = Rig::new(
        vec![serde_json::json!({"status": "reproduced"})],
        Vec::new(),
    );
    let report = rig.walk(&steps(&["${attempt}", "${triage-gate?}"]));
    assert!(
        report.completed() && report.gate_opened().is_none(),
        "PREMISE: the control completes without reaching the gate: {}",
        crate::on_false_exec::refusal(&report)
    );
    assert_eq!(
        evidence_of(&rig),
        vec![attempt_ptr],
        "a gate never reached is `null` to `${{triage-gate?}}`, and omitted"
    );
    let rig = Rig::new(
        vec![serde_json::json!({"status": "reproduced"})],
        Vec::new(),
    );
    let report = rig.walk(&steps(&["${attempt}", "${triage-gate}"]));
    let e = crate::on_false_exec::refusal(&report);
    assert!(
        !report.completed() && e.contains("triage-gate") && e.contains("has not run"),
        "a mandatory `${{triage-gate}}` to a gate never reached refuses by name: {e}"
    );
}
