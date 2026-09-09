# P2 — human "should re-run" labels (BLANK template). METHOD-P §0 rule 1 / §2.2: for every downstream artifact in every
# perturbation case, label from the DIFF ALONE (git diff <prev_rev> <sha> -- .bklg/<item>/<upstream>) before the engine runs.
# Generate rows from spikes/results/p2-cases.jsonl:
#   python3 -c "import json;[print(f'| {c[\"case\"]} | {c[\"item\"]} | {c[\"upstream\"]} | {d} |  |  |') for c in map(json.loads,open('spikes/results/p2-cases.jsonl')) if c['kind']=='perturb' for d in c['downstream']]"
labelled_at: FILL-ME-ISO8601Z
labeller: FILL-ME
corpus_sha: 47faec9f46b8d983c2c75f0794164e68415ae706
cases_file: spikes/results/p2-cases.jsonl

# GENERATED 2026-09-09 from p2-cases.jsonl: 12 perturbation cases,
# 41 (case, downstream) rows. Label from the DIFF ALONE:
#   git -C D:/repos/redkiln diff <replaced_with_rev> 47faec9f -- .bklg/<item>/<upstream replaced>
# Advisory cases A01-A06 are labelled AFTER the engine, in a separate file (METHOD-P 2.3).
| case | item | upstream replaced | downstream artifact | should re-run (yes/no) | why (one line) |
| --- | --- | --- | --- | --- | --- |
| P01 | support/advance-commit-ignored-path | discover.md | implementation-report.md |  |  |
| P01 | support/advance-commit-ignored-path | discover.md | report.md |  |  |
| P02 | support/fix-lane-plumbing/issue-50-fix-story-before-worktree | story.md | spec.md |  |  |
| P02 | support/fix-lane-plumbing/issue-50-fix-story-before-worktree | story.md | implementation-report.md |  |  |
| P02 | support/fix-lane-plumbing/issue-50-fix-story-before-worktree | story.md | report.md |  |  |
| P03 | support/fix-planning-workflow-reliability/issue-81-plan-discover-spec-gate-mismatch | story.md | spec.md |  |  |
| P03 | support/fix-planning-workflow-reliability/issue-81-plan-discover-spec-gate-mismatch | story.md | implementation-report.md |  |  |
| P03 | support/fix-planning-workflow-reliability/issue-81-plan-discover-spec-gate-mismatch | story.md | report.md |  |  |
| P03 | support/fix-planning-workflow-reliability/issue-81-plan-discover-spec-gate-mismatch | story.md | _review.md |  |  |
| P04 | support/fix-planning-workflow-reliability/issue-72-plan-briefs-readonly-grounding | story.md | spec.md |  |  |
| P04 | support/fix-planning-workflow-reliability/issue-72-plan-briefs-readonly-grounding | story.md | implementation-report.md |  |  |
| P04 | support/fix-planning-workflow-reliability/issue-72-plan-briefs-readonly-grounding | story.md | report.md |  |  |
| P04 | support/fix-planning-workflow-reliability/issue-72-plan-briefs-readonly-grounding | story.md | _review.md |  |  |
| P05 | support/fix-planning-workflow-reliability/issue-79-plan-prds-verifier-false-pass | story.md | spec.md |  |  |
| P05 | support/fix-planning-workflow-reliability/issue-79-plan-prds-verifier-false-pass | story.md | implementation-report.md |  |  |
| P05 | support/fix-planning-workflow-reliability/issue-79-plan-prds-verifier-false-pass | story.md | report.md |  |  |
| P05 | support/fix-planning-workflow-reliability/issue-79-plan-prds-verifier-false-pass | story.md | _review.md |  |  |
| P06 | support/fix-lane-plumbing/issue-47-fix-report-md-unauthored | story.md | spec.md |  |  |
| P06 | support/fix-lane-plumbing/issue-47-fix-report-md-unauthored | story.md | implementation-report.md |  |  |
| P06 | support/fix-lane-plumbing/issue-47-fix-report-md-unauthored | story.md | report.md |  |  |
| P07 | support/fix-initiative-forging-flow/issue-78-initiative-adopt-templates-contamination | story.md | spec.md |  |  |
| P07 | support/fix-initiative-forging-flow/issue-78-initiative-adopt-templates-contamination | story.md | implementation-report.md |  |  |
| P07 | support/fix-initiative-forging-flow/issue-78-initiative-adopt-templates-contamination | story.md | report.md |  |  |
| P08 | support/fix-gate-honesty-sweep/issues-135-129-declared-boundary-parse | story.md | spec.md |  |  |
| P08 | support/fix-gate-honesty-sweep/issues-135-129-declared-boundary-parse | story.md | implementation-report.md |  |  |
| P08 | support/fix-gate-honesty-sweep/issues-135-129-declared-boundary-parse | story.md | report.md |  |  |
| P08 | support/fix-gate-honesty-sweep/issues-135-129-declared-boundary-parse | story.md | _review.md |  |  |
| P09 | support/fix-ci-pinned-pack-drift-guard/issue-80-ci-pinned-pack-drift-guard | story.md | spec.md |  |  |
| P09 | support/fix-ci-pinned-pack-drift-guard/issue-80-ci-pinned-pack-drift-guard | story.md | implementation-report.md |  |  |
| P09 | support/fix-ci-pinned-pack-drift-guard/issue-80-ci-pinned-pack-drift-guard | story.md | report.md |  |  |
| P10 | support/fix-serve-child-leak/reap-leaked-serve-children | story.md | spec.md |  |  |
| P10 | support/fix-serve-child-leak/reap-leaked-serve-children | story.md | implementation-report.md |  |  |
| P10 | support/fix-serve-child-leak/reap-leaked-serve-children | story.md | report.md |  |  |
| P10 | support/fix-serve-child-leak/reap-leaked-serve-children | story.md | _review.md |  |  |
| P11 | support/fix-initiative-forging-flow/issue-68-forge-initiative-readonly-write | story.md | spec.md |  |  |
| P11 | support/fix-initiative-forging-flow/issue-68-forge-initiative-readonly-write | story.md | implementation-report.md |  |  |
| P11 | support/fix-initiative-forging-flow/issue-68-forge-initiative-readonly-write | story.md | report.md |  |  |
| P12 | support/fix-planning-workflow-reliability/issue-73-plan-briefs-missing-artifacts-discarded | story.md | spec.md |  |  |
| P12 | support/fix-planning-workflow-reliability/issue-73-plan-briefs-missing-artifacts-discarded | story.md | implementation-report.md |  |  |
| P12 | support/fix-planning-workflow-reliability/issue-73-plan-briefs-missing-artifacts-discarded | story.md | report.md |  |  |
| P12 | support/fix-planning-workflow-reliability/issue-73-plan-briefs-missing-artifacts-discarded | story.md | _review.md |  |  |
