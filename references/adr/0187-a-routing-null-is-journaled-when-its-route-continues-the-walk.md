---
id: "0187"
title: "**A routing `null` is journaled when its route continues the walk; a `null` routed to a refusal is not** (`s8e`'s record of D21-92): a `null` `repeat` `until` or `gate` step `when` is a control decision's row on the same rule as a `true` or a `false` — written when its route continues the walk (another pass, a `continue` or an escalation at `max`, a gate walked past), with `\"holds\": null`, and replayed on re-entry as the route the `null` took; a `null` `until` exhausted into a refusal leaves no row, so a refused-then-fixed re-run still completes (D21-77). `Decision::holds` becomes the predicate's three-valued `Tri`. Supersedes in part `0174` decision 7 — its D2177-8d `null` bullet and the matching clause of its D21-77 forbids bullet — and discharges that bullet's named residual"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-92 (*"A `null` `until` or gate `when` is journaled on the same rule as a `true` or `false`: **when its route continues the walk**. A `null` routed to a refusal leaves no row, so a refused-then-fixed re-run still completes (D21-77). Built red-first with mutants in `s8e`, stated in a new record written `proposed` that supersedes ADR-0174 decision 7 in part (its null sentence only)"*) and D21-77 ruling 1; runbook/phase-1/s8d-step-grain-and-resolver.md §8d.13 (the question owed to the operator); runbook/phase-1/s8e-tier-one-and-the-lane.md §8e.12 (the tests and mutants); references/adr/0174 decision 7 (the sentences superseded in part, quoted in Decision clause 4); crates/rk-kernel/src/runtime/step.rs (`one_step`, `after_step`, `run_step`), crates/rk-kernel/src/runtime/activity.rs (`Decision`, `decision_event`, `journaled_decision`) and crates/rk-kernel/tests/it/on_false_exec.rs, as built 2026-09-28 on `da5dff8`
supersedes: ["0174"]
superseded_by: []
related: ["0031", "0103", "0173", "0175"]
evidence: []
consumers: []
---

## Context and problem

**A `null` `until` or gate `when` routes the walk, and it was never journaled.** ADR-0174 decision 7 (accepted,
D21-81) made every control decision whose route continues the walk a journaled row that a re-entry replays, and its
D2177-8d paragraph set the `null` apart: *"A `null` is not a decision, for all three kinds. A `null` `check` refuses by
name as an abstention and is not journaled (unchanged). A `null` `until` takes another pass, or at `max` its
`on_exhausted`, as before, and a `null` gate `when` skips the gate, as before. Neither is journaled now, so a later
walk evaluates it again."* The same paragraph named what that costs: *"a re-entered walk decides fresh WHEREVER nothing
is journaled, and that includes a `null` `until` or gate `when` that sits before a later journaled decision (a `null`
`until` at `repeat:1` under a journaled `true` at `repeat:2`). Re-evaluated over re-executed tool output that is no
longer `null`, it can route differently from the first walk before the last journaled decision (end the repeat at pass
1, or ask a gate the first walk skipped)."* It closed: *"No ruling says whether a `null` that routes the walk should be
journaled; the question is owed to the operator."*

`s8d` §8d.13 carried the question into the ledger: *"A `null` `until` or gate `when` is never journaled. A re-entry
therefore re-evaluates it even when it sits before a later journaled decision, and over re-executed tool output it can
route differently there."* D21-92 answers it.

Two facts of the tree shape the build.

- **The row's verdict was a `bool`.** `Decision { kind, holds: bool }` and `"holds": <bool>` could say only `true` or
  `false`. The D21-76 build had journaled a `null` `until` as `false` (§8d's red line: *"("g", "repeat:1", Object
  {"kind": String("until"), "holds": Bool(false)})"*), which routes the same way but records an answer the predicate
  did not give. D2177-8d removed that row rather than make it truthful.
- **A `null` `until` and a `false` one route identically, and so do a `null` gate `when` and a `false` one.** Each
  takes another pass (or at `max` its `on_exhausted`), and each skips the gate. So the route a `null` took can be
  replayed exactly without re-evaluating it, and a gate `when` never routes to a refusal: a skipped gate always
  continues the walk.

## Decision drivers

- D21-92 in terms: journaled *"when its route continues the walk"*; *"a `null` routed to a refusal leaves no row"*.
- D21-77 ruling 1, which D21-92 keeps: a refusal is never replayed, so a refused-then-fixed re-run completes.
- D21-76: *"up to the last journaled decision it neither re-executes the deciding `check` nor re-decides a loop"* — the
  residual is a hole in exactly that promise.
- ADR-0103 / ADR-0089: a `null` is an abstention and is never the same as `false`, so a journal row must not record
  one as the other.
- ADR-0174 decision 7's at-most-once rule per commit key and kind, which the new row must honour unchanged.

## Considered options

### Option — journal the `null` as `"holds": null`, on the continuing-route rule (taken)

`Decision::holds` becomes `predicate::Tri`, the row renders it as JSON `true`, `false` or `null`, and
`journaled_decision` reads all three back. The walk writes a `null` `until` exactly where it would write a `false`
one (another pass, or at `max` a `continue` or an escalation to a gate that asks its question) and a `null` gate
`when` wherever the gate is walked past; a replayed `null` re-takes that route. Chosen: it is D21-92's rule in terms,
the row says what the predicate answered, and the replay is exact because the route of a `null` is the route of a
`false`.

### Option — journal the `null` as `"holds": false`

What the D21-76 build did. Rejected: the route replays correctly, but the journal records a `false` the predicate did
not answer, which ADR-0089's abstention rule and ADR-0103's three-valued logic exist to prevent, and a later reader
(`explain`, a replay comparison) could not tell an abstention from a decision.

### Option — keep the `null` unjournaled and name the residual

ADR-0174's D2177-8d paragraph as written. Rejected by D21-92, which the operator ruled on this card.

### Option — journal every `null`, a refusal-routed one included

Rejected by D21-92 (*"A `null` routed to a refusal leaves no row"*) and for D21-77's reason: the journaled `null`
would be replayed into the same refusal on every later run, so a tree fixed between runs could never complete.

## Decision

**1. A `null` `until` or gate `when` is journaled when its route continues the walk.** `Runtime::commit_decision`
writes it as one more control decision's row, `decision = {"kind": "until" | "when", "holds": null}`, under the same
commit key and with the same attempt rule as a `true` or `false` (ADR-0174 decisions 4 and 7, unchanged). A `null`
`until` continues when it takes another pass, or at `max` when its `on_exhausted` is `continue` or an escalation to a
later gate that asks its question. A `null` gate `when` always continues: the gate is walked past.

**2. A `null` routed to a refusal leaves no row.** A `null` `until` at `max` whose `on_exhausted` is `fail`, absent,
an `escalate` with no gate to proceed to, or any other value refuses by name and writes nothing, as a `false` does
(D21-77 ruling 1). The next run evaluates it fresh over re-executed tool output, so a tree fixed between runs
completes. A `check`'s `null` is unchanged: it abstains and refuses by name, and no `check` row is ever `null`.

**3. A journaled `null` is replayed as the route it took.** On re-entry `journaled_decision` answers `Tri::Null` for a
`"holds": null` row. A replayed `null` `until` takes another pass (or at `max` its `on_exhausted`), and a replayed
`null` or `false` gate `when` walks past the gate, each without evaluating the predicate. A `holds` member that is
none of `true`, `false` and `null` is not read as a decision, and the walk decides fresh there. The type is
`Decision { kind, holds: Tri }`; `StepSite::guard`, the replayed guard, is `Option<Tri>`.

**4. Supersessions.** This record supersedes **ADR-0174 decision 7 in part** — its null sentence only, in two places,
each quoted:
- the D2177-8d paragraph's bullet *"**A `null` is not a decision, for all three kinds.** A `null` `check` refuses by
  name as an abstention and is not journaled (unchanged). A `null` `until` takes another pass, or at `max` its
  `on_exhausted`, as before, and a `null` gate `when` skips the gate, as before. Neither is journaled now, so a later
  walk evaluates it again."* — its `check` sentence stands; the `until` and `when` sentences are replaced by decisions
  1–3; and the residual the bullet names (*"No ruling says whether a `null` that routes the walk should be journaled;
  the question is owed to the operator"*) is **discharged**;
- the same paragraph's replay sentence *"and journals what it decided when its route continues the walk and it is not
  `null`"*, whose *"and it is not `null`"* no longer holds;
- and the matching clause of its dated forbids bullet, *"Forbids journaling a `false` whose route is a refusal, or a
  `null`"*, at *"or a `null`"*, which becomes decision 2's narrower rule (a `null` routed to a refusal).

Every other sentence of ADR-0174 stands, decision 7's D21-77 rule for a `false` included. **ADR-0174's
`superseded_by` back-link and the README Supersessions-table row move in this record's flip commit**, not with the
proposed record, as ADR-0180's, ADR-0181's and ADR-0183's did.

**5. Design §5's `activity_committed` row** takes a dated in-place amendment under this record, as ADR-0174 and
ADR-0176 amended it: a decision's `holds` may be `null`, for an `until` or `when` whose route continued.

## Consequences

**Positive**
- The walk's replay promise has no `null`-shaped hole: up to the last journaled decision, a re-entry re-decides
  nothing, a `null` included.
- The journal records the predicate's own answer; an abstention is never written as a `false`.

**Negative**
- One more decision row per continuing `null`, each taking an attempt (drawing nothing from AC-029's cap, ADR-0174
  decision 7's D2177-8d paragraph).
- A reader of `decision.holds` must now handle `null`; `journaled_decision` does, and no other reader interprets it
  (the fold skips decision rows; the breaker and `explain` never read them).

**What this forbids**
- Leaving unjournaled a `null` `until` or gate `when` whose route continued the walk.
- Journaling a `null` `until` whose route is a refusal.
- Recording a `null` decision as `false` (or as `true`).
- Re-evaluating, on re-entry, a `null` `until` or gate `when` journaled under the key the walk reaches.

## Evidence and confidence

**Measured** (2026-09-28, `s8e` build, package S8E-RULINGS, on `da5dff8` plus the stage's uncommitted work), through
the no-dispatch helper: `on_false_exec::a_null_until_or_gate_when_whose_route_continues_is_journaled_and_replayed`
RED first for its stated reason (*"left: [("g", "repeat:2", Object {"kind": String("until"), "holds": Bool(true)})]
right: [("g", "repeat:1", Object {"kind": String("until"), "holds": Null}), …]"*), then green;
`on_false_exec::a_null_until_routed_to_a_refusal_is_not_journaled_so_a_fixed_re_run_completes` green at entry by design
(the refusal arm is behaviour that must not move) and RED under mutant S8E-M52. Mutants, each applied and restored
from Python with the file's sha256 verified: **S8E-M51a** (a continuing `null` `until` not journaled — the old rule)
and **S8E-M51b** (a continuing `null` gate `when` not journaled) red on the continuing test's `until` and `when`
arms; **S8E-M51c** (a journaled `null` read back as no decision) red on its re-entry assertion (*"left: (2, 3, …)
right: (2, 4, …)"*); **S8E-M52** (a refusal-routed `null` journaled) red on the refusal test (*"left: [("g",
"repeat:1", … "holds": Null})] right: []"*). All four killed. The whole `on_false_exec` module (21 tests) is green.
**Not measured:** no shipped activity has been walked to a `null` `until` or gate `when` (the shipped review stage is
unreachable, ADR-0156). This record is `proposed` and flips only on the operator's dated ledger line.
