//! `rk-adapter-claude`'s ONE integration binary (ADR-0083; `[[test]] name = "it"`), a `mod` per
//! case cluster (s7c §7c.18, §7c.20 items 4 and 5 — Slice 4's configuration-time half and Slice
//! 5's runtime half). The live probe and the spike drivers compile only under `--features live`
//! and are `#[ignore]` (D17-5): `cargo test --workspace` never runs a paid dispatch.

mod binder;
mod card;
mod classify;
mod command;
mod config;
mod conformance;
mod dispatch;
mod env;
mod event;
mod headroom;
mod isolation;
#[cfg(feature = "live")]
mod live;
// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* `s8h` §8h.7's lint over
// `live.rs`, OUTSIDE the `live` feature so every suite runs it.
mod live_guard;
// *(dated 2026-10-02, s8h build, package R1FIX-8h.)* The live probes' denial reading (`s8h`
// §8h.11), OUTSIDE the `live` feature so its scripted-fixture tests run in every suite.
mod probe_denials;
mod reading;
mod registry;
mod stream;
mod support;
mod throttle;
mod ttl;
mod ui;
