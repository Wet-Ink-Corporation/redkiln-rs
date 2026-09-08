#!/usr/bin/env bash
# Commit set for the 2026-09-07 process-catalog session. Run from repo root on Windows/Git Bash.
# Excludes the CRLF-churned tracked files (LICENSE, older references, spikes/METHOD*.md etc.) — check `git diff --stat` first;
# if those show whole-file diffs with no content change, `git checkout -- <file>` them or set core.autocrlf before staging.
set -e
git add references/2026-09-07-process-catalog-proposal.md \
        references/2026-09-07-activity-graph-schema.md \
        references/research/2026-09-07-method-engineering.md \
        references/research/2026-09-07-selection-and-planning.md \
        references/research/2026-09-07-competitive-workflows.md \
        references/research/2026-09-07-activity-catalog-raw.md \
        references/2026-09-05-architecture-proposal.md \
        spikes/METHOD-P9.md spikes/METHOD-P.md spikes/F32-DRAFT.md spikes/F33-DRAFT.md spikes/HANDOFF.md \
        spikes/p9/
git commit -m "Process catalog: PC1–PC8, activity-graph schema (§9a/§9b), P9/P9b (F32/F33), METHOD-P, lint v2, D20–D22

- references: catalog proposal (artifact build-graph selection, rigor enum, PC1–PC8, spikes P1–P9),
  activity-graph schema with §9a (P9) and §9b (P9b) amendments, four research notes
- architecture proposal: D20 rigor enum, D21 loop-aware memo key, D22 encapsulated activities / tech-pack contract
- spikes: METHOD-P9 + F32 (P9 authoring ergonomics), F33 (P9b second author), METHOD-P (P1/P2/P6/P7/P8 pre-registration),
  p9/ catalog (10, v2-clean), catalog-b (3, second author's record), lint v1 (frozen) + v2, registries, result files
- HANDOFF: P-series section"
