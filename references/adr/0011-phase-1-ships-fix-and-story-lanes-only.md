---
id: "0011"
title: Phase 1 ships fix and story lanes only
status: accepted
date: 2026-09-05
legacy_id: D11
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

Phase 1 (§7) has to stand up the kernel, store, and adapters on a fresh repo and reach self-hosting before any further work is trusted. The proposal needed to fix which process packs exist at that point, because the exit criterion for Phase 1 is that "Phase 2's stories are authored under it" — the pack set has to be sufficient for the programme to run its own fix lane and author its own stories, and no more than that.

## Decision drivers

- Phase 1's exit criterion (§7) requires the system to run its own fix lane end to end and to have Phase 2's stories authored under Redkiln-RS.
- Remaining process packs (incident, spike, initiative, project) are explicitly deferred to Phase 2 (§7), so Phase 1 does not need them to hit its own exit bar.

## Decision

Phase 1 ships exactly two process packs: `bug-standard` and `story`, "so Phase 2's own stories are authored under Redkiln-RS."

## Considered options

### Option — fix lane only

Ship only `bug-standard`. Named in the row's "Alternatives considered" and rejected there without a stated reason; the reason reconstructed here from §7 is that it would let the programme dogfood its own fix workflow but would not let Phase 2's stories be authored under the new system, which is half of Phase 1's exit criterion.

### Option — full set

Ship all process packs (incident, spike, initiative, project) in Phase 1. Named in the row's "Alternatives considered" and rejected there without a stated reason; the reason reconstructed here from §7 is that it is more than Phase 1's exit bar needs — §7 places these packs in Phase 2's scope alongside the remaining toolchain adapters and the project DAG.

## Consequences

**Positive**
- Phase 1 scope is bounded to exactly what its own exit criterion requires, keeping the phase small enough to self-host.

**Negative**
- Any process not covered by `bug-standard` or `story` (incidents, spikes, initiatives, projects) has no home until Phase 2.

**What this forbids**
- Phase 1 may not be declared complete by running any pack outside `bug-standard`/`story`, nor by claiming self-hosting through the full pack set — the exit bar is specifically "runs its own fix lane end to end (self-hosting) and Phase 2's stories are authored under it" (§7).

## Evidence and confidence

- [documented] The choice — `bug-standard` + `story`, "so Phase 2's own stories are authored under Redkiln-RS" — and both alternatives are direct source text in the decision table (§9, D11).
- [documented] The Phase 1 scope line ("`bug-standard` and `story` process packs (D11)"), the Phase 1 exit criterion, and the Phase 2 placement of the remaining process packs are direct source text in the phasing table (§7).
- [inferred] The reasons the two alternatives lost are not stated in the source row; they are reconstructed here from §7's exit criterion and scope split.
- The source row cites no finding, spike, or measurement for D11, and no number is attached or invented here. This decision therefore does not license any claim that two packs were shown sufficient by evidence, nor that the deferred packs are harder or less valuable — only that §7's Phase 1 exit bar does not require them.
