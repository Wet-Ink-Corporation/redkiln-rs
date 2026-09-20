// POSITIVE CONTROL for ADR-0083's kernel-side half: `Config` is not constructible from the
// environment INSIDE the kernel (`s7d` §7d.14's third red for
// `adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs`).
//
// Never compiled, never linked. The scan that reads `crates/rk-kernel/src/config.rs` runs over this
// file FIRST and must find the token here, or a scanner that visited nothing would report a clean
// kernel having read no kernel at all.
//
// This control exists because the ban it pairs with was MISSING from the slice that shipped the
// type, and the only thing standing in its place was a sentence in the module under test saying the
// crate reads no environment. Measured 2026-09-19: with the scan absent, inserting
// `std::env::var("RK_HARNESS_OVERRIDE")` into `Config::new` left all 181 kernel tests green and both
// Checkpoint 7b and Checkpoint 7b-ii at rc=0 — the slice's own stated proof passed the mutant.

fn a_config_built_from_the_environment(harness: &str) -> String {
    let over = std::env::var("RK_HARNESS_OVERRIDE");
    over.unwrap_or_else(|_| harness.to_owned())
}
