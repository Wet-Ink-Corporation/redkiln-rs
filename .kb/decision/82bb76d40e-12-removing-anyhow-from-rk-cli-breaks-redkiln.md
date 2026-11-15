---
id: "kb-decision-82bb76d40e"
title: "Remove both, rewrite test"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#12: removing anyhow from rk-cli breaks redkiln-migrate's graph test, which asserts anyhow's users are exactly rk-cli and redkiln-migrate (ADR-0102 d6, Checkpoint 7d). Rewrite that expectation, or remove only the unused proptest dev-dependency?\", facing rk-cli declares anyhow and proptest but uses neither; removing anyhow breaks a graph test that pins anyhow's users to exactly rk-cli and redkiln-migrate, we decided for Remove both, rewrite test and neglected Remove proptest only; Leave both, on the premise that rk-cli keeps its own Halt/Exit errors, accepting that if wrong: re-adding anyhow later is one line plus the test set."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-7f77ac3c50"
question: "#12: removing anyhow from rk-cli breaks redkiln-migrate's graph test, which asserts anyhow's users are exactly rk-cli and redkiln-migrate (ADR-0102 d6, Checkpoint 7d). Rewrite that expectation, or remove only the unused proptest dev-dependency?"
door: two-way
blast_radius: module
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T15:34:30Z"
tree_hash: "c2932d021bbd9f0a216c08b9ceae538ddf1ad60f"
recommended: "A"
flip_condition: "Ryan reads the graph test as pinning the permission, not the usage"
---

# Remove both, rewrite test

## Context and problem statement

#12: removing anyhow from rk-cli breaks redkiln-migrate's graph test, which asserts anyhow's users are exactly rk-cli and redkiln-migrate (ADR-0102 d6, Checkpoint 7d). Rewrite that expectation, or remove only the unused proptest dev-dependency?

rk-cli declares anyhow and proptest but uses neither; removing anyhow breaks a graph test that pins anyhow's users to exactly rk-cli and redkiln-migrate.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-7f77ac3c50`. Anchor: `crates/redkiln-migrate/tests/it/graph.rs:125`.

## Considered options

### A · Remove both, rewrite test (chosen)

Drop anyhow and proptest from rk-cli; test expects only redkiln-migrate (still a subset of ADR-0102 d6's permission).

- Holds if rk-cli keeps its own Halt/Exit errors.
- If wrong: re-adding anyhow later is one line plus the test set.

### B · Remove proptest only

No test change; anyhow stays unused.

- Holds if the test's exact set is meant to pin ADR-0102's permission.
- If wrong: an unused dependency lingers.

### C · Leave both

Close #12 as won't-fix.

- Holds if build time is not a concern.
- If wrong: misleading dependency signal stays.

## Evidence

- `crates/redkiln-migrate/tests/it/graph.rs:130`: assert_eq!(anyhow_users, ["redkiln-migrate", "rk-cli"]

## Decision outcome

Chosen option: **Remove both, rewrite test**, the recommended option.

Decider's note: Ryan tapped A on the decision card 2026-11-15 15:34Z: remove anyhow and proptest from rk-cli; graph test expects only redkiln-migrate.

### Consequences

- Good, because it holds if rk-cli keeps its own Halt/Exit errors.
- Bad, because if wrong: re-adding anyhow later is one line plus the test set.

### Confirmation

Revisit when: Ryan reads the graph test as pinning the permission, not the usage

## Why this might be wrong

the test may be deliberately pinning ADR-0102 d6's two-crate allowance

## Provenance

- Decided 2026-11-15T15:34:30Z by human:claude (user-approved), via chat.
- Raised in session `f4c1a666-b730-54c4-86a3-2dd8c84d60e0`.
- Verbatim: "#12: removing anyhow from rk-cli breaks redkiln-migrate's graph test, which asserts anyhow's users are exactly rk-cli and redkiln-migrate (ADR-0102 d6, Checkpoint 7d). Rewrite that expectation, or remove only the unused proptest dev-dependency?"
