Run spike S-TYPED (METHOD-S §2) in D:\repos\redkiln-rs. Start ONLY after S-VAR's 60 records are complete — both draw on the same Opus five-hour window.

Read spikes/METHOD-S.md §0, §2, §6 and the 2026-09-06 S-TYPED amendment in §7, and spikes/HANDOFF.md ("How to run dispatches", "Traps") first. Do not edit METHOD-S.md except to append a dated note under §7, and never after a record the change would affect.

1. PRECONDITIONS
- METHOD-S.md, spikes/results/audit-s.py, spikes/runner/src/e3.rs and spikes/runner/src/main.rs committed. If dirty, stop and tell me. Record HEAD.
- cd spikes/runner && cargo build; cargo test (118 pass); doctor clean, claude >= 2.1.259.
- The typed schema now requires `rationale` per finding and a structured `repro` object {command, observed, expected_when_met}; confirm the built binary carries it (grep the schema in a dry record or run the unit test `the_typed_schema_has_no_verdict_field_and_a_closed_evidence_enum`).

2. VALIDATE ONE TYPED DRAW BEFORE THE SET — this is the F20 check for the new schema.
Run one cheap S-SILENT item with the typed schema:

  review-probe --condition s-silent --arm b --schema typed --only issue-62-new-stale-pinned-template --out ../results/s-typed.jsonl --timeout-secs 3600

Inspect the last record: outcome "ok", card_adopted true, fix_unreachable true, schema "typed", arm "b-typed-s-silent", scored present, a "policy" object with "strict" and "lenient" each carrying blocks/blocking_files/counts, and every finding in step.digest.findings carrying a non-empty `rationale` and a `repro` object with `command`, `observed`, `expected_when_met`. If the record is void with error_kind "schema", STOP: the closed enum did not conform — that is an instrument finding (METHOD-S §2.6 last bullet) and I need to see the raw digest text before anything else is spent.

3. RUN THE SET — 51 typed dispatches (23 S-SILENT + 23 CLEAN + 5 S-SUBTLE), prose comparators are NOT re-run.

  review-probe --condition s-silent --arm b --schema typed --out ../results/s-typed.jsonl --timeout-secs 3600
  review-probe --condition clean    --arm b --schema typed --out ../results/s-typed.jsonl --timeout-secs 3600
  review-probe --condition s-subtle --arm b --schema typed --out ../results/s-typed.jsonl --timeout-secs 3600 --seeds ../results/e3h-seeds.jsonl

Each command walks the manifest (s-subtle runs only the five seeded items) and skips recorded draws; a killed run costs one dispatch. Launch DETACHED (Start-Process, tee to spikes/s-typed.log), poll from the foreground with a waiter that matches "wrote|HALT|error|panicked". Run S-SILENT first (~$0.5 each, ~130 s), then S-SUBTLE, then CLEAN (~$1–5 each, up to ~2400 s — the 3600 s ceiling is deliberate, see the S-VAR amendment).

4. THROTTLE AND VOIDS
The runner halts at the 0.90 five_hour margin with "HALT before <slug>" — a pause, not a failure; report utilisation and resetsAt, wait, re-run the same command. Do not lower --max-five-hour.
Platform-error voids (529/500): re-run the same command. Schema voids: do NOT retry blindly — count them (they are the conformance metric, §2.5), then re-run once so the item also has a scored record if the model conforms on a second draw; both records stay in the file.
Any other void: stop and report.

5. AUDIT AND REPORT

  python results/audit-s.py typed results/s-typed.jsonl results/e3.jsonl results/e3h.jsonl results/s-var-citation-targets.json

Give me: schema conformance rate; S-SILENT strong detection typed/strict vs prose (with any discordant items named); S-SUBTLE strong detection vs E3-H arm b (5/5); CLEAN false positives under strict and lenient vs prose; the F24 re-classification table; the CONTRADICTION SECONDARY (F24 target cited with blocking-grade evidence, predicted >= 4/5) next to the prose arm's verdict for the same item; the evidence-class distribution per condition; the `repro.command` fill rate (how many blocking-grade findings carry a non-empty command — the flip criterion needs one); cost and wall-clock delta vs the prose arm; the repeated-items table if any item was drawn twice; and which §2.6 decision-rule branch applies.

Then draft a FINDINGS.md entry in the house style ("## F31 — S-TYPED RESULT: ...", "Established by:", tables, "What this settles", "Limits") comparing against the §2.7 predictions line by line — a DRAFT for my review, do not commit. Do not interpret beyond what §2.6 licenses; the prose CLEAN comparator is a single draw of a process F27 showed to be flaky, and the write-up must say so wherever a one- or two-item difference appears.
