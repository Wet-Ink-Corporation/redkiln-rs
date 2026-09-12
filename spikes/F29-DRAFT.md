## F29 — S-VAR RESULT: not one defect class is stable-reject. The verdict is a weighted coin, but every rejection it makes is on target

**Established by:** 60 CLEAN dispatches, 2026-09-05/06, arm (b), prose digest,
Opus/`xhigh`, corpus pin `47faec9f`, `claude` **2.1.263** on all 60 records.
`spikes/results/s-var.jsonl`, its own file so repeats can never pool with E3.
Pre-registered in `METHOD-S.md` §1 (`c2c3fc8`); citation sub-metric pre-registered
at `80094d1`; dispatch-ceiling amendment at `8c6f2dd`. **60 records, 60 valid,
0 void.** $176.96, 16.2 h of dispatch, no throttle halt.

### The primary result

| item | class | n | reject | rate | Wilson 95% | band |
| --- | --- | --- | --- | --- | --- | --- |
| `fix-advance-write-lock-gate-scope` | **Reproduced** | 10 | 7 | 0.70 | [0.40, 0.89] | **flaky** |
| `issues-135-129-declared-boundary-parse` | MutationSurvived | 10 | 6 | 0.60 | [0.31, 0.83] | **flaky** |
| `issue-146-147-boundary-window` | MutationSurvived | 10 | 3 | 0.30 | [0.11, 0.60] | **flaky** |
| `issue-74-plan-briefs-dead-plan-anchor` | MutationSurvived | 10 | 2 | 0.20 | [0.06, 0.51] | **flaky** |
| `issue-63-advance-gate-diagnostic-truncated` | NarrowedPin | 10 | 1 | 0.10 | [0.02, 0.40] | stable-approve |
| `issue-62-new-stale-pinned-template` | none (control) | 10 | 0 | 0.00 | [0.00, 0.28] | stable-approve |

**No item reached stable-reject.** The band requires a Wilson lower bound above
0.55, i.e. 9/10 or 10/10; the highest result in the set is 7/10.

**Every item carrying a defect F24 confirmed is flaky.** The four items with a
known, replicated finding all land in the middle band. The two items that
classify are the two with the *least* to find.

### The one that matters: a live regression is approved 3 times in 10

`fix-advance-write-lock-gate-scope` is not a test-quality quibble. F24 reproduced
its two behavioural claims end to end: `basis` flips `evaluated`→`memoized`, and
a refused entry gate stops rendering `produces`. Both E3 arms rejected it. Run
ten times, **this reviewer approves it in 3 of 10 draws.**

That is not a threshold wobble on a marginal finding. It is the strongest defect
in the corpus, present in the code the reviewer is reading, with `Read` and
`Bash` available — and roughly a third of the time the review comes back
`pass: true` with **zero issues recorded**.

### Evidence class does predict stability — the ordering holds

| class | mean reject rate | items |
| --- | --- | --- |
| Reproduced | **0.70** | 1 |
| MutationSurvived | **0.37** | 3 |
| NarrowedPin | **0.10** | 1 |
| none (control) | **0.00** | 1 |

Monotone, and in exactly the predicted order: Reproduced ≥ MutationSurvived >
NarrowedPin, control at the floor. F27 offered "strength of the underlying defect
predicts verdict stability" as a hypothesis on n=4. **It survives at n=60** — as
an ordering. It does *not* deliver a blocking threshold, because the ordering
runs 0.70 → 0.37 → 0.10 with no class reaching stability.

### Secondary: all 19 rejections are on target. Zero false alarms

| item | rejections | citing a pre-registered F24 target file | off-target |
| --- | --- | --- | --- |
| all six, pooled | **19** | **19** | **0** |

Scored by `results/s-var-citations.py` against `results/s-var-citation-targets.json`,
both committed before any record existed. **When this reviewer rejects, it
rejects for the right reason, every single time.** The control never rejected at
all: 0/10, upper bound 0.28. Every rejection carried 2–9 issues; every approval
carried exactly 0.

The failure mode is therefore **entirely one-sided**. There is no false-positive
problem here — there is a *false-negative* problem, and it is large.

Note `audit-s.py`'s own on-target column reads `0/k` throughout. That column is
dead by construction: `review_probe` passes `reverted = []` for `--condition
clean`, so `score_files` has no targets and `matched_files` is always empty. The
sub-metric was recovered from `scored.cited_files` against a target list fixed
before the data. **`audit-s.py` was not modified.**

### The §1.6 predictions, line by line

| # | prediction | outcome |
| --- | --- | --- |
| 1 | `fix-advance` is stable-reject (10/10 or 9/10) | **FALSIFIED.** 7/10, flaky. Two draws short of the band, in the direction that matters. |
| 2 | at least one MutationSurvived item is flaky; `issue-146-147` flips at 0.4–0.8 | **SPLIT.** All three are flaky — stronger than predicted. But `issue-146-147` landed at **0.30**, below its predicted band. |
| 3 | `issue-63` (NarrowedPin) is flaky, 0.3–0.7 | **FALSIFIED.** 1/10 = 0.10, **stable-approve**. Predicted to wobble; it barely fires. |
| 4 | `issue-62` approves ≥9/10 | **CONFIRMED.** 10/10. |

One of four confirmed. Both falsifications say "the reviewer blocks less than
predicted", which is the same direction as the headline.

### §1.5 decision rule — which branches fire

- **Branch 3 FIRES — the architecture-changing one.** *"If `Reproduced` is flaky
  (the item with a live behavioural regression is approved in ≥2 of 10): the
  review gate cannot be trusted as a sole blocker for any class, and Phase 1 must
  pair it with the deterministic gate for every required AC."* Approved 3 of 10.
  Not a marginal call: the threshold was ≥2, the result is 3, and the whole
  Wilson interval [0.40, 0.89] falls short of stable-reject.
- **Branch 2 FIRES.** All three `MutationSurvived` items are flaky, so the
  finding is reliable while the reviewer's own threshold is not: **the policy
  must block on the finding's presence, not on the reviewer's `pass`** — which is
  what S-TYPED's schema does. **This strengthens D4.**
- **Branch 1 does NOT hold.** The proposal's default policy assumed
  `Reproduced`/`MutationSurvived` were stable-reject. Neither is.
- **Branch 4 does NOT fire.** The control rejected 0/10, well under the ≥2/10
  that would make the false-positive floor real. **No `Advisory` sink is needed
  on this evidence.**

### What this settles

1. **The review gate cannot be a sole blocker.** Not for any class, including the
   strongest. D4's thresholds cannot be set to "Reproduced blocks" and inherit
   reliability from the reviewer, because the reviewer supplies the finding only
   70% of the time.
2. **D4's central move is validated by its failure mode rather than its success.**
   Removing `pass` and computing the verdict from findings is right — but only if
   the finding is *present*. On 30% of draws this reviewer emitted no finding at
   all on a reproduced regression, and no policy over an empty findings list can
   block.
3. **Pairing with the deterministic gate is required, not preferred.** Every
   required AC needs a mechanical check. The reviewer is a high-precision,
   low-recall instrument: use it to *add* findings, never to *clear* an AC.
4. **The false-positive fear was misplaced.** 19/19 on-target, control 0/10.
   E3's 4/23 and 2/23 FP rates, already weakened by F24 and F27, should be read
   as noise around a floor near zero.
5. **F27's hypothesis holds as an ordering and fails as a threshold.** Defect
   strength predicts *relative* stability. It does not produce a class that can
   be trusted to block.

### Limits

1. **One arm (b) only.** Nothing is claimed about arm (a).
2. **The class ordering rests on one item per class** for Reproduced, NarrowedPin
   and control. "Class" is confounded with "item" in three of four cells; only
   MutationSurvived has n=3 items, and those three span 0.20 to 0.60 — most of
   the range the ordering is built on. **The ordering is suggestive, not
   established.**
3. **Repetition confound.** All six items were reviewed by this model in E3 and
   F27. Affects the absolute rates; the ordering less so.
4. **n=10 gives ±0.3 intervals at worst.** Enough to separate §1.4's bands, not
   to estimate a rate finely. `fix-advance` at [0.40, 0.89] is consistent with
   anything from a coin flip to 9-in-10.
5. **Instrument version drift.** All 60 records are `claude 2.1.263`; the
   programme's earlier records are 2.1.258–2.1.261. The set is internally
   uniform, but it is not the binary that produced E3 or F27 — a second reason
   §1.3's refusal to merge F27's draws was right.
6. **The dispatch ceiling was raised mid-programme** (§7, `8c6f2dd`); 10 earlier
   records at 2400 s are quarantined in `results/s-var-2400.jsonl`, unedited and
   excluded. The censoring risk that forced it was real but smaller than argued
   at the time: in the final set only 1 of 60 draws exceeded 2400 s (it was a
   rejection), 6 exceeded 2000 s, and rejections ran a median 707 s against
   approvals' 628 s. The direction held; the magnitude was modest.
7. **Draw order was repeat-major** (round r across all six items, then r+1), an
   operational choice not fixed by §1.3. It was taken so that five-hour window
   position could not be confounded with item. Given that the throttle turned out
   never to bind, item-major would probably have been harmless — but that was not
   knowable in advance.
8. **CLEAN's ground truth remains wrong for 5 of 23 items** (F24). This spike
   measures verdict *stability*, not correctness, so it does not bite here — but
   the "rejections" counted above are, on F24's evidence, correct findings.

### Incidental: F21's per-dispatch draw is ~8× too high for this workload

Not what the spike was for; recorded because 60 dispatches is the largest clean
sample of it in the programme.

| window | draws | delta | per draw |
| --- | --- | --- | --- |
| 1 | 20 | 0.21 | 0.0105 |
| 2 | 21 | 0.20 | 0.0095 |
| 3 | 15 | 0.16 | 0.0107 |
| **pooled** | **56** | — | **0.0098** *[corrected 2026-09-11: 56 draws pool to 0.0102; 0.0098 is the four-window pool over 60 — Correction (2026-09-11)]* |

F21 records **+0.077** five_hour per CLEAN dispatch. The measured figure here is
**0.0098 — 7.8× lower** *[the four-window pool over 60 draws; three full windows give 0.0102, 7.5× — Correction (2026-09-11)]* — consistent across three independent full windows and
far past the 0.01 quantisation that made F21's estimate resolution-limited.
`seven_day` drew 0.0017/draw (0.27 → 0.37 across the run).

**Consequence:** the throttle never came close to binding, the run halted zero
times, and METHOD-S §1.7's "three five-hour windows" sizing was wrong in the safe
direction. **Wall clock, not headroom, is the binding constraint on a CLEAN
set:** 16.2 h for 60 draws at a median 671 s each.

### Correction (2026-09-11)

Dated correction from the 2026-09-10 pre-implementation audit (PIA-077). The five-hour-draw table's **pooled** row
mixes two pools: its **56** draws are the three full windows (20 + 21 + 15, Σ delta 0.57 → **0.0102** per draw),
while **0.0098** is the pool over all **60** draws including a fourth, partial window (4 draws, delta 0.02, reset
2026-09-07T01:00Z; Σ delta 0.59 / 60 = **0.0098**) [measured, `spikes/results/s-var.jsonl` `step.headroom`
recount 2026-09-11, grouped by `five_hour_resets_at`]. The table's three rows, the 0.0098, the 60-dispatch count in
the surrounding prose and the seven-day figure (0.27 → 0.37) are all reproduced; only the pooled row's n was wrong.
**The downstream figure uses the four-window pool: 0.0098 over 60 draws**, and "7.8× lower" than F21's 0.077 is
0.077 / 0.0098 = 7.86. On the three-window pool it is 0.077 / 0.0102 = 7.5×. Either reading leaves the consequence
paragraph unchanged (the throttle never bound; wall clock, not headroom, binds a CLEAN set).
