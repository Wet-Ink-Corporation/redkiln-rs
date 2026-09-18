//! AC-499's fake half — the shared per-port conformance suite, over the **library** bodies.
//!
//! **One parameterised body per port** (register :7136-7137), each taking the port as a **trait
//! object** so the identical body runs against a real adapter when 7c fills the real rows in. Since
//! Step 7c Slice 1 the bodies are `rk_ports::conformance`'s — library code behind
//! `[features] conformance = []` (RT-16 as re-amended, `s7c` §7c.1) — and this module is
//! `#[cfg(feature = "conformance")]` for the same reason: the fake rows here and the real rows in
//! each adapter's suite run **the same bodies**, and this file holds only the matrix. It runs as
//! `cargo test -p rk-ports --features conformance -- port_conformance_fake_rows`; without the flag
//! the module is not compiled and the filter selects nothing.
//!
//! The fake row of all five ports is instantiated here; each Phase-1 real row is present as a
//! **named** row rather than absent — SQLite, Claude, Node, Rust (register :7139-7140) — empty
//! until the slice that lands its adapter fills it (`s7c` §7c.20), so that what is owed is visible
//! in the suite rather than only in a document. SQLite was filled at Slice 3 (2026-09-18); the
//! other three are empty, named as empty, until Slices 5 and 6.
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

use rk_ports::conformance::{
    conform_docs_site, conform_event_store, conform_harness, conform_toolchain, conform_tracker,
    findings, spec,
};
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::toolchain::Classification;
use rk_types::drift::Drift;
use rk_types::error::StepError;

use crate::source_parse::Source;

/// The four `{fake, real}` rows Phase 1 ships (ADR-0094; register :7139-7140). The third element
/// is the row's assertion list, **filled by the slice that lands its adapter** (`s7c` §7c.20
/// items 3-6; Slice 1 landed the bodies and filled none) and **empty, named as empty, until
/// then**. A filled row names the real-row test in the adapter's own suite — which runs the SAME
/// body over the real adapter, through its dev edge on this crate's `conformance` feature — and
/// what that run asserts beyond the body.
///
/// **SQLite (Slice 3, 2026-09-18)**: `port_conformance_real_row_sqlite` in `rk-adapter-sqlite`
/// runs `conform_event_store` with the capability arm fed from the adapter's registry — the
/// `snapshot` and `rebuild` rows D17-9 names, so the arm asserts `Err(Unsupported { capability })`
/// naming each — and `wal_and_busy_timeout_are_enforced` adds the two ADR-0105 clause 7 settings
/// the fake cannot carry.
///
/// **Claude (Slice 5, 2026-09-18)** — the register's *"local-only, named-runner"* row (D17-5;
/// `s7c` §7c.15): `port_conformance_real_row_claude` in `rk-adapter-claude` runs `conform_harness`
/// over the REAL `ClaudeHarness` with the fixture child (`rk-fixture-child-claude`, spawned through
/// `rk-containment`) as the program — NOT a real `claude` — on both arms the body covers (a replayed
/// `Ok` transcript; a child that exits 3), so the body's `PROBE_RAN_CONFORMANCE=1` is printed by a
/// run that reached its end. The live probe (`live_dispatch_probe`, `#[ignore]`, `--features live`)
/// runs the same body over the real CLI before its own prints and is the fifteenth line's, never
/// CI's; `ac360_one_event_per_termination_path` is the adapter's own count of one
/// `dispatch_completed` per path, which this body does not assert (the module doc). **Never wired
/// into CI as a matrix row** (that instrument is ARMED, `s7c` §7c.22).
const PHASE_1_REAL_ROWS: [(&str, &str, &[&str]); 4] = [
    (
        "EventStore",
        "SQLite",
        &[
            "port_conformance_real_row_sqlite: conform_event_store core (append then query is identity)",
            "port_conformance_real_row_sqlite: snapshot -> Err(Unsupported { capability: \"snapshot\" }) from the dated registry row",
            "port_conformance_real_row_sqlite: rebuild -> Err(Unsupported { capability: \"rebuild\" }) from the dated registry row",
            "wal_and_busy_timeout_are_enforced: journal_mode = wal",
            "wal_and_busy_timeout_are_enforced: busy_timeout = 5000",
        ],
    ),
    (
        "Harness",
        "Claude",
        &[
            "port_conformance_real_row_claude: conform_harness core over the real ClaudeHarness and the fixture child (Ok arm: instrument_version present, a Pinned arm equals the pin, Typed carries its schema id)",
            "port_conformance_real_row_claude: conform_harness error arm (exit 3 -> NonZero): the store grew by the dispatch_completed row (AC-360's shape)",
            "port_conformance_real_row_claude: exactly one fixture-child spawn per body run (the marker count)",
            "live_dispatch_probe: the same body over the real CLI, #[ignore] behind --features live, local-only (s7c section 7c.15; never CI)",
        ],
    ),
    ("Toolchain", "Node", &[]),
    ("Toolchain", "Rust", &[]),
];

// ---------------------------------------------------------------- the matrix

#[test]
fn port_conformance_fake_rows() {
    // The expected port set comes from the SAME source parse AC-481's parity count uses, not from
    // a list in this file: a port added without a conformance row is then caught here too.
    let ports = Source::parse().traits;
    let mut exercised: BTreeSet<String> = BTreeSet::new();

    // --- EventStore / fake: the in-memory Vec, which supports snapshot/rebuild (as identity), so
    // its capability arm is the Ok path and the row declares nothing unsupported.
    let mut store = VecStore::default();
    conform_event_store(&mut store, &[]);
    exercised.insert("EventStore".to_owned());

    // --- Harness / fake: the scripted responder, run over BOTH of the body's arms. One row
    // scripting only success leaves the Err arm -- where AC-360's event assertion lives -- dead
    // [measured: a `panic!` as its first statement left this test green].
    let harness = FakeHarness::scripted(vec![Ok(findings(serde_json::json!({"ok": true})))]);
    let mut harness_store = VecStore::default();
    conform_harness(&harness, &mut harness_store, &spec());
    let refusing = FakeHarness::scripted(vec![Err(StepError::Unsupported {
        capability: "affected-selection".to_owned(),
    })]);
    let mut refused_store = VecStore::default();
    conform_harness(&refusing, &mut refused_store, &spec());
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
    // The fake's unselected `test(&[])` runs nothing and says so (0); it implements
    // `change_summary` as the opaque newtype, so its capability arm is the Ok path.
    let toolchain = FixtureToolchain::from_fixture(fixture.to_vec());
    conform_toolchain(&toolchain, &fixture, 0, &[]);
    exercised.insert("Toolchain".to_owned());

    // Every Phase-1 real row is PRESENT, named with what it is waiting for or what it runs. What
    // is asserted is that the row names a DECLARED port -- the half that discriminates -- and,
    // for a filled row, that every assertion it lists names the real-row test that carries it (a
    // list entry is `<test name>: <what>`), so the list can be read against the adapter's suite by
    // name. The list's contents are this file's own `const`, so whether a row is filled is a
    // compile-time fact no mutation of production code can reach; the runs themselves are the
    // adapters' (Checkpoint 7c's `port_conformance_real_row_*` lines).
    for (port, real, assertions) in PHASE_1_REAL_ROWS {
        assert!(
            ports.contains(port),
            "real row {real} names a port that is not declared: {port}"
        );
        for assertion in assertions {
            assert!(
                assertion.split_once(": ").is_some(),
                "real row {real}: assertion {assertion:?} does not name its test as `<test>: <what>`"
            );
        }
    }

    let missing: BTreeSet<&String> = ports.difference(&exercised).collect();
    assert!(
        missing.is_empty(),
        "ports with no fake conformance row: {missing:?}"
    );
}
