---
id: "0158"
title: "**The tier-1 store layer has NO WRITER on any axis, and that is recorded rather than closed here.** `0001_initial.sql` declares `t1_stage_transition`, `t1_gate` and `t1_slice_seal` under the header *\"tier 1: git first — these rows never exist without their trailer commit … which is why every one is anchored to a commit sha\"*, each with an `anchor_sha` column. Measured 2026-09-22: **zero `INSERT INTO t1_*` anywhere in the workspace**, all three tables EMPTY in a live store after real runs, and the `EventStore` port carries only `append`/`query`/`snapshot`/`rebuild` — `append` writes `t2_event`. So AC-160..AC-163's *\"a failed git write leaves ZERO of them\"* and AC-162's *\"the trailer commit exists before any store row for that transition is readable\"* are vacuous on every axis, **`Gate:` included, whose trailer producer HAS existed since 7d**. Adding a tier-1 write path is a change to a named port's surface (ADR-0003's five) and is not `s7h`'s to take; the residual is bounded and routed instead"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-store/src/migrations/0001_initial.sql (lines 15-44 — the tier-1 header and the three tables, each with `anchor_sha TEXT NOT NULL -- repo-key`); crates/rk-ports/src/event_store.rs (the whole port surface — `append` at 52, `query` at 55, `snapshot` at 58, `rebuild` at 61); a live `.git/redkiln/store.sqlite3` after real `redkiln run` invocations (`t1_stage_transition` 0, `t1_gate` 0, `t1_slice_seal` 0, `t2_event` 30); crates/rk-cli/src/verbs/run.rs (`terminal_read`'s `Gate:` trailer commit — the one existing tier-1 trailer producer, with no store row beside it); references/2026-09-08-acceptance-criteria.md AC-160..AC-163; references/adr/0003 (the five ports); references/adr/0105 (clause 7's event-kind table); references/adr/0109 (the two-tier split)
supersedes: []
superseded_by: ["0181"]
related: ["0003", "0105", "0109", "0155", "0156", "0157"]
evidence: []
consumers: []
---

## Context and problem

§7h.2 exists to give two trailer axes their producers, and AC-162 states the ordering both halves must
satisfy:

> GIVEN a tier-1 transition (stage change, gate opened/answered, verdict, slice seal) WHEN the
> transition is applied THEN the trailer commit exists in git **before any store row for that
> transition is readable** AND if the git write fails the store contains no row for that transition

Working out where to put the `Story:` write meant finding out what *"the store row for that
transition"* is. It is a named table. `crates/rk-store/src/migrations/0001_initial.sql:15-18`:

```sql
-- ---------------------------------------------------------------- tier 1: git first
-- These rows never exist without their trailer commit. A failed git write leaves ZERO of them
-- (AC-160..AC-163), which is why every one is anchored to a commit sha.
```

followed by three tables — `t1_stage_transition (id, item, from_stage, to_stage, anchor_sha,
tree_hash)`, `t1_gate (…, anchor_sha)` and `t1_slice_seal (…, anchor_sha)`. The design is explicit and
good: the `anchor_sha` **is** the trailer commit, so a tier-1 row cannot exist without one.

**Nothing writes any of them.**

- `grep -rn 'INSERT INTO t1_' crates/ --include=*.rs --include=*.sql`, excluding tests, returns
  **nothing**. The only `t1_` mentions outside the migration are in
  `crates/rk-store/tests/it/contention.rs`, against a table that test creates for itself
  (`t1_slice_seal_probe`).
- The `EventStore` port's entire surface is `append`, `query`, `snapshot`, `rebuild`. `append` takes an
  `Event` and writes `t2_event`. **There is no method a caller could use to write a tier-1 row.**
- A live store, after the real `redkiln run` invocations that took this tree's first stage boundary:
  `t1_stage_transition` **0**, `t1_gate` **0**, `t1_slice_seal` **0**, `t2_event` **30**.

So the transition this stage now writes lands in `t2_event` — the tier-**2** table, whose own header
says truncating all of it *"leaves the stage-level next action unchanged (AC-164)"* and that `doctor`
classifies its loss as *"telemetry-only, never corruption"*.

**And this is not a gap 7h opened.** `Gate:` has had a trailer producer since 7d (`terminal_read`), and
`t1_gate` is empty too. The tier-1 layer has never been written on any axis, by anything, ever.

## Decision drivers

- AC-162's two clauses are about an ordering between a git write and a tier-1 row. With no tier-1 row
  on any axis, there is no ordering to get right or wrong — the criterion is **vacuous**, not met and
  not violated, and saying which of those it is matters.
- A vacuous criterion that reads as satisfied is the failure this runbook has paid for repeatedly: a
  filter that matches nothing reported as a clean zero. The same shape, one layer down.
- Adding a tier-1 write path means adding a method to `EventStore`, which is one of ADR-0003's five
  named ports with a fake-per-port rule. That is a port surface change, and it needs its own record,
  its fake, its conformance row and its fault-injection test (AC-162's *"a fault-injection test with a
  git backend that fails the commit, asserting zero store rows afterwards"*).
- §7h.2's declared scope is the two trailer axes' **producers**. The git half is in scope; inventing a
  port method is not, and doing it quietly under a stage that says *"no new write path is introduced"*
  would be the opposite of what that sentence promises.
- ADR-0148 and ADR-0150 both set the precedent: when the tempting repair is larger than the stage, the
  finding is named, measured and routed rather than half-built.

## Considered options

**1. Add a tier-1 write method to `EventStore` here.** It is the real fix and it is the largest thing
in reach: a port surface change, a fake, a conformance row, three call sites, and the fault-injection
test AC-162 names. It would also arrive with no consumer for two of the three axes, since
`Slice-Verdict:` has no run-path producer until §7h.3 and `Gate:`'s belongs to 7d. Rejected as scope.

**2. Write the tier-1 row from `rk-cli` with raw SQL, around the port.** Fastest, and it puts a
database write in the composition root, outside the port that owns persistence — exactly the seam
ADR-0003 exists to keep. Rejected outright.

**3. Redefine "store row" as the `t2_event` row, so AC-162 is met.** The `t2_event` row for the
transition *is* written, and after the trailer commit it would be written second. It is a redefinition
that makes a criterion pass by changing what it means: the tier-2 table is declared truncatable and
rebuildable, so anchoring a durability criterion to it would be false comfort. Rejected.

**4. Record the finding, bound it, and route it (chosen).** The `Story:` trailer is written as §7h.2
scopes it; the tier-1 half is named as OWED with its exact shape, and no document claims AC-162 is met.

## Decision

**1.** `s7h` writes the **git half** of the two axes it owns and no tier-1 store row. No method is
added to `EventStore` and no SQL is written outside `rk-store`.

**2. The finding is recorded as measured:** all three tier-1 tables have **zero** writers and are
**empty** in a live store; AC-160..AC-163 and AC-162 are **vacuous on every axis**, `Gate:` included.

**3.** No document, record or checkpoint may state that AC-162 is met, that a tier-1 row is written, or
that a failed git write leaves zero store rows — the last is currently true only because no tier-1 row
is ever written at all, which is not what the criterion is about.

**4. The route out is named**, so the next stage inherits a shape rather than a discovery: a tier-1
write method on `EventStore` (with its fake and conformance row), called from the same place the
trailer commit is made, taking the `anchor_sha` the commit returns — `commit_with_trailers` already
returns it, having re-read `HEAD` for exactly this kind of reason (ADR-0105 clause 4).

**5.** `Gate:`'s missing `t1_gate` row is **7d's axis, not this stage's**, and is recorded here only
because it is the evidence that the gap is structural rather than introduced by 7h.

## Consequences

The two axes §7h.2 delivers are durable **in git** and readable **through `doctor --rebuild`**, which
is the direction AC-427's differential exercises. They are not durable in the tier-1 sense the schema
describes, and the schema's own promise — *"these rows never exist without their trailer commit"* — is
true in the degenerate way: they never exist.

What this buys is that the gap is now countable. Before this record, `t1_stage_transition` being empty
was indistinguishable from a feature nobody had exercised; after it, the absence has a number (zero
writers, three tables), a cause (no port method), and a named route.

**What this forbids**

- Adding a tier-1 write method to `EventStore` under `s7h`.
- Writing `t1_*` rows from `rk-cli`, or any SQL against them outside `rk-store`.
- Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis.
- Reading `t1_*` emptiness as *"no transition has occurred"*, or as corruption.
- Treating the `t2_event` `stage_transition` row as a tier-1 durability record — it is in the table
  whose loss `doctor` classifies as telemetry-only.
- Describing `Gate:`'s missing tier-1 row as something `s7h` introduced.

## Evidence and confidence

**Three independent measurements, all 2026-09-22 at this stage's work, and each reproducible in one
command.**

1. **No writer.** `grep -rn 'INSERT INTO t1_\|insert into t1_' crates/ --include=*.rs --include=*.sql`
   filtered to non-test paths returns zero lines. A second, broader sweep — every `t1_` occurrence in
   `crates/**/*.rs` outside `tests/` — also returns zero.
2. **No rows.** A live `.git/redkiln/store.sqlite3`, after the real `redkiln run` invocations that took
   the first stage boundary this tree has ever taken:
   `t1_stage_transition` 0, `t1_gate` 0, `t1_slice_seal` 0, `t2_event` 30, `t2_memo` 0, `t2_usage` 0,
   `rk_identity` 1. The `stage_transition` payloads present in `t2_event` are `{"to":"triage"}` and
   `{"from":"triage","to":"analysis"}`.
3. **No method.** `crates/rk-ports/src/event_store.rs` declares four functions on the port and no
   fifth. A caller holding the port cannot write a tier-1 row even if it wanted to, which is why this
   is a port-surface finding and not an oversight at a call site.

**The `Gate:` control is what makes the claim structural rather than about `Story:`.** That axis has had
a producer since 7d — `terminal_read` writes a real `Gate:` trailer commit through
`commit_with_trailers` — and `t1_gate` is still empty. So the absence is on the store side of every
axis, not on the trailer side of the new ones.

**Confidence, and what is NOT asserted.** This record does not assert that the tier-1 tables are the
right design, nor that the route in clause 4 is the right one in detail — it asserts that the layer has
no writer, that the criteria over it are therefore vacuous, and that closing it is larger than this
stage. It asserts nothing about whether the existing `Gate:` trailer's own ordering relative to the
`t2_event` rows is correct; that ordering question becomes answerable only once a tier-1 row exists to
order against.
