//! **The four demands restored** (`s8g` §8g.6; R4 = ADR-0194, proposed; D21-42 6a; D21-43 N25;
//! D21-46 R6). *(Added 2026-09-29, `s8g` build, package S8G-P7-DEMANDS.)*
//!
//! `SmokeResult`, `ConfirmationResult`, `ScanReport` and `BuildArtifact` are each produced by a walk
//! of the SHIPPED catalog activity (`smoke-test`, `confirmation-test`, `sca-scan`,
//! `build-package`) under `s8d`'s resolver, and each writes ONE `artifact.write` through
//! `rk-store`'s writer (ADR-0189). The rig is the production composition wherever that is
//! observable in-process:
//!
//! - **The routing is `rk-cli`'s own** — [`rk_cli::verbs::run::tool_routing`] over THIS repository's
//!   `.redkiln/registry.toml` (so `[checklist.smoke]`, the `smoke` row and `[toolchain.rust]`'s
//!   `package_binaries` are the shipped ones), with `sca_scan`'s two database locations bound as
//!   `rk-cli` binds them.
//! - **The activities are read off disk** from `.redkiln/catalog/`, never a fixture copy.
//! - **The writer is `rk-store`'s** [`rk_store::render::ArtifactWriter`], journalling into the same
//!   store the walk reads, over a temp tree standing for the run-owned worktree.
//! - **The toolchain is a recording double** ([`Demands`]): the three verb handlers and their
//!   semantics are `s8c`'s record V (ADR-0170), which this stage cites and does not re-test. The
//!   double answers typed `rk_types::verb_output` values and records every call it is handed.
//!
//! 🛑 **What the double hides, named.** The `checklist:smoke` input reaches the port as the
//! resolver's envelope (`{ key, path, content, digest }`, ADR-0173), while record V's Rust handler
//! reads `checklist` as a LIST of items. The double parses the envelope's `content` itself; the
//! production handler does not, so a real `smoke` call over the shipped checklist is refused
//! `input-malformed` until the bridge is ruled (package S8G-P7-DEMANDS's blocker).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Activity, Catalog};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::ActivityReport;
use rk_kernel::runtime::step::{Activation, GateSite};
use rk_kernel::select::Item;
use rk_kernel::workflow::breaker::JournaledNow;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_ports::docs_site::DocsSite;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::fakes::{BoardTracker, FakeHarness, RecordingDocs, VecStore};
use rk_ports::harness::Harness;
use rk_ports::root::Ports;
use rk_ports::toolchain::{
    ChangeSummary, Classification, Selection, StepBound, TestOutcome, TestRunner, Toolchain,
    VerbCall, VerbOutput,
};
use rk_ports::tracker::Tracker;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::verb_output::{
    Advisory, ArtifactBinary, BuildArtifact, BuildReport, BuildStatus, ScanReport, Severity,
    SmokeResult, SmokeRow, SmokeStatus,
};

type Json = serde_json::Value;

const ITEM: &str = "demands-1";
const RUST: &str = "rk-adapter-toolchain-rust";
/// The fix commit the seeded `Changeset` names: 40 hex, the shape a real one has.
const FIX_SHA: &str = "f1e2d3c4b5a6978877665544332211009988aabb";
/// The tree hash a walk's gates open over (the kernel never reads a tree).
const TREE_HASH: &str = "0123456789abcdef0123456789abcdef01234567";
/// The repro command the seeded `ReproCase` carries: `selection_of` reads the tokens after ` -- `.
const REPRO: &str = "cargo test -p rk-kernel -- repro_crash";
/// The declared release set, ADR-0179 decision 4's `package_binaries` row (cited, never written here).
const DECLARED_BINARIES: [&str; 2] = ["redkiln", "redkiln-migrate"];

// ------------------------------------------------------------------------------- the shipped tree

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// The shipped activity `id`, read off `.redkiln/catalog/`.
fn shipped_activity(id: &str) -> Activity {
    let catalog = Catalog::from_dir(&root().join(".redkiln").join("catalog"))
        .unwrap_or_else(|e| panic!("the shipped catalog loads: {e:?}"));
    catalog
        .activities()
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .unwrap_or_else(|| panic!("the shipped catalog carries `{id}`"))
}

// ------------------------------------------------------------------------------- the double

/// What the double answers.
#[derive(Debug, Clone)]
struct Script {
    /// How many smoke rows fail (the first `n` items).
    smoke_failed: u32,
    /// The repro's test run: `true` passes at the fix sha.
    test_passes: bool,
    /// The scan's counts: `(critical, high, unrated)`.
    scan: (u32, u32, u32),
    build: BuildStatus,
}

impl Script {
    const fn happy() -> Self {
        Self {
            smoke_failed: 0,
            test_passes: true,
            scan: (0, 0, 0),
            build: BuildStatus::Passing,
        }
    }
}

/// **A recording `Toolchain`**: every call by name with the inputs it was handed.
struct Demands {
    script: Script,
    calls: Mutex<Vec<(String, Json)>>,
}

impl Demands {
    const fn new(script: Script) -> Self {
        Self {
            script,
            calls: Mutex::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<(String, Json)> {
        self.calls.lock().expect("unpoisoned").clone()
    }

    fn note(&self, name: &str, inputs: Json) {
        self.calls
            .lock()
            .expect("unpoisoned")
            .push((name.to_owned(), inputs));
    }
}

/// One smoke item in record V's form (ADR-0170 decision 4; the handler's `SmokeItem`, mirrored so
/// an item the handler would refuse for an unknown key is refused here too).
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SmokeItem {
    id: String,
    argv: Vec<String>,
    cwd: String,
    expect_exit: i32,
    stdout_contains: String,
}

/// The `[[item]]` rows of a `checklist:` envelope's `content`, through the kernel's own TOML subset.
fn items_of(envelope: &Json) -> Result<Vec<SmokeItem>, String> {
    let content = envelope
        .get("content")
        .and_then(Json::as_str)
        .ok_or_else(|| format!("the checklist input carries no `content`: {envelope}"))?;
    let doc = rk_kernel::toml_subset::parse(content).map_err(|e| format!("{e:?}"))?;
    let items = doc
        .get("item")
        .and_then(rk_kernel::toml_subset::Value::as_array)
        .ok_or_else(|| "the checklist declares no `[[item]]`".to_owned())?;
    items
        .iter()
        .map(|item| serde_json::from_value(toml_json(item)).map_err(|e| e.to_string()))
        .collect()
}

fn toml_json(v: &rk_kernel::toml_subset::Value) -> Json {
    use rk_kernel::toml_subset::Value as T;
    match v {
        T::Str(s) => Json::from(s.clone()),
        T::Int(i) => Json::from(*i),
        T::Float(f) => Json::from(*f),
        T::Bool(b) => Json::from(*b),
        T::Array(items) => Json::Array(items.iter().map(toml_json).collect()),
        T::Table(t) => Json::Object(t.iter().map(|(k, v)| (k.clone(), toml_json(v))).collect()),
    }
}

fn verb_output<T: serde::Serialize>(kind: &str, value: &T) -> Result<VerbOutput, StepError> {
    Ok(VerbOutput {
        kind: kind.to_owned(),
        value: serde_json::to_value(value).expect("a verb output serialises"),
    })
}

impl Toolchain for Demands {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }

    fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
        self.note("build", Json::Null);
        Ok(BuildReport {
            argv: vec!["cargo".to_owned(), "build".to_owned()],
            toolchain_version: "rustc 1.0.0-double".to_owned(),
            wall_secs: 0,
            errors: u32::from(self.script.build == BuildStatus::Failing),
            status: self.script.build,
        })
    }

    fn test(&self, selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        self.note("test", Json::from(format!("{selection:?}")));
        let n = match selection {
            Selection::Filter(names) => u32::try_from(names.len()).unwrap_or(1),
            _ => 1,
        };
        let (passed, failed) = if self.script.test_passes {
            (n, 0)
        } else {
            (0, n)
        };
        TestOutcome::new(
            rk_types::verify::VerifyReport {
                basis: rk_types::verify::Basis::WholeSuite,
                argv: vec!["cargo".to_owned(), "test".to_owned()],
                selected_test_count: n,
                total_test_count: n,
                total_test_count_source: rk_types::verify::TotalTestCountSource::OwnExecutedCount,
                wall_secs: 0,
                toolchain_version: "rustc 1.0.0-double".to_owned(),
                suite_sha: FIX_SHA.to_owned(),
                basis_reason: Some(rk_types::verify::BasisReason::ProbeUnverified),
            },
            passed,
            failed,
            TestRunner::named("double", "0"),
        )
    }

    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        Err(StepError::Unsupported {
            capability: "lint".to_owned(),
        })
    }

    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        Ok(Classification::Text)
    }

    fn change_summary(&self, _range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        Err(StepError::Unsupported {
            capability: "change_summary".to_owned(),
        })
    }

    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        let inputs = Json::Object(
            call.inputs
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        );
        self.note(call.verb, inputs);
        let refuse = |why: String| StepError::Unsupported { capability: why };
        match call.verb {
            "smoke" => {
                let envelope = call
                    .inputs
                    .get("checklist")
                    .ok_or_else(|| refuse("smoke: no `checklist` input".to_owned()))?;
                let items = items_of(envelope).map_err(|e| refuse(format!("smoke: {e}")))?;
                let rows: Vec<SmokeRow> = items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let passed =
                            u32::try_from(i).unwrap_or(u32::MAX) >= self.script.smoke_failed;
                        SmokeRow {
                            id: item.id.clone(),
                            argv: item.argv.clone(),
                            cwd: "<the handler's scratch>".to_owned(),
                            expect_exit: item.expect_exit,
                            exit: if passed { item.expect_exit } else { 101 },
                            predicate: format!("stdout contains {:?}", item.stdout_contains),
                            predicate_held: passed,
                            passed,
                        }
                    })
                    .collect();
                let passed =
                    u32::try_from(rows.iter().filter(|r| r.passed).count()).unwrap_or(u32::MAX);
                let failed = u32::try_from(rows.len())
                    .unwrap_or(u32::MAX)
                    .saturating_sub(passed);
                let status = if failed > 0 {
                    SmokeStatus::Failing
                } else if passed > 0 {
                    SmokeStatus::Passing
                } else {
                    SmokeStatus::Empty
                };
                verb_output(
                    "SmokeResult",
                    &SmokeResult {
                        rows,
                        passed,
                        failed,
                        status,
                    },
                )
            }
            "sca_scan" => {
                let (critical, high, unrated) = self.script.scan;
                let mut advisories = Vec::new();
                for (n, severity) in [
                    (critical, Some(Severity::Critical)),
                    (high, Some(Severity::High)),
                    (unrated, None),
                ] {
                    for i in 0..n {
                        advisories.push(Advisory {
                            id: format!("RUSTSEC-2099-{i:04}"),
                            package: "demo".to_owned(),
                            version: "1.0.0".to_owned(),
                            severity,
                        });
                    }
                }
                verb_output(
                    "ScanReport",
                    &ScanReport {
                        critical,
                        high,
                        medium: 0,
                        low: 0,
                        unrated,
                        advisories,
                        tools: Vec::new(),
                    },
                )
            }
            "package" => {
                let names: Vec<String> = call
                    .inputs
                    .get("binaries")
                    .and_then(Json::as_array)
                    .ok_or_else(|| refuse("package: no `binaries` input".to_owned()))?
                    .iter()
                    .map(|b| b.as_str().unwrap_or_default().to_owned())
                    .collect();
                let sha = call
                    .inputs
                    .get("sha")
                    .and_then(Json::as_str)
                    .unwrap_or_default()
                    .to_owned();
                verb_output(
                    "BuildArtifact",
                    &BuildArtifact {
                        profile: "release".to_owned(),
                        binaries: names
                            .iter()
                            .map(|name| ArtifactBinary {
                                name: name.clone(),
                                path: format!("release/{name}"),
                                sha256: rk_types::digest::sha256(name.as_bytes())
                                    .as_str()
                                    .to_owned(),
                            })
                            .collect(),
                        argv: vec![
                            "cargo".to_owned(),
                            "build".to_owned(),
                            "--release".to_owned(),
                        ],
                        toolchain_version: "rustc 1.0.0-double".to_owned(),
                        sha: sha.clone(),
                        head: sha,
                    },
                )
            }
            other => Err(refuse(other.to_owned())),
        }
    }
}

// ------------------------------------------------------------------------------- the walk

/// One walk's product: the report, the item's journal after it, and what the double was handed.
struct Walked {
    report: ActivityReport,
    events: Vec<Event>,
    calls: Vec<(String, Json)>,
    tree: tempfile::TempDir,
}

impl Walked {
    /// Whether the activation completed with no refusal on either channel.
    const fn walks(&self) -> bool {
        self.report.completed()
            && self.report.step_error().is_none()
            && !matches!(self.report.outcome(), Some(Err(_)))
    }

    fn refusal(&self) -> String {
        format!("{:?} {:?}", self.report.step_error(), self.report.outcome())
    }

    /// The `artifact.write` rows of the walk.
    fn writes(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.kind == EventKind::ArtifactWrite)
            .collect()
    }

    /// The kind each `artifact.write` names.
    fn written_kinds(&self) -> Vec<String> {
        self.writes()
            .iter()
            .filter_map(|e| e.payload.get("kind").and_then(Json::as_str))
            .map(str::to_owned)
            .collect()
    }

    /// The value the writer wrote for `kind` (its sidecar under the managed directory).
    fn written(&self, kind: &str) -> Json {
        let sidecar = self
            .tree
            .path()
            .join(".redkiln/artifacts")
            .join(ITEM)
            .join(format!("{kind}.json"));
        serde_json::from_slice(
            &std::fs::read(&sidecar)
                .unwrap_or_else(|e| panic!("{} is written: {e}", sidecar.display())),
        )
        .expect("the sidecar is JSON")
    }

    /// The question of the gate the walk opened, if it opened one.
    fn gate_question(&self) -> Option<String> {
        let token = self.report.gate_opened()?;
        self.events
            .iter()
            .filter(|e| e.kind == EventKind::GateOpened)
            .find(|e| e.payload.get("token").and_then(Json::as_str) == Some(token.as_str()))
            .and_then(|e| e.payload.get("kind").and_then(Json::as_str))
            .map(str::to_owned)
    }

    /// The journalled `check` decision of the step `step`, if the walk took one.
    fn check_decision(&self, step: &str) -> Option<Json> {
        self.events
            .iter()
            .filter(|e| e.kind == EventKind::ActivityCommitted)
            .filter(|e| e.payload.get("step_id").and_then(Json::as_str) == Some(step))
            .filter_map(|e| e.payload.get("decision"))
            .find(|d| d.get("kind").and_then(Json::as_str) == Some("check"))
            .and_then(|d| d.get("holds").cloned())
    }

    /// The inputs the double's `verb` call was handed (the last such call).
    fn call(&self, verb: &str) -> Option<&Json> {
        self.calls
            .iter()
            .rev()
            .find(|(name, _)| name == verb)
            .map(|(_, inputs)| inputs)
    }
}

fn append(db: &Arc<Mutex<dyn EventStore>>, event: Event) {
    db.lock()
        .expect("unpoisoned")
        .append(event)
        .expect("the in-memory store never refuses an append");
}

fn journal(db: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    db.lock()
        .expect("unpoisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("the in-memory store never refuses a query")
}

/// A seeded upstream binding of `kind`: an `emit` completion row, where `artifact:<Kind>` reads from.
fn seed(db: &Arc<Mutex<dyn EventStore>>, kind: &str) {
    let value = match kind {
        "ReproCase" => serde_json::json!({ "command": REPRO, "steps": ["run it"] }),
        "Changeset" => serde_json::json!({
            "sha": FIX_SHA, "diff": "", "files": ["crates/rk-kernel/src/predicate/eval.rs"],
        }),
        "ReviewVerdict" => serde_json::json!({ "verdict": "approve", "blocking": [] }),
        other => serde_json::json!({ "seeded": other }),
    };
    append(
        db,
        Event::new(
            EventKind::ActivityCommitted,
            serde_json::json!({
                "activity_id_at_version": format!("demands-seed-{kind}@1"),
                "attempt": 1,
                "completion": true,
                "item": ITEM,
                "iteration_path": [],
                "run_id": "demands-seed",
                "step_id": "emit",
                "at": 50,
                "artifacts": { kind: value },
                "evidence": [],
            }),
        ),
    );
}

/// **Walk the shipped `activity` once** over the recording double, through `rk-cli`'s routing and
/// `rk-store`'s writer, with every mandatory `requires` seeded.
fn walk(activity_id: &str, script: Script) -> Walked {
    let activity = shipped_activity(activity_id);
    let stage = activity.stage.clone();
    let db: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    for req in activity.requires.iter().filter(|r| !r.optional) {
        seed(&db, &req.kind);
    }
    let tree = tempfile::tempdir().expect("a temp tree stands for the run-owned worktree");
    let run_id = format!("{stage}-0123456789abcdef");
    let (a, r) = (Arc::clone(&db), Arc::clone(&db));
    let writer = rk_store::render::ArtifactWriter::new(
        tree.path(),
        rk_store::render::StageRun {
            item: ITEM.to_owned(),
            stage: stage.clone(),
            run: run_id.clone(),
        },
        activity
            .provides
            .iter()
            .map(|k| (k.clone(), k.clone()))
            .collect(),
        rk_store::render::Journal::new(
            move |e| {
                a.lock()
                    .map_err(|_| "poisoned".to_owned())?
                    .append(e)
                    .map_err(|e| format!("{e:?}"))
            },
            move || {
                r.lock()
                    .map_err(|_| "poisoned".to_owned())?
                    .query(Query::ByItem(ITEM.to_owned()))
                    .map_err(|e| format!("{e:?}"))
            },
        ),
    );
    let routing = rk_cli::verbs::run::tool_routing(&root(), Some(RUST))
        .unwrap_or_else(|e| panic!("the shipped registry loads through rk-cli's routing: {e}"))
        .with_row_input(
            "sca_scan",
            "audit_db",
            Json::from("/demands-cargo-home/advisory-db"),
        )
        .with_row_input(
            "sca_scan",
            "deny_db",
            Json::from("/demands-cargo-home/advisory-dbs"),
        );
    let double = Arc::new(Demands::new(script));
    let ports = Ports {
        store: Arc::clone(&db),
        harness: Arc::new(FakeHarness::scripted(Vec::new())) as Arc<dyn Harness>,
        tracker: Arc::new(BoardTracker::default()) as Arc<dyn Tracker>,
        docs: Arc::new(RecordingDocs::default()) as Arc<dyn DocsSite>,
        toolchain: Arc::clone(&double) as Arc<dyn Toolchain>,
    };
    let rt = Runtime::new(ports, &run_id, &stage)
        .with_tool_routing(routing)
        .with_write_seam(Arc::new(writer));
    let aid = activity
        .id_at_version()
        .unwrap_or_else(|| activity.id.clone());
    let state = rk_kernel::workflow::fsm::fold(ITEM, &stage, &journal(&db))
        .unwrap_or_else(|e| panic!("the seeded journal folds: {e:?}"));
    let no_specs = BTreeMap::new();
    let report = rt
        .run_activity_steps(
            Activation {
                item: ITEM,
                activity_id_at_version: &aid,
                step_id: activity_id,
                spec: None,
                step_specs: Some(&no_specs),
                rigor: rk_kernel::rigor::Rigor::Optional,
            },
            Some(&activity.steps),
            &Item::new(ITEM, "bug").context(),
            GateSite {
                state: &state,
                tree_hash: TREE_HASH,
                deadline: Timestamp::from_unix_seconds(9_000),
            },
            AttemptCap::declared(8),
            JournaledNow::from_journal(Timestamp::from_unix_seconds(100)),
        )
        .unwrap_or_else(|e| panic!("the walk of `{aid}` returns a report: {e:?}"));
    Walked {
        report,
        events: journal(&db),
        calls: double.calls(),
        tree,
    }
}

// ------------------------------------------------------------------------------- SmokeResult

/// **`SmokeResult` is produced through the registry verb** (§8g.6; R4 decisions 1 and 2; M26).
///
/// The shipped `smoke-test` walks: `build`, then `smoke` reached through the registry's `toolchain`
/// row with `checklist:smoke` resolved through `[checklist.smoke]`'s `path` to the SHIPPED
/// `.redkiln/checklists/smoke.toml` (never `unauthored`), then one `SmokeResult` `artifact.write`.
/// Every item is in record V's form, runs a binary the call's build produces (the declared release
/// set, never an installed `redkiln`), dispatches nothing (no `run`) and names `cwd = "scratch"`,
/// the handler's own directory under `<git-common-dir>/redkiln/tmp` (ADR-0170 decision 4), so no
/// item runs inside `.redkiln/worktrees/<item>` (D21-44).
#[test]
fn smoke_result_is_produced_through_the_registry_verb() {
    let w = walk("smoke-test", Script::happy());
    assert!(
        w.walks(),
        "the shipped smoke-test does not walk: {}",
        w.refusal()
    );
    let names: Vec<&str> = w.calls.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["build", "smoke"], "the port calls, in step order");
    let envelope = w
        .call("smoke")
        .expect("the smoke verb was reached")
        .get("checklist")
        .expect("the smoke call carries its `checklist` input");
    let shipped = std::fs::read(root().join(".redkiln/checklists/smoke.toml"))
        .expect("the shipped smoke checklist");
    assert_eq!(
        envelope.get("path").and_then(Json::as_str),
        Some("checklists/smoke.toml"),
        "`checklist:smoke` resolves through [checklist.smoke]'s path: {envelope}"
    );
    assert_eq!(
        envelope.get("digest").and_then(Json::as_str),
        Some(rk_types::digest::sha256(&shipped).as_str()),
        "the input is the SHIPPED checklist's bytes"
    );
    let items = items_of(envelope).expect("the shipped checklist is in record V's item form");
    assert!(!items.is_empty(), "the checklist holds items");
    let mut ids = std::collections::BTreeSet::new();
    for item in &items {
        assert!(
            ids.insert(item.id.clone()),
            "item ids are unique: {}",
            item.id
        );
        assert_eq!(
            item.cwd, "scratch",
            "`{}` runs outside the handler's scratch: only `scratch` is admitted (record V \
             decision 4), and it is never inside .redkiln/worktrees/<item> (D21-44)",
            item.id
        );
        assert!(
            DECLARED_BINARIES.contains(&item.argv[0].as_str()),
            "`{}` runs `{}`, not a binary of the declared release set",
            item.id,
            item.argv[0]
        );
        assert!(
            !item.argv.iter().any(|a| a == "run" || a.contains("://")),
            "`{}` dispatches or reaches the network: {:?}",
            item.id,
            item.argv
        );
    }
    assert_eq!(w.written_kinds(), ["SmokeResult"], "one SmokeResult write");
    let result = w.written("SmokeResult");
    assert_eq!(result.get("status").and_then(Json::as_str), Some("passing"));
    assert_eq!(
        result.get("rows").and_then(Json::as_array).map(Vec::len),
        Some(items.len()),
        "one row per checklist item"
    );
}

/// **A failing `SmokeResult` fails the stage, on both arms** (D21-46 R6; R4 decision 1; M15, M23).
///
/// Passing: one `SmokeResult` `artifact.write`, the `passing` check journalled `true`. Failing
/// (`failed = 1`): the activity refuses by name at `passing`, no `SmokeResult` is written, and the
/// activation does not complete, so the verify stage's baseline `SmokeResult` demand stays unmet.
#[test]
fn a_failing_smoke_result_fails_the_stage() {
    // Both arms are walked and each reports its own failure, so a mutant is attributed to the arm
    // that sees it (M23 the passing arm's, M15 the failing arm's).
    let mut wrong = Vec::new();

    let pass = walk("smoke-test", Script::happy());
    if !pass.walks() {
        wrong.push(format!("PASSING arm refuses: {}", pass.refusal()));
    }
    if pass.written_kinds() != ["SmokeResult"] {
        wrong.push(format!(
            "PASSING arm wrote {:?}, not one SmokeResult",
            pass.written_kinds()
        ));
    }
    if pass.check_decision("passing") != Some(Json::Bool(true)) {
        wrong.push(format!(
            "PASSING arm: the `passing` check is not journalled holding: {:?}",
            pass.check_decision("passing")
        ));
    }

    let fail = walk(
        "smoke-test",
        Script {
            smoke_failed: 1,
            ..Script::happy()
        },
    );
    if !fail.writes().is_empty() {
        wrong.push(format!(
            "FAILING arm: a failing SmokeResult was written as the activity's product: {:?}",
            fail.written_kinds()
        ));
    }
    if fail.report.completed() {
        wrong.push(
            "FAILING arm completed its activation, so the verify stage would complete on it"
                .to_owned(),
        );
    }
    let refusal = fail.refusal();
    if !refusal.contains("`passing`") {
        wrong.push(format!(
            "FAILING arm is not refused naming the `passing` check: {refusal}"
        ));
    }
    assert!(
        wrong.is_empty(),
        "smoke-test does not gate on its own result (D21-46 R6):\n  {}",
        wrong.join("\n  ")
    );
}

// ------------------------------------------------------------------------------- ConfirmationResult

/// **`ConfirmationResult` passes at the fix sha** (§8g.6; R4 decision 3, as its dated note settles
/// the emit fields; D21-73 item 4).
///
/// `before` runs the `ReproCase`'s command through `selection_of` (ADR-0162), the passing result
/// holds `fixed`, `not-fixed` is guarded off, and the composing `emit` writes one
/// `ConfirmationResult` whose `command` and `sha` are the `ReproCase`'s command and the fix
/// `Changeset`'s sha.
#[test]
fn confirmation_result_passes_at_the_fix_sha() {
    let w = walk("confirmation-test", Script::happy());
    assert!(w.walks(), "confirmation-test refuses: {}", w.refusal());
    assert_eq!(
        w.call("test"),
        Some(&Json::from(format!(
            "{:?}",
            Selection::Filter(vec!["repro_crash".to_owned()])
        ))),
        "the repro's `--` filter reached the port through selection_of"
    );
    assert_eq!(w.gate_question(), None, "a passing result opens no gate");
    assert_eq!(w.written_kinds(), ["ConfirmationResult"]);
    assert_eq!(
        w.written("ConfirmationResult"),
        serde_json::json!({
            "status": "passing",
            "passes_after": 1,
            "command": REPRO,
            "sha": FIX_SHA,
        }),
        "the ConfirmationResult names the repro's command and the fix sha"
    );
}

/// **`confirmation.not-fixed` opens on a result that does not pass, and nothing is emitted**
/// (§8g.6). The question is one of `s8d`'s, loaded from `.redkiln/processes/`.
#[test]
fn confirmation_not_fixed_opens_its_gate() {
    let registry = rk_cli::processes::load(&root()).expect("the shipped processes registry loads");
    assert!(
        registry.question("confirmation.not-fixed").is_some(),
        "PREMISE: `confirmation.not-fixed` is declared under .redkiln/processes/"
    );
    let w = walk(
        "confirmation-test",
        Script {
            test_passes: false,
            ..Script::happy()
        },
    );
    assert_eq!(
        w.gate_question().as_deref(),
        Some("confirmation.not-fixed"),
        "a failing repro at the fix sha opens confirmation.not-fixed: {}",
        w.refusal()
    );
    assert_eq!(
        w.check_decision("fixed"),
        Some(Json::Bool(false)),
        "the `fixed` check reads the failing result"
    );
    assert!(
        w.writes().is_empty(),
        "a not-fixed result emitted: {:?}",
        w.written_kinds()
    );
    assert!(
        !w.report.completed(),
        "a gated activation does not complete"
    );
}

// ------------------------------------------------------------------------------- ScanReport

/// A scan walk's gate question and its `clean` decision.
fn scan(critical: u32, high: u32, unrated: u32) -> Walked {
    walk(
        "sca-scan",
        Script {
            scan: (critical, high, unrated),
            ..Script::happy()
        },
    )
}

/// **A high finding opens `scan.accept-findings`** (§8g.6; D21-43 N25).
#[test]
fn scan_high_finding_opens_scan_accept_findings() {
    let w = scan(0, 1, 0);
    assert_eq!(
        w.gate_question().as_deref(),
        Some("scan.accept-findings"),
        "a high finding did not open the gate: {}",
        w.refusal()
    );
    assert_eq!(w.check_decision("clean"), Some(Json::Bool(false)));
    assert!(
        w.writes().is_empty(),
        "a gated scan emitted before the answer"
    );
}

/// **An unrated advisory opens `scan.accept-findings`, never counted as zero** (§8g.6; `s8b`'s
/// `sca-scan@2`; M7): with no critical and no high finding, `unrated = 1` alone makes `clean`
/// false and opens the gate.
#[test]
fn scan_unrated_advisory_opens_scan_accept_findings() {
    let w = scan(0, 0, 1);
    assert_eq!(
        w.check_decision("clean"),
        Some(Json::Bool(false)),
        "the `clean` check passed with an unrated advisory (sca-scan's v1 expression)"
    );
    assert_eq!(
        w.gate_question().as_deref(),
        Some("scan.accept-findings"),
        "an unrated advisory did not open the gate: {}",
        w.refusal()
    );
    assert!(
        w.writes().is_empty(),
        "a gated scan emitted before the answer"
    );
}

/// **A clean scan emits without a gate** (§8g.6).
#[test]
fn scan_clean_emits_without_a_gate() {
    let w = scan(0, 0, 0);
    assert!(w.walks(), "a clean scan refuses: {}", w.refusal());
    assert_eq!(w.gate_question(), None, "a clean scan opened a gate");
    assert_eq!(w.check_decision("clean"), Some(Json::Bool(true)));
    assert_eq!(w.written_kinds(), ["ScanReport"]);
    assert_eq!(w.written("ScanReport").get("unrated"), Some(&Json::from(0)));
}

// ------------------------------------------------------------------------------- BuildArtifact

/// **`BuildArtifact` carries the release digests of EXACTLY the declared set** (§8g.6; R4 decision
/// 5; ADR-0179 decision 4's `package_binaries`, cited; M9).
///
/// `build`, the `built` check over a passing report, then `package` handed the shipped
/// `[toolchain.rust]` `package_binaries` as its `binaries` and the reviewed `Changeset`'s sha; one
/// `BuildArtifact` write whose binaries are `redkiln` and `redkiln-migrate` and no fixture binary.
#[test]
fn build_artifact_carries_release_digests() {
    let w = walk("build-package", Script::happy());
    assert!(w.walks(), "build-package refuses: {}", w.refusal());
    let package = w.call("package").expect("the package verb was reached");
    assert_eq!(
        package.get("binaries"),
        Some(&serde_json::json!(DECLARED_BINARIES)),
        "`package` was handed a set other than the declared release set"
    );
    assert_eq!(
        package.get("sha").and_then(Json::as_str),
        Some(FIX_SHA),
        "`package` builds the reviewed Changeset's sha"
    );
    assert_eq!(w.check_decision("built"), Some(Json::Bool(true)));
    assert_eq!(w.written_kinds(), ["BuildArtifact"]);
    let artifact = w.written("BuildArtifact");
    let names: Vec<&str> = artifact
        .get("binaries")
        .and_then(Json::as_array)
        .expect("the artifact lists its binaries")
        .iter()
        .filter_map(|b| b.get("name").and_then(Json::as_str))
        .collect();
    assert_eq!(
        names, DECLARED_BINARIES,
        "the exact declared set, and no fixture binary"
    );
    for b in artifact["binaries"].as_array().expect("binaries") {
        let digest = b.get("sha256").and_then(Json::as_str).unwrap_or_default();
        assert!(
            digest.len() == 64 && digest.bytes().all(|c| c.is_ascii_hexdigit()),
            "each binary carries a sha256 digest: {b}"
        );
    }
}

// ------------------------------------------------------------------------------- one write each

/// **Every restored demand writes exactly one `artifact.write`** (ADR-0116's forbids; §8g.6; R4
/// decision 6): the four happy walks, each one row naming its own kind, at its own stage.
#[test]
fn every_restored_demand_writes_one_artifact_write() {
    let mut failures = Vec::new();
    for (activity, kind, stage) in [
        ("smoke-test", "SmokeResult", "verify"),
        ("confirmation-test", "ConfirmationResult", "verify"),
        ("sca-scan", "ScanReport", "verify"),
        ("build-package", "BuildArtifact", "release"),
    ] {
        let w = walk(activity, Script::happy());
        let writes = w.writes();
        let ok = w.walks()
            && writes.len() == 1
            && writes[0].payload.get("kind").and_then(Json::as_str) == Some(kind)
            && writes[0].payload.get("stage").and_then(Json::as_str) == Some(stage);
        if !ok {
            failures.push(format!(
                "`{activity}`: walks={} writes={:?} refusal={}",
                w.walks(),
                w.written_kinds(),
                w.refusal()
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "a restored demand did not write exactly one artifact.write:\n  {}",
        failures.join("\n  ")
    );
}
