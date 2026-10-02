//! **The two review verb rows** — `shallow_clone` and `verdict_record`, each routed by a row of the
//! shipped `.redkiln/registry.toml` onto a route R5 adds to the closed set by record (`s8d` §8d.6;
//! R5 = ADR-0177, proposed). *(Added 2026-09-26, `s8d` build, package P7.)*
//!
//! Each test drives the KERNEL's `tool` arm through the shipped registry and `rk-cli`'s own
//! `ReviewCloneRoutes`, the value the composition root binds: nothing here calls the clone or
//! `review::confirm` directly, so a row that routed elsewhere would be red.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_cli::review_pass::ReviewCloneRoutes;
use rk_kernel::catalog::{InputValue, Step, StepKind, StepTimeout};
use rk_kernel::predicate::Context;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::reference::Scope;
use rk_kernel::runtime::step::{StepVerdict, ToolRouting};
use rk_kernel::workflow::memo::{Route, ToolchainValues, VerbRegistry};
use rk_ports::event_store::EventStore;
use rk_ports::toolchain::{
    ChangeSummary, Classification, Selection, StepBound, TestOutcome, TestRunner, Toolchain,
    VerbCall, VerbOutput,
};
use rk_types::artifact::findings::{EvidenceLabel, Finding, Findings};
use rk_types::error::StepError;
use rk_types::repro::{Expected, Repro, ReproKind};
use rk_types::text::Text;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// The SHIPPED registry, loaded as `run` loads it.
fn shipped_registry() -> VerbRegistry {
    let path = workspace_root().join(".redkiln").join("registry.toml");
    VerbRegistry::load_file(&path).unwrap_or_else(|e| panic!("the shipped registry loads: {e}"))
}

/// A `Runtime` whose `tool` arm routes through `registry` and `routes`.
fn runtime(registry: VerbRegistry, routes: ReviewCloneRoutes) -> Runtime {
    let store: Arc<Mutex<dyn EventStore>> =
        Arc::new(Mutex::new(rk_ports::fakes::VecStore::default()));
    let harness: Arc<dyn rk_ports::harness::Harness> =
        Arc::new(rk_ports::fakes::FakeHarness::scripted(Vec::new()));
    Runtime::new(
        crate::support::ports(harness, &store),
        "run-verb-rows",
        "review",
    )
    .with_tool_routing(
        ToolRouting::new(
            registry,
            ToolchainValues::unselected("no value table in this test"),
        )
        .with_review_routes(Arc::new(routes)),
    )
}

/// A step of `kind` with every optional member absent.
fn base_step(id: &str, kind: StepKind) -> Step {
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

/// A `tool` step running `toolchain:<verb>` with literal/reference `inputs`.
fn tool_step(id: &str, verb: &str, inputs: &[(&str, &str)], secs: u64) -> Step {
    Step {
        run: Some(format!("toolchain:{verb}")),
        inputs: inputs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), InputValue::Text((*v).to_owned())))
            .collect(),
        timeout_secs: Some(StepTimeout::Literal(secs)),
        ..base_step(id, StepKind::Tool)
    }
}

/// The fixture's git, through `rk-git`'s one process-exec site (`git_exec::run`, *"Used by the startup
/// probe … and by fixtures"*). *(2026-09-27, `s8d` build, package R2FIX-8d: this was a
/// process `Command` for `git` of its own, which Checkpoint 6's and CI's wrapper-only bans read; routed
/// through the wrapper so `s8d` adds no site to either.)*
fn git(cwd: &Path, args: &[&str]) -> String {
    rk_git::git_exec::run(cwd, args)
        .unwrap_or_else(|e| panic!("git {args:?} failed: {e:?}"))
        .stdout_trimmed()
        .trim()
        .to_owned()
}

/// A source repository with three commits, a tag and a second branch, serving any sha — AC-191's
/// fixture shape (`rk-git`'s `review_clone.rs`): the clone must leave all of that behind.
fn source_repo(dir: &Path) -> (String, String) {
    std::fs::create_dir_all(dir).expect("mkdir");
    git(dir, &["init", "--quiet"]);
    git(dir, &["config", "user.email", "test@example.invalid"]);
    git(dir, &["config", "user.name", "Test"]);
    for (n, body) in [("one", "1\n"), ("two", "2\n"), ("three", "3\n")] {
        std::fs::write(dir.join("f.txt"), body).expect("write");
        git(dir, &["add", "f.txt"]);
        git(dir, &["commit", "--quiet", "-m", n]);
    }
    git(dir, &["tag", "v1"]);
    git(dir, &["branch", "sidebranch"]);
    git(dir, &["config", "uploadpack.allowAnySHA1InWant", "true"]);
    let sha = git(dir, &["rev-parse", "HEAD~1"]);
    let url = format!("file:///{}", dir.display().to_string().replace('\\', "/"));
    (url, sha)
}

fn no_toolchain() -> rk_cli::review_pass::ToolchainAt {
    Box::new(|root: &Path| {
        Err(format!(
            "no toolchain is bound in this test at {}",
            root.display()
        ))
    })
}

/// **`shallow_clone` routes to the retained review clone** (ADR-0098 decision 2; R5 decisions 1-4).
///
/// The shipped row takes the `review_clone` route, is `side_effecting`, and types its output
/// `Workspace`; the kernel's arm, through `rk-cli`'s routes, clones at the sha with a working tree
/// CHECKED OUT (the file at the sha, not the tip), while the object set stays AC-191's: one commit,
/// no tag, no other branch. The committed value names the path, the sha and the working-tree choice.
#[test]
fn shallow_clone_routes_to_the_retained_review_clone() {
    let registry = shipped_registry();
    let row = registry
        .entry("shallow_clone")
        .expect("the shipped registry has a `shallow_clone` row (R5)");
    assert_eq!(row.route, Route::ReviewClone, "`shallow_clone`'s route");
    assert!(
        row.side_effecting,
        "`shallow_clone` is side_effecting (R1, R5)"
    );
    assert_eq!(row.outputs, "Workspace");
    assert_eq!(row.inputs, ["sha"]);

    let tmp = tempfile::tempdir().expect("a temp dir");
    let (url, sha) = source_repo(&tmp.path().join("origin"));
    let clones = tmp.path().join("clones");
    let rt = runtime(
        registry,
        ReviewCloneRoutes {
            source_url: url,
            clones_dir: clones.clone(),
            reference_root: tmp.path().join("origin"),
            toolchain_at: no_toolchain(),
        },
    );
    let step = tool_step("clone", "shallow_clone", &[("sha", &sha)], 120);
    let (verdict, output) = rt
        .execute_step_output(
            &step,
            &Context::new(),
            &Scope::new(std::slice::from_ref(&step)),
        )
        .expect("the step resolves");
    assert_eq!(verdict, StepVerdict::Continue, "the clone step continues");
    let (kind, value) = output.expect("the clone step has a typed output");
    assert_eq!(kind, "Workspace");
    let dest = clones.join(&sha);
    assert_eq!(
        value,
        serde_json::json!({
            "path": dest.display().to_string(),
            "sha": sha,
            "working_tree": "declared",
        }),
        "the committed `Workspace` names the path, the sha and the working-tree choice"
    );

    // THE CHECKED-OUT FILE, AT THE SHA: `f.txt` reads its content at HEAD~1 (`2`), not the tip's.
    let checked_out = std::fs::read_to_string(dest.join("f.txt")).unwrap_or_else(|e| {
        panic!(
            "no checked-out `f.txt` in the review clone at {}: {e} — `WorkingTree::Declared` \
             fetched and checked out nothing (R5 decision 4)",
            dest.display()
        )
    });
    // The line terminator is the checkout's (a `core.autocrlf` in the operator's git config applies
    // to a gix checkout as to `git clone`); the CONTENT is the sha's: `2`, not the tip's `3`.
    assert_eq!(
        checked_out.trim_end(),
        "2",
        "the working tree is the tree AT the reviewed sha"
    );
    assert_eq!(
        git(&dest, &["rev-parse", "HEAD"]),
        sha,
        "HEAD is the reviewed sha"
    );
    assert!(
        git(&dest, &["status", "--porcelain"]).is_empty(),
        "the checked-out tree is exactly the sha's: git reports no change against it"
    );

    // AC-191: exactly one commit, and none of the source's other refs.
    assert_eq!(
        git(&dest, &["rev-list", "--count", "--all"]),
        "1",
        "the review clone carries exactly one commit (AC-191)"
    );
    assert!(
        git(&dest, &["tag", "-l"]).is_empty(),
        "the clone carries a tag"
    );
    assert!(
        !git(&dest, &["for-each-ref", "--format=%(refname)"]).contains("sidebranch"),
        "the clone carries the source's other branch"
    );
}

// ---- verdict_record ------------------------------------------------------------------------------

/// A toolchain whose `test` fails every selected test (`red`) or passes it.
struct Arm {
    red: bool,
    calls: Arc<Mutex<u32>>,
}

impl Toolchain for Arm {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self, _b: StepBound) -> Result<rk_types::verb_output::BuildReport, StepError> {
        Err(refusal("build"))
    }
    fn test(&self, selection: &Selection, _b: StepBound) -> Result<TestOutcome, StepError> {
        *self.calls.lock().expect("the counter") += 1;
        let Selection::Filter(names) = selection else {
            return Err(refusal("a selection other than a filter"));
        };
        let executed = u32::try_from(names.len()).unwrap_or(0);
        let failed = if self.red { executed } else { 0 };
        TestOutcome::new(
            rk_types::verify::VerifyReport {
                basis: rk_types::verify::Basis::WholeSuite,
                argv: vec!["scripted".to_owned()],
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
    fn lint(&self, _b: StepBound) -> Result<(), StepError> {
        Err(refusal("lint"))
    }
    fn classify(&self, _p: &str) -> Result<Classification, StepError> {
        Err(refusal("classify"))
    }
    fn change_summary(&self, _r: &str, _b: StepBound) -> Result<ChangeSummary, StepError> {
        Err(refusal("change_summary"))
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        Err(refusal(call.verb))
    }
}

fn refusal(what: &str) -> StepError {
    StepError::Unsupported {
        capability: format!("the scripted arm does not implement {what}"),
    }
}

fn finding(ac: &str, test: &str) -> Finding {
    Finding {
        ac: ac.to_owned(),
        claim: Text::new(format!("{ac}: the claim")),
        evidence: EvidenceLabel::Reproduced,
        // *(dated 2026-10-02, s8h build, package P2-rk-types: `Finding.severity` is required, ADR-0197 d6.)*
        severity: rk_types::artifact::findings::FindingSeverity::Blocking,
        rationale: Text::new("the rationale"),
        repro: Repro {
            kind: ReproKind::Run,
            command: format!("cargo test -- {test}"),
            expected: Expected::Exit { exit: 0 },
            observed: "exit 101".to_owned(),
            expected_when_met: "exit 0".to_owned(),
            mutation: None,
        },
        files: vec![],
    }
}

/// **`verdict_record` routes to `review::confirm`** (ADR-0162; R5 decision 5).
///
/// The shipped row takes the `review_confirm` route and is `side_effecting`. Over one finding, with
/// the candidate arm rooted in the clone the activation's `review_clone` step committed and red
/// there, and the reference arm green, `review::confirm` classifies the flip `Reproduced` and, at
/// the `mandatory` stage rigor's `Strict` tier, BLOCKS it — the `Verdict`'s `blocking` names it. At
/// an `optional` rigor the tier is `ReadOnly` (AC-302), which re-executes nothing and blocks nothing
/// (AC-299). A finding the candidate does not reproduce is an advisory. Each reading is one the
/// classifier and the policy table produce and a pass-through could not.
#[test]
fn verdict_record_routes_to_review_confirm() {
    let registry = shipped_registry();
    let row = registry
        .entry("verdict_record")
        .expect("the shipped registry has a `verdict_record` row (R5)");
    assert_eq!(row.route, Route::ReviewConfirm, "`verdict_record`'s route");
    assert!(
        row.side_effecting,
        "`verdict_record` is side_effecting (R1, R5)"
    );
    assert_eq!(row.outputs, "Verdict");

    let tmp = tempfile::tempdir().expect("a temp dir");
    let clone_path = tmp.path().join("clones").join("c".repeat(40));
    let reference_root = tmp.path().join("reference");
    let sha = "c".repeat(40);
    let workspace = serde_json::json!({
        "path": clone_path.display().to_string(),
        "sha": sha,
        "working_tree": "declared",
    });
    let calls = Arc::new(Mutex::new(0_u32));

    let verdict_at = |rigor: Rigor, candidate_red: bool| {
        let clone_for = clone_path.clone();
        let counter = Arc::clone(&calls);
        let toolchain_at: rk_cli::review_pass::ToolchainAt = Box::new(move |root: &Path| {
            let red = candidate_red && root == clone_for;
            Ok(Arc::new(Arm {
                red,
                calls: Arc::clone(&counter),
            }) as Arc<dyn Toolchain>)
        });
        let rt = runtime(
            registry.clone(),
            ReviewCloneRoutes {
                source_url: "file:///unused".to_owned(),
                clones_dir: tmp.path().join("clones"),
                reference_root: reference_root.clone(),
                toolchain_at,
            },
        );
        let clone = tool_step("clone", "shallow_clone", &[("sha", &sha)], 120);
        let review = Step {
            role: Some("reviewer".to_owned()),
            ..base_step("review", StepKind::Agent)
        };
        let record = tool_step(
            "record",
            "verdict_record",
            &[
                ("findings", "${review.outputs}"),
                ("override", "${override.answer?}"),
            ],
            30,
        );
        let gate = Step {
            question: Some("review.blocking".to_owned()),
            ..base_step("override", StepKind::Gate)
        };
        let steps = vec![clone, review, gate, record.clone()];
        let mut scope = Scope::new(&steps);
        scope.bind_output("clone", workspace.clone());
        scope.bind_output(
            "review",
            serde_json::to_value(Findings {
                items: vec![finding("AC-1", "the_bug_is_fixed")],
            })
            .expect("the finding set serialises"),
        );
        scope.enter("record");
        let facets = rk_kernel::registry::Registry::from_toml(
            &std::fs::read_to_string(workspace_root().join(".redkiln").join("facets.toml"))
                .expect("facets.toml reads"),
        )
        .expect("facets.toml loads");
        let ctx = rk_kernel::rigor::with_rigor(Context::new(), rigor, &facets);
        rt.execute_step_output(&record, &ctx, &scope)
            .expect("the step resolves")
    };

    // ---- STRICT: the candidate reproduces, the reference does not — a flip, and a block. ----------
    let (verdict, output) = verdict_at(Rigor::Mandatory, true);
    assert_eq!(
        verdict,
        StepVerdict::Continue,
        "the record step continues: {verdict:?}"
    );
    let (kind, value) = output.expect("the record step has a typed output");
    assert_eq!(kind, "Verdict");
    assert_eq!(value["sha"], sha, "the verdict is at the clone's sha");
    assert_eq!(
        value["weight"], "strict",
        "mandatory rigor resolves the Strict tier (AC-302)"
    );
    let blocking = value["blocking"].as_array().expect("a blocking list");
    assert_eq!(
        blocking.len(),
        1,
        "the reproduced finding BLOCKS at Strict — `review::confirm`'s decision: {value}"
    );
    assert_eq!(blocking[0]["ac"], "AC-1");
    assert_eq!(blocking[0]["class"], "reproduced");
    assert!(
        blocking[0]["finding_id"]
            .as_str()
            .is_some_and(|d| !d.is_empty()),
        "each finding carries its AC-305 identity"
    );
    assert!(value["override"].is_null(), "no override was answered");
    assert_eq!(
        *calls.lock().expect("the counter"),
        2,
        "both arms re-executed once"
    );

    // ---- NOT REPRODUCED: both arms green — no flip, so an advisory, never a block. ---------------
    let (_, output) = verdict_at(Rigor::Mandatory, false);
    let (_, value) = output.expect("an output");
    assert_eq!(
        value["blocking"].as_array().map(Vec::len),
        Some(0),
        "a finding the candidate does not reproduce blocks nothing: {value}"
    );
    assert_eq!(value["advisories"].as_array().map(Vec::len), Some(1));

    // ---- READ-ONLY: `optional` rigor re-executes nothing and blocks nothing (AC-299, AC-302). ---
    *calls.lock().expect("the counter") = 0;
    let (_, output) = verdict_at(Rigor::Optional, true);
    let (_, value) = output.expect("an output");
    assert_eq!(value["weight"], "read_only");
    assert_eq!(value["blocking"].as_array().map(Vec::len), Some(0));
    assert_eq!(
        *calls.lock().expect("the counter"),
        0,
        "the read-only tier dispatches no re-execution (AC-299)"
    );

    // ---- NO CLONE COMMITTED: refused by name, never confirmed against some other tree. -----------
    let rt = runtime(
        registry.clone(),
        ReviewCloneRoutes {
            source_url: "file:///unused".to_owned(),
            clones_dir: tmp.path().join("clones"),
            reference_root,
            toolchain_at: no_toolchain(),
        },
    );
    let record = tool_step(
        "record",
        "verdict_record",
        &[
            ("findings", "${review.outputs}"),
            ("override", "${override.answer?}"),
        ],
        30,
    );
    let review = base_step("review", StepKind::Agent);
    let gate = Step {
        question: Some("review.blocking".to_owned()),
        ..base_step("override", StepKind::Gate)
    };
    let steps = vec![review, gate, record.clone()];
    let mut scope = Scope::new(&steps);
    scope.bind_output(
        "review",
        serde_json::to_value(Findings {
            items: vec![finding("AC-1", "t")],
        })
        .expect("serialises"),
    );
    scope.enter("record");
    let facets = rk_kernel::registry::Registry::from_toml(
        &std::fs::read_to_string(workspace_root().join(".redkiln").join("facets.toml"))
            .expect("facets.toml reads"),
    )
    .expect("facets.toml loads");
    let ctx = rk_kernel::rigor::with_rigor(Context::new(), Rigor::Mandatory, &facets);
    let (verdict, output) = rt
        .execute_step_output(&record, &ctx, &scope)
        .expect("the step resolves");
    assert!(output.is_none(), "a refused record commits no verdict");
    match verdict {
        StepVerdict::Failed(StepError::Unsupported { capability }) => assert!(
            capability.contains("review_clone"),
            "the refusal names the missing review clone: {capability}"
        ),
        other => panic!("a verdict with no committed clone must refuse by name: {other:?}"),
    }
}
