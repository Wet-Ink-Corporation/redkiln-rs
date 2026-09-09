# P8 — the 24 h arm: exact resume commands and the earliest time each may be resumed

METHOD-P §5.2 anchors the delay arms at **0 / 65 min / 24 h** (F18/F28). §5.5 names the constraint this file
exists for: *"the 24 h delay gives one draw per item per repeat and cannot be hurried."*

The AFK brief's lane E3 says: **drive to the gate mark and stop. Do not resume.** That is what was done — this
file is the handover.

---

## Status at handover

**The 24 h arm was NOT started.** This is a departure from lane E3 and is disclosed here rather than papered over.

The reason is a measured one, not a scheduling one. The P8 control run (`spikes/results/p8.jsonl`, arm `control`)
showed that the fixture as first built **does not reach the gate at all**: with only the "Steps to reproduce"
section withheld, haiku reproduced `fix-advance-write-lock-gate-scope` on its first attempt (754 s) and the graph
went straight past `triage-gate`. A 24 h arm started on that fixture would have spent a day to measure nothing.

The redaction was rebuilt as a keep-list (DECISIONS-2026-09-09 #28) — the title and the narrative problem
statement survive, the acceptance criteria and behaviour specification are withheld — and the **0-delay** arm was
started on the corrected fixture to prove the gate fires and that resume is lossless before a 24 h draw is
committed to. Starting the long arm on an unvalidated fixture is the mistake `HANDOFF.md`'s own trap list already
names: *"A rig that compiles and passes its unit tests has not been run."*

**So the 24 h arm's precondition is: the 0-delay arm produced at least one `triage-gate` record.** Check that
first — one line:

```bash
python -c "import json,io;print(sum(1 for l in io.open('spikes/results/p8.jsonl',encoding='utf-8') if json.loads(l).get('phase')=='triage-gate'))"
```

If that prints `0`, the fixture still does not fire the gate and the 24 h arm must not be started; strengthen the
keep-list in `spikes/runner/src/p8.rs` (`strip_repro`) and re-check with `--phase control` first.

---

## Items

From `spikes/plans/p8-items.txt`, selected by `sha256(slug)` order from the 12 E3 items whose `spec.md` carries a
withholdable section:

| slug | item |
| --- | --- |
| `fix-advance-write-lock-gate-scope` | RK-S0263 |
| `issue-79-plan-prds-verifier-false-pass` | RK-S0226 |
| `issue-146-147-boundary-window` | RK-S0273 |

3 items × 3 repeats = **9 runs** in the 24 h arm.

---

## Step 1 — drive to the gate mark (do this when you start the arm)

Each invocation drives one item to its `triage-gate` mark, writes its state to
`spikes/results/p8-state/`, prints `GATE-MARK <slug> 24-h r<n>`, and then **blocks** holding that state in memory
so the kill is a real kill of a live process rather than a graceful exit.

Run each in its own detached process (HANDOFF F22 — a harness background job is killed at any age):

```powershell
cd D:\repos\redkiln-rs\spikes\runner
foreach ($slug in 'fix-advance-write-lock-gate-scope','issue-79-plan-prds-verifier-false-pass','issue-146-147-boundary-window') {
  foreach ($r in 1,2,3) {
    Start-Process pwsh -ArgumentList '-NoProfile','-Command', `
      "Set-Location 'D:\repos\redkiln-rs\spikes\runner'; .\target-p8\debug\redkiln-spike-runner.exe repro-probe --phase run --arm 24-h --repeat $r --only $slug --worktree ..\fixtures\p8\wt --out ..\results\p8.jsonl --block-secs 3600 2>&1 | Tee-Object -FilePath 'D:\repos\redkiln-rs\spikes\p8-24h.log' -Append" `
      -WindowStyle Hidden
    Start-Sleep -Seconds 10
  }
}
```

Wait for nine `GATE-MARK` lines, then **record the wall-clock time** — that is when the 24 h clock starts:

```bash
grep -c "GATE-MARK" spikes/p8-24h.log     # expect 9
date -u +%Y-%m-%dT%H:%M:%SZ                # write this down; T0
```

## Step 2 — kill the processes (this is the measurement)

Windows has no SIGTERM. Both forms are legitimate and they are different experiments, so pick one and record
which: `taskkill /PID <id>` posts WM_CLOSE (the polite form); `taskkill /PID <id> /F` terminates. The runs so far
used `/F`.

```powershell
Get-Process redkiln-spike-runner | Where-Object { $_.StartTime -gt (Get-Date).AddHours(-2) } |
  ForEach-Object { taskkill /PID $_.Id /T /F }
```

Confirm the state files survived the kill — they are the whole point:

```bash
ls spikes/results/p8-state/*24-h*.json | wc -l    # expect 9
```

## Step 3 — wait

**Earliest resume time: T0 + 24 h.** Not "about a day": F18's anchor is 24 h and F10 put the prompt-cache TTL at
~1 h, so the arm's whole purpose is to resume far outside the cache. Resuming early collapses it into the 65-min
arm and the draw is wasted.

## Step 4 — answer, then resume

`--phase answer` touches no model and can be run at any point after the kill; §9b.15 makes `answer` the gate's
only output, so it is a closed option id and never free text.

```powershell
cd D:\repos\redkiln-rs\spikes\runner
foreach ($r in 1,2,3) {
  .\target-p8\debug\redkiln-spike-runner.exe repro-probe --phase answer --arm 24-h --repeat $r --answer provide-repro-steps --out ..\results\p8.jsonl
}
```

Then resume — a **fresh process**, which is the point: it must rebuild every step output from disk.

```powershell
foreach ($slug in 'fix-advance-write-lock-gate-scope','issue-79-plan-prds-verifier-false-pass','issue-146-147-boundary-window') {
  foreach ($r in 1,2,3) {
    .\target-p8\debug\redkiln-spike-runner.exe repro-probe --phase resume --arm 24-h --repeat $r --only $slug --worktree ..\fixtures\p8\wt --out ..\results\p8.jsonl 2>&1 |
      Tee-Object -FilePath 'D:\repos\redkiln-rs\spikes\p8-24h.log' -Append
  }
}
```

## Step 5 — read the result

Every resume writes an `emit` record carrying `resume_check`. The three things §5.3 asks for:

```bash
python - <<'EOF'
import json, io, collections
rows = [json.loads(l) for l in io.open('spikes/results/p8.jsonl', encoding='utf-8')]
em = [r for r in rows if r.get('phase') == 'emit' and r.get('arm') == '24-h']
for r in em:
    c = r['resume_check']
    print(f"{r['item'][:40]:42} r{r['repeat']} lossless={c['lossless']} "
          f"changed={c['changed']} lost={c['lost']} "
          f"gap={r['gap_minutes']:.0f}min second_mark={r['second_arrival_minted_a_new_mark']} "
          f"resume_tok={r['resume_tokens']} F28ratio={r['resume_vs_f28_ratio']:.2f}")
EOF
```

- **`lossless == false` on ANY run fires METHOD-P §5.4's first kill**, on a single instance. This is the one place
  the "nothing inferred from a single draw" rule does not soften a result: a lossy resume is a property of the
  mechanism, not a rate. If it fires, §3's "resume into the same step" is withdrawn and escalation becomes
  activity-terminal.
- **`resume_vs_f28_ratio > 1.5`** fires the second kill: a mid-graph gate is priced like a stage gate plus the
  graph's replay.
- **`second_arrival_minted_a_new_mark == false`** reverses §9b's A8 decision and the gate step gains an explicit
  `rearm` flag.

Compare against the `control` arm (no kill) for the resume-cost baseline, and against F28's flat ~18.6k for a
stage-level gate.
