//! **AC-487's positive control. NOTHING COMPILES THIS FILE, and it is not a `trybuild` case.**
//!
//! It lives under `tests/fixtures/lint-controls/` and **not** under `tests/ui/`, because `tests/ui/`
//! is the `trybuild::TestCases` glob root (`s7b` §7b.4: one harness *"over every
//! `crates/rk-kernel/tests/ui/` case"*): a file placed there must FAIL to compile and must commit a
//! `.stderr`, and a lint's positive control is an ordinary file that carries the banned tokens.
//! `s7b-ii` §7b-ii.10 opens this directory and says exactly that.
//!
//! # What it is for
//!
//! `ac487_kernel_denylist` returns `Ok` over `crates/rk-kernel/src/`. A rule that returned `Ok` over
//! **every** input would pass that assertion, so the green says nothing on its own. This file is
//! what the rule is run over to show it **bites**, and the test asserts the exact
//! `(line, pattern)` set it produces — so a rule that flagged the wrong line, or that flagged the
//! prose below, would be red.
//!
//! # 🛑 THE COMMENT LINES ARE PART OF THE CONTROL, NOT DECORATION
//!
//! This header names `rk-adapter-sqlite`, `downcast_ref` and `downcast_mut` in prose, exactly as
//! `crates/rk-kernel/src/lib.rs`:172 and `crates/rk-kernel/src/workflow/retry.rs`:493 do. A naive
//! `contains` over the file would report them. **The test asserts they are NOT in the violation
//! set**, which is the discriminating half: without it, a rule with the comment-stripping removed
//! would still flag every code line below and still look like it worked.

use std::any::Any;
use std::sync::Arc;

use rk_ports::harness::Harness;

/// Clause 1, the manifest spelling — *"no adapter name string"*.
fn route_by_adapter_name(adapter_id: &str) -> u32 {
    if adapter_id == "rk-adapter-sqlite" { 1 } else { 0 } // the shape proposal §4.1 forbids by name
}

/// Clause 1, the Rust path spelling. The crate-name entry alone misses this: there is no hyphen.
fn route_by_adapter_path() -> u32 {
    rk_adapter_claude::CAPABILITY_RANK
}

/// Clause 2 — the register's own `downcast_ref`, on a port object.
fn which_harness_is_this(harness: &Arc<dyn Harness>) -> bool {
    harness.downcast_ref::<FakeHarness>().is_some()
}

/// Clause 2 again — `downcast_mut`, on the other port object the kernel holds by exclusive
/// reference.
fn which_store_is_this(store: &mut dyn rk_ports::event_store::EventStore) -> bool {
    store.downcast_mut::<VecStore>().is_some()
}

/// Clause 3 — the THEN's `Any::downcast`, which neither `_ref` nor `_mut` spells.
fn ask_the_port_what_it_is(erased: &dyn Any) -> bool {
    erased.downcast::<FakeHarness>().is_ok()
}

/// Clause 4 — the adapter-discriminant enum, in the prefix shape a substring rule can reach.
pub enum AdapterKind {
    Sqlite,
    Claude,
}

fn main() {}
