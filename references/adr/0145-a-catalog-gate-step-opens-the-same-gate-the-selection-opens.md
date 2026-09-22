---
id: "0145"
title: "A catalog `[[step]] kind = \"gate\"` **opens the same gate the selection-level path opens**: it mints `GateRecord::derive_reach` through `workflow::gate::reach` and appends `runtime::gate_opened_event`, the one token-bearing row, so the fold cannot tell the two producers apart and does not have to. Its `question` key becomes the gate's **kind** and is parsed by `catalog::Step` for the first time — until this record the loader dropped it, and so the pack's **80** gate steps had no identity to derive a token over. A gated activation does **not** commit and the run **exits 0** (design §4.5), which `replay::dispatch_through` must ask about **before** `committed()`, or the 15 activities that gate before dispatching are refused as failed dispatches"
status: accepted
date: 2026-09-22
legacy_id: none
source: design §4.5 (*"`gate` (the kernel writes the mark and **exits 0**, resuming into the same step)"*); references/adr/0022 (the kernel takes no action until a gate is answered); references/adr/0144 (its sibling on the `Failed` arm, and the precedent for a second carrier on `ActivityReport`); crates/rk-kernel/src/workflow/gate.rs (`reach` — *"the entry a loop executor calls"*, with no `src/` caller until now); crates/rk-kernel/src/workflow/fsm.rs (`GateRecord::derive_reach`; the `GateOpened` fold arm, which tells two producers apart by `tree_hash`/`deadline`); crates/rk-kernel/src/runtime.rs (`gate_opened_event`, and `Runtime::step`'s `OpenGate` arm); crates/rk-kernel/src/catalog.rs (the hand-written `[[step]]` loader); crates/rk-cli/src/verbs/run.rs (`terminal_read`, which already exits 0 on a token-bearing row)
supersedes: []
superseded_by: ["0146"]
related: ["0022", "0044", "0135", "0136", "0138", "0142", "0143", "0144"]
evidence: []
consumers: []
---

## Context and problem

design §4.5 fixes five step kinds and says what a `gate` does: *"the kernel writes the mark and
**exits 0**, resuming into the same step."* The walk implemented it as:

```rust
StepVerdict::Gate | StepVerdict::Emitted => break,
```

with a comment stating that the mark was *"written by `Runtime::step`'s `OpenGate` arm, and this
walk authors no second path."* **The second clause was true and the first was false.** `OpenGate` is
reached only from `fsm::next_action`'s step 2, which iterates `selection.gates()` — the abstention,
unsatisfiable and appetite gates, whose kind is the synthesised string `"{reason}:{axis}"`. Nothing
in the tree connected a catalog `gate` step to it.

So the pack's gate steps were a **different population** from the gates the runtime serves, and
nothing joined them. Measured over `.redkiln/catalog/`:

| reading | count |
| --- | --- |
| `gate` steps in the pack | **80**, every one declaring a `question`, over 45 distinct values |
| activities with a gate AFTER the first agent step — **silently skipped** | **55** |
| activities with a gate BEFORE it — given the `emit` refusal | **2** |
| activities with gates and **no** agent step at all | **13** |

The 2 and the 13 are worse than the 55. Their walk fell through to the *"no `agent` step produced a
value"* arm, whose message is about `emit` binding outputs to `provides` and about the reference
resolver ADR-0136 records as OWED — **a refusal naming a mechanism that has nothing to do with
gates.** An operator reading it would have gone looking in the wrong place.

**Four things blocked the repair, and each was a real absence rather than an oversight.**

1. **`catalog::Step` never parsed `question`.** `GateRecord::derive_reach` digests a `kind`;
   `QuestionRegistry::question_for` looks one up. A gate step that reached neither carried no
   identity at all.
2. **`step.rs` had no `tree_hash` and no `deadline`.** Both are `FsmContext`'s, and this crate may
   read neither a clock (AC-001) nor a tree (`workflow/gate.rs`'s header).
3. **`ActivityReport` had no gate carrier.** `mark()` is `Option<&RetryAtMark>` — AC-030's
   rate-limit mark, set only on the `WaitForReset` arm — and `opened()` is module-private.
4. **`dispatch_through` asks only `committed()`.** A gated activation does not commit, so all 15
   activities that gate before dispatching would be refused as failed dispatches.

## Decision drivers

1. **One gate mechanism, not two.** ADR-0022 makes the gate a kernel-owned durable mark and the
   trailer, answer, expiry, void and spent-token machinery is all keyed on `GateRecord`. A second
   path would need all of it again, and the two would drift.
2. **Nothing may be invented in this crate.** The kind comes from the catalog, the tree hash and
   deadline from the caller, the reach from the fold.
3. **A gate is neither a success nor a failure.** It cannot travel on `outcome` and it cannot travel
   on ADR-0144's `step_error`.
4. **The exit-0 path already exists and is already correct.** `terminal_read` finds the last
   token-bearing `gate_opened`, commits the `Gate:` trailer and returns `Exit::Success`. The repair
   should reach it, not reimplement it.

## Considered options

**(a) Refuse a catalog gate step by name, and record the join as OWED.** ADR-0135/0136/0143's house
pattern, small and honest. Rejected **because the mechanism was not actually missing** — `gate::reach`
exists, is documented as *"the entry a loop executor calls"*, and had no caller; `gate_opened_event`
exists; `terminal_read` exists. Refusing by name would have recorded as OWED a thing that was three
call sites away, and 80 steps would still do nothing.

**(b) Mint a second kind of gate row for step gates.** Rejected: `fsm::apply` already tells two
producers apart by shape, and a third shape would need its own fold arm, its own answer path and its
own expiry. The whole point of ADR-0022 is that there is one mark.

**(c) Derive the kind from the step id or the activity id.** Rejected: an answer bound to an invented
kind is bound to a gate the catalog never declared. `question` is the declared name and is what the
question registry already keys on.

**(d) Wire it fully: parse `question`, pass the context, mint the record, append the row, and teach
`dispatch_through` that a gate is an exit.** Taken.

## Decision

1. **`catalog::Step` gains `question: Option<String>`**, read by the hand-written loader for **every**
   kind — not only `gate` — because a key read only where it is expected is a key that goes missing
   silently everywhere else. It is transcribed and not interpreted, exactly as `role` is (ADR-0014):
   the question vocabulary is the pack's, so an unknown id is refused at resolution, not at load.
2. **`run_activity_steps` takes a `GateSite { state, tree_hash, deadline }`**, bundled on
   `Activation`'s precedent. All three are **passed** from `replay`'s loop, where `fsm_context`
   already built them one line above the dispatch. `execute_step`'s signature is unchanged: the arm
   still answers `StepVerdict::Gate` and the walk does the minting, because the walk is what holds
   the step.
3. **The `gate` arm mints through `workflow::gate::reach` and appends `runtime::gate_opened_event`.**
   The same function, the same row, the same token the selection-level path writes. *"This walk
   authors no second path"* was an intention before this record and is now a fact.
4. **A `gate` step with no `question` is refused BY NAME**, not defaulted. All 80 shipped steps
   declare one, so this guards a catalog that changes rather than a state the pack is in.
5. **`ActivityReport` gains `gate: Option<Digest>`**, with `with_gate` for a walk that had already
   dispatched and `gated` for one that had not. `appended` is **not** incremented: its contract is
   *"events THIS MODULE appended"* and the row is the walk's, counted in the journal delta
   `dispatch_through` reads back.
6. **`dispatch_through` asks `gate_opened()` BEFORE `committed()`** and returns the journal delta.
   Nothing else is needed: the fold sees the row, `awaiting_gate` answers, `next_action` returns
   `AwaitGate`, the resume loop breaks on a non-`Dispatch` action, and `terminal_read` exits 0.
7. **A gated activation does not commit**, and that is design §4.5's intent rather than a limitation
   worked around. The run resumes into the same step when the gate is answered.

## Consequences

**What this forbids**

- Forbids a `gate` arm that writes nothing. A step whose kind is `gate` either opens a gate or
  refuses by name; silently walking past one is what made 55 activities look like they worked.
- Forbids a second gate-mark shape. There is one `GateRecord` and one `gate_opened` producer path
  for the runtime, and `fsm::apply`'s two-producer discrimination stays a two-way question.
- Forbids deriving a gate kind from anything but the declared `question`. An answer bound to an
  invented kind is bound to a gate the catalog never declared.
- Forbids computing the tree hash or the deadline inside `rk-kernel`. Both are passed; AC-001 and
  `workflow/gate.rs`'s header already forbid it and this record does not create an exception.
- Forbids `dispatch_through` treating a gated activation as a failed dispatch. `committed()` is
  `false` for a gate **by design**, and asking it first is what refused the 15.
- Forbids counting the gate row in `ActivityReport::appended`, which is `runtime::activity`'s own
  reading and not the walk's.

**What follows**

🛑 **This is the first repair in `s7g` that makes something EXECUTABLE rather than honestly refused.**
A walk that reaches a gate now ends in `Exit::Success` with a `Gate:` trailer and an answerable
token, which is a path criterion (1) needs. It does **not** follow that any of the 102 activities
completes: the `check` arm still ends most walks (ADR-0142 clause 4 — 94 of 94 shipped `expr`s read a
prior step's output, and step-output resolution stays OWED), per-step `ActivitySpec`s stay OWED
(ADR-0143 clause 5), and the `${…}` resolver for the remaining 46 guards stays OWED (ADR-0136
clause 4).

**What it does not claim.** Nothing here asserts that the gate a step opens is the gate an operator
*should* be asked — `question_for` is not consulted, no question text is rendered, and the answer
path is unchanged. This record connects the step to the mark; the question registry's own join is
not this stage's.

## Evidence and confidence

**Measured, not argued.** Four tests in `rk-kernel/tests/it/gate_step.rs`, covering both populations
the pack actually contains: a gate **after** a committed dispatch (55 activities) and a gate
**before** any dispatch (15), which take different branches and would not both be fixed by either
alone. Each asserts that the row is the **token-bearing** one — `select()`'s announcement rows share
the `gate_opened` kind and carry no token, so a walk appending that shape would look like it worked
and gate nothing — and that the token on the report is the token in the journal, rather than two
independently derived values that happen to agree.

**Two controls, both live.** A gate whose `when` does not hold must open nothing and append nothing;
without it every assertion above is satisfied by an arm that fires unconditionally, and
`StepVerdict::Skipped` is reachable at all only because ADR-0142 landed. And a gate with no
`question` must refuse by name rather than default.

**🛑 What is NOT asserted, stated rather than left to be discovered: no test spawns the binary over a
pack whose activity carries a `gate` step.** The four guards enter the walk directly, as
`second_agent.rs` does. `rk-cli`'s existing gate tests — `ac403_gate_open_writes_trailer_and_event_…`
and `ac264_run_after_applied_answer_resumes_past_gate` — exercise the **selection-level** gate over
`pack-min`, whose `emit-report` activity declares no gate step, and they are unchanged by this record
(measured: `-p rk-cli` 105 passed, 0 failed, before and after). So the last link — a *step* gate
producing exit 0 and a `Gate:` trailer through the real composition root — is composed from parts each
of which is asserted, and is not itself observed. Closing it means a fixture activity with a gate step,
which is `rk-cli`'s to author and not this record's; ADR-0141 clause 4's behavioural half is the
precedent for what that would look like.

**Confidence: high on the mechanism, high on the counts, and explicitly lower on reachability.** The
55/2/13 figures are a structural reading of the catalog and model where a walk ends, not which guards
evaluate at run time. That no shipped activity is asserted to complete is stated above rather than
left for a later pass to discover.
