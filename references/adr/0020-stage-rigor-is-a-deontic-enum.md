---
id: "0020"
title: Stage rigor is a deontic enum
status: accepted
date: 2026-09-07
legacy_id: D20
source: references/2026-09-05-architecture-proposal.md §4.2; references/2026-09-07-process-catalog-proposal.md §7 PC1
supersedes: ["0015"]
superseded_by: []
related: ["0023"]
evidence: ["F27"]
consumers: []
---

## Context and problem

§4.2's process packs select stage rigor from a numeric weight in `[0, 1]`: Triage 0.2, Investigation 0.5, Implement 1.0, Verify 1.0, Review 1.0, Closeout 0.5, with `0` an explicit logged skip. D15 (review tiering) uses the same scheme to choose between read-only (`0.2`) and execution-grade (`1.0`) review. PC1's grounding column names what pushed the form off numbers: the OPF deontic matrix and V-Modell XT's mandatory modules as prior art, "rank reversal in weighted sums (selection §5)", and "F27 threshold instability". That last citation must not be read as more than it is: F27 is the spike finding that the *reviewer's* verdict is unstable at the threshold while its detection is stable (architecture proposal §1, question 15; §4.6 rev. 4) — it was not run against the numeric rigor scheme. The catalog proposal's §5 risk table makes the transfer explicit and marks it as such: "F27 showed the reviewer's threshold judgement is unstable; the same risk applies to ordinal axes [inferred from measured]". A numeric weight also carries a further objection the catalog states in its own voice (§2.3): what is gained by the enum is that "why was this skipped" is a row, not an arithmetic result.

## Decision drivers

- Rank reversal is a known failure of weighted-sum selection — "adding an option changes the ranking of the others" — and weighted rubrics are hard to explain at a gate (catalog §0, marked `[documented, selection §5]`). PC1's grounding column cites it for the row as a whole; the source does not attach it to any one rejected alternative.
- F27, cited in PC1's grounding as "threshold instability" — but see the Evidence section: F27 measured the reviewer's verdict threshold, and its application to an ordinal rigor axis is marked `[inferred from measured]` in the catalog's own §5 risk table.
- Prior art for a deontic ordering exists and is cited directly: the OPF deontic matrix and the V-Modell XT mandatory-modules scheme (V-Modell XT's tailoring is documented as two decades in production).
- The scheme needs to support ordinal comparison (`forbidden < discouraged < optional < recommended < mandatory`) without licensing summation; the catalog states the intended semantics directly (§2.3): "ordinal, never summed: an activity with `min_rigor = recommended` is ineligible when the stage runs at `optional`".

## Considered options

### Option — numeric 0–1

Keep the existing `0.2`/`0.5`/`1.0`-style weight per stage (as used in §4.2's `policy.bug-standard` table and in D15). Rejected: it is the status quo PC1 explicitly supersedes, on the grounds listed in PC1's grounding column — rank-reversal risk in weighted-sum selection, the F27 threshold-instability transfer, and (catalog §2.3) that a skip should be a row rather than an arithmetic result.

### Option — enum with numeric sort key

Use a named enum but back it with a numeric sort key (effectively numeric weights with enum labels attached). Named directly as a rejected alternative in the PC1 decision row (and in the D20 row). The source records no reason for its loss. Inferred, and marked as inference: it does not remove the mechanism PC1 forbids, because a numeric sort key can still be summed or averaged even with a label layered on top, whereas "never summed" is the property the chosen form is defined by.

### Option — per-activity deontic matrix

Assign a deontic value per activity rather than per stage, following the OPF matrix form more literally. Named as a rejected alternative in the PC1 decision row (it does not appear in the D20 row's shorter alternatives list). The source records no reason for its loss; what the source does show is the chosen shape — stage rigor set by `policy.<class>`, and a single `min_rigor` declared per activity — rather than an obligation value per activity per context.

## Decision

Stage rigor is a **deontic enum**: `forbidden < discouraged < optional < recommended < mandatory`, ordinal, never summed. Activities declare `min_rigor`. This supersedes the numeric weights in §4.2 and in D15 (review tiering). Anything already written numerically maps as: `0 → forbidden`, `0.2 → optional`, `0.5 → recommended`, `1.0 → mandatory` — under this mapping, D15's read-only review becomes `optional` and execution-grade review becomes `mandatory`.

## Consequences

**Positive:**
- Removes the rank-reversal exposure of weighted-sum selection (PC1's cited rationale) by making the comparison strictly ordinal.
- Gives §4.2's existing per-stage weight table (`policy.bug-standard`) and D15's read-only/execution-grade review split a direct, lossless mapping onto named levels, so no existing policy table needs to be redesigned, only relabelled.
- Aligns with prior art with a documented track record (OPF deontic matrix, V-Modell XT mandatory modules).

**Negative:**
- Every place in the corpus that wrote or reasoned about rigor as a number (§4.2's policy tables, D15's `0.2`/`1.0` split) must be re-expressed as an enum value. The source calls this mapping one-to-one and says "nothing already decided is lost" (catalog §2.3), which holds for the four weights actually written; a value between them that some future pack author might have written (e.g. `0.7`) has no defined mapping, and `discouraged` has no numeric counterpart at all — both observations are this ADR's, not the source's.
- Loses whatever fine-grained numeric distinction a `0-1` scale allowed (e.g. distinguishing `0.3` from `0.4`); the enum only distinguishes five ordinal bands.

**What this forbids:**
- It forbids summing or averaging rigor values across stages or activities — the enum is ordinal only, never arithmetic, which is the property the "enum with numeric sort key" alternative failed to provide and was rejected for.
- It forbids per-activity deontic assignment as the selection mechanism — the per-activity deontic matrix alternative was considered and rejected in favour of keeping rigor at the stage level via `min_rigor` declared per activity.

## Evidence and confidence

- [inferred from measured] F27 is cited in PC1's grounding as "threshold instability", and this ADR's earlier drafts read that as instability measured in the numeric rigor scheme. It is not. F27 is the reviewer-reliability finding — "Detection stable (0/7 flaky, 21/21 draws). Verdict unstable (2/4 items flip)" (architecture proposal §1, question 15), i.e. instability in the *reviewer's verdict threshold*, at n=2–3 on arm (b) only. The catalog's §5 risk table performs the transfer explicitly and labels it: "F27 showed the reviewer's threshold judgement is unstable; the same risk applies to ordinal axes [inferred from measured]". No measurement of the numeric `0–1` rigor scheme itself is cited anywhere in the sources available to this cohort.
- [documented] Rank reversal in weighted sums is cited as a known property of weighted-sum rubrics (catalog §0, `[documented, selection §5]`; repeated in PC1's grounding column). It is a documented general risk, not a measurement run against this system's own selection logic, and the source attaches it to the row as a whole rather than to any individual rejected alternative.
- [documented] OPF deontic matrix and V-Modell XT mandatory modules are cited as prior art supporting the enum's plausibility; they are not evidence of this system's own behaviour.
- [inferred] The four-point numeric-to-enum mapping (`0/0.2/0.5/1.0 → forbidden/optional/recommended/mandatory`) is stated as the mapping "for anything already written numerically" — it is a conversion rule applied retroactively to D15 and §4.2's existing tables, not itself independently measured or re-validated against the corpus.
- This decision was promoted from PC1 (process-catalog-proposal §7) into D20 of the architecture proposal; both rows describe the same decision (see related: PC1 is ADR 0023).
