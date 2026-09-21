# `RK-D-REBUILT-TIER2` - the rejecting fixture

**Severity:** `ok` · **Pins:** RT-56 - tier-2 emptiness after a rebuild is EXPECTED, not a defect

**The defect.** The `rebuilt` sidecar is present beside a freshly rebuilt store. This is the table's one `ok`-severity row: the finding says `expected after rebuild`, and `ac415_` counts findings of ANY severity, so an `ok` finding satisfies the per-id join. `doctor/clean/` carries no sidecar, so the row is SILENT there rather than `ok` - which is what keeps the clean run clean. The sidecar's home is RT-56, routed and not decided.

## How this directory is used

`ac415_every_rule_id_has_a_rejecting_fixture` and this row's own per-row test **materialise** it
into a `tempfile` git repository before running the rule (`s7d` §7d.13): `git init`, `dot-git/` is
renamed to `.git/`, and any `store/` description is built into a real store through
`rk_store::conn::Writer::open` + `rk_store::identity::initialise`. Nothing runs against the tracked
copy, and a fixture that materialises to nothing is red on the rule's own finding count.

Parts of a repository that cannot be tracked as themselves are carried as descriptions: git refuses
to track a nested `.git/`, and a SQLite file is not a reviewable fixture.

## What this directory is NOT

It is not a runnable repository for `redkiln run`. It is the smallest tree that makes **this one
rule** fire, and every other rule's silence over it is incidental - `ac415_` runs each fixture
through **its own rule only**, and `doctor/clean/` is the configuration every rule passes.
