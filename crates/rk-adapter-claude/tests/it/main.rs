//! `rk-adapter-claude`'s ONE integration binary (ADR-0083; `[[test]] name = "it"`), a `mod` per
//! case cluster (s7c §7c.18, §7c.20 item 4 — Slice 4, the configuration-time half).

mod binder;
mod card;
mod command;
mod config;
mod env;
mod registry;
mod support;
