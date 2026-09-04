# Pre-registration #4 — E3-H: subtle-defect detection (the hard tier)

**Status:** new pre-registration, reported as such per charter §8.2.
**Written before any E3-H dispatch. No E3-H data has been seen.**
**Baseline:** this document's commit. **Corpus pin:** redkiln @ `47faec9f`.
**Does not supersede METHOD-E3.** E3 is answered and reported (F23). This is a
second difficulty tier using the same rig, arms, and metrics.

---

## 1. Why a second tier is needed

E3's primary metric **saturated**: 23/23 strong detection in both arms, zero
discordant pairs, nothing to test. §9 anticipated the mirror case — an all-arms
*floor* — and treats it as grounds for reporting inconclusive rather than firing
the kill criterion. The same reasoning applies to a ceiling: **a metric on which
both arms score perfectly cannot rank them.**

The whole E3 comparison therefore fell to an underpowered co-primary FPR, where
a 4-vs-2 difference gave McNemar p = 0.625.

**The cause is the seed construction, and it is diagnosable.** S-SILENT presents
the *pre-fix tree*: the capability is entirely absent. Detecting that requires
noticing that code which should exist does not — with the spec supplied, the AC
list enumerated, and the fix's own file list named in the prompt. That is not
the review task anyone is worried about.

## 2. The construction

**S-SUBTLE.** The fix is **fully present**. One line of it is wrong.

| condition | tree | gate | ground truth |
| --- | --- | --- | --- |
| `s-subtle` | the fix, with a single mutation baked in | **green** | `changes-requested` |

Every seed is a mutation **already verified to leave the affected suite green**
(F24) — so the reviewer faces exactly the escape-hatch shape the card exists to
catch: plausible code, a green gate, an honest-looking report, and a criterion
that is not actually met.

This is strictly harder than S-SILENT along the axis that matters: the reviewer
cannot succeed by observing absence. It must reason about *what the present code
does* against *what the criterion claims*.

## 3. Seeds

Five, each with its mutation already replicated and its greenness measured.
Four are line-level; one is the file-level survivor S0.1 found.

| # | item | mutation | verified green |
| --- | --- | --- | --- |
| 1 | `issue-146-147-boundary-window` | `forge-implement.js` step 7 ordering inverted to "FIRST, before the commit" | 36/36 |
| 2 | `issue-63-advance-gate-diagnostic-truncated` | `gates.ts` `detail: err.message` → `.split("\n").slice(0, 2).join("\n")` | 18/18 |
| 3 | `issues-135-129-declared-boundary-parse` | `verify.ts` `Path`-header guard → `if (false) return []` | 63/63 |
| 4 | `issue-74-plan-briefs-dead-plan-anchor` | `forge-plan-prds.js` `planWritten === true` → `!== false` | 28/28 |
| 5 | `issue-75-item-frontmatter-strict-drift` | `src/schema/common.ts` reverted (file-level) | S0.1 survivor, full gate green |

**Seed 5 is the one seed not of my construction** — S0.1 found it as the single
mutant its whole-suite gate failed to kill, which is independent evidence that
this difficulty tier exists in the corpus rather than being manufactured for it.

**Equivalent-seed triage (METHOD-E3 §10) is satisfied by construction for 1–4:**
each mutation violates a named AC, and the AC it violates is recorded per seed.
A reviewer approving the tree is therefore wrong about a specific criterion, not
merely insufficiently suspicious.

## 4. The fixture must hide the correct line, not merely change it

F17's lesson generalises. A mutation applied to a working tree shows up in
`git diff`, and a mutation committed on top of the fix leaves the fix as
`HEAD~1`. Either gives a reviewer with `Bash` the answer for free.

The fixture is therefore built as: depth-1 fetch of the **fix commit** (a
one-commit history, no parent), apply the mutation, `commit --amend`, then
`reflog expire --expire=now --all && gc --prune=now` to drop the unreferenced
original. **`assert_original_unreachable` then requires `git cat-file -e
<fix-sha>` to FAIL**, exactly as `assert_fix_unreachable` does for S-SILENT. A
record whose fixture still contains the pre-mutation commit is **void**, not a
miss.

## 5. Arms, metrics, checks — unchanged from METHOD-E3

Arms (a) and (b) as in METHOD-E3 §6, including the `StructuredOutput` channel
and the arm (a) validity checks (`dispatched_card`, `parent_inspected`,
`subagents_spawned`). Metrics as in §7: **strong detection** is
`changes-requested` **and** a cited finding naming a file the mutation touched.
Because each mutation touches exactly one file, the file test is sharper here
than in E3.

**No CLEAN arm is run.** E3 already measured the false-positive rate on the same
items with the same arms at n=23 each; re-running it would buy nothing and cost
the most expensive condition in the programme. The FPR carried forward is E3's,
and this is stated wherever E3-H detection is reported rather than presented as
a fresh confusion matrix.

## 6. What is predicted, before the data

Recorded so the result cannot be rationalised afterwards:

1. **Detection will fall below 23/23 — probably well below.** If it does not,
   the difficulty lever is wrong and the tier is uninformative, which is itself
   reportable.
2. **The arms will still not separate.** E3 found no quality difference at any
   power; the prior is that this tier has more power and still finds none.
3. **If the arms DO separate**, that is the first real quality signal in the
   programme and the direction matters more than the size: arm (b) worse would
   partially revive the RFC's concern; arm (a) worse would strengthen the
   inversion case beyond cost.

## 7. Sizing

5 seeds × 2 arms = **10 dispatches**. The tree is complete, so cost should track
CLEAN (~700–2000s, ~$1–4) rather than S-SILENT. Budget ~$20 and one five-hour
window. Resume is keyed per `(arm, slug, repeat)` as always.

## 8. Threats specific to this tier

1. **n = 5.** Only a large effect is detectable; anything short of it is
   reported as inconclusive, not as a trend.
2. **Four of five seeds are mutations I chose**, and I chose them because two
   reviewers had already found them in E3's CLEAN condition. That biases toward
   *detectable* defects and makes any high detection rate partly circular. Seed
   5 is not mine, and the write-up must not generalise past that.
3. **The reviewer may have reviewed these items before** — arm (b) saw all five
   in CLEAN. This is a within-corpus repetition confound. It affects both arms
   equally, so the comparison holds even if the absolute rate is inflated.
4. **A green suite is not a green world.** The seeds are verified green against
   the affected suite; the full gate was verified only for seed 5.
