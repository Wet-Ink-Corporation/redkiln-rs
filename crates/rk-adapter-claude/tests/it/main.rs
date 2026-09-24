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
mod reading;
mod registry;
mod stream;
mod support;
mod throttle;
mod ttl;
mod ui;
