//! The `EventStore` port — *"append; query by item / SHA / run; snapshot; rebuild"* (design:265).
//!
//! **The method NAMES are design §3.3's responsibility column, not the spike's.**
//! `spikes/s-ports/` has `append` + `read` and no query, snapshot or rebuild; it is a shape check,
//! not an interface.
//!
//! **Everything below the names is a ROUTING, not a decision** (runbook `s7a` §7a.3), reversible
//! by the record that is owed:
//!
//! * design:265's *"query by item / SHA / run"* fixes neither the arity nor the key types, so the
//!   trio is **one method with a selector**, [`Query`], an `rk-ports` enum of three variants. What
//!   each variant's key type should be is fixed by no record and Checkpoint 7a asserts none of it.
//! * `snapshot` / `rebuild` take an opaque [`Snapshot`]. **No field set is invented.**
//! * every fallible port method returns `Result<_, StepError>`, `rk-types`' eleven-variant enum —
//!   the one error type any record puts across a port boundary (ADR-0099:57).
//!
//! `append`'s parameter is `rk-types`' `Event { kind: EventKind, payload: serde_json::Value }`,
//! routed there because ADR-0105 clause 7 places the event-kind table in design §5 with `rk-types`
//! as the owning crate. **The per-kind payload field sets are NOT typed here and that is reported
//! as ARMED, NOT DISCHARGED** — no assertion in Checkpoint 7a claims the payload is typed.

use rk_types::error::StepError;
use rk_types::event::Event;

/// Which events to return. design:265's three keys as one selector.
///
/// **Routed, not decided.** The key types are `String` because no record fixes them and a typed
/// item id, a typed commit SHA and a typed run id are three type-design decisions Step 7a is not
/// the stage to take.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Query {
    ByItem(String),
    BySha(String),
    ByRun(String),
}

/// A point-in-time snapshot of a store. **Opaque: no field set is invented.**
///
/// One of this crate's four opaque newtypes, and one of the three with no record behind them
/// (`ChangeSummary` is ADR-0099 clause 5's). The cost is stated rather than smoothed in `s7a`
/// §7a.3. *(amended 2026-09-25, s8a fix round 1, ADR-0165: `ChangeSummary` is no longer an opaque
/// newtype but an enum with a finalised `Text` arm, so this is one of the crate's three opaque
/// newtypes, none of them with a record behind it.)* *(Corrected 2026-09-25, s8a fix round 3: the
/// crate still holds four. `ChangeSummary`'s sealed `Binary` payload, `BinaryChangeSummary`, is
/// the fourth, and ADR-0165 stands behind it; this is one of the three with no record behind
/// them, beside `ExternalRef` and `TypedInput`.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot(pub(crate) ());

/// The event store the kernel appends to and replays from.
///
/// `append` takes `&mut self`, which is why the composition root holds this one port as
/// `Arc<Mutex<dyn EventStore>>` and not as a bare `Arc<dyn EventStore>` (ADR-0115 clause 1).
pub trait EventStore: Send + Sync {
    /// Append one event. ADR-0105 clause 7 puts one writer per process behind a mutex; the lock
    /// is the composition root's and **this method never acquires it**.
    fn append(&mut self, event: Event) -> Result<(), StepError>;

    /// Every event matching the selector, in append order.
    fn query(&self, by: Query) -> Result<Vec<Event>, StepError>;

    /// Take a snapshot of the current state.
    fn snapshot(&self) -> Result<Snapshot, StepError>;

    /// Rebuild from a snapshot.
    fn rebuild(&mut self, snapshot: &Snapshot) -> Result<(), StepError>;
}
