//! **AC-032.** *"GIVEN an activity that completed successfully and returned findings that block /
//! WHEN the retry policy is consulted / THEN no retry occurs — a domain rejection is not an error
//! class."*
//!
//! *Verified by:* **a type test proving the success and failure channels are disjoint
//! (`Result<Findings, StepError>`), so "the reviewer said no" cannot be constructed as a
//! `StepError`**; plus a unit test asserting dispatch count 1.
//!
//! This file must FAIL to compile.
//!
//! # This IS the register's own instrument, not a substitute for one
//!
//! The register calls its first instrument *"a type test"*. A type test's subject is what does and
//! does not compile, and a compile failure is the only thing that can observe it — so this file is
//! that instrument, and §7b.9:829-830's rule (*"assigning `trybuild` to a criterion whose register
//! instrument is a unit test changes an instrument"*) is not engaged. The **second** instrument,
//! *"a unit test asserting dispatch count 1"*, is `disjoint_channels_ac032` in
//! `crates/rk-kernel/tests/it/retry.rs`, and both travel under that one name.
//!
//! # What each statement forbids
//!
//! | statement | what must not compile | why it matters |
//! | --- | --- | --- |
//! | `Findings` → `StepError` by `.into()` | an `impl From<Findings> for StepError` | one `?` in a future boundary would route a *completed* review's blocking findings into the failure channel, and the retry policy would then be consulted for a domain rejection |
//! | `Finding` → `StepError` by `From::from` | the same at item grain | a per-item conversion is the easier mistake: it reads like "this one finding failed" |
//! | `StepError` → `Findings` | the inverse | RK-B0013's own shape — *"review failed to produce a digest"* was a transport error wearing a verdict's clothes |
//!
//! RK-B0013 is why `StepError` exists at all (`crates/rk-types/src/error.rs`:3-5): *"one HTTP 529
//! produced a committed `changes-requested` seal whose entire surviving finding was 'review failed
//! to produce a digest', because `!review` conflated 'the reviewer said no' with 'the reviewer never
//! spoke'."* The two channels of `Result<Findings, StepError>` are that distinction, and a `From`
//! in either direction is the collapse.
//!
//! # This file is asserted by no Checkpoint 7b line
//!
//! §7b.4's case table has six rows and none is this file; §7b.14's by-path loop (:1580-1589)
//! enumerates the same six. `disjoint_channels_ac032` asserts the path exists, on
//! `ac020_counters_never_share.rs`'s recorded precedent, because nothing in the checkpoint would
//! notice its deletion or its `.stderr` drifting.

use rk_types::artifact::findings::{Finding, Findings};
use rk_types::error::StepError;

/// A completed review that said no. **Not an error**, and there is no conversion that makes it one.
fn blocking_findings_are_not_an_error(findings: Findings) -> StepError {
    findings.into()
}

/// The same at item grain.
fn one_finding_is_not_an_error(finding: Finding) -> StepError {
    StepError::from(finding)
}

/// And the inverse: silence is never a verdict.
fn silence_is_not_a_verdict(error: StepError) -> Findings {
    error.into()
}

fn main() {}
