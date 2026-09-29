---
id: "0188"
title: "**The landed commit is a kernel-made content checkpoint on `rk-run/<item>`, made at every committed step that changed the tree and at every tier-1 boundary that finds work unlanded, through `s8e`'s one boundary function** (`s8f`'s landing record; D21-41 b; D21-42 9; D21-43 N10): the staging set is tracked modifications and deletions plus the untracked files not in the journalled baseline, honouring ignores, computed by one function that re-entry also discards; every staged path is checked against a named denylist, a 2 MiB blob limit and `0180`'s worst-case path bound, and a refusal lands nothing, opens a `landing.refused-path` gate (`exclude` | `abort`) whose tree is `0105` clause 4's no-checkpoint-due derivation, and is never a silent drop; each commit is `read-tree HEAD` + `add` on a per-item temporary index, `write-tree`, `commit-tree`, a re-read, `update-ref`, and `read-tree HEAD` on the worktree's own index; a step checkpoint carries no trailer, names its step-grain key on its subject line and carries its row in its body, so `reconcile`'s heal re-appends a lost row; the gate binds the run-owned tree (defect b); nothing is integrated, and `explain` prints `git merge --no-ff rk-run/<item>`. Supersedes `0105` clause 5 in part, for three writes"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (row b — *"A kernel-made content checkpoint commit on `rk-run/<item>` that carries the tier-1 trailer. No integration in Phase 1."*), D21-42 (row 8a — *"The kernel owns it, through the journalled `GitOps` seam. It commits (zero writes on replay), then appends the t1 and t2 rows with `anchor_sha`. `reconcile` heals a commit without a row."*; row 9 — *"The worktree diff plus named refusals."*), D21-43 (N10 — *"Land at every committed step that changed the worktree. The commit and the content checkpoint are one boundary act."*; N9), D21-45 (W2), D21-47 (*"ADR-0105 clause 5 has one reading. A git WRITE not on its closed list is a supersession in part. That covers s8f's `commit-tree`/`update-ref`"*), D21-48 (awareness line on the refused landing's loose object) and D21-97 (the owed-forward list: defect (b), finding 11, AC-518's two halves); runbook/phase-1/s8f-landing-and-rendering.md §8f.1, §8f.2 (items 1, 3, 6, 7 and 15, measured 2026-09-28 at `37bba5c`), §8f.3's first row and §8f.4; references/adr/0105 (clauses 1–5 and its forbids list), 0109 (decision 2), 0180 (decisions 15 and 16), 0181, 0183, 0184 and 0185; crates/rk-kernel/src/runtime/tier_one.rs (`Act`, `Content`, `commit_act`, `heal`), crates/rk-kernel/src/runtime/activity.rs (`commit_at`, `commit_walk_row_emitting`), crates/rk-git/src/git_exec.rs (`GitOp`), crates/rk-git/src/tree.rs and crates/rk-cli/src/verbs/run.rs (`prepare_inputs`), as read 2026-09-28 at `37bba5c`
supersedes: ["0105"]
superseded_by: []
related: ["0009", "0017", "0018", "0109", "0129", "0152", "0164", "0174", "0180", "0181", "0183", "0184", "0185"]
evidence: []
consumers: []
---

## Context and problem

**Nothing on the tree can produce a landed commit.** AC-473 asks for *"every lane stage from intake to a landed
commit"*. At Checkpoint 8e every tier-1 commit is made by one function, `rk_kernel::runtime::tier_one::commit_act`,
through the journalled `GitOps` seam, and every one of them is `--allow-empty`: the function's `Content` enum has one
arm, `Content::Empty`, whose doc reads *"`s8f`'s landed arm changes what the commit holds and not the order"*. A run
therefore makes **zero content commits**. The harness's edits in the run-owned worktree are either discarded at the
next entry (`s8d-ii`'s W2 discard) or left for a human to find.

**The operator took the decision in three steps.** D21-41 (b): *"A kernel-made content checkpoint commit on
`rk-run/<item>` that carries the tier-1 trailer. No integration in Phase 1."* D21-42 9: *"The worktree diff plus named
refusals. Stage tracked changes and files the run created; the untracked baseline is snapshotted at worktree
creation, and `.gitignore` is honoured. A secret-pattern or oversize path is refused by name with a gate-worthy exit.
Committed through the `GitOps` seam"*. D21-43 N10 moved the grain: *"Land at every committed step that changed the
worktree. The commit and the content checkpoint are one boundary act."*

**What `s8e` left to this record, by name** (D21-97, the owed-forward list):
- **Defect (b).** *"`run`'s gate binds the user checkout's tree (`prepare_inputs` hashes `resolved.worktree`) while
  `answer` hashes the run-owned tree, so once the run tree moves every later answer is void too"*. Measured at
  `37bba5c`: `prepare_inputs` calls `rk_git::tree::tree_hash(&resolved.exec, &resolved.worktree, …,
  &TreeBasis::WorkingState)`.
- **Finding 11.** *"A content commit whose step row never reached the store is not covered by the tier-1 heal"*;
  *"`s8f`'s landing record settles how that step is read"*. `tier_one::heal` re-derives rows from trailer marks only,
  and a step checkpoint carries no trailer.
- **AC-518's file-write and index-change halves** (*"`s8f` claims them"*).

**ADR-0105 clause 5 is a closed list.** It reads: *"**5. The wrapper's write set is enumerated**: `worktree
add`/`remove`; `add`; `commit` (incl. `--allow-empty`, `--trailer`); `interpret-trailers`; `write-tree` on a temporary
index; `patch-id`; fast-forward/merge on the integration ref; `clone` removal; `rev-parse --git-common-dir` on
S-GIX/a's no-go (ADR-0098). Anything not listed is a successor."* Clause 3 names the plumbing, *"plumbing is
`commit-tree` + `update-ref` where the wrapper needs an exact parent"*, and D21-47 reads clause 5 one way: *"A git
WRITE not on its closed list is a supersession in part. That covers s8f's `commit-tree`/`update-ref`"*.

**Measured at stage open** (`s8f` §8f.2, 2026-09-28, at `37bba5c`, read-only):
- Item 1: every tier-1 commit is made in `rk-kernel`'s `runtime/tier_one.rs` through `GitOps` (`commit_act` calls
  `git.commit_with_trailers(&cwd, &message, …, true)`), and each row is appended through `append_tier1` with a
  `TierOneAnchor` carrying the commit's sha and tree. Outside `rk-git` and `tier_one.rs`, the only code-line caller of
  `commit_with_trailers` is `redkiln-migrate`'s `run_contract.rs`.
- Item 3: `Act` is `{ mark: Option<Mark>, content: Content, rows: Vec<Event>, opening_run }`, and `commit_act`'s order
  is authorise, commit, re-read `HEAD^{tree}`, then every row and then the `git_op` rows, all anchored to the one
  commit. A trailer-less act is refused (`TierOneError::EmptyAct`) because `Content` has only its empty arm. The step
  commit row is appended by `Runtime::commit_at` and `commit_walk_row_emitting` in `runtime/activity.rs`, each under
  ONE store-lock acquisition that re-reads the journal for a prior commit and then appends (AC-012's critical
  section); neither makes a git write.
- Item 6: the legacy tree `.git/redkiln/worktrees/rk-fix-doctor-test-count` is at `73c087a` with an empty
  `status --porcelain=v1 --untracked-files=all`.
- Item 7: **0** of the **2,043** tracked paths match the denylist below (every glob, matched on the final path
  component, ASCII case-insensitively); the largest tracked blob is **1,439,568 bytes**
  (`references/audit/2026-09-10/GROUNDING-MERGED.json`); the longest tracked path is **140** characters.
- Item 15: ADR-0180 decision 16 reads *"The bound is `len(root) + 20 + len(item) + 11 + 1 +` the longest path tracked
  at the fork commit … Creation refuses by name when the bound exceeds **259** (Windows' 260-character `MAX_PATH`
  leaves 259 usable) and `core.longpaths` is not `true`"*, computed *"on the **actual** item"*.

## Decision drivers

- N10: a committed step's tree effect is never left unlanded, so re-entry's discard can only lose the work of a step
  that has no commit to serve.
- D21-42 8a: git first, through the one journalled seam, zero writes on replay, `anchor_sha` on every row.
- ADR-0105 clause 4: one `tree_hash` derivation; a gate's tree must be its own commit's tree when a checkpoint is due.
- Halt 7: a refusal is by name and never a silent drop, and no glob or limit is widened to make a run pass.
- AC-518: the checkout the operator runs from is never written; only `rk-run/<item>` moves.
- Tier-1 state is keyed by commit SHA (ADR-0018; design §5.5): the hand-off must keep every trailer the run wrote.

## Considered options

### Option — land through `s8e`'s boundary function, on a per-item temporary index, with `commit-tree` and `update-ref` (taken)

The landing is a second arm of `tier_one::Content` and a step-grain act in the same function. It stages onto a
temporary index, derives T with `write-tree`, commits T with `commit-tree -p <tip>`, re-reads the new object's tree,
and only then moves the ref. The tree is known before the commit exists, which a `Gate:` value needs, and it is the
commit's own tree by construction. It costs three writes that are not on clause 5's list, which this record takes as
a supersession in part.

### Option — stage the worktree's real index and run porcelain `git commit`

Rejected. It needs `write-tree` on a real index, which clause 5 does not list either, and it derives T from equal
inputs rather than from the object committed. A `Gate:` trailer must carry T before the commit exists; porcelain
`commit` gives no T to put in it except by a second derivation, which clause 4 forbids.

### Option — one content checkpoint per tier-1 boundary, "when due" (D21-41 b's original grain)

Rejected by N10. A kill between a committed step and the next boundary leaves the step committed in the journal and
its edits unlanded, so re-entry must either discard work a committed step owns or re-dispatch a committed step
over its own edit (the draft's void-and-re-run route, Halt 4). N10 closes that window at the step.

### Option — salvage a refused landing against its recorded tree

Weighed and rejected. It would keep the refused step's non-refused edits by landing a partial tree the step never
produced, so the landed tree would no longer be any step's output. The cost of the taken route is stated below
instead: the refused step re-dispatches at the next entry, with the exclusion in force.

## Decision

**1. What "landed" means in Phase 1.** A landed commit is a kernel-made content checkpoint commit on `rk-run/<item>`.
After the lane's terminal `closed` mark (N9; ADR-0183), the diff `<base>..rk-run/<item>` is not empty whenever any
step changed the tree. **Nothing is integrated**: design §7.7's *"The queue, not the agent, is the authority on what
lands"* stands, and design §10.2's Phase-2 row schedules the merge queue. `GitOp::Integrate` keeps no caller. The
operator makes the hand-off (decision 12).

**2. The two kinds of point (N10).**
- **At every committed step that changed the worktree.** An agent step's commit (`Runtime::commit_at` with a step),
  a `side_effecting` tool step's commit, and an activation's completion on its `emit` binding
  (`commit_walk_row_emitting`) are each ONE step-grain act of `commit_act`. The act computes the staging set
  (decision 4) and T. When T differs from `HEAD^{tree}`, it makes the content checkpoint commit first (git first,
  D21-42 8a), re-reads it, and only then appends the step's commit row with `anchor_sha` = that commit. When T equals
  `HEAD^{tree}`, the step changed nothing: **no commit is made**, and the row is appended unanchored, as a tier-2 row
  is today. A `check` step and an idempotent `tool` step do not commit (`s8d` §8d.2), so they make no act.
- **At every tier-1 boundary** (a stage's `Story:` mark, a `Gate:` transition, the lane's `closed` mark), the act
  carries the landed arm: the trailer rides on a content commit when the staging set's T differs from `HEAD^{tree}`,
  and on an empty commit otherwise (ADR-0109 decision 2's *"a trailer on a checkpoint commit or an empty commit
  carrying the trailer"*; AC-163's empty commit). Under N10 a boundary usually finds the tree already landed. What it
  still carries is the work of steps that do not commit and the work done at entry (the render restore and the
  vouched root's store, the render record's).

**The act encloses AC-012's critical section.** The step-grain arm takes the store's guard, re-reads the journal for
a prior commit under the step's commit key, and, when one exists, returns the late-duplicate discard **before any
git write**. Otherwise it holds the guard through the landing's git writes and the append, so the re-read, the commit
and the row are one critical section. Holding the writer mutex across a git subprocess is the cost; no SQLite
transaction spans it (ADR-0105 clause 7's `busy_timeout` bound is on transactions, and the append's transaction
stays one statement pair).

**3. A step checkpoint carries no tier-1 trailer, and names its key where the heal can read it (finding 11).** Design
§5.4's tier-1 grammar has no step row, and the fold never reads a step checkpoint as tier-1 state. Its message is:
- **subject line:** `rk step <item> <activity_id@version> <step_id> <iteration-path>`, where `<iteration-path>` is the
  commit key's iteration path as compact canonical JSON (`rk_store::canon::canonical`, which carries no whitespace),
  and `<step_id>` is `emit` for an activation's completion;
- **body:** one paragraph, the line `rk-step-row ` followed by the canonical JSON of the exact row the act appends
  (its `kind` and `payload`).

Neither line is in trailer form (`Key: value` at the message's end), so `rk_git::trailer::grammar::parse_body` reads
**zero** trailers from a step checkpoint, and not even an unknown-key count moves. **The heal reads the step's value
from the body.** `tier_one::heal` gains one arm: for every commit reachable from `rk-run/<item>` whose subject begins
`rk step ` and whose sha anchors no row (`Query::BySha`), it re-appends the body's row anchored to that sha. The key
is read from the subject and must equal the key inside the row; a body that is absent, not canonical JSON, or whose
key disagrees with the subject is **refused by name**, naming the commit, and the step is never re-dispatched over its
own landed edit. A crash between the commit and the row is D21-42 8a's own window, and this is how it closes for a
step. The heal writes nothing to git (AC-174).

**4. What is staged, and the baseline.** The staging set is (a) tracked modifications and deletions, and (b) the
untracked files that are not in the **baseline**. Nothing `.gitignore` (or `info/exclude`, or a global exclude)
excludes is ever a candidate: the set is read with `status --porcelain=v1 -z --untracked-files=all`, which honours the
standard excludes. **The baseline** is that same read, taken through the seam in the first act after the tree is
**created** (the act that follows a `worktree_assigned` row whose `reused` is `false`), so it is journalled as that
act's `git_op` row (N7; ADR-0185 decision 6) and served on replay. A later process reads it from the item's journal.
**A reused tree whose baseline row is not in the journal** (a deleted database) is fail-closed: a landing that finds
any untracked, non-ignored path refuses by name with the rule `baseline-unknown`, because nothing can say whether the
run created it. Staged text is normalised by `.gitattributes`' first row, `* text=auto eol=lf` (ADR-0164), so a
landing's tree is the same on every box. **One function computes the set**, ~~`rk_git::landing::staging_set`~~
`rk_git::landing::classify`, pure over the parsed `status` entries, the baseline and the exclusions; the landing
stages from it (through `staging_set`, which reads `status` through the seam and calls it), and `s8d-ii`'s re-entry
discard names and discards exactly it (decision 13), calling it over its own `status` read. *(Corrected 2026-09-29,
s8f build, package R1FIX-8f-A-CODE, while `proposed`: the round-1 review measured that the discard classified the
entries by a second hand-kept copy, sharing only the parser; the classification is now one function, and the git
read is not shared because the landing reads through `GitOps` and the discard through `GitExec`.)*

**5. The refusals, checked before anything is committed.** Every path in the staging set is checked, run-created or
tracked (a deletion is exempt from the size check only):
- **The denylist.** A path is refused when its final component matches, ASCII case-insensitively, one of: `.env`,
  `.env.*`, `*.pem`, `*.key`, `id_*`, `credentials*`, `*.p12`, `*.pfx`, `.netrc`, `.pgpass`, `.git-credentials`,
  `.npmrc`, `.pypirc`, `*.ppk`, `*.jks`, `*.keystore`, `*.kdbx`. The list is the named constant
  `rk_git::landing::DENYLIST`. `id_*` also matches files such as `id_map.rs`; that is accepted, and the `exclude`
  answer is the remedy, never a narrower glob taken to make a run pass (Halt 7).
- **The size limit.** A staged blob larger than `rk_git::landing::MAX_LANDED_BLOB_BYTES` = **2,097,152** bytes (2 MiB)
  is refused; a blob of exactly that size lands. **Warrant:** the largest tracked blob is 1,439,568 bytes (§8f.2 item
  7), so the limit admits every blob this repository tracks with a 1.46× margin, and a landing above it is a binary
  or generated artifact that a code-change lane should not produce. The size is the staged blob's (`cat-file -s`
  after `add` on the temporary index), so it is measured after `eol=lf` normalisation.
- **The path-length bound.** A staged path is refused when `len(root) + 20 + len(item) + 11 + 1 + len(path)` exceeds
  **259** and `core.longpaths` is not `true`, where `root` is the main worktree's directory and `item` is the
  landing's own item: ADR-0180 decision 16's formula and constant, read from that record, with the landed path in
  place of the longest path at the fork. `0180` refuses at creation over the paths the fork tracks; this rule covers
  the paths a landing adds after it, so a landed path cannot make the next creation or the review tree unreachable
  on a box without `core.longpaths`.

The denylist and the length bound are checked on path names before any `add`; the size is checked after `add` on the
temporary index and before `write-tree`.

**6. The form of the refusal.** A landing that refuses any path **lands nothing**, and the step whose act it was
**does not commit** (its row is not appended), so no committed step is left with an unlanded effect. The kernel opens
a human gate of class **`landing.refused-path`**, git-first (N6; ADR-0184). Its question (ADR-0129 decisions 3–4)
names every refused path with its rule, spelled `denylist:<glob>`, `size:<bytes>`, `path-length:<bound>` or
`baseline-unknown`, and offers two options:
- `exclude`: every later landing of this lane leaves exactly those paths unstaged, and names them each time on stderr;
- `abort`: the lane stops.

`run` exits `0` at the gate (golden constraint 2; `exit.rs`'s `Success` row, *"**and a run that opens a gate**"*). A
new exit-table row was considered and rejected: AC-438's table is closed, and a gate uses the one answer channel
(ADR-0009). The class needs a `gate_expiry` row in `s8h`'s table; until `s8h` lands it reaches no expiry.

**7. What "due" means, and the gate's tree at a refused landing** (ADR-0105 clause 4's *"When a checkpoint commit is
due"*). A checkpoint commit is **due** when a landing **will commit**: its staging set passes every refusal and its T
differs from `HEAD^{tree}`. A refused landing lands nothing, so **no checkpoint commit is due** at its gate. Its
`Gate:` commit is empty, and the gate's `tree_hash` is clause 4's no-checkpoint-due derivation over the run-owned
worktree: `write-tree` on a temporary index built from the current index plus tracked modifications, untracked files
excluded. It is **not** the empty commit's tree. The refused step's edits stay in the worktree until the next entry,
so `answer`'s identical derivation gives the same tree and cannot void the gate (AC-176). **The cost, stated:** the
derivation writes a refused tracked blob into the object store as a loose object that no ref reaches (D21-48's
awareness line); an untracked refused file never enters it.

**8. This record's reading of ADR-0105's bullet *"forbids including untracked files in it"*.** The bullet's *"it"* is
the `tree_hash` derivation, and the rule it protects is clause 4's no-checkpoint-due basis. At a due checkpoint, and
at every step checkpoint, the run-created files are **staged** (by an explicit `add -- <path>…` of named paths on the
temporary index) **before** the derivation, so the derivation reads a tree in which they are tracked, and still sees
no untracked file. The bullet stands whole; AC-245's note says the same (§8f.10).

**9. How each commit is made — one derivation.** Every landing, at a step or a boundary:
1. Stage onto a **temporary index** under the common dir, `<common-dir>/redkiln/tmp/landing-<item>.index`: derived
   per item, never random (`tree.rs`: *"The name is derived, never random"*), and one worktree per concurrent item
   (Halt 6) makes the fixed name collision-free. On it: `read-tree HEAD` (`GitOp::Add`, as `tree.rs` classes it), then
   `add -u`, then `add -- <run-created paths>` (`GitOp::Add`).
2. `write-tree` on it → **T** (`GitOp::WriteTree`; clause 5 lists *"`write-tree` on a temporary index"*).
3. Compose the message: the trailer at a boundary, or the step lines at a step, through the existing
   `interpret_trailers_in_place` (`--if-exists=add`).
4. `commit-tree T -p <tip>` (**`GitOp::CommitTree`**), under the wrapper's `-c core.hooksPath=<empty>` (clause 2).
5. Re-read the new object's tree (`TreeBasis::CheckpointCommit`); it must equal T, or no ref moves (`s6` §6.2
   constraint 4: *"anything written by the executable and then read is RE-READ, never assumed"*).
6. `update-ref refs/heads/rk-run/<item> <new> <tip>` (**`GitOp::UpdateRef`**), the old value guarding a race.
7. `read-tree HEAD` on the run-owned worktree's **own** index (**`GitOp::ResyncIndex`**), so its index agrees with the
   new tip and the next staging set does not re-read landed paths as modified.

Every invocation, reads included, goes through `s8e`'s seam, so each is journalled as a `git_op` row anchored with
the act's rows; on replay T and the commit are served from those rows and `rk-git` performs **zero** writes (clause
1). `status`, `rev-parse`, `ls-files`, `cat-file -s` and `log` are reads and supersede nothing (D21-47: *"A READ is
not a supersession"*); `cat-file -s` takes a read variant, `GitOp::CatFile`.

**10. ADR-0105 clause 5 is superseded in part, for three writes on the run-owned tree only.** Clause 5 reads
*"Anything not listed is a successor."*, and its forbids bullet reads *"Forbids a wrapper write not on clause 5's
list"*. This record adds to the list, and exempts from the bullet: `commit-tree` (`GitOp::CommitTree`); `update-ref
refs/heads/rk-run/<item>` (`GitOp::UpdateRef`, on that ref and no other); and `read-tree HEAD` on the run-owned
worktree's own index (`GitOp::ResyncIndex`). Clause 3's *"plumbing is `commit-tree` + `update-ref` where the wrapper
needs an exact parent"* is the design warrant, and it does not substitute for this supersession (D21-47). **The
consolidated clause-5 list**, each part in its own record and no operation in two: `0180` decision 15 (`worktree
lock`/`unlock`, the `RK_BASE` marker, `reset --hard HEAD`, `clean -fd`); `s8g`'s R3 (`tag`, `apply`); this record (the
three above). Clause 6's *"re-created, never reused"* is `0180`'s supersession, cited here. Every other clause of
`0105`, and every other member of its bullet (*"a `\\?\` path handed to git"*; `0180` holds *"a reused worktree"*),
stands. The three variants are named in `rk-git` because *"adding a variant is that decision made visible"*
(`git_exec.rs`).

**11. The landing's writes stay inside `s8e`'s boundary module (AC-518).** `s8e`'s lint
`tier_one::every_run_commit_resolves_through_the_worktree_resolver` is **widened, not added**: every code-line call of
`rk-git`'s landing primitive under `crates/rk-cli/src/` and `crates/rk-kernel/src/` is in `runtime/tier_one.rs`, and
the `GitOp::CommitTree`, `GitOp::UpdateRef` and `GitOp::ResyncIndex` variants are named on code lines only inside that
primitive under `crates/rk-git/src/`. The checkout is never written, and only `rk-run/<item>` moves. These are AC-518's
file-write and index-change halves, claimed through the widened lint.

**12. The gate binds the run-owned tree (defect b), and the hand-off.**
- `gate_opened`'s `tree_hash` is the landing's T over the run-owned worktree when the gate opens (the checkpoint's
  tree when one is due, clause 4's derivation otherwise). It is no longer `prepare_inputs`' stage-start
  `WorkingState` over the checkout. `prepare_inputs` hashes the **run-owned worktree**, since that is the tree the
  stage's inputs come from. `run_id` is `format!("{}-{}", stage, &digest[..16])` over the inputs, so its values move
  for new runs and its grain does not (ADR-0152); a committed run-record fixture's bytes are pinned as they are
  (ADR-0151) and do not move.
- `explain <item>` gains a landing block after the recorded resolution, read from git (`GitOp::RevParse`,
  `GitOp::Log`) and from the tier-1 rows' `anchor_sha`, never from a selection (AC-432). Its lines are exactly:
  `landing: rk-run/<item> at <sha>`, `landing: <n> commits ahead of <base>, <c> with content` and
  `landing: git merge --no-ff rk-run/<item>`, where `<base>` is the tree's `RK_BASE` fork (`0180`) and a commit has
  content when its tree differs from its first parent's. With no run ref it prints one line,
  `landing: nothing landed (no rk-run/<item>)`. **No squash and no rebase**: tier-1 state is keyed by commit SHA
  (ADR-0018; design §5.5), so a squashed hand-off would orphan every trailer the run wrote.

**13. The discarded set at re-entry is the staging set.** What `s8d-ii`'s reuse discards (W2; `0180`) is exactly what
a landing would have staged at that moment, computed by ~~`staging_set`~~ `rk_git::landing::classify` (decision 4;
corrected 2026-09-29, s8f build, package R1FIX-8f-A-CODE, while `proposed`), printed path by path and journalled in
`worktree_assigned.discarded` (ADR-0185). Under N10 every committed step's tree effect is on `rk-run/<item>` before
its row exists, so only a step with no commit can lose work, and `s8d`'s walk re-dispatches it (*"the first step with
no commit executes"*). AC-013 stands unamended. A human's edit to a run's artifact is a commit on `rk-run/<item>`; an
uncommitted edit left in the run-owned worktree is in-flight dirt, named and discarded at the next entry.

**14. `answer` does not land.** After a landing, the owned worktree's no-checkpoint-due tree equals `HEAD`'s tree, so
an answer's derivation matches its gate's. An answer's effect on an artifact is performed by the next `run` at entry
and lands there.

## Consequences

**Positive**
- A run makes content commits, and a lane's work is reachable on `rk-run/<item>` for the operator's merge.
- A kill after a committed step resumes over its landed edit; re-entry's discard can lose only uncommitted work.
- A gate binds the tree its answer will derive, so an answer is no longer voided by the run tree moving (defect b).
- A lost step row is healed from git, and a step is never re-dispatched over its own landed edit (finding 11).

**Negative**
- The writer mutex is held across the landing's git subprocesses at every committed step.
- A refused landing costs the refused step's edits, which are discarded at the next entry and re-dispatched with the
  exclusion in force, and it leaves an unreferenced loose object for a refused tracked blob.
- A deleted database makes a reused tree's baseline unknown, and its next landing with an untracked file refuses by
  name until the operator answers `exclude` or `abort`.
- `id_*` refuses legitimate `id_`-prefixed file names; the remedy is `exclude`, one path at a time.
- `run_id` values move for new runs, because `prepare_inputs` hashes a different tree.

**What this forbids**
- A landed commit made anywhere but `tier_one::commit_act`, and a landing primitive called from outside
  `runtime/tier_one.rs`.
- A committed step's row appended before its content checkpoint commit, or anchored to any other commit.
- A silent drop of a refused path, and a partial landing that lands the non-refused paths of a refused step.
- Staging an ignored file, a baseline file, or any path a refusal named.
- Widening a denylist glob, raising `MAX_LANDED_BLOB_BYTES`, or relaxing the path bound to make a run pass (Halt 7).
- Recording the empty commit's tree as a refused landing's gate tree.
- A step checkpoint that carries a tier-1 trailer, or the fold reading a step checkpoint's subject or body as tier-1
  state.
- Re-dispatching a step whose checkpoint is on the run ref, even when its row is missing.
- Integration, squash or rebase of `rk-run/<item>` by the run; `GitOp::Integrate` gaining a caller in Phase 1.
- A git write by the landing that is not `add`, `write-tree` on the temporary index, `commit-tree`, `update-ref
  refs/heads/rk-run/<item>` or `read-tree HEAD` on the run-owned worktree's own index; any git write on replay.

## Evidence and confidence

**Measured** (2026-09-28, at `37bba5c`, `s8f` §8f.2): items 1, 3, 6, 7 and 15 as quoted in the context. **Documented:**
the ledger lines and records quoted. **Not measured, and named:** the landing's wall-time cost per step, and the heal's
arm over a real crash between commit and row, which `landing::a_kill_after_a_committed_writer_step_resumes_over_its_landed_edit`
and `step_landing::a_committed_step_that_changed_the_tree_lands_before_its_commit_row` will measure over fixtures. This
record is `proposed` and flips only on the operator's dated ledger line (ADR-0126's route; D21-105 item 1).
