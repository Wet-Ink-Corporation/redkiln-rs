---
id: "0041"
title: Alternative outcomes materialise through a final tool/check step, exit set stays three
status: superseded
date: 2026-09-07
legacy_id: SCHEMA-11
source: references/2026-09-07-activity-graph-schema.md §9a
supersedes: []
superseded_by: ["0064"]
related: []
evidence: []
consumers: []
---

## Context and problem

§3 fixes an activity's exit set to exactly three typed exits toward the stage: `Materialized`, `Blocked`, `RequestDeviation`. Authoring ten activities against §1–§9 (P9 spike, `spikes/F32-DRAFT.md` §4) raised the case of an activity whose internal graph can take different branches to different outcomes — the example given is a rolled-back release — and needed a pattern for expressing that without adding a fourth exit type.

## Decision drivers

- The three-exit contract in §3 is a fixed design constraint ("Nothing else leaks"); the amendment had to fit within it, not extend it.
- Different internal branches must still converge on producing the activity's declared `provides` artifact so the stage-facing contract (`Present(artifact, evidence)`, PC6) is unaffected by which branch ran.

## Considered options

### Option — add a fourth exit type for branch-dependent alternative outcomes
Would let the stage see, e.g., a distinct "rolled back" exit alongside `Materialized`/`Blocked`/`RequestDeviation`. The source row names no alternative option; this one is stated here because the amendment's own closing clause ("the exit set stays at three") is what rules it out — a fourth exit would break the closed three-exit contract §3 fixes with "Nothing else leaks".

### Option — converge branches on an unconditional final `tool`/`check` step (chosen)
Whichever branch ran, a final step unconditionally materialises the artifact, keeping the exit set at three.

## Decision

"Alternative outcomes (e.g. a rolled-back release) are expressed by an unconditional final `tool`/`check` step that materialises the artifact from whichever branch ran; the exit set stays at three."

## Consequences

**Positive**: preserves the §3 closed three-exit contract while still letting an activity's internal graph branch on outcome.

**Negative**: the pattern was later superseded for pure-composition cases. §9c amendment 34 gives `emit` the ability to compose an artifact from several refs and states that §9a.11's "final tool step" pattern is superseded by composition for those cases; the source gives no measurement or count of how often the pattern proved awkward.

**What this forbids**: an activity may not expose a fourth, branch-specific exit to the stage; regardless of which internal branch executed, the stage only ever observes one of `Materialized`/`Blocked`/`RequestDeviation`.

## Evidence and confidence

[documented] Recorded in `2026-09-07-activity-graph-schema.md` §9a item 11, sourced to `spikes/F32-DRAFT.md` §4 (P9 authoring of ten activities). No finding id or measurement is cited for this item specifically.

**Supersession**: §9c amendment 34 (2026-09-08, `spikes/F34-DRAFT.md` §4, decided in `spikes/p9/DECISIONS-2026-09-08.md`) states explicitly: "`emit` may compose an artifact from several refs — `Kind = { field = "${ref}", … }` — with fields checked against the kind's schema; §9a.11's 'final tool step' pattern is superseded by composition for pure-composition cases." The source scopes that supersession to pure-composition cases and does not say what, if anything, survives of the pattern for cases that are not pure composition. This record is nonetheless marked **superseded** with `superseded_by: ["0064"]` per the fixed supersession map for this retrofit, which records the relation at record granularity rather than clause granularity; ADR 0064 (amendment 34) is the successor record.
