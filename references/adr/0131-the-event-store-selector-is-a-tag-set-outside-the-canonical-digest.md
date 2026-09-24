---
id: "0131"
title: The event store's selector is a tag set carried on the event, not three string-keyed variants; tags and the store-minted instant sit **outside** `canonical_events`, so no golden log moves and no replay diverges; `rk-adapter-happenstance` is the intended tier-2 adapter and git remains the tier-1 authority; and the wiring lands after Checkpoint 8, gated on S-HS
status: accepted
date: 2026-09-21
legacy_id: none
source: crates/rk-ports/src/event_store.rs at `1308311` — the module header in terms (*"Everything below the names is a **ROUTING, not a decision** (runbook `s7a` §7a.3), reversible by the record that is owed"*; *"design:265's 'query by item / SHA / run' fixes neither the arity nor the key types, so the trio is **one method with a selector**, [`Query`], an `rk-ports` enum of three variants. What each variant's key type should be is fixed by no record and **Checkpoint 7a asserts none of it**"*; *"the per-kind payload field sets are **NOT** typed here and that is reported as **ARMED, NOT DISCHARGED**"*), the `Query` enum :29-35 and the `Snapshot` opaque newtype :41; runbook/phase-1/s7a-ports-and-fakes.md §7a.3 (the routing this record discharges); references/2026-09-08-architecture-design.md §3.3 :265 (*"append; query by item / SHA / run; snapshot; rebuild"*), §5.4 (the tier-1 test), §5.6 (*"`reconcile()` at every entry, git → store, never the reverse"*), §8.6 (projections are derived, at most one write); crates/rk-types/src/event.rs :61 (`Event { kind, payload: serde_json::Value }`) and its OWED typed-payload note; crates/rk-adapter-sqlite/src/lib.rs :322 (`SELECT kind, payload FROM t2_event ORDER BY id` — the selector is ignored); crates/rk-cli/src/verbs/status.rs, the **RAISED** block (*"the store cannot tell and inventing an owner would be worse than over-reporting"*; the three tier-1 kinds that carry no item); crates/rk-kernel/src/replay.rs at `1308311` (`canonical_events` / `event_json` :1073-1078 — `{kind, payload}` and nothing else; the `golden-logs/` layout :88-120; `run_id` over **inputs** :968-977) and runbook/phase-1/s7b-kernel.md §7b.11 (the layout and the single `CORPUS_PIN` move); spikes/DECISIONS-2026-09-18.md **D18-14** as the byte-stability precedent (*"no kernel event payload changes"*); runbook/phase-1/s7d-cli-and-migrate.md §7d.1 and §7d.15 (two crates written, **one** closed crate edited, the disturbance occasions enumerated and *"Slice 4 is indeed the last"*); references/2026-09-20-operator-surface-proposal.md §§3.2, 12.2-12.4, 13 (the three gaps, the fit, the landing point) and the operator's ruling of 2026-09-21 that an interactive read model is a requirement of the product rather than an option
supersedes: []
superseded_by: ["0163"]
related: ["0003", "0099", "0100", "0105", "0109", "0115", "0116"]
evidence: ["F19", "F22"]
consumers: []
---

## Context and problem

**The event store port was routed, not decided, and the record that fixes it has been owed since Step 7a.**
`rk-ports/src/event_store.rs` says so in its own header rather than leaving it to be discovered: design:265's
*"query by item / SHA / run"* *"fixes neither the arity nor the key types"*, so the three keys became
`Query::{ByItem, BySha, ByRun}` over `String`, and *"what each variant's key type should be is fixed by no record
and **Checkpoint 7a asserts none of it**."* The same header reports the payload half as **ARMED, NOT DISCHARGED**.
Two halves of one debt: how an event is *addressed*, and what an event *contains*. This record discharges the
first and leaves the second exactly where it is.

**The debt is no longer abstract, because a consumer arrived.** Measured on the tree 2026-09-21: `SqliteStore::query`
is `SELECT kind, payload FROM t2_event ORDER BY id` **with no `WHERE`** — every selector returns the whole table —
and `status`'s own module header states the consequence rather than hiding it:

> The store's `query` **ignores its selector** … the event carries no item id, commit SHA or run id to filter on
> until that same record lands. … Three tier-1 kinds carry no item at all — `stage_transition` is `{from, to}`,
> `gate_opened` is `{deadline, kind, reach, token, tree_hash}`, and `dag_assembled` carries neither `run_id` nor
> `item` — and **no record says which item they belong to.** This module hands them to every item, because the
> store cannot tell and inventing an owner would be worse than over-reporting … **RAISED, not settled.**

So `redkiln status` over a multi-item store over-reports its gates and its transitions, by construction, and the
module is honest about it. Every roll-up above the single item inherits that.

**And the product now requires the thing that gap blocks.** The operator's ruling of 2026-09-21: because redkiln
orchestrates agent sessions and workflows, a CLI with no interactive surface is not a usable development
experience, so **a read model is a requirement of the product** rather than a Phase-3 nicety, whatever renders it.
A read model is a per-item, per-time partition of the journal. The journal has neither key. That is the whole of
the problem this record addresses.

**A candidate substrate exists and has been measured.** `happenstance` (Wet Ink Corporation), a DCB-conformant
event-sourcing library for Rust, published at `0.3.2` on 2026-09-21, MIT OR Apache-2.0, MSRV 1.97.1, with a
116-rule conformance suite its SQLite adapter clears against a real file. Its `Tags` / `Query` / `QueryItem`
generalise the three selectors; its `SequencedEvent.recorded_at` is **store-minted** and deliberately not an
ordering key; and its `ProjectionStore` + `run_projection(..., chunk) -> Progressed` give a checkpointed read
model that is pumped by a call and needs no surviving process. The survey is
`references/2026-09-20-operator-surface-proposal.md` §12; this record takes only what that survey decided.

**The one thing that must not happen by accident.** `RecordedRun::canonical_events` hashes
`{ "kind": …, "payload": … }` and nothing else. A golden run's id is `sha256_lp(["rk-kernel/golden-log/v2", stage,
canonical(inputs)])` — keyed on **inputs** — and the logs live in the corpus behind `CORPUS_PIN`. Put the
correlation key **inside the payload** and every golden log's canonical events change: the corpus regenerates,
`CORPUS_PIN` moves, and Checkpoints 7b, 7b-ii and Step 8's AC-161 all re-derive. Put it **beside** the payload and
nothing moves. That choice is cheap today, expensive after the corpus has regenerated once, and it is the reason
this record is written now rather than with the code.

## Decision drivers

- **An owed record is a liability with a decay rate.** `s7a` §7a.3's *"routing, not a decision … reversible by the
  record that is owed"* is true precisely because nothing has shipped against it. After Phase 1 exits there is a
  self-hosting run behind the present shape and the same change becomes a contract modification. Writing the
  destination down is what preserves the debt's status; an owed record that is never written becomes the status quo.
- **D18-14's byte-stability property is the house precedent and must be extended, not re-argued.** It landed
  `Config` as an `Option` *omitted from `to_json()` when `None`* so that *"every recorded `run_id` /
  `inputs_digest` on the tree is byte-stable and the 7b/7b-ii re-run is the proof"*, and states the rule in four
  words: **"no kernel event payload changes."**
- **A tag is addressing metadata, not decision content.** Two runs that decided identically must produce identical
  canonical events even when one was tagged into a different initiative. This is the substantive ground for
  decision 2; the corpus argument above is the cheap one, and it agrees.
- **The kernel reads no clock (design §3.4, §9.3).** Any instant must therefore be minted at the edge. A store that
  stamps its own append is the only placement that does not touch the determinism ban list.
- **Step 7d's disturbance budget is spent and the stage says so.** §7d.1 permits two crates written and **one**
  closed crate edited, `rk-kernel`; the disturbance occasions are Slices 2, 3 and 4 and *"Slice 4 is indeed the
  last."* `rk-ports` is not on that permission at any slice.
- **Waiting accretes nothing.** Measured 2026-09-21: 14 selector sites across seven files; **Step 7e writes zero
  Rust** (`grep -cE 'crates/[a-z-]+/src'` → 0) and **Step 8 writes one read-only `grep`**. The count is final for
  Phase 1, so the cost of deferring the wiring is not a growing cost.
- **Tier-1 authority is paid for by measurement and is not in play.** F19 reconstructed 174/174 from git losslessly;
  F22 forbids depending on a surviving process. ADR-0109 decision 2, ADR-0105 clause 8, design §5.6 and AC-184 rest
  on those, and nothing measured says an inversion is better.

## Decision

**1. The event store's selector is a tag set, and the three variants become three reserved keys.**
`rk-ports::Query` is widened to select by a conjunction of `key:value` tags over a disjunction of event kinds.
`ByItem`, `BySha` and `ByRun` are re-expressed as the reserved keys **`item`**, **`sha`** and **`run`** and carry
exactly the meaning they carry today. This discharges the selector half of `s7a` §7a.3's owed record. The keys
this record reserves are `item`, `run`, `sha`, `stage`, `lane`, `node`, `gate`, `ac` and `worktree`; a tenth key is
a decision, taken in a record, not in a call site.

**2. Tags are carried on the event and are OUTSIDE `canonical_events`. `canonical_events` stays `{kind, payload}`,
byte for byte.** This is the load-bearing clause and it is stated twice on purpose. A tag is addressing metadata:
it says which conversations an event belongs to, not what was decided. The consequences follow mechanically —
**no golden log's canonical events change, `CORPUS_PIN` does not move, `run_id` is untouched** (it is over inputs),
**and no replay diverges because a later run addressed an event differently.** D18-14 is the precedent for the
shape and `Config`'s conditional insertion is the precedent for the care.

**3. The instant is minted by the store at append, is carried beside the event, and is not an ordering key.**
Order is the store's own sequence. The instant exists for duration, staleness and display, and for nothing that a
decision depends on. It is likewise **outside `canonical_events`**, for decision 2's reason and for one more: an
instant inside the digest would make the same decision hash differently on two runs, which is the property
`run_id` exists to detect.

**4. The three item-less tier-1 kinds gain an `item` tag.** `stage_transition`, `gate_opened` and `dag_assembled`
are addressed like every other kind. This is what closes `status`'s **RAISED** over-reporting, and it is a redkiln
decision that no store can make: a store cannot supply an owner it was never told.

**5. `rk-adapter-happenstance` is the intended adapter, at tier 2, and git remains the tier-1 authority.**
The library supplies the substrate — tags, the store-minted instant, a checkpointed `ProjectionStore` and a
`run_projection` that is pumped by a call rather than by a daemon (F22, design §8.6). **DCB's append condition is
explicitly declined at tier 2**, and the reason is stated rather than left as an omission: ADR-0105 clause 7 puts
one store per repository behind one writer, so there is no contention for an append condition to arbitrate.
Adopting the substrate and declining the mechanism is the decision, not a half-measure.

**6. ADR-0109 decision 2, ADR-0105 clause 8, design §5.6 and AC-184 are NOT re-opened, and this record says so in
terms.** The five tier-1 kinds are written to git first; `reconcile()` runs git → store and never the reverse;
`doctor --rebuild` re-derives from git alone; the store stays disposable under the deleted-database test. A later
pass that reads decision 5 as licence to move the log's authority is reading this record against its own words.

**7. `rk-adapter-sqlite` is not removed by this record.** Two adapters behind one port is what the port is for, and
`rk-ports`' conformance fixture already carries per-adapter *real rows* (`port_conformance_real_row_node` /
`_rust`). A happenstance-backed store is a third real row. Whether the SQLite adapter is later retired is a
decision for the record that has the conformance result in front of it.

**8. This record edits no crate, and the wiring lands after Checkpoint 8.** No file under `crates/` is touched by
it. The implementation is the first self-hosted initiative — M5 is *"Redkiln-RS runs its own fix lane and authors a
Phase-2 story"* — gated on the **S-HS** spike and its four kill criteria
(`references/2026-09-20-operator-surface-proposal.md` §12.6): a licence refusal under
`happenstance-sqlite`'s off-by-default `projection-store` feature stops it; any part of happenstance reaching
`rk-kernel` or `rk-ports` stops it (ADR-0076); a projection that cannot be pumped inline and wants a daemon stops
it (F22); and a route that requires moving tier-1 authority stops it and is closed by decision 6.

**9. The typed per-kind payload record stays OWED and is not discharged here.** `rk-types/src/event.rs`'s
**ARMED, NOT DISCHARGED** note stands unchanged. This record decides how an event is *addressed*; the 38 kinds'
payload field sets remain a design task against design §5's table. The two halves are separable precisely because
decision 2 keeps them in different places.

**10. What this record does NOT decide**, named rather than left to be discovered:

- **Whether `doctor --rebuild` can re-derive every tag from git alone.** If it cannot, the rebuild differential
  (AC-184) must exclude tags, which is consistent with decision 2 and is not a weakening — but it is **unmeasured**
  and is the sharpest open item this record leaves. Registered below.
- **The wire form of a tag.** `key:value` is the convention; whether the key is enforced, and by what, is the
  adapter's record.
- **Whether `snapshot` / `rebuild` gain semantics from happenstance's `Checkpoint` / `Authority`.** Both port
  methods are stubs today (`rebuild(&mut self, _snapshot)`); the fit is suggestive and is not taken here.
- **Whether the projection store shares a connection with the event store.** `happenstance-sqlite` carries both
  roles behind one driver; ADR-0105 clause 7 owns redkiln's connection contract. The two must be reconciled
  explicitly, and **R-SQLITE** is already registered as unmeasured under self-hosting load.
- **Which renderer comes first.** A terminal view and a web view are two renderers over one read model; this record
  decides the model and neither renderer.
- **Whether the pin is `= "=0.3.2"`.** It will be, under ADR-0100 decision 2, when a manifest names it. No manifest
  names it today, so no pin is taken here.

## Considered options

### Option — a tag set on the event, outside the canonical digest *(taken)*
Closes the addressing gap generally rather than by enumeration, expresses the conjunctions a roll-up needs
(*gates open, stage `implement`, initiative X* — which three fixed selectors cannot say at all), and moves no
digest. Its cost is one more concept in the port and a place where a defect is invisible to replay; decision 10's
first bullet and the residual under **Evidence and confidence** name that rather than smoothing it.

### Option — add `(item, run_id, at)` as three fields on `Event`
The smaller change, and the one this record's own predecessor proposal recommended before the substrate was
measured. Rejected on expressiveness: three fields answer three questions and the board asks compound ones, so the
next roll-up adds a fourth field and the one after that a fifth, each an edit to a closed crate. A tag set is the
same change made once.

### Option — put the correlation key in the payload
Rejected, and it is the option most likely to be reached for by accident because it needs no port change at all.
`canonical_events` hashes the payload, so every golden log's events change, the corpus regenerates, `CORPUS_PIN`
moves and three checkpoints re-derive — for a fact that is not part of any decision. D18-14 refused the same shape
for `Config` and stated the rule this option breaks.

### Option — index the correlation key in the adapter only, leaving the port and the event untouched
Rejected as a projection that cannot be rebuilt. An index the adapter derives is a fact the record does not carry,
so `doctor --rebuild` could not reproduce it and a second adapter could not agree with the first. It also fails the
deleted-database test in the direction that matters: deleting the store would lose addressing that nothing else
holds.

### Option — adopt happenstance as the durable log, with git as a projection of it
Rejected, and the rejection is decision 6. It supersedes ADR-0109 decision 2, ADR-0105 clause 8, design §5.6 and
AC-184 at once. The present shape is paid for by F19 (174/174 reconstructed from git) and F22, and **nothing
measured says the inversion is better**. It is a Phase-2-or-later question with its own evidence to gather, and it
is named here so that adopting the substrate is not later read as having half-taken it.

### Option — integrate now, inside Step 7d
Rejected on the stage's own terms. It needs `rk-ports` — closed at 7a and not on §7d.1's one-closed-crate
permission — while Slice 4 is in flight and Checkpoint 7d has never run, and it re-opens Checkpoints 5, 6, 7a, 7b,
7b-ii and 7c. Checkpoint 7c is 153 assertions that passed on the fifth run of its block and includes one live
`claude -p` dispatch.

### Option — a new Step 7d-ii between Checkpoint 7d and 7e
The strongest rejected option, and 7b-ii is the precedent for the shape. The gain is real: Step 8's check (1)
reads *the event log* for every lane stage and check (2) is AC-184's rebuild differential, so the new store would
be proved by the exit criterion itself. Rejected on the mirror image of that gain — **Phase 1's exit would then be
blocked on an adapter swap** against a dependency whose own README says nothing has run outside a test. The proof
is available later; the milestone is not.

### Option — write nothing now, decide with the code
Rejected because the clause that must be decided early is decision 2, and it is the one clause that stops being
free the moment the corpus regenerates for any other reason. It also leaves the last call sites of Phase 1 written
in ignorance of where they are going, for no saving.

## Consequences

**Positive**
- `status`'s **RAISED** over-reporting acquires a route to closure, and `redkiln status` over a multi-item store
  can stop handing three tier-1 kinds to every item.
- The read model the operator's 2026-09-21 ruling requires becomes expressible, and expressible for any renderer
  rather than for one.
- Decision 2 makes the whole change **digest-neutral**: no golden log moves, `CORPUS_PIN` does not move, `run_id`
  is untouched and no replay diverges. The migration is therefore additive rather than a corpus event.
- The instant arrives without the kernel reading a clock, so design §3.4's determinism ban list is untouched.
- The selector half of `s7a` §7a.3's owed record is discharged, and the payload half is left visibly owed rather
  than quietly conflated with it.
- Deciding the destination now means the fourteen selector sites standing at Phase-1 exit are the last ones written
  without one.

**Negative**
- One more concept in a port that Checkpoint 7a already passed. The port grows before it has shipped, which is the
  right order, but it is still growth.
- **A mis-addressed event is invisible to replay.** Decision 2 buys digest-neutrality with exactly this: a tag
  changed after the fact does not move `canonical_events` and `replay()` will not refuse it. The defect class is a
  routing defect rather than a decision defect, and its instrument is a `doctor` row and the projection's own
  checkpoint — neither of which exists yet.
- Whether `doctor --rebuild` can re-derive every tag from git alone is **unmeasured**, and if it cannot, AC-184's
  differential must exclude tags. Registered as an open item rather than assumed away.
- When the wiring lands it re-opens Checkpoints 7a, 7b, 7b-ii, 7c and 7d. Deferring does not avoid that bill; it
  changes who pays it, from a hand-driven stage to the machine Phase 1 exists to build.
- Redkiln would be among the first consumers of `happenstance` (71–169 downloads per crate at 2026-09-21), whose
  README states it carries no production mileage. The mitigation is D-V12's — run the published
  `happenstance-testkit` in redkiln's own CI against the shipped pin — and it is a mitigation, not a cancellation.

**What this forbids**
- Forbids putting an item id, a run id, a commit SHA, a stage, a lane, a node, a gate token, an AC id, a worktree
  or any other correlation key **inside** `rk-types::Event.payload` for the purpose of addressing. Addressing is a
  tag.
- Forbids `canonical_events` or `event_json` gaining any member beyond `kind` and `payload` — tags, the instant,
  the store identity and the sequence position are each forbidden inside it — and forbids moving
  `rk-kernel/golden-log/v2` for this change.
- Forbids minting the instant anywhere but the store's own `append`, and forbids using it as an ordering key
  anywhere at all.
- Forbids a tenth reserved tag key without a record, and forbids a tag key invented at a call site.
- Forbids any part of `happenstance` — or any async event-store library — appearing in `rk-kernel`, `rk-ports`,
  `rk-types`, `rk-store` or `rk-git`. ADR-0076 confines `.await` to adapters and AC-484 forbids a
  `rk-kernel` → `rk-adapter-*` edge; both stand unchanged.
- Forbids reading decision 5 as moving tier-1 authority. Forbids a store adapter that is not disposable under the
  deleted-database test, forbids `reconcile()` running store → git in any direction for any reason, and forbids a
  `doctor --rebuild` that re-derives from anything but git.
- Forbids an append condition, a lease, a fencing token or any other concurrency mechanism entering redkiln's use
  of the store while ADR-0105 clause 7's one-writer rule holds — a mechanism adopted for a contention that does not
  exist is a mechanism nobody can test.
- Forbids a projection that requires a surviving process, a daemon, a watcher or a background indexer (F22).
- Forbids editing `rk-ports`, `rk-kernel`, `rk-adapter-sqlite` or any other closed crate **for this record** before
  Checkpoint 8 is recorded as passed, and forbids landing the wiring without S-HS having run and reported.
- Forbids treating this record as discharging the typed per-kind payload. That note stays **ARMED, NOT
  DISCHARGED** until its own record exists.

## Evidence and confidence

- [measured 2026-09-21, `1308311`] `crates/rk-ports/src/event_store.rs`'s header carries *"a ROUTING, not a
  decision … reversible by the record that is owed"*, *"Checkpoint 7a asserts none of it"* and *"ARMED, NOT
  DISCHARGED"*, verbatim; `Query` is three `String`-keyed variants :29-35; `Snapshot` is an opaque newtype with no
  field set :41.
- [measured 2026-09-21, `1308311`] `crates/rk-adapter-sqlite/src/lib.rs`:322 is
  `SELECT kind, payload FROM t2_event ORDER BY id` — no `WHERE`, the selector inert.
  `crates/rk-cli/src/verbs/status.rs`'s RAISED block states the over-reporting and its cause, quoted above.
- [measured 2026-09-21, `1308311`] `crates/rk-kernel/src/replay.rs`'s `event_json` emits
  `{"kind": …, "payload": …}` and `canonical_events` hashes that alone (:1073-1078); `run_id` is
  `sha256_lp([SCHEMA, stage, canonical(inputs)])` (:968-977), so it is over **inputs** and a tag cannot move it;
  the `golden-logs/` layout is :88-120 and its corpus placement is `s7b` §7b.11.
- [measured 2026-09-21, `1308311`] the blast radius: **8** `SqliteStore::open` call sites, all in `rk-cli`
  (`adapters.rs`, `doctor/mod.rs`, `verbs/{run,answer,status,explain,init}.rs`); **14** selector sites
  (`Query::By*` / `journal_query`) across seven files; **11** `.query(` sites in the same seven; `snapshot` and
  `rebuild` are stubs — `rebuild(&mut self, _snapshot)`.
- [measured 2026-09-21, `1308311`] `grep -cE 'crates/[a-z-]+/src' runbook/phase-1/s7e-pack-authoring.md` → **0**;
  the same over `s8-exit-criterion.md` → **1**, a read-only `grep` over `rk-git`. The fourteen selector sites are
  therefore the final Phase-1 count and deferring the wiring accretes none.
- [measured 2026-09-21, crates.io API] `happenstance` / `-core` / `-sqlite` / `-testkit` at **0.3.2**, published
  2026-09-21, **MIT OR Apache-2.0**, MSRV **1.97.1**; downloads 91 / 169 / 71 / 104. `happenstance-sqlite` depends
  on `rusqlite = "0.40"` with `bundled` — the same minor line and feature as this workspace's `= "=0.40.2"`, so the
  graph unifies under redkiln's exact pin — and on `tokio` with `rt` as a **normal** dependency, which is why
  decision 5 confines it to an adapter.
- [measured 2026-09-21, `D:\repos\happenstance` @ `55288c5`] `SequencedEvent` carries `position`, `id`,
  `recorded_at` and `event`; `RecordedAt(i64)` is millis since epoch and **deliberately not `Ord`** — the derive
  was removed at `0.2.0` so a caller cannot order by it. `Tags` / `Query` / `QueryItem` are
  `crates/happenstance-core/src/{tag,query}.rs`. `run_projection(events, models, projection, codec, chunk) ->
  Progressed{through, applied}` is `crates/happenstance/src/runner.rs`:514 — one call, chunked, no daemon;
  `Checkpoint::{NeverRun, Live{through}, Rebuilding{through}}` and `Authority::{Live, Rebuilding}` are
  `projection.rs`:184-231.
- [documented] `s7d` §7d.1 and §7d.15 — two crates written, one closed crate edited (`rk-kernel`), the disturbance
  occasions Slices 2/3/4 and *"Slice 4 is indeed the last"*; `PROGRESS.md` §1 — 7d Slices 1-3 landed, Slice 4 in
  flight, Slice 5 not started, Checkpoint 7d not run, 7e and 8 not started; Checkpoint 7c recorded as passing on
  the fifth run of its block at 153 assertions including one live dispatch.
- [documented] D18-14's *"no kernel event payload changes"* and its byte-stability reasoning, quoted above;
  ADR-0109 decision 2's deleted-database test; ADR-0105 clauses 7, 8 and 9; design §5.6 and §8.6; F19 (174/174) and
  F22 as the warrants for tier-1 authority and for the no-surviving-process rule.
- **Confidence: high on the gap, high on decision 2, and lower on decision 5's adapter than on the shape it
  adopts.** The gap is measured, not inferred — the adapter's `SELECT` and `status`'s own header are the evidence,
  and neither is an interpretation. Decision 2 is the clause a reviewer should press on least: it follows from a
  one-line reading of `event_json` plus D18-14's precedent, and both are cheap to re-measure. **What a reviewer
  should press on is decision 10's first bullet** — whether `doctor --rebuild` can re-derive every tag from git
  alone — because if it cannot, AC-184's differential narrows, and this record asserts only that the narrowing is
  *consistent* with decision 2, not that it is harmless. **The second thing to press on is decision 5's residual
  named under Negative**: a mis-addressed event is invisible to replay by construction, and the instruments that
  would catch it are named and do not exist. **And the third is the adoption itself** — one library, one
  maintainer, 71-169 downloads, a README that disclaims production mileage — where the honest mitigation is the
  published 116-rule conformance suite run in redkiln's own CI, which is more than a third-party store would offer
  and is still not mileage.
- **Status is `proposed`, on ADR-0126's route and ADR-0128 / ADR-0129 / ADR-0130's precedent**: it is written
  against a measured gap and a stated product requirement, and it awaits the operator's dated confirmation line
  under `spikes/DECISIONS-2026-09-21.md`. Until that line exists, the selector routing in
  `crates/rk-ports/src/event_store.rs` stands as written and no crate is edited.
