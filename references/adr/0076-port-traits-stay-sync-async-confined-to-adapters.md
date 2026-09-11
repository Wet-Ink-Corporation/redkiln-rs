---
id: "0076"
title: Port traits stay sync; async is confined to adapters behind an explicit cancel
status: accepted
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §1 row 3, §3 (Rust practice), §4 rec 5, §5 row 1
supersedes: []
superseded_by: ["0099"]
related: ["0003", "0083"]
evidence: ["F19", "F22", "F38"]
consumers: []
---

## Context and problem

D3 (0003) fixes five ports — `EventStore`, `Harness`, `Tracker`, `DocsSite`, `Toolchain` — each with one real adapter and one fake, and the kernel is described throughout the corpus as dyn-dispatched with "no generics leaking into the core". Three of those five ports do I/O that is naturally asynchronous: the harness spawns and streams from `claude -p`, the tracker talks HTTP, the docs site shells out.

The external pass makes the tension explicit. Native `async fn` in traits has been stable since Rust 1.75, but it is **still not dyn-compatible**; `async_fn_in_dyn_trait` remains nightly-only, seen in nightly symbol tables through roughly 1.93–1.96 with no stabilisation announcement found. Only statically dispatched (generic / `impl Trait`) async ports can use native async fn in trait. So "one trait per port, no generics leaking into the kernel" and "dyn-dispatched port" are in direct tension the moment a port method is `async` (evidence-augmentation §1 row 3, severity **high**; §4 rec 5). The pass also notes that hexagonal/ports-and-adapters has no dedicated page in the Rust API Guidelines or the Microsoft Pragmatic Rust Guidelines, and that this async-object-safety gap is the pattern's known Rust failure mode (§3, Rust practice).

The corpus never recorded a choice here. It has to be recorded, because the two ways out have different costs and different blast radii, and one of them silently makes the kernel async.

## Decision drivers

- The kernel is a **deterministic, replayable core**. Suspension is lossless at activity granularity (F19, 174/174) and at mid-graph step granularity across a real `taskkill /T /F` (F38, 3/3, digests identical). A replayable core has no business owning an executor, and every `await` inside it is a scheduling decision that replay must not depend on.
- **Nothing may depend on a process surviving** (F22): ~12 background-job kills against one 12,000 s survivor under `Start-Process`; `issue-81` was killed three times as a background job and completed in 116 s in the foreground. Every wait is therefore a durable mark plus a later read, not an in-flight future.
- Cancellation for the harness port is a **process-group kill**, not a dropped future. Dropping a boxed future does not reap a spawned child process tree; an explicit kill is required under either option, so async-trait's implicit drop-cancellation buys nothing here.
- `async-trait` is a proc-macro dependency in the compile graph of every crate that names a port, and costs one heap-allocated `Pin<Box<dyn Future>>` per call across the boundary the whole system funnels through.
- The dyn boundary is load-bearing for the fakes: one fake per port, swapped at the composition root, is how the kernel is tested at all.
- The evidence on stabilisation is **search-only**. Nobody confirmed whether Rust 1.97 specifically shipped a dyn-async-trait stabilisation (§5 row 1), so the decision must be safe under the assumption that it did not, and must name the check that would settle it.

## Considered options

### Option — `async-trait` on every dyn port

Annotate all five port traits with `#[async_trait]`, keep every port method `async`, and let the kernel `await` across the boundary. This is the shape most hexagonal Rust write-ups assume.

It loses on three counts. It makes the kernel async, which puts an executor inside a core whose defining property is deterministic replay (F19, F38) — the replay path would then depend on task scheduling that no event log records. It pays a boxed future per call on the one boundary every operation crosses. And it does not actually deliver the cancellation it appears to: killing a `claude -p` process group still needs an explicit call, because dropping the future that was awaiting the child does not reap the child (F22 is the measured shape of that failure). It buys ergonomics the kernel does not want and does not buy the semantics the harness does want.

### Option — generic (statically dispatched) ports with native `async fn` in trait

Make the kernel generic over `H: Harness, T: Tracker, …` so native async-fn-in-trait applies with no boxing and no proc macro. Rejected: this is precisely the "generics leak into the kernel" that 0003's port design exists to prevent. It also monomorphises the whole kernel per adapter set, and forces every kernel type that stores a port to carry its type parameters — the fake/real swap stops being a value substitution and becomes a type-level one, which spreads through every signature.

### Option — wait for `async_fn_in_dyn_trait`

Defer and build on nightly. Rejected: it is nightly-only with no stabilisation announcement found, and the pass could not even confirm the state of the 1.97 line (§5 row 1). Building the Phase-1 kernel on an unstable feature makes the toolchain pin of 0083 a hostage.

## Decision

**Every port trait declared for the kernel is sync.** No port method is `async fn`, and no port method returns a future. The kernel contains no `.await`, holds no runtime handle, and depends on no async runtime crate.

**Every `.await` lives inside an adapter**, behind a sync method. An adapter that needs async I/O owns its own runtime handle — constructed once at the composition root, never per call — and blocks on it inside the adapter's own method. An adapter must not call `Handle::block_on` from inside a thread that runtime is already driving; it owns a dedicated runtime, or blocks off-runtime, and that constraint is documented on the adapter type.

**Cancellation is an explicit port operation, not a dropped future.** Any port whose operations can be long-running exposes a cancel path that takes a handle minted when the operation started, and the adapter implements it as a real kill — for the harness adapter, a process-group kill of the spawned `claude -p` tree. The kernel's model of a pending long operation is a durable mark plus a later read (F22), so a cancel that races a completion is resolved by the mark, not by the future's state.

**Concurrency across ports is the composition root's job, not the kernel's.** Where fan-out is needed, it is expressed as threads or as an adapter-internal concurrent call, and its results enter the kernel as values in a defined order. The kernel never observes interleaving.

**Working assumption, recorded as such:** as of 2026-09-08, a dyn-compatible native async fn in trait does not exist on stable Rust. **The check that settles it** is a direct read of the Rust release notes for the version pinned in `rust-toolchain.toml` (0083) plus the `async_fn_in_dyn_trait` tracking issue, performed as part of the review of any toolchain-pin bump. That review is the named consumer; the check does not gate Phase 1, because this decision is correct under either answer.

## Consequences

**Positive**
- The kernel stays synchronous and deterministic, which is what makes F19's lossless suspension and F38's mid-graph resume expressible as replay over an event log rather than as executor state.
- No proc-macro dependency and no per-call `Pin<Box<dyn Future>>` on the boundary every operation crosses; the port traits remain plainly dyn-compatible with no macro in the way.
- The fakes stay trivial: a scripted responder or an in-memory `Vec` implements a sync trait with no runtime, so kernel tests need no async test harness.
- Cancellation is explicit and therefore testable. A process-group kill is a call with a result, not a hoped-for consequence of a drop.

**Negative**
- Adapters block the calling thread. A kernel operation that fans out to three ports takes the sum of their latencies unless the composition root introduces threads.
- The `Handle::block_on`-inside-a-runtime-thread hazard is real and is now an adapter-authoring rule enforced by review rather than by the type system.
- Streaming responses (the `-p` stream events of F4) must be consumed inside the adapter and surfaced as a completed value or through an explicit sync callback; the kernel cannot iterate a stream.
- Timeouts move into adapters, so each async adapter carries its own timeout policy and they can drift apart. The timeout history in the spike record (900 s → 2,400 → 3,600 → one 5,400 s retry) is a warning that this policy needs one home per adapter and a written default.

**What this forbids**
- Forbids `async fn` — native or `#[async_trait]` — in any trait the kernel names.
- Forbids a port method returning `impl Future`, `Pin<Box<dyn Future>>`, or any other deferred value.
- Forbids the kernel depending on `tokio`, `async-std`, `futures`, or `async-trait`.
- Forbids making the kernel generic over its ports to obtain native async-fn-in-trait; 0003's dyn boundary stands.
- Forbids treating a dropped future, a cancelled task, or a closed channel as a cancellation of external work — a cancel is an explicit call ending in a process-group kill.
- Forbids building the Phase-1 kernel on the nightly `async_fn_in_dyn_trait` feature.

## Evidence and confidence

- [documented] Native async fn in trait is stable since 1.75 and is **not** dyn-compatible; `async_fn_in_dyn_trait` remains nightly-only, seen in nightly symbol tables ~1.93–1.96, no stabilisation announcement found. n = 3 sources ([1P] rust-lang.github.io/async-fundamentals-initiative, docs.rs/async-trait, [blog] debasishg.github.io), evidence-augmentation §1 row 3. This does **not** license the statement that no stabilisation exists — the pass is explicit that it did not read the Rust 1.97 release notes (§5 row 1), and that "still requires `async-trait` for dyn ports" is the safe working assumption, not a confirmed fact.
- [measured] F22: ~12 background-job kills against one 12,000 s survivor under `Start-Process`; `issue-81` killed three times as a background job, 116 s in the foreground. Operational, n small, **cause still unestablished** — it licenses "do not depend on a process surviving", not any claim about why processes died.
- [measured] F19: suspension lossless 174/174; F38: mid-graph resume lossless across a real process kill, 3/3, digests identical. Both support a replayable core. F19's stated limit is carried here: **the FSM was never replayed**, so "same next action after resume" is unproven; F38 is PARTIAL, its state holds exactly one step, and its 65-minute and 24-hour arms and all three controls are unrun. These findings motivate keeping the kernel deterministic; they do not prove the kernel is replayable today.
- [inferred] That dropping a future does not reap a spawned process tree is standard Rust/OS semantics, not something this pass measured. It is stated here as reasoning, not as a citation.
- [documented] Hexagonal ports-and-adapters has no dedicated page in the Rust API Guidelines or Microsoft's Pragmatic Rust Guidelines; the "no generics leak, one fake per port, dyn dispatch" description is practitioner synthesis (evidence-augmentation §3, Rust practice). This record therefore does not cite an external norm for the port shape — only for the async-dispatch fact that constrains it.
- **Corpus vs external:** the corpus implied a dyn-dispatched async port needed no `async-trait` under current Rust. The external pass contradicts that at severity high. This record acts on the external evidence, because the corpus claim rests on the 1.75 stabilisation being read as dyn-compatibility, which it is not.
