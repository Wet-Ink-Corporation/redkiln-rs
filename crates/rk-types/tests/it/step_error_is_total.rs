//! `step_error_is_total` — AC-229, against ADR-0095 decision 2's table as ADR-0112 extends it.
//!
//! The claim under test is not "`StepError` exists". It is that the set is **closed at eleven**,
//! that `spoke()` and `event()` are **total over it with no catch-all arm**, and that each variant
//! carries the `spoke()` value and the single event arm the two tables assign it.
//!
//! The register's AC-229 still reads "exactly one of the **eight** closed `StepError` variants of
//! ADR-0095's table", and AC-428 notes "`Platform`'s absence from ADR-0095's table". ADR-0112
//! **supersedes** ADR-0095 and makes the set eleven; under ADR-0092 the accepted ADR governs the
//! register, so eleven is what is built here. The register's wording is stale and an erratum is
//! owed — recorded, not silently absorbed.

use rk_types::digest::Digest;
use rk_types::error::{CancelledBy, IsolationSurface, RetryClass, StepError, TransportKind};
use rk_types::event::{ErrorEvent, FailureClass, RefusalClass};
use rk_types::time::Timestamp;

fn digest() -> Digest {
    Digest::from_hex(&"a".repeat(64)).expect("64 lowercase hex chars is a valid digest")
}

/// Every variant, its `spoke()` value, and its event arm — read off ADR-0095 decision 2 and
/// ADR-0112, not off any implementation.
fn table() -> Vec<(&'static str, StepError, bool, ErrorEvent)> {
    vec![
        (
            "Transport",
            StepError::Transport {
                kind: TransportKind::Spawn,
                detail: "spawn failed".into(),
            },
            false,
            ErrorEvent::StepFailed {
                class: FailureClass::Transport,
                status: None,
            },
        ),
        (
            "Schema",
            StepError::Schema {
                attempts: 3,
                last_error: "missing structured_output".into(),
            },
            true,
            ErrorEvent::StepFailed {
                class: FailureClass::Schema,
                status: None,
            },
        ),
        (
            "Timeout",
            StepError::Timeout { after_secs: 900 },
            true,
            ErrorEvent::StepFailed {
                class: FailureClass::Timeout,
                status: None,
            },
        ),
        (
            "Cancelled",
            StepError::Cancelled {
                by: CancelledBy::Operator,
            },
            true,
            ErrorEvent::StepCancelled,
        ),
        (
            "NonZero",
            StepError::NonZero {
                code: Some(1),
                stderr_digest: digest(),
            },
            true,
            ErrorEvent::StepFailed {
                class: FailureClass::NonZero,
                status: None,
            },
        ),
        (
            "RateLimited",
            StepError::RateLimited {
                resets_at: Some(Timestamp::from_unix_seconds(1_757_000_000)),
            },
            true,
            ErrorEvent::RateLimited,
        ),
        (
            "NotAdopted",
            StepError::NotAdopted {
                expected_agent: "reviewer".into(),
                observed: "planner".into(),
            },
            false,
            ErrorEvent::StepRefused {
                class: RefusalClass::NotAdopted,
                capability: None,
            },
        ),
        (
            "NotIsolated",
            StepError::NotIsolated {
                surface: IsolationSurface::Tools,
                diff_digest: digest(),
            },
            false,
            ErrorEvent::StepRefused {
                class: RefusalClass::NotIsolated,
                capability: None,
            },
        ),
        (
            "Platform",
            StepError::Platform {
                status: 529,
                retry_after: Some(30),
            },
            true,
            ErrorEvent::StepFailed {
                class: FailureClass::Platform,
                status: Some(529),
            },
        ),
        (
            "Unsupported",
            StepError::Unsupported {
                capability: "structured_output".into(),
            },
            false,
            ErrorEvent::StepRefused {
                class: RefusalClass::Unsupported,
                capability: Some("structured_output".into()),
            },
        ),
        (
            "Overage",
            StepError::Overage {
                reason: Some("overageDisabledReason".into()),
            },
            false,
            ErrorEvent::StepRefused {
                class: RefusalClass::Overage,
                capability: None,
            },
        ),
    ]
}

#[test]
fn step_error_is_total() {
    let table = table();

    // The set is closed at ELEVEN. A twelfth variant is a successor to ADR-0112, not an edit.
    assert_eq!(
        table.len(),
        11,
        "the closed StepError set is eleven variants (ADR-0112)"
    );

    for (name, err, want_spoke, want_event) in table {
        assert_eq!(
            err.spoke(),
            want_spoke,
            "{name}: spoke() must equal ADR-0095/0112's table value, not an implementer's reading"
        );
        assert_eq!(
            err.event(),
            want_event,
            "{name}: exactly one event arm (AC-360)"
        );
    }
}

/// `spoke()` is true for SIX variants and false for FIVE. Asserted as a partition rather than
/// per-variant, so a future edit that flips one cannot be absorbed by the loop above.
#[test]
fn spoke_partitions_six_true_five_false() {
    let spoke_true = table().into_iter().filter(|(_, e, _, _)| e.spoke()).count();
    assert_eq!(
        spoke_true, 6,
        "Schema, Timeout, Cancelled, NonZero, RateLimited, Platform"
    );
    assert_eq!(
        11 - spoke_true,
        5,
        "Transport, NotAdopted, NotIsolated, Unsupported, Overage"
    );
}

/// **AC-523's zero-backoff row, re-homed here from the adapter** (`s7c` §7c.18): *"the zero-backoff
/// table is not the adapter's to break"* — `RateLimited → RetryClass::WaitUntilReset` is this
/// crate's `retry_class()` (Step 5), consumed by `rk-kernel`'s `retry.rs` (7b), and no adapter
/// mutation moves it. What the adapter keeps is only what it can break: that a usage-limit result
/// CLASSIFIES as `RateLimited` (`rate_limited_from_stream`). Both arms of the reading — a
/// `resets_at` present and absent — land on the same row: a rate limit is waited out until the
/// reset, or until the pack delay, and is **never** backed off against. `Backoff` is `Transport`'s
/// and `Platform`'s class and no other's.
#[test]
fn ac523_zero_backoff_table() {
    for resets_at in [None, Some(Timestamp::from_unix_seconds(1_789_000_000))] {
        let err = StepError::RateLimited { resets_at };
        assert_eq!(
            err.retry_class(),
            RetryClass::WaitUntilReset,
            "a rate limit is waited out until reset, never backed off (AC-523; ADR-0095 decision 2)"
        );
        assert_ne!(err.retry_class(), RetryClass::Backoff);
    }
    // The converse, so the row above cannot be satisfied by a table that answers WaitUntilReset
    // for everything: backoff is exactly the two classes ADR-0095 gives it.
    let backoff: Vec<&str> = table()
        .iter()
        .filter(|(_, e, _, _)| e.retry_class() == RetryClass::Backoff)
        .map(|(name, ..)| *name)
        .collect();
    assert_eq!(backoff, ["Transport", "Platform"]);
}
