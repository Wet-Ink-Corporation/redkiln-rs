//! **The runtime boundary** — `src/runtime.rs` and `src/runtime/activity.rs`. Slice 4h.
//!
//! This is the first file in the workspace that CALLS `rk-ports`. Everything before it declared the
//! edge and asserted things about the dependency graph; `crates/rk-kernel/Cargo.toml` said so in
//! terms until 2026-09-14 — *"Where `rk-ports` will first be USED … **Delete that line when its call
//! site lands**"* — and this slice deletes it.
//!
//! | test | criteria | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac009_vector_is_journaled_once_and_reread`] | AC-009 | the emitted vector is journaled **unnormalised** and every later read resolves from the journal | **never that the vector is stable** — the register forbids reading AC-009 as licensing that |
//! | [`ac010_malformed_key_is_refused_before_spawn`] | AC-010 | the three named malformations, each with a dispatch count of **0** | the `@version`'s **producer**, which no type in this tree has |
//! | [`ac011_attempt_increases_and_the_triple_is_unique`] | AC-011 | strictly increasing `attempt`; the triple unique **across the whole log** | |
//! | [`ac012_late_duplicate_is_discarded`] | AC-012 | two threads, one activation, one commit, one discard | a **process** race — the register says *"driving two attempts"*, and where it means two processes it says so (AC-181 :2941, AC-271 :4166) |
//! | [`ac013_re_entry_reexecutes_nothing`] | AC-013 — **PROVISIONAL** | zero re-dispatch of a committed step; byte-identical committed digests | **BUILT, NOT CLAIMED.** P8 is PARTIAL (3 of 27+3, one recorded step), and the process boundary here is a **dropped `Runtime`** — ADR-0087 clause 2: *"a reload that reads state back in-process is not a valid implementation of this decision and is not evidence for it"*. **ROUTED TO 7d, 2026-09-14 (D-8):** a real re-exec harness now exists in [`crate::cross_process`] and is deliberately **not** pointed here, because AC-013's WHEN is *"WHEN `redkiln run <item>` re-enters"* — the thing re-entering is the CLI |
//! | [`ac030_rate_limited_writes_a_mark_and_never_sleeps`] | AC-030 | the `retry_at(resets_at)` mark, no further dispatch, no clock anywhere in the module | *"exits 0"* and *"the next run resumes from the mark"*, both **ROUTED TO 7d, 2026-09-14 (D-8)**. This cell read *"a **process** claim, 7d's"* of the exit; that reason is now too wide — [`crate::cross_process`] reads a **child's** exit for AC-024. AC-030 is not routed there because its own *Verified by* names *"the next **`redkiln run`** resumes from the mark (AC-013's re-entry test)"*: a binary that is 7d's, and a criterion that is PROVISIONAL |
//! | [`ac031_schema_reprompts_exactly_once`] | AC-031 — **compound** | exactly two dispatches and `Schema { attempts: 2 }`; **and** the prose-extraction grep-lint with its must-fail fixture | AC-031 is **not claimed whole**: the register's own residual-risk list says *"the prose-extraction ban will be under-tracked until the range widens"* |
//! | [`runtime_step_appends_only_what_the_action_names`] | — | that three of the six `NextAction` arms append **nothing at all** | |
//! | [`the_fsm_dispatch_key_has_no_version_and_the_boundary_refuses_it`] | — | 🛑 **a finding, pinned**: the DAG node carries a bare `activity_id`, so the FSM's own `Dispatch` key is refused by AC-010 and `next_action` → `step` cannot complete end to end on this tree | it does not **fix** it: the join has three candidate homes and choosing one is a decision `00-preamble.md`:55 reserves |
//!
//! 🛑 **The extra names are authorised and the reason is worth stating.** Residual 10a is FORCED —
//! *"the ten [runbook] names are a floor, not a ceiling"* — and `00-preamble.md`:188-189 requires a
//! discriminating test for every criterion in the acceptance set. Seven of the eight ids here are
//! reached by **nothing** in Checkpoint 7b except the unfiltered `cargo test -p rk-kernel` floor,
//! which `s7b-kernel.md`:992-996 says *"is satisfied by an implementation that nothing pins"*. None
//! of the eight names contains the substring `golden_log_replay` or `ac161_ac164_delete_db_replay`,
//! which the `--list` differential at :1345-1346 counts **without** a word anchor.
//!
//! 🛑 **The two source lints were MEASURED to bite, not assumed to** [2026-09-14], on
//! `tests/it/ui.rs`'s own precedent for AC-025's compile-fail case. Adding one
//! `serde_json::from_str` call to `src/runtime/activity.rs` turns
//! [`ac031_schema_reprompts_exactly_once`] red at *"the boundary module itself carries no path from
//! free text to structure"*; adding one `std::thread::sleep` to `src/runtime/marks.rs` turns
//! [`ac030_rate_limited_writes_a_mark_and_never_sleeps`] red at *"marks.rs carries `sleep` outside a
//! comment"*. Both mutations were reverted immediately after the observation, so each lint is known
//! to fail rather than merely known to be present — which is what separates an absence assertion
//! from a vacuous one.
//!
//! 🛑 **Why the re-entry assertions COUNT a fake rather than panicking inside one.** AC-008's and
//! AC-009's instruments both name *"a fake that fails the test on any `dispatch` call"*. A panic
//! inside `Harness::dispatch` happens **while the store lock is held** — `dispatch` takes
//! `&mut dyn EventStore` and `rk_kernel::runtime::Runtime` holds the guard across the call — and
//! ADR-0118 clauses 1 and 4 make a poisoned store lock a **halt**, not a test failure, with clause
//! 3's forbids barring recovery. So the zero is read off [`CountingHarness::dispatches`] on the
//! joining thread instead. The two shapes are equally strong and only one is lawful under ADR-0118.

use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex};

use rk_kernel::runtime::activity::{
    BOUNDARY_ANCHORS, BoundaryLintViolation, CommitOutcome, PROSE_EXTRACTION_FORMS,
    boundary_prose_extraction_rule, committed_result,
};
use rk_kernel::runtime::marks::{RetryAtMark, read_retry_at_marks};
use rk_kernel::runtime::{Runtime, RuntimeError, StepOutcome};
use rk_kernel::select::{Digest, digest_of};
use rk_kernel::workflow::breaker::JournaledNow;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_kernel::workflow::fsm::{GateRecord, HaltReason, NextAction, fold};
use rk_kernel::workflow::idem::{KeyError, checked_key};
use rk_ports::docs_site::DocsSite;
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_ports::fakes::{BoardTracker, FixtureToolchain, RecordingDocs};
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_ports::root::Ports;
use rk_ports::toolchain::Toolchain;
use rk_ports::tracker::Tracker;
use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::retention::Retained;
use rk_types::time::Timestamp;
use rk_types::typed::{SchemaId, Typed};
use rk_types::usage::{CliVersion, Timings};

pub const ITEM: &str = "INC-1";
pub const STAGE: &str = "implement";
pub const AID: &str = "impl-report@1";
pub const STEP: &str = "impl-report[node]";
pub const SCHEMA: &str = "ImplReport/v1";

/// The pack's `N`. **Supplied, never defaulted** — `AttemptCap::declared` is its only constructor
/// and it implements no `Default` (AC-029; Slice 4b's `ac029_n_is_declared_never_defaulted` is what
/// asserts the pack-validation half).
pub const CAP: u32 = 4;

/// A journaled instant. `rk_types::time::Timestamp` has no `now()`, deliberately (AC-001).
#[must_use]
pub const fn at(secs: i64) -> JournaledNow {
    JournaledNow::from_journal(Timestamp::from_unix_seconds(secs))
}

// ---------------------------------------------------------------------------------------------
// The fakes. `rk-ports` ships five and neither of the two this file drives counts, so the counters
// are declared here on `ReplayPorts.dispatches`' own model: the counter is the CALLER's and the
// fake moves it, so every count below is a READING and never a literal the code under test printed
// about itself (`s7b`:1018-1023).
// ---------------------------------------------------------------------------------------------

/// A scripted responder that **counts**, and that appends nothing.
///
/// `rk_ports::fakes::FakeHarness` appends a `step_failed` of its own on every `Err`, which is a real
/// adapter journaling as it goes and is correct there — but it would put rows into the journal this
/// file reads for AC-011's whole-log uniqueness and AC-030's mark, and a count over a journal the
/// fake also writes measures two things at once.
pub struct CountingHarness {
    script: Mutex<Vec<Result<Typed, StepError>>>,
    dispatches: AtomicU32,
}

impl CountingHarness {
    #[must_use]
    pub const fn scripted(script: Vec<Result<Typed, StepError>>) -> Self {
        Self {
            script: Mutex::new(script),
            dispatches: AtomicU32::new(0),
        }
    }

    /// How many times the port was reached. **AC-010's *"before any process is spawned"* and
    /// AC-031's *"exactly two dispatches"* are both read off this.**
    #[must_use]
    pub fn dispatches(&self) -> u32 {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl Harness for CountingHarness {
    fn dispatch(&self, _spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        let next = {
            let mut script = self.script.lock().expect("the script lock is not poisoned");
            if script.is_empty() {
                None
            } else {
                Some(script.remove(0))
            }
        };
        DispatchOutcome {
            record: DispatchRecord {
                argv_digest: digest_of(b"argv"),
                env_digest: digest_of(b"env"),
                instrument_version: "counting-fake".to_owned(),
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
                cli_version: CliVersion::Pinned("counting-fake".to_owned()),
                api_retry_events: 0,
                prompt_cache_ttl: "1h".to_owned(),
                subagent_prompt_cache_ttl: "1h".to_owned(),
                cancelled_by: None,
                detected_by: None,
            },
            result: next.unwrap_or_else(|| {
                Err(StepError::Transport {
                    kind: TransportKind::Spawn,
                    detail: "script exhausted".to_owned(),
                })
            }),
        }
    }
}

/// An in-memory store that **counts its reads and its writes separately**.
///
/// The read counter is AC-525 amendment clause 4's instrument, used in `it/advisory.rs`: *"a replay
/// over a run that emitted an advisory must re-derive the same DAG with **no store read**"*, and an
/// implementation that de-duplicated by scanning the log would move it.
pub struct CountingStore {
    events: Vec<Event>,
    queries: Arc<AtomicUsize>,
    appends: Arc<AtomicUsize>,
}

impl CountingStore {
    #[must_use]
    pub fn new(queries: &Arc<AtomicUsize>, appends: &Arc<AtomicUsize>) -> Self {
        Self {
            events: Vec::new(),
            queries: Arc::clone(queries),
            appends: Arc::clone(appends),
        }
    }
}

impl EventStore for CountingStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        self.appends.fetch_add(1, Ordering::SeqCst);
        self.events.push(event);
        Ok(())
    }

    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        self.queries.fetch_add(1, Ordering::SeqCst);
        Ok(self.events.clone())
    }

    /// **Refused rather than faked.** `Snapshot`'s contents are decided by no record
    /// (`crates/rk-ports/src/event_store.rs`:40-47), so an identity snapshot here would be a
    /// round-trip a test could pass without anything being restored. Nothing in this slice calls
    /// it, and if something starts to, this refusal is what says so.
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

/// Every port bound, with the two this file drives named and the three it does not bound to
/// `rk-ports`' own fakes.
///
/// **`Toolchain`, `Tracker` and `DocsSite` are bound and never called**, and that is structural
/// rather than incidental: `rk_ports::root::Ports` holds five, and a `Runtime` cannot be constructed
/// over four.
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

/// One store, shared by however many `Runtime`s a case constructs.
#[must_use]
pub fn store(queries: &Arc<AtomicUsize>, appends: &Arc<AtomicUsize>) -> Arc<Mutex<dyn EventStore>> {
    Arc::new(Mutex::new(CountingStore::new(queries, appends))) as Arc<Mutex<dyn EventStore>>
}

/// A fresh pair of counters.
#[must_use]
pub fn counters() -> (Arc<AtomicUsize>, Arc<AtomicUsize>) {
    (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)))
}

/// The whole journal, read through the port.
///
/// # Panics
///
/// If the store refuses the query, which the fake above never does.
#[must_use]
pub fn journal(db: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    db.lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("the in-memory fake never refuses a query")
}

/// An activity spec whose `schema` is the one the boundary demands.
///
/// # Panics
///
/// If the constant digest stops being 64 lowercase hex characters.
#[must_use]
pub fn spec() -> ActivitySpec {
    ActivitySpec {
        card_digest: Digest::from_hex(&"cd".repeat(32)).expect("64 lowercase hex characters"),
        tools: vec!["Read".to_owned()],
        schema: SchemaId::new(SCHEMA),
        timeout_secs: 900,
        inputs: vec![("item".to_owned(), serde_json::json!(ITEM))],
    }
}

/// A well-formed result carrying the schema the spec demands.
pub fn ok_typed(value: serde_json::Value) -> Result<Typed, StepError> {
    Ok(Typed::new(SchemaId::new(SCHEMA), value))
}

/// How many events of one kind the journal holds.
#[must_use]
pub fn count_kind(events: &[Event], kind: EventKind) -> usize {
    events.iter().filter(|e| e.kind == kind).count()
}

/// The `(item, activity_id@version, attempt)` triple off every `dispatch` row, in append order.
#[must_use]
pub fn dispatch_triples(events: &[Event]) -> Vec<(String, String, u64)> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::Dispatch)
        .filter_map(|e| {
            Some((
                e.payload.get("item")?.as_str()?.to_owned(),
                e.payload
                    .get("activity_id_at_version")?
                    .as_str()?
                    .to_owned(),
                e.payload.get("attempt")?.as_u64()?,
            ))
        })
        .collect()
}

/// The boundary module's own source — the subject of AC-030's no-clock lint and AC-031's
/// prose-extraction lint.
pub const BOUNDARY_SRC: &str = include_str!("../../src/runtime/activity.rs");
/// The mark writer's source, the second half of AC-030's no-clock subject.
pub const MARKS_SRC: &str = include_str!("../../src/runtime/marks.rs");
/// **AC-031's committed must-fail fixture.** A `.txt` and not a `.rs`, so it is a source the lint
/// READS and never a file the compiler builds — `tests/ui/` is the `trybuild` glob root and holds
/// compile-fail cases only (§7b.4:433-436).
pub const MUST_FAIL_BOUNDARY: &str =
    include_str!("../fixtures/boundary/extracts-structure-from-prose.txt");

// ---------------------------------------------------------------------------------------------
// AC-009
// ---------------------------------------------------------------------------------------------

/// **AC-009.** *"THEN it is recorded **verbatim** in the activity result that produced it, and every
/// later read — replay, resume or a second stage — **resolves from that journal entry rather than a
/// new elicitation**."*
///
/// 🛑 **What this test may not assert, and the prohibition is inside the criterion:** *"it does not
/// claim the vector is stable and **must not be read as licensing selection on an unstable
/// vector**."* Nothing below compares two elicitations, and nothing below asserts that a second one
/// would agree.
///
/// **THE PREMISE that makes *verbatim* falsifiable.** The vector's keys are deliberately **not** in
/// sorted order. A runtime that canonicalised the value on its way into the journal would still
/// satisfy a canon-to-canon comparison — both sides equal by construction — so the comparison below
/// is over the **serialised** form, which `serde_json`'s workspace-wide `preserve_order` keeps in
/// insertion order. The premise asserts the two forms actually differ.
#[test]
fn ac009_vector_is_journaled_once_and_reread() {
    let vector = serde_json::json!({
        "zeta": "z",
        "alpha": {"nested_b": 2, "nested_a": 1},
        "mid": [3, 1, 2],
    });
    let raw = serde_json::to_string(&vector).expect("the vector serialises");
    let canon = String::from_utf8(rk_store::canon::canonical(&vector).expect("no floats"))
        .expect("canonical output is UTF-8");
    assert_ne!(
        raw, canon,
        "PREMISE: the emitted vector's key order differs from its canonical order, so a runtime \
         that normalised on the way in is detectable. Without this the comparison below is \
         satisfied by every implementation"
    );

    let (q, a) = counters();
    let db = store(&q, &a);
    let h1 = Arc::new(CountingHarness::scripted(vec![ok_typed(vector)]));
    let rt1 = Runtime::new(
        ports(Arc::clone(&h1) as Arc<dyn Harness>, &db),
        "run-a",
        STAGE,
    );

    let report = rt1
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(100))
        .expect("the run resolves");
    assert!(report.committed(), "the first run commits the vector");

    let recorded = journal(&db);
    let stored = committed_result(&recorded, ITEM, AID).expect("the activation committed");
    assert_eq!(
        serde_json::to_string(stored).expect("the recorded vector serialises"),
        raw,
        "AC-009: the vector is recorded VERBATIM — same bytes, same key order, no normalisation"
    );
    assert_eq!(
        count_kind(&recorded, EventKind::ActivityCommitted),
        1,
        "journaled ONCE"
    );

    // ---- "every later read resolves from that journal entry rather than a new elicitation" ----
    // A SECOND `Runtime` over the SAME store, with its own harness and an EMPTY script: a
    // re-elicitation moves its counter and the joining assertion fails.
    drop(rt1);
    let h2 = Arc::new(CountingHarness::scripted(vec![]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&h2) as Arc<dyn Harness>, &db),
        "run-b",
        STAGE,
    );
    let again = rt2
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(200))
        .expect("the re-entry resolves");
    assert!(
        again.skipped(),
        "the activation is committed, so the second run dispatches nothing"
    );
    assert_eq!(
        h2.dispatches(),
        0,
        "AC-009: ZERO dispatches of the classifier activity on re-entry"
    );
    assert_eq!(
        h1.dispatches(),
        1,
        "and the first run's single dispatch is where the vector came from"
    );
    let after = journal(&db);
    assert_eq!(
        serde_json::to_string(committed_result(&after, ITEM, AID).expect("still committed"))
            .expect("serialises"),
        raw,
        "the later read resolves from the journal entry, byte for byte"
    );

    // ---- THE CONTROL: not everything resolves from the journal -------------------------------
    // An activation that never committed is NOT resolvable and IS elicited. Without this, "zero
    // dispatches on re-entry" is satisfied by a runtime that never dispatches anything.
    let other = "changeset@1";
    assert!(
        committed_result(&after, ITEM, other).is_none(),
        "CONTROL: an uncommitted activation resolves to nothing"
    );
    let h3 = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"k": "v"}),
    )]));
    let rt3 = Runtime::new(
        ports(Arc::clone(&h3) as Arc<dyn Harness>, &db),
        "run-c",
        STAGE,
    );
    let fresh = rt3
        .run_activity(
            ITEM,
            other,
            "changeset[node]",
            &spec(),
            AttemptCap::declared(CAP),
            at(300),
        )
        .expect("the fresh activation resolves");
    assert!(fresh.committed());
    assert_eq!(
        h3.dispatches(),
        1,
        "CONTROL: an uncommitted activation IS elicited, so the zero above is a property of the \
         journal and not of a runtime that never dispatches"
    );
}

// ---------------------------------------------------------------------------------------------
// AC-010
// ---------------------------------------------------------------------------------------------

/// **AC-010.** *"THEN it cannot be constructed without an `(item, activity_id@version, attempt)`
/// idempotency key, and a malformed key (empty `item`, empty `activity_id`, missing `@version`) is
/// **refused before any process is spawned**."*
///
/// The three named malformations, each asserted twice: once against the validator, and once against
/// a live `Runtime` whose `Harness` counter must stay at **0**. The second half is what makes
/// *"before any process is spawned"* a measurement rather than a claim about statement order inside
/// a function body.
///
/// **THE CONTROL** is the last block: a well-formed key over the same runtime **does** reach the
/// port. Without it every assertion here is satisfied by a runtime that never dispatches at all.
#[test]
fn ac010_malformed_key_is_refused_before_spawn() {
    // The criterion's own three, in its own order.
    let cases: [(&str, &str, KeyError, &str); 3] = [
        ("", AID, KeyError::EmptyItem, "empty_item"),
        (ITEM, "", KeyError::EmptyActivityId, "empty_activity_id"),
        (
            ITEM,
            "impl-report",
            KeyError::MissingAtVersion {
                got: "impl-report".to_owned(),
            },
            "missing_at_version",
        ),
    ];
    assert_eq!(
        cases.len(),
        3,
        "PREMISE: the criterion enumerates exactly three malformations and this table is those three"
    );

    for (item, aid, expected, tag) in cases {
        assert_eq!(
            checked_key(item, aid, 1),
            Err(expected.clone()),
            "the validator refuses ({item:?}, {aid:?})"
        );
        assert_eq!(expected.as_str(), tag, "the stable tag, never message text");

        // The same refusal, through a live boundary, with the port counter as the instrument.
        let (q, a) = counters();
        let db = store(&q, &a);
        let h = Arc::new(CountingHarness::scripted(vec![ok_typed(
            serde_json::json!({"never": "reached"}),
        )]));
        let rt = Runtime::new(
            ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
            "run-refused",
            STAGE,
        );
        let got = rt.dispatch_once(item, aid, STEP, 1, &spec());
        assert_eq!(
            got.err().map(|e| e.as_str()),
            Some("key"),
            "the boundary refuses ({item:?}, {aid:?}) as a key error"
        );
        assert_eq!(
            h.dispatches(),
            0,
            "AC-010: refused BEFORE any process is spawned — the Harness port was never reached \
             for ({item:?}, {aid:?})"
        );
        assert_eq!(
            a.load(Ordering::SeqCst),
            0,
            "and nothing was appended either: a `dispatch` row for a key that was refused would \
             put an activation into the journal that never happened"
        );
    }

    // Three further shapes the criterion's third case covers and a naive `contains('@')` does not.
    assert_eq!(
        checked_key(ITEM, "impl-report@", 1).unwrap_err().as_str(),
        "missing_at_version",
        "an `@` with nothing after it is not a version"
    );
    assert_eq!(
        checked_key(ITEM, "@1", 1).unwrap_err().as_str(),
        "empty_activity_id",
        "an `@` with nothing before it is not an activity id — and it is the ID that is empty, \
         which is a different refusal from the version's"
    );
    assert_eq!(
        checked_key(ITEM, "a@b@c", 1).unwrap_err().as_str(),
        "missing_at_version",
        "two `@`s have no unambiguous split, and guessing one would put two activities under one \
         key (the module header states this as a reading)"
    );

    // ---- THE CONTROL --------------------------------------------------------------------------
    let (q, a) = counters();
    let db = store(&q, &a);
    let h = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-ok",
        STAGE,
    );
    let ok = rt
        .dispatch_once(ITEM, AID, STEP, 1, &spec())
        .expect("a well-formed key dispatches");
    assert!(ok.dispatched());
    assert_eq!(
        h.dispatches(),
        1,
        "CONTROL: the same runtime DOES reach the port for a well-formed key, so the three zeros \
         above are a property of the key and not of the runtime"
    );
    assert_eq!(ok.key().item, ITEM);
    assert_eq!(ok.key().activity_id_at_version, AID);
    assert_eq!(ok.key().attempt, 1);

    // ---- AC-010's compile-time half, asserted BY PATH -----------------------------------------
    // The case and its committed `.stderr` are what pin "cannot be CONSTRUCTED without the key".
    // §7b.14's by-path loop (:1580-1589) enumerates six files and this is not among them, so
    // nothing in Checkpoint 7b catches its deletion — which is gap G-2, extended again. This
    // assertion catches a deletion; it does not catch `.stderr` drift.
    let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    assert!(
        ui.join("ac010_dispatch_without_key.rs").is_file(),
        "AC-010's trybuild case is absent"
    );
    assert!(
        ui.join("ac010_dispatch_without_key.stderr")
            .metadata()
            .is_ok_and(|m| m.len() > 0),
        "AC-010's case commits no .stderr, so it would be green whenever compilation failed for \
         ANY reason (s7a §7a.8)"
    );
}

// ---------------------------------------------------------------------------------------------
// AC-011
// ---------------------------------------------------------------------------------------------

/// **AC-011.** *"THEN the new dispatch carries `attempt + 1` and never reuses a previous attempt
/// number."* *Verified by:* *"a unit test over the retry loop asserting strictly increasing
/// `attempt`; an event-log assertion that `(item, activity_id@version, attempt)` is **unique across
/// the whole log**."*
///
/// Two instruments and both are here. The second is the one with no other site: it is over the
/// **whole log**, which means more than one activation has to be in it — so the second activation
/// below is not decoration, it is what makes *"across the whole log"* mean anything.
///
/// **THE DISCRIMINATOR.** Both activations dispatch under `attempt = 1`, so the bare `attempt`
/// column **does** carry a duplicate and only the triple does not. That is asserted in both
/// directions: a uniqueness check written over `attempt` alone would fail here, and a uniqueness
/// check written over the triple must pass.
#[test]
fn ac011_attempt_increases_and_the_triple_is_unique() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let transport = || StepError::Transport {
        kind: TransportKind::Spawn,
        detail: "connection reset".to_owned(),
    };
    let h = Arc::new(CountingHarness::scripted(vec![
        Err(transport()),
        Err(transport()),
        ok_typed(serde_json::json!({"report": "ok"})),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-retry",
        STAGE,
    );

    let report = rt
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(100))
        .expect("the retry loop resolves");

    assert_eq!(
        report.attempts(),
        &[1, 2, 3],
        "AC-011: the new dispatch carries attempt + 1"
    );
    for pair in report.attempts().windows(2) {
        assert!(
            pair[1] > pair[0],
            "STRICTLY increasing, asserted as an order and not as a set: {:?}",
            report.attempts()
        );
    }
    assert_eq!(report.dispatches(), 3);
    assert_eq!(h.dispatches(), 3, "read off the port, not off the report");
    assert!(report.committed());

    // PREMISE: the two increments are RETRIES — the policy said so — and not a loop counter that
    // would increment whatever happened.
    assert_eq!(
        report
            .decisions()
            .iter()
            .map(rk_kernel::workflow::retry::RetryDecision::as_str)
            .collect::<Vec<_>>(),
        vec!["backoff", "backoff"],
        "PREMISE: both increments follow a Backoff decision, so this is the retry loop AC-011 names"
    );

    // ---- a SECOND activation, so "the whole log" holds more than one -------------------------
    let h2 = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"changeset": []}),
    )]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&h2) as Arc<dyn Harness>, &db),
        "run-retry",
        STAGE,
    );
    let second = rt2
        .run_activity(
            ITEM,
            "changeset@2",
            "changeset[node]",
            &spec(),
            AttemptCap::declared(CAP),
            at(200),
        )
        .expect("the second activation resolves");
    assert_eq!(second.attempts(), &[1]);

    // ---- the whole-log assertion -------------------------------------------------------------
    let whole = journal(&db);
    let triples = dispatch_triples(&whole);
    assert_eq!(
        triples.len(),
        4,
        "PREMISE: four dispatch rows across two activations, or the uniqueness below is over a \
         set too small to collide"
    );
    let mut seen = std::collections::BTreeSet::new();
    for triple in &triples {
        assert!(
            seen.insert(triple.clone()),
            "AC-011: `(item, activity_id@version, attempt)` is unique across the WHOLE log; \
             {triple:?} repeats"
        );
    }
    assert_eq!(seen.len(), 4);

    // THE DISCRIMINATOR, in the other direction.
    let attempts_only: Vec<u64> = triples.iter().map(|(_, _, n)| *n).collect();
    let distinct: std::collections::BTreeSet<u64> = attempts_only.iter().copied().collect();
    assert!(
        distinct.len() < attempts_only.len(),
        "DISCRIMINATOR: the bare `attempt` column DOES repeat (both activations start at 1), so \
         the uniqueness above is a property of the TRIPLE and could not be passed by a check \
         written over `attempt` alone"
    );
}

// ---------------------------------------------------------------------------------------------
// AC-012
// ---------------------------------------------------------------------------------------------

/// **AC-012.** *"GIVEN two attempts of the same `(item, activity_id@version)` step activation where
/// the first has already committed its result / WHEN the second attempt **returns late** / THEN the
/// late result is discarded as a duplicate and the committed value is unchanged."*
/// *Verified by:* *"a **concurrency test** in `rk-kernel` driving two attempts against the fake
/// store; an assertion that **at most one** `activity_committed` event exists **per step
/// activation**."*
///
/// 🛑 **Every assertion is made on the joining thread, and that is ADR-0118's doing.** Clause 1: *"a
/// poisoned store lock **halts the run**"*; clause 4 makes it govern both mutexes; clause 3's
/// forbids bar recovering from it. A failed `assert!` inside a worker is a panic, and a panic while
/// holding the store lock would produce a halt rather than a test failure. So the workers return
/// values and the joining thread asserts over them.
///
/// **The barrier is what makes this a LATE ARRIVAL rather than a pre-flight skip.** Both threads
/// dispatch before either commits; without it the scheduler may run one thread to completion first,
/// the second finds the activation already committed at its pre-flight check, and the case
/// degenerates into AC-013's re-entry path — which is a different criterion and does not discharge
/// this one.
#[test]
fn ac012_late_duplicate_is_discarded() {
    let (q, a) = counters();
    let db = store(&q, &a);
    // Two DISTINGUISHABLE results, so "the committed value is unchanged" names which one.
    let h = Arc::new(CountingHarness::scripted(vec![
        ok_typed(serde_json::json!({"who": "first"})),
        ok_typed(serde_json::json!({"who": "second"})),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-race",
        STAGE,
    );
    let barrier = Barrier::new(2);

    let mut outcomes: Vec<(u32, bool, Option<serde_json::Value>, CommitOutcome)> =
        std::thread::scope(|scope| {
            let handles: Vec<_> = [1_u32, 2_u32]
                .into_iter()
                .map(|attempt| {
                    let rt = &rt;
                    let barrier = &barrier;
                    scope.spawn(move || {
                        let attempt_out = rt
                            .dispatch_once(ITEM, AID, STEP, attempt, &spec())
                            .expect("the dispatch resolves");
                        let dispatched = attempt_out.dispatched();
                        let typed = match attempt_out.result() {
                            Some(Ok(t)) => t.clone(),
                            other => panic!("the script serves two Ok values, got {other:?}"),
                        };
                        // BOTH threads are now past their pre-flight and both have a result in
                        // hand. Whichever commits second is the LATE arrival.
                        barrier.wait();
                        let commit = rt
                            .commit(attempt_out.key(), &typed, at(400))
                            .expect("the commit resolves");
                        (attempt, dispatched, Some(typed.value), commit)
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().expect("no worker panicked"))
                .collect()
        });
    outcomes.sort_by_key(|(attempt, ..)| *attempt);

    // PREMISE: both threads reached the port. Without it the "late" arrival could be a thread that
    // never dispatched, which is AC-013's path and not this one.
    assert_eq!(h.dispatches(), 2, "PREMISE: both attempts dispatched");
    for (attempt, dispatched, ..) in &outcomes {
        assert!(dispatched, "PREMISE: attempt {attempt} reached the port");
    }

    let committed: Vec<_> = outcomes.iter().filter(|(.., c)| c.is_committed()).collect();
    let discarded: Vec<_> = outcomes
        .iter()
        .filter(|(.., c)| !c.is_committed())
        .collect();
    assert_eq!(committed.len(), 1, "exactly one attempt committed");
    assert_eq!(
        discarded.len(),
        1,
        "AC-012: the other was discarded as a late duplicate"
    );

    let whole = journal(&db);
    assert_eq!(
        count_kind(&whole, EventKind::ActivityCommitted),
        1,
        "AC-012: AT MOST ONE `activity_committed` per step activation (ADR-0097 decision 2)"
    );

    let (winner_attempt, _, winner_value, _) = committed[0];
    let stored = committed_result(&whole, ITEM, AID).expect("the activation committed");
    assert_eq!(
        Some(stored),
        winner_value.as_ref(),
        "the committed value is the winner's, unchanged by the late arrival"
    );
    match &discarded[0].3 {
        CommitOutcome::DiscardedLate {
            committed_attempt, ..
        } => assert_eq!(
            committed_attempt, winner_attempt,
            "the discard names the attempt it deferred to"
        ),
        other => panic!("expected a DiscardedLate, got {other:?}"),
    }
    assert_eq!(discarded[0].3.as_str(), "discarded_late");

    // ---- THE CONTROL: a THIRD late arrival is discarded too, and the value still does not move --
    // Without it, "one commit and one discard" is satisfied by a boundary that happens to admit
    // exactly two calls.
    let third = rt
        .commit(
            &checked_key(ITEM, AID, 3).expect("well-formed"),
            &Typed::new(SchemaId::new(SCHEMA), serde_json::json!({"who": "third"})),
            at(500),
        )
        .expect("the third commit resolves");
    assert!(!third.is_committed(), "CONTROL: the third is discarded too");
    let after = journal(&db);
    assert_eq!(count_kind(&after, EventKind::ActivityCommitted), 1);
    assert_eq!(
        committed_result(&after, ITEM, AID),
        Some(stored),
        "CONTROL: and the committed value is still byte-identical"
    );
}

// ---------------------------------------------------------------------------------------------
// AC-013 — PROVISIONAL, built and NOT claimed
// ---------------------------------------------------------------------------------------------

/// **AC-013 — PROVISIONAL. BUILT, NOT CLAIMED.**
///
/// *"GIVEN a run whose graph committed N steps and whose process was killed mid-graph / WHEN
/// `redkiln run <item>` re-enters (there is no `--resume` flag) / THEN no committed step re-executes
/// and every committed digest is byte-identical to its pre-kill value."*
///
/// 🛑 **TWO separate reasons this may not be counted, and the register names only the first.**
///
/// 1. The register's own label: *"**PROVISIONAL:** P8 is PARTIAL — 3 of a pre-registered 27+3 runs,
///    and its recorded state held **exactly one step**."* Register §0: *"A PROVISIONAL criterion may
///    be implemented, but it must not be counted as covered."* §7b.15 conforms: *"the
///    kill-and-resume test is **built, not claimed**."*
/// 2. **The process boundary here is a dropped `Runtime`, not a `taskkill`** — and ADR-0087 clause 2
///    says in terms that *"a reload that reads state back in-process is not a valid implementation
///    of this decision and is not evidence for it."* AC-013's WHEN names `redkiln run`, which is
///    `rk-cli`'s and Step **7d**'s. The ARMED row's *missing at 7b* column carries only reason 1,
///    and that is a finding this slice reports rather than repairs.
///
///    🛑 **2026-09-14: THE HARNESS THIS WOULD NEED NOW EXISTS, AND AC-013 IS STILL NOT CLAIMED.**
///    Operator decision **D-8** built a re-exec harness in [`crate::cross_process`] —
///    `std::env::current_exe()`, an env marker, a child that writes and a parent that asserts what
///    the child left behind — and pointed it at **AC-035** and **AC-024**, whose *Verified by*
///    lines name no binary. **D-8 routes AC-013 to 7d** on the clause above: the thing that
///    re-enters is `redkiln run <item>`. Re-pointing the harness at this test would produce a green
///    named for a CLI re-entry no binary in this tree performs, and would leave reason 1 —
///    PROVISIONAL on P8 — untouched in any case. This test is unchanged.
///
/// What IS measured: the re-entry predicate itself — a second `Runtime` over the same store
/// re-dispatches nothing that committed, and every committed digest is unchanged.
#[test]
fn ac013_re_entry_reexecutes_nothing() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let graph = [
        ("impl-report@1", "impl-report[node]"),
        ("changeset@2", "changeset[node]"),
        ("spec-authoring@1", "spec-authoring[node]"),
    ];
    assert_eq!(
        graph.len(),
        3,
        "PREMISE: N >= 3, as the criterion's own fixture requires"
    );

    // ---- the first process lifetime: two of the three commit, then the Runtime is DROPPED -----
    let h1 = Arc::new(CountingHarness::scripted(vec![
        ok_typed(serde_json::json!({"step": 1})),
        ok_typed(serde_json::json!({"step": 2})),
    ]));
    let rt1 = Runtime::new(
        ports(Arc::clone(&h1) as Arc<dyn Harness>, &db),
        "run-pre-kill",
        STAGE,
    );
    for (aid, step) in &graph[..2] {
        let r = rt1
            .run_activity(ITEM, aid, step, &spec(), AttemptCap::declared(CAP), at(100))
            .expect("the pre-kill run resolves");
        assert!(r.committed(), "{aid} committed before the kill");
    }
    let before = journal(&db);
    assert_eq!(
        count_kind(&before, EventKind::ActivityCommitted),
        2,
        "PREMISE: the graph committed 2 of its 3 steps before the boundary"
    );
    let pre_kill: Vec<String> = graph[..2]
        .iter()
        .map(|(aid, _)| {
            let v = committed_result(&before, ITEM, aid).expect("committed");
            rk_store::canon::canonical_digest(v)
                .expect("no floats")
                .as_str()
                .to_owned()
        })
        .collect();
    assert_eq!(h1.dispatches(), 2);
    drop(rt1);

    // ---- the second lifetime, over the SAME store ---------------------------------------------
    let h2 = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"step": 3}),
    )]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&h2) as Arc<dyn Harness>, &db),
        "run-re-entry",
        STAGE,
    );
    let mut reports = Vec::new();
    for (aid, step) in &graph {
        reports.push(
            rt2.run_activity(ITEM, aid, step, &spec(), AttemptCap::declared(CAP), at(200))
                .expect("the re-entry resolves"),
        );
    }
    assert!(reports[0].skipped(), "a committed step does not re-execute");
    assert!(reports[1].skipped(), "a committed step does not re-execute");
    assert!(reports[2].committed(), "the uncommitted step runs");
    assert_eq!(
        h2.dispatches(),
        1,
        "AC-013: ZERO re-dispatch of a committed step — the only dispatch is the one that had not \
         committed"
    );

    let after = journal(&db);
    let post: Vec<String> = graph[..2]
        .iter()
        .map(|(aid, _)| {
            let v = committed_result(&after, ITEM, aid).expect("still committed");
            rk_store::canon::canonical_digest(v)
                .expect("no floats")
                .as_str()
                .to_owned()
        })
        .collect();
    assert_eq!(
        post, pre_kill,
        "AC-013: every committed digest is byte-identical to its pre-boundary value"
    );
    assert_eq!(
        count_kind(&after, EventKind::ActivityCommitted),
        3,
        "and exactly one new commit joined them"
    );

    // ---- THE CONTROL --------------------------------------------------------------------------
    // A fourth activation the graph never ran is NOT skipped. Without it, "the committed steps
    // were skipped" is satisfied by a runtime that skips everything.
    let h3 = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"step": 4}),
    )]));
    let rt3 = Runtime::new(
        ports(Arc::clone(&h3) as Arc<dyn Harness>, &db),
        "run-control",
        STAGE,
    );
    let never = rt3
        .run_activity(
            ITEM,
            "review@1",
            "review[node]",
            &spec(),
            AttemptCap::declared(CAP),
            at(300),
        )
        .expect("the control resolves");
    assert!(!never.skipped());
    assert_eq!(
        h3.dispatches(),
        1,
        "CONTROL: an activation the graph never committed IS dispatched on re-entry"
    );
}

// ---------------------------------------------------------------------------------------------
// AC-030
// ---------------------------------------------------------------------------------------------

/// **AC-030.** *"GIVEN a `RateLimited { resets_at }` failure / WHEN the kernel handles it / THEN it
/// writes a `retry_at(resets_at)` mark and **exits 0**; it **never sleeps** until the reset."*
///
/// 🛑 **Two halves are ARMED, are not asserted here, and are ROUTED TO 7d (D-8, 2026-09-14).**
///
/// This note read *"*\"Exits 0\"* is a **process** claim and `rk-kernel` is a library (residual 10d;
/// `rk-cli` is 7d) — a `cargo test` binary cannot assert its own exit and `std::process::exit`
/// inside one aborts the harness"*. The clause about a test's OWN exit is still true. The clause
/// about the crate is not a reason any more: [`crate::cross_process`] re-execs this binary and
/// reads a **child's** exit code for AC-024. **AC-030 is deliberately not routed there**, and the
/// reason is the criterion's own *Verified by*: *"the next **`redkiln run`** resumes from the mark
/// (AC-013's re-entry test)"* — it names the binary, and AC-013's test is PROVISIONAL and may not
/// be counted. So both halves are 7d's, for the criterion's reason rather than for a limit of this
/// harness. What is discharged here is the **mark** and the **no-sleep**.
#[test]
fn ac030_rate_limited_writes_a_mark_and_never_sleeps() {
    let reset = Timestamp::from_unix_seconds(1_700_000_600);
    let now = at(1_700_000_000);
    let (q, a) = counters();
    let db = store(&q, &a);
    let h = Arc::new(CountingHarness::scripted(vec![
        Err(StepError::RateLimited {
            resets_at: Some(reset),
        }),
        // A second scripted value that must NEVER be served: the policy forbids a re-dispatch.
        ok_typed(serde_json::json!({"must": "not be reached"})),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-limited",
        STAGE,
    );

    let report = rt
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), now)
        .expect("the run resolves");

    assert_eq!(
        report
            .decisions()
            .iter()
            .map(rk_kernel::workflow::retry::RetryDecision::as_str)
            .collect::<Vec<_>>(),
        vec!["wait_for_reset"],
        "ADR-0095 forbids backoff on this class"
    );
    assert_eq!(
        report.dispatches(),
        1,
        "no further dispatch: the run ends on the mark"
    );
    assert_eq!(h.dispatches(), 1, "read off the port");
    assert!(!report.committed(), "nothing committed");

    // ---- the mark ----------------------------------------------------------------------------
    assert_eq!(
        report.mark(),
        Some(&RetryAtMark::carried(Some(reset))),
        "AC-030: a `retry_at(resets_at)` mark, carrying the reset the FAILURE named"
    );
    let whole = journal(&db);
    let marks = read_retry_at_marks(&whole);
    assert_eq!(marks.len(), 1, "exactly one durable mark");
    assert_eq!(marks[0].resets_at(), Some(reset));
    assert_eq!(
        count_kind(&whole, EventKind::RateLimited),
        1,
        "the mark rides `rate_limited` — design §5 row 18, not a reading"
    );

    // ---- "zero elapsed sleep on the injected clock" -------------------------------------------
    assert_eq!(
        report.now(),
        now,
        "the journaled instant the run was handed comes back unchanged"
    );

    // ---- CONTROL 1: a non-rate-limited failure writes NO mark ---------------------------------
    let (q2, a2) = counters();
    let db2 = store(&q2, &a2);
    let h2 = Arc::new(CountingHarness::scripted(vec![
        Err(StepError::Transport {
            kind: TransportKind::Spawn,
            detail: "reset".to_owned(),
        }),
        ok_typed(serde_json::json!({"ok": true})),
    ]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&h2) as Arc<dyn Harness>, &db2),
        "run-transport",
        STAGE,
    );
    let other = rt2
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), now)
        .expect("resolves");
    assert!(other.mark().is_none());
    assert!(
        read_retry_at_marks(&journal(&db2)).is_empty(),
        "CONTROL: a `Transport` failure writes no `retry_at` mark, so the mark above is a property \
         of the CLASS and not of every failure"
    );
    assert_eq!(other.decisions()[0].as_str(), "backoff");

    // ---- CONTROL 2: a bare `rate_limited` row is not a mark -----------------------------------
    // Design §5 gives `rate_limited` to the FAILURE event as well; `kind = "retry_at"` is what
    // tells the two producers apart, and a reader that took every row would over-count.
    let bare = vec![Event::new(
        EventKind::RateLimited,
        serde_json::json!({"status": 429}),
    )];
    assert!(
        read_retry_at_marks(&bare).is_empty(),
        "CONTROL: a `rate_limited` row that is not a mark is skipped, never defaulted into one"
    );

    // ---- the no-clock source lint, which is the instrument the equality above is NOT ----------
    // `report.now() == now` is a restatement of the fact that nothing in this crate can move a
    // `JournaledNow`. What would catch a real sleep is the absence of the call, and an absence
    // needs an anchor or it is vacuous.
    for (name, src) in [("activity.rs", BOUNDARY_SRC), ("marks.rs", MARKS_SRC)] {
        let code: String = src
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            code.contains("pub fn") || code.contains("pub struct"),
            "ANCHOR: {name} is not empty"
        );
        for banned in ["sleep", "SystemTime", "Instant::", "std::time"] {
            assert!(
                !code.contains(banned),
                "AC-030 / AC-003: {name} carries `{banned}` outside a comment — the kernel never \
                 sleeps and never reads a clock"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// AC-031 — compound
// ---------------------------------------------------------------------------------------------

/// **AC-031 — COMPOUND, and both of its instruments are here.**
///
/// *"GIVEN a dispatch whose returned value fails schema validation at the boundary / WHEN the kernel
/// handles it / THEN it re-prompts **exactly once** carrying the validation error, and on a second
/// failure returns `Schema { attempts: 2 }`."* *Verified by:* *"a unit test over a scripted responder
/// returning two invalid values, asserting **exactly two dispatches** and the typed error."*
///
/// *Sub-clause (compound, split blocked — no free id in AC-001..AC-039):* *"the boundary never
/// extracts structure from prose. Verified separately by a **grep-lint over the boundary module**
/// rejecting any markdown/JSON-from-prose extraction path, **with a must-fail fixture**."*
///
/// 🛑 **AC-031 IS NOT CLAIMED WHOLE.** The register's own residual-risk list says *"AC-031 is still
/// compound … the prose-extraction ban will be under-tracked until the range widens"*. Both halves
/// are built; the criterion is reported compound.
#[test]
fn ac031_schema_reprompts_exactly_once() {
    let wrong = || {
        Ok(Typed::new(
            SchemaId::new("SomethingElse/v1"),
            serde_json::json!({"shape": "wrong"}),
        ))
    };
    assert_ne!(
        SchemaId::new("SomethingElse/v1").as_str(),
        SCHEMA,
        "PREMISE: the scripted value's schema is NOT the one the spec demands"
    );

    // ---- two invalid values --------------------------------------------------------------------
    let (q, a) = counters();
    let db = store(&q, &a);
    let h = Arc::new(CountingHarness::scripted(vec![
        wrong(),
        wrong(),
        // A third value that must never be served: exactly ONE re-prompt.
        ok_typed(serde_json::json!({"must": "not be reached"})),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-schema",
        STAGE,
    );
    let report = rt
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(100))
        .expect("the run resolves");

    assert_eq!(
        h.dispatches(),
        2,
        "AC-031: EXACTLY two dispatches — one prompt and exactly one re-prompt"
    );
    assert_eq!(report.dispatches(), 2);
    assert_eq!(report.schema_failures(), 2);
    match report.outcome() {
        Some(Err(StepError::Schema { attempts, .. })) => assert_eq!(
            *attempts, 2,
            "AC-031: the typed error carries `attempts: 2`"
        ),
        other => panic!("expected Err(Schema {{ attempts: 2 }}), got {other:?}"),
    }
    assert_eq!(
        report
            .decisions()
            .iter()
            .map(rk_kernel::workflow::retry::RetryDecision::as_str)
            .collect::<Vec<_>>(),
        vec!["reprompt", "refuse"],
        "one re-prompt, then a refusal — the ceiling is \
         `rk_kernel::workflow::retry::SCHEMA_REPROMPT_LIMIT` and not a second constant here"
    );
    assert!(!report.committed());
    assert_eq!(count_kind(&journal(&db), EventKind::ActivityCommitted), 0);

    // ---- CONTROL 0: AN UNRELATED FAILURE FIRST MUST NOT CONSUME THE RE-PROMPT -----------------
    //
    // ADDED 2026-09-14, and it exists because a mutation-style review found the gap. The policy's
    // `UpToAttempts` arm gated on `key.attempt` — the activation's GLOBAL dispatch counter — while
    // the schema-failure count lives in `StepError::Schema { attempts }`. The two agree only when
    // nothing else has failed first, so AC-031's *"re-prompt exactly once"* silently became
    // *"re-prompt only if the first dispatch was the schema failure"*. Measured before the repair
    // on this exact script: the FIRST schema failure was REFUSED, decisions read
    // `["backoff", "refuse"]`, and `ac031_schema_reprompts_exactly_once` above stayed GREEN
    // throughout, because its script has no unrelated failure in it.
    //
    // A resume whose `next_attempt` is non-zero has the same shape, which is why this is a
    // behaviour test and not a source assertion.
    let (q0, a0) = counters();
    let db0 = store(&q0, &a0);
    let h0 = Arc::new(CountingHarness::scripted(vec![
        Err(StepError::Transport {
            kind: TransportKind::Io,
            detail: "an unrelated class, before any schema failure".to_owned(),
        }),
        wrong(),
        wrong(),
        // Never served: the re-prompt allowance is ONE, counted on schema failures alone.
        ok_typed(serde_json::json!({"must": "not be reached"})),
    ]));
    let rt0 = Runtime::new(
        ports(Arc::clone(&h0) as Arc<dyn Harness>, &db0),
        "run-schema-after-transport",
        STAGE,
    );
    let report0 = rt0
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(100))
        .expect("the run resolves");

    assert_eq!(
        report0.schema_failures(),
        2,
        "PREMISE: both scripted schema failures were reached, so the re-prompt really was exercised \
         after the unrelated failure rather than the run ending early"
    );
    assert_eq!(
        h0.dispatches(),
        3,
        "one transport failure, then the prompt and EXACTLY ONE re-prompt — the unrelated class \
         must not spend AC-031's allowance"
    );
    assert_eq!(
        report0
            .decisions()
            .iter()
            .map(rk_kernel::workflow::retry::RetryDecision::as_str)
            .collect::<Vec<_>>(),
        vec!["backoff", "reprompt", "refuse"],
        "the transport failure is backed off, the FIRST schema failure is re-prompted, the second \
         is refused. Reading `key.attempt` instead of the error's own counter gives \
         [\"backoff\", \"refuse\"] and never re-prompts at all"
    );
    assert!(!report0.committed());

    // ---- CONTROL 1: a valid value on the first call is ONE dispatch ---------------------------
    let (q1, a1) = counters();
    let db1 = store(&q1, &a1);
    let h1 = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt1 = Runtime::new(
        ports(Arc::clone(&h1) as Arc<dyn Harness>, &db1),
        "run-clean",
        STAGE,
    );
    let clean = rt1
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(100))
        .expect("resolves");
    assert_eq!(
        h1.dispatches(),
        1,
        "CONTROL: 'exactly two' is not a constant — a conforming value takes one"
    );
    assert!(clean.committed());
    assert_eq!(clean.schema_failures(), 0);

    // ---- CONTROL 2, the discriminating one: the re-prompt can SUCCEED -------------------------
    // Without it, "re-prompts exactly once" is satisfied by a boundary that always fails twice.
    let (q2, a2) = counters();
    let db2 = store(&q2, &a2);
    let h2 = Arc::new(CountingHarness::scripted(vec![
        wrong(),
        ok_typed(serde_json::json!({"ok": "on the re-prompt"})),
    ]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&h2) as Arc<dyn Harness>, &db2),
        "run-recovers",
        STAGE,
    );
    let recovered = rt2
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(CAP), at(100))
        .expect("resolves");
    assert_eq!(h2.dispatches(), 2);
    assert_eq!(recovered.schema_failures(), 1);
    assert!(
        recovered.committed(),
        "DISCRIMINATOR: the one re-prompt carries the validation error and CAN succeed — a \
         boundary that merely failed twice would fail here"
    );

    // ---- the sub-clause: the prose-extraction lint --------------------------------------------
    assert_eq!(
        boundary_prose_extraction_rule(BOUNDARY_SRC),
        Ok(()),
        "the boundary module itself carries no path from free text to structure"
    );

    // A POSITIVE CONTROL PER FORM. Without one per form, six absences over a file are six
    // assertions nothing has shown can fail.
    let anchors = BOUNDARY_ANCHORS.join("\n");
    for form in PROSE_EXTRACTION_FORMS {
        let synthetic = format!("{anchors}\nlet v = {form});\n");
        let got = boundary_prose_extraction_rule(&synthetic)
            .expect_err("a source carrying a banned form is refused");
        assert_eq!(
            got.len(),
            1,
            "exactly one violation for `{form}`, and it is that form's"
        );
        assert_eq!(got[0].clause(), "prose_extraction");
        match got[0] {
            BoundaryLintViolation::ProseExtraction { form: f, .. } => assert_eq!(f, form),
            other => panic!("expected a ProseExtraction, got {other:?}"),
        }
    }

    // AN EMPTY SOURCE FAILS ON THE ANCHOR, never by passing six absences.
    let empty = boundary_prose_extraction_rule("")
        .expect_err("an emptied boundary module is not a clean boundary module");
    assert_eq!(empty.len(), BOUNDARY_ANCHORS.len());
    assert!(empty.iter().all(|v| v.clause() == "anchor_missing"));

    // THE COMMITTED MUST-FAIL FIXTURE the register names. It is a `.txt` and not a `.rs`, so it is
    // a source the lint READS and never a file the compiler builds -- `tests/ui/` is the
    // compile-fail glob root and holds compile-fail cases only (§7b.4:433-436).
    let fixture = boundary_prose_extraction_rule(MUST_FAIL_BOUNDARY)
        .expect_err("AC-031's must-fail fixture must be refused");
    assert!(
        fixture.iter().all(|v| v.clause() == "prose_extraction"),
        "the fixture carries every anchor, so it fails on the BAN and not on an anchor — \
         otherwise it would be refused for being empty rather than for extracting from prose"
    );
    assert!(
        fixture.len() >= 3,
        "PREMISE: the fixture carries at least three distinct extraction paths, got {}",
        fixture.len()
    );
}

// ---------------------------------------------------------------------------------------------
// The boundary's own shape
// ---------------------------------------------------------------------------------------------

/// Three of `NextAction`'s six arms append **nothing at all**, and each of the three is a record
/// rather than an omission: `AwaitGate` because ADR-0022 says *"the kernel takes no action until it
/// is answered"*, `Halt` because the refusal is already in the journal, and `Done` because there is
/// nothing left to do.
///
/// An extra name, authorised by residual 10a. Without it `Runtime::step` — the one method design §1.1
/// names as *"the one method that turns a `NextAction` into an effect"* — is asserted by nothing.
#[test]
fn runtime_step_appends_only_what_the_action_names() {
    let tree = "aaaa111122223333444455556666777788889999";
    let deadline = Timestamp::from_unix_seconds(1_700_009_999);
    let record = GateRecord::derive("unknown_facet:criticality", tree, deadline)
        .expect("the gate record derives");
    let token = record.token().clone();

    let cases: Vec<(NextAction, usize, &str)> = vec![
        (NextAction::OpenGate(record.clone()), 1, "gate_opened"),
        (
            NextAction::AwaitGate {
                token: token.clone(),
            },
            0,
            "awaiting",
        ),
        (
            NextAction::AdvanceStage {
                from: STAGE.to_owned(),
                to: "verify".to_owned(),
            },
            1,
            "stage_advanced",
        ),
        (
            NextAction::Halt {
                reason: HaltReason::Refused {
                    class: "overage".to_owned(),
                },
            },
            0,
            "halted",
        ),
        (NextAction::Done, 0, "done"),
    ];
    assert_eq!(
        cases.len() + 1,
        NextAction::ALL_TAGS.len(),
        "PREMISE: every one of the six arms is exercised — five here and `Dispatch` below — so a \
         seventh arm is a failure here rather than a forgotten row"
    );

    for (action, expected_appends, tag) in cases {
        let (q, a) = counters();
        let db = store(&q, &a);
        let h = Arc::new(CountingHarness::scripted(vec![]));
        let rt = Runtime::new(
            ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
            "run-step",
            STAGE,
        );
        let before = a.load(Ordering::SeqCst);
        let outcome = rt
            .step(&action, None, AttemptCap::declared(CAP), at(100))
            .expect("the step resolves");
        let moved = a.load(Ordering::SeqCst) - before;
        assert_eq!(outcome.as_str(), tag);
        assert_eq!(
            moved, expected_appends,
            "`{tag}` appends {expected_appends}, measured on the store's own counter"
        );
        assert_eq!(
            outcome.appended(),
            expected_appends,
            "and the outcome's own count agrees with the store's, so the two cannot drift"
        );
        assert_eq!(
            h.dispatches(),
            0,
            "no arm but `Dispatch` reaches the Harness port"
        );
    }

    // ---- `OpenGate`'s row is the RUNTIME's producer, and the fold reads it as one -------------
    let (q, a) = counters();
    let db = store(&q, &a);
    let h = Arc::new(CountingHarness::scripted(vec![]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-gate",
        STAGE,
    );
    rt.step(
        &NextAction::OpenGate(record.clone()),
        None,
        AttemptCap::declared(CAP),
        at(100),
    )
    .expect("the gate opens");
    let state = fold(ITEM, STAGE, &journal(&db)).expect("the fold reads the row it just wrote");
    assert_eq!(
        state.open_gates().collect::<Vec<_>>(),
        vec![&record],
        "the row this boundary writes is the shape `fsm::fold` reads as the RUNTIME's durable mark \
         — the two producers of `gate_opened` cannot drift apart without this going red"
    );
    assert_eq!(state.awaiting_gate(), Some(&token));

    // ---- `Dispatch` without a spec is REFUSED, never defaulted --------------------------------
    let refused = rt.step(
        &NextAction::Dispatch {
            key: checked_key(ITEM, AID, 1).expect("well-formed"),
            step_id: STEP.to_owned(),
        },
        None,
        AttemptCap::declared(CAP),
        at(100),
    );
    assert_eq!(
        refused.err().map(|e| e.as_str()),
        Some("spec_not_supplied"),
        "an empty ActivitySpec would be a dispatch of a card nobody wrote"
    );
    assert_eq!(h.dispatches(), 0);

    // ---- and with one, it runs the activity --------------------------------------------------
    let (q2, a2) = counters();
    let db2 = store(&q2, &a2);
    let h2 = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let rt2 = Runtime::new(
        ports(Arc::clone(&h2) as Arc<dyn Harness>, &db2),
        "run-dispatch",
        STAGE,
    );
    let outcome = rt2
        .step(
            &NextAction::Dispatch {
                key: checked_key(ITEM, AID, 1).expect("well-formed"),
                step_id: STEP.to_owned(),
            },
            Some(&spec()),
            AttemptCap::declared(CAP),
            at(100),
        )
        .expect("the dispatch resolves");
    assert_eq!(outcome.as_str(), "activity");
    assert_eq!(h2.dispatches(), 1);
    match &outcome {
        StepOutcome::Activity(report) => assert!(report.committed()),
        other => panic!("expected an Activity outcome, got {other:?}"),
    }
    // The store counter and the outcome's own count agree here too.
    assert_eq!(outcome.appended(), a2.load(Ordering::SeqCst));
}

/// 🛑 **THE FSM'S OWN `Dispatch` KEY IS REFUSED BY THIS BOUNDARY TODAY, and that is a finding
/// rather than a defect in either half.**
///
/// `crate::workflow::fsm::next_action` builds `NextAction::Dispatch { key }` from the DAG node's
/// `activity_id` (`src/workflow/fsm.rs`: *"the DAG node's `activity_id` verbatim"*), and a DAG node
/// carries the **bare** catalog id. AC-010's third malformation is *"missing `@version`"*, so
/// [`rk_kernel::workflow::idem::checked_key`] refuses it and the dispatch never reaches the port.
///
/// [measured 2026-09-14: `grep -rhoE '^id *= *"[^"]*"' spikes/p9/catalog/*.toml | grep -c '@'` →
/// **0** of **676**; `grep -l "^version " spikes/p9/catalog/*.toml | wc -l` → **102** of 102. The
/// version exists on every shipped activity and is read by `crate::catalog::Activity::id_at_version`
/// as of Slice 4g; **nothing joins it onto the DAG node.**]
///
/// **This slice does not close it, and closing it is not a stage's to take.** The join has three
/// candidate homes — step 5's `DagNode` construction, `next_action`'s node lookup, or the
/// composition root — and choosing one is a decision `00-preamble.md`:55 reserves. Widening
/// `checked_key` to accept a bare id is the one move that is definitely wrong: it would make
/// AC-010's third refusal untrue.
///
/// The assertion below is what stops this being a latent gap: if a later slice makes the join, this
/// test goes **red** and whoever made it has to say so.
#[test]
fn the_fsm_dispatch_key_has_no_version_and_the_boundary_refuses_it() {
    let selection = crate::advisory::planned_selection();
    let (q, a) = counters();
    let db = store(&q, &a);
    let h = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"never": "reached"}),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&h) as Arc<dyn Harness>, &db),
        "run-join",
        STAGE,
    );

    let state = fold(ITEM, STAGE, &journal(&db)).expect("an empty journal folds");
    let ctx = rk_kernel::workflow::fsm::FsmContext::supplied(
        ITEM,
        "aaaa111122223333444455556666777788889999",
        Timestamp::from_unix_seconds(1_700_009_999),
        None,
    );
    let action = rk_kernel::workflow::fsm::next_action(&selection, &state, &ctx)
        .expect("the FSM resolves a next action");

    let NextAction::Dispatch { key, .. } = &action else {
        panic!("PREMISE: the fixture selection's next action is a Dispatch, got {action:?}");
    };
    assert!(
        !key.activity_id_at_version.contains('@'),
        "PREMISE: the DAG node carries a BARE activity id — this is the finding, and if it ever          stops being true this assertion is where a reader is told"
    );

    let refused = rt.step(&action, Some(&spec()), AttemptCap::declared(CAP), at(100));
    assert_eq!(
        refused.err().map(|e| e.as_str()),
        Some("key"),
        "AC-010 refuses the FSM's own key for want of an `@version`, so `next_action` -> `step`          cannot complete end to end on this tree. A record naming where the join lives is OWED"
    );
    assert_eq!(
        h.dispatches(),
        0,
        "and the refusal is before any spawn, which is the one thing about it that is correct"
    );
}

/// A refusal this file needs to name but no case above reaches: the error type's own tags.
#[test]
fn runtime_error_tags_are_stable() {
    assert_eq!(RuntimeError::Key(KeyError::EmptyItem).as_str(), "key");
    assert_eq!(
        RuntimeError::SpecNotSupplied {
            step_id: STEP.to_owned()
        }
        .as_str(),
        "spec_not_supplied"
    );
    assert_eq!(
        RuntimeError::Store {
            op: "append",
            detail: String::new()
        }
        .as_str(),
        "store"
    );
}
