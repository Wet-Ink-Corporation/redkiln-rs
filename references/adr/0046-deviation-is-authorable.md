---
id: "0046"
title: Deviation is authorable through on_false, not just declarable
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-16
source: references/2026-09-07-activity-graph-schema.md §9b amendment 16
supersedes: []
superseded_by: []
related: ["0031", "0044"]
evidence: []
consumers: []
---

## Context and problem

§3 defines `RequestDeviation { target, kind, claimed_predicate, why }` as one of an activity's three typed exits toward the stage, and §0's own constraint table requires that "deviation is typed; the model never edits the stage's required set" (proposal §4.2, catalog §2.5 step 7). But no step kind in §1's closed five-kind table, and no field in §7's TOML shape, could actually produce that exit — the source states plainly: "Previously `RequestDeviation` was an exit no step could produce." A documented, typed exit existed in the type system with no path to reach it.

## Decision drivers

- §3's three exits are meant to be exhaustive and reachable; an exit no step can produce is dead in practice even though it is declared in `ActivityExit`.
- §0's constraint that deviation stay typed and that "the model never edits the stage's required set" rules out any free-form or ad hoc way for an activity to signal a deviation — it has to route through an already-governed mechanism.
- `on_false`'s bounded loop-back machinery (amendment 14, ADR 0044) already exists as a governed, lint-checked way for a `check` step to redirect control flow; reusing it avoids inventing a second control-flow primitive solely for deviation.

## Considered options

### Option — do nothing (leave RequestDeviation unreachable)
Keep `RequestDeviation` declared in §3/§8 without giving any step a way to produce it. The source names no alternative weighed against the chosen form; this is the pre-amendment state, which the source itself diagnoses as broken ("an exit no step could produce"). It loses because it leaves a typed exit permanently unreachable, so §0's requirement that deviation be typed and kernel-adjudicated is satisfied only on paper.

## Decision

"`on_false.to = "deviate"` with `deviate = { kind = skip|downgrade|add, target = <ArtifactKind>, claimed = <predicate> }`; the kernel handles it per catalog §2.5 step 7."

## Consequences

**Positive**
- Closes the dead-exit gap: `RequestDeviation` now has a concrete, lint-checkable step-level trigger.
- Reuses the existing bounded `on_false` construct (ADR 0044) rather than introducing a new control-flow primitive, keeping the step-kind table closed at five kinds.

**Negative**
- Overloads `on_false.to`, which otherwise names an earlier step id (ADR 0044), with a second, non-step-id special value (`"deviate"`) that lint must distinguish from a genuine loop-back target.

**What this forbids**
- Forbids any way of requesting a deviation other than through this typed `on_false.to = "deviate"` construct — no step may emit a free-text or untyped deviation request.
- Forbids a `deviate` declaration whose `kind` falls outside the closed `skip | downgrade | add` vocabulary.
- Reaffirms, by giving deviation an actual mechanism, that the activity still never edits the stage's required set directly (§0) — it can only *request* a deviation, which the kernel evaluates against the `claimed` predicate per catalog §2.5 step 7.

## Evidence and confidence

- [documented] Stated normatively in §9b amendment 16; the "previously... no step could produce" framing is the source's own diagnosis of the prior gap, not an inferred claim.
- [inferred] The source does not cite a spike or finding showing how often an activity actually needed to request a deviation before this fix; the decision closes a type-system gap, and no runtime-frequency claim is made here.
