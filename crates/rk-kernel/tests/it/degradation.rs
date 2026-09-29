//! Degradation — **AC-036, AC-037 and AC-038** under one name, which is three criteria and seven
//! obligations.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | measured here | ARMED, not discharged |
//! | --- | --- | --- |
//! | **AC-036** | the refusal against a pack without the permission, the must-fail fixture pack, and the `None`/`Some(false)` split that keeps *"never considered"* and *"considered and refused"* apart | the **model-tier binding**: the tier of the *resolved (bound)* card is the binder's, `runbook/PROGRESS.md`:279 records that no stage builds the binder, and `ActivitySpec` carries the card as a digest. Every tier below is a token this test supplies |
//! | **AC-037** | both sinks — the `degraded` event row with exactly three payload members, and the activity-result digest, which **differs** from the digest of the same result without the record — plus the third instrument as a type fact | the **typed payload record**, per kind, which `rk-types/src/event.rs`:46-58 records as OWED. The field set asserted here is `Degraded::FIELDS`, not a schema |
//! | **AC-038** | a policy-table row test over **all five** of `Rigor::ALL`, in both columns, with the reason naming the degradation | an **ordering** over model tiers. No record gives one, so *"below that tier"* is read fail-closed as *"not the demanded tier"* — stated in `src/workflow/degrade.rs`'s second 🛑 and asserted below |
//!
//! **AC-236 and AC-363 are not claimed here.** §0.3 records that AC-236's *gate* half — *"the gate
//! does not close on a degraded result"* — is AC-038's behaviour under a second id and that AC-038
//! owns it (PIA-070). Nothing in this file may be read as covering either twin.

use rk_kernel::rigor::Rigor;
use rk_kernel::select::Pack;
use rk_kernel::workflow::degrade::{
    Degraded, ModelTier, SubstitutedDispatch, SubstitutionOutcome, activity_result_digest,
    degraded_event, gate_closes, substitute,
};
use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::event_kind::EventKind;
use rk_types::typed::SchemaId;

/// The pack that **permits** substitution.
const FIXTURE_PERMITTED: &str = include_str!("../fixtures/packs/runtime-degrade-permitted.toml");
/// AC-036's must-fail fixture pack: `permit_substitution = false`, declared.
const FIXTURE_REFUSED: &str = include_str!("../fixtures/packs/runtime-no-degrade.toml");
/// A pack with no `[degradation]` table at all — the absence, which is not the same state.
const FIXTURE_SILENT: &str = include_str!("../fixtures/packs/runtime-envelope.toml");

/// The degradation module's own source, for the type-fact assertion AC-037's third instrument is.
const DEGRADE_SRC: &str = include_str!("../../src/workflow/degrade.rs");

/// Load one pack fixture.
///
/// # Panics
///
/// With the loader's own error, so a fixture that stops parsing names its reason.
fn pack(name: &str, src: &str) -> Pack {
    Pack::from_toml(name, src).unwrap_or_else(|e| panic!("the {name} fixture loads: {e}"))
}

/// Strip comment lines before a source lint runs: this module's header quotes the shapes it bans.
fn code_only(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// A dispatch specification. Five fields and no sixth (ADR-0099 clause 1)
/// (*(dated 2026-09-29, s8f build, package S8F-P7; ADR-0190 decision 2, proposed; D21-107.)* plus `activity_id_at_version`, `None` in a
/// fixture) — and **no tier**, which
/// is the point: the tier reaches this test as a token and could not have reached it through here.
///
/// # Panics
///
/// If the constant digest below stops being 64 lowercase hex characters.
fn spec() -> ActivitySpec {
    ActivitySpec {
        card_digest: Digest::from_hex(&"ab".repeat(32)).expect("64 lowercase hex characters"),
        tools: vec!["Read".to_owned()],
        schema: SchemaId::new("ImplReport/v1"),
        timeout_secs: 900,
        inputs: vec![("item".to_owned(), serde_json::json!("issue-81"))],
        activity_id_at_version: None,
    }
}

/// **AC-036 + AC-037 + AC-038**, the runbook's own name (§7b.13:1169-1170, §7b.14's third loop).
///
/// Seven obligations, and none of them may drop:
///
/// * **AC-036** (1) a pack without the permission refuses the substitution; (2) the must-fail
///   fixture pack is exercised as itself, and the **absence** of a `[degradation]` table is
///   distinguished from a declared `false` — without which a `permits_substitution()` hard-wired to
///   `false` would pass; (3) the control: a permitting pack permits, so "refuses" is not a constant.
/// * **AC-037** (4) the event sink — `EventKind::Degraded`, exactly three payload members; (5) the
///   digest sink — the digest of a result carrying the record differs from the digest of the same
///   result without it, and a different `why` moves it again, so the record's *content* is in the
///   digest and not a flag; (6) the property, as a type fact — `SubstitutedDispatch` takes
///   `Degraded` by value, asserted at runtime and pinned in the source.
/// * **AC-038** (7) a policy-table row test over **all five** rigor levels in **both** columns, with
///   the reason naming both tiers of the degradation.
#[test]
fn degradation_ac036_ac038() {
    let opus = ModelTier::resolved("opus");
    let sonnet = ModelTier::resolved("sonnet");
    assert_ne!(opus, sonnet, "the premise: two distinct tiers");

    // ================ AC-036 — the refusal, and the two shapes of "no permission" ================

    let refusing = pack("runtime-no-degrade.toml", FIXTURE_REFUSED);
    let silent = pack("runtime-envelope.toml", FIXTURE_SILENT);
    let permitting = pack("runtime-degrade-permitted.toml", FIXTURE_PERMITTED);

    // THE DISCRIMINATOR. `None` and `Some(false)` are different states of the world, and a
    // `permits_substitution()` written to return `false` unconditionally would satisfy every
    // refusal assertion below. This is the pair that catches it.
    assert_eq!(
        refusing.substitution_permitted(),
        Some(false),
        "the must-fail fixture pack DECLARES the refusal"
    );
    assert_eq!(
        silent.substitution_permitted(),
        None,
        "a pack with no `[degradation]` table never considered substitution — that is an absence, \
         not a declared `false`"
    );
    assert_eq!(
        permitting.substitution_permitted(),
        Some(true),
        "and the permitting pack is read as permitting, so the accessor is not a constant"
    );

    // Both refusing shapes fold to the same refusal — fail-closed, which is the direction AC-036's
    // THEN points. An absent permission is a permission that was never granted.
    for (name, p) in [("declared false", &refusing), ("absent", &silent)] {
        assert!(
            !p.permits_substitution(),
            "a pack whose permission is {name} does not permit substitution"
        );
        let outcome = substitute(&opus, &sonnet, p.permits_substitution(), "capacity");
        assert_eq!(
            outcome,
            SubstitutionOutcome::Refused {
                card_tier: opus.clone(),
                proposed: sonnet.clone(),
            },
            "AC-036's THEN: the substitution is refused and the activity fails rather than running \
             at the lower tier ({name})"
        );
        assert!(outcome.is_refused());
        assert_eq!(outcome.as_str(), "refused");
        assert_eq!(
            outcome.record(),
            None,
            "a refused substitution mints NO `Degraded` record: there is nothing to record in \
             either of AC-037's sinks for a substitution that did not happen"
        );
    }

    // THE CONTROL, and it is the one that stops `substitute` being `fn(..) -> Refused`.
    assert!(permitting.permits_substitution());
    let permitted = substitute(
        &opus,
        &sonnet,
        permitting.permits_substitution(),
        "capacity",
    );
    let degraded = match &permitted {
        SubstitutionOutcome::Permitted(d) => d.clone(),
        other => panic!("a permitting pack permits the substitution, got {other:?}"),
    };
    assert_eq!(degraded.from, opus);
    assert_eq!(degraded.to, sonnet);
    assert_eq!(degraded.why, "capacity");
    assert_eq!(permitted.as_str(), "permitted");
    assert_eq!(permitted.record(), Some(&degraded));

    // A proposal of the card's OWN tier is not a substitution and mints no record — so a permitting
    // pack does not produce a `Degraded { from: X, to: X }` into both sinks on every dispatch.
    assert_eq!(
        substitute(&opus, &opus, true, "capacity"),
        SubstitutionOutcome::NotASubstitution { tier: opus.clone() }
    );
    assert_eq!(
        substitute(&opus, &opus, true, "capacity").record(),
        None,
        "nothing was degraded, so nothing is recorded"
    );

    // ======================== AC-037 — BOTH sinks, and the type fact ========================

    // ---- sink one: the event log --------------------------------------------------------------
    let event = degraded_event(&degraded);
    assert_eq!(
        event.kind,
        EventKind::Degraded,
        "`degraded` is an EXISTING variant of the closed 29 (design §5 row 20) — no kind is minted \
         here, and ADR-0116's closure rule is untouched"
    );
    let payload = event
        .payload
        .as_object()
        .expect("the `degraded` payload is an object");
    assert_eq!(
        payload.len(),
        3,
        "exactly `Degraded::FIELDS`, and no fourth member: {payload:?}"
    );
    for field in Degraded::FIELDS {
        assert!(payload.contains_key(field), "the payload carries `{field}`");
    }
    assert_eq!(payload["from"], serde_json::json!("opus"));
    assert_eq!(payload["to"], serde_json::json!("sonnet"));
    assert_eq!(payload["why"], serde_json::json!("capacity"));

    // ---- sink two: the activity's digest ------------------------------------------------------
    let result = serde_json::json!({ "verdict": "pass", "findings": [] });
    let with = activity_result_digest(&result, Some(&degraded)).expect("no float reached canon");
    let without = activity_result_digest(&result, None).expect("no float reached canon");
    assert_ne!(
        with, without,
        "AC-037's second sink: the record APPEARS in the digest, so the digest of a result that \
         carries it differs from the digest of the same result that does not. Equal digests mean \
         the record reached one sink only"
    );
    // ... and the record's CONTENT is in the digest, not merely a flag that one exists.
    let other_why = Degraded::new(opus.clone(), sonnet, "rate limit");
    let with_other = activity_result_digest(&result, Some(&other_why)).expect("no float");
    assert_ne!(
        with, with_other,
        "a different `why` moves the digest: a digest that folded in a bare boolean would not"
    );
    let other_to = Degraded::new(opus, ModelTier::resolved("haiku"), "capacity");
    assert_ne!(
        with,
        activity_result_digest(&result, Some(&other_to)).expect("no float"),
        "and a different `to` moves it too"
    );
    // Determinism, which is what makes the two `assert_ne!`s above mean anything.
    assert_eq!(
        with,
        activity_result_digest(&result, Some(&degraded)).expect("no float"),
        "the digest is a pure function of its inputs"
    );

    // ---- the third instrument, as a TYPE FACT -------------------------------------------------
    // "a property test that a substituted dispatch cannot be constructed without the record."
    let dispatch = SubstitutedDispatch::new(spec(), degraded.clone());
    assert_eq!(dispatch.degraded(), &degraded);
    assert_eq!(dispatch.spec(), &spec());
    // The runtime assertion above is satisfied by a type with an `Option`. The property is not, and
    // it is the compiler's — so the shape is pinned in the source rather than inferred from a call.
    let src = code_only(DEGRADE_SRC);
    for anchor in [
        "pub struct SubstitutedDispatch",
        "degraded: Degraded,",
        "pub const fn new(spec: ActivitySpec, degraded: Degraded) -> Self",
    ] {
        assert!(
            src.contains(anchor),
            "the anchor `{anchor}` is absent from src/workflow/degrade.rs, so the absences below \
             assert nothing about a type that may have been renamed or gutted"
        );
    }
    for banned in [
        "degraded: Option<Degraded>",
        "impl Default for SubstitutedDispatch",
        "impl Default for Degraded",
    ] {
        assert!(
            !src.contains(banned),
            "src/workflow/degrade.rs carries `{banned}`: a substituted dispatch could then be \
             constructed without the record, which is exactly what AC-037's third instrument bans"
        );
    }

    // ========================= AC-038 — the five-row policy table =========================

    let demanded = ModelTier::resolved("opus");
    let below = Degraded::new(demanded.clone(), ModelTier::resolved("sonnet"), "capacity");
    assert_eq!(
        Rigor::ALL.len(),
        5,
        "ADR-0020's deontic enum is five levels, so the sweep is five rows and not one case"
    );

    let mut closed_undegraded = 0_usize;
    for level in Rigor::ALL {
        // The exhaustive match is the row table: a sixth deontic level would be an E0004 here and
        // not a forgotten row.
        let expect_closes_undegraded = match level {
            Rigor::Forbidden | Rigor::Discouraged => false,
            Rigor::Optional | Rigor::Recommended | Rigor::Mandatory => true,
        };

        // Column one: no degradation. THIS is what stops the sweep being vacuous — a `gate_closes`
        // that never closed would satisfy every row of column two and fail three rows here.
        let undegraded = gate_closes(&demanded, None, level);
        assert_eq!(
            undegraded.is_closed(),
            expect_closes_undegraded,
            "at rigor `{level}` an UNDEGRADED review must {} the gate",
            if expect_closes_undegraded {
                "close"
            } else {
                "not close"
            }
        );
        if undegraded.is_closed() {
            closed_undegraded += 1;
        } else {
            let reason = undegraded.reason().expect("a non-closure names its reason");
            assert!(
                reason.contains(level.as_str()),
                "at rigor `{level}` the non-closure has nothing to do with a degradation, so its \
                 reason must name the level: {reason}"
            );
        }

        // Column two: the degraded review. AC-038's THEN, at every level.
        let verdict = gate_closes(&demanded, Some(&below), level);
        assert!(
            !verdict.is_closed(),
            "AC-038: at rigor `{level}` a gate demanding `{demanded}` does not close on a review \
             produced under a substitution to `{}`",
            below.to
        );
        let reason = verdict.reason().expect("a non-closure names its reason");
        assert!(
            reason.contains(below.from.as_str()) && reason.contains(below.to.as_str()),
            "AC-038's second clause: *the reason names the degradation*. At rigor `{level}` the \
             reason must name both `{}` and `{}`: {reason}",
            below.from,
            below.to
        );
    }
    assert_eq!(
        closed_undegraded, 3,
        "exactly the three LIVE levels close without a degradation — `forbidden` runs nothing \
         (ADR-0057) and `discouraged` is reserved (AC-050)"
    );

    // THE DISCRIMINATOR for "below" being read as "not the demanded tier": a substitution that
    // LANDED on the demanded tier met the demand, so the gate closes. Without this, `gate_closes`
    // could be `fn(_, Some(_), _) -> Open` and every assertion above would pass.
    let onto_demanded = Degraded::new(ModelTier::resolved("sonnet"), demanded.clone(), "upgrade");
    assert!(
        gate_closes(&demanded, Some(&onto_demanded), Rigor::Mandatory).is_closed(),
        "a substitution whose `to` IS the demanded tier met the demand: the gate closes, so the \
         rule is about the tier and not about the mere presence of a record"
    );
    // ... and that same record still does not close a gate demanding something else.
    assert!(
        !gate_closes(
            &ModelTier::resolved("haiku"),
            Some(&onto_demanded),
            Rigor::Mandatory
        )
        .is_closed()
    );
}
