---
id: "0060"
title: Boolean facets are applicable_if-only; tagging one is a lint error
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-30
source: references/2026-09-07-activity-graph-schema.md §9c amendment 30
supersedes: []
superseded_by: []
related: ["0030"]
evidence: []
consumers: []
---

## Context and problem

PC8 (ADR 0030) established facets as closed, enumerated vocabularies and named `auth`, `data-migration`,
`external-dep`, `ui`, `market-facing` as boolean risk facets among its initial facet set, alongside PC8's tag-authoring
sugar that compiles `tags = [...]` into a conjunction in `applicable_if`. The P6 authoring batch found this
underspecified for boolean facets specifically: nothing said whether a boolean facet could be written as a tag (e.g.
`auth`) the way an enumerated facet value can. The source does not record whether any author in the batch actually wrote
one that way.

## Decision drivers

- PC8 rule 3 already holds that facets filter admissibility only and never trigger "run all X" behaviour; a boolean
  facet written as a bare tag risks reading as exactly that kind of trigger.
- Consistency with PC8's existing tag-to-`applicable_if` compilation model favours a single, unambiguous way to write a
  boolean-facet condition.

## Decision

Boolean facets are **`applicable_if`-only**. A tag on a boolean facet is a lint error. This was decided alongside a
rejection of "boolean tags" as a named alternative in the same P6 adjudication pass.

## Considered options

### Option — boolean tags (rejected)
Allow a boolean facet to be written as a bare tag (e.g. `auth`) the same way an enumerated facet value can, compiling to
a truthy check in `applicable_if`. Explicitly rejected in the same adjudication pass ("Rejected: boolean tags (30)").
The source gives no further rationale beyond the rejection itself, and none is invented here.

## Consequences

**Positive**
- Removes an ambiguous authoring path (tag vs. `applicable_if`) for the boolean risk facets PC8 already named (`auth`,
  `data-migration`, `external-dep`, `ui`, `market-facing`), leaving exactly one way to condition on them.

**Negative**
- Authors who are used to PC8's tag-as-sugar convention for enumerated facets must remember that boolean facets are an
  exception to it.

**What this forbids**
- A boolean facet may not appear in an activity's `tags` list; lint rejects it. It must be expressed as a condition in
  `applicable_if` instead.

## Evidence and confidence

- [documented] No finding id or spike is cited; the row records the decision and its explicit rejection of the boolean-
  tags alternative, without a measured basis. No evidence is invented here.
