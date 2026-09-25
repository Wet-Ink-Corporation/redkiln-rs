# Step 8e — Tier-1 writes git-first, the lane across processes, and `Slice-Verdict:` (Phase 1, part 8e)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8e` | Phase 1 | Checkpoint 8d | Checkpoint 8e | `s8d` | `s8f` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s7h §7h.2`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** Three things the lane needs are recorded as missing, and they are one piece of work
because they meet at the same boundary act. **Measured at `ca8f387`, 2026-09-24, read-only:**

- **The tier-1 store layer has no writer.** `crates/rk-store/src/migrations/0001_initial.sql` declares
  `t1_stage_transition`, `t1_gate` and `t1_slice_seal` under the header *"These rows never exist without their trailer
  commit. A failed git write leaves ZERO of them (AC-160..AC-163), which is why every one is anchored to a commit
  sha."* The `EventStore` port is still `append` / `query` / `snapshot` / `rebuild`, and `SqliteStore::append` writes
  `"INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2, NULL)"`. ADR-0158 records this and routes it:
  *"a tier-1 write method on `EventStore` (with its fake and conformance row), called from the same place the trailer
  commit is made, taking the `anchor_sha` the commit returns"*.
- **Every tier-1 write on the tree is store-first.** `Runtime::step`'s advance arm is
  `self.append(stage_transition_event(from, to))?;`, and the `Story:` commit comes later, in `rk-cli`'s
  `story_trailer`, after `resume` has returned. `resume` appends the stage-entry row through `rt.append` with no
  trailer at all. `answer` appends `outcome.events` (the `gate_answered` row) **before** it calls
  `commit_with_trailers`. ADR-0109 decision 2 says tier 1 *"is written to git first"*, and its forbids list says
  *"Forbids a tier-1 transition recorded only in the store"*.
- **A lane does not cross processes.** ADR-0157 decision 4: *"Across processes, a second invocation reports `Done`
  and exits 0 having done nothing"*, with the route *"named here as OWED"*. The stall is in the seed:
  `through_step_seven` reads the stage from the item record (*"the stage is read once, from the record, and is not
  defaulted"*) and **discards the reconcile it performs** — the call is `let _ = startup` … `.reconcile(&repo.exec,
  &repo.worktree, entry)`. So the `Reconciled<State>` ADR-0109 decision 1 makes the planner's only input is built on
  every entry and thrown away.
- **`Slice-Verdict:` has no producer.** The ARMED lint still reads `#[ignore = "ARMED: Slice-Verdict has 0
  producers; Gate and Story are both minted as of s7h.2; discharged by s7h"]`, and ADR-0160 names the route: *"declare
  the slice on the item record (or decide that a fix lane has exactly one, and say so), then a `slice_sealed`
  producer at the review stage's boundary, then the trailer through the same `commit_with_trailers` path `Story:` now
  uses."*
- **The lane loop has no automated test.** `grep -rn "verbs::run::run(" crates/*/tests` returns nothing. ADR-0161
  names the cost: two of its three defects *"are in the seam between the kernel's report and the composition root's
  loop — which no unit fixture in either crate covers"*.

**This step takes decisions, and they were already taken.** The operator took them in D21-41 (a) and (e) and
D21-42 7, 8a–8d, 24b and 24c (`spikes/DECISIONS-2026-09-21.md`). They are written as records at this stage, and each
record takes **the next free id at writing** — this file assigns none. Where a decision moves a `## Decision`
sentence or a forbids bullet of an accepted record, the record is a supersession in part (ADR-0092 rule 1), never an
erratum. Every record is written `proposed` and flips only on the operator's dated ledger line. Where this file found
a residual the decisions do not name, it says so in §8e.12 rather than taking it silently.

**What this step owns** (the plan's row for `s8e`): D21-41 **a**; D21-41 **e**, both halves — `Slice-Verdict:` AND
the tier-1 writer; D21-42 **7**, **8a**, **8b**, **8c**, **8d**, **24b** (L3) and **24c** (`terminal_read` plus the
first FakeHarness end-to-end test of `verbs::run::run`).

**What it does not own.** ADR-0164's flip (the operator's ledger line). The landing checkpoint commit and staging
(D21-41 b, D21-42 9 — `s8f`). The verify record and `verify_completed` (`s8g`). Cards, models, the launcher and the
run environment (`s8h`). The fresh fix item and the story item (D21-42 18, D21-41 f — `s8j`). Every amendment to
`s8-exit-criterion.md` itself, including the mechanical set-and-order assertion D21-42 7 puts on criterion (1) (`s8j`,
which owns the `s8` instrument amendments). **No stage before `s8i` dispatches to a real account, and this one
dispatches nothing** — every run in it is over a fixture repository, a scripted `FakeHarness`, or the fixture child.

**Order of work, and why.** §8e.1 first: D21-42 24b says L3 is fixed *"before the writer builds on it"*. Then the
writer (§8e.2), the kernel-owned order (§8e.3) and the ownership row (§8e.4), because every mark written after that
goes through them. Then the three axes (§8e.5–§8e.8), then the rebuild (§8e.9). The end-to-end tests (§8e.10) are
written **first, red**, and go green last.

## Records this step writes

Each record below is **a new record (next free id at writing)**, in the ADR-0085 `### Option — <name>` heading form
that D21-42 24a makes binding from ADR-0164 onward. The adversarial pass may fold two into one; it may not drop a
supersession listed here without a reason written into the record.

| record | executes | supersedes in part | cites without superseding |
| --- | --- | --- | --- |
| **the stage lives on the run ref** | D21-41 a; D21-42 8d's two defaults | ADR-0105 clause 3 (the `Story:` value delegation to design §5.4's row); ADR-0155 decisions 1, 2 and 5(a), and its bullets *"A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes."* and *"A `Story:` value carrying a stage, a boundary, or any third token, until the grammar is widened by a record that supersedes `0105` clause 3."*; ADR-0157 decisions 4 and 5, and its bullet *"Describing a lane as resumable across processes, or a lane's completion as anything other than within-one-process."* | ADR-0109 decisions 1, 3 and 4 (**decision 3 is NOT superseded**: resume reads all three axes once `Slice-Verdict:` lands); ADR-0161 decision 1 (`stages_entered` keeps replays idempotent); ADR-0159 |
| **the tier-1 writer, git-first, owned by the kernel** | D21-41 e (second half); D21-42 8a, 8c, 24b's placement | ADR-0158 decision 3 and its bullet *"Claiming AC-160, AC-161, AC-162 or AC-163 is met, on any axis"* — **only** for the criteria this stage measures (§8e.13); ADR-0155 decision 5(b) and its bullet *"Stating or implying that AC-162 is satisfied for stage changes 2..N of a lane"*, on the same condition (§8e.12, finding 3) | ADR-0003 (the port **count** stays five; a method is a surface change); ADR-0105 clauses 1, 2, 4, 8 and 9; ADR-0109 decision 2; ADR-0118 clause 4 |
| **`Gate:` carries kind and reach** | D21-42 8b | design §5.4's `Gate:` grammar row; ADR-0105 clause 3's value delegation, for `Gate:` | ADR-0105 clause 10 (a fresh mark per re-reach); `s7d` §7d.18's RAISED widening item, which this discharges |
| **the slice is declared, and sealed on both outcomes** | D21-41 e (first half) | **nothing**, provided the declared slice may not equal the item id (ADR-0160's first bullet names *"the item id"*) | ADR-0160 decision 6; ADR-0162 decision 2 (scoped to its own work, so re-stated here, not superseded); ADR-0161 decisions 1–2; ADR-0159 (the refusal pattern) |
| **`worktree_assigned`** and **`stage_empty`**, two kinds | D21-42 8d; D21-42 7 | nothing — minted by ADR-0116's route: *"The table row and the `rk-types` variant land in the same change."* | ADR-0105 clause 8; AC-529; the design §5 `reconcile_started` row's RT-47 members (§8e.4 says what happens to them) |

**Attendant edits, each dated and each in the commit that makes it true:** design §5.4's `Story:` and `Gate:` rows;
design §5's event table (two new rows); design §3.3's `EventStore` row (a dated note naming the tier-1 method); a
dated register note on **AC-444** (its drift counts are pinned to the grammar table, and it is corpus-bound Phase-2,
so the note re-pins nothing Phase 1 asserts); a dated re-amendment of `s7h` §7h.2, whose ADR-0155 decision-6 erratum
says *"the first stage boundary"*; the dated amendments to Checkpoint 7h guards (6) and (7) that §8e.12 spells out;
and a ledger entry.

### 8e.1 L3 first — the writer mutex halts, and is never recovered (D21-42 24b)

`crates/rk-store/src/conn.rs`'s `Writer::transaction` still reads:

```rust
let mut guard = self
    .conn
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
```

ADR-0118 clause 3 forbids exactly this spelling (*"not by `unwrap_or_else(std::sync::PoisonError::into_inner)`"*),
and clause 4 scopes the rule to **both** locks and says of this line that *"its repair is owed"*. The tier-1 writer
§8e.2 adds runs through this mutex, so the repair lands first, in its own commit.

**The halt's shape is fixed by record and not re-decided here.** Clause 1: *"No `StepError` is constructed, nothing
crosses a port boundary, no retry"*. So the adapter's `map_err(|e| io(e.to_string()))` may not carry a poison
across the port as `StepError::Transport`. The house mechanism is the kernel's `.expect(LOCK_HALTS)` panic, which
`rk-cli`'s hook turns into exit 70 (`is_lock_halt`). `rk-store` depends on `rk-types` alone and cannot name
`rk_kernel::runtime::LOCK_HALTS`, while `is_lock_halt`'s doc forbids a second copy of the string (*"two copies can
drift, and the drift would be invisible"*). So the constant moves **down** to `rk-types`, and
`rk_kernel::runtime::LOCK_HALTS` becomes a re-export of it. That is an implementation move, not a decision: there is
still one string.

🛑 **Red first.** `store::writer_poison_is_a_halt_and_never_recovered` is written before the fix. It poisons the
writer with a transaction closure that panics, catches that one panic in the test, calls `transaction` again, and
asserts the second call panics with a payload containing the constant **and writes no row**. It is RED at
`ca8f387`: the second call recovers the guard and commits.

### 8e.2 The tier-1 writer on the port (D21-41 e, second half; ADR-0158 decision 4)

**One method, on the one port that owns persistence.** `EventStore` gains
`fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError>`. `TierOneAnchor` carries
the trailer commit's sha and its tree. The method writes the `t1_*` row for the event's kind **and** the `t2_event`
row with `anchor_sha` set, in **one** transaction. The fold keeps reading `t2_event` through `query`, so no reader
changes. The method name is the record's. If the adversarial pass renames it, the tests, the lint in §8e.3 and
Checkpoint 8e rename in the same commit.

- **The port count stays five.** ADR-0003 says *"The kernel exposes exactly five ports"*. A method changes a port's
  surface, not the count. Checkpoint 7a is re-run whole at Checkpoint 8e, because it is the gate on the port surface.
- **`rk-ports`' AC-364 source parse moves from four to five, in the same commit, by the record.**
  `event_store_port::ac364_event_store_port_has_no_update_or_delete` asserts `methods.len()` is `4`, and its message
  says *"a fifth is a write path a record has to admit"*. The tier-1 record is that admission. The method name
  carries none of `FORBIDDEN_VERBS`, and the append-only rule is unchanged.
- **The fake and the conformance row.** `VecStore` implements the method (it keeps its anchored rows beside its
  events). `rk_ports::conformance::conform_event_store` gains a tier-1 arm that both rows run: an anchored append is
  queryable as the event, and the row reports its anchor.
- **The SQL lives in `rk-store`, not in the adapter.** ADR-0158's bullet *"Writing `t1_*` rows from `rk-cli`, or any
  SQL against them outside `rk-store`"* stands un-superseded. So `rk-store` gains a `tier_one` module of statement
  functions over a `rusqlite::Transaction`, and `SqliteStore::append_tier1` calls them inside `Writer::transaction`.
  Checkpoint 8e asserts that no code line under any `crates/*/src/` outside `crates/rk-store/src/` names a `t1_`
  table.

**The schema frictions, resolved at the writer and not by a migration.** The brief measured two, and this file found
a third:

1. **`t1_stage_transition.tree_hash TEXT NOT NULL`.** Resolved by ADR-0105 clause 4, whose first sentence already
   answers it: *"When a checkpoint commit is due, the recorded tree is that commit's tree."* The anchor is the trailer
   commit, so the row's tree is `<anchor>^{tree}`, read through `GitOp::RevParse`, which is on the wrapper's read
   list. An empty commit's tree is its parent's, which is deterministic.
2. **`t1_stage_transition.from_stage TEXT NOT NULL`.** The lane-entry row (`stage_entry`'s `{"to": …}`) has no
   `from`, and `stage_entry`'s own doc says so. **Resolution: `t1_stage_transition` holds boundaries — rows with a
   `from` — and a lane entry is not a transition between two stages.** The entry mark's tier-1 durability is its
   `Story:` trailer plus the anchored `t2_event` row, and the rebuild reproduces exactly that split (§8e.9).
   **Rejected: a `0002` migration relaxing the column.** `rk-store`'s `SCHEMA_VERSION` is `1`, and the identity check
   refuses a store stamped lower with *"A migration from {} to {} is required; there is no implicit in-place
   migration"* (AC-180). A bump would refuse the shipped `.git/redkiln/store.sqlite3` and every fixture store, and it
   would break Checkpoint 7i's `redkiln cost` over the shipped store (§8e.12). If the record takes the migration
   anyway, it owns an explicit AC-180 migration and the 7i re-point in the same stage.
3. **`t1_gate.token TEXT NOT NULL UNIQUE` beside `answered INTEGER NOT NULL DEFAULT 0` — found here, not in the
   brief.** The only way to record an answer in that table is an `UPDATE`, and AC-364 forbids modifying an appended
   row. **Resolution: `t1_gate` holds openings.** `gate_answered`'s tier-1 durability is its `Gate:` trailer plus the
   anchored `t2_event` row. The record states that `answered` is never written, and design §5 carries a dated note
   saying so. A fresh token per re-reach (ADR-0105 clause 10) keeps `UNIQUE` satisfiable.

### 8e.3 Git-first, owned by the kernel (D21-42 8a)

**The decision:** *"The kernel owns it, through the journalled `GitOps` seam. It commits (zero writes on replay),
then appends the t1 and t2 rows with `anchor_sha`. `reconcile` heals a commit without a row."*

**One boundary function, in one module.** `rk-kernel` gains `runtime/tier_one.rs`. Its function takes a
`&mut rk_git::gitops::GitOps`, the store, the run-owned `RunOwnership` read from the store (§8e.4), the event and its
trailer. In order, it:

1. authorises the target through `rk_git::worktree::authorise_tier1_write`;
2. commits through `GitOps` (`--allow-empty`, the trailer, `-c core.hooksPath=<empty>` and `-c
   trailer.ifexists=add` — ADR-0105 clauses 2 and 3);
3. re-reads `HEAD` and the tree;
4. calls `append_tier1`.

A failed commit returns before step 4, so no row exists. That is AC-162's second clause **by construction**, and the
fault-injection test watches it. Git is not a port (ADR-0003's bullet *"Git may not be abstracted behind a port"*), so
the `GitOps` handle travels beside `Ports`, not inside it. `resume` gains it as a parameter on the live path, as
`ReplayPorts` already carries one on the replay path.

**Every tier-1 act on the tree routes through it:**

- the lane entry (`stage_entry`, today appended with `rt.append`);
- the advance (`Runtime::step`'s `AdvanceStage` arm, today `self.append(stage_transition_event(from, to))?;`);
- the gate opening (`OpenGate`, today appended in the kernel and committed later by `terminal_read`);
- the gate answer (`answer.rs`, today append-then-commit);
- the seal (§8e.8).

`story_trailer` and `terminal_read`'s commit are **deleted**, not re-ordered. ADR-0155's bullet *"Writing the
`Story:` trailer from `terminal_read`, or from anywhere after the store row is readable"* is then true by
construction.

**Zero writes on replay.** In `Mode::Replay`, `GitOps::run` *"matches on the mode *first* and returns the recorded
result"*. The boundary function's replay arm therefore performs no commit, and
`tier_one::replay_of_a_boundary_performs_zero_git_writes` reads `writes_performed()` before and after.
⚠️ **A residual the tier-1 record settles, because the decision does not.** The live `GitOps` needs a `GitJournal`
backed by the store, and no `EventKind` records a git invocation. The anchor already rides in `t2_event.anchor_sha`,
which is a column and not part of the payload. So the journal the replay compares keeps its bytes, and the committed
golden logs are **not re-recorded to go green**. The record picks between two routes and says which: a new tier-2
kind minted by ADR-0116's route, or the anchored row serving as the replay's journal entry.
`recorded_log::write_run_and_read_run_round_trip_every_recorded_input` and the golden-log replay must stay green
without their fixtures being touched.

**The heal — the converse of AC-163.** AC-163's instrument *"fails on a store row with no matching commit"*. Its
converse is a crash between step 2 and step 4: a trailer commit on `rk-run/<item>` whose sha anchors no row. At every
entry, after `reconcile_at_entry`, the kernel's `tier_one::heal` appends the missing rows for every reconciled mark
whose anchor is reachable from `rk-run/<item>` and absent from the store. This is git → store, the direction ADR-0109
decision 1 permits, and it writes nothing to git (AC-174). A mark whose anchor is unreachable heals nothing, and a
separate test covers that branch (§8e.11).

**AC-162's instruments, all three, named by the register.** *"a fault-injection test with a git backend that fails
the commit, asserting zero store rows afterwards; a second fault-injection fixture installing a hook that rewrites the
commit message, which must have no effect on the trailer commit … plus a reachability lint that the tier-1 store
writer is callable only from the commit path."* The lint is a source lint over **code lines only**, and it asserts that
every call site of `append_tier1` under `crates/*/src/` is inside `crates/rk-kernel/src/runtime/tier_one.rs`. It is a
floor. The two fault injections are the criterion, as the register says in terms.

⚠️ **`gate_expired` is tier 1 with no carrier — found here.** `EventKind::tier()` lists `GateExpired` under `Tier::One`,
and `trailer_axis()` maps it to `None`. `resume_and_read`'s expiry pass appends it to the store alone, which is the
shape ADR-0109's bullet forbids. ADR-0109 decision 2's own list (*"stage transitions, gate opened/answered, verdicts,
slice seals"*) does not name it, while design §5's row gives it tier `1`. The tier-1 record takes a position,
either carrying it on the widened `Gate:` axis (§8e.7) or reclassifying it by the deleted-database test. It is
listed in §8e.12 as an unassigned item, and it is not closed silently.

### 8e.4 The run-owned worktree is read from the store (D21-42 8d; AC-529; R-63)

**The decision:** *"A `worktree_assigned` row is written at run start. Every tier-1 write is checked against it and
refused with a typed rk-git error on mismatch."*

Today the two verbs disagree, which is R-63. `answer`'s `recorded_ownership` reads the pair off the last
`reconcile_started` row (RT-47, *"ROUTED, not decided"*). `run`'s `story_trailer` and `terminal_read` **rebuild it by
convention**: `resolved.common_dir.join("redkiln").join("worktrees").join(item)` on `format!("rk-run/{item}")`.

- **The kind.** `worktree_assigned { item, worktree, branch }` is minted by ADR-0116's route: a design §5 row and the
  `rk-types` variant in one change, with `tier()`, `ALL`, the completeness and snapshot tests, and the no-catch-all
  arms in `fsm.rs`, `breaker.rs` and `memo.rs`. The record answers the deleted-database question for it in terms. If
  the database is deleted, `answer` finds no row and refuses (fail-closed) until a `run` re-creates the worktree and
  writes one. The record states whether that is a tier-2 disposal or a next-action change.
- **It carries the item, and every reader filters by it.** `recorded_ownership` filters by kind alone, and
  `SqliteStore::query` ignores its selector — every `Query` variant *"returns everything"* (RT-4). With two items in
  one store, `answer` would take the last pair of **either** item. The target and the ownership would then both come
  from that one row, so `authorise_tier1_write`'s comparison would pass for the wrong run.
- **Every tier-1 write resolves its `RunOwnership` from this row**, through §8e.3's function, and never from `cwd` or
  a path convention. A mismatch is the existing typed `GitError::ForeignWorktree { target_path, target_branch,
  owned_path, owned_branch }`, which already names all four values AC-529 asks for.
- **RT-47's pair on `reconcile_started`.** The record either withdraws those members by a dated design §5 note, or
  keeps them as tier-2 context and says no reader resolves ownership from them. It may not leave two sources.
- **AC-529 becomes real on all three of its instruments.** A second worktree of the same repository on another
  branch attempts the write and gets the typed refusal, with `HEAD` unchanged on both branches and zero new rows. The
  run-owned worktree succeeds as the positive control. And the test assertion that the resolution reads the store,
  which is killed by a mutant that resolves by convention. That test gives the fixture a `worktree_assigned` row
  whose path **differs** from the convention path, because with equal paths the mutant is unobservable (the lesson
  of a fixture per branch).

### 8e.5 The stage lives in git: a stage-bearing `Story:` at every boundary (D21-41 a)

**The decision:** *"A stage-bearing `Story:` trailer on `rk-run/<item>` at every boundary, folded by `reconcile`,
seeding `through_step_seven` so `run` and `answer` both see it."*

- **The grammar.** The value becomes `<project> "/" <slug> SP <stage>`.
  - `classify`'s `Story` arm splits on whitespace first. Today it calls `split_qualified(value)` on the whole value.
    Unchanged, it would parse `redkiln-rs/x analysis` as the slug `x analysis`, which is ADR-0155 option 2's failure
    mode (*"That it type-checks is what makes it dangerous"*).
  - One token is the **legacy stage-less mark**. Two tokens are a stage-bearing mark. Three or more are drift, with a
    new `MalformedReason` surfaced and never coerced (AC-183).
  - The stage token contains no whitespace and no `/`, and the producer refuses a stage that violates this at the
    mint (the ADR-0161 decision 3 pattern).
- **The fold.** `reconcile` keeps `stories_committed` (membership) and gains a latest stage per `(project, slug)`.
  Latest wins, commits are folded oldest-first, and this is the rule `Slice-Verdict:` already uses (*"Latest wins:
  commits are folded oldest-first."*). A repeated **identical** `(project, slug, stage)` stays
  `DuplicateStoryTrailer` drift. The record states the order assumption the packet flagged: `read_commits` runs
  `log --all --reverse` without `--topo-order`. For one linear run ref, the default walk emits a child before its
  parent, so `--reverse` puts the parent first.
- **The one stage-less mark on the tree is admitted, read-only** (D21-42 8d's default).
  `rk-run/rk-fix-doctor-test-count` at `73c087a` carries `Story: redkiln-rs/rk-fix-doctor-test-count`. It folds as
  membership with **no stage**, so the seed falls back to the item record, which is today's behaviour exactly. A
  stage-less mark never resets a stage a later-or-earlier stage-bearing mark set. The producer never writes one
  again.
- **The seed.** `through_step_seven` keeps the `Outcome` its reconcile returns. `resolved.stage` becomes the item's
  reconciled latest stage, and falls back to the record only when git carries no stage-bearing mark for the item.
  `answer` resolves through `run::through_step_seven`, so both verbs see it. This is also the first time ADR-0109
  decision 1's `Reconciled<State>` reaches the planner rather than being discarded.
- **The mint.** The mint is §8e.3's boundary function, at entry and at every advance. The idempotence read changes
  from *"item already committed"* to *"this `(item, stage)` already committed"*. ADR-0161's `stages_entered` guard
  still forbids re-entry within a lane.
- **`doctor --rebuild` takes `to` from the marks.** A stage-bearing mark gives its own stage. A legacy mark falls back
  to `.redkiln/items/<slug>.toml`'s `stage`, which is the current rule, and it stays listed unresolvable when the
  record has none (AC-165).

⚠️ **Two forbids bullets the decision does not list, raised for the record's adversarial pass rather than taken
here** (§8e.12, finding 4). ADR-0157's *"Reading a second invocation's exit 0 as progress, or as evidence that the lane
finished."* becomes false to hold once a second invocation does progress. ADR-0161's *"Describing the lane loop as
covered by tests, or reading a green suite as evidence about it."* is what §8e.10's tests exist to contradict. Under
ADR-0092 rule 1, moving either one is a supersession.

### 8e.6 Every stage the lane may enter is entered and trailered; an empty one says so (D21-42 7)

**The decision:** *"Every `stage_order` stage whose rigor is not `forbidden` is entered and carries its trailer. An
empty stage records `stage_empty { reason }`."*

- **The set is the pack's declared order, filtered.** It is the same filter `Pack::successor` already applies
  (ADR-0153: *"successor skips forbidden and ends the lane"*). For `bug-standard` that is ten of twelve stages:
  `discovery` and `operate` are `forbidden`.
- **An empty stage is entered, trailered, and passed, and it is not skipped.** `triage` carries **0** of the shipped
  activities, and `s7h`'s record says the plan there is empty. Today the lane advances out of such a stage with no
  record of why nothing ran. The kind `stage_empty { stage, reason }` is minted by ADR-0116's route, and its reason is
  a closed set that the record fixes (for example, *no activity applicable at this rigor*). Its tier comes from the
  deleted-database test. The `Story:` mark carries the entry either way, so it is expected to be tier 2, and the
  record says so or says why not.
- **What `s8` gains, and what it does not.** D21-42 7 says *"s8 asserts set and order equality mechanically from the
  lane's own trailer commits"*. That amendment is `s8j`'s. This stage proves the property over a fixture lane
  (`lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order`), with no pack literal: the
  expected list is computed from the fixture pack's own `stage_order` and `stages`.

### 8e.7 `Gate:` carries kind and reach (D21-42 8b), and `terminal_read` reads the awaited gate (D21-42 24c)

**Why the triple is not enough, in the rebuild's own words.** `doctor/rebuild.rs`'s header says: *"The fold's
`gate_opened` arm REQUIRES `kind` and `deadline` and **derives** the token from `(kind, tree_hash, deadline,
reach)` … The `Gate:` trailer carries `(token, tree_hash, deadline)` and no `kind` or `reach`."* So no gate survives a
deleted database, and AC-161's *"the gate records the trailers carry"* has nothing to carry.

- **The value becomes `<token> SP <tree_hash> SP <deadline> SP <kind> SP <reach>`.** The record fixes each token's
  grammar. The kind contains no whitespace, and the mint refuses one that does
  (`tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint`). A three-token value becomes legacy drift under
  the widened grammar. There are **0** `Gate:` trailers across all refs today (`git log --all
  --format='%(trailers:key=Gate,valueonly=true)'` prints nothing), so no repository value moves. Only fixtures and
  F12's corpus are read differently, and that is AC-444's dated note.
- **The deadline's spelling is settled here, because the record and the code disagree — found here.** design §5.4
  says `deadline` is *"RFC 3339"*. Both producers write integer unix seconds: `terminal_read` builds
  `format!("{token_hex} {tree_hash} {deadline}")` from `as_i64`, and `answer.rs` builds
  `record.deadline().unix_seconds()`. ADR-0163 already found that *"no crate formats RFC 3339"*. The record picks one
  and amends the row to match.
- **An opening and an answer must be told apart in git — found here, and the decision does not name it.** `answer`
  writes *"THE SAME TRIPLE `run` writes"*. `reconcile` keeps the latest mark per token, so after a deleted database
  the rebuild cannot tell an answered gate from an open one, and the next action differs (`AwaitGate` against
  proceed). The widening record must carry the disposition (for example, a sixth token or a distinct answer form), or
  say in terms that AC-161 is not claimed for answered gates (§8e.12, finding 5).
- **`terminal_read` stops committing and reads the fold.** Its scan (`e.kind == …GateOpened &&
  e.payload.get("token").is_some_and(|t| !t.is_null())`) has *"no answered/void/expired filter"* (ADR-0161's recorded
  fourth finding). It re-commits a `Gate:` for a dead token and prints that the gate is open. After §8e.3 the commit
  belongs to the kernel's `OpenGate` act, so `terminal_read` only reports. It reports the token `fsm::awaiting_gate()`
  returns over the recorded journal, which excludes answered, void and expired tokens. A re-run after an answer
  therefore mints no second `Gate:` and prints no open gate.
- **The rebuild now writes gates.** `events_from_trailers`'s `Gate:` arm stops listing every mark unresolvable and
  emits the `gate_opened` rows the fold can derive. That discharges `s7d` §7d.18's RAISED widening item.

### 8e.8 `Slice-Verdict:` — the slice is declared, and sealed on both review outcomes (D21-41 e, first half)

**The identity is declared, not derived** (ADR-0159's pattern; ADR-0160's route).

- **The key.** `ItemRecord` (today `id`, `kind`, `stage`, `[facets]`) gains an optional `slice`. It is optional in
  the type so the refusal can name the item, as `stage` is.
- **The refusals, at the mint and by name.** ADR-0159 decision 2's siting: *"The refusal is at the mint, not at
  startup"*.
  - An absent `slice` refuses the seal by name.
  - A `slice` equal to the item id refuses. That is D21-41 e's rule.
  - A `slice` equal to the project name, or to any stage in the pack's `stage_order`, also refuses. ADR-0160's bullet
    names more than the item id: *"whose `<slice>` is the item id, the project, the stage, the run id, or any value
    not declared as a slice"*. The run id cannot be known at the record, and it contains a digest.
  - A slice with whitespace refuses. The value is *"exactly two whitespace-separated tokens"*.

  The cost of siting the refusal at the mint is stated rather than hidden: a fix lane without a slice walks to its
  review stage before it refuses. The record may move the refusal to entry, and if it does it says why ADR-0159's
  siting does not apply.
- **Cardinality.** A fix-lane item declares exactly one slice (D21-41 e: *"one per fix-lane item"*). The record says
  the same of the story item `s8j` mints.
- **The seal sits on the review stage's exit outcome, because the boundary cannot carry the rejecting verdict.**
  ADR-0161 decision 2: *"The lane loop returns a non-`Exit::Success` status immediately, **before** reading
  `advanced`."* A `changes-requested` outcome crosses no boundary. So the producer is `review_pass`'s outcome, which
  ADR-0162 wired without a stage literal:
  - `blocked.is_empty()` seals `approved`;
  - `Some(Exit::FindingsBlock)` seals `changes-requested` **before** it returns.

  Both seals go through §8e.3's function: the `Slice-Verdict: <project>/<slice> <verdict>` commit, then
  `t1_slice_seal` plus the anchored `slice_sealed` row. The record states that it reads ADR-0160 decision 6's *"at
  the review stage's boundary"* as *"at the review stage's exit outcome"*, and why that moves no sentence. If the
  adversarial pass finds that it does, the record takes it as a supersession in part.
- **`fsm::RunState::terminal()` puts a seal first — a risk measured at the stage's open, not assumed.** Its order is
  *"1. a `slice_sealed` → [`TerminalState::Sealed`]"*. After an `approved` seal at `review`, every later stage's
  recorded terminal state is `Sealed`. The stage's open measures whether any reader (`explain`, the replay
  comparison) misreads a lane that continued past review. If one does, the slice record says how the fold scopes a
  seal.
- **The ARMED lint is un-ignored in the commit whose real fixture run reaches the producer**, on ADR-0162's
  precedent that a real `redkiln run` over a fixture corpus reaches a producer. The runs are the binary:
  - `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` runs over `exit/findings-block/`, which
    exits 4, and reads `Slice-Verdict:` off `rk-run/<item>` with git's own parser;
  - `slice_verdict::approved_is_sealed_on_the_advance_path` runs over a new `slice/approved/` corpus, which exits 0.

  The `#[ignore = "ARMED: …"]` line is deleted in that commit. No erratum is owed for it, because it is source and
  not a record. ADR-0162 decision 2 (*"The `Slice-Verdict:` axis is NOT touched, and the checkpoint enforces that"*)
  was scoped to that record's work. The slice record cites it and re-states Checkpoint 7h guards (6) and (7), as
  §8e.12 spells out.
- 🛑 **No document claims the review gate is guarded against false approval** (ADR-0101 clause 5; S-SUBTLE-CB is
  UNRUN). A seal records a verdict. It says nothing about the verdict's calibration.

### 8e.9 The rebuild moves into `rk-adapter-sqlite`, and criterion (2) is re-measured (D21-42 8c)

**The decision:** *"Moves into rk-adapter-sqlite (`open_at` / `Capability::Rebuild`, the [s7d] §7d.18 RAISED item).
rk-cli's duplicated INSERT is retired, and criterion (2) is re-measured against ADR-0105 cl.9's real projection."* (The
bracketed `[s7d]` is inserted into the quotation here: the ledger's bare reference names `s7d`'s section.)

- **The duplicated statement goes.** `rebuild.rs` carries *"🛑 THE SECOND COPY OF `SqliteStore::append`'s
  STATEMENT"*. `rk-adapter-sqlite` gains `SqliteStore::open_at(path)`, which targets the staging file, and the rebuild
  writes through it. It writes the `t1_*` rows (boundaries, gate openings and seals, each anchored to its own
  trailer commit) and the anchored `t2_event` rows of tier-1 kinds, through `rk-store`'s `tier_one` statements. The
  git read and the item-record fallback stay in `rk-cli`, because the order *"git → store, never the reverse"* is
  the doctor's. Checkpoint 8e asserts that no `INSERT INTO` code line remains under `crates/rk-cli/src/`.
- **`Capability::Rebuild` is settled, one way or the other.** The port's `rebuild(&Snapshot)` takes an opaque
  `Snapshot` with no field set. Either the record gives it a meaning and the registry row flips, or the git rebuild
  stays an inherent adapter method and the row keeps its dated refusal, with the record saying which. The conformance
  row's `unsupported` list follows the choice.
- **Criterion (2) is re-measured against the real projection.** ADR-0105 clause 9 defines it: *"AC-184's **tier-1
  projection** = the tier-1 rows whose anchor is reachable from the clone's refs"*. `projection::rebuild_differential`
  cannot measure that. Its own header says *"The ROUTE is not measured, only the EQUALITY"*, and `rk-kernel` may not
  depend on an adapter (AC-484). So the anchored differential is a new `rk-cli` test,
  `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`:
  1. run a fixture lane that writes boundary, gate and seal rows;
  2. take the `t1_*` rows whose anchors `git merge-base --is-ancestor` finds reachable from the repository's refs, in
     canonical form;
  3. delete the store and run `doctor --rebuild`;
  4. compare under the canonical serialiser;
  5. add an adversarial row whose anchor is unreachable, which must fall outside the projection.

  The kernel test keeps its name. `s8` (2)'s filter selects it (`cargo test -p rk-kernel -- rebuild_differential`),
  and its header gains a dated line naming the anchored test as the half that measures the route. Adding the anchored
  test to `s8` (2)'s own block is `s8j`'s instrument amendment, and it is listed there rather than made here.

### 8e.10 The first end-to-end tests of `verbs::run::run` (D21-42 24c; D21-40 decision 2)

**The seam is one field, and the code says so in advance.** `run()` always builds
`rk_adapter_claude::ClaudeHarness::new(binder.into_config(), token.clone())`, so no test can hand it a
`FakeHarness`. `Startup`'s own doc says it is shaped for this: *"The three seams a verb's outer `run` takes, as one
value — so a fourth seam is one field rather than a fourth parameter on six functions."* So `Startup` gains
`harness: &dyn HarnessFactory`. `Startup::real()` builds `ClaudeHarness` exactly as today, and a test passes a factory
over `rk_ports::fakes::FakeHarness::scripted(…)`. Every test below calls **`rk_cli::verbs::run::run(`** by that name,
so `grep -rn "verbs::run::run(" crates/*/tests` stops returning nothing. That grep is a floor, and the tests are the
assertion.

**The fixture lane.** `pack-min` gains a lane variant with a declared `stage_order` that includes:

- a `forbidden` stage;
- an empty stage;
- a stage whose activity opens a gate;
- a review stage over `review-min`.

It also gains an item with a `slice`. Its `config.toml`, and every fixture repository a lane runs in, gains `[project]
name`. **That last edit is forced, and it is the one most likely to be missed.** Entry now mints a `Story:` mark
(§8e.6), and ADR-0159 decision 2 refuses at the mint. So every existing binary fixture that runs a stage would change
its exit from its row to `3`: AC-438's table, `cost`'s run, and `exit/findings-block/`. Checkpoints 7h (10a) and 7i
(3) are the witnesses. For the same reason, the item that `exit/findings-block/` runs gains a `slice`, or its seal
refuses and exit 4 becomes 3.

**The tests, in `crates/rk-cli/tests/it/lane_e2e.rs`:**

- `lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer` — open, then `answer`, then
  re-run. Git's parser counts exactly one opening `Gate:` and one answer mark on `rk-run/<item>`, and the re-run's
  stdout reports no open gate.
- `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane` — the stall ADR-0157 recorded,
  closed. Invocation 1 exits 0 at the gate, `answer` applies, and invocation 2 (a fresh `run`, which opens its own
  store) crosses the next boundary and mints that stage's mark. This is a **two-hop** walk, because a one-hop fixture
  cannot judge a multi-hop guard (ADR-0161).
- `lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order` — the list of stage tokens read
  off the run ref equals the pack's filtered `stage_order`, in order.
- `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason`.
- `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` — after hop 2 the store file is deleted. The next
  `run` seeds from the reconciled mark, and its stage-level next action is canonically equal to the one before the
  deletion (AC-161's own comparison).
- `lane_e2e::terminal_read_reports_only_an_awaited_gate` — an answered gate, a void gate and an expired gate, one
  branch each. None prints *"is open"* and none commits.
- `lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child` — the real `redkiln` binary with
  `RK_FIXTURE_MODE=replay:<frames>`: `run` exits 0 at a gate, then `redkiln answer`, then `redkiln run` crosses the
  next boundary. This is the one arm where "process" means a process.

### 8e.11 Named tests, red first, and the mutants

**Every test of a NEW behaviour is written before the code it covers and watched RED for the reason in its row.** A
test first seen green is not evidence (PIA-035). **Three rows are pins over behaviour that must NOT move**
(`projection::rebuild_differential`, `lane_order::adr0161_…` and `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record`).
They are green at entry by design, and each is shown able to fail by the mutant named for it: ADR-0161's `M10` for
the lane-order pin, `projection.rs`'s own source lint for the tier filter, and `S8E-M18` below for the legacy rule.
Each test is selected in Checkpoint 8e by its module-qualified name with `--exact`, and a bare function name under
`--exact` runs zero tests and exits 0.

| crate | test | red at entry because |
| --- | --- | --- |
| `rk-git` | `trailers::story_value_with_a_stage_parses_as_a_stage_bearing_mark` | the slug absorbs the stage |
| `rk-git` | `trailers::the_stage_less_legacy_mark_parses_and_carries_no_stage` | no stage field exists |
| `rk-git` | `trailers::a_story_value_with_three_tokens_is_malformed_drift` | it parses as a story |
| `rk-git` | `trailers::the_widened_gate_value_carries_kind_and_reach` | five tokens are legacy drift |
| `rk-git` | `trailers::a_gate_value_without_kind_and_reach_is_legacy_drift` | the triple is valid today |
| `rk-git` | `trailers::reconcile_folds_the_latest_stage_mark_per_item` | no per-item stage |
| `rk-git` | `trailers::a_repeated_identical_stage_mark_is_duplicate_story_drift` | slug-only duplicate |
| `rk-git` | `trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage` | no stage to keep |
| `rk-store` | `store::writer_poison_is_a_halt_and_never_recovered` | `into_inner` recovers |
| `rk-store` | `store::tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree` | no statement exists |
| `rk-ports` | `event_store_port::ac364_event_store_port_has_no_update_or_delete` (asserts five) | the method count is four |
| `rk-ports` | `fake_behaviour::vec_store_append_tier1_is_queryable_with_its_anchor` | no method |
| `rk-adapter-sqlite` | `conformance::sqlite_store_conforms_including_the_tier_one_arm` | no arm |
| `rk-adapter-sqlite` | `store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction` | `anchor_sha` is NULL |
| `rk-adapter-sqlite` | `store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows` | no `open_at` |
| `rk-kernel` | `tier_one::a_failed_commit_leaves_zero_store_rows` | the row is appended first |
| `rk-kernel` | `tier_one::a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit` | no kernel commit path |
| `rk-kernel` | `tier_one::the_commit_exists_before_any_row_is_readable` | store first |
| `rk-kernel` | `tier_one::the_tier_one_writer_is_called_only_from_the_boundary` | no writer, and its PREMISE asserts a call site exists |
| `rk-kernel` | `tier_one::replay_of_a_boundary_performs_zero_git_writes` | no git on the boundary |
| `rk-kernel` | `tier_one::a_commit_without_a_row_is_healed_at_entry` | no heal |
| `rk-kernel` | `tier_one::a_heal_writes_no_row_for_an_unreachable_anchor` | no heal (this branch's own fixture) |
| `rk-kernel` | `tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint` | no mint in the kernel |
| `rk-kernel` | `conformance::conformance_every_trailer_key_has_a_producer` (un-ignored) | `#[ignore]`, and 0 seals |
| `rk-kernel` | `projection::rebuild_differential` | green — the kernel half; its dated header line is new |
| `rk-kernel` | `lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again` | green — the regression witness for the seed change |
| `rk-cli` | the seven `lane_e2e::…` tests in §8e.10 | `verbs::run::run(` has no FakeHarness seam |
| `rk-cli` | `slice_verdict::approved_is_sealed_on_the_advance_path` | no seal |
| `rk-cli` | `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` | no seal |
| `rk-cli` | `slice_verdict::an_absent_slice_is_refused_by_name` | no key |
| `rk-cli` | `slice_verdict::a_slice_equal_to_the_item_id_is_refused` | no key |
| `rk-cli` | `worktree_assigned::a_tier1_write_from_a_foreign_worktree_is_refused_typed` | no row kind |
| `rk-cli` | `worktree_assigned::the_positive_control_from_the_run_owned_worktree_succeeds` | no row kind |
| `rk-cli` | `worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention` | convention path |
| `rk-cli` | `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` | no `t1_*` rows |
| `rk-cli` | `doctor::rebuild_recovers_gate_opened_from_the_widened_gate_value` | gates unresolvable |
| `rk-cli` | `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record` | green today — the legacy rule's pin |
| `rk-cli` | `root::adr0118_a_poisoned_writer_mutex_halts_exit_70` | the writer recovers |

The existing `story_axis::adr0155_…` tests and `doctor::ac427_…`'s synthetic repository are **re-pointed** in the
record's commit, and none is deleted. `story_value` gains the stage, and the old single-mark assertions become the
legacy-mark fixture.

**The mutants.** Each mutant is applied and restored **from Python, never the shell**: a backtick once swallowed a
restore, and the next backup captured the mutant. Each diff is read. Each must turn the named test RED **for the
named reason**, printed and read, not merely "the suite went red". Names carry an `S8E-` prefix, because `M7` and
`M10` are already taken by `s7h`.

| mutant | change | must redden |
| --- | --- | --- |
| S8E-M1 | `through_step_seven` seeds from the item record, ignoring the reconciled stage | `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane`, `lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted` |
| S8E-M2 | the mint's idempotence read restored to *"item already committed"* (ADR-0155's first boundary only) | `lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order` |
| S8E-M3 | `append_tier1` before the commit | `tier_one::a_failed_commit_leaves_zero_store_rows`, `tier_one::the_commit_exists_before_any_row_is_readable` |
| S8E-M4 | `append_tier1` drops the `t1_*` insert | `store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction`, `doctor::ac184_…` |
| S8E-M5 | the heal disabled | `tier_one::a_commit_without_a_row_is_healed_at_entry` |
| S8E-M6 | the heal ignores reachability | `tier_one::a_heal_writes_no_row_for_an_unreachable_anchor` |
| S8E-M7 | no seal on the `FindingsBlock` path | `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` |
| S8E-M8 | an absent slice defaults to the item id | `slice_verdict::an_absent_slice_is_refused_by_name` |
| S8E-M9 | ownership resolved by path convention | `worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention` |
| S8E-M10 | `terminal_read`'s old filter (latest tokened `gate_opened`) restored | `lane_e2e::terminal_read_reports_only_an_awaited_gate` |
| S8E-M11 | `unwrap_or_else(std::sync::PoisonError::into_inner)` restored | `store::writer_poison_is_a_halt_and_never_recovered`, `root::adr0118_a_poisoned_writer_mutex_halts_exit_70` |
| S8E-M12 | the boundary commits in `Mode::Replay` | `tier_one::replay_of_a_boundary_performs_zero_git_writes` |
| S8E-M13 | the `Gate:` value drops kind and reach | `trailers::the_widened_gate_value_carries_kind_and_reach`, `doctor::rebuild_recovers_gate_opened_from_the_widened_gate_value` |
| S8E-M14 | `stage_empty` not appended | `lane_e2e::an_empty_stage_records_stage_empty_with_its_reason` |
| S8E-M15 | the rebuild writes `t2_event` only | `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` |
| S8E-M16 | `reconcile` keeps the first stage mark, not the latest | `trailers::reconcile_folds_the_latest_stage_mark_per_item` |
| S8E-M17 | a legacy mark resets the stage | `trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage` |
| S8E-M18 | the rebuild drops the item-record fallback for a stage-less mark | `doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record` |

A mutant that survives is not waved through. It is either an **unobservable property**, which moves to a source lint
with its reason written down, as `projection.rs`'s tier filter did, or a **missing fixture branch**. Both are recorded
by the mutant's name.

### 8e.12 What moves in other checkpoints, how 7h and 7i stay runnable, and what this file found

**Checkpoint 7h is amended, dated, in the commit that un-ignores the lint, and re-run at Checkpoint 8e.** Guards (6)
and (7) assert the lint is ARMED and names `Slice-Verdict` alone. After §8e.8 both are RED by design. That is the
*"Checkpoint 7h guards (6)/(7) … amended before the seal lands"* the packet names, and the preamble's *"A checkpoint
that changed after it passed is provisional"* applies: 7h's §1 row becomes `passed 490c755, re-run pending`.

- **(6)** becomes: `conformance::conformance_every_trailer_key_has_a_producer --exact`, with its `test … ok` line and
  `1 passed; 0 failed; 0 ignored`.
- **(7)** becomes: the `ARMED: Slice-Verdict has 0 producers` string absent from `conformance.rs`, with `rc` exactly
  1.
- The block's bullet *"`Slice-Verdict:` has a producer. ❌"* is dated ✅ with the two `slice_verdict::` tests named.
- The same amendment repairs guard (10a)'s FAIL message (found here). It is `echo "FAIL: … a real `redkiln run` over
  the review fixture did not exit 4"`, and backticks inside double quotes are **command substitution**, so the FAIL
  path executes `redkiln run`.

**Checkpoint 7f** (`s8a`'s repair of it) counts ARMED lints. If `s8a`'s repaired form still expects one ignored
lint, or greps this lint's reason, this stage's discharge turns it RED. This stage measures that at its open and
amends the line, dated, in the same commit.

**How Checkpoints 7h and 7i stay runnable.** Both read `rk-run/rk-fix-doctor-test-count`, and 7i also reads the
shipped store and `redkiln cost`.

- **No lane runs against the product repository at this stage.** Every run is in a fixture repository. So the ref
  stays at `73c087a` with its one legacy mark. 7h's (2) (exactly one real `Story:` trailer) and (3) (the value
  `redkiln-rs/rk-fix-doctor-test-count`, slug equal to the item) keep reading what they read today, and the legacy
  mark is exactly what §8e.5 admits read-only. Checkpoint 8e asserts the ref's sha and the mark's value at its start
  and at its end.
- 🛑 **Running `redkiln run rk-fix-doctor-test-count` under this stage's binary would break 7h on its own.** It would
  add stage-bearing marks, 7h's (2) would count more than one, and (3)'s slug would carry a stage token. D21-42 18
  keeps that item *"the untouched fixture item"*, and `s8j` re-points 7h and 7i when it mints the fresh item. Until
  then nobody runs it.
- **The shipped store keeps schema version 1**, because §8e.2's frictions are resolved at the writer. So `redkiln
  cost` still opens it, and 7i's `mode=ro` probe reads the same `t2_event`. Checkpoint 8e backs up and restores
  `.git/redkiln/store.sqlite3` (with its WAL, SHM and the `rebuilt` sidecar) around `s8` (2)'s `doctor --rebuild`,
  and it byte-compares the restore.
- **The binary fixtures 7h (10a) and 7i (3) run get `[project] name` and, for `findings-block`, a `slice`**, in the
  commit that makes entry mint (§8e.10). Their exits do not move.

**Findings made while authoring this file, each measured, none owned by a decision** (the operator's to route):

1. **`gate_expired` is tier 1 with no trailer axis** (§8e.3). It is store-only today, which ADR-0109's bullet forbids,
   and ADR-0109 decision 2's list omits it.
2. **`t1_gate` implies an `UPDATE` AC-364 forbids** (§8e.2, friction 3).
3. **Claiming AC-162 needs more than D21-41 e names.** ADR-0155 decision 5(b) (*"No document may state that AC-162
   is met for this axis, or for any."*) and its bullet, and ADR-0158 decision 3, are `## Decision` text. D21-41 e's
   *"ADR-0158's forbids bullet on claiming them yields only once they are measured"* covers one bullet.
4. **Two forbids bullets the stage-location decision does not list** (§8e.5): ADR-0157's on a second invocation's
   exit 0, and ADR-0161's on describing the lane loop as covered by tests.
5. **An answered gate is indistinguishable from an open one in git** (§8e.7). Unless the `Gate:` widening carries
   the disposition, AC-161 cannot hold for an answered gate after a deleted database.
6. **The `deadline` spelling** — RFC 3339 in design §5.4, integer seconds in both producers (§8e.7).
7. **A multi-item store folds items together.** `SqliteStore::query` ignores its selector (RT-4),
   `stage_transition_event` carries no `item`, and `rebuild.rs` says of the kernel's fold that *"`fsm::fold`
   partitions nothing"*. A second item's `run` in the same store would fold the first item's transitions, and
   `stages_entered` could then refuse its advance. `s8` runs a fix item **and** a story item (criteria (1) and (1b))
   in one repository. §8e.4 fixes the one reader this stage owns. The fold's partition has no owner.
8. **D21-42 18 versus the packet's "added to the shipped items".** The packet's (e) work line adds `slice` to the
   shipped fix and story items. D21-42 18 keeps `rk-fix-doctor-test-count` untouched. This stage follows D21-42 18:
   no shipped item gains a key here, and `s8j`'s fresh item carries its slice.
9. **Two stale comments on the tree** (hygiene, not this stage's): `fsm.rs`'s *"TOTAL OVER THE CLOSED 29"* over an
   `EventKind::ALL` of `38`, and `story_trailer`'s claim that a subject containing `": "` is parsed as a trailer.
   `read_commits` splits `%s` from `%b` and parses the body only, so that is true only of `story_axis`'s own test
   helper, which feeds the subject to `parse_body`. The tracker's `s7h` row records `answer.rs`'s subject as *"the
   same shape"*. It is re-measured before anyone repairs it.

### 8e.13 Acceptance set

**Claimed: AC-161, AC-162, AC-163, AC-184 (re-measured), AC-529, AC-172 (the reconciled state now reaches the
planner), AC-183 (a malformed `Story:` or `Gate:` value is drift, never coerced), AC-364 (the append-only port with
five methods), AC-518 (every commit is on `rk-run/<item>`).** AC-161 and AC-162 are claimed **over fixture
repositories, as measured**, and only once the supersessions in the records table are accepted. Until the operator's
ledger line flips those records, no document states that either is met.

**Excluded:**

- **AC-160**, the `trybuild` tier-annotation fixture: not this stage's, and it does not depend on `t1_*`.
- **AC-164**, whose truncation is `s8` (2)'s kernel half, unchanged here.
- **AC-444**, corpus-bound and Phase-2: the dated note only.
- **AC-473 and AC-521**: `s8`'s, over real runs.
- **AC-245**'s gate `tree_hash`: `s7b-ii`'s, cited for clause 4's derivation only.

### 8e.14 Checkpoint 8e

- **The writer mutex halts** (ADR-0118 clause 3), and no poison recovery remains in `rk-store`. *(24b)*
- **`EventStore` has five methods**, the fake and both conformance rows exercise the tier-1 arm, and design §3.3
  carries the dated note. *(D21-41 e)*
- **Every tier-1 act is git-first through the kernel.** AC-162's two fault injections pass, the reachability lint
  passes, replay performs zero git writes, and a commit without a row heals. *(8a)*
- **No `t1_` SQL outside `rk-store`, and no `INSERT` left in `rk-cli`.** *(8c; ADR-0158's bullet)*
- **The lane crosses invocations and processes**, every non-forbidden stage is entered and trailered in declared
  order, and an empty stage records why. *(D21-41 a; D21-42 7; 24c)*
- **`Slice-Verdict:` is sealed on both outcomes by a real fixture run**, and the ARMED lint is discharged. *(D21-41 e)*
- **A foreign-worktree tier-1 write is refused, typed, with ownership read from the store.** *(8d; AC-529)*
- **The anchored differential holds**, and the whole suite, fmt and clippy are green. *(8c)*
- **Checkpoints 8d, 7a and 7h (as amended) re-run whole from their own bytes, and so do `s8` (2) and (3)'s asserted
  half**, with the store restored byte-for-byte around the rebuild.
- **`rk-run/rk-fix-doctor-test-count` and its legacy mark are unchanged**, so 7h and 7i stay runnable.

**BEFORE (PIA-035): on the entry tree these lines FAIL:** (2)'s selections (the tests do not exist), (3) (the ARMED
string is present), (4) (`conn.rs`'s `into_inner`), (5) (four methods), (6) (no caller), and (7)'s `rk-cli` `INSERT`
line (`rebuild.rs`). A block that passed on `ca8f387` would be asserting nothing.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a SCRIPT (save it and `bash` it). Every guard ends in `exit 1`, and the two helper functions
# below exit the script, which only holds outside a subshell.
#
# Conventions, each one paid for on this tree: every cargo reading is `out=$(…); rc=$?` and never a pipe's status
# (under build-lock contention cargo exits 0 having run ZERO tests); every selection is `--test it`, MODULE-QUALIFIED
# and `--exact`, asserting each test's own `... ok` line AND an exact count, because a bare name under `--exact` runs
# zero tests and exits 0 (measured at s7h guard (10a)); source greps read CODE lines only, anchored to grep's
# `path:LINE:` prefix, because a guard can be satisfied by the prose beside it; and every synthetic control proves a
# guard can go RED. Scratch files live under .git/ (Windows Python cannot see Git Bash's /tmp).
scratch=.git/cp8e-scratch
rm -rf "$scratch" && mkdir -p "$scratch/store-backup" || { echo "FAIL: the scratch directory $scratch could not be made"; exit 1; }
CODE_ONLY='^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'

# ---- (0) BEFORE READINGS, taken before any cargo run and before any prune.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }
ref_before=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ -n "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count is absent — Checkpoints 7h and 7i read it (see this file's section 8e.12)"; exit 1; }
case "$ref_before" in
  73c087a*) ;;
  *) echo "FAIL: rk-run/rk-fix-doctor-test-count is at $ref_before, not 73c087a — something ran the untouched fixture item (D21-42 18) and 7h guards (2)/(3) will not hold"; exit 1 ;;
esac
legacy=$(git log -1 --format='%(trailers:key=Story,valueonly=true)' "$ref_before"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git could not read the legacy mark (rc=$rc)"; exit 1; }
legacy=$(printf '%s\n' "$legacy" | grep .)
[ "$legacy" = "redkiln-rs/rk-fix-doctor-test-count" ] || { echo "FAIL: the one real Story: mark reads '$legacy', not the stage-less legacy value this stage admits read-only"; exit 1; }
test -d crates/rk-adapter-toolchain-node/tests/fixtures/project/node_modules || { echo "FAIL: the node fixture has no node_modules — run pnpm install --frozen-lockfile --dir crates/rk-adapter-toolchain-node/tests/fixtures/project first, or 10 node tests fail for a reason unrelated to this stage"; exit 1; }

# ---- (1) BUILD-OR-FAIL, before any binary is read (Halt 1).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8e NOT RUN (Halt 1, Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists — every binary-arm test below would be about nothing"; exit 1; }

# ---- (2) THE NAMED TESTS, per crate, each selected exactly. `sel <package> <count> <names…>`.
sel() {
  pkg=$1; want=$2; shift 2
  [ "$#" -eq "$want" ] || { echo "FAIL: the $pkg list names $# tests and its count says $want — the list and the assertion disagree"; exit 1; }
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the $pkg selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -30; exit 1; }
  for t in "$@"; do
    printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted, still ignored, or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
  done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed; 0 ignored" || { echo "FAIL: the $pkg selection did not run exactly $want tests with none ignored"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
# CONTROL: a filter that matches nothing EXITS 0. This is why `sel` asserts each ok line and an exact count.
out=$(cargo test -p rk-git --test it -- trailers::s8e_control_no_such_test --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the zero-match control did not run (rc=$rc)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed' || { echo "FAIL: CONTROL — a zero-match filter did not report 0 passed, so this block cannot tell it from a real run"; exit 1; }

sel rk-git 8 trailers::story_value_with_a_stage_parses_as_a_stage_bearing_mark trailers::the_stage_less_legacy_mark_parses_and_carries_no_stage trailers::a_story_value_with_three_tokens_is_malformed_drift trailers::the_widened_gate_value_carries_kind_and_reach trailers::a_gate_value_without_kind_and_reach_is_legacy_drift trailers::reconcile_folds_the_latest_stage_mark_per_item trailers::a_repeated_identical_stage_mark_is_duplicate_story_drift trailers::a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage
sel rk-store 2 store::writer_poison_is_a_halt_and_never_recovered store::tier_one_statements_insert_a_stage_transition_with_its_anchor_and_tree
sel rk-ports 2 event_store_port::ac364_event_store_port_has_no_update_or_delete fake_behaviour::vec_store_append_tier1_is_queryable_with_its_anchor
sel rk-adapter-sqlite 3 conformance::sqlite_store_conforms_including_the_tier_one_arm store::append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows
sel rk-kernel 11 tier_one::a_failed_commit_leaves_zero_store_rows tier_one::a_hook_rewriting_the_message_has_no_effect_on_the_trailer_commit tier_one::the_commit_exists_before_any_row_is_readable tier_one::the_tier_one_writer_is_called_only_from_the_boundary tier_one::replay_of_a_boundary_performs_zero_git_writes tier_one::a_commit_without_a_row_is_healed_at_entry tier_one::a_heal_writes_no_row_for_an_unreachable_anchor tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint conformance::conformance_every_trailer_key_has_a_producer projection::rebuild_differential lane_order::adr0161_a_stage_the_lane_has_entered_is_not_entered_again
sel rk-cli 18 lane_e2e::run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane lane_e2e::every_non_forbidden_stage_is_entered_and_trailered_in_declared_order lane_e2e::an_empty_stage_records_stage_empty_with_its_reason lane_e2e::the_stage_seeds_from_git_when_the_store_is_deleted lane_e2e::terminal_read_reports_only_an_awaited_gate lane_e2e::the_binary_crosses_a_process_boundary_over_the_fixture_child slice_verdict::approved_is_sealed_on_the_advance_path slice_verdict::changes_requested_is_sealed_on_the_findings_block_path slice_verdict::an_absent_slice_is_refused_by_name slice_verdict::a_slice_equal_to_the_item_id_is_refused worktree_assigned::a_tier1_write_from_a_foreign_worktree_is_refused_typed worktree_assigned::the_positive_control_from_the_run_owned_worktree_succeeds worktree_assigned::ownership_is_read_from_the_store_not_from_cwd_or_convention doctor::ac184_rebuild_differential_over_anchored_tier_one_rows doctor::rebuild_recovers_gate_opened_from_the_widened_gate_value doctor::the_legacy_stage_less_mark_rebuilds_from_the_item_record root::adr0118_a_poisoned_writer_mutex_halts_exit_70

# ---- (3) THE ARMED LINT IS DISCHARGED — its ignore reason is gone, not reworded (ADR-0160's bullet; D21-41 e).
test -f crates/rk-kernel/tests/it/conformance.rs || { echo "FAIL: conformance.rs is absent (Halt 8)"; exit 1; }
hits=$(grep -nF 'ARMED: Slice-Verdict has 0 producers' crates/rk-kernel/tests/it/conformance.rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the ARMED reason is still present or unreadable (grep rc=$rc) — the lint was not discharged in the commit whose fixture run reaches the producer"; printf '%s\n' "$hits"; exit 1; }

# ---- (4) L3: NO POISON RECOVERY IN rk-store (ADR-0118 clauses 3 and 4). A floor; (2)'s two poison tests are the assertion.
raw=$(grep -rnE 'into_inner|PoisonError|catch_unwind' crates/rk-store/src/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-store poison search could not run (grep rc=$rc)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$hits" ] || { echo "FAIL: a poison recovery on a CODE line in rk-store (D21-42 24b):"; printf '%s\n' "$hits"; exit 1; }

# ---- (5) THE PORT SURFACE: five methods on EventStore, and the design carries the dated note (ADR-0003's count stays five).
test -f crates/rk-ports/src/event_store.rs || { echo "FAIL: the EventStore port file is absent (Halt 8)"; exit 1; }
n=$(grep -cE '^[[:space:]]*fn [a-z0-9_]+\(' crates/rk-ports/src/event_store.rs); rc=$?
[ "$rc" -le 1 ] && [ "$n" -eq 5 ] || { echo "FAIL: EventStore declares $n methods (grep rc=$rc), expected 5 — design section 3.3's four plus the tier-1 writer"; exit 1; }
hits=$(grep -nF 'append_tier1' references/2026-09-08-architecture-design.md); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the design carries no dated note naming the tier-1 method (rc=$rc) — a port-surface change lands with its design row"; exit 1; }

# ---- (6) AC-162's REACHABILITY FLOOR: every call of the writer is in the kernel's boundary module. The lint test in
# ---- (2) is the structural assertion and the two fault injections are the criterion; this names a stray caller by line.
raw=$(grep -rnF '.append_tier1(' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the reachability search could not run (grep rc=$rc)"; exit 1; }
calls=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -n "$calls" ] || { echo "FAIL: no code line calls append_tier1 — a writer nothing calls is the s7g lesson again"; exit 1; }
stray=$(printf '%s\n' "$calls" | grep -v '^crates/rk-kernel/src/runtime/tier_one\.rs:')
[ -z "$stray" ] || { echo "FAIL: append_tier1 is called outside the kernel's tier-one boundary:"; printf '%s\n' "$stray"; exit 1; }

# ---- (7) NO t1_ SQL OUTSIDE rk-store (ADR-0158's bullet), AND rk-cli's DUPLICATED INSERT IS RETIRED (8c).
raw=$(grep -rnE 't1_(stage_transition|gate|slice_seal)' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the t1_ search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-store/src/' | grep .)
[ -z "$code" ] || { echo "FAIL: a code line outside rk-store names a t1_ table:"; printf '%s\n' "$code"; exit 1; }
raw=$(grep -rn 'INSERT INTO' crates/rk-cli/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-cli INSERT search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE "$CODE_ONLY" | grep .)
[ -z "$code" ] || { echo "FAIL: rk-cli still carries an INSERT on a code line — the rebuild's second copy (s7d section 7d.18) is not retired:"; printf '%s\n' "$code"; exit 1; }
# CONTROL: the same filter KEEPS a code line and DROPS a comment line, so the two emptiness checks above can go red.
ctl=$(printf '%s\n' 'crates/rk-cli/src/x.rs:9:        tx.execute("INSERT INTO t1_gate (token) VALUES (?1)", p)?;' 'crates/rk-cli/src/x.rs:10:    // INSERT INTO t1_gate belongs to rk-store' | grep -vE "$CODE_ONLY" | grep -v '^crates/rk-store/src/' | grep -c .)
[ "$ctl" -eq 1 ] || { echo "FAIL: CONTROL — the code-line filter kept $ctl of one code line and one comment line"; exit 1; }
# terminal_read reads the fold's awaited gate — a floor beneath lane_e2e::terminal_read_reports_only_an_awaited_gate.
raw=$(grep -nF 'awaiting_gate(' crates/rk-cli/src/verbs/run.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the awaiting_gate search could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -n "$code" ] || { echo "FAIL: no code line in run.rs calls awaiting_gate — terminal_read still scans for any tokened gate_opened (ADR-0161's fourth finding)"; exit 1; }

# ---- (8) THE WHOLE SUITE, no fail-fast, with a floor. 740 passed at ADR-0164's measurement, plus this stage's 40 new
# ---- tests and the one un-ignored lint. The exact figures are recorded in PROGRESS.md; the floor only refuses a loss.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
tot=$(printf '%s\n' "$out" | awk '/^test result:/ { p += $4; f += $6; i += $8; n += 1 } END { print p+0, f+0, i+0, n+0 }')
read -r passed failed ignored results <<< "$tot"
[ "$failed" -eq 0 ] && [ "$passed" -ge 781 ] && [ "$results" -ge 44 ] || { echo "FAIL: the suite read passed=$passed failed=$failed ignored=$ignored over $results results — the floor is 781 passed over at least 44 results"; exit 1; }
out=$(cargo fmt --all -- --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | grep -E '^(error|warning)' | head -20; exit 1; }

# ---- (9) THE WORKTREE PATH SET, compared BEFORE the re-run blocks, because Checkpoint 7h prunes.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] && [ -z "$stale" ] || { echo "FAIL: stale worktree registrations (rc=$rc):"; printf '%s\n' "$stale"; exit 1; }

# ---- (10) RE-RUN FROM THEIR OWN BYTES: Checkpoint 8d whole, 7a whole, 7h whole (as amended here), s8 (2) whole and
# ---- s8 (3)'s asserted half. The extractor takes the ONE bash block of a file that contains a needle, and refuses on
# ---- zero or two. The fence string is built from chr(96), so no triple backtick appears inside this block.
cat > "$scratch/extract.py" <<'PY'
import sys
src, needle, dst = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(src, encoding="utf-8").read()
fence = chr(96) * 3
opener = fence + "bash" + chr(10)
blocks, pos = [], 0
while True:
    a = text.find(opener, pos)
    if a < 0:
        break
    b = text.find(chr(10) + fence, a + len(opener))
    if b < 0:
        break
    blocks.append(text[a + len(opener):b + 1])
    pos = b + 4
hits = [blk for blk in blocks if needle in blk]
if len(hits) != 1:
    print("EXTRACT-FAIL: %d of %d bash blocks in %s contain the needle" % (len(hits), len(blocks), src))
    sys.exit(1)
open(dst, "w", encoding="utf-8", newline=chr(10)).write(hits[0])
print("EXTRACT-OK " + dst)
PY
test -s "$scratch/extract.py" || { echo "FAIL: the extractor was not written"; exit 1; }
# CONTROL: the extractor REFUSES a needle no block carries, so a renamed checkpoint cannot be skipped silently.
out=$(python "$scratch/extract.py" runbook/phase-1/s7a-ports-and-fakes.md 'S8E-CONTROL-NO-SUCH-NEEDLE' "$scratch/none.sh" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: CONTROL — the extractor accepted a needle no block contains"; exit 1; }
ext() {
  test -f "$1" || { echo "FAIL: $1 does not exist — the checkpoint it holds cannot be re-run (Halt 8)"; exit 1; }
  out=$(python "$scratch/extract.py" "$1" "$2" "$3" 2>&1); rc=$?
  [ "$rc" -eq 0 ] && [ -s "$3" ] || { echo "FAIL: the block in $1 could not be extracted whole (rc=$rc)"; printf '%s\n' "$out"; exit 1; }
}

# (10a) CHECKPOINT 8d, the entry gate, whole. If it needs an environment variable, set it as s8d says; this block does not guess.
ext runbook/phase-1/s8d-step-grain-and-resolver.md 'echo "CP8D-END-OF-BLOCK reached"' "$scratch/cp8d.sh"
out=$(bash "$scratch/cp8d.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8d, re-run whole, exited $rc — this stage's entry gate no longer holds"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP8D-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8d exited 0 without reaching its end marker"; exit 1; }

# (10b) CHECKPOINT 7a whole — the gate on the port surface this stage changed. Its block has no end marker, so one is appended.
ext runbook/phase-1/s7a-ports-and-fakes.md 'pub store: Arc<Mutex<dyn EventStore>>' "$scratch/cp7a.sh"
printf '\necho "CP7A-EXTRACTED-END reached"\n' >> "$scratch/cp7a.sh" || { echo "FAIL: the 7a end marker could not be appended"; exit 1; }
out=$(bash "$scratch/cp7a.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7a, re-run whole, exited $rc"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP7A-EXTRACTED-END reached' || { echo "FAIL: Checkpoint 7a stopped before its last line"; exit 1; }

# (10c) CHECKPOINT 7h whole, AS AMENDED BY THIS STAGE (section 8e.12). Its guards (6)/(7) are the discharge's witnesses.
ext runbook/phase-1/s7h-lane-completes.md 'echo "CP7H-END-OF-BLOCK reached"' "$scratch/cp7h.sh"
out=$(FIX_ITEM=rk-fix-doctor-test-count bash "$scratch/cp7h.sh" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 7h, re-run whole, exited $rc — if guard (6) or (7) names ARMED, the dated amendment did not land"; printf '%s\n' "$out" | tail -30; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP7H-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 7h exited 0 without reaching its end marker"; exit 1; }

# (10d) s8 CRITERION (2) whole — it runs doctor --rebuild, which REPLACES the shipped store. Back up the store, its
# WAL, its SHM and the `rebuilt` sidecar; restore them whatever the block did; byte-compare; only then read its rc.
sd=$(git rev-parse --git-common-dir); rc=$?
[ "$rc" -eq 0 ] && [ -f "$sd/redkiln/store.sqlite3" ] || { echo "FAIL: no shipped store under $sd/redkiln (rc=$rc)"; exit 1; }
sd="$sd/redkiln"
for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
  if [ -f "$sd/$f" ]; then cp -p "$sd/$f" "$scratch/store-backup/$f" || { echo "FAIL: could not back up $f — criterion (2) NOT RUN"; exit 1; }; fi
done
ext runbook/phase-1/s8-exit-criterion.md 'echo "(2) asserted: AC-184 differential' "$scratch/cp8-2.sh"
printf '\necho "CP8-2-EXTRACTED-END reached"\n' >> "$scratch/cp8-2.sh" || { echo "FAIL: the (2) end marker could not be appended"; exit 1; }
out2=$(bash "$scratch/cp8-2.sh" 2>&1); rc2=$?
for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
  rm -f "$sd/$f"
  if [ -f "$scratch/store-backup/$f" ]; then cp -p "$scratch/store-backup/$f" "$sd/$f" || { echo "FAIL: RESTORE of $f failed — restore it by hand from $scratch/store-backup before anything else runs"; exit 1; }; fi
done
for f in store.sqlite3 store.sqlite3-wal store.sqlite3-shm rebuilt; do
  if [ -f "$scratch/store-backup/$f" ]; then cmp -s "$scratch/store-backup/$f" "$sd/$f" || { echo "FAIL: the restored $f differs from its backup — the shipped store is not the one this block found"; exit 1; }; fi
done
[ "$rc2" -eq 0 ] || { echo "FAIL: s8 criterion (2), re-run whole, exited $rc2"; printf '%s\n' "$out2" | tail -30; exit 1; }
printf '%s\n' "$out2" | grep -qxF 'CP8-2-EXTRACTED-END reached' || { echo "FAIL: s8 criterion (2) stopped before its last line"; exit 1; }

# (10e) s8 CRITERION (3), ASSERTED HALF — the block is run whole with ITEM UNSET. Its asserted half must pass, and it
# must then stop at exactly one FAIL, the operational arm's `set ITEM` (D21-40's re-measurement reads the same).
ext runbook/phase-1/s8-exit-criterion.md 'echo "(3) asserted: AC-013' "$scratch/cp8-3.sh"
out=$(env -u ITEM bash "$scratch/cp8-3.sh" 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: s8 (3) with ITEM unset exited $rc, expected 1 at the operational arm"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qxF '(3) asserted: AC-013 N-step kill-and-resume, digest equality, zero re-dispatch' || { echo "FAIL: s8 (3)'s asserted half did not report — kill_and_resume_n_step is red or unselected"; printf '%s\n' "$out" | tail -20; exit 1; }
fails=$(printf '%s\n' "$out" | grep '^FAIL')
[ "$fails" = "FAIL: set ITEM to the item the run is on" ] || { echo "FAIL: s8 (3) stopped somewhere other than the operational arm:"; printf '%s\n' "$fails"; exit 1; }

# ---- (11) AFTER: the fixture item's ref and mark are untouched, so 7h and 7i stay runnable.
ref_after=$(git rev-parse --verify --quiet refs/heads/rk-run/rk-fix-doctor-test-count); rc=$?
[ "$rc" -eq 0 ] && [ "$ref_after" = "$ref_before" ] || { echo "FAIL: rk-run/rk-fix-doctor-test-count moved during this block ($ref_before -> $ref_after)"; exit 1; }
rm -rf "$scratch" || { echo "FAIL: the scratch directory $scratch could not be removed"; exit 1; }

echo "CP8E-END-OF-BLOCK reached"
```

---
