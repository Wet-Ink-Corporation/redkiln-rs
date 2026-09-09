# 4/20 — support/phase-1-total-outcomes/gate-passes-its-known-base

**kind:** story · **commits:** 0 · **files from commits:** 0 · **paths named only in the card:** 2

## Files changed (outside .bklg/)

- `src/process/gates.ts`  [named in the card; no commit found]
- `src/store/verify.ts`  [named in the card; no commit found]

## Card body (the item as written)

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
`verifyIdentity(root, spec.item, spec.grain, spec.base)` — it had `spec.base` in
hand and passed it to `verifyCacheKey` on the very next line.

The parameter is **required, not optional**: an optional `base?` is what let this
call site omit it silently. The residual note in `src/store/verify.ts` that
described exactly this window is shortened, as the issue asks.

Guard: `AC-MEMO-BASE-THREADED`.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
```
