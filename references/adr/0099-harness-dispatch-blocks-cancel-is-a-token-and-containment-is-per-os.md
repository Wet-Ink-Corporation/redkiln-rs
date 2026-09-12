---
id: "0099"
title: Harness dispatch blocks and returns the record; cancel is a token the composition root owns; containment is a Job Object on Windows and a process group on POSIX; ports are Arc<dyn Port + Send + Sync>
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-010 (§3.4), PIA-045 (§4.1), PIA-059 (§4.5), PIA-038; references/adr/0076 §Decision; spikes/runner/src/step.rs, runner.rs; Microsoft Rust guidelines M-DI-HIERARCHY / M-MOCKABLE-SYSCALLS / M-AVOID-WRAPPERS (fetched 2026-09-10)
supersedes: ["0076"]
superseded_by: []
related: ["0003", "0083", "0094", "0095", "0105"]
evidence: ["PIA-010", "PIA-045", "PIA-059", "F22", "F38", "S-PORTS"]
consumers: []
---

## Context and problem

ADR-0076 keeps the port traits sync and says "any port whose operations can be long-running exposes a cancel path that
takes a handle minted when the operation started". Read against the rest of the design, that contract cannot be
written: a sync `dispatch` blocks to completion, so no caller can hold a handle while the call is in flight; the
trait's return leaves no slot for the AC-360 dispatch record; no `Send`/`Sync` bounds or trait-object shape are stated,
so whether the five ports can be held across the composition root's threads is unstated; and AC-225's "exits 143"
rests on a POSIX process-group kill that never obtains on Windows — the lifted runner does `taskkill /T /F` and the
one measurement is exit code 1, n=1 [measured, PIA-010]. `ChangeSummary`, blocked on S-BIN, has no stated home while
Checkpoint 5 greps `crates/` for its absence; the runbook names the Node adapter `rk-adapter-node` where §9.1 says
`rk-adapter-toolchain-node`; and about 95 criteria carry eight `rk-*` labels that are not crates [measured, PIA-038].
Microsoft's Rust guidelines do address the fake-vs-real dispatch question ADR-0076's evidence calls unsourced —
M-DI-HIERARCHY ranks `dyn` last after generics and enum dispatch, M-MOCKABLE-SYSCALLS and M-AVOID-WRAPPERS bear
on the boundary [documented, PIA-045]. Nothing was compiled (audit §8 M-01); **S-PORTS** is the throwaway crate that
checks this record's shape on the pinned toolchain.

This record supersedes ADR-0076 **in part**: the cancel clause and the evidence line on prior art. Sync traits, no
`.await` in the kernel, adapters owning their runtimes, and concurrency at the composition root all stand; ADR-0076
stays `accepted`.

## Decision drivers

- A sync boundary means the thing that can cancel is outside the call, so cancellation has to be a value handed in,
  not a handle handed out.
- F22: nothing may depend on a process surviving; F38: a kill mid-graph must be lossless. Both put the kill on the
  adapter's side of the boundary, keyed on something the composition root controls.
- Windows has no process group; a Job Object with kill-on-close is the primitive that reaps a tree when the parent
  dies, and it must exist before the child can fork.
- Fakes are scripted responders swapped at one root (ADR-0076); `dyn` is the boundary that makes that a one-line
  change, which is the reason to diverge from M-DI-HIERARCHY's ranking and say so.

## Decision

**1. The Harness port.**

```rust
pub trait Harness: Send + Sync {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome;
}
pub struct DispatchOutcome { pub record: DispatchRecord, pub result: Result<Typed, StepError> }
```

`dispatch` **blocks to completion**. The adapter's declared per-activity timeout (AC-223) is the in-band cancel. The
`record` is the AC-360 dispatch record (argv digest, env digest, instrument version, usage, timings, raw-retention
per ADR-0105); the `result` is the typed output or a `StepError` from ADR-0095's table. `Typed` is an erased,
validated value — `serde_json::Value` plus the schema id it validated against — which the kernel deserialises to the
step's declared output type. `ActivitySpec` carries `StepSpec`'s fields (card digest, tools, schema id, timeout,
inputs); `DispatchRecord` carries `StepRecord`'s. Both are `rk-types`.

**2. Cancellation is a `CancelToken` the composition root owns.** The token is constructed at the root, cloned into
the adapter at construction (never per call), and triggered by exactly two owners: the CLI's signal path (Ctrl-C /
SIGTERM) and the per-activity timeout. **The kernel never calls cancel.** When the token fires, the adapter kills the
child tree, and `dispatch` returns `Err(StepError::Cancelled { by })`; a cancel that races a completion is resolved by
the durable mark, not by the future's state (ADR-0076, unchanged). Only the Harness port carries cancellation in
Phase 1; the two Toolchain adapters run bounded commands under the same timeout without a token.

**3. Containment is per operating system, and both are named.** On **Windows**, the adapter creates a Job Object with
`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` **before** spawning, assigns the child to it before the child can fork, and holds
the handle for the child's lifetime, so parent death reaps the tree; the `unsafe` FFI lives in one module of
`rk-adapter-claude` under a commented `#![allow(unsafe_code)]` per ADR-0102's override rule. On **POSIX**, the adapter
spawns with `CommandExt::process_group(0)` and cancels with `killpg`. AC-224 enumerates the group via the job (Windows)
or the pgid (POSIX) and includes a detached grandchild fixture; AC-225's WHEN reads "terminated by the adapter's own
cancel" and its classifier is keyed on the **cancel token**, not on the exit code — "exits 143" is a POSIX
observation, recorded per platform.

**4. Ports are held as `Arc<dyn Port + Send + Sync>`**, with `Send + Sync` as supertraits on all five traits, so the
composition root may hold them across threads. This diverges from M-DI-HIERARCHY's ranking (generics, then enum
dispatch, then `dyn`) on a stated ground: the boundary exists so one fake per port swaps at one root, the call count
across it is small, and the fakes are scripted responders rather than sans-io mocks; `dyn` is the cheapest expression
of that and its cost is one vtable call per dispatch. The boundary is a bare trait object, not a newtype.

**5. Names.** Design §9.1 is the crate-name authority: `rk-adapter-toolchain-node`, `rk-adapter-toolchain-rust`
(ADR-0094), `rk-adapter-claude`, `rk-adapter-sqlite`. The AC register's `rk-*` label after each id is a **subject
tag**, mapped in its §0: `rk-select`, `rk-catalog`, `rk-packs`, `rk-policy`, `rk-kernel-verify` → modules of
`rk-kernel`; `rk-store-artifacts` → `rk-store`; `rk-port-toolchain` → `rk-ports`; `rk-doctor` → `rk-cli`.
`ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`** until S-BIN reports; Checkpoint 5's grep for its
absence covers `crates/` minus that one site.

## Considered options

### Option — `start() -> Handle` / `wait(Handle)`
Closer to ADR-0076's wording. Rejected: the kernel would own the child's lifetime across a `wait` that blocks anyway,
and the root still needs a token for Ctrl-C; two mechanisms for one kill.

### Option — cancel on Harness, Toolchain and DocsSite
Rejected for Phase 1: only the Harness spawns long-lived children; the toolchain adapters run bounded commands. A
port gains a token when a real adapter needs one, by successor.

### Option — a safe wrapper crate for the Job Object
Rejected narrowly: it trades ~30 lines of `unsafe` behind one commented allow for a dependency on the audit surface;
ADR-0102's override rule exists for exactly this case. Revisit if the module grows.

### Option — newtype-wrapped boundary per M-AVOID-WRAPPERS
Rejected: the guideline targets wrappers that hide a type's own API; the port trait *is* the API.

## Consequences

**Positive**
- The port compiles as stated (S-PORTS checks it), the record has a slot, the kill has an owner and a primitive on
  both platforms, and the criteria test the mechanism rather than an exit code.
- The crate-name and subject-tag confusion is resolved by one table.

**Negative**
- `unsafe` enters the workspace, in one file, under a commented allow — the exception ADR-0083 designed for.
- A blocking `dispatch` means fan-out is threads at the root (ADR-0076's rule); the root grows.
- `Typed` as an erased value means the kernel validates twice (adapter and deserialise); the second is cheap and is
  the closed-output guard.

**What this forbids**
- Forbids a `Harness::dispatch` that returns before the child has exited or been killed.
- Forbids the kernel calling `cancel`, and forbids a cancel path that is not the token.
- Forbids spawning a child on Windows outside a Job Object created before the spawn, and on POSIX outside a process
  group.
- Forbids keying AC-225's classifier on an exit code.
- Forbids a port trait without `Send + Sync` supertraits, and forbids holding a port other than as
  `Arc<dyn Port + Send + Sync>`.
- Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports.

## Evidence and confidence

- [measured, PIA-010] the unimplementable handle, the missing record slot, the missing bounds, the Windows gap; one
  measurement of `taskkill /T /F`'s victim exit code (1), n=1.
- [measured, F22] ~12 background kills against one survivor; [measured, F38] lossless resume across a real
  `taskkill /T /F` at one step, n=3.
- [documented, Microsoft Rust guidelines, fetched 2026-09-10] M-DI-HIERARCHY ranks `dyn` last; M-MOCKABLE-SYSCALLS
  and M-AVOID-WRAPPERS exist; the divergence is stated above.
- [documented, Win32] `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`; [documented, POSIX] `setpgid`/`killpg`.
- **What this evidence does not license.** It does not license claiming the trait shape compiles — S-PORTS does, on
  the pinned toolchain, and a failure yields a successor. It does not license any figure for kill latency or
  grandchild reaping on either platform; AC-224's fixture is the first measurement.
