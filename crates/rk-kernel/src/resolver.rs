//! **AC-494** — a missing capability is `StepError::Unsupported { capability }` carrying the token,
//! and the claimed set is **data the kernel is handed**.
//!
//! > **AC-494** — GIVEN an activity that needs a capability the selected adapter's registry does not
//! > claim / WHEN the kernel resolves the activity / THEN the activity fails with
//! > `StepError::Unsupported(capability)` carrying the capability token / AND the failure is not a
//! > generic error, not a retry, and never a `pass`.
//! > *Verified by:* a unit test on the resolver with a fake adapter whose registry omits the
//! > capability; an exhaustiveness test that `StepError::Unsupported` carries the token; a negative
//! > test asserting the retry policy does not fire on `Unsupported`.
//!
//! # 🛑 THE CLAIMED SET IS DATA, AND THAT IS WHAT KEEPS THIS CRITERION CONSISTENT WITH AC-487
//!
//! **No capability registry exists in this workspace** [measured 2026-09-14: `grep -rn capabilit
//! crates/rk-ports/src crates/rk-types/src` finds `StepError::Unsupported`'s field, the
//! `ErrorEvent::StepRefused` arm that carries it, and nothing else — no registry type, no
//! `Harness::capabilities`, no probe row]. So the set of tokens an adapter claims arrives here as a
//! **value** — [`ClaimedCapabilities`], constructed by the composition root and passed in — and
//! **never as a lookup on an adapter type**.
//!
//! That is not a convenience. [`crate::kernel_denylist`] is **AC-487**, and it forbids this crate
//! from holding any adapter identity at all: no adapter crate name, no `Any::downcast_ref`, no
//! adapter-discriminant enum. A resolver that asked *"which adapter is this?"* before deciding what
//! it can do would be the exact construct that lint refuses, and the two criteria would contradict
//! each other. They do not, because the question this module asks is *"does this **set** contain
//! this **token**?"* — a `BTreeMap` lookup on data, with no adapter in scope. Design P6's own row
//! says it: *"the lint proves absence of identity checks only; that a capability lookup exists is
//! carried by AC-494/AC-495"*, and proposal §4.1 says *"capability differences are data … never `if
//! adapter == …` in the kernel"*.
//!
//! Read the signatures as the evidence: [`resolve_activity`] takes `&[CapabilityNeed]` and
//! `&ClaimedCapabilities`. It takes no `&dyn Harness`, no `Ports`, no adapter name and no type
//! parameter — **AC-480** forbids the last of those anyway — so there is no value in scope from
//! which an identity could be read even by a later edit.
//!
//! # The row shape, and the three criteria this module deliberately does NOT claim
//!
//! Design §3.3 P5: *"Capability differences are **data**: `{capability, verified_on, probe}`, with
//! no capability readable except through a row carrying all three, and an unknown token failing the
//! build rather than defaulting."* [`CapabilityClaim::new`] enforces the **row shape** half — a
//! claim cannot be constructed without all three, so *"a capability with no dated probe is
//! `Unsupported`, never assumed"* (design:363) holds by construction rather than by a check
//! something could skip.
//!
//! **What is NOT built here, by id:**
//!
//! | id | its instrument | why not here |
//! | --- | --- | --- |
//! | **AC-489** | the registry **schema** test and the row-completeness rule over a loaded registry | there is no registry file and no loader in this tree; the adapter that ships one is 7c's |
//! | **AC-490** | the **staleness** rule — a row older than the declared maximum claim age | nothing here reads a clock and no maximum age is declared by any record; `verified_on` is **carried, never compared** |
//! | **AC-492** | the **vocabulary** test and the catch-all-arm lint over the token mapper | there is no token mapper here; a token is a `String` this crate never interprets |
//!
//! Each of those is an adapter-side criterion in the register's own area column, and a rule written
//! here would be a second one to keep in sync. `verified_on` and `probe` are carried on the claim so
//! the row is whole, and **read by nothing in this crate** — [`CapabilityClaim::verified_on`] and
//! [`CapabilityClaim::probe`] exist for the caller that owns those rules.
//!
//! # Where the NEEDS come from is a decision this stage does not take
//!
//! `rk_types::activity::ActivitySpec` carries five fields and no sixth (ADR-0099:61, and
//! `crates/rk-types/src/activity.rs`:26 says *"five fields and no sixth"* in terms)
//! (*(dated 2026-09-29, s8f build, package S8F-P7; ADR-0190 decision 2, proposed; D21-107.)* six now: ADR-0190's
//! `activity_id_at_version`, which is not a capability either); the activity
//! catalog's schema declares no capability key [measured 2026-09-14:
//! `grep -rhoE '^[a-z_]+ *=' spikes/p9/catalog/*.toml | tr -d ' =' | sort -u` lists **36** keys
//! across the 102 shipped activities and none of them is `capability`]; and `rk-types` is
//! Checkpoint 5's and may not be edited here. So **nothing in this
//! tree says which capability a given activity needs**, and inventing the channel — a new
//! `ActivitySpec` field, a new catalog key, or a mapping from a `tool` step's `run = "toolchain:X"`
//! — would be deciding what no record has.
//!
//! `00-preamble.md`:55 governs: the need arrives as data ([`CapabilityNeed`]), and **a record
//! routing the need from the catalog or the pack to the dispatch site is OWED**. The consequence is
//! stated rather than smoothed: [`crate::runtime::Runtime`] is **not** modified by this slice and
//! calls nothing here, so *"the kernel resolves the activity"* is discharged as the resolver the
//! register's *Verified by* names — *"a **unit test on the resolver**"* — and **not** as an ordering
//! observed inside a live `Runtime::step`. [`dispatch_if_resolved`] is what makes the ordering the
//! kernel's rather than a test's: the closure it is handed **runs only on `Ok`**, so *"never a
//! `pass`"* is measurable as a dispatch count of zero on a fake adapter, with no `match` in the test
//! deciding it.

use std::collections::BTreeMap;
use std::fmt;

use rk_types::error::StepError;

// ---- the row -------------------------------------------------------------------------------------

/// What stopped a claim from being constructed.
///
/// **Not** a staleness verdict and not a vocabulary verdict — those are AC-490's and AC-492's, and
/// the module header says why neither is here. This is the row-shape half of design P5 and nothing
/// else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimError {
    /// One of the three fields of design P5's row is empty. A row that does not carry all three is
    /// not a claim, so there is no value to construct.
    FieldEmpty { field: &'static str },
    /// Two rows claim the same token. Which one wins is a merge policy no record states, so the set
    /// refuses rather than picking (`00-preamble.md`:55).
    DuplicateToken { capability: String },
}

impl fmt::Display for ClaimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldEmpty { field } => write!(
                f,
                "a capability claim carries `capability`, `verified_on` and `probe` (design §3.3 \
                 P5: \"no capability readable except through a row carrying all three\"), and \
                 `{field}` is empty"
            ),
            Self::DuplicateToken { capability } => write!(
                f,
                "two rows claim `{capability}`; which one wins is a merge policy no record states, \
                 so the set refuses rather than choosing one"
            ),
        }
    }
}

impl std::error::Error for ClaimError {}

/// One row of an adapter's capability registry: `{capability, verified_on, probe}`.
///
/// Constructed only through [`Self::new`], which is what makes *"no capability readable except
/// through a row carrying all three"* structural. `verified_on` and `probe` are **carried and never
/// compared** here — see the module header's table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityClaim {
    capability: String,
    verified_on: String,
    probe: String,
}

impl CapabilityClaim {
    /// One complete row, or the refusal.
    ///
    /// # Errors
    ///
    /// [`ClaimError::FieldEmpty`] naming the field, for a row that does not carry all three.
    pub fn new(capability: &str, verified_on: &str, probe: &str) -> Result<Self, ClaimError> {
        for (field, value) in [
            ("capability", capability),
            ("verified_on", verified_on),
            ("probe", probe),
        ] {
            if value.trim().is_empty() {
                return Err(ClaimError::FieldEmpty { field });
            }
        }
        Ok(Self {
            capability: capability.to_owned(),
            verified_on: verified_on.to_owned(),
            probe: probe.to_owned(),
        })
    }

    /// The token this row claims.
    #[must_use]
    pub fn capability(&self) -> &str {
        &self.capability
    }

    /// The date the claim was probed, **carried and never compared** — the comparison is AC-490's.
    #[must_use]
    pub fn verified_on(&self) -> &str {
        &self.verified_on
    }

    /// The probe that established the claim, **carried and never run** — running one is AC-491's
    /// `doctor --probe`, an unnamed verb at this pin (design:1318).
    #[must_use]
    pub fn probe(&self) -> &str {
        &self.probe
    }
}

// ---- the set -------------------------------------------------------------------------------------

/// **The selected adapter's registry, as the kernel receives it: a value.**
///
/// There is no adapter in this type, no name, no discriminant and nothing downcastable — see the
/// module header on AC-487. Lookup is by token over a `BTreeMap`, so the order a caller supplied the
/// rows in never reaches a verdict.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClaimedCapabilities {
    claims: BTreeMap<String, CapabilityClaim>,
}

impl ClaimedCapabilities {
    /// The registry of an adapter that claims **nothing**.
    ///
    /// Named rather than spelled `Default::default()` at call sites, because *"claims nothing"* is
    /// the state AC-494's GIVEN is about and it should be readable as such.
    #[must_use]
    pub fn claims_nothing() -> Self {
        Self::default()
    }

    /// Build the set from rows.
    ///
    /// # Errors
    ///
    /// [`ClaimError::DuplicateToken`] when two rows claim the same token.
    pub fn from_claims(claims: Vec<CapabilityClaim>) -> Result<Self, ClaimError> {
        let mut map = BTreeMap::new();
        for claim in claims {
            if map.contains_key(claim.capability()) {
                return Err(ClaimError::DuplicateToken {
                    capability: claim.capability().to_owned(),
                });
            }
            map.insert(claim.capability().to_owned(), claim);
        }
        Ok(Self { claims: map })
    }

    /// **The whole of the lookup.** Does this set contain this token?
    ///
    /// Byte equality on the token, and deliberately nothing else: no case folding, no trimming, no
    /// prefix match. A near-miss token is a token this adapter does not claim, which is the
    /// direction AC-494 points — *"never assumed"* (design:363).
    #[must_use]
    pub fn claims_token(&self, token: &str) -> bool {
        self.claims.contains_key(token)
    }

    /// The row behind a claimed token, for a caller that owns AC-490's or AC-491's rule.
    #[must_use]
    pub fn claim(&self, token: &str) -> Option<&CapabilityClaim> {
        self.claims.get(token)
    }

    /// Every claimed token, in byte order.
    #[must_use]
    pub fn tokens(&self) -> Vec<&str> {
        self.claims.keys().map(String::as_str).collect()
    }

    /// How many rows the set holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.claims.len()
    }

    /// Whether the adapter claims nothing at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }
}

// ---- the need ------------------------------------------------------------------------------------

/// One capability an activity needs, with the site that declared it.
///
/// **Handed in, never derived here** — the module header names the record that is OWED for the
/// channel. `activity` and `step` are carried so a caller can say *where* the need came from; the
/// **token alone** is what reaches `StepError::Unsupported`, because that is the only field the
/// variant has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityNeed {
    activity: String,
    step: String,
    capability: String,
}

impl CapabilityNeed {
    /// One need.
    #[must_use]
    pub fn new(activity: &str, step: &str, capability: &str) -> Self {
        Self {
            activity: activity.to_owned(),
            step: step.to_owned(),
            capability: capability.to_owned(),
        }
    }

    /// The activity that needs it.
    #[must_use]
    pub fn activity(&self) -> &str {
        &self.activity
    }

    /// The step that needs it.
    #[must_use]
    pub fn step(&self) -> &str {
        &self.step
    }

    /// The token.
    #[must_use]
    pub fn capability(&self) -> &str {
        &self.capability
    }
}

// ---- the verdict ---------------------------------------------------------------------------------

/// House Shape C over one resolution: the counts travel with the verdict, so *"the resolver checked
/// nothing"* is distinguishable from *"the resolver checked everything and found nothing wrong"*.
///
/// A resolver handed an empty need list returns a report whose `needs_checked` is **0**, and a test
/// that asserts a green resolution without reading this number is asserting nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionReport {
    needs_checked: usize,
    claims_walked: usize,
    resolved: Vec<String>,
}

impl ResolutionReport {
    /// How many needs were put to the set.
    #[must_use]
    pub const fn needs_checked(&self) -> usize {
        self.needs_checked
    }

    /// How many rows the set held while they were put to it.
    #[must_use]
    pub const fn claims_walked(&self) -> usize {
        self.claims_walked
    }

    /// Every token that resolved, in the order the needs declared them.
    #[must_use]
    pub fn resolved(&self) -> &[String] {
        &self.resolved
    }
}

/// **The one place `StepError::Unsupported` is minted in this crate**, so the token in the error is
/// the need's token verbatim and cannot drift into a message.
///
/// It is a one-line body on purpose, and there is exactly one of it: a mint site with a body is a
/// mint site that eventually formats something into the field, and *"carrying the capability
/// token"* would quietly become *"carrying a sentence mentioning the capability token"*.
/// `tests/it/resolver.rs` compares the **whole value** against
/// `StepError::Unsupported { capability }`, so that drift is red rather than plausible.
#[must_use]
pub fn unsupported(capability: &str) -> StepError {
    StepError::Unsupported {
        capability: capability.to_owned(),
    }
}

/// The first need this set does not claim, in declaration order, or `None` when every need resolves.
///
/// Declaration order and not byte order: the error names **a** capability, and which one it names
/// must be a function of the pack/catalog the needs came from rather than of a `BTreeMap`'s
/// iteration. The site is returned rather than the token so a caller can report *where*.
#[must_use]
pub fn first_unclaimed<'a>(
    needs: &'a [CapabilityNeed],
    claimed: &ClaimedCapabilities,
) -> Option<&'a CapabilityNeed> {
    needs
        .iter()
        .find(|need| !claimed.claims_token(need.capability()))
}

/// **AC-494's resolver.** Put every need to the set; refuse on the first one it does not claim.
///
/// # Errors
///
/// [`StepError::Unsupported`] carrying the **token** of the first need the set does not claim.
/// There is no other error, no fallback and no generic variant: AC-494's AND is *"not a generic
/// error, not a retry, and never a `pass`"*, and the one refusal this function can produce is the
/// one the criterion names.
pub fn resolve_activity(
    needs: &[CapabilityNeed],
    claimed: &ClaimedCapabilities,
) -> Result<ResolutionReport, StepError> {
    // THE REFUSAL COMES FIRST AND IT IS THE ONLY ONE. There is no second error channel here to
    // widen into a generic failure, and no `Ok` path that a missing capability can reach — AC-494's
    // AND is *"not a generic error … and never a `pass`"*, and the shape of this function is what
    // makes both structural rather than conventional.
    if let Some(need) = first_unclaimed(needs, claimed) {
        return Err(unsupported(need.capability()));
    }
    Ok(ResolutionReport {
        needs_checked: needs.len(),
        claims_walked: claimed.len(),
        resolved: needs
            .iter()
            .map(|need| need.capability().to_owned())
            .collect(),
    })
}

/// **Resolve, and dispatch only if it resolved** — the ordering, owned by the kernel.
///
/// `dispatch` is a closure and not a port, so this function binds no `Harness`, takes no `Ports` and
/// is not generic over a port trait (**AC-480**). What it owns is the **order**: the resolution
/// happens first and the closure runs **only** on `Ok`, so *"never a `pass`"* is observable as a
/// dispatch count of zero at the caller's own counter rather than as a `match` the test wrote.
///
/// # Errors
///
/// [`StepError::Unsupported`], exactly as [`resolve_activity`] returns it, with `dispatch` never
/// called.
pub fn dispatch_if_resolved<T>(
    needs: &[CapabilityNeed],
    claimed: &ClaimedCapabilities,
    dispatch: impl FnOnce() -> T,
) -> Result<T, StepError> {
    // THE `?` IS THE CRITERION. The closure is not named before this line and cannot be reached
    // past it on a refusal, so *"never a `pass`"* is a dispatch count of zero at the caller's own
    // counter rather than a sentence in a comment.
    resolve_activity(needs, claimed)?;
    Ok(dispatch())
}
