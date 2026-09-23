---
id: "0160"
title: "**The `Slice-Verdict:` axis is NOT delivered at `s7h`, and the reason is a missing identity rather than missing plumbing.** Three absences, each measured: `EventKind::SliceSealed` has **exactly one construction in the whole workspace and it is `doctor/rebuild.rs`** — the REBUILD path, reading trailers back, not a run-path producer; `review::confirm` produces `ReviewClassDrift`, `ReviewBlock`, `AdvisoryCreated`, `QuarantineExpired` and `AdvisoryRetired` and **no `SliceSealed`**, so wiring the review gate would still not seal a slice; and the value's `<slice>` component is **declared nowhere** — `ItemRecord` is `{id, kind, stage, [facets]}` and there is no slice in this system, which is the same free-residual shape ADR-0159 had to settle for `<project>`. A producer built without settling it would invent an identity in code, and the failure mode is silent: an unqualified value is drift `unqualified-slice-verdict`, counted and never a transition. `s7h` therefore closes with the axis ARMED and this record as its debt"
status: proposed
date: 2026-09-22
legacy_id: none
source: crates/rk-cli/src/doctor/rebuild.rs (205-224 — the ONLY `EventKind::SliceSealed` construction, on the rebuild path); crates/rk-kernel/src/review/confirm.rs (99, 137, 157, 164, 185, 351-362 — the five kinds it does produce, `SliceSealed` not among them); crates/rk-git/src/trailer/grammar.rs (172-202 — exactly two tokens, `split_qualified` on token 0, `MalformedReason::UnqualifiedSliceVerdict`); crates/rk-cli/src/verbs/run.rs (`ItemRecord` — `id`, `kind`, `stage`, `[facets]`, and no slice); references/2026-09-08-architecture-design.md §5.4 (line 694, the `Slice-Verdict:` grammar row; line 688, F12's two-axis warrant); references/adr/0159 (the same residual, settled for `<project>`); references/adr/0156 (the six blockers on the shipped review stage); references/adr/0147 (the scope-boundary criterion this record is applied under)
supersedes: []
superseded_by: []
related: ["0109", "0147", "0155", "0156", "0158", "0159"]
evidence: ["F12"]
consumers: []
---

## Context and problem

`s7h` §7h.2 owes two axes and §7h.3 owes the `review`/`verify` call site. The first axis landed:
`Story:` has a producer, minted once per item by a real run. The second did not, and working out what
it needed is what this record exists to state, because "it was not built" and "it could not be built
without taking a decision nobody has taken" are different claims.

**Three absences, and none of them is the plumbing.**

**1. `slice_sealed` has no run-path producer, and its one construction is on the READ path.**
`grep -rn 'SliceSealed' crates/ --include=*.rs`, excluding tests, returns nine lines: the enum
declaration and its `ALL`/`as_str`/`Tier`/`trailer_axis` arms in `rk-types`, three `match` arms
(`fsm.rs:1508`, `breaker.rs:600`, `memo.rs:665`), and **one construction — `doctor/rebuild.rs:214`**,
inside the function that turns `Slice-Verdict:` trailers *back* into events. So the only thing that
makes a `slice_sealed` row is the rebuilder reading a trailer that something else was supposed to have
written.

**2. Wiring the review gate would not fix that.** `review::confirm` is the obvious candidate and it is
not one: its event constructors are `review_class_drift_event`, `review_block_event`,
`advisory_created_event`, `quarantine_expired_event` and `retire_not_re_raised`'s `AdvisoryRetired`.
**`SliceSealed` is not among them.** A `Confirmed` carries a `block_set()` and records, not a seal. So
even with the call site §7h.3 owes, the axis would still have no producer — which means §7h.3 and the
second axis are not one task done in sequence but two, and only the second is blocked on an identity.

**3. There is no slice.** design §5.4 fixes the value as:

> | `Slice-Verdict:` | `<project> "/" <slice> SP <verdict>` — exactly two whitespace-separated tokens |
> Column 0 | `approved` \| `changes-requested` … | A value without the `project/slice` form is drift
> `unqualified-slice-verdict` … |

`<project>` was a free residual and ADR-0159 settled it by declaring it. **`<slice>` is the same
residual and it is not settled.** `ItemRecord` is `{ id, kind, stage, [facets] }`; nothing in
`.redkiln/` names a slice; `grep` finds the word only in `rk-git`'s reconcile types and in the spike
corpus that F12 measured. The shipped fix item is one item, not a slice of one.

And the failure mode of guessing is the same shape ADR-0159 refused, one arm along: `classify`'s
`SliceVerdict` arm requires **exactly two** whitespace-separated tokens and `split_qualified` on the
first, so `Slice-Verdict: redkiln-rs/rk-fix-doctor-test-count approved` would parse — but a slice
identity equal to the item id is a claim that an item is a slice, made in code, that no record makes.
Get it wrong the other way (one token, three tokens, an unqualified first token) and the trailer is
`Malformed` → drift `unqualified-slice-verdict`, **counted and never a transition**, from a commit that
landed and that `grep '^Slice-Verdict:'` passes.

Underneath all three sits ADR-0156's six blockers: no shipped review activity can execute anyway.

## Decision drivers

- ADR-0147 already fixed this stage-boundary criterion at `s7g`: a finding that is the *instrument* is
  carried; a finding that is the *thing under test* is repaired; and what is neither is **scope**, named
  rather than half-built. A missing identity is scope.
- ADR-0159 is the precedent and the warning. The `<project>` residual looked like a detail and turned out
  to be a silent-inertness bug waiting to happen; `<slice>` has the identical shape and the identical
  failure mode, and it additionally requires deciding **what a slice IS** in a system whose item record
  has no such concept.
- A producer emitted from an invented identity would discharge
  `conformance_every_trailer_key_has_a_producer` — the lint is a substring scan over two directories, so
  one `TrailerKey::SliceVerdict` on a code line turns it green. **Discharging that lint with an invented
  slice would be the exact "source lint passes on code nothing calls" failure this runbook has paid for
  twice.**
- F12 is the warrant for the axis existing at all (20.3% of 74 slices passed through a rejected state),
  and F12's corpus had real slices. Inventing one here would put a value in the axis that the evidence
  the axis rests on never contained.
- `00-preamble.md`: a decision is not taken by implementing it.

## Considered options

**1. Use the item id as the slice.** `Slice-Verdict: <project>/<item> approved` parses and would
discharge the lint today. It asserts that an item is a slice, which no record says and which F12's
corpus contradicts — there, slices were units *within* a project's work. It would also make
`sealed_more_than_once` and `ever_rejected`, both of which `reconcile` computes per slice, mean
something different from what F12 measured. Rejected: it buys a green lint with a false identity.

**2. Declare `[slice]` on the item record.** The honest fix, and it is a schema change to `ItemRecord`
plus a decision about whether an item has one slice or many, plus what seals each, plus who writes the
`slice_sealed` row. That is a design surface, not a field. Rejected **for this stage** and named as the
route.

**3. Mint the trailer from a `slice_sealed` row and leave the row's producer to a later stage.** A
producer whose upstream does not exist: it would never fire in production, and the lint it discharges
cannot tell. Rejected outright — it is the shape the runbook's own traps name.

**4. Record the axis as ARMED with its three absences measured (chosen).** The `#[ignore]` stays, the
lint's reason and message are corrected to the measured state, and the debt is this record.

## Decision

**1.** `s7h` does **not** deliver the `Slice-Verdict:` axis. The ARMED lint
`conformance_every_trailer_key_has_a_producer` keeps its `#[ignore]`, and the reason is corrected to the
measured state — *"Slice-Verdict has 0 producers; Gate and Story are both minted as of s7h.2"* — rather
than left saying `Story` has none.

**2.** The lint's failure message is corrected too. It said *"a stage advance appends `stage_transition`
to the store with no `Story:` commit beside it"*, which is **no longer true** and would have misled the
next reader into looking for a defect that was repaired.

**3. The three absences are recorded by number** and are not to be re-derived: no run-path
`slice_sealed` producer (one construction, on the rebuild path); `review::confirm` produces no seal; no
declared `<slice>`.

**4.** No producer is written from an invented slice identity, and the lint is not discharged by a
`TrailerKey::SliceVerdict` occurrence that no run reaches.

**5. §7h.3's call site is separable and is also not delivered**, for ADR-0156's six blockers. This
record does not merge the two: a review call site could exist without sealing a slice, and sealing a
slice needs an identity a call site would not supply.

**6.** The route out is named: declare the slice on the item record (or decide that a fix lane has
exactly one, and say so), then a `slice_sealed` producer at the review stage's boundary, then the
trailer through the same `commit_with_trailers` path `Story:` now uses.

## Consequences

`s7h` closes with one of its two axes delivered and the other recorded, which is why its tracker row
says so rather than claiming the stage's scope. Phase-1 exit criterion (1) is no closer than ADR-0156
left it, and `s7i`'s first blocker — *"zero `dispatch_completed` events because no run completes"* — is
**not** cleared: the lane advances, and nothing dispatches.

What this buys over silence is that the axis's debt is now three measured absences with file and line
numbers rather than one unfinished bullet, and the next stage inherits a route instead of a discovery.

**What this forbids**

- Emitting a `Slice-Verdict:` trailer whose `<slice>` is the item id, the project, the stage, the run
  id, or any value not declared as a slice.
- Writing a `slice_sealed` producer whose row no run path appends, or a trailer producer whose upstream
  event has no producer.
- Removing `conformance_every_trailer_key_has_a_producer`'s `#[ignore]` before a **reachable**
  `Slice-Verdict:` producer exists.
- Claiming that `s7h` delivered both trailer axes, that the review gate has a call site, or that
  `s7i`'s first blocker is cleared.
- Reading ADR-0155's delivered `Story:` axis as evidence about this one — the two have different
  cardinalities and different blockers.

## Evidence and confidence

**Absence 1 is a census, not an impression.** `SliceSealed` occurs nine times in non-test `crates/`:
`rk-types/src/event_kind.rs` ×5 (declaration, `ALL`, `as_str`, `Tier::One`, `trailer_axis`), three
`match` arms (`fsm.rs:1508`, `breaker.rs:600`, `memo.rs:665`), and **one construction**,
`doctor/rebuild.rs:214`. That construction sits inside the block whose own comment says it writes
`slice_sealed` **per history entry** from trailers it has just read.

**Absence 2 was checked in the module, not assumed from the module's name.**
`grep -n 'EventKind::' crates/rk-kernel/src/review/confirm.rs` returns `ReviewClassDrift` (105),
`ReviewBlock` (157), `AdvisoryCreated` (171), `QuarantineExpired` (192), `AdvisoryRetired` (362). Five
kinds, no seal.

**Absence 3 is a property of the item schema.** `ItemRecord` carries `id`, `kind`, `stage` and
`[facets]`; both shipped items declare only those; `.redkiln/` has no slice anywhere.

**The lint's exact position is measured, and it moved.** Run un-ignored at this commit, it reports:
*"design §5.4 closes the trailer grammar at three keys and **2** of them are minted — no producer for:
`["Slice-Verdict"]`"*. Before `s7h` the same lint reported `Gate` 2, `Story` 0, `Slice-Verdict` 0 — one
of three. So exactly half this stage's discharge is done, and the half that is not is this record's.

**Confidence, and what is NOT asserted.** This record does not assert that a slice must live on the item
record — option 2 is the route, not the decision. It does not assert that `review::confirm` *should*
produce a seal; sealing may well belong at the stage boundary rather than in the confirm half, and that
is part of what the route has to settle. And it asserts nothing about whether the six blockers in
ADR-0156 are the *only* obstacles to a review-stage run: only blocker (0) has been reached by a real
run, and the others are read off code that nothing has executed.
