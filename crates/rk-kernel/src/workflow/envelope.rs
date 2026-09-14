//! **The pack-declared runtime envelope** — `K`, `N` and the per-item token appetite, and the
//! refusal a pack that declares none produces.
//!
//! | criterion | what this module carries | state |
//! | --- | --- | --- |
//! | **AC-029** (2/2) | [`AttemptCap`] — the pack's `N`, and [`EnvelopeError::AttemptCapNotDeclared`], which is what a pack declaring none produces | **built**; the backoff envelope itself is Slice 4c's |
//! | **AC-033** (part) | [`BreakerThreshold`] — the pack's `K`, *"a pack value, asserted present by pack validation"* | the **type and the accessor** are built; the breaker's behaviour, its durable mark and its own refusal are Slice 4d's and **nothing here may be read as covering AC-033** |
//! | **AC-023 / AC-389** (part) | [`EnvelopeError::TokenAppetiteNotDeclared`] — a gate is not evaluated against a **defaulted** token budget | **built**, as the refusal only; C3 is open and the appetite-raise mechanism is not built (see [`super::budget`]) |
//!
//! # The rule this module exists to make unwritable
//!
//! §7b.10's 🛑, verbatim (`runbook/phase-1/s7b-kernel.md`:852-857):
//!
//! > **`K`, `N`, the backoff base and cap, the two-tick headroom margin and the per-item token
//! > appetite HAVE NO MEASURED VALUES** (design §3.6). `K`, `N` and the appetite are **declared in
//! > the pack and refused as code defaults** (AC-033, AC-023); the backoff base and cap are kernel
//! > policy constants (AC-029). The criteria verify conformance to the **declared envelope**, never
//! > that the envelope is right — so build the refusal and the conformance check, and **do not
//! > default a constant here** (`00-preamble.md`:55).
//!
//! So [`AttemptCap`] and [`BreakerThreshold`] derive **no `Default`**, carry no `const` fallback and
//! are constructed only by [`AttemptCap::declared`] / [`BreakerThreshold::declared`] — names chosen
//! so that a call site reads as a transcription of a declaration and never as a choice. The model is
//! `Margins` (AC-377: *"`Margins` implements no `Default`"*) and [`super::budget::BudgetEnvelope`],
//! which derives none for the same reason.
//!
//! **The backoff base and cap are deliberately NOT here.** PIA-055 (register §0.3's authority table,
//! quoted on AC-029's *Home of the numbers* line) puts them the other way round: *"base and cap are
//! `rk-kernel::retry` constants, `K`/`N`/the appetite are pack values, and design §3.6/§3.10 are
//! amended toward this."* A pack key for either would violate the register as surely as a code
//! default for `N` does, and both mistakes are one edit away from each other. The constants are
//! Slice 4c's, in `rk-kernel::retry`, and they are not stubbed here.
//!
//! # What `None` means, which is the whole of residual 8
//!
//! An absent declaration is `None`, and **`None` is not "unbounded"** — the doctrine is
//! `select/pack.rs`:249-256's, already shipped for the sibling field: *"Step 5 runs no appetite
//! check for a pack that declares none and records the absence, because a code default here would
//! decide a value the pack owes (`00-preamble.md`:55)."* The accessors on [`crate::select::Pack`]
//! therefore return `Option`, and the **refusal is a separate call** — `require_attempt_cap` /
//! `require_token_appetite` — so that the place where a missing declaration fails is the
//! **pack-validation** half AC-029 names (*"a pack-validation test that `N` is declared, never
//! defaulted in code"*) and AC-033 names (*"`K` is a pack value, asserted present by pack
//! validation"*), and **not** the loader: a pack that declares no `[retry]` table still loads, and
//! every fixture pack in this crate that predates 2026-09-14 still loads unchanged.
//!
//! # 🛑 THE TOML KEY NAMES ARE A READING. NO RECORD NAMES THEM, AND A RECORD IS OWED
//!
//! `K` and `N` are named in every record that discusses them and **spelled in none**: there is no
//! `attempts_cap`, `consecutive_platform_failures` or `appetite.tokens` key in any shipped pack
//! [measured 2026-09-14: `grep -rn 'attempts_cap\|consecutive_platform\|tokens' spikes/p9/packs/`
//! → 0 hits]. The three keys this build reads are stated here as a **reading**, not as a discovery:
//!
//! | value | key read | why this spelling |
//! | --- | --- | --- |
//! | `N` | `[retry] attempts_cap` | AC-029's own words — *"the attempt count never exceeds the pack's `N`"* — and `[retry]` is the table the activity-graph schema already uses for the per-step `retry = { max }` |
//! | `K` | `[breaker] consecutive_platform_failures` | AC-033's GIVEN is *"K consecutive `Platform` failures"*, and `Platform` is ADR-0112's variant; a bare `k = 3` would be unreadable at the call site |
//! | the appetite | `appetite.tokens` | it sits beside `appetite.points`, which `select/pack.rs` already reads, and the two are **different counters** — points are step 5's cost-class sum (AC-058, invariant I6) and tokens are AC-020's second counter |
//!
//! A record naming the three keys is **OWED**. Until it lands these are this crate's proposal, and a
//! pack author following a future record with different spellings gets `None` and a refusal — which
//! is the failure direction the whole module is built in, and not a silent default.
//!
//! # 🛑 AND THE AUTHORITY FOR "DECLARED IN THE PACK" IS AC-029's AND AC-033's, NOT AC-023's
//!
//! §7b.10:854-855 cites **AC-023** for *"`K`, `N` and the appetite are declared in the pack and
//! refused as code defaults"*. AC-023's GIVEN/WHEN/THEN is about a **mid-run appetite raise** and
//! says nothing about pack declaration, so on that clause the stage file is adding a THEN, which
//! ADR-0092's precedence does not let it do. The two criteria that **do** say it in their own words
//! are cited above instead — AC-029's *Verified by* for `N` and AC-033's for `K` — and the token
//! appetite's refusal is built on the same shape without claiming AC-023, whose contradiction C3 is
//! open. Nothing here decides C3 and nothing here is named `ac023_…`.
//!
//! # What is deliberately NOT here
//!
//! **Corrected 2026-09-14, Slice 4d.** The paragraph that stood here said
//! *"`EnvelopeError::BreakerThresholdNotDeclared` is design §2.4's third variant and is **not
//! declared**: nothing at Slice 4b constructs it, and a variant no path can reach reads as a
//! refusal that has been built … Slice 4d adds it beside the breaker whose refusal it is."* Slice
//! 4d has: the variant is below, [`crate::select::Pack::require_breaker_threshold`] constructs it,
//! and `breaker_ac033_ac035` asserts it against a pack that declares no `[breaker]` table. The
//! rule the paragraph carried is unchanged and is why the variant waited.
//!
//! Still **not here**: `BreakerState`, `OpenMark` and `BreakerDecision` live in
//! [`super::breaker`] rather than in this module, because this module holds the **declared
//! numbers** and the breaker's state is a fold over a journal. `RetryDecision`, `decide`,
//! `backoff_delay`, `BACKOFF_BASE_SECS`, `BACKOFF_CAP_SECS` and `InnerAttempts` are
//! [`super::retry`]'s, for the same split.

use std::fmt;

/// The pack's `N` — **AC-029's attempt cap.** No `Default`, no `const` fallback, no `unwrap_or`.
///
/// > THEN the delay is exponential with full jitter, base 2 s, capped at 5 min, and **the attempt
/// > count never exceeds the pack's `N`**.
///
/// The constructor is named [`AttemptCap::declared`] rather than `new` because there is no other
/// lawful way for one to come into existence: every call site is transcribing a number a pack
/// declared, and a `new` would read as a value this crate is entitled to choose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttemptCap(u32);

impl AttemptCap {
    /// From the value a pack declared. **The only constructor.**
    #[must_use]
    pub const fn declared(n: u32) -> Self {
        Self(n)
    }

    /// The declared cap.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for AttemptCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} attempts", self.0)
    }
}

/// The pack's `K` — **AC-033's breaker threshold.** No `Default`, for [`AttemptCap`]'s reason.
///
/// > **`K` is a pack value, asserted present by pack validation.**
///
/// The threshold is carried here because it is a **declared** number and this module is where the
/// declared numbers live. **The breaker is not built**: its state, its per-family keying, its
/// durable mark and its refusal are Slice 4d's, and no test in Slice 4b may be read as covering
/// AC-033 — what is asserted at 4b is that `K` is read from the pack and is not defaulted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BreakerThreshold(u32);

impl BreakerThreshold {
    /// From the value a pack declared. **The only constructor.**
    #[must_use]
    pub const fn declared(k: u32) -> Self {
        Self(k)
    }

    /// The declared threshold.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for BreakerThreshold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} consecutive platform failures", self.0)
    }
}

/// Why a pack cannot drive the runtime: it declares a number the runtime may not choose for it.
///
/// A hand-rolled enum with a hand-written `Display`, on this crate's ten-error precedent
/// (`catalog.rs`:214, `select/pack.rs`:51, `replay.rs`:127, …) — there is no `thiserror` in this
/// workspace. Each variant names the **pack**, so the failure names a file an operator can open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvelopeError {
    /// `[retry] attempts_cap` is absent. **AC-029's pack-validation half.**
    AttemptCapNotDeclared {
        /// The pack's `[pack] id`.
        pack: String,
    },
    /// `appetite.tokens` is absent, so there is no per-item token budget to evaluate a gate against.
    TokenAppetiteNotDeclared {
        /// The pack's `[pack] id`.
        pack: String,
    },
    /// `[breaker] consecutive_platform_failures` is absent. **AC-033's pack-validation half**,
    /// added 2026-09-14 at Slice 4d, where the breaker that needs `K` is built. The variant was
    /// deliberately absent at Slice 4b — *"nothing at 4b would assert against it"* — because an
    /// error variant no call site can produce reads as a refusal the system performs.
    BreakerThresholdNotDeclared {
        /// The pack's `[pack] id`.
        pack: String,
    },
}

impl EnvelopeError {
    /// A stable tag, so a test names the refusal it expects without matching message text
    /// (`kernel-style.md` §2.3).
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AttemptCapNotDeclared { .. } => "attempt_cap_not_declared",
            Self::TokenAppetiteNotDeclared { .. } => "token_appetite_not_declared",
            Self::BreakerThresholdNotDeclared { .. } => "breaker_threshold_not_declared",
        }
    }
}

impl fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AttemptCapNotDeclared { pack } => write!(
                f,
                "pack `{pack}` declares no `[retry] attempts_cap`, so the runtime has no attempt \
                 cap `N`. It is not defaulted here: `K`, `N` and the per-item token appetite are \
                 declared in the pack and refused as code defaults (AC-029's pack-validation half; \
                 §7b.10). An absent declaration is an absence, never \"unbounded\""
            ),
            Self::TokenAppetiteNotDeclared { pack } => write!(
                f,
                "pack `{pack}` declares no `appetite.tokens`, so there is no per-item token budget \
                 to evaluate a gate against. It is not defaulted here, and `appetite.points` is a \
                 different counter — step 5's cost-class sum (AC-058), not AC-020's tokens"
            ),
            Self::BreakerThresholdNotDeclared { pack } => write!(
                f,
                "pack `{pack}` declares no `[breaker] consecutive_platform_failures`, so there is \
                 no `K` for the circuit breaker to trip at. It is not defaulted here: \
                 AC-033's own *Verified by* says `K` is a pack value, asserted present by \
                 pack validation, and no corpus figure sizes it (gap 3). An absent \
                 declaration is an absence, never a breaker that never trips"
            ),
        }
    }
}

impl std::error::Error for EnvelopeError {}
