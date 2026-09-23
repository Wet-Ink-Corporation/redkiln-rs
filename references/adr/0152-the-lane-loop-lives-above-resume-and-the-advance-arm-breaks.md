---
id: "0152"
title: "The lane's stage loop lives **above** `replay::resume`, in `rk-cli`: one `resume` per stage, one run record per stage, the successor supplied per stage — and `resume`'s new `AdvanceStage` arm **steps and breaks** rather than continuing into the next stage. `s7h` §7h.1's instruction to *\"continue the loop into the next stage\"* is not implementable as written: `inputs.stage` is fixed before the loop and is what `fsm::fold` reads on every iteration, so a `continue` re-folds the same exhausted stage with the same `Some(successor)` and the arm fires again — an unbounded append of `stage_transition` rows. The grain is not incidental but is in the identifier: `RunInputs::run_id` is `format!(\"{}-{}\", self.stage, &digest[..16])`, so advancing inside one call would file a later stage's events under a run record whose id names the FIRST stage, and that digest's bytes are what accepted `0151` pins"
status: proposed
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/replay.rs (`resume`'s pre-loop bindings at 1693-1707 and the loop's `fsm::fold(item.id(), &inputs.stage, &journal)` at 1767; `RunInputs::run_id` at 1078-1087; `RuntimeContext::advances_to` at 616-622 — *"supplied, never invented"*; the `OpenGate` arm at 1801-1808, the model this arm copies); crates/rk-cli/src/verbs/run.rs (`base_inputs` at 1047-1080, which never chains `advances_to`; `resume_and_read` at 1091); crates/rk-kernel/src/workflow/fsm.rs (`next_action`'s arm 4 at 1700-1707); references/adr/0151 (the pinned run-record digest); references/adr/0083 (the `run_id` preimage)
supersedes: []
superseded_by: []
related: ["0083", "0130", "0151", "0153", "0154"]
evidence: []
consumers: []
---

## Context and problem

Step 7h exists to make a lane advance. `s7h` §7h.1 states the mechanism in one sentence:

> `replay::resume` gains the `AdvanceStage` arm: append the transition, write its trailer commit, and
> **continue the loop into the next stage** rather than breaking.

**That sentence cannot be executed.** Everything `resume`'s loop reads about "which stage" is bound
once, before the loop, from a single immutable field:

- `replay.rs:1693-1694` — `let run_id = inputs.run_id()?; let rt = Runtime::new(ports, &run_id, &inputs.stage);`
- `replay.rs:1698-1699` — `let mut selection: Selection = select(&pack, …, &inputs.stage, &item)?;`
- `replay.rs:1707` — `journal.push(stage_entry(&inputs.stage, runtime));`
- `replay.rs:1767` — **inside** the loop — `let state = fsm::fold(item.id(), &inputs.stage, &journal)?;`

`inputs.stage` is a field of the recorded input set and never changes during a call. So the arm
`continue`ing produces this: fold the **same** stage, get the **same** exhausted plan, read the
**same** `Some(successor)` off the context, and return `AdvanceStage` again — appending a
`stage_transition` row on every pass, without bound. The instruction describes a loop that cannot
terminate.

Underneath that is a grain question the instruction does not ask. `RunInputs::run_id`
(`replay.rs:1078-1087`) is:

```rust
let digest = rk_types::digest::sha256_lp(&[SCHEMA.as_bytes(), self.stage.as_bytes(), &body]);
Ok(format!("{}-{}", self.stage, &digest.as_str()[..16]))
```

The stage is in the hash preimage **and is the identifier's own prefix**. A run record is per-stage by
construction, not by convention. Advancing inside one `resume` would file `implement`-stage events
under a record literally named `triage-<digest>`, and the byte-faithfulness of that digest's preimage
is what accepted `0151` went to the trouble of pinning.

## Decision drivers

- A loop that cannot terminate is not a design choice to be weighed; it removes one option outright.
- `RuntimeContext::advances_to`'s own contract already answers the ownership question
  (`replay.rs:616-617`): *"The successor stage … Same reason as [`Self::entered_from`]: supplied, never
  invented."* The kernel is told the successor; it does not compute one.
- The run-record grain is load-bearing for three accepted records (`0083`'s preimage, `0130`'s named
  fixture directory, `0151`'s pinned bytes). A change to it is a change to all three.
- `resume` already has the right shape for a boundary that ends a call: the `OpenGate` arm
  (`replay.rs:1801-1808`) reads the journal length, calls `rt.step`, extends from the delta, and
  breaks — *"the row is the boundary's, read back off the journal rather than rebuilt here to agree
  with it."*
- Golden constraint 2 (`run` NEVER WAITS) is untouched either way: advancing past a stage is not
  advancing past a gate.

## Considered options

**1. Re-derive inside `resume` (what §7h.1 literally asks).** Recompute `run_id`, `Runtime`,
`selection`, `stage_entry`, `cap` and the fold's stage per iteration, holding the current stage in a
mutable local. It makes one process equal one lane, which reads well. It also redefines the run record
from per-stage to per-lane, makes the stage-prefixed id a lie for every stage after the first, and puts
pressure on `0151`'s pinned digest fixture — a cost paid to buy a shape the CLI can have for free.

**2. Advance in `resume` but return after one boundary.** Append the transition and return, letting the
caller decide. This is nearly the chosen option, but it splits the decision: the caller must still own
the loop, so the only question left is whether the kernel or the caller holds the successor — and
`advances_to` has already answered that.

**3. The loop lives in `rk-cli`, one `resume` per stage (chosen).** `base_inputs` chains
`.advances_to(..)` for the current stage; `resume`'s `AdvanceStage` arm steps the transition and
breaks; `rk-cli` observes the advance, builds a fresh `RunInputs` for the successor, and calls again.
One run record per stage, each with an id that names its own stage.

## Decision

**1.** The lane's stage loop lives in `rk-cli`, above `resume`. One `resume` call per stage, one run
record per stage, and the successor supplied per stage through `RuntimeContext::advances_to`.

**2.** `replay::resume` gains an `AdvanceStage` arm that **steps and breaks**, modelled on the existing
`OpenGate` arm: read the journal length, `rt.step(&action, None, cap, now)`, extend `journal` from the
delta so the row is the boundary's own, then `break`. It does not `continue`.

**3.** Nothing in `resume` re-derives `run_id`, `Runtime`, `selection`, `stage_entry` or the fold's
stage. `inputs.stage` remains the one stage a call is about.

**4.** `s7h` §7h.1's *"continue the loop into the next stage"* is corrected by errata rather than
reinterpreted silently, because a reader who implements the sentence as written writes a
non-terminating loop.

## Consequences

The advance becomes reachable without touching the run-record grain, so `0083`, `0130` and `0151` are
all undisturbed and no fixture digest moves. A lane is then a sequence of run records, one per stage,
which is also what `redkiln cost` will read at `s7i` — `dispatch_completed` rows accumulate per stage
rather than in one undifferentiated record.

The cost is that `rk-cli` carries the loop, and therefore carries the termination condition: a stage
with no successor ends the lane. That is the same place the pack already resolves and the same place
the item's stage is read (`run.rs:550`), so no new authority is created.

**What this forbids**

- Re-deriving `run_id`, `Runtime`, `selection`, `stage_entry` or the fold's stage inside `resume`.
- An `AdvanceStage` arm that `continue`s the loop.
- Any lane loop that reuses one `RecordedRun` or one `run_id` across two stages.
- The kernel computing a successor for itself — it is an argument (`0153` decides where it comes from).
- Reading `s7h` §7h.1's "continue the loop" sentence as implementable.

## Evidence and confidence

**The non-termination is read off the code, not simulated.** `inputs.stage` is a field on `RunInputs`
with no setter reachable from `resume`; `fsm::fold`'s second parameter at `replay.rs:1767` is
`&inputs.stage`; `next_action`'s arm 4 (`fsm.rs:1700-1707`) tests `ctx.successor` and nothing else, and
`fsm_context` (`replay.rs:1546-1558`) rebuilds that context from the same `runtime` every iteration.
Each of the three is a single line and none of them varies with the loop. The conclusion follows from
those lines alone.

**The grain claim is verbatim, not paraphrased.** `run_id`'s body is quoted above from
`replay.rs:1078-1087`; the stage appears twice, once in the `sha256_lp` preimage and once as the
returned string's prefix.

**Measured 2026-09-22 at `fc45eeb`:** `NextAction::AdvanceStage` has exactly **one** construction in
any `src/` — `fsm.rs:1703` — and it is gated on `ctx.successor`; `base_inputs`
(`rk-cli/src/verbs/run.rs:1047-1080`) is the only production builder of `RuntimeContext` and never
chains `.advances_to(..)`, so `successor` is `None` on every real run and the arm has never executed
outside tests. `grep -rn "advances_to" crates/` returns four hits: the builder, `read_run`'s replay of
a recorded value, and two **test** fixtures (`tests/it/resume.rs:158`, `tests/it/replay.rs:314`).

**Confidence, and what is NOT asserted.** This record decides where the loop lives; it does not assert
that a lane reaches any particular stage. The walk beyond the first boundary is blocked by absences
this record does not touch and `0156` enumerates. It also does not assert that one run record per stage
is the right grain for Phase 2 — only that it is the grain this tree already has, in an identifier, and
that 7h is not the stage to move it.
