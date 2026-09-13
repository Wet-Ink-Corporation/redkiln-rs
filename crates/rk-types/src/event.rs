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
