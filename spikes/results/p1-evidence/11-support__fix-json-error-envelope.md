# 11/20 — support/fix-json-error-envelope

**kind:** project · **commits:** 1 · **files from commits:** 18 · **paths named only in the card:** 0

## Files changed (outside .bklg/)

- `.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity@runs.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-gate-honesty-sweep@runs.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-issue-146-147-story-gate@runs.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-json-error-envelope.jsonl`
- `.redkiln/telemetry/events/ryan-britton@fix-serve-child-leak@runs.jsonl`
- `.redkiln/telemetry/events/ryan-britton@issue-backlog-sweep-lane@runs.jsonl`
- `.redkiln/telemetry/events/ryan-britton@redkiln.jsonl`
- `.redkiln/telemetry/events/ryanm@fix-json-error-envelope.jsonl`
- `dist/cli.js`
- `dist/index.js`
- `src/cli/advance.ts`
- `src/cli/json-error.ts`
- `src/cli/link.ts`
- `src/cli/new.ts`
- `src/cli/record-links.ts`
- `src/cli/record-run.ts`
- `src/cli/run-project.ts`
- `test/json-error-envelope.test.ts`

## Card body (the item as written)

```
## Summary

The coherent body of work this project delivers and how it advances its initiative.

## Goals

What "done" means for this project.

## Stories

The mergeable units of work decomposed from this project (see `_storymap`).

## Notes

Risks, dependencies, and references.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
cf5dbd07 fix(cli): emit a JSON error envelope on every --json failure path (#149, #116) (#158)

diff --git a/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity@runs.jsonl b/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity@runs.jsonl
index bb9efc65..507d4e12 100644
--- a/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity@runs.jsonl
+++ b/.redkiln/telemetry/events/ryan-britton@docs-site-brand-identity@runs.jsonl
@@ -29,3 +29,6 @@
 {"ts":"2026-08-12T03:33:35.397Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0024","type":"project","initiative":"docs-site-brand-identity","project":"kb-harvest-and-closeout","process":"project","stage":"implementation","tokens_out":589545,"workflow":"implement","result":"halted:stories","agents":14,"degraded":1,"tier":"standard","sha":"d2745c78","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":14503},{"phase":"Stories","agents":13,"retried":0,"failed":0,"outTokens":575042}],"models":{"sonnet":4,"opus":7,"haiku":3},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
 {"ts":"2026-08-12T07:00:14.819Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0024","type":"project","initiative":"docs-site-brand-identity","project":"kb-harvest-and-closeout","process":"project","stage":"implementation","tokens_out":416935,"workflow":"implement","result":"halted:stories","agents":11,"degraded":1,"tier":"standard","sha":"f3e77917","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":15355},{"phase":"Stories","agents":10,"retried":0,"failed":0,"outTokens":401580}],"models":{"sonnet":3,"opus":5,"haiku":3},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
 {"ts":"2026-08-12T15:40:48.064Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0024","type":"project","initiative":"docs-site-brand-identity","project":"kb-harvest-and-closeout","process":"project","stage":"implementation","tokens_out":317062,"workflow":"implement","result":"halted:integration","agents":7,"degraded":1,"tier":"standard","sha":"0df64280","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":16281},{"phase":"Stories","agents":5,"retried":0,"failed":0,"outTokens":142473},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":158308}],"models":{"sonnet":2,"opus":3,"haiku":2},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-12T20:53:02.788Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-P0024","type":"project","initiative":"docs-site-brand-identity","project":"kb-harvest-and-closeout","process":"project","stage":"implementation","tokens_out":335413,"workflow":"implement","result":"completed:fixed","agents":5,"degraded":0,"score":3,"tier":"standard","sha":"a9c0c7e7","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":12547},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":147303},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":2877},{"phase":"Project review","agents":2,"retried":0,"failed":0,"outTokens":172686},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":4},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-13T06:27:18.334Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-I0005","type":"initiative","initiative":"docs-site-brand-identity","project":null,"process":"initiative","stage":"implementation","tokens_out":394433,"workflow":"closeout","result":"completed:unverified","agents":16,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Audit","agents":8,"retried":0,"failed":0,"outTokens":52082},{"phase":"Knowledge harvest","agents":1,"retried":0,"failed":0,"outTokens":48968},{"phase":"Retrospective","agents":1,"retried":0,"failed":0,"outTokens":65456},{"phase":"Closure","agents":1,"retried":0,"failed":0},{"phase":"Link reconciliation","agents":1,"retried":0,"failed":0,"outTokens":83128},{"phase":"Reconcile plan","agents":1,"retried":0,"failed":0,"outTokens":32649},{"phase":"Release plan","agents":1,"retried":0,"failed":0},{"phase":"Verify","agents":2,"retried":0,"failed":0,"outTokens":112150}],"models":{"sonnet":15,"opus":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-13T15:13:39.489Z","actor":"ryan-britton","worktree":"docs-site-brand-identity","branch":"initiative/docs-site-brand-identity","event":"workflow_run","item":"RK-I0005","type":"initiative","initiative":"docs-site-brand-identity","project":null,"process":"initiative","stage":"implementation","tokens_out":63129,"workflow":"closeout","result":"halted:audit-not-ready","agents":8,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Audit","agents":8,"retried":0,"failed":0,"outTokens":63129}],"models":{"sonnet":8},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null,"gaps":["working tree not clean (gitClean is not true)"],"ready":false}}
diff --git a/.redkiln/telemetry/events/ryan-britton@fix-gate-honesty-sweep@runs.jsonl b/.redkiln/telemetry/events/ryan-britton@fix-gate-honesty-sweep@runs.jsonl
new file mode 100644
index 00000000..bf0a4625
--- /dev/null
+++ b/.redkiln/telemetry/events/ryan-britton@fix-gate-honesty-sweep@runs.jsonl
@@ -0,0 +1,6 @@
+{"ts":"2026-08-18T20:35:24.740Z","actor":"ryan-britton","worktree":"fix-gate-honesty-sweep","branch":"fix/gate-honesty-sweep","event":"workflow_run","item":"RK-S0267","type":"story","initiative":"support","project":"fix-gate-honesty-sweep","process":"story-lite","stage":"implement","tokens_out":66367,"workflow":"fix","result":"halted:verify","agents":4,"degraded":0,"tier":"small","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":2814},{"phase":"Fix","agents":3,"retried":0,"failed":0,"outTokens":63553}],"models":{"sonnet":2,"opus":1,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-18T21:20:16.552Z","actor":"ryan-britton","worktree":"fix-gate-honesty-sweep","branch":"fix/gate-honesty-sweep","event":"workflow_run","item":"RK-S0267","type":"story","initiative":"support","project":"fix-gate-honesty-sweep","process":"story-lite","stage":"implement","tokens_out":80578,"workflow":"fix","result":"committed","agents":6,"degraded":0,"score":3,"tier":"small","sha":"2a01a731","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":2897},{"phase":"Fix","agents":4,"retried":0,"failed":0,"outTokens":68690},{"phase":"Review","agents":1,"retried":0,"failed":0,"outTokens":8991}],"models":{"sonnet":4,"opus":1,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-18T23:26:29.052Z","actor":"ryan-britton","worktree":"fix-gate-honesty-sweep","branch":"fix/gate-honesty-sweep","event":"workflow_run","item":"RK-S0268","type":"story","initiative":"support","project":"fix-gate-honesty-sweep","process":"story-lite","stage":"implement","tokens_out":125309,"workflow":"fix","result":"committed","agents":6,"degraded":0,"score":3,"tier":"small","sha":"d9397179","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":3319},{"phase":"Fix","agents":4,"retried":0,"failed":0,"outTokens":110507},{"phase":"Review","agents":1,"retried":0,"failed":0,"outTokens":11483}],"models":{"sonnet":4,"opus":1,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-19T00:41:44.571Z","actor":"ryan-britton","worktree":"fix-gate-honesty-sweep","branch":"fix/gate-honesty-sweep","event":"workflow_run","item":"RK-S0269","type":"story","initiative":"support","project":"fix-gate-honesty-sweep","process":"story-lite","stage":"implement","tokens_out":128315,"workflow":"fix","result":"committed","agents":6,"degraded":0,"score":3,"tier":"small","sha":"19c836dd","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":3698},{"phase":"Fix","agents":4,"retried":0,"failed":0,"outTokens":117794},{"phase":"Review","agents":1,"retried":0,"failed":0,"outTokens":6823}],"models":{"sonnet":4,"opus":1,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-19T03:42:10.622Z","actor":"ryan-britton","worktree":"fix-gate-honesty-sweep","branch":"fix/gate-honesty-sweep","event":"workflow_run","item":"RK-S0270","type":"story","initiative":"support","project":"fix-gate-honesty-sweep","process":"story-lite","stage":"implement","tokens_out":92910,"workflow":"fix","result":"halted:verify","agents":4,"degraded":0,"tier":"small","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":7296},{"phase":"Fix","agents":3,"retried":0,"failed":0,"outTokens":85614}],"models":{"sonnet":2,"opus":1,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-19T06:04:50.892Z","actor":"ryan-britton","worktree":"fix-gate-honesty-sweep","branch":"fix/gate-honesty-sweep","event":"workflow_run","item":"RK-S0271","type":"story","initiative":"support","project":"fix-gate-honesty-sweep","process":"story-lite","stage":"implement","tokens_out":117186,"workflow":"fix","result":"committed","agents":6,"degraded":0,"score":3,"tier":"small","sha":"1e1955e5","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":4510},{"phase":"Fix","agents":4,"retried":0,"failed":0,"outTokens":103595},{"phase":"Review","agents":1,"retried":0,"failed":0,"outTokens":9081}],"models":{"sonnet":4,"opus":1,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
diff --git a/.redkiln/telemetry/events/ryan-britton@fix-issue-146-147-story-gate@runs.jsonl b/.redkiln/telemetry/events/ryan-britton@fix-issue-146-147-story-gate@runs.jsonl
new file mode 100644
index 00000000..0e972043
--- /dev/null
+++ b/.redkiln/telemetry/events/ryan-britton@fix-issue-146-147-story-gate@runs.jsonl
@@ -0,0 +1,2 @@
+{"ts":"2026-08-20T20:43:09.903Z","actor":"ryan-britton","worktree":"fix-issue-146-147-story-gate","branch":"fix/issue-146-147-story-gate","event":"workflow_run","item":"RK-S0273","type":"story","initiative":"support","project":"issue-146-147-story-gate","process":"story-lite","stage":"implement","tokens_out":210672,"workflow":"fix","result":"halted:review","agents":4,"degraded":0,"score":2,"tier":"small","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":2764},{"phase":"Fix","agents":3,"retried":0,"failed":0,"outTokens":207908}],"models":{"sonnet":2,"opus":2},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-21T00:47:52.110Z","actor":"ryan-britton","worktree":"fix-issue-146-147-story-gate","branch":"fix/issue-146-147-story-gate","event":"workflow_run","item":"RK-S0273","type":"story","initiative":"support","project":"issue-146-147-story-gate","process":"story-lite","stage":"implement","tokens_out":259149,"workflow":"fix","result":"committed","agents":7,"degraded":0,"score":3,"tier":"small","sha":"f21f55fc","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":10352},{"phase":"Fix","agents":5,"retried":0,"failed":0,"outTokens":230300},{"phase":"Review","agents":1,"retried":0,"failed":0,"outTokens":18497}],"models":{"sonnet":4,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
diff --git a/.redkiln/telemetry/events/ryan-britton@fix-json-error-envelope.jsonl b/.redkiln/telemetry/events/ryan-britton@fix-json-error-envelope.jsonl
new file mode 100644
index 00000000..b620e821
--- /dev/null
+++ b/.redkiln/telemetry/events/ryan-britton@fix-json-error-envelope.jsonl
@@ -0,0 +1,15 @@
+{"ts":"2026-08-22T04:56:50.433Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"item_created","item":"RK-P0043","type":"project"}
+{"ts":"2026-08-22T04:59:15.208Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"item_created","item":"RK-S0274","type":"story"}
+{"ts":"2026-08-22T04:59:44.139Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"stage_exit","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"intake","outcome":"completed"}
+{"ts":"2026-08-22T04:59:44.268Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"stage_enter","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"spec"}
+{"ts":"2026-08-22T05:02:42.168Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"item_edited","item":"RK-S0274"}
+{"ts":"2026-08-22T05:07:30.145Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"verdict","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"spec","pass":true,"note":"approved"}
+{"ts":"2026-08-22T05:18:43.584Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"gate_result","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"implement","outcome":"halted:gate","gate":"command","pass":false,"note":"command gate failed (EC-003): `{{redkiln}} verify --item {{id}} --grain story` — affected-gate (`pnpm typecheck && pnpm lint && pnpm test` exited non-zero on two consecutive runs; failing: tests/a.spec.ts:3:1 › nope, test/lockup-outliner.test.ts [ test/lockup-outliner.test.ts ], test/og-card-composition.test.ts [ test/og-card-composition.test.ts ], test/og-card-cut-declaration.test.ts [ test/og-card-cut-declaration.test.ts ], test/og-card-share.test.ts [ test/og-card-share.test.ts ], test/typography-delivery.test.ts > built and deployed artifact (RK-S0222), test/capture-surfaces.test.ts > AC-013: every surface this project declares can actually be captured > resolves both ids without the bench treating them as unknown, test/marks-guard.test.ts > capture bench: RK-P0021's six declared surfaces have real producers > renders real, differentiated documents for every surface and state (one build, --no-screenshot), test/og-card-build.test.ts > AC-001: a real raster exists at exactly 1200 x 630 > emits a PNG the manifest names, test/og-card-build.test.ts > AC-001: a real raster exists at exactly 1200 x 630 > is exactly the box every platform reserves, test/og-card-build.test.ts > AC-002: the emitted card carries real composed presentation > is not a stub-sized artifact, test/og-card-build.test.ts > AC-002: the emitted card carries real composed presentation > carries more than one distinct pixel value, test/og-card-build.test.ts > AC-002: the emitted card carries real composed presentation > puts a substantial non-ground region on the canvas, test/og-card-build.test.ts > AC-003: the content box is the placement a human signed off > claims ~900 x 279 px, centred on both axes, at ~3.2:1, test/og-card-build.test.ts > AC-003: the content box is the placement a human signed off > clears the 53 px clear-space floor on all four sides, test/og-card-build.test.ts > AC-003: the content box is the placement a human signed off > leaves all four corner pixels pure ground, test/og-card-build.test.ts > AC-004: the accent survived as accent, and stayed punctuation > has accent-valued pixels — the period is orange, not bone, test/og-card-build.test.ts > AC-004: the accent survived as accent, and stayed punctuation > spends under 1 % of the canvas on accent, test/og-card-build.test.ts > AC-009: one manifest is the single source for every card tag > carries exactly the four consumer fields, with `src` root-relative, test/og-card-build.test.ts > AC-009: one manifest is the single source for every card tag > carries the design's constant alt string verbatim)"}
+{"ts":"2026-08-22T05:19:36.953Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"verdict","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"spec","pass":true,"note":"approved"}
+{"ts":"2026-08-22T05:21:28.692Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"verdict","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"spec","pass":true,"note":"approved"}
+{"ts":"2026-08-22T05:27:03.319Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"stage_exit","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"spec","outcome":"completed"}
+{"ts":"2026-08-22T05:27:03.407Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"stage_enter","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"implement"}
+{"ts":"2026-08-22T05:27:03.482Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"gate_result","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"implement","gate":"{{redkiln}} verify --item {{id}} --grain story","pass":true,"basis":"evaluated"}
+{"ts":"2026-08-22T07:07:35.707Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"item_edited","item":"RK-S0274"}
+{"ts":"2026-08-22T07:07:50.768Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"item_edited","item":"RK-S0274"}
+{"ts":"2026-08-22T07:07:55.219Z","actor":"ryan-britton","worktree":"fix-json-error-envelope","branch":"fix/json-error-envelope","event":"item_edited","item":"RK-S0274"}
diff --git a/.redkiln/telemetry/events/ryan-britton@fix-serve-child-leak@runs.jsonl b/.redkiln/telemetry/events/ryan-britton@fix-serve-child-leak@runs.jsonl
new file mode 100644
index 00000000..aff86941
--- /dev/null
+++ b/.redkiln/telemetry/events/ryan-britton@fix-serve-child-leak@runs.jsonl
@@ -0,0 +1 @@
+{"ts":"2026-08-21T21:45:57.328Z","actor":"ryan-britton","worktree":"fix-serve-child-leak","branch":"fix/serve-child-leak","event":"workflow_run","item":"RK-S0272","type":"story","initiative":"support","project":"fix-serve-child-leak","process":"story-lite","stage":"implement","tokens_out":327228,"workflow":"fix","result":"committed","agents":7,"degraded":0,"score":3,"tier":"small","sha":"e6081860","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":4402},{"phase":"Fix","agents":5,"retried":0,"failed":0,"outTokens":317175},{"phase":"Review","agents":1,"retried":0,"failed":0,"outTokens":5651}],"models":{"sonnet":4,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
diff --git a/.redkiln/telemetry/events/ryan-britton@issue-backlog-sweep-lane@runs.jsonl b/.redkiln/telemetry/events/ryan-britton@issue-backlog-sweep-lane@runs.jsonl
index 9825d13a..48b168f9 100644
--- a/.redkiln/telemetry/events/ryan-britton@issue-backlog-sweep-lane@runs.jsonl
+++ b/.redkiln/telemetry/events/ryan-britton@issue-backlog-sweep-lane@runs.jsonl
@@ -2,3 +2,27 @@
 {"ts":"2026-08-01T07:32:28.573Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-I0004","type":"initiative","initiative":"issue-backlog-sweep-lane","project":null,"process":"initiative","stage":"storymap","tokens_out":689555,"workflow":"plan-briefs","result":"completed","agents":32,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Project distillation","agents":5,"retried":0,"failed":0},{"phase":"Grounding","agents":5,"retried":0,"failed":0},{"phase":"Briefs","agents":12,"retried":0,"failed":0},{"phase":"Story map","agents":5,"retried":0,"failed":0},{"phase":"Verify","agents":5,"retried":4,"failed":4}],"models":{"opus":17,"sonnet":15},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
 {"ts":"2026-08-01T08:26:14.442Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-I0004","type":"initiative","initiative":"issue-backlog-sweep-lane","project":null,"process":"initiative","stage":"storymap","tokens_out":628239,"workflow":"plan-design","result":"completed","agents":18,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Design ground","agents":5,"retried":0,"failed":0},{"phase":"Resolve","agents":5,"retried":0,"failed":0},{"phase":"Mock","agents":4,"retried":0,"failed":0},{"phase":"Verify","agents":4,"retried":0,"failed":0}],"models":{"sonnet":10,"opus":8},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
 {"ts":"2026-08-03T00:57:24.765Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0013","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-backlog-snapshot","process":"project","stage":"implementation","tokens_out":369567,"workflow":"implement","result":"halted:stories","agents":7,"degraded":1,"tier":"standard","sha":"0c1e535079f5f89fc056b4e0b9f3c1376926e6b0","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":5232},{"phase":"Stories","agents":6,"retried":0,"failed":0,"outTokens":364335}],"models":{"sonnet":2,"opus":3,"haiku":2},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-13T20:59:39.164Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0013","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-backlog-snapshot","process":"project","stage":"implementation","tokens_out":538410,"workflow":"implement","result":"halted:stories","agents":13,"degraded":0,"tier":"standard","sha":"3328fc3c","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":12628},{"phase":"Stories","agents":12,"retried":0,"failed":0,"outTokens":525782}],"models":{"sonnet":4,"opus":5,"haiku":4},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-13T23:57:31.999Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0013","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-backlog-snapshot","process":"project","stage":"implementation","tokens_out":251514,"workflow":"implement","result":"halted:design-review","agents":8,"degraded":0,"tier":"standard","sha":"4ec1cd67","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":10423},{"phase":"Stories","agents":5,"retried":0,"failed":0,"outTokens":138929},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":66052},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":36110}],"models":{"sonnet":2,"opus":4,"haiku":2},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-14T02:01:14.537Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0013","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-backlog-snapshot","process":"project","stage":"implementation","tokens_out":195501,"workflow":"implement","result":"completed","agents":4,"degraded":0,"score":3,"tier":"standard","sha":"bc2e8d81","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":11368},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":80638},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":39388},{"phase":"Project review","agents":1,"retried":0,"failed":0,"outTokens":64107},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":3},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-14T06:19:34.684Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":410107,"workflow":"implement","result":"halted:stories","agents":6,"degraded":0,"tier":"standard","sha":"8d7b0c81","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":9363},{"phase":"Stories","agents":5,"retried":0,"failed":0,"outTokens":400744}],"models":{"sonnet":2,"opus":3,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-14T16:18:59.011Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":138397,"workflow":"implement","result":"halted:stories","agents":5,"degraded":0,"tier":"standard","sha":"5a4dd27b","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":12263},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":126134}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-15T05:34:37.948Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":1331339,"workflow":"implement","result":"halted:design-review","agents":32,"degraded":0,"tier":"standard","sha":"cbb5184a","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":17057},{"phase":"Stories","agents":29,"retried":0,"failed":0,"outTokens":1181949},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":120657},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":11676}],"models":{"sonnet":10,"opus":11,"haiku":11},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-15T15:03:21.727Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":18161,"workflow":"implement","result":"halted:preflight","agents":1,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":18161}],"models":{"sonnet":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-15T17:16:42.650Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":112571,"workflow":"implement","result":"halted:integration","agents":2,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":26299},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":86272},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-15T19:24:16.683Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":183237,"workflow":"implement","result":"completed:fixed","agents":5,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":17234},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":96046},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":4083},{"phase":"Project review","agents":2,"retried":0,"failed":0,"outTokens":65874},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":4},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-15T21:02:52.759Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":149337,"workflow":"implement","result":"halted:design-review","agents":3,"degraded":0,"tier":"standard","sha":"fe359ed0","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":28916},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":80011},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":40410},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":2},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-15T22:57:05.010Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0014","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-cluster-proposal","process":"project","stage":"implementation","tokens_out":204089,"workflow":"implement","result":"completed","agents":4,"degraded":0,"score":3,"tier":"standard","sha":"9634d4f2","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":11234},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":98707},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":20266},{"phase":"Project review","agents":1,"retried":0,"failed":0,"outTokens":73882},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":3},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-16T06:53:33.337Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":552611,"workflow":"implement","result":"halted:stories","agents":13,"degraded":0,"tier":"standard","sha":"95bd9fd2","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":7934},{"phase":"Stories","agents":12,"retried":0,"failed":0,"outTokens":544677}],"models":{"sonnet":5,"opus":4,"haiku":4},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-16T19:56:01.130Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":624458,"workflow":"implement","result":"halted:stories","agents":9,"degraded":0,"tier":"standard","sha":"c130631a","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":27504},{"phase":"Stories","agents":8,"retried":0,"failed":0,"outTokens":596954}],"models":{"sonnet":3,"opus":4,"haiku":2},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-16T22:32:10.775Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":217781,"workflow":"implement","result":"halted:stories","agents":5,"degraded":0,"tier":"standard","sha":"a881e94b","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":10117},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":207664}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-17T04:37:03.558Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","workflow":"implement","result":"stopped:operator","degraded":0,"tier":"standard","sha":"fb4978cc","run":{"phases":[{"phase":"Stories","note":"partial - operator stop"}],"budgetTotal":null,"budgetRemaining":null,"note":"Run 4 was stopped by the operator at a budget checkpoint, not by a halt. No manifest was returned, so this record is reconstructed from git. Slice disposition-review-surface sealed approved (a740e0be) after the finding-12 recovery flag landed (1750dc86); slice judgment-advisories delivered both stories (RK-S0145 bf480f85, RK-S0146 67eca610, plus fix fb4978cc) but was NOT sealed when the run was stopped. Tree clean at fb4978cc. Integration, design review and project review did not run.","storiesDoneAtStop":8,"storiesTotal":10,"slicesSealedAtStop":3,"slicesTotal":5}}
+{"ts":"2026-08-17T05:52:27.727Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":123765,"workflow":"implement","result":"halted:stories","agents":5,"degraded":1,"tier":"standard","sha":"d7f7cfd","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":11369},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":112396}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-17T19:09:42.579Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":187117,"workflow":"implement","result":"halted:stories","agents":5,"degraded":1,"tier":"standard","sha":"bd413339","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":9843},{"phase":"Stories","agents":4,"retried":0,"failed":0,"outTokens":177274}],"models":{"sonnet":2,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-18T00:09:50.285Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":66553,"workflow":"implement","result":"halted:stories","agents":4,"degraded":1,"tier":"standard","sha":"21efe7e3","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":12763},{"phase":"Stories","agents":3,"retried":1,"failed":1,"outTokens":53790}],"models":{"sonnet":1,"opus":2,"haiku":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-18T05:14:08.549Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":712366,"workflow":"implement","result":"halted:design-review","agents":19,"degraded":1,"tier":"standard","sha":"3c57c3f2","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":20262},{"phase":"Stories","agents":16,"retried":0,"failed":0,"outTokens":538880},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":106010},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":47214}],"models":{"sonnet":5,"opus":8,"haiku":6},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-18T15:54:57.088Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0015","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-disposition-gate","process":"project","stage":"implementation","tokens_out":266591,"workflow":"implement","result":"completed","agents":4,"degraded":1,"tier":"standard","sha":"89b383ab","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":17093},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":110662},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":53036},{"phase":"Project review","agents":1,"retried":0,"failed":0,"outTokens":85800},{"phase":"Stories","outTokens":0}],"models":{"sonnet":1,"opus":3},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-19T05:30:19.490Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0016","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-materialization","process":"project","stage":"implementation","tokens_out":1013035,"workflow":"implement","result":"halted:stories","agents":21,"degraded":0,"tier":"standard","sha":"5758b8d9","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":13356},{"phase":"Stories","agents":20,"retried":0,"failed":0,"outTokens":999679}],"models":{"sonnet":6,"opus":8,"haiku":7},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-19T23:18:42.585Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0016","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-materialization","process":"project","stage":"implementation","tokens_out":1034074,"workflow":"implement","result":"completed","agents":30,"degraded":2,"tier":"standard","sha":"4866263","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":18817},{"phase":"Stories","agents":26,"retried":0,"failed":0,"outTokens":817805},{"phase":"Integration","agents":1,"retried":0,"failed":0,"outTokens":85240},{"phase":"Design review","agents":1,"retried":0,"failed":0,"outTokens":3404},{"phase":"Project review","agents":1,"retried":0,"failed":0,"outTokens":108808}],"models":{"sonnet":9,"opus":13,"haiku":8},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-20T04:59:04.201Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0017","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-lane-surface","process":"project","stage":"implementation","tokens_out":20425,"workflow":"implement","result":"halted:preflight","agents":1,"degraded":0,"tier":"standard","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":20425}],"models":{"sonnet":1},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
+{"ts":"2026-08-20T11:35:12.485Z","actor":"ryan-britton","worktree":"issue-backlog-sweep-lane","branch":"initiative/issue-backlog-sweep-lane","event":"workflow_run","item":"RK-P0017","type":"project","initiative":"issue-backlog-sweep-lane","project":"sweep-lane-surface","process":"project","stage":"implementation","tokens_out":771422,"workflow":"implement","result":"halted:stories","agents":18,"degraded":0,"tier":"standard","sha":"6676b9e5","run":{"phases":[{"phase":"Preflight","agents":1,"retried":0,"failed":0,"outTokens":17260},{"phase":"Stories","agents":17,"retried":0,"failed":0,"outTokens":754162}],"models":{"sonnet":5,"opus":8,"haiku":5},"agentsRetried":0,"agentsFailed":0,"budgetTotal":null,"budgetRemaining":null}}
diff --git a/.redkiln/telemetry/events/ryan-britton@redkiln.jsonl b/.redkiln/telemetry/events/ryan-britton@redkiln.jsonl
index 4c37b627..972ab5db 100644
--- a/.redkiln/telemetry/events/ryan-britton@redkiln.jsonl
+++ b/.redkiln/telemetry/events/ryan-britton@redkiln.jsonl
@@ -53,3 +53,5 @@
 {"ts":"2026-08-20T15:34:06.819Z","actor":"ryan-britton","worktree":"redkiln","branch":"fix/announcer-client-convergence","event":"session_start"}
 {"ts":"2026-08-21T13:20:30.091Z","actor":"ryan-britton","worktree":"redkiln","branch":"fix/announcer-client-convergence","event":"session_start"}
 {"ts":"2026-08-21T19:29:27.116Z","actor":"ryan-britton","worktree":"redkiln","branch":"main","event":"session_start"}
+{"ts":"2026-08-22T02:49:19.886Z","actor":"ryan-britton","worktree":"redkiln","branch":"main","event":"session_end"}
+{"ts":"2026-08-22T04:47:04.105Z","actor":"ryan-britton","worktree":"redkiln","branch":"main","event":"session_start"}
diff --git a/.redkiln/telemetry/events/ryanm@fix-json-error-envelope.jsonl b/.redkiln/telemetry/events/ryanm@fix-json-error-envelope.jsonl
new file mode 100644
index 00000000..8b67d39a
--- /dev/null
+++ b/.redkiln/telemetry/events/ryanm@fix-json-error-envelope.jsonl
@@ -0,0 +1 @@
+{"ts":"2026-08-22T05:20:32.956Z","actor":"ryanm","worktree":"fix-json-error-envelope","branch":"","event":"gate_result","item":"RK-S0274","type":"story","initiative":"support","project":"fix-json-error-envelope","process":"story-lite","stage":"implement","outcome":"halted:gate","gate":"command","pass":false,"note":"command gate failed (EC-003): `{{redkiln}} verify --item {{id}} --grain story` — affected-gate (`pnpm typecheck && pnpm lint && pnpm test` exited non-zero on two consecutive runs)"}
diff --git a/dist/cli.js b/dist/cli.js
index 5fc6d228..2ab72a27 100644
--- a/dist/cli.js
+++ b/dist/cli.js
@@ -16659,10 +16659,34 @@ function upgradeCommand(flags) {
 // src/cli/advance.ts
 import { existsSync as existsSync15 } from "node:fs";
 import { resolve as resolve7 } from "node:path";
+
+// src/cli/json-error.ts
+var PUBLISHED_CODES = /* @__PURE__ */ new Set(["EC-001"]);
+var ERROR_CODE_MARKER = /\(EC-(\d{3})\)/;
+function errorCode(message) {
+  const digits = ERROR_CODE_MARKER.exec(message)?.[1];
+  if (digits === void 0) return void 0;
+  const code = `EC-${digits}`;
+  return PUBLISHED_CODES.has(code) ? code : void 0;
+}
+function jsonErrorEnvelope(message, code) {
+  const resolved = code ?? errorCode(message);
+  return { error: resolved === void 0 ? { message } : { code: resolved, message } };
+}
+function failJson(json, message, code) {
+  if (json === true) {
+    process.stdout.write(`${JSON.stringify(jsonErrorEnvelope(message, code))}
+`);
+  }
+  process.stderr.write(`redkiln: ${message}
+`);
+}
+
+// src/cli/advance.ts
 function advanceCommand(flags) {
   const root = resolve7(flags.cwd ?? process.cwd());
   if (flags.id === void 0 || flags.id.trim() === "") {
-    process.stderr.write("redkiln: 'advance' requires an item id.\n");
+    failJson(flags.json, "'advance' requires an item id.");
     return 1;
   }
   const dryRun = flags.dryRun === true;
@@ -16770,8 +16794,7 @@ function advanceCommand(flags) {
       }
     }
     const message = err instanceof AdvanceError || err instanceof Error ? err.message : String(err);
-    process.stderr.write(`redkiln: ${message}
-`);
+    failJson(flags.json, message);
     if (unpersistedHops !== null) process.stderr.write(unpersistedHops);
     return 1;
   }
@@ -19781,7 +19804,7 @@ async function linkCommand(flags) {
   const root = resolve12(flags.cwd ?? process.cwd());
   const id = flags.id;
   if (id === void 0 || id === "") {
-    process.stderr.write("redkiln: link needs an item id: redkiln link <id> --blocks <id>\n");
+    failJson(flags.json, "link needs an item id: redkiln link <id> --blocks <id>");
     return 1;
   }
   const edges = [
@@ -19789,7 +19812,7 @@ async function linkCommand(flags) {
     ...ids(flags.blockedBy).map((blocker) => ({ blocker, blocked: id }))
   ];
   if (edges.length === 0) {
-    process.stderr.write("redkiln: give at least one --blocks <ids> or --blocked-by <ids>\n");
+    failJson(flags.json, "give at least one --blocks <ids> or --blocked-by <ids>");
     return 1;
   }
   try {
@@ -19829,8 +19852,7 @@ async function linkCommand(flags) {
     return 0;
   } catch (err) {
     if (err instanceof LinkError) {
-      process.stderr.write(`redkiln: ${err.message}
-`);
+      failJson(flags.json, err.message);
       return 1;
     }
     throw err;
@@ -20145,26 +20167,23 @@ async function newCommand(flags) {
   const root = resolve13(flags.cwd ?? process.cwd());
   const typeParse = ItemType.safeParse(flags.type);
   if (!typeParse.success) {
-    process.stderr.write(
-      `redkiln: 'new' requires a type (initiative|project|story|bug|incident|spike).
-`
-    );
+    failJson(flags.json, "'new' requires a type (initiative|project|story|bug|incident|spike).");
     return 1;
   }
   const type = typeParse.data;
   if (flags.title === void 0 || flags.title.trim() === "") {
-    process.stderr.write("redkiln: 'new' requires --title.\n");
+    failJson(flags.json, "'new' requires --title.");
     return 1;
   }
   if (flags.slug === void 0 || flags.slug.trim() === "") {
-    process.stderr.write("redkiln: 'new' requires --slug.\n");
+    failJson(flags.json, "'new' requires --slug.");
     return 1;
   }
   let automation;
   if (flags.automation !== void 0) {
     const a = Automation.safeParse(flags.automation);
     if (!a.success) {
-      process.stderr.write("redkiln: --automation must be HITL or AFK.\n");
+      failJson(flags.json, "--automation must be HITL or AFK.");
       return 1;
     }
     automation = a.data;
@@ -20173,7 +20192,7 @@ async function newCommand(flags) {
   if (flags.severity !== void 0) {
     const s = Severity.safeParse(flags.severity);
     if (!s.success) {
-      process.stderr.write("redkiln: --severity must be blocker|high|medium|low.\n");
+      failJson(flags.json, "--severity must be blocker|high|medium|low.");
       return 1;
     }
     severity = s.data;
@@ -20182,24 +20201,24 @@ async function newCommand(flags) {
   if (flags.archetype !== void 0) {
     const a = Archetype.safeParse(flags.archetype);
     if (!a.success) {
-      process.stderr.write("redkiln: --archetype must be capability or foundation.\n");
+      failJson(flags.json, "--archetype must be capability or foundation.");
       return 1;
     }
     archetype = a.data;
   }
   if (flags.terminal !== void 0 && type !== "project") {
-    process.stderr.write("redkiln: --terminal/--no-terminal applies to a project only.\n");
+    failJson(flags.json, "--terminal/--no-terminal applies to a project only.");
     return 1;
   }
   let tier;
   if (flags.tier !== void 0) {
     if (type !== "initiative" && type !== "project") {
-      process.stderr.write("redkiln: --tier applies to an initiative or project only.\n");
+      failJson(flags.json, "--tier applies to an initiative or project only.");
       return 1;
     }
     const t = Tier.safeParse(flags.tier);
     if (!t.success) {
-      process.stderr.write("redkiln: --tier must be small or standard.\n");
+      failJson(flags.json, "--tier must be small or standard.");
       return 1;
     }
     tier = t.data;
@@ -20240,8 +20259,7 @@ async function newCommand(flags) {
     return 0;
   } catch (err) {
     const message = err instanceof ItemCreateError || err instanceof Error ? err.message : String(err);
-    process.stderr.write(`redkiln: ${message}
-`);
+    failJson(flags.json, message);
     return 1;
   }
 }
@@ -20443,7 +20461,7 @@ function driveStory(root, id, afk, paused, driven, failed, failureReasons) {
 function runProjectCommand(flags) {
   const root = resolve15(flags.cwd ?? process.cwd());
   if (flags.id === void 0 || flags.id.trim() === "") {
-    process.stderr.write("redkiln: 'run-project' requires a project id.\n");
+    failJson(flags.json, "'run-project' requires a project id.");
     return 1;
   }
   try {
@@ -20451,7 +20469,7 @@ function runProjectCommand(flags) {
     if (flags.json === true) {
       process.stdout.write(`${JSON.stringify(result)}
 `);
-      return 0;
+      return result.failed.length > 0 ? 2 : 0;
     }
     if (flags.quiet !== true) {
       const parts = [
@@ -20475,8 +20493,7 @@ function runProjectCommand(flags) {
     return result.failed.length > 0 ? 2 : 0;
   } catch (err) {
     const message = err instanceof RunProjectError || err instanceof Error ? err.message : String(err);
-    process.stderr.write(`redkiln: ${message}
-`);
+    failJson(flags.json, message);
     return 1;
   }
 }
@@ -20605,9 +20622,7 @@ async function recordLinksCommand(flags) {
   const root = resolve16(flags.cwd ?? process.cwd());
   const id = flags.id;
   if (id === void 0 || id === "") {
-    process.stderr.write(
-      "redkiln: record-links needs an item id: redkiln record-links <id> --sha <sha>\n"
-    );
+    failJson(flags.json, "record-links needs an item id: redkiln record-links <id> --sha <sha>");
     return 1;
   }
   try {
@@ -20677,8 +20692,7 @@ async function recordLinksCommand(flags) {
     return 0;
   } catch (err) {
     const message = err instanceof RecordLinksError || err instanceof Error ? err.message : String(err);
-    process.stderr.write(`redkiln: ${message}
-`);
+    failJson(flags.json, message);
     return 1;
   }
 }
@@ -20937,8 +20951,7 @@ function recordRunCommand(flags) {
     return 0;
   } catch (err) {
     const message = err instanceof RecordRunError || err instanceof Error ? err.message : String(err);
-    process.stderr.write(`redkiln: ${message}
-`);
+    failJson(flags.json, message);
     return 1;
   }
 }
diff --git a/dist/index.js b/dist/index.js
index 9c319e75..b8cbab42 100644
--- a/dist/index.js
+++ b/dist/index.js
@@ -16659,10 +16659,34 @@ function upgradeCommand(flags) {
 // src/cli/advance.ts
 import { existsSync as existsSync15 } from "node:fs";
 import { resolve as resolve7 } from "node:path";
+
+// src/cli/json-error.ts
+var PUBLISHED_CODES = /* @__PURE__ */ new Set(["EC-001"]);
+var ERROR_CODE_MARKER = /\(EC-(\d{3})\)/;
+function errorCode(message) {
+  const digits = ERROR_CODE_MARKER.exec(message)?.[1];
+  if (digits === void 0) return void 0;
+  const code = `EC-${digits}`;
+  return PUBLISHED_CODES.has(code) ? code : void 0;
+}
+function jsonErrorEnvelope(message, code) {
+  const resolved = code ?? errorCode(message);
+  return { error: resolved === void 0 ? { message } : { code: resolved, message } };
+}
+function failJson(json, message, code) {
+  if (json === true) {
+    process.stdout.write(`${JSON.stringify(jsonErrorEnvelope(message, code))}
+`);
+  }
+  process.stderr.write(`redkiln: ${message}
+`);
+}
+
+// src/cli/advance.ts
 function advanceCommand(flags) {
   const root = resolve7(flags.cwd ?? process.cwd());
   if (flags.id === void 0 || flags.id.trim() === "") {
-    process.stderr.write("redkiln: 'advance' requires an item id.\n");
+    failJson(flags.json, "'advance' requires an item id.");
     return 1;
   }
   const dryRun = flags.dryRun === true;
@@ -16770,8 +16794,7 @@ function advanceCommand(flags) {
       }
     }
     const message = err instanceof AdvanceError || err instanceof Error ? err.message : String(err);
-    process.stderr.write(`redkiln: ${message}
-`);
+    failJson(flags.json, message);
     if (unpersistedHops !== null) process.stderr.write(unpersistedHops);
     return 1;
   }
@@ -19781,7 +19804,7 @@ async function linkCommand(flags) {
   const root = resolve12(flags.cwd ?? process.cwd());
   const id = flags.id;
   if (id === void 0 || id === "") {
-    process.stderr.write("redkiln: link needs an item id: redkiln link <id> --blocks <id>\n");
+    failJson(flags.json, "link needs an item id: redkiln link <id> --blocks <id>");
     return 1;
   }
   const edges = [
@@ -19789,7 +19812,7 @@ async function linkCommand(flags) {
     ...ids(flags.blockedBy).map((blocker) => ({ blocker, blocked: id }))
   ];
   if (edges.length === 0) {
-    process.stderr.write("redkiln: give at least one --blocks <ids> or --blocked-by <ids>\n");
+    failJson(flags.json, "give at least one --blocks <ids> or --blocked-by <ids>");
     return 1;
   }
   try {
@@ -19829,8 +19852,7 @@ async function linkCommand(flags) {
     return 0;
   } catch (err) {
     if (err instanceof LinkError) {
-      process.stderr.write(`redkiln: ${err.message}
-`);
... TRUNCATED at 400 lines; 1 commits total
```
