# Crate charter: `rk-git`

## Purpose
Git as the tier-1 store: trailers, `reconcile()`, worktrees, disposable clones and the merge queue (ADR-0077 as ADR-0098 amends it). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
infrastructure (library).

## Allowed dependencies
- Workspace crates: `rk-types` (from `Cargo.toml`).
- Third-party: `gix`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: adapter crates, `rk-kernel`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports (inferred, confirm).

## Invariants
- All shelling out to git goes through one wrapper type in this crate; no `std::process::Command` for git exists anywhere else (ADR-0077:70, ADR-0105 clause 5).

## Errors
See the error types named in `lib.rs` and the ADRs it cites (inferred, confirm).

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-git` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
