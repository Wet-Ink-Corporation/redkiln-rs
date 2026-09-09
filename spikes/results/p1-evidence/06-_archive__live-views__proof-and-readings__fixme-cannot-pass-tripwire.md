# 6/20 — _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire

**kind:** story · **commits:** 1 · **files from commits:** 4 · **paths named only in the card:** 0

## Files changed (outside .bklg/)

- `package.json`
- `scripts/dod-tripwire.mjs`
- `test/dod-gate-wiring.test.ts`
- `test/dod-tripwire.test.ts`

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
d35850dc feat(proof-and-readings): Fixme cannot pass tripwire

diff --git a/package.json b/package.json
index 5d238bf8..f147f1b8 100644
--- a/package.json
+++ b/package.json
@@ -54,7 +54,7 @@
     "format": "prettier --write .",
     "check:workflows": "node workflows/_lib/check-sync.mjs && node workflows/_lib/check-parse.mjs && node workflows/_lib/check-prompts.mjs",
     "ci": "pnpm typecheck && pnpm lint && pnpm test && pnpm check:workflows && pnpm build && pnpm dogfood",
-    "ci:dod": "pnpm run ci && pnpm test:dod",
+    "ci:dod": "pnpm run ci && node scripts/dod-tripwire.mjs",
     "dogfood": "node bin/redkiln.mjs validate && node bin/redkiln.mjs validate --kb && node bin/redkiln.mjs doctor",
     "prepublishOnly": "pnpm build"
   },
diff --git a/scripts/dod-tripwire.mjs b/scripts/dod-tripwire.mjs
new file mode 100644
index 00000000..f0398982
--- /dev/null
+++ b/scripts/dod-tripwire.mjs
@@ -0,0 +1,437 @@
+// The fixme/skip-forbid tripwire on the declared `e2e` chain.
+//
+// WHY IT EXISTS. `redkiln verify` runs the repo's declared commands and does not
+// itself grep for `test.fixme`. A skipped test reports *skipped, exit 0*, so a DoD
+// scenario that was never run is indistinguishable from one that passed
+// (`.kb/decisions/implement-loop-done-wired-and-reachable.md`, "Enforcement
+// nuance"). The same Accepted decision names the remedy — a fixme-forbid tripwire
+// carried by the DECLARED `e2e` command — and records that declaring it is the
+// recommended config. This is that tripwire.
+//
+// IT MUST NEVER EXIT 0 HAVING PRODUCED NOTHING. The rule is already house doctrine
+// in this repo, in caps, at `scripts/capture.mjs:9`, for the one other instrument
+// in this pipeline that can be silently vacuous. Zero collected scenarios, an
+// unparseable result, a crashed child or a missing expected-slug source is a
+// FAILURE here, never a pass.
+//
+// IT NEVER LAUNDERS A RED INTO A GREEN. A wrapper that runs a suite and renders
+// its own verdict is one bad `catch` away from converting a real product failure
+// into a pass. This one is ADDITIVE: the child's stdout and stderr are inherited
+// so its diagnostics reach the operator unmodified, and a non-zero child exit is
+// propagated as-is. The verdict can only ever turn a green into a red.
+//
+// THERE IS NO OFF SWITCH. No `SKIP_TRIPWIRE`, no `--no-tripwire`, no permitted-skip
+// list, no threshold below thirteen. "No advertised safeguard may exist only in
+// prose" (same atom, P8) — and a guard with a documented bypass is that failure
+// wearing a switch. Amending the ratified thirteen is a human decision recorded
+// through `redkiln advance`, never a quiet edit to an expectation here.
+//
+// IT RUNS THE SUITE ONCE. The DoD invocation on the declared chain goes THROUGH
+// this step rather than beside it: `pnpm run ci:dod` = `pnpm run ci && node
+// scripts/dod-tripwire.mjs`, and this file is the only thing on that chain that
+// starts the suite. A chain that ran the whole DoD twice is the kind of gate people
+// build a bypass for.
+//
+// THE DECISION IS A PURE FUNCTION. `evaluate()` takes an expectation, a parsed
+// outcome and a scan result and returns a verdict; everything above it is I/O. That
+// is what lets the focused test (`test/dod-tripwire.test.ts`) plant every evasion
+// cheaply and still prove the wiring with real child-process cases.
+//
+// Usage:  node scripts/dod-tripwire.mjs [--root <dir>]
+//   --root   the repository to guard (default: this script's repo). Everything
+//            else is derived from it: the charter that declares the thirteen
+//            slugs, the suite directory to scan, and the runner config.
+
+import { spawnSync } from "node:child_process";
+import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
+import { createRequire } from "node:module";
+import { tmpdir } from "node:os";
+import { dirname, join, relative, resolve } from "node:path";
+import { fileURLToPath } from "node:url";
+
+const HERE = dirname(fileURLToPath(import.meta.url));
+/** This repository's root, from `scripts/`. */
+const REPO_ROOT = resolve(HERE, "..");
+
+/** The rule every failure cites, so a red at 6pm needs no archaeology. */
+export const DOD_RULE =
+  ".kb/decisions/implement-loop-done-wired-and-reachable.md — every ratified DoD scenario " +
+  "must RUN and pass; 100%, no threshold, zero skipped";
+
+/** Every skip form a runner understands, as a literal source marker. */
+const SKIP_MARKERS = [
+  "test.fixme",
+  "it.fixme",
+  "test.skip",
+  "it.skip",
+  "describe.skip",
+  "suite.skip",
+  "test.todo",
+  "it.todo",
+  "describe.todo",
+  "xit",
+  "xdescribe",
+  "skipIf",
+];
+
+/**
+ * One scenario outcome, as the runner reported it.
+ *
+ * @typedef {object} ScenarioOutcome
+ * @property {string} scenario the `DoD-NN <slug>` title the case sits under
+ * @property {string} title the case title
+ * @property {string} status `passed` | `failed` | `pending` | `todo` | `skipped`
+ */
+
+/**
+ * One skip marker found in the suite's source.
+ *
+ * @typedef {object} SkipHit
+ * @property {string} file repo-relative path
+ * @property {number} line 1-indexed
+ * @property {string} marker the literal marker
+ * @property {string} scenario the enclosing `DoD-NN <slug>`, or `"(unknown scenario)"`
+ */
+
+/**
+ * The thirteen scenario titles the initiative charter ratifies, in order.
+ *
+ * READ FROM THE CHARTER, NEVER A CONSTANT HERE. A hard-coded list would drift from
+ * the ratified set silently, and EC-005 makes that drift a failure rather than a
+ * preference.
+ *
+ * @param {string} charter the contents of `.bklg/<initiative>/initiative.md`
+ * @returns {string[]} `["DoD-01 live-keeps-up", …]`
+ */
+export function expectedScenarios(charter) {
+  // Anchored with `^…$` in multiline mode rather than `indexOf("\n## …")`, so a
+  // charter whose section is the very first line still parses — the difference
+  // between "the ratified set is empty" and "I could not find it" is the whole
+  // point of EC-005, and a heading at offset 0 must not read as the former.
+  const heading = /^## Definition of Done\s*$/m.exec(charter);
+  if (heading === null) return [];
+  const rest = charter.slice(heading.index);
+  const end = rest.indexOf("\n## ", 1);
+  const section = end === -1 ? rest : rest.slice(0, end);
+  const out = [];
+  for (const m of section.matchAll(/^\s*(\d+)\.\s+\*\*@smoke\s+`([a-z0-9-]+)`\*\*/gm)) {
+    out.push(`DoD-${String(m[1]).padStart(2, "0")} ${m[2]}`);
+  }
+  return out;
+}
+
+/** A source file's executable half — block and line comments removed. */
+function codeOf(source) {
+  return source
+    .replace(/\/\*[\s\S]*?\*\//g, (m) => m.replace(/[^\n]/g, " "))
+    .replace(
+      /(^|[^:])\/\/.*$/gm,
+      (m, p1) => p1 + " ".repeat(Math.max(0, m.length - String(p1).length)),
+    );
+}
+
+/**
+ * Every literal skip marker reachable under `dir`, with its locator and the DoD
+ * scenario it sits inside.
+ *
+ * Comments are blanked (not deleted) before the scan, so prose ABOUT a marker —
+ * this file's own header, for instance — cannot trip it while line numbers stay
+ * true to the original source.
+ *
+ * @param {string} dir the suite directory
+ * @param {string} [root] the root locators are reported relative to
+ * @returns {SkipHit[]}
+ */
+export function scanForSkipMarkers(dir, root = dir) {
+  /** @type {SkipHit[]} */
+  const hits = [];
+  if (!existsSync(dir)) return hits;
+  /** @param {string} at */
+  const walk = (at) => {
+    for (const entry of readdirSync(at).sort()) {
+      const full = join(at, entry);
+      if (statSync(full).isDirectory()) {
+        walk(full);
+        continue;
+      }
+      if (!/\.(ts|mts|js|mjs)$/.test(entry)) continue;
+      const lines = codeOf(readFileSync(full, "utf8")).split(/\r?\n/);
+      let scenario = "(unknown scenario)";
+      lines.forEach((line, i) => {
+        const title = /describe\(\s*["'`](DoD-\d{2} [a-z0-9-]+)["'`]/.exec(line);
+        if (title !== null) scenario = String(title[1]);
+        for (const marker of SKIP_MARKERS) {
+          const re = new RegExp(`(^|[^\\w.])${marker.replace(".", "\\.")}\\s*[(<]`);
+          if (re.test(line)) {
+            hits.push({
+              file: relative(root, full).split("\\").join("/"),
+              line: i + 1,
+              marker,
+              scenario,
+            });
+          }
+        }
+      });
+    }
+  };
+  walk(dir);
+  return hits;
+}
+
+/**
+ * Flatten a vitest JSON (jest-compatible) payload into scenario outcomes.
+ *
+ * THROWS rather than returning an empty list when the payload is absent or
+ * unparseable: an unreadable result is exactly the state that must never be read
+ * as clean (AC-003 / EC-002).
+ *
+ * @param {string | undefined} raw
+ * @returns {ScenarioOutcome[]}
+ */
+export function parseResults(raw) {
+  if (raw === undefined || raw === null || String(raw).trim() === "") {
+    throw new Error(
+      "the DoD run produced no machine-readable result file — nothing ran, or the runner " +
+        "died before writing one. A guard that cannot read what it guards never reports clean.",
+    );
+  }
+  let payload;
+  try {
+    payload = JSON.parse(String(raw));
+  } catch (err) {
+    throw new Error(
+      `the DoD run's machine-readable result is unparseable JSON (${String(err)}). ` +
+        "An unparseable result is never treated as clean.",
+    );
+  }
+  const files = Array.isArray(payload.testResults) ? payload.testResults : [];
+  /** @type {ScenarioOutcome[]} */
+  const out = [];
+  for (const file of files) {
+    const cases = Array.isArray(file.assertionResults) ? file.assertionResults : [];
+    for (const c of cases) {
+      const ancestors = Array.isArray(c.ancestorTitles) ? c.ancestorTitles : [];
+      out.push({
+        scenario: String(ancestors[0] ?? "(no scenario)"),
+        title: String(c.title ?? ""),
+        status: String(c.status ?? "unknown"),
+      });
+    }
+  }
+  return out;
+}
+
+/**
+ * The verdict: given what was ratified, what ran, and what the source says.
+ *
+ * FAILS ON THE PRESENCE OF AN UNEXPECTED STATE rather than passing on the absence
+ * of a known bad one — "must be exactly the ratified set, exactly zero skipped"
+ * rather than "must not contain fixme" — so an evasion nobody has thought of trips
+ * it by default.
+ *
+ * @param {{expected: string[], observed: ScenarioOutcome[], markers: SkipHit[]}} input
+ * @returns {{ok: boolean, message: string}}
+ */
+export function evaluate({ expected, observed, markers }) {
+  /** @type {string[]} */
+  const problems = [];
+
+  if (expected.length !== 13) {
+    problems.push(
+      `the ratified Definition of Done declares ${String(expected.length)} scenarios, expected 13 — ` +
+        "the expectation has drifted from the charter. Amending the ratified set is a human " +
+        "decision recorded through `redkiln advance`, never a quiet edit here (EC-005).",
+    );
+  }
+
+  for (const hit of markers) {
+    problems.push(
+      `${hit.scenario} carries a skip marker \`${hit.marker}\` at ${hit.file}:${String(hit.line)} — ` +
+        "a skipped scenario reports skipped, exit 0, and is indistinguishable from one that passed.",
+    );
+  }
+
+  const seen = new Set(observed.map((o) => o.scenario));
+  const missing = expected.filter((slug) => !seen.has(slug));
+  const unexpected = [...seen].filter(
+    (slug) => !expected.includes(slug) && slug !== "(no scenario)",
+  );
+
+  if (observed.length === 0) {
+    problems.push(
+      "nothing ran: the DoD run reported zero scenario outcomes. An empty collection is a " +
+        "failure, never a pass — this instrument must never exit 0 having produced nothing.",
+    );
+  }
+  if (missing.length > 0) {
+    problems.push(
+      `these ratified DoD scenarios produced no outcome at all: ${missing.join(", ")} — a ` +
+        "runtime skip, a filtered invocation, an include glob that stopped matching, or a " +
+        "deleted suite file all look like this.",
+    );
+  }
+  if (unexpected.length > 0) {
+    problems.push(
+      `the run reported scenarios that are not in the ratified set: ${unexpected.join(", ")} — ` +
+        "a swapped or renamed scenario cannot balance the count.",
+    );
+  }
+  for (const outcome of observed) {
+    if (outcome.status !== "passed" && outcome.status !== "failed") {
+      problems.push(
+        `${outcome.scenario} > ${outcome.title} reported status \`${outcome.status}\` — a DoD ` +
+          "scenario has exactly two outcomes, pass or fail.",
+      );
+    }
+  }
+
+  const counted = new Set(observed.map((o) => o.scenario).filter((s) => expected.includes(s))).size;
+  if (problems.length === 0) {
+    return {
+      ok: true,
+      message:
+        `DoD tripwire: ${String(counted)} of ${String(expected.length)} ratified scenarios ran, ` +
+        `zero skipped — ${expected.join(", ")}`,
+    };
+  }
+  return {
+    ok: false,
+    message: [
+      "DoD tripwire: REFUSED.",
+      ...problems.map((p) => `  - ${p}`),
+      `  expected ${String(expected.length)} ratified scenarios, zero skipped; ` +
+        `observed ${String(counted)} scenario outcome(s).`,
+      `  rule: ${DOD_RULE}`,
+    ].join("\n"),
+  };
+}
+
+/**
+ * The runner config the DECLARED `test:dod` script names — never a second copy of
+ * that decision.
+ *
+ * The tripwire wraps the existing invocation (NF-003): it must run exactly the
+ * collector `pnpm test:dod` runs, so it reads `package.json`'s `scripts["test:dod"]`
+ * and takes the `--config` out of it. Hard-coding `vitest.dod.config.ts` here would
+ * let the declared script and the guarded run drift apart silently — and a guard
+ * pointed at a config nobody uses is the safeguard-in-prose failure again. A root
+ * with no such script (a scratch fixture under test) falls back to the conventional
+ * filename, and `main()` refuses if that does not exist either.
+ *
+ * @param {string} root
+ * @returns {string} absolute path to the runner config
+ */
+export function runnerConfigFor(root) {
+  const pkgPath = join(root, "package.json");
+  if (existsSync(pkgPath)) {
+    try {
+      const scripts = JSON.parse(readFileSync(pkgPath, "utf8")).scripts ?? {};
+      const declared = String(scripts["test:dod"] ?? "");
+      const named = /--config[= ]([^\s]+)/.exec(declared);
+      if (named !== null) return resolve(root, String(named[1]));
+    } catch {
+      // An unreadable package.json is not this guard's failure to diagnose; the
+      // conventional path below is refused loudly by `main()` if it is absent.
+    }
+  }
+  return join(root, "vitest.dod.config.ts");
+}
+
+/** `--root <dir>` off argv, defaulting to this script's repository. */
+function rootFromArgv(argv) {
+  const at = argv.indexOf("--root");
+  return at >= 0 && argv[at + 1] !== undefined ? resolve(String(argv[at + 1])) : REPO_ROOT;
+}
+
+/**
+ * Run the DoD suite ONCE, streaming its own output, and return the machine-readable
+ * result beside the child's exit status.
+ *
+ * The runner is resolved from THIS repository (`createRequire`), so a guarded root
+ * needs no `node_modules` of its own — which is what lets the focused test prove
+ * the wiring against a real scratch fixture instead of a hand-written payload.
+ */
+function runSuite(root, config, resultFile) {
+  const require = createRequire(import.meta.url);
+  const vitest = require.resolve("vitest/vitest.mjs");
+  const child = spawnSync(
+    process.execPath,
+    [
+      vitest,
+      "run",
+      "--config",
+      config,
+      "--reporter=default",
+      "--reporter=json",
+      `--outputFile.json=${resultFile}`,
+    ],
+    { cwd: root, stdio: ["ignore", "inherit", "inherit"] },
+  );
+  return child.status === null ? 1 : child.status;
+}
+
+function main() {
+  const root = rootFromArgv(process.argv.slice(2));
+  const charterPath = join(root, ".bklg", "live-views", "initiative.md");
+  const suiteDir = join(root, "test", "dod");
+  const configPath = runnerConfigFor(root);
+
+  for (const [what, path] of [
... TRUNCATED at 400 lines; 1 commits total
```
