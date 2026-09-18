//! `rk-adapter-toolchain-node`'s ONE integration binary (ADR-0083; `[[test]] name = "it"`), a `mod`
//! per case cluster (s7c §7c.14, §7c.16, §7c.18), every test over the crate's `project/` fixture
//! (§7c.17) — installed ONCE by `pnpm install --frozen-lockfile --dir <project>` before the block;
//! `support::preflight` refuses to run anything when `node_modules/.bin/vitest` is absent, because
//! `npx` without it would fetch or prompt on a closed stdin — and never over the workspace running
//! the checkpoint:
//!
//! | module | tests | criterion |
//! | --- | --- | --- |
//! | `conformance` | `port_conformance_real_row_node` | AC-499's Node real row over `rk_ports::conformance::conform_toolchain` (RT-16), count 2, the capability arm fed from the registry, a spawn marker |
//! | `report` | `ac329_report_conforms_no_prose_parse`, `ac325_recorded_argv_is_os_vector`, `ac331_killed_verify_is_unverifiable`, `ac344_missing_binary_is_not_a_candidate_failure`, `d34_vitest_invoked_directly_never_pnpm_forwarded`, `os_killed_run_is_keyed_by_detail_prefix`, `concurrent_verify_reports_share_no_output_file` | the `VerifyReport` from vitest's JSON reporter file with the store's whole-suite rule on `total_test_count` (ADR-0104 d3), the OS vector through the argument-dropping wrapper, a killed run never a failing test, an absent program never `NonZero`, the full vector as separate elements with the per-OS program element pinned by name and never `pnpm test --` (DECISIONS-2026-09-09 #34), the OS-killed detail prefix, two concurrent calls sharing no output file |
//! | `registry` | `ac489_capability_registry_dated`, `ac492_unknown_capability_token_fails`, `ac328_affected_selection_unsupported_without_probe` | the dated rows behind `change_summary` (D17-9) and `affected-selection` (AC-328; S-SEL-NODE owed), RT-24 |
//! | `classify` | `ac497_classify_binary_paths` | `classify` per path, honest for non-Node paths (ADR-0029 i), PROVISIONAL (S-BIN unrun) |
//! | `cancel` | `s_cancel_win_vitest_arm` | S-CANCEL-WIN's vitest arm (O-10; §7c.16): a mid-vitest victim under the job — alive mid-run by its own started/finished markers — killed by `run_bounded`'s timeout, Halt 6's own fixture tree, `TempDir::close()` after, the removal and listing lags read and the result in design §10.5's row |
//! | `ac334` (`--features corpus`) | `ac334_flip_predicate_real_toolchain` | AC-334's real row over the predecessor checkout: the F25 inversion applied with `git apply`, the suite run through the real adapter, reverted in a `Drop` guard, the checkout asserted clean, the flip predicate driven over what was measured |
//!
//! **The process environment is read ONCE, at `support`'s edge** (`std::env::vars()` projected to
//! the two names the measured list carries, `SystemRoot` and `PATH`) — the composition root's
//! job, done here because no `rk-cli` exists at 7c; `set_var` / `remove_var` are banned and never
//! called; no adapter `src/` file reads the environment (the block's AC-217 ban, whose scope is
//! `src/`).

#[cfg(feature = "corpus")]
mod ac334;
mod cancel;
mod classify;
mod conformance;
mod registry;
mod report;
mod support;
