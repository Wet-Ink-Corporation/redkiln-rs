---
id: "0042"
title: Feedback refs are normative, not lint folklore
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-12
source: references/2026-09-07-activity-graph-schema.md §9b amendment 12
supersedes: []
superseded_by: ["0173"]
related: ["0031", "0043"]
evidence: []
consumers: []
---

## Context and problem

§1 "Edge" and §9 lint rule 1 define an edge as implicit — a step depending on another step it references — but never state whether a step may reference a step *declared later* in the same activity's TOML. In practice this was already relied on to let a loop-back target read a downstream step's prior output, but the rule was never written down; the source calls it "lint folklore." A second author (P9b), writing three activities against §1–§9a with no access to that folklore, produced 27 ambiguities, 20 of them new; this rule is one of the thirteen amendments adjudicated out of that batch. The source does not say whether this particular gap was among the five newcomer guesses lint v1 passed untouched.

## Decision drivers

- The schema's own back-edge machinery (`repeat`/`on_false`, given its bounded table form by the sibling amendment 14) depends on a loop body being able to read a later step's output from the previous iteration — something only a forward-looking ref can express.
- Lint should be able to *decide* whether a ref is legal, which an unwritten convention cannot support.
- A second, independent author guessing at the schema is the mechanism that exposed the gap — evidence the rule needed to exist on the page, not only in the schema author's head.

## Considered options

### Option — do nothing (leave it as lint folklore)
Keep relying on convention rather than a written rule. This is the status quo the amendment replaces; the source names it directly ("was lint folklore, now normative"). It lost because it does not scale past the single author who originated the convention: the second author's guesses were not checked against it and the ambiguity list (27 items, 20 new) shows unwritten rules are not transmitted by reading §1–§9a alone.

## Decision

"A ref to a later-declared step is legal iff that step or one after it has `on_false`/`repeat` targeting the referencing step; it is `null` on the first pass." This goes into §1 "Edge" and lint rule 1.

## Consequences

**Positive**
- Loop bodies can legitimately read a downstream step's own prior-iteration output, which the loop-learning fix (amendment 13) depends on being expressible at all.
- The rule is lint-checkable: a ref to a later step either has a qualifying `on_false`/`repeat` back-edge or it does not.

**Negative**
- Lint rule 1 must now trace forward refs against the set of back-edges targeting the referencing step, rather than treating "later-declared" as flatly illegal — added lint complexity.

**What this forbids**
- Forbids a ref to a later-declared step where no step at or after the target has an `on_false`/`repeat` back-edge pointing at the referencing step — that remains a genuine forward reference and stays illegal.
- Forbids treating such a legal forward ref as anything but `null` on the activity's first pass through the graph.

## Evidence and confidence

- [documented] The rule is stated normatively in §9b amendment 12, adjudicated from a second author's three-activity authoring pass (`spikes/F33-DRAFT.md`) that produced 27 ambiguities against §1–§9a, 20 of them new to that batch.
- [inferred] The source does not give a count of how many of the 27 ambiguities this specific gap accounted for, nor a lint-run number isolating this rule the way amendment 14's `on_false` form errors are counted; only that the general P9b batch "passed five newcomer guesses untouched" under lint v1. No numeric claim is made here beyond what the source states.
