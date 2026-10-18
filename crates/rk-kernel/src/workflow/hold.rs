//! The boundary hold's gate (ADR-0201 decisions 2 and 3, proposed).
//!
//! *(Added 2026-10-18, `s8j` build §8j.3, package S8J-C3-HOLD.)*
//!
//! An item record's `hold_before = "<stage>"` is supplied to the kernel beside the successor
//! ([`FsmContext::with_hold_before`](crate::workflow::fsm::FsmContext::with_hold_before)). When
//! [`next_action`](crate::workflow::fsm::next_action)'s step 5 would advance into the held stage
//! and no answer to the hold stands, it opens a gate of kind `hold:<stage>` instead, answered by
//! the default question (ADR-0129 decision 3). A standing `approve` releases the advance; any other
//! standing verdict holds the lane at the boundary, and the hold never re-fires.
//!
//! **The Phase-1 spelling is a constant on a code line** (ADR-0201 decision 5), so the gate-expiry
//! lint's minted table and Checkpoint 8h read it where it is spelt.
//! [`gate_kind`](crate::workflow::hold::gate_kind) is the one mint.

use crate::workflow::fsm::{GateAnswer, RunState};

/// The gate-kind prefix: the held stage follows it.
const PREFIX: &str = "hold";

/// The verdict that releases the advance (the default question's first option).
const RELEASE: &str = "approve";

/// The reserved fix item's hold: the gate the analysis → specification boundary opens.
pub const SPECIFICATION_KIND: &str = "hold:specification";

/// The gate kind of the hold before `stage`: `hold:<stage>`.
#[must_use]
pub fn gate_kind(stage: &str) -> String {
    format!("{PREFIX}:{stage}")
}

/// The answer to the hold before `stage` that **stands** on the item: the latest reach of its gate,
/// answered and not void. `None` while it was never opened, is open, expired unanswered, or its
/// latest reach was voided (a void is not an answer that stood, ADR-0201 decision 3).
#[must_use]
pub fn standing<'s>(state: &'s RunState, stage: &str) -> Option<&'s GateAnswer> {
    let latest = state.latest_gate(&gate_kind(stage))?;
    if state.is_void(latest) {
        return None;
    }
    state.answered_gates().find(|a| a.token() == latest)
}

/// Whether a standing answer releases the advance into the held stage: `approve` only.
pub(crate) fn releases(answer: &GateAnswer) -> bool {
    answer.verdict() == RELEASE
}
