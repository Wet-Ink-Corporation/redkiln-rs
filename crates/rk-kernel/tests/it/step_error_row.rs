//! **The kernel writes the error row** — `s8d` §8d.5; record R4 (ADR-0176), proposed.
//!
//! D21-42 20: *"The kernel appends `StepError::event()` for every dispatch, so fake and real
//! harnesses behave alike, and AC-033's breaker counts those rows across processes."* Until R4 the
//! row was the HARNESS's: `rk_ports::fakes::FakeHarness` appended it through its own `event_for`,
//! and `ClaudeHarness` appended none, so the class reached the journal under the fake only.
//!
//! Every test below reads the JOURNAL a later process would fold, not only the report. Tests 1 and
//! 4 drive the runtime through `crate::runtime::CountingHarness`, which appends nothing, so every
//! error row they count is the kernel's; tests 2 and 3 drive `FakeHarness` itself, so a fake that
//! kept its own append would put a second row beside the kernel's.

use std::collections::BTreeMap;
use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::activity::ActivityReport;
use rk_kernel::runtime::marks::read_retry_at_marks;
use rk_kernel::runtime::step::Activation;
use rk_kernel::runtime::{BreakerBinding, Runtime};
use rk_kernel::select::Pack;
use rk_kernel::workflow::breaker::{BreakerState, FamilyKey, JournaledNow};
use rk_kernel::workflow::envelope::AttemptCap;
use rk_ports::fakes::{FakeHarness, VecStore};
use rk_ports::harness::Harness;
use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::error::{CancelledBy, IsolationSurface, StepError, TransportKind};
use rk_types::event::{ErrorEvent, Event};
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::typed::{SchemaId, Typed};

use crate::runtime::{
    AID, CountingHarness, ITEM, STAGE, STEP, at, counters, gate_site, journal, ports, spec, store,
};

/// The pack that declares the KEY (D-7) beside the breaker envelope: `K = 2`, the expiry `= 900`,
/// `investigator`/`implementer` → `deliberative`, `reviewer` → `adversarial`.
pub const ROLE_FAMILIES: &str = include_str!("../fixtures/packs/role-families.toml");

/// The four error kinds, the only rows this file counts as "an error row".
pub const ERROR_KINDS: [EventKind; 4] = [
    EventKind::StepFailed,
    EventKind::StepCancelled,
    EventKind::StepRefused,
    EventKind::RateLimited,
];

/// The error rows of a journal, in append order.
#[must_use]
pub fn error_rows(events: &[Event]) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| ERROR_KINDS.contains(&e.kind))
        .collect()
}

/// The kind `StepError::event()` maps a variant to — `rk-types`' own total map (AC-360).
#[must_use]
pub const fn kind_of(event: &ErrorEvent) -> EventKind {
    match event {
        ErrorEvent::StepFailed { .. } => EventKind::StepFailed,
        ErrorEvent::StepCancelled => EventKind::StepCancelled,
        ErrorEvent::RateLimited => EventKind::RateLimited,
        ErrorEvent::StepRefused { .. } => EventKind::StepRefused,
    }
}

/// One of each of the closed eleven, in `rk-types`' declaration order.
#[must_use]
pub fn every_step_error() -> Vec<StepError> {
    let d = Digest::from_hex(&"ab".repeat(32)).expect("64 lowercase hex characters");
    vec![
        StepError::Transport {
            kind: TransportKind::Spawn,
            detail: "spawn refused".to_owned(),
        },
        StepError::Schema {
            attempts: 1,
            last_error: "bad".to_owned(),
        },
        StepError::Timeout { after_secs: 5 },
        StepError::Cancelled {
            by: CancelledBy::Signal,
        },
        StepError::NonZero {
            code: Some(2),
            stderr_digest: d.clone(),
        },
        StepError::RateLimited {
            resets_at: Some(Timestamp::from_unix_seconds(4_000)),
        },
        StepError::NotAdopted {
            expected_agent: "a".to_owned(),
            observed: "b".to_owned(),
        },
        StepError::NotIsolated {
            surface: IsolationSurface::Tools,
            diff_digest: d,
        },
        StepError::Platform {
            status: 529,
            retry_after: None,
        },
        StepError::Unsupported {
            capability: "affected-selection".to_owned(),
        },
        StepError::Overage { reason: None },
    ]
}

/// The role-families pack.
///
/// # Panics
///
/// If the fixture stops loading.
#[must_use]
pub fn role_families_pack() -> Pack {
    Pack::from_toml("role-families.toml", ROLE_FAMILIES).expect("the role-families fixture loads")
}

/// One agent step at `role`, every optional field absent.
#[must_use]
pub fn agent_step(id: &str, role: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Agent,
        run: None,
        output_type: None,
        role: Some(role.to_owned()),
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: None,
        inputs: BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    }
}

/// `[agent(step_id, role), emit]` of activation `aid` on `item`, walked once at `now` with `cap`,
/// each agent step dispatching under [`spec`].
///
/// # Panics
///
/// If the walk returns a `RuntimeError` (a store or fold failure, never a step's own failure).
#[must_use]
pub fn walk_one(
    rt: &Runtime,
    (item, aid): (&str, &str),
    (step_id, role): (&str, &str),
    cap: u32,
    now: JournaledNow,
) -> ActivityReport {
    let steps = vec![
        agent_step(step_id, role),
        Step {
            kind: StepKind::Emit,
            role: None,
            ..agent_step("emit", "none")
        },
    ];
    let specs: BTreeMap<String, ActivitySpec> =
        std::iter::once((step_id.to_owned(), spec())).collect();
    let state = rk_kernel::workflow::fsm::fold(item, STAGE, &[]).expect("an empty journal folds");
    rt.run_activity_steps(
        Activation {
            item,
            activity_id_at_version: aid,
            step_id: STEP,
            spec: None,
            step_specs: Some(&specs),
            rigor: Rigor::Recommended,
        },
        Some(&steps),
        &Context::new(),
        gate_site(&state),
        AttemptCap::declared(cap),
        now,
    )
    .expect("a step's own failure is a value, never an error of the walk (AC-032)")
}

// ---- the tests ---------------------------------------------------------------------------------

/// **One error row per failed dispatch, written by the kernel** — each of the closed eleven, plus
/// the one error the KERNEL classifies (a well-formed value under the wrong schema, AC-031's
/// `Schema`), and a retried activation whose two failed dispatches write two rows.
///
/// The harness is `CountingHarness`, which appends nothing, so every row here is the kernel's. Red
/// before R4 because the kernel wrote none: every variant but `RateLimited` (whose AC-030 mark the
/// kernel already wrote) read zero rows.
#[test]
fn the_kernel_appends_exactly_one_error_row_per_failed_dispatch() {
    let variants = every_step_error();
    assert_eq!(variants.len(), 11, "PREMISE: the closed set is eleven");
    for error in variants {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(CountingHarness::scripted(vec![Err(error.clone())]));
        let rt = Runtime::new(
            ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
            "run-error-row",
            STAGE,
        );
        // `N = 1`: no class re-dispatches, so one dispatch fails once.
        let report = rt
            .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(1), at(100))
            .expect("a failure is a value in the report");
        assert_eq!(harness.dispatches(), 1, "{error:?}: exactly one dispatch");
        assert!(!report.committed(), "{error:?}: nothing committed");
        let events = journal(&db);
        let rows = error_rows(&events);
        assert_eq!(
            rows.len(),
            1,
            "{error:?}: a failed dispatch writes EXACTLY ONE error row, and the kernel writes it; \
             rows = {:?}",
            rows.iter()
                .map(|e| (e.kind, &e.payload))
                .collect::<Vec<_>>()
        );
        let want = kind_of(&error.event());
        assert_eq!(
            rows[0].kind, want,
            "{error:?} maps to {want:?} through `StepError::event()` (AC-360)"
        );
        match error.event() {
            ErrorEvent::StepFailed { class, status } => {
                assert_eq!(
                    rows[0].payload.get("class"),
                    Some(&serde_json::json!(class)),
                    "{error:?}: the row carries the class"
                );
                assert_eq!(
                    rows[0].payload.get("status"),
                    Some(&serde_json::json!(status)),
                    "{error:?}: and the status"
                );
            }
            ErrorEvent::StepRefused { class, capability } => {
                assert_eq!(
                    rows[0].payload.get("class"),
                    Some(&serde_json::json!(class))
                );
                assert_eq!(
                    rows[0].payload.get("capability"),
                    Some(&serde_json::json!(capability))
                );
            }
            ErrorEvent::StepCancelled | ErrorEvent::RateLimited => {}
        }
        // The dispatch row precedes its error row: a row written before the dispatch is not
        // about it.
        let dispatch_at = events
            .iter()
            .position(|e| e.kind == EventKind::Dispatch)
            .expect("the dispatch row");
        let error_at = events
            .iter()
            .position(|e| ERROR_KINDS.contains(&e.kind))
            .expect("the error row");
        assert!(
            dispatch_at < error_at,
            "{error:?}: dispatch, then its error row"
        );
    }

    // ---- the error the KERNEL classifies: a value under the wrong schema (AC-031) -------------
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(vec![Ok(Typed::new(
        SchemaId::new("Findings/v1"),
        serde_json::json!({ "x": 1 }),
    ))]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-error-row-schema",
        STAGE,
    );
    let report = rt
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(1), at(100))
        .expect("a failure is a value");
    assert_eq!(
        report.schema_failures(),
        1,
        "PREMISE: the boundary refused the schema"
    );
    let events = journal(&db);
    let rows = error_rows(&events);
    assert_eq!(
        rows.len(),
        1,
        "a boundary schema failure is a failed dispatch too"
    );
    assert_eq!(rows[0].kind, EventKind::StepFailed);
    assert_eq!(
        rows[0].payload.get("class"),
        Some(&serde_json::json!("schema"))
    );

    // ---- ONE ROW PER FAILED DISPATCH, not per activation: two failures, then a commit ----------
    let (q, a) = counters();
    let db = store(&q, &a);
    let platform = StepError::Platform {
        status: 529,
        retry_after: None,
    };
    let harness = Arc::new(CountingHarness::scripted(vec![
        Err(platform.clone()),
        Err(platform),
        crate::runtime::ok_typed(serde_json::json!({ "done": true })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-error-row-retry",
        STAGE,
    );
    let report = rt
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(4), at(100))
        .expect("the retry loop reports");
    assert!(report.committed(), "the third dispatch commits");
    assert_eq!(harness.dispatches(), 3);
    let events = journal(&db);
    assert_eq!(
        error_rows(&events).len(),
        2,
        "two failed dispatches write two error rows, and the committed one writes none"
    );
}

/// **The fake writes no error row of its own** (M11). `FakeHarness::dispatch` over a bare
/// `VecStore` appends NOTHING for any of the eleven, nor on its cancel path; the row is the
/// kernel's. Through the runtime, the same fake leaves exactly ONE row per failed dispatch — so a
/// fake that kept its append would leave two.
#[test]
fn the_fake_harness_writes_no_error_row_of_its_own() {
    for error in every_step_error() {
        let harness = FakeHarness::scripted(vec![Err(error.clone())]);
        let mut bare = VecStore::default();
        let outcome = harness.dispatch(&spec(), &mut bare);
        assert_eq!(outcome.result, Err(error.clone()), "the script is returned");
        assert!(
            bare.events().is_empty(),
            "{error:?}: FakeHarness appended {:?} — the error row is the KERNEL's (R4), and a \
             harness that writes its own puts a second row beside it",
            bare.events()
        );
    }
    // The cancel path, which returns before the script.
    let token = rk_ports::cancel::CancelToken::new();
    token.cancel();
    let harness = FakeHarness::scripted(vec![]).with_token(token);
    let mut bare = VecStore::default();
    let outcome = harness.dispatch(&spec(), &mut bare);
    assert!(
        matches!(
            outcome.result,
            Err(StepError::Cancelled {
                by: CancelledBy::Operator
            })
        ),
        "PREMISE: a fired token cancels"
    );
    assert!(
        bare.events().is_empty(),
        "the cancel path appended {:?}; the row is the kernel's",
        bare.events()
    );

    // Through the runtime: exactly one row per failed dispatch, under the fake as under any
    // harness (D21-42 20: "so fake and real harnesses behave alike").
    for error in every_step_error() {
        let (q, a) = counters();
        let db = store(&q, &a);
        let harness = Arc::new(FakeHarness::scripted(vec![Err(error.clone())]));
        let rt = Runtime::new(
            ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
            "run-fake-row",
            STAGE,
        );
        let _ = rt
            .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(1), at(100))
            .expect("a failure is a value");
        let events = journal(&db);
        assert_eq!(
            error_rows(&events).len(),
            1,
            "{error:?}: under FakeHarness the journal holds {} error rows, not one: {:?}",
            error_rows(&events).len(),
            error_rows(&events)
                .iter()
                .map(|e| (e.kind, &e.payload))
                .collect::<Vec<_>>()
        );
    }
}

/// **`RateLimited` is ONE `rate_limited` row — AC-030's mark — never a second.** Before R4 the
/// fake appended a bare `rate_limited` failure row and the kernel appended the `retry_at` mark:
/// two rows of the one kind.
#[test]
fn a_rate_limited_dispatch_yields_one_rate_limited_row_not_two() {
    const RESET: i64 = 7_200;
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(FakeHarness::scripted(vec![Err(StepError::RateLimited {
        resets_at: Some(Timestamp::from_unix_seconds(RESET)),
    })]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-rate-limited",
        STAGE,
    );
    let report = rt
        .run_activity(ITEM, AID, STEP, &spec(), AttemptCap::declared(4), at(100))
        .expect("a rate limit is a value in the report");
    assert!(report.mark().is_some(), "PREMISE: AC-030's mark was taken");
    let events = journal(&db);
    let rate: Vec<&Event> = events
        .iter()
        .filter(|e| e.kind == EventKind::RateLimited)
        .collect();
    assert_eq!(
        rate.len(),
        1,
        "a rate-limited dispatch writes ONE `rate_limited` row, AC-030's mark, and never a bare \
         failure row beside it: {:?}",
        rate.iter().map(|e| &e.payload).collect::<Vec<_>>()
    );
    assert_eq!(
        rate[0].payload.get("kind"),
        Some(&serde_json::json!("retry_at"))
    );
    assert_eq!(
        rate[0].payload.get("resets_at"),
        Some(&serde_json::json!(RESET))
    );
    assert_eq!(
        read_retry_at_marks(&events).len(),
        1,
        "the one row IS the mark"
    );
    assert_eq!(error_rows(&events).len(), 1, "and it is the only error row");
}

/// **The row carries what the breaker folds: `family` and `at`.** `BreakerState::from_events`
/// refuses a family-scoped `step_failed` without `class` and an integer `at`; the kernel's row
/// carries the pack's family for the step's role (D-7) and the journalled instant, so the fold
/// reads it and counts it. The commit row of a successful agent step carries the family too, which
/// is what the fold's reset arm reads.
#[test]
fn the_error_row_carries_family_and_at_so_the_breaker_can_fold_it() {
    let pack = role_families_pack();
    let k = pack
        .require_breaker_threshold()
        .expect("the fixture declares K");
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(vec![
        Err(StepError::Platform {
            status: 529,
            retry_after: None,
        }),
        crate::runtime::ok_typed(serde_json::json!({ "draft": 1 })),
    ]));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-family-at",
        STAGE,
    )
    .with_breaker(BreakerBinding::of(&pack));

    let failed = walk_one(&rt, (ITEM, AID), ("review", "reviewer"), 1, at(100));
    assert!(
        !failed.committed(),
        "PREMISE: the reviewer's dispatch failed"
    );
    let events = journal(&db);
    let rows = error_rows(&events);
    assert_eq!(rows.len(), 1, "one failed dispatch, one row");
    assert_eq!(rows[0].kind, EventKind::StepFailed);
    assert_eq!(
        rows[0].payload.get("class"),
        Some(&serde_json::json!("platform"))
    );
    assert_eq!(
        rows[0].payload.get("family"),
        Some(&serde_json::json!("adversarial")),
        "the row carries the PACK's family for the step's role (`reviewer` → `adversarial`, D-7), \
         never the activity's"
    );
    assert_eq!(
        rows[0].payload.get("at"),
        Some(&serde_json::json!(100)),
        "the row carries the journalled instant"
    );
    let state = BreakerState::from_events(&events, k).expect("the kernel's row folds");
    let adversarial = FamilyKey::supplied("adversarial");
    assert_eq!(state.consecutive(&adversarial), 1, "and the fold counts it");
    assert_eq!(state.survey().platform_failures, 1);

    // The success row carries the family too, so the fold's reset arm has something to read.
    let ok = walk_one(&rt, ("INC-2", AID), ("draft", "implementer"), 1, at(101));
    assert!(
        ok.committed() || ok.completed(),
        "PREMISE: the implementer's step committed"
    );
    let events = journal(&db);
    let committed_draft = events
        .iter()
        .find(|e| {
            e.kind == EventKind::ActivityCommitted
                && e.payload.get("step_id") == Some(&serde_json::json!("draft"))
        })
        .expect("the draft step's commit row");
    assert_eq!(
        committed_draft.payload.get("family"),
        Some(&serde_json::json!("deliberative")),
        "the step's commit row carries its family, the success the fold resets on"
    );
}
