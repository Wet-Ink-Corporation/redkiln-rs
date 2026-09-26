---
id: "0168"
title: "**The severity vocabulary is `blocking | advisory`, and no severity threshold T is admissible on this evidence: ADR-0080 decision 2 bullet 2 governs.** S-SEV (D21-42 11c) labelled F37's 188 adversary findings with an independent rater of record (D21-58). The vocabulary was usable, but no dispatch of 33 was left without a `blocking` label, so T = `blocking` would be a constant terminal condition. Supersedes ADR-0080 decision 2 bullet 1 in part: its severity threshold, on this evidence. Its confidence arm, bullet 2 and the rest of ADR-0080 stand."
status: proposed
date: 2026-09-26
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 11c (T settled now, on a measurement), D21-43 N18 (F37 as corpus; E3 excluded; (A) for threat-model), D21-54 (sequential sample of whole dispatches), D21-57 (the open question on severity-only rows), D21-58 (Jev the rater of record; stability replaces the re-label); spikes/METHOD-SEV.md with Amendments 1-5; runbook/phase-1/s8b-catalog-quality.md §8b.5 (threat-model's row), §8b.6; runbook/phase-1/s8h-cards-and-run-environment.md §8h.4; redkiln-corpus results/sev-f37-result.txt at 285e011c70cfa0fc92d91089792834846d99bcf7; references/adr/0080 decisions 2 and 3
supersedes: ["0080"]
superseded_by: []
related: ["0074", "0082", "0092", "0101", "0111", "0136", "0162"]
evidence: ["S-SEV", "F37"]
consumers: []
---

## Context and problem

ADR-0080 decision 3 withheld the threshold: *"Pick the threshold when severity has a settled vocabulary and a
destination for advisory findings; not before."* ADR-0082 supplies the destination (*"The Advisory sink is a
first-class destination, not a discard"*). The vocabulary did not exist. At `s8b`'s open [measured, §8b.6]:

- **39** shipped `expr` rows in 39 files under `.redkiln/` read `severity`, every one as
  `findings.none(severity in ['blocking'])`, alone or in a conjunction [re-measured 2026-09-26: 39 files].
- `spikes/p9/schemas.toml` declares `fields = ["ac", "claim", "evidence", "severity", "files", "repro"]`.
- `rk_types::artifact::findings::Finding` has **no** `severity` member.
- **188 of 188** F37 findings carry none. Their keys are `ac, claim, evidence, files, rationale, repro`.

D21-42 11c settled that the vocabulary and T are fixed *now*, by record, on a severity-labelling measurement. That
measurement is **S-SEV**, pre-registered in `spikes/METHOD-SEV.md` before any sheet existed.

- **Corpus.** F37's 188 adversary findings over 33 dispatches, read through the object store at `CORPUS_PIN`.
  E3's six findings are excluded (D21-43 N18).
- **Design.** D21-54 made it a sequential sample of whole dispatches. Stage 1 was one dispatch per item, 56
  findings over 10 dispatches. On the operator's stage-1 labels, `--decide-stage1` printed `CONTINUATION REQUIRED`
  (METHOD-SEV Amendment 3), so all 188 were labelled.
- **Rater.** The operator labelled all 188, stage 1 at the labels commit and the continuation at
  `1b3e1025e8cd13f173cb849833b7f52b57c24d52`. He then found the findings *"too difficult to make an honest
  determination on the context supplied"* and doubts his labels.
- **D21-58** made TypeSafe's Jev (`typesafe/jev-1.13-20260917`, through OpenRouter's decisions endpoint) the rater
  of record. It withdrew the 48-hour re-label and redefined A1 for a model rater: CANNOT-TELL at most 10% of the
  labelled n, **and** Cohen's κ ≥ 0.61 between the first run and each of two identical re-asks and one re-ask with the
  options' order reversed. METHOD-SEV Amendment 5 fixed that analysis, and it was committed before any of Jev's
  answers was opened.

§8b.6 fixes what the record says in each branch. The measured result selects its second branch: **A1 passes, and A3
shows no variability.**

## Decision drivers

- **A threshold that every dispatch fails is not a terminal condition.** ADR-0080 rejected
  `findings.none(evidence in [...])` because 33 of 33 dispatches failed it: the verdict was a constant function of the
  arm. A severity threshold that fails on the same 33 of 33 dispatches repeats that defect under another field name.
  This is golden constraint 15, and A3 measures it.
- **D21-42 11c took T *now*, against the recommendation to owe it.** A record that settles no T is a deferral, which
  11c and D21-54's panel both declined. A record that settles *"no admissible severity threshold on this evidence"* is
  a settled T (§8b.6).
- **ADR-0080 decision 2 bullet 1 is a Decision sentence.** A record that leaves it permitting a severity threshold
  while settling none leaves an accepted permission with no admissible value. Under ADR-0092 decision 1, only a
  supersession moves it.
- **The 39 shipped rows read a field that no type carries**, so whatever the branch, the field needs a named source
  and owner.
- **The rater saw little.** Jev saw only the masked finding text, which is the context the operator judged too thin.
  The result can settle what a context-limited independent rater assigns, and no more (D21-58).

## Considered options

### Option — the second branch: `blocking | advisory`, and no admissible severity threshold on this evidence (taken)

The vocabulary is fixed. T is *"no admissible severity threshold on this evidence; bullet 2 governs"*, and ADR-0080
decision 2 bullet 1 is superseded in part. Taken because it is the branch §8b.6 and METHOD-SEV Amendment 5
pre-registered for this result. A1 passes: CANNOT-TELL 0/188, and κ 1.000 in all three stability comparisons. A3
shows no variability: 0 of 33 dispatches carry no `BLOCKING` label.

### Option — the first branch: T = `blocking`, which `threat-model`'s (A) edit would then bind

This reading needs A1 to pass **and** A2 and A3 both to show variability. A2 does: 5/147 execution-class findings
were labelled `ADVISORY` and 29/41 other-class findings `BLOCKING`. It lost on **A3**. Every one of the 33 dispatches
carries at least one `BLOCKING`-labelled finding (0/33, Wilson 95% [0.000, 0.104]). So a terminal condition
`findings.none(severity in ['blocking'])` would have been false on every dispatch in the corpus. That is F37's
constant-function result, which §8b.6 pre-registered as making T = `blocking` **inadmissible**. The sensitivity
analysis gives the same reading (0 of 33).

### Option — owe T to a later measurement or stage

Record the vocabulary and leave T unset, as ADR-0080 decision 3 originally did. Declined twice before the data:

- by **D21-42 11c** (*"Settled now, against the recommendation to owe it"*);
- by **D21-54's judge panel**. Its "owe the measurement" option, the ledger records, reversed 11c's *"now"* outright
  and sent S-SEV to no gated stage.

A settled *"no admissible threshold on this evidence"* is not that option: it is a value, it has a stated reopening
condition (a new measurement, by a new record), and bullet 2 governs meanwhile.

### Option — the measurement as first registered: the operator the sole rater, with a 48-hour re-label for A1's κ

This was §8b.6's *"Rater: the operator, alone"* and D21-54's re-label of 38. **D21-58 superseded it** for three reasons:

- the operator found the masked findings too thin to judge honestly;
- he doubts the accuracy of his labels;
- the stage could not sit idle for the 48-hour gap.

His labels are kept, committed and reported below. ~~On them A1's κ half was never computed, so they could not have
passed A1 as registered.~~ *(dated 2026-09-26, `s8b` refuting round 8)* On them A1's κ half **could not pass**: all
188 of his answers are `BLOCKING`, so any re-label drawn from stage 1 (METHOD-SEV §5) gives p_e = p_o, and κ is 0 or
undefined; §6.1 fails A1 on either. Under the method as registered at `bbf809c` this measurement could end only in a
NOT SETTLED STOP. Both label commits (`a2f4fab`, `1b3e102`) predate D21-58's commit (`c04b316`), so the rater change
was made after that outcome was fixed in committed data, and it moved the result from STOP to Branch 2. Amendment 5 was
committed before Jev's answers were opened, but not before the operator's were. D21-58 carries the same disclosure, and the operator's confirmation
that the switch stands with this known is owed before this record is accepted.

### Option — read T off the high-confidence subset only

Take Jev's findings at or above median confidence (0.95; 98 findings) as the labelled set. Rejected:

- Amendment 5 registers confidence as a sensitivity analysis that decides nothing.
- The subset gives the same A3 reading (0 of 33), so it could not have selected another branch.

### Option — supersede nothing, and leave bullet 1 standing with T unset

Rejected by §8b.6's own text: *"It does not supersede nothing."* Bullet 1 permits a terminal condition at a
threshold T. With no admissible T, a standing bullet 1 is a permission no author can use correctly and any author can
cite.

## Decision

**1. The severity vocabulary is `blocking | advisory`.** These are the two values a finding's `severity` may take,
spelled lower-case as the 39 shipped rows spell `'blocking'`. `CANNOT-TELL` is a **rater's answer, not a severity
value**. It was the labelling instrument's third option: the rater could not say from the text shown. No finding
carries it, and no type admits it. Its measured rate here is 0/188.

**2. T is settled: *"no admissible severity threshold on this evidence; bullet 2 governs"*.** This is a value, not a
deferral. It answers ADR-0080 decision 3, whose two preconditions are now both met: a settled vocabulary (clause 1)
and a destination for advisory findings (ADR-0082). ADR-0080 decision 3 is discharged, not superseded. A quality
block's terminal condition is chosen from ADR-0080 decision 2 bullet 2's quantities: a bounded loop-back count, a
re-execution outcome, or a delta against the previous iteration's findings. It is not chosen from a severity threshold.

**3. What this record supersedes, in part.** ADR-0080 decision 2 bullet 1 reads: *"a **threshold** over severity or
confidence — the terminal condition is "no finding at or above threshold T survives re-execution", not "no finding
exists""*. This record supersedes that bullet **as to severity only, and on this evidence.** A severity threshold is
no longer an admissible terminal condition for a quality block.

The following stand unchanged, and ADR-0080 stays `accepted`:
- bullet 1's **confidence** arm, which S-SEV did not measure;
- decision 2's lead sentence, *"A policy step's terminal condition must be decidable over something the reviewer
  varies"*, and its lint sentence;
- bullet 2;
- decisions 1, 4 and 6, and decision 5 as ADR-0111 left it;
- every forbids bullet.

*On this evidence* means F37's corpus, labelled by a rater who saw only the masked finding text. A later measurement
that shows termination variability can admit a severity T again, but only by a new record that supersedes this one.

**4. `threat-model`'s (A) edit has nothing to bind.** §8b.5's row prescribes (A), a threshold at 11c's T (D21-43
N18). This record settles no admissible T, so (A) binds nothing, and §8b.5's STOP applies. **This record does not
decide that row.** Its disposition is recorded separately, in D21-59 (taken 2026-09-26 by the operator):
`threat-model` declines the block under ADR-0080 decision 4, and its `policy` step is deleted (v1 → v2, ADR-0072).

**The severity field's source and owner.** The owner is **`s8h`** (D21-47 cross-file finding 4; `s8h` §8h.4). `s8h`
adds `severity` to `rk_types` `Finding` as a required member (not an `Option`), typed by a closed enum whose
spellings are clause 1's `blocking | advisory`, read from this record by its recorded id, with its tests
(`findings::severity_is_a_required_member_typed_by_the_11c_vocabulary`,
`findings::a_severity_outside_the_11c_vocabulary_is_refused`,
`quality_block::a_severity_row_reads_the_typed_member_over_a_non_empty_list`, mutant M28). The step that fills it is
**the reviewer's structured output**: `s8h` §8h.4 reads *"The reviewer card's structured output fills it, because
`Findings` is that card's output kind and the schema is the contract"*. `review::policy`'s table does not fill it,
because that table computes blocking from class, weight and re-execution. This record does not add the field. Until
`s8h` lands, the 39 shipped rows behave as the evaluator has them (`rk_kernel::predicate`'s `quantify` and its path
lookup), and ADR-0136 does not refuse them at bind time because it governs `when` guards:
- over an **EMPTY** findings list, `findings.none(severity …)` is `Tri::True`, and `severity` is never read;
- over a non-empty list whose findings carry no `severity`, the element lookup falls through to the context and
  returns `EvalError::UnknownField("severity")`;
- no non-empty list is read as absent-means-true.

The rows are evaluated in `s8d`'s resolver (R2). Under clause 2, these 39 rows read a severity that no threshold may
terminate on: their `findings.none(severity in ['blocking'])` conjunct is the T = `blocking` form that A3 found
constant. What those rows become is **not decided here**. D21-57 raised and left open whether a severity-only row
with no re-execution (code-review's and checklist-review's) conforms to ADR-0080 decision 2. This record neither
answers that question nor rewrites any row.

**5. The labels contradict the live policy table's `narrowed-pin` and `advisory` cells. The contradiction is
reported here, and no row moves.**

The row lives in **ADR-0074 decision 5**. ADR-0096 supersedes ADR-0074 in part, but only *"clause 1's repro shape
and clause 2's `mutation-survived` definition"*. Decision 5 reads: *"`reproduced` and `mutation-survived` block;
`narrowed-pin` blocks only under a strict weight; `advisory` and `Unverifiable` never block."*
`crates/rk-kernel/src/review/policy.rs` carries it as its table:

| weight \ class | `reproduced` | `mutation-survived` | `narrowed-pin` | `advisory` |
| --- | --- | --- | --- | --- |
| `Lenient` (`recommended`, the default) | block iff flipped | block iff flipped | advise | advise |
| `Strict` (`mandatory`) | block iff flipped | block iff flipped | block iff flipped | advise |

The module says of it that *"the row for `narrowed-pin` is the operator's severity call, not a measurement"*.

Against it, at class grain over all 188 findings (from the sealed keys, whose `evidence` and `arm` fields the
sheets masked):
- **`narrowed-pin`, recommended arm (`Lenient`, where the table advises):** 16 of 16 labelled `BLOCKING` by **both**
  raters.
- **`narrowed-pin`, mandatory arm (`Strict`):** 3 of 3 labelled `BLOCKING` by both raters. This does not contradict
  the table.
- **`advisory`, where the table never blocks:**
  - Jev labelled 10 of 22 `BLOCKING`: 9 of 20 in the recommended arm and 1 of 2 in the mandatory arm.
  - The operator labelled all 22 `BLOCKING`.
- **In the other direction,** Jev labelled 5 of 118 `reproduced` findings `ADVISORY`, where the table blocks iff
  flipped. The operator labelled none `ADVISORY`.

**This is a contradiction, reported and not acted on.** An accepted Decision sentence moves only by supersession, on
the operator's word (ADR-0092 decision 1). ADR-0074 decision 5, `policy.rs`'s table and AC-298 stand as they are.

**Stated limits:**
- The raters answered a normative question, *"should this finding have blocked the item?"*, on masked text. The
  table decides after re-execution (*"block iff flipped"*), and neither rater saw a re-execution outcome.
- Jev is a context-limited rater: it saw no code, scope or ACs.
- The operator labelled against his own stricter standard, not the card's, and doubts his labels (METHOD-SEV
  Amendment 2, D21-58).
- Reading the F37 arm as the table's weight follows `resolve_review_tier`'s projection
  (`recommended → Lenient`, `mandatory → Strict`). F37 did not run this table.

**6. No claim about the review gate's calibration.** S-SEV measures a rater's normative labels on masked finding
text. It does not measure the reviewer's emitted `severity`, the gate's decisions, or either half of its error
rates. ADR-0101 clause 5 stands, and S-SUBTLE-CB is unrun.

**7. The record flips only on the operator's dated ledger line** (§8b.6).

**The measured result, as the checkpoint reads it:**

**S-SEV labels commit:** `a2f4fab9fb411002c3d700bb3da9632667b1a2a2`

**S-SEV stability commit:** `285e011c70cfa0fc92d91089792834846d99bcf7`

**S-SEV labelled:** n = 188 findings over 33 dispatches; rule: continued to 188

The labels commit adds the operator's filled stage-1 sheet, `results/sev-f37-labels.md`, to `redkiln-corpus`. The
continuation sheet was added by `1b3e1025e8cd13f173cb849833b7f52b57c24d52`. The stability commit adds
`results/sev-f37-jev.jsonl` (Jev's first run), `results/sev-f37-jev-stability.jsonl`, the two Jev-filled sheets,
both sealed keys and `results/sev-f37-result.txt`. None of these commits moves `CORPUS_PIN`. This record names none
of the corpus checkout's HEADs.

## Consequences

**Positive**

- ADR-0080 decision 3's two preconditions are met, and its question has an answer on the record rather than an open
  item.
- The 39 rows' field now has a vocabulary, an owner (`s8h`) and a named filling step, and its interim behaviour is
  stated as the evaluator has it.
- A second constant-function terminal condition is stopped before it ships under a new field name.
- The shipped quality blocks' route is bullet 2, which ADR-0080 already specified. No new quantity is invented here.

**Negative**

- `threat-model`'s (A) edit, which D21-43 N18 prescribed, cannot be made, and `s8b` stopped on that row until D21-59
  was taken (2026-09-26: the row declines the block).
- 39 shipped rows carry a conjunct that this record declares inadmissible as a terminal threshold. Their disposition
  waits on D21-57's open question and on `s8h`/`s8d`, and until then they pass on an empty list and error on a
  non-empty one.
- The settled T rests on one context-limited rater with no ground truth. A richer-context arm (the item's spec and
  ACs), offered in D21-58, was not taken. If it were run and showed termination variability, a new record would be
  owed.
- Jev's perfect stability is weak evidence. A deterministic model returns the same answer to an identical request, so
  κ = 1.000 against `r1` and `r2` shows only reproducibility. κ = 1.000 against `perm` shows the answers are invariant
  to the options' order. Neither shows that the judgement is stable.

**What this forbids**

- Forbids using a severity threshold, T = `blocking` or any other, as a quality block's terminal condition on this
  evidence. It also forbids citing ADR-0080 decision 2 bullet 1's severity arm as the licence for one, in the catalog,
  a technology pack, a shorthand expansion or a lint's admitted forms.
- Forbids binding `threat-model`'s (A) edit to a severity threshold under this record, and forbids reading this record
  as deciding that row's disposition (D21-59).
- Forbids a `severity` value outside `blocking | advisory`, and forbids `CANNOT-TELL`, or any "unknown" spelling, as a
  severity value.
- Forbids reading an absent `severity` on a non-empty findings list as `advisory` or as absent-means-true.
- Forbids quoting A2's discordance proportions or A3's `k/33` as population rates, and forbids quoting Jev's `confidence`
  as a probability. It is TypeSafe's spread statistic.
- Forbids quoting Jev's κ = 1.000 as evidence that its judgement is stable, or its labels as the findings' true severity.
- Forbids moving ADR-0074 decision 5's `narrowed-pin` or `advisory` cells, or `policy.rs`'s table, on this record. The
  contradiction in decision clause 5 is reported, and only a supersession on the operator's word moves a cell.
- Forbids citing this record for any claim about the review gate's calibration (ADR-0101 clause 5).

## Evidence and confidence

- [measured] **A1, CANNOT-TELL:** 0/188, Wilson 95% [0.000, 0.020]. The point rule (≤ 10% of n) is met, and the
  interval's upper bound is at or below the bar. `score.py --first` over Jev's two sheets; redkiln-corpus
  `results/sev-f37-result.txt` at `285e011` (sha256 `f27b33f16dd588ec78074be5faf8034e9dbc5172c7908a60c477e9756aec8347`,
  byte-identical to the working copy in `D:/repos/redkiln-sev/`).
- [measured] **A1, stability** (Cohen's κ, first run against each re-ask, bar 0.61 in all three), from the same file.
  Stability passes, and A1 passes.

  | re-ask | κ | p_o | p_e | n | changed answers |
  | --- | --- | --- | --- | --- | --- |
  | `r1` | 1.0000 | 1.0000 | 0.8355 | 188 | 0 |
  | `r2` | 1.0000 | 1.0000 | 0.8355 | 188 | 0 |
  | `perm` | 1.0000 | 1.0000 | 0.8355 | 188 | 0 |

  A deterministic model's perfect agreement shows order-invariance (`perm`) and reproducibility (`r1`, `r2`), not
  stable judgement.
- [measured] **A2** (rows: class group; columns: BLOCKING / ADVISORY / CANNOT-TELL):

  | class group | BLOCKING | ADVISORY | CANNOT-TELL |
  | --- | --- | --- | --- |
  | `reproduced` + `mutation-survived` | 142 | 5 | 0 |
  | `advisory` + `narrowed-pin` | 29 | 12 | 0 |

  Execution class labelled `ADVISORY`: 5/147, [0.015, 0.077]. Other class labelled `BLOCKING`: 29/41,
  [0.555, 0.824]. A2 shows variability: severity is not a function of class on these labels. The proportions are
  not rates.
- [measured] **A3:** dispatches with no `BLOCKING`-labelled finding: **0/33**, [0.000, 0.104]. There is no variability,
  which makes it constant. With `CANNOT-TELL` counted as `BLOCKING` (report only): 0/33.
- [measured] **Branch, printed by Amendment 5's analysis driver:** *(re-worded 2026-09-26, refuting round 8: the
  driver and the Jev runners are not committed; their committed output is `results/sev-f37-result.txt` at `285e011`,
  and the committed `score.py` alone prints PENDING over Jev's sheets, having no stability arm)* *"BRANCH 2 — A1 passes, A2 or A3 shows no variability:
  vocabulary fixed; T = "no admissible severity threshold on this evidence; bullet 2 governs"; the record supersedes
  ADR-0080 d2 bullet 1 in part"*.
- [measured] **Sensitivity, deciding nothing** (Jev first-run confidence ≥ its median 0.95; 98 findings). Execution
  class labelled `ADVISORY`: 0/83. Other class labelled `BLOCKING`: 15/15. A2 variability: true. A3: k = 0 of 33
  dispatches, no variability. The branch reading is the same. Confidence: median 0.9500, min 0.2200, max 1.0000.
  Jev's choices: `BLOCKING` 171, `ADVISORY` 17, `CANNOT-TELL` 0.
- [measured] **Report only, powerless:** dispatches by (no `BLOCKING` label, run root-cause correct): (False, False) 8
  and (False, True) 25. No dispatch lacks a `BLOCKING` label, whether its run's root cause was right or wrong.
- [measured] **The operator's labels** (reported; he labelled against his own stricter standard, not the card's, and
  doubts them: METHOD-SEV Amendment 2, D21-58). All 188 are `BLOCKING`. A2: 0/147 and 41/41. A3: 0/33. ~~His A1 is
  pending, because no re-label was supplied.~~ *(dated 2026-09-26, refuting round 8)* His A1's κ half could not pass:
  with a constant first pass any re-label gives κ = 0 or undefined (see the first-registered option above). Agreement with Jev: 0.9096 over 188 (171 both `BLOCKING`; 17 operator
  `BLOCKING`, Jev `ADVISORY`). κ is 0.0000, because the operator's labels have no variance, so p_o = p_e. The κ of
  zero reflects that constancy and is no finding about disagreement.
- [measured] **The instrument.** 752 requests, 188 first-run plus 564 stability (188 each of `r1`, `r2`, `perm`).
  Every request returned status `ok` and was served by `typesafe/jev-1.13-20260917`. Total cost US$0.027077
  (US$0.006769 first run, US$0.020307 stability), summed from the committed JSONL files' `usage.cost`.
- [measured] **Order of events** (commit times, 2026-09-25 -0700):
  - METHOD-SEV added at `bbf809c` (16:16:30);
  - Amendment 2 at 22:18:05, before the labels commit `a2f4fab` at 22:18:16;
  - Amendment 5 at `c04b316` (23:01:12), before the stability commit `285e011` (23:13:49).

  The sealed keys in the corpus hash to the values Amendments 1 and 3 recorded before labelling (`2a5dea38…`,
  `020e2c15…`).
- [measured] **Class grain, both raters, all 188** (`score.py`'s `load_pair` over the four filled sheets and the two
  sealed keys in `D:/repos/redkiln-sev/`, each byte-identical to its redkiln-corpus copy at `a2f4fab`, `1b3e102` and
  `285e011` [measured 2026-09-26]):

  | class | Jev `BLOCKING` | Jev `ADVISORY` | operator `BLOCKING` | operator `ADVISORY` |
  | --- | --- | --- | --- | --- |
  | `reproduced` | 113 | 5 | 118 | 0 |
  | `mutation-survived` | 29 | 0 | 29 | 0 |
  | `narrowed-pin` | 19 | 0 | 19 | 0 |
  | `advisory` | 10 | 12 | 22 | 0 |

  `CANNOT-TELL` is 0 in every cell. By arm:
  - `narrowed-pin`: recommended 16, mandatory 3, all `BLOCKING` under both raters.
  - `advisory`, Jev: recommended 9 `BLOCKING` and 11 `ADVISORY`; mandatory 1 and 1.
  - `advisory`, operator: 22 `BLOCKING`.
- [documented] The live `narrowed-pin` row is ADR-0074 decision 5, which ADR-0096 does not reach (*"clause 1's repro
  shape and clause 2's `mutation-survived` definition"* only). It is carried by `crates/rk-kernel/src/review/policy.rs`'s
  table and `classify`, which returns `Advise(NarrowedPinAdvisesAtWeight)` below `Strict` and `Advise(AdvisoryClass)`
  for `advisory` at every weight.
- [inferred] Comparing the F37 arm with the table's weight uses `resolve_review_tier`'s projection
  (`recommended → Lenient`, `mandatory → Strict`). F37's run did not execute this table, and the labels are
  normative answers given without the re-execution outcome that the table's *"block iff flipped"* reads.
- [measured] **39** files under `.redkiln/` (excluding `worktrees/`) carry an `expr` reading `severity`, all as
  `findings.none(severity in ['blocking'])` [2026-09-26]. `EvalError::UnknownField` is raised at the path lookup in
  `crates/rk-kernel/src/predicate/eval.rs`.
- [documented] D21-42 11c, D21-43 N18, D21-54, D21-57 (its open question), D21-58 (the rater, the redefined A1, and
  *"What the result can claim"*); `s8h` §8h.4 (the owner and the filling step); ADR-0080 decisions 2 and 3; ADR-0082;
  ADR-0074 decision 5; ADR-0092 decision 1.
- **Stated limits** (D21-58, METHOD-SEV §9).
  - Jev saw only the masked finding text: no code, no scope, no ACs. This settles the severity **a context-limited,
    independent rater** assigns, not the findings' true severity.
  - There is no ground truth to validate either rater against.
  - Jev's `confidence` is TypeSafe's spread statistic, `(3 × top probability − 1) / 2`, and is not a probability.
  - Blindness is attested, not verified. A `rationale` leaks the evidence class in part.
  - One graph, one stage, ten seeded items and an Opus-only reviewer. Findings within a dispatch are not
    independent.
  - S-SEV measures a rater's normative labels, not the reviewer's emitted `severity`, so it does not show how the
    shipped rows would behave at run time (D21-54, disclosed).

Confidence in the branch is moderate. A3's constancy is a census over the corpus (33 of 33), and both raters and the
sensitivity subset give the same reading. Confidence that a richer-context rater would read the same is low: that arm
is unrun.
