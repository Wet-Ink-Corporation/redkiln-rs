---
id: "0059"
title: Registries are one table per value; a verb's live flag implies memo = false
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-29
source: references/2026-09-07-activity-graph-schema.md §9c amendment 29
supersedes: ["0043"]
superseded_by: []
related: ["0064"]
evidence: []
consumers: []
---

## Context and problem

`registry.toml` (introduced for toolchain verbs/feeds/timeouts by the P9b lint v2 work) had no fixed shape, and the
per-step `memo = false` declaration from amendment 13 (§9b) put the burden of noticing "this verb reads live state" on
every author of every step that used that verb, individually, forever. The P6 authoring batch needed a shape that lets a
registry addition carry provenance (a date) and lets a verb's type declare its own liveness once, rather than being
re-declared at every call site.

## Decision drivers

- A registry addition should carry a date, so `registry.toml`'s history is auditable.
- Verbs need a declared output type, not just a name, now that `emit` composition (amendment 34) and tool-step typing
  depend on knowing a verb's output kind.
- The per-step `memo = false` burden from amendment 13's second sentence is a duplication risk: every step using a
  live-reading verb must remember to declare it, and nothing catches a forgotten declaration.

## Decision

Registries move to **one table per value** — e.g. `[verb.bisect] outputs = "BisectReport" live = true timeout =
"test_timeout"` — so that each addition carries a date and each verb carries a type. `live = true` on a verb now
**implies `memo = false` on every step using that verb**, superseding the per-step declaration that amendment 13's
second sentence required (§9b.13: "`tool` steps that read live state (telemetry, flags, clocks) declare `memo =
false`"). The liveness fact moves from the call site to the registry entry.

## Considered options

### Option — do nothing (keep per-step memo = false declarations, no table-per-value shape)
Keep declaring `memo = false` at each step that happens to call a live-reading verb, and keep whatever ad hoc shape
`registry.toml` had from the P9b lint v2 work. Rejected implicitly by adopting the table-per-value shape and the `live`
flag — the source gives no separate name to this alternative, but the decision is a direct replacement of it.

## Consequences

**Positive**
- A verb's liveness is declared exactly once, in the registry, instead of at every step that calls it — removing a
  duplication and forgetting risk.
- Registry entries carry a date and a type, making `registry.toml` auditable and letting tooling (e.g. amendment 34's
  tool-step typing) check a step's `outputs` against its verb's declared `outputs`.

**Negative**
- Amendment 13's second sentence is now partially withdrawn: authors who memorized "declare `memo = false` on live
  tool steps" must instead look up the verb's registry entry. The lint rule from amendment 13 ("a loop-back target with
  neither a feedback ref nor `memo = false` is an error") still applies, but is now satisfied by the registry's `live =
  true` rather than a per-step declaration.

**What this forbids**
- A step may no longer declare `memo = false` as its own way of asserting liveness for a verb the registry has not
  marked `live = true` — liveness is a registry property of the verb, not a per-step assertion the author can attach
  independently.
- Registries may not mix shapes; the table-per-value form applies uniformly so that additions carry a date and verbs
  carry a type.

## Evidence and confidence

- [documented] No finding id or spike is cited for this specific registry-shape choice; it is a structural adjudication
  from the P6 batch, not a measured result. No evidence is invented here.
- This decision explicitly supersedes the second sentence of amendment 13 (ADR 0043) only — amendment 13's first
  sentence and its lint rule (a loop-back target needs a feedback ref or a live-implied `memo = false`) remain in force
  and are recorded as still-accepted on ADR 0043, with the partial withdrawal noted there.
