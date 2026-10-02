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
//! false, and the two rows it pointed at are rewritten below with the date. And the clause that
//! replaced it — *"AC-008's GIVEN names N dispatched activities and N is 0, so the criterion is
//! ARMED"* — is false as of **Slice 4i**: `record()` drives the FSM's `Dispatch` through a bound
//! `Harness` and the recorded run dispatches N = 2. Its own section below is rewritten rather
//! than deleted, because what it said about the vacuity was right while it stood.**
//!
//! | # | assertion | state |
//! | --- | --- | --- |
//! | 1 | every port bound to its fake | **a type fact, not a measurement, and it is now stated as one.** [`rk_kernel::replay::ReplayPorts`] has no optional field, so an unbound port does not compile. The report used to carry a `ports_bound: vec!["Harness", "EventStore", "GitWriter"]` LITERAL and this file asserted that literal against the same literal; the field is deleted. What is asserted below instead is the ports' **observable effects**: the store holds the served journal, and both counters are read off the caller's own fakes |
//! | 2 | byte-identical decisions | **measured** — the re-derived event journal is compared to the recorded one **byte for byte under `rk_store::canon`** |
//! | 3 | byte-identical **next action** | **MEASURED, 2026-09-14.** `rk_kernel::workflow::fsm::next_action`'s canonical bytes, recomputed from the re-derived journal and compared against the value **committed in `run.json`** at record time. The recorded side is READ BACK, never recomputed from the same bytes: two folds of byte-identical journals are equal by construction, so both-sides-recomputed would assert nothing. A premise below rules out `done` first, so the comparison is not held over a next action that decides nothing |
//! | 4 | zero `rk-git` writes | **measured against `rk-git`'s OWN counter.** The replay is handed a real [`rk_git::gitops::GitOps`] in [`rk_git::gitops::Mode::Replay`], driving a [`rk_git::gitops::VecJournal`] — the fake `rk-git`'s own header calls *"the fake, and what the replay tests drive"* — and the zero is `GitOps::writes_performed()`. Until 2026-09-13 this file counted a `GitWriter` trait the KERNEL declared, justified by *"`rk-git` … exposes no port to bind a fake to"*, which `crates/rk-git/src/gitops.rs`:44-51 refutes. `golden_log_replay_writing_fake_is_red` is its positive control, and it now drives a `GitOps` that has performed one write |
//! | 5 | zero harness dispatches **with the terminal state matched** | **both halves measured, 2026-09-14, and at Slice 4i over a GIVEN that is satisfied** — the recorded run dispatches N = 2 and the replay serves both results from the journal (next section). The dispatch count is real: the fake bound below **fails the test on any `dispatch` call** (AC-008's own shape) and the counter is the CALLER's `AtomicU32`, read by the replay before and after. The terminal state is compared against the committed manifest exactly as row 3 is, with `assert_ne!(.., NotStarted)` first so *"a kernel that does nothing at all"* cannot reach a green. **Checkpoint 7b:1515's `terminal_state=matched` grep is reachable and this file now prints the token — computed from the comparison, never as a literal** |
//!
//! # AC-008's GIVEN NAMES N DISPATCHED ACTIVITIES, AND N WAS 0 UNTIL SLICE 4i
//!
//! > **AC-008** GIVEN a replay of a run that **dispatched N activities** … THEN the replay reaches
//! > the recorded terminal state with **all N results served from the journal**, zero harness
//! > dispatches, and zero `rk-git` writes.
//!
//! **What this section said while it was true, and it was:**
//!
//! > [measured 2026-09-14: the recorded run holds **4** events — `stage_transition`,
//! > `dag_assembled`, `gate_opened`, `dag_amended` — and **zero** `dispatch` and
//! > `activity_committed` events.] So *"all N results served from the journal"* is a statement
//! > about an empty set, and **the dispatch half of this criterion is vacuous** … **Why a journal
//! > with dispatches was not produced in this slice.** `record()` would have to drive
//! > `NextAction::Dispatch` through the bound `Harness` … **Nothing in `rk-kernel` resolves any of
//! > those five**, and the activity-dispatch boundary is Slice 4h's.
//!
//! Slice 4h built that boundary and **Slice 4i makes `record()` drive it**. [measured 2026-09-14,
//! after: the recorded run holds **8** events — the four above plus **2** `dispatch` and **2**
//! `activity_committed` — its terminal state is `stage_complete` rather than `awaiting_dispatch`,
//! and its next action is `advance_stage`.] The five `ActivitySpec` fields are still resolved by
//! nothing, and that is not papered over: they arrive as a **recorded input**, `inputs/specs.json`,
//! folded into `run_id` like every other input, so AC-042's *"from the logged inputs alone"* holds
//! and **a record routing spec assembly is OWED** (`rk_kernel::replay::RecordedSpec`'s own doc).
//!
//! **Three things make the served N non-vacuous, and each is an assertion rather than a claim:**
//!
//! 1. `recorded_dispatches > 0`, with the recording's own fake counter equal to it — so the rows
//!    are the port's and not rows the recorder minted beside a port it never reached;
//! 2. `events_compared == recorded.events.len()` with `divergences == 0` — the re-derivation
//!    asked the journal for every one of the N results and got them, and a loop that served none
//!    would re-derive a 4-event journal that the length comparison inside `replay()` refuses;
//! 3. [`golden_log_replay_missing_result_is_red`] — the positive control: strip the
//!    `activity_committed` rows from a scratch copy and the replay must return
//!    `ReplayError::JournalHasNoResult` rather than skipping the step and folding on.
//!
//! 🛑 **AND WHAT IS STILL NOT DISCHARGED.** The N dispatches are a **scripted fake's**. AC-008 asks
//! for exactly that, so this is coverage and not a shortfall — but nothing here is evidence about a
//! real adapter, which is 7c's, and no line of this file should be read as such. AC-161's route
//! *"from git alone"* and AC-164's `doctor` half are unchanged and still uncovered, below.
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
//! | `results_served=N` | `golden_log_replay_missing_result_is_red` | **2026-09-14, Slice 4i** — the token is new because N was 0 before it, and an absence over an empty set needs no control |
//! | the served rows' **activation order** | `golden_log_replay_swapped_activations_is_red` | **2026-09-14**, after a mutation found `ServedJournal::serve`'s activation-match check deletable with all seven replay tests still green. The served rows are CLONES of the recorded rows, so the byte comparison over that region is tautological; the count and the per-activation commit are controlled above, and *which* activation was asked for, in *what* order, was not |
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
//! `CORPUS_PIN` move copies into `golden-logs/` at the corpus root.
//!
//! 🛑 **THE §7b.11 LAYOUT AMENDMENT IS *OWED*, NOT DISCHARGED — corrected 2026-09-14 on an
//! adversarial verification pass.** The sentence that stood here read *"the per-run layout is
//! written into `§7b.11` in the same commit as this test, as :831-832 requires"*, and **both
//! halves of it were false**:
//!
//! * **The layout was not written.** §7b.11's layout block (`s7b`:890-902) names `run.json`,
//!   `inputs/{pack,catalog,artifacts,facets.toml,context.json,artifact-state.json,deviation.json}`
//!   and `events.jsonl` — and **nothing else**. `grep -n 'inputs/specs.json\|results_at'
//!   runbook/phase-1/s7b-kernel.md` returns **zero lines** [measured 2026-09-14], while this
//!   crate's shipped layout (`src/replay.rs`:95-112) adds `inputs/runtime.json` (Slice 4f),
//!   `inputs/specs.json` (Slice 4i), `runtime.json`'s `results_at` (4i) and `run.json`'s
//!   `terminal_state`/`next_action` (4f). §7b.11 precondition **3** — *"the `golden-logs/` per-run
//!   layout is written into this section"* — is therefore **NOT met for the layout this crate
//!   writes**, and it gates the `CORPUS_PIN` move, which is what copies these logs into the
//!   corpus. What §7b.11 *did* gain (:918-926) is the **staging path**, `target/golden-logs-
//!   staging/`; that is a different obligation and it is discharged.
//! * **The citation was stale.** `s7b`:831-832 is the *"### 7b.10 The kernel runtime"* heading, not
//!   a requirement about the layout. The requirement is precondition 3 at :946.
//!
//! The runbook is outside this session's editable set (`00-preamble.md`:55 — a stage records what
//! is owed rather than deciding it), so what is done here is the record: **a dated §7b.11 amendment
//! naming the four layout additions is OWED to the operator, BEFORE the pin move**, and until it
//! lands precondition 3 is false and the move may not run on precondition 3's word. Nothing in this
//! file may be read as having discharged it.
//!
//! **HALF OF THAT WAS DISCHARGED AT `0780198` AND A DIFFERENT HALF IS NOW OWED — 2026-09-14.** The
//! four layout additions landed in §7b.11 at that commit, so the paragraph above is satisfied as
//! written. What that amendment deliberately did **not** take is the version bump, on the stated
//! ground that *"a runbook that described a version the code does not write would be the
//! documented-but-false state this file has been repaired for twice already"* — and the operator's
//! decision **D-4** has now taken the code half: `rk_kernel::replay::SCHEMA` reads
//! `rk-kernel/golden-log/v2` and the committed fixture is re-blessed at
//! `implement-bf1d8cec3b22d288`. So the OWED debt has **changed shape, not size**: §7b.11 still
//! says `v1`, which is the mirror of the state `0780198` refused to create, and **a dated §7b.11
//! amendment carrying `v2` plus the re-read of precondition 3 that block asks for are OWED to the
//! operator, still BEFORE the pin move**. [`the_layout_version_reaches_the_run_id_derivation`] is
//! the pin on the code half and carries no AC id for exactly this reason.
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

// *(dated 2026-09-28, s8e build, package S8E-P3.)* The module-wide `#![cfg(feature = "corpus")]` that stood
// here is now one `#[cfg(feature = "corpus")]` on every item below it, unchanged in effect for every
// item that was here: an unfeatured build compiles none of them, and Checkpoint 7b's `--list`
// differential still selects zero `golden_log_replay|ac161_ac164_delete_db_replay` without the
// feature. The one ungated item is `golden_log_canonical_digests_are_unchanged_by_the_interim_item_key`
// (ADR-0182 decision 6; s8e section 8e.10), whose first arm reads the COMMITTED copy of the golden log
// and must run in the unfeatured suite: s8e section 8e.0 item 6 found that under the module-wide gate
// the name would run zero times under Checkpoint 8e (2) and (8). It spawns no git.

#[cfg(feature = "corpus")]
use std::path::{Path, PathBuf};
#[cfg(feature = "corpus")]
use std::sync::atomic::AtomicU32;
#[cfg(feature = "corpus")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "corpus")]
use rk_git::git_exec::{GitExec, GitOp};
#[cfg(feature = "corpus")]
use rk_git::gitops::{GitOps, Mode, VecJournal};
#[cfg(feature = "corpus")]
use rk_git::path::WorktreePath;
#[cfg(feature = "corpus")]
use rk_kernel::replay::{
    HiddenInput, RecordedSpec, ReplayError, ReplayPorts, RunInputs, RuntimeContext,
};
#[cfg(feature = "corpus")]
use rk_kernel::workflow::fsm::{NextAction, StageAction, TerminalState, stage_action};
#[cfg(feature = "corpus")]
use rk_ports::docs_site::DocsSite;
#[cfg(feature = "corpus")]
use rk_ports::event_store::{EventStore, Query, Snapshot};
#[cfg(feature = "corpus")]
use rk_ports::fakes::{BoardTracker, FixtureToolchain, RecordingDocs};
#[cfg(feature = "corpus")]
use rk_ports::harness::{DispatchOutcome, Harness};
#[cfg(feature = "corpus")]
use rk_ports::root::Ports;
#[cfg(feature = "corpus")]
use rk_ports::toolchain::Toolchain;
#[cfg(feature = "corpus")]
use rk_ports::tracker::Tracker;
#[cfg(feature = "corpus")]
use rk_types::activity::{ActivitySpec, DispatchRecord};
#[cfg(feature = "corpus")]
use rk_types::digest::Digest;
#[cfg(feature = "corpus")]
use rk_types::error::StepError;
#[cfg(feature = "corpus")]
use rk_types::event::Event;
#[cfg(feature = "corpus")]
use rk_types::event_kind::{EventKind, Tier};
#[cfg(feature = "corpus")]
use rk_types::retention::Retained;
#[cfg(feature = "corpus")]
use rk_types::typed::{SchemaId, Typed};
#[cfg(feature = "corpus")]
use rk_types::usage::{CliVersion, Timings};

#[cfg(feature = "corpus")]
const DEVIATION: &str = include_str!("../fixtures/packs/incident-deviation.toml");
#[cfg(feature = "corpus")]
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

#[cfg(feature = "corpus")]
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

#[cfg(feature = "corpus")]
const BANDS: [&str; 4] = ["low", "med", "high", "critical"];

/// The item and the stage every fold below is keyed on. They are the values [`inputs`] builds, and
/// they are named once so no assertion re-types one and silently keys a fold on a different run.
#[cfg(feature = "corpus")]
const ITEM: &str = "INC-1";
#[cfg(feature = "corpus")]
const STAGE: &str = "implement";

/// **ADR-0105 clause 4's tree hash, SUPPLIED and recorded — never read from git here.**
///
/// A real 64-character hex string rather than a placeholder, because
/// `rk_kernel::workflow::fsm::GateRecord::derive` hashes it and a shorter value would derive a
/// token that no real gate could carry. It is an INPUT: it goes into `inputs/runtime.json` and
/// into `run_id`, so a replay reads it back rather than being told it out of band.
#[cfg(feature = "corpus")]
const TREE_HASH: &str = "9f2c1ab4d6e80357bb41c0d9e2f7a85643c1908df5b2e7461ac30d98f5b1e2c7";

/// The gate deadline, as a recorded value. **Not a clock reading** — `rk_types::time::Timestamp`
/// *"has no `now()`, deliberately"*, and a deadline the replay computed would make two runs of
/// identical inputs produce different bytes.
#[cfg(feature = "corpus")]
const DEADLINE: i64 = 1_760_000_000;

/// The journaled instant every `activity_committed` this run writes carries. Same rule as
/// [`DEADLINE`] and same reason: a clock reading here would make the recorded journal a different
/// byte string on every run and turn AC-042's byte-identity into a comparison of clocks.
#[cfg(feature = "corpus")]
const RESULTS_AT: i64 = 1_759_900_000;

/// The schema every recorded [`ActivitySpec`] demands, and the one the scripted responder returns.
///
/// One id for all three bindings deliberately: the boundary's whole AC-031 check is
/// `typed.schema == spec.schema`, and a script whose entries had to be matched to nodes by
/// position would make this file's fixture depend on the plan's order rather than on the bindings.
#[cfg(feature = "corpus")]
const RECORDED_SCHEMA: &str = "RecordedResult/v1";

/// **The dispatch bindings, as RECORDED INPUTS** — one per DAG node the amended plan carries.
///
/// 🛑 **Why they are supplied and not resolved.** `rk_types::activity::ActivitySpec` has five
/// fields and this tree resolves none of them from a DAG node [measured 2026-09-14: `card_digest`,
/// `tools` and `schema` appear in **zero** catalog fixtures; `timeout_secs` is declared in ten and
/// parsed by nothing]. The spec is the **binder's** output and the binder is unbuilt, so at Step 7b
/// it is a logged input and **a record routing spec assembly is OWED** —
/// `rk_kernel::replay::RecordedSpec`'s own doc carries the same statement beside the code.
///
/// `activity_id_at_version` is here for the sibling reason: the join between a `DagNode` and
/// `Activity::id_at_version` has three candidate homes and `workflow::fsm`'s header records that
/// choosing one is a decision this stage may not take. The versions below are the ones the
/// **committed catalog fixtures declare** (`version = 1` in each of the three), transcribed.
#[cfg(feature = "corpus")]
const BINDINGS: [(&str, &str, &str); 3] = [
    ("change-summary[node]", "change-summary@1", "ChangeSummary"),
    ("impl-report", "impl-report@1", "ImplReport"),
    ("implement-change[node]", "implement-change@1", "Changeset"),
];

#[cfg(feature = "corpus")]
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root resolves")
}

/// **The staging tree, named once.** `target/golden-logs-staging/` — untracked, so AC-465 holds,
/// and stable, so the operator's move is a copy of a named directory rather than a search.
#[cfg(feature = "corpus")]
fn staging_root() -> PathBuf {
    workspace_root().join("target/golden-logs-staging")
}

/// Scratch for the perturbation case. **Deliberately NOT under [`staging_root`]**: the staging tree
/// is what the operator's single `CORPUS_PIN` move copies, and a deliberately-corrupted run sitting
/// in it would be copied into the corpus with the good ones.
#[cfg(feature = "corpus")]
fn scratch_root() -> PathBuf {
    workspace_root().join("target/golden-log-scratch")
}

/// The committed in-crate copies (§7b.12's `logs/` row).
#[cfg(feature = "corpus")]
fn committed_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/logs")
}

/// The one run every assertion in this file is made over.
///
/// It is the **deviation** fixture deliberately: a run carrying `dag_assembled` **and**
/// `dag_amended` is the only one that makes AC-064's second AND — *"the deviation record is part of
/// the replay input set, so an amended DAG is not a hidden input"* — checkable at all.
#[cfg(feature = "corpus")]
fn inputs() -> RunInputs {
    let facets_src = std::fs::read_to_string(workspace_root().join("spikes/p9/facets.toml"))
        .expect("the shipped facet registry is readable");

    let mut inputs = RunInputs::new(STAGE)
        .with_runtime(
            // Supplied, never derived, and recorded as part of the input set. `entered_from` and
            // `advances_to` are both present so neither branch of the fold's `optional_str`
            // reading nor `next_action`'s successor argument is exercised only by its absence.
            RuntimeContext::supplied(TREE_HASH, DEADLINE)
                .entered_from("triage")
                .advances_to("verify")
                .results_at(RESULTS_AT),
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
        .requested_by("impl-report");

    // One binding per node the ASSEMBLED dag carries — including `change-summary[node]`, which the
    // accepted deviation removes. Recording a binding for a node the amended plan never dispatches
    // is deliberate: the input set is a function of the DAG, not of the amendment, so the log does
    // not quietly encode the outcome of step 7 in what it supplies.
    for (node, activation, kind) in BINDINGS {
        inputs = inputs.with_spec(
            node,
            RecordedSpec::supplied(
                activation,
                ActivitySpec {
                    card_digest: Digest::from_hex(&card_digest_hex(node))
                        .expect("64 lowercase hex characters"),
                    tools: vec!["Read".to_owned()],
                    schema: SchemaId::new(RECORDED_SCHEMA),
                    timeout_secs: 900,
                    inputs: vec![
                        ("item".to_owned(), serde_json::json!(ITEM)),
                        ("kind".to_owned(), serde_json::json!(kind)),
                    ],
                    activity_id_at_version: None,
                },
            ),
        );
    }
    inputs
}

/// A distinct, derived card digest per node. **Derived and not a constant**: three bindings sharing
/// one digest would let a spec mixed up between two nodes reach the port unnoticed.
#[cfg(feature = "corpus")]
fn card_digest_hex(node: &str) -> String {
    rk_types::digest::sha256(node.as_bytes())
        .as_str()
        .to_owned()
}

/// **The recording's result source: a scripted `Harness` that COUNTS.**
///
/// AC-008's fake fails the test on any dispatch, and that is [`RefusingHarness`] — bound on the
/// **replay** side, which is where the criterion's zero is asserted. A recording has to dispatch,
/// so the fake bound here answers instead, and its counter is what makes *"N is not 0"* a reading.
#[derive(Debug, Default)]
#[cfg(feature = "corpus")]
struct RecordingHarness {
    dispatches: AtomicU32,
}

#[cfg(feature = "corpus")]
impl RecordingHarness {
    fn dispatches(&self) -> u32 {
        self.dispatches.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(feature = "corpus")]
impl Harness for RecordingHarness {
    fn dispatch(&self, spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        self.dispatches
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        DispatchOutcome {
            record: DispatchRecord {
                argv_digest: rk_types::digest::sha256(b"recording-argv"),
                env_digest: rk_types::digest::sha256(b"recording-env"),
                instrument_version: "recording-fake".to_owned(),
                stdout: Retained::of(b""),
                stderr: Retained::of(b""),
                // ADR-0126's members (Step 7c Slice 1), in the pre-spawn shape: this fake spawns
                // nothing and reads no `system/init`, so the version is `Pinned` and every
                // usage-adjacent member is `None`, never `Some(0)` (AC-362).
                usage: None,
                timings: Timings {
                    wall_ms: 0,
                    api_ms: None,
                },
                total_cost_usd: None,
                headroom_before: None,
                headroom_after: None,
                cli_version: CliVersion::Pinned("recording-fake".to_owned()),
                api_retry_events: 0,
                prompt_cache_ttl: "1h".to_owned(),
                subagent_prompt_cache_ttl: "1h".to_owned(),
                cancelled_by: None,
                detected_by: None,
                activity_id_at_version: None,
                // *(dated 2026-10-02, s8h build, package P2-rk-types; ADR-0197, proposed.)* No init read here.
                observed_model: None,
                settings_file: None,
            },
            // **A function of the SPEC, never of the call order.** The value is derived from the
            // card digest and the kind the binding supplied, so two runs of identical inputs
            // produce identical bytes; a counter or a clock in this value would move
            // `events.jsonl` on every run and make the byte-identity assertion untestable. It also
            // differs per node, so a spec delivered to the wrong dispatch is visible in the
            // journal rather than absorbed.
            result: Ok(Typed::new(
                SchemaId::new(RECORDED_SCHEMA),
                serde_json::json!({
                    "card": spec.card_digest.as_str(),
                    "produced": spec
                        .inputs
                        .iter()
                        .find(|(k, _)| k == "kind")
                        .map_or(serde_json::Value::Null, |(_, v)| v.clone()),
                }),
            )),
        }
    }
}

/// Every port bound, for the **recording**. The three this run never calls are `rk-ports`' own
/// fakes; a `Runtime` cannot be constructed over four, which is what makes *"every port bound"* a
/// type fact here as it is on the replay side.
#[cfg(feature = "corpus")]
fn recording_ports(harness: &Arc<RecordingHarness>) -> Ports {
    Ports {
        store: Arc::new(Mutex::new(CountingStore::default())) as Arc<Mutex<dyn EventStore>>,
        harness: Arc::clone(harness) as Arc<dyn Harness>,
        tracker: Arc::new(BoardTracker::default()) as Arc<dyn Tracker>,
        docs: Arc::new(RecordingDocs::default()) as Arc<dyn DocsSite>,
        toolchain: Arc::new(FixtureToolchain::default()) as Arc<dyn Toolchain>,
    }
}

/// Record the one run this file is about, and hand back the fake that answered so its dispatch
/// count is a **reading** rather than a number the recording printed about itself.
#[cfg(feature = "corpus")]
fn record_the_run() -> (rk_kernel::replay::RecordedRun, Arc<RecordingHarness>) {
    let harness = Arc::new(RecordingHarness::default());
    let recorded = rk_kernel::replay::record(&inputs(), recording_ports(&harness))
        .expect("the recorded run resolves");
    (recorded, harness)
}

// ---------------------------------------------------------------------------- the bound fakes

/// The `Harness` fake AC-008 names: **it fails the test on any `dispatch` call.**
///
/// It implements `rk_ports::harness::Harness` and **nothing else**. There is no kernel-side
/// `ReplayHarness: Harness` supertrait any more: the dispatch counter is this fake's own
/// `AtomicU32`, handed to the replay as `ReplayPorts::dispatches` and read there before and after.
#[derive(Debug, Default)]
#[cfg(feature = "corpus")]
struct RefusingHarness {
    dispatches: AtomicU32,
}

#[cfg(feature = "corpus")]
impl RefusingHarness {
    fn dispatches(&self) -> u32 {
        self.dispatches.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(feature = "corpus")]
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
#[cfg(feature = "corpus")]
struct CountingStore {
    events: Vec<Event>,
}

#[cfg(feature = "corpus")]
impl EventStore for CountingStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        self.events.push(event);
        Ok(())
    }

    /// *(dated 2026-09-28, s8e build; ADR-0181, proposed.)* The port's fifth method on this test
    /// double. The anchor is not modelled here, so the row is appended exactly as `append`
    /// appends it; the anchored behaviour is `VecStore`'s and `SqliteStore`'s, under test there.
    fn append_tier1(
        &mut self,
        event: Event,
        _anchor: &rk_ports::event_store::TierOneAnchor,
    ) -> Result<(), StepError> {
        self.append(event)
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
#[cfg(feature = "corpus")]
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
#[cfg(feature = "corpus")]
fn bless_the_committed_golden_log() {
    let (recorded, _harness) = record_the_run();
    let dir = rk_kernel::replay::write_run(&committed_root(), &recorded)
        .expect("the fixture tree is writable");
    println!("wrote {}", dir.display());
}

/// The `run_id` this exact input set derived while [`rk_kernel::replay::SCHEMA`] read
/// `rk-kernel/golden-log/v1`.
///
/// **Read off the committed fixture, never computed here.**
/// `tests/fixtures/logs/implement-a539b5a7b654d16f/run.json` at `7437a26` carried
/// `"run_id":"implement-a539b5a7b654d16f"` beside `"schema":"rk-kernel/golden-log/v1"`, and the
/// directory that name spells is the tree that stood before the bump.
#[cfg(feature = "corpus")]
const V1_RUN_ID: &str = "implement-a539b5a7b654d16f";

/// The selection's own `inputs_digest` at that same commit — **and the discriminator.**
///
/// It is AC-041's four-digest tuple as `select()` derived it, and the layout version is **not**
/// among its inputs: [`rk_kernel::replay::SCHEMA`] reaches `run_id` and reaches nothing else. So
/// it must be byte-identical across the bump while the run id moves. Without it, *"the run id
/// changed"* would be satisfied by anything at all changing under the derivation; with it, what
/// moved is the schema string and nothing else.
#[cfg(feature = "corpus")]
const V1_INPUTS_DIGEST: &str = "3345ce0749d822cf08e28883d9dc18db36d1741fdf60cd94f04d1f91fd0a719c";

/// **THE LAYOUT VERSION IS AN INPUT TO `run_id`, AND NOTHING CHECKED THAT IT WAS.**
///
/// §7b.11 fixes `run_id = sha256_lp([SCHEMA, stage, canonical(inputs)])`, and the operator's
/// decision of 2026-09-14 (**D-4**) bumps `SCHEMA` to `rk-kernel/golden-log/v2` **before** the
/// single `CORPUS_PIN` move: three distinct shapes had come to share `v1`, against §7b.11's own
/// stated reason for carrying a version — *"so a reader never guesses which shape a run is in"* —
/// and after the move the rename would invalidate corpus-committed run ids under a SHA AC-464
/// freezes, which `PROGRESS`:103 forbids re-taking.
///
/// # Why this exists rather than the re-bless being its own evidence
///
/// [`bless_the_committed_golden_log`] writes the fixture tree at whatever name the derivation
/// produces, and [`golden_log_replay`] then joins `committed_root()` with that **same** derived
/// name. **If the schema string never reached the derivation, both sides would agree on the old
/// name and every replay test would stay green** — the bump would have been performed, recorded
/// in the diff, and be worth nothing, with nothing red anywhere. The whole warrant for bumping
/// *now* rather than after the pin move is that the string renames runs; a bump that renamed
/// nothing would have spent the one act `PROGRESS`:103 allows on a no-op. This is that check, and
/// it is worth its lines precisely because its failure would otherwise be silent.
///
/// # What discriminates
///
/// The run id must have **moved off** [`V1_RUN_ID`] while `inputs_digest` **stays at**
/// [`V1_INPUTS_DIGEST`]. One without the other proves nothing: a run id that changed alongside a
/// changed input set says only that something moved, and an unchanged input set alone says only
/// that the fixture was not edited. Together they say the layout version — and only the layout
/// version — reached the derivation.
///
/// Part 5 is the other half of the same decision: a log written under the old version is
/// **refused by [`rk_kernel::replay::read_run`], and the refusal names both versions**, so a
/// reader of the error learns *why* the log was rejected rather than only *that* it was.
///
/// # 🛑 THE RUNBOOK HALF IS THE OPERATOR'S AND IS **NOT** DONE HERE
///
/// §7b.11's own amendment (committed `0780198`) says the bump *"lands in ONE act touching both
/// files"* and deliberately left the block reading `v1`, because *"a runbook that described a
/// version the code does not write would be the documented-but-false state this file has been
/// repaired for twice already."* `runbook/` is outside this session's editable set, so what now
/// stands is the mirror image of the state that amendment refused to create: **the code writes
/// `v2` and §7b.11 still says `v1`.** A dated §7b.11 amendment carrying `v2`, and the **re-read
/// of precondition 3** that block itself asks for, are **OWED to the operator**; nothing in this
/// file may be read as having discharged either.
///
/// **This test's name carries no AC id** for the same reason the D-5, D-6, D-7 and D-10 phases'
/// pins carry none — a record is owed for the reading — on
/// `appetite_raise_from_an_activity_result_is_refused`'s precedent.
#[test]
#[cfg(feature = "corpus")]
fn the_layout_version_reaches_the_run_id_derivation() {
    // ---- 1. the bump itself, pinned as a literal -------------------------------------------
    //
    // A literal and not a `starts_with("rk-kernel/golden-log/")`: what D-4 decided is a specific
    // version, and a prefix check would be green against the `v1` it exists to have left.
    assert_eq!(
        rk_kernel::replay::SCHEMA,
        "rk-kernel/golden-log/v2",
        "D-4 bumps the golden-log layout version to v2, before the CORPUS_PIN move"
    );

    // ---- 2. the derivation MOVED ------------------------------------------------------------
    //
    // `inputs()` is the pure input set — the same one `record_the_run` folds — so this is the
    // derivation itself, not a value read back out of a file something else wrote.
    let derived = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    assert_ne!(
        derived, V1_RUN_ID,
        "THE SCHEMA STRING IS NOT REACHING `run_id`. `SCHEMA` reads v2 and this input set still \
         derives the v1 name {V1_RUN_ID}, so bumping the version renamed no run — and the whole \
         warrant for bumping NOW rather than after the pin move (§7b.11: the string is an INPUT \
         to the derivation) is void"
    );

    // ---- 3. exactly one tree replaced one tree ----------------------------------------------
    let committed = committed_root().join(&derived);
    assert!(
        committed.is_dir(),
        "the committed in-crate copy is missing at {} — re-bless it with `cargo test -p \
         rk-kernel --features corpus --test it -- --ignored bless_the_committed_golden_log`",
        committed.display()
    );
    assert!(
        !committed_root().join(V1_RUN_ID).exists(),
        "the v1 tree {V1_RUN_ID} is STILL on disk beside the v2 one. `write_run` removes nothing \
         outside the directory it writes, so a re-bless ADDS a tree; the superseded one is \
         deleted deliberately, or §7b.12's `logs/` row carries two runs of the same inputs under \
         two layout versions and `read_run` accepts only one of them"
    );

    // ---- 4. the manifest agrees, and NOTHING BUT the version moved --------------------------
    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(committed.join("run.json"))
            .expect("the committed manifest is readable"),
    )
    .expect("run.json parses");
    assert_eq!(
        manifest["schema"].as_str(),
        Some(rk_kernel::replay::SCHEMA),
        "the committed manifest was not re-blessed after the bump"
    );
    assert_eq!(
        manifest["run_id"].as_str(),
        Some(derived.as_str()),
        "the committed manifest's run_id is not the one this input set derives"
    );
    assert_eq!(
        manifest["inputs_digest"].as_str(),
        Some(V1_INPUTS_DIGEST),
        "THE INPUT SET MOVED TOO, so the rename above is not attributable to the schema string. \
         `inputs_digest` is the SELECTION's digest and the layout version is not among its \
         inputs, so it is byte-identical across a version bump — and across nothing else"
    );

    // ---- 5. AN OLDER LOG IS REFUSED BY VERSION, AND THE REFUSAL NAMES BOTH -------------------
    //
    // The control comes first: the untouched copy reads back, so the refusal below is
    // attributable to the one member this test rewrites and not to a broken copy.
    let stale = scratch_root().join("schema-v1");
    copy_tree(&committed, &stale);
    rk_kernel::replay::read_run(&stale)
        .expect("PREMISE: the unperturbed copy reads back, so the refusal below is the rewrite's");

    let manifest_path = stale.join("run.json");
    let mut rewritten: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path).expect("the copied manifest is readable"),
    )
    .expect("run.json parses");
    rewritten["schema"] = serde_json::Value::from("rk-kernel/golden-log/v1");
    assert_ne!(
        rewritten["schema"].as_str(),
        Some(rk_kernel::replay::SCHEMA),
        "PREMISE: the rewrite must name a DIFFERENT version, or `read_run` would be asked to \
         refuse a log it should accept"
    );
    std::fs::write(
        &manifest_path,
        serde_json::to_vec(&rewritten).expect("serialisable"),
    )
    .expect("the scratch copy is writable");

    match rk_kernel::replay::read_run(&stale) {
        Err(ReplayError::Malformed { detail, .. }) => {
            assert!(
                detail.contains("rk-kernel/golden-log/v1"),
                "the refusal must name the version the log CARRIES, so a reader learns why it was \
                 rejected rather than only that it was: {detail}"
            );
            assert!(
                detail.contains(rk_kernel::replay::SCHEMA),
                "the refusal must name the version this build READS: {detail}"
            );
        }
        Ok(_) => panic!(
            "`read_run` ACCEPTED a log stamped with an older layout version. The version is \
             carried *\"so a reader never guesses which shape a run is in\"*, and a reader that \
             accepts every shape is the guess"
        ),
        Err(other) => panic!("expected Malformed naming both versions, got {other}"),
    }
}

/// **AC-006 / AC-008 — and the first replay in this system's history that had anything to serve.**
///
/// Read the module header before reading this green. The sentence that stood here said *"assertions
/// 3 and the terminal-state half of 5 are **not buildable without an FSM** and are not claimed"*;
/// the FSM landed at Slice 4e, the recorded side of both comparisons at 4f, and **the N dispatched
/// activities AC-008's GIVEN names at 4i**. What is still not claimed is named in the header's
/// last 🛑 paragraph: these are a scripted fake's dispatches, not an adapter's.
#[test]
#[cfg(feature = "corpus")]
fn golden_log_replay() {
    // ---- 1. produce the log. THIS RUN produces it; nothing here is hand-made ------------------
    let (recorded, recording_harness) = record_the_run();
    assert!(
        !recorded.events.is_empty(),
        "an empty journal would satisfy every comparison below vacuously"
    );
    let staged = rk_kernel::replay::write_run(&staging_root(), &recorded)
        .expect("the staging tree is writable");

    // ---- 1b. AC-008's GIVEN: **a run that dispatched N activities** ---------------------------
    //
    // This is the premise the whole criterion hangs on, and until 2026-09-14 it was false: the
    // recorded journal held 4 events and **zero** `dispatch` and `activity_committed` rows, so
    // *"all N results served from the journal"* was a statement about an empty set and a
    // do-nothing kernel satisfied it. `record()` now drives the FSM's `Dispatch` through the
    // bound `Harness`, and the two counts below are what make N real.
    let recorded_dispatches = crate::runtime::count_kind(&recorded.events, EventKind::Dispatch);
    let recorded_commits =
        crate::runtime::count_kind(&recorded.events, EventKind::ActivityCommitted);
    assert!(
        recorded_dispatches > 0,
        "AC-008's GIVEN is a replay of a run that DISPATCHED N activities, and N is \
         {recorded_dispatches}. With N = 0 the dispatch half of the criterion is vacuous and the \
         green below discharges nothing"
    );
    assert_eq!(
        recorded_dispatches, recorded_commits,
        "every dispatch this recording made committed, so the N the replay serves back is the N \
         the recording produced"
    );
    // …and the rows are not a decoration the recorder wrote about itself: the RECORDING's own
    // fake was reached exactly N times. The counter is the fake's, read on this thread.
    assert_eq!(
        recording_harness.dispatches(),
        u32::try_from(recorded_dispatches).expect("N fits a u32"),
        "the `dispatch` rows in the journal are the ones the bound Harness produced, not rows the \
         recorder minted beside a port it never reached"
    );

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

    // assertion 5c — **ALL N RESULTS SERVED FROM THE JOURNAL**, and the discriminating reading is
    // `report.events_compared`, NOT the store.
    //
    // 🛑 **The store would say `matched` either way and that is worth stating.** `replay()` serves
    // the RECORDED journal to the bound `EventStore` at the end of its own body, so counting
    // `dispatch` rows in `store.events` measures that loop and nothing about the re-derivation. It
    // is the equality above — `events_compared == recorded.events.len()` with `divergences == 0`,
    // over a journal whose last {recorded_dispatches} × 2 rows ARE the dispatches — that says the
    // replay's own `next_action` → `Dispatch` → obtain → fold loop asked for every one of the N
    // results and got them from the recording: a loop that served none would re-derive a
    // {short}-event journal and the length comparison inside `replay()` would refuse.
    // `golden_log_replay_missing_result_is_red` is the positive control for that refusal.
    let short = recorded.events.len() - (recorded_dispatches + recorded_commits);
    assert_eq!(
        report.events_compared,
        recorded.events.len(),
        "AC-008: the re-derivation reached all {recorded_dispatches} dispatches and all \
         {recorded_commits} results, not the {short}-event prefix a loop that served nothing \
         would produce"
    );
    assert!(
        short < report.events_compared,
        "PREMISE: the journal must actually carry dispatch rows for the comparison above to be \
         about them"
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
         terminal_state={} next_action_bytes={} recorded_dispatches={} results_served={}",
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
        recorded_dispatches,
        crate::runtime::count_kind(&store.events, EventKind::ActivityCommitted),
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
#[cfg(feature = "corpus")]
fn golden_log_replay_terminal_state_mismatch_is_red() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let source = committed_root().join(&run_id);

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
                d.activity_id, run_id,
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
#[cfg(feature = "corpus")]
fn golden_log_replay_next_action_mismatch_is_red() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let source = committed_root().join(&run_id);

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
                d.activity_id, run_id,
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

/// **THE POSITIVE CONTROL FOR *"all N results served from the journal"*** — the half of AC-008
/// that had no instrument at all before 2026-09-14, because N was 0 and an absence over an empty
/// set needs no serving.
///
/// `golden_log_replay` asserts that the re-derivation reached every recorded event. This asserts
/// the other direction: with the recorded `activity_committed` rows **removed** from a scratch
/// copy, the FSM asks for a result the journal cannot give, and the replay must **refuse** with
/// `ReplayError::JournalHasNoResult`. A replay that skipped the step instead would serve N-1
/// results, fold on, and report a green — which is precisely the silent shortfall AC-008's
/// *"**all** N"* rules out.
///
/// It is also the control for the loop's termination: a replay that neither served nor refused
/// would ask for the same activation for ever.
#[test]
#[cfg(feature = "corpus")]
fn golden_log_replay_missing_result_is_red() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let source = committed_root().join(&run_id);

    let stripped = scratch_root().join("results-stripped");
    copy_tree(&source, &stripped);
    let events_path = stripped.join("events.jsonl");
    let text = std::fs::read_to_string(&events_path).expect("the journal is readable");
    let kept: Vec<&str> = text
        .lines()
        .filter(|line| !line.contains(r#""kind":"activity_committed""#))
        .collect();
    assert!(
        kept.len() < text.lines().count(),
        "PREMISE: the committed journal must actually carry `activity_committed` rows, or this \
         control removes nothing and asserts a refusal over an unchanged log"
    );
    std::fs::write(&events_path, format!("{}\n", kept.join("\n"))).expect("writable");

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

    match rk_kernel::replay::replay(&stripped, &mut ports) {
        Err(ReplayError::JournalHasNoResult {
            step_id,
            activity_id_at_version,
            ..
        }) => {
            assert!(
                !step_id.is_empty() && activity_id_at_version.contains('@'),
                "the refusal names the activation it could not serve: {step_id} / \
                 {activity_id_at_version}"
            );
            assert_eq!(
                harness.dispatches(),
                0,
                "and it refused rather than falling back to the port — a replay that dispatched \
                 on a journal miss would spend money to paper over a divergence"
            );
        }
        Ok(report) => panic!(
            "the replay served {} events from a journal with no results in it — the loop is not \
             asking the journal for anything",
            report.events_compared
        ),
        Err(other) => panic!("expected JournalHasNoResult, got {other}"),
    }
}

/// **AC-042's hidden input, on the RECORDED INPUT SET rather than on the journal.**
///
/// Added 2026-09-14. `replay()` read `inputs_digest` out of the manifest and compared it to
/// nothing, and never re-derived `run_id`, so a rewritten `inputs/` was accepted by `replay()`
/// itself. `golden_log_replay` caught it only because that test re-records and byte-compares
/// against the committed copy — a check that does **not** travel to `redkiln replay --run <id>`
/// (7d's), which runs against an arbitrary recorded run with no committed copy to compare to.
///
/// The tamper is the mildest available: one `timeout_secs`, on one activation, in a value the
/// selection never reads. It must still refuse, because the input set is hashed whole.
#[test]
#[cfg(feature = "corpus")]
fn golden_log_replay_tampered_input_is_red() {
    // **AC-042's hidden input, on the RECORDED INPUT SET rather than on the journal.** Added
    // 2026-09-14: `replay()` read `inputs_digest` out of the manifest and compared it to nothing,
    // and never re-derived `run_id`, so a rewritten `inputs/` was accepted by `replay()` itself.
    // The golden-log test caught it only because that test re-records and byte-compares against
    // the committed copy — a check that does NOT travel to `redkiln replay --run <id>` (7d's),
    // which runs against an arbitrary recorded run with no committed copy to compare against.
    //
    // The tamper below is the mildest one available: a single `timeout_secs` on ONE activation,
    // in a value the selection never reads. It must still refuse, because the input set is hashed
    // whole.
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let source = committed_root().join(&run_id);

    let tampered = scratch_root().join("inputs-tampered");
    copy_tree(&source, &tampered);
    let specs_path = tampered.join("inputs").join("specs.json");
    let mut specs: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&specs_path).expect("readable"))
            .expect("specs.json parses");
    let key = specs
        .as_object()
        .expect("an object")
        .keys()
        .next()
        .expect("at least one recorded binding")
        .clone();
    let before = specs[&key]["timeout_secs"].clone();
    specs[&key]["timeout_secs"] = serde_json::Value::from(1);
    assert_ne!(
        before, specs[&key]["timeout_secs"],
        "PREMISE: the rewrite must actually change the input set, or the replay would be asked to \
         refuse a log it should accept"
    );
    std::fs::write(
        &specs_path,
        serde_json::to_vec(&specs).expect("serialisable"),
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

    match rk_kernel::replay::replay(&tampered, &mut ports) {
        Err(ReplayError::Divergent(hidden)) => {
            assert_eq!(hidden.run_id, run_id, "the halt names the run it refused");
            assert!(
                hidden.first_divergent_node.contains("inputs/"),
                "the halt must say the INPUT SET diverged, not a journal row: {}",
                hidden.first_divergent_node
            );
        }
        Ok(_) => panic!(
            "the replay ACCEPTED a rewritten input set. `replay()` is then trusting bytes it never \
             checked, and AC-042's re-derivation *\"from the logged inputs alone\"* is over inputs \
             the recording never saw"
        ),
        Err(other) => panic!("expected Divergent(HiddenInput), got {other}"),
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
#[cfg(feature = "corpus")]
fn ac161_ac164_delete_db_replay() {
    let (recorded, _harness) = record_the_run();

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
#[cfg(feature = "corpus")]
fn golden_log_replay_writing_fake_is_red() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let dir = committed_root().join(&run_id);

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
#[cfg(feature = "corpus")]
fn golden_log_replay_dispatching_fake_is_red() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let dir = committed_root().join(&run_id);

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
#[cfg(feature = "corpus")]
fn replay_reports_hidden_input_on_divergence() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let source = committed_root().join(&run_id);

    let run_id_expected = run_id.clone();
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
            assert_eq!(run_id, run_id_expected);
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
#[cfg(feature = "corpus")]
fn the_logged_inputs_are_sufficient_to_rebuild_the_run() {
    let (recorded, _harness) = record_the_run();
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

#[cfg(feature = "corpus")]
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

#[cfg(feature = "corpus")]
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

#[cfg(feature = "corpus")]
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

/// **The activation-match check in `ServedJournal::serve` had no control, and this is it.**
///
/// *(Authored 2026-09-14 on an adversarial verification pass, which reproduced the gap: deleting
/// the `if item != key.item || activation != key.activity_id_at_version { break; }` guard at
/// `src/replay.rs`'s `ServedJournal::serve` left **every replay test green**.)*
///
/// # Why this is the only non-tautological fact in that region
///
/// The rows the journal serves back are **clones of the recorded rows**, and the re-derived journal
/// is then compared byte for byte against the recorded one. Over the served region that comparison
/// is **tautological**: the same bytes are being compared with themselves. Three facts survive it,
/// and only three — the **count** (`results_served=N`, controlled by
/// `golden_log_replay_missing_result_is_red`), the **per-activation commit** (the same control's
/// second half), and **that the FSM asked for the right activation in the right order**. The third
/// was pinned by nothing, and it is the one that carries AC-008's *"served once, in order"*.
///
/// # The control
///
/// Copy the log and **swap the two recorded activation pairs** in `events.jsonl` — the same four
/// rows, the same bytes, a different order. The FSM's first request is then answered by a run of
/// rows belonging to the *other* activation, and `serve` must refuse with
/// [`ReplayError::JournalHasNoResult`] naming what the journal offered instead. With the guard
/// deleted, `serve` takes rows up to the first `activity_committed` regardless of whose it is, and
/// **the wrong activation's result is served for the request** — which is a replay that agrees with
/// a recording it did not reproduce.
///
/// **Two premises make it discriminating:** the unperturbed copy is replayed successfully first, so
/// a refusal below cannot be about a broken copy; and the swap is asserted to have actually changed
/// the file's bytes, so it cannot be a no-op over a log with one activation.
#[test]
#[cfg(feature = "corpus")]
fn golden_log_replay_swapped_activations_is_red() {
    let run_id = inputs()
        .run_id()
        .expect("the run id is a content hash of the inputs");
    let source = committed_root().join(&run_id);

    // ---- PREMISE 1: the UNPERTURBED copy replays ------------------------------------------------
    let control = scratch_root().join("activations-unswapped");
    copy_tree(&source, &control);
    {
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
        rk_kernel::replay::replay(&control, &mut ports)
            .expect("the unperturbed copy replays, or the refusal below is about the copy");
    }

    // ---- the swap: the same four rows, a different order ----------------------------------------
    let swapped = scratch_root().join("activations-swapped");
    copy_tree(&source, &swapped);
    let events_path = swapped.join("events.jsonl");
    let before = std::fs::read_to_string(&events_path).expect("readable");

    let mut lines: Vec<String> = before.lines().map(str::to_owned).collect();
    let activation_of = |line: &str| -> Option<(String, String)> {
        let v: serde_json::Value = serde_json::from_str(line).ok()?;
        let kind = v["kind"].as_str()?;
        if kind != "dispatch" && kind != "activity_committed" {
            return None;
        }
        Some((
            v["payload"]["item"].as_str()?.to_owned(),
            v["payload"]["activity_id_at_version"].as_str()?.to_owned(),
        ))
    };
    let indices: Vec<usize> = (0..lines.len())
        .filter(|i| activation_of(&lines[*i]).is_some())
        .collect();
    let activations: Vec<(String, String)> = {
        let mut seen: Vec<(String, String)> = Vec::new();
        for i in &indices {
            let a = activation_of(&lines[*i]).expect("filtered above");
            if !seen.contains(&a) {
                seen.push(a);
            }
        }
        seen
    };
    // PREMISE 2: there are TWO activations to swap. With one, the swap is a no-op and the refusal
    // below would never be asked for — which is the state this log was in before Slice 4i.
    assert_eq!(
        activations.len(),
        2,
        "the committed log must carry two recorded activations for this control to mean anything; \
         got {activations:?}"
    );
    assert_eq!(
        indices.len(),
        4,
        "two activations, a dispatch and a commit each"
    );

    let second_first: Vec<String> = [&activations[1], &activations[0]]
        .into_iter()
        .flat_map(|want| {
            indices
                .iter()
                .filter(|i| activation_of(&lines[**i]).as_ref() == Some(want))
                .map(|i| lines[*i].clone())
                .collect::<Vec<_>>()
        })
        .collect();
    assert_eq!(second_first.len(), indices.len());
    for (slot, line) in indices.iter().zip(second_first) {
        lines[*slot] = line;
    }
    let after = format!("{}\n", lines.join("\n"));
    assert_ne!(
        before, after,
        "PREMISE: the swap must actually change the file, or the replay is being asked to refuse \
         the log it already accepts"
    );
    std::fs::write(&events_path, after).expect("writable");

    // ---- and the replay REFUSES -----------------------------------------------------------------
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

    match rk_kernel::replay::replay(&swapped, &mut ports) {
        Err(ReplayError::JournalHasNoResult {
            activity_id_at_version,
            served_instead,
            ..
        }) => {
            let offered = served_instead.expect(
                "the journal offered a row belonging to the other activation, and the halt must \
                 NAME it — an `Option::None` here would report an absence where there was a \
                 substitution",
            );
            assert!(
                !offered.contains(&activity_id_at_version),
                "the halt must name a DIFFERENT activation as what was served instead; it asked \
                 for {activity_id_at_version} and reports {offered}"
            );
        }
        Ok(_) => panic!(
            "the replay ACCEPTED a journal whose two activation pairs were swapped — the served \
             rows are CLONES of the recorded rows, so the byte comparison over that region is \
             tautological and the only fact left is that the FSM asked for the right activation \
             in the right order. This is that fact going unchecked"
        ),
        Err(other) => panic!("expected JournalHasNoResult, got {other}"),
    }
    assert_eq!(
        harness.dispatches(),
        0,
        "AC-008: a refusal is not a licence to dispatch"
    );
}

/// **R3: the interim `item` key moves no golden log's canonical digest** (D21-46 R3; ADR-0182
/// decision 6; `s8e` §8e.10). Ungated on purpose (the module note above).
///
/// - **Arm 1, a pin:** every committed golden log's canonical digest equals the value `s8e` §8e.0
///   item 3 recorded at `766854c`, printed then from inside this binary as
///   `rk_types::digest::sha256(read_run(dir).canonical_events())`. Under `--features corpus` the
///   copy under `CORPUS_PIN` is asserted to be the same bytes at the pinned sha.
/// - **Arm 2:** the same journal with the key set on every event has the same canonical digest. A
///   carrier that wrote the key into `payload` (S8E-M35) reddens this arm.
///
/// RED at entry: no key carrier — `Event::with_item` does not exist (E0599), so arm 2 cannot be
/// written; arm 1 alone is a pin, green before and after.
#[test]
fn golden_log_canonical_digests_are_unchanged_by_the_interim_item_key() {
    const RECORDED: [(&str, &str); 1] = [(
        "implement-bf1d8cec3b22d288",
        "73f7553df2f84cc7b931a39b276fcfdb0b36371bb93ea461a9b10ca8a47e4e90",
    )];
    let logs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/logs");
    let mut on_disk: Vec<String> = std::fs::read_dir(&logs)
        .expect("the committed golden logs read")
        .filter_map(Result::ok)
        .filter(|e| e.path().join("events.jsonl").is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    on_disk.sort();
    assert_eq!(
        on_disk,
        RECORDED
            .iter()
            .map(|(n, _)| (*n).to_owned())
            .collect::<Vec<_>>(),
        "PREMISE: every committed golden log has a digest recorded at stage open (a log added \
         later needs its own recorded value)"
    );
    for (name, want) in RECORDED {
        let mut run = rk_kernel::replay::read_run(&logs.join(name)).expect("the golden log reads");
        let bytes = run.canonical_events().expect("canonical events");
        assert_eq!(
            rk_types::digest::sha256(&bytes).as_str(),
            want,
            "arm 1: {name}'s canonical digest moved from its recorded value"
        );
        assert!(!run.events.is_empty(), "PREMISE: {name} holds events");
        run.events = run
            .events
            .iter()
            .map(|e| e.clone().with_item("INC-1"))
            .collect();
        assert!(
            run.events.iter().all(|e| e.item() == Some("INC-1")),
            "PREMISE: every event now carries the interim key"
        );
        let keyed = run.canonical_events().expect("canonical events, keyed");
        assert_eq!(
            rk_types::digest::sha256(&keyed).as_str(),
            want,
            "arm 2: carrying the interim key moved {name}'s canonical digest — the key is inside \
             `payload` or `event_json` (ADR-0131's forbids; D21-46 R3)"
        );
    }

    #[cfg(feature = "corpus")]
    {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let text = std::fs::read_to_string(root.join("CORPUS_PIN")).expect("CORPUS_PIN reads");
        let get = |k: &str| -> String {
            text.lines()
                .find_map(|l| l.trim().strip_prefix(&format!("{k}=")))
                .unwrap_or_else(|| panic!("CORPUS_PIN carries no {k}"))
                .trim()
                .to_owned()
        };
        let repo = std::path::PathBuf::from(get("CORPUS_REPO"));
        let sha = get("CORPUS_SHA");
        for (name, _) in RECORDED {
            let shown = rk_git::git_exec::run(
                &repo,
                ["show", &format!("{sha}:golden-logs/{name}/events.jsonl")],
            )
            .expect("the corpus golden log reads at the pin");
            let committed = std::fs::read(logs.join(name).join("events.jsonl")).expect("read");
            assert_eq!(
                shown.stdout, committed,
                "the corpus copy of {name} at CORPUS_SHA is not the committed copy's bytes"
            );
        }
    }
}
