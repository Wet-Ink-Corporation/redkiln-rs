---
id: "0154"
title: "A **recorded halt outranks a supplied successor**: `fsm::next_action`'s halt check moves above its `AdvanceStage` check, so a run whose plan is exhausted AND whose journal carries a refusal reports `Halt` rather than advancing a stage. The current order — `AdvanceStage` at 4, `Halt` at 5 — is harmless today only because `ctx.successor` is `None` on every real run; `s7h` supplies it, and from that commit a refused run would have ADVANCED, falsifying arm 5's own comment that it sits *\"BEFORE `Done`, so a refused run is not reported finished\"*. Nothing pins the present order: the halt assertion at `tests/it/fsm.rs:748-755` uses the NO-successor context, so the `halt + successor` combination has no fixture anywhere in the workspace — a fixture per function, not per branch"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/workflow/fsm.rs (the documented evaluation order at 1601-1613; the body's arm 4 at 1700-1707 and arm 5 at 1708-1713, whose comment states the intent this record restores; `NextAction` at 550-569); crates/rk-kernel/tests/it/fsm.rs (724-790 — the order sweep, which asserts `Halt` against `ctx_none` and `AdvanceStage` against `ctx_next`, never a context carrying both); crates/rk-kernel/src/replay.rs (`resume`'s `_ => break` at 1809, which is where a `Halt` lands); references/adr/0152 (which supplies the successor and therefore activates this)
supersedes: []
superseded_by: []
related: ["0022", "0152", "0153"]
evidence: []
consumers: []
---

## Context and problem

`fsm::next_action` writes its evaluation order out in a doc comment rather than leaving it to be
inferred (`fsm.rs:1601-1613`), and the body matches the comment:

1. an open gate neither answered nor void → `AwaitGate`
2. a selection gate the journal has not opened → `OpenGate`
3. an uncommitted activation in the plan's order → `Dispatch`
4. **the plan exhausted and a successor supplied → `AdvanceStage`**
5. **a recorded halt → `Halt`**
6. otherwise → `Done`

Arm 5 carries its own warrant (`fsm.rs:1708-1710`):

> 5. A recorded halt, at design §3.2's own position in the order — AFTER the plan, so a run with work
>    still dispatchable does the work, and BEFORE `Done`, so a refused run is not reported finished.

The comment names two neighbours and is right about both. It does not mention arm 4, which sits between
them — and **arm 4 returns unconditionally whenever `ctx.successor.is_some()`** (`fsm.rs:1702-1707`):

```rust
if let Some(to) = ctx.successor {
    return Ok(NextAction::AdvanceStage {
        from: state.stage.clone(),
        to: to.to_owned(),
    });
}
```

So for a run with an exhausted plan, a recorded refusal **and** a successor, `next_action` returns
`AdvanceStage`. The halt is never reached and never reported.

**This has been latent and harmless for exactly one reason: `ctx.successor` is `None` on every real
run.** `base_inputs` never chains `advances_to`, so no production path has ever put a successor in the
context. `0152` supplies one. From the commit that lands it, a refused run advances a stage — the precise
outcome arm 5's comment says the order exists to prevent.

The gap is not only in the code. **Nothing in the workspace instruments the combination.** The order
sweep at `tests/it/fsm.rs:724-790` is careful and still misses it: it asserts `AdvanceStage` against
`ctx_next` (a context *with* a successor), then asserts `Halt` against `ctx_none` (a context *without*
one), and adds a control that the same exhausted journal without the refusal is `Done`. Every arm has a
fixture; the arm *interaction* has none. A halted journal has never been folded against a context
carrying a successor.

## Decision drivers

- Arm 5's comment states the invariant explicitly. The repair is to make the code match a sentence that
  is already in the file, not to introduce a new rule.
- A refused run reported as advancing is the worst available failure: it is silent, it writes a tier-1
  `stage_transition`, and the next stage then starts from a journal whose refusal nothing acted on.
- `Halt`'s position relative to the *plan* must not change. Arm 5's other neighbour claim — *"AFTER the
  plan, so a run with work still dispatchable does the work"* — is correct and is load-bearing for
  AC-013 re-entry; moving the halt above `Dispatch` would starve a resumable run.
- The absence of a fixture for the combination is itself the finding. Whichever repair is chosen, the
  test has to be on the pair, or the next reordering is invisible again.
- `0022` already fixes that the kernel takes no action on an open gate until it is answered; `AwaitGate`
  and `OpenGate` must stay above everything, halt included.

## Considered options

**1. Gate arm 4 on `state.halt.is_none()`.** One conjunct added to the `if let`. It works, and it leaves
the documented order saying `AdvanceStage` is fourth while its real precondition has quietly become
"fourth, unless halted" — the doc comment and the body drift apart again, which is the shape that
produced this defect.

**2. Move the halt check above the successor check (chosen).** Arm 5 becomes arm 4 and vice versa; the
doc comment's numbered list is updated to match. `Halt` stays after the plan and before `Done`, so both
of arm 5's own neighbour claims remain true, and the third — the one it did not make — becomes true as
well.

**3. Refuse the combination in `resume` instead.** Let the FSM return `AdvanceStage` and have the caller
notice the halt. This puts a state-machine precedence rule outside the state machine, where `record`'s
twin loop and any future caller would each have to re-implement it.

## Decision

**1.** In `fsm::next_action`, the `state.halt` check is evaluated **before** the `ctx.successor` check.
The resulting order is: `AwaitGate`, `OpenGate`, `Dispatch`, **`Halt`**, **`AdvanceStage`**, `Done`.

**2.** The doc comment's numbered list (`fsm.rs:1601-1613`) is renumbered with the body, in the same
commit. The order **is** the answer, so the two are never allowed to disagree.

**3.** Arm `Halt`'s comment keeps both of its existing claims and gains the third: a supplied successor
does not outrank a recorded refusal.

**4.** The test is on the **combination** — one exhausted journal carrying a `step_refused` row, folded
against a context that **does** supply a successor, asserting `Halt`. The two existing single-arm
assertions stay; this is added beside them, not instead of them.

## Consequences

`NextAction::AdvanceStage` becomes unreachable for a halted run, which is what `0152` needs in order to
supply a successor safely. No variant is added, removed or renumbered; no event kind changes; no exit row
moves.

A run that halts still reaches `resume`'s `_ => break` (`replay.rs:1809`) and still leaves through
`terminal_read`, which maps a no-gate outcome to `Exit::Success`. **This record does not change that**,
and the residual is named below rather than left to be discovered: after this change a halt is reported
*as a halt by the FSM* and is still *indistinguishable from a clean finish at the process boundary*.
Those are two different defects and only the first is this record's.

**What this forbids**

- Arm `AdvanceStage` returning while `state.halt.is_some()`.
- Moving the halt check above `Dispatch` or above either gate arm.
- Encoding the precedence in `replay`, `record` or any caller rather than in `next_action`.
- Letting `next_action`'s doc-comment order differ from its body's order.
- Asserting this repair with a fixture that supplies no successor — the no-successor context cannot
  distinguish the old order from the new one.

## Evidence and confidence

**Both halves are quoted, not summarised.** Arm 4's body is reproduced above from `fsm.rs:1702-1707`;
arm 5's comment from `:1708-1710`. The unconditional `return` inside arm 4 is the whole mechanism — there
is no other guard on it, and `ctx.successor` is its only input besides `state.stage`.

**The missing fixture was measured, not inferred.** `grep -rn "advances_to" crates/rk-kernel/tests/`
returns two sites: `tests/it/resume.rs:158` and `tests/it/replay.rs:314`, both inside builders for gated
runs that break at a gate before the plan is exhausted. In `tests/it/fsm.rs` the halt assertion
(`:748-755`) passes `ctx_none`; the advance assertion (`:729`) passes `ctx_next`; no third context exists.
So no test in the workspace folds a refusal against a supplied successor, and the reorder is therefore
**not** pinned by any current green — which is exactly why the new test is part of the decision rather
than an afterthought.

**The activation date is precise.** `ctx.successor` is `None` on every production run at `fc45eeb`
because `base_inputs` (`rk-cli/src/verbs/run.rs:1047-1080`) is the only production builder of
`RuntimeContext` and does not chain `advances_to`. The defect is therefore unobservable today and becomes
observable in the commit that lands `0152` — a sequencing fact, which is why this record is written
before that commit rather than after it.

**Confidence, and what is NOT asserted.** `HaltReason::Refused` from a `step_refused` row is the only
halt this crate can fold out of a journal; AC-024's budget exhaustion and AC-033's breaker refusal have
no `EventKind` at all, and both `budget.rs` and `breaker.rs` already report that ARMED. So this record
improves the precedence of the one halt that is readable and says nothing about the two that are not. It
also asserts nothing about the exit code a halted run produces — see Consequences.
