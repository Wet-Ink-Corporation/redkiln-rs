---
id: "0161"
title: "**Three defects this stage's own adversarial pass found in this stage's own work, all of them in code that was green.** (1) ADR-0157's `to != state.stage` guard is **insufficient for a lane longer than one hop** and is superseded in part: `apply` sets `state.stage` to the LAST transition's target while the composition root computes the successor from the item record's stage, so a two-hop lane's next run appends a **BACKWARD** `stage_transition` and then re-advances — two tier-1 rows per run, for ever. The test is `stages_entered`, which already existed and was unused. (2) **A non-zero exit alongside an advance was DISCARDED**: the lane loop read its exit only on the no-advance path, so all four of `story_trailer`'s refusals reached stderr and the process still exited on the last stage's status — ADR-0159's *\"refuses by name\"* was unobservable to any caller reading an exit code. (3) **A `[project] name` containing `/` breaks the axis silently**: `split_qualified` takes the FIRST slash, so the mark parses under a different project than it was written for, the idempotence read can never match it, and it re-mints as `duplicate-story-trailer` drift on every run"
status: accepted
date: 2026-09-23
legacy_id: none
source: crates/rk-kernel/src/workflow/fsm.rs (`apply`'s `StageTransition` arm — `state.stages_entered.push(to.clone()); state.last_transition = …; state.stage = to;`; `RunState::stages_entered` and its accessor; `next_action`'s successor arm); crates/rk-cli/src/verbs/run.rs (the lane loop's `let Some(next) = advanced else { return Ok(last_exit) }`; `story_trailer`'s four refusal paths; `story_value`); crates/rk-git/src/trailer/grammar.rs (`split_qualified` — `v.split_once('/')?`, the FIRST slash); crates/rk-git/src/reconcile.rs (`stories_committed` keyed per project); crates/rk-kernel/tests/it/lane_order.rs (the two-hop test and mutant M10); references/adr/0157 (clause 1, superseded in part); references/adr/0152 and references/adr/0159 (extended, not superseded)
supersedes: ["0157"]
superseded_by: ["0183"]
related: ["0152", "0155", "0157", "0159"]
evidence: []
consumers: []
---

## Context and problem

`s7h` ran its adversarial pass against its own closing state: nine records accepted, seven lane
fixtures green, nine mutants each diffed and read, four suites green, fmt and clippy clean, and
Checkpoint 7h failing only on the bullet it records as OWED. Two refuting verifiers then found three
defects, **all in code this stage wrote**, and none of them visible to anything that was green.

**1. ADR-0157's guard is a proxy, and it is sufficient only for a one-hop lane.** That record guarded
the advance arm on `to != state.stage`, reasoning that *"the equality IS the 'already there' test"*. It
is not. `apply`'s `StageTransition` arm sets `state.stage` to the **last** transition's target, while
the composition root computes the successor from the item record's stage, which — as ADR-0157 itself
records — never moves. So for a lane `[triage, analysis, specification]` walked in one process:

| | fold's `state.stage` | supplied successor | `to != state.stage` | result |
| --- | --- | --- | --- | --- |
| run 1, hop 1 | `triage` | `analysis` | true | advance ✓ |
| run 1, hop 2 | `analysis` | `specification` | true | advance ✓ |
| **run 2** | **`specification`** | **`analysis`** | **true** | **advance — BACKWARD** |

Run 2 appends `{"from":"specification","to":"analysis"}`, the lane loop re-enters at `analysis`, and
`{"from":"analysis","to":"specification"}` follows. Two tier-1 rows per invocation, without bound —
**the exact failure ADR-0157 claimed to have closed, one hop further out.**

And ADR-0157's own experiment could not have seen it: run 1 stops at `analysis` with exit 3 because
that stage's bindings refuse, so the shipped tree has never taken two boundaries in one process. Its
guard, its test and its record all instrument the one-hop case only.

**2. A non-zero exit alongside an advance was discarded.** `resume_and_read` sets `*advanced` before it
calls `story_trailer`, and the lane loop reads its exit only on the no-advance path:

```rust
last_exit = run_with_advance(…, &mut advanced)?;
let Some(next) = advanced else { return Ok(last_exit); };   // last_exit read ONLY here
// … advance, re-iterate; last_exit overwritten next pass
```

So with no `[project] name` declared, a lane that takes a boundary prints ADR-0159's refusal to stderr
once per boundary, mints nothing, **keeps advancing**, and exits on the last stage's status — `0` if the
lane ends on `Done` or a gate. All four of `story_trailer`'s `ConfigError` paths were affected. **A
refusal nothing can observe is not a refusal**, and ADR-0159's whole decision was to refuse rather than
emit a silently-inert trailer.

**3. A project name containing `/` breaks the axis silently.** `story_value` is `{project}/{item}` and
`split_qualified` takes the **first** slash. `name = "acme/web"` therefore emits
`Story: acme/web/<item>`, which parses as `project = "acme"`, `slug = "web/<item>"`. Two failures at
once: the idempotence read looks up `state.projects.get("acme/web")`, a key `reconcile` can never
produce, so the producer re-mints on **every** boundary and every run, each fold reporting
`Drift::DuplicateStoryTrailer` — precisely the state `story_trailer`'s own header says it exists to
prevent; and the slug matches no `.redkiln/items/` stem, so `doctor --rebuild` reports it
`unresolvable`. Nothing validated the name: `Project` is a bare `String`.

## Decision drivers

- Defect 1 is a correction to a **`## Decision` clause** of an accepted record, so it is a supersession
  and not an erratum (ADR-0092 decision 1). Defects 2 and 3 extend ADR-0152 and ADR-0159 without
  moving any of their clauses.
- `stages_entered` already exists, is already pushed by the same `apply` arm that moves `state.stage`,
  and already has a public accessor. The sufficient guard needed no new state — which is also why
  ADR-0157's proxy is the wrong kind of shortcut: the real answer was one field away.
- A guard whose fixture cannot reach the failing case is the shape this runbook has paid for
  repeatedly. ADR-0157's fixture could hop once; the replacement's fixture hops twice, and the mutant
  that restores the old guard must go red against it or nothing has changed.
- An exit code is the only channel a caller outside the process can read. A refusal that reaches stderr
  and not the exit status is indistinguishable from success to CI, to a script, and to Checkpoint 7h.
- Validating the project name at the mint matches ADR-0159's own siting of its refusal (at the mint,
  not at startup), so a run that mints nothing is still unaffected by a bad name.

## Considered options

**For defect 1.** *Compute the successor from the fold rather than the record* — inverts the order, the
fold is `resume`'s and the context is an input to it, and it leaves the arm still able to advance on a
successor equal to an already-entered stage. *Refuse when the record and the fold disagree* — refuses
the ordinary case, since after any successful advance they legitimately differ. *Guard on
`stages_entered` (chosen)* — answers the question the proxy was approximating, with state that already
exists.

**For defect 2.** *Return the exit from `story_trailer` through `advanced` somehow* — conflates two
channels that exist precisely because a boundary and a failure are different facts (ADR-0152's carrier
note). *Check the exit inside `run_with_advance`* — puts a lane invariant inside a single stage's
function. *Check it in the loop, before the advance (chosen)* — the lane is the loop's invariant: a
stage that failed has not earned a successor.

**For defect 3.** *Percent-encode or escape the name* — invents an encoding no reader implements.
*Accept it and key the idempotence read on the parsed project* — makes the emitted value and the parsed
value legitimately differ, which is the drift this axis is built to detect. *Refuse a name containing
`/` (chosen)* — one character class, refused where every other bad name is refused.

## Decision

**1. ADR-0157 clause 1 is superseded in part.** The advance arm fires only when the successor names a
stage the fold has **not already entered**: `!state.stages_entered.iter().any(|s| s == to)`, retaining
`to != state.stage`. Every other clause of ADR-0157 stands, including its named cross-process residual,
and it keeps `status: accepted`.

**2. A stage that failed has not earned a successor.** The lane loop returns a non-`Exit::Success`
status immediately, **before** reading `advanced`. A boundary taken by a stage whose run then failed
does not continue the lane.

**3. A `[project] name` containing `/` is refused at the mint**, naming the character and the reason.
`<project>` is a single path segment.

**4. The two-hop case is instrumented.** The fixture pack gains a third live stage so a lane can take
two boundaries, and the test walks both and then replays — which is what a second `redkiln run` does.
Mutant **M10**, which restores ADR-0157's guard exactly, is RED against it.

## Consequences

The advance is now idempotent for a lane of any length rather than for a one-hop lane, and the axis's
refusals are observable where a caller can see them. ADR-0159's refusal becomes real: before this, it
could not fail a run.

The honest cost is what this record says about the previous nine: **they were all green when these
three defects were in the tree**, and two of the three are in the seam between the kernel's report and
the composition root's loop — which no unit fixture in either crate covers, because the lane loop has
no automated test at all. That gap is named here and not closed: `grep -rln "verbs::run::run(" crates/*/tests`
returns nothing, so every claim about the loop's behaviour rests on manual binary runs.

**What this forbids**

- Guarding the advance arm on `to != state.stage` alone.
- Continuing a lane past a stage whose run returned a non-`Exit::Success` status.
- Emitting a `Story:` value whose `<project>` contains `/`, or any character that makes the emitted
  value and the parsed value differ.
- Keying an idempotence read on a project string that `reconcile` cannot produce.
- Describing the lane loop as covered by tests, or reading a green suite as evidence about it.
- Reading ADR-0157's *"the boundary is taken once, durably, and never again"* as true for a lane
  longer than one hop — it was true of the one-hop case its experiment exercised.

## Evidence and confidence

**All three defects were reproduced from the code, and the fixes are each killed by a mutant.**

Defect 1's mechanism is three lines in one arm: `apply` pushes `to` onto `stages_entered` **and** sets
`state.stage = to`, so after two hops the two disagree about which question `to != state.stage`
answers. The new two-hop test walks `triage → implement → closeout` and then replays with the item
record's stage; **mutant M10 removes the `stages_entered` conjunct and the test is RED** at
`lane_order.rs:555` reporting `left: Some("implement")` against `right: None` — the backward advance,
caught by name.

Defect 2 is read off the control flow quoted above: `last_exit` has exactly one read, and it is inside
the `else` of the `let Some(next) = advanced`.

Defect 3 follows from `split_qualified`'s `v.split_once('/')?` taking the first slash, and from
`reconcile`'s `stories_committed` being keyed per parsed project.

**Confidence, and what is NOT asserted.** None of the three fixes is exercised by an end-to-end test of
the lane loop, because no such test exists; defect 2's fix in particular is asserted by reading the
control flow and by one manual run, not by a fixture. This record does not claim the loop is now
correct — it claims three specific defects are closed and instrumented where instrumentation was
possible. It also asserts nothing about a fourth finding the same pass produced and this stage does not
own: `terminal_read` scans for any tokened `gate_opened` row with **no** answered/void/expired filter —
unlike `RunState::awaiting_gate`, which excludes all three — so after a gate is answered it re-commits
a `Gate:` trailer for the dead token and prints that the gate is open. That is 7d's producer and it is
recorded, not repaired.
