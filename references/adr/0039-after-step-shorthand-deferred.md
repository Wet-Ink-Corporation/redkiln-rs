---
id: "0039"
title: "After=<step> shorthand deferred pending recurrence at scale"
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-9
source: references/2026-09-07-activity-graph-schema.md §9a
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

Rigor-conditional step chains — a downstream step whose `when` restates or extends an upstream step's condition — are visible in the schema's own §7 worked example, where `adversary` (`when = "rigor >= recommended"`) feeds `signoff` (`when = "rigor >= mandatory"`). Authoring ten activities against §1–§9 (P9 spike, `spikes/F32-DRAFT.md` §4) produced a proposed shorthand for this, `after = "<step>"` implying the same `when` as the referenced step; it was not adopted at this pass. The source does not report how often the pattern actually occurred across the ten activities.

## Decision drivers

- Avoid adding schema surface (a new step field) before there is enough repeated evidence that the pattern is common across the catalog, not just within one author's ten activities.
- The condition for adoption is stated explicitly: "if rigor-conditional chains recur at scale."

## Considered options

### Option — adopt `after = "<step>"` now
Would let a step inherit its `when` from a named earlier step, removing the repetition the pattern forces. Not adopted: the amendment record explicitly defers this pending recurrence "at scale", rather than judging the ten-activity pass sufficient on its own. (The source names no alternative option for this row; it frames the row as a deferral, so the options above are "adopt now" versus "defer".)

### Option — defer the shorthand (chosen)
No new field is added yet; authors continue restating `when` predicates on each chained step until (if) the pattern is shown to recur at scale.

## Decision

`after = "<step>"`, which would imply the same `when` as the referenced step, is **deferred** — not adopted in this schema revision. It remains a candidate amendment "if rigor-conditional chains recur at scale."

## Consequences

**Positive**: the schema does not grow a new field ahead of evidence that it is needed broadly, keeping the step-declaration surface smaller for now.

**Negative**: pack authors must keep repeating identical or overlapping `when` predicates on chained steps (the `adversary`/`signoff` shape in the §7 worked example) until this lands, if it ever does.

**What this forbids**: no `after=` field exists in this schema revision; catalog authors cannot use it, and lint will not recognise or validate it.

## Evidence and confidence

[documented] Recorded in `2026-09-07-activity-graph-schema.md` §9a item 9, sourced to `spikes/F32-DRAFT.md` §4 (P9 authoring of ten activities). This decision is explicitly provisional: it is conditional on a future, unmeasured event ("rigor-conditional chains recur at scale") and no threshold or measurement for "at scale" is given in the source. No finding id is cited, and none should be inferred.
