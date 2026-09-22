---
id: "0136"
title: "A step's `when` guard is evaluated at execution time through `predicate::evaluate`, and a guard carrying a `${…}` reference **refuses the step by name** rather than being nulled out — `rigor.rs`'s `null_out_refs` is a STATIC lint's treatment and reproducing it at runtime would run or skip a step on an invented value; **54 of the shipped pack's 121 step guards carry such a reference**, and the reference resolver that would evaluate them is OWED and named here"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/2026-09-08-architecture-design.md §4.5 (the five closed step kinds); references/adr/0103 (the predicate grammar, three-valued connectives, `on_false`); references/adr/0032 (absent optional inputs are `null`); crates/rk-kernel/src/predicate/eval.rs:157-160 (`EvalError::UnresolvedRef` — *"A `${…}` reference reached the evaluator. Reference resolution is the selection function's step 2 … nothing in this slice resolves one."*); crates/rk-kernel/src/rigor.rs:69 and :405-422 (`null_out_refs`, and why the static lint must null); runbook/phase-1/s7g-step-executor.md §7g.1 (*"`when` consulted per step"*)
supersedes: []
superseded_by: []
related: ["0032", "0044", "0055", "0079", "0103"]
evidence: []
consumers: []
---

## Context and problem

`s7g` §7g.1 makes the step the unit of execution, *"in declaration order, with `when` consulted per step"*.
Consulting `when` at execution time requires evaluating it, and **45% of the shipped pack's guards cannot be
evaluated on this tree.**

Measured 2026-09-22 over `.redkiln/catalog/`: **121 step-level `when` clauses, of which 54 carry a `${…}`
reference** — `${bisect.outputs.status} != 'found'`, `${approve.answer} == 'proceed'`,
`${audit.outputs.gaps}.len() > 0`, and 51 more.

**Nothing on this tree resolves such a reference.** `predicate::eval`'s own error variant says so in terms
(`eval.rs:157-160`): *"A `${…}` reference reached the evaluator. Reference resolution is the selection
function's step 2 … **nothing in this slice resolves one**."* The evaluator takes a parsed `Expr` and a
`Context` of bindings; no code path populates a `Context` from a prior step's outputs, because until `s7g`
no prior step ever ran.

**There is exactly one existing consumer of `when`, and it is a lint.** `rigor.rs:405-422` compiles each
guard and applies **`null_out_refs`** before evaluating, then skips a step's evidence contribution when the
guard is not `Tri::True`. Its own docstring says why (`rigor.rs:69`): a step guarded by
`when = "${bisect.outputs.status} != 'found'"` **is not statically decidable**, so every `${…}` reference is
nulled and the rigor ladder reasons about what is knowable ahead of a run.

**That treatment is correct for a static lint and wrong for an executor.** Nulling a reference at
*execution* time does not express uncertainty — it manufactures a value. Under ADR-0103's three-valued
connectives a nulled comparison yields `Tri::Null`, which is not `Tri::True`, so **every one of the 54
guarded steps would be silently skipped** on the grounds of a `null` the evaluator invented. A run would
report success having executed a subset of the activity chosen by an artefact of the lint's approximation.

**An unresolved reference is not the same as an absent value, and the distinction is already in the type
system.** ADR-0032 makes an absent optional input `null`, and `Tri::Null` routes to AC-079's abstention gate
through `Outcome::abstention_axes()`. That is a *known* absence with a decided handling. `UnresolvedRef` is
an `EvalError` — a *failure to ask the question at all*. Collapsing the second into the first would file a
missing mechanism under a decided policy.

## Decision drivers

- A guard that cannot be evaluated must not be allowed to decide anything. Skipping on an invented `null`
  and running on an invented `true` are the same defect with opposite signs.
- The 54 affected guards are not an edge: they are 45% of the pack's step guards and they sit on the
  loop-and-retry steps (`!= 'found'`, `!= 'passing'`, `!= 'reproduced'`) where a wrong skip is least visible.
- Building the reference resolver is a genuinely larger piece of work than `s7g`: it needs an output-binding
  model that `emit` does not yet have, a name resolution over prior steps in the same activation, and the
  `for_each` / `repeat` iteration semantics that `catalog.rs:166-167` explicitly leaves unparsed
  (*"evaluating it is the runtime's"*). Folding it into this stage would decide all of that implicitly.
- `s7g`'s value does not depend on it. The 67 ref-free guards evaluate; `toolchain:test` — the largest verb
  and criterion (1)'s path — is reached through steps whose guards are ref-free or absent.

## Considered options

1. **Evaluate ref-free guards; refuse a step whose guard carries an unresolved reference (taken).**
2. **Null out references at runtime, as `rigor.rs` does.** Rejected: it silently skips 54 guarded steps on a
   value the evaluator invented, and it does so in the shape that reads exactly like correct behaviour — a
   run that exits 0 having executed less than the activity declares.
3. **Treat an unresolved reference as `Tri::Null` and route it to AC-079's abstention gate.** Rejected, and
   this is the near-miss worth naming: abstention is a decided policy for a value that is *known absent*
   (ADR-0032), and routing a *missing mechanism* through it would make the gate fire 54 times per run for a
   reason no operator can act on, while recording the kernel as having asked a question it never asked.
4. **Build the reference resolver in `s7g`.** Rejected **as scope, not as direction** — it is the right end
   state and clause 4 below names it as owed. It requires the output-binding model, prior-step name
   resolution, and `for_each`/`repeat` iteration semantics, none of which any record fixes today.
5. **Refuse the whole activity when any of its steps carries an unresolvable guard.** Rejected: it is
   coarser than the evidence supports. A ref-free step in the same activity is evaluable and should run; the
   refusal belongs at the grain of the thing that cannot be decided.

## Decision

1. **A step's `when` is evaluated at execution time through the existing evaluator** — `predicate::parse`
   then `predicate::evaluate(&expr, &ctx)` — and **not through a second evaluator written for the runtime**.
   `when` and `expr` already share one evaluator; what differs is only who builds the `Context`.

2. **A guard evaluating to `Tri::True` runs the step. Anything else — `Tri::False` or `Tri::Null` — does
   not.** `Tri::Null` keeps its decided handling: it is an abstention under ADR-0032 and AC-079, and this
   record does not re-decide it.

3. **A guard whose evaluation returns `EvalError::UnresolvedRef` refuses the step**, with a typed error
   naming **the step id and the reference that could not be resolved**. The refusal is an outcome, not a
   panic: it is the step's result and the surrounding machinery treats it as it treats any other
   `StepError`. **`null_out_refs` is not called on the execution path**, and the prohibition is in this
   record because the function is `pub(crate)`, one import away, and superficially the obvious reuse.

4. **The reference resolver is OWED and is named here rather than left implicit.** What it must supply: a
   `Context` populated from the outputs of prior steps in the same activation, under a name resolution that
   agrees with the grammar ADR-0103 fixes and with ADR-0055's rule that a self-reference inside a loop binds
   the **previous** iteration. It cannot be built without an output-binding model — which is the `emit`
   arm's subject — and without `for_each` / `repeat.until` iteration semantics, which `catalog.rs` parses
   and deliberately does not evaluate. **It is not `s7g`'s**, and no stage in this runbook currently claims
   it.

5. **The refusal count is expected to be large on the shipped pack, and that is the measurement, not a
   regression.** 54 of 121 guards carry a reference. A run that reaches those steps will refuse them by
   name, which is the first time this tree will have said out loud which guards it cannot decide.

**What this record does NOT decide.** It does not design the reference resolver, its name resolution or its
iteration semantics. It does not change `rigor.rs`'s static treatment, which stays correct for a lint. It
does not re-decide `Tri::Null`'s routing to AC-079. It does not decide what a `check` step's `expr` does
with a reference — `expr` is the `check` arm's and is evaluated over a context that record does not fix
here.

## Consequences

**What this forbids**

- Forbids calling `null_out_refs`, or any equivalent substitution, on the **execution** path. Nulling a
  reference at runtime invents the value the guard turns on.
- Forbids treating `EvalError::UnresolvedRef` as `Tri::Null`, as `Tri::False`, or as a skip. It is a failure
  to ask the question, not an answer to it.
- Forbids running a step whose guard did not evaluate to `Tri::True`, including on the reasoning that a
  refusing guard is "probably" satisfied.
- Forbids a refusal that does not name both the step id and the unresolved reference. A refusal whose text
  does not say what could not be resolved is indistinguishable from a bug in the evaluator.
- Forbids extending the executor with an ad-hoc partial resolver — a lookup for `${x.answer}` but not
  `${x.outputs.y}`, say. The resolver is one mechanism with one record, or it is absent.

**What follows**

- 67 of the pack's 121 step guards evaluate at execution time; **54 refuse by name**, and the pack's
  reliance on unresolvable guards becomes a measured figure instead of an assumption.
- The `emit` arm's output-binding model acquires a named consumer: the resolver cannot be built before it.
- A stage is owed for the resolver. Until it exists, activities whose control flow depends on a prior step's
  output cannot complete, and that limit is now recorded rather than discovered at the next criterion.

## Evidence and confidence

- [measured 2026-09-22] `.redkiln/catalog/`: **121** step-level `when =` lines; **54** of them contain
  `${`. The referencing forms observed include `${<step>.outputs.<field>}`, `${<step>.answer}`, the optional
  `${<step>.outputs.<field>?}` spelling, and `.len()` over a referenced list.
- [documented] `eval.rs:157-160`, `EvalError::UnresolvedRef`: *"A `${…}` reference reached the evaluator.
  Reference resolution is the selection function's step 2 … nothing in this slice resolves one."*
- [documented] `rigor.rs:69` — a step guarded by `when = "${bisect.outputs.status} != 'found'"` is not
  statically decidable, *"So every `${…}` reference…"* is nulled; `rigor.rs:405-422` is the implementation,
  and `rigor.rs:453-464` the consumer that skips a step whose guard is not `Tri::True`.
- [measured 2026-09-22] `predicate::evaluate` takes `(&Expr, &Context)` and no ports, no store and no
  journal; its only inputs are the parsed expression and a `BTreeMap` of bindings. Its call sites are
  `quality_block.rs`, `rigor.rs`, `select/step2.rs`, `select/step4.rs`, `select/step7.rs` and `rk-cli`'s
  `explain.rs` — **all selection- or lint-time, none in `runtime/`**.
- [documented] `catalog.rs:166-167` parses `repeat.max` and not `repeat.until`, stating *"evaluating it is
  the runtime's"* — the iteration semantics the resolver would need are unfixed by any record.
- **Confidence: high on the count and on the absence of a resolver, both of which are measured over named
  greps.** What is **not** measured is how many of the 54 guarded steps a real run actually reaches — that
  depends on selection and on which activities activate, and the first honest figure comes from the first
  run that executes steps. This record does not predict it.
