# Crate charter: `rk-containment`

## Purpose
Process containment for every child the workspace spawns: a Windows Job Object, a POSIX process group, and a token-less bounded run (ADR-0127). The full account is this crate's `src/lib.rs` module doc and design §9.1; this charter does not restate them.

## Layer
infrastructure (library).

## Allowed dependencies
- Workspace crates: none (from `Cargo.toml`).
- Third-party: . Every pin and its reason is in the root `Cargo.toml` (ADR-0100 decision 2).
- Never: tokio, `rk-types`, `rk-ports`, `rk-adapter-*`, `rk-kernel`.

## Public surface
Workspace-internal, never published (`publish = false`). The deliberate surface is what `lib.rs` re-exports (inferred, confirm).

## Invariants
- Owns the workspace's one commented `#![allow(unsafe_code)]` module (`ffi`), ADR-0102 decision 2's exception relocated by ADR-0127 decision 4.
- Depends only on `std`, `windows-sys` (Windows) and `libc` (Unix): no tokio, no `rk-types`, no `rk-ports`, no `rk-adapter-*` (RT-15).
- On POSIX a holder started with its own process group is outside `killpg`'s reach, and ADR-0127 forbids the PID walk that would close it; `cancel_reaps_process_tree` measures that gap.

## Errors
See the error types named in `lib.rs` and the ADRs it cites (inferred, confirm).

## Test strategy
Integration tests in `tests/it/`; unit tests beside the code. Run with `cargo test -p rk-containment` (CI runs `cargo nextest run` through `ci.yml`).

## Do not
- Change anything an ADR or acceptance criterion above fixes without the record that amends it (the runbook's halt conditions apply).

## Decisions
| Date | Decision | Rule | Decided by |
|---|---|---|---|
