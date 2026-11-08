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
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- Implements the same `Toolchain` port as `rk-adapter-toolchain-node`.
- Every child it spawns goes through `rk-containment`.
- *(dated 2026-11-08, s8 repro-pair route; ADR-0204, proposed.)* The `repro_place` verb (`src/verbs/repro.rs`) appends the item's committed repro pair to `crates/<crate>/tests/it/<module>.rs` or `crates/<crate>/src/<file>` under the project root and nowhere else: the target must already exist as a regular file whose canonical path stays under the canonical root, the read and the result are capped at the landing bound (`rk_git::landing::MAX_LANDED_BLOB_BYTES`), a re-entry writes nothing, and a precondition refusal is `Unsupported`. It spawns nothing. The `crates/<crate>/tests/it`, `--test it` and `--lib` spellings are Rust facts and live here, never in the kernel.

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-adapter-toolchain-rust` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
| 2026-11-08 | Add the `repro_place` verb (registry row `toolchain`, side-effecting) so the fix lane reproduces from the committed repro pair (ADR-0204, proposed; plan D1, D12) | H-16, ARC-02 | Ryan Britton |
