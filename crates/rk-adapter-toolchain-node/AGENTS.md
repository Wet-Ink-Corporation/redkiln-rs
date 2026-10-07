# Crate charter: `rk-adapter-toolchain-node`

## Purpose
The real Node `Toolchain` row (ADR-0094 decision 1, s7c §7c.14), over a fixture project, never the workspace running the checkpoint. The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
adapter.

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-ports`, `rk-git`, `rk-containment` (from `Cargo.toml`).
- Third-party: `serde`, `serde_json`, `thiserror`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: `rk-kernel`, `rk-store`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- Every child it spawns goes through `rk_containment::run_bounded` under a construction-time timeout.
- The test fixture needs `pnpm install --frozen-lockfile` once before the suite (CI does this; §8j.7).

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-adapter-toolchain-node` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
