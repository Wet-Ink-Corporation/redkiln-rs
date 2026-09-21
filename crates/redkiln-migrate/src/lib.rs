//! `redkiln-migrate` v0 — the old-corpus converter (`s7d` §7d.11).
//!
//! A separate binary, never a `redkiln` verb (AC-441; ADR-0002, ADR-0013). It converts files and
//! commits once; it **dispatches nothing and has no port to dispatch through** — its dependency
//! closure reaches no `rk-adapter-*`, no `rk-ports` and no `rk-kernel` (AC-466), which
//! `ac466_ac441_migrate_depends_on_no_harness_adapter_or_ports` and Checkpoint 7d's `cargo
//! metadata` walk both assert.
//!
//! This library is what the integration binary links: an integration test cannot link a `[[bin]]`,
//! and AC-449's read-order arm drives [`lift::run`]'s [`lift::DirOrder`] seam **in process**,
//! because a seam is unreachable from a spawned process without the flag or environment toggle
//! AC-474 forbids. `main.rs` is the thin caller that maps a result onto RT-53's exit values.
//!
//! **This crate reads no environment variable and no clock.** `std::env::var` would be an AC-474
//! toggle over the write path, and git stamps the commit (AC-001). Checkpoint 7d bans both over
//! `crates/redkiln-migrate/src`, each against a positive control under `rk-cli`'s
//! `tests/fixtures/lint-controls/`.

pub mod legacy;
pub mod lift;
pub mod report;
pub mod run_contract;
