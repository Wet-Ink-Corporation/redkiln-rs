---
id: "0149"
title: "`detect()` chooses, and a candidate adapter that cannot be CONFIGURED is recorded and walked past rather than ending the run. `real_toolchain` constructed AND validated every listed adapter with `?` before `detect()` was asked anything, so the FIRST candidate whose `Config::new` refused exited the whole run at **3** — and every shipped and fixture `.redkiln/config.toml` in this repository declares `toolchains = [\"rk-adapter-toolchain-node\", \"rk-adapter-toolchain-rust\"]`, **node first**, while the two adapters' required-env tables DIFFER (node on Windows wants `SystemRoot` and `PATH`; rust wants `SystemDrive` and one of `TMP`/`TEMP`/`USERPROFILE`). So a child environment that satisfies rust and not node refused a Rust repository **naming Node**, before the question of which toolchain the project uses was ever asked. ADR-0137 clause 2 already said selection is by declared order and `detect()`; this record makes the implementation match it, and routes every refusal into ADR-0139's `tried` string so nothing is swallowed"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0137 (clause 2 — selection by declared order and `detect()`, which the implementation did not honour); references/adr/0139 (the `UndetectedToolchain` that carries `tried` when nothing detects); crates/rk-cli/src/verbs/run.rs (`real_toolchain`); crates/rk-adapter-toolchain-node/src/env.rs (`REQUIRED_CHILD_ENV`); crates/rk-adapter-toolchain-rust/src/env.rs (`REQUIRED_CHILD_ENV`, `REQUIRED_ANY_OF`)
supersedes: []
superseded_by: []
related: ["0003", "0134", "0135", "0137", "0139", "0148"]
evidence: []
consumers: []
---

## Context and problem

`real_toolchain` walks `[adapters] toolchains` in the file's declared order and asks each candidate
`detect()`. But the construction of each candidate used `?`:

```rust
let cfg = rk_adapter_toolchain_node::Config::new(...).map_err(|e| { …; Exit::ConfigError })?;
Arc::new(rk_adapter_toolchain_node::NodeToolchain::new(cfg).map_err(|e| { …; Exit::ConfigError })?)
```

so a candidate that could not be configured ended the **entire run** before `detect()` was reached —
including the `detect()` of every later candidate.

**This is the default ordering, not an edge case.** Every `.redkiln/config.toml` in the repository,
shipped and fixture alike, declares:

```toml
toolchains = ["rk-adapter-toolchain-node", "rk-adapter-toolchain-rust"]
```

and the two adapters' required-env tables are **not the same**. Measured 2026-09-22 on Windows:

| adapter | `REQUIRED_CHILD_ENV` | `REQUIRED_ANY_OF` |
| --- | --- | --- |
| `rk-adapter-toolchain-node` | `SystemRoot`, `PATH` | — |
| `rk-adapter-toolchain-rust` | `SystemDrive` | `TMP`, `TEMP`, `USERPROFILE` |

So a child environment carrying `SystemDrive` and `TMP` but no `SystemRoot` refuses node's
configuration and satisfies rust's — and the run died at exit **3** with a message naming Node, on a
repository whose **root** carries no `package.json` *(the tree holds exactly one, under
`crates/rk-adapter-toolchain-node/tests/fixtures/project/` — that adapter's own fixture, not a
project this binary is ever run against. The claim is about the project root, and is written that
way rather than as the absolute it first was)*.

## Decision drivers

- **ADR-0137 clause 2 already decided this.** Selection is by declared order and `detect()`. A loop
  that refuses on a candidate's *configuration* has made a selection decision on a ground clause 2
  does not name. This record does not change that policy; it makes the code honour it.
- **The information must not be lost.** The reason a candidate could not be built is exactly what an
  operator needs when nothing detects. ADR-0139 already built the channel: `UndetectedToolchain`
  carries a `tried` string that names every candidate and why it was not selected.
- **A hard error is still right for one case, and it is a different case.** A resolved adapter name
  this binary cannot construct is a **build defect** — `AdapterSet::resolve` and the match have
  drifted apart — not a configuration one. That arm keeps its `return Err(Exit::ConfigError)`.

## Considered options

**(a) Reorder the shipped configs to put rust first.** Rejected: it changes which toolchain wins on
a tree where both detect, which is a *policy* change, and it leaves the defect intact for whichever
adapter ends up first. Declared order is the operator's to choose.

**(b) Construct every candidate lazily, only when `detect()` is about to be asked.** This is what
the change effectively does; stated as an option because it is the shape, not a different outcome.

**(c) Ask `detect()` on a candidate that failed to build.** Impossible — `detect` is a method on the
constructed adapter — and it is why the refusal has to become a recorded non-selection rather than a
deferred question.

**(d) Record the refusal in `tried` and continue.** Taken. It reuses ADR-0139's existing channel, so
no information is invented and none is lost.

**(e) Keep `?` for the unknown-adapter arm.** Taken, and kept deliberately separate: that arm is a
drift between `AdapterSet::resolve` and this match — a build defect — and reporting it as "not
present" would hide it behind a configuration message.

## Decision

1. **A candidate whose `Config::new` or `<Adapter>::new` refuses is recorded in `tried` and the loop
   continues** to the next declared candidate. It is no longer a `?`.

2. **The recorded reason names the adapter and distinguishes the two failures** — *"the
   configuration was refused"* versus *"could not be built"* — so `tried` says which stage of
   construction the candidate died at.

3. **If no candidate detects, the behaviour is unchanged**: ADR-0139's `UndetectedToolchain`, whose
   `tried` now additionally carries the construction failures. A run needing no toolchain still
   proceeds; a run reaching a `tool` step is refused *there*, by name, with the accumulated reasons.

4. **An unknown adapter name still ends the run at `Exit::ConfigError`.** That is a drift between
   `AdapterSet::resolve` and this match, and reporting it as "not present" would hide a build defect
   behind a configuration message.

## Consequences

`real_toolchain` is now `pub`, and **only so that it can be observed**. `rk-cli`'s tests are
integration tests in a separate crate, so a private function's behaviour is reachable only through a
whole `redkiln run`, which passes through steps (1)–(7) and out into a dispatch. The property this
record decides is a property *of this loop*; asserting it two layers up would be asserting it of
something else.

The guard is `rk-cli`'s
`adapter_selection::a_candidate_whose_config_is_refused_does_not_end_the_run_before_detect_chooses`,
which builds the environment above and asserts the loop returns `Ok`. It carries **two controls**,
because *"the loop returned `Ok`"* is also what a loop that refused nothing returns: it asserts
against the adapters' own `Config::new` that this environment really does refuse node **and** really
does satisfy rust, so an environment that satisfied both would fail the test rather than pass it
vacuously.

**What this forbids**

- Ending a run on a candidate's construction failure while a later candidate is undetermined.
- Discarding a construction failure: every one reaches `tried`.
- Treating an unknown adapter name as a non-detection.

## Evidence and confidence

**Measured, not argued.** The two required-env tables were read out of the adapters' own
`src/env.rs`: node `REQUIRED_CHILD_ENV = ["SystemRoot", "PATH"]` with no `REQUIRED_ANY_OF`; rust
`REQUIRED_CHILD_ENV = ["SystemDrive"]` and `REQUIRED_ANY_OF = ["TMP", "TEMP", "USERPROFILE"]`. The
ordering claim was taken over **every** `toolchains = [...]` declaration in the tree — the shipped
`.redkiln/config.toml`, `init`'s emitted template and its expected fixture, the `rk-cli` config
fixtures, the `pack-min` fixture and all 41 `doctor/**` fixtures — **node first, with no exception**.

**Mutant-tested 2026-09-22.** Turning the node arm's `continue` back into
`return Err(Exit::ConfigError)` turns the guard RED. The mutant was applied by script, diffed and
read, and restored byte-exact.

**Confidence, and one thing this does NOT assert.** The guard exercises `real_toolchain` directly.
It does not assert the end-to-end exit code of `redkiln run` under this environment, because every
route from the CLI into this loop passes through steps (1)–(7) and out the far side into a dispatch,
and a process-level assertion would be about that whole path rather than about the selection loop.
The property decided here is a property of the loop, and it is asserted there.
