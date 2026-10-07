# Crate charter: `rk-adapter-claude`

## Purpose
The real `Harness` row (ADR-0094 decision 1): one `claude -p` child per dispatch, spawned through `rk-containment`, its stream consumed inside, every termination appending one `dispatch_completed` row (ADR-0099, ADR-0126). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
adapter.

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-ports`, `rk-containment` (from `Cargo.toml`).
- Third-party: `tokio`, `serde`, `serde_json`, `jsonschema`, `thiserror`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: `rk-kernel`, `rk-store`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- Constructed once at the composition root, never per call (design §9.1).
- The kernel never calls cancel; the `CancelToken` is cloned in at construction (ADR-0099).
- tokio is a dependency of this crate only, confined behind sync port methods (ADR-0076, RT-15).

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-adapter-claude` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
