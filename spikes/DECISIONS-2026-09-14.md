# Decisions taken at Step 7b Slice 4, 2026-09-14 (D-1…D-14)

Same shape as `DECISIONS-2026-09-12.md`. Every entry was taken **by the operator** (Ryan Britton), one at a time,
with the grounding and the enumerated options in front of him, **before** the code or record that depends on it was
written. Grounding is a document clause or a measured number, never taste.

**Numbering.** These carry **`D`-ids**, not the `#1`–`#27` sequence of `DECISIONS-2026-09-11.md` nor the `E`-ids of
`DECISIONS-2026-09-12.md`: a different pass with its own id space. Cite them as `DECISIONS-2026-09-14 D-<n>`.

**Where they came from.** Step 7b Slice 4 — the kernel runtime — could not be built without them. Each was a
question the stage was **not permitted to decide for itself** (`00-preamble.md`:55: *"Where a stage is tempted to
decide something else, it stops."*), surfaced by reading the governing records rather than by writing code, and
put to the operator with the measurement that made it decidable.

**A note on ADR-0122.** It was written on 2026-09-14 before this file existed, and its `source:` key says in terms
*"there is no `spikes/DECISIONS-2026-09-14.md` and this record is that decision's primary record."* That remains
true of **D-1**: ADR-0122 is C1's primary record and this log points at it rather than superseding it. The
sentence in its source key is now stale by one clause and is the sort of thing a dated correction handles; it is
**named here and not written**, because ADR-0122 is a frozen record.

---

## The seven that are executed in code, landed 2026-09-14/15

| # | decision | choice | grounding | landed |
| --- | --- | --- | --- | --- |
| **D-4** | The golden-log layout is stale and the version string no longer identifies the shape | **Amend §7b.11 to the current members AND bump `SCHEMA` to `v2`, before the pin move** | `run_id = sha256_lp([SCHEMA, stage, canonical(inputs)])` — the schema string is an **input to the run id**, so a bump renames every run. The corpus held **no** `golden-logs/` [measured 2026-09-14], so the bump cost one fixture re-bless; after the move it would invalidate corpus-committed run ids under a SHA **AC-464** freezes and `PROGRESS.md`:103 forbids moving twice | `0780198` (members), `b37fcbb` (`SCHEMA`), `f4cd32a` (`v2` in the runbook). The fixture moved `implement-a539b5a7b654d16f` → `implement-bf1d8cec3b22d288`, which is the **assertion** that the bump reached the derivation |
| **D-5** | `gate_opened → gate_expired → gate_answered` made a journal permanently unfoldable, and the refusal accused the operator of forging a token | **Split the arm three ways; void the expired case; decide nothing about re-opening** | The code **conflated two situations in one arm** and only one is forgery. `EventKind::GateAnswerVoid` already exists and `apply` already has an arm for it, so nothing is minted. AC-176 voids a *tree_hash*-drifted answer **and re-opens**, but its Boundary note assigns gate-record semantics to the gates area — so re-opening stays **7b-ii's** | `b37fcbb` |
| **D-6** | `exhausted_counter` used one comparison for all three counters and no record fixes one | **`>` for attempts and tokens, `>=` for the wall clock**, each following its criterion's own verb | AC-024: *"a run that **reaches** its wall-clock budget"*; AC-025: *"an activity that **ran out** of any budget"*. **The defect was live**: 9 of 102 shipped activities declare exactly their ADR-0037 rule-8 minimum, so under a uniform `>=` each was exhausted by its own **successful** pass and `evaluate_gate` could never close a gate on its evidence | `b37fcbb`. A dated note fixing the comparison per counter is **owed** |
| **D-7** | The breaker is keyed on *model family*, which exists nowhere in the type system | **A pack-declared role → family table; the kernel resolves at plan time** | The only route consistent with records already written: the pack owns roles and tier policy (design :480-481, ADR-0014) and `K` is **already** a pack value asserted by pack validation (AC-033's own *Verified by*). A sixth `ActivitySpec` field is refused by *"ADR-0099:61's five fields and no sixth"*; keying on the **role** is a departure on the merits (N roles → one model, so the breaker would stop one role's share of traffic to a failing model); moving it to the adapter needs an `rk-types` change anyway, since `StepError` (11) and `RefusalClass` (4) are both closed and neither has a breaker member | `b37fcbb`. `catalog.rs` now parses `role`, which the shipped catalog carries **185 times in 8 values** and which nothing read. The **shipped** pack's correctness cross-lists to **7e** |
| **D-8** | Four criteria need a process boundary a library test cannot provide | **A re-exec harness for AC-035 and AC-024; AC-013 and AC-030 route to 7d** | **ADR-0087 clause 2**: *"Resume must cross a real process boundary. A reload that reads state back in-process is not a valid implementation of this decision and **is not evidence for it**."* So the in-process version was not weaker evidence — it was **not evidence**. AC-013 and AC-030 name `redkiln run`, which is the CLI's | `b37fcbb`. AC-013 stays **PROVISIONAL on P8**: built, not claimed |
| **D-9** | Is `JitterDraw::from_key` a proposal owing a record, or is it sanctioned? | **FORCED — sanctioned; no record owed for the source** | **AC-002's own closing sentence**: *"Ids the workflow needs are derived from the idempotency key `(item, activity_id@version, attempt)` (ADR-0097) **or from a content hash**."* Its WHEN bans drawing *"from a **non-deterministic** source"* — not determinism — and AC-029 **requires** a seeded draw | `b37fcbb`. The separate note that `HARNESS_ALREADY_RETRIED` is this crate's proposed class token **stands** and is still owed |
| **D-10** | Memo-key component 8 is *"the output type's `schema_version`"*, and no such field exists | **The whole `SchemaId` string** | `grep -rn schema_version crates/rk-types/src crates/rk-ports/src` → **0** [measured 2026-09-14]. The id is the only value carrying a version and AC-014's property holds. Parsing the `vN` suffix was refused: it invents a convention no record states, has no defined behaviour on an id without one, and makes `RecordedResult/v1` and `ChangeSummary/v1` **collide** in component 8 | `b37fcbb`. A dated note saying what `schema_version` formally is remains **owed** |

## The two that are build-scope, executed 2026-09-14/15

| # | decision | choice | grounding | landed |
| --- | --- | --- | --- | --- |
| **D-2** | AC-498's *"CI job with network egress denied"* has no job at 7b | **Build all three halves; deny egress with `--network none`** | `--network none` is a Docker primitive, so **no marketplace action** enters a repo gated on `cargo-deny` and `cargo-audit`. The two in-tree halves were measurable already: `rk-kernel`'s resolved graph carries **0** networking crates | `7437a26`. One new job; `gates` and `structural` untouched. A warm step keeps it from becoming a permanently-red gate — the shape `deny.toml`'s own header warns of: *"which is how a gate gets deleted rather than fixed"* |
| **D-11** | `ActivitySpec` assembly is the binder's, and no stage builds the binder | **7c — mechanism and fixture cards; shipped cards cross-list to 7e** | The binder is `rk-kernel`'s by the register's §0 tag map (`rk-packs` → a module of `rk-kernel`) and its criteria sit inside §7b.15's claimed AC-080–AC-118 range, yet `PROGRESS.md`:279 records that **no stage authors an agent card or builds the binder**. 7e was refused **on ordering** — execution runs `7b → 7b-ii → 7c → 7d → 7e`, so a binder at 7e leaves 7c's adapters and 7d's CLI with no bound card | Scope only; **7c's to build**. At 7b the spec stays a **recorded input** (`inputs/specs.json`), which keeps AC-042's *"re-derives from the logged inputs alone"* true of a run that dispatches |

## The five that are record work above the runbook — OWED, not executed here

Each is drafted with its proposed text and its grounding; none is minted, because each edits the AC register,
`references/adr/ERRATA.md` or the design, which `00-preamble.md`:6-10 puts above the runbook in ADR-0092's chain.

| # | decision | choice | grounding | what is owed |
| --- | --- | --- | --- | --- |
| **D-1** | Register §0.3's **C1**: do harness-internal `system/api_retry` attempts charge the activity's attempt budget? | **AC-026 / AC-232 — they charge**, one charge per recorded event | The register recorded the distribution and refused the decision (*"The decision is the operator's"*), and no ADR, DECISIONS entry or erratum settled it. AC-391's *Cut* is **refuted against its own cited source**: proposal §4.3's Inner cell reads *"**Observe, record, do not duplicate.** Inner attempts count toward the activity's budget"* and the *Cut* quoted the bolded first sentence and stopped; schema §4:83 and design §3.6:338 say the same independently | **ADR-0122 is landed** and is this decision's primary record. Still owed: withdraw AC-391's *Cut*; a C1 resolution line **correcting** its *"ADR-0037 is superseded"* sentence (it is not — ADR-0037's whole Decision is a load-time lint rule on a **declared** field, and nothing in it moves); a clarification to design §3.6:337 |
| **D-3** | §7b.11 precondition 2 requires AC-071's stability record, which has no producer and no lawful path | **Transcribe F36 and land the amendment** — satisfy the guard rather than narrow it | The guard is **too wide** (golden constraint 9 / Halt 7): there are **zero `llm` facets** [measured 2026-09-14 — 9 human, 6 derived, 4 kernel, 1 declared; Step 4a took llm 7 → 0 at `db6b55c`] so AC-071's THEN requires a record *"for that facet"* and no facet requires one; and **Checkpoint 7b reads the record nowhere**. Satisfied rather than narrowed **because the pin move happens exactly once** (`PROGRESS.md`:103) | The record itself — `results/p1-arm-l-stability.json`, transcribing F36's identity figures **0.25 / 0.15 / 0.10 / 0.10** against the **0.80** line, scalar = the **minimum**, fail-closed — lands in the **corpus**, which §7b.11 precondition 7 reserves to the operator. Plus its dated register amendment naming path and producer, and **ERRATA E-0071-1** for the dead `--stability-record` path |
| **D-12** | §7b.10 routes ADR-0114 decision 2's executed-once ledger *"through AC-110"*, whose THEN says nothing about a ledger | **Allocate AC-532** | Under ADR-0092 a runbook cannot add a THEN. Amending AC-110 was refused: it would bolt a second unrelated subject onto one id, against the register's own definition of a criterion as *"One GIVEN/WHEN/THEN naming **one** observable behaviour"* | A register amendment allocating **AC-532** (allocated ids run to AC-531 [measured]), and a note re-pointing `ac110_executed_ledger_first_and_last` at it |
| **D-13** | Register §0.3's **C3**: AC-023 admits a typed authored budget-change event; AC-389 refuses an in-run appetite raise outright | **AC-389 — refuse outright** | *"A build cannot implement both refusals."* The escape hatch is not merely unbuilt but **unbuildable without a record**: it needs an `EventKind`, the enum is **closed at 29**, and ADR-0116 permits minting only by record. AC-396 already routes the appetite **value** to a human gate | A register amendment cutting AC-023's *"unless"* clause (and withdrawing its second instrument **with** the clause rather than leaving it ARMED against no stage), and a C3 resolution line in C4/C5's form |
| **D-14** | Phase-1 exit criterion (5) gates on the `--stay` fixture and no criterion states it | **Fold into AC-294; withdraw exit criterion (5)** | AC-294 already owns a private constructor + a module-boundary lint + a `trybuild` case — the identical instrument family, in the same crate. Exit check (5)'s only warrant was ADR-0001:42, which the register classes as *"a premise, not a checkable behaviour"* | A register amendment extending AC-294's THEN, and the withdrawal of exit criterion (5). **Separately owed**: the by-path assertion in `s8` check (5), which Checkpoint 7b has and `s8` lacks |

---

## What the decisions cost, stated once

Two of the fourteen closed defects that were **live and not theoretical**, and both were found by reading a record
against the code rather than by testing:

- **D-6** — 9 of 102 shipped activities could never close a gate on their own evidence.
- **D-5** — a journal carrying one ordinary race (F18: 45% of gate→verdict intervals exceed an hour) was
  **permanently unfoldable**, because the log is append-only.

And one the decision itself opened, caught on review and closed in the same commit: D-5's split made a **forged
token load-bearing**. `GateOpened` derives its token and is unforgeable; the `GateExpired` arm read its token
verbatim and inserted unconditionally, so one forged `gate_expired` row routed a forged answer around the single
branch D-5 deliberately kept. Expiry is now recorded only for a gate the fold actually saw open.

**Four of the fourteen turned out to be FORCED rather than open** once the record was read rather than the
summary of it — D-9 outright, and the reading in D-5, D-12 and D-14 each followed from a clause already written.
A question that looks like a decision is worth reading the primary source for before putting it to the operator.
