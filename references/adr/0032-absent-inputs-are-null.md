---
id: "0032"
title: Absent optional inputs are null, with vacuous-true quantifiers and a lint error elsewhere
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-2
source: references/2026-09-07-activity-graph-schema.md §9a (amendment 2)
supersedes: []
superseded_by: []
related: ["0069"]
evidence: ["P9 spikes/F32-DRAFT.md §4"]
consumers: []
---

## Context and problem

§8's `Predicate` type and §9 lint rule 3 ("every `${ref}` resolves; `?` marks every ref to a conditional step") establish that a `?`-marked ref can be legitimately absent — for example `${adversary.outputs?}` in the worked `root-cause` example (§7) when the `adversary` step is skipped by `when = "rigor >= recommended"`. §1–§9 never state what value such an absent ref actually holds when a predicate reads it, so a `check` step or a quantifier over a skipped step's output had no defined behaviour. Authoring against the schema (spike P9) forced the question.

## Decision drivers

- The worked example in §7 already depends on this case (`policy`'s `check` step reads `${adversary.outputs?}` when `adversary` may not have run).
- The predicate language (§8) is a closed-vocabulary expression type with no user functions; it needs a total semantics for absence, not an author-supplied default per site.
- Quantifiers (`none/all/any`, added normatively alongside this amendment as §9a.3) need a defined result over an absent/empty collection to be decidable per lint rule 5.

## Considered options

### Option — do nothing (leave absence undefined per-site)
Each predicate author would have to guard every `?`-marked ref before using it, or the behaviour would depend on implementation accident. The source records no alternative to weigh against the chosen semantics; this is the honest default state amendment 2 replaces.

### Option — absence is `null`, with a closed set of legal operations on it (chosen)
Gives the predicate language a total semantics for absence — one defined value, vacuously-true quantifiers, equality only — instead of a per-site author convention.

## Decision

"Absent `?` inputs are `null`; quantifiers over `null` are vacuously true; `null == null`; any other operation on `null` is a lint error."

## Consequences

**Positive**: predicates over conditionally-present steps (like `policy`'s read of `${adversary.outputs?}`) have one defined value to reason about instead of author-chosen guards; quantifiers stay decidable because "vacuously true" is a fixed rule, not a per-predicate choice.

**Negative**: any predicate author who reaches for an ordering or arithmetic comparison against a possibly-absent ref is caught only at lint time, not by a type they can see in the TOML.

**What this forbids**: any operation on `null` other than `==`/quantifier membership is a lint error — ordering, arithmetic, and (until later refined) any comparison beyond equality against `null` are all rejected rather than silently coerced.

## Evidence and confidence

[documented] Stated normatively in §9a amendment 2, one of eleven amendments from authoring ten activities against §1–§9 (spike P9); no measurement is cited for it specifically. [inferred] The rule was later refined, not reversed: §9c amendment 39 (ADR 0069, outside this cohort's range) states `==`/`!=` are defined against `null` while ordering operators on `null` remain lint errors, and describes itself as refining this amendment — so this decision's boundary ("any other operation… is a lint error") was tightened to name `!=` explicitly rather than leaving it under "any other operation." This ADR does not itself record that refinement's text, only flags the dependency.
