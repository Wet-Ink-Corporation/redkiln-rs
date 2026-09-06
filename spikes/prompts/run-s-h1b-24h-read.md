Take the S-H1b `h1b-24h` read (METHOD-S §4a) and close out F30. This is the last
outstanding dispatch of the S-H1b/S-WT set; everything else is done and committed.

**Do not run this before 2026-09-07T18:55:48Z.** The mark was laid at
2026-09-06T18:55:48Z and is durable — it is a line in the results file, not a
live process, so nothing was lost by the session ending.

**Work in the existing worktree, not the main checkout.** S-VAR was running in
`D:\repos\redkiln-rs` when this set was produced; do not touch
`results/s-var.jsonl` or any `e3-*` fixture.

Read `spikes/METHOD-S.md` §4a and §7 (all of §7 — the last two amendments are
this set's, dated 2026-09-06), and `spikes/FINDINGS-DRAFT-F30.md`, before
dispatching.

## 0. STATE — nothing here needs rebuilding

- Worktree `D:\repos\redkiln-rs.swt`, branch `spike/s-h1b-s-wt`, 6 commits,
  tree clean apart from the untracked `spikes/FINDINGS-DRAFT-F30.md` (a
  deliberate draft — do NOT commit it).
- `spikes/runner/target/debug/redkiln-spike-runner.exe` is built. Do not rebuild
  unless you changed the crate; the main checkout's binary may be in use.
- Fixtures are in place and gitignored: `fixtures/h1-{a,b,c}` (payload sha
  `cff4af90…`, 37,169 bytes), `fixtures/s-wt/w1..w10`, and the prompt texts
  `fixtures/static-system-prompt.txt` (S-H1b) and
  `static-system-prompt-s-wt.txt` (S-WT).
- Done: S-WT 10/10 (steady state 1.000), `h1b-1m` (1.48 min, creation 0, share
  1.000), `h1b-65m` (66.08 min, creation 29,619, share 0.000).
- `results/s-h1b-24h.jsonl` holds exactly one `warm` line and no read.

## 1. THE ONE THING TO CHECK BEFORE DISPATCHING

The read is only clean if **no other haiku `--system-prompt` dispatch has run on
this machine in the hour before it**. The mark's prefix decomposes as ~15.1k of
tool definitions (shared across *different* system-prompt texts — see §7's
2026-09-06 correction) plus ~14.5k of system prompt and payload, and any haiku
dispatch carrying `--system-prompt` refreshes the first block.

So: if other haiku spike work has run recently, wait an hour for it to age out
before taking the read. S-VAR is Opus and cannot touch this prefix; ignore it.

If the read comes back with **share > 0**, that is evidence of refresh by other
traffic, not of survival — the 65-minute read in the same set went fully cold.
Say so in the report rather than treating a partial share as a result.

## 2. THE DISPATCH

One command, from `D:\repos\redkiln-rs.swt\spikes\runner` (every path in the
runner is CWD-relative; run it from anywhere else and `--pin-file` and `--out`
resolve elsewhere). Read the system prompt from the file rather than retyping it
— a changed byte is a new prefix and a cold write.

```powershell
Set-Location D:\repos\redkiln-rs.swt\spikes\runner
$sp = [IO.File]::ReadAllText('D:\repos\redkiln-rs.swt\spikes\fixtures\static-system-prompt.txt')
.\target\debug\redkiln-spike-runner.exe ttl-read `
  --experiment S --arm h1b-24h `
  --cwd ../fixtures/h1-c --out ../results/s-h1b-24h.jsonl `
  --model haiku --prompt-file ../fixtures/h1-c/payload.txt `
  --system-prompt $sp
```

`$sp` must be 63 characters: `You are a careful software reviewer. Answer only what is asked.`

## 3. VALIDATE THE PAIR — a mismatch is VOID, not a result

From `results/s-h1b-24h.jsonl`, confirm on the `warm` and `read` step records:

- identical last element of `flags` (the `--system-prompt` string),
- identical `effective.cwd` (`…\fixtures\h1-c`),
- `prompt_sha256` starting `cff4af90`, `prompt_bytes` 37,169, both ends,
- `outcome: "ok"`, `isolation_ok: true`, `effective.mcp_servers` empty,
- exactly **one** `"label":"warm"` line in the file.

That last one matters: `journal::last_warm_mark` (`runner/src/journal.rs:151`)
pairs a read with the globally latest `warm` line in the `--out` file with no
filter on arm or cwd, which is why each arm has its own file. Do not merge the
files before taking the read.

`cache_read` is **not** on the `ttl_pair` line — read it off the `label:"read"`
step record as `usage.cache_read_input_tokens`.

## 4. CLOSE OUT

1. Concatenate, in arm order, into the deliverable — keep the per-arm files as
   provenance: `s-h1b-1m.jsonl` + `s-h1b-65m.jsonl` + `s-h1b-24h.jsonl` →
   `results/s-h1b.jsonl`.
2. Fill the `h1b-24h` row in `spikes/FINDINGS-DRAFT-F30.md` (gap, cache_read,
   cache_creation, share) against H1's 49.7 h comparator of 18,603 / 0.484, and
   update Limit 4, which currently says the arm is unread. State which §4a.3
   branch it lands in. The 65-minute read landed in **none of the three as
   written** — 29,619, above the ≈ 18.6k branch — so if the 24 h read is also
   ~29,619 the honest statement is that the cost is flat under this flag too,
   just flat at the whole prefix instead of at 18.6k.
3. Do not interpret beyond what §4a.3 licenses. Leave the "Recorded, and NOT
   licensed as a conclusion" section as a spike specification, not a finding.
4. Commit `results/s-h1b-24h.jsonl` and `results/s-h1b.jsonl` on
   `spike/s-h1b-s-wt`. **Leave `FINDINGS-DRAFT-F30.md` uncommitted** — it is a
   draft for Ryan's review.
5. Report the branch. Merging into `main` and folding F30 into `FINDINGS.md` are
   Ryan's calls, not the session's. The worktree can be removed once he has
   merged; the fixtures under it are gitignored and disposable.

## 5. CONTEXT THE REPORT MUST NOT GET WRONG

S-H1b is **replication, not discovery**. F28's own 2026-09-05 "Correction and
completion" already recorded a `--system-prompt` read at 66.8 min with share
0.000 and creation 29,680, and already concluded that the flag *forfeits* the
shared-prefix subsidy rather than blunting time decay. What S-H1b adds is the
per-arm directory isolation §4a.2 requires, the 1-minute control that makes the
decay attributable to time, and this 24-hour arm — which has never been measured
under `--system-prompt`.
