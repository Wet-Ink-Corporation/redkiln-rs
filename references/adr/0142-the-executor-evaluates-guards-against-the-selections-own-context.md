---
id: "0142"
title: "A step's `when` guard is evaluated against **the selection's own context** — the item's facet bindings plus the effective rigor as an ordinal, built in the kernel from values `replay::resume` already holds and **not** transcribed onto `RecordedSpec`. Measured: the empty `Context` made **121 of 121** `when` guards and **94 of 94** `check` exprs fail on `UnknownField`; the selection's context takes `when` to **75 of 121 evaluable**, the remaining 46 keeping ADR-0136's refusal. It repairs **none** of the 94 `check` exprs, which read STEP OUTPUTS and need the resolver ADR-0136 already records as OWED"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0136 (the guard's refusal, and the reference resolver it records as OWED); references/adr/0020 (rigor as a deontic ordinal); references/adr/0089 and AC-079 (abstention; absence of evidence never resolves to `false`); crates/rk-kernel/src/predicate/eval.rs (the unbound-path arm returning `UnknownField`); crates/rk-kernel/src/select/item.rs (`Item::context`, the context vector *"as the evaluator sees it, with no narrowing"*); crates/rk-kernel/src/rigor.rs (`context_for`, the axis bindings plus `rigor` as an ordinal); crates/rk-kernel/src/select.rs (`Selection::rigor`, *"The rigor in force at this stage (step 1)"*); crates/rk-kernel/src/replay.rs (`resume`, which already destructures the item and runs `select`)
supersedes: []
superseded_by: []
related: ["0020", "0032", "0089", "0103", "0136", "0138", "0141"]
evidence: []
consumers: []
---

## Context and problem

`s7g`'s step executor evaluates every guard against `predicate::Context::new()` — a context that binds
**nothing**. `eval.rs`'s unbound-path arm returns `EvalError::UnknownField`, `execute_step` maps that to
`StepVerdict::Failed`, and the walk ends. Measured over the shipped pack:

| | with an EMPTY context | with the selection's context |
| --- | --- | --- |
| `when` guards (121) | **0 evaluable** | **75 evaluable**, 46 `UnresolvedRef` |
| `check` exprs (94) | **0 evaluable** | **0 evaluable**, 94 `UnknownField` |

**ADR-0136 named the wrong scope, and E-0136-1 has already corrected it.** That record presents the gap
as bounded by `${…}` reference resolution, which would be 54 guards. The real cause was the whole guard
language: **67 of the 67 ref-free guards read `rigor`** (plus `blast_radius` ×2 and `criticality` ×1),
so they failed before any reference was reached. Nine of the reference-bearing guards fail the same way,
because `rigor >= mandatory` is the left conjunct.

**The values were never missing — they were simply not carried to the executor.** Two definitions
already exist and are what selection itself evaluates against:

- `Item::context()` — *"The context vector as the evaluator sees it, with no narrowing"*, binding every
  facet the item carries.
- `rigor::context_for(bindings, rigor, registry)` — *"the axis bindings plus `rigor` as an ordinal over
  ADR-0020's five"*, which is the shape the 67 guards read.

And both are in scope where the walk is driven: `replay::resume` already destructures
`(pack, catalog, artifacts, facets, item)` from `inputs.build()` and runs `select()`, whose `Selection`
exposes `rigor()` — *"The rigor in force at this stage (step 1)"*.

**So this is a threading problem, not a modelling one**, and that is what makes it decidable now rather
than after a design pass.

## Decision drivers

- A guard must not decide anything on a value the executor invented. ADR-0136's prohibition on nulling
  references stands, and the same reasoning forbids inventing a facet.
- **The executor must see what selection saw.** A second, independently built context would mean a step
  could be guarded on one reading of the item while the activity was selected on another — a divergence
  no test would catch, because both would be internally consistent.
- The transcription route taken for the step list (ADR-0141) is available and is **not** wanted here: it
  would widen the run record, require `serde` on `predicate::Value`, and duplicate a value the kernel
  already holds. ADR-0138's own driver applies — *the need arrives as data* — and here the data has
  already arrived.
- The honest scope must be stated in the record, because 75 of 121 is a real improvement and 0 of 94 is a
  real limit, and a reader who took "guards now evaluate" as covering `check` would be badly misled.

## Considered options

1. **Build the context in the kernel from the item and the selection's rigor (taken).**
2. **Transcribe a context onto `RecordedSpec`, as ADR-0141 does for steps.** Rejected: the kernel already
   has both inputs, so this would carry a derived value through the run record for no gain, add `serde`
   to `predicate::Value` (including `Ordinal`'s domain, which is registry-derived and would then be
   pinned in every record), and create a second thing that can disagree with selection.
3. **Bind only `rigor`.** Rejected: it repairs the 67 that read `rigor` and leaves `blast_radius` and
   `criticality` failing for no reason, when the item's full facet context is one call away — and it
   would institutionalise a partial context that a later reader would have to re-derive the limits of.
4. **Null the unbound paths, as `rigor.rs` does for its static pass.** Rejected for ADR-0136's reason,
   unchanged: at execution time that manufactures the value the guard turns on. It is the same defect the
   record already forbids, one field short.
5. **Also resolve `check` exprs' step outputs.** Rejected as scope: those are 94 reads of `report.status`,
   `check.failed`, `findings` and their kin — prior steps' results, which need the output-binding model
   `emit` does not have and the resolver ADR-0136 records as OWED.

## Decision

1. **A step's `when` is evaluated against the selection's own context**: the item's facet bindings
   (`Item::context()`) with `rigor` bound as an ordinal over the registry's domain, exactly as
   `rigor::context_for` shapes it. **One definition, reused — not a second one written for the
   executor.**

2. **The context is built in the kernel, from values `replay::resume` already holds**, and threaded to
   the walk. It is **not** carried on `RecordedSpec`, not serialised into the run record, and
   `predicate::Value` gains no `serde` derive.

3. **The scope is measured and is stated as a limit, not a headline.** `when` goes from **0 to 75 of
   121** evaluable. The other **46 keep ADR-0136's refusal** — they carry `${…}` references and refuse
   by name, which is unchanged and correct.

4. **`check` steps are NOT repaired by this record, and the `check` arm continues to refuse.** All **94**
   `check` exprs read step outputs — `report.status` (22), `check.failed` (21), `findings` (11),
   `lint.errors` (4) and twenty more — none of which is a facet and none of which any context assembled
   from the item can supply. Binding them requires the reference resolver ADR-0136 clause 4 records as
   **OWED**, and this record does not build it.

5. **A guard that evaluates to `Tri::Null` keeps its decided handling**, unchanged: ADR-0032's absent
   optional, AC-079's abstention gate, and ADR-0089's rule that absence of evidence never resolves to
   `false`. This record adds no new meaning to a null.

**What this record does NOT decide, and does not make true.** It does not make any activity complete a
committing walk: `check` steps still refuse, and `runbook/PROGRESS.md` §1's Step 7g row records three
further blocking defects — the second-`agent`-step report overwrite, the swallowed post-commit failures
and the no-op `gate` arm — none of which this touches. **Step 7g remains BLOCKED.**

## Consequences

**What this forbids**

- Forbids evaluating a step guard against a context the executor assembled independently of selection.
  One definition; a divergence between the two would be invisible.
- Forbids binding a facet the registry does not carry, or inventing a value for one the item lacks. An
  absent facet is `Tri::Null` and AC-079's gate, not a default.
- Forbids calling `null_out_refs`, or any substitution, on the execution path. ADR-0136's prohibition is
  untouched and is not weakened by the context now being populated.
- Forbids reading this record as repairing `check` steps. All 94 still refuse, by measurement.
- Forbids re-deriving the effective rigor by a second route. It is `Selection::rigor()`, step 1's.

**What follows**

- 75 of the pack's 121 step guards decide for the first time, instead of ending their activation.
- The remaining 46 refuse **by name with their reference**, which is ADR-0136's intended behaviour and
  was previously masked: they were failing on `UnknownField("rigor")` before reaching the reference at
  all, so the refusal a reader saw named the wrong cause.
- The `check` arm's refusal becomes the dominant remaining reason a walk ends, which is the correct next
  thing to look at and is recorded as such.

## Evidence and confidence

- [measured 2026-09-22, over `.redkiln/catalog/`'s 102 activities, driving every guard through the real
  `predicate::parse` + `evaluate`] With `Context::new()`: **0** of 121 `when` and **0** of 94 `expr`
  evaluate. With a context binding every registered facet plus `rigor` as an ordinal: **75** `when` OK,
  **46** `UnresolvedRef`, **0** `expr` OK, **94** `expr` `UnknownField`.
- [measured 2026-09-22] The unresolved `expr` fields, by frequency: `report.status` 22, `check.failed`
  21, `findings` 11, `lint.errors` 4, `answer` 2, `cook.status` 2, `report.breached` 2, `report.failed`
  2, `smoke.status` 2, and a tail of singletons. **Every one is a prior step's output**, not a facet.
- [measured 2026-09-22] 67 of the 67 ref-free guards read `rigor`; `blast_radius` appears in 2 and
  `criticality` in 1, co-occurring with it.
- [documented] `Item::context()` and `rigor::context_for` are quoted in `source:` above; `Selection`
  exposes `rigor()` as *"The rigor in force at this stage (step 1)"*; `replay::resume` destructures the
  item and runs `select()` before driving the walk.
- **Confidence: high on the counts, which come from the real evaluator over the real pack rather than
  from a grep.** What is **not** measured is how many of the 75 evaluate to `True` on a real item — the
  probe bound each facet to a plausible in-domain value rather than to any particular item's, so the
  figure is *"can be decided"*, not *"will run"*. That distinction is the record's, and the first real
  figure comes from the first run that selects an activation.
