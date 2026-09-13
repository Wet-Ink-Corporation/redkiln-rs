//! AC-160's instrument: a state field with **no durability-tier annotation** must be a compile
//! error, not a silent default.
//!
//! This file must FAIL to compile. If it ever compiles, a state field can enter the system without
//! anyone having answered the deleted-database question — *if the database were deleted, would
//! `redkiln run` take a different next action?* — and ADR-0109 decision 2's *"a state field with no
//! tier annotation is a compile error"* is enforced by nothing.

use rk_store::tiered_state;

tiered_state! {
    pub struct Unannotated {
        tier1 stage: String,
        // No `tier1` / `tier2`. This is the whole fixture.
        wall_clock_ms: u64,
    }
}

fn main() {
    let _ = Unannotated {
        stage: String::new(),
        wall_clock_ms: 0,
    };
}
