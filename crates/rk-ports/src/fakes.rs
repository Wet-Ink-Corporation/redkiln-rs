//! The five fakes — design §3.3's Required-fake column, and AC-481's evidence line verbatim
//! (register :6917): in-memory `Vec`, scripted responder, in-memory board, no-op recorder,
//! fixture-driven.
//!
//! **All five are `pub` library items, not `#[cfg(test)]` stubs.** The reference shape ships two
//! — `VecStore` and `FakeHarness` — and implements `Tracker`, `DocsSite` and `Toolchain` only with
//! test-local `struct T` / `struct D` / `struct C` inside its own `#[cfg(test)] mod tests`. Those
//! are stubs, not fakes: Checkpoint 7a counts `impl <Port> for` **anchored at column 0**, and a
//! stub inside a test module is indented; and AC-498's *"using fakes for all five ports"* is a
//! claim about what the kernel's own suite can construct, one crate over, where a `#[cfg(test)]`
//! item does not exist at all.
//!
//! **Every error these fakes construct is `rk-types`'.** The spike declares its own `StepError`
//! and `spoke()` under a `// ---- rk-types` header because it is a throwaway single crate; the
//! payloads differ from the shipped ones in three variants, so an error construction lifted
//! unchanged does not compile. `spoke()` is **not** re-implemented here and may not be: the
//! spike's `matches!` form expands to an implicit catch-all that would silently accept a twelfth
//! variant, and the arm-per-variant form trips `clippy::match_same_arms`, denied individually at
//! `Cargo.toml`:65. The bridge below goes through `StepError::event()`, which `rk-types` already
//! owns and which is total over the closed eleven.
//!
//! *(2026-09-26, `s8d` build, package P6; R4 = ADR-0176, proposed.)* **That bridge is gone.**
//! `FakeHarness` wrote the `step_failed` / `step_cancelled` / `rate_limited` / `step_refused` row
//! itself (`fn event_for`), and `ClaudeHarness` wrote none, so the class reached the journal under
//! the fake only. D21-42 20: *"The kernel appends `StepError::event()` for every dispatch, so fake
//! and real harnesses behave alike"*. The kernel's retry loop now writes that row
//! (`rk_kernel::runtime::activity::error_row`), and this fake appends nothing on any path.
//!
//! *(dated 2026-09-26, `s8d` build, package F1-D2172; D21-72 option (B), R4 = ADR-0176,
//! proposed.)* ~~appends nothing on any path~~ is re-amended to **no error row**: on an `Err`
//! termination and on its cancel path the fake appends ADR-0126's `dispatch_completed` (the serde
//! of its fake record), as the real adapter does, which is what `conformance::conform_harness`'s
//! error arm (AC-360) reads; on no `Ok` path does it append anything. **Stated cost** (D21-72): on
//! the `Ok` path the fake's journal differs from `ClaudeHarness`'s, which appends
//! `dispatch_completed` on every termination.

use std::sync::Mutex;

use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::digest::sha256;
use rk_types::docset::DocSet;
use rk_types::drift::Drift;
use rk_types::error::{CancelledBy, StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::retention::Retained;
use rk_types::typed::Typed;
use rk_types::usage::{CliVersion, Timings};

use crate::cancel::CancelToken;
use crate::docs_site::DocsSite;
use crate::event_store::{EventStore, Query, Snapshot, SnapshotRow, TierOneAnchor};
use crate::harness::{DispatchOutcome, Harness};
use crate::toolchain::{
    ChangeKind, ChangeSummary, Classification, ManifestEntry, Selection, StepBound, TestOutcome,
    TestRunner, TextChangeSummary, Toolchain, VerbCall, VerbOutput,
};
use crate::tracker::{ExternalRef, Tracker, TypedInput};
use rk_types::verb_output::{BuildReport, BuildStatus};
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource, VerifyReport};

/// The message every fake's own lock carries when it is poisoned.
///
/// **This is not poison recovery and may not become it** (ADR-0118 clauses 1 and 3): `.expect()`
/// propagates, it does not resume. No spelling of `PoisonError::into_inner`, and no `catch_unwind`
/// that resumes the run, may appear in this crate — and note the scope: ADR-0118 clause 4 is about
/// the two **store** locks, while these are a fake's own internal state, which ADR-0115 clause 5's
/// one-seam rule also does not reach (its subject is how the composition root *holds* a port).
const LOCK: &str = "a poisoned lock halts the run; it is never recovered (ADR-0118)";

// ---------------------------------------------------------------- EventStore: an in-memory Vec

/// The `EventStore` fake: an in-memory `Vec` (design:265).
///
/// **`query` returns every appended event whatever the selector, and that is a consequence of a
/// routing rather than a shortcut.** `rk-types`' `Event` is `{ kind, payload }` and the per-kind
/// payload field sets are not typed yet (`s7a` §7a.5 — the typed-payload record is OWED), so an
/// event carries no item id, commit SHA or run id for the fake to filter on. When that record
/// lands this fake gains the filter, and the conformance body above it does not change.
///
/// **`snapshot`/`rebuild` are identity here for the same reason.** `Snapshot` is one of this
/// crate's opaque newtypes with no field set decided, so it carries nothing to restore from. What
/// the conformance suite can assert at 7a is that rebuilding from a snapshot of the *current*
/// state leaves the store unchanged; a round-trip through a **stale** snapshot is not assertable
/// until `Snapshot`'s contents are decided, and it is not claimed.
///
/// *(dated 2026-09-28, s8e build; ADR-0181 decisions 1 and 12 and ADR-0182 decisions 4 and 5,
/// proposed.)* ~~`query` returns every appended event whatever the selector~~: the event now
/// carries its interim `item` key and an anchored row its anchor, so the fake filters as the real
/// store's SQL does — `ByItem(x)` the rows keyed `x` plus, in the interim, the unkeyed rows;
/// `BySha(s)` the rows `append_tier1` anchored to `s`; `ByRun` every row. **`append_tier1` keeps the
/// anchor beside the event** and models no `t1_*` table: the tier-1 row is the SQLite row's, and
/// `rk-adapter-sqlite`'s `store::append_tier1_…` asserts it there.
#[derive(Debug, Default)]
pub struct VecStore {
    events: Vec<Event>,
    anchors: Vec<Option<String>>,
}

impl VecStore {
    /// Everything appended, in order. The fake's own window, not part of the port.
    #[must_use]
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// The anchor the `index`-th appended event was written against, if it came through
    /// `append_tier1`. The fake's own window, not part of the port.
    #[must_use]
    pub fn anchor_of(&self, index: usize) -> Option<&str> {
        self.anchors.get(index).and_then(Option::as_deref)
    }
}

impl EventStore for VecStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        self.events.push(event);
        self.anchors.push(None);
        Ok(())
    }

    fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError> {
        self.events.push(event);
        self.anchors.push(Some(anchor.sha().to_owned()));
        Ok(())
    }

    fn query(&self, by: Query) -> Result<Vec<Event>, StepError> {
        let keep = |(event, anchor): &(&Event, &Option<String>)| match &by {
            Query::ByItem(item) => event.item().is_none_or(|i| i == item),
            Query::BySha(sha) => anchor.as_deref() == Some(sha.as_str()),
            Query::ByRun(_) => true,
        };
        Ok(self
            .events
            .iter()
            .zip(self.anchors.iter())
            .filter(keep)
            .map(|(event, _)| event.clone())
            .collect())
    }

    /// *(dated 2026-09-28, `s8e` build, package S8E-P7; ADR-0181 decision 11, proposed.)* Every
    /// row in order, each with the anchor the fake kept. The fake keeps the anchor's sha only, so its
    /// rows carry no tree and no opening run.
    fn snapshot(&self) -> Result<Snapshot, StepError> {
        Ok(Snapshot::new(
            self.events
                .iter()
                .zip(self.anchors.iter())
                .map(|(event, anchor)| SnapshotRow {
                    event: event.clone(),
                    anchor_sha: anchor.clone(),
                    tree: None,
                    opening_run: None,
                })
                .collect(),
        ))
    }

    /// *(dated 2026-09-28, `s8e` build, package S8E-P7.)* Append-only, as the port states: the
    /// fake's rows must be a prefix of the snapshot's, and it gains the rest.
    fn rebuild(&mut self, snapshot: &Snapshot) -> Result<(), StepError> {
        let current = self.snapshot()?;
        let held = current.rows();
        if held.len() > snapshot.rows().len() || snapshot.rows()[..held.len()] != *held {
            return Err(StepError::Transport {
                kind: TransportKind::Io,
                detail: "rebuild: the store's rows are not a prefix of the snapshot's, and a \
                         rebuild neither updates nor deletes (AC-364)"
                    .to_owned(),
            });
        }
        for row in &snapshot.rows()[held.len()..] {
            self.events.push(row.event.clone());
            self.anchors.push(row.anchor_sha.clone());
        }
        Ok(())
    }
}

// ---------------------------------------------------------------- Harness: a scripted responder

/// The `Harness` fake: a scripted responder (design:266).
#[derive(Debug)]
pub struct FakeHarness {
    script: Mutex<Vec<Result<Typed, StepError>>>,
    token: CancelToken,
}

impl FakeHarness {
    /// Script a sequence of outcomes, consumed front to back.
    #[must_use]
    pub fn scripted(script: Vec<Result<Typed, StepError>>) -> Self {
        Self {
            script: Mutex::new(script),
            token: CancelToken::new(),
        }
    }

    /// Hand the fake the composition root's token, exactly as a real adapter receives it at
    /// construction. The kernel never holds it and never fires it (ADR-0099 clause 1).
    #[must_use]
    pub fn with_token(mut self, token: CancelToken) -> Self {
        self.token = token;
        self
    }

    fn next(&self) -> Option<Result<Typed, StepError>> {
        let mut script = self.script.lock().expect(LOCK);
        if script.is_empty() {
            None
        } else {
            Some(script.remove(0))
        }
    }
}

/// The record every dispatch produces, whatever the outcome (AC-360). Argv and env are **digests
/// only** (ADR-0105 clause 7) and there is no field for the child environment at all.
///
/// **Completed at Step 7c Slice 1 with ADR-0126's members, in the pre-spawn shape**: the fake
/// spawns nothing and reads no `system/init`, so its version is `Pinned` (the shape every
/// pre-spawn row has — a fake that wrote `Observed` would be the pin masquerading as a
/// measurement, Halt 11's shape) and every usage-adjacent member is `None` — never `Some(0)`
/// (AC-362). The count of `system/api_retry` events is `0`, a measurement of a stream the fake
/// never read; the two TTL values are the pair the register's fixtures set (AC-210); `wall_ms`
/// is `0` because the fake reads no clock (AC-001).
fn fake_record() -> DispatchRecord {
    DispatchRecord {
        argv_digest: sha256(b"fake-argv"),
        env_digest: sha256(b"fake-env"),
        instrument_version: "fake".to_owned(),
        stdout: Retained::of(b""),
        stderr: Retained::of(b""),
        usage: None,
        timings: Timings {
            wall_ms: 0,
            api_ms: None,
        },
        total_cost_usd: None,
        headroom_before: None,
        headroom_after: None,
        cli_version: CliVersion::Pinned("fake".to_owned()),
        api_retry_events: 0,
        prompt_cache_ttl: "1h".to_owned(),
        subagent_prompt_cache_ttl: "1h".to_owned(),
        cancelled_by: None,
        detected_by: None,
    }
}

// *(2026-09-26, `s8d` build, package P6; R4 = ADR-0176 decision 2, proposed.)* `fn event_for`, the
// fake's own bridge from a failed step to its event arm, is REMOVED with both of its appends. The
// kernel writes the one error row per failed dispatch; a fake that also wrote one would put a
// second row beside it (mutant M11).

/// *(dated 2026-09-26, `s8d` build, package F1-D2172; D21-72 option (B).)* Append ADR-0126's
/// `dispatch_completed` for `record` — the record serialised as the payload, the shape
/// `rk-adapter-claude`'s `record::completed_event` writes. Called on an `Err` termination and on
/// the cancel path only, never on an `Ok` one.
///
/// A refused append does not fail the dispatch: the row is tier 2 and its loss is a warning, never
/// a `StepError` (AC-368), and the fake keeps no warning counter, so a refusal is dropped here.
fn append_dispatch_completed(store: &mut dyn EventStore, record: &DispatchRecord) {
    let payload = serde_json::to_value(record)
        .expect("a DispatchRecord serialises (every member derives Serialize)");
    if store
        .append(Event::new(EventKind::DispatchCompleted, payload))
        .is_err()
    {
        // AC-368: a tier-2 append failure never fails the activity.
    }
}

impl Harness for FakeHarness {
    fn dispatch(&self, _spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        let record = fake_record();

        // The token is the composition root's, fired by the CLI's signal path or the per-activity
        // timeout. The fake READS it; it never fires it, and neither does the kernel.
        if self.token.is_cancelled() {
            append_dispatch_completed(store, &record);
            return DispatchOutcome {
                record,
                result: Err(StepError::Cancelled {
                    by: CancelledBy::Operator,
                }),
            };
        }

        let result = self.next().unwrap_or_else(|| {
            Err(StepError::Transport {
                kind: TransportKind::Spawn,
                detail: "script exhausted".to_owned(),
            })
        });

        // D21-72 (B): the `Err` path appends `dispatch_completed`; the `Ok` path appends nothing.
        if result.is_err() {
            append_dispatch_completed(store, &record);
        }

        DispatchOutcome { record, result }
    }
}

// ---------------------------------------------------------------- Tracker: an in-memory board

/// The `Tracker` fake: an in-memory board (design:267).
#[derive(Debug, Default)]
pub struct BoardTracker {
    projected: Mutex<Vec<String>>,
    inbox: Mutex<Vec<TypedInput>>,
}

impl BoardTracker {
    /// Put one typed input on the inbox, as an external tracker would.
    pub fn push_inbox(&self, input: TypedInput) {
        self.inbox.lock().expect(LOCK).push(input);
    }

    /// What the board holds. The fake's own window, not part of the port.
    #[must_use]
    pub fn board(&self) -> Vec<String> {
        self.projected.lock().expect(LOCK).clone()
    }
}

impl Tracker for BoardTracker {
    fn project(&self, item: &str) -> Result<ExternalRef, StepError> {
        let mut board = self.projected.lock().expect(LOCK);
        // **Idempotent writes** (design:267): projecting the same item twice leaves ONE entry.
        // Getting this wrong is the failure shipmates L5.4 records — one thread holding the same
        // report posted three times by their own orchestrator.
        if !board.iter().any(|existing| existing == item) {
            board.push(item.to_owned());
        }
        Ok(ExternalRef(()))
    }

    fn inbox(&self) -> Result<Vec<TypedInput>, StepError> {
        Ok(self.inbox.lock().expect(LOCK).clone())
    }
}

// ---------------------------------------------------------------- DocsSite: a no-op recorder

/// The `DocsSite` fake: a no-op recorder (design:268).
#[derive(Debug, Default)]
pub struct RecordingDocs {
    calls: Mutex<Vec<&'static str>>,
    drift: Mutex<Vec<Drift>>,
}

impl RecordingDocs {
    /// Script what the next `drift` call reports.
    pub fn will_report(&self, drift: Vec<Drift>) {
        *self.drift.lock().expect(LOCK) = drift;
    }

    /// What the recorder observed, in order. The fake's own window, not part of the port.
    #[must_use]
    pub fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().expect(LOCK).clone()
    }

    fn record(&self, call: &'static str) {
        self.calls.lock().expect(LOCK).push(call);
    }
}

impl DocsSite for RecordingDocs {
    fn build(&self, _docs: &DocSet) -> Result<(), StepError> {
        self.record("build");
        Ok(())
    }

    fn check_links(&self) -> Result<(), StepError> {
        self.record("check_links");
        Ok(())
    }

    fn drift(&self, _changed_sources: &[String]) -> Result<Vec<Drift>, StepError> {
        self.record("drift");
        Ok(self.drift.lock().expect(LOCK).clone())
    }
}

// ---------------------------------------------------------------- Toolchain: fixture-driven

/// The `Toolchain` fake: fixture-driven (design:269).
///
/// The fixture is an **ordered list of `(path, classification)` pairs**, not a map: `HashMap` and
/// `HashSet` are in `clippy.toml`'s `disallowed-types`, denied through the workspace's
/// `all = { level = "deny", priority = -1 }`, and CI lints test targets.
///
/// **A path the fixture does not name classifies as `Text`.** The fixture drives what is *not*
/// text, which is what AC-497 turns on — `Binary` is the discriminant its port shape carries
/// (register :7109).
///
/// *(added 2026-09-24, s8a, ADR-0165)* **A range carries what [`Self::with_range`] scripted for
/// it**, and `change_summary` answers from that: `Text` when every path classifies `Text` or
/// `Generated`, a by-name refusal when one classifies `Binary`.
///
/// *(added 2026-09-26, s8c §8c.2, records T and V, proposed)* **`test` and `build` return
/// outcomes.** `test` executes as many tests as a `Filter` selection names (`passed` = that
/// length, `failed = 0`) unless [`Self::failing`] scripts `n` of them red, which is `Ok` with
/// `failed = n`: a red suite is never `Err` (D21-42 1a). A `WholeSuite` selection executes none,
/// so its status is `empty`, never `passing` (AC-332). `Affected` and `Named` are refused by name:
/// the fake declares neither. `build` is `passing` unless [`Self::build_failing`] scripts it red,
/// which is `Ok` with `status: failing`. The report members are non-empty fixture values and the
/// runner is `fixture`. `run_verb` answers the verbs [`Self::with_verb`] scripted and refuses
/// every other one BY its bare name.
#[derive(Debug, Default)]
pub struct FixtureToolchain {
    fixture: Vec<(String, Classification)>,
    ranges: Vec<(String, Vec<(String, ChangeKind)>)>,
    present: bool,
    failing: u32,
    build_failing: bool,
    verbs: Vec<(String, VerbOutput)>,
}

impl FixtureToolchain {
    /// Drive the fake from a fixture: a path, and what the toolchain says about it.
    #[must_use]
    pub fn from_fixture(fixture: Vec<(&str, Classification)>) -> Self {
        Self {
            fixture: fixture
                .into_iter()
                .map(|(path, class)| (path.to_owned(), class))
                .collect(),
            ranges: Vec::new(),
            present: true,
            failing: 0,
            build_failing: false,
            verbs: Vec::new(),
        }
    }

    /// Script what a range carries: its changed paths and how each changed.
    #[must_use]
    pub fn with_range(mut self, range: &str, paths: Vec<(&str, ChangeKind)>) -> Self {
        self.ranges.push((
            range.to_owned(),
            paths
                .into_iter()
                .map(|(path, change)| (path.to_owned(), change))
                .collect(),
        ));
        self
    }

    /// Script `detect` as reporting the toolchain absent.
    #[must_use]
    pub fn absent(mut self) -> Self {
        self.present = false;
        self
    }

    /// Script `n` of the selected tests red: `test` answers `Ok` with `failed = n` (capped at
    /// the executed count), never `Err` (D21-42 1a).
    #[must_use]
    pub const fn failing(mut self, n: u32) -> Self {
        self.failing = n;
        self
    }

    /// Script the build red: `build` answers `Ok` with `status: failing` and one error, never
    /// `Err` (D21-43 N25).
    #[must_use]
    pub const fn build_failing(mut self) -> Self {
        self.build_failing = true;
        self
    }

    /// Script what `run_verb` answers for `verb`. Every verb not scripted is refused by name.
    #[must_use]
    pub fn with_verb(mut self, verb: &str, output: VerbOutput) -> Self {
        self.verbs.push((verb.to_owned(), output));
        self
    }
}

impl Toolchain for FixtureToolchain {
    fn detect(&self) -> Result<bool, StepError> {
        Ok(self.present)
    }

    fn build(&self, _bound: StepBound) -> Result<BuildReport, StepError> {
        let (status, errors) = if self.build_failing {
            (BuildStatus::Failing, 1)
        } else {
            (BuildStatus::Passing, 0)
        };
        Ok(BuildReport {
            argv: vec!["fixture".to_owned(), "build".to_owned()],
            toolchain_version: "fixture".to_owned(),
            wall_secs: 0,
            errors,
            status,
        })
    }

    fn test(&self, selection: &Selection, _bound: StepBound) -> Result<TestOutcome, StepError> {
        let (args, reason): (&[String], BasisReason) = match selection {
            Selection::WholeSuite { reason } => (&[], *reason),
            Selection::Filter(args) => (args.as_slice(), BasisReason::ProbeUnverified),
            Selection::Affected { .. } => {
                return Err(StepError::Unsupported {
                    capability: "affected-selection".to_owned(),
                });
            }
            Selection::Named(name) => {
                return Err(StepError::Unsupported {
                    capability: format!(
                        "selection `{name}`: the fixture declares no named selection"
                    ),
                });
            }
        };
        let executed = u32::try_from(args.len()).unwrap_or(u32::MAX);
        let failed = self.failing.min(executed);
        let mut argv = vec!["fixture".to_owned(), "test".to_owned()];
        argv.extend(args.iter().cloned());
        let report = VerifyReport {
            basis: Basis::WholeSuite,
            argv,
            selected_test_count: executed,
            total_test_count: executed,
            total_test_count_source: TotalTestCountSource::OwnExecutedCount,
            wall_secs: 0,
            toolchain_version: "fixture".to_owned(),
            suite_sha: "fixture".to_owned(),
            basis_reason: Some(reason),
        };
        TestOutcome::new(
            report,
            executed - failed,
            failed,
            TestRunner::named("fixture", "0"),
        )
    }

    fn lint(&self, _bound: StepBound) -> Result<(), StepError> {
        Ok(())
    }

    fn classify(&self, path: &str) -> Result<Classification, StepError> {
        let found = self
            .fixture
            .iter()
            .find(|(fixture_path, _)| fixture_path == path)
            .map(|(_, class)| *class);
        Ok(found.unwrap_or(Classification::Text))
    }

    /// *(amended 2026-09-24, s8a, ADR-0165: this returned the opaque `ChangeSummary(())` for every
    /// range.)* The range carries the paths [`FixtureToolchain::with_range`] scripted for it, each
    /// classified by the fixture. Every path `Text` or `Generated` yields the `Text` arm with one
    /// manifest row per path. A path that classifies `Binary` refuses **by name** -- the capability
    /// token, the range and the path -- because the `Binary` arm is sealed until S-BIN and a
    /// `Text` that silently omitted the path would be the AC-497 failure. An empty or blank range,
    /// and a range nobody scripted, refuse **by name** -- the capability token and the range --
    /// because ADR-0165 forbids *"returning an empty `Text`, or any `Ok`, for an empty or
    /// unresolvable range"*, and the fake answers as the Rust adapter does (amended 2026-09-25, s8a
    /// fix round 3: both yielded an empty `Text`, so the kernel's `change_summary("")` was an `Ok`
    /// on every walk binding this fake). A range scripted with no paths changes nothing and refuses
    /// the same way, as the Rust adapter refuses a change-less range (amended 2026-09-25, s8a fix
    /// round 4: it answered `Ok(Text)` with an empty manifest). The fake's `diff` stays empty: it
    /// reads no repository, so it has no diff to carry. The `sha` is the range's head token as
    /// written -- `HEAD` for `HEAD~1..HEAD`, the whole string for a range with no `..` -- and never
    /// a resolved 40-hex commit: a fake reads no repository (amended 2026-09-25, s8a fix round 1).
    fn change_summary(&self, range: &str, _bound: StepBound) -> Result<ChangeSummary, StepError> {
        let paths = self
            .ranges
            .iter()
            .find(|(scripted, _)| scripted == range)
            .map(|(_, paths)| paths.as_slice())
            .filter(|paths| !range.trim().is_empty() && !paths.is_empty())
            .ok_or_else(|| StepError::Unsupported {
                capability: format!(
                    "change_summary: the range `{range}` is empty, was never scripted, or changes \
                     nothing; an empty, unresolvable or change-less range never yields an empty \
                     Text (ADR-0165)"
                ),
            })?;
        let mut manifest = Vec::with_capacity(paths.len());
        for (path, change) in paths {
            let classification = self.classify(path)?;
            if classification == Classification::Binary {
                return Err(StepError::Unsupported {
                    capability: format!(
                        "change_summary: the range `{range}` carries `{path}`, which classifies \
                         Binary; the Binary arm is sealed until S-BIN (ADR-0165, AC-497)"
                    ),
                });
            }
            manifest.push(ManifestEntry {
                path: path.clone(),
                classification,
                change: *change,
            });
        }
        let sha = range
            .split_once("..")
            .map_or(range, |(_, head)| head)
            .to_owned();
        Ok(ChangeSummary::Text(TextChangeSummary {
            sha,
            diff: String::new(),
            manifest,
        }))
    }

    /// The scripted verbs answer their output; every other verb is refused by its bare name
    /// (record V decision 2; ADR-0135 clause 2's shape).
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.verbs
            .iter()
            .find(|(verb, _)| verb == call.verb)
            .map(|(_, output)| output.clone())
            .ok_or_else(|| StepError::Unsupported {
                capability: call.verb.to_owned(),
            })
    }
}
