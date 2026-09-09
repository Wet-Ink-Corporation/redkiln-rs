---
id: "0038"
title: Escalate carries only a question id; options live in the question registry
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-8
source: references/2026-09-07-activity-graph-schema.md §9a
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

§3 originally allowed a step to declare `escalate = { on = <predicate>, question = <QuestionId>, options = [..] }` — an inline predicate and an inline option list alongside the question id. Authoring ten activities against §1–§9 (P9 spike, `spikes/F32-DRAFT.md` §4) found this shape needed tightening: the §7 worked example already only specified `question` at the call site (`escalate = { question = "root-cause.no-hypothesis" }`, with the comment "options live in questions.toml"), showing the inline `on`/`options` fields were not actually load-bearing at authoring time.

## Decision drivers

- A single source of truth for a question's option set avoids the escalate call site and the registry drifting apart.
- Open question 2 (§10) already proposes a pack-level `questions.toml` owning stable id/text/option sets for the comment grammar and web view — inline `options` at the escalate site duplicates that.

## Considered options

### Option — keep the original inline shape (`on`, `question`, `options` all inline)
This is the pre-amendment §3 shape. It permits each escalate declaration to define its own option set, risking drift from whatever the registry (or another escalate site sharing the same question) declares.

### Option — `escalate = { question = … }` only (chosen)
Options are owned exclusively by the question registry; the escalate declaration names only which question is being asked.

## Decision

`escalate = { question = … }` only; options live in the question registry.

## Consequences

**Positive**: eliminates duplicated option sets between escalate call sites and the registry, and matches the pattern already used in the §7 worked example.

**Negative**: an escalate declaration alone no longer shows a reader which answers are possible; the registry must be consulted. The `on` predicate present in the original §3 shape is also dropped from `escalate` itself; the source states no consequence for the dropped `on` field and does not say what, if anything, replaces it.

**What this forbids**: an `escalate` declaration may not carry an inline `options` list, nor an inline `on` predicate; both concerns move outside the `escalate` block itself.

## Evidence and confidence

[documented] Recorded in `2026-09-07-activity-graph-schema.md` §9a item 8, sourced to `spikes/F32-DRAFT.md` §4 (P9 authoring of ten activities). No finding id or measurement is cited. The source row names no alternative option; the losing option above is the pre-amendment §3 shape (line 63 of the schema), which the amendment replaces. A later amendment (§9b.15) documents the interim pattern for `on_exhausted = "escalate"` as an explicit `gate` step with `when` over the looped step's status — the source does not present that as a replacement for the dropped `on` field, and no ADR for §9b.15 exists in the current map, so no cross-reference is asserted here.
