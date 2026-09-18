---
id: "0126"
title: The event-kind table gains `dispatch_completed` — the usage-bearing row AC-360 requires on every termination path, appended by the Harness adapter through the store it is handed; `DispatchRecord` is completed in `rk-types` and the kernel's pre-dispatch `dispatch` row stands
status: proposed
date: 2026-09-17
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-360 (THEN *"exactly one dispatch event is appended carrying every token counter the harness reports verbatim, `total_cost_usd`, the `five_hour` and `seven_day` headroom readings taken before and after, and the pinned harness/CLI version AND no dispatch path exists that can terminate without appending that event"*), AC-362 (*"an `Option` with no `Default` impl and no `unwrap_or(0)` at any call site"*), AC-227 (*"it carries the CLI version the dispatch actually ran against"*; *"a schema requirement (non-optional field)"*), AC-385 (*"the observed CLI version is recorded on the event"*), AC-232 (*"each is recorded and counted against the activity's attempt budget"*), AC-225 (`Cancelled { by }` — signal, timeout or operator), AC-523 (`detected_by` — subtype or text_marker), AC-374/AC-375 (ticks; exactly two window fields), AC-368 (the tier-2 append-failure rule this record does NOT reconcile), **cited by id and named clause** (the register's line numbers moved +39 on 2026-09-13; AC-360 reads at :5613-5627 on this tree, `6152219`; DECISIONS-2026-09-15 D15-2); references/2026-09-08-architecture-design.md §5 :699 (the closed-enum sentence) and its table's row *"dispatch record (cost row) | 2 | argv digest, env digest, instrument version, usage, timings, `total_cost_usd`, stdout/stderr verbatim up to a size cap then digest + head/tail, the child environment **never** | `Harness::dispatch` (AC-360)"* — a row with a tier, a payload and a producer and **no wire name**; references/adr/0099 decision 1 (*"The `record` is the AC-360 dispatch record (argv digest, env digest, instrument version, usage, timings, raw-retention per ADR-0105)"* and *"`DispatchRecord` carries `StepRecord`'s. Both are `rk-types`."*); references/adr/0113 decision 1 (*"The adapter records the tier split on every `step_*` event that carries usage"*); references/adr/0122 decision 6 (*"this record mints no kind and asserts none exists"* for the inner attempt) and decision 1 (one charge per recorded `system/api_retry` event); references/adr/0110 (ticks; the two windows); references/adr/0105 clause 7 (`rk-types` owns the enum; argv and environment as digests only); references/adr/0109 decision 2 (the deleted-database test); references/adr/0116 :40-41 and :95-99 (the minting rule); references/adr/0120, 0123, 0124, 0125 (the four precedents this record follows clause for clause); crates/rk-types/src/event_kind.rs and crates/rk-types/tests/it/event_kinds.rs at `6152219` (37 variants); crates/rk-types/src/activity.rs at `6152219` (`DispatchRecord` at five fields, its module doc saying the set is *"incomplete by the record"* and *"owed at 7c"*); crates/rk-kernel/src/runtime/activity.rs at `6152219` (`dispatch_event` appended before the call; `outcome.record` dropped after it); crates/rk-store/src/retention.rs at `6152219` (the second `DispatchRecord`); runbook/phase-1/s7a-ports-and-fakes.md §7a.5 (the typed-payload record OWED); runbook/00-preamble.md — *"Where a stage is tempted to decide something else, it stops"* (the closing sentence of its precedence paragraph; cited by clause, since the line moved 55 → 64 → 81 across the tree's own history); the operator's decision of 2026-09-17, taken before Step 7c's stage-open commit with the alternatives named (a second `dispatch` row; fields on the `step_*` rows; a kind minted by record — the third was taken), recorded as `spikes/DECISIONS-2026-09-17.md` D17-8
supersedes: []
superseded_by: []
related: ["0003", "0099", "0105", "0109", "0110", "0113", "0116", "0120", "0122", "0123", "0124", "0125"]
evidence: ["F1", "F3", "F6"]
consumers: []
---

## Context and problem

**AC-360 names one row that carries everything a dispatch cost, and no kind carries it.** The criterion's THEN:
*"exactly one dispatch event is appended carrying every token counter the harness reports verbatim,
`total_cost_usd`, the `five_hour` and `seven_day` headroom readings taken before and after, and the pinned
harness/CLI version AND no dispatch path exists that can terminate without appending that event."* Its *Verified
by* says which record the event carries: *"The `DispatchRecord` half of ADR-0099's `DispatchOutcome` (argv digest,
env digest, instrument version, usage, timings, raw retention per ADR-0105 clause 7 — argv and environment as digests
only, the child environment never) is what this event carries."*

**Design §5's table has the row and not the name.** Its cost row reads *"dispatch record (cost row) | 2 | argv
digest, env digest, instrument version, usage, timings, `total_cost_usd`, stdout/stderr verbatim up to a size cap
then digest + head/tail, the child environment never | `Harness::dispatch` (AC-360)"* — every other row in that
table is a wire name in backticks; this one is a description. `git grep -n 'dispatch_completed\|DispatchCompleted'`
over `references/`, `runbook/`, `crates/` and `spikes/` returns **0** [measured 2026-09-17, `6152219`]. `EventKind`
carries **37** variants (`ALL: [Self; 37]`) [measured]. Design §5:699 is the constraint: *"the `EventKind` enum is
closed, a kind not in this table is a schema change reviewed as one."*

**The built kernel writes a `dispatch` row and then drops the record.** `crates/rk-kernel/src/runtime/activity.rs`
appends `dispatch_event(&key, step_id)` — payload `activity_id_at_version, attempt, item, step_id`, no usage, no
version — **before** calling `harness.dispatch(spec, &mut *guard)`, and its `DispatchAttempt` keeps
`outcome.result` and **discards `outcome.record`** [measured, `6152219`]. So today no code path persists a
`DispatchRecord` at all, and the record itself is five fields (`argv_digest, env_digest, instrument_version, stdout,
stderr`) whose module doc says in terms: *"ADR-0099:58 / AC-360 also require usage and timings, and the `Usage`
type exists in no crate — so no field carries them here, and completing the field set is owed at 7c."* `grep -rn
'struct Usage\|struct CacheCreation' crates/` → **0** [measured]. A second `DispatchRecord` lives in
`rk-store::retention` with the same five fields, which `s7a` §7a.5 named as the collision 7c collapses.

**No existing kind carries the row without deciding something else.** The `step_failed { class }` /
`step_cancelled` / `step_refused { class }` / `rate_limited` arms are the *failure* rows of ADR-0095's table, keyed
by `StepError`; the success path has no `step_*` kind at all (there is no `step_succeeded`), so *"every termination
path"* cannot be spelled over them. The kernel's `dispatch` row is written before the child exists and carries the
idempotency key; putting usage on a second row of the same kind would give one kind two producers, two moments and
two payload shapes, distinguished by a payload member — the stringly-typed variant with extra steps that
`RefusalClass`'s own doc comment forbids and ADR-0123's rejected option names. `rate_limit_reading` is one reading,
not a before/after pair with a cost. `gate_resumed` (ADR-0123) carries two of the metric fields, for one dispatch in
one situation, and its own record says the adapter fills them *at 7c* from a measurement it has to have taken
somewhere — which is this row.

**Two more criteria are blocked on the same absence.** AC-232's *"each is recorded and counted"* for
`system/api_retry` has no channel: ADR-0122 decision 6 declines to mint a per-attempt kind (*"this record mints no
kind and asserts none exists"*) and `DispatchRecord` has no member for the count. AC-362's *"an `Option` with no
`Default` impl"* is ARMED-vacuous since Step 5 because the `Usage` type it constrains does not exist.

**The stage stopped rather than minting, and that was correct.** `00-preamble.md`'s closing sentence of its precedence paragraph — *"Where a stage is tempted
to decide something else, it stops"* — and ADR-0116 forbids the shortcut by name: *"the kind is minted by record or
not at all"*. ADR-0120, ADR-0123, ADR-0124 and ADR-0125 are the four precedents for exactly this situation (a
criterion's THEN naming an event no table defines), and this record follows them clause for clause. The operator
was shown the alternatives before the stage-open commit and chose the record.

## Decision drivers

- A criterion naming an event no table defines cannot be built as worded (ADR-0116's driver, sixth application).
- AC-360's *"no dispatch path exists that can terminate without appending that event"* is a property of the
  **adapter's** code, not the kernel's: ADR-0099 decision 1 hands the adapter `&mut dyn EventStore` for exactly this,
  and the kernel drops the record it gets back. A row appended by the kernel *after* the call would miss every path
  where the call does not return normally (a panic across the boundary, a process kill) — F22's survivor is the shape.
- ADR-0113 decision 1 puts the TTL split *"on every `step_*` event that carries usage"*; a dispatch that succeeds
  carries usage on no `step_*` event, because none exists for success. The usage-bearing event has to be one kind
  written on every outcome, or the five TTL states are recorded only for failures.
- ADR-0122 decision 1 charges *one per recorded `system/api_retry` event* and decision 6 mints no kind for the
  attempt; the count therefore travels on the one row every dispatch writes, or it travels nowhere.
- AC-227 and AC-385 want the CLI version on *every harness event* as a non-optional field; AC-385's mismatch
  warning attaches to *every cache metric taken in that run* (and AC-226's configured mismatch outcome fires on the
  same reading). One row per dispatch carrying both is how a reader
  finds the version beside the metric it qualifies.
- AC-362 / PIA-062: absent is `None`, never `Some(0)`, on a type with no `Default` — a rule that binds only once
  the type exists in a crate the checkpoint can grep.

## Decision

**1. One tier-2 event kind**, with its row in design §5's table replacing the un-named cost row:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `dispatch_completed` | 2 | the completed `rk_types::DispatchRecord`: `argv_digest`, `env_digest`, `instrument_version` (**the pinned harness/CLI version** AC-360's THEN names — the `Config` pin), `stdout`, `stderr` (`Retained`), `usage: Option<Usage>`, `timings`, `total_cost_usd: Option<..>`, `headroom_before: Option<WindowReading>`, `headroom_after: Option<WindowReading>`, `cli_version` (the **observed** version — below), `api_retry_events: u32`, the two TTL values `CLAUDE_CODE_PROMPT_CACHE_TTL` / `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` as set on the child (AC-210 THEN: *"both values are recorded on the dispatch event"*); and, on the paths where they apply, the cancel cause (`by: signal \| timeout \| operator`, AC-225) and `detected_by` (`subtype \| text_marker`, AC-523) | the Harness adapter, once per dispatch, on **every** termination path — `Ok(Typed)` and each `StepError` variant the adapter can produce, the pre-spawn refusals included — appended through the `&mut dyn EventStore` `dispatch` receives (ADR-0099 decision 1) |

`Usage` is *"every token counter the harness reports verbatim"* (AC-360): the counters as the harness named them,
each an `Option`, with the nested `cache_creation { ephemeral_1h_input_tokens, ephemeral_5m_input_tokens }` object
as `CacheCreation` (ADR-0113). The five-state TTL classification ADR-0113 decision 1 requires on every usage-bearing
event is derived from this row's `usage`; whether it is also materialised as a member is the typed-payload record's
(decision 6). `headroom_before` / `headroom_after` are readings of **exactly two windows** (`five_hour`,
`seven_day`; AC-375) in **u16 ticks parsed from the decimal digits** (AC-374, ADR-0110) and never an `f64`; the
row carries readings and no delta (AC-373). `cli_version` is **not optional** (AC-227) and is a two-armed value
whose provenance is distinguishable: **`Observed(version)`** — the version the child's own `system/init` reported
(AC-385) — on every path that read a `system/init`; **`Pinned(version)`** — the `Config` pin, `instrument_version`'s
value — on every path that terminated before one was read: the throttle's `RateLimited` refusal (AC-376), the
`Overage` refusal (AC-221), a spawn failure, a kill before init. AC-385's mismatch warning attaches only to
`Observed`; a pre-spawn row that wrote the pin as `Observed` would make an unverified claim look verified (Halt 11's
shape) and is forbidden below. `api_retry_events` is the count of `system/api_retry` events observed in
this dispatch's stream (ADR-0122 decision 1's unit); it is the count and not a list, because the attempt has no kind
(ADR-0122 decision 6) and this record mints none for it. **The payload's members are the register's and ADR-0099
decision 1's; this record fixes the names and the obligation, not the Rust types** (ADR-0120 decision 1's posture),
with two exceptions it does fix because a criterion fixes them: `usage` and every usage-adjacent measurement are
`Option` with **no `Default`** (AC-362; PIA-062), and the window readings are u16 ticks (AC-374).

**2. `Usage` and `CacheCreation` are `rk-types` types**, beside `DispatchRecord`, and neither derives or implements
`Default`. ADR-0099 decision 1 says of `ActivitySpec` and `DispatchRecord` *"Both are `rk-types`"* and says the
record carries *"usage, timings"*; a field of an `rk-types` struct has an `rk-types` type, because `rk-types`
depends on nothing internal [measured: its `[dependencies]` are `serde`, `serde_json`, `schemars`, `sha2`]. This is
the reading `s7a` §7a.5 called *"routed, not decided"*; it is decided here. Only `Margins` — a pack-configuration
type the adapter reads and the record never carries — stays adapter-side. `rk-store::retention::DispatchRecord`
collapses onto the `rk-types` type in the same change; one type, one declaration.

**3. The kernel's pre-dispatch `dispatch` row stands, and two rows per dispatch is the honest count.** AC-360 says
*"exactly one dispatch event is appended carrying every token counter …"* — the row it counts is the one carrying
the counters, and there is exactly one of those. The kernel's `dispatch` row carries the idempotency key and no
counter; it is written before the child exists, by the kernel, for AC-012/AC-013's commit protocol, and it is the
row a replay reads to know a dispatch was *attempted* when the adapter never returned. Removing it to make the
count read *one* would trade a replayable attempt record for a word. The two rows are two kinds with two
producers and two moments; nothing counts them as one, and nothing may.

**4. Tier 2, and the deleted-database test is why** (ADR-0109 decision 2; design §5's own row already says 2). The
next action after a dispatch is a function of the tier-1 commit state (`activity_committed`, the stage trailer);
losing this row loses the cost, the usage, the headroom pair and the version — the telemetry AC-361's rollup,
AC-372's batch delta and `redkiln cost` read — and never the decision. It carries no trailer; no fourth trailer axis
is minted.

**5. The table row and the `rk-types` variant land in the same change** (ADR-0116 clause 5; ADR-0120 decision 6;
ADR-0123 decision 4; ADR-0124 decision 4; ADR-0125 decision 5). `EventKind` gains one variant; `ALL` goes
`[Self; 37]` → `[Self; 38]`; `wire_name` and `tier` gain arms; the pin test gains a case; the kernel's exhaustive
matches (`fsm.rs`, `breaker.rs`, `memo.rs`) route it to their no-state-change arms — the kernel folds it and moves
nothing. `DispatchRecord` gains its members and `Usage` / `CacheCreation` are declared in `rk-types` in the same
commit, so the row and the record it carries are never on the tree one without the other. **Checkpoint 5 becomes
provisional and is re-run in that commit**: `PROGRESS.md` §1's Step 5 row goes `passed <hash>, re-run pending 7c
rk-types` in the same commit as the edit (`00-preamble.md`'s maintenance rule; the rule every 7b-ii record applied),
and the hardened block is re-run before the 7c slice that depends on it closes. The golden schema snapshot is
expected not to move (`EventKind` is in neither snapshotted set at every prior measurement) and is re-measured at
the commit, not assumed.

**6. What this record does NOT decide**, named rather than left to be discovered:

- **The typed per-kind payload record** (`s7a` §7a.5: *"The typed-payload record, per kind, off a design §5 table
  that must first be given types, is OWED"*). This row is the first with a fully typed payload on the producer's
  side; the record that gives every kind its type is still owed and its earliest real consumer is still
  `rk-adapter-sqlite` at 7c.
- **AC-368 against AC-360.** AC-368 says a tier-2 append failure *"is surfaced as a warning with a counter"* and
  *"no tier-2 append failure is ever mapped to a `StepError` that fails the activity"*; AC-360 says no path may
  terminate *without appending*. When the `dispatch_completed` append itself fails, both cannot hold as worded.
  This record makes the row tier 2 and leaves the reconciliation to a dated register amendment; it is raised in
  `spikes/DECISIONS-2026-09-17.md`.
- **Which `StepError` variant an expired per-activity timeout produces** (`Cancelled { by: timeout }` under
  ADR-0099 decision 2's token, or `Timeout { after_secs }` under ADR-0095's table). The row carries the cause
  either way; the variant is routed at 7c and raised, not settled here.
- **The `gate_resumed` fill path.** ADR-0123 decision 2 says the adapter fills `cache_creation_input_tokens` and
  `total_cost_usd` on `gate_resumed`; the kernel writes `gate_resumed` *before* the resumed dispatch. This row is
  where the measurement exists; how it reaches `gate_resumed` (a kernel change, or a reader joining the two rows)
  is not decided here.
- **AC-227's *"any harness event"* against the rows this record does not write.** AC-227 wants the version on
  *every* harness event; this row carries it, non-optionally, on both arms. The kernel's pre-dispatch `dispatch` row
  (decision 3) carries no version and never can — it is written before the child exists — and the kernel's
  `rate_limited` / `step_*` rows are the kernel's `StepError::event()` arm's, not the adapter's. Whether AC-227's
  *"any"* is read as *any adapter-written row* or takes a dated register amendment naming the kernel rows is raised
  in `spikes/DECISIONS-2026-09-17.md` and `runbook/phase-1/s7c-adapters.md` §7c.22, not decided here. Likewise
  AC-376's *"the refusal is appended as a `rate_limited` event"*: this record obliges the adapter to append
  `dispatch_completed` on that path; the `rate_limited` kind stays the kernel's one producer, and a register
  amendment naming that split is owed there.
- **ADR-0113 decision 1's location clause, read and not withdrawn.** ADR-0113 says the adapter records the tier
  split *"on every `step_*` event that carries usage"*. After this record **no `step_*` event carries usage** — the
  `step_failed` / `step_cancelled` / `step_refused` / `rate_limited` rows are ADR-0095's failure rows keyed by
  `StepError` and carry the class, not the counters; usage lives on this row alone — so 0113's clause holds
  vacuously over the `step_*` rows and its substance (the five states, the nested-object surface, every forbids) is
  satisfied on the one usage-bearing row. No clause of ADR-0113 is withdrawn, `supersedes: []` stands, and if a
  later record puts usage back on a `step_*` row, 0113's clause binds it as written.
- **Any `doctor` row, and `redkiln cost`'s rendering** (7d).

## Considered options

### Option — a second row of the existing `dispatch` kind, after the call, carrying the usage
The nearest existing shape, and rejected on its consequence rather than its name: one kind with two producers
(kernel before, adapter after), two moments and two payload shapes, told apart by a payload member. Every count
over `dispatch` rows would count attempts and completions together, the fold would need a payload read to know
which arm it is in, and the closed set would have moved from the enum into a string — the shape the closed enum
replaced (ADR-0123's rejected option, one kind over). AC-360's *"exactly one dispatch event … carrying every token
counter"* is not helped by it: the pre-dispatch row still carries none.

### Option — put the usage fields on the `step_*` failure rows and leave success to the kernel's `activity_committed`
Rejected: the success path is where nearly every token is spent, and `activity_committed` is the kernel's tier-2
row about a *result digest* (ADR-0022, ADR-0097), written by code that never sees the child's stream. It would
make the usage-bearing row a different kind on success and on failure, so ADR-0113's TTL split and AC-227's
version would live under two schemas, and AC-360's *"every termination path"* would be four kinds plus one
kernel row, asserted separately. Rejected by the operator on those terms.

### Option — mint a per-attempt kind for `system/api_retry` as well
Not taken: ADR-0122 decision 6 declines it in terms and AC-232's *"recorded and counted"* is satisfied by a count on
the row every dispatch writes. A per-attempt row can be minted by a later record if AC-391's per-attempt replay is
ever discharged rather than ARMED; nothing here forecloses it.

### Option — mint in the runbook or as a kernel `const`
Rejected on mechanism and by name (`00-preamble.md`'s *"Where a stage is tempted to decide something else, it stops"*; ADR-0116:95-99; ADR-0120, ADR-0123, ADR-0124, ADR-0125).

## Consequences

**Positive**
- AC-360 becomes implementable exactly as worded, and its *Verified by* — every `StepError` variant driven through
  the dispatch path with one event each, the `EventStore` fake as the surface — has a row to count. The exhaustive
  match over `StepError` that fails to compile without an arm is the adapter's, where the record is constructed.
- AC-362 stops being vacuous: `Usage` exists, in a crate Checkpoint 5's derive check can reach, with no `Default`.
- AC-232, AC-227, AC-385 and ADR-0113's five TTL states have one carrier, and `gate_resumed`'s two absent fields
  have a source.
- `s7a` §7a.5's `DispatchRecord` collision closes: one type, in the crate ADR-0099 assigns it to.

**Negative**
- A closed enum moves for the sixth time in six days, by one. The hand-written pins move; every count that says
  *"closed at 37"* is stale again and is corrected in place, dated, in the same commit.
- Checkpoint 5 becomes provisional and is re-run (decision 5). That is the maintenance rule working, not a cost
  to route around.
- Two rows per dispatch is what a reader will find, and this record says so rather than renaming one to make the
  count read *one*. A `doctor` row that counts `dispatch` against `dispatch_completed` is the natural instrument
  for the missing-completion case and is 7d's.

**What this forbids**
- Forbids a Harness adapter path — `Ok`, any `StepError`, a cancel, a timeout, a refusal before spawn — that
  returns from `dispatch` without appending a `dispatch_completed` row through the store it was handed.
- Forbids the kernel appending `dispatch_completed`; it is the adapter's row, from the code that has the record.
- Forbids a `dispatch_completed` row with `cli_version` absent, or with any usage-adjacent absence written as `0`;
  forbids a row on a path that read no `system/init` carrying `cli_version` as `Observed`.
- Forbids `Default` on `Usage`, `CacheCreation` or any usage-adjacent reading type, and `unwrap_or(0)` /
  `unwrap_or_default()` on their fields at any call site (AC-362).
- Forbids a headroom member that is an `f64`, that names a model family, or that carries more or fewer than two
  windows (AC-374, AC-375); forbids a per-dispatch delta member (AC-373).
- Forbids folding the row as anything but a no-state-change arm, and forbids a trailer, a trailer axis or tier 1.
- Forbids removing or renaming the kernel's pre-dispatch `dispatch` row on the strength of AC-360's *"exactly one"*.
- Forbids a second `DispatchRecord` type anywhere in `crates/` once this row's type lands.
- Forbids landing the variant without the design §5 row, the `ALL` count, the pin test, the `rk-types` members and
  the Checkpoint 5 provisional row in one commit.

## Evidence and confidence

- [measured 2026-09-17, `6152219`] the grep above: `dispatch_completed` / `DispatchCompleted` **0** hits across
  `references/`, `runbook/`, `crates/`, `spikes/`; `struct Usage` / `struct CacheCreation` **0** under `crates/`;
  `event_kind.rs` at 37 variants; `DispatchRecord` at five fields in `rk-types` and again in `rk-store`;
  `runtime/activity.rs` appends `dispatch_event` before the call and constructs `DispatchAttempt` from
  `outcome.result` alone.
- [documented] AC-360's THEN and *Verified by*; AC-362's THEN; AC-227's *Verified by*; AC-385's THEN; AC-232's THEN;
  ADR-0099 decision 1; ADR-0113 decision 1; ADR-0122 decisions 1 and 6; design §5's cost row.
- [measured, F1] headroom is readable only from the top-level `rate_limit_event`, in two windows, with no family
  key — the reason the row carries exactly two readings and no family member. [measured, F3 as re-read by
  ADR-0113] the nested `cache_creation` object is present on 950/955 recorded runs and is the TTL surface the row
  carries verbatim. [measured, F6] `utilization` is quantised to 0.01 — the reason the reading is u16 ticks.
- **Confidence:** the gap is measured; the members are the register's and ADR-0099's; the name is this record's
  (`dispatch_completed` follows the table's `<subject>_<verb>` spelling and pairs with the kernel's `dispatch`). The
  tier is design §5's own and is *reasoned* from ADR-0109's test, as the four precedents reasoned theirs; the
  two-rows count is measured from the kernel as built. The choice of one kind over the two alternatives is the
  operator's, taken on the grounds above and recorded as D17-8.
