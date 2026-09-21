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
use std::time::Duration;

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

    /// The counter itself, which `replay::ReplayPorts` takes by reference so the kernel can read
    /// the same cell this type increments — a replay that dispatched would be visible in both.
    #[must_use]
    pub const fn counter(&self) -> &AtomicU32 {
        &self.dispatches
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
    /// `pack.dispatch-1s.toml` — `[activity.emit-report] timeout_secs = 1`, which is
    /// the DISPATCH's bound and not the gate's. A `hold` child under it is killed by
    /// the adapter's own timer, which the adapter classifies `Cancelled { by: Timeout }`
    /// — §7d.4's row 13, never row 12.
    DispatchOneSecond,
}

impl PackVariant {
    const fn file(self) -> &'static str {
        match self {
            Self::Base => "pack.toml",
            Self::TwoActivities => "pack.two-activities.toml",
            Self::GateOneSecond => "pack.gate-1s.toml",
            Self::DispatchOneSecond => "pack.dispatch-1s.toml",
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
    for sub in ["packs", "catalog", "items", "cards", "schemas"] {
        std::fs::create_dir_all(rk.join(sub)).expect("the .redkiln subdirectory is created");
    }

    let pack = std::fs::read(src.join(variant.file())).expect("the pack variant is readable");
    for name in ["bug-standard.toml", "story.toml"] {
        std::fs::write(rk.join("packs").join(name), &pack).expect("the pack installs");
    }

    for (from, to) in [
        ("artifacts.toml", "artifacts.toml"),
        ("facets.toml", "facets.toml"),
    ] {
        std::fs::copy(src.join(from), rk.join(to)).expect("the registry installs");
    }

    // 🛑 `harness_binary` IS REWRITTEN TO AN ABSOLUTE PATH HERE, AND THE COMMITTED BARE NAME IS
    // NOT WHAT THE VERB READS. §7d.13 fixes the rule — the fixture child is located *"as the
    // sibling of `env!("CARGO_BIN_EXE_redkiln")` in the same target directory"* — but the
    // committed fixture carries the BARE name `rk-fixture-child-claude`, and nothing in `rk-cli`'s
    // `src/` resolves it: `Config::new` stores the string verbatim (its only check is a
    // `.bat`/`.cmd` refusal) and it reaches `Command::new` unchanged.
    //
    // A bare name is then resolved by the OS against ambient state no test controls — on Windows,
    // beginning with the directory the SPAWNING process loaded from, which is `target/debug`,
    // which is exactly where the child is built. So the suite's behaviour depended on whether
    // `target/debug/rk-fixture-child-claude.exe` happened to exist: absent, the spawn failed in
    // microseconds with `Transport { Spawn }` and the run RETURNED a plausible-looking refusal;
    // present, the child ran and `hold` blocked it. That is the intermittency measured on
    // 2026-09-20 — one run returned, a later one hung, on the same bytes.
    //
    // Absolutising kills the ambiguity, and `fixture_child`'s presence assertion turns the absent
    // case from a silent wrong-reason refusal into a failure that names its own remedy.
    // The cards and schemas step (8)'s binder reads, installed beside the pack.
    //
    // 🛑 FILTERED TO `*.json`, and that is not tidiness. `cards/` carries a `README.md` (the one
    // that RAISES `fulfils`), and an unfiltered copy would plant a non-card file in the directory
    // Slice 4's `doctor/roles.rs` is specified to SCAN — a fixture defect authored here and paid
    // for one slice later.
    let mut installed_bindables = 0_usize;
    for sub in ["cards", "schemas"] {
        for entry in std::fs::read_dir(src.join(sub)).expect("the fixture subdirectory reads") {
            let path = entry.expect("an entry").path();
            if path.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let name = path.file_name().expect("a file name");
            std::fs::copy(&path, rk.join(sub).join(name)).expect("the fixture file installs");
            installed_bindables += 1;
        }
    }
    assert!(
        installed_bindables >= 2,
        "only {installed_bindables} card/schema files installed; step (8)'s binder reads one card \
         per role and one schema per provided kind, and a binder that finds neither refuses on the \
         FIXTURE rather than on anything a test names"
    );

    let config_src = std::fs::read_to_string(src.join("config.toml"))
        .expect("the fixture configuration is readable");
    let child = fixture_child();
    // A TOML basic string, so a Windows path's separators survive the round trip.
    let escaped = child.display().to_string().replace('\\', "\\\\");
    let config_out = config_src
        .lines()
        .map(|l| {
            if l.trim_start().starts_with("harness_binary") {
                format!("harness_binary = \"{escaped}\"")
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(
        config_out, config_src,
        "no `harness_binary` row was rewritten in the fixture configuration — the verb would spawn \
         against the committed BARE name and resolve it against whatever `target/` happens to hold"
    );
    std::fs::write(rk.join("config.toml"), config_out).expect("the configuration installs");

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

// --------------------------------------------------------------- the bounded spawn (Halt 5)
//
// # 🛑 §7d.14'S BOUND, WHICH NO `-p rk-cli` TEST HAD UNTIL THIS CYCLE
//
// §7d.14: *"**Every process-spawning test is bounded** (30 s unless stated; Halt 5) and spawns
// with `Stdio::null()` on stdin (D-15)."* The `Stdio::null()` half was honoured from the first
// spawn; **the bound half never was**. Every spawn site in this suite called `.output()`, which
// blocks until stdout reaches EOF and carries no timer at all.
//
// That was latent while nothing the suite spawned ever held its stdout. It stopped being latent
// the moment `run`'s dispatch path became real: `rk-fixture-child-claude` reads its mode from
// `RK_FIXTURE_MODE`, and **an absent variable is not an error — it selects `hold`**
// (`crates/rk-containment/tests/fixtures/bin/rk-fixture-child.rs`:
// `env::var("RK_FIXTURE_MODE").unwrap_or_else(|_| "hold".to_owned())`), which parks for
// `HOLD_PARKS`(600) x `HOLD_PARK`(1 s) with stdout **open and never written**. Measured
// 2026-09-20 at `d109463`: three live dispatches blocked, the test binary sat in `.output()`
// holding its own `.exe`, and `cargo test -p rk-cli --test it` then failed `LNK1104` twice in a
// row because the linker could not write the output the wedged binary held.
//
// So the bound is not decoration, and it is not only a timer: **it must take the process TREE
// down**. A bound that stops waiting while leaving the child alive leaves the pipe held and the
// linker blocked, which is the failure it exists to prevent. `taskkill /T /F` is the shape §7d.5
// already names for a test-side kill at this stage (the AC-450 kill seam), and Checkpoint 7d
// carries no `taskkill|kill_tree` ban — `s7c` §7c.21's ban is `$SRC_SCOPE` and is about the
// *adapter's* runner not walking a tree instead of using its Job Object (AC-224; ADR-0099 d3).
// Nothing here is in an adapter, and nothing here replaces containment.

/// The bound. 30 s, §7d.14's stated default.
pub const SPAWN_BOUND: Duration = Duration::from_secs(30);

/// The poll interval. **A channel nobody sends on is the wait**, never `sleep`:
/// `std::thread::sleep` and `tokio::time::sleep` are clippy-banned workspace-wide (ADR-0127
/// decision 2), and `recv_timeout` returns after the interval without one. The idiom is
/// `rk-adapter-toolchain-node`'s `poll_until`, reused rather than re-invented.
const SPAWN_POLL: Duration = Duration::from_millis(25);

/// The grace the drains get AFTER the child is gone. A drain reads to EOF, and EOF arrives only
/// when the LAST holder of the write end closes it — a surviving grandchild withholds it
/// (`s7c` §7c.10's race; AC-278's shape). So the drains are never joined unbounded: after the
/// grace whatever they have collected is taken, and the thread is left to end on its own.
/// Blocking the test on a pipe a descendant holds is the exact wedge this helper prevents.
const DRAIN_GRACE: Duration = Duration::from_secs(2);

/// A spawn that exceeded [`SPAWN_BOUND`]. It carries what was drained before the kill, because a
/// bound that reports only *"timed out"* says nothing about where the child got to.
#[derive(Debug)]
pub struct TimedOut {
    /// How long the child ran before the kill.
    pub after: Duration,
    /// Whatever reached stdout before the kill.
    pub stdout: Vec<u8>,
    /// Whatever reached stderr before the kill.
    pub stderr: Vec<u8>,
}

/// Run `cmd` under [`SPAWN_BOUND`], killing the child (and sweeping its descendants) if the bound
/// passes. See [`kill_tree`] for what "sweeping" does and does not reach.
///
/// `Ok` is a `std::process::Output` carrying the same three fields `.output()` returns, so a call
/// site changes only its verb. `Err` is the bound firing — which [`output_bounded`] turns into a
/// test failure and which the bound's own positive control asserts.
///
/// # Panics
/// If the child cannot be spawned, or its piped handles cannot be taken.
#[expect(
    clippy::disallowed_types,
    reason = "the verbs are exercised AS PROCESSES (s7d §7d.13's temp-repository rule); this is \
              the one place in the suite that spawns one, and it exists to BOUND that spawn"
)]
pub fn try_output_bounded(
    cmd: &mut std::process::Command,
) -> Result<std::process::Output, TimedOut> {
    try_output_within(cmd, SPAWN_BOUND)
}

/// [`try_output_bounded`] at an explicit bound. The bound's own positive control uses a short one,
/// so proving the mechanism does not cost the suite [`SPAWN_BOUND`] of wall clock; the VALUE
/// 30 s is pinned by its own assertion rather than by how long a control sits.
///
/// # Panics
/// If the child cannot be spawned, or its piped handles cannot be taken.
#[expect(
    clippy::disallowed_types,
    reason = "the verbs are exercised AS PROCESSES (s7d §7d.13's temp-repository rule); this is \
              the one place in the suite that spawns one, and it exists to BOUND that spawn"
)]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side bound, not a decision (AC-001) — s7d §7d.15 names \"the 30 s bounds of \
              §7d.14\" among the reads that take exactly this attribute"
)]
#[expect(
    clippy::zombie_processes,
    reason = "the reap is DELIBERATELY non-blocking on the timeout path: `wait()` there would \
              block on a child the kill did not reach, wedging the very bound this helper is. \
              The exited path is already reaped — `try_wait` returned `Some` — and the timeout \
              path reaps through a bounded `try_wait` loop after the kill"
)]
pub fn try_output_within(
    cmd: &mut std::process::Command,
    bound: Duration,
) -> Result<std::process::Output, TimedOut> {
    use std::io::Read as _;

    let mut child = cmd
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("the binary under test spawns");

    let out_buf = Arc::new(Mutex::new(Vec::new()));
    let err_buf = Arc::new(Mutex::new(Vec::new()));
    let mut out_pipe = child.stdout.take().expect("stdout was piped just above");
    let mut err_pipe = child.stderr.take().expect("stderr was piped just above");
    let out_sink = Arc::clone(&out_buf);
    let err_sink = Arc::clone(&err_buf);
    // Two drains, so neither stream can deadlock the other by filling its pipe buffer.
    std::thread::spawn(move || {
        let mut sink = Vec::new();
        let _ = out_pipe.read_to_end(&mut sink);
        *out_sink.lock().expect("the stdout sink is not poisoned") = sink;
    });
    std::thread::spawn(move || {
        let mut sink = Vec::new();
        let _ = err_pipe.read_to_end(&mut sink);
        *err_sink.lock().expect("the stderr sink is not poisoned") = sink;
    });

    let (_tx, rx) = std::sync::mpsc::channel::<()>();
    let started = std::time::Instant::now();
    let mut exited = None;
    while started.elapsed() <= bound {
        match child.try_wait().expect("the child's status is readable") {
            Some(status) => {
                exited = Some(status);
                break;
            }
            None => {
                let _ = rx.recv_timeout(SPAWN_POLL);
            }
        }
    }

    let Some(status) = exited else {
        let after = started.elapsed();
        kill_tree(child.id());
        // The DIRECT child explicitly, and not only through the sweep above: it is the one
        // holding our two pipes, so it is the one whose death un-blocks the drains. `kill_tree`
        // is the best-effort sweep for whatever it spawned.
        let _ = child.kill();
        // Reap — but NEVER with a blocking `wait()`. A kill that did not land would then wedge the
        // bound itself, which is the failure this whole helper exists to prevent.
        let reap = std::time::Instant::now();
        while reap.elapsed() <= DRAIN_GRACE {
            if matches!(child.try_wait(), Ok(Some(_))) {
                break;
            }
            let _ = rx.recv_timeout(SPAWN_POLL);
        }
        let (stdout, stderr) = drained(&out_buf, &err_buf, &rx);
        return Err(TimedOut {
            after,
            stdout,
            stderr,
        });
    };
    let (stdout, stderr) = drained(&out_buf, &err_buf, &rx);
    Ok(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

/// Give the drains [`DRAIN_GRACE`] to reach EOF, then take whatever they hold. **Never joins** —
/// see [`DRAIN_GRACE`]. A live drain still owns its `Arc` clone, so the strong count is the
/// reading: 1 means the thread has dropped its clone and the sink is final.
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side bound, not a decision (AC-001) — the drain grace, s7d §7d.15's form"
)]
fn drained(
    out_buf: &Arc<Mutex<Vec<u8>>>,
    err_buf: &Arc<Mutex<Vec<u8>>>,
    rx: &std::sync::mpsc::Receiver<()>,
) -> (Vec<u8>, Vec<u8>) {
    let since = std::time::Instant::now();
    while since.elapsed() <= DRAIN_GRACE
        && (Arc::strong_count(out_buf) > 1 || Arc::strong_count(err_buf) > 1)
    {
        let _ = rx.recv_timeout(SPAWN_POLL);
    }
    let out = out_buf
        .lock()
        .expect("the stdout sink is not poisoned")
        .clone();
    let err = err_buf
        .lock()
        .expect("the stderr sink is not poisoned")
        .clone();
    (out, err)
}

/// Sweep the child's DESCENDANTS. On Windows that is `taskkill /T /F`, the shape §7d.5's AC-450
/// kill seam already names; on POSIX the process group.
///
/// **This is best-effort, and the limit is stated rather than papered over.** A tree walk reaches
/// only what is still reachable as a descendant: a process whose own parent has already exited —
/// `rk-fixture-child`'s `orphan` shape — is reparented and no `/T` finds it. That is precisely why
/// the *adapter* contains children with a Job Object instead of a tree walk (AC-224; ADR-0099 d3;
/// ADR-0127), and nothing here replaces that. What this helper must guarantee is narrower and is
/// what the caller above does explicitly: **the direct child dies**, because the direct child is
/// the one holding the piped stdout whose EOF the test is waiting on.
#[expect(
    clippy::disallowed_types,
    reason = "the tree kill IS the bound's mechanism — a bound that leaves the child alive leaves \
              the linker blocked, which is the wedge this helper exists to prevent"
)]
fn kill_tree(pid: u32) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("taskkill")
            .args(["/T", "/F", "/PID", &pid.to_string()])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
    #[cfg(unix)]
    {
        let _ = std::process::Command::new("kill")
            .args(["-KILL", &format!("-{pid}"), &pid.to_string()])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
}

/// [`try_output_bounded`], with the bound firing as a test failure. This is the call every test
/// makes; `try_output_bounded` is for the bound's own positive control.
///
/// # Panics
/// If the child outlived [`SPAWN_BOUND`] — quoting what it had emitted, so the failure names where
/// the child got to rather than only that it did not finish.
#[expect(
    clippy::disallowed_types,
    reason = "the parameter type is the spawn the caller built; see try_output_bounded"
)]
pub fn output_bounded(cmd: &mut std::process::Command) -> std::process::Output {
    match try_output_bounded(cmd) {
        Ok(out) => out,
        Err(TimedOut {
            after,
            stdout,
            stderr,
        }) => panic!(
            "the spawned binary outlived §7d.14's {SPAWN_BOUND:?} bound (ran {after:?}) and its \
             tree was killed. A child that neither exits nor closes stdout is the `hold` default \
             of `rk-fixture-child-claude` when RK_FIXTURE_MODE is unset (s7d §7d.13).\n\
             --- stdout ---\n{}\n--- stderr ---\n{}",
            String::from_utf8_lossy(&stdout),
            String::from_utf8_lossy(&stderr)
        ),
    }
}

/// The fixture child, located **the one way §7d.13 fixes**: as the sibling of the path cargo hands
/// this test at compile time, `.exe` on Windows.
///
/// `rk-fixture-child-claude` is a `[[bin]]` of `rk-adapter-claude`, and cargo sets
/// `CARGO_BIN_EXE_rk-fixture-child-claude` *"only for the owning package's own tests"* — so
/// `rk-cli`'s tests cannot name it that way, and `env!("CARGO_BIN_EXE_redkiln")`'s directory is the
/// route §7d.13 prescribes instead.
///
/// # 🛑 ITS PRESENCE IS ASSERTED, NOT ASSUMED
///
/// `rk-cli`'s `[dev-dependencies]` carry **no** `rk-adapter-claude`, so `cargo test -p rk-cli`
/// does not build that package's bins: the child is in `target/debug` only because a
/// `cargo build --workspace` (Checkpoint 7d's own first gate, GC3) or a `-p rk-adapter-claude`
/// run put it there. Absent, a spawn against it fails with `Transport { Spawn }` in microseconds
/// and the run returns a refusal that looks exactly like a legitimate one — which is how the
/// same bytes returned on one run and hung on the next. Asserting here makes that condition name
/// itself instead of arriving as a wrong-reason red three layers down.
///
/// # Panics
/// If the child is not built beside `redkiln`.
#[must_use]
pub fn fixture_child() -> PathBuf {
    let name = if cfg!(windows) {
        "rk-fixture-child-claude.exe"
    } else {
        "rk-fixture-child-claude"
    };
    let path = Path::new(crate::redkiln()).with_file_name(name);
    assert!(
        path.is_file(),
        "the fixture child is not built at {}. It is a [[bin]] of `rk-adapter-claude`, and \
         `cargo test -p rk-cli` does not build it (rk-cli has no dev-dependency on that package). \
         Run `cargo build --workspace` — Checkpoint 7d's first gate — and re-run.",
        path.display()
    );
    path
}

/// **THE BOUND'S POSITIVE CONTROL** (Halt 8 (a)/(g); §7d.14's self-control rule).
///
/// Every other test in this suite now calls [`output_bounded`], whose whole content is an absence:
/// *no spawn outlives 30 s*. Nothing in those tests would notice if the bound never fired — a
/// helper that simply called `.output()` would leave all thirty-five green, which is exactly the
/// state this cycle found them in. So the bound gets a test that puts a child under it which
/// **cannot** finish, and asserts the three facts that make it an instrument rather than a claim:
/// the bound FIRES, it fires *at* the bound, and it takes the child's TREE with it.
///
/// The victim is the fixture child in `hold` — the same 600 s park (`HOLD_PARKS` x `HOLD_PARK`)
/// that wedged the box at `d109463`, reached here deliberately and under a one-second bound.
#[test]
#[expect(
    clippy::disallowed_types,
    reason = "the victim is spawned AS A PROCESS — a holding child is the only thing that puts \
              the bound under test (s7d §7d.13)"
)]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side bound, not a decision (AC-001) — s7d §7d.15's form for §7d.14's bounds"
)]
fn spawn_bound_fires_on_a_holding_child_and_kills_it() {
    let child = fixture_child();
    let markers = tempfile::tempdir().expect("a marker directory");

    let mut cmd = std::process::Command::new(&child);
    cmd.env("RK_FIXTURE_MODE", "hold")
        .env("RK_FIXTURE_MARKER_DIR", markers.path());

    let bound = Duration::from_secs(1);
    let started = std::time::Instant::now();
    let outcome = try_output_within(&mut cmd, bound);
    let waited = started.elapsed();

    // 1. THE BOUND FIRED. `hold` parks for 600 s, so an `Ok` here is a helper that did not bound.
    let Err(TimedOut { after, .. }) = outcome else {
        panic!(
            "the bound did NOT fire against a child parked for 600 s — `try_output_within` \
             returned Ok, so the bound is not a bound"
        );
    };
    assert!(
        after >= bound,
        "the bound fired after {after:?}, before its own {bound:?} — it is measuring something \
         other than the elapsed wait"
    );

    // 2. IT FIRED AT THE BOUND, not at the child's own 600 s cap. Without this the arm would also
    //    pass against a helper that simply waited the child out.
    assert!(
        waited < Duration::from_secs(60),
        "the call took {waited:?} — the child's own cap is 600 s, so anything near it means the \
         helper waited for the child rather than bounding it"
    );

    // 3. THE TREE IS DOWN. The marker carries the child's real pid (§7c.17), so this is a reading
    //    of the process the helper actually killed, not of a pid the helper reported about itself.
    let marker = std::fs::read_dir(markers.path())
        .expect("the marker directory reads")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|x| x == "marker"))
        .expect(
            "the child wrote no marker, so it never reached `hold` and this arm measured a child \
             that was not yet holding",
        );
    let pid: u32 = std::fs::read_to_string(&marker)
        .expect("the marker reads")
        .lines()
        .find_map(|l| l.strip_prefix("pid=")?.parse().ok())
        .expect("the marker carries its pid");
    assert!(
        !pid_is_alive(pid),
        "the child (pid {pid}) SURVIVED the bound. A bound that stops waiting but leaves the \
         child alive leaves its stdout pipe held, and the held pipe is what blocked the linker \
         (LNK1104) twice on 2026-09-20."
    );
}

/// Is `pid` still alive? Windows `tasklist` filtered to the pid; POSIX `kill -0`.
#[expect(
    clippy::disallowed_types,
    reason = "a liveness READING of the process the bound killed — the control's third assertion \
              is worth nothing without it (Halt 8)"
)]
pub fn pid_is_alive(pid: u32) -> bool {
    #[cfg(windows)]
    {
        let out = std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/NH"])
            .stdin(std::process::Stdio::null())
            .output()
            .expect("tasklist runs");
        String::from_utf8_lossy(&out.stdout).contains(&pid.to_string())
    }
    #[cfg(unix)]
    {
        std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .stdin(std::process::Stdio::null())
            .status()
            .expect("kill -0 runs")
            .success()
    }
}

/// The bound's VALUE, pinned. The control above runs at one second so it costs the suite nothing;
/// this is what holds the 30 s §7d.14 states, so shortening the real bound cannot pass unnoticed.
#[test]
fn spawn_bound_is_the_thirty_seconds_the_stage_file_states() {
    assert_eq!(
        SPAWN_BOUND,
        Duration::from_secs(30),
        "§7d.14: \"Every process-spawning test is bounded (30 s unless stated; Halt 5)\""
    );
}

/// A spec supplier for a case that must never reach a dispatch, which **refuses by name** if one
/// happens anyway.
///
/// The two §7d.10 cases (a pre-fired token, a poisoned lock) return before `replay::resume` is
/// reached at all. Handing them [`any_spec`] would make a run that wrongly got as far as a dispatch
/// succeed quietly; this one turns that into a named refusal instead.
pub fn no_spec() -> impl FnMut(&str) -> Result<RecordedSpec, ReplayError> {
    move |node: &str| {
        Err(ReplayError::Malformed {
            path: "inputs/specs.json".to_owned(),
            detail: format!(
                "`{node}` reached the spec supplier, but this case asserts the run returns before                  any dispatch"
            ),
        })
    }
}

// ------------------------------------------------- the two in-process entry seams, shared
//
// **Promoted here at Slice 3, cycle 5 — moved, never copied.** `root.rs` held both privately for
// §7d.10's two cases; `exit.rs` and `run.rs` now drive the same verb function, and two definitions
// of "steps (1)-(7) over the fixture repository" can drift while both suites stay green against
// different notions of the same run. §7d.16 item 3 puts the shared in-process seams in this file.

/// The instant every in-process case builds its inputs at.
///
/// §7d.5 gives the composition root **one** clock read, and `prepare_inputs` takes `now` as a value
/// for exactly that reason. One constant across every case, so `run_id` — a content hash over the
/// input set, `results_at` included — is stable between them and a test never asserts against a
/// value that moves with the wall clock.
pub const FIXED_NOW: i64 = 1_760_000_000;

/// Steps (1)–(7) over a `pack-min` fixture repository, through the REAL seam.
///
/// Never hand-built: `Resolved`'s fields are `pub`, but `exec: GitExec` and `worktree: WorktreePath`
/// have no test constructor in reach, and a hand-built one would skip the very ordering steps
/// (1)–(7) exist to fix.
///
/// # Panics
/// If steps (1)–(7) refuse — which means the fixture, not the case under test, is wrong.
#[must_use]
pub fn resolved_over_pack_min(root: &Path, item: &str) -> rk_cli::verbs::run::Resolved {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::RunArgs {
        item: item.to_owned(),
        lane: None,
    };
    let env = Env::from_map(std::collections::BTreeMap::new());
    rk_cli::verbs::run::through_step_seven(
        &cli,
        &args,
        &env,
        rk_cli::verbs::Startup::real(),
        rk_git::reconciled::Entry::Run,
    )
    .unwrap_or_else(|e| {
        panic!("steps (1)-(7) resolve over the fixture repository for `{item}`: {e:?}")
    })
}

/// The input set over a `pack-min` fixture repository, at [`FIXED_NOW`], **and its tree hash**.
///
/// The hash is returned because `run_with` takes it: `gate::expire` needs one at step (9), and
/// `RunInputs::runtime()` is private to `rk-kernel`, so it cannot be read back off the input set
/// the first member carries. A case that does not reach a gate still passes it through.
///
/// # Panics
/// If the input set will not build — again a fixture fault, not the case's.
#[must_use]
pub fn inputs_over_pack_min(
    resolved: &rk_cli::verbs::run::Resolved,
) -> (rk_kernel::replay::RunInputs, String) {
    rk_cli::verbs::run::prepare_inputs(resolved, FIXED_NOW)
        .unwrap_or_else(|e| panic!("the input set builds over the fixture repository: {e:?}"))
}

/// [`inputs_over_pack_min`]'s twin, **without the composition root's `Config`**.
///
/// # 🛑 IT EXISTS BECAUSE `write_run` / `read_run` DO NOT ROUND-TRIP A CONFIG, AND THAT IS RAISED
///
/// `RunInputs::to_json` — which is exactly what `run_id` hashes — inserts a `config` key when one
/// was threaded in (`replay.rs`, Slice 2's ADR-0083 / D18-14 edit). **`write_run` writes no config
/// and `read_run` reads none**, so a record produced from config-carrying inputs re-derives a
/// DIFFERENT `run_id` on the way back, and `replay()` — which re-derives and compares — refuses it:
///
/// ```text
/// Divergent(HiddenInput { … the recorded run_id is implement-61b5dd057fefc7fe and the logged
/// input set hashes to implement-ff18038f5d185916 — some input was edited after the run was
/// recorded … })
/// ```
///
/// Measured 2026-09-20 while authoring `ac432_explain_over_replayed_log_is_byte_identical`. It is a
/// **kernel** defect, not a test-setup one: every record `run` writes carries a config, so no
/// record this binary produces can be replayed at all. The kernel's own replay suite does not catch
/// it because its committed golden log pre-dates the config edit and therefore carries none.
///
/// This helper is the narrowest thing that lets the byte-identity property be tested at all, and
/// the test says in its own doc comment that it is testing a config-less record for this reason.
///
/// # Panics
/// If the input set will not build — a fixture fault, not the case's.
#[must_use]
pub fn inputs_over_pack_min_without_config(
    resolved: &rk_cli::verbs::run::Resolved,
    tree_hash: &str,
) -> rk_kernel::replay::RunInputs {
    let root = Path::new(resolved.worktree.as_str()).to_path_buf();
    let dir = root.join(".redkiln").join("catalog");
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    paths.sort();
    let sources: Vec<(String, String)> = paths
        .iter()
        .map(|p| {
            (
                p.file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display())),
            )
        })
        .collect();
    let borrowed: Vec<(&str, &str)> = sources
        .iter()
        .map(|(n, s)| (n.as_str(), s.as_str()))
        .collect();

    let artifacts_src = std::fs::read_to_string(root.join(".redkiln").join("artifacts.toml"))
        .expect("the fixture's artifacts.toml reads");
    let facets_src = std::fs::read_to_string(root.join(".redkiln").join("facets.toml"))
        .expect("the fixture's facets.toml reads");
    let deadline = FIXED_NOW.saturating_add(resolved.extras.gate_deadline_secs);

    rk_kernel::replay::RunInputs::new(&resolved.stage)
        .with_pack(&format!("{}.toml", resolved.pack_name), &resolved.pack_src)
        .with_catalog(&borrowed)
        .with_artifacts("artifacts.toml", &artifacts_src)
        .with_facets(&facets_src)
        .with_item(&resolved.item)
        .with_runtime(
            rk_kernel::replay::RuntimeContext::supplied(tree_hash, deadline).results_at(FIXED_NOW),
        )
}

/// The [`ActivitySpec`] every in-process case binds `pack-min`'s one activity to.
///
/// The schema is `ImplReport` — what `emit-report` declares it provides — so a scripted `Ok`
/// carrying that schema COMMITS. A spec whose schema did not match would make `run_activity` mint
/// `StepError::Schema` itself, and every case would be red on row 11 for a reason unrelated to
/// what it names.
#[must_use]
pub fn any_activity_spec() -> ActivitySpec {
    ActivitySpec {
        card_digest: rk_types::digest::sha256(b"pack-min/cards/implementer.json"),
        tools: vec!["StructuredOutput".to_owned()],
        schema: rk_types::typed::SchemaId::new("ImplReport"),
        timeout_secs: 900,
        inputs: Vec::new(),
    }
}

/// A committing result for [`any_activity_spec`] — the schema matches, so the dispatch commits.
#[must_use]
pub fn impl_report() -> Typed {
    Typed::new(
        rk_types::typed::SchemaId::new("ImplReport"),
        serde_json::json!({ "summary": "pack-min" }),
    )
}

// --------------------------------------------------- the readbacks a SPAWNED run leaves behind

/// **Every row in the repository's store, in insertion order, whatever the repository is.**
///
/// [`journal_rows`] resolves its `GitExec` through `resolved_over_pack_min`, which runs steps
/// (1)-(7) and therefore REFUSES on a repository with no pack — so it cannot read the store of the
/// very case that proves step (6) wrote a row before step (7) refused. This reads the store the way
/// the store itself is addressed: discover the repository, open, query with no selector.
///
/// `SqliteStore::query` ignores its selector and returns the whole journal ordered by id [measured
/// at this pin], so `journal_query` here would be decoration; the whole-journal read is what the
/// row-COUNT assertions of AC-172 actually need.
///
/// # Panics
/// If the repository will not discover, or the store will not open — either means the run under
/// test never reached step (5), which is a different failure and is reported as itself.
#[must_use]
pub fn store_rows(root: &Path) -> Vec<Event> {
    let repo = rk_cli::repo::discover(Some(root))
        .unwrap_or_else(|e| panic!("the repository under test discovers: {e}"));
    let store = rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree)
        .unwrap_or_else(|e| panic!("the store the run wrote opens: {e}"));
    store
        .query(rk_kernel::runtime::journal_query("*"))
        .unwrap_or_else(|e| panic!("the store reads: {e:?}"))
}

/// The item's journal, read back out of the store a spawned `redkiln` wrote.
///
/// The callers spawn the binary, so they hold no store of their own; `SqliteStore::open` needs a
/// `GitExec` and a `WorktreePath`, and `resolved_over_pack_min` is already the one place that
/// produces both through the real seam. Re-running steps (1)–(7) to get them is cheap and writes
/// nothing (step (6)'s reconcile appends no row at this stage).
///
/// # Panics
/// If the store will not open — which means the run under test never created one.
#[must_use]
pub fn journal_rows(root: &Path, item: &str) -> Vec<Event> {
    let resolved = resolved_over_pack_min(root, item);
    let store = rk_adapter_sqlite::SqliteStore::open(&resolved.exec, &resolved.worktree)
        .unwrap_or_else(|e| panic!("the store the run wrote opens: {e}"));
    store
        .query(rk_kernel::runtime::journal_query(item))
        .unwrap_or_else(|e| panic!("the journal reads: {e:?}"))
}

/// `git -C <dir> log -1 --format=%B` — the HEAD commit's full message, trailers included.
#[expect(
    clippy::disallowed_types,
    reason = "reading a fixture worktree's own HEAD message; the product's git goes through \
              rk-git's one GitExec wrapper"
)]
#[must_use]
pub fn head_message(dir: &Path) -> String {
    let out = std::process::Command::new("git")
        .args(["log", "-1", "--format=%B"])
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("git log runs");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// `git rev-list --count HEAD` — how many commits the repository's current branch carries.
#[expect(
    clippy::disallowed_types,
    reason = "reading a fixture repository's own commit count; see `head_message` above"
)]
#[must_use]
pub fn commit_count(dir: &Path) -> usize {
    let out = std::process::Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(dir)
        .stdin(std::process::Stdio::null())
        .output()
        .expect("git rev-list runs");
    String::from_utf8_lossy(&out.stdout)
        .trim()
        .parse()
        .expect("git rev-list --count prints an integer")
}

/// The pid out of the first `<pid>.marker` file in `dir`, or `None` while none has appeared.
///
/// `rk-fixture-child` writes its marker BEFORE it parks (`hold` writes it first, then holds), so
/// the file's appearance is proof the child is alive and holding. It is also OUTSIDE the store
/// lock the kernel holds across a dispatch, which a store poll is not.
#[must_use]
pub fn marker_pid(dir: &Path) -> Option<u32> {
    for entry in std::fs::read_dir(dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().is_none_or(|e| e != "marker") {
            continue;
        }
        let text = std::fs::read_to_string(&path).ok()?;
        if let Some(pid) = text
            .lines()
            .find_map(|l| l.strip_prefix("pid=")?.parse::<u32>().ok())
        {
            return Some(pid);
        }
    }
    None
}
