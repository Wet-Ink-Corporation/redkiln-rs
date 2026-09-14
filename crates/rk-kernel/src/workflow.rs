//! The workflow side of the split — the deterministic half (**AC-001–AC-004**).
//!
//! Stage selection, gate evaluation, retry decisions and DAG order live here, and none of them may
//! read the wall clock, draw randomness, perform I/O or iterate an unordered map. Time, randomness
//! and I/O reach this module only as **journaled activity results**.
//!
//! # The attribute below is the criterion's own named instrument
//!
//! The root `clippy.toml` carries the ban list (`disallowed-methods`, `disallowed-types`), and it
//! has **no path or module scoping** — `clippy.toml:1-3` says so in terms, and `s5` §5.1 says why.
//! The workspace tables are therefore green with or without this module's attribute, and **no grep
//! anywhere in the runbook can detect its absence** except the one Checkpoint 7b anchors on the
//! declaration. AC-001's *Verified by* names it directly: the ban list *"applied to
//! `rk-kernel::workflow` under a module-level `#[deny(clippy::disallowed_methods)]`"*.
//!
//! **The level, not the attribute, is what makes the gate bite**, and the distinction is worth
//! stating because it is where a new member silently leaves the deny set:
//! `clippy::disallowed_methods` is **warn-by-default**, and it is `[workspace.lints.clippy] all =
//! { level = "deny", priority = -1 }` reaching this crate through `Cargo.toml`'s
//! `[lints] workspace = true` that turns a reintroduced clock read into a non-zero exit. Drop that
//! table from the manifest and every lint assertion in Checkpoint 7b passes over an unlinted crate.
//! This attribute is what scopes the criterion to the module the register names; the manifest table
//! is what gives it teeth. Both are asserted.
//!
//! # 🛑 WHAT THIS GATE MEASURES IS AC-001, AND AC-002 AND AC-003 ARE ARMED, NOT MEASURED
//!
//! The reintroduce control below reintroduces **a wall-clock read**, which is AC-001's WHEN. The
//! three sibling criteria share AC-001's *instrument* and not its coverage, and two of them have no
//! entry behind them:
//!
//! - **AC-002** (randomness, non-deterministic ids) — `clippy.toml` carries `rand::random`,
//!   `rand::thread_rng` and `uuid::Uuid::new_v4`, and **no fixture reintroduces one**, so the ban is
//!   declared and unexercised. ARMED.
//! - **AC-003** (I/O — *"filesystem read/write, network, process spawn, or a blocking sleep"*) —
//!   its *Verified by* names **`std::fs`, `std::process`, `std::net`, `tokio::time::sleep`,
//!   `thread::sleep`**. `clippy.toml` carries the two sleeps and `std::process::Command`, and
//!   **no filesystem entry and no network entry at all** [measured 2026-09-13]. The filesystem
//!   half of AC-003 is therefore enforced by **nothing**. ARMED.
//! - **AC-004** (unordered iteration) — `HashMap`/`HashSet` are in `disallowed-types` and this
//!   module's `#![deny(clippy::disallowed_types)]` scopes them here; unexercised by a fixture.
//!   ARMED.
//!
//! **And this crate now performs real filesystem I/O in a public API**: `Catalog::from_dir` calls
//! `std::fs::read_dir` and `std::fs::read_to_string` (`catalog.rs`:223, :241), reached from
//! [`crate::catalog`]'s own surface. Whether that sits on the workflow side of the split is a
//! question **no record answers** — AC-003's GIVEN is *"any function on the workflow side"* and
//! nothing assigns `rk-kernel::catalog` to a side. **Two things are owed and neither is taken
//! here** (`00-preamble.md`:55): `clippy.toml` gaining the filesystem and network entry points the
//! criterion names — a **workspace-wide** table, so `rk-store` and `rk-git` would each need an
//! `#[expect(clippy::disallowed_methods, reason = …)]` per ADR-0102 decision 3 — and a record
//! settling which side the catalog loader is on, because if it is the workflow side the load path
//! moves behind the boundary. `clippy.toml` is outside this session's editable set.
//!
//! # Its positive control, which is committed rather than observed and reverted
//!
//! [`reads_the_wall_clock`] sits behind the non-default `workflow-clock-reintroduce` feature **of
//! this crate**, so a workspace-scoped clippy selects it:
//!
//! ```text
//! green:  cargo clippy --workspace --all-targets
//! RED:    cargo clippy --workspace --all-targets --features rk-kernel/workflow-clock-reintroduce
//! ```
//!
//! A grep shows the attribute is *declared*; only a run that goes red shows it *bites*. The
//! fixture is committed on `tests/fixtures/rk-clock-read/`'s precedent (`s5` §5.1a,
//! DECISIONS-2026-09-12 E4) because a reverted observation is not an assertion. Checkpoint 7b
//! asserts **both** halves of that run — a non-zero exit **and** `disallowed_method` in the
//! output — so an unrelated failure cannot read as the pass (`s5`:168-174 is the shape).
//!
//! # What is not here yet
//!
//! The workflow/activity split, the memo, the budgets, retry, the breaker, degradation and replay
//! are **Slice 4's** (§7b.10), and the seven-step pure selection function is **Slice 3's**
//! (§7b.9). Neither is stubbed here: an empty module that returns a default reads as built.

#![deny(clippy::disallowed_methods)]
#![deny(clippy::disallowed_types)]

/// The AC-001 must-fail control: exactly one banned determinism call, behind a non-default feature.
///
/// If this function ever clips **clean** with `--features rk-kernel/workflow-clock-reintroduce`,
/// the module-level deny above is declared and does not bite, and AC-001–AC-004's `rk-kernel` half
/// is pinned by nothing.
#[cfg(feature = "workflow-clock-reintroduce")]
#[must_use]
pub fn reads_the_wall_clock() -> std::time::SystemTime {
    // AC-001: the wall clock on the workflow side of the split. Banned by `clippy.toml`'s
    // `disallowed-methods` table and denied at module level by the attribute above.
    std::time::SystemTime::now()
}
