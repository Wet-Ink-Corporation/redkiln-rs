# Pre-registration #8 — S-CODEX: the second harness on the E3 fixtures

**Status:** pre-registration, per charter §8.2. **Written before any Codex dispatch. No S-CODEX data exists**
(`rk-adapter-codex` does not exist either; the adapter is Phase 4, ADR-0007). Registered 2026-09-11 to absorb
audit PIA-084, which found that S-CODEX had a budget (46 Codex dispatches, proposal §8) and a consumer (AC-239,
SM-154; ADR-0007) but **no primary metric and no pass rule** — and that the metric its row implied, E3's S-SILENT
detection, is **saturated** (23/23 in both arms, F23; METHOD-E3H §1), so a harness could match it and prove nothing.
**Baseline:** this document's commit. **Corpus pin:** redkiln @ `47faec9f`. **Instrument:** `spikes/runner`'s E3 rig
with a Codex dispatch path (`codex exec --json --output-schema <file> -o`, ADR-0007 §4.3 mapping), which has to be
built; its JSONL stream is classified by the same error taxonomy as the Claude path, and every record carries the
Codex CLI version the way F20 requires of `claude`.
**Why it exists:** ADR-0007 admits no non-Claude adapter behind a review or verification gate "until it passes
E3-shaped checks (S-CODEX)", and names no criterion for "passes" (its own evidence line :55 says so). AC-239 and
AC-496 are PROVISIONAL on exactly that gap. This document is the criterion.

---

## 0. Rules inherited from METHOD, METHOD-E3, METHOD-E3H and METHOD-S

Everything in METHOD §0, METHOD-E3 §6–§7, METHOD-E3H §5 and METHOD-S §0 applies unchanged: isolation and card
adoption verified per record (or the record is void, never a miss); pre-flight fields on every record; the
platform-error taxonomy; resume keyed `(arm, slug, repeat)`; results append-only under `spikes/results/`; **nothing
inferred from a single draw** (F27) — every per-item rate is `k/n` with a Wilson 95% interval, never pooled first;
strong detection is mechanical (the blocking finding names the seeded file, METHOD-E3 §7); CLEAN's ground truth is
METHOD-E3 §5's, with the F24/F31 caveat that it is an upper bound on the true false-positive rate.

Two rules specific to S-CODEX:

1. **The Claude comparator is the recorded arm (b), never re-run.** Codex is compared to `results/e3.jsonl`,
   `results/e3h.jsonl` and `results/s-typed.jsonl` as they stand. Re-running Claude beside Codex would spend the
   Opus budget on a comparator that F23/F26/F31 already fixed, and would invite reading two-arm noise as a
   harness difference (F27).
2. **A capability gap is a finding, not a void.** Where Codex cannot honour a card term the rig records the gap
   by name (effort parameter, tool restriction, structured-output enforcement, subagent control) on the record and
   the dispatch proceeds under the nearest honoured configuration. The metrics are computed twice — over all
   records, and over records with no gap — and both are reported. A gap that makes a cell unmeasurable is
   reported **not measurable**, never scored.

---

## 1. S-CODEX — does a second harness pass E3-shaped checks?

### 1.1 Why it is needed

The harness port has one real adapter. Portability is asserted by the port's shape (F41 compiles it) and by
nothing behavioural. Before a Codex-sourced `Findings` can close a gate the programme has to know, on the same
fixtures Claude was measured on, whether the second harness detects the defects the review gate exists to catch and
whether it approves clean work — and it has to know that on a measure that can fail. E3's primary measure cannot:
both Claude arms scored 23/23 on S-SILENT and the whole E3 comparison fell to the co-primary false-positive rate
(F23 "Detection saturated"; METHOD-E3H §1). E3-H's two harder tiers also sat at ceiling for Claude (20/20, F26), but
they are the hardest fixtures the corpus holds and the ones a weaker reviewer is most likely to miss — so they are
the primary here, and the saturated tier is recorded as a check, not a criterion.

### 1.2 Items and their known evidence class

| condition | items | Claude arm (b) result, recorded | role here |
| --- | --- | --- | --- |
| **S-SUBTLE, sighted** (METHOD-E3H §2–§3) | 5 seeds | 5/5 (F26) | primary |
| **S-SUBTLE, blind** (METHOD-E3H §9) | the same 5 seeds, criteria not named | 5/5 (F26) | primary |
| **CLEAN** (METHOD-E3 §5) | 23 | FP 2/23 as recorded, 0/23 after severity adjudication — [0.000, 0.143] (F23, F24, `e3-severity-annotation-ryan.md`) | co-primary |
| S-SILENT (METHOD-E3 §5) | 23 | 23/23 (F23) — **saturated** | secondary, budget permitting |

Items are the recorded fixture sets, unchanged; the S-SUBTLE seed list is `METHOD-E3H.md` §3 and the CLEAN list is
METHOD-E3 §4. No item is added or removed for the harness.

### 1.3 Procedure

- **Arm:** headless (arm (b) shape) only — `codex exec` with the card rendered as Codex's equivalent of the system
  prompt, the typed findings schema of METHOD-S §2.2 passed through `--output-schema`, the same `Read`/`Bash`
  tool grant in a disposable clone at the item's checkpoint. Codex's effort/model parameters are pinned in the
  card and recorded; where a parameter has no Codex equivalent, rule 2 applies.
- **Draws:** S-SUBTLE sighted and blind — **3 draws per seed per tier** (F27: a single draw on a 5-item tier
  decides nothing); CLEAN — 1 draw per item, matching the recorded Claude comparator's n; S-SILENT — 1 draw per
  item if run.
- **Order:** S-SUBTLE sighted → S-SUBTLE blind → CLEAN → (S-SILENT). The primary tier runs first so that a
  capability gap that voids it is found before the CLEAN budget is spent.
- **Ceiling:** `--timeout-secs 3600` on every dispatch, the S-TYPED ceiling (METHOD-S §7, 2026-09-06), for the
  reason given there: a lower ceiling censors slow, rejecting draws preferentially.

### 1.4 Metrics

- **Primary — S-SUBTLE-tier strong detection**, per seed per tier as `k/3` with its Wilson interval; the tier
  figure is the count of seeds detected on the **majority** of their three draws (modal detection), out of 10
  (5 sighted + 5 blind), with the pooled 30-draw rate and interval reported beside it.
- **Co-primary — CLEAN false-positive rate**, `k/23` with its Wilson interval, under the resolved policy of F31 and
  the severity adjudication route of METHOD-E3 §7 (Ryan adjudicates any Codex rejection on a CLEAN item, facts first,
  severity second, disclosed as NOT BLIND if the sheet is model-filled, as E3-SEV was).
- **Secondary:** S-SILENT detection (`k/23`, saturation check); schema conformance rate (S-TYPED's §2.5 metric);
  `total_cost_usd` or Codex's equivalent per dispatch, and wall clock; the capability-gap count by kind (rule 2).

### 1.5 Decision rule — what "passes E3-shaped checks" means

Stated as bounds, because a 10-seed tier and a 23-item CLEAN set cannot show equality with a comparator at
ceiling — they can only show a harness is not distinguishable from it, or that it is.

| measure | PASS | FAIL | otherwise |
| --- | --- | --- | --- |
| S-SUBTLE-tier modal detection (of 10) | **≥ 9/10** and no seed at 0/3 — Wilson [0.596, 0.982] or better | **≤ 6/10** — the harness missed the hardest defects on a majority of their draws for four or more seeds; Wilson [0.313, 0.832], which still overlaps arm (b)'s 10/10 [0.722, 1.000], so this FAIL is a rule on the count, stated as such, not an interval separation | **7–8/10** — [0.397, 0.892] / [0.490, 0.943]: **powerless** to separate the harness from arm (b) at this n; reported by that name, and the adapter stays out of gates until a larger tier exists |
| CLEAN false positives (of 23) | **≤ 2/23** — upper bound ≤ 0.268, the interval contains arm (b)'s recorded 2/23 and its adjudicated 0/23 | **≥ 7/23** — lower bound 0.156, disjoint from arm (b)'s adjudicated [0.000, 0.143] | **3–6/23** — intervals overlap arm (b)'s; **powerless** at n = 23, reported by that name |

**The adapter passes E3-shaped checks only if both PASS cells hold.** Any FAIL cell fails it. A "powerless" cell
in either row leaves ADR-0007's gate closed — the spike then reports what n would decide it (for the CLEAN row,
the same arithmetic as SM-086's: 0/n needs n ≥ 73 for an upper bound below 0.05) rather than re-thresholding. The
S-SILENT tier decides nothing: a 23/23 there is expected and is the saturation E3 §9's mirror case already
recorded; a miss there is reported and is a finding about the harness, not a criterion.

### 1.6 Prediction, before the data

1. S-SUBTLE-tier modal detection **≥ 8/10** — the seeds are real historical one-line defects with the whole spec
   supplied (sighted) and a stated symptom (blind); a current frontier harness holding `Read` and `Bash` finds most
   of them. The blind tier is where a miss is most likely.
2. CLEAN false positives **2–4/23** — F24 and F31 established that ≥ 13 of the 23 CLEAN items carry real defects a
   green suite let through, so an execution-grade reviewer that runs things will reject some of them; a Codex
   arm that runs less will reject fewer and look "cleaner" for the wrong reason. The adjudication route exists to
   tell those apart.
3. At least one **capability gap** is recorded on every dispatch (the effort parameter has no Codex equivalent,
   ADR-0007 §4.3), so the "no gap" sub-sample is empty and the all-records figures are the ones reported.
4. Schema conformance ≥ 0.90 under `--output-schema` (S-TYPED's 52/52 under Claude's `--json-schema` is the
   comparator; the prior is that Codex's enforcement is looser).

### 1.7 Sizing and threats

**Dispatches:** 30 (S-SUBTLE, two tiers × 5 × 3) + 23 (CLEAN) = **53 required**; + 23 (S-SILENT) = **76** if run.
The proposal's 46 (23 + 23) was the saturated pair; the change is recorded here and the design's §10.5 row and
SM-154 carry the new count. **Cost: unpriced** — Codex dispatches are not on the Claude subscription, the rig has
no Codex cost counter yet, and the first smoke dispatch prices the rest; the run halts at a cap set from that
price before the primary tier starts. Model cost on the Claude side is $0 (rule 1). Ryan's time: the CLEAN
adjudication, at most the number of Codex rejections on CLEAN.

**Threats.** (1) n = 10 seeds, n = 23 CLEAN — the "otherwise" cells above are wide and are expected to be hit;
that is reported as powerless, not as a pass. (2) The card is Claude-shaped; a Codex-native prompt might do better
or worse, and this measures the *port*, not the best Codex. (3) The comparator is single-draw (E3, E3-H) while the
primary tier here is triple-draw; the per-item intervals make that visible and no pooled comparison is made.
(4) Instrument lineage: the Codex CLI version is on every record; a mid-run upgrade is reported the way F37 §7
reports Claude's. (5) A harness that cannot enforce the schema returns prose; under S-TYPED's rule a digest that
fails its schema is `void: schema`, which lowers n rather than hiding the failure.

---

## 2. Sequencing

Not scheduled. It runs after the Codex dispatch path exists in the rig and before any `rk-adapter-codex` is bound
to a gate-feeding activity (ADR-0007; AC-239, AC-496). It gates nothing in Phase 1 (design §10.5: "Gates Phase 1?
No").

---

## 7. Amendments

None. Any change is disclosed here with the date and the records it post-dates, in METHOD-S §7's form.
