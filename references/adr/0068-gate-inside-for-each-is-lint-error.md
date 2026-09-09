---
id: "0068"
title: A gate step inside a for_each is a lint error
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-38
source: references/2026-09-07-activity-graph-schema.md §9c amendment 38
supersedes: []
superseded_by: []
related: ["0035", "0022"]
evidence: []
consumers: []
---

## Context and problem

`for_each` fans a step out into a runtime-computed number of copies (§1 "Fan-out"), and `gate` steps write a
kernel-owned durable `Gate:` mark and pause the run pending a human answer (§3). Combining them — a gate inside a
`for_each` — would mean the cardinality of human-answered marks is runtime-computed too, which the P6 authoring batch
surfaced as an open question the schema had not settled. The schema prices an escalation at "the F28 flat ~18.6k
resume" (§3), a per-mark figure; what a runtime-sized set of marks costs is exactly what the source row says is not
yet known. The row gives one reason and no other: "revisit after P8 prices a mark".

## Decision drivers

- A mark's price was unknown: the row defers to P8 to price one before the rule is revisited.
- Gate marks are kernel-owned and durable (PC6(i)), and a `for_each`'s cardinality is runtime-computed (§1 "Fan-out"),
  so the number of marks an activity could open would not be knowable from the catalog.

## Considered options

### Option — per-element gate marks (allow gate inside for_each)
Rejected pending P8: allowing one gate mark per fan-out element would let an activity open an arbitrary, runtime-sized
number of human questions per run. Listed among the "Rejected" items at the end of §9c as "per-element gate marks (38,
pending P8)."

### Option — lint error (chosen)
A `gate` step inside a `for_each` is a lint error. The decision record says explicitly to "revisit after P8 prices a
mark" — i.e. this is not a permanent prohibition but a hold pending cost data.

## Decision

A `gate` step nested inside a `for_each` is a lint error. This closes off per-element human escalation until a mark's
cost is priced. The lint check is added in lint v3 (§9c's registry-consequences paragraph lists rule 38 among what lint
v3 implements).

## Consequences

**Positive**
- Avoids shipping a gate-per-fan-out-element capability before its cost (in mark volume, human load, or kernel storage)
  is understood.
- Keeps the set of gate marks an activity can open statically analysable from the catalog, since no gate sits under a
  runtime-computed cardinality.

**Negative**
- Activities that fan out per-item work (e.g. a review needed per slice) cannot ask a per-item human question inside
  the loop; any such escalation must be hoisted outside the `for_each` or redesigned as a single aggregate gate.

**What this forbids**
- Forbids authoring a `gate` step as a child of a `for_each` step at all — the lint rejects the catalog outright, not
  merely at some rigor levels.
- Forbids treating this as a permanent design position: the amendment explicitly frames it as provisional, to be
  revisited once P8 prices a mark, not as a settled architectural stance.

## Evidence and confidence

- [documented] The lint rule and its rationale ("revisit after P8 prices a mark") are stated directly in §9c amendment
  38 and repeated in the "Rejected" summary at the end of §9c.
- [documented] The rule is conditional on a named spike: **P8** ("revisit after P8 prices a mark"). At this date P8 is
  partial (F38) — it ran 3 of a pre-registered 27+3 and reported mid-graph resume as lossless at 0.45–0.78× of ~18.6k, with
  its 65-min and 24-h arms and all three controls unrun. It has therefore not priced a mark in the sense this rule
  waits on, and the hold stands.
- No finding id is cited for the amendment itself; the `evidence` field is left empty.
- This licenses only that the rule is in effect and is held provisionally. It does not license any claim about what a
  per-element mark would cost, nor that per-element marks are wrong on the merits — the row rejects them "pending P8",
  not on principle.
