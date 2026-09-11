---
id: "0074"
title: Review gate rebuilt — reviewer proposes, machine confirms
status: accepted
date: 2026-09-08
legacy_id: D4 rev.4
source: references/2026-09-05-architecture-proposal.md §4.6 (row **D4 (rev. 4)** in §9); data and prior art in references/2026-09-07-design-recommendation.md §2
supersedes: ["0004"]
superseded_by: ["0096"]
related: ["0001", "0015", "0019", "0026"]
evidence: ["F29", "F31", "F27", "F24", "F25", "S-VAR", "S-TYPED"]
consumers: []
---

## Context and problem

0004 (D4) chose *findings, not verdicts* on the general inversion principle, explicitly "confirmed pending S-VAR", and named no mechanism. Both gating spikes have since run and they answer different halves of the question.

S-VAR measured the verdict directly: over 60 draws (0 void, $176.96, 16.2 h) **not one defect class is stable-reject**. Class means came out `reproduced` 0.70 > `mutation-survived` 0.37 > `narrowed-pin` 0.10 > negative control 0.00, and **a reproduced regression was approved on 3 of 10 draws** — the kill criterion that fires architecture-changing. Yet **19/19 rejections were on target**: the reviewer's *claims* are sound while its *threshold* is a weighted coin. F27 says the same shape from a different angle — detection 0/7 items flaky over 21 draws, verdict 2/4 items flaky.

S-TYPED removed the `pass` field entirely and measured what was left: detection held at ceiling (S-SILENT 23/23, S-SUBTLE 5/5), schema conformance was **52/52 = 100%**, `repro.command` filled 147/147, and the reviewer raised blocking-grade findings on 12 of 23 shipped, human-approved fixes — of which **14/14 of the newly raised were confirmed by mechanical re-execution**, at **0/23 verified false positives** under the lenient policy, for **+74% cost** ($112.25 against a $64.61 prose comparator).

So 0004's shape survives and 0004's silence does not. What a class *means*, when a finding may block, what happens when the evidence cannot be re-run, where a disputed finding goes, and how a false block is even counted are all unanswered by the original row, and each of them is now a place the gate can fail open or fail loud. This record supplies the mechanism and replaces 0004 whole.

## Decision drivers

- The verdict is unstable at the threshold and the findings are not (S-VAR, S-TYPED, F27). Any design that keeps a model-held threshold inherits the instability; any design that keeps the findings inherits none of it.
- **Neither gate is sufficient alone.** A reproduced regression was approved 3/10 (F29) and four adjudicated defects survived fully green suites of 1,890/1,890 and 1,907/1,907 (F31). The review gate is high-precision, low-recall: it may *add* findings, never *clear* an acceptance criterion.
- The false-positive denominator is corrupt. CLEAN's ground truth is wrong for **≥13 of 23 (57%)**, so every false-positive rate measured against historical approval is an upper bound only. Adjudication is the only sound denominator available.
- Execution-grade review is expensive — +74% over prose, median $3.03 and ~1,071 s (~18 min) per CLEAN review — so the rule that converts a finding into a block has to be worth paying for, and the tier has to be selectable per stage (0015/D15).
- The lenient relaxation is free: `narrowed-pin` advising rather than blocking gave 9/23 rather than 12/23 blocked, 0/23 verified false positives, and unchanged detection (F31). But the 7 adjudicated-true `narrowed-pin` findings still need a destination, and no advisory sink exists today.
- External corroboration that "green" is not "pinned": SWE-Gate finds **221 of 644 (34%)** functionally-passing agent repairs fail review constraints derived from real PR comments (evidence-augmentation §4 rec 12, [preprint] arXiv 2609.04167).

## Considered options

### Option — keep the verdict, block on the model's word

Retain a `pass` field and act on it. Rejected on measurement, not principle: S-VAR shows the verdict flips on 3 of 10 draws for an item carrying a reproduced regression, and F27 shows 2 of 4 items flaky on the verdict while 0 of 7 are flaky on detection. Blocking on that field imports a ~30 pp flip rate into the loop's control flow.

### Option — model-chosen evidence class

Keep the class (`reproduced` / `mutation-survived` / `narrowed-pin` / `advisory`) as a model-emitted label and drive the policy table from it. Rejected: S-TYPED found the reviewer's class *labels* wobble at boundaries while its claims do not, so the label is exactly the unstable part of an otherwise stable output. The label is retained for drift monitoring only.

### Option — more repeats before redesign

Average out the instability by sampling draws and taking a majority. Rejected here on the same grounds 0004 rejected it, now with a cost figure: the published coin-flip work puts high-flip items at ~15 trials for 90% consensus fidelity, which against a median $3.03 / 18 min execution-grade review is a ~15× multiplier on the most expensive stage in the loop to recover a signal the design does not need.

### Option — block on the class without re-execution

Compute the class in Rust from the repro record, then block on it directly. Rejected: a repro record can be well-formed and still not discriminate. Agentless's numbers are the reference case — 213 of 300 generated reproductions failed on the base commit and only 94 flipped under the true fix — so "a command ran and disagreed" is not yet evidence that the candidate is at fault.

## Decision

The review gate is rebuilt as **reviewer proposes, machine confirms**. Eight clauses, all normative:

1. **The card returns `Findings`, with no `pass` field.** Each item is `{ ac, claim, evidence (advisory label), rationale, files[], repro { command, observed, expected_when_met } }`. Every finding is an executable claim; a finding that cannot be stated as one is not a finding.
2. **The evidence class is computed in Rust from the repro record**, never taken from the model: `reproduced` iff a command ran and `observed ≠ expected_when_met`; `mutation-survived` iff a mutation was applied and the named suite stayed green; `narrowed-pin` iff only the discriminating assertion flips; otherwise `advisory`. The model's own label is stored beside the computed class for drift monitoring only.
3. **A finding blocks only after the deterministic gate re-executes it and it flips** — fails on the candidate and passes on the reference, or the mutant survives when re-applied. Re-execution is a precondition of blocking, not a confirmation step that may be skipped under load.
4. **Both-fail or cannot-run is `Unverifiable`, and `Unverifiable` is advisory.** A finding whose evidence the gate cannot separate does not block, in either direction.
5. **The policy table is the only place a class becomes a block, and it is a rigor parameter per stage.** `reproduced` and `mutation-survived` block; `narrowed-pin` blocks only under a strict weight; `advisory` and `Unverifiable` never block. S-VAR's class ordering sets the default order and nothing else.
6. **`Quarantined { reason, by, until }` is a first-class finding state.** A human override moves a finding to `Quarantined`; a quarantined finding stops blocking and is not re-raised on re-review until `until` elapses.
7. **Negative-control fixtures stay in the corpus** as the reviewer's own regression test, because a stable approval is equally consistent with a correct judge and a lenient one. The S-SUBTLE family additionally gains a **"presented as already approved" arm** (adopted here from evidence-augmentation §4 rec 13): the corpus currently probes only rationale-driven false *rejection*, and the mirror-image failure — positively framed input driving false *approval* — is the one this gate's advisory-by-default posture is most exposed to.
8. **Every block and every override is an event.** `false_block_rate` and per-card `pass^k` are telemetry, and **false-block is measured against adjudication, never against historical approval.**

The reviewer keeps `Bash`, in a disposable shallow clone at the reviewed SHA. An absent digest is `Err`.

## Consequences

**Positive**
- The blocking decision is a pure function of re-executed evidence, so it is deterministic, replayable and testable without a model in the loop — the property 0004 wanted and could not specify.
- Detection is not paid for: removing the verdict cost nothing measurable (23/23 and 5/5 held), and the typed contract conformed 52/52 with `repro.command` filled 147/147.
- The gate has a measured false-positive floor against a sound denominator: 0/23 verified false positives under the lenient policy, with 14/14 adjudicated findings confirmed.
- `Quarantined` gives a disputed finding a destination that is neither "block forever" nor "silently drop", and it is an event, so overrides are countable.

**Negative**
- Execution-grade review costs +74% ($112.25 vs $64.61) and runs at a median 1,071 s against the prose reviewer's 682 s. Rigor selects the tier (0015), and that selection is now a live cost lever rather than a formality.
- Advisory findings accumulate with nowhere to go. The 7 adjudicated-true `narrowed-pin` findings under the lenient policy are real defects that this policy does not block on, and no advisory sink is specified here.
- Clause 7's confirmation-bias arm is adopted on an external preprint that this pass did not fetch in full; the effect size it reports (detection down 16–93%, asymmetric toward false negatives) is not yet a redkiln measurement.
- Re-execution makes the gate's latency a function of the repro command, and the deterministic gate has no affected-area selection today — every run is the whole suite.

**What this forbids**
- Forbids any `pass`, `verdict` or `approve` field on the review card's output schema, and forbids the policy reading one if a card emits it anyway.
- Forbids taking the evidence class from the model. The model's label may be recorded; it may not be routed into the policy table.
- Forbids blocking on a finding the deterministic gate has not re-executed and seen flip.
- Forbids treating `Unverifiable` as a block, and forbids escalating it to one by re-running until it flips.
- Forbids any second location — card, prompt, rubric row, adapter — where a class becomes a block. The policy table is the only one.
- Forbids re-raising a `Quarantined` finding as a block before its `until`.
- Forbids computing `false_block_rate` against historical human approval, since that denominator is known wrong for ≥57% of the CLEAN corpus.
- Forbids removing the negative-control fixtures from the corpus, or shipping the fixture family without the "presented as already approved" arm.
- Forbids the review gate clearing an acceptance criterion. It may only add findings.

## Evidence and confidence

- [measured] F29 / S-VAR, n = 60 draws, 0 void, $176.96, 16.2 h: class means `reproduced` 0.70 > `mutation-survived` 0.37 > `narrowed-pin` 0.10 > control 0.00; **19/19 rejections on target**; a reproduced regression approved on **3 of 10** draws. The levels are n = 10 per class at roughly ±30 pp, so they license the **ordering** used as the policy table's default and nothing more — not the values, and not a threshold.
- [measured] F31 / S-TYPED, 51 valid runs, $112.25: detection 23/23 (S-SILENT) and 5/5 (S-SUBTLE) with the verdict removed; schema conformance 52/52; `repro.command` fill 147/147; CLEAN blocks 12/23 strict, 9/23 lenient; **14/14 adjudicated findings confirmed**; **0/23 verified false positives**; **+74%** cost against the prose comparator.
- [measured] F27, n = 2–3 per item: detection 0/7 items flaky over 21 draws, verdict 2/4 items flaky. A "stable" label at this n tolerates a true flip rate up to ~30%, so this corroborates the direction of the S-VAR result rather than tightening it.
- [inferred] The severity call behind clause 5's default — that `narrowed-pin` advises rather than blocks — is Ryan's judgement on the measured data, not a measured result. F31 shows the relaxation is free on detection and on verified false positives; it does not show the 7 advised findings are unimportant.
- [documented] **Two of the citations behind this rebuild are one paper, not two.** "Are LLMs Reliable Code Reviewers?" and the "Jin & Chen fix-guided verification filter" are both arXiv 2603.00539 — same authors, same paper, the filter in §7.2 / Table 4 (evidence-augmentation §1 row 9, §4 rec 11). Every individual number re-verifies exactly, but **the rebuild rests on fewer independent replications than the proposal implied**, and the corpus should stop presenting them as two. Where the corpus and the research pass disagree here, the research pass is acted on: the citation is merged, and clause 3's rationale is carried by Agentless's reproduction numbers (213/300 failed on base, 94 flipped) plus SWE-Gate rather than by a double-counted filter result.
- [documented] SWE-Gate (evidence-augmentation §3 Verification, §4 rec 12): 303 repair instances across 75 Python repos; **221 of 644 (34%)** functionally-passing repairs fail review constraints. Fresher and more directly on-axis than anything previously cited for "green is not pinned", and it separates detection from verdict the same way this decision does. It is a benchmark constraint-failure rate, **not** a live false-block or override rate, and must not be quoted as one.
- [documented] Clause 6 has **no external prior art beyond GitLab flaky-quarantine and Cloudflare "won't fix"**; the research pass looked and found none (evidence-augmentation §5). `Quarantined` is original process design and should be stated as such.
- [documented] Clause 8's only public comparator is Cloudflare's break-glass rate, **288 of 48,095 merge requests = 0.6%**; the research pass found no other published false-block comparator (§5). Where cost is anchored on the same source, note that $1.68 is the full tier's **average** and $1.47 its median, against a pooled median of $0.98 (§1 row 13) — the corpus's pairing of $0.98 and $1.68 invites reading the latter as a median, and it is not.
- This record does **not** license: treating the policy table's numeric class means as calibrated probabilities; claiming a measured false-positive rate against any denominator other than adjudication; claiming the confirmation-bias arm of clause 7 has been run; or claiming that nine research agents finding nothing against a corpus claim confirms it — the research pass is explicit that this means only "no current source contradicts it".
- Superseded record: 0004 (D4) is carried at `superseded` pointing here. Its normative content — findings not verdicts, Rust computes the block — survives inside clauses 1, 2 and 5, but it is replaced whole because every operative term in it is now defined differently and its "pending S-VAR" provisionality no longer describes the state of the evidence.
