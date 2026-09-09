# 16/20 — _archive/docs-site-brand-identity

**kind:** initiative · **commits:** 61 · **files from commits:** 476 · **paths named only in the card:** 14

## Files changed (outside .bklg/)

- `.gitattributes`
- `.github/workflows/ci.yml`
- `.gitignore`
- `.kb/decisions/brand-authored-vs-derived.md`
- `.kb/decisions/brand-general-highlight.md`
- `.kb/decisions/brand-radix-seam.md`
- `.kb/decisions/per-cut-contrast-posture.md`
- `.kb/design/brand-mark-in-site-chrome.md`
- `.kb/design/cut-as-capture-state.md`
- `.kb/design/cut-toggle-two-state.md`
- `.kb/design/share-card-single-cut.md`
- `.kb/design/state-indication-redundant-channels.md`
- `.kb/design/webfont-delivery-long-form-prose.md`
- `.kb/maps/brand-identity-knowledge-map.md`
- `.kb/open-questions/brand-unwired-from-pipeline.md`
- `.kb/playbooks/design-gate-amendment.md`
- `.kb/product/README.md`
- `.kb/product/docs-surface-archetypes.md`
- `.kb/product/journey-design-steward.md`
- `.kb/product/journey-docs-advocate.md`
- `.kb/product/journey-docs-evaluator.md`
- `.kb/product/journey-docs-practitioner.md`
- `.kb/product/persona-design-steward.md`
- `.kb/product/persona-docs-advocate.md`
- `.kb/product/persona-docs-evaluator.md`
- `.kb/product/persona-docs-practitioner.md`
- `.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl`
- `.redkiln/telemetry/events/ryanm@docs-site-brand-identity.jsonl`
- `README.md`
- `brand/README.md`
- `brand/redkiln.tokens.json`
- `dist/cli.js`
- `dist/emit.js`
- `dist/index.js`
- `docs/.gitignore`
- `docs/fonts/outlining/IBMPlexMono-Regular.ttf`  [git says BINARY]
- `docs/fonts/outlining/IBMPlexSans-Light.ttf`  [git says BINARY]
- `docs/fonts/outlining/OFL.txt`
- `docs/fonts/outlining/README.md`
- `docs/package.json`
- `docs/pnpm-lock.yaml`
- `docs/public/favicon.svg`
- `docs/public/fonts/OFL.txt`
- `docs/public/fonts/README.md`
- `docs/public/fonts/ibm-plex-mono-400.woff2`  [git says BINARY]
- `docs/public/fonts/ibm-plex-sans-var.woff2`  [git says BINARY]
- `docs/scripts/README.md`
- `docs/scripts/gen-og-card.d.mts`
- `docs/scripts/gen-og-card.mjs`
- `docs/scripts/lockup-outliner.d.mts`
- `docs/scripts/lockup-outliner.mjs`
- `docs/scripts/og-card-assertions.d.mts`
- `docs/scripts/og-card-assertions.mjs`
- `docs/scripts/og-card-composition.d.mts`
- `docs/scripts/og-card-composition.mjs`
- `docs/scripts/og-card-emit.d.mts`
- `docs/scripts/og-card-emit.mjs`
- `docs/src/components/BrandMark.astro`
- `docs/src/components/Callout.astro`
- `docs/src/components/Card.astro`

## Card body (the item as written)

```
## One-line intent

Make Redkiln's public face actually look like Redkiln — in both cuts, at every touchpoint a
stranger meets before reading a word — and do it by giving the identity a **machine form** that a
search for this repo's design substrate finds, so the next design decision inherits the brand
instead of confidently contradicting it.

## Vision and narrative

Redkiln has a finished visual identity. A brick arch framing a terminal prompt: the arch is the
kiln, the prompt and cursor are what the product actually is, and exactly one voussoir at the
bottom right is fired. It ships as five forms — lockup, mark-on-tile, bare mark, small icon — each
in a dark and a light cut, specified down to the 400-unit construction grid, the 4-unit mortar gap
and the 44-unit jamb brick that defines clear space
([`brand/README.md`](../../../brand/README.md)). It is not a mood board. Its rules are recorded as an
**Accepted** decision atom with `reversibility: low`
([`.kb/decisions/brand-visual-identity-constraints.md`](../../../.kb/decisions/brand-visual-identity-constraints.md)),
because several of them exist for the reason that breaking them destroys the thing the mark is
trying to say.

The public docs site knows none of it. It runs Radix Themes on `accentColor="ruby"` /
`grayColor="sand"`, declared twice
([`docs/src/pages/index.astro`](../../../docs/src/pages/index.astro),
[`docs/src/layouts/DocsLayout.astro`](../../../docs/src/layouts/DocsLayout.astro)). Where the mark
belongs, a CSS gradient square stands in — `.brand-mark` is
`linear-gradient(140deg, var(--accent-9), var(--accent-11))`
([`docs/src/styles/theme.css`](../../../docs/src/styles/theme.css)). The tab carries a hand-drawn red
flame in a hue unrelated to the brand's. The head emits `og:title`, `og:description` and `og:type`
and **no `og:image` at all**, so every shared link renders as a blank card. Not one `@font-face`
rule exists in the repository, where the identity specifies IBM Plex Sans 300 for the wordmark and
IBM Plex Mono 400 for the tagline. Half of a two-cut identity has, today, nowhere to land.

The part that makes this more than cosmetic drift is already written down as an open question
([`.kb/open-questions/brand-unwired-from-pipeline.md`](../../../.kb/open-questions/brand-unwired-from-pipeline.md)).
The design stage grounds itself by grepping for the repo's real design substrate — `*token*.css`,
`*theme*.css`, `tailwind*`, `*.tokens.*` — and **that search does not come back empty.** It finds
`docs/src/styles/theme.css`, a real, complete, verifiable token system, and its own hard rule
against naming an unverified primitive would correctly stop it from citing `brand/` instead,
because the brand has no machine form to cite. The failure mode is not a missing answer. It is a
**confident wrong answer**, and Radix ruby-on-sand is a warm red on a warm neutral — sympathetic to
the brand in spirit, which makes the divergence harder to notice, not easier. This is the industry's
most-reported design-system failure under its usual name, drift; only 8% of teams call their design
system "very stable" and even Polaris drifted 14% in a year despite strong adoption
([research 02](_discovery/research/02-design-token-substrate.md)).

So the organizing idea is not "restyle the docs site". A one-off hand-restyle produces exactly the
same repository the next morning: a correct-looking site and a substrate search that still returns
Radix. The organizing idea is **an identity that can be checked against** — where the values have
one authored home, every rendered appearance is derived from it rather than transcribed beside it,
and a person or a stage that goes looking for this repo's design substrate finds the brand. Teams
who tried maintaining two hand-authored copies of the same brand values report them drifting within
somewhere between an hour and a year, and the reported fix is always derivation, never more
discipline ([research 02](_discovery/research/02-design-token-substrate.md)).

Two constraints give this initiative its temperature, and neither is negotiable.

The first is the **accent budget**. `#A84322` appears exactly twice in the whole identity — the
fired voussoir and the wordmark period — and the decision atom states plainly that a surface built
on this palette does not promote it to a general-purpose highlight colour. A docs site needs a
highlight colour for links, focus rings, active nav and callout severity. Today `theme.css` has one
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
6f1f0342 Merge pull request #106 from Wet-Ink-Corporation/initiative/docs-site-brand-identity

94fadbc5 Merge branch 'main' into initiative/docs-site-brand-identity

diff --cc .redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl
index eb120282,f1d9d7f9..93f6745b
--- a/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl
+++ b/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl
@@@ -1,2 -1,3 +1,5 @@@
 +{"ts":"2026-08-13T18:32:08.841Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"item_created","item":"RK-P0039","type":"project"}
 +{"ts":"2026-08-13T19:19:08.812Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"session_end"}
+ {"ts":"2026-08-10T02:07:24.089Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0022","type":"project","initiative":"docs-site-brand-identity","project":"typography-delivery","process":"project","stage":"implementation","tokens_out":171403,"workflow":"implement","result":"halted:stories","agents":5,"degraded":0,"tier":"standard","sha":"02d35559","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":14499},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":156904}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+ {"ts":"2026-08-10T04:25:39.795Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0022","type":"project","initiative":"docs-site-brand-identity","project":"typography-delivery","process":"project","stage":"implementation","tokens_out":152618,"workflow":"implement","result":"halted:stories","agents":5,"degraded":0,"tier":"standard","sha":"8f435822","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":11429},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":141189}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+ {"ts":"2026-08-10T06:15:44.484Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0022","type":"project","initiative":"docs-site-brand-identity","project":"typography-delivery","process":"project","stage":"implementation","tokens_out":167968,"workflow":"implement","result":"halted:stories","agents":5,"degraded":0,"tier":"standard","sha":"299f9628","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":16684},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":151284}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
diff --cc scripts/capture-cdp.mjs
index 46d5fcd8,992de67f..7b610361
--- a/scripts/capture-cdp.mjs
+++ b/scripts/capture-cdp.mjs
@@@ -103,9 -131,9 +131,15 @@@ export const REST_FEATURES = Object.val
   * Overlay `overrides` onto `base` BY FEATURE NAME, last writer wins.
   *
   * `Emulation.setEmulatedMedia` takes the complete override list every time, so a
-  * pass has to be expressed as the state's own features with its one feature
-  * replaced rather than appended: sending two entries for the same feature name
-  * leaves which one Chrome honours to the order of a list.
+  * pass has to be expressed as the rest condition with its own feature replaced
+  * rather than appended: sending `forced-colors: none` and `forced-colors: active`
+  * together leaves which one Chrome honours to the order of a list.
++ *
++ * Three layers stack through it, outermost last: the pinned REST condition, then
++ * the STATE's own features (a served surface's colour cut — see `setBaseFeatures`),
++ * then the PASS. Each is allowed to override the one beneath it by name and
++ * nothing else, which is what keeps a pass from silently dropping a cut and a cut
++ * from silently dropping the rest pin.
   */
  function mergeFeatures(base, overrides) {
    const by = new Map(base.map((f) => [f.name, f]));
@@@ -484,13 -464,13 +518,16 @@@ export class CdpPage 
        );
      }
      if ("features" in pass) {
-       // MERGED OVER THE STATE'S OWN FEATURES, never sent alone. `setEmulatedMedia`
-       // REPLACES the whole override list, so a pass sent by itself would silently
-       // drop the cut a served surface set with `setBaseFeatures` — and the frame
-       // would carry a filename its pixels disagree with, which is the one thing
 -      // MERGED OVER THE PINNED REST, never sent alone. `setEmulatedMedia` REPLACES
 -      // the whole override list, so sending only this pass's features would drop
 -      // every other feature back to the machine's own preference — re-opening, for
 -      // the duration of the pass, exactly the machine-dependence REST_FEATURES
 -      // exists to close.
++      // MERGED OVER THE REST PIN AND THE STATE'S OWN FEATURES, never sent alone.
++      // `setEmulatedMedia` REPLACES the whole override list, so a pass sent by
++      // itself would do two different kinds of damage at once: every other feature
++      // falls back to the machine's own preference — re-opening, for the duration
++      // of the pass, exactly the machine-dependence REST_FEATURES exists to close
++      // — and a served surface's colour cut (`setBaseFeatures`) is dropped, so the
++      // frame carries a filename its pixels disagree with. Both are the failure
 +      // this bench's self-verification family exists to prevent.
        await this.send("Emulation.setEmulatedMedia", {
-         features: mergeFeatures(this.baseFeatures, pass.features),
 -        features: mergeFeatures(REST_FEATURES, pass.features),
++        features: mergeFeatures(this.restCondition(), pass.features),
        });
        if (!(await this.matches(pass.query))) {
          throw new CdpError(
@@@ -510,13 -490,14 +547,25 @@@
    }
  
    /**
-    * Return the page to its BASE condition between passes.
 -   * Return the page to the REST condition between passes.
++   * This page's rest condition: the pinned {@link REST_FEATURES}, plus whatever
++   * the STATE itself declares.
++   *
++   * Rest is pinned rather than inherited — see {@link REST_FEATURES} for why the
++   * negative half of a comparison cannot be left to the operator's OS — and a
++   * state's own features (`setBaseFeatures`) sit on top of it, because a colour
++   * cut is not a pass and must survive one.
++   */
++  restCondition() {
++    return mergeFeatures(REST_FEATURES, this.baseFeatures);
++  }
++
++  /**
++   * Return the page to its rest condition between passes.
     *
-    * Base, not bare: the state's own features (see `setBaseFeatures`) are not a
-    * pass and must survive one.
 -   * Rest is the pinned condition of {@link REST_FEATURES}, not the machine's — see
 -   * there for why the negative half of a comparison cannot be left to the operator's
 -   * OS. Vision deficiency has no rest value to pin: `none` IS the absence.
++   * Vision deficiency has no rest value to pin: `none` IS the absence.
     */
    async clearPass() {
-     await this.send("Emulation.setEmulatedMedia", { features: this.baseFeatures });
 -    await this.send("Emulation.setEmulatedMedia", { features: REST_FEATURES });
++    await this.send("Emulation.setEmulatedMedia", { features: this.restCondition() });
      await this.send("Emulation.setEmulatedVisionDeficiency", { type: "none" });
    }
  
6f1152a9 Merge branch 'main' into initiative/docs-site-brand-identity

diff --cc .github/workflows/ci.yml
index 7d22bb27,b025d5e0..7bc81bdf
--- a/.github/workflows/ci.yml
+++ b/.github/workflows/ci.yml
@@@ -31,15 -31,38 +31,47 @@@ jobs
          if: runner.os == 'Windows'
          run: git config --global core.autocrlf false
  
-       # FULL HISTORY, because part of the gate is a DIFF. `test/typography-delivery.test.ts`
-       # asserts several of its claims against a pinned base commit (`git show <base>:<path>`)
-       # — "this story added no focusable control", "this story changed no font-size",
-       # "no responsive type step was introduced". A claim about a diff cannot be checked
-       # from a scan of the current tree, and `actions/checkout@v4` defaults to
-       # `fetch-depth: 1`, so the base blob was absent on every runner and those
-       # assertions degraded to nothing. They are hard failures now (the same warn-and-
-       # return class removed from `requireBuilt` below), which only works if the history
-       # they read is here.
++      # A SECOND, INDEPENDENT REASON FOR `fetch-depth: 0`, recorded so removing
++      # either one does not quietly remove the setting. Part of the gate is a DIFF:
++      # `test/typography-delivery.test.ts` asserts several claims against a pinned
++      # base commit (`git show <base>:<path>`) — "this story added no focusable
++      # control", "this story changed no font-size", "no responsive type step was
++      # introduced". A claim about a diff cannot be checked from a scan of the
++      # current tree, and under the `fetch-depth: 1` default the base blob was
++      # absent on every runner, so those assertions degraded to nothing. They are
++      # hard failures now, which only works if the history they read is here.
+       # THIS SUITE AUDITS THIS REPOSITORY'S OWN HISTORY, so the default checkout
+       # cannot run it. `actions/checkout` defaults to `fetch-depth: 1` and, on a
+       # `pull_request`, to a DETACHED checkout of the merge commit. Both defaults
+       # are wrong for a repo that commits readings and re-derives them from git:
+       #
+       #   fetch-depth: 0 — `.bklg/**` readings pin real shas (a branch merge-base,
+       #   an `audit.head`) and the harness re-derives them with `git diff
+       #   base..HEAD`, `git log base..head` and `cat-file -e`. Under a depth-1
+       #   clone none of those objects exist, so the suite reported `Invalid
+       #   revision range` and `Not a valid object name` across five test files —
+       #   CI announcing it had no history, never that a reading was wrong.
+       #
+       #   ref: the BRANCH, not the merge commit — a detached HEAD makes `git
+       #   rev-parse --abbrev-ref HEAD` answer the literal string "HEAD", which reds
+       #   AC-PROV-TREE-SCOPE ("the reading was taken on a different branch"), and
+       #   it leaves the clone carrying NO local branch at all, which is fatal to
+       #   the repository mirror in `scripts/capture-hermetic.mjs` ("carries no
+       #   branches, so it cannot photograph a cross-branch board"). `github.ref_name`
+       #   is the fallback for the `push: [main]` trigger, where `head_ref` is empty.
+       #
+       # NOT FORK-READY, stated so the next failure is legible rather than baffling:
+       # `head_ref` names a branch in the HEAD repository, and this step checks out
+       # the BASE one, so a pull request opened from a fork would fail to resolve the
+       # ref. No fork PR has ever been opened here (every branch is an initiative
+       # worktree in this repo). Supporting one means adding `repository:
+       # ${{ github.event.pull_request.head.repo.full_name }}` alongside the ref.
+       #
+       # THE TRADE, STATED RATHER THAN BURIED: CI now tests the branch head instead
+       # of the PR's merge-with-main preview, so a branch behind `main` is not
+       # tested against main's tip until it lands. That is the price of a suite
+       # whose subject is this repository's own recorded history — and `main`'s own
+       # push build still tests the merged result before anything is released.
        - uses: actions/checkout@v4
          with:
            fetch-depth: 0
diff --cc .kb/product/README.md
index 5099493c,ab7ca8af..21eb1469
--- a/.kb/product/README.md
+++ b/.kb/product/README.md
@@@ -23,67 -23,6 +23,66 @@@ of inventing a fresh set from the same 
  Ground every claim in real discovery evidence and cite it in `source_paths`. A persona nobody
  researched is a stock photo with a name, and acceptance criteria framed from it are fiction.
  
 +## Conventions in this directory
 +
 +Everything you need in order not to invent a competing shape. Read this before adding a file.
 +
 +- **Tier.** Every atom here carries `authority_tier: product` — the tier this layer is named for
 +  in [`../README.md`](../README.md) (_Suggested layout_). The field is a free string in the
 +  schema, so a typo like `prodcut` validates cleanly and quietly splits the layer in two. Nothing
 +  catches that but your eye.
 +- **Kind.** Persona → `kind: concept`. Journey → `kind: playbook`. Both are members of the closed
 +  `KbKind` enum (`src/schema/kb.ts`); there is no `persona` kind and none can be added.
 +- **Id format.** `kb-{kind}-{slug}`, matching the rest of the corpus
 +  (`kb-concept-brand-mark-rationale`, `kb-map-brand-identity`). Scope-qualify the slug wherever
 +  the subject is surface-specific — `kb-concept-persona-docs-evaluator`, not
 +  `kb-concept-persona-evaluator` — because ids are unique corpus-wide and a second initiative's
 +  evaluator must be able to be an addition rather than a collision.
 +- **Layout is flat.** This directory is a **tier** partition whose contents are mixed-kind by
 +  definition, not a second tree organized by kind. Do not create `product/concepts/` or
 +  `product/playbooks/`: they would shadow the corpus's top-level `concepts/` and `playbooks/`
 +  directories and turn "where does a persona live" into a two-step question. Kind is carried by
 +  the frontmatter and by the filename prefix (`persona-*`, `journey-*`).
 +- **One persona, one id.** Before authoring, search the **whole** corpus — not just this
 +  directory — for an atom already describing the same reader, and extend it rather than adding a
 +  near-duplicate. `validate --kb` errors on two atoms sharing an `id`; it has no opinion
 +  whatsoever about the same evaluator existing under two different ids, and that is the worse
 +  defect. A persona under two ids is worse than a persona under none: both are then citable, they
 +  drift apart, and nothing ever reports it.
 +- **Link by id, never by path.** `related` / `depends_on` take atom ids, which are link-checked.
 +  `source_paths` takes paths and is **not** checked — it is provenance, not content. Assume every
 +  path it names will one day be archived, and write the atom so it still answers a reader then.
 +
 +### Coordination: has anyone else populated this layer? (checked 2026-08-11)
 +
 +This layer is shared across initiatives, so the state of the directory is **rechecked at
 +authoring time rather than assumed**. Two observations, on this branch and on the mainline:
 +
 +```
 +$ git ls-tree -r --name-only HEAD -- .kb/product
 +.kb/product/README.md
 +
 +$ git ls-tree -r --name-only main -- .kb/product
 +.kb/product/README.md
 +```
 +
 +**Finding: no atoms existed on either ref — only this README, which arrived with the `.kb`
 +skeleton and carries no persona, journey or archetype content.** In particular the concurrent
 +`issue-backlog-sweep-lane` initiative (RK-I0004), which shares the obligation to populate this
 +layer, had not landed anything here: its backlog tree lives on its own initiative branch and no
 +`.kb/product/` atom of its appears on `main`. So the conventions above were **established** here
 +rather than adopted from an existing shape, and the README's own pre-existing tier/kind table was
 +kept verbatim rather than restated. Whichever initiative populates this layer next **extends**
 +these conventions rather than competing with them; if you find a shape here that contradicts this
 +section, the shape on disk wins and this section is what needs correcting.
 +
 +### Reverting
 +
 +Atoms here are additive: adding one changes nothing that already existed, and removing one is
 +safe **only while nothing references its id**. The persona and journey atoms name
 +`kb-concept-docs-surface-archetypes` in `related`, so that archetype atom and the persona/journey
 +set **revert together** — reverting the archetype atom alone leaves every persona pointing at an
 +id that no longer resolves and `validate --kb` fails on all of them. Revert the set, not one file.
- 
  ## What does not belong here
  
  An **unevidenced sketch**. A persona assembled from assumptions during one initiative's discovery
diff --cc .redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl
index 00000000,00000000..eb120282
new file mode 100644
--- /dev/null
+++ b/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl
@@@ -1,0 -1,0 +1,2 @@@
++{"ts":"2026-08-13T18:32:08.841Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"item_created","item":"RK-P0039","type":"project"}
++{"ts":"2026-08-13T19:19:08.812Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"session_end"}
diff --cc .redkiln/telemetry/events/ryanm@docs-site-brand-identity.jsonl
index 00000000,00000000..33a4081b
new file mode 100644
--- /dev/null
+++ b/.redkiln/telemetry/events/ryanm@docs-site-brand-identity.jsonl
@@@ -1,0 -1,0 +1,23 @@@
++{"ts":"2026-08-02T21:47:43.067Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_created","item":"RK-S0173","type":"story"}
++{"ts":"2026-08-02T21:53:00.578Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_created","item":"RK-S0177","type":"story"}
++{"ts":"2026-08-03T22:05:23.141Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"gate_result","item":"RK-P0018","type":"project","initiative":"docs-site-brand-identity","project":"brand-token-substrate","process":"project","stage":"integration","outcome":"halted:gate","gate":"command","pass":false,"note":"command gate failed (EC-003): `{{redkiln}} verify --item {{id}} --grain integration` — integration-scoped (`pnpm typecheck && pnpm lint && pnpm test` exited non-zero on two consecutive runs)"}
++{"ts":"2026-08-05T02:21:58.855Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"gate_result","item":"RK-S0212","type":"story","initiative":"docs-site-brand-identity","project":"highlight-and-states","process":"story","stage":"implement","outcome":"halted:gate","gate":"command","pass":false,"note":"command gate failed (EC-003): `{{redkiln}} verify --item {{id}} --grain story` — affected-gate (`pnpm typecheck && pnpm lint && pnpm test` exited non-zero on two consecutive runs)"}
++{"ts":"2026-08-05T03:44:46.547Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:45:06.014Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:45:24.259Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:45:38.616Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:45:53.017Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:46:08.472Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:46:30.449Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:46:56.893Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:47:14.529Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:47:34.010Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:47:50.164Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:48:00.004Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:48:24.441Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-05T03:48:34.361Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-P0020"}
++{"ts":"2026-08-06T21:12:03.307Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-S0221"}
++{"ts":"2026-08-06T21:12:21.538Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-S0221"}
++{"ts":"2026-08-06T21:12:35.544Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_edited","item":"RK-S0221"}
++{"ts":"2026-08-11T21:55:14.756Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"gate_result","item":"RK-S0194","type":"story","initiative":"docs-site-brand-identity","project":"social-card-and-outlining","process":"story","stage":"implement","outcome":"halted:gate","gate":"command","pass":false,"note":"command gate failed (EC-003): `{{redkiln}} verify --item {{id}} --grain story` — affected-gate (`pnpm typecheck && pnpm lint && pnpm test` exited non-zero on two consecutive runs)"}
++{"ts":"2026-08-13T18:32:50.999Z","actor":"ryanm","worktree":"docs-site-brand-identity","branch":"","event":"item_created","item":"RK-B0016","type":"bug"}
diff --cc package.json
index 0e65e96b,0cdc4916..e813b291
--- a/package.json
+++ b/package.json
@@@ -47,8 -47,8 +47,9 @@@
    "scripts": {
      "dev": "node --experimental-strip-types bin/redkiln.mjs",
      "build": "node scripts/build.mjs",
 +    "build:brand-tokens": "node scripts/build-brand-tokens.mjs",
      "test": "vitest run",
+     "test:dod": "vitest run --config vitest.dod.config.ts",
      "typecheck": "tsc --noEmit",
      "lint": "eslint . && prettier --check .",
      "format": "prettier --write .",
diff --cc scripts/capture-cdp.mjs
index 00000000,8e8290da..46d5fcd8
mode 000000,100644..100644
--- a/scripts/capture-cdp.mjs
+++ b/scripts/capture-cdp.mjs
@@@ -1,0 -1,711 +1,785 @@@
+ // The capture bench's EYES: a thin Chrome DevTools Protocol client (RK-S0098).
+ //
+ // WHY THIS EXISTS. Until this module, `scripts/capture.mjs` launched Chrome once
+ // per image in `--screenshot` one-shot mode against a `file://` URL and exited. A
+ // one-shot browser cannot be talked to, which meant the ONE instrument in this
+ // pipeline that perceives anything could not see the two media conditions the
+ // accessibility acceptance criteria are stated in (`forced-colors: active`,
+ // `prefers-reduced-motion: reduce`), could not press a key, and could not read a
+ // value back out of the page it had just photographed. A review that cannot see
+ // the condition is not a lesser check — it is a check that passes having never
+ // looked.
+ //
+ // NO NEW DEPENDENCY, and that is the same trade `scripts/capture.mjs` made and
+ // wrote down: adding Playwright — a browser download bigger than the entire
+ // repository — to take a screenshot would be a poor one. `package.json` declares
+ // `"node": ">=22.5"`, so the `WebSocket` global is here without a flag, a shim or
+ // a devDependency. CDP is a JSON request/response protocol over one socket; the
+ // whole client is below.
+ //
+ // IT IS A SIDE-EFFECT-FREE MODULE ON PURPOSE. `scripts/capture.mjs` calls `main()`
+ // at module scope, so a test that imported it would run a whole capture. Every
+ // capability this story lands therefore lives here, where `test/capture-bench.ts`
+ // can execute the real one instead of asserting about its source — the same shape
+ // `scripts/capture-viewport.mjs` already takes for the cropper.
+ import { spawn } from "node:child_process";
+ import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
+ import { tmpdir } from "node:os";
+ import { join } from "node:path";
+ 
+ /** Anything this client cannot do is thrown as one of these. */
+ export class CdpError extends Error {}
+ 
+ /** Every CDP command's ceiling, of the same order as the bench's old 60 s. */
+ const COMMAND_TIMEOUT_MS = 60_000;
+ 
+ /** How long the browser has to write its `DevToolsActivePort` file. */
+ const ENDPOINT_TIMEOUT_MS = 30_000;
+ 
+ /**
+  * The bounded settle that REPLACES `--virtual-time-budget=2000`.
+  *
+  * THIS IS THE ONE THING IN THIS FILE A FUTURE READER MUST NOT SIMPLIFY AWAY.
+  * `--virtual-time-budget` was not decoration: the board ships an inline hydration
+  * script (`src/viz/html.ts`) that needs a turn of the event loop before the shot,
+  * and the flag's one-shot mode is what honoured it. The flag has no meaning in a
+  * CDP session, so the settle is explicit here — load event, then `readyState`
+  * complete, then two animation frames, then this grace. Delete it and the bench
+  * starts producing blank or half-painted frames that sail past the 1024-byte
+  * tripwire and are then reviewed AS IF THEY WERE THE SURFACE, which is worse than
+  * the blindness this module exists to end, because it looks like evidence.
+  */
+ const SETTLE_MS = 400;
+ 
+ /**
+  * The passes a state may declare, and what each one actually does.
+  *
+  * A PASS IS A VARIANT OF A STATE, NOT A NEW DOCUMENT (`_design.md`: "`-forced-colors`
+  * and `-reduced-motion` are *media-feature* variants of an existing state"). One
+  * base producer plus one declaration yields
+  * `<surface>@<state>-<key>@<viewport>@<theme>.png` through the bench's unchanged
+  * filename shape, which is exactly how the signed-off design already names its two
+  * declared variant frames.
+  *
+  * `query` IS THE SELF-VERIFICATION, and it is not optional. The failure this
+  * instrument must not be able to commit is emitting a PNG *named* as an emulated
+  * capture that is in fact the ordinary render. So a media pass proves itself
+  * INSIDE the page it is about to photograph — and the plain pass proves the same
+  * queries FALSE, without which "the reduced-motion capture is indistinguishable
+  * from the default capture" compares two identical conditions and can never fail.
+  *
+  * `greyscale` is a DIFFERENT MECHANISM from `forced-colors` and the conflation is
+  * worth naming: forced colours is a media feature, greyscale is a rendering
+  * transform. Chrome emulates it natively as a vision deficiency, so no PNG decoder
+  * has to be hand-rolled and — critically — no capture-only filter reaches product
+  * output.
+  *
+  * `focus` is the interaction pass: it presses Tab and photographs where focus
+  * went. It is what makes "the control is keyboard-reachable with a visible focus
+  * ring" a picture rather than a promise.
+  */
+ export const MEDIA_PASSES = {
+   "forced-colors": {
+     query: "(forced-colors: active)",
+     features: [{ name: "forced-colors", value: "active" }],
+   },
+   "reduced-motion": {
+     query: "(prefers-reduced-motion: reduce)",
+     features: [{ name: "prefers-reduced-motion", value: "reduce" }],
+   },
+   greyscale: { vision: "achromatopsia" },
+   focus: { interact: "tab" },
+ };
+ 
+ /**
+  * The media queries the PLAIN pass must evaluate `false` inside every page it
+  * photographs — the negative half of the self-verification.
+  */
+ export const REST_QUERIES = Object.values(MEDIA_PASSES)
+   .map((p) => ("query" in p ? p.query : ""))
+   .filter((q) => q !== "");
+ 
++/**
++ * Overlay `overrides` onto `base` BY FEATURE NAME, last writer wins.
++ *
++ * `Emulation.setEmulatedMedia` takes the complete override list every time, so a
++ * pass has to be expressed as the state's own features with its one feature
++ * replaced rather than appended: sending two entries for the same feature name
++ * leaves which one Chrome honours to the order of a list.
... TRUNCATED at 400 lines; 61 commits total
```
