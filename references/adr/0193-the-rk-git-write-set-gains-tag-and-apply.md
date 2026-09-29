---
id: "0193"
title: "**The `rk-git` write set gains `tag` and `apply`, each scoped, and clause 5's consolidated list is stated once** (`s8g`'s write-set record, R3; D21-42 6b, 10c; D21-47's one reading of clause 5): `GitOp::Tag` makes one annotated local tag `rk-release/<item>` on a commit of the run-owned branch and is never pushed; `GitOp::Apply` applies a criterion's declared mutation patch only in the verify pass's ephemeral `<item>.mut` tree. Supersedes `0105` clause 5 in part, for these two writes, beside `0180` decision 15 and `0188` decision 10, the other two partial supersessions of the clause"
status: proposed
date: 2026-09-29
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (row 6b — *"The tag is local and never pushed by the run."*; row 10c), D21-47 (*"**ADR-0105 clause 5 has one reading.** A git WRITE not on its closed list is a supersession in part. That covers s8f's `commit-tree`/`update-ref`, s8g's `Tag`/`Apply`, and s8d-ii's lock, unlock, marker, reset and clean. A READ is not a supersession"*; *"Records are located by the id recorded at write time"*); references/adr/0105 (clause 5 and its forbids bullet), 0180 (decision 15), 0188 (decision 10), 0104 (decision 1); runbook/phase-1/s8g-verify-and-restored-demands.md §8g.5, §8g.7, §8g.8 (R3's row)
supersedes: ["0105"]
superseded_by: []
related: ["0104", "0114", "0162", "0180", "0188", "0191", "0195"]
evidence: []
consumers: []
---

## Context and problem

**ADR-0105 clause 5 is a closed list.** It reads *"**5. The wrapper's write set is enumerated**: `worktree
add`/`remove`; `add`; `commit` (incl. `--allow-empty`, `--trailer`); `interpret-trailers`; `write-tree` on a temporary
index; `patch-id`; fast-forward/merge on the integration ref; `clone` removal; `rev-parse --git-common-dir` on S-GIX/a's
no-go (ADR-0098). Anything not listed is a successor."* Its forbids bullet reads *"Forbids a wrapper write not on
clause 5's list"*, and `crates/rk-git/src/git_exec.rs` says *"Anything not here is a successor record, and adding a
variant is that decision made visible"*.

**Two of this stage's producers need a git write the list lacks.** `tag-release` (R5, ADR-0195) makes a release tag
on the landed commit (D21-42 6b), and the verify pass's mutation harness (R1, ADR-0191, decision 11) applies a
criterion's declared patch in a disposable tree (ADR-0104 decision 1: *"The harness applies the patch at the candidate
SHA, runs the suite, reverts, and records the outcome."*). Neither `tag` nor `apply` is on the list.

**This is the third record to supersede clause 5 in part.** D21-47 gives the clause one reading, and two records have
already taken it: ADR-0180 (`s8d-ii`'s worktree record, decision 15) and ADR-0188 (`s8f`'s landing record, decision
10, accepted on D21-117). A reader of ADR-0105 should find one enumeration, not three partial ones.

## Decision drivers

- D21-47: a git write not on the closed list is a supersession in part; a read is not.
- Every write runs through the one `GitOps` wrapper; none on replay.
- The narrowest scope that serves the producer: a tag on the run-owned branch only, a patch in the ephemeral tree only.
- Records are located by the id recorded at write time (D21-47), so the other two are cited by id.

## Considered options

### Option — two new `GitOp` variants, each scoped, and the consolidated list stated here (taken)

Taken: it makes each write visible as a variant (git_exec.rs's rule), scopes it to the one tree or ref its producer
needs, and gives clause 5 a single enumeration.

### Option — shell `git tag` and `git apply` from the toolchain adapter or the harness

Rejected: ADR-0105's bullet forbids a wrapper write not on the list, and a write outside the wrapper is invisible to
replay and to `rk-git`'s journal (D21-43 N7's `git_op` row).

### Option — apply the patch by rewriting files in Rust, with no git write

Rejected: it re-implements `git apply`'s context matching, and a divergence from git's reading of the declared
unified diff would make `mutation-survived` a property of the re-implementation, not of the suite.

## Decision

1. **`GitOp::Tag`** — `git tag -a rk-release/<item> -m <message> <commit>`, annotated and local. The target must be a
   commit on the run-owned branch `rk-run/<item>` (`authorise_tier1_write`'s ownership pair); any other target
   refuses by name. Re-tagging the same item at the same commit is idempotent; at a different commit it refuses by
   name. **`GitOp` has no push variant, and this record adds none.**

2. **`GitOp::Apply`** — `git apply` of a criterion's declared `mutation` patch, run **only** in the verify pass's
   ephemeral `<item>.mut` worktree under `.redkiln/worktrees/` (R1, decision 11), never in the run-owned worktree and
   never in the main worktree. The revert is the tree's removal, on every path.

3. **ADR-0105 clause 5 is superseded in part, for these two writes.** Clause 5's *"Anything not listed is a
   successor."* and its bullet *"Forbids a wrapper write not on clause 5's list"* admit `GitOp::Tag` and
   `GitOp::Apply` within the scopes above. ADR-0105 keeps `status: accepted` and gains this record's id in
   `superseded_by`.

4. **The consolidated clause-5 write set**, each part in its own record and no operation in two:
   - **ADR-0105 clause 5 itself:** `worktree add`/`remove`; `add`; `commit` (incl. `--allow-empty`, `--trailer`);
     `interpret-trailers`; `write-tree` on a temporary index; `patch-id`; fast-forward/merge on the integration ref;
     `clone` removal; `rev-parse --git-common-dir` on S-GIX/a's no-go.
   - **ADR-0180 decision 15** (`s8d-ii`'s worktree record): `worktree lock` and `worktree unlock`; the discard's
     `reset --hard HEAD` and `clean -fd` (and, per ADR-0188 decision 10's dated note, its pathspec-narrowed
     `clean -f -- <path>…` read as the same member); the `RK_BASE` marker file `rk-git` writes into the tree's gitdir.
   - **ADR-0188 decision 10** (`s8f`'s landing record): `commit-tree`; `update-ref refs/heads/rk-run/<item>`; and
     `read-tree HEAD` on the run-owned worktree's own index.
   - **This record:** `tag` (decision 1) and `apply` (decision 2).

   The reads each of those records names (`worktree list`, `status`, `ls-tree`, `check-ignore`, `merge-base`,
   `log --decorate-refs=refs/tags/rk-release/`, and the rest) are not supersessions (D21-47).

## Consequences

**Positive**
- `tag-release` and the mutation harness have their writes, each visible as a variant and journalled.
- ADR-0105's clause 5 is read from one enumeration.

**Negative**
- Two more variants on a closed enum, which moves Checkpoint 6's re-run (it is provisional until Checkpoint 8g (12)).
- A release tag created in the operator's repository is an operator moment; the real trial is `s8j`'s.

**What this forbids**
- A push of any ref, and a `GitOp` push variant.
- A tag on a commit that is not on `rk-run/<item>`, a tag outside `refs/tags/rk-release/`, or a lightweight tag.
- `git apply` in the run-owned worktree, the main worktree, or anywhere but the `<item>.mut` tree; `apply --index` or
  `--cached` on any index but that tree's.
- A git write not on the consolidated list above, by any crate, without a further supersession in part.

## Evidence and confidence

**Measured** (2026-09-29, at `178868f`): `GitOp` in `crates/rk-git/src/git_exec.rs` has no `Tag` and no `Apply`
variant, and no push variant; ADR-0105's `superseded_by` reads `["0116", "0180", "0183", "0184", "0188"]`.
**Documented:** clause 5's text, ADR-0180 decision 15, ADR-0188 decision 10. **Confidence:** high.
