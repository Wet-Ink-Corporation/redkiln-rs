---
id: "0183"
title: "**The stage lives on the run ref: a stage-bearing `Story: <project>/<slug> <stage>` mark at every boundary, folded by `reconcile` to a latest stage per item, seeds `through_step_seven`, and the lane's end is a terminal `closed` mark** (`s8e`'s record; D21-41 a; D21-42 8d's two defaults; D21-43 N7 and N9): `classify` splits the `Story:` value on whitespace first; one token is the legacy stage-less mark, admitted read-only; two tokens are a stage-bearing mark; three or more are drift, never coerced; the stage token carries no whitespace and no `/`, and `closed` is reserved for the lane's end, so a pack declaring a stage of that name is refused at load; `reconcile` keeps membership and gains the latest stage per `(project, slug)`, oldest-first, where only an IDENTICAL `(project, slug, stage)` repeat is `duplicate-story-trailer` drift and a stage-less mark never sets or resets the stage; the planner reads that fold. Supersedes in part `0105` clause 3's `Story:` value delegation, `0155` decisions 1, 2 and 5(a) with two of its forbids bullets, `0157` decisions 4 and 5 with two of its forbids bullets, and `0161`'s lane-loop bullet once `s8e` §8e.11's tests pass"
status: accepted
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (row a — *"A stage-bearing `Story:` trailer on `rk-run/<item>` at every boundary, folded by `reconcile`, seeding `through_step_seven` so `run` and `answer` both see it."*), D21-42 (row 8d — *"Defaults applied: the one stage-less `Story:` mark is admitted read-only; ADR-0109 d3 is not superseded."*) and D21-43 (N7 — *"The planner reads `reconcile()`'s output (execution under ADR-0109 d1), and D21-41 (a)'s stage seed comes from its fold."*; N9 — *"A terminal `Story: <project>/<item> closed` mark carrying closeout's content checkpoint."*); runbook/phase-1/s8e-tier-one-and-the-lane.md (the records table's first row, §8e.0 item 1's reading at `766854c`, §8e.5, §8e.6, §8e.12 and §8e.13); crates/rk-git/src/trailer/grammar.rs, crates/rk-git/src/reconcile.rs, crates/rk-kernel/src/select/pack.rs, crates/rk-cli/src/verbs/run.rs and crates/rk-cli/src/doctor/rebuild.rs, as read and changed 2026-09-28 at `d80609b`; references/2026-09-08-architecture-design.md §5.4 (the `Story:` grammar row); references/2026-09-08-acceptance-criteria.md AC-161, AC-162, AC-165 and AC-183
supersedes: ["0105", "0155", "0157", "0161"]
superseded_by: []
related: ["0092", "0105", "0109", "0152", "0153", "0155", "0157", "0159", "0161", "0180", "0181", "0184"]
evidence: ["F12"]
consumers: []
---

## Context and problem

**A lane does not cross processes, and the reason is where the stage lives.** ADR-0157 decision 4:
*"Across processes, a second invocation reports `Done` and exits 0 having done nothing"*, with the route
*"named here as OWED"*. The composition root reads the stage from the item record (*"the stage is read
once, from the record, and is not defaulted"*), and `through_step_seven` discards the reconcile it
performs, so ADR-0109 decision 1's `Reconciled<State>` is built on every entry and thrown away. AC-518
forbids writing the advanced stage back to `.redkiln/items/<item>.toml`, so the record cannot move either.

**The `Story:` grammar has no stage slot, and that was a choice, not a law.** Design §5.4's row is
`<project> "/" <slug>` — *"one item id"*. ADR-0155 took one mark per item at the first boundary, because
a mark per boundary under that grammar is `duplicate-story-trailer` drift, and it named the widening as
its option 3, *"the honest long-term answer"*, declined *"on cost and scope, not on correctness"*. Its
decision 5(a) recorded the residual: *"For a lane crossing* N *boundaries,* **N−1** *stage changes carry
no trailer commit at all; route out is option 3 above."*

**The parser would have absorbed a stage silently.** `classify`'s `Story` arm called `split_qualified`
on the whole value, which splits on the first `/` and counts no tokens, so `redkiln-rs/x analysis`
parsed as the slug `x analysis` — ADR-0155's option 2, of which it said *"That it type-checks is what
makes it dangerous."*

**Measured 2026-09-28 at `d80609b`, across all refs, with git's own trailer parser**
(`git log --all --format='%(trailers:key=Story,valueonly=true)'`): exactly **one** `Story:` value exists,
`redkiln-rs/rk-fix-doctor-test-count`, on `rk-run/rk-fix-doctor-test-count` at `73c087a` (`s8e` §8e.0
item 1 recorded the same at `766854c`). It is stage-less. D21-42 18 keeps that item *"the untouched fixture
item"*, and Checkpoints 7h and 7i read it.

**The operator took the decision** (D21-41 a), its defaults (D21-42 8d), the planner's input (D21-43 N7)
and the lane's end (D21-43 N9). This record writes them down, fixes the grammar details the ledger left to
the record, and names what it supersedes.

## Decision drivers

- A lane's progress must be readable by the next process from git, because the store is disposable
  (ADR-0131 decision 6) and the record may not move (AC-518).
- A malformed value is drift, surfaced, never coerced (ADR-0109 decision 3; AC-165; AC-183).
- The one legacy mark on the tree must keep reading exactly as it does, so Checkpoints 7h and 7i stay
  runnable until `s8j` re-points them.
- A producer that can emit a value the parser reads as something else must refuse at the mint (the
  ADR-0161 decision 3 pattern: *"refused at the mint, naming the character and the reason"*).
- Moving a `## Decision` sentence or a forbids bullet of an accepted record is a supersession in part,
  never an erratum (ADR-0092 rule 1).

## Considered options

### Option — a stage-bearing `Story:` mark at every boundary, folded to a latest stage per item (taken)

D21-41 a names it. The grammar gains one whitespace-separated token; `reconcile` keeps membership and
gains a latest stage per `(project, slug)`; the seed reads the fold. Every boundary has its trailer, so
ADR-0155's residual (a) closes, and the stage is recoverable from git after the store is deleted.

### Option — keep one mark per item and read the stage from the store (ADR-0155's decision)

Rejected by D21-41 a. It leaves ADR-0157's cross-process stall in place: the store is disposable, so a
deleted database loses every stage change after the first, which is the deleted-database test's own
definition of tier 1.

### Option — a stage-qualified slug (`<project>/<slug>@<stage>`)

Rejected by ADR-0155's own reasoning as option 2: the slug would no longer be an item file stem, so the
rebuild lists it unresolvable, and it parses under the pre-widening grammar, which is exactly the silent
failure mode. A separate whitespace token is split before the slug is read.

### Option — a separate trailer key for the stage (`Stage:`)

Rejected: a fourth axis. ADR-0109 decision 3 fixes three, and `Story:` is the axis whose name says
*"this item entered"*; the stage is what it entered.

## Decision

**1. The `Story:` value is `<project> "/" <slug> [SP <stage>]`, split on whitespace first.**
- **One token** is the **legacy stage-less mark** — exactly the pre-widening reading: a qualified value
  is a story of that project with **no stage**, and an unqualified one is a bare id, as design §5.4 row 1
  already says. **It is admitted read-only** (D21-42 8d): it folds as membership with no stage, so the
  seed falls back to the item record, which is today's behaviour exactly. The producer never writes one
  again.
- **Two tokens** are a **stage-bearing mark**. The first token is `<project>/<slug>`, split on its first
  `/` as before; a two-token value whose first token has no `project/` form is drift (the grammar has no
  bare-id-with-stage form).
- **Three or more tokens are drift**, surfaced under a new `MalformedReason::MalformedStoryValue` as an
  `unparsable_trailer` drift on the `Story` key, and never coerced (AC-183).
- **The stage token contains no whitespace and no `/`.** A value whose stage token carries `/` is drift.
  The producer renders through `rk_git`'s one renderer, `ParsedTrailer::to_trailer`, which parses its own
  output back and returns nothing when the two differ, so a stage the grammar cannot carry is refused at
  the mint with its reason, not written.

**2. `closed` is reserved for the lane's end (D21-43 N9).** `Story: <project>/<item> closed` is the
terminal mark, parsed as the lane's end and never as a stage. **A pack that declares a stage named
`closed` is refused at load** — in `[policy] stages`, in `[policy] stage_order`, or as a `[stage.closed]`
table — so the token can never name a stage. The renderer never emits a stage *named* `closed`, because
it would read back as the lane's end. The kernel mints the mark when `Pack::successor` ends the lane,
through `s8e` §8e.3's boundary function; `s8f`'s landed arm makes it carry closeout's content checkpoint.
A `run` on an item whose latest mark is `closed` reports that it is closed, exits 0 and mints nothing:
that exit 0 is **Done, read from git**, and it is not ADR-0157's stall. The removal of the run tree is
keyed to this mark (`s8d-ii`'s rule, ADR-0180; `s8e` §8e.6 wires the trigger).

**3. The fold.** `reconcile` keeps `stories_committed` (membership) and gains, per project,
`stages: slug → (stage, commit)`:
- **Latest wins, commits folded oldest-first** — the rule `Slice-Verdict:` already uses (*"Latest wins:
  commits are folded oldest-first."*).
- **A repeated IDENTICAL `(project, slug, stage)` is `DuplicateStoryTrailer` drift**, counted and not a
  transition, so it does not move the latest stage. Two marks for different stages of one item are not
  duplicates — that is the widening. Two identical stage-less marks remain the duplicate they always were.
- **A stage-less mark never sets or resets a stage** that a later-or-earlier stage-bearing mark set.
- A `closed` mark is the item's latest state when it is the latest mark.

**4. The order assumption, stated.** The fold rests on `read_commits`' order, which is
`log --all --reverse` **without `--topo-order`**. Git's default walk emits a commit only after every child
that reached it, so over **one linear run ref** a child always precedes its parent whatever the commit
dates, and `--reverse` puts the parent first. That is the only shape the lane writes: every `Story:` mark
for an item is a commit on `rk-run/<item>`. Across refs the default walk interleaves by commit date, so
two refs carrying marks for one item would be ordered by clock. Nothing writes that shape. This record
names the limit rather than adding `--topo-order`, which would also change the order of the F12 corpus
read that AC-444 is pinned against; a record that ever lets two refs carry one item's marks owns the
change.

**5. The seed (D21-43 N7; execution under ADR-0109 decision 1).** `through_step_seven` keeps the
`Outcome` its reconcile returns instead of discarding it. `resolved.stage` becomes the item's reconciled
latest stage and falls back to the item record only when git carries no stage-bearing mark for the item.
`answer` resolves through `run::through_step_seven`, so both verbs see it. This is the first time
`Reconciled<State>` reaches the planner rather than being thrown away.

**6. The mint (D21-41 a).** A stage-bearing mark is minted at the lane's entry and at every advance,
through the kernel's boundary function (ADR-0181 decision 7). The idempotence read changes from *"item
already committed"* to *"this `(item, stage)` already committed"*; ADR-0161 decision 1's
`stages_entered` guard still forbids re-entry within a lane. Until `s8e` §8e.3 lands that function, the
existing producer (`rk-cli`'s `story_trailer`) writes the widened value and nothing else changes in it.

**7. `doctor --rebuild` takes `to` from the marks.** A stage-bearing mark gives its own stage, anchored on
its own commit. A legacy mark falls back to `.redkiln/items/<slug>.toml`'s `stage` — the rule ADR-0155's
Context quotes from `rebuild.rs` — and stays listed unresolvable when the record has none (AC-165). A
`closed` mark rebuilds as the lane's end (`s8e` §8e.6).

**8. Supersessions (ADR-0092 rule 1), each in part.**
- **`0105` clause 3**, only its delegation of the `Story:` value to design §5.4's `<project> "/" <slug>`
  row (*"values follow the trailer grammar table in design §5.4"*): the row is amended by this record, with
  a dated line in design §5.4. Every other sentence of clause 3 stands (the plumbing, `ifexists=add`, the
  three keys).
- **`0155` decisions 1 and 2** (*"A lane mints **exactly one** `Story:` trailer per item, at the
  **first** stage boundary it crosses."*; *"The value is `Story: <project>/<item-id>`"*) and **decision
  5(a)** (the N−1 residual), and its forbids bullets *"A second `Story:` trailer for one `(project, slug)`,
  in one lane or across lanes."* and *"A `Story:` value carrying a stage, a boundary, or any third token,
  until the grammar is widened by a record that supersedes `0105` clause 3."* — this is that record.
  Decision 3 (column 0, `": "`, case) and the other bullets stand; decision 5(b) is ADR-0181's.
- **`0157` decisions 4 and 5** (the cross-process residual, and *"No document may describe a lane as
  resumable across processes, or read run two's exit 0 as progress."*), and its bullets *"Describing a lane
  as resumable across processes, or a lane's completion as anything other than within-one-process."* and
  *"Reading a second invocation's exit 0 as progress, or as evidence that the lane finished."* A second
  invocation does progress once the seed reads the fold, and an exit 0 on a `closed` item is Done read
  from git. Decisions 1 to 3 (the guard, as `0161` narrowed it) stand.
- **`0161`'s bullet** *"Describing the lane loop as covered by tests, or reading a green suite as evidence
  about it."* — **only once `s8e` §8e.11's `lane_e2e` tests pass**; until then it binds unchanged. Its
  decisions stand.

**9. Cited, not superseded.** ADR-0109 decisions 1 (N7 executes it), **3 — NOT superseded** (D21-42 8d's
default: resume reads all three axes once `Slice-Verdict:` lands, so resuming from `Story:` alone is
still forbidden) and 4; ADR-0161 decision 1 (`stages_entered` keeps replays idempotent); ADR-0159 (the
refusal of a missing project, whose pattern this record's refusals follow); ADR-0180 (the removal rule
the `closed` mark triggers); ADR-0181 (the boundary function that mints).

**10. What the build fixed where `s8e` §8e.5 and §8e.6 left it to the record** *(dated 2026-09-28, `s8e`
build, package S8E-P6; this record stays `proposed`)*. Each is named for the adversarial pass.
- **The seed's carrier.** `through_step_seven` keeps the `Outcome` (decision 5) and reads the item's mark
  from `State.projects[<project>].stages[<item>]` (`rk_cli::verbs::run::seeded_stage`). With no `[project]
  name` there is nothing to look the mark up under, so the record's `stage` is the fallback, as it is for the
  legacy stage-less mark. The record's `stage` is required (RT-50) only when git seeds none.
- **The `closed` act's row.** The terminal mark anchors **one tier-1 row**: `stage_transition { from: <the
  stage the lane ended in>, to: "closed" }` (`tier_one::close_act`). No kind is minted for it. `closed` is
  reserved by decision 2, so the `to` value cannot name a stage, and a deleted store gets the row back from
  the mark alone. The heal re-derives it from the previous stage mark on the ref, and `t1_stage_transition`
  holds `to_stage = 'closed'`.
- **When the lane ends.** The kernel reports the end (`RecordedRun::lane_ended`: the resume loop stopped on
  `Done` with no successor supplied) and does not act on it. The composition root mints the mark only after
  that stage's review pass has not blocked. A blocking verdict is not an end, and the mark comes after the
  pass that could have blocked. A pack that declares no `stage_order` has a one-stage lane, and its end is
  also marked.
- **Where the idempotence read sits.** Decision 6's *"this `(item, stage)` already committed"* is read from
  git inside the boundary function (`tier_one::stage_already_marked`, live arm only, every call a read). A
  `Story:` mark already on `rk-run/<item>` commits nothing and writes no row (`Committed::AlreadyMarked`).
  If its row is missing, restoring it is the heal's job. `replay::resume_routed` also takes no entry for a
  stage its journal has already entered (a `stage_transition` into it). The heal restores that row from the
  mark, so a deleted store rebuilt at entry folds no second, trailer-less entry.
- **The removal trigger.** The trigger is `rk_git::worktree::remove_landed`, called by `run` after the
  run's lock is released. `remove_landed` removes only a tree that is unlocked, so the call must come after
  the release. A kept tree is reported on stderr with its named reason. A refused removal is reported by
  name and is not retried or forced. The lane's exit stands, because the `closed` mark is already git's
  truth and the tree is kept for the operator.
- **A closed item's `run`** returns before the run tree is entered, so a removed tree is not re-created. It
  still writes the invocation's one `reconcile_started` row (AC-172) and nothing else.

## Consequences

**Positive**
- A lane crosses processes: the next invocation seeds from git, and a deleted store loses no stage.
- ADR-0155's residual (a) closes: every boundary has its trailer, which AC-162 needs for stage changes 2..N.
- The one legacy mark reads exactly as before, so Checkpoints 7h and 7i are unaffected until `s8j`.
- The widened parser reports a stage-qualified value it cannot read as drift instead of absorbing it.

**Negative**
- The grammar changed after F12's corpus was pinned. The corpus's `Story:` values are read by the same
  one-token rule, so no count moves unless the corpus carries multi-token values; AC-444's dated note
  records that the counts are re-read, not re-pinned.
- The order assumption (decision 4) is a stated limit, not a check.
- ~~Until `s8e` §8e.3 lands, the producer still mints once per item, now with the stage of its first
  boundary; the seed and the per-boundary mint are that section's.~~ *(dated 2026-09-28, `s8e` build,
  package S8E-P6: both landed; see decision 10.)*
- *(dated 2026-09-28, `s8e` build, package S8E-P6.)* Every `Story:` mint makes one extra read of git (the
  idempotence read, `log --all` over the run ref's reach). Its cost grows with the history.

**What this forbids**
- A `Story:` value with three or more tokens, or a stage token carrying whitespace or `/`, written by any
  producer.
- A pack stage named `closed`, in any declaring position.
- Minting a new stage-less `Story:` mark.
- Letting a stage-less mark set, reset or clear an item's reconciled stage.
- Counting two marks for different stages of one item as a duplicate, or two identical marks as a
  transition.
- Seeding a run's stage from the item record when git carries a stage-bearing mark for the item.
- Writing an item's advanced stage back to `.redkiln/items/` (AC-518, unchanged).
- Reading an exit 0 on an item whose latest mark is not `closed` as evidence that the lane finished.

## Evidence and confidence

**Measured** (2026-09-28 at `d80609b`): one `Story:` value across all refs, stage-less, at `73c087a`;
zero `Gate:` and zero `Slice-Verdict:` values. **Built and tested in `s8e` (package S8E-P4), red first:**
the seven `trailers::` `Story:` rows of `s8e` §8e.12 in `rk-git` (the stage-bearing parse, the legacy mark,
three tokens as drift, the latest-stage fold, the identical-duplicate rule, the legacy mark that does not
reset, the `closed` mark) and `lane_order::a_pack_stage_named_closed_is_refused_at_load` in `rk-kernel`;
mutants S8E-M16 (the fold keeps the first mark) and S8E-M17 (a legacy mark resets the stage) each
redden their row. **Not yet built, and named:** the seed (decision 5), the per-boundary mint (decision 6)
and the `closed` mint are `s8e` §8e.3, §8e.5 and §8e.6's, measured by §8e.11's `lane_e2e` tests. This
record is `proposed` and flips only on the operator's dated ledger line.

*(dated 2026-09-28, `s8e` build, package S8E-P6.)* **Now built:** the seed (decision 5), the per-`(item,
stage)` mint (decision 6), the `closed` mint and its row, and the removal trigger (decision 10). They are
measured green, after P2's recorded red, by `lane_e2e::run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane`,
`…the_stage_seeds_from_git_when_the_store_is_deleted`, `…every_non_forbidden_stage_is_entered_and_trailered_in_declared_order`,
`…the_lane_end_mints_a_terminal_closed_mark`, `…a_closed_item_reruns_as_done_and_mints_nothing`,
`…a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one` and the other §8e.11 tests of the
package, plus `tier_one::a_closed_mark_heals_as_the_lanes_end_transition`. Mutants S8E-M1 (the seed from
the record), M2 (the read *"item already committed"*), M28 (no `closed` mark), M37 (no trigger) and M38
(`--force`) each redden their named test.
