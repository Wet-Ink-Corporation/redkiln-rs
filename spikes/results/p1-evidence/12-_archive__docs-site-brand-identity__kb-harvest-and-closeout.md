# 12/20 — _archive/docs-site-brand-identity/kb-harvest-and-closeout

**kind:** project · **commits:** 29 · **files from commits:** 0 · **paths named only in the card:** 12

## Files changed (outside .bklg/)

- `src/store/verify.ts`  [named in the card; no commit found]
- `.redkiln/config.yaml`  [named in the card; no commit found]
- `src/schema/kb.ts`  [named in the card; no commit found]
- `src/viz/html.ts`  [named in the card; no commit found]
- `workflows/forge-plan-design.js`  [named in the card; no commit found]
- `docs/package.json`  [named in the card; no commit found]
- `src/schema/config.ts`  [named in the card; no commit found]
- `references/audit/findings/12-docs-and-adoption.md`  [named in the card; no commit found]
- `brand/README.md`  [named in the card; no commit found]
- `docs/src/content/docs/**/*.mdx`  [named in the card; no commit found]
- `.redkiln/processes/project.yaml`  [named in the card; no commit found]
- `.github/workflows/docs.yml`  [named in the card; no commit found]

## Card body (the item as written)

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
building the substrate; this project closes the *durable* half. Without it, the artifact that sent
the Steward looking in the first place —
[`.kb/open-questions/brand-unwired-from-pipeline.md`](../../../../.kb/open-questions/brand-unwired-from-pipeline.md),
`status: proposed`, with **no mechanism to update itself** — still says the question is open, and
sends the next reader hunting for a decision that has already been made. A stale open question is
worse than none.

It also delivers the initiative's two "nothing else moved / it still ships" criteria (AC-13 and DoD
14/15), which are meaningful only against the fully assembled site, and the exit criteria's
knowledge-base obligations, which are post-hoc by construction.

Owns **BR-17**, initiative **AC-12** and **AC-13**, scope letter **I**, and DoD scenarios **14** and
**15** ([`../_decomposition.md`](../_decomposition.md), traceability matrix and DoD coverage). It
owns **no** design tension: DT-1 … DT-8 are all assigned to siblings.

## In scope (this project)

- **Resolving the open question of record.** `kb-open-question-brand-unwired-from-pipeline` is
  either amended in place with the answer recorded, or marked `status: superseded` with
  `superseded_by` naming a new atom. One of the two, deliberately chosen and stated — not left
  `proposed`. Its four numbered sub-questions are each addressed against what the initiative
  actually decided (items 1 and 2 by RK-P0018/RK-P0020, item 4 by RK-P0023; item 3 is answered as
  scoped, since `DS_TOOLING` was deliberately not edited).
- **Harvesting the four durable, cross-surface decisions** taken along the way into `.kb/` atoms,
  each cited back to the owning project's `_design.md`:
  1. the **general-highlight concept** and its relationship to the two-instance accent budget
     (RK-P0020, DT-2/DT-3/DT-4);
  2. the **authored-vs-derived rule** for brand values — one authored home, everything else derived
     (RK-P0018, DT-1);
  3. the **per-cut contrast posture** — an ongoing property over declared pairs, mark and lockup
     exempt, reporting which threshold failed (RK-P0019);
  4. the **seam with the component library** — where the brand's roles and Radix's scale contract
     disagree, and what was decided (RK-P0020, BR-14).
- **Reconciling the index atom.**
  [`.kb/maps/brand-identity-knowledge-map.md`](../../../../.kb/maps/brand-identity-knowledge-map.md)
  currently describes the open question as `proposed` and carries its bullet inline; the map must
  reflect whichever resolution was taken and index the new atoms. The map's own note ("revisit once
  the corpus holds three or four open questions") is a real hook to evaluate, not to ignore.
```

## Diff (code only, .bklg/ bookkeeping excluded)

```diff
f2d2f4e7 docs(kb-harvest-and-closeout): re-score the project review to approved

```
