//! **AC-294's positive control.** A construction site for every spelling the rule bans, OUTSIDE the
//! module that owns `RunState` — plus the comment cases the rule must NOT flag.
//!
//! This file is a **fixture**, never compiled: it lives under `tests/fixtures/lint-controls/`, the
//! directory `s7b-ii` §7b-ii.10 opens for exactly this, and not under `tests/ui/`, which is the
//! `trybuild` glob root. `tests/it/module_boundary.rs` scans its bytes.
//!
//! # 🛑 THE COMMENT LINES BELOW ARE THE FOURTH GUARD'S CONTROL
//!
//! Three guards in this codebase were once satisfied by the comments explaining them, and this file
//! family documents its own bans by name. So the fixture carries the banned spellings in a doc
//! comment, in a line comment and in a block comment, and the test asserts the rule reports
//! **exactly** the code lines and **none** of these. A rule that counted them would be that defect
//! a fourth time.
//!
//! A doc-comment mention, and it is not a construction site: RunState::initial("INC-1", "verify").
//! A second one, the struct-literal spelling: RunState { item, stage }.

use rk_kernel::workflow::fsm::RunState;

// A line-comment mention, not a construction site: let x = RunState::initial("INC-1", "implement");

/*
 * A block-comment mention, not a construction site:
 *     let y = RunState { item: "INC-1".to_owned(), stage: "implement".to_owned() };
 *     let z = RunState::default();
 */

/// Doc comment on an item, mentioning RunState::default() and nothing more.
pub fn one() -> RunState {
    RunState::initial("INC-1", "implement")
}

pub fn two() -> RunState {
    RunState {
        item: "INC-1".to_owned(),
        stage: "implement".to_owned(),
    }
}

pub fn three() -> RunState {
    RunState::default()
}

// A declaration is not a construction site, and the rule must not count either of these two even
// though both carry the `RunState {` spelling. They cannot legally appear outside the owning module
// at all; they are here so the rule is shown to discriminate rather than to substring-match.
pub struct RunState {
    item: String,
}

impl RunState {
    fn noop(&self) {}
}
