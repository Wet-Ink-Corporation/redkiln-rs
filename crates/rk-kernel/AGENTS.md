# Crate charter: `rk-kernel`

## Purpose
The workflow side: the selection engine, the ADR-0103 predicate grammar and its three-valued evaluator, the registry lints, the memo, the budgets and the replay (Step 7b). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
application (workflow).

## Allowed dependencies
- Workspace crates: `rk-types`, `rk-ports`, `rk-store`, `rk-git` (from `Cargo.toml`). `rk-adapter-edge-fixture` appears only behind the non-default `adapter-edge-reintroduce` feature, the must-fail fixture that proves the adapter-edge assertion fires.
- Third-party: `serde`, `serde_json`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: `rk-containment`, `rk-adapter-*`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports (inferred, confirm).

## Invariants
- Never depends on `rk-containment` or any `rk-adapter-*` (ADR-0127 decision 3; Checkpoint 7c).
- Unbuilt steps are not stubbed: "powerless is reported as powerless" (golden constraint 10).

## Errors
See the error types named in `lib.rs` and the ADRs it cites (inferred, confirm).

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-kernel` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
