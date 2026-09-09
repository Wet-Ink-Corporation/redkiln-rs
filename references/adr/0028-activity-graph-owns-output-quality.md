---
id: "0028"
title: Each activity graph owns the quality of its own output
status: accepted
date: 2026-09-07
legacy_id: PC6
source: references/2026-09-07-process-catalog-proposal.md §7 PC6
supersedes: []
superseded_by: []
related: ["0022", "0029"]
evidence: ["Temporal child workflows", "Tekton Pipelines-in-Pipelines", "Dagster asset checks", "Prifti validation tokens as dispatch preconditions"]
consumers: []
---

## Context and problem

PC4 had already split review into three levels, including "(a) Activity-internal: an activity is itself a graph responsible for the quality of its output — self-review, adversarial reviewer, retry, HITL escalation all live *inside* it and are invisible to the stage." PC4 records itself as "amended by PC6", and lists "stage-visible sibling reviewer nodes" in its own considered column as "(rejected by PC6)". PC6 is that amendment: the question this row settles is what the contract between an activity and its stage actually is, and what must never leak across that boundary into stage or workflow definitions.

## Decision drivers

- The findings-not-verdicts principle from the architecture proposal (§4.6) needs a home at the sub-stage level, not just at the item-level review stage.
- Avoiding stage/workflow definitions that have to know about an activity's internal retry, self-review or escalation machinery — that knowledge duplicating outward would recreate the same coupling problem PC4 was trying to avoid with its three-level split.
- The memo key (proposal §4.2) must stay sound if an activity's internal reviewer changes — otherwise a stale artifact could be served after an internal quality-control change.
- Kernel ownership of durable human-interaction points (`Gate:` marks) cannot be surrendered to an activity even though the activity decides *when* to invoke one.

## Decision

Each activity graph owns the quality of its own output. The contract is: "materialise artifact X to the declared bar, with evidence"; the stage sees only `Present(X, evidence)`. Nothing about internal review, retries, or escalation leaks into stage or workflow definitions.

Two kernel constraints bound this encapsulation:
(i) an activity-internal HITL escalation is still a kernel-owned durable `Gate:` mark (architecture proposal §4.6) — the activity *declares* escalation points, the kernel *executes* them;
(ii) the memo key hashes every card in the subgraph, so editing an internal reviewer invalidates the artifact.

## Considered options

### Option — reviewer nodes visible at stage level
Expose an activity's internal review/retry nodes as siblings the stage can see and schedule around. Rejected: this is the option PC4 already rejected in its own "considered" column ("stage-visible sibling reviewer nodes (rejected by PC6)"), and PC6 is the decision that made that rejection concrete by drawing the encapsulation boundary.

### Option — stage schedules quality work
Have the stage itself schedule retries, reviews or escalations for an activity's output, rather than delegating that to the activity's own internal graph. Rejected: this reintroduces stage/workflow knowledge of activity internals, which is exactly what the `Present(X, evidence)` contract is designed to avoid.

## Consequences

**Positive**
- Stage and workflow definitions stay simple: they consume `Present(X, evidence)` and never need to model an activity's internal retry/review/escalation structure.
- The encapsulation boundary is precise enough to state as a one-sentence contract ("materialise artifact X to the declared bar, with evidence"), which makes it checkable rather than aspirational.
- Prior art (Temporal child workflows, Tekton Pipelines-in-Pipelines, Dagster asset checks) gives this pattern independent grounding outside this codebase.

**Negative**
- The kernel must still track escalation points that occur *inside* an encapsulated activity as durable `Gate:` marks, which means the encapsulation is not total — the kernel has visibility the stage does not, and that asymmetry has to be maintained correctly (constraint (i)).
- The memo key hashing every card in the subgraph (constraint (ii)) means any edit to an internal reviewer, however small, invalidates the artifact and forces re-materialisation — there is no partial-invalidation path recorded here.

**What this forbids**
- It forbids stage or workflow definitions from containing any knowledge of an activity's internal review, retry, or escalation structure — that information may never leak outward.
- It forbids an activity's internal HITL escalation from being anything other than a kernel-owned durable `Gate:` mark — an activity cannot invent its own out-of-band human-interaction mechanism.
- It forbids treating a memo key that ignores internal-subgraph cards as sound — every card in the subgraph must be hashed, or the memo is unsound for the reason PC6's constraint (ii) gives — a stale artifact could be served after an internal reviewer changes. (D21/ADR 0021 restates the same activity-level rule, citing PC6, in the memo-key decision.)

## Evidence and confidence

- [documented] Temporal child workflows, Tekton Pipelines-in-Pipelines, Dagster asset checks, and Prifti validation tokens as dispatch preconditions are cited as prior art grounding the encapsulation pattern; none of these are measurements taken on this system.
- [inferred] The claim that "nothing about internal review, retries or escalation leaks into stage or workflow definitions" is a design guarantee stated by the row, not something confirmed by a spike or test run in this ledger.
- [documented] This decision is related to, and was folded into, D22 (ADR 0022) per the corpus's own note that PC6 and PC7 were folded into D22 — record as related, not superseded, since PC6 remains the source-of-record for the fuller rationale (Temporal/Tekton/Dagster grounding) that D22's summary compresses.
