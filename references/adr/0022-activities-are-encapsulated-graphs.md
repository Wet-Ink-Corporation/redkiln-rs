---
id: "0022"
title: Activities are encapsulated graphs; technology packs contribute producers, not stages
status: accepted
date: 2026-09-07
legacy_id: D22
source: references/2026-09-05-architecture-proposal.md §9 (D22, "from PC6/PC7"); references/2026-09-07-process-catalog-proposal.md §7 (PC6, PC7)
supersedes: []
superseded_by: []
related: ["0014", "0021", "0028", "0029", "0073"]
evidence: []
consumers: []
---

## Context and problem

D14 already drew the line that process packs own structure and technology packs bind/contribute at slots, never restructure. Once activities were modelled as graphs of internal steps (the activity-graph schema), a second, sharper version of the same tension appeared inside a single activity: should an activity's internal quality machinery — self-review, retry, escalation — be visible to the stage as sibling nodes the stage schedules, or should it be invisible, with the stage seeing only the activity's finished output? And separately, when a technology pack needs to add toolchain-specific work (a cook step for Unreal, a binary change summary), is that a new stage, a new producer, or something else? Left unresolved, either question risks technology packs restructuring workflow shape by the back door, which is exactly what D14 forbade at the pack level.

## Decision drivers

- D14's existing rule ("process packs own structure... a different shape is a process-pack variant, never a merge") must extend cleanly to the inside of an activity, not be bypassed by moving the contested behaviour one level down.
- A stage should not need to know or schedule an activity's internal review/retry/escalation machinery; that machinery is the activity's own responsibility for its output quality.
- Escalations to a human must remain durable and kernel-tracked even though they originate inside an activity's internal graph.
- Technology packs legitimately need to contribute toolchain-specific facts, producers, and artifact kinds (Unreal's cook/asset-manifest work, binary change summaries) without being given the power to add stages or edit `require` rows.

## Decision

An activity owns the quality of its own output: internal review, retry, and escalation are invisible to the stage that calls it ("Contract = 'materialise artifact X to the declared bar, with evidence'; the stage sees only `Present(X, evidence)`" — PC6). Nothing about internal review, retries, or escalation leaks into stage or workflow definitions. Escalations remain kernel-owned: an activity-internal HITL escalation is still a kernel-owned durable `Gate:` mark (§4.6) — the activity *declares* escalation points, the kernel *executes* them.

A technology pack may contribute to this model only along the PC7 contract: (i) add **facts** to the context vector (`toolchains`, `change_kinds`, `binary_change`), computed per item from affected paths via `Toolchain::classify`, never per repo; (ii) contribute **producers** for existing artifact kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `ProfileReport`, ...), selected by specificity, with equal-specificity ties treated as a validation error (the same rule as §4.2.1 card selection); (iii) contribute additive **artifact kinds** (`CookReport`, `AssetManifest`, `AutomationReport`, `Captures`) and additive `contribute` rows at a declared stage; (iv) override `timeout_secs`/`cost_class` per activity. A technology pack may **not** add stages, remove baseline artifacts, or edit `require` rows — D14 is unchanged.

## Considered options

### Option — stage-visible reviewer nodes
Make an activity's internal reviewer, retry, and escalation logic visible as sibling nodes at the stage level, so the stage schedules and sees them directly. Rejected by PC6: this leaks quality machinery into stage/workflow definitions, breaking the encapsulation contract and reintroducing at the step level the exact ambiguity D14 already closed at the pack level.

### Option — per-toolchain catalogs
Give each technology pack (Node, .NET, Unreal, ...) its own separate activity catalog rather than a shared agnostic catalog that technology packs contribute producers into. Rejected: this duplicates the toolchain-agnostic activities (discovery, spec, design, planning, closeout — "most of the ~95") across every catalog, and gets toolchain facts wrong for mixed repos, since facts are computed per item from affected paths, not per repo.

## Consequences

**Positive**
- A stage's definition stays toolchain-agnostic and simple: it asks for an artifact at a declared bar with evidence, never how that bar was met internally.
- Toolchain-agnostic activities (the majority of the catalog) ship once; only the toolchain-realised ones (implement/verify/review/release producers, `bisect`, `minimal-repro`, `profiling`) need per-toolchain specialisation, as a generic producer with an `applicable_if` guard plus a per-pack specialisation.
- Escalations inside an activity are still durable and auditable via the kernel's `Gate:` mechanism, so encapsulation does not hide human-in-the-loop moments from the event log.

**Negative**
- Mixed-toolchain repos (e.g. Node front-end + Unreal content) need both change-summary producers to run per item, per PC7's explicit example, which is more producer-selection machinery than a single-toolchain repo needs.
- Equal-specificity producer ties are hard validation errors rather than resolved by any tie-break heuristic, so pack authors must design selectors to avoid collisions.

**What this forbids**
- A technology pack may not add a new stage, remove a baseline artifact, or edit a `require` row, under any circumstance — that remains a process-pack variant (e.g. `story-ue`), never a pack merge.
- A stage definition may not schedule or reference an activity's internal review, retry, or escalation nodes directly; it may only consume the activity's declared output artifact and evidence.
- Two producers or cards at equal selector specificity may not be silently resolved by ordering or preference; that condition is a configuration error.

## Evidence and confidence

- [documented] This decision is a synthesis point: the row and its source (PC6, PC7) state the model precisely but the ledger row cites no spike or measurement number for D22 itself — the empirical backing is deferred to the related spike P7 ("encapsulated quality, cost vs catch", registered in the catalog proposal §6.1), which asks whether an activity's internal adversary + policy catches more per token than a single dispatch per artifact. The source proposal records no P7 result — it carries result paragraphs for P9 and P9b only — so no P7 measurement backs this record either way. Evidence array is left empty because the row itself cites none.
- [documented] The specificity clause restated above is later narrowed: activity-graph-schema amendment 43 (ADR 0073) withdraws "specificity beats generic" for toolchain-keyed artifact kinds, expanding a required keyed kind to one instance per toolchain instead. This record is unchanged as to clauses (i), (iii) and (iv) and to non-keyed kinds; see ADR 0029 for the scoped withdrawal.
- [documented] PC7's specificity tie-break rule is stated as "the same rule as §4.2.1 cards" — an explicit cross-reference to D14/§4.2.1's card-selection mechanism, not an independently derived rule.
- This record folds PC6 and PC7 in as the decision's full text, following the D22 row's own pointer ("Full text: catalog proposal §7 PC6–PC8; `2026-09-07-activity-graph-schema.md`"). PC6 (ADR 0028) and PC7 (ADR 0029) keep their own records and are listed as **related**, not superseded — the row restates them, it does not replace them. The row's pointer also names PC8, which is the separate facets decision (ADR 0030) and is not part of this record's text.
