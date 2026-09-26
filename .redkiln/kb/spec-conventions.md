# Spec conventions

kb:spec-conventions — read by fix-spec/draft, bdd-scenarios/draft and bdd-scenarios/to-spec (s8c §8c.4;
ADR-0170 decision 8, proposed). Authored 2026-09-26. Each convention below restates a passage named in
`source:`; nothing here is authored from memory.

source: references/research/2026-09-07-activity-catalog-raw.md, the spec-form rows (user-story-ac, bdd-scenarios,
fix-spec) and the line "Ordered options for the primary spec form (pick one)"; .redkiln/catalog/fix-spec.toml and
.redkiln/catalog/bdd-scenarios.toml (their rationales and policy steps).

## One spec form per item

The raw catalog orders the primary spec forms: *"Ordered options for the primary spec form (pick one):
`user-story-ac` < `structured-nl-requirements` < `bdd-scenarios` < `use-case-spec` < `model-based-spec`."* An
item's spec is written in the one form its activity produces.

## A spec carries acceptance criteria

- `user-story-ac`: *"Write stories with acceptance-criteria-based requirements"* (SWEBOK ch.1 §4.3,
  *"Acceptance Criteria-Based Specification"*).
- `fix-spec`'s policy step refuses a spec without them: `spec.acceptance_criteria.len() > 0`.

## A bug's spec is derived from its cause

`fix-spec`: *"For a bug: expected behaviour, regression test intent, scope of fix"*, from the `RootCause`. Its
rationale: *"A bug's spec is the expected behaviour plus the regression test that pins it; it is derived from the
cause, not from a story."*

## Scenarios are executable examples

`bdd-scenarios`: *"Given/When/Then executable examples as spec"*. Its rationale: *"executable examples double as
the acceptance test"*. The scenarios are drafted first; `to-spec` then writes the `Spec` from them, so the spec and
the scenarios describe the same behaviour.
