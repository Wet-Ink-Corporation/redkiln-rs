---
id: "0124"
title: The event-kind table gains the review and verify gates' five kinds — review_block, quarantine_granted, quarantine_removed, quarantine_expired, selection_ineffective — each named by a criterion or a measure and minted by nothing
status: accepted
date: 2026-09-17
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-307, AC-304, AC-306, AC-326, AC-310, AC-530, AC-313, **cited by id and named clause** (the register's line numbers moved +39 on 2026-09-13; `s7b-ii`:63-70, ADR-0119/0120/0123 take the same route; AC-306's THEN reads at :4888 and AC-326's AND at :5173 on this tree, `M` against `900ce5e`); references/2026-09-08-success-measures.md :632, :640, :649 (`review_block` as an Instrument, three measures), :708 (`quarantine_granted` / `quarantine_expired` as an Instrument), :740 (`selection_ineffective` as an Instrument); references/2026-09-08-architecture-design.md §5 :699 (the closed-enum sentence) and :701-724 (the table — `review_class_drift` at :718 and the advisory rows at :722 are the review gate's only rows today); references/adr/0074 clause 6 (`Quarantined { reason, by, until }`) and clause 8 (*"Every block and every override is an event"*); references/adr/0082 (the advisory sink; decision 3, *"Both paths emit an event"*); references/adr/0104 decision 3 (the bootstrap comparator that emits `selection_ineffective`); references/adr/0116 :40-41 and :95-99 (the minting rule and the rejected runbook option); references/adr/0120 and references/adr/0123 (the two precedents this record follows clause for clause); references/adr/0105 clause 7 (`rk-types` owns the enum); references/adr/0109 decision 2 (the deleted-database test); crates/rk-types/src/event_kind.rs and crates/rk-types/tests/it/event_kinds.rs at `900ce5e` (31 variants); runbook/phase-1/s7b-ii-gates.md §7b-ii.5–.9 and §7b-ii.11; runbook/00-preamble.md :55; the operator's standing decision of 2026-09-17 in the Step 7b-ii session that a kind a criterion names and no table defines is minted by record (ADR-0123's question, answered once; this record applies the answer to the next five and is their primary record)
supersedes: []
superseded_by: []
related: ["0074", "0082", "0104", "0105", "0109", "0116", "0120", "0123"]
evidence: ["F29", "F31", "F18", "F19"]
consumers: []
---

## Context and problem

**Five events the review and verify gates must write have no kind in the closed table.**

| the event | who names it | the kind's name, as named |
| --- | --- | --- |
| *"any block and any override (quarantine grant or removal) … an event is appended carrying `{ finding_id, ac, computed_class, model_label, stage_weight, sha, re_execution_id, actor, adjudication: null }`, and no block or override exists without one"* | **AC-307**; ADR-0074 clause 8 | `review_block` (SM :632, :640, :649 — three measures name it as their Instrument); the two overrides have no name in any register beyond *"quarantine grant or removal"* |
| *"a quarantine granted … "* | **SM** :708 (Instrument: *"`quarantine_granted` / `quarantine_expired` events"*) | `quarantine_granted` |
| *"THEN the finding is re-raised and a `quarantine_expired` event is emitted"* | **AC-306** | `quarantine_expired` |
| *"AND a `selection_ineffective` event is emitted"* | **AC-326**; ADR-0104 decision 3 | `selection_ineffective` |

`git grep -n 'review_block\|quarantine_granted\|quarantine_removed\|quarantine_expired\|selection_ineffective'` over `references/`, `runbook/` and `crates/` returns hits in the AC register, the SM register, the audit dossier's quotations of them and `s7b-ii`'s own text — **and none in design §5's table and none in `crates/`** [measured 2026-09-17]. `EventKind` carries **31** variants (`ALL: [Self; 31]`) [measured]. Design §5:699 is the constraint: *"the `EventKind` enum is closed, a kind not in this table is a schema change reviewed as one."*

**No existing kind carries any of them without deciding something else.** `review_class_drift` records a class disagreement, not a block; `advisory_created` records intake to the sink, which a block is the opposite of; `step_refused`'s `RefusalClass` is closed (ADR-0095) and a block is not a step refusal; `gate_answer_void` and `gate_answer_spent` are the human gate's. A block carried under any of them would be AC-307's replay invariant `count(blocks) == count(block_events)` counting the wrong rows.

**ADR-0123 asked this question once and the operator answered it**: a kind a criterion names and no table defines is minted by record, on ADR-0120's precedent, never in the runbook or as a kernel constant. This record applies that answer to the next five kinds together, because three separate records for one shape would be the bookkeeping ADR-0116 exists to avoid, and because the quarantine trio is one lifecycle.

## Decision drivers

- A criterion naming an event kind no table defines cannot be built (ADR-0116's driver, fourth application).
- ADR-0074 clause 8: *"Every block and every override is an event"*, and the block event is what `false_block_rate` is computed over (AC-308) — *against adjudication, never against historical approval*. Without a block row there is no denominator to adjudicate.
- AC-307's payload is the register's and carries `adjudication: null` on write: the row is the thing a later adjudication attaches to, so it must exist before any adjudication can.
- A quarantine is a bounded override (ADR-0082; AC-319 makes `until` non-optional); its grant, removal and expiry are three facts the sink's lifetime invariant `live_sink == created − retired − dismissed` (AC-310, AC-530) reads beside the advisory rows.
- `selection_ineffective` is ADR-0104 decision 3's own emission: the bootstrap comparator's finding that an `affected` claim executed the whole suite, reported as reclassification and never as a pass.

## Decision

**1. Five tier-2 event kinds**, with their rows in design §5's table:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `review_block` | 2 | `finding_id, ac, computed_class, model_label, stage_weight, sha, re_execution_id, actor, adjudication: null` (AC-307's, verbatim) | the review gate's confirm half, once per finding whose decision is `Block` |
| `quarantine_granted` | 2 | AC-307's members, plus `reason, by, until` (ADR-0074 clause 6's record) | a human override moving a finding to `Quarantined` |
| `quarantine_removed` | 2 | AC-307's members | a human override lifting a quarantine before `until` |
| `quarantine_expired` | 2 | `finding_id, ac, until, review_id` | the next review after `until` has passed, when the finding is re-raised (AC-306) |
| `selection_ineffective` | 2 | `run_id, toolchain, reason: selection-ineffective \| probe-unverified, executed, recorded_whole_suite` | the verify gate's bootstrap comparator (AC-326; ADR-0104 decision 3) |

**The payloads are the registers' and are taken as given** where the register gives them (AC-307, AC-306's re-raise, AC-326's two reasons); this record fixes the names and their obligation, not their Rust types (ADR-0120 decision 1's posture).

**2. Tier 2, and the deleted-database test is why** (ADR-0109 decision 2). A block is *derived* — `policy::classify` over the class, the weight and the re-execution outcome, all recomputable from the recorded `Findings` payload and re-execution record (AC-313's k=10 replay is exactly that recomputation); losing the row loses the count of blocks, never the block. A quarantine's grant and removal are human dispositions whose *effect* on the next action is carried by the finding's persisted state (`rk_types::review::Quarantine`), and its expiry is a comparison of `until` against a journaled instant. `selection_ineffective` is a reclassification the verify report itself carries (`basis`, `basis_reason`). None is tier 1; none carries a trailer; no fourth trailer axis is minted.

**3. `review_block` is written by the same code path that decides the block, and nowhere else** (AC-307's architecture lint: *"the block path and the event append are the same code path"*). A block set computed without its rows, or rows appended from a second site, is the replay invariant failing in one direction or the other.

**4. The table rows and the `rk-types` variants land in the same change** (ADR-0116 clause 5; ADR-0120 decision 6; ADR-0123 decision 4). `EventKind` gains five variants; `ALL` goes `[Self; 31]` → `[Self; 36]`; `wire_name` and `tier` gain arms; the pin test gains a case. **The golden schema snapshot does not move** (`EventKind` is in neither snapshotted set; re-measured at ADR-0123).

**5. The consequence for Checkpoint 5 is taken, not routed around**: the row goes `passed c25588a, re-run pending 7b-ii rk-types` in the commit that lands the variants, and the hardened block is re-run before that commit.

**6. What this record does NOT decide**: the adjudication record's shape (AC-308 — PROVISIONAL, E3-SEV-BLIND unrun); who may grant or remove a quarantine (AC-309 — HUMAN-GATED); whether an advisory finding may be quarantined (it may not — ADR-0082 clause 4, AC-304's negative fixture — but that is a rule the gate enforces, not a kind); and the `doctor` rows over any of these (7d).

## Considered options

### Option — carry blocks and overrides under `review_class_drift` or the advisory rows
Rejected: each records a different fact, and AC-307's `count(blocks) == count(block_events)` and AC-310's `findings_in == blocks + sink_records` are counts over kinds — a block filed as drift or as sink intake makes one invariant over-count and the other under-count.

### Option — one kind, `review_event { kind: block | granted | removed | expired }`
Rejected as ADR-0123 rejected the same shape: it moves the closed set into a payload string, gives four facts with four producers one row, and makes the fold discriminate on a string.

### Option — mint each kind in the slice that first needs it, three records
Rejected: the same decision three times. The quarantine trio is one lifecycle and `selection_ineffective` is one criterion's AND clause; a record per kind would be the bookkeeping the minting rule exists to bound, not the review it exists to force.

### Option — mint in the runbook or as a kernel `const`
Rejected on mechanism and by name (`00-preamble.md`:55; ADR-0116:95-99; ADR-0120, ADR-0123).

## Consequences

**Positive**
- AC-307, AC-306 and AC-326 become implementable exactly as worded; AC-310/AC-530's lifetime invariant has every row it reads; AC-308's denominator exists to be adjudicated.
- Three success measures (SM :632, :640, :649) and the quarantine measure (:708) acquire the events they already name as their instrument.

**Negative**
- A closed enum moves for the fourth time in five days, by five. The hand-written pin moves; the count in every kernel doc that says *"closed at 31"* is stale again and is corrected in place, dated, in the same commit.
- Checkpoint 5 becomes provisional and is re-run (decision 5) for the third time in one day. That is the maintenance rule working.

**What this forbids**
- Forbids a block decided without a `review_block` row, or a `review_block` row appended anywhere but the path that decided the block.
- Forbids a quarantine grant, removal or expiry that writes no row (ADR-0082 decision 3: *"Both paths emit an event"*).
- Forbids carrying any of the five at tier 1, on a trailer, or under another kind.
- Forbids `adjudication` on a `review_block` row being anything but `null` at write; adjudication is a later, separate record (AC-308).
- Forbids landing the variants without the design §5 rows, the `ALL` count, the pin test and the Checkpoint 5 row in one commit.

## Evidence and confidence

- [measured 2026-09-17, `900ce5e`] the grep above: every one of the five names appears in a register (AC-306, AC-326, SM :632/:640/:649/:708/:740) or in ADR-0074 clause 8's prose; none in design §5's table; none in `crates/`. `event_kind.rs` at 31 variants.
- [documented] AC-307's payload; AC-306's THEN; AC-326's AND and ADR-0104 decision 3; ADR-0074 clauses 6 and 8; ADR-0082 decisions 3 and 4.
- [measured, F29 / F31] the block event's reason to exist: a reproduced regression approved on 3/10 draws, 14/14 adjudicated findings confirmed by re-execution — the block is the machine half's word, and it needs a row that an adjudication can later attach to.
- [measured, F18 / F19] why a quarantine needs an expiry row: a gate outlived its assumptions by 9.4 days; drift on 12/174.
- **Confidence:** the gaps are measured; the names and payloads are the registers'. The tier is *reasoned* from ADR-0109's test, as ADR-0116, 0120 and 0123 reasoned it for theirs, and is not measured.
