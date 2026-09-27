---
id: "0143"
title: "A **second runnable `agent` step** in one activation **refuses by name** rather than silently re-entering, and the committed report is **never overwritten**. `ActivitySpec` is ONE SPEC PER NODE (ADR-0099:61), so a second agent step has no spec, no card and no schema of its own; AC-013's per-activation grain then returns an EMPTY re-entry report which the walk was storing over the committed one, producing `NotCommitted { class: None }` — the exact hole Mechanism B exists to close. Measured: **63 of 102** activities declare more than one `agent` step and **43 of those 63 use DIFFERENT roles**, so one dispatch cannot honestly be said to cover them"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0099 decision 61's spec shape (`ActivitySpec` carries `StepSpec`'s five fields); references/adr/0138 (AC-013's grain stays per ACTIVATION; the step list is transcribed); references/adr/0142 (the guard context, and the same finding one level over); crates/rk-cli/src/verbs/run.rs (`bind_nodes` — *"🛑 ONE SPEC PER NODE, NOT PER STEP — THE RUNTIME DISPATCHES WHOLE ACTIVITIES"*, and the role taken from whichever step nominates one); crates/rk-kernel/src/runtime/activity.rs (AC-013's re-entry path returning an opened report); crates/rk-kernel/src/replay.rs (`dispatch_through`'s `NotCommitted`, and Mechanism B's `class`)
supersedes: []
superseded_by: ["0174"]
related: ["0097", "0099", "0138", "0141", "0142"]
evidence: []
consumers: []
---

## Context and problem

The walk's `agent` arm calls `Runtime::run_activity`, which is correct for the **first** agent step and
wrong for every one after it.

`run_activity` opens with AC-013's re-entry guard: a committed activation is not re-executed, and it
returns an **opened report** — `dispatches: 0`, `commit: None`, `outcome: None`. That is right, and it is
what `kill_and_resume_n_step` asserts. But the walk then did this:

```rust
let ended = !report.committed()
    && (report.outcome().is_some_and(Result::is_err) || report.mark().is_some());
dispatched = Some(report);   // <- UNCONDITIONAL
```

For the second agent step every disjunct is false, so `ended` is `false`, the walk continues — and
`dispatched` now holds the **empty** report instead of the committed one. `run_activity_steps` returns it,
`report.committed()` is `false`, and `dispatch_through` answers
**`ReplayError::NotCommitted { class: None }`**. The activation *did* commit. The run fails anyway, and it
fails carrying **no error class** — which is precisely the *"the `StepError` class never reaches the CLI"*
hole Mechanism B was added to close. `ActivityReport::skipped()` is also `true` for that report, so a
failed walk is indistinguishable from AC-013's legitimate re-entry.

**Measured: 63 of 102 shipped activities declare more than one `agent` step** — 19 declare one, 20 declare
none, and the rest run 2 (30), 3 (27), 4 (5) and 5 (1).

**And "one dispatch covers them all" is not available as an answer.** `ActivitySpec` is **one spec per
node**: `bind_nodes`' own header says so in terms — *"🛑 ONE SPEC PER NODE, NOT PER STEP — THE RUNTIME
DISPATCHES WHOLE ACTIVITIES … a DAG node gets exactly one binding whatever its steps' kinds are"* — and
that binding takes its `card_digest` from **whichever step first nominates a role** and its `schema` from
`provides[0]`. So a second agent step has no card, no schema and no inputs of its own.

Whether that matters is measurable, and it does: **43 of the 63 use DIFFERENT roles.**
`api-contract-spec` is `['spec-author', 'reviewer']`; `architecture-tradeoff-analysis` is
`['architect', 'reviewer', 'architect']`; `bdd-scenarios` is
`['spec-author', 'spec-author', 'reviewer', 'spec-author']`. **A dispatch bound to the spec-author's card
cannot be said to have performed the reviewer's step**, and treating it as already-served would silently
drop a review — the defect class this stage exists to end, reappearing one level up.

## Decision drivers

- A run that performs one of an activity's three agent steps and reports success is the vacuity `s7g`
  exists to remove. Silence is the failure mode, not the error.
- AC-013's grain is **per activation** and ADR-0138 pins it. Widening it to per-step would break
  `kill_and_resume_n_step` and criterion (3), and is not this record's to take.
- The pre-`s7g` runtime made exactly this approximation — one dispatch stood for the whole activity — and
  made it **invisibly**. `s7g`'s purpose is to stop approximating silently, not to approximate more
  carefully.
- `NotCommitted { class: None }` is worse than a refusal: the operator is told the activation did not
  commit, which is false, and is told nothing about why.

## Considered options

1. **Refuse the second runnable agent step, by name; never overwrite the report (taken).**
2. **Keep the first report and treat later agent steps as already served.** Rejected on the measurement:
   right for at most the 20 same-role activities, wrong for the 43 with distinct roles, and silent in both
   cases. It would convert a loud failure into a quiet half-execution.
3. **Widen AC-013's idempotence to per step.** Rejected: ADR-0138 pins the grain, `RunState::is_committed`
   has no step component, and `kill_and_resume_n_step` asserts the current one. It is also not sufficient
   on its own — without per-step specs there is still nothing to dispatch.
4. **Build per-step `ActivitySpec`s now.** Rejected as scope, and it is the real answer: it needs a spec
   per agent step with its own card and schema, which contradicts `bind_nodes`' stated contract and
   ADR-0099:61's five fields. **Recorded as OWED.**
5. **Refuse the whole activation up front when it declares >1 agent step.** Rejected as too coarse: a
   second agent step whose `when` guard does not hold never runs, and under ADR-0142 that guard can now
   actually be evaluated. The refusal belongs at the moment a second dispatch is really needed.

## Decision

1. **The committed report is never overwritten.** The walk keeps the report of the dispatch that
   committed, whatever later steps do. This half is unconditional and is the bug fix.

2. **A second *runnable* agent step refuses**, with `StepError::Unsupported` naming the activation, the
   step id, and the reason: `ActivitySpec` is one spec per node, so this step has no card or schema of its
   own and AC-013's per-activation grain makes a second dispatch impossible.

3. **"Runnable" means its guard held.** The check is dynamic, not a pre-walk count of `agent` steps: a
   second agent step whose `when` is false is `Skipped` and refuses nothing. ADR-0142 made that guard
   evaluable, so this is a real distinction rather than a theoretical one.

4. **AC-013's grain is untouched.** `RunState::is_committed(item, activity_id_at_version)` keeps no step
   component, `run_activity`'s re-entry path is unchanged, and `kill_and_resume_n_step` asserts the same
   property it always did. This record changes what the **walk** does with that report, not what the
   report means.

5. **Per-step specs are OWED.** A spec per agent step — with its own card digest, schema and inputs — is
   what would let an activity's declared agent steps each execute. It contradicts `bind_nodes`' current
   *one spec per node* contract and ADR-0099:61's field set, so it needs its own record and is not taken
   here.

**What this record does NOT decide, and does not make true.** It does not make the 63 multi-agent
activities executable — it makes their failure **honest**, replacing `NotCommitted { class: None }` with a
named refusal. It does not touch the `check` arm (ADR-0142 clause 4), the swallowed post-commit failures,
or the no-op `gate` arm. **Step 7g remains BLOCKED.**

## Consequences

**What this forbids**

- Forbids storing AC-013's empty re-entry report over a committed one. The committed report is the
  activation's result and a later step may not erase it.
- Forbids treating a second agent step as already served by the first dispatch. Measured, that is false
  for 43 of 63 activities, and silent for all 63.
- Forbids a pre-walk refusal keyed on the *declared* count of agent steps. A guarded second step that does
  not run is not a defect, and refusing it would make ADR-0142's newly-evaluable guards pointless.
- Forbids widening AC-013's idempotence grain without superseding ADR-0138.
- Forbids synthesising a card digest, schema or inputs for a second agent step. That is the per-step spec
  question, and inventing one would be deciding it by accident.

**What follows**

- 63 of 102 activities refuse where they previously failed — the same outcome, now with a cause. The
  journal names the activation and the step instead of reporting a commit that did not happen.
- `ReplayError::NotCommitted`'s `class` is populated on this path, because the refusal is a `StepError`
  the report carries. Mechanism B's channel is used rather than bypassed.
- The per-step-spec question acquires a measurement: 63 activities, 43 of them with distinct roles, which
  is the size of what a per-node spec cannot express.

## Evidence and confidence

- [measured 2026-09-22] Agent steps per activity over `.redkiln/catalog/`'s 102: **0 → 20, 1 → 19,
  2 → 30, 3 → 27, 4 → 5, 5 → 1**. **63** declare more than one.
- [measured 2026-09-22] Of those 63, **20** use a single role across their agent steps and **43** use two
  or more. Examples: `api-contract-spec` `[spec-author, reviewer]`;
  `architecture-tradeoff-analysis` `[architect, reviewer, architect]`; `bdd-scenarios`
  `[spec-author, spec-author, reviewer, spec-author]`.
- [documented] `bind_nodes`' header: *"🛑 ONE SPEC PER NODE, NOT PER STEP — THE RUNTIME DISPATCHES WHOLE
  ACTIVITIES. `Runtime::run_activity` takes one `ActivitySpec` for an `(item, activity_id@version)`
  activation, so a DAG node gets exactly one binding whatever its steps' kinds are."*
- [measured 2026-09-22] `run_activity`'s re-entry path returns a report with `dispatches: 0`,
  `commit: None`, `outcome: None`; `ActivityReport::skipped()` is `dispatches == 0 && commit.is_none()`,
  so that report is indistinguishable from a legitimate AC-013 skip.
- **Confidence: high on the counts and on the spec shape, both of which are enumerations rather than
  searches.** What is **not** established here is that per-step specs are the right answer rather than,
  say, splitting these activities in the catalog — 43 activities interleaving `spec-author` and `reviewer`
  may be describing two activities and a review gate. This record deliberately does not choose, and clause
  5 names the question rather than pre-empting it.
