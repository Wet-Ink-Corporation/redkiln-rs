## F43 — S-BIN RESULT: without a text diff the reviewer detects both binary seeds in both passes (2/2, 2/2) from the assets' own bytes, but blocks 1/3 then 2/3 clean items, so the registered rule decides pass 1 (branch 1) and not pass 2 (`undecided-fp`), and the `Binary` variant takes the human-gate route

**Status: proposed** (written 2026-10-14 by the s8i build, an AFK session, package S8I-C1-F43). Nothing here is
accepted, and nothing here flips a record. The `Binary` record (ADR-0199) is a separate record.

**Established by:** 16 Opus dispatches, arm `b-typed-sbin`, model `claude-opus-5-5` (`--model opus`, effort
`xhigh`, `observed_model` on every record), `claude` **2.1.289** on every record and both launch records, runner
`be2b629` (`runner_sha256` `00403595…007f`), `apiKeySource` `none` on every record.
- **Scored:** ten records, pass 1 (B2, 2026-10-14 00:16–01:47 UTC) and pass 2 (B3, 2026-10-14 05:22–06:52 UTC), one
  per `(pass, item)`, **10 valid, 0 void**, in `results/s-bin.jsonl` at corpus `835b122` (B2 `f5ac506`,
  missing-fields `326f4e8`, B3 `835b122`). The final `corpus_commit`, the one that also holds
  `results/s-bin/audit.txt`, is the operator's corpus write and is **owed** (the pins block below marks it).
- **Discarded, not scored:** pass 1's first set, six records of 2026-10-07 (UTC), moved to
  `results/s-bin/superseded/pass1-d21153/s-bin.jsonl` at the §8i.6 stop (D21-153 item 1, below).
- **Cost: $44.595018 of the $90.00 cap**, which counts the discarded set (D21-153 item 8: *"The $90 cap and the
  18-Opus dispatch cap stand, counting the discarded set."*): B2 $11.780753, B3 $11.111433, the discarded set
  $16.074148, B1's six Opus capture sessions $5.54 (procedure log), and two haiku probes $0.0886845
  ($0.021026 and the cwd probe $0.0676585). Opus dispatches: **16 of 18** (10 scored, 6 discarded). No B4: no pass
  fired branch 2.
- Pre-registered in `METHOD-S.md` §3 (2026-09-05, `c2c3fc8`), the 2026-09-09 entry, and the s8i pre-dispatch amendment with
  its extensions (§7), every one before the first Opus dispatch of the scored set; the review rendering, the fixture
  and the path rule were re-registered under D21-153 before B2. The audit is `results/audit-s.py bin` over the ten
  records, `missing-fields.jsonl` (empty) and `oot-shadow-rulings.jsonl` (ten `clear` rulings).

### Pass 1 (B2)

| item | kind | policy verdict (strict) | blocking findings by class | detected | cited fields | capture | wall s | cost |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | clean | approve | none (5 advisory) | — | assets, captures, tests.report_ref | yes | 1014 | $3.6421 |
| 2 | clean | approve | none (3 advisory) | — | assets | none (D21-149) | 856 | $2.4254 |
| 3 | clean | **blocks** | narrowed-pin 1 | — | assets, tests.report_ref | none (D21-149) | 598 | $2.6618 |
| 4 | seed | **blocks** | reproduced 3 | **yes** | assets, captures, tests.report_ref | yes | 463 | $1.3089 |
| 5 | seed (R) | **blocks** | reproduced 2, narrowed-pin 1 | **yes** | assets, build.log_ref, captures, tests.report_ref, text_diff | yes | 851 | $1.7426 |

**Strong detection 2/2. Approval on clean items 2/3, so false positives 1/3** (item 3). The reviewer ran the
automation suite itself on 0/5.

### Pass 2 (B3)

| item | kind | policy verdict (strict) | blocking findings by class | detected | cited fields | capture | wall s | cost |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | clean | **blocks** | narrowed-pin 1 | — | assets, captures | yes | 1113 | $4.7635 |
| 2 | clean | approve | none (1 advisory) | — | assets | none (D21-149) | 512 | $1.5963 |
| 3 | clean | **blocks** | narrowed-pin 1 | — | assets | none (D21-149) | 841 | $2.1019 |
| 4 | seed | **blocks** | reproduced 2 | **yes** | assets, build.log_ref, captures, tests.report_ref | yes | 353 | $1.2978 |
| 5 | seed (R) | **blocks** | reproduced 1, narrowed-pin 1 | **yes** | assets, build.log_ref, captures, tests.report_ref | yes | 345 | $1.3520 |

**Strong detection 2/2. Approval on clean items 1/3, so false positives 2/3** (items 1 and 3). The reviewer ran
the automation suite itself on 0/5.

**Cited fields over the ten records** (mechanical, the records' `cited_fields`; the input the `Binary` field set
line must argue from, METHOD-S §3.3): `assets` 10/10, `captures` 6/10 (every record whose item carries a capture
cited it: items 1, 4 and 5 in both passes), `tests.report_ref` 6/10, `build.log_ref` 3/10, `text_diff` 1/10
(pass 1 item 5, citing that it is empty). *(The build's brief said captures 5; the records read 6, counted by
`S8I-C1-F43` from `835b122:results/s-bin.jsonl`. The audit's per-pass lines agree: captures 3 and 3.)*

### METHOD-S §3.4, evaluated per pass — never pooled

- **Pass 1: branch 1** (*"2/2 detection and ≤1/3 false positive: `ChangeSummary` as drafted is sufficient; ship
  it"*), decided by the registered rule: 2/2 and 1/3. Under D21-152 item 2 it reads as **sufficient given the
  report**, not *"ChangeSummary alone sufficient"* (below).
- **Pass 2: `undecided-fp` — not decided by the registered rule.** Detection is 2/2, but false positives are 2/3,
  over branch 1's ≤1/3. No seed was missed, so branch 2 (*"A seed missed and its finding would have needed a field
  the summary lacks"*) and branch 3 (*"Both seeds missed …"*) cannot fire. This is the outcome D21-43 N21 registered
  before dispatch: *"If either occurs, the `Binary` variant takes the human-gate route."*
- `results/audit-s.py bin` reads `rule_outcome_pass1=branch1` and `rule_outcome_pass2=undecided-fp`, re-run by this
  draft's pins generator from the audit's own blob over the pinned blobs. `missing-fields.jsonl` is empty (0 bytes):
  neither pass missed a seed.
- **The passes disagree** (branch 1, then undecided). §8i.8's map takes the more conservative route for a
  disagreement and the human gate for `undecided-fp`; both give **route: human gate**. The disagreement is a
  **stability finding**: clean item 1 flipped from approve (pass 1) to a narrowed-pin block (pass 2); clean item 3
  blocked in both passes on different criteria (AC-006, the sword's `PreferredContainerQuery`; then AC-003, the
  vessel items); clean item 2 approved in both; both seeds were detected in both passes, each by `reproduced`
  findings. One draw per `(pass, item)`: a single flip is not a rate (F27 and F29 measured per-item flip rates
  from 0.00 to 0.70 for the prose reviewer).
- **Every false positive rests on a `narrowed-pin` finding alone.** Recorded, not scored: under F31's lenient
  policy (`narrowed-pin` advises) clean approval would read 3/3 in both passes and detection would be unchanged,
  since every seed carries at least one `reproduced` finding. METHOD-S §3.4 registers the **strict** policy, so this
  changes no outcome above, and none of the three findings was adjudicated (Limit 4).

### METHOD-S §3.5's prediction against the outcome

| # | prediction (registered 2026-09-05) | outcome |
| --- | --- | --- |
| 1 | *"Detection 1/2 or 2/2"* | **CONFIRMED.** 2/2 in each pass. |
| 2 | *"the reviewer runs the automation suite unprompted"* | **FALSIFIED as measured: 0/10.** But not tested under its premise: §3.3 registered *"a disposable clone with the engine available"*, and the scored rendering states, under D21-153 item 3, that the review has no runtime, no PIE, no built binaries and no base checkout. |
| 3 | *"the field it cites most is `tests.report_ref`, not `assets`"* | **FALSIFIED.** `assets` 10/10, `tests.report_ref` 6/10. |

### Each blocking finding, with D21-152 item 3's evidence-source tag — recorded only, never used in scoring

D21-152 item 3: *"for each blocking finding, whether its evidence cites something read from the asset (an editor,
Python or binary read, or a capture) or only strings that appear in the spec or report. §3.4's decision rule is
unchanged."* The tag below is the build's reading of each finding's `repro.command`, `observed` and `rationale` in
the pinned digest. `asset` = it cites a read of an asset's bytes or a capture's pixels; `spec/report only` = it
cites nothing but strings in the spec or report.

| pass | item | finding | AC | class | tag | what it read |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | 3 (clean) | f0 | AC-006 | narrowed-pin | **asset** | `strings` and an export-table decode of `DA_Item_Sword.uasset` (a 13-byte placement fragment); test sources |
| 1 | 4 (seed) | f0 | AC-001 | reproduced | **asset** | `struct.unpack_from` at offset 33492 of `WBP_LoadoutScreen.uasset`: alpha 0.097 |
| 1 | 4 (seed) | f1 | AC-001 | reproduced | **asset** | the capture's decoded pixels: alpha 186, a 0.097 fill over the 0.7 scrim |
| 1 | 4 (seed) | f2 | AC-002 | reproduced | **asset** | both `LoadoutFill` exports (authoring and archetype) decoded: 0.097 |
| 1 | 5 (seed) | f0 | AC-001 | reproduced | **asset** | byte counts over `WBP_GameVitals.uasset`'s name table (no styling names; sha256 = `lfs_oid`), against two control assets; the capture |
| 1 | 5 (seed) | f1 | AC-001 | narrowed-pin | **asset** | by its digest's f0 read; its own command reads `Hud_Restyle.cqtest.cpp` and `_s-bin/report/index.json` |
| 1 | 5 (seed) | f2 | AC-003 | reproduced | **asset** | byte counts over the asset; the capture |
| 2 | 1 (clean) | f2 | AC-004 | narrowed-pin | **asset** | a decode of `WBP_StashRow.uasset`'s `CanvasPanelSlot_0` and `RowRoot` exports; the test source |
| 2 | 3 (clean) | f0 | AC-003 | narrowed-pin | **asset** | `strings` over the ten `DA_Item_Vessel_*` assets; test sources; `DefaultGame.ini` |
| 2 | 4 (seed) | f0 | AC-001 | reproduced | **asset** | the offset-33492 decode (sha256 = `lfs_oid`); the capture's alpha histogram |
| 2 | 4 (seed) | f1 | AC-002 | reproduced | **asset** | both `LoadoutFill` exports decoded |
| 2 | 5 (seed) | f0 | AC-001 | reproduced | **asset** | `grep -a` counts and a name/import/export parse of the asset (sha256 = `lfs_oid`); the capture |
| 2 | 5 (seed) | f1 | AC-001 | narrowed-pin | **asset** | by its digest's f0 read; its own command greps `OurGameTests/` and reads `index.json` |

**13 blocking findings, 13 `asset`, 0 `spec/report only`.** A mechanical census of the ten raw transcripts
corroborates it: every record shows tool uses reading `.uasset` bytes (8 to 33 per record), and every record whose
item carries a capture read it (2 to 4 uses); the pass-2 transcripts hold 212 tool uses, as D21-153's after-B3 note
counts (`S8I-C1-F43_rawscan.py`, scratch). The tag is a reading, not a re-scoring.

### D21-152 item 2's disclosure: what a branch-1 result means here

The seeded items' reports carry the commit's own diagnosis: item 4's quotes *"alpha was 0.097 where 0.97 was
intended"* and names the fill widget; item 5's carries R's tokens and names cold verification as the missing check
(D21-152, *"the widget, the pre-fix value or state, and for item 5 the cold-verification method"*). The review
tree carries none of them (amendment item 3). So pass 1's branch-1 result reads as **"sufficient given the
report"**, **not "ChangeSummary alone sufficient"**, and branch 2's signal (a reviewer naming a field it lacked) was
less likely for the same reason. The tag table above bears on it without settling it: every blocking finding
re-read the asset's own bytes or capture, and pass 2 item 4's f1 cites the report's claim only to contradict it
from the bytes.

### D21-149 item 2's confound, beside the false-positive outcome

**Capture presence follows asset type, and both seeds are widgets.** `capture_scene_preview` rejected every asset of
items 2 and 3 (a World; a Blueprint class and a data asset), so only items 1, 4 and 5 carry a capture, and
P(seeded | capture) = 2/3 over the five items. Each reviewer saw only its own item, and clean item 1 carries a
capture. Measured beside it: the false positives fell on item 3 (no capture, both passes) and item 1 (a capture,
pass 2 only), so capture presence does not line up with blocking on the clean items in this sample of three.

### D21-153 item 3: what `reproduced` can mean without a runtime

The scored rendering states that the review has no runtime, no PIE, no built binaries and no base checkout, and
that any read outside the review tree voids the record. The operator accepted *"knowingly that this bears on what
`reproduced` can mean for a runtime-only criterion, and F43 says so."* So it says so: every `reproduced` finding
above is a **read-back of persisted state** (asset bytes, a pre-rendered capture), never an execution. Item 1's
criteria are runtime-only; pass 1 approved it with an advisory saying *"I could not run it because this review has
no PIE or runtime"*, and pass 2 blocked it on a `narrowed-pin` whose criterion's verification is *"a PIE count of
generated entries"*, which no review tree can perform. That false positive is partly a property of the
no-runtime rendering.

### The voids, named

- **Scored set: none.** 10 valid of 10. No runtime void (`out_of_tree` empty on all ten), and no operator void.
  Every record was shadow-flagged and **cleared** by the operator in `oot-shadow-rulings.jsonl` (ten lines), on
  ruling (24)'s cross-check (below).
- **Discarded set (D21-153, not scored, never pooled):** pass 1's first attempt, 2026-10-07. Item 1 voided twice
  `out-of-tree` (25 and 64 listed reads; *"about 5 and 3 real"* under the corrected cwd model), each a genuine read
  of BrittonGames. §8i.6: *"The same item voids twice in one pass — a fixture defect, not noise."* D21-153's finding:
  the motive was item 1's runtime-only criteria, and the destination was the real project's absolute path in every
  review tree's `CLAUDE.md`, `AGENTS.md` and `_workspace/` docs. Ruled: discard, remove the path leak by a
  registered rule, state the review's limits in the rendering, make the audit's cwd follow the CLI, and re-run B2
  and B3. The discarded six (item 2 approve, item 3 blocks, items 4 and 5 detected, item 1 void twice) cost
  $16.074148 and count against both caps; they decide nothing here.

### Ruling (24)'s cross-check, B3's lost capture and its substitute

- **B2:** Process Monitor captured the whole launch; each dispatch's process tree was rebuilt by process instance.
  *No tree touched BrittonGames, another copy, the bases, the review repositories, another session's slug or
  another repository* (D21-153's after-B2 note). The static audit and the capture agreed on all five records.
- **B3:** the capture was **lost**. A host restart came after B3 finished but before Process Monitor finalised its
  backing file (`b3.pml`, 4,366,386,000 bytes, no events), and the CSV beside it holds pre-launch events only. A
  re-run under a fresh capture needed 5 Opus dispatches with 2 left of 18.
- **The substitute** (the operator, 2026-10-14, *"go with A"*): a full manual review of all **212** pass-2 tool uses
  against each record's copy, beside the static audit's empty `out_of_tree`, with B2 as calibration. Every command
  and read targets its own copy; each `%TEMP%` file was written by its own session first; item 3's one other read
  is its own background-task output (D21-145). **Disclosed residual:** *"a read the transcript does not show, such
  as one made inside a program a tool use ran, is checked by neither the static audit nor this review."* Pass 2's
  evidence of no out-of-copy read is therefore weaker than pass 1's.

### AC-475's lift share

**`ac475_lift_share=0/0`: the pinned corpus holds no diff-prose section to lift, and no rate is asserted.** F40's
index line is the precedent: *"recall has no denominator"*. Measured 2026-10-14 at zero model cost, executing no
`target/debug/redkiln*` binary.

- **The corpus**, through the object store only: `CORPUS_PIN`'s `CORPUS_SHA` `61f590e51cbb00301c310e46aab615563a7787b8`,
  blob `predecessor-47faec9f.bundle` (sha256 `a21419f4…9a03`), fetched into a scratch bare repository, and
  `47faec9f`'s `.bklg/` and `.redkiln/` archived out (1,649 files, 154 `spec.md`).
- **n** = the diff-prose loss markers `redkiln_migrate::lift::run` produces, each on a `LiftedWithLoss` file, read
  by a scratch harness (`S8I-C1-F43_ac475_share.rs`, an example in a private worktree at `42a923f`, never a workspace
  test). The marker is the one `diff_prose_carried_opaque_with_loss_marker` pins: `opaque_sections` marks a `## `
  section carrying a fenced `` ```diff `` block. **n = 0.** Control: the same harness over
  `crates/redkiln-migrate/tests/fixtures/legacy-mini` reads **1** (`story-2/spec.md`, `section ## Diff prose`).
- **The v0 migrator's own failures do not hide one.** It lifts 488 of the 1,649 files (156 with loss) and fails
  1,161, which carry no marker. A byte census over all 1,649 files finds **0** fenced `` ```diff `` blocks (and 0
  `` ``` diff ``, `~~~diff`, `` ```patch `` or `diff --git` lines), so no failed file would have carried one.
- **k** = those markers whose fenced body the `Text` variant accepts as a patch. `TextChangeSummary.diff` carries
  git's unified diff (rk-git `diff.rs`, git's header shape), so acceptance is git's own patch parser reading the
  fenced body as at least one patch (`git apply --numstat`, nothing applied). **k = 0** over n = 0. Controls: a
  fenced `diff --git`/`---`/`+++`/`@@` body is accepted and diff-shaped prose is refused; the fixture's one marker
  (two `-`/`+` prose lines, no header) is refused, so the fixture reads 0/1.

```bash
# Scratch only (D:/repos/redkiln-sev/afk-tools/sp/s8i/c1f43); every cargo run under the no-dispatch wrapper.
S=D:/repos/redkiln-sev/afk-tools/sp/s8i/c1f43; export TMP=$S TEMP=$S; cd "$S"
git -C D:/repos/redkiln-corpus cat-file blob 61f590e51cbb00301c310e46aab615563a7787b8:predecessor-47faec9f.bundle > S8I-C1-F43_pred.bundle
git init -q --bare S8I-C1-F43_pred.git && git -C S8I-C1-F43_pred.git fetch -q ../S8I-C1-F43_pred.bundle '+refs/*:refs/bundle/*'
mkdir -p S8I-C1-F43_target && git -C S8I-C1-F43_pred.git archive 47faec9f46b8d983c2c75f0794164e68415ae706 .bklg .redkiln | tar -x -C S8I-C1-F43_target
git -C D:/repos/redkiln-rs worktree add --detach target/wt-s8i-c1-f43 HEAD
mkdir -p D:/repos/redkiln-rs/target/wt-s8i-c1-f43/crates/redkiln-migrate/examples
cp S8I-C1-F43_ac475_share.rs D:/repos/redkiln-rs/target/wt-s8i-c1-f43/crates/redkiln-migrate/examples/s8i_c1_f43_ac475.rs
bash D:/repos/redkiln-sev/afk-tools/sp/main-session/nd.sh S8I-C1-F43-ctl cargo run -q --manifest-path target/wt-s8i-c1-f43/Cargo.toml -p redkiln-migrate --example s8i_c1_f43_ac475 -- D:/repos/redkiln-rs/target/wt-s8i-c1-f43/crates/redkiln-migrate/tests/fixtures/legacy-mini "$S/S8I-C1-F43_ctl_out"
#   -> files=23 failed=0 lifted_with_loss=3 diff_prose_markers=1
bash D:/repos/redkiln-sev/afk-tools/sp/main-session/nd.sh S8I-C1-F43-run cargo run -q --manifest-path target/wt-s8i-c1-f43/Cargo.toml -p redkiln-migrate --example s8i_c1_f43_ac475 -- "$S/S8I-C1-F43_target" "$S/S8I-C1-F43_run_out"
#   -> files=1649 failed=1161 lifted_with_loss=156 diff_prose_markers=0
grep -rl '```diff' S8I-C1-F43_target | wc -l                              # -> 0
python -I S8I-C1-F43_ac475_k.py S8I-C1-F43_ctl_out --controls             # -> control good=True bad=False; ac475_lift_share=0/1
mkdir -p S8I-C1-F43_run_out && python -I S8I-C1-F43_ac475_k.py S8I-C1-F43_run_out --controls   # -> ac475_lift_share=0/0
```

So the register's replacement for AC-475 states `lift share 0/0`, asserts no rate, and says the corpus holds no
diff-prose section to lift (§8i.8). Its corpus-scale verification stays Phase 2's (E13), and the tripwire
`ac475_zero_change_summary_values_in_output` stands until a lift is built.

### What this settles

1. **A reviewer without a text diff detects binary-asset regressions in this sample:** 4/4 seeded dispatches, each
   by a `reproduced` finding that re-read the asset's persisted bytes, and a capture where one existed. What it
   carried to do so is `assets` (every record), plus `captures` wherever a capture existed.
2. **It does not reliably approve clean binary changes under the strict policy:** 1/3 then 2/3 false positives, all
   `narrowed-pin`, none adjudicated. Pass 2 is not decided by the registered rule.
3. **The registered map gives the human-gate route** (D21-43 N21; §8i.8's table, for `undecided-fp` and for passes
   that disagree). What the `Binary` field set carries, and what evidence a `binary`-class AC gets, are ADR-0199's
   to argue from the census above.

### Limits

1. **n=5 is a type-design sample, not a rate** (§3.6: *"nothing here generalises beyond 'the type carries what one
   reviewer used'"*). Two passes, one draw per `(pass, item)`.
2. **The seeds are selected for perceptual defects.** The 2026-09-09 entry: *"a reviewer good at perceptual defects
   and bad at others scores well here on a non-representative sample"*. Both seeds are widget styling regressions.
3. **Seed 5 was replaced** before dispatch by R = `6a101bdcd` (WBP_GameVitals, *"the styling never reached disk, so
   the HUD loads white-on-white"*; D21-142), under the amendment's item 12 rule. `bf9c499a1` was replaced for
   correction 5 (amendment item 1) and the 2026-09-09 entry's own *"Seed 5 violates no registered AC of 8.5.5g"*. R
   was picked by the operator from three qualifying candidates *"as the most visually unambiguous of the three in a
   capture"*, which is the same selection threat as Limit 2.
4. **The false positives are not adjudicated.** CLEAN's ground truth is "this shipped", not "defect-free" (F24,
   F31). Two of the three `narrowed-pin` findings (pass 2 items 1 and 3) may be true gaps in the items' tests.
5. **Sufficient given the report** (D21-152 item 2, above), the **capture confound** (D21-149 item 2, above) and the
   **no-runtime rendering** (D21-153 item 3, above) each bound what a detection here shows.
6. **Pass 2's out-of-copy evidence is a manual review**, not a capture (ruling (24)'s capture lost), with the
   residual stated above.
7. **The scored set is a re-run.** Pass 1's first set was discarded at a §8i.6 stop and the fixture changed (the
   path rule, the rendering's limits paragraph, the audit) before the scored set ran; the scored rendering differs
   from the one the discarded set saw.

### The pins

Generated from corpus `835b122`'s blobs by `S8I-C1-F43_pins_gen.py` (scratch, `D:/repos/redkiln-sev/afk-tools/sp/s8i/c1f43/`), which re-runs the pinned `audit-s.py bin`, and checked line by line against `git cat-file blob 835b122:<path>` by `S8I-C1-F43_pins_check.py` (57 checked, 0 bad). **Two values are owed operator inputs:** `corpus_commit` and `results/s-bin/audit.txt`'s line, both marked `OWED-operator`, so Checkpoint 8i (2) and (3) refuse the block until they are filled. After committing `results/s-bin/audit.txt` to the corpus's `main`, the operator runs `python -I S8I-C1-F43_pins_gen.py <that commit> --ac475 0/0 > pins.txt` and `python -I S8I-C1-F43_pins_apply.py spikes/F43-DRAFT.md pins.txt`; the generator refuses an `audit.txt` whose outcomes differ from the re-run audit's.

<!-- s-bin-pins
corpus_commit=OWED-operator:the-corpus-commit-that-adds-results/s-bin/audit.txt
cli_version=2.1.289
observed_model=claude-opus-5-5
ceiling_secs=4500
review_root=D:/sb
rule_outcome_pass1=branch1
rule_outcome_pass2=undecided-fp
ac475_lift_share=0/0
file=results/audit-s.py sha256=bd6df55b28690913772c0fedef03c3a71526412447617d0566b1d758b16817dd
file=results/s-bin.jsonl sha256=da27621ab099d809bed8873c1df57ac2bd014192ca1adbc7c3925f137e175480
file=results/s-bin/audit.txt sha256=OWED-operator:re-run-S8I-C1-F43_pins_gen.py-after-audit.txt-is-committed
file=results/s-bin/brittongames-snapshots.txt sha256=59fdff794573af2e350e508488b34d2d2891ea4b21a60e9ba537fd763666feef
file=results/s-bin/captures/item-1/capture-1.png sha256=9ba295de7b614c61b8e17585a5888ab30b654440d0b5e92de86d1a3f756a98d0
file=results/s-bin/captures/item-4/capture-1.png sha256=97e93259f73f150a68b9fdc1cb6adc981b7bda632e31d8f9c24e1accf649a1c4
file=results/s-bin/captures/item-5/capture-1.png sha256=0de695357e86f170ed59f40432634988ecc8ba9ddd2071033d3bfc5f261bf026
file=results/s-bin/captures/procedure-log.txt sha256=1c76d15067dabc05665a6bd8eb4d88a19f7005a95fd0d8092ef04f8d7ce04caf
file=results/s-bin/driver-tests.txt sha256=0a00dbe1e85c9df58ae11e516d475a6be4a3e9853ace63e8c9f55202608b6897
file=results/s-bin/dry-run.jsonl sha256=44a1b9864b5a4ffe437c4de975e6e8aa23243891e8e9a7e9897a83d3575a7197
file=results/s-bin/fixture.json sha256=82add83e5cf2ef8f3ccd8bb7e561afd40a14274496bedd49d9d0ea4b369397cb
file=results/s-bin/greenness/item-1/headless.log sha256=1a675cd0870be05b38560937794da23b0d57cc1b64fa63978eefe2bfe38aa27c
file=results/s-bin/greenness/item-1/index.json sha256=49e3b3f084d2c3bcbb7a00fd679567e81eec0cb62b4ab8ea94a6c6a496f41a0c
file=results/s-bin/greenness/item-2/headless.log sha256=058e7c5bd8b378f516859f8d6d26e0b403a35bab8f995dc91bdb979d5356fa70
file=results/s-bin/greenness/item-2/index.json sha256=f511374108406e549174ccefc127dca78037d3fb51fbfa22b4821ecb9a830582
file=results/s-bin/greenness/item-3/headless.log sha256=f383279c4c0724684935200d0eda327e4f259f683e1152297bca516bbbfc7e45
file=results/s-bin/greenness/item-3/index.json sha256=9176f1ff8ace344a5e993f5efb9c7fdbe74e058aa6e0e208bb3dbb6f545e32ed
file=results/s-bin/greenness/item-4/headless.log sha256=e3031a72ecb5e3b8f45584993afb80787cd1126c31f5471d505685f6b3df551c
file=results/s-bin/greenness/item-4/index.json sha256=ea43a11b3839a40d0c679b67a9b9bd536b1b75ca7c3ab7dd1509a0b9248d9d9e
file=results/s-bin/greenness/item-5/headless.log sha256=fabbcc89d8c8daab2c2fd11441c1406cef897eb62cdd61a6ff3e87acfb5fccfb
file=results/s-bin/greenness/item-5/index.json sha256=755517b4312585def291fc8c1fc28a2806b7f3dfca69bf7128b5f8bd452c1139
file=results/s-bin/missing-fields.jsonl sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
file=results/s-bin/oot-shadow-rulings.jsonl sha256=914cbe1238b0ca992df1e584a3109492bc50dcf0679ee3cd283f8d2c91dcda09
file=results/s-bin/pin-report-2026-09-05/index.json sha256=c3dae80254299b8400149388d38df91b2885a4cf5c11ac0f8faf534b5d7e6bc6
file=results/s-bin/probes.jsonl sha256=61a8af904316af02cb9117bd1fb34eee5717d945904f519d92aeeb215cea8a4c
file=results/s-bin/raw/dry-run/p1i1a1.jsonl sha256=970e72d5611a97ce06081c568135d873f99add7e8d1c859338f6e1fcf66a232c
file=results/s-bin/raw/dry-run/p1i1a1.stderr.txt sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
file=results/s-bin/raw/p1i1a1.jsonl sha256=30aebe922d5eabd8c811b945da2fdacb5156da65ff97e27e06e18c0b0a43a013
file=results/s-bin/raw/p1i1a1.stderr.txt sha256=522131df5365780889fbaff5778ca0c0dc991b5fba6798c494faf675d14a68cf
file=results/s-bin/raw/p1i2a1.jsonl sha256=c891af61a898585c32aed797e259b4b4de42bd33e8ea1f84e7f1ac9e19e1a308
file=results/s-bin/raw/p1i2a1.stderr.txt sha256=16b9d6e216e9d6a1b969a4ca63885fe535d3abde84f2427add55cb687b9d7617
file=results/s-bin/raw/p1i3a1.jsonl sha256=0c01f52eda8366e31c30c2cebba650d2728a664e4e63d8d09bd2b17ab076ea41
file=results/s-bin/raw/p1i3a1.stderr.txt sha256=3fcb257b1c7c4794fb13d26df7dfe4365f8319bd3c4e7c8fec816afa58bf734d
file=results/s-bin/raw/p1i4a1.jsonl sha256=f626386a16820e2772a4b676cde3c782f0c24b79c0ff77137b8363400c24c1dd
file=results/s-bin/raw/p1i4a1.stderr.txt sha256=9fc09258143f1559ffd19056c2431d2d5d14063de7fba843a43d00ee0f16255a
file=results/s-bin/raw/p1i5a1.jsonl sha256=ffd9c0a070846876cc372c49bf544fd38c9907d8fae0d1e453681fa3963040c9
file=results/s-bin/raw/p1i5a1.stderr.txt sha256=ea78d1f0482f649e3d6a4a2ba9a2f2d13c86c4ff521110d114c1141239f97fdd
file=results/s-bin/raw/p2i1a1.jsonl sha256=13181a805fbc3a044e49df6d24c0b77d10a3a830f72032378e379fe630e626d9
file=results/s-bin/raw/p2i1a1.stderr.txt sha256=292f070d932731572cdaf6914d86be07f7c122cc6c6d7bf3354a6c04dafbef33
file=results/s-bin/raw/p2i2a1.jsonl sha256=5c1e74d3b03b3156040027f7eb75a31f4a1d00e2d4e2e75b723cfccb77ff4332
file=results/s-bin/raw/p2i2a1.stderr.txt sha256=e71efa530f95cfb68b19293de73849711ab5eb923870d9902f81afa55c7120fe
file=results/s-bin/raw/p2i3a1.jsonl sha256=1399f4880f55efeb5a881e2312665b4e6ad05383f018866228f372a41af933ec
file=results/s-bin/raw/p2i3a1.stderr.txt sha256=31ea0d2ecbe117046b18287701ce7e79555895a70766dcb012764f23e89e46c9
file=results/s-bin/raw/p2i4a1.jsonl sha256=e2e735a2851ec8cfa94b59e0b03bc6fa86cab7ab58e77aab9f3a9c4a45f3c4f9
file=results/s-bin/raw/p2i4a1.stderr.txt sha256=4f993f89a88338a77e2c83756b59820296eaec73b502c3d3cd0984605a78d41b
file=results/s-bin/raw/p2i5a1.jsonl sha256=ba03a7f81917fd913b621f8afab4bd49577f6fc26f9420fe0976306363a087b9
file=results/s-bin/raw/p2i5a1.stderr.txt sha256=2aa485fe01aa19624ba8a4e8d523bf19ebfdce5581c0f69bf94bf977c85589b0
file=results/s-bin/raw/probe-1.jsonl sha256=daed01803e990ee14f6fa5e0457eee719c5d2afd9db88a668f21ee48efff3d57
file=results/s-bin/raw/probe-1.stderr.txt sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
file=results/s-bin/raw/probe-2-cwd.jsonl sha256=098e9625d1233c6b8673601df4c94beee7f58520d5c8685292306dd16c38ceb4
file=results/s-bin/raw/probe-2-cwd.stderr.txt sha256=ad3b11f7325dd5f155cbdd00fac97ae4e7261853d8ba10f6125dbad5085a838a
file=results/s-bin/rendered/item-1.txt sha256=5216d5225f2dbe2217e69be2d03256d17d4a6d937502a2a0f3333616729efd1b
file=results/s-bin/rendered/item-2.txt sha256=f3075a9205bcd65f829f7511a5a8573548dc96f7332788e02ce93436d8f0df9f
file=results/s-bin/rendered/item-3.txt sha256=d4d25d5e118d4f271a59ca041921f3fbf79852119e2ab51a685e913be0873d2c
file=results/s-bin/rendered/item-4.txt sha256=b9e0393321c950543ae4a6c901db265d2da2f8db37633a91c82c4727055fa055
file=results/s-bin/rendered/item-5.txt sha256=00c06ea656c423b32f55ae995555e07a8ba8d35e6e576eb16e065b1a5bef3da6
file=results/s-bin/superseded/pass1-d21153/s-bin.jsonl sha256=830b957b65253a8c174b0316e24efc1de1d4040ae6080c514d2668dc831e95ef
file=runner/src/sbin.rs sha256=664fd3678ea88145b4dbd35378ebbaa22b3880ccb977da2162f1ea2e1ef39c5a
-->
