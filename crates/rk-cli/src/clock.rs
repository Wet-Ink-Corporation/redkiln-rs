//! **The one wall-clock read** (AC-001, ADR-0083; `s7d` §7d.5, §7d.15).
//!
//! AC-001's whole content is that a decision may not depend on a hidden input, and a clock read
//! scattered through a program is the archetype: two reads a microsecond apart give two answers, so
//! a function that reads the clock twice is a function of something nobody passed it. The kernel
//! reads no clock at all — its `clippy.toml` denylist and the `select`/`workflow` module attributes
//! make that a build failure rather than a convention — and the composition root reads it **here**,
//! once per invocation, handing the value on.
//!
//! Checkpoint 7d's ban is scoped to `crates/rk-cli/src` with a module-path exclusion for exactly
//! this file, and `redkiln-migrate` has none at all: the migrator reads no clock, because git stamps
//! its commit.
//!
//! **Its consumers are Slice 3's** — the gate deadline the human gate records and the expiry read
//! `answer` performs (AC-248/AC-257) both take this one value rather than reading again. At Slice 2
//! the module exists and the rule it enforces is already in force.

use rk_types::time::Timestamp;

/// Read the wall clock. **Once per invocation**, at the root.
///
/// `#[expect]` and not `#[allow]`: `SystemTime::now` is on `clippy.toml`'s workspace-wide
/// `disallowed-methods` list, which is the point — the one admitted site says so out loud, and
/// every other site in either new crate is a build failure.
#[expect(
    clippy::disallowed_methods,
    reason = "the ONE wall-clock read ADR-0083 admits, at the composition root (AC-001); every \
              other site in rk-cli is banned by Checkpoint 7d's scan and by this lint"
)]
#[must_use]
pub fn now() -> Timestamp {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX));
    Timestamp::from_unix_seconds(secs)
}
