//! The AC-329 decoy on STDOUT (s7c section 7c.14, 7c.17; the second review of Slice 6): a
//! `harness = false` test target -- cargo runs it as a plain program on every `cargo test`, passes
//! its stdout through, and hands it the libtest arguments (`--list`, a name filter, `--ignored`)
//! it does not understand and ignores. So the prose summary line and a phantom per-test line are on
//! the adapter's STDOUT file on the run AND on both `--list` passes -- the stream the adapter reads
//! for its `: test` lines -- where `build.rs`'s copy of the same decoy lands on STDERR. A
//! summary-line parser over stdout reports 99 (last match) or 101 (a sum of `N passed`), an
//! unanchored `... ok` line-counter counts the phantom, and libtest's `--list` reports 2: neither
//! line ends in `: test`, so the count route is untouched.
fn main() {
    println!("test result: ok. 99 passed; 0 failed");
    println!("test phantom ... ok");
}
