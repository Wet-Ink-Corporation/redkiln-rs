---
id: "0182"
title: "**The interim `item` key rides beside the event, outside `payload` and outside `canonical_events`, and `query` honours `ByItem`** (`s8e`'s record; D21-43 N5; D21-46 R3): `rk_types::event::Event` gains a private optional `item` beside `kind` and `payload`, persisted in the `t2_event.item` column `0002` adds; `EventKind::item_key()` classifies every kind, total and with no catch-all, as carrying the key, keeping ADR-0163's payload member, or not item-scoped; `ByItem(x)` returns the rows keyed `x` plus, in the interim, the rows that carry no key; `ByRun` stays inert. No golden log's canonical digest moves. Supersedes nothing: ADR-0131's first forbids bullet is honoured, not superseded, because the key is not in the payload"
status: accepted
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-43 (N5 — *"**An interim `item` key on every item-scoped event kind**, moving to ADR-0131's tag later as ADR-0163's interim key does; `query` honours `ByItem`. Supersedes ADR-0131's first forbids bullet in part, for those members."*) and D21-46 (R3 — *"**Carried outside the canonical digest**, as ADR-0131 places its tag set. Digests and golden logs are unchanged, and `query` still uses the key."*); runbook/phase-1/s8e-tier-one-and-the-lane.md (the records table's fifth row, §8e.0 item 3's golden-log digests taken 2026-09-28 at `766854c`, §8e.10, and §8e.13 finding 12); references/adr/0131 (decisions 1, 2, 4 and 8 and its forbids list) and 0163 (its `rate_limit_reading.item` member); crates/rk-types/src/event.rs, crates/rk-types/src/event_kind.rs, crates/rk-kernel/src/replay.rs (`event_json`, `canonical_events`), crates/rk-kernel/src/runtime.rs (`journal_query`), crates/rk-ports/src/fakes.rs and crates/rk-adapter-sqlite/src/lib.rs, as read 2026-09-28 at `766854c`
supersedes: []
superseded_by: []
related: ["0097", "0116", "0131", "0163", "0181"]
evidence: []
consumers: []
---

## Context and problem

**Two items in one store fold together.** `rk-types`' `Event` is `{ kind, payload }`, `stage_transition_event`
carries no item, and `SqliteStore::query` ignored its selector: `fn query(&self, _by: Query)` ran `"SELECT kind,
payload FROM t2_event ORDER BY id"` (RT-4), and `VecStore::query` returned every event. Every kernel read goes through
`rk_kernel::runtime::journal_query(item)`, which is `Query::ByItem(item)`, so a second item's `run` in the same store
folds the first item's transitions, and `stages_entered` can refuse its advance. `s8` runs a fix item **and** a story
item in one repository.

**The operator took the decision in two steps.** D21-43 N5: *"**An interim `item` key on every item-scoped event
kind**, moving to ADR-0131's tag later as ADR-0163's interim key does; `query` honours `ByItem`. Supersedes ADR-0131's
first forbids bullet in part, for those members."* D21-46 R3 then placed it: *"**Carried outside the canonical
digest**, as ADR-0131 places its tag set. Digests and golden logs are unchanged, and `query` still uses the key."*

**ADR-0131 already fixed where addressing lives, and when it is wired.** Its decision 2: *"Tags are carried on the
event and are OUTSIDE `canonical_events`. `canonical_events` stays `{kind, payload}`, byte for byte."* Its first
forbids bullet: *"Forbids putting an item id, a run id, a commit SHA, a stage, … **inside** `rk-types::Event.payload`
for the purpose of addressing."* Its decision 8: *"This record edits no crate, and the wiring lands after Checkpoint
8."* And its bullet *"Forbids editing `rk-ports`, `rk-kernel`, `rk-adapter-sqlite` or any other closed crate **for
this record** before Checkpoint 8 is recorded as passed"*.

**The golden logs, measured** (`s8e` §8e.0 item 3, 2026-09-28 at `766854c`). One golden log exists under
`CORPUS_PIN`, `redkiln-corpus` at `61f590e`'s `golden-logs/implement-bf1d8cec3b22d288/`, and its `events.jsonl` is
the same git blob (`c2da022`) as the committed `crates/rk-kernel/tests/fixtures/logs/implement-bf1d8cec3b22d288/`.
Its canonical digest, printed from inside the `rk-kernel` test binary, is
`73f7553df2f84cc7b931a39b276fcfdb0b36371bb93ea461a9b10ca8a47e4e90` (8 lines, 2450 bytes).

## Decision drivers

- N5: the key exists on every item-scoped kind, and `query` honours `ByItem`.
- R3: the key is outside the canonical digest, so no golden log, `CORPUS_PIN` or `run_id` moves.
- ADR-0131's first and second forbids bullets, which a key inside `payload` or inside `event_json` would break.
- ADR-0131 decision 8: the tag set is not wired before Checkpoint 8, and this carrier must not be that wiring.
- A decision is taken once. ADR-0163 already keys `rate_limit_reading` in its payload; a second source for the same
  fact can disagree with the first.

## Considered options

### Option — a private optional `item` beside `kind` and `payload` on `Event`, persisted in `t2_event.item` (taken)

The smallest carrier that is outside `payload`, outside `event_json` and therefore outside `canonical_events`. It
reaches the store through the port's existing `append` and `append_tier1` (ADR-0181), so no port method is added for
it. It is one member, for the one reserved key ADR-0131 decision 4 names for the item-less kinds, and it moves to the
`item` tag when ADR-0131's wiring lands.

### Option — ADR-0131's tag set now

The destination. Rejected here: ADR-0131 decision 8 lands the wiring after Checkpoint 8, gated on S-HS, and a tag set
built now would be that record's wiring, taken early and without its spike.

### Option — the key inside `payload`, as ADR-0163 did for one row

What N5 first said. Rejected by R3: `canonical_events` hashes the payload, so every golden log's events would move,
and ADR-0131's first forbids bullet would need a supersession for every item-scoped kind.

### Option — three fields, `(item, run_id, at)`, on `Event`

ADR-0131's own rejected option, rejected there on expressiveness. This record does not take it: it adds one member,
for one reserved key, as an interim carrier that the tag replaces, and it widens no selector.

### Option — an index derived in the adapter only

ADR-0131 rejected it as *"a projection that cannot be rebuilt"*. Rejected here for the same reason, and because the
fake could not agree with it.

## Decision

**1. The carrier.** `rk_types::event::Event` gains a **private** `item: Option<String>` beside `kind` and `payload`.
`Event::new` leaves it `None`; `Event::with_item(item)` sets it; `Event::item()` reads it. It is not a member of
`payload`, and `rk-kernel`'s `event_json` and `canonical_events` stay `{kind, payload}` byte for byte, so the key is
outside the canonical digest. Event equality (`PartialEq`) includes the key: an event read back from a store equals
the event appended, key and all. A replay compares canonical bytes, which do not carry it.

**2. The column.** The store persists the key in `t2_event.item`, which ADR-0181's `0002_gate_transitions` adds,
beside `anchor_sha`, with an index. `append` and `append_tier1` write it; `query` reads it back onto the event.

**3. The members, enumerated.** `EventKind::item_key()` is total with no catch-all, so a kind added later without a
decision is a compile error, and `fsm::every_item_scoped_kind_carries_the_interim_item_key` iterates `EventKind::ALL`
against this list, so a kind added later without a decision also fails a test:
- **Carries the key** (`ItemKey::Carried`): `stage_transition`, `gate_opened`, `gate_answered`, `gate_expired`,
  `slice_sealed`, `reconcile_started`, `gate_answer_void`, `gate_answer_spent`, `gate_resumed`, `review_block`,
  `quarantine_granted`, `quarantine_removed`, `quarantine_expired`, `selection_ineffective`, `stage_exit_refused`,
  `dag_assembled`, `dag_amended`, `activity_committed`, `dispatch`, `dispatch_completed`, `step_failed`,
  `step_cancelled`, `step_refused`, `rate_limited`, `degraded`, `structured_drift`, `drift_advisory`,
  `review_class_drift`, `artifact.read` and `artifact.write`. Each is produced for one item's run, by the kernel, the
  composition root or the adapter a run's store is handed to. `dispatch_completed` is one of them; `s8f` cites this
  record for it and keeps `activity_id_at_version` and the breakdown (D21-42 12b).
- **Keeps ADR-0163's payload member and does not carry the key** (`ItemKey::InPayload`): `rate_limit_reading`. Its
  `item` stays in the payload where ADR-0163 put it, and `cost` reads it there. Carrying the same fact beside it as
  well would be a second source that can disagree with the first. Both move to the `item` tag when ADR-0131's wiring
  lands.
- **Not item-scoped** (`ItemKey::NotItemScoped`), each with its reason: `memo_hit`, `memo_miss` and `memo_evicted`,
  because the memo is keyed by content and shared across items by design (ADR-0097 decision 1); `rewrite`, because
  `old_sha → new_sha` is a fact about the repository that any item referencing the sha reads; and `advisory_created`,
  `advisory_retired` and `advisory_dismissed`, because the advisory sink is store-wide and AC-310's invariant
  (`live_sink == created - retired - dismissed`) is over the whole sink.
- Three kinds `s8e` mints later (`worktree_assigned`, `stage_empty`, `git_op`) take their classification in the change
  that mints them, because the match is total.

**4. `query` honours `ByItem`, in `rk-store`'s SQL and in `VecStore`.** `ByItem(x)` returns, in append order, the rows
whose `item` is `x`, **and, in the interim, the rows that carry no key**. A row keyed to another item is never
returned. **What an item-less row does, stated:** it is read as belonging to no item in particular, which is what
every row on the tree was before this record, and it is **not** read as the one item the store has ever held. `s8e`
§8e.10 recommends exclusion plus a named `doctor` finding, because a guess about ownership is the defect N5 exists to
end. This record takes the inclusive reading **as the interim**, and says why: at this record's writing no producer
sets the key yet, every reader (`rk_kernel::runtime::journal_query`, `cost` and `status` through
`journal_query("")`) reads through `ByItem`, and exclusion would empty every one of those reads until every producer
of a `Carried` kind keys its rows. The inclusive reading is the reversible one: exclusion is a one-clause change to
the `WHERE` and to `VecStore`'s filter, taken once every producer keys and `cost` and `status` read the whole store by
a selector that says so. The `doctor` finding for an item-less row of a `Carried` kind is named and owed, not built.
Rows written before this stage carry no key; the shipped store is rebuilt at schema version 2 (ADR-0181 decision 6),
and the rebuild keys what it re-derives.
*(dated 2026-09-28, `s8e` build, package S8E-RULINGS.)* **Confirmed as written by the operator**
(`spikes/DECISIONS-2026-09-21.md` D21-94, confirmation 1: *"under `ByItem(x)` a row that carries no key is returned,
as the **interim** (the inclusive reading), with exclusion plus a named `doctor` finding taken once every producer of
a `Carried` kind keys its rows"*), departing from `s8e` §8e.10's recommendation for the reason given above.

**5. `ByRun` stays inert, and `BySha` selects by anchor.** No run carrier exists on an event, so `ByRun(_)` returns
every row, as it did, and the conformance bodies that query by run are unchanged. `BySha` is ADR-0181 decision 12's:
it returns the rows anchored to that commit.

**6. The golden logs do not move, and a test pins it.**
`replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key` has two arms. The first: the committed
golden log's canonical digest equals the value §8e.0 item 3 recorded,
`73f7553df2f84cc7b931a39b276fcfdb0b36371bb93ea461a9b10ca8a47e4e90` (and, under the `corpus` feature, so does the copy
under `CORPUS_PIN`). The second: a journal with the key set on every event and the same journal without it have the
same canonical digest. A carrier that wrote the key into `payload` reddens the second arm.

**7. ADR-0131 is honoured, not superseded, and decision 8 is argued rather than dismissed.**
- The first forbids bullet forbids the key **inside** `payload`. It is not there. N5's *"Supersedes ADR-0131's first
  forbids bullet in part"* is overtaken by R3, and this record supersedes nothing of ADR-0131 (`s8e` §8e.13, finding
  12).
- The second forbids bullet, *"Forbids `canonical_events` or `event_json` gaining any member beyond `kind` and
  `payload`"*, is honoured: neither gains one.
- Decision 1 is untouched: `Query` is not widened to a tag conjunction, no key vocabulary is reserved, and `ByItem`
  means what decision 1 says the reserved keys mean, *"exactly the meaning they carry today"*.
- **Decision 8 and its bullet.** This record edits `rk-types`, `rk-ports`, `rk-store`, `rk-kernel` and
  `rk-adapter-sqlite` before Checkpoint 8. The bullet binds unless the edit is not *for this record* (ADR-0131). The
  argument that it is not: the carrier executes D21-43 N5 and D21-46 R3, not ADR-0131; it widens no `Query`, reserves
  no key and adopts no happenstance (decision 5). **The point an adversarial reader will press, stated:** the carrier
  does deliver, ahead of the tag, decision 4's substance, *"The three item-less tier-1 kinds gain an `item` tag."* It
  is a stand-in for decision 4, and it moves to the tag when the wiring lands, as ADR-0163's member does. The argument
  is named for this record's adversarial pass. If the pass rejects it, this record takes that bullet as a
  **supersession in part, for the interim carrier alone**, and says so by dated amendment before it flips.
- **Cited, not superseded:** ADR-0131 decisions 1, 2, 4 and 8, and its second forbids bullet; ADR-0163, whose
  `rate_limit_reading.item` stays where it put it.

**8. The cost, stated as ADR-0163 stated it.** ADR-0131's driver *"Waiting accretes nothing"* is broken for these
members in one respect: a carrier exists before ADR-0131's tag wiring does. It moves to the reserved `item` tag when
that wiring lands, and the column's meaning does not change.

## Consequences

**Positive**
- Two items' rows are told apart in one store once their producers key them, and `lane_e2e::two_items_in_one_store_do_not_fold_together`
  can hold.
- No canonical event, golden log, `CORPUS_PIN` or `run_id` moves.
- The classification is total, so a new kind cannot enter without a decision about its item.

**Negative**
- **The interim inclusive reading can mis-own a row a producer forgot to key**: an item-less row of a `Carried` kind
  is read by every item. The test pins the classification, not the producers; the `doctor` finding that would name
  such a row is owed, and exclusion waits on every producer keying.
  *(dated 2026-09-28, `s8e` build, package S8E-P6.)* **Which producers key now.** These key their rows:
  - the tier-1 boundary (`tier_one::commit_act` and the heal);
  - `worktree_assigned`;
  - `Runtime::append` when the runtime is bound to an item (`Runtime::with_item`, which `replay::resume_routed`
    binds): the plan's rows, `dispatch`, and the `stage_empty` row beside an advance.

  These still write item-less rows of `Carried` kinds, and remain on this residual:
  - the executor's own guard appends in `runtime/activity.rs` (`activity_committed` among them);
  - every row a harness appends through the store it is handed.

  `lane_e2e::two_items_in_one_store_do_not_fold_together` holds over this partial keying. The rows that decide
  its folds are keyed: the stage and gate transitions and `dag_assembled`. `activity_committed` is harmless
  here because its idempotency key names its item.
- `cost` and `status` read the whole store through `journal_query("")`, which returns every row only while rows are
  unkeyed. Once producers key, they need a whole-store selector, and that is owed with exclusion.
- Event equality now includes the key, so a test comparing a keyed event with an unkeyed one of the same content
  reads them as different.
- A mis-keyed event is invisible to replay, which is ADR-0131's own stated negative for its tags, inherited here.

**What this forbids**
- Writing the interim key into `payload`, into `event_json` or into `canonical_events`.
- A kind added to `EventKind` without an `item_key()` classification.
- Carrying `rate_limit_reading`'s item both in its payload and beside it.
- Reading an item-less row as the one item the store has ever held.
- `ByItem(x)` returning a row keyed to an item other than `x`.
- Reading this record as ADR-0131's tag wiring, or as reserving a key vocabulary.

## Evidence and confidence

**Measured** (2026-09-28, at `766854c`): the golden log's canonical digest, printed from inside the `rk-kernel` test
binary; that `query` ignored its selector on both rows; that every kernel read goes through `ByItem`. **Documented:**
the ledger lines and records quoted. **Not measured, and named:** exclusion's effect on every reader, and the `doctor`
finding, are owed. This record is `proposed` and flips only on the operator's dated ledger line.
