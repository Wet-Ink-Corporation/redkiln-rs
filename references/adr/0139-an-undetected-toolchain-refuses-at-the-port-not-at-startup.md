---
id: "0139"
title: "When no configured toolchain adapter detects, the run receives a `Toolchain` **every one of whose methods refuses by name** rather than being refused at startup — superseding **ADR-0137 clause 8 in part**, at that clause's `## Decision` sentence only. Clause 8's prohibition is untouched and is what this record keeps: there is still **no fixture fallback**, because a fixture's `test()` returns `Ok` and every method here returns `Err`. Measured: clause 8's startup refusal broke **11** `rk-cli` tests whose fixtures carry no project and whose subject is the gate/answer path — a run was refused for lacking a capability it never reached"
status: proposed
date: 2026-09-22
legacy_id: none
source: references/adr/0137 decision 8 (the clause this record supersedes in part); references/adr/0092 rule 1 (a `## Decision` sentence moves by supersession, never by erratum); references/2026-09-08-acceptance-criteria.md AC-473; crates/rk-ports/src/toolchain.rs:44-64 (the port's six methods); crates/rk-ports/src/fakes.rs:313-371 (`FixtureToolchain`, whose `test()` returns `Ok(selection.len())` — the shape clause 8 forbids); crates/rk-cli/src/verbs/run.rs (`UndetectedToolchain`, `real_toolchain`)
supersedes: ["0137"]
superseded_by: []
related: ["0094", "0099", "0135", "0137"]
evidence: []
consumers: []
---

## Context and problem

ADR-0137 clause 8 reads: *"When no listed adapter detects, `run` refuses, with a typed configuration
error naming the adapters tried and the project root, and exits as a configuration error. **It does not
fall back to a fixture.**"* It was written to end a measured defect — `run.rs` shipping
`FixtureToolchain::default()`, whose `Default` is `present: false`, so the run path carried a toolchain
that reported itself absent and whose `test()` nonetheless returned `Ok`.

**Implemented as written, it refused runs that never needed a toolchain.** Measured 2026-09-22, at the
first execution of clause 1: **11 of `rk-cli`'s 101 tests failed**, every one of them over a fixture
repository carrying neither a `Cargo.toml` nor a `package.json`, and every one of them about the
**gate/answer** path rather than the toolchain — for example
`ac406_answer_after_commit_is_void_gate_stays_open`, whose premise line reads *"opening the gate must
exit 0 (AC-403)"* and which got exit 3 with
*"no configured toolchain adapter detects a project at …/worktrees/gate-1"*.

**The defect is in the clause's timing, not in its prohibition.** A `run` that walks no `tool` step
needs no toolchain, and refusing it names a capability the run never reached. Clause 8's *reason* —
that a fixture answering `Ok` is how a toolchain that verified nothing has been reporting green — is
untouched by this and is what the replacement preserves.

## Decision drivers

- A refusal should name what was actually needed, at the point it was needed. A startup refusal for an
  unreached capability is the same class of miscommunication as a silent success, inverted.
- The prohibition is the load-bearing half. Whatever replaces clause 8 must not be able to report a
  green for work it did not do.
- `redkiln run`'s own contract is that a gate is an **exit, and exit 0 at that** (D-01, AC-240, AC-024).
  A startup refusal over a toolchain makes that contract conditional on an unrelated capability.
- ADR-0092 rule 1: this moves a `## Decision` sentence, so the route is a **supersession**, not an
  erratum — even though the record being moved is one day old and its own author is moving it.

## Considered options

1. **Refuse at the point of use, by name (taken).**
2. **Keep the startup refusal and give every fixture repository a project.** Rejected as the larger and
   less honest change: it would add a `Cargo.toml` and a toolchain-program override to fixtures whose
   subject is the gate path, coupling eleven tests to a capability they do not exercise, and it would
   make the `rk-cli` suite spawn a child per test to answer a question none of them asks.
3. **Fall back to `FixtureToolchain`.** Rejected — this is exactly what clause 8 forbids and the reason
   stands: `FixtureToolchain::test()` returns `Ok(selection.len())`, a success derived from the length
   of its own argument, which is how a toolchain that verified nothing has been reporting green.
4. **Make `Ports::toolchain` optional.** Rejected: it moves an `Option` into the kernel's port bundle
   for a configuration condition, and every call site would then have to decide what `None` means —
   which is the decision this record makes once, in one place.

## Decision

1. **ADR-0137 clause 8's `## Decision` sentence is superseded in part.** A startup refusal is replaced
   by a refusal at the point of use. **Every other clause of ADR-0137 stands** — the composition root
   constructs (clause 1), `detect()` in declared order chooses (clause 2), the run-owned worktree is
   `project_root` (clause 3), the harness's captured environment is reused (clause 4), the backstop's
   measured floor (clause 5), the scratch location (clause 6), RT-49's program override (clause 7), and
   `FixtureToolchain` keeping every use but `run.rs`'s (clause 9). ADR-0137 **stays `accepted`** and
   carries the `superseded_by` link, as a partial supersession requires.

2. **When no configured adapter detects, the run receives an `UndetectedToolchain`.** `detect()` answers
   `Ok(false)`, which is the truth. **Every other method returns `Err(StepError::Unsupported)`**, naming
   the verb, the project root and each adapter that was tried with why it declined.

3. **Clause 8's prohibition is kept in full and is the test of any future replacement**: no `Toolchain`
   on the `run` path may answer `Ok` to `build`, `test`, `lint`, `classify` or `change_summary` without
   having done the work. The difference between this record and the fallback clause 8 forbids is not one
   of degree — a fixture's `test()` returns `Ok`, and every method here returns `Err`.

4. **A construction refusal is NOT deferred.** A denylisted or incomplete child environment, a relative
   or absent project root, a zero bound or a registry that does not load are **configuration defects**
   and still refuse the run at startup, as ADR-0137's drivers require. Only *"nothing here to detect"*
   moves to the point of use.

**What this record does NOT decide.** It does not change which adapter is chosen or how. It does not
make criterion (1) met. It does not license a `tool` step succeeding without a toolchain: under ADR-0135
the verb is mapped and the port is called, and here the port refuses.

## Consequences

**What this forbids**

- Forbids any `Toolchain` on the `run` path whose methods can return `Ok` without doing the work —
  `FixtureToolchain` included, under any name, at startup or as a fallback.
- Forbids an `UndetectedToolchain` method that returns `Ok`, `detect` excepted, whose `Ok(false)` is the
  true answer.
- Forbids a refusal that does not name the verb, the project root and the adapters tried. A refusal that
  says only "no toolchain" is indistinguishable from a configuration typo.
- Forbids deferring a **construction** refusal (clause 4). An environment that cannot build is a defect
  now, not at the first `tool` step.
- Forbids reinstating a startup refusal for the undetected case without superseding this record.

**What follows**

- The 11 tests refused by clause 8 pass again, and they pass because the run no longer needs a
  capability it never reached — not because a fixture was taught to answer `Ok`.
- A run that **does** reach a `tool` step with no toolchain present is refused at that step, by name, in
  the journal. That is the first time this tree can say so.
- ADR-0137 gains a `superseded_by` link and stays `accepted`; both bodies record the scope.

## Evidence and confidence

- [measured 2026-09-22] Clause 8 implemented as written: `rk-cli` **90 passed, 11 failed**; with the
  refusal moved to the point of use, **101 passed, 0 failed**. The 11 are `answer::*` and `run::*` tests
  over fixture repositories with no project.
- [measured 2026-09-22] The observed refusal text:
  *"redkiln: no configured toolchain adapter detects a project at
  …/.git/redkiln/worktrees/gate-1 — rk-adapter-toolchain-node: not present; rk-adapter-toolchain-rust:
  not present"*, against a test whose premise is *"opening the gate must exit 0 (AC-403)"*.
- [measured 2026-09-22] `FixtureToolchain::test` is `Ok(u32::try_from(selection.len()).unwrap_or(u32::MAX))`
  and its `Default` is `present: false` — a success derived from the length of its own argument.
- **Confidence: high, and the limit is that this record is one day younger than the one it moves.** The
  startup refusal was not wrong in principle; it was untested against the suite it broke, because
  ADR-0137 was written before clause 1 had ever been executed. That is the honest reading — a decision
  taken ahead of its first measurement, corrected by it — and it is recorded rather than smoothed.
