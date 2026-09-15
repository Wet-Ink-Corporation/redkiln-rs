//! **AC-487's compile-fail fixture.** *Verified by:* *"a kernel lint (denylist: adapter crate
//! names, `downcast_ref`, `downcast_mut` on port objects); **a compile-fail fixture**."*
//!
//! This file must FAIL to compile.
//!
//! # It is NOT a second copy of the lint, and the difference is the whole reason it exists
//!
//! The lint (`src/kernel_denylist.rs`, driven by `tests/it/kernel_denylist.rs`) asserts that nobody
//! **has** written an identity check into `rk-kernel`. It is a source rule, and a source rule's
//! honest limit is that it is a rule about today's text: it would go red the moment someone wrote
//! one, and it says nothing about whether writing one is possible.
//!
//! This file pins the other half — **the language refuses the question.** None of the five port
//! traits carries an `Any` supertrait, so a `dyn Harness` cannot be downcast, cannot be coerced to
//! `dyn Any`, and cannot be asked what concrete adapter is behind it. `rk-kernel` does not merely
//! decline to branch on adapter identity; at the port boundary it **cannot**, and ADR-0003's
//! *"capability differences are data … never `if adapter == …` in the kernel"* (proposal §4.1) is
//! carried by the type system rather than by a grep.
//!
//! **What would make this file start compiling is the change it exists to catch**: adding `Any` as
//! a supertrait of `Harness` or `EventStore`, or adding an inherent `downcast_ref` to either. Under
//! `trybuild` a case that compiles is an *unexpected success* and turns
//! `trybuild_pins_the_reason_each_kernel_fixture_fails` red, so the guard is live in both
//! directions.
//!
//! # 🛑 IT IS ASSERTED BY NO CHECKPOINT LINE, AND THAT IS WHY IT IS SAID HERE
//!
//! Checkpoint 7b's by-path loop (`s7b`:1584-1589) enumerates **six** `tests/ui/` cases and this is
//! not one of them; §7b.4:426-433's case table does not carry it either. `tests/it/ui.rs`'s header
//! records the same for six other cases and states the consequence: **nothing in Checkpoint 7b
//! would notice this file going missing.** So it is asserted **by path** from the test that carries
//! its criterion — `ac487_kernel_denylist` in `tests/it/kernel_denylist.rs` — which requires this
//! `.rs` and its `.stderr` to exist and to be non-empty. That catches a deletion; it does not catch
//! `.stderr` drift, which only `trybuild` itself catches.
//!
//! | statement | what must not compile | what a leak would look like |
//! | --- | --- | --- |
//! | 1 | `downcast_ref` on the harness port object | an `Any` supertrait on `Harness`, which makes `if this_is_the_sqlite_adapter { … }` expressible |
//! | 2 | coercing a port object to `dyn Any` so the question can be asked one step removed | the same supertrait, reached by an upcast instead of a method |
//! | 3 | `downcast_mut` on the store, which the kernel uniquely holds by exclusive reference | an `Any` supertrait on `EventStore` — the port whose `&mut` the runtime already has |

use std::any::Any;
use std::sync::Arc;

use rk_ports::event_store::EventStore;
use rk_ports::fakes::{FakeHarness, VecStore};
use rk_ports::harness::Harness;

/// **1.** The register's own first downcast token, on the port object the kernel dispatches through.
fn which_harness_is_this(harness: &Arc<dyn Harness>) -> bool {
    harness.downcast_ref::<FakeHarness>().is_some()
}

/// **2.** And the question cannot be reached by erasing one step further: `Any` is not a supertrait
/// of `Harness`, so the coercion has nowhere to go.
fn ask_the_port_what_it_is(harness: &dyn Harness) -> bool {
    let erased: &dyn Any = harness;
    erased.is::<FakeHarness>()
}

/// **3.** The store is the one port the kernel holds by exclusive reference (ADR-0115 clause 1), so
/// it is the one where the mutable half of the register's denylist is even expressible as a call.
fn which_store_is_this(store: &mut dyn EventStore) -> bool {
    store.downcast_mut::<VecStore>().is_some()
}

fn main() {}
