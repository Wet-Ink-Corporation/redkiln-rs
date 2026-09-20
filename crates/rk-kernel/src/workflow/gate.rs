//! **The human gate's answer path, re-reach and expiry** — Step 7b-ii, `s7b-ii` §7b-ii.2, slices B and C.
//!
//! The fold in [`super::fsm`] is the only thing that moves the state, and it moves it from rows.
//! What this module owns is the two places a row is **decided** before it is written:
//!
//! | entry | criterion | what it decides | what it writes |
//! | --- | --- | --- | --- |
//! | [`reach`] | **AC-524** (PROVISIONAL, P8-B), ADR-0105 clause 10 | the record a reach of a gate step mints — reach `n + 1` after `n` prior reaches, over the tree and deadline the caller supplies **at this reach** | nothing; the caller appends [`crate::runtime::gate_opened_event`] |
//! | [`answer`] | **AC-246**, **AC-247**, **AC-249**, ADR-0017 | applied / void / refused-as-spent / refused-wrong-gate | the rows in [`AnswerOutcome::events`], or nothing on a refusal that is an error |
//! | [`expire`] | **AC-259**, **AC-272**, AC-256/AC-257's kernel half, ADR-0017, ADR-0105 clause 10 | not due / already settled / failed / escalated / auto-decided, at a journaled `now` | `gate_expired { action }`, plus the escalation's own `gate_opened` |
//! | [`drifted`], [`dispatches_across_open_gates`], [`prefix_rewrites_exceeding_one`], [`lint_expiry_policy_row`] | **AC-253**, **AC-243**, **AC-279**, **AC-260** | the drift predicate; two replay checkers; the expiry-policy lint | nothing — readers and checkers |
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
//! 2. **It resolves an expiry only at a journaled instant, never at a clock's.** [`expire`] takes
//!    `now` as data (AC-257: the expiry is observed at the next read, with no process alive at the
//!    deadline) and writes ADR-0017's terminal path — `fail`, `escalate` (a second gate of class
//!    `escalation`, AC-259), `auto-decide` — and an answer presented to an already-expired gate is
//!    [`GateError::Expired`], nothing written. What it does NOT do is choose the action: the
//!    policy is the caller's declaration, linted by [`lint_expiry_policy_row`] (AC-260), and
//!    `reconcile()`'s sweep over every past-deadline gate (AC-256) is `rk-git`'s / `rk-cli`'s.
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
    /// An `escalation`-class gate may not itself escalate (ADR-0105 clause 10; AC-259, AC-260).
    EscalateOnEscalation { token: Digest },
    /// The option, or the attachments presented with it, were refused by the question registry
    /// (ADR-0063's *"the kernel refuses to close the mark unless that attachment is supplied"*;
    /// ADR-0129 decisions 3 and 4). **The mark stays open.**
    OptionRefused(crate::workflow::question::OptionError),
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
            Self::OptionRefused(e) => write!(f, "the answer was refused: {e}"),
            Self::NeverOpened { token } => {
                write!(f, "no gate this journal opened carries {}", token.as_str())
            }
            Self::Expired { token } => write!(
                f,
                "gate {} expired; its disposition is the expiry policy's, not an answer",
                token.as_str()
            ),
            Self::EscalateOnEscalation { token } => write!(
                f,
                "escalation gate {} may not itself escalate; every chain terminates within one hop",
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
#[expect(
    clippy::too_many_arguments,
    reason = "the arity is the record's, not a design choice: §7d.15 fixes it as \"gains \n              three parameters (the one signature change, against today's five)\" — the \n              registry, the attachments and the answerer's report. Bundling them into a \n              struct would hide which three the record named and make the call sites stop \n              matching the sentence that authorises them"
)]
pub fn answer(
    state: &RunState,
    gate: &Digest,
    presented: &Digest,
    tree_hash: &str,
    verdict: &str,
    registry: &crate::workflow::question::QuestionRegistry,
    attachments: &[crate::workflow::question::Attachment],
    meta: &AnswerMeta,
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
        // ADR-0063's Decision, and it is the kernel's: *"the kernel refuses to close the mark
        // unless that attachment is supplied."* Checked BEFORE either disposition is returned and
        // AFTER the token is established as this gate's, so a wrong-gate answer is still refused as
        // a wrong gate and an inadmissible option never reaches a row. The refusal writes NOTHING:
        // the mark stays open, which is the whole content of the sentence.
        let question = registry.question_for(record.kind());
        registry
            .validate_attachments(question, verdict, attachments)
            .map_err(GateError::OptionRefused)?;

        if record.tree_hash() == tree_hash {
            return Ok(AnswerOutcome {
                disposition: Disposition::Applied,
                events: vec![gate_answered_event_with_meta(
                    presented, tree_hash, verdict, meta,
                )],
            });
        }
        return Ok(AnswerOutcome {
            disposition: Disposition::Void,
            events: vec![
                gate_answered_event_with_meta(presented, tree_hash, verdict, meta),
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

/// What the answerer reported about HOW they answered (ADR-0129 decision 1; RT-42).
///
/// Two members, and both are about the answer's provenance rather than its content: whether reading
/// the explanation sufficed to decide, and who decided. ADR-0129 decision 1 puts them on the
/// `gate_answered` payload, which is why they are added **here, kernel-side**, and never by a caller
/// editing the returned `Event` — a row the CLI could amend is a row the CLI could forge.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnswerMeta {
    /// Did reading the explanation suffice? `None` when the answerer did not say.
    pub explain_sufficiency: Option<String>,
    /// Who answered, as the repository records them.
    pub answerer: Option<String>,
}

/// The tier-1 `gate_answered` row **with the answerer's report** (ADR-0129 decision 1).
///
/// **A second constructor, and the first is left byte-for-byte alone — deliberately.** §7d.15 reads
/// as though `gate_answered_event` itself should widen, but that helper has a caller in a CLOSED
/// crate: `crates/rk-adapter-sqlite/tests/it/replay.rs` appends
/// `gate::gate_answered_event(open_gate.token(), TREE_A, "proceed")`, and §7d.15's own floor says
/// *"the four adapters are not edited … if a slice finds it must edit one, it stops"*. Widening the
/// arity would have broken `cargo test --workspace --features corpus` — a line Checkpoint 7b runs —
/// from inside this slice's own mandated re-run. So the row `answer` produces gains the members,
/// which is what §7d.14's red is actually about, and the three-argument helper keeps working.
///
/// **Absent members are OMITTED, not written as `null`.** A `gate_answered` row produced without an
/// `AnswerMeta` is byte-identical to one the three-argument constructor produces, so no recorded
/// journal moves. (Measured at this pin: no committed `.jsonl` carries a `gate_answered` row at
/// all, so nothing recorded moves either way — but the omission is what makes that still true the
/// day one does.)
#[must_use]
pub fn gate_answered_event_with_meta(
    token: &Digest,
    tree_hash: &str,
    verdict: &str,
    meta: &AnswerMeta,
) -> Event {
    let mut event = gate_answered_event(token, tree_hash, verdict);
    if let Some(payload) = event.payload.as_object_mut() {
        if let Some(sufficiency) = &meta.explain_sufficiency {
            payload.insert(
                "explain_sufficiency".to_owned(),
                serde_json::Value::String(sufficiency.clone()),
            );
        }
        if let Some(answerer) = &meta.answerer {
            payload.insert(
                "answerer".to_owned(),
                serde_json::Value::String(answerer.clone()),
            );
        }
    }
    event
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

// ------------------------------------------------------------------ slice C: expiry, drift, the replay checkers

/// ADR-0017's terminal path for an expired gate — *"fail / escalate / auto-decide, never a
/// hang"*. There is no fourth arm and no default: a gate class with no action declared is
/// AC-260's lint failure, never an implicit choice (AC-276 makes the choice a human's).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpiryAction {
    /// The run halts (`HaltReason::GateExpired`).
    Fail,
    /// A second, kernel-owned gate of class `escalation` is opened with its own
    /// `(token, tree_hash, deadline)` (AC-259; ADR-0022, ADR-0038). Its own expiry may not
    /// escalate again (ADR-0105 clause 10).
    Escalate { deadline: rk_types::time::Timestamp },
    /// The gate is answered with the policy's verdict, at the gate's own tree hash.
    AutoDecide { verdict: String },
}

impl ExpiryAction {
    /// The `action` member `gate_expired` carries — design §5:706, *"the terminal expiry action
    /// taken"*.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Fail => "fail",
            Self::Escalate { .. } => "escalate",
            Self::AutoDecide { .. } => "auto-decide",
        }
    }
}

/// How an expiry resolved. `NotDue` and `AlreadySettled` write nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// The deadline has not passed at the journaled instant supplied.
    NotDue,
    /// The gate is already answered, void or expired; there is nothing to resolve.
    AlreadySettled,
    Failed,
    Escalated {
        record: GateRecord,
    },
    AutoDecided,
}

/// The rows an expiry resolution writes, in the order they are appended.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpiryOutcome {
    pub resolution: Resolution,
    pub events: Vec<Event>,
}

/// The class every escalation gate carries (AC-259: *"fixed"*).
pub const ESCALATION_CLASS: &str = "escalation";

/// Resolve a gate's expiry **at a journaled instant** (`now` is data, never a clock; AC-257: the
/// expiry is observed at the next read, with no process alive at the deadline), under the
/// declared terminal path, over the tree hash the caller derived at this read.
///
/// - not yet due → [`Resolution::NotDue`], nothing written;
/// - already answered, void or expired → [`Resolution::AlreadySettled`], nothing written;
/// - `Fail` → one `gate_expired { action: "fail" }` row; the fold halts the run;
/// - `Escalate` → `gate_expired { action: "escalate" }` and a second `gate_opened` whose record
///   is a [`GateRecord`] like any other — class [`ESCALATION_CLASS`], the caller's tree hash, the
///   policy's deadline, its own reach — carrying **one** member beyond the record: `question`,
///   the id of the gate step it escalates (AC-259; ADR-0038: *"only a question id"*). An
///   escalation gate itself may not escalate: [`GateError::EscalateOnEscalation`];
/// - `AutoDecide` → one `gate_expired { action: "auto-decide", verdict }` row; the fold answers
///   the gate at its own tree hash.
///
/// # Errors
///
/// [`GateError::NeverOpened`] for a token this journal never opened;
/// [`GateError::EscalateOnEscalation`] as above.
pub fn expire(
    state: &RunState,
    token: &Digest,
    now: rk_types::time::Timestamp,
    action: &ExpiryAction,
    tree_hash: &str,
) -> Result<ExpiryOutcome, GateError> {
    let Some(record) = state.gate_record(token) else {
        return Err(GateError::NeverOpened {
            token: token.clone(),
        });
    };
    if state.is_answered(token) || state.is_void(token) || state.is_expired(token) {
        return Ok(ExpiryOutcome {
            resolution: Resolution::AlreadySettled,
            events: Vec::new(),
        });
    }
    if now.unix_seconds() < record.deadline().unix_seconds() {
        return Ok(ExpiryOutcome {
            resolution: Resolution::NotDue,
            events: Vec::new(),
        });
    }
    match action {
        ExpiryAction::Fail => Ok(ExpiryOutcome {
            resolution: Resolution::Failed,
            events: vec![gate_expired_event(token, action, None)],
        }),
        ExpiryAction::AutoDecide { verdict } => Ok(ExpiryOutcome {
            resolution: Resolution::AutoDecided,
            events: vec![gate_expired_event(token, action, Some(verdict))],
        }),
        ExpiryAction::Escalate { deadline } => {
            if record.kind() == ESCALATION_CLASS {
                return Err(GateError::EscalateOnEscalation {
                    token: token.clone(),
                });
            }
            let escalation = GateRecord::derive_reach(
                ESCALATION_CLASS,
                tree_hash,
                *deadline,
                state.reaches(ESCALATION_CLASS).saturating_add(1),
            )
            .map_err(|_| GateError::NeverOpened {
                token: token.clone(),
            })?;
            let mut opened = crate::runtime::gate_opened_event(&escalation);
            if let Some(obj) = opened.payload.as_object_mut() {
                obj.insert(
                    "question".to_owned(),
                    serde_json::Value::from(record.kind().to_owned()),
                );
            }
            Ok(ExpiryOutcome {
                resolution: Resolution::Escalated { record: escalation },
                events: vec![gate_expired_event(token, action, None), opened],
            })
        }
    }
}

/// The tier-1 `gate_expired` row (design §5:706): token, the terminal expiry action taken, and
/// the verdict when the action is `auto-decide`.
#[must_use]
pub fn gate_expired_event(token: &Digest, action: &ExpiryAction, verdict: Option<&str>) -> Event {
    let mut payload = serde_json::json!({ "action": action.as_str(), "token": token.as_str() });
    if let (Some(v), Some(obj)) = (verdict, payload.as_object_mut()) {
        obj.insert("verdict".to_owned(), serde_json::Value::from(v));
    }
    Event::new(EventKind::GateExpired, payload)
}

/// ADR-0123's `gate_resumed` row: the gate resumed from and the kernel's prefix-rewrite count.
/// The adapter's metric fields are **absent, not zero** — no member is written for them here.
#[must_use]
pub fn gate_resumed_event(token: &Digest, prefix_rewrites: u32) -> Event {
    Event::new(
        EventKind::GateResumed,
        serde_json::json!({ "prefix_rewrites": prefix_rewrites, "token": token.as_str() }),
    )
}

/// **AC-253 — drift is a comparison of tree hashes and nothing else.** Not mtimes, not elapsed
/// time, not commit counts, not the branch tip: the record's bound tree against the current one.
#[must_use]
pub fn drifted(record: &GateRecord, current_tree_hash: &str) -> bool {
    record.tree_hash() != current_tree_hash
}

/// A `dispatch` row that fell inside a gate's open window (AC-243).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchAcrossGate {
    pub gate: Digest,
    pub dispatch_index: usize,
}

fn payload_token(event: &Event) -> Option<Digest> {
    event
        .payload
        .get("token")
        .and_then(serde_json::Value::as_str)
        .and_then(|s| Digest::from_hex(s).ok())
}

/// **AC-243's replay assertion**: every `dispatch` row that falls between a runtime-shaped
/// `gate_opened` and that gate's terminal event (`gate_answered`, `gate_answer_void`,
/// `gate_expired`) in the same item's journal. Empty is the invariant.
#[must_use]
pub fn dispatches_across_open_gates(events: &[Event]) -> Vec<DispatchAcrossGate> {
    let mut open: Vec<Digest> = Vec::new();
    let mut found: Vec<DispatchAcrossGate> = Vec::new();
    for (i, e) in events.iter().enumerate() {
        match e.kind {
            EventKind::GateOpened => {
                let runtime_shaped = e.payload.get("tree_hash").is_some();
                if runtime_shaped && let Some(t) = payload_token(e) {
                    open.push(t);
                }
            }
            EventKind::GateAnswered | EventKind::GateAnswerVoid | EventKind::GateExpired => {
                if let Some(t) = payload_token(e) {
                    open.retain(|o| *o != t);
                }
            }
            EventKind::Dispatch => {
                for g in &open {
                    found.push(DispatchAcrossGate {
                        gate: g.clone(),
                        dispatch_index: i,
                    });
                }
            }
            _ => {}
        }
    }
    found
}

/// **AC-279's count assertion**: the indices of `gate_resumed` rows whose `prefix_rewrites` is
/// above one, or not a count at all. Empty is the invariant.
#[must_use]
pub fn prefix_rewrites_exceeding_one(events: &[Event]) -> Vec<usize> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::GateResumed)
        .filter(|(_, e)| {
            e.payload
                .get("prefix_rewrites")
                .and_then(serde_json::Value::as_u64)
                .is_none_or(|n| n > 1)
        })
        .map(|(i, _)| i)
        .collect()
}

/// AC-260's lint failures over a gate-class policy row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpiryPolicyLint {
    /// *"there is no implicit default expiry action"*.
    NoActionDeclared { class: String },
    /// *"no escalate-on-escalate (ADR-0105 clause 10)"*.
    EscalateOnEscalation,
    /// Not one of ADR-0017's three terminal paths.
    UnknownAction { class: String, action: String },
}

/// **AC-260 — the expiry-policy lint, one row at a time.** `class` is the gate class the row
/// declares; `action` is its declared expiry action, `None` when the row declares none.
///
/// # Errors
///
/// [`ExpiryPolicyLint`] naming the failure.
pub fn lint_expiry_policy_row(class: &str, action: Option<&str>) -> Result<(), ExpiryPolicyLint> {
    let Some(action) = action else {
        return Err(ExpiryPolicyLint::NoActionDeclared {
            class: class.to_owned(),
        });
    };
    match action {
        "escalate" if class == ESCALATION_CLASS => Err(ExpiryPolicyLint::EscalateOnEscalation),
        "fail" | "escalate" | "auto-decide" => Ok(()),
        other => Err(ExpiryPolicyLint::UnknownAction {
            class: class.to_owned(),
            action: other.to_owned(),
        }),
    }
}
