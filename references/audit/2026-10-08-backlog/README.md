# Backlog reconciliation, 2026-10-08

This compares the planning, audit and output records with GitHub Issues, and proposes a manifest that makes GitHub the backlog ledger. The conventions it adopts are in `docs/BACKLOG.md`. Everything here was verified at `71fc926` on `build/phase-0-to-1`.

**Status:** proposed. Nothing on GitHub has changed yet.

## Method

1. **Survey.** Three independent read-only passes, each with `file:line` evidence:
   - the roadmap, from `RUNBOOK.md`, `runbook/` and `PROGRESS.md`
   - the audits (`2026-09-10`, `2026-10-06-status`) mapped to the open issues, plus the DECISIONS ledgers and the F-drafts
   - GitHub's backlog features as of 2026, from GitHub docs and changelog
2. **Re-verification.** Three agents re-checked all 29 open issues against the code and docs at HEAD.
3. **Drafting.** One agent drafted `manifest.json` and `notes.md`.
4. **Review.** A fresh-context reviewer grepped every ID and `path:line`, and checked hierarchy, cycles, labels and granularity. It fixed 15 defects in place.
5. **Validation.** A final mechanical check covered refs, labels and readiness. A dry run of `tools/backlog/apply-manifest.mjs` gave 216 planned `gh` calls and no errors.

## Findings: the gap between the records and GitHub

**What GitHub holds today**
- 29 open issues, #5–#33. They map one to one to `2026-10-06-status/issues/A1..C12.md`. B8 and C8 were deliberately not filed (audit README).
- There are no milestones, no sub-issues, no blocked-by links and no issue types.
- Labels are GitHub's defaults plus `audit-2026-10-06`.
- Dependencies exist only as prose, for example "see C1" or "after M5".

**Stale issues** (re-verified at HEAD)
- **#23.** The merge (`88a8dbd`) and the push are done, and CI is green at `62dd3b8`, `88a8dbd` and `71fc926`. The records still say "never pushed". The issue is narrowed to the records half.
- **#20.** The kill fix has landed. `Group::is_empty` still counts zombies. Narrowed.
- **#32.** The rs-repo half is stale. The corpus repo still has no remote. Narrowed.
- **#29.** Widened to cover the stale tracker rows: L2, L3, D17-3, D17-4, RT-27 and R-40…R-62. They read as open, but s8e, s8g and s8h close them. M4 is still unstamped.
- **Still valid:** the other 25 issues. They were not fixed at HEAD.

**Epic-sized issues filed as one item**
- #8, #9, #10, #13, #14, #15, #25, #27 and #31 each hold 2–6 PR-sized parts.
- Each becomes an Epic, keeps its number, and gets sub-issues.

**Decisions mixed in with work**
- Decision issues: #21, #24, #26, #28, #30 and #33.
- Decisions with no issue at all: audit O-4 (sign ADR-0197), O-7 (fold the run-path fixes into s8j), R-37, the temper ruling r1, and the decision gates inside the A4, A6, A9, A10, B1, C3, C5 and C10 splits.
- In the manifest these become Tasks labelled `needs/decision`. Each blocks the work it decides.

**Untracked work outside the runbook**
- R-64 (`doctor --rebuild` tag re-derivation, which bears on Step 8 check (2)).
- The red temper gate (ruling r1).
- The unrun audit follow-ups and the spikes that were registered but never run.
- The post-M5 hardening and Phases 2–4 roadmap (appendix §16).

**Deliberately not filed**, because the runbook owns them until M5:
- D1 (s8j §8j.2), D9 (§8j.3), D6 (§8j.6), O-8 (§8j.5), L3 (s8e §8e.1)
- every s8i, s8j and Step 8 section, slice and go-ahead

**Namespace collision**
- Design §8.6 reserves `rk:stage/*` and `rk:gate/*` labels for the product's own tracker adapter (AC-507).
- The ledger never uses `rk:*`.

## Proposal (manifest summary)

**Existing issues**
- 15 kept, re-typed and re-labelled.
- 9 converted to Epics.
- 4 narrowed.
- 1 retitled.
- None closed.
- Each change carries a comment with its evidence.

**New issues: 42**
- 5 Initiatives:
  - *M5 — Phase-1 exit* (a pointer to the runbook)
  - *Post-M5 hardening*
  - *Phase 2*, *Phase 3* and *Phase 4* (placeholders, chained by blocked-by)
- 1 Epic: run-path fixes #5, #6, #7 and #11.
- 36 Tasks, Features and Bugs: the epic children and the decisions.

**Totals after apply**
- Types: 5 Initiatives, 10 Epics, 38 Tasks, 4 Features, 14 Bugs.
- 21 issues meet the definition of ready.

**Labels**
- 32 labels in `.github/labels.yml`.
- `audit-2026-10-06` is renamed to `source/audit-2026-10-06`.
- The GitHub defaults are deleted after migration (`--delete-retired`).

The full table is in `notes.md`, and the exact operations are in `manifest.json`.

## Before `--apply`

1. **Operator:** add the issue types **Epic** and **Initiative** in the org (Settings → Planning → Issue types). The script refuses to apply without them.
2. **Operator:** approve the manifest. This includes deciding whether #5, #6, #7 and #11 stay `status/ready` before audit O-7 is decided.
3. Run `node tools/backlog/apply-manifest.mjs references/audit/2026-10-08-backlog/manifest.json --apply`.
4. Run the dry run again. It must report `0 action(s)`.
5. Run `--apply --delete-retired`.
