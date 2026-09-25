# Pre-registration — S-SEV (D21-42 11c; D21-43 N18 as amended by D21-54)

**Attestation (2026-09-25, Ryan Britton):** Ryan Britton, confirmed in-session 2026-09-25 — I attest that I have not read F37's adversary findings
individually: no `claim`, `rationale`, `files` or `repro` of any of the 188 findings in `results/p7.jsonl` at the
corpus pin, in that file or in any document that quotes one, before labelling them under this method.

**Status:** pre-registration, `proposed`. **Written before any S-SEV sheet exists and before any label.** No
S-SEV data has been seen; the sheet is generated only from the commit that adds this file (s8b §8b.6: *"committed
**before** the sheet is generated"*), and `spikes/sev/make-sheet.py` refuses to run from an uncommitted or modified
copy of this file, or while the attestation above is unsigned.
**Baseline:** this document's commit. **Corpus pin:** `redkiln-corpus` @ `61f590e51cbb00301c310e46aab615563a7787b8`
(`CORPUS_PIN`), read through the object store only (AC-464). **Instrument:** `spikes/sev/make-sheet.py` (the blind
sheets and their sealed keys) and `spikes/sev/score.py` (validation and every figure below); both stdlib-only, no
dispatch, no model call, no network. **Spike id:** S-SEV. **Rater:** the operator, alone. There is no dispatch and
no model rater: a model-filled sheet would repeat E3-SEV's forfeit, the same rig's model not being an independent
rater (§8b.6).
**Why it exists:** ADR-0080 decision 3 waits on *"a settled vocabulary and a destination for advisory findings"*.
ADR-0082 supplies the destination. The vocabulary does not exist: 39 shipped `expr` rows read `severity`,
`rk_types::artifact::findings::Finding` has no such field, and 188 of 188 F37 findings carry none. D21-42 11c settles
the vocabulary and T on a measurement, inside `s8b`; this is that measurement. D21-54 makes it a sequential sample of
whole dispatches.

| criterion | question | decides |
| --- | --- | --- |
| **A1** | is the three-answer vocabulary usable by its rater? | whether anything is settled at all |
| **A2** | is severity a function of the evidence class? | whether T over severity is the evidence predicate renamed |
| **A3** | does the "no BLOCKING finding" condition vary across dispatches? | whether T = `blocking` is a constant function (golden constraint 15) |
| report only | does A3's condition track root-cause correctness? | nothing (powerless) |

## 0. Rules

Everything in METHOD-P §0 applies where it can: a rate is reported with its Wilson 95% interval; nothing is inferred
from a single draw; a figure is reproducible from committed inputs or it is void. Four rules specific to S-SEV:

1. **Labels are produced blind, before any figure is seen.** The operator labels from the sheet alone. He does not
   open a sealed key file, does not run `make-sheet.py` or `score.py` in a way that prints a key, and does not read
   `results/p7.jsonl` or any F37 text that quotes a finding, until the last label of the pass is committed.
2. **The seeds are fixed here and read from here.** `make-sheet.py` parses both seed lines below; the script and this
   document cannot disagree about them.
3. **Every key is sealed before its sheet is labelled.** For each sheet, the sha256 that `make-sheet.py` prints for
   its key file is recorded in this document by a dated amendment, committed, before the first label of that sheet
   is written.
4. **After the labels commit, this document is append-only.** Every later change adds a dated amendment in the form
   `*Amendment N (YYYY-MM-DD, who):*` and deletes no line (Checkpoint 8b section (9) enforces it). From this commit
   on, amendments are appended in that form anyway; no registered line is edited in place, the placeholders below
   included.

## 1. Corpus and unit

- **Corpus:** F37's adversary findings, `git -C "$CORPUS_REPO" show "$CORPUS_SHA":results/p7.jsonl` — 297 records, of
  which **33** have `phase = "adversary"`, carrying **188** findings at `digest.findings[]` (an identical copy sits at
  `step.digest.findings`; the script refuses if they differ). Each finding's keys are `ac, claim, evidence, files,
  rationale, repro`, and `repro`'s are `command, expected_when_met, observed`. Evidence classes: `reproduced` 118,
  `mutation-survived` 29, `advisory` 22, `narrowed-pin` 19 [measured 2026-09-25 at the pin]. The script refuses a
  corpus that does not reproduce these counts.
- **Unit: the whole dispatch**, keyed `(run_key, step.attempt)`. `step.identity` is not unique (32 distinct over 33
  dispatches), and 33 dispatches share 31 `run_key`s: the one `mandatory` run
  (`issue-77-advance-commit-partial-stage`) carries three adversary attempts. `adversary.item` names 10 items;
  `issue-77-advance-commit-partial-stage` has six dispatches (three `recommended`, three `mandatory`), every other
  item three.
- **Root-cause correctness** is joined by `run_key` from that run's `emit` record, `scored.correct`. It is **per run,
  not per item**, so the three `mandatory` dispatches share one value.
- **Arm and rigor** are `arm` / `arm_label` on the adversary record.
- **E3's six findings are excluded** (D21-43 N18, standing under D21-54). They are not in `p7.jsonl`, and the
  operator has read a filled E3 sheet, so his annotation of them could not be blind. If he labels E3's preserved BLANK
  anyway, it is reported as **NOT BLIND** and E3-SEV-BLIND stays open.
- **A blinding limit this corpus imposes:** four of E3's six findings are on items that are also P7 items
  (`fix-advance-write-lock-gate-scope`, `issue-146-147-boundary-window`, `issue-63-advance-gate-diagnostic-truncated`,
  `issue-74-plan-briefs-dead-plan-anchor`). The operator has read those E3 findings, so an F37 finding on the same
  item may be recognisable by topic. The mask removes the item's name, not its subject matter.

## 2. Design — a sequential sample of whole dispatches (D21-54)

- **Stage 1** labels every finding of **one dispatch per item: 10 dispatches**. The draw: the ten items in sorted
  order; each item's dispatches in `(run_key, attempt)` order; `random.Random(seed).choice` once per item, in that
  order. Stage 1's findings are shuffled together (not grouped by dispatch) and each is given a random id from
  `random.Random("S-SEV/stage1/<seed>")`.

**Stage-1 seed:** `20260925`

- **Stage 1 decides** exactly when it shows variability on both A2 and A3, each stated as the negation of §8b.6's
  inadmissibility condition. §8b.6 defines A2's as *"zero discordant findings in both directions"*: zero
  execution-class findings labelled `ADVISORY` **and** zero other-class findings labelled `BLOCKING`. Its negation is
  **at least one discordant finding, in either direction** (D21-54: *"at least one discordant finding, as §8b.6
  defines it"*). A3's is *"`k = 0` or `k = 33`"*, whose negation over stage 1's ten dispatches is **`0 < k < 10`**.
  Each is decided over the corpus by containment: a discordant finding, or a dispatch with and one without a
  `BLOCKING` label, in the sample is one in the corpus.

**Continuation rule:** stage 1 decides if and only if A2 has at least one discordant finding (an execution-class finding labelled `ADVISORY`, or an other-class finding labelled `BLOCKING`) **and** A3 has `0 < k < 10`; otherwise the remaining 23 dispatches are labelled under the same sheet rules, their key sealed first, and every criterion is computed over all 188 findings and 33 dispatches. A stage-1 constancy result is never reported as constancy over the 188.

- The continuation's findings are shuffled and id'd from `random.Random("S-SEV/continuation/<Stage-1 seed>")`, with
  ids prefixed `C-` (stage 1's are `S-`, the re-label's `R-`). The prefix reveals the pass, nothing else.
- **The decision is read without the tables.** `score.py --decide-stage1` prints one line, `STAGE 1 DECIDES` or
  `CONTINUATION REQUIRED`, and no counts, so the re-label stays blind to the first pass's figures.
- **What "labelled n" means below:** stage 1's findings if stage 1 decides; all 188 otherwise.

## 3. Blinding

- `make-sheet.py` renders, per finding, `claim`, `rationale`, `files` and `repro` (`command`, `observed`,
  `expected_when_met`) under a random id from the recorded seed. Every content line is indented, so no finding text
  can forge the sheet's structure.
- It **masks** `evidence`, arm, rigor, item, dispatch and the run's root-cause correctness. It also does not render
  `ac`, which §8b.6 neither lists for rendering nor for masking; an AC id without its spec text informs nothing the
  sheet can show.
- The id → key map (`run_key`, attempt, finding index, evidence, arm, item, correctness) is written to a sealed file,
  `key-<mode>.SEALED.json`, beside the sheet. The script prints only counts and digests.

**Sealed key sha256:** ____ (recorded by dated amendment when the sheet is generated)

- **Stated limits.**
  - A `rationale` often reveals whether the reviewer ran something, so the evidence class is partly inferable. The
    mask removes the label, not the evidence.
  - The key is reproducible from the committed seed and the committed script: that is what makes it checkable, and it
    also means the seal is a commitment against later alteration, not a secret. Blindness rests on rule 1 and on the
    attestation above, and is reported as **attested-blind**, the weaker form METHOD-P amendment 8 names.
  - Topics may be recognisable (§1's E3 overlap).

## 4. The question

Verbatim from E3's sheet, so that the two are comparable: *"should this finding have blocked the item?"*. The answers
are `BLOCKING`, `ADVISORY` or `CANNOT-TELL`, one per finding, written after the finding's `answer:` line. The first
two are E3's definitions: `BLOCKING` = the reviewer was right to return `changes-requested` on this; `ADVISORY` =
true, but should not have blocked. `CANNOT-TELL` = the finding as shown does not let the rater say. The standard is
the reviewer card's, quoted on every sheet as E3's sheet quotes it:

> `agents/redkiln-review.md`, the card the reviewer was given:
>
> > `approved` — every acceptance criterion is genuinely satisfied by real,
> > reachable behaviour, **with a real test backing each** […]
> >
> > `changes-requested` — at least one criterion is **unmet, untested**, only
> > superficially met […]
>
> step 2: *"is there a test that would fail if the behaviour regressed?"* … *"hunt
> for … missing or **tautological tests**"*.

If the operator labels against his own standard instead, he says so in the record, because a disagreement then
measures the standard rather than the judgement.

## 5. The re-label

- **38 findings** are drawn from **those labelled in stage 1**: stage 1's findings in `(item-sorted dispatch, finding
  index)` order, `random.Random(<Re-label seed>).sample(…, 38)`; then shuffled and re-id'd from
  `random.Random("S-SEV/relabel/<Re-label seed>")`. The re-label sheet is labelled blind to the first pass: the
  operator does not open his filled stage-1 sheet while labelling it.

**Re-label seed:** `20260927`

- **If stage 1 carries fewer than 38 findings,** all of them are drawn, reshuffled and re-id'd, and κ is reported
  over that n, named. The 0.61 bar does not move.
- **It is drawn from stage 1 even when the continuation runs** (D21-54). The continuation's findings are never
  re-labelled.
- **Timing.** At least **48 hours** after the S-SEV labels commit (§8) — the corpus commit that adds the filled
  stage-1 sheet. `make-sheet.py relabel` refuses before then, reading that commit's time from the corpus's object
  store.

## 6. What counts, and the n each criterion needs — fixed before the data (golden constraint 10)

### 6.1 A1 — vocabulary usability

- **Why.** A threshold over a vocabulary its only rater cannot apply, or cannot apply the same way twice, settles
  nothing.
- **Design.** Two halves over the labelled n and the re-label pairs.
- **Metrics.** (i) `CANNOT-TELL` count over the labelled n, with its Wilson 95% interval. (ii) Intra-rater Cohen's κ,
  unweighted, over the three answers, on the re-label pairs (each re-labelled finding joined to its first-pass label
  by `(run_key, attempt, finding index)`), with p_o and p_e; an approximate 95% interval is reported beside it and
  decides nothing.
- **Decision criteria.** A1 **passes** if and only if `CANNOT-TELL` ≤ 10% of the labelled n **as a point rule**
  **and** κ ≥ **0.61**. 0.61 is the Landis–Koch "substantial" band's lower edge, a **convention**, chosen here before
  the data and said to be one. If either half fails, the vocabulary is **NOT SETTLED by this measurement**: the
  record says so under that fault name and the stage STOPS for the operator. A κ that is undefined (p_e = 1, one
  answer throughout both passes) has not been shown ≥ 0.61, and A1 fails.
- **Power.** The point rule is what decides. The Wilson interval **decides the bar by itself only at 0–1 `CANNOT-TELL`
  of about 57** (the expected stage-1 n: 0/57 → upper 0.063, 1/57 → 0.093, 2/57 → 0.119); at 188 it does so at
  0–10 (10/188 → upper 0.095). Between that and the point rule (5 of 57; 18 of 188) the bar is met on the point and
  **powerless on the interval**, and is reported so. κ over 38 pairs has an interval roughly ±0.2 wide at κ ≈ 0.7, so
  a κ near 0.61 passes or fails on the point and the record says the interval straddles the bar.
- **Precedence.** The κ half is final whenever it is computed. The `CANNOT-TELL` half is final only on the final
  labelled n: a stage-1 `CANNOT-TELL` failure that coincides with the continuation is re-read over 188.

### 6.2 A2 — severity versus evidence class

- **Why.** If the labels are a function of the evidence class, a severity threshold is ADR-0080 decision 2's
  forbidden predicate under another name.
- **Design.** The execution classes (`reproduced`, `mutation-survived`; 147 of 188) against the others (`advisory`,
  `narrowed-pin`; 41), tabulated against `BLOCKING` / `ADVISORY`; `CANNOT-TELL` is tabulated beside them and excluded
  from both discordance denominators.
- **Metrics.** Discordance 1: execution-class findings labelled `ADVISORY`, over execution-class findings labelled
  `BLOCKING` or `ADVISORY`. Discordance 2: other-class findings labelled `BLOCKING`, over other-class findings labelled
  `BLOCKING` or `ADVISORY`. Each with its Wilson 95% interval.
- **Decision criteria.** **Zero discordant findings in both directions** ⇒ severity is a deterministic function of
  class ⇒ T over severity is **inadmissible**. At least one discordant finding in either direction ⇒ A2 shows
  variability.
- **Report only.** Whether, among `BLOCKING` / `ADVISORY` labels, each class group is label-homogeneous. It
  differs from the registered criterion only when the labels invert the evidence mapping or are constant; it decides
  nothing.
- **Power.** A2 decides existence, not a rate. Over 188 it is a census of the corpus; over stage 1 it decides only in
  the variability direction (by containment). The discordance intervals are reported and quoted as rates nowhere.
- **Also, in every branch:** if the labels contradict ADR-0074's `narrowed-pin` row, the record **reports** it and
  does not move the row (ADR-0092 decision 1).

### 6.3 A3 — termination variability (golden constraint 15, measured)

- **Why.** F37 found the shipped verdict a constant function of the arm. A threshold T = `blocking` is usable only if
  "no `BLOCKING`-labelled finding" is true for some dispatches and false for others.
- **Design.** For each labelled dispatch: does it carry **no** `BLOCKING`-labelled finding? A dispatch whose findings
  are all `ADVISORY` or `CANNOT-TELL` counts as carrying none.
- **Metrics.** `k / n_D` with its Wilson 95% interval, n_D the labelled dispatches (10 or 33). Report only: `k` with
  `CANNOT-TELL` counted as `BLOCKING`.
- **Decision criteria.** `k = 0` or `k = n_D` reproduces F37's constant-function result, and **T = `blocking` is then
  inadmissible**. `0 < k < n_D` ⇒ A3 shows variability.
- **Power.** At n_D = 10 a constancy result decides nothing and **fires the continuation**; only non-constancy is
  decided, by containment. At n_D = 33 the criterion separates constancy from non-constancy over the corpus, which is
  all it is asked to decide; it cannot estimate a rate (0/33 → [0.000, 0.104]; 1/33 → [0.005, 0.153]), and the
  record does not quote `k / n_D` as one.

### 6.4 Report only, powerless: `k` against root-cause correctness

Per labelled dispatch, the 2×2 of (no `BLOCKING` label) × (the run's `scored.correct`), and the same per item.
**Powerless and so labelled:** correctness is per run and nearly constant per item (ADR-0080: seven items solved 3/3
by both arms, two missed 0/3), so across 10 items it can separate nothing. It decides nothing and no branch reads it.

### 6.5 Powerless bars, by n (golden constraint 10)

| bar | n = stage 1 (10 dispatches, ≈ 57 findings) | n = 188 / 33 |
| --- | --- | --- |
| A1 `CANNOT-TELL` ≤ 10% | point rule decides; interval decides only at 0–1 | point rule decides; interval decides at 0–10 |
| A1 κ ≥ 0.61 (38 pairs, or all of stage 1 if fewer) | point rule; interval ≈ ±0.2, powerless near the bar | the same 38 pairs |
| A2 zero discordance | decides variability only; constancy fires the continuation | census: decides both ways |
| A3 `k ∈ {0, n_D}` | decides non-constancy only; constancy fires the continuation | census: decides both ways; no rate |
| report-only correctness | powerless | powerless |

## 7. What the record fixes, by branch (as §8b.6 lists them)

- **A1 passes, and A2 and A3 show variability:** the vocabulary is `blocking | advisory`, and T = `blocking`.
  `threat-model`'s (A) edit in §8b.5 then binds it.
- **A1 passes, and A2 or A3 fails** (over 188 only; a stage-1 failure is the continuation, not this branch): the
  vocabulary is fixed, and T is *"no admissible severity threshold on this evidence; bullet 2 governs"* — a settled
  T, not a deferral. Because ADR-0080 d2 bullet 1 needs a T and this branch settles none, the record **supersedes
  ADR-0080 d2 bullet 1 in part** (a severity threshold, on this evidence). `threat-model`'s (A) then has nothing to
  bind, and §8b.5's STOP applies.
- **A1 fails:** STOP, under the fault name **NOT SETTLED by this measurement**.
- **In every branch in which the record is written,** it names the source and owner of the `severity` field in a
  paragraph that opens `**The severity field's source and owner.**`: the owner is `s8h` (D21-47), which adds `severity`
  to `rk_types` `Finding` typed by this record's vocabulary, with tests; the paragraph names the step that fills the
  field (the reviewer's output, `review::policy`'s table, or something else); the record does not add the field. It
  states what the 39 rows do until then, as the evaluator behaves: over an empty findings list the row is `Tri::True`
  and `severity` is never read; over a non-empty list whose findings carry no `severity` the lookup returns
  `EvalError::UnknownField("severity")`; no non-empty list is read as absent-means-true.
- **In every branch,** a contradiction with ADR-0074's `narrowed-pin` row is reported, not acted on.
- **The record** is a new ADR at the next free id at writing, `status: proposed`, `evidence: ["S-SEV", "F37"]`,
  ADR-0085's `### Option — ` headings and a **What this forbids** block, flipping only on the operator's dated ledger
  line (§8b.6). It makes **no claim about the review gate's calibration**; it cites ADR-0101 clause 5 by number and
  does not quote it.
- **Disclosed, not decided (D21-54):** S-SEV measures the operator's normative labels, not the reviewer's emitted
  `severity`, so it does not show that the shipped row varies at run time.

## 8. Artefacts and where they live

- **Before any corpus commit,** the working sheets and sealed keys live in **`D:/repos/redkiln-sev/`**, a plain
  directory outside both repositories, so the corpus checkout stays clean (AC-464 refuses a dirty one).
  `make-sheet.py` refuses an `--out` inside either repository.
- **The labels commit.** The filled stage-1 sheet, as **`results/sev-f37-labels.md`** (the name Checkpoint 8b section
  (9) reads), is committed to `redkiln-corpus` as a new commit on `main`, **without moving `CORPUS_PIN`** — O-9's
  precedent (DECISIONS-2026-09-17 O-9: a spike's raw output is retained in the corpus as a new commit named by sha,
  the pin unmoved), and AC-465 bars tracked spike results under `spikes/results/`. It is made from a checkout of
  `main` that is not the pinned checkout, which stays at the pin and clean. The record names that commit's full sha
  on one line:

  ``**S-SEV labels commit:** `<40 hex>` ``

  and never names the corpus checkout's HEAD. `score.py --decide-stage1` is run on the committed sheet only after
  this commit.
- **Later corpus commits**, each also on `main` and without moving the pin: the filled continuation sheet if it runs
  (`results/sev-f37-labels-continuation.md`), the filled re-label (`results/sev-f37-relabel.md`), every sealed key
  (`results/sev-f37-key-<mode>.SEALED.json`, byte-identical to the file whose sha256 an amendment here recorded), and
  the scored result (`results/sev-f37-result.txt`, `score.py`'s full output). The keys enter the corpus last, after
  the re-label, so no key is in a repository while a pass it maps is unlabelled.
- **Order.** Commit this file (attestation completed) → `make-sheet.py stage1 --out D:/repos/redkiln-sev` → amendment
  recording the stage-1 key's sha256 → label → labels commit (corpus) → `--decide-stage1` → if the continuation is
  required: `make-sheet.py continuation`, amendment, label, corpus commit → at least 48 h after the labels commit:
  `make-sheet.py relabel --labels-commit <sha>`, amendment, label → `score.py` over every sheet → corpus commit of
  the re-label, keys and result → the record.

## 9. Limits

- One rater, labelling his own standard's application; κ is intra-rater and measures consistency, not correctness.
- One graph, one stage, ten seeded items, Opus-only reviewer (METHOD-P §4.5). The sample is of dispatches within
  that, not of reviews in general.
- Blindness is attested, not verified (§3); `rationale` leaks the class in part; E3-overlapping topics may be
  recognised.
- Stage 1 is a sample of whole dispatches, one per item; findings within a dispatch are not independent, so no
  per-finding interval here is quoted as a population rate.
- S-SEV settles the labels' vocabulary and T; it does not show that a reviewer would emit those labels.

## Amendments

*Amendment 1 (2026-09-25, session executing s8b):* the stage-1 sheet was generated after this method was committed
(`bbf809c`), by `python spikes/sev/make-sheet.py stage1 --out D:/repos/redkiln-sev`: 56 findings over 10 dispatches.
**Stage-1 sealed key sha256:** `2a5dea380f2ecda763a19b6954aaf843427e0b6345ccacce1f1dc1d8d67423b5` (`key-stage1.SEALED.json`); the
blank sheet's normalized sha256 is `06a4450b092a9cd3362b7add6ddcf59e4c8ed712a1cfa9b26e9baf979edd2d0e`. Recorded before any
label exists; the registered `**Sealed key sha256:** ____` line above is not edited in place (§0).
