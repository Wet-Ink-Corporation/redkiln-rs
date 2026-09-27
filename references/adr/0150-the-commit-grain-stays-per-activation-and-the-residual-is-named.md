---
id: "0150"
title: "The commit grain stays **per activation**, and the residual is NAMED rather than engineered away: an activation's `activity_committed` is appended when its `agent` step dispatches, so every later step in the walk runs inside an ALREADY-COMMITTED activation, and `RunState::is_committed` has no step component — a run that dies after the commit and before those steps finish will skip the whole activation on re-entry and **never run them**. Measured 2026-09-22: **39** mapped `tool` steps across **23** activities sit after the first `agent` step. Moving the commit to the end of the walk is refused, because a crash after a successful dispatch would then re-dispatch it, which is exactly what AC-013 and `kill_and_resume_n_step` forbid and what ADR-0143 and ADR-0144 each spent a record protecting. **And the module header asserted the opposite** — *\"the walk happens inside an activation that is not yet committed\"* — which was a description of the intent, not of the behaviour; it is corrected in the same change"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/runtime/step.rs (the module header's AC-013 grain note, corrected here; the `agent` arm's `run_activity` call); crates/rk-kernel/src/runtime/activity.rs (`is_committed`'s pre-flight skip, which has no step component); references/adr/0138 (the activation grain); references/adr/0143 (a committed report is never overwritten); references/adr/0144 (a post-commit step failure refuses by name and the commit STANDS)
supersedes: []
superseded_by: ["0174"]
related: ["0087", "0112", "0135", "0138", "0143", "0144", "0148"]
evidence: []
consumers: []
---

## Context and problem

`run_activity_steps` walks an activation's steps in declaration order. When it reaches the `agent`
step it calls `run_activity`, which dispatches and **appends `activity_committed`**. The walk then
continues into the remaining steps — which therefore execute inside an activation that is already
committed.

`RunState::is_committed(item, activity_id_at_version)` has **no step component**. Its pre-flight
returns `dispatched: false` for any activation already in the journal. So if the process dies after
the commit and before the walk finishes, the next run skips that activation entirely and the
remaining steps **never execute and never retry**.

Measured over `.redkiln/catalog/*.toml` on 2026-09-22: **39** `tool` steps naming one of ADR-0135's
four mapped verbs sit after the first `agent` step, across **23** activities —
`test` 26, `change_summary` 9, `lint` 2, `build` 2.

🛑 **And the module header said the opposite.** It read:

> *"the walk happens **inside** an activation that is not yet committed. The activation commits
> atomically. The consequence is intended rather than tolerated: a run that dies mid-activation
> re-runs that activation's steps."*

That is true of the steps **before** the dispatch and false of every step after it. ADR-0144 exists
precisely because a step can fail *after* a committed dispatch, so the tree already knew the
sentence was wrong when it was written. It is corrected in the same change as this record, rather
than left to be found — the ADR-0146 shape, where a clause described the defect as the intent.

## Decision drivers

- **Moving the commit to the end of the walk is not a smaller change; it is the opposite defect.**
  If the commit landed after the last step, a crash between a successful dispatch and the walk's end
  would leave no `activity_committed` — so the next run would **re-dispatch the agent step**. That
  is re-execution of work that already happened: what AC-013 forbids, what
  `kill_and_resume_n_step` asserts, and what criterion (3) is.
- **Two accepted records depend on the commit standing.** ADR-0143: a committed report is never
  overwritten. ADR-0144: a post-commit step failure refuses by name and **the commit STANDS** — with
  `refused(now, e)` named there as the tempting one-line fix that reports `committed() == false` for
  an activation whose row is in the journal.
- **A per-step commit grain is a different design, not a repair.** It needs per-step
  `ActivitySpec`s — already OWED under ADR-0143 clause 5 — and an idempotency key with a step
  component. Inventing half of that here to close a crash window would leave the harder half owed
  and the grain incoherent.
- **The live exposure is far smaller than 39, and saying so is part of the decision.** Under
  ADR-0148, `test` (26) now refuses; under ADR-0135 clause 4, `change_summary` (9) reaches the
  adapters' dated refusal. So **4** of the 39 — `lint` 2 and `build` 2 — are steps that could
  succeed today and be lost to this window. That does not make the window acceptable; it bounds it.

## Considered options

**(a) Move `activity_committed` to the end of the walk.** Rejected, and it is the one that looks
like a one-line fix. A crash between a successful dispatch and the walk's end would then leave no
`activity_committed`, so the next run **re-dispatches the agent step** — re-execution of work that
already happened, which AC-013 forbids and `kill_and_resume_n_step` asserts.

**(b) Give `is_committed` a step component.** Rejected as premature: without a spec and an
idempotency key at step grain, a step-aware `is_committed` would be asking a question the rest of
the machinery cannot answer, and would report "not committed" for activations that are.

**(c) Have the walk re-run only the trailing steps on re-entry.** Rejected: the walk restarts from
step 0, and "which steps already ran" is exactly the state no row in the journal carries. Inventing
it here means inventing the per-step grain in (b) without the record for it.

**(d) Record the grain, name the residual, bound it, and route the repair.** Taken.

## Decision

1. **The commit grain stays per activation.** `activity_committed` is appended when the `agent` step
   dispatches, and the steps after it run inside a committed activation. This is not changed here.

2. **The residual is recorded, not hidden**: a process death between the commit and the end of the
   walk loses the remaining steps, because `is_committed` has no step component. **39** steps across
   **23** activities are in that position; **4** of them could succeed today.

3. **The module header is corrected** in the same change, and the correction is dated and states
   what the old sentence got wrong rather than quietly replacing it.

4. **The repair is an idempotency key with a STEP component, and no record currently routes it.**

   🛑 **This clause named ADR-0143 clause 5 and that was a conflation, caught by an adversarial pass
   and corrected here rather than left.** Clause 5 reads: *"Per-step specs are OWED. A spec per
   **agent** step — with its own card digest, schema and inputs — is what would let an activity's
   declared **agent steps** each execute."* It is scoped to *agent* steps in multi-agent activities.
   The residual above is about the **39 mapped `tool` steps** that sit *after* the first agent step.
   A spec per agent step gives `is_committed` no step component covering a trailing tool step, so
   clause 5 would not close this even when it lands.

   What is owed is therefore **larger than clause 5 and adjacent to it**: a commit grain, and an
   idempotency key, whose identity includes the step reached — of which per-agent-step specs are one
   input and not the whole. No record routes that today, and this one does not invent it. Until a
   record does, no partial mechanism is built.

## Consequences

Nothing executable changes. What changes is that the window is written down, bounded, and attached
to the record that will close it. A stage that inherits this must not read the walk as crash-safe
past the dispatch.

This record decides by **declining** a change, which is the honest outcome here: the tempting repair
is a one-line move that would re-introduce a criterion-(3) regression two accepted records were
written to prevent. It is recorded rather than implemented, per `00-preamble.md`'s rule that a
decision is not taken by implementing it.

**What this forbids**

- Moving `activity_committed` to the end of the walk.
- Making `run_activity_steps` return `committed() == false` for an activation whose row is journaled,
  in order to force a re-run (ADR-0144's named wrong fix, ADR-0143's with the sign reversed).
- Adding a step component to `is_committed` without the spec and key grain that would make it mean
  something.
- Restating "the walk happens inside an uncommitted activation" anywhere in the tree.
- Citing ADR-0143 clause 5 as the repair for this residual — it is scoped to `agent` steps.

## Evidence and confidence

**Measured, not argued.** The population was taken by parsing every `[[step]]` block in all 102
`.redkiln/catalog/*.toml` and counting mapped `tool` verbs occurring after the first
`kind = "agent"`: **39 steps across 23 activities**, `test` 26, `change_summary` 9, `lint` 2,
`build` 2 — summing to 39. The live exposure of **4** is 39 − 26 − 9: `lint` and `build` are the two
verbs with real adapter implementations and no dated refusal row.

**Verified by reading code, not comments** — which matters here, because a comment is what this
record corrects. `is_committed` is `fn is_committed(&self, item: &str, activity_id_at_version: &str)`
and folds on those two fields only: **no step component**. The agent arm's commit is appended inside
`run_activity`'s success path, and the walk continues unless `ended` — and on every continuing path
(fresh commit, `DiscardedLate`, AC-013 re-entry, a concurrent attempt's commit) the activation **is**
in the journal. The load-bearing half is `next_action`'s plan walk, which `continue`s past a
committed activation, so the node is never selected again and the walk is never re-entered for it.

**Confidence, and what is NOT asserted.** No test exercises the crash window; there is no fixture
that kills a process between the commit and a trailing step. This record is a statement about code
that was read, not about a scenario that was run, and it says so rather than implying a coverage it
does not have. The one thing a test does hold is the adjacent property ADR-0144 already asserts —
that a post-commit step failure reaches the caller and the commit stands.
