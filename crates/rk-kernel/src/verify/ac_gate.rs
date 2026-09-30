//! **AC-357's gate, minted by the kernel when the verify pass's stage exit is refused, and AC-359's
//! drift** — `s8g` §8g.5 and §8g.6 (R1 = ADR-0191 decision 10, proposed). *(Added 2026-09-29,
//! `s8g` build, package S8G-P6-MUTATION-RESUME-DRIFT.)*
//!
//! - **One gate class, [`AC_GATE_KIND`], answered by the question of the same id.** A catalog
//!   `gate` step's kind is its `question` (ADR-0145), and no catalog step opens this one: the kernel
//!   mints it here, for the first criterion the ledger leaves neither `pinned` nor
//!   `human_answered` (AC-346's refusal names every such id; the gate asks about one at a time).
//!   The spelling is the one `s8h`'s `MINTED_CLASSES` takes (`s8g` §8g.6).
//! - **AC-357's machine-checkable part is the payload**: the gate node carries the criterion's text
//!   (`criterion`), the assertion source (`assertion`) and the suite result (`suite_result`), beside
//!   the runtime shape every `gate_opened` row carries (`deadline`, `kind`, `reach`, `token`,
//!   `tree_hash`) and the criterion's id (`ac`).
//! - **AC-359: an answer against a drifted tree is void, and the gate re-opens with the drift
//!   attached.** The void itself is `workflow::gate::answer`'s (the tree hash the answer presents
//!   against the one the gate bound, AC-249). What this module adds is the re-open: when the
//!   latest gate of the class is void, the next one is minted over the CURRENT tree, one reach on
//!   (AC-252's shape), carrying `drift` = the void row's two tree hashes and its token.
//! - **Only an applied `discriminates` answer that names who answered is an answer for the
//!   ledger** ([`answers`]): a void answer is not one (AC-359: *"the verify evidence recorded
//!   before the drift is not carried across"*), and neither is `does-not-discriminate`. A
//!   criterion answered `does-not-discriminate` is held by name ([`Opening::Held`]) rather than
//!   asked again.
//!
//! Nothing here reads a clock or a tree: the tree hash and the deadline arrive as values.

use rk_store::canon::CanonError;
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::text::Text;
use rk_types::time::Timestamp;

use super::ledger::HumanAnswer;
use crate::workflow::fsm::{GateRecord, RunState};

/// The gate class, and the question id that answers it (`.redkiln/processes/`, `s8d`'s registry).
pub const AC_GATE_KIND: &str = "verify.ac-discriminates";

/// The option that answers the criterion: the named assertion discriminates it.
pub const DISCRIMINATES: &str = "discriminates";

/// The option that does not: the criterion stays unpinned, and it is held by name.
pub const DOES_NOT_DISCRIMINATE: &str = "does-not-discriminate";

/// The three facts AC-357 puts on the gate node, and the criterion they are about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcFacts<'a> {
    /// The criterion's id.
    pub ac: &'a str,
    /// The criterion's text.
    pub criterion: &'a str,
    /// The assertion source the question is about (the criterion's authored verification, or the
    /// named absence).
    pub assertion: &'a str,
    /// What the suite reported: the whole suite's counts, and the criterion's own mutation run.
    pub suite_result: serde_json::Value,
}

/// What [`open`] decided for the criterion.
#[derive(Debug, Clone, PartialEq)]
pub enum Opening {
    /// A fresh gate: the `gate_opened` row to append (its `Gate:` mark first, through the caller's
    /// tier-1 boundary).
    Open(Event),
    /// The latest gate of the class is still open and unanswered.
    Awaiting(Digest),
    /// The latest gate asked about THIS criterion and was answered with a verdict that is not an
    /// answer for the ledger: held, not asked again.
    Held { token: Digest, verdict: String },
    /// The latest gate expired: its disposition is the expiry policy's, not a re-open.
    Expired(Digest),
}

/// The `ac` a `gate_opened` row of this class carries, by token.
fn ac_of(events: &[Event], token: &Digest) -> Option<String> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| {
            e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token.as_str())
        })
        .find_map(|e| {
            e.payload
                .get("ac")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
}

/// The `gate_answer_void` row that voided `token`, as the drift the re-opened gate carries.
fn drift_of(events: &[Event], token: &Digest) -> Option<serde_json::Value> {
    events
        .iter()
        .rev()
        .filter(|e| e.kind == EventKind::GateAnswerVoid)
        .find(|e| {
            e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token.as_str())
        })
        .map(|e| {
            serde_json::json!({
                "bound_tree_hash": e.payload.get("bound_tree_hash").cloned(),
                "observed_tree_hash": e.payload.get("observed_tree_hash").cloned(),
                "voided": token.as_str(),
            })
        })
}

/// **Decide the gate for one criterion**: a fresh one (the class never opened; its latest gate was
/// void, and then carrying the drift; or its latest gate answered ANOTHER criterion), or why not.
///
/// # Errors
///
/// [`CanonError`] if the token's canonical form cannot be taken.
pub fn open(
    state: &RunState,
    events: &[Event],
    facts: &AcFacts<'_>,
    tree_hash: &str,
    deadline: Timestamp,
) -> Result<Opening, CanonError> {
    let reaches = state.reaches(AC_GATE_KIND);
    let mut drift = None;
    if let Some(latest) = state.latest_gate(AC_GATE_KIND) {
        if state.is_void(latest) {
            drift = drift_of(events, latest);
        } else if state.is_answered(latest) {
            if ac_of(events, latest).as_deref() == Some(facts.ac) {
                let verdict = state
                    .answered_gates()
                    .find(|a| a.token() == latest)
                    .map(|a| a.verdict().to_owned())
                    .unwrap_or_else(|| "unrecorded".to_owned());
                return Ok(Opening::Held {
                    token: latest.clone(),
                    verdict,
                });
            }
        } else if state.is_expired(latest) {
            return Ok(Opening::Expired(latest.clone()));
        } else {
            return Ok(Opening::Awaiting(latest.clone()));
        }
    }
    let record =
        GateRecord::derive_reach(AC_GATE_KIND, tree_hash, deadline, reaches.saturating_add(1))?;
    let mut payload = serde_json::json!({
        "ac": facts.ac,
        "assertion": facts.assertion,
        "criterion": facts.criterion,
        "deadline": record.deadline().unix_seconds(),
        "kind": record.kind(),
        "reach": record.reach(),
        "suite_result": facts.suite_result,
        "token": record.token().as_str(),
        "tree_hash": record.tree_hash(),
    });
    if let (Some(drift), Some(members)) = (drift, payload.as_object_mut()) {
        members.insert("drift".to_owned(), drift);
    }
    Ok(Opening::Open(Event::new(EventKind::GateOpened, payload)))
}

/// **The human answers the ledger reads** (AC-357's recorded answer; AC-346's other exit): for each
/// gate of this class, its applied answer when the verdict is [`DISCRIMINATES`] and the row names
/// who answered. A voided token contributes nothing (AC-359). `at` is the journaled instant the
/// caller reads the answers at: the `gate_answered` row carries no time of its own.
#[must_use]
pub fn answers(events: &[Event], at: Timestamp) -> Vec<HumanAnswer> {
    let voided = |token: &str| {
        events.iter().any(|e| {
            e.kind == EventKind::GateAnswerVoid
                && e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token)
        })
    };
    let mut out: Vec<HumanAnswer> = Vec::new();
    for opened in events.iter().filter(|e| {
        e.kind == EventKind::GateOpened
            && e.payload.get("kind").and_then(serde_json::Value::as_str) == Some(AC_GATE_KIND)
    }) {
        let (Some(token), Some(ac)) = (
            opened
                .payload
                .get("token")
                .and_then(serde_json::Value::as_str),
            opened.payload.get("ac").and_then(serde_json::Value::as_str),
        ) else {
            continue;
        };
        if voided(token) {
            continue;
        }
        let Some(answered) = events.iter().rev().find(|e| {
            e.kind == EventKind::GateAnswered
                && e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token)
        }) else {
            continue;
        };
        let verdict = answered
            .payload
            .get("verdict")
            .and_then(serde_json::Value::as_str);
        let by = answered
            .payload
            .get("answerer")
            .and_then(serde_json::Value::as_str)
            .filter(|b| !b.trim().is_empty());
        if let (Some(DISCRIMINATES), Some(by)) = (verdict, by) {
            out.push(HumanAnswer {
                ac: ac.to_owned(),
                by: by.to_owned(),
                at,
                reason: Text::new(format!(
                    "answered `{DISCRIMINATES}` at gate {token} by {by}"
                )),
            });
        }
    }
    out
}
