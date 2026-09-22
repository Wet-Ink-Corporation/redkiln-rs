---
id: "0144"
title: "A step that fails **after** its activation's dispatch has committed **refuses by name**, and the commit **stands**. The failure travels on its own field (`ActivityReport::step_error`) and out through its own refusal (`ReplayError::StepFailedAfterCommit`), because `outcome` is the dispatch's own reading and `committed()` is the only question `dispatch_through` asked. Until this record the walk discarded that error outright whenever a dispatch had already happened, so a run that executed **nothing** after its dispatch reported a success — voiding ADR-0135's by-name refusal at the last step, for a re-measured **33 of 102** activities, **29** of which reach a step that cannot succeed on this tree at all"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0135 (the `tool` verb routing, and the by-name refusal of 29 of 33 verbs); references/adr/0138 (AC-013's grain stays per ACTIVATION); references/adr/0143 (the committed report is never overwritten — this is its sibling defect on the other arm); crates/rk-kernel/src/runtime/step.rs (`run_activity_steps`' `StepVerdict::Failed` arm); crates/rk-kernel/src/runtime/activity.rs (`ActivityReport`, and `outcome` set beside `commit` on the success path); crates/rk-kernel/src/replay.rs (`dispatch_through`, whose only question was `committed()`); crates/rk-cli/src/verbs/run.rs (the `Err(e) => Exit::ConfigError` catch-all); crates/rk-cli/src/exit.rs (`impl From<&StepError> for Exit`, eleven arms, no wildcard)
supersedes: []
superseded_by: []
related: ["0032", "0135", "0138", "0142", "0143"]
evidence: []
consumers: []
---

## Context and problem

`run_activity_steps` walks an activation's steps in declaration order. Its `Failed` arm read:

```rust
StepVerdict::Failed(e) => {
    return Ok(dispatched.unwrap_or_else(|| ActivityReport::refused(now, e)));
}
```

`dispatched` is `Some` once an `agent` step has run. So for every activation whose walk reaches a
failing step **after** its dispatch, `e` is discarded and the committed report is returned unchanged.

`replay::dispatch_through` then asks one question:

```rust
if !report.committed() { return Err(ReplayError::NotCommitted { .. }) }
Ok(journal delta)
```

`committed()` is `true`. The function returns `Ok`. The loop folds on, the run completes, and
`terminal_read` returns `Exit::Success`.

**This voids ADR-0135 at the last step.** That record refuses 29 of the 33 `tool` verbs **by name**,
for one stated reason: *"a silent fallthrough would turn 66 of 113 steps into invisible non-events,
and a run that executed nothing would be indistinguishable from one that worked."* The refusal was
raised correctly by `run_tool_step`, carried correctly by `StepVerdict::Failed` — and then dropped
by the arm that received it. The same is true of every `check` that does not hold, of every guard
ADR-0136 refuses, and of every port error a mapped verb returns.

**The population is re-measured here rather than restated.** `PROGRESS.md` row 37 records *"38 of
102"*, taken **before ADR-0143** made a second runnable `agent` step end the walk. Re-derived at this
repair, with the walk's real termination modelled:

| reading | count |
| --- | --- |
| any non-agent step after the first agent step | 82 of 102 |
| reaches a `tool` or `check` step before the walk ends | **33 of 102** |
| …and that step **cannot succeed on this tree today** | **29 of 102** |

The last row is the one that matters: for 29 activities the dropped error is not hypothetical. It is
the error the run was always going to produce.

## Decision drivers

1. **A refusal that does not reach the operator is not a refusal.** ADR-0135's whole warrant.
2. **The commit is real and may not be un-said.** The `activity_committed` row is in the journal
   before the later step runs. AC-013's grain is per activation (ADR-0138) and a re-entry will skip
   it. A report claiming `committed() == false` would be false, and would be ADR-0143's defect with
   the sign reversed.
3. **The dispatch's own reading may not be overwritten to carry someone else's error.** That is
   ADR-0143's defect one field over, and it is what makes a second field necessary rather than tidy.
4. **§7d.4's exit table is closed and its rows are a committed snapshot.** A repair that needed a new
   row would be a much larger change than this defect warrants.

## Considered options

**(a) Return `refused(now, e)` unconditionally, discarding the committed report.** One line, carries
the error, and makes the tests green. Rejected: it reports `NotCommitted` for an activation that
committed, which is a false statement about the journal, and it re-opens exactly the
`NotCommitted { class: … }` confusion Mechanism B was added to close.

**(b) Write the step's error into `ActivityReport::outcome`.** Also small. Rejected on measurement:
`outcome` is already spoken for in both shapes a committed report takes — `Some(Ok(typed))` on the
success path, where it is set beside `commit`, and `None` on AC-013's re-entry. Either way the
dispatch's own reading would be destroyed to report the walk's.

**(c) Keep the commit, carry the failure on its own field, refuse with its own variant.** Taken.

**(d) Add a row to §7d.4's exit table.** Rejected as unnecessary: the exit code is the *class's*,
and `impl From<&StepError> for Exit` already maps every class. Only the message differs.

## Decision

1. **`ActivityReport` gains `step_error: Option<StepError>`**, set only by the walk, through a
   consuming `pub(super) fn with_step_failure`. `refused`'s precedent holds: a report is produced by
   the loop and never minted beside it, and every count in the returned value is the committed run's
   own reading, unchanged.
2. **The walk's `Failed` arm keeps the commit and attaches the error.** `Some(done) =>
   done.with_step_failure(e)`, `None => refused(now, e)`. The `None` branch is unchanged: an
   activation that never dispatched is still refused outright.
3. **`dispatch_through` asks `step_error()` BEFORE `committed()`**, because `committed()` is `true`
   on this report and would end the function. It returns
   **`ReplayError::StepFailedAfterCommit { step_id, activity_id_at_version, class }`**.
4. **`class` on that variant is NOT `Option`.** Unlike `NotCommitted`'s, it is only constructible
   from a step failure, so there is always exactly one and there is no `None` branch to invent a row
   for.
5. **The CLI carries an EXPLICIT arm for it.** `run.rs`'s `Err(e) => Exit::ConfigError` catch-all
   maps everything it is not told about to row `3` — which is the *"all eleven rows arrived here as
   the same value"* failure Mechanism B exists to close. A new variant left to fall through would
   re-open it silently, so the arm is written rather than the next author asked to remember.
6. **No row of §7d.4's exit table is added, moved or renumbered.** The exit code is
   `Exit::from(&class)`, the class's own.

## Consequences

**What this forbids**

- Forbids returning `Ok` from `dispatch_through` for a walk that ended on a step failure. A run that
  executed nothing after its dispatch may not report a success.
- Forbids discarding the commit in order to report the failure. The `activity_committed` row exists;
  saying otherwise is a false statement about the journal and breaks AC-013's re-entry contract.
- Forbids writing a later step's error into `ActivityReport::outcome`, which is the dispatch's own
  reading in both of its shapes.
- Forbids letting a new `ReplayError` variant reach `run.rs`'s catch-all. Every variant that can
  carry a terminal class gets an explicit arm, or the class is lost and the run exits `3`.
- Forbids adding an exit row for this. The class's row is the answer; a second row for *"a class, but
  later"* would make the table describe when an error happened rather than what it was.

**What follows**

Nothing becomes executable. **29 of 102** activities now fail where they previously reported success,
carrying the cause they always had — that is the whole gain, and this record does not dress it as
more. It does not repair the `check` arm (ADR-0142 clause 4 measures 94 of 94 shipped `expr`s reading
a prior step's output, and step-output resolution stays OWED), it does not touch the `gate` arm, and
it does not widen AC-013's grain.

🛑 **Row 37's "38 of 102" is superseded by the table above and was true when written.** The number
moved because ADR-0143 changed where a walk ends, not because it was measured wrongly — which is the
standing hazard that a number is the one kind of citation carrying no timestamp inside itself.

## Evidence and confidence

**Measured, not argued.** The guard is
`rk-kernel/tests/it/post_commit_failure.rs::a_step_failing_after_a_committed_dispatch_refuses_by_name_and_keeps_the_commit`,
which asserts all three halves — the failure is carried and names the verb, the commit survives, and
the dispatch's own `outcome` is untouched — each of which is satisfiable by a wrong fix the other two
reject. It carries a live control: the same walk with the failing step removed must commit and report
no `step_error`, or `step_error` would be satisfied by a constant.

**Mutant-tested.** Reverting the `Failed` arm to its pre-record one-liner, in an isolated
`git clone --local` export, turns it RED for the right reason.

**Confidence: high on the defect, high on the mechanism, and the population is a structural reading.**
The 33/29 counts model the walk's termination from the catalog text; they do not model which guards
ADR-0142 leaves evaluable at run time, and the script that derives them asserts its filter non-empty
before counting — the pack spells the table `[[step]]`, and a `[[steps]]` filter returns a confident
zero over all 102 files.
