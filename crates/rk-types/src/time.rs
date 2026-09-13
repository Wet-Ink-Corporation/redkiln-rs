//! A timestamp that can only be built from data.
//!
//! `Timestamp` has no `now()`. The wall clock is banned on the workflow side (AC-001, the
//! `clippy.toml` table), and time reaches the kernel only as a journaled activity result — so the
//! type offers no constructor that could read one. `StepError::RateLimited { resets_at }` is the
//! field this exists for (ADR-0095 decision 2).

use serde::{Deserialize, Serialize};

/// Seconds since the Unix epoch, as reported by a harness payload.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(transparent)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Build from a value the harness reported. There is deliberately no clock-reading constructor.
    #[must_use]
    pub const fn from_unix_seconds(secs: i64) -> Self {
        Self(secs)
    }

    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.0
    }
}
