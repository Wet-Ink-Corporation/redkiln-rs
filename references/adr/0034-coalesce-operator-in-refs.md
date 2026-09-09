---
id: "0034"
title: Refs support a coalesce operator over a conditional step's output
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-4
source: references/2026-09-07-activity-graph-schema.md §9a (amendment 4)
supersedes: []
superseded_by: []
related: ["0042", "0054"]
evidence: ["P9 spikes/F32-DRAFT.md §4"]
consumers: []
---

## Context and problem

§1 and §9 lint rule 3 already allow a `?`-marked ref to a conditional step's output (e.g. `${adversary.outputs?}` in the §7 worked example), and amendment 2 (ADR 0032) defines that an absent such ref reads as `null`. Neither §8 nor §9 gave authors a way to fall back to a second value when the first ref is absent — every such case had to be handled downstream as a `null`-aware branch, with no single-expression way to say "use A if present, else B."

## Decision drivers

- Amendment 2 (ADR 0032) makes `null` a defined, first-class value for absent refs, which makes a fallback operator over that value meaningful to define.
- The `${...}` ref syntax (§8) is already the place field access and conditional marking (`?`) live; adding a fallback operator to the same syntax keeps ref resolution in one place rather than pushing a conditional into every consuming predicate.
- Authoring ten activities against the schema (spike P9) surfaced the need.

## Considered options

### Option — do nothing (no coalesce; guard absence at each use site)
Every fallback would be written downstream as a `null`-aware branch or an extra `check` step, under the `null` semantics of amendment 2 (ADR 0032). The source row names no alternative fallback syntax; this is the pre-amendment state amendment 4 replaces.

### Option — a coalesce operator in the ref syntax (chosen)
`${a.outputs?} ?? ${b.outputs}` keeps fallback where ref resolution already happens, so a consuming predicate never has to encode the presence test.

## Decision

Refs support a coalesce operator: `${a.outputs?} ?? ${b.outputs}`.

## Consequences

**Positive**: a predicate or step input can express "prefer this ref, fall back to that one" in the ref syntax itself, without a separate `check` step or repeated `null`-guard logic at every use site.

**Negative**: the coalesce syntax is defined only at the level of a ref expression (`${...} ?? ${...}`); the source gives no grammar for chaining, nesting, or coalescing more than two refs, so any such need is unresolved by this amendment.

**What this forbids**: nothing beyond what amendment 2 already forbids on `null` — coalesce itself is additive syntax, not a restriction. The amendment does not extend coalesce to non-ref values (facets, literals); the source states it only for refs.

## Evidence and confidence

[documented] Stated normatively in §9a amendment 4, one of eleven amendments from authoring ten activities against §1–§9 (spike P9); the source cites no separate measurement of how often the fallback case arose across the ten activities, only that the amendment was adopted. [inferred] This amendment is closely related to two later, out-of-range amendments this ADR does not itself record: §9b.12 (ADR 0042) makes a ref to a later-declared step legal under feedback-ref conditions, and §9b.24 (ADR 0054) legalises `artifact:Kind?` while forbidding alternation in `requires` — both extend how conditional/absent refs behave but neither is stated by the source as amending or superseding this decision.
