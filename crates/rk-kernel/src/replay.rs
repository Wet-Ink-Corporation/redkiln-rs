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
//! | # | assertion | state at Slice 4i, 2026-09-14 |
//! | --- | --- | --- |
//! | 1 | every port bound to its fake | **a type fact, not a measurement** — [`ReplayPorts`] has no optional field, so an unbound port does not compile. This module reports **no list of bound ports**: a `Vec` of string literals the replay prints about itself is the very shape `s7b`:1018-1023 forbids, and it was one until 2026-09-13 |
//! | 2 | byte-identical decisions | **measured** under `rk_store::canon` |
//! | 3 | byte-identical **next action** | **MEASURED**, as of 2026-09-14. [`crate::workflow::fsm::next_action`]'s canonical bytes, recomputed from the re-derived journal and compared against the value `run.json` **committed at record time**. Diverging is [`ReplayError::NextActionDiverged`], never a warning |
//! | 4 | zero `rk-git` writes | **measured against `rk-git`'s OWN counter** — [`rk_git::gitops::GitOps::writes_performed`], read before and after. ADR-0105 decision 1's named instrument, not a seam this crate invented. `git` is `&mut` as of 2026-09-14 (residual 10h), so the absence is over a seam the replay COULD have written through |
//! | 5 | zero harness dispatches **with the terminal state matched** | **dispatch half measured** through the caller's own counter ([`ReplayPorts::dispatches`]), read before and after; **terminal-state half MEASURED** as of 2026-09-14, the same way as row 3. **And as of Slice 4i the GIVEN is satisfied**: the recorded run dispatches N ≥ 1 and the replay serves all N from the journal — see the next section |
//!
//! **The sentence that stood in rows 3 and 5 until 2026-09-14 was:** *"**NOT BUILDABLE — there is
//! no FSM.** The workflow/activity split, the memo, retry and the runtime are §7b.10's Slice 4."*
//! The FSM is now on disk at `src/workflow/fsm.rs` and both rows are measured. What that sentence
//! got right, and what survives it, is the next paragraph.
//!
//! # AC-008's GIVEN IS *"a run that dispatched N activities"*, AND N WAS 0 UNTIL SLICE 4i
//!
//! **The section this replaces said, and was right to say:**
//!
//! > [measured 2026-09-14: the committed golden log holds **4** events — `stage_transition`,
//! > `dag_assembled`, `gate_opened`, `dag_amended` — and **zero** `dispatch` and
//! > `activity_committed` events.] So *"all N results served from the journal"* is satisfied over
//! > an empty N, and the **dispatch half of AC-008 is vacuous** … **A recording with dispatches
//! > needs a dispatcher, and there is none.**
//!
//! Slice 4h built the dispatcher — [`crate::runtime::Runtime::run_activity`], through
//! `rk_ports::harness::Harness`, appending `dispatch` and `activity_committed`. Slice 4i makes
//! [`record`] drive it. **What changed, in four parts:**
//!
//! 1. [`record`] takes [`rk_ports::root::Ports`] and runs the FSM loop — `next_action` → if
//!    `Dispatch` { obtain } → fold → repeat, until the next action is not a dispatch. The recorded
//!    journal therefore carries a real `dispatch` and a real `activity_committed` per activation,
//!    and the terminal state the manifest commits is `stage_complete` rather than
//!    `awaiting_dispatch`.
//! 2. [`replay`] runs **the same loop** with the recorded journal as the result source: every row
//!    is served once, in order, and **nothing is dispatched**. An activation the journal does not
//!    carry is [`ReplayError::JournalHasNoResult`] — a typed divergence, never a skipped step.
//!    That is *"all N results served from the journal, zero harness dispatches"* as an
//!    architecture rather than as an assertion (design-a §3.2).
//! 3. The [`rk_types::activity::ActivitySpec`] is a **RECORDED INPUT** — `inputs/specs.json`, in
//!    the layout below, folded into `run_id` and `inputs_digest` like every other input. See
//!    [`RecordedSpec`] for the measurement behind that: the five fields are the **binder's**
//!    output, the binder is unbuilt, and **a record routing spec assembly is OWED**.
//! 4. Every earlier recorded `run_id` moved again, so `implement-d8401695754074c7` is gone and the
//!    fixture directory it names is re-blessed through `bless_the_committed_golden_log` — never
//!    hand-edited.
//!
//! 🛑 **WHAT IS STILL NOT DISCHARGED, so a green here is not read as more than it is.** AC-008's
//! dispatches are the **fake** `Harness`'s: a scripted responder the test binds, not a model. The
//! criterion asks for exactly that (*"a replay test with a scripted-responder `Harness` fake that
//! fails the test on any `dispatch` call"*), so this is coverage rather than a shortfall — but the
//! N being served is N **fake** results, and no line of this module should be read as evidence
//! about a real adapter, which is 7c's. AC-007's divergence detection, AC-161's route *"from git
//! alone"* and AC-164's `doctor` half are each unchanged and each still named where they are owed.
//!
//! **Both counters are read from the caller's fakes, before and after**, so neither is a constant
//! this module prints about itself — which is exactly what `s7b`:1018-1023 says the positive control
//! exists to prevent. The third token, `terminal_state=matched`, had **no** control until
//! 2026-09-14 (gap G-1); `golden_log_replay_terminal_state_mismatch_is_red` is now it.
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
//!     run.json                    { schema, run_id, stage, inputs_digest, event_count, producer,
//!                                   terminal_state, next_action }   ← the last two NEW at 4f
//!     inputs/
//!       pack/<name>.toml          the pack, verbatim
//!       catalog/<name>.toml       one file per activity, verbatim
//!       artifacts/<name>.toml     the artifact-kind registry, verbatim
//!       facets.toml               the facet registry, verbatim
//!       context.json              the item's context digest preimage (canonical)
//!       artifact-state.json       the item's artifact digests preimage (canonical)
//!       runtime.json              NEW at 4f: { tree_hash, deadline, entered_from, successor,
//!                                   results_at }   ← `results_at` NEW at 4i
//!       specs.json                NEW at 4i: { "<dag node id>": { activity_id_at_version,
//!                                   card_digest, tools, schema, timeout_secs, inputs } }
//!       deviation.json            OPTIONAL: the RequestDeviation and its requesting activity
//!     events.jsonl                one canonical JSON object per line, LF-terminated
//! ```
//!
//! **`specs.json` is REQUIRED, and an empty object is how a run with no dispatchable node says
//! so.** A missing file is refused by [`read_run`] on `terminal_state`'s exact warrant: a log
//! without it records no dispatches, and replaying one would satisfy AC-008's GIVEN over an empty
//! N — the vacuous green Slice 4i exists to remove.
//!
//! **`run_id` is derived, never stamped.** It is a content hash of the inputs, so it needs no clock,
//! no counter and no random source — the three things AC-041 forbids and AC-043 exists to catch
//! downstream — and a replay recomputes it rather than reading it back. **Nothing in the layout is
//! a CLOCK READING**, for the same reason: a `now()` anywhere would make two runs of identical
//! inputs produce different bytes and turn AC-042's byte-identity into a comparison of clocks.
//! `runtime.json`'s `deadline` is not an exception and the distinction is the whole reason it is
//! an **input**: it is a value the caller recorded, read back on replay, and never read from a
//! clock by anything in this crate. **`results_at` is the second value of that shape, added at
//! 4i** — the `at` every `activity_committed` carries. It is supplied, never defaulted, and
//! **refused when absent**, because `rk_types::time::Timestamp` *"has no `now()`, deliberately"*
//! and a zero `at` is a commit stamped at the epoch. The sentence that stood here until 4i read
//! *"nothing in the layout carries a timestamp"*, which was true of a journal with no commits in
//! it; the proposition it was **for** is the one above, and it is unchanged.
//!
//! **`inputs/runtime.json` participates in `run_id`, and that is the point.** AC-042's THEN is
//! *"re-derives the DAG from the **logged inputs alone**"*; a tree hash or a deadline the replay
//! had to be told out of band would be exactly the hidden input [`HiddenInput`] exists to report.
//! Adding it moved every recorded `run_id`, which is why the Slice-3 fixture directory
//! `implement-ea970997d42e7e04` is gone and `implement-d8401695754074c7` stands in its place —
//! re-blessed through the named `bless_the_committed_golden_log` act, never hand-edited.
//!
//! **A log written before 2026-09-14 is REFUSED, not read with defaults.** [`read_run`] requires
//! both new manifest members: a `None` default would let a replay succeed with nothing to compare
//! the FSM residue against, which is a green whose terminal-state half asserts exactly what AC-008
//! warns about.
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
use rk_ports::root::Ports;
use rk_store::memo::IdempotencyKey;
use rk_types::activity::ActivitySpec;
use rk_types::event::Event;
use rk_types::event_kind::{EventKind, ReplayDivergence};
use rk_types::time::Timestamp;
use rk_types::typed::SchemaId;

use crate::catalog::Catalog;
use crate::registry::Registry;
use crate::runtime::{Runtime, RuntimeError};
use crate::select::{
    ArtifactRegistry, DeviationKind, Item, Pack, RequestDeviation, SelectError, Selection, select,
    step7,
};
use crate::workflow::breaker::JournaledNow;
use crate::workflow::envelope::AttemptCap;
use crate::workflow::fsm::{self, Divergence, FsmContext, FsmError, NextAction};

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
    /// **AC-006 assertion 3.** *"…the sequence of decisions and the **next action** are
    /// byte-identical to the recorded ones."* The recorded side is read back out of `run.json` —
    /// a value written at record time and committed — and never recomputed from the same bytes.
    ///
    /// It carries [`rk_types::event_kind::ReplayDivergence`] and not a pair of strings, because
    /// AC-007's THEN is *"the run halts with a typed `ReplayDivergence { activity_id, recorded,
    /// recomputed }`"*; [`Divergence::halt`] is what mints it, and it consumes the recomputed
    /// value so no continuation on it is reachable.
    NextActionDiverged(ReplayDivergence),
    /// **AC-008's terminal-state half.** *"…a test that separately asserts the terminal state
    /// matches the record — so a kernel that does nothing at all fails the terminal-state half."*
    /// A fold that returns [`crate::workflow::fsm::TerminalState::NotStarted`] for everything
    /// produces `{"terminal":"not_started"}` here and the manifest says otherwise.
    TerminalStateDiverged(ReplayDivergence),
    /// **The runtime boundary refused a dispatch the RECORDING had to make.** A recording drives
    /// [`crate::workflow::fsm::NextAction::Dispatch`] through [`crate::runtime::Runtime`], so
    /// AC-010's key refusal, the store's refusal and the fold's all reach this module — carried
    /// whole rather than flattened, because each names a different repair.
    Runtime(RuntimeError),
    /// A recorded dispatch produced no `activity_committed`, so the FSM would ask for the same
    /// activation for ever. **Refused, never looped**: a run whose activity failed is not a run
    /// this stage can record, and saying so is better than a journal that grows without end.
    NotCommitted {
        step_id: String,
        activity_id_at_version: String,
    },
    /// **The replay asked the recorded journal for an activation it does not carry.** This is
    /// AC-008's *"all N results served from the journal"* failing on the side that matters: the
    /// FSM wanted a result, the recording has none, and the replay refuses rather than skipping
    /// the step silently and folding on.
    JournalHasNoResult {
        step_id: String,
        activity_id_at_version: String,
        /// What the journal offered instead, when it offered anything — so the halt names the
        /// divergence rather than merely the absence.
        served_instead: Option<String>,
    },
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
            Self::NextActionDiverged(d) => write!(
                f,
                "the recomputed next action is not the recorded one (AC-006 assertion 3). \
                 activity_id={:?} recorded={} recomputed={}. The recorded side was read back out \
                 of `run.json`, so this is a comparison against a committed value and not two \
                 folds of the same bytes",
                d.activity_id, d.recorded, d.recomputed
            ),
            Self::TerminalStateDiverged(d) => write!(
                f,
                "the recomputed terminal state is not the recorded one (AC-008). activity_id={:?} \
                 recorded={} recomputed={}. A kernel that does nothing at all lands on \
                 `not_started` here and fails this half, which is what the criterion exists to \
                 catch",
                d.activity_id, d.recorded, d.recomputed
            ),
            Self::Runtime(e) => write!(f, "the runtime boundary refused a dispatch: {e}"),
            Self::NotCommitted {
                step_id,
                activity_id_at_version,
            } => write!(
                f,
                "the dispatch of `{step_id}` ({activity_id_at_version}) returned without an \
                 `activity_committed`, so the FSM would select it again for ever. A recording \
                 refuses rather than looping"
            ),
            Self::JournalHasNoResult {
                step_id,
                activity_id_at_version,
                served_instead,
            } => write!(
                f,
                "the recorded journal carries no result for `{step_id}` \
                 ({activity_id_at_version}), and the re-derivation asked for one. AC-008's THEN is \
                 that **all N results are served from the journal**; a replay that skipped the \
                 step instead would serve N-1 and still report a green. The journal offered {}",
                served_instead
                    .as_deref()
                    .map_or_else(|| "nothing further".to_owned(), |s| format!("`{s}`"))
            ),
        }
    }
}

impl std::error::Error for ReplayError {}

impl From<RuntimeError> for ReplayError {
    fn from(e: RuntimeError) -> Self {
        Self::Runtime(e)
    }
}

impl From<FsmError> for ReplayError {
    /// **Mapped onto [`ReplayError::Malformed`], and no new variant is minted for it.** A journal
    /// the fold cannot read is a malformed log — the same defect `Malformed` already names — and
    /// the payload member that refused travels in `detail` rather than being flattened into a
    /// third divergence variant. The two variants this slice adds are the two AC-006 assertion 3
    /// and AC-008 name; a fold refusal is neither.
    fn from(e: FsmError) -> Self {
        Self::Malformed {
            path: "events.jsonl".to_owned(),
            detail: format!("{} ({e})", e.as_str()),
        }
    }
}

impl From<rk_store::canon::CanonError> for ReplayError {
    fn from(e: rk_store::canon::CanonError) -> Self {
        Self::Malformed {
            path: "<canonical>".to_owned(),
            detail: e.to_string(),
        }
    }
}

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
    ///
    /// **`&mut`, as of 2026-09-14 (residual 10h).** It was `&GitOps<'j>` until then, and that
    /// shared reference made the binding a *promise*: `rk_git::gitops::GitOps::run` — the one
    /// method that can move `writes_performed()` — takes `&mut self`, so a shared borrow could
    /// never reach it and the replay was structurally incapable of the write it asserts the
    /// absence of. An absence measured over a seam that cannot act is not a measurement. With the
    /// exclusive borrow the replay *could* call `run` and does not, and
    /// `golden_log_replay_writing_fake_is_red` is what shows the counter moves when something
    /// does.
    pub git: &'a mut GitOps<'j>,
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
    /// The canonical bytes of the **recomputed** terminal state, as UTF-8.
    ///
    /// A `String` and not a `bool`, because `terminal_state=matched` is a token the test prints
    /// **about itself** (§7b.14:1547-1553) and a caller that can only see a boolean cannot print
    /// what it compared. The recorded side it was compared against is
    /// [`RecordedRun::terminal_state`], read back out of `run.json`.
    pub terminal_state: String,
    /// The canonical bytes of the **recomputed** next action, as UTF-8 — AC-006 assertion 3.
    pub next_action: String,
}

/// The four values [`crate::workflow::fsm::next_action`] needs, **recorded as part of the input
/// set**.
///
/// 🛑 **They are inputs, not context, and that is the whole point.** AC-042's THEN is that the run
/// *"re-derives the DAG from the **logged inputs alone**"*; a tree hash or a deadline the replay
/// had to be told out of band would be exactly the hidden input `HiddenInput` exists to report.
/// They are written to `inputs/runtime.json` and they participate in `run_id`, on AC-064's own
/// model for the deviation record — *"the deviation record is part of the replay input set, so an
/// amended DAG is not a hidden input."*
///
/// **Nothing here is defaulted.** [`RunInputs`] carries an `Option<RuntimeContext>` and
/// [`record`] refuses a run without one rather than substituting an empty tree hash or a zero
/// deadline: `select()` cannot mint either (AC-041 bans the clock and the filesystem read), and a
/// zero deadline is a gate that expired at the epoch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext {
    tree_hash: String,
    deadline: i64,
    entered_from: Option<String>,
    successor: Option<String>,
    results_at: Option<i64>,
}

impl RuntimeContext {
    /// The only constructor, named `supplied` on [`crate::workflow::fsm::FsmContext`]'s own model:
    /// every call site is transcribing something this crate did not derive.
    #[must_use]
    pub fn supplied(tree_hash: &str, deadline: i64) -> Self {
        Self {
            tree_hash: tree_hash.to_owned(),
            deadline,
            entered_from: None,
            successor: None,
            results_at: None,
        }
    }

    /// The journaled instant every `activity_committed` this run writes carries — **a recorded
    /// input, exactly as [`Self::deadline`] is, and for the identical reason.**
    ///
    /// `rk_types::time::Timestamp` *"has no `now()`, deliberately"*, so an `at` member could only
    /// have come from data. Recording it keeps the module header's *"nothing in the layout carries
    /// a timestamp"* true in the sense that sentence means: no value here is a **clock reading**,
    /// and two runs of identical inputs produce identical bytes. It is **never defaulted** — a
    /// recording that needs one and was given none is refused, because a zero `at` is a commit
    /// stamped at the epoch.
    #[must_use]
    pub const fn results_at(mut self, unix_seconds: i64) -> Self {
        self.results_at = Some(unix_seconds);
        self
    }

    /// The stage this run entered FROM, if the caller recorded one.
    ///
    /// Optional because **this system models no stage ordering** — `spikes/p9/packs/incident.toml`
    /// :1-2, in its own first comment: *"Stage ORDER … is not modelled by the selection
    /// function."* With none supplied the stage-entry event carries no `from` member, and the
    /// fold's own `optional_str` reading is what makes that legible rather than a silent zero.
    #[must_use]
    pub fn entered_from(mut self, stage: &str) -> Self {
        self.entered_from = Some(stage.to_owned());
        self
    }

    /// The successor stage, for [`crate::workflow::fsm::NextAction::AdvanceStage`]. Same reason as
    /// [`Self::entered_from`]: supplied, never invented.
    #[must_use]
    pub fn advances_to(mut self, stage: &str) -> Self {
        self.successor = Some(stage.to_owned());
        self
    }

    #[must_use]
    pub fn tree_hash(&self) -> &str {
        &self.tree_hash
    }

    #[must_use]
    pub const fn deadline(&self) -> i64 {
        self.deadline
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "deadline": self.deadline,
            "entered_from": self.entered_from,
            "results_at": self.results_at,
            "successor": self.successor,
            "tree_hash": self.tree_hash,
        })
    }

    /// The journaled instant, **refused rather than defaulted** when absent.
    fn require_results_at(&self) -> Result<JournaledNow, ReplayError> {
        let secs = self.results_at.ok_or_else(|| ReplayError::Malformed {
            path: "inputs/runtime.json".to_owned(),
            detail:
                "no `results_at` was recorded, and the instant an `activity_committed` carries \
                     cannot be defaulted here: this crate has no clock (AC-001) and a zero `at` is \
                     a commit stamped at the epoch"
                    .to_owned(),
        })?;
        Ok(JournaledNow::from_journal(Timestamp::from_unix_seconds(
            secs,
        )))
    }
}

/// **One node's dispatch binding, as a RECORDED INPUT** — the `ActivitySpec` AC-008's dispatches
/// need, plus the activation identity the idempotency key's second part is.
///
/// 🛑 **Why it is an input and not something this crate resolves.** `rk_types::activity::
/// ActivitySpec` has five fields and **this tree resolves none of them from a DAG node**
/// [measured 2026-09-14: `card_digest`, `tools` and `schema` appear in **zero** catalog fixtures;
/// `timeout_secs` is declared in 10 and parsed by nothing — `grep -c timeout_secs
/// crates/rk-kernel/src/catalog.rs` → 0 — and one of the ten is `"${toolchain.test_timeout}"`, a
/// substitution no `Toolchain` adapter exists to resolve at 7b]. **The spec is the binder's
/// output, and the binder is unbuilt**, so at Step 7b it is supplied as a recorded input and
/// **a record routing spec assembly is OWED**.
///
/// Nothing is invented to fill the gap: no catalog key is read that the loader does not already
/// parse, and no field is defaulted. What makes this lawful under AC-042 — *"re-derives the DAG
/// from the **logged inputs alone**"* — is that the binding is **logged**: it lives in
/// `inputs/specs.json`, it participates in `run_id`, and a replay reads it back rather than being
/// told it out of band. That is AC-064's own model for the deviation record.
///
/// **`activity_id_at_version` is here for the same reason and is NOT a sixth spec field.** The
/// join between a DAG node and `crate::catalog::Activity::id_at_version` has three candidate homes
/// and `crate::workflow::fsm`'s header records that choosing one is a decision this stage may not
/// take; the binding transcribes it instead, and the record naming the join's home is still OWED.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordedSpec {
    activity_id_at_version: String,
    spec: ActivitySpec,
}

impl RecordedSpec {
    /// The only constructor, named `supplied` on [`RuntimeContext::supplied`]'s model: every call
    /// site is transcribing something this crate did not derive.
    #[must_use]
    pub fn supplied(activity_id_at_version: &str, spec: ActivitySpec) -> Self {
        Self {
            activity_id_at_version: activity_id_at_version.to_owned(),
            spec,
        }
    }

    /// The activation identity — the idempotency key's second part (ADR-0097 decision 2).
    #[must_use]
    pub fn activity_id_at_version(&self) -> &str {
        &self.activity_id_at_version
    }

    /// The spec the `Harness` port is handed.
    #[must_use]
    pub const fn spec(&self) -> &ActivitySpec {
        &self.spec
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "activity_id_at_version": self.activity_id_at_version,
            "card_digest": self.spec.card_digest.as_str(),
            "inputs": self
                .spec
                .inputs
                .iter()
                .map(|(name, value)| serde_json::json!({ "name": name, "value": value }))
                .collect::<Vec<_>>(),
            "schema": self.spec.schema.as_str(),
            "timeout_secs": self.spec.timeout_secs,
            "tools": self.spec.tools,
        })
    }

    /// Read one binding back. **Every member is required**; none is defaulted, for the reason the
    /// type's own doc gives.
    fn from_json(node: &str, value: &serde_json::Value) -> Result<Self, ReplayError> {
        let bad = |detail: String| ReplayError::Malformed {
            path: "inputs/specs.json".to_owned(),
            detail: format!("{node}: {detail}"),
        };
        let member = |name: &str| -> Result<&serde_json::Value, ReplayError> {
            value.get(name).ok_or_else(|| bad(format!("no `{name}`")))
        };
        let text = |name: &str| -> Result<String, ReplayError> {
            member(name)?
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| bad(format!("`{name}` is not a string")))
        };
        let card_digest = rk_types::digest::Digest::from_hex(&text("card_digest")?)
            .map_err(|e| bad(format!("`card_digest` is not a digest: {e:?}")))?;
        let tools = member("tools")?
            .as_array()
            .ok_or_else(|| bad("`tools` is not an array".to_owned()))?
            .iter()
            .map(|t| {
                t.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| bad("a `tools` member is not a string".to_owned()))
            })
            .collect::<Result<Vec<String>, ReplayError>>()?;
        let timeout_secs = member("timeout_secs")?.as_u64().ok_or_else(|| {
            bad("`timeout_secs` is absent or negative — never a silent 0".to_owned())
        })?;
        let inputs = member("inputs")?
            .as_array()
            .ok_or_else(|| bad("`inputs` is not an array".to_owned()))?
            .iter()
            .map(|row| {
                let name = row
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .ok_or_else(|| bad("an `inputs` row has no `name`".to_owned()))?;
                let value = row
                    .get("value")
                    .ok_or_else(|| bad("an `inputs` row has no `value`".to_owned()))?;
                Ok((name.to_owned(), value.clone()))
            })
            .collect::<Result<Vec<(String, serde_json::Value)>, ReplayError>>()?;
        Ok(Self {
            activity_id_at_version: text("activity_id_at_version")?,
            spec: ActivitySpec {
                card_digest,
                tools,
                schema: SchemaId::new(text("schema")?),
                timeout_secs,
                inputs,
            },
        })
    }
}

/// The four digests' preimages, plus the optional deviation and the per-node dispatch bindings, as
/// one recorded value.
///
/// **`Eq` is not derived**, and the reason is [`RecordedSpec`]'s: it carries an
/// `rk_types::activity::ActivitySpec`, which is `PartialEq` and not `Eq` because a spec's `inputs`
/// are `serde_json::Value`s. The comparison this module makes is the **canonical byte** one, and
/// deriving `Eq` here would offer one AC-042 does not use — `RecordedRun`'s own doc already states
/// the rule.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RunInputs {
    stage: String,
    pack: (String, String),
    catalog: Vec<(String, String)>,
    artifacts: (String, String),
    facets: String,
    context: serde_json::Value,
    artifact_state: serde_json::Value,
    deviation: Option<(RequestDeviation, String)>,
    runtime: Option<RuntimeContext>,
    /// `(DAG node id, binding)`, sorted — so the log is a function of the bindings and not of the
    /// caller's argument order, exactly as [`Self::with_catalog`] sorts.
    specs: Vec<(String, RecordedSpec)>,
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

    /// Record the [`RuntimeContext`] as part of the run's input set.
    #[must_use]
    pub fn with_runtime(mut self, runtime: RuntimeContext) -> Self {
        self.runtime = Some(runtime);
        self
    }

    /// Record one DAG node's dispatch binding as part of the run's input set.
    ///
    /// **This is what makes AC-008's GIVEN — *"a run that dispatched N activities"* — reachable at
    /// Step 7b.** See [`RecordedSpec`] for why the binding is a logged input rather than something
    /// this crate resolves, and for the record that is OWED.
    #[must_use]
    pub fn with_spec(mut self, node_id: &str, binding: RecordedSpec) -> Self {
        self.specs.retain(|(id, _)| id != node_id);
        self.specs.push((node_id.to_owned(), binding));
        self.specs.sort_by(|a, b| a.0.cmp(&b.0));
        self
    }

    /// The recorded bindings, in node order.
    #[must_use]
    pub fn specs(&self) -> &[(String, RecordedSpec)] {
        &self.specs
    }

    /// The `(node id, activity_id@version)` list [`crate::workflow::fsm::FsmContext`] is supplied.
    fn activations(&self) -> Vec<(String, String)> {
        self.specs
            .iter()
            .map(|(id, b)| (id.clone(), b.activity_id_at_version.clone()))
            .collect()
    }

    /// The binding for one DAG node, **refused rather than defaulted** when absent: an empty spec
    /// is a dispatch of a card nobody wrote (`crate::runtime::RuntimeError::SpecNotSupplied`'s own
    /// words, one module over).
    fn spec_for(&self, node_id: &str) -> Result<&RecordedSpec, ReplayError> {
        self.specs
            .iter()
            .find(|(id, _)| id == node_id)
            .map(|(_, b)| b)
            .ok_or_else(|| ReplayError::Malformed {
                path: "inputs/specs.json".to_owned(),
                detail: format!(
                    "the FSM selected `{node_id}` for dispatch and no binding was recorded for it. \
                     The spec is the binder's output, the binder is unbuilt, and this stage \
                     supplies it as a recorded input rather than assembling half of one"
                ),
            })
    }

    /// The recorded runtime context, **refused rather than defaulted** when absent.
    ///
    /// # Errors
    ///
    /// [`ReplayError::Malformed`] naming `inputs/runtime.json`.
    fn runtime(&self) -> Result<&RuntimeContext, ReplayError> {
        self.runtime.as_ref().ok_or_else(|| ReplayError::Malformed {
            path: "inputs/runtime.json".to_owned(),
            detail: "no runtime context was recorded, and neither the tree hash nor the gate \
                     deadline can be defaulted here: `select()` is forbidden both a git read and \
                     a clock (AC-041), and a zero deadline is a gate that expired at the epoch"
                .to_owned(),
        })
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
            "runtime": self.runtime.as_ref().map(RuntimeContext::to_json),
            // NEW at Slice 4i. The bindings participate in `run_id` like every other input, which
            // is exactly what keeps AC-042's *"from the logged inputs alone"* true of a run that
            // dispatches: a spec the replay had to be told out of band would be the hidden input
            // `HiddenInput` exists to report.
            "specs": self.specs.iter().map(|(id, b)| serde_json::json!({
                "node": id, "binding": b.to_json(),
            })).collect::<Vec<_>>(),
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
    terminal_state: String,
    next_action: String,
}

impl RecordedRun {
    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// The canonical bytes of the terminal state **as the recording computed them**, as UTF-8.
    ///
    /// This is the value `run.json` carries and the value a replay compares against. It is never
    /// recomputed from the recorded journal during a replay: two folds of byte-identical journals
    /// are equal by construction, and AC-008's whole content is that *"a kernel that does nothing
    /// at all fails the terminal-state half."*
    #[must_use]
    pub fn terminal_state(&self) -> &str {
        &self.terminal_state
    }

    /// The canonical bytes of the next action as the recording computed them — AC-006 assertion 3.
    #[must_use]
    pub fn next_action(&self) -> &str {
        &self.next_action
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

/// **Where a dispatched activity's result comes from — the ONE thing that differs between a
/// recording and a replay.**
///
/// design-a §3.2's own sentence, which this enum is: *"the whole runtime is therefore a pure fold
/// the replay can re-run … where `source` is the only thing that differs between a recording (a
/// `Harness`) and a replay (the recorded journal). That is AC-008's **all N results served from the
/// journal, zero harness dispatches** stated as an architecture rather than as an assertion."*
///
/// The loop above it is identical in both directions — `next_action` → `Dispatch` → obtain → fold
/// → repeat — so a replay cannot agree with a recording because the two were written to agree.
enum Results<'a> {
    /// **record**: the caller's bound ports. [`crate::runtime::Runtime::run_activity`] dispatches
    /// through `rk_ports::harness::Harness` and appends, and the events collected are the ones the
    /// store actually received — read back off the port, never reconstructed here to match.
    Dispatched(&'a Runtime),
    /// **replay**: the recorded journal, and **nothing is dispatched**.
    Journal(ServedJournal<'a>),
}

/// The recorded `dispatch` / `activity_committed` rows, served back in order and **once each**.
///
/// A cursor rather than a search, because *"all N results served from the journal"* is a statement
/// about the same N in the same order: a lookup that re-served row 1 for attempt 2 would make a
/// journal with one recorded attempt satisfy a re-derivation that wanted three.
struct ServedJournal<'a> {
    rows: Vec<&'a Event>,
    next: usize,
}

impl<'a> ServedJournal<'a> {
    fn of(events: &'a [Event]) -> Self {
        Self {
            rows: events
                .iter()
                .filter(|e| matches!(e.kind, EventKind::Dispatch | EventKind::ActivityCommitted))
                .collect(),
            next: 0,
        }
    }

    /// Which activation a served row belongs to, read off the payload the fold already reads.
    fn activation_of(event: &Event) -> Option<(&str, &str)> {
        Some((
            event.payload.get("item")?.as_str()?,
            event.payload.get("activity_id_at_version")?.as_str()?,
        ))
    }

    /// Serve one activation's recorded rows: everything from the cursor that belongs to it, up to
    /// and including its `activity_committed`.
    ///
    /// **A divergence, never a silent skip.** If the cursor is exhausted, or the next row belongs
    /// to a different activation, or the run of rows carries no commit, this refuses and names
    /// what the journal offered instead.
    fn serve(&mut self, key: &IdempotencyKey, step_id: &str) -> Result<Vec<Event>, ReplayError> {
        let mut out: Vec<Event> = Vec::new();
        let mut committed = false;
        while let Some(event) = self.rows.get(self.next) {
            let Some((item, activation)) = Self::activation_of(event) else {
                break;
            };
            if item != key.item || activation != key.activity_id_at_version {
                break;
            }
            out.push((*event).clone());
            self.next += 1;
            if event.kind == EventKind::ActivityCommitted {
                committed = true;
                break;
            }
        }
        if committed {
            return Ok(out);
        }
        let served_instead = self.rows.get(self.next).map(|event| {
            Self::activation_of(event).map_or_else(
                || event.kind.wire_name().to_owned(),
                |(item, activation)| format!("{} for {item}/{activation}", event.kind.wire_name()),
            )
        });
        Err(ReplayError::JournalHasNoResult {
            step_id: step_id.to_owned(),
            activity_id_at_version: key.activity_id_at_version.clone(),
            served_instead,
        })
    }
}

/// **Run selection over the logged inputs, then drive the FSM to a non-dispatch action.**
///
/// This is the one place the journal is derived, and both [`record`] and [`replay`] go through it —
/// so a replay cannot agree with a recording because the two were written to agree.
///
/// # Errors
///
/// [`ReplayError`] for a malformed input, a selection that could not resolve, a dispatch the
/// boundary refused, or an activation the recorded journal does not carry.
fn derive(
    inputs: &RunInputs,
    results: &mut Results<'_>,
) -> Result<(Selection, Item, Vec<Event>), ReplayError> {
    let (pack, catalog, artifacts, facets, item) = inputs.build()?;
    let mut selection: Selection =
        select(&pack, &catalog, &artifacts, &facets, &inputs.stage, &item)?;
    let runtime = inputs.runtime()?;

    // **THE RUNTIME'S OWN FIRST DURABLE ACT, and it is first in the journal because it happens
    // first.** Entering a stage is a tier-1, trailer-carried `stage_transition` (design §5;
    // `EventKind::tier`), and it needs no port, no clock and no dispatcher.
    let mut events = vec![stage_entry(&inputs.stage, runtime)];
    events.extend(selection.journal());

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
        // 🛑 **THE RUN CONTINUES ON THE AMENDED SELECTION, and until Slice 4i nothing in this
        // function noticed the difference.** `step7`'s accepted branch *"returns a second
        // `Selection` produced by re-running steps 2-5"*, and before there was a dispatch loop the
        // only consumer of a selection here was `fsm_residue`'s single `next_action` — which, on
        // this fixture, answers the same either way. A loop is not so forgiving: driving the BASE
        // selection would dispatch the node the accepted deviation REMOVED, and the recorded
        // journal would show a run doing exactly what its own `dag_amended` says it did not do.
        // AC-064's *"the deviation record is part of the replay input set"* is about a replay
        // re-deriving the amendment, not about a run ignoring it.
        if let Some(amended) = outcome.selection() {
            selection = amended.clone();
        }
    }

    // ---- the envelope a RECORDING dispatches under, read before the loop -----------------------
    //
    // `N` is the PACK's (AC-029: *"declared, never defaulted in code"*) and the journaled instant
    // is `inputs/runtime.json`'s. Both are read here rather than inside the loop so a recording
    // that could satisfy neither refuses before it dispatches anything, and both are **logged
    // inputs**: nothing in this function chooses a number.
    let dispatch_envelope = match results {
        Results::Dispatched(rt) => {
            let cap = pack
                .require_attempt_cap()
                .map_err(|e| ReplayError::Malformed {
                    path: format!("inputs/pack/{}", inputs.pack.0),
                    detail: e.to_string(),
                })?;
            let now = runtime.require_results_at()?;
            // The pre-events go through the port too, so the runtime's own fold reads the journal
            // the run actually has rather than a truncated one.
            for event in &events {
                rt.append(event.clone())?;
            }
            Some((cap, now))
        }
        Results::Journal(_) => None,
    };

    // ---- THE LOOP. `next_action` -> if `Dispatch` { obtain } -> fold -> repeat -----------------
    //
    // AC-008's *"the replay reaches the recorded terminal state with all N results served from the
    // journal, zero harness dispatches"* — as an architecture. The loop is the same in both
    // directions; only [`Results`] differs.
    let activations = inputs.activations();
    loop {
        let state = fsm::fold(item.id(), &inputs.stage, &events)?;
        let ctx = fsm_context(runtime, &item, &activations);
        let action = fsm::next_action(&selection, &state, &ctx)?;
        let NextAction::Dispatch { key, step_id } = &action else {
            break;
        };
        let binding = inputs.spec_for(step_id)?;
        let produced = match (&mut *results, dispatch_envelope) {
            (Results::Dispatched(rt), Some((cap, now))) => {
                dispatch_through(rt, key, step_id, binding, cap, now)?
            }
            // Unreachable by construction: the envelope is `Some` exactly when the source is
            // `Dispatched`. Refused rather than unwrapped, because an `expect` here would be a
            // panic on a state a later edit could make reachable.
            (Results::Dispatched(_), None) => {
                return Err(ReplayError::Malformed {
                    path: "inputs/runtime.json".to_owned(),
                    detail: "a recording reached a dispatch with no envelope".to_owned(),
                });
            }
            (Results::Journal(journal), _) => journal.serve(key, step_id)?,
        };
        events.extend(produced);
    }
    Ok((selection, item, events))
}

/// One dispatch through the bound ports, and **the events are read back off the store**.
///
/// The journal is queried before and after, and the delta is what the boundary appended. Nothing
/// here re-builds a `dispatch` or an `activity_committed` to match what `crate::runtime` writes:
/// a recording that constructed its own copy of the rows would be the two-written-to-agree defect
/// this module's header names, one level down.
fn dispatch_through(
    rt: &Runtime,
    key: &IdempotencyKey,
    step_id: &str,
    binding: &RecordedSpec,
    cap: AttemptCap,
    now: JournaledNow,
) -> Result<Vec<Event>, ReplayError> {
    let before = rt.journal(&key.item)?.len();
    let report = rt.run_activity(
        &key.item,
        &key.activity_id_at_version,
        step_id,
        binding.spec(),
        cap,
        now,
    )?;
    if !report.committed() {
        return Err(ReplayError::NotCommitted {
            step_id: step_id.to_owned(),
            activity_id_at_version: key.activity_id_at_version.clone(),
        });
    }
    Ok(rt
        .journal(&key.item)?
        .into_iter()
        .skip(before)
        .collect::<Vec<Event>>())
}

/// The FSM's context for this run — four supplied values plus the recorded activations.
fn fsm_context<'a>(
    runtime: &'a RuntimeContext,
    item: &'a Item,
    activations: &'a [(String, String)],
) -> FsmContext<'a> {
    FsmContext::supplied(
        item.id(),
        &runtime.tree_hash,
        Timestamp::from_unix_seconds(runtime.deadline),
        runtime.successor.as_deref(),
    )
    .with_activations(activations)
}

/// The stage entry, as an event.
///
/// `from` is present only when the caller recorded a predecessor: the fold reads it with
/// `optional_str` and this system models no stage ordering, so inventing one here would put a
/// transition into the trailer record that nothing decided.
fn stage_entry(stage: &str, runtime: &RuntimeContext) -> Event {
    let mut payload = serde_json::Map::new();
    payload.insert("to".to_owned(), serde_json::Value::from(stage));
    if let Some(from) = &runtime.entered_from {
        payload.insert("from".to_owned(), serde_json::Value::from(from.clone()));
    }
    Event::new(
        EventKind::StageTransition,
        serde_json::Value::Object(payload),
    )
}

/// **The FSM residue of a journal** — the terminal state and the next action, as canonical UTF-8.
///
/// Both are computed the same way on the recording side and on the replay side, and that is
/// deliberate: what makes the comparison discriminate is not that the two computations differ, it
/// is that the recording's answer is **committed into `run.json`** and the replay's is compared
/// against those bytes. A fold that stops moving the state produces `{"terminal":"not_started"}`
/// here, the committed manifest says otherwise, and the replay refuses.
///
/// # Errors
///
/// [`ReplayError::Malformed`] carrying the fold's own refusal, or the canonical serialiser's.
fn fsm_residue(
    inputs: &RunInputs,
    selection: &Selection,
    item: &Item,
    events: &[Event],
) -> Result<(String, String), ReplayError> {
    let runtime = inputs.runtime()?;
    let state = fsm::fold(item.id(), &inputs.stage, events)?;
    let activations = inputs.activations();
    let ctx = fsm_context(runtime, item, &activations);
    let next = fsm::next_action(selection, &state, &ctx)?;
    let as_text = |bytes: Vec<u8>| -> Result<String, ReplayError> {
        String::from_utf8(bytes).map_err(|e| ReplayError::Malformed {
            path: "<canonical>".to_owned(),
            detail: e.to_string(),
        })
    };
    Ok((
        as_text(state.terminal().canonical_bytes()?)?,
        as_text(next.canonical_bytes()?)?,
    ))
}

/// Produce one golden-log run from its inputs. **The journal is derived, never hand-written.**
///
/// 🛑 **It takes `Ports` as of 2026-09-14 (Slice 4i), and the signature change IS the criterion.**
/// AC-008's GIVEN is *"a replay of a run that **dispatched N activities**"*, and until this slice N
/// was 0 on the only recorded run — so *"all N results served from the journal"* was a statement
/// about an empty set and a kernel that dispatched nothing satisfied it. A recording with
/// dispatches needs a dispatcher, and [`crate::runtime::Runtime::run_activity`] is it. The
/// `run_id` the boundary stamps into every `activity_committed` is [`RunInputs::run_id`]'s derived
/// content hash and is **not** a value the caller may choose, which is why the [`Runtime`] is
/// constructed here rather than handed in whole.
///
/// # Errors
///
/// [`ReplayError`] for a malformed input, a selection that could not resolve, a journal the FSM
/// refused to fold, or a dispatch the boundary refused.
pub fn record(inputs: &RunInputs, ports: Ports) -> Result<RecordedRun, ReplayError> {
    let runtime = Runtime::new(ports, &inputs.run_id()?, &inputs.stage);
    let (selection, item, events) = derive(inputs, &mut Results::Dispatched(&runtime))?;
    let (terminal_state, next_action) = fsm_residue(inputs, &selection, &item, &events)?;
    Ok(RecordedRun {
        run_id: inputs.run_id()?,
        inputs_digest: selection.inputs_digest().as_str().to_owned(),
        inputs: inputs.clone(),
        events,
        terminal_state,
        next_action,
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
        // NEW at Slice 4f. Neither is a clock reading, so the module header's rule still holds:
        // the gate deadline inside `next_action` is a RECORDED INPUT read back out of
        // `inputs/runtime.json`.
        "terminal_state": run.terminal_state,
        "next_action": run.next_action,
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
    if let Some(runtime) = &run.inputs.runtime {
        let mut bytes =
            rk_store::canon::canonical(&runtime.to_json()).map_err(|e| ReplayError::Malformed {
                path: "runtime.json".to_owned(),
                detail: e.to_string(),
            })?;
        bytes.push(b'\n');
        write_file(&inputs.join("runtime.json"), &bytes)?;
    }
    // NEW at Slice 4i. **Always written, even when empty**, so the layout is one shape: a reader
    // that had to distinguish "no bindings" from "an older log" would be back where `read_run`'s
    // refusal of a missing `terminal_state` started.
    let mut specs = serde_json::Map::new();
    for (node, binding) in &run.inputs.specs {
        specs.insert(node.clone(), binding.to_json());
    }
    let mut spec_bytes =
        rk_store::canon::canonical(&serde_json::Value::Object(specs)).map_err(|e| {
            ReplayError::Malformed {
                path: "specs.json".to_owned(),
                detail: e.to_string(),
            }
        })?;
    spec_bytes.push(b'\n');
    write_file(&inputs.join("specs.json"), &spec_bytes)?;

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
    // **A LOG WITHOUT THESE TWO IS REFUSED, and the refusal is the point.** A manifest written
    // before 2026-09-14 carries neither, and reading one with a `None` default would let a replay
    // *succeed* with nothing to compare the FSM residue against — a green whose terminal-state
    // half asserts nothing, which is precisely AC-008's named failure. The Slice-3 fixture
    // therefore goes RED here until it is re-blessed through
    // `bless_the_committed_golden_log`, rather than replaying vacuously.
    let terminal_state = manifest["terminal_state"]
        .as_str()
        .ok_or_else(|| {
            bad(
                "no terminal_state — this log predates Slice 4f's layout and there is nothing to \
                 compare the recomputed terminal state against (AC-008). Re-bless it with \
                 `cargo test -p rk-kernel --features corpus --test it -- --ignored \
                 bless_the_committed_golden_log`",
            )
        })?
        .to_owned();
    let next_action = manifest["next_action"]
        .as_str()
        .ok_or_else(|| {
            bad(
                "no next_action — this log predates Slice 4f's layout and AC-006 assertion 3 has \
                 nothing to compare against. Re-bless it",
            )
        })?
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

    let runtime_path = inputs_dir.join("runtime.json");
    if runtime_path.is_file() {
        let value = parse_json(&runtime_path)?;
        let bad = |d: &str| ReplayError::Malformed {
            path: runtime_path.display().to_string(),
            detail: d.to_owned(),
        };
        let mut runtime = RuntimeContext::supplied(
            value["tree_hash"]
                .as_str()
                .ok_or_else(|| bad("no tree_hash"))?,
            value["deadline"].as_i64().ok_or_else(|| {
                bad("`deadline` is absent or is not an integer — never a silent 0")
            })?,
        );
        if let Some(from) = value["entered_from"].as_str() {
            runtime = runtime.entered_from(from);
        }
        if let Some(to) = value["successor"].as_str() {
            runtime = runtime.advances_to(to);
        }
        if let Some(at) = value["results_at"].as_i64() {
            runtime = runtime.results_at(at);
        }
        inputs = inputs.with_runtime(runtime);
    }

    // **A LOG WITHOUT `inputs/specs.json` IS REFUSED**, on `terminal_state`'s exact warrant one
    // page up: a run recorded before Slice 4i carries no dispatch bindings, so a replay of one
    // would re-derive a journal with **zero** dispatches and compare it against a recorded journal
    // with zero dispatches — AC-008's GIVEN satisfied over an empty N, which is the vacuous green
    // this whole slice exists to remove. Reading an absent file as an empty map would restore it.
    let specs_path = inputs_dir.join("specs.json");
    if !specs_path.is_file() {
        return Err(ReplayError::Malformed {
            path: specs_path.display().to_string(),
            detail: "no specs.json — this log predates Slice 4i's layout, so its journal records \
                     no dispatches and AC-008's GIVEN would be satisfied over an empty N. \
                     Re-bless it with `cargo test -p rk-kernel --features corpus --test it -- \
                     --ignored bless_the_committed_golden_log`"
                .to_owned(),
        });
    }
    let specs = parse_json(&specs_path)?;
    let specs = specs.as_object().ok_or_else(|| ReplayError::Malformed {
        path: specs_path.display().to_string(),
        detail: "specs.json is not an object keyed by DAG node id".to_owned(),
    })?;
    for (node, value) in specs {
        inputs = inputs.with_spec(node, RecordedSpec::from_json(node, value)?);
    }

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
        terminal_state,
        next_action,
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

    // ---- THE RECORDED INPUT SET IS POLICED BEFORE IT IS TRUSTED (AC-042) -----------------------
    //
    // ADDED 2026-09-14, and it closes a gap an adversarial pass found: `replay()` read
    // `inputs_digest` out of the manifest and **never compared it to anything**, and never
    // re-derived `run_id` either. A tampered `inputs/` — a rewritten `specs.json`, a card digest,
    // an edited pack — was therefore accepted by `replay()` itself. It went red only in
    // `golden_log_replay`, and only because that TEST re-records and byte-compares against the
    // committed copy [measured 2026-09-14: editing one `timeout_secs` in the committed fixture
    // fails at `tests/it/replay.rs`'s fixture comparison, not inside `replay()`].
    //
    // **The test's check does not travel.** `redkiln replay --run <id>` is 7d's, it runs against an
    // arbitrary recorded run with **no committed copy to compare against**, and AC-042's whole
    // subject is re-derivation *"from the logged inputs alone"* with a `HiddenInput` on any
    // mismatch. An input set the replay accepts without checking is a hidden input by construction.
    //
    // `run_id` is a content hash of the whole input set — `sha256_lp([SCHEMA, stage,
    // canonical(inputs)])`, and `specs.json` is folded into it like every other input — so one
    // comparison policies all of them. It needs no clock and no port: the recomputation is the
    // same pure function that minted the name.
    let rederived_run_id = recorded.inputs.run_id()?;
    if rederived_run_id != recorded.run_id {
        return Err(ReplayError::Divergent(HiddenInput {
            run_id: recorded.run_id.clone(),
            first_divergent_node: format!(
                "inputs/: the recorded run_id is {} and the logged input set hashes to {} — some \
                 input was edited after the run was recorded, so the re-derivation below would \
                 have run against inputs the recording never saw",
                recorded.run_id, rederived_run_id
            ),
        }));
    }

    // **THE RECORDED JOURNAL IS THE RESULT SOURCE, and the harness is never reached.** Every
    // `dispatch`/`activity_committed` row the FSM asks for is served from the recording, once, in
    // order; an activation the journal does not carry is `ReplayError::JournalHasNoResult` and
    // never a step quietly skipped. That is AC-008's *"all N results served from the journal, zero
    // harness dispatches"* — the zero below is then a reading over a loop that had somewhere else
    // to get its results from, rather than over a loop that never wanted any.
    let (selection, item, rederived) = derive(
        &recorded.inputs,
        &mut Results::Journal(ServedJournal::of(&recorded.events)),
    )?;

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

    // ---- 4b + 4c: the next action and the terminal state ---------------------------------------
    //
    // **The recorded side of both comparisons is READ BACK out of `run.json`** — a value written
    // at record time and committed — and is NOT recomputed from the recorded journal here.
    // Recomputing both sides from the same input would make the assertion vacuous: two folds of
    // byte-identical journals are equal by construction, and AC-008's whole content is that *"a
    // kernel that does nothing at all fails the terminal-state half."* With the recorded value
    // read back from the manifest, a fold that returns `TerminalState::NotStarted` for everything
    // produces `{"terminal":"not_started"}`, the manifest says otherwise, and the replay refuses.
    let (recomputed_terminal, recomputed_next) =
        fsm_residue(&recorded.inputs, &selection, &item, &rederived)?;
    if recorded.next_action != recomputed_next {
        return Err(ReplayError::NextActionDiverged(
            Divergence::NextAction {
                recorded: recorded.next_action.clone(),
                recomputed: recomputed_next,
            }
            .halt(&recorded.run_id),
        ));
    }
    if recorded.terminal_state != recomputed_terminal {
        return Err(ReplayError::TerminalStateDiverged(
            Divergence::TerminalState {
                recorded: recorded.terminal_state.clone(),
                recomputed: recomputed_terminal,
            }
            .halt(&recorded.run_id),
        ));
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
        terminal_state: recomputed_terminal,
        next_action: recomputed_next,
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
