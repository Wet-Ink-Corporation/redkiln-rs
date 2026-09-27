---
id: "0175"
title: "**`on_false` as iteration** (`s8d`'s record R3): a `check` that does not hold is routed through its `on_false`, classified by the one classifier over ADR-0103 clause 6's closed set — a `{ to, max }` loop-back re-enters its earlier target under a NEW loop-back frame and the earlier frames' committed step activations stand; after `max` loop-backs, with `\"fail\"`, or with no `on_false`, the walk refuses by name, as `StepFailedAfterCommit` when a step committed (ADR-0144) and as a refusal otherwise, the exit code being the class's own; `\"continue\"` proceeds; and `{ to = \"deviate\" }` refuses by name, naming ADR-0046's route OWED (the narrowing of D21-42 4d that D21-48 X7 took). An escalate gate's re-entry is a loop-back frame too, and every loop-back and `repeat` frame is a pass charged on its own, a `for_each` once. Executes `0031`, `0044`, `0103` clause 6 and `0144`; supersedes nothing"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R3's row), §8d.3 (*The attempt charge per frame (m14)*, the escalate re-entry's binding), §8d.4 and §8d.10; spikes/DECISIONS-2026-09-21.md D21-42 4d (*"**`on_false` is honoured as iteration.** A `{to,max}` loop-back runs under a new iteration path, keyword forms follow the schema, and exhaustion or no `on_false` refuses as `StepFailedAfterCommit` (ADR-0144). Committed steps stand."*) and 3b (*"The iteration path is the stack of for_each index, repeat count and loop-back count."*), D21-47 (the review's m13 and m14), D21-48 X7 (*"**Refused by name, naming ADR-0046's route OWED.** R3 states the narrowing of D21-42 4d."*) and X8, D21-64 ruling 10 (this record's flip condition); references/adr/0031, 0044, 0046, 0103 (clause 6), 0122 (decisions 1, 3 and 4; 3 and 4 cited from 2026-09-27, `s8d` build, package R4FIX-8d), 0144; references/2026-09-07-activity-graph-schema.md §1 (Loop) and §3 (Escalation); crates/rk-kernel/src/runtime/step.rs (`run_activity_steps`' loop executor, `Runtime::on_false`, `after_step`, `gate_arm`, `fan_out`), crates/rk-kernel/src/runtime/reference.rs (`Scope::open_frame`), crates/rk-kernel/src/runtime/activity.rs (`ActivityReport::pass_charges`), crates/rk-kernel/src/catalog.rs (`Step::on_false`, `Step::on_exhausted`, `repeat_body_end`) and crates/rk-kernel/src/workflow/fsm.rs (`RunState::gates_of`) at `s8d`'s build (package P5)
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

*(dated 2026-09-27, `s8d` build, package D2176-8d; amended while `proposed`; D21-76, option (a).)* **On every re-entry
the walk REPLAYS the loop decisions it journaled, so an answered escalate gate resumes into its escalating step.**
Before this amendment the re-entry re-walked from the top and re-decided every `check`, `until` and guard over
re-executed tool answers. In `tdd-red-green-refactor` (both lanes) `green` changes the tree before `green-gate` opens,
so on the answer's re-walk `red-verify` re-ran over tests `green` had fixed, `red-check` (`report.failed > 0 and
report.passed == 0`) no longer held, and the walk looped back into `red` instead of resuming into `green` (refuting
round 7; D21-76's *Why*). D21-76 rules: *"The walk **journals its control decisions** — each `check` verdict and each
loop-back or escalation decision — under the commit key of the frame that took it, and a re-entered walk **replays**
them"*. ADR-0174 decision 7, as amended the same day, states the row and the replay. For this record's routes:
- A `check`'s `false` is journaled before its `on_false` is taken (decision 2's loop-back, decision 3's refusal or
  decision 4's `continue`). A replayed `false` re-takes the same route over the loop counts the replay itself rebuilds,
  so the same frames are re-entered and their commits are served.
- A `repeat`'s `until` is journaled after each pass, under the repeating step's commit key at that pass's path. A
  replayed `false` at `max` re-takes the step's `on_exhausted`, so an `escalate` re-raises the same question.
- A `gate` step's `when` is journaled when it is decided, and a replayed `true` reaches the gate. The escalate
  re-entry of this decision's first paragraph then reads the same answered reach, re-enters the escalating step under
  a new loop-back frame, and dispatches it once.
- Where the prior walk journaled no decision, the walk decides fresh and journals it. So a crash re-entry re-verifies
  what it had not decided, and re-takes no loop it had not taken.

**7. The attempt charge per frame (m14).** A pass is an iteration path with its `for_each` frames removed. Each
loop-back frame and each `repeat` frame is a pass of its own for AC-021's per-pass count; a `for_each` counts once,
whatever its element count (AC-022, ADR-0053). Every agent or tool step the walk EXECUTES in a pass charges that pass
through `budget::charge_for_each`, the one charge site AC-022 names; a step SERVED from its commit on re-entry charges
nothing. The walk reports the charges per pass (`ActivityReport::pass_charges`), and `budget::fold` accumulates them
across passes (ADR-0122 decision 3). ADR-0122 decision 1 stands for the inner attempts: *"An inner harness attempt
charges the activity's attempt budget, one charge per recorded `system/api_retry` event, in the pass that dispatched
it"*; its charge site is `budget::charge_observed_inner_attempts`. ADR-0037 is a load-time lint on the declared field
and does not move.

*(dated 2026-09-27, `s8d` build, package M14-8d; amended while `proposed`, m14 settled by accepted ADR-0122.)* ~~Every
agent or tool step the walk EXECUTES in a pass charges that pass through `budget::charge_for_each`~~ — the charge is
**per dispatch**, not per executed step: every dispatch an agent or tool step makes in a pass charges that pass, a
kernel re-dispatch of the same step inside `run_activity`'s retry loop included. A non-`for_each` step charges
`budget::charge_dispatched(step, n)`, `charge_for_each`'s kind rule scaled by the `n` dispatches the step made
(`ActivityReport::dispatches`; a `tool` step is one); a `for_each` step still charges once through `charge_for_each`,
whatever its element count (AC-022, ADR-0053); a SERVED step, having made no dispatch, charges nothing. The grounds are
accepted ADR-0122's. Decision 1: *"The runtime attempt counter for the activity is debited by the harness's own
attempts observed on the stream of a dispatch, as well as by the dispatch itself."* Decision 3: *"AC-021's quantity is
the declared budget field"* and *"AC-026's quantity is the runtime draw-down"*, so AC-021's per-pass step count does
not set the runtime charge. Decision 4: *"ADR-0037 was cited as the authority for a runtime rule it never states"*, so
ADR-0037's *"`budget.attempts_total ≥ number of agent/tool steps on one pass`"* does not either. The harness's own inner
attempts (decision 1's *"one charge per recorded `system/api_retry` event"*) remain at
`budget::charge_observed_inner_attempts`, which the run path does not call: ARMED, not discharged, as recorded since
`s7b`.

*(dated 2026-09-27, `s8d` build, package R4FIX-8d; amended while `proposed`; refuting round 4's spec-conformance
major.)* ~~(`ActivityReport::dispatches`; a `tool` step is one)~~: the text claimed the rule held for tool steps, and
the build did not meet it. A tool step whose port call erred charged nothing. A walk that halted before any commit
dropped every charge already noted. A `for_each` that halted part-way charged nothing. Now every dispatch counts,
answered or not. A `tool` step makes its one dispatch when its port, verb or route call is MADE, whether the call
answered or erred. A refusal before the call makes none and charges nothing: the registry lookup, the selection, an
unresolved input, a missing `range` or `sha`, an unbound or unbuilt route, or no stage `rigor`. A halt keeps every
charge the walk noted before it, whether it is a refusal, a gate or a step failure after a commit. A `for_each` that
halts charges its pass once for the copies it dispatched (AC-022).

## Consequences

**Positive**

- The 66 shipped loop-backs loop, the 15 `continue`s proceed and the 13 `fail`s refuse by name, each as its catalog
  declares; a refusal after a commit still reaches the operator with its class. *(dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting round 2's records minor.)* The three counts are the census at `ca8f387`. Re-measured by a `tomllib` census over `.redkiln/catalog/*.toml` (`scratchpad/s8d/r2fix-8d-onfalse.py`, 102 files at each pin): `ca8f387` **66 / 15 / 13 = 94**, `d46b2eb` (this stage's entry) **65 / 15 / 13 = 93**, and HEAD and the working tree **65 / 15 / 12 = 92** (loop-back / `continue` / `fail`). The drop from 13 `fail`s to 12 is D21-67 ruling 1, which deleted `migration-dry-run`'s policy check and its `fail`. The same edit moved `quality_block.rs`'s ac527 census of `check` steps with an `expr` from `checks=93` to `checks=92`. On today's catalog the figures read 65 loop-backs, 15 `continue`s and 12 `fail`s.
- A loop-back is a new frame, so re-entry after a crash or a gate replays the same loops over the served commits, and
  no frame's commit is overwritten. *(dated 2026-09-27, `s8d` build, package D2176-8d; D21-76; refuting round 7
  found the claim unmet: a re-walk past an answered gate re-decided the loops over re-executed tool answers.)* ~~replays
  the same loops over the served commits~~: re-entry replays the same loops because it replays their journaled
  decisions (decision 6's dated amendment; ADR-0174 decision 7), not because the re-executed tools answer as they did.
  An answered escalate gate resumes into its escalating step.
- The escalate re-entry, which D21-48 X8 made a licence for forward refs, has a frame to bind them in.

**Negative**

- A walk re-entered after a crash re-executes its `check`s and idempotent tool steps (ADR-0174 decision 7's re-entry
  rule), so it reproduces its earlier loops only if those tools answer as they did. *(dated 2026-09-27, `s8d` build,
  package D2176-8d; D21-76.)* ~~re-executes its `check`s and idempotent tool steps (…), so it reproduces its earlier
  loops only if those tools answer as they did~~. A walk re-entered after a crash **or after a gate is answered**
  re-executes its idempotent tool steps and replays every decision it journaled. It reproduces its earlier loops up to
  its last journaled decision whatever the tools now answer, and past that point it decides fresh. The cost: a later
  `${…}` binds the re-executed value while the route stays the journaled one. A step's input can therefore carry a
  tool answer that the replayed decision was not taken over. A `when` on a step that is not a `gate`, and a `for_each`
  source, are still re-evaluated (ADR-0174 decision 7's *Named, not closed*). Each fresh decision also appends one
  `activity_committed` row that is not a commit.
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
- [measured 2026-09-27 at `s8d`'s build, package M14-8d, through the no-dispatch helper] decision 7's per-dispatch
  amendment: `on_false_exec::a_kernel_retried_agent_step_charges_its_pass_once_per_dispatch` (an agent step failing
  `Transport` on its first dispatch and committing on the kernel's re-dispatch charges `root` 2) was RED on the
  flat-1 charge (*"left: [("root", Attempts(Attempts(1)))] right: [("root", Attempts(Attempts(2)))]"*), then
  GREEN; mutants MA (the loop charges `charge_dispatched(step, 1)`) and MB (the agent arm records 1 dispatch) are
  KILLED by it (`s8d` §8d.10's M14-8d note).
- [measured 2026-09-27 at `s8d`'s build, package R4FIX-8d, through the no-dispatch helper] decision 7 for tool steps
  and across a halt. Each test below was RED on the pre-fix code:
  - `on_false_exec::a_tool_dispatch_that_fails_at_the_port_charges_its_pass`: a `test` port reached once and erring
    charges `root` 2 with the agent step before it, where the pre-fix code read *"left: [("root", 1)]"*. A pre-port
    selection refusal charges 1.
  - `a_walk_that_halts_before_any_commit_keeps_the_charges_already_made`: a `lint` dispatch before a pre-port refusal,
    and before a gate, each keeps `root` 1, where the pre-fix code read *"left: []"*.
  - `a_fan_out_that_halts_part_way_charges_its_pass_for_the_copies_it_dispatched`: `root` 1, where the pre-fix code read
    *"left: []"*.
  - `every_tool_call_site_charges_its_pass_once_the_call_is_made`: one arm per call site, each with its pre-call
    control. Its red was measured with the pre-fix `step.rs` and `activity.rs` swapped in.

  Sixteen mutants were KILLED (`s8d` §8d.10's R4FIX-8d note). They cover the flag honoured only on success, every tool
  step charged, each fresh halt dropping charges, `fan_out`'s halting arm, the flag unset at each of the seven call
  sites, and the flag set before three pre-call refusals.
- [measured 2026-09-27 at `s8d`'s build, package D2176-8d, through the no-dispatch helper] decision 6's dated
  amendment and the corrected Positive and Negative bullets.
  `on_false_exec::a_tdd_shaped_escalate_re_entry_dispatches_green_not_red_after_the_answer` shows an answered escalate
  gate resuming into `green` under `loop_back:1/repeat:1`, where the pre-D21-76 walk looped back into `red`.
  `a_re_entry_past_an_answered_gate_replays_the_check_verdict_and_re_dispatches_nothing` re-enters past a gate with a
  changed `test` answer and re-dispatches nothing. `a_crash_re_entry_replays_only_the_decisions_it_journaled_and_re_
  decides_the_rest` re-decides only what the cut journal never recorded. Each was RED first. The mutants that ignore the
  `until` and gate `when` replays, or read an absent decision as `true`, are KILLED (`s8d` §8d.10's D2176-8d note).
- **What this evidence does not license.** The walks are over fixture step lists and fake harnesses; no shipped
  activity's loop is run against a real harness here, and the lane walk table is `s8d` §8d.9's. ADR-0046's route is
  owed and unbuilt. This record flips at `s8d`'s close under D21-64 ruling 10, on its own dated ledger line, only if
  the formal Checkpoint 8d passes whole and a refuting round finds no blocker or major.
