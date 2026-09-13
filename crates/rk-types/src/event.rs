//! The event arm each `StepError` variant maps to (AC-360).
//!
//! The arms are ADR-0095 decision 2's and ADR-0112's tables: `step_failed { class }`,
//! `step_cancelled`, `rate_limited`, `step_refused { class }`. Both class enums are CLOSED — a
//! class not in these tables is a schema change, reviewed as one.

use serde::{Deserialize, Serialize};

/// A failure class on `step_failed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    Transport,
    Schema,
    Timeout,
    NonZero,
    Platform,
}

/// A refusal class on `step_refused`. A refusal is a configuration or billing state, never a
/// judgement — nothing here ever reaches the gate as a verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RefusalClass {
    NotAdopted,
    NotIsolated,
    Unsupported,
    Overage,
}

/// The event the kernel appends for a failed step. Exactly one per `StepError` variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorEvent {
    StepFailed {
        class: FailureClass,
        status: Option<u16>,
    },
    StepCancelled,
    RateLimited,
    StepRefused {
        class: RefusalClass,
        capability: Option<String>,
    },
}

/// One event the kernel appends through `EventStore::append`.
///
/// **This is the minimum `append` needs, and it is deliberately not the typed-payload record**
/// (runbook `s7a` §7a.5). `EventKind` is closed (`event_kind.rs`) and so are `FailureClass` and
/// `RefusalClass` above, so **no class is stringly-typed**: a caller that wants a class uses those
/// enums and serialises one. What is *not* fixed is the per-kind payload **field set**: design §5's
/// event-kind table gives payload fields in prose, not types — its column is headed *"Payload
/// fields (named in this document)"* — and typing all twenty-eight of them off that table is a
/// design task with a certain halt inside it, which Step 7a is not the stage that takes.
///
/// So `payload` is `serde_json::Value` **provisionally and visibly**. The typed-payload record,
/// per kind, is **OWED**; its earliest real consumer is the SQLite adapter at 7c; and it is
/// reported as **ARMED, NOT DISCHARGED** — no assertion in Checkpoint 7a claims the payload is
/// typed.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub kind: crate::event_kind::EventKind,
    pub payload: serde_json::Value,
}

impl Event {
    #[must_use]
    pub const fn new(kind: crate::event_kind::EventKind, payload: serde_json::Value) -> Self {
        Self { kind, payload }
    }
}
