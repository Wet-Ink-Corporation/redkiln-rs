# Crate charter: `rk-adapter-sqlite`

## Purpose
The real `EventStore` row over `rk-store` (ADR-0094 decision 1, s7c §7c.13). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
adapter.

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-ports`, `rk-store`, `rk-git` (from `Cargo.toml`).
- Third-party: `serde`, `serde_json`, `thiserror`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: `rusqlite` directly, `rk-kernel`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports (inferred, confirm).

## Invariants
- Consumes `rk-store`'s connection discipline and opens no second writer connection (ADR-0105 clause 7); `rusqlite` is not a dependency of this manifest.
- The on-disk layout is `rk-store`'s, applied unchanged (AC-371).

## Errors
See the error types named in `lib.rs` and the ADRs it cites (inferred, confirm).

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-adapter-sqlite` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
