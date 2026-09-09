---
id: "0062"
title: Shared question ids are legal; a mark is identified by the triple
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-32
source: references/2026-09-07-activity-graph-schema.md §9c amendment 32
supersedes: []
superseded_by: []
related: ["0063"]
evidence: []
consumers: []
---

## Context and problem

The schema's escalation model (§3, §9 lint rule 6) requires every `escalate`/`gate` step to name a `QuestionId` from the pack's question registry (`questions.toml`, open question 2 in §10). Authoring 90 activities against this in the P6 batch raised the question of whether two different activities, or two different steps of the same activity, may reuse the same question id. The author did reuse them: F34 B7 records a shared `draft.accept` (23 uses) and `loop.exhausted`, against a schema whose registry "owns options" and a P9 batch in which every gate had its own id.

## Decision drivers

- The question registry (`questions.toml`) is pack-level and meant to hold a stable id, text and option set per question (§10 open question 2) — duplicating near-identical questions under distinct ids would fragment the registry without benefit.
- The kernel needs an unambiguous way to identify one open gate mark even when the same question id is escalated from more than one place.

## Considered options

### Option — do nothing
No alternative is named in the source row. Reasoned through: forbidding shared question ids would force every activity/step pairing to mint its own question even when the question text and options are identical, growing the registry without adding information and creating spurious near-duplicates for authors to keep in sync by hand.

## Decision

Shared question ids are legal: more than one activity, or more than one step within an activity, may declare the same `QuestionId`. A gate mark is identified by the triple `(activity, step, question)`, not by the question id alone.

## Consequences

**Positive**
- Lets the pack's question registry stay small and reusable across activities that need the same human decision (the batch's own `draft.accept`, reused 23 times), consistent with `questions.toml` being pack-owned (§10 open question 2).

**Negative**
- Anything that indexes marks or `HumanAnswer` evidence by question id alone will silently conflate distinct gates; consumers must key on the full triple.

**What this forbids**
- Forbids treating a `QuestionId` as a unique identifier for a mark or an answer. Code, tooling, or the comment grammar/web view (§10 open question 2) that assumes one question id maps to one gate is wrong under this rule.

## Evidence and confidence

[documented] Adjudicated in the §9c batch from the third author's 90-activity P6 catalog authoring pass (`spikes/F34-DRAFT.md` §4; `spikes/p9/DECISIONS-2026-09-08.md` decision 10, "32 accept (shared question ids)"). The only quantity the source attaches is the reuse count in F34 B7 (`draft.accept`, 23 uses); no measurement of the rule's effect accompanies this item — it is a naming/identity rule, not a result — and note that F34's proposed wording identified the mark by `(activity, step)`, which the adjudicated §9c text widens to the triple including the question. It and it should not be read as settling the separate, still-pending question (§9b, "decided pending P8") of whether a gate re-reached via loop-back is a new mark and a new `HumanAnswer`.
