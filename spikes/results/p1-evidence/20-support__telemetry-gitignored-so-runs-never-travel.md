# 20/20 — support/telemetry-gitignored-so-runs-never-travel

**kind:** incident · **commits:** 0 · **files from commits:** 0 · **paths named only in the card:** 4

## Files changed (outside .bklg/)

- `src/store/init.ts`  [named in the card; no commit found]
- `src/store/run-log.ts`  [named in the card; no commit found]
- `test/support-filings.test.ts`  [named in the card; no commit found]
- `src/hook.ts`  [named in the card; no commit found]

## Card body (the item as written)

```
> **WITHDRAWN 2026-08-11 — the observation below is no longer true of this repository.**
>
> This was filed while `.gitignore:14` still listed `.redkiln/telemetry/` on this branch. Merging
> `origin/main` (`3e931b64`) into `initiative/live-views` brought `.gitignore:12`'s
> **`TELEMETRY IS COMMITTED`** block, which removed the entry and names issue #65 as the reason.
> Both live probes now answer the opposite way here: `git check-ignore -v --no-index --
> .redkiln/telemetry/events/x.jsonl` matches nothing (exit 1), and `git ls-files .redkiln/telemetry`
> returns 22 tracked partitions where it previously returned none. DoD 13's *"from committed
> telemetry"* clause, which this item recorded as unreachable by construction, is reachable now.
>
> The originating finding (`gitignored-telemetry`, raised by `RK-S0121`) was re-settled from
> `confirmed` to `withdrawn`, and the census that filed this item stopped deriving the gap.
>
> **This is the recommended fix arriving, not a decision reversed.** RK-S0121's disposition of
> initiative AC-013 recommended `gitignore-change` and recorded that the repository owner had already
> made it on `main`. The merge is that change being inherited here, so the recommendation is
> **satisfied** — it is not withdrawn with this item.
>
> **The record below is kept verbatim, not rewritten.** Full withdrawal record, with evidence and
> decision owner, in
> `.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md` under
> `withdrawn[]`.
>
> **Still open as a stage transition.** Closing this item is `redkiln advance`, which belongs to the
> initiative owner at the closeout gate.

## Summary

This repository's `.gitignore:14` ignores `.redkiln/telemetry/`, which the stock `redkiln init`
scaffold **tracks** (it ignores only `.redkiln/index/`). Telemetry is therefore written, and is
correct, and never travels: it exists only in whichever working tree produced it. `record-run` warns
about exactly this and the warning had been standing unread. Populated run data cannot be read from a
clone, from CI, or from another worktree — so any question of the form "did this workflow actually
run" is answerable only by whoever is sitting at the machine. Status: **already changed on `main`** at
`e09892f1` (with a comment block naming issue #65); this branch does not carry it.

Filed by **RK-S0124** `file-every-gap-as-a-support-item` from the census in
`.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md`, census gap id
`gitignored-telemetry`. **Raised by: `RK-S0121`** (`settle-the-instrument-findings`). **Grounding
path:** `.bklg/_archive/live-views/proof-and-readings/settle-the-instrument-findings/_findings.md`.

## Findings Ledger

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **Instrument.** `.gitignore:14` read against `redkiln init`'s own scaffold — `src/store/init.ts:38,123` — and `record-run`'s standing warning at `src/store/run-log.ts:319-327`. The instrument is the repo's own configuration compared with the tool's default, which is why nothing ever failed. | `.gitignore:14`; `src/store/init.ts:38,123`; `src/store/run-log.ts:319-327` | — |
| **Observation, verbatim from the reading.** *returns no output at all: not one partition is tracked on this branch* | `.bklg/_archive/live-views/proof-and-readings/settle-the-instrument-findings/_findings.md` (verdict `confirmed`, raised by `RK-S0121`) | The observation is a command's empty output, not an inference from one |
| **Why it was not fixed here.** `project.md` *Out of scope* names `.gitignore` explicitly: changing it is a **repo-wide policy call** RK-P0011 may not make. RK-S0121's disposition *recommends* the change and records that `main` already made it at `e09892f1` with a comment block naming issue #65. | `.bklg/_archive/live-views/proof-and-readings/project.md` (Out of scope) | A recommendation is not an adoption; the adoption is the owner's |
| **Decision owner — open.** The repository owner (ryan-britton) — `.gitignore` is repo-wide policy and has already been exercised on `main`. Whether **this branch** inherits it before closeout is the initiative owner's call at the closeout gate. | Disposition recorded in `_findings.md`; recorded again in `.bklg/_archive/live-views/_implementation.md` as reserved for closeout | This item records the recommendation and settles nothing |

## Timeline

- Detected: 2026-08-09, by `RK-S0121` while dispositioning initiative AC-013.
- Mitigated: on `main` at `e09892f1`; **not** on `initiative/live-views`.
- Resolved: open on this branch.

## Impact

- Affected: every consumer of `.redkiln/telemetry/` — `redkiln runs`, the index replay, and any
  after-the-fact audit of workflow execution.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
```
