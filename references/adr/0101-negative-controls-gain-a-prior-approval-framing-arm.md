---
id: "0101"
title: Negative-control fixtures gain a prior-approval framing arm, cited to the paper that measured it and reported on both halves of the classifier
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-013 (§3.5), PIA-095 (§4.3); arXiv 2603.18740 v2 (full text, fetched 2026-09-10); references/adr/0088; spikes/F24, F29-DRAFT.md, F31-DRAFT.md
supersedes: ["0088"]
superseded_by: []
related: ["0004", "0019", "0074", "0082", "0096"]
evidence: ["PIA-013", "PIA-095", "F24", "F26", "F27", "F29", "F31", "S-SUBTLE"]
consumers: []
---

## Context and problem

ADR-0088 adds a "presented as already approved" arm to the S-SUBTLE negative-control family on the strength of "a
controlled experiment over 250 CVE vulnerability–patch pairs" finding that bug-free framing "reduces the
vulnerability-detection rate by 16–93%" with "false negatives rise sharply while false positives barely move". Its
source row names the paper by a different title and its mechanism is inverted [measured, PIA-013]. The paper is
**"Measuring and Exploiting Contextual Bias in LLM-Assisted Security Code Review"** (arXiv 2603.18740; v2 2026-04-23
titled "Contextual Bias"); 250 pairs is right; the effect is **16–93 percentage points** across models under *strong*
bug-free framing on 5 of 6 models — Opus 4.5 moved **−4.9 pp, not significant** — and 0.8–24.7 pp under *weak*
framing; and the mechanism is a **whole-classifier shift toward "clean"**: false negatives rise **and false positives
collapse** (+46 to +95 pp true negatives), the opposite of "barely move" [documented, full text fetched 2026-09-10].
ADR-0088 rejected the symmetric "presented as already rejected" arm on the ground that "FP barely move", which the
paper does not support. Its evidence line also credits F24 with "≥13 of 23" (F24 said ≥5/23; the 13 is F31's) and
says "four survived fully green" where two did [measured, PIA-095].

The decision — add the arm, measure detection, report false negatives separately, keep CLEAN observational, register
per 0012, size before running — was right. Its citation, its quoted effect, its mechanism and one of its rejections
are wrong, and the mechanism is load-bearing for the arm's pre-registration. This is a **full** supersession: every
operative sentence of ADR-0088 is restated below, corrected; ADR-0088 moves to `superseded`.

## Decision drivers

- The pre-registration for this arm must predict the right direction on both halves of the classifier or the
  arm's result is uninterpretable (METHOD-S: predictions before any draw).
- A rejection argued from a false premise is not a rejection; the symmetric arm has to be re-argued or reopened.
- The gate is measured high-precision/low-recall (F29: a reproduced regression approved 3/10; F31: two of four
  adjudicated defects escaped through fully green suites), so a framing that moves the whole classifier toward
  "clean" is the failure it can least afford — the driver survives the correction.

## Decision

1. **The S-SUBTLE family gains a "prior-approval framing" arm** (the name replaces "presented as already approved"):
   known-defective items whose card asserts the change was reviewed and approved, in the paper's strong-framing form.
2. **The measured quantity is detection**, because the manipulation predicts a whole-classifier shift and detection
   is the stable half of the instrument (F27: 0 of 7 items flaky over 21 draws); a verdict-side reading is not
   admissible.
3. **Both halves are reported, separately, every time.** Missed seeded defects (FN) *and* the **CLEAN-pair approval
   rate** (the false-positive half) are reported distinctly, because the effect is a shift of the operating point and
   a pooled error rate hides it — and because the corrected mechanism predicts FP will *fall*, which a pooled rate
   would read as an improvement.
4. **CLEAN is an observational instance, never the controlled arm.** CLEAN is genuinely prior-approved input carrying
   real defects — **F24 established ≥ 5 of 23, raised to ≥ 13 of 23 by F31** — usable as evidence that the framing
   occurs in the field and unusable as the controlled comparison.
5. **The arm is a named spike with a named consumer, registered per 0012, and it is unrun.** Consumer: the
   negative-control fixture design under ADR-0096 and the review card's framing rules. **Until it runs, no document
   may claim the review gate is guarded against false approval**, and no FP rate from S-VAR or S-TYPED is evidence of
   overall calibration.
6. **The arm is sized before it runs.** At n=5 with detection at ceiling only a large effect is observable; the
   registration carries its sizing or reports a bound. Its **predictions** — detection down; CLEAN-pair approval up;
   Opus-class models possibly null — go into METHOD-S before any draw.
7. **The symmetric "presented as already rejected" arm is registered as a follow-on spike, S-SUBTLE-SYM** (consumer:
   the review-gate policy table; cost ≈ 2 framings × 5 items × k draws at S-TYPED's median $3.03/review), not
   rejected. The false-rejection direction is the one the gate's re-execution guard already protects against, which
   is why it is second, not why it is absent.

## Considered options

### Option — rely on the existing negative controls
Rejected, as in ADR-0088: a 0/10 stable-approve (F29) is consistent with a calibrated judge and a lenient one.

### Option — add the arm to S-SILENT instead
Rejected, as in ADR-0088: S-SILENT is saturated (23/23 across arms; McNemar p=0.625 on the CLEAN FP 3-vs-1 discordant
pairs, not on detection); it remains a confirmatory second family.

### Option — reject the symmetric arm on cost alone
The audit's suggested re-argument. Rejected by the operator in favour of registering it: the corrected mechanism says
framing moves *both* halves, so the false-rejection half is a measurable question with a named consumer, and a
question with a consumer is a spike row, not a paragraph.

### Option — keep ADR-0088 and file the corrections as errata
Rejected: the mechanism sentence is in the Decision's rationale and drives clause 3's reporting rule; an erratum may
not move that (ADR-0092).

## Consequences

**Positive**
- The arm's pre-registration predicts the right direction on both halves; a null on Opus-class models is an
  anticipated result, not a surprise.
- The corpus's account of F24/F31 is what the spike files say.

**Negative**
- Two arms in a family whose binding constraint is wall clock (S-TYPED CLEAN reviews: median 1,071 s, max 4,561 s).
- These runs assert something false to the reviewer and are excluded from pooled detection figures (unchanged).
- The corpus carries framing-sensitive risks in both directions with no card design known safe against both
  (unchanged).

**What this forbids**
- Forbids describing the paper's effect as a percentage reduction, or its mechanism as FP-stable.
- Forbids reporting this arm's detection without the CLEAN-pair approval rate beside it.
- Forbids claiming the gate is guarded against false approval before the arm runs.
- Forbids citing F24 for ≥ 13/23.
- Forbids running the arm before its predictions are in METHOD-S.

## Evidence and confidence

- [documented, arXiv 2603.18740 v2, full text fetched 2026-09-10] 250 CVE/patch pairs; strong bug-free framing:
  detection down 16–93 pp on 5 of 6 models, Opus 4.5 −4.9 pp n.s.; weak framing 0.8–24.7 pp; TN up +46 to +95 pp.
- [measured, F24 (FINDINGS.md:1350)] ≥ 5/23 CLEAN items carry a defect; [measured, F31-DRAFT:91] raised to ≥ 13/23.
- [measured, F31] two adjudicated defects escaped through fully green suites (issue-70 AC-001 1,890/1,890; issue-83
  AC-002 1,907/1,907); two (issue-131) with zero new failures against an already-red 54-failure baseline.
- [measured, F29] a reproduced regression approved 3/10; negative control stable-approve 0/10.
- [measured, F27] detection flaky on 0/7 items over 21 draws; verdict flaky on 2/4.
- **What this evidence does not license.** It does not license predicting the size of the effect on this gate's
  reviewer model — the paper's per-model spread is 16–93 pp with one null. It does not license any claim about the
  symmetric arm; S-SUBTLE-SYM is unrun.
