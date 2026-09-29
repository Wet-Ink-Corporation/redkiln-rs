---
id: "0190"
title: "**`dispatch_completed` carries `activity_id_at_version` in its payload, `cost` breaks an item's owned rows down by activity, `status` renders draws through `cost`'s one renderer, and `doctor` reads the store's journal** (`s8f`'s telemetry-key record; D21-42 12b and 12c; D21-43 N5 (cited) and N12; D21-46 R3): the member is an interim payload key, a string or `null`, carried to the adapter on `ActivitySpec` outside the prompt and every digest; ownership is ADR-0182's `item` key beside the row; the breakdown lines join `cost`'s closed grammar; AC-372's doctor clause is read as a renderer test; `RK-D-HEADROOM-MARGIN` reads the latest reading that answers for `five_hour`, which deliberately differs from `cost`. Supersedes in part ADR-0131's first forbids bullet and four passages of ADR-0163, for this one member, until ADR-0131's tag is wired; and, TAKEN PROVISIONALLY under D21-107, ADR-0138 (its first forbids bullet's *\"or any sixth field\"*, decision 3's `ActivitySpec` half and decision 2's *\"NOT a sixth spec field\"* as applied to this id) and ADR-0174 (decision 1's parenthetical and decision 10's sentence) for this one member"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (12b — *"**ADR-0163's interim key is extended** to `dispatch_completed` (`item`, `activity_id_at_version`), moving to ADR-0131's tag when that wiring lands. The per-activity breakdown and the status half are built now."*; 12c — *"**Read from the store before any real run**; fixture mode keeps `events.jsonl`. The flat-reading silencing of `RK-D-HEADROOM-MARGIN` is fixed too."*), D21-43 (N5; N12 — *"**AC-372's** unfireable doctor rule is read, by dated note, as a test of `cost`'s renderer, with AC-415's red case."*) and D21-46 (R3 — *"**Carried outside the canonical digest**, as ADR-0131 places its tag set."*); runbook/phase-1/s8f-landing-and-rendering.md §8f.2 (items 4 and 5, measured 2026-09-28 at `37bba5c`), §8f.3's third row, §8f.8 and §8f.9; references/adr/0099 (decision 1), 0126, 0131 (its first forbids bullet), 0133 (clause 5), 0138 (its first forbids bullet), 0163 (decisions 4 and 5 and three forbids bullets) and 0182; crates/rk-types/src/activity.rs (`ActivitySpec`, `DispatchRecord`), crates/rk-adapter-claude/src/record.rs (`completed_event`), crates/rk-kernel/src/replay.rs (`spec_json`), crates/rk-kernel/src/workflow/memo.rs (`for_agent_step`), crates/rk-cli/src/verbs/cost.rs and crates/rk-cli/src/doctor/mod.rs, as read 2026-09-28 at `37bba5c`; spikes/DECISIONS-2026-09-21.md D21-107 (TAKEN PROVISIONALLY 2026-09-28, left open for the operator) and references/adr/0174 (decisions 1 and 10), added 2026-09-29 by package S8F-P7
supersedes: ["0131", "0138", "0163", "0174"]
superseded_by: []
related: ["0099", "0110", "0126", "0133", "0182", "0188", "0189"]
evidence: []
consumers: []
---

## Context and problem

**AC-395's breakdown and AC-372's `status` half were carried, not built.** ADR-0163 decision 5 reads: *"AC-395's
per-activity breakdown — token counters, `total_cost_usd`, wall clock, cache counters — is **not rendered**: those live
on `dispatch_completed`, which carries no item, so they cannot be attributed without the same ownership key on a row
`rk-types` owns, and `s7i` §7i.2 says *"nothing else"*. AC-372's `status` renderer and its *"doctor rule that a flagged
reading is never rendered bare"* are not built. And `doctor`'s journal never comes from the store's event rows —
`Ctx::build` reads `events.jsonl` — so this producer does **not** make `RK-D-HEADROOM-MARGIN` fire on a shipped run
(E-0133-3); and when it does read real rows, a flat reading as the last row has `windows: null` and silences the
rule"*.

**The item half is already `s8e`'s.** ADR-0182 carries N5's interim `item` key beside the event, outside `payload` and
outside `canonical_events`, and classifies `dispatch_completed` as `ItemKey::Carried`; it supersedes nothing of
ADR-0131, and this record cites it rather than re-specifying it. Its negative names what is still open here: *"every
row a harness appends through the store it is handed"* is written item-less.

**The adapter cannot see an activity id.** `ActivitySpec` is `card_digest`, `tools`, `schema`, `timeout_secs` and
`inputs`, and its doc reads *"ADR-0099:61's five fields and no sixth."* The `dispatch_completed` payload is the
`DispatchRecord` serialised (`record.rs`: `completed_event` is `serde_json::to_value(record)`).

**A binding record this stage's file did not name.** ADR-0138 is accepted, and its first forbids bullet reads
*"Forbids adding a step list, or any sixth field, to `rk_types::activity::ActivitySpec`. The spec is the Harness port's
payload and is ADR-0099:61's five fields."* Its superseding records, ADR-0141 and ADR-0174, withdraw other clauses and
leave this bullet whole. §8f.8 settles the carrier as *"a member on `ActivitySpec` that is outside the prompt and
outside every digest"*, which is a sixth field, and §8f.3 does not list ADR-0138 among this record's supersessions.
Decision 2 below takes that carrier **PROVISIONALLY**, pending the operator's ruling.

**Measured at stage open** (`s8f` §8f.2, 2026-09-28, at `37bba5c`, read-only):
- Item 4: the one golden log under `CORPUS_PIN` (`implement-bf1d8cec3b22d288`, committed as the same blob) holds **no**
  `dispatch_completed` row, so no golden log's canonical events move. Twenty-six `-text`-pinned fixture files do hold
  hand-authored `dispatch_completed` rows (`crates/rk-cli/tests/fixtures/cost/**` 23 including its README,
  `fixtures/doctor/RK-D-CREDENTIAL-RETAINED/**` 2, `fixtures/exit/README.md` 1). They are **inputs** to `cost`'s and
  `doctor`'s tests, not a producer's recorded output, and no test regenerates them or pins a producer's
  `dispatch_completed` bytes, so the member moves none of them. `s8e` answered the same question for the N5 key by
  carrying it outside the payload (ADR-0182 decision 6).
- Item 5: `ActivitySpec` derives no `Serialize`. Its serialised form is `RecordedSpec::spec_json` in `replay.rs`, an
  explicit five-member object written to a run record's `inputs/specs.json`; the pinned fixture
  `crates/rk-kernel/tests/fixtures/logs/implement-bf1d8cec3b22d288/inputs/specs.json` already carries
  `activity_id_at_version` per node as `RecordedSpec`'s own member. The memo key (`memo::…::for_agent_step`) reads
  `inputs`, `card_digest` and `schema` field by field, and `run_id` digests the stage's inputs. None of them serialises
  the spec whole.

## Decision drivers

- AC-395: per-activity token counters, cache counters, wall clock and `total_cost_usd` over the rows the item owns.
- ADR-0131: addressing is a tag, and a payload correlation key is an interim that must name its exit.
- ADR-0163's grounds, answered the same way: no golden log moves, the member is closed to one kind, and it moves to a
  tag when ADR-0131's wiring lands.
- ADR-0133 clause 5: sums *"**as decimals** with no intermediate binary floating-point step"*; AC-362: absent is
  never zero; AC-373: no per-dispatch headroom figure.
- AC-220: the margin rule and the guard must agree on which reading answers for `five_hour`.

## Considered options

### Option — a payload member, carried to the adapter on `ActivitySpec` outside the prompt and every digest (taken)

The adapter writes the row, as ADR-0126 requires (*"Forbids the kernel appending `dispatch_completed`; it is the
adapter's row"*), and the adapter is handed the value on the spec it already receives. The member is omitted from
`spec_json`, so no run record or pinned fixture moves. It costs a sixth `ActivitySpec` field, which ADR-0138's bullet
forbids; see decision 2.

### Option — an `inputs` key

Rejected on ADR-0163's own reasoning: it would *"move the prompt bytes of every existing binding"*, and with them
every card-and-inputs memo key.

### Option — pair each completion with the kernel's preceding `dispatch` row

Rejected: that is reading ownership off journal position, which ADR-0163's bullet keeps forbidden (decision 6), and a
second process's row can interleave.

### Option — a kernel-side store decorator that stamps the member on the adapter's row as it is appended

Weighed. It needs no `ActivitySpec` field and no port change, but the row would no longer be the adapter's own record
(ADR-0126's bullet above), and the member would be written by code that never saw the dispatch.

### Option — widen `Harness::dispatch` with a context argument

Rejected here: a port-signature change moves Checkpoint 7a's port surface and ADR-0099 decision 1's signature, for one
string.

## Decision

**1. The member.** `dispatch_completed`'s payload carries `activity_id_at_version`: the dispatching activity's
`<id>@<version>` string, or `null` when the spec carries none (a fixture's spec). It is a `DispatchRecord` member,
`Option<String>`, serialised on every row (the key is always present, `null` for `None`), and a row read back without
the key deserialises to `None`. It is an **interim** key: when ADR-0131's tag set is wired it moves to a tag, and the
sentences that allow it lapse, on ADR-0163's own clause. `M16`'s rule holds: a value without its `@version` is never
written.

**2. The carrier — `ActivitySpec.activity_id_at_version: Option<String>`, TAKEN PROVISIONALLY.** It is set where the
spec is built (`rk-cli`'s `bind_nodes`, and each per-step spec on `RecordedSpec` carries its own activity's), and it
is outside the prompt (the adapter's prompt assembly never reads it), outside the card digest, outside the memo key
(which reads `inputs`, `card_digest` and `schema` field by field) and outside `run_id`. **`spec_json` does not write
it**: a run record already records `activity_id_at_version` as `RecordedSpec`'s own member, and `spec_from_json`
repopulates the field from that member, so no run record, `specs.json` or pinned fixture changes a byte (§8f.2 item
5). **This is a sixth field**, so this record supersedes in part ADR-0138's first forbids bullet, only its *"or any
sixth field"* and only for this one member, and reads ADR-0099 decision 1's *"`ActivitySpec` carries `StepSpec`'s
fields (card digest, tools, schema id, timeout, inputs)"* as naming what the spec carries from `StepSpec`, not as a
closed list; ADR-0138's bullet is the closure. ADR-0138's ground was *"`rk-types` is Checkpoint 5's and is not edited in
this range"*; `s8e` edited `rk-types` and re-ran Checkpoint 5 whole (D21-97, (10i)). The step list stays off the spec,
and every other clause of ADR-0138 stands. ~~**This decision waits on the operator's ruling** on the blocker this record
reports; if another option is ruled, this decision is rewritten before the record flips and `0138` leaves
`supersedes`.~~

*(dated 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; D21-107 item 2.)* **Taken
PROVISIONALLY under D21-107** (the AFK rule D21-105; weigh-in wi-838696, left OPEN for the operator). The member is set
**explicitly at every construction site** — no `Default` exists and no `..` spread builds a spec from nothing:
`Some("<id>@<version>")` from `rk-cli`'s `bind_nodes` (through `BindRequest.activity_id_at_version`, which the binder
copies unchanged and reads nothing from) for the activation's spec and each per-step spec, `None` in fixtures and
fakes. `spec_json` stays its five-member object; `spec_from_json` repopulates the member from the binding's own
recorded `activity_id_at_version`, and only when that carries its `@version` (an unversioned activation's spec reads
back `None`, exactly as `bind_nodes` sets it). The member is kept out of the prompt, the card digest, the memo key and
`run_id`; the adapter alone writes it, on its own `dispatch_completed` row (ADR-0126 cited, untouched). The
supersessions this takes are decision 9's last two bullets. **ADR-0099 decision 1 is carried as a DECLARED READING,
flagged for the operator's ruling:** it names what the spec carries FROM `StepSpec` and is not a closed list; if the
operator declines that reading, it converts to a supersession in part of ADR-0099 decision 1 for this one member.
**If the operator declines D21-107**, the fallback is its option B — a kernel-side `EventStore` decorator that stamps
the member onto the adapter's rows, with its ADR-0126 tension recorded in a record — and this decision is rewritten,
and `0138` and `0174` leave `supersedes`, before the record flips. This record does not flip at `s8f`'s close while
D21-107 is unconfirmed (D21-49's flip-stop).

**3. Ownership is ADR-0182's key.** The adapter keys its `dispatch_completed` row with `Event::with_item`, taking the
value as the reading's `item` is taken, from `ActivitySpec.inputs`' `"item"` (`ITEM_KEY`, *"Read for attribution
only"*). A row is **owned** by `<ITEM>` when that key equals `<ITEM>`; a row whose key is `null` or absent is owned by
no item (ADR-0163's *"Forbids treating a reading with a `null` or absent `item` as owned by any item"*, applied to this
row). The payload is never read for ownership, and journal position never.

**4. The breakdown (AC-395), and the grammar widened.** After `cost <ITEM>`'s existing lines, when the item owns at
least one `dispatch_completed` row, `cost` prints:
- 9. `breakdown <ITEM>: dispatch_completed rows owned: <k>`, `k >= 1`, the header;
- 10. one line per activity, `activity <A>: dispatches=<N> input_tokens=<S> output_tokens=<S>
  cache_read_input_tokens=<S> cache_creation_input_tokens=<S> wall_ms=<W> total_cost_usd=<S>`, ordered bytewise by
  `<A>`, where `<A>` is an `<id>@<version>` or `(none)` for owned rows whose member is `null`, printed last.

`<S>` is `<sum>` when every one of the group's `N` rows carries the counter, `<sum>(absent=<k>)` when `0 < k < N` do
not, and `absent` when none does: **an absent counter is never zero** (AC-362), and a group names how many were
absent. `total_cost_usd` is summed **as decimal strings**, digit by digit, with no binary float anywhere (ADR-0133
clause 5); `<W>` is the integer sum of `timings.wall_ms`, which every row carries. No line carries a headroom figure
(AC-373). An item owning neither a reading nor a `dispatch_completed` row prints exactly ADR-0163's empty line and
nothing else; an item owning rows but no reading prints the empty line and then the breakdown. `cost_output_grammar_is_closed`
is amended to hold forms 9 and 10 beside ADR-0163's eight.

**5. `status` (AC-372's `status` half).** `status` prints an item's draw lines, every flag included, through the one
rendering function `cost` uses, which lives in `crates/rk-cli/src/verbs/cost.rs` and has no second home; `status.rs`
calls it. A flag therefore cannot be kept on one surface and dropped on the other. An item with no reading gets no
headroom line, so `status` on the shipped tree (zero readings) does not move.

**6. AC-372's doctor clause is read as a renderer test (N12).** A rendering is not journalled, so a `doctor` rule over
renderings has no input and could never fire. The clause is built as
`cost::a_flagged_reading_is_never_rendered_bare_and_a_bare_rendering_is_rejected`, which drives the shared renderer
over every flag-bearing vector and asserts each flag, with AC-415's red case (a bare rendering of a flagged reading,
which the same check rejects). No `doctor` rule id is added, so AC-415's rule-table join is unchanged.

**7. `doctor`'s journal.** `Ctx::build` stays the one builder and gains a journal-source parameter: the verb passes
the **store's rows**, the whole journal, with the selector inert; a fixture test may name `events.jsonl`; the verb
**never** names the fixture source. The false field doc (*"Read from the STORE in the shipped binary"*) becomes true
in the same change.

**8. The five-hour reading.** `RK-D-HEADROOM-MARGIN` reads the latest reading that **answers for `five_hour`**, exactly
as the guard's `RateLimitReading::ticks(Window::FiveHour)` does: a unified reading's window, and a flat reading whose
`rate_limit_type` is `None` or names `five_hour`. A flat reading naming `seven_day` is skipped, so it no longer
silences the rule. **This deliberately differs from `cost`**, which counts a flat reading with no named window as
`unwindowed` and forms no sample, because attribution never extrapolates; the margin rule shows the guard's own
margin, and AC-220's two halves must agree. Nobody "unifies" the two.

**9. What this record supersedes, each in part, for `dispatch_completed.activity_id_at_version` alone and until
ADR-0131's tag is wired:**
- **ADR-0131's first forbids bullet**, *"Forbids putting an item id, a run id, a commit SHA, a stage, a lane, a node, a
  gate token, an AC id, a worktree or any other correlation key **inside** `rk-types::Event.payload` for the purpose of
  addressing."* The `item` key is not in the payload (ADR-0182) and is not this supersession.
- **ADR-0163 decision 4's closed-grammar sentence**, *"Every line `cost` prints is one of these eight forms, and nothing
  else"*: it now reads ten forms (decision 4). The eight stand as written.
- **ADR-0163 decision 5's not-delivered list** (the breakdown, the `status` renderer and its doctor clause) and its
  sentence *"And `doctor`'s journal never comes from the store's event rows"*: each is delivered (decisions 4–7).
- **ADR-0163's bullet** *"Forbids reading this record's `0131` supersession as licensing any other payload correlation
  key, on any other kind"*, for this one member; the bullet's second half, on `rate_limit_reading.item`, stands.
- **ADR-0163's bullet** *"Forbids reading item ownership off `dispatch_completed` rows, which carry none, or off journal
  position"*, **for its first half only**: the breakdown reads a row's owner from the `item` key ADR-0182 carries
  beside it, so its ground, *"which carry none"*, is gone for a keyed row. Reading item ownership off
  `dispatch_completed` rows is admitted only through that key; the **journal-position** half stands whole.
- ~~**ADR-0138's first forbids bullet**, *"or any sixth field"*, for this one member (decision 2, TAKEN PROVISIONALLY).~~
  *(dated 2026-09-29, s8f build, package S8F-P7; D21-107 item 2 widens this bullet into the five below, each TAKEN
  PROVISIONALLY and each for `ActivitySpec.activity_id_at_version` alone.)*
- **ADR-0138's first forbids bullet**, only its *"or any sixth field"*: *"Forbids adding a step list, or any sixth
  field, to `rk_types::activity::ActivitySpec`."* The step list stays forbidden.
- **ADR-0138 decision 3's `ActivitySpec` half**: *"`ActivitySpec` is not edited, and the catalog is not loaded from
  `runtime/`."* The catalog half stands whole. *(Corrected 2026-09-29, s8f build (R1FIX-8f-B-RECORDS), refuting round
  1: this quotation bolded only its first clause, where ADR-0138 bolds the whole sentence; it is now given without
  emphasis, word for word. No word moved.)*
- **ADR-0138 decision 2's *"NOT a sixth spec field"***, as applied to this id: *"that it is **"NOT a sixth spec
  field"**"* — `activity_id_at_version` is now ALSO a spec member, beside the binding's own recorded one, which stays.
- **ADR-0174 decision 1's parenthetical**: *"`ActivitySpec` is not edited (ADR-0099's five fields; ADR-0138's
  forbids)."* The per-step specs stand; each carries its own activity's member.
- **ADR-0174 decision 10's sentence**: *"ADR-0138's decisions 1–5 and its forbids stand"*, for the three ADR-0138
  passages above only.

**Cited, not superseded:** ADR-0182 (the `item` key and its inclusive `ByItem` interim), ADR-0126 (the row is the
adapter's), ADR-0133 clause 5, and every other decision and bullet of ADR-0131 and ADR-0163.

## Consequences

**Positive**
- `cost` attributes tokens, cache, wall clock and cost to the activities that spent them, for the item that owns them.
- `status` and `cost` cannot disagree about a flag, because they share one renderer.
- `doctor` reads what a real run wrote, and the margin rule is no longer silenced by a flat `seven_day` reading.

**Negative**
- A second interim payload key exists until ADR-0131's wiring lands, as ADR-0163's did.
- `ActivitySpec` grows a sixth field, which ADR-0138 had closed.
- Rows written before this record carry no `activity_id_at_version`; they group under `(none)` when owned, and rows a
  harness appended unkeyed are owned by no item (ADR-0182's residual).
- A `dispatch_completed` row whose store append failed is counted on the adapter's warning counter and is absent
  from the breakdown.

**What this forbids**
- Reading a `dispatch_completed` row's owner from anything but ADR-0182's `item` key, and reading it off journal
  position.
- Summing `total_cost_usd` through a binary float, and printing an absent counter as `0`.
- A breakdown line for an item that owns no `dispatch_completed` row, and any line outside `cost`'s ten forms.
- A second draw renderer for `status`.
- The `doctor` verb naming the `events.jsonl` fixture source.
- `RK-D-HEADROOM-MARGIN` reading the last row regardless of window, and unifying its five-hour reading with `cost`'s.
- Writing `activity_id_at_version` into `spec_json`, the prompt, the card digest, the memo key or `run_id`'s inputs,
  and writing it without its `@version`.
- Reading this record as licensing any other payload correlation key, on this or any other kind.

## Evidence and confidence

**Measured** (2026-09-28, at `37bba5c`, `s8f` §8f.2): items 4 and 5 as quoted in the context; ADR-0138's bullet read
in full, with its `superseded_by` (`0141`, `0174`) leaving it whole. **Documented:** the ledger lines and records
quoted. **Not measured:** the breakdown over a real run's rows, which waits on `s8j`. This record is `proposed` and
flips only on the operator's dated ledger line.

## As built *(dated 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; this record stays `proposed`)*

- **Decisions 1–3:** `DispatchRecord.activity_id_at_version` (serialised on every row; an older row reads back `None`)
  and `RecordTail`'s member; the adapter copies the spec's member on every termination path and keys its row with the
  spec's `item` input through `Event::with_item` (`record::completed_event(record, item)`). Tests:
  `dispatch::dispatch_completed_carries_activity_id_at_version_or_null` (with the card-digest half),
  `recorded_log::the_activity_member_moves_no_specs_json_byte_no_run_id_and_no_memo_key`,
  `recorded_log::the_activity_member_is_repopulated_from_the_recorded_binding_on_read`,
  `cost::a_real_run_writes_its_activitys_id_at_version_on_dispatch_completed`.
- **Decision 4:** `cost::breakdown` and `cost::render`; `cost_output_grammar_is_closed` amended to ten forms.
- **Decisions 5 and 6:** `cost::draw_line` is the one draw renderer; `status` prints `cost::draw_lines`.
- **Decision 7:** `doctor::JournalSource`; the verb passes `JournalSource::Store`.
- **Decision 8:** `RK-D-HEADROOM-MARGIN` reads `five_hour_ticks`, which rebuilds the adapter's `RateLimitReading` and
  calls `ticks(Window::FiveHour)`.
- **Measured residual, for the operator:** `RK-D-UNREAD` now reads real rows. Its kind-per-stage predicate (D-08) keys a
  write and a read by each row's OWN stage, so an artifact emitted at one stage and read only at a later one (the
  fixture lane's `ImplReport`, written at `implement` and read at `review`) is counted as unread at the writing stage.
  `doctor_journal::unread_counts_every_artifact_row_a_fake_harness_lane_writes` reads the emitted kind back at the
  stage that wrote it, which is the case §8f.5 names as silent. Whether cross-stage consumption should count as a read
  at the writing stage is not settled by this record. *(Closed 2026-09-29, s8f build, package R1FIX-8f-A-CODE, by
  D21-111, taken provisionally under D21-105: the rule's read side is the set of `kind` over every `artifact.read`
  row, whatever its stage or run; the write side is unchanged. The lane test's `check-report` workaround is dropped and
  it asserts that `ImplReport`, read at `review`, is not flagged; `unread/cross-stage-51.jsonl` and
  `doctor_journal::unread_reads_a_cross_stage_read_as_consuming_the_write` pin it. Item and journal-order matching
  stay D21-111's named residual.)*
