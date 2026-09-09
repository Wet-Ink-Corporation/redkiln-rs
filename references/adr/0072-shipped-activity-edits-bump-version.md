---
id: "0072"
title: Coherence edits to shipped activities bump version and enter the changelog
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-42
source: references/2026-09-07-activity-graph-schema.md §9c amendment 42
supersedes: []
superseded_by: []
related: ["0027"]
evidence: []
consumers: []
---

## Context and problem

The P6 authoring batch involved a third author writing 90 activities and running the P6 lattice across the whole
catalog. Editing an already-shipped activity for catalog coherence — the source does not enumerate what such edits
are — changes its behaviour for every pack that selects it, and the schema already keys memoisation
on `activity id@version` (§5, step key and activity key both hash the activity's version). Without a rule tying
catalog-coherence edits to a version bump, an edit could silently invalidate or fail to invalidate memoised results and
leave no record of why an activity changed.

## Decision drivers

- Both the step key and the activity key (§5) open with `hash(activity id@version, …)`. Card edits are caught
  separately by the card digests those keys also hash; an edit to the activity's own non-card fields has no handle in
  the key other than the version component, so a version that does not move leaves such an edit invisible to
  memoisation. (This reading of §5 is the ADR's; amendment 42 does not itself mention the memo key.)
- The P6 lattice is already the mechanism that checks reachability across the whole catalog (PC5: "every shipped
  activity must pass P6 lint... and be executed at least once before the pack is tagged"); it is the natural test that
  a coherence edit did not break something elsewhere.

## Considered options

The source row records no alternatives column and no rejected option; the option below is reconstructed as the status
quo the amendment displaced, with a reason drawn from §5 rather than from the row.

### Option — do nothing (edit shipped activities in place without a version bump)
Lost: this would change behaviour for every pack selecting the activity, with nothing but the version component for
the memo keys to notice a non-card edit by, and no changelog record of what changed or why.

### Option — bump `version`, changelog the edit, re-run the P6 lattice (chosen)
Adopted as stated in the amendment.

## Decision

Edits to shipped activities made for catalog coherence bump `version` and are listed in the pack changelog. The P6
lattice is the test that such an edit did not break reachability.

## Consequences

**Positive**
- Keeps the memo keys' version component meaningful: a coherence edit is guaranteed to move `version`, so results
  memoised against the pre-edit activity are invalidated.
- Gives every coherence edit a changelog record and a reachability test (the P6 lattice), rather than relying on the
  editor's memory or code review alone to catch a break.

**Negative**
- Adds process overhead to what might be a small, low-risk edit (e.g. a rank number) — every coherence edit now
  requires a version bump, a changelog entry, and a full P6 lattice run to be considered complete.

**What this forbids**
- Forbids editing a shipped activity's definition for catalog coherence without also bumping its version and recording
  the change in the pack changelog.
- Forbids treating a coherence edit as complete without having re-run the P6 lattice against it.

## Evidence and confidence

- [documented] The rule is stated directly in §9c amendment 42.
- [documented] The connection to the memo key's version component is stated in §5 of the same document ("Step key =
  hash(activity id@version, ...)"), though §5 predates this amendment and is not itself part of the P6 batch.
- No finding id or specific measurement (e.g. how many coherence edits occurred, or what the lattice caught) is cited
  for this amendment; the `evidence` field is left empty accordingly.
- This licenses only the process rule as stated. It does not license any claim that the rule has been applied to a
  particular edit, that the changelog exists yet, or that the P6 lattice detects every class of breakage a coherence
  edit could cause — the source calls it the test for reachability specifically.
