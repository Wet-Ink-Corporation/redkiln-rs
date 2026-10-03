# Crate charter: `rk-ports`

## Purpose
The five port traits (event store, harness, tracker, docs site, toolchain), their fakes, the `CancelToken` and `ChangeSummary` (ADR-0003, design §9.1). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
application (ports).

## Allowed dependencies
- Workspace crates: `rk-types` (from `Cargo.toml`).
- Third-party: `serde`, `serde_json`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: adapter crates, `rk-git`, `rk-kernel`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports (inferred, confirm).

## Invariants
- Five ports and no sixth; git is deliberately not a port, and a trait matching the git surface is rejected by the denylist test (AC-483).
- Every port trait has `Send + Sync` supertraits; the composition root holds each as `Arc<dyn Port + Send + Sync>`, with the one recorded exception in `lib.rs`.

## Errors
See the error types named in `lib.rs` and the ADRs it cites (inferred, confirm).

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-ports` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
