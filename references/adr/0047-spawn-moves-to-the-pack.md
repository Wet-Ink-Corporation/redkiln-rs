---
id: "0047"
title: Spawning a follow-on initiative is a pack rule, not an activity behaviour
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-17
source: references/2026-09-07-activity-graph-schema.md §9b amendment 17
supersedes: []
superseded_by: []
related: ["0014", "0024", "0028"]
evidence: []
consumers: []
---

## Context and problem

The process-catalog proposal's incident-lane example (§3 "Worked examples") has closeout, at `mandatory` rigor, calling `spawn_followup` to create a problem-record initiative directly (ITIL incident/problem split): "`spawn_followup` creates the problem-record initiative." That put a requirement-level decision — when a new initiative gets created, and of what kind — inside the activity's own behaviour, where PC2 (ADR 0024) had already ruled that only admissibility fields (`provides`, `requires`, `applicable_if`, `method_rank`, `min_rigor`) belong; requirement (`baseline` + `require` rows) belongs to the process pack.

## Decision drivers

- PC2's rubric-home split: admissibility lives in the activity, requirement lives in the pack, and `redkiln explain` prints both from their separate homes.
- D14 (ADR 0014): process packs own structure (stages, activities, roles, slots, tier per role); an activity contributing a structural decision like "spawn a new initiative" is exactly the boundary D14 draws against.
- PC6's encapsulation contract: an activity's public surface is what it materialises, with attached evidence — not side effects on the initiative graph.

## Considered options

### Option — spawn_followup on the activity (status quo in the incident example)
Let the closeout activity itself decide, via `spawn_followup`, that a `Postmortem` implies creating a new problem-record initiative. This is the form the process-catalog proposal's incident example used before this amendment. It is rejected because it lets an activity make a requirement-level decision (what gets spawned, and when) that PC2 assigns to the pack, and because it gives the activity a side effect — creating a new initiative — outside the "materialise an artifact with evidence" contract PC6 sets for it.

## Decision

"Not on the activity (PC2: policy lives in the pack). The activity provides `ProblemRecord`; the pack declares `[[stage.closeout.spawn]] when = "exists(ProblemRecord)" kind = "initiative"`, which replaces the proposal's `spawn_followup`."

## Consequences

**Positive**
- Keeps the activity's public surface limited to producing artifacts with evidence (PC6), with no initiative-graph side effect hidden inside it.
- Puts the spawn condition where PC2 already puts every other requirement decision (`require` rows, `baseline`) — inspectable and editable in the pack alongside them, not buried in an activity's TOML.

**Negative**
- Introduces a new pack-level TOML construct, `[[stage.<stage>.spawn]]`, that the source names but does not further specify (its lint treatment, if any, is not given here).

**What this forbids**
- Forbids an activity from declaring `spawn_followup` or any equivalent construct that directly creates a new initiative; an activity may only *provide* the artifact (e.g. `ProblemRecord`) that a pack-level spawn rule conditions on.
- Forbids the pack's spawn rule from running on anything other than a condition over provided artifacts (`when = "exists(ProblemRecord)"`) — it is a requirement rule in the same family as `require`, not an arbitrary hook.

## Evidence and confidence

- [documented] The prior `spawn_followup` form is quoted directly from the process-catalog proposal's incident-lane example in §3 "Worked examples" ("Closeout at `mandatory` requires `Postmortem` and `ProblemRecord`, and `spawn_followup` creates the problem-record initiative (ITIL incident/problem split)"); the replacement is stated normatively in §9b amendment 17.
- [inferred] No finding id or spike is cited for this specific reassignment; it is adjudicated directly from the PC2/D14 boundary already established elsewhere in the corpus, not from a new measurement.
