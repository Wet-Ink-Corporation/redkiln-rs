//! The pack-declared runtime envelope — **AC-029's second instrument**, and the premise that the
//! three declared numbers do not alias.
//!
//! # Why this file is not `tests/it/retry.rs`
//!
//! Design §4 files `ac029_n_is_declared_never_defaulted` in `it/retry.rs` beside `outer_retry_ac029`
//! and the four other retry names. **`src/workflow/retry.rs` does not exist**: the backoff envelope,
//! `decide`, `BACKOFF_BASE_SECS`, `BACKOFF_CAP_SECS` and `InnerAttempts` are Slice 4c's, and design
//! §5's own ordering table puts them there. A `tests/it/retry.rs` holding one test that asserts
//! nothing about retry would be the shape `tests/it/main.rs`'s header refuses — *"no empty module
//! stands in for an unbuilt one"* — one level up: a module named for a criterion range that is
//! nine-tenths unbuilt. The test's subject is the **declared envelope refused as a code default**,
//! which is Slice 4b's whole title, so it lives beside the module that carries it
//! (`src/workflow/envelope.rs`). The checkpoint grep is over the test NAME
//! (`^test .*\bac029_n_is_declared_never_defaulted\b.* \.\.\. ok$`) and not over its file, so the
//! move costs no assertion.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | measured here | not here |
//! | --- | --- | --- |
//! | **AC-029** (2/2) | *"a pack-validation test that `N` is declared, never defaulted in code"* | the whole first instrument — injected clock, seeded jitter, the delay envelope, base 2 s, cap 5 min, the cap decision — is `outer_retry_ac029`'s and is **Slice 4c's**. Nothing here asserts a delay |
//! | **AC-033** | nothing | `K`'s accessor is exercised as a **premise** (that the three accessors do not alias), and the breaker — its per-family state, its refusal, its durable mark — is Slice 4d's. No assertion here may be read as covering AC-033 |
//! | **AC-023** | nothing | the token appetite's accessor is exercised as the same premise. C3 is **RESOLVED** in favour of AC-389 — refused outright, AC-023's *"unless"* clause **CUT** (DECISIONS-2026-09-14 D-13). The refusal that IS built is in `tests/it/budget.rs`, whose `ac023_…` taboo the resolution **LIFTS** (the rename is Checkpoint 7b's and is not done here). AC-389's append half is still unbuilt — residual **R-1**, `spikes/DECISIONS-2026-09-15.md` |

use rk_kernel::budget::Tokens;
use rk_kernel::select::Pack;
use rk_kernel::workflow::envelope::{AttemptCap, BreakerThreshold, EnvelopeError};

/// The pack that declares all three. **Compiled in, never opened** — a fixture loaded from a path
/// is a `Result` an implementer can `unwrap_or_default()` into nothing, and a rule over nothing
/// passes (`tests/it/support.rs`:6-12).
const FIXTURE_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-envelope.toml");
/// The same pack with no `[retry]` table.
const FIXTURE_NO_N: &str = include_str!("../fixtures/packs/runtime-no-n.toml");
/// A **shipped-shape** pack that declares `appetite = { points = 4 }` and no envelope at all.
const FIXTURE_POINTS_ONLY: &str = include_str!("../fixtures/packs/incident-appetite.toml");

/// Load one pack fixture.
///
/// # Panics
///
/// With the loader's own error, so a fixture that stops parsing names its reason.
fn pack(name: &str, src: &str) -> Pack {
    Pack::from_toml(name, src).unwrap_or_else(|e| panic!("the {name} fixture loads: {e}"))
}

// ---- AC-029 (2/2) — `N` is declared, never defaulted in code ------------------------------------

/// **AC-029.** *"THEN the delay is exponential with full jitter, base 2 s, capped at 5 min, and the
/// attempt count never exceeds the pack's `N`. Verified by: a unit test with an injected clock and
/// seeded jitter asserting the delay envelope and the attempt cap; **a pack-validation test that
/// `N` is declared, never defaulted in code**."*
///
/// This is the **second** instrument and only the second. The first — the delay envelope, the two
/// kernel constants and the cap decision — is `outer_retry_ac029`'s and is Slice 4c's; nothing here
/// computes a delay, and a green here is not coverage of AC-029.
///
/// **What would make it vacuous, and what stops each:**
///
/// * A code default. An `AttemptCap` deriving `Default`, or an accessor written `unwrap_or(3)`,
///   would return `Some(0)` or `Some(3)` for the no-`N` pack; both are asserted against by name, so
///   a defaulted value cannot pass as an absence.
/// * A mis-keyed read. The envelope pack declares `N = 4`, `K = 3` and `appetite.points = 50` —
///   **four different numbers with the token appetite** — so an accessor reading the wrong key
///   returns a value this test names and rejects. On a pack declaring 3, 3 and 3 the whole test
///   would pass against an accessor that read any of them.
/// * A validator that refuses everything. The no-`N` pack must still **LOAD**, and the envelope
///   pack must return `Ok`; the refusal is at the pack-validation call and not at the loader.
///
/// 🛑 `K` and the token appetite are asserted here as a **premise** — that the three accessors do
/// not alias — and **not as coverage of AC-033 or AC-023**. The breaker is Slice 4d's, and C3 over
/// AC-023 is **RESOLVED** in favour of AC-389 (DECISIONS-2026-09-14 D-13) — which changes nothing
/// here, because this test never asserted an appetite raise either way. This sentence read *"C3
/// over AC-023 is open"* until 2026-09-15.
#[test]
fn ac029_n_is_declared_never_defaulted() {
    let declared = pack("runtime-envelope.toml", FIXTURE_ENVELOPE);

    // THE PREMISE: the three declared numbers are three DIFFERENT numbers, or an accessor that read
    // the wrong key would satisfy every assertion below.
    assert_eq!(declared.attempt_cap(), Some(AttemptCap::declared(4)));
    assert_eq!(
        declared.breaker_threshold(),
        Some(BreakerThreshold::declared(3))
    );
    assert_eq!(declared.token_appetite(), Some(Tokens::new(50_000)));
    assert_eq!(
        declared.appetite(),
        Some(50),
        "`appetite.points` is a fourth number and a different counter: step 5's cost-class sum \
         (AC-058), never AC-020's tokens"
    );

    // ---- the pack-validation call returns the DECLARED value, not a chosen one -----------------
    let cap = declared
        .require_attempt_cap()
        .expect("the pack declares `[retry] attempts_cap`");
    assert_eq!(cap.get(), 4, "the cap is the pack's number, transcribed");

    // ---- and a pack that declares none REFUSES, rather than being handed a default -------------
    let absent = pack("runtime-no-n.toml", FIXTURE_NO_N);
    assert_eq!(
        absent.attempt_cap(),
        None,
        "an absent declaration is an absence: `None` is not \"unbounded\" and not a number \
         (`select/pack.rs`:249-256)"
    );
    assert_ne!(
        absent.attempt_cap(),
        Some(AttemptCap::declared(0)),
        "a defaulted `N` would most likely arrive as zero, and zero is a policy value this crate \
         does not own (`00-preamble.md`:55)"
    );
    assert_ne!(
        absent.attempt_cap(),
        Some(AttemptCap::declared(3)),
        "3 is `K`, which this pack DOES declare: an accessor reading `[breaker] \
         consecutive_platform_failures` would land here"
    );

    let err = absent
        .require_attempt_cap()
        .expect_err("a pack declaring no `[retry] attempts_cap` cannot drive the runtime");
    assert_eq!(
        err,
        EnvelopeError::AttemptCapNotDeclared {
            pack: "runtime-no-n-fixture".to_owned()
        },
        "the refusal names the pack, so an operator has a file to open"
    );
    assert_eq!(
        err.as_str(),
        "attempt_cap_not_declared",
        "the stable tag, so this assertion does not match message text"
    );

    // THE CONTROL, first half: the no-`N` pack must still LOAD. Without this the assertions above
    // are satisfied by a loader that rejects every pack lacking an envelope — which would make the
    // six pack fixtures that predate 2026-09-14, and every shipped pack, unloadable.
    assert_eq!(absent.id, "runtime-no-n-fixture");
    assert_eq!(
        absent.breaker_threshold(),
        Some(BreakerThreshold::declared(3)),
        "the pack still declares `K`: only `N` is absent, and the two absences are independent"
    );

    // THE CONTROL, second half, and it is the discriminating one. A SHIPPED-SHAPE pack declaring
    // `appetite = { points = 4 }` and no envelope must yield NOTHING — not `N = 4`. An accessor
    // that read `appetite.points` would pass every assertion above (the envelope fixture declares
    // 50 there, and `attempt_cap()` would then be `Some(50)`, which the first assertion catches) —
    // but this case catches the inverse mistake too, where `points` is read only when `[retry]` is
    // absent.
    let points_only = pack("incident-appetite.toml", FIXTURE_POINTS_ONLY);
    assert_eq!(
        points_only.appetite(),
        Some(4),
        "the premise: it declares 4"
    );
    assert_eq!(
        points_only.attempt_cap(),
        None,
        "`appetite.points = 4` is not `N = 4`"
    );
    assert_eq!(
        points_only.token_appetite(),
        None,
        "`appetite.points` is not `appetite.tokens`: points are step 5's cost-class sum and tokens \
         are AC-020's second counter"
    );
    assert_eq!(points_only.breaker_threshold(), None);
}
