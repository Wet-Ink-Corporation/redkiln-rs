//! **The idempotency key, and the dispatch that cannot be constructed without one** — AC-010,
//! and the type AC-011 and AC-012 count over.
//!
//! > **AC-010** — GIVEN an activity about to be executed / WHEN a dispatch is constructed / THEN it
//! > cannot be constructed without an `(item, activity_id@version, attempt)` idempotency key, and a
//! > malformed key (empty `item`, empty `activity_id`, missing `@version`) is **refused before any
//! > process is spawned**.
//! > *Verified by:* a `trybuild` test that a `dispatch` call without the key does not compile (the
//! > port takes it by value, not `Option`); plus a unit test on the key constructor rejecting empty
//! > components.
//!
//! # Why this module is a VALIDATOR and not a type
//!
//! `rk_store::memo::IdempotencyKey` is the type, and ADR-0119 clause 1's sibling rule applies: this
//! crate assembles `rk-store`'s value and never re-derives its digest. But that struct's three
//! fields are **`pub`** (`crates/rk-store/src/memo.rs`:112-115), so `IdempotencyKey { item:
//! String::new(), .. }` is a legal expression and every one of AC-010's three malformed shapes is
//! constructible today. [`checked_key`] is the function that refuses them, and it is the whole of
//! what this module adds over the store's type.
//!
//! **`rk-store` is not edited to make this a type fact**, and the reason is `00-preamble.md`
//! :232-237: Checkpoints 5 and 6 are recorded passed over `rk-types` and `rk-store`, and an edit to
//! either after the fact makes both rows provisional. Slice 4g took the same decision for
//! `StepKeyComponents` and stated it in `super::memo`'s header.
//!
//! # 🛑 The three refusals are the criterion's own enumeration, and there is no fourth
//!
//! Empty `item`; empty `activity_id`; an `activity_id` carrying no `@version`. The criterion names
//! those three and this module refuses exactly those three. **`attempt` is deliberately
//! unvalidated**: AC-010's *Note* says the key *"is **not** required to be globally fresh — re-entry
//! under AC-013 deliberately reuses `item`, and uniqueness is asserted only over the triple
//! (AC-011)"*, and no record gives `attempt` a floor. A refusal of `attempt = 0` would be this
//! stage inventing a fourth malformation, which `00-preamble.md`:55 forbids; the FSM's
//! `RunState::next_attempt` starts at 1 and that is where the numbering lives.
//!
//! # 🛑 A READING, stated because no record states it: where `@version` is required to sit
//!
//! ADR-0097 decision 2 fixes the key as `(item, activity_id@version, attempt)` and spells the
//! middle part with an `@`. No record says whether the version may be empty, whether more than one
//! `@` is admissible, or what the version's shape is. The reading taken here is the narrowest that
//! makes the criterion's third case a refusal at all: **exactly one `@`, with a non-empty id before
//! it and a non-empty version after it**. A second `@` is refused as `MissingAtVersion` carrying the
//! whole offending string, because a two-`@` id has no unambiguous split and guessing one would put
//! two different activities under one key. A record fixing the version's grammar would narrow this;
//! nothing here widens.
//!
//! # 🛑 WHAT IS MEASURED AND WHAT IS NOT
//!
//! | clause | state | where |
//! | --- | --- | --- |
//! | the three malformed shapes are refused | **measured** | `ac010_malformed_key_is_refused_before_spawn` |
//! | *"before any process is spawned"* | **measured**, as a dispatch count of 0 on a counting `Harness` fake | same test |
//! | *"cannot be constructed without the key"* | 🛑 **ARMED — the register's clause is about the PORT, and this is a kernel-local analogue.** AC-010's *Verified by* names a dispatch that cannot be built without the triple; `rk_ports::harness::Harness::dispatch` takes `(spec, store)` and **no key** (`crates/rk-ports/src/harness.rs`:37), and [`PlannedDispatch`] has **zero construction sites in `src/`** [measured 2026-09-14, `grep -rn "PlannedDispatch::" crates/rk-kernel/src/` -> no output]. What the case pins is that *this crate's* planned-dispatch value cannot be built without the key — real, and **not** the port-side clause. Editing the port signature is refused here: ADR-0099 clause 1 fixes it and moving it would disturb AC-481/AC-483/AC-499 after Checkpoint 7a passed. **A record carrying the key across the port boundary is OWED.** *(Corrected 2026-09-14: this row read "measured as a type fact" with no ARMED marker.)* | `tests/ui/ac010_dispatch_without_key.rs` |
//! | the `@version` half has a **producer** | **ARMED** | the DAG node carries a bare `activity_id` (`super::fsm`'s header says so); all 102 shipped activities declare `version` and `crate::catalog::Activity::id_at_version` reads it, but nothing joins the two. A record is owed |

use core::fmt;

use rk_store::memo::IdempotencyKey;
use rk_types::activity::ActivitySpec;

use super::degrade::Degraded;

/// Why a key was refused. **AC-010's three named malformations and no fourth.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
    /// `item` was empty. The criterion's first case.
    EmptyItem,
    /// The activity id was empty — either the whole string, or the part before the `@`.
    EmptyActivityId,
    /// No `@version`: no `@` at all, nothing after it, or more than one.
    MissingAtVersion {
        /// The offending string, verbatim, so a refusal names what it refused.
        got: String,
    },
}

impl KeyError {
    /// A stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyItem => "empty_item",
            Self::EmptyActivityId => "empty_activity_id",
            Self::MissingAtVersion { .. } => "missing_at_version",
        }
    }
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyItem => write!(f, "the idempotency key's `item` is empty (AC-010)"),
            Self::EmptyActivityId => {
                write!(f, "the idempotency key's activity id is empty (AC-010)")
            }
            Self::MissingAtVersion { got } => write!(
                f,
                "`{got}` is not an `activity_id@version`: exactly one `@` is required, with a \
                 non-empty id before it and a non-empty version after it (ADR-0097 decision 2)"
            ),
        }
    }
}

impl std::error::Error for KeyError {}

/// **AC-010's constructor half.** The one door a key reaches a dispatch through.
///
/// # Errors
///
/// One [`KeyError`] per named malformation. The refusal happens **here**, before the `Harness` port
/// is reached, which is what makes *"refused before any process is spawned"* a countable assertion
/// rather than a claim about ordering inside a function body.
pub fn checked_key(
    item: &str,
    activity_id_at_version: &str,
    attempt: u32,
) -> Result<IdempotencyKey, KeyError> {
    if item.is_empty() {
        return Err(KeyError::EmptyItem);
    }
    if activity_id_at_version.is_empty() {
        return Err(KeyError::EmptyActivityId);
    }
    let mut parts = activity_id_at_version.split('@');
    // `split` always yields at least one piece, so the first `next` cannot be `None`.
    let id = parts.next().unwrap_or_default();
    let version = parts.next();
    let extra = parts.next();
    // THE EMPTY ID IS ITS OWN REFUSAL, and the order matters: `@1` has a version and no id, and
    // reporting it as `MissingAtVersion` would name the half that is present.
    if id.is_empty() {
        return Err(KeyError::EmptyActivityId);
    }
    match (version, extra) {
        (Some(version), None) if !version.is_empty() => Ok(IdempotencyKey {
            item: item.to_owned(),
            activity_id_at_version: activity_id_at_version.to_owned(),
            attempt,
        }),
        // No `@`; an `@` with nothing after it; or a second `@`, which has no unambiguous split.
        _ => Err(KeyError::MissingAtVersion {
            got: activity_id_at_version.to_owned(),
        }),
    }
}

/// **AC-010's compile-time half.** A dispatch that cannot exist without its key.
///
/// The key is a by-value field and not an `Option`, the fields are private, and both constructors
/// take the key by value — so there is no expression that produces a `PlannedDispatch` without one.
/// The criterion's own words: *"the port takes it by value, not `Option`"*.
///
/// `degraded` **is** an `Option`, and the asymmetry is the point: AC-036's `SubstitutionOutcome`
/// has a branch that mints no record at all, so most dispatches carry none. A dispatch that *is*
/// substituted carries [`Degraded`] **by value** through [`Self::substituted`], which is AC-037's
/// third instrument — and `super::degrade::SubstitutedDispatch` is the type that states it without
/// an `Option` anywhere.
#[derive(Debug, Clone, PartialEq)]
pub struct PlannedDispatch {
    key: IdempotencyKey,
    spec: ActivitySpec,
    degraded: Option<Degraded>,
}

impl PlannedDispatch {
    /// One planned dispatch. The key is taken **by value**.
    #[must_use]
    pub const fn new(key: IdempotencyKey, spec: ActivitySpec) -> Self {
        Self {
            key,
            spec,
            degraded: None,
        }
    }

    /// One planned dispatch that runs at a substituted tier, carrying AC-037's record by value.
    #[must_use]
    pub const fn substituted(key: IdempotencyKey, spec: ActivitySpec, degraded: Degraded) -> Self {
        Self {
            key,
            spec,
            degraded: Some(degraded),
        }
    }

    #[must_use]
    pub const fn key(&self) -> &IdempotencyKey {
        &self.key
    }

    #[must_use]
    pub const fn spec(&self) -> &ActivitySpec {
        &self.spec
    }

    #[must_use]
    pub const fn degraded(&self) -> Option<&Degraded> {
        self.degraded.as_ref()
    }
}
