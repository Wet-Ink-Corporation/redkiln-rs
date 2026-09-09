---
id: "0063"
title: Gate output carries typed attachments bound to question options
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-33
source: references/2026-09-07-activity-graph-schema.md §9c amendment 33
supersedes: []
superseded_by: []
related: ["0025", "0030", "0062"]
evidence: []
consumers: []
---

## Context and problem

§9b.15 had already fixed a gate step's output to `answer` only, with `.outputs` on a gate treated as a lint error. Authoring the P6 catalog surfaced cases where a human answer to a gate needs to carry more than a chosen option: F34 B11 records gates that must return data — user-test results, the chosen sketch, attached session notes — where the author could only write enum options like `results-attached` while "the results themselves have no channel". PC8 rule 5 forbids free-text answer fields, so the schema needed a typed way for a gate to receive an artifact from the human without reopening free text.

## Decision drivers

- PC8 rule 5's prohibition on free-text answer fields must hold even when the human needs to supply more than a choice.
- §9b.15's `answer`-only gate output must not be violated by an ad hoc second field.
- PC3's vouched-root model (`derived_from = []`, never blocks or re-runs) already exists for human-supplied artifacts and is the natural home for something a human attaches at a gate.

## Considered options

### Option — free-text answer fields
Let a gate collect arbitrary text alongside the chosen option. Rejected and explicitly listed among the batch's rejections ("free-text gate answers (33)") — it reopens exactly the ungoverned prose input PC8 rule 5 closed.

### Option — do nothing (answer only, no attachment path)
Keep gates limited to `answer: OptionId` per §9b.15, with no way to bind an artifact at the point of the human decision. The source does not name this as considered, but it is the status quo the amendment changes: it would leave activities with no typed way to let a human hand over supporting material at a gate, forcing that material through a separate, unlinked step.

## Decision

A gate's output is `answer` plus `attachments: [ArtifactRef]`. A question option may declare `attaches = <Kind>`; the kernel refuses to close the mark unless that attachment is supplied. The attachment is stored as a human-supplied root under PC3's `derived_from = []` model, and is readable downstream as `${<gate>.attachments.<Kind>}`. Free-text answer fields remain forbidden per PC8 rule 5.

## Consequences

**Positive**
- Lets a human decision at a gate carry evidence (an artifact) without reintroducing free text, closing the gap left by §9b.15's `answer`-only rule.
- Reuses PC3's existing vouched-root semantics rather than inventing a second human-input mechanism.

**Negative**
- Adds a required-attachment obligation the kernel must enforce before a mark can close, a new failure mode (mark cannot close because the declared attachment is missing) that did not exist when gates only returned an option id.

**What this forbids**
- Forbids free-text answer fields on a gate, unconditionally (PC8 rule 5, reaffirmed here).
- Forbids closing a mark whose chosen option declares `attaches = <Kind>` without that attachment present — the kernel refuses.

## Evidence and confidence

[documented] Adjudicated in the §9c batch from the P6 authoring pass (`spikes/F34-DRAFT.md` §4; `spikes/p9/DECISIONS-2026-09-08.md`), which also lists "free-text gate answers (33)" among the batch's explicit rejections. The grounding recorded for the decision is a count of need, not a measurement of effect: F34 B11 (marked consequential — "discovery's human steps are theatre without it") and `DECISIONS-2026-09-08.md` decision 7, "5 discovery/analysis gates in the batch need data". Nothing here licenses any claim about how often attachments will be used or how costly enforcing them is at runtime.
