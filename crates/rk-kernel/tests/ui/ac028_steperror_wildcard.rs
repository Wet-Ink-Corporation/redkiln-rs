//! **AC-028.** *"GIVEN the `StepError` enum in `rk-types` / WHEN a new variant is added without a
//! row in the retry policy table / THEN the kernel fails to compile."*
//!
//! *Verified by:* an exhaustive `match` over `StepError` in the policy function with no wildcard
//! arm; **a compile-fail test (`trybuild`) proving the wildcard cannot be reintroduced.**
//!
//! This file must FAIL to compile.
//!
//! # 🛑 THE SECOND INSTRUMENT IS NOT CONSTRUCTIBLE IN THE DIRECTION THE CRITERION WRITES IT
//!
//! *"Proving the wildcard cannot be reintroduced"* has **no literal construction as a compile
//! failure**: adding `_ => …` to a `match` is a valid Rust program here as it is anywhere, and
//! nothing in this workspace forbids it — `clippy::wildcard_enum_match_arm` is a `restriction` lint
//! and is not in `[workspace.lints.clippy]`, and `clippy.toml` configures only `disallowed-methods`
//! and `disallowed-types`. Reported plainly rather than worked around: **the case below is a
//! substitute and says so.**
//!
//! What it pins instead is the property that makes AC-028's THEN true — *a policy `match` over
//! `StepError` that does not cover every variant does not compile* — plus the fact that a wildcard
//! placed after an exhaustive match is **dead**, so a reintroduced one is at best redundant and
//! cannot silently absorb a twelfth variant unless the variant is dropped from the match first,
//! which is `case_one` below.
//!
//! | statement | what must not compile | error |
//! | --- | --- | --- |
//! | `case_one` | a policy `match` missing `StepError::Overage` | `E0004` |
//! | `case_two` | a wildcard arm after all eleven, under `deny(unreachable_patterns)` | `unreachable_patterns` |
//!
//! **`case_two`'s `deny` is this file's own attribute and proves nothing about `rk-kernel`.** It
//! demonstrates the redundancy and no more; the assertion that `rk-kernel`'s own policy carries no
//! `_ =>` arm and no `matches!` is the **source lint** in
//! `crates/rk-kernel/tests/it/retry.rs::ac028_policy_match_has_no_wildcard`, which reads the shipped
//! file. Both are needed and neither is the other.
//!
//! # This file is asserted by no Checkpoint 7b line
//!
//! §7b.9:824-830 names AC-028's case among the *"four `trybuild` compile-fail obligations"*;
//! §7b.4's case table has six rows and none is this file, and §7b.14's by-path loop (:1580-1589)
//! enumerates the same six. So its absence, and any drift in its `.stderr`, would be caught by
//! nothing in the checkpoint. `ac028_policy_match_has_no_wildcard` asserts the path exists, on
//! `ac020_counters_never_share.rs`'s recorded precedent.

use rk_types::error::StepError;

/// A policy row per variant — **except one**. This is what a twelfth `StepError` variant looks like
/// from the policy's side, and `E0004` is AC-028's THEN.
fn case_one(error: &StepError) -> &'static str {
    match error {
        StepError::Transport { .. } => "backoff",
        StepError::Schema { .. } => "up_to_attempts",
        StepError::Timeout { .. } => "once_then_escalate",
        StepError::Cancelled { .. } => "never_cancelled",
        StepError::NonZero { .. } => "never_non_zero",
        StepError::RateLimited { .. } => "wait_until_reset",
        StepError::NotAdopted { .. } => "never_not_adopted",
        StepError::NotIsolated { .. } => "never_not_isolated",
        StepError::Platform { .. } => "backoff_platform",
        StepError::Unsupported { .. } => "never_unsupported",
        // `StepError::Overage { .. }` is deliberately absent.
    }
}

/// All eleven, and then a wildcard. The wildcard is dead, and this file's own `deny` makes the
/// deadness an error so the redundancy is visible rather than merely true.
#[deny(unreachable_patterns)]
fn case_two(error: &StepError) -> &'static str {
    match error {
        StepError::Transport { .. } => "backoff",
        StepError::Schema { .. } => "up_to_attempts",
        StepError::Timeout { .. } => "once_then_escalate",
        StepError::Cancelled { .. } => "never_cancelled",
        StepError::NonZero { .. } => "never_non_zero",
        StepError::RateLimited { .. } => "wait_until_reset",
        StepError::NotAdopted { .. } => "never_not_adopted",
        StepError::NotIsolated { .. } => "never_not_isolated",
        StepError::Platform { .. } => "backoff_platform",
        StepError::Unsupported { .. } => "never_unsupported",
        StepError::Overage { .. } => "never_overage",
        _ => "unknown",
    }
}

fn main() {
    let error = StepError::Timeout { after_secs: 1 };
    let _ = case_one(&error);
    let _ = case_two(&error);
}
