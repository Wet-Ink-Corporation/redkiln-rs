//! The AC-329 decoy's STDERR half (s7c §7c.14, §7c.17). Cargo echoes every `cargo:warning=` line
//! on stderr as `warning: rk-toolchain-rust-fixture@0.0.0: <text>` on EVERY build of this package
//! -- the adapter's `--list` passes included. So the prose summary line and a per-test `... ok`
//! line are on the adapter's stderr file every time it runs cargo here. The STDOUT half -- the
//! stream the adapter reads its `: test` lines from, where a summary-line parser would naturally
//! look -- is `tests/zz_decoy.rs`, a `harness = false` test target [measured 2026-09-18: cargo's
//! own summary lines on stdout say `2 passed`, so a stderr-only decoy did not make a stdout parser
//! observable]. On either stream a summary-line parser reports 99, an unanchored `... ok`
//! line-counter counts the phantom, and only libtest's `--list` reports 2.
fn main() {
    println!("cargo:warning=test result: ok. 99 passed; 0 failed");
    println!("cargo:warning=test phantom ... ok");
    println!("cargo:rerun-if-changed=build.rs");
}
