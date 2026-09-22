---
id: "0141"
title: "The step list is carried on `RecordedSpec` as an **explicit `Option`** — `supplied` (none transcribed) and `with_steps` (transcribed) — superseding **ADR-0138 clause 4 in part**, at that clause's whole `## Decision` sentence. Clause 4 was an if-then whose antecedent FIRED (the fan-out is **17**, not three) and whose consequent — take the `&[Step]` fallback — was **declined**; a Decision sentence in that state does not stand, which is the route E-0138-1 named as owed after being corrected for taking the cheaper one. Clause 1 stands and is what was executed"
status: proposed
date: 2026-09-22
legacy_id: none
source: references/adr/0138 decision 4 (the clause this record supersedes in part) and decision 1 (which stands); references/adr/0092 rule 1 (a `## Decision` sentence moves by supersession, never by erratum); references/adr/ERRATA.md E-0138-1, whose parenthetical claiming clause 4's instruction *"stands and was followed"* is withdrawn and which names this supersession as OWED; references/adr/0139 (the same route taken for the identical shape 53 minutes earlier, cited as the precedent this record follows); crates/rk-kernel/src/replay.rs (`RecordedSpec`, `supplied`, `with_steps`, `steps`); crates/rk-kernel/src/runtime/step.rs (`run_activity_steps`' fall-through and its empty-list refusal); crates/rk-cli/src/verbs/run.rs (`bind_nodes`, the one production transcription)
supersedes: ["0138"]
superseded_by: []
related: ["0083", "0097", "0099", "0105", "0138", "0139"]
evidence: []
consumers: []
---

## Context and problem

ADR-0138 clause 4 reads, in full:

> **If the transcription fans out more widely than the three sites that thread `RecordedSpec` today, the
> fallback is option 4 — `&[Step]` as a parameter to `run_activity` — and the switch is recorded in the
> commit that makes it, with the measured fan-out that caused it.** It is not made silently, and it does
> not reopen options 2, 3 or 5.

**The antecedent fired and the consequent was declined.** Measured at the tree clause 1 was executed
against: `RecordedSpec::supplied` had **17** call sites (1 production, 16 across the `rk-cli` and
`rk-kernel` integration suites) and `run_activity` **19** (2 production, 17 in suites). The "three" counted
the production thread only and silently excluded every test call site — which is precisely the population a
signature change is paid for in. E-0138-1 corrects that arithmetic and the arithmetic is not in dispute.

**What is in dispute is the route.** E-0138-1 justified itself as an erratum on the ground that *"Clause 4's
normative content is its **instruction** — take the fallback, and record the switch with the measurement —
and that instruction stands and was followed."* The instruction is *take the fallback*. The fallback was not
taken; the same entry says so two sentences earlier (*"the shape taken is a third one neither the clause nor
its options list names"*). **An if-then `## Decision` sentence whose antecedent fired and whose consequent
was declined does not "still stand".** ADR-0092 rule 1 routes it by supersession.

**And the corpus had already applied the opposite route to the identical shape, the same morning.** ADR-0139
supersedes ADR-0137 clause 8 — one clause, same author, same day, the prohibition retained — citing ADR-0092
rule 1 in terms: *"this moves a `## Decision` sentence, so the route is a **supersession**, not an erratum —
even though the record being moved is … its own author is moving it."* Two opposite routes, one rule, one
morning. An adversarial pass found it; E-0138-1's parenthetical is now withdrawn in place and names this
record as owed.

**What was actually built, and it is neither clause 1's plain form nor clause 4's fallback.**
`RecordedSpec` gained `steps: Option<Vec<crate::catalog::Step>>`. `RecordedSpec::supplied` keeps its
signature and yields `None`; a new `RecordedSpec::with_steps` yields `Some`. `run_activity_steps` falls
through to `run_activity` unchanged on `None`, and refuses an explicitly **empty** list by name. So clause
1's *"the step list is carried on `RecordedSpec`"* was executed exactly; what the record did not describe is
the **optionality**, and optionality is what made the 16 machinery call sites survive untouched.

## Decision drivers

- ADR-0092 rule 1, applied consistently rather than by convenience. The cost of getting this wrong is not
  the clause — it is that two records one morning apart would disagree about what the rule requires.
- The 16 untouched call sites are the substantive gain and the reason the third shape was taken: they test
  dispatch, retry, commit races and AC-013 re-entry, and have no catalog activity behind them. Making them
  assert a step list would couple the machinery suite to the catalog for no evidence.
- ADR-0138's forbids bullet — *"a missing transcription is an error naming the activation, never an empty
  walk"* — is the property that had to survive, and it does; `None` is not an empty walk.
- This record must not be read as repairing anything. The executor built under ADR-0138 is recorded
  **BLOCKED**, and a correct bookkeeping route for its records changes none of that.

## Considered options

1. **Supersede clause 4 in full and record the optional shape as the decision (taken).**
2. **Leave E-0138-1's erratum standing.** Rejected on ADR-0092 rule 1, and rejected harder because it makes
   the corpus inconsistent with ADR-0139 in a way a reader cannot resolve from the records themselves.
3. **Take clause 4's fallback now** — move to `&[Step]` on `run_activity`. Rejected: it churns 19 call
   sites, removes the step list from the run record that clause 5 wanted it in, and buys nothing the
   optional shape does not already give.
4. **Supersede ADR-0138 in FULL.** Rejected: clauses 1, 2, 3 and 5 stand and were executed. A full
   supersession would move the predecessor to `superseded` and imply its routing warrant was withdrawn,
   which it was not — `RecordedSpec` is still the right home and residual 7 still forbids the alternative.

## Decision

1. **ADR-0138 clause 4's `## Decision` sentence is superseded in full**, both the if-then and its trailing
   *"It is not made silently, and it does not reopen options 2, 3 or 5."* **Clauses 1, 2, 3 and 5 stand
   unchanged**, and clause 1 is what was executed. ADR-0138 **stays `accepted`** and carries the
   `superseded_by` link, as a partial supersession requires.

2. **The step list is `steps: Option<Vec<Step>>` on `RecordedSpec`, with two constructors.**
   `RecordedSpec::supplied(activation, spec)` yields `None` and means *this caller transcribed none*.
   `RecordedSpec::with_steps(activation, spec, steps)` yields `Some` and is what the composition root uses.
   The kernel still derives, guesses and defaults nothing — `None` is an **explicit absence**, which is the
   treatment ADR-0032 gives an absent optional input and the opposite of a default.

3. **`None` runs the pre-`s7g` activation dispatch, and is never an empty walk.**
   `run_activity_steps` falls through to `run_activity` unchanged. An explicitly **empty** transcribed list
   is a different thing and is **refused by name**, because a zero-step walk reporting success is the
   vacuity the stage exists to end. ADR-0138's forbids bullet is preserved exactly.

4. **The production path must always transcribe, and that is currently a convention rather than a guard.**
   Measured: **one** `with_steps` call site (`rk-cli`'s `bind_nodes`) against **16** `supplied`, all in
   suites. Nothing fails if a future production site reaches for `supplied`. **A guard asserting that the
   composition root transcribes is OWED**, and its absence is recorded here rather than left to be
   discovered — the same class of gap as a lint that passes on code nothing calls.

5. **The `&[Step]` fallback is not reopened by this record and is not forbidden by it.** If it is ever
   taken, it needs its own record; clause 4 no longer pre-authorises it, because clause 4 no longer stands.
   ADR-0138's closed routes — a sixth `ActivitySpec` field, a catalog load inside `runtime/`, re-derivation
   from the spec — remain closed by their own records.

**What this record does NOT decide.** It repairs **none** of the executor defects `runbook/PROGRESS.md` §1's
Step 7g row records — the empty `predicate::Context`, the second-`agent`-step report overwrite, the
swallowed post-commit failures, the no-op `gate` arm. It does not decide the DAG-node ↔
`Activity::id_at_version` join, still OWED and still named by `RecordedSpec`'s own docstring. It does not
change AC-013's per-activation grain.

## Consequences

**What this forbids**

- Forbids treating `steps: None` as an empty walk, a zero-step success, or grounds for deriving a list.
  It means one thing: this caller transcribed none.
- Forbids a **production** call site constructing `RecordedSpec::supplied`. The composition root holds the
  `catalog::Activity` and must transcribe; until the guard clause 4 owes exists, this bullet is the rule.
- Forbids citing ADR-0138 clause 4 as authority for moving to `&[Step]` on `run_activity`. That clause is
  withdrawn; the move needs a new record.
- Forbids reading this record as evidence the step executor works. It does not, and the tracker says so.
- Forbids correcting a false `## Decision` sentence by erratum in future, whatever the record's age or
  authorship — the inconsistency this record exists to close.

**What follows**

- E-0138-1 is reduced to what an erratum may legitimately carry: the corrected arithmetic (17, and 19).
  Its route parenthetical is withdrawn in place and points here.
- ADR-0138 gains a `superseded_by` link and stays `accepted`; the README's Supersessions table gains the
  row, and its *"none moving a Decision"* clause — already withdrawn at the repair pass — is now true again
  of every remaining entry in `ERRATA.md`.
- The corpus applies one route to this shape: ADR-0139 for ADR-0137 clause 8, ADR-0140 for clause 5, and
  this record for ADR-0138 clause 4.

## Evidence and confidence

- [measured 2026-09-22, at `085d1aa`, the tree clause 1 was executed against] `RecordedSpec::supplied`
  **17** call sites; `run_activity` **19**. At HEAD, after the transcription landed: `supplied` **16**,
  `with_steps` **1**, `run_activity` **20**.
- [measured 2026-09-22] `RecordedSpec` carries `steps: Option<Vec<crate::catalog::Step>>`; `supplied` sets
  `None`, `with_steps` sets `Some`; `steps()` returns `Option<&[Step]>`.
- [measured 2026-09-22] `run_activity_steps`' first act on `None` is
  `return self.run_activity(item, activity_id_at_version, step_id, spec, cap, now)` — the pre-`s7g` path,
  unchanged; and an empty `Some(&[])` returns `ActivityReport::refused` naming the activation.
- [documented] E-0138-1's withdrawn parenthetical and its own body, which contradict each other on whether
  clause 4's instruction was followed; and ADR-0139:45, the same-rule precedent taken 53 minutes earlier.
- **Confidence: high on the route, which is now settled by two independent applications of ADR-0092 rule 1
  on the same day.** What this record cannot claim, and does not, is that the shape it ratifies is *good* —
  the executor it serves is recorded BLOCKED on four blocking defects, none of which is about where the
  step list is carried. The optionality survives that judgement because it is orthogonal to it.
