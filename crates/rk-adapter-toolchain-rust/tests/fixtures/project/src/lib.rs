//! The fixture project's library: one function, so `cargo build --workspace` builds something
//! and `cargo clippy --workspace --all-targets` lints something. No doc tests (`doctest = false`),
//! so libtest's `--list` counts the suite in `tests/suite.rs` and nothing else.

/// Add two numbers.
#[must_use]
pub const fn add(a: u32, b: u32) -> u32 {
    a + b
}
