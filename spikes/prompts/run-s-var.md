Run spike S-VAR (METHOD-S §1) in D:\repos\redkiln-rs.

Read spikes/METHOD-S.md §0, §1 and §6, and spikes/HANDOFF.md ("How to run dispatches" and "Traps") before doing anything. Do not modify METHOD-S.md except to append a dated note under §7 if something forces a change, and never after a record exists that the change would affect.

1. PRE-REGISTRATION MUST PRECEDE DATA.
Confirm spikes/METHOD-S.md, spikes/results/audit-s.py, spikes/runner/src/e3.rs and spikes/runner/src/main.rs are committed. If the working tree is dirty for any of those files, stop and tell me — I will commit. Record the HEAD SHA; it goes in the findings entry.

2. BUILD THE BINARY.
cd spikes/runner && cargo build
(cargo test and cargo clippy do NOT rebuild the bin.) Then run cargo test and confirm 118 pass. Run the runner's `doctor` subcommand and confirm claude >= 2.1.259 and a clean environment.

3. VALIDATE ONE DRAW BEFORE THE SET.
Run a single dispatch and inspect the record before spending anything else:

  review-probe --condition clean --arm b --only issue-62-new-stale-pinned-template --repeat 1 --out ../results/s-var.jsonl --timeout-secs 2400

Open the last line of results/s-var.jsonl and confirm: outcome "ok", card_adopted true, node_modules_linked true, schema "prose", arm "b-clean", and a non-null scored object. If it is void, read void_reason and fix the instrument, not the data.

4. RUN THE SET: 6 items x repeats 1..10 = 60 CLEAN dispatches.
Items, in this order:

  fix-advance-write-lock-gate-scope
  issue-146-147-boundary-window
  issues-135-129-declared-boundary-parse
  issue-74-plan-briefs-dead-plan-anchor
  issue-63-advance-gate-diagnostic-truncated
  issue-62-new-stale-pinned-template

For each item, for r in 1..10:

  review-probe --condition clean --arm b --only <slug> --repeat <r> --out ../results/s-var.jsonl --timeout-secs 2400

Use the same --out file for every draw. Resume is keyed per (arm, slug, repeat): re-running the same command skips recorded draws, and a killed run costs one dispatch.

Launch the loop DETACHED (Start-Process a PowerShell script that runs the loop and tees to spikes/s-var.log), not as a harness background job — background jobs are killed unpredictably (F22). Poll the log from the foreground with a waiter that also matches a failure marker (e.g. Select-String -Pattern "wrote|HALT|error|panicked"), never an unbounded until-grep loop.

5. RESPECT THE THROTTLE.
Each CLEAN dispatch draws about +0.077 of five_hour (F21); 60 draws span several windows. The runner halts at the 0.90 margin with "HALT before <slug>" — that is a pause, not a failure. When it halts, report utilisation and resetsAt from the last record's headroom, and wait for the window to roll before re-launching the same loop. Do not lower --max-five-hour. The assistant session shares the account, so headroom readings are upper bounds.

6. VOIDS.
After each item's ten draws, run:

  python results/audit-s.py var results/s-var.jsonl

A void whose reason is a platform error (529/500) is retried by re-running the same command; the resume key is unchanged until a valid record exists. Any other void reason: stop and report it to me before continuing. Never delete or edit records.

7. REPORT.
When all 60 valid records exist, run the audit and give me:
  - the per-item table (n, reject count, rate, Wilson interval, band, on-target citations)
  - the class-vs-stability ordering
  - total cost and wall clock
  - which of the §1.5 decision-rule branches the bands land in

Then draft a FINDINGS.md entry in the house style ("## F29 — S-VAR RESULT: ...", "Established by:", tables, "What this settles", "Limits") comparing the result against the §1.6 predictions line by line. This is a DRAFT for my review — do not commit it. Do not interpret beyond what §1.5 licenses; if a band is ambiguous, say so.

NOTES
- issue-146-147 CLEAN dispatches ran up to 2055 s in E3; the 2400 s ceiling is deliberate.
- The F27 draws in results/e0-stability.jsonl are NOT to be merged into s-var.jsonl (METHOD-S §1.3).
