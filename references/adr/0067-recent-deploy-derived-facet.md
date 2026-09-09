---
id: "0067"
title: recent_deploy joins the facet registry as a derived boolean
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-37
source: references/2026-09-07-activity-graph-schema.md §9c amendment 37
supersedes: []
superseded_by: []
related: ["0030", "0060", "0061"]
evidence: []
consumers: []
---

## Context and problem

The P6 authoring batch (a third author writing 90 activities and running the P6 lattice) surfaced a want for the fact
"the item's surface was recently deployed". The source row does not say which activities wanted it; what it records is
the disposition — where "recently deployed" lives: as a derived fact in the facet registry, or as an author-declared
request (`wants_facet`) on the activity. §9c's "Rejected" list settles it: "`wants_facet` (37 — the fact was derivable,
so it became a facet)".

## Decision drivers

- PC8 rule 2: every facet must map to a computed fact in the context vector — if nothing can compute it, it cannot be a
  facet.
- The source names `toolchain:deploy_log` as the fact's source, so the fact is computable — which is what PC8 rule 2
  requires of anything that is to be a facet.
- The source row adds "P1 arm D measures it". It gives no measurement result, and no rationale beyond that forward
  reference.

## Considered options

### Option — `wants_facet` (an author-declared flag requesting the fact)
Rejected: the fact was derivable from `toolchain:deploy_log`, so an author-declared want was redundant with a registry
entry the engine could just compute. (Listed among the "Rejected" items at the end of §9c.)

### Option — derived boolean facet (chosen)
`recent_deploy` joins the facet registry as a derived boolean, computed from `toolchain:deploy_log`, anchored as "a
deploy touching the item's surface within 24 h." This is a registry owner's edit (`facets.toml`), not a catalog
(activity-authoring) change.

## Decision

`recent_deploy` is added to `facets.toml` as a derived boolean facet. Its value is computed from
`toolchain:deploy_log`; the anchor is "a deploy touching the item's surface within 24 h." Per PC8 rule 3, it filters
admissibility only (e.g. `applicable_if`), never selection directly. Ownership of the registry entry belongs to whoever
owns `facets.toml`, not to individual catalog/pack authors. P1 arm D measures its effect.

## Consequences

**Positive**
- Mitigation and deploy-pattern activities gain a concrete, computed condition instead of relying on free text or an
  author-declared flag.
- Consistent with PC8's closed-vocabulary, fact-backed facet model — no new class of facet mechanism was introduced.

**Negative**
- Adds a new registry entry and a dependency on `toolchain:deploy_log` being populated correctly; a stale or absent
  deploy log silently yields `recent_deploy = false`.
- The 24 h anchor is a specific, somewhat arbitrary threshold baked into the registry rather than the activity.

**What this forbids**
- Forbids authors from declaring their own ad hoc "wants this fact" flags (the `wants_facet` alternative) — if a fact is
  derivable, it must become a registry-level facet, not a per-activity request.
- Forbids treating "recent deploy" as a boolean *tag* per amendment 30 (boolean facets are `applicable_if`-only; a tag
  on a boolean facet is a lint error).

## Evidence and confidence

- [documented] The decision and its anchor (24 h window, `toolchain:deploy_log` source) are stated directly in §9c
  amendment 37 of the schema document.
- [documented] The source row says "P1 arm D measures it" — a forward reference only. No measurement result is given,
  and P1 arm D has not been read out (its blind panel is unfilled), so nothing here rests on a measured effect.
- No finding id or spike result is cited for this amendment beyond that forward reference; the `evidence` field is left
  empty accordingly.
- This licenses only that the facet exists in the registry with a stated source and anchor. It does not license any
  claim that `recent_deploy` improves selection, that the 24 h anchor is the right one, or — a separate question the
  source row does not address — that `deploy_log` is populated in any current deployment.
