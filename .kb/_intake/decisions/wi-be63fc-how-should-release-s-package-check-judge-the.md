---
id: "kb-decision-wi-be63fc"
title: "Compare trees outside .redkiln/"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"How should release's package check judge the Changeset against HEAD (D4)? Option (a) compares trees outside .redkiln/; option (b) takes the commit from the release stage's HEAD.\", facing package requires the Changeset sha to equal HEAD, but the run commits under .redkiln/ after it, so release always refuses (D4), we decided for Compare trees outside .redkiln/ and neglected Use release stage's HEAD, on the premise that only the run's own bookkeeping lands after the Changeset, accepting that if wrong: a product change under .redkiln/ ships unchecked."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-18"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-be63fc"
question: "How should release's package check judge the Changeset against HEAD (D4)? Option (a) compares trees outside .redkiln/; option (b) takes the commit from the release stage's HEAD."
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-18T14:32:29Z"
tree_hash: "cf741283b2e67fa452a1b4b3cb8e303db184a284"
recommended: "A"
flip_condition: "an existing record already decides which commit release packages"
---

# Compare trees outside .redkiln/

## Context and problem statement

How should release's package check judge the Changeset against HEAD (D4)? Option (a) compares trees outside .redkiln/; option (b) takes the commit from the release stage's HEAD.

package requires the Changeset sha to equal HEAD, but the run commits under .redkiln/ after it, so release always refuses (D4).

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-be63fc`. Anchor: `crates/rk-adapter-toolchain-rust/src/verbs.rs:881`.

## Decision drivers

- What is packaged

## Considered options

### A · Compare trees outside .redkiln/ (chosen)

package accepts HEAD when its tree matches the Changeset outside .redkiln/

- Holds if only the run's own bookkeeping lands after the Changeset.
- If wrong: a product change under .redkiln/ ships unchecked.

### B · Use release stage's HEAD

package takes the commit from the release stage's HEAD

- Holds if release should package whatever the stage sees.
- If wrong: a commit after the Changeset ships without its own check.

## Evidence

- `crates/rk-adapter-toolchain-rust/src/verbs.rs:881`: package requires sha == HEAD

## Decision outcome

Chosen option: **Compare trees outside .redkiln/**, the recommended option.

Decider's note: tapped on the card 2026-10-18T14:32Z

### Consequences

- Good, because it holds if only the run's own bookkeeping lands after the Changeset.
- Bad, because if wrong: a product change under .redkiln/ ships unchecked.

### Confirmation

Revisit when: an existing record already decides which commit release packages

## Why this might be wrong

if .redkiln/ content is meant to ship in the package

## Provenance

- Decided 2026-10-18T14:32:29Z by human:ryan (user-approved), via cli.
- Raised in session `080afb48-86aa-5313-8ab8-967958b9d4d4`.
- Verbatim: "kind: question"
