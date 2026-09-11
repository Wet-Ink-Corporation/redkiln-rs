---
id: "0033"
title: Predicate grammar is fixed and closed, with no user-defined functions
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-3
source: references/2026-09-07-activity-graph-schema.md §9a (amendment 3)
supersedes: []
superseded_by: ["0103"]
related: ["0048", "0049", "0052", "0070"]
evidence: ["P9 spikes/F32-DRAFT.md §4"]
consumers: []
---

## Context and problem

§8 names `Predicate` as "the same closed-vocabulary expression type used by `applicable_if` and `require.when` (PC8), extended with `${step.outputs.*}` field access and `rigor`. No user-defined functions" but does not enumerate what roots, operators, or literal forms the grammar actually contains. §2's `checks` examples already use forms like `evidence.any(GreenAt)` and `evidence.none(Findings.items[...])` that §8's one-line description does not by itself define. Authoring ten activities against the schema needed a fixed grammar to write against, not a description by example.

## Decision drivers

- The schema commits to a closed vocabulary (no user functions) as a design constraint (§8); a closed vocabulary must actually be closed, i.e. enumerated, or authors and lint diverge on what is legal.
- Lint rule 9 ("no predicate references a fact outside the facet registry") and rule 5 (decidability of stage-exit checks) both presuppose a fixed grammar to check membership and decidability against.
- Ten activities were authored against an undefined grammar (spike P9), which is itself the forcing measurement.

## Considered options

### Option — do nothing (leave the grammar described by example)
§8's one-line description plus the §2 and §7 worked examples would remain the only definition, so authors and lint would each infer the legal forms independently. The source row names no alternative grammar; this is the pre-amendment state amendment 3 replaces.

### Option — enumerate the grammar normatively (chosen)
Fixes roots, accessors, `len()`, quantifiers, comparison and boolean operators and literal types as a closed list, so "closed vocabulary" is a checkable property rather than a stated intent.

## Decision

Predicate grammar (normative): roots are step inputs, facets, `rigor`, `item`; `.field` access; `len()`; `none/all/any(<field> in […])` with the element's fields in scope; operators `== != < <= > >= in`; boolean connectives `and or not`; literals are string/number/bool/`null`. No user functions.

## Consequences

**Positive**: authors and the lint checker share one enumerated grammar instead of inferring it from worked examples; lint rules 5 and 9 (decidability, facet-registry membership) become checkable against a closed set of forms.

**Negative**: any predicate need that falls outside this enumerated set (a form not yet anticipated by the ten P9 activities) has no expression path until a further amendment extends the grammar.

**What this forbids**: user-defined functions of any kind; predicate expressions the closed vocabulary above cannot represent.

## Evidence and confidence

[documented] Stated normatively in §9a amendment 3, one of eleven amendments produced by authoring ten activities against §1–§9 (spike P9); the source gives no separate measurement of how many predicate forms in those ten activities the grammar as stated actually covers. [inferred] The grammar was extended, not reversed, by later amendments outside this cohort's range: §9b.18 (ADR 0048) adds parentheses and quoted-vs-bare facet-value literal rules; §9b.19 (ADR 0049) extends quantifier bodies to full comparisons, not membership only; §9b.22 (ADR 0052) extends rule 3 to resolve paths, not just step ids; §9c.40 (ADR 0070) extends quantifier bodies to admit `len()` and nested calls. This ADR records the grammar as it stood at amendment 3 only; it does not restate those later extensions.
