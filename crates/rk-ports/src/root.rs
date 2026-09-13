//! `Ports` — the five ports as the composition root holds them.
//!
//! **This struct is `rk-ports`', and the warrant is stated so an operator who disagrees routes
//! rather than silently re-homes it.** ADR-0115:63-71 gives the struct and names no crate;
//! design:1704's `rk-ports` row enumerates this crate's contents and does not list `Ports`;
//! design:1707 makes `rk-cli` the composition root that **constructs** one, at Step 7d. The type
//! must exist before its only construction site, and the reference shape already puts it in the
//! port crate (`spikes/s-ports/src/lib.rs`:173) — **or ADR-0115's narrowing ships unexercised for
//! three sub-steps.** This is a routing, not a decision; a record naming the home is owed.
//!
//! **The `store` field is the one line that discriminates ADR-0115 from the clause it narrows.**
//! The supertrait count reads five under either holding shape, so nothing else at any level tells
//! `Arc<Mutex<dyn EventStore>>` from the withdrawn `Arc<dyn EventStore>`; the second `compile_fail`
//! fence and `tests/ui/bare_arc_store_has_no_mut.rs` are the instruments that do.
//!
//! **Two downstream restatements of the un-narrowed form survive and are recorded, not resolved
//! here** (ADR-0092 precedence): design §3.3 P10 still ends *"a bare trait object and not a
//! newtype"* (design:288) with no ADR-0115 carve-out, and ADR-0105 clause 11 still reads *"Ports
//! are `Arc<dyn Port + Send + Sync>` (ADR-0099)"* at :102-103. ADR-0115 is the later record and
//! the only one that compiles; a dated amendment at design:288 is **OWED**.

use std::sync::{Arc, Mutex};

use crate::docs_site::DocsSite;
use crate::event_store::EventStore;
use crate::harness::Harness;
use crate::toolchain::Toolchain;
use crate::tracker::Tracker;

/// Every port, held exactly once, at the root.
pub struct Ports {
    /// **ADR-0115 clause 1**, narrowing ADR-0099 clause 4. `Arc<dyn T>` yields `&T` and never
    /// `&mut T`, so a bare `Arc` cannot produce the `&mut dyn EventStore` that `dispatch` takes.
    /// What clause 4 still forbids is a *user-defined* wrapper; `Mutex` adds neither a second
    /// dispatch nor a seam a fake must be swapped at (clause 3). The lock is taken **here**, at
    /// the call boundary, and released when the dispatch returns; it is never held across a
    /// dispatch to another port, and a poisoned acquisition halts the run (ADR-0118).
    pub store: Arc<Mutex<dyn EventStore>>,
    pub harness: Arc<dyn Harness>,
    pub tracker: Arc<dyn Tracker>,
    pub docs: Arc<dyn DocsSite>,
    pub toolchain: Arc<dyn Toolchain>,
}
