---
id: "0111"
title: The S-P7B re-run must change the criterion, not only n — P7's criterion 1 had zero power at n = 3 and ten draws per cell are still powerless for the effect it registers
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-073 (§4.4), PIA-077; references/adr/0080 decision 5; spikes/F37-DRAFT.md §2 and Limits 1; spikes/METHOD-P.md §4.4; references/2026-09-08-architecture-design.md §10.5 :1870
supersedes: ["0080"]
superseded_by: []
related: ["0020", "0023", "0081", "0106"]
evidence: ["PIA-073", "PIA-077", "F37", "F36", "F39"]
consumers: []
---

## Context and problem

ADR-0080 decision 5 says "what would settle the default: more draws per cell, roughly ten — not more items. At n = 10
per cell a 1-in-10 effect is resolvable". The arithmetic the audit re-ran says otherwise. P7's criterion 1 compared
3/3 against 0/3: Wilson 95% intervals [0.439, 1.000] and [0.000, 0.561] **overlap by construction** — at n = 3 no two
Wilson intervals can be disjoint — so §4.4(1) had **zero power** and could not have failed to fire [measured,
PIA-073]. At n = 10 per cell, 6/10 is the first cell whose interval separates from 0/10; an effect of 0 vs 0.33 needs
roughly **n = 20 per cell** under interval overlap. S-P7B as registered ("n = 10 per cell") is therefore powerless for
the effect ADR-0080 registers it to detect — the fourth instance of the programme's pre-registration pattern
(golden constraint 10; F36's α rule, F39's 0.95 bar, F40's recall) and now the fifth. The `mandatory` cost ratio behind
R-P7RATIO is two denominators (ADR-0106 clause 3); the `mandatory` figure stays n = 1 (ADR-0081).

This record supersedes ADR-0080 **in part**: decision 5. Decisions 1–4 (the block is retained; the empty-set
terminal predicate is forbidden; the threshold waits on a severity vocabulary; the shorthand ships with no default)
and every forbids bullet stand; ADR-0080 stays `accepted`. F37 gains a dated erratum stating the power result; the
design's S-P7B row carries the corrected sizing.

## Decision drivers

- A criterion whose registered n cannot decide it is reported powerless, never re-thresholded (golden constraint 10)
  — and a re-run that inherits the criterion inherits the fault.
- Adding draws to a success-rate criterion whose cells sit at 0 and 1 narrows nothing that overlaps by construction.

## Decision

1. **S-P7B's re-run changes the criterion.** Its pre-registration replaces §4.4(1)'s per-cell success-rate comparison
   with either **a paired per-item comparison** (each item run under both arms, the paired difference tested) or **an
   interval on the pooled difference of proportions with a stated minimum detectable effect and the n that reaches
   it** — for 0 vs 0.33, roughly n = 20 per cell under interval overlap; fewer under a paired design. The registered
   wording of P7's §4.4 stays verbatim; the re-run is a new METHOD-P amendment, dated before any draw.
2. **The sizing is in the register.** Design §10.5's S-P7B row reads "the quality shorthand under a paired or
   pooled-difference criterion at the n that reaches a stated MDE, with a settled severity vocabulary; P7's n = 10
   per cell is powerless for a 0 vs 0.33 effect [measured]".
3. **F37 §2 and Limits 1 gain a dated erratum**: "at n = 3 no two Wilson intervals can be disjoint; the criterion could
   not fail to fire at this n"; the design's :24/:103 restatements follow.
4. **The `mandatory` figure is n = 1 and stays so** until the re-run; R-P7RATIO is reconciled by denominator
   (ADR-0106).

## Considered options

### Option — keep decision 5 and run ten draws per cell
Rejected: powerless for the registered effect; a second pre-registration with the same fault.

### Option — thirty draws per cell with the same criterion
Rejected on cost: P7 spent $131.62 of a $200 cap for 61 runs; a success-rate design at n = 30 per cell is a
multiple of that for a criterion a paired design answers at a fraction.

## Consequences

**Positive**
- The re-run is designed to be able to fail.

**Negative**
- The quality shorthand's default waits on a re-designed spike, not just a re-run.

**What this forbids**
- Forbids running S-P7B under §4.4(1)'s criterion at any n.
- Forbids setting the quality shorthand's default from any per-cell comparison whose intervals overlap by
  construction.
- Forbids quoting P7's criterion 1 as having fired without stating it had zero power.

## Evidence and confidence

- [measured, PIA-073] Wilson 95% at n = 3: 3/3 → [0.439, 1.000], 0/3 → [0.000, 0.561]; at n = 10, 6/10 first separates
  from 0/10; 0 vs 0.33 needs ≈ n = 20 per cell.
- [measured, F37] P7's 61 runs, $131.62; criterion 1 fired on 10/10 items.
- [measured, F36, F39, F40] the three earlier powerless pre-registrations this joins.
- **What this evidence does not license.** It does not license any effect size for the shorthand's tiers; the re-run
  measures that.
