---
id: "0120"
title: The event-kind table gains dag_amended — the tier-2 delta an accepted deviation writes when selection steps 2–5 re-run, so AC-064's THEN names a kind that exists
status: accepted
date: 2026-09-13
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-064, AC-063, AC-062, AC-054, AC-061, AC-041, AC-042, **cited by id and named clause rather than by line**: the register was under concurrent amendment on 2026-09-13 and its line numbers moved +39 during Step 7b's repair pass, a hazard `runbook/phase-1/s7b-kernel.md`:19-23 and ADR-0119's own `source:` key both record and take the same route (AC-064's THEN reads at :1016 on this tree, `M` against `60f17c6`); references/2026-09-08-architecture-design.md §4.4 step 7 :515 (*"true ⇒ re-run 2–5 and log the delta"*) and §5 the event-kind table :699 (the closed-enum sentence), :701-722 (the table, `dag_assembled` at :710, `artifact.write` at :720); references/2026-09-07-process-catalog-proposal.md §2.5 step 7 :121 (*"re-run 2–5 and log the delta"*, AC-064's evidence line); references/2026-09-08-success-measures.md SM-017 :146-151 (the Instrument line naming `dag_amended`, and the floor kill); references/adr/0116-the-event-kind-table-gains-artifact-write.md :40-41 (*"the kind is minted by record or not at all"*) and :95-99 (the runbook option, rejected on mechanism); references/adr/0109-reconcile-at-every-entry-two-tiers-and-three-trailer-axes.md decision 2 :50-54 (the deleted-database test); references/adr/0046-deviation-is-authorable.md (the `on_false.to = "deviate"` decision, which mints no kind); references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md clause 7 (`rk-types` owns the enum); crates/rk-types/src/event_kind.rs and crates/rk-types/tests/it/event_kinds.rs at `60f17c6`; runbook/phase-1/s7b-kernel.md §7b.2(1) (the Slice-0 pass that STOPPED rather than minting it) and §7b.15's ARMED table; runbook/00-preamble.md :55 — the operator's decision of 2026-09-13 authorising the mint, taken against Step 7b's Slice-0 stop; there is no `spikes/DECISIONS-2026-09-13.md` and this record is that decision's primary record
supersedes: []
superseded_by: []
related: ["0024", "0046", "0105", "0109", "0116", "0119"]
evidence: ["F37"]
consumers: []
---

## Context and problem

**AC-064's THEN names an event kind that does not exist.** Its wording, in full:

> GIVEN a `RequestDeviation` whose `claimed_predicate` evaluates **true** / WHEN the engine handles the exit / THEN
> steps 2–5 re-run and a `dag_amended { deviation_id, added, removed, rows_refired }` event is written naming every
> change to `R` / AND the deviation record is part of the replay input set, so an amended DAG is not a hidden input.

**`dag_amended` / `DagAmended` occurs six times in this tree, in three files** [measured, 2026-09-13,
`git grep -n 'dag_amended\|DagAmended'`]: **twice above the runbook** — AC-064's THEN (`:1016`) and SM-017's
*Instrument* line (`references/2026-09-08-success-measures.md`:147) — and **four times inside
`runbook/phase-1/s7b-kernel.md`** (`:179`, `:180`, `:308`, `:1532`), which are that stage file's own record of this
gap, written by the Slice-0 pass on 2026-09-13. *(The two-hit figure `s7b`:181 states was measured before that pass
wrote its own paragraph; excluding the file that records the gap, it still holds.)* It is **absent from design §5's
event-kind table** (`:701-722`) and **absent from `crates/rk-types/src/event_kind.rs`**, whose `ALL` is `[Self; 28]`
[measured, 2026-09-13]. Design §5 states the constraint that makes this a record's problem rather than an
implementer's: *"the `EventKind` enum is **closed**, a kind not in this table is a schema change reviewed as one"*
(`:699`).

**Neither decision AC-064 cites mints it.** ADR-0046 decides only that deviation is *authorable* —
`on_false.to = "deviate"` with `deviate = { kind, target, claimed }`, *"the kernel handles it per catalog §2.5 step
7"* — and names no event. ADR-0024 (admissibility lives in the activity, requirement lives in the pack) does not
mention an event at all [measured, 2026-09-13: `dag`, `event` and `EventKind` return no substantive hit in either
record]. So AC-064 is unimplementable as written, and Step 7b's §7b.15 carries it **ARMED, NOT DISCHARGED** with
*"an ADR is OWED"* as the reason.

**The Slice-0 pass STOPPED rather than adding the variant, and that was correct.** `00-preamble.md`:55 —
*"Where a stage is tempted to decide something else, it stops"* — and **ADR-0116 forbids the shortcut by name**:
*"the kind is minted by record or not at all"* (`:40-41`), with *"mint the kind in the runbook"* enumerated and
**rejected on mechanism** (`:95-99`). What was owed was this record. It is now authorised and written.

**The gap is not only AC-064's.** Design §4.4 step 7 requires the engine to *"re-run 2–5 and **log the delta**"*
(`:515`), and with no kind there is no log: the one place `R` legitimately changes after step 5 changes without a
durable mark. SM-017's *Instrument* line names `dag_amended` events as one of its three instruments, and its only
fireable kill — *"if uses stay at 0 over the first 100 items, the channel is dead code and is removed rather than
carried"* — is a count of accepted deviations that nothing can take. The channel's traffic is already the weakest
thing in this area: deviation was **offered 71 times and used 0** in 61 runs, so P7 §4.4(3) is recorded as *not
answerable*, not as 0% [measured, F37].

## Decision drivers

- A criterion naming an event kind no table defines cannot be built, and the enum is closed by design — **so the kind
  is minted by record or not at all** (ADR-0116's driver, unchanged and applied to a second kind).
- *"The agent never edits `R`"* is the constraint the whole deviation channel exists to preserve (design §4.4 step 7,
  ADR-0046). An accepted deviation is the one path on which `R` changes after assembly, and an unrecorded change to
  `R` is precisely the hidden input AC-064's second AND forbids.
- SM-017's floor kill is the only kill in that row that can fire, and it is a count of uses. Without the event the
  count has no source, and a channel measured at **0 uses in 71 offers** would be retired or retained on no evidence
  either way.
- `dag_assembled` already records the assembly of `R` with `rows_fired` and `skipped`. Recording the assembly and not
  its amendment leaves the log describing a required set the run did not execute.

## Decision

**1. `dag_amended` is a tier-2 event kind**, carrying the four payload fields AC-064 names, and it is written **once
per accepted deviation**. Its row in design §5's table:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `dag_amended` | 2 (rebuildable by replay) | `deviation_id, added, removed, rows_refired` | selection step 7, on a `claimed_predicate` that evaluates **true** — the re-run of steps 2–5, not step 5 |

**The payload is the register's and is taken as given**, not re-derived here: `deviation_id, added, removed,
rows_refired`, together *"naming every change to `R`"*. This record fixes the four names and their obligation; it
does not fix their Rust types.

**2. Tier 2, and the deleted-database test is why.** ADR-0109 decision 2 asks one question — *if the database were
deleted, would `redkiln run` take a different next action?* — and here the answer is no, for a reason AC-064 itself
supplies: **the deviation record is part of the replay input set**, so the amended DAG is reproduced by replaying
selection over that input, never by reading this event back. The event is the *log* of the delta, not its carrier.

**There is a second, structural half to the same answer: a delta may not outrank its base.** `dag_assembled` is
tier 2 in design §5's own table, *"rebuildable by replay"*. A tier-1 record of a change to a tier-2 object would
survive into the `doctor --rebuild` projection (ADR-0109 decision 4, ADR-0105 clause 9) after the object it amends
had been dropped, leaving a delta with no base — which is worse than not recording it. It is **not** tier 1, it
carries **no trailer**, and no fourth trailer axis is minted for it (ADR-0105 clause 3's three axes are closed).

**3. The producing step is step 7's accepted branch, not step 5.** AC-064's evidence line is catalog §2.5 step 7,
*"re-run 2–5 and log the delta"*, and design §4.4 row 7 says the same. The producer is therefore **the re-run**: one
`dag_amended` per accepted deviation, naming the delta that re-run produced. A second `dag_assembled` is **not** a
substitute and is not emitted in its place (decision 5's forbids).

**4. The append happens OUTSIDE the pure function.** `select()` reads no store and writes none — AC-041 makes its
inputs exactly the four digests, and §7b.9(5) names a store-reading `select()` as the hidden input AC-041–AC-043
exist as a chain to prevent. The delta is **returned** by the re-run and the kernel's step-7 handler appends the
event through the `EventStore` port, on the shape ADR-0119 fixes for the selection memo key (*derived inside
`select`, looked up outside it*). The kind itself lives in `rk-types` (ADR-0105 clause 7); the single write path is
`rk-store`'s.

**5. The other two branches of step 7 write no `dag_amended`.** A `claimed_predicate` that evaluates **false** opens
a gate with the request, the predicate and its evaluated operands attached, and `R` is unchanged at the moment the
gate opens (AC-063) — there is no delta, so there is no event. A `kind` outside the activity's own `deviation_ok` is
refused as a configuration error **before** predicate evaluation (AC-062) — no gate, no delta, no event. Only the
true branch amends `R`, and only the true branch writes.

**6. The table row and the `rk-types` variant land in the same change** — ADR-0116 clause 5, restated for this kind:
`EventKind` is closed and `rk-types` owns it, so a kind in the design table and not in the enum, or the reverse, is
schema drift. **The instrument that catches it here is the hand-written completeness pin, not the golden schema
snapshot**, and the difference is measured: `crates/rk-types/tests/it/schema_snapshot.rs` snapshots
`registry::export_all()` and `CompanionKind::ALL`, and `EventKind` is in neither, so **adding a variant does not move
the snapshot** [measured, 2026-09-13; `runbook/phase-1/s7b-kernel.md`:189-193 states the same]. What moves is
`crates/rk-types/tests/it/event_kinds.rs`'s `assert_eq!(EventKind::ALL.len(), 28)` and `event_kind.rs`'s
`[Self; 28]`, plus the exhaustive `wire_name` and `tier` matches — the `E0004` in `tier()` being what forces
decision 2 to be answered by whoever adds a kind, rather than defaulted.

**7. What this record does NOT decide**, named rather than left to be discovered: **how `deviation_id` is derived**.
AC-064's *Verified by* asserts *"fields present, `added`/`removed` match the fixture's expected delta"* and says
nothing about the identity's construction, and §7b.9(5) shows that this class of identity question — run-scoped
versus item-scoped, derived versus stored — is a register amendment's to settle, not a record's by implication. It
is a residual against AC-064's implementation in Step 7b Slice 3, not a free residual this record leaves open for
a *stage* to decide.

## Considered options

### Option — reuse `dag_assembled` for the re-run

Emit a second `dag_assembled` when steps 2–5 re-run, and rewrite AC-064's assertion against it. **Rejected:** its
payload is `inputs_digest, nodes, edges, rows_fired, skipped` and has no place for `deviation_id` or `rows_refired`,
so the two fields that make the event a *deviation* record would be dropped; a reader could not tell a run that
deviated from one that re-assembled for any other reason; and SM-017's use count would have no numerator, which is
half of what the kind is for. It also states the wrong thing: a second assembly reads as a fresh `R`, while the
record AC-064 wants is *the change to* an `R` that already existed.

### Option — make it tier 1, on a trailer

Treat an amendment of `R` as decision-bearing and write it git-first. **Rejected** on decision 2's two grounds: the
deleted-database test answers *no* because the deviation record is already in the replay input set, and a tier-1
delta over a tier-2 base survives into the rebuild projection after its base is gone. It would additionally need a
fourth trailer axis, and ADR-0105 clause 3's grammar table is closed at three — a change of a different size, taken
for a kind whose channel has **0 measured uses**.

### Option — leave AC-064 ARMED and record the kind as OWED

Exactly what Step 7b's Slice-0 pass did: stop, record two free residuals (the tier and the producing step), and
carry AC-064 in §7b.15's ARMED table. **Rejected as a terminal state, not as a stage act** — the stop was correct
and is the reason this record exists. Halt 8's carve-out is for what a *stage* cannot run yet, not for an instrument
no record defines (ADR-0116:88-93), and leaving it arms a criterion whose blocker is one record rather than one
measurement.

### Option — mint the kind in the runbook

**Rejected on mechanism**, and by name: `00-preamble.md`:55 — *"Where a stage is tempted to decide something else,
it stops"* — and the enum is closed by a design section ADR-0105 clause 7 owns. This is ADR-0116's own rejected
option (`:95-99`) applied to the second kind that reached it; a kind invented at Step 7b would be exactly the *"a
rule with no ADR is a rule the next refactor removes"* failure ADR-0107's drivers name.

## Consequences

**Positive:**
- AC-064 becomes implementable exactly as the register words it, and its §7b.15 ARMED row closes at Slice 3.
- Design §4.4 step 7's *"log the delta"* acquires the log it names.
- SM-017's floor kill — 0 uses over 100 items — gains its count, so the deviation channel is retained or retired on
  evidence rather than on the absence of it.
- AC-042's replay can distinguish a run that deviated from one that did not, which is what makes AC-064's second AND
  (*"an amended DAG is not a hidden input"*) checkable rather than assertable.

**Negative:**
- One more event on a path that has **never once been taken in a measured run** — 71 offers, 0 uses [measured, F37].
  Tier 2 and small, but it is code and a variant carried for a channel with an empty denominator, and no measurement
  bounds its volume because there is none to bound.
- A closed enum moves for the second time in two days. The golden schema snapshot does **not** move with it
  (decision 6), but the hand-written completeness pin does, and every count of the set — `[Self; 28]`, the test's
  `28`, design §5's row count — moves with it.

**What this forbids**
- Forbids an accepted deviation that amends `R` without writing a `dag_amended` event.
- Forbids writing `dag_amended` on a **false** `claimed_predicate` (that is AC-063's gate) or on a `kind` outside the
  activity's `deviation_ok` (that is AC-062's configuration error, refused before predicate evaluation).
- Forbids emitting a second `dag_assembled` in place of `dag_amended` for the re-run of steps 2–5.
- Forbids recording `dag_amended` at tier 1, carrying it on a trailer, or minting a fourth trailer axis for it.
- Forbids appending it from inside `select()`, or reading it back inside `select()` — the pure function's inputs stay
  the four digests (AC-041).
- Forbids treating an absent `dag_amended` count as a measured 0 uses for SM-017's floor kill before the event
  exists in a run.

## Evidence and confidence

- [measured, 2026-09-13] `git grep -n 'dag_amended\|DagAmended'` over the whole tree returns **6 hits in 3 files**:
  `references/2026-09-08-acceptance-criteria.md`:1016, `references/2026-09-08-success-measures.md`:147, and
  `runbook/phase-1/s7b-kernel.md`:179, :180, :308, :1532. The four runbook hits are the Slice-0 pass's own record of
  this gap and postdate the *"exactly two hits"* measurement `s7b`:181 quotes; above the runbook the figure is two.
- [measured, 2026-09-13] `crates/rk-types/src/event_kind.rs` carries **28** variants and `ALL: [Self; 28]`; no
  variant's `wire_name` is `dag_amended`.
- [measured, 2026-09-13] `crates/rk-types/tests/it/schema_snapshot.rs` snapshots `registry::export_all()` and
  `CompanionKind::ALL` only; `EventKind` appears in neither, so this addition does not move the golden snapshot. The
  hand-written pin at `crates/rk-types/tests/it/event_kinds.rs` is what moves.
- [documented, references/2026-09-08-architecture-design.md :699] *"the `EventKind` enum is closed, a kind not in
  this table is a schema change reviewed as one"*; the table at :701-722 carries `dag_assembled` (:710) and no
  amendment kind.
- [documented, references/2026-09-08-architecture-design.md :515] design §4.4 step 7: *"true ⇒ re-run 2–5 and log the
  delta; false ⇒ gate with the request and its evaluated operands attached"*, with *"The agent never edits `R`"* in
  the same row.
- [documented, references/2026-09-07-process-catalog-proposal.md :121] catalog §2.5 step 7, AC-064's evidence line,
  in the same words.
- [documented, references/adr/0116 :40-41, :88-99] the minting rule and the two rejected options this record
  re-applies.
- [measured, F37; 61 runs, 297 records, 0 void, $131.62] deviation was **offered 71 times and used 0**. P7 §4.4(3) is
  recorded as **not answerable**, not as 0%, and SM-017 carries the same reading.
- **Confidence:** the gap is measured and unambiguous, and the payload is the register's own. The **tier** and the
  **producing step** are *reasoned* — from ADR-0109's deleted-database test, from `dag_assembled`'s tier in the same
  table, and from AC-064's own evidence line — and are **not measured**; no spike bears on either, and the channel
  they describe has zero measured traffic. This is the same confidence ADR-0116 recorded for the same two columns,
  and it is stated here rather than inherited.
