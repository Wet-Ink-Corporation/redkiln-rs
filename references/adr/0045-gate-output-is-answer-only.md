---
id: "0045"
title: A gate step's output is answer, never outputs
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-15
source: references/2026-09-07-activity-graph-schema.md §9b amendment 15
supersedes: []
superseded_by: []
related: ["0038", "0063"]
evidence: []
consumers: []
---

## Context and problem

§8's Rust sketch types a `gate` step as `Gate { question: QuestionId, options: Vec<OptionId> }`, distinct from `Agent`/`Tool`, but §7's TOML shape and §1's step table never state what a downstream step reads back from a gate, or whether a gate exposes `.outputs` the way `agent`/`tool`/`for_each` steps do. Left open, the question arose in the second author's P9b batch (three activities written against §1–§9a) and was adjudicated as amendment 15.

## Decision drivers

- A gate produces a human decision (an `OptionId`), not a computed value from a typed output schema the way `agent`/`tool` steps do; conflating the two under `.outputs` blurs a distinction the rest of §1's step table otherwise keeps closed.
- Downstream refs need one settled field name to resolve against (lint rule 3, path resolution) rather than an open question per author.

## Considered options

### Option — do nothing (gate exposes .outputs like other steps)
Let a `gate` step's result be read as `${gate.outputs}`, consistent with `agent`/`tool`/`check`. The source does not name this as a considered alternative, but it is the status quo the amendment closes off: §7/§8 gave no field name for a gate's result before this amendment, and `.outputs` was the shape every other non-`emit` step already used, making it the natural default guess. It is rejected here because a gate's result is a chosen `OptionId`, not a typed computation output, and treating it as `.outputs` leaves no distinct field for the human-decision case to later grow attachments into (§9c amendment 33).

## Decision

"Gate output is `answer`, only. Downstream reads `${<gate-step>.answer}`; `.outputs` on a gate is a lint error; §8 `Gate` gains `→ answer: OptionId`. Until the post-P7 shorthand lands, the documented pattern for `on_exhausted = "escalate"` is an explicit `gate` step with `when` over the looped step's status."

## Consequences

**Positive**
- Gives lint rule 3 (path resolution) one settled field to resolve gate refs against, instead of an open question per author.
- Keeps a human decision typed and distinct from a computed step output, consistent with §1's closed five-kind step table.

**Negative**
- Adds a special case to the step-output surface: `gate` alone among the five closed step kinds is read through `.answer`, where the kinds that carry a declared output schema (`agent`, `tool`) are read through `.outputs` — an asymmetry an author must remember, and one lint must encode rather than resolve uniformly.

**What this forbids**
- Forbids any reference to `${gate.outputs}` anywhere in the graph; lint rejects it as an error, not a warning.
- Forbids treating a gate's answer as anything other than the closed `OptionId` type — it is not a free-form value.

## Evidence and confidence

- [documented] Stated normatively in §9b amendment 15, from the second author's three-activity batch (`spikes/F33-DRAFT.md`).
- [inferred] The source does not attribute a specific count of the batch's 27 ambiguities to this particular question, nor does it name this as one of the five newcomer guesses lint v1 let through; no such number is claimed here.
- [open] The field name settled here is not spike-conditional, but an adjacent gate question in the same §9b block is: the source records "Decided pending P8: a gate re-reached via loop-back is a new mark and a new `HumanAnswer`." P8 is partial (3 of a pre-registered 27+3 runs; the second-arrival question is recorded as not answerable as run), so how a looped gate's repeated `answer` accumulates is not settled by this record.
