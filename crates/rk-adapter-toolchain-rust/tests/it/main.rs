//! `rk-adapter-toolchain-rust`'s ONE integration binary (ADR-0083; `[[test]] name = "it"`), a `mod`
//! per case cluster (s7c §7c.14, §7c.18), every test over the crate's `project/` fixture (§7c.17)
//! and never over the workspace running the checkpoint:
//!
//! | module | tests | criterion |
//! | --- | --- | --- |
//! | `conformance` | `port_conformance_real_row_rust` | AC-499's Rust real row over `rk_ports::conformance::conform_toolchain` (RT-16), count 2, the capability arm fed from the registry, a spawn marker |
//! | `report` | `ac329_report_conforms_no_prose_parse`, `ac325_recorded_argv_is_os_vector`, `ac331_killed_verify_is_unverifiable`, `ac344_missing_binary_is_not_a_candidate_failure`, `nextest_fallback_recorded_on_report`, `os_killed_run_is_keyed_by_detail_prefix`, `concurrent_verify_reports_share_no_output_file` | the `VerifyReport` from libtest's stable `--list` (RT-26) with the store's whole-suite rule on `total_test_count` (ADR-0104 d3), the OS vector and the count vectors through the wrapper fixture, a killed run never a failing test, an absent program never `NonZero`, the nextest fallback recorded, the OS-killed detail prefix, two concurrent calls sharing no output file |
//! | `registry` | `ac489_capability_registry_dated`, `ac492_unknown_capability_token_fails`, `ac328_affected_selection_unsupported_without_probe` | the dated rows behind `change_summary` (D17-9) and `affected-selection` (AC-328), RT-24 |
//! | `classify` | `ac497_classify_binary_paths` | `classify` per path, honest for non-Rust paths (ADR-0029 i), PROVISIONAL (S-BIN unrun) |
//! | `pack` | `rust_pack_fixture_binds_kinds_per_adr0029` | the Rust technology pack fixture read through `rk_kernel::toml_subset` over the dev edge (ADR-0094 d1, ADR-0029 i-iv, AC-337, AC-213) |
//!
//! **The process environment is read ONCE, at `support`'s edge** (`std::env::vars()` projected to
//! the two names the measured list carries) — the composition root's job, done here because no
//! `rk-cli` exists at 7c; `set_var` / `remove_var` are banned and never called; no adapter `src/`
//! file reads the environment (the block's AC-217 ban, whose scope is `src/`).

mod classify;
mod conformance;
mod pack;
mod registry;
mod report;
mod support;
