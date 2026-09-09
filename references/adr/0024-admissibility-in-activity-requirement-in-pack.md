---
id: "0024"
title: Admissibility lives in the activity, requirement lives in the pack
status: accepted
date: 2026-09-07
legacy_id: PC2
source: references/2026-09-07-process-catalog-proposal.md §7
supersedes: []
superseded_by: []
related: ["0014"]
evidence: ["SPEM content/process split", "EPF clone-and-own failure", "Provop context rules on the base process"]
consumers: []
---

## Context and problem

A single rubric mixing "what an activity is capable of" with "what a given stage on a given kind of item demands" would tie the catalog's reusable content to any one pack's policy, or force every pack to redeclare admissibility facts the catalog already knows. The process-catalog proposal's §2 model needed a home for each half that let one catalog serve many packs (D14's existing pack-composition split motivates the same separation one level down).

## Decision drivers

- D14 already separates process-pack structure from technology-pack contribution; PC2 extends that split inside the rubric itself.
- SPEM's content/process separation is cited as the structural precedent.
- EPF's clone-and-own failure mode — where packs fork the whole content model to change policy — is the failure this split is meant to avoid.
- Provop's context-rule mechanism, which keeps variability in context rules layered over a base process rather than forked into it.

## Considered options

### Option — all-in-activity
Fold requirement (`baseline`, `require` rows) into the activity record itself. Rejected: this would make an activity's own eligibility facts (`provides`, `requires`, `applicable_if`, `method_rank`, `min_rigor`) inseparable from a specific pack's policy, defeating catalog reuse across packs — the EPF clone-and-own failure mode.

### Option — all-in-pack
Move admissibility facts (what an activity provides, needs, and when it applies) into the pack as well. The source row lists this option but gives no reason it lost; the reason stated here — that it would make the catalog non-authoritative about its own content and duplicate admissibility facts across every pack using the activity — is read off the decision's own "catalog content, pack-independent" framing, not quoted from the source.

## Decision

**Split**: admissibility — `provides / requires / applicable_if / method_rank / min_rigor` — lives on the activity (catalog content, pack-independent, §2.2). Requirement — `baseline` and `require` rows, §2.3 — lives in the process pack (pack policy). `redkiln explain` prints both, so the two halves stay observable as one explanation even though they are authored in different places.

## Consequences

**Positive**
- One activity can serve many packs' differing policies without being forked.
- A pack's `require` rows read as pure policy, auditable independently of catalog content.

**Negative**
- Understanding why an activity ran requires reading two artifacts (activity record and pack rubric) rather than one; `redkiln explain` exists specifically to paper over this cost.

**What this forbids**
- A pack editing an activity's admissibility facts (`provides`, `requires`, `applicable_if`, `method_rank`, `min_rigor`) to express its own policy — that is not an option this decision leaves open (echoing D14, which already forbids tech packs from defining activities directly).
- An activity declaring its own `baseline`/`require` obligations — those belong to the pack, never the activity.

## Evidence and confidence

- [documented] SPEM's content/process split and EPF's clone-and-own failure are cited as prior-art grounding; the proposal does not report a spike measuring this split's effect on this corpus specifically.
- [documented] Provop's context-rule layering over a base process is cited as the mechanism precedent for keeping requirement separate from content.
- [inferred] The claim that this split prevents an EPF-style content fork is reasoned from the cited failure mode, not measured against this catalog's own packs. The source row assigns no reason to either rejected alternative; both reasons above are reconstructions from the grounding column.
- [documented] No spike in the P-series is scoped to test this split, so the decision is not conditional on an unrun spike. The nearest downstream check is **P6** (catalog lint coverage), which exercises whether the shipped catalog is satisfiable for every reachable context — it would surface consequences of the split, not adjudicate it.
