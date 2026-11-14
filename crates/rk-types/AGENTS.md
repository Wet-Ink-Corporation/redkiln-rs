# Crate charter: `rk-types`

## Purpose
The typed core: ids, the typed artifacts (`Spec`, `ImplReport`, `Findings`, `DocSet`, digests), schema derivation and the closed-output transform, `StepError`, `Drift`, `GateBasis`. The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
domain.

## Allowed dependencies
- Workspace crates: none (from `Cargo.toml`).
- Third-party: `serde`, `serde_json`, `schemars`, `sha2`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: any `rk-*` crate, any async runtime.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- No async runtime: no `tokio`, `async-std`, `futures` or `async-trait` (ADR-0076).
- The S-BIN-blocked `Binary` arm of `ChangeSummary` is not built here; `ChangeSummary` lives only in `rk-ports` (ADR-0165).

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-types` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
| 2026-11-08 | Add `repro_pair` (the `.repro.toml` DTO, the validated `ReproPair`, the `ReproPairInput` envelope, `ReproPairError`) and `verb_output::ReproPlacement`; manual `Display`/`Error`, no `thiserror` (ADR-0204, proposed; plan D12) | H-16, API-03, ERR-01 | Ryan Britton |
| 2026-11-14 | `artifact::lane::DebtItem { title, location }`; `DebtRegisterEntry.items` is `Vec<DebtItem>` (was `Vec<Text>`), so `debt-logging`'s `item_create` can file each item (D21-179; schema snapshot moved) | H-16 | Ryan Britton (card 2026-11-14) |
