# 14/20 — _archive/docs-site-brand-identity/contrast-verification

**kind:** project · **commits:** 26 · **files from commits:** 383 · **paths named only in the card:** 9

## Files changed (outside .bklg/)

- `dist/cli.js`
- `dist/index.js`
- `eslint.config.js`
- `src/store/verify.ts`
- `test/capture-bench.test.ts`
- `test/cli-help.smoke.test.ts`
- `test/contrast-verification.test.ts`
- `test/docs-brand.test.ts`
- `test/helpers/tool-bookkeeping.ts`
- `test/project-diff-scope.test.ts`
- `test/serve-perimeter.test.ts`
- `test/tool-bookkeeping.test.ts`
- `test/verify.test.ts`
- `tools/eslint-rules/no-unpinned-diff-base.js`
- `test/dod-gate-wiring.test.ts`
- `.claude-plugin/plugin.json`
- `.gitattributes`
- `.github/workflows/ci.yml`
- `.gitignore`
- `.kb/README.md`
- `.kb/_intake/board-contract-additive-evolution.md`
- `.kb/decisions/gate-decision-write-surface.md`
- `.kb/decisions/live-board-loopback-serve.md`
- `.kb/design/evidence-before-decision-surface.md`
- `.kb/design/hierarchical-gate-rollup.md`
- `.kb/design/persistent-context-banner-chrome.md`
- `.kb/design/resolve-a-design-against-a-measured-mock.md`
- `.kb/design/severity-in-flow-not-overlay.md`
- `.kb/design/status-vocabulary-non-colour-non-motion.md`
- `.kb/maps/live-board-surface-knowledge-map.md`
- `.kb/open-questions/brand-unwired-from-pipeline.md`
- `.kb/playbooks/instruments-produce-readings.md`
- `.kb/product/README.md`
- `.kb/product/journey-discover-and-clear-a-gate.md`
- `.kb/product/journey-glance-up-and-read-state.md`
- `.kb/product/journey-open-the-board-once-and-trust-it.md`
- `.kb/product/persona-approver-at-the-gate.md`
- `.kb/product/persona-author-leaving-the-board-open.md`
- `.kb/product/persona-bystander-with-open-tab.md`
- `.kb/product/persona-watcher-second-monitor.md`
- `.kb/reference/workflow-schema-validation-envelope.md`
- `.prettierignore`
- `.redkiln/config.yaml`
- `.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity.jsonl`
- `.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity@runs.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-advance-write-lock-gate-scope.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-doctor-git-config-repair.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-62-new-stale-pinned-template.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-63-advance-gate-diagnostic-truncated.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-66-id-lock-ec001-rmdir.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-72-plan-briefs-readonly-grounding.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-73-plan-briefs-missing-artifacts-discarded.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-74-plan-briefs-dead-plan-anchor.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-75-item-frontmatter-strict-drift.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-79-plan-prds-verifier-false-pass.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-81-plan-discover-spec-gate-mismatch.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-82-status-terminal-field-collision.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-83-plan-prds-write-timeout.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-repo-root-memo.jsonl`
- `.redkiln/telemetry/events/ryan-britton@issue-backlog-sweep-lane@runs.jsonl`

## Card body (the item as written)

```
## One-line objective

Make "every brand-derived colour meets its contrast threshold in the cut it appears in" a
continuously-proven property of the token substrate — asserted over the pairs the design
commits to, in both cuts, reporting *which* threshold failed — so the known dark-cut failure
is visible at the first candidate value rather than discovered at ship.

## How this advances the initiative

The initiative's contrast risk is not hypothetical and it is not symmetric. Computed from the
identity's own published values, the fired brick `#A84322` reads ≈5.58:1 against the light
ground `#FAF6F0` and ≈2.98:1 against the dark ground `#1B1613`
([`brand/README.md`](../../../../brand/README.md) palette table) — pass and fail from one value,
missing both the 4.5:1 body-text threshold and the 3:1 non-text floor. The charter names
"dark-cut contrast failure ships, because the light cut passed and symmetry was assumed" as a
**High / Critical** risk, and records the resolution that verification is an *ongoing
property*, not a ship-time measurement, because a one-time check cannot catch the token nudged
six months from now that silently drops the dark cut back under the floor
([`.bklg/docs-site-brand-identity/initiative.md`](../initiative.md) — *Clarifications resolved
at distillation*).

This project owns initiative requirement **BR-5** and acceptance criterion **AC-5**, and scope
letter **F**. It is the primary owner of Definition-of-Done scenario **8** ("contrast, measured,
in both cuts") and supplies the 3:1 non-text floor that scenario **10**'s keyboard traverse is
judged against
([`.bklg/docs-site-brand-identity/_decomposition.md`](../_decomposition.md) — *Definition-of-Done
coverage*).

Its position in the DAG is the point of it. The decomposition sequences this project **second,
before `highlight-and-states`**, on the explicit ground that the harness must exist *before* any
highlight value is chosen: that converts the initiative's highest-impact risk from "discovered
late" to "visible at the first candidate value", and it removes the multi-week retrofit every
surveyed retokenization account warns about
([research 05](../_discovery/research/05-accent-budget-vs-ui-state.md),
[research 07](../_discovery/research/07-docs-retokenize-pain-points.md)).

There is a second, quieter payoff the charter names: a test that reads the substrate is
continuous evidence the substrate is machine-readable and load-bearing. It keeps **BR-1** honest
after the point-in-time DoD check that proves it has passed.

## In scope (this project)

- **A contrast harness over declared foreground/background pairs.** It reads the pairs the
  substrate enumerates — never the cartesian product of every token, which manufactures failures
  on combinations nothing renders.
- **Per-cut assertion.** Light and dark are evaluated separately against the ground each pair is
  declared for. A pass in one cut asserts nothing about the other.
- **Threshold classification by role.** Each declared pair carries the role it plays — body text
  (4.5:1), large text (3:1), or non-text/UI including focus indicators and state chrome (3:1) —
  and is asserted against that floor, not a single global number.
- **A failure message that names the failing thing.** Pair, cut, measured ratio, the required
  threshold, and the threshold's name. "Failed" alone does not tell a reader that ≈2.98:1 missed
  both applicable floors.
- **The WCAG logo exemption, stated explicitly.** The mark and lockup are exempt and are seen to
  be exempt, in the harness itself — not exempt by having been quietly left out of the pair list.
- **The ratio computation and its own fixture tests.** The repository's root package depends only
  on `yaml` and `zod` ([`package.json`](../../../../package.json)), so sRGB relative luminance and
  the `(L1+0.05)/(L2+0.05)` ratio are computed in-repo and proven against known-good values from
  `brand/README.md` before they are trusted to judge anything.
- **A recorded-exception mechanism.** DoD 8 permits an exception as "a recorded, justified
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
9a4d8fb2 fix(guards): guards that assert behaviour, and an exemption that is itself asserted

diff --git a/dist/cli.js b/dist/cli.js
index 7c2b7746..31ccf21c 100644
--- a/dist/cli.js
+++ b/dist/cli.js
@@ -16469,6 +16469,16 @@ function waiverOf(entry, actor) {
   return { ok: true };
 }
 var PLACEHOLDER_EVIDENCE = /^(|-|n\/?a|tbd|todo|fixme|xxx|\.{3}|…)$/i;
+var EVIDENCE_CITATION = /[\w./@-]+\.(?:ts|tsx|js|mjs|cjs|md|mdx|ya?ml|json|html|css|png)\b|\b[0-9a-f]{7,40}\b/i;
+function satisfiedEvidenceProblem(ac, evidence) {
+  const text = evidence.trim();
+  if (PLACEHOLDER_EVIDENCE.test(text)) return `${ac}: satisfied without cited evidence`;
+  const withoutSelf = text.replace(new RegExp(ac.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "gi"), "");
+  if (!EVIDENCE_CITATION.test(withoutSelf)) {
+    return `${ac}: satisfied, but its evidence cites nothing a reader can check \u2014 name the file, test or commit that demonstrates it`;
+  }
+  return void 0;
+}
 function ledgerBlock(text) {
   const lines = text.split(/\r?\n/);
   const headingIdx = lines.findIndex((l) => /^#{1,6}\s+.*ledger/i.test(l));
@@ -16578,9 +16588,8 @@ function ledgerCheck(root, dir, state, base, cfg) {
       continue;
     }
     if (entry.satisfied) {
-      if (PLACEHOLDER_EVIDENCE.test(entry.evidence.trim())) {
-        failures.push(`${ac}: satisfied without cited evidence`);
-      }
+      const problem = satisfiedEvidenceProblem(ac, entry.evidence);
+      if (problem !== void 0) failures.push(problem);
       continue;
     }
     const waiver = waiverOf(entry, actor);
diff --git a/dist/index.js b/dist/index.js
index 434b797e..9df9594b 100644
--- a/dist/index.js
+++ b/dist/index.js
@@ -16469,6 +16469,16 @@ function waiverOf(entry, actor) {
   return { ok: true };
 }
 var PLACEHOLDER_EVIDENCE = /^(|-|n\/?a|tbd|todo|fixme|xxx|\.{3}|…)$/i;
+var EVIDENCE_CITATION = /[\w./@-]+\.(?:ts|tsx|js|mjs|cjs|md|mdx|ya?ml|json|html|css|png)\b|\b[0-9a-f]{7,40}\b/i;
+function satisfiedEvidenceProblem(ac, evidence) {
+  const text = evidence.trim();
+  if (PLACEHOLDER_EVIDENCE.test(text)) return `${ac}: satisfied without cited evidence`;
+  const withoutSelf = text.replace(new RegExp(ac.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "gi"), "");
+  if (!EVIDENCE_CITATION.test(withoutSelf)) {
+    return `${ac}: satisfied, but its evidence cites nothing a reader can check \u2014 name the file, test or commit that demonstrates it`;
+  }
+  return void 0;
+}
 function ledgerBlock(text) {
   const lines = text.split(/\r?\n/);
   const headingIdx = lines.findIndex((l) => /^#{1,6}\s+.*ledger/i.test(l));
@@ -16578,9 +16588,8 @@ function ledgerCheck(root, dir, state, base, cfg) {
       continue;
     }
     if (entry.satisfied) {
-      if (PLACEHOLDER_EVIDENCE.test(entry.evidence.trim())) {
-        failures.push(`${ac}: satisfied without cited evidence`);
-      }
+      const problem = satisfiedEvidenceProblem(ac, entry.evidence);
+      if (problem !== void 0) failures.push(problem);
       continue;
     }
     const waiver = waiverOf(entry, actor);
diff --git a/eslint.config.js b/eslint.config.js
index ddf8c665..bcd33203 100644
--- a/eslint.config.js
+++ b/eslint.config.js
@@ -2,6 +2,7 @@ import js from "@eslint/js";
 import globals from "globals";
 import tseslint from "typescript-eslint";
 import * as espree from "espree";
+import noUnpinnedDiffBase from "./tools/eslint-rules/no-unpinned-diff-base.js";
 
 // The Workflow runtime de-exports `meta` and evaluates a workflow's body inside an
 // async wrapper (workflows/_lib/check-parse.mjs:49-50), so these files legally carry
@@ -112,10 +113,34 @@ export default tseslint.config(
       },
     },
   },
+  {
+    // THE #139 RATCHET. A guard scoped to one initiative asserts something strictly
+    // broader than the AC it belongs to, and once that initiative merges the assertion
+    // graduates from "this initiative behaved" to "nobody may ever do this again" — and
+    // starts reding inside unrelated work. The three originally-named instances are
+    // fixed and the rest are consciously fenced, so what is left is stopping the next
+    // one being written.
+    //
+    // A repo-local rule as an inline plugin object: the same shape `workflowParser`
+    // above already uses, and it needs no new dependency. Scoped to `test/` because
+    // that is where blast-radius guards live; `src/store/verify.ts` computes a
+    // `${base}...HEAD` diff as PRODUCT behaviour, which is correct and not this defect.
+    files: ["test/**/*.ts"],
+    plugins: { redkiln: { rules: { "no-unpinned-diff-base": noUnpinnedDiffBase } } },
+    rules: { "redkiln/no-unpinned-diff-base": "error" },
+  },
   {
     // Build/config scripts and the bin shim are plain ESM, not part of the
     // typed program. Lint them with node globals and no type-aware rules.
-    files: ["scripts/**/*.mjs", "bin/**/*.mjs", "workflows/_lib/*.mjs", "*.config.{js,mjs}"],
+    files: [
+      "scripts/**/*.mjs",
+      "bin/**/*.mjs",
+      "workflows/_lib/*.mjs",
+      // The repo-local eslint rules. Plain ESM, and they must match SOME config block
+      // or they lint under bare `js.configs.recommended` with no node globals.
+      "tools/**/*.js",
+      "*.config.{js,mjs}",
+    ],
     languageOptions: {
       globals: { ...globals.node },
     },
diff --git a/src/store/verify.ts b/src/store/verify.ts
index fe36e676..2c13eb08 100644
--- a/src/store/verify.ts
+++ b/src/store/verify.ts
@@ -1590,6 +1590,86 @@ function waiverOf(entry: LedgerEntry, actor: string): { ok: boolean; reason?: st
 /** Evidence that is present in form but says nothing — treated as no evidence. */
 const PLACEHOLDER_EVIDENCE = /^(|-|n\/?a|tbd|todo|fixme|xxx|\.{3}|…)$/i;
 
+/**
+ * A CITATION: something a reader could go and look at.
+ *
+ * A repo-relative path with a known extension, or a git sha. Deliberately NOT "any
+ * prose" — see {@link satisfiedEvidenceProblem} for why non-emptiness was not enough.
+ */
+const EVIDENCE_CITATION =
+  /[\w./@-]+\.(?:ts|tsx|js|mjs|cjs|md|mdx|ya?ml|json|html|css|png)\b|\b[0-9a-f]{7,40}\b/i;
+
+/**
+ * A DENIAL DETECTOR WAS BUILT HERE AND BACKED OUT. Recorded so it is not re-derived.
+ *
+ * #134's third instance is a row whose evidence states in prose that the criterion was
+ * NOT met, so refusing self-denying prose looks like the direct fix. It was written —
+ * `/\bnot\s+(met|satisfied|implemented|verified|done|completed)\b|\bdeferred to\b/i`,
+ * already narrowed past the obvious `\bfails?\b` trap — and then run over all 1,218
+ * satisfied rows in this repo's corpus before shipping.
+ *
+ * It produced TWO hits and BOTH were false positives:
+ *
+ *   - `legibility-without-colour-read` AC-008 quotes the product's own UI copy:
+ *     "Currency: [!] Held over — the last read could not complete".
+ *   - `reconcile-knowledge-map` AC-006 describes a rule's verified outcome:
+ *     "the verdict is stated (threshold not met, the bullet stays)".
+ *
+ * Evidence legitimately quotes product strings and describes conditions, so these
+ * words carry no reliable signal about the ROW. This repo already has two guards that
+ * false-positive on ordinary prose (`AC-KB-TIER-ONE-HOME`, `AC-RUN-EMIT-BEFORE-HALT`),
+ * each of which cost a full CI cycle to diagnose; a third would be a net loss.
+ *
+ * What survives is the CITATION requirement below, which is a fact rather than a
+ * judgement about prose — and which is exactly what `_ledger.md`'s own header has
+ * always promised: "MUST cite real evidence (a `file:line` and/or the verifying test
+ * id)". The promise was in the document; nothing enforced it. That is #134's thesis,
+ * so enforcing the promise is the honest fix and reading intent out of prose is not.
+ */
+
+/**
+ * Why a `satisfied: true` row's evidence is not evidence — or `undefined` if it is.
+ *
+ * #134's third instance, and the one on `main`. The check here used to be
+ * {@link PLACEHOLDER_EVIDENCE} alone: a closed set of eight literal placeholder forms,
+ * anchored `^…$`. So **any** other string cleared a flipped row — including prose
+ * stating in so many words that the criterion was NOT met and deferring it to a human.
+ * Two rows shipped exactly that way. The implementer's escalation was correct and openly
+ * written; the row state was the opposite of it, and only the row state is
+ * machine-visible, so a project review reading rows would have ratified both silently.
+ *
+ * The shape of the defect is #134's thesis: the guard asked whether a DESCRIPTION was
+ * present, and presence of a description is not evidence of the thing described.
+ *
+ * SO THE CONDITION IS CITATION, and it is a fact rather than a reading of prose: the
+ * evidence must name a path or a sha. "the tests pass" is a claim; `test/foo.test.ts`
+ * is a place to go and check. The row's own AC id is stripped first, so echoing
+ * `AC-007` back is not a citation.
+ *
+ * The waiver path has required a tracking id (`WAIVER_TRACKS`) since it was written —
+ * this gives the `satisfied` path the counterpart it never had, which is why a waiver
+ * was the honest route and `satisfied: true` was the cheap one.
+ *
+ * MEASURED BEFORE SHIPPING, over all 1,218 satisfied rows in this repo's corpus:
+ * **0 live rows** newly fail, and 7 archived ones do (all in `.bklg/_archive/**`,
+ * where nothing advances). Rows citing "Devtools measurement on the real built site"
+ * with no file are the shape it catches, and that is the intended catch, not a
+ * regression.
+ */
+function satisfiedEvidenceProblem(ac: string, evidence: string): string | undefined {
+  const text = evidence.trim();
+  if (PLACEHOLDER_EVIDENCE.test(text)) return `${ac}: satisfied without cited evidence`;
+  // Strip the row's own id so it cannot cite itself.
+  const withoutSelf = text.replace(new RegExp(ac.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"), "gi"), "");
+  if (!EVIDENCE_CITATION.test(withoutSelf)) {
+    return (
+      `${ac}: satisfied, but its evidence cites nothing a reader can check — ` +
+      `name the file, test or commit that demonstrates it`
+    );
+  }
+  return undefined;
+}
+
 /**
  * The body of the first fenced code block that follows a heading whose text
  * contains "ledger", or `undefined` when there is none.
@@ -1782,9 +1862,8 @@ function ledgerCheck(
       continue;
     }
     if (entry.satisfied) {
-      if (PLACEHOLDER_EVIDENCE.test(entry.evidence.trim())) {
-        failures.push(`${ac}: satisfied without cited evidence`);
-      }
+      const problem = satisfiedEvidenceProblem(ac, entry.evidence);
+      if (problem !== undefined) failures.push(problem);
       continue;
     }
     // NOT satisfied. The gate passes on "satisfied OR waived-with-reason" — and a
diff --git a/test/capture-bench.test.ts b/test/capture-bench.test.ts
index f359bc10..a198bd88 100644
--- a/test/capture-bench.test.ts
+++ b/test/capture-bench.test.ts
@@ -29,11 +29,20 @@
 import { afterAll, beforeAll, describe, expect, it } from "vitest";
 import { execFileSync, spawnSync } from "node:child_process";
 import { createServer } from "node:http";
-import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
+import {
+  existsSync,
+  mkdtempSync,
+  readFileSync,
+  readdirSync,
+  rmSync,
+  statSync,
+  writeFileSync,
+} from "node:fs";
 import { tmpdir } from "node:os";
 import { dirname, join, resolve } from "node:path";
 import { fileURLToPath, pathToFileURL } from "node:url";
 
+import { decodePng } from "../scripts/capture-viewport.mjs";
 import {
   MEDIA_PASSES,
   browserExecutable,
@@ -168,6 +177,34 @@ try {
       `  AC-001/002/003/004/005/006/007/009/011 are NOT proven by this run.\n`,
   );
 }
+/**
+ * ON CI, AN UNRESOLVABLE BROWSER IS A RESULT — NOT A SILENCE (#167).
+ *
+ * `describe.skip` reports GREEN. So on any machine without Chrome the nine ACs above
+ * are simply not proven, and the run is indistinguishable from one that proved them —
+ * which is the same defect class as `PLACEHOLDER_EVIDENCE` and the whole subject of
+ * this remediation phase. #167 filed it as "the browser lifecycle has no behavioural
+ * test"; that headline is FALSE at HEAD (the leg below runs `capture.mjs` in its real
+ * mode and asserts four named PNGs, a manifest bijection and the AC-011 teardown proof).
+ * The true residual is that nothing ever established the leg RAN.
+ *
+ * Locally, skipping stays right: a contributor without Chrome should not be blocked, and
+ * the stdout note above tells them what went unproven. On CI there is no such tradeoff —
+ * both runners ship Chrome, and a leg that quietly stopped running is exactly what a
+ * trunk must not tolerate.
+ *
+ * VERIFIED BEFORE SHIPPING, so this cannot red the trunk on day one: PR #187's run logs
+ * show `test/capture-bench.test.ts (25 tests) 136953ms` on ubuntu and `145557ms` on
+ * windows, with no `BROWSER LEG NOT RUN` marker on either. The leg runs on both legs
+ * today; this only stops it stopping unnoticed.
+ */
+if (process.env["CI"] === "true" && browserPath === "") {
+  throw new Error(
+    `capture-bench: no browser resolved on CI, so the capture lifecycle (AC-001/002/003/004/` +
+      `005/006/007/009/011) cannot be proven by this run — and a skipped guard reports green, ` +
+      `which is indistinguishable from a passing one (#167). ${absence}`,
+  );
+}
 const withBrowser = browserPath === "" ? describe.skip : describe;
 
 /** One board document, rendered by the real CLI, shot by the real camera. */
@@ -322,7 +359,14 @@ withBrowser("a pass cannot leak into the next capture (AC-001, AC-002, AC-012)",
     ).toBe(false);
   }, 180_000);
 
-  it("the bench opens its own page for every capture", () => {
+  // SOURCE-SHAPE GUARD, not a behavioural one (#167). It reads the file's CHARACTERS,
+  // so it is green whether `screenshot()` opens one browser or forty, awaits the right
+  // CDP event, or produces a viable PNG at all — which is precisely what diverged in
+  // #107 and what a text scan cannot see. The behavioural cover for this path is the
+  // `withBrowser` block above; this only pins the SHAPE a reader would otherwise
+  // "simplify" away. Labelled so its green is not mistaken for coverage of the
+  // lifecycle.
+  it("the bench opens its own page for every capture (source shape)", () => {
     const source = readFileSync(CAPTURE, "utf8");
     expect(source).toMatch(/const page = await browser\.newPage\(\)/);
     expect(source).toMatch(/await page\.close\(\)/);
@@ -500,8 +544,98 @@ withBrowser("the gate command still produces its images, through the new camera"
   });
 });
 
+withBrowser("reference mode drives its own browser lifecycle (#167)", () => {
+  // THE ONE CAPTURE PATH NOTHING EVER EXERCISED. `runReferenceMode`
+  // (`scripts/capture.mjs:4584`) opens its own browser at `:4606` and releases it in a
+  // `finally` — and no test in this repo passes `--reference-out` at all, so both the
+  // open and the release were source-only. It is a SECOND lifecycle beside the
+  // surface-capture one below, which is exactly the shape #107 records diverging: two
+  // initiatives grew `capture.mjs` in parallel and settled on incompatible browser
+  // lifecycle models, and nothing would have caught it.
+  //
+  // A hand-built mock rather than a project's signed one, so the case owns its own
+  // fixture and cannot go red when a `.bklg/_archive/**` mock is rewritten — the
+  // failure mode #124 records.
+  const VIEWPORT = "1440x900";
+  const SURFACE = "probe";
+  let dir = "";
+  let out = "";
+  let res = { status: 1, stdout: "", stderr: "" };
+
+  beforeAll(() => {
+    dir = scratch();
+    out = join(dir, "reference");
+    const mock = join(dir, "mock.html");
+    // The id shape `runReferenceMode` derives its selector from:
+    // `#${surface}--light--${vp}--light`.
+    writeFileSync(
+      mock,
+      `<!doctype html><meta charset="utf-8"><body style="margin:0">
+         <div id="${SURFACE}--light--${VIEWPORT}--light"
+              style="width:320px;height:200px;background:linear-gradient(90deg,#123 0%,#cfe 100%)">
+           <p style="font:16px/1.4 system-ui;color:#fff;padding:8px">reference probe</p>
+         </div>
+       </body>`,
+      "utf8",
+    );
+    res = runBench([
+      "--reference-out",
+      out,
+      "--mock",
+      mock,
+      "--surfaces",
+      SURFACE,
+      "--viewports",
+      VIEWPORT,
+    ]);
+  }, 900_000);
+
+  it("photographs the signed frame and writes a viable PNG", () => {
+    expect(res.status, res.stderr).toBe(0);
+    const png = join(out, `${SURFACE}@${VIEWPORT}.png`);
+    expect(existsSync(png), `${png} was not written`).toBe(true);
+
+    // NOT `size > 0`. A blank frame is a valid PNG and compresses to almost
+    // nothing, so bytes alone cannot tell "photographed the frame" from
+    // "photographed nothing" — the exact false green this whole phase is about.
+    const decoded = decodePng(readFileSync(png));
+    expect(decoded.width, "the frame was not measured and resized to fit").toBe(320);
+    expect(decoded.height).toBe(200);
+
+    // A gradient, so a uniform field means the page never painted.
+    const { pixels, channels } = decoded;
+    const seen = new Set<string>();
+    for (let i = 0; i + 2 < pixels.length; i += channels) {
+      seen.add(`${pixels[i]},${pixels[i + 1]},${pixels[i + 2]}`);
+    }
+    expect(seen.size, "the capture is a uniform field — the frame never rendered").toBeGreaterThan(
+      16,
+    );
+  });
+
+  it("releases its browser, so a second reference run succeeds", () => {
+    // The `finally` at the open site, made observable. A stranded headless Chrome
+    // makes the NEXT run fail for a reason nobody can see, which is what the comment
+    // at the open site says it exists to prevent — and what nothing checked.
+    const again = runBench([
+      "--reference-out",
+      join(dir, "reference-2"),
+      "--mock",
+      join(dir, "mock.html"),
+      "--surfaces",
+      SURFACE,
+      "--viewports",
+      VIEWPORT,
+    ]);
+    expect(again.status, again.stderr).toBe(0);
+    expect(existsSync(join(dir, "reference-2", `${SURFACE}@${VIEWPORT}.png`))).toBe(true);
+  });
+});
+
 describe("the media-pass declarations produce the design's own filenames (AC-012)", () => {
-  it("declares the two variant frames the signed-off design names", () => {
+  // SOURCE-SHAPE GUARD (#167): a regex over `capture.mjs`'s characters. The frames it
+  // names are actually PRODUCED and decoded in the `withBrowser` block above.
+  it("declares the two variant frames the signed-off design names (source shape)", () => {
     const source = readFileSync(CAPTURE, "utf8");
     expect(source).toContain("forced-colors");
     expect(source).toContain("reduced-motion");
@@ -520,7 +654,10 @@ describe("the media-pass declarations produce the design's own filenames (AC-012
 });
 
 describe("the settle that replaced --virtual-time-budget is documented (NF-006)", () => {
... TRUNCATED at 400 lines; 26 commits total
```
