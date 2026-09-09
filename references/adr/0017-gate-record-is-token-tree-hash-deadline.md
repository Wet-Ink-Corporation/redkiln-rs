---
id: "0017"
title: Gate record binds a single-use token to the tree hash, with a deadline and terminal path
status: accepted
date: 2026-09-07
legacy_id: D17
source: references/2026-09-05-architecture-proposal.md §4.6
supersedes: []
superseded_by: []
related: []
evidence: ["F28", "S-H1b"]
consumers: []
---

## Context and problem

A human gate answer can arrive after the state it was based on has moved (the tree the human reviewed is no longer the tree at HEAD), and a gate with no expiry can hang indefinitely with no defined next action. Comparable platforms (Temporal, Restate, Step Functions, GitHub environments) all mint a token for a gate/approval, but of these only GitHub also pins a SHA — and even that is weaker than binding to the tree the human actually saw. Separately, the cost analysis of resuming after a gate (a prefix rewrite: 18.6k tokens under the default prompt at both 66 min and 49.7 h elapsed; ~30k under a card prompt at 66 min, with the 24 h case unmeasured per the F28 correction and S-H1b) established that gate *duration* is free and gate *frequency* is marginal, which removed one argument against letting a gate sit open.

## Decision drivers

- Existing platform precedent mints a token per gate, but only GitHub also pins a SHA, and a SHA is a weaker binding than the full tree hash the human actually reviewed.
- Resume cost is dominated by a single prefix rewrite regardless of how long the gate stayed open (18.6k default-prompt at 66 min and 49.7 h; ~30k card-prompt at 66 min), so a longer deadline does not by itself add cost.
- An open gate needs a defined terminal path rather than an indefinite hang.

## Considered options

### Option — item-keyed answer
Key the answer to the item under review rather than to a specific tree state. Rejected: this does not detect the case where the item has drifted since the human looked at it — an answer keyed only to the item can be reconciled against a state the human never saw.

### Option — reconcile stale answers
Accept an answer whose tree hash no longer matches the current tree, and reconcile it anyway (e.g., re-apply best-effort). Rejected: the source explicitly rules this out — a stale answer is void, not reconciled, because reconciling it would apply a decision made against a tree state that no longer exists.

## Decision

"The gate record is `(token, tree_hash, deadline)` — a single-use token, the tree hash the human saw, and an expiry with a terminal path (fail / escalate / auto-decide, never a hang); an answer whose tree hash no longer matches is void, not reconciled." Binding to the tree hash is treated as stronger than the SHA-pinning done by GitHub environments, the closest precedent among Temporal, Restate, Step Functions and GitHub environments.

## Consequences

**Positive**
- A drifted tree cannot silently absorb a stale approval: the tree-hash binding makes drift detectable at the point the answer is applied, not later.
- Because resume cost is dominated by a single prefix rewrite independent of elapsed time, a gate is free to stay open as long as needed without an added cost penalty for duration; the source calls the residual cost of gate frequency marginal.
- The terminal path (fail / escalate / auto-decide) means an expired gate always has a defined next action; the design explicitly rules out an indefinite hang.

**Negative**
- Any drift at all — however small — voids the answer outright; there is no partial-reconciliation path, so a human's work in answering a gate can be discarded and must be redone if the tree moved before it landed.
- The token, tree hash and deadline all have to be computed, stored and checked on every gate, rather than a simpler item-keyed scheme.

**What this forbids**
- Forbids keying an answer only to the item (rejected alternative): item identity alone does not detect drift.
- Forbids reconciling a stale answer (rejected alternative): a tree-hash mismatch must produce a void result, not a best-effort merge.
- Forbids a gate with no terminal path — an expiry must resolve to fail, escalate, or auto-decide, never leave the gate hanging.

## Evidence and confidence

- [documented] Comparator platforms (Temporal, Restate, Step Functions, GitHub environments) all mint a token; only GitHub also pins a SHA. This is presented as prior art, not as a measurement, and the claim that tree-hash binding is "stronger" than SHA-pinning is the source's own reasoning, not an independently measured result.
- [measured] Resume/prefix-rewrite cost: 18.6k tokens under the default prompt at both 66 min and 49.7 h elapsed; ~30k under a card prompt at 66 min elapsed. The 24 h case under the card prompt is explicitly noted as "read untaken" (unmeasured) — this record does not claim that figure exists.
- [documented] F28 correction and S-H1b together ground the "gate duration is free, gate frequency is marginal" framing used as a decision driver; the correction itself is not restated here beyond noting its role.
- This record does not license treating every possible drift scenario as covered — only the tree-hash-mismatch case is specified as void; other drift shapes are not enumerated in the source.
