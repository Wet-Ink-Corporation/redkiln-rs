---
id: "kb-decision-wi-be9dcb"
title: "Kernel resolves baseline"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should the kernel read the catalog's `baseline` in `baseline..HEAD` as the run's fork point (D2), rather than rewriting the twelve catalog rows?\", facing twelve catalog activities use baseline..HEAD, but baseline is not a git ref, so they fail (D2), we decided for Kernel resolves baseline and neglected Rewrite the 12 rows, on the premise that baseline means the run's fork point everywhere, accepting that if wrong: a row meant some other base."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-18"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-be9dcb"
question: "Should the kernel read the catalog's `baseline` in `baseline..HEAD` as the run's fork point (D2), rather than rewriting the twelve catalog rows?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-18T14:32:04Z"
tree_hash: "cf741283b2e67fa452a1b4b3cb8e303db184a284"
recommended: "A"
flip_condition: "a row needs a base other than the fork point"
---

# Kernel resolves baseline

## Context and problem statement

Should the kernel read the catalog's `baseline` in `baseline..HEAD` as the run's fork point (D2), rather than rewriting the twelve catalog rows?

Twelve catalog activities use baseline..HEAD, but baseline is not a git ref, so they fail (D2).

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-be9dcb`. Anchor: `crates/rk-adapter-toolchain-rust/src/verbs.rs`.

## Decision drivers

- Files touched

## Considered options

### A · Kernel resolves baseline (chosen)

The kernel reads baseline as the run's fork point; the catalog rows stay as written

- Holds if baseline means the run's fork point everywhere.
- If wrong: a row meant some other base.

### B · Rewrite the 12 rows

Edit each catalog row to a real ref

- Holds if rows should name refs literally.
- If wrong: twelve rows each hardcode a ref the run cannot know in advance.

## Evidence

- `crates/rk-adapter-toolchain-rust/src/verbs.rs`: baseline..HEAD in 12 catalog activities

## Decision outcome

Chosen option: **Kernel resolves baseline**, the recommended option.

Decider's note: tapped on the card 2026-10-18T14:31Z

### Consequences

- Good, because it holds if baseline means the run's fork point everywhere.
- Bad, because if wrong: a row meant some other base.

### Confirmation

Revisit when: a row needs a base other than the fork point

## Why this might be wrong

if baseline was meant as a named tag

## Provenance

- Decided 2026-10-18T14:32:04Z by human:ryan (user-approved), via cli.
- Raised in session `080afb48-86aa-5313-8ab8-967958b9d4d4`.
- Verbatim: "kind: question"
