//! **The golden logs and the replay** — AC-006, AC-008, AC-042.
//!
//! > **AC-042** — `redkiln replay --run <id>` re-derives the DAG from the logged inputs alone and
//! > reports `HiddenInput { run_id, first_divergent_node }` on any mismatch, **never a warning**.
//! > **This is a first measurement.** F19's stated limit is that **the FSM was never replayed** —
//! > 174/174 suspensions were lossless, but "same next action" is unproven [measured, F19, one
//! > operator]. SM-013 records **no baseline** and reports non-identical runs as a count, never as a
//! > percentage rounded to 100%.
//!
//! # WHY THIS MODULE IS NOT UNDER [`crate::select`]
//!
//! It opens files. [`crate::select`] carries `#![deny(clippy::disallowed_methods)]` and its own
//! doc states that *"nothing reachable from `select` opens a file"* — AC-041's whole content. The
//! recorder and the replayer are the **runtime boundary**: they read the log, call [`select`], and
//! compare. Putting them inside the pure function would make the criterion false by construction.
//!
//! # 🛑 WHAT IS MEASURED, AND WHAT IS NOT — AC-006's five assertions
//!
//! | # | assertion | state at Slice 3 |
//! | --- | --- | --- |
//! | 1 | every port bound to its fake | **a type fact, not a measurement** — [`ReplayPorts`] has no optional field, so an unbound port does not compile. This module reports **no list of bound ports**: a `Vec` of string literals the replay prints about itself is the very shape `s7b`:1018-1023 forbids, and it was one until 2026-09-13 |
//! | 2 | byte-identical decisions | **measured** under `rk_store::canon` |
//! | 3 | byte-identical **next action** | **NOT BUILDABLE — there is no FSM.** The workflow/activity split, the memo, retry and the runtime are §7b.10's Slice 4. Reported ARMED |
//! | 4 | zero `rk-git` writes | **measured against `rk-git`'s OWN counter** — [`rk_git::gitops::GitOps::writes_performed`], read before and after. ADR-0105 decision 1's named instrument, not a seam this crate invented |
//! | 5 | zero harness dispatches **with the terminal state matched** | **dispatch half measured** through the caller's own counter ([`ReplayPorts::dispatches`]), read before and after; **terminal-state half NOT BUILDABLE**, for assertion 3's reason |
//!
//! **Both counters are read from the caller's fakes, before and after**, so neither is a constant
//! this module prints about itself — which is exactly what `s7b`:1018-1023 says the positive control
//! exists to prevent.
//!
//! # The git counter is `rk-git`'s, and the earlier claim that it could not be was FALSE
//!
//! Until 2026-09-13 this module declared its own `pub trait GitWriter { fn writes(&self) -> u32 }`
//! and justified it as *"`rk-git` is a concrete crate and exposes no port to bind a fake to"*. That
//! sentence is **refuted by `rk-git`'s own source**: `crates/rk-git/src/gitops.rs`:44-51 declares
//! `trait GitJournal` (crate-local, *"not a port"*) and `VecJournal` — *"an in-memory journal — **the
//! fake, and what the replay tests drive**"* — and `GitOps` exposes `writes_performed()` because
//! *"`writes_performed` counts what actually happened so a test can assert the zero rather than
//! trust it"*. Being **not a port** is exactly why `GitOps` is bindable without touching the port
//! surface AC-483 counts. AC-006, AC-008 and `s7b`:1121 all name `GitOps` by name. The kernel-local
//! trait is deleted and the replay now counts `rk-git`'s own invocations.
//!
//! # The per-run layout inside `golden-logs/` — **fixed here, as `s7b`:831-832 requires**
//!
//! > The per-run layout inside `golden-logs/` is this stage's to fix … Write it into this section in
//! > the commit that authors the replay test, before the pin moves.
//!
//! ```text
//! golden-logs/
//!   <run_id>/                     run_id = sha256_lp(["rk-kernel/golden-log/v1", stage, inputs])
//!     run.json                    { schema, run_id, stage, inputs_digest, event_count, producer }
//!     inputs/
//!       pack/<name>.toml          the pack, verbatim
//!       catalog/<name>.toml       one file per activity, verbatim
//!       artifacts/<name>.toml     the artifact-kind registry, verbatim
//!       facets.toml               the facet registry, verbatim
//!       context.json              the item's context digest preimage (canonical)
//!       artifact-state.json       the item's artifact digests preimage (canonical)
//!       deviation.json            OPTIONAL: the RequestDeviation and its requesting activity
//!     events.jsonl                one canonical JSON object per line, LF-terminated
//! ```
//!
//! **`run_id` is derived, never stamped.** It is a content hash of the inputs, so it needs no clock,
//! no counter and no random source — the three things AC-041 forbids and AC-043 exists to catch
//! downstream — and a replay recomputes it rather than reading it back. **Nothing in the layout
//! carries a timestamp**, for the same reason: a recorded `at` would make two runs of identical
//! inputs produce different bytes and turn AC-042's byte-identity into a comparison of clocks.
//!
//! **`events.jsonl` is canonical JSON, one object per line.** `rk_store::canon` emits no trailing
//! newline, so the line separator is added here and the last line carries one — stated because the
//! byte comparison depends on it.
//!
//! # Where the logs are staged before the corpus move
//!
//! `s7b`:860 says *"produce all three artifacts into a **staging** tree, never into the corpus"* and
//! :848-849 says *"the staged records live in a **scratch tree**, never under `spikes/results/`"*.
//! **Neither names a path, and this stage fixes it: `target/golden-logs-staging/`** — untracked, so
//! AC-465 holds, and stable, so the operator's single `CORPUS_PIN` move is a copy of a named
//! directory. The committed in-crate copies at `crates/rk-kernel/tests/fixtures/logs/` are §7b.12's
//! own `logs/` row, *"for the red step, before the corpus copies exist"*.

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use rk_git::gitops::GitOps;
use rk_ports::event_store::EventStore;
use rk_ports::harness::Harness;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::catalog::Catalog;
use crate::registry::Registry;
use crate::select::{
    ArtifactRegistry, DeviationKind, Item, Pack, RequestDeviation, SelectError, Selection, select,
    step7,
};

/// The layout version, carried in `run.json` so a reader never has to guess which shape a run is in.
pub const SCHEMA: &str = "rk-kernel/golden-log/v1";

/// **AC-042's THEN.** *"…reports `HiddenInput { run_id, first_divergent_node }` on any mismatch,
/// never a warning."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenInput {
    pub run_id: String,
    /// The **first** divergent node, not a list: a replay that reported every difference would make
    /// the first — the one that caused the rest — hard to find.
    pub first_divergent_node: String,
}

impl fmt::Display for HiddenInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HiddenInput {{ run_id: {:?}, first_divergent_node: {:?} }} — the run could not be \
             replayed to the same DAG from its logged inputs, so it has a hidden input and its \
             memo key is UNSOUND (design §4.4's reproducibility contract; ADR-0024). This is never \
             a warning",
            self.run_id, self.first_divergent_node
        )
    }
}

/// What stopped a record, a write, a read or a replay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayError {
    Io {
        path: String,
        detail: String,
    },
    Malformed {
        path: String,
        detail: String,
    },
    Select(SelectError),
    /// **AC-006's fourth assertion, refused rather than reported.** ADR-0105 decision 1: *"on replay
    /// the kernel is served the journal and `rk-git` performs zero writes."* `writes` is
    /// `rk_git::gitops::GitOps::writes_performed()`, `rk-git`'s own counter.
    GitWroteOnReplay {
        writes: usize,
    },
    /// **AC-008's fake.** *"A replay test with a scripted-responder `Harness` fake that fails the
    /// test on any `dispatch` call."*
    HarnessDispatchedOnReplay {
        dispatches: u32,
    },
    /// The store refused an append the replay had to make to serve the journal back.
    StoreRefused {
        detail: String,
    },
    Divergent(HiddenInput),
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Two distinct variants, one rendering — the same shape and the same warrant as
            // `Step2Error::Parse | Step2Error::Eval`. They are NOT merged into one variant: a log
            // that could not be OPENED and one that was opened and did not have the recorded shape
            // are different defects with different repairs, and collapsing them would make the
            // distinction unrecoverable from the error a caller matches on.
            Self::Io { path, detail } | Self::Malformed { path, detail } => {
                write!(f, "{path}: {detail}")
            }
            Self::Select(e) => write!(f, "{e}"),
            Self::GitWroteOnReplay { writes } => write!(
                f,
                "the git seam performed {writes} write(s) during a replay. ADR-0105 decision 1: on \
                 replay the kernel is SERVED the journal and `rk-git` performs zero writes \
                 (AC-006). The replay refuses rather than reporting a counter"
            ),
            Self::HarnessDispatchedOnReplay { dispatches } => write!(
                f,
                "the harness was dispatched {dispatches} time(s) during a replay. Selection spends \
                 $0.00 in model dispatches, ALWAYS, and any model spend attributed to selection \
                 falsifies purity and invalidates every replay claim in this area (AC-008, AC-044)"
            ),
            Self::StoreRefused { detail } => {
                write!(f, "the event store refused an append: {detail}")
            }
            Self::Divergent(h) => write!(f, "{h}"),
        }
    }
}

impl std::error::Error for ReplayError {}

impl From<SelectError> for ReplayError {
    fn from(e: SelectError) -> Self {
        Self::Select(e)
    }
}

/// Every port the replay can reach, each bound. **No field is optional**, so an unbound port is a
/// compile error rather than a runtime `None` — AC-006's *"with every port bound to its fake"*.
///
/// **No trait declared here.** `harness` is `rk-ports`' own [`Harness`] and `git` is `rk-git`'s own
/// [`GitOps`]. Two port-shaped `pub trait`s used to live in this file — `GitWriter` and
/// `ReplayHarness: Harness` — and neither was covered by the guard that exists for the class:
/// AC-483's port-inventory denylist parses `rk-ports`' source only, so a git-shaped trait declared
/// in `rk-kernel` was invisible to the assertion that exists to prevent it. Both are deleted.
///
/// The dispatch counter is the **caller's**, shared rather than abstracted: the `Harness` fake
/// increments it and the replay reads it before and after, so `harness_dispatches` is a reading and
/// not a literal this module prints about itself (`s7b`:1018-1023).
pub struct ReplayPorts<'a, 'j> {
    pub harness: &'a dyn Harness,
    /// Moved by the fake's own `dispatch`, read here before and after the work.
    pub dispatches: &'a std::sync::atomic::AtomicU32,
    pub store: &'a mut dyn EventStore,
    /// `rk-git`'s journalled seam (ADR-0105 clause 1), in [`rk_git::gitops::Mode::Replay`]. Its
    /// `writes_performed()` is what AC-006's *"zero `rk-git` writes"* is measured off.
    pub git: &'a GitOps<'j>,
}

/// What the replay measured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReport {
    pub run_id: String,
    pub events_compared: usize,
    /// **A count, never a percentage** — SM-013 has no baseline and the register says so in terms.
    /// Always 0 on a successful replay, because the first divergence is an error, not a tally.
    pub divergences: usize,
    /// `rk_git::gitops::GitOps::writes_performed()` as read **after** the replay.
    pub rk_git_writes: usize,
    pub harness_dispatches: u32,
}

/// The four digests' preimages, plus the optional deviation, as one recorded value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RunInputs {
    stage: String,
    pack: (String, String),
    catalog: Vec<(String, String)>,
    artifacts: (String, String),
    facets: String,
    context: serde_json::Value,
    artifact_state: serde_json::Value,
    deviation: Option<(RequestDeviation, String)>,
}

impl RunInputs {
    #[must_use]
    pub fn new(stage: &str) -> Self {
        Self {
            stage: stage.to_owned(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_pack(mut self, name: &str, src: &str) -> Self {
        self.pack = (name.to_owned(), src.to_owned());
        self
    }

    #[must_use]
    pub fn with_catalog(mut self, sources: &[(&str, &str)]) -> Self {
        self.catalog = sources
            .iter()
            .map(|(n, s)| ((*n).to_owned(), (*s).to_owned()))
            .collect();
        // Sorted, so the log is a function of the catalog and not of the caller's argument order.
        self.catalog.sort();
        self
    }

    #[must_use]
    pub fn with_artifacts(mut self, name: &str, src: &str) -> Self {
        self.artifacts = (name.to_owned(), src.to_owned());
        self
    }

    #[must_use]
    pub fn with_facets(mut self, src: &str) -> Self {
        self.facets = src.to_owned();
        self
    }

    #[must_use]
    pub fn with_item(mut self, item: &Item) -> Self {
        let (context, artifact_state) = item.recorded_json();
        self.context = context;
        self.artifact_state = artifact_state;
        self
    }

    /// Record a `RequestDeviation` as part of the run's input set.
    ///
    /// **AC-064's second AND is what this is for:** *"the deviation record is part of the replay
    /// input set, so an amended DAG is not a hidden input."* Pair it with [`Self::requested_by`].
    #[must_use]
    pub fn with_deviation(mut self, request: &RequestDeviation) -> Self {
        let by = self
            .deviation
            .as_ref()
            .map_or_else(String::new, |(_, b)| b.clone());
        self.deviation = Some((request.clone(), by));
        self
    }

    /// The activity that returned the deviation — needed for AC-062's `deviation_ok` check.
    #[must_use]
    pub fn requested_by(mut self, activity_id: &str) -> Self {
        if let Some((request, _)) = self.deviation.clone() {
            self.deviation = Some((request, activity_id.to_owned()));
        }
        self
    }

    /// The whole input set as one canonical value — what `run_id` hashes and what the layout writes.
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "schema": SCHEMA,
            "stage": self.stage,
            "pack": { "name": self.pack.0, "src": self.pack.1 },
            "catalog": self.catalog.iter().map(|(n, s)| serde_json::json!({
                "name": n, "src": s,
            })).collect::<Vec<_>>(),
            "artifacts": { "name": self.artifacts.0, "src": self.artifacts.1 },
            "facets": self.facets,
            "context": self.context,
            "artifact_state": self.artifact_state,
            "deviation": self.deviation.as_ref().map(|(r, by)| serde_json::json!({
                "request": r.to_json_public(),
                "requested_by": by,
            })),
        })
    }

    /// `sha256_lp([SCHEMA, stage, canonical(inputs)])`, hex.
    ///
    /// **Derived, not stamped** — see the module header.
    ///
    /// # Errors
    ///
    /// [`ReplayError`] if the canonical serialiser refused a value.
    pub fn run_id(&self) -> Result<String, ReplayError> {
        let body =
            rk_store::canon::canonical(&self.to_json()).map_err(|e| ReplayError::Malformed {
                path: "<inputs>".to_owned(),
                detail: e.to_string(),
            })?;
        let digest =
            rk_types::digest::sha256_lp(&[SCHEMA.as_bytes(), self.stage.as_bytes(), &body]);
        Ok(format!("{}-{}", self.stage, &digest.as_str()[..16]))
    }

    /// Parse the four registries and rebuild the item — *"from the logged inputs alone"*.
    ///
    /// # Errors
    ///
    /// [`ReplayError::Malformed`] naming the input that did not have the recorded shape.
    pub fn build(&self) -> Result<(Pack, Catalog, ArtifactRegistry, Registry, Item), ReplayError> {
        let bad = |path: &str, detail: String| ReplayError::Malformed {
            path: path.to_owned(),
            detail,
        };
        let pack = Pack::from_toml(&self.pack.0, &self.pack.1)
            .map_err(|e| bad(&self.pack.0, e.to_string()))?;
        let sources: Vec<(&str, &str)> = self
            .catalog
            .iter()
            .map(|(n, s)| (n.as_str(), s.as_str()))
            .collect();
        let catalog = Catalog::from_sources(&sources).map_err(|e| bad("catalog", e.to_string()))?;
        let artifacts = ArtifactRegistry::from_toml(&self.artifacts.0, &self.artifacts.1)
            .map_err(|e| bad(&self.artifacts.0, e.to_string()))?;
        let facets =
            Registry::from_toml(&self.facets).map_err(|e| bad("facets.toml", e.to_string()))?;
        let item = Item::from_recorded(&self.context, &self.artifact_state)
            .map_err(|e| bad("context.json", e))?;
        Ok((pack, catalog, artifacts, facets, item))
    }
}

/// One recorded run: its inputs and the journal they produced.
///
/// **`Eq` is deliberately not derived.** [`rk_types::event::Event`]'s payload is a
/// `serde_json::Value`, which is `PartialEq` and not `Eq` because JSON numbers admit floats — and
/// the comparison this module actually makes is the **canonical byte** one, not `==` on a `Value`.
/// Deriving `Eq` here would offer a comparison AC-042 does not use.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordedRun {
    pub inputs: RunInputs,
    pub events: Vec<Event>,
    run_id: String,
    inputs_digest: String,
}

impl RecordedRun {
    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// The four-digest tuple AC-041 names, as the selection derived it.
    #[must_use]
    pub fn inputs_digest(&self) -> &str {
        &self.inputs_digest
    }

    /// The journal under `rk_store::canon`, one object per line, LF-terminated.
    ///
    /// # Errors
    ///
    /// [`ReplayError::Malformed`] if the canonical serialiser refused an event payload.
    pub fn canonical_events(&self) -> Result<Vec<u8>, ReplayError> {
        canonical_events(&self.events)
    }
}

fn event_json(event: &Event) -> serde_json::Value {
    serde_json::json!({
        "kind": event.kind.wire_name(),
        "payload": event.payload,
    })
}

fn canonical_events(events: &[Event]) -> Result<Vec<u8>, ReplayError> {
    let mut out = Vec::new();
    for event in events {
        let line =
            rk_store::canon::canonical(&event_json(event)).map_err(|e| ReplayError::Malformed {
                path: "events.jsonl".to_owned(),
                detail: e.to_string(),
            })?;
        out.extend_from_slice(&line);
        out.push(b'\n');
    }
    Ok(out)
}

/// **Run selection over the logged inputs and collect the journal.**
///
/// This is the one place the journal is derived, and both [`record`] and [`replay`] go through it —
/// so a replay cannot agree with a recording because the two were written to agree.
///
/// # Errors
///
/// [`ReplayError`] for a malformed input or a selection that could not resolve.
fn derive(inputs: &RunInputs) -> Result<Vec<Event>, ReplayError> {
    let (pack, catalog, artifacts, facets, item) = inputs.build()?;
    let selection: Selection = select(&pack, &catalog, &artifacts, &facets, &inputs.stage, &item)?;
    let mut events = selection.journal();

    if let Some((request, requested_by)) = &inputs.deviation {
        let outcome = step7(
            &pack,
            &catalog,
            &artifacts,
            &facets,
            &inputs.stage,
            &item,
            requested_by,
            request,
            &selection,
        )?;
        // ADR-0120 decision 5: only the accepted branch writes, and decision 3 forbids a second
        // `dag_assembled` in place of the `dag_amended`. So the re-run contributes exactly one
        // event, and the refused and gated branches contribute none.
        if let Some(event) = outcome.event() {
            events.push(event);
        }
    }
    Ok(events)
}

/// Produce one golden-log run from its inputs. **The journal is derived, never hand-written.**
///
/// # Errors
///
/// [`ReplayError`] for a malformed input or a selection that could not resolve.
pub fn record(inputs: &RunInputs) -> Result<RecordedRun, ReplayError> {
    let events = derive(inputs)?;
    let (pack, catalog, artifacts, facets, item) = inputs.build()?;
    let selection = select(&pack, &catalog, &artifacts, &facets, &inputs.stage, &item)?;
    Ok(RecordedRun {
        run_id: inputs.run_id()?,
        inputs_digest: selection.inputs_digest().as_str().to_owned(),
        inputs: inputs.clone(),
        events,
    })
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), ReplayError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| ReplayError::Io {
            path: parent.display().to_string(),
            detail: e.to_string(),
        })?;
    }
    std::fs::write(path, bytes).map_err(|e| ReplayError::Io {
        path: path.display().to_string(),
        detail: e.to_string(),
    })
}

fn read_file(path: &Path) -> Result<Vec<u8>, ReplayError> {
    std::fs::read(path).map_err(|e| ReplayError::Io {
        path: path.display().to_string(),
        detail: e.to_string(),
    })
}

fn read_text(path: &Path) -> Result<String, ReplayError> {
    String::from_utf8(read_file(path)?).map_err(|e| ReplayError::Malformed {
        path: path.display().to_string(),
        detail: e.to_string(),
    })
}

/// Write one run into `root/<run_id>/`, in the layout the module header fixes.
///
/// The directory is removed first, so a run written twice has no residue from a previous shape —
/// otherwise a renamed file would survive and the byte-identity comparison would pass over a tree
/// carrying something the layout no longer names.
///
/// # Errors
///
/// [`ReplayError::Io`] naming the path, or [`ReplayError::Malformed`] from the serialiser.
pub fn write_run(root: &Path, run: &RecordedRun) -> Result<PathBuf, ReplayError> {
    let dir = root.join(&run.run_id);
    let _ = std::fs::remove_dir_all(&dir);

    let manifest = serde_json::json!({
        "schema": SCHEMA,
        "run_id": run.run_id,
        "stage": run.inputs.stage,
        "inputs_digest": run.inputs_digest,
        "event_count": run.events.len(),
        "producer": "rk-kernel::replay::record",
    });
    let mut manifest_bytes =
        rk_store::canon::canonical(&manifest).map_err(|e| ReplayError::Malformed {
            path: "run.json".to_owned(),
            detail: e.to_string(),
        })?;
    manifest_bytes.push(b'\n');
    write_file(&dir.join("run.json"), &manifest_bytes)?;

    let inputs = dir.join("inputs");
    write_file(
        &inputs.join("pack").join(&run.inputs.pack.0),
        run.inputs.pack.1.as_bytes(),
    )?;
    for (name, src) in &run.inputs.catalog {
        write_file(&inputs.join("catalog").join(name), src.as_bytes())?;
    }
    write_file(
        &inputs.join("artifacts").join(&run.inputs.artifacts.0),
        run.inputs.artifacts.1.as_bytes(),
    )?;
    write_file(&inputs.join("facets.toml"), run.inputs.facets.as_bytes())?;
    for (name, value) in [
        ("context.json", &run.inputs.context),
        ("artifact-state.json", &run.inputs.artifact_state),
    ] {
        let mut bytes = rk_store::canon::canonical(value).map_err(|e| ReplayError::Malformed {
            path: name.to_owned(),
            detail: e.to_string(),
        })?;
        bytes.push(b'\n');
        write_file(&inputs.join(name), &bytes)?;
    }
    if let Some((request, by)) = &run.inputs.deviation {
        let mut bytes = rk_store::canon::canonical(&serde_json::json!({
            "request": request.to_json_public(),
            "requested_by": by,
        }))
        .map_err(|e| ReplayError::Malformed {
            path: "deviation.json".to_owned(),
            detail: e.to_string(),
        })?;
        bytes.push(b'\n');
        write_file(&inputs.join("deviation.json"), &bytes)?;
    }

    write_file(&dir.join("events.jsonl"), &run.canonical_events()?)?;
    Ok(dir)
}

/// Read one run back out of `dir`.
///
/// # Errors
///
/// [`ReplayError`] naming the file that was missing or did not have the recorded shape.
pub fn read_run(dir: &Path) -> Result<RecordedRun, ReplayError> {
    let manifest: serde_json::Value = serde_json::from_str(&read_text(&dir.join("run.json"))?)
        .map_err(|e| ReplayError::Malformed {
            path: dir.join("run.json").display().to_string(),
            detail: e.to_string(),
        })?;
    let bad = |detail: &str| ReplayError::Malformed {
        path: dir.join("run.json").display().to_string(),
        detail: detail.to_owned(),
    };
    if manifest["schema"].as_str() != Some(SCHEMA) {
        return Err(bad("schema is not this layout's"));
    }
    let stage = manifest["stage"].as_str().ok_or_else(|| bad("no stage"))?;
    let run_id = manifest["run_id"]
        .as_str()
        .ok_or_else(|| bad("no run_id"))?
        .to_owned();
    let inputs_digest = manifest["inputs_digest"]
        .as_str()
        .ok_or_else(|| bad("no inputs_digest"))?
        .to_owned();

    let inputs_dir = dir.join("inputs");
    let mut inputs = RunInputs::new(stage);

    let pack_path = one_file(&inputs_dir.join("pack"))?;
    let pack_name = file_name(&pack_path);
    inputs = inputs.with_pack(&pack_name, &read_text(&pack_path)?);

    let artifacts_path = one_file(&inputs_dir.join("artifacts"))?;
    let artifacts_name = file_name(&artifacts_path);
    inputs = inputs.with_artifacts(&artifacts_name, &read_text(&artifacts_path)?);

    let mut catalog: Vec<(String, String)> = Vec::new();
    for path in list_files(&inputs_dir.join("catalog"))? {
        catalog.push((file_name(&path), read_text(&path)?));
    }
    catalog.sort();
    let borrowed: Vec<(&str, &str)> = catalog
        .iter()
        .map(|(n, s)| (n.as_str(), s.as_str()))
        .collect();
    inputs = inputs.with_catalog(&borrowed);

    inputs = inputs.with_facets(&read_text(&inputs_dir.join("facets.toml"))?);
    inputs.context = parse_json(&inputs_dir.join("context.json"))?;
    inputs.artifact_state = parse_json(&inputs_dir.join("artifact-state.json"))?;

    let deviation_path = inputs_dir.join("deviation.json");
    if deviation_path.is_file() {
        let value = parse_json(&deviation_path)?;
        let bad = |d: &str| ReplayError::Malformed {
            path: deviation_path.display().to_string(),
            detail: d.to_owned(),
        };
        let request = &value["request"];
        let kind =
            DeviationKind::from_token(request["kind"].as_str().ok_or_else(|| bad("no kind"))?)
                .ok_or_else(|| {
                    bad("kind is outside ADR-0046's closed skip|downgrade|add vocabulary")
                })?;
        inputs = inputs
            .with_deviation(&RequestDeviation::new(
                request["target"].as_str().ok_or_else(|| bad("no target"))?,
                kind,
                request["claimed_predicate"]
                    .as_str()
                    .ok_or_else(|| bad("no claimed_predicate"))?,
                request["why"].as_str().unwrap_or_default(),
            ))
            .requested_by(
                value["requested_by"]
                    .as_str()
                    .ok_or_else(|| bad("no requested_by"))?,
            );
    }

    let raw = read_text(&dir.join("events.jsonl"))?;
    let mut events = Vec::new();
    for (i, line) in raw.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let value: serde_json::Value =
            serde_json::from_str(line).map_err(|e| ReplayError::Malformed {
                path: dir.join("events.jsonl").display().to_string(),
                detail: format!("line {}: {e}", i + 1),
            })?;
        let wire = value["kind"]
            .as_str()
            .ok_or_else(|| ReplayError::Malformed {
                path: dir.join("events.jsonl").display().to_string(),
                detail: format!("line {}: no kind", i + 1),
            })?;
        let kind = EventKind::ALL
            .into_iter()
            .find(|k| k.wire_name() == wire)
            .ok_or_else(|| ReplayError::Malformed {
                path: dir.join("events.jsonl").display().to_string(),
                detail: format!(
                    "line {}: {wire:?} is not a kind in the closed EventKind table",
                    i + 1
                ),
            })?;
        events.push(Event::new(kind, value["payload"].clone()));
    }

    Ok(RecordedRun {
        inputs,
        events,
        run_id,
        inputs_digest,
    })
}

fn parse_json(path: &Path) -> Result<serde_json::Value, ReplayError> {
    serde_json::from_str(&read_text(path)?).map_err(|e| ReplayError::Malformed {
        path: path.display().to_string(),
        detail: e.to_string(),
    })
}

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into(),
    )
}

fn list_files(dir: &Path) -> Result<Vec<PathBuf>, ReplayError> {
    let entries = std::fs::read_dir(dir).map_err(|e| ReplayError::Io {
        path: dir.display().to_string(),
        detail: e.to_string(),
    })?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    paths.sort();
    // **An empty directory is an error, not an empty success** — the same rule `Catalog::from_dir`
    // states: a replay over an empty input set re-derives an empty DAG and compares equal.
    if paths.is_empty() {
        return Err(ReplayError::Malformed {
            path: dir.display().to_string(),
            detail:
                "holds no files — an empty input set replays to an empty DAG and compares equal"
                    .to_owned(),
        });
    }
    Ok(paths)
}

fn one_file(dir: &Path) -> Result<PathBuf, ReplayError> {
    let mut paths = list_files(dir)?;
    if paths.len() != 1 {
        return Err(ReplayError::Malformed {
            path: dir.display().to_string(),
            detail: format!("expected exactly one file, found {}", paths.len()),
        });
    }
    Ok(paths.remove(0))
}

/// **The replay.** Re-derive the DAG from the logged inputs alone and compare the journal byte for
/// byte with the recorded one.
///
/// Read the module header before reading a green: two of AC-006's five assertions are **not
/// buildable at Slice 3** and this function does not claim them.
///
/// # Errors
///
/// [`ReplayError::Divergent`] carrying [`HiddenInput`] on any mismatch — **never a warning**;
/// [`ReplayError::GitWroteOnReplay`] or [`ReplayError::HarnessDispatchedOnReplay`] when a bound
/// fake's counter is or becomes non-zero; [`ReplayError::Io`] / [`ReplayError::Malformed`] for the
/// log itself.
pub fn replay(dir: &Path, ports: &mut ReplayPorts<'_, '_>) -> Result<ReplayReport, ReplayError> {
    // The counters are read from the caller's fakes, BEFORE the work. A seam that has already
    // written, or a harness that has already dispatched, means the replay is not the isolated
    // re-derivation AC-006 asserts, and it refuses rather than reporting a number.
    let git_before = ports.git.writes_performed();
    if git_before != 0 {
        return Err(ReplayError::GitWroteOnReplay { writes: git_before });
    }
    let dispatches_before = ports.dispatches.load(Ordering::SeqCst);
    if dispatches_before != 0 {
        return Err(ReplayError::HarnessDispatchedOnReplay {
            dispatches: dispatches_before,
        });
    }

    let recorded = read_run(dir)?;
    let rederived = derive(&recorded.inputs)?;

    // ---- byte-identical decisions --------------------------------------------------------------
    let recorded_lines = split_lines(&canonical_events(&recorded.events)?);
    let rederived_lines = split_lines(&canonical_events(&rederived)?);
    for (i, (left, right)) in recorded_lines
        .iter()
        .zip(rederived_lines.iter())
        .enumerate()
    {
        if left != right {
            return Err(ReplayError::Divergent(HiddenInput {
                run_id: recorded.run_id.clone(),
                first_divergent_node: first_divergent_node(left, right, i),
            }));
        }
    }
    if recorded_lines.len() != rederived_lines.len() {
        return Err(ReplayError::Divergent(HiddenInput {
            run_id: recorded.run_id.clone(),
            first_divergent_node: format!(
                "event[{}]: the recorded journal has {} events and the re-derivation produced {}",
                recorded_lines.len().min(rederived_lines.len()),
                recorded_lines.len(),
                rederived_lines.len()
            ),
        }));
    }

    // ---- the recorded EVENT journal is served back to the store --------------------------------
    //
    // **This is not ADR-0105 decision 1, and it said it was until 2026-09-13.** Decision 1's
    // journal is `GitOps`'s journal of git INVOCATIONS, served back through
    // `GitJournal::next_recorded`; this loop is the EVENT log of selection decisions, which is
    // AC-042/AC-006 assertion 2's business. Two different journals, and citing the record clause at
    // a site that does not implement it is how a citation stops being checkable.
    for event in &recorded.events {
        ports
            .store
            .append(event.clone())
            .map_err(|e| ReplayError::StoreRefused {
                detail: format!("{e:?}"),
            })?;
    }

    // ---- and the counters have not moved. ADR-0105 decision 1's zero is READ off `rk-git` -------
    let git_after = ports.git.writes_performed();
    if git_after != 0 {
        return Err(ReplayError::GitWroteOnReplay { writes: git_after });
    }
    let dispatches_after = ports.dispatches.load(Ordering::SeqCst);
    if dispatches_after != 0 {
        return Err(ReplayError::HarnessDispatchedOnReplay {
            dispatches: dispatches_after,
        });
    }

    Ok(ReplayReport {
        run_id: recorded.run_id,
        events_compared: recorded_lines.len(),
        divergences: 0,
        rk_git_writes: git_after,
        harness_dispatches: dispatches_after,
    })
}

fn split_lines(bytes: &[u8]) -> Vec<Vec<u8>> {
    bytes
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(<[u8]>::to_vec)
        .collect()
}

/// Name the **first divergent node**, not merely the first divergent event.
///
/// When both sides are `dag_assembled` the node ids are compared in order and the first that
/// differs is named, because *"`first_divergent_node`"* is what AC-042 says the report carries.
/// Otherwise the event index and its kind are named, which is the finest identity the payload
/// offers.
fn first_divergent_node(left: &[u8], right: &[u8], index: usize) -> String {
    let parse = |b: &[u8]| -> Option<serde_json::Value> { serde_json::from_slice(b).ok() };
    if let (Some(l), Some(r)) = (parse(left), parse(right)) {
        let kind = l["kind"].as_str().unwrap_or("?").to_owned();
        // Nodes first: a divergent node is the cause, and a divergent edge is usually its effect.
        if let (Some(ln), Some(rn)) = (
            l["payload"]["nodes"].as_array(),
            r["payload"]["nodes"].as_array(),
        ) {
            for (a, b) in ln.iter().zip(rn.iter()) {
                if a != b {
                    let id = a["id"].as_str().unwrap_or("?");
                    return format!("event[{index}] {kind}: node {id}");
                }
            }
            if ln.len() != rn.len() {
                return format!(
                    "event[{index}] {kind}: node count {} vs {}",
                    ln.len(),
                    rn.len()
                );
            }
        }
        // Then edges, named by the **nodes** they join — AC-042's word is `first_divergent_node`,
        // so an edge divergence still has to answer in nodes rather than in edge indices.
        if let (Some(le), Some(re)) = (
            l["payload"]["edges"].as_array(),
            r["payload"]["edges"].as_array(),
        ) {
            for (a, b) in le.iter().zip(re.iter()) {
                if a != b {
                    let from = a["from"].as_str().unwrap_or("?");
                    let to = a["to"].as_str().unwrap_or("?");
                    return format!("event[{index}] {kind}: node {from} (edge {from} -> {to})");
                }
            }
            if le.len() != re.len() {
                return format!(
                    "event[{index}] {kind}: edge count {} vs {}",
                    le.len(),
                    re.len()
                );
            }
        }
        // Anything else: the finest identity the payload offers is the event and its kind, and
        // saying so is better than naming a node the divergence is not in.
        return format!("event[{index}] {kind} (divergence is outside `nodes` and `edges`)");
    }
    format!("event[{index}]")
}
