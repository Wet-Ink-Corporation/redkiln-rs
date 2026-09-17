//! The circuit breaker — **AC-033, AC-034 and AC-035**, and the one half of AC-033 that has no
//! carrier.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | measured here | ARMED, not discharged |
//! | --- | --- | --- |
//! | **AC-033** | the K-consecutive-`Platform` counting rule, the reset rule, the refusal, and `K` read from a pack fixture and **refused as a code default** (`require_breaker_threshold`) | *"a durable breaker mark for the family is **written**"* — **no `EventKind`, no `RefusalClass` and no `StepError` variant can carry one**, and minting a kind outside a record is schema drift under ADR-0116. The mark is **derived** from the store here and never written |
//! | **AC-034** | the two-family property over two arbitrary distinct keys (one refusal, one success), and the lint banning module-level `static`/`lazy_static`/`OnceLock` breaker state — and, as of 2026-09-14, the same isolation over two **resolved** keys belonging to two roles of one activity | nothing |
//! | **AC-035** | a dropped-and-rebuilt state over the same journal still refuses; the lint that breaker state is constructed only from the store; the expiry compared against a **journaled** instant, and (2026-09-14) **pack-declared and refused as a code default** | the expiry's **value** (sized by no corpus figure — only its home is settled). *(**The real process-death grain left this cell on 2026-09-14.** It read *"and the **real process-death grain** — the grain here is a dropped value, and a `taskkill` boundary is 7d's"*. Operator decision **D-8** built the boundary in `tests/it/cross_process.rs`, whose `ac035_breaker_refusal_survives_a_real_process_death` re-execs this test binary and folds a journal a **dead child process** wrote — the criterion's own *"kill-and-re-enter test"*. What is still 7d's is narrower: *"`redkiln run` is invoked again"* as a **CLI** re-entry. The grain in THIS file is still a dropped value and it still claims nothing more.)* |
//!
//! # THE KEY IS BUILT AS OF 2026-09-14 — READ THIS BEFORE ADDING A CASE
//!
//! **This section was headed *"🛑 AND THE KEY ITSELF IS ARMED"* until 2026-09-14 and said**:
//!
//! > Every case below supplies its own family token through [`FamilyKey::supplied`]. **Nothing in
//! > this tree represents a model family** … and `crates/rk-kernel/src/catalog.rs`'s `family` is
//! > **ADR-0058's activity family** … A breaker keyed on it would trip globally with respect to the
//! > model and locally with respect to the method, which is the inverse of AC-034 on both axes —
//! > **and every assertion in this file would still pass.** The instrument cannot tell the two
//! > apart … The binding is 7c's and a record is OWED.
//!
//! **Operator decision D-7 built the binding here**: the pack declares `[role.<name>] family`, the
//! kernel resolves role → family at plan time, and the breaker keys on the resolved family. The
//! sentence that mattered most in the quoted paragraph — *"every assertion in this file would still
//! pass"* — **was true when it was written and is false now**, and making it false is the point of
//! the last test in this file. `breaker_key_is_the_packs_role_family_never_the_activity_family`
//! resolves two roles of **one** fixture activity that declares `family = "localisation"` and
//! asserts the two keys differ, that neither is `localisation`, that `localisation` is *refused* as
//! a role, and that tripping one role's family leaves the other proceeding. A resolver that read
//! `.family` fails four of those assertions.
//!
//! **The two cases above still supply their own arbitrary tokens, and that is deliberate.** AC-034's
//! property is over two *arbitrary* distinct keys and does not depend on what a key means; writing
//! them over resolved keys would make them depend on a pack fixture for no gain. What changed is
//! that the file no longer relies on the arbitrary-token form **alone**.
//!
//! **What is still OWED**, and it is why the new test's name carries no AC id: the key spellings
//! `[role.<name>] family` and `[breaker] expiry_secs` are a **reading** — the same gap
//! `src/workflow/envelope.rs`'s 🛑 already records for the three envelope keys — and whether two
//! model names are one family or two is a question for a pack author, whose shipped tables are
//! **Step 7e's** [measured 2026-09-14: `grep -rn role spikes/p9/packs/` → 0 hits across all twelve].

use rk_kernel::catalog::{Activity, Catalog};
use rk_kernel::select::Pack;
use rk_kernel::workflow::breaker::{
    BREAKER_ANCHORS, BreakerDecision, BreakerExpiry, BreakerLintViolation, BreakerState, FamilyKey,
    JournaledNow, MODULE_STATE_FORMS, NOT_FROM_STORE_FORMS, NOT_THE_ACTIVITY_FAMILY_FORMS,
    ROLE_RESOLUTION_ANCHORS, breaker_no_module_state_rule, breaker_state_from_store_rule,
    family_key_never_from_the_activity_catalog_rule,
};
use rk_kernel::workflow::envelope::{AttemptCap, BreakerThreshold, EnvelopeError};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

/// The pack that declares `K = 3` (and `N = 4`, and a token appetite — four different numbers).
const FIXTURE_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-envelope.toml");
/// The same pack with no `[breaker]` table.
const FIXTURE_NO_K: &str = include_str!("../fixtures/packs/runtime-no-k.toml");
/// **The pack that declares the KEY** — the `[role.<name>] family` table of 2026-09-14 (D-7) — and
/// the breaker envelope beside it: `K = 2`, the expiry `= 900`. Every number in it differs from
/// `runtime-envelope.toml`'s, so a test reading the wrong pack lands on a number this file names.
const FIXTURE_ROLE_FAMILIES: &str = include_str!("../fixtures/packs/role-families.toml");
/// The fixture activity carrying **both** families: `family = "localisation"` (ADR-0058's activity
/// family) and steps at two different roles.
const FIXTURE_ACTIVITY: &str = include_str!("../fixtures/catalog/differential-debugging.toml");

/// `crates/rk-kernel/src/select/pack.rs`'s own source — the **second** file the key's substitution
/// could be written in, because the role → family resolution lives there. Compiled in, never
/// opened, for [`BREAKER_SRC`]'s reason.
const PACK_SRC: &str = include_str!("../../src/select/pack.rs");

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
    // The grain here is a DROPPED VALUE, not a `taskkill`, and that is deliberate rather than a
    // limit. This comment continued "the real process-death grain is 7d's and is reported ARMED"
    // until 2026-09-14; operator decision D-8 made both clauses false, and the correction is stated
    // rather than the sentence deleted. `cross_process::ac035_breaker_refusal_survives_a_real_
    // process_death` re-execs this binary and folds what a DEAD process left on disk — AC-035's own
    // named instrument, a "kill-and-re-enter test". This case KEEPS the dropped-value grain because
    // it is the cheap in-process form of the same property and it runs under `breaker_ac033_ac035`,
    // a Checkpoint 7b name that spawns nothing. What this asserts is the property AC-035's THEN
    // turns on — that the refusal is a function of the store's contents and of nothing held in
    // memory.
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
    // already takes for its WALL-CLOCK counter (AC-024's *"reaches"*), which is this quantity's
    // kind; its attempts and token counters take `>` on AC-025's *"ran out"*. Both sides are
    // asserted.
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
        31,
        // DECIDED HERE 2026-09-17, when ADR-0123 minted the thirtieth and thirty-first: neither
        // `gate_answer_spent` (a refused duplicate answer) nor `gate_resumed` (resume telemetry)
        // is a breaker mark. AC-033's `written` half stays ARMED; only the count moved, 29 -> 31.
        "`EventKind` is closed at 31 (29 until ADR-0123) and none of them is a breaker mark: AC-033's `written` half is \
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

// ---- THE KEY ITSELF, decided 2026-09-14 ------------------------------------------------------

/// **The breaker's partition key is the PACK's `[role.<name>] family`, resolved from the step's
/// role — and it is never the activity's own `family`.**
///
/// > **Operator decision D-7, 2026-09-14** — *"The pack declares the family per role (extending
/// > design :480's 'tier policy per role'); the kernel resolves role → family at plan time and keys
/// > the breaker on the resolved family."*
///
/// **THE NAME CARRIES NO AC ID, AND THAT IS DELIBERATE.** It is the precedent
/// `appetite_raise_from_an_activity_result_is_refused` set in `tests/it/budget.rs` and the D-5, D-6
/// and D-10 phases of this same session followed: the key spellings this test reads —
/// `[role.<name>] family` and `[breaker] expiry_secs` — are a **reading**, and a record naming them
/// is **OWED** exactly as `crates/rk-kernel/src/workflow/envelope.rs`'s 🛑 already records for
/// `[retry] attempts_cap`, `[breaker] consecutive_platform_failures` and `appetite.tokens`. A green
/// here must not be read as a settlement no record has made. What *is* settled by AC-033/AC-034 and
/// is asserted under their own names above is the counting rule, the isolation and the refusal.
///
/// **WHAT THIS TEST EXISTS TO CATCH, WHICH NOTHING ABOVE COULD.** Until today every case in this
/// file supplied its own arbitrary token through [`FamilyKey::supplied`], and this file's own header
/// said what that cost: *"a breaker keyed on [the activity family] would trip globally with respect
/// to the model and locally with respect to the method, which is the inverse of AC-034 on both axes
/// — **and every assertion in this file would still pass.**"* The fixture activity below carries
/// **both** kinds of family at once — one `family = "localisation"` (ADR-0058's activity family, a
/// rank-escalation ladder) and two steps at two different roles — so the substitution the header
/// describes is now a **red** rather than a pass.
///
/// Six parts, each with its premise asserted first:
///
/// 1. the catalog carries the role and the loader now reads it (it parsed it **zero** times until
///    today), and the same activity carries the other `family`;
/// 2. the pack's table resolves role → family and is **not an identity map** — two roles share one
///    family, which is D-7's own reason for rejecting a breaker keyed on the *role*;
/// 3. **THE PIN** — the activity's `family` is refused as a role, and a pack with no role table
///    refuses every role;
/// 4. the breaker keyed on the resolved families isolates two steps of **one** activity, and the
///    activity-family token and the role token are both shown to key nothing;
/// 5. the expiry is pack-declared and refused as a code default, independently of `K`;
/// 6. the structural guard, over **both** files the substitution could be written in, with a
///    positive control per file.
#[test]
fn breaker_key_is_the_packs_role_family_never_the_activity_family() {
    // ---- 1. the role is on the step, and the ACTIVITY family is on the activity ---------------
    let activity = fixture_activity();
    assert_eq!(
        activity.id, "differential-debugging",
        "the premise: the fixture this test reasons about"
    );
    assert_eq!(
        activity.family.as_deref(),
        Some("localisation"),
        "THE PREMISE THE WHOLE TEST TURNS ON: this activity declares ADR-0058's ACTIVITY family, so \
         a resolver that read `.family` has something to read and would not simply fail"
    );
    let roles: Vec<Option<&str>> = activity
        .steps
        .iter()
        .map(|s| s.role.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(
        roles,
        vec![
            Some("investigator"),
            None,
            None,
            Some("reviewer"),
            None,
            None,
            None,
        ],
        "`catalog.rs` parsed `role` ZERO times until 2026-09-14 while the shipped catalog declared \
         one 185 times in 8 distinct values; a loader that still dropped the key reports seven \
         `None`s here"
    );
    // TWO DISTINCT ROLES ON ONE ACTIVITY — without this the isolation case in part 4 could be
    // satisfied by an activity that has only one role to resolve.
    assert_ne!(
        roles[0], roles[3],
        "the premise: one activity, one activity-family, TWO roles"
    );

    // ---- 2. the pack resolves role -> family, and the table is not an identity map -------------
    let role_pack = pack("role-families.toml", FIXTURE_ROLE_FAMILIES);
    let declared: Vec<(&str, &str)> = role_pack
        .role_families()
        .map(|(r, f)| (r.as_str(), f.as_str()))
        .collect();
    assert_eq!(
        declared,
        vec![
            ("implementer", "deliberative"),
            ("investigator", "deliberative"),
            ("reviewer", "adversarial"),
        ],
        "the premise: the table the fixture declares, in byte order"
    );

    let investigator = role_pack
        .require_family_for_role("investigator")
        .expect("the fixture pack binds `investigator`");
    let reviewer = role_pack
        .require_family_for_role("reviewer")
        .expect("the fixture pack binds `reviewer`");
    let implementer = role_pack
        .require_family_for_role("implementer")
        .expect("the fixture pack binds `implementer`");

    assert_ne!(
        investigator, reviewer,
        "two roles of ONE activity resolve to two families — a resolver keyed on the activity's \
         `family` returns one key for both and fails here"
    );
    assert_eq!(
        investigator, implementer,
        "N ROLES MAP TO ONE MODEL, and that is D-7's own reason for rejecting a breaker keyed on \
         the ROLE: a per-role key stops one role's share of traffic to a failing model while the \
         rest keeps flowing, and AC-034's `never trips globally` becomes true in a way that misses \
         the point"
    );
    assert_eq!(investigator.as_str(), "deliberative");
    assert_eq!(reviewer.as_str(), "adversarial");
    // ... and neither resolved key is the ROLE, nor the ACTIVITY family. A resolver that returned
    // its argument verbatim, or that read `.family`, lands on one of these three.
    for wrong in ["investigator", "reviewer", "localisation"] {
        assert_ne!(
            investigator,
            FamilyKey::supplied(wrong),
            "the resolved family is the pack's token, not `{wrong}`"
        );
        assert_ne!(
            reviewer,
            FamilyKey::supplied(wrong),
            "the resolved family is the pack's token, not `{wrong}`"
        );
    }
    assert_eq!(
        role_pack.family_for_role("investigator"),
        Some("deliberative")
    );
    assert_eq!(
        role_pack.family_for_role("localisation"),
        None,
        "the ACTIVITY family is not a role, so the raw accessor reports an absence for it"
    );

    // ---- 3. THE PIN: the activity family is refused as a role ---------------------------------
    let refused = role_pack
        .require_family_for_role("localisation")
        .expect_err("the activity family is not a role and this pack binds no such role");
    assert_eq!(
        refused,
        EnvelopeError::RoleFamilyNotDeclared {
            pack: "role-families-fixture".to_owned(),
            role: "localisation".to_owned(),
        },
        "the refusal names the pack AND the role, so `this pack has no role table` and `this role \
         is missing from it` are told apart"
    );
    assert_eq!(refused.as_str(), "role_family_not_declared");
    // ... and a pack declaring no `[role]` table at all refuses EVERY role — the absence is an
    // absence, never an identity map and never a wildcard.
    let no_roles = pack("runtime-envelope.toml", FIXTURE_ENVELOPE);
    assert_eq!(
        no_roles.role_families().count(),
        0,
        "the premise: this pack declares no role table"
    );
    for role in ["investigator", "reviewer", "implementer"] {
        assert_eq!(
            no_roles.require_family_for_role(role),
            Err(EnvelopeError::RoleFamilyNotDeclared {
                pack: "runtime-envelope-fixture".to_owned(),
                role: role.to_owned(),
            }),
            "an absent table is not a fallback to the role's own name, and not a wildcard: a \
             resolver that returned `FamilyKey::supplied(role)` here would make every pack in this \
             tree look bound"
        );
    }
    // THE CONTROL: it must still LOAD and must still report `K`, or the three refusals above are
    // passed by a loader that rejects every pack lacking a role table.
    assert_eq!(no_roles.id, "runtime-envelope-fixture");
    assert_eq!(
        no_roles
            .require_breaker_threshold()
            .map(BreakerThreshold::get),
        Ok(3),
        "only the role table is absent; the absences are independent"
    );

    // ---- 4. the breaker, keyed on the RESOLVED families ---------------------------------------
    let k = role_pack
        .require_breaker_threshold()
        .expect("the fixture pack declares `K`");
    assert_eq!(
        k,
        BreakerThreshold::declared(2),
        "the premise: this pack's `K` is 2, which is NOT `runtime-envelope.toml`'s 3 — a test \
         reading the wrong pack lands on the other number"
    );
    // Two steps of ONE activity, at two roles, resolving to two families. The investigator's family
    // takes K consecutive `Platform` failures; the reviewer's takes one.
    let journal = [
        platform_failure(investigator.as_str(), 2_000),
        platform_failure(reviewer.as_str(), 2_005),
        platform_failure(investigator.as_str(), 2_010),
    ];
    let state = BreakerState::from_events(&journal, k).expect("the journal folds");
    assert_eq!(
        state.consecutive(&investigator),
        2,
        "the premise: the fold counted the investigator's family's rows"
    );
    assert_eq!(state.consecutive(&reviewer), 1);

    let expiry = role_pack
        .require_breaker_expiry()
        .expect("the fixture pack declares the expiry");
    let at = now(2_100);
    assert!(
        state.decide(&investigator, at, expiry).refuses(),
        "the investigator's family reached `K` and is refused"
    );
    assert_eq!(
        state.decide(&reviewer, at, expiry),
        BreakerDecision::Proceed,
        "THE ISOLATION, over REAL resolved keys rather than two arbitrary tokens: the other step of \
         the SAME activity proceeds"
    );
    // ... and the two tokens a substitution would have produced key NOTHING. Both of these would be
    // open marks if the fold had been handed the activity family or the role.
    assert_eq!(
        state.decide(&FamilyKey::supplied("localisation"), at, expiry),
        BreakerDecision::Proceed,
        "the ACTIVITY family keys nothing here — a fold that read `.family` would have opened a \
         mark under this token and refused both steps"
    );
    assert_eq!(
        state.decide(&FamilyKey::supplied("investigator"), at, expiry),
        BreakerDecision::Proceed,
        "and neither does the ROLE"
    );

    // ---- 5. the expiry is PACK-DECLARED and refused as a code default -------------------------
    assert_eq!(
        expiry,
        BreakerExpiry::declared(900),
        "the premise: the pack's number, and it is none of `K`(2), `N`(5) or the appetite"
    );
    assert_eq!(
        role_pack.breaker_expiry(),
        Some(BreakerExpiry::declared(900))
    );
    // The boundary MOVES with the declared value, so the argument is read rather than ignored.
    assert!(
        state
            .decide(&investigator, now(2_010 + 899), expiry)
            .refuses(),
        "one second inside the pack's declared expiry, the mark still refuses"
    );
    assert!(
        !state
            .decide(&investigator, now(2_010 + 900), expiry)
            .refuses(),
        "at the pack's declared expiry the mark is found expired (AC-024's `reaches`)"
    );
    // A pack that declares `K` and NOT the expiry refuses by name, so the two `[breaker]` members
    // are independent and an accessor reading the wrong key lands on 3.
    assert_eq!(
        no_roles.breaker_expiry(),
        None,
        "an absent declaration is an absence, never a duration"
    );
    assert_ne!(
        no_roles.breaker_expiry(),
        Some(BreakerExpiry::declared(0)),
        "a defaulted expiry would most likely arrive as zero — a mark that is expired the instant \
         it opens"
    );
    let no_expiry = no_roles
        .require_breaker_expiry()
        .expect_err("a pack declaring no expiry cannot drive the breaker's expiry comparison");
    assert_eq!(
        no_expiry,
        EnvelopeError::BreakerExpiryNotDeclared {
            pack: "runtime-envelope-fixture".to_owned()
        }
    );
    assert_eq!(no_expiry.as_str(), "breaker_expiry_not_declared");

    // ---- 6. the structural guard, over BOTH files it could be written in -----------------------
    // The behavioural parts above catch a substitution that has already been made. This catches the
    // route: `crate::catalog` reachable from either the fold or the resolution.
    let breaker_src = code_only(BREAKER_SRC);
    let pack_src = code_only(PACK_SRC);
    for (label, src, anchors) in [
        ("src/workflow/breaker.rs", &breaker_src, &BREAKER_ANCHORS),
        ("src/select/pack.rs", &pack_src, &ROLE_RESOLUTION_ANCHORS),
    ] {
        for anchor in anchors.as_slice() {
            assert!(
                src.contains(anchor),
                "the anchor `{anchor}` is absent from {label}, so the ban below would be a vacuous \
                 pass"
            );
        }
        family_key_never_from_the_activity_catalog_rule(src, anchors.as_slice()).unwrap_or_else(
            |v| {
                panic!("{label} reaches the activity catalog: {v:?}");
            },
        );
        // THE POSITIVE CONTROL, per file. Without it "the rule passed" is indistinguishable from
        // "the rule matched nothing" (Halt 8, one level up).
        for form in NOT_THE_ACTIVITY_FAMILY_FORMS {
            let poisoned =
                format!("{src}\nfn poisoned(a: &crate{form}::Activity) {{ a.family; }}\n");
            let found =
                family_key_never_from_the_activity_catalog_rule(&poisoned, anchors.as_slice())
                    .expect_err("the rule must see a path into the activity catalog");
            assert!(
                found.iter().any(|v| matches!(
                    v,
                    BreakerLintViolation::ReadsTheActivityCatalog { form: f, .. } if *f == form
                )),
                "the rule saw something in {label}, but not `{form}`: {found:?}"
            );
            assert!(
                found
                    .iter()
                    .all(|v| v.clause() == "reads-the-activity-catalog"),
                "a clean file plus one banned form yields exactly that clause: {found:?}"
            );
        }
        let empty = family_key_never_from_the_activity_catalog_rule("", anchors.as_slice())
            .expect_err("an emptied file must fail on the anchor rather than pass an absence");
        assert!(
            empty.iter().any(|v| v.clause() == "anchor-missing"),
            "an emptied file fails on the ANCHOR, not on an absence: {empty:?}"
        );
    }
}

/// The fixture activity that carries **both** families — `family = "localisation"` (ADR-0058's
/// activity family) and two steps at `role = "investigator"` and `role = "reviewer"`.
///
/// **Compiled in, never opened**, for [`BREAKER_SRC`]'s reason.
///
/// # Panics
///
/// With the loader's own error, so a fixture that stops parsing names its reason.
fn fixture_activity() -> Activity {
    let catalog = Catalog::from_sources(&[("differential-debugging.toml", FIXTURE_ACTIVITY)])
        .unwrap_or_else(|e| panic!("the differential-debugging fixture loads: {e}"));
    catalog
        .activities()
        .first()
        .cloned()
        .expect("the fixture holds one activity")
}

/// A journaled instant. There is no clock in this file either.
const fn now(secs: i64) -> JournaledNow {
    JournaledNow::from_journal(Timestamp::from_unix_seconds(secs))
}
