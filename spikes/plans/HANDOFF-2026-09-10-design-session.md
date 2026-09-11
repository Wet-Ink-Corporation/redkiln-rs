# Continuation prompt — redkiln-rs design session, 2026-09-10

Paste everything below the line into a fresh session at `D:\repos\redkiln-rs`.

---

You are continuing a design session on `redkiln-rs`. The 2026-09-09 objectives were completed and a second
finding landed on top of them. Nothing is half-written; what remains is listed and each item has a named owner.

**Read first, in this order, and nothing else until you have:**
1. `spikes/F40-DRAFT.md` — the newest result. Read it in full.
2. `spikes/F39-DRAFT.md` — the result the previous session absorbed. Read it in full.
3. `references/2026-09-08-architecture-design.md` §0.1 — the decided/blocked table, **39 rows, ten BLOCKED**,
   one screen. It is the whole decision surface.
4. `references/2026-09-08-architecture-design.md` §10.5 — the spike register, 35 rows with a status column.
   **This is the tracking.** Do not build a second list somewhere else.

Do **not** read `references/2026-09-08-acceptance-criteria.md` (~530 KB) or `-success-measures.md` (~230 KB) end
to end. Grep them for the ids you need.

## State

Branch `main`, tree clean apart from 17 untracked entries that RUNBOOK Step 1.2 owns deliberately (14 tee'd
driver logs, `spikes/results/p8-state/`, `spikes/runner/target-{dev,p8}/`). Everything else is committed.

Delivered since 2026-09-09: F39 absorbed across seven documents; **ADR-0089** (facet provenance, declared
`surfaces`, abstention) partially superseding ADR-0030; **AC-520** and **SM-157**; the P2 worksheet and its
data-entry Artifact; **S-BIN** bound to a real Unreal project as a dated METHOD-S amendment; **F40** (P2's
perturbation half scored) and its absorption; and `spikes/p2/score-p2.py`, which METHOD-P §2.3 registered and
nothing had computed.

## What F40 says, in four sentences

P2's perturbation corpus **contains no staleness to detect** — 0 of 41 rows are "should re-run", because 11 of
the 12 perturbations change nothing but the YAML frontmatter the CLI rewrites on every stage advance
(`stage`, `status`, `updated`, `gate_open`). So recall has no denominator and **two of P2's three kill criteria
are not answerable**; the honest statistic is specificity, coarse **0.73** [0.58, 0.84] against fine **0.55**
[0.38, 0.70] — intervals overlapping, so not separated at this n, and separated instead by the paired
comparison: on the 33 rows both scopes scored, **10 flip `Present`→`Stale` coarse-to-fine and none flips back**.
Criterion 1's precision half fires, and **the remedy it prescribes — make fine scope mandatory for `L`-class —
is contraindicated by the same measurement**. The mechanism is F39's failure class at the artifact level: a
derivation inferred from a trace the instrument itself writes into.

## The open loops, each with its owner

**Blocking Phase 1** (§0.1 says so explicitly; everything else stops specific work, not the phase):

| item | state | what unblocks it |
| --- | --- | --- |
| **`ChangeSummary` type design** | **S-BIN fixture PINNED, spike UNRUN** | Its greenness precondition, below |
| Selection engine — the LLM-emitted vector | BLOCKED (ADR-0079, `proposed`) | AC-071's identity clause: F36 measured 0.25/0.15/0.10/0.10 against 0.80. **F39 discharged the arm-D-panel clause and only that one** |
| Selection engine — the DERIVED-facet source | BLOCKED (ADR-0089) | **S-FACET**, registered and unrun |

**Owed by Ryan, in the order the handoff's own list gives them:**

1. **DECISIONS-2026-09-09 #15** — the `human_authored` remap. It gates P2's advisory half and nothing else now.
2. **`spikes/results/p1-arm-l-labels-RYAN-TODO.md`** — the one label file still owed. Note the gap recorded in
   `spikes/HANDOFF.md`: **arm L's labels have no scorer**, so filling them currently feeds nothing. Write the
   consumer first or the day is spent for no result.
3. **The `bisect` ceiling question** — §10.5 calls it the only "awaiting Ryan" row left.

**S-BIN's remaining precondition, and it is a precondition rather than a detail.** METHOD-S §3.2 requires the
seeds' greenness verified **before** dispatch: the automation suite must still pass with the reverted asset in
place, because a seed the suite catches measures nothing about review-without-a-diff. It has **not been run**.
It needs `_workspace/tooling/verify_headless.ps1` in `D:/repos/BrittonGames` with the editor closed. The
adversarial pass named a live route for it to fail — `Inventory_StashLoop.cqtest.cpp` loads `WBP_StashPanel` by
hard path, so **seed 5 must be checked against that spec specifically**. If a seed goes red it is replaced from
the remaining 27 asset-only commits and the METHOD-S amendment is extended, never quietly edited.

**Registered and unrun, with named consumers:** `S-FACET` (ADR-0089), `P2-B` (recall's missing denominator),
`S-GIX`, `S-TTL1`, `S-H1c`, `S-SUBTLE-CB`, `S-CODEX`, `S-RETRY`. §10.5 carries all of them with cost and gate
status. *[corrected 2026-09-11, PIA-102: not all with a cost — on 2026-09-10 `S-SUBTLE-CB`'s Cost cell held no
figure ("must be sized before it runs") and `P2-B`'s read "Not computed"; `S-CODEX`'s 46 dispatches carried no
price. As of 2026-09-11 S-SUBTLE-CB carries a first-order estimate (2 framings × 5 items × k draws at S-TYPED's
median $3.03/review, ADR-0101), P2-B is sized in METHOD-P amendment 12 ($0 model cost; ≥ 35 positives to label),
and S-CODEX is registered in `spikes/METHOD-S-CODEX.md` with its cost stated as unpriced.]* **P2-B is registered once, in §10.5** — an earlier draft of the F40 absorption registered the same
spike three times under three names, which is the exact failure ADR-0012's named-consumer rule exists to
prevent.

**Three audits recorded rather than resolved**, each because resolving it needs judgement rather than a count:

- **`ADR-0059`'s citation direction.** AC-070 and AC-072 cite it for the facet registry; its text contains the
  word "facet" zero times — it is the *verb* registry record. Retained rather than struck (a loose citation is
  not a fabricated one), and AC-520 dropped it on authoring. Recorded in the AC register's §15.2 closing note.
- **The success-measure register's "48"** cannot be reproduced from the register's own labels; recomputing under
  each consistent reading gives 28, 37 and 95. Flagged for review, not asserted wrong.
- **SM-022 and SM-050** are cited as §2 examples and have no row in the §2 table.

## Traps this session actually hit

- **A check that exercises something adjacent to the operation is not a verification of it.** Twice. (1) The
  S-BIN seed design said "revert this asset to its pre-fix revision, against the pin" and claimed the reverts
  were verified; what had been checked was `git diff --stat <seed>^ <seed>`, which reports what that *commit
  changed*, not what reverting *at the pin* restores. Both assets had been touched again since, so seed 4 would
  have carried seed 5's defect and seed 5 would have stripped a shipped feature whose spec is live. The
  settling check was a blob-oid comparison. (2) F40 claimed the labeller's frontmatter-only reason appeared on
  "39 of 41 rows" — 41 minus P01's two, computed rather than counted. It is 36. And F40 said P01 changed "two
  body lines"; it changed **one**, shown as a `-`/`+` pair.
- **A claim that the machine checks something must be true.** F40 said P2's blindness was "checked by the
  scorer's own guard"; the scorer compared labels-to-engine *identity*, never chronology. Fixed by making the
  claim true — `score-p2.py` now asserts both stamps are UTC `...Z` and that the labels precede the engine, and
  refuses otherwise. Verified both ways.
- **An adversarial pass that swaps a term for another has removed one.** RUNBOOK golden constraint 10 had
  "not-measurable as not measurable" replaced by "not-answerable as not answerable". They are different faults
  and the programme has one of each (F39's `data_migration`; F40's recall). Both restored.
- **Verify before calling a correct claim a defect.** A naive substring classifier said the design's §0.1
  recount was wrong; direct inspection showed the author right — `DECIDED (was PROVISIONAL…)` and
  `DECIDED (…the BLOCKED row above)` had been mis-binned.
- **Numbered lists in the evidence brief are cited by number from four documents.** §C runs to **C.14**; F39's
  is C.13 and F40's is C.14, both **appended, never inserted**. An earlier pass inserted at position 10 and
  silently broke every live §C.10–C.12 citation.
- **`spikes/HANDOFF.md` runs stale** and has now misled four separate pieces of work. Check its status lines
  against `git log` before acting on one.
- **Bash heredocs break on apostrophes and backslashes** here, and the Windows console cannot print `→`, `—` or
  `§`, so a script that prints its own anchor string crashes *before* it writes. Use the Write tool for prose,
  and never print non-ASCII from a fix script.

## House rules

Evidence discipline is the point of this repo. Every load-bearing claim carries a finding number or a cited
source, tagged `[measured]` / `[documented]` / `[inferred]`. A rate carries a Wilson interval or says the
interval is not computed. A saturated metric is reported as saturated, a powerless criterion as powerless, a
**not-measurable** one as not measurable and a **not-answerable** one as not answerable — those last two are
different faults. An abstention is reported as its own rate and never as a miss. Unknowns are discharged as a
named spike with a named consumer (ADR-0012), never as prose. Accepted ADRs are immutable — supersede, never
edit, both sides linked. `spikes/METHOD*.md` are pre-registrations: a result *about* one is reported in the
finding, never as an edit; the only permitted change is a dated amendment that leaves the registered wording
verbatim. Commit subjects state the finding, not the change. British spelling.
