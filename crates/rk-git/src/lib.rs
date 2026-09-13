//! `rk-git` — git as the tier-1 store: trailers, `reconcile()`, worktrees, disposable clones and
//! the merge queue, split by direction (ADR-0077, as ADR-0098 amends it).
//!
//! **At s6 §6.1 this crate is a spike harness and nothing more.** It carries the one process-exec
//! wrapper and the three S-GIX arms, because each arm's oracle is the `git` executable and
//! ADR-0077:70 puts every git invocation in the workspace behind a single wrapper type: *"All
//! shelling out goes through one wrapper type inside `rk-git`. No `std::process::Command` for git
//! exists anywhere else in the workspace."* A test inside this package is not inside that type,
//! so the wrapper has to exist before the arms can run.
//!
//! What is deliberately ABSENT until §6.2, each because it is that step's own red test: the
//! `core.hooksPath` and `--if-exists=add` defaults (ADR-0105 clauses 2 and 3), the `\?\`
//! normaliser (clause 6), the single `tree_hash` derivation (clause 4), the journalled `GitOps`
//! seam (clause 1), `reconcile()` and `Reconciled<State>` (ADR-0109), and the worktree lifecycle.
//!
//! No async runtime: ADR-0076 confines `tokio`/`async-std`/`futures`/`async-trait` to adapters,
//! and this crate is not one. No `anyhow` either — that is confined to `rk-cli` and
//! `redkiln-migrate` (ADR-0102 decision 6), and CI asserts it over `cargo metadata`.

pub mod clone;
pub mod error;
pub mod git_exec;
pub mod gitops;
pub mod path;
pub mod reconcile;
pub mod reconciled;
pub mod trailer;
pub mod tree;
pub mod version;
pub mod worktree;
