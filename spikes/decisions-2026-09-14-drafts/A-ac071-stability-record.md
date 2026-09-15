# DRAFT · AC-071's stability record, and the register amendment that names it

**Status: DRAFT. Nothing minted, nothing written to the corpus.** The record file below lands in
`D:/repos/redkiln-corpus` at the single `CORPUS_PIN` move, which §7b.11 precondition 7 reserves to the operator:
*"No agent and no session may write to `D:/repos/redkiln-corpus` or edit `CORPUS_PIN` before that act."*

Executes **decision D-3** of 2026-09-14.

**Revised 2026-09-15**, against the primary records rather than against this draft's own earlier text. Four of its
load-bearing claims were wrong and are corrected below: the `corpus_sha` **value** (`DECISIONS-2026-09-15 D15-1`),
the **absence** of the three registry digests `s7b-kernel.md`:294-296 requires, the **span** of the `F36-DRAFT.md`
citation this draft proposes as a verbatim checker (:98-:100, not :99), and the **`$9.70`** figure F36 corrects
inline at its own :3. The erratum this draft names throughout is **withdrawn as an instrument** by
`DECISIONS-2026-09-15 D15-2`; the obligation lands instead as a dated in-place register amendment.

🛑 **The corpus write is the operator's and nothing in this repository may perform it.** §7b.11 precondition 7:
*"No agent and no session may write to `D:/repos/redkiln-corpus` or edit `CORPUS_PIN` before that act."* This file
is a **draft in `redkiln-rs`**. It is the text the operator mints at that single irreversible act; minting is not
part of any session that authors it, and no revision of this file — including this one — has touched the corpus.

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

## 2. The record — path `results/p1-arm-l-stability.json`, fixed by D-3

**Why that path.** It is the sibling of AC-520's `results/p1-arm-d-panel-scored.json`, in the corpus's own
`p1-arm-l-*` family beside `p1-arm-l.jsonl`, `p1-arm-l-v2.jsonl`, `p1-arm-l-analysis.txt` and
`p1-arm-l-v2-analysis.txt` [measured 2026-09-14, `git -C $CORPUS_REPO ls-tree HEAD results/`]. It is **not**
AC-071's current `--stability-record spikes/results/p1/stability.json`, which is dead twice over: nothing has ever
written it, and `spikes/results/` moved to the corpus under ADR-0013 / AC-465. **The path is not proposed here — it
is fixed by `DECISIONS-2026-09-14 D-3`**, which names `results/p1-arm-l-stability.json` in terms; this section
records why that choice is the corpus's own shape, not a fresh proposal. The correction of the dead
`--stability-record` path lands as a **dated in-place amendment to AC-071 in the register**, not as an erratum:
`DECISIONS-2026-09-15 D15-2` withdraws **E-0071-1 as an instrument** while leaving the obligation standing.

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
  "corpus_sha": "47faec9f46b8d983c2c75f0794164e68415ae706",

  "registry_digests": {
    "catalog": "f89154d762a7",
    "facets_and_artifacts": "149a74a0d053",
    "packs": "6994b61bc64d"
  },

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
lint reads. **It is the MINIMUM (0.10), and the four are disclosed beside it.** This is not proposed here either:
`DECISIONS-2026-09-14 D-3` fixes it — *"scalar = the **minimum**, fail-closed"*. The fail-closed choice is the only
defensible one for a gate whose whole purpose is to refuse: a gate that passed on the best of four measurements
while three others failed would be the *"a lint that reads identity and skips the panel clause"* failure one level
over. On this data the choice changes nothing — all four are below 0.80 and the gate fails under any of them — but
it is fixed now so that a future record with mixed figures cannot be read generously.

### 2.1b What `corpus_sha` denotes here, and the three registry digests beside it

**`corpus_sha` is the pin the measurement was taken at, not the catalog pin at read time.** That is
`DECISIONS-2026-09-15 D15-1`, and for this record the value is **`47faec9f46b8d983c2c75f0794164e68415ae706`** —
the sha F36's own engine rows carry (`$CORPUS_REPO/results/p1-arm-l.jsonl`), the pin `spikes/F36-DRAFT.md`:3 names
(*"corpus pin `47faec9f`"*), and the value this tree already hard-codes at
`crates/rk-kernel/tests/it/facet_stability.rs`:37 as `CATALOG_CORPUS_PIN`. An earlier draft of this file carried
`27d97e9b…` — `CORPUS_PIN`'s **current** value, which §7b.11's own move changes — and because the record is
**never regenerated**, that wrong value would have been permanent.

**The three registry digests are not decoration; `s7b-kernel.md`:294-296 requires them.** It says the record is
transcribed from F36's frozen output *"together with the three registry digests it was measured under"*, and the
reason is in the same paragraph: re-running the analyser today reads a registry **Step 4a rewrote**
(`cat:fbc5d98ae488 fac+art:592fa138d68d`) and returns **20/20 = 100% on all four**, i.e. a record that would
UNBLOCK the gate. Without the digests the record cannot say which registry it belongs to. The values are F36's own
stamp, verified 2026-09-15 by reading `spikes/F36-DRAFT.md`:3: **`cat:f89154d762a7 fac+art:149a74a0d053
pk:6994b61bc64d`** (the same stamp `s7b-kernel.md`:284-285 quotes).

### 2.2 Three of the five clauses are satisfied, and the two that do not clear are why the gate stands

| clause | bar | this record | clears |
| --- | --- | --- | --- |
| identity | ≥ 0.80 | **0.10** (min of 0.25 / 0.15 / 0.10 / 0.10) | **NO** |
| n | ≥ 20 items | 20 | yes |
| draws | ≥ 10 per item | 10 | yes |
| arm-D panel | `"scored"` | scored by **F39** — 140 truth cells at pin `47faec9f`, panel-adjudicated under METHOD-P amendment 9, never human-panel | yes |
| corpus pinned | `rec.corpus_sha == <catalog corpus pin>` | `47faec9f…` — the pin the measurement was taken at (D15-1), which is **not** today's catalog pin | **NO** |

**Clause 5 does not clear, and that is fail-closed and correct.** An earlier draft of this table claimed it did, by
reading the clause as *"a corpus sha is present"* rather than as the equality AC-071 actually states. Under
`DECISIONS-2026-09-15 D15-1` the value is the pin the measurement was taken at, so the equality against the catalog
pin at read time does not hold. The gate is unchanged either way: **AC-071 already fails on clause 1**, identity
**0.10** against **0.80**. A record that made clause 5 pass while clause 1 failed would be the *"a lint that reads
identity and skips the panel clause"* failure one level over. The two refused values fail on the record: `27d97e9b…`
is the **current** pin, which the pin move changes, so a record carrying it fails clause 5 at the moment the act
that commits it completes; and the post-move sha does not exist when the transcription is made.

AC-071's own text says the same in prose: *"One clause of five is discharged, 2026-09-09; the gate is unchanged
… the **identity** clause measured 0.25 / 0.15 / 0.10 / 0.10 against 0.80 [measured, F36]."*

---

## 3. Provenance — the transcription's source range, so the copy is checkable

Every figure above is transcribed from `spikes/F36-DRAFT.md`, which is in this repository and therefore diffable:

- **:98-:100** — *"v1: **5/20** (`bug-standard`) and **3/20** (`story`). v2: **2/20** and **2/20**. Against an 80%
  line, in both instruments, on both packs"* — the four identity figures. **The sentence spans three lines and an
  earlier draft cited `:99` alone.** Measured 2026-09-15: `sed -n '99p'` returns only
  *"(`bug-standard`) and **3/20** (`story`). v2: **2/20** and **2/20**. Against an 80% line, in both instruments,
  on"* — `v1: **5/20**` is on **:98** and `both packs` on **:100**.
- **:1** (the finding's own headline) — *"an identical required-artifact set `R(stage, ctx)` across ten draws is
  reached on **5/20, 3/20, 2/20 and 2/20** items against an 80% line — the kill fires four times out of four"* —
  corroborates the four and gives `kills_fired = 4`.
- **:23-24** — the pre-registered table rows, `5/20 = 25%` and `3/20 = 15%`, each against `< 80%`, each **KILL
  FIRES** — gives `items = 20` and `draws_per_item = 10` (*"10 draws/item"*, :15).
- **:57-:58** — *"200 dispatches, **0 void**, $9.70"* for the v2 instrument — but **$9.70 is superseded by F36's
  own inline correction**: :3 reads *"$9.70 *[corrected 2026-09-11: $10.71]*"* and :58 carries the same correction
  beside the :57 figure. The v2 cost is **$10.71**, and it is the corrected pair that reconciles to the evidence
  brief's F36 row of **400 dispatches, 0 void, $21.37** — `$10.66 + $10.71 = $21.37`, where `$10.66 + $9.70` does
  not.
- **:3** — *"stamped `cat:f89154d762a7 fac+art:149a74a0d053 pk:6994b61bc64d`"* — the three registry digests, which
  `s7b-kernel.md`:294-296 requires the record to carry beside the four figures and which an earlier draft of this
  file omitted entirely.
- `panel_verdict` is **not** F36's — F36 §0 says arm D *"waits for the panel"* and its precision/recall were **NOT
  COMPUTED at F36**. It is F39's, and AC-071's own amended text carries it.

**A checker for the transcription is therefore possible and is recommended** — but it must be scoped to the
sentence, not to a line. Assert that each of the four `measurements` rows appears verbatim **within
`spikes/F36-DRAFT.md`:98-:100**, and that the three `registry_digests` values appear verbatim at **:3**. A checker
pinned to `:99` alone would fail on `5/20`, which is on `:98` — and because this draft proposed that checker as a
verbatim test, the citation error would have been **landed as a red test**. Scoped to the span it turns *"never
regenerated"* from a promise into a test, on the same warrant that makes `bless_the_committed_golden_log` an
`#[ignore]`d named act. **Line numbers are a convenience and may go stale; the quoted clause is the citation that
survives.**

---

## 4. The register amendment this record needs

§7b.11 precondition 2 requires *"its dated register amendment has landed naming its path and its producer."*
Proposed text, to be appended to **AC-071** in `references/2026-09-08-acceptance-criteria.md`:

> ***Amended 2026-09-15 — the record's path, its producer and what `corpus_sha` denotes, which this criterion
> named and did not have.*** The scored P1 stability record this criterion requires is
> **`results/p1-arm-l-stability.json`**, resolved **corpus-relative through `CORPUS_PIN`** and never through a
> working tree (AC-464), carrying `modal_required_set_identity`, `n_items`, `draws_per_item`, `panel_verdict` and
> `corpus_sha` — the field names `rk-kernel::facet_stability` already reads — **together with the three registry
> digests it was measured under** (`s7b-kernel.md`:294-296): `cat:f89154d762a7 fac+art:149a74a0d053
> pk:6994b61bc64d`. **Its producer is a transcription, not a script**: the four identity figures are copied from
> **F36**'s frozen output (`spikes/F36-DRAFT.md`:98-:100) and the record is **never regenerated**, because
> re-running P1 arm L produces different draws — F36 §3.3 measured untouched anchors drifting up to **−0.168**
> between its two instruments, and re-running the analyser today reads the registry Step 4a rewrote and returns
> **20/20**, a number that would unblock the gate — so a regenerated record would be a new and unregistered
> measurement rather than F36's result. `modal_required_set_identity` is the **minimum** of the four measurements;
> the four are carried beside it in `measurements`. **For a transcribed record, `corpus_sha` denotes the pin the
> measurement was taken at, not the catalog pin at read time** — here
> `47faec9f46b8d983c2c75f0794164e68415ae706` (`DECISIONS-2026-09-15 D15-1`). Clause 5 therefore does **not** clear
> for this record, which is fail-closed and correct: this criterion already fails on its identity clause at 0.10
> against 0.80. *(This criterion's previous `--stability-record spikes/results/p1/stability.json` is dead on both
> halves — nothing ever wrote it, and `spikes/results/` moved to the corpus under ADR-0013 / AC-465. **That
> correction lands here, in place and dated, not as an erratum**: `DECISIONS-2026-09-15 D15-2` withdraws
> **E-0071-1** as an instrument while leaving the obligation standing, because a criterion in an amendable register
> is not a frozen record and `ERRATA.md`:273-276 concedes the route in its own words.)*

**Form.** The italic dated run-in is the register's own house form for an in-place amendment; the precedent for a
correction that must actually **cut** something is **AC-204** (:3266-3281), which rewrote the THEN and the
instrument in place and then appended a dated Correction paragraph. Nothing here cuts a clause, so the run-in
alone is the right instrument.

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
- **Still owed either way, and in a different instrument than this draft first said.** Both obligations stand;
  both are **dated in-place amendments**, not errata (`DECISIONS-2026-09-15 D15-2`). (i) The dead
  `--stability-record spikes/results/p1/stability.json` path — the correction rides in §4's amendment above, and
  **E-0071-1 is withdrawn as an instrument** (its id also collided with the accepted **ADR-0071**, which is what
  forced the rule). (ii) AC-071's THEN and `00-preamble.md`'s golden constraint 1 still call ADR-0079 `proposed`
  where ADR-0090 superseded it at Step 4.4 — **E-0079-2 is likewise withdrawn as an instrument**, and on its own
  terms it was always the wrong one: ADR-0079's frontmatter already reads `status: superseded` and ADR-0090:108-110
  says its body is deliberately untouched, so **there is no frozen line for an erratum to correct**. The stale text
  lives in the register, the SM register, the design and the runbook, each amendable in place.
- **Nothing in this pass is written to `references/adr/ERRATA.md`, and its convention header is unchanged**
  (`DECISIONS-2026-09-15 D15-2`).
