# Step 7e §7e.2 — row dispositions, 2026-09-21

Every pack and catalog row in the **installed** registry (`.redkiln/`) that keys on a facet
whose source moved at Step 4a, read once and given a written disposition (PIA-011).

## The three greps, re-derived

§7e.2 says the counts in the runbook are *"the audit's at the pin"* and instructs that they be
re-derived. They were, against the tree as it stood at `f9b07b5` before §7e.2 ran, restored from
git and scoped to what `.redkiln/` actually holds — the **two** installed packs (ADR-0011, E7)
and the 102 `catalog/` activities. `catalog-b/` is not copied (ADR-0027) and is out of scope.

| grep | subject | rows |
| --- | --- | --- |
| 1 | the seven axes Step 4a moved to `source = "human"` | **50** |
| 2 | the withdrawn `auth` boolean | **7** |
| 3 | the scored-derived facets S-FACET gates (`ui`, `flag_exists`, `data_migration`) | **15** |

**Naive sum 72; distinct rows 68.** Two rows match two greps each —
`packs/story.toml`'s `[[stage.design.require]]` and `catalog/threat-model.toml`'s
`applicable_if`, both reading `auth or data_migration or external_dep`. Checkpoint 7e line 143's
`# →` says the row count *"equals the three greps' sum"*; **it is 68, not 72**, and the
difference is exactly that double-count.

The runbook's own figures (72 for grep 1, 15 for grep 2) are whole-p9-tree readings: over all
twelve packs plus `catalog/` the three greps measure **101 / 15 / 31**.

## Summary

| disposition | rows |
| --- | --- |
| keep | 48 |
| drop | 9 |
| null-guarded | 7 |
| rewrite to `surfaces` | 2 |
| narrow | 1 |
| rewrite to `surfaces` + null-guarded | 1 |
| **total** | **68** |

## The rows

Line numbers are the row's line in the **pristine** file, before §7e.2 edited it.

| file | line | table | key | predicate | disposition | rule |
| --- | --- | --- | --- | --- | --- | --- |
| `packs/bug-standard.toml` | 14 | `stage.analysis.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 25 | `stage.design.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 32 | `stage.planning.require` | `when` | `blast_radius >= med` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 35 | `stage.planning.require` | `when` | `size >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 48 | `stage.verify.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 55 | `stage.verify.require` | `when` | `criticality >= high and rigor >= mandatory` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 64 | `stage.review.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 68 | `stage.review.require` | `when` | `auth` | **drop** | `auth` is withdrawn from the registry entirely; a pack `require` table has `when`, not `applicable_if`, so the rewrite has no home here. The demand rides on the catalog activity's own `applicable_if` (ADR-0089 d2) |
| `packs/bug-standard.toml` | 71 | `stage.review.require` | `when` | `blast_radius >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/bug-standard.toml` | 78 | `stage.release.require` | `when` | `data_migration` | **drop** | data_migration is `deriver_class = "lexical"` and NOT MEASURABLE at the pin and may key NO `require` row (AC-520; ADR-0093 d2). S-FACET gates it, so §7e's acceptance set excludes it |
| `packs/bug-standard.toml` | 89 | `stage.closeout.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 14 | `stage.specification.require` | `when` | `auth` | **drop** | `auth` is withdrawn from the registry entirely; a pack `require` table has `when`, not `applicable_if`, so the rewrite has no home here. The demand rides on the catalog activity's own `applicable_if` (ADR-0089 d2) |
| `packs/story.toml` | 17 | `stage.specification.require` | `when` | `external_dep` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 24 | `stage.design.require` | `when` | `auth or data_migration or external_dep` | **narrow** | auth/data_migration barred from a `require` row (ADR-0089 d2); external_dep is `human` and survives, so the row narrows rather than dropping |
| `packs/story.toml` | 27 | `stage.design.require` | `when` | `data_migration` | **drop** | data_migration is `deriver_class = "lexical"` and NOT MEASURABLE at the pin and may key NO `require` row (AC-520; ADR-0093 d2). S-FACET gates it, so §7e's acceptance set excludes it |
| `packs/story.toml` | 30 | `stage.design.require` | `when` | `ui` | **drop** | ui is `deriver_class = "lexical"` and may key NO `require` row (AC-520; ADR-0093 d2). S-FACET gates it, so §7e's acceptance set excludes it |
| `packs/story.toml` | 33 | `stage.design.require` | `when` | `size >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 36 | `stage.design.require` | `when` | `novelty >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 43 | `stage.planning.require` | `when` | `blast_radius >= med` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 50 | `stage.implement.require` | `when` | `blast_radius >= med` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 53 | `stage.implement.require` | `when` | `data_migration` | **drop** | data_migration is `deriver_class = "lexical"` and NOT MEASURABLE at the pin and may key NO `require` row (AC-520; ADR-0093 d2). S-FACET gates it, so §7e's acceptance set excludes it |
| `packs/story.toml` | 65 | `stage.verify.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 72 | `stage.verify.require` | `when` | `data_migration` | **drop** | data_migration is `deriver_class = "lexical"` and NOT MEASURABLE at the pin and may key NO `require` row (AC-520; ADR-0093 d2). S-FACET gates it, so §7e's acceptance set excludes it |
| `packs/story.toml` | 75 | `stage.verify.require` | `when` | `external_dep` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 78 | `stage.verify.require` | `when` | `criticality >= high and rigor >= mandatory` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 87 | `stage.review.require` | `when` | `criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 91 | `stage.review.require` | `when` | `auth` | **drop** | `auth` is withdrawn from the registry entirely; a pack `require` table has `when`, not `applicable_if`, so the rewrite has no home here. The demand rides on the catalog activity's own `applicable_if` (ADR-0089 d2) |
| `packs/story.toml` | 94 | `stage.review.require` | `when` | `blast_radius >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `packs/story.toml` | 101 | `stage.release.require` | `when` | `data_migration` | **drop** | data_migration is `deriver_class = "lexical"` and NOT MEASURABLE at the pin and may key NO `require` row (AC-520; ADR-0093 d2). S-FACET gates it, so §7e's acceptance set excludes it |
| `packs/story.toml` | 108 | `stage.operate.require` | `when` | `blast_radius >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `catalog/api-contract-spec.toml` | 10 | `activity` | `applicable_if` | `external_dep or kind == project` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/appetite-check.toml` | 41 | `step` | `when` | `${size.outputs.fits_appetite} == false` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `catalog/architecture-tradeoff-analysis.toml` | 11 | `activity` | `applicable_if` | `criticality >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/bisect.toml` | 12 | `activity` | `applicable_if` | `has_repro` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/blameless-postmortem.toml` | 12 | `activity` | `applicable_if` | `kind in [incident, 'security-vuln'] or criticality >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/blue-green-deploy.toml` | 11 | `activity` | `applicable_if` | `blast_radius >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/branch-by-abstraction.toml` | 11 | `activity` | `applicable_if` | `size >= high and blast_radius < high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/c4-component.toml` | 12 | `activity` | `applicable_if` | `size >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/canary-release.toml` | 9 | `activity` | `applicable_if` | `blast_radius >= med` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/change-authorization.toml` | 10 | `activity` | `applicable_if` | `kind == incident or blast_radius >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/competitor-analysis.toml` | 9 | `activity` | `applicable_if` | `market_facing` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/data-migration-plan.toml` | 10 | `activity` | `applicable_if` | `data_migration` | **null-guarded** | data_migration is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/data-migration-plan.toml` | 58 | `step` | `when` | `rigor >= recommended or blast_radius >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `catalog/design-sprint.toml` | 12 | `activity` | `applicable_if` | `novelty >= high and criticality >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/exploitability-analysis.toml` | 57 | `step` | `when` | `rigor >= recommended or criticality >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `catalog/failover.toml` | 12 | `activity` | `applicable_if` | `blast_radius >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/fault-tree-analysis.toml` | 11 | `activity` | `applicable_if` | `criticality >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/feature-flag-kill.toml` | 12 | `activity` | `applicable_if` | `flag_exists` | **null-guarded** | flag_exists is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/feature-flag-wrap.toml` | 10 | `activity` | `applicable_if` | `blast_radius >= med` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/flag-progressive-rollout.toml` | 12 | `activity` | `applicable_if` | `flag_exists` | **null-guarded** | flag_exists is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/formal-inspection.toml` | 11 | `activity` | `applicable_if` | `criticality >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/integration-tests.toml` | 10 | `activity` | `applicable_if` | `kind == project or external_dep` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/migration-dry-run.toml` | 10 | `activity` | `applicable_if` | `data_migration` | **null-guarded** | data_migration is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/migration-expand-execute.toml` | 10 | `activity` | `applicable_if` | `data_migration` | **null-guarded** | data_migration is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/property-based-testing.toml` | 11 | `activity` | `applicable_if` | `'text' in change and criticality >= high and novelty >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/prototype-test.toml` | 11 | `activity` | `applicable_if` | `novelty >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/rate-limit-shed.toml` | 12 | `activity` | `applicable_if` | `blast_radius >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/rolling-deploy.toml` | 12 | `activity` | `applicable_if` | `blast_radius < med or rigor < recommended or kind == incident"   # P6 run1 fix: incident release at high blast radius` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/rollout-plan.toml` | 48 | `step` | `when` | `rigor >= mandatory or blast_radius >= high` | **keep** | a `require` row may key on a `human` facet freely -- golden constraint 1 forbids LLM-SOURCED axes, not human ones (ADR-0090; ADR-0079's honest destination, now superseded by 0090) |
| `catalog/schema-migration-script.toml` | 10 | `activity` | `applicable_if` | `data_migration` | **null-guarded** | data_migration is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/security-requirements.toml` | 10 | `activity` | `applicable_if` | `auth or kind == 'security-vuln'` | **rewrite to `surfaces`** | `auth` withdrawn -> `'auth' in surfaces`; authored in an applicable_if, which is the only place ADR-0089 d2 admits it. DORMANT until `surfaces.toml`'s boundary map lands (S-FACET, unrun) |
| `catalog/security-review.toml` | 10 | `activity` | `applicable_if` | `auth or kind == 'security-vuln'` | **rewrite to `surfaces`** | `auth` withdrawn -> `'auth' in surfaces`; authored in an applicable_if, which is the only place ADR-0089 d2 admits it. DORMANT until `surfaces.toml`'s boundary map lands (S-FACET, unrun) |
| `catalog/strangler-fig.toml` | 11 | `activity` | `applicable_if` | `size >= high and blast_radius >= high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/t-shirt-sizing.toml` | 11 | `activity` | `applicable_if` | `size < high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/threat-model.toml` | 9 | `activity` | `applicable_if` | `auth or data_migration or external_dep` | **rewrite to `surfaces` + null-guarded** | `auth` -> `'auth' in surfaces` (ADR-0089 d2); data_migration null-guarded (ADR-0103 cl.4); external_dep kept -- now `human` (ADR-0090) |
| `catalog/unit-tests.toml` | 11 | `activity` | `applicable_if` | `'text' in change"   # 2026-09-08: escalation by rigor (max_rigor), not criticality` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |
| `catalog/ux-flow-design.toml` | 10 | `activity` | `applicable_if` | `ui` | **null-guarded** | ui is tri-valued since ADR-0093; the predicate handles `null` explicitly (ADR-0103 cl.4's narrowing form) |
| `catalog/walking-skeleton.toml` | 11 | `activity` | `applicable_if` | `size < high` | **keep** | keys only on facets Step 4a moved to `human`; an `applicable_if` may read them freely (ADR-0090) |

## Two things this table records that the runbook does not

**The `surfaces` rewrite is DORMANT, deliberately.** `[facet.surfaces]`'s value set was a
declared placeholder — `flags`, `migration`, `payments`, `secrets` — and did **not** contain
`auth`; `surfaces.toml`, which ADR-0089 decision 2 makes the source of that set, does not exist
anywhere in the repo, and S-FACET (the spike that would score a declared boundary map) is unrun.
A rewritten row would therefore have linted green and never fired, silently disabling the three
security activities. `auth` was added to the value set at Step 7e — §7e.2's own table names it
as the surface value, so that is a citation rather than an invention — and the rows are
authored, linted and **inert** until the boundary map lands.

**The installed tree and the spike tree diverge, deliberately.** `.redkiln/`'s three security
`applicable_if` rows carry `'auth' in surfaces`; `spikes/p9/`'s carry the `auth` term **removed**
instead. `lattice.py` has no `surfaces` axis and dies `KeyError: 'surfaces'` on the rewritten
form, so the instrument §7e.4 must run cannot read the product's disposition. The spike tree is
the measurement input; the installed tree is the product.

**The form is `'auth' in surfaces`, not §7e.2's `any(s in surfaces, s == "auth")`.** Measured:
`RK-D-FACET-UNKNOWN` reads the quantifier's bound variable `s` as a free facet reference and
fails the row at load under AC-072. `'<value>' in <set-facet>` is the idiom every shipped row
already uses (`'text' in change`, `'unreal' in toolchain`), it reads only the registered
`surfaces` facet, and it means the same thing.
