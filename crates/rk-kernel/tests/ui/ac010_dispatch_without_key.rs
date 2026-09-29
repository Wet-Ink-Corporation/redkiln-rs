//! **AC-010.** *"GIVEN an activity about to be executed / WHEN a dispatch is constructed / THEN it
//! **cannot be constructed without** an `(item, activity_id@version, attempt)` idempotency key."*
//!
//! *Verified by:* **"a `trybuild` test that a `dispatch` call without the key does not compile (the
//! port takes it by value, not `Option`)"**; plus a unit test on the key constructor rejecting empty
//! components.
//!
//! This file must FAIL to compile. It is the **first** of AC-010's two instruments and the register
//! names `trybuild` outright, so this is the criterion's own instrument and not a substituted one —
//! the contrast with `ac025_exhaustion_is_not_a_pass.rs`, whose register text says *"a negative
//! fixture"* and where the compile-fail reading is taken **in addition**.
//!
//! # The three statements, and what each one is for
//!
//! 1. **A struct literal without the key.** `PlannedDispatch`'s fields are private, so there is no
//!    expression that fills two of three. A `pub` field set would make the key a convention a caller
//!    could decline.
//! 2. **`new` with `None` where the key goes.** This is the register's own clause — *"the port takes
//!    it by value, not `Option`"* — and it is the one that would silently start compiling if
//!    someone widened the parameter to make a call site easier.
//! 3. **`new` with the key omitted entirely.** The arity is what makes *"cannot be constructed"*
//!    true of the constructor and not merely of the literal.
//!
//! # What this case does NOT pin, named rather than assumed
//!
//! It does not pin that the key is *validated* — that is `checked_key`'s and
//! `ac010_malformed_key_is_refused_before_spawn`'s, because a well-typed key can still carry an
//! empty `item` (`rk_store::memo::IdempotencyKey`'s three fields are `pub`). The two halves are two.
//!
//! And **nothing here is asserted by any Checkpoint 7b line.** §7b.4's case table has six rows and
//! none is this file; §7b.14's by-path loop (:1580-1589) enumerates the same six. Its absence is
//! caught by the by-path assertion inside `ac010_malformed_key_is_refused_before_spawn`, which
//! catches a deletion but **not** `.stderr` drift. That is gap G-2, extended for the sixth time.

use rk_kernel::workflow::idem::PlannedDispatch;
use rk_store::memo::IdempotencyKey;
use rk_types::activity::ActivitySpec;
use rk_types::digest::sha256;
use rk_types::typed::SchemaId;

fn spec() -> ActivitySpec {
    ActivitySpec {
        card_digest: sha256(b"card"),
        tools: Vec::new(),
        schema: SchemaId::new("ImplReport/v1"),
        timeout_secs: 900,
        inputs: Vec::new(),
        activity_id_at_version: None,
    }
}

fn main() {
    // 1. The fields are private, so a dispatch cannot be assembled around the key's absence.
    let _literal = PlannedDispatch {
        spec: spec(),
        degraded: None,
    };

    // 2. The key is taken BY VALUE and not as an `Option` — the register's own clause.
    let _optional = PlannedDispatch::new(spec(), None);

    // 3. And it cannot be left out: the arity is the type fact.
    let _omitted = PlannedDispatch::new(spec());

    // A well-formed one, so the errors above are about the key's absence and not about the module
    // path, the import or the spec.
    let _ok = PlannedDispatch::new(
        IdempotencyKey {
            item: "INC-1".to_owned(),
            activity_id_at_version: "impl-report@1".to_owned(),
            attempt: 1,
        },
        spec(),
    );
}
