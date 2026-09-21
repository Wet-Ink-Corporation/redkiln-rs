# `RK-D-DANGLING-ATTACH` - the rejecting fixture

**Severity:** `error` · **Pins:** AC-420 - "... or a skill `attach.roles` naming a role that no enabled pack declares"

**The defect.** `skills/summarise.toml` attaches to `archivist`; the enabled pack declares no such role. The skill half of §7d.13's floor is spelled exactly as written - nothing outside `doctor/roles.rs` reads skills, so no closed-crate schema collides with it.

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
