//! **The retry policy** — AC-026, AC-027, AC-028, AC-029 and AC-032, over ADR-0112's eleven
//! variants.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT — READ THIS BEFORE READING A GREEN
//!
//! | criterion | measured here | not here |
//! | --- | --- | --- |
//! | **AC-026** | ADR-0122 decision 1's charge unit (one charge per recorded `system/api_retry` **event**, never the terminal `attempt` value) and decision 2's dispatch-scoped suppression, over a committed fixture | **the register's own instrument** — *"rows from `spikes/results/e3.jsonl` (and `p7`/`s-var`) **at the corpus pin**"*. That half is [`inner_retry_ac026_at_the_corpus_pin`], behind `--features corpus`. The unfeatured half below pins the **accounting rule** and is not AC-026's named instrument |
//! | **AC-027** | the four classes the criterion names, table-driven: `redispatches() == 0` **and** the typed error surfaced, per class | nothing is claimed for `Cancelled` or `NonZero`, which ADR-0095's table also marks `never` and **no criterion's THEN names** |
//! | **AC-028** | that the two policy `match`es over `StepError` are exhaustive and carry no wildcard — a **source lint** with anchors, plus `tests/ui/ac028_steperror_wildcard.rs` | see [`ac028_policy_match_has_no_wildcard`]'s doc: the literal *"the wildcard cannot be reintroduced"* is **not constructible** as a compile-fail case and the substitute is named there |
//! | **AC-029** | the two kernel constants, the exponent, the cap, full jitter's envelope **and its two endpoints**, determinism across two evaluations, and the cap read from a **pack fixture** | the **production jitter draw is unbound** — [`rk_kernel::retry::JitterDraw`] is a parameter and this crate mints none. AC-029's *"full jitter"* conjunct is discharged at the envelope and at the determinism, and the draw is **ARMED, NOT DISCHARGED** |
//! | **AC-030** | nothing. `WaitForReset` is exercised only as a row of AC-027's control sweep | the `retry_at(resets_at)` mark, the exit and the resume need the runtime boundary and are not built |
//! | **AC-031** | nothing | the scripted-responder instrument needs a dispatcher; the prose-extraction sub-clause needs a boundary module. Both are **not built** |
//! | **AC-032** | both halves the criterion names: the type test (`tests/ui/ac032_finding_is_not_a_steperror.rs`) and **dispatch count 1** | — |
//!
//! # 🛑 ONE PUBLISHED NUMBER DOES NOT REPRODUCE, AND AN ERRATUM IS OWED
//!
//! ADR-0122 decision 1 supports its charge unit with *"reading the charge off the counter would lose
//! 61 of 284 attempts"*, which puts the terminal-`attempt` sum at **223**. Summed over the corpus at
//! `CORPUS_SHA` 27d97e9b it is **216**, and the loss is **68** [measured 2026-09-14, over
//! `$CORPUS_REPO/results/*.jsonl`'s `step.api_retries`]. Everything else that record and AC-026's
//! PIA-068 Correction publish reproduces exactly — 139 rows, 284 events, restarts on 37, the twelve
//! per-file row counts, `max_retries = 10` on all 284, and the 67 : 217 `overloaded`/`unknown` split.
//! **ADR-0122's decision is unaffected**: 284 is the number it takes as the charge, and the gap
//! between the two readings is *wider* under the measurement than under the stated figure. But an
//! **erratum against that bullet is OWED** (ADR-0092 decision 1 routes a number to an erratum, never
//! to a supersession), and this file's fixture is built to the measurement — fitting it to 223 would
//! have made a green out of a figure the corpus contradicts.
//!
//! # There is no dispatcher at Slice 4c, and every "dispatch count" below says what it counts
//!
//! `rk-ports` is a declared and uncalled edge; the runtime boundary that binds `Harness` is Slice
//! 4h's. So the counts here are read off
//! [`rk_kernel::retry::RetrySchedule`] — **the policy's own account of how many times it would
//! authorise a step to be dispatched**. That is weaker than counting a fake harness's calls and it
//! is stated rather than implied: what it cannot catch is a future runtime that ignores the policy.
//! What it *does* catch is the defect each criterion names — a policy that authorises a re-dispatch
//! for a configuration class, one that runs past the pack's `N`, or one that routes a completed
//! activity's blocking findings into the error channel — and all three counts come from one
//! function, [`rk_kernel::retry::RetryDecision::authorises_redispatch`], so they cannot disagree.

use std::collections::BTreeSet;

use rk_kernel::budget::{
    Attempts, BudgetEnvelope, Charge, Counter, Tokens, WallClockSecs,
    charge_observed_inner_attempts, exhausted_counter, fold,
};
use rk_kernel::retry::{
    ActivityOutcome, ApiRetryEvent, BACKOFF_BASE_SECS, BACKOFF_CAP_SECS, InnerAttempts, JitterDraw,
    RetryDecision, backoff_ceiling_secs, backoff_delay, consult, decide,
    decide_after_inner_retries, error_tag, refused_before_spawn, schedule, schedule_for,
};
use rk_kernel::select::Pack;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_store::memo::IdempotencyKey;
use rk_types::artifact::findings::{Finding, Findings};
use rk_types::digest::{Digest, sha256};
use rk_types::error::{CancelledBy, IsolationSurface, RetryClass, StepError, TransportKind};
use rk_types::repro::{Expected, Repro, ReproKind};
use rk_types::text::Text;
use rk_types::time::Timestamp;

/// The pack that declares `N = 4`. **Compiled in, never opened** — a fixture loaded from a path is
/// a `Result` an implementer can `unwrap_or_default()` into nothing (`tests/it/support.rs`:6-12).
const FIXTURE_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-envelope.toml");

/// The hand-authored `system/api_retry` stream, one JSON object per recorded dispatch row.
///
/// **A transcription of the published distribution, not a corpus artefact.** AC-465 forbids
/// tracking spike JSONL in this repository, and AC-026's own fixture is corpus-resident; the rows
/// below carry the *shape* and the *statistics* the register publishes and none of its bytes.
/// [`inner_retry_ac026`]'s doc names every number and where it comes from.
const FIXTURE_API_RETRIES: &str = include_str!("../fixtures/retry/api-retry-rows.jsonl");

// ---- shared helpers ----------------------------------------------------------------------------

/// The idempotency key at one attempt. **AC-002's licensed tuple**, and the only seed in this file.
fn key_at(attempt: u32) -> IdempotencyKey {
    IdempotencyKey {
        item: "RK-0001".to_owned(),
        activity_id_at_version: "review-implementation@3".to_owned(),
        attempt,
    }
}

/// A second, different key — so "deterministic" cannot be satisfied by a constant.
fn other_key_at(attempt: u32) -> IdempotencyKey {
    IdempotencyKey {
        item: "RK-0002".to_owned(),
        activity_id_at_version: "adr-record@1".to_owned(),
        attempt,
    }
}

/// `N`, read from the pack fixture and **never written as a literal here**.
///
/// # Panics
///
/// With the loader's or the refusal's own message, so a fixture that stops declaring `N` names its
/// reason rather than defaulting to a number this test chose.
fn declared_cap() -> AttemptCap {
    let pack = Pack::from_toml("runtime-envelope-fixture", FIXTURE_ENVELOPE)
        .unwrap_or_else(|e| panic!("the runtime-envelope fixture loads: {e}"));
    pack.require_attempt_cap()
        .unwrap_or_else(|e| panic!("the runtime-envelope fixture declares `N`: {e}"))
}

/// A digest that is not a key's, for the `from_digest` path.
fn some_digest() -> Digest {
    sha256(b"rk-kernel/retry/fixture")
}

/// One of each of ADR-0112's eleven, in the enum's own order.
///
/// A `Vec` and not a sweep helper, because two tests below assert **eleven** as a premise: a
/// twelfth variant would make `StepError`'s matches fail to compile, and a *tenth* would make this
/// list silently short — so the count is asserted where it is used.
fn every_step_error() -> Vec<StepError> {
    vec![
        StepError::Transport {
            kind: TransportKind::Io,
            detail: "connection reset".to_owned(),
        },
        StepError::Schema {
            attempts: 2,
            last_error: "expected object".to_owned(),
        },
        StepError::Timeout { after_secs: 900 },
        StepError::Cancelled {
            by: CancelledBy::Operator,
        },
        StepError::NonZero {
            code: Some(1),
            stderr_digest: some_digest(),
        },
        StepError::RateLimited {
            resets_at: Some(Timestamp::from_unix_seconds(1_757_000_000)),
        },
        StepError::NotAdopted {
            expected_agent: "reviewer".to_owned(),
            observed: "default".to_owned(),
        },
        StepError::NotIsolated {
            surface: IsolationSurface::Tools,
            diff_digest: some_digest(),
        },
        StepError::Platform {
            status: 529,
            retry_after: Some(30),
        },
        StepError::Unsupported {
            capability: "json_schema".to_owned(),
        },
        StepError::Overage {
            reason: Some("credit balance too low".to_owned()),
        },
    ]
}

/// Findings that **block**: a completed activity that said no.
fn blocking_findings() -> Findings {
    Findings {
        items: vec![Finding {
            ac: "AC-032".to_owned(),
            claim: Text::new("the change reverses the invariant"),
            rationale: Text::new("the guard is evaluated after the write"),
            repro: Repro {
                kind: ReproKind::Run,
                command: "cargo test -p rk-kernel -- disjoint_channels_ac032".to_owned(),
                expected: Expected::Exit { exit: 0 },
                observed: "exit 101".to_owned(),
                expected_when_met: "exit 0".to_owned(),
                mutation: None,
            },
        }],
    }
}

/// `src/workflow/retry.rs` with every comment line removed.
///
/// The criteria's own words are quoted in that file's header, so a lint that matched prose would be
/// measuring its own documentation. `tests/it/budget.rs` uses the same stripper for AC-024's
/// no-sleep half and the reason is the same.
fn retry_source_without_comments() -> String {
    const SRC: &str = include_str!("../../src/workflow/retry.rs");
    SRC.lines()
        .filter(|line| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---- AC-027 — configuration classes are never retried -------------------------------------------

/// **AC-027.** *"GIVEN a `StepError` of class `NotAdopted`, `NotIsolated`, `Unsupported(_)` or
/// `Overage` / WHEN the outer retry policy is applied / THEN zero re-dispatches occur and the run
/// fails with the configuration error surfaced. Verified by: a **table-driven** unit test over the
/// four classes asserting **dispatch count 0** and a **surfaced typed error**."*
///
/// **The table is the FOUR the criterion names and not the six ADR-0095's table marks `never`.**
/// `Cancelled` and `NonZero` are also `RetryClass::Never` in the shipped classifier — asserted below
/// as a recorded fact — but **no criterion's THEN asserts dispatch-count-0 for either**, and a
/// six-row table reported as discharging AC-027 would be reporting more coverage than the register
/// carries.
///
/// **The spellings are ADR-0112's, not this criterion's.** AC-027 writes `Unsupported(_)` and
/// `Overage`; the shipped types are `Unsupported { capability }` and `Overage { reason }`. §0.3's
/// C5 row settles it: *"the thirteen criteria keep their names"*, so the criterion text is not
/// amended and the **type** is ADR-0112's.
///
/// **What would make this vacuous, and what stops each:**
///
/// - *A policy that never retries anything* passes every row. **Control 1** is a `Transport`
///   failure over the same cap, which must authorise `N - 1` re-dispatches.
/// - *A table whose four rows are the same value* passes on one implemented class. **Premise 1**
///   asserts the four surfaced errors are four distinct values and the four tags four distinct
///   strings.
/// - *A classifier that disagrees with ADR-0095's table* would make the rows right for the wrong
///   reason. **Premise 2** asserts `retry_class()` is `Never` for each of the four before the policy
///   is consulted.
/// - *A refusal that surfaces a different error than the one that went in* — the `surfaced()`
///   assertion compares the whole value, not its tag.
#[test]
fn never_retried_class_ac027() {
    let cap = declared_cap();
    let key = key_at(1);

    // THE FOUR THE CRITERION NAMES, and their stable tags.
    let table: [(StepError, &str); 4] = [
        (
            StepError::NotAdopted {
                expected_agent: "reviewer".to_owned(),
                observed: "default".to_owned(),
            },
            "not_adopted",
        ),
        (
            StepError::NotIsolated {
                surface: IsolationSurface::McpServers,
                diff_digest: some_digest(),
            },
            "not_isolated",
        ),
        (
            StepError::Unsupported {
                capability: "json_schema".to_owned(),
            },
            "unsupported",
        ),
        (
            StepError::Overage {
                reason: Some("credit balance too low".to_owned()),
            },
            "overage",
        ),
    ];

    // PREMISE 1 — the four rows do not alias, so a policy that implements one does not pass four.
    let tags: BTreeSet<&str> = table.iter().map(|(_, tag)| *tag).collect();
    assert_eq!(tags.len(), 4, "AC-027's four classes must be four tags");
    for (i, (a, _)) in table.iter().enumerate() {
        for (b, _) in table.iter().skip(i + 1) {
            assert_ne!(a, b, "AC-027's four rows must be four different errors");
        }
    }

    for (error, tag) in &table {
        // PREMISE 2 — the shipped classifier agrees this is a `never` class before the policy runs.
        assert_eq!(
            error.retry_class(),
            RetryClass::Never,
            "ADR-0095 decision 2's table, encoded in rk-types, classes {tag} as never-retried"
        );
        assert_eq!(error_tag(error), *tag, "the stable tag for {tag}");

        let sched = schedule(error, &key, cap);

        // THE CRITERION'S FIRST ASSERTION — "zero re-dispatches occur".
        assert_eq!(
            sched.redispatches(),
            0,
            "AC-027: a {tag} failure authorises zero re-dispatches, got {:?}",
            sched.decisions()
        );
        assert_eq!(
            sched.dispatches(),
            1,
            "the one dispatch that produced the failure, and no other"
        );
        assert_eq!(
            sched.decisions(),
            [RetryDecision::Refuse { class: tag }],
            "one decision, and it names the class"
        );

        // THE CRITERION'S SECOND ASSERTION — "the run fails with the configuration error
        // surfaced". The WHOLE value, not its tag: a refusal that surfaced a different
        // `NotIsolated` surface would pass a tag comparison.
        assert_eq!(
            sched.surfaced(),
            Some(error),
            "AC-027: the typed {tag} error is surfaced, unchanged"
        );
    }

    // CONTROL 1 — THE DISCRIMINATOR. Without it every assertion above is satisfied by a policy that
    // refuses everything. `Transport` is `RetryClass::Backoff` and must run to the pack's `N`.
    let transport = StepError::Transport {
        kind: TransportKind::Io,
        detail: "connection reset".to_owned(),
    };
    let control = schedule(&transport, &key, cap);
    assert!(
        cap.get() > 1,
        "the control is vacuous at N = 1; the fixture pack declares {cap}"
    );
    assert_eq!(
        control.redispatches(),
        cap.get() - 1,
        "a weather class retries to the declared cap; without this the four rows above are \
         satisfied by a policy that never retries"
    );

    // `Overage` IS REFUSED BEFORE SPAWN, and the other three are not. ADR-0112 and AC-221/ADR-0010:
    // `Overage` prevents a dispatch; `Unsupported` fails pack validation at the requiring row; the
    // two adoption/isolation classes are observed on a dispatch that happened. Asserted rather than
    // commented, so the distinction cannot quietly collapse.
    assert!(
        refused_before_spawn(&StepError::Overage { reason: None }),
        "AC-221 / ADR-0010: `Overage` is refused BEFORE spawn"
    );
    for (error, tag) in table.iter().filter(|(_, tag)| *tag != "overage") {
        assert!(
            !refused_before_spawn(error),
            "{tag} is observed on a dispatch that happened, not refused before one"
        );
    }
    assert!(!refused_before_spawn(&transport));

    // RECORDED, NOT CLAIMED (§6-S-5). Two more variants are `never` in the shipped table and no
    // criterion's THEN asserts dispatch-count-0 for either. Stated here so a later reader does not
    // add them to the table above and report AC-027 as covering six classes.
    for error in [
        StepError::Cancelled {
            by: CancelledBy::Signal,
        },
        StepError::NonZero {
            code: Some(2),
            stderr_digest: some_digest(),
        },
    ] {
        assert_eq!(error.retry_class(), RetryClass::Never);
        assert!(
            !table.iter().any(|(e, _)| *e == error),
            "{} is deliberately NOT in AC-027's table",
            error_tag(&error)
        );
    }
}

// ---- AC-028 — the policy match is exhaustive and carries no wildcard ----------------------------

/// **AC-028.** *"GIVEN the `StepError` enum in `rk-types` / WHEN a new variant is added without a
/// row in the retry policy table / THEN the kernel fails to compile. Verified by: an exhaustive
/// `match` over `StepError` in the policy function **with no wildcard arm**; a compile-fail test
/// (`trybuild`) proving the wildcard cannot be reintroduced."*
///
/// # 🛑 THE SECOND INSTRUMENT IS NOT CONSTRUCTIBLE IN THE DIRECTION THE CRITERION WRITES IT
///
/// *"A compile-fail test proving the wildcard cannot be reintroduced"* has no literal construction:
/// **adding `_ => …` to a `match` compiles**, in this crate as in any other, and no lint in
/// `clippy.toml` or in `[workspace.lints]` forbids it (`clippy::wildcard_enum_match_arm` is a
/// `restriction` lint and is not enabled). Reported plainly rather than worked around. What is built
/// in its place is the **equivalent and stronger** pair:
///
/// 1. `tests/ui/ac028_steperror_wildcard.rs` — a policy `match` over `StepError` **missing one
///    variant** does not compile (`E0004`), which is the property that makes a twelfth variant fail
///    the build; and, on a second `fn`, the wildcard shown to be **unreachable** once the match is
///    exhaustive (`unreachable_patterns`, denied *by the fixture's own attribute*, which is stated
///    in the fixture because a fixture's own attribute proves nothing about this crate).
/// 2. the source lint below, which asserts the two policy matches in `src/workflow/retry.rs` carry
///    **no `_ =>` arm and no `matches!`** — the two shapes `rk_types::error::StepError::spoke`'s own
///    doc names as the ways exhaustiveness is silently lost.
///
/// **This test is not one of §7b.14's ten names.** It is authored because AC-028 is named by no test
/// in the runbook's list and `00-preamble.md`:188-189 requires an AC in the acceptance set to have a
/// test that fails when the behaviour is absent. A criterion left on the crate floor is reported as
/// not pinned, never as covered.
///
/// **What would make it vacuous, and what stops each:** an emptied or renamed source file makes
/// every absence trivially true, so three **anchor** assertions run first; and a lint that matched
/// the module header's prose would be measuring its own documentation, so comment lines are stripped
/// before anything is asserted.
#[test]
fn ac028_policy_match_has_no_wildcard() {
    let src = retry_source_without_comments();

    // ANCHORS FIRST. An empty file passes every absence below.
    for anchor in [
        "pub const fn error_tag",
        "pub const fn refused_before_spawn",
        "pub fn decide",
    ] {
        assert!(
            src.contains(anchor),
            "anchor {anchor:?} is missing from src/workflow/retry.rs; an absence assertion over a \
             renamed or emptied file is a vacuous pass"
        );
    }

    // EVERY ONE OF ADR-0112'S ELEVEN IS NAMED IN THE SOURCE. A match that dropped a variant into a
    // wildcard would lose the name, and the two absences below would not catch a wildcard spelled
    // some other way.
    let eleven = [
        "Transport",
        "Schema",
        "Timeout",
        "Cancelled",
        "NonZero",
        "RateLimited",
        "NotAdopted",
        "NotIsolated",
        "Platform",
        "Unsupported",
        "Overage",
    ];
    assert_eq!(eleven.len(), 11, "ADR-0112 closes the set at eleven");
    for variant in eleven {
        assert!(
            src.contains(&format!("StepError::{variant}")),
            "the policy matches name `StepError::{variant}` explicitly"
        );
    }

    // THE TWO ABSENCES.
    assert!(
        !src.contains("_ =>"),
        "AC-028: no wildcard arm anywhere in the retry policy"
    );
    assert!(
        !src.contains("matches!("),
        "AC-028: no `matches!`, which expands to an implicit `_ => false` under which a twelfth \
         variant compiles silently (rk-types error.rs:106-109)"
    );

    // AND THE COMPILE-FAIL CASE IS ON DISK. No Checkpoint 7b line asserts this path (§6-G-2: the
    // by-path loop enumerates six files and this is not one of them), so its absence would be
    // caught by nothing. It is caught here.
    for path in [
        "tests/ui/ac028_steperror_wildcard.rs",
        "tests/ui/ac028_steperror_wildcard.stderr",
    ] {
        assert!(
            std::path::Path::new(path).exists(),
            "{path} is missing; AC-028's compile-fail half is asserted by no checkpoint line"
        );
    }
}

// ---- AC-029 — the backoff envelope and the attempt cap ------------------------------------------

/// **AC-029.** *"GIVEN a `Transport`, `Platform 5xx` or `Timeout` failure / WHEN the kernel retries
/// / THEN the delay is exponential with full jitter, base 2 s, capped at 5 min, and **the attempt
/// count never exceeds the pack's `N`**. Verified by: a unit test with an **injected clock and
/// seeded jitter** asserting the delay envelope and the attempt cap."*
///
/// **The seeded jitter is literal, not figurative.** This crate mints no draw:
/// [`rk_kernel::retry::JitterDraw`] is a parameter, and the sequence below is supplied by this test.
/// The second, derived constructor — `JitterDraw::from_key` — reads the leading bytes of the
/// idempotency key's digest, which is `sha256_lp(item, activity_id@version, attempt)`, so a replay
/// recomputes the same delay from recorded inputs alone. **A record authorising a *draw* seeded from
/// that tuple is OWED**: AC-002 licenses it for *ids* and a delay is not an id. The module header
/// carries the residual.
///
/// **`N` is read from a pack fixture and appears as no literal here.** Inverting the two homes —
/// pack-reading the base and cap, code-defaulting `N` — violates the register in both directions at
/// once, and the two edits are one line apart (AC-029's *Home of the numbers*, PIA-055).
///
/// **`Timeout` is excluded from the `N`-sweep, and conflict C-5 is why.** ADR-0095's row is *"retry
/// once, then escalate"*; AC-029's is *"never exceeds the pack's `N`"*. They are reconciled **by
/// bound**, which is asserted below rather than asserted away: the `Timeout` schedule's dispatch
/// count is 2 **and** 2 is inside `N`.
///
/// **What would make it vacuous, and what stops each:**
///
/// - *A delay that ignores the draw* (always the ceiling) fails the `draw = 0` endpoint.
/// - *A delay that ignores the ceiling* (always 0) fails the `draw = u64::MAX` endpoint, and a
///   delay that ignores the cap fails the `attempt = 9` ceiling.
/// - *A constant "deterministic" delay* passes a same-seed comparison, so a **different** key must
///   give a different delay on at least one attempt.
/// - *A cap that is a literal* passes when the fixture changes `N`; the cap is read through
///   `Pack::require_attempt_cap`.
/// - *A schedule that never retries* passes `dispatches() <= N`; the premise asserts it is `> 1`.
#[test]
fn outer_retry_ac029() {
    // ---- the two kernel constants, and their home -----------------------------------------------
    assert_eq!(BACKOFF_BASE_SECS, 2, "AC-029: base 2 s, a kernel constant");
    assert_eq!(BACKOFF_CAP_SECS, 300, "AC-029: capped at 5 min = 300 s");

    // ---- the exponent, and the cap that truncates it ---------------------------------------------
    let ladder: [(u32, u64); 9] = [
        (1, 2),
        (2, 4),
        (3, 8),
        (4, 16),
        (5, 32),
        (6, 64),
        (7, 128),
        (8, 256),
        (9, 300),
    ];
    for (attempt, expected) in ladder {
        assert_eq!(
            backoff_ceiling_secs(attempt),
            expected,
            "AC-029: the envelope ceiling at attempt {attempt}"
        );
    }
    // PREMISE — the ladder actually climbs, so a constant ceiling is not passing the rows above.
    assert_ne!(backoff_ceiling_secs(1), backoff_ceiling_secs(8));
    // AND THE CAP ACTUALLY BINDS. 2 << 8 is 512, which is past 300.
    assert!(
        backoff_ceiling_secs(9) < BACKOFF_BASE_SECS << 8,
        "the 5 min cap truncates the exponent from attempt 9 on"
    );
    for far in [10_u32, 32, 63, 64, 200, 1_000_000] {
        assert_eq!(
            backoff_ceiling_secs(far),
            BACKOFF_CAP_SECS,
            "a shift wide enough to overflow saturates to the cap rather than wrapping small"
        );
    }

    // ---- FULL JITTER: the envelope, and BOTH endpoints -------------------------------------------
    // The "seeded" sequence. Four supplied draws, chosen to sit at and between the endpoints.
    let seeded: [u64; 4] = [0, 1, u64::MAX / 2, u64::MAX];
    for attempt in 1_u32..=12 {
        let ceiling = backoff_ceiling_secs(attempt);
        for raw in seeded {
            let delay = backoff_delay(attempt, JitterDraw::supplied(raw));
            assert!(
                delay <= ceiling,
                "AC-029: the delay at attempt {attempt} is inside [0, {ceiling}], got {delay}"
            );
        }
        // FULL jitter, not equal jitter and not "no jitter": the interval reaches 0 at one end and
        // the ceiling at the other. A build that returned the ceiling regardless fails the first;
        // one that returned 0 regardless fails the second.
        assert_eq!(
            backoff_delay(attempt, JitterDraw::supplied(0)),
            0,
            "full jitter is U(0, ceiling) — the lower endpoint is reachable"
        );
        assert_eq!(
            backoff_delay(attempt, JitterDraw::supplied(u64::MAX)),
            ceiling,
            "full jitter is U(0, ceiling) — the upper endpoint is the ceiling"
        );
    }
    // AND IT IS NOT A DEGENERATE INTERVAL. At attempt 8 the ceiling is 256, so a mid draw must land
    // strictly inside — otherwise "uniform on [0, ceiling]" is a two-point distribution.
    let mid = backoff_delay(8, JitterDraw::supplied(u64::MAX / 2));
    assert!(mid > 0 && mid < backoff_ceiling_secs(8), "got {mid}");

    // ---- determinism, and that the seed is actually read ------------------------------------------
    let cap = declared_cap();
    let first = decide(&transport(), &key_at(1), cap);
    let again = decide(&transport(), &key_at(1), cap);
    assert_eq!(
        first, again,
        "AC-029's replayability: the same (key, attempt) recomputes the same delay"
    );
    // THE DISCRIMINATOR against a constant: a different key must move the delay on at least one
    // attempt. **Not on every attempt, and the sweep is deliberately wider than the fixture's `N`**:
    // at attempt 1 the ceiling is 2, so two unrelated digests land on the same delay one time in
    // three, and a one-attempt comparison would be a flaky assertion rather than a discriminating
    // one. Twelve attempts over a ceiling that reaches 300 is not.
    let wide = AttemptCap::declared(13);
    let moved = (1..wide.get()).any(|a| {
        delay_of(&decide(&transport(), &key_at(a), wide))
            != delay_of(&decide(&transport(), &other_key_at(a), wide))
    });
    assert!(
        moved,
        "the delay must be a function of the idempotency key; a constant delay is not jitter"
    );
    // The derived draw and the supplied one are the same function of their input.
    assert_eq!(
        JitterDraw::from_key(&key_at(1)),
        JitterDraw::from_digest(&key_at(1).digest()),
        "`from_key` is `from_digest` over the key's own digest and nothing else"
    );

    // ---- THE ATTEMPT CAP — "never exceeds the pack's `N`" -----------------------------------------
    assert!(cap.get() > 1, "the sweep is vacuous at N = 1; N is {cap}");
    for attempt in 1..cap.get() {
        let decision = decide(&transport(), &key_at(attempt), cap);
        match decision {
            RetryDecision::Backoff {
                attempt: a,
                delay_secs,
            } => {
                assert_eq!(a, attempt, "the decision carries the attempt it was given");
                assert!(delay_secs <= backoff_ceiling_secs(attempt));
            }
            other => panic!("attempt {attempt} is inside N = {cap}, got {other:?}"),
        }
    }
    // AT and PAST the cap, and for two attempts beyond it — never `Backoff`.
    for attempt in cap.get()..=cap.get() + 2 {
        assert_eq!(
            decide(&transport(), &key_at(attempt), cap),
            RetryDecision::CapReached { cap },
            "AC-029: the attempt count never exceeds N = {cap}"
        );
    }
    let sched = schedule(&transport(), &key_at(1), cap);
    assert_eq!(
        sched.dispatches(),
        cap.get(),
        "a weather class runs to N dispatches and stops"
    );
    assert!(sched.dispatches() > 1, "the schedule must be non-trivial");
    // CONTROL — `N` is read, not chosen: a different declared cap gives a different schedule.
    let narrow = AttemptCap::declared(2);
    assert_ne!(narrow, cap);
    assert_eq!(schedule(&transport(), &key_at(1), narrow).dispatches(), 2);

    // `Platform 5xx` is in AC-029's GIVEN and takes the same envelope (ADR-0112).
    let platform = StepError::Platform {
        status: 529,
        retry_after: Some(30),
    };
    assert_eq!(platform.retry_class(), RetryClass::Backoff);
    assert_eq!(schedule(&platform, &key_at(1), cap).dispatches(), cap.get());

    // ---- C-5: `Timeout` is reconciled BY BOUND, and the bound is asserted -------------------------
    let timeout = StepError::Timeout { after_secs: 900 };
    assert_eq!(
        timeout.retry_class(),
        RetryClass::OnceThenEscalate,
        "ADR-0095 decision 2's row, encoded in rk-types"
    );
    let timed = schedule(&timeout, &key_at(1), cap);
    assert_eq!(
        timed.dispatches(),
        2,
        "ADR-0095: retry once, then escalate — one dispatch and one retry"
    );
    assert!(
        timed.dispatches() <= cap.get(),
        "AC-029: and `retry once` is inside `never exceeds N` for this pack's N = {cap}"
    );
    assert_eq!(
        timed.decisions()[1],
        RetryDecision::EscalateAfterOne,
        "the second decision escalates rather than backing off again"
    );
    // AC-029's GIVEN names `Timeout`, so the ONE retry carries the same envelope.
    assert!(
        matches!(timed.decisions()[0], RetryDecision::Backoff { attempt: 1, delay_secs } if delay_secs <= backoff_ceiling_secs(1))
    );
    // RESIDUAL 10f'S READING, ASSERTED: escalation surfaces the typed error and opens NO gate. No
    // criterion in AC-024-AC-038 names a `Timeout` gate, and an unasserted tier-1 `gate_opened` on
    // every timeout would change the recorded next action.
    assert_eq!(timed.surfaced(), Some(&timeout));

    // ---- the "injected clock" half: nothing here can sleep or read a clock ------------------------
    let src = retry_source_without_comments();
    for anchor in ["pub const fn backoff_delay", "pub fn decide"] {
        assert!(src.contains(anchor), "anchor {anchor:?} missing");
    }
    for banned in [
        "sleep",
        "SystemTime",
        "Instant::",
        "std::time",
        "rand::",
        "thread_rng",
    ] {
        assert!(
            !src.contains(banned),
            "AC-001-AC-003 / AC-029: `{banned}` may not appear in the retry policy. A delay is a \
             number returned, never a wait performed"
        );
    }
}

/// The `Transport` failure the envelope sweep runs over.
fn transport() -> StepError {
    StepError::Transport {
        kind: TransportKind::Io,
        detail: "connection reset".to_owned(),
    }
}

/// The delay a decision carries, or `None` when it carries none.
const fn delay_of(decision: &RetryDecision) -> Option<u64> {
    match decision {
        RetryDecision::Backoff { delay_secs, .. } => Some(*delay_secs),
        RetryDecision::Reprompt { .. }
        | RetryDecision::EscalateAfterOne
        | RetryDecision::WaitForReset { .. }
        | RetryDecision::Refuse { .. }
        | RetryDecision::CapReached { .. } => None,
    }
}

// ---- AC-032 — a domain rejection is not an error class ------------------------------------------

/// **AC-032.** *"GIVEN an activity that completed successfully and returned findings that block /
/// WHEN the retry policy is consulted / THEN no retry occurs — a domain rejection is not an error
/// class. Verified by: **a type test** proving the success and failure channels are disjoint
/// (`Result<Findings, StepError>`), so "the reviewer said no" cannot be constructed as a
/// `StepError`; **plus a unit test asserting dispatch count 1**."*
///
/// **Both halves travel with this name**, because the name suggests only the first. RK-B0013 is the
/// defect the criterion exists for: one HTTP 529 produced a committed `changes-requested` seal whose
/// entire surviving finding was *"review failed to produce a digest"*, because `!review` conflated
/// *"the reviewer said no"* with *"the reviewer never spoke"*.
///
/// The **type** half's instrument is `tests/ui/ac032_finding_is_not_a_steperror.rs` — there is no
/// `From<Findings> for StepError` and no `From<Finding>`, so the conversion is `E0277`. A compile
/// failure is the only instrument that can observe it, and the register calls its own instrument *"a
/// type test"*, so this is that instrument and not a substituted one. What is asserted **here** is
/// the observable consequence: the one door from an outcome to the policy returns `None` for every
/// `Ok`, and `Some` for every one of ADR-0112's eleven — so the `None` is a property of the channel
/// and not of a function that always returns `None`.
///
/// **What would make it vacuous, and what stops each:**
///
/// - *`consult` returning `None` for everything* — the eleven-variant sweep asserts `Some`.
/// - *Findings that do not block* — the premise asserts the fixture's findings are non-empty.
/// - *`dispatches()` being the constant 1* — the control asserts a `Transport` outcome schedules
///   `N` of them.
#[test]
fn disjoint_channels_ac032() {
    let cap = declared_cap();
    let key = key_at(1);

    // PREMISE — the findings BLOCK. An empty finding set is not a domain rejection.
    let findings = blocking_findings();
    assert!(
        !findings.items.is_empty(),
        "AC-032's GIVEN is findings that BLOCK; an empty set asserts nothing"
    );

    // THE SUCCESS CHANNEL NEVER REACHES THE POLICY.
    let completed: ActivityOutcome = Ok(findings);
    assert_eq!(
        consult(&completed, &key, cap),
        None,
        "AC-032: an activity that completed never consults the retry policy, findings and all"
    );

    // AND IT IS NOT `None` BECAUSE `consult` ALWAYS IS. Every one of ADR-0112's eleven reaches it.
    let eleven = every_step_error();
    assert_eq!(eleven.len(), 11, "ADR-0112 closes the set at eleven");
    for error in &eleven {
        let outcome: ActivityOutcome = Err(error.clone());
        assert!(
            consult(&outcome, &key, cap).is_some(),
            "the failure channel reaches the policy: {}",
            error_tag(error)
        );
    }
    // The eleven tags are eleven distinct strings, and none of them is findings-shaped: there is no
    // arm of the closed error set a domain rejection could be routed into by name.
    let tags: BTreeSet<&str> = eleven.iter().map(error_tag).collect();
    assert_eq!(tags.len(), 11);
    for tag in &tags {
        assert!(
            !tag.contains("finding") && !tag.contains("reject") && !tag.contains("block"),
            "no `StepError` arm is findings-shaped, and {tag} would be one"
        );
    }

    // ---- "plus a unit test asserting dispatch count 1" --------------------------------------------
    let completed: ActivityOutcome = Ok(blocking_findings());
    let sched = schedule_for(&completed, &key, cap);
    assert_eq!(
        sched.dispatches(),
        1,
        "AC-032: one dispatch, and the policy is never consulted"
    );
    assert_eq!(sched.redispatches(), 0);
    assert!(
        sched.decisions().is_empty(),
        "a completed activity produces no retry decision at all"
    );
    assert_eq!(
        sched.surfaced(),
        None,
        "a domain rejection is not a surfaced error class"
    );

    // CONTROL — 1 is not a constant.
    let failed: ActivityOutcome = Err(transport());
    let control = schedule_for(&failed, &key, cap);
    assert_eq!(control.dispatches(), cap.get());
    assert!(
        control.dispatches() > sched.dispatches(),
        "without this, `dispatches() == 1` is satisfied by a function that returns 1"
    );

    // AND THE COMPILE-FAIL CASE IS ON DISK — no checkpoint line asserts its path (§6-G-2).
    for path in [
        "tests/ui/ac032_finding_is_not_a_steperror.rs",
        "tests/ui/ac032_finding_is_not_a_steperror.stderr",
    ] {
        assert!(
            std::path::Path::new(path).exists(),
            "{path} is missing; AC-032's type test is asserted by no checkpoint line"
        );
    }
}

// ---- AC-026 — inner harness retries charge the activity budget ----------------------------------

/// One recorded dispatch row of the `api_retry` fixture.
#[derive(Debug)]
struct Row {
    result_file: String,
    events: Vec<ApiRetryEvent>,
}

/// Parse the committed fixture.
///
/// # Panics
///
/// If a line is not the object the fixture's generator writes — a fixture that stops parsing names
/// its reason rather than shrinking silently to zero rows.
fn api_retry_rows() -> Vec<Row> {
    FIXTURE_API_RETRIES
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let v: serde_json::Value =
                serde_json::from_str(line).unwrap_or_else(|e| panic!("fixture row parses: {e}"));
            let events = v["api_retries"]
                .as_array()
                .expect("every row carries an api_retries array")
                .iter()
                .map(|e| ApiRetryEvent {
                    attempt: u32::try_from(e["attempt"].as_u64().expect("attempt")).expect("u32"),
                    max_retries: u32::try_from(e["max_retries"].as_u64().expect("max_retries"))
                        .expect("u32"),
                    error_status: e["error_status"]
                        .as_u64()
                        .map(|s| u16::try_from(s).expect("u16")),
                })
                .collect();
            Row {
                result_file: v["result_file"].as_str().expect("result_file").to_owned(),
                events,
            }
        })
        .collect()
}

/// **AC-026.** *"GIVEN a dispatch during which the harness emitted `system/api_retry` events
/// carrying an attempt counter / WHEN the activity's attempt budget is evaluated / THEN those inner
/// attempts are **counted against the activity's budget** and the kernel **does not add a retry of
/// its own for the same failure**."*
///
/// # 🛑 THIS IS THE UNFEATURED HALF OF A PAIR, AND IT IS NOT THE CRITERION'S NAMED INSTRUMENT
///
/// AC-026's *Verified by* names *"rows from `spikes/results/e3.jsonl` (and `p7`/`s-var`) **at the
/// corpus pin**"*. §7b.14's third loop runs `cargo test -p rk-kernel` **without** `--features
/// corpus` and asserts this exact name, and AC-465 forbids tracking spike JSONL in this repository,
/// so the criterion's own fixture cannot be read by a test in that loop. The lawful resolution is a
/// **pair**: this test, over a hand-authored fixture *shaped like* the recorded rows, which pins the
/// **accounting rule**; and [`inner_retry_ac026_at_the_corpus_pin`], which is AC-026's named
/// instrument and reads at the pin. **AC-026 is discharged only under `--features corpus`.**
///
/// The fixture's numbers are the register's own, transcribed and not invented — AC-026's 2026-09-11
/// PIA-068 Correction: *"1 attempt on 89 rows, 2 on 21, 3 on 12, 4 on 4, 5 on 1, 6 on 6 and 10 on 6
/// … 284 attempts in all … `max_retries` is 10 on every row"*, 139 rows across 12 result files, and
/// ADR-0122's *"the counter restarts within a single dispatch on 37 of 139 recorded rows"*. **Every
/// one of those reproduces** against the corpus at `CORPUS_SHA` 27d97e9b [measured 2026-09-14, by
/// summing `step.api_retries` over `$CORPUS_REPO/results/*.jsonl`], including the twelve per-file
/// row counts (e3 26, p7 38, s-var 28, s-typed 18, e0-stability 6, p8 5, s-var-2400 5, h4 4,
/// p1-arm-l-v2 4, s-typed-oldschema 3, e3-pilot 1, e3h 1) and the `overloaded`/529 : `unknown`/null
/// split of 67 : 217. The per-file **event** counts are published nowhere and are not reproduced.
///
/// 🛑 **One published number does not reproduce, and this fixture is built to the measurement.**
/// ADR-0122 decision 1 states the loss as *"61 of 284 attempts"*, which puts the terminal-`attempt`
/// sum at 223; the measured sum is **216** and the loss is **68**. An erratum against that bullet is
/// **OWED**. The decision itself is untouched — 284 is the number it takes — and the assertion below
/// says why fitting the fixture to 223 would have been the wrong repair.
///
/// # What ADR-0122 settled, and what is still owed above it
///
/// Conflict C1 (AC-026/AC-232 vs AC-391's *Cut*) was open until 2026-09-14 and is **resolved for
/// AC-026** by ADR-0122, accepted the same day. Decision 1 fixes the **unit**: one charge per
/// recorded event, *"not the terminal value of its `attempt` field"*. Decision 2 fixes the
/// **scope**: the suppression is dispatch-scoped, on AC-026's *"for the same failure"*. Decision 5
/// records what is owed **above** the record and is not this test's: AC-391's *Cut:* paragraph must
/// be withdrawn by a dated register amendment, §0.3's C1 row gains a resolution line, and AC-391's
/// surviving class-scoped clause needs a dated clarification. **Until they land, a stage claiming
/// AC-391 reports it armed, not discharged.**
///
/// **What would make it vacuous, and what stops each:**
///
/// - *A charge read off the terminal counter* gives 223 and not 284 — the whole point of the
///   restart rows, and `assert_ne!(284, 223)` is written out so the two can never be the same
///   number by accident.
/// - *A fixture that shrank to zero rows* — the premises assert 139 rows, 284 events, 12 result
///   files and `max_retries == 10` everywhere before any accounting runs.
/// - *A policy that never retries `Platform` at all* passes "the outer retry count stays at zero" —
///   the control dispatches the **same** error class with **zero** inner attempts and requires a
///   backoff, which is the discriminator between ADR-0122's dispatch scope and AC-391's class scope.
/// - *A charge that also moved a sibling counter* — AC-020's separation is re-asserted through this
///   charge site.
#[test]
fn inner_retry_ac026() {
    let rows = api_retry_rows();

    // ---- PREMISES: the fixture carries the distribution the register publishes -------------------
    assert_eq!(rows.len(), 139, "139 rows carry the `api_retry` payload");
    let files: BTreeSet<&str> = rows.iter().map(|r| r.result_file.as_str()).collect();
    assert_eq!(files.len(), 12, "across 12 result files");
    assert!(
        files.contains("e3.jsonl") && files.contains("p7.jsonl") && files.contains("s-var.jsonl")
    );
    for row in &rows {
        assert!(
            !row.events.is_empty(),
            "a row carrying the payload carries at least one event"
        );
        for e in &row.events {
            assert_eq!(
                e.max_retries, 10,
                "`max_retries` is 10 on every recorded row"
            );
        }
    }
    let per_row: BTreeSet<usize> = rows.iter().map(|r| r.events.len()).collect();
    assert_eq!(
        per_row.into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 10],
        "the recorded per-row attempt counts"
    );

    // ---- ADR-0122 DECISION 1: THE UNIT IS THE EVENT, NOT THE COUNTER -----------------------------
    let events_total: u32 = rows
        .iter()
        .map(|r| u32::try_from(r.events.len()).expect("u32"))
        .sum();
    let terminal_sum: u32 = rows
        .iter()
        .map(|r| r.events.last().expect("non-empty").attempt)
        .sum();
    assert_eq!(events_total, 284, "284 recorded attempts in all");
    assert_ne!(
        events_total,
        terminal_sum,
        "ADR-0122 decision 1: reading the charge off the counter loses {} of {events_total} \
         attempts. If these two are ever equal the fixture has stopped discriminating",
        events_total - terminal_sum
    );
    // 🛑 **216, NOT 223.** ADR-0122 decision 1 states the loss as *"61 of 284 attempts"*, which puts
    // the terminal-`attempt` sum at 223. It does not reproduce: summed over the corpus at
    // `CORPUS_SHA` 27d97e9b the terminal sum is **216** and the loss is **68**
    // [measured 2026-09-14, over `$CORPUS_REPO/results/*.jsonl`'s `step.api_retries`; the same pass
    // reproduces every other figure the record and the register publish — 139 rows, 284 events,
    // restarts on 37, `max_retries = 10` on all 284, `overloaded`/529 on 67 and `unknown`/null on
    // 217, and the twelve per-file row counts exactly]. **The decision is unaffected**: 284 is the
    // number ADR-0122 takes as the charge, and the two readings differ by more under the measured
    // figure than under the stated one. An **erratum against ADR-0122's decision-1 bullet is OWED**
    // (ADR-0092 decision 1: an erratum is for numbers), and this fixture is built to the
    // measurement rather than to the record, because fitting it to 223 would make a green out of a
    // figure the corpus contradicts.
    assert_eq!(terminal_sum, 216, "the MEASURED terminal-`attempt` sum");
    assert_eq!(events_total - terminal_sum, 68, "the measured loss");

    let restarts = rows
        .iter()
        .filter(|r| r.events.windows(2).any(|w| w[1].attempt <= w[0].attempt))
        .count();
    assert_eq!(
        restarts, 37,
        "the counter restarts within a single dispatch on 37 of 139 rows"
    );

    // PER ROW: the counter is the number of EVENTS.
    let mut differing = 0_usize;
    for row in &rows {
        let observed = InnerAttempts::from_recorded(&row.events);
        assert_eq!(
            observed.get(),
            u32::try_from(row.events.len()).expect("u32"),
            "one charge per recorded event"
        );
        if observed.get() != row.events.last().expect("non-empty").attempt {
            differing += 1;
        }
    }
    assert_eq!(
        differing, restarts,
        "the rows where the two readings differ are exactly the restart rows"
    );

    // ---- THE CHARGE LANDS ON THE ATTEMPTS COUNTER AND ON NO OTHER (AC-020 through AC-026) --------
    let envelope = BudgetEnvelope {
        attempts_total: Attempts::new(1_000),
        tokens: Tokens::new(1_000_000),
        wall_clock_secs: WallClockSecs::new(86_400),
    };
    let charges: Vec<Charge> = rows
        .iter()
        .map(|r| charge_observed_inner_attempts(InnerAttempts::from_recorded(&r.events)))
        .collect();
    let (ledger, report) = fold(&charges, &envelope);
    assert_eq!(
        ledger.attempts(),
        Attempts::new(events_total),
        "AC-026: the inner attempts are counted against the activity's budget"
    );
    assert_ne!(
        ledger.attempts(),
        Attempts::new(terminal_sum),
        "and at the event unit, never the counter's terminal value"
    );
    assert_eq!(ledger.tokens(), Tokens::new(0), "AC-020: no sibling moved");
    assert_eq!(ledger.wall_clock(), WallClockSecs::new(0));
    assert_eq!(report.attempts_charges, 139);
    assert_eq!(report.token_charges, 0);
    assert_eq!(report.wall_clock_charges, 0);

    // ADR-0122 DECISION 3's FOURTH GROUND, ASSERTED: under the runtime reading, exhaustion on the
    // retry path must be REACHABLE. A single 10-attempt dispatch exhausts an activity declaring 3.
    let tight = BudgetEnvelope {
        attempts_total: Attempts::new(3),
        tokens: Tokens::new(1_000_000),
        wall_clock_secs: WallClockSecs::new(86_400),
    };
    let ten = rows
        .iter()
        .find(|r| r.events.len() == 10)
        .expect("the fixture carries the `max_retries = 10` ceiling rows");
    let (tight_ledger, _) = fold(
        &[charge_observed_inner_attempts(
            InnerAttempts::from_recorded(&ten.events),
        )],
        &tight,
    );
    assert_eq!(
        exhausted_counter(&tight_ledger, &tight),
        Some(Counter::Attempts),
        "AC-025 is reachable on the retry path, which is the reading ADR-0122 decision 3 keeps"
    );

    // ---- ADR-0122 DECISION 2: NO OUTER RETRY FOR THE FAILURE THE HARNESS ALREADY RETRIED ---------
    let cap = declared_cap();
    let overloaded = StepError::Platform {
        status: 529,
        retry_after: Some(30),
    };
    // PREMISE — this class WOULD retry, so the suppression below is doing work.
    assert_eq!(overloaded.retry_class(), RetryClass::Backoff);

    for row in &rows {
        let inner = InnerAttempts::from_recorded(&row.events);
        assert!(inner.any(), "every fixture row retried inside the dispatch");
        let decision = decide_after_inner_retries(&overloaded, &key_at(1), cap, inner);
        assert!(
            !decision.authorises_redispatch(),
            "AC-026: the outer retry count stays at ZERO for a failure the harness already \
             retried; got {decision:?}"
        );
    }

    // THE CONTROL, AND IT IS THE DISCRIMINATOR. ADR-0122 decision 2 is DISPATCH-scoped: the same
    // error class, on a dispatch the harness did NOT retry inside, is decided by the policy
    // unchanged. A class-scoped suppression would silently disable outer retry for `Platform` for
    // the rest of the run, and would pass every assertion above.
    let none_observed = InnerAttempts::observed(0);
    assert!(!none_observed.any());
    let unsuppressed = decide_after_inner_retries(&overloaded, &key_at(1), cap, none_observed);
    assert_eq!(
        unsuppressed,
        decide(&overloaded, &key_at(1), cap),
        "with no inner retry observed the policy is untouched"
    );
    assert!(
        unsuppressed.authorises_redispatch(),
        "without this, `authorises_redispatch() == false` above is satisfied by a policy that \
         never retries `Platform` at all"
    );
}

/// **AC-026's own named instrument** — the captured rows, read **at the corpus pin**.
///
/// *"a unit test over the **captured** stream fixture — rows from `spikes/results/e3.jsonl` (and
/// `p7`/`s-var`) **at the corpus pin**, containing real `api_retry` events — asserting the counter
/// increments and the outer retry count stays at zero for that failure."*
///
/// Gated on `--features corpus` because it reads `redkiln-corpus` at `CORPUS_PIN` and no hosted
/// runner can fetch it (decision D-N, PIA-002), and because AC-464's read route resolves through the
/// pin and never through a working tree. **The name deliberately does not match
/// `\binner_retry_ac026\b`** — an underscore after `6` defeats the word boundary — so §7b.14's third
/// loop cannot be satisfied by this half, and §7b.14:1345-1346's bare-substring count is untouched
/// because neither forbidden string appears in it.
///
/// It **panics** on a missing corpus rather than returning early: a corpus test that silently passes
/// when the corpus is absent reports its own absence as success (`corpus_f12.rs`:35-38's rule).
#[cfg(feature = "corpus")]
#[test]
fn inner_retry_ac026_at_the_corpus_pin() {
    use std::path::{Path, PathBuf};

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = std::fs::read_to_string(root.join("CORPUS_PIN"))
        .expect("CORPUS_PIN must exist; this test is gated on --features corpus");
    let get = |k: &str| -> String {
        text.lines()
            .find_map(|l| l.trim().strip_prefix(&format!("{k}=")))
            .unwrap_or_else(|| panic!("CORPUS_PIN carries no {k}"))
            .trim()
            .to_owned()
    };
    let repo = PathBuf::from(get("CORPUS_REPO"));
    let sha = get("CORPUS_SHA");
    assert_eq!(
        sha.len(),
        40,
        "CORPUS_SHA is not an unabbreviated SHA: {sha}"
    );
    assert!(sha.chars().all(|c| c.is_ascii_hexdigit()), "not hex: {sha}");
    assert!(repo.is_dir(), "corpus repo absent at {}", repo.display());

    // **AC-464'S REFUSAL HALF, BEFORE ANY READ.** `crates/rk-git/tests/it/corpus_f12.rs`:72-96 is the
    // house rule and this is the same shape: a run against an unpinned reference, or against a
    // corpus checkout with uncommitted changes, is REFUSED. Without it the read below is a working
    // tree read dressed as a read at the pin — which is a proxy for the operation, not the
    // operation. `rk_git::git_exec::run` is the one wrapper `std::process::Command` is allowed
    // behind, and it is why `rk-git` is a dev-dependency of this crate as of 2026-09-14.
    let head = rk_git::git_exec::run(&repo, ["rev-parse", "HEAD"])
        .expect("git rev-parse in the corpus checkout")
        .stdout_trimmed();
    assert_eq!(
        head, sha,
        "the corpus checkout is not at CORPUS_SHA; a run against an unpinned reference is refused \
         before any read (AC-464)"
    );
    let porcelain = rk_git::git_exec::run(&repo, ["status", "--porcelain"])
        .expect("git status in the corpus checkout")
        .stdout_trimmed();
    assert!(
        porcelain.is_empty(),
        "the corpus checkout has uncommitted changes; refused before any read (AC-464):\n\
         {porcelain}"
    );

    let cap = declared_cap();
    let overloaded = StepError::Platform {
        status: 529,
        retry_after: Some(30),
    };

    // The three files AC-026's *Verified by* names, with the register's own per-file row counts.
    // Asserted PER FILE rather than only in aggregate: a census that only totalled would stay green
    // if one file emptied and another grew.
    let census: [(&str, usize); 3] = [("e3.jsonl", 26), ("p7.jsonl", 38), ("s-var.jsonl", 28)];

    let mut rows_seen = 0_usize;
    let mut events_seen = 0_u32;
    for (name, published) in census {
        let path = repo.join("results").join(name);
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("AC-464: {} is read at the pin: {e}", path.display()));
        let mut in_file = 0_usize;
        for line in body.lines().filter(|l| !l.trim().is_empty()) {
            let v: serde_json::Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(e) => panic!("{} carries JSONL: {e}", path.display()),
            };
            // **`step.api_retries`, not a top-level key.** The runner nests the payload under the
            // step record (`spikes/runner/src/runner.rs`:61's `api_retries: Vec<ApiRetry>`), and a
            // reader that looked only at the top level finds nothing and passes an empty sweep --
            // which is what the per-file census below exists to catch. It caught exactly that on
            // this test's first run [2026-09-14].
            let Some(retries) = v
                .get("step")
                .and_then(|step| step.get("api_retries"))
                .and_then(|r| r.as_array())
            else {
                continue;
            };
            if retries.is_empty() {
                continue;
            }
            let events: Vec<ApiRetryEvent> = retries
                .iter()
                .map(|e| ApiRetryEvent {
                    attempt: u32::try_from(e["attempt"].as_u64().unwrap_or(0)).unwrap_or(0),
                    max_retries: u32::try_from(e["max_retries"].as_u64().unwrap_or(0)).unwrap_or(0),
                    error_status: e["error_status"]
                        .as_u64()
                        .and_then(|s| u16::try_from(s).ok()),
                })
                .collect();
            let observed = InnerAttempts::from_recorded(&events);
            assert_eq!(
                observed.get(),
                u32::try_from(events.len()).expect("u32"),
                "ADR-0122 decision 1: one charge per recorded event"
            );
            assert!(observed.any(), "the counter increments");
            assert!(
                !decide_after_inner_retries(&overloaded, &key_at(1), cap, observed)
                    .authorises_redispatch(),
                "AC-026: the outer retry count stays at zero for that failure"
            );
            for e in &events {
                assert_eq!(
                    e.max_retries, 10,
                    "`max_retries` is 10 on every recorded row"
                );
            }
            rows_seen += 1;
            in_file += 1;
            events_seen += observed.get();
        }
        assert_eq!(
            in_file, published,
            "the register publishes {published} payload-carrying rows in {name}; the corpus at              this pin carries {in_file}. A change is a finding about the pin, not a defect here"
        );
    }

    // 26 + 38 + 28. The Halt-8 guard is the `>= 1`; the census is the equality.
    assert_eq!(
        rows_seen, 92,
        "the register's 2026-09-11 recount over the three named files"
    );
    assert!(
        rows_seen >= 1,
        "AC-464: the pinned corpus must actually carry `api_retry` rows; a zero-row pass is Halt \
         8's `a filter that matches nothing exits 0`"
    );
    println!("inner_retry at the pin: rows={rows_seen} events={events_seen}");
}
