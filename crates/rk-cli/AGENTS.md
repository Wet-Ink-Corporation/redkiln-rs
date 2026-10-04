# Crate charter: `rk-cli`

## Purpose
The composition root: the `redkiln` binary, and the library its integration tests link (ADR-0083). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
binary.

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-ports`, `rk-kernel`, `rk-store`, `rk-git`, `rk-containment`, `rk-adapter-claude`, `rk-adapter-sqlite`, `rk-adapter-toolchain-node`, `rk-adapter-toolchain-rust` (from `Cargo.toml`).
- Third-party: `clap`, `anyhow`, `toml`, `ctrlc`, `serde`, `serde_json`, `tempfile`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: none (the composition root). No crate may depend on `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- The one environment read (`env`), the one wall-clock read (`clock`) and the one store acquisition (`store`) live here.
- Exits through the closed exit table in `exit`.

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-cli` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
