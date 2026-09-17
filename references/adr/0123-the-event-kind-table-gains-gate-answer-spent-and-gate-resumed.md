---
id: "0123"
title: The event-kind table gains gate_answer_spent and gate_resumed — the two tier-2 kinds the human-gate criteria name and nothing minted
status: accepted
date: 2026-09-17
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-246, AC-271, AC-267, AC-279, AC-524, AC-249, AC-270, **cited by id and named clause rather than by line** (the register moved +39 lines during the 2026-09-13 amendment pass; `runbook/phase-1/s7b-ii-gates.md`:63-70 and ADR-0119/0120's `source:` keys record the hazard and take the same route; AC-246's THEN reads at :4085 and AC-279's *Verified by* at :4347 on this tree, both `M` against `11b347e`); references/2026-09-08-success-measures.md SM-073 :554 and :605 (the two Instrument lines naming `gate_resumed`); references/2026-09-08-architecture-design.md §5 :699 (the closed-enum sentence) and :701-723 (the table — `gate_answer_void` at :709 is the shape this record's first kind mirrors); references/adr/0017 (the single-use token); references/adr/0116 :40-41 (*"the kind is minted by record or not at all"*) and :95-99 (the runbook option, rejected on mechanism); references/adr/0120 (the precedent this record follows clause for clause, including decision 6's measured note that the golden schema snapshot does NOT move); references/adr/0105 clause 7 (`rk-types` owns the enum) and clause 10 (the fresh mark on re-reach); references/adr/0109 decision 2 (the deleted-database test); references/adr/0095 decision 2 (the closed `RefusalClass`, which is why `step_refused` is not the carrier); crates/rk-types/src/event_kind.rs and crates/rk-types/tests/it/event_kinds.rs at `11b347e`; crates/rk-kernel/src/workflow/fsm.rs `gate_answered` arm at `11b347e` (the measured overwrite); runbook/phase-1/s7b-ii-gates.md §7b-ii.2 and §7b-ii.11 (`ac246_token_is_single_use`, `ac279_one_prefix_rewrite`); runbook/00-preamble.md :55; the operator's decision of 2026-09-17, taken in the Step 7b-ii session against the stage's stop on this question — there is no `spikes/DECISIONS-2026-09-17.md` and this record is that decision's primary record
supersedes: []
superseded_by: []
related: ["0017", "0087", "0095", "0105", "0109", "0116", "0120"]
evidence: ["F28", "F38", "F19"]
consumers: []
---

## Context and problem

**Two human-gate criteria name an event the closed table does not define, and a third names a field on it.**

- **AC-246**'s THEN: *"the second answer is refused as spent, **records a distinct event**, and changes no item
  state."* AC-271 restates the same refusal (*"the second is refused as spent (AC-246), and no second mark is minted"*).
- **AC-279**'s *Verified by*: *"a count assertion on prefix-rewrite events per **`gate_resumed`**, asserted ≤ 1."*
- **AC-267**'s *Verified by*: *"a metric assertion on the **`gate_resumed`** event's `cache_creation_input_tokens`"*;
  SM-073's two Instrument lines name the same event and the same field.

**Neither kind exists.** `git grep -n 'gate_answer_spent\|GateAnswerSpent'` returns **0 hits** on the whole tree;
`git grep -n 'gate_resumed\|GateResumed'` returns hits in the register (AC-267, AC-279), the success-measure register
(SM-073, twice), the audit dossier quoting SM-073, and `runbook/phase-1/s7b-ii-gates.md`'s own record of the gap —
and **none in `references/2026-09-08-architecture-design.md` §5's table and none in `crates/`** [measured 2026-09-17].
`crates/rk-types/src/event_kind.rs` carries **29** variants and `ALL: [Self; 29]`; `crates/rk-types/tests/it/event_kinds.rs`
pins that count [measured 2026-09-17]. Design §5:699 is the constraint: *"the `EventKind` enum is **closed**, a kind
not in this table is a schema change reviewed as one."*

**No existing kind can carry the refusal without deciding something else.** Two were considered and each fails on
a clause, not on taste:

- **`gate_answer_void`** is the nearest shape — tier 2, `token`, a drift observation — but its fold arm in
  `crates/rk-kernel/src/workflow/fsm.rs` **removes the token from `gates_answered`** and inserts it in `gates_void`.
  Recording a spent refusal under it would *un-answer* the gate: the first answer, which AC-246 says *applies*, would
  stop counting. That is a state change on exactly the path AC-246 says *"changes no item state"*. Void and spent are
  also different facts — void is AC-249's *tree hash no longer matches*; spent is ADR-0017's *single-use token
  presented twice* — and a `doctor` reading `gate_answer_void` counts as drift would count refusals as drift.
- **`step_refused { class }`** is ADR-0095 decision 2's refusal arm, and its `RefusalClass` is **closed** at
  `NotAdopted | NotIsolated | Unsupported | Overage` (`crates/rk-types/src/event.rs`:24-29). A `SpentToken` class is a
  supersession of ADR-0095 in part, for a refusal that is not a *step* refusal at all — no activity was dispatched.

**The behaviour AC-246 forbids is live today.** Measured on `11b347e` with a probe test over the fold: a journal
`gate_opened → gate_answered(approve) → gate_answered(changes-requested)` on **one** token folds with the recorded
verdict **`approve` → `changes-requested`** — the second answer silently rewrites the first, because the
`gate_answered` arm asks only *is the token in `gates_open`?* and the token stays there after it is answered
[measured 2026-09-17: `PROBE: the second answer on a spent token changed the recorded verdict: Some("approve") ->
Some("changes-requested")`]. Repairing that in Step 7b-ii's gate module is the stage's work; the event the repair
must record is this record's.

**The stage stopped rather than minting, and that was correct.** `00-preamble.md`:55 — *"Where a stage is tempted
to decide something else, it stops"* — and ADR-0116 forbids the shortcut by name: *"the kind is minted by record or
not at all"*. ADR-0120 is the precedent for exactly this situation (a criterion's THEN naming a kind no table
defines) and this record follows it clause for clause. The operator authorised the mint on 2026-09-17, in the
Step 7b-ii session, choosing it over shipping two of AC-246's three THEN clauses with the third ARMED.

## Decision drivers

- A criterion naming an event kind no table defines cannot be built, and the enum is closed by design — so the kind
  is minted by record or not at all (ADR-0116's driver, applied for the third time).
- ADR-0017's token is *single-use*, and single-use is not a property the system has unless the second use is
  **recorded as refused**. A refusal that leaves no row is indistinguishable, in the log AC-270 reconstructs from,
  from an answer that never arrived — and F38's second-arrival result is circular by construction (AC-246's own
  evidence note), so the log is the only instrument that will ever count arrivals (SM-078).
- AC-279 and AC-267 are the kernel's and the adapter's halves of one measurement — *at most one prefix rewrite, flat
  in duration* [measured, F28: 18,603 tokens at 65.2 min and at 49.7 h] — and both are assertions **on a named event**.
  Without the event there is no row to assert on, and the tempting regression design §5 names (*a keep-the-cache-warm
  dispatch*) has nothing to be caught by.
- A refusal folded as an **error** would make the journal unfoldable for ever (the append-only log; the D-5 lesson of
  2026-09-14). The refusal has to be a **row the fold reads and does not act on**, which is what a tier-2 kind is.

## Decision

**1. `gate_answer_spent` is a tier-2 event kind**, written **once per refused second answer**. Its row in design §5's
table:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `gate_answer_spent` | 2 (a refusal observation) | `token`, `tree_hash` (the refused answer's) | the answer path — `redkiln answer` (7d) through the kernel's gate module (7b-ii) — on an answer whose token is already in the answered set |

**The fold reads it and moves nothing.** The refusal itself is derived, not read: a second `gate_answered` on a token
already in `gates_answered` is refused by the fold **from the tier-1 record alone** — the first answer stands, no
member of the state changes, the row is counted — and `gate_answer_spent` is the *record* that the refusal happened,
appended by the answer path and folded as a no-op. It is **not** the mechanism; the mechanism is the answered set.

**2. `gate_resumed` is a tier-2 event kind**, written **once per resume** — the first kernel action on an item after
one of its gates reaches a terminal state (answered, void-then-re-opened-and-answered, expired-and-resolved). Its row:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `gate_resumed` | 2 | `token` (the gate resumed from), `prefix_rewrites` (a count), `cache_creation_input_tokens`, `total_cost_usd` | the kernel's runtime at the resume; the two metric fields are filled by the adapter that performed the resumed dispatch (7c) and are **absent, not zero**, when no adapter has |

**The count is the kernel's and the metrics are the adapter's, and the split is stated so neither half is claimed by
the other.** AC-279's `prefix_rewrites ≤ 1` is asserted in `rk-kernel` at Step 7b-ii over the kernel's own count.
AC-267's threshold over `cache_creation_input_tokens` *against the recorded baseline* is Step 7c's: there is no adapter
to fill the field and no baseline to assert against before then (`s7b-ii` §7b-ii.14's cross-listed block says so and
this record does not move it). **An absent metric field is absent** — it is never written as `0`, because a zero
would satisfy every threshold (golden constraint 10's shape).

**3. Both are tier 2, and the deleted-database test is why** (ADR-0109 decision 2: *if the database were deleted,
would `redkiln run` take a different next action?*). For `gate_answer_spent`: **no** — the refusal is derived from
`gates_answered`, which is folded from tier-1 `gate_answered` rows that git carries; losing the spent row loses the
count of refusals, not the refusal. For `gate_resumed`: **no** — it is telemetry about a dispatch that has already
happened; the next action is a function of the gate's terminal state, which is tier 1. Neither carries a trailer and
no fourth trailer axis is minted (ADR-0105 clause 3's three axes are closed).

**4. The table rows and the `rk-types` variants land in the same change** (ADR-0116 clause 5; ADR-0120 decision 6).
What moves: design §5's table gains two rows; `EventKind` gains `GateAnswerSpent` and `GateResumed`; `ALL` goes
`[Self; 29]` → `[Self; 31]`; the exhaustive `wire_name` and `tier` matches gain arms (the `E0004` in `tier()` is
what forces decision 3 to be answered by whoever adds a kind); `crates/rk-types/tests/it/event_kinds.rs`'s `29` → `31`
with a pin per kind. **The golden schema snapshot does not move**: `schema_snapshot.rs` snapshots `registry::export_all()`
and `CompanionKind::ALL`, and `EventKind` is in neither [re-measured 2026-09-17, unchanged from ADR-0120's measurement].

**5. The consequence for a closed checkpoint is taken, not routed around.** `crates/rk-types` is Checkpoint 5's and
that checkpoint is recorded `passed 96ba9c2`. Under `00-preamble.md`'s maintenance rule and `s7b-ii` §7b-ii.12, the
commit that lands this record's variants marks `PROGRESS.md` §1's Step 5 row **`passed 96ba9c2, re-run pending 7b-ii
rk-types`** in the same commit, and Checkpoint 5's hardened block is re-run before the 7b-ii slice that depends on it
closes.

**6. What this record does NOT decide**, named rather than left to be discovered:

- **AC-271's cross-process arrival semantics** beyond *"the second is refused as spent"* — which of two answers in
  flight from two processes is *first* is `rk-store`'s single-writer serialisation (ADR-0105 clause 7), and the
  register itself says the semantics are *declared, not derived* (PROVISIONAL, SM-078 unmeasured). This record fixes
  what the loser's row is called. It does not fix who loses.
- **Whether a spent refusal re-opens anything.** It does not: the gate is answered, and a refused duplicate is not
  drift. AC-252's re-open is the *void* path's and is untouched.
- **The threshold on `cache_creation_input_tokens`** (AC-267, SM-073) — Step 7c's, against a baseline that does not
  exist yet.
- **The Rust types of the payload members.** As ADR-0120 decision 1: the names and their obligation are fixed; the
  types are the implementation's.

## Considered options

### Option — record the spent refusal as `gate_answer_void` with a `reason: spent` member

The nearest existing shape, and rejected on its fold arm rather than on its name: `gate_answer_void`'s arm removes
the token from `gates_answered`, so recording a refusal under it un-answers the gate the first answer answered — a
state change on the one path AC-246 says changes no state. Fixing that by making the arm read `reason` first would
give one kind two fold semantics selected by a payload string, which is a stringly-typed variant with extra steps and
the shape `RefusalClass`'s own doc comment (*"no class is stringly-typed"*) exists to forbid.

### Option — a `SpentToken` arm on `RefusalClass`, carried by `step_refused`

Rejected: `RefusalClass` is closed by ADR-0095 decision 2 and adding a class supersedes that record in part, for a
refusal that is not a step refusal — nothing was dispatched, no capability was asked for, no `StepError` exists to
carry it. The register's own vocabulary is *"records a distinct event"*, and `step_refused` is not distinct from the
refusal of a dispatch.

### Option — build the refusal and the no-state-change, leave the event ARMED

What the stage would have done had the operator not decided: ship two of AC-246's three THEN clauses and carry the
third as *"a record is OWED"*. **Rejected as a terminal state, not as a stage act** (ADR-0120's own words): the stop
was correct, and the blocker was one record rather than one measurement. The operator chose the record.

### Option — mint the kinds in the runbook, or as a `const` in `rk-kernel`

Rejected on mechanism and by name: `00-preamble.md`:55, ADR-0116:95-99, ADR-0120's same rejected option. A kind
invented at Step 7b-ii would be the *"a rule with no ADR is a rule the next refactor removes"* failure ADR-0107's
drivers name, on the enum ADR-0105 clause 7 closes.

### Option — one kind, `gate_event { kind: spent | resumed }`

Rejected: it moves the closed set from the enum into a payload string, which is the design the closed enum replaced,
and it would give the two kinds — a refusal on the answer path and telemetry on the dispatch path — one tier column
and one producing-step cell when they have neither in common.

## Consequences

**Positive**
- AC-246 becomes implementable exactly as the register words it — all three THEN clauses — and AC-271's refusal has
  a row a concurrency test can count.
- AC-279's `≤ 1` and AC-267's threshold have the event both name, and design §5's *keep-the-cache-warm* regression
  has a row that would show it.
- AC-270's *"every gate open, answer, void, expiry, escalation and re-open"* reconstruction can tell a refused
  duplicate from an answer that never arrived, which is the only way SM-078's arrival rate will ever be measured.
- The live overwrite measured above becomes a criterion's red rather than an unrecorded hazard.

**Negative**
- A closed enum moves for the third time in five days, by two. The hand-written completeness pin moves with it; every
  count of the set — `[Self; 29]`, the test's `29`, design §5's row count — moves.
- **Checkpoint 5 becomes provisional** and is re-run (decision 5). That cost is the maintenance rule working, not a
  side effect to be minimised away.
- `gate_resumed` carries two fields nothing can fill until Step 7c. Absent-not-zero is stated (decision 2) so the
  gap is visible in every row until then.

**What this forbids**
- Forbids a second answer on a spent token that changes any member of the item's state, in either direction — the
  first answer stands and the second is refused.
- Forbids folding a spent second answer as an **error**: the journal is append-only and an error there is a run that
  can never be folded again.
- Forbids recording a spent refusal as `gate_answer_void`, or a void as `gate_answer_spent` — they are two facts with
  two fold effects.
- Forbids writing `prefix_rewrites`, `cache_creation_input_tokens` or `total_cost_usd` as `0` when no measurement was
  taken; an absent measurement is an absent field.
- Forbids asserting AC-267's threshold, or claiming AC-267, before an adapter fills the field against a recorded
  baseline (Step 7c).
- Forbids landing either variant in `rk-types` without the design §5 rows, the `ALL` count, the pin test and the
  Checkpoint 5 provisional row in the same commit.
- Forbids a trailer, a trailer axis or tier 1 for either kind.

## Evidence and confidence

- [measured 2026-09-17, `11b347e`] `git grep -n 'gate_answer_spent\|GateAnswerSpent'` → **0**;
  `git grep -n 'gate_resumed\|GateResumed'` → register :4333 (AC-267), :4347 (AC-279); SM :554, :605; the audit
  dossier's two quotations of SM-073; `runbook/phase-1/s7b-ii-gates.md` :487, :826, :829, :832 — **nothing in design
  §5's table, nothing in `crates/`**.
- [measured 2026-09-17] `crates/rk-types/src/event_kind.rs` carries 29 variants, `ALL: [Self; 29]`; `event_kinds.rs`
  asserts 29. `schema_snapshot.rs` snapshots `export_all()` and `CompanionKind::ALL` only.
- [measured 2026-09-17] the probe over `fold` at `11b347e`: one token, `gate_opened → gate_answered(approve) →
  gate_answered(changes-requested)` → recorded verdict `Some("approve") -> Some("changes-requested")`. The second
  answer is applied, not refused; no row records that it arrived.
- [documented] AC-246's THEN and its evidence note (*"F38's second-arrival result is not evidence here … P8 §5.4(3) is
  circular by construction"*); AC-271's PROVISIONAL note (*"the arrival rate is entirely unmeasured, see SM-078"*).
- [documented] AC-279's *Verified by* and AC-267's; SM-073's Instrument lines.
- [measured, F28 as corrected] 18,603 `cache_creation_input_tokens` at 65.2 min and at 49.7 h under the default
  prompt — one point per duration, n=1 each; the invariance is measured for the default prompt only and *assumed, not
  measured* for the card prompt (AC-267's own PROVISIONAL note, carried unchanged).
- [measured, F19] drift on 12/174 gates — the reason the *void* path is common and must stay distinct from *spent*.
- [documented] ADR-0116:40-41, :95-99 and ADR-0120's six decisions — the rule and the precedent.
- **Confidence:** the gap is measured and unambiguous, the overwrite is measured, and the payload names are the
  register's. The **tier** of both kinds is *reasoned* from ADR-0109's deleted-database test and from
  `gate_answer_void`'s tier in the same table, and is **not measured**; no spike bears on it. The **producing step**
  of `gate_resumed` is reasoned from AC-279's *"WHEN the item resumes"*. Both are stated here rather than inherited,
  which is the same confidence ADR-0116 and ADR-0120 recorded for the same two columns.
