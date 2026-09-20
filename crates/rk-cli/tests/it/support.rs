//! **The in-process test seams** — §7d.16 item 3's *"the scripted-harness helpers, the
//! `EntryReconciler` recorder, the counting store fake"*, plus `MutatingHarness`, which §7d.5 names
//! by file **and** type (`tests/it/support.rs::MutatingHarness`).
//!
//! This module asserts nothing. It is what makes every later red observable, which is why it lands
//! as a cycle of its own before any verb body: a seam invented alongside the first test that needs
//! it gets the shape that test happens to want.
//!
//! # 🛑 TWO BEHAVIOURS OF `rk_ports::fakes::FakeHarness` THAT NO RECORD STATES, AND BOTH BITE HERE
//!
//! 1. **It appends to the store on every `Err`.** `dispatch`'s body is
//!    `if let Err(error) = &result { let _ = store.append(event_for(error)); }`, routed through
//!    `StepError::event()` into one of `step_failed` / `step_cancelled` / `rate_limited` /
//!    `step_refused`. §7d.4 drives *all eleven `StepError` rows* through this fake in-process, so
//!    every one of those dispatches puts a row into whatever store `Ports.store` holds. **Any
//!    assertion here that counts store rows around a scripted failure must count that row or
//!    exclude its kind** — `rk-kernel`'s own test tree wrote a separate counting harness for exactly
//!    this reason and says so: *"a count over a journal the fake also writes measures two things at
//!    once."*
//! 2. **A fired token overrides the script.** `dispatch` checks `self.token.is_cancelled()`
//!    **before** consulting the script and, on a fired token, returns
//!    `StepError::Cancelled { by: CancelledBy::Operator }` — **not** the scripted element and
//!    **not** `by: Signal`. So a scripted `Cancelled { by: Signal }` is reachable only with an
//!    UNFIRED token. Harmless for §7d.4's table (all three `CancelledBy` arms map to `Exit::Cancelled`
//!    = 13) and fatal to any test that asserts the `by` it scripted while also firing the token.
//!
//! # Why every fake hands its concrete `Arc` back
//!
//! `rk_ports::root::Ports` has five fields, no constructor and no `Clone`, and both
//! `rk_kernel::replay::record` and `replay::resume` take it **by value**. A fake moved into `Ports`
//! is out of the test's reach unless the test kept its own `Arc` — the precedent is
//! `rk-kernel`'s `recording_ports`, *"so its dispatch count is a **reading** rather than a number
//! the recording printed about itself."*

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rk_git::error::GitError;
use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::reconciled::{Entry, Outcome};

use rk_kernel::replay::{RecordedSpec, ReplayError};
use rk_ports::cancel::CancelToken;
use rk_ports::docs_site::DocsSite;
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_ports::root::Ports;
use rk_ports::toolchain::Toolchain;
use rk_ports::tracker::Tracker;
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::Typed;

use rk_cli::adapters::{Refusal, Resolved};
use rk_cli::config::{ConfigError, ConfigFile};
use rk_cli::env::Env;
use rk_cli::verbs::{AdapterResolver, ConfigReader, EntryReconciler};

// ------------------------------------------------------------------ the ordered observation log

/// One observed act, in the order it happened.
///
/// **An ordered log and not a pair of counters**, because §7d.14's bullet fixes AC-402 as an
/// ORDERING claim — red *"by any of the three reaching a store read before its recorded
/// `EntryReconciler` call … a weaker red counts calls without ordering"* — and two counters cannot
/// express "before".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// A reconcile went through the seam, with the `Entry` it carried.
    Reconcile(Entry),
    /// The store was read.
    Query,
    /// The store was written, with the kind of the row.
    Append(EventKind),
}

/// The shared log every seam in this module pushes into.
#[derive(Debug, Default)]
pub struct Trace(Mutex<Vec<Step>>);

impl Trace {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn push(&self, step: Step) {
        self.0.lock().expect("the trace lock is sound").push(step);
    }

    #[must_use]
    pub fn steps(&self) -> Vec<Step> {
        self.0.lock().expect("the trace lock is sound").clone()
    }

    /// Every `Entry` that reached the seam, in order. **Slice 3's reader** —
    /// `ac402_status_and_explain_do_not_reconcile` asserts this is empty on both verbs.
    #[must_use]
    pub fn reconciles(&self) -> Vec<Entry> {
        self.steps()
            .into_iter()
            .filter_map(|s| match s {
                Step::Reconcile(entry) => Some(entry),
                Step::Query | Step::Append(_) => None,
            })
            .collect()
    }

    // 🛑 **`first_reconcile()` / `first_store_touch()` ARE NOT AUTHORED HERE, and their absence is
    // the point.** They are AC-402's ORDERING readback, and AC-402's ordering test —
    // `ac402_reconcile_called_on_run_answer_doctor` — is **Slice 4's**: §7d.16 item 3 says it
    // *"drives `doctor` in-process and is Slice 4's, where `verbs/doctor.rs` exists — in this slice
    // it would be red for the stub's `3`"*. Slice 3's only reader of this recorder is the NEGATIVE,
    // `ac402_status_and_explain_do_not_reconcile`, which reads [`Self::reconciles`] and asserts it
    // empty on both verbs. The contract's own rule decides the omission: *"Build `first_reconcile()`
    // / `first_store_touch()` only if a Slice-3 test reads them; otherwise they land with Slice 4's
    // test … an unread readback is not evidence of anything."* The ORDER is recorded regardless —
    // [`Step::Query`] and [`Step::Append`] go into the same log as [`Step::Reconcile`] — so Slice 4
    // adds two readers over data this module already keeps, and does not re-author the recorder.
}

// ------------------------------------------------------------------------------ the harnesses

/// A `Harness` that **counts its own calls** and delegates to `FakeHarness::scripted`.
///
/// **Why a new type rather than `FakeHarness` bare.** §7d.5's AC-485 wants more than a row count:
/// *"both dispatches went through the one `Harness` object handed in (the impl counts its own
/// calls)"*, and `FakeHarness` has no counter — §7d.5 says so in terms for AC-401 (*"the fake has no
/// counter"*). **Why delegation rather than a re-authored responder**: §7d.4 and §7d.5 both name
/// `FakeHarness::scripted` as the value the eleven `StepError` rows are driven through, and a
/// re-implementation would silently drop `rk-types`' `StepError::event()` bridge — at the cost
/// documented in this module's header, which a row-count test must be written against.
///
/// One script element **per admitted attempt**, never per node: §7d.5 fixes the reason — the fake
/// returns `Transport { kind: Spawn, detail: "script exhausted" }` when it runs dry, *"and a
/// retried class would otherwise land on row `10` for a reason unrelated to the mapping"*.
pub struct ScriptedHarness {
    inner: FakeHarness,
    dispatches: AtomicU32,
}

impl ScriptedHarness {
    #[must_use]
    pub fn scripted(script: Vec<Result<Typed, StepError>>) -> Self {
        Self {
            inner: FakeHarness::scripted(script),
            dispatches: AtomicU32::new(0),
        }
    }

    /// The composition root's token, as a real adapter receives it (ADR-0099 d2). See this module's
    /// header for what a FIRED token does to the script.
    #[must_use]
    pub fn with_token(self, token: CancelToken) -> Self {
        Self {
            inner: self.inner.with_token(token),
            dispatches: self.dispatches,
        }
    }

    /// How many times the port was reached — a READING, never a number the run printed about itself.
    #[must_use]
    pub fn dispatches(&self) -> u32 {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl Harness for ScriptedHarness {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        self.inner.dispatch(spec, store)
    }
}

/// The `Harness` §7d.5's AC-485 bullet names by file and type: *"whose second `dispatch` rewrites
/// `config.toml`'s `harness` to `rk-adapter-linear` before delegating to a `FakeHarness::scripted`
/// (`rk_ports::fakes::FakeHarness` … **can run no side effect between dispatches**)"*.
///
/// The rewrite is the mid-run edit. What discriminates is **not** this harness but the counting
/// seams: AC-485's red is *"a second `AdapterSet::resolve` call anywhere after step (4) or a second
/// `ConfigFile` read"*, read off [`CountingConfigReader`] and [`CountingResolver`].
pub struct MutatingHarness {
    inner: FakeHarness,
    config: PathBuf,
    dispatches: AtomicU32,
}

impl MutatingHarness {
    #[must_use]
    pub fn new(config: &Path, script: Vec<Result<Typed, StepError>>) -> Self {
        Self {
            inner: FakeHarness::scripted(script),
            config: config.to_path_buf(),
            dispatches: AtomicU32::new(0),
        }
    }

    #[must_use]
    pub fn dispatches(&self) -> u32 {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl Harness for MutatingHarness {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        // The pre-increment count: `1` is the SECOND dispatch.
        if self.dispatches.fetch_add(1, Ordering::SeqCst) == 1
            && let Ok(src) = std::fs::read_to_string(&self.config)
        {
            let _ = std::fs::write(
                &self.config,
                src.replace(
                    "harness = \"rk-adapter-claude\"",
                    "harness = \"rk-adapter-linear\"",
                ),
            );
        }
        self.inner.dispatch(spec, store)
    }
}

// ------------------------------------------------------------------------ the recording seams

/// The `EntryReconciler` that **records and then delegates**.
///
/// It cannot fabricate an `Outcome` — ADR-0109 decision 1 gives `Reconciled<T>` a private field with
/// no public constructor — so the real call is the only way to produce one, and that is exactly why
/// `rk_cli::verbs::EntryReconciler` passes `exec` and `repo` through.
pub struct RecordingReconciler {
    trace: Arc<Trace>,
}

impl RecordingReconciler {
    #[must_use]
    pub fn new(trace: &Arc<Trace>) -> Self {
        Self {
            trace: Arc::clone(trace),
        }
    }
}

impl EntryReconciler for RecordingReconciler {
    fn reconcile(
        &self,
        exec: &GitExec,
        repo: &WorktreePath,
        entry: Entry,
    ) -> Result<Outcome, GitError> {
        self.trace.push(Step::Reconcile(entry));
        rk_git::reconciled::reconcile_at_entry(exec, repo, entry)
    }
}

/// AC-485's first counter. Delegates to [`rk_cli::verbs::FileConfigReader`]'s behaviour.
pub struct CountingConfigReader {
    reads: AtomicUsize,
}

impl CountingConfigReader {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            reads: AtomicUsize::new(0),
        }
    }

    #[must_use]
    pub fn reads(&self) -> usize {
        self.reads.load(Ordering::SeqCst)
    }
}

impl ConfigReader for CountingConfigReader {
    fn read(&self, path: &Path) -> Result<ConfigFile, ConfigError> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        rk_cli::verbs::FileConfigReader.read(path)
    }
}

/// AC-485's second counter.
pub struct CountingResolver {
    resolves: AtomicUsize,
}

impl CountingResolver {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            resolves: AtomicUsize::new(0),
        }
    }

    #[must_use]
    pub fn resolves(&self) -> usize {
        self.resolves.load(Ordering::SeqCst)
    }
}

impl AdapterResolver for CountingResolver {
    fn resolve(&self, file: &ConfigFile, env: &Env) -> Result<Resolved, Refusal> {
        self.resolves.fetch_add(1, Ordering::SeqCst);
        rk_cli::verbs::RealAdapterResolver.resolve(file, env)
    }
}

// ----------------------------------------------------------------------------- the store fake

/// The counting `EventStore` §7d.5's AC-402 bullet names — *"whose first `query` / `append` records
/// whether the reconcile call preceded it"*.
///
/// How it differs from `rk_ports::fakes::VecStore`, which stays in use where a test needs a store
/// and no reading: `VecStore` has no counters, no ordering and identity `snapshot`/`rebuild`.
pub struct CountingStore {
    events: Vec<Event>,
    trace: Arc<Trace>,
    queries: AtomicUsize,
    appends: AtomicUsize,
}

impl CountingStore {
    #[must_use]
    pub fn new(trace: &Arc<Trace>) -> Self {
        Self {
            events: Vec::new(),
            trace: Arc::clone(trace),
            queries: AtomicUsize::new(0),
            appends: AtomicUsize::new(0),
        }
    }

    #[must_use]
    pub fn queries(&self) -> usize {
        self.queries.load(Ordering::SeqCst)
    }

    #[must_use]
    pub fn appends(&self) -> usize {
        self.appends.load(Ordering::SeqCst)
    }
}

impl EventStore for CountingStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        self.appends.fetch_add(1, Ordering::SeqCst);
        self.trace.push(Step::Append(event.kind));
        self.events.push(event);
        Ok(())
    }

    /// **The selector is ignored, deliberately.** §7d.6 records that the REAL `SqliteStore::query`
    /// also ignores its selector and returns every event, so a fake that filtered would be sharper
    /// than the product and would hide a verb that queried for the wrong thing.
    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        self.queries.fetch_add(1, Ordering::SeqCst);
        self.trace.push(Step::Query);
        Ok(self.events.clone())
    }

    /// **Refused rather than faked**, on `rk-kernel`'s own stated warrant: *"`Snapshot`'s contents
    /// are decided by no record … an identity snapshot here would be a round-trip a test could pass
    /// without anything being restored."* Nothing in Slice 3 calls either; `doctor --rebuild` is
    /// Slice 4's, and if something starts to, this refusal is what says so.
    fn snapshot(&self) -> Result<Snapshot, StepError> {
        Err(StepError::Unsupported {
            capability: "snapshot".to_owned(),
        })
    }

    fn rebuild(&mut self, _snapshot: &Snapshot) -> Result<(), StepError> {
        Err(StepError::Unsupported {
            capability: "rebuild".to_owned(),
        })
    }
}

// --------------------------------------------------------------------------- the port builders

/// Every port bound, with the ones a case drives handed in and the three no Slice-3 verb calls
/// bound to `rk-ports`' own fakes.
///
/// `Toolchain`, `Tracker` and `DocsSite` are bound and never called, and that is structural rather
/// than incidental: `Ports` holds five, and a `Runtime` cannot be constructed over four.
#[must_use]
pub fn ports(harness: Arc<dyn Harness>, store: &Arc<Mutex<dyn EventStore>>) -> Ports {
    Ports {
        store: Arc::clone(store),
        harness,
        tracker: Arc::new(BoardTracker::default()) as Arc<dyn Tracker>,
        docs: Arc::new(RecordingDocs::default()) as Arc<dyn DocsSite>,
        toolchain: Arc::new(FixtureToolchain::default()) as Arc<dyn Toolchain>,
    }
}

/// A [`CountingStore`] behind the root's own lock, in the shape `Ports` holds it.
#[must_use]
pub fn counting_store(trace: &Arc<Trace>) -> Arc<Mutex<dyn EventStore>> {
    Arc::new(Mutex::new(CountingStore::new(trace))) as Arc<Mutex<dyn EventStore>>
}

/// A store whose lock is **really** poisoned — a thread panics while holding the guard, and is
/// joined.
///
/// `adr0118_poisoned_store_lock_halts_run_exit_70` hands the verb this `Arc`. The poison is asserted
/// before it is returned, on `root.rs`'s own rule: a test that asserted the halt over a SOUND lock
/// would pass against an `acquire` that halted unconditionally.
///
/// # Panics
/// If the poisoning thread did not panic, or if the lock is not poisoned afterwards — either way
/// nothing below would be testing a poisoned lock.
#[must_use]
pub fn poisoned_store() -> Arc<Mutex<dyn EventStore>> {
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let poisoner = Arc::clone(&store);
    let handle = std::thread::spawn(move || {
        let _guard = poisoner
            .lock()
            .expect("the lock is sound when this thread takes it");
        panic!("a writer panicked while holding the store lock");
    });
    assert!(
        handle.join().is_err(),
        "the poisoning thread was expected to panic, and did not — nothing given this `Arc` would \
         be testing a poisoned lock"
    );
    assert!(
        store.is_poisoned(),
        "the lock is not poisoned after a panic while its guard was held"
    );
    store
}

/// The spec supplier every in-process loop test hands `rk_kernel::replay::resume`.
///
/// §7d.5 step (9): *"the fake-harness tests hand a supplier over `RecordedSpec::supplied(..)`
/// values — the fake ignores `card_digest`, `spec_for` does not"*. Without one, a fake-harness run
/// is red on `inputs/specs.json` **before any dispatch**, which RT-55 spells as the wrong reason.
///
/// Note the OWNED return: `RunInputs::spec_for` returns `&RecordedSpec`, so the closure constructs
/// a fresh binding rather than lending one.
pub fn any_spec(
    activity_id_at_version: &str,
    spec: ActivitySpec,
) -> impl FnMut(&str) -> Result<RecordedSpec, ReplayError> {
    let activation = activity_id_at_version.to_owned();
    move |_node: &str| Ok(RecordedSpec::supplied(&activation, spec.clone()))
}

// ------------------------------------------------------------------------- the store readbacks
//
// **Promoted here from `tests/it/init.rs` at Slice 3, cycle 3 — moved, never copied.** `ac413_…`
// and `common_dir_…` need the identical readings `init`'s own tests make, and two definitions of
// "the store's identity" can drift while both suites stay green against different notions of the
// same fact. §7d.16 item 3 puts the shared in-process seams in this file; these are two of them.

/// The store's three identity rows, read the way §7d.9 names: through `rk_store::conn::Reader` and
/// `rk_store::identity::read`, not through `SqliteStore` — the callers spawn the binary, so they
/// hold no store of their own.
///
/// **`read` returns an `Option`, and the `Some` is asserted here rather than compared.** Two `None`s
/// compare equal, so a caller that compared the `Option`s directly would pass over a repository with
/// no identity table at all — which is precisely the state an `init` that never opened the store
/// leaves behind.
///
/// # Panics
/// If the store does not open, or carries no identity rows — either way the caller's assertion
/// would be over nothing.
#[must_use]
pub fn identity_rows(common_dir: &Path) -> rk_store::identity::Identity {
    let reader = rk_store::conn::Reader::open(common_dir).unwrap_or_else(|e| {
        panic!(
            "the store at {} opens for reading: {e}",
            common_dir.display()
        )
    });
    rk_store::identity::read(reader.conn())
        .expect("the identity table reads")
        .expect(
            "the store carries identity rows — `init` opens it eagerly (D-14), which stamps them",
        )
}

/// The store file's digest, or `None` if there is no store.
#[must_use]
pub fn store_digest(common_dir: &Path) -> Option<String> {
    let path = common_dir.join("redkiln").join("store.sqlite3");
    std::fs::read(&path)
        .ok()
        .map(|b| rk_types::digest::sha256(&b).as_str().to_owned())
}

// ------------------------------------------------------------------ the pack-min fixture repository
//
// §7d.13 requires that "every integration test … mints its own" repository (D18-9), and four Slice-3
// case modules need the same one. The builder is new, so it lands in this new file and calls
// `crate::temp_repo` / `crate::git` rather than re-implementing them.

/// Which `pack-min` pack variant a fixture repository installs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackVariant {
    /// `pack.toml` — one baseline kind, one node.
    Base,
    /// `pack.two-activities.toml` — two baseline kinds, two nodes. Installs the second catalog
    /// activity beside the first.
    TwoActivities,
    /// `pack.gate-1s.toml` — `gate_deadline_secs = 1`, for the expiry tests.
    GateOneSecond,
}

impl PackVariant {
    const fn file(self) -> &'static str {
        match self {
            Self::Base => "pack.toml",
            Self::TwoActivities => "pack.two-activities.toml",
            Self::GateOneSecond => "pack.gate-1s.toml",
        }
    }
}

/// The `pack-min/` fixture directory, read in place and copied from.
#[must_use]
pub fn pack_min_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("pack-min")
}

/// A temp repository with `pack-min` installed at §7d.5's paths.
///
/// The pack is installed **twice**, as `.redkiln/packs/bug-standard.toml` and
/// `.redkiln/packs/story.toml` — ADR-0128 decision 3's two lane names, one file — and
/// `[packs] enabled` names both. `init` is NOT run: this builder writes the scaffold itself, because
/// `init`'s scaffold carries `enabled = []` and every test here needs a pack set.
///
/// # Panics
/// If the fixture directory is absent or a file will not copy — either way the caller's assertions
/// would be over a repository that is not the fixture.
#[must_use]
pub fn pack_min_repo(variant: PackVariant) -> tempfile::TempDir {
    let repo = crate::temp_repo();
    let root = repo.path();
    let src = pack_min_dir();
    let rk = root.join(".redkiln");
    for sub in ["packs", "catalog", "items"] {
        std::fs::create_dir_all(rk.join(sub)).expect("the .redkiln subdirectory is created");
    }

    let pack = std::fs::read(src.join(variant.file())).expect("the pack variant is readable");
    for name in ["bug-standard.toml", "story.toml"] {
        std::fs::write(rk.join("packs").join(name), &pack).expect("the pack installs");
    }

    for (from, to) in [
        ("artifacts.toml", "artifacts.toml"),
        ("facets.toml", "facets.toml"),
        ("config.toml", "config.toml"),
    ] {
        std::fs::copy(src.join(from), rk.join(to)).expect("the registry installs");
    }

    let mut catalog = vec!["emit-report.toml"];
    if variant == PackVariant::TwoActivities {
        catalog.push("second-activity.toml");
    }
    for name in catalog {
        std::fs::copy(
            src.join("catalog").join(name),
            rk.join("catalog").join(name),
        )
        .expect("the catalog activity installs");
    }

    // 🛑 THE INSTALL NAME IS THE RECORD'S OWN `id`, NOT THE FIXTURE FILE'S NAME. §7d.5 fixes the
    // path the verb reads as `.redkiln/items/<item>.toml`, where `<item>` is the argument to
    // `redkiln run` — i.e. the id. The fixture files are named for what they PIN (`kindless-item`,
    // `stageless-item`) so a reader can see the manifest at a glance, and those two names are
    // §7d.13's own. Copying them across verbatim would put `kindless-item.toml` where the verb
    // looks for `kindless-1.toml`, and every refusal test would pass on a MISSING FILE rather than
    // on the refusal it names.
    let mut installed = 0_usize;
    for entry in std::fs::read_dir(src.join("items")).expect("the items directory reads") {
        let path = entry.expect("an entry").path();
        let src_text = std::fs::read_to_string(&path).expect("the item record is readable");
        let id = src_text
            .lines()
            .find_map(|l| l.strip_prefix("id"))
            .and_then(|rest| rest.split('=').nth(1))
            .map(|v| v.trim().trim_matches('"').to_owned())
            .unwrap_or_else(|| panic!("{} declares no top-level id", path.display()));
        std::fs::write(rk.join("items").join(format!("{id}.toml")), &src_text)
            .expect("the item record installs");
        installed += 1;
    }
    assert!(
        installed >= 5,
        "only {installed} item records installed; §7d.13 names five (bug, story, kindless, \
         stageless, gate) and a test whose record is missing refuses on the absent FILE rather than \
         on the refusal it is named for"
    );

    repo
}

/// Rewrite `[packs] enabled` in a fixture repository's configuration.
///
/// `adr0128_lane_maps_to_pack_fix_bug_standard_story_story`'s not-enabled arm removes `story`.
///
/// # Panics
/// If the configuration is absent or the row is not there to replace — a silent no-op would leave
/// the test asserting a refusal the repository never earned.
pub fn set_enabled(root: &Path, enabled: &[&str]) {
    let path = root.join(".redkiln").join("config.toml");
    let src = std::fs::read_to_string(&path).expect("the fixture configuration is readable");
    let list = enabled
        .iter()
        .map(|p| format!("\"{p}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let want = format!("enabled = [{list}]");
    let out = src
        .lines()
        .map(|l| {
            if l.trim_start().starts_with("enabled") {
                want.clone()
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(
        out,
        src,
        "no `enabled` row was rewritten in {} — the arm would run against the fixture's own pack set",
        path.display()
    );
    std::fs::write(&path, out).expect("the configuration writes");
}
