---
id: "0066"
title: rigor is a legal root in applicable_if predicates
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-36
source: references/2026-09-07-activity-graph-schema.md §9c amendment 36
supersedes: []
superseded_by: []
related: ["0033", "0056"]
evidence: []
consumers: []
---

## Context and problem

`rigor` was already established as a predicate root for step-level `when` conditions (variant-by-rigor, §1; §9a.3's predicate grammar lists roots as "step inputs, facets, `rigor`, `item`"). Amendment 26 (rank semantics) separately introduced `max_rigor` on a family's lower members to express a rigor ceiling on admissibility. Authoring the P6 catalog raised the question of whether `applicable_if` — the admissibility predicate evaluated at selection time (PC8 rule 3) — may itself reference `rigor` directly, since `applicable_if` and step `when` share the same predicate grammar (§8: "the same closed-vocabulary expression type used by `applicable_if` and `require.when`").

## Decision drivers

- The predicate grammar is already shared between `when` and `applicable_if` (§8); disallowing `rigor` in one but not the other would be an inconsistency with no stated rationale.
- Amendment 26 already gives activities a rigor ceiling via `max_rigor`, which does the same admissibility-narrowing job for the common case of "inadmissible above this rigor" — so allowing bare `rigor` comparisons in `applicable_if` risks two ways of saying the same thing.

## Considered options

### Option — do nothing (rigor stays a when-only root)
Leave `rigor` usable only in step-level `when` predicates, not in `applicable_if`. Not named as considered in the source, but implicit as the prior state; it would have left `applicable_if` predicates unable to express any rigor-dependent admissibility directly, pushing every such case through `max_rigor` even where that field's shape does not fit what the author needs to say.

## Decision

`rigor` — the stage rigor at selection — is a legal root in `applicable_if`. Lint warns (not errors) where the same condition would be better expressed with `max_rigor`.

## Consequences

**Positive**
- Makes `applicable_if` consistent with `when`, which already treats `rigor` as a legal root (§9a.3).
- Allows rigor-dependent admissibility conditions that do not fit the simple ceiling shape `max_rigor` expresses.

**Negative**
- Introduces two overlapping ways to express a rigor ceiling (`max_rigor` vs. a bare `rigor >=` comparison in `applicable_if`), managed only by a non-blocking lint warning rather than a hard rule.

**What this forbids**
- Forbids nothing outright — this amendment is permissive, adding a root rather than restricting one. It does not forbid or deprecate `max_rigor`; the warning is advisory only.

## Evidence and confidence

[documented] Adjudicated in the §9c batch from the third author's 90-activity P6 catalog pass (`spikes/F34-DRAFT.md` §4; `spikes/p9/DECISIONS-2026-09-08.md` decision 10, "36 accept — `rigor` legal in `applicable_if`, lint warns where `max_rigor` would do"). The need is recorded as F34 B19: §9a.3 lists `rigor` among the roots for step predicates while PC8 lists activity facets without it, and the author used it anyway in two activities (`code-review`, `rolling-deploy`) to keep kinds satisfiable at `optional` where the higher family member has `min_rigor = recommended`. No measurement is cited for this item; it is a grammar-consistency decision, not a result, and the warning-vs-error choice for the `max_rigor` overlap is not justified by any cited data.
