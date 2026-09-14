//! **AC-480, first THEN clause.** *"no generic parameter bound by a port trait, no `impl Trait`
//! return naming a port"* — design §3 principle 9 / port rule P1 (design:279): *"One trait per
//! port; **no generic parameter bound by a port trait appears anywhere in the kernel**, and every
//! port is held as one object-safe trait object."*
//!
//! *Verified by:* **an API-surface lint plus a compile-fail fixture** (design:279). This file is the
//! compile-fail fixture. `s7b`:425 hands it here by name — *"`ac480_generics_leak.rs` | a
//! `<P: Harness>`-shaped bound, or an `impl Trait` return naming a port | AC-480's first THEN
//! clause, handed here by `s7a`:995"* — and Checkpoint 7b's by-path loop (`s7b`:1584-1589) asserts
//! it and its `.stderr` **by path**. It was the one of that loop's six files that did not exist;
//! authored 2026-09-14, Slice 4i.
//!
//! This file must FAIL to compile.
//!
//! # 🛑 THIS IS A SUBSTITUTE, AND IT STANDS EXACTLY WHERE `ac028_steperror_wildcard.rs` STANDS
//!
//! **A `<P: Harness>` bound compiles.** Writing `fn f<P: Harness>(p: P)` in a downstream crate is
//! ordinary Rust and no compiler diagnostic exists for it, so the runbook's own gloss — *"red by
//! adding one `<P: Harness>` bound"* (`s7b`:1020) — has **no literal construction**, here as
//! anywhere. `ac028_steperror_wildcard.rs` records the same problem for AC-028's *"a compile-fail
//! test proving the wildcard cannot be reintroduced"* and takes the same route, and this file takes
//! it and labels it rather than letting a green stand for a property it does not pin.
//!
//! What is pinned instead is **the property that makes AC-480's THEN true of `rk-kernel`**: the
//! kernel's port-taking surface is `rk_ports::root::Ports`, five bare trait objects, and there is
//! no generic seam through which a port-bound type parameter could reach it. Each statement below
//! is a distinct error, and each would stop being one under a real leak.
//!
//! | statement | what must not compile | what a leak would look like |
//! | --- | --- | --- |
//! | 1 | handing a port-bound type parameter to the kernel's only runtime constructor | a `Runtime::new<H: Harness>(h: H, …)` — the generic entry point P1 forbids |
//! | 2 | erasing a bare port-bound type parameter into the holding shape `Ports` uses | a `Ports` whose fields were `H`/`S`/`T` rather than `Arc<dyn …>`, which is what makes the kernel itself generic (ADR-0076's forbid) |
//! | 3 | returning a kernel type as `impl Harness` | an `impl Trait` return naming a port — AC-480's THEN, second half, word for word |
//! | 4 | placing a port-bound type parameter in a `Ports` field | the same leak one level in: `Ports` is the struct the whole rule is about, and its five fields are the five trait objects |
//!
//! Statement 2's diagnostic is `E0310` and it is honest about its own limit: adding `+ 'static` to
//! the bound makes it compile. That is stated rather than hidden, and it is why statement 1 — which
//! **no** added bound rescues, because the parameter is the wrong *type*, not the wrong lifetime —
//! is the load-bearing one. The **API-surface lint** design:279 names beside this fixture is the
//! half that would catch a bound added inside `rk-kernel`'s own source; it is `s7a`:584's *"added at
//! 7b"* item and is **ARMED, NOT DISCHARGED** — `crates/rk-kernel/tests/it/ui.rs` says so, and no
//! grep in Checkpoint 7b asserts it.

use std::sync::Arc;

use rk_kernel::runtime::Runtime;
use rk_ports::harness::Harness;
use rk_ports::root::Ports;

/// **1.** The kernel's one runtime constructor takes `Ports` — the five ports already erased — and
/// there is no overload, no builder and no second entry point that takes a port-bound parameter.
fn the_kernel_entry_point_is_not_generic_over_a_port<H: Harness>(harness: H) -> Runtime {
    Runtime::new(harness, "run-1", "implement")
}

/// **2.** And a bare port-bound parameter cannot even be erased into the shape `Ports` holds.
fn a_port_bound_parameter_is_not_a_trait_object<H: Harness>(harness: H) -> Arc<dyn Harness> {
    Arc::new(harness)
}

/// **3.** *"No `impl Trait` return naming a port."* The kernel's own runtime handle is not a port
/// and there is no type in `rk-kernel`'s surface that is one.
fn the_kernel_returns_no_port(runtime: Runtime) -> impl Harness {
    runtime
}

/// And `Ports` is not constructible from parameters either — the field types are the trait objects.
fn ports_are_not_generic<H: Harness>(harness: H, rest: Ports) -> Ports {
    Ports {
        harness,
        ..rest
    }
}

fn main() {}
