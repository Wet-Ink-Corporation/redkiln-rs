---
id: "0175"
title: "**`on_false` as iteration** (`s8d`'s record R3): a `check` that does not hold is routed through its `on_false`, classified by the one classifier over ADR-0103 clause 6's closed set — a `{ to, max }` loop-back re-enters its earlier target under a NEW loop-back frame and the earlier frames' committed step activations stand; after `max` loop-backs, with `\"fail\"`, or with no `on_false`, the walk refuses by name, as `StepFailedAfterCommit` when a step committed (ADR-0144) and as a refusal otherwise, the exit code being the class's own; `\"continue\"` proceeds; and `{ to = \"deviate\" }` refuses by name, naming ADR-0046's route OWED (the narrowing of D21-42 4d that D21-48 X7 took). An escalate gate's re-entry is a loop-back frame too, and every loop-back and `repeat` frame is a pass charged on its own, a `for_each` once. Executes `0031`, `0044`, `0103` clause 6 and `0144`; supersedes nothing"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R3's row), §8d.3 (*The attempt charge per frame (m14)*, the escalate re-entry's binding), §8d.4 and §8d.10; spikes/DECISIONS-2026-09-21.md D21-42 4d (*"**`on_false` is honoured as iteration.** A `{to,max}` loop-back runs under a new iteration path, keyword forms follow the schema, and exhaustion or no `on_false` refuses as `StepFailedAfterCommit` (ADR-0144). Committed steps stand."*) and 3b (*"The iteration path is the stack of for_each index, repeat count and loop-back count."*), D21-47 (the review's m13 and m14), D21-48 X7 (*"**Refused by name, naming ADR-0046's route OWED.** R3 states the narrowing of D21-42 4d."*) and X8, D21-64 ruling 10 (this record's flip condition); references/adr/0031, 0044, 0046, 0103 (clause 6), 0122 (decision 1), 0144; references/2026-09-07-activity-graph-schema.md §1 (Loop) and §3 (Escalation); crates/rk-kernel/src/runtime/step.rs (`run_activity_steps`' loop executor, `Runtime::on_false`, `after_step`, `gate_arm`, `fan_out`), crates/rk-kernel/src/runtime/reference.rs (`Scope::open_frame`), crates/rk-kernel/src/runtime/activity.rs (`ActivityReport::pass_charges`), crates/rk-kernel/src/catalog.rs (`Step::on_false`, `Step::on_exhausted`, `repeat_body_end`) and crates/rk-kernel/src/workflow/fsm.rs (`RunState::gates_of`) at `s8d`'s build (package P5)
supersedes: []
superseded_by: []
related: ["0031", "0037", "0042", "0044", "0046", "0053", "0055", "0103", "0122", "0144", "0173", "0174"]
evidence: []
consumers: []
---

## Context and problem

Until this record a `check` step that did not hold ended the walk. `s8d`'s resolver (ADR-0173 decision 6) made a
`check` decide from its resolved inputs, and routed `Tri::False` to a by-name refusal: *"did not hold; its `on_false`
(…) is routed by `s8d` §8d.4 (R3)"*. Every shipped `on_false` therefore did the same thing whatever it declared.
Measured over `.redkiln/catalog/*.toml` at `ca8f387` (`s8d` §8d.4): **94** `on_false` values — `{ to = …, max = N }`
**66**, `"continue"` **15**, `"fail"` **13**, `{ to = "deviate", … }` **0** — all inside ADR-0103 clause 6's closed
set as `predicate::classify_on_false` reads it. A loop-back, the form two thirds of them take, was never a loop.

Four accepted records define the forms and one defines the failure:

- **ADR-0031**: *"`on_false ∈ { <earlier-declared step>, continue, fail }` — loop-back only."*
- **ADR-0044**: *"`on_false = { to = "<earlier step>", max = N }`; the scalar form is an error."*
- **ADR-0103 clause 6**: *"`on_false` is closed: `{ to = <earlier step id>, max = N }` | `"continue"` | `"fail"` |
  `{ to = "deviate", … }` (ADR-0046's form), with `max` required only for the loop-back form"*.
- **ADR-0046**: *"`on_false.to = "deviate"` with `deviate = { kind = skip|downgrade|add, target = <ArtifactKind>,
  claimed = <predicate> }`; the kernel handles it per catalog §2.5 step 7."*
- **ADR-0144** decision 3: *"`dispatch_through` asks `step_error()` BEFORE `committed()` … It returns
  **`ReplayError::StepFailedAfterCommit { step_id, activity_id_at_version, class }`**."* Decision 6: *"No row of §7d.4's
  exit table is added, moved or renumbered. The exit code is `Exit::from(&class)`, the class's own."*

The ledger took the behaviour. D21-42 4d and 3b are quoted in `source`. D21-48 X7 took the one form the walk does not
execute: *"Refused by name, naming ADR-0046's route OWED. R3 states the narrowing of D21-42 4d."* D21-47's review
added m14: the attempt charge per iteration frame must be stated (`s8d` §8d.3, *The attempt charge per frame*).

## Decision drivers

- **A declared loop that never loops is an invisible non-event**, ADR-0135's defect one construct over: the catalog
  says *"try again, at most N times"* and the run said *"refused"*.
- **Nothing committed moves or is lost** (D21-42 4d, *"Committed steps stand"*; ADR-0144's forbids). A loop-back
  re-runs step activations under a new frame; it never re-uses or overwrites an earlier frame's commit key (ADR-0174
  decision 4).
- **A refusal carries its class and reaches the operator** (ADR-0144): an exhausted loop is not a success and not a
  new exit row.
- **No unbuilt route is faked**: ADR-0046's deviation is adjudicated by catalog §2.5 step 7, which no mid-walk code
  path reaches, and no shipped activity asks for it.

## Considered options

### Option — Route every closed form in the walk, refuse `deviate` by name (taken)

D21-42 4d as D21-48 X7 narrows it. The four forms the catalog uses execute; the fifth, used nowhere, refuses by name
and names its owed route.

### Option — Build ADR-0046's `deviate` route now

A mid-walk `RequestDeviation` exit and a `step7` re-adjudication. Rejected by D21-48 X7: zero shipped uses, and the
route is an amendment of the stage's selection, not an iteration of the activity's graph.

### Option — Keep the by-name refusal for every false `check`

ADR-0173 decision 6's interim state. Rejected by D21-42 4d; it leaves 66 declared loop-backs unexecuted. *(dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting round 2's records minor.)* 66 is the census at `ca8f387`. Re-measured by a `tomllib` census over `.redkiln/catalog/*.toml` (`scratchpad/s8d/r2fix-8d-onfalse.py`, 102 files at each pin): `ca8f387` **66 / 15 / 13 = 94**, `d46b2eb` (this stage's entry) **65 / 15 / 13 = 93**, and HEAD and the working tree **65 / 15 / 12 = 92** (loop-back / `continue` / `fail`). The drop from 13 `fail`s to 12 is D21-67 ruling 1, which deleted `migration-dry-run`'s policy check and its `fail`. The same edit moved `quality_block.rs`'s ac527 census of `check` steps with an `expr` from `checks=93` to `checks=92`. On today's catalog the figures read 65 loop-backs, 15 `continue`s and 12 `fail`s.

## Decision

**1. The walk routes a `check` that does not hold through its `on_false`.** The step's declared `on_false` is carried
verbatim (`catalog::Step::on_false`) and classified at that moment by the one classifier,
`predicate::classify_on_false`, so ADR-0103 clause 6's closed set is enforced where it is executed. A form outside the
set refuses by name and is never read as another form. A LONE step (`Runtime::execute_step`, outside any walk) keeps
the by-name refusal: no loop exists there to route it into.

**2. `{ to, max }` re-enters `to` under a new loop-back frame** (ADR-0031, ADR-0044). `to` must be a step declared
before the `check` in the same activity; any other target refuses by name. The walk moves back to `to` and runs every
step from it onward again under the iteration path `[loop_back:<n>]`, where `n` counts the walk's re-entries in order
(D21-42 3b's loop-back count). Every step activation that commits in that frame commits under its own commit key
(ADR-0174 decision 4); the step activations committed in earlier frames **stand** and are served, never re-dispatched,
when a later walk re-enters the activation. A step's `${…}` of the re-run steps binds the frame that re-entered it: the
PREVIOUS frame's value for a self or feedback reference (ADR-0055, ADR-0042), `null` on the first pass.

**3. After `max` loop-backs through the same `check`, and for `"fail"` and for no `on_false`, the walk refuses by
name.** The refusal names the step and why (*"did not hold, and its `on_false = { to = …, max = N }` is exhausted after
N loop-back(s)"*; *"… the keyword `fail`"*; *"… it declares no `on_false`"*). Its class is `StepError::Unsupported`.
When a step of the activation committed — in this walk or an earlier run — it travels on `step_error` with the commit
kept (ADR-0144 decision 2), which `dispatch_through` maps to `ReplayError::StepFailedAfterCommit`; when none committed
it is a refusal (`ActivityReport::refused`). The exit code is the class's own row (ADR-0144 decision 6); no exit row is
added. `StepError::Unsupported` with *"`on_false` has no executor"* is not reused.

**4. `"continue"` proceeds** to the next step (ADR-0103 clause 6).

**5. `{ to = "deviate", … }` refuses by name, naming ADR-0046's route OWED** (D21-48 X7). **This narrows D21-42 4d's
*"keyword forms follow the schema"*** as X7 took it: the schema's deviation form is recognised, classified and refused,
not executed. ADR-0046 is not superseded — its route is owed, not withdrawn — and no mid-walk `step7` amendment is
built.

**6. An escalate gate's re-entry is a loop-back frame.** When a step's `repeat` exhausts with `on_exhausted =
"escalate"` and the walk reaches the `gate` step asking that step's `escalate` question (schema item 15's pattern;
ADR-0173 decision 5), an ANSWERED reach re-enters the escalating step (schema §3: *"resumes **into the same step**"*)
under a new `[loop_back:<n>]` frame, the n counting re-entries as decision 2's does. D21-42 3b names three frame forms
and this adds no fourth. The walk's n-th escalation through a gate kind reads that kind's n-th reach that was not voided
(AC-252 re-opens a void reach at the next): answered, it binds the answer and re-enters; expired, the walk proceeds
past it (ADR-0017's terminal path); open, the run is at that gate; absent, the gate is opened. So a later step's value
from the escalating frame — a gate's answer or a later **tool** step's output — binds the resumed pass as its previous
frame (D21-48 X8).

**7. The attempt charge per frame (m14).** A pass is an iteration path with its `for_each` frames removed. Each
loop-back frame and each `repeat` frame is a pass of its own for AC-021's per-pass count; a `for_each` counts once,
whatever its element count (AC-022, ADR-0053). Every agent or tool step the walk EXECUTES in a pass charges that pass
through `budget::charge_for_each`, the one charge site AC-022 names; a step SERVED from its commit on re-entry charges
nothing. The walk reports the charges per pass (`ActivityReport::pass_charges`), and `budget::fold` accumulates them
across passes (ADR-0122 decision 3). ADR-0122 decision 1 stands for the inner attempts: *"An inner harness attempt
charges the activity's attempt budget, one charge per recorded `system/api_retry` event, in the pass that dispatched
it"*; its charge site is `budget::charge_observed_inner_attempts`. ADR-0037 is a load-time lint on the declared field
and does not move.

## Consequences

**Positive**

- The 66 shipped loop-backs loop, the 15 `continue`s proceed and the 13 `fail`s refuse by name, each as its catalog
  declares; a refusal after a commit still reaches the operator with its class. *(dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting round 2's records minor.)* The three counts are the census at `ca8f387`. Re-measured by a `tomllib` census over `.redkiln/catalog/*.toml` (`scratchpad/s8d/r2fix-8d-onfalse.py`, 102 files at each pin): `ca8f387` **66 / 15 / 13 = 94**, `d46b2eb` (this stage's entry) **65 / 15 / 13 = 93**, and HEAD and the working tree **65 / 15 / 12 = 92** (loop-back / `continue` / `fail`). The drop from 13 `fail`s to 12 is D21-67 ruling 1, which deleted `migration-dry-run`'s policy check and its `fail`. The same edit moved `quality_block.rs`'s ac527 census of `check` steps with an `expr` from `checks=93` to `checks=92`. On today's catalog the figures read 65 loop-backs, 15 `continue`s and 12 `fail`s.
- A loop-back is a new frame, so re-entry after a crash or a gate replays the same loops over the served commits, and
  no frame's commit is overwritten.
- The escalate re-entry, which D21-48 X8 made a licence for forward refs, has a frame to bind them in.

**Negative**

- A walk re-entered after a crash re-executes its `check`s and idempotent tool steps (ADR-0174 decision 7's re-entry
  rule), so it reproduces its earlier loops only if those tools answer as they did.
- The re-entry count is per walk: a loop-back and an escalate re-entry share one counter, so a frame's number says in
  what order re-entries happened, not which construct took it.
- The pass charges are reported and folded, not yet gated: no runtime budget gate reads them on the run path at this
  record's writing, and inner harness attempts are charged only where a caller hands `charge_observed_inner_attempts`
  a recorded count.

**What this forbids**

- Forbids reading a false `check` as a pass, a skip or `null`, and forbids ending a walk at a false `check` without
  consulting its `on_false`.
- Forbids a loop-back that re-uses an earlier frame's commit key, and forbids discarding or overwriting an earlier
  frame's committed step activation.
- Forbids a loop-back past `max`, and forbids an exhausted loop, a `"fail"` or an absent `on_false` reporting success.
- Forbids reading `{ to = "deviate" }` as a loop-back to a step named `deviate`, or executing it by any route but
  ADR-0046's.
- Forbids a fourth frame form, and forbids a `for_each` element being charged as a pass of its own.
- Forbids a new exit row for any refusal this record makes.

## Evidence and confidence

- [documented] D21-42 3b and 4d, D21-47 m13 and m14, D21-48 X7 and X8, each quoted in `source` or above; ADR-0031,
  0044, 0046, 0103 clause 6, 0122 decision 1 and 0144 decisions 3 and 6, quoted in Context and Decision.
- [measured 2026-09-26 at `s8d`'s build, through the no-dispatch helper] `on_false_exec::` (5 tests) —
  `a_loop_back_re_enters_its_target_under_a_new_iteration_path` (two frames, `root` and `loop_back:1`, each row in the
  journal; one charged pass per frame, folding to 2 attempts; a second walk over the answered gate serves both
  frames, re-dispatches nothing and charges nothing), `loop_back_exhaustion_refuses_as_step_failed_after_commit_and_
  committed_steps_stand` (three frames' rows stand; `step_error` carries `Unsupported`; the activation stays
  incomplete), `continue_proceeds_and_fail_refuses`, `a_false_check_with_no_on_false_refuses_as_step_failed_after_commit`
  and `a_deviate_on_false_refuses_by_name` — each first shown RED on the pre-R3 walk (the by-name stub), then GREEN.
  The resolver's iteration tests (`reference_resolver::` ×6, ADR-0173 decisions 4 and 5) run over the same executor.
- [measured, same session] Mutants M7 (a loop-back ignores `max`: the harness script's bound fired, *"LOOP-HARNESS
  BOUND: dispatch 4 is past the script's end"*) and M8 (exhaustion returns `Continue`: the walk completed with no
  `step_error`) are KILLED by `loop_back_exhaustion_refuses_as_step_failed_after_commit_and_committed_steps_stand`, and
  M6, M24, M26 and M35 by the resolver's iteration tests (`s8d` §8d.10), each run from a Python driver with every
  mutated file restored and its sha256 verified. `rk-cli`'s `exit::adr0144_post_commit_step_failure_exits_the_class_
  row_not_three` stays green.
- **What this evidence does not license.** The walks are over fixture step lists and fake harnesses; no shipped
  activity's loop is run against a real harness here, and the lane walk table is `s8d` §8d.9's. ADR-0046's route is
  owed and unbuilt. This record flips at `s8d`'s close under D21-64 ruling 10, on its own dated ledger line, only if
  the formal Checkpoint 8d passes whole and a refuting round finds no blocker or major.
