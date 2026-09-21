# `RK-D-UNREAD` - the rejecting fixture

**Severity:** `warning` · **Pins:** AC-426 / AC-472 / AC-147 - an artifact kind written and never read, over the last N runs

**The defect.** `events.jsonl` carries 51 runs of one stage producing kind `k` with an `artifact.write` and never an `artifact.read` of it, against `unread_window_runs = 50`. A WARNING. The 49/51 pair that makes the window boundary discriminating is `tests/fixtures/unread/`'s, at cycle 8; this directory exists so the id has its own rejecting fixture (AC-415).

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
