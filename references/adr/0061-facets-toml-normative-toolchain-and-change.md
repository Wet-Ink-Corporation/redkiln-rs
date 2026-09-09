---
id: "0061"
title: facets.toml is the normative source for the toolchain and change facets
status: accepted
date: 2026-09-08
legacy_id: SCHEMA-31
source: references/2026-09-07-activity-graph-schema.md §9c amendment 31
supersedes: []
superseded_by: []
related: ["0029", "0030"]
evidence: []
consumers: []
---

## Context and problem

PC7 (technology-pack contribution contract) and PC8 (facets, not tags) both describe `toolchain` and `change` as facets in the context vector — PC7 defines `change_kinds` as `{text, binary, generated}` computed per item, and PC8 lists `toolchain`, `change` among the initial facets sourced from "the §2.4 context vector plus the activity's placement fields". Authoring the P6 catalog against these two descriptions plus `facets.toml` surfaced that the proposal prose and the registry file could drift, with no stated arbiter for which one governs the closed vocabulary and values when they disagree.

## Decision drivers

- PC8 rule 1 already requires a closed, enumerated vocabulary per facet, held in `facets.toml`, with lint rejecting unknown facets or values.
- Two prose descriptions of the same facets (process-catalog-proposal §2.4, PC7) is a live drift risk once `facets.toml` is the file lint actually reads.

## Considered options

### Option — do nothing
Leave both the proposal prose and `facets.toml` as co-equal descriptions of `toolchain` and `change`. Rejected implicitly: nothing in the source names an alternative, but leaving no normative source is exactly the ambiguity the amendment closes — lint enforces against a file, not against prose, so the file must win.

## Decision

`facets.toml`'s definitions of the `toolchain` and `change` facets are normative. Process-catalog-proposal §2.4 and PC7's descriptions of these two facets are read as accordant with `facets.toml`, not as an independent or competing specification.

## Consequences

**Positive**
- Removes the ambiguity between prose and registry for exactly the two facets PC7 depends on (per-item `toolchains`, `change_kinds`).
- Matches how lint actually validates facets (against the TOML file, per PC8 rule 1).

**Negative**
- The source says only that proposal §2.4 and PC7 are "read accordingly"; it does not say those documents are amended in place, so a reader consulting them for `toolchain`/`change` values must know to defer to `facets.toml`.

**What this forbids**
- Forbids treating the proposal's or PC7's prose enumeration of `toolchain`/`change` values as authoritative if it ever diverges from `facets.toml` — the TOML file, not the narrative document, is the source of truth for those two facets.

## Evidence and confidence

[documented] The amendment is adjudicated directly in the schema document's §9c batch, from the P6 authoring batch (90 activities, `spikes/F34-DRAFT.md` §4; decision recorded in `spikes/p9/DECISIONS-2026-09-08.md` decision 10, "31 accept (registry names normative)"). The drift itself is recorded as F34 B6, classed "new (doc drift)": "proposal/task: `toolchains`, `change_kinds`; `facets.toml`: `toolchain`, `change`" — the author used the registry names. No measurement is cited for this specific amendment — it is a naming/authority clarification, not a result. It does not license any claim about the correctness or stability of the `toolchain`/`change` values themselves.
