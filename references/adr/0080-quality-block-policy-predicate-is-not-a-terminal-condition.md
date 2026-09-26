---
id: "0080"
title: The encapsulated quality block's policy predicate is not a usable terminal condition, and the quality shorthand's default stays unset
status: accepted
date: 2026-09-08
legacy_id: none
source: spikes/F37-DRAFT.md §2–§5, §9 (P7 RESULT); spikes/METHOD-P.md §4; references/2026-09-07-activity-graph-schema.md §9 (post-P7 shorthands)
supersedes: []
superseded_by: ["0111", "0168"]
related: ["0026", "0028", "0004", "0046", "0065", "0081"]
evidence: ["F37", "F23", "F26", "F31", "F27"]
consumers: []
---

## Context and problem

PC6 (ADR 0028) gave each activity graph ownership of the quality of its own output: self-review, adversarial reviewer,
retry and HITL escalation live *inside* the activity, and the stage sees only `Present(X, evidence)`. The concrete shape
of that ownership is the `adversary → policy → gate` block, and the activity-graph schema deferred a shorthand for it —
`quality = { adversary = <role>, policy = <policy-id>, gate = <rigor> }` — explicitly "to add **after P7**", with P7
supplying the default internal rigor at which the block is present. P7 has now run.

[F37] 61 graph runs of `spikes/p9/catalog/root-cause.toml` v4 (`306b58ba7504`) over the E3 rig, 297 records, **0 void**,
$131.62 of a $200 cap. Arms `optional` and `recommended` at 10 items × 3 repeats, plus one `mandatory` run. Investigator
and adversary both Opus at effort `xhigh`. What it measured:

- **Cost.** A run with the block costs **$2.69** against **$1.02** without it — **2.64×** — driven by one extra Opus
  dispatch at $1.68 and 676 s.
- **Benefit.** The block changed the root cause on **1 item in 10**: seven items are solved 3/3 by both arms, two are
  missed 0/3 by both, and `issue-81` separates 0/3 against 1/3. Mean citation precision is 0.63 (`optional`) against
  0.64 (`recommended`) — the block does not narrow the file set either.
- **The verdict channel carries no information.** The catalog's policy check is
  `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`. It evaluated **30 pass / 0 false** in the `optional`
  arm (where no adversary runs, so it is vacuously true) and **0 pass / 30 false** in `recommended`, plus 3 false in the
  single `mandatory` run. **33 of 33 adversary dispatches produced at least one finding in a blocking evidence class and
  none produced zero.** The verdict is therefore fully predicted by whether the arm has an adversary step at all: it is a
  constant function of the arm, not a function of the hypothesis under review.
- **In the `recommended` arm it is additionally inert.** Loop-back is a `mandatory`-only behaviour on this graph, so
  30 `false` verdicts produced **0** loop-backs and the graph proceeded to `emit` regardless. Whatever the block cost in
  that arm, it bought a verdict nothing consumed.
- **The deviation channel was offered on 71 investigator dispatches and used 0 times.** METHOD-P §4.4's three deviation
  thresholds are computed over emitted requests and have an empty denominator.

The predicate is the sharp part. "No finding exists in a blocking evidence class" is satisfiable only by a reviewer that
finds nothing — and F23 (23/23 detection, both arms), F26 (20/20 across two harder tiers, sighted and blind) and F31
(23/23 S-SILENT, 5/5 S-SUBTLE, 14/14 adjudicated confirmed, 0/23 verified false positives) establish beyond argument that
this reviewer is not that. A terminal condition that the instrument is known never to satisfy is not a terminal
condition.

Separately, P7's primary criterion — "`recommended` catches no more than `optional`, interval overlap on every item" —
fired on 10/10 items. It could not have done otherwise. At n = 3 a 3/3 cell has a Wilson interval of [0.44, 1.00] and a
0/3 cell has [0.00, 0.56]; the only way two such cells fail to overlap is 3/3 against 0/3, the maximum possible
separation. The criterion as written can detect only an effect that is total.

## Decision drivers

- A predicate whose truth value is determined by the arm rather than by the artifact under review is not evidence, and
  PC6's whole encapsulation claim rests on the block producing something the activity can act on.
- The measured cost of the block (2.64×, one extra Opus dispatch) is real and recurring, so leaving its terminal
  condition ill-formed spends money for a verdict nothing can consume.
- D4 (ADR 0004) already establishes the shape of the answer at the item level — findings are data and a policy component
  in code computes blocking. The activity-internal policy step must be held to the same standard, and currently is not.
- Setting a shorthand's default is a decision with a long tail: it changes what every future activity gets by default,
  and reversing it means editing shipped activities and bumping versions (ADR 0072).
- The programme's own method finding applies here: every discriminating result it produced came from cost, stability or
  mechanism, never from success rates. §3's constant-function result is a mechanism finding and does not depend on sample
  size; §1's catch-rate table is a success rate at n = 3 and does.
- The adversary's own output is a positive result that must not be discarded along with the verdict channel that failed.

## Decision

**1. Retain the `adversary → policy` block.** PC6's encapsulation is not withdrawn and this record does not supersede
ADR 0028. The block's evidence-producing half works: **188 findings over 33 dispatches**, mean 5.7 per dispatch, of which
**147 (78%)** are in the two classes that require the reviewer to have actually run something — 118 `reproduced` and
29 `mutation-survived`, against 22 `advisory` and 19 `narrowed-pin`. It produced that at that rate **while the `confirm`
tool step was broken** and handing it full-suite output instead of a discriminating test (F37 §8.1), which makes it
self-evidencing: the block's evidence does not depend on the graph's tool step working. The block works. Its verdict
channel does not.

**2. The predicate `findings.none(evidence in [Reproduced, MutationSurvived])` is forbidden as a quality block's terminal
condition.** A policy step's terminal condition must be decidable over something the reviewer varies. Concretely, it must
be one of:

- a **threshold** over severity or confidence — the terminal condition is "no finding at or above threshold T survives
  re-execution", not "no finding exists"; or
- a **different quantity entirely** — a bounded loop-back count, a re-execution outcome, or a delta against the previous
  iteration's findings — chosen so that a reviewer doing its job can terminate the block.

Lint rejects a policy predicate whose satisfaction requires an empty finding set.

**3. The threshold value itself is not set here, and is not blocked on P7.** It is blocked on the severity vocabulary,
which is unsettled by a different route: F24's E3 adjudication settled the *facts* of all six rejections (6/6 confirmed)
but left **severity unadjudicated**, and F31's lenient policy call — that `narrowed-pin` advises rather than blocks — is
recorded as a human judgement, with the corollary that the seven adjudicated-true `narrowed-pin` findings still have no
Advisory sink to go to. Pick the threshold when severity has a settled vocabulary and a destination for advisory
findings; not before.

**4. The `quality` shorthand ships with NO default.** `quality = { adversary, policy, gate }` is authorable and expands
as specified, but there is **no implied rigor at which the block is present**. Every activity that wants the block writes
it. This is decided explicitly and on the record, because P7's primary criterion fired on a test that could not have come
out differently: deciding the default from that result would be deciding it on an unfalsifiable comparison. What P7
established is the criterion's **power**, not its verdict.

**5. What would settle the default: more draws per cell, roughly ten — not more items.** At n = 10 per cell a 1-in-10
effect is resolvable; at n = 3 it is not, and adding items multiplies wide intervals rather than narrowing any of them. A
re-run that adds items instead of draws does not lift this.

**6. The `recommended`-arm inertness is recorded as measured confirmation of an existing rule, not as a new one.** A
policy check active at a rigor where its `on_false` target is not active is exactly what lint R4b (ADR 0065) forbids.
The P7 catalog exhibited it 30 times in a live run; R4b's origin was previously a single case found by reading, with no
measured instance. It has one now.

## Considered options

### Option — keep the predicate and read the 33/33 falses as a real signal
Treat "the reviewer always finds something in a blocking class" as evidence that the investigator's hypotheses are
routinely defective, and keep the block looping until findings stop. Rejected: the verdict is fully predicted by the arm,
not by the hypothesis, and 22 of the 30 `recommended` runs had a **correct** root cause *and* a false policy verdict
simultaneously. Under this reading the block would loop indefinitely on correct answers.

### Option — set the quality default to `recommended` on P7's catch-rate result
Adopt "block present at `recommended` and above" as the shorthand's default, on the basis that P7's first criterion fired
against `optional`. Rejected: the criterion fired on a test with almost no power. A 3/3-versus-0/3 separation is the only
outcome that could have avoided firing, so the result is uninformative about the default and adopting it would launder an
unfalsifiable comparison into a shipped default on every future activity.

### Option — remove the block entirely on the cost result
Drop `adversary → policy` from the catalog on the 2.64× cost and the 1-in-10 benefit. Rejected on the adversary's own
output: 188 findings, 78% in classes requiring execution, produced while the tool step was broken. Removing the block
discards a working, self-evidencing evidence producer in order to fix a predicate. Fix the predicate.

### Option — defer the whole question to a re-run before deciding anything
Decide nothing until P7 is re-run at n = 10 per cell. Rejected in part: the constant-function result (§3) does not depend
on sample size and is not a power question, so the predicate can and should be decided now. Only the *default* is
deferred, and it is deferred explicitly rather than left silently unset.

## Consequences

**Positive**
- A measured, mechanism-level defect in the block's terminal condition is closed by a lint rule rather than left to
  authoring discipline.
- The block's genuinely positive result — an evidence producer that works even when the graph around it does not — is
  recorded as a keeper, so fixing the predicate does not cost the catalog its adversary.
- The shorthand's default being explicitly unset is auditable. A future reader can see that it was withheld and why,
  rather than inferring a default from the absence of a decision.
- The re-run that would settle the default is specified precisely enough to cost: roughly ten draws per cell on the same
  ten items, not a larger item set.
- Lint R4b (ADR 0065) acquires its first measured instance, 30 occurrences in a live run.

**Negative**
- Every activity wanting the quality block must now write it explicitly. Authoring cost rises, and coverage becomes an
  authoring outcome rather than a default — F34 measured the block present in 42 of 90 activities under free authoring,
  so the spread is wide.
- The threshold value is deferred to a severity adjudication that is itself outstanding, so the replacement predicate's
  *shape* is decided while its parameter is not. An author writing a quality block today must choose a threshold without
  a settled vocabulary behind it.
- The deviation channel's value is left unanswered rather than resolved: 0 requests in 71 offers is a result about this
  graph, not about the channel, and answering it needs a graph the investigator has reason to object to.
- Retaining the block retains its 2.64× cost on every activity that declares it, for a benefit this run could not
  resolve.

**What this forbids**
- Forbids `findings.none(evidence in [...])`, or any policy predicate satisfiable only by an empty finding set, as a
  quality block's terminal condition — in the catalog, in a technology pack, or in a shorthand expansion.
- Forbids the `quality` shorthand carrying an implied default rigor, and forbids any pack or lint rule inserting the
  block into an activity that did not write it.
- Forbids setting that default from P7's data, or from any comparison at n = 3 per cell where the Wilson intervals
  overlap by construction.
- Forbids reading the 33/33 false verdicts as a finding about this reviewer's quality. It is a finding about the
  predicate; F23, F26 and F31 measure the reviewer.
- Forbids citing P7 as evidence that the block does not catch more, or that it does. The run establishes the criterion's
  power; it does not report a verdict.
- Forbids resolving the deviation channel's fate from P7's empty denominator in either direction — neither "keep it, it
  costs nothing" nor "remove it, nobody used it" is licensed by 0 requests in 71 offers on one graph.
- Forbids treating the `mandatory` arm's cost figure as settled: F37 reports 3.66× and the decisions log reports 4.4×,
  unreconciled, at n = 1, on a configuration the engine does not select (see ADR 0081).

## Evidence and confidence

- [measured] F37: 61 graph runs, 297 records, 0 void, $131.62; catalog `root-cause.toml` v4 `306b58ba7504`; corpus pin
  `47faec9f`; answer key from `spikes/results/s03-fixlane-manifest.jsonl`. Cost $2.69 (`recommended`) against $1.02
  (`optional`) per run = 2.64×; adversary dispatch $1.68 / 676 s. Root-cause correctness is mechanical (the
  investigator's `root_cause.files` intersect the answer key), so no judgement enters the primary number.
- [measured] Policy evaluations: `optional` 30 pass / 0 false; `recommended` 0 pass / 30 false; `mandatory` 0 pass /
  3 false. 33 of 33 adversary dispatches produced ≥ 1 blocking-class finding; none produced zero. 30 false verdicts
  produced 0 loop-backs.
- [measured] Adversary output: 188 findings over 33 dispatches, mean 5.7 — `reproduced` 118, `mutation-survived` 29,
  `advisory` 22, `narrowed-pin` 19; 147/188 = 78% in the two execution-requiring classes.
- [measured] Deviation: `RequestDeviation` optional on all 71 investigator dispatches, emitted 0 times.
- [measured] Wilson arithmetic at n = 3: 3/3 → [0.44, 1.00], 0/3 → [0.00, 0.56]. The overlap is arithmetic, not an
  empirical finding.
- [measured] That this reviewer is not one that finds nothing: F23 (detection 23/23 vs 23/23, McNemar p = 0.625), F26
  (20/20 across two harder tiers, blind and sighted), F31 (S-SILENT 23/23, S-SUBTLE 5/5, schema conformance 52/52,
  adjudication 14/14 confirmed, verified false positives 0/23).
- [inferred] "The criterion establishes power, not verdict" is F37 §2's own reading, tagged `[inferred]` there, and this
  record adopts it as the basis for decision 4.
- [documented] The `quality = { adversary, policy, gate }` shorthand and its "add after P7" status are stated in
  `references/2026-09-07-activity-graph-schema.md` §9; the deferral to P7 for the default is the schema's own wording.
- **Disclosed confounds, from the source.** The `claude` CLI upgraded mid-run, 2.1.263 → 2.1.265, catching exactly
  `recommended`'s r3 draw on all ten items (20 dispatches) while `optional` ran entirely on 2.1.263 — a confound *inside*
  the primary comparison. No effect is detectable at this resolution (r3 scored 7/10, identical to r2 on the old build,
  and the single differentiating result came from r1 on the old build), but the comparison is not instrument-matched.
  26 of 68 `confirm` invocations were full-suite runs rather than discriminating tests; the primary metric never read
  them, but the adversary was handed them. The `emit` record's `corpus_sha` field holds a per-item checkpoint sha and is
  misnamed.
- **What this evidence does NOT license.** It does not license a verdict on whether the block catches more root causes —
  that is the question P7 could not resolve, and decision 4 exists precisely because of it. It does not license any
  statement about `mandatory` (n = 1, arm dropped, and the configuration is unreachable — ADR 0081). It does not license
  a claim about the deviation channel's value (empty denominator). It does not generalise past **one graph, one stage,
  Opus only**: `root-cause` is one of four `localisation` family members and the others were not run. It does not
  generalise past ten seeded S-SILENT items from one corpus, so whatever E3's fixture construction gets wrong, P7
  inherits — and CLEAN's ground truth is separately known wrong for ≥ 13 of 23 items, which is why no false-positive rate
  is quoted here. It says nothing about gate behaviour: both `no-hypothesis` and `signoff` were auto-answered, and
  `no-hypothesis` never fired in 61 runs.
- **Corpus versus external evidence.** Nothing in the 2026-09-08 research pass contradicts this record, and nothing in it
  confirms it either — no external source addresses an activity-internal policy predicate. The nearest on-axis material
  cuts the same way from the opposite direction: evidence-augmentation §3 records *SWE-Gate* (arXiv 2609.04167), where
  **221 of 644 (34%)** agent repairs that pass functional tests fail review constraints derived from real PR comments,
  which is a further reason to expect a competent reviewer to keep finding things and therefore never to satisfy an
  empty-finding-set predicate. That is corroborating context for the mechanism, not a measurement of this block. Per the
  pass's own discipline, an absence of contradiction is not a confirmation.
