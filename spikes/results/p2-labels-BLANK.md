# P2 — human "should re-run" labels (BLANK template). METHOD-P §0 rule 1 / §2.2: for every downstream artifact in every
# perturbation case, label from the DIFF ALONE (git diff <prev_rev> <sha> -- .bklg/<item>/<upstream>) before the engine runs.
# Generate rows from spikes/results/p2-cases.jsonl:
#   python3 -c "import json;[print(f'| {c[\"case\"]} | {c[\"item\"]} | {c[\"upstream\"]} | {d} |  |  |') for c in map(json.loads,open('spikes/results/p2-cases.jsonl')) if c['kind']=='perturb' for d in c['downstream']]"
labelled_at: FILL-ME-ISO8601Z
labeller: FILL-ME
corpus_sha: 47faec9f46b8d983c2c75f0794164e68415ae706
cases_file: spikes/results/p2-cases.jsonl

| case | item | upstream replaced | downstream artifact | should re-run (yes/no) | why (one line) |
| --- | --- | --- | --- | --- | --- |
| P01 | <slug> | spec.md | implementation-report.md |  |  |

# Advisory cases (A01–A10) are labelled AFTER the engine, per METHOD-P §2.3 ("useful / not useful"), in a separate file
# p2-advisory-labels-<who>.md — do not put them here.
