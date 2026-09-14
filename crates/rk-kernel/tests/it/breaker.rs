//! The circuit breaker — **AC-033, AC-034 and AC-035**, and the one half of AC-033 that has no
//! carrier.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | measured here | ARMED, not discharged |
//! | --- | --- | --- |
//! | **AC-033** | the K-consecutive-`Platform` counting rule, the reset rule, the refusal, and `K` read from a pack fixture and **refused as a code default** (`require_breaker_threshold`) | *"a durable breaker mark for the family is **written**"* — **no `EventKind`, no `RefusalClass` and no `StepError` variant can carry one**, and minting a kind outside a record is schema drift under ADR-0116. The mark is **derived** from the store here and never written |
//! | **AC-034** | the two-family property over two arbitrary distinct keys (one refusal, one success), and the lint banning module-level `static`/`lazy_static`/`OnceLock` breaker state | nothing |
//! | **AC-035** | a dropped-and-rebuilt state over the same journal still refuses; the lint that breaker state is constructed only from the store; the expiry compared against a **journaled** instant | the **value and the home of the expiry** (§7b.10's two lists contain neither) and the **real process-death grain** — the grain here is a dropped value, and a `taskkill` boundary is 7d's |
//!
//! # 🛑 AND THE KEY ITSELF IS ARMED — READ THIS BEFORE ADDING A CASE
//!
//! Every case below supplies its own family token through [`FamilyKey::supplied`]. **Nothing in
//! this tree represents a model family** [measured 2026-09-14: `grep -rn
//! "family\|ModelFamily\|model_tier" crates/rk-types/src/ crates/rk-ports/src/` → 0 hits, exit 1],
//! and `crates/rk-kernel/src/catalog.rs`'s `family` is **ADR-0058's activity family** — a
//! rank-escalation ladder whose members would typically all run on the *same* model. A breaker keyed
//! on it would trip globally with respect to the model and locally with respect to the method, which
//! is the inverse of AC-034 on both axes — **and every assertion in this file would still pass.**
//! The instrument cannot tell the two apart, which is why the substitution is banned in prose here
//! and by construction in `src/workflow/breaker.rs` (one constructor, taking the token from the
//! caller). The binding is 7c's and a record is OWED.

use rk_kernel::select::Pack;
use rk_kernel::workflow::breaker::{
    BREAKER_ANCHORS, BreakerDecision, BreakerExpiry, BreakerLintViolation, BreakerState, FamilyKey,
    JournaledNow, MODULE_STATE_FORMS, NOT_FROM_STORE_FORMS, breaker_no_module_state_rule,
    breaker_state_from_store_rule,
};
use rk_kernel::workflow::envelope::{AttemptCap, BreakerThreshold, EnvelopeError};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

/// The pack that declares `K = 3` (and `N = 4`, and a token appetite — four different numbers).
const FIXTURE_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-envelope.toml");
/// The same pack with no `[breaker]` table.
const FIXTURE_NO_K: &str = include_str!("../fixtures/packs/runtime-no-k.toml");

/// The breaker module's own source, for the two lints AC-034 and AC-035 name.
///
/// **Compiled in, never opened.** A fixture loaded from a path is a `Result` an implementer can
/// `unwrap_or_default()` into nothing, and a rule over nothing passes
/// (`tests/it/support.rs`:6-12).
const BREAKER_SRC: &str = include_str!("../../src/workflow/breaker.rs");

/// Load one pack fixture.
///
/// # Panics
///
/// With the loader's own error, so a fixture that stops parsing names its reason.
fn pack(name: &str, src: &str) -> Pack {
    Pack::from_toml(name, src).unwrap_or_else(|e| panic!("the {name} fixture loads: {e}"))
}

/// A `step_failed { class: platform, family, at }` row — AC-033's GIVEN, one occurrence.
fn platform_failure(family: &str, at: i64) -> Event {
    Event::new(
        EventKind::StepFailed,
        serde_json::json!({ "class": "platform", "family": family, "at": at }),
    )
}

/// A `step_failed` row of some **other** class. The breaker keys on `Platform` **only** (AC-033's
/// GIVEN) and no record licenses widening it, so this row must not count toward a threshold — and
/// it must reset the consecutive run, because the run of *`Platform`* failures was broken.
fn other_failure(family: &str, class: &str, at: i64) -> Event {
    Event::new(
        EventKind::StepFailed,
        serde_json::json!({ "class": class, "family": family, "at": at }),
    )
}

/// An `activity_committed` row — ADR-0022's activation record, the only success this crate can see.
fn committed(family: &str) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({ "family": family }),
    )
}

/// A row that is not family-scoped at all, so the fold must ignore it without failing.
fn unrelated() -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "triage", "to": "implement" }),
    )
}

/// Strip comment lines before a source lint runs.
///
/// `src/workflow/breaker.rs`'s own header quotes AC-034, which names all three banned forms. A lint
/// that matched prose would be measuring its own documentation, and would go red on the file that
/// carries the criterion it enforces.
fn code_only(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

// ---- AC-033 and AC-035 ---------------------------------------------------------------------

/// **AC-033 + AC-035**, the runbook's own name (§7b.13:1169-1170, §7b.14's third loop).
///
/// Six obligations, in order:
///
/// 1. `K` comes from the **pack** and is refused as a code default. The envelope fixture declares
///    `K = 3` against `N = 4`, so a read of the wrong key returns a number this test names.
/// 2. The count is **consecutive**: `K - 1` failures do not open, `K` do, and a success in the
///    middle resets — so a counter that summed all `Platform` failures for a family would open one
///    case early and is caught.
/// 3. The trip condition is `Platform` **only**: a `transport` failure does not count toward the
///    threshold.
/// 4. AC-035's survival: the state is **dropped** and rebuilt over the same journal, and the
///    refusal stands. The control is `BreakerState::default()` — a fresh process with no store —
///    which proceeds, so the refusal demonstrably comes from the journal and not from the type.
/// 5. AC-035's expiry, compared against a **journaled** instant, asserted on both sides of the
///    boundary and below it.
/// 6. AC-035's own named lint, *"a lint that breaker state is constructed only from the store"*,
///    with a positive control: the same rule over a source carrying a `Mutex<` must fail.
///
/// **What is NOT asserted, and must not be read as covered:** AC-033's *"a durable breaker mark …
/// is **written**"*. The mark is derived; no kind carries it; `EventKind::ALL`'s length is pinned
/// below so a minted thirtieth kind goes red here rather than passing as the missing carrier.
#[test]
fn breaker_ac033_ac035() {
    // ---- 1. `K` is a pack value, asserted present by pack validation (AC-033's Verified by) ----
    let declared = pack("runtime-envelope.toml", FIXTURE_ENVELOPE);
    let k = declared
        .require_breaker_threshold()
        .expect("the envelope pack declares `[breaker] consecutive_platform_failures`");
    assert_eq!(k, BreakerThreshold::declared(3), "`K` is the pack's number");
    // THE PREMISE: `K` and `N` are DIFFERENT numbers in this fixture, so every assertion below
    // that turns on 3 would fail against an accessor that read `[retry] attempts_cap`.
    assert_ne!(
        k.get(),
        declared
            .require_attempt_cap()
            .expect("the envelope pack declares `N`")
            .get(),
        "`K` = 3 and `N` = 4: a test written over a pack declaring 3 and 3 would not discriminate"
    );

    // A pack that declares no `K` REFUSES, and is not handed one.
    let absent = pack("runtime-no-k.toml", FIXTURE_NO_K);
    assert_eq!(
        absent.breaker_threshold(),
        None,
        "an absent declaration is an absence, never a number"
    );
    assert_ne!(
        absent.breaker_threshold(),
        Some(BreakerThreshold::declared(0)),
        "a defaulted `K` would most likely arrive as zero — a breaker open before anything failed"
    );
    assert_ne!(
        absent.breaker_threshold(),
        Some(BreakerThreshold::declared(4)),
        "4 is `N`, which this pack DOES declare: an accessor reading `[retry] attempts_cap` lands \
         here"
    );
    let err = absent
        .require_breaker_threshold()
        .expect_err("a pack declaring no `K` cannot drive the breaker");
    assert_eq!(
        err,
        EnvelopeError::BreakerThresholdNotDeclared {
            pack: "runtime-no-k-fixture".to_owned()
        },
        "the refusal names the pack, so an operator has a file to open"
    );
    assert_eq!(err.as_str(), "breaker_threshold_not_declared");
    // THE CONTROL: the no-`K` pack must still LOAD and must still report `N`, or these assertions
    // are passed by a loader that rejects every pack lacking an envelope.
    assert_eq!(absent.id, "runtime-no-k-fixture");
    assert_eq!(
        absent.attempt_cap().map(AttemptCap::get),
        Some(4),
        "only `K` is absent; the two absences are independent"
    );

    let family = FamilyKey::supplied("family-under-test");

    // ---- 2. K - 1 consecutive failures do NOT open --------------------------------------------
    let short = [
        platform_failure(family.as_str(), 1_000),
        platform_failure(family.as_str(), 1_010),
    ];
    let state = BreakerState::from_events(&short, k).expect("the journal folds");
    assert_eq!(
        state.consecutive(&family),
        2,
        "the premise: two failures were counted"
    );
    assert!(
        state.mark(&family).is_none(),
        "K - 1 failures must not open the breaker — a breaker that tripped at a hard-coded number \
         below the pack's `K` would open here"
    );
    assert_eq!(
        state.decide(&family, now(2_000), BreakerExpiry::declared(600)),
        BreakerDecision::Proceed
    );

    // ---- ... and K consecutive DO ---------------------------------------------------------
    let tripped = [
        platform_failure(family.as_str(), 1_000),
        platform_failure(family.as_str(), 1_010),
        platform_failure(family.as_str(), 1_020),
    ];
    let state = BreakerState::from_events(&tripped, k).expect("the journal folds");
    let mark = state
        .mark(&family)
        .expect("K consecutive `Platform` failures open the breaker for this family");
    assert_eq!(mark.family(), &family);
    assert_eq!(mark.consecutive(), 3);
    assert_eq!(
        mark.opened_at(),
        Timestamp::from_unix_seconds(1_020),
        "the mark's instant is READ OFF the failure that opened it, never minted here (AC-001)"
    );
    let decision = state.decide(&family, now(1_100), BreakerExpiry::declared(600));
    assert_eq!(
        decision,
        BreakerDecision::Refused {
            family: family.clone(),
            consecutive: 3,
        },
        "AC-033's THEN, first conjunct: the next dispatch for that family is refused"
    );
    assert!(decision.refuses());
    assert_eq!(decision.as_str(), "refused");

    // THE SURVEY (House Shape C): a state with no mark reads exactly like a journal the fold never
    // looked at, so the counters travel beside the verdict and are asserted.
    let survey = state.survey();
    assert_eq!(survey.events_folded, 3);
    assert_eq!(survey.family_scoped, 3);
    assert_eq!(survey.platform_failures, 3);
    assert_eq!(survey.resets, 0);

    // ---- the reset rule: "consecutive", not "total" -------------------------------------------
    // FIVE `Platform` failures for one family, in runs of two, two and one. A counter that summed
    // them would stand at 5 — well past `K` = 3 — and would have opened; AC-033's GIVEN says
    // CONSECUTIVE, and no run here reaches three.
    let with_reset = [
        platform_failure(family.as_str(), 1_000),
        platform_failure(family.as_str(), 1_010),
        committed(family.as_str()),
        platform_failure(family.as_str(), 1_030),
        platform_failure(family.as_str(), 1_040),
        committed(family.as_str()),
        platform_failure(family.as_str(), 1_050),
    ];
    let state = BreakerState::from_events(&with_reset, k).expect("the journal folds");
    assert_eq!(
        state.survey().platform_failures,
        5,
        "the premise: five rows"
    );
    assert_eq!(state.survey().resets, 2);
    assert_eq!(state.consecutive(&family), 1);
    assert!(
        state.mark(&family).is_none(),
        "five `Platform` failures broken into runs of 2, 2 and 1 are not K CONSECUTIVE failures — \
         a total counter opens here and AC-033's GIVEN says consecutive"
    );

    // ---- 3. the breaker keys on `Platform` ONLY -----------------------------------------------
    // AC-033's GIVEN is `Platform`; no record licenses widening it to `Transport` or `Timeout`,
    // both of which the shipped `retry_class()` already sends to `Backoff` — the outer retry
    // handles them and the breaker does not.
    //
    // THE DISCRIMINATOR: K failures of another class open nothing at all. A breaker that counted
    // any failure would open here, and every other assertion in this test would still pass.
    let never_platform = [
        other_failure(family.as_str(), "transport", 1_000),
        other_failure(family.as_str(), "timeout", 1_010),
        other_failure(family.as_str(), "schema", 1_020),
        other_failure(family.as_str(), "non_zero", 1_030),
    ];
    let state = BreakerState::from_events(&never_platform, k).expect("the journal folds");
    assert_eq!(
        state.survey().family_scoped,
        4,
        "the premise: the fold read all four rows"
    );
    assert_eq!(
        state.survey().platform_failures,
        0,
        "none of the four is a `Platform` failure"
    );
    assert!(
        state.mark(&family).is_none(),
        "four non-`Platform` failures past `K` = 3 open nothing: AC-033's GIVEN is `Platform`"
    );

    // ... and a non-`Platform` row does not prevent a later run from opening.
    let mixed = [
        platform_failure(family.as_str(), 1_000),
        other_failure(family.as_str(), "transport", 1_010),
        platform_failure(family.as_str(), 1_020),
        platform_failure(family.as_str(), 1_030),
        platform_failure(family.as_str(), 1_040),
    ];
    let state = BreakerState::from_events(&mixed, k).expect("the journal folds");
    assert_eq!(
        state.survey().platform_failures,
        4,
        "the transport row is not counted toward the threshold"
    );
    assert_eq!(
        state.consecutive(&family),
        3,
        "and the run AFTER it is three, so the breaker opens"
    );
    let mark = state
        .mark(&family)
        .expect("the run after the reset reaches K");
    assert_eq!(
        mark.opened_at(),
        Timestamp::from_unix_seconds(1_040),
        "the mark opens on the failure that COMPLETED the run, not on the first of the journal"
    );
    // ... and the same journal with the transport row LAST leaves a run of one.
    let broken = [
        platform_failure(family.as_str(), 1_000),
        platform_failure(family.as_str(), 1_010),
        other_failure(family.as_str(), "timeout", 1_015),
        platform_failure(family.as_str(), 1_020),
    ];
    let state_broken = BreakerState::from_events(&broken, k).expect("the journal folds");
    assert_eq!(state_broken.consecutive(&family), 1);
    assert!(
        state_broken.mark(&family).is_none(),
        "a non-`Platform` failure breaks the consecutive run of `Platform` failures"
    );
    assert_eq!(state_broken.survey().resets, 1);

    // ---- an open mark STANDS through a success, and REFRESHES when the family re-trips --------
    // AC-035's THEN is *"dispatches to that family are still refused UNTIL THE MARK IS READ AND
    // FOUND EXPIRED"* — not "until the family next succeeds". Clearing the mark on an
    // `activity_committed` would be a route around the expiry, and a half-open probe policy is in
    // no record. Both readings are stated in `src/workflow/breaker.rs`'s header.
    let recovers_then_trips_again = [
        platform_failure(family.as_str(), 1_000),
        platform_failure(family.as_str(), 1_010),
        platform_failure(family.as_str(), 1_020),
        committed(family.as_str()),
        platform_failure(family.as_str(), 2_000),
        platform_failure(family.as_str(), 2_010),
        platform_failure(family.as_str(), 2_020),
    ];
    let state =
        BreakerState::from_events(&recovers_then_trips_again, k).expect("the journal folds");
    assert_eq!(
        state.consecutive(&family),
        3,
        "the premise: the second run also reaches `K`"
    );
    let mark = state.mark(&family).expect("the mark stands");
    assert_eq!(
        mark.opened_at(),
        Timestamp::from_unix_seconds(2_020),
        "the mark records the MOST RECENT failure that completed a run of `K`: a re-trip restarts \
         the expiry clock, and keeping the first instant would let a stale mark expire a breaker \
         that has just re-opened"
    );
    // ... and the success alone, with no second run, leaves the mark in place.
    let recovers = [
        platform_failure(family.as_str(), 1_000),
        platform_failure(family.as_str(), 1_010),
        platform_failure(family.as_str(), 1_020),
        committed(family.as_str()),
    ];
    let state = BreakerState::from_events(&recovers, k).expect("the journal folds");
    assert_eq!(
        state.consecutive(&family),
        0,
        "the success reset the consecutive run"
    );
    assert!(
        state
            .decide(&family, now(1_100), BreakerExpiry::declared(600))
            .refuses(),
        "AC-035: the refusal stands until the mark is read and FOUND EXPIRED — a success is not an \
         expiry, and a fold that cleared the mark here would reopen the family 80 seconds into a \
         600-second mark"
    );
    assert_eq!(
        state
            .mark(&family)
            .expect("the mark is still there")
            .opened_at(),
        Timestamp::from_unix_seconds(1_020)
    );

    // A row that is not family-scoped is ignored rather than refused or counted.
    let with_noise = [
        unrelated(),
        platform_failure(family.as_str(), 1_000),
        unrelated(),
        platform_failure(family.as_str(), 1_010),
        platform_failure(family.as_str(), 1_020),
    ];
    let state_noise = BreakerState::from_events(&with_noise, k).expect("the journal folds");
    assert_eq!(state_noise.survey().events_folded, 5);
    assert_eq!(state_noise.survey().family_scoped, 3);
    assert!(state_noise.mark(&family).is_some());

    // ---- 4. AC-035: the refusal survives the state being dropped ------------------------------
    // The grain is a DROPPED VALUE, not a `taskkill`: the real process-death grain is 7d's and is
    // reported ARMED. What this asserts is the property AC-035's THEN turns on — that the refusal
    // is a function of the store's contents and of nothing held in memory.
    let first = BreakerState::from_events(&tripped, k).expect("the journal folds");
    let refused_before = first.decide(&family, now(1_100), BreakerExpiry::declared(600));
    drop(first);
    let second = BreakerState::from_events(&tripped, k).expect("the journal folds again");
    assert_eq!(
        second.decide(&family, now(1_100), BreakerExpiry::declared(600)),
        refused_before,
        "AC-035: a second lifetime over the same store decides the same way"
    );
    // THE CONTROL, and it is the discriminating one. A fresh state with NO store proceeds — so the
    // refusal above is demonstrably derived from the journal rather than being what this type does.
    assert_eq!(
        BreakerState::default().decide(&family, now(1_100), BreakerExpiry::declared(600)),
        BreakerDecision::Proceed,
        "an empty fold refuses nothing; without this, `decide` returning `Refused` always would \
         pass every assertion above"
    );
    assert_eq!(BreakerState::default().survey().events_folded, 0);

    // ---- 5. AC-035's expiry, against a JOURNALED instant ---------------------------------------
    // The expiry VALUE and its HOME are settled by no record: §7b.10:852-856 sorts every Slice-4
    // number with no measured value into pack-declared and kernel-constant, and the breaker's
    // expiry is in NEITHER list. It arrives here as a declared argument, and this crate names no
    // number. The BOUNDARY is `elapsed >= expiry`, the reading `budget.rs`'s `exhausted_counter`
    // already takes for a declared quantity, and both sides are asserted.
    let expiry = BreakerExpiry::declared(600);
    assert_eq!(expiry.get(), 600, "the premise: the test declared it");
    let opened = 1_020_i64;
    assert!(
        second.decide(&family, now(opened + 599), expiry).refuses(),
        "one second inside the declared expiry, the mark still refuses"
    );
    assert!(
        !second.decide(&family, now(opened + 600), expiry).refuses(),
        "at the declared expiry the mark is found expired"
    );
    assert!(!second.decide(&family, now(opened + 601), expiry).refuses());
    assert!(
        second.decide(&family, now(opened - 1), expiry).refuses(),
        "a journaled instant BEFORE the mark does not expire it — a signed subtraction that \
         wrapped or saturated the wrong way would let a stale clock reopen every family"
    );
    // A different declared expiry moves the boundary, so the argument is read rather than ignored.
    assert!(
        second
            .decide(&family, now(opened + 600), BreakerExpiry::declared(5_000))
            .refuses(),
        "under a longer declared expiry the same instant is still inside it"
    );

    // ---- 6. AC-035's own named lint, with its positive control --------------------------------
    let src = code_only(BREAKER_SRC);
    for anchor in BREAKER_ANCHORS {
        assert!(
            src.contains(anchor),
            "the anchor `{anchor}` is absent from src/workflow/breaker.rs, so every absence below \
             would be a vacuous pass"
        );
    }
    breaker_state_from_store_rule(&src)
        .expect("breaker state is constructed only from the store (AC-035)");
    // THE POSITIVE CONTROL: the rule must FAIL on a source that carries one of the banned forms,
    // or "it passed" is indistinguishable from "it matched nothing" (Halt 8, one level up).
    for form in NOT_FROM_STORE_FORMS {
        let poisoned = format!("{src}\nfn poisoned(_x: {form}u8>) {{}}\n");
        let found = breaker_state_from_store_rule(&poisoned)
            .expect_err("the rule must see a state-carrying form");
        assert!(
            found
                .iter()
                .any(|v| matches!(v, BreakerLintViolation::StateNotFromStore { form: f, .. } if *f == form)),
            "the rule saw something, but not `{form}`: {found:?}"
        );
    }
    let empty = breaker_state_from_store_rule("")
        .expect_err("an emptied file must fail rather than pass four absences");
    assert!(
        empty.iter().any(|v| v.clause() == "anchor-missing"),
        "an emptied file fails on the ANCHOR, not on an absence: {empty:?}"
    );

    // No clock anywhere in the module the expiry comparison lives in. The module-level
    // `#![deny(clippy::disallowed_methods)]` is the enforcing instrument; this is the readable one.
    for banned in ["SystemTime", "Instant::", "std::time", "sleep"] {
        assert!(
            !src.contains(banned),
            "src/workflow/breaker.rs reads `{banned}`: AC-035's Note says the expiry comparison \
             reads the current time as a journaled activity result (AC-001), never from the \
             workflow's own clock"
        );
    }

    // ---- AC-033's SECOND CONJUNCT IS ARMED, AND THIS IS THE PIN THAT KEEPS IT HONEST ----------
    // "a durable breaker mark for the family is WRITTEN". No `EventKind` carries one; the enum is
    // closed at 29 and ADR-0116's rule is that "the kind is minted by record or not at all". If a
    // thirtieth kind appears, this assertion goes red and whoever added it must decide whether it
    // is the breaker's carrier — rather than the gap quietly closing or quietly widening.
    assert_eq!(
        EventKind::ALL.len(),
        29,
        "`EventKind` is closed at 29 and none of them is a breaker mark: AC-033's `written` half is \
         ARMED, NOT DISCHARGED, and no kind is minted at Slice 4d"
    );
}

// ---- AC-034 ---------------------------------------------------------------------------------

/// **AC-034**, which `breaker_ac033_ac035`'s name omits — and *"a name that omits an id does not
/// pin it"* (`00-preamble.md`:188-189).
///
/// > GIVEN an open breaker on one model family / WHEN a dispatch is requested for a different
/// > family / THEN it proceeds; the breaker never trips globally.
/// > *Verified by:* a two-family unit test asserting **one refusal and one success**; a lint banning
/// > module-level `static`/`lazy_static`/`OnceLock` breaker state, so process-wide breaker state
/// > cannot be introduced.
///
/// Both instruments, under one name. The two-family half is a property over two **arbitrary**
/// distinct keys — nothing here depends on what a key means — and the lint half runs over
/// `src/workflow/breaker.rs`'s own source with a positive control per banned form.
#[test]
fn ac034_breaker_never_trips_globally() {
    let declared = pack("runtime-envelope.toml", FIXTURE_ENVELOPE);
    let k = declared
        .require_breaker_threshold()
        .expect("the envelope pack declares `K`");

    let a = FamilyKey::supplied("family-a");
    let b = FamilyKey::supplied("family-b");
    let c = FamilyKey::supplied("family-c");
    assert_ne!(a, b, "the premise: two distinct keys");

    // Family A trips. Family B takes K - 1 failures interleaved with A's, so B's count is its own
    // and is not a count of nothing. Family C only ever succeeds.
    let journal = [
        platform_failure(a.as_str(), 1_000),
        platform_failure(b.as_str(), 1_001),
        platform_failure(a.as_str(), 1_010),
        platform_failure(b.as_str(), 1_011),
        committed(c.as_str()),
        platform_failure(a.as_str(), 1_020),
    ];
    let state = BreakerState::from_events(&journal, k).expect("the journal folds");

    // THE PREMISES: the fold saw both families' rows, and it counted them separately.
    assert_eq!(state.survey().platform_failures, 5);
    assert_eq!(state.consecutive(&a), 3);
    assert_eq!(state.consecutive(&b), 2);
    assert_eq!(state.consecutive(&c), 0);

    let now = now(1_100);
    let expiry = BreakerExpiry::declared(600);

    // ---- ONE REFUSAL AND ONE SUCCESS ----------------------------------------------------------
    let decisions = [
        state.decide(&a, now, expiry),
        state.decide(&b, now, expiry),
        state.decide(&c, now, expiry),
    ];
    let refused = decisions.iter().filter(|d| d.refuses()).count();
    assert_eq!(
        refused, 1,
        "exactly one of three families is refused: a breaker that tripped globally would refuse \
         three, and one that never tripped would refuse none — {decisions:?}"
    );
    assert_eq!(
        decisions[0],
        BreakerDecision::Refused {
            family: a.clone(),
            consecutive: 3,
        },
        "the refusal names the family it is for, never `the breaker`"
    );
    assert_eq!(
        decisions[1],
        BreakerDecision::Proceed,
        "AC-034's THEN: a dispatch for a different family proceeds"
    );
    assert_eq!(decisions[2], BreakerDecision::Proceed);

    // ... and the open set is exactly {A}. `open_families` is what a caller would read to answer
    // "is the breaker open?", and answering that without a key is the global breaker AC-034 bans.
    assert_eq!(
        state.open_families(),
        vec![&a],
        "one open mark, keyed, in byte order"
    );

    // A family the journal never mentioned proceeds, so the partition is total rather than a set
    // of the keys that happen to appear.
    assert_eq!(
        state.decide(&FamilyKey::supplied("never-seen"), now, expiry),
        BreakerDecision::Proceed
    );

    // ---- THE LINT HALF ------------------------------------------------------------------------
    let src = code_only(BREAKER_SRC);
    for anchor in BREAKER_ANCHORS {
        assert!(
            src.contains(anchor),
            "the anchor `{anchor}` is absent, so the four absences below assert nothing"
        );
    }
    breaker_no_module_state_rule(&src)
        .expect("no module-level `static`/`lazy_static`/`OnceLock` breaker state (AC-034)");

    // THE POSITIVE CONTROL, one per banned form. Without it a rule that matched nothing at all
    // would be indistinguishable from a clean file.
    for form in MODULE_STATE_FORMS {
        let poisoned = format!("{src}\n{form}BREAKER_STATE: u8 = 0;\n");
        let found = breaker_no_module_state_rule(&poisoned)
            .expect_err("the rule must see module-level state");
        assert!(
            found
                .iter()
                .any(|v| matches!(v, BreakerLintViolation::ModuleLevelState { form: f, .. } if *f == form)),
            "the rule saw something, but not `{form}`: {found:?}"
        );
        assert!(
            found.iter().all(|v| v.clause() == "module-level-state"),
            "a clean file plus one banned form yields exactly that clause: {found:?}"
        );
    }
    // Every violation is collected, never the first only (house Shape B).
    let three = format!("{src}\nstatic A: u8 = 0;\nthread_local! {{}}\nstatic B: u8 = 1;\n");
    let found = breaker_no_module_state_rule(&three).expect_err("three forms, three violations");
    assert_eq!(
        found.len(),
        3,
        "a rule that returned early would report one: {found:?}"
    );

    let empty = breaker_no_module_state_rule("")
        .expect_err("an emptied file must fail on the anchor, not pass four absences");
    assert!(empty.iter().any(|v| v.clause() == "anchor-missing"));
}

/// A journaled instant. There is no clock in this file either.
const fn now(secs: i64) -> JournaledNow {
    JournaledNow::from_journal(Timestamp::from_unix_seconds(secs))
}
