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
///
/// *(dated 2026-09-28, s8e build, §8e.10; ADR-0182, proposed.)* **The interim `item` key rides
/// beside `kind` and `payload`, never inside them** (D21-43 N5; D21-46 R3: *"Carried outside the
/// canonical digest, as ADR-0131 places its tag set"*). It is private so it has exactly one
/// writer, [`Event::with_item`], and it is not a member of `payload`, so `rk-kernel`'s `event_json`
/// and `canonical_events` stay `{kind, payload}` byte for byte (ADR-0131 decision 2 and its second
/// forbids bullet). The store persists it in `t2_event.item`. Equality includes it: an event read
/// back from a store equals the event appended, key and all. It moves to ADR-0131's reserved
/// `item` tag when that wiring lands.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub kind: crate::event_kind::EventKind,
    pub payload: serde_json::Value,
    // `Box<str>`, not `String`: the key is immutable once set, and the narrower carrier keeps every
    // `Result` whose error carries an `Event` under `clippy::result_large_err`'s 128 bytes.
    item: Option<Box<str>>,
}

impl Event {
    /// An event with no interim `item` key.
    #[must_use]
    pub const fn new(kind: crate::event_kind::EventKind, payload: serde_json::Value) -> Self {
        Self {
            kind,
            payload,
            item: None,
        }
    }

    /// The same event carrying the interim `item` key (ADR-0182 decision 1) — beside the payload,
    /// which this never touches.
    #[must_use]
    pub fn with_item(mut self, item: impl Into<String>) -> Self {
        self.item = Some(item.into().into_boxed_str());
        self
    }

    /// The interim `item` key, when the producer set one (ADR-0182).
    #[must_use]
    pub fn item(&self) -> Option<&str> {
        self.item.as_deref()
    }
}
