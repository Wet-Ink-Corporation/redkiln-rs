---
id: "0026"
title: Review is three encapsulated levels, not a stage-visible reviewer
status: accepted
date: 2026-09-07
legacy_id: PC4
source: references/2026-09-07-process-catalog-proposal.md §7
supersedes: []
superseded_by: []
related: ["0028", "0004"]
evidence: ["12207/DA V&V alongside technical processes", "Dagster asset checks inside materialisation", "proposal §4.6 findings-not-verdicts", "F23", "F26"]
consumers: []
---

## Context and problem

Quality work (self-review, adversarial review, retry, escalation) needed a place to live in the process model without becoming either invisible catalog-wide policy or a stage-schedulable unit of work that every pack must wire up separately. F23/F26 are cited grounding for headless-review behaviour that this shape has to accommodate. The proposal's own §4.6 findings-not-verdicts design (D4/ADR 0004) constrains what a review step is even allowed to hand back.

## Decision drivers

- F23/F26's evidence on headless review.
- Proposal §4.6's findings-not-verdicts rule (D4): a review outcome is not a pass/fail verdict, so wherever review lives it must attach evidence, not a verdict.
- ISO/IEC/IEEE 12207 and DA's pattern of running V&V alongside, not embedded serially inside, technical processes.
- Dagster's asset-checks precedent for evaluating quality predicates over already-materialised output rather than scheduling quality as separate pipeline stages.

## Considered options

### Option — stage only
Put all review inside a stage as ordinary stage work, with no activity-internal quality mechanism. Rejected: this makes every activity's internal retry/escalation logic stage-visible plumbing, which the chosen shape (b) explicitly declines — stage-exit `checks` are acceptance predicates over evidence the activity already attached, never extra work the stage schedules.

### Option — gate attribute only
Express review purely as an attribute on the human gate mechanism, with no activity-internal or stage-exit layer. Rejected: this collapses three distinct concerns (an activity's responsibility for its own output quality, a stage's acceptance check, and the item-level adversarial change review) into one mechanism, which the source treats as three separable levels.

### Option — stage-visible sibling reviewer nodes
Model an adversarial reviewer as a sibling node visible at the stage/workflow level, alongside the activity it reviews. Rejected by PC6: PC6's decision is that nothing about internal review, retries, or escalation should leak into stage or workflow definitions — a stage-visible reviewer node is exactly what that constraint rules out.

## Decision

Review has **three levels, encapsulated** (subsequently amended by PC6, ADR 0028):

(a) **Activity-internal**: an activity is itself a graph responsible for the quality of its own output — self-review, adversarial reviewer, retry, and HITL escalation all live *inside* it and are invisible to the stage.

(b) **Stage-exit `checks`**: acceptance predicates the stage evaluates over evidence the activity attached — never extra work the stage schedules.

(c) **A formal `review` stage** for the item-level adversarial change review, which no single activity owns.

## Consequences

**Positive**
- A stage's definition stays free of the internal retry/escalation machinery of the activities feeding it — that machinery is amortised inside each activity once.
- Item-level adversarial change review has an explicit home (level c) distinct from an activity's self-quality-control (level a), so neither is asked to substitute for the other.

**Negative**
- Three levels is more structure to reason about than a single review mechanism; understanding "why did this get flagged" requires knowing which of the three levels produced the flag.

**What this forbids**
- Scheduling quality work as extra stage-level steps beyond evaluating the `checks` predicates over evidence already attached (level b is acceptance-only, never additional work).
- A stage-visible sibling reviewer node running alongside the activity it reviews — this option was considered and rejected by PC6.
- A review outcome expressed as a verdict rather than findings-plus-evidence, per proposal §4.6 (D4/ADR 0004), at any of the three levels.

## Evidence and confidence

- [documented] ISO/IEC/IEEE 12207 and DA's V&V-alongside-technical-processes pattern, and Dagster's asset-checks-inside-materialisation pattern, are cited as prior-art precedent, not as measurements taken on this corpus.
- [documented] Proposal §4.6's findings-not-verdicts rule (D4) is a constraint this decision inherits, not independently re-derived here.
- [documented] F23/F26 are cited by the row as "headless review" grounding. The row does not restate their content and gives no number, so nothing here is stated as [measured].
- [unmeasured] Level (a) — the activity-internal adversary — is the subject of unrun spike **P7** ("encapsulated quality, cost vs catch"): does an activity's internal adversary + policy catch more per token than a single dispatch per artifact, measured against the F23/F26 single-dispatch baseline? Its kill criterion ("`mandatory` > 3× `recommended` cost with no extra catches") tunes the internal default rigor rather than removing the level, but §6.1 lists P7 among the spikes that "can each overturn a taken decision". Until it runs, the three-level shape is a design choice, not a measured one.
- This decision is recorded as amended by PC6 (ADR 0028, quality encapsulation) — the encapsulation principle stated loosely here becomes PC6's normative two kernel constraints. The two are `related`, not one superseding the other, since PC6 sharpens rather than reverses this shape.
