---
id: "kb-decision-0810092202"
title: "#18 failed the corpus check under final-paragraph-only (74 slices fell to 4): how should the trailer parser change instead: F"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#18 failed the corpus check under final-paragraph-only (74 slices fell to 4): how should the trailer parser change instead?\", facing the question an agent raised, we decided for F."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-c8535d08ea"
question: "#18 failed the corpus check under final-paragraph-only (74 slices fell to 4): how should the trailer parser change instead?"
door: two-way
decider: "human:claude"
provenance: user-directed
decided_at: "2026-11-15T17:08:04Z"
tree_hash: "2f9ff9123eab506fdde3b58380072b76dcfd4c83"
---

# #18 failed the corpus check under final-paragraph-only (74 slices fell to 4): how should the trailer parser change instead: F

## Context and problem statement

#18 failed the corpus check under final-paragraph-only (74 slices fell to 4): how should the trailer parser change instead?

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-c8535d08ea`. Anchor: `crates/rk-git/src/trailer/grammar.rs:229`.

## Decision outcome

Chosen option: **F**.

Decider's note: Ryan tapped F on the decision card 2026-11-15 17:05Z: read every all-trailer paragraph; fallback B if the F12 replay does not hold 74/15. Supersedes wi-3f8ab271bf's A, which the replay refuted (4 slices, 2 rejected).

## Provenance

- Decided 2026-11-15T17:08:04Z by human:claude (user-directed), via chat.
- Raised in session `f4c1a666-b730-54c4-86a3-2dd8c84d60e0`.
- Verbatim: "#18 failed the corpus check under final-paragraph-only (74 slices fell to 4): how should the trailer parser change instead?"
