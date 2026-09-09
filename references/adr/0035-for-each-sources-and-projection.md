---
id: "0035"
title: for_each sources and fan-out output projection are fixed syntactic forms
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-5
source: references/2026-09-07-activity-graph-schema.md §9a (amendment 5)
supersedes: []
superseded_by: []
related: ["0053", "0068"]
evidence: ["P9 spikes/F32-DRAFT.md §4"]
consumers: []
---

## Context and problem

§1 defines fan-out as `for_each = "${step.outputs.list}"` and says "a `for_each` step's outputs are a list", and §9 lint rule 7 requires `for_each` sources to be list-typed with outputs consumed only as lists — but neither section enumerates what forms a `for_each` source string may take, nor how a downstream step projects a field out of a fan-out step's list of outputs. The `implement-slice` fan-out example in §7 uses `for_each = "${plan.outputs.slices}"` but that is one instance, not a grammar. Authoring ten activities against the schema needed both settled.

## Decision drivers

- Lint rule 7 needs a decidable notion of "list-typed" source to check against; an unenumerated source grammar cannot be linted.
- The schema commits to a closed-vocabulary expression type in §8, and the same P9 batch fixed the input-source grammar to a closed list (`artifact:Kind[.field]`, `kb:`, `checklist:`, `toolchain:`, `${…}`, literals — §9a.10, ADR 0040); fan-out sources and projections needed to fit that same closed-syntax discipline rather than being ad hoc.
- Ten activities authored against the schema (spike P9) forced the question in practice.

## Considered options

### Option — do nothing (leave source and projection forms to convention)
§1's `for_each = "${step.outputs.list}"` line and §7's `implement-slice` example would remain the only guidance, and lint rule 7's "list-typed" requirement would have no enumerated set of source forms to check against. The source row names no alternative source/projection syntax; this is the pre-amendment state amendment 5 replaces.

### Option — two closed source forms plus a fixed projection syntax (chosen)
A step-output list-field or an artifact list-field as source, and `${step.outputs[*].field}` as the only projection, so both are decidable by lint.

## Decision

`for_each` sources: `${step.outputs.<list-field>}` or `artifact:Kind.<list-field>`. Fan-out outputs are projected with `${step.outputs[*].field}`.

## Consequences

**Positive**: `for_each` sourcing has exactly two closed forms (a prior step's list-field output, or an artifact kind's list-field), both checkable by lint rule 7; downstream consumers of a fan-out step's outputs have one fixed projection syntax (`[*].field`) instead of inventing per-activity conventions.

**Negative**: any list source that isn't a step output field or an artifact field (e.g. a computed or literal list) has no expression path under this grammar.

**What this forbids**: `for_each` sourced from anything other than the two named forms; projecting a fan-out step's outputs by any syntax other than `${step.outputs[*].field}`.

## Evidence and confidence

[documented] Stated normatively in §9a amendment 5, one of eleven amendments produced by authoring ten activities against §1–§9 (spike P9); the source records no count of how many of the ten activities used fan-out, only that the grammar was settled from the exercise. [inferred] Later, out-of-range amendments build on this without altering its stated forms: §9b.23 (ADR 0053) clarifies that a `for_each` step counts once against lint rule 8's budget accounting, and §9c.38 (ADR 0068) makes a `gate` step inside a `for_each` a lint error, pending P8 pricing a mark — neither is recorded by the source as amending this decision's source-and-projection syntax itself.
