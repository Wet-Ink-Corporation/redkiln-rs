//! `rk-adapter-sqlite`'s ONE integration binary (ADR-0083; `[[test]] name = "it"`), a `mod` per
//! case cluster (s7c §7c.13, §7c.18):
//!
//! | module | tests | criterion |
//! | --- | --- | --- |
//! | `conformance` | `port_conformance_real_row_sqlite`, `wal_and_busy_timeout_are_enforced` | AC-499's SQLite real row over `rk_ports::conformance::conform_event_store` (RT-16), plus the two clause-7 settings the real row adds |
//! | `store` | `ac364_correction_appends_never_mutates`, `ac365_missing_store_file_recreated`, `ac368_sqlite_append_error_is_typed_not_panic` | the append-only store halves |
//! | `identity` | `ac369_identity_mismatch_refuses_open`, `open_refuses_a_repository_with_no_commit` | a stranger's store is refused and nothing is written; a repository with no root commit gets no store file |
//! | `worktrees` | `ac370_one_store_per_common_dir` | three worktrees, one store file, under the real `--git-common-dir` |
//! | `registry` | `ac489_capability_registry_dated`, `ac492_unknown_capability_token_fails` | the dated registry rows behind `snapshot` / `rebuild` (D17-9, RT-24) |
//! | `replay` | `ac243_gate_open_replay_over_sqlite_store` | the kernel's open-gate fixture over the real store |
//!
//! The three-worktree repository builder lives where s7c §7c.17 places it,
//! `tests/fixtures/worktrees/`, and is compiled into this binary by path.

mod conformance;
mod identity;
mod registry;
mod replay;
mod store;
mod support;
#[path = "../fixtures/worktrees/three_worktrees.rs"]
mod three_worktrees;
mod worktrees;
