//! **Who assembles the review input, and where the `Binary` route lives** — `s8i` §8i.8, ADR-0199
//! (proposed): the `Binary` review route is the human gate (D21-43 N21), opened through D21-48
//! X16's type-neutral gate request.
//!
//! [`rk_cli::review_input`] reads the item's typed `ChangeSummary` through the `Toolchain` port for
//! the item's range. For a `Text` summary it assembles the reviewer's input, diff included; for a
//! `Binary` one it assembles no diff and refuses an automated verdict, and that refusal opens a
//! gate through `rk_kernel::workflow::gate::GateRequest` with the closed reason
//! `automated-verdict-refused` and, as the opaque attachment, `review_input`'s serialisation of the
//! typed summary. Its hook is `review_input::route`, which `redkiln run`'s `resume_and_read` wraps
//! the stage's toolchain in; the gate then opens on the run path.
//!
//! | test | what it measures | its mutants |
//! | --- | --- | --- |
//! | [`ac497_binary_classification_assembles_no_diff_and_carries_the_typed_summary`] | a `Toolchain` fake driven by a binary fixture: the summary is read once, for the item's range; the diff assembler is never called for the `Binary` classification (and is, for a `Text` control); the input carries the typed summary, and its serialisation carries no diff | call the diff assembler for a `Binary` path |
//! | [`binary_item_refuses_an_automated_verdict_and_opens_a_human_gate`] | the `Binary` item reaches no reviewer: it refuses an automated verdict, and the refusal opens a gate whose kind is `automated-verdict-refused`, whose token is `derive_reach`'s, and whose attachment is byte for byte the typed summary's serialisation, which the render side renders; the decorator refuses the value too | the reviewer route's behaviour; open the gate through a gate step's free `kind` string with no attachment |
//! | [`binary_item_on_the_run_path_opens_one_human_gate_and_dispatches_no_reviewer`] | a `Binary` fake toolchain behind `verbs::run::run_with_advance`: the stage's `toolchain:change_summary` step refuses, exactly one gate opens with the closed reason and `review_input`'s serialisation as its attachment, no dispatch is made, and `run` exits `0` | the screen-refusal path (refuse the value and keep nothing) |
//! | [`a_refusal_after_a_committed_agent_step_opens_one_human_gate`] | the walk's other shape: an agent step commits, then the `change_summary` step refuses, so the kernel returns `StepFailedAfterCommit` carrying the refusal; exactly one gate opens and `run` exits `0` | accept only `NotCommitted` as the refusal's own outcome |
//! | [`the_bound_review_gate_is_a_tier_one_act_through_the_stages_sibling_boundary`] | the bound path: a live tier-1 boundary over a real run tree and store; the gate's `opened` row is a tier-1 act, its `Gate:` mark the run ref's tip, the row anchored to it | open the gate unbound (append the row, no act) |
//!
//! The kernel's outcome while a refusal is kept is read by `verbs::run`'s private `kept_refusal`
//! module, whose unit tests take each branch: the refusal's own outcome opens the gate; a success
//! or any other failure is reported and mapped as it stands. Those branches are unreachable from
//! here: after a refused tool step `replay::dispatch_through` returns `NotCommitted` (or
//! `StepFailedAfterCommit`) straight from the walk's report, with no retry and no later store write.
//!
//! The reviewer-route test (`binary_item_reaches_the_reviewer_with_the_typed_summary`) is not
//! written: the record's route is the human gate.

use std::sync::{Arc, Mutex};

use rk_cli::review_input::{self, DiffAssembler, ReviewInput};
use rk_kernel::workflow::fsm::{FsmContext, GateRecord, fold};
use rk_kernel::workflow::gate::{GateAttachment, GateReason};
use rk_ports::toolchain::{
    BinaryAsset, BinaryCapture, BinaryChangeSummary, ChangeKind, ChangeSummary, Classification,
    ManifestEntry, Selection, StepBound, TestOutcome, TextChangeSummary, Toolchain, VerbCall,
    VerbOutput,
};
use rk_types::error::StepError;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::verb_output::BuildReport;

const RANGE: &str =
    "4b825dc642cb6eb9a060e54bf8d69288fbee4904..9daeafb9864cf43055ae93beb0afd6c7d144bfa4";
const ASSET: &str = "Content/Maps/Arena.umap";
const OID: &str = "3f4a9c1b8e2d7f6a5c4b3a291807f6e5d4c3b2a1908f7e6d5c4b3a2918070605";
const ITEM: &str = "INC-1";
const STAGE: &str = "review";
const TREE: &str = "9daeafb9864cf43055ae93beb0afd6c7d144bfa4";
const DEADLINE: i64 = 1_700_003_600;
/// The reason's one spelling, as the record writes it.
const REASON: &str = "automated-verdict-refused";

use fixture::{
    FixtureSummary, RecordingAssembler, binary_summary, bound, serialised, text_summary,
};

/// The fixtures and fakes, in a `#[cfg(test)]` module so the test relaxations reach them (TST-14).
#[cfg(test)]
mod fixture {
    use super::*;

    pub(super) fn bound() -> StepBound {
        StepBound::secs(300).expect("a non-zero bound")
    }

    /// The binary fixture: one LFS asset, one capture, the two refs.
    pub(super) fn binary_summary() -> BinaryChangeSummary {
        let pointer =
            format!("version https://git-lfs.github.com/spec/v1\noid sha256:{OID}\nsize 48213\n");
        let asset = BinaryAsset::from_pointer(
            ASSET.to_owned(),
            "umap".to_owned(),
            ChangeKind::Modified,
            pointer.as_bytes(),
        )
        .expect("the fixture pointer is a git-lfs pointer");
        let capture = BinaryCapture {
            path: "captures/arena-top.png".to_owned(),
            viewport: "top".to_owned(),
            note: "/Game/Maps/Arena".to_owned(),
        };
        BinaryChangeSummary::new(
            vec![asset],
            vec![capture],
            "reports/tests.xml".to_owned(),
            "logs/build.log".to_owned(),
            &format!("Binary files a/{ASSET} and b/{ASSET} differ\n"),
        )
        .expect("a content-free binary record is admitted")
    }

    /// The `Text` control: one source path and its diff.
    pub(super) fn text_summary() -> TextChangeSummary {
        TextChangeSummary {
            sha: TREE.to_owned(),
            diff: concat!(
                "diff --git a/src/lib.rs b/src/lib.rs\n",
                "--- a/src/lib.rs\n",
                "+++ b/src/lib.rs\n",
                "@@ -1 +1 @@\n",
                "-a\n",
                "+b\n",
            )
            .to_owned(),
            manifest: vec![ManifestEntry {
                path: "src/lib.rs".to_owned(),
                classification: Classification::Text,
                change: ChangeKind::Modified,
            }],
        }
    }

    /// A `Toolchain` fake driven by one fixture summary; it records every range it is asked for.
    pub(super) struct FixtureSummary {
        summary: ChangeSummary,
        ranges: Mutex<Vec<String>>,
    }

    impl FixtureSummary {
        pub(super) const fn new(summary: ChangeSummary) -> Self {
            Self {
                summary,
                ranges: Mutex::new(Vec::new()),
            }
        }

        pub(super) fn ranges(&self) -> Vec<String> {
            self.ranges
                .lock()
                .expect("the range log is not poisoned")
                .clone()
        }
    }

    fn not_this_fake(verb: &str) -> StepError {
        StepError::Unsupported {
            capability: format!(
                "{verb}: the review-input fake answers change_summary and classify only"
            ),
        }
    }

    impl Toolchain for FixtureSummary {
        fn detect(&self) -> Result<bool, StepError> {
            Ok(true)
        }
        fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
            Err(not_this_fake("build"))
        }
        fn test(
            &self,
            _selection: &Selection,
            _bound: StepBound,
        ) -> Result<TestOutcome, StepError> {
            Err(not_this_fake("test"))
        }
        fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
            Err(not_this_fake("lint"))
        }
        fn classify(&self, path: &str) -> Result<Classification, StepError> {
            Ok(if path == ASSET {
                Classification::Binary
            } else {
                Classification::Text
            })
        }
        fn change_summary(
            &self,
            range: &str,
            _bound: StepBound,
        ) -> Result<ChangeSummary, StepError> {
            self.ranges
                .lock()
                .map_err(|e| not_this_fake(&format!("change_summary: the range log: {e}")))?
                .push(range.to_owned());
            Ok(self.summary.clone())
        }
        fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
            Err(not_this_fake(call.verb))
        }
    }

    /// A diff assembler that records every summary it is handed and assembles a marker.
    #[derive(Default)]
    pub(super) struct RecordingAssembler {
        calls: Mutex<Vec<ChangeSummary>>,
    }

    impl RecordingAssembler {
        pub(super) fn calls(&self) -> Vec<ChangeSummary> {
            self.calls
                .lock()
                .expect("the call log is not poisoned")
                .clone()
        }
    }

    impl DiffAssembler for RecordingAssembler {
        fn assemble(&self, summary: &ChangeSummary) -> Result<String, StepError> {
            self.calls
                .lock()
                .map_err(|e| not_this_fake(&format!("assemble: the call log: {e}")))?
                .push(summary.clone());
            Ok("ASSEMBLED-DIFF".to_owned())
        }
    }

    /// `review_input`'s serialisation of the typed summary, as the test computes it independently.
    pub(super) fn serialised(summary: &BinaryChangeSummary) -> Vec<u8> {
        serde_json::to_vec(&ChangeSummary::Binary(summary.clone())).expect("the summary serialises")
    }
}

#[test]
fn ac497_binary_classification_assembles_no_diff_and_carries_the_typed_summary() {
    let expected = binary_summary();
    let fake = FixtureSummary::new(ChangeSummary::Binary(expected.clone()));
    assert_eq!(
        fake.classify(ASSET).expect("the fake classifies"),
        Classification::Binary,
        "PREMISE: the fixture's asset path classifies Binary"
    );
    let assembler = RecordingAssembler::default();

    let input = review_input::assemble(&fake, RANGE, bound(), &assembler)
        .expect("the review input assembles");

    assert_eq!(
        fake.ranges(),
        vec![RANGE.to_owned()],
        "the typed summary is read through the Toolchain port once, for the item's range"
    );
    assert!(
        assembler.calls().is_empty(),
        "the diff assembler was called for a Binary classification: {:?}",
        assembler.calls()
    );
    let ReviewInput::HumanGate(refusal) = &input else {
        panic!("a Binary summary assembled a reviewer input: {input:?}");
    };
    assert_eq!(
        refusal.summary(),
        &ChangeSummary::Binary(expected.clone()),
        "the input carries the typed summary"
    );
    let bytes = refusal.request().attachment().bytes();
    assert_eq!(
        bytes,
        serialised(&expected).as_slice(),
        "the carried bytes are the typed summary's serialisation"
    );
    let value: serde_json::Value =
        serde_json::from_slice(bytes).expect("the serialisation is JSON");
    let mut members: Vec<&str> = value
        .as_object()
        .expect("the serialisation is an object")
        .keys()
        .map(String::as_str)
        .collect();
    members.sort_unstable();
    assert_eq!(
        members,
        vec!["assets", "build", "captures", "tests"],
        "the Binary arm's four members and no `diff`"
    );
    let text = String::from_utf8(bytes.to_vec()).expect("the serialisation is UTF-8");
    assert!(
        !text.contains("diff --git") && !text.contains("@@") && !text.contains("Binary files"),
        "no textual diff of the binary path rides in the input: {text}"
    );

    // CONTROL: the assembler is reachable, so its silence above is not vacuous. A Text summary
    // reaches it once and the reviewer receives the typed summary with the assembled diff.
    let text_fake = FixtureSummary::new(ChangeSummary::Text(text_summary()));
    let control = RecordingAssembler::default();
    let text_input = review_input::assemble(&text_fake, RANGE, bound(), &control)
        .expect("CONTROL: the text input assembles");
    assert_eq!(
        control.calls(),
        vec![ChangeSummary::Text(text_summary())],
        "CONTROL: a Text summary reaches the diff assembler once"
    );
    let ReviewInput::Reviewer(reviewer) = &text_input else {
        panic!("CONTROL: a Text summary did not reach the reviewer: {text_input:?}");
    };
    assert_eq!(reviewer.summary(), &ChangeSummary::Text(text_summary()));
    assert_eq!(reviewer.diff(), "ASSEMBLED-DIFF");
}

#[test]
fn binary_item_refuses_an_automated_verdict_and_opens_a_human_gate() {
    let expected = binary_summary();
    let fake = FixtureSummary::new(ChangeSummary::Binary(expected.clone()));
    let input = review_input::assemble(&fake, RANGE, bound(), &review_input::SummaryDiff)
        .expect("the review input assembles");

    // No reviewer: an automated verdict is refused, with the closed reason.
    let ReviewInput::HumanGate(refusal) = &input else {
        panic!("a Binary item reached the reviewer: {input:?}");
    };
    assert_eq!(
        refusal.request().reason(),
        GateReason::AutomatedVerdictRefused,
        "the refusal's reason is the closed one"
    );
    assert_eq!(refusal.request().reason().as_str(), REASON);

    // The hand-off: the refusal opens a gate through X16's request.
    let empty = fold(ITEM, STAGE, &[]).expect("an empty journal folds");
    let ctx = FsmContext::supplied(ITEM, TREE, Timestamp::from_unix_seconds(DEADLINE), None);
    let event = refusal
        .open_gate(&empty, &ctx)
        .expect("the refusal opens its gate");
    assert_eq!(event.kind, EventKind::GateOpened);
    let derived = GateRecord::derive_reach(REASON, TREE, Timestamp::from_unix_seconds(DEADLINE), 1)
        .expect("derive_reach mints");
    assert_eq!(
        event.payload["kind"], REASON,
        "the gate's kind is the closed reason, not a gate step's free kind"
    );
    assert_eq!(
        event.payload["token"],
        derived.token().as_str(),
        "the token is derive_reach's, through gate::reach"
    );
    let attachment =
        GateAttachment::from_gate_opened(&event).expect("the opened gate carries the attachment");
    assert_eq!(
        attachment.bytes(),
        serialised(&expected).as_slice(),
        "the attachment's bytes are review_input's serialisation of the typed summary"
    );
    let state = fold(ITEM, STAGE, std::slice::from_ref(&event)).expect("the row folds");
    assert_eq!(state.gate_record(derived.token()), Some(&derived));
    assert_eq!(state.open_gates().count(), 1, "the human gate is open");

    // The render side reads the attachment as the Binary arm (rk-store's AC-154 renderer).
    let carried: serde_json::Value =
        serde_json::from_slice(attachment.bytes()).expect("the attachment is JSON");
    let rendered = rk_store::render::render_binary_summary(&carried)
        .expect("the render side renders the carried summary");
    let rendered = String::from_utf8(rendered).expect("the rendering is UTF-8");
    assert!(
        rendered.contains(ASSET) && rendered.contains(OID),
        "the rendering names the asset and its oid: {rendered}"
    );

    // The hook: the run's Toolchain decorator, `review_input::route`, hands no reviewer a Binary
    // value either, and keeps the refusal for the gate.
    let (decorated, refusals) = review_input::route(Arc::new(FixtureSummary::new(
        ChangeSummary::Binary(expected.clone()),
    )));
    let refused = decorated
        .change_summary(RANGE, bound())
        .expect_err("the decorator refuses a Binary summary");
    assert!(
        format!("{refused:?}").contains(REASON),
        "the decorator's refusal names the reason: {refused:?}"
    );
    assert_eq!(
        refusals
            .first()
            .map(review_input::AutomatedVerdictRefused::summary),
        Some(&ChangeSummary::Binary(expected)),
        "the hook keeps the refusal, with the typed summary, for the gate"
    );
    // CONTROL: a Text summary passes the decorator unchanged, and nothing is refused.
    let (text_route, text_refusals) = review_input::route(Arc::new(FixtureSummary::new(
        ChangeSummary::Text(text_summary()),
    )));
    let passed = text_route
        .change_summary(RANGE, bound())
        .expect("CONTROL: a Text summary passes the decorator");
    assert_eq!(passed, ChangeSummary::Text(text_summary()));
    assert_eq!(
        text_refusals.first(),
        None,
        "CONTROL: a Text summary refuses nothing"
    );
}

/// The fixture activity for the run-path test: `pack-min`'s `emit-report`, with a
/// `toolchain:change_summary` tool step over the item's range BEFORE its agent step, so the agent
/// step is the dispatch a `Binary` item must never reach.
#[cfg(test)]
fn summarising_activity() -> String {
    concat!(
        "[activity]\n",
        "id           = \"emit-report\"\n",
        "version      = 1\n",
        "stage        = \"implement\"\n",
        "provides     = [\"ImplReport\"]\n",
        "requires     = []\n",
        "derived_from = []\n",
        "deviation_ok = []\n",
        "tags         = [\"kind:bug\", \"kind:story\"]\n",
        "method_rank  = 10\n",
        "min_rigor    = \"optional\"\n",
        "cost_class   = \"S\"\n",
        "budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }\n",
        "\n",
        "[[step]]\n",
        "id      = \"summarise\"\n",
        "kind    = \"tool\"\n",
        "run     = \"toolchain:change_summary\"\n",
        "inputs  = { range = \"RANGE\" }\n",
        "outputs = \"value:ChangeSummary\"\n",
        "timeout_secs = 120\n",
        "\n",
        "[[step]]\n",
        "id      = \"check\"\n",
        "kind    = \"agent\"\n",
        "role    = \"implementer\"\n",
        "outputs = \"value:ImplReport\"\n",
        "gate_deadline_secs = 900\n",
    )
    .replace("RANGE", RANGE)
}

/// **The human-gate route on `redkiln run`'s production path** (`s8i` §8i.8; ADR-0199 decision 3,
/// proposed; operator ruling R7 of 2026-10-14). A `Binary` fake toolchain behind `run_with_advance` (the function `run`'s
/// lane loop calls, over `resume_and_read`): the stage's `toolchain:change_summary` step reads the
/// `Binary` value, `review_input::assemble` refuses an automated verdict, and exactly one gate opens
/// with the closed reason, carrying `review_input`'s serialisation of the typed summary as its
/// attachment. The stage's agent step is never dispatched.
#[test]
fn binary_item_on_the_run_path_opens_one_human_gate_and_dispatches_no_reviewer() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let rk = root.join(".redkiln");
    std::fs::write(
        rk.join("catalog").join("emit-report.toml"),
        summarising_activity(),
    )
    .expect("the summarising activity installs");
    // The shipped registry's `change_summary` row, verbatim: a `port` route taking `range`.
    std::fs::write(
        rk.join("registry.toml"),
        concat!(
            "[verb]\n",
            "change_summary = { outputs = \"ChangeSummary\", live = false, timeout = ",
            "\"build_timeout\", inputs = [\"range\"], route = \"port\", side_effecting = false }\n",
        ),
    )
    .expect("the verb registry installs");

    // `story-1`: `has_repro = false`, so no `TestReport` requirement gates the walk before the
    // activity is reached.
    let resolved = crate::support::resolved_over_pack_min(root, "story-1");
    let (inputs, tree_hash) = crate::support::inputs_over_pack_min(&resolved);
    // The activation's steps, transcribed from the installed activity as the composition root's
    // binder transcribes them (ADR-0138).
    let source = summarising_activity();
    let catalog = rk_kernel::catalog::Catalog::from_sources(&[("emit-report.toml", &source)])
        .expect("the summarising activity loads");
    let steps = catalog
        .activities()
        .iter()
        .find(|a| a.id == "emit-report")
        .map(|a| a.steps.clone())
        .expect("the catalog holds emit-report");
    assert_eq!(
        steps.len(),
        2,
        "PREMISE: the tool step, then the agent step"
    );
    let inputs = inputs.with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::with_steps(
            "emit-report@1",
            crate::support::any_activity_spec(),
            steps,
        ),
    );
    let routing = rk_cli::verbs::run::tool_routing(root, None).expect("the registry loads");
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let harness = Arc::new(crate::support::ScriptedHarness::scripted(vec![Ok(
        crate::support::impl_report(),
    )]));
    let expected = binary_summary();
    // A second handle on the counting harness: the run takes one, the assertion reads the other.
    let dispatcher: Arc<dyn rk_ports::harness::Harness> =
        Arc::<crate::support::ScriptedHarness>::clone(&harness);
    let mut ports = crate::support::ports(dispatcher, &store);
    ports.toolchain = Arc::new(FixtureSummary::new(ChangeSummary::Binary(expected.clone())));
    let mut supply = crate::support::any_spec("emit-report@1", crate::support::any_activity_spec());
    let token = rk_ports::cancel::CancelToken::new();

    let exit = rk_cli::verbs::run::run_with_advance(
        &resolved,
        &inputs,
        crate::support::FIXED_NOW,
        &tree_hash,
        ports,
        routing,
        &crate::support::clean_env(),
        &mut supply,
        &token,
        &mut None,
        &mut false,
        None,
        None,
    )
    .expect("the run returns an exit, not a halt");

    assert_eq!(
        harness.dispatches(),
        0,
        "a Binary item's stage dispatched {} time(s): no automated reviewer may run",
        harness.dispatches()
    );
    let rows = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("story-1"))
        .expect("the fixture store queries");
    let opened: Vec<&rk_types::event::Event> = rows
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .collect();
    assert_eq!(
        opened.len(),
        1,
        "exactly one gate opens on the run path: {:?}",
        rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
    );
    let [gate] = opened.as_slice() else {
        panic!("one gate_opened row");
    };
    assert_eq!(
        gate.payload["kind"], REASON,
        "the gate's kind is the closed reason: {rows:?}"
    );
    let ReviewInput::HumanGate(refusal) = review_input::assemble(
        &FixtureSummary::new(ChangeSummary::Binary(expected)),
        RANGE,
        bound(),
        &review_input::SummaryDiff,
    )
    .expect("the review input assembles") else {
        panic!("a Binary summary assembled a reviewer input");
    };
    assert_eq!(
        GateAttachment::from_gate_opened(gate)
            .expect("the gate carries the attachment")
            .bytes(),
        refusal.request().attachment().bytes(),
        "the attachment is review_input's serialisation of the typed summary"
    );
    assert_eq!(
        exit,
        rk_cli::exit::Exit::Success,
        "`run` exits 0 at a gate (golden constraint 2)"
    );
}

/// The walk's other shape: `emit-report`'s agent step FIRST, then the `toolchain:change_summary`
/// step, so the activation commits and a later step of its walk refuses.
#[cfg(test)]
fn summary_after_agent_activity() -> String {
    summarising_activity().replace(
        concat!(
            "[[step]]\n",
            "id      = \"summarise\"\n",
            "kind    = \"tool\"\n",
            "run     = \"toolchain:change_summary\"\n",
            "inputs  = { range = \"RANGE\" }\n",
            "outputs = \"value:ChangeSummary\"\n",
            "timeout_secs = 120\n",
            "\n",
            "[[step]]\n",
            "id      = \"check\"\n",
            "kind    = \"agent\"\n",
            "role    = \"implementer\"\n",
            "outputs = \"value:ImplReport\"\n",
            "gate_deadline_secs = 900\n",
        )
        .replace("RANGE", RANGE)
        .as_str(),
        concat!(
            "[[step]]\n",
            "id      = \"check\"\n",
            "kind    = \"agent\"\n",
            "role    = \"implementer\"\n",
            "outputs = \"value:ImplReport\"\n",
            "gate_deadline_secs = 900\n",
            "\n",
            "[[step]]\n",
            "id      = \"summarise\"\n",
            "kind    = \"tool\"\n",
            "run     = \"toolchain:change_summary\"\n",
            "inputs  = { range = \"RANGE\" }\n",
            "outputs = \"value:ChangeSummary\"\n",
            "timeout_secs = 120\n",
        )
        .replace("RANGE", RANGE)
        .as_str(),
    )
}

/// Install `source` as `emit-report` and the shipped registry's `change_summary` row, verbatim.
#[cfg(test)]
fn install_summarising(root: &std::path::Path, source: &str) {
    let rk = root.join(".redkiln");
    std::fs::write(rk.join("catalog").join("emit-report.toml"), source)
        .expect("the activity installs");
    std::fs::write(
        rk.join("registry.toml"),
        concat!(
            "[verb]\n",
            "change_summary = { outputs = \"ChangeSummary\", live = false, timeout = ",
            "\"build_timeout\", inputs = [\"range\"], route = \"port\", side_effecting = false }\n",
        ),
    )
    .expect("the verb registry installs");
}

/// `inputs`, with `emit-report`'s activation bound to the steps `source` transcribes (ADR-0138).
#[cfg(test)]
fn with_steps(inputs: rk_kernel::replay::RunInputs, source: &str) -> rk_kernel::replay::RunInputs {
    let catalog = rk_kernel::catalog::Catalog::from_sources(&[("emit-report.toml", source)])
        .expect("the activity loads");
    let steps = catalog
        .activities()
        .iter()
        .find(|a| a.id == "emit-report")
        .map(|a| a.steps.clone())
        .expect("the catalog holds emit-report");
    inputs.with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::with_steps(
            "emit-report@1",
            crate::support::any_activity_spec(),
            steps,
        ),
    )
}

/// The `gate_opened` rows of the human gate's kind among `rows`.
#[cfg(test)]
fn review_gates(rows: &[rk_types::event::Event]) -> Vec<&rk_types::event::Event> {
    rows.iter()
        .filter(|e| e.kind == EventKind::GateOpened && e.payload["kind"] == REASON)
        .collect()
}

/// **The refusal after a committed agent step opens one human gate** (`s8i` §8i.8; ADR-0199
/// decision 3, proposed). The agent step dispatches and commits; the `change_summary` step then
/// reads the `Binary` value and refuses, so the kernel returns `StepFailedAfterCommit` carrying
/// exactly the routed toolchain's refusal. That is the refusal's own outcome: exactly one gate
/// opens and `run` exits `0`.
#[test]
fn a_refusal_after_a_committed_agent_step_opens_one_human_gate() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let source = summary_after_agent_activity();
    install_summarising(root, &source);
    let resolved = crate::support::resolved_over_pack_min(root, "story-1");
    let (inputs, tree_hash) = crate::support::inputs_over_pack_min(&resolved);
    let inputs = with_steps(inputs, &source);
    let routing = rk_cli::verbs::run::tool_routing(root, None).expect("the registry loads");
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let harness = Arc::new(crate::support::ScriptedHarness::scripted(vec![Ok(
        crate::support::impl_report(),
    )]));
    // A second handle on the counting harness: the run takes one, the assertion reads the other.
    let dispatcher: Arc<dyn rk_ports::harness::Harness> =
        Arc::<crate::support::ScriptedHarness>::clone(&harness);
    let mut ports = crate::support::ports(dispatcher, &store);
    ports.toolchain = Arc::new(FixtureSummary::new(ChangeSummary::Binary(binary_summary())));
    let mut supply = crate::support::any_spec("emit-report@1", crate::support::any_activity_spec());

    let exit = rk_cli::verbs::run::run_with_advance(
        &resolved,
        &inputs,
        crate::support::FIXED_NOW,
        &tree_hash,
        ports,
        routing,
        &crate::support::clean_env(),
        &mut supply,
        &rk_ports::cancel::CancelToken::new(),
        &mut None,
        &mut false,
        None,
        None,
    )
    .expect("the run returns an exit, not a halt");

    assert_eq!(
        harness.dispatches(),
        1,
        "PREMISE: the agent step before the summary dispatched once"
    );
    let rows = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("story-1"))
        .expect("the fixture store queries");
    assert!(
        rows.iter().any(|e| e.kind == EventKind::ActivityCommitted),
        "PREMISE: the agent step committed before the summary refused: {:?}",
        rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
    );
    assert_eq!(
        review_gates(&rows).len(),
        1,
        "exactly one human gate opens after the committed step: {:?}",
        rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
    );
    assert_eq!(
        exit,
        rk_cli::exit::Exit::Success,
        "`run` exits 0 at a gate (golden constraint 2)"
    );
}

/// **The bound review gate is a tier-1 act through a sibling of the stage's boundary** (`s8i`
/// §8i.8; ADR-0199 decision 3, proposed; `s8e` §8e.3's N6), modelled on the refused landing's
/// gate. A live boundary over a real run tree (`rk_git::worktree::enter`, as `run`'s step (8)
/// makes it) and the repository's real store, its ownership read back from the `worktree_assigned`
/// row as `run_lane` reads it. The `Binary` item's walk refuses; the gate's `opened` row goes
/// through `gate_rows_through` as an act: its `Gate:` mark is the run ref's new tip, naming the
/// closed reason and `opened`, and the row is anchored to that commit. No dispatch is made.
#[test]
fn the_bound_review_gate_is_a_tier_one_act_through_the_stages_sibling_boundary() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    let item = "story-1";
    let source = summarising_activity();
    install_summarising(root, &source);
    let resolved = crate::support::resolved_over_pack_min(root, item);
    let found = rk_cli::repo::discover(Some(root)).expect("the repository discovers");

    // The run tree and its lock owner, as `run`'s step (8) enters them.
    let me = std::process::id();
    let start = rk_containment::process_start_time(me)
        .expect("PREMISE: the OS reports this test process's start time");
    let no_process = |_pid: u32| -> Option<u64> { None };
    let entered = rk_git::worktree::enter(
        &found.exec,
        root,
        item,
        &rk_git::worktree::LockOwner { pid: me, start },
        &no_process,
        None,
    )
    .expect("the run tree is created");
    let tree = entered.tree.path.clone();

    // The repository's real store, and the ownership `run_lane` reads back from its row.
    let store: Arc<Mutex<dyn rk_ports::event_store::EventStore>> = Arc::new(Mutex::new(
        rk_adapter_sqlite::SqliteStore::open(&found.exec, &found.worktree)
            .expect("the store opens"),
    ));
    store
        .lock()
        .expect("the store's lock is sound")
        .append(rk_kernel::runtime::tier_one::worktree_assigned_event(
            item,
            &tree,
            &entered.tree.branch,
            false,
            &entered.discarded,
        ))
        .expect("the worktree_assigned row is written");
    let ownership = rk_cli::verbs::run::assigned_ownership(&store, item)
        .expect("a sound lock")
        .expect("PREMISE: the worktree_assigned row reads back");
    let mut boundary = rk_kernel::runtime::tier_one::Boundary::live(
        found.exec,
        rk_kernel::runtime::tier_one::Target {
            worktree: tree.clone(),
            ownership,
            project: resolved.project.clone(),
            item: item.to_owned(),
        },
    );
    boundary
        .take_baseline(&store)
        .expect("the created tree's baseline is read");
    let tip_before = crate::landing::rev(root, &format!("refs/heads/rk-run/{item}"))
        .expect("PREMISE: the run ref exists once the tree is created");

    let (inputs, tree_hash) = rk_cli::verbs::run::prepare_inputs(
        &resolved,
        &rk_git::path::WorktreePath::new(&tree).expect("the run tree's path normalises"),
        crate::support::FIXED_NOW,
    )
    .expect("the input set builds over the run tree");
    let inputs = with_steps(inputs, &source);
    let routing = rk_cli::verbs::run::tool_routing(root, None).expect("the registry loads");
    let harness = Arc::new(crate::support::ScriptedHarness::scripted(vec![Ok(
        crate::support::impl_report(),
    )]));
    let dispatcher: Arc<dyn rk_ports::harness::Harness> =
        Arc::<crate::support::ScriptedHarness>::clone(&harness);
    let mut ports = crate::support::ports(dispatcher, &store);
    ports.toolchain = Arc::new(FixtureSummary::new(ChangeSummary::Binary(binary_summary())));
    let mut supply = crate::support::any_spec("emit-report@1", crate::support::any_activity_spec());

    let exit = rk_cli::verbs::run::run_with_advance(
        &resolved,
        &inputs,
        crate::support::FIXED_NOW,
        &tree_hash,
        ports,
        routing,
        &crate::support::clean_env(),
        &mut supply,
        &rk_ports::cancel::CancelToken::new(),
        &mut None,
        &mut false,
        Some(boundary),
        None,
    )
    .expect("the run returns an exit, not a halt");

    assert_eq!(
        exit,
        rk_cli::exit::Exit::Success,
        "`run` exits 0 at a gate (golden constraint 2)"
    );
    assert_eq!(
        harness.dispatches(),
        0,
        "a Binary item's stage dispatched: no automated reviewer may run"
    );
    let rows = store
        .lock()
        .expect("the store's lock is sound")
        .query(rk_kernel::runtime::journal_query(item))
        .expect("the store reads");
    let gates = review_gates(&rows);
    let [gate] = gates.as_slice() else {
        panic!(
            "exactly one human gate opens: {:?}",
            rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
        );
    };
    let token = gate.payload["token"]
        .as_str()
        .expect("the opened row names its token")
        .to_owned();
    let tip = crate::landing::rev(root, &format!("refs/heads/rk-run/{item}"))
        .expect("the run ref exists");
    assert_ne!(
        tip, tip_before,
        "the gate's opening committed nothing on the run ref: it was appended, not acted"
    );
    let trailers = crate::support::git_ok(
        root,
        &["log", "-1", "--format=%(trailers:key=Gate,valueonly)", &tip],
    );
    let gate_mark = String::from_utf8_lossy(&trailers.stdout).trim().to_owned();
    let words: Vec<&str> = gate_mark.split_whitespace().collect();
    assert!(
        words.contains(&REASON) && words.contains(&"opened") && words.contains(&token.as_str()),
        "the run ref's tip is the human gate's `opened` mark naming its kind and token: \
         {gate_mark:?}"
    );
    let anchored = store
        .lock()
        .expect("the store's lock is sound")
        .query(rk_ports::event_store::Query::BySha(tip.clone()))
        .expect("the store reads");
    assert_eq!(
        review_gates(&anchored).len(),
        1,
        "the opened row is anchored to its `Gate:` commit {tip}: {:?}",
        anchored
            .iter()
            .map(|e| e.kind.wire_name())
            .collect::<Vec<_>>()
    );
}
