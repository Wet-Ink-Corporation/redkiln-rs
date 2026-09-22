---
id: "0073"
title: Toolchain-keyed artifact kinds replace the specificity tie-break for mixed-toolchain items
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-43
source: references/2026-09-07-activity-graph-schema.md §9c amendment 43
supersedes: ["0029"]
superseded_by: ["0132"]
related: ["0014", "0057"]
evidence: ["F35 §1.3"]
consumers: []
---

## Context and problem

PC7(ii) had ruled that a tech pack contributes producers for existing artifact kinds selected "by specificity —
toolchain match beats generic, equal-specificity ties are a validation error." The P6 lattice run (F35) measured what
this rule actually did to a mixed-toolchain item: a story touching both Node and UE received only the UE change
summary, and the source records that this happened **silently**. It does not spell out the mechanism further; the
reading that the UE-specific producer's specificity beat the generic producer for a single `ChangeSummary` slot is
this ADR's, from PC7(ii)'s own rule.

This amendment withdraws one clause of PC7 (ADR 0029, PC7(ii)'s tie-break). The rest of PC7 — the fact, producer,
artifact-kind and override contract, and the prohibition on stages and `require` rows — stands. This is therefore a
**partial** supersession, recorded the way this retrofit records every partial (cf. 0059 over 0043, 0020 over 0015):
the link is carried on both sides at record granularity, and 0029 stays at status "accepted" because only one clause
of it is withdrawn.

## Decision drivers

- Measured failure: under the old specificity tie-break, a mixed Node+UE item received the UE `ChangeSummary` only,
  silently (F35 §1.3) — the opposite of PC7's own stated goal, since PC7(i) already required "a story touching Node + UE
  content needs both change summaries" as a fact-level matter.
- The specificity rule was designed for a single producer slot per kind; it had no way to represent "one instance of
  this kind per toolchain," which is what a genuinely mixed item needs.

## Considered options

### Option — do nothing (keep PC7(ii)'s specificity tie-break)
Rejected: this is the option the P6 lattice measured and found broken — it silently drops non-winning toolchains'
artifacts for kinds that should exist once per toolchain, rather than raising the validation error PC7(ii) intended for
genuine ties.

## Decision

Artifact kinds that vary meaningfully by toolchain are declared `keyed_by = "toolchain"` in `artifacts.toml`:
`ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`. A required keyed kind expands to one instance
per element of the item's `toolchain` set, with one producer per instance. PC7(ii)'s "specificity beats generic"
tie-break is withdrawn — the source states the withdrawal without qualifying it to the keyed kinds — and a generic
producer's `'text' in change` condition becomes an instance-level predicate instead. What replaces the tie-break for
kinds that are *not* keyed is not stated in the source, and is not asserted here.

## Consequences

**Positive**
- Fixes the measured silent-drop failure: a mixed Node+UE item now gets one `ChangeSummary` instance per toolchain
  instead of only the more "specific" one winning the single slot.
- Keeps PC7(i)'s original intent (mixed-toolchain items need per-toolchain facts and, by extension, per-toolchain
  artifacts) actually realised in the artifact model, not just the context-vector facts.

**Negative**
- Adds a new registry mechanism (`keyed_by` in `artifacts.toml`) and a new expansion step (one instance per toolchain
  element) that the selection engine must implement and that catalog authors must understand.
- Generic producers must now be written as instance-level predicates (`'text' in change`, evaluated per instance)
  rather than item-level predicates, which is a semantic shift for any producer already written against the old model.

**What this forbids**
- Forbids relying on PC7(ii)'s specificity-beats-generic tie-break: it is withdrawn, not demoted to a fallback. For
  the five keyed kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`) keyed expansion plus
  an instance-level predicate replaces it outright.
- Forbids a single producer instance from silently standing in for multiple toolchains' worth of a keyed kind — one
  producer per toolchain instance is required.

## Evidence and confidence

- [measured] "Under the old rule a mixed Node+UE item received the UE change summary only, silently" — cited as F35
  §1.3. This is the specific finding that forced the withdrawal; it demonstrates the failure mode, not a broader
  quantification of how often it occurs across the catalog.
- [documented] The mechanism (`keyed_by`, one instance per toolchain element, one producer per instance, predicate
  becomes instance-level) is stated normatively in §9c amendment 43.
- This evidence licenses the claim that the old tie-break silently dropped artifacts for at least one measured
  mixed-toolchain case; it does not license a claim about the frequency of mixed-toolchain items in the wider catalog,
  which is not measured here.
