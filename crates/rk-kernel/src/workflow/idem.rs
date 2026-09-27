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

// ---- R1: the STEP-GRAIN commit key (s8d §8d.2; D21-42 3b; D21-43 N15) ----------------------------
//
// *(Added 2026-09-26, `s8d` build, package P3; record R1, proposed.)* **A separate commit key, and
// `IdempotencyKey` above keeps its three parts.** D21-43 N15: *"A separate commit key. ADR-0097's
// memo key is untouched."* D21-42 3b: *"`(item, activity@version, step_id, iteration path)`. The
// iteration path is the stack of for_each index, repeat count and loop-back count."* The attempt
// number is not part of it: a committed step is not re-dispatched under a higher attempt, exactly
// as a committed activation was not.

/// One frame of an iteration path — design's three loop forms, and no fourth (D21-42 3b).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Frame {
    /// A `for_each` element's index.
    ForEach(u32),
    /// A `repeat` pass's count.
    Repeat(u32),
    /// An `on_false = { to, max }` loop-back's count.
    LoopBack(u32),
}

impl Frame {
    /// The frame's journal token, `"<form>:<n>"`.
    #[must_use]
    pub fn token(self) -> String {
        match self {
            Self::ForEach(n) => format!("for_each:{n}"),
            Self::Repeat(n) => format!("repeat:{n}"),
            Self::LoopBack(n) => format!("loop_back:{n}"),
        }
    }

    /// Read one token back. Anything but the three forms and a `u32` is `None`, which the fold
    /// refuses naming the row — never a frame guessed from a malformed one.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        let (form, n) = token.split_once(':')?;
        let n: u32 = n.parse().ok()?;
        match form {
            "for_each" => Some(Self::ForEach(n)),
            "repeat" => Some(Self::Repeat(n)),
            "loop_back" => Some(Self::LoopBack(n)),
            _ => None,
        }
    }

    /// The frame's counter.
    #[must_use]
    pub const fn counter(self) -> u32 {
        match self {
            Self::ForEach(n) | Self::Repeat(n) | Self::LoopBack(n) => n,
        }
    }
}

/// **The iteration path** — a stack of frames, outermost first, and EMPTY outside any loop.
///
/// Two iterations of one step never share a path, so they never share a [`CommitKey`] (AC-017's
/// *"the loop iteration counter is part of the step key"*, carried to the commit).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IterationPath(Vec<Frame>);

impl IterationPath {
    /// The empty path: a step outside any loop.
    #[must_use]
    pub const fn root() -> Self {
        Self(Vec::new())
    }

    /// This path with one more (innermost) frame.
    #[must_use]
    pub fn pushed(&self, frame: Frame) -> Self {
        let mut frames = self.0.clone();
        frames.push(frame);
        Self(frames)
    }

    /// The frames, outermost first.
    #[must_use]
    pub fn frames(&self) -> &[Frame] {
        &self.0
    }

    /// **ADR-0097 decision 2's component 7**, *"the loop iteration counter (0 outside a loop)"*, fed
    /// as that record writes it: the INNERMOST frame's counter, and `0` on the empty path.
    ///
    /// **Residual, named (§8d.1 S5):** two nested paths sharing an innermost counter share this
    /// component, so the memo key alone cannot tell them apart. The commit key can.
    #[must_use]
    pub fn loop_iteration(&self) -> u32 {
        self.0.last().map_or(0, |f| f.counter())
    }

    /// The journal form: an array of frame tokens, outermost first; `[]` on the empty path.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::Value::Array(
            self.0
                .iter()
                .map(|f| serde_json::Value::String(f.token()))
                .collect(),
        )
    }

    /// Read the journal form back, or `None` for anything that is not an array of frame tokens.
    #[must_use]
    pub fn from_json(value: &serde_json::Value) -> Option<Self> {
        value
            .as_array()?
            .iter()
            .map(|v| v.as_str().and_then(Frame::from_token))
            .collect::<Option<Vec<Frame>>>()
            .map(Self)
    }
}

impl fmt::Display for IterationPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str("root");
        }
        let tokens: Vec<String> = self.0.iter().map(|fr| fr.token()).collect();
        f.write_str(&tokens.join("/"))
    }
}

/// **R1's commit key: `(item, activity@version, step_id, iteration path)`** (D21-42 3b).
///
/// Separate from [`IdempotencyKey`] (D21-43 N15), which keeps its three parts and never a fourth
/// (ADR-0097's forbids). Each `agent` step activation — and each `side_effecting` `tool` step
/// activation — commits under its own, as one `activity_committed` row carrying `step_id` and
/// `iteration_path`; `crate::workflow::fsm::RunState::is_step_committed` folds it back.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommitKey {
    /// The item.
    pub item: String,
    /// `activity_id@version`.
    pub activity_id_at_version: String,
    /// The `[[step]]` id — never the DAG node id.
    pub step_id: String,
    /// Where in the activation's loops this step activation sits.
    pub iteration_path: IterationPath,
}
