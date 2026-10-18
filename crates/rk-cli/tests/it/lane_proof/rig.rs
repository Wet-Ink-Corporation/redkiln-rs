//! **The lane proofs' rig** (`s8j` §8j.4): the fixture repository, the scripted harness with no
//! default response, the gate driver, and the git and store readers the proofs assert over.
//!
//! Nothing here asserts a product claim; each helper asserts only its own PREMISES (the copy
//! arrived, the fixture materialised, `init` exited 0), so a proof that fails on the rig names the
//! rig.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_cli::exit::Exit;
use rk_cli::verbs::{HarnessFactory, Startup};
use rk_ports::cancel::CancelToken;
use rk_ports::event_store::EventStore;
use rk_ports::fakes::FakeHarness;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::Typed;

/// `.redkiln/config.toml`'s `[project] name`, as the byte copy carries it (ADR-0159).
pub(crate) const PROJECT: &str = "redkiln-rs";
/// The fix-lane item the fixture writes: `kind = "bug"`, the real fix item's facets, a `slice`,
/// `hold_before = "specification"` and `[trial] activity = "tag-release"` (§8j.1(e)).
pub(crate) const FIX_ITEM: &str = "rk-lane-fix";
/// The story-lane item the fixture writes: `kind = "story"`, a `slice` and the same `[trial]`.
pub(crate) const STORY_ITEM: &str = "rk-lane-story";

/// The fixture crate's path inside the repository.
pub(crate) const CRATE_DIR: &str = "crates/rk-lane-fixture";

// ------------------------------------------------------------------------------- the fixture

/// The workspace root this test binary was built from: the tree whose tracked `.redkiln/` is copied.
pub(crate) fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// `crates/rk-cli/tests/fixtures/lane/`, the fixture workspace stored as `*.in` data.
fn fixture_src() -> PathBuf {
    crate::fixtures_dir().join("lane")
}

/// Every file under `dir`, as a path relative to it, sorted.
fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(at) = stack.pop() {
        for entry in
            std::fs::read_dir(&at).unwrap_or_else(|e| panic!("{} reads: {e}", at.display()))
        {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(
                    path.strip_prefix(dir)
                        .expect("under the walked directory")
                        .to_path_buf(),
                );
            }
        }
    }
    out.sort();
    out
}

/// The fixture file's name in the materialised tree: `.in` dropped, `gitignore` dotted.
fn materialised_name(rel: &Path) -> PathBuf {
    let name = rel
        .file_name()
        .and_then(|n| n.to_str())
        .expect("a UTF-8 fixture file name");
    let stem = name
        .strip_suffix(".in")
        .unwrap_or_else(|| panic!("the fixture file {} is not stored as `*.in`", rel.display()));
    let stem = if stem == "gitignore" {
        ".gitignore"
    } else {
        stem
    };
    rel.with_file_name(stem)
}

/// Write the fixture workspace into `root`, every `*.in` renamed (judgement call J1: no manifest is
/// checked in under the repository).
fn materialise_crate(root: &Path) {
    let src = fixture_src();
    let files = files_under(&src);
    assert!(
        files.len() >= 7,
        "PREMISE: the lane fixture under {} holds its seven files; found {files:?}",
        src.display()
    );
    for rel in files {
        let to = root.join(materialised_name(&rel));
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent).expect("the fixture directory is created");
        }
        std::fs::copy(src.join(&rel), &to)
            .unwrap_or_else(|e| panic!("{} copies: {e}", rel.display()));
    }
}

/// Byte-copy every file `git ls-files` lists under `pathspec` in the workspace into `root`, skipping
/// any whose path `skip` names; returns how many were copied.
fn copy_tracked(root: &Path, pathspec: &str, skip: impl Fn(&Path) -> bool) -> usize {
    let ws = workspace_root();
    let listed = crate::support::git_ok(&ws, &["ls-files", "-z", "--", pathspec]);
    let mut copied = 0_usize;
    for rel in listed
        .stdout
        .split(|b| *b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| PathBuf::from(String::from_utf8_lossy(p).into_owned()))
    {
        if skip(&rel) {
            continue;
        }
        let to = root.join(&rel);
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent).expect("the copy's directory is created");
        }
        let bytes =
            std::fs::read(ws.join(&rel)).unwrap_or_else(|e| panic!("{} reads: {e}", rel.display()));
        std::fs::write(&to, bytes).unwrap_or_else(|e| panic!("{} writes: {e}", rel.display()));
        copied += 1;
    }
    copied
}

/// **The byte copy of the tracked `.redkiln/`, minus `worktrees/`** (§8j.4; review finding 25;
/// D21-44). Every file `git ls-files` lists under the workspace's `.redkiln/`, copied byte for byte;
/// `registry.toml` is asserted to have arrived, and no `worktrees` directory exists afterwards.
fn copy_tracked_redkiln(root: &Path) {
    let copied = copy_tracked(root, ".redkiln", |rel| {
        rel.components()
            .nth(1)
            .is_some_and(|c| c.as_os_str() == "worktrees")
    });
    assert!(
        root.join(".redkiln").join("registry.toml").is_file(),
        "PREMISE: the byte copy carries `.redkiln/registry.toml`, s8c's verb table (review \
         finding 25); copied {copied} files"
    );
    assert!(
        !root.join(".redkiln").join("worktrees").exists(),
        "the byte copy must skip `.redkiln/worktrees/` (D21-44)"
    );
}

/// **The projects the copied registry's `setup_projects` names, byte-copied from the workspace**
/// (judgement call J3). Run-tree creation runs `setup` over each (ADR-0170 decision 7), so a fixture
/// without them is refused at creation; the paths are read from the copy, never typed here.
fn copy_setup_projects(root: &Path) {
    let src = std::fs::read_to_string(root.join(".redkiln").join("registry.toml"))
        .expect("the copied registry reads");
    let registry: toml::Value = toml::from_str(&src).expect("the copied registry parses");
    let Some(tables) = registry.get("toolchain").and_then(toml::Value::as_table) else {
        return;
    };
    for (name, table) in tables {
        let projects = table
            .get("setup_projects")
            .and_then(|row| row.get("value"))
            .and_then(toml::Value::as_array)
            .cloned()
            .unwrap_or_default();
        for project in projects {
            let path = project
                .as_str()
                .unwrap_or_else(|| panic!("`toolchain.{name}.setup_projects` holds a path"));
            let n = copy_tracked(root, path, |_| false);
            assert!(n > 0, "PREMISE: the setup project {path} has tracked files");
        }
    }
}

/// The two lane items and their human roots (§8j.4's last bullet; §8j.2's intake convention).
fn write_items(root: &Path) {
    let items = root.join(".redkiln").join("items");
    std::fs::create_dir_all(&items).expect("the items directory exists");
    std::fs::write(
        items.join(format!("{FIX_ITEM}.toml")),
        format!(
            "# The lane proofs' fix item (s8j section 8j.4): the real fix item's facets.\n\
             id = \"{FIX_ITEM}\"\nkind = \"bug\"\nstage = \"triage\"\nslice = \"{FIX_ITEM}-fix\"\n\
             hold_before = \"specification\"\n\n[facets]\ncos = \"standard\"\nhas_repro = true\n\
             criticality = \"low\"\nblast_radius = \"low\"\nsize = \"low\"\n\n[trial]\n\
             activity = \"tag-release\"\n"
        ),
    )
    .expect("the fix item writes");
    std::fs::write(
        items.join(format!("{STORY_ITEM}.toml")),
        format!(
            "# The lane proofs' story item (s8j section 8j.4).\n\
             id = \"{STORY_ITEM}\"\nkind = \"story\"\nstage = \"triage\"\nslice = \"{STORY_ITEM}-story\"\n\n\
             [facets]\ncos = \"standard\"\ncriticality = \"low\"\nblast_radius = \"low\"\n\
             size = \"low\"\nnovelty = \"low\"\nexternal_dep = false\n\n[trial]\n\
             activity = \"tag-release\"\n"
        ),
    )
    .expect("the story item writes");
    let intake = root.join(".redkiln").join("intake");
    std::fs::create_dir_all(&intake).expect("the intake directory exists");
    std::fs::write(
        intake.join(format!("{FIX_ITEM}.BugReport.md")),
        "`word_count(\"\")` returns 1; an empty text has no words.\n",
    )
    .expect("the bug report writes");
    std::fs::write(
        intake.join(format!("{STORY_ITEM}.IntakeRecord.md")),
        "Count words separated by any run of whitespace.\n",
    )
    .expect("the intake record writes");
}

/// The path of an item's human-root file, relative to the repository root.
pub(crate) fn root_file(item: &str, kind: &str) -> PathBuf {
    PathBuf::from(".redkiln")
        .join("intake")
        .join(format!("{item}.{kind}.md"))
}

/// **The fixture repository** (§8j.4): a fresh git repository holding the fixture workspace, the
/// byte copy of the tracked `.redkiln/` and the two lane items, committed, then initialised through
/// the product's own `init` (so `info/exclude` carries the `# redkiln-runtime` block).
pub(crate) fn fixture() -> tempfile::TempDir {
    let repo = crate::temp_repo();
    let root = repo.path();
    materialise_crate(root);
    copy_tracked_redkiln(root);
    copy_setup_projects(root);
    write_items(root);
    crate::git(root, &["add", "-A"]);
    crate::git(root, &["commit", "--quiet", "-m", "the lane fixture"]);
    let init = crate::run_init(root);
    assert_eq!(
        init.status.code(),
        Some(0),
        "PREMISE: `redkiln init` initialises the fixture. stderr:\n{}",
        String::from_utf8_lossy(&init.stderr)
    );
    let exclude = std::fs::read_to_string(crate::common_dir(root).join("info").join("exclude"))
        .expect("info/exclude reads");
    assert!(
        exclude
            .lines()
            .any(|l| l == rk_cli::verbs::init::EXCLUDE_BEGIN),
        "PREMISE: `init` wrote its `# redkiln-runtime` block (D21-45 W4)"
    );
    lay_advisory_databases(root);
    repo
}

/// The fixture's `CARGO_HOME`: inside the repository's common directory, so it is outside the
/// checkout `git status` reads (AC-518) and is removed with the fixture.
pub(crate) fn cargo_home(root: &Path) -> PathBuf {
    crate::common_dir(root).join("lane-proof-cargo-home")
}

/// **The two advisory databases `sca_scan` reads, at the tools' default locations under the
/// fixture's `CARGO_HOME`** (judgement call J5): `rk-adapter-toolchain-rust`'s scan fixture
/// database, git-initialised twice, as `<CARGO_HOME>/advisory-db` (`cargo audit --db`) and
/// `<CARGO_HOME>/advisory-dbs/<deny's child>` (`cargo deny`'s `db-path`), the paths `rk-cli`'s
/// `advisory_db_row_inputs` binds. The fixture crate has no dependency, so the scan is clean.
fn lay_advisory_databases(root: &Path) {
    let src = workspace_root()
        .join("crates")
        .join("rk-adapter-toolchain-rust")
        .join("tests")
        .join("fixtures")
        .join("advisory-db")
        .join("db");
    let home = cargo_home(root);
    for to in [
        home.join("advisory-db"),
        home.join("advisory-dbs")
            .join("advisory-db-3157b0e258782691"),
    ] {
        for rel in files_under(&src) {
            let dest = to.join(&rel);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).expect("the database directory is created");
            }
            std::fs::copy(src.join(&rel), &dest).expect("the database file copies");
        }
        crate::git(&to, &["init", "--quiet"]);
        crate::git(&to, &["add", "-A"]);
        crate::git(
            &to,
            &[
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "user.name=Test",
                "commit",
                "--quiet",
                "-m",
                "the scan fixture database",
            ],
        );
    }
}

/// The environment the in-process proofs hand `run`: built key by key from this process, never
/// `Env::capture()` and never a provider name (issue #11). The real Rust adapter's `cargo` needs a
/// `PATH`, a home and rustup's toolchain selection; `CARGO_HOME` is the fixture's own.
pub(crate) fn lane_env(root: &Path) -> rk_cli::env::Env {
    let mut map = BTreeMap::from([(
        "CARGO_HOME".to_owned(),
        cargo_home(root).display().to_string(),
    )]);
    for key in [
        "PATH",
        "Path",
        "PATHEXT",
        "HOME",
        "USERPROFILE",
        "RUSTUP_HOME",
        "RUSTUP_TOOLCHAIN",
        "SystemRoot",
        "SystemDrive",
        "TMPDIR",
        "TMP",
        "TEMP",
    ] {
        if let Some(v) = std::env::var_os(key).and_then(|v| v.into_string().ok()) {
            map.insert(key.to_owned(), v);
        }
    }
    map.insert(
        rk_cli::env::LAUNCH_RECORD.to_owned(),
        crate::fixture_launch_record().display().to_string(),
    );
    rk_cli::env::Env::from_map(map)
}

// ------------------------------------------------------------------------- the scripted harness

/// A scripted response's key: `(activity@version, step id, visit)`. The visit is the ordinal of
/// this dispatch among the item's `dispatch` rows for that activity and step, read from the
/// journal (judgement call J2): the harness cannot see the iteration path at dispatch, and every
/// committed row's `iteration_path` is read back afterwards ([`committed_paths`]).
pub(crate) type Key = (String, String, usize);

/// One scripted response: the typed value, and the files the double writes inside the run-owned
/// worktree before it answers (a failing test at `red`, the fix at `green`).
pub(crate) struct Response {
    value: serde_json::Value,
    writes: Vec<(String, String)>,
}

impl Response {
    /// A value of an `rk-types` lane kind, serialised: so it validates against the `EXPORTED`
    /// schema `s8h` produced by construction.
    pub(crate) fn of<T: serde::Serialize>(value: &T) -> Self {
        Self {
            value: serde_json::to_value(value).expect("an rk-types value serialises"),
            writes: Vec::new(),
        }
    }

    /// The same response, writing `contents` to `path` (relative to the run-owned worktree) first.
    pub(crate) fn writing(mut self, path: &str, contents: &str) -> Self {
        self.writes.push((path.to_owned(), contents.to_owned()));
        self
    }
}

/// What the harness saw: the visited keys in order, and every dispatch no response was scripted for.
#[derive(Default)]
struct Seen {
    visited: Vec<Key>,
    unscripted: Vec<Key>,
}

/// **The scripted harness**: a response per key, **no default response**. An unscripted dispatch
/// returns `StepError::Unsupported` naming its key, and is recorded, so the proof fails by name.
pub(crate) struct ScriptedLane {
    item: String,
    tree: PathBuf,
    // A `Mutex` because `Harness::dispatch` takes `&self` and one harness serves every stage's
    // build through an `Arc` (OWN-11: the script is consumed, so it is shared AND mutable).
    script: Mutex<BTreeMap<Key, Response>>,
    seen: Mutex<Seen>,
    catalog: BTreeMap<String, Vec<AgentStep>>,
}

impl ScriptedLane {
    /// A harness for `item` over `root`'s run-owned worktree.
    pub(crate) fn new(root: &Path, item: &str, script: BTreeMap<Key, Response>) -> Arc<Self> {
        Arc::new(Self {
            item: item.to_owned(),
            tree: crate::run_tree(root, item).path,
            script: Mutex::new(script),
            seen: Mutex::new(Seen::default()),
            catalog: agent_steps(root),
        })
    }

    /// Every key dispatched with no response scripted.
    pub(crate) fn unscripted(&self) -> Vec<Key> {
        self.seen.lock().expect("unpoisoned").unscripted.clone()
    }

    /// Every key dispatched, in order.
    pub(crate) fn visited(&self) -> Vec<Key> {
        self.seen.lock().expect("unpoisoned").visited.clone()
    }

    /// Every scripted key no dispatch reached.
    pub(crate) fn unconsumed(&self) -> Vec<Key> {
        self.script
            .lock()
            .expect("unpoisoned")
            .keys()
            .cloned()
            .collect()
    }

    /// This dispatch's key. The activity is the `dispatch` row's (the kernel appends it before the
    /// port); the row's `step_id` is the DAG node's, so the STEP is the one agent step of that
    /// activity whose declared output kind is the spec's schema and whose declared input names are
    /// the spec's resolved inputs (read from the copied catalog). The visit is how many rows of
    /// that step the item has COMMITTED, so a retry keeps its visit and a loop-back takes the next.
    fn key_of(&self, spec: &ActivitySpec, store: &dyn EventStore) -> Key {
        let rows = store
            .query(rk_kernel::runtime::journal_query(&self.item))
            .expect("the journal reads inside a dispatch");
        let str_of = |e: &Event, member: &str| {
            e.payload
                .get(member)
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_owned()
        };
        let activity = rows
            .iter()
            .rev()
            .find(|e| e.kind == EventKind::Dispatch)
            .map(|e| str_of(e, "activity_id_at_version"))
            .expect("the kernel appends a `dispatch` row before the port");
        let names: std::collections::BTreeSet<&str> = spec
            .inputs
            .iter()
            .map(|(name, _)| name.as_str())
            // The binder's own keys (`item`, `worktree`), which no catalog step declares.
            .filter(|name| !matches!(*name, "item" | "worktree"))
            .collect();
        let candidates: Vec<&AgentStep> = self
            .catalog
            .get(&activity)
            .map(|steps| {
                steps
                    .iter()
                    .filter(|s| s.kind == spec.schema.as_str())
                    .filter(|s| {
                        s.inputs
                            .iter()
                            .map(String::as_str)
                            .collect::<std::collections::BTreeSet<_>>()
                            == names
                    })
                    .collect()
            })
            .unwrap_or_default();
        let step = match candidates.as_slice() {
            [one] => one.id.clone(),
            _ => format!(
                "<{} steps of {activity} match {} {names:?}>",
                candidates.len(),
                spec.schema.as_str()
            ),
        };
        let visit = rows
            .iter()
            .filter(|e| e.kind == EventKind::ActivityCommitted)
            .filter(|e| {
                str_of(e, "activity_id_at_version") == activity && str_of(e, "step_id") == step
            })
            .count();
        (activity, step, visit)
    }
}

/// One agent step of a catalog activity: its id, its `value:<Kind>` output, its input names.
struct AgentStep {
    id: String,
    kind: String,
    inputs: Vec<String>,
}

/// Every activity's agent steps, by `<id>@<version>`, read from the fixture's catalog and
/// extended catalog.
fn agent_steps(root: &Path) -> BTreeMap<String, Vec<AgentStep>> {
    let mut out = BTreeMap::new();
    for dir in ["catalog", rk_kernel::catalog::EXTENDED_DIR] {
        let Ok(entries) = std::fs::read_dir(root.join(".redkiln").join(dir)) else {
            continue;
        };
        for path in entries.filter_map(|e| e.ok().map(|e| e.path())) {
            if path.extension().is_none_or(|x| x != "toml") {
                continue;
            }
            let src = std::fs::read_to_string(&path).expect("a catalog file reads");
            let parsed: toml::Value = toml::from_str(&src).expect("a catalog file parses");
            let Some(activity) = parsed.get("activity") else {
                continue;
            };
            let id = format!(
                "{}@{}",
                activity
                    .get("id")
                    .and_then(toml::Value::as_str)
                    .unwrap_or_default(),
                activity
                    .get("version")
                    .and_then(toml::Value::as_integer)
                    .unwrap_or_default()
            );
            let steps = parsed
                .get("step")
                .and_then(toml::Value::as_array)
                .map(|steps| {
                    steps
                        .iter()
                        .filter(|s| s.get("kind").and_then(toml::Value::as_str) == Some("agent"))
                        .map(|s| AgentStep {
                            id: s
                                .get("id")
                                .and_then(toml::Value::as_str)
                                .unwrap_or_default()
                                .to_owned(),
                            kind: s
                                .get("outputs")
                                .and_then(toml::Value::as_str)
                                .and_then(|o| o.strip_prefix("value:"))
                                .unwrap_or_default()
                                .to_owned(),
                            inputs: s
                                .get("inputs")
                                .and_then(toml::Value::as_table)
                                .map(|t| t.keys().cloned().collect())
                                .unwrap_or_default(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            out.insert(id, steps);
        }
    }
    out
}

impl Harness for ScriptedLane {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        let key = self.key_of(spec, store);
        let response = self.script.lock().expect("unpoisoned").remove(&key);
        let result = {
            let mut seen = self.seen.lock().expect("unpoisoned");
            seen.visited.push(key.clone());
            match response {
                Some(r) => {
                    for (path, contents) in &r.writes {
                        let to = self.tree.join(path);
                        if let Some(parent) = to.parent() {
                            std::fs::create_dir_all(parent).expect("the write's directory");
                        }
                        std::fs::write(&to, contents).expect("the double's write lands");
                    }
                    Ok(Typed::new(spec.schema.clone(), r.value))
                }
                None => {
                    seen.unscripted.push(key.clone());
                    Err(StepError::Unsupported {
                        capability: format!("LANE-PROOF: no scripted response for {key:?}"),
                    })
                }
            }
        };
        FakeHarness::scripted(vec![result]).dispatch(spec, store)
    }
}

/// The test side of `Startup::harness`: every stage of every invocation gets the one harness.
pub(crate) struct Factory(pub(crate) Arc<ScriptedLane>);

impl HarnessFactory for Factory {
    fn build(
        &self,
        _config: rk_adapter_claude::config::Config,
        _token: CancelToken,
    ) -> Result<Arc<dyn Harness>, String> {
        Ok(Arc::clone(&self.0) as Arc<dyn Harness>)
    }

    fn builds_the_real_harness(&self) -> bool {
        false
    }
}

// ------------------------------------------------------------------------------- the driver

/// One `run <item>` in process through `rk_cli::verbs::run::run`.
pub(crate) fn run_once(root: &Path, item: &str, harness: &Arc<ScriptedLane>) -> Exit {
    let factory = Factory(Arc::clone(harness));
    rk_cli::verbs::run::run(
        &crate::lane_e2e::cli(root, item),
        &crate::lane_e2e::args(item),
        &lane_env(root),
        Startup {
            harness: &factory,
            ..Startup::real()
        },
        &CancelToken::new(),
    )
    .unwrap_or_else(|halt| panic!("`run {item}` halted: {halt:?}"))
}

/// The gate the item's journal awaits, as `(token, kind)`, or `None`.
pub(crate) fn awaited_gate(root: &Path, item: &str) -> Option<(String, String)> {
    let rows = crate::support::journal_rows(root, item);
    let stage = story_stages(root, item)
        .last()
        .cloned()
        .unwrap_or_else(|| "triage".to_owned());
    let state = rk_kernel::workflow::fsm::fold(item, &stage, &rows).ok()?;
    let token = state.awaiting_gate()?.as_str().to_owned();
    let kind = rows
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .find(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(&token))
        .and_then(|e| e.payload.get("kind").and_then(serde_json::Value::as_str))
        .unwrap_or_default()
        .to_owned();
    Some((token, kind))
}

/// The gate's question's FIRST option (as `s8d`'s walk answers), with the item's root file attached
/// when that option attaches a kind.
pub(crate) fn first_option(
    root: &Path,
    item: &str,
    kind: &str,
) -> (String, Vec<rk_cli::cli::Attachment>) {
    let registry = rk_cli::processes::load(root).expect("the fixture's questions load");
    let question_id = registry.question_for(kind).to_owned();
    let question = registry.question(&question_id).unwrap_or_else(|| {
        panic!("the gate kind `{kind}` resolves to no question `{question_id}`")
    });
    // The DECLARED first option: `Question::options` is sorted, so the order is read from the
    // question's own `options` array (the built-in default's is `DEFAULT_OPTIONS`).
    let option = declared_first_option(root, &question_id).unwrap_or_else(|| {
        assert_eq!(
            question_id,
            rk_kernel::workflow::question::DEFAULT_QUESTION,
            "the question `{question_id}` declares no first option"
        );
        rk_kernel::workflow::question::DEFAULT_OPTIONS[0].to_owned()
    });
    let attach = question
        .attaches(&option)
        .unwrap_or_default()
        .iter()
        .map(|k| rk_cli::cli::Attachment {
            kind: k.clone(),
            path: root.join(root_file(item, k)),
        })
        .collect();
    (option, attach)
}

/// The first entry of `[question."<id>"] options` in any `.redkiln/processes/*.toml`, in file order:
/// a bare string, or an option table's `id`.
fn declared_first_option(root: &Path, question_id: &str) -> Option<String> {
    let dir = rk_cli::processes::processes_dir(root);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    files.iter().find_map(|path| {
        let src = std::fs::read_to_string(path).ok()?;
        let parsed: toml::Value = toml::from_str(&src).ok()?;
        let first = parsed
            .get("question")?
            .get(question_id)?
            .get("options")?
            .as_array()?
            .first()?
            .clone();
        match first {
            toml::Value::String(s) => Some(s),
            toml::Value::Table(t) => t.get("id")?.as_str().map(str::to_owned),
            _ => None,
        }
    })
}

/// `answer <item> <option> [--attach ..] --explain-sufficed yes --toml-opened no`, in process.
pub(crate) fn answer(
    root: &Path,
    item: &str,
    option: &str,
    attach: Vec<rk_cli::cli::Attachment>,
) -> Exit {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::AnswerArgs {
        item: item.to_owned(),
        option: option.to_owned(),
        attach,
        explain_sufficed: rk_cli::cli::YesNo::Yes,
        toml_opened: rk_cli::cli::YesNo::No,
    };
    rk_cli::verbs::answer::run(
        &cli,
        &args,
        &crate::support::launched_env(),
        Startup::real(),
    )
    .unwrap_or_else(|halt| panic!("`answer {item}` halted: {halt:?}"))
}

/// What a walk saw: each gate it answered, as `(kind, option)`, how many `run`s it took, and every
/// worktree git knew after any `run` (the run tree is removed at close, so it is read on the way).
pub(crate) struct Walk {
    pub(crate) gates: Vec<(String, String)>,
    pub(crate) runs: usize,
    pub(crate) trees: Vec<PathBuf>,
}

/// **Walk the lane to its `closed` mark**: run; while a gate is awaited, answer its question's
/// first option and run again. Each `run` must exit 0, and the walk stops at `max_runs`.
pub(crate) fn walk_to_close(
    root: &Path,
    item: &str,
    harness: &Arc<ScriptedLane>,
    max_runs: usize,
) -> Walk {
    let mut walk = Walk {
        gates: Vec::new(),
        runs: 0,
        trees: Vec::new(),
    };
    while walk.runs < max_runs {
        let exit = run_once(root, item, harness);
        walk.runs += 1;
        for tree in worktrees(root) {
            if !walk.trees.contains(&tree) {
                walk.trees.push(tree);
            }
        }
        assert_eq!(
            exit,
            Exit::Success,
            "`run {item}` #{} exited {exit:?}; unscripted dispatches {:?}; visited {:?}; stages \
             so far {:?}; gates answered {:?}; the journal's last rows:\n{}",
            walk.runs,
            harness.unscripted(),
            harness.visited(),
            story_stages(root, item),
            walk.gates,
            journal_tail(root, item, 12)
        );
        if story_stages(root, item).last().map(String::as_str) == Some("closed") {
            return walk;
        }
        let Some((_, kind)) = awaited_gate(root, item) else {
            panic!(
                "`run {item}` #{} exited 0 at no awaited gate and before `closed`; stages {:?}",
                walk.runs,
                story_stages(root, item)
            );
        };
        let (option, attach) = first_option(root, item, &kind);
        assert_eq!(
            answer(root, item, &option, attach),
            Exit::Success,
            "`answer {item} {option}` for the `{kind}` gate applies"
        );
        walk.gates.push((kind, option));
    }
    panic!(
        "the lane did not close within {max_runs} runs; stages {:?}; gates {:?}",
        story_stages(root, item),
        walk.gates
    );
}

/// The item journal's last `n` rows, one per line, for a failure message.
pub(crate) fn journal_tail(root: &Path, item: &str, n: usize) -> String {
    let rows: Vec<Event> = crate::support::journal_rows(root, item)
        .into_iter()
        .filter(|e| e.kind.wire_name() != "git_op")
        .collect();
    rows.iter()
        .skip(rows.len().saturating_sub(n))
        .map(|e| format!("  {} {}", e.kind.wire_name(), e.payload))
        .collect::<Vec<_>>()
        .join("\n")
}

// ------------------------------------------------------------------------------- git, read

/// The stage token of every `Story:` mark for `item` on `rk-run/<item>`, oldest first, by git's
/// own `%(trailers)` parser, taking the last whitespace token of each value.
pub(crate) fn story_stages(root: &Path, item: &str) -> Vec<String> {
    let qualified = format!("{PROJECT}/{item}");
    let format = "--format=%(trailers:key=Story,valueonly=true)";
    let out = crate::lane_e2e::git_read(
        root,
        &["log", "--reverse", format, &crate::lane_e2e::run_ref(item)],
    )
    .unwrap_or_default();
    out.lines()
        .map(str::trim)
        .filter(|v| v.split_whitespace().next() == Some(qualified.as_str()))
        .filter_map(|v| v.split_whitespace().last().map(str::to_owned))
        .collect()
}

/// The pack's lane: `stage_order` without its `forbidden` stages, from the installed pack's bytes.
pub(crate) fn declared_lane(root: &Path, pack: &str) -> Vec<String> {
    let src = std::fs::read_to_string(root.join(".redkiln").join("packs").join(pack))
        .expect("the installed pack reads");
    let parsed = rk_kernel::select::Pack::from_toml(pack, &src)
        .unwrap_or_else(|e| panic!("the installed pack loads: {e}"));
    parsed
        .stage_order()
        .iter()
        .filter(|s| parsed.rigor(s) != Some(rk_kernel::rigor::Rigor::Forbidden))
        .cloned()
        .collect()
}

/// Each committed step row's `(activity@version, step id, iteration path)`, in journal order.
pub(crate) fn committed_paths(root: &Path, item: &str) -> Vec<(String, String, serde_json::Value)> {
    crate::support::journal_rows(root, item)
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .filter_map(|e| {
            Some((
                e.payload
                    .get("activity_id_at_version")?
                    .as_str()?
                    .to_owned(),
                e.payload.get("step_id")?.as_str()?.to_owned(),
                e.payload.get("iteration_path")?.clone(),
            ))
        })
        .collect()
}

/// The fixture checkout's observable state for AC-518: `(HEAD, git status --porcelain, git
/// diff-index --quiet HEAD's exit code)`.
pub(crate) fn checkout_state(root: &Path) -> (String, String, Option<i32>) {
    let head = crate::lane_e2e::git_read(root, &["rev-parse", "HEAD"])
        .expect("the fixture has a HEAD")
        .trim()
        .to_owned();
    let porcelain = crate::lane_e2e::git_read(root, &["status", "--porcelain"])
        .expect("git status reads the fixture");
    let quiet = crate::lane_e2e::git_rc(root, &["diff-index", "--quiet", "HEAD"]);
    (head, porcelain, quiet)
}

/// The fork point of `rk-run/<item>` from the fixture's `HEAD`.
pub(crate) fn fork_point(root: &Path, item: &str) -> String {
    crate::lane_e2e::git_read(
        root,
        &["merge-base", "HEAD", &crate::lane_e2e::run_ref(item)],
    )
    .unwrap_or_else(|| panic!("`rk-run/{item}` shares no history with HEAD"))
    .trim()
    .to_owned()
}

/// The commits on `<fork>..rk-run/<item>` whose own diff touches `path`, oldest first.
pub(crate) fn commits_touching(root: &Path, item: &str, path: &str) -> Vec<String> {
    let range = format!(
        "{}..{}",
        fork_point(root, item),
        crate::lane_e2e::run_ref(item)
    );
    crate::lane_e2e::git_read(
        root,
        &["log", "--reverse", "--format=%H", &range, "--", path],
    )
    .unwrap_or_default()
    .lines()
    .map(str::to_owned)
    .collect()
}

/// Every worktree git knows for the fixture, by `git worktree list --porcelain`.
pub(crate) fn worktrees(root: &Path) -> Vec<PathBuf> {
    crate::lane_e2e::git_read(root, &["worktree", "list", "--porcelain"])
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.strip_prefix("worktree "))
        .map(PathBuf::from)
        .collect()
}

/// Every `artifact.write` payload of `kind` for `item`, scoped by the interim `t2_event.item`
/// column (s8e §8e.10), oldest first.
pub(crate) fn artifact_writes(root: &Path, item: &str, kind: &str) -> Vec<serde_json::Value> {
    let sql = format!(
        "SELECT payload FROM t2_event WHERE item = '{item}' AND kind = 'artifact.write' ORDER BY id"
    );
    crate::lane_e2e::sql_rows(&crate::common_dir(root), &sql)
        .unwrap_or_else(|e| panic!("the store reads: {e}"))
        .into_iter()
        .filter_map(|row| row.into_iter().next().flatten())
        .map(|text| serde_json::from_str::<serde_json::Value>(&text).expect("a payload is JSON"))
        .filter(|p| p.get("kind").and_then(serde_json::Value::as_str) == Some(kind))
        .collect()
}

/// The bytes of `path` as committed at `rev`, by `git cat-file blob` (`None` when absent).
pub(crate) fn blob_at(root: &Path, rev: &str, path: &str) -> Option<Vec<u8>> {
    let spec = format!("{rev}:{path}");
    crate::support::git_run(root, &["cat-file", "blob", &spec])
        .ok()
        .map(|out| out.stdout)
}

/// The first commit on `<fork>..rk-run/<item>` whose `Story:` mark names `stage`, oldest first.
pub(crate) fn first_mark_of(root: &Path, item: &str, stage: &str) -> Option<String> {
    let range = format!(
        "{}..{}",
        fork_point(root, item),
        crate::lane_e2e::run_ref(item)
    );
    let format = "--format=%H %(trailers:key=Story,valueonly=true,separator=%x20)";
    let qualified = format!("{PROJECT}/{item}");
    crate::lane_e2e::git_read(root, &["log", "--reverse", format, &range])
        .unwrap_or_default()
        .lines()
        .find_map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens.as_slice() {
                [sha, project, .., last] if *project == qualified && *last == stage => {
                    Some((*sha).to_owned())
                }
                _ => None,
            }
        })
}

/// Each `acceptance_criteria` entry's `(id, verification)` of the item's `Spec` as committed at
/// `rev`, at the path the item's last `Spec` write names.
pub(crate) fn spec_verifications(root: &Path, item: &str, rev: &str) -> Vec<(String, String)> {
    let write = artifact_writes(root, item, "Spec")
        .pop()
        .expect("the item journalled a Spec write");
    let path = write
        .get("path")
        .and_then(serde_json::Value::as_str)
        .expect("the Spec write names its path");
    let bytes = blob_at(root, rev, path)
        .unwrap_or_else(|| panic!("the Spec `{path}` is not committed at {rev}"));
    let spec: serde_json::Value = serde_json::from_slice(&bytes).expect("the landed Spec is JSON");
    let field = |ac: &serde_json::Value, name: &str| {
        ac.get(name)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    spec.get("acceptance_criteria")
        .and_then(serde_json::Value::as_array)
        .expect("the Spec carries acceptance_criteria")
        .iter()
        .map(|ac| (field(ac, "id"), field(ac, "verification")))
        .collect()
}
