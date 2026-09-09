# 17/20 — support

**kind:** initiative · **commits:** 72 · **files from commits:** 30 · **paths named only in the card:** 0

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
- `.redkiln/telemetry/events/ryan-britton@redkiln.jsonl`
- `src/store/project.ts`
- `src/store/validate-bklg.ts`
- `test/ledger-sed-detection.test.ts`
- `src/store/verify-cache.ts`
- `test/gate-memo-and-artifacts.test.ts`
- `test/brand-tokens.test.ts`
- `test/capture-surfaces-contract.test.ts`
- `.redkiln/telemetry/events/ryan-britton@fix-serve-child-leak.jsonl`
- `.redkiln/telemetry/events/ryanm@fix-serve-child-leak.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-gate-honesty-sweep.jsonl`
- `.github/workflows/ci.yml`
- `.redkiln/telemetry/events/ryan-britton@live-views.jsonl`
- `src/index/db.ts`
- `.redkiln/telemetry/events/ryanm@live-views.jsonl`

## Card body (the item as written)

```
## Summary

The strategic outcome this initiative pursues and why it matters now.

## Outcomes

The durable changes this initiative is meant to produce.

## Projects

The projects decomposed from this initiative (see `_decomposition`).

## Notes

Bets, constraints, and references.
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
... TRUNCATED at 400 lines; 72 commits total
```
