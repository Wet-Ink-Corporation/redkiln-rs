//! AC-499's shared per-port conformance suite — the **bodies**, as library code behind
//! `[features] conformance = []` (RT-16 as re-amended, `s7c` §7c.1).
//!
//! **One parameterised body per port** (register :7136-7137), each taking the port as a **trait
//! object**, so the identical body runs against the fake row in this crate's own suite
//! (`tests/it/conformance.rs`, `port_conformance_fake_rows`) and against every real row Step 7c
//! fills in — SQLite, Claude, Node, Rust — from each adapter's own suite, over a **dev** edge that
//! names this feature. That is AC-499's THEN read literally: *"the same assertions execute against
//! both implementations and both pass"*. The bodies lived in the test binary until Step 7c Slice 1;
//! a body a real row cannot reach is a body it cannot run, and a copy is a second body.
//!
//! **Each body is a required core plus a capability arm** (the third review's amendment, `s7c`
//! §7c.13, §7c.14). The core is what every implementation of the port must do. The arm takes what
//! the row's registry declares **unsupported** — `snapshot` / `rebuild` on the store, `change_summary`
//! on the toolchain (DECISIONS-2026-09-17 D17-9: every real Phase-1 row answers those from a dated
//! registry entry with `Err(StepError::Unsupported { capability })`) — and asserts exactly that
//! error, **naming the token**, where the unchanged 7a body's `.expect("snapshot")` would have
//! panicked on every real row. The registry row *type* is each adapter's own (RT-24: four
//! `registry.rs` files, one shape); what crosses into this crate is the list of tokens, because a
//! shared registry type would be a type-design decision no record homes. **What that split is
//! and is not**, stated once here rather than re-argued per body: a declared-unsupported
//! capability answered as `Err(Unsupported { capability })` is *expected*, and any other assertion
//! that passes on the fake and fails on a real row surfaces as an assertion failure naming the
//! assertion — AC-499's AND — but reading the former as *not* a divergence is D17-9's decision
//! about the adapters' return value, not the register's word, and **a dated register amendment is
//! OWED** (`s7c` §7c.22).
//!
//! *(Noted 2026-09-25, s8a fix round 2, ADR-0165: "every real Phase-1 row answers those" is no
//! longer true of `change_summary` on the Rust row. That adapter answers `Ok(ChangeSummary::Text)`
//! over a non-empty range that changes only text, and refuses by name through its re-dated
//! `Probe::None` row only for an empty, unresolvable or unopenable range and for a range carrying
//! a `Binary` or LFS pointer path. The Rust row still hands this body `change_summary` as an
//! unsupported token, because the re-dated row keeps `Probe::None` (§8a.2), and the arm below
//! passes on that row because its fixture project is NOT a git repository: the refusal it
//! certifies is the no-repository path (`RangeError::Open`), not an absent capability. The `Text`
//! arm is certified by the adapter's own `change_summary::*` tests, not by this body. Giving the
//! shared body a `Text` assertion needs an arm kind the registry's token list cannot express, and
//! that is left to the stage that next moves AC-499's rows.)*
//!
//! **The Harness body is parameterised by a spec, not a registry** (RT-16 as amended again): the
//! fake row passes [`spec`], and the live probe (`s7c` §7c.15) passes its bound probe-card spec and
//! calls this body **before** its own prints. [`conform_harness`] prints `PROBE_RAN_CONFORMANCE=1`
//! **itself, as its last statement**, so the line the probe's block greps for is evidence that this
//! body ran to its end and not a token the probe printed about itself (a probe that never called
//! the body could otherwise satisfy the grep — the third review).
//!
//! **What the Harness core does NOT assert at Slice 1, named rather than smoothed:** it does not
//! count `dispatch_completed` rows. ADR-0126 obliges the Harness *adapter* to append exactly one
//! per termination path, and `ac360_one_event_per_termination_path` (`rk-adapter-claude`, Slice 5)
//! is that instrument over the real `dispatch`; the scripted fake appends none — it is a fake of the
//! port, not of the adapter's record-keeping — and a fake that did would move every kernel test
//! that counts journal rows. The core keeps 7a's observation on the error arm (the store grew) and
//! adds the record-shape checks a fake and a real row must both satisfy.
//!
//! `spec()` and `findings()` travel with the bodies (RT-16's *"and their `spec()`/`findings()`
//! helpers"*), so a real row's crate needs nothing from this crate's test binary. The test binary's
//! `support.rs` keeps its own copies for the tests that do not enable this feature.

use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::docset::DocSet;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::{SchemaId, Typed};
use rk_types::usage::CliVersion;

use crate::docs_site::DocsSite;
use crate::event_store::{EventStore, Query, Snapshot};
use crate::harness::Harness;
use crate::toolchain::{Classification, Selection, StepBound, TestOutcome, TestStatus, Toolchain};
use crate::tracker::Tracker;
use rk_types::verb_output::BuildStatus;
use rk_types::verify::BasisReason;

/// The run key every body queries by.
///
/// The fake's `query` ignores the selector (its `Event` carries no run id yet — the typed-payload
/// record is OWED, `s7a` §7a.5; ADR-0126 decision 6 leaves it so), so this is the same key on both
/// sides of a dispatch and the count is what the assertion turns on.
pub const RUN: &str = "run-1";

/// A dispatchable spec. The digest is built from data, as `Digest`'s only constructor requires —
/// there is deliberately no way to mint one.
#[must_use]
pub fn spec() -> ActivitySpec {
    ActivitySpec {
        card_digest: Digest::from_hex(&"a".repeat(64)).expect("64 lowercase hex"),
        tools: vec!["StructuredOutput".to_owned()],
        schema: SchemaId::new("findings"),
        timeout_secs: 5,
        inputs: Vec::new(),
    }
}

/// A typed result carrying the schema id it validated against.
#[must_use]
pub fn findings(value: serde_json::Value) -> Typed {
    Typed::new(SchemaId::new("findings"), value)
}

/// The capability arm's one assertion: the call answered `Err(StepError::Unsupported)` **naming
/// the token the row's registry declared** — not some other error, not `Ok`, and not a different
/// token (D17-9; AC-238's *"a dated registry entry"*).
fn assert_unsupported<T: std::fmt::Debug>(
    what: &str,
    result: Result<T, StepError>,
    capability: &str,
) {
    match result {
        Err(StepError::Unsupported { capability: named }) if named == capability => {}
        other => panic!(
            "the row declares `{capability}` unsupported, so `{what}` must answer \
             Err(StepError::Unsupported {{ capability: {capability:?} }}) from its dated registry \
             entry (D17-9; AC-238) — it answered {other:?}"
        ),
    }
}

// ---------------------------------------------------------------- one body per port

/// `EventStore`: what was appended comes back (the **core**); and rebuilding from a snapshot of the
/// current state leaves it unchanged (the **capability arm**, over design:265's other two methods).
///
/// `unsupported` is the list of capability tokens the row's registry declares unsupported —
/// `"snapshot"` and/or `"rebuild"`. For each token named, the call must answer
/// `Err(Unsupported { capability })` with that token; a token not named takes the `Ok` path. The
/// fake row passes `&[]`; the SQLite real row passes both (D17-9).
pub fn conform_event_store(store: &mut dyn EventStore, unsupported: &[&str]) {
    // ---- the core: append, then query, is identity on what was appended
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

    // ---- the capability arm: snapshot / rebuild, per the row's declaration
    let snapshot_unsupported = unsupported.contains(&"snapshot");
    let rebuild_unsupported = unsupported.contains(&"rebuild");
    if snapshot_unsupported {
        assert_unsupported("snapshot", store.snapshot(), "snapshot");
    }
    if rebuild_unsupported {
        // A `Snapshot` is this crate's opaque newtype, so the body can hand the row one to refuse.
        assert_unsupported("rebuild", store.rebuild(&Snapshot(())), "rebuild");
    }
    if !snapshot_unsupported && !rebuild_unsupported {
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
    // Declaring one of the pair without the other is a registry the body cannot exercise as a
    // round trip; it is stated, not silently accepted as the Ok path.
    assert!(
        snapshot_unsupported == rebuild_unsupported,
        "the row declares one of snapshot/rebuild unsupported and not the other ({unsupported:?}): \
         the round-trip half of this body cannot run and the registry should say which it means"
    );
}

/// `Harness`: `dispatch` blocks to completion and hands back **both** halves — the record on every
/// termination (AC-360) and the step's own outcome as a decided value — for the spec the row
/// supplies.
///
/// What the body can assert about the record **from the port alone**: the instrument version (the
/// pin AC-360's THEN names) is present, and a `cli_version` written as `Pinned(v)` carries the pin
/// itself — ADR-0126 decision 1 defines `Pinned(version)` as *"the `Config` pin,
/// `instrument_version`'s value"*, so a `Pinned` arm that disagrees with `instrument_version` is a
/// row that wrote something other than the pin under the pin's name. The `Observed` arm is not
/// checkable here (the body has no `system/init` to compare against) and is Slice 5's
/// `ac227_…` / `ac385_…` over the real adapter; that AC-227's member is non-optional is by
/// construction of the type, which no runtime assertion adds to (the first form of this body
/// asserted `!as_str().is_empty()`, which discriminated nothing — corrected 2026-09-17).
///
/// **Prints `PROBE_RAN_CONFORMANCE=1` as its last statement** — see the module doc. The fake row
/// prints it too, harmlessly; the live probe's block reads it as proof this body ran to its end.
pub fn conform_harness(harness: &dyn Harness, store: &mut dyn EventStore, spec: &ActivitySpec) {
    let before = store
        .query(Query::ByRun(RUN.to_owned()))
        .expect("query")
        .len();
    let outcome = harness.dispatch(spec, store);

    assert!(
        !outcome.record.instrument_version.is_empty(),
        "every dispatch produces a record carrying the instrument version (AC-360; Halt 2)"
    );
    match &outcome.record.cli_version {
        CliVersion::Pinned(v) => assert_eq!(
            v, &outcome.record.instrument_version,
            "a `Pinned` version IS the Config pin (ADR-0126 decision 1): the row wrote {v:?} under \
             the pin's name against pin {:?}",
            outcome.record.instrument_version
        ),
        // Whether an `Observed` value is the one the child's `system/init` reported is Slice 5's
        // assertion over the real adapter (AC-227, AC-385); this body has no init to compare to.
        CliVersion::Observed(_) => {}
    }
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
            // carry that sentence in prose and nothing that could fail — and it never ran at
            // all, because the matrix scripted only a successful row [measured: a `panic!` as its
            // first statement left `port_conformance_fake_rows` green]. The matrix runs this
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
    println!("PROBE_RAN_CONFORMANCE=1");
}

/// `Tracker`: `project` is called twice on one item, `inbox` is readable, and projecting does not
/// enqueue inbound work.
///
/// **What this body can and cannot assert about idempotency, stated rather than implied** (golden
/// constraint 10). design:267 fixes *"idempotent writes"*, and the equality below is the shape of
/// that claim — but it is **ARMED, NOT ASSERTED**: `ExternalRef` is `pub struct
/// ExternalRef(pub(crate) ())` while its contents are OWED (no record assigns them; AC-508's
/// idempotency key is Phase 3), so every value of it is equal to every other and the assertion
/// holds by construction whatever `project` does [measured at 7a: removing `BoardTracker`'s dedup
/// guard left this assertion passing]. It becomes live the moment that record lands and the
/// newtype gains contents, which is why it is written now rather than added then.
///
/// **The discriminating observation is the fake's own board window** (`tracker.board()` in the
/// matrix), which is fake-specific and does NOT carry to a real row. What this body carries
/// instead is the port-visible invariant under it: a `project` is an outbound write and never
/// enqueues inbound work, so `inbox` is unchanged across both calls. `Tracker` has a fake only
/// until Phase 3 (ADR-0094 decision 2) and no real row runs this body in Phase 1.
pub fn conform_tracker(tracker: &dyn Tracker) {
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
/// [measured at 7a: `assert!(false)` as its body left `port_conformance_fake_rows` green].
/// `expect_drift` is what makes the vacuity a failure instead of a silence — a real row that has
/// no drift to report passes `0` and says so. `DocsSite` has a fake only until Phase 3.
pub fn conform_docs_site(docs: &dyn DocsSite, expect_drift: usize) {
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

/// **AC-329 as a port conformance row** (record T = ADR-0169 decision 9, proposed).
///
/// What a `TestOutcome` must satisfy whoever produced it: its report's own shape
/// (`VerifyReport::validate_shape`), the count invariant `passed + failed ==
/// report.selected_test_count`, a `status` that agrees with the counts ([`TestStatus::of`]), and a
/// runner that names itself. It does NOT duplicate the store's whole-suite rule:
/// `rk_store::artifacts::verify_report::validate_report` stays the boundary validator (ADR-0104
/// decision 3), and `rk-ports` cannot depend on `rk-store`.
///
/// *(Added 2026-09-26, s8c §8c.2.)*
///
/// # Errors
///
/// A message naming the rule the outcome breaks.
pub fn validate_report(outcome: &TestOutcome) -> Result<(), String> {
    outcome
        .report
        .validate_shape()
        .map_err(|e| format!("the outcome's report fails its own shape: {e}"))?;
    let executed = outcome.passed.checked_add(outcome.failed);
    if executed != Some(outcome.report.selected_test_count) {
        return Err(format!(
            "passed + failed ({} + {}) is not the report's selected_test_count ({}) (record T \
             decision 1's invariant)",
            outcome.passed, outcome.failed, outcome.report.selected_test_count
        ));
    }
    let derived = TestStatus::of(outcome.passed, outcome.failed);
    if outcome.status != derived {
        return Err(format!(
            "status {:?} disagrees with the counts ({} passed, {} failed), which fix {derived:?}: \
             status is derived, never stored independently",
            outcome.status, outcome.passed, outcome.failed
        ));
    }
    if outcome.runner.name.trim().is_empty() {
        return Err(
            "the outcome's runner has an empty name (D21-48 X15: the report records which \
                    runner ran)"
                .to_owned(),
        );
    }
    Ok(())
}

/// `Toolchain`: a required core over five of design:269's methods, and a capability arm over the
/// sixth.
///
/// *(Amended 2026-09-26, s8c §8c.2: `test` is called with `Selection::WholeSuite` and returns a
/// `TestOutcome`, whose `passed` is read against the row's count and which must pass
/// [`validate_report`]; `build` returns a `BuildReport`, which must read `passing` over the row's
/// green project. Every step-reached method is handed `StepBound::Undeclared`. `run_verb`, the
/// seventh method, is not in this body: which verbs a row implements is the row's, and each
/// adapter's own `verbs::` tests assert them.)*
///
/// The **core** is `detect` + `classify` over the row's `(path, classification)` pairs plus
/// `build` / `test` / `lint` over the row's project **with a count assertion the row
/// parameterises**; the **capability arm** is `change_summary`, per the row's registry.
///
/// **The row supplies its own `(path, classification)` expectations, and that is what makes this
/// body able to fail.** The 7a form before it asserted `matches!(c, Text | Binary | Generated)`,
/// which is **irrefutable over a three-variant enum**: a `classify` that ignored its input and
/// answered `Binary` for everything satisfied it, and so did one that answered `Text` for
/// everything [both measured — `port_conformance_fake_rows` stayed green in both directions, and
/// only `toolchain_fake_classifies_binary`, outside this body, caught them]. The real rows pass
/// their own real paths.
///
/// `expected_test_count` is what `test(&[])` — the unselected run — must report: `2` for the
/// fixture projects (`s7c` §7c.14: two passing tests, the decoy among them; the failing and the
/// slow test are reachable only by selection), `0` for the fake. An adapter whose `build` / `test`
/// / `lint` return `Ok` without spawning anything is red on the count. `unsupported` names the
/// capability tokens the row's registry declares unsupported — `"change_summary"` on every real
/// Phase-1 row (D17-9; S-BIN unrun) — and the fake passes `&[]`. *(Noted 2026-09-25, s8a fix
/// round 2, ADR-0165: on the Rust row the token now marks a partial refusal, not an absent
/// capability, and the arm passes there only through the no-repository path; the module doc says
/// why.)*
pub fn conform_toolchain(
    toolchain: &dyn Toolchain,
    expected: &[(&str, Classification)],
    expected_test_count: u32,
    unsupported: &[&str],
) {
    // ---- the core
    assert!(
        toolchain.detect().expect("detect"),
        "a conformance row runs over a project whose toolchain is PRESENT: `detect` answered false"
    );
    let built = toolchain.build(StepBound::Undeclared).expect("build");
    assert_eq!(
        built.status,
        BuildStatus::Passing,
        "a conformance row's project builds green: {built:?}"
    );
    let outcome = toolchain
        .test(
            &Selection::WholeSuite {
                reason: BasisReason::ProbeUnverified,
            },
            StepBound::Undeclared,
        )
        .expect("test(selection)");
    validate_report(&outcome).unwrap_or_else(|e| {
        panic!("the row's TestOutcome fails validate_report (AC-329 as a conformance row): {e}")
    });
    assert_eq!(
        outcome.passed, expected_test_count,
        "the unselected whole-suite `test` must report the passed count the row declares \
         ({expected_test_count}); it reported {}",
        outcome.passed
    );
    assert_eq!(
        outcome.failed, 0,
        "a conformance row's project is green: {outcome:?}"
    );
    toolchain.lint(StepBound::Undeclared).expect("lint");

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

    // ---- the capability arm
    if unsupported.contains(&"change_summary") {
        assert_unsupported(
            "change_summary",
            toolchain.change_summary("HEAD~1..HEAD", StepBound::Undeclared),
            "change_summary",
        );
    } else {
        toolchain
            .change_summary("HEAD~1..HEAD", StepBound::Undeclared)
            .expect("change_summary(range)");
    }
}
