# 15/20 — _archive/live-views

**kind:** initiative · **commits:** 74 · **files from commits:** 381 · **paths named only in the card:** 9

## Files changed (outside .bklg/)

- `CHANGELOG.md`
- `commands/closeout.md`
- `dist/cli.js`
- `dist/index.js`
- `src/process/engine.ts`
- `src/process/reverse.ts`
- `src/store/items.ts`
- `src/store/paths.ts`
- `src/store/read.ts`
- `src/store/verify.ts`
- `test/bklg-references.test.ts`
- `test/closeout-gate-integrity.test.ts`
- `test/kb-source-paths.test.ts`
- `test/store-read.test.ts`
- `workflows/forge-closeout.js`
- `test/capture-surfaces-contract.test.ts`
- `test/marks-guard.test.ts`
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

## Card body (the item as written)

```
## One-line intent

An author can leave the board open — or a terminal running — and trust what it says without
re-issuing a command, including when what it says is that a decision is waiting on them.

## Vision and narrative

The board today is a photograph. `redkiln view` reads the current state and writes a self-contained
artifact, and from that instant the artifact and the backlog drift apart with nothing on the surface
willing to admit it. The freshness problem people assume lives in the data layer is already solved —
`openFreshIndex` rebuilds on every call (`src/cli/index-access.ts`), so every invocation is honest at
the moment of invocation. What is missing is everything after that moment: presentation, lifecycle,
and the one hole in a read-only surface that watching a pipeline eventually forces you to notice.

The situation this bites hardest in is the one the board exists for. A `/redkiln:implement` run moves
items through stages for a long time with nobody watching closely. The only way to follow it is to
re-run the command and re-open the file — active labor to obtain a passive answer — and each answer
begins expiring the moment it is rendered. The author's real question is never "what did the backlog
look like when I last asked"; it is "is it still going, and does it need me yet."

Staleness has an honesty half that is easy to miss and impossible to retrofit. A board that updates
is only an improvement if it is legible about **when** it last updated, **whether** its last read
succeeded, and **whose** backlog it is showing. The craft literature is blunt about why: rendering
the last-received value is the identical code path whether the source is live or dead, so nothing
distinguishes current from fossil unless the surface does deliberate extra work to say so
(`_discovery/research/02-*`). The failure that follows has a name in every domain we surveyed —
silent death — and it is the single most-repeated complaint shape found across bundlers, watchers,
dashboards, and TUIs (`_discovery/research/03-*`). Pipeline runs happen on worktree branches, which
raises the stakes on one axis further: a board quietly showing the wrong branch's items is worse than
one that never refreshed, because the first misleads where the second merely ages.

The second half of the vision is the waiting room. HITL is the product default everywhere
(`docs/src/content/docs/explanation/design-principles.mdx`), so watching a pipeline is mostly waiting
to find out that it is waiting for you. The board has never been able to say that, and not for want
of data: `gateOpen` has been written by the engine since 0.12.0 (`src/process/engine.ts:1200`),
indexed (`src/index/build.ts:205,228`), and exposed on `BoardItem` (`src/index/queries.ts:232`) with
**zero readers** in either renderer. An answered question the board has never asked. Making it ask is
mostly a rendering job — with one catch that decides the true size of the work: the field exists only
at the leaf, while the pauses that dominate this pipeline are the project review gate and the
initiative design sign-off.

Once the board can say "this is waiting on you," the next thing an author wants is to answer it from
where they are standing. That turns a read surface into a write surface for exactly one decision, and
imports a constraint that is the hardest and most interesting problem in this initiative: an approve
control must not be reachable without the evidence it approves. The gate exists because a human is
supposed to read the spec, the review report, or the design mock before deciding. A one-click
approval on a surface that shows only status chips makes rubber-stamping *easier than* reading, which
inverts the purpose of the gate the feature is meant to serve. This is not a hypothetical: a
within-reviewer study of 11,429 agent-authored PR reviews measured approval rate climbing 30.1% →
36.8% (p<10⁻⁶) across a reviewer's first ten episodes while inline comments fell 22–28% and latency
rose 3.5× — and the same reviewers' approval rate on human-authored PRs *fell* over the identical
period (`_discovery/research/07-*`). Whatever is shown at decision time has to survive that curve,
not merely look sufficient on day one.

Success looks like this. An author starts a run, opens the board once, and walks away. They glance
back from across the room and know in under a second whether it is live-and-current, idle-but-current,
or degraded — without decoding a timestamp. Whatever they had scrolled to or expanded is still where
they left it. The branch scope is stated, permanently, not inferred. When the run reaches a human
gate, the board says so at whatever level of the tree they happen to be looking at, and when they sit
down to decide, the thing the gate exists to make them read is in front of them at the moment they
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
a7afc9d0 fix(archive): one archive resolver, three consumers

diff --git a/CHANGELOG.md b/CHANGELOG.md
index d8e9b9e9..f6947edb 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -84,6 +84,37 @@ in-flight work on the pinned version first.
 
 ### Fixed
 
+- **An archived id now says it is archived, instead of saying it does not exist.**
+  Closeout moves an initiative into `.bklg/_archive/`, which takes its items out of
+  the live store — so every id under it became unresolvable the moment its
+  initiative closed, and `advance`/`verify`/`new --under` all answered
+  `item 'RK-P0011' does not exist`. That is false, and it sends an operator looking
+  for a drifted id that does not exist either. They now name the archived path and
+  say plainly that nothing needs repair. Behaviour is unchanged: an archived item
+  still cannot be advanced, verified, or made a parent — only the message changed.
+
+- **Closeout stops certifying link reconciliation it cannot see, and an unresolved
+  reference now blocks the close** (#97). The sweep and its own verifier both used
+  `rg "<slug>/"`, which matches only a path written as one literal string; paths
+  composed segment-by-segment (`join(ROOT, ".bklg", "<slug>", …)`) were invisible to
+  both, and in this repo they OUTNUMBER the literal form roughly three to one. So
+  the verifier was blind in exactly the way the sweep was blind, and reported
+  `pass: true` over a tree where most references had not been rewritten. Both now
+  search all three written forms; code references are REPORTED rather than
+  rewritten, because a source path repointed by an agent mid-closeout is a code
+  change nobody reviewed. And `linkReconciliation.unresolved` is now machine-
+  blocking — it sets `ready: false` and its entries appear in `gaps`. It was
+  previously a line of prose asking a human to notice.
+
+- **A reference into an archived initiative resolves instead of 404ing.**
+  `.bklg/<slug>/…` now resolves at `.bklg/_archive/<slug>/…` when the initiative has
+  been closed out, through one shared resolver rather than per-reader guesswork.
+  This is what makes the accepted-decision citations in `.kb/` survive a closeout:
+  their bodies are frozen by `validate --kb`, so repointing them in place is refused
+  — the pre-move path is the only thing a correct atom can say, and recognising the
+  rename is what keeps it true. Deliberately narrow: only when the cited path does
+  not exist, only when the archived one does. A typo is still a typo.
+
 - **A resolver that could not answer used to answer "pass"** — the I2 invariant
   of the 2026-08-22 recovery audit's remediation. Three facts were one fact:
   "nothing was declared for this check to measure", "it ran and its subject did
diff --git a/commands/closeout.md b/commands/closeout.md
index 1d25a746..c2b53a06 100644
--- a/commands/closeout.md
+++ b/commands/closeout.md
@@ -172,7 +172,11 @@ produce that verdict and both are stops: the audit found the initiative not clos
 5. **On return — relay the manifest.** Summarize, in order: the **knowledge-harvest report**
    (`kbHarvest.atoms` by kind — concept/decision/playbook — noting any `supersedes`, plus any
    `kbHarvest.gaps`), the **link-reconciliation report** (`linkReconciliation.rewrites` count by bucket
-   — `kb` vs `archive` — plus any `unresolved`; treat `unresolved` as a blocker to flag, like `gaps`),
+   — `kb` vs `archive` — plus any `unresolved`, which is now a REAL blocker: a non-empty `unresolved`
+   makes `ready` false and its entries appear in `gaps`, so there is nothing here for you to remember
+   to treat as one. It used to be prose asking you to notice, in a manifest with a dozen other fields,
+   at the moment you were being told the audit passed — and a sweep that could not rewrite a reference
+   returned `ready: true` and the close proceeded over it (#97)),
    the `retrospectivePath` and `closurePath`, the **reconcile plan** (ordered steps with risk tags), and
    the **release plan** (whether a changeset is needed, target version + channel, the ordered steps). Note
    the workflow's `verify` verdict; if it failed, the manifest already returned `ready: false` — surface
diff --git a/dist/cli.js b/dist/cli.js
index 31ccf21c..712320ee 100644
--- a/dist/cli.js
+++ b/dist/cli.js
@@ -13683,11 +13683,47 @@ function locateItemIn(root, store, id) {
 function locateItem(root, id) {
   return locateItemIn(root, loadStore(root), id);
 }
-function unresolvedReason(store, id) {
+function findArchivedItem(root, id) {
+  const archive = join2(targetPath(root, TARGET.bklg), ARCHIVE_DIRNAME);
+  if (!existsSync2(archive)) return null;
+  const stack = [archive];
+  while (stack.length > 0) {
+    const dir = stack.pop();
+    let entries;
+    try {
+      entries = readdirSync(dir, { withFileTypes: true });
+    } catch {
+      continue;
+    }
+    for (const ent of entries) {
+      const abs = join2(dir, ent.name);
+      if (ent.isDirectory()) {
+        stack.push(abs);
+        continue;
+      }
+      if (!isCanonicalItemFile(ent.name)) continue;
+      let raw;
+      try {
+        raw = readFileSync(abs, "utf8");
+      } catch {
+        continue;
+      }
+      if (new RegExp(`^id:\\s*['"]?${id}['"]?\\s*$`, "m").test(raw)) {
+        return relative(root, abs).split("\\").join("/");
+      }
+    }
+  }
+  return null;
+}
+function unresolvedReason(store, id, root) {
   const own = store.errors.find((e) => e.claimedId === id);
   if (own !== void 0) {
     return `item '${id}' could not be loaded from ${own.path}: ${own.reason}`;
   }
+  const archived = root === void 0 ? null : findArchivedItem(root, id);
+  if (archived !== null) {
+    return `item '${id}' is ARCHIVED at ${archived} \u2014 it exists, and it is not part of the live store, so it cannot be advanced, verified, or made a parent. Nothing has drifted and nothing needs repair: its initiative was closed out`;
+  }
   if (store.errors.length === 0) return `item '${id}' does not exist`;
   return `item '${id}' does not exist (${store.errors.length} item file(s) could not be loaded and were skipped \u2014 run \`redkiln validate\` to see them; the id may be in one of those)`;
 }
@@ -16698,7 +16734,7 @@ function runVerify(opts) {
   const store = loadStore(root);
   const located = locateItemIn(root, store, item);
   if (located === void 0) {
-    throw new VerifyError(unresolvedReason(store, item));
+    throw new VerifyError(unresolvedReason(store, item, root));
   }
   const read = readRepoConfig(root);
   if (read.kind === "malformed") {
@@ -17496,7 +17532,7 @@ function advanceUnlocked(id, opts) {
   const store = loadStore(root);
   const located = locateItemIn(root, store, id);
   if (located === void 0) {
-    throw new AdvanceError(unresolvedReason(store, id));
+    throw new AdvanceError(unresolvedReason(store, id, root));
   }
   const item = ItemFrontmatter.parse(located.data);
   const proc = loadProcess(root, item.process);
@@ -18830,7 +18866,7 @@ function runReverse(id, kind, opts, plan) {
   const { root } = opts;
   const store = loadStore(root);
   const located = locateItemIn(root, store, id);
-  if (located === void 0) throw new ReverseError(unresolvedReason(store, id));
+  if (located === void 0) throw new ReverseError(unresolvedReason(store, id, root));
   const item = ItemFrontmatter.parse(located.data);
   const proc = loadProcess(root, item.process);
   const currentIndex = proc.stages.findIndex((s) => s.id === item.stage);
@@ -22681,7 +22717,7 @@ function resolvePlacement(opts, supportInitiative) {
   const store = loadStore(root);
   const parent = locateItemIn(root, store, opts.under);
   if (parent === void 0) {
-    throw new ItemCreateError(`parent: ${unresolvedReason(store, opts.under)}`);
+    throw new ItemCreateError(`parent: ${unresolvedReason(store, opts.under, root)}`);
   }
   const parentType = parent.data.type;
   if (parentType !== expectedParentType) {
diff --git a/dist/index.js b/dist/index.js
index 9df9594b..324468e5 100644
--- a/dist/index.js
+++ b/dist/index.js
@@ -13683,11 +13683,47 @@ function locateItemIn(root, store, id) {
 function locateItem(root, id) {
   return locateItemIn(root, loadStore(root), id);
 }
-function unresolvedReason(store, id) {
+function findArchivedItem(root, id) {
+  const archive = join2(targetPath(root, TARGET.bklg), ARCHIVE_DIRNAME);
+  if (!existsSync2(archive)) return null;
+  const stack = [archive];
+  while (stack.length > 0) {
+    const dir = stack.pop();
+    let entries;
+    try {
+      entries = readdirSync(dir, { withFileTypes: true });
+    } catch {
+      continue;
+    }
+    for (const ent of entries) {
+      const abs = join2(dir, ent.name);
+      if (ent.isDirectory()) {
+        stack.push(abs);
+        continue;
+      }
+      if (!isCanonicalItemFile(ent.name)) continue;
+      let raw;
+      try {
+        raw = readFileSync(abs, "utf8");
+      } catch {
+        continue;
+      }
+      if (new RegExp(`^id:\\s*['"]?${id}['"]?\\s*$`, "m").test(raw)) {
+        return relative(root, abs).split("\\").join("/");
+      }
+    }
+  }
+  return null;
+}
+function unresolvedReason(store, id, root) {
   const own = store.errors.find((e) => e.claimedId === id);
   if (own !== void 0) {
     return `item '${id}' could not be loaded from ${own.path}: ${own.reason}`;
   }
+  const archived = root === void 0 ? null : findArchivedItem(root, id);
+  if (archived !== null) {
+    return `item '${id}' is ARCHIVED at ${archived} \u2014 it exists, and it is not part of the live store, so it cannot be advanced, verified, or made a parent. Nothing has drifted and nothing needs repair: its initiative was closed out`;
+  }
   if (store.errors.length === 0) return `item '${id}' does not exist`;
   return `item '${id}' does not exist (${store.errors.length} item file(s) could not be loaded and were skipped \u2014 run \`redkiln validate\` to see them; the id may be in one of those)`;
 }
@@ -16698,7 +16734,7 @@ function runVerify(opts) {
   const store = loadStore(root);
   const located = locateItemIn(root, store, item);
   if (located === void 0) {
-    throw new VerifyError(unresolvedReason(store, item));
+    throw new VerifyError(unresolvedReason(store, item, root));
   }
   const read = readRepoConfig(root);
   if (read.kind === "malformed") {
@@ -17496,7 +17532,7 @@ function advanceUnlocked(id, opts) {
   const store = loadStore(root);
   const located = locateItemIn(root, store, id);
   if (located === void 0) {
-    throw new AdvanceError(unresolvedReason(store, id));
+    throw new AdvanceError(unresolvedReason(store, id, root));
   }
   const item = ItemFrontmatter.parse(located.data);
   const proc = loadProcess(root, item.process);
@@ -18830,7 +18866,7 @@ function runReverse(id, kind, opts, plan) {
   const { root } = opts;
   const store = loadStore(root);
   const located = locateItemIn(root, store, id);
-  if (located === void 0) throw new ReverseError(unresolvedReason(store, id));
+  if (located === void 0) throw new ReverseError(unresolvedReason(store, id, root));
   const item = ItemFrontmatter.parse(located.data);
   const proc = loadProcess(root, item.process);
   const currentIndex = proc.stages.findIndex((s) => s.id === item.stage);
@@ -22681,7 +22717,7 @@ function resolvePlacement(opts, supportInitiative) {
   const store = loadStore(root);
   const parent = locateItemIn(root, store, opts.under);
   if (parent === void 0) {
-    throw new ItemCreateError(`parent: ${unresolvedReason(store, opts.under)}`);
+    throw new ItemCreateError(`parent: ${unresolvedReason(store, opts.under, root)}`);
   }
   const parentType = parent.data.type;
   if (parentType !== expectedParentType) {
diff --git a/src/process/engine.ts b/src/process/engine.ts
index 7f0b918a..a5e0bf61 100644
--- a/src/process/engine.ts
+++ b/src/process/engine.ts
@@ -1403,7 +1403,7 @@ function advanceUnlocked(id: string, opts: AdvanceOptions): AdvanceResult {
   const store = loadStore(root);
   const located = locateItemIn(root, store, id);
   if (located === undefined) {
-    throw new AdvanceError(unresolvedReason(store, id));
+    throw new AdvanceError(unresolvedReason(store, id, root));
   }
 
   const item = ItemFrontmatter.parse(located.data);
diff --git a/src/process/reverse.ts b/src/process/reverse.ts
index 9e009f19..22c9f3b8 100644
--- a/src/process/reverse.ts
+++ b/src/process/reverse.ts
@@ -284,7 +284,7 @@ function runReverse(
   const { root } = opts;
   const store = loadStore(root);
   const located = locateItemIn(root, store, id);
-  if (located === undefined) throw new ReverseError(unresolvedReason(store, id));
+  if (located === undefined) throw new ReverseError(unresolvedReason(store, id, root));
 
   const item = ItemFrontmatter.parse(located.data);
   const proc = loadProcess(root, item.process);
diff --git a/src/store/items.ts b/src/store/items.ts
index 9eb35ad9..a7f7163a 100644
--- a/src/store/items.ts
+++ b/src/store/items.ts
@@ -271,7 +271,7 @@ function resolvePlacement(opts: CreateItemOptions, supportInitiative: string): P
   const store = loadStore(root);
   const parent = locateItemIn(root, store, opts.under);
   if (parent === undefined) {
-    throw new ItemCreateError(`parent: ${unresolvedReason(store, opts.under)}`);
+    throw new ItemCreateError(`parent: ${unresolvedReason(store, opts.under, root)}`);
   }
   const parentType = parent.data.type;
   if (parentType !== expectedParentType) {
diff --git a/src/store/paths.ts b/src/store/paths.ts
index 5e7a732e..7b949611 100644
--- a/src/store/paths.ts
+++ b/src/store/paths.ts
@@ -147,3 +147,49 @@ export const ARCHIVE_DIRNAME = "_archive";
 export function isArchivedPath(pathLike: string): boolean {
   return pathLike.split(/[\\/]/).includes(ARCHIVE_DIRNAME);
 }
+
+/**
+ * Where closeout moved `rel`, if it moved it: `.bklg/<slug>/…` → `.bklg/_archive/<slug>/…`.
+ * Returns `null` when the mapping does not apply. PURE — it resolves nothing on disk.
+ *
+ * WHY THE MAPPING IS OWNED HERE RATHER THAN BY EACH READER. Closeout renames
+ * `.bklg/<slug>/` to `.bklg/_archive/<slug>/`, and every reference written while
+ * the initiative was live keeps naming the pre-move path. That is not a defect in
+ * the reference — for a citation inside an ACCEPTED KB decision it is the ONLY
+ * thing a correct atom can say, because `validate-kb` folds the body into the
+ * frozen signature and refuses the in-place edit that would repoint it. So the
+ * choice is either to recognise the rename once, here, or to re-break every
+ * reference on every closeout and repair them by hand afterwards. On this repo
+ * that is 39 of 47 KB atoms citing an archived path, and it recurs per initiative.
+ *
+ * It was written, argued and shipped as a LOCAL HELPER INSIDE ONE TEST
+ * (`test/kb-source-paths.test.ts`), where only that test could use it — while
+ * `redkiln`'s own id resolution, its closeout link sweep and that sweep's verifier
+ * each went on being blind to the same rename. This is that helper, promoted, with
+ * its reasoning intact.
+ *
+ * NARROW ON PURPOSE, and the narrowness is the whole safety argument:
+ *
+ *   - ONLY `.bklg/<slug>/…`. A path that does not start under `.bklg` is not
+ *     something closeout moves.
+ *   - NEVER a path already under `_archive`. Otherwise the mapping composes with
+ *     itself and invents `.bklg/_archive/_archive/…`.
+ *   - The slug itself counts. `.bklg/<slug>` with no child is the initiative
+ *     folder, and it is PRECISELY what moves — the command is
+ *     `git mv .bklg/<slug> .bklg/_archive/<slug>`. An earlier draft required a
+ *     segment after the slug and so failed to redirect the one path the move is
+ *     literally named after; a real reference to `.bklg/live-views` found it.
+ *   - `.bklg` alone is not a reference to anything that moves.
+ *
+ * IT DOES NOT LOOK AT DISK, so it cannot decide whether the answer is right — the
+ * caller must check, and every caller in this repo does. A path that resolves in
+ * NEITHER place is still an error, and a typo is still a typo: this widens where a
+ * reference may be found, never whether it must be found at all.
+ */
+export function archivedTwin(rel: string): string | null {
+  const segs = rel.split(/[\\/]/);
+  if (segs[0] !== TARGET.bklg) return null;
+  if (segs.length < 2) return null;
+  if (segs[1] === ARCHIVE_DIRNAME) return null;
+  return [TARGET.bklg, ARCHIVE_DIRNAME, ...segs.slice(1)].join("/");
+}
diff --git a/src/store/read.ts b/src/store/read.ts
index 035fca76..b2d46a73 100644
--- a/src/store/read.ts
+++ b/src/store/read.ts
@@ -337,7 +337,55 @@ export function locateItem(root: string, id: string): LocatedItem | undefined {
 }
 
 /**
- * Explain an unresolvable id. Three cases, and they are genuinely different:
+ * Find an ARCHIVED item's canonical file by id, or `null`.
+ *
+ * ONLY EVER CALLED ON A PATH THAT HAS ALREADY FAILED, which is what makes the
+ * cost acceptable: the live store is an in-memory map lookup, and this walk runs
+ * exclusively when that lookup returned nothing and an error message is being
+ * composed. It is bounded to canonical item files under `.bklg/_archive` (1,312
+ * files here, 2,555 in the largest corpus) and it never runs on a resolving id.
+ *
+ * Frontmatter is read with the tolerant regex fallback rather than the schema:
+ * archived records need not satisfy the CURRENT `ItemFrontmatter` (they may
+ * predate it, or come from another tool — that is the stated reason the archive
+ * is excluded from the store at all), so a strict parse would throw on exactly
+ * the old items this exists to find.
+ */
+function findArchivedItem(root: string, id: string): string | null {
+  const archive = join(targetPath(root, TARGET.bklg), ARCHIVE_DIRNAME);
+  if (!existsSync(archive)) return null;
+  const stack = [archive];
+  while (stack.length > 0) {
+    const dir = stack.pop() as string;
+    let entries;
+    try {
+      entries = readdirSync(dir, { withFileTypes: true });
+    } catch {
+      continue; // an unreadable archive directory is not this function's to report
+    }
+    for (const ent of entries) {
+      const abs = join(dir, ent.name);
+      if (ent.isDirectory()) {
+        stack.push(abs);
+        continue;
+      }
+      if (!isCanonicalItemFile(ent.name)) continue;
+      let raw: string;
+      try {
+        raw = readFileSync(abs, "utf8");
+      } catch {
+        continue;
+      }
+      if (new RegExp(`^id:\\s*['"]?${id}['"]?\\s*$`, "m").test(raw)) {
+        return relative(root, abs).split("\\").join("/");
+      }
+    }
+  }
+  return null;
+}
+
+/**
+ * Explain an unresolvable id. FOUR cases, and they are genuinely different:
  *
  *  1. The id's OWN file is on disk and failed to load ⇒ name it and say why.
  *     "does not exist" here is not merely unhelpful, it is false, and it sends
@@ -345,13 +393,37 @@ export function locateItem(root: string, id: string): LocatedItem | undefined {
  *     is rejected at the schema boundary now, so the item that carries it is
  *     absent from the store rather than reaching the runtime guard.
  *  2. Something ELSE failed to load ⇒ it might be the one, so say so.
- *  3. The store read cleanly ⇒ the id really does not exist; add nothing.
+ *  3. The item EXISTS, ARCHIVED ⇒ say where it went. See below.
+ *  4. The store read cleanly and the archive does not hold it ⇒ the id really
... TRUNCATED at 400 lines; 74 commits total
```
