//! `rk-ports` — the five port traits, their fakes, the `CancelToken` and the `ChangeSummary`
//! newtype (design:1704).
//!
//! Five named ports and no sixth: event store, harness, tracker, docs site, toolchain
//! (**ADR-0003**:34). **Git is deliberately not a port** — trailers, worktrees, disposable clones
//! and `reconcile()` have measured behaviour and no alternative VCS is named, so the axis fails
//! ADR-0003's own test; `rk-git` is a concrete crate the kernel names and its `GitOps` seam is a
//! struct (ADR-0105 clause 1). A trait here whose name or methods match the git surface is
//! rejected by the denylist test (**AC-483**).
//!
//! Every port trait carries `Send + Sync` as supertraits and the composition root holds each as
//! `Arc<dyn Port + Send + Sync>`, with **one** exception that is a record and not a workaround:
//! the event-store port is held as `Arc<Mutex<dyn EventStore>>` (**ADR-0115** clause 1, narrowing
//! ADR-0099 clause 4), because an `Arc<dyn T>` yields `&T` and never `&mut T` and so cannot
//! produce the `&mut dyn EventStore` that `Harness::dispatch` takes. What clause 4 still forbids
//! is a *user-defined* wrapper between the `Arc` and the trait object; `Mutex` adds neither a
//! second dispatch nor a seam a fake must be swapped at. The lock is taken by the composition
//! root at the call boundary and released when the dispatch returns: **no port method acquires
//! it**, and a poisoned store lock **halts the run** — it is never silently recovered
//! (**ADR-0118** clauses 1 and 3). Step 7a writes no `lock()` call site at all; Step 7d owns the
//! acquisition and owes the discriminating test.
//!
//! # Why the ports are synchronous
//!
//! **ADR-0076**, as ADR-0099 restates it: port traits stay sync and every `.await` lives in an
//! adapter. Native `async fn` in trait is still not dyn-compatible and `async_fn_in_dyn_trait`
//! remains nightly-only through 1.98.1, the pin (R-ASYNC, discharged negative 2026-09-10). *"No
//! generics leak into the core"* and *"dyn-dispatched port"* are in direct tension the moment a
//! port method is asynchronous, and the `dyn` boundary is the one the fakes swap at. This crate's
//! normal dependency graph therefore names no runtime for driving futures — `cargo tree -p
//! rk-ports -e normal` is the assertion (**AC-528**).
//!
//! # Rule 1 — a port trait object requires `Send + Sync`; a fake that holds an `Rc` cannot be one
//!
//! F41's first `compile_fail` doctest, carried over unchanged but re-imported. This is design
//! §3.3 P10's own named instrument, *"a compile-fail fixture for a port trait without the
//! supertraits"* (design:288). **The fence pins that compilation fails; it cannot pin why** — an
//! expected error code on a doctest fence is inert at this pin [measured 2026-09-13 on rustc
//! 1.97.1 and 1.98.1: a fence marked with a code that does not exist still reports `ok`]. The
//! reason is pinned by `tests/ui/arc_dyn_port_rejects_rc.rs`, whose committed `.stderr` snapshot
//! goes red if the case starts failing for a different reason.
//!
//! ```compile_fail
//! use rk_ports::event_store::EventStore;
//! use rk_ports::harness::{DispatchOutcome, Harness};
//! use rk_types::activity::ActivitySpec;
//! use std::rc::Rc;
//! use std::sync::Arc;
//!
//! struct NotSendHarness(Rc<()>);
//!
//! impl Harness for NotSendHarness {
//!     fn dispatch(&self, _spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
//!         unreachable!()
//!     }
//! }
//!
//! let _h: Arc<dyn Harness> = Arc::new(NotSendHarness(Rc::new(())));
//! ```
//!
//! # Rule 2 — `Ports::store` is not the withdrawn bare `Arc<dyn EventStore>`
//!
//! **F41's second doctest is not carried over**, and the reason is mechanical: it constructs the
//! spike's own `kernel::Kernel`, which this crate may not contain (design:1704 enumerates its
//! contents and names no kernel module) and which `rk-kernel` does not provide until 7b. Pasted
//! in, it fails with E0433 — unresolved module — and `compile_fail` passes on that, inside the
//! very assertion that is supposed to prove both fences are carried over.
//!
//! So the second fence is re-homed on a rule this crate owns: ADR-0115 clause 1's holding shape,
//! and the shape AC-480's AND clause carries (register :6912).
//!
//! **Which instruments discriminate ADR-0115 from the clause it narrows, stated after measuring
//! rather than asserted.** The supertrait count reads five under either holding shape, because
//! ADR-0115:57-58 leaves the trait declarations unchanged, so something else has to do it. An
//! earlier revision of this header claimed `tests/ui/bare_arc_store_has_no_mut.rs` was *"the only
//! instrument at any level"* that did, and **that was false for the artefacts as built**: both it
//! and this fence declared their own local `Arc<dyn EventStore>` holder, so neither was sensitive
//! to [`root::Ports`]' actual field — reverting the field to the withdrawn shape left this fence
//! reporting `ok` [measured 2026-09-13 on the pin]. The instruments that do discriminate are:
//!
//! 1. `ports_struct_holds_arc_dyn_send_sync` (`tests/it/ports_struct.rs`), which **stops
//!    compiling** under the withdrawn field — `Arc<dyn T>` yields `&T` and never `&mut T`, so
//!    `lock()` is gone and `&mut *guard` cannot be produced [measured, ADR-0115:130-132];
//! 2. Checkpoint 7a's grep, anchored on the field DECLARATION
//!    `^[[:space:]]*pub store: Arc<Mutex<dyn EventStore>>`;
//! 3. **this fence, now that it names the real field** — under the correct shape the binding
//!    below is a type error, and under the withdrawn shape it compiles and the fence goes red.
//!
//! The fence is deliberately a **single** error: `compile_fail` is green whenever compilation
//! fails for any reason, so a snippet whose second error survives the mutation cannot discriminate
//! at all. The *reason* the withdrawn shape was rejected — that it cannot produce the
//! `&mut dyn EventStore` `dispatch` takes — is pinned by
//! `tests/ui/bare_arc_store_has_no_mut.rs`'s committed `.stderr`, which carries both halves.
//!
//! ```compile_fail
//! use rk_ports::event_store::EventStore;
//! use rk_ports::root::Ports;
//! use std::sync::Arc;
//!
//! fn withdrawn(ports: &Ports) {
//!     let _bare: &Arc<dyn EventStore> = &ports.store;
//! }
//! ```

pub mod cancel;
/// AC-499's shared per-port conformance bodies, behind `[features] conformance = []` (RT-16 as
/// re-amended, `s7c` §7c.1): the fake rows in this crate's own suite and the real rows in each
/// adapter's run **the same bodies**. The attribute is on its own line above the item because
/// Checkpoint 7c greps for exactly that.
#[cfg(feature = "conformance")]
pub mod conformance;
pub mod docs_site;
pub mod event_store;
pub mod fakes;
pub mod harness;
pub mod root;
pub mod toolchain;
pub mod tracker;
