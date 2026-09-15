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
//! | **AC-029** | the two kernel constants, the exponent, the cap, full jitter's envelope **and its two endpoints**, determinism across two evaluations, the cap read from a **pack fixture**, and — as of 2026-09-14 — the **production draw**, whose identity at both delay-carrying arms is [`jitter_source_is_forced_and_owes_no_record`]'s | *(this cell said the production draw was **unbound** and **ARMED, NOT DISCHARGED**. Both were false in the tree: `decide` calls `JitterDraw::from_key` at two sites, and AC-002's own closing sentence licenses that source. The correction is measured, not argued — see that test's doc)* |
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
    ActivityOutcome, ApiRetryEvent, BACKOFF_BASE_SECS, BACKOFF_CAP_SECS, HARNESS_ALREADY_RETRIED,
    InnerAttempts, JitterDraw, RetryDecision, backoff_ceiling_secs, backoff_delay, consult, decide,
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

/// `src/workflow/retry.rs`, verbatim. **The only reader that wants the comments is the one below.**
const RETRY_SRC: &str = include_str!("../../src/workflow/retry.rs");

/// `src/workflow/retry.rs` with every comment line removed.
///
/// The criteria's own words are quoted in that file's header, so a lint that matched prose would be
/// measuring its own documentation. `tests/it/budget.rs` uses the same stripper for AC-024's
/// no-sleep half and the reason is the same.
fn retry_source_without_comments() -> String {
    const SRC: &str = RETRY_SRC;
    SRC.lines()
        .filter(|line| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The module header of `src/workflow/retry.rs` — the **leading run of `//!` lines and nothing
/// else**, in source order, joined with a space.
///
/// **A whole-file `contains` is decoration, and this module has the measurement to prove it.**
/// `inner_retry_ac026`'s own comment records it: a whole-file match for *"this crate's proposal"*
/// stayed green when the phrase was deleted from the declaration it claimed to be about, because a
/// copy of the same phrase sat 400 lines away. So every prose assertion in this file windows first.
///
/// Joined in **source** order, not reverse, so a phrase that wraps across two `//!` lines is still
/// found rather than being silently unmatchable.
fn retry_module_header() -> String {
    RETRY_SRC
        .lines()
        .take_while(|l| l.trim_start().starts_with("//!"))
        // THE MARKER IS STRIPPED, AND THAT IS NOT COSMETIC. Joining raw lines leaves a `//!`
        // between them, so any phrase that wraps across two lines becomes unmatchable and every
        // positive assertion silently weakens to "must not wrap" [measured 2026-09-14: the first
        // draft of this helper went red on a phrase that is present].
        .map(|l| l.trim_start().trim_start_matches("//!").trim())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The doc block immediately above one declaration in `src/workflow/retry.rs`, as one line.
///
/// Three properties, each of which was a defect in some earlier version of this instrument:
///
/// - **Attribute-aware.** The walk back passes over the `#[must_use]` between a doc block and its
///   item; a walk that stopped at the first non-`///` line would make `from_key`'s window empty and
///   every negative assertion over it vacuously true.
/// - **Newline-agnostic.** The window is taken by walking lines, never by searching for a blank
///   line with a two-newline needle — which matched nothing on a CRLF tree, fell through to the
///   start of the file and produced a 30,288-byte "window" where the real block is 1,861
///   [measured 2026-09-14; the note lives on `inner_retry_ac026`].
/// - **Source order.** Joined forward, so a phrase that wraps across two `///` lines matches.
fn retry_doc_block_above(decl: &str) -> String {
    let at = RETRY_SRC
        .find(decl)
        .unwrap_or_else(|| panic!("`{decl}` must be declared in src/workflow/retry.rs"));
    let head = &RETRY_SRC[..at];
    let head = &head[..head.rfind('\n').map_or(0, |i| i + 1)];
    let mut doc: Vec<&str> = head
        .lines()
        .rev()
        .take_while(|l| {
            let t = l.trim_start();
            t.starts_with("///") || t.starts_with('#')
        })
        .filter(|l| l.trim_start().starts_with("///"))
        // Stripped for [`retry_module_header`]'s reason: a retained `///` between two joined lines
        // makes every wrapped phrase unmatchable.
        .map(|l| l.trim_start().trim_start_matches("///").trim())
        .collect();
    doc.reverse();
    doc.join(" ")
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
/// **The seeded jitter is literal, not figurative.** [`rk_kernel::retry::JitterDraw`] is a
/// parameter of `backoff_delay`, and the sequence swept below is supplied by this test, which is
/// what puts a delay at each endpoint of the envelope. The second, derived constructor —
/// `JitterDraw::from_key` — reads the leading bytes of the idempotency key's digest, which is
/// `sha256_lp(item, activity_id@version, attempt)`, so a replay recomputes the same delay from
/// recorded inputs alone.
///
/// **That source is FORCED and owes no record** *(corrected 2026-09-14; this paragraph claimed one
/// was OWED on the reading that AC-002 licenses the tuple for* ids *and a delay is not an id)*.
/// AC-002 bans a **non-deterministic** source and names the idempotency key and a content hash as
/// the licensed derivations; AC-029 requires a seeded draw. The binding on the production path is
/// [`jitter_source_is_forced_and_owes_no_record`]'s to assert; what is asserted **here** is the
/// envelope, its endpoints, and that a different key moves the delay somewhere.
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

// ---- the jitter SOURCE — forced by AC-002, and the record that is NOT owed -----------------------

/// **The jitter source is FORCED by AC-002, it is WIRED on the production path, and no record is
/// owed for it** — decided 2026-09-14, and this is what stops the module drifting back.
///
/// # The warrant, in the register's own words
///
/// AC-002's WHEN bans a draw *"from a **non-deterministic** source (`rand`, `uuid::new_v4`, address
/// hashing)"* — it bans **non-determinism**, not derivation. Its closing sentence sanctions the
/// route in terms: *"Ids the workflow needs are derived from the idempotency key `(item,
/// activity_id@version, attempt)` (ADR-0097) **or from a content hash**."*
/// `rk_kernel::retry::JitterDraw::from_key` reads that key's digest, which is `sha256_lp` over
/// exactly that tuple; AC-029 *requires* a seeded draw. The class is blessed, the route is the
/// blessed one, and **there is no free residual for an operator to settle**.
///
/// # What stood here until today, and why it is withdrawn rather than edited away
///
/// `src/workflow/retry.rs`'s header said *"AC-002 licenses that tuple for **ids**, and a delay is
/// not an id. No record says the idempotency key may seed a **draw** … `JitterDraw::from_key` is
/// this crate's proposal"*, and its criterion table called the production draw *"not bound"*. Both
/// are false in this tree. The second is refuted by **part 1**, which reads the delay `decide`
/// actually returns rather than the delay it is allowed to return. The first is refuted by AC-002's
/// own closing sentence — which that same header **quoted at its own `:41-43`, five lines above the
/// sentence at `:48` that contradicted it**.
///
/// # How part 2 tells a withdrawn claim from a deleted one
///
/// Both sites keep the withdrawn sentence **verbatim**, because a correction that deletes what it
/// corrects leaves the next reader nothing to check it against — `budget.rs`'s header takes the
/// same discipline with its own false history. A blunt `!contains` would therefore be wrong in both
/// directions: red on an honest correction, green on a silent scrub. The marker
/// `WITHDRAWN 2026-09-14` splits each window into the **live claims** and the **quoted history**,
/// and both halves are asserted — the phrase absent above it, present below it.
///
/// # 🛑 THE MODULE'S OTHER OWED-RECORD CLAIM IS **KEPT**, AND PART 3 IS WHAT KEEPS IT
///
/// `HARNESS_ALREADY_RETRIED` **is** this crate's proposal with a record OWED — spelled by no
/// register line, by no ADR-0122 decision and by no design §5 entry — and nothing here withdraws
/// it. Part 2's two **negative** assertions are otherwise satisfiable by deleting every such phrase
/// from the file, which would silently withdraw the claim that must stand; part 3 is the control
/// that refuses that, and `inner_retry_ac026` pins the same window independently.
///
/// # Why the name carries no AC id
///
/// **Not** for `appetite_raise_from_an_activity_result_is_refused`'s reason (`it/budget.rs`) — that
/// name is AC-id-free because a record is owed, and here none is. It carries none because it
/// discharges no criterion's named instrument: AC-002's are `clippy.toml`'s denylist and the source
/// lint `it/no_egress.rs` runs; AC-029's are `outer_retry_ac029`'s. What is measured here is a
/// **binding** and a **warrant**.
///
/// # What would make it vacuous, and what stops each
///
/// - *An arm that passes a constant draw.* Every equality in part 1 would still hold if `decide`
///   passed `JitterDraw::supplied(0)` and `from_key` also drew 0. **Premise 1** asserts two
///   different keys draw differently and that the attempt moves the draw; **control 1** asserts a
///   production delay is actually non-zero.
/// - *The `Timeout` arm, which only ever backs off at attempt 1, where the ceiling is 2.* One key
///   there cannot discriminate and the first draft's single equality did not: rewriting that arm to
///   `supplied(0)` left the whole workspace green [measured 2026-09-14 by mutation]. It is swept
///   over eight keys and the delays must take **more than one value**, which no constant draw can.
/// - *A window that fell through to the whole file.* **Premise 2** asserts each of the three
///   windows is non-empty, is a fraction of the file, and carries a phrase only it can carry.
/// - *Deleting the phrases everywhere.* **Part 3** keeps `HARNESS_ALREADY_RETRIED`'s label, and
///   part 2's history half keeps the withdrawn words at both of their own sites.
/// - *A seventh number appearing in this module.* **Part 4** — the six quantities §7b.10 fixes, and
///   the three `pub const`s this file is allowed to write.
#[test]
fn jitter_source_is_forced_and_owes_no_record() {
    // ---- PREMISE 1 — the draw is a function of the key ------------------------------------------
    assert_ne!(
        JitterDraw::from_key(&key_at(1)),
        JitterDraw::from_key(&other_key_at(1)),
        "PREMISE: two different keys must draw differently, or every equality below is satisfied \
         by a `from_key` that returns a constant"
    );
    assert_ne!(
        JitterDraw::from_key(&key_at(1)),
        JitterDraw::from_key(&key_at(2)),
        "PREMISE: the attempt is INSIDE the key, so it moves the draw too"
    );

    // ---- (1) THE SOURCE IS WIRED, AT BOTH ARMS THAT CARRY A DELAY -------------------------------
    //
    // This is the assertion that refutes *"the production draw is not bound"*. `outer_retry_ac029`
    // asserts the delay is INSIDE the envelope and that a different key moves it somewhere; what is
    // asserted here is the **identity** — the number `decide` returns is `backoff_delay` over this
    // key's own `from_key` draw and over nothing else.
    let cap = declared_cap();
    assert!(cap.get() > 2, "the sweep is vacuous at N <= 2; N is {cap}");

    let mut any_non_zero = false;
    for attempt in 1..cap.get() {
        let key = key_at(attempt);
        let produced = delay_of(&decide(&transport(), &key, cap));
        assert_eq!(
            produced,
            Some(backoff_delay(attempt, JitterDraw::from_key(&key))),
            "the `Backoff` arm draws from the idempotency key at attempt {attempt}"
        );
        any_non_zero |= produced.unwrap_or(0) > 0;
    }
    // CONTROL 1 — a zero draw yields zero at every attempt, so an arm hard-wired to one would pass
    // every equality above if `from_key` happened to draw zero as well.
    assert_eq!(backoff_delay(9, JitterDraw::supplied(0)), 0);
    assert!(
        any_non_zero,
        "every production delay was zero — the equalities above would then be satisfied by an arm \
         that draws nothing at all"
    );

    // The second arm. AC-029's GIVEN names `Timeout` explicitly, so ADR-0095's ONE retry carries
    // the same envelope — and, measured here, the same source through the same function.
    let timeout = StepError::Timeout { after_secs: 900 };
    assert_eq!(
        timeout.retry_class(),
        RetryClass::OnceThenEscalate,
        "PREMISE: ADR-0095 decision 2's row, encoded in rk-types"
    );
    //
    // 🛑 AND IT IS SWEPT OVER SEVERAL KEYS, BECAUSE ONE KEY CANNOT DISCRIMINATE HERE. This arm
    // backs off only at `attempt <= 1`, where the ceiling is **2** — so the delay is one of
    // {0, 1, 2} and a single equality against `from_key` is satisfied by an arm hard-wired to
    // `supplied(0)` roughly one key in three. It was: the first draft asserted exactly one key, and
    // rewriting this arm to `JitterDraw::supplied(0)` left the WHOLE workspace suite green
    // [measured 2026-09-14 by mutation — `key_at(1)` happens to draw into the 0 bucket at ceiling
    // 2]. The discriminator is that the delays this arm produces take **more than one value** over
    // a key sweep, which no constant draw can do.
    let sweep: Vec<IdempotencyKey> = (0..8)
        .map(|n| IdempotencyKey {
            item: format!("RK-{n:04}"),
            activity_id_at_version: "review-implementation@3".to_owned(),
            attempt: 1,
        })
        .collect();
    let mut produced: BTreeSet<Option<u64>> = BTreeSet::new();
    for key in &sweep {
        let delay = delay_of(&decide(&timeout, key, cap));
        assert_eq!(
            delay,
            Some(backoff_delay(1, JitterDraw::from_key(key))),
            "the `OnceThenEscalate` arm draws from the key it was handed, through the same function"
        );
        produced.insert(delay);
    }
    assert!(
        produced.len() > 1,
        "the `Timeout` arm produced the single delay {produced:?} across {} distinct keys — a \
         constant draw satisfies every equality above, so this arm's binding would be unpinned",
        sweep.len()
    );

    // ---- PREMISE 2 — three REAL windows, none of them the whole file ----------------------------
    let header = retry_module_header();
    let from_key_doc = retry_doc_block_above("pub fn from_key(");
    let token_doc = retry_doc_block_above("pub const HARNESS_ALREADY_RETRIED");
    for (name, window, only_here) in [
        ("the module header", &header, "AC-029'S"),
        (
            "`from_key`'s doc",
            &from_key_doc,
            "no arithmetic constant is introduced",
        ),
        (
            "`HARNESS_ALREADY_RETRIED`'s doc",
            &token_doc,
            "the class of the *refusal*",
        ),
    ] {
        assert!(
            !window.is_empty(),
            "the window for {name} is EMPTY — the walk found nothing and every assertion over it \
             would be vacuously true"
        );
        assert!(
            window.len() < RETRY_SRC.len() / 2,
            "the window for {name} is {} bytes of a {}-byte file — it fell through, and a \
             `contains` over the whole file is decoration",
            window.len(),
            RETRY_SRC.len()
        );
        assert!(
            window.contains(only_here),
            "the window for {name} does not carry {only_here:?}, so it is not the block it claims \
             to be"
        );
    }

    // ---- (2) THE HEADER STATES THE CLAUSE AND CLAIMS NO OWED RECORD FOR THE SOURCE --------------
    for clause in [
        "AC-002",
        "or from a content hash",
        "idempotency key `(item, activity_id@version, attempt)`",
    ] {
        assert!(
            header.contains(clause),
            "the header must state the clause that FORCES the source; {clause:?} is missing"
        );
    }
    // THE WITHDRAWN CLAIM IS QUOTED, NOT SCRUBBED — AND THE GUARD HAS TO TELL THOSE APART.
    //
    // Both sites keep the sentence that stood until today, verbatim, because a correction that
    // deletes what it corrects leaves the next reader unable to check it (this is the same
    // discipline `budget.rs`'s header takes for the `spent > declared` false history). So a blunt
    // `!contains` is wrong in BOTH directions: it fails on an honest correction, and it passes on a
    // silent deletion. The marker `WITHDRAWN 2026-09-14` splits each window into the LIVE claims
    // and the QUOTED HISTORY, and both halves are asserted — the phrase must be absent above the
    // marker and present below it.
    let marker = "WITHDRAWN 2026-09-14";
    for (name, window, phrase) in [
        ("the module header", &header, "this crate's proposal"),
        ("`from_key`'s doc", &from_key_doc, "is OWED"),
    ] {
        let (live, history) = window.split_once(marker).unwrap_or_else(|| {
            panic!(
                "{name} carries no {marker:?} marker, so the withdrawal is unrecorded and this \
                 assertion cannot tell a live claim from a quoted one"
            )
        });
        assert!(
            !live.contains(phrase),
            "{name} still asserts {phrase:?} as a LIVE claim. AC-002's closing sentence sanctions \
             the idempotency key and a content hash outright and bans only a non-deterministic \
             source, so there is no proposal here and no record is owed for it"
        );
        assert!(
            history.contains(phrase),
            "{name} withdrew {phrase:?} without quoting it. A correction that deletes what it \
             corrects leaves the next reader nothing to check the correction against"
        );
    }

    // ---- (3) THE CONTROL — the module's OTHER proposal keeps its label --------------------------
    //
    // Without this, part 2 is satisfied by deleting the phrases from the file, which would withdraw
    // the one claim that IS owed. `inner_retry_ac026` asserts the same window for its own reason;
    // that this is asserted twice is deliberate, because the two tests fail for different causes.
    assert!(
        token_doc.contains("this crate's proposal"),
        "`HARNESS_ALREADY_RETRIED` is spelled by NO record and must keep saying so"
    );
    assert!(
        token_doc.contains("OWED"),
        "and the record naming the refusal-class vocabulary is still owed"
    );
    assert!(
        RETRY_SRC.contains("this crate's proposal"),
        "PREMISE: the phrase still exists in this module, so part 2's negatives are about WHERE it \
         appears and not about whether the vocabulary was scrubbed"
    );

    // ---- (4) THE SIX CONSTANTS, AND THE SEVENTH THIS MODULE MAY NOT MINT ------------------------
    //
    // §7b.10's 🛑 fixes six quantities with no measured value: `K`, `N`, the backoff base and cap,
    // the two-tick headroom margin and the per-item token appetite. Two are kernel literals and
    // live here (PIA-055). `K`, `N` and the appetite are pack-declared and refused as code defaults
    // — `ac029_n_is_declared_never_defaulted` (`it/envelope.rs`) is `N`'s instrument and neither
    // `AttemptCap` nor `BreakerThreshold` derives `Default`. The headroom margin is **AC-377's**,
    // in `rk-adapter-claude`/`doctor`, which is in NO §7b.15 claimed range: it gets no constant
    // here and no test named for it [measured 2026-09-14: `grep -rni headroom
    // crates/rk-kernel/tests` -> 0 hits].
    let src = retry_source_without_comments();
    for declared in [
        "pub const BACKOFF_BASE_SECS: u64 = 2;",
        "pub const BACKOFF_CAP_SECS: u64 = 300;",
    ] {
        assert!(
            src.contains(declared),
            "the kernel literal {declared:?} is this module's, by PIA-055"
        );
    }
    for foreign in [
        "headroom",
        "margin",
        "Margins",
        "appetite",
        "AttemptCap::declared",
        "BreakerThreshold",
    ] {
        assert!(
            !src.contains(foreign),
            "`{foreign}` may not appear in the retry policy's CODE. The other four of §7b.10's six \
             are declared elsewhere — three in the pack and one in `rk-adapter-claude` — and a \
             constant for any of them written here would violate the register in the direction \
             PIA-055 names"
        );
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
/// row counts (p7 38, s-var 28, e3 26, s-typed 18, e0-stability 6, p8 5, s-var-2400 5, h4 4,
/// p1-arm-l-v2 4, s-typed-oldschema 3, e3-pilot 1, e3h 1) and the `overloaded`/529 : `unknown`/null
/// split of 67 : 217. The per-file **event** counts are published nowhere and are not reproduced.
///
/// 🛑 **Every one of those numbers is now ASSERTED below, and until 2026-09-14 none of them was.**
/// The paragraph above published twelve per-file counts and a two-way split and the body asserted
/// **neither**, so a fixture that drifted would have left this doc describing a distribution its
/// own bytes no longer carried and nothing would have gone red. An unasserted number in a doc
/// comment is a claim with no instrument. Both are read off the fixture now, and both are
/// cross-checked against the totals already asserted — the twelve must sum to 139 rows and the
/// split must exhaust 284 events — so the per-file figures and the aggregate figures cannot drift
/// apart while each stays individually true.
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

    // ---- THE TWELVE PER-FILE ROW COUNTS AND THE STATUS SPLIT, ASSERTED ---------------------------
    //
    // Added 2026-09-14 on an adversarial verification pass. This doc comment **published** these
    // twelve numbers and the 67 : 217 split and asserted **none of them**: a drifted fixture would
    // have left the prose describing a distribution the bytes no longer carried, and nothing would
    // have gone red. An unasserted number in a doc comment is a claim with no instrument, so each
    // is now read off the fixture. Every figure below is the register's (AC-026's PIA-068
    // Correction) and ADR-0122's, re-derived from the corpus at `CORPUS_SHA` 27d97e9b on
    // 2026-09-14, and the fixture reproduces all of them.
    let mut per_file: Vec<(&str, usize)> = {
        let mut counts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
        for row in &rows {
            *counts.entry(row.result_file.as_str()).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    };
    // Descending by count, then by name — the order the register publishes them in.
    per_file.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    assert_eq!(
        per_file,
        vec![
            ("p7.jsonl", 38),
            ("s-var.jsonl", 28),
            ("e3.jsonl", 26),
            ("s-typed.jsonl", 18),
            ("e0-stability.jsonl", 6),
            ("p8.jsonl", 5),
            ("s-var-2400.jsonl", 5),
            ("h4.jsonl", 4),
            ("p1-arm-l-v2.jsonl", 4),
            ("s-typed-oldschema.jsonl", 3),
            ("e3-pilot.jsonl", 1),
            ("e3h.jsonl", 1),
        ],
        "the twelve per-file row counts the register publishes and this test's doc names"
    );
    assert_eq!(
        per_file.iter().map(|(_, n)| n).sum::<usize>(),
        139,
        "the twelve per-file counts must sum to the 139 rows asserted above — otherwise the two \
         premises could drift apart and each stay individually true"
    );

    // The `overloaded`/529 : `unknown`/null split, over the 284 EVENTS and not over the rows.
    let overloaded = rows
        .iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.error_status == Some(529))
        .count();
    let unknown = rows
        .iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.error_status.is_none())
        .count();
    assert_eq!(
        (overloaded, unknown),
        (67, 217),
        "the 67 : 217 status split"
    );
    assert_eq!(
        overloaded + unknown,
        284,
        "the split must exhaust the 284 events — a third status would make both halves true and \
         the distribution wrong"
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

    // ---- THE REFUSAL CLASS IS THIS CRATE'S PROPOSAL, AND THAT IS ASSERTED RATHER THAN ASSUMED ----
    //
    // Added 2026-09-14 on an adversarial verification pass. `HARNESS_ALREADY_RETRIED` is spelled by
    // **no record anywhere** — not the register, not ADR-0122 (which settled the rule and named no
    // class token), not design §5 — while every other `class` this module emits comes from
    // `error_tag`'s closed eleven. `src/workflow/retry.rs` declares it as the crate's proposal with
    // the record OWED, on `rk-types`' `event_kind.rs`:16-21 precedent, and these two assertions are
    // what stop it drifting into looking like a record's word.
    let suppressed =
        decide_after_inner_retries(&overloaded, &key_at(1), cap, InnerAttempts::observed(1));
    assert_eq!(
        suppressed,
        RetryDecision::Refuse {
            class: HARNESS_ALREADY_RETRIED
        },
        "the suppression is a typed Refusal carrying the named token, not an arbitrary string"
    );
    let eleven_tags: BTreeSet<&str> = every_step_error().iter().map(error_tag).collect();
    assert_eq!(eleven_tags.len(), 11, "PREMISE: ADR-0112's closed set");
    assert!(
        !eleven_tags.contains(HARNESS_ALREADY_RETRIED),
        "the token is NOT one of `error_tag`'s eleven — it names the class of the REFUSAL (the \
         kernel declined because the harness already retried), not the class of the FAILURE. If it \
         ever collapses into one of the eleven, a refusal for this reason becomes \
         indistinguishable from `UpToAttempts`' refusal of the same error"
    );
    assert!(
        // ANCHORED TO THE DECLARATION, NOT THE FILE. A whole-file `contains` was green when the
        // label was deleted from this declaration, because `JitterDraw`'s header section carried a
        // copy of the same phrase [measured 2026-09-14 by mutation]. That copy has since been
        // WITHDRAWN — the jitter source is forced by AC-002 and owes no record — so the defect is
        // no longer reproducible at that particular site. The anchoring stays regardless: a
        // control that passes on some other item's text is decoration however few other items
        // there happen to be today. The window is the declaration's own preceding doc.
        {
            let src = include_str!("../../src/workflow/retry.rs");
            let decl = src
                .find("pub const HARNESS_ALREADY_RETRIED")
                .expect("declared here");
            // NEWLINE-AGNOSTIC, AND THAT IS THE WHOLE POINT. This walked back to the previous
            // blank line with a two-newline needle, which on this CRLF tree matched NOTHING, fell
            // through to the start of the file, and made the "window" 30,288 bytes -- the entire
            // file, including the same phrase used of `JitterDraw` at :51. It was decoration twice
            // over [measured 2026-09-14: a probe printed len=30288 where the real doc block is
            // 1,861 bytes]. Walking back over doc-comment lines cannot care about the ending.
            let doc: String = src[..decl]
                .lines()
                .rev()
                .take_while(|l| l.trim_start().starts_with("///"))
                .collect::<Vec<_>>()
                .join(" ");
            let doc = doc.as_str();
            doc.contains("this crate's proposal") && doc.contains("OWED")
        },
        "the declaration must keep saying it is this crate's proposal with the record OWED; a \
         token that quietly stops being labelled reads as a record's spelling"
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

// ---- the scheduler's over-cap entry -------------------------------------------------------------

/// **An over-cap entry surfaces the error. It used to come back as "no failure surfaced".**
///
/// *(Authored 2026-09-14 on an adversarial verification pass that reproduced the defect.)*
/// `rk_kernel::retry::schedule` drove its loop with `while attempt <= fence`, `fence = cap + 1`, so
/// a key whose `attempt` was already past `cap + 1` **never entered the body**: `decisions()` came
/// back **empty**, `surfaced()` came back **`None`** and `dispatches()` still reported **1**. An
/// error handed to the scheduler was reported as a run with no failure — the silent pass
/// `00-preamble.md`:55 forbids, and the exact inverse of AC-027's *"the run fails with the
/// configuration error surfaced"*.
///
/// **This is not a hypothetical key.** `key.attempt` is the activation's own dispatch counter
/// (`crate::runtime`'s `next_attempt`), so a resumed run, or a pack whose `N` was lowered between
/// runs, reaches an over-cap attempt ordinarily — and the failure mode is silence, which is the
/// one the instrument set is least able to notice.
///
/// **What makes each assertion discriminating:**
///
/// - The **control comes first**: at attempt 1 the same error and the same cap produce a schedule
///   that surfaces and re-dispatches. Without it, "surfaced is `Some`" is satisfied by a function
///   that surfaces unconditionally.
/// - `decisions()` is asserted **non-empty and terminal**: exactly one decision, authorising no
///   re-dispatch. An over-cap entry must stop, not restart the ladder from the top.
/// - `u32::MAX` is included, because `attempt + 1` saturates there and a fence checked in the wrong
///   place is how a saturating counter becomes a non-terminating loop.
#[test]
fn schedule_refuses_to_swallow_an_over_cap_attempt() {
    let cap = declared_cap();
    let error = transport();

    // ---- CONTROL: inside the cap the schedule surfaces AND re-dispatches -------------------------
    let inside = schedule(&error, &key_at(1), cap);
    assert_eq!(inside.surfaced(), Some(&error));
    assert!(
        inside.redispatches() >= 1,
        "PREMISE: this class re-dispatches inside the cap, so the over-cap cases below are a \
         different outcome and not the only one this function can produce"
    );
    assert_eq!(inside.dispatches(), cap.get());

    // ---- and PAST the cap it surfaces too, rather than reporting nothing -------------------------
    for attempt in [
        cap.get().saturating_add(1),
        cap.get().saturating_add(2),
        cap.get().saturating_add(50),
        u32::MAX,
    ] {
        let sched = schedule(&error, &key_at(attempt), cap);
        assert!(
            !sched.decisions().is_empty(),
            "attempt {attempt} is past the cap of {}, and an entry the loop never reached reports \
             an error as a run with no decisions at all",
            cap.get()
        );
        assert_eq!(
            sched.surfaced(),
            Some(&error),
            "attempt {attempt}: the error handed to the scheduler must come back SURFACED. \
             `None` here is the scheduler saying no failure occurred"
        );
        assert_eq!(
            sched.redispatches(),
            0,
            "attempt {attempt}: past the cap nothing is authorised — AC-029's *never exceeds the \
             pack's N*"
        );
        let terminal = sched
            .decisions()
            .last()
            .expect("a non-empty schedule has a last decision");
        assert!(
            !terminal.authorises_redispatch(),
            "attempt {attempt}: the terminal decision must authorise nothing; got {terminal:?}"
        );
    }
}
