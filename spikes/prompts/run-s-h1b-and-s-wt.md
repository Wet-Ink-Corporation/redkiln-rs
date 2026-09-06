Run spikes S-H1b (METHOD-S §4a) and S-WT (METHOD-S §4b) in D:\repos\redkiln-rs. Both are haiku and cost effectively no plan headroom (F14); they are designed to run WHILE S-VAR is running. Do not touch results/s-var.jsonl or any e3-* fixture.

Read spikes/METHOD-S.md §0, §4a, §4b, §6, and spikes/HANDOFF.md ("How to run dispatches", "Traps") first. Do not edit METHOD-S.md except to append a dated note under §7, and never after a record that the change would affect.

0. PRECONDITIONS
- METHOD-S.md must be committed with §4a/§4b present before any dispatch. If dirty, stop and tell me.
- cd spikes/runner && cargo build (cargo test does not rebuild the bin).
- The static system prompt used for BOTH spikes is exactly this text, saved to spikes/fixtures/static-system-prompt.txt and passed by content (not path) on every dispatch:
  "You are a careful software reviewer. Answer only what is asked."
  It must be byte-identical on every dispatch; a changed byte is a new prefix and a new cold write.

1. S-H1b — static system prompt across TIME (§4a)
Fixture: spikes/fixtures/h1 (already contains payload.txt, the 37 KB review-scale prefix). Use the ttl-mark / ttl-read pair; a mark is durable, so nothing waits in a process.

Three arms, each with its own mark, all with the same --system-prompt:

  ttl-mark --experiment S --arm h1b-1m  --cwd ../fixtures/h1 --out ../results/s-h1b.jsonl --model haiku --prompt-file ../fixtures/h1/payload.txt --system-prompt "<static text>"
  (wait ~1 minute)
  ttl-read --experiment S --arm h1b-1m  --cwd ../fixtures/h1 --out ../results/s-h1b.jsonl --model haiku --prompt-file ../fixtures/h1/payload.txt --system-prompt "<static text>"

  ttl-mark --experiment S --arm h1b-65m ... (same flags)
  (wait 65 minutes — use a durable timestamp and come back; do NOT sleep in a harness background job, F22)
  ttl-read --experiment S --arm h1b-65m ...

  ttl-mark --experiment S --arm h1b-24h ... (same flags)
  (read any time after 24 h — the mark stays valid)
  ttl-read --experiment S --arm h1b-24h ...

Check each read's record: the "ttl_pair" line reports gap_minutes, cache_read, cache_creation and share. Confirm the mark and read used the same system prompt (the resume key and arm must match).

Report per gap: gap_minutes, cache_creation, share, next to H1's values (1 min: 0 / 1.000; 65 min: 18,603 / 0.484; 49.7 h: 18,603 / 0.484). State which §4a.3 branch the 65-min read lands in, and compare to the registered prediction (4k–10k).

2. S-WT — static system prompt across WORKTREES (§4b)
Reproduce E1 arm (d)'s topology: ten git worktrees of the same repo, one dispatch each, in sequence. If spikes/fixtures/e1-d-wt/w1..w10 still exist, reuse them; otherwise create ten fresh worktrees from spikes/fixtures/e1 under spikes/fixtures/s-wt/w1..w10 with `git worktree add`.

For N in 1..10:

  probe --experiment S --arm wt-sysprompt-wN --repeats 1 --cwd ../fixtures/<dir>/wN --out ../results/s-wt.jsonl --model haiku --flag --system-prompt --flag "<static text>"

(`--flag` appends one token to the child's argv each time, so the flag name and its value are two separate --flag arguments.)

Confirm from each record's system/init that isolation_ok is true and the child reported the reduced tool set. Report cache_read, cache_creation and share per worktree; steady state is w2..w10 (w1 pays the one-time cold write for the new system prompt, as in F16 arm e). Compare the steady-state mean to E1 arm (d) 0.685 and arm (e) 1.000, state which §4b.3 branch it lands in, and compare to the registered prediction (≥ 0.95).

3. REPORT
For both spikes, draft a FINDINGS.md entry in the house style ("## F30 — S-H1b/S-WT RESULT: ...", "Established by:", tables, "What this settles", "Limits") as a DRAFT for my review — do not commit it. Do not interpret beyond what §4a.3 / §4b.3 license. If a read was taken with a different system prompt than its mark, the pair is void, not a result.
