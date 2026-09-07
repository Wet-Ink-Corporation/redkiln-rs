Run spike S-H1c (METHOD-S §4c) in D:\repos\redkiln-rs. Haiku, ~free, 26 dispatches. It measures the --append-system-prompt route as an alternative to S-H1b's --system-prompt route.

Read spikes/METHOD-S.md §0, §4a, §4c, §6 and the 2026-09-06 notes in §7 (especially the one about the shared ~15.1k tool block: ANY haiku dispatch on this account refreshes it). Do not edit METHOD-S.md except to append a dated note under §7, and never after a record the change would affect.

0. PRECONDITIONS
- METHOD-S.md with §4c committed before any dispatch. If dirty, stop and tell me.
- cd spikes/runner && cargo build.
- The static card text is the SAME text S-H1b used (spikes/fixtures/static-system-prompt.txt), passed as the value of --append-system-prompt, byte-identical on every dispatch.
- Do NOT run this while an S-H1b long mark (h1b-24h) is still awaiting its read: these dispatches refresh the shared tool block and would contaminate that read. Wait until the h1b-24h read is recorded.

1. TIME ARMS (§4c.2 part 1) — three fresh directory copies of spikes/fixtures/h1: h1-d, h1-e, h1-f. One arm per directory; no other dispatch in a directory between its mark and its read.

  ttl-mark --experiment S --arm h1c-1m  --cwd ../fixtures/h1-d --out ../results/s-h1c.jsonl --model haiku --prompt-file ../fixtures/h1-d/payload.txt --flag --exclude-dynamic-system-prompt-sections --flag --append-system-prompt --flag "<static text>"
  (wait ~1 minute)
  ttl-read (same flags, same arm, same cwd)

  ttl-mark --arm h1c-65m --cwd ../fixtures/h1-e ...   (wait 65 minutes — durable timestamp, no harness background sleep, F22)
  ttl-read --arm h1c-65m --cwd ../fixtures/h1-e ...

  ttl-mark --arm h1c-24h --cwd ../fixtures/h1-f ...   (read any time after 24 h)
  ttl-read --arm h1c-24h --cwd ../fixtures/h1-f ...

(ttl-mark/ttl-read accept the same --flag passthrough as probe since the 2026-09-06 runner change; confirm with `ttl-mark --help`.)

Check each read's ttl_pair line: gap_minutes, cache_read, cache_creation, share. Verify from the record's `flags` array that both flags and the exact text were passed, and from system/init that isolation_ok is true.

2. WORKTREE AND COMMIT ARMS (§4c.2 part 2) — run these BEFORE laying the 65m and 24h marks above (they refresh the shared block), or after those reads are recorded; never in between.

  For N in 1..10:
    probe --experiment S --arm wt-append-wN --repeats 1 --cwd ../fixtures/<worktree dir>/wN --out ../results/s-h1c.jsonl --model haiku --flag --exclude-dynamic-system-prompt-sections --flag --append-system-prompt --flag "<static text>"
  (reuse the ten worktrees S-WT used, or create ten fresh ones from spikes/fixtures/e1)

  probe --experiment S --arm commit-append --repeats 10 --commit-between --cwd ../fixtures/e1-b --out ../results/s-h1c.jsonl --model haiku --flag --exclude-dynamic-system-prompt-sections --flag --append-system-prompt --flag "<static text>"

3. REPORT
Per §4c.3: the 65-min read's cache_creation against the three branches (≤ ~5k / ≈ 18.6k / ≈ 29.6k) and the registered prediction (4k–8k); steady-state cache-read share for wt-append-w2..w10 (predicted ≥ 0.95) and commit-append draws 2–10 (predicted 0.85–0.95), against S-WT's 1.000 and F16 arm c's 0.846. Then a DRAFT FINDINGS.md entry ("## F3x — S-H1c RESULT: …") for my review — do not commit. State plainly which of the two card-delivery routes (replace vs append) the numbers favour and on what trade-off (cost per resume vs cost per dispatch).
