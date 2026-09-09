---
id: "0050"
title: Tag semantics — same facet disjoins, distinct facets conjoin
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-20
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 20)
supersedes: []
superseded_by: []
related: ["0030"]
evidence: []
consumers: []
---

## Context and problem

PC8 (ADR 0030) established that `tags` compile to a conjunction in `applicable_if`, but left the case of repeated facets in one tag list unspecified. The schema's own §7 worked example used `tags = ["kind:bug", "kind:incident", "change:text"]`, evidently intending "bug or incident, and text" — but a pure conjunction reading makes that combination unsatisfiable (no activity is simultaneously `kind:bug` and `kind:incident`). The P9b batch surfaced this as an amendment.

## Decision drivers

- The §7 example itself was wrong under a naive all-conjunction reading, so the ambiguity was not hypothetical — it broke the schema's own illustration.
- The source's stated reason is exactly that unsatisfiability: for a facet like `kind`, whose computed fact takes one value for a given item, two tags on that facet cannot both hold at once, so conjoining them can only ever be false. (The source does not generalise this to every facet, and PC8's vocabulary includes set-valued facts such as `toolchain`; the rule is stated per facet, not as a claim that every facet is single-valued.)

## Considered options

### Option — do nothing (pure conjunction of every tag)
This is the reading PC8's prose literally supports and is what made the §7 example unsatisfiable. Rejected because it produces a self-contradiction the schema's own example already exhibited.

## Decision

Tags compile to a predicate where tags sharing the same facet disjoin, and tags on distinct facets conjoin: `kind:bug, kind:incident` means "bug or incident". Applied to the §7 example, `["kind:bug", "kind:incident", "change:text"]` now compiles to `(kind == bug OR kind == incident) AND change == text`, which is satisfiable.

## Consequences

**Positive**
- The tag-sugar-to-`applicable_if` compilation now matches how authors actually write multi-value tag lists, and fixes the schema's own worked example.

**Negative**
- The compilation rule is no longer "just AND everything" — it requires grouping tags by facet before conjoining, adding a step to the tags-to-predicate compiler and to lint's reasoning about satisfiability.

**What this forbids**
- A tag list can no longer be read as expressing "all of these facet values must hold at once" for the same facet — that combination is not representable via tags at all (it would require two mutually exclusive facet values to be simultaneously true, which the facet model does not allow).

## Evidence and confidence

[documented]: the amendment is adjudicated in the source (§9b.20), which explicitly names the §7 example as "unsatisfiable as a conjunction" under the prior reading — this is a logical/definitional finding, not a measured one. No spike or frequency count is cited for how often this ambiguity would have misfired in practice.
