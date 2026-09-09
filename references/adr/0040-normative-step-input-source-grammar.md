---
id: "0040"
title: Normative closed grammar for step input sources
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-10
source: references/2026-09-07-activity-graph-schema.md §9a
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

Before this amendment, step `inputs` values appeared in the §7 worked example in several forms — `artifact:ReproCase`, `toolchain:logs?`, `${hypothesise.outputs.discriminating_test}` — without those forms being declared as a closed, normative set. Authoring ten activities against §1–§9 (P9 spike, `spikes/F32-DRAFT.md` §4) required settling exactly which input-source syntaxes are legal and which of them lint can validate against a registry.

## Decision drivers

- Lint needs a closed, enumerable grammar over `inputs` values to catch invalid or misspelled references (particularly `kb:` and `checklist:` ids) at catalog-load time rather than at run time.
- The existing worked example already used several source forms informally; the amendment makes that set explicit and closed rather than adding new syntax.

## Considered options

### Option — do nothing; leave input sources as informally illustrated by example
The §7 example already showed `artifact:`, `toolchain:`, and `${…}` forms without a normative list. The source row names no alternative option, so this is not a rejected candidate the source records — it is the pre-amendment state the amendment replaces, stated here so the decision has something to be measured against. What the source does say against it is implicit in the amendment itself: the grammar is declared normative and lint is given `kb:`/`checklist:` registries to check against, neither of which the informal state supported.

### Option — declare a closed, normative set of input-source forms with registry-checked prefixes (chosen)
Fixes the legal forms and adds lint validation for the two registry-backed prefixes.

## Decision

Input sources are normative and closed to: `artifact:Kind[.field]`, `kb:<atom>`, `checklist:<id>`, `toolchain:<feed>[?]`, `${…}`, and literals. Lint checks `kb:` and `checklist:` references against their respective registries.

## Consequences

**Positive**: catalog-load-time lint can now catch a misspelled or unregistered KB atom or checklist id before an activity ever runs, rather than that failing silently or at runtime.

**Negative**: this closes off any other ad hoc input-source syntax an author might otherwise reach for (e.g. arbitrary string interpolation or file-path-shaped references), and requires the `kb:`/`checklist:` registries to be kept current for lint to be useful rather than merely permissive.

**What this forbids**: a step's `inputs` value may not use any source form outside this closed set; `kb:` and `checklist:` references that are not present in their registries are a lint error, not a silent pass-through.

## Evidence and confidence

[documented] Recorded in `2026-09-07-activity-graph-schema.md` §9a item 10, sourced to `spikes/F32-DRAFT.md` §4 (P9 authoring of ten activities). No finding id or measurement is cited for this item specifically; the source states the grammar and its lint consequence without further evidence.
