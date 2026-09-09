---
id: "0064"
title: Tool steps are typed by their verb; emit may compose from several refs
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-34
source: references/2026-09-07-activity-graph-schema.md §9c amendment 34
supersedes: ["0041"]
superseded_by: []
related: ["0059"]
evidence: []
consumers: []
---

## Context and problem

§9c amendment 29 reshaped the toolchain-verb registry into one table per value (e.g. `[verb.bisect] outputs = "BisectReport" live = true timeout = "test_timeout"`), so a verb now carries its own output type. Before this, §9a.11 handled an activity with several possible outcomes (e.g. a rolled-back release) by requiring an unconditional final `tool` step that materialised the artifact from whichever branch ran, and `toolchain:record` was serving as a generic, untyped recording verb (11 uses in the batch, F34 B12). Authoring 90 activities against the reshaped registry surfaced that a `tool` step's declared `outputs` could disagree with its verb's registered output type, and that forcing a synthetic final `tool` step purely to assemble an artifact from several refs was a workaround the registry no longer needed.

## Decision drivers

- Amendment 29 gives every verb a registered output type; a `tool` step whose declared `outputs` disagrees with that type is now a detectable inconsistency, not a matter of author discretion.
- `toolchain:record` was being used as a catch-all producer with no typed contract: F34 B12 records it used 11 times for `ClosureRecord`/`ChangeRecord`/`ConfirmationResult`/…, with the verb registry listing names only, so "the lint accepts any `outputs` on any verb" and R3's schema check "is decorative for tools" — in tension with §1's "output is a typed report; never free text" rule for `tool` steps.
- §9a.11's "final tool step" pattern was a workaround for composing an artifact from several branches' outputs, not an actual toolchain invocation.

## Considered options

### Option — do nothing (verb-untyped tool outputs, final-tool-step composition)
Leave `tool` step `outputs` independently declarable regardless of the verb's registered type, and keep requiring a terminal `tool` step to assemble multi-branch artifacts per §9a.11. Not named as an alternative in the source, but it is the state the amendment replaces: it leaves the registry's new per-verb output typing (amendment 29) unenforced, and keeps forcing a fake toolchain invocation onto activities that only need to combine already-produced refs.

## Decision

A `tool` step's `outputs` must equal its verb's registered `outputs` (per the amendment-29 registry) or be omitted entirely. `toolchain:record` is removed as a verb. `emit` may compose an artifact from several refs — `Kind = { field = "${ref}", … }` — with each field checked against the target kind's schema. §9a.11's "final tool step" pattern is superseded by this composition mechanism for cases that are pure composition (no genuine toolchain invocation needed).

## Consequences

**Positive**
- Enforces amendment 29's per-verb output typing at the step level, closing a gap where a step could declare an `outputs` type inconsistent with its verb.
- Removes a generic, untyped verb (`toolchain:record`) that sat awkwardly against the "never free text" rule for tool outputs.
- Lets an activity assemble a multi-branch artifact directly in `emit`, without inventing a toolchain call that does no toolchain work.

**Negative**
- Any activity authored under §9a.11's final-tool-step pattern for pure composition must be rewritten to use `emit` composition instead.

**What this forbids**
- Forbids a `tool` step declaring `outputs` other than its verb's registered type (or omitting `outputs`).
- Forbids using `toolchain:record` — the verb no longer exists.
- Forbids relying on §9a.11's "final tool step" pattern for the pure-composition case; that case must go through `emit` composition instead.

## Evidence and confidence

[documented] Adjudicated in the §9c batch from the third author's 90-activity P6 catalog pass (`spikes/F34-DRAFT.md` §4; `spikes/p9/DECISIONS-2026-09-08.md` decision 8, taken as "A + C"), building directly on amendment 29's registry reshape. The only quantity attached is the size of the problem — 11 uses of `toolchain:record` (F34 B12), rewritten when the decisions were applied. No measurement of the rule's effect is cited; it is a typing/consistency rule, not a result.

This decision supersedes ADR 0041 (§9a.11, "alternative outcomes are expressed by an unconditional final `tool`/`check` step") only for the pure-composition case that `emit` composition now covers; the source's wording is that §9a.11's pattern "is superseded by composition for pure-composition cases", and it does not say the exit set or the final-step pattern is withdrawn where a genuine toolchain invocation is still needed. ADR 0041 is nonetheless carried at status "superseded" in the record map, pointing here.
