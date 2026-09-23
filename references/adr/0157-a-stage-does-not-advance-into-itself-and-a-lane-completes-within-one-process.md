---
id: "0157"
title: "**A stage does not advance into itself**: `fsm::next_action`'s advance arm fires only when the supplied successor differs from the stage the fold is already in. `from` is the FOLD's stage and `to` is the SUPPLIED successor, and across invocations the two diverge — the composition root computes the successor from the item record, whose `stage` never moves, while the fold follows the `stage_transition` rows the previous run wrote. Measured against the real binary 2026-09-22: a second `redkiln run` over `rk-fix-doctor-test-count` appended **`{\"from\":\"analysis\",\"to\":\"analysis\"}`**, and a third would have appended another — one meaningless tier-1 row per run, without bound. **And the residual under it is named rather than closed: a lane completes WITHIN ONE PROCESS.** Across processes the run is now idempotent and STALLED — run 2 and run 3 exit 0 and append nothing, because `inputs.stage` is still the record's `triage` while the journal says `analysis`, and AC-518 forbids the orchestrator writing the new stage back to the user's checkout"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/workflow/fsm.rs (`next_action`'s advance arm — `from` is `state.stage`, the fold's; `to` is `ctx.successor`, the caller's); crates/rk-cli/src/verbs/run.rs (`base_inputs`, which resolves the successor from `resolved.stage`; the stage read at 550 — *"the stage is read once, from the record, and is not defaulted"*; the lane loop); crates/rk-kernel/src/replay.rs (`fsm::fold(item.id(), &inputs.stage, &journal)` — seeded with the input stage, then moved by the journal's own rows); references/2026-09-08-acceptance-criteria.md AC-518 (the orchestrator never writes the user's checkout); the measurement itself — three consecutive `./target/debug/redkiln run rk-fix-doctor-test-count --lane fix` invocations over a fresh store, 2026-09-22
supersedes: []
superseded_by: ["0161"]
related: ["0152", "0153", "0154", "0158", "0161"]
evidence: []
consumers: []
---

## Context and problem

ADR-0152 put the lane loop above `resume` and ADR-0153 made the successor come from the pack's declared
order. Both were asserted by unit fixtures and by seven mutants, and all of it was green. **Then the
real binary was run, and it found something no fixture had.**

Run one did what it should — the first lane advance ever taken on this tree:

```
$ ./target/debug/redkiln run rk-fix-doctor-test-count --lane fix
rk-fix-doctor-test-count advanced triage -> analysis
```

Run two appended this:

```
{"to":"triage"}
{"from":"triage","to":"analysis"}
{"from":"analysis","to":"analysis"}      <-- run two
```

**A transition from a stage into itself**, and one more on every subsequent invocation.

The mechanism is a divergence between two sources of "which stage", and neither is wrong on its own:

- **`to`** is `ctx.successor`, resolved by `base_inputs` from `resolved.stage` — the **item record's**
  stage, which is `triage` and never moves. `Pack::successor("triage")` is `analysis`, every time.
- **`from`** is `state.stage`, which `fsm::fold(item.id(), &inputs.stage, &journal)` seeds with the
  input stage and then **moves** as it folds the journal's own `stage_transition` rows. After run one
  the journal carries a transition to `analysis`, so run two's fold reports `state.stage = "analysis"`.

So run two was handed the successor `analysis` while already folding as `analysis`, and the arm — which
tested only that a successor was supplied — advanced.

No fixture could have caught it, and that is worth saying precisely rather than generally: every
`AdvanceStage` fixture in the workspace folds a journal that does **not** already contain a transition
into the supplied successor. The defect needs a *replayed* journal, which is what `rk-cli` hands
`resume` at step (9) and what no unit test was doing.

## Decision drivers

- A row whose `from` equals its `to` is not a transition. It is tier-1 by kind, so it is exactly the
  class of row that must never be written speculatively.
- Unbounded growth on repeated invocation is the failure ADR-0152 was written to prevent inside one
  process; the same failure reappearing across processes deserves the same answer.
- The equality is already the whole test: if the fold is in the stage the successor names, the journal
  has recorded that transition, so there is nothing left to do.
- The deeper divergence — record stage versus journal stage — cannot be closed here. AC-518 forbids
  writing the new stage back to `.redkiln/items/<item>.toml`, and `run.rs:550` states that the stage is
  read once from the record. Whichever way that is resolved, it is a decision about where an item's
  current stage LIVES, and it is bigger than this stage.
- `00-preamble.md`'s rule that a decision is not taken by implementing it applies to the residual: it
  is named and bounded here, not quietly worked around.

## Considered options

**1. Guard the arm on `to != state.stage` (chosen).** One conjunct, and it reads as what it means: a
stage does not advance into itself. It makes repeated invocation idempotent and leaves every other arm
untouched.

**2. Compute the successor from the fold rather than from the record.** `base_inputs` would have to
fold the journal before building `RunInputs`, which inverts the order — the fold is `resume`'s, and the
context is an *input* to it. It also does not fix the arm: two callers could still disagree and the FSM
would still advance on a successor equal to its own stage.

**3. Write the advanced stage back to the item record.** It would make the record and the journal agree
and let a lane continue across processes, which is the residual's real fix. It is forbidden here:
AC-518 is asserted by Checkpoint 7h itself, and `rk-cli`'s own tests pin that the product repo's commit
count does not move.

**4. Refuse when the record and the fold disagree.** Turns a silent stall into a loud one, which is
better, but it refuses the ordinary case: after any successful advance the two legitimately differ
until something reconciles them, and nothing does yet.

## Decision

**1.** `fsm::next_action`'s advance arm fires only when `ctx.successor` names a stage **different from
`state.stage`**. When they are equal the arm is skipped and evaluation falls through to `Done`.

**2.** The documented evaluation order is updated with the body, in the same commit, as ADR-0154
required of its own change.

**3.** The test replays the first run's journal as `prior`, because that is the only shape that
distinguishes the guard — a fresh journal cannot. It asserts that the second `resume` reports **no**
boundary and appends **no** `stage_transition`, and that no row carries `from == to`.

**4. The cross-process residual is RECORDED, not closed.** A lane completes **within one process**: the
loop advances stage by stage until a stage cannot proceed. Across processes, a second invocation reports
`Done` and exits 0 having done nothing, because `inputs.stage` is the record's and the journal has moved
past it. Measured: runs two and three both exit **0** and append **nothing**.

**5.** No document may describe a lane as resumable across processes, or read run two's exit 0 as
progress.

## Consequences

Repeated `redkiln run` over one item is now idempotent at the stage boundary: two rows after three
runs, and no self-transition. That is a precondition for anything that reads the journal, `redkiln
cost` at `s7i` included.

The stall is the honest cost. A lane's progress is durable in the journal and invisible to the next
invocation, so the lane advances exactly as far as one process can carry it and then reports success
for ever after. **Exit 0 on run two is not evidence of a completed lane**, and clause 5 forbids reading
it as such. The route out is a decision about where an item's current stage lives — the record, the
journal, or a reconciliation between them — and it is named here as OWED.

**What this forbids**

- An `AdvanceStage` whose `to` equals its `from`.
- Writing an item's advanced stage back to `.redkiln/items/` (AC-518).
- Describing a lane as resumable across processes, or a lane's completion as anything other than
  within-one-process.
- Reading a second invocation's exit 0 as progress, or as evidence that the lane finished.
- Asserting this guard with a fixture whose `prior` journal is empty — such a fixture cannot
  distinguish the guarded arm from the unguarded one.

## Consequences addendum — why this record exists at all

It is worth recording that ADR-0152's and ADR-0153's own instruments were **all green** when this was
found: six lane fixtures, seven mutants each diffed and read, four suites, fmt and clippy. The defect
was found by running the binary. A stage that asserts an executor's semantics and never executes it is
asserting the fixtures.

## Evidence and confidence

**The defect and the repair were both measured against the real binary, not against a fixture.** Three
consecutive invocations over a fresh store (`.git/redkiln/store.sqlite3` moved aside first, so the
count starts at zero):

| | before the guard | after the guard |
| --- | --- | --- |
| run 1 | `advanced triage -> analysis`, exit 3 | `advanced triage -> analysis`, exit 3 |
| run 2 | appended `{"from":"analysis","to":"analysis"}` | exit **0**, appended nothing |
| run 3 | would append another | exit **0**, appended nothing |
| `stage_transition` rows | 3 and growing | **2**, stable |

The two surviving rows are `{"to":"triage"}` (the stage entry, which carries no `from` — the one case
`stage_entry` says it cannot state) and `{"from":"triage","to":"analysis"}` (the boundary).

**The unit fixture kills a mutant that removes the guard**, attributed to a line and a message rather
than to "the test went red": with `&& to != state.stage` deleted, `lane_order.rs:448` reports
`left: Some("implement")` against `right: None` — *"the second run must take NO boundary"*.

**Why the exit is 3 and not 0 on run one** is a different finding and is not this record's: the lane
advanced into `analysis` and the stage's bindings refused, which ADR-0158 and the corrected ADR-0156
account for by name.

**Confidence, and what is NOT asserted.** This record does not assert that the lane reaches any stage
beyond the first boundary, and it does not assert that exit 3 on run one is the right exit code — only
that the boundary is taken once, durably, and never again. It says nothing about the tier-1 store layer,
which ADR-0158 measures separately, and nothing about whether a `Story:` trailer accompanies the
transition — that is ADR-0155's and §7h.2's.
