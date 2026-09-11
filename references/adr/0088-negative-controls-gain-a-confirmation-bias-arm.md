---
id: "0088"
title: Negative-control fixtures gain a confirmation-bias arm
status: superseded
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §3 (verification) and §4 rec 13; §1 row 9; spikes/F29-DRAFT.md; spikes/F31-DRAFT.md; spikes/F27, F26 (FINDINGS.md)
supersedes: []
superseded_by: ["0101"]
related: ["0004", "0019", "0082"]
evidence: ["F24", "F26", "F27", "F29", "F31", "S-SUBTLE"]
consumers: []
---

## Context and problem

The review gate is designed against **one** direction of judge bias. Jin & Chen measure that rationale-heavy prompting drives false *rejection* — GPT-4o's false-negative rate on their regime moves from 26–36% to 58–88% — and the design answers it directly: the reviewer returns evidence rather than a class or a verdict, blocking requires the deterministic gate to re-execute the repro and observe a flip, and "both fail" or "cannot run" degrades to advisory (D4's rev. 4 rebuild; the fix-guided verification filter is the named precedent). Every negative control in the corpus, and every false-positive rate reported from S-VAR and S-TYPED, measures this same direction.

The mirror-image risk is unaddressed. A controlled experiment over 250 CVE vulnerability–patch pairs finds that **framing a change as "bug-free" before review reduces the vulnerability-detection rate by 16–93%**, and that the effect is **strongly asymmetric: false negatives rise sharply while false positives barely move**. Nothing in the corpus measures that. The fixture families are built to ask "does the reviewer reject something clean?" and never "does the reviewer approve something broken because it looks approved?"

This is not a hypothetical framing for redkiln. The CLEAN corpus **is** prior-approved input: every item in it shipped and was reviewed and approved in the real repository, and F24 established its ground truth is wrong for at least 13 of 23 items (57%) — the defects were there and the human process approved them anyway. "This shipped and was approved" is therefore exactly the framing a real review card carries into the gate, and exactly the framing the literature says degrades detection.

The standing principle makes the gap sharper rather than softer. Negative controls exist in this corpus **because a stable approval is consistent with a lenient judge**: F29's control was `stable-approve` at 0 rejections in 10 draws, which is the correct behaviour and is also indistinguishable from a reviewer that approves everything. A control that only ever confirms approvals cannot separate those two readings. Only a control whose items are *known defective* and whose framing asserts approval can.

## Decision drivers

- The measured effect is large, on-axis (security code review), controlled, and recent: 250 CVE patch pairs, detection down 16–93%, asymmetric toward false negatives.
- The design's existing guard (re-execute before blocking) protects against over-rejection and does nothing against under-detection: a finding never raised cannot be re-executed.
- The gate is already known to be high-precision and low-recall — a reproduced regression was approved 3 of 10 draws (F29), four adjudicated defects survived fully green suites (F31) — so a further recall loss under positive framing is the failure mode the corpus can least afford.
- Detection is the corpus's *stable* measurement (F27: 0 of 7 items flaky over 21 draws) while the verdict is the flaky one (2 of 4 items flaky), so a detection-side manipulation is measurable where a verdict-side one would not be.
- S-SUBTLE sits at ceiling — 5/5 sighted and 5/5 blind across two harder tiers, 20/20 overall (F26) — which gives a degradation arm full headroom in the direction the effect is predicted to move.
- The programme's own method finding: every discriminating result it produced came from cost, stability or mechanism, never from success rates, because five metrics saturated. A saturated metric is useless for showing improvement and is exactly the right instrument for showing a drop.

## Considered options

### Option — rely on the existing negative controls

Keep the CLEAN control set and treat F29's 0/10 as evidence the gate is not biased. Rejected: the existing control measures rejection of clean-*looking* input under neutral framing. A 0/10 stable-approve result is consistent both with a correctly-calibrated reviewer and with a lenient one, which is the ambiguity the control was introduced to resolve and cannot resolve alone. It says nothing about detection under asserted approval.

### Option — add the arm to S-SILENT instead

Use the 23-item seeded-defect family. Rejected as the primary home: S-SILENT is saturated at 23/23 across every arm run against it, and F23 already recorded that a saturated primary metric has **no discriminating power** — the E3 kill criterion failed to fire for exactly this reason (23/23 vs 23/23, McNemar p=0.625). S-SILENT remains available as a confirmatory second family once the effect size is known; it is not where the arm is registered.

### Option — add a symmetric "presented as already rejected" arm

Test both framings for balance. Rejected on the measured asymmetry and on cost: the source finds false positives barely move under framing manipulation, the over-rejection direction is already covered by Jin & Chen and answered by the re-execution rule, and a second arm doubles the draws on a family where wall clock, not headroom, is the binding constraint (16.2 h for 60 draws in S-VAR).

### Option — strip provenance from the review card instead of measuring the effect

Sanitise the input so the reviewer never learns a candidate was previously approved. Rejected as a substitute for the arm, though not as a later mitigation: the production gate reviews a candidate that has by construction passed earlier stages, so some approval signal is inherent to the context; and choosing a mitigation before the effect is measured would fix an unmeasured quantity at an unknown cost to the gate's other inputs.

## Decision

**The S-SUBTLE fixture family gains a "presented as already approved" arm, and false negatives are reported separately from false positives wherever it is used.**

1. **The arm.** Each S-SUBTLE item is presented twice: once under the existing neutral framing, and once under an explicit prior-approval framing that asserts the change shipped, was reviewed and was approved. Same items, same typed `Findings` schema, same policy table, same stage weight; the framing is the only manipulated variable, and the two presentations are **paired per item**.
2. **The measured quantity is detection, not verdict.** The arm reports whether a finding is raised against the seeded defect, because the manipulation predicts a false-negative shift and because detection is the stable half of this instrument while the verdict is the flaky half. A verdict-side reading of this arm is not admissible.
3. **False negatives are reported explicitly and separately.** A negative control's result is not a single rate. Every use of this family reports missed seeded defects distinctly from spurious findings, because the effect being tested is asymmetric and a pooled error rate would hide it.
4. **CLEAN is an observational instance, never the controlled arm.** The CLEAN set is genuinely prior-approved input carrying real defects (≥13/23 by F24), which makes it evidence *that the framing occurs in the field* and unusable as the controlled comparison, because its framing cannot be removed and its ground truth is known wrong. It is retained and cited as an observation; it is not counted as this arm.
5. **The arm is a named spike with a named consumer, registered per 0012, and it is unrun.** Its consumer is D4's negative-control fixture design and the review card's framing rules. **Until it runs, no document may claim the review gate is guarded against false approval**, and no false-positive rate from S-VAR or S-TYPED may be presented as evidence of overall gate calibration — those rates measure the other direction.
6. **The arm must be sized before it is run.** At the family's present n=5 with detection at ceiling, only a large effect is observable; a 16% drop is not. The registration carries its own sizing, or it reports a bound rather than a rate.

## Consequences

**Positive**
- Closes a named, one-sided gap: the gate acquires a control for the direction of bias that costs recall, on a design already measured as high-precision and low-recall.
- Uses a saturated metric for the only thing a saturated metric is good for — showing a drop — turning F26's ceiling from a dead end into an instrument.
- Makes the standing negative-control principle honest: with a known-defective, asserted-approved arm, a stable approval can finally be distinguished from a lenient judge.
- Gives the review card's framing rules an empirical basis rather than an intuition, before Phase 1 fixes what a card says about an item's history.

**Negative**
- Adds draws to the family whose binding constraint is wall clock rather than headroom, and S-TYPED's CLEAN reviews already run at a median 1,071 s with a slowest of 4,561 s.
- At n=5 the arm can only refute a large effect, so a null result will be weak and must be reported as a bound. Sizing it properly means growing the family, which means authoring more subtle-defect fixtures.
- Introduces a fixture whose construction asserts something false to the reviewer. That is the point of the manipulation, but it means these runs are not usable as detection measurements for any other purpose and must be excluded from pooled detection figures.
- The corpus now carries two framing-sensitive risks pointing in opposite directions — advisory findings surfaced at a gate are rationale-heavy input (0082) while this arm is positively-framed input — and no single card design is known to be safe against both.

**What this forbids**
- Forbids claiming the review gate is guarded against false approval, or against confirmation bias generally, while this arm is unrun.
- Forbids presenting a false-positive rate from S-VAR, S-TYPED or the CLEAN control as evidence of the gate's overall calibration; those measure over-rejection only.
- Forbids reading F29's 0/10 stable-approve control as evidence that the reviewer is not lenient — that reading is exactly the ambiguity this arm exists to resolve.
- Forbids counting the CLEAN set as the controlled confirmation-bias arm, or treating its false-positive denominator as sound: its ground truth is wrong for ≥13/23.
- Forbids reporting this arm as a single pooled error rate, or reading it on the verdict rather than on detection.
- Forbids pooling runs from the prior-approval framing into any other detection figure.
- Forbids treating a null result at the family's current size as evidence of no effect.

## Evidence and confidence

- [documented, preprint] *Measuring and Exploiting Confirmation Bias in LLM-Assisted Security Code Review* (~Mar 2026, arXiv 2603.18740): a controlled experiment over 250 CVE vulnerability–patch pairs finds that framing a change as "bug-free" before review reduces the vulnerability-detection rate by **16–93%**, strongly asymmetric, with false negatives rising sharply and false positives barely moving. Carried from evidence-augmentation §3 (verification) and §4 rec 13. **The augmentation's own instruction is carried with it: fetch the full paper for the per-condition breakdown before quoting a specific number.** The 16–93% band is a range across conditions, not an expected effect size for this corpus, and the per-condition breakdown was not read this pass.
- [documented, peer/preprint] Jin & Chen, arXiv 2603.00539: rationale-heavy prompts raise false rejection (GPT-4o FNR 26–36% → 58–88%), and the fix-guided verification filter recovers 54.8%→16.3% (HumanEval), 69.0%→28.9% (MBPP), 51.0%→24.0% (QuixBugs), and GPT-4o MBPP 88.7%→40.0%. **Provenance correction carried from evidence-augmentation §1 row 9: "Are LLMs Reliable Code Reviewers?" and the "fix-guided verification filter" are the same paper, not two.** Wherever the design cites both, it is citing one source, and the guard against over-rejection rests on one paper's replication, not two.
- [measured] F26 (E3-H, $36.18): S-SUBTLE detection 5/5 sighted and 5/5 blind across two harder tiers, 20/20 overall; the prediction was falsified twice and the line closed. Blind arms cost +18% median wall. This is the ceiling the new arm has headroom to fall from, and it is n=5 per arm.
- [measured] F27 (E0 variance, n=2–3 per item): detection **0 of 7 items flaky** over 21 draws; verdict **2 of 4 items flaky**. A "stable" label at this n tolerates a true flip rate up to ~30%. This is why the arm reads detection and not the verdict, and it is also why a small detection shift will be hard to distinguish from draw noise.
- [measured] F29 (S-VAR, 60 draws, 0 void, $176.96, 16.2 h): the negative control was `stable-approve` at **0 rejections in 10 draws**; 19/19 rejections on-target; class means Reproduced 0.70 > MutationSurvived 0.37 > NarrowedPin 0.10 > control 0.00. A reproduced regression was nonetheless approved **3 of 10** times.
- [measured] F31 (S-TYPED, $112.25): four adjudicated defects survived fully green suites of 1890/1890 and 1907/1907; adjudication 14/14 confirmed; verified false positives 0/23.
- [measured] F24: CLEAN's ground truth is wrong for **at least 13 of 23 items (57%)** — every item shipped and was approved, and the defects were present. This is the observational anchor for "presented as already approved" being the real-world framing, and it is also why CLEAN cannot serve as the controlled arm.
- [documented] Method finding: five metrics saturated across the programme, and every discriminating result came from cost, stability or mechanism rather than success rates. This is the argument for choosing a ceiling metric to measure a drop, and it cuts both ways — a ceiling metric will show nothing if the effect is small.
- **Corpus vs external, and which is acted on.** The corpus treats negative controls as sufficient because its control never fired (F29 branch 4, 0/10). The external pass names this as a one-sided guard and supplies the missing direction (§4 rec 13, "the mirror-image risk the design does not currently address"). **The external reading is acted on**: the corpus result is retained as true and reinterpreted as uninformative about approval bias, rather than being treated as reassurance.
- **What this evidence does not license.** It does not license any claim about the size of the confirmation-bias effect on *this* reviewer, this corpus, or this schema — the number is from a different task (security review of CVE patches), a different model set, and a different prompt regime, and its per-condition breakdown has not been read. It does not license claiming the arm will detect the effect: at n=5 with detection at ceiling, only a large drop is observable. It does not license any inference that the existing gate is currently biased toward approval — that is the hypothesis, not a finding. And per the research pass's own discipline, nothing here is confirmed by the absence of a contradicting source: nine agents finding nothing against a claim means no current source contradicts it.
- [documented] **Superseding note (2026-09-11):** ADR-0101 supersedes this record **in full** — the paper is "Measuring and Exploiting Contextual Bias in LLM-Assisted Security Code Review" (arXiv 2603.18740), the effect is 16–93 percentage points with false positives collapsing rather than "barely moving", the arm is renamed the prior-approval framing arm, and the symmetric arm is registered as S-SUBTLE-SYM rather than rejected. Every operative sentence is restated there.
