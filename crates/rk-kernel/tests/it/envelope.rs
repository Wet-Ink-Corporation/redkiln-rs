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
// *(dated 2026-10-02, s8h build, package P5-kernel-envelope; s8h §8h.9; Record B = ADR-0198
// decision 4, proposed.)* The per-item envelope: the dispatch cap and the token appetite, counted
// from the store before each dispatch.
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_kernel::runtime::{Runtime, RuntimeError};
use rk_kernel::workflow::envelope::{DispatchCap, EnvelopeRefusal, ItemEnvelope};
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_ports::harness::Harness;
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::runtime::{AID, CountingHarness, CountingStore, ITEM, STAGE, STEP, ok_typed, spec};

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

// ---- s8h §8h.9 — THE PER-ITEM ENVELOPE (Record B = ADR-0198 decision 4, proposed) ------------------
//
// *(dated 2026-10-02, s8h build, package P5-kernel-envelope.)* D21-42 19a: *"The kernel enforces a
// per-item dispatch-count cap and token appetite, refusing with a typed class."* D21-43 N17: Q2 is
// the sum of `Usage`'s four counters; Q3 is `Exit::Overage` (20), which is `rk-cli`'s
// `exit::an_envelope_refusal_leaves_through_overage`. Every count below is read off the STORE (the
// stage-open reading §8h.0 item 4: the `dispatch` rows and the `dispatch_completed` rows keyed to
// the item through `query(ByItem)`), never off a counter this process holds.

/// The pack that declares the per-item envelope: dispatch cap 3, token appetite 5000, `N` = 4,
/// `K` = 2, points 50 — five different numbers.
const FIXTURE_ITEM_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-item-envelope.toml");

/// The fixture's dispatch cap and token appetite, written out so a test that read the wrong key
/// lands on a number named here.
const CAP: u32 = 3;
const APPETITE: u64 = 5000;

/// The item no row of the control set belongs to.
const OTHER_ITEM: &str = "INC-2";

/// The workspace root, for the two installed packs.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

/// The envelope the fixture declares, through the pack-validation call.
fn item_envelope() -> ItemEnvelope {
    pack("runtime-item-envelope.toml", FIXTURE_ITEM_ENVELOPE)
        .require_item_envelope()
        .expect("the fixture declares both `appetite.dispatches` and `appetite.tokens`")
}

/// A `Runtime` over `store`, keyed to [`ITEM`] (ADR-0182's interim key, which `resume_routed` binds
/// beside the envelope), the envelope bound, dispatching to `harness`.
fn enveloped(store: &Arc<Mutex<dyn EventStore>>, harness: &Arc<CountingHarness>) -> Runtime {
    Runtime::new(
        crate::runtime::ports(Arc::clone(harness) as Arc<dyn Harness>, store),
        "run-envelope",
        STAGE,
    )
    .with_item(ITEM)
    .with_envelope(item_envelope())
}

/// A counting in-memory store (it returns every row whatever the selector, so the counter itself
/// must filter by item).
fn counting_store() -> Arc<Mutex<dyn EventStore>> {
    let queries = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let appends = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    Arc::new(Mutex::new(CountingStore::new(&queries, &appends)))
}

/// A harness scripted with `n` well-formed results; it appends nothing.
fn harness(n: usize) -> Arc<CountingHarness> {
    Arc::new(CountingHarness::scripted(
        (0..n)
            .map(|i| ok_typed(serde_json::json!({ "n": i })))
            .collect(),
    ))
}

/// Every row the store holds.
fn rows(store: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    store
        .lock()
        .expect("the store lock is not poisoned")
        .query(Query::ByRun("any".to_owned()))
        .expect("the store reads")
}

/// The `dispatch` rows keyed to `item`.
fn keyed_dispatches(store: &Arc<Mutex<dyn EventStore>>, item: &str) -> usize {
    rows(store)
        .iter()
        .filter(|e| e.kind == EventKind::Dispatch && e.item() == Some(item))
        .count()
}

/// Append one row through the store's own port, as a harness or another process would.
fn seed(store: &Arc<Mutex<dyn EventStore>>, event: Event) {
    store
        .lock()
        .expect("the store lock is not poisoned")
        .append(event)
        .expect("the seed row appends");
}

/// A `dispatch_completed` row carrying `usage` (or `null`), keyed to `item` when `Some`.
fn completed(item: Option<&str>, usage: &serde_json::Value) -> Event {
    let row = Event::new(
        EventKind::DispatchCompleted,
        serde_json::json!({ "usage": usage }),
    );
    match item {
        Some(item) => row.with_item(item),
        None => row,
    }
}

/// One dispatch of the fixture activation at `attempt`.
fn dispatch(rt: &Runtime, attempt: u32) -> Result<bool, RuntimeError> {
    rt.dispatch_once(ITEM, AID, STEP, attempt, &spec())
        .map(|d| d.dispatched())
}

/// **The dispatch cap is declared, never defaulted** (Record B decision 4: *"An absent
/// `appetite.dispatches` is refused by pack validation as an `EnvelopeError` beside
/// `TokenAppetiteNotDeclared`, naming the pack; no code default exists for either key"*), and the
/// two installed packs declare Record B decision 2's values.
#[test]
fn dispatch_cap_is_declared_never_defaulted() {
    let declared = pack("runtime-item-envelope.toml", FIXTURE_ITEM_ENVELOPE);

    // THE PREMISE: five different numbers, so an accessor that read the wrong key is caught.
    assert_eq!(declared.dispatch_cap(), Some(DispatchCap::declared(CAP)));
    assert_eq!(declared.attempt_cap(), Some(AttemptCap::declared(4)));
    assert_eq!(
        declared.breaker_threshold(),
        Some(BreakerThreshold::declared(2))
    );
    assert_eq!(declared.token_appetite(), Some(Tokens::new(APPETITE)));
    assert_eq!(declared.appetite(), Some(50));

    // ---- the pack-validation call returns the DECLARED values ---------------------------------
    assert_eq!(
        declared
            .require_dispatch_cap()
            .expect("the fixture declares `appetite.dispatches`")
            .get(),
        CAP
    );
    let envelope = declared
        .require_item_envelope()
        .expect("the fixture declares both keys");
    assert_eq!(envelope.cap(), DispatchCap::declared(CAP));
    assert_eq!(envelope.appetite(), Tokens::new(APPETITE));

    // ---- a pack that declares no `appetite.dispatches` REFUSES, naming the pack ----------------
    // `runtime-envelope.toml` declares `appetite.tokens` and no `dispatches`: the two absences are
    // independent, and the token appetite it DOES declare is not read as a cap.
    let absent = pack("runtime-envelope.toml", FIXTURE_ENVELOPE);
    assert_eq!(
        absent.dispatch_cap(),
        None,
        "an absent declaration is an absence, never a number"
    );
    assert_eq!(
        absent.token_appetite(),
        Some(Tokens::new(50_000)),
        "the premise: this pack declares a token appetite, so only the cap is absent"
    );
    let err = absent
        .require_dispatch_cap()
        .expect_err("a pack declaring no `appetite.dispatches` cannot drive a dispatch");
    assert_eq!(
        err,
        EnvelopeError::DispatchCapNotDeclared {
            pack: "runtime-envelope-fixture".to_owned()
        },
        "the refusal names the pack"
    );
    assert_eq!(err.as_str(), "dispatch_cap_not_declared");
    assert_eq!(
        absent
            .require_item_envelope()
            .expect_err("no cap, no envelope"),
        err,
        "the envelope's pack-validation call refuses with the cap's own refusal"
    );
    // The absent pack still LOADS: the refusal is the validation call's, never the loader's.
    assert_eq!(absent.id, "runtime-envelope-fixture");

    // A shipped-shape `appetite = { points = 4 }` declares no cap: `points` is not `dispatches`.
    let points_only = pack("incident-appetite.toml", FIXTURE_POINTS_ONLY);
    assert_eq!(
        points_only.appetite(),
        Some(4),
        "the premise: it declares 4"
    );
    assert_eq!(
        points_only.dispatch_cap(),
        None,
        "`points = 4` is not a cap of 4"
    );

    // ---- the two installed packs declare Record B decision 2's values ---------------------------
    for (file, points) in [("bug-standard.toml", 40), ("story.toml", 60)] {
        let path = workspace_root().join(".redkiln").join("packs").join(file);
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        let shipped = pack(file, &src);
        assert_eq!(
            shipped.dispatch_cap(),
            Some(DispatchCap::declared(40)),
            "{file}: `appetite.dispatches` is Record B's 40"
        );
        assert_eq!(
            shipped.token_appetite(),
            Some(Tokens::new(24_000_000)),
            "{file}: `appetite.tokens` is Record B's 24000000"
        );
        assert_eq!(
            shipped.appetite(),
            Some(points),
            "{file}: `appetite.points` is unchanged"
        );
        assert!(
            shipped.require_item_envelope().is_ok(),
            "{file}: the installed pack passes the envelope's pack validation"
        );
    }
}

/// **The dispatch at the cap proceeds.** With a cap of 3, the third dispatch (two prior `dispatch`
/// rows keyed to the item) reaches the port.
#[test]
fn the_dispatch_at_the_cap_proceeds() {
    let store = counting_store();
    let h = harness(3);
    let rt = enveloped(&store, &h);
    for attempt in 1..=CAP {
        assert_eq!(
            keyed_dispatches(&store, ITEM),
            usize::try_from(attempt - 1).expect("small"),
            "the premise: {} prior dispatches before attempt {attempt}",
            attempt - 1
        );
        assert_eq!(
            dispatch(&rt, attempt),
            Ok(true),
            "dispatch {attempt} of a cap of {CAP} must proceed"
        );
    }
    assert_eq!(h.dispatches(), CAP, "the port was reached {CAP} times");
    assert_eq!(keyed_dispatches(&store, ITEM), 3);
}

/// **The dispatch past the cap is refused with its class**, before the port, writing no row; and
/// only rows KEYED to the item count — an unkeyed `dispatch` row and one keyed to another item are
/// the controls (Record B decision 4: *"only rows keyed to the item are counted"*).
#[test]
fn the_dispatch_past_the_cap_is_refused_with_its_class() {
    let store = counting_store();
    // The controls, seeded first: neither is this item's dispatch.
    seed(
        &store,
        Event::new(
            EventKind::Dispatch,
            serde_json::json!({ "item": ITEM, "activity_id_at_version": "other@1", "attempt": 1, "step_id": "x" }),
        ),
    );
    seed(
        &store,
        Event::new(
            EventKind::Dispatch,
            serde_json::json!({ "item": OTHER_ITEM, "activity_id_at_version": "other@1", "attempt": 1, "step_id": "x" }),
        )
        .with_item(OTHER_ITEM),
    );
    let h = harness(4);
    let rt = enveloped(&store, &h);
    for attempt in 1..=CAP {
        assert_eq!(
            dispatch(&rt, attempt),
            Ok(true),
            "dispatch {attempt}: the two control rows are not this item's keyed dispatches"
        );
    }
    let before = rows(&store).len();
    let refused = dispatch(&rt, CAP + 1);
    assert_eq!(
        refused,
        Err(RuntimeError::Envelope(
            EnvelopeRefusal::DispatchCapReached {
                item: ITEM.to_owned(),
                count: CAP,
                cap: DispatchCap::declared(CAP),
            }
        )),
        "the fourth dispatch of a cap of {CAP} is refused with the cap's class"
    );
    let Err(RuntimeError::Envelope(refusal)) = &refused else {
        unreachable!("asserted above")
    };
    assert_eq!(refusal.as_str(), "dispatch_cap_reached");
    assert_eq!(
        refused.as_ref().map_err(RuntimeError::as_str),
        Err("envelope")
    );
    let shown = refusal.to_string();
    assert!(
        shown.contains(ITEM) && shown.contains("appetite.dispatches"),
        "the refusal names the item and the key: {shown}"
    );
    assert_eq!(h.dispatches(), CAP, "refused BEFORE the port: no spawn");
    assert_eq!(
        rows(&store).len(),
        before,
        "the refusal writes no row: no `dispatch` cost row and no tier-1 row"
    );
}

/// **The dispatch count survives a second process.** Two runtimes over one durable store, each
/// with its OWN handle on the file, so nothing the first held in memory reaches the second: the
/// first makes two dispatches and is dropped; the second makes the third (at the cap) and is
/// refused the fourth. A per-runtime or per-handle counter would let the second make three.
#[test]
fn the_dispatch_count_survives_a_second_process() {
    let path = fresh_file("survives-a-second-process");
    {
        let first: Arc<Mutex<dyn EventStore>> =
            Arc::new(Mutex::new(KeyedFileStore::at(path.clone())));
        let h = harness(2);
        let rt = enveloped(&first, &h);
        assert_eq!(dispatch(&rt, 1), Ok(true));
        assert_eq!(dispatch(&rt, 2), Ok(true));
        assert_eq!(h.dispatches(), 2);
    }
    let second: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(KeyedFileStore::at(path)));
    assert_eq!(
        keyed_dispatches(&second, ITEM),
        2,
        "the premise: the first runtime's two keyed `dispatch` rows are on disk"
    );
    let h = harness(2);
    let rt = enveloped(&second, &h);
    assert_eq!(
        dispatch(&rt, 3),
        Ok(true),
        "the third dispatch, the one at the cap, proceeds in the second runtime"
    );
    assert_eq!(
        dispatch(&rt, 4),
        Err(RuntimeError::Envelope(
            EnvelopeRefusal::DispatchCapReached {
                item: ITEM.to_owned(),
                count: CAP,
                cap: DispatchCap::declared(CAP),
            }
        )),
        "the second runtime counts the first's dispatches from the store"
    );
    assert_eq!(h.dispatches(), 1, "the second runtime spawned once");
}

/// **The token appetite refuses the next dispatch once the item's charged sum has reached it.** The
/// sum is `Usage`'s four counters (N17 Q2), cache reads included: 1000 + 1000 + 1000 + 1999 = 4999
/// proceeds, and one more token reaches 5000. With cache reads excluded the sum would be 3001 and
/// the dispatch would proceed, so the cache-read counter is what decides it. Rows keyed to another
/// item, and an unkeyed row, are not charged.
#[test]
fn token_appetite_past_the_appetite_refuses_the_next_dispatch() {
    let store = counting_store();
    seed(
        &store,
        completed(
            Some(ITEM),
            &serde_json::json!({
                "input_tokens": 1000,
                "output_tokens": 1000,
                "cache_creation_input_tokens": 1000,
                "cache_read_input_tokens": 1999,
            }),
        ),
    );
    // The controls: far past the appetite, and neither is this item's.
    let huge = serde_json::json!({ "input_tokens": 9_000_000 });
    seed(&store, completed(Some(OTHER_ITEM), &huge));
    seed(&store, completed(None, &huge));
    let h = harness(2);
    let rt = enveloped(&store, &h);
    assert_eq!(
        dispatch(&rt, 1),
        Ok(true),
        "4999 charged tokens of an appetite of {APPETITE} proceed"
    );
    seed(
        &store,
        completed(Some(ITEM), &serde_json::json!({ "output_tokens": 1 })),
    );
    let before = rows(&store).len();
    assert_eq!(
        dispatch(&rt, 2),
        Err(RuntimeError::Envelope(
            EnvelopeRefusal::TokenAppetiteReached {
                item: ITEM.to_owned(),
                sum: Tokens::new(APPETITE),
                appetite: Tokens::new(APPETITE),
            }
        )),
        "5000 charged tokens reach the appetite, and the next dispatch is refused"
    );
    assert_eq!(h.dispatches(), 1, "refused before the port");
    assert_eq!(rows(&store).len(), before, "the refusal writes no row");
    assert!(
        keyed_dispatches(&store, ITEM) < usize::try_from(CAP).expect("small"),
        "the premise: the cap is not what refused it"
    );
}

/// **A dispatch with no `usage` still counts against the cap** (Record B decision 4: *"which is why
/// the cap exists beside the appetite"*). Three dispatches whose `dispatch_completed` rows carry
/// `usage: null` charge no token, and the fourth is refused by the CAP.
#[test]
fn a_dispatch_with_no_usage_still_counts_against_the_cap() {
    let store = counting_store();
    let h = harness(4);
    let rt = enveloped(&store, &h);
    for attempt in 1..=CAP {
        assert_eq!(dispatch(&rt, attempt), Ok(true));
        seed(&store, completed(Some(ITEM), &serde_json::Value::Null));
    }
    assert_eq!(
        ItemEnvelope::charged_tokens(&rows(&store), ITEM),
        Tokens::new(0),
        "the premise: no row carried usage, so nothing was charged"
    );
    assert_eq!(
        dispatch(&rt, CAP + 1),
        Err(RuntimeError::Envelope(
            EnvelopeRefusal::DispatchCapReached {
                item: ITEM.to_owned(),
                count: CAP,
                cap: DispatchCap::declared(CAP),
            }
        )),
        "three usage-less dispatches reach a cap of three"
    );
    assert_eq!(h.dispatches(), CAP);
}

// ---- the durable medium for the second-process test ---------------------------------------------

/// The file a case writes, under cargo's per-target temp dir, emptied first so a journal left by an
/// earlier run cannot satisfy a readback.
fn fresh_file(case: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("s8h-envelope")
        .join(case);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("the case directory is removable");
    }
    std::fs::create_dir_all(&dir).expect("the case directory is creatable");
    dir.join("events.jsonl")
}

/// A file-backed [`EventStore`] that keeps ADR-0182's interim `item` key beside each row.
/// `cross_process.rs`'s `FileStore` writes `{kind, payload}` only, so a row's key would not survive
/// it; the per-item count reads exactly that key. One JSON line per row, appended and synced on
/// every call; `query` reads the file back on every call, so nothing is cached per handle.
struct KeyedFileStore {
    path: PathBuf,
}

impl KeyedFileStore {
    const fn at(path: PathBuf) -> Self {
        Self { path }
    }

    fn io(detail: &str) -> StepError {
        StepError::Transport {
            kind: TransportKind::Io,
            detail: detail.to_owned(),
        }
    }
}

impl EventStore for KeyedFileStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        let line = serde_json::json!({
            "kind": event.kind.wire_name(),
            "payload": event.payload,
            "item": event.item(),
        })
        .to_string();
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| Self::io(&format!("open {}: {e}", self.path.display())))?;
        f.write_all(line.as_bytes())
            .and_then(|()| f.write_all(b"\n"))
            .and_then(|()| f.sync_all())
            .map_err(|e| Self::io(&format!("append to {}: {e}", self.path.display())))
    }

    fn append_tier1(
        &mut self,
        event: Event,
        _anchor: &rk_ports::event_store::TierOneAnchor,
    ) -> Result<(), StepError> {
        self.append(event)
    }

    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        let raw = match std::fs::read_to_string(&self.path) {
            Ok(raw) => raw,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(Self::io(&format!("read {}: {e}", self.path.display()))),
        };
        let mut events = Vec::new();
        for (i, line) in raw.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            let value: serde_json::Value = serde_json::from_str(line)
                .map_err(|e| Self::io(&format!("line {}: {e}", i + 1)))?;
            let wire = value["kind"]
                .as_str()
                .ok_or_else(|| Self::io(&format!("line {}: no kind", i + 1)))?;
            let kind = EventKind::ALL
                .into_iter()
                .find(|k| k.wire_name() == wire)
                .ok_or_else(|| Self::io(&format!("line {}: {wire:?} is no kind", i + 1)))?;
            let event = Event::new(kind, value["payload"].clone());
            events.push(match value["item"].as_str() {
                Some(item) => event.with_item(item),
                None => event,
            });
        }
        Ok(events)
    }

    fn snapshot(&self) -> Result<Snapshot, StepError> {
        Err(StepError::Unsupported {
            capability: "snapshot".to_owned(),
        })
    }

    fn rebuild(&mut self, _snapshot: &Snapshot) -> Result<(), StepError> {
        Err(StepError::Unsupported {
            capability: "rebuild".to_owned(),
        })
    }
}
