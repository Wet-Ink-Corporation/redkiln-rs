---
id: "0052"
title: Lint rule 3 resolves paths against the step's output schema, not just step ids
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-22
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 22)
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

§9 lint rule 3 checks that "every `${ref}` resolves". As originally scoped this reads as resolving the step-id portion of a reference (does `${hypothesise...}` name a real step), leaving unchecked whether the field path after the step id (`.outputs.summary`, or `.answer` for a gate per amendment §9b.15) actually exists on that step's declared output schema. The P9b author's ambiguities included this gap.

## Decision drivers

- A reference like `${hypothesise.outputs.summary}` is only meaningful if `summary` is a real field of `hypothesise`'s output schema; a step-id-only check would silently accept a typo'd or non-existent field path.
- Amendment §9b.15 had already given gates a distinct output shape (`answer`, not `outputs`) — rule 3 needs to know per-step-kind what a valid path looks like in order to check it.

## Considered options

### Option — do nothing (rule 3 checks step-id existence only)
Field-path typos and references to non-existent output fields would pass lint and fail only at runtime (or silently return null under §9a.2's null semantics), which is a worse failure mode than a catalog-load-time lint error. Rejected as one of the ambiguities the P9b batch was run to surface.

## Decision

Rule 3 resolves paths, not just step ids, against the step's output schema — `answer` for gates (per amendment §9b.15), the typed `outputs` schema for `agent`/`tool` steps.

## Consequences

**Positive**
- A reference to a non-existent field on a step's output is now caught at catalog-load time rather than surfacing as a silent `null` or a runtime error.

**Negative**
- Lint must carry each step kind's output schema and check every path segment against it, not merely check the step id exists — increasing lint's implementation surface (rolled into lint v2 per §9b's closing status note).

**What this forbids**
- A reference can no longer name an existing step but a non-existent field path on it and pass lint; every segment of the path must resolve against that step's declared output schema.

## Evidence and confidence

[documented]: adjudicated in the source (§9b.22); no measurement is cited. The source notes lint v2 (implementing this alongside the rest of §9b) raises 15 errors against P9b's three files (§9b closing paragraph), but that count is aggregate across all of §9b's amendments and is not attributed to this rule specifically.
