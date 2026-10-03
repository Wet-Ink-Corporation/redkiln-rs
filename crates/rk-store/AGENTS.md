# Crate charter: `rk-store`

## Purpose
The typed store: canonical serialiser, memo key, identity, connection discipline, section locator, retention. One store per repository, at the common dir (AC-178). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
infrastructure (library).

## Allowed dependencies
- Workspace crates: `rk-types` (from `Cargo.toml`).
- Third-party: `serde`, `serde_json`, `rusqlite`. Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: adapter crates, `rk-kernel`, `rk-cli`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports.

## Invariants
- Two tiers by the deleted-database test (ADR-0109 decision 2): tier 1 is written to git first; the step memo is tier 2 (ADR-0097).
- Markdown is a rendering, never a source (ADR-0006, AC-120).
- One writer connection per process (ADR-0105 clause 7).

## Errors
See the error types named in `lib.rs` and the ADRs it cites.

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-store` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
