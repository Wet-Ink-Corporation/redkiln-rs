---
id: "0023"
title: Rigor is a deontic enum, not a summed numeric weight
status: accepted
date: 2026-09-07
legacy_id: PC1
source: references/2026-09-07-process-catalog-proposal.md §7
supersedes: []
superseded_by: []
related: ["0020"]
evidence: ["OPF deontic matrix", "V-Modell XT mandatory modules", "rank reversal in weighted sums (selection §5)", "F27"]
consumers: []
---

## Context and problem

The 2026-09-05 architecture proposal §4.2 scored stage rigor with numeric weights (`0.2`/`0.5`/`1.0`). The catalog proposal's §0 verdict records that weighted-sum rubrics suffer rank reversal (adding an option changes the ranking of the others) and are hard to explain at a gate, citing an external selection research note (`[documented, selection §5]`). PC1's grounding column also cites F27 threshold instability. A form was needed that lets an activity declare its eligibility bar (`min_rigor`) without arithmetic deciding admissibility.

## Decision drivers

- Rank reversal under weighted summation, cited to the selection §5 research note via the proposal's §0 verdict — not a property measured on this corpus.
- F27 threshold instability, named in PC1's grounding column. The source's own §5 risk table states F27 as "the reviewer's threshold judgement is unstable" and adds that "the same risk applies to ordinal axes" — so the source does not present F27 as a reason the enum is safer than the numbers.
- Precedent in OPF's deontic matrix and V-Modell XT's mandatory-module vocabulary, where obligation is expressed ordinally, not numerically.
- The requirement that a skip be inspectable as a row ("why was this skipped"), not the residue of an arithmetic result.

## Considered options

### Option — numeric 0–1
The original architecture-proposal §4.2 scheme: stage weight is a float in `[0,1]`, activities compare against it. Rejected: rank reversal under summation, and "why was this skipped" is an arithmetic result rather than a row. (PC1's grounding also names F27 threshold instability, but see Evidence — that finding is about the reviewer's judgement and the source says it applies to ordinal axes too.)

### Option — enum with numeric sort key
An ordinal enum backed by a numeric sort key, keeping arithmetic available for tie-breaking or interpolation. Rejected in favour of a pure ordinal: the proposal gives no rationale for retaining a numeric key once ordering suffices, and it reintroduces the door to summation the enum was meant to close.

### Option — per-activity deontic matrix
A matrix mapping each activity individually to an obligation level per context, following OPF's own matrix form more literally. Not adopted as the mechanism; the chosen form keeps the enum at the stage-rubric/activity boundary (`min_rigor` on the activity, stage rigor from policy) rather than a full matrix per activity.

## Decision

Rigor is the deontic enum `forbidden < discouraged < optional < recommended < mandatory`, ordinal and **never summed**. Activities declare `min_rigor`; a stage runs at a rigor level set by `policy.<class>`, and an activity is eligible only when `min_rigor ≤ stage rigor`. This choice **supersedes the numeric `0.2/0.5/1.0` weights in the 2026-09-05 proposal §4.2** (the mapping onto the enum is one-to-one, so nothing already decided is lost, per §2.3 of the catalog proposal).

## Consequences

**Positive**
- A skipped activity is a logged row (`forbidden` is a logged skip, §2.5 step 1), not an arithmetic remainder.
- Ordinal comparison eliminates the specific rank-reversal failure mode of weighted sums.

**Negative**
- Ordinal levels cannot express fine-grained interpolation between two rigor bands; a scheme that needed that would have to reopen this decision.

**What this forbids**
- Summing or otherwise combining rigor values across activities or stages.
- Any admissibility check that compares rigor by numeric magnitude rather than ordinal position.

## Evidence and confidence

- [documented] OPF's deontic matrix and V-Modell XT's mandatory-module vocabulary are cited as prior art for the ordinal form; the proposal does not report a spike measuring this specifically against the enum's own performance.
- [inferred from measured] F27 is cited only as "threshold instability" in PC1's grounding column. The proposal's §5 risk table tags it `[inferred from measured]` and says F27 showed *the reviewer's threshold judgement* is unstable, with "the same risk applies to ordinal axes" — so F27 does not license a claim that the enum is more stable than the numeric form.
- [documented] Rank reversal under weighted sums is cited to an external selection research note (`[documented, selection §5]`) via the proposal's §0 verdict, not measured on this corpus.
- [documented] Spike **P3** ("rows vs weighted sum") was the spike that would have tested this choice empirically; §6.1 marks it **cut**, with the reason "PC1 decided the enum on documented grounds; re-open only if P6 shows `require` rows sprawling". So this decision rests on prior art and explainability, not on a measurement, and **P6** (catalog lint coverage) is the unrun spike that can reopen it.
- This decision was later promoted into the main architecture-proposal ledger as D20 (ADR 0020); ADR 0023 and ADR 0020 record the same decision from its two homes and are marked `related`, not superseding one another.
