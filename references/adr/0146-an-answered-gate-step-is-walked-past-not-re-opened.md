---
id: "0146"
title: "An **answered** catalog `gate` step is **walked past**, not re-opened — superseding **`0145` clause 7 in part**, at that clause's sentence *\"The run resumes into the same step when the gate is answered\"*, which described the defect rather than the behaviour. The `gate` arm now asks `RunState` the same three-way question `fsm::next_action`'s clause 2 has asked since 2026-09-17 — never reached → open at reach 1; latest **void** → re-open at reach n+1 (AC-252); otherwise **proceed** — because `gate::reach` mints at `reaches + 1` unconditionally, so an answered gate re-opened at reach 2, then 3, for ever. Measured through the real binary over three run→answer rounds: three `gate_opened` rows, three distinct tokens, and **never a `Dispatch`**. The **15** activities that gate before any dispatch never commit, so `is_committed` never skips their node, so every resume walked back to the same step: they were permanently unexecutable and `answer` was a treadmill. Clause 4 additionally bans a `:` in a `question`, because a catalog kind colliding with a selection gate's `\"{reason}:{axis}\"` can **suppress that gate entirely**"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0145 (clause 7, superseded in part here); crates/rk-kernel/src/workflow/fsm.rs (`next_action` clause 2 — the three-way discipline this mirrors, and its own comment recording the same bug one level up); crates/rk-kernel/src/workflow/gate.rs (`reach`, which mints at `reaches + 1` and asks nothing); crates/rk-kernel/src/runtime/step.rs (the `StepVerdict::Gate` arm); crates/rk-kernel/src/catalog.rs (the `[[step]]` loader, clause 4's refusal); crates/rk-kernel/src/workflow/fsm.rs (`RunState::gates_by_kind`, keyed on the bare kind string)
supersedes: ["0145"]
superseded_by: []
related: ["0017", "0022", "0044", "0145"]
evidence: []
consumers: []
---

## Context and problem

ADR-0145 wired the catalog `gate` step to the runtime's own gate machinery. Its arm called
`workflow::gate::reach` **unconditionally**:

```rust
let record = match crate::workflow::gate::reach(gate.state, kind, &ctx) {
```

`gate::reach` mints at `state.reaches(kind).saturating_add(1)`. It asks nothing about whether the
gate has already been reached, answered, or voided — by design: it is the *minting* entry, and the
*decision* belongs to the caller. `fsm::next_action`'s clause 2 makes that decision, and has since
2026-09-17, with a comment recording the very bug this record repairs one level up:

> *"(Until 2026-09-17 this derived `(kind, ctx.tree_hash, ctx.deadline)` and looked the token up — so
> a tree that moved after an answer would have re-opened an answered gate on every call, and a void
> token could never re-open at all.)"*

The step arm did not mirror it. **So an answered gate step re-opened at reach 2, then 3, for ever.**

Measured through the real binary, `pack-min` with a `[[step]] kind = "gate"` ahead of the agent
step, three run→answer rounds:

```
ROUND 1 run exit=0   rows: [8e41594d@reach1]
ROUND 2 run exit=0   rows: [8e41594d@reach1, a64562d1@reach2]
ROUND 3 run exit=0   rows: [8e41594d@reach1, a64562d1@reach2, 86f5bf45@reach3]
TOTAL runtime gate_opened rows after 3 ANSWERED rounds: 3 — and never a `Dispatch`
```

The same harness over the pristine **selection** gate resumes past it on the second run and reaches
`Dispatch, DispatchCompleted, ActivityCommitted`. So ADR-0145's clause *"it mints … so the fold
cannot tell the two producers apart and does not have to"* is true of the **row shape** and false of
the **gate lifecycle**: the two producers behaved differently on re-reach, and only the step
producer was unbounded.

**Blast radius: the 15.** An activation that gates before dispatching never commits, so
`is_committed` never skips its node, so every resume walks back to the same gate step. Those
activities — 13 with no `agent` step at all, 2 with a gate ahead of one — were **permanently
unexecutable**, and the `answer` verb was a treadmill. The 55 that gate *after* a commit were safe
only because `is_committed` skips the node next time: **the defect was masked there, not absent.**

🛑 **ADR-0145 clause 7 states this as the design** — *"The run resumes into the same step when the
gate is answered"* — and there was no mechanism by which resuming into the same step could ever get
**past** it. The clause is the defect, written down as the intent, which is why this is a
supersession and not an erratum.

**Why it was invisible.** All four of ADR-0145's guards pass `empty_state()`, where
`reaches(kind) == 0` and the arm's only reachable branch is the one that opens. **A guard whose
fixtures all sit at reach 1 cannot see a re-reach defect at all** — the same shape as `s7g`'s own
finding (12), where six filters could not see a walk they never entered.

**And a second hole this record's predecessor OPENED.** `RunState::gates_by_kind` is keyed on the
bare kind string. Before ADR-0145 a catalog `question` was dropped at load and could collide with
nothing; after it, `question = "unknown_facet:has_repro"` lands in the same reach sequence as that
abstention gate. Measured: the step gate minted **reach 2 of the selection gate's sequence**,
`latest_gate` then pointed at the step gate's token, and — because `fsm`'s clause 2 opens only when
`reaches == 0` — **the abstention gate of that axis could no longer be opened at all.** A catalog
edit that silently suppresses an abstention, unsatisfiable or appetite gate is a far larger hole
than the one ADR-0145 set out to close.

## Decision drivers

1. **One discipline, in two places, written once and mirrored — not invented twice.** `fsm`'s clause
   2 is the reference implementation and its three dispositions are already reasoned about in
   AC-252, AC-257 and AC-259.
2. **A gate must be answerable in finite time.** A mechanism whose answer produces a fresh gate is
   not a gate.
3. **A new kind namespace must not be able to capture an existing one.**
4. **Refuse at load, where the value enters** — the `run` path loads through the strict loader, so a
   diagnostic-only refusal would leave `run` loading the collision.

## Considered options

**(a) Re-open only when the tree hash moves.** Rejected by `fsm`'s own recorded history: that is
exactly what clause 2 did until 2026-09-17, and it both re-opened answered gates and made a voided
token un-re-openable.

**(b) Have the walk skip a `gate` step whose kind has any reach at all.** Rejected: it breaks
AC-252, where a **void** must be followed by a re-open fresh in all three fields.

**(c) Mirror `fsm::next_action` clause 2's three-way question.** Taken.

**(d) For the collision: refuse only the exact `"{reason}:{axis}"` shapes.** Rejected: `GateReason`
is a closed set a later record may widen, and a ban tracking its members goes stale the moment it
does. A colon is banned outright.

## Decision

1. **Supersedes ADR-0145 clause 7 in part**, at the sentence *"The run resumes into the same step
   when the gate is answered."* Resuming into the same step means **evaluating that step's gate
   disposition**, and proceeding when it is settled. ADR-0145 stays `accepted`; every other clause
   stands.
2. **The `gate` arm asks `RunState` before minting**, mirroring `fsm::next_action` clause 2:
   - `reaches(kind) == 0` → open at reach 1;
   - latest record **void** → re-open at reach n+1 (**AC-252**);
   - otherwise → **the walk proceeds to the next step.**
3. **An EXPIRED latest is not re-opened here.** Its disposition is the expiry policy's terminal path
   (ADR-0017; AC-257, AC-259), exactly as `fsm` treats it — both fall through.
4. **A `question` carrying a `:` is refused at LOAD**, by the strict loader, naming the activity and
   the step. Zero of the shipped pack's 45 distinct questions contains one, so this refuses nothing
   that exists; it is what keeps that true.
5. **The guards must exercise reach > 1.** A fixture set that sits entirely at reach 1 cannot
   observe any of this, and that is how the defect shipped.

## Consequences

**What this forbids**

- Forbids minting a gate record without first asking `RunState` what that gate's disposition is. A
  gate whose answer produces a fresh gate is not a gate.
- Forbids re-opening an answered gate, and forbids failing to re-open a voided one. Both directions
  are wrong and the three-way question is the only shape that gets both right.
- Forbids re-opening an expired gate from this arm; expiry is ADR-0017's terminal path.
- Forbids a `question` that can collide with a selection gate's kind namespace.
- Forbids a guard set for gate behaviour in which every fixture sits at reach 1.

**What follows**

The 15 gate-before-dispatch activities become executable in principle: the gate opens once, the
answer settles it, and the next run walks past it to the steps behind. **It does not follow that any
of them completes** — the `check` arm still ends most walks (ADR-0142 clause 4) and step-output
resolution stays OWED.

🛑 **And one thing this does NOT close, measured and named rather than left to be found.** A step
gate's **class** is its `question`, and `run`'s `expiry_pass` refuses a class with no
`[[gate_expiry]]` row: `lint_expiry_policy_row(class, None)` → `NoActionDeclared` → exit **3**.
Measured over the shipped packs: `grep -c gate_expiry .redkiln/packs/*.toml` → `bug-standard.toml:0`,
`story.toml:0`. **Zero rows, for zero of the 45 question classes.** So a step gate that passes its
deadline makes the next run exit 3 with *"the pack declares no usable expiry action for gate class
`draft.accept`"*. ADR-0145 added 45 classes to the runtime's reachable set and added no expiry rows
for any of them; this record does not add them either. **It is OWED**, and the honest options are to
ship the rows or to make an unmatched step-gate class a load-time refusal rather than a run-time
exit 3.

## Evidence and confidence

**Measured, not argued.** Two guards added to `rk-kernel/tests/it/gate_step.rs`:
`an_answered_gate_step_is_walked_past_and_not_re_opened` puts a real opened-and-answered history
behind the walk — folded, not stubbed, because `RunState::initial` is unreachable from an
integration test — asserts `reaches == 1` as a PREMISE so the test cannot silently fall back to
reach 0, and then asserts the gate does not re-open, appends no row, and that the agent step
**behind** it actually dispatches. `a_voided_gate_step_re_opens_at_the_next_reach` is its control:
without it, "an answered gate is walked past" is satisfied by an arm that never opens twice under
any circumstance, which breaks AC-252 instead.

Clause 4's guard is `catalog_diagnostic::adr0146_gate_question_may_not_carry_a_colon`, over the
**strict** loader `run` uses, with a positive control asserting a colon-free `question` still loads
**and is still read** — so a loader that refused every `question`, or stopped parsing it to make the
test pass, is red.

**Mutant-tested** in a byte-faithful export (`git -c core.autocrlf=false clone --no-hardlinks`, with
fidelity verified rather than assumed): removing the three-way question restores the unconditional
mint and turns the guards RED.

**Confidence: high on the defect and the mechanism.** The defect was found by an adversarial pass
driving the **real binary** over three run→answer rounds, not by reading the arm — which is what
ADR-0145's own evidence section conceded was missing when it said no test spawns the binary over a
pack with a gate step. That concession was written the same day and was the right one.
