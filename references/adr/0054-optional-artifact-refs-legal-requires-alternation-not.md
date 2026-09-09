---
id: "0054"
title: Optional artifact refs are legal; alternation inside requires is not
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-24
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 24)
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

The §9a.10 input-source grammar already allows `toolchain:<feed>[?]` as an optional feed reference. The P9b author's ambiguities included whether the equivalent optionality marker is legal on an `artifact:Kind` input source, and separately whether `requires` (an activity's declared artifact-kind dependencies, §1/§7) can express an "either of these kinds" alternation.

## Decision drivers

- `requires` is part of an activity's declared public surface (§1) and states what must exist before it can run; a disjunctive entry would leave that set indeterminate, where the rest of the schema keeps its declared vocabularies closed and its entries specific.
- An optional input reference (`?`) already has defined null semantics under §9a.2 (`null` on absence, vacuous truth in quantifiers); extending that same marker to `artifact:Kind?` inputs is a narrow, already-understood extension, not a new semantic category.

## Considered options

### Option — allow alternation in requires (e.g. "requires A or B")
Would require selection logic to reason about disjunctive dependency satisfaction rather than a flat required-kind set; the source treats this as the rejected half of the amendment with no further alternative construction proposed.

## Decision

`artifact:Kind?` is legal as an optional input reference, resolving to `null` under §9a.2 semantics when absent. Alternation in `requires` (an activity declaring that one of several artifact kinds satisfies its dependency) is not legal.

## Consequences

**Positive**
- Steps can read an artifact input that may not exist yet (or ever, depending on rigor/branch) without the author having to route around the lack of an optional-input marker for artifact sources specifically.

**Negative**
- An activity that could genuinely be satisfied by either of two artifact kinds cannot express that in `requires` directly; the author must either pick one canonical required kind or restructure as separate activities/variants.

**What this forbids**
- `requires` cannot contain a disjunctive/alternation form ("this or that kind") — every entry in `requires` names one specific artifact kind, full stop.

## Evidence and confidence

[documented]: adjudicated in the source (§9b.24); no measurement or spike is cited for either half of this decision.
