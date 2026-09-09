---
id: "0055"
title: Self-reference inside a loop binds the previous iteration's output
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-25
source: references/2026-09-07-activity-graph-schema.md §9b (amendment 25)
supersedes: []
superseded_by: []
related: ["0043"]
evidence: []
consumers: []
---

## Context and problem

Amendment §9b.13 ("Loops must be able to learn") puts the iteration counter into the step key for every step inside a `repeat`/`on_false` cycle, and makes it a lint error for a loop-back target to carry neither a feedback ref nor `memo = false`. Some steps' only learning input is their own prior attempt — P9's `dag` step under `acyclic.on_false` is the example the source names — where there is no other step to take a feedback ref from. The grammar as it stood did not say what `${<this-step>.outputs?}` means when the step refers to itself.

## Decision drivers

- A loop-back target with neither a feedback ref nor `memo = false` fails lint under §9b.13; a step whose only prior information is its own last attempt needed a legal way to satisfy that rule.
- The feedback-ref rule (§9b.12) is defined in terms of a ref to a "later-declared step" that some earlier-or-same step loops back to — a step referencing itself is the degenerate case of that same shape and needed the same `null`-on-first-pass treatment for consistency with §9a.2's null semantics.

## Considered options

### Option — do nothing (self-reference undefined)
A step like `dag` with no other learning input available would have no legal way to observe its own prior attempt and would be forced into `memo = false` even where its live-state read is only its own history, not external state. Rejected as one of the ambiguities the P9b batch surfaced.

## Decision

Inside a loop, `${<this-step>.outputs?}` in a step's own inputs binds the previous iteration's output, and is `null` on the first pass. This satisfies §9b.13's feedback requirement for steps whose only learning input is their own prior attempt.

## Consequences

**Positive**
- A step that loops on itself (no other step to reference) has a defined, lint-legal way to read its own history and satisfy the loop-learning requirement, using the same optional-ref and null-on-absence conventions already established elsewhere (§9a.2, §9a.4).

**Negative**
- None beyond the added grammar surface area to specify and lint (self-reference is a special case the parser/lint must recognise rather than treat as an ordinary undeclared or forward reference).

**What this forbids**
- A step cannot use `${<this-step>.outputs}` (without the `?`) to assume its previous output always exists — the first pass is unconditionally `null`, so downstream logic must handle the absent case per §9a.2's null semantics.

## Evidence and confidence

[documented]: adjudicated in the source (§9b.25), grounded in the concrete example of P9's `dag` step under `acyclic.on_false`. No measurement is cited. The decision explicitly rests on and satisfies the loop-learning requirement recorded in amendment §9b.13 (ADR 0043); that record should be treated as the reason this amendment exists, not as independent justification.
