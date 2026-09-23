---
id: "0155"
title: "**One `Story:` trailer per item, minted at the FIRST stage boundary a lane crosses** — not one per boundary — because four requirements cannot all hold and this is the only combination that breaks no green instrument. design §5.4 gives the value `<project> \"/\" <slug>`, **one item id with no stage slot**; `reconcile` makes a second identical value `duplicate-story-trailer` DRIFT, *\"counted, not a transition\"*; `doctor --rebuild` already writes ONE `stage_transition` per item on purpose, and its own comment says why — *\"`to` is read from the item record and the record holds ONE stage: a second row would claim a transition into a stage no source names\"*; and AC-162 wants a trailer commit before the store row of EVERY tier-1 transition. The fourth is the one that gives: stage changes 2..N get their store row with no trailer beside them, and that residual is **named and bounded here** rather than closed by widening an accepted grammar. The `<slug>` MUST be the `.redkiln/items/` file stem, or rebuild reports `unresolvable`"
status: proposed
date: 2026-09-22
legacy_id: none
source: references/2026-09-08-architecture-design.md §5.4 (line 693, the `Story:` grammar row; line 688, F12's two-axis warrant); crates/rk-git/src/trailer/grammar.rs (`parse_body` at 143-162, `classify`'s `Story` arm at 166-171, `split_qualified` at 224-228 — first slash, no token count); crates/rk-git/src/reconcile.rs (116-124, `stories_committed.insert` and `Drift::DuplicateStoryTrailer`); crates/rk-cli/src/doctor/rebuild.rs (165-202, one row per item, anchored on the LAST mark, `to` from `stages.get(slug)`; 205-224, `Slice-Verdict:` per HISTORY entry); references/2026-09-08-acceptance-criteria.md AC-162 (3012-3018); references/adr/0105 (decision 3, the trailer plumbing; clause 8, ownership); references/adr/0109 (decision 3, three axes; AC-183, drift is never coerced)
supersedes: []
superseded_by: []
related: ["0105", "0109", "0152", "0156", "0158"]
evidence: ["F12"]
consumers: []
---

## Context and problem

`s7h` §7h.2 says where the first axis is written: *"**`Story:` at the stage boundary** — value per design
§5.4's grammar table, column-anchored, through the wrapper."* Taken literally — one trailer per boundary
— it conflicts with three things that are already true and green.

**1. The value has no stage slot.** design §5.4's row (`architecture-design.md:693`) is:

> | `Story:` | `<project> "/" <slug>` — one item id | Column 0 of a trailer line … |

and `classify`'s `Story` arm (`grammar.rs:166-171`) splits on the **first** `/` via `split_qualified`,
which applies **no token-count check**. So the value identifies an item. It cannot say which boundary was
crossed, and there is no third field in which to put one.

**2. A repeat is drift, not a transition.** `reconcile` (`reconcile.rs:116-124`):

```rust
ParsedTrailer::Story { project, slug } => {
    let p = project_entry(&mut state, project);
    if !p.stories_committed.insert(slug.clone()) {
        drift.push(Drift::DuplicateStoryTrailer { project, slug, commit: c.sha.clone() });
```

A lane crossing *N* boundaries for one item would mint *N* identical values and produce **N−1** drift
rows. design §5.4 states the same rule in prose, and `0109` decision 3 with AC-183 forbids coercing drift
into a transition.

**3. The rebuild path already decided the cardinality, and wrote down why.** This is the load-bearing
evidence, because it is not this stage's opinion. `rebuild.rs:165-169`:

> `Story:` → ONE `stage_transition` per item, anchored on that item's LAST mark.
>
> One per item and not one per mark, because `to` is read from the item record and the record holds ONE
> stage: a second row would claim a transition into a stage no source names.

And the code matches: `last_mark` keeps only the latest sha per `(project, slug)`, then `to` comes from
`stages.get(slug)` — the `.redkiln/items/<slug>.toml` record — not from the trailer (`:181-196`). A slug
that is not an item file stem yields `unresolvable` naming that file.

Against those three stands **AC-162** (`acceptance-criteria.md:3014-3016`):

> GIVEN a tier-1 transition (**stage change**, gate opened/answered, verdict, slice seal) WHEN the
> transition is applied THEN the trailer commit exists in git **before any store row for that transition
> is readable**

Four requirements, and **any three are satisfiable but not all four**. Note the asymmetry this exposes:
the *other* axis does not have the problem. `rebuild.rs:205-224` writes `slice_sealed` **per history
entry**, and F12 is the reason (`design:688` — 20.3% of 74 slices passed through a rejected state). So
`Slice-Verdict:` is many-per-slice by design and `Story:` is one-per-item by design. The two axes have
different cardinalities on purpose, and §7h.2's phrasing flattens them.

## Decision drivers

- Three of the four requirements are enforced by running code with tests behind it; the fourth is a
  criterion whose per-transition reading nothing has ever satisfied for this axis (there are **0** real
  `Story:` trailers anywhere in the repository). Breaking a green instrument to satisfy a criterion that
  was never met is a net loss of evidence.
- `rebuild.rs`'s comment is a prior, reasoned decision about exactly this cardinality. Contradicting it
  from the producer side would make the run path and the rebuild path disagree, which is precisely what
  AC-427's git↔store differential exists to catch — the instrument would go red, correctly.
- An accepted record is never edited (`0092` decision 1). Widening the grammar means superseding `0105`
  clause 3 and moving AC-444's pinned drift counts: a real cost, and one this stage would be paying while
  it still cannot reach a review verdict at all (`0156`).
- `00-preamble.md`'s rule that a decision is not taken by implementing it cuts the other way too: a
  residual that is named, measured and attached to a record is a better artifact than a silent widening.
- The cheapest route — a stage-qualified slug — *parses*, precisely because `Story:` has no token count.
  That it type-checks is what makes it dangerous.

## Considered options

**1. One `Story:` per boundary.** Matches §7h.2's sentence and AC-162's per-transition reading. Produces
N−1 `duplicate-story-trailer` drift rows per lane, makes `doctor --rebuild` disagree with the store, and
turns AC-427 red. Rejected: it satisfies the criterion by breaking the check that would notice.

**2. A stage-qualified slug** — `Story: rk/rk-fix-doctor-test-count triage->analysis`. Parses today
(`split_qualified` takes the first slash; nothing counts tokens), so no duplicate drift and a trailer per
boundary. But `stages.get(slug)` then looks for `.redkiln/items/rk-fix-doctor-test-count triage->analysis.toml`,
which does not exist, so every such mark becomes `unresolvable` and the differential breaks anyway —
with a worse failure mode, because the trailers look well-formed. Rejected.

**3. Widen the grammar to a three-token value.** Amend design §5.4, supersede `0105` clause 3 in part,
re-pin AC-444's drift counts, and teach `reconcile` and `rebuild` the stage component. This is the route
that actually satisfies AC-162 per transition, and it is the honest long-term answer. Rejected **for this
stage**, not on merit: it is a grammar change spanning two accepted records and two pinned count sets,
taken while the axis has no producer and the review half of the lane is unreachable. It is named here so
the next stage inherits it as a route, not as a discovery.

**4. One `Story:` per item per lane, residual named (chosen).** Every green instrument stays green, the
producer agrees with the rebuild path by construction, and the gap is recorded with its exact shape.

## Decision

**1.** A lane mints **exactly one** `Story:` trailer per item, at the **first** stage boundary it crosses.
Subsequent boundaries in the same lane append their `stage_transition` row to the store and mint no
further trailer.

**2.** The value is `Story: <project>/<item-id>` where `<item-id>` **is** the `.redkiln/items/` file stem.
A value whose slug is not an item stem is forbidden, because `rebuild.rs:186-193` reports it
`unresolvable` by name.

**3.** The emission is column 0, separator `": "` **with the space**, key case-sensitive — `parse_body`
(`grammar.rs:143-162`) skips any line failing any of the three, and a skipped line is a trailer that
silently does not exist.

**4.** The trailer commit is written as close to the boundary as the seams allow, and **not** from
`terminal_read`: that function runs after `resume` has already appended every minted row
(`replay.rs:1744-1746`), which is the inverse of *"git first"*.

⚠️ **AC-162's ordering cannot be satisfied by this record, and the reason is structural rather than
this axis's.** *"Any store row for that transition"* is a named table — `t1_stage_transition`, whose own
schema header says *"these rows never exist without their trailer commit"* — and **ADR-0158 measures that
the whole tier-1 layer has no writer on any axis**: zero `INSERT INTO t1_*` in the workspace, all three
tables empty in a live store, and no method on the `EventStore` port that could write one. So there is no
tier-1 row to order the commit against, on this axis or on `Gate:`. The transition lands in `t2_event`,
the tier-**2** table whose loss `doctor` classifies as telemetry-only.

**5. The residual is recorded, not closed, and it is TWO residuals rather than one.**
(a) For a lane crossing *N* boundaries, **N−1** stage changes carry no trailer commit at all; route out is
option 3 above. (b) AC-162's ordering is **vacuous** — ADR-0158's finding, not a consequence of the
cardinality chosen here. No document may state that AC-162 is met for this axis, or for any.

**6.** `s7h` §7h.2's *"`Story:` at the stage boundary"* is corrected by errata to say *the first* stage
boundary, and to record that the two axes have different cardinalities.

## Consequences

The producer and `doctor --rebuild` agree by construction, so AC-427's differential keeps its meaning and
`reconcile` reports zero drift for a clean lane. The `Story:` axis becomes what `stories_committed`'s name
always said it was: a set of items that have been committed, not a log of boundaries.

The honest cost is that `Story:` is a weaker record than AC-162 asks for, and a reader who wants "which
stage did this item enter, in git" still cannot get it from the trailer — only "this item entered the
lane", plus the store's rows. That is written down in clause 5 rather than left for a later stage to
rediscover.

**What this forbids**

- A second `Story:` trailer for one `(project, slug)`, in one lane or across lanes.
- A `Story:` value whose slug is not a `.redkiln/items/` file stem.
- A `Story:` value carrying a stage, a boundary, or any third token, until the grammar is widened by a
  record that supersedes `0105` clause 3.
- Writing the `Story:` trailer from `terminal_read`, or from anywhere after the store row is readable.
- Coercing a `duplicate-story-trailer` drift into a transition (`0109` decision 3, AC-183).
- Stating or implying that AC-162 is satisfied for stage changes 2..N of a lane.
- Reading this record as a statement about `Slice-Verdict:`, which is many-per-slice by design.

## Evidence and confidence

**Every one of the four requirements is quoted above from its own source**, and three of the four are
running code rather than prose: `grammar.rs:166-171` + `:224-228`, `reconcile.rs:116-124`,
`rebuild.rs:165-202`. The fourth is AC-162's THEN, verbatim.

**Measured 2026-09-22 at `fc45eeb`, across ALL refs**, with git's own trailer parser
(`git log --all --format='%(trailers:key=<K>,valueonly=true)'`):

| key | real trailer values | lines matching `^<K>:` |
| --- | --- | --- |
| `Story` | **0** | 0 |
| `Slice-Verdict` | **0** | **1** |
| `Gate` | **0** | 0 |

So this record gives an axis its first producer; it breaks no existing emission, because there is none.
The single `^Slice-Verdict:` line is **prose** inside commit `ba46ed9`'s body — git does not report it as
a trailer — and it is the false positive `0156` hardens Checkpoint 7h against.

**The cardinality is not this stage's invention.** `rebuild.rs:165-169`'s comment predates Step 7h and
states the one-per-item rule with its reason. The chosen option is the one that agrees with it; the three
rejected options each disagree with it in a different way.

**Confidence, and what is NOT asserted.** This record does not assert that one `Story:` per item is the
right long-run grammar — option 3 is named as the route that would satisfy AC-162 properly, and this
record declines it on cost and scope, not on correctness. It asserts nothing about `Gate:`'s existing
store-then-git ordering, which is a separate defect on a separate axis and belongs to the stage that
built it. And it asserts nothing about F12's headline figures beyond the two-axis warrant they support:
`rk-git`'s parser is deliberately stricter than the spike's, and `corpus_f12.rs` already records that its
counts are not required to equal the spike's.
