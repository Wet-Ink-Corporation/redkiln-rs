---
id: "0174"
title: "**Step-grain commits** (`s8d`'s record R1): each agent step dispatches under its OWN per-step spec, carried on `RecordedSpec` beside the activation's, and commits under a SEPARATE commit key `(item, activity@version, step_id, iteration path)` as one `activity_committed` row carrying `step_id` and `iteration_path`; an activation completes on one more row, its `emit` step's; re-entry serves every committed step activation and resumes at the first uncommitted one; a `side_effecting` tool step commits and is served like an agent step. `IdempotencyKey` keeps three parts. Supersedes in part `0143` decision 2 (narrowed) and decision 4 (in full), `0150` decision 1 (in full), decision 2 and forbids bullets 1 and 4 (in part), `0138`'s grain sentence, `0140` decision 5 with its forbids bullet (for per-step specs) and, for agent steps, `0140` decision 1's withdrawal, decision 4 and its second forbids bullet; discharges `0143` clause 5 and `0150` clause 4 (widened 2026-09-27, `s8d` build, package R2FIX-8d: and, for per-step specs, `0171`'s sixth forbids bullet at its `ActivitySpec.timeout_secs` half, the rest of `0140` decision 5 and its third forbids bullet, and `0099` decisions 1 and 2 as to *per-activity*)"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R1's row), §8d.2, §8d.8 defects 2 and 4, §8d.10 and §8d.12; spikes/DECISIONS-2026-09-21.md D21-42 3a (*"Step grain in the runtime. Per-step specs are carried on `RecordedSpec` (ADR-0138's warrant), each agent step commits under its own key, and re-entry resumes at the first uncommitted step. Supersedes in part ADR-0143 d2/d4, ADR-0150 d1 and AC-013's per-activation grain; discharges ADR-0143 c5 and ADR-0150 c4."*) and 3b (*"`(item, activity@version, step_id, iteration path)`. The iteration path is the stack of for_each index, repeat count and loop-back count."*), D21-43 N15 (*"A separate commit key. ADR-0097's memo key is untouched."*) and N16 (*"Carried by each per-step spec (3a), resolved through 4a's table and bounded by the activity budget. Owner: s8d."*), D21-47 (the review's M5, the journal row, and M8, side-effecting tool steps), D21-64 ruling 10 (this record's flip condition); references/adr/0138, 0140, 0143, 0150 (the sentences superseded in part, quoted in Decision clause 10); references/adr/0171 clause 7 (*"each for `tool` steps only"*; *"The withdrawal stands for `agent` steps until `s8d`'s per-step specs carry their bound"*), the tool-step half of the ADR-0140 sites clause 10 supersedes for agent steps (dated 2026-09-27, `s8d` build, package R1FIX-8d); references/adr/0171's sixth forbids bullet (*"Enforcing an agent step's `timeout_secs` through `ActivitySpec.timeout_secs`, or through this record's `StepBound`."*), references/adr/0140 decision 5 and its third forbids bullet, and references/adr/0099 decisions 1 and 2, the sites clause 10 supersedes in part for per-step specs (dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting round 2's records blocker and the audit it asked for); crates/rk-kernel/src/workflow/idem.rs (`CommitKey`, `IterationPath`), crates/rk-kernel/src/workflow/fsm.rs (`RunState::is_committed`, `is_step_committed`), crates/rk-kernel/src/runtime/activity.rs (`StepAt`, `commit_walk_row`, `completion_event`), crates/rk-kernel/src/runtime/step.rs (the walk), crates/rk-kernel/src/replay.rs (`RecordedSpec`), crates/rk-kernel/src/workflow/memo.rs (`VerbEntry::side_effecting`, `StepContext::for_agent_step`), crates/rk-cli/src/verbs/run.rs (`bind_nodes`, `agent_step_timeout_secs`) and crates/rk-cli/src/verbs/explain.rs at `s8d`'s build (package P3)
supersedes: ["0099", "0138", "0140", "0143", "0150", "0171"]
superseded_by: []
related: ["0022", "0087", "0097", "0099", "0128", "0141", "0144", "0145", "0170", "0171", "0173"]
evidence: []
consumers: []
---

## Context and problem

The step executor `s7g` built walks an activation's steps, and an activation commits **once**, when its first agent
step dispatches. Four accepted records state that grain and name what it costs.

- **ADR-0143** (a second agent step refuses). Decision 2: *"A second *runnable* agent step refuses, with
  `StepError::Unsupported` naming the activation, the step id, and the reason: `ActivitySpec` is one spec per node, so
  this step has no card or schema of its own and AC-013's per-activation grain makes a second dispatch impossible."*
  Decision 4: *"AC-013's grain is untouched. `RunState::is_committed(item, activity_id_at_version)` keeps no step
  component, `run_activity`'s re-entry path is unchanged, and `kill_and_resume_n_step` asserts the same property it
  always did."* Clause 5: *"Per-step specs are OWED."* Measured there: **63 of 102** activities declare more than one
  agent step and **43** of those use different roles.
- **ADR-0150** (the commit grain stays per activation). Decision 1: *"The commit grain stays per activation.
  `activity_committed` is appended when the `agent` step dispatches, and the steps after it run inside a committed
  activation. This is not changed here."* Decision 2 names the residual: *"a process death between the commit and the
  end of the walk loses the remaining steps, because `is_committed` has no step component. **39** steps across **23**
  activities are in that position"*. Clause 4: *"The repair is an idempotency key with a STEP component, and no record
  currently routes it."* Its forbids bullets 1 and 4: *"Moving `activity_committed` to the end of the walk."* and
  *"Restating "the walk happens inside an uncommitted activation" anywhere in the tree."*
- **ADR-0138** (the step list reaches the runtime beside the spec). Its *"What this record does NOT decide"* paragraph
  holds the grain sentence: *"It does not change the idempotency grain: AC-013 stays per **activation**,
  `RunState::is_committed(item, activity_id_at_version)` is unchanged, and the walk happens inside an activation that
  is not yet committed."* ADR-0143's fourth forbids bullet reads *"Forbids widening AC-013's idempotence grain without
  superseding ADR-0138."*
- **ADR-0140** decision 5: *"`ActivitySpec.timeout_secs` is untouched. It is the Harness's, per activity, and this
  record neither widens nor narrows it. A future per-step bound may not be built by overloading it."* Its third
  forbids bullet: *"Forbids conflating `ActivitySpec.timeout_secs` with a step timeout, and forbids repurposing it as
  one."*

`s8d` §8d.8 measured two defects on the same seam. **Defect 2:** ADR-0143's refusal returned
`ActivityReport::refused(now, cause)` even when the first dispatch had committed, so `committed()` was `false` and
`skipped()` `true` for an activation whose row was in the journal — ADR-0144's named lie. **Defect 4:** `bind_nodes`
took the role from whichever step first nominated one and `continue`d past a node with none, so a node with **no**
agent step was never bound (six of the fix lane's fifteen selected activities), and it took every node's schema from
`activity.provides.first()`, which is wrong for every agent step whose output is not `provides[0]`.

The ledger took the grain. D21-42 3a and 3b are quoted in `source`. D21-43 N15 settles which key: *"A separate commit
key. ADR-0097's memo key is untouched."* D21-43 N16 settles the per-step bound. D21-47's review added two findings:
the journal row must be named (M5), and a tool step whose re-execution repeats a write cannot close ADR-0150's
residual by re-execution (M8).

## Decision drivers

- **AC-013 at more than one step.** *"no committed step re-executes"* must hold for the steps of one activation, not
  only for whole activations; D21-42 3a measured the per-activation branch dead (*"The mandatory implement stage's
  only Changeset producer (`tdd-red-green-refactor`) and (f)'s `user-story-ac` both need a second runnable agent
  step."*).
- **Nothing committed moves or is lost.** ADR-0150's decision drivers stand as reasons: moving the commit to the end
  of the walk would re-dispatch a committed agent step after a crash, which is the criterion-(3) regression ADR-0143
  and ADR-0144 were written to prevent.
- **`IdempotencyKey` keeps three parts and never a fourth** (ADR-0097's forbids; `rk-store`'s own table), so the step
  component lives on a separate key (N15).
- **A missing value is refused, never defaulted**: a per-step spec that is absent refuses by name, an unresolved
  `${toolchain.*}` bound refuses by name (D21-42 4a), and a verb row without `side_effecting` refuses the registry.

## Considered options

### Option — Per-step specs, a separate commit key, a completion row (taken)

D21-42 3a and 3b, D21-43 N15 and N16 and D21-47's M5 and M8, as taken. Each agent step dispatches under its own spec
and commits under its own key; the activation completes on its `emit` row; re-entry serves what is committed.

### Option — A fourth part on `IdempotencyKey`

Put `step_id` and the iteration path into the memo key's idempotency triple. Rejected by D21-43 N15 and by ADR-0097's
forbids (*"a fourth part on the idempotency key"*); it would also move every recorded journal's key and re-open
Checkpoint 6.

### Option — Move `activity_committed` to the end of the walk

ADR-0150 option (a). Rejected for ADR-0150's reason, which stands: a crash between a successful dispatch and the walk's
end would re-dispatch the agent step.

### Option — Keep the per-activation grain and split the 63 multi-agent activities in the catalog

ADR-0143's closing question. Rejected by D21-42 3a, which took step grain in the runtime; splitting 63 activities is
a catalog redesign, not a coherence edit under ADR-0072.

## Decision

**1. Per-step specs, on `RecordedSpec`.** `RecordedSpec` carries, beside the activation's spec, a map from `[[step]]`
id to one `ActivitySpec` per **agent** step (`RecordedSpec::step_specs`), serialised into `inputs/specs.json` as a
`step_specs` member that is ABSENT when the binding carries none (so a pre-R1 binding serialises byte-identically).
`ActivitySpec` is not edited (ADR-0099's five fields; ADR-0138's forbids). `rk-cli`'s `bind_nodes` binds each agent
step with **that step's** role → its card and **that step's** `outputs = "value:<Kind>"` → its schema, never
`activity.provides.first()`. The activation's own spec is the first agent step's card and schema at the pack's
per-activity `timeout_secs` (ADR-0140 decision 5's value, which it keeps).

**2. Every node is bound.** A node with **no** agent step is bound as an explicit absence of agent specs — an empty
`step_specs` map and **no activation spec** — and never with a synthesised card (ADR-0143's fifth forbids bullet,
standing). A runnable agent step with no per-step spec refuses by name (decision 3).

**3. ADR-0143 decision 2, narrowed.** A runnable agent step refuses by name only when it has **no per-step spec**: in
a binding that carries per-step specs, one that carries none for it; in a binding that carries none (a pre-R1
transcription, and the kernel's machinery suites), every runnable agent step after the first, which dispatches under
the activation's own spec. The refusal names the activation, the step and *"no per-step spec"*. **When an earlier
step of the activation committed — in this walk or an earlier run — the report stays committed:** the refusal travels
on `step_error` (`done.with_step_failure(cause)`, ADR-0144's shape; §8d.8 defect 2), never through
`ActivityReport::refused`.

**4. The commit key is separate: `(item, activity@version, step_id, iteration path)`** (`CommitKey`, D21-42 3b). The
iteration path is a stack of frames, outermost first — a `for_each` index, a `repeat` count, a loop-back count — and is
empty outside any loop; two iterations of one step never share a key. `IdempotencyKey` stays `(item,
activity_id@version, attempt)`, and `attempt` stays strictly increasing across the whole activation (AC-011): every row
the walk appends takes the next attempt of the activation, so the triple stays unique across the log. ADR-0097's memo
key is untouched; its component 7 is fed from the innermost frame of the iteration path, `0` outside a loop.
**Residual, named (`s8d` §8d.1 S5):** two nested paths that share an innermost counter share component 7.

**5. The journal row (D21-47 M5).** Each agent step's commit is one `activity_committed` row, appended when **that
step** dispatches, carrying two payload members beside the existing ones: `step_id` and `iteration_path` (an array of
frame tokens, `[]` outside a loop). No `EventKind` is minted. The activation's **completion** is one more
`activity_committed` row, appended when the walk reaches the activation's `emit` step, whose `step_id` is the `emit`
step's and which carries a third member, `completion: true`. *(The stage file names two members; the third exists
because the fold holds no catalog and must tell the completion row from an agent step's.)* The completion row
carries no `result` and no `schema`: binding the `emit` step's outputs is `0173`'s (decision 7), and the row says only
that the walk reached it. A zero-agent activation writes only that row. A transcribed step list that has no `emit`
step (a fixture catalog; every shipped activity ends in one) completes where its walk ends, on its last step's id, so
the plan cannot re-walk a node with nothing left to do. *(dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting
round 2's records minor.)* Design §5.4's `activity_committed` row took a dated in-place amendment under this record,
in the way ADR-0176 (R4) amended the `step_failed` row. The row now names `step_id` and `iteration_path` as the commit
key's members, `completion: true` on the completion row, and "at most once per commit key". The idempotency triple it
already named is unchanged.

**6. The fold and the plan walk.** `RunState::is_step_committed` folds the commit key off every row carrying
`step_id`. `RunState::is_committed(item, activity_id@version)` answers **completion**: a pre-R1 row (no `step_id`,
the activation grain's one commit, which completed its activation) or a completion row. `fsm::next_action` moves past a
node only on that. A row carrying `step_id` without a readable `iteration_path` is refused by the fold as malformed,
never folded under a guessed path.

**7. Re-entry resumes at the first uncommitted step.** The walk runs in declaration order. A complete activation is
not walked. A step activation whose commit is in the journal is **served** from it and never re-dispatched (AC-013 at
step grain); the first agent step with no commit executes, under its own spec, at its own commit key; the dispatch
pre-flight and AC-012's late-duplicate discard both ask the commit key. A `check` step and an **idempotent** `tool`
step do not commit, so on re-entry they re-execute: ADR-0150 decision 2's residual closes for idempotent verbs by
re-execution.

**8. Side-effecting tool steps (D21-47 M8).** Every row of `.redkiln/registry.toml`'s `[verb]` table declares a sixth
column, `side_effecting`, and a row without a boolean value refuses the registry at load. A `tool` step whose verb is
`side_effecting = true` commits its typed output under the commit key like an agent step, and on re-entry it is
**served, never re-executed**. Marked `true`: `package`, `item_create` and `release_tag` here, and `shallow_clone` and
`verdict_record` with the rows `s8d`'s R5 adds.

**9. Per-step specs carry their bound (D21-43 N16).** Each per-step spec's `timeout_secs` is the agent step's own
declaration — a literal, or a `${toolchain.<key>}` resolved through `s8c`'s per-toolchain table, refused by name when
unresolved — bounded by the activity budget: `min(step, budget.wall_clock_secs)`. A step that declares none takes the
activation's per-activity value, bounded the same way; an activity with no `budget.wall_clock_secs` refuses the
binding by name. An activation's own spec keeps the per-activity value decision 5 of `0140` describes.

**10. What is superseded, each in part and quoted.**
- **ADR-0143 decision 2**, narrowed by decision 3 above: the refusal stands for a runnable agent step with no per-step
  spec, and no longer for every second runnable agent step. **Decision 4** in full: *"AC-013's grain is untouched.
  `RunState::is_committed(item, activity_id_at_version)` keeps no step component, `run_activity`'s re-entry path is
  unchanged"* — the grain moves to the step activation (decisions 4–7). ADR-0143's forbids stand; its fourth bullet
  (*"Forbids widening AC-013's idempotence grain without superseding ADR-0138"*) is satisfied by the next bullet.
- **ADR-0138**, at the grain sentence in its *"What this record does NOT decide"* paragraph, which is not in its
  `## Decision`: *"It does not change the idempotency grain: AC-013 stays per **activation**,
  `RunState::is_committed(item, activity_id_at_version)` is unchanged, and the walk happens inside an activation that
  is not yet committed."* This record changes that grain; ADR-0138's decisions 1–5 and its forbids stand, and per-step
  specs sit on `RecordedSpec` on decision 2's warrant.
- **ADR-0150 decision 1** in full (*"The commit grain stays per activation."*). **Decision 2** in part: its residual
  closes by re-execution for an idempotent tool step and by commit-and-serve for a side-effecting one (decisions 7 and
  8). **Forbids bullet 1** in part (*"Moving `activity_committed` to the end of the walk."*): no agent-step row moves;
  the completion row is ADDED at the walk's end. **Forbids bullet 4** in part (*"Restating "the walk happens inside an
  uncommitted activation" anywhere in the tree."*): an activation with no completion row is *incomplete*, and its
  agent-step rows stand committed. ADR-0150's other forbids stand.
- **ADR-0140 decision 5**, at *"A future per-step bound may not be built by overloading it"*, and its third forbids
  bullet at *"and forbids repurposing it as one"*, for **per-step** specs only (decision 9). An activation's own spec
  keeps the per-activity value; `ActivitySpec.timeout_secs` is still not a step timeout for the activation.
- *(dated 2026-09-27, `s8d` build, package R1FIX-8d; the refuting round's records finding.)* **ADR-0140, for agent steps, at the three sites ADR-0171
  superseded for `tool` steps only** (ADR-0171 clause 7: *"each for `tool` steps only"*, and *"The withdrawal stands for
  `agent` steps until `s8d`'s per-step specs carry their bound"*). Decision 9 makes each agent step's per-step spec carry
  that step's declared `timeout_secs` to the Harness, so each of these is false for an agent step, and each is superseded
  in part:
  - **Decision 1's withdrawal** of ADR-0137 clause 5's *"while a step's own `timeout_secs` remains the kernel's"*, which
    decision 1 says *"is **withdrawn as false**"*. For an agent step whose per-step spec carries its bound, the clause is
    true again under decision 9. The rest of decision 1, the withdrawal of *"The backstop may not be smaller than the
    largest step it could serve, or a declared step timeout would be silently capped"* as reasoning, stands, as it stands
    under ADR-0171.
  - **Decision 4**, *"The catalog's `timeout_secs` declarations are recorded as UNREAD, and that is a gap this record opens
    rather than closes."* `rk-cli`'s binder reads each agent step's declaration (`agent_step_timeout_secs`), so decision
    4's OWED item is **discharged for agent steps**, as ADR-0171 discharged it for `tool` steps.
  - **The second forbids bullet**, *"Forbids any prose, docstring or record asserting that a step's declared
    `timeout_secs` is applied, enforced or honoured by the kernel or by a toolchain adapter. Nothing reads it."* For an
    agent step, the per-step spec carries the declared bound, bounded by the activity budget, to the Harness that
    enforces it.
  ADR-0140's first forbids bullet (the *backstop* bullet) is about the toolchain adapters' construction-time `timeout`,
  which no agent step reaches, and it is not moved here. ADR-0140 decisions 2 and 3 and its last two forbids bullets
  stand. Both sides already carried the link (`supersedes: ["0140", …]` here, `superseded_by: ["0171", "0174"]` there), so
  no link is added.
- *(dated 2026-09-27, `s8d` build, package R2FIX-8d; refuting round 2's records blocker.)* **ADR-0171 (accepted), at its
  sixth forbids bullet, for per-step specs only:** *"Enforcing an agent step's `timeout_secs` through
  `ActivitySpec.timeout_secs`, or through this record's `StepBound`."* Decision 9 does exactly the first half. Each
  per-step spec is an `rk_types::activity::ActivitySpec` (`rk-cli`'s `bind_nodes` fills `step_specs: BTreeMap<String,
  ActivitySpec>`), and its `timeout_secs` is `agent_step_timeout_secs(..)`, the agent step's declared bound, so the
  Harness enforces that bound through `ActivitySpec.timeout_secs`. The bullet is superseded in part, at *"through
  `ActivitySpec.timeout_secs`"*, for per-step specs only. Three things stand. An activation's own spec keeps the
  per-activity value (ADR-0140 decision 5, as the next bullet leaves it). The bullet's second half, *"or through this
  record's `StepBound`"*, stands: no agent step's bound reaches a `Toolchain` port call. ADR-0171 decision 5's
  *"`ActivitySpec.timeout_secs` is untouched"* is that record's statement of its own scope (*"The agent steps are not
  this record's"*), and it stays true of that record: the type is not edited (ADR-0138's forbids), and *"enforcing the
  185 agent-step declarations is `s8d`'s (N16)"* is what decision 9 does. ADR-0171's other decisions and forbids
  bullets stand. Links: `0171` joins this record's `supersedes`, and `0174` joins ADR-0171's `superseded_by`.
- *(dated 2026-09-27, `s8d` build, package R2FIX-8d; the audit of the accepted records this build touches, which the
  same finding asked for.)* **ADR-0140 decision 5 and its third forbids bullet, the rest of each, for per-step specs
  only.** The earlier bullet moved only *"A future per-step bound may not be built by overloading it"* and *"and
  forbids repurposing it as one"*. Two more sites in the same sentences say what decision 9 contradicts for a per-step
  spec. Decision 5's *"It is the Harness's, per activity"* does not hold, because a per-step spec's value is per step.
  The third bullet's *"Forbids conflating `ActivitySpec.timeout_secs` with a step timeout"* does not hold either,
  because a per-step spec's `timeout_secs` is its step's bound. Both are superseded in part, for per-step specs only.
  For an activation's own spec both stand, and so does *"`ActivitySpec.timeout_secs` is untouched"* as to the type.
  The link already exists.
- *(dated 2026-09-27, `s8d` build, package R2FIX-8d; the same audit.)* **ADR-0099 (accepted) decision 1's *"The
  adapter's declared per-activity timeout (AC-223) is the in-band cancel"* and decision 2's *"the per-activity
  timeout"*** (one of the token's two owners), in part, as to *per-activity* and for per-step specs only. Under
  decision 1 the Harness dispatches each agent step under its own spec. The in-band cancel for that dispatch is the
  spec's bound, `min(step, budget.wall_clock_secs)` (decision 9), which is a step's bound and not an activity's. The
  rest of both decisions stands: `dispatch` blocks, the kernel never calls cancel, the token has exactly two owners,
  and the Toolchain half is as ADR-0171 left it. ADR-0099's supersessions by `0115`, `0127`, `0165` and `0171` are
  unaffected. Links: `0099` joins this record's `supersedes`, and `0174` is appended to ADR-0099's `superseded_by`.
  **Residual, named and not claimed:** AC-223's lint (*"the declared timeout is at least the recorded slow-condition
  figure for that class"*) reads a pack-declared timeout per activity class. A per-step bound below that figure is
  not linted here, and no such lint exists in the tree (`s8d` §8d.1's R2FIX-8d note). **Owed, also named:**
  `crates/rk-types/src/activity.rs`'s field doc on `timeout_secs` still reads *"The adapter's declared per-activity
  timeout (AC-223)"*. It is left as written because an edit under `crates/rk-types` would pull Checkpoint 5 into
  Checkpoint 8d (13) (`s8d` §8d.11). Its dated correction is owed in the change that flips this record.

**11. What is discharged, not superseded.** ADR-0143 clause 5 (*"Per-step specs are OWED"*) by decisions 1–3, and
ADR-0150 clause 4 (*"The repair is an idempotency key with a STEP component, and no record currently routes it"*) by
decisions 4–8.

**12. The per-step memo facts.** Each dispatched agent step's `workflow::memo::StepContext` is assembled with **that
step's** card digest (component 5) and schema id (component 8), never the node's (`StepContext::for_agent_step`). The
memo is still consulted by nothing on the run path, and this record does not wire it. *(dated 2026-09-27, `s8d` build,
package R2FIX-8d; refuting round 2's memo minor.)* Component 6 is now filled as well. The digest of each of the step's
resolved `kb:` inputs comes from R2's `kb:` resolution, `{key, path, content, digest}`, through
`StepContext::with_kb_atoms`, which both call sites chain. A `checklist:` input and a literal are not atoms. Test:
`step_grain::an_agent_steps_kb_inputs_reach_its_memo_context_as_their_content_digests`, red before the change at *"left:
[]"*. Mutants MR2b–MR2e are killed (`s8d` §8d.10).

**13. `explain` prints one `output_digest` per committed step activation** (ADR-0128 decision 4's field), labelled
with the step and its iteration path; a pre-R1 record prints the one line it always did.

**14. AC-013's register row takes a dated in-place amendment, drafted here and landed with this record's flip** (D15-2's
form, not an erratum; `s8d` §8d.1 m17). The draft, verbatim:

> *(Amended in place 2026-09-26 at `s8d` under ADR-0174 (R1), landing with its flip; D15-2's form, not an erratum.)*
> **A committed step is a committed step activation.** *"no committed step re-executes"* reads at step-activation
> grain: a step activation is identified by R1's commit key `(item, activity_id@version, step_id, iteration path)`,
> and an activation whose walk committed some of its steps before the kill resumes at its first uncommitted step,
> serving every committed one from the journal with its digest unchanged. A record written before R1, whose
> `activity_committed` rows carry no `step_id`, has one step activation per activation, so the criterion reads as it
> did. *Verified by (amended):* `cross_process::kill_and_resume_n_step` over ONE activation with three agent steps —
> two commit, the process dies inside the third's dispatch, a different process re-enters, the two are served with
> byte-identical digests and the third dispatches once — and `cross_process::kill_and_resume_n_activation`, the
> activation-grain body it replaced, over three activations. The **PROVISIONAL** label and its P8 reason are
> unchanged.

## Consequences

**Positive**

- The 63 multi-agent activities can each dispatch every agent step under its own card and schema; the refusal is left
  for a step the binding really has no spec for.
- A process death between two steps of one activation loses nothing committed and re-executes nothing committed.
- A zero-agent activation binds and completes instead of being refused by the run's supplier.
- A tool step that writes is never written twice by a re-entry.

**Negative**

- An activation now writes one more `activity_committed` row, its completion, so a count of `activity_committed` rows
  is no longer a count of activations; a reader that wants activations counts completion rows and pre-R1 rows.
- A fixture step list with no `emit` completes where it ends, which is a reading this record states rather than a rule
  the schema gives.
- The memo key's component 7 cannot tell two nested paths with the same innermost counter apart (decision 4's
  residual).

**What this forbids**

- Forbids a fourth part on `IdempotencyKey`; the step component lives on the commit key.
- Forbids re-dispatching a step activation whose commit is in the journal, and forbids re-executing a
  `side_effecting` tool step that committed.
- Forbids `fsm::next_action` moving past a node on any row but its completion (or a pre-R1 row).
- Forbids a binder that skips a node with no agent step, takes an agent step's schema from `provides[0]`, or
  synthesises a card for an activation that has no agent step.
- Forbids a per-step bound above its activity's `budget.wall_clock_secs`, and forbids defaulting an unresolved
  `${toolchain.*}` bound.
- Forbids reporting `committed() == false` for an activation whose step row is in the journal: a refusal after a
  commit travels on `step_error`.
- Forbids a `[verb]` row without a boolean `side_effecting`, and forbids reading an absent one as either value.
- Forbids folding a step row whose iteration path does not read.

## Evidence and confidence

- [measured 2026-09-26 at `s8d`'s build (package P3), in the main checkout, through `runbook/tools/nodispatch.sh`]
  `rk-kernel`'s `step_grain::` (nine tests), `verb_registry::every_row_declares_side_effecting_and_the_writing_verbs_are_marked`,
  `cross_process::kill_and_resume_n_step` (rewritten at step grain) and `cross_process::kill_and_resume_n_activation`
  (the preserved body), and `second_agent::a_second_runnable_agent_step_refuses_and_the_commit_survives` (amended to
  assert `committed()` and `skipped()`) are green; `rk-cli`'s three `transcription::` tests over a real run of the
  binary and `explain::explain_prints_one_output_digest_per_committed_step_activation` are green. Each was first shown
  RED on the pre-R1 code.
- [measured, same session] Mutants M1, M2, M3, M4, M12, M13, M18, M20, M27, M28 and M30 of `s8d` §8d.10 are each
  KILLED by the tests the table names, run from a Python driver with every mutated file restored and its sha256
  verified.
- [measured, same session] The step-grain record written by `resume` replays with zero divergences and zero dispatches,
  and `explain` renders it byte-identically before and after the replay.
- **What this evidence does not license.** No shipped activity completes against a real harness here; cards, schemas
  and models are `s8h`'s. The memo is consulted by nothing. Iteration frames are pushed only by the loop executors
  `0173` and `s8d`'s R3 build, so the walk's own path is empty at this record's writing; the commit key's
  iteration-path component is exercised through `Runtime::run_agent_step`. *(dated 2026-09-26, `s8d` build, package P5: the loop
  executors are now built — `0173` decision 5 and R3 (`0175`, `proposed`) — so the walk itself pushes `for_each`,
  `repeat` and loop-back frames, and every step activation it re-runs in a new frame commits under that frame's
  path, as decision 4 says; `on_false_exec::` and `reference_resolver::`'s iteration tests read those rows off the
  journal.)* This record flips at `s8d`'s close under
  D21-64 ruling 10, on its own dated ledger line, only if the formal Checkpoint 8d passes whole and a refuting round
  finds no blocker or major; AC-013's in-place amendment (decision 14) lands in the same change as the flip.
