---
id: "0205"
title: "**A bug with a placed repro implements against it; agent outputs that feed a test selection are checked at the agent step; a walk-ending failure reaches stderr and the journal** (`s8`'s repro-aware implement; the operator's ruling of 2026-11-08 22:34Z after `rk-fix-doctor-margin-2` exited 19 at `red-verify`): implement for a bug with `has_repro = true` is `repro-green-refactor@1`, which re-proves the committed repro red and holds the fix green on it and on `affected`; `tdd-red-green-refactor@4` tells each agent step its contract and loops back on an all-empty red; a selection-feeding agent field that is not a plain command is a retryable `Schema` failure whose re-prompt carries `refused`; a walk that ends at a non-agent step appends one error row and `run` names the class on stderr"
status: proposed
date: 2026-11-09
legacy_id: none
source: the 2026-11-08 run of `rk-fix-doctor-margin-2` (`tdd-red-green-refactor@3`'s `red` committed the fix and named the prose test `"doctor test"`, `red-verify` refused it as `Unsupported`, exit 19, and no journal row and no stderr line named the step); Ryan's ruling, card 2026-11-08 22:34Z ("Repro-aware tdd plus hardening"); his approval of the plan's decisions D1-D14 as recommended, card 2026-11-08 23:51Z; spikes/plans/s8-repro-aware-implement-2026-11-08.md; spikes/DECISIONS-2026-09-21.md D21-177
supersedes: []
superseded_by: []
related: ["0031", "0072", "0095", "0173", "0188", "0204"]
evidence: []
consumers: ["crates/rk-kernel/src/runtime/reference/selection.rs", "crates/rk-kernel/src/runtime/activity.rs", "crates/rk-kernel/src/runtime/step.rs", "crates/rk-cli/src/verbs/run.rs", ".redkiln/catalog/repro-green-refactor.toml", ".redkiln/catalog/tdd-red-green-refactor.toml"]
---

## Context and problem

**The implement stage asked an agent to write red tests for a bug whose red test was already committed.** ADR-0204
made the committed repro pair the fix lane's red proof: analysis places it and runs it red. Implement was still
`tdd-red-green-refactor@3`, whose `red` step asks an implementer for new failing tests. On the operator's run of
`rk-fix-doctor-margin-2` (2026-11-08), `red` for_each:0 made the fix itself and named no test. for_each:1 named
`"doctor test"`. `red-verify` passed that token to `check_selection_token`, which refused it as `Unsupported`. The walk
ended with exit 19, and `lib.rs` had been committed by a step called `red`.

**Nothing told the red step what a red step is.** Its inputs were the spec and the criterion. No input stated the
contract.

**The refusal came one step too late.** ADR-0204 decision 6 refuses a non-plain selection at the `toolchain:test`
step. By then the agent's answer is committed, and the agent is never asked again. ADR-0204 left this as an owed
issue (its "D4, option c").

**The failure was silent.** `run` printed the sentence without the step's class, and a walk that ended at a
non-agent step appended no row to the journal, so `explain` had nothing to name.

## Decision drivers

- **A committed, human-written repro is the red proof for a bug.** An agent never re-derives it.
- **A bad agent answer is the agent's to correct.** A retryable class, with the refusal in the next prompt (AC-031).
- **One parser for a selection** (ADR-0173). It is called earlier, never changed.
- **Cards and the claude adapter stay untouched** (the D10 guard over `crates/rk-adapter-claude`,
  `crates/rk-containment`, `.redkiln/cards` and `tools/live-run.sh`).
- **No new `StepError` variant, no new `EventKind`, no port change, no new dependency.**

## Considered options

### Option — A separate implement-loop member, `repro-green-refactor@1` (taken)

Rank 15, `applicable_if = "has_repro"`, chosen by step 4 for a bug with a placed repro. `tdd` keeps its shape for
stories. Taken: plan D1, option C.

### Option — `tdd@4` with `when` guards on `has_repro`

Not taken: a `null` facet (every story) would skip red silently, and `green` and `emit` would have to go optional.

### Option — `tdd@4` reading `artifact:ReproCase?`

Not taken: guards cannot read artifacts, so it needs an ADR-0173 grammar change.

### Option — A `schemars` pattern on `ImplReport.tests_added`, or an adapter ingest check

Not taken: the first spells the token rule a second time and moves a schema snapshot; the second sits inside the
D10 guard and re-opens the paid steps (plan D5, options ii and iii).

### Option — A new tier-2 `EventKind` for a walk-ending failure

Not taken: a closed-set change that needs its own record (plan D7, option J2).

## Decision

**1. The repro-aware implement.** `.redkiln/catalog/repro-green-refactor.toml`, version 1, family `implement-loop`,
`method_rank = 15`, `applicable_if = "has_repro"` (plan D1 option C, D2 option A). Its steps:

- `red-proof` runs `artifact:ReproCase.command` through the one parser.
- `red-holds` checks it red (`failed > 0 and passed == 0`).
- `not-red` opens the `repro.cannot-reproduce` gate otherwise (plan D3 option A).
- `green` asks the implementer for the fix, with a `task` literal stating the contract.
- `repro-verify`, `green-verify` and `green-gate` hold the repro and `affected` green.
- `refactor`, `refactor-verify`, `refactor-repro` and `still-green` repeat the hold after the refactor.
- `summary` and `emit` compose the `ImplReport`.

Stories, and bugs without a placed repro, keep `tdd-red-green-refactor`.

**2. `tdd-red-green-refactor@4`.** `red`, `green` and `refactor` each read a `task` literal naming the step's
contract: `red` writes failing tests only and names them as bare test names (plan D4 option i). A new check,
`red-named`, loops back to `red` when the aggregate of names is empty. A deterministic check that `red` changed
only test paths is not built; it is an owed issue (plan D4 option ii).

**3. The feed check.** `rk_kernel::runtime::reference::selection_feeds(steps)` maps each agent step to the
`toolchain:test` readers it feeds. A feed is a mandatory, plain `${<agent>.outputs.<path>}` reference: `[*]` only on
a `for_each` agent, and a plain path only on a non-`for_each` agent. `check_feeds(value, feeds)` runs each fed value
through `parse_selection` and `check_selection_token`. `run_activity_at` calls it after the schema check and before
commit. A refusal is a `StepError::Schema` failure, with the `FeedRefusal` text, so AC-031's re-prompt limit applies
(plan D5 option i). A served row is not re-checked, so a replay or `explain` over a store holding an old bad value is
unchanged.

**4. The re-prompt carries the refusal.** Every `Schema` re-prompt dispatches the step's spec with one extra input,
`refused`, whose value is the last error. `refused` is a reserved input name: no shipped step declares it, and a
test pins that (plan D6 option A).

**5. A walk-ending failure is journalled.** When the walk ends on a failure or a stop at a non-agent step,
`Runtime::end_walk` appends one `error_row` with `step_id` and `activity_id_at_version`. `Unsupported` becomes
`step_refused`, which halts the item (ADR-0095). Other classes become `step_failed` (plan D7 option J1).

**6. `run` names the class.** On `StepFailedAfterCommit`, `run` writes the sentence and then
`redkiln: the step's class: <class>` to a locked stderr. The exit code is unchanged.

**7. Public API.** Additive: `StepAt::feeds`, and `SelectionFeed`, `FeedRefusal`, `selection_feeds` and
`check_feeds` beside `parse_selection` (plan D8).

This record amends nothing in ADR-0204 or ADR-0173. It answers ADR-0204's owed issue "D4, option c": decisions 3
and 4 are that check. The operator files that issue closed as done, or does not file it.

## Consequences

- A bug with a placed repro lands its fix through `green`, held green on the repro and on `affected`. No agent writes
  or names its red test.
- A prose test name from an agent is re-asked once with the refusal in its inputs, then refused as `Schema` with
  nothing committed. It is never a walk-ending `Unsupported` at the tool step.
- `explain` can name a walk-ending tool-step failure, because the journal holds its row.
- **`step_refused` halts the item.** An operator who re-launches the same item after a catalog fix meets the halt.
  That is consistent with ADR-0095 and D21-163's route, which mints a new item.
- **Measured at this record's writing (2026-11-09):** a bug with `has_repro = null` does not open an
  `unknown_facet:has_repro` gate. Step 4 records the abstention and picks the lowest admissible rank, so it keeps
  `tdd-red-green-refactor`, exactly today's choice. Whether step 4 should gate on a lower-rank abstention is an
  open operator question (AC-079, ADR-0103). It is not decided here.
- An agent step's own pre-dispatch stop (no per-step spec, an input clash, an unbound role family) gets no error row
  under decision 5's "non-agent" rule.
- Implement runs the repro up to three times, besides `affected`.

**What this forbids**

- An agent step whose answer feeds a `toolchain:test` selection committing a value that `parse_selection` or
  `check_selection_token` refuses.
- A `Schema` re-prompt that dispatches the same inputs as the refused attempt.
- A shipped catalog step that declares an input named `refused`.
- A walk that ends at a non-agent step on a failure or a stop without one journal row naming the step.
- `run` exiting on `StepFailedAfterCommit` without naming the step's class on stderr.
- Selecting `tdd-red-green-refactor` for a bug whose `has_repro` is `true`.

### Owed issue (plan D4, option ii)

The implementer does not create issues. The operator files this one, with the backlog's issue body contract
(`docs/BACKLOG.md`):

> **Title:** A deterministic check that `tdd`'s `red` changed only test paths (`toolchain:test_scope`, both adapters)
>
> **Body:** `tdd-red-green-refactor@4`'s `red` is told by a `task` literal to write failing tests only (ADR-0205
> decision 2). Nothing enforces it: on 2026-11-08 a `red` step committed the fix itself. A `toolchain:test_scope`
> verb in both toolchain adapters, with a registry row, a `VerbOutput` and a `check` over `red`'s paths, would refuse
> a production path at `red`. Done when a `red` answer that writes a non-test path fails at a check step, with a
> test per adapter.

## Evidence and confidence

- rk-kernel `selection_feeds::*` (the feed map), `feed_check::*` (the check as `Schema`, the re-prompt with
  `refused`, twice refused commits nothing, a served bad value is not re-checked), `test_selection::a_tool_step_failure_after_a_commit_appends_one_error_row`,
  `selection::repro_green_refactor_is_chosen_for_a_bug_with_a_repro`, `selection::no_shipped_step_declares_the_reserved_input_refused`
  and `on_false_exec::tdd_red_named_loops_back_on_an_all_empty_red`.
- rk-cli `step_refused::a_walk_ending_refusal_names_its_capability_on_stderr`,
  `lane_proof::improvising::fix_lane_with_an_improvising_implementer_lands_through_the_repro` and
  `lane_proof::improvising::story_lane_red_with_a_bad_test_name_is_reprompted_as_schema`.
- The mutants `M-feed-check-off` (kernel), `M-repro-aware-unselected` and `M-step-class-silent` are in
  `runbook/tools/s8j-mutants.py`. Their killed lines are the operator's, from §8j.6 step 1b.
- Confidence: high for the kernel check and the selection. The real implementer's behaviour under the `task`
  literals is measured on `rk-fix-doctor-margin-3`'s run, not here.
