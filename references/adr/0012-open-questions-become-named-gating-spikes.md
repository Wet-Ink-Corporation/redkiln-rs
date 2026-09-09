---
id: "0012"
title: Open questions become named, gating spikes, not prose
status: accepted
date: 2026-09-05
legacy_id: D12
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

The proposal carries a working set of unresolved questions about the design — verdict instability, interactive-session cache TTL, binary-artifact ecosystems and quality on a second harness are among those §2 lists as still open, and migration fidelity is among those §8 carries as a spike. Recorded only as narrative caveats, such questions have no owner, no cost estimate, and no defined point at which the design can proceed past them. The proposal needed a single, uniform place for "what we don't yet know" that forces each unknown to be tracked to closure.

## Decision drivers

- §8 already exists as a table of spikes, each with a question, a consumer (the decision or artifact the answer feeds), whether it gates Phase 1, and a cost.
- A hedge recorded in running text does not force anyone to resolve it or say what happens if they don't; §2's own open items (cache TTL, binary-artifact ecosystems) are carried as a table of hedges and, where they need an answer rather than a hedge, are pushed into §8 (§2's second-harness and binary-artifact rows both point at §8).

## Decision

Remaining unknowns are carried as §8 spikes, each with a named consumer, not as prose. Every open question in the design must appear as a row in §8 — Spike, Question, Feeds, Gates Phase 1?, Cost — rather than as a hedge inside the running text.

## Considered options

### Option — do nothing

Continue recording open questions as inline prose caveats scattered through the relevant sections. The source row's "Alternatives considered" column is empty ("—"), so no alternative is transcribed here; this is the status quo the rule displaces, reconstructed rather than quoted. It loses because a prose caveat has no named consumer, no cost, and no gating status, so nothing forces resolution and the design can silently proceed on an unverified assumption.

## Consequences

**Positive**
- Every open question has a named consumer (a decision or artifact it feeds) and a declared gating status, making it visible whether Phase 1 can proceed without an answer (§8: S-VAR, S-TYPED and S-MIG are marked "yes" and S-BIN "yes (type design)"; S-H1b, S-WT, S-RETRY, S-CODEX, R-LINEAR and R-DOCS are marked "no", S-RETRY with the qualifier "policy is configurable").

**Negative**
- Adds process overhead: an unknown cannot just be noted in passing, it must be turned into a spike row with a cost estimate before it counts as tracked.

**What this forbids**
- An open question may not be left as narrative hedge text in the proposal body; it must be entered into §8 or it is not considered tracked at all.

## Evidence and confidence

- [documented] The rule — "Remaining unknowns are carried as §8 spikes with a named consumer, not as prose" — is direct source text in the decision table (§9, D12). The row's "Alternatives considered" column is empty ("—") and it cites no finding, spike, or measurement.
- [documented] The §8 table the rule institutionalises is direct source text: five columns (Spike, Question, Feeds, Gates Phase 1?, Cost), ten named rows, and the preamble "each names the decision it feeds and whether it gates Phase 1".
- [inferred] The "do nothing" option and its losing reason are reconstructed from the rule itself, not transcribed from the row.
- This decision does not license any claim that the §8 list is complete, that its cost estimates are accurate, or that any spike has been run; §8 records questions and their gating status only, and the answers live in `FINDINGS.md` per §7's Phase 0 exit criterion.
