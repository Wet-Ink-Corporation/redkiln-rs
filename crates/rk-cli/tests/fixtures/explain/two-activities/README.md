# `two-activities/` — AC-430's fixture, and AC-431's card

A `replay::write_run` layout read back through `replay::read_run`.

**Two nodes, and they enter the DAG by DIFFERENT routes.** `emit-report` provides `ImplReport`,
which `[stage.implement] baseline` names; `emit-tests` provides `TestReport`, which the one
`[[stage.implement.require]]` row asks for when `has_repro` — and the recorded `context.json` carries
`has_repro = true`, so the row fired (`rows_fired = 1`). AC-430 requires **both** row families for
**every** included node and fails *"if either family is empty while the other is populated"*; a
fixture whose nodes all arrived the same way could not fail that.

Each recorded catalog activity declares all five admissibility fields AC-430 names — `provides`,
`requires`, `applicable_if`, `method_rank`, `min_rigor`.

`inputs/specs.json` carries one `RecordedSpec` per node, which is where AC-431's card comes from:
`card_digest`, and nothing else. Skills, fillers and tier are on **no recorded value** — the
binder's binding is never logged — so they are RAISED and the test asserts only the card.

Every digest here is a sha256 over a label naming what it stands for. **No corpus byte.**
