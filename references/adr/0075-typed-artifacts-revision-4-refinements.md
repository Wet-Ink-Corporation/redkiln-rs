---
id: "0075"
title: Typed artifacts, revision-4 refinements — permissive AC type, emitted gates, sidecar per companion, no plan.md
status: accepted
date: 2026-09-08
legacy_id: D5/D6 rev.4
source: references/2026-09-05-architecture-proposal.md §4.4 (row **D5/D6 (rev. 4)** in §9); census data in references/2026-09-07-design-recommendation.md §5
supersedes: ["0005", "0006"]
superseded_by: []
related: ["0019", "0025", "0073", "0074"]
evidence: ["S-MIG", "F25"]
consumers: []
---

## Context and problem

0005 (D5) settled *what a human may edit* — designated prose freely, structured fields only through CLI, web or gate answers — and 0006 (D6) settled *the file layout that realises it* — typed frontmatter plus rendered read-only structured sections plus opaque prose, with machine-only artifacts as JSON sidecars. Both were taken on design reasoning alone; both records say so, and both carry an empty `evidence` array.

S-MIG has since run the census those records lacked: `redkiln-migrate` over the frozen corpus, **1,312 files / 6,877 sections, zero model cost**. It measured how much of the old `.bklg` actually reclassifies into the proposed types, and it did not come back uniform. Spec sections lift at **98% overall** and frontmatter is regular everywhere (209/209 cards, 153/153 specs), but three section kinds fall below the 80% reclassify threshold, one of them catastrophically. It also produced two findings that are about the product rather than the parser: an artifact that has never once been filled in, and an evidence column that has never once been written.

So the layout stands and its type decisions do not. 0005 and 0006 each left one clause under-specified — 0005 never said what shape a structured field takes when the humans who wrote it did not follow the shape, and 0006 named `plan.md` in the human-facing set without checking whether anyone fills it. This record withdraws those two clauses and leaves the rest of both records standing.

## Decision drivers

- **The corpus does not obey a strict AC grammar.** 19 of 153 specs — **12%**, all shipped and reviewed — write criteria without GIVEN/WHEN/THEN or without a verification note. A type that rejects them rejects working history; a type that accepts them needs the *policy*, not the type, to decide what a missing verification costs.
- **Gate prose does not reclassify.** Gate results are free-form in **25% of implementation reports and 83% of reviews**. Nothing that lifts at 17% can be parsed as structure — but the toolchain that produces the result can emit it typed.
- **Companions are not one shape.** `_integration`, `_grounding` and `_decomposition` are three different shapes, and 10 of 28 companions carry no frontmatter at all.
- **`plan.md` is dead.** **110 of 110** are the unfilled scaffold; planning actually lived in `_plan.md`, `_storymap.md` and the spec. Nobody instruments downstream reads, so nobody noticed.
- **The mutation column does not exist.** The F25 remediation's per-AC mutation evidence appears in **0 of 156** historical implementation reports, and TDD evidence therefore migrates with `mutation: unknown` on 142 of 156 rows. This is 0074's "green is not pinned" seen from the artifact side.
- The governing constraint from 0006 is unchanged and binds every option below: nothing parses structure out of prose, and the old redkiln's `template_sig`/`rendered_sig` drift is the failure to avoid.

## Considered options

### Option — strict AC grammar

Require `{ id, criterion, gwt, verification }` with both `gwt` and `verification` mandatory, and fail migration on anything else. Rejected on the census: it fails 12% of shipped, reviewed specs at the type boundary, which converts a policy question ("should this AC have named its verifier?") into a migration error, and gives the policy no way to answer it differently per stage or per rigor.

### Option — parse the gate prose

Keep gate results as free text and lift them with a parser or a narrow LLM re-import step. Rejected twice over: at 17% reclassify on review gate tables there is no grammar to find, and re-import is the exact structure-from-markdown path 0006 already forbids.

### Option — one schema for all companions

Give `_integration`, `_grounding`, `_decomposition` and `_ledger` a single union schema. Rejected: they are three shapes, so a union collapses to a mostly-optional record that validates almost nothing, and it would drag `oneOf` nesting into the type set that §4.4's design rules exclude.

### Option — keep `plan.md` and fix the process

Retain `plan.md` as a human-facing artifact and treat 110/110 unfilled as a discipline problem. Rejected: an artifact that has never been filled once across the whole corpus is not under-used, it is unwanted, and keeping it costs a rendering path, a schema, a locator and a stage whose output nothing reads.

## Decision

The typed-artifact model is refined as follows. The layout itself (0006's layout C) and the human-edit rule (0005) are unchanged.

1. **The AC type is `{ id, criterion: text, gwt: Option<Gwt>, verification: Option<text> }`.** The type accepts a criterion with neither GWT nor a verification note, because 12% of shipped specs write one. **The policy — not the type — decides what a missing `verification` blocks**, per stage and per rigor.
2. **Gate results are emitted by the toolchain and rendered, never parsed.** The toolchain adapter produces a typed gate result; the report renders it. Migration fills `unknown` where the historical value was prose.
3. **Each companion is a sidecar with its own schema** — one schema per companion kind (`_integration`, `_grounding`, `_decomposition`, `_ledger`), not a shared union. The 2 `_ledger.md` files whose YAML does not parse take a hand-fix path.
4. **`plan.md` does not exist in Redkiln-RS.** It is removed from the human-facing artifact set 0006 named. Planning lives in the story map, the rollup and the spec.
5. **Every activity input emits an `artifact.read` event**, and `doctor` flags any stage whose artifact has zero downstream reads over N runs. This is how clause 4's finding is detected next time by the product rather than by a census.
6. **Three section kinds reclassify below the 80% threshold and are therefore carried as opaque prose, not as typed sections**: implementation-report gate sections (75%), companion frontmatter (42%) and the review gate table (17%). They are located by the section locator 0005 already permits and rendered verbatim; nothing derives structure from them.
7. **Per-AC mutation evidence is a first-class column on the TDD evidence type**, populated going forward and migrated as `mutation: unknown`. `unknown` is a distinguishable value, never an implied pass.
8. **Gate-checked criteria are phrased as EARS single-trigger statements by preference**, with Gherkin reserved for criteria that ship with a runnable feature file (adopted from evidence-augmentation §4 rec 20). `gwt` stays `Option<Gwt>` — the census requires the type to be permissive — but the recommended phrasing for a criterion a lint or gate must act on is EARS, not GWT.

## Consequences

**Positive**
- The migration is total: with a permissive AC type and three section kinds carried as prose, every item in the frozen corpus round-trips without a hand-written exception list.
- The gate result becomes trustworthy at the point of use — it is produced by the thing that ran, not recovered from the thing that described it — which is the precondition 0074's clause 3 re-execution depends on.
- Removing `plan.md` deletes a schema, a template, a locator and a stage output in one move, with a measured basis rather than an intuition.
- `artifact.read` makes dead artifacts a product signal. No surveyed system instruments downstream artifact reads; Agent OS v3 pruned its orchestration phases by intuition and Spec Kit's markdown-to-LOC ratio is the same finding unmeasured.
- The mutation column being explicitly `unknown` on 142 of 156 migrated rows makes the F25 gap visible in the data instead of absent from it.

**Negative**
- Clause 1 moves a real question from the type system, where it is checked once, into the policy table, where it must be answered per stage — and the policy table is now carrying this alongside 0074's class-to-block mapping. That table is the concentration point for two separate decisions.
- Three section kinds carried as opaque prose are three places where information exists in the corpus and is not queryable. The 17% review gate table is the worst of them, and clause 2 fixes it only going forward.
- One schema per companion is four schemas to keep coherent, and the census found companions to be the least regular part of the corpus.
- Clause 8 is a phrasing preference with no enforcement, and the external evidence for it is a norm claim about requirement grammars rather than a measurement on this corpus.

**What this forbids**
- Forbids an AC type that requires `gwt` or `verification`, and forbids the migration failing an item because a criterion lacks either.
- Forbids the type deciding what a missing `verification` blocks; that is the policy's decision.
- Forbids parsing a gate result out of report or review prose, in the migration or at runtime, by regex or by LLM re-import.
- Forbids a single shared schema across companion kinds, and forbids a companion with no schema at all.
- Forbids `plan.md` — creating it, templating it, rendering it, or naming it as an input.
- Forbids an activity consuming an input without emitting `artifact.read`.
- Forbids typing a section kind that reclassifies below the 80% threshold; forbids equally silently dropping it rather than carrying it as prose.
- Forbids rendering `mutation: unknown` as a pass, a blank, or an omission.

## Evidence and confidence

- [measured] S-MIG, zero model cost, **1,312 files / 6,877 sections** over the frozen corpus. Reclassify rates: spec sections **98%** overall; acceptance criteria **88%** strict (the 12% shortfall is 19 of 153 specs lacking GWT or a verification note); implementation-report gates **75%**; review gate table **17%**; companion frontmatter **42%**. Threshold **80%** — three kinds below it, hence clause 6.
- [measured] S-MIG structural counts: frontmatter regular at 209/209 cards, 153/153 specs, 46/47 atom files carrying `id`; **110/110 `plan.md` unfilled**; 10 of 28 companions with no frontmatter; 2 `_ledger.md` files with unparseable YAML.
- [measured] F25 / S-MIG: the per-AC mutation column exists in **0 of 156** historical implementation reports; TDD evidence migrates with `mutation: unknown` on 142 of 156 rows.
- [documented] Against S-MIG's four pre-registered criteria, **2 of 4 held**: anchors/boundary ≥90% ✓ and kb bodies opaque ✓; ACs ≥95% ✗ (88%) and TDD <80% ✗ (91%). The two misses are what clauses 1 and 7 are for — they name type decisions, not parser defects — and this record acts on the misses rather than reporting a pass.
- [documented] **The corpus and the research pass disagree about `gwt`.** The corpus keeps `gwt: Option<Gwt>` as the AC's structured shape; the research pass argues a well-formed EARS statement ("While ⟨precondition⟩, the ⟨system⟩ shall ⟨response⟩") is closer to machine-checkable than Gherkin, whose fixed keyword patterns constrain it to one measurable trigger-response pair, and that Gherkin is checkable only once wired to a step-definition harness — so `gwt` "is the wrong default shape for gate-checked criteria" (evidence-augmentation §3 Decision records, §4 rec 20; [peer] Mavin et al., IEEE RE'09). **Both are acted on, in different places**: the *type* stays permissive because the census requires it, and the *phrasing preference* moves to EARS because the external evidence is about checkability and the census says nothing about it. Neither source is overridden.
- [documented] The research pass looked for and did not find **any external norm that an acceptance criterion should name its own verifying instrument**, and none for phrasing an AC for a deliberately blocked item (evidence-augmentation §5). The `verification: Option<text>` field and the AC-###/`depends_on` convention are therefore **original process design, possibly ahead of external practice** — not confirmation of one. Nine agents finding nothing against a claim means no current source contradicts it, not that it is confirmed.
- [inferred] Clause 5's `doctor` rule threshold (`zero reads over N runs`) is not set by any measurement here. S-MIG establishes that a dead artifact existed and was invisible; it does not establish how many runs of silence should flag one.
- This record does **not** license: claiming layout C was benchmarked against a sidecar or embedded layout on any metric (0006's evidence gap is unchanged by this record); claiming the 98% spec-section figure applies to any individual section kind, since the three below-threshold kinds are inside that average; or treating `mutation: unknown` as evidence about the 142 rows carrying it.
- Partial supersession: **0005 and 0006 both stay `accepted`.** From 0005 this record withdraws only the implication that a structured field's shape is fixed independently of what the corpus contains — the prose-free/structure-gated rule, the CLI/web/gate-only write path and the locator-not-a-grammar rule all stand unchanged. From 0006 this record withdraws only `plan.md` from the human-facing artifact list, and refines the companion sidecars into one schema per kind — layout C itself, digest-stamped read-only structured sections, typed drift surfaced not applied, and "nothing parses structure out of prose" all stand unchanged. Anyone acting on 0005 or 0006 must read this Decision section for what has moved.
