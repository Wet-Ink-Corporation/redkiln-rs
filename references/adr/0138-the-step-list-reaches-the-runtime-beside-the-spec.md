---
id: "0138"
title: "An activity's **step list reaches the runtime beside the `ActivitySpec`**, transcribed onto `RecordedSpec` by the composition root that already holds the `catalog::Activity` — not as a sixth `ActivitySpec` field (`rk-types` is Checkpoint 5's), and never by loading the catalog from inside `runtime/` (residual 7 forbids it); this is the same warrant `RecordedSpec::activity_id_at_version` already stands on"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/runtime.rs:22-27 (residual 7 — *"never call `Catalog::from_dir` from inside `workflow` or `select`"*); crates/rk-types/src/activity.rs:3-5 and :46-59 (ADR-0099:61 — `ActivitySpec` carries `StepSpec`'s five fields); crates/rk-kernel/src/replay.rs:640-664 (`RecordedSpec`, and its *"NOT a sixth spec field"* warrant); crates/rk-kernel/src/catalog.rs:351 (`pub steps: Vec<Step>`); crates/rk-cli/src/verbs/run.rs:1380-1383 and :1423 (the binder already reads `activity.steps`); references/adr/0099 decision 5 and decision 61's spec shape; runbook/phase-1/s7g-step-executor.md §7g.1
supersedes: []
superseded_by: ["0141"]
related: ["0083", "0097", "0099", "0105"]
evidence: []
consumers: []
---

## Context and problem

`s7g` §7g.1 makes the step the unit of execution. **The runtime cannot see a step.**

- `catalog::Activity` **does** carry its steps — `pub steps: Vec<Step>` at `catalog.rs:351`.
- What the runtime is handed is **not** a `catalog::Activity`. It is `rk_types::activity::ActivitySpec`, a
  five-field adapter payload — `card_digest`, `tools`, `schema`, `timeout_secs`, `inputs` — whose header
  says *"ADR-0099:61 — `ActivitySpec` carries `StepSpec`'s fields (card digest, tools, schema id, timeout,
  inputs)"*. There is no sixth field and no step list.
- `Runtime::run_activity(item, activity_id_at_version, step_id, spec, cap, now)` therefore has **no route to
  a `Step`**. `s7f`'s armed lint says so in its own failure text: *"`Runtime::run_activity` dispatches an
  ACTIVATION, not an activity's STEPS, and `ActivitySpec` carries no step list to route on."*
- `run.rs:1380-1383` states the current contract deliberately: *"🛑 **ONE SPEC PER NODE, NOT PER STEP — THE
  RUNTIME DISPATCHES WHOLE ACTIVITIES.**"*

**Two of the three conceivable routes are closed by existing records.** `rk-types` is Checkpoint 5's crate
and is not edited by the stages in this range, so `ActivitySpec` may not grow a sixth field. And loading the
catalog inside the runtime is forbidden outright: `runtime.rs:22-27` carries residual 7 — *"never call
`Catalog::from_dir` from inside `workflow` or `select`"* — and asserts that nothing reachable from `runtime`
reaches it.

**The value is already in hand at the one place that builds a spec.** `run.rs`'s `bind_nodes` holds the
`catalog::Activity` and already reads its steps, at `run.rs:1423`, to find a role:
`activity.steps.iter().find_map(|s| s.role.clone())`. The composition root has the step list and throws it
away.

## Decision drivers

- `00-preamble.md`:55's rule, which `resolver.rs` cites for the sibling question: the need arrives **as
  data**, and the kernel does not go and fetch what it was not given.
- ADR-0083's *"the composition root builds it; the kernel reads it and never makes one"* is already the
  shape `Resolved::config` uses; a step list is the same kind of value.
- Residual 7 exists because a catalog load inside the runtime would make the runtime's behaviour depend on
  a directory rather than on what it was handed — which would also make `Runtime` untestable without a
  pack on disk.
- Whatever carries the steps is **serialised into the run record** (`inputs/specs.json`), so the choice is
  also a choice about what the record shows. A step list beside the spec makes the run record say which
  steps the run believed the activation had.

## Considered options

1. **Transcribe the steps onto `RecordedSpec`, beside the spec (taken).**
2. **Add a sixth field to `ActivitySpec`.** Rejected: `rk-types` is Checkpoint 5's and is not edited in this
   range, and ADR-0099:61 fixes the spec as `StepSpec`'s five fields. It is also wrong on the merits — the
   spec is what the **Harness port** is handed, and the step list is not the harness's business.
3. **Load the catalog inside `runtime/`.** Rejected: residual 7 forbids it by name, and `runtime.rs:22-27`
   asserts the absence structurally.
4. **Pass `&[Step]` as a new parameter to `run_activity`.** Not rejected as wrong — it is the honest
   fallback and clause 4 below keeps it available — but it is the weaker default: it widens a signature
   that three call sites already thread, and it leaves the value out of the run record, where the
   `RecordedSpec` route puts it for free.
5. **Re-derive the steps in the kernel from the `ActivitySpec`.** Rejected as not possible: the spec carries
   a card digest, a schema id, tools, a timeout and inputs. None of them determines a step list.

## Decision

1. **The step list is carried on `RecordedSpec`, beside `activity_id_at_version` and `spec`.** It is
   transcribed by the caller, through the existing `RecordedSpec::supplied` constructor's model, and the
   runtime reads it.

2. **This is the warrant `RecordedSpec` already stands on, applied unchanged.** Its own docstring says
   `activity_id_at_version` is there because the join *"has three candidate homes and choosing one is a
   decision this stage may not take; the binding transcribes it instead"*, and that it is **"NOT a sixth
   spec field"**. A step list is the same kind of value from the same source: held by the composition root,
   not derivable by the kernel, and not the Harness port's business. `RecordedSpec` is the type for
   *"something this crate did not derive"*, and this is one.

3. **`ActivitySpec` is not edited, and the catalog is not loaded from `runtime/`.** Both prohibitions stand
   exactly as written; this record adds nothing to them and relies on both.

4. **If the transcription fans out more widely than the three sites that thread `RecordedSpec` today, the
   fallback is option 4 — `&[Step]` as a parameter to `run_activity` — and the switch is recorded in the
   commit that makes it, with the measured fan-out that caused it.** It is not made silently, and it does
   not reopen options 2, 3 or 5.

5. **The step list a run executed becomes part of the run record**, through `RecordedSpec`'s existing
   serialisation into `inputs/specs.json`. That is a gain, not an incidental: a run record that names the
   activation but not the steps cannot be read against a catalog that has since changed, and
   `activity_id@version` pins the activity while the transcribed list pins what was actually walked.

**What this record does NOT decide.** It does not decide the join between a DAG node and
`Activity::id_at_version` — that record is still OWED and `RecordedSpec`'s docstring names it. It does not
decide what any step arm **does** (ADR-0135, ADR-0136). It does not change the idempotency grain: AC-013
stays per **activation**, `RunState::is_committed(item, activity_id_at_version)` is unchanged, and the walk
happens inside an activation that is not yet committed.

## Consequences

**What this forbids**

- Forbids adding a step list, or any sixth field, to `rk_types::activity::ActivitySpec`. The spec is the
  Harness port's payload and is ADR-0099:61's five fields.
- Forbids `Catalog::from_dir`, or any other catalog load, from inside `crate::runtime`, `crate::workflow` or
  `crate::select`. Residual 7 is unchanged and its structural assertion stands.
- Forbids re-deriving, guessing or defaulting a step list inside the kernel when one was not supplied. A
  missing transcription is an error naming the activation, never an empty walk — an activation that walks
  zero steps and reports success is the vacuity this whole stage exists to end.
- Forbids reading the transcribed list as authority over what the catalog says. It records what the run was
  handed; `activity_id@version` remains the identity, and a disagreement between them is a defect to
  surface, not to reconcile in the runtime.

**What follows**

- `Runtime` can route on `StepKind`, which is what `conformance_every_step_kind_has_an_executor` measures.
- The run record gains the steps each activation walked, making a run readable against a catalog that has
  moved since.
- The OWED join record (`RecordedSpec`'s docstring) acquires a second transcribed value, which strengthens
  rather than weakens the case for writing it: two values now stand on the same provisional warrant.

## Evidence and confidence

- [measured 2026-09-22] `rk_types::activity::ActivitySpec` has exactly five fields — `card_digest`,
  `tools`, `schema`, `timeout_secs`, `inputs` — and its module header names them as `StepSpec`'s.
- [measured 2026-09-22] `catalog::Activity` carries `pub steps: Vec<Step>` as its last field
  (`catalog.rs:351`), and `Step` carries `id`, `kind`, `run`, `output_type`, `role`, `when`, `expr`,
  `for_each`, `retry_max`, `repeat_max`, `memo_declared`.
- [measured 2026-09-22] `run.rs:1423` already reads `activity.steps` at bind time, for the role only.
- [documented] `runtime.rs:22-27` carries residual 7 verbatim and asserts nothing reachable from `runtime`
  reaches `Catalog::from_dir`.
- [documented] `replay.rs:640-664` — `RecordedSpec`'s two fields, its `supplied` constructor *"named on
  [`RuntimeContext::supplied`]'s model: every call site is transcribing something this crate did not
  derive"*, and the *"NOT a sixth spec field"* clause quoted above.
- [measured 2026-09-22] `supply_spec: &mut dyn FnMut(&str) -> Result<RecordedSpec, ReplayError>` is the one
  supply channel (`replay.rs:1495`, called at `:1569`).
- **Confidence: high on the two closed routes, each forbidden by a record this tree already carries and
  asserts.** What is **not** measured is the fan-out of the transcription itself — how many call sites
  construct a `RecordedSpec` once tests are counted. Clause 4 is written for that measurement to come out
  badly, and names what to do if it does rather than leaving the switch to judgement.
