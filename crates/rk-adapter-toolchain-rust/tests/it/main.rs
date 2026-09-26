//! `rk-adapter-toolchain-rust`'s ONE integration binary (ADR-0083; `[[test]] name = "it"`), a `mod`
//! per case cluster (s7c §7c.14, §7c.18), every test over the crate's `project/` fixture (§7c.17)
//! and never over the workspace running the checkpoint:
//!
//! | module | tests | criterion |
//! | --- | --- | --- |
//! | `conformance` | `port_conformance_real_row_rust` | AC-499's Rust real row over `rk_ports::conformance::conform_toolchain` (RT-16), count 2, the capability arm fed from the registry, a spawn marker |
//! | `report` | `ac329_report_conforms_no_prose_parse`, `ac325_recorded_argv_is_os_vector`, `ac331_killed_verify_is_unverifiable`, `ac344_missing_binary_is_not_a_candidate_failure`, `nextest_fallback_recorded_on_report`, `os_killed_run_is_keyed_by_detail_prefix`, `concurrent_verify_reports_share_no_output_file` | the `VerifyReport` from libtest's stable `--list` (RT-26) with the store's whole-suite rule on `total_test_count` (ADR-0104 d3), the OS vector and the count vectors through the wrapper fixture, a killed run never a failing test, an absent program never `NonZero`, the nextest fallback recorded, the OS-killed detail prefix, two concurrent calls sharing no output file |
//! | `registry` | `ac489_capability_registry_dated`, `ac492_unknown_capability_token_fails`, `ac328_affected_selection_unsupported_without_probe` | the dated rows behind `change_summary` (D17-9) and `affected-selection` (AC-328), RT-24 |
//! | `change_summary` | `change_summary_text_variant_from_git_diff`, `change_summary_refuses_a_range_carrying_a_binary_path`, `change_summary_refuses_an_empty_range`, `change_summary_refuses_a_range_carrying_an_lfs_pointer_path`, `change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold` | the `Text` arm over a throwaway two-commit repository through `rk_git::diff::range_diff`, and the by-name refusals (ADR-0165; *added 2026-09-24, s8a*; the last *added 2026-09-25, s8a fix round 1*) |
//! | `classify` | `ac497_classify_binary_paths`, `classify_is_invariant_to_a_cargo_manifest`, `classify_source_spawns_nothing` | `classify` per path, honest for non-Rust paths (ADR-0029 i), PROVISIONAL (S-BIN unrun); a function of the path and its bytes, invariant to a `Cargo.toml` beside them, and spawning nothing by a code-only source lint (*the last two added 2026-09-25, s8a, D21-51; ADR-0165 decision 9*) |
//! | `pack` | `rust_pack_fixture_binds_kinds_per_adr0029` | the Rust technology pack fixture read through `rk_kernel::toml_subset` over the dev edge (ADR-0094 d1, ADR-0029 i-iv, AC-337, AC-213) |
//! | `libtest_lines` | `the_pinned_goldens_parse_to_their_recorded_counts`, `a_record_line_outside_a_block_is_never_counted`, `a_block_whose_record_count_differs_from_its_header_is_unverifiable`, `a_summary_that_disagrees_with_its_records_is_unverifiable`, `a_nonzero_exit_with_no_failed_record_is_unverifiable_never_green`, `an_unpinned_toolchain_version_is_refused_by_name`, `this_repositorys_whole_suite_stdout_parses_to_its_recorded_tally`, `a_slow_test_notice_inside_a_block_is_skipped_never_refused` | the strict libtest-line parser over the six pinned goldens and synthetic streams (s8c §8c.1, D21-43 N4, record T = ADR-0169 decision 3, D21-62 A's foreign-line skip; *added 2026-09-26, s8c*) |
//! | `nextest_lines` | `the_pinned_nextest_goldens_parse_to_their_recorded_counts`, `an_unpinned_nextest_version_is_refused_by_name`, `a_summary_that_disagrees_with_its_status_lines_is_unverifiable`, `a_nonzero_exit_with_no_fail_line_is_unverifiable_never_green`, `a_retry_or_timeout_line_is_unverifiable_never_counted` | the strict cargo-nextest status-line parser over the `0.9.146` goldens and synthetic streams (§8c.1, D21-48 X15, record T decision 4, D21-62 B's `ListFailed`; *added 2026-09-26, s8c*) |
//! | `build` | `a_compile_error_is_ok_with_status_failing`, `a_build_stream_without_build_finished_is_err` | `build`'s outcome from cargo's `--message-format=json` stream (s8c §8c.2, record T = ADR-0169 decision 8; *added 2026-09-26, s8c P5*) |
//! | `bound` | `a_step_bound_tighter_than_construction_fires_first`, `the_construction_bound_caps_a_larger_step_bound` | `min(step, construction)`, and `Timeout` naming the bound that fired (§8c.6, record B = ADR-0171 decision 4; *added 2026-09-26, s8c P5*) |
//! | `affected` | `affected_set_is_touched_crates_plus_reverse_dependency_closure`, `a_path_in_no_package_runs_the_whole_suite_with_a_reason`, `no_recorded_whole_suite_runs_the_whole_suite_probe_unverified`, `a_package_set_that_lists_no_test_escalates_selection_empty`, `a_named_selection_the_adapter_does_not_declare_is_refused_by_name` | the Rust `Affected` rules over the workspace fixture and the pure `affected_packages` (§8c.3, record T decision 7; *added 2026-09-26, s8c P5*) |
//! | `verbs` | `smoke_runs_each_checklist_row_against_the_built_binary`, `a_failing_smoke_row_is_a_failing_result_not_an_error`, `sca_scan_counts_an_unrated_advisory_as_unrated_never_zero`, `sca_scan_records_the_advisory_database_commit`, `sca_scan_refuses_a_non_git_database`, `sca_scan_refuses_a_deny_db_path_with_other_than_one_database`, `sca_scan_refuses_a_database_moved_mid_scan`, `package_digests_only_the_declared_binaries_of_a_locked_release_build`, `setup_installs_each_declared_node_project_offline`, `a_failing_setup_is_refused_naming_the_project`, `an_empty_setup_set_is_ok_with_no_projects`, `an_undeclared_verb_is_refused_by_name` | the four verb handlers behind `run_verb` and the bare-name refusal (§8c.4, record V = ADR-0170 decisions 4-7, D21-62 C; *added 2026-09-26, s8c P5*) |
//!
//! **The process environment is read ONCE, at `support`'s edge** (`std::env::vars()` projected to
//! the two names the measured list carries) — the composition root's job, done here because no
//! `rk-cli` exists at 7c; `set_var` / `remove_var` are banned and never called; no adapter `src/`
//! file reads the environment (the block's AC-217 ban, whose scope is `src/`).

mod affected;
mod bound;
mod build;
mod change_summary;
mod classify;
mod conformance;
mod libtest_lines;
mod nextest_lines;
mod pack;
mod registry;
mod report;
mod runner;
mod support;
mod verbs;
