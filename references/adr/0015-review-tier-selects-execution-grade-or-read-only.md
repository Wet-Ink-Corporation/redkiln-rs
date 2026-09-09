---
id: "0015"
title: Review tier selects execution-grade or read-only, not a uniform policy
status: accepted
date: 2026-09-07
legacy_id: D15
source: references/2026-09-05-architecture-proposal.md §4.6
supersedes: []
superseded_by: ["0020"]
related: []
evidence: ["S-VAR", "S-TYPED", "CLEAN"]
consumers: []
---

## Context and problem

Execution-grade review (the reviewer keeps `Bash`, in a disposable shallow clone at the reviewed SHA, and its own evidence is re-executed before a block) measured +40% to +101% cost over a prose-only reviewer depending on condition (+71% on CLEAN; median $3.03, ~18 min per CLEAN review, against a read-only full tier at $1.68 and Anthropic's own verification-heavy review at $15–25). At the same time S-TYPED found the reviewer surfaces blocking-grade findings on 12 of 23 shipped, human-approved fixes, with 14/14 of the new ones confirmed by mechanical re-execution — CLEAN's ground truth is wrong for ≥52% of the corpus. A single review policy applied everywhere either overpays for stages where the cheaper tier is adequate or underpays where the errors concentrate. The fix lane (`bug-standard`) is named in the source as the place the 52% lives.

## Decision drivers

- Execution-grade review costs materially more than prose review (+40%–101%, condition-dependent), so applying it uniformly is not free.
- S-TYPED shows shipped, human-approved fixes still carry unconfirmed defects at a rate high enough (≥52% of ground truth wrong) to justify paying for execution-grade review where it matters most.
- No data yet exists on the recall loss of a cheap-first cascade (cheap tier first, escalate to execution-grade on failure).

## Considered options

### Option — execution-grade everywhere
Every review, regardless of card or stage, would run the +40%–101%-cost execution-grade tier. Rejected: pays the full mutation/re-execution cost even where the read-only tier is adequate, with no measured justification for the added spend on those stages.

### Option — read-only default
Every review would default to the cheaper, advisory-only tier unless explicitly upgraded. Rejected: the fix lane (`bug-standard`) is exactly where S-TYPED's 52%-wrong-ground-truth finding applies, and a read-only default would not catch what execution-grade catches there.

## Decision

"Review stage weight selects the tier — `0.2` read-only advisory, `1.0` execution-grade — and `bug-standard` is execution-grade because the fix lane is where the 52% lives." No cheap-first cascade is adopted until its recall loss is measured.

## Consequences

**Positive**
- Review cost is matched to where the source data shows errors concentrate (the fix lane), rather than spent or withheld uniformly.
- The cost delta (+40%–101%, CLEAN +71%, $3.03 median) is stated explicitly rather than left implicit, so the trade is auditable.

**Negative**
- Stages assigned the `1.0` weight pay the full execution-grade cost premium, and the source states that premium only as a condition-dependent range (+40%–101%, with +71% named for CLEAN); it does not break the range down per stage, so a `1.0` stage cannot be costed more precisely than that range.
- No cascade exists to recover part of that cost cheaply, because the recall loss of doing so is unmeasured.

**What this forbids**
- Forbids applying execution-grade review to every stage regardless of weight (rejected alternative).
- Forbids defaulting every review to the read-only tier, including the fix lane (rejected alternative).
- Forbids shipping a cheap-first cascade before its recall loss has been measured.

## Evidence and confidence

- [measured] Execution-grade review cost premium: +40% to +101% over prose review, +71% on CLEAN specifically; median $3.03, ~18 min per CLEAN review (comparators: Cloudflare read-only full tier $1.68, Anthropic verification-heavy review $15–25). The source does not state the sample size behind the range, nor which conditions other than CLEAN it spans.
- [measured] S-TYPED: reviewer surfaces blocking-grade findings on 12/23 shipped, human-approved fixes; 14/14 of the new ones confirmed by mechanical re-execution — CLEAN ground truth wrong for ≥52% of the corpus.
- [documented] "No cheap-first cascade until its recall loss is measured" — this is stated as an open gap in the source, not a result; the ADR does not license treating a cascade as safe.
- This record does not license extending the `0.2`/`1.0` split to any stage not named in the source row. The numeric weights themselves are superseded by ADR 0020's deontic enum, which maps `0.2 → optional` and `1.0 → mandatory`; this ADR's normative content is the *tiering rule* (weight selects tier, `bug-standard` is execution-grade), which survives that renumbering.
