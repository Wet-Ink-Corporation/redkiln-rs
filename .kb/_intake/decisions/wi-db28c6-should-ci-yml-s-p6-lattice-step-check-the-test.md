---
id: "kb-decision-wi-db28c6"
title: "Should ci.yml's P6 lattice step check the test's ok line with a here-string (grep -qxF '...' <<<\"$out\") instead of piping printf into grep -q, which can fail under pipefail even after the test passes: A"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should ci.yml's P6 lattice step check the test's ok line with a here-string (grep -qxF '...' <<<\"$out\") instead of piping printf into grep -q, which can fail under pipefail even after the test passes?\", facing the question an agent raised, we decided for A."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-18"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-db28c6"
question: "Should ci.yml's P6 lattice step check the test's ok line with a here-string (grep -qxF '...' <<<\"$out\") instead of piping printf into grep -q, which can fail under pipefail even after the test passes?"
door: two-way
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-18T21:08:57Z"
tree_hash: "cf741283b2e67fa452a1b4b3cb8e303db184a284"
---

# Should ci.yml's P6 lattice step check the test's ok line with a here-string (grep -qxF '...' <<<"$out") instead of piping printf into grep -q, which can fail under pipefail even after the test passes: A

## Context and problem statement

Should ci.yml's P6 lattice step check the test's ok line with a here-string (grep -qxF '...' <<<"$out") instead of piping printf into grep -q, which can fail under pipefail even after the test passes?

Raised by an agent (marker) as a deferral and captured by Weigh-In as `wi-db28c6`. Anchor: `.github/workflows/ci.yml:152`.

## Decision outcome

Chosen option: **A**.

Decider's note: Ryan tapped 'Apply both' 2026-10-18T21:08Z

## Provenance

- Decided 2026-10-18T21:08:57Z by human:ryan (user-directed), via card.
- Raised in session `080afb48-86aa-5313-8ab8-967958b9d4d4`.
- Verbatim: "kind: deferral"
