---
id: "0057"
title: Producer candidacy is stage-scoped but requires resolve from any unforbidden stage or a human source
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-27
source: references/2026-09-07-activity-graph-schema.md §9c amendment 27
supersedes: []
superseded_by: []
related: ["0020", "0025"]
evidence: []
consumers: []
---

## Context and problem

The P6 authoring batch needed a rule for two related but distinct questions that the schema had left open: (1) which
activities are candidates to produce a given stage's artifact, and (2) where the artifacts named in a producer's
unconditional `requires` are allowed to come from. Without an answer, `forbidden` — the lowest level of the deontic rigor
enum (D20/PC1), which proposal §4 reads as "nothing in that stage runs" — had no enforcement mechanism against a producer
reaching sideways into a forbidden stage's output to satisfy a `requires` row.

## Decision drivers

- Candidacy for a stage's artifact must stay scoped to that stage's own activities, matching the existing selection
  model (process-catalog-proposal.md §2.5).
- `forbidden` (proposal §4) must actually mean nothing runs in that stage, including indirectly through another
  activity's `requires`.
- Human-supplied artifacts (PC3: vouched root, `derived_from = []`) are a legitimate source for a `requires` row and
  need a place to be declared as such, since they have no producing activity.

## Decision

Candidates for a stage's artifact are activities whose `stage` equals the requiring stage — candidacy does not cross
stage boundaries. A producer's *unconditional* `requires` may resolve from any stage the pack does not forbid, or from
an artifact kind registered `source = "human"` in a new registry, `artifacts.toml` (which also holds the §2.1 kind
vocabulary). Lint enforces this: every unconditional `requires` kind must be produced by some activity or be
human-sourced. Consequently `forbidden` means what proposal §4 says — nothing in that stage runs, even via `requires`.

## Considered options

### Option — do nothing (leave requires-resolution unscoped)
Leave `requires` free to resolve from any stage regardless of `forbidden`, with no registry distinguishing human-sourced
kinds. This is the implicit prior state the amendment closes. Rejected because it lets a producer route around a
`forbidden` stage by depending on that stage's artifact through `requires`, silently violating proposal §4's meaning of
`forbidden`. The source row names no alternatives; this is the prior state the amendment closes, not a named candidate.

## Consequences

**Positive**
- `forbidden` now has an enforceable meaning that survives indirect production via another activity's `requires`.
- Human-supplied roots get an explicit registry entry (`source = "human"` in `artifacts.toml`) instead of being an
  implicit exception every producer's `requires` logic has to special-case.
- Lint can catch an unconditional `requires` kind that no activity produces and is not human-sourced, at pack-load time
  rather than at runtime `Unsatisfiable`.

**Negative**
- Introduces a new registry (`artifacts.toml`) that authors must keep in sync with which kinds are genuinely
  human-sourced versus producible.

**What this forbids**
- A producer's unconditional `requires` may not resolve an artifact kind from a stage the pack has marked `forbidden`.
- An artifact kind may not silently stand in as human-sourced without being registered `source = "human"` in
  `artifacts.toml` — lint treats an unresolvable, unregistered kind as an error, not a fallback to human input.
- Candidacy for a stage's own artifact may not be drawn from activities declared at a different stage, regardless of
  `requires` resolution rules.

## Evidence and confidence

- [documented] The row cites no finding id or spike; the decision rests on the existing proposal §4 meaning of
  `forbidden` and the PC3 human-root vouching rule (ADR 0025), not on new measurement. No evidence is invented here.
- The interaction between this rule and rule 26's family totality (both from the same P6 batch) is not characterized in
  the source beyond both being adopted in the same adjudication pass; this ADR does not assert one.
