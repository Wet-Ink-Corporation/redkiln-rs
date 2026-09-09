---
id: "0056"
title: Family totality caps rank escalation by rigor
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-26
source: references/2026-09-07-activity-graph-schema.md §9c amendment 26
supersedes: []
superseded_by: []
related: ["0020", "0024", "0058"]
evidence: ["F35"]
consumers: []
---

## Context and problem

The rank-among-producers rule ("lowest `method_rank` wins") was already settled (process-catalog-proposal.md §2.5 step 4;
proposal §3 worked example). What was undecided was what happens when a family of producers for the same artifact kind
escalates with rigor: a lower-ranked, cheaper member can have a `max_rigor` ceiling above which it is no longer admissible,
and nothing previously guaranteed that some other family member covers the contexts the ceiling leaves behind. The P6
authoring batch (90 activities, `F34-DRAFT.md` §4) surfaced 30 ambiguities, 25 of them not covered by §9a/§9b, and the P6
lattice run (`F35-DRAFT.md`) measured the consequence directly: ceilings without a totality rule took unsatisfiability
from 11.4% to 30.1% of enumerated contexts. The source does not name the condition the 11.4% figure was measured under.

## Decision drivers

- The base rank rule (lowest `method_rank` wins among admissible producers) must stay unchanged — it is already load-bearing
  (§2.5 step 4; proposal §3 example) and is not being revisited.
- A `max_rigor` ceiling on a family's lower (cheaper) members is a real authoring need: cheap methods should not be asked
  to run at rigor levels they cannot support.
- The lattice run showed that ceilings without a covering guarantee create silent unsatisfiability, not a loud error.
- The totality property needs to be lint-checkable, not just true by authoring discipline.

## Decision

The rank rule is unchanged: among admissible producers of a kind, the lowest `method_rank` wins. A family may escalate
with rigor by putting a `max_rigor` on its lower members; above its ceiling, that member is inadmissible. This is now
constrained by a totality lint rule: **for every rigor level ≥ the lowest member's `min_rigor`, some member of the family
must be admissible in every context the lowest member's tags cover** ("family total over rigor"). This applies to the
rigor-driven families (spec form, test design, review, learning). Fact-driven families (mitigation, deploy pattern) are
exempt and keep selecting purely on `applicable_if`.

## Considered options

### Option — ceilings without a totality check (status quo before this amendment)
Allow `max_rigor` on family members with no lint guarantee that some member remains admissible past the ceiling. This is
what the corpus measured directly: 30.1% of enumerated contexts went unsatisfiable, against a prior 11.4%. Rejected on
that measurement. The source row names no alternatives for this amendment; this option is the prior state the amendment
replaces, not a named candidate.

### Option — family total over rigor (adopted)
Keep ceilings, but require by lint that the family as a whole remains satisfiable at every rigor ≥ the lowest member's
`min_rigor`, over every context the lowest member's tags cover. Chosen because it keeps the authoring flexibility of
ceilings while removing the silent-failure mode the lattice run exposed.

## Consequences

**Positive**
- Cheap, low-ranked producers can still be capped by rigor without silently starving high-rigor contexts of any producer.
- The failure mode (unsatisfiable context) becomes a lint-time error instead of a runtime `Unsatisfiable(kind,
  failed_predicates)` gate discovered late.

**Negative**
- Authoring a family now carries an additional obligation (prove coverage above every ceiling), which the P6 batch treated
  as one of the harder-to-satisfy new lint rules.
- The totality property is checked over "every context the lowest member's tags cover" — a lattice enumeration — which is
  the same mechanism §9 lint rule 5 already uses and so shares its cost profile.

**What this forbids**
- A family may not define a `max_rigor` ceiling on a member without another member covering the same tag-context space at
  and above that rigor. Rank ties among admissible producers remain forbidden per the pre-existing rule (a pack validation
  error, never a runtime choice) and are not reopened here.
- Fact-driven families (mitigation, deploy pattern) may not adopt the totality rule as a substitute for `applicable_if` —
  the rule applies only to the rigor-driven families named in the decision.

## Evidence and confidence

- [measured] The lattice run (`F35-DRAFT.md` scratch run) found unsatisfiability rose from 11.4% to 30.1% with ceilings
  present but ungoverned by the totality rule; the source does not name the 11.4% condition — n is the enumerated context lattice for the P6 catalog batch (90 activities); the
  exact lattice size is not given in the row and is not restated here.
- [documented] The base rank rule itself ("lowest `method_rank` wins... a tie is a pack validation error") is unchanged and
  traces to process-catalog-proposal.md §2.5 step 4 and the proposal's §3 worked example; this decision does not re-derive
  or re-justify that rule.
- This measurement licenses only the claim that *ungoverned* ceilings raised unsatisfiability on this corpus; it does
  not measure whether the totality-lint version recovers to 11.4% or improves on it — that number is not reported in
  the source.
