# OPERATOR · The three acts that unblock Checkpoint 7b — mint, move, check

**What this is.** A derived, ordered checklist for the one session that performs the `CORPUS_PIN` move. **It
decides nothing.** `runbook/phase-1/s7b-kernel.md` §7b.11 is the record; where this file disagrees with it, **this
file is wrong**. It exists because the move happens **exactly once** (`s7b` §7b.11 precondition 6 and the order
paragraph), so the cost of discovering a defect mid-sequence is not symmetric with the cost of reading ahead.

**Prepared 2026-09-15 at `a639eb0`.** Every state claim below was measured on that tree and is re-checkable with
the command beside it. **Nothing in this repository may perform step B or step C** — `s7b` §7b.11 precondition 7:
*"The operator, not an agent, performs the corpus write and the pin edit."*

---

## 0. Where the seven preconditions actually stand

Run this first. It is read-only and it answers the whole gate.

```bash
cd D:/repos/redkiln-rs
CORPUS_REPO=$(grep '^CORPUS_REPO=' CORPUS_PIN | cut -d= -f2-)
CORPUS_SHA=$(grep '^CORPUS_SHA='  CORPUS_PIN | cut -d= -f2-)

# p1 — Slice 3 green, and the logs produced by that run
cargo build --workspace || { echo "FAIL p1: build"; exit 1; }
out=$(cargo test -p rk-kernel --features corpus -- golden_log_replay 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL p1: golden_log_replay rc=$rc"; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\bgolden_log_replay\b.* \.\.\. ok$' || { echo "FAIL p1: no such test ran"; exit 1; }
echo "p1 OK"

# p2 — the stability record: SECOND half satisfied 2026-09-15, FIRST half is what you are about to do
grep -q 'results/p1-arm-l-stability.json' references/2026-09-08-acceptance-criteria.md \
  && echo "p2 amendment: LANDED" || echo "FAIL p2: the dated register amendment is absent"
test -s "$CORPUS_REPO/results/p1-arm-l-stability.json" \
  && echo "p2 record: PRESENT" || echo "p2 record: ABSENT  <-- THIS IS THE ONLY THING BLOCKING THE MOVE"

# p3 — the layout is in the section, at v2
grep -q 'rk-kernel/golden-log/v2' runbook/phase-1/s7b-kernel.md && echo "p3 OK" || echo "FAIL p3"

# p4 — both trees clean, corpus HEAD == pin
[ -z "$(git status --porcelain)" ] && echo "p4 rs clean" || echo "FAIL p4: redkiln-rs dirty"
[ -z "$(git -C "$CORPUS_REPO" status --porcelain)" ] && echo "p4 corpus clean" || echo "FAIL p4: corpus dirty"
h=$(git -C "$CORPUS_REPO" rev-parse HEAD)
[ "$h" = "$CORPUS_SHA" ] && echo "p4 pin matches" || echo "FAIL p4: HEAD $h != pin $CORPUS_SHA"

# p5 — the THREE STAGED RECORDS are not tracked here (AC-465). Scoped to the staged records, NOT to
#      spikes/results/ as a whole: see the note below, which is why this check reads the way it does.
n=$(git ls-files spikes/results/ \
      | grep -cE '(p1-arm-d-panel-scored\.json|p1-arm-l-stability\.json)|^spikes/results/golden-logs/')
[ "$n" -eq 0 ] && echo "p5 OK" || { echo "FAIL p5: $n staged record(s) tracked under spikes/results/"; git ls-files spikes/results/; }

# p6 — the backup bundle exists NOW; it is RE-CUT during step B, after the commit and before the pin edit
test -f "$(grep '^CORPUS_BACKUP_BUNDLE=' CORPUS_PIN | cut -d= -f2-)" && echo "p6 current bundle present" || echo "FAIL p6"
```

**Measured 2026-09-15 at `a639eb0`: p1, p3, p4, p5, p6 pass; p2's amendment half passes and its record half does
not.** Precondition 2 is therefore **FALSE**, and it is false **on the corpus write alone** — which is step B
below, and is yours. That is the whole gate.

> 🛑 **p5's check is narrowed deliberately, and the first draft of it was a guard scoped wrong.** Written as
> `git ls-files spikes/results/ | wc -l` → 0 it **FAILS on this tree**, on one file:
> `spikes/results/instrument-checks/RUNBOOK-instrument.txt` — which `runbook/phase-0/s1-commit-spike-output.md`
> **`git add`s by name** at Step 1.3, and which is the instrument version record, not a corpus-shaped result.
> Precondition 5's own text scopes it: *"the **staged records** live in a scratch tree, never under
> `spikes/results/`"*. So the broad form refuses valid work — **golden constraint 9 / Halt 7, the guard too wide**
> — and it was caught only by running the block rather than reading it (golden constraint 4). The narrowed check
> above asserts what the precondition actually says. **If it ever fires, read what it names before you delete
> anything.**
>
> **p4 will report `redkiln-rs dirty` while this file is uncommitted.** That is the check working, not a defect.

**Precondition 7 is not a checkbox; it is the reason this file stops at step A.**

---

## A. Stage the three artifacts — into a scratch tree, never the corpus

`s7b` §7b.11's order paragraph: *"produce all three artifacts into a **staging** tree, never into the corpus;
verify the staged artifacts **before** any copy."* Precondition 5 (AC-465) is why the staging tree must not be
`spikes/results/`.

```bash
STAGE=C:/rk-pin-move-2026-09-15        # NOT under D:/repos/redkiln-rs, NOT under the corpus
mkdir -p "$STAGE/results" "$STAGE/golden-logs"
```

### A1 · AC-520's scored precision record

```bash
python spikes/p1/score-arm-d-panel.py --out "$STAGE/results/p1-arm-d-panel-scored.json"
```

Verify before copying — the order paragraph names what it must carry: *"the pin, 140 truth cells and a per-facet
interval-plus-positives pair."*

### A2 · AC-071's stability record — **a transcription, never regenerated**

`s7b` §7b.2 forbids regenerating it: re-running P1 arm L produces different draws (F36 §3.3 measured untouched
anchors drifting up to −0.168 between its two instruments), so a regenerated file would be a new and unregistered
measurement, not F36's result. **Write this file by hand** to `$STAGE/results/p1-arm-l-stability.json`. The body
below is the one `spikes/decisions-2026-09-14-drafts/A-ac071-stability-record.md` carries, corrected on 2026-09-15:

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

**Three things about it that are decisions of record, not choices to re-take at the keyboard:**

- **`corpus_sha` is `47faec9f…`, the pin the MEASUREMENT was taken at** — not today's catalog pin
  (`DECISIONS-2026-09-15 D15-1`). It is the value F36's own engine rows carry and the value
  `crates/rk-kernel/tests/it/facet_stability.rs` already hard-codes as `CATALOG_CORPUS_PIN`.
- **AC-071 clause 5 therefore does NOT clear**, because that is not the catalog pin. **This is correct and it is
  fail-closed.** AC-071 already fails on clause 1 — identity **0.10** against the **0.80** line — and the gate is
  unchanged. A record that made clause 5 pass while clause 1 failed would be golden constraint 9's guard-scoped-
  wrong, one level up. 🛑 **If you find yourself editing this value to make a lint go green, stop: that is Halt 7.**
- **`modal_required_set_identity` is the MINIMUM of the four figures**, with all four disclosed in `measurements`
  (`DECISIONS-2026-09-14 D-3`, *"scalar = the minimum, fail-closed"*).

Verify before copying — the order paragraph: the record carries **F36's four figures and not `1.00`**.

### A3 · The golden event logs

Produced by Slice 3's replay test; the committed fixture is at
`crates/rk-kernel/tests/fixtures/logs/implement-bf1d8cec3b22d288/` (the post-`v2` run id — D-4's bump reached the
derivation, which was the assertion that it had). Copy the tree into `$STAGE/golden-logs/`.

Verify before copying: `golden-logs/` is **non-empty**, and the layout matches `s7b` §7b.11's block — `run.json`
with its eight members, `inputs/` with its nine, `events.jsonl` LF-terminated.

---

## B. 🛑 YOURS ALONE — one corpus commit, re-cut the bundle, one pin edit

Precondition 7. Nothing in `D:/repos/redkiln-rs` may do any of this.

**Rehearse the rollback BEFORE the pin edit** — the order paragraph requires it, and it is one line: restore the
old `CORPUS_SHA` (`27d97e9b164d09dc755fe38e353de922cb97c482`, `PINNED_AT=2026-09-13`). The corpus commit itself is
**additive and need not be reverted**; the pin is the switch.

1. **One corpus commit** carrying all three staged artifacts. One commit, not three.
2. **Re-cut `CORPUS_BACKUP_BUNDLE`** — *after* the corpus commit and *before* the pin edit (precondition 6). A pin
   naming a bundle that predates the commit is a stale backup. Update the `CORPUS_BACKUP_BUNDLE` path/date in
   `CORPUS_PIN` in the same edit as step 3.
3. **One `CORPUS_PIN` edit.** Anchor-assert before writing (golden constraint 7) and re-assert 40-hex after:

```bash
NEW=$(git -C D:/repos/redkiln-corpus rev-parse HEAD)
grep -q "^CORPUS_SHA=27d97e9b164d09dc755fe38e353de922cb97c482$" CORPUS_PIN \
  || { echo "FAIL: the BEFORE anchor is absent — you are editing the wrong file or it already moved"; exit 1; }
# ...make the edit (CORPUS_SHA, PINNED_AT, CORPUS_BACKUP_BUNDLE)...
grep -qE "^CORPUS_SHA=[0-9a-f]{40}$" CORPUS_PIN || { echo "FAIL: CORPUS_SHA is not 40-hex after the edit"; exit 1; }
grep -q "^CORPUS_SHA=$NEW$" CORPUS_PIN || { echo "FAIL: the pin does not name the new corpus HEAD"; exit 1; }
```

4. **Re-run EVERY corpus-bound assertion at the new pin** — the order paragraph says *"every"*, and it means the
   whole of Checkpoint 7b, not the two lines that were deferred.
5. **Then commit `CORPUS_PIN`.**

---

## C. Run Checkpoint 7b

`s7b` §7b.14, top to bottom, exit 0 required. Record the result in `PROGRESS.md` §1 and §6.

**What the 2026-09-15 dry run already tells you, so a red line is diagnosable rather than a surprise**
(`PROGRESS.md` §6, row *Checkpoint 7b — DRY RUN, not a pass*):

- **163 of the block's 165 executable statements already pass on the pre-move tree.** The block was run whole: it
  exits 1 at exactly one statement, `FAIL: the golden event logs are not in the pinned corpus`, and everything
  before it — the six gates, `cargo deny`, the `--features corpus` suite, the AC-001 lint-level source check, the
  corpus preconditions — passed.
- **The 57 statements that sit behind that gate were run too**, with the two corpus guards deferred in a scratch
  copy: exit 0, 0 FAIL. Completion was **proven by an end-of-script marker**, not inferred from silence — they are
  silent-on-success guards, and absence of output is Halt 8 shape (g), which is not evidence of anything.
- **So if Checkpoint 7b goes red anywhere other than at a corpus-bound line, the cause is something the move
  itself introduced** — the staged artifacts, the new pin, or the bundle — and not a pre-existing defect in the
  block. That is the whole point of having run it first.

### 🛑 One defect the dry run found, and it is owed either side of the move

`s7b` §7b.14's by-path loop asserts **6** `trybuild` case files by path while **13** exist in
`crates/rk-kernel/tests/ui/` [measured 2026-09-15]. Not asserted: `ac007_divergence_match_has_no_wildcard`,
`ac010_dispatch_without_key`, `ac020_counters_never_share`, `ac025_exhaustion_is_not_a_pass`,
`ac028_steperror_wildcard`, `ac032_finding_is_not_a_steperror`, `ac487_port_identity_is_not_askable`. Any of the
seven could vanish and the harness would still report `ok` — **the same any-case-passes filter `s8` check (5)
carried until 2026-09-15**, still half-present in Checkpoint 7b itself. **Owed: extend the loop to all thirteen,
or name the seven and say why they are excluded.** It does not block the move; it does mean Checkpoint 7b's pass
attests less than it appears to until it is fixed.

---

## What this does NOT unblock

Checkpoint 7b's pass says **selection and the lints exist and fail closed**. It does **not** say the gates do —
that is **Checkpoint 7b-ii**, and **M3 needs both** (`INDEX.md`'s milestone table). **ADR-0101 clause 5** forbids
claiming the review gate is guarded against false approval until **S-SUBTLE-CB** runs, whatever either checkpoint
says.
