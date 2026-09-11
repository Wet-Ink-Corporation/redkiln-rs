---
id: "0027"
title: Curated ~95-activity catalog at v0, with unshipped extended file
status: accepted
date: 2026-09-07
legacy_id: PC5
source: references/2026-09-07-process-catalog-proposal.md §7 PC5
supersedes: []
superseded_by: ["0114"]
related: []
evidence: ["EPF rot from unexecuted content", "Essence \"start minimal\"", "superpowers' too-many-skills failure"]
consumers: []
---

## Context and problem

The raw activity catalog (`research/2026-09-07-activity-catalog-raw.md`) runs to roughly 267 rows once every stage's candidate methods and unordered activities are counted. Shipping all of it risks the failure mode the proposal names explicitly: content that is never executed rots (EPF), and unused options carry a real authoring and lint cost even when the engine's own selection is deterministic. The question the row answers is how much of that raw list ships in the default pack at v0, and what happens to the rest.

## Decision drivers

- EPF's documented pattern of content rot from activities that are authored but never executed.
- Essence's "start minimal" guidance as a countervailing pull against premature completeness.
- Deterministic selection (§2.5's pure function) means an unused activity costs nothing at *runtime* — the risk it poses is at *authoring and lint* time, not execution time. This is explicitly distinguished from the `superpowers` too-many-skills failure, which is called out in the row as LLM-selection-specific and therefore not applicable here.
- Need for full kind × stage coverage so every artifact kind has at least one admissible producer at every reachable context (the §2.6 lint requirement).

## Decision

Ship a curated **~95**-activity default pack at v0, with full kind × stage coverage across the twelve stages (§4's table: triage, discovery, analysis, specification, design, planning, implement, verify, review, release, operate, closeout). The remaining **~170** raw rows stay in an unshipped `extended` file — not deleted, not shipped, held out of the default pack's lint and execution surface.

Two conditions gate what is allowed into the shipped ~95: "Every shipped activity must pass P6 lint (reachable from some context) and be executed at least once before the pack is tagged."

## Considered options

### Option — phase-aligned ~40→95
Ship a smaller phase-1 slice (~40 activities) and grow toward ~95 as phases advance. Rejected in favour of shipping the full curated ~95 at v0 directly — the row does not explain why phasing lost, but it is listed as a considered and rejected alternative.

### Option — everything (~267)
Ship the full raw catalog unfiltered. Rejected on the EPF rot evidence: unexecuted content in a catalog degrades over time, and shipping ~267 rows without a reachability/execution gate would guarantee dead content at v0.

### Option — minimal ~20 + decision memory
Ship a much smaller core (~20 activities) and rely on the decision-memory mechanism (§2.4's per-context override replay) to grow coverage incrementally. Rejected in favour of the curated ~95 — the row does not give the specific reason this lost, only that it was considered.

## Consequences

**Positive**
- Full kind × stage coverage is guaranteed at v0 rather than grown incrementally, so the pack lint (§2.6: every stage-exit artifact has ≥1 admissible producer for every reachable context) can pass on day one.
- The P6 lint + at-least-once-executed gate on every shipped activity is a concrete, checkable bar against EPF-style rot, applied before the pack is tagged rather than after content has already accumulated unused.
- Keeping ~170 rows in an `extended` file preserves optionality without paying the authoring/lint cost of shipping them.

**Negative**
- ~170 raw rows are held out of the shipped pack, meaning any context that would need one of them at v0 has no admissible producer until the extended file is promoted or the pack is revised.
- The "at least once executed" requirement is a promotion gate for the *pack tag*, not a per-request guarantee — it says content must have run before the pack is tagged, which the corpus notes elsewhere (§7 consequences) makes "spike P6 becomes a release gate for the pack."

**What this forbids**
- It forbids shipping the full ~267-row raw catalog at v0.
- It forbids tagging the pack with any shipped activity that has not passed P6 lint (reachable from some context) or has never been executed — an activity that is authored but dead-on-arrival cannot ship.
- It forbids treating "everything is deterministically selected, so authoring it costs nothing" as license to skip curation — the row explicitly separates runtime cost (zero, under deterministic selection) from authoring/lint cost (real, and the reason for curation).

## Evidence and confidence

- [documented] EPF's rot-from-unexecuted-content failure mode is cited as grounding for the curation decision; no number is given for EPF's own rot rate.
- [documented] Essence's "start minimal" principle is cited as a countervailing consideration; not a metric.
- [inferred] "unused activities cost nothing at runtime under deterministic selection" is the row's own reasoning, presented as a conclusion from the selection model (§2.5) rather than a spike measurement.
- [documented] The `superpowers` too-many-skills failure is explicitly distinguished as "LLM-selection-specific" and therefore judged not to transfer to this catalog's deterministic, non-LLM selection mechanism — this is an argument by disanalogy, not a measurement on this system.
- [documented] The only count the source states outright is the raw catalog's 267 sourced activities (proposal header, `research/2026-09-07-activity-catalog-raw.md`). "~95" and "~170" are the row's own approximate split of that 267; no spike or measured count backing that particular split is cited, so the two figures are planning estimates, not results.
