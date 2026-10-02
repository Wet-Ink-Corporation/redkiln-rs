---
id: "0198"
title: "**RT-57's pack scalars, the per-item envelope, and gate expiry are recorded, each value with its warrant and every reachable gate class fail-closed** (`s8h`'s Record B; D21-42 19a, 19c, 21a, 21c; D21-43 N17 Q2, Q3; RT-57; ADR-0146's owed item): the key spellings RT-57 routed to *\"7e's pack schema record\"* are fixed; both installed packs declare `appetite.dispatches = 40` and `appetite.tokens = 24000000` (the sum of `Usage`'s four counters, about twice a happy-path estimate of 17 × 681,083), `gate_deadline_secs = 864000`, `[breaker] consecutive_platform_failures = 3` and `expiry_secs = 600`, and `[margins] five_hour = 10` and `seven_day = 10`; the kernel counts an item's dispatches and charged tokens from the store and refuses the next dispatch past either limit before any spawn, through a kernel envelope class (not `StepError::Overage`) that leaves as `Exit::Overage` (20); every reachable gate class carries a `[[gate_expiry]]` row drafted `fail` for the operator's row-by-row signature, and an unmatched or misspelt class refuses at load. Supersedes nothing"
status: proposed
date: 2026-10-02
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 19a, 19c, 21a, 21c, D21-43 N17 (Q2, Q3), D21-46's integration call (one row per minted gate class), D21-47 cross #7, D21-129 (owed forward to `s8h`); spikes/DECISIONS-2026-09-18.md RT-57; runbook/phase-1/s8h-cards-and-run-environment.md §8h.9, §8h.10, finding 6 and finding 12, and the stage-open readings of 2026-10-02; references/2026-09-08-acceptance-criteria.md AC-020, AC-029, AC-033, AC-039, AC-220, AC-260, AC-276 and the expiry scope note; references/adr/0146 (its owed item); references/adr/0017
supersedes: []
superseded_by: []
related: ["0017", "0110", "0112", "0126", "0146", "0176", "0182", "0184", "0188", "0189", "0191", "0197"]
evidence: []
consumers: []
---

## Context and problem

**The pack's numbers are authored and named by no record.** RT-57 routed *"The `PackExtras` key floor — `margin_ticks`,
the two TTL keys, `gate_deadline_secs`, per-activity `timeout_secs`, the gate-class expiry action"* to *"7e's pack schema
record, which re-points the floor"*, and none was written. Both installed packs say so in their own bytes: the
`gate_deadline_secs` comment ends *"Authored at Step 7e -- RT-57, OWED."*, and the breaker block reads *"🛑 THE THREE
NUMBERS BELOW MARKED `NO RECORD` ARE AUTHORED HERE AND NAMED BY NO RECORD"*, with
`consecutive_platform_failures = 3   # NO RECORD -- authored at Step 7e (RT-57)` and
`expiry_secs = 600                   # NO RECORD -- authored at Step 7e (RT-57)` [measured 2026-10-02 at `2770bda`].
D21-42 19c: *"The scalar record is written and signed before any spend, each value with a warrant, replacing the `NO
RECORD` markers."*

**No per-item envelope exists.** The whole appetite is `appetite = { points = 40 }` in `bug-standard` and `appetite = {
points = 60 }` in `story`; `appetite.tokens` is read by `select/pack.rs` (`Pack::require_token_appetite`) and set in
neither pack; no per-item dispatch count exists. D21-42 19a: *"The kernel enforces a per-item dispatch-count cap and
token appetite, refusing with a typed class. First values: 40 dispatches, and about 2× the happy-path token estimate.
Both are revised and recorded from measurement."* D21-43 N17 took the unit and the exit: *"Q2: `appetite.tokens` is the
sum of `Usage`'s four counters, about 24M per pass at first. Q3: `Exit::Overage` (20)."*

**No gate class has an expiry row.** ADR-0146 measured it and owed it: *"**Zero rows, for zero of the 45 question
classes.** So a step gate that passes its deadline makes the next run exit 3 … **It is OWED**, and the honest options are
to ship the rows or to make an unmatched step-gate class a load-time refusal rather than a run-time exit 3."* AC-276 says
the choice per class is *"made and signed by a human, per class"*, and D21-42 21a took *"Drafted fail-closed with
exceptions justified; the operator signs row by row (AC-276)"*; 21c took *"Both built"* for ADR-0146's lint.

**And one implementation question was left for this record** (`s8h` finding 12): *"`StepError::Overage { reason }`
already exists. … Before building a parallel kernel `EnvelopeError`, the implementer has to decide whether the envelope
refusal is this variant with a `reason` naming the counter. If it is, Record B says so. If it is not, Record B says why
not."*

## Decision drivers

- **AC-029** (PIA-055): *"only K, N and the appetite are pack values"*; a code default for any of them is refused, and
  each is declared, never defaulted (`require_*`'s doctrine in `select/pack.rs`).
- **AC-020**: three budget counters, never shared. `points` (step 5's cost-class sum, invariant I6), `tokens` and
  `dispatches` are different counters.
- **D21-42 19a**: a typed class; first values 40 and about 2×; revised from measurement.
- **AC-276**: *"routed to a human gate; not machine-decidable"*; ADR-0017 fixes the set of expiry actions and *"explicitly
  does not choose among them"*. AC-260 is the machine half: the field must be present.
- **Fail-closed over fail-open.** A row drafted `fail` costs a failed run; a row drafted `auto-decide` would decide on an
  unreviewed tree, and `auto-decide` has no verdict carrier (RT-57), so `run.rs` refuses it by name.
- **The store, never a per-process counter**, because an item's dispatches span processes (`s8e`'s cross-process lane).

## Considered options

### Option — the drafted values with a warrant each, a kernel envelope class leaving as `Exit::Overage`, and fail-closed rows with both of ADR-0146's remedies (taken)

Everything below under `## Decision`.

### Option — count `appetite.tokens` with cache reads excluded

The alternative unit for Q2. Over the same corpus rows the pooled median per-dispatch total would be **M = 62,725**,
giving 2 × 17 × 62,725 ≈ **2.2 M** per pass [measured 2026-09-24 at `CORPUS_PIN` by `s8h`'s author, read-only; not
re-measured at this writing]. **Not taken**: N17 took Q2 as the four-counter sum, which every `dispatch_completed` row
can compute from its verbatim `Usage` without a price model or a reading of whether cache reads draw the window (F14's
question, which this does not re-open). Kept here as a considered option only.

### Option — reuse `StepError::Overage { reason }` with a `reason` naming the counter

Finding 12's alternative. **Not taken**, for three reasons, measured from the code at `2770bda`:
1. **`StepError` is the Harness port's outcome channel** (ADR-0095's table; ADR-0112 closes it at eleven). Its
   `Overage` is the adapter's pre-spawn refusal on `isUsingOverage: true` (`throttle.rs`'s `HaltOverage {
   overage_status }`, AC-221), and ADR-0126 decision 1 has the adapter append `dispatch_completed` *"on **every**
   termination path — `Ok(Typed)` and each `StepError` variant the adapter can produce, the pre-spawn refusals
   included"*. The envelope refusal is made by the kernel before the Harness is called. As a `StepError` it would either
   be the one `StepError` path with no `dispatch_completed` row, or force a row for a dispatch that never existed.
2. **A typed counter, not prose.** `Overage`'s payload is `reason: Option<String>`. Which counter refused, and its count
   and limit, would reach `cost`, `doctor` and the tests only as text to parse, and AC-020's counters are kept apart by
   type.
3. **The kernel already owns the envelope's pack half.** `rk_kernel::workflow::envelope::EnvelopeError` holds
   `TokenAppetiteNotDeclared`, `AttemptCapNotDeclared` and the breaker's two, each *"Why a pack cannot drive the
   runtime"*. The refusal half sits beside it in the kernel, and the error the retry loop writes through
   `StepError::event()` for a failed dispatch (ADR-0176) is not written for a dispatch that was never made.

### Option — ship the rows only, or the load-time lint only

ADR-0146's two remedies, each alone. Rows alone leave a misspelt or forgotten class to fail at run time as exit 3, the
symptom ADR-0146 measured; the lint alone refuses every shipped pack until rows exist. D21-42 21c took *"Both built"*.

### Option — draft `escalate` or `auto-decide` for some classes

Rejected as a draft. `auto-decide` has no verdict carrier and is refused by name; `escalate` is admissible only as a
signed exception with a warrant (AC-260 forbids `escalation → escalate`). None is drafted; a signed exception amends the
table by dated line.

## Decision

**1. The key spellings (RT-57's floor, re-pointed here).** Each installed pack (`.redkiln/packs/bug-standard.toml`,
`.redkiln/packs/story.toml`) declares every key below itself, so each stays independently complete. Integers carry no
`_` separators (`toml_subset`).

| key | where | type |
| --- | --- | --- |
| `gate_deadline_secs` | top level, before `[pack]` | integer seconds |
| `appetite.points` | `[pack] appetite = { … }` | integer (unchanged: 40 / 60) |
| `appetite.tokens` | the same inline table | integer, the four-counter sum |
| `appetite.dispatches` | the same inline table | integer, a count of dispatches |
| `consecutive_platform_failures` | `[breaker]` | integer (K) |
| `expiry_secs` | `[breaker]` | integer seconds |
| `five_hour`, `seven_day` | `[margins]` | integer ticks (u16) |
| `class`, `action` | each `[[gate_expiry]]` row | the full gate kind as `GateRecord::kind()` renders it; one of `fail` \| `escalate` \| `auto-decide` |

**2. The first values, each with its warrant**, signed by the operator value by value:

| key | value | warrant |
| --- | --- | --- |
| `appetite.dispatches` | **40** | D21-42 19a's value. It is above the research pass's realistic 20–35 per fix-lane pass (`s8h` §8h.9's facts brief, not re-derived here). |
| `appetite.tokens` | **24000000** | D21-42 19a's *"about 2× the happy-path token estimate"*: 2 × E, E = N × M (decision 3), ≈ 23.2 M, rounded up. D21-43 N17's *"about 24M per pass at first"*. |
| `gate_deadline_secs` | **864000** (240 h, 10 d) | The register carries **two readings of the 226 h figure, and they contradict** (`s8h` finding 6). The expiry scope note: *"the longest was answered at 226 h"*. AC-039's evidence line: *"the max 226 h / 9.4 days belongs to **stage→verdict** n=191, not to gate→verdict"*, with gate→verdict at *"n=174, median 10.5 min, 45% > 1 h"* and its maximum not stated. 240 h clears the larger series' maximum under either reading, which is the conservative direction for "do not expire an answered gate". The operator signs it with both readings in view. |
| `[breaker] consecutive_platform_failures` (K) | **3** | [measured 2026-09-24, `results/e3.jsonl` at `CORPUS_PIN`, file order, by `s8h`'s author]: 7 of 101 rows carry `is_error: true`, in runs of 1, 4, 1, 1. The one measured `Platform` episode reaches four consecutive; K = 3 trips inside it, the singles never trip it, and K ≥ 5 would never have tripped. An episode **shape**, not a rate: AC-033's note (*"no spike measured a `Platform` failure rate at all"*) stands, and file order is taken as append order. |
| `[breaker] expiry_secs` | **600** | [inferred] 2 × AC-029's 5 min backoff cap, so an open mark outlasts two capped backoffs. No measurement exists; it is signed as chosen. |
| `[margins] five_hour` / `seven_day` | **10 / 10** ticks | Prior art: the runner's `max_five_hour: 0.90` (`throttle.rs`'s header). The floor is AC-220's `MIN_MARGIN_TICKS = 2`. F14: two 20-dispatch haiku batches moved `seven_day` by 0 ticks. These lines carry no `NO RECORD` marker; they are in RT-57's floor and are recorded here anyway. |

**The warrant for `appetite.tokens` is weak, and this record says so.** The rows behind M are mostly Opus reviews and
investigations on another CLI version, with no agentic Sonnet row, so no tier-matched figure exists. D21-42 19a: *"Both
are revised and recorded from measurement."* The first real run's `dispatch_completed` usage revises both
`appetite.tokens` and `appetite.dispatches`, each by a dated line naming the measurement.

**3. E, the happy-path token estimate: E = N × M.**
- **N = 17**, the upper end of the research pass's *"about 14–17 dispatches on the happy path"*, re-derived exactly by
  `s8j`'s FakeHarness walk of the lane.
- **M = 681,083**, the pooled median of every per-dispatch four-counter total over the corpus's agentic dispatch rows
  (`p7`, `p8`, `e3`, `e3h`, `s-typed`, `s-typed-oldschema`, `s-var`, `s-var-2400`, `h4`: n = 384; p75 1,321,531)
  [measured 2026-09-24 at `CORPUS_PIN` `61f590e5…`, read-only, by `s8h`'s author; not re-measured at this writing].
- **E = 11,578,411**, 2 × E = 23,156,822, and the appetite is **24,000,000**.

**4. The per-item envelope (D21-42 19a; N17 Q2, Q3).**
- **Where the counts come from: the store.** Before each dispatch the kernel counts the item's prior `dispatch` rows and
  sums the four `Usage` counters (`input_tokens`, `output_tokens`, `cache_creation_input_tokens`,
  `cache_read_input_tokens`, verbatim) over the item's `dispatch_completed` rows, both read through `query(ByItem)`.
  Both kinds carry ADR-0182's interim `item` key (`ItemKey::Carried`), and the `dispatch` row's payload also names
  the item [measured 2026-10-02: `activity::dispatch_event`'s payload `{activity_id_at_version, attempt, item,
  step_id}`]. `ByItem` also returns rows with no key in the interim (ADR-0182 decision 4), so only rows keyed to the
  item are counted. Never a per-process counter.
- **When it refuses.** The next dispatch is refused once the prior count has reached `appetite.dispatches` (the
  dispatch at the cap proceeds; the one past it does not), or once the prior sum has reached `appetite.tokens`. A
  dispatch with no `usage` still counts against the cap, which is why the cap exists beside the appetite.
- **How it refuses.** Before any spawn, writing no tier-1 row, through **a kernel envelope class with one variant per
  counter** (drafted `DispatchCapReached { item, count, cap }` and `TokenAppetiteReached { item, sum, appetite }`), never
  `StepError::Overage` (the option above, finding 12). rk-cli maps the class to **`Exit::Overage` (20)**, whose doc
  already reads *"A budget or billing state that does not improve by being repeated"*, so the exit snapshot does not
  move and `StepError`'s set and mapping do not move.
- **Declared, never defaulted.** An absent `appetite.dispatches` is refused by pack validation as an `EnvelopeError`
  beside `TokenAppetiteNotDeclared`, naming the pack; no code default exists for either key.

**5. Every `NO RECORD` marker is replaced** by a comment naming this record as `ADR-0198`, on the `gate_deadline_secs`,
K, `expiry_secs`, `[margins]` and `appetite` lines at least, and `grep -c 'NO RECORD'` reads 0 in both packs.

**6. Gate expiry: one `[[gate_expiry]]` row per reachable class, each drafted `fail`, in both packs (D21-42 21a).** The
reachable classes are:
- **The 45 catalog question classes** [re-measured 2026-10-02: `grep -h '^question' .redkiln/catalog/*.toml | sort -u
  | wc -l` → 45, and the inline `question = "…"` spellings add none].
- **`unknown_facet:<axis>` for every axis a pack's `require` rows read**, because `step2.rs` opens one per abstaining
  axis (`gate.rs`: `format!("unknown_facet:{axis}")`): `bug-standard` reads `blast_radius`, `change`, `criticality`,
  `rigor` and `size` (5); `story` reads those and `external_dep` and `novelty` (7) [re-measured 2026-10-02 by parsing
  each pack's `require` rows]. `unknown_facet:change` is also `s8d`'s R6 triage gate (ADR-0178), answered with the
  `facet.change` question.
- **`escalation`** (`gate::ESCALATION_CLASS`), drafted `fail` so that any `escalate` exception the operator signs is
  lint-valid (AC-260: no `escalation → escalate`).
- **The kernel-minted classes**, read from their `&str` constants on code lines under `crates/*/src` [measured
  2026-10-02]: `landing.refused-path` (`tier_one::LANDING_REFUSED_KIND`; ADR-0188), `render.structured-drift`
  (`fsm::DRIFT_GATE_KIND`; ADR-0189) and `verify.ac-discriminates` (`verify::ac_gate::AC_GATE_KIND`; ADR-0191 decision
  10). None is a catalog question. Both packs carry all three, because both ship `verification_required = ["verify"]`
  and either pack's lane can land or drift.

That is **54 rows in `bug-standard` and 56 in `story`**. `budget_extension` has no producer (`wall_clock_expiry_mark`
has no call site under `crates/*/src` [measured 2026-10-02]), so it has no row. `s8j`'s three classes
(`human_root:BugReport`, `human_root:IntakeRecord`, `hold:specification`, all drafts) bring their rows in the change that
mints them, drafted fail-closed for the same signature, because the lint below refuses a row whose class nothing mints.
`facet.change`, `root.bug-report` and `root.intake-record` are question ids, not gate classes.

**The draft is `fail` for every row.** `auto-decide` is not drafted (no verdict carrier; `run.rs` refuses it by name),
and `escalate` is admissible only as a signed exception with a warrant. The operator signs **row by row** (AC-276). A
signed exception amends this table and Checkpoint 8h's exception list by dated line. The table, one row per class, as
drafted:

| # | class | kind of class | `bug-standard` | `story` | drafted action | signed |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `adr.accept` | catalog question | yes | yes | `fail` | |
| 2 | `api-contract.accept` | catalog question | yes | yes | `fail` | |
| 3 | `appetite.overrun` | catalog question | yes | yes | `fail` | |
| 4 | `asvs.level` | catalog question | yes | yes | `fail` | |
| 5 | `atam.accept-risks` | catalog question | yes | yes | `fail` | |
| 6 | `bisect.no-culprit` | catalog question | yes | yes | `fail` | |
| 7 | `bluegreen.cutover` | catalog question | yes | yes | `fail` | |
| 8 | `canary.degraded` | catalog question | yes | yes | `fail` | |
| 9 | `canary.proceed` | catalog question | yes | yes | `fail` | |
| 10 | `change.authorize` | catalog question | yes | yes | `fail` | |
| 11 | `closure.signoff` | catalog question | yes | yes | `fail` | |
| 12 | `confirmation.not-fixed` | catalog question | yes | yes | `fail` | |
| 13 | `dependency.breaking` | catalog question | yes | yes | `fail` | |
| 14 | `discovery.go-no-go` | catalog question | yes | yes | `fail` | |
| 15 | `draft.accept` | catalog question | yes | yes | `fail` | |
| 16 | `hotfix.authorize` | catalog question | yes | yes | `fail` | |
| 17 | `incident.end` | catalog question | yes | yes | `fail` | |
| 18 | `inspection.meeting` | catalog question | yes | yes | `fail` | |
| 19 | `loop.exhausted` | catalog question | yes | yes | `fail` | |
| 20 | `migration.approve` | catalog question | yes | yes | `fail` | |
| 21 | `migration.execute` | catalog question | yes | yes | `fail` | |
| 22 | `mitigation.not-restored` | catalog question | yes | yes | `fail` | |
| 23 | `mitigation.proceed` | catalog question | yes | yes | `fail` | |
| 24 | `postmortem.review` | catalog question | yes | yes | `fail` | |
| 25 | `prototype.user-test` | catalog question | yes | yes | `fail` | |
| 26 | `prr.accept-gaps` | catalog question | yes | yes | `fail` | |
| 27 | `repro.cannot-reproduce` | catalog question | yes | yes | `fail` | |
| 28 | `review.blocking` | catalog question | yes | yes | `fail` | |
| 29 | `rollback.confirm` | catalog question | yes | yes | `fail` | |
| 30 | `rollout.approve` | catalog question | yes | yes | `fail` | |
| 31 | `rollout.ramp` | catalog question | yes | yes | `fail` | |
| 32 | `root-cause.accept` | catalog question | yes | yes | `fail` | |
| 33 | `root-cause.no-hypothesis` | catalog question | yes | yes | `fail` | |
| 34 | `scan.accept-findings` | catalog question | yes | yes | `fail` | |
| 35 | `seam.cutover` | catalog question | yes | yes | `fail` | |
| 36 | `slicing.approve` | catalog question | yes | yes | `fail` | |
| 37 | `spec.ambiguity` | catalog question | yes | yes | `fail` | |
| 38 | `spec.requirements-conflict` | catalog question | yes | yes | `fail` | |
| 39 | `spike.decision` | catalog question | yes | yes | `fail` | |
| 40 | `spike.timebox-exhausted` | catalog question | yes | yes | `fail` | |
| 41 | `sprint.decide` | catalog question | yes | yes | `fail` | |
| 42 | `sprint.test-results` | catalog question | yes | yes | `fail` | |
| 43 | `tdd.green-exhausted` | catalog question | yes | yes | `fail` | |
| 44 | `threat-model.accept-residual` | catalog question | yes | yes | `fail` | |
| 45 | `vuln.accept-sla` | catalog question | yes | yes | `fail` | |
| 46 | `unknown_facet:blast_radius` | abstaining axis | yes | yes | `fail` | |
| 47 | `unknown_facet:change` | abstaining axis | yes | yes | `fail` | |
| 48 | `unknown_facet:criticality` | abstaining axis | yes | yes | `fail` | |
| 49 | `unknown_facet:rigor` | abstaining axis | yes | yes | `fail` | |
| 50 | `unknown_facet:size` | abstaining axis | yes | yes | `fail` | |
| 51 | `unknown_facet:external_dep` | abstaining axis | no | yes | `fail` | |
| 52 | `unknown_facet:novelty` | abstaining axis | no | yes | `fail` | |
| 53 | `escalation` | `gate::ESCALATION_CLASS` | yes | yes | `fail` | |
| 54 | `landing.refused-path` | kernel-minted, `tier_one::LANDING_REFUSED_KIND` (ADR-0188) | yes | yes | `fail` | |
| 55 | `render.structured-drift` | kernel-minted, `fsm::DRIFT_GATE_KIND` (ADR-0189) | yes | yes | `fail` | |
| 56 | `verify.ac-discriminates` | kernel-minted, `verify::ac_gate::AC_GATE_KIND` (ADR-0191 decision 10) | yes | yes | `fail` | |

**7. ADR-0146's load-time lint, both halves (D21-42 21c).** At pack load, `run` and `answer` refuse, before any dispatch:
- **an unmatched class**: a reachable class of the enabled pack with no `[[gate_expiry]]` row. ADR-0146's words are
  *"make an unmatched step-gate class a load-time refusal rather than a run-time exit 3"*, and **this record reads
  "unmatched" as having no row in the enabled process pack**;
- **a misspelt class**: a row whose class is neither a catalog question, nor `unknown_facet:<registered facet>`, nor
  `escalation`, nor a gate kind the kernel mints, because a misspelt class leaves the real class uncovered. The minted
  kinds are read from the code's constants, never from a list copied beside them.

**8. Signing and flipping.** The operator signs decision 2 value by value and decision 6 row by row on a dated ledger
line. This record flips to `accepted` at `s8h`'s close, before `s8i`, on that line, because D21-42 19c says *"written and
signed before any spend"* and `s8i` spends. The token appetite the operator signs replaces Checkpoint 8h's
`TOKENS_APPETITE` by dated edit if it differs from 24000000.

**9. It supersedes nothing.** ADR-0146's owed item is executed, not superseded; ADR-0017's action set and ADR-0110's
two-tick margin floor are executed; `StepError`, `Exit` and the exit table do not move.

## Consequences

- **Both packs are complete on RT-57's floor**, with no `NO RECORD` marker, and every number carries a warrant an
  operator can contest.
- **A runaway lane stops at 40 dispatches or 24 M tokens per item**, across processes, before the next spawn, with an
  exit that names a budget state rather than a configuration error.
- **An expired gate fails its run by declared row**, never by the absence of one, and a pack that forgets a class or
  misspells one refuses at load.
- **The values are first values.** The cap and the appetite move on the first real run's measurement, by dated line,
  and a signed exception moves a gate row.

**What this forbids**

- Forbids a code default for `appetite.dispatches`, `appetite.tokens`, `gate_deadline_secs`, K, `expiry_secs` or either
  margin, and forbids a pack that leaves any of them to another pack.
- Forbids counting an item's dispatches or tokens in process memory, or from rows not keyed to the item.
- Forbids refusing an envelope breach after the spawn, through `StepError::Overage`, or through any exit other than
  `Exit::Overage` (20).
- Forbids letting a dispatch with no `usage` escape the dispatch cap.
- Forbids a `NO RECORD` marker in either installed pack.
- Forbids a reachable gate class with no `[[gate_expiry]]` row, a row whose class nothing mints, a drafted
  `auto-decide`, and an `escalate` that is not a signed exception.
- Forbids reading a minted class's spelling from anywhere but its constant in the minting code.
- Forbids treating the 226 h figure as gate→verdict evidence without AC-039's correction beside it.

## Evidence and confidence

- [measured 2026-10-02 at `2770bda`] both packs' `NO RECORD` lines, `appetite = { points = 40 }` and `{ points = 60 }`,
  `gate_deadline_secs = 864000`, K = 3, `expiry_secs = 600`, margins 10 / 10, and 0 `[[gate_expiry]]` rows; the 45
  question classes; the 5 and 7 axes; the three minted constants; `ESCALATION_CLASS = "escalation"`;
  `dispatch_event`'s payload and both kinds' `ItemKey::Carried`; `StepError::Overage { reason: Option<String> }` and
  `HaltOverage { overage_status }`; `EnvelopeError`'s four pack-validation variants.
- [measured 2026-09-24 by `s8h`'s author, read-only at `CORPUS_PIN`, not re-measured here] M = 681,083 (n = 384), the
  cache-excluded M = 62,725, and `e3.jsonl`'s error runs 1, 4, 1, 1.
- [documented] the register's expiry scope note and AC-039's evidence line, quoted in decision 2.
- [inferred] `expiry_secs = 600`.
- **Confidence.** High in the key spellings, the store-sourced count and the fail-closed rows, which are constructions.
  **Low in `appetite.tokens`**: no tier-matched measurement exists, and the first real run's usage revises it. Moderate
  in K = 3, a shape from one measured episode. `gate_deadline_secs` rests on a register that contradicts itself, stated
  rather than resolved.
