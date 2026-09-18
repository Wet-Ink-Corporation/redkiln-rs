//! `StepError` — the closed error set that crosses the port boundary (AC-229).
//!
//! RK-B0013 is why this type exists: one HTTP 529 produced a committed `changes-requested` seal
//! whose entire surviving finding was "review failed to produce a digest", because `!review`
//! conflated "the reviewer said no" with "the reviewer never spoke".

use serde::{Deserialize, Serialize};

use crate::digest::Digest;
use crate::event::{ErrorEvent, FailureClass, RefusalClass};
use crate::time::Timestamp;

/// Where a transport failure happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportKind {
    Spawn,
    Io,
    Decode,
}

/// Who cancelled the step.
///
/// `Serialize` + `Deserialize` since Step 7c Slice 1 (`s7c` §7c.11): the `dispatch_completed`
/// row carries the cancel cause as `cancelled_by: Option<CancelledBy>` on the `Cancelled { by }`
/// path (AC-225; ADR-0126 decision 1), and a member of that payload has to travel through
/// `Event.payload`'s `serde_json::Value`. Wire form: `signal | timeout | operator`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelledBy {
    Signal,
    Timeout,
    Operator,
}

/// Which isolation surface disagreed with the card's declaration.
///
/// Tool-set inequality against the card's allowlist is `NotIsolated`, in AC-204 as in
/// AC-202/AC-386 and design §7.4; `NotAdopted` is reserved for the wrong agent answering
/// (ADR-0095 decision 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationSurface {
    Tools,
    McpServers,
    Env,
}

/// Is retrying this plausibly useful, and on what schedule (AC-029/AC-234)?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryClass {
    /// Retry with exponential backoff.
    Backoff,
    /// Retry up to the recorded attempt ceiling, then fail.
    UpToAttempts,
    /// Retry once, then escalate.
    OnceThenEscalate,
    /// Wait until `resets_at`, else the pack delay; never backoff.
    WaitUntilReset,
    /// Never. A configuration or billing state does not improve by being repeated.
    Never,
}

/// The closed set. The kernel and every adapter match it exhaustively; no stringly-typed or
/// catch-all error crosses the port boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepError {
    Transport {
        kind: TransportKind,
        detail: String,
    },
    Schema {
        attempts: u32,
        last_error: String,
    },
    Timeout {
        after_secs: u64,
    },
    Cancelled {
        by: CancelledBy,
    },
    NonZero {
        code: Option<i32>,
        stderr_digest: Digest,
    },
    RateLimited {
        resets_at: Option<Timestamp>,
    },
    NotAdopted {
        expected_agent: String,
        observed: String,
    },
    NotIsolated {
        surface: IsolationSurface,
        diff_digest: Digest,
    },
    // ADR-0112 (2026-09-11): the three that thirteen criteria and design §3.6 already named.
    Platform {
        status: u16,
        retry_after: Option<u64>,
    },
    Unsupported {
        capability: String,
    },
    Overage {
        reason: Option<String>,
    },
}

impl StepError {
    /// Did the child produce output the record must retain?
    ///
    /// This is the RK-B0013 predicate. Everything that is not `spoke()` is silence, and silence is
    /// never a verdict.
    ///
    /// Written as a two-arm `match` with or-patterns, and the shape is forced from both sides: an
    /// arm-per-variant form trips `clippy::match_same_arms` (denied workspace-wide), and
    /// `matches!(…)` expands to an implicit `_ => false` catch-all, under which a twelfth variant
    /// would compile silently. Only this form keeps `E0004` on a new variant *and* passes the lint.
    #[must_use]
    pub const fn spoke(&self) -> bool {
        match self {
            Self::Schema { .. }
            | Self::Timeout { .. }
            | Self::Cancelled { .. }
            | Self::NonZero { .. }
            | Self::RateLimited { .. }
            | Self::Platform { .. } => true,
            Self::Transport { .. }
            | Self::NotAdopted { .. }
            | Self::NotIsolated { .. }
            | Self::Unsupported { .. }
            | Self::Overage { .. } => false,
        }
    }

    /// The retry class of ADR-0095 decision 2's table.
    #[must_use]
    pub const fn retry_class(&self) -> RetryClass {
        // Grouped by CLASS, not listed per variant: `clippy::match_same_arms` is in the
        // workspace deny list (ADR-0102 decision 4), so arms with identical bodies must be
        // or-patterns. Still exhaustive, so a twelfth variant is still E0004 here.
        match self {
            // `Platform` backs off honouring `retry_after`; the exit code is never consulted to
            // overturn the class (AC-218). Constants are AC-029's: base 2 s, cap 5 min.
            Self::Transport { .. } | Self::Platform { .. } => RetryClass::Backoff,
            Self::Schema { .. } => RetryClass::UpToAttempts,
            Self::Timeout { .. } => RetryClass::OnceThenEscalate,
            Self::RateLimited { .. } => RetryClass::WaitUntilReset,
            // Cancellation, a non-zero exit, and the four configuration/billing states. `Overage`
            // is refused BEFORE spawn (AC-221, ADR-0010); `Unsupported` fails pack validation at
            // the requiring row (design §3.6 P7). None improves by being repeated.
            Self::Cancelled { .. }
            | Self::NonZero { .. }
            | Self::NotAdopted { .. }
            | Self::NotIsolated { .. }
            | Self::Unsupported { .. }
            | Self::Overage { .. } => RetryClass::Never,
        }
    }

    /// The one event arm this variant maps to (AC-360).
    #[must_use]
    pub fn event(&self) -> ErrorEvent {
        match self {
            Self::Transport { .. } => ErrorEvent::StepFailed {
                class: FailureClass::Transport,
                status: None,
            },
            Self::Schema { .. } => ErrorEvent::StepFailed {
                class: FailureClass::Schema,
                status: None,
            },
            Self::Timeout { .. } => ErrorEvent::StepFailed {
                class: FailureClass::Timeout,
                status: None,
            },
            Self::Cancelled { .. } => ErrorEvent::StepCancelled,
            Self::NonZero { .. } => ErrorEvent::StepFailed {
                class: FailureClass::NonZero,
                status: None,
            },
            Self::RateLimited { .. } => ErrorEvent::RateLimited,
            Self::NotAdopted { .. } => ErrorEvent::StepRefused {
                class: RefusalClass::NotAdopted,
                capability: None,
            },
            Self::NotIsolated { .. } => ErrorEvent::StepRefused {
                class: RefusalClass::NotIsolated,
                capability: None,
            },
            Self::Platform { status, .. } => ErrorEvent::StepFailed {
                class: FailureClass::Platform,
                status: Some(*status),
            },
            Self::Unsupported { capability } => ErrorEvent::StepRefused {
                class: RefusalClass::Unsupported,
                capability: Some(capability.clone()),
            },
            Self::Overage { .. } => ErrorEvent::StepRefused {
                class: RefusalClass::Overage,
                capability: None,
            },
        }
    }
}
