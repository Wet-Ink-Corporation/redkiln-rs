---
id: "0070"
title: Quantifier bodies admit len() and nested calls
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-40
source: references/2026-09-07-activity-graph-schema.md §9c amendment 40
supersedes: []
superseded_by: []
related: ["0033", "0049"]
evidence: []
consumers: []
---

## Context and problem

The predicate grammar (§9a.3) already included `len()` as a root-level function and `none/all/any(<field> in […])` as
quantifiers, later widened by §9b.19 to admit full comparisons inside quantifier bodies (`all(owner != null and due !=
null)`), not just membership. The P6 authoring batch, writing 90 activities against the full grammar, produced an
amendment widening quantifier bodies again — to admit `len()` and nested calls. The source row states the change and
the tooling consequence ("the lint's regex becomes a parser"); it does not give the predicates that motivated it, so
no example of a needed predicate is reconstructed here.

## Decision drivers

- The amendment comes out of authoring at scale (90 activities against the full grammar), the same batch that produced
  §9c's other grammar refinements. The source does not name the predicates that forced it.
- A regex-based body matcher cannot recognise nested calls, which is why the source pairs the grammar change with a
  tooling change ("the lint's regex becomes a parser").

## Considered options

The source row records no alternatives column and no rejected option; the option below is reconstructed as the status
quo the amendment displaced, not quoted from the source.

### Option — do nothing (keep quantifier bodies to flat comparisons, per §9b.19)
Lost because the amendment was written: the source records the widening as adopted. It does not state the reason the
narrower form was insufficient, so none is asserted here.

### Option — admit `len()` and nested calls (chosen)
Adopted, with the lint's regex body matcher replaced by a parser.

## Decision

Quantifier bodies admit `len()` and nested calls. The lint's regex-based body matcher is replaced by a parser, since a
regex cannot represent nesting.

## Consequences

**Positive**
- Lets authors express richer per-element conditions — a body may itself call `len()` or another quantifier — without
  inventing a separate mechanism outside the predicate grammar.
- Aligns the lint implementation with what the grammar already conceptually allowed once `len()` and quantifiers are
  both just expressions.

**Negative**
- Replacing a regex with a parser is a larger implementation commitment for the lint tool than the regex it replaces;
  no cost or timeline for that rewrite is given in the source.

**What this forbids**
- Forbids the lint from continuing to rely on the regex-based body matcher — the amendment states the regex "becomes a
  parser," i.e. the old matching approach is retired, not kept as a fallback.

## Evidence and confidence

- [documented] The change is stated directly in §9c amendment 40: "Quantifier bodies admit `len()` and nested calls;
  the lint's regex becomes a parser."
- No finding id, count of affected activities, or measurement is cited for this specific amendment; it is presented as
  a grammar/tooling extension surfaced by the P6 authoring batch, not a measured result. The `evidence` field is left
  empty accordingly.
- This licenses only the grammar and tooling change as stated. It does not license any claim that the parser has been
  written, that the widened bodies were exercised by the 90-activity batch, or that nesting is bounded to any depth —
  the source fixes no depth limit.
