---
id: "0122"
title: Conflict C1 resolves for AC-026 and AC-232 — a harness-internal `system/api_retry` attempt charges the activity's attempt budget, one charge per recorded event in the pass that dispatched it, and the kernel adds no outer retry for the failure the harness already retried; AC-391's *Cut* does not stand and ADR-0037's lint rule is untouched
status: accepted
date: 2026-09-14
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — **cited by id and named clause as well as by line**, because this record's decision 5 names two dated amendments owed to that file (AC-391's *Cut* paragraph and §0.3's C1 row), so the lines below move as soon as they land; read at `60dc234`: §0.3's contradiction table, row **C1** (:115, *"The decision is the operator's; the register records the distribution and does not take it"* and *"If C1 resolves against AC-391, ADR-0037 is superseded, never edited"*), **AC-026** whole (:523-542, its THEN and its 2026-09-11 PIA-068 Correction), **AC-232** whole (:3647-3657, `rk-adapter-claude`'s half and its PIA-068 amendment), **AC-391** whole (:5795-5811, its THEN, its PIA-068 *Limit* and its *Cut:* paragraph), **AC-021** whole (:485-490), AC-020 (:478-483), AC-022 (:491-495), the exhaustion pair AC-024 (:506-513) and AC-025 (:514-521), AC-027 (:546-554), AC-029 (:562-), AC-234 (:3666-3673), AC-390 (:5786-5794), and §0.3's C4 and C5 rows (:118-119) as the form a resolved contradiction row takes; references/2026-09-07-activity-graph-schema.md **§4 :83** (the declaration-and-draw-down sentence and *"Inner harness retries (`system/api_retry`) count."*), **§9a item 7 :236** (lint rule 8, ADR-0037's own source), :111 (the worked `attempts_total = 8` budget); references/2026-09-05-architecture-proposal.md **§4.3 :252** (the Inner layer row — the cell AC-391's *Cut* quotes half of), :253 (the Outer row), :254 (the Budgets row); references/2026-09-08-architecture-design.md **§3.6 :338** (the Inner retry row, already aligned with this decision) and §3.10 :426 (the discharged payload-capture row); references/adr/0037-budget-attempts-total-counts-steps-per-pass.md in full — its `## Decision` (one sentence), its `**What this forbids**` block and its Consequences **Negative**; references/adr/0053-for-each-step-counts-once-against-budget.md `## Decision`, **Negative** and forbids block; references/adr/0112-steperror-gains-platform-unsupported-and-overage.md `## Decision` (the `Platform { status, retry_after }` row) and references/adr/0095 decision 2's eight surviving rows through it; references/adr/0022-activities-are-encapsulated-graphs.md `## Decision` paragraph 1 (retry is invisible to the stage); references/adr/0092-errata-precedence-and-cut-id-reissue.md decision 1 :64-68 (the erratum/supersession line) and decision 2 (precedence); references/adr/0003-five-named-ports-no-postgres.md (AC-232's `decisions:` line); references/adr/README.md (the partial-supersession rule and the eleven-key frontmatter); references/adr/ERRATA.md (read in full for a 0037, 0053 or 0003 section — there is none); runbook/phase-1/s7b-kernel.md §7b.10 :846-850 and §7b.13 :1170 and §7b.14's third loop :1526 (`inner_retry_ac026`); runbook/00-preamble.md :6-10 (precedence) and :55; the corpus at `CORPUS_PIN` `CORPUS_SHA=27d97e9b164d09dc755fe38e353de922cb97c482`, `results/*.jsonl` (all 12 files carrying `api_retries`); spikes/p9's 105 live activity files, read at `60dc234`; and **the operator's decision of 2026-09-14, taken at Step 7b Slice 4 — there is no `spikes/DECISIONS-2026-09-14.md` and this record is that decision's primary record**
supersedes: []
superseded_by: []
related: ["0003", "0022", "0037", "0053", "0092", "0095", "0112"]
evidence: ["S-RETRY", "F4"]
consumers: []
---

## Context and problem

**§0.3's C1 is the register's oldest open contradiction and it is one clause wide.** Three criteria describe the same
dispatch. Their THENs, whole:

> **AC-026** — GIVEN a dispatch during which the harness emitted `system/api_retry` events carrying an attempt counter
> / WHEN the activity's attempt budget is evaluated / THEN those inner attempts are counted against the activity's
> budget and the kernel does not add a retry of its own for the same failure.

> **AC-232** — GIVEN a stream containing `system/api_retry` events / WHEN the adapter processes them / THEN each is
> recorded and counted against the activity's attempt budget, and the adapter issues no retry of its own for those
> attempts.

> **AC-391** — GIVEN a dispatch during which the harness performs its own inner retries / WHEN those retries are
> observed on the stream / THEN each observed inner attempt is recorded as a tier-2 event carrying the harness's own
> attempt fields / AND the kernel issues no retry of its own for an error class the harness already retried.

**On the no-duplicate half all three agree, and none of them is in conflict.** The kernel adds no retry of its own.
AC-391's *Cut* removed exactly one clause and named it:

> *Cut:* "counted toward the activity's budget for that pass". ADR-0037 counts *steps* per pass and the §4.3 gloss says
> observe and record, do not duplicate; nothing cited licensed folding harness-internal retries into the attempts
> budget, and doing so would make the budget a function of platform-side behaviour the kernel cannot see or bound.

So the contradiction is **2-vs-1 over one clause**: whether the observed inner attempts *charge*. The register's own
disposition, verbatim from the C1 row: *"Both readings are implementable and they give different budgets. … The
decision is the operator's; the register records the distribution and does not take it."* And its instruction on
mechanism, verbatim: *"If C1 resolves against AC-391, ADR-0037 is superseded, never edited."*

### The *Cut*'s stated warrant is false against the source it cites, and this is measured

AC-391's `evidence:` line names *"§4.3 inner/outer table"*. That table's Inner row, quoted whole from
`2026-09-05-architecture-proposal.md`:252:

> | Inner | the harness itself (Claude Code emits `system/api_retry` with `attempt`, `max_retries`, `retry_delay_ms`,
> `error`) | **Observe, record, do not duplicate.** Inner attempts count toward the activity's budget |

The *Cut* quotes the bolded first sentence of that cell and concludes from it that *"nothing cited licensed folding
harness-internal retries into the attempts budget"*. The second sentence of the same cell is the licence
[measured 2026-09-14, by reading :252 whole]. *"Do not duplicate"* governs the kernel's **retry**, not the kernel's
**counting** — it is the sentence that produces the no-duplicate half all three criteria already share.

The same is true one document down. `2026-09-07-activity-graph-schema.md`:83, whole:

> Every `agent`/`tool` step declares `timeout_secs` … and `retry = { max = N }` selecting the kernel's per-error-class
> policy (§4.3) — a step never defines its own backoff. The activity declares
> `budget = { attempts_total, tokens, wall_clock_secs }`; **steps draw from it**; exhaustion is
> `Blocked { reason: BudgetExhausted }`. **Inner harness retries (`system/api_retry`) count.**

And design §3.6 :338 already reads *"observe, record, do not duplicate; inner attempts count against the activity's
budget"* — so the design needs no amendment from this record and none is claimed.

**The *Cut*'s second ground is not false, and it is this record's Negative.** *"Doing so would make the budget a
function of platform-side behaviour the kernel cannot see or bound"* is true, it is measured below, and the decision
is taken against it with the cost recorded rather than argued away.

### What the corpus actually holds, re-measured today rather than restated

Every figure below is re-derived from the corpus at `CORPUS_SHA=27d97e9b164d09dc755fe38e353de922cb97c482`, by walking
every `results/*.jsonl` record for an `api_retries` array and counting its entries
[measured 2026-09-14, `git -C D:/repos/redkiln-corpus show 27d97e9b:results/<f>.jsonl` piped to a JSON walk over the
12 files, run today]. It **reproduces the register's PIA-068 recount of 2026-09-11 exactly**, which is the first thing
worth saying about it.

- **139 rows carry a non-empty `api_retries`, across 12 result files**, holding **284 entries** in all:
  `p7` 38, `s-var` 28, `e3` 26, `s-typed` 18, `e0-stability` 6, `p8` 5, `s-var-2400` 5, `h4` 4, `p1-arm-l-v2` 4,
  `s-typed-oldschema` 3, `e3-pilot` 1, `e3h` 1.
- **Per-row entry count:** 1 on 89 rows, 2 on 21, 3 on 12, 4 on 4, 5 on 1, 6 on 6, and **10 on 6**.
- **`max_retries` is 10 on all 284 entries**, so the six rows at ten are the ceiling, reached.
- **`error`** is `unknown` on 217 and `overloaded` on 67; **`error_status`** is null on 217 and **529** on 67.
- **`retry_delay_ms`** spans **502 ms to 39,044 ms** and sums to **1,243.8 s** across the 284.

One fact the register does not carry, and it decides the unit: **the `attempt` counter restarts inside a single
dispatch**. Sequences like `[1, 1, 2, 3, 1, 2]` occur; **37 of the 139 rows have more entries than their largest
`attempt` value**, and the sum of the per-row **maximum** `attempt` values is **223** against **284** recorded entries (the sum of the per-row **final** values is **216**)
[measured 2026-09-14, same walk]. A charge read off the counter would therefore under-charge by **61 attempts, 21.5%
of the recorded events**. AC-232's instrument already picks the other unit — *"an `e3.jsonl` row with two inner
retries … asserting budget decremented by two"* — and this record fixes it.

### And AC-021, which is the hard part

> **AC-021** — GIVEN an activity whose graph runs A agent/tool steps in one pass and then retries the pass / WHEN
> `attempts_total` is evaluated / THEN it counts the agent/tool steps of a pass, not the sum of retried steps across
> passes.
> *Verified by:* a unit test over a two-pass fixture asserting `attempts_total == A`, not `2A`.

AC-021 and AC-026 are both **binding**, both live in `rk-kernel::budget`, and they use the same word for two
quantities. Decision 3 reconciles them and names what the reconciliation costs.

## Decision drivers

- **The register does not hold the answer and says so.** C1's row records the distribution and refuses the decision;
  §7b.10 and §7b.13 give `inner_retry_ac026` a name, but naming a test is not deciding a criterion: precedence is
  *"the AC register owns every criterion's wording, instrument and state … Where a stage file disagrees with any of
  those, the stage file is wrong"* (`00-preamble.md`:6-10, ADR-0092). *(Corrected 2026-09-14, before this record was
  committed: this bullet carried the string "may not add a THEN to a criterion" in quotation marks and attributed it
  to `00-preamble.md`:6-10. **That string appears nowhere in `runbook/` or `references/`** — `grep -rn "add a THEN"
  runbook/ references/` returns only this file [measured 2026-09-14]. The paraphrase states the precedence rule
  correctly and the rule is real; the quotation marks were not. A fabricated quotation inside a record whose whole
  subject is that one criterion quoted half a source is the defect this record exists to name, and it is corrected
  here rather than carried.)* A stage may not settle a record's ambiguity in code (`:55`). What was owed was this
  record.
- **The *Cut*'s first ground is refuted by its own cited source**, measured above; its second ground survives and is
  carried honestly in Consequences.
- **2-vs-1 with an adapter on the majority side.** AC-232 is `rk-adapter-claude`'s, with a recorded fixture and a
  numeric instrument; AC-026 is the kernel's. AC-391 is a `rk-kernel` / `rk-adapter-claude` refutation-pass criterion
  whose surviving clauses are compatible with both.
- **A budget that ignores the platform's own attempts is not a ceiling on anything the operator pays for.** Six
  recorded dispatches spent ten API attempts apiece. Under AC-391's reading the attempts budget records none of them,
  and the operator's only ceiling on that spend is the token appetite and the wall clock.
- **The alternative is not free either**, and it is the shape of the cost rather than its size that decides: an
  uncharged inner attempt is invisible to the one counter named *attempts*, and AC-025's `Err(BudgetExhausted)` then
  never fires for the failure mode that most often exhausts a real run.

## Decision

**1. An inner harness attempt charges the activity's attempt budget, one charge per recorded `system/api_retry`
event, in the pass that dispatched it.** Three parts, each separately binding:

- **That it charges.** AC-026's and AC-232's THENs stand as written. The runtime attempt counter for the activity is
  debited by the harness's own attempts observed on the stream of a dispatch, as well as by the dispatch itself.
- **The unit is the recorded event, not the terminal value of its `attempt` field.** A dispatch whose stream carries
  *n* `system/api_retry` events charges *n*. This is AC-232's instrument taken literally (*"budget decremented by
  two"* for *"a row with two inner retries"*), and it is forced by measurement: the counter restarts within a single
  dispatch on 37 of 139 recorded rows, and reading the charge off the counter would lose 61 of 284 attempts.
- **The charge lands in the pass that dispatched it**, against that activity's `budget.attempts_total`, and it is not
  carried forward to another pass or another activity. Exhaustion is `Blocked { reason: BudgetExhausted }`
  (schema §4 :83) and reaches AC-024/AC-025 unchanged.

**2. The kernel issues no outer retry for the failure that terminated a dispatch the harness already retried inside
that dispatch.** The suppression is **dispatch-scoped**, on AC-026's wording — *"for the same failure"* — and it
suppresses nothing else. AC-029's backoff envelope, AC-033's breaker and ADR-0112's `Platform` disposition are
untouched for every other failure, including a later failure of the same class on a different dispatch. AC-391's
surviving second clause reads *"for an error class the harness already retried"*, which is broader and, read
class-wide, would silently disable outer retry for `Platform` for the rest of a run.

🛑 **This clause decides more than C1 asked, and that is stated rather than hidden.** *(Added 2026-09-14, before this
record was committed, on an adversarial verification pass.)* C1's question, verbatim from the register row at :115, is
**"Do harness-internal retries charge the activity's attempt budget?"** — the *scope* of the no-outer-retry
suppression is not part of it, and all three criteria already agree that the kernel adds no retry of its own. The
sentence above chooses **dispatch scope** over AC-391's broader class scope, and an earlier draft of this clause
claimed to be "naming rather than settling" while settling it in the same breath. Both halves cannot be true. **The
resolution taken: this record takes the dispatch scope for the build, because AC-026 is the criterion Step 7b claims
and *"for the same failure"* is its wording — and decision 5(c) routes the register wording to a dated amendment.
Until that amendment lands, AC-391's surviving second clause and this clause disagree in scope, and any stage
claiming AC-391 must report it ARMED, NOT DISCHARGED for that reason.** A reader who needs only C1's answer should
read clause 1; clause 2 is a build-scoped consequence and is labelled as one.

**3. AC-021 and AC-026 both hold, because `attempts_total` names two quantities and only one of them is AC-021's.**
This is the load-bearing clause and it is stated against its own interest.

- **AC-021's quantity is the declared budget field.** Its `evidence:` line is *"activity-graph schema §9a"*, whose
  item 7 (:236) is lint rule 8; its only `decisions:` citation is **ADR-0037**, whose whole Decision is that lint
  rule; and its instrument is that rule restated as a test — `attempts_total == A`, not `2A`. What AC-021 forbids is
  **summing the same steps across passes to inflate the declared minimum**: a two-pass graph does not require `2A`.
- **AC-026's quantity is the runtime draw-down.** Schema §4 :83 puts both in one sentence and keeps them apart: the
  activity *declares* `budget`, **steps draw from it**, and inner harness retries **count**. The declaration is
  AC-021's; the draw-down is AC-026's.
- **The two were already known to diverge, by an accepted record.** ADR-0053's **Negative**, verbatim: *"The static
  attempts-budget check does not reflect the true worst-case attempt count of a fan-out step whose cardinality is
  large — a `for_each` over many elements can still exhaust `budget.attempts_total` at runtime even though it passed
  the static lint check."* A declared floor that the runtime charge may exceed is the existing, accepted design. This
  record adds a second source of that divergence; it does not introduce the divergence.
- **Under the alternative reading they cannot both hold, and the alternative is rejected.** If `attempts_total` in
  AC-021's THEN names the runtime counter, then a counter invariant across passes can never be exhausted by repeated
  passes, AC-025's `Err(BudgetExhausted)` becomes unreachable on the retry path, and clause 1 contradicts it outright.
  That reading is rejected on (a) AC-021's evidence line, (b) its sole decision citation, (c) schema :83's split and
  (d) ADR-0053's Negative — four independent grounds, all documented.
- **What survives either reading, and is the narrow claim this record actually needs:** AC-021's forbidden quantity is
  **cross-pass summation of the same steps**. An inner attempt observed within one dispatch, inside one pass, is not
  that. Even a reader who keeps the runtime reading of AC-021 does not thereby find a collision with clause 1; they
  find a collision with AC-025, which is a different and older problem.

**4. ADR-0037's Decision survives intact, ADR-0053 is undisturbed, and this record supersedes nothing — the
register's C1 row predicted a supersession that its own text does not support.** The register's instruction was
tested, not assumed, and the finding is reported:

- **ADR-0037's `## Decision` is one sentence:** *"Lint rule 8 becomes: `budget.attempts_total ≥ number of agent/tool
  steps on one pass`."* It is a **load-time check on a declared field**. It says nothing about what consumes the
  budget at runtime, and clause 1 moves no word of it.
- **ADR-0037's `**What this forbids**` block is likewise about the author:** *"an activity's declared
  `budget.attempts_total` cannot be lower than the number of `agent`/`tool` steps that execute on one pass through its
  graph; a catalog author cannot rely on the old summed-retry accounting to justify a smaller budget."* Untouched.
- **Its title's *"not summed retries"* means the old lint formula**, `Σ (step retry.max × 1)` — ADR-0037's Context
  says so: *"a formula stated in terms of summed per-step retry allowances"*. It is not a runtime accounting rule, and
  AC-391's *Cut* read it as one.
- **What was actually superseded was an interpretation, and an interpretation cannot be superseded.** ADR-0037 was
  cited as the authority for a runtime rule it never states — by AC-021, AC-026, AC-390 and AC-391 alike. Under
  ADR-0092 decision 1 :64-68 an erratum is for evidence, citations, numbers and characterisations and a supersession
  is for a `## Decision` sentence or a forbids bullet; **this record needs neither**, because nothing of ADR-0037
  moves and no evidence line of it is corrected. It carries `supersedes: []`, ADR-0037 keeps `superseded_by: []`, and
  **ADR-0037 is not edited** — which is the half of the C1 row's instruction that binds.
- **ADR-0053 is undisturbed and its Negative gains a second instance.** Its Decision — *"Rule 8's step count treats a
  `for_each` step as counting once, regardless of its runtime fan-out cardinality"* — is the same load-time check and
  is unmoved. Its forbids bullet — *"Lint rule 8 cannot be used as a runtime attempts-safety guarantee"* — is now true
  for a second reason as well as fan-out cardinality, which strengthens it rather than contradicting it.
- **If a later reader disagrees**, the disagreement is about whether ADR-0037's Decision sentence has a runtime
  reading. It does not, on its own text; but the register said it would be superseded, and a record that quietly
  declined to do so without saying it declined would be the defect. This clause is that saying.

**5. What is owed above this record, named and deliberately not written here.** ADR-0092 decision 2 makes the AC
register the owner of every criterion's wording; a record is the authority for an amendment, never the amendment.

- **(a) AC-391's *Cut:* paragraph must be withdrawn by a dated register amendment**, restoring *"counted toward the
  activity's budget for that pass"* to its THEN, and recording that the *Cut*'s first ground was refuted against
  proposal §4.3 :252 whole. AC-391's remaining clauses — the tier-2 event per observed attempt, its PIA-068 *Limit*
  note and its F4 citation — are untouched by this record and should survive the amendment.
- **(b) §0.3's C1 row gains a resolution line**, in the form C4 and C5 already carry — *"Resolved 2026-09-14 by the
  operator's decision at Step 7b Slice 4, recorded as ADR-0122: AC-026 and AC-232 stand; AC-391's *Cut* is
  withdrawn"* — and the row's *"If C1 resolves against AC-391, ADR-0037 is superseded"* sentence should be corrected
  to what decision 4 found.
- **(c) A dated clarification of AC-391's surviving second clause** — *"an error class the harness already retried"* —
  against AC-026's *"the same failure"*, so the two do not ship as a dispatch-scoped and a class-scoped rule under one
  behaviour. Decision 2 takes the dispatch scope for the build; the register owns the wording.
- **Until (a) and (b) land, `inner_retry_ac026` is built against this record and AC-391 still carries a cut clause
  its own register row contradicts.** A stage claiming AC-391 reports it **armed, not discharged**.
- 🛑 **(d) A dated amendment to design §3.6 :337 is OWED, and an earlier draft of this record missed it.** *(Added
  2026-09-14 on an adversarial verification pass; the draft asserted "nothing is owed to design §3.6" on the strength
  of :338 alone, while :337 is the row a later reader will use to attack decision 3.)* §3.6's **Budgets** row, whole
  and verbatim [measured 2026-09-14, by reading it]:

  > | Budgets | three counters, **never shared** | attempts per activity · tokens per item (appetite, fixed at
  > triage) · wall clock per run. Distinct newtypes with no `From`/`Add` crossing them, so a shared counter does not
  > compile. `attempts_total` counts the agent/tool steps of a **pass**, not the sum of retried steps across passes;
  > a `for_each` over N items charges **once**, independent of N | AC-020, AC-021, AC-022, AC-023; ADR-0037, ADR-0053 |

  This states AC-021's rule *inside a cell whose subject is runtime counters*, which is the **strongest documentary
  argument against decision 3's split** and it must be met rather than omitted. **It does not defeat the split, and
  the reason is that the design holds both readings one line apart**: the very next row, :338, reads *"inner attempts
  count against the activity's budget"*. A table that asserts "not the sum of retried steps" at :337 and "inner
  attempts count" at :338, in adjacent rows of one in-force table, is either self-contradictory or is naming two
  quantities — and decision 3 is the reading under which the design is coherent. **What is owed is a dated
  clarification to :337** saying which quantity its sentence governs (the declared field, on ADR-0037's unit) so the
  two rows cannot be read as one. Named here, **not written here** (`00-preamble.md`:55).
- 🛑 **(e) AC-022 is cited in this record's `source:` key and treated nowhere in its body, and clause 1 puts it
  under real pressure.** *(Added 2026-09-14, same pass.)* AC-022's THEN is **more runtime-worded than AC-021's** —
  *"GIVEN a `for_each` step projecting over N items / WHEN the attempts budget **is charged** / THEN the step counts
  once, independent of N"* (:491-495, verbatim) — and under clause 1 a `for_each` over N items whose fan-out
  dispatches carry inner retries charges **far more than once at runtime**. The two are reconciled the same way
  clause 3 reconciles AC-021: **AC-022 governs the step's own charge (one, not N), and clause 1 governs the
  harness's attempts within each dispatch**; they compose rather than conflict, because a `for_each` charging once
  plus the inner attempts observed inside it is still not a charge that scales with N. **That reading is
  [inferred], not documented**, it is the weakest link in this record, and it is stated as such: if it is wrong,
  what moves is AC-022's instrument, not clause 1. A stage building `for_each_charges_once_independent_of_n` must
  assert the step-charge cardinality at N = 1, 2 and 50 **with no inner attempts in the fixture**, so the two rules
  are measured separately and neither masks the other.
- **Nothing further is owed to design §3.6 :338**, which already states clause 1's rule [measured 2026-09-14, by reading it],
  and nothing is owed to ADR-0037, ADR-0053 or ADR-0003, none of which has an ERRATA section
  [measured 2026-09-14, `references/adr/ERRATA.md` read in full: sections exist for 0001, 0012, 0019, 0056, 0067,
  0074, 0075, 0077, 0079, 0080, 0081, 0082, 0083, 0084, 0085, 0086, 0087, 0088, 0094, 0089, 0100 and AC-520 — no
  0037, no 0053, no 0003].

**6. What this record does NOT decide**, named rather than left to be discovered.

- **Whether the token and wall-clock counters are charged by an inner attempt.** They are not charged *by this
  record*. An inner attempt plainly consumes wall clock — the recorded delays alone sum to 1,243.8 s over 284 attempts
  [measured 2026-09-14] — but that is elapsed time consuming a clock, not one counter deriving another, and AC-020's
  independence is about derivation. **No implementer may read this record as licensing a counter computed from
  another**; AC-020's `trybuild` newtype separation stands unweakened.
- **Whether an activity whose budget is exhausted purely by inner attempts escalates differently.** Schema §4 :83
  gives one exit, `Blocked { reason: BudgetExhausted }`, and this record adds no second one. Whether that block should
  carry a *why* distinguishing platform weather from graph cost is a register question and is not taken here.
- **What the tier-2 event for an observed inner attempt is called.** AC-391's first clause requires one per observed
  attempt; the closed `EventKind` table (design §5, ADR-0105 clause 7, ADR-0116 clause 5) is where it must exist or be
  minted by record, and this record mints no kind and asserts none exists.
- **Any production rate.** The 139 rows are spike traffic from one account over one programme. They establish that the
  ceiling is reachable and how often it was reached *there*; they bound nothing about a real run, and no interval was
  computed for any figure above.

## Considered options

### Option — resolve C1 for AC-391: record inner attempts, charge nothing (rejected)

The reading AC-391's *Cut* installs, and the lighter one: the budget stays a pure function of the graph, the kernel
can bound it by inspection, and the lint rule 8 floor becomes a real runtime guarantee for non-fan-out activities.
**Rejected on the ground that its stated warrant is false.** The *Cut* asserts *"nothing cited licensed folding
harness-internal retries into the attempts budget"*; proposal §4.3 :252 — the *Cut*'s own cited evidence — licenses it
in the second sentence of the cell the *Cut* quotes, schema §4 :83 repeats it, and design §3.6 :338 restates it
[all three measured 2026-09-14]. The option also loses the only counter named *attempts* as a record of attempts
actually made: six dispatches in the corpus made ten apiece and would have charged one.

### Option — charge by the terminal `attempt` value rather than per event (rejected)

The reading that "counts an attempt counter" most literally suggests: debit the largest `attempt` seen. **Rejected on
measurement.** The counter restarts inside a dispatch — 37 of 139 rows carry more entries than their maximum
`attempt`, and the per-row maximum values sum to 223 against 284 recorded events (the per-row final values sum to 216, so the under-charge is 61 or 68 depending on which value is read). It also contradicts AC-232's instrument,
which decrements by the number of retries recorded.

### Option — an erratum against ADR-0037 (rejected on mechanism)

Correct ADR-0037's reach in ERRATA.md rather than writing a record. **Rejected twice over.** ADR-0092 decision 1
:64-68 scopes an erratum to Context, Drivers, Options or Evidence and there is nothing in ADR-0037's to correct — its
text is accurate about the lint rule; it is the *citations of it* that overreached. And an erratum would put an
operator decision outside the supersession graph, which is the ground ADR-0115 and ADR-0121 both rejected this option
on.

### Option — write the resolution into the register and skip the record (rejected)

Amend AC-391 and C1 directly and let the register carry the decision. **Rejected on mechanism, as ADR-0116 rejected
the same shape for minting an event kind**: the register owns each criterion's wording, not the decision behind it,
and C1's own row says *"The decision is the operator's; the register records the distribution and does not take it."*
A register amendment with no record above it would leave a reader of the ADR index unable to find the decision at all.
The amendments are owed **after** this record and are named in decision 5.

### Option — leave C1 open and build `inner_retry_ac026` under one reading with a note (rejected)

What Slice 4 could do: implement AC-026, pin it with a named test, record the residual in PROGRESS. **Rejected as a
terminal state rather than as a stage act.** The charge is not a code detail — it changes what `Blocked
{ reason: BudgetExhausted }` means and it changes the numbers in 105 shipped activity budgets. Shipping it under an
undecided contradiction writes the ambiguity into the event log and into the budgets catalog authors write against.

## Consequences

**Positive**

- **C1 closes, and it closes with the measurement its own row said would decide it.** The register's PIA-068 recount
  reproduces exactly on an independent walk today, so the decision rests on a figure that has now been derived twice
  from the pinned corpus by two different readers.
- **The one counter named *attempts* counts attempts.** A dispatch that made eleven API calls debits eleven, not one,
  and an operator reading a budget-exhausted block is reading about work that was actually attempted.
- **AC-026, AC-232 and AC-391 stop pointing three ways.** All three already agreed that the kernel adds no retry of
  its own; after decision 5(a) they agree on the charge as well, and `rk-adapter-claude`'s AC-232 and the kernel's
  AC-026 can share one fixture and one assertion.
- **The unit is fixed before the first event is written.** The `attempt`-counter restart is a property of recorded
  data that a synthetic fixture would never have shown, and a build that read the charge off the counter would have
  under-charged by a fifth without any test going red.
- **ADR-0037 and ADR-0053 survive untouched**, and the set does not gain a supersession that withdraws nothing.

**Negative**

- **An activity's realised attempt count becomes a function of platform-side behaviour the kernel cannot see in
  advance or bound. This is AC-391's warrant and it is now this record's cost.** The measured shape of that
  exposure, at the corpus pin [measured 2026-09-14]:
  - `max_retries` is **10 on every one of the 284 recorded attempts**, so a single dispatch can charge up to ten
    before the kernel acts once, and **6 of 139 rows reached exactly that**.
  - **105 live activity files under `spikes/p9` declare `budget.attempts_total`, one each**, and the declared values
    are: **2** on 6, **3** on 21, **4** on 17, **5** on 20, **6** on 22, **8** on 10, **10** on 4, **12** on 4 and
    **14** on 1 [measured 2026-09-14, `grep -rh attempts_total spikes/p9 --include=*.toml`, read at `60dc234`].
  - So **one** ceiling-reaching dispatch charges more than the *entire* declared attempt budget of **96 of those 105
    activities**, and exactly all of it for **4** more. Under this decision, 100 of 105 shipped activities can be
    blocked by a single dispatch's platform weather, having completed one step.
  - The schema's own worked example (:111) declares `attempts_total = 8`. Ten exceeds it.
- **The budget's failure mode moves from graph cost to weather, and nothing in this record softens that.** The
  register's C1 row put it plainly and it remains true after the decision: *"six recorded dispatches would have spent
  ten attempts of an activity budget on platform-side `overloaded`/529 weather before the kernel acted once."*
- **No instrument bounds the exposure.** S-RETRY's throttle-margin arm is registered and **unrun**; no spike has
  measured an inner-retry rate at the throttle margin, no interval is computed for any figure here, and the 139 rows
  are a convenience sample of one programme's own traffic. The decision is taken on a distribution, not on a rate.
- **AC-021's instrument is not discriminating on this axis, and decision 3 depends on a reading its own test cannot
  distinguish.** A two-pass fixture asserting `attempts_total == A, not 2A` passes under both the lint reading and the
  runtime reading. Decision 3 chooses the lint reading on four documentary grounds, and the test named for it in
  §7b.13 does not choose.
- **The runbook glosses AC-021 the other way and a Slice-4 implementer will meet that first.** §7b.13 :1170 describes
  `attempts_total_counts_a_pass_not_retries` as *"red on a counter that sums retried steps across passes"* — a runtime
  counter. The runbook defers to the register and to accepted ADRs (`00-preamble.md`:6-10) so it is not authority, but
  a test written against its gloss and a charge written against decision 1 will disagree about the same field name
  inside one module. **The word `attempts_total` is now overloaded in `rk-kernel::budget` and this record does not
  rename it.**
- **105 catalog budgets were authored against the other reading.** Lint rule 8 passed them all; none was sized for a
  ten-attempt dispatch, because until today nothing said a dispatch could charge ten. No amendment to those budgets is
  ordered here, and no basis for sizing them exists — the numbers have never been measured (§7b.10's 🛑).
- **One more open question is created where one closed.** Decision 5(c) — AC-391's class-scoped versus AC-026's
  failure-scoped suppression — was not visible while C1 was open, and it is named rather than settled.

**What this forbids**

- Forbids a runtime attempt counter that ignores `system/api_retry` events observed on a dispatch's stream.
- Forbids deriving the charge from the terminal value of the `attempt` field, or from any quantity other than the
  number of `system/api_retry` events recorded for that dispatch.
- Forbids the kernel issuing an outer retry for the failure that terminated a dispatch the harness already retried
  inside that dispatch — and forbids extending that suppression class-wide to later dispatches, which would disable
  AC-029's envelope for `Platform`.
- Forbids reading AC-021 as a rule about the runtime draw-down counter, or ADR-0037's Decision sentence as anything
  but lint rule 8 on the declared `budget.attempts_total`.
- Forbids treating a passing lint rule 8 check as a bound on an activity's realised attempt count — ADR-0053's forbids
  bullet, now true for inner retries as well as for `for_each` cardinality.
- Forbids editing ADR-0037 or ADR-0053, and forbids recording this record as a supersession of either.
- Forbids charging the token or wall-clock counters from the attempts counter, or from each other, on the strength of
  this record — AC-020's newtype separation is untouched.
- Forbids a stage claiming AC-391 as discharged before the register amendments of decision 5(a) and 5(b) land.
- Forbids presenting the resolution of C1 as a measurement of what inner-retry rates *are*: the corpus establishes
  that the ceiling is reachable and was reached six times, and bounds nothing further.

## Evidence and confidence

- [measured 2026-09-14, corpus at `CORPUS_SHA=27d97e9b164d09dc755fe38e353de922cb97c482`, by walking every
  `results/*.jsonl` record for a non-empty `api_retries` array] **139 rows in 12 files, 284 entries**; per-row entry
  counts 1×89, 2×21, 3×12, 4×4, 5×1, 6×6, 10×6; per-file 38/28/26/18/6/5/5/4/4/3/1/1 for
  `p7`/`s-var`/`e3`/`s-typed`/`e0-stability`/`p8`/`s-var-2400`/`h4`/`p1-arm-l-v2`/`s-typed-oldschema`/`e3-pilot`/`e3h`;
  `error` `unknown` 217 / `overloaded` 67; `error_status` null 217 / 529 67; `max_retries` = 10 on all 284;
  `retry_delay_ms` min 502, max 39,044, sum 1,243.8 s. **This reproduces the register's PIA-068 recount of 2026-09-11
  exactly**, which is itself the evidence that the recount is sound.
- [measured 2026-09-14, same walk] **37 of 139 rows carry more `api_retries` entries than their maximum `attempt`
  value**; observed sequences include `[1, 1, 2, 3, 1, 2]`; every sequence begins at 1 but not every sequence is
  monotone; per-row **maximum**-value sum **223** and per-row **final**-value sum **216**, each against **284** recorded entries. This is the measurement that fixes the unit
  in decision 1 and it is not in the register.
- [measured 2026-09-14, `grep -rh attempts_total spikes/p9 --include=*.toml`, at `60dc234`] **105 activity files, one
  `attempts_total` each**: 2×6, 3×21, 4×17, 5×20, 6×22, 8×10, 10×4, 12×4, 14×1 — 96 strictly below 10, 4 at 10, 5
  above.
- [documented, `references/2026-09-05-architecture-proposal.md`:252] the Inner layer row, quoted whole above;
  [documented, :253] the Outer row's per-class policy; [documented, :254] *"three counters, never shared"*.
- [documented, `references/2026-09-07-activity-graph-schema.md`:83] the declaration / draw-down / *"Inner harness
  retries (`system/api_retry`) count."* sentence; [documented, :236] lint rule 8; [documented, :111] the worked
  `attempts_total = 8` budget.
- [documented, `references/2026-09-08-architecture-design.md`:338] *"observe, record, do not duplicate; inner attempts
  count against the activity's budget"* — the design already agrees, so no design amendment is owed.
- [documented, `references/adr/0037`] the whole record: a one-sentence Decision naming lint rule 8, a forbids block
  about a catalog author's declared budget, a Negative naming only the `for_each` residual ADR-0053 later closed.
  [documented, `references/adr/0053`] Decision, Negative and forbids, all load-time.
- [measured 2026-09-14, `references/adr/ERRATA.md` read in full] no section exists for ADR-0037, ADR-0053 or ADR-0003.
- [documented, register §0.3 row C1, AC-026, AC-232, AC-391, AC-021, read whole at `60dc234`] every criterion text
  quoted above is quoted, never paraphrased.
- **Confidence.** That the corpus holds the distribution above is **measured**, twice, three days apart, by two
  readers. That AC-391's *Cut* misreads its own cited source is **measured**, by reading proposal :252 whole. That
  ADR-0037's Decision has no runtime reading is **measured from the record's own text** — one sentence, a forbids
  block about authors, a Context that glosses *"summed retries"* as the old `Σ retry.max` formula. That AC-021 is the
  lint quantity rests on four documentary grounds and is **[inferred]**, not measured: no instrument distinguishes the
  two readings, and AC-021's own test passes under both. **That charging is the better of the two budgets is the
  operator's judgement**, taken against a real and measured cost, and it is recorded as a judgement.
- **What this evidence does not license.** It does not license any claim about how often inner retries occur in
  production — 139 rows of one programme's own spike traffic, no interval, S-RETRY's throttle-margin arm unrun. It
  does not license claiming that the 105 declared budgets are wrong; nothing has measured what they should be
  (§7b.10's 🛑). It does not license reading decision 3's reconciliation as settling what `attempts_total` should be
  *called* in `rk-kernel::budget` — the name stays overloaded and the record says so. And it does not license
  treating decision 4 as a criticism of the register: the C1 row's supersession sentence was a forecast about record
  mechanics, it was tested against ADR-0037's text as this record was asked to test it, and the finding is reported
  rather than quietly followed or quietly ignored.
