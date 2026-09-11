# P1 arm D — human labels (Ryan). Worksheet-populated 2026-09-09.
#
# METHOD-P §0 rule 1: fill this from the item and its diff ALONE. Everything you need is inline below;
# each item block carries the card's own summary, the paths it names, and any linked commits.
# The engine's answers (spikes/results/p1-arm-d.jsonl) were NOT read when building this file and must
# not be opened until every value cell is filled.
#
# HOW TO FILL: one row per (item, facet). Put the value in the third cell. Leave `?` where the item
# alone cannot tell — `?` is a real answer and is excluded from scoring, not counted against the script.
#
#   toolchain       set from {node, dotnet, python, rust, unreal} — comma-separated if several
#   change          set from {text, binary, generated} — comma-separated if several
#   binary_change   1/0 — did the change touch a binary asset? (NOT in facets.toml; measured anyway, decision #11)
#   auth            1/0 — does the change touch authentication/authorisation code?
#   data_migration  1/0 — does it carry a schema or data migration?
#   ui              1/0 — does it change a user-visible surface?
#   flag_exists     1/0 — is a feature flag involved?
#   recent_deploy   pre-filled ? — underivable at this pin, no deploy_log in the frozen corpus (decision #12)
#
# WHEN DONE: set labelled_at to the UTC time you finished (must end in Z), fill blind_attestation, then
#   python spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl \
#     --labels spikes/results/p1-arm-d-labels-ryan.md --attested-blind
# Any facet printing 'LEAVES THE MODEL (>=0.95)' never needs a human or a model again.

labelled_at: FILL-ME-ISO8601Z
labeller: FILL-ME
# 2026-09-11 (audit PIA-096; DECISIONS-2026-09-11 #13; METHOD-P amendment 10(b)): this line is Ryan's to write and is left FILL-ME. It must carry, in one line: WHO attests (Ryan Britton), WHAT was not opened before each of the 32 contested-cell answers (spikes/results/p1-arm-d.jsonl; p1-arm-d-panel-accepted.jsonl / p1-arm-d-contested.json; and the Artifact page's script/panel reveal, which showed only after the answer was entered), and WHEN (the adjudication date, 2026-09-09, and the date of the attestation). It covers F39's 32 adjudicated cells only; the label rows below stay unfilled by design (amendment 9 replaced this worksheet's route). No script reads this line until --attested-blind is passed.
blind_attestation: FILL-ME (e.g. "Ryan Britton attests on 2026-09-09 that he had not opened spikes/results/p1-arm-d.jsonl before completing these labels")
corpus_sha: 47faec9f46b8d983c2c75f0794164e68415ae706
items_file: spikes/plans/p1-items.txt

---

## 1/20 — Per-weight IBM Plex fallback metric overrides for cold-load reflow residual

`_archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics`

**type:** bug · **severity:** medium · **status:** done

<details><summary>card summary (first lines of the body)</summary>

```
## Summary

> **Superseded 2026-08-09 — read this first.** The claim below that AC-010 "is not fully met" was
> true against the criterion as originally written, and that criterion no longer exists: the DT-5
> re-ruling (`../_design.md` Sign-off row of 2026-08-09, option (c) of the three this item names)
> replaced the absolute with a budget — no block gaining or losing more than one line, no block
> moving more than 40 px, font-attributable CLS under 0.1. The measurement in this Summary is
> unchanged and still accurate; what changed is the bar it is read against. Against the amended
> criterion the worst readings are **1** line of 1 allowed and **23.8 px** of 40 allowed, and AC-010
> is met — with the cost stated rather than hidden: 6 of 28 pairs re-wrap a block at a different
> word. The paragraph is kept as filed so the ruling can be checked against the report that provoked
> it. See *RULED 2026-08-09 — option (c), amend the criteria* below for where it is recorded.

RK-P0022 typography-delivery's `cold-load-delivery-behaviour` story (RK-S0221) implements DT-5's
`font-display: swap` + metric-matched-fallback delivery mode. AC-010's anti-pattern-1 half — "the
frame the reader starts on occupies the geometry the identity face will occupy" — is not fully met.
A single `size-adjust` scalar per family (derived from the variable font's default 400 instance)
equalises the *average* advance of body prose, but cannot simultaneously equalise an individual
word's advance and a heading set at 600/700. The residual: of 228 measured blocks across 28 declared
pairs, 4 change line count and 6 break at a different word, worst case a 40.8px position difference
where an `h2` wraps to two lines instead of one.
```

</details>

**Paths named in the card** (16) — an index into the text above, not a conclusion:

- `../_design.md`
- `scripts/observe-cold-load.mjs`
- `cold-load-delivery-behaviour/evidence/README.md`
- `cold-load-delivery-behaviour/evidence/pair-review.md`
- `src/store/verify.ts`
- `src/store/actor.ts`
- `scripts/measure-font-metrics.mjs`
- `cold-load-delivery-behaviour/spec.md`
- `../../_implementation.md`
- `.kb/decisions/brand-visual-identity-constraints.md`
- `../cold-load-delivery-behaviour/evidence/observation.json`
- `../project.md`
- `../cold-load-delivery-behaviour/spec.md`
- `../design/mock.html`
- `../design/_build.mjs`
- `test/typography-delivery.test.ts`

**Commits touching this item** (1) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `1fd8c145 docs(typography-delivery): waive AC-010 anti-pattern-1 residual, file RK-B0014`

**Files changed** — 4 total: **0 outside `.bklg/`**, 4 backlog bookkeeping.
**This is the diff — label from it.**

- *(every changed file is `.bklg/` bookkeeping — this item records no code change at the pin)*

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics/bug.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | toolchain |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | change |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | binary_change |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | auth |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | data_migration |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | ui |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | flag_exists |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | recent_deploy | ? |  |

---

## 2/20 — Dark-cut focus ring is invisible on scrollable code blocks

`_archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre`

**type:** bug · **severity:** high · **status:** done

<details><summary>card summary (first lines of the body)</summary>

```
## Summary

Scrollable code blocks on the docs site are real keyboard tab stops, and in the **dark cut** they
carry no visible focus indicator.

Expressive Code emits every scrollable `<pre>` with `tabindex="0"` so a keyboard user can scroll it —
verifiable in the built artifact, e.g. `docs/dist/how-to/advance-items/index.html`:

```
<pre class="astro-code astro-code-themes github-light github-dark"
     style="background-color:#fff;--shiki-dark-bg:#24292e;…; overflow-x: auto;"
     tabindex="0" data-language="text">
```

Those stops match **none** of the seven selectors in the `:focus-visible` rules at
[`docs/src/styles/theme.css`](../../../../../docs/src/styles/theme.css):806-829 (`a`, `.sidebar
.sidebar-link`, `.toc-link`, `.pager a`, `#menu-toggle`, `.search-trigger`, `.copy-btn`), so they
fall back to Chrome's user-agent ring.

Measured on the assembled build during RK-P0024's terminal Definition-of-Done run (real Chrome over
CDP, 1440x900, served from `docs/dist`): **22 of 298 real tab stops per cut** are these blocks. In the
dark cut the entire focused-versus-unfocused difference is a single 1 px row,
```

</details>

**Paths named in the card** (4) — an index into the text above, not a conclusion:

- `docs/dist/how-to/advance-items/index.html`
- `docs/src/styles/theme.css`
- `test/docs-brand.test.ts`
- `../../kb-harvest-and-closeout/_integration.md`

**Commits touching this item** (1) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `3e413968 fix(RK-B0015): give scrollable code blocks the authored focus ring in both cuts`

**Files changed** — 7 total: **4 outside `.bklg/`**, 3 backlog bookkeeping.
**This is the diff — label from it.**

- `docs/src/styles/theme.css`
- `test/docs-brand.test.ts`
- `test/helpers/tab-stops.ts`
- `test/typography-delivery.test.ts`

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre/bug.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | toolchain |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | binary_change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | auth |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | data_migration |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | ui |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | flag_exists |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | recent_deploy | ? |  |

---

## 3/20 — Landing and component consumers

`_archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers`

**type:** story · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (1) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `66ebc412 feat(highlight-and-states): Landing and component consumers`

**Files changed** — 10 total: **7 outside `.bklg/`**, 3 backlog bookkeeping.
**This is the diff — label from it.**

- `docs/src/components/Card.astro`
- `docs/src/components/CommandBlock.astro`
- `docs/src/components/Search.tsx`
- `docs/src/components/Steps.astro`
- `docs/src/pages/index.astro`
- `test/brand-tokens.test.ts`
- `test/docs-brand.test.ts`

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers/story.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | toolchain |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | binary_change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | auth |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | data_migration |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | ui |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | flag_exists |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | recent_deploy | ? |  |

---

## 4/20 — The gate passes the base it already knows

`support/phase-1-total-outcomes/gate-passes-its-known-base`

**type:** story · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

```
> **DELIVERED OUTSIDE THE LIFECYCLE — read this before acting on the stage above.**
>
> This story's work is **merged to `main`** in PR #168, merge commit `a4211ab7`.
> Its frontmatter still says `stage: intake, status: pending`, and that is not a
> half-run pipeline — it is the tracker having no vocabulary for this outcome.
>
> The remediation plan's execution-vehicle decision is that redkiln does **not**
> dogfood its own lifecycle for this work, so these items were created as a
> TRACKER only: they exist to give the `backlog link` check a real id, and no
> `advance` was ever run against them. Walking them to `closeout` now would mean
> authoring artifacts at every gated stage and spending two human verdicts each,
> to record something already true.
>
> Redkiln cannot currently say "tracked, delivered outside the lifecycle". The
> reverse verbs (`close`, `cancel`, `reopen`) arrive as transition kinds in
> Phase 3 of the same plan. Until then `redkiln status` counts these twelve as
> `intake` and includes them in its ready queue, which is wrong and is written
> down here rather than left to be rediscovered.

## What shipped

Closes #152. `src/process/gates.ts` now calls
```

</details>

**Paths named in the card** (2) — an index into the text above, not a conclusion:

- `src/process/gates.ts`
- `src/store/verify.ts`

**Commits touching this item:** none found by slug or trailer at this pin.

**Files changed:** none resolvable at this pin — label from the card body, and use `?` freely.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/phase-1-total-outcomes/gate-passes-its-known-base/story.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/phase-1-total-outcomes/gate-passes-its-known-base | toolchain |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | change |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | binary_change |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | auth |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | data_migration |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | ui |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | flag_exists |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | recent_deploy | ? |  |

---

## 5/20 — KB and graph validation

`_archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation`

**type:** story · **severity:** null · **status:** in-review

<details><summary>card summary (first lines of the body)</summary>

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (2) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `c31017f9 fix(kb-harvest-and-closeout): close the run-2 slice review on assembled-tree-verification`
- `6074520f feat(kb-harvest-and-closeout): KB and graph validation`

**Files changed** — 11 total: **0 outside `.bklg/`**, 11 backlog bookkeeping.
**This is the diff — label from it.**

- *(every changed file is `.bklg/` bookkeeping — this item records no code change at the pin)*

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation/story.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | toolchain |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | binary_change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | auth |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | data_migration |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | ui |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | flag_exists |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | recent_deploy | ? |  |

---

## 6/20 — A fixme'd scenario cannot buy a pass

`_archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire`

**type:** story · **severity:** null · **status:** in-review

<details><summary>card summary (first lines of the body)</summary>

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (1) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `d35850dc feat(proof-and-readings): Fixme cannot pass tripwire`

**Files changed** — 7 total: **4 outside `.bklg/`**, 3 backlog bookkeeping.
**This is the diff — label from it.**

- `package.json`
- `scripts/dod-tripwire.mjs`
- `test/dod-gate-wiring.test.ts`
- `test/dod-tripwire.test.ts`

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire/story.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | toolchain |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | change |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | binary_change |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | auth |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | data_migration |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | ui |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | flag_exists |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | recent_deploy | ? |  |

---

## 7/20 — Resolve open question

`_archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question`

**type:** story · **severity:** null · **status:** in-review

<details><summary>card summary (first lines of the body)</summary>

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (2) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `c82c23b0 chore(kb-harvest-and-closeout): seal slice kb-decision-harvest approved`
- `0f3dd127 feat(kb-harvest-and-closeout): Resolve open question`

**Files changed** — 5 total: **1 outside `.bklg/`**, 4 backlog bookkeeping.
**This is the diff — label from it.**

- `.kb/open-questions/brand-unwired-from-pipeline.md`

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question/story.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | toolchain |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | binary_change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | auth |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | data_migration |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | ui |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | flag_exists |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | recent_deploy | ? |  |

---

## 8/20 — "Fix #50: fix.md creates story before cutting worktree, stranding it on wrong branch (verify: may already be fixed)"

`support/fix-lane-plumbing/issue-50-fix-story-before-worktree`

**type:** story · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

```
> **This is the story item card — status only.** The full specification lives in `spec.md` beside
> this file: it is the source of truth (scope, behavior, data, and AC-### acceptance criteria with
> Given/When/Then and `depends_on`). This card intentionally carries **no** spec body — in the story
> process each stage authors its own artifact (`discover.md` → `spec.md` → `plan.md` →
> `implementation-report.md` → `report.md`), and the implement workflow reads `spec.md`, never this
> card. Machine state (id / status / stage / archetype / slice / blocked_by) lives in the frontmatter
> above; leaving this body as-is is expected, not a half-run pipeline.
```

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item:** none found by slug or trailer at this pin.

**Files changed:** none resolvable at this pin — label from the card body, and use `?` freely.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/fix-lane-plumbing/issue-50-fix-story-before-worktree/story.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | toolchain |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | change |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | binary_change |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | auth |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | data_migration |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | ui |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | flag_exists |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | recent_deploy | ? |  |

---

## 9/20 — Typography delivery

`_archive/docs-site-brand-identity/typography-delivery`

**type:** project · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

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
```

</details>

**Paths named in the card** (39) — an index into the text above, not a conclusion:

- `../_decomposition.md`
- `docs/package.json`
- `docs/astro.config.mjs`
- `docs/src/styles/theme.css`
- `.kb/open-questions/brand-unwired-from-pipeline.md`
- `.kb/concepts/brand-mark-rationale.md`
- `brand/README.md`
- `brand/showcase.html`
- `docs/src/layouts/DocsLayout.astro`
- `docs/src/pages/index.astro`
- `../styles/theme.css`
- `references/audit/findings/12-docs-and-adoption.md`
- `docs/src/lib/url.ts`
- `.github/workflows/docs.yml`
- `plex-per-weight-fallback-metrics/bug.md`
- `cold-load-delivery-behaviour/evidence/observation.json`
- `test/typography-delivery.test.ts`
- `design/_build.mjs`
- `design/mock.html`
- `../initiative.md`
- `src/store/actor.ts`
- `src/store/verify.ts`
- `src/schema/brand-tokens.ts`
- `.github/workflows/ci.yml`
- …and 15 more

**Commits touching this item** (57) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `4954f1ef docs(theme): the Pagefind search-link comment names the real mechanism`
- `6120ad6e fix(verify): declare how the repo is PREPARED, and sweep before the run`
- `4a658f6b fix(test): stop marks-guard racing typography-delivery over docs/dist (#113)`
- `4e3238be fix: repoint source and test references into the archived initiative`
- `f255af97 fix(typography-delivery): correct RK-B0014 artifacts to the live drift budget`
- `b95ffb2d docs(typography-delivery): author RK-B0014 verification report`
- `a1fb2564 docs(typography-delivery): author RK-B0014 discovery record for closure`
- `bc10608d fix(social-card-and-outlining): pin the far end of RK-P0023's blast-radius diff`
- …and 49 more

**Files changed** — 39 total: **21 outside `.bklg/`**, 18 backlog bookkeeping.
**This is the diff — label from it.**

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

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/typography-delivery/project.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/typography-delivery | toolchain |  |  |
| _archive/docs-site-brand-identity/typography-delivery | change |  |  |
| _archive/docs-site-brand-identity/typography-delivery | binary_change |  |  |
| _archive/docs-site-brand-identity/typography-delivery | auth |  |  |
| _archive/docs-site-brand-identity/typography-delivery | data_migration |  |  |
| _archive/docs-site-brand-identity/typography-delivery | ui |  |  |
| _archive/docs-site-brand-identity/typography-delivery | flag_exists |  |  |
| _archive/docs-site-brand-identity/typography-delivery | recent_deploy | ? |  |

---

## 10/20 — "Fix: gate honesty sweep (#122, #129, #130, #131, #135, #137)"

`support/fix-gate-honesty-sweep`

**type:** project · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

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

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (3) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `a0fa1d57 chore(fix-gate-honesty-sweep): close out all five fix stories`
- `edcbf193 docs(fix-gate-honesty-sweep): fix specs for all five stories`
- `1ba293ca docs(fix-gate-honesty-sweep): scaffold the six-issue fix project`

**Files changed** — 19 total: **2 outside `.bklg/`**, 17 backlog bookkeeping.
**This is the diff — label from it.**

- `.redkiln/telemetry/events/ryan-britton@fix-gate-honesty-sweep.jsonl`
- `.redkiln/telemetry/events/ryanm@fix-gate-honesty-sweep.jsonl`

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/fix-gate-honesty-sweep/project.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/fix-gate-honesty-sweep | toolchain |  |  |
| support/fix-gate-honesty-sweep | change |  |  |
| support/fix-gate-honesty-sweep | binary_change |  |  |
| support/fix-gate-honesty-sweep | auth |  |  |
| support/fix-gate-honesty-sweep | data_migration |  |  |
| support/fix-gate-honesty-sweep | ui |  |  |
| support/fix-gate-honesty-sweep | flag_exists |  |  |
| support/fix-gate-honesty-sweep | recent_deploy | ? |  |

---

## 11/20 — "Fix: --json failure envelope + exit-code audit (#149)"

`support/fix-json-error-envelope`

**type:** project · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

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

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (1) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `cf5dbd07 fix(cli): emit a JSON error envelope on every --json failure path (#149, #116) (#158)`

**Files changed** — 23 total: **18 outside `.bklg/`**, 5 backlog bookkeeping.
**This is the diff — label from it.**

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

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/fix-json-error-envelope/project.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/fix-json-error-envelope | toolchain |  |  |
| support/fix-json-error-envelope | change |  |  |
| support/fix-json-error-envelope | binary_change |  |  |
| support/fix-json-error-envelope | auth |  |  |
| support/fix-json-error-envelope | data_migration |  |  |
| support/fix-json-error-envelope | ui |  |  |
| support/fix-json-error-envelope | flag_exists |  |  |
| support/fix-json-error-envelope | recent_deploy | ? |  |

---

## 12/20 — KB harvest and closeout

`_archive/docs-site-brand-identity/kb-harvest-and-closeout`

**type:** project · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

```
## One-line objective

Leave the knowledge base true — the open question that motivated RK-I0005 no longer describes a
fork this initiative settled, its durable decisions and personas are atoms the next surface
inherits — and prove on the fully assembled site that nothing outside the docs surface moved and
the site still builds.

## How this advances the initiative

This is the initiative's **terminal** project (`terminal: true`), and the decomposition assigns it
that role alone: RK-P0018 … RK-P0023 were each created `--no-terminal`
([`../_decomposition.md`](../_decomposition.md), *Terminal-role assignment*). That is not a label.
`src/store/verify.ts` reads `terminal` straight off the frontmatter — an item is held to the
whole-initiative Definition of Done unless it carries an explicit `terminal: false`
(`isTerminalProject`, `src/store/verify.ts:680-683`) — so at this project's `integration` gate the
command gate declared in `.redkiln/processes/project.yaml:69-70` runs `verify.reachability_static`
and `verify.e2e` from [`.redkiln/config.yaml`](../../../../.redkiln/config.yaml) (`pnpm check:workflows`
and `pnpm run ci`) rather than the project-scoped bar
(`src/store/verify.ts:749-769`). Every sibling defers those journeys here.

The initiative's central risk is a **beautiful restyle that leaves the Steward's search returning
Radix** — the visible half done, the structural half missed. RK-P0018 closes the structural half by
```

</details>

**Paths named in the card** (26) — an index into the text above, not a conclusion:

- `../_decomposition.md`
- `src/store/verify.ts`
- `.redkiln/config.yaml`
- `.kb/open-questions/brand-unwired-from-pipeline.md`
- `.kb/maps/brand-identity-knowledge-map.md`
- `.kb/README.md`
- `../_discovery/distillation/personas-and-journeys.md`
- `../_discovery/distillation/opportunities.md`
- `src/schema/kb.ts`
- `src/viz/html.ts`
- `workflows/forge-plan-design.js`
- `docs/package.json`
- `.kb/decisions/brand-visual-identity-constraints.md`
- `src/schema/config.ts`
- `references/audit/findings/12-docs-and-adoption.md`
- `brand/README.md`
- `docs/src/content/docs/**/*.mdx`
- `.bklg/support/initiative.md`
- `../initiative.md`
- `.kb/concepts/brand-mark-rationale.md`
- `.kb/playbooks/brand-asset-usage.md`
- `.kb/_templates/atom.md`
- `.kb/_governance/integration-waves/2026-07-31-intake/03-integration-summary.md`
- `.redkiln/processes/project.yaml`
- …and 2 more

**Commits touching this item** (29) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `f2d2f4e7 docs(kb-harvest-and-closeout): re-score the project review to approved`
- `89993524 docs(kb-harvest-and-closeout): re-run DoD 14 and 15 at the final tree`
- `7a810c46 fix(kb-harvest-and-closeout): flip BR-17 to met with evidence`
- `f678aa1b chore(kb-harvest-and-closeout): record project review`
- `ff6a1ef7 chore(kb-harvest-and-closeout): record integration audit`
- `111c946d chore(kb-harvest-and-closeout): record integration audit`
- `0df64280 chore(kb-harvest-and-closeout): seal slice assembled-tree-verification approved`
- `c31017f9 fix(kb-harvest-and-closeout): close the run-2 slice review on assembled-tree-verification`
- …and 21 more

**Files changed** — 15 total: **0 outside `.bklg/`**, 15 backlog bookkeeping.
**This is the diff — label from it.**

- *(every changed file is `.bklg/` bookkeeping — this item records no code change at the pin)*

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/kb-harvest-and-closeout/project.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | toolchain |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | binary_change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | auth |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | data_migration |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | ui |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | flag_exists |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | recent_deploy | ? |  |

---

## 13/20 — "Fix: CI pinned-pack drift guard"

`support/fix-ci-pinned-pack-drift-guard`

**type:** project · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

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

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item:** none found by slug or trailer at this pin.

**Files changed:** none resolvable at this pin — label from the card body, and use `?` freely.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/fix-ci-pinned-pack-drift-guard/project.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/fix-ci-pinned-pack-drift-guard | toolchain |  |  |
| support/fix-ci-pinned-pack-drift-guard | change |  |  |
| support/fix-ci-pinned-pack-drift-guard | binary_change |  |  |
| support/fix-ci-pinned-pack-drift-guard | auth |  |  |
| support/fix-ci-pinned-pack-drift-guard | data_migration |  |  |
| support/fix-ci-pinned-pack-drift-guard | ui |  |  |
| support/fix-ci-pinned-pack-drift-guard | flag_exists |  |  |
| support/fix-ci-pinned-pack-drift-guard | recent_deploy | ? |  |

---

## 14/20 — Contrast verification

`_archive/docs-site-brand-identity/contrast-verification`

**type:** project · **severity:** null · **status:** done

<details><summary>card summary (first lines of the body)</summary>

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
```

</details>

**Paths named in the card** (26) — an index into the text above, not a conclusion:

- `brand/README.md`
- `.bklg/docs-site-brand-identity/initiative.md`
- `.bklg/docs-site-brand-identity/_decomposition.md`
- `src/viz/html.ts`
- `.redkiln/config.yaml`
- `src/schema/config.ts`
- `references/audit/findings/12-docs-and-adoption.md`
- `.github/workflows/ci.yml`
- `.github/workflows/docs.yml`
- `test/**/*.test.ts`
- `.bklg/docs-site-brand-identity/contrast-verification/_intake-brief.md`
- `.bklg/docs-site-brand-identity/brand-token-substrate/_intake-brief.md`
- `.kb/decisions/brand-visual-identity-constraints.md`
- `.kb/open-questions/brand-unwired-from-pipeline.md`
- `.kb/README.md`
- `docs/src/styles/theme.css`
- `docs/src/layouts/DocsLayout.astro`
- `docs/src/pages/index.astro`
- `test/version-sync.test.ts`
- `.bklg/docs-site-brand-identity/_discovery/research/05-accent-budget-vs-ui-state.md`
- `.bklg/docs-site-brand-identity/_discovery/research/07-docs-retokenize-pain-points.md`
- `.bklg/docs-site-brand-identity/_discovery/research/08-interaction-pattern-prior-art.md`
- `workflows/forge-plan-design.js`
- `../_decomposition.md`
- …and 2 more

**Commits touching this item** (26) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `9a4d8fb2 fix(guards): guards that assert behaviour, and an exemption that is itself asserted`
- `98cfc709 fix(test): two guards that could not see the `build:docs` script for what it is`
- `6f1152a9 Merge branch 'main' into initiative/docs-site-brand-identity`
- `ba1ba83a docs(docs-site-brand-identity): mark RK-P0019 contrast-verification done`
- `f5a68e73 docs(docs-site-brand-identity): record RK-P0019 lock-contention pause`
- `d27748f8 chore(contrast-verification): record project review`
- `bed030ba chore(contrast-verification): record integration audit`
- `dd380d34 chore(contrast-verification): seal slice substrate-integration approved`
- …and 18 more

**Files changed** — 1255 total: **383 outside `.bklg/`**, 872 backlog bookkeeping.
**This is the diff — label from it.**

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
- …and 353 more outside `.bklg/`

> git reports **160** of these as binary.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/contrast-verification/project.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/contrast-verification | toolchain |  |  |
| _archive/docs-site-brand-identity/contrast-verification | change |  |  |
| _archive/docs-site-brand-identity/contrast-verification | binary_change |  |  |
| _archive/docs-site-brand-identity/contrast-verification | auth |  |  |
| _archive/docs-site-brand-identity/contrast-verification | data_migration |  |  |
| _archive/docs-site-brand-identity/contrast-verification | ui |  |  |
| _archive/docs-site-brand-identity/contrast-verification | flag_exists |  |  |
| _archive/docs-site-brand-identity/contrast-verification | recent_deploy | ? |  |

---

## 15/20 — Live views for the board

`_archive/live-views`

**type:** initiative · **severity:** null · **status:** archived

<details><summary>card summary (first lines of the body)</summary>

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
```

</details>

**Paths named in the card** (31) — an index into the text above, not a conclusion:

- `src/cli/index-access.ts`
- `docs/src/content/docs/explanation/design-principles.mdx`
- `src/process/engine.ts`
- `src/index/build.ts`
- `src/index/queries.ts`
- `_discovery/grounding/product-functional-alignment.md`
- `src/viz/html.ts`
- `src/store/ids.ts`
- `archive/live-updating-board-view-v0.3.1`
- `_discovery/grounding/backlog-adjacency.md`
- `.bklg/_archive/live-views/_discovery/distillation/personas-and-journeys.md`
- `.redkiln/config.yaml`
- `scripts/capture.mjs`
- `skills/conventions/SKILL.md`
- `.bklg/_archive/live-views/_discovery/distillation/interaction-patterns.md`
- `docs/src/content/docs/explanation/process-engine.mdx`
- `docs/src/content/docs/explanation/verify-gate.mdx`
- `docs/src/content/docs/explanation/the-index.mdx`
- `docs/src/content/docs/explanation/hierarchy-model.mdx`
- `docs/src/content/docs/explanation/telemetry-and-cross-branch.mdx`
- `docs/src/content/docs/how-to/visualize-the-backlog.mdx`
- `src/cli/view.ts`
- `src/viz/mermaid.ts`
- `.kb/playbooks/integration-first-vertical-slice.md`
- …and 7 more

**Commits touching this item** (74) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `a7afc9d0 fix(archive): one archive resolver, three consumers`
- `a259a670 test: guard every LIVE project's design surfaces, and stop a load-kill reading as a red`
- `6f1152a9 Merge branch 'main' into initiative/docs-site-brand-identity`
- `8eb7e534 fix: repair the repo after the live-views closeout, and correct two false docs claims`
- `f1e9acfb release: 0.19.0`
- `f3c5d08d fix(telemetry): reintegrate three orphaned RK-P0011 workflow_run records`
- `c3ee2def docs(live-views): repoint archived distillation frontmatter to _archive`
- `42ce6fc5 chore(live-views): archive completed initiative to _archive`
- …and 66 more

**Files changed** — 1905 total: **381 outside `.bklg/`**, 1524 backlog bookkeeping.
**This is the diff — label from it.**

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
- …and 351 more outside `.bklg/`

> git reports **320** of these as binary.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/live-views/initiative.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/live-views | toolchain |  |  |
| _archive/live-views | change |  |  |
| _archive/live-views | binary_change |  |  |
| _archive/live-views | auth |  |  |
| _archive/live-views | data_migration |  |  |
| _archive/live-views | ui |  |  |
| _archive/live-views | flag_exists |  |  |
| _archive/live-views | recent_deploy | ? |  |

---

## 16/20 — Docs site brand identity

`_archive/docs-site-brand-identity`

**type:** initiative · **severity:** null · **status:** archived

<details><summary>card summary (first lines of the body)</summary>

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
```

</details>

**Paths named in the card** (33) — an index into the text above, not a conclusion:

- `brand/README.md`
- `.kb/decisions/brand-visual-identity-constraints.md`
- `docs/src/pages/index.astro`
- `docs/src/layouts/DocsLayout.astro`
- `docs/src/styles/theme.css`
- `.kb/open-questions/brand-unwired-from-pipeline.md`
- `.kb/playbooks/brand-asset-usage.md`
- `src/viz/html.ts`
- `.redkiln/config.yaml`
- `workflows/forge-plan-design.js`
- `src/schema/config.ts`
- `.kb/README.md`
- `_discovery/distillation/personas-and-journeys.md`
- `_discovery/distillation/opportunities.md`
- `typography-delivery/project.md`
- `typography-delivery/_design.md`
- `scripts/observe-cold-load.mjs`
- `_discovery/distillation/interaction-patterns.md`
- `.kb/concepts/brand-mark-rationale.md`
- `.kb/maps/brand-identity-knowledge-map.md`
- `.kb/_governance/integration-waves/2026-07-31-intake/03-integration-summary.md`
- `brand/showcase.html`
- `brand/redkiln-lockup-{dark,light}.svg`
- `brand/redkiln-mark-{dark,light}.svg`
- …and 9 more

**Commits touching this item** (61) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `6f1f0342 Merge pull request #106 from Wet-Ink-Corporation/initiative/docs-site-brand-identity`
- `94fadbc5 Merge branch 'main' into initiative/docs-site-brand-identity`
- `6f1152a9 Merge branch 'main' into initiative/docs-site-brand-identity`
- `4e3238be fix: repoint source and test references into the archived initiative`
- `6d0d16c8 chore(docs-site-brand-identity): repoint .kb references to the archived path`
- `78ad864c chore(docs-site-brand-identity): archive completed initiative to _archive`
- `73608d9c docs(docs-site-brand-identity): record the initiative review roll-up`
- `e352e74e docs(docs-site-brand-identity): record the placeholder-closure ruling`
- …and 53 more

**Files changed** — 2623 total: **476 outside `.bklg/`**, 2147 backlog bookkeeping.
**This is the diff — label from it.**

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
- …and 446 more outside `.bklg/`

> git reports **448** of these as binary.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/_archive/docs-site-brand-identity/initiative.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity | toolchain |  |  |
| _archive/docs-site-brand-identity | change |  |  |
| _archive/docs-site-brand-identity | binary_change |  |  |
| _archive/docs-site-brand-identity | auth |  |  |
| _archive/docs-site-brand-identity | data_migration |  |  |
| _archive/docs-site-brand-identity | ui |  |  |
| _archive/docs-site-brand-identity | flag_exists |  |  |
| _archive/docs-site-brand-identity | recent_deploy | ? |  |

---

## 17/20 — Support

`support`

**type:** initiative · **severity:** null · **status:** exploring

<details><summary>card summary (first lines of the body)</summary>

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

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (72) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `a7afc9d0 fix(archive): one archive resolver, three consumers`
- `a7d55cab chore(backlog): close the sixteen fix-lane project shells`
- `f9e761b0 feat(validate): the item file is checked against the ledger, so a sed is caught`
- `939dc656 fix(verify): the memo key sees WHAT changed, not only which file changed`
- `eaf02339 fix(test): two guards whose subject was the old shape, not the promise`
- `e7ed2f28 chore(fix-serve-child-leak): track the RK-S0272 lane artifacts`
- `1ba293ca docs(fix-gate-honesty-sweep): scaffold the six-issue fix project`
- `4fe7e765 fix(ci): give the suite a real checkout, and silence node 22's sqlite warning`
- …and 64 more

**Files changed** — 63 total: **30 outside `.bklg/`**, 33 backlog bookkeeping.
**This is the diff — label from it.**

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

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/initiative.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support | toolchain |  |  |
| support | change |  |  |
| support | binary_change |  |  |
| support | auth |  |  |
| support | data_migration |  |  |
| support | ui |  |  |
| support | flag_exists |  |  |
| support | recent_deploy | ? |  |

---

## 18/20 — links can only be written by record-links, so a provenance gap is a process gap with no backfill affordance

`support/links-hand-edit-denied-with-no-backfill-path`

**type:** incident · **severity:** low · **status:** triage

<details><summary>card summary (first lines of the body)</summary>

```
## Summary

`links` is on the PreToolUse **DENY** list (`src/hook.ts:140`), so `record-links` is the sole write
path and a missing sha can never be patched by editing frontmatter. **This is a correct and
deliberate property of the single-write-path rule, and this item does not argue otherwise.** What is
filed is its *consequence*: a provenance gap is therefore a **process** gap, closable only by someone
deliberately running the command — and, combined with `links` being on no surface (RK-N0006) and the
provenance gate being off (same), nothing prompts anybody to run it. There is no backfill affordance:
no `--backfill`, no doctor repair, no reconciliation pass. Status: open, and filed **with the
disagreement recorded**.

Filed by **RK-S0124** `file-every-gap-as-a-support-item` from the census in
`.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md`, census gap id
`hand-edit-denied`. **Raised by: `RK-S0120`** (`enumerate-provenance-links`). **Grounding path:**
`.bklg/_archive/live-views/proof-and-readings/enumerate-provenance-links/_provenance-reading.md`.

## Findings Ledger

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **Instrument.** The PreToolUse write gate — `src/hook.ts:140`, the DENY list that makes `links` unwritable by hand. | `src/hook.ts:140`; the companion rule at `:179-181` (a `Write` to a canonical item file is always denied) | — |
| **Observation, verbatim from the reading.** *`links` is on the PreToolUse DENY list, so `record-links` is the sole write path and a missing sha can never be patched by editing frontmatter.* | `.bklg/_archive/live-views/proof-and-readings/enumerate-provenance-links/_provenance-reading.md`, `findings[].id: hand-edit-denied` (raised by `RK-S0120`) | — |
```

</details>

**Paths named in the card** (5) — an index into the text above, not a conclusion:

- `src/hook.ts`
- `.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md`
- `.bklg/_archive/live-views/proof-and-readings/enumerate-provenance-links/_provenance-reading.md`
- `.bklg/_archive/live-views/proof-and-readings/project.md`
- `.bklg/_archive/live-views/_implementation.md`

**Commits touching this item:** none found by slug or trailer at this pin.

**Files changed:** none resolvable at this pin — label from the card body, and use `?` freely.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/links-hand-edit-denied-with-no-backfill-path/incident.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/links-hand-edit-denied-with-no-backfill-path | toolchain |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | change |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | binary_change |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | auth |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | data_migration |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | ui |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | flag_exists |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | recent_deploy | ? |  |

---

## 19/20 — advance --commit aborts the git commit when a written path is gitignored

`support/advance-commit-ignored-path`

**type:** incident · **severity:** high · **status:** closed

<details><summary>card summary (first lines of the body)</summary>

```
## Summary

What is happening now, blast radius, and current status.

## Timeline

- Detected:
- Mitigated:
- Resolved:

## Impact

Affected systems, severity, and customer/user impact.

## Mitigation

Immediate actions taken to reduce impact.

## Notes

Links to dashboards, alerts, and related items.
```

</details>

**Paths named in the card:** none found by the index. Read the card body above.

**Commits touching this item** (1) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:

- `f32b4caf chore(support): file the two gate limitations blocking three proof-and-readings stories`

**Files changed** — 4 total: **0 outside `.bklg/`**, 4 backlog bookkeeping.
**This is the diff — label from it.**

- *(every changed file is `.bklg/` bookkeeping — this item records no code change at the pin)*

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/advance-commit-ignored-path/incident.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/advance-commit-ignored-path | toolchain |  |  |
| support/advance-commit-ignored-path | change |  |  |
| support/advance-commit-ignored-path | binary_change |  |  |
| support/advance-commit-ignored-path | auth |  |  |
| support/advance-commit-ignored-path | data_migration |  |  |
| support/advance-commit-ignored-path | ui |  |  |
| support/advance-commit-ignored-path | flag_exists |  |  |
| support/advance-commit-ignored-path | recent_deploy | ? |  |

---

## 20/20 — gitignored .redkiln/telemetry makes populated run data from committed telemetry unreachable

`support/telemetry-gitignored-so-runs-never-travel`

**type:** incident · **severity:** high · **status:** triage

<details><summary>card summary (first lines of the body)</summary>

```
> **WITHDRAWN 2026-08-11 — the observation below is no longer true of this repository.**
>
> This was filed while `.gitignore:14` still listed `.redkiln/telemetry/` on this branch. Merging
> `origin/main` (`3e931b64`) into `initiative/live-views` brought `.gitignore:12`'s
> **`TELEMETRY IS COMMITTED`** block, which removed the entry and names issue #65 as the reason.
> Both live probes now answer the opposite way here: `git check-ignore -v --no-index --
> .redkiln/telemetry/events/x.jsonl` matches nothing (exit 1), and `git ls-files .redkiln/telemetry`
> returns 22 tracked partitions where it previously returned none. DoD 13's *"from committed
> telemetry"* clause, which this item recorded as unreachable by construction, is reachable now.
>
> The originating finding (`gitignored-telemetry`, raised by `RK-S0121`) was re-settled from
> `confirmed` to `withdrawn`, and the census that filed this item stopped deriving the gap.
>
> **This is the recommended fix arriving, not a decision reversed.** RK-S0121's disposition of
> initiative AC-013 recommended `gitignore-change` and recorded that the repository owner had already
> made it on `main`. The merge is that change being inherited here, so the recommendation is
> **satisfied** — it is not withdrawn with this item.
>
> **The record below is kept verbatim, not rewritten.** Full withdrawal record, with evidence and
> decision owner, in
> `.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md` under
> `withdrawn[]`.
```

</details>

**Paths named in the card** (8) — an index into the text above, not a conclusion:

- `.bklg/_archive/live-views/proof-and-readings/file-every-gap-as-a-support-item/_filings.md`
- `.bklg/_archive/live-views/proof-and-readings/settle-the-instrument-findings/_findings.md`
- `src/store/init.ts`
- `src/store/run-log.ts`
- `.bklg/_archive/live-views/proof-and-readings/project.md`
- `.bklg/_archive/live-views/_implementation.md`
- `test/support-filings.test.ts`
- `src/hook.ts`

**Commits touching this item:** none found by slug or trailer at this pin.

**Files changed:** none resolvable at this pin — label from the card body, and use `?` freely.

**Read the whole card if a row is unclear:**

```bash
git -C D:/repos/redkiln show 47faec9f:.bklg/support/telemetry-gitignored-so-runs-never-travel/incident.md
```

| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| support/telemetry-gitignored-so-runs-never-travel | toolchain |  |  |
| support/telemetry-gitignored-so-runs-never-travel | change |  |  |
| support/telemetry-gitignored-so-runs-never-travel | binary_change |  |  |
| support/telemetry-gitignored-so-runs-never-travel | auth |  |  |
| support/telemetry-gitignored-so-runs-never-travel | data_migration |  |  |
| support/telemetry-gitignored-so-runs-never-travel | ui |  |  |
| support/telemetry-gitignored-so-runs-never-travel | flag_exists |  |  |
| support/telemetry-gitignored-so-runs-never-travel | recent_deploy | ? |  |

