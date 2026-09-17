//! **The human gate's answer path and re-reach** — Step 7b-ii, `s7b-ii` §7b-ii.2, slice B.
//!
//! The fold in [`super::fsm`] is the only thing that moves the state, and it moves it from rows.
//! What this module owns is the two places a row is **decided** before it is written:
//!
//! | entry | criterion | what it decides | what it writes |
//! | --- | --- | --- | --- |
//! | [`reach`] | **AC-524** (PROVISIONAL, P8-B), ADR-0105 clause 10 | the record a reach of a gate step mints — reach `n + 1` after `n` prior reaches, over the tree and deadline the caller supplies **at this reach** | nothing; the caller appends [`crate::runtime::gate_opened_event`] |
//! | [`answer`] | **AC-246**, **AC-247**, **AC-249**, ADR-0017 | applied / void / refused-as-spent / refused-wrong-gate | the rows in [`AnswerOutcome::events`], or nothing on a refusal that is an error |
//!
//! # What is decided here and what is derived from the journal
//!
//! **The mechanism of every refusal is the folded state, never a row this module wrote.** A spent
//! token is one the fold's answered or void set already holds; a wrong-gate token is one that is
//! not the gate's own; the tree-hash comparison is [`GateRecord::tree_hash`] against the answer's.
//! The rows this module returns are the *records* of those decisions — `gate_answered` (tier 1),
//! `gate_answer_void` (AC-249's, design §5:709) and `gate_answer_spent` (ADR-0123's) — and a
//! replay that drops every tier-2 row decides identically, which is ADR-0109 decision 2 holding.
//!
//! # 🛑 Three things this module does NOT do, each stated so the absence is visible
//!
//! 1. **It does not drive a loop.** [`reach`] is the entry a loop executor calls when an `on_false
//!    { to, max }` re-enters a gate step. Step 7b parsed and linted that form
//!    (`crate::predicate::on_false`) and built no executor; nothing in `src/` re-enters an earlier
//!    step yet, and `tests/it/gates.rs::ac524_fresh_record_on_rereach` says in its header that the
//!    test performs the re-reach the executor will.
//! 2. **It does not resolve an expiry.** An answer presented to an **expired** gate is
//!    [`GateError::Expired`] — nothing written — because the disposition of an expired gate is the
//!    expiry policy's terminal path (ADR-0017: fail / escalate / auto-decide; AC-257, AC-259),
//!    which is slice C's, and a raced raw `gate_answered` row on an expired token is already the
//!    fold's D-5 case (void, and the fold continues). Refusing here decides nothing about that.
//! 3. **It computes no tree hash.** `tree_hash` arrives as a value on both sides (ADR-0105 clause
//!    4; `rk_git::tree::tree_hash` is the one derivation), and [`FsmContext`] carries the one the
//!    caller derived at this reach.
//!
//! # Time and blocking
//!
//! A child of [`super`], so its `#![deny(clippy::disallowed_methods)]` reaches here: no clock, no
//! sleep, no blocking receive on the gate path (AC-242's syntactic half — the lint is the parent
//! module's attribute, and `tests/fixtures/lint-controls/` is where a positive control would sit).
//! The deadline is a journaled value on the context, and this module reads it and never makes one.

use rk_store::canon::CanonError;
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use super::fsm::{FsmContext, GateRecord, RunState};

/// How an answer resolved. Three dispositions and no fourth: a refusal that names the wrong gate
/// or a gate this journal never opened is a [`GateError`], because nothing is written for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// The token was open, the tree hash matched: the answer applies (tier-1 `gate_answered`).
    Applied,
    /// The token was open, the tree hash did not match: **void, not reconciled** (ADR-0017;
    /// AC-249). Both the answer and its voiding are recorded.
    Void,
    /// The token was already consumed — by an applied answer or by a void — and the second
    /// presentation is **refused as spent** (AC-246), recorded as `gate_answer_spent` (ADR-0123).
    RefusedSpent,
}

/// The rows the answer path writes for one presented answer, in the order they are appended.
#[derive(Debug, Clone, PartialEq)]
pub struct AnswerOutcome {
    pub disposition: Disposition,
    pub events: Vec<Event>,
}

/// A refusal that writes nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    /// The answer presents a token that is not the gate's own (AC-247): refused, and the gate
    /// remains open. Both tokens are named so the refusal can be read.
    WrongGate { gate: Digest, presented: Digest },
    /// No gate this journal opened carries the token.
    NeverOpened { token: Digest },
    /// The gate expired; its disposition is the expiry policy's terminal path, not an answer.
    Expired { token: Digest },
}

impl std::fmt::Display for GateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongGate { gate, presented } => write!(
                f,
                "the answer presents token {} against gate {}; refused, the gate remains open",
                presented.as_str(),
                gate.as_str()
            ),
            Self::NeverOpened { token } => {
                write!(f, "no gate this journal opened carries {}", token.as_str())
            }
            Self::Expired { token } => write!(
                f,
                "gate {} expired; its disposition is the expiry policy's, not an answer",
                token.as_str()
            ),
        }
    }
}

impl std::error::Error for GateError {}

/// Mint the record the **next reach** of `kind` writes: reach `n + 1` where `n` is the number of
/// runtime-shaped `gate_opened` rows the fold has seen for this gate step
/// ([`RunState::reaches`]), over the tree hash and deadline the caller derived **at this reach**.
///
/// The reach is a member of the token's digest ([`GateRecord::derive_reach`]), so a loop-back
/// that re-reaches an answered gate over an unchanged tree inside one deadline window still mints
/// a fresh `(token, tree_hash, deadline)` — the earlier answer is not replayed and the earlier
/// token stays spent (AC-524's THEN; AC-246).
///
/// # Errors
///
/// [`CanonError`] as [`GateRecord::derive_reach`].
pub fn reach(state: &RunState, kind: &str, ctx: &FsmContext<'_>) -> Result<GateRecord, CanonError> {
    GateRecord::derive_reach(
        kind,
        ctx.tree_hash(),
        ctx.deadline(),
        state.reaches(kind).saturating_add(1),
    )
}

/// Decide an answer presented **against `gate`** and carrying `presented`, at the item's current
/// `tree_hash`, and return the rows that record the decision.
///
/// The order of the checks is the order of the guarantees:
///
/// 1. `presented != gate` → [`GateError::WrongGate`], nothing written, the gate remains open
///    (AC-247: a token is the identity of one gate instance).
/// 2. the token is answered or void → [`Disposition::RefusedSpent`], one `gate_answer_spent` row
///    (AC-246: single-use; a void token is terminal, ADR-0017).
/// 3. the token is open → [`Disposition::Applied`] on a matching tree hash, else
///    [`Disposition::Void`] with `gate_answered` **and** `gate_answer_void` (AC-249).
/// 4. the token is expired → [`GateError::Expired`], nothing written (slice C's terminal path).
/// 5. otherwise → [`GateError::NeverOpened`].
///
/// # Errors
///
/// [`GateError`] for the refusals that write nothing, as above.
pub fn answer(
    state: &RunState,
    gate: &Digest,
    presented: &Digest,
    tree_hash: &str,
    verdict: &str,
) -> Result<AnswerOutcome, GateError> {
    if presented != gate {
        return Err(GateError::WrongGate {
            gate: gate.clone(),
            presented: presented.clone(),
        });
    }
    if state.is_answered(presented) || state.is_void(presented) {
        return Ok(AnswerOutcome {
            disposition: Disposition::RefusedSpent,
            events: vec![gate_answer_spent_event(presented, tree_hash)],
        });
    }
    if let Some(record) = state.gate_record(presented) {
        if record.tree_hash() == tree_hash {
            return Ok(AnswerOutcome {
                disposition: Disposition::Applied,
                events: vec![gate_answered_event(presented, tree_hash, verdict)],
            });
        }
        return Ok(AnswerOutcome {
            disposition: Disposition::Void,
            events: vec![
                gate_answered_event(presented, tree_hash, verdict),
                gate_answer_void_event(presented, record.tree_hash(), tree_hash),
            ],
        });
    }
    if state.is_expired(presented) {
        return Err(GateError::Expired {
            token: presented.clone(),
        });
    }
    Err(GateError::NeverOpened {
        token: presented.clone(),
    })
}

/// The tier-1 `gate_answered` row (design §5:705): token, the answer, the tree hash at answer.
#[must_use]
pub fn gate_answered_event(token: &Digest, tree_hash: &str, verdict: &str) -> Event {
    Event::new(
        EventKind::GateAnswered,
        serde_json::json!({ "token": token.as_str(), "tree_hash": tree_hash, "verdict": verdict }),
    )
}

/// AC-249's `gate_answer_void` row (design §5:709): token, bound tree hash, observed tree hash.
#[must_use]
pub fn gate_answer_void_event(
    token: &Digest,
    bound_tree_hash: &str,
    observed_tree_hash: &str,
) -> Event {
    Event::new(
        EventKind::GateAnswerVoid,
        serde_json::json!({
            "bound_tree_hash": bound_tree_hash,
            "observed_tree_hash": observed_tree_hash,
            "token": token.as_str(),
        }),
    )
}

/// ADR-0123's `gate_answer_spent` row: the refused answer's token and tree hash.
#[must_use]
pub fn gate_answer_spent_event(token: &Digest, tree_hash: &str) -> Event {
    Event::new(
        EventKind::GateAnswerSpent,
        serde_json::json!({ "token": token.as_str(), "tree_hash": tree_hash }),
    )
}
