---
id: "0065"
title: Lint R4b requires an on_false target reachable at every rigor the check is active
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-35
source: references/2026-09-07-activity-graph-schema.md §9c amendment 35
supersedes: []
superseded_by: []
related: ["0044"]
evidence: []
consumers: []
---

## Context and problem

§9 lint rule 4 already requires that, for every rigor level ≥ `min_rigor`, the sub-graph selected by `when` predicates is well-formed and reaches `emit`. `check` steps loop back via `on_false = { to = "<earlier step>", max = N }` (§9b.14). Because steps can be gated behind `when = "rigor >= …"` predicates (variant-by-rigor, §1), authoring the P6 catalog surfaced a case rule 4 did not already cover: a `check`'s `on_false.to` target could itself be a step that is only active (via `when`) at a higher rigor than the check that jumps to it, leaving the loop-back target unreachable at lower rigors where the check still runs.

## Decision drivers

- §9 rule 4 checks whole-graph reachability to `emit` at each rigor, but does not specifically check that a loop-back target named by `on_false.to` is active at every rigor the originating check itself is active at.
- A `check` step that is active at a given rigor but whose `on_false.to` target is not active at that same rigor is a dangling jump the existing rules do not name.

## Considered options

### Option — do nothing
Rely on rule 4's whole-graph well-formedness check to catch this case indirectly. The source does not name this as considered, but it is the implicit prior state: rule 4 proves the graph reaches `emit` at each rigor, which does not by itself prove that every `on_false.to` target used by an active check is itself active — a gap the amendment closes with a dedicated rule.

## Decision

New lint rule, **R4b**: an `on_false.to` target must be active at every rigor at which the check that names it is active.

## Consequences

**Positive**
- Closes a specific reachability gap left open by rule 4: a `check` cannot loop back to a step that has been conditioned off at the same rigor.

**Negative**
- None named in the source beyond the authoring cost of satisfying one more lint rule.

**What this forbids**
- Forbids a `check` step's `on_false.to` from pointing at a step whose own `when` predicate excludes it at any rigor where the checking step itself is active.

## Evidence and confidence

[documented] Adjudicated in the §9c batch from the third author's 90-activity P6 catalog pass (`spikes/F34-DRAFT.md` §4; `spikes/p9/DECISIONS-2026-09-08.md` decision 10, "35 accept (R4b)"). The rule's origin is a single case, not a count: F34 B18 records "a check whose `on_false` target is rigor-inactive — nothing; no lint rule", "found by reading (`unit-tests`)", and F34 §5 calls it "the one defect that would have shipped an activity with no exit at `optional` … in a class with no rule". The source states lint v3 implements rule R4b among rules 26–28, 30, 33–36, 38–41, with v2 frozen as `lint-v2.py` so F34's result files replay — but no pass/fail count specific to R4b is given here, unlike the aggregate 25/15-error counts reported for the v2 lint in §9b. No measurement licenses a claim about how many catalog activities this rule actually caught.
