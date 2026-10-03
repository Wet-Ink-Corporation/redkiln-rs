# Crate charter: `rk-adapter-toolchain-rust`

## Purpose
The real Rust `Toolchain` row (ADR-0094 decision 1): `build` = `cargo build --workspace`, `test` = `cargo test --workspace` or `cargo nextest run` where present, recorded on the report. The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
adapter.

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-ports`, `rk-git`, `rk-containment` (from `Cargo.toml`).
- Third-party: `serde`, `serde_json`, `cvss`, `thiserror`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: `rk-kernel`, `rk-store`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports (inferred, confirm).

## Invariants
- Implements the same `Toolchain` port as `rk-adapter-toolchain-node`.
- Every child it spawns goes through `rk-containment` (inferred, confirm).

## Errors
See the error types named in `lib.rs` and the ADRs it cites (inferred, confirm).

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-adapter-toolchain-rust` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
