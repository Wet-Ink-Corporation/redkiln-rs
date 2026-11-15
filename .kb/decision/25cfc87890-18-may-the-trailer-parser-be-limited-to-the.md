---
id: "kb-decision-25cfc87890"
title: "Final paragraph only"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#18: may the trailer parser be limited to the commit's final paragraph, given ADR-0155 clause 3 names column 0, ': ' and case as the only gates and the change moves tier-1 derivation?\", facing parse_body reads every column-0 'Key: value' line in a body as a trailer, so a quoted 'Story: x' mid-body can mint a phantom tier-1 signal, we decided for Final paragraph only and neglected Keep as is, name the residual; Shell to interpret-trailers --parse, on the premise that redkiln's own trailers always land in the final block (they are written by interpret-trailers), accepting that if wrong: a hand-placed mid-body trailer stops counting."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: medium
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-3f8ab271bf"
question: "#18: may the trailer parser be limited to the commit's final paragraph, given ADR-0155 clause 3 names column 0, ': ' and case as the only gates and the change moves tier-1 derivation?"
door: unsure
blast_radius: system
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T15:34:26Z"
tree_hash: "c2932d021bbd9f0a216c08b9ceae538ddf1ad60f"
recommended: "A"
flip_condition: "a redkiln-read commit with a trailer outside its final paragraph"
---

# Final paragraph only

## Context and problem statement

#18: may the trailer parser be limited to the commit's final paragraph, given ADR-0155 clause 3 names column 0, ': ' and case as the only gates and the change moves tier-1 derivation?

parse_body reads every column-0 'Key: value' line in a body as a trailer, so a quoted 'Story: x' mid-body can mint a phantom tier-1 signal.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-3f8ab271bf`. Anchor: `crates/rk-git/src/trailer/grammar.rs:229`.

## Decision drivers

- Phantom signals
- Cost

## Considered options

### A · Final paragraph only (chosen)

Successor record narrows ADR-0155 clause 3: trailers are read only from the last paragraph, git interpret-trailers semantics.

- Holds if redkiln's own trailers always land in the final block (they are written by interpret-trailers).
- If wrong: a hand-placed mid-body trailer stops counting.

### B · Keep as is, name the residual

Status quo; record phantom-trailer exposure as a known residual.

- Holds if hand-written commits on run refs stay rare.
- If wrong: a quoted line mints a phantom signal or Malformed drift.

### C · Shell to interpret-trailers --parse

Same rule as A, delegated to git through the wrapper.

- Holds if one extra git spawn per read is acceptable.
- If wrong: slower reconcile on long histories.

## Evidence

- `crates/rk-git/src/trailer/grammar.rs:231`: for line in body.lines() { ... split_once(": ")
- `references/adr/0155-one-story-trailer-per-item-per-lane-and-the-ac162-residual-is-named.md:115`: column 0, separator ": ", key case-sensitive are the three gates

## Decision outcome

Chosen option: **Final paragraph only**, the recommended option.

Decider's note: Ryan tapped A on the decision card 2026-11-15 15:34Z: trailers read from the final paragraph only, via a record amending ADR-0155 clause 3.

### Consequences

- Good, because it holds if redkiln's own trailers always land in the final block (they are written by interpret-trailers).
- Bad, because if wrong: a hand-placed mid-body trailer stops counting.

### Confirmation

Revisit when: a redkiln-read commit with a trailer outside its final paragraph

## Why this might be wrong

if any existing run ref relied on mid-body trailers, reconcile would lose them

## Provenance

- Decided 2026-11-15T15:34:26Z by human:claude (user-approved), via chat.
- Raised in session `f4c1a666-b730-54c4-86a3-2dd8c84d60e0`.
- Verbatim: "#18: may the trailer parser be limited to the commit's final paragraph, given ADR-0155 clause 3 names column 0, ': ' and case as the only gates and the change moves tier-1 derivation?"
