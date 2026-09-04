# E3 severity annotation — the MODEL's side (κ, METHOD-E3 §7)

**SEALED FOR BLINDING.** METHOD-E3 §7 computes Cohen's κ between an LLM judge
and **Ryan's blind annotations**. Reading this file before annotating
`e3-severity-annotation-ryan.md` forfeits the blind, and κ becomes meaningless.
Annotate first, then compare.

**Annotator:** Claude Opus 5, the same model that built the rig — which is
itself a threat to this annotation's independence and is stated as one.
**Date:** 2026-09-04. **Facts:** all six claims independently replicated (F24);
this file annotates SEVERITY only.

---

## The rubric is not my taste — it is the card's own verdict rule

`agents/redkiln-review.md` states the standard the reviewer is held to:

> `approved` — every acceptance criterion is genuinely satisfied by real,
> reachable behaviour, **with a real test backing each**, no escape hatch […]
>
> `changes-requested` — at least one criterion is **unmet, untested**, only
> superficially met, met via an escape hatch […]

and step 2 requires the reviewer to ask, per AC:

> tests (**is there a test that would fail if the behaviour regressed?**) […]
> hunt for unmet criteria, missing or **tautological tests**

**A test that survives mutation of the behaviour it names is, by this
definition, a test that would not fail if the behaviour regressed.** It is the
card's own "tautological test". So for these findings the severity call is not a
matter of judgement at all — it follows from the contract the reviewer was given.

That is worth stating plainly because §7 assumed severity would need a judge.
For five of six findings it does not.

---

## Annotations

| # | item | arm | finding | severity | basis |
| --- | --- | --- | --- | --- | --- |
| 1 | `fix-advance-write-lock-gate-scope` | a+b | `basis` flips `evaluated`→`memoized` | **BLOCKING** | live undocumented regression, reproduced end-to-end; changes recorded state that S0.2's reconcile reads |
| 2 | `fix-advance-write-lock-gate-scope` | b | refused entry gate stops rendering `produces` | **ADVISORY** | true and reproduced, but the new behaviour is arguably *more* correct — not writing artifacts for a stage never entered. A behaviour change worth documenting, not a defect |
| 3 | `issue-146-147-boundary-window` | b | AC-015's ordering test is vacuous | **BLOCKING** | AC-015's behaviour IS met; its test is tautological — passes with the ordering inverted. Card: "untested" |
| 4 | `issue-63-advance-gate-diagnostic-truncated` | a | AC-001 untested under its own precondition | **BLOCKING** | the mutation that survives is *the exact regression #63 existed to fix*. Card: a test that would not fail if the behaviour regressed |
| 5 | `issues-135-129-declared-boundary-parse` | a | `Path`-header guard unpinned; AC-013 pass-through unasserted | **BLOCKING** | guard replaceable with `if (false)`, 63/63 green. Card: "untested" |
| 6 | `issue-74-plan-briefs-dead-plan-anchor` | a | strictness claim unproven | **BLOCKING** | the code comment names the exact distinction (`=== true`, not `!== false`) that no test pins. Card: "untested" |

**Model's verdict: 5 BLOCKING, 1 ADVISORY.**

### Consequence if this annotation is adopted

Every CLEAN rejection by both arms rests on at least one BLOCKING finding:

- arm (a)'s four — `fix-advance` (#1), `issue-63` (#4), `issues-135-129` (#5),
  `issue-74` (#6) — all blocking → **FP 0/23**
- arm (b)'s two — `fix-advance` (#1), `issue-146-147` (#3) — both blocking →
  **FP 0/23**

Both arms reach **MCC +1.000** and become indistinguishable on every quality
axis, leaving the 1.72× cost difference as the only discriminator.

### The part I am least confident in

**#2 is the one real judgement call**, and I have annotated it against the
reviewer: the finding is true but the change looks like an improvement. If Ryan
disagrees, arm (b)'s rejection of `fix-advance` gains a second basis but its
verdict does not change — no rate moves. #2 is therefore the annotation with the
least riding on it, which is a poor property for the only item where a judge is
genuinely needed.

**The deeper issue this exposes** is not about severity at all: the historical
reviews that approved these five items were supposed to apply *this same card*.
Either they did not apply its stated standard, or the standard is stricter than
the process enforces in practice. E3's CLEAN ground truth encodes **what
happened**, not **what the card says should happen** — and those differ on at
least 5 of 23 items.
