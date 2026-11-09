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
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- Never depends on `rk-containment` or any `rk-adapter-*` (ADR-0127 decision 3; Checkpoint 7c).
- Unbuilt steps are not stubbed: "powerless is reported as powerless" (golden constraint 10).

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-kernel` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
| 2026-11-08 | Add `runtime::reference::{parse_selection, check_selection_token, SelectionRefusal}`: the one selection parser refuses a multiline, control, shell-syntax, filterless or empty command by name, and `selection_of` is its `.ok()`; the `ReproPlacement` decode arm (ADR-0204, proposed; D21-174) | ERR-01, API-02 | Ryan Britton (plan D4, D5, D12, 2026-11-08) |
| 2026-11-09 | Add `runtime::reference::{SelectionFeed, FeedRefusal, selection_feeds, check_feeds}` and `StepAt::feeds`: an agent step's mandatory plain `${<agent>.outputs…}` fields that feed a `toolchain:test` selection are checked through `parse_selection`/`check_selection_token` before commit, and a refusal is a `Schema` failure; every `Schema` re-prompt carries the reserved input `refused` (no shipped step declares it); `Runtime::end_walk` appends one `error_row` with `step_id` and `activity_id_at_version` when the walk ends at a non-agent step (ADR-0205, proposed; D21-177) | ERR-01, API-02, ERR-04 | Ryan Britton (plan D5, D6, D7, D8, 2026-11-08) |
