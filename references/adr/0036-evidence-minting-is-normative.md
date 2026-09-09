---
id: "0036"
title: Each step kind mints a fixed, closed set of evidence variants
status: accepted
date: 2026-09-07
legacy_id: SCHEMA-6
source: references/2026-09-07-activity-graph-schema.md §9a (amendment 6)
supersedes: []
superseded_by: []
related: ["0063", "0071"]
evidence: ["P9 spikes/F32-DRAFT.md §4"]
consumers: []
---

## Context and problem

§2 defines the closed `Evidence` enum (`SelfCheck`, `Findings`, `ToolReport`, `GreenAt`, `Mutation`, `HumanAnswer`, `Degraded`, `Attempts`) and the rule that "every variant names the step that produced it," and §7's worked example shows an `emit` step collecting evidence refs (`["${confirm}", "${adversary?}", "${signoff?}"]`) — but §1–§9 never state which step *kind* mints which evidence variant, leaving it to be inferred per activity from what each step happens to output. Authoring ten activities against the schema needed a fixed mapping to write `evidence` promises (§7's `evidence = [...]` field) and lint rule 5's satisfiability check against.

## Decision drivers

- §2's closed `Evidence` enum only constrains the shape of evidence, not which step kind is allowed to produce which variant; without a mapping, an activity's `evidence` promise (§7) and lint rule 5 (promise satisfiable at every rigor level) have nothing fixed to check against.
- The five step kinds (§1: `agent`, `tool`, `check`, `gate`, `emit`) already have fixed responsibilities (`check` "cannot dispatch", `emit` is "the activity's terminal node"); minting rules needed to be consistent with those responsibilities rather than ad hoc per activity.
- Ten activities authored against the schema (spike P9) forced the mapping to be made explicit.

## Considered options

### Option — do nothing (infer minting per activity from what each step outputs)
Which `Evidence` variant a step produced would be read off whatever that step happened to emit, leaving the activity's `evidence` promise and lint rule 5's satisfiability check with no fixed mapping to resolve against. The source row names no alternative mapping; this is the pre-amendment state amendment 6 replaces.

### Option — fix minting per step kind, normatively (chosen)
Each of the five step kinds mints a determined set of variants (including none, for `check` and `emit`), so the promise and the lint check are decidable from the graph's shape alone.

## Decision

Evidence minting is normative: a `tool` step mints `ToolReport` (plus `GreenAt` when `run = toolchain:test` and status is passing; plus `Mutation` when the output is a `MutationReport`); an `agent` step mints `Findings` or `SelfCheck` depending on its output type; a `gate` step mints `HumanAnswer`; `check` and `emit` steps mint no evidence.

## Consequences

**Positive**: an activity's `evidence` promise (§7) and lint rule 5's satisfiability check now resolve against a fixed step-kind → evidence-variant mapping instead of per-activity inference; `check` and `emit` are confirmed to be evidence-inert, consistent with `check` being a pure predicate and `emit` being a binder, not a producer.

**Negative**: an activity that needs evidence attached from logic that naturally belongs in a `check` or `emit` step has no minting path there and must move that logic into an `agent`/`tool`/`gate` step instead.

**What this forbids**: `check` and `emit` steps minting any evidence variant; a `tool` step minting `Findings`/`SelfCheck` or an `agent` step minting `ToolReport`/`GreenAt`/`Mutation` — minting is fixed per step kind, not author-selectable.

## Evidence and confidence

[documented] Stated normatively in §9a amendment 6, one of eleven amendments produced by authoring ten activities against §1–§9 (spike P9); the source gives no separate count of how many evidence-minting mismatches the ten activities actually surfaced before the rule was fixed. [inferred] Later, out-of-range amendments extend but do not reverse this mapping: §9c.33 (ADR 0063) adds that a gate's output also carries `attachments: [ArtifactRef]` alongside its minted `HumanAnswer`, and §9c.41 (ADR 0071) constrains when the `evidence` promise may be empty (only for activities with no `agent` step) — neither is recorded by the source as amending the per-step-kind minting rule itself.
