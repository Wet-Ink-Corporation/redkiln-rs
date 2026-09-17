//! **AC-244's type-level half.** *"a type-level test (the gate record is unconstructible without all
//! three fields)"* — ADR-0017: the record IS `(token, tree_hash, deadline)`, and its rejected option
//! "item-keyed answer" is *"exactly what an optional token degrades into"* (the criterion's own Why).
//!
//! This file must FAIL to compile, for two reasons the committed `.stderr` pins separately:
//!
//! 1. the fields of `GateRecord` are private, so no struct literal — complete OR partial — can
//!    build one outside `rk_kernel::workflow::fsm`; and
//! 2. `GateRecord::derive` is the only constructor and takes all three, so a call that omits the
//!    deadline is an arity error, not a record with a defaulted deadline.
//!
//! If this ever compiles, a gate can be opened against a record one of whose three bindings is
//! absent, and AC-249's void rule then checks a claim nobody made.

use rk_kernel::workflow::fsm::GateRecord;
use rk_types::time::Timestamp;

fn main() {
    // (1) a partial literal: no `deadline`, no `token` — and the fields are private anyway.
    let _partial = GateRecord {
        kind: "human:review".to_owned(),
        tree_hash: "aaaa111122223333444455556666777788889999".to_owned(),
    };

    // (2) the one constructor, called without the deadline.
    let _two_of_three = GateRecord::derive("human:review", "aaaa111122223333444455556666777788889999");

    // Control that the imports are the real ones: this line compiles on its own.
    let _ts = Timestamp::from_unix_seconds(1_700_000_000);
}
