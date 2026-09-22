# `RK-D-EXECUTED-LEDGER/` - an activity the journal has never committed

AC-532; ADR-0114 decision 2's `doctor` half. The rule reports the never-executed **count and
list**, and this tree is the configuration it rejects: a catalog carrying one activity,
`emit-report@1`, and **no `events.jsonl` at all**, so the folded ledger is empty and that activity
reads `executed: never`.

## Why this fixture is `clean/` minus one file

It is a byte copy of `clean/` with nothing added and nothing removed - `clean/` carried no journal
either, until this rule landed. What makes it *rejecting* rather than a second clean control is
the file that was added to `clean/` at the same time: `clean/events.jsonl`, one
`activity_committed` row for `emit-report@1`. With that row `clean/` has nothing unrun and the
rule is silent there; without it here, the rule fires.

That is deliberate and it is the only honest split available. Never-executed is **not a defect** -
ADR-0114: *"a doctor row, never a lint failure"* - so the finding is `Ok`-severity, and the pair of
trees has to differ in *whether anything is unrun*, not in whether anything is wrong.

## 🛑 The discriminating byte is an ABSENCE, which is the weaker kind of fixture

A fixture that rejects by omitting a file cannot distinguish "the rule read the journal and found
no committed row" from "the rule never read the journal". The per-row test therefore does not rest
on this tree alone: it asserts the finding **names `emit-report`**, which a rule that read nothing
could not produce, and `clean/` asserts the other direction - a journal that *does* carry the row
silences the rule. Neither half is sufficient; together they pin the fold.

## What it does not carry

No `rebuilt` sidecar, for `clean/`'s reason - `RK-D-REBUILT-TIER2` stays silent. The operator
`pre-commit` hook is carried across unchanged, because `RK-D-HOOKS` reads it and a tree without it
would fire a second rule and make this fixture reject for two reasons instead of one.
