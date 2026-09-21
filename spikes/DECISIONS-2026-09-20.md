# Decisions taken for Step 7d, 2026-09-20 (D20-1), and what it licenses

Same shape as `DECISIONS-2026-09-18.md`. The entry below was taken **by the operator** (Ryan Britton) with the
record's nine decisions and the measured state of the tree in front of him, **after** the repair landed and
**before** the status flip that depends on it was committed. Grounding is a document clause or a measured
number, never taste.

**Numbering.** This carries a **`D20`-id** and opens a new space: `runbook/PROGRESS.md` §2 holds no `D19` or
`D20` row before this file, so it numbers from **D20-1**. Cite it as `DECISIONS-2026-09-20 D20-1`. No routing,
open item or residual is taken here, so the `RT-`, `O-` and `R-` spaces are untouched and continue from `RT-59`,
`O-16` and `R-62` wherever they are next opened.

**Precedence, stated once.** ADR-0092's: accepted ADRs > the AC register > the SM register > spike files > the
runbook.

---

## D20-1 · ADR-0130 is confirmed as written and flips to `accepted`

**Confirmed 2026-09-20 (dated line, the operator's word).** ADR-0130 — the recorded `Config` written to the
run-record layout as a conditional `inputs/config.json` and read back through `Config::new`, with `run_id`
keeping the config in its preimage, the golden-log schema staying `rk-kernel/golden-log/v2`, and a pre-repair
record carrying a config re-recorded rather than re-blessed — is **confirmed as written on this date**, all nine
decisions, with no clause withdrawn and none added. It flips `proposed → accepted` in the commit this line
licenses, `date: 2026-09-20` **unchanged** (ADR-0126's route, D18-17's form); `adr/README.md` takes its recount
and its index-row status cell in the same commit. No `supersedes` or `superseded_by` link is owed — the record
supersedes nothing, and its `related` list (`0024`, `0083`, `0099`, `0105`) stands as written.

**Grounding, measured rather than asserted.** The repair is landed at `e9a44af` and **R-54 is REPAIRED**:
decision 9's symmetry test was authored before the repair and observed RED on the defect itself, both its
premises passing first; three declared mutants ran after the fix from Python with a byte-compared restore and
each was caught; decision 7's confirmation is the deletion of `support::inputs_over_pack_min_without_config`, so
no marker of the defect is left to read as open. Gates at that commit: `fmt` ok, `clippy --workspace
--all-targets` rc=0, `-p rk-kernel` 189 + 11, `-p rk-cli --test it` 62, `--workspace --features corpus` 626 / 0;
Checkpoints 7b and 7b-ii re-run WHOLE and green.

**Decision 8's `producer` clause was corrected in place before this confirmation, at `e66dac5`.** As written it
read *"that is **R-38**, still open, and untouched here"*. R-38 was **SETTLED 2026-09-20** at `4b5119b` —
record-only, no code change, no ADR, the literal standing — so the status word was stale while the clause's
normative content (that this record does not decide `producer` on a resumed run) stayed true. The correction was
made while the record was still `proposed`, on ADR-0128/0129's precedent (`a94192f`), because ADR-0092's
`**What this forbids**` forbids an erratum that touches a `## Decision` sentence — after acceptance the only
lawful repair would have been a superseding record. The record is confirmed **as it now stands**, that
correction included.

**Alternative named.** Leaving ADR-0130 `proposed` through Slice 4 and flipping at the stage close (rejected: the
record's subject is landed, gated and unmoved, and ADR-0126's precedent is that the flip follows the repair
rather than the stage; a `proposed` record the tree has already built against is the ambiguity the flip exists
to remove).