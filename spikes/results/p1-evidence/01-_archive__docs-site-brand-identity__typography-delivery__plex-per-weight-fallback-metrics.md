# 1/20 — _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics

**kind:** bug · **commits:** 1 · **files from commits:** 0 · **paths named only in the card:** 8

## Files changed (outside .bklg/)

- `scripts/observe-cold-load.mjs`  [named in the card; no commit found]
- `cold-load-delivery-behaviour/evidence/README.md`  [named in the card; no commit found]
- `cold-load-delivery-behaviour/evidence/pair-review.md`  [named in the card; no commit found]
- `src/store/verify.ts`  [named in the card; no commit found]
- `src/store/actor.ts`  [named in the card; no commit found]
- `scripts/measure-font-metrics.mjs`  [named in the card; no commit found]
- `cold-load-delivery-behaviour/spec.md`  [named in the card; no commit found]
- `test/typography-delivery.test.ts`  [named in the card; no commit found]

## Card body (the item as written)

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

Expected: every declared pair (docs-prose-type, docs-code-type, command-block-type, landing-hero-type,
topbar-type, search-modal-type — all viewports, both cuts) breaks at the same word and places every
heading baseline within 2px between the fallback-only and faces-loaded frames.

Observed: 22 of 28 pairs are clean; the residual is concentrated in `docs-prose-type` (both viewports,
both cuts) and `command-block-type@390x844` (both cuts).

## Reproduction

Run `node scripts/observe-cold-load.mjs` against the built `docs/dist` site (see
`cold-load-delivery-behaviour/evidence/README.md` for the harness) and read
`cold-load-delivery-behaviour/evidence/pair-review.md` — it enumerates the exact blocks, viewports and
cuts where line count or break-word diverges, with the measured pixel deltas.

## Impact

A visual reflow on font swap, on a fraction of surfaces, at specific viewports. It does not affect
deep-link landing (0px drift — AC-004) or selection/focus survival (AC-005), and the cold-load shift
figure is now measured ON the surfaces the residual lives on rather than only at 1440x900 on two
surfaces that do not carry it — see `cold-load-delivery-behaviour/evidence/pair-review.md`, which
tabulates the CLS reading per surface and viewport.

**There is no waiver.** The AC-010 row was briefly recorded as `waived_by: "Ryan Britton"` while the
acting actor resolved (from `git config user.email`) to `ryan-britton`; `src/store/verify.ts`'s
self-grant check passed it only because "Ryan Britton" and "ryan-britton" differ by one character of
punctuation. That check now normalises both sides (`normalizeActorName`, `src/store/actor.ts`), the
waiver no longer stands, and the AC-010 row carries `satisfied: false` with no waiver at all. It stays
that way until a human grants the acceptance through a channel an agent cannot author — a
`redkiln advance` transition record, a signed design-gate entry, or their own message quoted with its
provenance. Severity `medium`: real, bounded, and now BLOCKING RK-P0022's
`identity-faces-on-both-surfaces` slice rather than absorbed by it.

## Acceptance Criteria

- AC-001: a second, per-weight override set is derived and authored (extending
  `scripts/measure-font-metrics.mjs`'s single-scalar-per-family approach to also account for the 600/700
  weight step), and the AC-010 pair review in `cold-load-delivery-behaviour/evidence/pair-review.md`
  (or its successor) shows 28/28 pairs clean on both anti-pattern 1 sub-claims (line count and
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
1fd8c145 docs(typography-delivery): waive AC-010 anti-pattern-1 residual, file RK-B0014

```
