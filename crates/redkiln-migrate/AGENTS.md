# Crate charter: `redkiln-migrate`

## Purpose
The old-corpus converter: a separate binary, never a `redkiln` verb (AC-441; ADR-0002, ADR-0013). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
binary.

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-store`, `rk-git` (from `Cargo.toml`).
- Third-party: `yaml_serde`, `serde`, `serde_json`, `jsonschema`, `clap`, `anyhow`, `tempfile`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: `rk-adapter-*`, `rk-ports`, `rk-kernel`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- Dispatches nothing: its dependency closure reaches no `rk-adapter-*`, no `rk-ports` and no `rk-kernel` (AC-466).
- Reads no environment variable and no clock (AC-474).
- The only crate that may depend on a YAML reader (`yaml_serde`).

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p redkiln-migrate` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
