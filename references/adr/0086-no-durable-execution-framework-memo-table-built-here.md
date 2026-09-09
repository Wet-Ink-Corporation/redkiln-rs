---
id: "0086"
title: No durable-execution framework; the memo table is built here, re-argued on facts that hold
status: accepted
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §1 row 5, §3 (durable execution), §4 rec 9, §5; re-argues references/2026-08-31-rust-inversion-spike-charter.md ("Workflow-crate decision") and references/2026-09-05-architecture-proposal.md §6
supersedes: []
superseded_by: []
related: ["0003", "0017", "0021"]
evidence: ["F22", "F38", "F19"]
consumers: []
---

## Context and problem

The corpus has excluded every Rust durable-execution framework since the spike charter, and the architecture proposal's crate layout carries the consequence: "No durable-execution framework (Temporal/Restate mandate a server); the memo table + replayable log is ~800 LOC." The **conclusion is right and is not in question here**. The **rationale is not**: two of its three supporting facts were checked against live sources on 2026-09-08 and are false (evidence-augmentation §1 row 5, severity medium — "the conclusion stands, the rationale does not"; §4 rec 9).

- "`temporal-sdk-core` on crates.io is a 2021 alpha placeholder" — **withdrawn**. Temporal shipped a native Rust SDK in Public Preview: `temporalio-sdk` on `temporalio-sdk-core` v0.5.0, updated June 2026, actively maintained. The charter was reading the abandoned 2021 `temporal-sdk-core` 0.1.0-alpha.1 and drawing a currency conclusion from the wrong crate.
- "`graph-flow` hard-depends on `sqlx`+postgres" — **withdrawn**. graph-flow v0.6 (docs July 2026) makes postgres an **optional** Cargo feature and ships `InMemorySessionStorage` with no required database. Its mandatory dependencies are anyhow, async-trait, chrono, dashmap, serde, serde_json, thiserror, tokio, tracing and uuid.
- "Temporal, Restate and Obelisk each mandate a server, because arbitrating the journal *is* the product" — **holds**, and is now the load-bearing fact rather than one of three.

A decision resting on two retired facts would not survive an audit, and this set is the traceability spine for a build that has not started. The exclusion is therefore re-argued here as a decision of record, on facts that were checked this pass, together with the confirming evidence that strengthens what is kept.

## Decision drivers

- The record must be citable. A conclusion that is right for reasons that are demonstrably wrong is a defect in a decision spine, however sound the conclusion.
- The operating assumption in [0003](0003-five-named-ports-no-postgres.md) is "one machine per repo, git for sharing", with SQLite under `<git-common-dir>/redkiln/` and **no Postgres adapter**. Any dependency that reintroduces a server or a Postgres requirement crosses that decision, whether it arrives as a framework or as a feature flag.
- What this design actually needs is **memoisation keyed by step identity**, not deterministic replay of arbitrary user code. Steps here are coarse, side-effecting, minutes-long subprocesses. The frameworks sell the larger thing and charge a server for it.
- **Nothing may depend on a process surviving** (F22): roughly twelve background-job kills against a single 12,000 s detached survivor, and `issue-81` killed three times as a background job but completing in 116 s in the foreground. Any "durable" mode that lives only in process memory is disqualified by measurement, not by preference.
- The determinism ban list is worth keeping even without the framework, and it is now known to be convergent rather than vendor-specific.
- Rust remains second-class across every first-party durable-execution SDK checked, which is a standing reason to expect to build rather than adopt.

## Considered options

### Option — Temporal via `temporalio-sdk` (Rust, Public Preview)
The real, current, actively maintained Rust SDK — `temporalio-sdk` on `temporalio-sdk-core` v0.5.0, updated June 2026. Rejected, **and not for the reason previously given**: currency is no longer an objection and the "2021 alpha placeholder" claim is withdrawn. It loses on architecture. The Rust SDK still talks gRPC to a Temporal server or Temporal Cloud, and no embedded mode was found in this pass. That is a running service in the deployment picture for a product whose stated posture is one machine per repo with git for sharing (0003), and the capability being bought — arbitrated deterministic replay — is not the capability needed.

### Option — `graph-flow` v0.6
Rejected, **and not for the reason previously given**: the sqlx+postgres hard dependency claim is withdrawn; postgres is an optional feature and `InMemorySessionStorage` ships with no database. The charter's assessment of its *shape* was and remains correct — a pluggable `SessionStorage` trait, a one-step-then-persist runner, native suspend/resume — which is why reading its `storage.rs` and `runner.rs` is still the right preparation. It loses on the durability the name promises: the in-memory mode **does not survive a process restart**, so it is not durable in the sense this design requires, and F22 measures process death as a routine event rather than an edge case. Turning on the postgres feature to get real durability reintroduces exactly the server dependency 0003 excludes. The two supported configurations are therefore not-durable or not-permitted.

### Option — `iopsystems/durable`
New in this pass; not previously named, so not something the charter rejected. Genuinely embeddable, which is the property the others lack. Rejected on cost of entry: it requires **Postgres** — crossing 0003 directly — **and** compiling workflow code to WASM components via WASI, which imposes a build pipeline and an authoring constraint on every step for a benefit measured in ~800 LOC of memo table. Maturity is additionally unresolved: no version or release tags visible and roughly 58 stars (§5).

### Option — AWS's first-party Durable Execution SDK
Lambda-backed, distinct from Step Functions, shipped 2025/2026. Rejected on a fact that needs no argument: it supports TypeScript, Python, Java and C#, and is **explicitly not Rust**. Its value to this record is as evidence, not as an option.

### Option — keep the charter's rationale as written
Re-assert the exclusion on the original three facts. Rejected: two of them are false as of 2026, and the pass says so in terms. Re-asserting them would put a known-stale claim into the citable record and would leave the true objection — the server, and the durability gap — unstated and therefore unchallengeable.

## Decision

**No durable-execution framework enters redkiln-rs's dependency graph.** `temporalio-sdk`/`temporalio-sdk-core`, `graph-flow` and `iopsystems/durable` are each excluded, on the reasons recorded above and on no others. The memoisation table and replayable log are built in this repo against the `EventStore` port of 0003 (SQLite, WAL, `busy_timeout`, under `<git-common-dir>/redkiln/`), keyed per 0021's content-addressed, loop-aware step key.

**The retired facts are struck.** No redkiln document may argue this exclusion from "`temporal-sdk-core` is a 2021 alpha placeholder" or "`graph-flow` hard-depends on sqlx+postgres". Where those sentences appear they are corrected to the facts above, naming the live crates and versions.

**The exclusion rests on three surviving facts.** (1) Temporal's Rust SDK still requires a server or Temporal Cloud over gRPC, with no embedded mode found. (2) graph-flow's in-memory mode does not survive a process restart and so is not durable, while the postgres feature that would make it durable reintroduces the server. (3) `iopsystems/durable` is embeddable but requires Postgres and WASM-component compilation via WASI. Underlying all three: none of these frameworks sells memoisation keyed by step identity — they sell arbitrated deterministic replay, and the server is the arbiter.

**Adopt the determinism split as a convergent idiom, and cite it as such.** The workflow/activity separation is transplanted deliberately: the deterministic layer performs no wall-clock reads, no UUID generation, no HTTP/database/SDK calls, no random-number generation, and **no branching on any of these** — each is wrapped in a checkpointed step whose result is recorded and replayed. This is stated as convergent, not as a Temporal-ism, because AWS's first-party Durable Execution SDK independently re-derived the same ban list for its `step` primitive.

**One open item is registered, and it is scoped.** Nobody checked whether `temporalio-sdk` ships a time-skipping local test server, as some Temporal SDKs do (§5). If it does, it would soften "mandates a server" **for local development and testing only**. It is recorded as a testing-strategy question against the memo table's own test harness. It does **not** reopen the exclusion, and this record may not be cited as having checked it.

## Consequences

**Positive**
- The exclusion becomes auditable: every fact under it was checked against a live first-party source on 2026-09-08, and the two that failed are struck by name rather than quietly dropped.
- No server, no Postgres and no WASM toolchain enter the deployment picture, keeping 0003's "one machine per repo, git for sharing" assumption intact and the product a single binary.
- The determinism ban list gains independent, non-Temporal corroboration, so the workflow/activity split can be defended as a converged idiom rather than as one vendor's house style.
- The scope of what is built is honest and small — memoisation keyed by step identity, not deterministic replay of arbitrary code — which is what makes the ~800 LOC estimate credible.
- graph-flow stays useful as a design reference (`storage.rs`, `runner.rs`) precisely because its shape was assessed correctly; only the dependency claim was wrong.

**Negative**
- Everything the frameworks would have supplied is now this repo's to build and maintain: the memo table, the replayable log, retry and backoff, and the test harness for all of it. The ~800 LOC estimate is an estimate and has never been measured against a finished implementation.
- Redkiln takes on the determinism discipline without a runtime that enforces it. Temporal and AWS detect ban-list violations at replay; here a violation is a latent, possibly silent, memoisation bug.
- Resume is evidenced thinly. F38 is explicitly PARTIAL — 3 runs of a pre-registered 27+3 — and P8's state holds exactly one step, so the mechanism is demonstrated, not characterised.
- The design forgoes real capabilities that exist and are current: Temporal's Rust SDK is genuinely maintained, and this record chooses not to use it. That trade should be revisited if a server ever becomes acceptable for other reasons.

**What this forbids**
- Forbids adding `temporalio-sdk`, `temporalio-sdk-core`, `graph-flow` or `iopsystems/durable` to the dependency graph, in any crate, including dev-dependencies used to smuggle in a runtime.
- Forbids arguing the exclusion from either retired fact — "2021 alpha placeholder" or "hard-depends on sqlx+postgres" — in any document, commit message, spike pre-registration or activity card.
- Forbids describing `graph-flow`'s `InMemorySessionStorage`, or any in-process state, as durable. Durable means "survives a process restart", and F22 makes process death the expected case.
- Forbids a Postgres dependency arriving by a durable-execution feature flag; 0003's exclusion of a Postgres adapter is not satisfied by making the server optional at compile time.
- Forbids wall-clock reads, UUID generation, HTTP/database/SDK calls, random-number generation, or any branch on their results, inside the deterministic layer; each must be a checkpointed step.
- Forbids presenting the determinism ban list as Temporal-specific, now that it has an independent first-party re-derivation.
- Forbids claiming that `temporalio-sdk` has no local or time-skipping test mode: that was not checked, and the exclusion does not depend on it.
- Forbids re-litigating the exclusion on maturity or star-count grounds; the surviving reasons are architectural, and a framework becoming more mature does not answer any of them.

## Evidence and confidence

- [documented, 1P] `temporalio-sdk` on `temporalio-sdk-core` **v0.5.0**, updated June 2026, Public Preview, actively maintained — crates.io and `github.com/temporalio/sdk-rust`, read 2026-09-08. This is the fact that **falsifies** the corpus's "2021 alpha placeholder" claim, and it is the one place in this record where the external evidence directly contradicts the corpus. Acting on the external evidence, because the corpus claim is a currency claim about a crates.io listing and a same-day first-party read of that listing strictly dominates a 2026-08-31 reading of a different crate.
- [documented, 1P] `graph-flow` **v0.6**, docs July 2026: postgres is an optional Cargo feature; `InMemorySessionStorage` ships with no required database; mandatory deps anyhow / async-trait / chrono / dashmap / serde / serde_json / thiserror / tokio / tracing / uuid — crates.io, read 2026-09-08. Same disposition, same reason. The corpus's assessment of graph-flow's *shape* is not contradicted and is retained.
- [documented, 1P] Temporal's Rust SDK requires a server or Temporal Cloud, **no embedded mode found**; `iopsystems/durable` requires Postgres plus WASM-component compilation via WASI — evidence-augmentation §1 row 5 and §4 rec 9. Note the pass's own discipline: "no embedded mode found" is an absence from one pass's reading, so it licenses "no current source describes one", not "none exists". The exclusion does not turn on the absolute claim — a server that must be run for production use is disqualifying whether or not a test-only embedded mode exists.
- [documented, 1P] AWS shipped a first-party **Durable Execution SDK** (2025/2026), Lambda-backed, distinct from Step Functions, supporting TypeScript, Python, Java and C# and **explicitly not Rust**; its `step` primitive bans `Date.now`/`time.time`/`Instant.now`, UUID generation, HTTP/DB/SDK calls, `Math.random`, and branching on any of them — evidence-augmentation §2 (durable execution, claim 1) and §3. This is the **confirming** evidence: an independent vendor re-derived Temporal's ban list, so the split is convergent. It is documentation of two vendors' designs, not a measurement that the split is optimal.
- [measured] **F22**, operational: roughly twelve background-job kills against one 12,000 s survivor under `Start-Process`; `issue-81` killed three times as a background job, 116 s in the foreground. Licenses "nothing may depend on a process surviving", which is what disqualifies in-memory-only durability. The **cause is still unestablished**, so it does not license any claim about *why* the kills occur or that a different runtime would avoid them.
- [measured] **F38**, PARTIAL, n=3 of a pre-registered 27+3, $3.01: mid-graph resume across a real `taskkill /T /F` was lossless — 0 changed, 0 lost, digests identical — and cost 0.45×/0.55×/0.78× of the ~18.6k stage-gate figure. The 65-min and 24-h arms and all three controls are **unrun**, and the second-arrival question is circular by construction, so this licenses "the mechanism works at n=3" and nothing about steady-state behaviour.
- [measured] **F19**, n=174: suspension is lossless 174/174, with drift 12/174 = 7%. Stated limit carried through: **the FSM was never replayed**, so "same next action after resume" is unproven. The memo table this record commits to building is exactly the component that would have to make that guarantee.
- [documented, 1P] Stale-answer handling across every surveyed system is workflow-side **re-validation**, not signal-side invalidation; no surveyed platform invalidates a stale-but-unexpired token when upstream state changes — evidence-augmentation §3. This is why [0017](0017-gate-record-is-token-tree-hash-deadline.md)'s tree-hash binding is stronger than the surveyed platforms, and it is a reason the built table must own re-validation rather than expect it from a framework.

**What this record does not license.** It does not license any maturity judgment on `iopsystems/durable` — §5 records that as only partly answerable from a repo snapshot. It does not license the claim that `temporalio-sdk` lacks a time-skipping local test server; that was never checked, and if it exists it changes the testing strategy and not the exclusion. It does not license "no Rust durable-execution framework exists" — three do, and one is a maintained first-party SDK; the claim is that none of them fits **these** constraints. It does not validate the ~800 LOC estimate for the memo table, which is a projection with no implementation behind it. It does not establish that redkiln's own memo table will be more reliable than the excluded frameworks, only that it can be built without a server. And per the pass's discipline, nine agents finding nothing against the surviving three facts means "no current source contradicts them", not "confirmed".
