---
id: "0069"
title: Ordering operators against null are lint errors; equality is defined
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-39
source: references/2026-09-07-activity-graph-schema.md §9c amendment 39
supersedes: []
superseded_by: []
related: ["0032", "0033"]
evidence: []
consumers: []
---

## Context and problem

§9a amendment 2 had already established that absent `?` inputs are `null`, that quantifiers over `null` are vacuously
true, that `null == null`, and that "any other operation on `null` is a lint error." The P6 authoring batch, writing 90
activities against the full predicate grammar, produced an amendment refining it: the blanket "any other operation"
did not say in terms whether ordering comparisons (`<`, `<=`, `>`, `>=`) against `null` were included. The source row
records the refinement and nothing about who needed it or why the general phrasing was insufficient.

## Decision drivers

- The predicate grammar (§9a.3) includes `== != < <= > >= in` as a fixed operator set; each needed a defined behaviour
  against `null` or an explicit rejection, not an implicit fallthrough.
- Ordering a `null` against a number or string has no well-defined result, unlike equality, which has an obvious
  identity semantics already fixed by §9a.2.

## Considered options

The source row records no alternatives column and no rejected option; the option below is reconstructed as the
status quo the amendment displaced, not quoted from the source.

### Option — do nothing (leave ordering against null under the general "any other operation is a lint error" rule)
Lost because the amendment was written at all: §9a.2's general phrasing left the operator set implicit, and the
amendment states the split (`==`/`!=` defined, ordering rejected) explicitly instead. The source does not say what
harm the implicit form caused.

### Option — equality defined, ordering a lint error (chosen)
`==` and `!=` are defined against `null`; `<`, `<=`, `>`, `>=` against `null` are lint errors. A third possibility —
giving `null` an ordering position (e.g. smallest) so ordering evaluates rather than fails — is not recorded in the
source as having been considered.

## Decision

`==` and `!=` are defined against `null`; ordering operators (`<`, `<=`, `>`, `>=`) against `null` are lint errors.
The only equality value the sources fix is §9a.2's `null == null`; neither §9a.2 nor amendment 39 states the result of
comparing `null` for equality against a non-null value, so this ADR does not state one either. The amendment note says
it "refines §9a.2" rather than replacing it.

## Consequences

**Positive**
- Removes ambiguity for catalog authors and for the lint implementation about exactly which operators are legal against
  `null` and which are not.
- Keeps the predicate language's `null` handling internally consistent: equality has an identity meaning everywhere;
  ordering, which has no natural meaning for an absent value, is rejected outright rather than silently coerced.

**Negative**
- Authors who want an ordering-like check against a possibly-absent field (e.g. "due date is more than 3 days away, if
  set") must explicitly guard with an equality/`!= null` check first; the grammar gives no shorthand for "ordering that
  treats null as satisfying/failing."

**What this forbids**
- Forbids any predicate that applies `<`, `<=`, `>`, or `>=` to an operand that can be `null` — lint must reject it,
  not evaluate it with some default ordering (e.g. treating `null` as smallest).

## Evidence and confidence

- [documented] The rule is stated directly in §9c amendment 39: "`==` and `!=` are defined against `null`; ordering
  operators on `null` are lint errors (refines §9a.2)."
- No finding id, spike, or measurement is cited for this amendment; it is a grammar-precision fix surfaced by
  authoring, not a measured result. The `evidence` field is left empty accordingly.
- This licenses only the operator rule as stated. It does not license any claim about how often authors wrote ordering
  comparisons against possibly-null operands, nor about the equality result for `null` against a non-null value, which
  no source fixes.
