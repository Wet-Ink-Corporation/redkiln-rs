---
id: "kb-decision-wi-eb62f5"
title: "Which source should row 8i's passed count come from: A, a fresh run on your box at b3c746e; B, the CI windows log; or C, the entry commit on your box: A"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Which source should row 8i's passed count come from: A, a fresh run on your box at b3c746e; B, the CI windows log; or C, the entry commit on your box?\", facing the question an agent raised, we decided for A."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-19"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-eb62f5"
question: "Which source should row 8i's passed count come from: A, a fresh run on your box at b3c746e; B, the CI windows log; or C, the entry commit on your box?"
door: two-way
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-19T02:12:52Z"
tree_hash: "cf741283b2e67fa452a1b4b3cb8e303db184a284"
---

# Which source should row 8i's passed count come from: A, a fresh run on your box at b3c746e; B, the CI windows log; or C, the entry commit on your box: A

## Context and problem statement

Which source should row 8i's passed count come from: A, a fresh run on your box at b3c746e; B, the CI windows log; or C, the entry commit on your box?

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-eb62f5`. Anchor: `runbook/phase-1/s8j-run-preparation.md:2944`.

## Decision outcome

Chosen option: **A**.

Decider's note: Ryan 2026-10-19 01:56Z: 'go with A'. Measured at b3c746e on his Windows Git Bash: rc=0, 44 results, 0 with failures, passed=1432, ignored=9, nd_check silent, rust-src removed (R6).

## Provenance

- Decided 2026-10-19T02:12:52Z by human:ryan (user-directed), via cli.
- Raised in session `080afb48-86aa-5313-8ab8-967958b9d4d4`.
- Verbatim: "kind: question"
