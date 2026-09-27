//! **The walk table** — `s8d` §8d.9, Checkpoint 8d (6) and (6a).
//!
//! §8d.9: *"The executor reaches the activity's `emit` and commits it, over the **shipped**
//! `.redkiln/` pack and catalog, with a scripted `FakeHarness` returning happy-path typed values per
//! step, a scripted `Toolchain` over the verbs Checkpoint 8c routes, specs supplied by the test (the
//! shipped cards and schemas are `s8h`'s), and each stage's `requires` seeded from a fixture artifact
//! so one stage's refusal does not mask the next stage's verdict. A happy-path gate is answered with
//! its question's first option. That includes N3's `unknown_facet:change` gate at triage, which is
//! answered `text`."*
//!
//! Each test asserts, stage by stage, the selected set (a real [`select`] run over the shipped pack
//! with the lane's item) and each selected activity's verdict (a real
//! [`Runtime::run_activity_steps`] walk of its shipped step list) **exactly**, and prints one
//! `LANE_WALK <lane> walked=<n> refused=<m>` line, which Checkpoint 8d (6a) floors. §8d.9: *"If a
//! test measures a different row, **the table is corrected by a dated line; the test is not bent to
//! the table.**"* The expected rows below are therefore §8d.9's table as corrected by its dated
//! lines, and each row that differs from the section's first claim cites that line.
//!
//! **What the rig is, and what it is not.**
//!
//! - **The pack, catalog, artifact registry, facet registry, verb registry and question file are
//!   read off disk** from `.redkiln/` — never a fixture copy, which could drift from what `redkiln
//!   run` walks.
//! - **The item** is bound as `rk-cli`'s `build_item` binds one: every non-kernel facet the lane does
//!   not state is an explicit `null` (an absent facet must be nulled explicitly), `kind` is the item's
//!   own, and the ordered facets carry `facets.toml`'s domain. `toolchain` is `["rust"]` for both lanes
//!   (the repository's one toolchain; the feed table `[feed.rust.*]` is the one a `toolchain:<feed>`
//!   input reads). `change` is **not** set on the record: it is `null` until the triage gate's answer
//!   binds it (N3), exactly as `rk-cli`'s root binds `gate::answered_facet`.
//! - **Agent steps** are answered by one scripted [`FakeHarness`] PER STEP, chosen by the per-step
//!   spec's schema id (`<Kind>@<activity>/<step>`), each scripted with that step's happy-path value a
//!   bounded number of times. A dispatch past a step's script is `FakeHarness`'s own *"script
//!   exhausted"* refusal, so an unbounded loop ends on a named refusal rather than hanging.
//! - **Tool steps** reach [`LaneToolchain`] through the SHIPPED `registry.toml` and its `[toolchain.rust]`
//!   value table: `test` answers failing for a name beginning `red_` or `hypothesis_` (the two
//!   selections a happy path must see fail: `tdd`'s red phase and `root-cause`'s discriminating test)
//!   and passing otherwise; `build` is green; `change_summary` answers the lane's one-path `Text`
//!   summary; `run_verb` answers `sca_scan` clean and `package` a built artifact, and refuses every
//!   other verb by its bare name. The two review routes (`shallow_clone`, `verdict_record`) are bound
//!   to [`LaneReview`], which clones nothing and re-executes nothing (the scripted review returns no
//!   finding, so `review::confirm` has nothing to re-run).
//! - **Seeding.** Before each activity walks, every MANDATORY `requires` kind of that activity with no
//!   committed `emit` binding in the item's journal is seeded with a fixture completion row
//!   (`lane-walk-seed-<Kind>@1`), which is where `artifact:<Kind>` reads from (R2 = ADR-0173,
//!   proposed). An optional `requires` kind is never seeded: its absence reads `null` (ADR-0032). The
//!   seeds are counted and printed; a kind an upstream activity DID emit is read from that emit, never
//!   seeded over.
//! - **Gates** the walk opens are answered with their question's FIRST option from
//!   `.redkiln/processes/questions.toml`, through the fold (`gate_answered`), and the walk re-entered.
//!
//! 🛑 **It is not a real run and says nothing about `redkiln run`**, which is `s8e`'s (§8d.9).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Activity, StepKind};
use rk_kernel::registry::Registry;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::{Activation, GateSite, ReviewRoutes, ToolRouting};
use rk_kernel::select::{ArtifactRegistry, Catalog, Item, Pack, select};
use rk_kernel::workflow::envelope::AttemptCap;
use rk_kernel::workflow::memo::VerbRegistry;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::fakes::FakeHarness;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_ports::toolchain::{
    ChangeKind, ChangeSummary, Classification, ManifestEntry, Selection, StepBound, TestOutcome,
    TestRunner, TextChangeSummary, Toolchain, VerbCall, VerbOutput,
};
use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::typed::{SchemaId, Typed};

use crate::runtime::{TREE_HASH, at, counters, ports_with_toolchain, store};

type Json = serde_json::Value;

/// The item both lanes walk. One id, so every `artifact:` read and every gate fold is this item's.
const ITEM: &str = "LANE-WALK-1";

/// How many times one agent step's `FakeHarness` answers. Above any shipped bound a happy path takes
/// (`tdd`'s `red` runs once per acceptance criterion, two here), below an unbounded loop.
const PER_STEP_SCRIPT: usize = 6;

/// How many gate rounds one activation may take before the rig calls it a loop.
const GATE_ROUNDS: usize = 6;

/// The head commit the scripted `change_summary` names: 40 hex, the shape `shallow_clone` demands.
const HEAD_SHA: &str = "a1b2c3d4e5f60718293a4b5c6d7e8f9012345678";

// ---- what a row asserts ----------------------------------------------------------------------------

/// One selected activity's verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Verdict {
    /// The walk journalled the activation's completion row and no step refused.
    Walks,
    /// The walk refused, and its refusal names this text.
    Refuses(&'static str),
}

/// One stage's row: the stage, and each selected activity with its verdict, in the plan's order.
struct Row {
    stage: &'static str,
    selected: &'static [(&'static str, Verdict)],
}

/// A lane: its pack, its item's facets, and its table.
struct Lane {
    name: &'static str,
    pack: &'static str,
    kind: &'static str,
    /// `(facet, value)` the item's record states; every other non-kernel facet is `null`.
    facets: &'static [(&'static str, FacetIs)],
    /// The one path the scripted `change_summary` reports.
    path: &'static str,
    rows: &'static [Row],
    /// `(stage, needle)`: a stage whose SELECTION refuses, naming `needle`, so nothing there walks.
    selection_refusals: &'static [(&'static str, &'static str)],
}

#[derive(Debug, Clone, Copy)]
enum FacetIs {
    Str(&'static str),
    Ordinal(&'static str),
    Bool(bool),
}

// ---- the shipped pack, read off disk ---------------------------------------------------------------

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()))
}

struct Shipped {
    pack: Pack,
    catalog: Catalog,
    artifacts: ArtifactRegistry,
    facets: Registry,
    verbs: VerbRegistry,
    /// Question id → its first option, from `.redkiln/processes/questions.toml`.
    first_option: BTreeMap<String, String>,
    redkiln: PathBuf,
}

fn shipped(pack_file: &str) -> Shipped {
    let redkiln = root().join(".redkiln");
    let pack = Pack::from_toml(pack_file, &read(&redkiln.join("packs").join(pack_file)))
        .unwrap_or_else(|e| panic!("the shipped pack {pack_file} loads: {e}"));
    let catalog = Catalog::from_dir(&redkiln.join("catalog"))
        .unwrap_or_else(|e| panic!("the shipped catalog loads: {e:?}"));
    let artifacts =
        ArtifactRegistry::from_toml("artifacts.toml", &read(&redkiln.join("artifacts.toml")))
            .unwrap_or_else(|e| panic!("the shipped artifact registry loads: {e:?}"));
    let facets = Registry::from_toml(&read(&redkiln.join("facets.toml")))
        .unwrap_or_else(|e| panic!("the shipped facet registry loads: {e:?}"));
    let verbs = VerbRegistry::load_file(&redkiln.join("registry.toml"))
        .unwrap_or_else(|e| panic!("the shipped verb registry loads: {e}"));
    let first_option = first_options(&read(&redkiln.join("processes").join("questions.toml")));
    assert!(
        first_option.len() >= 10,
        "PREMISE: .redkiln/processes/questions.toml declares {} questions; §8d.6 ships ten, so the \
         gates below would be answered from nothing",
        first_option.len()
    );
    Shipped {
        pack,
        catalog,
        artifacts,
        facets,
        verbs,
        first_option,
        redkiln,
    }
}

/// Each `[question."<id>"]`'s first option: the first quoted token after its `options = [`. An
/// option written `{ id = "x", attaches = "K" }` yields `x`, its first quoted token.
fn first_options(src: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in src.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        if let Some(rest) = t.strip_prefix("[question.\"") {
            current = rest.strip_suffix("\"]").map(str::to_owned);
            continue;
        }
        if let (Some(id), Some(rest)) = (current.as_ref(), t.strip_prefix("options")) {
            let first = rest.split('"').nth(1).map(str::to_owned);
            if let Some(first) = first {
                out.insert(id.clone(), first);
            }
        }
    }
    out
}

/// The item as `rk-cli`'s `build_item` binds one, before triage: `change` is `null`.
fn item(shipped: &Shipped, lane: &Lane, change: Option<&str>) -> Item {
    let mut item = Item::new(ITEM, lane.kind);
    for facet in shipped.facets.facets() {
        if facet.name == "kind" || facet.source == rk_kernel::registry::Source::Kernel {
            continue;
        }
        item = item.with_facet_null(&facet.name);
    }
    for (name, value) in lane.facets {
        item = match value {
            FacetIs::Str(s) => item.with_facet_str(name, s),
            FacetIs::Bool(b) => item.with_facet_bool(name, *b),
            FacetIs::Ordinal(member) => {
                let domain: Vec<String> = shipped
                    .facets
                    .facet(name)
                    .unwrap_or_else(|| panic!("facets.toml declares `{name}`"))
                    .values
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .unwrap_or_else(|| panic!("`{name}`'s values are strings"))
                            .to_owned()
                    })
                    .collect();
                let refs: Vec<&str> = domain.iter().map(String::as_str).collect();
                item.with_facet_ordinal(name, &refs, member)
            }
        };
    }
    item = item.with_facet_set("toolchain", ["rust"]);
    match change {
        Some(kind) => item.with_toolchain("rust", [kind]),
        None => item,
    }
}

// ---- the doubles -----------------------------------------------------------------------------------

/// One scripted [`FakeHarness`] per agent step, chosen by the per-step spec's schema id.
struct LaneHarness {
    per_step: BTreeMap<String, FakeHarness>,
}

impl Harness for LaneHarness {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        match self.per_step.get(spec.schema.as_str()) {
            Some(fake) => fake.dispatch(spec, store),
            None => FakeHarness::scripted(vec![Err(StepError::Unsupported {
                capability: format!(
                    "LANE-WALK RIG: no scripted FakeHarness for the spec `{}`",
                    spec.schema.as_str()
                ),
            })])
            .dispatch(spec, store),
        }
    }
}

/// The happy-path value one agent step returns, by its declared output kind.
fn happy(kind: &str) -> Json {
    match kind {
        "ReproAttempt" => serde_json::json!({ "status": "reproduced", "command": ["repro_crash"] }),
        "ReproCase" => serde_json::json!({ "command": ["repro_crash"], "steps": ["run it"] }),
        "HypothesisLog" => serde_json::json!({
            "status": "confirmed",
            "discriminating_test": ["hypothesis_off_by_one"],
            "summary": "an off-by-one in the parser",
            "root_cause": { "summary": "an off-by-one in the parser" },
        }),
        "Findings" => serde_json::json!({ "items": [] }),
        "Spec" => serde_json::json!({ "acceptance_criteria": ["AC-1", "AC-2"] }),
        "SelfCheck" => serde_json::json!({ "passed": 3, "failed": 0, "notes": "all hold" }),
        "ImplReport" => serde_json::json!({ "tests_added": ["red_ac"], "status": "done" }),
        "TestDesign" => serde_json::json!({ "conditions": ["EP-1"], "uncovered_ac": [] }),
        "GherkinScenarios" => serde_json::json!({ "scenarios": ["Scenario: one"] }),
        "Estimate" => serde_json::json!({ "size": "S", "confidence": "high" }),
        "Docs" => serde_json::json!({ "files": ["docs/lane.md"] }),
        "Retrospective" => serde_json::json!({ "went_well": ["x"], "to_change": ["y"] }),
        "DebtRegisterEntry" => serde_json::json!({ "items": [{ "title": "debt" }] }),
        other => serde_json::json!({ "kind": other }),
    }
}

/// The toolchain the tool steps reach through the shipped registry.
struct LaneToolchain {
    path: &'static str,
}

fn verify_report(n: u32) -> rk_types::verify::VerifyReport {
    rk_types::verify::VerifyReport {
        basis: rk_types::verify::Basis::WholeSuite,
        argv: vec!["lane".to_owned(), "test".to_owned()],
        selected_test_count: n,
        total_test_count: n,
        total_test_count_source: rk_types::verify::TotalTestCountSource::OwnExecutedCount,
        wall_secs: 0,
        toolchain_version: "lane".to_owned(),
        suite_sha: "lane".to_owned(),
        basis_reason: Some(rk_types::verify::BasisReason::ProbeUnverified),
    }
}

impl Toolchain for LaneToolchain {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }

    fn build(&self, _bound: StepBound) -> Result<rk_types::verb_output::BuildReport, StepError> {
        Ok(rk_types::verb_output::BuildReport {
            argv: vec!["lane".to_owned(), "build".to_owned()],
            toolchain_version: "lane".to_owned(),
            wall_secs: 0,
            errors: 0,
            status: rk_types::verb_output::BuildStatus::Passing,
        })
    }

    fn test(&self, selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        let names: Vec<String> = match selection {
            Selection::Filter(names) => names.clone(),
            Selection::Named(name) => vec![name.clone()],
            Selection::Affected { changed, .. } => changed.clone(),
            Selection::WholeSuite { .. } => vec!["whole-suite".to_owned()],
        };
        let failing = names
            .iter()
            .any(|n| n.starts_with("red_") || n.starts_with("hypothesis_"));
        let n = u32::try_from(names.len().max(1)).expect("a handful of names");
        let (passed, failed) = if failing { (0, n) } else { (n, 0) };
        TestOutcome::new(
            verify_report(n),
            passed,
            failed,
            TestRunner::named("lane", "0"),
        )
    }

    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        Ok(())
    }

    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        Ok(Classification::Text)
    }

    fn change_summary(&self, _range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        Ok(ChangeSummary::Text(TextChangeSummary {
            sha: HEAD_SHA.to_owned(),
            diff: format!("--- a/{0}\n+++ b/{0}\n", self.path),
            manifest: vec![ManifestEntry {
                path: self.path.to_owned(),
                classification: Classification::Text,
                change: ChangeKind::Modified,
            }],
        }))
    }

    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        match call.verb {
            "sca_scan" => Ok(VerbOutput {
                kind: "ScanReport".to_owned(),
                value: serde_json::json!({
                    "critical": 0, "high": 0, "medium": 0, "low": 0, "unrated": 0,
                    "advisories": [], "tools": [],
                }),
            }),
            "package" => Ok(VerbOutput {
                kind: "BuildArtifact".to_owned(),
                value: serde_json::json!({
                    "profile": "release",
                    "binaries": [],
                    "argv": ["lane", "package"],
                    "toolchain_version": "lane",
                    "sha": HEAD_SHA,
                    "head": HEAD_SHA,
                }),
            }),
            other => Err(StepError::Unsupported {
                capability: other.to_owned(),
            }),
        }
    }
}

/// The two review routes, clone-free: the scripted review returns no finding, so nothing re-runs.
struct LaneReview;

struct NoReExecution;

impl rk_kernel::review::reexec::ReExecutor for NoReExecution {
    fn run(
        &self,
        _finding: &rk_types::artifact::findings::Finding,
        _sha: &str,
    ) -> Option<rk_kernel::review::reexec::ReExecutionRecord> {
        None
    }
}

impl ReviewRoutes for LaneReview {
    fn clone_at(&self, sha: &str) -> Result<Json, StepError> {
        Ok(serde_json::json!({ "path": "lane-walk-clone", "sha": sha, "working_tree": "declared" }))
    }

    fn reexecutor(
        &self,
        _workspace: &Json,
    ) -> Result<Box<dyn rk_kernel::review::reexec::ReExecutor + '_>, StepError> {
        Ok(Box::new(NoReExecution))
    }
}

// ---- the walk --------------------------------------------------------------------------------------

fn journal(db: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    db.lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("the in-memory store never refuses a query")
}

fn append(db: &Arc<Mutex<dyn EventStore>>, event: Event) {
    db.lock()
        .expect("the store lock is not poisoned")
        .append(event)
        .expect("the in-memory store never refuses an append");
}

/// Whether the item's journal carries a committed `emit` binding of `kind`.
fn has_binding(events: &[Event], kind: &str) -> bool {
    events.iter().any(|e| {
        e.kind == EventKind::ActivityCommitted
            && e.payload.get("item").and_then(Json::as_str) == Some(ITEM)
            && e.payload
                .get("artifacts")
                .and_then(|a| a.get(kind))
                .is_some()
    })
}

/// A fixture value for a kind a lane seeds.
fn seed_value(kind: &str, path: &str) -> Json {
    match kind {
        "BugReport" => serde_json::json!({ "title": "a parser off-by-one", "observed": "panic" }),
        "IntakeRecord" => serde_json::json!({ "title": "a spike pack story", "asks": ["one"] }),
        "Changeset" | "ChangeSummary" => serde_json::json!({
            "sha": HEAD_SHA, "diff": "", "files": [path],
            "manifest": [{ "path": path, "classification": "text", "change": "modified" }],
        }),
        other => {
            let kind_of = match other {
                "ReproCase" | "RootCause" | "Spec" | "ImplReport" => other,
                _ => "",
            };
            if kind_of.is_empty() {
                serde_json::json!({ "seeded": other })
            } else {
                happy(kind_of)
            }
        }
    }
}

/// Seed every mandatory `requires` kind of `activity` the journal has no binding for.
fn seed_requires(db: &Arc<Mutex<dyn EventStore>>, activity: &Activity, path: &str) -> Vec<String> {
    let mut seeded = Vec::new();
    for req in &activity.requires {
        if req.optional {
            continue;
        }
        let kind = &req.kind;
        if has_binding(&journal(db), kind) {
            continue;
        }
        append(
            db,
            Event::new(
                EventKind::ActivityCommitted,
                serde_json::json!({
                    "activity_id_at_version": format!("lane-walk-seed-{kind}@1"),
                    "attempt": 1,
                    "completion": true,
                    "item": ITEM,
                    "iteration_path": [],
                    "run_id": "lane-walk-seed",
                    "step_id": "emit",
                    "at": 50,
                    "artifacts": { kind.as_str(): seed_value(kind, path) },
                    "evidence": [],
                }),
            ),
        );
        seeded.push(kind.clone());
    }
    seeded
}

/// One spec per agent step, each under its own schema id so the harness can find its step.
fn step_specs(activity: &Activity) -> BTreeMap<String, ActivitySpec> {
    activity
        .steps
        .iter()
        .filter(|s| s.kind == StepKind::Agent)
        .map(|s| {
            let kind = s.output_type.clone().unwrap_or_else(|| {
                panic!("`{}`'s agent step `{}` declares outputs", activity.id, s.id)
            });
            (
                s.id.clone(),
                ActivitySpec {
                    card_digest: rk_types::digest::sha256(
                        s.role.as_deref().unwrap_or("none").as_bytes(),
                    ),
                    tools: vec!["Read".to_owned()],
                    schema: SchemaId::new(format!("{kind}@{}/{}", activity.id, s.id)),
                    timeout_secs: 900,
                    inputs: vec![("item".to_owned(), serde_json::json!(ITEM))],
                },
            )
        })
        .collect()
}

/// The per-step `FakeHarness`es for one activity.
fn harness_for(activity: &Activity) -> LaneHarness {
    let per_step = step_specs(activity)
        .into_iter()
        .map(|(id, spec)| {
            let kind = spec
                .schema
                .as_str()
                .split('@')
                .next()
                .unwrap_or_default()
                .to_owned();
            let _ = id;
            let script = (0..PER_STEP_SCRIPT)
                .map(|_| Ok(Typed::new(spec.schema.clone(), happy(&kind))))
                .collect();
            (
                spec.schema.as_str().to_owned(),
                FakeHarness::scripted(script),
            )
        })
        .collect();
    LaneHarness { per_step }
}

/// A walk's refusal, whichever channel carries it.
fn refusal_text(report: &rk_kernel::runtime::activity::ActivityReport) -> String {
    format!("{:?} {:?}", report.step_error(), report.outcome())
}

/// The measured verdict of one activity, as a printable row.
#[derive(Debug)]
struct Measured {
    activity: String,
    walks: bool,
    refusal: String,
    gates: Vec<String>,
}

struct Rig<'a> {
    shipped: &'a Shipped,
    lane: &'a Lane,
    db: Arc<Mutex<dyn EventStore>>,
    seeded: Vec<String>,
}

impl Rig<'_> {
    fn routing(&self) -> ToolRouting {
        let values = self
            .shipped
            .verbs
            .values_for("rust")
            .expect("registry.toml declares [toolchain.rust]");
        // *(dated 2026-09-27, `s8d` build, package F2-D2173; D21-73 item 5; R8 = ADR-0179 decision
        // 4, proposed.)* The row inputs, bound as `rk-cli`'s composition root binds them: the shipped
        // `[toolchain.rust]` declaration rows through `row_inputs_for` (`package_binaries` ->
        // `package.binaries`), and `sca_scan`'s two databases at a cargo home. `rk-cli` computes that
        // home from the captured environment (`run::advisory_db_row_inputs`, proved in `rk-cli`); the
        // rig names a fixed one, because the scripted toolchain opens no database.
        ToolRouting::new(self.shipped.verbs.clone(), values)
            .with_sources(Some("rust".to_owned()), Some(self.shipped.redkiln.clone()))
            .with_review_routes(Arc::new(LaneReview))
            .with_row_inputs(self.shipped.verbs.row_inputs_for("rust"))
            .with_row_input(
                "sca_scan",
                "audit_db",
                Json::from("/lane-cargo-home/advisory-db"),
            )
            .with_row_input(
                "sca_scan",
                "deny_db",
                Json::from("/lane-cargo-home/advisory-dbs"),
            )
    }

    /// Walk one activity to its end, answering each gate it opens with its first option.
    fn walk(
        &mut self,
        stage: &str,
        node: &str,
        activity: &Activity,
        item: &Item,
        rigor: rk_kernel::rigor::Rigor,
    ) -> Measured {
        self.seeded
            .extend(seed_requires(&self.db, activity, self.lane.path));
        let aid = activity
            .id_at_version()
            .unwrap_or_else(|| activity.id.clone());
        let specs = step_specs(activity);
        let first_agent = activity
            .steps
            .iter()
            .find(|s| s.kind == StepKind::Agent)
            .and_then(|s| specs.get(&s.id))
            .cloned();
        let ctx = rk_kernel::rigor::with_rigor(item.context(), rigor, &self.shipped.facets);
        let cap = self
            .shipped
            .pack
            .require_attempt_cap()
            .expect("the shipped pack declares its attempt cap");
        let rt = Runtime::new(
            ports_with_toolchain(
                Arc::new(harness_for(activity)) as Arc<dyn Harness>,
                &self.db,
                Arc::new(LaneToolchain {
                    path: self.lane.path,
                }) as Arc<dyn Toolchain>,
            ),
            &format!("lane-walk-{}-{stage}", self.lane.name),
            stage,
        )
        .with_tool_routing(self.routing());
        let mut gates = Vec::new();
        for _ in 0..GATE_ROUNDS {
            let events = journal(&self.db);
            let state = rk_kernel::workflow::fsm::fold(ITEM, stage, &events)
                .unwrap_or_else(|e| panic!("the item's journal folds before `{aid}`: {e:?}"));
            let report = rt
                .run_activity_steps(
                    Activation {
                        item: ITEM,
                        activity_id_at_version: &aid,
                        step_id: node,
                        spec: first_agent.as_ref(),
                        step_specs: Some(&specs),
                        rigor,
                    },
                    Some(&activity.steps),
                    &ctx,
                    GateSite {
                        state: &state,
                        tree_hash: TREE_HASH,
                        deadline: Timestamp::from_unix_seconds(9_000),
                    },
                    AttemptCap::declared(cap.get()),
                    at(100),
                )
                .unwrap_or_else(|e| panic!("the walk of `{aid}` returns a report: {e:?}"));
            if let Some(token) = report.gate_opened() {
                let question = gate_question(&journal(&self.db), token);
                let answer = self
                    .shipped
                    .first_option
                    .get(&question)
                    .unwrap_or_else(|| {
                        panic!("`{aid}` opened the gate `{question}`, which questions.toml does not declare")
                    })
                    .clone();
                append(
                    &self.db,
                    rk_kernel::workflow::gate::gate_answered_event(token, TREE_HASH, &answer),
                );
                gates.push(format!("{question}={answer}"));
                continue;
            }
            let walks = report.completed()
                && report.step_error().is_none()
                && !matches!(report.outcome(), Some(Err(_)));
            return Measured {
                activity: activity.id.clone(),
                walks,
                refusal: if walks {
                    String::new()
                } else {
                    refusal_text(&report)
                },
                gates,
            };
        }
        Measured {
            activity: activity.id.clone(),
            walks: false,
            refusal: format!("LANE-WALK RIG: more than {GATE_ROUNDS} gate rounds: {gates:?}"),
            gates,
        }
    }
}

/// The gate kind of the `gate_opened` row carrying `token`.
fn gate_question(events: &[Event], token: &Digest) -> String {
    events
        .iter()
        .rev()
        .filter(|e| e.kind == EventKind::GateOpened)
        .find(|e| e.payload.get("token").and_then(Json::as_str) == Some(token.as_str()))
        .and_then(|e| e.payload.get("kind").and_then(Json::as_str))
        .unwrap_or_else(|| panic!("no gate_opened row carries the token {}", token.as_str()))
        .to_owned()
}

/// Walk `lane` over the shipped pack and assert its table exactly.
fn walk_lane(lane: &Lane) {
    let shipped = shipped(lane.pack);
    let (q, a) = counters();
    let mut rig = Rig {
        shipped: &shipped,
        lane,
        db: store(&q, &a),
        seeded: Vec::new(),
    };

    // ---- triage: N3's `unknown_facet:change` gate, opened by the kernel's own mint and answered
    // with facet.change's first option; its answer binds `change` for every later stage. ---------
    let order: Vec<String> = shipped.pack.stage_order().to_vec();
    assert_eq!(
        order.first().map(String::as_str),
        Some("triage"),
        "PREMISE: {}'s lane opens at triage",
        lane.pack
    );
    let before = rk_kernel::workflow::fsm::fold(ITEM, "triage", &journal(&rig.db)).expect("folds");
    let opened = rk_kernel::workflow::gate::derived_facet_gate(
        &before,
        "change",
        TREE_HASH,
        Timestamp::from_unix_seconds(9_000),
    )
    .expect("the gate derives")
    .expect("a null `change` at triage opens the unknown_facet:change gate (N3)");
    let token = opened
        .payload
        .get("token")
        .and_then(Json::as_str)
        .expect("the gate row carries its token")
        .to_owned();
    append(&rig.db, opened);
    let answer = shipped
        .first_option
        .get("facet.change")
        .expect("questions.toml declares facet.change (N3)")
        .clone();
    assert_eq!(
        answer, "text",
        "PREMISE: §8d.9 answers the triage gate `text`, facet.change's first option"
    );
    append(
        &rig.db,
        rk_kernel::workflow::gate::gate_answered_event(
            &Digest::from_hex(&token).expect("a 64-hex token"),
            TREE_HASH,
            &answer,
        ),
    );
    let after = rk_kernel::workflow::fsm::fold(ITEM, "triage", &journal(&rig.db)).expect("folds");
    let change = rk_kernel::workflow::gate::answered_facet(&after, "change")
        .expect("the answered gate binds `change`");
    assert_eq!(change, "text");

    // ---- every stage in the pack's lane order ---------------------------------------------------
    // A stage whose SELECTION refuses is a measured row too: it is recorded and the lane goes on, so
    // one stage's refusal never masks a later stage's verdict (§8d.9).
    let mut measured_rows: Vec<(String, Result<Vec<Measured>, String>)> = Vec::new();
    for stage in &order {
        let item = item(&shipped, lane, Some(&change));
        let selection = match select(
            &shipped.pack,
            &shipped.catalog,
            &shipped.artifacts,
            &shipped.facets,
            stage,
            &item,
        ) {
            Ok(selection) => selection,
            Err(e) => {
                measured_rows.push((stage.clone(), Err(e.to_string())));
                continue;
            }
        };
        let plan = match selection.plan() {
            Ok(plan) => plan,
            Err(e) => {
                measured_rows.push((stage.clone(), Err(e.to_string())));
                continue;
            }
        };
        let mut row = Vec::new();
        for node_id in plan.order() {
            let node = selection
                .dag()
                .nodes()
                .iter()
                .find(|n| n.id == node_id)
                .expect("a plan node is a DAG node");
            let activity = shipped
                .catalog
                .activities()
                .iter()
                .find(|a| a.id == node.activity_id)
                .unwrap_or_else(|| panic!("`{}` is a catalog activity", node.activity_id));
            row.push(rig.walk(stage, &node.id, activity, &item, selection.rigor()));
        }
        measured_rows.push((stage.clone(), Ok(row)));
    }

    // ---- the measured table, printed whole BEFORE any assertion, so a red run shows every row ---
    let mut table = String::new();
    for (stage, row) in &measured_rows {
        match row {
            Err(e) => table.push_str(&format!("  {stage:<14} SELECTION REFUSES: {e}\n")),
            Ok(row) => {
                for m in row {
                    table.push_str(&format!(
                        "  {stage:<14} {:<26} {:<8} gates={:?} {}\n",
                        m.activity,
                        if m.walks { "walks" } else { "REFUSES" },
                        m.gates,
                        if m.walks { "" } else { m.refusal.as_str() }
                    ));
                }
            }
        }
    }
    let all = || {
        measured_rows
            .iter()
            .filter_map(|(_, r)| r.as_ref().ok())
            .flat_map(|r| r.iter())
    };
    let walked = all().filter(|m| m.walks).count();
    let refused = all().filter(|m| !m.walks).count();
    println!(
        "lane {} measured (seeded {:?}):\n{table}",
        lane.name, rig.seeded
    );
    println!("LANE_WALK {} walked={walked} refused={refused}", lane.name);

    // ---- the table, exactly: every stage, its selection and each verdict. Every difference is
    // collected before the one assertion, so a red run names all of them at once. ----------------
    let expected: BTreeMap<&str, &Row> = lane.rows.iter().map(|r| (r.stage, r)).collect();
    let refusals: BTreeMap<&str, &str> = lane.selection_refusals.iter().copied().collect();
    let mut wrong: Vec<String> = Vec::new();
    for (stage, row) in &measured_rows {
        let row = match (row, refusals.get(stage.as_str())) {
            (Err(e), Some(needle)) => {
                if !e.contains(needle) {
                    wrong.push(format!(
                        "`{stage}`: the selection refuses, but not naming `{needle}`: {e}"
                    ));
                }
                continue;
            }
            (Err(e), None) => {
                wrong.push(format!(
                    "`{stage}`: the SELECTION refuses, and §8d.9's table says it selects: {e}"
                ));
                continue;
            }
            (Ok(_), Some(needle)) => {
                wrong.push(format!(
                    "`{stage}`: the selection succeeds, and §8d.9's table says it refuses naming \
                     `{needle}`"
                ));
                continue;
            }
            (Ok(row), None) => row,
        };
        // The selected SET: the plan's order within a stage is step 6's, and the table does not
        // claim one.
        let mut got: Vec<&str> = row.iter().map(|m| m.activity.as_str()).collect();
        got.sort_unstable();
        let mut want: Vec<&str> = expected
            .get(stage.as_str())
            .map(|r| r.selected.iter().map(|(a, _)| *a).collect())
            .unwrap_or_default();
        want.sort_unstable();
        if got != want {
            wrong.push(format!(
                "`{stage}` selected {got:?}, and §8d.9's table says {want:?}"
            ));
            continue;
        }
        for m in row {
            let verdict = expected[stage.as_str()]
                .selected
                .iter()
                .find(|(a, _)| *a == m.activity)
                .map(|(_, v)| v.clone())
                .expect("selected above");
            match verdict {
                Verdict::Walks if !m.walks => wrong.push(format!(
                    "`{stage}` `{}` REFUSES, and §8d.9's table says it walks: {}",
                    m.activity, m.refusal
                )),
                Verdict::Refuses(needle) if m.walks => wrong.push(format!(
                    "`{stage}` `{}` WALKS, and §8d.9's table says it refuses naming `{needle}`",
                    m.activity
                )),
                Verdict::Refuses(needle) if !m.refusal.contains(needle) => wrong.push(format!(
                    "`{stage}` `{}` refuses, but not naming `{needle}`: {}",
                    m.activity, m.refusal
                )),
                _ => {}
            }
        }
    }
    for want in lane.rows {
        if !measured_rows
            .iter()
            .any(|(s, r)| s == want.stage && r.as_ref().is_ok_and(|r| !r.is_empty()))
        {
            wrong.push(format!(
                "§8d.9's table has a `{}` row and the lane selected nothing there",
                want.stage
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the {} lane differs from §8d.9's table in {} place(s); correct the TABLE by a dated line, \
         never this test:\n  {}\nMeasured:\n{table}",
        lane.name,
        wrong.len(),
        wrong.join("\n  ")
    );
}

// ---- the two tables --------------------------------------------------------------------------------
//
// *(dated 2026-09-26, `s8d` build, package P8.)* **These are §8d.9's tables AS MEASURED, and they
// differ from the section's first claim** (11 of 15 and 9 of 13 walk). The first run of this module
// asserted the claim and failed on every differing row (§8d.10's red); §8d.9 takes a dated line per
// row and these constants follow it, never the other way round. Each row's reason is in its needle:
//
// - `analysis` (fix): step 4 chooses `bisect` (rank 30) over `differential-debugging` (20) and
//   `root-cause` (10) for `RootCause`, and `bisect`'s `toolchain:bisect` has no registry row, so it
//   refuses by its bare name.
// - `implement` (both lanes) and `closeout` (both lanes): the SELECTION refuses. The fix lane's
//   `implement` is AC-078's cycle refusal with an EMPTY path: `tdd-red-green-refactor` is chosen for
//   both `Changeset` and `ImplReport`, step 5 keeps two `DagNode`s with one id (its `dedup` compares
//   the instance too), and `layers` counts the id once. The story lane's `implement` and both
//   lanes' `closeout` are step 2 evaluating a `require` row keyed on `rigor` against the item's
//   context, which binds no `rigor`: `UnknownField("rigor")`.
// - `confirmation-test`: its composing `emit` reads `${before.outputs.selection}` and `.sha`, and
//   record T's `TestOutcome` carries neither.
// - `sca-scan` and `build-package`: the shipped rows declare inputs the catalog steps do not supply
//   (`audit_db`/`deny_db`; `binaries`), and ADR-0170 names `audit_db`/`deny_db`'s default locations
//   "the resolver's to supply (`s8d`)".
// - `bdd-scenarios`: `policy` reads `${gherkin-lint.outputs}`, and the `lint` port method returns no
//   value, so the step has no output to read.
// - `planning` (story): step 4 chooses `relative-estimation` (rank 30), not `t-shirt-sizing`.
//
// **The `severity` dependency** (§8d.9's dated lines, D21-67 ruling 1): no row here depends on the
// scripted findings list being empty. After §8d.3a (b) the three class-C `policy` checks on these
// lanes read `check.failed == 0 and spec.acceptance_criteria.len() > 0` (`fix-spec`),
// `lint.errors.len() == 0 and check.failed == 0` (`bdd-scenarios`) and
// `design.uncovered_ac.len() == 0` (`equivalence-boundary`), and bind no `findings`. The scripted
// `Findings` value is `{ "items": [] }` only so `adversarial-review`'s `override` gate stays guarded
// off and `verdict_record` has nothing to re-execute.
//
// *(dated 2026-09-27, `s8d` build, package F2-D2173; D21-73; R8 = ADR-0179, proposed.)* **RE-MEASURED
// after D21-73's fixes, and the constants below follow §8d.9's dated line.** Step 4 now chooses by
// lowest rank (`root-cause`, 10, for `RootCause`; `t-shirt-sizing` for the story's estimate), step 5
// builds one node per activity (the fix lane's `implement` selects), step 2 binds `rigor` (both
// lanes' `closeout` and the story's `implement` rows evaluate), and the routing binds the row inputs
// `rk-cli` binds (`sca-scan` and `build-package` walk). The needles above the P8 note that named
// those causes are superseded. What still refuses, and why:
//
// - `tdd-red-green-refactor` (fix `implement`): `green-verify`'s `inputs.selection = "affected"`
//   resolves to the item's COMMITTED `Changeset` (R2 = ADR-0173 decision 9), and `tdd` is that
//   `Changeset`'s producer, so none is committed while it runs. The rig seeds only an activity's
//   mandatory `requires`, and `tdd` requires `Spec`. Never measured before: both lanes' `implement`
//   refused at selection.
// - the story lane's `implement` SELECTION: `docs-update` (rank 5) now wins `Changeset` and declares
//   `requires = ["Spec", "Changeset?"]`, so step 5 draws an edge from its own node to itself and the
//   graph is cyclic. The refusal names `change-summary[rust]`, a node downstream of the loop, because
//   `layers`' path walk starts at the lowest surviving id.
// - `confirmation-test`, `bdd-scenarios`, `smoke-test`, `rolling-deploy`, `debt-logging` and
//   `closure-record`: as the P8 note (owed to `s8g`/`s8h`, `s8g`, `s8g`, `s8f`, `s8f`).

/// §8d.9's fix lane, as measured.
const FIX: Lane = Lane {
    name: "fix",
    pack: "bug-standard.toml",
    kind: "bug",
    facets: &[
        ("cos", FacetIs::Str("standard")),
        ("has_repro", FacetIs::Bool(true)),
        ("criticality", FacetIs::Ordinal("low")),
        ("blast_radius", FacetIs::Ordinal("low")),
        ("size", FacetIs::Ordinal("low")),
    ],
    path: "crates/rk-kernel/src/predicate/eval.rs",
    rows: &[
        Row {
            stage: "analysis",
            selected: &[
                ("minimal-repro", Verdict::Walks),
                ("root-cause", Verdict::Walks),
            ],
        },
        Row {
            stage: "specification",
            selected: &[("fix-spec", Verdict::Walks)],
        },
        Row {
            stage: "implement",
            selected: &[
                (
                    "tdd-red-green-refactor",
                    Verdict::Refuses("no committed `emit` binding of `Changeset`"),
                ),
                ("change-summary", Verdict::Walks),
            ],
        },
        Row {
            stage: "verify",
            selected: &[
                ("equivalence-boundary", Verdict::Walks),
                (
                    "confirmation-test",
                    Verdict::Refuses("`before`'s output has no field `.selection`"),
                ),
                (
                    "smoke-test",
                    Verdict::Refuses("`checklist:smoke` is mandatory"),
                ),
                ("sca-scan", Verdict::Walks),
            ],
        },
        Row {
            stage: "review",
            selected: &[("adversarial-review", Verdict::Walks)],
        },
        Row {
            stage: "release",
            selected: &[
                ("build-package", Verdict::Walks),
                ("rolling-deploy", Verdict::Refuses("capability: \"deploy\"")),
            ],
        },
        Row {
            stage: "closeout",
            selected: &[
                (
                    "debt-logging",
                    Verdict::Refuses("`item_create` takes the `store` route"),
                ),
                ("lightweight-retro", Verdict::Walks),
                (
                    "closure-record",
                    Verdict::Refuses("`feed.rust.backlog` is mandatory"),
                ),
            ],
        },
    ],
    selection_refusals: &[],
};

/// §8d.9's story lane, as measured.
const STORY: Lane = Lane {
    name: "story",
    pack: "story.toml",
    kind: "story",
    facets: &[
        ("cos", FacetIs::Str("standard")),
        ("criticality", FacetIs::Ordinal("low")),
        ("blast_radius", FacetIs::Ordinal("low")),
        ("size", FacetIs::Ordinal("low")),
        ("novelty", FacetIs::Ordinal("low")),
        ("external_dep", FacetIs::Bool(false)),
    ],
    path: "spikes/p9/packs/spike.toml",
    rows: &[
        Row {
            stage: "specification",
            selected: &[(
                "bdd-scenarios",
                Verdict::Refuses("the step `gherkin-lint` has not run"),
            )],
        },
        Row {
            stage: "planning",
            selected: &[("t-shirt-sizing", Verdict::Walks)],
        },
        Row {
            stage: "verify",
            selected: &[
                ("equivalence-boundary", Verdict::Walks),
                (
                    "smoke-test",
                    Verdict::Refuses("`checklist:smoke` is mandatory"),
                ),
                ("sca-scan", Verdict::Walks),
            ],
        },
        Row {
            stage: "review",
            selected: &[("adversarial-review", Verdict::Walks)],
        },
        Row {
            stage: "release",
            selected: &[
                ("build-package", Verdict::Walks),
                ("rolling-deploy", Verdict::Refuses("capability: \"deploy\"")),
            ],
        },
        Row {
            stage: "closeout",
            selected: &[
                (
                    "debt-logging",
                    Verdict::Refuses("`item_create` takes the `store` route"),
                ),
                ("lightweight-retro", Verdict::Walks),
                (
                    "closure-record",
                    Verdict::Refuses("`feed.rust.backlog` is mandatory"),
                ),
            ],
        },
    ],
    selection_refusals: &[("implement", "the assembled DAG is not acyclic")],
};

#[test]
fn the_fix_lane_walk_table_holds_for_a_rust_text_bug() {
    walk_lane(&FIX);
}

#[test]
fn the_story_lane_walk_table_holds_for_a_spike_toml_story() {
    walk_lane(&STORY);
}
