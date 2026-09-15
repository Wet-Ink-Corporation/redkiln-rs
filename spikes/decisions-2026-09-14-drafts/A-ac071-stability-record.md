# DRAFT · AC-071's stability record, and the register amendment that names it

**Status: DRAFT. Nothing minted, nothing written to the corpus.** The record file below lands in
`D:/repos/redkiln-corpus` at the single `CORPUS_PIN` move, which §7b.11 precondition 7 reserves to the operator:
*"No agent and no session may write to `D:/repos/redkiln-corpus` or edit `CORPUS_PIN` before that act."*

Executes **decision D-3** of 2026-09-14.

---

## 1. What this record is, and what it is not

It is the artifact §7b.11 precondition 2 requires: *"AC-071's stability record exists, was **transcribed from
F36**, and its dated register amendment has landed naming its path and its producer."*

**It records a gate that does NOT clear.** Every figure in it is below its bar on the clause that matters. That is
the point: AC-071 is *"the machine-checkable statement of the block"*, and this record is the evidence the block
stands on. A reader who expects an unblocking record has the sign backwards.

**It is a transcription, and it is never regenerated.** §7b.2 fixes this: *"transcribed from F36's frozen output
and never regenerated."* Re-running P1 arm L would produce different draws — F36's own §3.3 measured untouched
anchors drifting by up to **−0.168** between two instruments — so a regenerated record would not be F36's result,
it would be a new and unregistered measurement. The producer is therefore a **transcription with a named source
range**, not a script, and §2's provenance block is what makes it checkable.

---

## 2. The record — proposed path `results/p1-arm-l-stability.json`

**Why that path.** It is the sibling of AC-520's `results/p1-arm-d-panel-scored.json`, in the corpus's own
`p1-arm-l-*` family beside `p1-arm-l.jsonl`, `p1-arm-l-v2.jsonl`, `p1-arm-l-analysis.txt` and
`p1-arm-l-v2-analysis.txt` [measured 2026-09-14, `git -C $CORPUS_REPO ls-tree HEAD results/`]. It is **not**
AC-071's current `--stability-record spikes/results/p1/stability.json`, which is dead twice over: nothing has ever
written it, and `spikes/results/` moved to the corpus under ADR-0013 / AC-465. **ERRATA E-0071-1 is owed for that
path** and is named here rather than written.

**The field names are not invented — they are the consumer's.** `crates/rk-kernel/src/facet_stability.rs` already
reads `modal_required_set_identity`, `n_items`, `draws_per_item`, `panel_verdict` and `corpus_sha`, with the bars
`IDENTITY_BAR`, `N_ITEMS_BAR` and `DRAWS_PER_ITEM_BAR` and the only satisfying verdict `"scored"`.

```json
{
  "schema": "redkiln/p1-stability/v1",
  "facet_axis": "modal_required_set",
  "source_finding": "F36",
  "transcribed_from": "spikes/F36-DRAFT.md",
  "transcribed_at": "2026-09-14",
  "regenerated": false,

  "modal_required_set_identity": 0.10,
  "n_items": 20,
  "draws_per_item": 10,
  "panel_verdict": "scored",
  "corpus_sha": "27d97e9b164d09dc755fe38e353de922cb97c482",

  "identity_bar": 0.80,
  "clears_identity_bar": false,

  "measurements": [
    { "instrument": "v1", "pack": "bug-standard", "identical_modal_r": 5, "items": 20, "identity": 0.25 },
    { "instrument": "v1", "pack": "story",        "identical_modal_r": 3, "items": 20, "identity": 0.15 },
    { "instrument": "v2", "pack": "bug-standard", "identical_modal_r": 2, "items": 20, "identity": 0.10 },
    { "instrument": "v2", "pack": "story",        "identical_modal_r": 2, "items": 20, "identity": 0.10 }
  ],

  "kills_fired": 4,
  "kills_possible": 4
}
```

### 2.1 The one reading this record takes, stated rather than buried

**`modal_required_set_identity` is a scalar and F36 produced four figures**, so the record must say which one the
lint reads. **It is the MINIMUM (0.10), and the four are disclosed beside it.** The fail-closed choice is the only
defensible one for a gate whose whole purpose is to refuse: a gate that passed on the best of four measurements
while three others failed would be the *"a lint that reads identity and skips the panel clause"* failure one level
over. On this data the choice changes nothing — all four are below 0.80 and the gate fails under any of them — but
it is fixed now so that a future record with mixed figures cannot be read generously.

### 2.2 Four of the five clauses are satisfied, and that is why the fifth matters

| clause | bar | this record | clears |
| --- | --- | --- | --- |
| identity | ≥ 0.80 | **0.10** (min of 0.25 / 0.15 / 0.10 / 0.10) | **NO** |
| n | ≥ 20 items | 20 | yes |
| draws | ≥ 10 per item | 10 | yes |
| arm-D panel | `"scored"` | scored by **F39** — 140 truth cells at pin `47faec9f`, panel-adjudicated under METHOD-P amendment 9, never human-panel | yes |
| corpus pinned | present | `27d97e9b…` | yes |

AC-071's own text says the same in prose: *"One clause of five is discharged, 2026-09-09; the gate is unchanged
… the **identity** clause measured 0.25 / 0.15 / 0.10 / 0.10 against 0.80 [measured, F36]."*

---

## 3. Provenance — the transcription's source range, so the copy is checkable

Every figure above is transcribed from `spikes/F36-DRAFT.md`, which is in this repository and therefore diffable:

- **:99** — *"v1: **5/20** (`bug-standard`) and **3/20** (`story`). v2: **2/20** and **2/20**. Against an 80% line,
  in both instruments, on both packs"* — the four identity figures.
- **:1** (the finding's own headline) — *"an identical required-artifact set `R(stage, ctx)` across ten draws is
  reached on **5/20, 3/20, 2/20 and 2/20** items against an 80% line — the kill fires four times out of four"* —
  corroborates the four and gives `kills_fired = 4`.
- **:23-24** — the pre-registered table rows, `5/20 = 25%` and `3/20 = 15%`, each against `< 80%`, each **KILL
  FIRES** — gives `items = 20` and `draws_per_item = 10` (*"10 draws/item"*, :15).
- **:57** — *"200 dispatches, **0 void**, $9.70"* for the v2 instrument; the evidence brief's F36 row gives the
  pair at **400 dispatches, 0 void, $21.37**.
- `panel_verdict` is **not** F36's — F36 §0 says arm D *"waits for the panel"* and its precision/recall were **NOT
  COMPUTED at F36**. It is F39's, and AC-071's own amended text carries it.

**A checker for the transcription is therefore possible and is recommended**: assert that each of the four
`measurements` rows appears verbatim in `spikes/F36-DRAFT.md`:99. That turns *"never regenerated"* from a promise
into a test, on the same warrant that makes `bless_the_committed_golden_log` an `#[ignore]`d named act.

---

## 4. The register amendment this record needs

§7b.11 precondition 2 requires *"its dated register amendment has landed naming its path and its producer."*
Proposed text, to be appended to **AC-071** in `references/2026-09-08-acceptance-criteria.md`:

> ***Amended 2026-09-14 — the record's path and producer, which this criterion named and did not have.*** The
> scored P1 stability record this criterion requires is **`results/p1-arm-l-stability.json`**, resolved
> **corpus-relative through `CORPUS_PIN`** and never through a working tree (AC-464), carrying
> `modal_required_set_identity`, `n_items`, `draws_per_item`, `panel_verdict` and `corpus_sha` — the field names
> `rk-kernel::facet_stability` already reads. **Its producer is a transcription, not a script**: the four identity
> figures are copied from **F36**'s frozen output (`spikes/F36-DRAFT.md`:99) and the record is **never
> regenerated**, because re-running P1 arm L produces different draws — F36 §3.3 measured untouched anchors
> drifting up to **−0.168** between its two instruments — so a regenerated record would be a new and unregistered
> measurement rather than F36's result. `modal_required_set_identity` is the **minimum** of the four measurements;
> the four are carried beside it in `measurements`. *(This criterion's previous
> `--stability-record spikes/results/p1/stability.json` is dead on both halves — nothing ever wrote it, and
> `spikes/results/` moved to the corpus under ADR-0013 / AC-465. **ERRATA E-0071-1** carries that correction.)*

---

## 5. What this does NOT do, and one finding recorded with it

- **It does not unblock anything.** The identity clause fails at 0.10 against 0.80, so AC-071 still fails closed
  and golden constraint 1 still stands. ADR-0079 remains `proposed`; ADR-0090 superseded it at Step 4.4 and that
  is unaffected.
- **It is required by §7b.11's precondition list, not by AC-071's own THEN.** Measured 2026-09-14: the registry
  carries **zero `llm` facets** (9 human, 6 derived, 4 kernel, 1 declared; Step 4a took llm 7 → 0 at `db6b55c`),
  and AC-071's THEN requires a record *"for that facet"* — so no facet requires one. And **Checkpoint 7b reads the
  record nowhere**: its only two corpus file assertions are `golden-logs/` non-empty and
  `results/p1-arm-d-panel-scored.json` present. Under golden constraint 9 / Halt 7 the precondition is a guard
  that is **too wide**. D-3 chose to **satisfy it rather than narrow it**, because the pin move happens exactly
  once (`PROGRESS`:103) — but the finding is recorded here so a later reader is not left to rediscover it.
- **Still owed either way:** ERRATA **E-0071-1** (the dead `--stability-record` path) and **E-0079-2** (AC-071's
  THEN and `00-preamble.md`'s golden constraint 1 still call ADR-0079 `proposed` where ADR-0090 superseded it).
