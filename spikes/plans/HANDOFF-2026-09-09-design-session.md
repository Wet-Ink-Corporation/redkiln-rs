# Continuation prompt — redkiln-rs design session, 2026-09-09

Paste everything below the line into a fresh session at `D:\repos\redkiln-rs`.

---

You are continuing a design session on `redkiln-rs`. The main objectives were completed; what remains is
absorbing one late result into the documents, and three pieces of unblocked work.

**Read first, in this order, and nothing else until you have:**
1. `spikes/F39-DRAFT.md` — the result that changes things. Read it in full.
2. `references/2026-09-09-change-classification.md` §0, §4, §6a — the research that independently reached the
   same conclusion.
3. `references/2026-09-08-architecture-design.md` §0.1 — the decided/blocked table, 37 rows, one screen. It is
   the whole decision surface.
4. `references/adr/README.md` — 88 records, the legacy-id map, the supersession table.

Do **not** read `references/2026-09-08-acceptance-criteria.md` (524 KB) or `-success-measures.md` (221 KB) end to
end. Grep them for the ids you need.

## State

Branch `design/2026-09-08-acceptance-criteria-adrs-architecture`, **6 commits ahead of `main`, nothing pushed.**
Tree clean apart from `spikes/results/p8-state/` and raw driver logs, which Runbook Step 1 owns deliberately.

Delivered and committed: 88 ADRs + index (`references/adr/`); 513 Given/When/Then acceptance criteria; 156
success measures; the authoritative architecture design; an external research pass; a condensed evidence brief;
`RUNBOOK.md` (Phase 0 → Phase 1, ten steps); METHOD-P amendments 8 and 9; the P1 arm-D oracle, worksheet,
triage and scoring tooling; F39.

## What F39 changed, and what has not caught up

P1 arm D was scored against 140 truth cells (60 deterministic oracle, 48 unanimous model panel, 32 human
adjudications by Ryan, blind). **No facet clears METHOD-P §1.4's 0.95 bar.** `auth` scores **0.00 over 11
positives** — every positive false, because `auth` is a substring of *authored*, `oauth` of *coauthor*, and
`token` matches a design token. `data_migration` emits zero positives and is **not measurable**. `ui` is 7/7 but
its 95% lower bound is 0.65.

Three consequences are **written down but not yet absorbed into the design documents**:

1. **`auth` should stop being a repo-wide boolean.** It becomes a set-valued `surfaces` facet over a declared
   path map, and each activity's `applicable_if` owns its own evidence question (ADR-0024 already places that on
   the activity). This amends **ADR-0030 (PC8, facets not tags)**, which currently requires every facet to map
   to exactly one computed fact.
2. **Absence of evidence must stop resolving to `false`.** Four of twenty items changed no code; the panel said
   `unknown` on all sixteen of their semantic cells; the script emitted a confident boolean on every one. The
   boolean facets need the third value the ordinals already have, routed to `AC-079`'s existing
   `gate_opened { reason: unknown_facet }`.
3. **Source asymmetry is a separate, untaken decision.** `commits` beats `docs` on every facet (toolchain 1.00
   vs 0.50), and `union` is *worse than `commits` alone* on two facets because it imports the imprecision.

## Do these, in this order

**1. Absorb F39 into the documents.** The design's §0.1 selection-engine row, §4, and §10 open items; the
affected acceptance criteria (grep `AC-071`, `AC-079`, `AC-113`); the success measures for the facet surface
(grep `SM-015`, `SM-024`). Write the new ADR amending ADR-0030 per consequence 1 — next free number is **0089**;
ADR-0030 stays `accepted` with a partial supersession, both sides linked, per `references/adr/README.md`.
**ADR-0079 stays `proposed`** — arm D measures a *derived* vector; 0079 asks about an *LLM-emitted* one. Say what
F39 contributes without deciding it.

**2. P2 — 41 rows, not blocked, $0.** `spikes/results/p2-labels-RYAN-TODO.md`. Unlike arm D it needs no
amendment: its engine phase requires only a timestamp and its verdicts do not exist yet, so labelling now is
blind on the original terms. Build Ryan a worksheet the way arm D got one —
`spikes/p1/build-arm-d-worksheet.py` is the pattern, and the lesson from it was that `links.commits` is empty on
most cards and commits must be found by the slug leaf in the subject and by `Story:`/`Fix:` trailers.

**3. S-BIN — Ryan has a real Unreal project.** This is one of only three items that genuinely gate Phase 1 and
the only one with no plan. Ask for: repo path, engine version, whether automation already runs headless
(`-NullRHI -ReportExportPath`), and whether assets are in LFS. Then design the five-item fixture set and the
seeded regression against the real project rather than the paper design. The Runbook offers a re-scope branch if
running it is impractical; that is a decision for Ryan, and it is an ADR either way.

**4. Ask before pushing or merging.** Six commits sit on a branch. Recent history commits straight to `main`, so
`git checkout main && git merge --ff-only <branch>` is likely what Ryan wants — but ask.

## Traps this session actually hit

- **Read the type definitions before calling an API.** Three defects, all the same class — assuming a value's
  shape instead of checking it. An artifact db path segment may only contain `_ - . ~ : @ +`, and `db.doc()`
  throws *synchronously* on a bad one; `collection().get()` returns a `QuerySnapshot` with a `.docs` array, not
  an array; the oracle emits `binary_change` as a JSON bool, not a string. The first cost Ryan 32 adjudications
  (recovered from browser memory); the third briefly scored a facet at 0.00 when it is 0.60.
- **A save failure on data-entry work must be loud.** A caught exception written to a small grey status line let
  29 failed writes look like success while the progress ticks still advanced.
- **An adversarial pass is only as good as the corpus it may check against.** Thirteen area refuters were told to
  verify citations against a condensed brief; the brief is a subset, so eleven *real* citations were struck as
  fabrications (`P3`, `P4`, `P5`, `GATE-02`, `GATE-05`, `F35 §1.3`, `30.1%`, `+40% to +101%`, `0/156`,
  `≥12 of 23`, SDK #571, shipmates `L2.5`). All verified and restored. Budget a pass over what an adversarial
  pass *removed*, not only what it left.
- **`spikes/HANDOFF.md` runs stale and has misled three separate pieces of work.** Its "HEAD is `ab58733` and
  nothing was committed" claim was corrected in this session; assume other status lines in it are also behind the
  tree and check against `git log` before acting on one.
- **Bash heredocs break on apostrophes and backslash escapes** in this environment. Use the Write tool for any
  file with prose or regex in it.
- **Three pre-registered criteria in this programme cannot be decided by their own registered sample size** —
  P1's α rule (F36), P7's criterion 1 (F37), and now §1.4's 0.95 bar (F39: `ui` is 7/7 and still cannot clear
  it; ~60 positives would be needed). Treat that as a pattern when writing the next pre-registration, not as
  three coincidences.

## House rules

Evidence discipline is the point of this repo. Every load-bearing claim carries a finding number or a cited
source, tagged `[measured]` / `[documented]` / `[inferred]`. A rate carries a Wilson interval or says the
interval is not computed. A saturated metric is reported as saturated, a powerless criterion as powerless, and a
`?` is excluded from scoring rather than counted against anyone. Unknowns are discharged as a named spike with a
named consumer (ADR-0012), never as prose. Accepted ADRs are immutable — supersede, never edit, both sides
linked. Commit subjects state the finding, not the change. British spelling.
