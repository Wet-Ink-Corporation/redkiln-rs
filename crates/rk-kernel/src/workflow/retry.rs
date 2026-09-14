//! **The retry policy** — ADR-0112's eleven variants, the backoff envelope, the attempt cap, and
//! the one door an activity outcome reaches a retry decision through.
//!
//! | criterion | what this module carries | state |
//! | --- | --- | --- |
//! | **AC-026** | [`InnerAttempts`], [`ApiRetryEvent`] and [`decide_after_inner_retries`] — ADR-0122 decision 1's charge unit and decision 2's dispatch-scoped suppression | 🛑 **ARMED, NOT DISCHARGED** — see the section below. The rule is on disk and pinned by `inner_retry_ac026`; **no dispatcher calls it**, and C1 is resolved by ADR-0122 |
//! | **AC-027** | [`decide`]'s `Never` classes and [`refused_before_spawn`] | **built** — `redispatches() == 0` and the typed error surfaced, per class |
//! | **AC-028** | [`error_tag`] and [`refused_before_spawn`] — two exhaustive `match`es over the eleven, **no wildcard, no `matches!`** | **built**, compile-time only; no unit test discharges it |
//! | **AC-029** | [`BACKOFF_BASE_SECS`], [`BACKOFF_CAP_SECS`], [`backoff_ceiling_secs`], [`backoff_delay`], [`JitterDraw`], the cap check in [`decide`] | the **envelope, the exponent, the cap and the determinism** are built; the *production* draw is not bound (see the 🛑 below) |
//! | **AC-030** | [`RetryDecision::WaitForReset`] only | the `retry_at(resets_at)` **mark**, the exit and the resume are the runtime boundary's — **not built, not claimed** |
//! | **AC-031** | [`RetryDecision::Reprompt`] and [`SCHEMA_REPROMPT_LIMIT`] | the scripted-responder instrument needs a dispatcher and is **not built**; the prose-extraction sub-clause is **not built** and the register itself records it as under-tracked |
//! | **AC-032** | [`ActivityOutcome`], [`consult`], [`schedule_for`] | **built** — an `Ok`, findings and all, never reaches [`decide`] |
//!
//! # The two constants, and why they are the only numbers written here
//!
//! AC-029's *Home of the numbers (2026-09-11, PIA-055)*, verbatim:
//!
//! > Base and cap are **kernel retry-policy constants** — the position of this criterion, AC-233,
//! > the activity-graph schema (*"a step never defines its own backoff"*) and design §3.6 :338;
//! > only `K`, `N` and the appetite are pack values.
//!
//! So [`BACKOFF_BASE_SECS`] and [`BACKOFF_CAP_SECS`] are written here as literals, and `N` is read
//! from the pack through [`super::envelope::AttemptCap`] and is **never** defaulted. The two
//! mistakes are one edit apart and the register is violated in both directions at once by swapping
//! them. AC-029's own *Note* travels with them: *"base 2 s and the 5 min cap are chosen constants,
//! not measured — no spike sized them (gap 3). The criterion verifies conformance to the declared
//! envelope, not that the envelope is right."*
//!
//! # 🛑 AC-029'S *"SEEDED JITTER"*: WHAT IS BUILT, WHAT IS INJECTED, AND WHAT IS STILL OWED
//!
//! `rand` is not in this workspace — `grep -rn "rand" Cargo.toml crates/*/Cargo.toml` → **0 hits,
//! exit 1** [measured 2026-09-14] — and `clippy.toml` bans `rand::random` / `rand::thread_rng`
//! workspace-wide. **This module contains no random-number generator, seeded or otherwise, and no
//! PRNG constant** (no multiplier, no increment, no word size). The full-jitter draw enters as a
//! **value**: [`JitterDraw`], which [`backoff_delay`] takes by parameter. A test that supplies a
//! fixed sequence *is* the *"seeded jitter"* AC-029's instrument names, and the seeding is the
//! caller's.
//!
//! The one derivation offered — [`JitterDraw::from_key`] — reads the leading bytes of
//! `rk_store::memo::IdempotencyKey::digest()`, which is `sha256_lp(item, activity_id@version,
//! attempt)`. That is a **content-hash read, not a generator**: AC-002's own *Verified by* names
//! this exact tuple as the licensed derivation (*"Ids the workflow needs are derived from the
//! idempotency key `(item, activity_id@version, attempt)` (ADR-0097) or from a content hash"*), and
//! every component is a recorded input, so a replay recomputes the same delay with no clock, no
//! counter and no entropy source — the property `replay.rs`:63-70 says the golden-log layout exists
//! to keep.
//!
//! **What is OWED and is stated rather than assumed:** AC-002 licenses that tuple for **ids**, and
//! a delay is not an id. No record says the idempotency key may seed a *draw*. The owed record's
//! own words are that this *"is the free residual, and it is an operator's"*. Until it lands,
//! [`JitterDraw::from_key`] is this crate's proposal; [`JitterDraw::supplied`] is the shape that is
//! correct under either resolution, and it is the constructor the envelope test drives. The
//! production binding of a draw is the stage that owns a binary; at Step 7b there is none.
//! **AC-029's *"full jitter"* conjunct is therefore discharged at the envelope and at the
//! determinism, and the production draw is ARMED, NOT DISCHARGED.**
//!
//! # 🛑 `Timeout`: TWO RECORDS, ONE VARIANT (conflict C-5), AND WHAT *"ESCALATE"* RESOLVES TO
//!
//! ADR-0095 decision 2's row is *"retry once, then escalate"*; AC-029's THEN is that the attempt
//! count *"never exceeds the pack's `N`"*. They are **reconciled by bound, not by a decision
//! taken here**: for `N >= 2`, *"retry once"* is strictly inside *"never exceeds `N`"*, and
//! [`decide`] checks the cap **before** the class so the `N = 1` pack still gets `CapReached`.
//! The class itself is **never re-derived here**: it is read from the shipped
//! `rk_types::error::StepError::retry_class()`, which is ADR-0095 decision 2's table encoded in a
//! checkpoint-passed crate. AC-029's GIVEN names `Timeout` explicitly, so the one retry carries the
//! same delay envelope as `Transport` and `Platform`.
//!
//! **What *"escalate"* resolves to is a reading, and no record states one.** ADR-0022 says
//! escalation is a kernel-owned durable `Gate:` mark; ADR-0105 clause 10 forbids an escalate gate's
//! expiry action being `escalate`; **no criterion in AC-024–AC-038 names a `Timeout` gate**. The
//! reading taken: [`RetryDecision::EscalateAfterOne`] **fails the activity with the typed
//! `Timeout { after_secs }` surfaced and opens no gate**. Opening one would write a tier-1
//! `gate_opened` carrying a `Gate:` trailer that the replay must then serve back and that no
//! criterion asserts — an unasserted tier-1 write on every timeout, changing the recorded next
//! action, surfacing only as a replay divergence long afterwards. A later decision to escalate to a
//! gate is purely **additive**.
//!
//! # What is deliberately NOT here
//!
//! No sleep of any kind — `std::thread::sleep` and `tokio::time::sleep` are on `clippy.toml`'s
//! `disallowed-methods` and the parent module's `#![deny(clippy::disallowed_methods)]` reaches this
//! child. A delay is a **number returned**, never a wait performed; AC-030's *"it never sleeps
//! until the reset"* and AC-024's *"zero elapsed sleep"* are type facts of this module rather than
//! assertions about it. No clock: nothing here reads one, and `RetryDecision::WaitForReset` carries
//! the `resets_at` it was handed. No dispatcher, no port, no `EventStore` — [`schedule`] reports
//! what the **policy** authorises and spawns nothing; the mark writes, the append sites and the
//! real dispatch loop are the runtime boundary's, which does not exist yet. And no breaker: `K`,
//! `BreakerState` and the per-family refusal are Slice 4d's and none of them is stubbed here.

use std::fmt;

use rk_store::memo::IdempotencyKey;
use rk_types::artifact::findings::Findings;
use rk_types::digest::Digest;
use rk_types::error::{RetryClass, StepError};
use rk_types::time::Timestamp;

use super::envelope::AttemptCap;

/// **AC-029's backoff base, in seconds.** A kernel retry-policy constant (PIA-055), never a pack
/// value.
///
/// *"A chosen constant, not measured — no spike sized it (gap 3); the criterion verifies
/// conformance to the declared envelope, never that the envelope is right"* (AC-029's *Note*). The
/// AWS "Exponential backoff and jitter" post the register cites is cited for the backoff **shape**
/// (full jitter) only; neither *"base 2 s"* nor *"cap 5 min"* appears in it.
pub const BACKOFF_BASE_SECS: u64 = 2;

/// **AC-029's backoff cap, in seconds** — five minutes. A kernel retry-policy constant, with
/// [`BACKOFF_BASE_SECS`]'s warrant and [`BACKOFF_BASE_SECS`]'s caveat.
pub const BACKOFF_CAP_SECS: u64 = 300;

/// **AC-031's ceiling**, stated by that criterion's own THEN and by nothing else: *"it re-prompts
/// exactly once carrying the validation error, and on a second failure returns
/// `Schema { attempts: 2 }`."*
///
/// It is here because [`decide`] must answer for `RetryClass::UpToAttempts` and the alternative is a
/// silent default. **AC-031 is not claimed by this module**: its instrument is a scripted responder
/// over a real dispatcher, and its second instrument — a grep-lint over the boundary module banning
/// JSON-from-prose extraction — has no boundary module to run over yet.
pub const SCHEMA_REPROMPT_LIMIT: u32 = 2;

/// One full-jitter draw, as a **value the caller supplies**.
///
/// This crate contains no generator. The draw is a fraction of `2^64`: [`backoff_delay`] maps it
/// uniformly onto `[0, ceiling]`, which is full jitter's `U(0, min(cap, base · 2^attempt))`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JitterDraw(u64);

impl JitterDraw {
    /// The draw the caller supplies. **The constructor AC-029's instrument names**: a test that
    /// hands in a fixed sequence is the *"seeded jitter"*, and the seeding is the test's.
    #[must_use]
    pub const fn supplied(raw: u64) -> Self {
        Self(raw)
    }

    /// Derived from the idempotency key's digest — **a content-hash read, not a generator**.
    ///
    /// `IdempotencyKey::digest()` is `sha256_lp(item, activity_id@version, attempt)`, so the draw is
    /// a pure function of three recorded inputs and a replay recomputes the same delay. The leading
    /// 64 bits of the hex digest are read; no arithmetic constant is introduced.
    ///
    /// 🛑 **A record authorising this seed is OWED** — see the module header. Prefer
    /// [`JitterDraw::supplied`] wherever the draw can be handed in.
    #[must_use]
    pub fn from_key(key: &IdempotencyKey) -> Self {
        Self::from_digest(&key.digest())
    }

    /// [`JitterDraw::from_key`]'s body, over any content hash.
    ///
    /// The leading 64 bits of the hex form. `rk_types::digest::Digest` is constructible only from
    /// validated hex (`Digest::from_hex`, or one of the two hash functions), so the parse cannot
    /// fail on any value this workspace can produce; the arm is written out rather than `unwrap`ped
    /// and yields the draw **0**, which is the lower endpoint of the envelope. That direction is
    /// deliberate: a fallback of 0 shortens a delay and can never place one outside
    /// [`BACKOFF_CAP_SECS`], whereas a fallback of `u64::MAX` would silently pin every delay to the
    /// ceiling.
    #[must_use]
    pub fn from_digest(digest: &Digest) -> Self {
        let hex = digest.as_str();
        let head = if hex.len() >= 16 { &hex[..16] } else { hex };
        // The `0` is the unreachable arm, and it is the LOW endpoint of the envelope on purpose.
        u64::from_str_radix(head, 16).map_or(Self(0), Self)
    }

    /// The raw draw.
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// What the retry policy decided for one observed failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryDecision {
    /// Re-dispatch after `delay_secs`, which is inside [`backoff_ceiling_secs`] for `attempt`.
    Backoff {
        /// The attempt that just failed, counted from 1.
        attempt: u32,
        /// The full-jitter delay. **A number returned; nothing here sleeps it.**
        delay_secs: u64,
    },
    /// Re-dispatch carrying the validation error — AC-031's one re-prompt.
    Reprompt {
        /// The attempt number the re-prompt will carry.
        attempts: u32,
    },
    /// ADR-0095's `Timeout` row, second half. The activity fails with the typed error surfaced and
    /// **no gate is opened** — the reading is in the module header.
    EscalateAfterOne,
    /// AC-030's disposition: wait for the reset, never back off, never sleep. The mark write and
    /// the exit are the runtime boundary's.
    WaitForReset {
        /// As the failure carried it. Derived by the adapter, never by this crate.
        resets_at: Option<Timestamp>,
    },
    /// **AC-027.** No re-dispatch: a configuration or billing state does not improve by repetition.
    Refuse {
        /// A stable tag, never message text.
        class: &'static str,
    },
    /// **AC-029.** The pack's `N` is spent.
    CapReached {
        /// The declared cap, carried so a failure names the number it hit.
        cap: AttemptCap,
    },
}

impl RetryDecision {
    /// A stable tag for the decision itself.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Backoff { .. } => "backoff",
            Self::Reprompt { .. } => "reprompt",
            Self::EscalateAfterOne => "escalate_after_one",
            Self::WaitForReset { .. } => "wait_for_reset",
            Self::Refuse { .. } => "refuse",
            Self::CapReached { .. } => "cap_reached",
        }
    }

    /// **Does this decision authorise another dispatch of the same step?**
    ///
    /// One definition, so AC-027's *"zero re-dispatches occur"*, AC-029's *"the attempt count never
    /// exceeds the pack's `N`"* and AC-026's *"the outer retry count stays at zero"* are all counted
    /// off the same function and cannot disagree.
    #[must_use]
    pub const fn authorises_redispatch(&self) -> bool {
        match self {
            Self::Backoff { .. } | Self::Reprompt { .. } => true,
            Self::EscalateAfterOne
            | Self::WaitForReset { .. }
            | Self::Refuse { .. }
            | Self::CapReached { .. } => false,
        }
    }
}

/// A stable tag per `StepError` variant.
///
/// **AC-028's instrument in this crate.** An exhaustive `match` over ADR-0112's eleven with **no
/// wildcard arm**, so a twelfth variant is `E0004` here. The shape is forced from both sides, and
/// `rk_types::error::StepError::spoke` states it: an arm-per-variant form with equal bodies trips
/// `clippy::match_same_arms` (denied workspace-wide) and `matches!(…)` expands to an implicit
/// `_ => …` catch-all under which a twelfth variant compiles silently. Here every arm's body
/// differs, so eleven arms is the lawful form.
#[must_use]
pub const fn error_tag(error: &StepError) -> &'static str {
    match error {
        StepError::Transport { .. } => "transport",
        StepError::Schema { .. } => "schema",
        StepError::Timeout { .. } => "timeout",
        StepError::Cancelled { .. } => "cancelled",
        StepError::NonZero { .. } => "non_zero",
        StepError::RateLimited { .. } => "rate_limited",
        StepError::NotAdopted { .. } => "not_adopted",
        StepError::NotIsolated { .. } => "not_isolated",
        StepError::Platform { .. } => "platform",
        StepError::Unsupported { .. } => "unsupported",
        StepError::Overage { .. } => "overage",
    }
}

/// Is this failure refused **before** a process is spawned?
///
/// Exactly one variant is, and `rk_types::error::StepError::retry_class`'s own comment is the
/// authority: *"`Overage` is refused BEFORE spawn (AC-221, ADR-0010); `Unsupported` fails pack
/// validation at the requiring row (design §3.6 P7)."* The two are different moments and this
/// function keeps them apart — folding `Unsupported` in here would claim a pre-spawn refusal for a
/// condition pack validation catches earlier still.
///
/// This is the second exhaustive no-wildcard `match` over the eleven (see [`error_tag`]).
#[must_use]
pub const fn refused_before_spawn(error: &StepError) -> bool {
    match error {
        StepError::Overage { .. } => true,
        StepError::Transport { .. }
        | StepError::Schema { .. }
        | StepError::Timeout { .. }
        | StepError::Cancelled { .. }
        | StepError::NonZero { .. }
        | StepError::RateLimited { .. }
        | StepError::NotAdopted { .. }
        | StepError::NotIsolated { .. }
        | StepError::Platform { .. }
        | StepError::Unsupported { .. } => false,
    }
}

/// **AC-029's envelope ceiling** for one attempt: `min(cap, base · 2^(attempt-1))`, in seconds.
///
/// Attempt 1 is the first failure, so its ceiling is the base itself. A shift wide enough to
/// overflow saturates to [`BACKOFF_CAP_SECS`] rather than wrapping — there is no attempt number at
/// which the envelope silently becomes small again.
#[must_use]
pub const fn backoff_ceiling_secs(attempt: u32) -> u64 {
    // Attempt 1 is the first failure, so it doubles zero times and its ceiling is the base itself.
    let doublings = attempt.saturating_sub(1);
    let mut value = BACKOFF_BASE_SECS;
    let mut done = 0_u32;
    while done < doublings {
        // The cap is applied BEFORE each doubling, so the product never approaches `u64::MAX` and
        // the exponent cannot wrap. **`checked_shl` is not the way to write this**: it checks the
        // shift AMOUNT and not the value, so `2u64.checked_shl(63)` is `Some(0)` — an envelope that
        // silently becomes zero seconds at a high attempt number. That shape was in this function
        // until `outer_retry_ac029`'s far-attempt row went red on it [measured 2026-09-14].
        if value >= BACKOFF_CAP_SECS {
            return BACKOFF_CAP_SECS;
        }
        value *= 2;
        done += 1;
    }
    if value < BACKOFF_CAP_SECS {
        value
    } else {
        BACKOFF_CAP_SECS
    }
}

/// **AC-029's full jitter**: the delay is uniform on `[0, ceiling]` for the supplied draw.
///
/// The draw is a parameter and this crate mints none. Full jitter is `U(0, min(cap, base ·
/// 2^attempt))`; the mapping is `draw · (ceiling + 1) / 2^64`, which uses no constant beyond the
/// word size of the draw itself.
#[must_use]
pub const fn backoff_delay(attempt: u32, draw: JitterDraw) -> u64 {
    let ceiling = backoff_ceiling_secs(attempt);
    // Uniform on [0, ceiling]: the draw is a fraction of 2^64 and the width is ceiling + 1, so a
    // draw of 0 yields 0 and a draw of u64::MAX yields the ceiling. No constant beyond the draw's
    // own word size appears here, which is what keeps this arithmetic rather than a generator.
    (((draw.0 as u128) * (ceiling as u128 + 1)) >> 64) as u64
}

/// **The policy.** One observed failure, one decision.
///
/// The attempt number comes from `key.attempt` and from nowhere else, so the delay and the cap
/// check cannot disagree about which attempt this is. The class comes from the shipped
/// `rk_types::error::StepError::retry_class`, which is ADR-0095 decision 2's table in a
/// checkpoint-passed crate; a second table here would be a second policy to keep in sync.
///
/// **The cap is checked before the class** — see the module header's C-5 section.
#[must_use]
pub fn decide(error: &StepError, key: &IdempotencyKey, cap: AttemptCap) -> RetryDecision {
    let attempt = key.attempt;

    // `Overage` is refused BEFORE a process is spawned, so it has no post-dispatch disposition to
    // reach. The branch is kept even though the class below refuses it anyway: a pre-spawn refusal
    // and a post-dispatch one are different moments, and collapsing them would lose the first.
    if refused_before_spawn(error) {
        return RetryDecision::Refuse {
            class: error_tag(error),
        };
    }

    // THE CAP IS CHECKED BEFORE THE CLASS (module header, conflict C-5): AC-029's "never exceeds
    // the pack's `N`" is an upper bound on every class that would re-dispatch, ADR-0095's
    // "retry once" included.
    let class = error.retry_class();
    if attempt >= cap.get() && class_would_redispatch(class) {
        return RetryDecision::CapReached { cap };
    }

    match class {
        RetryClass::Never => RetryDecision::Refuse {
            class: error_tag(error),
        },
        RetryClass::WaitUntilReset => RetryDecision::WaitForReset {
            resets_at: resets_at_of(error),
        },
        RetryClass::Backoff => RetryDecision::Backoff {
            attempt,
            delay_secs: backoff_delay(attempt, JitterDraw::from_key(key)),
        },
        // ADR-0095's `Timeout` row: one retry, carrying AC-029's envelope because AC-029's GIVEN
        // names `Timeout` explicitly, and then escalate.
        RetryClass::OnceThenEscalate => {
            if attempt <= 1 {
                RetryDecision::Backoff {
                    attempt,
                    delay_secs: backoff_delay(attempt, JitterDraw::from_key(key)),
                }
            } else {
                RetryDecision::EscalateAfterOne
            }
        }
        // AC-031's one re-prompt. That criterion's own THEN is the source of the ceiling; nothing
        // here is defaulted, and AC-031 is not claimed by this module.
        //
        // 🛑 THE COUNTER IS THE ERROR'S, NOT THE KEY'S, AND THAT IS A REPAIR OF 2026-09-14.
        // This gated on `attempt` — the activation's GLOBAL dispatch counter — while the real
        // schema-failure count lives in `StepError::Schema { attempts }`, which `run_activity`
        // increments per schema failure and hands in on the error. The two diverge whenever a
        // dispatch fails for any OTHER class first: measured on the shipped
        // `CountingHarness::scripted` with `[Err(Transport), wrong-schema, wrong-schema, ok]`, one
        // unrelated `Transport` failure consumed the re-prompt allowance and the FIRST schema
        // failure was refused outright — AC-031's *"re-prompt exactly once"* silently became
        // *"re-prompt only if nothing else failed first"*. A resume whose `next_attempt` is
        // non-zero has the same effect. `Schema`'s own field is the only counter that counts the
        // thing AC-031's THEN is about, so it is the one read here; every other class is unchanged
        // and still reads `attempt`.
        RetryClass::UpToAttempts => {
            // AN `if let`, NOT A `match` WITH A CATCH-ALL. AC-028's source assertion
            // (`ac028_policy_match_has_no_wildcard`) bans `_ =>` anywhere in this policy, and it
            // caught the first draft of this repair doing exactly that [measured 2026-09-14: the
            // test went red on the wildcard]. `UpToAttempts` is `Schema`'s class alone, so the
            // `else` is unreachable through `retry_class()`; this function is `pub`, so it degrades
            // to the old reading rather than panicking on a hand-made pair.
            let schema_failures = if let StepError::Schema { attempts, .. } = error {
                *attempts
            } else {
                attempt
            };
            if schema_failures < SCHEMA_REPROMPT_LIMIT {
                RetryDecision::Reprompt {
                    attempts: schema_failures.saturating_add(1),
                }
            } else {
                RetryDecision::Refuse {
                    class: error_tag(error),
                }
            }
        }
    }
}

/// Would this class ask for another dispatch at all?
///
/// Read only by [`decide`]'s cap check, so the cap bounds every class that re-dispatches and no
/// other. An exhaustive `match` over `RetryClass`'s five with no wildcard, for [`error_tag`]'s
/// reason one type over.
const fn class_would_redispatch(class: RetryClass) -> bool {
    match class {
        RetryClass::Backoff | RetryClass::UpToAttempts | RetryClass::OnceThenEscalate => true,
        RetryClass::WaitUntilReset | RetryClass::Never => false,
    }
}

/// The reset a `RateLimited` failure carried, and `None` for every other variant.
///
/// **Carried, never derived.** AC-030's own *Note* records that `resets_at` arrives on the
/// platform's `rate_limit_event` and not on a failure result, so an adapter derives it from the
/// stream; this crate has no clock with which to invent one. The third exhaustive no-wildcard
/// `match` over the eleven.
const fn resets_at_of(error: &StepError) -> Option<Timestamp> {
    match error {
        StepError::RateLimited { resets_at } => *resets_at,
        StepError::Transport { .. }
        | StepError::Schema { .. }
        | StepError::Timeout { .. }
        | StepError::Cancelled { .. }
        | StepError::NonZero { .. }
        | StepError::NotAdopted { .. }
        | StepError::NotIsolated { .. }
        | StepError::Platform { .. }
        | StepError::Unsupported { .. }
        | StepError::Overage { .. } => None,
    }
}

/// **AC-026 / ADR-0122 decision 2.** The policy for a failure the harness already retried **inside
/// the dispatch that produced it**.
///
/// > The kernel issues no outer retry for the failure that terminated a dispatch the harness
/// > already retried inside that dispatch. The suppression is **dispatch-scoped**, on AC-026's
/// > wording — *"for the same failure"* — and it suppresses nothing else.
///
/// So a later failure of the **same class on a different dispatch** is decided by [`decide`]
/// unchanged: AC-029's envelope, AC-033's breaker and ADR-0112's `Platform` disposition are
/// untouched. A class-scoped reading would silently disable outer retry for `Platform` for the rest
/// of a run, which is why the scope is named here rather than left to a call site.
///
/// # 🛑 NOTHING IN THE PRODUCT CALLS THIS, AND AC-026 IS THEREFORE ARMED, NOT DISCHARGED
///
/// *(Added 2026-09-14 on an adversarial verification pass, which reproduced it:
/// `grep -rn "decide_after_inner_retries\|charge_observed_inner_attempts" crates/*/src/` returns
/// **three lines, all definitions or doc text, and zero callers under `src/`**.)*
///
/// [`crate::runtime`]'s `Runtime::run_activity` is the only path in this tree that dispatches, and
/// its retry-decision site calls [`decide`], not this function. So **a `Platform` failure the
/// harness already retried inside the dispatch is re-dispatched by the shipped loop** — the
/// no-duplicate half that AC-026, AC-232 and AC-391 *all three* agree on — and no inner attempt is
/// charged onto any ledger.
///
/// **The kernel is genuinely powerless to close this at Step 7b, and that is the reason rather than
/// an excuse.** The charge unit is *"one charge per recorded `system/api_retry` event"* (ADR-0122
/// decision 1), and the kernel has no way to observe one: `EventKind` is closed at 29 variants with
/// **no `system/api_retry` member**, ADR-0122 decision 6 deliberately mints none, and
/// `rk_ports::harness::DispatchOutcome` / `rk_types::activity::DispatchRecord` carry **no
/// inner-attempt channel**. The stream those events arrive on is the *adapter's* — AC-232 is
/// `rk-adapter-claude`'s criterion and `rk-adapter-*` does not exist until **Step 7c**.
///
/// What is discharged here is the **rule**: the charge unit, the dispatch scope, and the refusal,
/// pinned by `inner_retry_ac026` over a hand-authored fixture shaped like the recorded rows. What
/// is ARMED is the **wiring** — the call site, which needs a carrier no record has minted. A record
/// routing inner attempts from the adapter stream to the kernel is **OWED** and is named here, not
/// written here (`00-preamble.md`:55).
#[must_use]
pub fn decide_after_inner_retries(
    error: &StepError,
    key: &IdempotencyKey,
    cap: AttemptCap,
    inner: InnerAttempts,
) -> RetryDecision {
    if inner.any() {
        return RetryDecision::Refuse {
            class: "harness_already_retried",
        };
    }
    decide(error, key, cap)
}

/// One recorded `system/api_retry` event, as the harness stream carries it.
///
/// The field set is the runner's own `ApiRetry` (`spikes/runner/src/runner.rs`:61) —
/// `{attempt, error, error_status, max_retries, retry_delay_ms}` — carried here as the three fields
/// the accounting reads. **`attempt` is carried and deliberately not summed**: ADR-0122 decision 1
/// makes the charge unit the *event*, and keeping the counter visible is what lets
/// `inner_retry_ac026` prove the two differ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRetryEvent {
    /// The harness's own attempt counter. **Restarts within a single dispatch on 37 of the 139
    /// recorded rows**, which is why it is not the charge.
    pub attempt: u32,
    /// The harness's ceiling — 10 on every recorded row.
    pub max_retries: u32,
    /// The platform status, where one was recorded.
    pub error_status: Option<u16>,
}

/// **AC-026's inner attempts. A counter, not a retry.**
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InnerAttempts(u32);

impl InnerAttempts {
    /// From a count the caller observed.
    #[must_use]
    pub const fn observed(n: u32) -> Self {
        Self(n)
    }

    /// **ADR-0122 decision 1's unit, and the whole of it:** *"The unit is the recorded event, not
    /// the terminal value of its `attempt` field. A dispatch whose stream carries n
    /// `system/api_retry` events charges n."*
    ///
    /// It is forced by measurement and not by taste — *"the counter restarts within a single
    /// dispatch on 37 of 139 recorded rows, and reading the charge off the counter would lose 61 of
    /// 284 attempts"* — which is exactly what `inner_retry_ac026` asserts over the committed
    /// fixture: 284 events against a 223 terminal-sum.
    #[must_use]
    pub fn from_recorded(events: &[ApiRetryEvent]) -> Self {
        Self(u32::try_from(events.len()).unwrap_or(u32::MAX))
    }

    /// The count.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Did the harness retry inside this dispatch at all?
    #[must_use]
    pub const fn any(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for InnerAttempts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} inner attempts", self.0)
    }
}

/// **AC-032's two channels**, spelled once so a call site cannot widen either.
///
/// > *a type test proving the success and failure channels are disjoint
/// > (`Result<Findings, StepError>`), so "the reviewer said no" cannot be constructed as a
/// > `StepError`.*
///
/// RK-B0013 is the defect: one HTTP 529 produced a committed `changes-requested` seal because
/// `!review` conflated *"the reviewer said no"* with *"the reviewer never spoke"*.
pub type ActivityOutcome = Result<Findings, StepError>;

/// **AC-032.** The **one** door from an activity outcome into the retry policy.
///
/// An `Ok` returns `None` — the policy is *never consulted* for a completed activity, findings and
/// all. There is no second path from an outcome to [`decide`], which is what makes *"dispatch count
/// 1"* a countable consequence rather than a convention.
#[must_use]
pub fn consult(
    outcome: &ActivityOutcome,
    key: &IdempotencyKey,
    cap: AttemptCap,
) -> Option<RetryDecision> {
    match outcome {
        Ok(_) => None,
        Err(error) => Some(decide(error, key, cap)),
    }
}

/// What the policy authorised, over one error class recurring until it stops.
///
/// House Shape C (`kernel-style.md` §2.2): the counts travel with the verdict, so a vacuous pass is
/// detectable. This is the **policy's own account** — it dispatches nothing, reaches no port and
/// spawns no process; the real loop is the runtime boundary's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrySchedule {
    decisions: Vec<RetryDecision>,
    surfaced: Option<StepError>,
}

impl RetrySchedule {
    /// Every decision in order, the terminal one last.
    #[must_use]
    pub fn decisions(&self) -> &[RetryDecision] {
        &self.decisions
    }

    /// **AC-027's *"the run fails with the configuration error surfaced"***, and residual 10f's
    /// reading for `Timeout`: the typed error the run ends with.
    ///
    /// `None` when the schedule ends in [`RetryDecision::WaitForReset`], because AC-030's THEN is
    /// that the kernel *"writes a `retry_at(resets_at)` mark and **exits 0**"* — a rate limit is
    /// not a failed run. That branch is **not built** here; only the absence of a surfaced failure
    /// is.
    #[must_use]
    pub const fn surfaced(&self) -> Option<&StepError> {
        self.surfaced.as_ref()
    }

    /// How many **re**-dispatches the policy authorised. AC-027's quantity.
    #[must_use]
    pub fn redispatches(&self) -> u32 {
        let n = self
            .decisions
            .iter()
            .filter(|d| d.authorises_redispatch())
            .count();
        u32::try_from(n).unwrap_or(u32::MAX)
    }

    /// How many dispatches in all — the first one plus [`RetrySchedule::redispatches`]. AC-029's
    /// *"the attempt count"*, and AC-032's *"dispatch count 1"*.
    #[must_use]
    pub fn dispatches(&self) -> u32 {
        self.redispatches().saturating_add(1)
    }
}

/// Drive [`decide`] over one error class recurring, starting at `key`'s attempt, until it stops.
///
/// Terminating by construction: every class either refuses at once or is bounded by `cap`, and
/// [`decide`] returns [`RetryDecision::CapReached`] at `attempt >= cap`.
#[must_use]
pub fn schedule(error: &StepError, key: &IdempotencyKey, cap: AttemptCap) -> RetrySchedule {
    let mut decisions = Vec::new();
    let mut attempt = if key.attempt == 0 { 1 } else { key.attempt };
    // The bound is the pack's and the loop cannot outrun it: `decide` returns `CapReached` at
    // `attempt >= cap`, which authorises nothing. The fence is a second, redundant stop so that a
    // future change to the policy cannot turn this into a non-terminating function.
    let fence = cap.get().saturating_add(1);
    while attempt <= fence {
        let at = IdempotencyKey {
            item: key.item.clone(),
            activity_id_at_version: key.activity_id_at_version.clone(),
            attempt,
        };
        let decision = decide(error, &at, cap);
        let again = decision.authorises_redispatch();
        decisions.push(decision);
        if !again {
            break;
        }
        attempt = attempt.saturating_add(1);
    }

    // AC-027's "the run fails with the configuration error surfaced", and AC-030's "exits 0" read
    // in reverse: a rate limit is not a failed run, so no error is surfaced for one.
    let surfaced = decisions.last().and_then(|last| match last {
        RetryDecision::WaitForReset { .. } => None,
        RetryDecision::Backoff { .. }
        | RetryDecision::Reprompt { .. }
        | RetryDecision::EscalateAfterOne
        | RetryDecision::Refuse { .. }
        | RetryDecision::CapReached { .. } => Some(error.clone()),
    });

    RetrySchedule {
        decisions,
        surfaced,
    }
}

/// [`schedule`] over an outcome. An `Ok` schedules **nothing**: one dispatch, no decisions, no
/// surfaced error. **AC-032.**
#[must_use]
pub fn schedule_for(
    outcome: &ActivityOutcome,
    key: &IdempotencyKey,
    cap: AttemptCap,
) -> RetrySchedule {
    match outcome {
        Ok(_) => RetrySchedule {
            decisions: Vec::new(),
            surfaced: None,
        },
        Err(error) => schedule(error, key, cap),
    }
}
