---
id: "0048"
title: Predicate grammar gains parentheses and quoted facet-value literals
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-18
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 18)
supersedes: []
superseded_by: []
related: ["0030"]
evidence: []
consumers: []
---

## Context and problem

The normative predicate grammar fixed by amendment §9a.3 gave roots, field access, comparison and boolean operators but said nothing about grouping or about how a facet value with punctuation in it (e.g. `security-vuln`) is written as a literal. A second author writing three activities against §1–§9a hit this as one of the 13 amendments the P9b batch produced (§9b, falsifying P9's prediction that the amendment count would fall on the next batch, at n=3).

## Decision drivers

- The grammar already excludes user-defined functions; grouping is needed for readable `and`/`or` compositions without inventing precedence rules by convention.
- Facet values are closed, enumerated vocabularies (PC8 rule 1); some values are not valid bare identifiers.
- Ordinal facets need ordering comparisons (the source names `>=`) to be checkable against a declared order, which the grammar had no way to express.

## Considered options

### Option — do nothing (leave grouping and literal syntax undocumented)
Authors would improvise ad hoc conventions per pack, and the lint (a regex-based checker at this point) would have no normative form to check against. Rejected: this is exactly the ambiguity class the P9b batch was run to surface.

## Decision

Parentheses are allowed in predicates. Facet values are written bare when they match `[a-z_][a-z0-9_]*`, and quoted otherwise (e.g. `'security-vuln'`). `facets.toml` gains an `ordered = true` flag on ordinal facets so that `>=` and other ordering comparisons against that facet are checkable at lint time.

## Consequences

**Positive**
- Predicates with mixed `and`/`or` can be written unambiguously.
- Facet values with hyphens or other non-identifier characters are expressible without inventing an escape convention per pack.
- Ordinal facets (like `rigor`) can be lint-checked for valid use of ordering comparisons, catching a category of error at catalog-load time.

**Negative**
- Every pack author must now learn the quoting rule, and the lint must parse it correctly (lint v2 raises 15 errors against P9b's three files including all five guesses v1 let through — this amendment is one of several driving that revision).

**What this forbids**
- A facet value cannot be freely bare if it fails the `[a-z_][a-z0-9_]*` pattern — it must be quoted; the two forms are not interchangeable.
- An ordering comparison (`>=`) against a facet not marked `ordered = true` remains outside the checkable grammar — ordering is not implicitly available on every facet.

## Evidence and confidence

[documented]: the amendment is adjudicated directly in the source (§9b.18); no measurement is cited for it specifically. [documented]: the batch-level count — 13 proposed amendments from three activities, 20 of 27 ambiguities not covered by the prior P9 batch — is reported in §9b's lead paragraph, but that number is not decomposed per amendment, so it does not license attributing a specific defect rate to grammar issues alone.
