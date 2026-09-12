## F36 — P1 arm L RESULT (unlabelled half): the LLM-emitted context vector is **not stable enough to drive a deterministic selection engine**, and the pre-registered remedy does not repair it. Across two instruments and both packs, an identical required-artifact set `R(stage, ctx)` across ten draws is reached on **5/20, 3/20, 2/20 and 2/20** items against an 80% line — the kill fires four times out of four. §1.4's anchor-rewrite re-run left `blast_radius` unmoved (0.488 → 0.473, so it becomes human-supplied) and made `external_dep` **constant** rather than reliable (200/200 `false`). And three axes whose anchors were byte-identical between the two runs drifted by up to **−0.168**, which is larger than the effect the α < 0.6 threshold is meant to detect — so on this corpus that threshold is not a decidable predicate

**Established by:** 200 classification dispatches (20 items × 5 repeats × 2 models), 2026-09-07 22:22Z – 2026-09-08 07:41Z, arms `L-haiku` and `L-sonnet`, default temperature, no effort parameter, corpus pin `47faec9f`, `claude` **2.1.263** on all 200 records. `spikes/results/p1-arm-l.jsonl`. **200 valid, 0 void.** Pre-registered in `METHOD-P.md` §1. **$10.66** of a $40 cap, ~3.4 h of dispatch, no throttle halt. **Plus the §1.4 re-run** (§3): a further 200 dispatches under arm `L2-*`, card `0384080216a9`, `spikes/results/p1-arm-l-v2.jsonl`, **200 valid, 0 void**, $9.70 *[corrected 2026-09-11: $10.71 — Correction (2026-09-11)]* — analysed together by `spikes/p1/analyse-arm-l.py --compare-to`. Card `spikes/prompts/p1-classify.md` (`9d86e7044490`), schema `spikes/schemas/p1-classify.schema.json` (`5fd33472d73e`), both committed before the first dispatch and unedited during the run — every record carries both digests and the run asserts they never changed. Analysis: `spikes/p1/analyse-arm-l.py` → `spikes/results/p1-arm-l-analysis.txt`, stamped `cat:f89154d762a7 fac+art:149a74a0d053 pk:6994b61bc64d` (METHOD-P §0 rule 2).

**This is the unlabelled half only.** METHOD-P §1.2's human agreement panel — two labels per item per facet, written from the item alone before unblinding (§0 rule 1) — is not written yet, and nothing here is compared to it. Arm D's precision/recall against that panel, which is the criterion that decides whether a facet *leaves the model*, therefore has no result in this draft. The skeleton is `spikes/results/p1-arm-l-labels-RYAN-TODO.md`.

**Disclosed instrument bias:** the anchors were written by the author of the schema they serve, in the same session lineage as the catalog — §1.5 names this as a limit and it is not mitigated here. A second author would be the check, and has not been run.

**Corpus shortfall, disclosed:** 20 items, not §1.2's 30. The frozen `.bklg` holds **2** `bug.md`, **3** `initiative.md` and **0** spike cards of any kind, against quotas of 8 / 4 / 3. Strata were not back-filled (AFK brief lane A2). See §4.

### Metrics against the kill criteria (METHOD-P §1.4)

| metric | value [measured] | kill | verdict |
| --- | --- | --- | --- |
| Krippendorff alpha, `criticality` (ordinal, 10 draws/item) | **0.806** | < 0.6 | does not fire |
| alpha, `novelty` | **0.745** | < 0.6 | does not fire |
| alpha, `size` | **0.604** | < 0.6 | does not fire — by 0.004 |
| alpha, `blast_radius` | **0.488** | < 0.6 | **KILL FIRES** |
| alpha, `market_facing` (nominal) | 0.721 | < 0.6 | does not fire |
| alpha, `external_dep` (nominal) | **0.165** | < 0.6 | **KILL FIRES** |
| alpha, `has_repro` (nominal) | 0.784 | < 0.6 | does not fire |
| alpha, `kind` (nominal) | 0.847 | < 0.6 | does not fire |
| items with an identical modal `R(stage, ctx)` across all 10 draws, pack `bug-standard` | **5/20 = 25%** | < 80% | **KILL FIRES** |
| same, pack `story` | **3/20 = 15%** | < 80% | **KILL FIRES** |
| haiku-vs-sonnet modal disagreement, `novelty` | 5/20 = **25%** | > 20% is *reported*, not a kill | reported |
| same, `blast_radius` | 5/20 = **25%** | > 20% is *reported*, not a kill | reported |
| same, `criticality` / `size` / `kind` | 5% / 15% / 15% | | below the reporting line |
| derived-facet precision (arm D) | **not computed** | ≥ 0.95 removes it from the record | **waits for the panel** (§0 rule 1) |

Four of the eight registered facets pass the alpha bar comfortably; two fail; `size` passes by 0.004 and should be read as failing-adjacent rather than as passing. The two kills that matter most are the **modal-`R` kills**, because they are the ones the selection engine actually feels.

### 1. What the draws showed

**1.1 Per-facet stability is not the same question as engine determinism [measured].** Three ordinal axes clear alpha 0.6 and `kind` clears 0.85, yet only 25% / 15% of items reach an identical `R` across ten draws. The axes are individually reasonable and jointly insufficient: `R` is a function of all four at once, so four axes each flipping on a minority of draws compose into a required-artifact set that rarely repeats. **Reporting per-facet alpha alone would have made this instrument look far healthier than it is** — which is why §1.3 asks for both, and why the second metric is the one Phase 1 should act on.

**1.2 The `unknown` rate is the mechanism [measured].** Per axis, over 200 draws: `novelty` **46.0%**, `size` **39.5%**, `blast_radius` **31.0%**, `criticality` **27.0%**. An `unknown` is not a neutral abstention in this engine: F34 B14 and F35 §1.6 establish that at selection it is treated as **high** by position, and F35 measured 23 activities that are selected at `<axis> = unknown` and not at `low`. So an axis oscillating between `unknown` and `low` moves `R` by the full distance between the cheapest and most expensive band. The anchors, as written, sent between a quarter and a half of all draws up that path.

**1.3 `external_dep` at alpha 0.165 is an anchor defect, not model noise [inferred].** The v1 anchor read "`true` when finishing this needs something the team does not control: a third-party service or API, another team's merge, a vendor, a purchase, a credential someone else issues, an upstream release." It never says whether a library the project already depends on counts — and this corpus is a Node project whose items routinely touch npm packages. Two readings of the same sentence give opposite answers on the same item, which is what an alpha of 0.165 looks like. Named as [inferred] because the flip's cause is not directly observable in the record; the v2 re-run (§3) is the test.

**1.4 The models agree with each other more than either agrees with itself [measured].** Modal haiku-vs-sonnet disagreement is 0–25% per facet, and 0% on `market_facing`. Within-model draw-to-draw instability is larger than between-model disagreement on every axis. This is F27's finding recurring in a new place: the *threshold judgement* is the unstable part, and it is unstable in the same way in both tiers. It also means §1.4's model-disagreement clause does not discriminate here — the cheaper model is not worse, so cost, not stability, chooses the triage default.

**1.5 Zero voids in 200 dispatches [measured].** Every record carried a schema-valid classification and an adopted card (tool-set equality against the declared `["StructuredOutput"]`). The F20 failure mode — a card allowlist suppressing `--json-schema` so the model answers in prose — did not occur, having been checked with a single validated draw before the set.

### 2. What this settles, and what it does not

- **Settles:** the v0 anchors are not fit to drive `R` deterministically. Both modal-`R` kills fire by a wide margin (25% and 15% against an 80% line), so §1.4's second consequence is live: *the rubric rows over these axes are rewritten to coarser bands before Phase 1 authors packs*.
- **Settles:** `blast_radius` fails the per-axis bar, does not respond to the rewrite (§3.1), and therefore becomes **human-supplied at the triage gate** on §1.4's own rule.
- **Settles:** `external_dep` should not stay LLM-emitted. v1 was unreliable (α 0.165 on a 96%-skewed marginal); v2 was unanimous only by becoming **constant** (§3.2). Neither reading supports it.
- **Settles, and was not asked:** α < 0.6 is not a decidable predicate on 20 items × 10 draws. Untouched anchors moved up to −0.168 between instruments, carrying `size` across the kill line without an anchor change (§3.3). §1.4's per-axis rule needs a bootstrap interval on α, a bigger corpus, or replacement by modal-`R`.
- **Does not settle:** whether any facet can leave the model. That is arm D's precision/recall against the human panel, and the panel is unwritten.
- **Does not settle:** whether coarser bands would fix it. Nothing here measures a two-band engine; F35 §1.4 already showed that banding choices are load-bearing in the other direction (`canary-release` was unreachable under two bands).

### 3. The §1.4 remediation: run, and it answers a different question than §1.4 asked [measured]

§1.4's first bullet: an axis below 0.6 has "its anchors rewritten first and re-run once; if still < 0.6,
human-supplied." `spikes/prompts/p1-classify-v2.md` (`0384080216a9`) rewrote **`blast_radius` and `external_dep`
only**; every other anchor and the `unknown` policy are byte-identical to v1. 200 dispatches, **0 void**, $9.70,
recorded under arm `L2-*` in `spikes/results/p1-arm-l-v2.jsonl` — never pooled with `L-*`. *[$9.70 corrected 2026-09-11 to $10.71 — Correction (2026-09-11)]*

| facet | rewritten? | α v1 | α v2 | Δ |
| --- | --- | --- | --- | --- |
| `criticality` | no | 0.806 | **0.638** | **−0.168** |
| `novelty` | no | 0.745 | 0.748 | +0.003 |
| `size` | no | 0.604 | **0.530** | **−0.074** |
| `blast_radius` | **yes** | 0.488 | 0.473 | −0.015 |
| `market_facing` | no | 0.721 | 0.764 | +0.043 |
| `external_dep` | **yes** | 0.165 | **undefined** | — |
| `has_repro` | no | 0.784 | 0.717 | −0.066 |
| `kind` | no | 0.847 | 0.846 | −0.001 |

**3.1 `blast_radius` did not respond.** 0.488 → 0.473. §1.4's remedy is exhausted, so on the pre-registered rule
this axis becomes **human-supplied at the triage gate**. The rewrite was not inert: its `unknown` rate fell
**31.0% → 19.5%** and `med` went from 14 to 54 draws. The anchor stopped the model reaching for `unknown` and
moved those draws into a *spread* across `med`/`high` instead of into agreement. Decisiveness did not produce
reliability, which is worth knowing before anyone rewrites another anchor expecting it to.

**3.2 `external_dep` collapsed rather than improved.** v1: 8 `true` / 192 `false`, disagreeing on 4 of 20 items.
v2: **200 `false`, 0 `true`, disagreement on none.** α is reported as *undefined* rather than 1.0 — correct, since
no disagreement was available to measure — but the honest reading is that the rewritten anchor did not make a hard
judgement reliable, it made the facet **constant**. A facet with no variance still keys a `require` row and will
never change the answer, which is worse than an unreliable one because it looks healthy.
Two caveats stated rather than buried: v1's 0.165 is partly an artifact of a badly skewed marginal (α is harsh
when one category holds 96% of draws), and v2's explicit "`false` for a library already in the manifest" plus
"`false` when the item is silent" is the likely cause of the collapse. **Neither number supports keeping this
facet LLM-emitted.**

**3.3 The α statistic is not stable enough to carry a 0.6 threshold at this sample size.** Three axes whose
anchors are **byte-identical between the two instruments** moved by −0.168, −0.074 and −0.066. The drift on
untouched anchors is **larger than the effect §1.4's threshold is meant to detect**, and it carried `size`
(0.604 → 0.530) across the kill line without a word of its anchor changing.

This is F27 recurring one level up. F27 established that the model's *threshold judgement* flips on identical
input; this says the *statistic computed over those judgements* also moves materially between two draws of the
same instrument at 20 items × 10 draws. **"α < 0.6" is therefore not a decidable predicate on this corpus** — an
axis can be moved across it by resampling alone. §1.4's per-axis rule needs a bigger corpus, an interval on α
itself (a bootstrap over items), or replacement by the modal-`R` criterion.

**3.4 The modal-`R` criterion needs none of that, and fires four times out of four.** v1: **5/20**
(`bug-standard`) and **3/20** (`story`). v2: **2/20** and **2/20**. Against an 80% line, in both instruments, on
both packs. It is computed from the engine's own output rather than from an inter-rater statistic, it did not
drift ambiguously, and it is the number that reflects what the selection engine actually feels. **This is the
result Phase 1 should act on.**

### 4. Limits

1. **20 items, not 30.** The corpus cannot supply §1.2's strata: 2 `bug.md` (quota 8), 3 `initiative.md` (quota 4), 0 spike (quota 3). Reported as a shortfall, never back-filled.
2. **The `bug-standard` reading rests on 2 bug items.** With only two bug cards in the corpus, the `bug-standard` modal-`R` figure of 5/20 is computed over a mostly non-bug item set classified against a bug pack. The `story` figure (3/20, 165 story cards available) is the sound one. This is the single largest caveat in this draft.
3. **Anchors written by the schema's author** (§1.5), unmitigated.
4. **Derived facets held fixed** at a text/node context when computing `R`, so that only what the model emits varies between draws. A real item's derived half varies too, and would add, not remove, variance.
5. **One corpus, one pin.** Everything here is `47faec9f`.
6. **Alpha is undefined, not 1.0**, where every draw agreed; no facet hit that case in this run, but the analysis reports `n/a` rather than a flattering number if one ever does.
7. **Nothing is compared to a human panel.** Half of §1.3's metrics have no value in this draft by design.
8. **The v2 instrument is a single re-run**, which is what §1.4 licenses ("re-run once") and no more. The drift finding in §3.3 rests on one pair of draws per axis; it is enough to show α is not resolvable at this size, and not enough to quantify how unstable it is. A bootstrap over items would do that and was not run.

### Correction (2026-09-11)

Dated correction from the 2026-09-10 pre-implementation audit (PIA-077; ERRATA E-0079-1). The §1.4 re-run's cost is
stated twice above as **$9.70**. That figure was read from the sonnet driver's last spend line before the haiku
driver had finished its final repeats. The re-run cost **$10.71** — haiku **$4.91** + sonnet **$5.79** — from the
two log lines and the record file:

- `spikes/p1-arm-l-v2-haiku.log` :265 — `spend in this file: $10.71 of $40.00` (the last spend line written to the
  shared results file);
- `spikes/p1-arm-l-v2-sonnet.log` :316 — `spend in this file: $9.70 of $40.00` (sonnet's last line, written while
  haiku's repeats 4–5 were still running);
- `spikes/results/p1-arm-l-v2.jsonl` — Σ `total_cost_usd` over 200 records: `L2-haiku` $4.9142 (100), `L2-sonnet`
  $5.7926 (100), total **$10.7068** [measured, recounted 2026-09-11].

The v1 run's **$10.66** (`L-haiku` $4.7539 + `L-sonnet` $5.9068, `p1-arm-l.jsonl`) is unaffected, and the two-run
total `HANDOFF.md` carries — **$21.37** — was already the sum on the corrected figure (10.6607 + 10.7068 = 21.3675),
so only this draft's two cost readings were wrong. No metric, α or verdict in this draft depends on the cost.
ADR-0079 :211's cost parenthetical is reconciled by ERRATA E-0079-1.
