---
id: "0105"
title: rk-git and rk-store contracts as rules — the GitOps seam, hooks isolation, trailer plumbing, the tree_hash algorithm, the wrapper's write set, one writer with a bounded busy_timeout, an event-kind table, and retention
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-040, PIA-043, PIA-050 (§4.6), PIA-035; references/adr/0018, 0077, 0084, 0098; git interpret-trailers(1) `ifExists`; sqlite.org WAL; docs.rs rusqlite 0.40.2 (fetched 2026-09-10)
supersedes: []
superseded_by: ["0116"]
related: ["0013", "0017", "0018", "0077", "0084", "0086", "0095", "0097", "0098", "0099"]
evidence: ["PIA-040", "PIA-043", "PIA-050", "F12", "F19", "F20", "F38"]
consumers: []
---

## Context and problem

Step 6's contracts for `rk-git` and `rk-store` are lists of operations, not rules; the audit counted eighteen
behaviours left to the implementer [measured, PIA-040]. Among them: who executes a git op the kernel's replay must
not re-run; whether a trailer commit runs the target repository's hooks; what the answer record's trailer key, value,
ref and plumbing are; whether a worktree is reused or re-created and what a failed `worktree remove` is; the `\\?\`
path form that broke `git fetch` (F20/2); how `tree_hash` is derived over a dirty worktree (AC-245's "not of the index
at a later instant" says what it is not); which write operations the wrapper owns; whether the review clone is bare;
`busy_timeout`'s value and the transaction bound it implies; whether the one writer is a connection or a queue; the
event vocabulary; what a dispatch record retains verbatim; and the `Send`/`Sync` bounds on the port objects. Two
Step-6 doctor rules rest on undefined terms — AC-184's "tier-1 projection" never places orphans or rewrite events, and
AC-159's "store write record" disagrees with its prose [measured, PIA-043]. Gate re-reach, escalation chains and the
empty-commit tree hash are undefined [measured, PIA-050]. One rider from the external pass has a direct consequence:
`git interpret-trailers` defaults `ifExists` to `addIfDifferentNeighbor`, so a repeated identical `Gate:` or
`Slice-Verdict:` trailer is silently **not appended** unless `--if-exists=add` is passed [documented].

ADR-0018 (rewrite events; refuse to write when the current worktree's HEAD is not the run's branch) and ADR-0077 (the
hybrid) stand; this record is their companion and adds rules they leave open. Where it narrows ADR-0018's "current
worktree" it says so in clause 8.

## Decision drivers

- A replayable kernel (ADR-0076) cannot re-run git; the git ops it observed must come back from a journal.
- Tier-1 state written through the target repository's hooks is tier-1 state a hook can rewrite.
- A trailer the wrapper believes it wrote and git silently dropped is the worst kind of missing record.
- `tree_hash` binds a gate to a tree; two derivations bind it to two trees.

## Decision

**1. Git operations on the activity side go through a journalled `GitOps` seam.** `GitOps` is a **struct** (so
AC-483's port-inventory denylist stands) inside `rk-git` that records every git invocation and its result to the
EventStore; on replay the kernel is served the journal and `rk-git` performs **zero** writes (AC-006 and AC-008 gain
"and zero rk-git writes").

**2. Hooks are isolated.** Every commit the wrapper makes runs with `-c core.hooksPath=<empty directory>`; the
target repository's hooks never run on a Redkiln commit. AC-162 gains a fault-injection fixture (a hook that rewrites
the message) that must have no effect.

**3. Trailer plumbing.** The answer record is written as a trailer commit: `git commit --allow-empty --trailer
"<Key>: <value>"` through the wrapper, with `-c trailer.ifexists=add` (and `--if-exists=add` on any
`interpret-trailers` call), so a repeated identical trailer is appended, never dropped. Keys are `Story:`,
`Slice-Verdict:`, `Gate:` (the legacy `Gate:` drift rule stands); values follow the trailer grammar table in design
§5.4 (key, value ABNF, column anchoring, the verdict vocabulary, the unknown-key rule); AC-444's drift counts are
pinned to that table. The commit's ref is the run's branch; plumbing is `commit-tree` + `update-ref` where the
wrapper needs an exact parent.

**4. `tree_hash` is one algorithm, written once, used at gate open and at answer.** When a checkpoint commit is due,
the recorded tree is that commit's tree. Otherwise it is `git write-tree` on a **temporary index** built from the
current index plus tracked modifications; **untracked files are excluded**, and the design says so. The same
derivation is used for AC-245's `gate_opened`, for the answer record and for AC-254's diff ("commits since the trailer
commit, and paths differing between the two trees"). The op is on the wrapper's write-side list.

**5. The wrapper's write set is enumerated**: `worktree add`/`remove`; `add`; `commit` (incl. `--allow-empty`,
`--trailer`); `interpret-trailers`; `write-tree` on a temporary index; `patch-id`; fast-forward/merge on the
integration ref; `clone` removal; `rev-parse --git-common-dir` on S-GIX/a's no-go (ADR-0098). Anything not listed is
a successor. The **review clone is bare** unless the review activity declares it needs a working tree.

**6. Worktrees.** One worktree per concurrent item (Halt 6). A worktree is **re-created**, never reused, per run; a
failed `worktree remove` (the Windows handle case) is a typed `rk-git` error and a `doctor` orphans row, with
S-CANCEL-WIN's fixture as the regression. Paths are passed to git in **drive-letter form**, never `\\?\`; the
`git_remote_path` normaliser is ported under Step 6.2 constraint 4 with a Windows unit test (F20/2).

**7. rk-store.** `busy_timeout` = **5000 ms** (rusqlite's default; R-SQLITE confirms); every write transaction is
shorter than it — a rebuild writes to a separate file and renames. **One writer connection per process, behind a
mutex, no queue**; reader connections are one per thread and also set `busy_timeout` (readers can see
`SQLITE_BUSY`); `doctor` checks the common dir is not on a network filesystem (WAL does not work over one). The
**event-kind table** (kind · tier · payload fields · producing step) lives in design §5 with `rk-types` as the owning
crate; ADR-0095's `rate_limited`, `step_refused` and `step_cancelled` are rows in it. **Retention:** a dispatch
record keeps argv and environment as **digests only**, stdout and stderr **verbatim up to a size cap** (then digest
+ head/tail), the child environment **never**; `doctor` carries a rule row for credential-shaped values in any
retained field.

**8. Ownership.** A tier-1 write resolves the run-owned worktree **from the store**, regardless of cwd, and a target
that is not run-owned is refused naming its path and branch — a new criterion under AC §6.4. This narrows ADR-0018's
"current worktree" to "the worktree the store owns for the run"; AC-181's GIVEN reads "of two different items".
AC-193: removal is the entry step's action and `reconcile()` only reports drift; AC-172: `status` does not reconcile.

**9. Doctor terms.** AC-184's **tier-1 projection** = the tier-1 rows whose anchor is reachable from the clone's
refs; orphan rows and rewrite events are tier-2-derived and sit **outside** the AC-184/SM-051/SM-056 rebuild
differential, with a rebased-commit fixture. AC-159's GIVEN reads "a managed artifact whose structured-section or
frontmatter digest differs".

**10. Gates.** A loop-back that re-reaches a gate mints a **fresh** `(token, tree_hash, deadline)` (ADR-0087 cl. 4;
PROVISIONAL until P8-B) — a new criterion outside AC-240..279; AC-271/SM-078 are duplicate-answer instruments and a
`gate_opened`-per-(item, gate step) counter is the discovery instrument. An `escalate` gate's expiry action may not
itself be `escalate` (AC-260 lint row); chains terminate in `fail` or an auto-decide within one hop.

**11. Ports are `Arc<dyn Port + Send + Sync>`** (ADR-0099); `rusqlite::Connection` is `Send` and `!Sync`, which is why
clause 7's writer is thread-owned behind a mutex.

## Considered options

### Option — write these as Step 6 constraints in the runbook only
Rejected: nothing in the forbids machinery would protect them, and PIA-082's pattern — DECIDED rules with no ADR —
would recur on the first refactor.

### Option — a queue for the single writer
Rejected: a queue is a second place a write can be lost across a kill (F22); a mutex around one connection is the
smallest thing that serialises writers.

### Option — reuse worktrees across runs
Rejected: Halt 6 was written on a Windows handle that survived `Stop-Process`; a fresh worktree per run is the only
state the kernel can assert.

## Consequences

**Positive**
- Eighteen implementer choices become eleven rules with fixtures; the trailer-drop hazard is closed by a flag.

**Negative**
- The event-kind table and the trailer grammar table are two design sections to keep in step with `rk-types`.
- Digest-only argv/env retention means a failed dispatch cannot be replayed from its record alone; that is the
  point of the credential rule.

**What this forbids**
- Forbids `rk-git` performing any write during replay.
- Forbids a Redkiln commit running the target repository's hooks.
- Forbids an `interpret-trailers`/`--trailer` invocation without `ifExists = add`.
- Forbids a second `tree_hash` derivation, and forbids including untracked files in it.
- Forbids a wrapper write not on clause 5's list, a `\\?\` path handed to git, and a reused worktree.
- Forbids a write transaction longer than `busy_timeout`, a second writer connection, and a retained child
  environment.
- Forbids a tier-1 write to a worktree the store does not own for the run.
- Forbids an `escalate` expiry action on an `escalate` gate.

## Evidence and confidence

- [documented, git-scm interpret-trailers(1)] `trailer.ifexists` defaults to `addIfDifferentNeighbor`.
- [documented, sqlite.org] WAL "only one writer at a time"; WAL over a network filesystem unsupported;
  [documented, docs.rs rusqlite 0.40.2] `busy_timeout` default 5000 ms; `Connection: Send + !Sync`.
- [measured, F20/2] a `\\?\` path handed to `git fetch` failed; [measured, F12] 360 trailer commits / 74 slices with
  the legacy `Gate:` drift; [measured, F19] 174/174 lossless activity-grain suspensions; [measured, F38] one-step
  mid-graph resume lossless at n=3.
- [measured, PIA-040, PIA-043, PIA-050, PIA-035] the eighteen behaviours, the two undefined doctor terms, the three
  gate gaps, the checkpoint lines.
- **What this evidence does not license.** It does not license claiming the write-tree derivation is cheap on large
  trees, or that 5000 ms suffices under the self-hosting run's load — R-SQLITE and Step 8 (3) are the measurements.
