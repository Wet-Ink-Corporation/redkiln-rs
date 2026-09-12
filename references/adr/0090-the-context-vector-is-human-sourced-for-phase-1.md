---
id: "0090"
title: The context vector is human-sourced for Phase 1 — every ordinal axis and all three LLM booleans are source = "human" at the triage gate, and ADR-0079 is superseded in full
status: accepted
date: 2026-09-12
legacy_id: none
source: runbook/phase-0/s4-blocking-gate.md §4.1 (the pre-registered rule) and §4.4; spikes/DECISIONS-2026-09-11.md #2 (the booleans clause, pre-registered before this record); spikes/METHOD-P.md §1.4 (both thresholds) and amendments 8/9/10(b); spikes/F36-DRAFT.md (arm L, v1 and the §1.4 re-run L2); spikes/F39-DRAFT.md (arm D, panel-adjudicated); references/adr/ERRATA.md E-0079-1 and E-0089-2; references/2026-09-08-acceptance-criteria.md AC-071; references/2026-09-10-pre-implementation-audit.md PIA-003, PIA-020
supersedes: ["0079"]
superseded_by: []
related: ["0012", "0023", "0030", "0056", "0060", "0061", "0089", "0092", "0093"]
evidence: ["F36", "F39", "F27", "F34", "F35"]
consumers: []
---

## Context and problem

ADR-0079 recorded that the selection engine's context-vector source was **undecided**, left itself at
`status: proposed`, and **forbade being amended into a decision** — a successor had to decide it or
nothing did. Until that successor existed, no `require` row in any pack could be keyed on an LLM-emitted
`criticality`, `novelty`, `size`, `blast_radius`, `has_repro`, `external_dep` or `market_facing`, and
AC-071 — *"THIS IS THE UNBLOCKING GATE"* — failed pack load closed on any facet declaring
`source = "llm"`. That block is why Phase 1's kernel work could not start: the engine's *semantics* were
decided and buildable, but its *input* was not.

Two things had to be separated before the question could be answered, and conflating them is what an
earlier reading of the evidence did.

**What P1 arm L measures, and it is what decides this record.** METHOD-P §1.4 pre-registered a **modal
required-set identity** bar: across repeated draws at a fixed context, ≥ **80%** of items must derive an
identical required set `R(stage, ctx)`. This is the quantity that decides whether an LLM may supply the
context vector at all, because a selection engine whose required set changes between draws on the same
item is not a deterministic kernel with a noisy input — it is a non-deterministic kernel.

**What P1 arm D measures, and it does not decide it.** METHOD-P §1.4 also pre-registered a per-facet
precision bar of ≥ **0.95**, above which a facet leaves the classification record. Arm D scores a
**derived** context vector — a regex over paths and diff lines. ADR-0079 asks about an **LLM-emitted**
one. So arm D's result narrows this record's options without choosing among them, and a document that
rests the supersession on arm D has answered a different question.

A third thing was unassigned entirely, and the 2026-09-10 audit found it as **PIA-003**: `facets.toml`
carries **seven** `source = "llm"` facets, not the four ordinal axes. Three are booleans —
`has_repro`, `external_dep`, `market_facing`. Nineteen catalog and pack rows key on them, and the
kernel's rigor-monotonicity rule presumes `has_repro` is a live selection input. They had no disposition
between the pre-registered rule and the registry work that follows this record, so AC-071 would have
failed closed on three facets whose status was an oversight rather than a decision.

## Decision drivers

1. **The pre-registered rule must decide, not a rule written after the numbers.** METHOD-P §1.4 fixed
   both bars before any arm ran. This record applies them unchanged and records the results beside
   them, never in place of them.
2. **A decision must exist even when the measurement clears nothing.** ADR-0079's failure mode was that
   it recorded an open question and forbade its own amendment, so the block had no exit that was not a
   new record. The gate is discharged **by a recorded decision**, never by a passing number.
3. **AC-071 must have nothing left to fail closed on.** A criterion that fails load on `source = "llm"`
   is only honest if the registry carries no such facet; leaving three booleans at `llm` would make the
   gate fire on an oversight.
4. **Where a measurement is powerless, the honest destination is the one ADR-0079 already named** —
   human-supplied or human-confirmed at triage — not a weaker threshold, and not a bigger n on the
   critical path.
5. **A decision this record is not entitled to take, it must not take.** Source asymmetry and
   `stale_policy` are both live, and both belong in their own records.

## Considered options

**A — every ordinal axis and all three booleans are `source = "human"` for Phase 1**, supplied or
confirmed at the triage gate. ADR-0079's own "honest destination"; the hedge the architecture design
already ships; its operator cost is what SM-024 measures. Costs three extra triage questions per item.

**B — drop the three boolean facets from the registry.** Simpler, and it makes the `source = "llm"`
count zero by deletion. Rejected: it strands nineteen catalog and pack rows and the kernel's
`has_repro` rung, deciding a modelling question by removing the evidence of it.

**C — keep the seven at `source = "llm"` and gate on AC-071's stability record.** Faithful to the
criterion as written. Rejected: identity was measured at 0.25 / 0.15 / 0.10 / 0.10 against the 0.80
line, so no pack keyed on any of the seven would load in Phase 1 — this is option A's outcome with a
load failure standing in for a decision.

**D — raise the sampling temperature, or take more draws.** Rejected on the measurement: the
instability is schema-induced and was corroborated across five 2026 frontier models, with
Claude-Sonnet-4 measured at background temperature 0. There is no temperature to lower.

**E — commission the full P1 re-run (AC-071 clause C) and decide afterwards.** Legitimate, and the
re-run remains available. Rejected as the Phase-1 route: it puts a spike on the critical path of every
downstream stage in order to reach a destination option A reaches now, and Phase 1 proceeds on
human-sourced axes either way.

## Decision

**Option A.** Three numbered clauses.

**1. Every ordinal axis is `source = "human"` for Phase 1.** `criticality`, `novelty`, `size` and
`blast_radius` are supplied or confirmed by a person at the triage gate. The engine refuses an
LLM-emitted value for any of them.

**2. All three LLM boolean facets are `source = "human"` for Phase 1**, on the same rule and at the
same gate: **`has_repro`**, **`external_dep`** and **`market_facing`**. This clause was pre-registered
before this record was drafted, as `DECISIONS-2026-09-11` #2 — *"A: human at triage —
`source = "human"`, supplied or confirmed at the triage gate, the same rule as the ordinal axes;
pre-registered into s4 §4.1 now and carried into ADR-0090 at Step 4.4"* — against audit concern
PIA-003. The consequence is arithmetic and checkable: after the registry work that executes this
record, **no facet in `facets.toml` declares `source = "llm"`**, and AC-071 has nothing left to fail
closed on.

**3. ADR-0079 is superseded in full**, not amended. It decided nothing and forbade being amended into a
decision, so a successor replaces it whole. Both sides carry the link: this record's
`supersedes: ["0079"]` and ADR-0079's `superseded_by: ["0090"]`, with its status moved
`proposed → superseded`. That status change and that back-link are the **only** edits made to
ADR-0079; its body is untouched. They are permitted because ADR-0079 was `proposed` — the immutability
rule binds `accepted` records.

**What this record decides on, stated so it cannot be misread.** It decides on **clause A** — the
modal-`R` identity bar — which failed **4 times out of 4**. It does **not** decide on clause B. Arm D
measures a *derived* context vector and ADR-0079 asks about an *LLM-emitted* one, so arm D's scoring
narrows this record's options without choosing among them. Of AC-071's five clauses, arm D's scoring
**discharges the fourth only** ("arm-D panel scored"); the block stood, and still stands, on the first.

**Two things this record does not decide, and must not be read as deciding.**

- **Source asymmetry.** `commits` beats `docs` on every facet and `union` is **worse than `commits`
  alone** on two of them, because unioning a precise source with an imprecise one imports the
  imprecision. Which source a deriver trusts is registered as an arm of **S-FACET**, whose named
  consumer is `facets.toml`'s `source`/`deriver` fields. ADR-0089 explicitly does not decide it and
  neither does this record.
- **`stale_policy`.** That is P2's subject, not this one. P2 measures the `derived_from`
  reconstruction; METHOD-P §2.4's own remedies touch `stale_policy` and the `L`-class scope and
  **nothing in the selection path**. P2's criterion 1 fires on its precision half, the remedy it
  prescribes is contraindicated by the same run, and criterion 2 is not answerable — so `stale_policy`
  is an **OPEN** design consequence owed to its own record. It is not settled here, in either
  direction, and it is not this record's subject.

## Consequences

Triage gains three boolean questions per item on top of the four ordinal axes, and SM-024 is where that
operator cost is measured rather than assumed. The registry work that follows this record re-classes the
seven facets and tri-values the booleans, so that a boolean whose evidence is absent resolves to `null`
rather than to `false` (ADR-0089 decision 3, as narrowed by ADR-0093). AC-071 becomes a criterion with
no live subject in Phase 1 — which is the intended state, not a dead rule: it fails closed the moment a
pack reintroduces `source = "llm"` without a scored stability record.

The gate is **discharged**, and discharged is not unblocked. Every later stage checks that this record
exists, and that check asserts that a decision was taken — not that the block lifted. ADR-0089 is
**not** this token: it is accepted, it is about facet *provenance*, it partially supersedes ADR-0030 and
it explicitly does not touch ADR-0079. A reader who finds `0089` on disk and concludes the gate is
discharged has read the wrong number.

AC-071 clause C remains the only route that unblocks LLM-sourced ordinals, and it needs a full P1
re-run at n ≥ 20 items × ≥ 10 draws per item in addition to the panel. That re-run is **not** on Phase
1's critical path. Commission it or do not; Phase 1 proceeds on human-sourced axes either way.

**What this forbids**

- **No `require` row, predicate or rigor rule may be keyed on an LLM-sourced facet.** Not on
  `criticality`, `novelty`, `size` or `blast_radius`; and not on `has_repro`, `external_dep` or
  `market_facing` either. `kind` is out of scope of this record — it is human input under ADR-0030
  rule 4.
- **No facet may declare `source = "llm"` in a Phase-1 pack.** The count is zero, and it is asserted as
  zero rather than described as zero.
- **No facet may be promoted out of the classification record on this evidence.** The ≥ 0.95 branch is
  **unreachable at n = 20 for any facet however good the method**: `ui` is 7 of 7 and its 95% lower
  bound is 0.65, and **73** all-correct positives are needed for a lower bound above 0.95 at the
  registered two-sided z = 1.96 (92 with continuity correction; 72 under Clopper–Pearson). A document
  that offers promotion as a route is offering a route the measurement closed.
- **No arm-D figure may be cited as human-panel.** It is **PANEL-ADJUDICATED**, and the label travels
  with the number in every result table and at each document's first citation.
- **No facet with no positive instance may be reported as a pass or a failure**, and never as `0.00`,
  `1.00` or `n/a`. `data_migration` is **NOT MEASURABLE**.
- **This record may not be amended.** A reversal is a successor superseding `0090`, carrying both
  links. Nor may it be read as lifting AC-071's first clause, which no measurement in this programme
  has met.
- **A temperature change is not a remedy** and may not be offered as one.

## Evidence and confidence

**Clause A — the decisive measurement.** Modal required-set identity across repeated draws at a fixed
context, against the pre-registered 0.80 line: **5/20 = 0.25** and **3/20 = 0.15** (arm L v1, packs
`bug-standard` and `story`), **2/20 = 0.10** and **2/20 = 0.10** (the METHOD-P §1.4 anchor-rewrite
re-run, arm `L2`). Wilson 95% upper bounds **46.9 / 36.0 / 30.1 / 30.1** — every one far below the line.
`KILL fires` on all four; 400 dispatches, 0 void. Arm `L2` was confirmed as **kept** rather than
discarded (`DECISIONS-2026-09-12`, the Step 4 §4.2 precondition section) **before** this record was
drafted; the verdict is identical under either reading, which is why that confirmation changes nothing
here. [measured, F36; re-run 2026-09-12 at this pin]

**Clause B — applied unchanged, and it clears nothing.** Scored against 140 panel-adjudicated truth
cells at corpus pin `47faec9f`, union source, 20 items, Wilson 95% intervals: `ui` **1.00** [0.65, 1.00]
over 7 positives — the point clears 0.95 and **the interval does not**; `toolchain` 0.73 [0.48, 0.89];
`change` 0.72 [0.54, 0.85]; `binary_change` 0.60 [0.23, 0.88]; `flag_exists` 0.00 [0.00, 0.66] over 2
positives; **`auth` 0.00** — 0 of 11 [0.00, 0.26] on the 2026-09-09 scoring of 136 cells, and 0 of 12
[0.00, 0.24] after the 2026-09-11 panel-file key fix scored all 140 (ERRATA **E-0089-2**; no verdict
moved); `data_migration` **NOT MEASURABLE**, 0 positives on 20 items across all three sources, so
precision is undefined and a perfect method scores identically. **The sub-0.95 clause fires on five
facets. The ≥ 0.95 clause fires on none, and cannot at this n.** [measured, F39 §1]

**`auth` is not a threshold miss, and it is the load-bearing secondary result.** Eleven positives,
eleven false, upper bound 0.26: the rule carries **no signal** on this corpus. It is an unanchored
case-insensitive regex over paths **and** diff lines, marking `auth = true` on **14 of 20** items, and
its collisions are with the vocabulary of the instrument under test — `token` 44 hits of which **36**
are a design token; `auth` 43 of which **40** sit inside *authored*; `oauth` 8 of which **8** sit inside
*coauthor*; `session` 8 of which 6 are a *work* session; `role` 2 of which both are an ARIA attribute.
The failure is **correlated, not random**, so it does not average out with more data. Word boundaries
remove the `coauthor` and ARIA hits and leave the *authored* and design-`token` hits untouched — they
are whole words at proper boundaries. [measured, F39 §2]

**How the truth set was built, because "panel-adjudicated" is not "human-panel" and the difference is
the result.** 140 cells: **60** deterministic oracle (independently implemented over git metadata and a
canonical extension/manifest table); **39** unanimous model panel; **9** unanimous panel where the
adversarial flag was mechanically moot; **32 human adjudication** — blind, with the panel's and the
script's positions revealed only after each answer was entered (8 true, 24 false, 0 unknown, and 15 of
32 differ from the script). The blindness of those 32 rests on a **signed attestation** rather than a
machine-checkable timestamp, because the chronology proxy had expired: the engine ran before the labels
existed, by design. That is **strictly weaker evidence** and is recorded as such, never as
timestamp-verified. [measured, F39 §5; METHOD-P amendments 8, 9, 10(b)]

**F39's limits travel with its numbers and are not separable from them.** (1) The 0.95 bar is
unreachable at n = 20. (2) **39 of the 140 cells rest on unanimous 3-seat model agreement with no
human**, and small-panel unanimity fails in the correlated direction — a 9-judge panel was measured to
supply ~2.18 effective independent votes at 8–22 points short of independent-vote predictions
[documented, preprint, arXiv:2605.29800]. (3) The oracle is independent on `toolchain` and on
`binary_change`'s shortcut and **not on the rest** — for `change`, git's marker *is* the ground truth
for the binary component, so oracle and script agree there by construction. (4) **One repo, one
operator, one pin**; every figure is over `47faec9f`. (5) **A scorer defect was found and fixed before
this was reported** — the oracle emits `binary_change` as a JSON bool and the first scorer compared it
to the string `"true"`, scoring the facet 0.00 where it is 0.60. (6) **`unknown` is excluded, never
counted against the script**: 16 semantic cells are excluded on that basis, and counting them as
agreement would have raised every semantic precision figure.

**Absence of evidence, measured — and it is what the registry work exists for.** **4 of the 20 items
changed no code whatsoever**; every changed file is bookkeeping or telemetry. The panel returned
`unknown` on all **16** of their semantic-facet cells, and **the script emitted a confident boolean on
every one**. [measured, F39 §4]

**ADR-0079's external corroboration is corrected here rather than repeated.** Per ERRATA **E-0079-1**:
arXiv 2603.04417 is Sonnet-4.5 and arXiv 2604.22411 is Sonnet-4, and **neither states a
"schema-induced" mechanism**; arXiv 2511.01176's boundary sentence is an **unmeasured premise**; arXiv
2508.16445 v2 is retitled and makes **no determinism claim**. F36's cost is **$10.71** (haiku $4.91 +
sonnet $5.79), not the $9.70 ADR-0079's line 211 carried. The schema-induced mechanism is therefore
retained as **[inferred, F36 + ADR-0079]**, not as documented external corroboration.
[documented, full text fetched 2026-09-10; measured, F36 log lines]

**Confidence: high on the decision, on a powerless measurement.** That is the honest pairing, not a
contradiction. Clause A fails by a wide margin under every reading — 4 of 4, upper bounds ≤ 46.9
against a 0.80 line — so the destination is not in doubt. What is **not** established is the converse:
no measurement in this programme shows that an LLM-emitted context vector *cannot* work, because clause
B's threshold was never reachable at the registered n and clause A's re-run at n ≥ 20 × ≥ 10 has not
been commissioned. This record decides Phase 1's input. It does not close the question for Phase 2, and
AC-071 clause C remains the route that would reopen it.
