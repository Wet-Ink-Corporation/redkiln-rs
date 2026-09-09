---
id: "0058"
title: Families are a declared catalog field, lint-checked for sharing and totality
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-28
source: references/2026-09-07-activity-graph-schema.md §9c amendment 28
supersedes: []
superseded_by: []
related: ["0056"]
evidence: []
consumers: []
---

## Context and problem

Amendment 26 (rank semantics and family totality) presupposes that "family" is a recognizable grouping of activities the
lint can enumerate and check. Before this amendment nothing in the schema declared what a family *is* — it existed only
as an informal grouping implied by activities sharing a `provides` kind and competing by `method_rank`. The P6 authoring
batch needed an explicit, lint-checkable way to say which activities belong to the same family so that amendment 26's
totality rule has something concrete to iterate over.

## Decision drivers

- Amendment 26's totality lint needs an enumerable set of "family members" to check coverage over; an implicit grouping
  by shared `provides` plus distinct ranks is not sufficient to check on its own (multiple unrelated families could share
  a `provides` kind without being a rank-escalation family).
- The declaration should be cheap to author (a single field) rather than a new top-level structure.

## Decision

An activity declares its family membership with a `family = "<name>"` field. Lint checks three things for every declared
family: that its members share `provides`, that they hold distinct `method_rank` values, and that the family satisfies
rule 26's totality-over-rigor property.

## Considered options

### Option — do nothing (families stay implicit)
Leave family membership implicit in shared `provides` and distinct ranks, with no declared name and no lint. This is the
prior state. Rejected because amendment 26's totality check needs an explicit, named grouping to enumerate — an implicit
grouping by `provides` alone conflates every producer of a kind into one undifferentiated pool, which is not what
"family" means when ceilings and escalation are in play. The source row names no alternatives; this is the prior state
the amendment replaces, not a named candidate.

## Consequences

**Positive**
- Gives amendment 26's totality lint a concrete object (the named family) to enumerate members of and check.
- Makes family membership visible and greppable in the catalog rather than inferred from rank and `provides` alone.

**Negative**
- Adds a field authors must set and keep correct; an activity added to a family but given a duplicate rank, or omitted
  from a family it logically belongs to, is now a distinct authoring error surface.

**What this forbids**
- Two activities in the same declared family may not share a `method_rank` — lint rejects the tie the same way the
  pre-existing rank rule already rejects a tie among admissible producers.
- A family that fails rule 26's totality-over-rigor check may not ship; the `family` declaration is not decorative, it is
  the thing lint checks against.

## Evidence and confidence

- [documented] No finding id or spike is cited in the source row; this is a catalog-structure decision made in the same
  P6 adjudication pass as amendment 26, not a separately measured result. No evidence is invented here.
