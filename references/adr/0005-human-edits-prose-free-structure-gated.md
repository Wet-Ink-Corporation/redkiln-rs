---
id: "0005"
title: Human edits are prose-free and structure-gated
status: accepted
date: 2026-09-05
legacy_id: D5
source: references/2026-09-05-architecture-proposal.md §9 (choice grounded in §4.4)
supersedes: []
superseded_by: ["0075"]
related: ["0006", "0004"]
evidence: []
consumers: []
---

## Context and problem

§4.4 establishes that every activity output is a typed Rust value and that the store renders markdown from that value — "the store **renders** markdown from the value with `minijinja` templates and **never parses markdown to recover structure**." That rule needs an answer for what a human is allowed to touch directly in the `.md` file without breaking the typed-value/rendering contract. D5 answers that question: designated prose sections are directly editable; everything structured is not.

## Decision drivers

- The no-parsing-markdown-to-recover-structure rule in §4.4 rules out any mechanism that would require inferring structured fields back out of free-form edits.
- A need for humans to be able to write unstructured commentary (rationale, notes) without going through CLI/web/gate machinery for every sentence.
- The old redkiln's `template_sig`/`rendered_sig` drift is named in §4.4 as "the failure to avoid" — a hazard specific to systems that let rendered output and canonical data disagree silently.

## Considered options

### Option — everything via forms

Require every edit, including free-text commentary, to go through CLI, web, or gate-answer forms; the `.md` file is entirely read-only to humans. Rejected implicitly by D5's choice to allow direct editing of designated prose sections — the row records this as a live alternative, and the decision explicitly carves out prose sections that are edited freely rather than through forms.

### Option — free edit + LLM re-import

Let humans edit the whole file freely and re-import any drift back into structured fields via an LLM step that reparses the markdown. Rejected: this is the exact "parses markdown to recover structure" pattern the typed-artifacts design in §4.4 forbids, and it reintroduces the rendered/canonical drift risk the old redkiln suffered from (`template_sig`/`rendered_sig` drift).

## Decision

"Humans edit *designated prose sections* of the `.md` directly in an editor or PR; every structured field (ACs, boundary, status, findings) changes only through CLI, web or gate answers and is re-rendered." "The one piece of markdown logic that survives" is a **section locator** — "a heading-anchored slice, not a grammar" — used only to find where the opaque prose lives, never to parse its content, and it is tested as such.

## Consequences

**Positive**
- Humans retain a low-friction place to write free-form rationale and notes without needing CLI/web/gate ceremony for every remark.
- The typed-value/rendering contract in §4.4 stays intact: structured data never round-trips through markdown parsing.

**Negative**
- The row and the surrounding prose do not quantify how often humans currently edit structured-looking content outside the designated prose sections, or how disruptive re-rendering over such edits is in practice; no measurement is cited for this row.

**What this forbids**
- No editor, PR, or human edit may change a structured field (ACs, boundary, status, findings) directly in the `.md` — such changes must go through CLI, web, or gate answers.
- The section locator may not become a grammar that parses the content of prose sections; it may only locate a heading-anchored slice.
- Markdown is never parsed to recover structure anywhere in this scheme, per the governing §4.4 rule.

## Evidence and confidence

- [documented] The decision and its stated form ("prose free, structure gated") come directly from the D5 row and are elaborated, not altered, by §4.4's "Human edits (D5)" paragraph.
- [inferred] The drift risk of the rejected "free edit + LLM re-import" option is inferred from §4.4's naming of `template_sig`/`rendered_sig` drift as "the failure to avoid," which is stated as a general design hazard rather than measured specifically against this alternative.
- No finding id or spike is cited for D5 in the source row or in §4.4's "Human edits (D5)" paragraph; this ADR's evidence array is empty accordingly, and no finding id has been supplied here.
- This ADR does not license a claim that the prose/structure split was measured, trialled, or compared against the rejected alternatives on any metric: the source records a design choice and its rationale only.
- [documented] Scope note: the source also carries a later **D5/D6 (rev. 4)** row amending the artifact model (AC type with optional GWT/verification, gates emitted not parsed, a sidecar schema per companion, `plan.md` removed, `artifact.read` events; §4.4). That row is now recorded as [0075](0075-typed-artifacts-revision-4-refinements.md), which supersedes this record **partially** — this record stays `accepted` and carries `superseded_by: ["0075"]`. What 0075 withdraws from here is only the implication that a structured field's shape is fixed independently of what the corpus contains; the prose-free/structure-gated rule, the CLI/web/gate-only write path and the locator-not-a-grammar rule all stand. This record covers the original D5 choice only.
