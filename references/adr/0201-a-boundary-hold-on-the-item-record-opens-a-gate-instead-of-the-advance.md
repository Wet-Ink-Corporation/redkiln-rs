---
id: "0201"
title: "**A boundary hold is human-authored on the item record and opens a gate instead of the advance** (`s8j`'s boundary-hold record; D21-42 19b; D21-40 decision 2; ADR-0129 decision 3; ADR-0183): the item record's key `hold_before = \"<stage>\"`; when `fsm::next_action` would return `AdvanceStage` into the held stage and no answer to that hold stands on the item, it opens a gate of kind `hold:<stage>` instead, answered by the default question; an answered hold is spent and never re-fires, `approve` releases the advance and `changes-requested` holds the lane at the boundary; `run` refuses at load, by name, a hold naming a stage the item's pack does not order; `hold:specification` is a `&str` constant with a fail-closed `[[gate_expiry]]` row in both packs. Supersedes nothing that this record could find, and invites refutation"
status: proposed
date: 2026-10-18
legacy_id: none
source: runbook/phase-1/s8j-run-preparation.md §8j.3 (the mechanism and the record's four clauses) and §8j.10 Checkpoint 8j (3); spikes/plans/s8j-build-plan-2026-10-16.md Round B (S8J-B2-HOLD) and Round C (S8J-C3-HOLD); spikes/DECISIONS-2026-09-21.md D21-40 decision 2, D21-42 19a, 19b and 21a, D21-43 N6, D21-46 (the launcher's `answer` mode), D21-47 (review finding 26); references/adr/0022, 0116 decision 5, 0123, 0128, 0129 decisions 1 and 3, 0152, 0153, 0154, 0157, 0161, 0183, 0198 decisions 6 and 7; references/2026-09-08-acceptance-criteria.md AC-039, AC-252, AC-276, AC-403, AC-518
supersedes: []
superseded_by: []
related: ["0022", "0123", "0128", "0129", "0152", "0153", "0154", "0157", "0161", "0183", "0198", "0200"]
evidence: []
consumers: []
---

## Context and problem

**The first real fix-lane run must stop at analysis → specification.** D21-42 19b: *"The first real fix-lane
invocation stops at the analysis → specification boundary for the operator to review."* No record decides how, and no
mechanism exists (`s8j` §8j.0 list item 2). `s8j` §8j.3 drafts one: a hold on the item record, a gate opened in place
of the advance, answered like any other gate.

**What exists, measured at the stage's entry commit `3fa69db`:**

- **The item record admits unknown keys.** `ItemRecord` in `crates/rk-cli/src/verbs/run.rs` derives `Deserialize`
  without `deny_unknown_fields` and declares `id`, `kind`, `stage`, `[facets]`, `slice` and `[trial]`. A `hold_before`
  key today is parsed and dropped, which is why `run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load`
  is red: nothing reads the key.
- **The advance is decided and executed inside the kernel.** `fsm::next_action`'s step 5 returns `AdvanceStage { from,
  to }` when the plan is exhausted, a successor is supplied, the lane has not entered it (`stages_entered`, ADR-0161)
  and it is not the fold's stage. `replay::resume` executes that arm, writing the `stage_transition` row and the
  stage-bearing `Story:` mark (ADR-0183), and breaks; `rk-cli`'s lane loop learns of it afterwards through `run_with`'s
  `advanced` out-parameter (ADR-0152). A check placed in the lane loop would run after the transition was written.
- **The successor is supplied, never invented**: the composition root resolves it from the pack's `[policy]
  stage_order` (ADR-0153; `Pack::successor` skips a `forbidden` stage). Both shipped packs order `triage, discovery,
  analysis, specification, design, planning, implement, verify, review, release, operate, closeout`.
- **The default question** is `approve` / `changes-requested`, neither attaching anything, and a gate kind no file binds
  resolves to it (ADR-0129 decision 3; `question.rs`: *"The default binding is the absence of one"*).
- **`gate_opened`'s payload is untyped.** `rk_types::event::Event`'s payload is `serde_json::Value`, provisionally and
  visibly, and `rk-types` carries no closed set of `gate_opened` reasons; the typed payload per kind is owed
  (`crates/rk-types/src/event.rs`). The kernel's own closed `workflow::gate::GateReason` is the gate *request*'s reason
  set (ADR-0199 decision 3, one value, `automated-verdict-refused`), and the landing and drift gates are minted by
  `GateRecord::derive_reach` under a `&str` kind constant without it.

**Four clauses are this record's** (`s8j` §8j.3): the item-record key, the gate cause, the rule that an answered hold
never re-fires, and the refusal at load of a hold naming a stage the item's pack does not order.

## Decision drivers

- **One `run` grammar** (ADR-0128: *"Forbids a second `run` grammar"*). The hold cannot be a flag or an environment
  variable `run` reads.
- **Stop at the boundary, not mid-stage**, and for this item only (D21-42 19b; 19a's dispatch cap is not a boundary).
- **`run` never waits** (golden constraint 2; AC-403: *"the process exits 0 without waiting on any channel, prompt,
  socket or sleep"*).
- **Nothing writes the item record** (ADR-0157's surviving forbids bullet; AC-518). The hold is human-authored in the
  checkout before the run.
- **A gate like any other**: git-first tier-1 transitions (D21-43 N6), the default question, a fail-closed expiry row
  (ADR-0198 decision 6), spent on answer (ADR-0123).
- **A halt outranks a successor** (ADR-0154), so a hold must not let a refused stage advance or mask a halt.

## Considered options

### Option — `hold_before` on the item record, consulted by the kernel's advance arm (taken)

Everything under `## Decision`.

### Option — a `run` flag, or an environment variable `run` reads

**Not taken.** ADR-0128 forbids a second `run` grammar, and an environment variable read by `run` is the same flag
under another name (`s8j` §8j.3).

### Option — a per-item dispatch cap (D21-42 19a)

**Not taken.** It stops wherever the count runs out, mid-stage and not at the boundary.

### Option — a pack-level gate at `specification`, or a `gate` step in the specification activity

**Not taken.** It would hold every fix-lane item for ever, not the one item whose first run is staged.

### Option — check the hold in `rk-cli`'s lane loop, after `resume` reports the advance

Where Round C's table first placed it (*"the lane loop opens the hold's gate instead of `AdvanceStage`"*). **Not
taken.** `resume` has already executed the `AdvanceStage` arm and written the `stage_transition` row and the
`Story: <project>/<slug> specification` mark before the lane loop sees `advanced` (ADR-0152, ADR-0183), so a hold
checked there would hold a stage the run ref says was entered. The hold is a supplied value, like the successor, and
the arm that decides the advance reads it.

### Option — a `[hold]` table (`[hold] before = "<stage>"`) or a list of held stages

**Not taken.** One key carrying one stage is all D21-42 19b asks for, and §8j.3's draft spells it `hold_before`. A
second held stage is a later item's need and a later record's widening.

## Decision

**1. The item-record key is `hold_before`**, a top-level string on `.redkiln/items/<id>.toml`, naming one stage:

```toml
hold_before = "specification"
```

It is optional; an item without it is unchanged. A value that is not a string is refused at load by name. The key is
human-authored before the run and is never written by any `redkiln` verb (AC-518).

**2. The gate cause: the advance into the held stage, while no answer to the hold stands.**

- **Where.** `rk-cli`'s composition root reads `hold_before` from the item record and supplies it to the kernel beside
  the successor (ADR-0153's route for supplied values). `fsm::next_action`'s step 5, at the point where it would return
  `AdvanceStage { from, to }` with `to` equal to the held stage, returns `OpenGate` for the hold's gate instead, unless
  an answer to that hold stands on the item (decision 3). Steps 1 to 4 are untouched, so an open gate, an unopened
  selection gate, an uncommitted node and a recorded halt each still come first (ADR-0154).
- **The gate.** Its kind is `hold:<stage>`, the held stage behind the cause: `hold:specification` for the reserved fix
  item. Its token is derived by `GateRecord::derive_reach` over the current tree hash and deadline, as every gate's
  is, and its `gate_opened` row is the runtime's three-member shape (`kind`, `tree_hash`, `deadline`), git-first tier 1
  (D21-43 N6). No `reason` member is added to any payload, so no closed reason set is widened: `rk-types` holds none
  for `gate_opened` [measured], and the kernel's `workflow::gate::GateReason` is the gate request's set, which the hold
  does not use. ADR-0116 decision 5's same-change rule, applied by analogy in §8j.3, therefore has nothing to move.
- **The question.** No file binds `hold:<stage>`, so the gate is answered by the default question, `approve` /
  `changes-requested` (ADR-0129 decision 3). The loader today would accept a pack file that binds a `hold:` kind, so
  this record forbids it (below) rather than leaving it to the loader.
- **What `run` does.** No `stage_transition` row and no stage-bearing `Story:` mark is written for the held stage; the
  run stops at the open gate and exits 0 without waiting (AC-403), as any run that stops at a gate does.

**3. An answered hold is spent and never re-fires.**

- **An answer stands** when the hold's latest gate on the item is answered and not void. While one stands, no second
  `hold:<stage>` gate is opened for that item, in this process or any later one: the state is folded from the journal,
  so it crosses the process boundary §8j.3 item 6 crosses on purpose.
- **A second answer** to the answered gate is refused as spent, one `gate_answer_spent` row (ADR-0123).
- **A void is not an answer that stood.** An answer whose tree hash differs from the gate's is void (AC-039), and a void
  is followed by a fresh re-open (AC-252), as for every gate; that is not a re-fire.
- **An expired hold** takes its row's `action = "fail"` (ADR-0198 decision 6): the item fails at the boundary and does
  not advance.
- **`approve` releases the advance.** The next `redkiln run <item>` (in `s8`, `tools/live-run.sh run`) folds the
  standing `approve`, takes the `AdvanceStage` into the held stage and enters it.
- **`changes-requested` holds the lane at the boundary** (D21-40 decision 2: stop and report). It stands too, so it
  never re-opens the gate; every later run stops at the boundary, reports the standing `changes-requested` by the
  hold's token, appends no `stage_transition`, and exits 0. Moving past it is a human act on the checkout (a new item,
  or the record's hold edited), never a verb's.

**4. A hold naming a stage the item's pack does not order is refused at load, by name.** In `run`'s load, beside the
gate-expiry lint and before any event row or dispatch, an item whose `hold_before` names a stage absent from its pack's
`[policy] stage_order` is refused with `Exit::ConfigError`, the message naming the item, the key, the stage and the
pack. Zero events are written. The test is `run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load`.

**5. The class is fail-closed and minted as a constant.** `hold:specification` is a `&str` constant on a code line under
`crates/*/src`, joins `rk-cli`'s `MINTED_GATE_KINDS` table so `run`'s load-time lint counts it reachable in every pack,
and takes one `[[gate_expiry]]` row, `action = "fail"`, in **both** shipped packs, because either pack's item can carry
a hold. The row is a draft until the operator signs it (D21-42 21a; AC-276). The spelling joins Checkpoint 8h's
`MINTED_CLASSES` by the same dated edit as ADR-0200's two classes; Checkpoint 8j (3)'s `S8J_GATE_CLASSES` already
carries it. A hold naming any other ordered stage mints `hold:<that stage>`, which has no row: `run`'s lint must count
the item's own `hold:<stage>` reachable, so such an item is refused at load for the missing row until a record adds the
constant and the rows.

**6. What this record supersedes: nothing that this record could find.** It was read against ADR-0128 (no grammar is
added), ADR-0152 (the lane loop and `resume`'s break are unchanged), ADR-0154 (a halt still outranks the advance),
ADR-0157's surviving forbids bullet and AC-518 (nothing writes the item record), ADR-0161 (`stages_entered` is
unchanged; a held stage is simply not entered yet), ADR-0183 (no mark is written for a stage not entered) and ADR-0129
decision 3 (the default question, unchanged). **This finding invites refutation**: a refuting round that finds a
Decision sentence or a forbids bullet the hold moves takes this record to a supersession, by ADR-0092 rule 1.

## Consequences

**Positive**

- The staged first run stops exactly at analysis → specification, for one item, with no new grammar, no wait and no
  write to the checkout, and the operator's answer crosses the process boundary through the journal.
- The hold is a gate like any other: the same lifecycle, the same answer path through the launcher's `answer` mode
  (D21-46), the same expiry discipline.

**Negative**

- **A misspelt key is silently ignored**, because `ItemRecord` admits unknown keys [measured]; `hold_befor =
  "specification"` would run the lane through the boundary. This record does not close it (closing it is a schema
  decision over every item record); Checkpoint 8j and §8j.3's first proof read the real item's hold.
- **A hold on a stage the lane never enters by an advance does nothing**: one at or before the item's entry stage, or
  on a stage the pack sets `forbidden` (which `Pack::successor` skips). Decision 4 refuses only an unordered stage, as
  §8j.3 drafts it. A refuting round may widen the refusal; this record names the gap rather than deciding it.
- **`changes-requested` has no in-system route forward**; the lane stays at the boundary until a human changes the
  checkout. That is D21-40 decision 2's rule, and its cost.
- **One more class needs a signed row** before the reserved item's first run.

**What this forbids**

- Forbids a `run` flag, subcommand or environment variable that holds a lane at a boundary.
- Forbids any `redkiln` verb writing `hold_before`, or any other key, to an item record (AC-518, restated for the key).
- Forbids writing a `stage_transition` row or a stage-bearing `Story:` mark for the held stage while no answer to its
  hold stands.
- Forbids opening a second `hold:<stage>` gate on an item whose answer to that hold stands, and forbids reading a void
  answer as standing.
- Forbids advancing into the held stage on a `changes-requested` answer.
- Forbids binding a `hold:` kind to a pack question; the hold is answered by the default question.
- Forbids accepting, at load, a hold naming a stage the item's pack does not order.
- Forbids minting a `hold:` class without its `&str` constant and its fail-closed row in both packs in the same change.

## Evidence and confidence

- [measured 2026-10-18 at `3fa69db`] `ItemRecord`'s fields and its missing `deny_unknown_fields`, `MINTED_GATE_KINDS`
  and `lint_gate_expiry` in `crates/rk-cli/src/verbs/run.rs`; `next_action`'s evaluation order and step 5's
  `stages_entered` guard in `crates/rk-kernel/src/workflow/fsm.rs`; `run_with`'s `advanced` out-parameter, set when
  `replay::resume` executed its `AdvanceStage` arm; `Pack::successor` in `select/pack.rs`; both packs' `stage_order`;
  `DEFAULT_OPTIONS` in `workflow/question.rs`; `Event`'s `serde_json::Value` payload in `rk-types`; `GateReason`'s one
  value in `workflow/gate.rs`; `LANDING_REFUSED_KIND` minted through `GateRecord::derive_reach` in
  `runtime/tier_one.rs`.
- [documented] D21-42 19a and 19b, D21-40 decision 2, D21-43 N6, ADR-0128, ADR-0129 decision 3, ADR-0152, ADR-0153,
  ADR-0154, ADR-0157, ADR-0161, ADR-0183, ADR-0198 decisions 6 and 7, AC-039, AC-252, AC-403, AC-518.
- [inferred] that no accepted Decision sentence or forbids bullet moves (decision 6); this is a reading over the records
  named, not an exhaustive search, and is stated as one.
- **Confidence.** High in the key, the gate's kind and the default question: each follows a pattern the tree already
  has. Moderate in the placement in `next_action`'s step 5, argued from where the transition is written; C3's tests and
  the three `lane_proof::boundary_hold_*` proofs are the first to exercise it.
