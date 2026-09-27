---
id: "0176"
title: "**The kernel writes the error row, and the breaker refuses from it** (`s8d`'s record R4): every failed dispatch gets exactly one `step_failed` / `step_cancelled` / `step_refused` / `rate_limited` row, written by the kernel's retry loop through `StepError::event()` whatever the harness — a `RateLimited` failure's row being AC-030's `retry_at` mark, never a second — and no harness writes one. Each row carries `at`, and `family` when the dispatch's family is bound; an agent step's commit row carries `family` too. Before an agent step's dispatch the executor folds the store's family-scoped rows with the pack's `K` and refuses an open, unexpired family before the port (AC-033..AC-035). The journalled rows are the durable mark, and no `EventKind` is minted. Supersedes nothing"
status: proposed
date: 2026-09-26
legacy_id: none
source: runbook/phase-1/s8d-step-grain-and-resolver.md §8d.1 (R4's row), §8d.5, §8d.10 (the `step_error_row::`, `breaker::` and `cross_process::` names; mutants M10 and M11) and §8d.11; spikes/DECISIONS-2026-09-21.md D21-42 20 (*"The kernel appends `StepError::event()` for every dispatch, so fake and real harnesses behave alike, and AC-033's breaker counts those rows across processes."*), D21-64 ruling 10 (this record's flip condition); references/2026-09-08-acceptance-criteria.md AC-033, AC-034, AC-035, AC-360; references/2026-09-08-architecture-design.md §5.4's event table (the `step_failed { class }` row); references/adr/0022, 0095 (decision 2), 0112, 0120, 0126; crates/rk-kernel/src/runtime/activity.rs (`Runtime::run_activity_at`'s retry loop, `error_row`, `Runtime::breaker_refusal`, `step_committed_event`, `ActivityReport::breaker_refusal`), crates/rk-kernel/src/runtime.rs (`BreakerBinding`, `Runtime::with_breaker`), crates/rk-kernel/src/runtime/step.rs (`Runtime::step_family`, the agent arm), crates/rk-kernel/src/replay.rs (`resume_routed`), crates/rk-kernel/src/workflow/breaker.rs (`BreakerState::from_events`, unchanged) and crates/rk-ports/src/fakes.rs (`FakeHarness::dispatch`) at `s8d`'s build (package P6)
supersedes: []
superseded_by: []
related: ["0017", "0022", "0095", "0112", "0120", "0126", "0144", "0174"]
evidence: []
consumers: []
---

## Context and problem

`rk-types` has carried the map from a failure to its row since Step 5: `StepError::event()`, documented *"The one
event arm this variant maps to (AC-360)"*, total over the closed eleven, returning `rk_types::event::ErrorEvent`,
whose own doc reads *"The event the kernel appends for a failed step."* Nothing in the kernel appended it. The row
was the harness's. `rk_ports::fakes::FakeHarness::dispatch` wrote it through a private bridge, `fn event_for`, on
every `Err` and on its cancel path. `ClaudeHarness` writes none: its row is ADR-0126's `dispatch_completed`, and
ADR-0126 already says the failure rows are not the adapter's — *"the kernel's `rate_limited` / `step_*` rows are the
kernel's `StepError::event()` arm's, not the adapter's"* and *"the `rate_limited` kind stays the kernel's one
producer"*. So the failure class reached the journal under the fake only, which is the gap the memory note *"the
StepError class reaches the exit code, not the journal"* recorded.

Design §5.4's table names the producer as the harness. Its row reads *"`step_failed { class }` / `step_cancelled` /
`step_refused { class }` / `rate_limited` | 2 | the `StepError` variant's payload per ADR-0095's table; `resets_at`
on `rate_limited` | the adapter, via the dispatch outcome"*.

The breaker had no call site. `BreakerState::from_events` folds family-scoped rows and refuses one without `class`
and `at`: *"`family` is present but `at` is absent or not an integer, so an opened mark would have no journaled
instant to expire against (AC-035)"*. No `src/` code called it, and no row in the tree carried `family`.
`crates/rk-kernel/src/runtime.rs`'s table of durable marks reads *"🛑 AC-033's breaker mark | **none exists** |
`RefusalClass` is closed at four and `FailureClass` at five; **ARMED, NOT DISCHARGED** since Slice 4d and unchanged
here"*.

The criteria:

- **AC-033**: *"GIVEN K consecutive `Platform` failures for one model family / WHEN the next dispatch for that family
  is requested / THEN it is refused and a durable breaker mark for the family is written."*
- **AC-034**: *"GIVEN an open breaker on one model family / WHEN a dispatch is requested for a different family /
  THEN it proceeds; the breaker never trips globally."*
- **AC-035**: *"GIVEN an open breaker / WHEN the process dies and `redkiln run` is invoked again / THEN dispatches to
  that family are still refused until the mark is read and found expired; breaker state is never held only in
  memory."*
- **AC-360**'s *Verified by*: *"a kernel test that drives every `StepError` variant of ADR-0095's table through the
  dispatch path and asserts one event each"*.

D21-42 20 took the direction, quoted in `source`. `s8d` §8d.5 fixed the details. The rows must carry *"`family` (the
role's pack family, as the breaker key already is) and `at` (the journaled instant)"*, and R4 must state them *"as
members added to ADR-0095 decision 2's event arm, and whether that moves a `## Decision` sentence"*. `(durable mark)`
is read as *"the journalled rows are the mark"*. The `EventKind` is minted by ADR-0120's route only *"If R4 finds
AC-033's THEN … needs a row of its own"*.

## Decision drivers

- **One producer per row.** A row written by the fake and not by the real harness is a proposition the test suite
  holds and production does not. That is the drift D21-42 20 names.
- **A failure written twice is two failures.** The breaker counts rows, so a second row per failed dispatch would
  open it at half its declared `K`.
- **The breaker's state is the store's** (AC-035), so its input must be rows a later process can read, written on
  every path that dispatches.
- **No kind is minted to say what an existing row already says.** `EventKind` is closed and a kind is minted only by
  record (ADR-0116's driver, restated as ADR-0120's).

## Considered options

### Option — The kernel writes one row per failed dispatch and folds its own rows before each agent dispatch (taken)

D21-42 20 as `s8d` §8d.5 details it. The bridge the fake held moves into the kernel's retry loop, the fake's appends
are removed, and the breaker reads the rows the kernel wrote.

### Option — Keep the harness as the producer and add the row to `ClaudeHarness`

Each adapter would write its own failure row. Rejected: D21-42 20 took the kernel. ADR-0126 already records the
failure rows as the kernel's. Every adapter would have to re-implement one total map and stay in step with the others.

### Option — Mint a `breaker_opened` kind for AC-033's mark

It would be a row of its own, written when the K-th failure lands. Rejected: the K-th `step_failed { class:
platform, family, at }` row is already durable, family-scoped and journalled, and the fold derives the open mark
from it. A second row would carry nothing the first does not, and the two could disagree. `s8d` §8d.5 reads D21-42
20's *"(durable mark)"* this way.

## Decision

**1. The kernel writes the error row: exactly one per failed dispatch.** In `Runtime::run_activity_at`'s retry loop,
once a dispatch's outcome is an error, the kernel appends `StepError::event()`'s arm. The outcome may be the port's
`Err`, or the boundary's own `Schema` classification of a value returned under the wrong schema (AC-031). The row
is appended after that dispatch's `dispatch` row and before the retry decision. It is appended whatever the harness
is, and a dispatch that commits writes none. A retried activation writes one row per failed dispatch.

The payload keeps the members the fake's bridge wrote: `{ class, status }` on `step_failed`, `{ class, capability }`
on `step_refused`, and nothing else on `step_cancelled`. Decision 3 adds two members.

**2. No harness writes an error row.** `FakeHarness::dispatch` appends nothing on any path: `fn event_for` and both of
its appends are removed. `ClaudeHarness` never wrote one. A harness's journal row is ADR-0126's `dispatch_completed`,
and this record does not touch it.

**3. `RateLimited`'s row is AC-030's mark, never a second.** A `RateLimited` failure always takes
`RetryDecision::WaitForReset`: its class is `WaitUntilReset`, which is neither refused before spawn nor capped
(`retry::decide`). So the kernel writes no separate row for it. The `retry_at` mark it already appends
(`{ kind: "retry_at", resets_at }`, design §5 row 18) is that dispatch's one `rate_limited` row.

**4. The members `at` and `family` are added.** Every error row the kernel writes carries:

- `at`: the journalled instant the run was handed (`JournaledNow`, AC-001). This is also true of the mark.
- `family`: added when the dispatch's family is bound. It is the pack's family for the dispatching step's role
  (decision 5).

An agent step's commit row (`activity_committed` at step grain, ADR-0174) also carries `family` when one is bound.
That row is the success the fold's reset arm reads, so `K` counts **consecutive** failures.

**These are members added to ADR-0095 decision 2's event arm** (as ADR-0112 extends that table). **No `## Decision`
sentence moves.** The table's arm column names each arm by its kind and its discriminating members
(`step_failed { class: transport }`, `step_failed { class: platform, status }`,
`step_refused { class: unsupported, capability }`). No sentence of it closes the member set. Its one closing sentence,
*"Any further variant is a successor to this record"*, is about variants. This record adds none, so it supersedes
neither ADR-0095 nor ADR-0112. Design §5.4's row takes a dated amendment under this record in its payload and
producer cells.

**5. The breaker gets its call site, and refuses from the store.** The call site is `Runtime::run_activity_at`.
Before each dispatch attempt of an activation entered at a step whose family is bound, the kernel:

- reads the store's journal. Every `EventStore` in this tree ignores the query's selector, so this is the whole log,
  and AC-034's partition is the family, not the item;
- folds it with `BreakerState::from_events` and the pack's `K` (`Pack::require_breaker_threshold`);
- decides the family with `BreakerState::decide` against the journalled instant and the pack's expiry
  (`Pack::require_breaker_expiry`).

An open, unexpired mark **refuses the dispatch before the port**. The report carries the decision
(`ActivityReport::breaker_refusal`), and its outcome is a by-name `StepError::Unsupported` naming the family and
AC-033. A refusal is not a dispatch, so no error row is written for it. A pack that declares no `K` or no expiry, or
a family-scoped row that does not fold, also refuses by name at that dispatch. Nothing is defaulted.

The family is the one D-7 names. It is the selected pack's `[role.<role>] family` for the step's role
(`Pack::require_family_for_role`), resolved in the walk's agent arm (`Runtime::step_family`) and in
`Runtime::run_agent_step`. An agent step with no role, or a role the pack binds to no family, is refused by name. The
binding is `Runtime::with_breaker(BreakerBinding::of(&pack))`. The production run path binds the selected pack in
`replay::resume_routed`. `Runtime::new` binds none: the kernel's machinery suites have no pack, and there a row
carries `at` with no `family`. The pre-R1 activation path (`Runtime::run_activity`, and `Runtime::step`'s
`NextAction::Dispatch`, which transcribe no step list) keys no family and consults no breaker. `rk-cli`'s production
run transcribes every node (ADR-0138).

**6. The journalled rows are AC-033's durable mark.** *"A durable breaker mark for the family is written"* is read as
follows: the K-th consecutive kernel-written `step_failed { class: platform, family, at }` row is written durably and
is family-scoped, and the fold derives the open mark from it (`OpenMark.opened_at` is that row's `at`). **ADR-0120's
route is not required, and no `EventKind` is minted.** `crates/rk-kernel/src/runtime.rs`'s table row *"AC-033's
breaker mark | none exists"* is corrected in the same change.

**7. A report that carries an outcome was not skipped.** `ActivityReport::skipped()` answers true only when nothing was
dispatched, nothing committed and no outcome was recorded. Without this, a dispatch the breaker refused (no port
reached, no commit) would read as AC-013's re-entry, and the walk would serve it as a committed step.

## Consequences

**Positive**

- Fake and real harnesses leave the same failure rows (D21-42 20). The failure class reaches the journal under
  `ClaudeHarness`, and a `NotAdopted` or `Unsupported` refusal is folded the same way under both.
- AC-033's refusal, AC-034's per-family isolation and AC-035's survival and expiry are measured **at the executor**,
  over rows the kernel wrote, across a real process death.
- AC-360's *"kernel test that drives every `StepError` variant … through the dispatch path and asserts one event
  each"* exists (`step_error_row::the_kernel_appends_exactly_one_error_row_per_failed_dispatch`).

**Negative**

- **The refusal has no class of its own.** It travels as `StepError::Unsupported`, so its exit code is that class's
  row (19). A breaker variant would move `rk-types` and ADR-0095's closed set, and this record does not take it.
- **A `step_refused` row now reaches the store under a real harness,** and the FSM fold halts on it
  (`HaltReason::Refused`). It did so under the fake before this record. Under `ClaudeHarness` it is new behaviour.
- **One instant per run.** Every row a run writes carries the run's single journalled instant, so a mark opened
  during a run is dated from that run's instant, and a family's failures within one run share one `at`.
- **The fold reads the whole journal before every agent dispatch attempt.** It is O(n) in the store's size. No
  incremental form is built.
- **The pre-R1 path is not keyed.** A dispatch without a step list writes its row with no `family` and is not
  refused. `rk-cli`'s in-process tests that bind `RecordedSpec::supplied` run there.
- `K` and the expiry still carry *"NO RECORD"* in the shipped pack. Their record is `s8h`'s (D21-42 19c).

**What this forbids**

- Forbids a harness appending a `step_failed`, `step_cancelled`, `step_refused` or `rate_limited` row.
- Forbids more than one error row per failed dispatch, and forbids a bare `rate_limited` failure row beside AC-030's
  mark.
- Forbids an error row, or the breaker's input, that a later process cannot read back from the store, and forbids
  breaker state held only in memory (AC-035).
- Forbids defaulting `K`, the expiry or a role's family, and forbids keying the breaker on the activity's `family`
  (ADR-0058's), which D-7 and `family_key_never_from_the_activity_catalog_rule` already forbid.
- Forbids minting an `EventKind` for the breaker mark except by ADR-0120's route.
- Forbids reading a breaker refusal as a served, committed or skipped step.

## Evidence and confidence

- [documented] D21-42 20, `s8d` §8d.5, AC-033..AC-035, AC-360, ADR-0095 decision 2, ADR-0112's table and ADR-0126's
  two sentences on the kernel's rows, each quoted above or in `source`.
- [measured 2026-09-26 at `s8d`'s build, through the no-dispatch helper] These tests were each first shown RED on a
  scaffold with the binding and accessors present and no kernel row, fake append kept and no call site, then GREEN:
  - `step_error_row::` (4 tests): one row per failed dispatch for all eleven variants plus the boundary's `Schema`,
    and two rows for two failed dispatches; `FakeHarness` writes none on any path, and one row through the runtime;
    one `rate_limited` row, the mark; `family` and `at` on the row, folded by the breaker, and `family` on the
    commit row;
  - `breaker::the_executor_refuses_a_dispatch_whose_family_the_kernels_rows_opened`;
  - `cross_process::ac035_breaker_refusal_survives_a_real_process_death`, rewritten: the child drives the kernel and
    the parent's executor refuses.

  `breaker::breaker_ac033_ac035` and `breaker::ac034_breaker_never_trips_globally` stay green. Three `rk-ports`
  `fake_behaviour::` tests and `rk-cli`'s `root::ac404_rate_limited_writes_retry_mark_and_exits` asserted the fake's
  own row. They take dated amendments and keep their names.
- [measured, same session] Mutant M10 deletes the kernel's append. It is killed by
  `step_error_row::the_kernel_appends_exactly_one_error_row_per_failed_dispatch` and by
  `breaker::the_executor_refuses_…` (the latter at its premise that the kernel wrote two rows). Mutant M11 keeps a
  fake append. It is killed by `step_error_row::the_fake_harness_writes_no_error_row_of_its_own`. Both were run from
  a Python driver, with each file restored and its sha256 verified.
- [measured] `git diff --quiet e8e4f68 -- crates/rk-types` → 0: `rk-types` did not move, so Checkpoint 5 does not
  re-run for this record.
- **What this evidence does not license.** Every dispatch here is a fake harness's. No real `Platform` failure has
  opened a breaker, and `K` and the expiry are sized by no corpus figure (AC-033's own *Note*). This record flips at
  `s8d`'s close under D21-64 ruling 10, on its own dated ledger line, only if the formal Checkpoint 8d passes whole
  and a refuting round finds no blocker or major.
