---
id: "0167"
title: "The Codex Harness adapter is **Phase 4**, and ships only after S-CODEX passes, as ADR-0007 decides. Supersedes ADR-0094 decision 2's *Codex is Phase 2* sentence in part, toward ADR-0007, which is accepted and has never been superseded. The rest of ADR-0094 decision 2 stands, AC-499's phase clause included."
status: accepted
date: 2026-09-24
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 decision 22 (Phase 4, after S-CODEX; supersedes ADR-0094 d2's phase clause in part toward ADR-0007; E-0094-2 records that E12 was taken without ADR-0007's phase sentence in view) and D21-41 (f) (the conflict raised and not settled there); spikes/DECISIONS-2026-09-12.md E12; references/adr/0007 Decision; references/adr/0094 decision 2; references/adr/ERRATA.md E-0094-1; runbook/phase-1/s8a-records-and-red-checkpoints.md §8a.6
supersedes: ["0094"]
superseded_by: []
related: ["0003", "0007", "0010", "0092"]
evidence: []
consumers: []
---

## Context and problem

Two accepted records give the Codex adapter different phases.

- **ADR-0007** (2026-09-05), `## Decision`: *"Per the phasing table (§7), the Codex adapter ships in Phase 4
  ("reach"), and only after S-CODEX passes."* It has never been superseded.
- **ADR-0094 decision 2** (2026-09-11): *"The Harness port's second real adapter (Codex) is Phase 2 and stays behind
  the CI decision ADR-0010 defers, as a named open item with `rk-adapter-codex` as consumer."*

On 2026-09-12 the runbook alignment review raised the phase as **E12** (DECISIONS-2026-09-12). Its question named
ADR-0094's Decision, ADR-0094's own Evidence sentence and design `:1905`. It took option A, *"Phase 2 — ADR-0094
decision 2 stands as written. No supersession"*, and wrote **E-0094-1** against ADR-0094's Evidence sentence. E12's
row cites ADR-0094 and the design only. ADR-0007's phase sentence was not in view, and E12 left two accepted
records disagreeing.

D21-41 (f) retired `rk-story-codex-adapter` as the story-lane candidate on ADR-0007's ground and raised the
conflict separately. D21-42 decision 22 settled it: *"Phase 4, after S-CODEX."*

The design already says Phase 4 in its §10.2 phase table. Its §10.4 S-CODEX row, and seven register sites, say
Phase 2 *(corrected 2026-09-25 at `s8a` fix round 4: "six"; AC-239's clause was missed, decision 5)*.

## Decision drivers

- **Two accepted `## Decision` sentences contradict each other**, and ADR-0092 rule 1 settles that only by
  superseding one of them in part.
- **ADR-0007 is the older and the more specific record on this question.** It ties the phase to S-CODEX's result,
  and S-CODEX is unrun.
- **Nothing in Phase 1 depends on the answer** (E12's own reading), so the correction moves no code and no
  checkpoint's subject.
- **E12's error is in what it read, not in any Decision**, so it is recorded as an erratum, not a supersession.

## Considered options

### Option — Phase 4, after S-CODEX, toward ADR-0007 (taken)

Supersede ADR-0094 decision 2's phase sentence in part, and let ADR-0007 govern. Taken: it is D21-42 decision 22,
and it leaves one phase on the record.

### Option — Phase 2, superseding ADR-0007 in part

Rejected. It would move the older record's decision that the adapter follows the spike, on no new evidence.
S-CODEX is still unrun.

### Option — leave both standing and read the runbook phase-agnostically

E12's second alternative. Rejected: it leaves two accepted records disagreeing, which is the state ADR-0092's
precedence rule exists to end.

## Decision

**1. What this record supersedes, in part.** ADR-0094 decision 2's sentence *"The Harness port's second real adapter
(Codex) is Phase 2 and stays behind the CI decision ADR-0010 defers, as a named open item with `rk-adapter-codex`
as consumer"*, **as to its phase only**. The Codex adapter is **Phase 4**, and ships only after S-CODEX passes
(ADR-0007).

**2. What stands.** Every other clause of ADR-0094 decision 2 stands: the "named real adapter, in the phase design
§10.2 assigns" reading of ADR-0003's column, the Phase-1 `{fake, real}` instantiation, `Tracker` and `DocsSite` fake
only until Phase 3, **AC-499's phase clause**, the deferral of the CI decision to ADR-0010, and `rk-adapter-codex` as
the named consumer. ADR-0094's decision 3 and its forbids bullets stand, and so does decision 1 except its clause *"`change_summary`
remaining an opaque non-finalised newtype until S-BIN (ADR-0093 decision 4)"*, which ADR-0165 supersedes in part
*(corrected 2026-09-25 at `s8a` fix round 1, before acceptance: this said "decisions 1 and 3 … stand", which
contradicted ADR-0165)*. ADR-0094 stays `accepted`.

**3. ADR-0007 is not touched.** Its decision and its forbids bullet (no non-Claude adapter behind a gate until it
passes E3-shaped checks) stand as written.

**4. E12's reading is recorded as an erratum.** **E-0094-2** in `ERRATA.md` records that DECISIONS-2026-09-12 E12
was taken with ADR-0007's phase sentence out of view. It corrects no Decision.

**5. The phase-bearing texts take dated corrections, not rewrites.** The design's §10.4 S-CODEX row, the five
acceptance-register sites (the `| 5 | The second harness |` row, AC-239's phase clause, the area's *"Phase-1
subset"* note, AC-496's body and the *"Phasing means roughly half the area"* bullet), the two success-measure sites
(SM-148's baseline correction and its table row) and the runbook's Codex exclusion reasons each carry a dated note
citing this record. *(Corrected 2026-09-25 at `s8a` fix round 4, before acceptance: this said "the four
acceptance-register sites" and omitted AC-239, whose clause names "the second Harness adapter", not Codex, so the
Evidence regex below did not see it. ERRATA E-0094-1's "Codex = Phase 2" clause is corrected by E-0094-2.)*
`rk-story-codex-adapter` is retired at `s8j` (D21-41 (f)), not here.

## Consequences

**Positive**

- One phase for the Codex adapter on the record, and it is the one tied to S-CODEX's result.
- E-0094-1's correction of ADR-0094's Evidence sentence stays true: the phase is decided, now by ADR-0007.

**Negative**

- Every register and runbook line that learned "Phase 2" from E12 carries a dated note, and a reader must follow it
  to this record.

**What this forbids**

- Forbids scheduling, building or shipping `rk-adapter-codex` in any phase before Phase 4.
- Forbids shipping it in Phase 4 before S-CODEX has passed.
- Forbids citing ADR-0094 decision 2 for the Codex adapter's phase.

## Evidence and confidence

- [documented] ADR-0007's `## Decision` and ADR-0094 decision 2, quoted above; both `accepted`, and ADR-0007's
  `superseded_by` is `[]` [measured 2026-09-24].
- [documented] DECISIONS-2026-09-12 E12: its question cites *"ADR-0094's Decision says Phase 2, its own Evidence says
  it does not decide the phase, and design `:1905` says Phase 4"*, and its grounding cites ADR-0094 and the design.
  ADR-0007 appears in neither.
- [measured 2026-09-24 at `b8e7aa9`] `grep -nE 'Codex[^|]*\bPhase 2\b'` over the acceptance-criteria and
  success-measures registers matches **6** lines, the six listed in decision 5; the design's §10.4 S-CODEX row
  reads *"**Phase 2 is the phase** (ADR-0094 decision 2, 2026-09-11 …)"*. *(Corrected 2026-09-25 at `s8a` fix
  round 4: that regex counts lines naming Codex, not the phase-bearing texts. AC-239's clause, *"the second Harness
  adapter is **Phase 2**"*, is a seventh site. `grep -nE '(Codex|[Hh]arness adapter)[^|]*\bPhase 2\b'` over the
  same two registers matches **7** lines, the seven in decision 5 [measured 2026-09-25].)*

Confidence is high. The conflict is textual and both sentences are quoted whole.
