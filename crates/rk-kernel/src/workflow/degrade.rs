//! **Degradation — a typed value, never silent.** AC-036's refusal, AC-037's two sinks and
//! AC-038's tier-demanding gate.
//!
//! > **AC-036** — `rk-kernel` policy · decisions: ADR-0014, ADR-0016
//! > GIVEN an activity whose card names a model tier
//! > WHEN the kernel would substitute a different model (Opus → Sonnet) and the pack does not permit
//! > substitution
//! > THEN the substitution is refused and the activity fails rather than running at the lower tier.
//! > *Verified by:* a unit test over a pack without the permission asserting refusal; a must-fail
//! > fixture pack.
//! >
//! > **AC-037** — `rk-kernel` policy · decisions: ADR-0014, ADR-0006
//! > GIVEN a permitted model substitution
//! > WHEN the activity completes
//! > THEN `Degraded { from, to, why }` appears in both the event log and the activity's digest.
//! > *Verified by:* an event-schema test and a digest-schema test asserting the field set; a
//! > property test that a substituted dispatch cannot be constructed without the record.
//! >
//! > **AC-038** — `rk-kernel` policy · decisions: ADR-0015, ADR-0020, ADR-0074
//! > GIVEN a gate whose policy demands a review tier
//! > WHEN the review that would close it was produced under a `Degraded` substitution below that
//! > tier
//! > THEN the gate does not close and the reason names the degradation.
//! > *Verified by:* a unit test pairing a degraded review with a tier-demanding gate, asserting
//! > non-closure; a policy-table row test covering each rigor level (ADR-0020's deontic enum).
//!
//! # 🛑 "CARD" MEANS THE RESOLVED (BOUND) CARD, AND THE BINDER IS BUILT BY NO STAGE
//!
//! The register's §0 glossary, which names AC-036 explicitly (PIA-015): *"**the resolved (bound)
//! card** | The dispatch card the binder materialises by injecting the **role's** tier (`model`,
//! `effort`) into the authored card; the authored card file never carries `model` or `effort`
//! (AC-118, AC-418), so a dispatch's tier is traceable only through its role."* So the tier AC-036's
//! GIVEN names is **not** on any file this crate reads: it is a value the binder computes at plan
//! time, and `runbook/PROGRESS.md`:279 records that *"no stage authors an agent card or builds the
//! binder"*. `ActivitySpec` carries the card as `card_digest` — *"ADR-0099:61's five fields and no
//! sixth"* — so the resolved tier and the dispatch are separated by a hash in any case.
//!
//! [`ModelTier`] therefore has exactly one constructor, [`ModelTier::resolved`], which takes the
//! token from the caller. **The binding is ARMED, NOT DISCHARGED against Step 7c**, and it is the
//! same missing model-identity routing the breaker's key needs — `{from, to}` here and the family
//! there are two consumers of one absent record. Whoever decides it should decide it once.
//!
//! # 🛑 NO ORDERING OVER `ModelTier` IS INVENTED, AND "BELOW" IS READ FAIL-CLOSED
//!
//! AC-038's WHEN says *"below that tier"*. An ordering over model tiers exists in no record: nothing
//! above this file says whether `claude-opus-4` outranks `claude-sonnet-4-5`, or by what axis. So
//! [`gate_closes`] does **not** compare magnitudes. It reads *"below"* as **"not the tier the gate
//! demanded"**: a gate demands a tier, the review ran under a substitution to some other tier, and
//! the demand was therefore not met. That is fail-closed — a lateral or upward substitution also
//! fails to close a tier-demanding gate — which is the safe direction and is the one principle 4
//! licenses without a record. A record settling the ordering would narrow it; until then the reading
//! is stated here and asserted in `degradation_ac036_ac038`.
//!
//! # The permission is a pack property, and its absence is a refusal rather than a default
//!
//! AC-036's *"the pack does not permit substitution"* is read through
//! [`crate::select::Pack::permits_substitution`], over the key `[degradation] permit_substitution`.
//! **The key spelling is a READING and no record states it** — the same gap Slice 4b recorded for
//! `[retry] attempts_cap`, `[breaker] consecutive_platform_failures` and `appetite.tokens`, and a
//! record naming all four is OWED. [measured 2026-09-14: `grep -rn "permit_substitution\|
//! \[degradation\]" spikes/p9/packs/` → 0 hits across all twelve shipped packs.]
//!
//! **An absent permission is not a defaulted number.** §7b.10's 🛑 covers `K`, `N`, the backoff
//! base and cap, the headroom margin and the appetite — quantities with no measured value. A
//! permission is not one of those: AC-036's own WHEN is *"the pack **does not permit**
//! substitution"*, and a permission that was never granted is a permission that is absent.
//! [`crate::select::Pack::substitution_permitted`] keeps `None` and `Some(false)` distinguishable so
//! the absence stays visible; [`crate::select::Pack::permits_substitution`] folds the two to the
//! refusal.
//!
//! # AC-037's two sinks, and why one of them is a type fact
//!
//! *"appears in **both** the event log and the activity's digest"* is two obligations, and this
//! module builds a function for each: [`degraded_event`] mints the `EventKind::Degraded` row with
//! exactly [`Degraded::FIELDS`]'s three members and no fourth, and [`activity_result_digest`] folds
//! the record into the digest so that the digest over a result **without** it differs from the one
//! with it. `EventKind::Degraded` already exists (`event_kind.rs`), so **no kind is minted here**
//! and ADR-0116's closure rule is untouched.
//!
//! The third instrument — *"a property test that a substituted dispatch cannot be constructed
//! without the record"* — is built as a **type fact** rather than as a test: [`SubstitutedDispatch`]
//! holds [`Degraded`] **by value**, with no `Option`, no `Default` and no second constructor, so
//! there is no expression in any crate that produces one without the record. The test asserts the
//! two sinks; the compiler asserts the third.
//!
//! **The digest is `rk_store::canon`'s**, never `serde_json::to_vec`: `serde_json` carries
//! `preserve_order` workspace-wide, so a plain serialisation would leak construction order into a
//! comparison AC-037 rests on (`src/select/canonical.rs`:1-19).
//!
//! # AC-038's policy table, and where each row's behaviour comes from
//!
//! *"a policy-table row test covering each rigor level (ADR-0020's deontic enum)"*. `Rigor::ALL` is
//! **five**, so the sweep is five rows and not one case. Two of the five behave differently from the
//! other three, and both differences are read off records rather than chosen:
//!
//! | level | undegraded review | degraded review |
//! | --- | --- | --- |
//! | `forbidden` | **does not close** — ADR-0057: nothing in that stage runs, so no review closes a gate there | does not close |
//! | `discouraged` | **does not close** — reserved, its semantics Phase 2's, and a pack setting it fails validation (AC-050) | does not close |
//! | `optional` / `recommended` / `mandatory` | closes | **does not close, and the reason names the degradation** (AC-038) |
//!
//! The undegraded column is what stops the sweep being vacuous: a `gate_closes` that never closed
//! would satisfy every degraded row and fail three undegraded ones.
//!
//! # What is deliberately NOT here
//!
//! No dispatch and no append: [`degraded_event`] returns an [`Event`] and the write is the runtime
//! boundary's (Slice 4h). No `EventKind` minted. No model catalogue, no tier ordering, no binder.
//! No `Default` on [`Degraded`] or [`ModelTier`] — a defaulted tier is exactly the silent
//! substitution AC-036 exists to refuse. And **AC-236 and AC-363 are other areas' ids and are not
//! claimed here**: §0.3 records that AC-236's *gate* half is AC-038's behaviour under a second id
//! and that AC-038 owns it, so nothing in this module may be read as covering either twin.

use std::fmt;

use rk_store::canon::CanonError;
use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use super::budget::GateVerdict;
use crate::rigor::Rigor;

/// The resolved (bound) card's model tier — **an opaque token this crate never computes.**
///
/// One constructor, named `resolved` for the register's own phrase: the tier is the **binder's**
/// output, injected into the authored card from the role, and the binder is built by no stage. See
/// the header's first 🛑.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModelTier(String);

impl ModelTier {
    /// From the tier a resolved card carried. **The only constructor.**
    #[must_use]
    pub fn resolved(token: &str) -> Self {
        Self(token.to_owned())
    }

    /// The tier token, as resolved.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModelTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// **AC-037's record.** All three fields non-optional, and no fourth.
///
/// No `Default`: a `Degraded` whose `from`, `to` and `why` were all empty strings is exactly the
/// silent substitution AC-036 and AC-037 exist to make impossible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Degraded {
    /// The tier the resolved card named.
    pub from: ModelTier,
    /// The tier the dispatch actually ran at.
    pub to: ModelTier,
    /// Why the substitution happened, in the words of whatever decided it.
    pub why: String,
}

impl Degraded {
    /// The field set, in the criterion's own order. **Exactly three, and the count is asserted**, so
    /// a fourth member added to either sink goes red rather than passing unnoticed.
    pub const FIELDS: [&'static str; 3] = ["from", "to", "why"];

    /// The record. All three by value.
    #[must_use]
    pub fn new(from: ModelTier, to: ModelTier, why: &str) -> Self {
        Self {
            from,
            to,
            why: why.to_owned(),
        }
    }

    /// The record as both sinks carry it — hand-written, because there is no `serde` derive in this
    /// crate (`kernel-style.md` §3.2).
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "from": self.from.as_str(),
            "to": self.to.as_str(),
            "why": self.why,
        })
    }
}

impl fmt::Display for Degraded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} ({})", self.from, self.to, self.why)
    }
}

/// What [`substitute`] decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubstitutionOutcome {
    /// **AC-036's THEN.** The pack does not permit substitution, so the activity fails rather than
    /// running at the other tier.
    Refused {
        /// The tier the resolved card named.
        card_tier: ModelTier,
        /// The tier that was proposed and refused.
        proposed: ModelTier,
    },
    /// **AC-037's GIVEN.** The pack permits it, and the record travels with the permission.
    Permitted(Degraded),
    /// The proposed tier **is** the card's tier, so nothing was substituted.
    ///
    /// **A third variant beyond design §2.6's two, and the reason is the two sinks.** With two
    /// variants a same-tier proposal must be either `Refused` — which fails an activity that needed
    /// no substitution, and AC-036's THEN is about *"running at the lower tier"*, not about running
    /// at the tier the card named — or `Permitted(Degraded { from: X, to: X })`, which would put a
    /// degradation record that degrades nothing into **both** of AC-037's sinks on a dispatch where
    /// nothing happened. Neither is the criterion. This variant costs nothing, mints no record, and
    /// keeps `from != to` true of every [`Degraded`] this module produces.
    NotASubstitution {
        /// The tier the card named, which is also the tier proposed.
        tier: ModelTier,
    },
}

impl SubstitutionOutcome {
    /// A stable tag, so a test names the outcome it expects without matching message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Refused { .. } => "refused",
            Self::Permitted(_) => "permitted",
            Self::NotASubstitution { .. } => "not_a_substitution",
        }
    }

    /// Whether the substitution was refused.
    #[must_use]
    pub const fn is_refused(&self) -> bool {
        matches!(self, Self::Refused { .. })
    }

    /// The record, when one was minted. `None` on a refusal — **there is no degradation record for
    /// a substitution that did not happen.**
    #[must_use]
    pub const fn record(&self) -> Option<&Degraded> {
        match self {
            Self::Refused { .. } | Self::NotASubstitution { .. } => None,
            Self::Permitted(degraded) => Some(degraded),
        }
    }
}

impl fmt::Display for SubstitutionOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Refused {
                card_tier,
                proposed,
            } => write!(
                f,
                "the substitution of `{proposed}` for the card's `{card_tier}` is refused: the \
                 pack does not permit substitution, so the activity fails rather than running at \
                 the other tier (AC-036)"
            ),
            Self::Permitted(degraded) => write!(f, "substitution permitted: {degraded}"),
            Self::NotASubstitution { tier } => write!(
                f,
                "no substitution: the proposed tier is the card's own `{tier}`, so nothing is \
                 degraded and no record is minted (AC-037's record is for a substitution that \
                 happened)"
            ),
        }
    }
}

/// **AC-036.** Decide whether a proposed model substitution may happen.
///
/// A substitution to the tier the card already names is not a substitution and mints no record: the
/// activity runs at the tier it was bound to.
#[must_use]
pub fn substitute(
    card_tier: &ModelTier,
    proposed: &ModelTier,
    pack_permits: bool,
    why: &str,
) -> SubstitutionOutcome {
    // A tier substituted for itself is not a substitution — checked FIRST, so a permitting pack
    // cannot mint a `Degraded { from: X, to: X }` into both of AC-037's sinks on a dispatch where
    // nothing changed.
    if card_tier == proposed {
        return SubstitutionOutcome::NotASubstitution {
            tier: card_tier.clone(),
        };
    }
    if pack_permits {
        return SubstitutionOutcome::Permitted(Degraded::new(
            card_tier.clone(),
            proposed.clone(),
            why,
        ));
    }
    // AC-036's THEN. The refusal carries both tiers, so the failure names what was asked for and
    // what was on the card — and it mints no record, because nothing was degraded.
    SubstitutionOutcome::Refused {
        card_tier: card_tier.clone(),
        proposed: proposed.clone(),
    }
}

/// **AC-037's first sink.** The `degraded` row, carrying exactly [`Degraded::FIELDS`]'s three
/// members.
///
/// `EventKind::Degraded` is an existing variant of the closed 31 *(29 until ADR-0123)* (design §5 row 20), so nothing is
/// minted here. The **write** is the runtime boundary's; this returns the value.
#[must_use]
pub fn degraded_event(degraded: &Degraded) -> Event {
    Event::new(EventKind::Degraded, degraded.to_json())
}

/// **AC-037's second sink.** The activity result's digest, with the record folded in.
///
/// The digest over a result carrying the record and the digest over the same result without it are
/// **different values**, which is what makes *"appears in … the activity's digest"* an assertion
/// rather than a claim. `rk_store::canon` is the one canonical form and `rk_types::digest` the one
/// hash (ADR-0116 clause 6).
///
/// # Errors
///
/// [`CanonError`] when a float reached the canonical serialiser, which `rk-store` refuses rather
/// than choosing a format for.
pub fn activity_result_digest(
    result: &serde_json::Value,
    degraded: Option<&Degraded>,
) -> Result<Digest, CanonError> {
    // The member is present on BOTH sides — `null` when there was no substitution — rather than
    // omitted, so the digest distinguishes "ran undegraded" from "the field was dropped". The
    // canonical form sorts the two keys byte-wise, so this is not an insertion-order artefact.
    rk_store::canon::canonical_digest(&serde_json::json!({
        "result": result,
        "degraded": degraded.map_or(serde_json::Value::Null, Degraded::to_json),
    }))
}

/// **AC-037's third instrument, as a type fact.** A dispatch that ran under a substitution carries
/// the record **by value**.
///
/// > *a property test that a substituted dispatch cannot be constructed without the record.*
///
/// There is no `Option`, no `Default`, no second constructor and no public field, so there is no
/// expression in any crate that produces one of these without a [`Degraded`]. The property is
/// therefore the compiler's and not a test's.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstitutedDispatch {
    spec: ActivitySpec,
    degraded: Degraded,
}

impl SubstitutedDispatch {
    /// The only constructor. Both by value.
    #[must_use]
    pub const fn new(spec: ActivitySpec, degraded: Degraded) -> Self {
        Self { spec, degraded }
    }

    /// The dispatch specification.
    #[must_use]
    pub const fn spec(&self) -> &ActivitySpec {
        &self.spec
    }

    /// The degradation record this dispatch could not have been built without.
    #[must_use]
    pub const fn degraded(&self) -> &Degraded {
        &self.degraded
    }
}

/// **AC-038.** Whether a tier-demanding gate closes on a review produced under a substitution.
///
/// Total over `Rigor::ALL`'s five levels with no catch-all arm, so a sixth deontic level would be
/// an `E0004` here. See the header's policy table for where each row's behaviour comes from and the
/// second 🛑 for why *"below that tier"* is read as *"not the demanded tier"*.
#[must_use]
pub fn gate_closes(
    demanded: &ModelTier,
    produced_under: Option<&Degraded>,
    level: Rigor,
) -> GateVerdict {
    // TWO INDEPENDENT BARS, and the verdict is open if EITHER stands. Keeping them separate is
    // what lets AC-038's THEN hold at every one of the five levels: *"the gate does not close and
    // the reason names the degradation"*. A level that runs nothing is a reason of its own, and
    // collapsing the two would leave the degradation unnamed on two of the five rows.
    let level_bar = match level {
        // ADR-0057: nothing in a `forbidden` stage runs, so there is no review to close a gate
        // with — degraded or not.
        Rigor::Forbidden => Some(format!(
            "the stage's rigor is `{level}`, so nothing in it runs (ADR-0057) and no review \
             closes this gate"
        )),
        // `discouraged` is RESERVED, its semantics Phase 2's, and a pack that sets it fails
        // validation (AC-050, `select/pack.rs`'s `DiscouragedIsReserved`). A gate evaluated at a
        // level no pack may declare does not close.
        Rigor::Discouraged => Some(format!(
            "the rigor `{level}` is reserved and its semantics are Phase 2's (AC-050), so no \
             review is evaluated at it and this gate does not close"
        )),
        Rigor::Optional | Rigor::Recommended | Rigor::Mandatory => None,
    };

    let tier_bar = produced_under.and_then(|degraded| {
        // "below that tier", read as "not the demanded tier" — see the header's second 🛑. No
        // ordering over `ModelTier` exists in any record, and the fail-closed reading needs none.
        if degraded.to == *demanded {
            return None;
        }
        Some(format!(
            "the gate demands the `{demanded}` tier, and the review that would close it was \
             produced under a substitution from `{}` to `{}` ({}): the gate does not close \
             and the reason names the degradation (AC-038)",
            degraded.from, degraded.to, degraded.why
        ))
    });

    match (level_bar, tier_bar) {
        (None, None) => GateVerdict::closed(),
        (Some(reason), None) | (None, Some(reason)) => GateVerdict::open(reason),
        (Some(level_reason), Some(tier_reason)) => {
            GateVerdict::open(format!("{level_reason}; and {tier_reason}"))
        }
    }
}
