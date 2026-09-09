---
id: "0037"
title: Budget attempts_total counts agent/tool steps per pass, not summed retries
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-7
source: references/2026-09-07-activity-graph-schema.md §9a
supersedes: []
superseded_by: []
related: ["0053"]
evidence: []
consumers: []
---

## Context and problem

§9 lint rule 8 originally read "Budget ≥ Σ (step `retry.max` × 1) attempts; otherwise the budget is unreachable and the author is warned" — a formula stated in terms of summed per-step retry allowances. Authoring ten activities against §1–§9 (the P9 spike, `spikes/F32-DRAFT.md` §4) surfaced this as one of eleven amendments needed to tighten the schema; the amendment record gives no further diagnostic detail than that the rule needed restating.

## Decision drivers

- Lint rule 8 must be a computable, unambiguous minimum an author can check against a graph's actual shape.
- The prior wording ties the minimum to summed `retry.max` values rather than to the number of dispatchable (`agent`/`tool`) steps a single pass through the graph actually contains.

## Considered options

### Option — keep the original formula (§9 rule 8: budget ≥ Σ retry.max)
This is the pre-amendment wording, quoted from §9 rule 8. The source row names no alternative option, so this is not a candidate the source weighed and rejected — it is the losing formula the amendment replaces. The source does not detail the specific catalog case that broke it, only that the rule "becomes" the new form.

### Option — restate rule 8 against per-pass step count (chosen)
`budget.attempts_total ≥ number of agent/tool steps on one pass` ties the minimum directly to the graph's dispatchable-step count rather than to retry allowances summed across steps.

## Decision

Lint rule 8 becomes: `budget.attempts_total ≥ number of agent/tool steps on one pass`.

## Consequences

**Positive**: the lint check is now stated directly against a countable graph property (dispatchable steps per pass) rather than an indirect sum over retry settings, making the "budget is unreachable" warning easier to verify by inspection.

**Negative**: as stated, this amendment does not yet address `for_each` fan-out step counting — that refinement ("a `for_each` step counts once") is a later amendment (§9b.23) outside this record.

**What this forbids**: an activity's declared `budget.attempts_total` cannot be lower than the number of `agent`/`tool` steps that execute on one pass through its graph; a catalog author cannot rely on the old summed-retry accounting to justify a smaller budget.

## Evidence and confidence

[documented] The amendment is recorded in `2026-09-07-activity-graph-schema.md` §9a item 7, sourced to `spikes/F32-DRAFT.md` §4 (P9 authoring of ten activities). No finding id or measurement is cited for this specific item — the source states the rule change without further evidence, so none is invented here.
