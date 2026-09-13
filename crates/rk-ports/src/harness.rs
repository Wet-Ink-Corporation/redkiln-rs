//! The `Harness` port — the **only** one of the five whose Rust signature a record fixes.
//!
//! ADR-0099 clause 1 (`0099`:50-55), restated in design §3.3's Harness cell (design:266) and as
//! port rule P10 (design:288):
//!
//! > `Harness::dispatch(&self, &ActivitySpec, &mut dyn EventStore) -> DispatchOutcome { record:
//! > DispatchRecord, result: Result<Typed, StepError> }` — **blocks to completion**.
//!
//! The adapter's declared timeout (AC-223) is the in-band cancel; out-of-band cancellation is the
//! [`crate::cancel::CancelToken`] the composition root owns. `dispatch` returns no `Result` of its
//! own: the record is produced on **every** termination, and the step's own outcome travels in
//! `result`. That is AC-360's shape — *"no dispatch path exists that can terminate without
//! appending that event"* — and it is why a failure is a value here and not an error.

use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::error::StepError;
use rk_types::typed::Typed;

use crate::event_store::EventStore;

/// What one dispatch produced: the record, always, and the step's own outcome.
///
/// **Routed to `rk-ports`, not decided.** No record assigns it a crate; it is declared inside
/// ADR-0099 clause 1's own code block as `dispatch`'s return type, and the trait cannot be
/// declared without it. design:1704's `rk-ports` row does not list it and neither does
/// design:1699's `rk-types` row. **A record naming the home is OWED.**
#[derive(Debug)]
pub struct DispatchOutcome {
    pub record: DispatchRecord,
    pub result: Result<Typed, StepError>,
}

/// The model harness. One dispatch, blocking to completion.
pub trait Harness: Send + Sync {
    /// Dispatch one activity and **block until it is finished**. The store is handed in by
    /// exclusive reference so the adapter can journal as it goes.
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome;
}
