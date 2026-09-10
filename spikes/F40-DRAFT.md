## F40 — P2 RESULT: **the corpus contains no staleness to detect, so two of P2's three kill criteria have no denominator** — and on the rows both scopes score, the *fine* `derived_from` is three times worse than the coarse one, firing on the CLI's own stage bookkeeping

**Date:** 2026-09-10 · **Corpus pin:** redkiln @ `47faec9f` · **Labels:** 41 rows, one labeller, **blind and timestamp-verified** (`labelled_at` 2026-09-10T15:58:08Z precedes the engine's earliest record at 16:01:12Z; P2 is unaffected by METHOD-P amendment 8, so this is the one arm in the programme whose blindness is *checked* rather than *attested*) · **Model cost:** **$0** · **Instrument:** `spikes/p2/reconstruct-derived-from.py --phase engine`, scored by `spikes/p2/score-p2.py` (written for this result; nothing in the repository computed METHOD-P §2.3 before)
**Evidence discipline:** every figure is a count over 41 `(case, downstream)` rows at the pin. Rates carry a Wilson 95% interval. An `excluded` verdict is an abstention, reported as its own rate and never counted as a miss. Where a quantity has no denominator it is reported as **not computable**, never as 0.00 and never as 1.00.

### Metrics against the kill criteria (METHOD-P §2.4)

| criterion | threshold | measured | fires? |
| --- | --- | --- | --- |
| 1 — coarse `derived_from` | recall < 0.9 **or** precision < 0.7 → fine scope mandatory for `L`-class, `stale_policy = gate` | recall **NOT ANSWERABLE** (no denominator); precision **0.00** [0.00, 0.26] over 11 emitted positives | **the precision half FIRES; the recall half cannot be evaluated** |
| 2 — fine scope | cannot reach recall ≥ 0.9 either → `stale_policy = gate` for all `L`/`XL` | **NOT ANSWERABLE** — same missing denominator | **cannot fire either way** |
| 3 — advisory useful fraction | < 0.3 → PC3's advisory demoted to a log line | **out of scope here** — the advisory half (A01–A06) is labelled *after* the engine, in its own file (§2.3), and DECISIONS-2026-09-09 **#15** must be settled first | not evaluated |

### 1. The label set has no positive class [measured]

**0 of 41 rows are "should re-run"; all 41 are "need not."** That is not a labelling failure. It is a property of the perturbation corpus, and it is checkable:

**Eleven of the twelve perturbation cases change nothing but YAML frontmatter.** Classifying every hunk against the artifact's own frontmatter boundary at the pin:

| | cases | changed keys |
| --- | --- | --- |
| entirely inside frontmatter | **11 of 12** | `stage`, `status`, `updated`, `gate_open` |
| touching the body | **1** (P01) | two lines: *"The next stage (spec) has a clear starting point"* → *"The next stage has a clear starting point"* |

Those four keys are what the redkiln CLI rewrites as an item advances. They carry no fact about the system, which is what the labeller wrote against **36 of the 41 rows** (35 the phrase exactly, one a shorter variant of it; four rows carry no reason at all and one is P01's) — *"only frontmatter properties (process/stage status), no new/updated facts of the system"* — before any of this was computed.

**The mechanism is a recorded decision.** DECISIONS-2026-09-09 **#19** doubled the case count from 6 to 12 by adding the **card** as a perturbation candidate ("11/20 cards have ≥2 revisions vs 6/20 for any downstream artifact"). Cards are precisely the artifacts the CLI rewrites on every stage transition, so the extra six cases are the churn. The correlation is exact: the 11 frontmatter-only cases are the 11 card (`story.md`) perturbations, and P01 — the only non-card perturbation, on `discover.md` — is the only one with a body change. §2.2 registered "replaced by its previous git revision (**a real historical change**, not a synthetic edit)"; on this corpus the previous revision of a card is, almost always, the same document one stage earlier.

### 2. What the corpus *can* measure is the false-alarm rate, not precision [measured]

With no positive labels, every `Stale` the engine emits is a false positive **by construction**, so `precision = 0.00` is a property of the label set and not an independent reading of the engine. The quantity genuinely measured is **specificity** — how often the engine leaves a correctly-unchanged artifact alone.

| scope | specificity | 95% CI | precision | recall | tp | fp | fn | tn | excl | scored |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **coarse** | **0.73** | [0.58, 0.84] | 0.00 [0.00, 0.26] | **not computable** | 0 | 11 | 0 | 30 | 0 | 41 of 41 |
| **fine** | **0.55** | [0.38, 0.70] | 0.00 [0.00, 0.20] | **not computable** | 0 | 15 | 0 | 18 | **8** | 33 of 41 |

Fine's abstention rate is **8 of 41 = 19.5%** [0.10, 0.34] — a reconstructed scope of `unknown`, excluded from scoring and never counted as a miss (DECISIONS #16).

### 3. The fine scope is worse than the coarse one, and the aggregate intervals hide it [measured]

The two specificity intervals overlap, so on the aggregate the difference is **not separated at this n**. The paired comparison is what separates them. On the **33 rows both scopes actually scored**:

- coarse marks **5** `Stale`; fine marks **15**
- **10 rows flip `Present` → `Stale` going from coarse to fine. None flips the other way.**

Ten discordant pairs, all one direction. The direction is not in doubt even where the aggregate rate is.

**The mechanism, from the verdict breakdown.** Coarse fires on exactly one artifact kind and always the same one:

| downstream | coarse | fine |
| --- | --- | --- |
| `spec.md` | **11 Stale** | 5 Stale, 6 excluded |
| `implementation-report.md` | 12 Present | **5 Stale**, 6 Present, 1 excluded |
| `report.md` | 12 Present | **5 Stale**, 6 Present, 1 excluded |
| `_review.md` | 6 Present | 6 Present |

Coarse's declared map carries `spec.md ← <card>`, so a card write invalidates the spec and nothing else — one hop, and 11 false alarms out of 11 emissions. Fine additionally fires on `implementation-report.md` and `report.md`, which the **declared map does not connect to the card at all**. The reconstruction — "which upstream file changed before this artifact was last written" (§2.2) — is picking the card up as a derivation edge because the card is rewritten so often that it is nearly always the most recent prior write.

**So the fine scope is inferring a dependency from the frequency of the tool's own bookkeeping.** That is the same failure class as F39: an inference over a trace the instrument itself writes into. F39's `auth` regex collided with the vocabulary of a tool whose documents are *authored*; F40's `derived_from` reconstruction collides with the writes of a tool that stamps `stage` and `updated` on every advance. In both, the refinement that was supposed to help imports the noise — F39 §3 measured `union` worse than `commits` alone on two facets; this is the same shape at the artifact level.

### 4. What this settles, and what it does not

**Settles:** the coarse map's `spec.md ← <card>` edge invalidates a specification on stage bookkeeping, 11 times out of 11 on this corpus. Criterion 1's precision half fires on that alone, and the remedy it prescribes — make fine scope mandatory for `L`-class activities — is **contraindicated by §3**: the fine scope is the worse of the two here.

**Does not settle:** anything about detection. **No corpus item carries a substantive upstream change**, so nothing here says whether either scope catches real staleness. Criterion 1's recall half and criterion 2 in full remain unevaluated, and a re-run on a corpus with real content perturbations is what would evaluate them.

### Limits

1. **Two of three registered criteria cannot be decided by their own registered sample.** This is the **fourth** instance in this programme, after P1's α rule (F36), P7's criterion 1 (F37) and METHOD-P §1.4's 0.95 bar (F39). Four is a pattern in how this programme writes pre-registrations, not four coincidences: each of the four fixed a threshold without checking that the registered corpus could produce the denominator the threshold needs.
2. **The zero-positive label set is the corpus, not the labeller.** §1 establishes that mechanically. But it is also one labeller's judgement on 41 rows, and a second labeller is not available — §2.2 registers no second-rater check, and none was run.
3. **`derived_from` is reconstructed, not recorded** — the old redkiln never stored it (§2.5). Fine scope is therefore being scored as a *reconstruction method*, not as the design's intended recorded value.
4. **Twelve cases, one repo, one operator, one pin.** Eight of the twenty selected items produced no perturbation case at all (no artifact with two revisions).
5. **The advisory half is unrun**, and DECISIONS #15's `human_authored` remap changes what §2.3's advisory fraction measures. Criterion 3 is untouched by this finding.
6. **The specificity figures are over a single labeller's negatives** and inherit whatever systematic reading he brought to "no new facts of the system". That reading is stated in the label file's own `why` column on **36 of the 41 rows** (35 the phrase exactly, one a shorter variant of it; four rows carry no reason at all and one is P01's), so it is checkable rather than implicit.
