//! **The pack-declared runtime envelope** — `K`, `N` and the per-item token appetite, and the
//! refusal a pack that declares none produces.
//!
//! | criterion | what this module carries | state |
//! | --- | --- | --- |
//! | **AC-029** (2/2) | [`AttemptCap`] — the pack's `N`, and [`EnvelopeError::AttemptCapNotDeclared`], which is what a pack declaring none produces | **built**; the backoff envelope itself is Slice 4c's |
//! | **AC-033** (part) | [`BreakerThreshold`] — the pack's `K`, *"a pack value, asserted present by pack validation"* — and, as of 2026-09-14 (D-7), [`EnvelopeError::BreakerExpiryNotDeclared`] and [`EnvelopeError::RoleFamilyNotDeclared`], the refusals for the two values the same decision made pack-declared | the **types and the accessors** are built; the breaker's behaviour and its durable mark are [`super::breaker`]'s and **nothing here may be read as covering AC-033** |
//! | **AC-023 / AC-389** (part) | [`EnvelopeError::TokenAppetiteNotDeclared`] — a gate is not evaluated against a **defaulted** token budget | **built**, as the refusal only. C3 is **RESOLVED** in favour of AC-389 (DECISIONS-2026-09-14 D-13): an in-run raise is refused **outright** and AC-023's *"unless"* clause is **CUT**, so there is no appetite-raise mechanism left to build. AC-389's *"the attempt is appended as an event"* is still unbuilt — residual **R-1** in `spikes/DECISIONS-2026-09-15.md` (see [`super::budget`]) |
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
//! → 0 hits]. The keys this build reads are stated here as a **reading**, not as a discovery:
//!
//! | value | key read | why this spelling |
//! | --- | --- | --- |
//! | `N` | `[retry] attempts_cap` | AC-029's own words — *"the attempt count never exceeds the pack's `N`"* — and `[retry]` is the table the activity-graph schema already uses for the per-step `retry = { max }` |
//! | `K` | `[breaker] consecutive_platform_failures` | AC-033's GIVEN is *"K consecutive `Platform` failures"*, and `Platform` is ADR-0112's variant; a bare `k = 3` would be unreadable at the call site |
//! | the appetite | `appetite.tokens` | it sits beside `appetite.points`, which `select/pack.rs` already reads, and the two are **different counters** — points are step 5's cost-class sum (AC-058, invariant I6) and tokens are AC-020's second counter |
//! | the breaker expiry | `[breaker] expiry_secs` | **added 2026-09-14 (D-7)** — it sits in the table that already holds `K`, which is the co-location the decision chose it for, and `_secs` is the suffix the activity-graph schema's own `wall_clock_secs` and `timeout_secs` already use |
//! | the breaker's key, per role | `[role.<name>] family` | **added 2026-09-14 (D-7)** — design :480 already puts a `[role.<name>]` table in the process pack (*"tier policy per role"*, `[role.implementer] tier = { model = …, effort = … }`), so this is one key beside `tier` and not a new table shape |
//!
//! **A record naming all FIVE keys is OWED.** *(This paragraph said "the three keys" and "A record
//! naming the three keys is OWED" until 2026-09-14; D-7 added two and changed nothing about the
//! rule.)* Until it lands these are this crate's proposal, and a pack author following a future
//! record with different spellings gets `None` and a refusal — which is the failure direction the
//! whole module is built in, and not a silent default. [Re-measured 2026-09-14 for the two new ones:
//! `grep -rn 'expiry_secs\|role' spikes/p9/packs/` → **0 hits across all twelve shipped packs**, so
//! no shipped pack contradicts either spelling and none satisfies it; the shipped packs' own tables
//! are **Step 7e's**.]
//!
//! # 🛑 AND THE AUTHORITY FOR "DECLARED IN THE PACK" IS AC-029's AND AC-033's, NOT AC-023's
//!
//! §7b.10:854-855 cites **AC-023** for *"`K`, `N` and the appetite are declared in the pack and
//! refused as code defaults"*. AC-023's GIVEN/WHEN/THEN is about a **mid-run appetite raise** and
//! says nothing about pack declaration, so on that clause the stage file is adding a THEN, which
//! ADR-0092's precedence does not let it do. The two criteria that **do** say it in their own words
//! are cited above instead — AC-029's *Verified by* for `N` and AC-033's for `K` — and the token
//! appetite's refusal is built on the same shape without claiming AC-023. **C3 is RESOLVED in
//! favour of AC-389 — refused outright, AC-023's *"unless"* clause CUT (DECISIONS-2026-09-14
//! D-13)** — and that changes nothing here: the authority for *"declared in the pack"* was never
//! AC-023's, before the resolution or after it. This sentence ended *"whose contradiction C3 is
//! open. Nothing here decides C3"* until 2026-09-15. Nothing here is named `ac023_…`, now for the
//! narrower reason that this module asserts no appetite refusal at all — the one that is built is
//! `tests/it/budget.rs`'s.
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

use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use super::budget::Tokens;

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
    /// `[breaker] expiry_secs` is absent, so AC-035's *"until the mark is read and found expired"*
    /// has no duration to compare against. **Added 2026-09-14 (operator decision D-7)**, which is
    /// what gave the expiry a home: it is pack-declared, beside `K`.
    BreakerExpiryNotDeclared {
        /// The pack's `[pack] id`.
        pack: String,
    },
    /// `[role.<role>] family` is absent, so the breaker has no partition key for a step that
    /// nominates that role. **Added 2026-09-14 (operator decision D-7).**
    ///
    /// The refusal names **both** the pack and the role, because the two failures an operator has to
    /// tell apart are *"this pack declares no role table at all"* and *"this pack declares one and
    /// this role is missing from it"* — and a message naming only the pack collapses them.
    RoleFamilyNotDeclared {
        /// The pack's `[pack] id`.
        pack: String,
        /// The role the step nominated, as the catalog spelled it.
        role: String,
    },
    /// `appetite.dispatches` is absent, so there is no per-item dispatch cap. *(Added 2026-10-02,
    /// `s8h` build, package P5-kernel-envelope; `s8h` §8h.9; Record B = ADR-0198 decision 4,
    /// proposed: "An absent `appetite.dispatches` is refused by pack validation as an
    /// `EnvelopeError` beside `TokenAppetiteNotDeclared`, naming the pack".)*
    DispatchCapNotDeclared {
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
            Self::BreakerExpiryNotDeclared { .. } => "breaker_expiry_not_declared",
            Self::RoleFamilyNotDeclared { .. } => "role_family_not_declared",
            Self::DispatchCapNotDeclared { .. } => "dispatch_cap_not_declared",
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
            Self::BreakerExpiryNotDeclared { pack } => write!(
                f,
                "pack `{pack}` declares no `[breaker] expiry_secs`, so an open breaker mark has no \
                 duration to be found expired against (AC-035). It is not defaulted here and it is \
                 not derived from the backoff cap: a default with arithmetic in front of it is \
                 still a default. An absent declaration is an absence, never a mark that never \
                 expires and never one that expires at once"
            ),
            Self::RoleFamilyNotDeclared { pack, role } => write!(
                f,
                "pack `{pack}` declares no `[role.{role}] family`, so a step nominating the role \
                 `{role}` has no model family for the circuit breaker to be keyed on (AC-033, \
                 AC-034). It is not defaulted here and the role is NOT used as its own family: N \
                 roles map to one model, so a per-role key would stop one role's share of traffic \
                 to a failing model while the rest kept flowing. The activity's own `family` is \
                 ADR-0058's ACTIVITY family and is never read here"
            ),
            Self::DispatchCapNotDeclared { pack } => write!(
                f,
                "pack `{pack}` declares no `appetite.dispatches`, so the runtime has no per-item \
                 dispatch cap. It is not defaulted here: the cap and the token appetite are \
                 declared in the pack and refused as code defaults (Record B = ADR-0198 decision \
                 4). `appetite.points` and `appetite.tokens` are different counters (AC-020)"
            ),
        }
    }
}

impl std::error::Error for EnvelopeError {}

// ---- the per-item envelope (s8h §8h.9; Record B = ADR-0198 decision 4, proposed) -----------------
//
// *(dated 2026-10-02, `s8h` build, package P5-kernel-envelope.)* D21-42 19a: *"The kernel enforces a
// per-item dispatch-count cap and token appetite, refusing with a typed class. First values: 40
// dispatches, and about 2x the happy-path token estimate."* D21-43 N17 Q2: the appetite counts the
// sum of `Usage`'s four counters; Q3: the refusal leaves the CLI as `Exit::Overage` (20). Record B
// took finding 12's question: the refusal is THIS kernel class, never `StepError::Overage`, because
// it is made before the `Harness` is called and a `StepError` is the port's outcome channel.

/// The pack's per-item dispatch cap, `appetite.dispatches`. No `Default`, no `const` fallback, for
/// [`AttemptCap`]'s reason: [`DispatchCap::declared`] is the only constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DispatchCap(u32);

impl DispatchCap {
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

impl fmt::Display for DispatchCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} dispatches", self.0)
    }
}

/// **The envelope's refusal: one variant per counter** (Record B decision 4's drafted
/// `DispatchCapReached { item, count, cap }` and `TokenAppetiteReached { item, sum, appetite }`).
///
/// Made before any spawn and writing no row; the runtime carries it as
/// `crate::runtime::RuntimeError::Envelope`, and `rk-cli` maps it to `Exit::Overage` (20).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvelopeRefusal {
    /// The item's prior `dispatch` rows have reached `appetite.dispatches`.
    DispatchCapReached {
        /// The item the rows are keyed to.
        item: String,
        /// How many prior `dispatch` rows the store holds for it.
        count: u32,
        /// The pack's cap.
        cap: DispatchCap,
    },
    /// The item's charged tokens have reached `appetite.tokens`.
    TokenAppetiteReached {
        /// The item the rows are keyed to.
        item: String,
        /// The sum of the four `Usage` counters over its `dispatch_completed` rows.
        sum: Tokens,
        /// The pack's appetite.
        appetite: Tokens,
    },
}

impl EnvelopeRefusal {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DispatchCapReached { .. } => "dispatch_cap_reached",
            Self::TokenAppetiteReached { .. } => "token_appetite_reached",
        }
    }
}

impl fmt::Display for EnvelopeRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DispatchCapReached { item, count, cap } => write!(
                f,
                "item `{item}` has made {count} dispatches and the pack's `appetite.dispatches` \
                 is {cap}; the next dispatch is refused before any spawn (Record B = ADR-0198 \
                 decision 4). A budget state that does not improve by being repeated"
            ),
            Self::TokenAppetiteReached {
                item,
                sum,
                appetite,
            } => write!(
                f,
                "item `{item}` has been charged {sum} (the four `Usage` counters over its \
                 `dispatch_completed` rows) and the pack's `appetite.tokens` is {appetite}; the \
                 next dispatch is refused before any spawn (Record B = ADR-0198 decision 4)"
            ),
        }
    }
}

impl std::error::Error for EnvelopeRefusal {}

/// **The per-item envelope a runtime enforces**: the pack's dispatch cap and token appetite, both
/// declared ([`crate::select::Pack::require_item_envelope`] is the only way to obtain one).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemEnvelope {
    cap: DispatchCap,
    appetite: Tokens,
}

impl ItemEnvelope {
    /// From the two values a pack declared.
    #[must_use]
    pub const fn declared(cap: DispatchCap, appetite: Tokens) -> Self {
        Self { cap, appetite }
    }

    /// `appetite.dispatches`.
    #[must_use]
    pub const fn cap(self) -> DispatchCap {
        self.cap
    }

    /// `appetite.tokens`.
    #[must_use]
    pub const fn appetite(self) -> Tokens {
        self.appetite
    }

    /// The item's prior dispatches: the `dispatch` rows KEYED to `item` (ADR-0182's interim key).
    /// `query(ByItem)` also returns unkeyed rows in the interim (ADR-0182 decision 4), and those
    /// are not this item's, so they are not counted (Record B decision 4).
    #[must_use]
    pub fn prior_dispatches(events: &[Event], item: &str) -> u32 {
        let n = events
            .iter()
            .filter(|e| e.kind == EventKind::Dispatch && e.item() == Some(item))
            .count();
        u32::try_from(n).unwrap_or(u32::MAX)
    }

    /// The item's charged tokens: the sum of `Usage`'s four counters — `input_tokens`,
    /// `output_tokens`, `cache_creation_input_tokens` and `cache_read_input_tokens`, each read
    /// verbatim, the two cache counters falling back to their `ephemeral_*` namings as
    /// `rk_types::usage::Usage`'s readers do — over the `dispatch_completed` rows keyed to `item`
    /// (D21-43 N17 Q2). A row whose `usage` is absent or `null` charges nothing, and a counter that
    /// did not arrive adds nothing: the sum is of what was reported, so a dispatch that reported
    /// nothing is bounded by the CAP, which is why the cap exists beside the appetite.
    #[must_use]
    pub fn charged_tokens(events: &[Event], item: &str) -> Tokens {
        let mut sum: u64 = 0;
        for e in events
            .iter()
            .filter(|e| e.kind == EventKind::DispatchCompleted && e.item() == Some(item))
        {
            let Some(usage) = e.payload.get("usage").filter(|u| !u.is_null()) else {
                continue;
            };
            let Ok(usage) = serde_json::from_value::<rk_types::usage::Usage>(usage.clone()) else {
                continue;
            };
            let counters = [
                usage.input_tokens,
                usage.output_tokens,
                usage
                    .cache_creation_input_tokens
                    .or(usage.ephemeral_cache_creation_input_tokens),
                usage
                    .cache_read_input_tokens
                    .or(usage.ephemeral_cache_read_input_tokens),
            ];
            for n in counters.into_iter().flatten() {
                sum = sum.saturating_add(n);
            }
        }
        Tokens::new(sum)
    }

    /// **Admit the item's next dispatch, or refuse it**, from `events` — the store's journal for
    /// the item, read before the dispatch. The dispatch AT the cap proceeds (prior count `cap - 1`);
    /// the one past it does not (prior count `cap`). The cap is checked first.
    ///
    /// # Errors
    ///
    /// [`EnvelopeRefusal::DispatchCapReached`] once the prior count has reached the cap;
    /// [`EnvelopeRefusal::TokenAppetiteReached`] once the charged sum has reached the appetite.
    pub fn admit(self, events: &[Event], item: &str) -> Result<(), EnvelopeRefusal> {
        let count = Self::prior_dispatches(events, item);
        if count >= self.cap.get() {
            return Err(EnvelopeRefusal::DispatchCapReached {
                item: item.to_owned(),
                count,
                cap: self.cap,
            });
        }
        let sum = Self::charged_tokens(events, item);
        if sum >= self.appetite {
            return Err(EnvelopeRefusal::TokenAppetiteReached {
                item: item.to_owned(),
                sum,
                appetite: self.appetite,
            });
        }
        Ok(())
    }
}
