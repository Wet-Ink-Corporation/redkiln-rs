---
id: "0077"
title: gix is restricted to the read path; the git write path shells out pending spike S-GIX
status: accepted
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §1 rows 2 and 8, §3 (dependencies), §4 recs 3, 4 and 6, §5 row 2
supersedes: []
superseded_by: ["0098"]
related: ["0003", "0012", "0018", "0084"]
evidence: ["F12", "F19"]
consumers: []
---

## Context and problem

Git is deliberately not a port (0003): trailers, worktrees, shallow clones for review and `reconcile()` are load-bearing and measured, and no alternative VCS is named anywhere in the corpus. That makes `rk-git` a concrete, non-substitutable component rather than an adapter behind a trait — and it makes the choice of *how* `rk-git` talks to git a real architecture decision with no escape hatch.

The Phase-1 design leans on `gix` for the whole job, including two things the external pass found gix does not currently claim.

**The common-dir gap.** The store lives at `<git-common-dir>/redkiln/`, one store shared by every worktree (0003, 0018). gitoxide's own `crate-status.md` states that "the delicate interplay between `GIT_COMMON_DIR` and `GIT_WORK_TREE` isn't implemented" — precisely the mechanism the store design leans on. This is a named gap in first-party documentation, not a hypothesis (evidence-augmentation §1 row 2, severity **high**; §4 rec 3).

**The trailer-write gap.** `crate-status.md` documents trailer *parsing*. No mature first-class trailer-*writing* API comparable to `git interpret-trailers --in-place` was found, and the agent could not fetch a definitive docs.rs API listing either way — so the absence is itself **UNVERIFIED** (§1 row 8, severity medium; §5 row 2, which calls this "the highest-value single check in the list, because git trailers are tier-1 state"). Trailers are how run state is written onto commits: F12 read 360 trailer commits across 74 slices to compute its result. A trailer write that silently does the wrong thing is a state-corruption class, not a bug class.

Two further facts bear on the shape. `git2-rs`/libgit2 remains more feature-complete for full git semantics, and gix has named gaps in push maturity, full merge workflows, hooks and the common-dir interplay (§3, dependencies). And gix's RUSTSEC history is dense and concentrated in **checkout and path handling**, several of them Windows-specific (§3, dependencies; 0084) — which is the disposable-worktree surface this design exercises hardest.

## Decision drivers

- Git trailers are **tier-1 state**. A write path whose correctness is unverified is not acceptable for tier-1 state, whatever its ergonomics.
- The common-dir/work-tree interplay is exactly the mechanism the store sits on, and its absence is stated by the library's own maintainers.
- `git` the executable is already a hard runtime dependency of this system's operating model ("one machine per repo, git for sharing", 0003), so shelling out adds no new class of dependency — only a new call site.
- Restricting gix to read operations happens to avoid the surface its advisory history concentrates in: `gix-worktree-state` checkout permissions, `gix-worktree` path traversal, and Windows reserved-device-name handling (0084).
- D12 (0012) forbids carrying an unknown as prose: an open question becomes a named spike with a named consumer, or it does not exist.
- Taking on libgit2 to close the gap would trade a documented Rust gap for a C toolchain plus a second FFI surface, and would not be reversible cheaply.

## Considered options

### Option — gix for everything, as the Phase-1 design assumed

Rejected on the library's own documentation. The common-dir/work-tree interplay is stated as not implemented, and that is the mechanism the shared store depends on; the trailer-write API could not be shown to exist. Proceeding would mean discovering both at integration time, on tier-1 state, inside the Phase-1 exit gate.

### Option — `git2-rs` / libgit2 for the whole job

Rejected. It is more feature-complete for full git semantics, but it introduces a C toolchain into the build, a second FFI surface, and its own advisory profile — and it would be adopted to solve a problem a `Command` invocation solves with no new dependency at all. Taking on C-toolchain risk and gix's worktree gap in the same phase is the trade §3 explicitly argues against.

### Option — shell out for everything, drop gix

Rejected, but narrowly. It would be simpler and it would work. It loses the read-side benefits that are real and already exercised: typed access to rev-parse and status without parsing porcelain, shallow clone by SHA (confirmed supported, gitoxide discussion #2309), and structured trailer parsing rather than a regex over a commit message. Parsing git's text output for the read path is the substitution, and it is worse.

### Option — block Phase 1 on the spike

Rejected. The fallback is known, cheap and unblocking; blocking a phase on a docs read that has a decided answer either way would be a hedge, not a gate.

## Decision

`rk-git` uses a **hybrid**, split by direction.

**gix owns the read path**, at the exact pin recorded in 0084:

- `rev-parse`, including resolving the repository common dir that 0018 requires at startup;
- repository and worktree **status**;
- **shallow clone by SHA** for review checkouts;
- **trailer parsing**.

**A process-exec wrapper owns the write path**, invoking the `git` executable:

- `git worktree add` and `git worktree remove` for the disposable per-item worktrees;
- `git interpret-trailers --in-place` and/or `git commit --trailer` for every trailer write.

**All shelling out goes through one wrapper type inside `rk-git`.** No `std::process::Command` for git exists anywhere else in the workspace. The wrapper captures stdout, stderr and the exit status, and maps a non-zero exit to a typed `rk-git` error — git's stderr text is diagnostic payload, never a control-flow signal parsed for meaning.

**The wrapper asserts its preconditions at startup**, not at first use: `git` is on `PATH`, and its version is at or above the minimum that supports the trailer flags this design uses. That minimum version is to be established and written into the assertion when the wrapper is built; it is not asserted here, because this pass did not verify it.

**Spike S-GIX is registered**, per 0012, with two arms and a named consumer:

| Spike | Question | Consumer | Gates Phase 1? |
| --- | --- | --- | --- |
| **S-GIX/a** | Against the pinned gix, can a store rooted at the git common dir be opened and written correctly from a linked worktree — i.e. does the `GIT_COMMON_DIR`/`GIT_WORK_TREE` interplay behave, or is `crate-status.md` still accurate? | `rk-git` worktree lifecycle | **No** — the shell-out fallback is decided. It gates only the *narrowing*. |
| **S-GIX/b** | Does the pinned gix expose a first-class trailer-**writing** API, and does a five-line unit test round-trip a trailer through it byte-identically to `git interpret-trailers --in-place`? | `rk-git` trailer write path (tier-1 state) | **No**, same reason. Highest-value single check in the external pass's list. |

A **go** on an arm is what may narrow this record; nothing else may. Narrowing is written as a successor ADR that supersedes this one in part, not as an edit.

## Consequences

**Positive**
- The tier-1 state path — trailer writes — runs on the reference implementation of the feature, whose semantics the parsing side is already being validated against.
- Worktree creation and removal, the operation with the least library support and the densest advisory history, is done by git itself.
- gix's exposure narrows to read operations, away from `gix-worktree-state` checkout and `gix-worktree` path handling, where its advisories cluster (0084).
- The unknown is registered, bounded and named, with a decided fallback, so Phase 1 proceeds without a hedge.

**Negative**
- Two git implementations live in one process, and they can disagree — notably about config resolution and path handling on Windows. Anything read by gix and then written by the executable must be re-read after the write, not assumed.
- `git` on `PATH` becomes an explicitly asserted runtime precondition, and its version becomes a compatibility surface the pure-gix design would not have had.
- A process spawn per trailer write and per worktree operation. F12's 360 trailer commits across 74 slices is the order of magnitude; this is not a hot loop, but it is not free either.
- Errors from the write path arrive as exit codes plus text, so the typed error surface is coarser on the write side than on the read side.
- The minimum required `git` version is not established in this record, and until it is, the startup assertion is incomplete.

**What this forbids**
- Forbids `gix` for `worktree add` or `worktree remove`.
- Forbids `gix` for writing a trailer until S-GIX/b returns go, no matter how plausible an API looks.
- Forbids treating documented trailer **parsing** support as evidence of trailer **writing** support.
- Forbids any `std::process::Command` invocation of `git` outside `rk-git`'s single wrapper.
- Forbids parsing git's stderr to decide control flow.
- Forbids adopting `git2-rs`/libgit2 as the remedy for a gix gap without a new ADR.
- Forbids narrowing this record on the strength of a docs read alone: the arms of S-GIX are defined as executed tests against the pin.
- Forbids leaving S-GIX unnamed or consumer-less in the spike register (0012).

## Evidence and confidence

- [documented] gitoxide `crate-status.md`: "the delicate interplay between `GIT_COMMON_DIR` and `GIT_WORK_TREE` isn't implemented." n = 1, first-party, evidence-augmentation §1 row 2. This is a statement about what the library documents, not a measurement of failure — nobody has run the scenario and watched it break. The decision is a precaution against a documented gap, not a reaction to an observed one.
- [documented, absence UNVERIFIED] `crate-status.md` documents trailer parsing; no first-class trailer-writing API was found, **and the pass could not fetch a definitive docs.rs API listing either way** (§1 row 8; §5 row 2). The absence is explicitly unverified. This is why the decision is a fallback with a registered spike rather than a finding that gix cannot write trailers.
- [documented] Shallow clone at a specific commit is supported — [Pr] gitoxide discussion #2309 (§3, dependencies). One discussion thread, not a docs guarantee.
- [documented] `git2-rs` remains more feature-complete; gix has named gaps in push maturity, full merge workflows, hooks and the common-dir interplay (§3, dependencies).
- [measured] F12: 360 trailer commits across 74 slices; ever-rejected 15/74 = 20.3%. This establishes that trailers carry real run state at real volume. It does **not** measure any gix or shell-out behaviour — F12's instrument was the existing system, and its registered primary metric was uncomputable and re-scoped.
- [measured] F19: drift 12/174 = 7% overall, 3.6× more likely on gates over one hour, forcing `reconcile()` on every entry. Relevant because `reconcile()` is a read-path operation and therefore lands on gix; F19 does not license any claim about which library performs it correctly.
- **Corpus vs external:** the corpus assigned gix the whole job, including worktree management under a shared common dir and trailer writing. The external pass contradicts the first at severity high on first-party evidence and flags the second at severity medium with the absence unverified. This record acts on the external evidence for the write path, and keeps the corpus's gix bet for the read path, where nothing contradicts it.
