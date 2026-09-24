//! **The tier-1 projection, asserted UNFEATURED** — Phase-1 exit criterion (2), Step 8.
//!
//! Two criteria are discharged here and both are named by `s8`'s own filters, which run
//! `cargo test -p rk-kernel -- <name>` with **no `--features corpus`**:
//!
//! | test | criterion | what it compares |
//! | --- | --- | --- |
//! | [`rebuild_differential`] | **AC-184** / ADR-0105 clause 9 | a journal's tier-1 projection against the same journal round-tripped, under the canonical serialiser |
//! | [`stage_level_next_action`] | **AC-161** | the stage-grain next action across a truncated tier 2 and a deleted event database |
//!
//! # Why this module exists rather than a test in `replay.rs`
//!
//! [`crate::replay`] is `#![cfg(feature = "corpus")]` over the **whole module**, so a test placed
//! there is not compiled in the run `s8`'s block makes and the block fails with *"0 tests
//! matched"*. [`crate::recorded_log`] records the same trap and is the precedent for siting an
//! unfeatured golden-log control outside `replay.rs`; this module follows it and reads the same
//! committed log through the same helper.
//!
//! **No test name here contains `golden_log_replay` or `ac161_ac164_delete_db_replay`**, so
//! Checkpoint 7b's zero-selection line — a bare-substring count over the unfeatured `--list` with
//! no word anchor — stays at **0**.
//!
//! # What is asserted, and what is ARMED
//!
//! `stage_action` IS the tier-1 projection: it filters `EventKind::tier() == Tier::One` itself and
//! folds only those, so a caller holding tier-2 facts cannot leak one into a stage-grain answer.
//! `StageAction::canonical_bytes` is the comparison AC-161 and AC-184 both name — `rk_store::canon`,
//! whose own header lists these two criteria among its four consumers.
//!
//! 🛑 **THREE THINGS ARE ARMED HERE AND NONE IS REPORTED AS COVERED.**
//!
//! 1. **The ROUTE is not measured, only the EQUALITY.** AC-184's *"from git alone"* and the F12
//!    substrate — `predecessor-47faec9f.bundle`, 118 refs — are read through `rk-git` and `rk-cli`,
//!    and the bundle is corpus-bound while this module is deliberately unfeatured. What is measured
//!    is that the stage action is a function of the tier-1 events alone and survives a round trip.
//! 2. **The 40-hex corpus-pin assertion and the rebased-commit fixture AC-184 names do not exist.**
//!    Neither is claimed by this module.
//! 3. **`doctor --rebuild` is not invoked here and its stdout is not the comparison** — `s8`'s own
//!    block says so in terms, and runs it separately as AC-427's operator path.

use rk_kernel::replay::read_run;
use rk_kernel::workflow::fsm::{StageAction, stage_action};
use rk_types::event::Event;
use rk_types::event_kind::{EventKind, Tier};

use crate::recorded_log::the_committed_run;

/// The committed golden log's own item and stage — `INC-1` at `implement`, which the fixture's
/// `run.json` and its `activity_committed` rows pin.
const ITEM: &str = "INC-1";
const STAGE: &str = "implement";

/// **The tier filter is UNOBSERVABLE through `StageAction`, so it is asserted at the SOURCE.**
///
/// Measured 2026-09-21 by a mutant cycle: replacing `stage_action`'s tier filter with
/// `events.to_vec()` left **every** test in this crate green, these two included. The reason is
/// structural and worth writing down rather than re-deriving — `StageAction` exposes exactly
/// `transition` and `gates`; `transition` is moved only by `StageTransition`, and `gates_open` only
/// by `GateOpened` and `GateExpired`. All three are `Tier::One`. So no tier-2 event can move a
/// `StageAction` under the current `apply` arms, and removing the filter is **behaviourally
/// equivalent today**. The filter is defence in depth: it makes the stage grain a property of the
/// function rather than a standing coincidence about which arms happen to touch which fields, and
/// `fsm.rs` says so — *"it filters `Tier::One` itself, so a caller holding tier-2 facts cannot leak
/// one into a stage-grain answer"*.
///
/// A guard that cannot fail is the defect wearing a green, so the guard is moved to where it CAN
/// fail: this crate's established source-lint pattern (`tests/it/retry.rs`'s
/// `ac028_policy_match_has_no_wildcard` and `tests/it/recorded_log.rs`'s
/// `the_replay_still_calls_the_divergence_detection` both read their `src/` file the same way).
///
/// **It scans CODE lines only.** A comment naming the filter must not satisfy it — that is how a
/// guard gets satisfied by the prose beside it.
#[test]
fn stage_action_still_filters_to_tier_one() {
    const SRC: &str = include_str!("../../src/workflow/fsm.rs");

    let body: String = SRC
        .lines()
        .skip_while(|l| !l.starts_with("pub fn stage_action("))
        .take_while(|l| !l.starts_with("}"))
        .map(|l| l.split("//").next().unwrap_or("")) // CODE only
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !body.is_empty(),
        "PREMISE: `pub fn stage_action(` was not found in fsm.rs — this lint is reading the wrong \
         file or the signature moved, and a lint that finds nothing asserts nothing"
    );
    assert!(
        body.contains("e.kind.tier() == Tier::One"),
        "`stage_action` no longer filters its input to the tier-1 projection. AC-161 and AC-184 \
         are both stage-grain and both turn on that filter, and NO behavioural test can catch its \
         removal: measured 2026-09-21, deleting it leaves the whole crate green, because no \
         tier-2 event moves a `StageAction` under today's `apply` arms. Body scanned:\n{body}"
    );
}

/// The committed journal, whole.
fn journal() -> Vec<Event> {
    let run = read_run(&the_committed_run()).expect("the committed golden log reads");
    run.events
}

/// Split a journal into its tier-1 projection and the tier-2 count that was dropped.
fn project(events: &[Event]) -> (Vec<Event>, usize) {
    let tier_one: Vec<Event> = events
        .iter()
        .filter(|e| e.kind.tier() == Tier::One)
        .cloned()
        .collect();
    let dropped = events.len() - tier_one.len();
    (tier_one, dropped)
}

/// **AC-184 / ADR-0105 clause 9.** *"THEN the resulting tier-1 state is byte-identical under the
/// canonical serialiser to the **tier-1 projection** of the original store … orphan rows and
/// rewrite events are tier-2-derived and sit **outside** this differential."*
///
/// The differential is taken kernel-side, which is the only place it can be taken from this crate:
/// `rk-cli`'s `doctor::rebuild` holds the git route and `rk-kernel` may not depend on `rk-cli` (and
/// must not depend on any `rk-adapter-*` — AC-484). So what is compared is the projection of the
/// journal as recorded against the projection of that journal **round-tripped through its own
/// serialisation**, which is the operation a rebuild performs on the rows it recovers.
///
/// **Four things make this non-vacuous**, and each would otherwise be satisfied by a constant:
/// the round trip must really re-serialise; the journal must really span both tiers; a `Rewrite`
/// event must really be excluded by the tier filter rather than by its absence; and a different
/// journal must give a different answer.
#[test]
fn rebuild_differential() {
    let events = journal();
    let (tier_one, dropped) = project(&events);

    // ---- premises ------------------------------------------------------------------------------
    assert!(
        !tier_one.is_empty() && dropped >= 2,
        "PREMISE: an all-tier-1 journal makes this differential trivially true. Got {} tier-1 and \
         {dropped} tier-2 of {} events",
        tier_one.len(),
        events.len()
    );
    let before = stage_action(ITEM, STAGE, &events).expect("the tier-1 fold resolves");
    assert_ne!(
        before,
        StageAction::default(),
        "PREMISE: an empty stage action equals another empty one, and every equality below would \
         hold over two nothings"
    );

    // ---- the differential: the rows a rebuild recovers, round-tripped ---------------------------
    //
    // Serialise each tier-1 row's payload and read it back, then rebuild the row with
    // `Event::new(kind, payload)` — which is precisely how `rk-cli`'s `events_from_trailers`
    // reconstructs a row it recovers from git. `Event` is `Serialize` and deliberately not
    // `Deserialize`, so a whole-struct round trip is not available and would be the wrong shape
    // anyway: a rebuild never deserialises a stored `Event`, it MINTS one from recovered parts.
    let recovered: Vec<Event> = tier_one
        .iter()
        .map(|e| {
            let bytes = serde_json::to_string(&e.payload).expect("a recorded payload serialises");
            let payload: serde_json::Value = serde_json::from_str(&bytes).expect("and reads back");
            Event::new(e.kind, payload)
        })
        .collect();
    assert_eq!(
        recovered.len(),
        tier_one.len(),
        "PREMISE: the round trip kept every tier-1 row"
    );
    let after = stage_action(ITEM, STAGE, &recovered).expect("resolves over the recovered rows");
    assert_eq!(
        before.canonical_bytes().expect("canonical"),
        after.canonical_bytes().expect("canonical"),
        "AC-184: the rebuilt tier-1 state is byte-identical, under the canonical serialiser, to \
         the tier-1 projection of the original store"
    );

    // ---- ADR-0105 clause 9: a rewrite event is OUTSIDE the differential -------------------------
    //
    // Asserted rather than inherited. `EventKind::Rewrite` is documented as tier-2-derived and
    // outside AC-184's rebuild; the tier filter already drops it, and this pins that it does.
    assert_eq!(
        EventKind::Rewrite.tier(),
        Tier::Two,
        "ADR-0105 clause 9: rewrite events are tier-2-derived and sit outside this differential"
    );
    let mut with_rewrite = events.clone();
    with_rewrite.push(Event::new(
        EventKind::Rewrite,
        serde_json::json!({ "item": ITEM, "from": "deadbeef", "to": "cafef00d" }),
    ));
    let noisy = stage_action(ITEM, STAGE, &with_rewrite).expect("resolves");
    assert_eq!(
        before.canonical_bytes().expect("canonical"),
        noisy.canonical_bytes().expect("canonical"),
        "a rewrite event appended to the journal may not move the tier-1 projection"
    );

    // ---- THE ADVERSARIAL STORE: tier-2 noise, in bulk, must change nothing -----------------------
    //
    // AC-184 names an adversarial-store variant that must produce identical output. The adversary
    // available to this crate is a store that returns the same tier-1 rows buried in tier-2 traffic.
    //
    // Since Step 7i (ADR-0163 decision 2) the Claude adapter appends a `rate_limit_reading` row per
    // dispatch through the child, so the noise carries one beside every `dispatch` row — Checkpoint
    // 7i's *"a tier-2 row added per reading moves no tier-1 projection"*. The payload is ADR-0163
    // decision 3's six-key shape (`cli_version` as `DispatchRecord` serialises it, AC-227), written
    // as a literal: `rk-kernel` may not depend on `rk-adapter-claude` (AC-484), so the producer's
    // `reading_payload` is not reachable from here.
    let mut adversarial: Vec<Event> = Vec::new();
    for event in &events {
        adversarial.push(Event::new(
            EventKind::Dispatch,
            serde_json::json!({ "item": ITEM, "activity_id_at_version": "noise@1", "attempt": 1, "step_id": "noise" }),
        ));
        adversarial.push(Event::new(
            EventKind::RateLimitReading,
            serde_json::json!({
                "windows": { "five_hour": 4, "seven_day": 37 },
                "utilization": null,
                "resets_at": { "five_hour": 1_788_370_800_i64, "seven_day": 1_788_408_000_i64 },
                "rate_limit_type": "five_hour",
                "item": ITEM,
                "cli_version": { "observed": "2.1.278" },
            }),
        ));
        adversarial.push(event.clone());
    }
    assert!(
        adversarial
            .iter()
            .any(|e| e.kind == EventKind::RateLimitReading),
        "PREMISE: the adversarial store carries no `rate_limit_reading` row, so the equality below \
         would say nothing about the one tier-2 kind Step 7i added a producer for"
    );
    let (adv_tier_one, _) = project(&adversarial);
    assert_eq!(
        adv_tier_one.len(),
        tier_one.len(),
        "PREMISE: the adversarial store carries the SAME tier-1 rows, only buried"
    );
    let adv = stage_action(ITEM, STAGE, &adversarial).expect("resolves");
    assert_eq!(
        before.canonical_bytes().expect("canonical"),
        adv.canonical_bytes().expect("canonical"),
        "AC-184's adversarial-store variant: identical output over a store that interleaves tier-2 \
         traffic around the same tier-1 rows"
    );

    // ---- THE CONTROL ----------------------------------------------------------------------------
    let elsewhere = vec![Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": STAGE, "to": "verify" }),
    )];
    let control = stage_action(ITEM, "verify", &elsewhere).expect("resolves");
    assert_ne!(
        before.canonical_bytes().expect("canonical"),
        control.canonical_bytes().expect("canonical"),
        "CONTROL: a different tier-1 journal must give a different projection, or every equality \
         above is satisfied by a constant"
    );

    println!(
        "rebuild_differential: tier1={} tier2={dropped} bytes={}",
        tier_one.len(),
        before.canonical_bytes().expect("canonical").len()
    );
}

/// **AC-161.** *"GIVEN a repository whose EventStore database file has been deleted / WHEN
/// `redkiln run <item>` is invoked / THEN the **stage-level next action** … is equal, under the
/// canonical serialiser, to the one selected before the deletion."*
///
/// The grain is the criterion's own: *"The comparison is at stage grain; a step-grain comparison
/// would assert what tier 2 cannot promise"* (ADR-0097 decision 1 narrowed AC-161 and AC-164 to it).
///
/// 🛑 **ARMED, and not reported as covered.** The *"event database"* deleted here is the collection
/// holding the journal, **dropped** rather than unlinked. `rk-kernel` may not depend on
/// `rk-adapter-sqlite` (AC-484), so creating and deleting a real `store.sqlite3` beside this test
/// would be a filesystem gesture producing the same green while measuring nothing. What is measured
/// is the operation this crate can perform and the one the criterion turns on: everything holding
/// tier-2 goes away, and the recomputation is handed the tier-1 rows alone. AC-161's *"from git
/// alone"* route and AC-164's `doctor`-classification half are neither of them here.
#[test]
fn stage_level_next_action() {
    let events = journal();
    let (tier_one, dropped) = project(&events);

    assert!(
        !tier_one.is_empty() && dropped >= 2,
        "PREMISE: the journal must really span both tiers. Got {} tier-1 and {dropped} tier-2",
        tier_one.len()
    );

    // ---- while the store holds EVERYTHING -------------------------------------------------------
    let store: Vec<Event> = events.clone();
    assert_eq!(
        store.len(),
        events.len(),
        "PREMISE: the event database really held the whole journal before it was deleted"
    );
    let full = stage_action(ITEM, STAGE, &store).expect("the tier-1 fold resolves");
    assert_ne!(
        full,
        StageAction::default(),
        "PREMISE: two empty stage actions are equal, and the comparisons below would hold over \
         two nothings"
    );

    // ---- AC-164's half: tier 2 is truncated -----------------------------------------------------
    assert!(
        tier_one.len() < events.len(),
        "PREMISE: something must actually be truncated"
    );
    let projected = stage_action(ITEM, STAGE, &tier_one).expect("resolves");
    assert_eq!(
        full.canonical_bytes().expect("canonical"),
        projected.canonical_bytes().expect("canonical"),
        "losing every tier-2 event does not change the stage-grain next action"
    );

    // ---- AC-161: the event database is gone ------------------------------------------------------
    drop(store);
    let after = stage_action(ITEM, STAGE, &tier_one).expect("resolves");
    assert_eq!(
        full.canonical_bytes().expect("canonical"),
        after.canonical_bytes().expect("canonical"),
        "AC-161: the stage-level next action after the deletion is byte-identical, under the \
         canonical serialiser, to the one selected before it"
    );

    // ---- THE CONTROL ----------------------------------------------------------------------------
    let elsewhere = vec![Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": STAGE, "to": "verify" }),
    )];
    let control = stage_action(ITEM, "verify", &elsewhere).expect("resolves");
    assert_ne!(
        full.canonical_bytes().expect("canonical"),
        control.canonical_bytes().expect("canonical"),
        "CONTROL: a different tier-1 journal must give a different stage action, or the equalities \
         above are satisfied by a constant"
    );

    println!(
        "stage_level_next_action: tier1={} tier2={dropped} bytes={}",
        tier_one.len(),
        full.canonical_bytes().expect("canonical").len()
    );
}
