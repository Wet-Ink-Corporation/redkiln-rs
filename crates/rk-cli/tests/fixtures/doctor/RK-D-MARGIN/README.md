# `RK-D-MARGIN` - the rejecting fixture

**Severity:** `error` · **Pins:** s7c ARMED half - a pack whose declared margin is below the floor

**The defect.** `[margins] five_hour = 1`. §7d.8's sketch spells this `margin_ticks = 1`; the pack schema's measured keys are `[margins] five_hour` / `seven_day` (`tests/fixtures/pack-min/pack.toml`, 2026-09-21), and the fixture spells what the tree reads. If 7e re-points the key this fixture follows it.

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
