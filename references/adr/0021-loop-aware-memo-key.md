---
id: "0021"
title: Loop-aware memo key inside activity graphs
status: accepted
date: 2026-09-07
legacy_id: D21
source: references/2026-09-05-architecture-proposal.md §9 (D21, "from schema §9b.13")
supersedes: []
superseded_by: ["0097"]
related: ["0022", "0043", "0059"]
evidence: ["P9b A18"]
consumers: []
---

## Context and problem

§4.2 defines the activity memoisation key as `hash(activity id, resolved inputs, card digest, injected KB atoms, toolchain version)`, computed once per activity. Once activities became internal graphs of steps (D22, PC6), that key had to move to per-*step* granularity inside the graph. A step-level key computed the same way as the activity-level key is blind to loop iteration: a `repeat`/`on_false` cycle that re-enters a step with unchanged resolved inputs (P9's `hypothesise`, P9b's `observe`) would hash identically on every pass and the memo layer would short-circuit it as a no-op, even though the step's job on a repeat pass is to read new evidence (telemetry, a hypothesis outcome) and continue learning. P9b finding A18 is cited as the case: a telemetry or hypothesis loop over inputs that look unchanged to the key is silently prevented from doing new work.

## Decision drivers

- The memo key must distinguish a genuine repeat with no new information from a loop iteration that has new information available, or memoisation defeats the loop's purpose.
- `tool` steps that read live state (telemetry, flags, clocks) cannot be made key-stable without lying about their inputs — reading live state is exactly what they are for.
- The key must remain content-addressed and computable without a human in the loop (consistent with §4.2).

## Decision

The content-addressed key from §4.2 is applied **per step** inside an activity graph, not once per activity. For any step inside a `repeat`/`on_false` cycle, the loop iteration counter enters the step's key. `tool` steps that read live state declare `memo = false`, opting out of memoisation entirely rather than being hashed on a key that cannot capture what they read. The activity-level key (the one the stage sees) hashes every card reachable in the subgraph (PC6), so the activity as a whole is still one content-addressed unit from the outside.

## Considered options

### Option — iteration-blind key
Keep the activity-level key formula unchanged and apply it verbatim at step granularity, with no iteration counter and no `memo = false` escape hatch. Rejected: "loops cannot learn" — a loop whose step key does not change across iterations is memoised into repeating its first answer, defeating the reason the loop exists (P9b A18).

## Consequences

**Positive**
- Loops that are supposed to accumulate evidence across iterations (hypothesis refinement, telemetry-driven repair) now actually do so, because each iteration gets a distinct key.
- `tool` steps reading live state get an explicit, auditable opt-out (`memo = false`) instead of a key that silently fails to capture their real inputs.

**Negative**
- Every step inside a `repeat`/`on_false` cycle re-executes on each iteration (by construction — the iteration counter guarantees a cache miss), which is more model/tool cost than a naively memoised loop, though a naively memoised loop was not doing useful work anyway.
- Authors must correctly mark live-state-reading `tool` steps as `memo = false`; a missed declaration reintroduces the stale-memo failure mode.

**What this forbids**
- A step-level memo key computed identically to the activity-level formula, with no iteration term, is not permitted for any step inside a `repeat`/`on_false` cycle.
- A `tool` step that reads live state may not rely on the ordinary content-addressed key; it must declare `memo = false`.

## Evidence and confidence

- [documented] P9b finding A18 is the cited motivating case: a telemetry/hypothesis loop over unchanged-looking inputs was a memo no-op under the iteration-blind key. The row does not give further quantitative detail (no n, no measured recall/precision figure) — only that the failure mode was observed and named.
- [documented] The schema source the row cites (§9b.13) pairs this decision with a lint rule, stated there as "a loop-back target with neither a feedback ref nor `memo = false` is an error", and names P9's `hypothesise` and P9b's `observe` as the steps it applied to. The ledger row itself states neither the lint rule nor the step names — both come from §9b.13, whose own record is ADR 0043.
- This decision's second sentence (the `tool`/`memo = false` clause) is later partially superseded by amendment 29 (ADR 0059); see ADR 0043/0059 for the narrower schema-level restatement. That does not change this record's status.
