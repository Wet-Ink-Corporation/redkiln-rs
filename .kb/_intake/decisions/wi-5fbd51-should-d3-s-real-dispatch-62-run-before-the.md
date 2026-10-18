---
id: "kb-decision-wi-5fbd51"
title: "Move D3 after step 1b"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should D3's real dispatch (#62) run before the fixture-proof evidence line, as approved, even though D21-47 finding 7 and Checkpoint 8j (6) require every paid run to come after it?\", facing the approved plan runs D3's paid dispatch first, but D21-47 finding 7 and Checkpoint 8j (6) say every paid run comes after the fixture-proof evidence line, we decided for Move D3 after step 1b and neglected Waive finding 7 once, on the premise that the D3 evidence does not need to come first, accepting that if wrong: D3's evidence lands one step later."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-18"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-5fbd51"
question: "Should D3's real dispatch (#62) run before the fixture-proof evidence line, as approved, even though D21-47 finding 7 and Checkpoint 8j (6) require every paid run to come after it?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-18T14:31:46Z"
tree_hash: "cf741283b2e67fa452a1b4b3cb8e303db184a284"
recommended: "A"
flip_condition: "D3's result is needed to write the proof line"
---

# Move D3 after step 1b

## Context and problem statement

Should D3's real dispatch (#62) run before the fixture-proof evidence line, as approved, even though D21-47 finding 7 and Checkpoint 8j (6) require every paid run to come after it?

The approved plan runs D3's paid dispatch first, but D21-47 finding 7 and Checkpoint 8j (6) say every paid run comes after the fixture-proof evidence line.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-5fbd51`. Anchor: `runbook/phase-1/s8j-run-preparation.md`.

## Decision drivers

- Paid spend before proof

## Considered options

### A · Move D3 after step 1b (chosen)

D3's dispatch runs after the fixture-proof line, as D21-47 requires

- Holds if the D3 evidence does not need to come first.
- If wrong: D3's evidence lands one step later.

### B · Waive finding 7 once

Run D3 first as approved, with a dated waiver of D21-47 finding 7 for this run

- Holds if D3 must precede the proof line.
- If wrong: a paid run happens without the fixture proof behind it.

## Evidence

- `runbook/phase-1/s8j-run-preparation.md`: §8j.6 step 1a, marked not to run until settled

## Decision outcome

Chosen option: **Move D3 after step 1b**, the recommended option.

Decider's note: tapped on the card 2026-10-18T14:31Z

### Consequences

- Good, because it holds if the D3 evidence does not need to come first.
- Bad, because if wrong: D3's evidence lands one step later.

### Confirmation

Revisit when: D3's result is needed to write the proof line

## Why this might be wrong

if the proof line depends on D3's output, A deadlocks

## Provenance

- Decided 2026-10-18T14:31:46Z by human:ryan (user-approved), via cli.
- Raised in session `080afb48-86aa-5313-8ab8-967958b9d4d4`.
- Verbatim: "kind: question"
