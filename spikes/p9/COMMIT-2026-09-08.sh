#!/usr/bin/env bash
# Commit set for the 2026-09-08 session: P6 authoring batch + lattice (part 1), then decisions 1–10 applied (part 2).
# Assumes spikes/p9/COMMIT-2026-09-07.sh has been committed first.
#
# CRLF churn: `git status` shows whole-file M on LICENSE, .gitignore, references/2026-08-31-*.md, references/2026-09-0{1,2,3,4}-*.md,
# references/conformance-*.md, references/research/2026-09-03-*.md, spikes/FINDINGS.md etc. with no content change (line-ending
# normalisation on this Windows checkout). Check with `git diff -w --stat`; `git checkout -- <file>` those or set core.autocrlf BEFORE
# staging. Nothing below touches them. Do NOT stage catalog-b/, lint-v1.py, F32/F33.
# NOTE: METHOD-P.md and references/2026-09-07-activity-graph-schema.md ARE staged here — edited by Claude on Ryan's explicit
# authorization (amendments 6–7; §9c), see spikes/p9/RYAN-TODO-2026-09-08.md.
set -e
git add spikes/F34-DRAFT.md spikes/F35-DRAFT.md spikes/HANDOFF.md spikes/METHOD-P.md \
        references/2026-09-07-activity-graph-schema.md \
        spikes/p9/catalog/ spikes/p9/packs/ spikes/p9/lattice.py spikes/p9/lattice-v1.py spikes/p9/reach-check.py \
        spikes/p9/lint.py spikes/p9/lint-v2.py spikes/p9/registry-v2.toml \
        spikes/p9/facets.toml spikes/p9/questions.toml spikes/p9/schemas.toml spikes/p9/registry.toml spikes/p9/artifacts.toml \
        spikes/p9/lines-v0.txt spikes/p9/results-v2-batch-*.txt spikes/p9/results-v2-catalog-full.txt spikes/p9/results-v3-catalog-run*.txt \
        spikes/p9/results-p6-*.txt spikes/p9/DECISIONS-2026-09-08.md spikes/p9/RYAN-TODO-2026-09-08.md spikes/p9/COMMIT-2026-09-08.sh \
        spikes/p1/ spikes/p2/ spikes/results/p1-arm-d-labels-BLANK.md spikes/results/p1-arm-l-labels-BLANK.md spikes/results/p2-labels-BLANK.md
git commit -m "P6: v0 catalog (102 activities), lattice engine + 12 packs, F34/F35, decisions 1–10 applied, P1-D/P2 offline tooling

Part 1 — authoring + lattice (F34/F35 §1–5): 90 new activities, per-batch first-run lint outputs, lattice v1 (results-p6-run0..run3),
  8 pre-registered + 4 supplementary packs, P1 arm D / P2 scripts + blank label templates (unrun: corpus not mounted)
Part 2 — decisions (spikes/p9/DECISIONS-2026-09-08.md, Ryan-authorized): schema §9c (amendments 26–43); METHOD-P amendments 6–7
  (post-data, additive); artifacts.toml; typed [verb] registry (record removed → 12 composing emits); recent_deploy facet;
  lint v3 (v2 frozen); family keys + max_rigor; implement-ue; 46 version bumps; lattice v2 (v1 frozen; --validate; 3 bands +
  unknown sweep; keyed instances). P6 run4→run9: 0.3% unsatisfiable, mixed = node, dead 6/102, ties 0 (F35 §1.6); Ryan reviewed R1–R7 (DECISIONS §Review)"
