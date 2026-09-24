---
id: "0163"
title: "**`redkiln cost <ITEM>` is the ninth verb, and the reading it renders is journalled ONCE PER DISPATCH, keyed by the item the composition root already binds.** `0133` decision clause 2's *\"once per reading\"*, clause 3's `<RFC 3339 | null>` spelling and — for the flat shape only — its `windows` member and matching forbids bullet are **superseded in part**, and so is `0131`'s first forbids bullet, for this row's `item` alone and until `0131`'s `item` tag is wired: `BatchDraw.n` is documented as *\"The number of dispatches in the set\"*, so a row per parsed frame would make `n` count frames under the same name; and the platform reports `resetsAt` as integer unix seconds, `Timestamp` is a transparent `i64`, and no crate in the workspace can format RFC 3339 — the spelling would be a derivation, not a measurement. The payload gains **`item`**, a member `0133` did not name, copied from the `\"item\"` input `verbs/run.rs` already binds rather than from a new reserved key; `cost` counts interleaving over **traffic**, not only over samples, because `throttle::attribute` sees only the samples it is given; and Checkpoint 7i's *\"an item with dispatches\"* is satisfied over a **fixture corpus**, `0162`'s route, because the shipped store holds zero `dispatch_completed` rows"
status: proposed
date: 2026-09-23
legacy_id: none
source: references/adr/0133 (decision clause 2 — *"The producer is the Harness adapter, once per reading"* and *"A reading is appended when the adapter's stream reader parses a `rate_limit_event` line"*; clause 3's `resets_at` bullet — *"`{ five_hour: <RFC 3339 | null>, seven_day: <RFC 3339 | null> }`"*; the forbids bullet *"Forbids a payload that drops `windows.five_hour` / `windows.seven_day` or renames them"*); references/adr/0110 (decision 1 — *"The named set is the account's complete dispatch sequence between the two readings"*); references/adr/0162 (the fixture-corpus route); runbook/phase-1/s7i-cost.md (§7i.1–§7i.3 as read at `76ae138`); runbook/phase-1/s7d-cli-and-migrate.md (§7d.3's *"and no others"*); and, each read at `76ae138` on 2026-09-23 — crates/rk-cli/src/cli.rs's module doc (*"a ninth is a decision, taken in a record, not a derive attribute"*), crates/rk-cli/src/verbs/run.rs's `BindRequest` (*"inputs: vec![(\"item\".to_owned(), serde_json::json!(resolved.item.id()))]"*), crates/rk-adapter-claude/src/throttle.rs (`BatchDraw.n`'s doc; `attribute`'s *"`owned[i]` says whether sample `i`'s dispatch belongs to the item"*), crates/rk-types/src/time.rs (`#[serde(transparent)] pub struct Timestamp(i64);`), crates/rk-types/src/event.rs (`Event { kind, payload }`), crates/rk-cli/src/doctor/mod.rs (`let journal = read_journal(&worktree.join("events.jsonl"));`), and the shipped store `.git/redkiln/store.sqlite3` opened read-only
supersedes: ["0133", "0131"]
superseded_by: []
related: ["0110", "0126", "0131", "0133", "0162"]
evidence: ["F6"]
consumers: []
---

## Context and problem

Phase-1 exit criterion (7) invokes `redkiln cost <item>`. At `76ae138` [measured 2026-09-23] nothing
the verb needs exists except the calculator:

- **No verb.** `crates/rk-cli/src/cli.rs`'s `Command` enum has eight variants — `Run`, `Answer`,
  `Status`, `Explain`, `Doctor`, `Init`, `Catalog`, `Lint` — under *"The eight verbs, and no others
  (§7d.3)"*, and its module doc says *"a ninth is a decision, taken in a record, not a derive
  attribute"*.
- **No producer.** `EventKind::RateLimitReading`'s only non-`rk-types` occurrences in any `src/` are
  the three kernel fold arms (`workflow/breaker.rs`, `workflow/fsm.rs`, `workflow/memo.rs`).
  `headroom::read_rate_limit_event` parses each reading, the drain keeps the last one, the guard
  observes it, and it is never journalled.
- **No caller.** `throttle::attribute` has zero callers in any `src/`; its two callers are in
  `crates/rk-adapter-claude/tests/it/throttle.rs`.
- **No renderer.** `not attributable`, `low_sample` and `resolution_limited` occur zero times in
  `crates/rk-cli/`.

`0133` settled the producer and the payload in outline. Three things it did not settle surface the
moment the verb is built, and one of them decides whether `cost <item>` can mean what its name says.

**1. Nothing on the journal can say which item a reading belongs to.** `attribute`'s contract is
*"`owned[i]` says whether sample `i`'s dispatch belongs to the item"*, and ADR-0110's first rendering
— `not attributable` with the interleaving count — is computed from exactly that. Measured at
`76ae138`: `Event` is `{ kind, payload }` and nothing else; `DispatchRecord` has no `item` and no
`run_id`; `ActivitySpec` is `card_digest, tools, schema, timeout_secs, inputs`; `t2_event` is
`(id, kind, payload, anchor_sha)`; and `SqliteStore::query` is `SELECT kind, payload FROM t2_event
ORDER BY id` with no `WHERE`, so `Query::ByItem` selects everything. **But the item already reaches
the adapter**: `verbs/run.rs` binds every activity with
`inputs: vec![("item".to_owned(), serde_json::json!(resolved.item.id()))]`, and `ActivitySpec.inputs`
is what `ClaudeHarness::dispatch` receives.

**2. `0133` clause 2's "once per reading" makes `n` count something else.** ADR-0110 decision 1 makes
headroom *"a batch delta over an explicitly named set of dispatches, with `n` on it"*;
`throttle::BatchDraw.n` is documented *"The number of dispatches in the set — reported with the
batch, never divided into it"*; and `batch_draws_by_reset` sets `n` to the **sample** count. One row
per parsed `rate_limit_event` frame makes samples equal frames, so a dispatch whose stream carries
two readings contributes two to `n`. And the drain keeps only `Drained::last_reading`, on a task
that holds no store, so "once per reading" is not reachable from where the store is in scope without
widening the runner — to produce a count this record would then have to undo.

**3. `0133` clause 3's `<RFC 3339 | null>` is not expressible, and would be a derivation.** The
platform reports `resetsAt` as an integer — the runner-journal records in `e1-smoke.L1-3.jsonl`, which
re-serialise the captured `rate_limit_event` lines inside their `unknown_events` array, carry
`"resetsAt":1788370800`, and the scripted stream fixtures under `crates/rk-adapter-claude/tests/fixtures/streams/`
carry the same integer shape — `headroom.rs` reads it with `Timestamp::from_unix_seconds`, and
`Timestamp` is `#[serde(transparent)] pub struct Timestamp(i64);`. No crate's `Cargo.toml` depends on
`chrono` (the workspace table carries a row nobody uses) and nothing in the workspace formats RFC
3339. `s7i` §7i.1 says *"A reading is a measurement, not a derivation. The row carries what the
platform reported, verbatim"*.

**And the shipped tree cannot exercise any of it.** The shipped store holds `dag_assembled` 1,
`reconcile_started` 3, `stage_transition` 2 — **zero `dispatch_completed`** and zero
`rate_limit_reading` rows [measured 2026-09-23 at `76ae138`, `mode=ro`]. `s7i-cost.md` says *"The
first blocker is cleared by `s7h`"*; `s7h` made the lane advance and made a **fixture** dispatch,
and **0 of 102** shipped activities complete a committing walk, unchanged.

## Decision drivers

- **`n` must mean what ADR-0110 says it means.** Every rendering is keyed on it: `low_sample` is
  `n < 10`, and the delta is reported *with* `n`. A producer that changes what `n` counts silently
  redefines the measure — the same failure `0133` rejected ADR-0110's amendment for.
- **Absent is not zero, and verbatim is not reformatted.** `0133`'s own forbids already refuse a
  coerced `resets_at`; a formatted one is a milder form of the same edit.
- **An ownership key must fail safe.** A reading with no owner must count against attribution, never
  for it, so that a missing key produces `not attributable` rather than a false delta.
- **`attribute` counts only the samples it is given.** A foreign dispatch that produced no reading —
  killed before the first `rate_limit_event`, or one whose reading the scanner refused — never becomes a
  sample, and may have drawn on the account all the same. (One refused before spawn drew nothing, and
  is counted anyway: the verb cannot tell the two apart from the journal, and errs toward
  `not attributable`.)
- **The shipped registry cannot dispatch.** Measured at `e0802a5` on 2026-09-24: **0 of 8**
  `.redkiln/cards/*.json` carry `permission_mode` (`0156`'s blocker (0)); `.redkiln/schemas/` does not
  exist and `.redkiln/processes/` is empty; and **0** `model =` rows sit under the **16** `[role.*]`
  tables of the two shipped packs, which the binder refuses as `ConfigError::ModelUntraceable`
  (`crates/rk-adapter-claude/src/binder.rs`) — a refusal `0156`'s six-item list does not name. `0156`
  forbade authoring `.redkiln/schemas/` or populating `.redkiln/processes/` *"under this stage's
  scope"* — `s7h`'s — and `s7i` §7i.1–§7i.2 likewise name none of these, so this stage does not author
  them either.
- **`0162` already established the route** for a checkpoint bullet the shipped tree cannot satisfy:
  a fixture corpus under `crates/rk-cli/tests/fixtures/`, a behavioural guard over a real binary
  run, and the shipped tree left alone and said to be left alone.

## Considered options

### Option — A reserved input key set by the binder

Add a key beside `binder.rs`'s `WORKTREE_KEY`: the binder sets it from a new `BindRequest` member
and refuses a collision; the adapter reads it and filters it out of the prompt. This was the route
pre-taken, unrecorded, at `s7h`'s close. **Rejected on a measurement that route did not have**: the
item already travels on `spec.inputs` under `"item"`, set by the one composition root that binds
activities. A second key would carry the same value twice, change `BindRequest`'s contract, and
move the prompt bytes of every existing binding for no information gained.

### Option — Ownership by journal position

The kernel appends `dispatch{activity_id_at_version, attempt, item, step_id}` immediately before
`harness.dispatch` (under a guard `Runtime::append` releases before the dispatch's own guard is
taken), so within one process a reading's owner is the item of the nearest preceding `dispatch` row. No adapter change. **Rejected**: two processes over one
store interleave their rows, and position then attributes a reading to the wrong item *silently* —
the one error `not attributable` exists to surface.

### Option — `0131`'s `item` tag, outside the payload

`0131` decided that addressing is a tag carried on the event and outside `canonical_events`, reserved
`item` as a key, and forbade *"putting an item id, a run id, a commit SHA, … or any other correlation
key **inside** `rk-types::Event.payload` for the purpose of addressing"*. That is the right home for
this member, and `cost` does address readings by it. **Rejected for now, not in principle**: the tag
does not exist — `Event` is `{ kind, payload }` — and `0131` decision 8 lands its wiring *"after
Checkpoint 8"*, gated on **S-HS**, and forbids editing `rk-ports` or any closed crate for it before
then. The only carriers available to `s7i` are the payload and journal position, and position is
rejected above.

### Option — Once per reading, with `Drained` widened to a `Vec`

Keep `0133` clause 2 literally. **Rejected**: it makes `n` a frame count (driver 1), and `cost` would
then have to collapse readings back into dispatches, which is a second decision taken to undo the
first.

### Option — `resets_at` as RFC 3339

Keep `0133` clause 3 literally, with a hand-written civil-from-days formatter in the adapter and a
parser in `cost`. **Rejected**: it adds a date codec in both directions to write a value the platform
never sent in that form, and every sample would round-trip through it before being compared.

### Option — Satisfy Checkpoint 7i on the shipped tree

Run a real lane after the producer lands. **Rejected**: the shipped registry does not dispatch
(driver 5), so the run would not reach `ClaudeHarness::dispatch`, and the checkpoint would read the
empty case and fail its grep — or worse, be rewritten until the empty case passed it.

### Option — The record's route (chosen)

The existing `"item"` input on the reading as an **interim** payload member that moves to `0131`'s tag
when that wiring lands, one row per dispatch, integer seconds, interleaving over traffic, and a fixture
corpus for the dispatched case.

## Decision

**1. `redkiln cost <ITEM>` is the ninth verb.** This amends `s7d` §7d.3's *"and no others"* and
discharges `cli.rs`'s *"a ninth is a decision, taken in a record"*. It **appends no event**: it does
not reconcile, constructs no harness it calls, and writes no journal row (AC-395: *"from the event
store alone … it dispatches no model call"*). It is not strictly read-only at the file level: opening
the store through `SqliteStore::open` applies the schema and, on a repository with no store, creates
and identity-stamps one — as `status` and `explain` already do. It exits `0` on every rendering **including the empty
one**. A repository it cannot discover, a store it cannot open or read, or an **owned** row whose
payload is malformed is `Exit::ConfigError` (`3`), as `explain` treats the same conditions.
`Exit::NoRecord` (`5`) is not used: `crates/rk-cli/src/verbs/status.rs`'s module doc calls it
`explain`'s alone, and AC-432 gives it to *"an item with no recorded resolution"*, which is not a
condition `cost` has.

**2. `0133` decision clause 2 is superseded in part: the reading is appended ONCE PER DISPATCH.** The
Claude adapter appends one `rate_limit_reading` row per dispatch that went **through the child**,
from `Run::last_reading` — the stream's last representable reading — **before** that dispatch's
`dispatch_completed` row. What `0133` clause 2 decided stands otherwise: the producer is the Harness
adapter, through the `&mut dyn EventStore` it already holds, **whether or not the dispatch succeeds**.
No row is written on a pre-spawn path (the throttle's and overage's refusals hold only the *previous*
reading, and writing it again would double-count a sample), on the panic path, or when the stream's
reading was refused (`reading_refused` already leaves `last_reading` as `None`). A refused append is
counted under AC-368 exactly as a refused `dispatch_completed` append is, and does not overturn the
dispatch's result.

**3. `0133` decision clause 3 is superseded in part: the payload, with a fixed key set.** Every row
carries all six members, and a member the frame did not carry is `null`:

- **`windows`** — `{ five_hour: <u16>, seven_day: <u16> }` for a `Shape::Unified` reading, byte-
  compatible with `doctor`'s reader as `0133` required. **A `Shape::Flat` reading writes
  `"windows": null`.** `0133`'s forbids bullet *"Forbids a payload that drops `windows.five_hour` /
  `windows.seven_day`"* is superseded **for the Flat shape only**: a flat frame carries one
  utilisation and no window pair, and writing it into both windows is the extrapolation AC-374 and
  `s7i` §7i.1 forbid.
- **`utilization`** — the Flat reading's ticks, verbatim; `null` for Unified. A member `0133` did not
  name, so that a flat measurement is journalled rather than discarded.
- **`resets_at`** — `{ five_hour: <integer unix seconds | null>, seven_day: <… | null> }`, read
  through `resets_at(window)` exactly as `0133` clause 3 prescribes, so a Flat reading falls back to
  the top-level value. **Only the spelling is superseded**: integer seconds, the value the platform
  reported, never a formatted date.
- **`rate_limit_type`** — verbatim or `null`, as `0133` clause 3.
- **`item`** — the `"item"` entry of the dispatch's `ActivitySpec.inputs` when it is a JSON string,
  otherwise `null`. A member `0133` did not name. The binder is not changed and the key is not
  reserved; the adapter reads it and does not remove it from the prompt, where it already is.
  **This supersedes `0131`'s first forbids bullet in part** — for `rate_limit_reading.item` alone,
  and only until `0131`'s `item` tag is wired, at which point the member moves to the tag and this
  sentence lapses. `0131` gives **two** reasons for the bullet, and this record answers both rather
  than the convenient one. The *cheap* one — its considered options: *"`canonical_events` hashes the
  payload, so every golden log's events change, the corpus regenerates, `CORPUS_PIN` moves and three
  checkpoints re-derive — for a fact that is not part of any decision"* — is **not incurred**: no golden
  log contains a `rate_limit_reading` row, because none had ever been written. The *substantive* one —
  its driver, bold markup dropped: *"A tag is addressing metadata, not decision content. Two runs that
  decided identically must produce identical canonical events even when one was tagged into a
  different initiative."* — **is** incurred in principle: two readings identical in every measured
  member now differ by `item`. It is accepted as the interim cost because the tag cannot exist before
  Checkpoint 8, because a reading is tier 2 and records no decision, and because the kernel's own
  `dispatch` payload has carried `"item"` since `71b20c1` (2026-09-14), a week before `0131` was
  written, without `0131` removing it — so this is the second such member, not the first. Every other
  kind, and every other correlation key, stays under the bullet as written. The operator's decision of
  2026-09-24, taken when this stage's adversarial pass found the conflict.
- **`cli_version`** — the dispatch's `DispatchRecord.cli_version`, serialised exactly as that row
  serialises it. A member `0133` did not name, and owed by **AC-227** in terms: *"GIVEN any harness
  event written to the event store WHEN it is persisted THEN it carries the CLI version the dispatch
  actually ran against."* The reading row is a harness event; without the member it is the one row
  the adapter writes that breaks that criterion. Found by this stage's first verifier pass, before
  acceptance.

Exactly one of `windows` and `utilization` is non-`null`. No value in the payload is a float.

**4. What `cost` computes.**

- It reads **the whole journal** — the selector is inert, and a future store that honoured
  `ByItem` must not make foreign traffic vanish from the span — and takes `rate_limit_reading` rows in
  journal order.
- A reading is **owned** when its `item` is a string equal to `<ITEM>`. `null`, absent and
  non-string are **not** owned.
- **No owned reading is the empty case**, and it renders one line that says there is nothing to
  attribute and names none of the three renderings. It never prints `low_sample` over `n = 0`.
- The **span** runs from the item's first owned reading to its last, **inclusive**.
- **Every owned reading in the span is validated first**, before interleaving is counted: all six
  members present as keys, each of the type clause 3 gives it, and exactly one of `windows` and
  `utilization` non-`null`. An owned row that fails is refused with exit `3` — never skipped, which
  would shrink `n` silently, and never masked by a `not attributable` the rest of the span would
  have rendered. Foreign rows are not validated; they are only counted.
- **Interleaving is counted over traffic in the span**, not only over samples: each foreign or
  `null`-item kernel `dispatch` row counts once whether or not it produced a reading, and a foreign or
  `null`-item reading counts once unless it directly follows a foreign `dispatch` row with no
  `dispatch` or reading row between them, in which case the pair counts once. Any interleaving renders `not attributable` with the count. **The count is an indicator biased
  upward, not an exact tally**: when two processes interleave so that a foreign dispatch and its
  reading straddle an owned reading, both are counted, and the count can exceed the number of
  foreign dispatches by one per such straddle. It errs toward `not attributable` and never reads
  zero when a foreign row lies **inside** the span. A foreign dispatch whose `dispatch` row precedes
  the span and whose reading follows it — in flight across the whole span, from another process —
  leaves no row inside it and is **not** counted; that is the edge of *"this store's traffic"*, below.
- Otherwise, **per window**, a sample is formed from each owned reading that carries both that
  window's ticks and a non-`null` `resets_at`; a reading with ticks and a `null` `resets_at` splits
  the sequence and is never grouped under `0`. Each segment goes through `throttle::attribute`, and
  its draws are rendered with `n`, `resolution_limited` when set, `low_sample` with **no** delta when
  `n < 10`, and `non_monotonic` rather than a negative delta. `first` and `last` are never printed:
  their difference is a delta, and printing both would quote one at `n < 10` by subtraction.
- An owned flat reading whose `rate_limit_type` names neither window yields no sample in either —
  its utilisation is not written into both, which would be the extrapolation clause 3 refuses — and is
  counted on one `unwindowed=<N>` line rather than dropped, so it cannot shrink `n` unseen.
- Nothing is divided by anything. There is no per-run or per-dispatch figure (AC-373).

**The named limit**: "the only traffic" means the only traffic **this store saw**. Another
repository, or interactive `claude` use on the same account, draws on the same windows and is
invisible here. ADR-0110 decision 1's *"the account's complete dispatch sequence"* is the property
the rendering cannot check, and the output says *"this store's traffic only"* rather than let a reader
assume it can.

**5. What this record does not deliver, said here so that no stage infers it.** AC-395's
per-activity breakdown — token counters, `total_cost_usd`, wall clock, cache counters — is **not
rendered**: those live on `dispatch_completed`, which carries no item, so they cannot be attributed
without the same ownership key on a row `rk-types` owns, and `s7i` §7i.2 says *"nothing else"*. AC-372's
`status` renderer and its *"doctor rule that a flagged reading is never rendered bare"* are not
built. And `doctor`'s journal never comes from the store's event rows — `Ctx::build` reads
`events.jsonl` — so this producer does **not** make `RK-D-HEADROOM-MARGIN` fire on a shipped run
(E-0133-3); and when it does read real rows, a flat reading as the last row has `windows: null` and
silences the rule, whose reader is byte-compatible with the **unified** shape only. Both are
carried, not repaired.

**6. The dispatched case is proved over a fixture corpus, `0162`'s route.** A corpus under
`crates/rk-cli/tests/fixtures/cost/`, byte-pinned by `.gitattributes`, carries both a synthetic
`rate_limit_event` transcript that a real `redkiln run` dispatches through `ClaudeHarness`, and seeded
journals for the renderings a single run cannot reach (`n >= 10`, a reset crossing, interleaving).
**The shipped tree is left alone**, renders only the empty case, and Checkpoint 7i asserts that the
empty case is distinguishable there. Phase-1 criterion (7) therefore becomes **computable and
rendered over a fixture — not met on the shipped tree**; `s8`'s block still needs an `ITEM` that
owns readings.

## Consequences

`rate_limit_reading` has its first producer, `throttle::attribute` its first `src/` caller, and
ADR-0110's three renderings a verb. The batch key the renderings are grouped by is the value the
platform reported, and `n` counts what ADR-0110 says it counts.

The cost is a verb whose honest output on the shipped tree is *"nothing to attribute"*, and a
producer that no shipped run yet reaches. Neither is hidden: the empty line says why it is empty, and
this record says the fixture proves the wiring, not the catalog.

**What this forbids**

- Forbids any per-run or per-dispatch headroom figure, in `cost`'s output or anywhere else — no
  division by `n`, no `first`/`last` pair printed beside a withheld delta (AC-373; F6).
- Forbids rendering `low_sample`, `resolution_limited`, a delta or `not attributable` for an item that
  owns no reading — the vacuous green `s7i` §7i.2 and D21-5 both refuse.
- Forbids coercing a `null` `resets_at` to `0`, an epoch or "now", and forbids forming a delta across
  one.
- Forbids appending a reading per parsed frame, and forbids re-journalling the guard's previous
  reading on a pre-spawn path.
- Forbids a `rate_limit_reading` row without `cli_version` (AC-227).
- Forbids writing a Flat reading's utilisation into both windows, or into any window its
  `rate_limit_type` does not name.
- Forbids treating a reading with a `null` or absent `item` as owned by any item.
- Forbids skipping an owned reading that is malformed or missing a member, and forbids letting a
  `not attributable` rendering stand in for its refusal.
- Forbids `cost` appending to the store, dispatching, or reconciling.
- Forbids describing a `cost` rendering as attribution over the account's complete traffic; it is
  attribution over this store's traffic.
- Forbids claiming that Phase-1 criterion (7) is met on the shipped tree, that the shipped registry
  dispatches, or that `0 of 102` has moved.
- Forbids reading item ownership off `dispatch_completed` rows, which carry none, or off journal
  position.
- Forbids reading this record's `0131` supersession as licensing any other payload correlation key,
  on any other kind, and forbids keeping `rate_limit_reading.item` in the payload once `0131`'s
  `item` tag is wired.

## Evidence and confidence

**Every absence in `## Context and problem` was measured on 2026-09-23 at `76ae138`, tree clean.**
`grep -rn "RateLimitReading" crates/*/src --include=*.rs`, outside `rk-adapter-claude` and
`rk-types`, → the three kernel fold arms. `grep -rn "attribute(" crates/*/src --include=*.rs`
excluding its definition → **0**. `grep -rn "not attributable\|low_sample\|resolution_limited"
crates/rk-cli` → **0**. `grep -l chrono crates/*/Cargo.toml` → **none**. The store counts were read
through `sqlite3` with `?mode=ro`. The baseline was measured, not carried: Checkpoints 7h
(`FIX_ITEM=rk-fix-doctor-test-count`) and 7g each ran whole to their end markers, rc=0; suites
`rk-cli` **115**, `rk-kernel` **238** (5 ignored), `rk-adapter-claude` **102**, `redkiln-migrate`
**25**; `cargo fmt --all --check` rc=0; `cargo clippy --workspace --all-targets` **0 errors**.

**The decisions of clauses 2 and 3 are the operator's, taken 2026-09-23** in the session that opened
`s7i`, between the options above; clause 1's item source was chosen there over the reserved-key route
on the measurement that `verbs/run.rs` already binds `"item"`.

**What is NOT asserted.** That one reading per dispatch is always the *right* reading — it is the
stream's last representable one, which is the one the guard already acts on; a dispatch that never
emitted one contributes nothing to `n`, so `n` is a lower bound on the dispatches in the span, and is
never divided into anything. That the interleaving count is exact — a foreign dispatch refused before
spawn is counted though it drew nothing, and a straddling foreign dispatch and reading count twice
(decision 4), both of which err toward `not attributable`, the safe side. And
that any rendering here has been produced by real account traffic: the corpus is synthetic, as
`0162`'s was, and says so.
