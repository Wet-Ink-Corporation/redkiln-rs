---
id: "0051"
title: Provided artifacts are unconditional at every rigor at or above min_rigor
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-21
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 21)
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

§9 lint rule 3 requires "every `provides` bound" but did not state, in words, at which rigor levels that binding must hold. An activity's variant-by-rigor structure (§1) means some steps only run above a given rigor, so it was ambiguous by reading alone whether a `provides` artifact could be conditionally emitted only at higher rigor. The P9b batch records that the lint already enforced the stricter reading in practice, but the rule's text had not caught up.

## Decision drivers

- The lint (as implemented) already required this; the amendment closes the gap between implementation and normative text rather than changing behaviour.
- An activity's contract (§1: `provides`, `requires`, `applicable_if`, `min_rigor`, ...) is declared once per activity, not per rigor level, so a `provides` kind that only sometimes materialises would make the contract's meaning rigor-dependent in a way nothing else in the schema does.

## Considered options

### Option — do nothing (leave rule 3 silent on rigor-conditionality of provides)
Text and lint behaviour would continue to diverge, and an author reading only the prose could reasonably (and wrongly) believe a `provides` kind could be emitted only above some rigor threshold within a single activity. Rejected: this is the exact gap the amendment closes.

## Decision

Provided artifacts are unconditional at every rigor greater than or equal to the activity's `min_rigor`. This is a clarification of rule 3's text; the lint already enforced it before the wording changed.

## Consequences

**Positive**
- The activity's `provides` promise now has one unambiguous meaning across every rigor level at which the activity is selectable, matching what the lint already checked.

**Negative**
- None beyond the cost of documentation catching up to enforced behaviour — no new constraint on authors who were already passing lint.

**What this forbids**
- An activity cannot declare a `provides` kind that is emitted only above some rigor threshold higher than its own `min_rigor` — if a kind is in `provides`, every rigor level at or above `min_rigor` must produce it (rigor-conditional internal steps like `adversary`/`signoff` in §7's example may still vary, but the final `provides` set they feed does not).

## Evidence and confidence

[documented]: adjudicated in the source (§9b.21) with the note "lint already enforced" — this is a textual clarification, not a new empirically-grounded rule, and the source cites no measurement for it.
