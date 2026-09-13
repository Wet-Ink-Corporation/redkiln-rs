//! The `Tracker` port — *"`project(item) -> ExternalRef`; `inbox() -> Vec<TypedInput>`; idempotent
//! writes"* (design:267).
//!
//! The spike has `sync(&self, item: &str)`, which design:267 replaced; Checkpoint 7a refuses that
//! name for exactly that reason.
//!
//! **`ExternalRef` and `TypedInput` are assigned to no crate by any record.** design:267 is their
//! only occurrence in the design and design:1699's `rk-types` row lists `DocSet` and neither of
//! them. They are **not** a halt, because a halt on them takes the other four ports with it —
//! Checkpoint 7a requires five trait declarations and five fakes, so `Tracker` must exist. They
//! are **routed to `rk-ports`, provisionally and by name**, on the narrowest warrant available:
//! `Tracker` and `DocsSite` are fake-only until Phase 3 (AC-499's GIVEN), so no adapter and no
//! kernel consumer of either type exists before then, and a type nothing outside this crate names
//! is the crate's own until a record says otherwise.
//!
//! Each is an **opaque newtype in the `ChangeSummary` shape**, deliberately: an opaque newtype
//! declares the port shape without inventing a Phase-3 schema, which is the treatment ADR-0099
//! clause 5 gives the one other blocked type. **No field set is invented and no idempotency key is
//! designed** — that is AC-508's, in Phase 3. **A record naming their home and their contents is
//! OWED** before the GitHub adapter is written.

use rk_types::error::StepError;

/// A reference to an item in the external tracker. **Opaque: contents OWED.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalRef(pub(crate) ());

/// One typed input read off the tracker's inbox. **Opaque: contents OWED.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedInput(pub(crate) ());

/// The external work tracker. Fake-only in Phase 1; GitHub is Phase 3.
pub trait Tracker: Send + Sync {
    /// Project an item outward. **Writes are idempotent** (design:267) — projecting the same item
    /// twice produces one external item and the same reference, which is the rule the Phase-3
    /// idempotency key exists to keep.
    fn project(&self, item: &str) -> Result<ExternalRef, StepError>;

    /// Everything waiting to come in.
    fn inbox(&self) -> Result<Vec<TypedInput>, StepError>;
}
