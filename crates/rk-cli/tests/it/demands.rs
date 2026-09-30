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
//!
//! *(dated 2026-09-29, `s8g` build, package R-D2124-D2127-8g; D21-124, TAKEN PROVISIONALLY under
//! D21-105.)* **Ruled and closed: the double no longer hides it.** The bridge is `rk-cli`'s
//! [`rk_cli::verbs::run::bridged`] decorator, the one `choose_toolchain` wraps every adapter in. The
//! double now reads `checklist` STRICTLY as a list, as the real handler does (`items_of`'s envelope
//! parsing is deleted), and sits behind the same decorator, so this suite goes red over any shape the
//! real handler refuses. A [`Tap`] in front of the decorator records what the kernel hands the PORT
//! (the envelope, with its digest); the double records what the ADAPTER receives (the item list).

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

/// The `checklist` input as the real handler reads it: STRICTLY a list of record V's items
/// (`rk-adapter-toolchain-rust`'s `verbs::smoke`, `input(inputs, "smoke", "checklist")`), refused
/// with the handler's own `input-malformed` wording otherwise. *(dated 2026-09-29, `s8g` build,
/// package R-D2124-D2127-8g; D21-124 item 5: this was `items_of`, which parsed the envelope's
/// `content` itself and so went green over a shape the real handler refuses.)*
fn strict_items(checklist: &Json) -> Result<Vec<SmokeItem>, StepError> {
    serde_json::from_value(checklist.clone()).map_err(|e| StepError::Transport {
        kind: rk_types::error::TransportKind::Decode,
        detail: format!("smoke refused (input-malformed): the `checklist` input: {e}"),
    })
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
                let checklist = call
                    .inputs
                    .get("checklist")
                    .ok_or_else(|| refuse("smoke: no `checklist` input".to_owned()))?;
                let items = strict_items(checklist)?;
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

/// **What the kernel hands the PORT**, recorded in front of the checklist bridge (D21-124): every
/// `run_verb` call's inputs by verb, then delegated. *(Added 2026-09-29, `s8g` build, package
/// R-D2124-D2127-8g.)*
struct Tap {
    inner: Arc<dyn Toolchain>,
    calls: Mutex<Vec<(String, Json)>>,
}

impl Toolchain for Tap {
    fn detect(&self) -> Result<bool, StepError> {
        self.inner.detect()
    }
    fn build(&self, bound: StepBound) -> Result<BuildReport, StepError> {
        self.inner.build(bound)
    }
    fn test(&self, selection: &Selection, bound: StepBound) -> Result<TestOutcome, StepError> {
        self.inner.test(selection, bound)
    }
    fn lint(&self, bound: StepBound) -> Result<(), StepError> {
        self.inner.lint(bound)
    }
    fn classify(&self, path: &str) -> Result<Classification, StepError> {
        self.inner.classify(path)
    }
    fn change_summary(&self, range: &str, bound: StepBound) -> Result<ChangeSummary, StepError> {
        self.inner.change_summary(range, bound)
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.calls.lock().expect("unpoisoned").push((
            call.verb.to_owned(),
            Json::Object(
                call.inputs
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        ));
        self.inner.run_verb(call)
    }
}

// ------------------------------------------------------------------------------- the walk

/// One walk's product: the report, the item's journal after it, and what the double was handed.
struct Walked {
    report: ActivityReport,
    events: Vec<Event>,
    calls: Vec<(String, Json)>,
    /// What the kernel handed the port, in front of the bridge (D21-124).
    port_calls: Vec<(String, Json)>,
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

    /// The inputs the kernel handed the PORT for `verb` (the last such call), before the bridge.
    fn port_call(&self, verb: &str) -> Option<&Json> {
        self.port_calls
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
    // *(dated 2026-09-29, `s8g` build, package R-D2124-D2127-8g; D21-124 item 5.)* The double sits
    // behind the production decorator, and the tap in front of it. Was `toolchain:
    // Arc::clone(&double) as Arc<dyn Toolchain>`.
    let tap = Arc::new(Tap {
        inner: rk_cli::verbs::run::bridged(Arc::clone(&double) as Arc<dyn Toolchain>),
        calls: Mutex::new(Vec::new()),
    });
    let ports = Ports {
        store: Arc::clone(&db),
        harness: Arc::new(FakeHarness::scripted(Vec::new())) as Arc<dyn Harness>,
        tracker: Arc::new(BoardTracker::default()) as Arc<dyn Tracker>,
        docs: Arc::new(RecordingDocs::default()) as Arc<dyn DocsSite>,
        toolchain: Arc::clone(&tap) as Arc<dyn Toolchain>,
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
        port_calls: tap.calls.lock().expect("unpoisoned").clone(),
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
    // *(dated 2026-09-29, `s8g` build, package R-D2124-D2127-8g; D21-124.)* The envelope is read at
    // the PORT (the tap, before the bridge); the items at the ADAPTER (the double, after it). Was
    // both from the double's own call, which parsed the envelope itself (`items_of`).
    let envelope = w
        .port_call("smoke")
        .expect("the smoke verb reached the port")
        .get("checklist")
        .expect("the smoke call carries its `checklist` input");
    assert_eq!(
        envelope.get("key").and_then(Json::as_str),
        Some("checklist:smoke"),
        "the port is handed the resolver's envelope (ADR-0173): {envelope}"
    );
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
    let items = strict_items(
        w.call("smoke")
            .expect("the smoke verb reached the adapter")
            .get("checklist")
            .expect("the adapter's call carries `checklist`"),
    )
    .expect("the adapter is handed the shipped checklist as a list in record V's item form");
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

// ------------------------------------------------------------------------------- D21-125's guard

/// **The smoke checklist carries no `init` or `status` row until a record makes the smoke scratch
/// a repository** (`s8g` B2; D21-125, TAKEN PROVISIONALLY under D21-105; R4 = ADR-0194 decision 2's
/// dated note, proposed). *(Added 2026-09-29, `s8g` build, package R-D2124-D2127-8g.)*
///
/// ADR-0170 decision 4's escape clause (*"a row that needs the directory to be a repository makes
/// it one in its own argv"*) cannot be met under the same decision's rule (*"against the binaries
/// that build produced, never an installed `redkiln`"*): the handler's scratch is an empty
/// directory, it runs no git, and `argv[0]` must be a built binary. Measured, `redkiln init` and
/// `redkiln status` exit 3 there. A row pinning exit 3 would certify a refusal as health, so the
/// rows stay out, owed to whichever record makes the scratch a repository.
///
/// The file is PARSED (`rk_kernel::toml_subset`) and only each item's `argv` ELEMENTS are read: the
/// header comment and the `--help` rows' `stdout_contains` both name `init` and `status`, so a text
/// scan would be satisfied or broken by its own prose. The ADR's owed note is matched on its exact
/// phrase inside decision 2's text.
#[test]
fn smoke_checklist_carries_no_init_or_status_row_until_ruled() {
    let text = std::fs::read_to_string(root().join(".redkiln/checklists/smoke.toml"))
        .expect("the shipped smoke checklist reads");
    let doc = rk_kernel::toml_subset::parse(&text).expect("the smoke checklist parses");
    let items = doc
        .get("item")
        .and_then(rk_kernel::toml_subset::Value::as_array)
        .expect("the checklist declares [[item]] rows");
    assert!(!items.is_empty(), "the guard read no item");
    let mut argvs = 0usize;
    let mut offending = Vec::new();
    for item in items {
        let table = item.as_table().expect("an [[item]] row is a table");
        let argv = table
            .get("argv")
            .and_then(rk_kernel::toml_subset::Value::as_array)
            .expect("every item carries an argv array");
        argvs += 1;
        if argv
            .iter()
            .filter_map(rk_kernel::toml_subset::Value::as_str)
            .any(|a| a == "init" || a == "status")
        {
            offending.push(format!(
                "{}: {argv:?}",
                table
                    .get("id")
                    .and_then(rk_kernel::toml_subset::Value::as_str)
                    .unwrap_or("?")
            ));
        }
    }
    assert_eq!(argvs, items.len(), "every item's argv was read");
    assert!(
        offending.is_empty(),
        "the smoke checklist carries an `init` or `status` row: {offending:?}. D21-125: these rows \
         land only with the record that makes the smoke scratch a repository (remedy (i) handler \
         init plus a root commit through rk-git, or (ii) a checklist scratch-setup item form), \
         measured with positive expectations; the same commit inverts or retires this test"
    );

    let adr = std::fs::read_to_string(
        root().join("references/adr/0194-the-four-verify-and-release-demands-have-producers.md"),
    )
    .expect("R4 = ADR-0194 reads");
    let start = adr
        .find("2. **The smoke checklist**")
        .expect("ADR-0194 carries decision 2");
    let end = adr[start..]
        .find("3. **`ConfirmationResult`")
        .map_or(adr.len(), |e| start + e);
    // Whitespace is collapsed first: the record is hard-wrapped, and the phrase may cross a line.
    let decision_two = adr[start..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        decision_two.contains("owed to a ruling"),
        "ADR-0194 decision 2 no longer carries the owed note (\"owed to a ruling\") that names the two \
         rows (D21-125)"
    );
}
