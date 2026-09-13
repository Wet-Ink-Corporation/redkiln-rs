//! AC-499's fake half — the shared per-port conformance suite.
//!
//! **One parameterised body per port** (register :7136-7137), each taking the port as a **trait
//! object** so the identical body runs against a real adapter when 7c fills the real rows in. The
//! fake row of all five ports is instantiated here; each Phase-1 real row is present as a **named,
//! empty** row rather than absent — SQLite, Claude, Node, Rust (register :7139-7140) — so that
//! what is owed is visible in the suite rather than only in a document.
//!
//! **`Tracker` and `DocsSite` have a fake only until Phase 3** and this criterion does not fail on
//! their absence (ADR-0094 decision 2; AC-499's GIVEN), so they carry no real row at all.
//!
//! **The register's qualifier, carried rather than dropped:** *"The Harness real row runs a real
//! dispatch and therefore needs the CI decision ADR-0010 defers — until it is taken, that row is a
//! **local-only, named-runner** row"* (register :7140-7141). It may not be wired into CI here or at
//! 7c. *(The register is self-inconsistent about where that dependency is recorded: AC-499 says
//! *"the §14 State line carries the dependency"* and the State line at register :6899 carries no
//! AC-499, no local-only row and no ADR-0010 dependency. An amendment to :6899 is OWED; this file
//! states it rather than choosing.)*
//!
//! **It is ONE `#[test]`**, deliberately: Checkpoint 7a requires exactly one line matching
//! `^test .*\bport_conformance_fake_rows\b.* \.\.\. ok$`, so five separate `#[test]`s are not
//! available. The parameterisation is in the bodies, not in the test count.

use std::collections::BTreeSet;

use rk_ports::docs_site::DocsSite;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::harness::Harness;
use rk_ports::toolchain::{Classification, Toolchain};
use rk_ports::tracker::Tracker;
use rk_types::docset::DocSet;
use rk_types::drift::Drift;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::source_parse::Source;
use crate::support::{findings, spec};

/// The run key every body queries by. The fake's `query` ignores the selector (its `Event` carries
/// no run id yet — the typed-payload record is OWED, `s7a` §7a.5), so this is the same key on both
/// sides of a dispatch and the count is what the assertion turns on.
const RUN: &str = "run-1";

/// The four `{fake, real}` rows Phase 1 ships (ADR-0094; register :7139-7140). The third element
/// is the row's assertion list: **empty, and named as empty**, because filling them is 7c's.
const PHASE_1_REAL_ROWS: [(&str, &str, &[&str]); 4] = [
    ("EventStore", "SQLite", &[]),
    ("Harness", "Claude", &[]),
    ("Toolchain", "Node", &[]),
    ("Toolchain", "Rust", &[]),
];

// ---------------------------------------------------------------- one body per port

/// `EventStore`: what was appended comes back, and rebuilding from a snapshot of the current state
/// leaves it unchanged (design:265's four methods).
fn conform_event_store(store: &mut dyn EventStore) {
    let event = Event::new(EventKind::ReconcileStarted, serde_json::json!({"n": 1}));
    store.append(event.clone()).expect("append");

    let back = store
        .query(Query::ByRun(RUN.to_owned()))
        .expect("query by run");
    assert_eq!(
        back,
        vec![event.clone()],
        "append then query must return what was appended"
    );

    let snapshot = store.snapshot().expect("snapshot");
    store.rebuild(&snapshot).expect("rebuild");
    assert_eq!(
        store
            .query(Query::ByRun(RUN.to_owned()))
            .expect("query after rebuild"),
        vec![event],
        "rebuilding from a snapshot of the current state is identity"
    );
}

/// `Harness`: `dispatch` blocks to completion and hands back **both** halves — the record on every
/// termination (AC-360) and the step's own outcome as a decided value.
fn conform_harness(harness: &dyn Harness, store: &mut dyn EventStore) {
    let before = store
        .query(Query::ByRun(RUN.to_owned()))
        .expect("query")
        .len();
    let outcome = harness.dispatch(&spec(), store);

    assert!(
        !outcome.record.instrument_version.is_empty(),
        "every dispatch produces a record carrying the instrument version (AC-360; Halt 2)"
    );
    match &outcome.result {
        Ok(typed) => assert!(
            !typed.schema.as_str().is_empty(),
            "a Typed value carries the schema id it validated against (ADR-0099:59-60)"
        ),
        Err(err) => {
            // Not a catch-all: both are total over the closed eleven, so an error that reaches
            // here has a retry class and exactly one event arm.
            let _ = err.retry_class();
            let _ = err.event();
            // AC-360 (register :5192-5197), as an assertion and not as a comment: *"no dispatch
            // path exists that can terminate without appending that event"*. This arm used to
            // carry that sentence in prose and nothing that could fail -- and it never ran at
            // all, because the matrix scripted only a successful row [measured: a `panic!` as its
            // first statement left `port_conformance_fake_rows` green]. The matrix now runs this
            // body over BOTH a scripted-Ok row and a scripted-error row.
            let after = store
                .query(Query::ByRun(RUN.to_owned()))
                .expect("query")
                .len();
            assert!(
                after > before,
                "a dispatch terminated in {err:?} without appending an event (AC-360, register \
                 :5192-5197): the store held {before} events before and {after} after"
            );
        }
    }
}

/// `Tracker`: `project` is called twice on one item, `inbox` is readable, and projecting does not
/// enqueue inbound work.
///
/// **What this body can and cannot assert about idempotency, stated rather than implied** (golden
/// constraint 10). design:267 fixes *"idempotent writes"*, and the equality below is the shape of
/// that claim — but at 7a it is **ARMED, NOT ASSERTED**: `ExternalRef` is `pub struct
/// ExternalRef(pub(crate) ())` while its contents are OWED (no record assigns them; AC-508's
/// idempotency key is Phase 3), so every value of it is equal to every other and the assertion
/// holds by construction whatever `project` does [measured: removing `BoardTracker`'s dedup guard
/// left this assertion passing]. It becomes live the moment that record lands and the newtype
/// gains contents, which is why it is written now rather than added then.
///
/// **The discriminating observation at 7a is the fake's own board window** (`tracker.board()` in
/// the matrix below), which is fake-specific and does NOT carry to 7c's real rows. What this body
/// carries to 7c instead is the port-visible invariant under it: a `project` is an outbound write
/// and never enqueues inbound work, so `inbox` is unchanged across both calls.
fn conform_tracker(tracker: &dyn Tracker) {
    let inbox_before = tracker.inbox().expect("inbox").len();

    let first = tracker.project("item-1").expect("project");
    let again = tracker.project("item-1").expect("project is idempotent");
    assert_eq!(
        first, again,
        "projecting the same item twice yields the same external reference (design:267)"
    );

    assert_eq!(
        tracker.inbox().expect("inbox").len(),
        inbox_before,
        "projecting an item outward must not enqueue inbound work: the inbox held \
         {inbox_before} entries before two projections"
    );
}

/// `DocsSite`: all three of design:268's methods are callable, and drift is reported as
/// observations rather than as a boolean.
///
/// **The caller must script at least one drift observation before calling this**, and the
/// parameter says so rather than leaving it to a convention: the loop below is the only assertion
/// in the body, and over an unscripted row it is a **zero-iteration** loop that asserts nothing
/// [measured: `assert!(false)` as its body left `port_conformance_fake_rows` green]. `expect_drift`
/// is what makes the vacuity a failure instead of a silence -- a real row at 7c that has no drift
/// to report passes `0` and says so.
fn conform_docs_site(docs: &dyn DocsSite, expect_drift: usize) {
    docs.build(&DocSet::new()).expect("build");
    docs.check_links().expect("check_links");
    let drift = docs
        .drift(&["docs/index.md".to_owned()])
        .expect("drift(changed_sources)");
    assert_eq!(
        drift.len(),
        expect_drift,
        "the row scripted {expect_drift} drift observations and `drift(changed_sources)` \
         reported {}: {drift:?}",
        drift.len()
    );
    for observation in &drift {
        assert!(
            !observation.kind().is_empty(),
            "every drift observation carries a stable discriminator: {observation:?}"
        );
    }
}

/// `Toolchain`: all six of design:269's methods are callable, and `classify` answers the
/// classification the row's fixture declares **per path**.
///
/// **The row supplies its own `(path, classification)` expectations, and that is what makes this
/// body able to fail.** The previous form asserted
/// `matches!(c, Text | Binary | Generated)`, which is **irrefutable over a three-variant enum**: a
/// `classify` that ignored its input and answered `Binary` for everything satisfied it, and so did
/// one that answered `Text` for everything [both measured -- `port_conformance_fake_rows` stayed
/// green in both directions, and only `toolchain_fake_classifies_binary`, outside this body,
/// caught them]. 7c's real rows pass their own real paths.
fn conform_toolchain(toolchain: &dyn Toolchain, expected: &[(&str, Classification)]) {
    toolchain.detect().expect("detect");
    toolchain.build().expect("build");
    toolchain.test(&[]).expect("test(selection)");
    toolchain.lint().expect("lint");

    assert!(
        !expected.is_empty(),
        "a Toolchain conformance row must name at least one (path, classification) pair, or the \
         loop below asserts nothing (design:269; AC-497's header fixes Text/Binary/Generated)"
    );
    for (path, want) in expected {
        assert_eq!(
            toolchain.classify(path).expect("classify(path)"),
            *want,
            "classify({path}) must answer {want:?} (design:269; AC-497's header)"
        );
    }

    toolchain
        .change_summary("HEAD~1..HEAD")
        .expect("change_summary(range)");
}

// ---------------------------------------------------------------- the matrix

#[test]
fn port_conformance_fake_rows() {
    // The expected port set comes from the SAME source parse AC-481's parity count uses, not from
    // a list in this file: a port added without a conformance row is then caught here too.
    let ports = Source::parse().traits;
    let mut exercised: BTreeSet<String> = BTreeSet::new();

    // --- EventStore / fake: the in-memory Vec
    let mut store = VecStore::default();
    conform_event_store(&mut store);
    exercised.insert("EventStore".to_owned());

    // --- Harness / fake: the scripted responder, run over BOTH of the body's arms. One row
    // scripting only success leaves the Err arm -- where AC-360's event assertion lives -- dead
    // [measured: a `panic!` as its first statement left this test green].
    let harness = FakeHarness::scripted(vec![Ok(findings(serde_json::json!({"ok": true})))]);
    let mut harness_store = VecStore::default();
    conform_harness(&harness, &mut harness_store);
    let refusing = FakeHarness::scripted(vec![Err(StepError::Unsupported {
        capability: "affected-selection".to_owned(),
    })]);
    let mut refused_store = VecStore::default();
    conform_harness(&refusing, &mut refused_store);
    exercised.insert("Harness".to_owned());

    // --- Tracker / fake: the in-memory board
    let tracker = BoardTracker::default();
    conform_tracker(&tracker);
    // THE DISCRIMINATING IDEMPOTENCY OBSERVATION, and it is deliberately here and not in the
    // reusable body: `ExternalRef`'s contents are OWED, so the body's `first == again` cannot fail
    // yet (see `conform_tracker`). This window is the fake's own and does not carry to 7c.
    assert_eq!(
        tracker.board().len(),
        1,
        "the in-memory board holds ONE entry after two projections of one item"
    );
    exercised.insert("Tracker".to_owned());

    // --- DocsSite / fake: the no-op recorder, with ONE drift observation scripted so the body's
    // per-observation assertion actually executes.
    let docs = RecordingDocs::default();
    docs.will_report(vec![Drift::Template {
        artifact: "docs/index.md".to_owned(),
        from: "v1".to_owned(),
        to: "v2".to_owned(),
    }]);
    conform_docs_site(&docs, 1);
    assert_eq!(
        docs.calls(),
        vec!["build", "check_links", "drift"],
        "the no-op recorder observed each call"
    );
    exercised.insert("DocsSite".to_owned());

    // --- Toolchain / fake: fixture-driven, and the row declares what each path must classify as,
    // so the body's assertion can fail on a `classify` that ignores its input.
    let fixture = [
        ("README.md", Classification::Text),
        ("assets/logo.png", Classification::Binary),
        ("target/schema.json", Classification::Generated),
    ];
    let toolchain = FixtureToolchain::from_fixture(fixture.to_vec());
    conform_toolchain(&toolchain, &fixture);
    exercised.insert("Toolchain".to_owned());

    // Every Phase-1 real row is PRESENT, named with what it is waiting for. What is asserted is
    // that the row names a DECLARED port -- the half that discriminates. The row's assertion list
    // is `&[]` in this file's own `const`, so a claim that it is empty is a compile-time tautology
    // no mutation of production code can reach; it is stated as the comment it always was.
    for (port, real, assertions) in PHASE_1_REAL_ROWS {
        assert!(
            ports.contains(port),
            "real row {real} names a port that is not declared: {port}"
        );
        // 7c fills these. Until then the row is present and empty, by construction.
        let _ = assertions;
    }

    let missing: BTreeSet<&String> = ports.difference(&exercised).collect();
    assert!(
        missing.is_empty(),
        "ports with no fake conformance row: {missing:?}"
    );
}
