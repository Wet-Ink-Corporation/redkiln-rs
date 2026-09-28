//! AC-497's and AC-500's fake-side instruments — the two a grep over a type declaration cannot
//! see — **plus the two F41 unit tests this step re-homes.**
//!
//! `s7a` §7a.3 gives exactly two dispositions for the three F41 tests that construct the spike's
//! own `kernel::Kernel` (a module `rk-ports` may not contain): *"Re-home each against
//! `Harness::dispatch` directly, calling the port on a fake instead of through a kernel, or carry
//! it to Checkpoint 7b and record the deferral."* `cancel_token_is_the_roots_and_yields_cancelled`
//! (spike lib.rs:287) and `step_error_is_matched_exhaustively_with_one_event_arm_each` (:299) are
//! **re-homed here, against `Harness::dispatch` on the scripted fake**, which needs no kernel.
//! `dispatch_blocks_and_returns_record_and_result` (:276) is the body of
//! `conformance::conform_harness` and is not duplicated.

use rk_ports::cancel::CancelToken;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::fakes::{FakeHarness, FixtureToolchain, VecStore};
use rk_ports::harness::Harness;
use rk_ports::toolchain::{ChangeKind, ChangeSummary, Classification, StepBound, Toolchain};
use rk_types::error::{CancelledBy, StepError, TransportKind};
use rk_types::event_kind::EventKind;

use crate::support::{findings, spec};

/// AC-497's first named instrument (register :7117): *"a Toolchain fake driven by a binary
/// fixture"*.
///
/// **What this test can and cannot discharge, stated rather than assumed** (golden constraint 10).
/// AC-497's THEN has two clauses: the reviewer receives a typed `ChangeSummary`, **and** no
/// textual diff of the binary path is assembled or dispatched. The first is assertable here. The
/// second is **not**, and not because it is hard: nothing in `rk-ports` assembles a diff, so an
/// assertion here that no diff was assembled would be an absence over an empty universe — Halt 8
/// shape (g). That half is AC-497's *input-assembly* test, whose WHEN is *"the review activity's
/// inputs are assembled"* — the kernel's, at **7b** — and `s7a` §7a.11 routes it there. It is
/// reported as **ARMED, NOT DISCHARGED** and is not claimed by this test.
#[test]
fn toolchain_fake_classifies_binary() {
    let toolchain = FixtureToolchain::from_fixture(vec![
        ("assets/logo.png", Classification::Binary),
        ("src/lib.rs", Classification::Text),
        ("target/schema.json", Classification::Generated),
    ]);

    assert_eq!(
        toolchain.classify("assets/logo.png").expect("classify"),
        Classification::Binary,
        "the fixture-driven fake answers Binary for the binary path it was scripted with"
    );

    // The fake is driven by the fixture and is not a constant: the same call on the other two
    // scripted paths answers differently. Without this, a `classify` that returned `Binary`
    // unconditionally would pass the assertion above.
    assert_eq!(
        toolchain.classify("src/lib.rs").expect("classify"),
        Classification::Text
    );
    assert_eq!(
        toolchain.classify("target/schema.json").expect("classify"),
        Classification::Generated
    );

    // What the reviewer receives INSTEAD of a diff (AC-497's port shape). *(amended 2026-09-24,
    // s8a, ADR-0165: this asserted `format!("{summary:?}") == "ChangeSummary(())"` over the opaque
    // newtype. `ChangeSummary` is now an enum whose `Text` arm is finalised and whose `Binary` arm
    // is sealed until S-BIN, so the fake is asserted on both: a range carrying only text and
    // generated paths yields `Text` with one manifest row per path, and a range carrying a
    // `Binary` path refuses BY NAME -- never an empty `Text`, and never a `Text` that silently
    // omits the path, AC-497.)*
    let toolchain = toolchain
        .with_range(
            "HEAD~1..HEAD",
            vec![
                ("src/lib.rs", ChangeKind::Modified),
                ("target/schema.json", ChangeKind::Added),
            ],
        )
        .with_range(
            "HEAD~2..HEAD~1",
            vec![
                ("src/lib.rs", ChangeKind::Modified),
                ("assets/logo.png", ChangeKind::Added),
            ],
        );

    match toolchain.change_summary("HEAD~1..HEAD", StepBound::Undeclared) {
        Ok(ChangeSummary::Text(text)) => {
            let rows: Vec<(&str, Classification, ChangeKind)> = text
                .manifest
                .iter()
                .map(|m| (m.path.as_str(), m.classification, m.change))
                .collect();
            assert_eq!(
                rows,
                vec![
                    ("src/lib.rs", Classification::Text, ChangeKind::Modified),
                    (
                        "target/schema.json",
                        Classification::Generated,
                        ChangeKind::Added
                    ),
                ],
                "a text range yields Text with one manifest row per scripted path, each carrying \
                 the fixture's classification and the scripted change kind"
            );
        }
        other => {
            panic!("a range carrying only text and generated paths yields Text; got {other:?}")
        }
    }

    match toolchain.change_summary("HEAD~2..HEAD~1", StepBound::Undeclared) {
        Err(StepError::Unsupported { capability }) => {
            assert!(
                capability.starts_with("change_summary") && capability.contains("assets/logo.png"),
                "a range carrying a Binary path refuses BY NAME -- the capability token and the \
                 path: {capability:?}"
            );
        }
        other => panic!(
            "a range carrying a Binary path refuses by name (the Binary arm is sealed until \
             S-BIN); it must never yield a Text that omits the path. Got {other:?}"
        ),
    }

    // An EMPTY range, and one nobody scripted, refuse by name: ADR-0165 forbids *"returning an
    // empty `Text`, or any `Ok`, for an empty or unresolvable range"*, and the fake is held to it
    // as the Rust adapter is. *(Added 2026-09-25, s8a fix round 3: both answered an empty `Text`,
    // so the kernel's `change_summary("")` was an `Ok` on every walk binding this fake.)* The
    // empty range is SCRIPTED here, so it refuses for being empty and not merely for being
    // unscripted.
    // A range SCRIPTED with no paths changes nothing, and refuses by name as the Rust adapter
    // refuses a change-less range. *(Added 2026-09-25, s8a fix round 4: it answered `Ok(Text)`
    // with an empty manifest, the empty `Text` ADR-0165 decision 6 forbids.)*
    let toolchain = toolchain
        .with_range("", vec![("src/lib.rs", ChangeKind::Modified)])
        .with_range("HEAD~3..HEAD~2", vec![]);
    for range in ["", "  ", "HEAD~9..HEAD~8", "HEAD~3..HEAD~2"] {
        match toolchain.change_summary(range, StepBound::Undeclared) {
            Err(StepError::Unsupported { capability }) => assert!(
                capability.starts_with("change_summary")
                    && capability.contains(&format!("`{range}`")),
                "an empty or unscripted range refuses BY NAME -- the capability token and the \
                 range: {capability:?}"
            ),
            other => panic!(
                "an empty or unscripted range `{range}` refuses by name, never an empty Text; got \
                 {other:?}"
            ),
        }
    }
}

/// AC-500's first clause (register :7149's *"an integration test on the fake"*): the scripted
/// responder produces `StepError::Unsupported(capability)` **carrying the token** when a test
/// scripts a capability as absent.
///
/// **The route-around half is not here and is not claimed.** *"the pack's route-around path
/// executes under test"* and the mutation-style check that deleting that branch turns the test red
/// both need a pack, which is 7b/7e's. Reported as ARMED, NOT DISCHARGED.
#[test]
fn harness_fake_produces_unsupported() {
    let capability = "affected-selection";
    let harness = FakeHarness::scripted(vec![Err(StepError::Unsupported {
        capability: capability.to_owned(),
    })]);
    let mut store = VecStore::default();

    let outcome = harness.dispatch(&spec(), &mut store);

    match outcome.result {
        Err(StepError::Unsupported { capability: token }) => assert_eq!(
            token, capability,
            "Unsupported carries the capability TOKEN, never a generic error (design §3.6 P7)"
        ),
        other => panic!("the scripted responder did not produce Unsupported: {other:?}"),
    }

    // A missing capability is a REFUSAL, not a failure: one event arm, and it carries the token
    // (AC-360; `rk-types`' own `event()`). A refusal is a configuration state, never a judgement.
    //
    // *(2026-09-26, `s8d` build, package P6; R4 = ADR-0176 decision 2, proposed.)* The arm is
    // `rk-types`' map, read here directly; the ROW is the kernel's (`rk_kernel::runtime::activity::
    // error_row`, pinned by `rk-kernel`'s `step_error_row::`). This test asserted that the fake
    // appended the `step_refused` row itself; after R4 the fake appends NOTHING, and a fake that did
    // would put a second row beside the kernel's (mutant M11).
    let error = StepError::Unsupported {
        capability: capability.to_owned(),
    };
    match error.event() {
        rk_types::event::ErrorEvent::StepRefused {
            capability: Some(token),
            ..
        } => assert_eq!(
            token, capability,
            "the refusal arm carries the capability token too"
        ),
        other => panic!("Unsupported maps to step_refused carrying the token, got {other:?}"),
    }
    //
    // *(dated 2026-09-26, `s8d` build, package F1-D2172; D21-72 option (B), R4 = ADR-0176, proposed.)*
    // ~~"the fake appends NOTHING"~~ is re-amended to **"no error row"**: on an `Err` termination
    // the fake appends exactly ADR-0126's `dispatch_completed` (the serde of its record), as the
    // real adapter does, and no `step_*` / `rate_limited` row.
    let events = store
        .query(rk_ports::event_store::Query::ByRun("r".to_owned()))
        .expect("query");
    assert_only_dispatch_completed(&events, &outcome.record, "Unsupported");
}

/// *(dated 2026-09-26, `s8d` build, package F1-D2172; D21-72 option (B).)* A failed or cancelled
/// fake dispatch leaves **no error row** (R4, ADR-0176: the kernel writes it) and **exactly one**
/// ADR-0126 `dispatch_completed` row, whose payload is the serde of the record the dispatch
/// returned.
fn assert_only_dispatch_completed(
    events: &[rk_types::event::Event],
    record: &rk_types::activity::DispatchRecord,
    what: &str,
) {
    let error_rows: Vec<_> = events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::StepFailed
                    | EventKind::StepCancelled
                    | EventKind::StepRefused
                    | EventKind::RateLimited
            )
        })
        .collect();
    assert!(
        error_rows.is_empty(),
        "{what}: the fake appends no error row of its own (R4): {error_rows:?}"
    );
    assert_eq!(
        events.len(),
        1,
        "{what}: a failed fake dispatch appends exactly ONE row, ADR-0126's `dispatch_completed` \
         (D21-72 (B)): {events:?}"
    );
    assert_eq!(events[0].kind, EventKind::DispatchCompleted, "{what}");
    assert_eq!(
        events[0].payload,
        serde_json::to_value(record).expect("a DispatchRecord serialises"),
        "{what}: the row's payload is the serde of the record the dispatch returned"
    );
}

/// F41's `cancel_token_is_the_roots_and_yields_cancelled` (spike `lib.rs`:287), **re-homed against
/// `Harness::dispatch`** — the disposition `s7a` §7a.3 requires for a spike test that constructed
/// `kernel::Kernel`. The spike drove the token through its own kernel; nothing in `rk-ports` may.
///
/// **The token is the composition ROOT's** (ADR-0099 clause 1, :50-55): it is cloned into the
/// adapter at construction, fired only by the root's signal path or the per-activity timeout, and
/// **never by the kernel** — which is why `with_token` is a constructor and not a port method, and
/// why nothing on the `Harness` trait can reach it. ADR-0076's withdrawn *"a handle minted when the
/// operation started"* shape is rejected at ADR-0099:95-96.
///
/// Without this test the seam design:1704 names as this crate's content had **zero** coverage at
/// any level: `with_token`, `cancel()` and `is_cancelled()` appeared in no test, and the cancel
/// branch of `FakeHarness::dispatch` never executed [measured: a `panic!` in that branch left all
/// nine tests green]. Checkpoint 7a asserts only that `CancelToken` is DECLARED.
#[test]
fn cancel_token_is_the_roots_and_yields_cancelled() {
    let token = CancelToken::new();
    let harness = FakeHarness::scripted(vec![
        Ok(findings(serde_json::json!({"ok": true}))),
        Ok(findings(serde_json::json!({"ok": true}))),
    ])
    .with_token(token.clone());
    let mut store = VecStore::default();

    // BEFORE the root fires it: the token is read at dispatch time, not at construction, so the
    // scripted outcome comes back. Without this half a `with_token` that dropped its argument, or
    // a fake that was simply always cancelled, would satisfy the assertion below.
    let uncancelled = harness.dispatch(&spec(), &mut store);
    assert!(
        uncancelled.result.is_ok(),
        "an unfired token must not cancel a dispatch: {:?}",
        uncancelled.result
    );
    assert_eq!(
        store.events().len(),
        0,
        "a successful dispatch appends nothing: no error row (R4) and, on the fake, no \
         `dispatch_completed` either (D21-72 (B): no `Ok` path appends)"
    );

    // The ROOT fires it. There is deliberately no way to un-fire it.
    token.cancel();
    assert!(token.is_cancelled(), "cancel() is one-way and observable");

    let cancelled = harness.dispatch(&spec(), &mut store);
    assert!(
        !cancelled.record.instrument_version.is_empty(),
        "the record is produced on EVERY termination, cancellation included (AC-360)"
    );
    match cancelled.result {
        Err(StepError::Cancelled {
            by: CancelledBy::Operator,
        }) => {}
        other => panic!("a fired token must yield Cancelled {{ by: Operator }}, got {other:?}"),
    }

    // Exactly ONE event arm, and it is `step_cancelled` (AC-360; `rk-types`' own total `event()`).
    //
    // *(2026-09-26, `s8d` build, package P6; R4 = ADR-0176 decision 2, proposed.)* The ROW is the
    // kernel's now, so the fake's store stays EMPTY on the cancel path too; the arm is read off
    // `rk-types`' own map.
    //
    // *(dated 2026-09-26, `s8d` build, package F1-D2172; D21-72 option (B).)* ~~the fake's store
    // stays EMPTY on the cancel path too~~ — re-amended to **"no error row"**: the cancel path
    // appends exactly ADR-0126's `dispatch_completed`, and the Ok dispatch above appended nothing.
    let events = store.query(Query::ByRun("r".to_owned())).expect("query");
    assert_only_dispatch_completed(&events, &cancelled.record, "the cancel path");
    assert_eq!(
        StepError::Cancelled {
            by: CancelledBy::Operator
        }
        .event(),
        rk_types::event::ErrorEvent::StepCancelled
    );
}

/// F41's `step_error_is_matched_exhaustively_with_one_event_arm_each` (spike `lib.rs`:299),
/// **re-homed against `Harness::dispatch`** on the same warrant as the test above.
///
/// **AC-360's shape, over every event family rather than over one of them** (register
/// :5192-5197): *"no dispatch path exists that can terminate without appending that event"*. One
/// error from each of `ErrorEvent`'s four families is scripted, and each must produce exactly one
/// event of its own kind. Three of the four arms of the fake's `event_for` bridge were dead in
/// this suite before this test — only `StepRefused` was ever constructed, and replacing the other
/// three with `panic!` left all nine tests green [measured].
///
/// **Exhaustiveness over the eleven `StepError` variants is `rk-types`' and stays there**:
/// `StepError::event()` is a total `match`, so a twelfth variant is `E0004` at the declaration and
/// not a silent gap here. What this test pins is that the fake's bridge preserves the mapping —
/// one arm per family, no family collapsed into another.
#[test]
fn step_error_is_matched_exhaustively_with_one_event_arm_each() {
    let families = [
        (
            StepError::Transport {
                kind: TransportKind::Spawn,
                detail: "spawn refused".to_owned(),
            },
            EventKind::StepFailed,
        ),
        (
            StepError::Cancelled {
                by: CancelledBy::Timeout,
            },
            EventKind::StepCancelled,
        ),
        (
            StepError::RateLimited { resets_at: None },
            EventKind::RateLimited,
        ),
        (StepError::Overage { reason: None }, EventKind::StepRefused),
    ];

    // *(2026-09-26, `s8d` build, package P6; R4 = ADR-0176 decision 2, proposed.)* The arm is read
    // off `rk-types`' own map, and the fake's store stays EMPTY: the one row per failed dispatch is
    // the KERNEL's (`rk-kernel`'s `step_error_row::` pins it through the dispatch path, AC-360's
    // "kernel test"). This test read the arm off a row the fake appended until R4 removed the
    // fake's `event_for` bridge.
    for (error, expected) in families {
        let harness = FakeHarness::scripted(vec![Err(error.clone())]);
        let mut store = VecStore::default();

        let outcome = harness.dispatch(&spec(), &mut store);

        assert_eq!(
            outcome.result,
            Err(error.clone()),
            "the scripted responder returns the error it was scripted with"
        );
        // *(dated 2026-09-26, `s8d` build, package F1-D2172; D21-72 option (B).)* ~~the fake's
        // store stays EMPTY~~ — re-amended to **"no error row"**, and exactly the
        // `dispatch_completed` row on `Err`.
        let events = store.query(Query::ByRun("r".to_owned())).expect("query");
        assert_only_dispatch_completed(&events, &outcome.record, &format!("{error:?}"));
        let arm = match error.event() {
            rk_types::event::ErrorEvent::StepFailed { .. } => EventKind::StepFailed,
            rk_types::event::ErrorEvent::StepCancelled => EventKind::StepCancelled,
            rk_types::event::ErrorEvent::RateLimited => EventKind::RateLimited,
            rk_types::event::ErrorEvent::StepRefused { .. } => EventKind::StepRefused,
        };
        assert_eq!(
            arm, expected,
            "{error:?} maps to {expected:?}, never to another family's arm"
        );
    }
}

// ---------------------------------------------------------------- s8c: outcomes, bounds, run_verb
//
// *(Added 2026-09-26, s8c §8c.2, §8c.4, §8c.6; records T, V and B = ADR-0169/0170/0171,
// proposed.)* `test` and `build` return OUTCOMES: a red suite and a red build are `Ok`, and `Err`
// means only cannot-run, killed or unreadable (D21-42 1a; D21-43 N25).

/// **A red suite is `Ok` with failed counts** (D21-42 1a: *"A red suite is `Ok` with counts;
/// `Err` is only cannot-run/timeout/killed"*). The fake scripts `failing(n)` over a selection.
#[test]
fn toolchain_fake_red_suite_is_ok_with_failed_counts() {
    use rk_ports::toolchain::{Selection, StepBound, TestStatus};
    let selection = Selection::Filter(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]);

    let green = FixtureToolchain::from_fixture(vec![]);
    let outcome = green
        .test(&selection, StepBound::Undeclared)
        .expect("a green fake suite is Ok");
    assert_eq!((outcome.passed, outcome.failed), (3, 0));
    assert_eq!(outcome.status, TestStatus::Passing);

    let red = FixtureToolchain::from_fixture(vec![]).failing(1);
    let outcome = red
        .test(&selection, StepBound::Undeclared)
        .expect("a RED suite is Ok with counts, never Err (D21-42 1a)");
    assert_eq!(
        (outcome.passed, outcome.failed),
        (2, 1),
        "failing(1) over three selected tests: two passed, one failed"
    );
    assert_eq!(outcome.status, TestStatus::Failing);
    assert_eq!(
        outcome.passed + outcome.failed,
        outcome.report.selected_test_count,
        "the invariant: passed + failed == the report's selected count"
    );
    assert_eq!(
        outcome.runner.name, "fixture",
        "the fake's runner names itself"
    );
    outcome
        .report
        .validate_shape()
        .expect("the fake's report members are non-empty fixture values");
}

/// **`status` is `passing` only with executed tests and no failures** (AC-332: *"'zero tests
/// executed' is never rendered as green"*). `status` is DERIVED from the counts, and its serde
/// form is the catalog's vocabulary.
#[test]
fn test_outcome_is_passing_only_with_executed_tests_and_no_failures() {
    use rk_ports::toolchain::{Selection, StepBound, TestOutcome, TestRunner, TestStatus};
    assert_eq!(
        TestStatus::of(0, 0),
        TestStatus::Empty,
        "nothing executed is EMPTY, never passing"
    );
    assert_eq!(TestStatus::of(1, 0), TestStatus::Passing);
    assert_eq!(TestStatus::of(0, 1), TestStatus::Failing);
    assert_eq!(TestStatus::of(4, 2), TestStatus::Failing);
    for (status, wire) in [
        (TestStatus::Passing, "passing"),
        (TestStatus::Failing, "failing"),
        (TestStatus::Empty, "empty"),
    ] {
        assert_eq!(
            serde_json::to_value(status).expect("serialises"),
            serde_json::json!(wire)
        );
    }

    // Through the fake: an empty selection executes nothing and is EMPTY.
    let fake = FixtureToolchain::from_fixture(vec![]);
    let empty = fake
        .test(&Selection::Filter(vec![]), StepBound::Undeclared)
        .expect("Ok");
    assert_eq!(
        empty.status,
        TestStatus::Empty,
        "an empty run is never passing"
    );
    let one = fake
        .test(
            &Selection::Filter(vec!["x".to_owned()]),
            StepBound::Undeclared,
        )
        .expect("Ok");
    assert_eq!(one.status, TestStatus::Passing);

    // The invariant is checked at construction: a count that disagrees with the report is
    // Transport { Decode } naming both numbers, never a corrected count.
    let report = one.report.clone();
    match TestOutcome::new(report, 5, 0, TestRunner::named("fixture", "0")) {
        Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        }) => assert!(
            detail.contains('5') && detail.contains('1'),
            "the refusal names both numbers: {detail}"
        ),
        other => {
            panic!("passed + failed != selected_test_count must refuse as Decode; got {other:?}")
        }
    }
    // The serialised outcome carries the derived status beside the counts.
    let wire = serde_json::to_value(&one).expect("serialises");
    assert_eq!(wire["status"], serde_json::json!("passing"));
    assert_eq!(wire["passed"], serde_json::json!(1));
}

/// **`validate_report` refuses an inconsistent outcome, each rule by name** (AC-329 as a port
/// conformance row, §8c.2; added 2026-09-26, s8c refuting round 2). `TestOutcome`'s members are
/// `pub`, so a value that `TestOutcome::new` would never build can still be assembled, and
/// `validate_report` is what re-checks it. The control passes first; then ONE member changes at a
/// time, and each change is refused with a message naming the rule it breaks: the count
/// invariant, the derived status, the runner's name and the report's own shape. Every other
/// caller expects `Ok`, so without this test a `validate_report` that always answers `Ok(())`
/// survives (mutant M38).
#[cfg(feature = "conformance")]
#[test]
fn validate_report_refuses_an_inconsistent_outcome() {
    use rk_ports::conformance::validate_report;
    use rk_ports::toolchain::{Selection, StepBound, TestStatus};

    let selection = Selection::Filter(vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]);
    let good = FixtureToolchain::from_fixture(vec![])
        .test(&selection, StepBound::Undeclared)
        .expect("a green fake suite is Ok");
    assert_eq!((good.passed, good.failed), (3, 0));
    assert_eq!(good.status, TestStatus::Passing);
    // The control: an outcome as the port builds it passes the row.
    validate_report(&good).expect("the control outcome passes validate_report");

    // (a) The count invariant: passed + failed != selected_test_count. The status is left as
    // `of(2, 0)` would derive it, so only the count rule is broken.
    let mut bad = good.clone();
    bad.passed = 2;
    assert_eq!(TestStatus::of(bad.passed, bad.failed), bad.status);
    let e = validate_report(&bad).expect_err("passed + failed != selected_test_count must refuse");
    assert!(
        e.contains("is not the report's selected_test_count") && e.contains("(2 + 0)"),
        "the count refusal names its rule and both numbers: {e}"
    );

    // (b) The derived status: counts 3 / 0 with a stored `failing`.
    let mut bad = good.clone();
    bad.status = TestStatus::Failing;
    let e = validate_report(&bad).expect_err("a status that disagrees with the counts must refuse");
    assert!(
        e.contains("disagrees with the counts") && e.contains("status is derived"),
        "the status refusal names its rule: {e}"
    );

    // (c) The runner's name: empty, and blank, are both refused.
    for name in ["", "   "] {
        let mut bad = good.clone();
        bad.runner.name = name.to_owned();
        let e = validate_report(&bad).expect_err("an empty runner name must refuse");
        assert!(
            e.contains("runner has an empty name"),
            "the runner refusal names its rule for {name:?}: {e}"
        );
    }

    // (d) The report's own shape: an empty executed-command record fails `validate_shape`.
    let mut bad = good.clone();
    bad.report.argv.clear();
    let e = validate_report(&bad).expect_err("a report that fails its own shape must refuse");
    assert!(
        e.contains("fails its own shape") && e.contains("executed-command record is empty"),
        "the shape refusal names validate_shape's rule: {e}"
    );

    // The control is untouched by the clones above.
    validate_report(&good).expect("the control still passes");
}

/// **A red build is `Ok` with `status: failing`** (D21-43 N25: *"`build` is widened to an outcome
/// report as 1a did for `test`"*).
#[test]
fn toolchain_fake_red_build_is_ok_with_status_failing() {
    use rk_ports::toolchain::StepBound;
    use rk_types::verb_output::BuildStatus;
    let green = FixtureToolchain::from_fixture(vec![])
        .build(StepBound::Undeclared)
        .expect("a green build is Ok");
    assert_eq!((green.status, green.errors), (BuildStatus::Passing, 0));
    let red = FixtureToolchain::from_fixture(vec![])
        .build_failing()
        .build(StepBound::Undeclared)
        .expect("a RED build is Ok with status failing, never Err");
    assert_eq!(red.status, BuildStatus::Failing);
    assert!(
        red.errors >= 1,
        "a failing build names at least one error: {red:?}"
    );
    assert!(!red.argv.is_empty() && !red.toolchain_version.is_empty());
}

/// **`run_verb` refuses a verb the adapter does not implement BY NAME** (record V decision 2:
/// *"An adapter implements the verbs it can and refuses every other one by name"*).
#[test]
fn toolchain_fake_run_verb_refuses_an_undeclared_verb_by_name() {
    use rk_ports::toolchain::{StepBound, VerbCall, VerbOutput};
    let inputs = std::collections::BTreeMap::new();
    let call = |verb| VerbCall {
        verb,
        inputs: &inputs,
        bound: StepBound::Undeclared,
    };
    let fake = FixtureToolchain::from_fixture(vec![]).with_verb(
        "smoke",
        VerbOutput {
            kind: "SmokeResult".to_owned(),
            value: serde_json::json!({"scripted": true}),
        },
    );
    match fake.run_verb(&call("sca_scan")) {
        Err(StepError::Unsupported { capability }) => assert_eq!(
            capability, "sca_scan",
            "the refusal is the bare verb name (ADR-0135 clause 2's shape)"
        ),
        other => panic!("an undeclared verb refuses by name; got {other:?}"),
    }
    let out = fake
        .run_verb(&call("smoke"))
        .expect("a scripted verb answers its scripted output");
    assert_eq!(out.kind, "SmokeResult");
}

/// **A zero step bound is refused** (RT-5: *"a zero bound is no bound"*; record B decision 3).
#[test]
fn step_bound_zero_is_refused() {
    use rk_ports::toolchain::StepBound;
    match StepBound::secs(0) {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.contains("timeout_secs = 0") && capability.contains("RT-5"),
            "the refusal names the zero bound and RT-5: {capability}"
        ),
        other => panic!("a zero bound is refused; got {other:?}"),
    }
    let five = StepBound::secs(5).expect("a positive bound constructs");
    assert_eq!(five.as_secs(), Some(5));
    assert_eq!(StepBound::Undeclared.as_secs(), None);
}

// ---------------------------------------------------------------- ADR-0181 / ADR-0182 (s8e §8e.2, §8e.10)

/// **The fake's tier-1 writer** (ADR-0181 decision 1; `s8e` §8e.2): `VecStore::append_tier1` keeps
/// the anchored row beside its event, so the event is queryable by its item, the row reports its
/// anchor through the port (`BySha`, decision 12), and a sha that anchors nothing selects nothing.
///
/// RED at entry: no `append_tier1` on the port (compile error E0599).
#[test]
fn vec_store_append_tier1_is_queryable_with_its_anchor() {
    use rk_ports::event_store::TierOneAnchor;
    use rk_types::event::Event;

    let sha = "1".repeat(40);
    let tree = "2".repeat(40);
    let mut store = VecStore::default();
    let boundary = Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "triage", "to": "implement" }),
    )
    .with_item("item-a");
    store
        .append_tier1(boundary.clone(), &TierOneAnchor::new(&sha, &tree))
        .expect("the fake's tier-1 append");
    // A tier-2 row appended the ordinary way carries no anchor.
    let plain =
        Event::new(EventKind::ReconcileStarted, serde_json::json!({ "n": 1 })).with_item("item-a");
    store.append(plain.clone()).expect("append");

    assert_eq!(
        store
            .query(Query::ByItem("item-a".to_owned()))
            .expect("by item"),
        vec![boundary.clone(), plain],
        "the anchored event is queryable as the event, in append order"
    );
    assert_eq!(
        store.query(Query::BySha(sha.clone())).expect("by sha"),
        vec![boundary],
        "the anchored row reports its anchor: BySha selects it and only it (ADR-0181 decision 12)"
    );
    assert_eq!(
        store
            .query(Query::BySha("3".repeat(40)))
            .expect("by another sha"),
        Vec::<Event>::new(),
        "a sha that anchors nothing selects nothing"
    );
    assert_eq!(store.anchor_of(0), Some(sha.as_str()));
    assert_eq!(store.anchor_of(1), None, "an ordinary append has no anchor");
}

/// **`ByItem` is honoured by the fake** (ADR-0182 decision 4; `s8e` §8e.10): a row keyed to
/// another item is never returned; an unkeyed row is returned to every item in the interim;
/// `ByRun` stays inert (decision 5).
///
/// RED at entry: no `Event::with_item` (compile error E0599); against a fake whose `query` ignores
/// the selector, `ByItem("item-a")` returns item-b's row.
#[test]
fn vec_store_query_by_item_returns_only_that_items_rows() {
    use rk_types::event::Event;

    let a = Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "to": "triage" }),
    )
    .with_item("item-a");
    let b = Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "to": "triage" }),
    )
    .with_item("item-b");
    let unkeyed = Event::new(EventKind::MemoMiss, serde_json::json!({ "key": "k" }));
    let mut store = VecStore::default();
    for e in [&a, &b, &unkeyed] {
        store.append(e.clone()).expect("append");
    }

    assert_eq!(
        store.query(Query::ByItem("item-a".to_owned())).expect("a"),
        vec![a.clone(), unkeyed.clone()],
        "ByItem(item-a) returns item-a's row and the unkeyed row, never item-b's"
    );
    assert_eq!(
        store.query(Query::ByItem("item-b".to_owned())).expect("b"),
        vec![b.clone(), unkeyed.clone()],
        "ByItem(item-b) returns item-b's row and the unkeyed row, never item-a's"
    );
    assert_eq!(
        store.query(Query::ByRun("r".to_owned())).expect("run"),
        vec![a, b, unkeyed],
        "ByRun stays inert: every row (ADR-0182 decision 5)"
    );
}
