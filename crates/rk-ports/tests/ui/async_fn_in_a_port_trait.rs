//! AC-528's own named fixture: *"a source lint over `rk-ports` … **with a compile-fail fixture
//! adding one**"* (register :7162-7163). This file **must fail to compile**.
//!
//! **What this fixture's subject is, stated so it is not over-claimed.** A file under `tests/ui/`
//! cannot add a method to `rk_ports::Harness` — a trait definition is closed outside its own crate
//! — so it declares its own trait, and its subject is therefore the **RULE** (ADR-0076's reason),
//! not this crate's five traits. The `rk-ports`-specific half is carried by the
//! comment-filtered, positively-controlled source denylist: Checkpoint 7a's async line,
//! `no_async_in_port_traits`, and the `ci.yml` row.
//!
//! **And `async fn` in a trait has compiled since Rust 1.75, so the fixture must also show the
//! trait is no longer dyn-compatible** — which is the half that matters here, because the `dyn`
//! boundary is what the fakes swap at and what "no generics leak into the core" depends on. A port
//! trait carrying an `async fn` cannot be held as `Arc<dyn Port>` at all (E0038), so ADR-0076's
//! *"port traits stay sync"* is not a style rule: it is the only shape that satisfies ADR-0099
//! clause 4 while `async_fn_in_dyn_trait` is nightly-only, as it remains through 1.98.1, the pin
//! (R-ASYNC, discharged negative 2026-09-10).

use std::sync::Arc;

pub trait AsyncPort: Send + Sync {
    async fn probe(&self);
}

struct Probe;

impl AsyncPort for Probe {
    async fn probe(&self) {}
}

fn main() {
    let _p: Arc<dyn AsyncPort> = Arc::new(Probe);
}
