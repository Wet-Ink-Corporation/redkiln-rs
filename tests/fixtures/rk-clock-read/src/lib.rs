//! The AC-001 must-fail fixture: it reintroduces exactly one banned determinism call.
//!
//! The offending call sits behind the non-default `reintroduce` feature, so
//! `cargo clippy --workspace --all-targets` is green and
//! `cargo clippy --workspace --all-targets --features rk-clock-read/reintroduce` is red on
//! `clippy::disallowed_methods` — the `clippy.toml` table at the workspace root.
//!
//! If this crate ever clips CLEAN with the feature on, the determinism ban is not in force and
//! AC-001–AC-004 are pinned by nothing.

#[cfg(feature = "reintroduce")]
pub fn reads_the_wall_clock() -> std::time::SystemTime {
    // AC-001: the wall clock on the workflow side of the split. Banned by `clippy.toml`'s
    // `disallowed-methods`. Time reaches the workflow only as a journaled activity result.
    std::time::SystemTime::now()
}
