//! **AC-494** — the resolver, its three named instruments, and the fake adapter whose registry omits
//! the capability.
//!
//! > **AC-494** — GIVEN an activity that needs a capability the selected adapter's registry does not
//! > claim / WHEN the kernel resolves the activity / THEN the activity fails with
//! > `StepError::Unsupported(capability)` carrying the capability token / AND the failure is not a
//! > generic error, not a retry, and never a `pass`.
//! > *Verified by:* **a unit test on the resolver with a fake adapter whose registry omits the
//! > capability; an exhaustiveness test that `StepError::Unsupported` carries the token; a negative
//! > test asserting the retry policy does not fire on `Unsupported`.*
//!
//! **Three instruments, three tests.** The register names three and a build that did one of them
//! would have satisfied a third of a criterion:
//!
//! | instrument | test |
//! | --- | --- |
//! | *"a unit test on the resolver with a fake adapter whose registry omits the capability"* | [`ac494_resolver`] |
//! | *"an exhaustiveness test that `StepError::Unsupported` carries the token"* | [`ac494_unsupported_carries_the_token_and_no_other_variant_does`] |
//! | *"a negative test asserting the retry policy does not fire on `Unsupported`"* | [`ac494_the_shipped_retry_policy_does_not_fire_on_unsupported`] |
//!
//! The third is measured against the **shipped** classifier — `rk_types::error::StepError::
//! retry_class` and `rk_kernel::retry::decide`, both of them checkpoint-passed code this stage did
//! not write — rather than against a second table declared here. A negative test over a copy of the
//! policy would pass while the real one retried.
//!
//! # 🛑 WHAT IS MEASURED AND WHAT IS NOT
//!
//! | clause | state | why |
//! | --- | --- | --- |
//! | *"carrying the capability token"* | **measured** — byte equality against the need's own token, and with the unclaimed need placed **second** in a two-need list, so a resolver that always names `needs[0]` is red | |
//! | *"not a generic error"* | **measured** — an exhaustive, wildcard-free `match` over ADR-0112's eleven, in which exactly one variant yields a token | a twelfth variant makes this file fail to compile, which is what *"exhaustiveness"* means here |
//! | *"not a retry"* | **measured** — over the resolver's **own** error, at every attempt from 1 to the declared cap + 2, against the shipped policy, with a `Transport` control that does retry | |
//! | *"never a `pass`"* | **measured** — `dispatch_if_resolved` is handed the fake adapter's own `dispatch`, and the adapter's counter reads **0** | the ordering is the kernel's; no `match` in this file decides whether to dispatch |
//! | *"WHEN the kernel **resolves the activity**"*, as an ordering inside a live `Runtime::step` | **ARMED, NOT DISCHARGED** | nothing in this tree says which capability an activity needs: `ActivitySpec` has *"five fields and no sixth"* (ADR-0099:61) (*(dated 2026-09-29, s8f build, package R3FIX-8f, for S8F-P7; ADR-0190 decision 2, proposed; D21-107 ruling 3; refuting round 3.)* now six: ADR-0190's `activity_id_at_version`, which is not a capability), the catalog schema declares no `capability` key, and `rk-types` is Checkpoint 5's. The channel is a decision `00-preamble.md`:55 reserves and **a record is OWED**; `src/resolver.rs`'s header carries it. The register's own instrument is *"a unit test on **the resolver**"*, and that is what is built |
//! | AC-489 / AC-490 / AC-492 — registry schema, staleness, vocabulary | **not claimed, and not built** | each is an adapter-side criterion; `src/resolver.rs`'s table names them by id |

use std::sync::atomic::{AtomicU32, Ordering};

use rk_kernel::pack_validation::pack_capability_routing_rule;
use rk_kernel::resolver::{
    CapabilityClaim, CapabilityNeed, ClaimError, ClaimedCapabilities, dispatch_if_resolved,
    first_unclaimed, resolve_activity, unsupported,
};
use rk_kernel::retry::{RetryDecision, decide, error_tag, refused_before_spawn};
use rk_kernel::select::Digest;
use rk_kernel::workflow::envelope::AttemptCap;
use rk_ports::event_store::EventStore;
use rk_ports::fakes::VecStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_store::memo::IdempotencyKey;
use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::error::{CancelledBy, IsolationSurface, RetryClass, StepError, TransportKind};
use rk_types::retention::Retained;
use rk_types::time::Timestamp;
use rk_types::typed::Typed;
use rk_types::usage::{CliVersion, Timings};

use crate::runtime::{AID, ITEM, spec};

/// The token design:1031 names, and the one this adapter does **not** claim.
const NEEDED: &str = "affected-selection";

/// The token design:1031 says the pack routes to instead, and the one it **does** claim.
const CLAIMED: &str = "whole-suite";

/// The pack's `N`. Supplied, never defaulted (AC-029).
const CAP: u32 = 4;

// ---------------------------------------------------------------------------------------------
// THE FAKE ADAPTER. It owns a registry and it counts its dispatches, and the kernel is handed the
// REGISTRY — never the adapter. That asymmetry is the whole of AC-487's consistency with AC-494:
// `resolve_activity` cannot ask what this type is, because no value of this type is in its scope.
// ---------------------------------------------------------------------------------------------

/// A harness fake carrying its own capability registry.
///
/// `rk_ports` ships five fakes and none of them holds a registry, because **no capability registry
/// exists in the workspace** — `src/resolver.rs`'s header carries that measurement. So the registry
/// is this fixture's, exactly as the register describes it: *"a fake adapter whose registry omits
/// the capability"*.
struct FakeAdapter {
    registry: ClaimedCapabilities,
    dispatches: AtomicU32,
}

impl FakeAdapter {
    fn new(registry: ClaimedCapabilities) -> Self {
        Self {
            registry,
            dispatches: AtomicU32::new(0),
        }
    }

    /// What the adapter claims, **as data**. This is what crosses into the kernel.
    fn registry(&self) -> &ClaimedCapabilities {
        &self.registry
    }

    /// How many times the port was reached. **AC-494's *"never a `pass`"* is read off this.**
    fn dispatches(&self) -> u32 {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl Harness for FakeAdapter {
    fn dispatch(&self, spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        DispatchOutcome {
            record: DispatchRecord {
                argv_digest: Digest::from_hex(&"a1".repeat(32)).expect("64 hex characters"),
                env_digest: Digest::from_hex(&"e2".repeat(32)).expect("64 hex characters"),
                instrument_version: "ac494-fake".to_owned(),
                stdout: Retained::of(b""),
                stderr: Retained::of(b""),
                // ADR-0126's members (Step 7c Slice 1), in the pre-spawn shape: this fake spawns
                // nothing and reads no `system/init`, so the version is `Pinned` and every
                // usage-adjacent member is `None`, never `Some(0)` (AC-362).
                usage: None,
                timings: Timings {
                    wall_ms: 0,
                    api_ms: None,
                },
                total_cost_usd: None,
                headroom_before: None,
                headroom_after: None,
                cli_version: CliVersion::Pinned("ac494-fake".to_owned()),
                api_retry_events: 0,
                prompt_cache_ttl: "1h".to_owned(),
                subagent_prompt_cache_ttl: "1h".to_owned(),
                cancelled_by: None,
                detected_by: None,
                activity_id_at_version: None,
            },
            result: Ok(Typed::new(
                spec.schema.clone(),
                serde_json::json!({"ran": true}),
            )),
        }
    }
}

/// A registry that claims one token and omits the one the activity needs.
///
/// **It is not empty**, and that matters: a resolver that refused everything would pass against an
/// empty registry. The set here holds a real claim, so the refusal below is a refusal of *this
/// token* rather than of *all tokens*.
fn registry_omitting_the_capability() -> ClaimedCapabilities {
    ClaimedCapabilities::from_claims(vec![
        CapabilityClaim::new(CLAIMED, "2026-09-14", "toolchain:whole-suite@1").expect("a row"),
        CapabilityClaim::new("json-schema", "2026-09-14", "harness:schema-probe@3").expect("a row"),
    ])
    .expect("two distinct tokens")
}

/// The same registry, plus the token the activity needs.
fn registry_claiming_the_capability() -> ClaimedCapabilities {
    ClaimedCapabilities::from_claims(vec![
        CapabilityClaim::new(CLAIMED, "2026-09-14", "toolchain:whole-suite@1").expect("a row"),
        CapabilityClaim::new("json-schema", "2026-09-14", "harness:schema-probe@3").expect("a row"),
        CapabilityClaim::new(NEEDED, "2026-09-14", "toolchain:affected@2").expect("a row"),
    ])
    .expect("three distinct tokens")
}

/// One need, from the activity the fixture pack's stage would select.
fn needs() -> Vec<CapabilityNeed> {
    vec![CapabilityNeed::new(AID, "select-tests", NEEDED)]
}

/// ADR-0112's eleven, one of each, distinct in every field that has one.
fn every_step_error() -> Vec<StepError> {
    vec![
        StepError::Transport {
            kind: TransportKind::Spawn,
            detail: "no such executable".to_owned(),
        },
        StepError::Schema {
            attempts: 2,
            last_error: "expected object".to_owned(),
        },
        StepError::Timeout { after_secs: 900 },
        StepError::Cancelled {
            by: CancelledBy::Timeout,
        },
        StepError::NonZero {
            code: Some(1),
            stderr_digest: Digest::from_hex(&"3c".repeat(32)).expect("64 hex characters"),
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
            diff_digest: Digest::from_hex(&"4d".repeat(32)).expect("64 hex characters"),
        },
        StepError::Platform {
            status: 529,
            retry_after: Some(30),
        },
        StepError::Unsupported {
            capability: NEEDED.to_owned(),
        },
        StepError::Overage {
            reason: Some("credit balance too low".to_owned()),
        },
    ]
}

/// **The token an error carries, by an exhaustive `match` with NO wildcard arm.**
///
/// This is the exhaustiveness instrument's mechanism and it is a function rather than an inline
/// `match` so its shape is stated once: a twelfth `StepError` variant is `E0004` **here**, in a test
/// file, which is the only place a test can observe exhaustiveness at all.
///
/// `clippy::match_same_arms` is denied workspace-wide, so the ten that carry no capability are one
/// or-pattern arm — the same shape `rk_types::error::StepError::spoke` is forced into, and for the
/// same two reasons: an arm per variant trips the lint, and `matches!` expands to an implicit
/// `_ => false` under which a twelfth variant would compile silently.
fn capability_of(error: &StepError) -> Option<&str> {
    match error {
        StepError::Unsupported { capability } => Some(capability),
        StepError::Transport { .. }
        | StepError::Schema { .. }
        | StepError::Timeout { .. }
        | StepError::Cancelled { .. }
        | StepError::NonZero { .. }
        | StepError::RateLimited { .. }
        | StepError::NotAdopted { .. }
        | StepError::NotIsolated { .. }
        | StepError::Platform { .. }
        | StepError::Overage { .. } => None,
    }
}

fn key_at(attempt: u32) -> IdempotencyKey {
    IdempotencyKey {
        item: ITEM.to_owned(),
        activity_id_at_version: AID.to_owned(),
        attempt,
    }
}

// ---- instrument 1 — the resolver, against a registry that omits the capability ------------------

/// **AC-494, instrument 1.** *"A unit test on the resolver with a fake adapter whose registry omits
/// the capability."*
///
/// **What would make this vacuous, and what stops each:**
///
/// - *A resolver that refuses everything* passes the refusal. **Control 1** puts the same needs to a
///   registry that claims the token and asserts a clean resolution **and** a dispatch.
/// - *A registry that claims nothing* makes the refusal true for the wrong reason. The registry here
///   holds two real claims and omits one token; the premise asserts both halves.
/// - *A resolver that always names `needs[0]`* passes a one-need test. **Control 3** puts the
///   unclaimed need **second**.
/// - *A lookup that normalises the token* would admit `Affected_Selection`. **Control 2** puts four
///   near-miss spellings and asserts every one of them is refused, naming the spelling that was
///   asked for.
/// - *A `dispatch_if_resolved` that dispatches first and checks after* passes every assertion about
///   the error. The adapter's own counter is what catches it, and it reads **0**.
#[test]
fn ac494_resolver() {
    let adapter = FakeAdapter::new(registry_omitting_the_capability());
    let needs = needs();

    // PREMISE — the registry OMITS the needed token and CLAIMS another. Both halves, because a
    // refusal against an empty registry is not this criterion's GIVEN.
    assert!(
        !adapter.registry().claims_token(NEEDED),
        "the fixture registry must omit {NEEDED}, or there is nothing to refuse"
    );
    assert!(
        adapter.registry().claims_token(CLAIMED),
        "the fixture registry must claim something, or the refusal below is a refusal of everything"
    );
    assert_eq!(adapter.registry().len(), 2, "two rows, one of them omitted");

    // THE CRITERION'S THEN — `StepError::Unsupported(capability)` CARRYING THE TOKEN.
    let err = resolve_activity(&needs, adapter.registry())
        .expect_err("AC-494: a need the registry does not claim must not resolve");
    assert_eq!(
        capability_of(&err),
        Some(NEEDED),
        "AC-494: the failure carries the capability TOKEN, verbatim — got {err:?}"
    );
    // "NOT A GENERIC ERROR": the whole value, not its tag. A refusal that carried a sentence
    // mentioning the token would pass a `contains` and fails this.
    assert_eq!(
        err,
        StepError::Unsupported {
            capability: NEEDED.to_owned()
        },
        "AC-494: the token is the field, never a message about the field"
    );
    assert_eq!(error_tag(&err), "unsupported", "the stable tag");
    assert_eq!(
        err,
        unsupported(NEEDED),
        "the one mint site in the crate produces the same value"
    );

    // The site is recoverable without putting it into the error, so a caller can say WHERE.
    let site = first_unclaimed(&needs, adapter.registry()).expect("the unclaimed need");
    assert_eq!(site.capability(), NEEDED);
    assert_eq!(site.activity(), AID);
    assert_eq!(site.step(), "select-tests");

    // "NEVER A `pass`" — the kernel owns the ordering, and the adapter's counter reads it.
    let mut store = VecStore::default();
    let refused = dispatch_if_resolved(&needs, adapter.registry(), || {
        adapter.dispatch(&spec(), &mut store)
    })
    .expect_err("AC-494: an unresolved need must not reach the port");
    assert_eq!(
        capability_of(&refused),
        Some(NEEDED),
        "the same refusal, carrying the same token"
    );
    assert_eq!(
        adapter.dispatches(),
        0,
        "AC-494: never a `pass` — the port is not reached at all when the capability is unclaimed"
    );
    assert!(
        store.events().is_empty(),
        "and nothing is journaled by a dispatch that did not happen"
    );

    // ---- CONTROL 1 — THE DISCRIMINATOR. Without it every assertion above is satisfied by a
    // resolver that refuses everything, and by a `dispatch_if_resolved` that never dispatches.
    let routed = FakeAdapter::new(registry_claiming_the_capability());
    let report = resolve_activity(&needs, routed.registry())
        .expect("a need this registry claims must resolve");
    assert_eq!(report.needs_checked(), 1, "one need was put to the set");
    assert_eq!(report.claims_walked(), 3, "against a three-row registry");
    assert_eq!(
        report.resolved(),
        [NEEDED.to_owned()],
        "and the token that resolved is named"
    );
    let mut store = VecStore::default();
    let outcome = dispatch_if_resolved(&needs, routed.registry(), || {
        routed.dispatch(&spec(), &mut store)
    })
    .expect("a resolved need reaches the port");
    assert!(outcome.result.is_ok(), "and the fake answers");
    assert_eq!(
        routed.dispatches(),
        1,
        "the control must DISPATCH, or the zero above measures a function that never calls anything"
    );

    // ---- CONTROL 2 — NEAR MISSES. A lookup that folded case, trimmed, or matched a prefix would
    // claim a token the registry never carried, which is the *"never assumed"* direction (design:363).
    for spelling in [
        "Affected-Selection",
        "affected_selection",
        " affected-selection",
        "affected-selection ",
        "affected",
    ] {
        let near = vec![CapabilityNeed::new(AID, "select-tests", spelling)];
        let err = resolve_activity(&near, routed.registry()).unwrap_err();
        assert_eq!(
            capability_of(&err),
            Some(spelling),
            "{spelling:?} is not {NEEDED:?}: a registry claiming the second must refuse the first, \
             naming the spelling that was asked for"
        );
    }

    // ---- CONTROL 3 — THE UNCLAIMED NEED IS SECOND. A resolver that always names the first need
    // passes every assertion above.
    let two = vec![
        CapabilityNeed::new(AID, "prepare", CLAIMED),
        CapabilityNeed::new(AID, "select-tests", NEEDED),
    ];
    let err = resolve_activity(&two, adapter.registry())
        .expect_err("the second need is unclaimed, so the pair does not resolve");
    assert_eq!(
        capability_of(&err),
        Some(NEEDED),
        "AC-494: the token named is the one that was missing, not the first one asked about"
    );
    assert_eq!(
        first_unclaimed(&two, adapter.registry()).map(CapabilityNeed::step),
        Some("select-tests"),
        "and the site is the second need's"
    );
}

// ---- instrument 2 — the exhaustiveness test -----------------------------------------------------

/// **AC-494, instrument 2.** *"An exhaustiveness test that `StepError::Unsupported` carries the
/// token."*
///
/// The exhaustiveness is [`capability_of`]'s wildcard-free `match`: a twelfth `StepError` variant
/// makes this file fail to compile with `E0004`. That is the only form in which a test can observe
/// exhaustiveness — a runtime assertion cannot see a variant nobody wrote yet.
///
/// The eleven are asserted to BE eleven and to be pairwise distinct first, so a table that silently
/// lost a row cannot pass.
#[test]
fn ac494_unsupported_carries_the_token_and_no_other_variant_does() {
    let all = every_step_error();
    assert_eq!(
        all.len(),
        11,
        "ADR-0112's closed set is eleven (ADR-0095's eight plus Platform, Unsupported and Overage)"
    );
    for (i, a) in all.iter().enumerate() {
        for b in all.iter().skip(i + 1) {
            assert_ne!(a, b, "the eleven rows must be eleven different values");
        }
    }

    // EXACTLY ONE of the eleven carries a capability token, and it is `Unsupported`.
    let carriers: Vec<&StepError> = all.iter().filter(|e| capability_of(e).is_some()).collect();
    assert_eq!(
        carriers.len(),
        1,
        "exactly one variant carries a capability token; got {carriers:?}"
    );
    assert_eq!(
        capability_of(carriers[0]),
        Some(NEEDED),
        "and it carries the token it was constructed with, unchanged"
    );
    assert_eq!(error_tag(carriers[0]), "unsupported");

    // And the value the RESOLVER mints is that variant, carrying that token — so the exhaustiveness
    // above is about the error this criterion actually produces and not about a hand-made one.
    let minted = resolve_activity(&needs(), &registry_omitting_the_capability())
        .expect_err("the fixture registry omits the token");
    assert_eq!(capability_of(&minted), Some(NEEDED));
    assert!(
        all.contains(&minted),
        "the resolver's refusal is one of ADR-0112's eleven and not a twelfth shape"
    );

    // A token with punctuation and a token that is a single character both survive the field
    // verbatim: the field is a `String` and this crate never parses it (AC-492's vocabulary rule is
    // the adapter's, and `src/resolver.rs` says so by id).
    for token in ["x", "a/b:c@1", "affected-selection"] {
        let need = vec![CapabilityNeed::new(AID, "s", token)];
        let err = resolve_activity(&need, &ClaimedCapabilities::claims_nothing())
            .expect_err("an adapter claiming nothing claims this token either");
        assert_eq!(
            capability_of(&err),
            Some(token),
            "{token:?} survives verbatim"
        );
    }
}

// ---- instrument 3 — the negative test over the SHIPPED retry policy ------------------------------

/// **AC-494, instrument 3.** *"A negative test asserting the retry policy does not fire on
/// `Unsupported`."*
///
/// **Over the resolver's own error and over the whole attempt range**, which is what keeps it from
/// being a second copy of `never_retried_class_ac027`: that test is table-driven over AC-027's four
/// classes at attempt 1, with a hand-made error. This one takes the value
/// [`rk_kernel::resolver::resolve_activity`] actually produced and puts it to the shipped policy at
/// every attempt from 1 to `N + 2`, because *"does not fire"* is a claim about every attempt and not
/// about the first.
///
/// **The policy is the shipped one.** `rk_kernel::retry::decide` and
/// `rk_types::error::StepError::retry_class` are checkpoint-passed code; a copy of the table here
/// would pass while the real policy retried.
#[test]
fn ac494_the_shipped_retry_policy_does_not_fire_on_unsupported() {
    let err = resolve_activity(&needs(), &registry_omitting_the_capability())
        .expect_err("the fixture registry omits the token");
    let cap = AttemptCap::declared(CAP);

    // PREMISE — the shipped classifier says `Never` before the policy is consulted.
    assert_eq!(
        err.retry_class(),
        RetryClass::Never,
        "ADR-0095 decision 2's table as ADR-0112 extends it: `Unsupported` is never retried — \
         \"configuration, not weather\""
    );

    // THE NEGATIVE — at every attempt, including past the cap, the decision is a refusal naming the
    // class, and it is never any of the four decisions that would re-dispatch, wait or cap.
    for attempt in 1..=CAP + 2 {
        let decision = decide(&err, &key_at(attempt), cap);
        assert_eq!(
            decision,
            RetryDecision::Refuse {
                class: "unsupported"
            },
            "AC-494: the retry policy does not fire on `Unsupported` at attempt {attempt}"
        );
    }

    // THE TWO MOMENTS ARE KEPT APART, and the shipped code says which is which:
    // `refused_before_spawn`'s own doc — *"`Overage` is refused BEFORE spawn; `Unsupported` fails
    // pack validation at the requiring row … folding `Unsupported` in here would claim a pre-spawn
    // refusal for a condition pack validation catches earlier still"*. The resolver IS that earlier
    // catch (AC-495 is its pack-side twin), so this predicate must stay false for `Unsupported`:
    // if it ever turns true, the run has been told the refusal happened at the wrong moment.
    assert!(
        !refused_before_spawn(&err),
        "`Unsupported` is caught earlier than the pre-spawn branch — at resolution (AC-494) and at \
         pack validation (AC-495) — and `rk_kernel::retry::refused_before_spawn` must keep the two \
         moments apart"
    );

    // ---- THE DISCRIMINATOR. Without it, every row above is satisfied by a policy that refuses
    // everything at every attempt. A `Transport` failure is `RetryClass::Backoff` and must retry.
    let weather = StepError::Transport {
        kind: TransportKind::Io,
        detail: "connection reset".to_owned(),
    };
    assert!(
        cap.get() > 1,
        "the control is vacuous at N = 1; the fixture declares {cap}"
    );
    let below_cap = decide(&weather, &key_at(1), cap);
    assert!(
        matches!(below_cap, RetryDecision::Backoff { .. }),
        "a weather class retries below the cap; got {below_cap:?} — without this the refusals above \
         are satisfied by a policy that refuses everything"
    );
    assert_ne!(
        decide(&weather, &key_at(1), cap),
        decide(&err, &key_at(1), cap),
        "the two classes must reach different decisions at the same attempt"
    );
}

// ---- the row shape, and the three criteria this crate does not claim ----------------------------

/// Design §3.3 P5: *"Capability differences are **data**: `{capability, verified_on, probe}`, with
/// **no capability readable except through a row carrying all three**."*
///
/// The row-shape half is structural — [`CapabilityClaim::new`] is the only constructor and it
/// refuses an incomplete row — which is what makes design:363's *"a capability with no dated probe
/// is `Unsupported`, never assumed"* true by construction here rather than by a check something
/// could skip.
///
/// **AC-489, AC-490 and AC-492 are NOT claimed by this test and are not built**: the registry
/// schema rule, the staleness rule and the token vocabulary are adapter-side criteria with their own
/// instruments. `verified_on` is carried and never compared — there is no clock in this crate to
/// compare it against.
#[test]
fn a_capability_is_readable_only_through_a_complete_row() {
    for (field, row) in [
        ("capability", ("", "2026-09-14", "probe:1")),
        ("verified_on", (NEEDED, "", "probe:1")),
        ("probe", (NEEDED, "2026-09-14", "")),
        ("verified_on", (NEEDED, "   ", "probe:1")),
    ] {
        let err = CapabilityClaim::new(row.0, row.1, row.2)
            .expect_err("an incomplete row is not a claim");
        assert_eq!(
            err,
            ClaimError::FieldEmpty { field },
            "design P5's row carries all three, and the refusal names the one that is missing"
        );
    }

    let claim = CapabilityClaim::new(NEEDED, "2026-09-14", "toolchain:affected@2").expect("a row");
    assert_eq!(claim.capability(), NEEDED);
    assert_eq!(claim.verified_on(), "2026-09-14", "carried, never compared");
    assert_eq!(claim.probe(), "toolchain:affected@2", "carried, never run");

    // TWO ROWS FOR ONE TOKEN ARE REFUSED rather than merged. Which row wins is a policy no record
    // states, and `00-preamble.md`:55 forbids a loader taking that decision silently.
    let dup = ClaimedCapabilities::from_claims(vec![
        CapabilityClaim::new(NEEDED, "2026-09-14", "probe:a").expect("a row"),
        CapabilityClaim::new(NEEDED, "2026-09-01", "probe:b").expect("a row"),
    ])
    .expect_err("two rows for one token");
    assert_eq!(
        dup,
        ClaimError::DuplicateToken {
            capability: NEEDED.to_owned()
        }
    );

    // The empty set claims nothing, and says so as a number rather than by returning `false` from a
    // lookup that might simply be broken.
    let none = ClaimedCapabilities::claims_nothing();
    assert!(none.is_empty());
    assert_eq!(none.len(), 0);
    assert!(none.tokens().is_empty());
    assert!(!none.claims_token(NEEDED));

    let set = registry_claiming_the_capability();
    assert_eq!(
        set.tokens(),
        [NEEDED, "json-schema", CLAIMED],
        "byte order, so the order rows were supplied in never reaches a verdict"
    );
    assert_eq!(
        set.claim(NEEDED).map(CapabilityClaim::probe),
        Some("toolchain:affected@2"),
        "and the row behind a claimed token is recoverable for AC-490's and AC-491's rules"
    );
}

// ---- the two criteria are one reading, and the fixtures agree -----------------------------------

/// **AC-494 and AC-495 meet here, and that is the reading `s7b`:7140 states**: *"correct is carried
/// by AC-494 and AC-495, which fail if it does not."*
///
/// The pack whose only route needs `affected-selection` (AC-495's unroutable fixture) and the
/// activity whose need is `affected-selection` (AC-494's) are **the same scenario at two moments**:
/// validation refuses the pack at startup, and the resolver refuses the activity at resolution. This
/// test asserts the two agree on the token — if one fixture's spelling drifts from the other's, the
/// pair stops describing one system and nothing else would notice.
#[test]
fn the_pack_that_cannot_route_and_the_activity_that_cannot_resolve_name_one_token() {
    const UNROUTABLE: &str = include_str!("../fixtures/packs/ac495-unroutable.toml");

    let claimed = registry_omitting_the_capability();

    let violations = pack_capability_routing_rule("ac495-unroutable.toml", UNROUTABLE, &claimed)
        .expect_err("the unroutable pack does not validate against this registry");
    let pack_side: Vec<String> = violations
        .iter()
        .filter_map(|v| match v {
            rk_kernel::pack_validation::RoutingViolation::StageUnroutable {
                capability, ..
            } => Some(capability.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        pack_side,
        [NEEDED.to_owned()],
        "AC-495 refuses the pack on exactly the token AC-494 refuses the activity on"
    );

    let resolver_side =
        resolve_activity(&needs(), &claimed).expect_err("the same token, unclaimed");
    assert_eq!(
        capability_of(&resolver_side),
        Some(pack_side[0].as_str()),
        "one token, two moments: pack validation at startup (AC-495) and resolution (AC-494)"
    );
}
