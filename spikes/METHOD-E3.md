# Pre-registration #2 — E3 (and E6) replacement metric

**Status:** new pre-registration, reported as such per charter §8.2.
**Supersedes:** the E3/E6 primary metric in `references/2026-08-31-rust-inversion-spike-charter.md` §10.
**Reason:** FINDINGS **F9** — the original answer key does not exist.
**Baseline:** this document's commit. **Corpus pin:** redkiln @ `47faec9f`.
**Nothing in E3 or E6 has been run.** No data has been seen. This registration
is written before any dispatch, which is the only reason a metric change at this
point is legitimate rather than fishing.

---

## 1. Why the original metric is dead

E3's primary metric was **blocker recall**: of the findings in a review's
historical `## Required Changes`, what fraction does each arm surface?

All fifteen such sections at the pinned SHA open with **"None"**. The numbered
items that follow are carry-forwards to *other* projects; six sections have no
list at all. There are no historical blockers to recall.

This is charter §8.1's censoring problem taken to its limit: the process cannot
advance an item until it passes, so `## Required Changes` is a section that
exists to record the absence of blockers. Recall against it is undefined.

## 2. The replacement, in one line

**Re-create real historical defects by reverting real historical fixes, and
measure whether each reviewer arm catches them — against clean controls, on
identical material, paired per item.**

## 3. Why this is stronger than the metric it replaces

- **Ground truth exists**, which is the property the corpus cannot otherwise
  offer.
- **The defects are real.** Each is a bug that actually shipped, was actually
  filed, and was actually fixed. This is not synthetic injection — it is the
  same construction S0.1 uses, and for the same reason: the accompanying test
  proves the defect is not equivalent to correct behaviour.
- **It tests the rubric the card claims.** `agents/redkiln-review.md` promises
  adversarial AC verification and escape-hatch hunting. This measures exactly
  that, rather than agreement with a predecessor whose output was censored.
- **It is paired.** Same item, same spec, same prompt, one bit flipped —
  which is what charter §9.2 asks for given that temperature 0 is not
  deterministic.

## 4. Inputs

**23 fix-lane items** under `.bklg/support/` that carry all four of: a `spec.md`
with AC-### criteria, an `implementation-report.md`, an approving `_review.md`,
and a checkpoint SHA that resolves at the pinned corpus commit.

**Amended after S0.3 (disclosed):** this section originally said 22, excluding
`issue-75-item-frontmatter-strict-drift` on my earlier finding that its SHA did
not resolve. It does (`582a3e9`); the earlier scan could not see it. Two further
items (`reap-leaked-serve-children`, `issue-146-147-boundary-window`) were
recorded as having 0–1 ACs because their specs define ACs as table rows rather
than list items. All three are now in. **The input set grew by corrections to my
own parsing, not by a change of criteria**, and no data had been seen.

The authoritative list is `spikes/results/s03-fixlane-manifest.jsonl`, which
records per item the AC ids, the checkpoint SHA, how that SHA was located, and
the fix commit's file set.

## 5. Conditions — three per item, paired

For each item, a fixture worktree is built at the fix commit and then modified:

| condition | construction | gate state | ground truth |
| --- | --- | --- | --- |
| **CLEAN** | fix intact, as shipped | green | `approved` |
| **S-FAIL** | fix's *code* reverted, its *tests* kept | **red** | `changes-requested` |
| **S-SILENT** | fix's code **and** tests reverted | **green** | `changes-requested` |

**S-SILENT is the discriminating condition and the reason this design was
chosen.** The tree is the pre-fix state: the affected-area gate passes, the
report reads plausibly, and nothing is obviously broken — but the spec's ACs are
not met by the code. Catching it requires doing what the card says it does:
adversarially verifying each AC against real, reachable behaviour rather than
taking a green gate at face value. It is the escape-hatch class the agent exists
to catch.

**S-FAIL is a sanity control**, not the measurement. An arm that misses S-FAIL is
not reading the tests at all. Because it is cheap to pass, it is run on a subset
(see §8) and reported separately; it must never be pooled with S-SILENT.

**CLEAN is the false-positive control** and is co-primary with detection.
Charter §10 is right that recall alone is meaningless: an arm that rejects
everything scores perfect detection. The literature it cites measured a reviewer
at FPR 0.20% / FNR 87.9% — it rejected 88% of real defects. Both numbers are
reported together, always.

## 6. Arms

Unchanged from the charter, with C6's framing correction made explicit:

| arm | configuration |
| --- | --- |
| **a** | headless parent whose prompt dispatches `Task({subagent_type: "redkiln:redkiln-review"})` |
| **b** | `claude -p --agents '<card>' --agent redkiln-review --model opus --effort xhigh --json-schema REVIEW_DIGEST` |
| **c** | as (b) plus explicitly passed context — repo conventions, the spec, the AC list |

**Arm (a) is not a live interactive session.** It cannot be: the charter's own
principle 4 requires every arm to be a plan the same runner executes. It is a
headless parent containing an in-session subagent, and that wrapper is a stated
confound rather than a hidden one.

**Arm (a) is verified, not assumed.** `subagent_stats.spawned` must be ≥ 1 or the
record is void. Without that check, "the subagent reviewed and disagreed" and
"the subagent never spawned" are indistinguishable — RK-B0013 one level up.

**Arms (b) and (c) must pass both `--agents` and `--agent`.** FINDINGS F7:
`--agents` alone is inert, and an arm built that way measures a generic Claude.
`effective.agent` is read back from the child's `system/init` and must equal the
card name, or the record is void.

**Model and effort are pinned explicitly** on every arm. The card is
`model: inherit` with no effort, so a headless dispatch that omits them silently
inherits the session default — which would confound the comparison in exactly
the direction that flatters arm (a).

## 7. Metrics

**Primary — detection rate on S-SILENT**, per arm:

> the fraction of S-SILENT items for which the arm returns `changes-requested`
> **and** its cited findings name at least one file touched by the revert.

The file test is **mechanical**: the revert's file set is known exactly, and the
digest's findings carry `file:line`. No judgement is required for the primary
metric. This matters because the strong criterion is what stops an
always-reject arm from scoring perfectly.

**Recorded alongside, never substituted:**

- **weak detection** — verdict alone, ignoring citations. The gap between weak
  and strong detection *is* the "rejected for the wrong reason" rate.
- **finding fidelity** — does the cited finding describe the actual defect?
  This needs judgement, and is the only place the LLM judge is used. Reported
  with **Cohen's κ** against Ryan's blind annotations, never raw agreement
  (PRIOR-ART §4; our labels are far too imbalanced for agreement to mean
  anything).

**Co-primary — false-positive rate on CLEAN**: fraction of CLEAN items the arm
returns `changes-requested` for.

**Combined — Matthews correlation coefficient** over the CLEAN / S-SILENT
confusion matrix. Reported because it is robust to class imbalance, which is the
defining property of this corpus, and because a single number that can be gamed
by always-reject is worse than none.

**Secondary:** verdict agreement between arms on identical material; cost;
headroom draw; wall clock; `thinking_tokens`; `overhead_ms`.

## 8. Sizing, and the order in which it is cut

Full design: 22 items × 3 conditions × 3 arms = **198 dispatches per repeat**, at
Opus/xhigh. That is not affordable at any useful number of repeats, and the Opus
family sub-limit is not readable (FINDINGS F1), so it will be interrupted.

**E0 fixes the repeat count.** Until then, the pre-declared reduction order — so
that cutting is not a judgement made after seeing results:

1. **S-FAIL drops to a 6-item subset.** It is a sanity control; it does not need
   power.
2. **Arm (c) drops before arm (b).** (a) vs (b) is the question; (c) is the
   mitigation, and only matters if (b) loses.
3. **Items drop last**, and by a pre-declared rule: keep the items whose specs
   carry the most ACs, since those exercise the rubric hardest. The dropped list
   is reported.

Resume is on, so an Opus interruption costs one dispatch, not an arm.

## 9. Kill criterion — unchanged in force, restated in the new metric

**If arms (b) and (c) both show materially lower S-SILENT detection than arm (a)
— a drop the E0-sized test can detect — at comparable or worse false-positive
rate, the inversion thesis fails, the RFC recommendation is withdrawn, and the
programme stops.**

Two guards against firing this on an artefact, both learned the hard way:

- A record whose `isolation_ok`, `effective.agent`, or (for arm a)
  `subagents_spawned` check fails is **void**, not a miss. F7 and F8 are both
  failure modes that would have manufactured exactly this outcome.
- If **every** arm, including (a), detects near zero, the finding is about the
  *seeds or the rubric*, not about headless dispatch. In that case E3 is
  inconclusive and is reported as such — it does not fire the kill criterion.

## 10. Equivalent-seed triage

A reverted fix that leaves behaviour genuinely unchanged is the equivalent-mutant
problem (PRIOR-ART §5) and must not count against a reviewer. Any S-SILENT seed
whose full test suite still passes **and** whose spec ACs are still satisfiable
is excluded from the denominator, and the exclusion list is published.

This is why S0.1 runs first: it produces exactly this list as a by-product, on
the same fixture rig.

## 11. E6 inherits all of the above

E6 (reviewer configuration: fresh vs self-review vs panel) shares the metric and
the inputs, and therefore shares this replacement. Its panel arm uses
**Krippendorff's α** rather than κ, since it has three raters.

## 12. Threats to validity, stated before the data

1. **A reverted fix is one defect shape — a regression.** Novel defects in
   unfamiliar code may be harder. This measures regression-catching, and the
   write-up must say so rather than generalising to "review quality".
2. **The reviewer may recognise the codebase.** These are real redkiln bugs, and
   a model may have priors about this repo. Affects all arms equally, so the
   *comparison* holds even if the absolute rate is inflated.
3. **Reverting can produce an incoherent tree** — a partial revert that no
   developer would ever have written. Every seed is inspected for plausibility
   before use; implausible ones are dropped and reported.
4. **The gate must genuinely be green under S-SILENT.** If reverting the tests
   leaves other tests failing, the condition collapses into S-FAIL. Verified per
   item at fixture-build time; failures are reclassified, not quietly kept.
5. **n is small** — at most 22 items, fewer after triage. Only large effects will
   be detectable, and near-misses are reported as inconclusive rather than as
   trends.
6. **Single repo, single operator, single domain** — carried forward from charter
   §13.1 and unchanged.
