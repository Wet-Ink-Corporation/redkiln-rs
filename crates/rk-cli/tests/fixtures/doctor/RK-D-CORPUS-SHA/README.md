# `RK-D-CORPUS-SHA` - the rejecting fixture

**Severity:** `error` · **Pins:** AC-314 (D18-19) - *"a doctor rule that refuses a regression record
whose corpus sha is unknown to the pinned corpus remote"*

**The defect.** `.redkiln/regression.toml` records a review regression run at corpus sha
`deadbeef…`, forty hex that resolves to no commit in any repository this fixture can reach.

## 🛑 THE RECORD IS A FILE, AND THAT IS MEASURED RATHER THAN CHOSEN

`s7d` §7d.8 sketches this fixture as an `events.jsonl` row **and** describes the corpus path as
*"a `ConfigFile` key"*. Measured 2026-09-21, neither can carry it:

* `EventKind::ALL` is **closed at 38** (`crates/rk-types/src/event_kind.rs`) and holds no
  regression kind. A row with an unknown kind is not an event;
* `ConfigFile` is `#[serde(deny_unknown_fields)]` over `[adapters]`, `[packs]` and `[doctor]`
  (`crates/rk-cli/src/config.rs`) and has no corpus key. A config carrying one does not parse.

So the record is `.redkiln/regression.toml`, and a dated §7d.8 note records the reading. 7e's
schemas may re-point it.

## Resolution is asked of git, never pattern-matched

The rule runs `rev-parse --verify <sha>^{commit}` through `rk_git::git_exec::run` - the one wrapper
`std::process::Command` is permitted behind (ADR-0077). A forty-character hex string is not a
commit; only a repository can say whether it names one, and a rule that checked the *shape* would
pass every well-formed sha that resolves nowhere, which is this fixture exactly.

## How this directory is used

`ac415_every_rule_id_has_a_rejecting_fixture` and
`ac314_regression_corpus_sha_unresolvable_doctor_row` **materialise** it into a `tempfile` git
repository before running the rule (`s7d` §7d.13): `git init`, `dot-git/` renamed to `.git/`, and
any store description built. Nothing runs against the tracked copy.
