//! **AC-294's compile-fail case** — *"the build fails if a block value is constructed anywhere
//! outside `rk-kernel::policy`"*. This file must FAIL to compile, for the reason the committed
//! `.stderr` pins: `Block`'s fields are private and it has no public constructor, so the only route
//! to a block value is `policy::classify`'s `(class, weight)` lookup (ADR-0074 clause 5: *"the
//! policy table is the only place a class becomes a block"*).
//!
//! It is ALSO the positive control for Checkpoint 7b-ii's construction-site grep, which is why it
//! stays under `ui/` while the AC-285/AC-286 controls do not (`s7b-ii` §7b-ii.10): the fixture must
//! both construct a block value and fail to compile.

use rk_kernel::review::classify::EvidenceClass;
use rk_kernel::review::policy::{Block, ReviewWeight};

fn main() {
    // A block value minted outside the policy table — the second location ADR-0074 forbids.
    let _forged = Block {
        class: EvidenceClass::Reproduced,
        weight: ReviewWeight::Strict,
    };
}
