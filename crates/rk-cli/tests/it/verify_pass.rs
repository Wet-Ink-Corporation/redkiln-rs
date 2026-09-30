//! **The verify pass — `TestReport`'s one producer on a lane** (`s8g` §8g.2 steps 1-5 and 7, §8g.3's
//! AC-139 row, §8g.5's rows AC-320, AC-331, AC-341, AC-344, AC-347, AC-348, AC-485; R1 = ADR-0191
//! decisions 1 and 2, proposed; D21-120, TAKEN PROVISIONALLY under D21-105). *(Added 2026-09-29,
//! `s8g` build, package S8G-P5-VERIFY-PASS.)*
//!
//! Two kinds of test, each where its property is observable:
//!
//! - **Through `rk_cli::verbs::run::run(`** over `pack-min`'s VERIFY variant (one live stage, named
//!   `check`, whose baseline `TestReport` is pass-sourced) with `rk-fixture-toolchain` answering as
//!   `cargo`: the pass is reached by the lane, selected by the required kind, names the adapter
//!   `detect()` chose, dispatches nothing, and survives a mid-run configuration edit.
//! - **Over `rk_cli::verify_pass` directly**, with a counting `Toolchain` double: the order of
//!   `setup` and `test`, the AC-320 refusal before any port call, the outcome mapping, the one keyed
//!   write, and the replay of the selection from the `verify_completed` row alone.
//!
//! The rigged fixture toolchain is evidence about the WIRING and about no real suite (its own
//! header says so): a tree carrying the committed `rk-fixture-green` answers one passing test.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rk_cli::exit::Exit;
use rk_cli::verify_pass::{
    self, PassInputs, Refused, ReportWriter, SetupRequest, Tree, Unverifiable, Verdict,
};
use rk_kernel::select::Instance;
use rk_kernel::verify::runner::VerifyIdentity;
use rk_ports::toolchain::{
    ChangeSummary, Classification, Selection, StepBound, TestOutcome, TestRunner, Toolchain,
    VerbCall, VerbOutput,
};
use rk_types::error::{CancelledBy, StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::verb_output::BuildReport;
use rk_types::verify::{
    Basis, BasisReason, TotalTestCountSource, VerifyCompleted, VerifyRange, VerifyReport,
    VerifySelection,
};

use crate::lane_e2e::{self, ScriptedFactory};
use crate::support::{self, PackVariant};

const ITEM: &str = "verify-1";
const RUST: &str = "rk-adapter-toolchain-rust";
const NODE: &str = "rk-adapter-toolchain-node";

// ------------------------------------------------------------------------------- the doubles

/// What the double's `test` answers.
#[derive(Debug, Clone)]
enum Script {
    Ran { passed: u32, failed: u32 },
    Refuses(StepError),
}

/// **A counting `Toolchain`**: every call is recorded by name, in order, and `test` records the
/// selection it was handed.
struct Counting {
    calls: Mutex<Vec<String>>,
    selections: Mutex<Vec<Selection>>,
    test: Script,
    setup: Result<(), StepError>,
}

impl Counting {
    fn new(test: Script) -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            selections: Mutex::new(Vec::new()),
            test,
            setup: Ok(()),
        }
    }

    fn with_setup(mut self, setup: Result<(), StepError>) -> Self {
        self.setup = setup;
        self
    }

    fn calls(&self) -> Vec<String> {
        self.calls.lock().expect("unpoisoned").clone()
    }

    fn note(&self, call: &str) {
        self.calls.lock().expect("unpoisoned").push(call.to_owned());
    }

    fn refuse(what: &str) -> StepError {
        StepError::Unsupported {
            capability: format!("{what}: the verify-pass double answers test and setup only"),
        }
    }
}

/// A whole-suite report whose counts agree (`TestOutcome::new`'s invariant).
fn outcome(passed: u32, failed: u32) -> TestOutcome {
    TestOutcome::new(
        VerifyReport {
            basis: Basis::WholeSuite,
            argv: vec![
                "cargo".to_owned(),
                "test".to_owned(),
                "--workspace".to_owned(),
            ],
            selected_test_count: passed + failed,
            total_test_count: passed + failed,
            total_test_count_source: TotalTestCountSource::OwnExecutedCount,
            wall_secs: 1,
            toolchain_version: "rustc 1.0.0-double".to_owned(),
            suite_sha: "0123456789abcdef".to_owned(),
            basis_reason: Some(BasisReason::ProbeUnverified),
        },
        passed,
        failed,
        TestRunner::named("fixture", "0"),
    )
    .expect("the double's counts agree")
}

impl Toolchain for Counting {
    fn detect(&self) -> Result<bool, StepError> {
        self.note("detect");
        Ok(true)
    }
    fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
        self.note("build");
        Err(Self::refuse("build"))
    }
    fn test(&self, selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        self.note("test");
        self.selections
            .lock()
            .expect("unpoisoned")
            .push(selection.clone());
        match &self.test {
            Script::Ran { passed, failed } => Ok(outcome(*passed, *failed)),
            Script::Refuses(e) => Err(e.clone()),
        }
    }
    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        self.note("lint");
        Err(Self::refuse("lint"))
    }
    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        self.note("classify");
        Ok(Classification::Text)
    }
    fn change_summary(&self, _range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        self.note("change_summary");
        Err(Self::refuse("change_summary"))
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.note(&format!("run_verb:{}", call.verb));
        self.setup.clone().map(|()| VerbOutput {
            kind: "SetupReport".to_owned(),
            value: serde_json::json!({}),
        })
    }
}

/// A writer that records what it was handed.
#[derive(Default)]
struct Recording {
    writes: Mutex<Vec<(String, Option<String>, serde_json::Value)>>,
}

impl Recording {
    fn writes(&self) -> Vec<(String, Option<String>, serde_json::Value)> {
        self.writes.lock().expect("unpoisoned").clone()
    }
}

impl ReportWriter for Recording {
    fn write(
        &self,
        kind: &str,
        key: Option<&str>,
        value: &serde_json::Value,
    ) -> Result<String, String> {
        self.writes.lock().expect("unpoisoned").push((
            kind.to_owned(),
            key.map(str::to_owned),
            value.clone(),
        ));
        Ok(format!(".redkiln/artifacts/{ITEM}/{kind}.json"))
    }
}

fn identity(adapter: &str) -> VerifyIdentity {
    VerifyIdentity {
        adapter: adapter.to_owned(),
        range: VerifyRange {
            base: "1111111111111111111111111111111111111111".to_owned(),
            head: "2222222222222222222222222222222222222222".to_owned(),
        },
    }
}

/// The pass over `tc`, un-keyed `TestReport`, a created tree with nothing declared to set up.
fn pass_over(
    tc: &dyn Toolchain,
    instance: &Instance,
    changed: Option<&[String]>,
    policy: Option<BasisReason>,
    writer: &dyn ReportWriter,
) -> (Result<Verdict, Refused>, Vec<Event>) {
    let tree = verify_pass::set_up(Tree::Reused, None).expect("a reused tree needs no setup");
    let id = identity(RUST);
    let mut rows: Vec<Event> = Vec::new();
    let verdict = verify_pass::run(
        tc,
        &tree,
        &PassInputs {
            item: ITEM,
            stage: "check",
            instance,
            changed,
            policy,
            identity: &id,
            bound: StepBound::Undeclared,
        },
        &mut |e| {
            rows.push(e);
            Ok(())
        },
        writer,
    );
    (verdict, rows)
}

fn completed(rows: &[Event]) -> Vec<VerifyCompleted> {
    rk_kernel::verify::runner::replay(rows).expect("every verify_completed row reads back")
}

// ------------------------------------------------------------------------ the e2e fixture

/// `pack-min`'s VERIFY variant with `rk-fixture-toolchain` as the Rust adapter's program, the
/// `[adapters] toolchains` row set to `toolchains` in that order, and — when `green` — the committed
/// marker that makes the stub answer one passing test.
fn verify_repo(toolchains: &[&str], green: bool) -> tempfile::TempDir {
    let repo = support::pack_min_repo(PackVariant::Verify);
    let root = repo.path();
    std::fs::write(
        root.join("Cargo.toml"),
        b"# RustToolchain::detect requires this file; the stub `cargo` answers. Never built.\n[package]\nname = \"rk-verify-fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("the fixture manifest writes");
    // The workspace's own pin, so the real `rustc -V` the adapter spawns in the run-owned tree
    // reports the release the libtest-line parser is pinned to (its `PINNED`), not the box default.
    std::fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rust-toolchain.toml"),
        root.join("rust-toolchain.toml"),
    )
    .expect("the workspace pin copies");
    let mut paths = vec!["Cargo.toml", "rust-toolchain.toml"];
    if green {
        std::fs::write(
            root.join("rk-fixture-green"),
            b"the verify fixture's green arm\n",
        )
        .expect("the green marker writes");
        paths.push("rk-fixture-green");
    }
    let mut add = vec!["add"];
    add.extend(paths);
    crate::git(root, &add);
    crate::git(
        root,
        &["commit", "--quiet", "-m", "the verify fixture's toolchain"],
    );

    let stub = PathBuf::from(env!("CARGO_BIN_EXE_rk-fixture-toolchain"));
    assert!(
        stub.is_file(),
        "the stub toolchain {} is not built",
        stub.display()
    );
    let escaped = stub.display().to_string().replace('\\', "\\\\");
    let list = toolchains
        .iter()
        .map(|t| format!("\"{t}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let path = root.join(".redkiln").join("config.toml");
    let src = std::fs::read_to_string(&path).expect("the configuration reads");
    let mut hits = 0;
    let out = src
        .lines()
        .map(|l| {
            if l.trim_start().starts_with("toolchains") {
                hits += 1;
                format!(
                    "toolchains = [{list}]\ntoolchain_programs = {{ \"{RUST}\" = \"{escaped}\" }}"
                )
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        hits, 1,
        "PREMISE: the fixture configuration has one `toolchains` row"
    );
    std::fs::write(&path, out).expect("the configuration installs");
    repo
}

/// The run's environment: what the Rust adapter's configuration requires on this box
/// (`SystemDrive` and a temp directory, `rk-adapter-toolchain-rust`'s measured table) and `PATH`, so
/// its `rustc -V` resolves. **No `SystemRoot`**, so the Node adapter's configuration is refused by
/// name (its measured table requires it) and never detects here. Projected from the process
/// environment by name, as the adapter crates' own `minimal_env` does; never `Env::capture()`.
fn verify_env() -> rk_cli::env::Env {
    let mut map: BTreeMap<String, String> = std::env::vars()
        .filter(|(k, _)| k.eq_ignore_ascii_case("PATH"))
        .collect();
    map.insert("SystemDrive".to_owned(), "C:".to_owned());
    map.insert("TMP".to_owned(), std::env::temp_dir().display().to_string());
    rk_cli::env::Env::from_map(map)
}

fn run_verify(root: &Path, factory: &ScriptedFactory) -> Exit {
    lane_e2e::ran(
        rk_cli::verbs::run::run(
            &lane_e2e::cli(root, ITEM),
            &lane_e2e::args(ITEM),
            &verify_env(),
            lane_e2e::seam(factory),
        ),
        ITEM,
    )
}

/// The pass's `verify_completed` rows (`origin = "pass"`) in the store.
fn pass_rows(root: &Path) -> Vec<VerifyCompleted> {
    completed(&support::store_rows(root))
        .into_iter()
        .filter(|v| v.origin == "pass")
        .collect()
}

fn test_report_writes(root: &Path) -> Vec<serde_json::Value> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some("TestReport"))
        .map(|e| e.payload)
        .collect()
}

// ------------------------------------------------------------------------------- the tests

/// **Selected by the required kind, never by a stage name** (ADR-0162 decision 5; M4). The fixture
/// pack's one stage is `check`; the pass runs there, because the stage requires `TestReport`.
#[test]
fn selected_by_required_kind_not_stage_name() {
    let owed = vec![Instance::of("TestReport"), Instance::of("MutationSummary")];
    let chosen: Vec<&str> = verify_pass::selected(&owed)
        .iter()
        .map(|i| i.kind.as_str())
        .collect();
    assert_eq!(
        chosen,
        vec!["TestReport"],
        "the pass is selected by its kind alone"
    );

    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    let pack = std::fs::read_to_string(root.join(".redkiln/packs/bug-standard.toml"))
        .expect("the installed pack reads");
    let loaded = rk_kernel::select::Pack::from_toml("bug-standard.toml", &pack)
        .expect("the installed pack loads");
    assert!(
        loaded.stage_order() == ["check".to_owned()] && loaded.rigor("verify").is_none(),
        "PREMISE: the fixture's one stage is `check`, and the pack declares no `verify` stage"
    );
    assert_eq!(
        run_verify(root, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    let rows = pass_rows(root);
    assert_eq!(
        rows.iter().map(|v| v.stage.as_str()).collect::<Vec<_>>(),
        vec!["check"],
        "the verify pass did not run at a stage named `check`: it is keyed on a stage name, not \
         on the required kind (M4)"
    );
    assert_eq!(
        test_report_writes(root).len(),
        1,
        "one TestReport write at `check`"
    );
}

/// **One keyed `TestReport`, one `artifact.write`**, through `rk-store`'s writer (ADR-0189), keyed
/// by the instance's `toolchain` element and never the adapter name (ADR-0073).
#[test]
fn writes_one_keyed_test_report_with_one_artifact_write() {
    let tree = tempfile::tempdir().expect("a run-owned tree stands in");
    let rows: Arc<Mutex<Vec<Event>>> = Arc::default();
    let (a, r) = (Arc::clone(&rows), Arc::clone(&rows));
    let writer = rk_store::render::ArtifactWriter::new(
        tree.path(),
        rk_store::render::StageRun {
            item: ITEM.to_owned(),
            stage: "check".to_owned(),
            run: "check-0123456789abcdef".to_owned(),
        },
        BTreeMap::from([("TestReport".to_owned(), "TestReport".to_owned())]),
        rk_store::render::Journal::new(
            move |e| {
                a.lock().expect("unpoisoned").push(e);
                Ok(())
            },
            move || Ok(r.lock().expect("unpoisoned").clone()),
        ),
    );
    let tc = Counting::new(Script::Ran {
        passed: 7,
        failed: 0,
    });
    let instance = Instance::keyed("TestReport", "rust");
    let changed = vec!["src/lib.rs".to_owned()];
    let (verdict, pass_rows) = pass_over(
        &tc,
        &instance,
        Some(&changed),
        Some(verify_pass::PASS_POLICY),
        &writer,
    );
    assert!(
        matches!(verdict, Ok(Verdict::Green { passed: 7, .. })),
        "a green suite is a green verify: {verdict:?}"
    );
    let writes: Vec<Event> = rows
        .lock()
        .expect("unpoisoned")
        .iter()
        .filter(|e| e.kind == EventKind::ArtifactWrite)
        .cloned()
        .collect();
    assert_eq!(writes.len(), 1, "exactly one artifact.write: {writes:?}");
    let row = &writes[0].payload;
    assert_eq!(
        row.get("kind").and_then(serde_json::Value::as_str),
        Some("TestReport")
    );
    assert_eq!(
        row.get("key").and_then(serde_json::Value::as_str),
        Some("rust"),
        "the write is keyed by the toolchain element, never the adapter name `{RUST}`"
    );
    assert_eq!(
        row.get("stage").and_then(serde_json::Value::as_str),
        Some("check")
    );
    let sidecar = tree
        .path()
        .join(".redkiln/artifacts")
        .join(ITEM)
        .join("TestReport.json");
    let value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&sidecar).expect("the sidecar is written"))
            .expect("the sidecar is JSON");
    assert_eq!(value.get("passed"), Some(&serde_json::json!(7)));
    assert_eq!(
        value.get("toolchain").and_then(serde_json::Value::as_str),
        Some("rust")
    );
    assert_eq!(completed(&pass_rows).len(), 1, "one verify_completed row");
}

/// **A two-toolchain item holds on the second key** (D21-122 item 2, TAKEN PROVISIONALLY under
/// D21-105): the pass writes `TestReport{rust}`, then the same stage run's `TestReport{node}` is
/// refused by the writer by name and reaches [`Refused::Write`]; no row is written for `node`, so
/// D21-120's guard (which matches a keyed instance on its `key` member) still owes it, and rust's
/// sidecar is intact. *(Added 2026-09-29, s8g build, package R-D2122-D2123-8g.)*
#[test]
fn a_two_toolchain_item_holds_on_the_second_key() {
    let tree = tempfile::tempdir().expect("a run-owned tree stands in");
    let rows: Arc<Mutex<Vec<Event>>> = Arc::default();
    let (a, r) = (Arc::clone(&rows), Arc::clone(&rows));
    let writer = rk_store::render::ArtifactWriter::new(
        tree.path(),
        rk_store::render::StageRun {
            item: ITEM.to_owned(),
            stage: "check".to_owned(),
            run: "check-0123456789abcdef".to_owned(),
        },
        BTreeMap::from([("TestReport".to_owned(), "TestReport".to_owned())]),
        rk_store::render::Journal::new(
            move |e| {
                a.lock().expect("unpoisoned").push(e);
                Ok(())
            },
            move || Ok(r.lock().expect("unpoisoned").clone()),
        ),
    );
    let changed = vec!["src/lib.rs".to_owned()];
    let keyed_writes = || -> Vec<Option<String>> {
        rows.lock()
            .expect("unpoisoned")
            .iter()
            .filter(|e| e.kind == EventKind::ArtifactWrite)
            .map(|e| {
                e.payload
                    .get("key")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
            })
            .collect()
    };

    let rust = Counting::new(Script::Ran {
        passed: 7,
        failed: 0,
    });
    let (first, _) = pass_over(
        &rust,
        &Instance::keyed("TestReport", "rust"),
        Some(&changed),
        Some(verify_pass::PASS_POLICY),
        &writer,
    );
    assert!(
        matches!(first, Ok(Verdict::Green { passed: 7, .. })),
        "PREMISE: the first key's green suite is written: {first:?}"
    );
    let sidecar = tree
        .path()
        .join(".redkiln/artifacts")
        .join(ITEM)
        .join("TestReport.json");
    let rust_bytes = std::fs::read(&sidecar).expect("the first key's sidecar is written");

    let node = Counting::new(Script::Ran {
        passed: 3,
        failed: 0,
    });
    let (second, _) = pass_over(
        &node,
        &Instance::keyed("TestReport", "node"),
        Some(&changed),
        Some(verify_pass::PASS_POLICY),
        &writer,
    );
    match &second {
        Err(Refused::Write(why)) => assert!(
            why.contains("KeyedSidecarCollision")
                && why.contains("\"rust\"")
                && why.contains("\"node\""),
            "the write refusal does not name the collision: {why}"
        ),
        other => panic!(
            "a second key for `TestReport` in one stage run was not refused as a write: {other:?}"
        ),
    }
    assert_eq!(
        keyed_writes(),
        vec![Some("rust".to_owned())],
        "a row was journalled for the refused key, so D21-120's guard would not owe it"
    );
    assert_eq!(
        std::fs::read(&sidecar).expect("the sidecar is still there"),
        rust_bytes,
        "the refused key replaced the first key's report"
    );
}

/// **`verify_completed` names the adapter `detect()` chose** (AC-487's carrier; M1): the fixture
/// lists Node FIRST, Node's configuration is refused by the clean environment, and the Rust
/// adapter over the stub detects. The row names Rust.
#[test]
fn verify_completed_names_the_adapter_detect_chose() {
    let repo = verify_repo(&[NODE, RUST], true);
    let root = repo.path();
    assert_eq!(
        run_verify(root, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    let rows = pass_rows(root);
    assert_eq!(
        rows.iter().map(|v| v.adapter.as_str()).collect::<Vec<_>>(),
        vec![RUST],
        "the row names the configured list's first entry, not the adapter detect() chose (M1)"
    );
}

/// **A Node-detected project records the Node adapter.** `run`'s own selection loop over the Node
/// fixture project (Rust listed FIRST, and it does not detect: no `Cargo.toml`), then the pass
/// through the chosen toolchain; the row names Node.
#[test]
fn node_detected_fixture_records_the_node_adapter() {
    let project = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../rk-adapter-toolchain-node/tests/fixtures/project");
    assert!(
        project.join("node_modules/.bin/vitest").is_file(),
        "PREMISE: run `pnpm install --frozen-lockfile --dir {}` once",
        project.display()
    );
    let env: BTreeMap<String, String> = std::env::vars()
        .filter(|(k, _)| k.eq_ignore_ascii_case("SystemRoot") || k.eq_ignore_ascii_case("PATH"))
        .collect();
    let common = tempfile::tempdir().expect("a common dir for the toolchain scratch");
    let toolchains = vec![RUST.to_owned(), NODE.to_owned()];
    let chosen = rk_cli::verbs::run::choose_toolchain(
        &rk_cli::verbs::run::ToolchainCandidates {
            toolchains: &toolchains,
            timeout_secs: 600,
            programs: None,
            common_dir: common.path(),
        },
        &rk_cli::env::Env::from_map(env),
        &project,
        &mut |_| {},
    )
    .unwrap_or_else(|exit| panic!("the selection loop ran: {exit:?}"));
    assert_eq!(
        chosen.adapter.as_deref(),
        Some(NODE),
        "PREMISE: Node detects the fixture"
    );

    let id = verify_pass::identity(chosen.adapter.as_deref(), identity(NODE).range);
    let tree = verify_pass::set_up(Tree::Reused, None).expect("a reused tree");
    let instance = Instance::of("TestReport");
    let changed: Vec<String> = Vec::new();
    let mut rows: Vec<Event> = Vec::new();
    let verdict = verify_pass::run(
        chosen.toolchain.as_ref(),
        &tree,
        &PassInputs {
            item: ITEM,
            stage: "check",
            instance: &instance,
            changed: Some(&changed),
            policy: Some(verify_pass::PASS_POLICY),
            identity: &id,
            bound: StepBound::Undeclared,
        },
        &mut |e| {
            rows.push(e);
            Ok(())
        },
        &Recording::default(),
    );
    assert!(
        !matches!(verdict, Ok(Verdict::Unverifiable { .. }) | Err(_)),
        "the Node suite completed: {verdict:?}"
    );
    assert_eq!(
        completed(&rows)
            .iter()
            .map(|v| v.adapter.as_str())
            .collect::<Vec<_>>(),
        vec![NODE]
    );
}

/// **`failed > 0` is a failed verify, never a report of green** (M2): no `TestReport` is written,
/// and the `verify_completed` row carries the red counts.
#[test]
fn red_suite_is_a_failed_verify_never_green() {
    let tc = Counting::new(Script::Ran {
        passed: 3,
        failed: 1,
    });
    let writer = Recording::default();
    let (verdict, rows) = pass_over(
        &tc,
        &Instance::of("TestReport"),
        Some(&[]),
        Some(verify_pass::PASS_POLICY),
        &writer,
    );
    assert!(
        matches!(
            verdict,
            Ok(Verdict::Failed {
                passed: 3,
                failed: 1
            })
        ),
        "a red suite must be a failed verify: {verdict:?}"
    );
    assert!(
        writer.writes().is_empty(),
        "a red suite wrote a TestReport: {:?}",
        writer.writes()
    );
    let vc = completed(&rows);
    assert_eq!(
        (vc.len(), vc[0].failed),
        (1, 1),
        "the completed red run is journalled"
    );
}

/// **A timeout and an OS kill are `Unverifiable`** (AC-331; M3): never a pass, never a failure
/// attributable to the candidate, no `verify_completed` row and no write.
#[test]
fn ac331_timeout_and_kill_are_unverifiable() {
    for (error, want) in [
        (StepError::Timeout { after_secs: 5 }, Unverifiable::Timeout),
        (
            StepError::Transport {
                kind: TransportKind::Io,
                detail: "os-killed: the child was killed by the OS".to_owned(),
            },
            Unverifiable::Killed,
        ),
        (
            StepError::Cancelled {
                by: CancelledBy::Timeout,
            },
            Unverifiable::Killed,
        ),
    ] {
        let tc = Counting::new(Script::Refuses(error.clone()));
        let writer = Recording::default();
        let (verdict, rows) = pass_over(
            &tc,
            &Instance::of("TestReport"),
            Some(&[]),
            Some(verify_pass::PASS_POLICY),
            &writer,
        );
        match verdict {
            Ok(Verdict::Unverifiable { why, .. }) => {
                assert_eq!(why, want, "{error:?} is `Unverifiable` as {want:?}");
            }
            other => panic!("{error:?} must be Unverifiable, never a pass or a failure: {other:?}"),
        }
        assert!(
            completed(&rows).is_empty(),
            "a killed run journalled verify_completed"
        );
        assert!(writer.writes().is_empty());
    }
}

/// **A missing binary is `Unverifiable { cannot_run }`** (AC-344's cannot-run fixture): the real
/// Rust adapter over a program that does not exist.
#[test]
fn ac344_missing_binary_is_cannot_run() {
    let root = tempfile::tempdir().expect("a project root");
    std::fs::write(
        root.path().join("Cargo.toml"),
        b"[package]\nname = \"rk-missing\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("the manifest writes");
    let scratch = tempfile::tempdir().expect("scratch");
    let missing = root.path().join("no-such-cargo.exe");
    let config = rk_adapter_toolchain_rust::Config::new(
        missing,
        std::time::Duration::from_secs(60),
        root.path(),
        verify_env().as_map().clone(),
        scratch.path().to_path_buf(),
    )
    .expect("a Config over a missing program constructs")
    .with_runner(rk_adapter_toolchain_rust::RunnerPolicy::Force(
        rk_adapter_toolchain_rust::ForcedRunner::CargoTest,
    ));
    let tc = rk_adapter_toolchain_rust::RustToolchain::new(config).expect("the adapter builds");
    let writer = Recording::default();
    let (verdict, rows) = pass_over(
        &tc,
        &Instance::of("TestReport"),
        Some(&[]),
        Some(verify_pass::PASS_POLICY),
        &writer,
    );
    match verdict {
        Ok(Verdict::Unverifiable { why, .. }) => assert_eq!(why, Unverifiable::CannotRun),
        other => panic!("a missing binary must be Unverifiable {{ cannot_run }}: {other:?}"),
    }
    assert!(completed(&rows).is_empty());
    assert!(writer.writes().is_empty());
}

/// **The pass dispatches nothing** (AC-347's zero-dispatch half): a lane whose only stage is held
/// by the pass completes with zero harness dispatches and zero dispatch rows.
#[test]
fn ac347_the_pass_dispatches_nothing() {
    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    let factory = ScriptedFactory::new(Vec::new());
    assert_eq!(run_verify(root, &factory), Exit::Success);
    assert_eq!(pass_rows(root).len(), 1, "PREMISE: the pass ran");
    assert_eq!(
        factory.dispatches(),
        0,
        "the verify run dispatched to the harness"
    );
    let dispatch_rows: Vec<String> = support::store_rows(root)
        .iter()
        .filter(|e| e.kind.wire_name().starts_with("dispatch"))
        .map(|e| e.kind.wire_name().to_owned())
        .collect();
    assert!(
        dispatch_rows.is_empty(),
        "dispatch rows in a verify run: {dispatch_rows:?}"
    );
}

/// **AC-320: an unresolved changed-file set or selection policy refuses before any port call** —
/// zero calls on the counting double, zero rows (no gate row), and a `StepError` to exit by.
#[test]
fn ac320_unresolved_inputs_refuse_before_any_port_call() {
    for (changed, policy, what) in [
        (None, Some(verify_pass::PASS_POLICY), "the changed-file set"),
        (Some(&[][..]), None, "the selection policy"),
    ] {
        let tc = Counting::new(Script::Ran {
            passed: 1,
            failed: 0,
        });
        let writer = Recording::default();
        let (verdict, rows) = pass_over(&tc, &Instance::of("TestReport"), changed, policy, &writer);
        match verdict {
            Err(refused @ Refused::Unresolved { .. }) => {
                assert!(
                    refused.to_string().contains(what) && refused.to_string().contains("AC-320"),
                    "the refusal names {what}: {refused}"
                );
                assert!(matches!(
                    refused.step_error(),
                    StepError::Unsupported { .. }
                ));
            }
            other => panic!("an unresolved {what} must refuse: {other:?}"),
        }
        assert!(
            tc.calls().is_empty(),
            "a port call was made: {:?}",
            tc.calls()
        );
        assert!(rows.is_empty(), "a row was written: {rows:?}");
        assert!(writer.writes().is_empty());
    }
    // CONTROL: both resolved, one `test` call.
    let tc = Counting::new(Script::Ran {
        passed: 1,
        failed: 0,
    });
    let (verdict, _) = pass_over(
        &tc,
        &Instance::of("TestReport"),
        Some(&[]),
        Some(verify_pass::PASS_POLICY),
        &Recording::default(),
    );
    assert!(matches!(verdict, Ok(Verdict::Green { .. })), "{verdict:?}");
    assert_eq!(tc.calls(), vec!["test"]);
}

/// A `Tracker` that counts every call.
#[derive(Default)]
struct CountingTracker {
    calls: AtomicUsize,
}

impl rk_ports::tracker::Tracker for CountingTracker {
    fn project(&self, _item: &str) -> Result<rk_ports::tracker::ExternalRef, StepError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(StepError::Unsupported {
            capability: "project".to_owned(),
        })
    }
    fn inbox(&self) -> Result<Vec<rk_ports::tracker::TypedInput>, StepError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(Vec::new())
    }
}

/// **The pass calls no `Tracker` method** (AC-341's port-call half): run with a counting `Tracker`
/// in the `Ports` value beside the toolchain, green and red, and the count stays zero. The pass
/// takes no `Tracker` at all, and its code lines name none (the static floor, with a control).
#[test]
fn ac341_the_pass_calls_no_tracker_method() {
    let tracker = Arc::new(CountingTracker::default());
    for script in [
        Script::Ran {
            passed: 2,
            failed: 0,
        },
        Script::Ran {
            passed: 1,
            failed: 1,
        },
    ] {
        let ports = rk_ports::root::Ports {
            store: Arc::new(Mutex::new(rk_ports::fakes::VecStore::default())),
            harness: Arc::new(rk_ports::fakes::FakeHarness::scripted(Vec::new())),
            tracker: Arc::clone(&tracker) as Arc<dyn rk_ports::tracker::Tracker>,
            docs: Arc::new(rk_ports::fakes::RecordingDocs::default()),
            toolchain: Arc::new(Counting::new(script)),
        };
        let (verdict, _) = pass_over(
            ports.toolchain.as_ref(),
            &Instance::of("TestReport"),
            Some(&[]),
            Some(verify_pass::PASS_POLICY),
            &Recording::default(),
        );
        assert!(verdict.is_ok(), "{verdict:?}");
    }
    assert_eq!(
        tracker.calls.load(Ordering::SeqCst),
        0,
        "the pass called the Tracker"
    );

    let src = std::fs::read_to_string(crate::src_dir().join("verify_pass.rs"))
        .expect("verify_pass.rs reads");
    let hits = |text: &str| {
        text.lines()
            .map(str::trim_start)
            .filter(|l| !l.starts_with("//"))
            .filter(|l| l.contains("Tracker") || l.contains("tracker"))
            .count()
    };
    assert_eq!(
        hits("// the Tracker is named in prose\nlet t = ports.tracker;\n"),
        1,
        "CONTROL"
    );
    assert_eq!(
        hits(&src),
        0,
        "verify_pass.rs names a Tracker on a code line"
    );
}

/// **AC-348: the selection is recomputed from `verify_completed` alone** — the row's recorded
/// selection, reason and path list rebuild the `Selection` the port was handed, byte for byte.
#[test]
fn ac348_selection_is_recomputed_from_verify_completed() {
    let tc = Counting::new(Script::Ran {
        passed: 4,
        failed: 0,
    });
    let changed = vec![
        "crates/a/src/lib.rs".to_owned(),
        "crates/b/src/main.rs".to_owned(),
    ];
    let (verdict, rows) = pass_over(
        &tc,
        &Instance::of("TestReport"),
        Some(&changed),
        Some(verify_pass::PASS_POLICY),
        &Recording::default(),
    );
    assert!(matches!(verdict, Ok(Verdict::Green { .. })), "{verdict:?}");
    let handed = tc.selections.lock().expect("unpoisoned").clone();
    assert_eq!(handed.len(), 1);
    let vc = completed(&rows);
    assert_eq!(vc.len(), 1);
    assert_eq!(vc[0].selection, VerifySelection::WholeSuite);
    assert_eq!(vc[0].paths, changed, "the path list is read back verbatim");
    let recomputed = verify_pass::whole_suite(
        vc[0]
            .basis_reason
            .expect("a whole-suite row carries its reason"),
    );
    assert_eq!(
        format!("{recomputed:?}").into_bytes(),
        format!("{:?}", handed[0]).into_bytes(),
        "the selection recomputed from the row is not the one the port was handed"
    );
}

/// A `ConfigReader` that answers the real file, then EDITS it to name another adapter — the
/// mid-run edit AC-485 describes — and counts its reads.
struct EditingReader {
    reads: AtomicUsize,
}

impl rk_cli::verbs::ConfigReader for EditingReader {
    fn read(&self, path: &Path) -> Result<rk_cli::config::ConfigFile, rk_cli::config::ConfigError> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        let file = rk_cli::verbs::FileConfigReader.read(path)?;
        let src = std::fs::read_to_string(path).expect("the configuration reads");
        let edited = src
            .lines()
            .map(|l| {
                if l.trim_start().starts_with("toolchains") {
                    format!("toolchains = [\"{NODE}\"]")
                } else {
                    l.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_ne!(edited, src, "PREMISE: the edit moves the adapter row");
        std::fs::write(path, edited).expect("the mid-run edit writes");
        Ok(file)
    }
}

/// **AC-485: a mid-run configuration edit does not move the verify adapter.** The configuration is
/// read once at startup; the file is then edited to name Node; the pass's row still names the
/// adapter resolved at startup.
#[test]
fn ac485_config_edit_mid_run_does_not_move_the_verify_adapter() {
    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    let reader = EditingReader {
        reads: AtomicUsize::new(0),
    };
    let factory = ScriptedFactory::new(Vec::new());
    let exit = lane_e2e::ran(
        rk_cli::verbs::run::run(
            &lane_e2e::cli(root, ITEM),
            &lane_e2e::args(ITEM),
            &verify_env(),
            rk_cli::verbs::Startup {
                config: &reader,
                ..lane_e2e::seam(&factory)
            },
        ),
        ITEM,
    );
    assert_eq!(exit, Exit::Success);
    assert_eq!(
        reader.reads.load(Ordering::SeqCst),
        1,
        "the configuration is read once"
    );
    let edited = std::fs::read_to_string(root.join(".redkiln/config.toml")).expect("reads");
    assert!(
        edited.contains(NODE) && !edited.contains(&format!("\"{RUST}\"]")),
        "PREMISE: edited"
    );
    assert_eq!(
        pass_rows(root)
            .iter()
            .map(|v| v.adapter.as_str())
            .collect::<Vec<_>>(),
        vec![RUST],
        "the verify adapter moved with a mid-run configuration edit"
    );
}

/// **The lane reaches the pass through `verbs::run::run`** (D21-120 item 4): the stage is held by
/// the pass-owed guard, the pass writes `TestReport`, `run` re-enters the same stage once, and the
/// lane ends with its `closed` mark; the report is landed on the run ref.
#[test]
fn the_lane_reaches_the_pass_through_verbs_run_run() {
    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    assert_eq!(
        run_verify(root, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    let rows = pass_rows(root);
    assert_eq!(rows.len(), 1, "the lane never reached the verify pass");
    assert_eq!(
        (rows[0].selection.clone(), rows[0].basis_reason),
        (
            VerifySelection::WholeSuite,
            Some(BasisReason::ProbeUnverified)
        ),
        "the pass requests the whole suite (AC-328)"
    );
    assert_eq!(test_report_writes(root).len(), 1);
    let stages = lane_e2e::story_stages(root, ITEM);
    assert_eq!(
        stages,
        vec!["check".to_owned(), "closed".to_owned()],
        "the lane entered `check` once and ended: {stages:?}"
    );
    let landed = lane_e2e::git_rc(
        root,
        &[
            "cat-file",
            "-e",
            &format!("rk-run/{ITEM}:.redkiln/artifacts/{ITEM}/TestReport.json"),
        ],
    );
    assert_eq!(landed, Some(0), "the TestReport is landed on rk-run/{ITEM}");
}

/// **`setup` precedes the pass's one `test` call on a fresh tree, and a failed `setup` leaves
/// zero** (§8g.2 step 1; D21-43 N25; M17).
#[test]
fn setup_precedes_the_whole_suite_in_a_fresh_run_worktree() {
    let request = SetupRequest {
        projects: vec![serde_json::json!(
            "crates/rk-adapter-toolchain-node/tests/fixtures/project"
        )],
        bound: StepBound::Undeclared,
    };
    let tc = Counting::new(Script::Ran {
        passed: 1,
        failed: 0,
    });
    let tree = verify_pass::set_up(Tree::Created, Some((&tc, &request))).expect("setup runs");
    let id = identity(RUST);
    let instance = Instance::of("TestReport");
    let verdict = verify_pass::run(
        &tc,
        &tree,
        &PassInputs {
            item: ITEM,
            stage: "check",
            instance: &instance,
            changed: Some(&[]),
            policy: Some(verify_pass::PASS_POLICY),
            identity: &id,
            bound: StepBound::Undeclared,
        },
        &mut |_| Ok(()),
        &Recording::default(),
    );
    assert!(matches!(verdict, Ok(Verdict::Green { .. })), "{verdict:?}");
    assert_eq!(
        tc.calls(),
        vec!["run_verb:setup", "test"],
        "setup must precede the one whole-suite test call on a created tree (M17)"
    );

    let failing = Counting::new(Script::Ran {
        passed: 1,
        failed: 0,
    })
    .with_setup(Err(StepError::Unsupported {
        capability: "setup: pnpm refused".to_owned(),
    }));
    assert!(
        verify_pass::set_up(Tree::Created, Some((&failing, &request))).is_err(),
        "a failed setup makes no witness"
    );
    assert_eq!(
        failing.calls(),
        vec!["run_verb:setup"],
        "a failed setup leaves zero test calls"
    );

    let reused = Counting::new(Script::Ran {
        passed: 1,
        failed: 0,
    });
    verify_pass::set_up(Tree::Reused, Some((&reused, &request))).expect("a reused tree");
    assert!(
        reused.calls().is_empty(),
        "a reused tree was set up at its creation"
    );
}

/// **AC-139: both shipped packs block a missing verification** (D21-43 N24; R2 = ADR-0192
/// decision 5; M8). Each shipped pack refuses its verification stage over a criterion at
/// `verification: None`, naming the AC; the same spec passes under a copy with the row removed —
/// the policy row decides, not the type (AC-138).
#[test]
fn ac139_shipped_packs_block_a_missing_verification() {
    let spec = vec![
        rk_types::acceptance::AcceptanceCriterion::ingest(serde_json::json!({
            "id": "AC-900",
            "criterion": "the fixture criterion carries no verification"
        }))
        .expect("the criterion ingests"),
    ];
    let packs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.redkiln/packs");
    for name in ["bug-standard.toml", "story.toml"] {
        let src = std::fs::read_to_string(packs.join(name)).expect("the shipped pack reads");
        let pack = rk_kernel::select::Pack::from_toml(name, &src).expect("the shipped pack loads");
        let stages: Vec<String> = pack
            .stage_order()
            .iter()
            .filter(|s| pack.requires_verification_at(s))
            .cloned()
            .collect();
        assert_eq!(
            stages.len(),
            1,
            "{name} requires a verification at exactly one stage: {stages:?}"
        );
        let refused = rk_kernel::verify::admissibility::admissible(&stages[0], &spec, &pack)
            .expect_err("the shipped default must block a missing verification (AC-139, N24)");
        assert_eq!(
            refused.acs,
            vec!["AC-900".to_owned()],
            "{name} names the AC"
        );

        let row = "verification_required = [\"verify\"]";
        assert_eq!(
            src.matches(row).count(),
            1,
            "PREMISE: {name} carries the row once"
        );
        let without = src.replace(row, "");
        let bare = rk_kernel::select::Pack::from_toml(name, &without).expect("the copy loads");
        assert!(
            rk_kernel::verify::admissibility::admissible(&stages[0], &spec, &bare).is_ok(),
            "{name}: with the row removed the same spec passes (AC-138)"
        );
    }
}

// ------------------------------------------------------------------------------------------------
// §8g.2 step 6 and §8g.5: the mutation harness (AC-338, AC-339), AC-353's kill and resume, AC-359's
// drift, and `verify.ac-discriminates` answered through the loaded registry. *(dated 2026-09-29,
// `s8g` build, package S8G-P6-MUTATION-RESUME-DRIFT; R1 = ADR-0191 decisions 10 and 11, proposed.)*
// ------------------------------------------------------------------------------------------------

/// The item the harness tests' run-owned tree is for.
const MUT_ITEM: &str = "mut-1";

/// The declared mutation: one line of the tracked `src.txt` changed.
const MUT_PATCH: &str = "--- a/src.txt\n+++ b/src.txt\n@@ -1 +1 @@\n-original\n+mutated\n";

/// A declared patch that applies nowhere: its context line is not in the file.
const MUT_PATCH_STALE: &str = "--- a/src.txt\n+++ b/src.txt\n@@ -1 +1 @@\n-absent\n+mutated\n";

/// The declared suite.
const MUT_SUITE: &str = "rk_fixture::the_reviewed_case";

/// **A `Toolchain` rooted at one tree**: its `test` records the root and what `src.txt` held there
/// when the suite ran, and, when `restore` is set, writes the original bytes back before it answers
/// green — F17's shape, an agent that restored the fix from the object store mid-run.
struct AtTree {
    root: PathBuf,
    restore: bool,
    seen: Arc<Mutex<Vec<(PathBuf, String)>>>,
}

impl Toolchain for AtTree {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }
    fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
        Err(Counting::refuse("build"))
    }
    fn test(&self, _selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        let file = self.root.join("src.txt");
        // Line endings normalised: a checkout under this box's `core.autocrlf` writes CRLF.
        let held = std::fs::read_to_string(&file)
            .unwrap_or_else(|e| format!("unreadable: {e}"))
            .replace("\r\n", "\n");
        self.seen
            .lock()
            .expect("unpoisoned")
            .push((self.root.clone(), held));
        if self.restore {
            std::fs::write(&file, b"original\n").expect("the double restores the file");
        }
        Ok(outcome(1, 0))
    }
    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        Err(Counting::refuse("lint"))
    }
    fn classify(&self, _path: &str) -> Result<Classification, StepError> {
        Ok(Classification::Text)
    }
    fn change_summary(&self, _range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        Err(Counting::refuse("change_summary"))
    }
    fn run_verb(&self, _call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        Err(Counting::refuse("run_verb"))
    }
}

/// A repository with a tracked `src.txt` and a run-owned tree for [`MUT_ITEM`] at
/// `.redkiln/worktrees/mut-1` on `rk-run/mut-1`, created as `run` creates it (the run root ignored).
fn mutation_repo() -> (tempfile::TempDir, rk_git::worktree::RunTree, String) {
    let repo = crate::temp_repo();
    let root = repo.path();
    crate::ignore_run_root(root);
    std::fs::write(root.join("src.txt"), b"original\n").expect("src.txt writes");
    crate::git(root, &["add", "src.txt"]);
    crate::git(root, &["commit", "--quiet", "-m", "the mutated file"]);
    let tree = crate::run_tree(root, MUT_ITEM);
    std::fs::create_dir_all(tree.path.parent().expect("the run root")).expect("the run root");
    crate::git(
        root,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &tree.branch,
            tree.path.to_str().expect("utf-8"),
        ],
    );
    let head = git_out(&tree.path, &["rev-parse", "HEAD"]);
    (repo, tree, head.trim().to_owned())
}

/// Run the harness over [`mutation_repo`]'s trees with an [`AtTree`] factory.
fn harness(
    root: &Path,
    tree: &rk_git::worktree::RunTree,
    head: &str,
    patch: &str,
    restore: bool,
) -> (
    verify_pass::MutationEvidence,
    Vec<(PathBuf, String)>,
    Vec<Event>,
) {
    let found = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let seen: Arc<Mutex<Vec<(PathBuf, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let seen_by = Arc::clone(&seen);
    let at = move |p: &Path| -> Result<Arc<dyn Toolchain>, String> {
        Ok(Arc::new(AtTree {
            root: p.to_path_buf(),
            restore,
            seen: Arc::clone(&seen_by),
        }))
    };
    let id = identity(RUST);
    let mut rows: Vec<Event> = Vec::new();
    let evidence = verify_pass::mutate(
        &verify_pass::MutationSite {
            exec: &found.exec,
            repo: &found.worktree,
            tree,
            candidate: head,
        },
        "AC-1",
        patch,
        MUT_SUITE,
        &at,
        &verify_pass::MutationCall {
            item: MUT_ITEM,
            stage: "check",
            identity: &id,
            bound: StepBound::Undeclared,
        },
        &mut |e| {
            rows.push(e);
            Ok(())
        },
    );
    let seen = seen.lock().expect("unpoisoned").clone();
    (evidence, seen, rows)
}

/// The run-owned tree's tree hash, ADR-0105 clause 4's one derivation over its working state.
fn run_tree_hash(root: &Path, tree: &Path) -> String {
    let found = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    rk_git::tree::tree_hash(
        &found.exec,
        &rk_git::path::WorktreePath::new(tree).expect("the run tree path"),
        &found.common_dir,
        &rk_git::tree::TreeBasis::WorkingState,
    )
    .expect("the tree hash derives")
    .as_str()
    .to_owned()
}

/// `git worktree list --porcelain`'s `worktree` lines, `/`-separated and case-folded.
fn listed_worktrees(root: &Path) -> Vec<String> {
    git_out(root, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|l| l.strip_prefix("worktree "))
        .map(|p| p.replace('\\', "/").to_lowercase())
        .collect()
}

fn folded(p: &Path) -> String {
    p.display().to_string().replace('\\', "/").to_lowercase()
}

/// **AC-338: a mutation restored before the suite finished is `Unverifiable`, never
/// `mutation-survived` and never `pinned`** (F17's shape; M11). The harness re-reads the mutated
/// region after the suite and compares its content hash to the one the applied patch left.
#[test]
fn ac338_restored_mutation_is_unverifiable() {
    use rk_kernel::verify::flip::{FlipOutcome, FlipUnverifiable};

    let (repo, tree, head) = mutation_repo();
    let root = repo.path();
    let (restored, seen, _) = harness(root, &tree, &head, MUT_PATCH, true);
    assert_eq!(
        seen.len(),
        1,
        "the declared suite ran once in the harness's tree: {seen:?} ({restored:?})"
    );
    assert_eq!(
        seen[0].1, "mutated\n",
        "PREMISE: the suite ran over the applied mutation"
    );
    assert_eq!(
        restored.outcome,
        Some(FlipOutcome::Unverifiable(
            FlipUnverifiable::NotPresentAtCompletion
        )),
        "a mutation restored mid-run must be Unverifiable (AC-338; F17), never survived or pinned: \
         {restored:?}"
    );
    assert_eq!(
        restored.run.as_ref().map(|r| r.still_present),
        Some(false),
        "the post-run content hash of the mutated region must record the restore: {restored:?}"
    );

    // The ledger reads it as no evidence: `unpinned`, never `pinned` or `mutation_survived`.
    let spec = vec![
        rk_types::acceptance::AcceptanceCriterion::ingest(serde_json::json!({
            "id": "AC-1",
            "criterion": "the fixture's one criterion",
            "mutation": { "kind": "declared", "patch": MUT_PATCH, "suite": MUT_SUITE },
        }))
        .expect("the criterion ingests"),
    ];
    let runs = BTreeMap::from([(
        "AC-1".to_owned(),
        restored.outcome.expect("an outcome was recorded"),
    )]);
    let ledger = rk_kernel::verify::ledger::compute_ledger(
        &spec,
        &runs,
        &BTreeMap::new(),
        &rk_ports::toolchain::MutationFeasibility::Feasible,
        &[],
    );
    assert_eq!(
        ledger.rows[0].status,
        rk_types::artifact::ledger::LedgerStatus::Unpinned,
        "a restored mutation is no evidence"
    );

    // CONTROL: the same run with the mutation left in place survives (a green suite over a present
    // mutant), so the Unverifiable above is the hash compare's, not the double's.
    let (kept, _, _) = harness(root, &tree, &head, MUT_PATCH, false);
    assert_eq!(
        kept.outcome,
        Some(FlipOutcome::MutationSurvived),
        "CONTROL: a mutation still present at completion over a green suite is a survivor: {kept:?}"
    );
}

/// **AC-339: the mutation runs in a disposable worktree** (M12). The suite runs in
/// `.redkiln/worktrees/<item>.mut` over the applied patch; the run-owned tree's `tree_hash` is equal
/// before and after and its `git status --porcelain` is empty; the ephemeral tree is removed and
/// unregistered on every path, a patch that does not apply included.
#[test]
fn ac339_mutation_runs_in_a_disposable_worktree() {
    use rk_kernel::verify::flip::{FlipOutcome, FlipUnverifiable};

    let (repo, tree, head) = mutation_repo();
    let root = repo.path();
    assert_ne!(
        tree.mutation, tree.path,
        "PREMISE: the mutation tree is not the run tree"
    );
    let before = run_tree_hash(root, &tree.path);
    let (evidence, seen, rows) = harness(root, &tree, &head, MUT_PATCH, false);

    assert_eq!(
        seen.len(),
        1,
        "the declared suite ran once: {seen:?} ({evidence:?})"
    );
    assert_eq!(
        folded(&seen[0].0),
        folded(&tree.mutation),
        "the suite did not run in the `<item>.mut` tree"
    );
    assert_eq!(
        seen[0].1, "mutated\n",
        "the patch was not applied in the tree the suite ran in"
    );
    assert_eq!(
        folded(&evidence.tree),
        folded(&tree.mutation),
        "the evidence names another tree"
    );
    assert_eq!(
        run_tree_hash(root, &tree.path),
        before,
        "the run-owned worktree's tree_hash moved across the mutation run (AC-339)"
    );
    let porcelain = git_out(&tree.path, &["status", "--porcelain"]);
    assert!(
        porcelain.trim().is_empty(),
        "a mutation artifact is left in the run-owned worktree: {porcelain}"
    );
    assert_eq!(
        std::fs::read_to_string(tree.path.join("src.txt"))
            .expect("src.txt reads")
            .replace("\r\n", "\n"),
        "original\n",
        "the run-owned worktree's file was mutated"
    );
    assert!(
        std::fs::symlink_metadata(&tree.mutation).is_err(),
        "the mutation tree was not removed: {}",
        tree.mutation.display()
    );
    assert!(
        !listed_worktrees(root).contains(&folded(&tree.mutation)),
        "the mutation tree is still registered: {:?}",
        listed_worktrees(root)
    );
    let origins: Vec<String> = completed(&rows).into_iter().map(|v| v.origin).collect();
    assert_eq!(
        origins,
        ["mutation/AC-1"],
        "the harness's one completed call journals one verify_completed row as the mutation's"
    );

    // Every path: a patch that applies nowhere runs no suite, and the tree is removed all the same.
    let (stale, seen, _) = harness(root, &tree, &head, MUT_PATCH_STALE, false);
    assert_eq!(
        stale.outcome,
        Some(FlipOutcome::Unverifiable(
            FlipUnverifiable::PatchDidNotApply
        )),
        "{stale:?}"
    );
    assert!(seen.is_empty(), "a patch that did not apply ran the suite");
    assert!(
        std::fs::symlink_metadata(&tree.mutation).is_err()
            && !listed_worktrees(root).contains(&folded(&tree.mutation)),
        "the mutation tree outlived a patch that did not apply"
    );
    assert_eq!(run_tree_hash(root, &tree.path), before);
}

// ---------------------------------------------------------------- the AC-357 gate, over the lane

/// The verify fixture with a committed `Spec` whose one criterion declares its mutation
/// infeasible (so the ledger leaves it `not_feasible`, which does not permit the exit), the pack's
/// `[mutation]` declaration (AC-337), and, when `questions`, the SHIPPED `.redkiln/processes/`
/// question file copied in as it is.
fn ac_gate_repo(questions: bool) -> tempfile::TempDir {
    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    let spec = serde_json::json!({
        "acceptance_criteria": [{
            "id": "AC-1",
            "criterion": "the fixture's one criterion holds",
            "verification": "rk_fixture::the_reviewed_case asserts it",
            "mutation": { "kind": "not_feasible", "reason": "the fixture declares no executable mutation" },
        }],
    });
    let dir = root.join(".redkiln").join("artifacts").join(ITEM);
    std::fs::create_dir_all(&dir).expect("the managed directory");
    std::fs::write(
        dir.join("Spec.json"),
        serde_json::to_vec_pretty(&spec).expect("the spec serialises"),
    )
    .expect("the spec writes");
    for pack in ["bug-standard.toml", "story.toml"] {
        let path = root.join(".redkiln").join("packs").join(pack);
        let mut src = std::fs::read_to_string(&path).expect("the pack reads");
        src.push_str("\n[mutation]\nfeasible = true\n");
        std::fs::write(&path, src).expect("the pack writes");
    }
    let mut add = vec!["add", ".redkiln/packs", ".redkiln/artifacts"];
    if questions {
        let shipped =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.redkiln/processes/questions.toml");
        let to = root.join(".redkiln").join("processes");
        std::fs::create_dir_all(&to).expect("the processes directory");
        std::fs::copy(shipped, to.join("questions.toml")).expect("the shipped questions copy");
        add.push(".redkiln/processes");
    }
    crate::git(root, &add);
    crate::git(
        root,
        &["commit", "--quiet", "-m", "the AC-357 fixture's spec"],
    );
    repo
}

/// The gate rows of the AC-357 class, in journal order.
fn ac_gates(root: &Path) -> Vec<serde_json::Value> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| {
            e.payload.get("kind").and_then(serde_json::Value::as_str)
                == Some(rk_kernel::verify::ac_gate::AC_GATE_KIND)
        })
        .map(|e| e.payload)
        .collect()
}

fn rows_of(root: &Path, kind: EventKind) -> Vec<serde_json::Value> {
    support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == kind)
        .map(|e| e.payload)
        .collect()
}

fn answer(root: &Path, option: &str) -> std::process::Output {
    crate::answer::spawn_answer(root, ITEM, option, &crate::answer::BOTH_FLAGS)
}

/// **AC-359: a commit that lands between the gate's open and its answer voids the answer; the gate
/// re-opens with the drift attached, and the AC ledger is recomputed** (AC-357's machine part, the
/// gate node's three payload facts, is exercised here and named; AC-357 itself is a HUMAN GATE and
/// is not claimed).
#[test]
fn ac359_drift_voids_the_answer_and_recomputes_the_ledger() {
    let repo = ac_gate_repo(true);
    let root = repo.path();
    let factory = ScriptedFactory::new(Vec::new());

    // (1) The pass refuses the stage exit and opens the gate.
    assert_eq!(run_verify(root, &factory), Exit::Success);
    let gates = ac_gates(root);
    assert_eq!(
        gates.len(),
        1,
        "the verify pass opened no `verify.ac-discriminates` gate for the unpinned criterion"
    );
    let first = &gates[0];
    for (member, want) in [
        ("ac", "AC-1"),
        ("criterion", "the fixture's one criterion holds"),
        ("assertion", "rk_fixture::the_reviewed_case asserts it"),
    ] {
        assert_eq!(
            first.get(member).and_then(serde_json::Value::as_str),
            Some(want),
            "AC-357's machine part: the gate node's `{member}`: {first}"
        );
    }
    assert!(
        first
            .get("suite_result")
            .is_some_and(serde_json::Value::is_object),
        "AC-357's machine part: the gate node carries the suite result: {first}"
    );
    let refusals = rows_of(root, EventKind::StageExitRefused);
    assert_eq!(
        refusals.len(),
        1,
        "the stage exit refusal is an event (AC-346)"
    );
    assert_eq!(refusals[0]["unpinned"], serde_json::json!(["AC-1"]));
    assert!(
        test_report_writes(root).is_empty(),
        "a refused exit wrote the TestReport"
    );

    // (2) A commit lands in the run-owned tree between the open and the answer.
    let tree = crate::run_tree(root, ITEM).path;
    std::fs::write(tree.join("drift.txt"), b"a commit after the gate opened\n").expect("writes");
    crate::git(&tree, &["add", "drift.txt"]);
    crate::git(
        &tree,
        &["commit", "--quiet", "-m", "drift between open and answer"],
    );

    // (3) The answer is void.
    let out = answer(root, "discriminates");
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let voids = rows_of(root, EventKind::GateAnswerVoid);
    assert_eq!(
        voids.len(),
        1,
        "the drifted answer was not void (AC-359): {voids:?}"
    );
    assert_eq!(voids[0]["token"], first["token"]);

    // (4) The next run recomputes the ledger and re-opens the gate with the drift attached.
    assert_eq!(run_verify(root, &factory), Exit::Success);
    let gates = ac_gates(root);
    assert_eq!(gates.len(), 2, "the void gate did not re-open: {gates:?}");
    let second = &gates[1];
    assert_eq!(second["ac"], "AC-1");
    assert_eq!(second["reach"], 2, "the re-open is one reach on");
    let drift = second
        .get("drift")
        .unwrap_or_else(|| panic!("the re-opened gate carries no drift: {second}"));
    assert_eq!(drift["voided"], first["token"]);
    assert_eq!(drift["bound_tree_hash"], first["tree_hash"]);
    assert_ne!(
        drift["observed_tree_hash"], first["tree_hash"],
        "the drift names no moved tree"
    );
    assert_eq!(
        second["tree_hash"], drift["observed_tree_hash"],
        "the re-opened gate is bound to the tree the answer saw"
    );
    let refusals = rows_of(root, EventKind::StageExitRefused);
    assert_eq!(
        refusals.len(),
        2,
        "the AC ledger was not recomputed after the re-open"
    );
    assert_eq!(
        refusals[1]["statuses"]["AC-1"], "not_feasible",
        "the void answer was carried across as evidence (AC-359)"
    );
    assert!(test_report_writes(root).is_empty());

    // (5) An answer over the tree the re-opened gate saw applies, and the recomputed ledger permits
    // the exit: the TestReport is written, and no third refusal.
    let out = answer(root, "discriminates");
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(rows_of(root, EventKind::GateAnswerVoid).len(), 1);
    assert_eq!(run_verify(root, &factory), Exit::Success);
    assert_eq!(
        rows_of(root, EventKind::StageExitRefused).len(),
        2,
        "the answered criterion was still refused"
    );
    assert_eq!(
        test_report_writes(root).len(),
        1,
        "the permitted exit wrote no TestReport"
    );
}

/// **`verify.ac-discriminates` is declared once in `s8d`'s registry and answered through `answer`'s
/// production path** (`s8g` §8g.6): the binary loads `.redkiln/processes/`, so the non-default
/// option `discriminates` is admitted there, and refused (exit 2, no answer row) where the file is
/// absent.
#[test]
fn verify_ac_discriminates_answers_through_the_loaded_processes_registry() {
    use rk_kernel::verify::ac_gate::{AC_GATE_KIND, DISCRIMINATES, DOES_NOT_DISCRIMINATE};

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let registry = rk_cli::processes::load(&workspace).expect("the shipped registry loads");
    let question = registry
        .question(AC_GATE_KIND)
        .expect("`verify.ac-discriminates` is not declared under .redkiln/processes/");
    assert_eq!(
        question
            .options()
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([DISCRIMINATES, DOES_NOT_DISCRIMINATE]),
    );
    assert_eq!(registry.question_for(AC_GATE_KIND), AC_GATE_KIND);

    let repo = ac_gate_repo(true);
    let root = repo.path();
    assert_eq!(
        run_verify(root, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    assert_eq!(ac_gates(root).len(), 1, "PREMISE: the gate opened");
    let out = answer(root, DISCRIMINATES);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the answer through the loaded registry was refused; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let answered = rows_of(root, EventKind::GateAnswered);
    assert_eq!(answered.len(), 1);
    assert_eq!(answered[0]["verdict"], DISCRIMINATES);
    assert!(rows_of(root, EventKind::GateAnswerVoid).is_empty());

    // CONTROL: the same gate in a repository without the question file keeps the default question,
    // so `discriminates` is a usage error and no answer row is written.
    let bare = ac_gate_repo(false);
    let broot = bare.path();
    assert_eq!(
        run_verify(broot, &ScriptedFactory::new(Vec::new())),
        Exit::Success
    );
    assert_eq!(ac_gates(broot).len(), 1, "PREMISE: the gate opened");
    let out = answer(broot, DISCRIMINATES);
    assert_eq!(
        out.status.code(),
        Some(2),
        "CONTROL: without the question file the option must be refused; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(rows_of(broot, EventKind::GateAnswered).is_empty());
}

// ------------------------------------------------------------------------------------------------
// D21-123 (TAKEN PROVISIONALLY under D21-105; R2 = ADR-0192 decision 2, proposed): mutation
// feasibility ships as a `mutation` declaration row in `.redkiln/registry.toml`'s `[toolchain.rust]`,
// read at step 6 through the adapter the pass ran under, beside the pack's `[mutation]` table; a
// disagreement between the two is refused by name. *(Added 2026-09-29, `s8g` build, package
// R-D2122-D2123-8g.)*
// ------------------------------------------------------------------------------------------------

/// This workspace's root, whose `.redkiln/` is the shipped tree.
fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// A shipped pack's source, read off disk.
fn shipped_pack(name: &str) -> String {
    std::fs::read_to_string(workspace().join(".redkiln").join("packs").join(name))
        .unwrap_or_else(|e| panic!("the shipped pack {name} reads: {e}"))
}

/// The shipped registry's one `[toolchain.rust]` `mutation` row, as a line.
fn shipped_mutation_row(registry: &str) -> &str {
    let rows: Vec<&str> = registry
        .lines()
        .filter(|l| l.starts_with("mutation = "))
        .collect();
    assert_eq!(
        rows.len(),
        1,
        "PREMISE: the shipped registry declares exactly one `mutation` row: {rows:?}"
    );
    rows[0]
}

/// The source tables' files the registry loader reads (`[checklist.*]` and `[kb.*]` paths), copied
/// from the shipped `.redkiln/` under `root`.
fn copy_source_tables(root: &Path) {
    for sub in ["checklists", "kb"] {
        let from = workspace().join(".redkiln").join(sub);
        let to = root.join(".redkiln").join(sub);
        std::fs::create_dir_all(&to).expect("the source directory");
        let mut n = 0_usize;
        for entry in std::fs::read_dir(&from).expect("the shipped source directory reads") {
            let path = entry.expect("an entry").path();
            if path.is_file() {
                std::fs::copy(&path, to.join(path.file_name().expect("a name")))
                    .expect("the source file copies");
                n += 1;
            }
        }
        assert!(n > 0, "PREMISE: the shipped .redkiln/{sub} has files");
    }
}

/// A tree whose `.redkiln/registry.toml` holds `registry`, with the files its source tables name.
fn registry_tree(registry: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("a temp dir");
    std::fs::create_dir_all(dir.path().join(".redkiln")).expect("mkdir .redkiln");
    std::fs::write(rk_cli::verbs::run::registry_path(dir.path()), registry)
        .expect("the registry writes");
    copy_source_tables(dir.path());
    dir
}

/// **The shipped Rust row is read through the pass's adapter** (D21-123 items 1 and 4): over this
/// workspace's own `.redkiln/`, unmodified, both shipped packs (which declare no `[mutation]`) read
/// `feasible` for the Rust adapter.
#[test]
fn ac337_the_shipped_rust_row_is_read_through_the_pass_adapter() {
    for name in ["bug-standard.toml", "story.toml"] {
        let src = shipped_pack(name);
        let pack = rk_kernel::select::Pack::from_toml(name, &src).expect("the shipped pack loads");
        assert_eq!(
            pack.mutation_declaration(),
            None,
            "PREMISE: the shipped {name} declares no `[mutation]`, so the registry row is the only \
             declaration on the shipped tree"
        );
        assert_eq!(
            rk_cli::verbs::run::mutation_feasibility_at(&workspace(), RUST, name, &src),
            Ok(rk_ports::toolchain::MutationFeasibility::Feasible),
            "{name}: the shipped Rust lane's feasibility does not read from the registry row"
        );
    }
}

/// **A Node lane with criteria is refused by name** (D21-123 item 2): `[toolchain.node]` carries no
/// `mutation` row, so over the shipped tree the Node adapter reads nothing and AC-337 refuses.
#[test]
fn ac337_a_node_lane_with_criteria_is_refused_by_name() {
    let registry = std::fs::read_to_string(rk_cli::verbs::run::registry_path(&workspace()))
        .expect("the shipped registry reads");
    let row = shipped_mutation_row(&registry);
    let node_at = registry
        .find("[toolchain.node]")
        .expect("PREMISE: the shipped registry has a [toolchain.node] table");
    assert!(
        registry.find(row).is_some_and(|at| at < node_at),
        "PREMISE: the one `mutation` row is in [toolchain.rust], above [toolchain.node]"
    );
    for name in ["bug-standard.toml", "story.toml"] {
        let refused = rk_cli::verbs::run::mutation_feasibility_at(
            &workspace(),
            NODE,
            name,
            &shipped_pack(name),
        );
        match &refused {
            Err(why) => assert!(
                why.contains("AC-337"),
                "{name}: the Node refusal does not name AC-337: {why}"
            ),
            Ok(f) => {
                panic!("{name}: a Node lane read a mutation feasibility it does not have: {f:?}")
            }
        }
    }
    // CONTROL: the same read for the Rust adapter over the same tree is not refused.
    assert!(
        rk_cli::verbs::run::mutation_feasibility_at(
            &workspace(),
            RUST,
            "story.toml",
            &shipped_pack("story.toml")
        )
        .is_ok(),
        "CONTROL: the Rust adapter reads the row"
    );
}

/// **Declared twice and disagreeing is refused by name** (D21-123 item 4's table): the registry row
/// and the pack's `[mutation]` table disagree, and no precedence picks a winner. Agreeing, or either
/// alone, is read.
#[test]
fn ac337_a_registry_and_pack_disagreement_is_refused_by_name() {
    let registry = std::fs::read_to_string(rk_cli::verbs::run::registry_path(&workspace()))
        .expect("the shipped registry reads");
    let tree = registry_tree(&registry);
    let story = shipped_pack("story.toml");
    let infeasible = format!("{story}\n[mutation]\nfeasible = false\nreason = \"the fixture\"\n");
    let feasible = format!("{story}\n[mutation]\nfeasible = true\n");
    let read = |root: &Path, src: &str| {
        rk_cli::verbs::run::mutation_feasibility_at(root, RUST, "story.toml", src)
    };

    match read(tree.path(), &infeasible) {
        Err(why) => assert!(
            why.contains("declared twice and disagrees")
                && why.contains("[toolchain.rust].mutation")
                && why.contains("[mutation]"),
            "the disagreement is not refused by name: {why}"
        ),
        Ok(f) => panic!("a disagreement between the registry and the pack was resolved to {f:?}"),
    }
    assert_eq!(
        read(tree.path(), &feasible),
        Ok(rk_ports::toolchain::MutationFeasibility::Feasible),
        "both present and equal: that value"
    );
    let bare = tempfile::tempdir().expect("a tree with no registry");
    assert_eq!(
        read(bare.path(), &infeasible),
        Ok(rk_ports::toolchain::MutationFeasibility::NotFeasible {
            reason: "the fixture".to_owned()
        }),
        "the pack alone: its value"
    );
    assert_eq!(
        read(tree.path(), &story),
        Ok(rk_ports::toolchain::MutationFeasibility::Feasible),
        "the registry alone: its value"
    );
}

/// The AC-357 fixture's criterion (declared `not_feasible`, so the ledger holds the exit) over the
/// verify fixture, with NO `[mutation]` added to its packs, and `.redkiln/registry.toml` copied from
/// the shipped one. One row is removed from the copy, asserted to hit once: `[toolchain.rust]`'s
/// `setup_projects`, which would install a project this fixture does not have at tree creation.
/// When `with_row` is false the `mutation` row is removed too (the CONTROL).
fn shipped_registry_repo(with_row: bool) -> tempfile::TempDir {
    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    let spec = serde_json::json!({
        "acceptance_criteria": [{
            "id": "AC-1",
            "criterion": "the fixture's one criterion holds",
            "verification": "rk_fixture::the_reviewed_case asserts it",
            "mutation": { "kind": "not_feasible", "reason": "the fixture declares no executable mutation" },
        }],
    });
    let dir = root.join(".redkiln").join("artifacts").join(ITEM);
    std::fs::create_dir_all(&dir).expect("the managed directory");
    std::fs::write(
        dir.join("Spec.json"),
        serde_json::to_vec_pretty(&spec).expect("the spec serialises"),
    )
    .expect("the spec writes");
    let shipped = std::fs::read_to_string(rk_cli::verbs::run::registry_path(&workspace()))
        .expect("the shipped registry reads");
    let mut drop = vec!["setup_projects = "];
    if !with_row {
        drop.push("mutation = ");
    }
    let mut copy = shipped.clone();
    for prefix in drop {
        let hits: Vec<&str> = shipped.lines().filter(|l| l.starts_with(prefix)).collect();
        assert_eq!(
            hits.len(),
            1,
            "PREMISE: the shipped registry has one `{prefix}` row: {hits:?}"
        );
        copy = copy.replacen(&format!("{}\n", hits[0]), "", 1);
    }
    std::fs::write(rk_cli::verbs::run::registry_path(root), copy).expect("the registry copies");
    copy_source_tables(root);
    crate::git(
        root,
        &[
            "add",
            ".redkiln/artifacts",
            ".redkiln/registry.toml",
            ".redkiln/checklists",
            ".redkiln/kb",
        ],
    );
    crate::git(
        root,
        &[
            "commit",
            "--quiet",
            "-m",
            "the D21-123 fixture's spec and registry",
        ],
    );
    repo
}

/// **A shipped Rust lane with criteria is not refused for feasibility** (D21-123 item 5): the
/// verify fixture's packs declare no `[mutation]`, the registry carries the shipped Rust row, and
/// the pass reaches the AC ledger and the stage-exit check (AC-346), which refuses the unpinned
/// criterion and opens AC-357's gate. Without the row (the CONTROL) the same lane is AC-337's
/// configuration error, and neither the ledger nor the gate is reached.
#[test]
fn ac337_a_shipped_rust_lane_with_criteria_reaches_the_ledger() {
    let repo = shipped_registry_repo(true);
    let root = repo.path();
    let factory = ScriptedFactory::new(Vec::new());
    assert_eq!(
        run_verify(root, &factory),
        Exit::Success,
        "the lane was refused before the ledger (AC-337's configuration error)"
    );
    let refusals = rows_of(root, EventKind::StageExitRefused);
    assert_eq!(
        refusals.len(),
        1,
        "the ledger was not reached: no stage-exit refusal (AC-346)"
    );
    assert_eq!(refusals[0]["unpinned"], serde_json::json!(["AC-1"]));
    assert_eq!(ac_gates(root).len(), 1, "AC-357's gate was not opened");
    assert!(
        test_report_writes(root).is_empty(),
        "the row read as the criteria lane passing: a TestReport was written"
    );

    // CONTROL: the row removed from the copy.
    let bare = shipped_registry_repo(false);
    let broot = bare.path();
    assert_eq!(
        run_verify(broot, &ScriptedFactory::new(Vec::new())),
        Exit::ConfigError,
        "CONTROL: without the row the lane must be AC-337's configuration error"
    );
    assert!(rows_of(broot, EventKind::StageExitRefused).is_empty());
    assert!(ac_gates(broot).is_empty());
}

// ---------------------------------------------------------------- AC-353: kill and resume

/// `redkiln -C <root> run verify-1`, the real binary, with `rk-fixture-toolchain` as the Rust
/// adapter's program (the fixture's configuration).
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS: AC-353 kills the orchestrating process tree"
)]
fn redkiln_run(root: &Path) -> std::process::Command {
    let markers = root.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(root).arg("run").arg(ITEM);
    cmd.env("RK_FIXTURE_MODE", "exit:0")
        .env("RK_FIXTURE_MARKER_DIR", &markers);
    cmd
}

/// The pass's rows, the TestReport writes and the exit: what "the same decision" compares.
fn decision(root: &Path, code: Option<i32>) -> (Option<i32>, usize, Vec<(u32, u32)>) {
    (
        code,
        test_report_writes(root).len(),
        pass_rows(root)
            .iter()
            .map(|v| (v.passed, v.failed))
            .collect(),
    )
}

/// **AC-353: a verify killed in progress resumes to the same decision.** A real `redkiln` over the
/// verify fixture, whose `rk-fixture-toolchain` blocks its one run vector (once), is killed through
/// `rk-containment` while the suite is in progress; the next `run` reaches the decision an
/// uninterrupted run reaches, `reconcile` ran on its entry, the changed-file set it records is
/// re-derived from the world (a commit made after the kill is in it), and AC-321's replay reads the
/// rows alone.
#[test]
fn ac353_killed_verify_resumes_to_the_same_decision() {
    // The uninterrupted decision.
    let control = verify_repo(&[RUST], true);
    let croot = control.path();
    let out = support::output_bounded(&mut redkiln_run(croot));
    let want = decision(croot, out.status.code());
    assert_eq!(
        want,
        (Some(0), 1, vec![(1, 0)]),
        "PREMISE: the uninterrupted verify is green and writes its TestReport; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // The killed run.
    let repo = verify_repo(&[RUST], true);
    let root = repo.path();
    let common = root.join(".git");
    std::fs::write(common.join("rk-fixture-block-once"), b"block\n").expect("the block arms");
    let mut cmd = redkiln_run(root);
    cmd.stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    let mut contained = rk_containment::spawn_into_job(cmd).expect("redkiln spawns into a job");
    let mut blocked = false;
    for _ in 0..1200 {
        if common.join("rk-fixture-blocked").is_file() {
            blocked = true;
            break;
        }
        if contained.child.try_wait().ok().flatten().is_some() {
            break;
        }
        std::thread::park_timeout(std::time::Duration::from_millis(100));
    }
    if !blocked {
        let _ = contained.group.kill();
        let _ = contained.child.wait();
        panic!("PREMISE: the verify never reached the fixture's blocking run vector");
    }
    contained
        .group
        .kill()
        .expect("the orchestrating process tree is killed through rk-containment");
    let _ = contained.child.wait();
    assert!(
        test_report_writes(root).is_empty() && pass_rows(root).is_empty(),
        "PREMISE: the killed run completed nothing"
    );
    let entries_before = rows_of(root, EventKind::ReconcileStarted).len();

    // The world moves while nothing runs: a commit on the run branch.
    let tree = crate::run_tree(root, ITEM).path;
    std::fs::write(tree.join("after-kill.txt"), b"committed after the kill\n").expect("writes");
    crate::git(&tree, &["add", "after-kill.txt"]);
    crate::git(&tree, &["commit", "--quiet", "-m", "after the kill"]);

    // Resume.
    let out = support::output_bounded(&mut redkiln_run(root));
    assert_eq!(
        decision(root, out.status.code()),
        want,
        "the resumed verify did not reach the uninterrupted decision; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rows = support::store_rows(root);
    let entries: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::ReconcileStarted)
        .map(|(i, _)| i)
        .collect();
    let pass_at = rows
        .iter()
        .position(|e| {
            e.kind == EventKind::VerifyCompleted
                && e.payload.get("origin").and_then(serde_json::Value::as_str) == Some("pass")
        })
        .expect("the resumed pass journalled its row");
    assert!(
        entries.len() > entries_before && entries.iter().any(|i| *i < pass_at),
        "reconcile did not run on the resumed entry before the pass (rows {entries:?}, pass at \
         {pass_at})"
    );
    let resumed = pass_rows(root);
    assert!(
        resumed[0].paths.iter().any(|p| p == "after-kill.txt"),
        "the resumed verify did not re-derive the changed-file set from the world: {:?}",
        resumed[0].paths
    );

    // Replay reads the rows alone: a later world does not move it.
    std::fs::write(root.join("after-resume.txt"), b"later\n").expect("writes");
    crate::git(root, &["add", "after-resume.txt"]);
    crate::git(root, &["commit", "--quiet", "-m", "after the resume"]);
    let replayed = completed(&support::store_rows(root));
    let replayed: Vec<&VerifyCompleted> = replayed.iter().filter(|v| v.origin == "pass").collect();
    assert_eq!(replayed[0].paths, resumed[0].paths);
}

#[expect(
    clippy::disallowed_types,
    reason = "reading the fixture repository with git; the product's git goes through rk-git's \
              one GitExec wrapper (ADR-0077)"
)]
fn git_out(cwd: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?} in {} failed: {}",
        cwd.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}
