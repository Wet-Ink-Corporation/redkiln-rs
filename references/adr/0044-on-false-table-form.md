---
id: "0044"
title: on_false is a bounded table, never a bare scalar target
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-14
source: references/2026-09-07-activity-graph-schema.md §9b amendment 14
supersedes: []
superseded_by: []
related: ["0031", "0042"]
evidence: ["lint v2 first run, spikes/p9/results-v2-catalog-run1.txt (12 of 25 errors labelled `on_false` form; not broken down by rule)"]
consumers: []
---

## Context and problem

§9 lint rule 1 (amendment 1, ADR 0031) requires every `repeat`/`on_false` back-edge to be "declared and bounded... each with a finite `max`." `repeat` already carries its bound in `{ until, max }`. `on_false`, left as a bare step-id target, has nowhere to put that `max` — the scalar form satisfies "declared" but cannot satisfy "bounded."

## Decision drivers

- Lint rule 1's requirement applies to `on_false` back-edges exactly as it applies to `repeat`; a construct that cannot carry a finite `max` cannot be lint-checked against that rule at all.
- Uniform bounded-loop treatment across the two looping constructs (`repeat`, `on_false`) keeps lint rule 1's acyclic-except-through-bounded-back-edges check tractable — both back-edge kinds then carry the same shape of bound.

## Considered options

### Option — scalar on_false (bare step-id target)
`on_false = "hypothesise"`, naming only the target step. This is the form the amendment replaces; the source states directly why it lost: "the scalar form is an error (rule 1 demanded a finite `max` with nowhere to put it)." A scalar has no field to hold the bound rule 1 requires.

## Decision

"`on_false = { to = "<earlier step>", max = N }`; the scalar form is an error."

## Consequences

**Positive**
- Every `on_false` back-edge now carries the same finite-bound evidence `repeat` already carries, so lint rule 1 applies uniformly to both looping constructs.
- Closes a structural gap rather than a one-off ambiguity: the scalar form was not merely undocumented, it had no place to satisfy a rule already on the books.

**Negative**
- Adds table syntax (`{ to = ..., max = ... }`) where a bare string previously sufficed, and requires a second lint check (rejecting the now-illegal scalar form) on top of the existing back-edge/finiteness checks.

**What this forbids**
- Forbids the scalar `on_false = "<step>"` form outright — lint treats it as an error, not a deprecated-but-tolerated shorthand.
- Forbids an unbounded `on_false` back-edge in any form; there is no default `max`, so a table missing `max` remains as illegal as the scalar form it replaces.

## Evidence and confidence

- [measured] Lint v2 (`spikes/p9/lint.py`), which the source states implements §9 + §9a + §9b, raised 25 errors on its first run against the ten P9 activities (`results-v2-catalog-run1.txt`), of which 12 are recorded as "`on_false` form"; all were fixed in the catalog and the second run (`results-v2-catalog-run2.txt`) shows 0 errors.
- [inferred] The source does not break those 12 down, and lint v2 implements both this decision's table form and §9a.1's forward-target rejection (ADR 0031) under the same "`on_false` form" label, so the number of catches attributable to the scalar form specifically is not stated. The count is evidence that `on_false` form is mechanically enforced and that the pre-amendment catalog did not already conform, not a measurement of this rule alone — and it says nothing about how often a scalar `on_false` would have produced an actually unbounded loop at runtime.
