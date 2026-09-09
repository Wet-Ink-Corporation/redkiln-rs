---
id: "0053"
title: A for_each step counts once against the attempts budget
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-23
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 23)
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

§9 lint rule 8, as tightened by amendment §9a.7, reads `budget.attempts_total ≥ number of agent/tool steps on one pass`. A `for_each` step (§1: "Fan-out") runs one copy per element of a runtime-computed list, so it was ambiguous whether such a step should count once toward this budget check, or once per fan-out copy — the latter being unknowable at lint time since cardinality is runtime-computed while only the shape is declared.

## Decision drivers

- Rule 8's check runs at catalog-load time, before any run exists to compute a `for_each` step's actual cardinality (§1: "Cardinality is runtime-computed but the *shape* is declared").
- The activity-level `budget.attempts_total` (§4) is a per-activity ceiling, not a per-fan-out-copy one; conflating the two would make the static check either impossible (if cardinality is required) or meaningless (if it always assumes worst case).

## Considered options

### Option — count a for_each step once per potential fan-out element
The source row records no alternative considered; this option is stated here as the mechanical complement of the decision, and the reason it loses is drawn from §1's own rule ("cardinality is runtime-computed but the *shape* is declared"), not from the source row. Counting per element would require a maximum cardinality at lint time that the schema does not provide, so the check could not run at catalog load.

### Option — do nothing (leave rule 8 silent on for_each)
Rule 8's step count would stay ambiguous for any activity using fan-out. Rejected: this is one of the exact ambiguities the P9b batch was run to surface, and leaving it open leaves two defensible readings of a rule that must produce one verdict at catalog load.

## Decision

Rule 8's step count treats a `for_each` step as counting once, regardless of its runtime fan-out cardinality.

## Consequences

**Positive**
- Rule 8 remains a static, catalog-load-time check even for activities using fan-out steps (§7's `implement-slice` example), since it no longer depends on unknowable runtime cardinality.

**Negative**
- The static attempts-budget check does not reflect the true worst-case attempt count of a fan-out step whose cardinality is large — a `for_each` over many elements can still exhaust `budget.attempts_total` at runtime even though it passed the static lint check.

**What this forbids**
- Lint rule 8 cannot be used as a runtime attempts-safety guarantee for fan-out activities; it forbids treating a passing rule-8 check as proof the budget cannot be exhausted by a `for_each` step's actual cardinality.

## Evidence and confidence

[documented]: adjudicated in the source (§9b.23); no measurement is cited, and the source does not report how often fan-out cardinality in practice exceeds what a "counts once" static check would anticipate.
