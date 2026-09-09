#!/usr/bin/env bash
# Commit set for the 2026-09-08/09 AFK session: P1 arm D + P2 offline results, P1 arm L (two instruments,
# 400 dispatches), P7 complete (61 graph runs), P8 partial (3 resume cycles), and three new rig subcommands.
# Assumes spikes/p9/COMMIT-2026-09-08.sh has been committed first (HEAD ab58733 already carries it).
#
# CRLF churn: `git status` shows whole-file M on LICENSE, .gitignore, references/2026-08-31-*.md,
# references/2026-09-0{1,2,3,4}-*.md, references/conformance-*.md, references/research/2026-09-03-*.md,
# spikes/FINDINGS.md etc. with no content change (line-ending normalisation on this Windows checkout).
# Check with `git diff -w --stat`; `git checkout -- <file>` those or set core.autocrlf BEFORE staging.
# Nothing below touches them. Do NOT stage catalog-b/, lint-v1.py, lint-v2.py, lattice-v1.py, F32/F33.
#
# NOTHING IN THIS SESSION EDITED A METHOD FILE, A PRE-REGISTRATION, OR THE CATALOG. METHOD-P.md,
# METHOD-E3.md, F34, F35, p9/catalog/ and p9/packs/ are byte-identical to HEAD — verified with
# `git diff --quiet HEAD -- <those paths>`. Every interpretation went into spikes/DECISIONS-2026-09-09.md
# instead (AFK brief rule 2). The ONE exception Ryan must approve is the HANDOFF instrument pin, which
# moves 2.1.259 -> 2.1.263 (decision 1) — and note the CLI moved again to 2.1.265 mid-P7 (F37 §7).
#
# NOT STAGED, deliberately:
#   spikes/fixtures/**            — fixture clones (gitignored); regenerable and large
#   spikes/results/p8-state/**    — suspended graph state + gate answers; regenerable, and meaningless
#                                   without the run that made it. The digests that matter are IN p8.jsonl.
#   spikes/runner/target-dev/**   — a second build dir: the main target/ was locked by a live run
#   spikes/runner/target-p8/**    — a third, same reason. NEITHER IS GITIGNORED, so both show as untracked
#                                   noise until removed: `rm -rf spikes/runner/target-dev spikes/runner/target-p8`
#                                   (or add `target-*/` to .gitignore — but .gitignore is CRLF-churned, so
#                                   stage that separately).
#   spikes/*.log                  — run logs are tee'd for the waiters, not evidence; the JSONL is the evidence
set -e

# --- part 1: the offline lanes (zero model cost) ---------------------------------------------------
git add spikes/p1/p1-select-items.py spikes/p1/derive-facets.py spikes/p1/score-arm-d.py \
        spikes/p2/p2-select-items.py spikes/p2/reconstruct-derived-from.py \
        spikes/plans/p1-items.txt spikes/plans/p2-items.txt \
        spikes/results/p1-arm-d.jsonl spikes/results/p2-reconstruct.jsonl spikes/results/p2-cases.jsonl

# --- part 2: the label files Ryan fills ------------------------------------------------------------
git add spikes/results/p1-arm-d-labels-RYAN-TODO.md \
        spikes/results/p1-arm-l-labels-RYAN-TODO.md \
        spikes/results/p2-labels-RYAN-TODO.md \
        spikes/results/p7-deviation-labels-RYAN-TODO.md

# --- part 3: P1 arm L — two instruments, 400 dispatches, 0 void -------------------------------------
git add spikes/prompts/p1-classify.md spikes/prompts/p1-classify-v2.md \
        spikes/schemas/p1-classify.schema.json \
        spikes/p1/analyse-arm-l.py spikes/p1/run-arm-l.ps1 spikes/p1/run-arm-l-v2.ps1 \
        spikes/results/p1-arm-l.jsonl spikes/results/p1-arm-l-analysis.txt \
        spikes/results/p1-arm-l-v2.jsonl spikes/results/p1-arm-l-v2-analysis.txt

# --- part 4: the rig — three new subcommands -------------------------------------------------------
git add spikes/runner/src/main.rs spikes/runner/src/p1.rs spikes/runner/src/p7.rs spikes/runner/src/p8.rs

# --- part 5: P7 (complete) and P8 (partial) --------------------------------------------------------
git add spikes/plans/p7-items.txt spikes/plans/p8-items.txt spikes/plans/p8-24h-resume.md \
        spikes/p7/run-p7.ps1 spikes/p7/analyse-p7.py spikes/p8/run-p8-cycle.ps1 \
        spikes/results/p7.jsonl spikes/results/p7-analysis.txt \
        spikes/results/p8.jsonl

# --- part 6: the write-ups --------------------------------------------------------------------------
git add spikes/DECISIONS-2026-09-09.md \
        spikes/F36-DRAFT.md spikes/F37-DRAFT.md spikes/F38-DRAFT.md \
        spikes/HANDOFF.md spikes/plans/AFK-2026-09-09.md spikes/p9/COMMIT-2026-09-09.sh

git commit -m "P1 arm L x2 (400 dispatches, 0 void), P7 complete (61 runs), P8 partial, three rig subcommands

Spend \$156.00 of the \$300 night cap. Instrument: claude 2.1.263, not the pinned 2.1.259 — recorded on every
record and the pin moved (decision 1), because the rig's own doctor treats 2.1.259 as a floor. It moved AGAIN to
2.1.265 during P7, asymmetrically (F37 section 7).

P1 arm D / P2 (offline, \$0): 640 arm-D records over 20 items x 4 path sources x 8 facets; 77 reconstructed P2
  artifacts, 12 perturbation + 6 advisory cases. Eleven instrument fixes, incl. a certain crash (%b is the commit
  body, so the TSV parse died on item 1) and a human_authored heuristic that was False for EVERY artifact because
  every commit here is co-authored by Claude. Shortfalls reported, never back-filled: bug 2/8, initiative 3/4,
  spike 0/3; P2 perturbation 12/20, advisory 6/10; recent_deploy underivable at this pin.
P1 arm L (F36, \$21.37): 200 + 200 dispatches, 0 void. Modal R(stage,ctx) identical across 10 draws on only
  5/20, 3/20 (v1) and 2/20, 2/20 (v2) items against an 80% line — the kill fires 4 times out of 4. Section 1.4's
  prescribed anchor rewrite did NOT repair it: blast_radius 0.488 -> 0.473 (so human-supplied), external_dep
  collapsed to constant (200/200 false). And three axes with byte-identical anchors drifted up to -0.168, which
  is larger than the effect the alpha<0.6 threshold is meant to detect — that threshold is not decidable here.
P7 (F37, \$131.62): 61 graph runs, 0 void, both arms at 10 items x 3 repeats. The adversary block costs 2.64x and
  changes the root cause on 1 item in 10; its policy check returned FALSE on 33 of 33 runs where an adversary ran
  (33/33 dispatches produced a blocking-class finding), so the verdict channel is a constant function of the arm.
  Deviation channel offered 71 times, used 0. Section 4.4 criterion 1 fires, but at n=3 every Wilson interval
  overlaps by construction — the run establishes the criterion's POWER, not its verdict.
P8 (F38, \$3.01): 3 resume cycles. Lossless 3/3 across a real taskkill /T /F of a process holding state in
  memory; resume costs 0.45-0.78x F28's stage-gate price, so neither of section 5.4's first two criteria fires.
  The third is NOT answerable: the gate never fired on its own in any run. Section 5.2's fixture design does not
  work — three redactions and two selection rules, and the investigator reproduced from the symptom every time.
Catalog finding (F37 section 6): at rigor=mandatory with has_repro=true the engine selects \`bisect\`, which has
  NO adversary and NO policy step — escalating rigor DROPS the quality block. Candidate defect in F34 amendment
  26's ceiling design; nothing was edited.
Decisions: spikes/DECISIONS-2026-09-09.md, 38 entries with grounding, consequence and reversibility.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_012ZWjdcpCcGe2GkGdxDAYdT"
