---
id: "0125"
title: The event-kind table gains `stage_exit_refused` — the verify gate's refusal to let a stage exit over an unpinned AC, which AC-346 says is an event and no table defines
status: accepted
date: 2026-09-17
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-346 (THEN *"the exit is refused and the unpinned AC ids are named in the refusal AND the refusal is an event, not only a console line"*), AC-345 (*"an assertion that stage exit (AC-346) is still refused for that AC"*), AC-357 (the human answer *for that AC* that lifts the refusal), AC-337 (`not-feasible(reason)` *"is not `pinned`"*), AC-335, AC-334, **cited by id and named clause** (the register's line numbers moved +39 on 2026-09-13; AC-346 reads at :5400-5409 on this tree, `M` against `1ad4377`); references/2026-09-08-success-measures.md :708 (*"it does not answer AC-346 — a `mutation-survived` AC stays unpinned until a discriminating mutant flips or AC-357's human gate answers that AC"*); references/2026-09-08-architecture-design.md §5 :699 (the closed-enum sentence) and :701-716 (the table — no stage-exit row; `selection_ineffective` at :716 is the verify gate's only row); references/adr/0104 decision 4 (*"an AC whose mutant survived stays unpinned until a discriminating mutant flips or a human gate (AC-357) answers that AC explicitly"*) and its forbids (*"treating a quarantine as an answer to AC-346"*); references/adr/0019 (the per-AC discriminating test is the norm; suite-green is not evidence); references/adr/0095 (`RefusalClass` is closed at four — none is an unpinned AC); references/adr/0116 :40-41 and :95-99 (the minting rule); references/adr/0120, 0123, 0124 (the three precedents this record follows clause for clause); references/adr/0105 clause 7 (`rk-types` owns the enum); references/adr/0109 decision 2 (the deleted-database test); crates/rk-types/src/event_kind.rs and crates/rk-types/tests/it/event_kinds.rs at `1ad4377` (36 variants); crates/rk-kernel/src/workflow/fsm.rs :518-527 (`HaltReason` closed at two) and :1696-1700 (`AdvanceStage` with no ledger check); runbook/phase-1/s7b-ii-gates.md §7b-ii.9 and §7b-ii.11; runbook/00-preamble.md :55; the operator's decision of 2026-09-17 in the Step 7b-ii session (asked once, with the alternatives named: mint by record, fold into an existing kind, or arm the event half to 7d — the first was taken)
supersedes: []
superseded_by: []
related: ["0019", "0095", "0104", "0105", "0109", "0116", "0120", "0123", "0124"]
evidence: ["F25", "F31", "F24"]
consumers: []
---

## Context and problem

**AC-346's refusal is an event the closed table does not carry.** The criterion: *"GIVEN an AC that is neither pinned
by a flip-verified assertion nor answered at a human gate (AC-357) explicitly for that AC — a quarantine of the
survivor's finding (AC-345) is not an answer … WHEN the stage attempts to exit THEN the exit is refused and the unpinned
AC ids are named in the refusal AND the refusal is an event, not only a console line."* Its *Verified by* asks for
*"an assertion that the refusal event names the AC id"*.

`git grep -n 'stage_exit_refused\|exit_refused\|exit is refused'` over `references/`, `runbook/` and `crates/` returns
AC-346, AC-345's cross-reference, SM :708's gloss and `s7b-ii`'s restatement — **none in design §5's table and none in
`crates/`** [measured 2026-09-17, `1ad4377`]. `EventKind` carries **36** variants (`ALL: [Self; 36]`) [measured].
Design §5:699 is the constraint: *"the `EventKind` enum is closed, a kind not in this table is a schema change reviewed
as one."*

**No existing kind carries it without deciding something else.** `step_refused` is a *step's* refusal and its
`RefusalClass` is closed at four (ADR-0095: `NotAdopted | NotIsolated | Unsupported | Overage`), none of which is *an
AC has no evidence*; a fifth class would be a supersession of 0095 for a fact that is not about a step. `HaltReason`
(`Refused { class } | GateExpired { token }`) is a run's terminal state, and AC-346's refusal is not terminal — the
item stays in the stage, the survivor's mutant may yet be re-declared, AC-357's gate may yet answer. `gate_opened` is
what AC-357 writes *after* the refusal, for the human to answer; it is the consequence, not the refusal. `review_block`
is the review gate's word about a finding; AC-346 is the verify gate's word about an AC, and ADR-0104 decision 4 keeps
the two apart on purpose (*"quarantine is a review-gate state … not an answer to AC-346"*). A refusal filed under any of
them would make some count over kinds count the wrong rows.

**ADR-0123 asked this question once and the operator answered it**, ADR-0124 applied the answer to five, and this
record applies it to one more: a kind a criterion names and no table defines is minted by record, never in the
runbook or as a kernel constant. The operator was asked again for this kind because the alternatives were real
(fold into a halt; arm the event half) and each would have weakened a THEN the register states in full; the answer
was to mint.

## Decision drivers

- A criterion naming an event no table defines cannot be built as worded (ADR-0116's driver, fifth application).
- ADR-0019: *"'Reviewed and green' is not evidence that a criterion is pinned."* The refusal is where that decision
  bites the run — the one place a green suite over an unpinned AC is refused rather than passed — and a decision that
  bites only on a console line is not replayable, not countable and not auditable (AC-313's shape).
- AC-346 names the ids *in the refusal*; the human who then answers AC-357's gate for *that AC* needs the list the
  refusal carried, not a re-derivation.
- The quarantine trio (ADR-0124) exists so the review gate can stop re-raising a finding without touching the AC's
  evidence class; this kind exists so the verify gate can say, in a row, that the AC is still unpinned. Without it
  ADR-0104 decision 4 is enforced by nothing durable.

## Decision

**1. One tier-2 event kind**, with its row in design §5's table:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `stage_exit_refused` | 2 | `item, stage, unpinned: [ac ids], statuses: { ac id → pinned \| mutation_survived \| unpinned \| not_feasible \| human_answered }` | the verify gate's stage-exit check, once per refused attempt to exit (AC-346) |

`unpinned` is the list AC-346 names — every AC whose ledger status is not `pinned` and not `human_answered`;
`statuses` is the ledger the refusal was computed from, so a reader sees *why* each id is in the list without a second
read. **The payload's members are the register's** (`unpinned AC ids`) plus the ledger row that justifies them; this
record fixes the name and the obligation, not the Rust types (ADR-0120 decision 1's posture).

**2. Tier 2, and the deleted-database test is why** (ADR-0109 decision 2). The refusal is *derived* — the AC ledger
over the declared mutations, the flip predicate's outcomes and the recorded human answers, recomputed at every exit
attempt (AC-359: *"the AC ledger is recomputed after re-open"*; AC-336: at N+1 it *"recomputes rather than
inherits"*). Losing the row loses the record that an exit was attempted and refused, never the refusal itself: the
next attempt recomputes and refuses again. It carries no trailer; no fourth trailer axis is minted.

**3. The refusal is written by the same code path that computes it, and nowhere else** — the exit check returns the
row with its verdict, as `review_block` does for the block (ADR-0124 decision 3). A refusal computed without its row
or a row written for an exit that was not refused is the count of refusals disagreeing with the count of refused
exits.

**4. Quarantine is not an input to the check** (ADR-0104 decision 4; AC-345; AC-346's GIVEN). The exit check reads the
AC ledger; the ledger reads declared mutations, flip outcomes and human answers *for that AC*; neither reads a
finding's quarantine, the item's own suite verdict (AC-335), a review's `Findings` (AC-342) or a CI status (AC-341).
That is enforced as an architecture lint over the verify module, in `module_boundary`, not stated here as prose.

**5. The table row and the `rk-types` variant land in the same change** (ADR-0116 clause 5; ADR-0120 decision 6;
ADR-0123 decision 4; ADR-0124 decision 4). `EventKind` gains one variant; `ALL` goes `[Self; 36]` → `[Self; 37]`;
`wire_name` and `tier` gain arms; the pin test gains a case; the kernel's three exhaustive matches route it to their
no-state-change arms. **The golden schema snapshot does not move** (`EventKind` is in neither snapshotted set;
re-measured at ADR-0123 and ADR-0124).

**6. The consequence for Checkpoint 5 is taken, not routed around**: the row goes `passed cb16371, re-run pending
7b-ii rk-types` in the commit that lands the variant, and the hardened block is re-run before that commit.

**7. What this record does NOT decide**: AC-357's gate carrier — how a human answer names *that AC* (the gate
record's `kind` string or a payload member; no gate event carries an AC id today) is the verify slice's to build
against AC-357's *machine-checkable part* and is not a kind; AC-455's *admissibility* refusal on entry (*"the stage is
refused with the AC id named, at zero model cost"*) — a different moment, a different criterion, and its register text
asks for no event, so none is minted for it here and if one is ever wanted it takes its own record; the `doctor` rows
over any of this (7d); and the rendering of the refusal (AC-323 — `rk-cli`, 7d).

## Considered options

### Option — carry the refusal as a `step_refused` with a fifth `RefusalClass`
Rejected: `RefusalClass` is closed by ADR-0095 for *step* refusals whose classes are about the dispatch (not adopted,
not isolated, unsupported, over budget); *an AC has no evidence* is about the item's evidence, not a step. It would
supersede 0095 in part to file a fact under the wrong noun.

### Option — carry it as a halt, `HaltReason::Unpinned`
Rejected: a halt is terminal for the run (`RunState::terminal()`); AC-346's refusal is not — the stage stays open,
AC-357's gate is the next action, and AC-359 recomputes the ledger after a re-open. Making it a halt would make
every unpinned AC a dead run.

### Option — build the refusal as a typed `Result` and ARM the event half to 7d
Rejected by the operator: it weakens a THEN the register states in full (*"the refusal is an event, not only a console
line"*) at the stage that owns the criterion, for no external reason — the kind is one line in a closed enum and the
record is this file.

### Option — mint in the runbook or as a kernel `const`
Rejected on mechanism and by name (`00-preamble.md`:55; ADR-0116:95-99; ADR-0120, ADR-0123, ADR-0124).

## Consequences

**Positive**
- AC-346 becomes implementable exactly as worded, and AC-345's third instrument (*"stage exit is still refused for
  that AC"*) has a row to assert on rather than a return value.
- ADR-0104 decision 4 acquires a durable enforcement point: the refusal that a quarantine cannot lift is a row.

**Negative**
- A closed enum moves for the fifth time in five days, by one. The hand-written pins move; every kernel doc that says
  *"closed at 36"* is stale again and is corrected in place, dated, in the same commit.
- Checkpoint 5 becomes provisional and is re-run (decision 6) for the fourth time in one day. That is the maintenance
  rule working, and the cost of minting one kind at a time is the cost of asking before building.

**What this forbids**
- Forbids refusing a stage exit over an unpinned AC without a `stage_exit_refused` row, or writing that row anywhere
  but the path that computed the refusal.
- Forbids a `stage_exit_refused` row that names no AC id, or names an id whose ledger status is `pinned` or
  `human_answered`.
- Forbids the exit check, or the ledger it reads, taking a quarantine, the item's suite verdict, a review's `Findings`
  or a CI status as an input (ADR-0104 decision 4; AC-335; AC-341; AC-342).
- Forbids carrying it at tier 1, on a trailer, as a `HaltReason`, or under `step_refused`.
- Forbids landing the variant without the design §5 row, the `ALL` count, the pin test and the Checkpoint 5 row in
  one commit.

## Evidence and confidence

- [measured 2026-09-17, `1ad4377`] the grep above: the name and the obligation appear in AC-346, AC-345 and SM :708;
  no row in design §5's table; nothing in `crates/`. `event_kind.rs` at 36 variants; `HaltReason` at two;
  `RefusalClass` at four; `next_action`'s `AdvanceStage` arm reads no per-AC ledger.
- [documented] AC-346's THEN and *Verified by*; ADR-0104 decision 4 and its forbids; ADR-0019's decision; ADR-0095's
  closed set.
- [measured, F25 / F31] why the refusal must exist and must not be lifted by a green suite: an ordering inversion left
  36/36 green (n=1 AC, one operator); four adjudicated defects survived fully green suites of 1,890/1,890 and
  1,907/1,907. [measured, F24] why the human answer is the only other exit: 6/6 rejections confirmed true at zero model
  cost, with severity a human's call.
- **Confidence:** the gap is measured; the name is this record's (the register names the obligation, not a wire
  name — `stage_exit_refused` follows the table's `<subject>_<verb>` spelling, as `selection_ineffective` and
  `review_block` do). The tier is *reasoned* from ADR-0109's test, as the three precedents reasoned theirs, and is
  not measured.
