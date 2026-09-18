# lint-controls/ — the positive controls for Checkpoint 7c's absence bans (s7c §7c.17, §7c.21)

One file per `rk_absent` ban, named for it, each carrying the banned token **on a non-comment line** — checked
through the ban's own search and comment filter by `rk_control`, so a pattern that silently stops matching is
caught rather than reported as a clean tree. **None of these files is compiled**: nothing declares a `mod` over
them, they live here and never under `tests/ui/` (the trybuild glob root), and `ci.yml`'s git rule excludes the
directory by name (RT-13). The AC-362 laundering grep in `ci.yml` reads `crates/rk-adapter-claude/src/*` only,
which is why the control for it can carry the token.

| control | ban it controls | stated form (§7c.17) |
| --- | --- | --- |
| `adr0102_allow_attribute_control.rs` | `#!?\[allow\(` | both the inner and the outer form; never `allow(unsafe_code)` |
| `ac362_derive_default_control.rs` | the derive-Default walk `dd_hits` + `impl Default for` | two structs — a derive directly above `pub struct Usage {}` and one separated from `pub struct WindowReading {}` by a plain `//` line — and `impl Default for CacheCreation` |
| `adr0083_set_var_control.rs` | `set_var\|remove_var` | |
| `ac211_append_flag_control.rs` | the two append flags | both flags |
| `ac224_taskkill_control.rs` | `taskkill\|kill_tree` | |
| `ac219_result_rate_limit_control.rs` | `result\.rate_limit\|rate_limit_from_result` | |
| `ac374_f64_headroom_control.rs` | the AC-374 pattern (module-filtered) | `as_f64(` beside `f64`; its path is the filter's own negative control |
| `ac375_family_key_control.rs` | `headroom_saved\|cache_headroom\|seven_day_(opus\|sonnet)\|…` | |
| `ac362_unwrap_or_zero_control.rs` | the AC-362 laundering pattern | |
| `adr0118_poison_recovery_control.rs` | `PoisonError\|catch_unwind\|unwrap_or_else(… into_inner` | the canonical `unwrap_or_else(PoisonError::into_inner)` |
| `adr0102_anyhow_control.rs` | `\banyhow\b` | |
| `ac511_bare_flag_control.rs` | `"--bare"` | a call site that adds the flag, which `ac511_…` names |
| `ac201_deny_unknown_fields_control.rs` | `deny_unknown_fields` (module-filtered) | |
| `ac202_argv_isolation_control.rs` | `\bargv\b\|\bargs\b\|build_args\|extra_flags` (module-filtered) | `build_args` |
| `ac217_env_var_control.rs` | `std::env::var\|env::var(\|env::vars(` | |
| `adr0077_git_command_control.rs` | `Command::new("git")` | |
| `d34_pnpm_test_control.rs` | `pnpm test` | |
| `adr0102_toolchain_pin_control.rs` | `\+1\.98\.1` | |
| `adr0076_handle_block_on_control.rs` | `Handle::block_on\|current_thread` | |
| `adr0127_direct_spawn_control.rs` | `\.(spawn\|output\|status)\(\)` | a literal `.spawn()` |
| `adr0127_spawn_path_control.rs` | `tokio::process::(Command\|Child)` | |
| `ac208_prose_scrape_control.rs` | `Regex::new\|\bregex::\|from_str(… .text)` | `Regex::new(` |
| `ac329_prose_parse_control.rs` | the AC-329 prose-summary pattern | `test result: ok. 99 passed` |
| `adr0127_tokio_control.rs` | `\btokio\b` over rk-containment | |
