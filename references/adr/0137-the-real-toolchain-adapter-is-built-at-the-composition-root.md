---
id: "0137"
title: "`redkiln run` constructs a **real** `Toolchain` adapter at the composition root instead of `FixtureToolchain::default()` — chosen from `config.toml`'s `[adapters] toolchains` in declared order by the port's own `detect()`, over the **run-owned worktree** as `project_root` (§7c.14's isolation, never the workspace running the checkpoint), with the harness's already-captured child environment, a construction-time backstop bound, and `<common-dir>/redkiln/tmp` as scratch; **when no listed adapter detects, the run refuses** rather than falling back to a fixture that reports itself absent"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0094 decision 1 (Phase 1 ships Node and Rust `Toolchain` reals); references/adr/0003 (the five ports); references/2026-09-08-acceptance-criteria.md AC-473 (*"toolchain stage [to have] run through `rk-adapter-toolchain-rust` behind the `Toolchain` port"*), AC-217/AC-488/AC-509 (the child-environment denylist); references/adr/0099 decision 2 (*"the two Toolchain adapters run bounded commands under the same timeout without a token"*, RT-5); references/adr/0127 decision 3 (a `Command` is built and handed over, never spawned in the adapter); runbook/phase-1/s7c-adapters.md §7c.14 (the fixture-project rule); crates/rk-adapter-toolchain-rust/src/lib.rs:9-13 and :155-199; crates/rk-cli/src/verbs/run.rs:222-292; crates/rk-cli/src/adapters.rs:31,146-181
supersedes: []
superseded_by: ["0139", "0140", "0170"]
related: ["0003", "0073", "0094", "0099", "0107", "0127"]
evidence: []
consumers: []
---

## Context and problem

**`rk-adapter-toolchain-rust` exists, is complete, and is never constructed on any run path.**
`run.rs:285-292` builds the `Ports` bundle with
`toolchain: Arc::new(rk_ports::fakes::FixtureToolchain::default())`, and `FixtureToolchain`'s `Default`
gives `present: false` — so the live run path ships a toolchain **that reports itself absent**. Measured:
the only two non-test construction sites of any `Toolchain` are `run.rs:291` and `doctor.rs:195`, and
`RustToolchain::new` has **zero** call sites in `rk-cli` or `rk-kernel`.

AC-473 requires the self-hosting run's *"toolchain stage [to have] run through `rk-adapter-toolchain-rust`
behind the `Toolchain` port (ADR-0094 decision 1)"*. That is Phase-1 exit criterion (1)'s clause, and it is
a claim about **the binary on the run path**, not about a test. `s7g` gives the port its first call site
(ADR-0135); this record gives that call site a real adapter to reach.

**Four construction values have no decided source, and `Config::new` refuses on every one of them**
(`rk-adapter-toolchain-rust/src/lib.rs:155-199`): a relative or absent `project_root`, a zero `timeout`
(RT-5 — *"a zero bound is no bound"*), a `child_env` carrying a denylisted name or lacking one cargo needs,
a relative `scratch`. The adapter is deliberately strict; the composition root must therefore decide each.

**`project_root` is the one with a standing constraint already on the record.** The adapter's own header
(`lib.rs:9-13`) says it runs *"over a **fixture project, never the workspace running the checkpoint** (§7c.14:
a Rust adapter over `D:/repos/redkiln-rs` would run `cargo test --workspace` from inside Checkpoint 7c's
own, re-entering the real row without bound on the same target dir; **the self-hosting run over
`redkiln-rs` itself is Step 8's**)"*. So the rule is not "never redkiln-rs" — it is **never the checkout the
checkpoint is running in**.

**`Ports` holds one `Arc<dyn Toolchain>`** (`rk-ports/src/root.rs:41`), not a map keyed by toolchain, while
`config.toml` lists **two**: `toolchains = ["rk-adapter-toolchain-node", "rk-adapter-toolchain-rust"]`.
`adapters.rs` resolves those to **names and refuses unknown ones, constructing nothing** — *"a resolver that
built adapters would be the composition root, and the composition root is `main`."* ADR-0073's keyed
expansion is **selection**-side and answers a different question. So which adapter fills the one slot is
undecided, and this record decides it.

## Decision drivers

- Criterion (1) is one of the two Phase-1 exit criteria that failed, measured. A test-only demonstration
  would satisfy `s7g` §7g.4's wording and leave the criterion exactly where it is.
- A fixture that reports itself absent is worse than no toolchain: it makes every `detect()` false and every
  `test()` a count of its own argument list, so a run "succeeds" having verified nothing. The current state
  is invisible precisely because it is well-formed.
- §7c.14's hazard is real and must not be re-armed: a Rust adapter pointed at the checkout a checkpoint is
  running in would re-enter `cargo test --workspace` on the same `target/` without bound.
- The port already has the method for choosing between adapters. `Toolchain::detect` is documented as
  *"Is this toolchain present for the project under test?"* and has, today, no caller at all.
- Every value `Config::new` takes is already in scope at `run.rs:285`, or is one line from it. The captured
  environment is `env.as_map()`, which `real_harness_config` already clones for the harness under the same
  denylist (AC-217/AC-488/AC-509).

## Considered options

1. **Construct the real adapter at the composition root, chosen by `detect()` (taken).**
2. **Test-only observation** — drive a `tool` step through a real `RustToolchain` over a fixture project in
   a test, and leave `run.rs` on the fixture. Rejected: it satisfies §7g.4's bullet and leaves AC-473's
   clause with no mechanism, which is the exact gap `s7g` exists to close.
3. **Wire it behind a flag, defaulting to the fixture.** Rejected: the default path stays unmeasured, and a
   configuration axis is added that no record decides. The reversibility it buys is already available —
   `[adapters] toolchains` is the axis, and it is a decided one.
4. **Pick the adapter by position — first in `[adapters] toolchains`.** Rejected: the shipped order lists
   Node first, and this tree is Rust. Position would select an adapter that is measurably absent, and would
   make the pack's key order load-bearing for a reason no reader could infer.
5. **Fall back to `FixtureToolchain` when nothing detects.** Rejected: it restores the present state under a
   new name, and it does so on the path where a real failure is least visible.
6. **`project_root` = the repository's main working tree.** Rejected on §7c.14: `run` already creates a
   run-owned worktree, and pointing the adapter at the invoking checkout re-arms the unbounded re-entrancy
   §7c.14 names.

## Decision

1. **The composition root constructs the toolchain adapter.** `run.rs`'s `Ports` takes a real
   `Arc<dyn Toolchain>` in place of `FixtureToolchain::default()`. `adapters.rs` continues to return
   **names, not handles**; nothing about that separation changes.

2. **The adapter is chosen by `Toolchain::detect()`, in `[adapters] toolchains`' declared order, first
   success wins.** This is the port's own question — *"Is this toolchain present for the project under
   test?"* — asked of the project actually under test. Over this repository, Node's `detect` looks for
   `package.json` and a local `vitest` and answers `false`; Rust's looks for `Cargo.toml` and runs
   `cargo --version` and answers `true`. **`detect`'s use here is adapter selection at construction, not a
   `tool` step**, and it is therefore not in ADR-0135's four-verb map — the two records do not disagree.

3. **`project_root` is the run-owned worktree** — `<common-dir>/redkiln/worktrees/<item>`, the value
   `run.rs:223` binds as `owned` and already has in scope where `Ports` is built. This satisfies §7c.14
   exactly: it is a separate checkout with its own `target/`, and it is never the workspace a checkpoint is
   running in. It is also the only correct answer on the merits — it is the tree the run is operating on.

4. **`child_env` is the same captured map the harness receives**, `env.as_map().clone()`, passed to
   `Config::new` and **refused there** if it carries a denylisted name or lacks one cargo needs on this box.
   The capture is not re-done and the denylist is not re-implemented; both are the harness rule's
   (AC-217, AC-488, AC-509), which the adapter's `env` module already enforces.

5. **`timeout` is a construction-time backstop, and its floor is measured rather than chosen.** The port
   carries no timeout parameter (ADR-0099 decision 2, RT-5), so this single bound backstops every child the
   adapter spawns, while a step's own `timeout_secs` remains the kernel's. **The backstop may not be smaller
   than the largest step it could serve, or a declared step timeout would be silently capped.** Measured over
   the shipped pack, the largest declared literal `timeout_secs` is **3600**; the default is therefore
   3600 s, overridable by an optional `[adapters] toolchain_timeout_secs`, with zero refused by the adapter
   as it already is.

6. **`scratch` is `<common-dir>/redkiln/tmp`**, absolute, beside the store and the run records — a directory
   this tree already carries. No new top-level location is invented.

7. **`program` is `[adapters] toolchain_programs`' entry for the chosen adapter when the configuration names
   one (RT-49), otherwise the adapter's own default** (`cargo` for Rust). This is the existing hook and the
   one way a process-level test reaches a fixture child; it is reused, not duplicated.

8. **When no listed adapter detects, `run` refuses**, with a typed configuration error naming the adapters
   tried and the project root, and exits as a configuration error. **It does not fall back to a fixture.**
   A toolchain that reports itself absent is the state this record exists to end, and reinstating it as a
   fallback would end it only in name.

9. **`FixtureToolchain` keeps every use it has today except `run.rs:291`.** It remains the fake for the
   kernel's and the ports' suites, and `doctor.rs:195` keeps `.absent()` deliberately. This record removes
   one construction site, not the fake.

**What this record does NOT decide.** It does not decide which adapter serves an **artifact kind** — that is
ADR-0073's keyed expansion, selection-side, untouched. It does not add or remove a port method; the count
stays six. It does not decide the 29 catalog verbs with no port method (ADR-0135). It does not make
criterion (1) **met**: it supplies the mechanism AC-473's clause needs, and the run that demonstrates it is
Step 8's.

## Consequences

**What this forbids**

- Forbids `FixtureToolchain` on the `redkiln run` path, under any name, including as a fallback when
  detection finds nothing. A fake that answers `present: false` is not a degraded real.
- Forbids pointing `project_root` at the checkout a checkpoint is running in. §7c.14's unbounded
  `cargo test --workspace` re-entrancy is the named hazard.
- Forbids selecting the adapter by list position, by hostname, by an environment variable, or by anything
  other than `detect()` over the project under test.
- Forbids a construction-time backstop smaller than the pack's largest declared step timeout, and forbids a
  zero bound (RT-5, already refused by the adapter).
- Forbids re-capturing or re-filtering the child environment in `rk-cli`. The map is the harness's capture
  and the denylist is the adapter's; a second implementation of either is a second thing to keep in step.
- Forbids spawning a child from `rk-cli` to answer the detection question directly. Detection is the port's
  method, called through the port.

**What follows**

- `rk-adapter-toolchain-rust` is constructed on a run path for the first time, and AC-473's clause acquires
  the mechanism it has never had.
- `Toolchain::detect` gains its first caller, and both adapters' `detect` implementations become live code
  rather than conformance-suite fodder.
- The adapters' construction-time refusals become reachable from the CLI: a denylisted or incomplete
  environment now refuses a **run** rather than an unused constructor. Expect that to surface environment
  facts about the box, and treat the refusals as the instrument.
- Runs over a tree with neither Node nor Rust refuse. That is a real narrowing, stated rather than
  discovered; Phase 1's subject is self-hosting over this repository.

## Evidence and confidence

- [measured 2026-09-22] `FixtureToolchain::default()` non-test construction sites: exactly two —
  `rk-cli/src/verbs/run.rs:291` and `rk-cli/src/verbs/doctor.rs:195`. `RustToolchain::new` call sites in
  `rk-cli/src` and `rk-kernel/src`: **zero**.
- [measured 2026-09-22] `FixtureToolchain` derives `Default` with `present: false`, so `detect()` returns
  `Ok(false)` on the live run path and `test(selection)` returns `selection.len()`.
- [measured 2026-09-22] `.redkiln/config.toml` — `toolchains = ["rk-adapter-toolchain-node",
  "rk-adapter-toolchain-rust"]`; `adapters.rs:31`'s `TOOLCHAINS` carries the same two names and refuses any
  other with `Refusal::UnknownAdapter`.
- [measured 2026-09-22] Declared literal `timeout_secs` in `.redkiln/catalog/`: values 30, 60, 120, 300,
  600, 900, 1200, 1800, **3600**, largest **3600**. A further **94** step lines carry a
  `${toolchain.<key>}` substitution instead of a literal; `spikes/p9/registry.toml`'s `[toolchain.timeouts]`
  names the eleven legal keys and **carries no durations for them**, so those substitutions have no value
  source on this tree either — a separate gap, recorded and not filled here.
- [measured 2026-09-22] `.git/redkiln/` holds `hooks-empty`, `rebuilt`, `runs`, `store.sqlite3`, `tmp`,
  `worktrees`. The scratch location of clause 6 exists.
- [documented] `lib.rs:9-13` — the fixture-project rule and its *"the self-hosting run over `redkiln-rs`
  itself is Step 8's"* clause, quoted above.
- **Confidence: high on the construction values, each of which is either measured or already in scope at
  the construction site.** What is **not** measured is that `detect()` answers as expected for both adapters
  over a run-owned worktree — no run has ever constructed either adapter, so the first execution is the
  first measurement. Clause 8 is written so that a surprise there **refuses loudly** instead of degrading
  quietly, and this record does not predict the result.
