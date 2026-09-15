//! The AC-484 must-fail fixture: a package whose only property is its **name**.
//!
//! It exists so `rk-kernel` can declare an `rk-adapter-*` edge that is real to `cargo metadata`,
//! `cargo tree`, `cargo deny` and `Cargo.lock` — and that is OFF by default. The edge is optional
//! and sits behind `rk-kernel`'s non-default `adapter-edge-reintroduce` feature, so the everyday
//! resolved graph carries no adapter and `ac484_adapter_edge_fixture` can still watch the
//! assertion fire:
//!
//! ```text
//! green:  cargo metadata                                               -> no rk-adapter-* in rk-kernel's RESOLVED graph
//! RED:    cargo metadata --features rk-kernel/adapter-edge-reintroduce -> the assertion fails NAMING rk-adapter-sqlite
//! ```
//!
//! **There is deliberately no code here.** Not an empty stub awaiting an implementation — nothing
//! about AC-484 is a runtime property, and a `pub fn` would be a surface a future reader could bind
//! to and then find deleted when Step 7c builds the real `rk-adapter-sqlite` under `crates/`. The
//! item this crate contributes to the workspace is its entry in the dependency graph.
//!
//! **It has zero dependencies, and that is load-bearing.** An optional dependency enters
//! `Cargo.lock` whether or not its feature is enabled; `cargo audit` and `cargo deny` read the
//! lock; and AC-528 forbids an async runtime anywhere in `rk-kernel`'s resolved graph, directly or
//! transitively. Its `Cargo.toml` records the full reasoning.
//!
//! If this crate ever stops appearing in `rk-kernel`'s resolved graph under
//! `--features rk-kernel/adapter-edge-reintroduce`, AC-484's rule is pinned by nothing: the green
//! is then a green over an assertion that cannot fire.
