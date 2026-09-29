//! **`dispatch_completed` is folded and moves nothing** — ADR-0126 decision 5, as an assertion
//! over the three kernel folds and not as three comments beside three `ALL.len()` pins.
//!
//! ADR-0126 decision 5: *"the kernel's exhaustive matches (`fsm.rs`, `breaker.rs`, `memo.rs`) route
//! it to their no-state-change arms — the kernel folds it and moves nothing"*; its forbids:
//! *"Forbids folding the row as anything but a no-state-change arm"*. Step 7c Slice 1 made exactly
//! that routing — the one kernel code change the slice makes — and the three pins
//! (`tests/it/breaker.rs`, `tests/it/fsm.rs`, `tests/it/memo.rs`) answered their clauses in
//! comments and assertion strings only. A routing mutation that makes the row state-moving
//! survived the whole kernel suite [measured 2026-09-17 on a scratch copy: `fsm.rs` and `memo.rs`
//! with the variant moved into the `Dispatch` arm → `177 passed; 0 failed`]. This file is the test
//! that mutation fails, one function per fold, each **watched red against that mutant** before the
//! routing was confirmed [2026-09-17: FSM — `MalformedPayload { kind: "dispatch", detail:
//! "`item` is absent or is not a string" }`; memo — `left: 2 / right: 1` on `dispatch_events`;
//! breaker — `left: 0 / right: 1` on the consecutive count]:
//!
//! - **the FSM** (`workflow::fsm::fold`): the row folds `Ok`, `state_moving` does not move, and
//!   `events_folded` moves by one. Under the mutant the `Dispatch` arm reads
//!   `activation_key(event, "dispatch")` and a `dispatch_completed` payload — a `DispatchRecord`,
//!   which carries no `activity_id_at_version` — is an `FsmError::MalformedPayload` on **every**
//!   real journal;
//! - **the memo tally** (`memo::tally_activations`): `dispatch_events`, `executed_steps` and
//!   `unattributed_dispatches` are unchanged and `events_folded` is one greater. Under the mutant
//!   `dispatch_events` doubles per activation — the hazard `memo.rs`'s own arm comment names;
//! - **the breaker** (`breaker::BreakerState::from_events`): a family's consecutive count and its
//!   open set are what they were. Under the mutant that reads the row as the activation's success
//!   (the `ActivityCommitted` reset arm) the count goes 1 → 0. The breaker reads only family-scoped rows, and a real
//!   `dispatch_completed` payload carries no `family` member, so the kind's arm is reached only by
//!   a row that has one; the case below adds `family` to the record's payload **by hand** — a
//!   synthetic row, said so — because a fold that never reaches the arm cannot tell the arm is
//!   empty.
//!
//! The payload is the pre-spawn `DispatchRecord` (`Pinned`, every usage-adjacent member `None`,
//! `api_retry_events: 0`) serialised as `Event.payload` is — the shape the adapter writes on the
//! AC-221/AC-376 refusals — because it is the smallest real row and the fold reads none of its
//! members.

use rk_kernel::memo::tally_activations;
use rk_kernel::workflow::breaker::{BreakerState, FamilyKey};
use rk_kernel::workflow::envelope::BreakerThreshold;
use rk_kernel::workflow::fsm::fold;
use rk_types::activity::{DispatchRecord, RecordTail};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::usage::{CliVersion, Timings};

const ITEM: &str = "INC-1";
const STAGE: &str = "implement";
const ACTIVITY: &str = "impl-report@1";
const FAMILY: &str = "family-under-test";

/// The pre-spawn record: the pin as `Pinned`, nothing measured (AC-362: `None`, never `Some(0)`).
fn pre_spawn_record() -> DispatchRecord {
    DispatchRecord::new(
        &["claude".to_owned(), "-p".to_owned()],
        &[("CLAUDE_CODE_PROMPT_CACHE_TTL".to_owned(), "1h".to_owned())],
        "2.1.275",
        b"",
        b"",
        RecordTail {
            cli_version: CliVersion::Pinned("2.1.275".to_owned()),
            api_retry_events: 0,
            prompt_cache_ttl: "1h".to_owned(),
            subagent_prompt_cache_ttl: "1h".to_owned(),
            timings: Timings {
                wall_ms: 3,
                api_ms: None,
            },
            usage: None,
            total_cost_usd: None,
            headroom_before: None,
            headroom_after: None,
            cancelled_by: None,
            detected_by: None,
            activity_id_at_version: None,
        },
    )
}

/// The row as the adapter appends it: kind `dispatch_completed`, payload the record.
fn dispatch_completed() -> Event {
    Event::new(
        EventKind::DispatchCompleted,
        serde_json::to_value(pre_spawn_record()).expect("the record serialises"),
    )
}

fn transition(from: &str, to: &str) -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": from, "to": to }),
    )
}

fn memo_miss() -> Event {
    Event::new(
        EventKind::MemoMiss,
        serde_json::json!({ "key": "k-1", "item": ITEM, "activity_id_at_version": ACTIVITY, "attempt": 1 }),
    )
}

fn dispatch() -> Event {
    Event::new(
        EventKind::Dispatch,
        serde_json::json!({ "item": ITEM, "activity_id_at_version": ACTIVITY, "attempt": 1 }),
    )
}

fn platform_failure(at: i64) -> Event {
    Event::new(
        EventKind::StepFailed,
        serde_json::json!({ "class": "platform", "family": FAMILY, "at": at }),
    )
}

/// **ADR-0126 decision 5, the FSM.** Red under the routing mutant with
/// `Err(MalformedPayload { kind: "dispatch", .. })` — the `Dispatch` arm's `activation_key` read
/// over a payload that has no `activity_id_at_version`.
#[test]
fn adr0126_dispatch_completed_moves_nothing_in_the_fsm() {
    let without = vec![transition("triage", STAGE), memo_miss(), dispatch()];
    let mut with = without.clone();
    with.push(dispatch_completed());

    let base = fold(ITEM, STAGE, &without).expect("the journal without the row folds");
    let folded = fold(ITEM, STAGE, &with)
        .expect("a `dispatch_completed` row folds: the FSM reads no member of the record");

    assert_eq!(
        folded.survey().state_moving(),
        base.survey().state_moving(),
        "`dispatch_completed` moved the FSM's state: ADR-0126 decision 5 routes it to the \
         no-state-change arm and forbids anything else"
    );
    assert_eq!(
        folded.survey().events_folded(),
        base.survey().events_folded() + 1,
        "the row was folded (looked at), not skipped — House Shape C"
    );
    assert_eq!(folded.stage(), base.stage());
    assert_eq!(folded.open_gates().count(), base.open_gates().count());
}

/// **ADR-0126 decision 5, the memo tally** (AC-019's identity). Red under the routing mutant with
/// `dispatch_events` at 2 for one activation — the doubling `memo.rs`'s arm comment names.
#[test]
fn adr0126_dispatch_completed_is_not_counted_as_a_dispatch() {
    let without = vec![memo_miss(), dispatch()];
    let mut with = without.clone();
    with.push(dispatch_completed());

    let base = tally_activations(&without).expect("the journal without the row tallies");
    let tally = tally_activations(&with).expect("a `dispatch_completed` row tallies");

    assert_eq!(
        tally.dispatch_events, base.dispatch_events,
        "`dispatch_completed` was counted as a `dispatch`: ADR-0126 decision 3 — two kinds, two \
         producers, and *\"nothing counts them as one, and nothing may\"*"
    );
    assert_eq!(tally.executed_steps, base.executed_steps);
    assert_eq!(tally.unattributed_dispatches, base.unattributed_dispatches);
    assert_eq!(tally.activations, base.activations);
    assert_eq!(tally.events_folded, base.events_folded + 1);
    assert!(
        tally.identity_holds(),
        "AC-019's identity still holds over the journal with the row"
    );
}

/// **ADR-0126 decision 5, the breaker** (AC-033's *"none of them is a breaker mark"*). The row is
/// given a `family` by hand — SYNTHETIC, so the fold reaches the kind's arm at all — and neither
/// the consecutive count nor the open set moves. Red under a mutant that routes the kind to the
/// `ActivityCommitted` reset arm — a completion read as a success (consecutive 1 → 0).
#[test]
fn adr0126_dispatch_completed_is_not_a_breaker_mark() {
    let family = FamilyKey::supplied(FAMILY);
    let k = BreakerThreshold::declared(2);
    let without = vec![platform_failure(1)];
    let mut with = without.clone();
    let mut row = dispatch_completed();
    // SYNTHETIC: the adapter's record carries no `family`; without one the breaker skips the row
    // before it reads the kind, and this test would be green over an arm it never reached.
    row.payload["family"] = serde_json::json!(FAMILY);
    with.push(row);

    let base = BreakerState::from_events(&without, k).expect("the journal without the row folds");
    let state =
        BreakerState::from_events(&with, k).expect("a family-scoped `dispatch_completed` folds");

    assert_eq!(
        state.consecutive(&family),
        base.consecutive(&family),
        "`dispatch_completed` moved a family's consecutive-`Platform` count: it is telemetry about a \
         dispatch the `step_*` rows already classified, and not a breaker mark (ADR-0126 decision 4)"
    );
    assert_eq!(state.open_families(), base.open_families());
    assert!(
        state.open_families().is_empty(),
        "one failure under K = 2 opens nothing, with or without the row"
    );
    assert_eq!(
        state.survey().family_scoped,
        base.survey().family_scoped + 1,
        "the row was read as family-scoped"
    );
}
