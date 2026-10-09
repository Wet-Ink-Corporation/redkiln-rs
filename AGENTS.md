# Redkiln-RS

A clean-break, single Rust binary (`redkiln`) that owns the loop: a deterministic kernel (stage selection, gate evaluation, budgets, retry, replay) that dispatches one harness process per activity. The design is `references/2026-09-08-architecture-design.md`, the decisions are the ADRs under `references/adr/`, and the build follows `RUNBOOK.md` and `runbook/`. Where this charter and those records disagree, the records win; this file is the workspace charter temper reads.

## Crate map

| Crate | Layer | Purpose | Depends on |
|---|---|---|---|
| `crates/rk-types` | domain | The typed core: ids, typed artifacts, schema derivation, `StepError`, `Drift`, `GateBasis`. No async runtime (ADR-0076). | serde, serde_json, schemars, sha2 |
| `crates/rk-ports` | application (ports) | The five port traits, their fakes, `CancelToken`, `ChangeSummary` (ADR-0003). Git is not a port. | rk-types |
| `crates/rk-kernel` | application (workflow) | Selection engine, predicate grammar and evaluator, registry lints, memo, budgets, replay (Step 7b). | rk-types, rk-ports, rk-store, rk-git |
| `crates/rk-store` | infrastructure | The typed store at the common dir: serialiser, memo key, identity, connection discipline, retention (AC-178). | rk-types, rusqlite |
| `crates/rk-git` | infrastructure | Git as the tier-1 store, behind the one git wrapper (ADR-0077, ADR-0098). | rk-types, gix |
| `crates/rk-containment` | infrastructure | Process containment: Job Object, process group, bounded run. Owns the one `#![allow(unsafe_code)]` module (ADR-0127). | std, windows-sys, libc |
| `crates/rk-adapter-claude` | adapter | The real `Harness` (one `claude -p` child per dispatch). | rk-types, rk-ports, rk-containment, tokio |
| `crates/rk-adapter-sqlite` | adapter | The real `EventStore` over `rk-store`. | rk-types, rk-ports, rk-store, rk-git |
| `crates/rk-adapter-toolchain-node` | adapter | The real Node `Toolchain`. | rk-types, rk-ports, rk-git, rk-containment |
| `crates/rk-adapter-toolchain-rust` | adapter | The real Rust `Toolchain`. | rk-types, rk-ports, rk-git, rk-containment |
| `crates/rk-cli` | binary | The composition root, the `redkiln` binary (design §9.1). | everything above |
| `crates/redkiln-migrate` | binary | The old-corpus converter; never a `redkiln` verb (AC-441, AC-466). | rk-types, rk-store, rk-git |

`tests/fixtures/*` are workspace members too: the AC-001 to AC-005 must-fail fixture crates, each with its offending code behind a non-default feature (s5 §5.1a). They have no charter. Dependencies point inward (ARC-02), and the boundaries the ADRs fix are asserted by tests and by `ci.yml`'s structural job. Each crate's `AGENTS.md` is its charter (ARC-08).

## Commands

| Purpose | Command |
|---|---|
| Format | `cargo fmt --all` |
| Lint | `cargo clippy --workspace --all-targets` (what `ci.yml` runs), and `node .claude/temper/temper.mjs baseline --check` for temper's ratchet: clippy, rustdoc and temper's scan against `.claude/temper-baseline.json` |
| Test | `cargo nextest run --workspace` (or `cargo test --workspace`). The node adapter's fixture needs `pnpm install --frozen-lockfile` once first (s8j §8j.7) |
| Docs | `cargo doc --workspace --no-deps` (rustdoc lints are part of `baseline --check`) |
| Supply chain | `cargo audit --deny unmaintained`, `cargo deny check bans advisories licenses` |
| Full gate (Claude Code) | `/temper:gate`, or `node "$TEMPER_CLI" gate` |

Toolchain 1.98.1 (`rust-toolchain.toml`), and `rust-version` equals the pin (ADR-0102 decision 1). Profiles are the repository's own; none came from the template (PERF-09).

**Lints.** The groups are ADR-0102 decision 5's: `clippy::all` deny, an enumerated pedantic subset at deny, nursery at warn, and never a group-level pedantic. temper's adoption added the rust-craft template's named lints at **warn**, so `ci.yml`'s clippy step (no `-D warnings`) is unchanged, while temper's ratchet fails any new finding. `unsafe_code` stays `deny`, not `forbid`, because `rk-containment` holds the one approved allow.

## Engineering standard (rust-craft)

All Rust here follows rust-craft. The full text, with rule IDs, examples, the review rubric and the process, is the `rust-craft` skill shipped in the temper plugin. The non-negotiables for production code (tests may use `unwrap`/`expect`/`assert!`):

- H-01 no `unsafe`; H-02 no `unwrap`/`expect`; H-03 no `panic!`/`todo!`/`unimplemented!`/`unreachable!`, and no std API that panics on input the code does not control; H-04 no panicking indexing or slicing, including `map[&k]` and `deque[i]`, which no lint catches.
- H-05 no `#[allow]`; `#[expect(lint, reason)]` needs human approval; never weaken lint, CI or hook config.
- H-06 no `dbg!`, no print macros (use `tracing` or `writeln!` on a locked handle), no commented-out code, no untracked TODOs.
- H-07 no `Box::leak`/`mem::forget`/`transmute`/`static mut`/`'static` to silence lifetimes; H-08 no clones to appease the borrow checker.
- H-09 typed errors in library APIs (`thiserror`), with `anyhow` only in binaries; H-10 never discard failures; H-11 no lossy `as` casts.
- H-12 never weaken tests or checks, including manifest test switches (`test`, `doctest`, `harness`, `autotests`) and `#[ignore]` or `cfg_attr(.., ignore)` without a reason; H-13 bounded and owned resources.
- H-14 no blocking in async code, and no lock guard, `RefCell` borrow, entered tracing span or `watch::Ref` held across `.await`; H-15 zero warnings, documented public API.
- H-16 new dependencies and major-version bumps, `[patch]` overrides, `rust-version` raises, API breaks, lint exceptions, crate-boundary changes, migrations and test rewrites are human decisions.

Process: explore, frame, plan (`.temper/plans/`), red, green, refactor, self-review, gate, independent review, then report with a Status block (`DONE`, `WIP` or `BLOCKED`, plus the gate result, the review verdict, what changed, decisions needed and what was not verified).

## Backlog ledger

GitHub Issues are the backlog ledger for bugs, findings, audit items, post-M5 hardening and Phases 2-4. The runbook still owns s8i, s8j and Step 8 until M5. The conventions (hierarchy, labels, the issue body contract, the definition of ready, the AFK protocol) are in `docs/BACKLOG.md`. Unattended sessions use the `backlog-ledger` skill. Never create `rk:*` labels, because that namespace belongs to the product's tracker adapter (AC-507).

## Decisions log

| Date | Decision | Rule | Decided by |
|---|---|---|---|
| 2026-10-03 | Adopt temper (pilot, temper plan 2.1): the template's named lints at warn on top of ADR-0102's groups, the existing findings recorded in `.claude/temper-baseline.json`, and a trimmed `rust.yml` that runs only temper's ratchet, report and PR comment beside `ci.yml`. The template's `dependabot.yml` is left out, because a pin bump is a deliberate PR (ADR-0100 decision 2). | H-05, H-12, H-16 | Ryan Britton |

ADRs live in `references/adr/`; dated decisions in `spikes/DECISIONS-*.md` and `runbook/PROGRESS.md`.
