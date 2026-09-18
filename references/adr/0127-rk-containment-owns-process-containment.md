---
id: "0127"
title: A sixth Phase-1 crate, `rk-containment`, owns process containment — the one commented `#![allow(unsafe_code)]` module (a Windows Job Object, a POSIX process group) and a token-less `run_bounded` for the toolchain adapters; ADR-0099 decision 3's location clause is superseded in part and every other clause of 0099 stands
status: accepted
date: 2026-09-17
legacy_id: none
source: references/adr/0099 — decision 3 (*"the `unsafe` FFI lives in one module of `rk-adapter-claude` under a commented `#![allow(unsafe_code)]`"* — the clause this record withdraws), decision 2 (*"the two Toolchain adapters run bounded commands under the same timeout without a token"*), and its forbids (*"Forbids spawning a child on Windows outside a Job Object created before the spawn, and on POSIX outside a process group"*) — three clauses no single crate layout satisfies; references/adr/0102 decision 2 (*"the only per-crate exception to `unsafe_code = "deny"` is a commented `#![allow(unsafe_code)]` crate- or module-level attribute — the route ADR-0099's Job Object module takes — and it is the only `allow` permitted in source"*) and its forbids (*"Forbids `#[allow]` in source except the commented `unsafe_code` exception"*); references/adr/0100 decision 2 (*"Any new 0.x dependency is exact-pinned unless its row says why not"*); references/adr/0094 decision 1 (four real adapters — unchanged, since this crate is not an adapter); references/adr/0083 (the `unsafe` exception it designed for); references/adr/0076 (adapter-owned runtimes; the kernel never depends on tokio); references/2026-09-08-acceptance-criteria.md — AC-224 (*"the whole containment group is killed and no descendant of the dispatch survives the call"*; *"enumerating the group via the job (Windows) or the pgid (POSIX)"*), AC-225 (*"the classifier is keyed on the cancel token having fired, not on the exit code"*; *"the adapter's own `TerminateProcess` code on Windows"*), AC-278 (*"no child, background, detached or daemon process belonging to that run remains alive"*; *"enumerates the descendants of the run's job object / process group after exit"*), AC-331 (*"a verify run killed by its timeout or by the OS … the result is `Unverifiable`"*), AC-005 / AC-484 (no edge from `rk-kernel` to any `rk-adapter-*`), **cited by id and named clause** (DECISIONS-2026-09-15 D15-2); references/2026-09-08-architecture-design.md §9.1 (the crate table — *"this table is the crate-name authority"* — whose `-claude` row says it *"holds the Job Object / process-group containment in one module under a commented `#![allow(unsafe_code)]`"*, the row this record amends) and its counter-evidence paragraph (*"A tree kill — job or process group — reaps processes; it does not release an OS handle the dead process left behind"*); design §10.5 S-CANCEL-WIN; runbook/phase-1/s7b-ii-gates.md §7b-ii.14 (AC-278 armed to 7c/7d — its row reads *"AC-278 (the process table after exit — `rk-cli`/adapter, 7d/7c; no process is spawned on the gate path …)"* — and AC-278's own instrument, *"an integration test that enumerates the descendants of the run's job object / process group after exit"*, is what asks for an enumeration a caller outside the adapter can reach); Cargo.lock at `6152219` (`windows-sys` 0.61.2 and 0.52.0, `libc` 0.2.189 present transitively; no `nix`, no `windows`); `crates/` at `6152219` (five crates, `grep -rn 'allow(unsafe_code)' crates/` → 0); the operator's decision of 2026-09-17, taken before Step 7c's stage-open commit with the alternatives named (the toolchain crates depending on `rk-adapter-claude`; narrowing 0099's forbids to the Harness child; a shared crate — the third was taken), recorded as `spikes/DECISIONS-2026-09-17.md` D17-10
supersedes: ["0099"]
superseded_by: []
related: ["0076", "0083", "0094", "0100", "0102", "0115"]
evidence: ["F22", "PIA-010", "PIA-059"]
consumers: []
---

## Context and problem

**Three clauses of one accepted record cannot all be satisfied by any layout of the crates it names.**

- ADR-0099's forbids: *"Forbids spawning a child on Windows outside a Job Object created before the spawn, and on
  POSIX outside a process group."* — **any** child, not the Harness child.
- ADR-0099 decision 2: *"Only the Harness port carries cancellation in Phase 1; the two Toolchain adapters run
  bounded commands under the same timeout **without a token**."* — so the toolchain adapters spawn children
  (`cargo`, `npx vitest`) and those children are Windows children.
- ADR-0099 decision 3: *"the `unsafe` FFI lives in **one module of `rk-adapter-claude`** under a commented
  `#![allow(unsafe_code)]`"*, and ADR-0102 decision 2 makes that *"the only `allow` permitted in source"*.

A Job Object cannot be created, assigned or enumerated without FFI, and FFI is `unsafe`. So a toolchain adapter's
Windows child is either outside a Job Object (violating the forbids), or inside one through a second `unsafe`
site (violating 0102's *only*), or inside one through a dependency on `rk-adapter-claude` — which drags a tokio
multi-thread runtime, a stream decoder and a subscription-only Harness into a crate whose job is to run `cargo
test` and read a report. `s7c`'s 211-line form asserts *"exactly one allow(unsafe_code)"* workspace-wide and says
nothing about where the toolchain children go; the Step 7c reader map raised the conflict as unresolved by any
record, and the operator was asked.

**What is on the tree.** `crates/` holds five crates (`rk-git`, `rk-kernel`, `rk-ports`, `rk-store`, `rk-types`);
`grep -rn 'allow(unsafe_code)' crates/` → **0**; `git grep rk-containment` → **0**; `Cargo.lock` already resolves
`windows-sys` 0.61.2 (and 0.52.0) and `libc` 0.2.189 transitively, and carries no `nix` and no `windows`
[measured 2026-09-17, `6152219`]. The lifted runner's kill is `taskkill /T /F` — a PID-tree walk, the mechanism
AC-224 was amended to forbid (PIA-010, PIA-059) — and F22's one survivor was a detached child that outlived its
parent by 12,000 s, which is the hole a PID walk has and a job does not.

**The clause that gives way is the one about location, not the one about the mechanism.** Decision 3's mechanism —
`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, created before the spawn, assigned before the child can fork, the handle held
for the child's lifetime; `process_group(0)` and `killpg` on POSIX — is measured-correct and every criterion cites
it. Its *"in one module of `rk-adapter-claude`"* is a placement made when the Harness was the only spawner in view
(its own option analysis: *"only the Harness spawns long-lived children; the toolchain adapters run bounded
commands"*). The forbids and decision 2 together say the toolchain adapters' bounded commands are children too.

## Decision drivers

- ADR-0099's forbids is the safety property (no Windows child outside a job); decision 2 is the port shape (no
  token on Toolchain); decision 3's location is a packaging choice. When three clauses conflict, the packaging
  choice moves.
- ADR-0102 decision 2's *"the only `allow` permitted in source"* is the count that keeps `unsafe` auditable; the
  count must stay **one**, so the one module must be reachable by every spawner.
- ADR-0076 / AC-528: the kernel never depends on tokio, and nothing that reaches the toolchain adapters' dependency
  trees should either — `cargo tree -p rk-kernel -e normal` must name no tokio and no `rk-adapter-*` (AC-484,
  AC-005), and a toolchain crate that pulls `rk-adapter-claude` pulls tokio into every tree that holds it.
- AC-278 (armed from 7b-ii to 7c/7d as *"the process table after exit — `rk-cli`/adapter, 7d/7c"*; its instrument
  *"enumerates the descendants of the run's job object / process group after exit"*) needs the enumeration
  primitive in a place the kernel-side test can reach without an adapter.
- AC-331: a verify run killed by its timeout must come back distinguishable from a failing test; a bounded run
  whose timed-out outcome is a distinct value is what makes that a type, not a convention.

## Decision

**1. A sixth Phase-1 crate, `rk-containment`, owns process containment.** It is a library crate under `crates/`
beside the five on the tree, and it implements **no port** — ADR-0094 decision 1's *"four real adapters"* is
unchanged, and this crate is not one of them. It owns **the one** commented `#![allow(unsafe_code)]` module in the
workspace (ADR-0102 decision 2's exception, relocated), carrying ADR-0099 decision 3's mechanism unchanged:

- **Windows**: a Job Object with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, **created before the spawn**, the child
  **assigned before it can fork**, the **handle held** for the child's lifetime; the group is **enumerated via the
  job** and killed via the job (AC-224), never by a PID walk; the adapter's own `TerminateProcess` exit code is a
  named constant in this module (AC-225 as PIA-059 restates it: the Windows positive and negative controls share
  that code).
- **POSIX**: `CommandExt::process_group(0)` at spawn and `killpg` at kill; enumeration via the pgid.

**2. Two surfaces, one mechanism.** The crate exposes the containment primitives the Harness adapter composes with
its `CancelToken` (create the group, spawn into it, kill it, enumerate it, drop it) **and** a token-less
`run_bounded(cmd, timeout)` that runs a command inside a group to completion or to the timeout and returns an
outcome in which *timed out* and *killed by the OS* are values distinct from *exited with a status* — the value
AC-331's `Unverifiable` is built from. `run_bounded` takes no `CancelToken`, so ADR-0099 decision 2's *"without a
token"* stands as worded. What this crate does **not** contain: a tokio runtime, a stream decoder, any port trait,
any port type. Its dependencies are `std`, `windows-sys` (Windows) and `libc` (Unix), so that a toolchain
adapter's tree gains nothing async by depending on it.

**3. All three spawning adapters depend on it; `rk-kernel` never does.** `rk-adapter-claude`,
`rk-adapter-toolchain-node` and `rk-adapter-toolchain-rust` each take `rk-containment` as a normal dependency;
every child any of them spawns is spawned through it, on both platforms. `rk-kernel` spawns nothing (ADR-0077
confines git to `rk-git`'s one wrapper) and has no edge to this crate — `cargo tree -p rk-kernel -e normal` names
neither `rk-containment` nor tokio, and Checkpoint 7c asserts both.

**4. The count of `allow`s is unchanged: one.** ADR-0102 decision 2 and its forbids are not superseded; the one
commented `#![allow(unsafe_code)]` moves crate. Checkpoint 7c's line asserts exactly one such attribute in
`crates/rk-containment/` and **zero** `#[allow(` of any kind in the five crates Step 7c creates; every other
override in those crates is `#[expect(clippy::…, reason = "…")]` per site.

**5. Dependencies are exact-pinned per ADR-0100 decision 2**: `windows-sys = "=0.61.2"` (target Windows; already
resolved in the lock) and `libc = "=0.2.189"` (target Unix; already in the lock), each with a workspace row saying
what it is for. `nix` is not taken (not in the lock; a second binding layer for one `killpg`). A safe-wrapper crate
for the Job Object stays rejected on ADR-0099's own ground (*"it trades ~30 lines of `unsafe` behind one commented
allow for a dependency on the audit surface"*).

**6. What this record supersedes, exactly.** ADR-0099 decision 3, **the clause locating the unsafe module
"in one module of `rk-adapter-claude`"**, and nothing else of that record: decisions 1, 2, 4 (as narrowed by 0115),
5, the mechanism sentences of decision 3, and every bullet of its forbids stand; 0099 stays `accepted`. Design
§9.1's crate table is **amended**: it gains an `rk-containment` row and its `-claude` row's *"holds the Job Object /
process-group containment in one module"* becomes *"spawns through `rk-containment`"* — **a dated design note is
OWED** for that edit and for the `Origin` column's `journal.rs` lift the reader map found to disagree with §9.1's
own `rk-store` row; this record does not edit the design. ADR-0102 is untouched.

**7. What this record does NOT decide**, named rather than left to be discovered:

- **How the child is assigned before it can fork under `tokio::process::Command`**, which exposes no suspended
  spawn. Whether the Harness child is spawned suspended through `std` inside this crate and resumed after
  assignment, or spawned and assigned immediately with the breakaway fixture measuring the race, is the
  `rk-containment` slice's to prove with the fixture AC-224 names; it is raised in `DECISIONS-2026-09-17`.
- **The detached-grandchild fixture's mechanism** (an orphaned grandchild the job still holds, versus a
  `CREATE_BREAKAWAY_FROM_JOB` attempt that must fail because the job never grants `JOB_OBJECT_LIMIT_BREAKAWAY_OK`)
  — raised there too. What is decided: the job **does not** set `BREAKAWAY_OK`.
- **Which `StepError` variant an expired per-activity timeout produces** on the Harness path (`Cancelled { by:
  timeout }` vs `Timeout { after_secs }`) — routed at 7c and raised; this crate reports *timed out* as a value and
  the adapter classifies it.
- **Where the toolchain adapters' timeout comes from** (construction-time configuration, as routed at 7c, or a
  port parameter that would re-open Checkpoint 7a's six-method count) — raised. `run_bounded` takes a `timeout`
  argument and does not care who supplied it.
- **S-CANCEL-WIN's handle-leak question.** Design §9.1's counter-evidence stands: *"A tree kill — job or process
  group — reaps processes; it does not release an OS handle the dead process left behind."* This crate reaps
  processes; whether a killed `vitest` under a job still leaves an unremovable fixture tree is the spike's, run at
  7c against this crate's kill.
- **The kill of a run's descendants at gate exit** (AC-254/AC-257, 7d's composition root) — the primitive is here;
  the call site is not.

## Considered options

### Option — the toolchain crates depend on `rk-adapter-claude` for its containment module
Satisfies all three of 0099's clauses as worded and rejected on what it drags: `rk-adapter-claude` owns a tokio
multi-thread runtime (ADR-0076; `rt-multi-thread, process, io-util, time, sync, macros`), a stream decoder, the
denylist and a subscription-only constructor that refuses when `ANTHROPIC_API_KEY` is present (AC-488). A crate
whose job is `cargo test --workspace` would inherit every one of those into its dependency tree and its
conformance row, and tokio would enter every tree that holds a toolchain adapter — including the composition root's
in a build with no Harness. It also inverts the dependency direction: a Harness adapter is not a lower layer than a
toolchain adapter.

### Option — narrow ADR-0099's forbids to the Harness child
Would leave decision 3 in place and let the toolchain adapters spawn `cargo` and `vitest` outside a job. Rejected
because the hole is a known one, not a hypothetical: F22's survivor was a detached process that outlived its
parent by 12,000 s, `spikes/HANDOFF.md`'s defect (3) is a killed `vitest` on Windows, and AC-278's *"no child,
background, detached or daemon process belonging to that run remains alive"* is a property of the **run**, which
the verify activity's toolchain children belong to. A narrowing would be a supersession of the forbids that
weakens a measured safety property to keep a packaging sentence.

### Option — a second commented `#![allow(unsafe_code)]` in each toolchain crate
Rejected: ADR-0102 decision 2 says *"the only `allow` permitted in source"* and its forbids restate it; three
copies of the same FFI in three crates is three audit surfaces for one mechanism, and `s7c`'s *"exactly one"*
line would have to be rewritten to *"exactly three"*.

### Option — a safe-wrapper crate for the Job Object
Already rejected by ADR-0099 (*"trades ~30 lines of `unsafe` behind one commented allow for a dependency on the
audit surface"*); nothing here changes that reasoning, and the lock's `windows-sys` is the binding layer.

## Consequences

**Positive**
- ADR-0099's forbids holds for every child on both platforms, decision 2's token-less toolchain runs hold as
  worded, and ADR-0102's one-`allow` count holds — the three clauses are satisfied by a fourth thing rather than
  by weakening one of them.
- AC-224's enumeration, AC-278's external enumeration and AC-331's timed-out value have one home the kernel-side
  tests can reach without an adapter.
- `unsafe` stays in one file, in a crate small enough to read whole.

**Negative**
- A sixth crate, and a supersession-in-part of a record written six days ago. Design §9.1 is amended and owes a
  dated note; the register's AC-224 text (*"the `unsafe` FFI in one module of `rk-adapter-claude`"*) is stale in its
  parenthetical and takes a dated in-place amendment per D15-2, not an erratum.
- Two exact-pinned 0.x dependencies enter the workspace rows and are re-checked by `cargo deny` / `cargo audit`.
- The Windows arm is the live arm on this box and the POSIX arm is ARMED to a CI run that has never executed
  (`ci.yml` unrun); this record does not change that and the checkpoint says so.

**What this forbids**
- Forbids spawning any child from `rk-adapter-claude`, `rk-adapter-toolchain-node` or `rk-adapter-toolchain-rust`
  except through `rk-containment` **in those crates' library code** — a `Command::new` / `spawn` in a `src/` of
  those crates that does not go through it is a defect, not a shortcut. A test fixture binary that spawns its own
  descendant in order to exercise containment (the orphaned-grandchild and breakaway fixtures) is the **instrument**
  of AC-224 / AC-278, not a violation of this rule; each such spawn sits outside `src/` and carries a per-site
  `#[expect(clippy::disallowed_types, reason = …)]` naming what it is.
- Forbids a second `#![allow(unsafe_code)]` anywhere in `crates/`, and forbids `unsafe` outside `rk-containment`'s
  one module.
- Forbids `rk-kernel` depending on `rk-containment`, directly or transitively, and forbids `rk-containment`
  depending on tokio, `rk-ports`, or any `rk-adapter-*`.
- Forbids a Job Object created after the spawn, a child left unassigned until after it could have forked, a
  handle dropped before the child's lifetime ends, or `JOB_OBJECT_LIMIT_BREAKAWAY_OK` on the job.
- Forbids `taskkill`, a PID-tree walk, or any enumeration that is not via the job or the pgid.
- Forbids `run_bounded` reporting a timed-out or OS-killed run as an exit status.
- Forbids `nix`, a safe Job Object wrapper crate, or a caret range on `windows-sys` / `libc`.

## Evidence and confidence

- [measured 2026-09-17, `6152219`] five crates under `crates/`; `allow(unsafe_code)` **0** hits; `rk-containment`
  **0** hits; `windows-sys` 0.61.2 / 0.52.0 and `libc` 0.2.189 in `Cargo.lock`; `nix` and `windows` absent.
- [documented] ADR-0099 decisions 2 and 3 and its forbids; ADR-0102 decision 2 and forbids; ADR-0100 decision 2;
  AC-224, AC-225, AC-278, AC-331; design §9.1's `-claude` row and its counter-evidence paragraph; `s7b-ii`
  §7b-ii.14's AC-278 row.
- [measured, F22] one survivor, a detached `Start-Process`, alive 12,000 s past its parent — the hole a PID walk
  has and a job does not. [measured, PIA-010] the lifted runner's `taskkill /T /F` and its victim exit code of 1,
  n=1 — why the mechanism is a job and why AC-225 is keyed on the token. [documented, PIA-059] the Windows exit
  code is the adapter's own `TerminateProcess` code.
- **Confidence:** the conflict is a reading of three quoted clauses and is not in doubt; the mechanism is
  0099's, unchanged; the crate boundary is the operator's choice among three named options. Whether
  *assigned before it can fork* is literally achievable under `tokio::process::Command` is **not measured** and is
  the slice's to prove with the breakaway fixture — this record fixes the obligation, not the FFI sequence.
