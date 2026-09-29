---
id: "0185"
title: "**`worktree_assigned`, `stage_empty` and `git_op`: three tier-2 kinds, minted by ADR-0116's route** (`s8e`'s record; D21-42 7 and 8d; D21-43 N7): a `run` writes `worktree_assigned { item, worktree, branch, reused, discarded }` at every start and every tier-1 write resolves its `RunOwnership` from the latest one for its item and from nothing else; an empty stage records `stage_empty { stage, reason }` from a closed reason set; the kernel's tier-1 boundary journals one `git_op` row per git invocation, anchored to the act's commit, and a journal holding none is read as a run that made no git write. RT-47's pair on `reconcile_started` is KEPT as tier-2 context that no reader resolves ownership from. Supersedes nothing"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (7 — *"An empty stage records `stage_empty { reason }`."*; 8d — *"A `worktree_assigned` row is written at run start. Every tier-1 write is checked against it and refused with a typed rk-git error on mismatch."*) and D21-43 (N7 — *"A tier-2 `git_op` row is minted so replay serves git writes from the journal. Golden logs are read, by stated rule, as 'no git writes'."*); runbook/phase-1/s8e-tier-one-and-the-lane.md (the records table's sixth row, §8e.3, §8e.4, §8e.6, §8e.12); references/adr/0116 (the minting route), 0105 (clauses 1 and 8), 0180 (decision 9, W2's reuse and discard), 0181 (the tier-1 writer); references/2026-09-08-architecture-design.md §5.4's event table (the `reconcile_started` row's RT-47 members); crates/rk-types/src/event_kind.rs, crates/rk-git/src/gitops.rs, crates/rk-kernel/src/runtime/tier_one.rs, crates/rk-cli/src/verbs/run.rs and answer.rs, as built 2026-09-28 on `d80609b`
supersedes: []
superseded_by: []
related: ["0105", "0109", "0116", "0180", "0181", "0182"]
evidence: []
consumers: []
---

## Context and problem

**Three things the lane needs have no kind to carry them.** ADR-0116 closed `EventKind` with the rule *"a kind in the
design table and not in the enum (or the reverse) is schema drift"*, and its route for a new one is *"The table row and
the `rk-types` variant land in the same change."* At `d80609b` the enum is 38 kinds and none carries:

- **the run-owned worktree.** `answer` read its ownership off the last `reconcile_started` row carrying RT-47's pair,
  filtering by kind alone, so with two items in one store it took the last pair of EITHER item (R-63), and `run`'s
  `story_trailer` and `terminal_read` rebuilt the pair by convention. A derivation is still a convention (`s8e` §8e.4).
  `s8d-ii`'s lifecycle function returns whether the tree was reused and every in-flight path the re-entry discarded,
  and until a row exists `run` prints that list on stderr: W2's *"discarded explicitly and journalled"* is half met.
- **an empty stage.** `triage` carries no shipped activity, and a lane advances out of it with no record of why
  nothing ran (`s8e` §8e.6).
- **the boundary's git invocations.** ADR-0105 clause 1 says `GitOps` *"records every git invocation and its result to
  the EventStore"*; `rk-git`'s `GitJournal` exists, and nothing appends what it records (`s8e` §8e.3).

## Decision drivers

- D21-42 8d, D21-42 7 and D21-43 N7, in their words above.
- ADR-0116's route: a design §5 row and the `rk-types` variant in one change, with `tier()`, `ALL`, the completeness
  test and the no-catch-all arms in `fsm.rs`, `breaker.rs` and `memo.rs`.
- ADR-0109 decision 2's deleted-database test fixes each kind's tier.
- AC-162: a failed tier-1 git write leaves zero store rows.
- ADR-0105 clause 8 and AC-529: ownership comes from the store, never from `cwd` or a convention.
- One source per fact: RT-47's pair and the new row must not both be read.

## Considered options

### Option — three tier-2 kinds, each item-keyed, with `git_op` rows anchored to the act (taken)

Each kind carries exactly its section's members; each is tier 2 by the deleted-database test; each carries ADR-0182's
interim `item` key; `git_op` rows are rows of the boundary act and share its anchor. Chosen: it is the smallest
carrier for each fact and moves no canonical event.

### Option — carry the ownership, the empty-stage reason and the invocations on existing kinds

`reconcile_started` already carries RT-47's pair, and a stage's `stage_transition` could carry a `reason`. Rejected:
`reconcile_started` is written before the run's tree is entered and before the discard list exists, so it cannot say
`reused` or `discarded`; and a `reason` member on `stage_transition` would change that tier-1 kind's payload and every
canonical event carrying it. The invocations have no existing kind at all.

### Option — `worktree_assigned` at tier 1, a git mark per assignment

Rejected by the deleted-database test: `run` re-enters the registered, marked tree (ADR-0180 decision 9) and writes the
row again without re-creating anything, so losing the row changes no next action of `run`. A trailer commit at every
run start would be noise on the run ref and a git write ADR-0105 clause 5's list does not name.

### Option — journal every git invocation of a failed act, before the failure returns

Rejected: a failed commit would leave a `git_op` row, and AC-162's instrument asserts *"zero store rows afterwards"*.
The failure travels on the error instead.

### Option — withdraw RT-47's pair from `reconcile_started`

Considered and not taken: withdrawing it would move `reconcile_started`'s payload, which is AC-485's routed run-start
carrier and which tests pin, for no reader's benefit once no reader resolves ownership from it. Keeping the pair as
tier-2 context with no reader is the more reversible choice; the withdrawal stays available by a dated design note.

## Decision

**1. Three kinds are minted by ADR-0116's route** — `EventKind::WorktreeAssigned` (`worktree_assigned`),
`EventKind::StageEmpty` (`stage_empty`) and `EventKind::GitOp` (`git_op`) — with three design §5.4 event-table rows in
the same change, `ALL` at 41, and the three no-catch-all arms (`fsm::apply`, the breaker fold, the memo fold) routing
all three to their no-state-change arms. `fsm.rs`'s comment over `apply`, which stated the enum's size, names no count.

**2. `worktree_assigned { item, worktree, branch, reused, discarded }`, tier 2.** `run` writes it at **every** start,
after the invocation's `reconcile_started` row and after `rk_git::worktree::enter` has created, re-entered or refused
the tree: `reused` is `true` for a re-entered tree, and `discarded` lists every in-flight path the re-entry threw away
(W2's list, now journalled; the stderr lines stay as the operator's echo). **Every tier-1 act resolves its
`RunOwnership` from the latest `worktree_assigned` row for its item**, read through `ByItem` by one resolver,
`rk_cli::verbs::run::assigned_ownership` over `rk_kernel::runtime::tier_one::assigned_ownership`, and from nothing
else. A mismatch is the existing typed `GitError::ForeignWorktree { target_path, target_branch, owned_path,
owned_branch }`, raised by `authorise_tier1_write` inside the kernel's boundary before any commit or row (AC-529).

**3. The deleted-database reading of `worktree_assigned`, in terms: a tier-2 disposal, not a next-action change.** If
the database is deleted, `answer` finds no row and **refuses, fail-closed, by name**, before its heal and before any
gate read; it writes no row but its invocation's `reconcile_started` (AC-172's baseline), makes no commit, and
resolves nothing from the `.redkiln/worktrees/<item>` convention. The next `run` re-enters the registered, marked tree
under W2 and writes the row again without re-creating anything, so `run`'s next action is the same with or without
the row.
*(dated 2026-09-28, `s8e` build, package S8E-RULINGS.)* **Confirmed as written by the operator**
(`spikes/DECISIONS-2026-09-21.md` D21-94, confirmation 3: *"with the database deleted, `answer` refuses fail-closed
until the next `run` writes `worktree_assigned` again; ownership has one source"*).

**4. `stage_empty { stage, reason }`, tier 2, with a closed reason set.** The reason is one of
`rk_types::event_kind::STAGE_EMPTY_REASONS`, which holds one member at this record: **`no_applicable_activity`** — the
stage's assembled plan holds zero nodes, so nothing is dispatchable there at the effective rigor. A new reason is a
record, never a producer's choice. The row is a row of the stage entry's act: the `Story:` mark carries the entry
either way, and the row is anchored to that mark's commit (`s8e` §8e.3's *"a `stage_transition` and its
`stage_empty` in one act"*). Its producer is `s8e` §8e.6's; this record fixes the kind, its members, its tier and its
reason set.

**5. Each of the three carries ADR-0182's interim `item` key** (`ItemKey::Carried`), beside ADR-0182's thirty;
`worktree_assigned` also names its `item` in the payload, which is the value the resolver compares.

**6. `git_op { op, cwd, argv_digest, status, stdout, stderr_digest }`, tier 2: `GitInvocation`'s own members.** The
kernel's tier-1 boundary (`rk_kernel::runtime::tier_one::commit_act`, ADR-0181) runs over a `GitOps` seam whose
`GitJournal` holds each invocation; once the act's commit exists and its tree is re-read, every row of the act is
appended through `append_tier1` and then **one `git_op` row per invocation the seam made**, each anchored to the same
commit. **A failed act writes no `git_op` row**: its invocations are dropped with the error, because AC-162's zero
binds harder than journalling a failed invocation, and the error names the failed operation. `stdout` is carried as
UTF-8 (lossily); ADR-0105 clause 7's retention cap is not applied to the boundary's two short outputs at this record.

**7. Replay, and the stated rule for golden logs.** On replay the boundary takes no authorisation read, and its commit
and re-reads are SERVED from the act's recorded `git_op` rows, in order; nothing is written to git or to the store,
which `rk-git`'s own `writes_performed()` counter reads at zero. **A recorded journal that holds no `git_op` row is a
run that performed no git write**: the boundary's replay arm then serves no commit (`Committed::NoGitWrite`), so the
committed golden logs are read as they are and never re-recorded.

**8. RT-47's pair on `reconcile_started` is KEPT, as tier-2 context that no reader resolves ownership from.** `answer`'s
`recorded_ownership`, the one reader, is deleted. The design §5 row carries a dated note saying so. There is one
source of ownership, and it is decision 2's.

**9. The residuals, stated rather than left.**
- **The unbound machinery seam.** `Runtime::new` binds no boundary; an unbound runtime keeps the store-only tier-1
  arms for the kernel's machinery suites, `replay::resume` (which passes none) and `rk-cli`'s in-process `run_with`,
  which have no repository behind them. The golden-log recorder `replay::record` is the same seam. The production
  path — `replay::resume_routed` called by `rk-cli`'s `run` — binds one on every stage, and `tier_one::`'s lints name
  the one writer and the one commit site.
- **The heal's scope.** `tier_one::heal` re-derives the rows of a stage-bearing `Story:` mark for the item and of every
  `Gate:` mark reachable from `rk-run/<item>`. It skips ~~a `closed` mark and~~ a `Slice-Verdict:` mark, naming
  ~~them~~ it, until ~~`s8e` §8e.6 and~~ §8e.8 ~~fix those rows~~ fixes its row; and a healed `opened` row carries no run
  (D21-48 X11). *(dated 2026-09-28, `s8e` build, package S8E-P6: §8e.6 fixed the `closed` act's row, ADR-0183
  decision 10, and the heal re-derives it.)*
  *(dated 2026-09-28, `s8e` build, package R1FIX-8e, recording package S8E-P7's change; refuting round 1's
  spec-conformance minor.)* ~~It skips … a `Slice-Verdict:` mark, naming it, until §8e.8 fixes its row~~: since
  S8E-P7 the heal skips no `Slice-Verdict:` mark. It re-derives the mark's `slice_sealed` row from the mark's own
  tokens, `{project, slice, verdict}`, with **no `stage`**, because the trailer carries none; the fold scopes a
  stage-less seal to the stage it folds in (ADR-0186's stated residual; D21-93). The heal still skips, by name,
  only a legacy stage-less `Story:` mark, which records no stage.
- *(dated 2026-09-28, `s8e` build, package S8E-P6.)* **`stage_empty` on a stage entered by an ADVANCE is not a
  row of the entry's act.** The advance's act (the previous stage's `AdvanceStage`) is taken before the next
  stage's plan is selected, so the kernel cannot know then that the plan is empty. The row is appended beside it
  (tier 2, unanchored), once per `(item, stage)`, when `replay::resume_routed` selects the empty plan. A lane's
  FIRST stage is entered by `resume_routed` itself, and there the row IS a second row of the entry act, anchored
  to its commit (decision 4). `triage`, the empty stage `s8e` §8e.6 names, is first in both shipped packs'
  `stage_order`; whether any other shipped stage selects an empty plan at some rigor was not measured here.

## Consequences

**Positive**
- `answer` and every tier-1 act read ownership from one row, keyed to the item, so two items in one store cannot lend
  each other a worktree (R-63), and AC-529's resolution instrument can differ the store's path from the convention.
- W2's discard is journalled, and a re-entry is distinguishable from a creation in the store.
- The boundary's git writes are served on replay from the journal, with zero writes.

**Negative**
- Every run start writes one more row; a store deleted between `run` and `answer` refuses the answer until the next
  `run`.
- `git_op.stdout` is not capped at this record, and the kinds' payloads are not typed (the per-kind payload record
  stays OWED, as ADR-0126 decision 6 left it).
- The unbound seam remains, for the suites and the recorder; a caller that constructs an unbound runtime on a live
  path would write tier-1 rows store-first. The lints bound the writer and the commit, not the constructor.

**What this forbids**
- Resolving a tier-1 write's `RunOwnership` from `cwd`, from the `.redkiln/worktrees/<item>` convention, or from
  RT-47's pair on `reconcile_started`.
- `answer` proceeding without a `worktree_assigned` row for its item.
- A `stage_empty` reason outside `STAGE_EMPTY_REASONS`, or a reason added without a record.
- A `git_op` row written for an act whose commit failed, or one anchored to a commit other than its act's.
- Re-recording a committed golden log so that it carries `git_op` rows.

## Evidence and confidence

**Measured** (2026-09-28, the build of `s8e` package S8E-P5 on `d80609b`): each decision's test, red first and then
green — `worktree_assigned::` (five, over the real `run` and `answer` binaries and the kernel boundary),
`tier_one::the_boundary_journals_a_git_op_row_per_invocation`,
`tier_one::replay_serves_the_boundary_from_its_git_op_rows`,
`tier_one::a_journal_without_git_op_rows_replays_as_no_git_writes` (over the committed golden log, untouched) and
`tier_one::a_boundary_anchors_every_row_of_the_act_to_its_commit`; and the mutants S8E-M9, M29, M41 and M42, each
killed for its stated reason. ~~**Not measured, and named:** the `stage_empty` producer (§8e.6's).~~ This record is
`proposed` and flips only on the operator's dated ledger line.

*(dated 2026-09-28, `s8e` build, package R4FIX-8e, recording package R3FIX-8e's measurement; refuting round 4's
minor.)* **The `stage_empty` producer is now measured, with decision 4's anchor and reason set.**
`lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` asserts, over `verbs::run::run`, that the lane's
empty first stage writes one `stage_empty` row anchored to the entry act's `Story:` commit sha, whose reason is a
member of `STAGE_EMPTY_REASONS` (`s8e` §8e.6, §8e.12). The mutants that redden it for their stated reasons:
S8E-M14 (*"`stage_empty` not appended"*), S8E-M60 (the entry act takes the entry row alone, so the row is
appended beside it, unanchored) and S8E-M61 (a reason outside the closed set). M60 and M61 were measured
surviving at `259f714` and were killed after R3FIX-8e's re-point (`s8e` §8e.12's rows). Still unmeasured, as
decision 9 names: whether any shipped stage other than `triage` selects an empty plan at some rigor.
