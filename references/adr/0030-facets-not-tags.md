---
id: "0030"
title: Activity classification is closed facets compiling to applicable_if, not free tags
status: accepted
date: 2026-09-07
legacy_id: PC8
source: references/2026-09-07-process-catalog-proposal.md §7 PC8
supersedes: []
superseded_by: ["0089"]
related: ["0020"]
evidence: ["V-Modell XT typed project characteristics", "Provop context model with enumerated value ranges", "C-EPC facts", "competitive §B9, §D lesson 3"]
consumers: []
---

## Context and problem

Free-form tagging is a well-known failure mode in skill/activity marketplaces (skill-marketplace and Jira-label sprawl) and in description-matching selection systems (`superpowers`'s failure), because tags accumulate without a closed vocabulary and because tag-based selection reintroduces an LLM-shaped "which activities apply?" question that this catalog's deterministic selection (§2.5) is designed to avoid. PC8 settles how activity classification is represented so that selection stays a pure function over closed, computable facts rather than free text or LLM judgement.

## Decision drivers

- Avoiding the tag-sprawl failure pattern seen in skill marketplaces and Jira labelling.
- Avoiding `superpowers`'s description-matching failure, where tags/descriptions become the input to an LLM selection step.
- Selection (§2.5) must remain a pure function: artifact kind → producer → rank. A facet system that let tags drive selection directly would break that contract.
- The tracker already has human-authored labels (`rk:kind/*`, `rk:cos/*`); classification facets that are human inputs need to stay in sync with those labels rather than drift into a second vocabulary.
- Free text has a legitimate place (search) but must never become a predicate input, per §2.4's `notes` field already being explicitly untrusted and non-predicate.

## Decision

Activity classification is a set of **facets**: closed, enumerated vocabularies the engine evaluates against computed facts. `tags = ["kind:bug", "toolchain:unreal", "change:binary"]` is authoring sugar that compiles to a conjunction in `applicable_if`.

Five rules:
1. Closed vocabulary per facet in `facets.toml`; lint rejects unknown facets/values; a tech pack may extend a facet's *values*, never add a facet.
2. Every facet maps to a computed fact in the context vector — if nothing can compute it, it cannot be a facet.
3. Facets filter **admissibility only**; selection remains artifact kind → producer → rank. `security` never means "run all security activities."
4. Facets that are human inputs (`kind`, `cos`) share their exact value sets with the tracker labels `rk:kind/*`, `rk:cos/*`, so relabelling an issue is a facet change plus a recompute.
5. Free text lives only in an engine-ignored `keywords` field for `redkiln catalog search`.

Initial facets: `kind`, `stage`, `toolchain`, `change` {text, binary, generated}, `rigor`, `role`, `cost`, and the boolean risk facets `auth`, `data-migration`, `external-dep`, `ui`, `market-facing` — i.e. the §2.4 context vector plus the activity's placement fields, in one vocabulary.

## Considered options

### Option — free-form tags
Let activities and packs attach arbitrary tags. Rejected on the skill-marketplace / Jira-label sprawl pattern and on `superpowers`'s description-matching failure — an open vocabulary accumulates inconsistent, unlintable, eventually-meaningless labels.

### Option — tags as a selection mechanism
Use tags directly to drive which activities run, rather than compiling them into `applicable_if` predicates over admissibility only. Rejected because this "reintroduces the LLM-shaped 'which activities?' question" that the deterministic selection function (§2.5) was built to eliminate.

## Consequences

**Positive**
- Every facet is lint-checkable against a closed vocabulary (`facets.toml`), so an unknown facet or value is caught at pack-load time rather than silently accepted.
- Tag-shaped authoring sugar (`tags = [...]`) is preserved for ergonomics while compiling down to the same closed, checkable `applicable_if` conjunctions as hand-written predicates — authors keep a familiar surface without reopening the free-text failure mode.
- Human-input facets (`kind`, `cos`) staying in lockstep with tracker labels (`rk:kind/*`, `rk:cos/*`) means relabelling in the tracker has a single, well-defined effect: a facet change plus a recompute.
- Search remains possible via the `keywords` field without that field ever becoming a predicate input.

**Negative**
- Rule 2 makes the facet vocabulary hostage to what the context vector can compute: a classification that would be useful but has no computable fact behind it cannot be expressed at all, and there is no escape hatch in the row.
- Rule 4 couples the catalog to the tracker's label vocabulary — the `kind`/`cos` value sets must be kept in exact lockstep with `rk:kind/*` and `rk:cos/*`, and the row gives no mechanism for enforcing that beyond the requirement itself.
- Extending classification is a registry edit, not a pack-local act: a tech pack can widen a facet's values but must go to the facet owner for a new facet, so genuinely new axes are slower to add than a tag would be.

**What this forbids**
- It forbids any facet without a corresponding computed fact in the context vector — a facet that nothing can compute is disallowed outright (rule 2).
- It forbids a technology pack from adding a new facet; it may only extend an existing facet's value set (rule 1).
- It forbids facets from being used for anything beyond admissibility filtering — a facet can never mean "run everything matching this facet" (rule 3); selection stays artifact kind → producer → rank.
- It forbids free text (the `keywords` field, and by extension anything like §2.4's `notes`) from being a predicate input anywhere in the engine.
- It forbids `tags` from being an independent selection language: whatever the compilation rule is, tags are sugar that lowers to `applicable_if`. (PC8 states that rule as a conjunction; schema amendment 20 — outside this cohort's number range — later corrects it to "same facet disjoins, distinct facets conjoin", noting PC8's own §7 example was unsatisfiable as a pure conjunction. The sugar-only status is unaffected by that correction.)

## Evidence and confidence

- [documented] V-Modell XT's typed project characteristics, the Provop context model with enumerated value ranges, and C-EPC facts are cited as prior art for closed-vocabulary facet systems; none are measurements taken on this system.
- [documented] "competitive §B9, §D lesson 3" is cited as a source but its content is not reproduced in this row; the ADR cannot state what that lesson says beyond the citation itself.
- [inferred] The claim that tag-based selection "reintroduces the LLM-shaped 'which activities?' question" is the row's own reasoning about what selection-by-tag would imply, not a spike measurement of this catalog running that way.
- [documented] The skill-marketplace/Jira-label-sprawl pattern and `superpowers`'s description-matching failure are cited as known failure modes elsewhere referenced in this corpus (PC5 also cites the `superpowers` failure, there to argue it does *not* transfer to this system's deterministic selection); no measurement is given for either failure's rate or scope.
- Later refinements to this rule (e.g. schema amendment 20 on tag disjunction/conjunction semantics, amendment 30 on boolean facets being `applicable_if`-only, amendment 37 adding `recent_deploy` to the facet registry) are amendments within the activity-graph-schema document, outside this cohort's number range (0027-0030); they are not recorded here as supersessions since the source ledger does not mark PC8 itself as superseded by them, only elaborated.
