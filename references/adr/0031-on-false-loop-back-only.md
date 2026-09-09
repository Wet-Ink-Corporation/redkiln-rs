---
id: "0031"
title: on_false targets only earlier-declared steps; forward branching stays in when
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-1
source: references/2026-09-07-activity-graph-schema.md §9a (amendment 1)
supersedes: []
superseded_by: []
related: ["0044", "0065"]
evidence: ["P9 spikes/F32-DRAFT.md §4"]
consumers: []
---

## Context and problem

§9 lint rule 1 requires the activity graph to be acyclic "except through declared `repeat`/`on_false` back-edges, each with a finite `max`", but §9 left `on_false`'s target open to any step id — including one declared later in the same activity. Authoring ten activities against §1–§9 (spike P9) forced the question of what a forward `on_false` target would even mean, since the schema already has a separate mechanism (`when`) for skipping a downstream step. Left ambiguous, `on_false` could be used as a second, competing branching construct.

## Decision drivers

- §0's own constraint: "'Should this run?' (`when`) and 'how many copies?' (`for_each`) are distinct constructs" (workflow-model §2) — the schema already commits to one construct per concern; a forward `on_false` would blur `on_false` into `when`'s job.
- Lint rule 1 already requires back-edges to be declared and bounded (`max`); a forward target is not a back-edge and has no place in that rule.
- Ten activities authored against the schema surfaced the ambiguity in practice, not in theory (P9).

## Considered options

### Option — do nothing (leave `on_false`'s target unconstrained)
§9 as written places no restriction on which step an `on_false` may name, so a forward target stays legal by default and duplicates `when`'s job. The source row names no alternative weighed against loop-back-only; this is the pre-amendment state amendment 1 replaces.

### Option — loop-back only, forward branching via `when` (chosen)
Restricts `on_false` to an earlier-declared step, `continue` or `fail`, keeping one construct per concern and every `on_false` target a back-edge candidate for lint rule 1.

## Decision

`on_false ∈ { <earlier-declared step>, continue, fail }` — loop-back only. Forward branching is expressed with `when` on the downstream step, not with `on_false`. Lint rejects forward targets.

## Consequences

**Positive**: one construct per concern is preserved (`on_false` = loop back or terminate; `when` = should this step run); the acyclic-graph lint rule 1 stays simple because every `on_false` target is by construction a back-edge candidate.

**Negative**: an author who wants "if this check fails, skip ahead" must restructure as `when` on the downstream step(s) rather than reaching for `on_false`, which is a less direct idiom for that case.

**What this forbids**: `on_false` naming any step not already declared earlier in the same activity; lint treats a forward-target `on_false` as an error, not a warning.

## Evidence and confidence

[documented] The rule is stated normatively in §9a amendment 1 as one of eleven amendments produced by authoring ten activities against §1–§9 (spike P9); the source states "none reverse a section above, all tighten it." [inferred] No count of how many of the ten activities actually attempted a forward `on_false` before the rule was written is given, so the frequency of the ambiguity in practice is not established, only that it arose. This decision is downstream of lint v2 (`spikes/p9/lint.py`), which the source says implements §9a; lint v2's first run against the ten P9 activities raised 25 errors including "12 `on_false` form" errors, later fixed to 0. The source does not break those 12 down, and lint v2 implements both forward-target rejection (this decision) and §9b.14's `on_false` table form, so how many of the 12 belong to this rule is not stated — it is evidence that `on_false` form is enforced at all, not evidence about how often authors would reach for a forward target under §9's original open form.
