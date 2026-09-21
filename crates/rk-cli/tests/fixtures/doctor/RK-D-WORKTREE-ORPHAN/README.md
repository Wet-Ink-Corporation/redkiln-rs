# `RK-D-WORKTREE-ORPHAN` - the rejecting fixture

**Severity:** `error` · **Pins:** ADR-0105 clause 6/7 (RT-45) - a worktree entry whose working tree is gone

**The defect.** `dot-git/worktrees/gone/gitdir` points at a path that does not exist. It is read off the FILESYSTEM at `<common-dir>/worktrees/<name>/gitdir`: `GitOp` has no `worktree list` op and `rk-git` is not edited by this stage.

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
