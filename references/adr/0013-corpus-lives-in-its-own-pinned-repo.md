---
id: "0013"
title: Corpus lives in its own pinned repo, not under spikes/
status: accepted
date: 2026-09-05
legacy_id: D13
source: references/2026-09-05-architecture-proposal.md §9
supersedes: []
superseded_by: []
related: []
evidence: []
consumers: []
---

## Context and problem

The spike corpus — fixtures, JSONL run data, and the spike runner — has been living under `spikes/` inside the main working tree. Phase 0 of the phasing plan (§7) puts the extraction in its own scope, before Phase 1 begins ("extract `redkiln-corpus` (D13)"). The decision row states that the corpus is then "depended on by SHA"; the reason — that it must be pinnable at a fixed commit rather than drift with the main repo's history — is the row's own phrasing carried forward, not an argument the source spells out.

## Decision drivers

- Phase 0's scope names the extraction directly ("extract `redkiln-corpus` (D13)"), so the corpus has to reach a pinnable, depend-by-SHA state before Phase 1 starts.
- The row groups fixtures, JSONL and the spike runner as one unit to move. §7's Phase 2 exit criterion — "migration differential green on the frozen corpus" — and §8's S-MIG row are the one consumer the source names for a corpus fixed at a point in time.

## Decision

The corpus becomes a separate `redkiln-corpus` repository: pinned SHA, fixtures, JSONL, and the spike runner all live there, and other work depends on it by SHA.

## Considered options

### Option — keep under `spikes/`

Leave the corpus inside the main repo's `spikes/` directory. Named in the row's "Alternatives considered" and rejected there without a stated reason; the reason reconstructed here is that it ties the corpus's state to the main repo's commit history, so it cannot be pinned independently or referenced by a stable SHA from a downstream consumer such as the migration differential (S-MIG, §8).

### Option — archive results only

Keep only the JSONL results archived somewhere, without moving the fixtures and spike runner into a dedicated repo. Named in the row's "Alternatives considered" and rejected there without a stated reason; the reason reconstructed here is that the row's own choice names fixtures, JSONL and the spike runner together as the unit that moves, so archiving results alone would leave the fixtures and runner without the same pinning guarantee.

## Consequences

**Positive**
- Downstream work (e.g. S-MIG's migration differential over "the frozen corpus", §7 Phase 2) can depend on an exact, pinned corpus SHA rather than a moving target inside the main tree.

**Negative**
- Adds a second repository to maintain and to keep in sync by SHA reference rather than by living in the same commit history as the code that consumes it.

**What this forbids**
- The corpus (fixtures, JSONL, spike runner) may not be edited in place inside the main repo's `spikes/` directory once extracted — changes go through the separate `redkiln-corpus` repo and a SHA bump, not an in-tree edit.

## Evidence and confidence

- [documented] The choice — a separate `redkiln-corpus` repo holding pinned SHA, fixtures, JSONL and the spike runner, depended on by SHA — and both alternatives are direct source text in the decision table (§9, D13).
- [documented] The extraction appears in §7 as a Phase 0 **scope** item ("extract `redkiln-corpus` (D13)"), not in Phase 0's exit criterion, which is "RFC amendment closed with F28+; §8 results in `FINDINGS.md`".
- [documented] The one named downstream consumer of a frozen corpus is the migration differential: §8's S-MIG row ("`redkiln-migrate` over the frozen corpus") and §7's Phase 2 exit criterion ("migration differential green on the frozen corpus").
- [inferred] The reasons the two alternatives lost are not stated in the source row and are reconstructed here from the choice's own wording.
- The source row cites no finding, spike, or measurement for D13, and no number is attached or invented here. This decision does not license any claim about how the corpus is hosted, versioned, or licensed beyond "separate repo, referenced by SHA", nor any claim that the extraction has already happened.
