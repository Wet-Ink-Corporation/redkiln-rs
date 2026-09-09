# 2/20 — _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre

**kind:** bug · **commits:** 0 · **files from commits:** 0 · **paths named only in the card:** 3

## Files changed (outside .bklg/)

- `docs/dist/how-to/advance-items/index.html`  [named in the card; no commit found]
- `docs/src/styles/theme.css`  [named in the card; no commit found]
- `test/docs-brand.test.ts`  [named in the card; no commit found]

## Card body (the item as written)

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
`rgb(16,13,11)` → `rgb(16,16,16)` = **1.02:1**. The same ring measures **17.67:1** in the light cut.

Expected: every keyboard stop shows a focus indicator that is visible and distinguishable **in the cut
it appears in** — initiative AC-5 and DoD 10.

## Reproduction

1. `cd docs && pnpm build` (Astro + Pagefind), then serve `docs/dist` under base `/redkiln`.
2. Open any page carrying a fenced code block (e.g. `/redkiln/how-to/advance-items/`) in the **dark**
   cut.
3. Tab until focus lands on a scrollable `<pre>`.
4. Observe the focus indicator. Compare against the same stop in the light cut.

Deterministic — not a flake. Both cuts reproduce on every run; only the dark cut fails.

## Impact

**High.** A keyboard user in the dark cut cannot see where focus is on 22 stops per page-traverse.
This is precisely the failure mode the initiative charter rates High/Critical — *"dark-cut contrast
failure ships, because the light cut passed and symmetry was assumed"* — and it reached a green
project review, a green project integration gate and closeout on RK-P0020 without being seen.

The reason it was not seen is the second half of this bug: **the guard measures the wrong thing.**
`TAB_STOPS` at [`test/docs-brand.test.ts`](../../../../../test/docs-brand.test.ts):1131 is a hand-written
list of the same seven selectors, checked against the stylesheet at :3066. It can only ever confirm
that the stylesheet styles the stops someone already thought of; it structurally cannot discover a
stop injected at build time. A fix that adds the selector without replacing that fixture leaves the
repository unable to catch the next instance.

## Acceptance Criteria

- **AC-001** — Every keyboard-focusable element on a built content page shows a focus indicator
  meeting its contrast threshold **in both cuts**, including scrollable `<pre>` blocks. Measured, not
  asserted.
- **AC-002** — The dark-cut focused-versus-unfocused delta on a scrollable `<pre>` is no longer
  1.02:1; it meets the same bar the other stops are held to. The light cut does not regress from
  17.67:1.
- **AC-003** — The guard enumerates tab stops from a **real traverse of the built site**, not from a
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
```
