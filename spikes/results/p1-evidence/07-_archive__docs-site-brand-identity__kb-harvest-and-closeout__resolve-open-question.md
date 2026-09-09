# 7/20 — _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question

**kind:** story · **commits:** 2 · **files from commits:** 1 · **paths named only in the card:** 0

## Files changed (outside .bklg/)

- `.kb/open-questions/brand-unwired-from-pipeline.md`

## Card body (the item as written)

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
c82c23b0 chore(kb-harvest-and-closeout): seal slice kb-decision-harvest approved

diff --git a/.kb/open-questions/brand-unwired-from-pipeline.md b/.kb/open-questions/brand-unwired-from-pipeline.md
index dc242b65..b23ae71a 100644
--- a/.kb/open-questions/brand-unwired-from-pipeline.md
+++ b/.kb/open-questions/brand-unwired-from-pipeline.md
@@ -1,24 +1,37 @@
 ---
 id: kb-open-question-brand-unwired-from-pipeline
-title: "Open: the brand identity is not the repo's token substrate, and the design stage will not find it"
+title: "Settled: the brand identity has a machine form, and the design stage now finds it"
 kind: open_question
-status: proposed
+status: withdrawn
 authority_tier: note
 summary: >-
-  Redkiln's visual identity lives in brand/ as SVGs and prose. Nothing in the pipeline reads it.
-  The design stage grounds itself by grepping for token/theme CSS, and that grep does not come
-  back empty — it finds docs/src/styles/theme.css, which imports Radix Themes with accentColor
-  "ruby" and grayColor "sand". A design stage today would therefore ground itself confidently in
-  a token system that is not the brand, and would be right to, because the brand has no machine
-  form. The open question is what token substrate brand/ should compile into, whether the docs
-  site should be re-tokenized onto it or the identity stays signature-only, and who owns
-  automating the export-time outlining pass the playbook currently leaves to human memory.
+  SETTLED, and withdrawn on 2026-08-11 because it was answered — not abandoned. Redkiln's visual
+  identity lived in brand/ as SVGs and prose that nothing in the pipeline could read, so a design
+  stage grounding itself by grepping for token/theme CSS found docs/src/styles/theme.css and its
+  Radix Themes import (accentColor "ruby", grayColor "sand") instead — confidently, and correctly,
+  because the brand had no machine form at all. Initiative RK-I0005 settled that fork. The identity
+  now has exactly one authored machine form (kb-decision-brand-authored-vs-derived); the docs
+  site's highlight is a separately named interactive concept with the global accent slot vacated
+  (kb-decision-brand-general-highlight, kb-decision-brand-radix-seam); and contrast over the
+  declared pairs is an ongoing per-cut property (kb-decision-per-cut-contrast-posture). Context and
+  Body below are preserved unchanged as the record of what was true on 2026-07-31; Resolution
+  states which branch was taken and why, and each of the four numbered sub-questions is answered or
+  explicitly scoped out.
 depends_on: []
 related:
   - kb-decision-brand-identity-constraints
   - kb-concept-brand-mark-rationale
   - kb-playbook-brand-asset-usage
   - kb-map-brand-identity
+  - kb-decision-brand-authored-vs-derived
+  - kb-decision-brand-general-highlight
+  - kb-decision-brand-radix-seam
+  - kb-decision-per-cut-contrast-posture
+resolution_ref:
+  - kb-decision-brand-authored-vs-derived
+  - kb-decision-brand-general-highlight
+  - kb-decision-brand-radix-seam
+  - kb-decision-per-cut-contrast-posture
 source_paths:
   - brand/README.md
   - docs/src/styles/theme.css
@@ -29,13 +42,48 @@ source_paths:
   - .redkiln/config.yaml
   - .kb/_intake/brand-unwired-from-pipeline.md
   - .kb/_intake/brand-asset-usage.md
-last_reviewed: 2026-07-31
+  - .bklg/docs-site-brand-identity/initiative.md
+  - .bklg/docs-site-brand-identity/kb-harvest-and-closeout/_closeout-ledger.md
+last_reviewed: 2026-08-11
 ---
 
-# Open: the brand identity is not the repo's token substrate, and the design stage will not find it
+# Settled: the brand identity has a machine form, and the design stage now finds it
+
+## Resolution
+
+**Branch taken: amended in place, and withdrawn because answered. Date: 2026-08-11.**
+
+This atom keeps its id, its path and its links. Its `status` moves `proposed → withdrawn`, and the
+sections below it are left standing as the historical record. Two grounded reasons for the amend
+branch rather than a supersession:
+
+1. **Nothing forced a successor.** `redkiln validate --kb` freezes only atoms that are `kind:
+   decision` **and** `status: accepted`. This atom is an `open_question`, so editing it in place is
+   legal, and `status` is a mutable field in any case.
+2. **A successor atom would have been a near-duplicate.** The answers already exist as the four
+   decision atoms harvested alongside this edit. Spawning a fifth atom to restate them is exactly
+   what the corpus's "prefer merge-and-link over new files" rule exists to prevent.
+
+`withdrawn` is the only terminal status available — `KbStatus` is a closed enum
+(`draft | proposed | accepted | superseded | withdrawn`) and `accepted` is not meaningful for a
+question. It is used here to mean **withdrawn because answered**, not abandoned. A machine-readable
+`resolution_ref` in the frontmatter mirrors the answering atoms; every id it names also appears in
+`related`, which is the field the validator actually resolves.
+
+**Where the answer lives**, for a reader who stops here:
+[[kb-decision-brand-authored-vs-derived]] (does the identity have a machine form, and which file is
+authored), [[kb-decision-brand-general-highlight]] (what happened to the docs site's highlight, and
+why the fired brick did not become it), [[kb-decision-brand-radix-seam]] (where the identity's roles
+and the component library's scale contract disagreed) and
+[[kb-decision-per-cut-contrast-posture]] (how the resulting colours are kept honest, per cut, on
+every run).
 
 ## Context
 
+_Preserved unchanged as the historical record. This section and § Body describe the tree **as it
+stood on 2026-07-31**; § Resolution above records what was decided since, and § Consequences / links
+below carries the disposition of each numbered sub-question._
+
 [[kb-decision-brand-identity-constraints]] and [[kb-concept-brand-mark-rationale]] record an
 identity that exists only as SVG files and prose in `brand/`. This atom records what that identity
 is *not* connected to, so that the next person to touch design in this repo starts from the real
@@ -108,21 +156,72 @@ today only by whoever remembers to do it by hand.
 
 ## Consequences / links
 
-The question this leaves open, in the order it probably has to be answered:
-
-1. **Does `brand/` get a machine form?** A token file (CSS custom properties or equivalent) that a
-   grep can find and a stage can cite — or does the identity stay signature-only, applied by hand
-   to headers and icons, with product surfaces free to use Radix?
-2. **If it gets one, what happens to the docs site?** Re-tokenizing Radix onto the brand palette
-   is a real piece of work, and the accent budget from
-   [[kb-decision-brand-identity-constraints]] does not survive contact with a general-purpose
-   accent scale without a deliberate decision about what the site's highlight colour is.
-3. **What tells the design stage which one is authoritative?** Today `DS_TOOLING`'s glob list is
-   the whole answer, and it will keep finding `docs/src/styles/theme.css` regardless of what
-   `brand/` contains.
-4. **Who owns the outlining pass?** A build step that emits brand raster output needs it built in,
-   per [[kb-playbook-brand-asset-usage]]; today it lives in a person's memory and fails silently
-   elsewhere.
-
-Until (1) is settled, treat a design stage's grounding citations in this repo as describing the
-docs site's substrate, not the brand. Indexed in [[kb-map-brand-identity]].
+The four questions this atom left open, in the order it said they would have to be answered — each
+with its original wording preserved, and the disposition RK-I0005 gave it.
+
+### 1. Does `brand/` get a machine form? — **answered** (RK-P0018 `brand-token-substrate`, DT-1)
+
+> A token file (CSS custom properties or equivalent) that a grep can find and a stage can cite — or
+> does the identity stay signature-only, applied by hand to headers and icons, with product surfaces
+> free to use Radix?
+
+**It gets one, and exactly one authored one.** `brand/redkiln.tokens.json` is a vendor-neutral token
+document, compiled by a single generator into `docs/src/styles/brand.tokens.css`, which `theme.css`
+imports immediately after the Radix import; every derived artifact names itself generated and names
+the file a reader may edit. A grounding grep now resolves to something derived from the identity.
+Read [[kb-decision-brand-authored-vs-derived]].
+
+### 2. If it gets one, what happens to the docs site? — **answered** (RK-P0020 `highlight-and-states`, DT-2 / DT-3 / DT-4)
+
+> Re-tokenizing Radix onto the brand palette is a real piece of work, and the accent budget from
+> [[kb-decision-brand-identity-constraints]] does not survive contact with a general-purpose accent
+> scale without a deliberate decision about what the site's highlight colour is.
+
+**The deliberate decision was taken, and it went the other way from "promote the brick".** The
+site's highlight is a *second, separately named* concept — a six-token `--brand-interactive-*`
+family — and the global accent slot is **vacated** rather than shared or redefined, so the fired
+brick stays bound to zero selectors and the two-instance accent budget is never spent. Read
+[[kb-decision-brand-general-highlight]], and [[kb-decision-brand-radix-seam]] for where the
+identity's roles and the component library's scale contract disagreed. How the resulting colours are
+kept honest per cut is [[kb-decision-per-cut-contrast-posture]].
+
+### 3. What tells the design stage which one is authoritative? — **explicitly scoped out**
+
+> Today `DS_TOOLING`'s glob list is the whole answer, and it will keep finding
+> `docs/src/styles/theme.css` regardless of what `brand/` contains.
+
+**Scoped out, deliberately, and this is a decision rather than an omission.** `DS_TOOLING` in
+`workflows/forge-plan-design.js` was **not** edited: RK-I0005 made the *existing* search succeed
+instead of teaching the search a new place to look. The glob list still resolves to
+`docs/src/styles/theme.css` — and that file now opens by naming the brand layer it imports and
+stating that Radix is the vendor scale set rather than this repository's identity, with the
+generated brand layer beside it saying which file is authored. Changing the glob list, or adding a
+`design.tokens` config key, remains out of scope and unowned; if a future initiative wants the
+substrate declared rather than discovered, that is where it starts.
+
+### 4. Who owns the outlining pass? — **answered** (RK-P0023 `social-card-and-outlining`, BR-11)
+
+> A build step that emits brand raster output needs it built in, per
+> [[kb-playbook-brand-asset-usage]]; today it lives in a person's memory and fails silently
+> elsewhere.
+
+**A build step owns it, not a person.** The preview-card generator under `docs/scripts/` outlines
+the lockup's live `<text>` and asserts the zero-`<text>` invariant *before* it rasterises, so the
+silent font substitution the playbook warns about fails the build instead of shipping — verified by
+regenerating on a host with no IBM Plex installed. The procedure itself remains
+[[kb-playbook-brand-asset-usage]]'s; what changed is that a machine now enforces it.
+
+### The standing instruction is retracted
+
+This atom used to close by telling every future reader to treat a design stage's grounding citations
+in this repository as describing the docs site's substrate rather than the brand. **That instruction
+is retracted as of 2026-08-11.** It was true while the fork was open and it is the confident wrong
+answer now that the fork is settled — which is precisely the failure mode this atom was written to
+prevent, turned around on itself.
+
+**What to do instead.** Ground on the substrate: `brand/README.md` is the ultimate source,
+`brand/redkiln.tokens.json` is the one authored machine form, and anything under
+`docs/src/styles/*.tokens.css` is derived and says so. Before choosing a colour for an interactive
+role, read [[kb-decision-brand-general-highlight]]; before adding a brand-derived colour to a
+surface, add its pair to the declared list per [[kb-decision-per-cut-contrast-posture]]. Indexed in
+[[kb-map-brand-identity]].
```
