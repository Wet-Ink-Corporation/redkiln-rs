---
id: "0006"
title: Artifact layout C — split fields, opaque prose, rendered structure
status: accepted
date: 2026-09-05
legacy_id: D6
source: references/2026-09-05-architecture-proposal.md §9 (choice grounded in §4.4)
supersedes: []
superseded_by: ["0075"]
related: ["0005", "0025"]
evidence: []
consumers: []
---

## Context and problem

Given D5's rule that structured fields change only through CLI/web/gate answers while designated prose stays human-editable, the proposal still had to pick a concrete file layout that realises that split for every human-facing artifact (`spec.md`, `plan.md`, `story.md`). §4.4 lays out three candidate layouts in a comparison table (A — sidecar, B — embedded, C — split fields) and records C as chosen.

## Decision drivers

- The layout must not require parsing markdown to recover structure (§4.4's governing rule).
- The layout must let humans edit prose without their edits being silently ignored or requiring a re-import step.
- Precedent from tools that already split frontmatter/body successfully — "matches how Hugo/Obsidian/Backlog.md treat frontmatter + body."

## Considered options

### Option A — sidecar

`spec.json` canonical plus `spec.md` rendered and marked generated; edits happen only via CLI/web/gate answers and the markdown is read-only. Trade-off recorded in the source: "cleanest machine contract; two files per artifact in git review; humans who edit the `.md` are ignored (drift surfaced)." Rejected because it gives humans nowhere convenient to write prose that is actually kept.

### Option B — embedded

A single `spec.md` with a rendered body above and the canonical payload embedded as a trailing fenced block carrying a digest of the rendering. Edits to the rendered part are detected by digest mismatch, treated as typed drift, and either re-imported through a narrow LLM activity or rejected. Trade-off recorded in the source: "one file; git diffs show both; import path is a place structure can rot back in." Rejected because the re-import path reopens the structure-from-markdown parsing risk the design otherwise avoids.

### Option C — split fields (chosen)

Frontmatter plus declared *opaque prose* fields are human-editable; every structured field (ACs, boundary, status, findings) is rendered from types and read-only. Trade-off recorded in the source: "matches how Hugo/Obsidian/Backlog.md treat frontmatter + body; requires a section grammar, but only for *locating* opaque fields, not parsing their content."

## Decision

Layout C is chosen for human-facing artifacts: "typed frontmatter + rendered read-only structured sections + opaque prose" — concretely, human-facing artifacts (`spec.md`, `plan.md`, `story.md`) are typed frontmatter plus rendered structured sections (read-only, digest-stamped; an edit inside one is typed drift, surfaced not applied) plus opaque prose sections. Machine-only artifacts (`_review`, `_ledger`, digests, run reports) are instead JSON sidecars rendered to `.md` on demand — i.e. layout A's sidecar shape is retained, but only for the machine-only class, not for human-facing artifacts. "Nothing parses structure out of prose."

## Consequences

**Positive**
- Humans get exactly one place per artifact to write free prose, and it is never silently dropped (unlike option A) or subject to a re-import parse (unlike option B).
- The rendered structured sections are digest-stamped, so an edit landing inside one is detected as drift and surfaced rather than silently applied or silently lost.
- Follows an established pattern (Hugo/Obsidian/Backlog.md frontmatter + body) rather than a novel one.

**Negative**
- Requires a section grammar/locator to find the opaque prose fields, which is additional machinery beyond a plain sidecar (option A), even though the source notes this grammar only locates fields rather than parsing their content.

**What this forbids**
- No structured section of a human-facing artifact may be edited in place and kept; an edit there is typed drift, surfaced, never applied.
- Machine-only artifacts may not use layout C's prose-plus-frontmatter shape; they remain JSON sidecars rendered to markdown on demand.
- No re-import/parse-back-to-structure path (option B's mechanism) is used anywhere in this scheme.

## Evidence and confidence

- [documented] The three-option comparison table and its trade-off wording are taken directly from §4.4; no finding id or spike is cited for D6 in the source row or in §4.4's discussion of it.
- [inferred] The claim that option C "matches" Hugo/Obsidian/Backlog.md is the source's own stated precedent, not an independently measured comparison; it is recorded here as documented rationale, not as measured evidence.
- This ADR does not license a claim that layout C was benchmarked against A or B on any concrete metric (git review friction, drift incidence, etc.) — no such measurement is cited.
- [documented] Scope note: the source also carries a later **D5/D6 (rev. 4)** row amending the artifact model (AC type with optional GWT/verification, gates emitted not parsed, a sidecar schema per companion, `plan.md` removed, `artifact.read` events; §4.4). The `plan.md` named among human-facing artifacts above is §4.4's original list. That row is now recorded as [0075](0075-typed-artifacts-revision-4-refinements.md), which supersedes this record **partially** — this record stays `accepted` and carries `superseded_by: ["0075"]`. What 0075 withdraws from here is only `plan.md` from the human-facing artifact list, plus a refinement of the companion sidecars into one schema per kind; layout C itself, digest-stamped read-only structured sections, typed drift surfaced not applied, and "nothing parses structure out of prose" all stand. This record covers the original D6 choice only.
