---
id: "0079"
title: The selection engine's context-vector source is undecided; Phase 1 pack authoring is blocked on P1's human panel
status: superseded
date: 2026-09-08
legacy_id: none
source: spikes/F36-DRAFT.md (P1 arm L, v1 and the §1.4 re-run); spikes/METHOD-P.md §1; references/2026-09-08-evidence-augmentation.md §3 (method engineering), §4 rec 21
supersedes: []
superseded_by: ["0090"]
related: ["0012", "0030", "0056", "0060", "0061"]
evidence: ["F36", "F27", "F34", "F35"]
consumers: []
---

## Context and problem

The selection engine is a pure function: a context vector `ctx` determines a required-artifact set `R(stage, ctx)`, and each
missing artifact resolves to a producer by `applicable_if` and lowest `method_rank`
(`references/2026-09-07-process-catalog-proposal.md` §2.5 steps 3–4). Everything downstream of that — pack authoring, the
lattice lint, the memo key, and the whole determinism claim the inversion architecture rests on — assumes `ctx` is
reproducible. PC8 (ADR 0030, rule 2) already requires every facet to map to a computed fact in the context vector. It never
said *what computes it*. The v0 design has a model fill a closed classification form (catalog §2.4: five ordinal axes plus
three booleans) and the engine do all the rest.

P1 arm L measured that half of the vector, and the result is negative and unambiguous. [F36] Four hundred classification
dispatches — 20 items × 5 repeats × 2 models (haiku 4.5, sonnet 5) under card `9d86e7044490`, then a further 200 under the
§1.4 anchor-rewrite card `0384080216a9` — 0 void, $21.37, corpus pin `47faec9f`:

- **The engine-facing criterion fires four times out of four.** The fraction of items whose modal `R(stage, ctx)` is
  identical across all ten draws is **5/20 and 3/20** (packs `bug-standard` and `story`) in the first instrument and
  **2/20 and 2/20** in the second, against an 80% line.
- **The prescribed remedy did not repair it.** METHOD-P §1.4 licenses one anchor rewrite per failing axis. `blast_radius`
  moved 0.488 → 0.473 — the remedy is exhausted and, on the pre-registered rule, that axis becomes human-supplied.
  `external_dep` became **constant**: 200/200 `false`, 0 `true`, α reported as undefined because no disagreement was
  available to measure. A facet with no variance still keys a `require` row and can never change the answer; it is
  unanimity by collapse, and it looks healthier than the unreliable version it replaced.
- **The per-axis threshold is not a decidable predicate on this corpus.** Three axes whose anchors were **byte-identical
  between the two runs** drifted by −0.168 (`criticality`), −0.074 (`size`) and −0.066 (`has_repro`). That drift is larger
  than the effect the α < 0.6 threshold exists to detect, and it carried `size` (0.604 → 0.530) across the kill line
  without a word of its anchor changing. An axis can be moved across the threshold by resampling alone.
- **The mechanism is the `unknown` band.** Per-axis `unknown` rates run 27.0%–46.0% (`novelty` 46.0%, `size` 39.5%,
  `blast_radius` 31.0%, `criticality` 27.0%). At selection, `unknown` is treated as `high` by position (F34 B14, F35 §1.6;
  F35 measured 23 activities selected at `<axis> = unknown` and not at `low`), so an axis oscillating `unknown ↔ low`
  moves `R` by the full width of the band. Four axes each flipping on a minority of draws compose into a required set
  that rarely repeats — which is why per-facet α alone (four of eight axes clear 0.6 comfortably) makes the instrument
  look far healthier than it is.

The other half of P1 has **no result**. Arm D's precision and recall — the criterion that decides whether a facet leaves
the model at all — are **not computed**, because METHOD-P §0 rule 1 requires the human agreement panel to be written from
the items alone, timestamped, *before* the engine output is unblinded. The panel skeleton
(`spikes/results/p1-arm-l-labels-RYAN-TODO.md`) is unfilled. Choosing the source of the facets now would mean choosing
without the only measurement that bears on the choice.

## Decision drivers

- The measured failure is in `R`, not in any individual axis, so a per-axis fix cannot be assumed to reach it, and the
  per-axis statistic that would evaluate such a fix is itself not resolvable at this sample size.
- D12 (ADR 0012) requires an open question to be carried as a named, gating spike rather than as a hedge in prose. This
  record is that discipline applied to the largest remaining unknown in the design.
- Blinding is a measurement precondition, not a formality: unblinding arm D before the panel is written destroys the only
  evidence that could answer the question, and the loss is irreversible.
- Every candidate source has an unpriced cost. Human-supplied facets buy determinism with gate load, and F18 measures a
  human gate at a median of 10.5 minutes with 45% exceeding an hour; a triage gate on every item is a real tax that
  nothing here has weighed.
- Coarser bands are load-bearing in **both** directions: F35 §1.4 already showed a two-band reading made `canary-release`
  unreachable, so widening bands trades instability for dead activities at an unmeasured exchange rate.
- The engine's own semantics are not implicated by any of this and must not be held hostage to it.

## Decision

**This record decides nothing about the source of the LLM-emitted facets. It records the block, fixes what is decided
around it, and states the condition that lifts it.**

**1. Phase 1 pack authoring over the five ordinal axes and the three boolean risk facets is BLOCKED.** No `bug-standard`
or `story` pack may declare a `require` row keyed on `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`,
`external_dep` or `has_repro` sourced from an LLM classification dispatch, until the unblocking precondition below is
satisfied. `kind` is out of scope of this block: it is a human input already held in lockstep with the tracker's
`rk:kind/*` labels by ADR 0030 rule 4, not a model judgement.

**2. The engine's semantics ARE decided and buildable, and are not blocked.** They are settled independently of where the
vector comes from, and Phase 1 may build against them now:

- **Purity.** The agent fills the classification form and may raise a typed deviation request. It never chooses an
  activity, never edits `R`, and never interprets a free-text rule (process-catalog-proposal §2.5).
- **Rank.** Among admissible producers of a kind, lowest `method_rank` wins (ADR 0056).
- **Tie is an error.** A rank tie among admissible producers is a pack validation error, never a runtime choice
  (ADR 0056, ADR 0058).
- **Unsatisfiable is a gate.** No candidate producer yields `Unsatisfiable(kind, failed_predicates)`, which escalates to a
  human gate rather than degrading silently.
- **Deviation never edits `R`.** A deviation is requested through the typed `on_false.to = "deviate"` construct and
  evaluated by the kernel against its `claimed` predicate (ADR 0046); it is never a write to the required set.
- **Replay.** An engine or lint result is a function of `(catalog digest, facets digest, item digest)` recorded on the
  result line, or it is void (METHOD-P §0 rule 2).
- **Lint.** The lattice validator is real and cheap: F35 measured 0.3% unsatisfiability, 6/102 dead activities, **0** rank
  ties and **0** hard cycles over the v0 catalog, in 27 s.

That list is what makes the block survivable. The unstable thing is the *input*, and the engine is buildable, testable
and lintable against a vector supplied by any source.

**3. Unblocking precondition — executable, with a named owner.** The block lifts when all four of the following hold, in
order. **Owner: Ryan Britton.** Items (a) and (b) are human acts an agent may not perform.

- **(a)** `spikes/results/p1-arm-l-labels-RYAN-TODO.md` carries two labels per item per facet, written from the item alone,
  with a file timestamp preceding the first arm-D scoring run (METHOD-P §0 rule 1). If the second labeller is the same
  person at ≥ 24 h separation, that is disclosed on the file.
- **(b)** No agent, script or gate reads arm D's output before (a) is committed. Unblinding first voids the panel.
- **(c)** `spikes/p1/score-arm-d.py` is run against that panel and reports per-facet precision and recall. On METHOD-P
  §1.4's rule, a derived facet at precision ≥ 0.95 leaves the classification record; below 0.95 it stays LLM-emitted.
- **(d)** A source is chosen per axis and re-measured against the **modal-`R`** criterion — identical modal `R` on ≥ 80%
  of items across ten draws, on the `story` pack. Not against α: F36 §3.3 disqualified α at this corpus size, and the
  modal-`R` figure is computed from the engine's own output rather than from an inter-rater statistic.

Until (d) reports, the axis-source question stays open and this record stays `proposed`. It is superseded by the record
that answers it, not amended into a decision.

**4. Do not spend a spike on decoding parameters.** No temperature, seed or sampling-control arm may be registered against
this instability. The next diagnostic on the existing 400 draws is the **boundary-distance** analysis of
evidence-augmentation §4 rec 21: bucket each draw by its distance from the ordinal band boundary and test whether
instability concentrates there. It is offline, costs no dispatches, and if it holds it points at a narrow mandatory
human-confirm zone near the boundaries rather than routing whole axes to a human.

## Considered options

Four options are on the table and **none is eliminated**, because the measurement that separates them (arm D against the
panel) has no result. Each is recorded with what it would cost and what it is still missing.

### Option — derived and human-supplied facets only
Remove the model from the context vector entirely: compute what a script can compute from paths and diffs (arm D's
`toolchains`, `change_kinds`, `binary_change`, `auth`, `data_migration`, `ui`, `flag_exists`), and take the rest from a
human at a triage gate. Not chosen yet because the size of the derived half is exactly what arm D has not measured: if
precision lands below 0.95 on most facets, this option is not "derived plus a little human input", it is a mandatory
human classification form on every item, at a gate cost F18 prices at a median 10.5 min with 45% over an hour, and nothing
has weighed that against the alternatives.

### Option — LLM classification as a proposal, gated to a human on any axis feeding a require row
Keep the classification dispatch, but treat its output as a *proposal*: any axis whose value feeds a `require` row is
confirmed by a human at triage before `R` is computed. This preserves the form-filling role the design already assigns the
model, and it is the shape the external literature independently converges on — evidence-augmentation §2 records a 2025
applied paper building an LLM tool on Essence alphas that reached the same LLM-fills-the-form / engine-decides split as
the load-bearing choice [preprint, arXiv 2508.16445]. Not chosen yet because its gate load is a direct function of how
many axes survive arm D, and because the boundary-distance diagnostic (decision 4) could shrink the confirm set from
"every require-feeding axis" to "draws near a band boundary" — which would change this option's cost by a large and
currently unknown factor.

### Option — coarser bands
Rewrite the rubric rows over the unstable axes to fewer, wider bands, on the theory that a coarser target is a stabler one.
This is METHOD-P §1.4's own second consequence and it is live. Not chosen yet because nothing here measures a two-band
engine, and because F35 §1.4 measured the cost in the other direction: under two bands `canary-release` became unreachable.
Banding trades instability for dead activities and the exchange rate is unmeasured.

### Option — the axis is dropped
Delete an axis from the facet registry and from every `require` row that keys on it. Not chosen yet because dropping an
axis is not free either — F35 measured that 23 activities are selected at `<axis> = unknown` and not at `low`, so removing
an axis silently changes which activities run, and no measurement says which of those changes are wanted. This option is
nevertheless the honest destination for `external_dep` if nothing rescues it: a facet that is constant at 200/200 already
contributes nothing to `R` while still occupying a `require` row.

**What the evidence currently favours, and why that is still not a decision.** The evidence favours a *hybrid*:
`blast_radius` human-supplied (it exhausted §1.4's remedy without moving), `external_dep` off the model (neither its
unreliable v1 nor its collapsed v2 supports keeping it), whatever arm D clears at ≥ 0.95 derived, and the remainder
LLM-proposed with a human confirm on any require-feeding axis. It is not a decision because three of its four components
are functions of numbers that do not exist yet: the derived half is unmeasured, the confirm zone's width depends on the
boundary-distance diagnostic, and no candidate configuration has been re-measured against the modal-`R` criterion that
disqualified the current one. Recording the preference and calling it a decision would be the same error the α threshold
made — acting on a number the instrument cannot resolve.

## Consequences

**Positive**
- Phase 1 gets an unambiguous, executable statement of what it may build now (the engine, the lint, the lattice) and what
  it may not (packs keyed on LLM-emitted axes), instead of a hedge.
- The blinding rule is protected by an explicit prohibition rather than by convention, so the one irreversible way to
  destroy the deciding measurement is now a written forbid.
- Two axes are effectively settled by the pre-registered rule even though the record does not close the general question,
  so the eventual decision starts from a smaller problem.
- A named human owner and a four-part precondition make the block auditable: anyone can check whether it still holds.

**Negative**
- Pack authoring — a large, on-critical-path Phase 1 workstream — is stalled on a human act with no deadline attached, and
  this record does not give it one.
- The record ships a stated preference without the authority of a decision, which is a real hazard: a reader in a hurry
  may act on the preference as though it were normative.
- Keeping four options open carries the cost of keeping four options implementable; the engine must not assume any
  particular source of the vector, which forbids some optimisations that a settled source would allow.
- The modal-`R` re-measurement in (d) is a further 200-dispatch-class run at roughly $10 per instrument, so the
  unblocking path is not free.

**What this forbids**
- Forbids authoring or shipping a Phase 1 pack whose `require` rows key on `criticality`, `novelty`, `size`,
  `blast_radius`, `market_facing`, `external_dep` or `has_repro` sourced from an LLM classification dispatch, until the
  precondition in decision 3 is satisfied.
- Forbids any agent, script or gate reading arm D's engine output before the human panel is written and timestamped —
  unblinding voids the panel and the measurement cannot be recovered.
- Forbids treating per-facet Krippendorff's α as a decidable predicate on this corpus, in either direction: an axis may
  not be passed *or* failed on an α reading at 20 items × 10 draws without a bootstrap interval over items.
- Forbids reading `external_dep`'s v2 unanimity (200/200 `false`, α undefined) as reliability. A constant facet is not a
  stable facet, and a facet with no variance may not key a `require` row.
- Forbids registering a temperature, seed or other decoding-parameter arm against this instability.
- Forbids amending this record into a decision. It is superseded by the record that answers the axis-source question, per
  the immutability rule; a `proposed` record is not a draft to be filled in.

## Evidence and confidence

- [measured] F36, arm L v1: 200 dispatches (20 items × 5 repeats × 2 models), 0 void, card `9d86e7044490`, schema
  `5fd33472d73e`, `claude` 2.1.263 on all 200 records, corpus pin `47faec9f`. Modal-`R` identity 5/20 (`bug-standard`) and
  3/20 (`story`) against an 80% line. Per-axis α: `kind` 0.847, `criticality` 0.806, `has_repro` 0.784, `novelty` 0.745,
  `market_facing` 0.721, `size` 0.604, `blast_radius` 0.488, `external_dep` 0.165.
- [measured] F36 §3, the §1.4 re-run: a further 200 dispatches under card `0384080216a9`, 0 void, never pooled with v1.
  Modal-`R` identity 2/20 and 2/20. `blast_radius` 0.488 → 0.473; `external_dep` → 200/200 `false`, α undefined; and on
  byte-identical anchors `criticality` −0.168, `size` −0.074, `has_repro` −0.066. Combined spend across both instruments
  $21.37 (F36 reports $10.66 + $9.70; the ledger's arm-L total is quoted as $21.37 and the small residual is not
  reconciled in the source — nothing here turns on it).
- [measured] `unknown` rates over 200 draws: `novelty` 46.0%, `size` 39.5%, `blast_radius` 31.0%, `criticality` 27.0%.
  The `unknown`-is-`high`-by-position behaviour and the 23 activities selected at `unknown` and not at `low` are F34 B14
  and F35 §1.6.
- [measured] F35: the lattice validator over the v0 catalog reports 0.3% unsatisfiability, 6/102 dead activities, 0 rank
  ties, 0 hard cycles, 27 s runtime. This is the buildability half of decision 2 and it is a real measurement, not an
  assurance.
- [inferred] F36 §1.3's attribution of `external_dep`'s α 0.165 to an anchor defect rather than model noise is the
  draft's own reasoning and is tagged `[inferred]` there; the v2 collapse is consistent with it but does not confirm it.
- [preprint] External corroboration, carried from evidence-augmentation §3 (method engineering) and §4 rec 21: *Same
  Input, Different Scores* (arXiv 2603.04417) tested GPT-4o, GPT-4o-mini, Gemini-2.5-Flash, Claude-Haiku-4.5 and
  Claude-Sonnet-4.5 and found that "despite expectations of stability at temperature=0, substantial variability
  persisted across models", with temperature=0 having limited or inconsistent effects for the Anthropic models
  specifically. *Introducing Background Temperature* (arXiv 2604.22411) measures Claude-Sonnet-4 at T_bg = **0** —
  perfectly deterministic in that paper's harness. *The Necessity of Setting Temperature in LLM-as-a-Judge*
  (arXiv 2603.28304) finds temperature modulates the sharpness of the verdict distribution but does not change the
  ranking of verdict candidates. Together these put P1's instability on the **schema and prompt** side rather than the
  decoding side, which is the basis for decision 4.
- [preprint] The recommended replacement diagnostic is *An Empirical Study of LLM-Based Code Clone Detection*
  (arXiv 2511.01176), which reports classification instability correlating with proximity to the decision boundary. It is
  a different task on a different corpus; it motivates the diagnostic, it does not predict its outcome here.
- **Where the corpus and the external evidence pull apart.** They do not contradict each other on the finding — F36's
  mechanism claim and the external papers agree that the instability is anchor- and schema-shaped. What the external
  evidence removes is an escape hatch the corpus leaves implicitly open: that a decoding control might yet rescue the
  axes. This record acts on the external evidence and forbids that spike. The limit is stated plainly: arXiv 2604.22411
  measures **Claude-Sonnet-4** in **its own** harness, and P1 arm L ran haiku 4.5 and sonnet 5 at default temperature
  through the `claude` CLI. The papers therefore license "a temperature arm is very unlikely to be informative", not
  "our draws were deterministic at T=0".
- **What this evidence does NOT license.** It does not license any statement about whether a facet can leave the model:
  arm D's precision and recall are not computed and the human panel is unwritten. It does not license a claim that
  coarser bands would fix `R` — no two-band engine was measured. It does not license reading the `bug-standard` modal-`R`
  figure as sound: F36 limit 2 records that it rests on 2 bug cards in the frozen corpus, and names the `story` figure
  (3/20, 165 story cards available) as the defensible one. It does not license generalising beyond one corpus at one pin
  (`47faec9f`), 20 items rather than METHOD-P §1.2's 30, with anchors written by the author of the schema they serve — a
  disclosed, unmitigated instrument bias. And per the research pass's own discipline, nine agents finding nothing against
  a claim means "no current source contradicts it", not "confirmed"; nothing external here confirms redkiln's design, it
  only reproduces the phenomenon.
