# `no-resolution/` — AC-432's exit `5`, and **not a run record**

Its shape differs from its three siblings and that is deliberate, so this note says so rather than
leaving a reader to infer it from an absence.

The other `explain/*/` directories are `replay::write_run` layouts — `run.json`, `inputs/`,
`events.jsonl` — read back through `replay::read_run`. **This one is a store-shaped journal and
nothing else.** There is no `run.json` and no `inputs/`, so `read_run` is never reached: exit `5`
comes from `explain`'s own absence check, not from a `ReplayError`.

## Why it carries rows at all

`events.jsonl` holds two rows for `explain-none`, and **neither is a `dag_assembled` nor an
`activity_committed`**:

| kind | why it is here |
| --- | --- |
| `reconcile_started` | an invocation reached step (6) against this item, so the store is not empty |
| `stage_transition` | the item has a recorded history, so "no record at all" is not the reason |

An **empty** store would also produce exit `5`, and that is exactly why this fixture is not empty. A
test seeded from an empty store cannot distinguish *"`explain` refuses when no resolution was
recorded"* from *"`explain` refuses when the store has nothing in it"*, and only the first is
AC-432's THEN. The rows above make the store demonstrably live while leaving the run-id resolver
with nothing to find: §7d.7's resolver reads the last `activity_committed` whose `payload["item"]`
matches, and `dag_assembled` carries neither a `run_id` nor an `item` (GAP 10, and `Dag::to_json`
measured), so an item with no `activity_committed` row has no recorded resolution by construction.

## AC-465

`events.jsonl` here is synthetic. It tracks no corpus artifact, and the `ci.yml` allowlist row that
admits it says so by name.
