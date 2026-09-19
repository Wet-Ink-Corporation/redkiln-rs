---
id: "0009"
title: Phase 1 human gates answer from the CLI only; tracker and web channels wait for Phase 3
status: accepted
date: 2026-09-05
legacy_id: D9
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: ["0129"]
related: ["0017"]
evidence: []
consumers: []
---

## Context and problem

§4.6 describes a human gate as "a mark, not a process": opening it writes a `Gate:` trailer and a `gate_opened` event and the process exits 0, and answering is "a durable write from any channel" that lands in the same `reconcile()`. Three candidate channels are named — CLI, a tracker comment grammar ingested through the Tracker port's inbox, and a web view's single write — and Phase 1 has to pick which of them exist on day one without yet having a Tracker port adapter or a web surface built.

## Decision drivers

- Phase 1's scope (§7) is `rk-types`/`rk-kernel`/`rk-store`/`rk-git`/`rk-ports` plus SQLite, Claude and Node adapters — no GitHub/Tracker adapter and no web view exist yet in that phase.
- Every answer channel must land in the same `reconcile()` regardless of source, so adding a channel later is additive, not a redesign, provided the mark/answer mechanism (§4.6, D17) is channel-agnostic from the start.
- Phase 3's own scope line ("surfaces") explicitly assigns "GitHub adapter + comment grammar" and "web view" to itself, with the exit criterion "a gate answered from a phone" — i.e. those channels are already scheduled for a later phase, not deferred without a plan.

## Considered options

### Option — CLI + GitHub comments
Shipping the tracker comment grammar (`/redkiln approve` via the Tracker port's inbox) alongside the CLI in Phase 1, rather than waiting for Phase 3. Rejected as the Phase 1 scope: the source's Phase 1 row does not include a Tracker/GitHub adapter, and the comment grammar is explicitly listed under Phase 3 ("surfaces") instead.

## Decision

"CLI only; tracker comment grammar and web view in Phase 3." In Phase 1, the only channel through which a human gate may be answered is `redkiln answer <item> approve|changes-requested`. The tracker comment grammar (`/redkiln approve`, ingested through the Tracker port's inbox) and the web view's single write are deferred to Phase 3, alongside the GitHub adapter and web view themselves. All channels — present and future — write into the same `reconcile()` path, so this is a phasing decision about which channels exist yet, not about the gate's internal shape.

## Consequences

**Positive**
- Phase 1 needs no Tracker port adapter or web surface to close a gate, keeping its dependency surface to SQLite, Claude and Node adapters as scoped.
- Because every channel converges on the same `reconcile()`, adding the tracker and web channels in Phase 3 is additive rather than a rework of the answer path.

**Negative**
- Anyone answering a Phase 1 gate must have CLI access to the repo; there is no comment-based or phone-based path until Phase 3.

**What this forbids**
- No tracker comment grammar or web-view write may be treated as a valid gate answer during Phase 1 — only a CLI `redkiln answer` invocation reconciles.
- Phase 1 build-out may not spend effort standing up the Tracker port's inbox or a web view in service of gate-answering; that work is explicitly Phase 3 scope.

## Evidence and confidence

- [documented] The three-channel description (CLI, tracker comment grammar via Tracker port inbox, web view single write) and their common `reconcile()` landing point come directly from §4.6.
- [documented] Phase 1's scope line names "mark-based gate, CLI answers only (D9)" explicitly; Phase 3's scope line names "GitHub adapter + comment grammar; ... web view" with exit criterion "a gate answered from a phone" (§7).
- The source cites no measurement or finding for this row — it is a phasing/scope decision, not a spike-backed one; no evidence id is available to cite. [documented] only, no [measured] claim is made here.
