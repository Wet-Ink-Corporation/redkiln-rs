---
id: "0082"
title: The Advisory sink is a first-class destination, not a discard
status: accepted
date: 2026-09-08
legacy_id: none
source: spikes/F31-DRAFT.md §"Two things this does not settle"; spikes/F29-DRAFT.md §branch 4; references/2026-09-07-design-recommendation.md §2 rules 3–4; references/2026-09-08-evidence-augmentation.md §5
supersedes: []
superseded_by: []
related: ["0004", "0015", "0020", "0063"]
evidence: ["F31", "F29", "F24", "S-TYPED", "S-VAR"]
consumers: []
---

## Context and problem

The review gate's policy table is a stage-weight parameter: `reproduced` and `mutation-survived` block, `narrowed-pin` blocks under a strict weight and advises under a lenient one, `advisory` never blocks (design-recommendation §2 rule 3, on D4's rev. 4 rebuild). S-TYPED measured both weights on the same 23-item CLEAN set: strict blocks 12/23, lenient 9/23, detection unchanged at S-SILENT 23/23 and S-SUBTLE 5/5, schema conformance 52/52, and adjudication confirmed 14/14 findings true with **0/23 verified false positives** (F31). The lenient default was adopted on that evidence — the relaxation costs no detection and is therefore free.

The relaxation leaves a hole. **Seven `narrowed-pin` findings, every one of them adjudicated TRUE**, move from blocking to non-blocking and the design has nowhere to put them (F31: `issue-130` AC-005, `issue-81` AC-002, `issue-82` AC-003, `issue-83` AC-001, `fix-doctor-git-config-repair` AC-002 and AC-004, `memoize-repo-root` AC-003). F29's branch 4 tells us what the destination is *not* for: the negative control rejected 0 of 10 draws, so no `Advisory` sink is needed to absorb noise — the reviewer is not producing noise at this rate. The sink is needed to **carry real, adjudicated-true signal that does not block**. Discarding those findings silently throws away the most expensive evidence the programme produced: S-TYPED cost $112.25, +74% over its prose comparator, to generate them, on a corpus whose own ground truth F24 showed to be wrong for at least 13 of 23 items.

`Quarantined` does not fill the hole. It is a **human override of a finding the policy computed as blocking** — `{ reason, by, until }`, suppressing a block that would otherwise stand (design-recommendation §2 rule 4, after GitLab flaky-quarantine and Cloudflare "won't fix"). An advisory finding was never blocking, so there is nothing for a Quarantine to suppress. Nothing in the corpus says where an advisory finding is written, how it reaches a human, or when it stops being live.

## Decision drivers

- The seven displaced findings are adjudicated true, not suspected true: F31's adjudication is 14/14 confirmed, at zero model cost, with every blocking-grade finding carrying a re-runnable `repro.command` (147/147 fill).
- F29's control rejected 0/10, so noise suppression is not the sink's job and cannot be used to justify a lossy destination.
- Neither gate is sufficient alone: four adjudicated defects survived fully green suites of 1890/1890 and 1907/1907 (F31), and a reproduced regression was approved in 3 of 10 draws (F29). A non-blocking finding is often the only surviving trace of a real defect.
- `Quarantined` already occupies the "a human decided this should not block" slot and carries an actor and an expiry; overloading it would conflate a machine-computed disposition with a human override.
- No richer prior art exists to copy. GitLab flaky-quarantine and Cloudflare "won't fix" are the only comparators found, and the external research pass records this as a real gap rather than a short-search artefact (evidence-augmentation §5).

## Considered options

### Option — discard non-blocking findings

Compute the class, apply the policy, emit the block, drop everything that did not block. Rejected: it deletes seven adjudicated-true findings per 23 items, produced at $112.25 and +74% cost, and it makes the lenient default *not* free after all — free in detection, but paid for in permanently lost evidence. It also makes the strict-vs-lenient choice irreversible in practice, because the record needed to revisit it would no longer exist.

### Option — fold Advisory into `Quarantined`

Reuse `Quarantined { reason, by, until }` as the destination for anything that does not block. Rejected: `Quarantined` asserts that a *human* overrode a *blocking* finding, and its `until` is an override deadline. An advisory finding has no human actor, no block to suppress and no natural expiry, so every field would be filled with a placeholder and the override telemetry D4's rev. 4 calls for would stop meaning anything.

### Option — restore the strict weight so `narrowed-pin` blocks again

Make the problem disappear by keeping the seven findings blocking. Rejected on measurement: strict blocks 12/23 against lenient's 9/23 for **zero additional detection** (23/23 and 5/5 in both), and F31 records that no detection result ever depended on `narrowed-pin` blocking. This buys three extra blocks per 23 items and no recall, on a corpus whose ground truth is known wrong for ≥13/23 so that even the "false positive" denominator is an upper bound.

### Option — emit advisory findings to telemetry only

Write them to the event stream for later analysis and show nothing at the gate. Rejected: it satisfies the audit requirement and fails the design one. The human at the next gate is the consumer who can act, and a finding that never reaches the decision point is functionally discarded — with the added cost of looking as though it was retained.

## Decision

**An advisory finding is retained, surfaced and retired explicitly. It is never discarded and never silently aged out.**

1. **Written, not dropped.** Every finding the policy table computes as non-blocking — `advisory` by class, `narrowed-pin` under a lenient stage weight, or anything downgraded to advisory because re-execution returned `Unverifiable` — is written to the item's durable finding record in the same typed `Findings` shape as a blocking finding, carrying its `repro` record, its rationale and the advisory `evidence` label unaltered. The only difference between a blocking and an advisory finding on disk is the computed disposition; no field is stripped.

2. **Surfaced at the next gate without blocking.** Advisory findings are presented at the next human gate as typed attachments bound to the gate's question (0063), listed under their own heading and excluded from the block computation. They are visible to the answerer and inert to the policy. A gate answer is never conditioned on an advisory finding, and the presence of advisory findings alone never opens a gate.

3. **Retired only by re-review or by an explicit human act.** An advisory finding leaves the live set when either (a) a re-review at a later tree no longer raises it, or (b) a human dismisses it with a recorded reason, written as a durable event naming the actor. Both paths emit an event. Age, run count, gate passage and stage advancement retire nothing.

4. **Advisory and `Quarantined` are disjoint.** `Quarantined { reason, by, until }` applies **only** to a finding the policy computed as blocking, and its effect is to suppress that block. Applying a Quarantine to an advisory finding is an error, not a no-op — there is no block to suppress and the override telemetry would record a suppression that never happened. A finding is in exactly one of `blocking`, `quarantined` or `advisory` at any tree.

5. **The disposition is computed, never asserted.** Advisory status is derived in Rust from the class and the stage weight, exactly as a block is (D4, 0004). The model may not label its own finding advisory to avoid a block, and code may not promote an advisory finding to blocking without re-executing the flip criterion.

## Consequences

**Positive**
- The seven adjudicated-true `narrowed-pin` findings per 23 items survive the lenient default with their re-runnable commands intact, so the strict/lenient choice stays revisitable against real retained data rather than a re-run.
- Makes "the policy relaxation is free" true as stated: free in detection *and* free in evidence, which it is not if the displaced findings are dropped.
- Gives block/override telemetry a clean denominator, because a suppression event now unambiguously means a human overrode a real block.
- A finding that the deterministic gate could not re-execute (`Unverifiable`) reaches a human instead of vanishing between two mechanisms neither of which owns it.

**Negative**
- The live advisory set grows monotonically until something retires it, and nothing retires it automatically. On a long-lived item this is a real accumulation cost, and the gate surface will need a presentation rule this record does not specify.
- Every gate answerer now sees more material than the decision strictly requires, which is precisely the "rationale-heavy input" shape Jin & Chen associate with raised false rejection (arXiv 2603.00539). That risk is unmeasured for this surface; see 0088 for the mirror-image risk on the approval side.
- The design is original. No surveyed system carries a non-blocking finding forward as a first-class state with an explicit retirement path, so there is no implementation to copy and no field failure mode to anticipate.

**What this forbids**
- Forbids deleting, truncating or downgrading to prose any finding the policy computed as non-blocking.
- Forbids expiring an advisory finding by age, run count, gate passage or stage advancement — retirement requires a re-review at a later tree or a recorded human dismissal.
- Forbids applying `Quarantined` to a finding that was not computed as blocking, and forbids any state in which a finding is both quarantined and advisory.
- Forbids letting an advisory finding participate in the block computation, and forbids opening a gate solely because advisory findings exist.
- Forbids the model asserting `advisory` as a way of avoiding a block, and forbids promoting an advisory finding to blocking without re-executing its flip criterion.
- Forbids citing GitLab flaky-quarantine or Cloudflare "won't fix" as precedent for the Advisory sink; they are precedent for `Quarantined` only.

## Evidence and confidence

- [measured] F31 (S-TYPED, 51 valid dispatches, $112.25): CLEAN blocks strict 12/23, lenient 9/23; detection S-SILENT 23/23 and S-SUBTLE 5/5 under both; schema conformance 52/52 = 100%; `repro.command` fill 147/147 on blocking-grade findings; adjudication 14/14 confirmed; verified false positives 0/23. Seven `narrowed-pin` findings are individually listed as CONFIRMED and are the population this record routes.
- [measured] F29 (S-VAR, 60 draws, 0 void, $176.96, 16.2 h): the negative control rejected **0 of 10** draws, so branch 4 did not fire. This is what licenses the claim that the sink's purpose is carrying signal rather than absorbing noise. It is n=10 on one control item; a 0/10 result has a wide upper bound and does not establish a low noise rate in general.
- [measured] F31 and F29 jointly: four adjudicated defects survived fully green suites (1890/1890, 1907/1907) and a reproduced regression was approved 3/10. This is the reason a non-blocking finding is worth retaining, not merely tidy.
- [documented] The `Quarantined { reason, by, until }` semantics and its GitLab/Cloudflare precedent are design-recommendation §2 rule 4. This record does not change `Quarantined`; it draws the boundary around it.
- [documented] evidence-augmentation §5: "Are there published quarantine/override patterns for disputed findings beyond GitLab flaky-quarantine and Cloudflare 'won't fix'? — **No.** D4's `Quarantined` state has no richer prior art to draw on", recorded as a real gap rather than a search artefact. The Advisory sink is therefore **original design**, and no part of it should be presented as adopted practice.
- **Corpus vs external, and which is acted on.** The corpus (design-recommendation §2 rule 4) cites GitLab and Cloudflare as if they were a pattern to follow; the external pass finds they are the *entire* published field and that nothing covers a non-blocking-but-true finding. The external reading is acted on: the record claims no precedent and states the design as original.
- [inferred] The retirement rule (re-review at a later tree, or recorded human dismissal) is reasoned from the requirement that a finding's liveness track the tree it was raised against, by analogy with D17's tree-hash binding of a gate answer. It is not measured and no spike has tested it.
- **What this evidence does not license.** It does not license any claim that the Advisory sink improves detection, reduces cost, or is used correctly by a human — none of that has been measured. The 0/23 false-positive figure is an upper bound: F24 established CLEAN's ground truth is wrong for at least 13 of 23 items (57%), so every rate measured against it is provisional. The severity call itself — that `narrowed-pin` advises rather than blocks — is Ryan's judgement recorded in `results/s-typed-adjudication.jsonl`, not a measured optimum, and F31's own class labels disagreed with F24's on two items (`issue-63`, `issue-74`), so the class that routes a finding into this sink is itself known to wobble at boundaries. All figures are one corpus, one operator, one model family.
