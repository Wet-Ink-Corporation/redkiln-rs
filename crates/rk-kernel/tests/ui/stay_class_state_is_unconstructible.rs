//! **Phase-1 exit criterion (5)** — *"the `--stay` class of error is unrepresentable"*.
//! §7b.4:424: *"the resumed/half-advanced state the `--stay` class depends on"*.
//!
//! This file must FAIL to compile.
//!
//! # Why it could not be written before Slice 4e, and why it is written now
//!
//! §7b.13 assigns this case to **Slice 1** and it did not land there. Residual 9 is FORCED on why:
//! *"`stay_class_state_is_unconstructible.rs` is genuinely Slice 4's — it cannot be authored until
//! the FSM's resumed/half-advanced state type exists (§7b.4:437-444)."* Until 2026-09-14 there was
//! no such type in this crate: `src/workflow.rs`'s whole body was one `#[cfg(feature)]` control.
//! [`RunState`] is that type, and this case is what makes its restriction a **type fact** rather
//! than a convention.
//!
//! # 🛑 THIS CASE DECIDES NOTHING ABOUT WHICH TYPE CARRIES THE BAN, AND SAYS SO
//!
//! §7b.15 records that **the `--stay` fixture has no criterion** — *"searched `--stay`, `stay`,
//! `unrepresentable`: the only hits are AC-446's corpus list and ADR-0107's"* — and that
//! *"a criterion is allocated … or exit check (5) is withdrawn"*. That is the operator's, not a
//! stage's (`00-preamble.md`:55). What this file asserts is the narrow, checkable proposition the
//! runbook's own words name: **the half-advanced run state cannot be minted from nothing.** It is
//! derived by `fsm::fold` from a recorded journal, or it does not exist. The record naming the
//! criterion is OWED.
//!
//! ADR-0001:42 is the closest record — *"Every `advance`, gate, retry and budget becomes code,
//! making the `--stay` class of error unrepresentable"* — and it is a rationale, not a criterion.
//!
//! # The three statements, and the doctrine each comes from
//!
//! `RequiredSet`'s exact doctrine (`src/select.rs`:97-105), one module over: *"A `pub(crate)`
//! constructor, or a `#[cfg(test)]` public one, silently converts it from a type fact into a
//! convention."* `RunState::initial` is `pub(in crate::workflow)`, the type derives no `Default`,
//! and every field is private — so all three routes to a synthetic state are closed, and this file
//! names all three rather than the one that happens to be tried first.

use rk_kernel::workflow::fsm::RunState;

fn main() {
    // 1. The constructor is `pub(in crate::workflow)`. A resumed state cannot be minted from an
    //    item and a stage name.
    let _resumed = RunState::initial("INC-1", "implement");

    // 2. There is no `Default`. A state that is "empty" is still a state, and a run that believes
    //    it is at the start of a stage it is halfway through is the `--stay` class itself.
    let _fresh = RunState::default();

    // 3. Every field is private, so the struct literal is closed too — the route a `pub` field
    //    would have left open even with both of the above shut.
    let _literal = RunState {
        item: "INC-1".to_owned(),
        stage: "implement".to_owned(),
    };
}
