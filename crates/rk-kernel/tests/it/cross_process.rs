//! **A real process boundary** — the re-exec harness, and the two criteria whose instruments name
//! one. Operator decision **D-8**, 2026-09-14.
//!
//! # WHY THIS FILE EXISTS, IN THE RECORD'S OWN WORDS
//!
//! ADR-0087 clause 2: *"Resume must cross a real process boundary. A reload that reads state back
//! in-process is not a valid implementation of this decision and **is not evidence for it**."*
//!
//! That sentence is not a preference for a stronger test. It says an in-process reload is **not
//! evidence**, so the dropped-value forms elsewhere in this binary are not weaker instruments for
//! these two criteria — they are instruments for a *different* proposition, and both of their files
//! already say so in terms (`tests/it/breaker.rs`'s table reads *"the grain here is a dropped
//! value"*).
//!
//! # WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | the register's own instrument | measured here | ARMED, not discharged |
//! | --- | --- | --- | --- |
//! | **AC-035** | *"a **kill-and-re-enter** test asserting the refusal survives the restart"* | a second OS process folds the journal a **dead** process wrote and refuses the same family; the live process's own empty fold proceeds | the **expiry's value** (sized by no corpus figure) and *"`redkiln run` is invoked again"* as a **CLI** re-entry — `rk-cli` is 7d's |
//! | **AC-024** | *"a test asserting **process exit** and a **written mark**"* | a child that reaches its wall-clock budget writes the durable mark and **exits 0**, with a second child one second inside the same budget that exits 0 and writes **nothing** | that the process exiting 0 is **`redkiln run`** — it is a libtest binary, and `rk-cli` is 7d's. AC-024 is a **twin** (owner AC-240 + AC-278), so this half is not double-claimed there. *"No thread sleeps"* stays `exhaustion_ac024_ac025`'s source lint and is not re-claimed here |
//!
//! **Neither criterion's Verified-by names a binary**, and that is why these two are here and
//! AC-013/AC-030 are not. See the routing section below.
//!
//! # 🛑 THE MEDIUM IS A FILE, AND THE REASON IS A MEASUREMENT
//!
//! What crosses the boundary is the **journal's bytes**. Measured 2026-09-14:
//! `grep -rn 'impl EventStore' crates/*/src` returns **one** hit — `rk_ports::fakes::VecStore`,
//! which is an in-memory `Vec` and dies with the process that holds it. **There is no durable
//! `EventStore` implementation in this tree at 7b**; the SQLite adapter is Step 7c's. So this file
//! carries [`FileStore`], a test-owned `EventStore` whose `append` writes one canonical JSON line
//! per event and whose `query` reads them back — the **same wire shape**
//! `crate::replay`'s `events.jsonl` already uses (`{ "kind": <wire_name>, "payload": … }`,
//! `rk_store::canon`, LF-terminated), so no second serialisation convention is invented here.
//!
//! What is therefore **not** claimed: that a *store* survives a process death. What is claimed is
//! that the breaker's refusal is reconstructed, in a process that never saw the failures, **from
//! what the dead process left on disk** — which is AC-035's *"breaker state is never held only in
//! memory"* with the memory in question belonging to a process that no longer exists.
//!
//! # 🛑 A FINDING, RECORDED AND NOT REPAIRED: NO OUT-OF-CRATE STORE CAN SNAPSHOT
//!
//! `rk_ports::event_store::Snapshot` is `pub struct Snapshot(pub(crate) ());` — its only field is
//! `pub(crate)` **to `rk-ports`**. So no `EventStore` implementation outside that crate can
//! construct one, and `snapshot()` is unimplementable for every real adapter, not just for this
//! fixture. [`FileStore`] therefore **refuses** both `snapshot` and `rebuild` with
//! `StepError::Unsupported` rather than faking them. This is not repaired here: editing `rk-ports`
//! would disturb Checkpoint 7a after it passed (`00-preamble.md`:232-237), and the two methods are
//! reached by nothing in this file.
//!
//! # 🛑 AC-013 AND AC-030 ARE ROUTED TO **7d**, AND THE HARNESS IS DELIBERATELY NOT POINTED AT THEM
//!
//! **D-8.** Both criteria's re-entry is the **CLI's**, and the register says so at the point where
//! each names its instrument:
//!
//! * **AC-013** — its WHEN is *"WHEN `redkiln run <item>` re-enters (there is no `--resume`
//!   flag)"*. Measured 2026-09-14 against `references/2026-09-08-acceptance-criteria.md`:395-404:
//!   its *Verified by* does name *"a kill-and-resume integration test in `rk-kernel`"*, and the
//!   clause that routes it is the WHEN — the thing re-entering is `redkiln run`, which is `rk-cli`
//!   and does not exist before Step 7d. It stays **PROVISIONAL on P8** either way (3 of a
//!   pre-registered 27+3 runs, one recorded step), so it is **built, not claimed**, and the
//!   existing `ac013_re_entry_reexecutes_nothing` stays exactly as it is.
//! * **AC-030** — its *Verified by* names it outright: *"the next **`redkiln run`** resumes from
//!   the mark (AC-013's re-entry test)"*, and that half rides on AC-013's, which may not be
//!   counted.
//!
//! Both are reported **ARMED, NOT DISCHARGED** with that reason, in `tests/it/runtime.rs`'s table
//! and in `src/runtime.rs`'s. Pointing this harness at them would produce a green whose name
//! claimed a CLI re-entry no binary in this tree performs.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Output;

use rk_kernel::catalog::Catalog;
use rk_kernel::select::Pack;
use rk_kernel::workflow::breaker::{
    BreakerDecision, BreakerExpiry, BreakerState, FamilyKey, JournaledNow,
};
use rk_kernel::workflow::budget::{
    Attempts, BudgetEnvelope, BudgetExtensionMark, BudgetLedger, Charge, Tokens, WallClockSecs,
    fold, read_budget_extension_marks, wall_clock_expiry_mark,
};
use rk_kernel::workflow::envelope::BreakerThreshold;
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

// ---- the env marker D-8 names, and the two values that ride beside it ---------------------------

/// **The env marker.** A child entrypoint that does not see it REFUSES rather than running, so a
/// stray `cargo test -- --ignored` cannot write a journal nothing is reading.
const MARKER: &str = "RK_D8_CHILD";
/// Where the child writes. An absolute path the parent chose, handed down rather than recomputed,
/// so a child that disagreed with the parent about the path would leave the parent's premise
/// (*"nothing is there yet"*) standing and fail loudly instead of silently reading its own file.
const STORE_ENV: &str = "RK_D8_STORE";
/// Which case the child is running. One child entrypoint per criterion, one case per arm.
const CASE_ENV: &str = "RK_D8_CASE";

/// The pack that declares `K = 3`. The child does not choose `K`; §7b.10 requires it declared in
/// the pack and refused as a code default, and `require_breaker_threshold` is that refusal.
const FIXTURE_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-envelope.toml");

/// The family the child trips.
const OPEN_FAMILY: &str = "family-under-test";
/// The family the child touches **below** `K` and then resets. AC-034's isolation is not this
/// file's subject; this exists so *"the parent refuses"* is distinguishable from *"the parent's
/// fold refuses everything it was handed"*.
const QUIET_FAMILY: &str = "family-that-never-opened";
/// The journaled instant the child's last platform failure carries — so the mark's `opened_at` is a
/// number both processes name rather than a clock either of them read (AC-001).
const OPENED_AT: i64 = 1_000;
/// The declared breaker expiry the parent compares against. Arrives as an argument; this crate
/// names no number (§7b.10, and `tests/it/breaker.rs`'s obligation 5).
const EXPIRY_SECS: u64 = 600;

// ---- the harness -------------------------------------------------------------------------------

/// The directory this file owns under cargo's own per-target temp dir.
///
/// `CARGO_TARGET_TMPDIR` is set by cargo for integration-test targets, so the path is **derived and
/// not read from a clock or a random source** — `std::time::SystemTime::now`, `rand::random` and
/// `uuid::Uuid::new_v4` are all in `clippy.toml`'s denylist and that denylist is workspace-wide,
/// tests included (AC-001, AC-002). Uniqueness comes from the case name, and staleness is handled
/// by deleting before the spawn rather than by making the name unguessable.
fn case_dir(case: &str) -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join("d8").join(case)
}

/// The store path for one case, and **the directory is emptied first**.
///
/// The delete is a premise and not tidiness: a journal left by a previous run would satisfy every
/// readback assertion below without any child ever running.
fn fresh_store(case: &str) -> PathBuf {
    let dir = case_dir(case);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("the case directory is removable");
    }
    std::fs::create_dir_all(&dir).expect("the case directory is creatable");
    let path = dir.join("events.jsonl");
    assert!(
        !path.exists(),
        "PREMISE: the journal must not exist before the child runs, or every assertion about what \
         the child left behind is satisfied by a previous run's file: {}",
        path.display()
    );
    path
}

/// What a child process left behind on its way out.
struct ChildOutcome {
    status: std::process::ExitStatus,
    stdout: String,
    stderr: String,
}

impl ChildOutcome {
    /// **The vacuity guard, and it is two assertions rather than one.**
    ///
    /// libtest exits **0** on a filter that selects nothing, so *"the child succeeded"* is not
    /// evidence the child ran — it is exactly the shape
    /// `crates/rk-store/tests/it/contention.rs`:93-96 records being bitten by. The count is read
    /// off the child's own summary line.
    fn assert_ran_exactly_one_test_and_exited_zero(&self, test_path: &str) {
        assert_eq!(
            self.status.code(),
            Some(0),
            "the child process did not exit 0.\n--- child stdout ---\n{}\n--- child stderr ---\n{}",
            self.stdout,
            self.stderr
        );
        assert!(
            self.stdout.contains("1 passed"),
            "the child exited 0 but did not run `{test_path}` — libtest reports a filter that \
             matched nothing as success.\n--- child stdout ---\n{}\n--- child stderr ---\n{}",
            self.stdout,
            self.stderr
        );
        assert!(
            self.stdout.contains("0 failed"),
            "the child's summary line does not report zero failures.\n--- child stdout ---\n{}",
            self.stdout
        );
    }
}

/// **The re-exec harness.** Run one `#[ignore]`d child entrypoint of *this very binary* as a
/// separate OS process, with the env marker set.
///
/// `std::env::current_exe()` is the whole mechanism: the child is the same compiled test binary, so
/// nothing has to be built, installed or found on `PATH`, and the code the child runs is the code
/// this file was compiled with. The filter is the **full** test path under `--exact`, so a second
/// test whose name contains this one cannot be selected instead.
#[expect(
    clippy::disallowed_types,
    reason = "AC-035's named instrument is `a kill-and-re-enter test` and AC-024's is `a test \
              asserting process exit and a written mark`; ADR-0087 clause 2 says an in-process \
              reload `is not evidence for it`. Both therefore require a child PROCESS. \
              clippy.toml's std::process::Command entry is Checkpoint 6's git ban -- ADR-0077:70's \
              one-wrapper rule, whose write set ADR-0105 clause 5 enumerates -- and neither covers \
              a non-git child. AC-003's process ban is scoped to the WORKFLOW module and this is a \
              test binary. The precedents for this exception are \
              crates/rk-store/tests/it/contention.rs:77-85 (AC-181's two-process test, which \
              re-execs current_exe() exactly this way) and tests/it/adapter_edge.rs:117-133."
)]
fn re_exec(test_path: &str, store: &Path, case: &str) -> ChildOutcome {
    let exe = std::env::current_exe().expect("the test binary's own path");
    let out: Output = std::process::Command::new(&exe)
        .args([test_path, "--exact", "--ignored", "--nocapture"])
        .env(MARKER, "1")
        .env(STORE_ENV, store)
        .env(CASE_ENV, case)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .unwrap_or_else(|e| panic!("could not re-exec {} for {test_path}: {e}", exe.display()));
    ChildOutcome {
        status: out.status,
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The child's half of the marker contract: read the store path, or **refuse**.
///
/// # Panics
///
/// When the marker is absent, which is what stops a bare `cargo test -- --ignored` from running a
/// child entrypoint against a path nobody chose.
fn child_store() -> PathBuf {
    assert_eq!(
        std::env::var(MARKER).ok().as_deref(),
        Some("1"),
        "this is a child entrypoint for the D-8 re-exec harness and the parent sets `{MARKER}`; it \
         is not run directly"
    );
    PathBuf::from(std::env::var(STORE_ENV).expect("the parent sets the store path"))
}

/// The child's case selector.
fn child_case() -> String {
    std::env::var(CASE_ENV).expect("the parent sets the case")
}

// ---- the durable medium ------------------------------------------------------------------------

/// A file-backed [`EventStore`] — **the only durable one in this tree**, and it is a test fixture.
///
/// One canonical JSON object per line, LF-terminated, `{ "kind": <wire_name>, "payload": … }` — the
/// shape `crate::replay`'s `events.jsonl` already writes and reads, so this file invents no second
/// serialisation. `append` opens, writes and `sync_all`s on every call: the child may be killed
/// between two appends and the parent must still read the ones that landed.
struct FileStore {
    path: PathBuf,
}

impl FileStore {
    const fn at(path: PathBuf) -> Self {
        Self { path }
    }

    fn io(detail: &str) -> StepError {
        StepError::Transport {
            kind: TransportKind::Io,
            detail: detail.to_owned(),
        }
    }
}

impl EventStore for FileStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        let line = rk_store::canon::canonical(&serde_json::json!({
            "kind": event.kind.wire_name(),
            "payload": event.payload,
        }))
        .map_err(|e| Self::io(&format!("the canonical serialiser refused an event: {e}")))?;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| Self::io(&format!("open {}: {e}", self.path.display())))?;
        f.write_all(&line)
            .and_then(|()| f.write_all(b"\n"))
            .and_then(|()| f.sync_all())
            .map_err(|e| Self::io(&format!("append to {}: {e}", self.path.display())))
    }

    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        let raw = match std::fs::read_to_string(&self.path) {
            Ok(raw) => raw,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(Self::io(&format!("read {}: {e}", self.path.display()))),
        };
        let mut events = Vec::new();
        for (i, line) in raw.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            let value: serde_json::Value = serde_json::from_str(line)
                .map_err(|e| Self::io(&format!("line {}: {e}", i + 1)))?;
            let wire = value["kind"]
                .as_str()
                .ok_or_else(|| Self::io(&format!("line {}: no kind", i + 1)))?;
            let kind = EventKind::ALL
                .into_iter()
                .find(|k| k.wire_name() == wire)
                .ok_or_else(|| {
                    Self::io(&format!(
                        "line {}: {wire:?} is not a kind in the closed EventKind table",
                        i + 1
                    ))
                })?;
            events.push(Event::new(kind, value["payload"].clone()));
        }
        Ok(events)
    }

    /// **Refused, and the refusal is a finding rather than a shortcut.** `Snapshot`'s only field is
    /// `pub(crate)` to `rk-ports`, so no implementation outside that crate can construct one. See
    /// the module header.
    fn snapshot(&self) -> Result<Snapshot, StepError> {
        Err(StepError::Unsupported {
            capability: "snapshot: rk_ports::event_store::Snapshot has no constructor outside \
                         rk-ports"
                .to_owned(),
        })
    }

    fn rebuild(&mut self, _snapshot: &Snapshot) -> Result<(), StepError> {
        Err(StepError::Unsupported {
            capability: "rebuild: see snapshot".to_owned(),
        })
    }
}

// ---- the rows the child writes -----------------------------------------------------------------

/// A `step_failed { class: platform, family, at }` row — AC-033's GIVEN, one occurrence.
fn platform_failure(family: &str, at: i64) -> Event {
    Event::new(
        EventKind::StepFailed,
        serde_json::json!({ "class": "platform", "family": family, "at": at }),
    )
}

/// An `activity_committed` row — the only success this crate can see (ADR-0022).
fn committed(family: &str) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({ "family": family }),
    )
}

/// The pack's `K`, refused as a code default.
fn declared_k() -> BreakerThreshold {
    Pack::from_toml("runtime-envelope.toml", FIXTURE_ENVELOPE)
        .expect("the runtime-envelope fixture loads")
        .require_breaker_threshold()
        .expect("the envelope pack declares `[breaker] consecutive_platform_failures`")
}

/// A journaled instant. `rk_types::time::Timestamp` has no `now()`, deliberately (AC-001).
const fn at(secs: i64) -> JournaledNow {
    JournaledNow::from_journal(Timestamp::from_unix_seconds(secs))
}

// ---- AC-035 — the kill-and-re-enter test -------------------------------------------------------

/// **The child.** It opens a breaker and then this process ends.
///
/// `#[ignore]`d so no ordinary sweep runs it; the parent invokes it by its full path under
/// `--exact --ignored`.
/// It asserts the breaker is **open in its own process** before it ends, so the parent's refusal is
/// a survival rather than a verdict the parent derived from rows that never opened anything.
#[test]
#[ignore = "child entrypoint for AC-035's kill-and-re-enter test; the parent invokes it by name"]
fn ac035_child_opens_a_breaker() {
    let store = child_store();
    assert_eq!(
        child_case(),
        "open",
        "the parent sets exactly one case here"
    );
    let k = declared_k();
    let mut store = FileStore::at(store);

    // `K` consecutive `Platform` failures for one family, the last of them at `OPENED_AT` — which
    // is the instant the fold records as the mark's `opened_at` (it refreshes on the most recent
    // failure that completes a run of at least `K`).
    let k_i64 = i64::from(k.get());
    for i in 0..k_i64 {
        store
            .append(platform_failure(OPEN_FAMILY, OPENED_AT - (k_i64 - 1 - i)))
            .expect("the child appends");
    }
    // The other family: ONE failure — below `K` — and then a commit that resets it. Two rows, so
    // the parent's count is `K + 2` and a partially written file cannot satisfy it.
    store
        .append(platform_failure(QUIET_FAMILY, OPENED_AT))
        .expect("the child appends");
    store
        .append(committed(QUIET_FAMILY))
        .expect("the child appends");

    // THE CHILD'S OWN VERDICT, before this process ends. Without it, a parent refusal could be
    // read as the parent's fold disagreeing with the child's rather than as a state surviving.
    let written = store
        .query(Query::ByItem("any".to_owned()))
        .expect("read back");
    let state = BreakerState::from_events(&written, k).expect("the child's own journal folds");
    assert!(
        state
            .decide(
                &FamilyKey::supplied(OPEN_FAMILY),
                at(OPENED_AT),
                BreakerExpiry::declared(EXPIRY_SECS)
            )
            .refuses(),
        "the child must leave the breaker OPEN, or the parent asserts the survival of nothing"
    );
    println!("child: opened the breaker on {OPEN_FAMILY} and is about to die");
}

/// **AC-035 across a real process boundary** — the criterion's own *"kill-and-re-enter test"*.
///
/// > **AC-035** — GIVEN an open breaker / WHEN the process dies and `redkiln run` is invoked again
/// > / THEN dispatches to that family are still refused until the mark is read and found expired;
/// > breaker state is never held only in memory.
/// > *Verified by:* a **kill-and-re-enter test** asserting the refusal survives the restart; a lint
/// > that breaker state is constructed only from the store.
///
/// This is the first of those two instruments. The second — and the fold, the counting rule and the
/// expiry boundary — are `breaker_ac033_ac035`'s, and neither test claims the other's half.
///
/// # The obligations, in order
///
/// 1. **The premise**: nothing is on disk, and the *parent's own* fold over an empty journal
///    **proceeds**. Without this, a refusal below is indistinguishable from a type that always
///    refuses.
/// 2. The child runs **as a separate OS process**, writes, and exits; the parent waits for it and
///    asserts it ran exactly one test and exited 0.
/// 3. The parent folds what the dead process left on disk and **refuses** the tripped family.
/// 4. **The isolation control**: the family the child touched below `K` **proceeds**, so the
///    refusal is not *"this fold refuses everything it read"*.
/// 5. **The no-store control**: `BreakerState::default()` still proceeds, in the same process, at
///    the same instant — so the refusal is demonstrably derived from the bytes the child left.
/// 6. **The expiry**: *"until the mark is read and found expired"*, asserted on both sides of a
///    declared boundary, over a mark **this process never opened**.
#[test]
fn ac035_breaker_refusal_survives_a_real_process_death() {
    const CHILD: &str = "cross_process::ac035_child_opens_a_breaker";

    let k = declared_k();
    let open = FamilyKey::supplied(OPEN_FAMILY);
    let quiet = FamilyKey::supplied(QUIET_FAMILY);
    let expiry = BreakerExpiry::declared(EXPIRY_SECS);
    let store_path = fresh_store("ac035");

    // ---- 1. THE PREMISE, in this process, before anything is spawned --------------------------
    let store = FileStore::at(store_path.clone());
    let before = store.query(Query::ByItem("any".to_owned())).expect("read");
    assert!(
        before.is_empty(),
        "PREMISE: the journal is empty before the child runs, got {} rows",
        before.len()
    );
    assert_eq!(
        BreakerState::from_events(&before, k)
            .expect("an empty journal folds")
            .decide(&open, at(OPENED_AT), expiry),
        BreakerDecision::Proceed,
        "PREMISE: this process refuses nothing for this family before the child has run"
    );

    // ---- 2. THE PROCESS BOUNDARY ---------------------------------------------------------------
    let child = re_exec(CHILD, &store_path, "open");
    child.assert_ran_exactly_one_test_and_exited_zero(CHILD);
    // The child is gone: `re_exec` waited on it. Everything below reads bytes, never a value.
    assert!(
        store_path.is_file(),
        "the child exited 0 and left no journal at {}",
        store_path.display()
    );

    // ---- 3. THE REFUSAL, FOLDED IN A PROCESS THAT NEVER SAW THE FAILURES -----------------------
    let recovered = store
        .query(Query::ByRun("any".to_owned()))
        .expect("read back");
    assert_eq!(
        recovered.len(),
        usize::try_from(k.get()).expect("K fits") + 2,
        "the parent must read back exactly what the child wrote: {recovered:?}"
    );
    let state = BreakerState::from_events(&recovered, k).expect("the child's journal folds");
    let decision = state.decide(&open, at(OPENED_AT), expiry);
    assert!(
        decision.refuses(),
        "AC-035: the refusal did not survive the process that opened it — got {decision}"
    );
    let mark = state
        .mark(&open)
        .expect("AC-035: the open mark is rebuilt from the store");
    assert_eq!(mark.consecutive(), k.get());
    assert_eq!(mark.opened_at(), Timestamp::from_unix_seconds(OPENED_AT));

    // ---- 4. THE ISOLATION CONTROL --------------------------------------------------------------
    assert_eq!(
        state.decide(&quiet, at(OPENED_AT), expiry),
        BreakerDecision::Proceed,
        "the family the child touched BELOW K must proceed, or the refusal above is 'this fold \
         refuses every family it read'"
    );

    // ---- 5. THE NO-STORE CONTROL, in this same process, at this same instant -------------------
    assert_eq!(
        BreakerState::default().decide(&open, at(OPENED_AT), expiry),
        BreakerDecision::Proceed,
        "a state with no store proceeds; without this, `decide` returning `Refused` unconditionally \
         would pass every assertion above"
    );

    // ---- 6. "UNTIL THE MARK IS READ AND FOUND EXPIRED" -----------------------------------------
    let expiry_i64 = i64::try_from(EXPIRY_SECS).expect("the declared expiry fits");
    assert!(
        state
            .decide(&open, at(OPENED_AT + expiry_i64 - 1), expiry)
            .refuses(),
        "one second inside the declared expiry the mark still refuses"
    );
    assert!(
        !state
            .decide(&open, at(OPENED_AT + expiry_i64), expiry)
            .refuses(),
        "at the declared expiry the mark is FOUND EXPIRED and the refusal lifts"
    );
}

// ---- AC-024 — process exit and a written mark ---------------------------------------------------

/// The activity whose declared `budget = { attempts_total, tokens, wall_clock_secs }` table is the
/// envelope. **No number here is this file's**: the wall clock is the fixture's and the tokens are
/// the pack's per-item appetite, which is §7b.10's rule and `exhaustion_ac024_ac025`'s own route.
const FIXTURE_TWO_PASS: &str = include_str!("../fixtures/catalog/two-pass.toml");

/// The envelope both processes name, assembled the same way in each — so the parent's expectation
/// and the child's input are the same declaration rather than two literals that happen to agree.
fn declared_envelope() -> BudgetEnvelope {
    let catalog = Catalog::from_sources(&[("two-pass.toml", FIXTURE_TWO_PASS)])
        .unwrap_or_else(|e| panic!("the two-pass fixture loads: {e}"));
    let activity = catalog
        .activities()
        .first()
        .cloned()
        .expect("the fixture holds one activity");
    let declared = activity
        .budget
        .expect("the fixture declares budget = { attempts_total, tokens, wall_clock_secs }");
    let appetite = Pack::from_toml("runtime-envelope.toml", FIXTURE_ENVELOPE)
        .expect("the runtime-envelope fixture loads")
        .require_token_appetite()
        .expect(
            "the fixture pack declares `appetite.tokens`; there is no default behind this call",
        );
    BudgetEnvelope {
        attempts_total: declared.attempts_total,
        tokens: appetite,
        wall_clock_secs: declared.wall_clock_secs,
    }
}

/// A ledger at an exact spend, built the only way a ledger can be built: by folding charges.
///
/// [`BudgetLedger`] has no constructor — the fold is its only writer, which is the *"never a
/// mutable accumulator held across a process"* rule as a type fact.
fn ledger_at(envelope: &BudgetEnvelope, wall_clock: u64) -> BudgetLedger {
    let (ledger, _) = fold(
        &[
            Charge::Attempts(Attempts::new(1)),
            Charge::Tokens(Tokens::new(10)),
            Charge::WallClock(WallClockSecs::new(wall_clock)),
        ],
        envelope,
    );
    ledger
}

/// **The child.** It reaches its wall-clock budget, writes the durable mark, and **returns** —
/// which is the only thing a process can do that is not sleeping, aborting or spinning.
///
/// Two cases, differing in **one number**:
///
/// * `expired` — the ledger's wall clock **reaches** the declared one, so the mark is owed and
///   written.
/// * `inside` — one second short, so `wall_clock_expiry_mark` returns `None` and the child writes
///   nothing. **It still exits 0**, and that is what makes the parent's mark assertion attributable
///   to the budget rather than to the child having run.
#[test]
#[ignore = "child entrypoint for AC-024's exit-and-mark test; the parent invokes it by name"]
fn ac024_child_writes_the_wall_clock_mark() {
    let store = child_store();
    let case = child_case();
    let envelope = declared_envelope();
    let wall = envelope.wall_clock_secs.get();
    let spent = match case.as_str() {
        "expired" => wall,
        "inside" => wall - 1,
        other => panic!("the parent sets `expired` or `inside`, not {other:?}"),
    };
    let mut store = FileStore::at(store);

    if let Some(mark) = wall_clock_expiry_mark(&ledger_at(&envelope, spent), &envelope) {
        store.append(mark.to_event()).expect("the child appends");
        println!("child: wrote the budget-extension mark and is about to exit 0");
    } else {
        println!("child: owed no mark and is about to exit 0 having written nothing");
    }
    // AND NOTHING ELSE. No sleep, no wait, no second dispatch — the function returns, libtest
    // reports, and the process exits. AC-024's *"no thread sleeps waiting for the budget to be
    // extended"* has its own structural instrument (`exhaustion_ac024_ac025`'s obligation 5, a
    // source lint over `src/workflow/budget.rs`) and is not re-claimed here.
}

/// **AC-024's *"writes a durable mark and exits 0"*, across a real process boundary.**
///
/// > **AC-024** — GIVEN a run that reaches its wall-clock budget / WHEN the budget expires / THEN
/// > the kernel writes a durable mark and **exits 0**; no thread sleeps waiting for the budget to
/// > be extended.
/// > *Verified by:* a test asserting **process exit** and a **written mark**; the AC-003 lint (no
/// > `sleep` in the workflow) is the structural half of the same guarantee.
///
/// `exhaustion_ac024_ac025` asserts the mark's **value** and its carrier inside one process, and
/// reports *"exits 0"* ARMED in terms — *"a test that called `std::process::exit` would kill this
/// harness rather than assert anything"*. That is true of a test asserting its **own** exit. It is
/// not true of a test asserting a **child's**, and this is that test.
///
/// # The obligations, in order
///
/// 1. **The premise**: nothing on disk, and the parent's own recomputation says a run *inside* its
///    wall clock owes no mark — so a mark is evidence of expiry and not of arithmetic.
/// 2. The child reaches its budget, writes, and **the parent reads its exit code**: `Some(0)`.
/// 3. The parent reads the mark back **out of the bytes**, with its carrier and both numbers.
/// 4. **THE DISCRIMINATING CONTROL**: a second child, one second inside the same declared wall
///    clock, **also exits 0** and leaves **no mark**. Without it, every assertion above is passed
///    by a child that writes a mark unconditionally, and *"exits 0"* would be carrying the whole
///    test.
///
/// # 🛑 WHAT IS STILL ARMED, AND IT IS THE HALF A READER WOULD ASSUME
///
/// The process that exits 0 is a **libtest binary**, not `redkiln run`. What is measured is that a
/// process which reached its wall-clock budget wrote the mark and then **terminated normally** —
/// not that `rk-cli` maps that state to exit status 0, which is Step 7d's and is claimed nowhere
/// here. AC-024 is also a **twin**: the owner is the human-gate pair AC-240 + AC-278, and this half
/// may not be double-claimed against that area.
#[test]
fn ac024_wall_clock_mark_outlives_the_process_that_wrote_it() {
    const CHILD: &str = "cross_process::ac024_child_writes_the_wall_clock_mark";

    let envelope = declared_envelope();
    let wall = envelope.wall_clock_secs.get();

    // ---- 1. THE PREMISE, recomputed here rather than assumed ----------------------------------
    assert!(wall > 0, "PREMISE: the fixture declares a wall clock");
    assert_eq!(
        wall_clock_expiry_mark(&ledger_at(&envelope, wall - 1), &envelope),
        None,
        "PREMISE: one second inside the declared wall clock no mark is owed, so a mark on disk is \
         evidence the budget was REACHED"
    );
    let expected = wall_clock_expiry_mark(&ledger_at(&envelope, wall), &envelope)
        .expect("PREMISE: at the declared wall clock a mark IS owed");

    // ---- 2. THE PROCESS EXIT ------------------------------------------------------------------
    let expired_path = fresh_store("ac024-expired");
    let expired = re_exec(CHILD, &expired_path, "expired");
    expired.assert_ran_exactly_one_test_and_exited_zero(CHILD);
    assert_eq!(
        expired.status.code(),
        Some(0),
        "AC-024: the process that reached its wall-clock budget must EXIT 0"
    );

    // ---- 3. THE MARK, READ BACK OUT OF THE BYTES A DEAD PROCESS LEFT --------------------------
    let store = FileStore::at(expired_path);
    let events = store
        .query(Query::ByRun("any".to_owned()))
        .expect("read back");
    assert_eq!(
        events.len(),
        1,
        "the child wrote exactly one row: {events:?}"
    );
    assert_eq!(
        events[0].kind,
        BudgetExtensionMark::event_kind(),
        "the mark rides `EventKind::GateOpened`, one of the closed 29 — nothing here mints a \
         thirtieth (ADR-0116; `EventKind::ALL.len() == 29` is pinned in tests/it/fsm.rs)"
    );
    let marks = read_budget_extension_marks(&events);
    assert_eq!(
        marks.len(),
        1,
        "exactly one budget-extension mark: {marks:?}"
    );
    assert_eq!(marks[0].kind(), "budget_extension");
    assert_eq!(marks[0].declared(), envelope.wall_clock_secs);
    assert_eq!(marks[0].spent(), WallClockSecs::new(wall));
    assert_eq!(
        marks[0], expected,
        "the mark the dead process wrote is the mark this envelope owes"
    );

    // ---- 4. THE DISCRIMINATING CONTROL ---------------------------------------------------------
    let inside_path = fresh_store("ac024-inside");
    let inside = re_exec(CHILD, &inside_path, "inside");
    inside.assert_ran_exactly_one_test_and_exited_zero(CHILD);
    assert_eq!(
        inside.status.code(),
        Some(0),
        "the control child must also exit 0 — otherwise `exits 0` is doing the discriminating"
    );
    let control = FileStore::at(inside_path);
    let control_events = control
        .query(Query::ByRun("any".to_owned()))
        .expect("read back");
    assert!(
        control_events.is_empty(),
        "a run one second INSIDE its wall clock must leave no row at all: {control_events:?}"
    );
    assert!(
        read_budget_extension_marks(&control_events).is_empty(),
        "and therefore no mark"
    );
}

// ---- AC-013 — Phase-1 exit criterion (3), across a REAL process boundary ------------------------

/// The N-step fixture graph AC-013's *Verified by* names. **N = 3, and TWO of the three commit
/// before the kill** — which is the whole of exit criterion (3)'s *"at MORE THAN ONE STEP"*, against
/// F38's recorded state having held **exactly one**.
const N_STEP_GRAPH: [(&str, &str); 3] = [
    ("impl-report@1", "impl-report[node]"),
    ("changeset@2", "changeset[node]"),
    ("spec-authoring@1", "spec-authoring[node]"),
];

/// The child: commit two of three activations to a durable store, then **die mid-graph**.
///
/// `std::process::exit` rather than a return, a panic or a drop. Nothing unwinds, no destructor
/// runs and nothing is flushed on the way out — the third activation is simply never attempted.
/// `FileStore::append` `sync_all`s on every call, so the two rows that landed are on disk before
/// the process stops existing, which is the same property a real `taskkill /T /F` leaves behind.
///
/// **`std::process::abort` is deliberately not used**: on the MSVC CRT it can raise an error dialog,
/// and a harness that waits on a modal box is a hang rather than a test. `137` is `128 + SIGKILL`,
/// the status a killed process carries on the platforms that have one.
#[test]
#[ignore = "child entrypoint for AC-013's kill-and-resume test; the parent invokes it by name"]
fn ac013_child_commits_two_of_three_then_dies() {
    use std::sync::{Arc, Mutex};

    use rk_kernel::runtime::Runtime;
    use rk_kernel::workflow::envelope::AttemptCap;
    use rk_ports::harness::Harness;

    use crate::runtime::{CAP, CountingHarness, ITEM, STAGE, at, ok_typed, ports, spec};

    let path = child_store();
    let case = child_case();
    assert_eq!(
        case, "commit-two-of-three",
        "the parent sets this case and no other"
    );

    let db: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(FileStore::at(path)));
    let harness = Arc::new(CountingHarness::scripted(vec![
        ok_typed(serde_json::json!({ "step": 1 })),
        ok_typed(serde_json::json!({ "step": 2 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-pre-kill",
        STAGE,
    );
    for (aid, step) in &N_STEP_GRAPH[..2] {
        let report = rt
            .run_activity(ITEM, aid, step, &spec(), AttemptCap::declared(CAP), at(100))
            .expect("the pre-kill run resolves");
        assert!(report.committed(), "{aid} committed before the kill");
    }
    assert_eq!(
        harness.dispatches(),
        2,
        "the child dispatched exactly the two steps it committed"
    );
    println!("child: committed 2 of 3, about to die mid-graph with `spec-authoring@1` un-run");
    std::io::stdout()
        .flush()
        .expect("the child's stdout flushes");

    // THE KILL.
    std::process::exit(137);
}

/// **AC-013 across a real process boundary, at more than one step** — Phase-1 exit criterion (3)'s
/// asserted half.
///
/// > **AC-013** — GIVEN a run whose graph committed N steps and whose process was killed mid-graph /
/// > WHEN `redkiln run <item>` re-enters (there is no `--resume` flag) / THEN no committed step
/// > re-executes and every committed digest is byte-identical to its pre-kill value.
/// > *Verified by:* a kill-and-resume integration test in `rk-kernel` over an N-step fixture graph
/// > with the Harness fake, asserting digest equality and zero re-dispatch.
///
/// # What this adds over `runtime::ac013_re_entry_reexecutes_nothing`
///
/// That test's boundary is a **dropped `Runtime`**, and its own docstring reports the limit:
/// ADR-0087 clause 2 says *"a reload that reads state back in-process is not a valid implementation
/// of this decision and **is not evidence for it**."* Here the process that committed the steps
/// **ceases to exist** — a different OS process reads the journal it left — so the clause is
/// satisfied rather than disclosed around. D-8 routed AC-013 to Step 7d on the ground that *"the
/// thing that re-enters is `redkiln run <item>`"*; 7d is closed and this is Step 8.
///
/// # 🛑 STILL ARMED, AND NOT COUNTED
///
/// 1. **AC-013 remains PROVISIONAL on P8** — *"3 of a pre-registered 27+3 runs, and its recorded
///    state held exactly one step"* — and register §0 says a PROVISIONAL criterion may be
///    implemented but **must not be counted as covered**. This test generalises to N what the spike
///    measured at 1; it does not run the spike's missing arms.
/// 2. **The re-entrant is a kernel `Runtime`, not `redkiln run`.** The kill is real and the boundary
///    is real; the CLI re-entry AC-013's WHEN names is not exercised here, because exit criterion
///    (1) is blocked and no `redkiln run` completes on this tree. That half stays ARMED.
/// 3. The kill is `exit(137)`, not an external `taskkill /T /F` — the process is stopped by its own
///    last instruction rather than by another one. What is common to both, and what the criterion
///    turns on, is that no destructor runs and the graph is abandoned mid-flight.
#[test]
fn kill_and_resume_n_step() {
    use std::sync::{Arc, Mutex};

    use rk_kernel::runtime::Runtime;
    use rk_kernel::runtime::activity::committed_result;
    use rk_kernel::workflow::envelope::AttemptCap;
    use rk_ports::harness::Harness;

    use crate::runtime::{CAP, CountingHarness, ITEM, STAGE, at, ok_typed, ports, spec};

    const CHILD: &str = "cross_process::ac013_child_commits_two_of_three_then_dies";

    assert!(
        N_STEP_GRAPH.len() >= 3,
        "PREMISE: N >= 3, as the criterion's own fixture requires"
    );

    // ---- 1. the child commits two of three and is killed ---------------------------------------
    let path = fresh_store("ac013-kill-and-resume");
    let child = re_exec(CHILD, &path, "commit-two-of-three");
    assert_ne!(
        child.status.code(),
        Some(0),
        "PREMISE: the child must have DIED mid-graph, not finished. stdout:\n{}",
        child.stdout
    );
    assert!(
        child.stdout.contains("about to die mid-graph"),
        "PREMISE: the child reached the kill after committing, rather than failing earlier.\n--- \
         child stdout ---\n{}\n--- child stderr ---\n{}",
        child.stdout,
        child.stderr
    );

    // ---- 2. what survived the kill -------------------------------------------------------------
    let db: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(FileStore::at(path)));
    let before = db
        .lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("the journal the dead process left is readable");
    let committed_before = before
        .iter()
        .filter(|e| e.kind == EventKind::ActivityCommitted)
        .count();
    assert_eq!(
        committed_before, 2,
        "PREMISE: MORE THAN ONE step committed before the kill — F38 held exactly one, and \
         criterion (3) is that this holds at more than one"
    );
    let pre_kill: Vec<String> = N_STEP_GRAPH[..2]
        .iter()
        .map(|(aid, _)| {
            let v = committed_result(&before, ITEM, aid).expect("committed before the kill");
            rk_store::canon::canonical_digest(v)
                .expect("no floats")
                .as_str()
                .to_owned()
        })
        .collect();

    // ---- 3. a DIFFERENT process re-enters over the same journal ---------------------------------
    let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({ "step": 3 }),
    )]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-re-entry",
        STAGE,
    );
    let mut reports = Vec::new();
    for (aid, step) in &N_STEP_GRAPH {
        reports.push(
            rt.run_activity(ITEM, aid, step, &spec(), AttemptCap::declared(CAP), at(200))
                .expect("the re-entry resolves"),
        );
    }
    assert!(
        reports[0].skipped(),
        "a step committed before the kill does not re-execute"
    );
    assert!(
        reports[1].skipped(),
        "nor does the second — this is the `more than one step` the criterion names"
    );
    assert!(reports[2].committed(), "the un-run step runs");
    assert_eq!(
        harness.dispatches(),
        1,
        "AC-013: ZERO re-dispatch of a committed step across a real process boundary"
    );

    // ---- 4. digest equality --------------------------------------------------------------------
    let after = db
        .lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByItem(ITEM.to_owned()))
        .expect("readable");
    let post: Vec<String> = N_STEP_GRAPH[..2]
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
        "AC-013: every committed digest is byte-identical to its pre-kill value — this is the \
         `output_digest` ADR-0128 decision 4 names, computed the way `explain` computes it"
    );
    assert_eq!(
        after
            .iter()
            .filter(|e| e.kind == EventKind::ActivityCommitted)
            .count(),
        3,
        "and exactly one new commit joined them"
    );

    // ---- 5. THE CONTROL ------------------------------------------------------------------------
    // Without it, "the committed steps were skipped" is satisfied by a runtime that skips
    // everything — including a step the graph never ran.
    let control_harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({ "step": 4 }),
    )]));
    let control_rt = Runtime::new(
        ports(Arc::clone(&control_harness) as Arc<dyn Harness>, &db),
        "run-control",
        STAGE,
    );
    let never = control_rt
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
        control_harness.dispatches(),
        1,
        "CONTROL: an activation the killed graph never committed IS dispatched on re-entry"
    );

    println!(
        "kill_and_resume_n_step: N={} committed_before_kill={committed_before} re_dispatches=1 \
         digests={pre_kill:?}",
        N_STEP_GRAPH.len()
    );
}
