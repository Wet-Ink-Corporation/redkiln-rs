---
id: "0019"
title: Per-AC discriminating test is the norm
status: accepted
date: 2026-09-07
legacy_id: D19
source: references/2026-09-05-architecture-proposal.md §4.6
supersedes: []
superseded_by: []
related: []
evidence: ["F24", "F25"]
consumers: []
---

## Context and problem

The corpus-adjudication findings that reshape the review gate show that "reviewed and green" has been standing in for evidence that a criterion is actually pinned, and it is not. §4.6 item 8 records that the per-AC mutation-evidence columns "exist in 0/156 historical reports"; §4.6's S-TYPED paragraph records the reviewer surfacing blocking-grade findings on 12 of 23 shipped, human-approved fixes, with 14/14 of the newly raised findings confirmed by mechanical re-execution. The §1 preamble states the underlying root cause directly: an AC is a claim about one assertion, and root-cause analysis (F24/F25) traces the ≥12-of-23 (52%, rev. 4) rate of ACs with no pinning test back to this gap between "green" and "pinned."

## Decision drivers

- 0/156 historical reports carry per-AC mutation-evidence columns (§4.6 item 8) — the practice of demonstrating that a test actually discriminates the fix from its absence has never been the recorded norm.
- CLEAN's ground truth is wrong for ≥52% of the corpus (S-TYPED, §4.6). That "reviewed and green" was the evidence standard which let that gap go undetected is this ADR's reading of §1's root-cause note ("Red→Green evidenced per test, an AC is a claim about one assertion"), not a separate source claim.
- Google's review-start median of one live mutant per changelist (ICSE 2021, cited in §4.6 item 8) is offered as an existing precedent for treating mutation evidence as the normal unit of review, not an exceptional extra step.
- The review gate's own machinery already computes evidence classes (`reproduced`, `mutation-survived`, `narrowed-pin`, `advisory`) from a repro record in Rust (§4.6 item 2), giving the reviewer's role — producing that discriminating check — a concrete mechanism to plug into.

## Considered options

### Option — test-green as sufficient

Keep treating a green test suite at review time as sufficient evidence that an AC is satisfied and pinned, with no separate requirement for a discriminating (mutation-style) check. This is the status quo the row records losing: it is the practice that produced 0/156 historical reports with mutation-evidence columns and let ≥12 of 23 shipped fixes carry an AC no test actually pins, so it was rejected on the measured gap it left undetected.

## Decision

"Reviewed and green" is not evidence that a criterion is pinned. The per-AC discriminating test — TDD's normal mutation evidence — is the product's normal evidence standard, not an exceptional one (§4.6 item 8: "reviewed and green" is not evidence that a criterion is pinned). On a fix, the reviewer's job is to produce the discriminating check the author did not: this is stated as the reviewer's role, not an optional enhancement to review.

## Consequences

**Positive:**
- Makes the gap between "green" and "pinned" an explicit, checked property of the review stage rather than an invisible assumption, closing the failure mode that let ≥52% of shipped fixes ship without a pinning test.
- Gives the review gate's evidence-class machinery (§4.6 item 2: `reproduced`, `mutation-survived`, `narrowed-pin`, `advisory`, computed in Rust from the repro record) a concrete job to do on every fix, rather than only on fixes where the author already supplied a discriminating test.

**Negative:**
- Shifts real work onto the reviewer stage that the author's own report previously implied was already done — the reviewer must now actively construct or verify a mutation/discriminating check rather than confirming the suite is green, work that in §4.6 belongs to the execution-grade review tier, whose cost is stated there as median $3.03 and ~18 min per CLEAN review, +40% to +101% over the prose reviewer (+71% on CLEAN). Those figures are §4.6's price for that tier and the basis of D15's tier selection; reading them as the price of *this* decision is an inference, since the source attaches no cost figure to D19 itself.

**What this forbids:**
- It forbids treating a green test suite, by itself, as evidence that an AC's criterion is pinned — a report or review that cites suite-green as its sole justification for a criterion no longer counts as satisfying that criterion.
- It forbids the reviewer from passing a fix without having produced (or confirmed) a discriminating check on any AC the author's own report leaves unpinned — this is stated as "the reviewer's job," not a discretionary judgment call.

## Evidence and confidence

- [measured] 0/156 historical reports carry the discriminating-mutation columns (§4.6 item 8); n=156, the full cited corpus.
- [measured] ≥12 of 23 shipped, human-approved fixes (rev. 4: ≥12/23, i.e. 52%) carry an AC no test pins, with 14/14 of the newly raised findings confirmed by mechanical re-execution (S-TYPED, cited via §4.6 and echoed at the document's §1 preamble as F24/F25's root-cause finding).
- [inferred] The decision's normative claim — that per-AC discriminating tests should be the norm going forward — is a design response to the measured gap above; the measurement establishes that the old norm ("reviewed and green") was insufficient, it does not itself measure the new norm's effectiveness, since the new norm has not yet been run.
- This decision does not license a claim that the review gate's evidence-class computation (§4.6 item 2) has been validated in production; that mechanism is described as the how, this decision (D19) is the why, and the corpus gives no separate spike result for D19 itself beyond the S-TYPED/F24/F25 findings that motivated it.
