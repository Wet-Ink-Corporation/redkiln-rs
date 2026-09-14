//! **Step 3 — short-circuit** (design §4.4 row 3): per artifact, `Present` / `Stale` / `Missing`,
//! decided on `derived_from` digests.
//!
//! # What step 3 does NOT do, and where that was settled
//!
//! It does **not** read the event log, does **not** construct a `drift_advisory` and does **not**
//! append one. AC-525's 2026-09-13 amendment clause 1 is explicit: *"`select()` never reads the
//! event log — AC-041 makes the four digests its only inputs, and a `select()` that scanned the log
//! to de-duplicate would be exactly the hidden input AC-041, AC-042 and AC-043 exist as a chain to
//! prevent, while still passing all four of the fixtures named above."* Step 3 returns the
//! classification; clause 2 puts the derivation and the append **outside** the pure function, at the
//! kernel's step-3 boundary or in `reconcile()`, and clause 3 makes the *"exactly one"* run-scoped
//! with an **in-process** de-dup memory at the append site.

use rk_kernel::select::{ArtifactRegistry, Classification, Digest, Instance, digest_of, step3};

const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

fn registry() -> ArtifactRegistry {
    ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the fixture registry loads")
}

fn d(s: &str) -> Digest {
    digest_of(s.as_bytes())
}

/// **AC-066.** Absence is `Stale`, never "assumed fresh"; a `derived_from` digest that does not
/// match the current input is `Stale`, never `Present`. The eight cases are the register's own
/// `(exists × digests match × digest recorded)` table.
///
/// **REPORTED ARMED, NOT DISCHARGED.** §7b.9(6) says the discriminating fixtures — a human-sourced
/// kind carrying a declared input and a machine-sourced kind of the same shape — are **a register
/// edit and are OWED**. What is asserted here is the eight cases the register states *today*; the
/// two rows that tell the two readings apart are asserted separately, in
/// [`registered_source_breaks_the_ac066_ac067_tie`], against fixture kinds this crate mints.
#[test]
fn short_circuit_eight_case_table() {
    let reg = registry();
    // `DerivedBrief` is registered `source = "activity"`, so the vouched-root path is not open to
    // it and AC-066's rule applies unqualified.
    let kind = Instance::of("DerivedBrief");
    let declared_inputs = ["MarketScan"];

    for (exists, recorded, matches, expected) in [
        // exists, derived_from entry recorded for the declared input, and it matches
        (false, false, false, Classification::Missing),
        (false, false, true, Classification::Missing),
        (false, true, false, Classification::Missing),
        (false, true, true, Classification::Missing),
        // "an artifact with no recorded `derived_from` entry for a declared input is `Stale`, not
        // assumed fresh" -- both no-record rows, whatever the (vacuous) match column says.
        (true, false, false, Classification::Stale),
        (true, false, true, Classification::Stale),
        // recorded and mismatched
        (true, true, false, Classification::Stale),
        // recorded and matching: the only `Present` row for a machine-sourced kind
        (true, true, true, Classification::Present),
    ] {
        let mut state = rk_kernel::select::ArtifactState::new(&kind);
        if exists {
            state = state.exists(d("body"));
            if recorded {
                state = state.derived_from(
                    "MarketScan",
                    if matches { d("scan-v1") } else { d("scan-v0") },
                );
            }
        }
        let upstream = [("MarketScan", d("scan-v1"))];
        let got = step3(&reg, &state, &declared_inputs, &upstream);
        assert_eq!(
            got.classification, expected,
            "exists={exists} recorded={recorded} matches={matches}"
        );
    }
}

/// **AC-067.** A human-supplied artifact recorded as a vouched root stays `Present`, its producer is
/// not dispatched, and any drift signal is an **advisory that can never promote itself** to `Stale`
/// or to a blocking gate. The out-of-set fixture emits **nothing**.
#[test]
fn vouched_root_stays_present() {
    let reg = registry();
    let kind = Instance::of("VouchedBrief"); // registered source = "human"
    let state = rk_kernel::select::ArtifactState::new(&kind).exists(d("hand-written"));
    // A vouched root records `derived_from = []` -- no entries at all.
    assert!(state.derived_from_entries().is_empty());

    let in_set = [("MarketScan", d("scan-v2"))];
    let got = step3(&reg, &state, &["MarketScan"], &in_set);
    assert_eq!(
        got.classification,
        Classification::Present,
        "a vouched root stays Present when an in-set upstream moves; its producer is not dispatched"
    );
    assert!(
        got.drift.is_some(),
        "the in-set upstream changed, so step 3 reports the drift SIGNAL"
    );
    let drift = got.drift.as_ref().expect("checked");
    assert_eq!(drift.root, "VouchedBrief");
    assert_eq!(drift.upstream, "MarketScan");
    assert_eq!(drift.new_digest, d("scan-v2"));
    // Step 3 does NOT append, and the assertion is a TYPE fact rather than a value check:
    // `Step3Outcome`'s fields are `instance`, `classification` and `drift` - there is no
    // `advisory_appended`, no event, and no handle to a store, and `step3`'s own signature takes
    // none. AC-525's amendment clause 2 puts the derivation and the append outside the pure
    // function; what is returned here is the SIGNAL the boundary derives the event from.
    let _signal_not_event: &Option<rk_kernel::select::DriftSignal> = &got.drift;

    // AC-067's own negative companion: a same-stage artifact OUTSIDE the edge set changes and
    // NOTHING is emitted. The edge set is the registered producer's `requires` for the root's kind
    // (PIA-044), which here is `["MarketScan"]` alone.
    let out_of_set = [("SmokeResult", d("smoke-v9"))];
    let got = step3(&reg, &state, &["MarketScan"], &out_of_set);
    assert_eq!(got.classification, Classification::Present);
    assert!(
        got.drift.is_none(),
        "an upstream outside the edge set emits nothing at all"
    );

    // "No advisory can promote itself to `Stale` or to a blocking gate." There is no code path from
    // a drift signal to either, and the type is what says so: the classification is decided before
    // the drift is computed and the drift cannot write to it.
    assert_ne!(got.classification, Classification::Stale);
}

/// **§7b.9(6), resolved.** AC-066 and AC-067 classify one artifact two ways, and `derived_from = []`
/// is **not** the tie-breaker — the **registered kind's `source`** is.
///
/// The two fixtures below are the SAME SHAPE in every respect a classifier can see: each exists,
/// each carries a declared input (`MarketScan`), each records `derived_from = []`, and the upstream
/// has moved. They differ in exactly one bit — `artifacts.toml`'s `source`.
///
/// **This test fails under the other reading.** An implementer who applies AC-066's blanket absence
/// rule first classifies BOTH as `Stale`, which AC-067's own THEN forbids by name, re-dispatches a
/// producer against a human-authored artifact on every run, and suppresses the advisory path
/// entirely — making AC-525 dead code. Swap the two rules' order in `step3` and the first assertion
/// below goes red.
#[test]
fn registered_source_breaks_the_ac066_ac067_tie() {
    let reg = registry();
    let declared = ["MarketScan"];
    let upstream = [("MarketScan", d("scan-v2"))];

    let vouched = rk_kernel::select::ArtifactState::new(&Instance::of("VouchedBrief"))
        .exists(d("same-bytes"));
    let machine = rk_kernel::select::ArtifactState::new(&Instance::of("DerivedBrief"))
        .exists(d("same-bytes"));

    let a = step3(&reg, &vouched, &declared, &upstream);
    let b = step3(&reg, &machine, &declared, &upstream);

    assert_eq!(
        a.classification,
        Classification::Present,
        "registered source = \"human\" -> the vouched-root path (AC-067)"
    );
    assert_eq!(
        b.classification,
        Classification::Stale,
        "registered source = \"activity\" -> AC-066's absence rule"
    );
    assert!(a.drift.is_some(), "the vouched root gets an ADVISORY");
    assert!(
        b.drift.is_none(),
        "a machine-sourced Stale artifact is re-materialised; it is not advised about"
    );
}

/// **AC-525's amendment clause 1, and D4.** Step 3 is a function of its arguments. It takes no
/// store, no port and no log, and the type system is what says so: there is no parameter it could
/// read one through.
///
/// The discriminating instrument AC-525's amendment clause 4 adds — *"a replay over a run that
/// emitted an advisory must re-derive the same DAG with no store read"* — is asserted here in the
/// only form available inside a pure function: the same inputs produce the same output on the
/// hundredth call as on the first, with no accumulating state anywhere. A de-duplicating
/// implementation that remembered a previous advisory would go red on the second iteration.
#[test]
fn step_three_returns_a_classification_and_reads_no_log() {
    let reg = registry();
    let state = rk_kernel::select::ArtifactState::new(&Instance::of("VouchedBrief"))
        .exists(d("hand-written"));
    let upstream = [("MarketScan", d("scan-v2"))];

    let first = step3(&reg, &state, &["MarketScan"], &upstream);
    for i in 1..100 {
        let again = step3(&reg, &state, &["MarketScan"], &upstream);
        assert_eq!(
            again, first,
            "call {i} differed from call 0: step 3 is carrying state between calls, which is the \
             log scan AC-041 forbids wearing a different hat"
        );
        assert!(
            again.drift.is_some(),
            "call {i}: the classification must not learn that it has already reported this drift \
             -- the `exactly one` is the APPEND SITE's, run-scoped and in-process (AC-525 clause 3)"
        );
    }
}
