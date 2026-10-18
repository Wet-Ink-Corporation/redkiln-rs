---
id: "kb-decision-wi-f5c010"
title: "New record, partly supersede 0173"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Do you approve a new record that partly supersedes ADR-0173, so a human-attached root can be read by the next activity (D1)?\", facing an attached human root is never an emit binding, so read_artifact_input refuses it and the next activity cannot read it (D1), we decided for New record, partly supersede 0173 and neglected Keep ADR-0173 intact, on the premise that attached roots should be readable like emitted artifacts, accepting that if wrong: a second, record-level route to artifacts exists that ADR-0173 meant to forbid."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-18"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-f5c010"
question: "Do you approve a new record that partly supersedes ADR-0173, so a human-attached root can be read by the next activity (D1)?"
door: one-way
blast_radius: system
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-18T14:31:58Z"
tree_hash: "cf741283b2e67fa452a1b4b3cb8e303db184a284"
recommended: "A"
flip_condition: "ADR-0173 forbids a second artifact source on purpose"
---

# New record, partly supersede 0173

## Context and problem statement

Do you approve a new record that partly supersedes ADR-0173, so a human-attached root can be read by the next activity (D1)?

An attached human root is never an emit binding, so read_artifact_input refuses it and the next activity cannot read it (D1).

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-f5c010`. Anchor: `crates/rk-kernel/src/runtime/reference.rs:1141`.

## Decision drivers

- Records changed
- Code touched

## Considered options

### A · New record, partly supersede 0173 (chosen)

read_artifact_input also resolves a human-attached root; a probe of the fix is saved

- Holds if attached roots should be readable like emitted artifacts.
- If wrong: a second, record-level route to artifacts exists that ADR-0173 meant to forbid.

### B · Keep ADR-0173 intact

Attaching a root records an emit-shaped binding instead, with no reader change

- Holds if ADR-0173's single read path matters more than a smaller change.
- If wrong: the attach path forges a binding no activity emitted.

## Evidence

- `crates/rk-kernel/src/runtime/reference.rs:1141`: let Some((bound, producer)) = emit_binding_of(env.events, env.item, input.kind) else {

## Decision outcome

Chosen option: **New record, partly supersede 0173**, the recommended option.

Decider's note: tapped on the card 2026-10-18T14:31Z

### Consequences

- Good, because it holds if attached roots should be readable like emitted artifacts.
- Bad, because if wrong: a second, record-level route to artifacts exists that ADR-0173 meant to forbid.

### Confirmation

Revisit when: ADR-0173 forbids a second artifact source on purpose

## Why this might be wrong

if 0173's single source is load-bearing for replay

## Provenance

- Decided 2026-10-18T14:31:58Z by human:ryan (user-approved), via cli.
- Raised in session `080afb48-86aa-5313-8ab8-967958b9d4d4`.
- Verbatim: "kind: question"
