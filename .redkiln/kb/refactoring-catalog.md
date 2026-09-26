# Refactoring catalog

kb:refactoring-catalog — read by tdd-red-green-refactor/refactor (s8c §8c.4; ADR-0170 decision 8, proposed).
Authored 2026-09-26. Each line below restates a passage named in `source:`; nothing here is authored from
memory.

source: references/research/2026-09-07-activity-catalog-raw.md, the refactor-catalog-move row and the
tdd-red-green-refactor row; .redkiln/catalog/tdd-red-green-refactor.toml (its refactor, refactor-verify and
still-green steps).

## What a refactoring is here

The raw catalog's `refactor-catalog-move`: *"Apply named refactorings (Extract Function, Move, Rename, Split
Phase...) with tests green"*, lineage Fowler's *Refactoring* catalog, https://refactoring.com/catalog/.

- **Named.** Each change is one of the catalog's named refactorings; the four the row names are Extract Function,
  Move, Rename and Split Phase.
- **Tests green.** A refactoring changes structure, not behaviour, and the tests stay green through it.

## Where it sits in the loop

`tdd-red-green-refactor`: *"Write failing test, make it pass, refactor; from a test list"*. The `refactor` step
runs after `green`, at rigor recommended and above. `refactor-verify` re-runs the affected tests, and the
`still-green` check requires `report.status == 'passing'`, returning to `refactor` otherwise.
