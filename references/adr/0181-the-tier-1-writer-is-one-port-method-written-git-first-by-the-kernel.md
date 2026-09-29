---
id: "0181"
title: "**The tier-1 writer is one method on `EventStore`, `append_tier1(event, &TierOneAnchor)`, and it is written git-first by the kernel** (`s8e`'s record; D21-41 e, second half; D21-42 8a, 8c and 24b's placement; D21-43 N7 and N10's seam; D21-46 R2, the store half; D21-48 X10): the port gains a fifth method and keeps its count of five ports; the method writes the anchored `t2_event` row and, where the kind has one, its `t1_*` row in ONE transaction through `rk-store`'s `tier_one` statement functions; the `0002_gate_transitions` migration adds an append-only `t1_gate_transition`, drops the never-written `t1_gate`, adds `t2_event.item` and stamps schema version 2, and a version-1 store is refused by name and rebuilt, never migrated in place; the kernel commits first, re-reads `HEAD` and the tree, and only then appends, so a failed commit leaves zero rows by construction; `Snapshot` is the ordered list of anchored rows. Supersedes in part `0158` decision 3 and its bullet on claiming AC-160..AC-163, and `0155` decision 5(b) and its AC-162 bullet, only for the criteria `s8e` measures and only once measured"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (row e — *"The tier-1 writer is ADR-0158 decision 4's route: a tier-1 write method on `EventStore` with its fake and conformance row, called where the trailer commit is made and taking its `anchor_sha`. That is a port-surface change under ADR-0003."*), D21-42 (rows 8a, 8c and 24b–d), D21-43 (N6, N7 and N10), D21-46 (R2), D21-47 (the clause-5 rule) and D21-48 (X10); runbook/phase-1/s8e-tier-one-and-the-lane.md (the records table's second row, §8e.0's stage-open readings taken 2026-09-28 at `766854c`, §8e.2, §8e.3, §8e.9 and §8e.14); crates/rk-ports/src/event_store.rs, crates/rk-ports/src/fakes.rs, crates/rk-ports/src/conformance.rs, crates/rk-adapter-sqlite/src/lib.rs, crates/rk-store/src/migrations/0001_initial.sql, crates/rk-store/src/identity.rs and crates/rk-store/src/schema_lint.rs, as read 2026-09-28 at `766854c`; references/2026-09-08-acceptance-criteria.md AC-160 … AC-163, AC-166, AC-180 and AC-364; and D21-96 (the `answered` row's tree, dated amendment of 2026-09-28, package D2196-8e)
supersedes: ["0155", "0158"]
superseded_by: []
related: ["0003", "0092", "0105", "0109", "0118", "0131", "0155", "0158", "0182"]
evidence: []
consumers: []
---

## Context and problem

**The tier-1 store layer has no writer.** `0001_initial.sql` declares `t1_stage_transition`, `t1_gate` and
`t1_slice_seal` under the header *"These rows never exist without their trailer commit. A failed git write leaves ZERO
of them (AC-160..AC-163), which is why every one is anchored to a commit sha."* ADR-0158 decision 2 measured the rest:
*"all three tier-1 tables have **zero** writers and are **empty** in a live store; AC-160..AC-163 and AC-162 are
**vacuous on every axis**, `Gate:` included."* The port was `append` / `query` / `snapshot` / `rebuild`, and
`SqliteStore::append` wrote `"INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2, NULL)"`. Re-read at
`766854c` on 2026-09-28 (`s8e` §8e.0 item 2): the shipped `.git/redkiln/store.sqlite3` stamps schema version **1**
and holds `t1_stage_transition` 0, `t1_gate` 0, `t1_slice_seal` 0 and `t2_event` **11** (`dag_assembled` 1,
`gate_opened` 1, `reconcile_started` 7, `stage_transition` 2), every row with `anchor_sha` NULL.

**Every tier-1 write on the tree is store-first.** `Runtime::step`'s advance arm appends `stage_transition_event(from,
to)` and the `Story:` commit comes later, from `rk-cli`. `answer` appends `gate_answered` **before** it calls
`commit_with_trailers`. ADR-0109 decision 2 says tier 1 *"is written to git first"*, and its forbids list says
*"Forbids a tier-1 transition recorded only in the store, and forbids a store row for a transition whose trailer
commit failed."*

**`t1_gate` cannot hold N6's lifecycle.** Its `token TEXT NOT NULL UNIQUE` beside `answered INTEGER NOT NULL DEFAULT
0` admits one row per token, and the `answered` column can only move by an `UPDATE`, which AC-364 and D21-43 N6
(*"`t1_gate` is append-only and state is a fold."*) both forbid. D21-46 R2 took the route: *"**(A) A forward
migration to an append-only transitions table.** The store is disposable by design (ADR-0131 d6; the
deleted-database test), so old stores rebuild from git."*

**The operator took the decisions.** D21-41 e: *"The tier-1 writer is ADR-0158 decision 4's route: a tier-1 write
method on `EventStore` with its fake and conformance row, called where the trailer commit is made and taking its
`anchor_sha`. That is a port-surface change under ADR-0003."* D21-42 8a: *"**The kernel owns it, through the
journalled `GitOps` seam.** It commits (zero writes on replay), then appends the t1 and t2 rows with `anchor_sha`.
`reconcile` heals a commit without a row."* D21-42 8c: *"**Moves into rk-adapter-sqlite** (`open_at` /
`Capability::Rebuild`, the §7d.18 RAISED item). rk-cli's duplicated INSERT is retired, and criterion (2) is
re-measured against ADR-0105 cl.9's real projection."* D21-43 N7: *"**A tier-2 `git_op` row is minted** so replay
serves git writes from the journal. Golden logs are read, by stated rule, as "no git writes"."* D21-43 N10: *"The
commit and the content checkpoint are one boundary act."* This record writes them down, fixes the shapes the ledger
left to the record, and names what it supersedes.

## Decision drivers

- A tier-1 row may not exist without its commit (ADR-0109 decision 2's forbids list; AC-162's second clause).
- The port count is ADR-0003's (*"The kernel exposes exactly five ports"*); a method is a surface change, not a sixth
  port.
- AC-364: the store is append-only, and the port parse admits a new method only by a record.
- ADR-0158's bullet *"Writing `t1_*` rows from `rk-cli`, or any SQL against them outside `rk-store`"* stands.
- AC-180: an old store is refused by name and never migrated in place; the checked-in fixture is the instrument.
- A decision is taken once. The grammar of `Gate:` and `Story:`, the three new kinds and the seal are other records'
  (the `s8e` records table); this record owns the writer, its persistence surface and the kernel-owned order.

## Considered options

### Option — one port method, `append_tier1(event, &TierOneAnchor)`, over `rk-store` statement functions (taken)

D21-41 e names it. One method on the one port that owns persistence, taking the event and the anchor the commit
returned, writes the anchored `t2_event` row and the kind's `t1_*` row in one transaction. The fold keeps reading
`t2_event` through `query`, so no reader changes. The SQL lives in `rk-store`, so ADR-0158's bullet holds outright.

### Option — a second method per tier-1 table

`append_stage_transition`, `append_gate_transition` and `append_slice_seal` would each carry a typed row. Rejected:
three port methods for one act, each a separate AC-364 admission, and the anchored tier-2 rows of the same act
(`stage_empty`; N10's step row) would need a fourth. The event already names its kind.

### Option — write the `t1_*` row from `rk-cli`, beside the trailer commit

The shortest diff from `story_trailer`. Rejected: ADR-0158's bullet forbids it in terms, and D21-42 8a gives the order
to the kernel, through the journalled `GitOps` seam, so replay performs zero writes.

### Option — keep `t1_gate` and relax it (drop `UNIQUE`, keep `answered`)

Rejected by R2 and by N6. A column that can only change by `UPDATE` is the write AC-364 forbids, and a second row per
token would still leave `answered` meaningless.

### Option — migrate a version-1 store in place (`ALTER TABLE`, a data copy)

Rejected by AC-180 (*"no implicit in-place migration occurs"*) and by R2's ground: the store is disposable, and the
rebuild re-derives it from git.

## Decision

**1. `EventStore` gains one method, and the port count stays five.**
`fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError>`. ADR-0003's count is of
ports, and a method changes a port's surface, not the count. `rk-ports`' AC-364 source parse,
`event_store_port::ac364_event_store_port_has_no_update_or_delete`, moves from four methods to **five** in the same
commit. Its message said *"a fifth is a write path a record has to admit"*: this record is that admission. The name
carries none of the parse's `FORBIDDEN_VERBS`, and the store stays append-only. The name is the record's. Its
anchored tier-2 use argues for a wider name such as `append_anchored`. If the adversarial pass renames it, the tests,
the reachability lint and Checkpoint 8e rename in the same commit.

**2. `TierOneAnchor` carries the boundary act's facts that are not payload.** It is an `rk-ports` type. It holds the
trailer commit's sha and that commit's tree, and, for a gate opening only, the opening stage run's `stage` and
`run_id` (`s8e` §8e.7; the final consistency pass of 2026-09-24). Those two are columns of the `opened` row, not
members of `gate_opened`'s payload, so no canonical event moves for them. A row's tree is the anchor commit's tree:
ADR-0105 clause 4's first sentence, *"When a checkpoint commit is due, the recorded tree is that commit's tree."*

**3. One transaction, two rows.** `append_tier1` writes the `t2_event` row with `anchor_sha` set and its interim
`item` (ADR-0182), and, when the event's kind has a tier-1 table, that table's row, in **one** transaction. If either
insert fails, neither row exists. The mapping is by kind and it is closed:

| kind | tier-1 row |
| --- | --- |
| `stage_transition` carrying `from` | `t1_stage_transition (item, from_stage, to_stage, anchor_sha, tree_hash)` |
| `stage_transition` with no `from` (a lane entry) | none: the anchored `t2_event` row only |
| `gate_opened` / `gate_answered` / `gate_expired` / `gate_answer_void` | `t1_gate_transition`, `state` `opened` / `answered` / `expired` / `voided` |
| `slice_sealed` | `t1_slice_seal (project, slice, verdict, anchor_sha)` |
| every other kind | none: the anchored `t2_event` row only (a tier-2 row of the same boundary act) |

- **`t1_stage_transition` holds boundaries.** A lane entry is not a transition between two stages; its tier-1
  durability is its `Story:` trailer plus the anchored `t2_event` row, and the rebuild reproduces exactly that split.
  The `from_stage NOT NULL` column is kept, because this is a meaning, not a workaround.
- **`gate_answer_void` is mapped although design §5 still lists it as tier 2.** D21-43 N6 makes every gate transition,
  the void included, a git-first tier-1 event. Moving its tier and producer is the `Gate:` record's; the mapping here
  is what that move writes through.
  *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's records minor.)* ~~although design §5 still
  lists it as tier 2~~ — no longer: package S8E-P4 amended design §5's `gate_answer_void` row to tier 1 (a `Gate: …
  voided` trailer), producer the kernel at entry, by **ADR-0184** (proposed). This bullet was true when S8E-P3 wrote
  it; the mapping is unchanged.
- **A tier-1 row never invents a member.** A `NOT NULL` member the event does not carry (the `item` of a boundary,
  a gate's `token`, a seal's `project`, `slice` or `verdict`) refuses the write as a transport fault naming the
  member, and the transaction rolls back to zero rows. A nullable member the payload does not carry (`t1_gate_transition`'s
  `tree_hash` on an expiry, `deadline`, `kind`, `reach`, `option_id`, `stage`, `run_id`) is written `NULL`.
- *(dated 2026-09-28, `s8e` build, package D2196-8e; D21-96.)* **An `answered` row's `tree_hash` is the gate's BOUND tree, read from its token's `opened` row.**
  D21-96: *"The live `answered` row of `t1_gate_transition` carries the gate's **bound** tree, taken from the gate record, so the tier-1 row equals what its mark carries and a rebuild reproduces it byte for byte. The answer's observed tree is not lost: it stays in the anchored tier-2 `gate_answer_void` row."* *(dated 2026-09-28, s8e build, package R2FIX-8e: the
ruling's words are quoted verbatim; its "tier-2" names the `t2_event` table the anchored row is stored in, not the
kind's tier, which is 1 under ADR-0184, *"`gate_answer_void` moves to tier 1"*. Nothing is reclassified.)* The mapping above wrote the `gate_answered` payload's `tree_hash`, the answer's OBSERVED tree; over a
  void that differs from the bound tree ADR-0184 decision 1 puts on every `Gate:` mark for the token (*"the gate's
  bound tree (ADR-0105 clause 4), the same on every mark for the token"*), so the rebuilt row differed from the live one
  and decision 11's claim failed (measured: *"only recorded: …answered|6aaf314e…"*, *"only rebuilt:
  …answered|6ae68e31…"*). The writer — `rk_store::tier_one::append`, which the live act, the heal and the rebuild all
  call — now takes an `answered` row's tree from the `opened` row of the same token in `t1_gate_transition`, the gate
  record the store holds. The payload is unchanged, so no canonical event moves; the observed tree stays in the
  anchored `t2_event` rows (the `gate_answered` payload's `tree_hash` and the `gate_answer_void` payload's
  `observed_tree_hash`). **A reading this record states while `proposed`:** an `answered` row for a token the store
  holds no `opened` row for is refused as `StoreError::TierOne` naming the rule, and the transaction leaves zero rows —
  the store neither falls back to the payload's tree nor writes `NULL` (this decision's *"A tier-1 row never invents a
  member"*). No live path reaches it: `Act::gate` needs the fold's gate record, an opening is git-first and its row
  precedes the answer's, and the heal and the rebuild walk the marks in ref order. The other states are unchanged: an
  `opened` row's tree is its payload's (the bound tree), a `voided` row's is the payload's `bound_tree_hash`, and an
  `expired` row's is `NULL`.

**4. The SQL lives in `rk-store`, and its entry points are named.** `rk-store` gains `tier_one`, statement functions
over a `rusqlite` connection (`insert_stage_transition`, `insert_gate_transition`, `insert_slice_seal`, and `append`,
the transaction body `append_tier1` runs), and `events`, the `t2_event` insert and the selector's `SELECT`. The
callers of `rk_store::tier_one` are `SqliteStore::append_tier1` and, when `s8e` §8e.9 lands it, `SqliteStore::rebuild`,
and nothing outside `rk-store` and `rk-adapter-sqlite` names the module. **ADR-0158's bullet *"Writing `t1_*` rows
from `rk-cli`, or any SQL against them outside `rk-store`"* is NOT superseded.** Its second clause holds outright: no
code line outside `crates/rk-store/src/` names a `t1_` table. Its first clause is read as D21-48 X10 reads it:
*""From `rk-cli`" names where the write runs. The adapter's `Capability::Rebuild` owns the SQL, so there is no
supersession. The record states the reading."* `doctor --rebuild` is invoked from `rk-cli`, and the rows it causes
are written by the adapter's `rebuild`.

**5. The persistence half of D21-46 R2: `0002_gate_transitions`.** `rk-store` gains the pair
`migrations/0002_gate_transitions.sql` and `.keys` (the schema lint's form: *"each migration is a pair"*), and
`schema_lint::all()` lists it. It:
- creates **`t1_gate_transition`**, one row per transition: `token`, `state` (`opened`, `answered`, `expired` or
  `voided`, by a `CHECK`), `tree_hash`, `deadline` (`TEXT`, the integer unix seconds of D21-43 N8 as a decimal),
  `kind`, `reach`, `option_id`, the opening run's `stage` and `run_id`, and `anchor_sha NOT NULL`, with
  `UNIQUE (token, state)` and no column a later write could update. Its `.keys` declares
  `t1_gate_transition.tree_hash = tree` and `t1_gate_transition.anchor_sha = sha` (AC-166). A fresh token per
  re-reach (ADR-0105 clause 10) keeps `(token, state)` unique, and a gate's state is the fold over its rows;
- **drops `t1_gate`**, whose rows are 0 in every store measured (`s8e` §8e.0 items 2 and 4). **`answered` is never
  written again, because the column no longer exists.** Design §5 carries no `t1_*` column list, so this record and
  the migration header carry the note;
- adds the interim `item` column to `t2_event` (ADR-0182), with an index;
- and `SCHEMA_VERSION` becomes **2**.

*(dated 2026-09-28, `s8e` build, package D2196-8e; D21-96.)* `t1_gate_transition.tree_hash` holds the gate's bound tree on every row that carries one — `opened`, `answered` (read from the `opened` row, decision 3's dated bullet) and `voided` — and `NULL` on `expired`. The `.keys` declaration `tree_hash = tree` is unchanged.

The grammar that writes these rows, the `Gate:` value's six tokens and whether the answered form carries the option,
is the `Gate:` record's (`s8e` records table, third row). This record lands the surface it writes through.

**6. Each migration runs once, and old stores are rebuilt, never migrated (AC-180).** A store's own
`PRAGMA user_version` counts the shipped migrations applied to it, so an existing store at this binary's schema
version applies none again, and a fresh store applies `0001` then `0002` in one transaction with its identity stamp.
The count is the runner's bookkeeping; the identity is still `rk_identity` (AC-369) and is not widened. The identity
check still refuses a version-1 store with *"A migration from 1 to 2 is required; there is no implicit in-place
migration"*, before any statement runs, inside the open's one transaction, so the file's bytes are unchanged.
`rk-store` checks in the register's own instrument, `crates/rk-store/tests/fixtures/store-v1.sqlite3` (minted on the
entry tree before `0002` existed, pinned `binary` in `.gitattributes`; `s8e` §8e.0), and
`store::a_checked_in_version_one_store_is_refused_and_its_bytes_are_unchanged` compares a refused copy's sha256 with
it. **The explicit migration is `doctor --rebuild`**, which re-derives a version-2 store from git into a staging file
without first opening the old one through the identity check (`s8e` §8e.9). What it drops is tier 2 by definition:
at `766854c` the shipped store holds the eleven `t2_event` rows named in the Context and no tier-1 row. The operator
backs the store up, runs the rebuild and records the before and after counts in the migration commit's body; that
run is the operator's gate, not this record's.

**7. Git first, owned by the kernel (D21-42 8a).** `rk-kernel` gains `runtime/tier_one.rs`, one boundary function
taking the `GitOps` handle, the store, the `RunOwnership` read from the store's `worktree_assigned` row, and one
boundary act (an optional trailer, the act's content, and the rows it anchors). In order, it:
1. authorises the target through `rk_git::worktree::authorise_tier1_write`;
2. commits through `GitOps` (`--allow-empty`, the trailer, `-c core.hooksPath=<empty>` and
   `-c trailer.ifexists=add`, ADR-0105 clauses 2 and 3);
3. re-reads `HEAD` and the tree;
4. calls `append_tier1` for every row of the act, each anchored to that commit.

A failed commit returns before step 4, so no row exists: AC-162's second clause **by construction**. Git is not a port
(ADR-0003's bullet *"Git may not be abstracted behind a port"*), so the `GitOps` handle travels beside `Ports`, not
inside it. An act may carry several rows, tier 1 and tier 2, each anchored to its one commit, and an act with neither a
trailer nor content is refused. **N10's seam:** at `s8e` the act's content has one arm, empty; `s8f`'s landed arm
changes what step 2 commits and not the order. Every tier-1 act routes through it: the lane entry, the advance, the
lane's end, every gate transition (opening, answer, expiry, void) and the seal.

**8. Zero writes on replay, and the replay's git journal (D21-43 N7).** In `Mode::Replay` the boundary performs no
commit. A tier-2 `git_op` row, carrying `GitInvocation`'s own members (`op`, `cwd`, `argv_digest`, `status`,
`stdout`, `stderr_digest`), is appended per live invocation, which is ADR-0105 clause 1's *"records every git
invocation and its result to the EventStore"* executed. **The stated rule for golden logs:** a recorded journal that
holds no `git_op` row is a run that performed no git write, so no committed golden log is re-recorded to go green.
The kind is minted by ADR-0116's route in the change that adds it.

**9. The heal, git → store (D21-42 8a's last sentence).** At every entry, after `reconcile_at_entry`, the kernel
appends the missing rows for every reconciled mark whose anchor is reachable from `rk-run/<item>` and absent from the
store. It writes nothing to git (AC-174). A mark whose anchor is unreachable heals nothing.

**10. The writer is reachable only from the boundary (AC-162's lint).** A source lint over code lines names
`append_tier1` in every call form (a method call, a path call, a bare path handed on as a function value) and drops
only definitions. Every remaining hit under `crates/*/src/` is in `crates/rk-kernel/src/runtime/tier_one.rs`, with one
allow-listed exception and its reason: the tier-1 arm of `crates/rk-ports/src/conformance.rs`, behind the
`conformance` feature and called only from the two conformance rows under `tests/`, which exercises the port and
applies no transition. **The rebuild is exempt, in terms**: it writes tier-1 rows whose commits already exist, so there
is no commit for the row to precede, and it never calls `append_tier1`; it reaches `rk_store::tier_one` through the
adapter's `rebuild`.

**11. `Snapshot` has contents, and `Capability::Rebuild` is implemented (D21-42 8c).** A `Snapshot` is the ordered
list of anchored rows a store holds: each event with its `anchor_sha`, its `t1_*` row where its kind has one, and its
interim `item`. `rk-ports` gives it a public constructor. `SqliteStore::snapshot` reads those rows and
`SqliteStore::rebuild` writes them, in one transaction, into a store opened with `open_at` on a staging file, through
`rk_store::tier_one`. Both SQLite registry rows flip together to a recorded probe. Until `s8e` §8e.9 lands that, the
SQLite row answers both from its dated `Probe::None` rows, as today.

*(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.9.)* **Landed, with three readings this decision left
free, stated here while the record is `proposed`:**
- **What a `Snapshot` row carries.** `rk_ports::event_store::SnapshotRow { event, anchor_sha, tree, opening_run }`:
  the event with its interim `item`, its anchor, and the two facts of its `t1_*` row a payload does not carry — the
  anchor commit's tree, which the store keeps only on a boundary's `t1_stage_transition` (`None` elsewhere), and a gate
  opening's `(stage, run_id)`. The `t1_*` row itself is a function of those and the event (decision 3's mapping), so it
  is not carried twice. `Snapshot::new` is the public constructor.
- **`rebuild` is append-only.** A store whose rows are a PREFIX of the snapshot's gains the rest, in order, in one
  transaction; any other store refuses and nothing is written. So a rebuild from a snapshot of the current state is
  identity (the conformance round trip), a fresh store — the staging file `SqliteStore::open_at` opens — gains every
  row, and no `UPDATE` or `DELETE` is issued (this record's forbids bullet; AC-364). `VecStore` follows the same rule.
- **The rebuild's rows are the heal's.** `doctor --rebuild` builds its `Snapshot` from each item's `rk-run/<item>` ref
  through `tier_one::marked_commits`, the derivation decision 9's heal uses, so a rebuilt row is the row the live act
  anchored to that commit, and AC-184's projection compares equal
  (`doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`). The Negative bullet below that says *"The rebuild
  fills them from the value"* is read accordingly: the rebuild fills a transition row's columns from the members the
  mark's transition carries — an `opened` row's tree, deadline, kind and reach; an answered row's option — and leaves
  null what the live row leaves null. A rebuilt `opened` row's `stage` and `run_id` are null (D21-48 X11).

*(dated 2026-09-28, `s8e` build, package D2196-8e; D21-96.)* **Over a void the claim above did not hold, and now does.** A rebuilt `answered` row carries its mark's tree, the
gate's bound tree; the live row carried the answer's observed tree, so `doctor::ac184_…` failed once its lane walked
through a void. Decision 3's dated bullet makes the live row carry the bound tree, through the one writer every path
calls, so a rebuilt `answered` row is again *"the row the live act anchored to that commit"*, byte for byte. The
ac184 lane now walks through a void (`lane_e2e::walk_the_lane_through_a_void`) and is green; a rebuilt store keeps no
observed tree, because the `Gate:` value carries none — the observed tree is a fact of the live store's ~~tier-2 rows~~
anchored `t2_event` rows *(dated 2026-09-28, s8e build, package R2FIX-8e: `gate_answer_void` is a tier-1 kind,
ADR-0184's *"`gate_answer_void` moves to tier 1"* and `EventKind::tier()`'s `Tier::One` arm; its row is stored in the
`t2_event` table, which is what "tier-2" misnamed)*, outside AC-184's `t1_*` projection.

**12. `BySha` selects by anchor.** `Query::BySha(sha)` returns the rows whose `anchor_sha` is `sha`, in append order,
in `rk-store`'s SQL and in `VecStore`. A row appended through `append` has no anchor and never matches. That is how
a conformance row reports its anchor through the port. `ByItem` and `ByRun` are ADR-0182's.

**13. Supersessions, and what is cited without superseding.**
- **`0158` decision 3** (*"No document, record or checkpoint may state that AC-162 is met, that a tier-1 row is
  written, or that a failed git write leaves zero store rows"*) and its bullet *"Claiming AC-160, AC-161, AC-162 or
  AC-163 is met, on any axis"* are **superseded in part**: only for the criteria `s8e` measures (§8e.14), which are all
  four (AC-160 on its `s6` trybuild fixture, re-run), and only once they are measured. Until the operator's dated
  ledger line flips this record, no document states that any of them is met.
- **`0155` decision 5(b)** (*"No document may state that AC-162 is met for this axis, or for any."*) and its bullet
  *"Stating or implying that AC-162 is satisfied for stage changes 2..N of a lane"* are **superseded in part** on the
  same condition.
- **Cited, not superseded:** ADR-0003 (the port count stays five); ADR-0105 clauses 1 (decision 8 executes it), 2, 4,
  8 and 9; **ADR-0105 clause 5**, because every git write this record adds is a `commit`, which its closed list names
  (*"`commit` (incl. `--allow-empty`, `--trailer`)"*), and the heal and the rebuild write nothing to git (D21-47's
  clause-5 rule); ADR-0158's `rk-cli` bullet, on X10's reading (decision 4); ADR-0109 decision 2; ADR-0118 clause 4
  (the writer runs through the mutex `s8e` §8e.1 made halt).

## Consequences

**Positive**
- AC-160 … AC-163 stop being vacuous: a tier-1 row exists only after its commit, and a failed commit leaves zero
  rows by construction.
- One method, one transaction and one module of statements: the tier-1 SQL has one home, and the reachability lint
  can name every caller.
- The gate's lifecycle becomes a fold over append-only rows, so no `UPDATE` exists against any tier-1 table.
- AC-180 gets the instrument its register row names, a checked-in version-N−1 store.

**Negative**
- The port grows after Checkpoint 7a passed, which re-opens 7a at Checkpoint 8e.
- Every existing store is refused once, and the operator rebuilds it. What the rebuild drops is tier 2 and listed,
  but it is dropped.
- A migration counted by `PRAGMA user_version` is a second number beside `rk_identity.schema_version`. They agree on
  every store this binary writes, and the identity check reads only the stamp.
- `t1_gate_transition`'s descriptive columns are nullable, because the answered, expired and void payloads do not
  carry every member the `Gate:` value does. The rebuild fills them from the value; a writer row does not invent them.
  *(dated 2026-09-28, `s8e` build, package D2196-8e; D21-96.)* An `answered` row's tree is no longer the payload's: it is read from the gate's `opened` row, so a
  store holding no `opened` row for the token refuses the answer's row (decision 3's dated bullet).

**What this forbids**
- A `t1_*` row written without its trailer commit, or before it.
- A call of `append_tier1` in any form outside `crates/rk-kernel/src/runtime/tier_one.rs`, other than the
  allow-listed conformance arm.
- SQL naming a `t1_` table outside `crates/rk-store/src/`, and a code line outside `rk-store` and
  `rk-adapter-sqlite` naming `rk_store::tier_one`.
- An `UPDATE` or a `DELETE` against any tier-1 table, and re-introducing a gate-state column that only an `UPDATE`
  could move.
- A tier-1 row that invents a member its event does not carry.
- Migrating a version-1 store in place, adopting it, or stamping it before the refusal.
- The rebuild calling `append_tier1`.
- Stating that AC-160, AC-161, AC-162 or AC-163 is met before this record is `accepted` on the operator's dated line.

## Evidence and confidence

**Measured** (2026-09-28, at `766854c`, `s8e` §8e.0): the shipped store's schema version and row counts; zero rows in
every `t1_*` table; the fixture store's sha256
`603d60041fc41e0788deb8cc80957c330dd3d8101b2cdadb887fbfd42f055dd3`, schema version 1, `t2_event` 3, `t1_*` 0.
**Documented:** the ledger lines quoted in the Context, and the records cited. **Not yet measured, and named:** the
kernel-owned order (decision 7), the heal (decision 9) and the reachability lint (decision 10) are built and tested by
`s8e` §8e.3's tests; the rebuild (decisions 6 and 11) by §8e.9's. This record is `proposed` and flips only on the
operator's dated ledger line. *(dated 2026-09-28, `s8e` build, package S8E-P7: decision 11 is now measured —
`store::open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows`,
`registry::snapshot_and_rebuild_rows_carry_a_recorded_probe`, `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`,
`doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value` and
`doctor::a_version_one_store_is_refused_by_name_and_rebuild_re_derives_it_at_version_two`, each red first or pinned by a
mutant that kills it (S8E-M4, M13, M15, M36, M39, M48).)* *(dated 2026-09-28, `s8e` build, package D2196-8e; D21-96.)* Decision 3's `answered`-tree bullet is measured: `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows`, its lane walked through a void (red at the package's entry for the reported reason), and `store::an_answered_row_carries_its_gates_bound_tree_and_is_refused_without_an_opening` (red against the pre-D21-96 writer); S8E-M58 (the observed tree restored) is killed by both, S8E-M59 (the payload's tree taken when no `opened` row exists) by the `rk-store` test.
