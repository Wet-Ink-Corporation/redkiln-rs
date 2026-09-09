# 18/20 — support/links-hand-edit-denied-with-no-backfill-path

**kind:** incident · **commits:** 0 · **files from commits:** 0 · **paths named only in the card:** 1

## Files changed (outside .bklg/)

- `src/hook.ts`  [named in the card; no commit found]

## Card body (the item as written)

```
## Summary

`links` is on the PreToolUse **DENY** list (`src/hook.ts:140`), so `record-links` is the sole write
path and a missing sha can never be patched by editing frontmatter. **This is a correct and
deliberate property of the single-write-path rule, and this item does not argue otherwise.** What is
filed is its *consequence*: a provenance gap is therefore a **process** gap, closable only by someone
deliberately running the command — and, combined with `links` being on no surface (RK-N0006) and the
provenance gate being off (same), nothing prompts anybody to run it. There is no backfill affordance:
no `--backfill`, no doctor repair, no reconciliation pass. Status: open, and filed **with the
disagreement recorded**.

Filed by **RK-S0124** `file-every-gap-as-a-support-item` from the census in
`.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md`, census gap id
`hand-edit-denied`. **Raised by: `RK-S0120`** (`enumerate-provenance-links`). **Grounding path:**
`.bklg/_archive/live-views/proof-and-readings/enumerate-provenance-links/_provenance-reading.md`.

## Findings Ledger

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **Instrument.** The PreToolUse write gate — `src/hook.ts:140`, the DENY list that makes `links` unwritable by hand. | `src/hook.ts:140`; the companion rule at `:179-181` (a `Write` to a canonical item file is always denied) | — |
| **Observation, verbatim from the reading.** *`links` is on the PreToolUse DENY list, so `record-links` is the sole write path and a missing sha can never be patched by editing frontmatter.* | `.bklg/_archive/live-views/proof-and-readings/enumerate-provenance-links/_provenance-reading.md`, `findings[].id: hand-edit-denied` (raised by `RK-S0120`) | — |
| **Why it was not fixed here — and why "fixed" is the wrong word.** This is a deliberate and correct property, not a defect to repair. It is filed because its CONSEQUENCE is the thing that needs owning: a provenance gap can only be closed by someone deliberately running the command. `project.md` *Out of scope* forbids repairing the instruments in RK-P0011 in any case. | `.bklg/_archive/live-views/proof-and-readings/project.md` (Out of scope) | Anyone triaging this should read it as "pair the deny rule with an affordance", never as "relax the deny rule" |
| **Demonstrated between two readings, not argued.** The sixteen `proof-and-readings` stories that carried no `links.commits` at the first taking closed only when the orchestrating command ran `record-links` sixteen times by hand (`c32f527d`, `58ede01a`). Nothing else in the toolchain could have closed one of them. | `_provenance-reading.md`, "The fourth taking"; `.bklg/_archive/live-views/_implementation.md` | That backfill is also the reason RK-N0008 was withdrawn — the same event, read two ways |
| **Decision owner — open.** The redkiln product owner (ryan-britton): whether the deny rule is paired with a backfill affordance is a product decision. Filed **with the disagreement recorded** — raised as a consequence, not as a claim that the deny rule is wrong. | — | Triage with RK-N0005 and RK-N0006 |

## Timeline

- Detected: 2026-08-09, by `RK-S0120` while enumerating provenance for project AC-010.
- Mitigated: not applicable — there is nothing here to mitigate, only something to decide.
- Resolved: open.

## Impact

- Affected: anyone who discovers a provenance gap after the fact. The gap is real, the data is
  knowable, and the only route to closing it is a per-item command invocation by a human.
- Severity `low`, and deliberately so: this is the *consequence of a design property working as
  intended*, which the severity rule in `_filings.md` anchors to `low`. It is not a malfunction, and
  it should not compete for attention with the two `high` items filed from the same reading's
  neighbourhood.
- Real cost, measured once: sixteen manual invocations across two commits to close one branch's
  backlog of misses.

## Mitigation

None, and none is wanted from this item alone. The honest framing is that three findings compose into
one exposure — no read mode (RK-N0005), no projection or surface (RK-N0006), and no backfill
affordance (this item) — and that mitigating any one in isolation would be guessing at the product
decision the three share.

## Notes

- **This filing disagrees with itself on purpose.** The story map rule is that a story files
  everything it raises, including a finding it would argue with; recording the disagreement is what
  keeps that honest rather than turning the filing into an accusation.
- Sibling filings from the same reading: `record-links-has-no-read-mode` (RK-N0005),
  `links-projected-into-no-index-or-surface` (RK-N0006).
- This item was minted by `redkiln new` in phase 2 of RK-S0124's two-phase filing (`c32f527d`); only
  this body is hand-authored, the frontmatter is the CLI's (`src/hook.ts:179-181`) — which is the
  very rule this item is about.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
```
