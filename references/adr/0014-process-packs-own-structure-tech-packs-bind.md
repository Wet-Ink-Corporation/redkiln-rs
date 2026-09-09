---
id: "0014"
title: Process packs own structure; technology packs bind and contribute at slots
status: accepted
date: 2026-09-05
legacy_id: D14
source: references/2026-09-05-architecture-proposal.md §4.2.1
supersedes: []
superseded_by: []
related: ["0022", "0024", "0029"]
evidence: []
consumers: []
---

## Context and problem

The two pack axes — process packs (what runs: stages, activities, gates, weights) and technology packs (how an activity is realised in an ecosystem) — both want to own "the thing that runs" (§4.2.1). Without a resolution, a technology pack could define activities directly or a card could be named directly inside an activity, which would let ecosystem-specific concerns leak into process structure and make the two axes compose unpredictably per repo.

## Decision drivers

- Prior art from build systems that hit the identical tension: Nx targets vs executors, Tekton Pipelines vs Tasks, Cloud Native Buildpacks' ordered detect-and-contribute, GitHub reusable workflows vs composite actions (§4.2.1).
- `redkiln doctor`'s configuration-time rules (§4.2.1 table) depend on roles, slots, and fillers being distinct, named extension points rather than ad hoc references, so ambiguity (e.g. an unresolved or ambiguous role) can be caught at zero model cost.
- The binder needs a deterministic resolution order (role → candidate cards → most specific selector wins) that only works if activities nominate roles, never cards, and technology packs only bind and fill declared slots.

## Decision

Process packs own structure: stages, activities, gates, weights per classification, **roles** (archetypes such as `implementer`, `reviewer`, `investigator`, `spec-author`), tier policy per role, and **slots** (named extension points in a stage). Technology packs **bind** (cards, skills) and **contribute at slots** only. "A technology pack may *bind* (cards, skills) and *contribute at slots*; it may never *restructure*." An activity nominates a role and a typed contract, never a card. When an ecosystem needs a different shape than the declared slots allow, that is a **process-pack variant** (e.g. `story-ue`), selected at triage or by config — never a silent merge of the two packs.

## Considered options

### Option — tech packs defining activities directly

Let a technology pack define its own activities rather than binding to roles and filling declared slots. Rejected: this collapses the axis boundary the table in §4.2.1 sets up ("Never owns: new stages; edits to process activities" for technology packs) and would let ecosystem specifics dictate process structure per toolchain.

### Option — cards named in activities

Let an activity name a specific agent card directly instead of nominating a role. Rejected: this breaks the binder's deterministic resolution (`role → candidate cards → most specific selector wins`) and reintroduces per-toolchain hardcoding into process content, undermining "why did this run with that card" traceability via the run record.

## Consequences

**Positive**
- `redkiln doctor` can validate pack composition entirely at configuration time and at zero model cost: unresolved/ambiguous roles, dangling references, undeclared/dead slots, dead definitions, tier leakage, and unbounded activities are all catchable before any dispatch (§4.2.1 table).
- The binder's resolution (card, skills, fillers, tier) is recorded in the event log, so a run's card choice is auditable without reading TOML.
- Ecosystem-specific shape changes (e.g. Unreal's cook/bake stage) get an explicit, visible process-pack variant rather than a silent two-pack merge.

**Negative**
- An ecosystem need that doesn't fit an existing slot forces authoring a whole process-pack variant rather than a lighter technology-pack extension.
- Two candidate cards at equal selector specificity are a hard configuration error, never a guess — this can block dispatch until the pack is fixed.

**What this forbids**
- A technology pack may never add or edit stages, or edit process activities.
- An activity may never name a card directly — only a role.
- A technology pack may never restructure a stage outside its declared slots; a shape mismatch must become a process-pack variant, not a merge.

## Evidence and confidence

- [documented] The rule is direct source text in the decision table (§9, D14) and stated at length in §4.2.1: the three-layer Owns/Never-owns table, the "hard line" sentence quoted in the Decision, the binder's four numbered resolution steps, and the seven-row `redkiln doctor` rule table.
- [documented] The row's "Alternatives considered" — tech packs defining activities directly; cards named in activities — are transcribed as given, with no invented alternative. Their losing reasons are drawn from §4.2.1's own text (the "Never owns" column; the binder's specificity rule and run-record traceability), not from the row, which states none.
- [inferred] The prior-art list (Nx, Tekton, Cloud Native Buildpacks, GitHub reusable workflows) is §4.2.1's stated grounding for the resolution; the source asserts the analogy rather than demonstrating it, and no comparative study is cited.
- No spike, finding, or measurement id is cited for D14 itself. Related decisions: D22 (0022) states "(D14 unchanged)" and PC7 (0029) defers to it ("that remains a process-pack variant (`story-ue`) per D14") when extending the technology-pack contract to producers and artifact kinds; PC2 (0024) lists D14 among its own considered constraints. These cross-references are recorded as `related` — they show later decisions holding the line, not evidence for D14's own grounding.
- This decision does not license any claim that the composition rule has been validated in a running system, that `redkiln doctor` implements the seven rules today, or that the prior-art systems' outcomes transfer; it fixes the axis boundary and the configuration-time checks that boundary makes possible.
