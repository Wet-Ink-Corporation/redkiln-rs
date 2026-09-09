# 9/20 — _archive/docs-site-brand-identity/typography-delivery

**kind:** project · **commits:** 57 · **files from commits:** 21 · **paths named only in the card:** 23

## Files changed (outside .bklg/)

- `.changes/114-pagefind-comment-names-the-mechanism.md`
- `docs/src/styles/theme.css`
- `.redkiln/config.yaml`
- `scripts/reap-serve-children.mjs`
- `test/docs-dist-precondition.test.ts`
- `test/serve-child-reaping.test.ts`
- `test/typography-delivery.test.ts`
- `test/marks-guard.test.ts`
- `docs/scripts/gen-og-card.mjs`
- `docs/src/components/Callout.astro`
- `docs/src/components/ThemeToggle.tsx`
- `scripts/observe-cold-load.mjs`
- `src/schema/brand-tokens.ts`
- `test/brand-tokens.test.ts`
- `test/capture-surfaces.test.ts`
- `test/contrast-verification.test.ts`
- `test/docs-brand.test.ts`
- `test/helpers/contrast-interactive.ts`
- `test/og-card-cut-declaration.test.ts`
- `test/og-card-tree.test.ts`
- `test/helpers/tab-stops.ts`
- `docs/package.json`  [named in the card; no commit found]
- `docs/astro.config.mjs`  [named in the card; no commit found]
- `brand/README.md`  [named in the card; no commit found]
- `brand/showcase.html`  [named in the card; no commit found]
- `docs/src/layouts/DocsLayout.astro`  [named in the card; no commit found]
- `docs/src/pages/index.astro`  [named in the card; no commit found]
- `references/audit/findings/12-docs-and-adoption.md`  [named in the card; no commit found]
- `docs/src/lib/url.ts`  [named in the card; no commit found]
- `.github/workflows/docs.yml`  [named in the card; no commit found]
- `plex-per-weight-fallback-metrics/bug.md`  [named in the card; no commit found]
- `cold-load-delivery-behaviour/evidence/observation.json`  [named in the card; no commit found]
- `design/_build.mjs`  [named in the card; no commit found]
- `design/mock.html`  [named in the card; no commit found]
- `src/store/actor.ts`  [named in the card; no commit found]
- `src/store/verify.ts`  [named in the card; no commit found]
- `.github/workflows/ci.yml`  [named in the card; no commit found]
- `built-and-deployed-font-proof/spec.md`  [named in the card; no commit found]
- `brand/redkiln.tokens.json`  [named in the card; no commit found]
- `evidence/pair-review.md`  [named in the card; no commit found]
- `evidence/README.md`  [named in the card; no commit found]
- `scripts/capture.mjs`  [named in the card; no commit found]
- `src/viz/html.ts`  [named in the card; no commit found]
- `workflows/forge-plan-design.js`  [named in the card; no commit found]

## Card body (the item as written)

```
## One-line objective

Deliver the identity's faces — IBM Plex Sans in the prose, IBM Plex Mono in the code — to a real
reader of the docs site, self-hosted and consumed from the brand token substrate, on a cold
long-form read that produces no visible flash of substituted letterforms, no measurable jump in the
prose being read, and no request to a third-party origin.

## How this advances the initiative

This project owns **BR-12** and **AC-8** and scope letter **E**, and resolves **DT-5**
([`../_decomposition.md`](../_decomposition.md), traceability and design-tension tables). It is the
initiative's only wholly **net-new-risk** project: the docs tree loads no webfont today — there is no
font dependency in [`docs/package.json`](../../../../docs/package.json), no font integration in
[`docs/astro.config.mjs`](../../../../docs/astro.config.mjs), no `@font-face` in
[`docs/src/styles/theme.css`](../../../../docs/src/styles/theme.css), and the KB records that "the docs
site sets no font family from the identity"
([`.kb/open-questions/brand-unwired-from-pipeline.md`](../../../../.kb/open-questions/brand-unwired-from-pipeline.md)).
Everything the site renders today comes from the family tokens Radix Themes' own stylesheet supplies;
the one place the site names a family at all is
`docs/src/styles/theme.css:211`, `font-family: var(--code-font-family, ui-monospace, monospace)`.
So every flash, layout jump and third-party exposure this project could cause would be **introduced
here**, not inherited — which raises the bar on delivery rather than lowering it
([research 03](../_discovery/research/03-astro-font-pipeline.md)).

The identity is not decorative about this. IBM Plex was chosen because it "was drawn for an
industrial engineering brand", its "squared terminals rhyme with the brickwork", and the mono sibling
"ties the tagline to the CLI itself **and to the code blocks in the docs, which is the second place
the identity has to survive**"
([`.kb/concepts/brand-mark-rationale.md`](../../../../.kb/concepts/brand-mark-rationale.md)). Delivering
the faces is delivering that signal; failing to deliver them reliably erodes the same signal the
initiative exists to restore.

Downstream, this project is a hard dependency of **RK-P0023 `social-card-and-outlining`**: DoD 13
requires regenerating the lockup raster on a machine with *no* IBM Plex installed and no webfont
available, which is only possible against a repo-vendored file. It is the primary owner of **DoD 11**
(the cold-load type journey) and a named participant in **DoD 3** (the prose face at first paint),
**DoD 13** (the vendored font) and **DoD 15** (font assets in the real build).

## In scope (this project)

- **Consuming type values from the substrate, not re-authoring them.** Family, weight and size were
  decided at the Stage A gate to have a machine form
  ([`../_decomposition.md`](../_decomposition.md), *Decisions taken at this gate* #2), so this
  project reads RK-P0018's type tokens rather than transcribing `brand/README.md`'s typography table
  — a second hand-maintained copy is exactly what BR-2 forbids.
- **Deciding which families, weights and character coverage actually ship**, and justifying each
  against cold-load weight. `brand/README.md` specifies IBM Plex Sans **300** for the wordmark and
  IBM Plex Mono **400** for the tagline; the identity's own reference render
  ([`brand/showcase.html:9`](../../../../brand/showcase.html)) additionally pulls Sans 300/400/500 and
  Mono 400/500. What a 36-page prose surface needs is this project's call, not an inherited default.
- **Self-hosting: vendoring the font binaries into the repo** under `docs/`, with their SIL OFL
  licence text alongside, and wiring `@font-face` so the built site serves every byte from its own
  origin.
- **Subsetting strategy** — what coverage the docs corpus needs, and whether subsetting is a build
  step or a committed artifact.
- **Resolving DT-5 explicitly** — how the faces are delivered on a first cold read and what a reader
  sees while they arrive — with the rejected options and their named failure modes recorded in this
  project's `_design.md`. Not defaulted to `swap` by logotype convention.
- **The fallback stack**, including whether a metric-matched fallback is worth authoring for the
  delivery mode chosen, or whether the system stack's metrics are close enough that the shift is
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
4954f1ef docs(theme): the Pagefind search-link comment names the real mechanism

diff --git a/.changes/114-pagefind-comment-names-the-mechanism.md b/.changes/114-pagefind-comment-names-the-mechanism.md
new file mode 100644
index 00000000..16269fc2
--- /dev/null
+++ b/.changes/114-pagefind-comment-names-the-mechanism.md
@@ -0,0 +1,36 @@
+---
+kind: fixed
+---
+
+- **The Pagefind search-link comment now names the real mechanism** (issue 114). The note in
+  `docs/src/styles/theme.css` said a Pagefind result link _inherits_ the search panel's ink,
+  and that this repo's stylesheets "do not reach inside that bundle". Both halves were wrong,
+  and the second is contradicted by the same file's own fourth-cascade block a few hundred
+  lines below, which does reach inside it. Issue 114 then read the same symptom a third way
+  — the vendor's class winning on **specificity** — which is also not the mechanism.
+
+  Read out of the built bundle rather than remembered, as that block requires: every
+  result-link rule resolves its colour through `--pagefind-ui-text`, and the unreadable
+  dark-cut value is that property's own **vendor default, declared on `:root`**. Nothing is
+  inherited from the panel and no selector fight is involved. The remedy is a custom
+  property, not a rule: `.search-panel` rebinds `--pagefind-ui-text` to the brand ink, where
+  a declaration on a closer ancestor beats one on `:root` for that subtree whatever the
+  specificity or source order.
+
+  **The contrast defect itself was already fixed**, in `f4b93152` (2026-08-10) — three days
+  before issue 114 was filed, which is why it could still be observed on a build predating
+  it. `test/typography-delivery.test.ts` asserts all four `--pagefind-ui-*` bindings in the
+  EMITTED bundle, and a fresh `docs/dist` build passes it at 84, with the contrast harness
+  green at 198. So no colour changes here and none is needed; what was owed was an accurate
+  comment.
+
+  The issue reference is written without the `#` sigil, matching the convention this file
+  already documents: `test/docs-brand.test.ts` scans `docs/src/**` as text for
+  `#[0-9a-fA-F]{3,8}`, and an issue number reads as a hex literal to it.
+
+## Migration
+
+**Packs and templates: none.** The change is a comment in `docs/src/styles/theme.css`; no
+file under `processes/` or `templates/` changed, no shipped source changed, and no rendered
+byte of the docs site changes — so `redkiln adopt` has nothing to take and every in-flight
+item stays on the pack it is pinned to.
diff --git a/docs/src/styles/theme.css b/docs/src/styles/theme.css
index 9f7b03fb..1bd9575f 100644
--- a/docs/src/styles/theme.css
+++ b/docs/src/styles/theme.css
@@ -249,9 +249,24 @@ html {
    which is the current failure shipped again (anti-pattern 3).
 
    Known consequence, in fence and reported upward rather than guessed at: a
-   Pagefind result link inherits the search panel's ink and carries no underline.
-   Search.tsx loads pagefind-ui.css at runtime and this repo's stylesheets do not
-   reach inside that bundle (_design.md, disposition table's final row). */
+   Pagefind result link carries no underline.
+
+   THE COLOUR HALF OF THIS NOTE WAS WRONG TWICE, AND THE CORRECTION IS THE POINT
+   (issue 114). It said the result link INHERITS the panel's ink and that this repo's
+   stylesheets "do not reach inside that bundle"; issue 114 then read the same symptom
+   as the vendor's class winning on SPECIFICITY. Neither is the mechanism, and
+   both send the next reader somewhere there is nothing to find.
+
+   Read out of the built bundle rather than remembered, exactly as the fourth
+   cascade below requires: every result-link rule is `color: var(--pagefind-ui-text)`,
+   and the value that made the link unreadable in the dark cut is that property's
+   own VENDOR DEFAULT, declared on `:root`. Nothing is inherited from the panel and
+   no class fight is involved. The remedy is therefore a custom property, not a
+   selector: the block below rebinds `--pagefind-ui-text` on `.search-panel`, where
+   a declaration on a closer ancestor beats one on `:root` for that subtree whatever
+   the specificity or source order — so this repo's stylesheets DO reach inside the
+   bundle, through the one documented seam the vendor offers.
+   `test/typography-delivery.test.ts` asserts that binding in the EMITTED bundle. */
 a {
   color: inherit;
   text-decoration: none;
6120ad6e fix(verify): declare how the repo is PREPARED, and sweep before the run

diff --git a/.redkiln/config.yaml b/.redkiln/config.yaml
index 9ffcc154..a0fbf408 100644
--- a/.redkiln/config.yaml
+++ b/.redkiln/config.yaml
@@ -17,6 +17,27 @@ auto_stage_telemetry: true
 # `implement -> report` had never once fired on redkiln's own backlog. That is the
 # direct cause of the zero coverage on the generic command-gate path.
 verify:
+  # THE PRECONDITION, run before every assertion in both grains — the only key in
+  # this block that is not itself a claim about the change under test.
+  #
+  # Before it existed, .github/workflows/ci.yml ran a docs install and a docs
+  # build BEFORE its Verify step, and `affected_gate` below said nothing about
+  # either. So the declared gate was not the gate CI ran, and nothing in the
+  # product could notice the gap. A freshly cut fix worktree — which is redkiln's
+  # OWN execution substrate, since /redkiln:fix, /redkiln:implement and
+  # /redkiln:initiative each cut one — therefore met a wall of red assertions
+  # about OG-card pixel ratios and clear-space floors with nothing wrong with the
+  # tree at all (#151).
+  #
+  # A FAILURE HERE IS `not-ready`, NEVER changes-requested: the assertions below
+  # do not run, so no verdict about the change exists to record.
+  #
+  # It is CHEAP WHEN ALREADY SATISFIED, which is the constraint that matters most:
+  # it runs at least once per transition, and on the carve-out paths (--fresh,
+  # REDKILN_NO_GATE_CACHE, a tree git cannot identify) it runs inside the
+  # repo-wide write lock. scripts/build-docs.mjs mtime-compares and exits 0 when
+  # docs/dist is current, so the ordinary case spawns nothing.
+  prepare: "node scripts/build-docs.mjs"
   affected_gate: "pnpm typecheck && pnpm lint && pnpm test"
   reachability_static: "pnpm check:workflows"
   # `pnpm run ci`, NOT `pnpm ci` — pnpm reserves `ci` as a built-in (it answers
diff --git a/scripts/reap-serve-children.mjs b/scripts/reap-serve-children.mjs
index 67b0ae81..a18d8542 100644
--- a/scripts/reap-serve-children.mjs
+++ b/scripts/reap-serve-children.mjs
@@ -370,6 +370,40 @@ export async function reapServeChildren() {
   return { enumerated: true, scanned: rows.length, reaped, failed, inUse };
 }
 
+/**
+ * Vitest's suite-level SETUP — the default export of a `globalSetup` file, which
+ * vitest runs BEFORE the suite.
+ *
+ * IT WAS MISSING, AND ITS ABSENCE WAS INVISIBLE. This module exported `teardown`
+ * and nothing else, so the sweep only ever ran on the way OUT. A machine carrying
+ * leaked children from a previously CRASHED or KILLED run — the one case where
+ * teardown never got to fire, and therefore the only case that matters — began
+ * the next run dirty, and the sweep it was mounted for could not help it. That is
+ * #148's own shape (23 fixture servers alive across three days, turning an
+ * ordinary gate red with zero assertion failures) recurring in the fix for #148.
+ *
+ * SWEEPING BEFORE IS SAFE BY CONSTRUCTION, not by luck: `ownershipOf` already
+ * classifies a live sibling worker's children as `another-runner` and leaves them
+ * alone, which is what makes a sweep that runs against every process on the
+ * machine acceptable at all. Running it earlier changes nothing about that
+ * judgement — at setup time there are strictly FEWER of this run's own processes
+ * to misjudge, not more.
+ *
+ * It adds no blocking signal: like {@link teardown} it never throws and never
+ * fails the run.
+ *
+ * @returns {Promise<void>}
+ */
+export default async function setup() {
+  try {
+    await reapServeChildren();
+  } catch (err) {
+    warn(
+      `the pre-run leaked-serve-child sweep itself failed: ${err instanceof Error ? err.message : ""}`,
+    );
+  }
+}
+
 /**
  * Vitest's suite-level teardown (`vitest.config.ts` `globalSetup`, which is the
  * option that carries the `teardown` export — vitest has no separate
diff --git a/test/docs-dist-precondition.test.ts b/test/docs-dist-precondition.test.ts
new file mode 100644
index 00000000..947cc9a8
--- /dev/null
+++ b/test/docs-dist-precondition.test.ts
@@ -0,0 +1,183 @@
+/**
+ * The docs/dist precondition has ONE owner — issues #151 and #166.
+ *
+ * A whole tier of merge-blocking assertions reads `docs/dist`, and `docs/dist` is
+ * gitignored. That precondition was handled THREE different ways in three files,
+ * for the same fact:
+ *
+ *   - `typography-delivery.test.ts` BUILT it in one describe block
+ *     (`requireFreshDist`) and, in two others, went straight to `requireBuilt` —
+ *     which asserts PRESENCE and never freshness or buildability. On a checkout
+ *     that had never built the site those two produced 19 raw assertion failures
+ *     about font faces, not one of which named `docs/dist` (#166).
+ *   - `docs-brand.test.ts` and `capture-surfaces.test.ts` SILENTLY SKIPPED
+ *     (`it.skipIf(noDist)`), which is the OPPOSITE failure — a merge-blocking
+ *     tier reporting green having asserted nothing (#112).
+ *
+ * Three handlings of one fact is what happens when a precondition has no owner.
+ * It has one now: `verify.prepare` in `.redkiln/config.yaml`, which runs before
+ * any assertion, at the gate, in BOTH grains. This file guards that ownership.
+ *
+ * AC-DIST-OWNED     `verify.prepare` is declared and resolves to a real script
+ *                   that knows about `docs/dist`.
+ * AC-DIST-CHEAP     that script no-ops when `docs/dist` is already current — the
+ *                   constraint that matters most, because prepare runs at least
+ *                   once per transition and, on the carve-out paths, inside the
+ *                   repo-wide write lock.
+ * AC-DIST-FLOORED   every describe block in `typography-delivery.test.ts` that
+ *                   reads the built site floors the precondition first (#166).
+ * NF-DIST-REMEDY    the floor names the command that fixes it.
+ *
+ * PARSED AND EXECUTED, NEVER ASSUMED. AC-DIST-CHEAP actually runs the script
+ * twice and reads what it says, rather than reading its source for a mtime
+ * comparison — a staleness check that is present but wrong is exactly the shape
+ * this repo keeps rediscovering.
+ */
+import { describe, expect, it } from "vitest";
+import { execFileSync } from "node:child_process";
+import { existsSync, readFileSync } from "node:fs";
+import { join } from "node:path";
+
+import { parse as parseYaml } from "yaml";
+
+import { REPO_ROOT } from "./helpers/spawn-cli.js";
+
+/** The repo's own `verify:` block, as the product reads it. */
+function verifyBlock(): Record<string, unknown> {
+  const raw = readFileSync(join(REPO_ROOT, ".redkiln", "config.yaml"), "utf8");
+  const parsed = parseYaml(raw) as { verify?: Record<string, unknown> };
+  return parsed.verify ?? {};
+}
+
+describe("AC-DIST-OWNED: the docs/dist precondition is declared, not remembered", () => {
+  // MUTATION (run): blank `verify.prepare` in .redkiln/config.yaml — this reds
+  // naming the key, because the precondition is then owned by nothing again.
+  it("AC-DIST-OWNED: verify.prepare is declared and names a script that exists", () => {
+    const prepare = verifyBlock()["prepare"];
+    expect(
+      typeof prepare,
+      "verify.prepare is what gives the docs/dist precondition an owner; without it the " +
+        "declared gate is not the gate CI runs, and nothing in the product can notice (#151)",
+    ).toBe("string");
+
+    // ANTI-VACUITY: a declared string that names no real file owns nothing.
+    const script = /([\w./-]+\.mjs)/.exec(String(prepare))?.[1];
+    expect(script, `verify.prepare must invoke a script: ${String(prepare)}`).toBeDefined();
+    expect(existsSync(join(REPO_ROOT, script as string))).toBe(true);
+    expect(
+      readFileSync(join(REPO_ROOT, script as string), "utf8"),
+      "the prepare script must be the one that knows about docs/dist",
+    ).toMatch(/docs.{0,4}dist|dist.*index\.html/s);
+  });
+
+  it("AC-DIST-CHEAP: it no-ops when docs/dist is already current", () => {
+    // THE CONSTRAINT THAT MATTERS MOST, and it is asserted by RUNNING the thing.
+    // prepare fires at least once per transition, and on `--fresh`,
+    // `REDKILN_NO_GATE_CACHE`, or a tree git cannot identify it fires inside the
+    // repo-wide write lock — so a prepare that rebuilds unconditionally would
+    // serialize every worktree of the repo behind a site build.
+    //
+    // MUTATION (run): delete the `if (fresh) … exit(0)` short-circuit in
+    // scripts/build-docs.mjs and this reds — the second run spawns Astro.
+    const run = (): string =>
+      execFileSync("node", ["scripts/build-docs.mjs"], {
+        cwd: REPO_ROOT,
+        encoding: "utf8",
+        stdio: ["ignore", "pipe", "pipe"],
+      });
+    run(); // bring it current, whatever state the machine was in
+    const second = run();
+    expect(
+      second,
+      "the second back-to-back run must recognise a current docs/dist and skip the build",
+    ).toMatch(/current|skipping/i);
+  });
+});
+
+describe("AC-DIST-FLOORED: every block that reads the built site floors it first", () => {
+  // MUTATION (run): delete either `beforeAll(() => { requireFreshDist(); })` from
+  // typography-delivery.test.ts and this reds naming the block — which is exactly
+  // the #166 state, where two of the three blocks read docs/dist unfloored.
+  it("AC-DIST-FLOORED: each describe reading dist calls requireFreshDist", () => {
+    const source = readFileSync(join(REPO_ROOT, "test/typography-delivery.test.ts"), "utf8");
+
+    // ANCHORED AT COLUMN 0, NOT BRACE-MATCHED, and both earlier drafts of this
+    // guard are the reason. Splitting on the next `describe(` swept this file's
+    // MODULE-SCOPE helpers (`function requireBuilt`, `distPage`, `distFiles`) into
+    // the preceding block and accused `describe("payload")`. Replacing that with
+    // naive brace counting was worse — it passed with a floor deliberately
+    // deleted, because counting `{` and `}` through 293 KB of strings, template
+    // literals and regexes drifts, and a guard that cannot fail is not a guard.
+    //
+    // What IS reliable here is prettier's own output: a top-level `describe(` and
+    // a top-level `function` both begin at column 0 and are closed by a `}` at
+    // column 0. That is a property of the formatter this repo enforces in CI, so
+    // it is a fact about the file rather than a hope about its contents.
+    const topLevel = (kw: RegExp): { title: string; body: string }[] => {
+      const lines = source.split("\n");
+      const out: { title: string; body: string }[] = [];
+      for (let i = 0; i < lines.length; i += 1) {
+        const m = kw.exec(lines[i] as string);
+        if (m === null) continue;
+        let end = lines.length;
+        for (let j = i + 1; j < lines.length; j += 1) {
+          if (/^\}/.test(lines[j] as string)) {
+            end = j;
+            break;
+          }
+        }
+        out.push({ title: m[1] ?? "", body: lines.slice(i, end + 1).join("\n") });
+      }
+      return out;
+    };
+
+    // COMMENTS ARE STRIPPED FIRST, and that is not tidiness — it is the third
+    // defect this one guard has now had, and the one this repo keeps rediscovering.
+    // The comment two blocks up NAMES `requireFreshDist()` while explaining why the
+    // floor exists, so a guard reading raw source found the floor in the prose and
+    // passed with the call deleted. Phase 0 hit the identical shape when a comment
+    // in `scripts/build.mjs` naming `pnpm run ci:dod` put it on `ci`'s dependency
+    // chain. A guard that reads TEXT is asserting a promise's wording, not its
+    // behaviour (#134/#139).
+    const strip = (text: string): string =>
+      text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "");
+    const describes = topLevel(/^describe\("([^"]+)"/);
+    const helpers = topLevel(/^(?:export )?function (\w+)/);
+    expect(describes.length, "ANTI-VACUITY: no describe blocks were parsed").toBeGreaterThan(3);
+    expect(helpers.length, "ANTI-VACUITY: no module-scope helpers were parsed").toBeGreaterThan(0);
+
+    // A block READS the built site when its own body calls a dist reader. Helper
+    // bodies are excised first, because a definition is not a call — that excision
+    // is the whole difference between this guard and its first draft.
+    const helperBodies = new Set(helpers.map((h) => h.body));
+    const reads = (body: string): boolean => {
+      let own = body;
+      for (const h of helperBodies) own = own.split(h).join("");
+      return /\b(requireBuilt|distPage|distFiles)\(/.test(own);
+    };
+
+    const readsDist = describes.filter((d) => reads(d.body));
+    expect(
+      readsDist.map((d) => d.title),
+      "ANTI-VACUITY: fewer than two blocks read the built site, so this guard would pass " +
+        "over the exact regression it exists to catch (#166 named two)",
+    ).toHaveLength(3);
+
+    for (const { title, body } of readsDist) {
+      expect(
+        strip(body),
+        `the describe "${title}" reads the built site without flooring the precondition. ` +
+          "Unfloored, a checkout that has never built docs/ fails with raw assertions about " +
+          "font faces that never mention docs/dist (#166) — and bare `pnpm test` is this " +
+          "repo's own affected_gate, so a fresh clone cannot pass its own gate.",
+      ).toMatch(/requireFreshDist\(\)/);
+    }
+  });
+
+  it("NF-DIST-REMEDY: the floor names the command that fixes it", () => {
+    // A precondition failure that does not say how to satisfy it is the failure
+    // #166 describes: 19 reds, no remedy, and a fix nobody could infer.
+    const source = readFileSync(join(REPO_ROOT, "test/typography-delivery.test.ts"), "utf8");
+    expect(source).toMatch(/cd docs && pnpm install --frozen-lockfile && pnpm build/);
+  });
+});
diff --git a/test/serve-child-reaping.test.ts b/test/serve-child-reaping.test.ts
index 84dab3ca..344673c0 100644
--- a/test/serve-child-reaping.test.ts
+++ b/test/serve-child-reaping.test.ts
@@ -505,3 +505,56 @@ describe("AC-005: the sweep predicate never touches an author's real board", ()
     ).toBe(false);
   });
 });
+
+// ---------------------------------------------------------------------------
+// AC-006 — the sweep runs BEFORE the run, not only after it
+// ---------------------------------------------------------------------------
+
+describe("AC-006: the sweep is mounted on both ends of a run", () => {
+  // WHY THIS IS ITS OWN AC, AND WHY IT IS ABOUT THE MODULE'S EXPORTS. Every other
+  // case here proves the sweep WORKS. None of them could notice that it was only
+  // ever CALLED on the way out: this module exported `teardown` and nothing else,
+  // and vitest runs a `globalSetup` file's default export before the suite and its
+  // `teardown` export after. So the one situation the sweep exists for — a machine
+  // still carrying children from a run that CRASHED or was KILLED, which is
+  // precisely the run whose teardown never fired — began the next run dirty and
+  // was swept only on the way out of it. That is #148's own shape recurring inside
+  // the fix for #148, and it survived because "the sweep is mounted" and "the
+  // sweep runs before a run" are not the same claim.
+  //
+  // MUTATION (run): delete the `export default` from
+  // `scripts/reap-serve-children.mjs` and the first case reds naming the export.
+  // MUTATION (run): rename it to a named `export async function setup()` — nothing
+  // reds, and that is correct: vitest accepts either form, so the guard asserts a
+  // callable pre-run hook exists rather than pinning one spelling of it.
+
+  it("AC-006: it exports a pre-run hook, not only a teardown", async () => {
+    const mod: Record<string, unknown> = await import("../scripts/reap-serve-children.mjs");
+    const preRun = mod["default"] ?? mod["setup"];
+    expect(
+      typeof preRun,
+      "vitest runs a globalSetup file's default (or `setup`) export BEFORE the suite; with " +
+        "neither, the sweep only ever runs on the way out and a run that starts dirty stays dirty",
+    ).toBe("function");
+    expect(typeof mod["teardown"], "the after-the-run half must survive too").toBe("function");
+  });
+
+  it("AC-006: the pre-run hook is the one vitest actually mounts", () => {
+    // ANTI-VACUITY, and it is doing real work: an export nothing is wired to is
+    // indistinguishable from no export at all. This pins that `vitest.config.ts`
+    // still points `globalSetup` at THIS module — the file the assertion above
+    // inspected — so the two halves cannot drift apart.
+    const config = readFileSync(join(REPO_ROOT, "vitest.config.ts"), "utf8");
+    expect(config).toMatch(/globalSetup:\s*\[\s*["'].*reap-serve-children\.mjs["']/);
+  });
+
+  it("AC-006: sweeping before a run cannot reap a live sibling's children", () => {
+    // The safety argument for moving the sweep earlier, asserted rather than
+    // assumed. `ownershipOf` is what makes a machine-wide sweep acceptable at all,
+    // and it is unchanged by WHEN the sweep runs — so a sibling worker's child is
+    // still classified `another-runner` and left alone.
... TRUNCATED at 400 lines; 57 commits total
```
