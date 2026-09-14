//! **The golden logs, and the first replay this system has ever run.**
//!
//! `s7b`:1012-1017 and design §4.4's reproducibility contract: *"the FSM has **never been
//! replayed**. There is no prior green to regress from, so the red step here is the first honest
//! measurement, not a formality."* F19's own stated limit is the same: **174/174 suspensions were
//! lossless, but "same next action" is unproven** [measured, F19, one operator].
//!
//! # 🛑 WHAT THIS FILE MEASURES, AND WHAT IT CANNOT — READ THIS BEFORE READING A GREEN
//!
//! AC-006's instrument is **five assertions** (`s7b`:1012-1015): *"every port bound to its fake;
//! byte-identical decisions; byte-identical next action; zero `rk-git` writes; and zero harness
//! dispatches with the terminal state matched."*
//!
//! **All five are measured here as of 2026-09-14 (Slice 4f). The sentence that stood here until
//! then — *"Three of the five are measured here. Two are NOT BUILDABLE at Slice 3"* — is now
//! false, and the two rows it pointed at are rewritten below with the date. What it got right
//! survives it and is stated in its own section: AC-008's GIVEN names N dispatched activities and
//! N is 0, so the criterion is ARMED, not discharged.**
//!
//! | # | assertion | state |
//! | --- | --- | --- |
//! | 1 | every port bound to its fake | **a type fact, not a measurement, and it is now stated as one.** [`rk_kernel::replay::ReplayPorts`] has no optional field, so an unbound port does not compile. The report used to carry a `ports_bound: vec!["Harness", "EventStore", "GitWriter"]` LITERAL and this file asserted that literal against the same literal; the field is deleted. What is asserted below instead is the ports' **observable effects**: the store holds the served journal, and both counters are read off the caller's own fakes |
//! | 2 | byte-identical decisions | **measured** — the re-derived event journal is compared to the recorded one **byte for byte under `rk_store::canon`** |
//! | 3 | byte-identical **next action** | **MEASURED, 2026-09-14.** `rk_kernel::workflow::fsm::next_action`'s canonical bytes, recomputed from the re-derived journal and compared against the value **committed in `run.json`** at record time. The recorded side is READ BACK, never recomputed from the same bytes: two folds of byte-identical journals are equal by construction, so both-sides-recomputed would assert nothing. A premise below rules out `done` first, so the comparison is not held over a next action that decides nothing |
//! | 4 | zero `rk-git` writes | **measured against `rk-git`'s OWN counter.** The replay is handed a real [`rk_git::gitops::GitOps`] in [`rk_git::gitops::Mode::Replay`], driving a [`rk_git::gitops::VecJournal`] — the fake `rk-git`'s own header calls *"the fake, and what the replay tests drive"* — and the zero is `GitOps::writes_performed()`. Until 2026-09-13 this file counted a `GitWriter` trait the KERNEL declared, justified by *"`rk-git` … exposes no port to bind a fake to"*, which `crates/rk-git/src/gitops.rs`:44-51 refutes. `golden_log_replay_writing_fake_is_red` is its positive control, and it now drives a `GitOps` that has performed one write |
//! | 5 | zero harness dispatches **with the terminal state matched** | **both halves measured, 2026-09-14 — and the criterion is still ARMED, for a different reason (next section).** The dispatch count is real: the fake bound below **fails the test on any `dispatch` call** (AC-008's own shape) and the counter is the CALLER's `AtomicU32`, read by the replay before and after. The terminal state is compared against the committed manifest exactly as row 3 is, with `assert_ne!(.., NotStarted)` first so *"a kernel that does nothing at all"* cannot reach a green. **Checkpoint 7b:1515's `terminal_state=matched` grep is reachable and this file now prints the token — computed from the comparison, never as a literal** |
//!
//! # 🛑 AC-008 IS ARMED, NOT DISCHARGED — ITS GIVEN NAMES N DISPATCHED ACTIVITIES AND N IS 0
//!
//! > **AC-008** GIVEN a replay of a run that **dispatched N activities** … THEN the replay reaches
//! > the recorded terminal state with **all N results served from the journal**, zero harness
//! > dispatches, and zero `rk-git` writes.
//!
//! [measured 2026-09-14: the recorded run holds **4** events — `stage_transition`, `dag_assembled`,
//! `gate_opened`, `dag_amended` — and **zero** `dispatch` and `activity_committed` events.] So
//! *"all N results served from the journal"* is a statement about an empty set, and **the dispatch
//! half of this criterion is vacuous**. It is stated here rather than left for a reader to infer
//! from a short journal.
//!
//! **What is NOT vacuous, and why the terminal-state half still discriminates:** the fold lands on
//! `awaiting_dispatch`, `assert_ne!(.., NotStarted)` rules out AC-008's own named failure, and the
//! recorded side of the comparison is a byte string committed in `run.json` — so a changed fold, a
//! changed evaluation order or a changed canonical form all move off it and the replay refuses.
//!
//! **Why a journal with dispatches was not produced in this slice.** `record()` would have to
//! drive `NextAction::Dispatch` through the bound `Harness` and append an `activity_committed` per
//! commit — which means assembling a real `rk_types::activity::ActivitySpec` (card digest, tool
//! allowlist, schema id, timeout, resolved inputs) from a DAG node. **Nothing in `rk-kernel`
//! resolves any of those five**, and the activity-dispatch boundary is Slice 4h's. Building half a
//! dispatcher here to lengthen a journal would be building it without its criteria. What `record`
//! DOES drive is the one runtime act needing no port, no clock and no dispatcher: the tier-1
//! `stage_transition` that enters the stage — which is also what gives
//! `ac161_ac164_delete_db_replay` a tier-1 record to compare at stage grain.
//!
//! # And this replay is over the SELECTION journal plus the stage entry, not a full workflow run
//!
//! What is recorded and replayed is what exists: the runtime's `stage_transition` into the stage,
//! then `select()`'s own event journal — `dag_assembled`, `gate_opened`, `drift_advisory`,
//! `dag_amended` — re-derived from the logged inputs alone. That is AC-042's *"re-derives the DAG from the logged inputs alone and reports
//! `HiddenInput { run_id, first_divergent_node }` on any mismatch"*, and it is the half the scope
//! pass assigns to this slice. `redkiln replay --run <id>`, the cross-process half, is **7d's**.
//!
//! # The three self-reported tokens, and the third control that did not exist until today
//!
//! `s7b`:1018-1023 names three strings this test prints **about itself** — `rk_git_writes=0`,
//! `harness_dispatches=0`, `terminal_state=matched` — and warns that *"a kernel that counts
//! nothing and prints the three literals passes all three greps."* §7b.14:1266-1276 then states
//! the rule: *"every absence-based ban carries a positive control OR says per ban why it cannot."*
//!
//! | token | positive control | since |
//! | --- | --- | --- |
//! | `rk_git_writes=0` | `golden_log_replay_writing_fake_is_red` | §7b.13 |
//! | `harness_dispatches=0` | `golden_log_replay_dispatching_fake_is_red` | 2026-09-13, after mutation G23 found the counter unpinned |
//! | `terminal_state=matched` | `golden_log_replay_terminal_state_mismatch_is_red` | **2026-09-14, this slice** — it had neither a control nor a stated reason (gap G-1), which is the one exception §7b.14's own rule did not permit |
//!
//! **The control was observed red before it was made green, and the red was the interesting one:**
//! with the FSM residue computed and recorded but the two comparison steps not yet in `replay()`,
//! `golden_log_replay` passed and this control failed with *"the replay ACCEPTED a rewritten
//! terminal_state"*. That is the token being printed without being compared — precisely the defect
//! §7b.14:1547-1553 describes — and nothing else in the suite noticed.
//!
//! # Where the logs go
//!
//! `s7b`:860 — *"produce all three artifacts into a **staging** tree, never into the corpus"* — and
//! :848-849 — *"the staged records live in a **scratch tree**, never under `spikes/results/`"*.
//! Neither names a path. **This stage fixes it:** `target/golden-logs-staging/`, which is
//! `.gitignore`d, so nothing corpus-shaped is tracked (AC-465), and which the operator's single
//! `CORPUS_PIN` move copies into `golden-logs/` at the corpus root. The per-run layout is written
//! into `§7b.11` in the same commit as this test, as :831-832 requires.
//!
//! The committed in-crate copies at `tests/fixtures/logs/` are §7b.12's own row — *"the golden
//! event logs' in-crate copies for the red step, before the corpus copies exist"*. This file
//! asserts the staged run is **byte-identical** to them, which is what makes precondition 1's
//! *"its logs were produced by that run, not hand-made"* checkable: a hand-edited fixture goes red.
//!
//! # Why this whole module is `#![cfg(feature = "corpus")]`
//!
//! `s7b`:978 puts the replay group *"behind `#[cfg(feature = "corpus")]`"* and Checkpoint 7b:1316
//! asserts the differential: `cargo test -p rk-kernel -- --list` **without** the feature must select
//! **zero** tests whose name matches `golden_log_replay|ac161_ac164_delete_db_replay`. It measured 3
//! on 2026-09-13 because this gate was missing, which also made :1301's name loop pass vacuously —
//! a `--features corpus` run selects every test, gated or not.
//!
//! Two of the three inputs this module reads are already corpus-bound in spirit and one in fact:
//! `spikes/p9/facets.toml` is the shipped registry, and the golden logs themselves move to
//! `$CORPUS_REPO/golden-logs/` at §7b.11's single `CORPUS_PIN` act, after which
//! [`committed_root`] becomes a corpus-relative read (AC-464). **That re-pointing is the operator's
//! act and is NOT done here**: what is done here is the gate, so the two checkpoint lines are about
//! one deferral rather than two. This module also spawns `git` (through
//! `rk_git::version::assert_at_startup`, the only way to mint the capability token `GitExec::new`
//! demands), which is a second reason it does not belong in the unfeatured run.

#![cfg(feature = "corpus")]

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU32;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::gitops::{GitOps, Mode, VecJournal};
use rk_git::path::WorktreePath;
use rk_kernel::replay::{HiddenInput, ReplayError, ReplayPorts, RunInputs, RuntimeContext};
use rk_kernel::workflow::fsm::{NextAction, StageAction, TerminalState, stage_action};
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::{EventKind, Tier};

const DEVIATION: &str = include_str!("../fixtures/packs/incident-deviation.toml");
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

const CATALOG: &[(&str, &str)] = &[
    (
        "impl-report.toml",
        include_str!("../fixtures/catalog-select/impl-report.toml"),
    ),
    (
        "change-summary.toml",
        include_str!("../fixtures/catalog-select/change-summary.toml"),
    ),
    (
        "change-summary-ue.toml",
        include_str!("../fixtures/catalog-select/change-summary-ue.toml"),
    ),
    (
        "changeset.toml",
        include_str!("../fixtures/catalog-select/changeset.toml"),
    ),
    (
        "spec-authoring.toml",
        include_str!("../fixtures/catalog-select/spec-authoring.toml"),
    ),
];

const BANDS: [&str; 4] = ["low", "med", "high", "critical"];

/// The item and the stage every fold below is keyed on. They are the values [`inputs`] builds, and
/// they are named once so no assertion re-types one and silently keys a fold on a different run.
const ITEM: &str = "INC-1";
const STAGE: &str = "implement";

/// **ADR-0105 clause 4's tree hash, SUPPLIED and recorded — never read from git here.**
///
/// A real 64-character hex string rather than a placeholder, because
/// `rk_kernel::workflow::fsm::GateRecord::derive` hashes it and a shorter value would derive a
/// token that no real gate could carry. It is an INPUT: it goes into `inputs/runtime.json` and
/// into `run_id`, so a replay reads it back rather than being told it out of band.
const TREE_HASH: &str = "9f2c1ab4d6e80357bb41c0d9e2f7a85643c1908df5b2e7461ac30d98f5b1e2c7";

/// The gate deadline, as a recorded value. **Not a clock reading** — `rk_types::time::Timestamp`
/// *"has no `now()`, deliberately"*, and a deadline the replay computed would make two runs of
/// identical inputs produce different bytes.
const DEADLINE: i64 = 1_760_000_000;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// **The staging tree, named once.** `target/golden-logs-staging/` — untracked, so AC-465 holds,
/// and stable, so the operator's move is a copy of a named directory rather than a search.
fn staging_root() -> PathBuf {
    workspace_root().join("target/golden-logs-staging")
}

/// Scratch for the perturbation case. **Deliberately NOT under [`staging_root`]**: the staging tree
/// is what the operator's single `CORPUS_PIN` move copies, and a deliberately-corrupted run sitting
/// in it would be copied into the corpus with the good ones.
fn scratch_root() -> PathBuf {
    workspace_root().join("target/golden-log-scratch")
}

/// The committed in-crate copies (§7b.12's `logs/` row).
fn committed_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/logs")
}

/// The one run every assertion in this file is made over.
///
/// It is the **deviation** fixture deliberately: a run carrying `dag_assembled` **and**
/// `dag_amended` is the only one that makes AC-064's second AND — *"the deviation record is part of
/// the replay input set, so an amended DAG is not a hidden input"* — checkable at all.
fn inputs() -> RunInputs {
    let facets_src = std::fs::read_to_string(workspace_root().join("spikes/p9/facets.toml"))
        .expect("the shipped facet registry is readable");

    RunInputs::new(STAGE)
        .with_runtime(
            // Supplied, never derived, and recorded as part of the input set. `entered_from` and
            // `advances_to` are both present so neither branch of the fold's `optional_str`
            // reading nor `next_action`'s successor argument is exercised only by its absence.
            RuntimeContext::supplied(TREE_HASH, DEADLINE)
                .entered_from("triage")
                .advances_to("verify"),
        )
        .with_pack("incident-deviation.toml", DEVIATION)
        .with_catalog(CATALOG)
        .with_artifacts("artifacts.toml", ARTIFACTS)
        .with_facets(&facets_src)
        .with_item(
            &rk_kernel::select::Item::new(ITEM, "incident")
                .with_toolchain("node", ["text"])
                .with_facet_ordinal("criticality", &BANDS, "critical")
                .with_facet_bool("external_dep", true)
                .with_facet_bool("ui", false),
        )
        .with_deviation(&rk_kernel::select::RequestDeviation::new(
            "ChangeSummary",
            rk_kernel::select::DeviationKind::Skip,
            "criticality >= high",
            "the incident is already summarised in the timeline",
        ))
        .requested_by("impl-report")
}

// ---------------------------------------------------------------------------- the bound fakes

/// The `Harness` fake AC-008 names: **it fails the test on any `dispatch` call.**
///
/// It implements `rk_ports::harness::Harness` and **nothing else**. There is no kernel-side
/// `ReplayHarness: Harness` supertrait any more: the dispatch counter is this fake's own
/// `AtomicU32`, handed to the replay as `ReplayPorts::dispatches` and read there before and after.
#[derive(Debug, Default)]
struct RefusingHarness {
    dispatches: AtomicU32,
}

impl RefusingHarness {
    fn dispatches(&self) -> u32 {
        self.dispatches.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Harness for RefusingHarness {
    fn dispatch(&self, _spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        self.dispatches
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        panic!(
            "AC-008: the replay dispatched a model. Selection spends $0.00 in model dispatches, \
             ALWAYS, and any model spend attributed to selection falsifies purity and invalidates \
             every replay claim in this area"
        );
    }
}

/// A counting `EventStore` fake. `rk_ports::fakes::VecStore` would do, but it hands back no
/// borrow shape the replay can append through **and** the test can read afterwards, so this is the
/// same thing with the events kept where the assertion can reach them.
#[derive(Debug, Default)]
struct CountingStore {
    events: Vec<Event>,
}

impl EventStore for CountingStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        self.events.push(event);
        Ok(())
    }

    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        Ok(self.events.clone())
    }

    /// **Unsupported rather than `Ok(())`.** A snapshot fake that silently succeeds would let a
    /// replay that took one look like a replay that did not.
    fn snapshot(&self) -> Result<Snapshot, StepError> {
        panic!("the replay must not snapshot the store")
    }

    fn rebuild(&mut self, _snapshot: &Snapshot) -> Result<(), StepError> {
        panic!("the replay must not rebuild the store")
    }
}

/// `rk-git`'s own wrapper, built the only way it can be built.
///
/// [`rk_git::version::assert_at_startup`] is the sole minter of the `GitToolchain` capability token
/// [`GitExec::new`] demands, and it spawns `git --version` — which is the second reason this module
/// is `corpus`-gated. This is the same `exec_for` shape `crates/rk-git/tests/it/gitops.rs`:15-18
/// uses, and it is `rk-git`'s instrument rather than one this crate invented.
fn git_exec() -> GitExec {
    let root = workspace_root();
    let tc = rk_git::version::assert_at_startup(&root).expect("git at or above the floor");
    GitExec::new(scratch_root().join("hooks-empty"), tc).expect("the wrapper is constructible")
}

// ---------------------------------------------------------------------------- the assertions

/// **How the committed copy is produced — and it is a deliberate, named act.**
///
/// ```sh
/// cargo test -p rk-kernel --test it -- --ignored bless_the_committed_golden_log
/// ```
///
/// `#[ignore]` and not a default test, on `trybuild`'s own `TRYBUILD=overwrite` pattern: a test
/// that rewrote its own fixture on every run could never go red, and `golden_log_replay`'s
/// byte-identity assertion — which is what makes precondition 1's *"its logs were produced by that
/// run, not hand-made"* checkable — would assert nothing. When the run legitimately changes, this
/// is run once and the new bytes are reviewed in the diff.
#[test]
#[ignore = "writes the committed fixture; run deliberately when the recorded run changes"]
fn bless_the_committed_golden_log() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("the recorded run resolves");
    let dir = rk_kernel::replay::write_run(&committed_root(), &recorded)
        .expect("the fixture tree is writable");
    println!("wrote {}", dir.display());
}

/// **AC-006 / AC-008, as far as this slice can measure them — and the first replay in this
/// system's history.**
///
/// Read the module header before reading this green: assertions 3 and the terminal-state half of 5
/// are **not buildable without an FSM** and are not claimed.
#[test]
fn golden_log_replay() {
    let inputs = inputs();

    // ---- 1. produce the log. THIS RUN produces it; nothing here is hand-made ------------------
    let recorded = rk_kernel::replay::record(&inputs).expect("the recorded run resolves");
    assert!(
        !recorded.events.is_empty(),
        "an empty journal would satisfy every comparison below vacuously"
    );
    let staged = rk_kernel::replay::write_run(&staging_root(), &recorded)
        .expect("the staging tree is writable");

    // ---- 2. the committed in-crate copy is byte-identical to what this run just produced -------
    //
    // This is what makes precondition 1's *"its logs were produced by that run, not hand-made"*
    // a CHECK rather than a promise: edit a committed byte and this goes red.
    let committed = committed_root().join(recorded.run_id());
    assert!(
        committed.is_dir(),
        "the committed in-crate copy is missing at {} — §7b.12's `logs/` row",
        committed.display()
    );
    assert_identical_trees(&staged, &committed);

    // ---- 3. replay it, with every port bound to its fake ---------------------------------------
    let harness = RefusingHarness::default();
    let mut store = CountingStore::default();
    // `rk-git`'s OWN seam, in `Mode::Replay` — in which `GitOps` is structurally unreachable
    // (gitops.rs:13-16: "every method matches on the mode first … there is no branch in which a
    // replayed call reaches `GitExec`"). `writes_performed()` is the number AC-006 asserts.
    let mut journal = VecJournal::default();
    // `&mut`, as of 2026-09-14 (residual 10h): `GitOps::run` takes `&mut self`, so a shared borrow
    // could never reach the one method that moves `writes_performed()`. The zero below is now an
    // absence over a seam the replay COULD have written through.
    let mut git = GitOps::new(git_exec(), &mut journal, Mode::Replay);
    let report = {
        let mut ports = ReplayPorts {
            harness: &harness,
            dispatches: &harness.dispatches,
            store: &mut store,
            git: &mut git,
        };
        rk_kernel::replay::replay(&committed, &mut ports)
            .expect("REPLAY DIVERGED — this is a FINDING, not a flake: see AC-042 and Phase-1 exit criterion (2)")
    };

    // assertion 1 — every port bound to its fake. **A TYPE FACT, NOT A LINE OF THIS TEST.**
    // `ReplayPorts` has four non-`Option` fields, so the construction above is the assertion: it
    // does not compile with a port left out. What used to stand here was
    // `assert_eq!(report.ports_bound, vec!["Harness", "EventStore", "GitWriter"])` — a literal
    // compared to the same literal, which no implementation could fail. The field is gone; what
    // discriminates is each port's OBSERVABLE EFFECT, asserted further down: `store.events` holds
    // the served journal, `git.writes_performed()` is read off `rk-git`, and `harness.dispatches()`
    // is the fake's own counter.

    // assertion 2 — byte-identical decisions.
    assert_eq!(
        report.events_compared,
        recorded.events.len(),
        "every recorded event was compared, not a prefix of them"
    );
    assert!(
        report.events_compared >= 2,
        "a one-event journal is not a journal"
    );
    assert_eq!(
        report.divergences, 0,
        "byte-identical decisions under rk_store::canon"
    );

    // assertion 4 — zero `rk-git` writes, read off `rk-git`'s own counter (ADR-0105 decision 1).
    assert_eq!(report.rk_git_writes, 0);
    assert_eq!(
        git.writes_performed(),
        0,
        "rk-git performed {} invocation(s) during a REPLAY",
        git.writes_performed()
    );
    // …and the journal `GitOps` would have been served from was never drawn on, because nothing in
    // selection shells out to git at all. Stated rather than implied: this is a floor the replay
    // holds today by the pure function having no git call site, and `Mode::Replay` is what keeps it
    // true when Slice 4's runtime gives it one. `git` is dropped first so the journal's `&mut`
    // borrow ends.
    drop(git);
    assert!(
        journal.entries.is_empty(),
        "the replay recorded a git invocation into the journal"
    );

    // assertion 3 — byte-identical NEXT ACTION, and the recorded side is READ BACK from the
    // committed `run.json` rather than recomputed from the journal the replay just re-derived.
    // That is the whole discrimination: two folds of byte-identical journals are equal by
    // construction, so a comparison of both sides against each other would assert nothing. The
    // right-hand side below is a committed byte string that a changed fold, a changed evaluation
    // order or a changed canonical form all move off.
    let recorded_back =
        rk_kernel::replay::read_run(&committed).expect("the committed run reads back");
    let done = String::from_utf8(
        NextAction::Done
            .canonical_bytes()
            .expect("the canonical form of `done`"),
    )
    .expect("canonical JSON is UTF-8");
    assert_ne!(
        report.next_action, done,
        "PREMISE: this run has a plan still to dispatch, so `done` would mean the FSM saw no work \
         and the comparison below would hold over a next action that decides nothing"
    );
    assert_eq!(
        report.next_action,
        recorded_back.next_action(),
        "AC-006 assertion 3: the recomputed next action is not the recorded one"
    );

    // assertion 5, dispatch half — zero harness dispatches.
    assert_eq!(report.harness_dispatches, 0);
    assert_eq!(harness.dispatches(), 0);

    // assertion 5b — THE TERMINAL STATE, with AC-008's own named failure mode ruled out first.
    let not_started = String::from_utf8(
        TerminalState::NotStarted
            .canonical_bytes()
            .expect("the canonical form of `not_started`"),
    )
    .expect("canonical JSON is UTF-8");
    assert_ne!(
        report.terminal_state, not_started,
        "PREMISE: a fold that never moved would print `matched` against an initial state and \
         assert nothing — *\"a kernel that does nothing at all fails the terminal-state half\"*"
    );
    let terminal_matched = report.terminal_state == recorded_back.terminal_state();
    assert!(
        terminal_matched,
        "AC-008: recomputed {} vs recorded {}",
        report.terminal_state,
        recorded_back.terminal_state()
    );

    // …and the journal the replay served back is the recorded one, in order.
    let served = store.events.len();
    assert_eq!(
        served,
        recorded.events.len(),
        "ADR-0105 decision 1: on replay the kernel is SERVED the journal"
    );

    // The amended DAG is in the journal, which is what makes AC-064's second AND checkable.
    assert!(
        store
            .events
            .iter()
            .any(|e| e.kind == rk_types::event_kind::EventKind::DagAmended),
        "the deviation record is part of the replay input set, so the amended DAG is not a hidden \
         input"
    );

    // ---- 4. the record carries the measurement, because the checkpoint reads it ---------------
    //
    // Checkpoint 7b:1513-1514 greps this run's `--nocapture` stdout for `rk_git_writes[= ]0` and
    // `harness_dispatches[= ]0`. **Nothing printed them until 2026-09-13** — the only occurrences
    // of those literals in the crate were inside `assert!` FAIL messages, which print only when the
    // test FAILS, and a failing test is already stopped by :1504. The two lines were therefore
    // unsatisfiable by any passing run. Both numbers below are read off the counters asserted
    // above, never written as literals, and `golden_log_replay_writing_fake_is_red` /
    // `golden_log_replay_dispatching_fake_is_red` are what show each counter moves.
    //
    // **`terminal_state` IS printed now, and the sentence that stood here until 2026-09-14 said it
    // deliberately was not:** *"there is no FSM and no terminal state at Slice 3 … that line is
    // unreachable until Slice 4 builds the runtime."* Half of that is now false — the FSM is on
    // disk at `src/workflow/fsm.rs` — and the half that remains true is stated in the module
    // header's row 5: **N is still 0**, so the DISPATCH half of AC-008's GIVEN is vacuous and is
    // reported ARMED.
    //
    // The token below is **computed from the comparison**, never written as a literal: it reads
    // `matched` only when the recomputed canonical bytes equal the ones committed in `run.json`,
    // and `DIVERGED` otherwise — and the replay would have refused before reaching this line in
    // that case anyway. `golden_log_replay_terminal_state_mismatch_is_red` is its positive
    // control, and it is the third one §7b.14:1266-1276's own rule demanded and the file named
    // for neither.
    println!(
        "replay {} events_compared={} divergences={} rk_git_writes={} harness_dispatches={} \
         terminal_state={} next_action_bytes={}",
        report.run_id,
        report.events_compared,
        report.divergences,
        report.rk_git_writes,
        report.harness_dispatches,
        if terminal_matched {
            "matched"
        } else {
            "DIVERGED"
        },
        report.next_action.len(),
    );
}

/// **THE THIRD POSITIVE CONTROL — the one §7b.14's own rule demanded and no file named.**
///
/// > §7b.14:1266-1276: *"Every absence-based ban below carries a positive control OR says per ban
/// > why it cannot."*
///
/// `rk_git_writes=0` has `golden_log_replay_writing_fake_is_red`; `harness_dispatches=0` has
/// `golden_log_replay_dispatching_fake_is_red`; **`terminal_state=matched` had neither a control
/// nor a reason, and a kernel that printed the literal without comparing passed the grep.** Gap
/// G-1 in the runbook pass, FORCED by residual 10i.
///
/// The adverse condition is constructed **in this test** on
/// `golden_log_replay_writing_fake_is_red`'s exact model, not committed as a second fixture tree:
/// a second tree is a second thing to keep in sync, and a stale one is green for the wrong reason.
/// `run.json`'s `terminal_state` is rewritten to a state the fold **cannot** produce from this
/// journal — `sealed`, which needs a `slice_sealed` event the journal does not carry — and the
/// replay must return `Err(ReplayError::TerminalStateDiverged)`.
///
/// **It is weaker than it looks in one direction and the doc says so rather than leaving it to be
/// discovered:** it shows the comparison happens and refuses. The `assert_ne!` premise inside
/// `golden_log_replay` is a second, weaker control that catches a fold which never moves but not
/// one that moves wrongly. Both are named; neither is reported as the other.
#[test]
fn golden_log_replay_terminal_state_mismatch_is_red() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("resolves");
    let source = committed_root().join(recorded.run_id());

    let rewritten = scratch_root().join("terminal-state-rewritten");
    copy_tree(&source, &rewritten);
    let manifest_path = rewritten.join("run.json");
    let mut manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).expect("readable"))
            .expect("the manifest parses");

    let honest = manifest["terminal_state"]
        .as_str()
        .expect("the committed manifest carries a terminal_state")
        .to_owned();
    let forged = String::from_utf8(
        TerminalState::Sealed {
            verdict: "accepted".to_owned(),
        }
        .canonical_bytes()
        .expect("canonical"),
    )
    .expect("canonical JSON is UTF-8");
    assert_ne!(
        honest, forged,
        "PREMISE: the rewrite must actually change the recorded state, or the replay would be \
         asked to refuse a value it should accept"
    );
    manifest["terminal_state"] = serde_json::Value::from(forged.clone());
    std::fs::write(
        &manifest_path,
        serde_json::to_vec(&manifest).expect("serialisable"),
    )
    .expect("writable");

    let harness = RefusingHarness::default();
    let mut store = CountingStore::default();
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(git_exec(), &mut journal, Mode::Replay);
    let mut ports = ReplayPorts {
        harness: &harness,
        dispatches: &harness.dispatches,
        store: &mut store,
        git: &mut git,
    };

    match rk_kernel::replay::replay(&rewritten, &mut ports) {
        Err(ReplayError::TerminalStateDiverged(d)) => {
            assert_eq!(d.recorded, forged, "the halt carries the RECORDED side");
            assert_eq!(
                d.recomputed, honest,
                "and the RECOMPUTED side, so the two are distinguishable in the halt"
            );
            assert_eq!(
                d.activity_id,
                recorded.run_id(),
                "AC-007's typed halt names what diverged"
            );
        }
        Ok(report) => panic!(
            "the replay ACCEPTED a rewritten terminal_state and reported terminal_state={} — the \
             token is printed without being compared, which is exactly what this control exists \
             to catch",
            report.terminal_state
        ),
        Err(other) => panic!("expected TerminalStateDiverged, got {other}"),
    }
}

/// **The positive control for AC-006 assertion 3, and it exists because a mutation found the gap.**
///
/// `golden_log_replay_terminal_state_mismatch_is_red` above controls the *terminal state* half.
/// Nothing controlled the **next action** half until 2026-09-14: replacing `src/replay.rs`'s
/// `if recorded.next_action != recomputed_next {` with `if false && …` left **every replay test
/// green** — 7 passed, 0 failed, with `golden_log_replay` itself among them [measured 2026-09-14,
/// mutation applied and reverted, `md5sum` identical before and after]. A guard that can be
/// disabled with the suite still green is declared and not measured, which is the same defect
/// `golden_log_replay_dispatching_fake_is_red` was authored for on the dispatch counter.
///
/// Same shape as the terminal-state control: rewrite the **recorded** side of the comparison in a
/// scratch copy to a well-formed but different value, and require the replay to **refuse**. The
/// recomputed side is unchanged, so a replay that compares nothing returns `Ok` and this test goes
/// red — which is exactly the state the mutation above put the tree in.
#[test]
fn golden_log_replay_next_action_mismatch_is_red() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("resolves");
    let source = committed_root().join(recorded.run_id());

    let rewritten = scratch_root().join("next-action-rewritten");
    copy_tree(&source, &rewritten);
    let manifest_path = rewritten.join("run.json");
    let mut manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).expect("readable"))
            .expect("the manifest parses");

    let honest = manifest["next_action"]
        .as_str()
        .expect("the committed manifest carries a next_action")
        .to_owned();
    let forged = String::from_utf8(NextAction::Done.canonical_bytes().expect("canonical"))
        .expect("canonical JSON is UTF-8");
    assert_ne!(
        honest, forged,
        "PREMISE: the rewrite must actually change the recorded action, or the replay would be \
         asked to refuse a value it should accept. `Done` is the action a run with nothing left to \
         do returns, and the committed run has an open gate, so the two differ by construction"
    );
    manifest["next_action"] = serde_json::Value::from(forged.clone());
    std::fs::write(
        &manifest_path,
        serde_json::to_vec(&manifest).expect("serialisable"),
    )
    .expect("writable");

    let harness = RefusingHarness::default();
    let mut store = CountingStore::default();
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(git_exec(), &mut journal, Mode::Replay);
    let mut ports = ReplayPorts {
        harness: &harness,
        dispatches: &harness.dispatches,
        store: &mut store,
        git: &mut git,
    };

    match rk_kernel::replay::replay(&rewritten, &mut ports) {
        Err(ReplayError::NextActionDiverged(d)) => {
            assert_eq!(d.recorded, forged, "the halt carries the RECORDED side");
            assert_eq!(
                d.recomputed, honest,
                "and the RECOMPUTED side, so the two are distinguishable in the halt"
            );
            assert_eq!(
                d.activity_id,
                recorded.run_id(),
                "AC-007's typed halt names what diverged"
            );
        }
        Ok(_) => panic!(
            "the replay ACCEPTED a rewritten next_action — AC-006 assertion 3's guard is declared \
             and does not bite. This is the exact state `if false && recorded.next_action != …` \
             produced, and the whole replay suite was green in it"
        ),
        Err(other) => panic!("expected NextActionDiverged, got {other}"),
    }
}

/// **AC-161 and AC-164 — the stage-grain next action survives the event database.**
///
/// > **AC-161** THEN the **stage-level next action** — the FSM's next stage transition and the
/// > gate records the trailers carry — is equal, under the canonical serialiser, to the one
/// > selected before the deletion. *The comparison is at stage grain; a step-grain comparison
/// > would assert what tier 2 cannot promise.*
///
/// 🛑 **TWO THINGS ARE ARMED HERE AND NEITHER IS REPORTED AS COVERED.**
///
/// 1. **The ROUTE is not measured, only the EQUALITY.** The tier-1 projection below is read from
///    the golden log's own `events.jsonl`, **not** from git trailers: the trailer reader is
///    `rk-git`'s and `rk-cli`'s and it is Step 7d's. What is measured is that the stage action is
///    a function of the tier-1 events alone. What is not is *"from git alone"*.
/// 2. **The "event database" here is the bound `EventStore` fake, and it is DROPPED rather than
///    unlinked.** There is no SQLite-backed `EventStore` in this workspace — the adapter is
///    `rk-adapter-sqlite` and it is 7c's — so creating and deleting an empty `store.sqlite3`
///    beside this test would be a filesystem gesture that produces the same green as the real
///    operation while measuring nothing. The deletion that is performed is the one this tree can
///    perform: the store holding every tier-2 event goes away, and the recomputation is handed the
///    tier-1 events alone.
///
/// **AC-164's `doctor`-classification half has no home at all** (residual 10l): it is in neither
/// §7b.15's claimed list nor its ARMED table, and `doctor` is 7d's. AC-164 is **not** covered.
#[test]
fn ac161_ac164_delete_db_replay() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("the recorded run resolves");

    // ---- premises: the journal really spans both tiers ----------------------------------------
    let tier_one: Vec<Event> = recorded
        .events
        .iter()
        .filter(|e| e.kind.tier() == Tier::One)
        .cloned()
        .collect();
    let tier_two = recorded.events.len() - tier_one.len();
    assert!(
        tier_two >= 2 && !tier_one.is_empty(),
        "PREMISE: an all-tier-1 journal makes the truncation equality trivially true. Got {} \
         tier-1 and {tier_two} tier-2 events",
        tier_one.len()
    );

    // ---- the stage action while the store holds EVERYTHING -------------------------------------
    let mut store = CountingStore::default();
    for event in &recorded.events {
        store.append(event.clone()).expect("the fake accepts");
    }
    assert_eq!(
        store.events.len(),
        recorded.events.len(),
        "PREMISE: the event database really held the whole journal before it was deleted"
    );
    let full = stage_action(ITEM, STAGE, &store.events).expect("the tier-1 fold resolves");
    assert_ne!(
        full,
        StageAction::default(),
        "PREMISE: an empty stage action is equal to another empty one, and the comparison below \
         would hold over two nothings"
    );

    // ---- AC-164: the tier-2 events are truncated ----------------------------------------------
    assert!(
        tier_one.len() < recorded.events.len(),
        "PREMISE: something must actually be truncated"
    );
    let projected = stage_action(ITEM, STAGE, &tier_one).expect("resolves");
    assert_eq!(
        full.canonical_bytes().expect("canonical"),
        projected.canonical_bytes().expect("canonical"),
        "AC-164: losing every tier-2 event does not change the stage-grain next action"
    );

    // ---- AC-161: the event database is gone -----------------------------------------------------
    drop(store);
    let after = stage_action(ITEM, STAGE, &tier_one).expect("resolves");
    assert_eq!(
        full.canonical_bytes().expect("canonical"),
        after.canonical_bytes().expect("canonical"),
        "AC-161: the stage-level next action after the deletion is byte-identical, under the \
         canonical serialiser, to the one selected before it"
    );

    // ---- THE CONTROL: a different journal must give a DIFFERENT stage action --------------------
    //
    // Without it every assertion above is satisfied by a `stage_action` that returns a constant —
    // and a constant is exactly what a function whose tier filter dropped everything returns.
    let elsewhere = vec![Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": STAGE, "to": "verify" }),
    )];
    let control = stage_action(ITEM, "verify", &elsewhere).expect("resolves");
    assert_ne!(
        full.canonical_bytes().expect("canonical"),
        control.canonical_bytes().expect("canonical"),
        "CONTROL: a different tier-1 journal must give a different stage action, or the three \
         equalities above are satisfied by a constant"
    );

    println!(
        "ac161_ac164 stage_action tier1={} tier2={tier_two} bytes={}",
        tier_one.len(),
        full.canonical_bytes().expect("canonical").len()
    );
}

/// **The positive control for the self-reported counters** (`s7b`:1018-1023).
///
/// > `rk_git_writes=0`, `harness_dispatches=0` and `terminal_state=matched` are strings the replay
/// > prints **about itself**: a kernel that counts nothing and prints the three literals passes all
/// > three greps. This test replays against a `GitOps` fake that performs **one** write and asserts
/// > the replay **refuses** — the counter must move off 0.
///
/// The fake is **`rk-git`'s own**, exactly as `s7b`:1121 words it — a real [`GitOps`] that has
/// performed one write — and the number is `rk-git`'s `writes_performed()`, not one this crate
/// supplies. A replay whose counter is a constant `0` passes the test above and fails this one.
///
/// The write is a real `git rev-parse HEAD` in [`Mode::Live`], because that is what moves `rk-git`'s
/// counter: a `Mode::Replay` `GitOps` is structurally incapable of reporting a write, and faking the
/// number would put this control back where the 2026-09-13 repair started.
#[test]
fn golden_log_replay_writing_fake_is_red() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("resolves");
    let dir = committed_root().join(recorded.run_id());

    let harness = RefusingHarness::default();
    let mut store = CountingStore::default();
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(git_exec(), &mut journal, Mode::Live);
    let cwd = WorktreePath::new(&workspace_root()).expect("the workspace root normalises");
    git.run(GitOp::RevParse, &cwd, &["rev-parse", "HEAD"])
        .expect("the workspace is a git checkout");
    assert_eq!(
        git.writes_performed(),
        1,
        "the control never reached git, so it would assert nothing"
    );
    let mut ports = ReplayPorts {
        harness: &harness,
        dispatches: &harness.dispatches,
        store: &mut store,
        git: &mut git,
    };

    match rk_kernel::replay::replay(&dir, &mut ports) {
        Err(ReplayError::GitWroteOnReplay { writes }) => {
            assert_eq!(writes, 1, "the counter moved off 0 and the replay refused");
        }
        Ok(report) => panic!(
            "the replay accepted a git fake that performed one write, and reported \
             rk_git_writes={} — the counter is a constant",
            report.rk_git_writes
        ),
        Err(other) => panic!("expected GitWroteOnReplay, got {other}"),
    }
}

/// **The positive control for the OTHER self-reported counter — and it exists because a
/// falsification found the first one unpinned.**
///
/// `s7b`:1018-1023 names three counters the replay prints about itself and gives a control for the
/// git one. **Mutation G23 of 2026-09-13 measured that `harness_dispatches` had none**: replacing
/// both of the replay's reads of the dispatch counter with a literal `0` left every test green,
/// because a replay that never dispatches cannot tell a read counter from a printed one. The register's own warning — *"a kernel that counts nothing and prints the three literals
/// passes all three greps"* — applied to this file, and it does not any more.
///
/// The control is the git one's shape: a harness fake that reports **one** prior dispatch, and the
/// replay must **refuse**.
#[test]
fn golden_log_replay_dispatching_fake_is_red() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("resolves");
    let dir = committed_root().join(recorded.run_id());

    let harness = RefusingHarness::default();
    harness
        .dispatches
        .store(1, std::sync::atomic::Ordering::SeqCst);
    let mut store = CountingStore::default();
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(git_exec(), &mut journal, Mode::Replay);
    let mut ports = ReplayPorts {
        harness: &harness,
        dispatches: &harness.dispatches,
        store: &mut store,
        git: &mut git,
    };

    match rk_kernel::replay::replay(&dir, &mut ports) {
        Err(ReplayError::HarnessDispatchedOnReplay { dispatches }) => {
            assert_eq!(
                dispatches, 1,
                "the counter moved off 0 and the replay refused"
            );
        }
        Ok(report) => panic!(
            "the replay accepted a harness fake that had already dispatched, and reported \
             harness_dispatches={} — the counter is a constant",
            report.harness_dispatches
        ),
        Err(other) => panic!("expected HarnessDispatchedOnReplay, got {other}"),
    }
}

/// **AC-042's THEN, on the branch that matters.** *"…reports `HiddenInput { run_id,
/// first_divergent_node }` on any mismatch, **never a warning**."*
///
/// A hidden input is simulated the only way it can be without one: the recorded journal is
/// perturbed in a copy, and the replay must refuse rather than report a near-match. **SM-013
/// records no baseline and reports non-identical runs as a count, never as a percentage rounded to
/// 100%** — so what this test asserts is the refusal, and the count is printed.
#[test]
fn replay_reports_hidden_input_on_divergence() {
    let recorded = rk_kernel::replay::record(&inputs()).expect("resolves");
    let source = committed_root().join(recorded.run_id());

    let perturbed = scratch_root().join("perturbed");
    copy_tree(&source, &perturbed);
    let events_path = perturbed.join("events.jsonl");
    let text = std::fs::read_to_string(&events_path).expect("the journal is readable");
    let broken = text.replacen("implement-change[node]", "implement-change[dotnet]", 1);
    assert_ne!(text, broken, "the perturbation must actually change a byte");
    std::fs::write(&events_path, &broken).expect("writable");

    let harness = RefusingHarness::default();
    let mut store = CountingStore::default();
    let mut journal = VecJournal::default();
    let mut git = GitOps::new(git_exec(), &mut journal, Mode::Replay);
    let mut ports = ReplayPorts {
        harness: &harness,
        dispatches: &harness.dispatches,
        store: &mut store,
        git: &mut git,
    };

    match rk_kernel::replay::replay(&perturbed, &mut ports) {
        Err(ReplayError::Divergent(HiddenInput {
            run_id,
            first_divergent_node,
        })) => {
            assert_eq!(run_id, recorded.run_id());
            assert!(
                first_divergent_node.contains("implement-change"),
                "the FIRST divergent node is named, not merely that something differed: \
                 {first_divergent_node}"
            );
            // SM-013's rule: a count, never a percentage.
            println!("SM-013 non-identical runs: 1 (of 1 perturbed); no baseline exists");
        }
        Ok(_) => panic!("a perturbed journal replayed clean — the comparison is not comparing"),
        Err(other) => panic!("expected HiddenInput, got {other}"),
    }
}

/// **The recorded inputs round-trip.** AC-042 says *"re-derives the DAG from the **logged inputs
/// alone**"*, so the logged inputs must be sufficient: the item reconstructed from the log must
/// produce the same context digest as the one that was recorded.
#[test]
fn the_logged_inputs_are_sufficient_to_rebuild_the_run() {
    let inputs = inputs();
    let recorded = rk_kernel::replay::record(&inputs).expect("resolves");
    let dir = committed_root().join(recorded.run_id());
    let read_back = rk_kernel::replay::read_run(&dir).expect("the committed run reads back");

    assert_eq!(read_back.run_id(), recorded.run_id());
    assert_eq!(
        read_back.inputs_digest(),
        recorded.inputs_digest(),
        "the four-digest tuple survives the round trip through the log"
    );
    assert_eq!(
        read_back.canonical_events().expect("canonical"),
        recorded.canonical_events().expect("canonical"),
        "the journal survives the round trip byte for byte"
    );
}

// ---------------------------------------------------------------------------- tree helpers

fn assert_identical_trees(a: &Path, b: &Path) {
    let mut left = list_files(a);
    let mut right = list_files(b);
    left.sort();
    right.sort();
    assert_eq!(
        left, right,
        "the staged run and the committed copy hold the same files"
    );
    assert!(
        !left.is_empty(),
        "an empty tree compares equal to an empty tree"
    );
    for rel in &left {
        let lb = std::fs::read(a.join(rel)).expect("readable");
        let rb = std::fs::read(b.join(rel)).expect("readable");
        assert_eq!(
            lb,
            rb,
            "{} differs between the staged run and the committed copy — either the committed copy \
             was hand-edited, or the run is not reproducible",
            rel.display()
        );
    }
}

fn list_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Ok(rel) = path.strip_prefix(root) {
                out.push(rel.to_path_buf());
            }
        }
    }
    out
}

fn copy_tree(from: &Path, to: &Path) {
    let _ = std::fs::remove_dir_all(to);
    std::fs::create_dir_all(to).expect("creatable");
    for rel in list_files(from) {
        let dest = to.join(&rel);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).expect("creatable");
        }
        std::fs::copy(from.join(&rel), dest).expect("copyable");
    }
}
