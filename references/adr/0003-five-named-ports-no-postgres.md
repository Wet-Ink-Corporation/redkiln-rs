---
id: "0003"
title: Five named ports with fakes; git excluded; no Postgres adapter
status: accepted
date: 2026-09-05
legacy_id: D3
source: references/2026-09-05-architecture-proposal.md §9 (choice grounded in §4.1)
supersedes: []
superseded_by: ["0094"]
related: ["0001", "0008"]
evidence: []
consumers: []
---

## Context and problem

A hexagonal kernel needs a bounded set of variation axes or the port surface grows without limit. The proposal states the operating assumption directly: "one machine per repo, git for sharing" — so the question is which pieces of the system actually vary across deployments (harness, tracker, docs generator, toolchain, event store) versus which are load-bearing, measured, and have no named alternative (git itself).

## Decision drivers

- Principle 9 (§3): "Ports at named variation axes only. A port exists where a substitute has been named (Codex, Linear, Astro, .NET/UE) or where the fake alone pays for it (EventStore); nothing else is abstracted. Git is not a port."
- Git's behaviour (trailers, worktrees, shallow clones for review, `reconcile()`) is measured and load-bearing (F12, F19), and no alternative VCS is named anywhere in the corpus read for this ADR.
- The EventStore port has no real alternative adapter planned ("no Postgres adapter") but is kept as a port because the fake (in-memory Vec) is needed for tests and because a future safety net is worth the trait boundary even without a second real adapter — the D3 row's own wording: "the EventStore port exists for testing and future safety only."
- The "one machine per repo, git for sharing" assumption removes the multi-writer concurrency case that would otherwise justify a networked event store (Postgres) today.

## Considered options

### Option — Postgres adapter for a fleet

Rejected, and not scheduled in any phase: §4's diagram annotates the port row "no Postgres adapter planned", and §7's Phase 4 scope names only the Codex, Linear and Astro adapters. A Postgres adapter would serve a fleet of machines sharing one event store, which contradicts the "one machine per repo, git for sharing" operating assumption this decision states outright. The row is explicit that "no Postgres adapter" is part of the decision, not an omission.

## Decision

The kernel exposes exactly five ports, each with one real adapter named for Phase 1 and one fake, per §4.1's table:

| Port | Real adapter (Phase 1) | Named alternatives | Fake |
| --- | --- | --- | --- |
| `EventStore` | SQLite (WAL, `busy_timeout`, at `<git-common-dir>/redkiln/`) | none planned | in-memory Vec |
| `Harness` | Claude Code (`claude -p`) | Codex, Pi | scripted responder |
| `Tracker` | GitHub | Linear, Jira | in-memory board |
| `DocsSite` | mdBook | Astro/Starlight, rustdoc, DocFX | no-op recorder |
| `Toolchain` | Node (vitest/pnpm) | .NET, Python, Unreal, Rust | fixture-driven |

Git is deliberately not a port: trailers, worktrees, shallow clones for review, and `reconcile()` are load-bearing, measured (F12, F19), and no alternative VCS is named. The EventStore port carries no Postgres adapter — it exists for testing and future safety only, under the stated operating assumption that this is "one machine per repo, git for sharing."

## Consequences

**Positive**
- The port surface stays bounded: one trait per port, no generic parameters leak into the kernel, adapters are selected once at startup from `.redkiln/config.toml`, and capability differences live as data in the adapter's registry rather than as `if adapter == …` branches in the kernel (§4.1).
- Every port has a fake as a hard requirement, keeping the kernel testable without live adapters.
- Where an adapter lacks a capability the kernel needs, the activity fails explicitly with `StepError::Unsupported(capability)` rather than degrading silently, and the pack can route around it.

**Negative**
- The source does not quantify a cost of excluding a Postgres adapter (e.g. what a future multi-machine deployment would have to rebuild); it is recorded as a scope decision under the stated single-machine assumption, not as a measured trade-off.

**What this forbids**
- No sixth port may be added without a named substitute or a fake-only justification equivalent to EventStore's (Principle 9).
- Git may not be abstracted behind a port; no alternative VCS adapter may be built against this architecture.
- No Postgres (or other networked, multi-writer) EventStore adapter is in scope under the "one machine per repo" assumption; building one would require revisiting the operating assumption itself, not just adding an adapter.
- The kernel may not contain adapter-identity branches (`if adapter == …`); capability differences must be data in the adapter's registry.

## Evidence and confidence

- [measured] Git's trailer/worktree/`reconcile()` behaviour is measured and load-bearing (F12: `reconcile.rs` validated on 360 commits / 74 slices; F19: 174/174 lossless resume, drift 7% overall).
- [documented] The five-port table, the "no Postgres adapter" clause, and the "Git is not a port" statement are direct source text in §4.1.
- [inferred] "the fake alone pays for it" as the standalone justification for the EventStore port is the source's own reasoning, not independently re-derived here.
- This decision does not license a claim that a Postgres or multi-machine adapter is technically infeasible — only that it is out of scope under the stated single-machine operating assumption, which itself is not measured, only stated.
