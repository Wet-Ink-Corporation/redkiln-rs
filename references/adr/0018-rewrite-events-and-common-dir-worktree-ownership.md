---
id: "0018"
title: Rewrite events and common-dir worktree ownership
status: accepted
date: 2026-09-07
legacy_id: D18
source: references/2026-09-05-architecture-proposal.md §4.5
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

The durable-state design (§4.5) keys every repo-referencing row by SHA or tree hash, never by branch or path (rule 2, whose stated precedent is redkiln's verify-cache keying on `(cmd, base, tree)`). Because git history is mutable, that keying has a direct cost the source names: rebase, reset, amend and force-push produce *orphaned* rows, which must be surfaced as drift and never deleted silently. One store per repository lives at `git rev-parse --git-common-dir`/redkiln/ and is shared by every worktree of that repository. Two problems follow directly from that shape: git operations that rewrite history need a way to carry a row's identity forward across a SHA change, and a kernel that shares one store across worktrees needs a rule for which worktree is allowed to write to it. (That the shared shape would otherwise let two worktrees write conflicting tier-1 rows is this ADR's inference; §4.5 states the shared-store rule and the single-write-path rule but does not spell out that failure mode.)

## Decision drivers

- Rows are keyed by SHA/tree hash, never branch or path (§4.5 rule 2) — a rewrite changes the key without changing the underlying content, so history rewriting operations need explicit handling or rows are orphaned with no trace of why.
- Orphaned rows must be surfaced as drift, never deleted silently (§4.5 rule 2).
- One store per repository is shared by every worktree via the common dir (§4.5 rule 1), so nothing at the filesystem level prevents a second worktree from writing to it.
- The single-write-path rule: the kernel is the only writer of tier-1 commits (§4.5 rule 4).

## Considered options

### Option — path-keyed rows

Key durable rows by file path instead of SHA/tree hash. The source row names this as an alternative considered but records no reason for its loss; the only reason available anywhere in the source is §4.5 rule 2's flat rule that every repo-referencing row is keyed by SHA/tree hash, *never branch or path*. The source does not say what a path key would cost, and this ADR does not invent one.

### Option — trust the cwd

Let any process operating in any worktree write to the shared store without checking which worktree's branch is actually checked out, trusting that the caller's working directory is the intended target. This is the alternative the row records losing; the source row states no reason. Inferred from the surrounding rules, and marked as inference: it leaves the shared-store shape of §4.5 rule 1 with no ownership check at all, whereas the chosen rule makes the branch/HEAD match the precondition for writing. (§4.5 rule 4's own stated scope is narrower than this — it rules out git hooks feeding the store and states that the kernel is the only writer of tier-1 commits; it does not itself discuss two worktrees racing.)

## Decision

On rebase or amend, the kernel emits a `rewrite(old_sha → new_sha)` event rather than silently re-keying or dropping the row. At startup, the kernel resolves the repository's common dir (`git rev-parse --git-common-dir`) and refuses to write when the current worktree's HEAD is not the run's branch.

## Consequences

**Positive:**
- History-rewriting git operations (rebase, amend, and by extension reset and force-push per §4.5 rule 2) leave an explicit, queryable trail (`rewrite` events) instead of orphaning rows invisibly.
- Refusing to write from the wrong worktree gives the shared common-dir store an explicit ownership check, which "trust the cwd" would not have, using only state the kernel already resolves at startup (the common dir and the run's branch).

**Negative:**
- The kernel must resolve and check the common dir and branch identity on every startup, adding a check on the write path that a path-keyed or trust-the-cwd design would not need.
- A legitimate multi-worktree workflow where a human intentionally checks out the run's branch in a second worktree will be refused, not just an accidental one — the rule does not distinguish intent from mistake.

**What this forbids:**
- It forbids any worktree other than the one whose HEAD matches the run's branch from writing to the shared store, even if that worktree's operator believes they are working on the same task.
- It forbids treating a rewritten SHA as a fresh, unrelated row — the rewrite must be recorded as a rewrite event, not silently dropped or re-created from scratch.

## Evidence and confidence

- [documented] The rule that rows are keyed by SHA/tree hash and that rebase/reset/amend/force-push produce orphaned rows surfaced as drift is stated directly in §4.5 rule 2; no measurement is cited for this claim.
- [documented] The single-store-per-repository rule (§4.5 rule 1) and the single-write-path rule (§4.5 rule 4) are stated as design rules, not measured outcomes. Rule 1 cites prior art — redkiln's existing locks, Beads' worktree redirect and v1.0 common-dir placement — as precedent for the store's *placement*, not as evidence for this decision's ownership check; rule 4's cited grounds are git-branchless's non-atomic `reference-transaction` hook (its issue #1334), which is about hooks, not worktrees.
- No spike or measurement is cited for D18 itself. This decision is a design rule inferred from the surrounding consistency rules in §4.5, not a result validated against data in this cohort's sources.
