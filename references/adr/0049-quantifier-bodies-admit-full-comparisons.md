---
id: "0049"
title: Quantifier bodies admit full comparisons, not membership only
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-19
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 19)
supersedes: []
superseded_by: []
related: ["0070"]
evidence: []
consumers: []
---

## Context and problem

The §9a.3 predicate grammar defined `none/all/any(<field> in […])` — a quantifier whose body was restricted to a membership test. The P9b second author, writing three activities against §1–§9a, needed a quantifier over a full comparison (e.g. every element of a list has both an owner and a due date set) and found the grammar could not express it.

## Decision drivers

- Real activity predicates need to assert conjunctions of field-level conditions across a list's elements, not just "is this field one of these values".
- The grammar is otherwise closed (no user functions); the fix has to widen the body of the quantifier form itself rather than open an escape hatch.

## Considered options

### Option — do nothing (membership-only quantifier bodies)
Authors needing a compound condition per element would have to decompose it into multiple `none`/`all`/`any` calls or drop into ambiguous ad hoc predicate authoring. Rejected as a P9b-surfaced ambiguity; the source records no alternative form considered.

## Decision

Quantifier bodies take full comparisons: `all(owner != null and due != null)`, not membership only. The element's fields remain in scope inside the body per the §9a.3 base grammar.

## Consequences

**Positive**
- A single `all`/`any`/`none` call can express a compound per-element condition, matching what stage-exit `checks` and `applicable_if` predicates actually need to state.

**Negative**
- The quantifier body is now a full sub-grammar of comparisons and boolean connectives, not a fixed membership shape — the lint's parser must handle the wider form (folded into lint v2 alongside the other §9b amendments).

**What this forbids**
- The wider body does not open the grammar's closure: §9a.3's "no user functions" still holds inside a quantifier body, and the element's fields are the scope the body reads, exactly as in the membership form.

## Evidence and confidence

[documented]: the amendment is adjudicated in the source (§9b.19) with the illustrative example `all(owner != null and due != null)`; no measurement or spike is cited for it. Later, §9c.40 further widens quantifier bodies to admit `len()` and nested calls — that is a separate, later amendment (amendment 40) and is not part of this decision's evidence.
