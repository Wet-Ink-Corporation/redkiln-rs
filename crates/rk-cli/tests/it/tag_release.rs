//! **`tag-release` — a CLI's release, entered by ADR-0114's trial** (`s8g` §8g.7; R5 = ADR-0195,
//! proposed; R3 = ADR-0193 decision 1, proposed; D21-42 6b; D21-46 R4). *(Added 2026-09-29, `s8g`
//! build, package S8G-P8-TAG-RELEASE-AND-MOVES.)*
//!
//! Every test runs in a TEMPORARY repository carrying a copy of this repository's `.redkiln/`
//! (the shipped packs, catalog, `catalog-extended/`, registries and `declared.toml`), with a bare
//! repository as its `origin` that the run never pushes to. **No `rk-release/*` tag is made in this
//! repository** (Checkpoint 8g (11)), and nothing here dispatches: agent steps would reach a
//! [`Counting`] harness over an EMPTY script, and each test asserts it was never called.
//!
//! The rig is the production composition wherever that is observable in-process:
//!
//! - **The item and its catalog are `run`'s own**: [`rk_cli::verbs::run::through_step_seven`]
//!   reads the item record (its `[trial]` declaration included) and the repository's
//!   `declared.toml`, and [`rk_cli::verbs::run::catalog_sources`] composes the catalog the run's
//!   recorded input set carries — the shipped catalog plus, for a declaring item, the one extended
//!   activity the kernel's `catalog::admit_trial` admits.
//! - **The selection is the kernel's** `select` over the lane's own pack.
//! - **The `git` route is `rk-cli`'s** [`rk_cli::release_tag::release_tag_route`], through
//!   `rk-git`, over the temporary repository; the verb routing is `rk-cli`'s `tool_routing` over
//!   the copied `registry.toml`; the writer is `rk-store`'s `ArtifactWriter`.
//! - **The toolchain is a recording double** for `build-package`'s `build` and `package`, whose
//!   handlers are `s8c`'s record V; every other verb (`deploy` included) is refused by its name.
//!
//! 🛑 **What this is not.** It is not `verbs::run::run` over a whole lane: the two lane items are
//! resolved by `run`'s steps (1)-(7) and their release stage is selected and walked in process.
//! The trial's REAL run on this repository is `s8j`'s (§8g.7).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{Activity, Catalog};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::ActivityReport;
use rk_kernel::runtime::step::{Activation, GateSite, StoreCall};
use rk_kernel::select::Item;
use rk_kernel::workflow::breaker::JournaledNow;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_ports::docs_site::DocsSite;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::fakes::{BoardTracker, FakeHarness, RecordingDocs, VecStore};
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_ports::root::Ports;
use rk_ports::toolchain::{
    ChangeSummary, Classification, Selection, StepBound, TestOutcome, Toolchain, VerbCall,
    VerbOutput,
};
use rk_ports::tracker::Tracker;
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::verb_output::{ArtifactBinary, BuildArtifact, BuildReport, BuildStatus};

type Json = serde_json::Value;

/// The item that declares the trial.
const TRIAL: &str = "rel-1";
/// The item that does not.
const PLAIN: &str = "rel-2";
const RUST: &str = "rk-adapter-toolchain-rust";
const TREE_HASH: &str = "0123456789abcdef0123456789abcdef01234567";

// ------------------------------------------------------------------------------- the repository

fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves")
}

fn git(cwd: &Path, args: &[&str]) {
    crate::git(cwd, args);
}

#[expect(
    clippy::disallowed_types,
    reason = "a test reading its own fixture repository; the product's git goes through rk-git"
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
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("mkdir");
    for entry in std::fs::read_dir(from).expect("the shipped directory reads") {
        let path = entry.expect("an entry").path();
        if path.is_file() {
            std::fs::copy(&path, to.join(path.file_name().expect("a name"))).expect("copy");
        }
    }
}

/// An item record at `release`, as this repository's fix-lane item reads, with or without the
/// `[trial]` declaration.
fn item_record(id: &str, trial: bool) -> String {
    let mut s = [
        format!("id    = \"{id}\""),
        "kind  = \"bug\"".to_owned(),
        "stage = \"release\"".to_owned(),
        String::new(),
        "[facets]".to_owned(),
        "cos          = \"standard\"".to_owned(),
        "has_repro    = true".to_owned(),
        "criticality  = \"low\"".to_owned(),
        "blast_radius = \"low\"".to_owned(),
        "size         = \"low\"".to_owned(),
    ]
    .join("\n");
    s.push('\n');
    if trial {
        s.push_str("\n[trial]\nactivity = \"tag-release\"\n");
    }
    s
}

/// The temporary repository: the shipped `.redkiln/` copied in, two items, one landed commit on
/// each item's `rk-run/<item>`, and a bare `origin` that already holds `HEAD` (so a push would have
/// somewhere to go).
struct Repo {
    dir: tempfile::TempDir,
    remote: tempfile::TempDir,
}

impl Repo {
    fn root(&self) -> &Path {
        self.dir.path()
    }

    fn new() -> Self {
        let dir = crate::temp_repo();
        let root = dir.path();
        let shipped = workspace().join(".redkiln");
        let rk = root.join(".redkiln");
        for sub in [
            "packs",
            "catalog",
            "catalog-extended",
            "processes",
            "checklists",
            "kb",
        ] {
            copy_dir(&shipped.join(sub), &rk.join(sub));
        }
        for file in [
            "artifacts.toml",
            "facets.toml",
            "registry.toml",
            "config.toml",
            "declared.toml",
        ] {
            std::fs::copy(shipped.join(file), rk.join(file)).expect("the registry copies");
        }
        std::fs::create_dir_all(rk.join("items")).expect("mkdir items");
        std::fs::write(
            rk.join("items").join(format!("{TRIAL}.toml")),
            item_record(TRIAL, true),
        )
        .expect("the trial item writes");
        std::fs::write(
            rk.join("items").join(format!("{PLAIN}.toml")),
            item_record(PLAIN, false),
        )
        .expect("the plain item writes");
        git(root, &["add", ".redkiln"]);
        git(root, &["commit", "--quiet", "-m", "the shipped .redkiln"]);
        let remote = tempfile::tempdir().expect("a temp dir for the bare remote");
        git(remote.path(), &["init", "--quiet", "--bare"]);
        git(
            root,
            &[
                "remote",
                "add",
                "origin",
                &remote.path().display().to_string(),
            ],
        );
        git(root, &["push", "--quiet", "origin", "HEAD"]);
        let repo = Self { dir, remote };
        for item in [TRIAL, PLAIN] {
            repo.commit_on_run_branch(item, "landed.txt", "the item's change");
        }
        repo
    }

    /// One commit on `rk-run/<item>` (created from `HEAD` when absent); returns its sha. The
    /// checkout returns to where it was.
    fn commit_on_run_branch(&self, item: &str, file: &str, message: &str) -> String {
        let root = self.root();
        let branch = format!("rk-run/{item}");
        let exists = git_out(root, &["branch", "--list", &branch]);
        if exists.is_empty() {
            git(root, &["branch", &branch]);
        }
        git(root, &["checkout", "--quiet", &branch]);
        std::fs::write(root.join(file), format!("{message}\n")).expect("the change writes");
        git(root, &["add", file]);
        git(root, &["commit", "--quiet", "-m", message]);
        let sha = git_out(root, &["rev-parse", "HEAD"]);
        git(root, &["checkout", "--quiet", "-"]);
        sha
    }

    fn head_of(&self, item: &str) -> String {
        git_out(self.root(), &["rev-parse", &format!("rk-run/{item}")])
    }

    /// The commit a tag names, or `None`.
    fn tag_target(&self, tag: &str) -> Option<String> {
        let refs = git_out(self.root(), &["tag", "--list", tag]);
        (!refs.is_empty()).then(|| {
            git_out(
                self.root(),
                &["rev-parse", &format!("refs/tags/{tag}^{{commit}}")],
            )
        })
    }

    /// Every tag the bare `origin` holds.
    fn remote_tags(&self) -> String {
        git_out(self.remote.path(), &["for-each-ref", "refs/tags"])
    }

    fn discovered(&self) -> rk_cli::repo::Repo {
        rk_cli::repo::discover(Some(self.root())).expect("the temporary repository discovers")
    }

    fn resolved(&self, item: &str) -> Result<rk_cli::verbs::run::Resolved, rk_cli::exit::Exit> {
        let cli = rk_cli::cli::Cli {
            cwd: Some(self.root().to_path_buf()),
            command: rk_cli::cli::Command::Status,
        };
        let args = rk_cli::cli::RunArgs {
            item: item.to_owned(),
            lane: None,
        };
        rk_cli::verbs::run::through_step_seven(
            &cli,
            &args,
            &crate::support::clean_env(),
            rk_cli::verbs::Startup::real(),
            rk_git::reconciled::Entry::Run,
        )
    }

    /// The catalog `run` composes for `trial`.
    fn catalog(&self, trial: Option<&str>) -> Catalog {
        let sources = rk_cli::verbs::run::catalog_sources(self.root(), trial)
            .unwrap_or_else(|e| panic!("run's catalog composes: {e:?}"));
        let borrowed: Vec<(&str, &str)> = sources
            .iter()
            .map(|(n, s)| (n.as_str(), s.as_str()))
            .collect();
        Catalog::from_sources(&borrowed).expect("the composed catalog loads")
    }
}

/// The extended `tag-release`, read off the copied `.redkiln/catalog-extended/`.
fn tag_release(repo: &Repo) -> Activity {
    repo.catalog(Some("tag-release"))
        .activities()
        .iter()
        .find(|a| a.id == "tag-release")
        .cloned()
        .expect("the trial admits tag-release")
}

// ------------------------------------------------------------------------------- the doubles

/// A harness over an EMPTY script that counts every call: nothing here may dispatch.
#[derive(Default)]
struct Counting {
    calls: AtomicUsize,
}

impl Harness for Counting {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        self.calls.fetch_add(1, Ordering::SeqCst);
        FakeHarness::scripted(Vec::new()).dispatch(spec, store)
    }
}

/// A recording toolchain for `build-package` (record V's handlers are `s8c`'s).
#[derive(Default)]
struct Release {
    calls: Mutex<Vec<String>>,
}

fn binaries() -> Vec<ArtifactBinary> {
    ["redkiln", "redkiln-migrate"]
        .iter()
        .map(|name| ArtifactBinary {
            name: (*name).to_owned(),
            path: format!("release/{name}"),
            sha256: rk_types::digest::sha256(format!("{name}-bytes").as_bytes())
                .as_str()
                .to_owned(),
        })
        .collect()
}

fn build_artifact(sha: &str) -> BuildArtifact {
    BuildArtifact {
        profile: "release".to_owned(),
        binaries: binaries(),
        argv: vec![
            "cargo".to_owned(),
            "build".to_owned(),
            "--release".to_owned(),
        ],
        toolchain_version: "rustc 1.0.0-double".to_owned(),
        sha: sha.to_owned(),
        head: sha.to_owned(),
    }
}

impl Toolchain for Release {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(true)
    }

    fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
        self.calls
            .lock()
            .expect("unpoisoned")
            .push("build".to_owned());
        Ok(BuildReport {
            argv: vec!["cargo".to_owned(), "build".to_owned()],
            toolchain_version: "rustc 1.0.0-double".to_owned(),
            wall_secs: 0,
            errors: 0,
            status: BuildStatus::Passing,
        })
    }

    fn test(&self, _selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        Err(StepError::Unsupported {
            capability: "test".to_owned(),
        })
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
        self.calls
            .lock()
            .expect("unpoisoned")
            .push(call.verb.to_owned());
        match call.verb {
            "package" => {
                let sha = call
                    .inputs
                    .get("sha")
                    .and_then(Json::as_str)
                    .unwrap_or_default();
                Ok(VerbOutput {
                    kind: "BuildArtifact".to_owned(),
                    value: serde_json::to_value(build_artifact(sha)).expect("serialises"),
                })
            }
            other => Err(StepError::Unsupported {
                capability: other.to_owned(),
            }),
        }
    }
}

// ------------------------------------------------------------------------------- the walk

fn journal(db: &Arc<Mutex<dyn EventStore>>, item: &str) -> Vec<Event> {
    db.lock()
        .expect("unpoisoned")
        .query(Query::ByItem(item.to_owned()))
        .expect("the in-memory store never refuses a query")
}

/// A seeded upstream `emit` binding of `kind` for `item`.
fn seed(db: &Arc<Mutex<dyn EventStore>>, item: &str, kind: &str, value: Json) {
    db.lock()
        .expect("unpoisoned")
        .append(Event::new(
            EventKind::ActivityCommitted,
            serde_json::json!({
                "activity_id_at_version": format!("release-seed-{kind}@1"),
                "attempt": 1,
                "completion": true,
                "item": item,
                "iteration_path": [],
                "run_id": "release-seed",
                "step_id": "emit",
                "at": 50,
                "artifacts": { kind: value },
                "evidence": [],
            }),
        ))
        .expect("the in-memory store never refuses an append");
}

/// Seed what the release stage reads from earlier stages: the item's `Changeset` at `sha`, and an
/// approving `ReviewVerdict`.
fn seed_upstream(db: &Arc<Mutex<dyn EventStore>>, item: &str, sha: &str) {
    seed(
        db,
        item,
        "Changeset",
        serde_json::json!({ "sha": sha, "diff": "", "files": ["landed.txt"] }),
    );
    seed(
        db,
        item,
        "ReviewVerdict",
        serde_json::json!({ "verdict": "approve", "blocking": [] }),
    );
}

struct Stage<'a> {
    repo: &'a Repo,
    item: String,
    db: Arc<Mutex<dyn EventStore>>,
    harness: Arc<Counting>,
    toolchain: Arc<Release>,
    artifacts: tempfile::TempDir,
}

impl<'a> Stage<'a> {
    fn new(repo: &'a Repo, item: &str) -> Self {
        Self {
            repo,
            item: item.to_owned(),
            db: Arc::new(Mutex::new(VecStore::default())),
            harness: Arc::new(Counting::default()),
            toolchain: Arc::new(Release::default()),
            artifacts: tempfile::tempdir().expect("a temp tree for the managed artifacts"),
        }
    }

    /// Walk `activity` once at `release`, through `rk-cli`'s routing and `git` route.
    fn walk(&self, activity: &Activity, rigor: rk_kernel::rigor::Rigor) -> ActivityReport {
        let stage = activity.stage.clone();
        let run_id = format!("{stage}-0123456789abcdef");
        let (a, r, item) = (
            Arc::clone(&self.db),
            Arc::clone(&self.db),
            self.item.clone(),
        );
        let writer = rk_store::render::ArtifactWriter::new(
            self.artifacts.path(),
            rk_store::render::StageRun {
                item: self.item.clone(),
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
                        .query(Query::ByItem(item.clone()))
                        .map_err(|e| format!("{e:?}"))
                },
            ),
        );
        let found = self.repo.discovered();
        let routing = rk_cli::verbs::run::tool_routing(self.repo.root(), Some(RUST))
            .unwrap_or_else(|e| panic!("the copied registry loads through rk-cli's routing: {e}"))
            .with_git_route(rk_cli::release_tag::release_tag_route(
                found.exec,
                found.worktree,
            ));
        let ports = Ports {
            store: Arc::clone(&self.db),
            harness: Arc::clone(&self.harness) as Arc<dyn Harness>,
            tracker: Arc::new(BoardTracker::default()) as Arc<dyn Tracker>,
            docs: Arc::new(RecordingDocs::default()) as Arc<dyn DocsSite>,
            toolchain: Arc::clone(&self.toolchain) as Arc<dyn Toolchain>,
        };
        let rt = Runtime::new(ports, &run_id, &stage)
            .with_tool_routing(routing)
            .with_write_seam(Arc::new(writer));
        let aid = activity
            .id_at_version()
            .unwrap_or_else(|| activity.id.clone());
        let state =
            rk_kernel::workflow::fsm::fold(&self.item, &stage, &journal(&self.db, &self.item))
                .unwrap_or_else(|e| panic!("the seeded journal folds: {e:?}"));
        let no_specs = BTreeMap::new();
        rt.run_activity_steps(
            Activation {
                item: &self.item,
                activity_id_at_version: &aid,
                step_id: &activity.id,
                spec: None,
                step_specs: Some(&no_specs),
                rigor,
            },
            Some(&activity.steps),
            &Item::new(&self.item, "bug").context(),
            GateSite {
                state: &state,
                tree_hash: TREE_HASH,
                deadline: Timestamp::from_unix_seconds(9_000),
            },
            AttemptCap::declared(8),
            JournaledNow::from_journal(Timestamp::from_unix_seconds(100)),
        )
        .unwrap_or_else(|e| panic!("the walk of `{aid}` returns a report: {e:?}"))
    }

    fn events(&self) -> Vec<Event> {
        journal(&self.db, &self.item)
    }

    /// The value the writer wrote for `kind`.
    fn written(&self, kind: &str) -> Json {
        let sidecar = self
            .artifacts
            .path()
            .join(".redkiln/artifacts")
            .join(&self.item)
            .join(format!("{kind}.json"));
        serde_json::from_slice(
            &std::fs::read(&sidecar)
                .unwrap_or_else(|e| panic!("{} is written: {e}", sidecar.display())),
        )
        .expect("the sidecar is JSON")
    }

    fn written_kinds(&self) -> Vec<String> {
        self.events()
            .iter()
            .filter(|e| e.kind == EventKind::ArtifactWrite)
            .filter_map(|e| e.payload.get("kind").and_then(Json::as_str))
            .map(str::to_owned)
            .collect()
    }

    /// The completion rows of `activity`'s activations.
    fn completions(&self, activity: &str) -> usize {
        self.events()
            .iter()
            .filter(|e| e.kind == EventKind::ActivityCommitted)
            .filter(|e| e.payload.get("completion").and_then(Json::as_bool) == Some(true))
            .filter(|e| {
                e.payload
                    .get("activity_id_at_version")
                    .and_then(Json::as_str)
                    .is_some_and(|a| a.split('@').next() == Some(activity))
            })
            .count()
    }

    fn dispatch_rows(&self) -> usize {
        self.events()
            .iter()
            .filter(|e| e.kind == EventKind::Dispatch || e.kind == EventKind::DispatchCompleted)
            .count()
    }
}

const fn walks(report: &ActivityReport) -> bool {
    report.completed() && report.step_error().is_none() && !matches!(report.outcome(), Some(Err(_)))
}

fn refusal(report: &ActivityReport) -> String {
    format!("{:?} {:?}", report.step_error(), report.outcome())
}

/// Walk the extended `tag-release` once for `item`, over a seeded `BuildArtifact` and `Changeset`
/// at the item's landed commit.
fn release_once<'a>(repo: &'a Repo, item: &str) -> (Stage<'a>, ActivityReport) {
    let stage = Stage::new(repo, item);
    let landed = repo.head_of(item);
    seed_upstream(&stage.db, item, &landed);
    seed(
        &stage.db,
        item,
        "BuildArtifact",
        serde_json::to_value(build_artifact(&landed)).expect("serialises"),
    );
    let report = stage.walk(&tag_release(repo), rk_kernel::rigor::Rigor::Optional);
    (stage, report)
}

/// The `git` route called directly, as the kernel calls it.
fn serve(repo: &Repo, item: &str, sha: &str) -> Result<VerbOutput, StepError> {
    let found = repo.discovered();
    let route = rk_cli::release_tag::release_tag_route(found.exec, found.worktree);
    let inputs: BTreeMap<String, Json> = [("sha".to_owned(), Json::from(sha))].into();
    route.serve(&StoreCall {
        verb: "release_tag",
        inputs: &inputs,
        item,
        stage: "release",
        activity_id_at_version: "tag-release@1",
        run_id: "release-0123456789abcdef",
    })
}

// ------------------------------------------------------------------------------- the tag

/// **The release tag is annotated, local, `rk-release/<item>`, on the landed commit** (R5 decision
/// 1). Re-running for the same item at the same commit is idempotent; at a different commit it
/// refuses by name and the tag stays where it was.
#[test]
fn release_record_tags_the_landed_commit_locally() {
    let repo = Repo::new();
    let landed = repo.head_of(TRIAL);
    let (stage, report) = release_once(&repo, TRIAL);
    assert!(
        walks(&report),
        "tag-release does not walk: {}",
        refusal(&report)
    );
    assert_eq!(
        repo.tag_target("rk-release/rel-1").as_deref(),
        Some(landed.as_str()),
        "the item's release tag is not on the landed commit"
    );
    assert_eq!(
        git_out(
            repo.root(),
            &["cat-file", "-t", "refs/tags/rk-release/rel-1"]
        ),
        "tag",
        "the release tag is not annotated"
    );
    let record = stage.written("ReleaseRecord");
    assert_eq!(
        record.get("tag").and_then(Json::as_str),
        Some("rk-release/rel-1")
    );
    assert_eq!(
        record.get("target").and_then(Json::as_str),
        Some(landed.as_str())
    );

    // Idempotent at the same commit.
    let again = serve(&repo, TRIAL, &landed).expect("re-tagging the landed commit is idempotent");
    assert_eq!(
        again.value.get("target").and_then(Json::as_str),
        Some(landed.as_str())
    );
    // Refused by name at another commit of the run branch, and the tag does not move.
    let later = repo.commit_on_run_branch(TRIAL, "later.txt", "a later change");
    let moved = serve(&repo, TRIAL, &later).expect_err("a second commit must not be tagged");
    assert!(
        format!("{moved:?}").contains("tagged-elsewhere"),
        "the refusal does not name `tagged-elsewhere`: {moved:?}"
    );
    assert_eq!(
        repo.tag_target("rk-release/rel-1").as_deref(),
        Some(landed.as_str()),
        "a refused re-tag moved the tag"
    );
    // A commit that is not on the item's run branch refuses by name, before any tag is made.
    let off = repo.head_of(PLAIN);
    let refused = serve(&repo, "rel-3", &off).expect_err("a commit off rk-run/rel-3 is refused");
    assert!(
        format!("{refused:?}").contains("off-run-branch"),
        "the refusal does not name `off-run-branch`: {refused:?}"
    );
    assert_eq!(repo.tag_target("rk-release/rel-3"), None);
}

/// **The rollback target is the nearest earlier `rk-release/*` tag on an ancestor** (R5 decision 1;
/// M10). Three decoys, each of which a wrong reading would take: a non-release tag on a NEARER
/// ancestor (the `--decorate-refs` filter), a release tag on a commit that is NOT an ancestor
/// (ancestry), and the landed commit itself (a default to the landed sha).
#[test]
fn rollback_target_is_the_previous_release_tag() {
    let repo = Repo::new();
    let root = repo.root();
    let older = repo.commit_on_run_branch(TRIAL, "older.txt", "an older release");
    git(
        root,
        &[
            "tag",
            "-a",
            "rk-release/older",
            "-m",
            "an older release",
            &older,
        ],
    );
    let nearer = repo.commit_on_run_branch(TRIAL, "nearer.txt", "a nearer, unreleased commit");
    git(
        root,
        &["tag", "-a", "v9.9.9", "-m", "not a release tag", &nearer],
    );
    let elsewhere = repo.commit_on_run_branch(PLAIN, "elsewhere.txt", "another item's release");
    git(
        root,
        &[
            "tag",
            "-a",
            "rk-release/elsewhere",
            "-m",
            "not an ancestor",
            &elsewhere,
        ],
    );
    let landed = repo.commit_on_run_branch(TRIAL, "landed2.txt", "the landed change");

    let (stage, report) = release_once(&repo, TRIAL);
    assert!(
        walks(&report),
        "tag-release does not walk: {}",
        refusal(&report)
    );
    let record = stage.written("ReleaseRecord");
    assert_eq!(
        record.get("rollback_target").and_then(Json::as_str),
        Some("rk-release/older"),
        "the rollback target is not the nearest earlier release tag on an ancestor: {record}"
    );
    assert_ne!(
        record.get("rollback_target").and_then(Json::as_str),
        Some(landed.as_str()),
        "the rollback target defaulted to the landed sha (M10)"
    );
}

/// **A first release names its rollback target explicitly as `none`** (R5 decision 1): never
/// absent, never empty, never the landed sha.
#[test]
fn first_release_names_its_rollback_target_as_none() {
    let repo = Repo::new();
    let landed = repo.head_of(TRIAL);
    let (stage, report) = release_once(&repo, TRIAL);
    assert!(
        walks(&report),
        "tag-release does not walk: {}",
        refusal(&report)
    );
    let record = stage.written("ReleaseRecord");
    assert_eq!(
        record.get("rollback_target").and_then(Json::as_str),
        Some("none"),
        "a first release does not name `none`: {record}"
    );
    assert_ne!(
        record.get("rollback_target").and_then(Json::as_str),
        Some(landed.as_str())
    );
}

/// **The record carries the `BuildArtifact`'s digests** (R5 decision 1; D21-42 6b: *"plus the
/// BuildArtifact digest"*): each released binary's `sha256`, exactly as the build recorded it.
#[test]
fn release_record_carries_the_build_artifact_digest() {
    let repo = Repo::new();
    let (stage, report) = release_once(&repo, TRIAL);
    assert!(
        walks(&report),
        "tag-release does not walk: {}",
        refusal(&report)
    );
    let record = stage.written("ReleaseRecord");
    let want = serde_json::to_value(binaries()).expect("serialises");
    assert_eq!(
        record.get("binaries"),
        Some(&want),
        "the ReleaseRecord does not carry the BuildArtifact's binary digests: {record}"
    );
    assert!(
        want.as_array().is_some_and(|b| b.iter().all(|x| x
            .get("sha256")
            .and_then(Json::as_str)
            .is_some_and(|d| d.len() == 64))),
        "PREMISE: each seeded binary carries a sha256 digest"
    );
    assert_eq!(
        stage.written_kinds(),
        ["ReleaseRecord"],
        "one ReleaseRecord write"
    );
}

/// **The tag is never pushed** (R5 decision 1; R3's write set): the bare `origin` holds no tag
/// after the release, and `GitOp`'s enumerated write set has no push.
#[test]
fn the_tag_is_never_pushed() {
    let repo = Repo::new();
    assert_eq!(
        repo.remote_tags(),
        "",
        "PREMISE: the bare remote starts with no tag"
    );
    assert!(
        !git_out(repo.root(), &["ls-remote", "origin", "HEAD"]).is_empty(),
        "PREMISE: origin is reachable and holds HEAD, so a push had somewhere to go"
    );
    let (_stage, report) = release_once(&repo, TRIAL);
    assert!(
        walks(&report),
        "tag-release does not walk: {}",
        refusal(&report)
    );
    assert!(
        repo.tag_target("rk-release/rel-1").is_some(),
        "PREMISE: the local tag exists"
    );
    assert_eq!(
        repo.remote_tags(),
        "",
        "the release tag reached the bare remote"
    );
    let ops: Vec<&str> = rk_git::git_exec::GitOp::ALL
        .iter()
        .map(|o| o.as_str())
        .collect();
    assert!(
        !ops.iter().any(|o| o.contains("push")),
        "GitOp carries a push variant: {ops:?}"
    );
}

// ------------------------------------------------------------------------------- the trial

/// **The trial runs one activity with zero dispatches** (R5 decisions 3, 4 and 7; §8g.7): the
/// declaring item's run admits `tag-release`, and its one activation writes one `ReleaseRecord`,
/// journals no `dispatch` or `dispatch_completed` row, calls no harness, and leaves a local tag the
/// bare remote never receives.
#[test]
fn trial_runs_one_activity_with_zero_dispatches() {
    let repo = Repo::new();
    let resolved = repo
        .resolved(TRIAL)
        .unwrap_or_else(|e| panic!("run's steps (1)-(7) resolve the trial item: {e:?}"));
    assert_eq!(resolved.trial.as_deref(), Some("tag-release"));
    let catalog = repo.catalog(resolved.trial.as_deref());
    let activity = catalog
        .activities()
        .iter()
        .find(|a| a.id == "tag-release")
        .cloned()
        .expect("the declaring item's run admits tag-release");
    assert!(
        activity
            .steps
            .iter()
            .all(|s| s.kind != rk_kernel::catalog::StepKind::Agent),
        "tag-release has an agent step, so its trial could dispatch"
    );
    let stage = Stage::new(&repo, TRIAL);
    let landed = repo.head_of(TRIAL);
    seed_upstream(&stage.db, TRIAL, &landed);
    seed(
        &stage.db,
        TRIAL,
        "BuildArtifact",
        serde_json::to_value(build_artifact(&landed)).expect("serialises"),
    );
    let report = stage.walk(&activity, rk_kernel::rigor::Rigor::Optional);
    assert!(
        walks(&report),
        "the trial does not walk: {}",
        refusal(&report)
    );
    assert_eq!(stage.completions("tag-release"), 1, "one activation");
    assert_eq!(stage.dispatch_rows(), 0, "a dispatch row was journalled");
    assert_eq!(
        stage.harness.calls.load(Ordering::SeqCst),
        0,
        "the harness was called"
    );
    assert_eq!(stage.written_kinds(), ["ReleaseRecord"]);
    assert!(
        repo.tag_target("rk-release/rel-1").is_some(),
        "no local tag"
    );
    assert_eq!(repo.remote_tags(), "", "the tag reached the bare remote");
}

/// **A lane run admits the trial activity only on the declaring item** (R5 decision 4; AC-110;
/// M25). Two items in one repository, each resolved by `run`'s steps (1)-(7), each release stage
/// selected over the lane's pack and walked: the declaring item selects `tag-release` and runs it
/// once; the other item's catalog carries no extended activity, it selects `rolling-deploy`, and no
/// release tag is made for it.
#[test]
fn a_lane_run_admits_the_trial_activity_only_on_the_declaring_item() {
    let repo = Repo::new();
    let extended: Vec<String> = std::fs::read_dir(repo.root().join(".redkiln/catalog-extended"))
        .expect("the extended directory reads")
        .filter_map(Result::ok)
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .collect();
    assert_eq!(extended, ["tag-release"], "PREMISE: the extended directory");
    let mut wrong = Vec::new();
    for item in [TRIAL, PLAIN] {
        let resolved = repo
            .resolved(item)
            .unwrap_or_else(|e| panic!("run's steps (1)-(7) resolve `{item}`: {e:?}"));
        let catalog = repo.catalog(resolved.trial.as_deref());
        let carries: Vec<&str> = catalog
            .activities()
            .iter()
            .map(|a| a.id.as_str())
            .filter(|id| extended.iter().any(|x| x == id))
            .collect();
        let pack = rk_kernel::select::Pack::from_toml(
            &format!("{}.toml", resolved.pack_name),
            &resolved.pack_src,
        )
        .expect("the lane's pack loads");
        let artifacts = rk_kernel::select::ArtifactRegistry::from_toml(
            "artifacts.toml",
            &std::fs::read_to_string(repo.root().join(".redkiln/artifacts.toml"))
                .expect("artifacts.toml"),
        )
        .expect("the artifact registry loads");
        let facets = rk_kernel::registry::Registry::from_toml(
            &std::fs::read_to_string(repo.root().join(".redkiln/facets.toml")).expect("facets"),
        )
        .expect("the facet registry loads");
        let selection = rk_kernel::select::select(
            &pack,
            &catalog,
            &artifacts,
            &facets,
            &resolved.stage,
            &resolved.item,
        )
        .unwrap_or_else(|e| panic!("`{item}`'s release stage selects: {e}"));
        let plan = selection.plan().expect("the release stage plans");
        let stage = Stage::new(&repo, item);
        seed_upstream(&stage.db, item, &repo.head_of(item));
        let mut selected = Vec::new();
        for node_id in plan.order() {
            let node = selection
                .dag()
                .nodes()
                .iter()
                .find(|n| n.id == node_id)
                .expect("a plan node is a DAG node");
            let activity = catalog
                .activities()
                .iter()
                .find(|a| a.id == node.activity_id)
                .expect("a selected activity is in the run's catalog");
            let report = stage.walk(activity, selection.rigor());
            selected.push((activity.id.clone(), walks(&report), refusal(&report)));
        }
        let ids: Vec<&str> = selected.iter().map(|(a, _, _)| a.as_str()).collect();
        if item == TRIAL {
            if carries != ["tag-release"] {
                wrong.push(format!(
                    "`{item}`'s catalog carries {carries:?}, not tag-release alone"
                ));
            }
            if !ids.contains(&"tag-release") || ids.contains(&"rolling-deploy") {
                wrong.push(format!(
                    "`{item}` selected {ids:?}: tag-release does not replace rolling-deploy"
                ));
            }
            if let Some((_, false, why)) = selected.iter().find(|(a, _, _)| a == "tag-release") {
                wrong.push(format!("`{item}`'s tag-release refuses: {why}"));
            }
            if stage.completions("tag-release") != 1 {
                wrong.push(format!(
                    "`{item}` ran tag-release {} times, not once",
                    stage.completions("tag-release")
                ));
            }
            if repo.tag_target("rk-release/rel-1").as_deref() != Some(repo.head_of(item).as_str()) {
                wrong.push(format!("`{item}` made no release tag on its landed commit"));
            }
        } else {
            if !carries.is_empty() {
                wrong.push(format!(
                    "`{item}` declares no trial and its run admits {carries:?} (AC-110; M25)"
                ));
            }
            if ids.contains(&"tag-release") || !ids.contains(&"rolling-deploy") {
                wrong.push(format!("`{item}` selected {ids:?}, not rolling-deploy"));
            }
            if repo.tag_target("rk-release/rel-2").is_some() {
                wrong.push(format!("`{item}` made a release tag"));
            }
        }
        if stage.harness.calls.load(Ordering::SeqCst) != 0 || stage.dispatch_rows() != 0 {
            wrong.push(format!("`{item}`'s release dispatched"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **An enabled pack referencing the extended activity is refused at load** (AC-110, unchanged by
/// ADR-0114 decision 3): a pack that names `tag-release` (`[activity.tag-release]`) refuses `run`
/// with exit 3 before anything is selected, and naming the reference. CONTROL: the same repository
/// with the shipped pack resolves.
#[test]
fn an_enabled_pack_referencing_the_extended_activity_is_refused() {
    let repo = Repo::new();
    assert!(
        repo.resolved(TRIAL).is_ok(),
        "CONTROL: the shipped pack, which references no extended activity, resolves"
    );
    let pack = repo.root().join(".redkiln/packs/bug-standard.toml");
    let mut src = std::fs::read_to_string(&pack).expect("the pack reads");
    src.push_str("\n[activity.tag-release]\ntimeout_secs = 1200\n");
    std::fs::write(&pack, &src).expect("the pack writes");
    let refused = repo.resolved(TRIAL);
    assert!(
        matches!(refused, Err(rk_cli::exit::Exit::ConfigError)),
        "a pack referencing the extended tag-release was not refused with exit 3 (AC-110)"
    );
    assert_eq!(
        rk_cli::verbs::run::extended_references(repo.root(), &src).expect("the references read"),
        ["tag-release"],
        "the refusal's reading does not name the referenced extended activity"
    );
}

/// **Selection picks `tag-release` over the four deploy patterns** (R5 decision 5, option (a)): on
/// both shipped packs, for an item whose run admits the trial, in a repository that declares
/// `release_form = "tag"`. CONTROLS: the same item where the repository declares `deploy`, and the
/// same repository for an item that declares no trial, each select `rolling-deploy`. No shipped
/// deploy activity was edited to make it win (their `applicable_if` and `method_rank` are the
/// values R5 measured).
#[test]
fn selection_picks_tag_release_over_the_deploy_patterns() {
    let repo = Repo::new();
    let rk = repo.root().join(".redkiln");
    let artifacts = rk_kernel::select::ArtifactRegistry::from_toml(
        "artifacts.toml",
        &std::fs::read_to_string(rk.join("artifacts.toml")).expect("artifacts.toml"),
    )
    .expect("the artifact registry loads");
    let facets = rk_kernel::registry::Registry::from_toml(
        &std::fs::read_to_string(rk.join("facets.toml")).expect("facets.toml"),
    )
    .expect("the facet registry loads");
    let producer = |pack_file: &str, item: &Item, trial: Option<&str>| -> Vec<String> {
        let pack = rk_kernel::select::Pack::from_toml(
            pack_file,
            &std::fs::read_to_string(rk.join("packs").join(pack_file)).expect("the pack"),
        )
        .expect("the pack loads");
        let catalog = repo.catalog(trial);
        let selection =
            rk_kernel::select::select(&pack, &catalog, &artifacts, &facets, "release", item)
                .unwrap_or_else(|e| panic!("{pack_file}'s release stage selects: {e}"));
        let mut ids: Vec<String> = selection
            .dag()
            .nodes()
            .iter()
            .map(|n| n.activity_id.clone())
            .filter(|id| {
                catalog.activities().iter().any(|a| {
                    &a.id == id && a.provides.first().map(String::as_str) == Some("ReleaseRecord")
                })
            })
            .collect();
        ids.sort();
        ids
    };
    let resolved = repo.resolved(TRIAL).expect("the trial item resolves");
    assert_eq!(
        resolved.item.facet("release_form"),
        Some(&rk_kernel::select::FacetValue::Str("tag".to_owned())),
        "the repository's declared.toml does not reach the item's context"
    );
    let plain = repo.resolved(PLAIN).expect("the plain item resolves");
    for pack in ["bug-standard.toml", "story.toml"] {
        assert_eq!(
            producer(pack, &resolved.item, resolved.trial.as_deref()),
            ["tag-release"],
            "{pack}: the ReleaseRecord producer is not tag-release"
        );
        assert_eq!(
            producer(pack, &plain.item, plain.trial.as_deref()),
            ["rolling-deploy"],
            "CONTROL {pack}: an item without the trial does not select rolling-deploy"
        );
    }
    std::fs::write(rk.join("declared.toml"), "release_form = \"deploy\"\n").expect("declared");
    let deploy = repo
        .resolved(TRIAL)
        .expect("the trial item resolves under deploy");
    for pack in ["bug-standard.toml", "story.toml"] {
        assert_eq!(
            producer(pack, &deploy.item, deploy.trial.as_deref()),
            ["rolling-deploy"],
            "CONTROL {pack}: release_form = deploy still selects tag-release"
        );
    }
    let shipped = Catalog::from_dir(&workspace().join(".redkiln/catalog")).expect("the catalog");
    for (id, applicable_if, rank) in [
        (
            "rolling-deploy",
            "blast_radius < med or rigor < recommended or kind == incident",
            10,
        ),
        ("blue-green-deploy", "blast_radius >= high", 20),
        ("canary-release", "blast_radius >= med", 30),
        (
            "flag-progressive-rollout",
            "flag_exists != null and flag_exists",
            40,
        ),
    ] {
        let a = shipped
            .activities()
            .iter()
            .find(|a| a.id == id)
            .unwrap_or_else(|| panic!("the shipped catalog carries {id}"));
        assert_eq!(
            (a.applicable_if.as_deref(), a.method_rank),
            (Some(applicable_if), rank),
            "{id} was edited: R5 forbids editing a shipped deploy activity to make tag-release win"
        );
    }
}
