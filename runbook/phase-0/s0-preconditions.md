# Step 0 — Preconditions and safety

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s0` | Phase 0 | tool preflight (00-preamble.md) passed | Checkpoint 0 | — | `s1` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


### 0.1 What the tree actually is (re-derive it; do not read it off this section)

Every previous revision of this section enumerated the working tree and was stale within a day (PIA-019). This
revision states the **rule** and the measurement that held when it was written. **Corrected 2026-09-11: this read
"and Checkpoint 0 re-derives the counts" — it does not. No Checkpoint 0 line counts anything; the count lives in
this section's own block below (the `awk` line), and that is where Step 0 ran it.**

**The rule.** `main` is at the remediation merge, **`66d0ced`** (2026-09-11), on top of the pin the pre-implementation
audit measured, **`009a4ec`** ("Closeout: the F40 pass's fourteen minor findings applied…", 2026-09-10). The merged
branch `remediation/pia-2026-09-11` (eight commits, `0e1b1bc`…`58ae8ef`; the branch itself is deleted) carried, on top
of it: the audit and its instrument (`references/audit/2026-09-10/`), ADR-0092–0114 and `ERRATA.md`,
`DECISIONS-2026-09-11.md`, the register/design/spike-file amendments, this `runbook/` folder, the promoted
`spikes/F30-DRAFT.md`, the S-PORTS and S-GRAMMAR instruments (`spikes/s-ports/`, `spikes/p9/grammar-probe.py`) and
their drafts F41/F42. **Step 0 starts from `main` at or after that merge** — if `git merge-base --is-ancestor 66d0ced main`
exits non-zero, stop and merge it first — 2026-09-11: it exits 0 [re-measured]. The earlier `git log --oneline main | grep -c 66d0ced` form prints `0` and exits **1** on a MISS, so its exit status inverts the sentence, and it matches the sha anywhere in the oneline text including a commit message; the stage files below assume every record they cite exists.

**What is still run noise, measured 2026-09-11 after the merge** — the same seventeen items the 2026-09-09 revision
listed. The remediation's first commit (`7954bd0`) had swept them in with `git add -A`; the merge closeout un-tracked
them again (`git rm --cached`, files kept on disk) so that Step 1.2 disposes of them as written:

```bash
git -C D:/repos/redkiln-rs status --porcelain | grep -v '^?? runbook/'
# ?? spikes/p1-arm-l-haiku.log             # ?? spikes/p1-arm-l-sonnet.log             |
# ?? spikes/p1-arm-l-v2-haiku.log           |
# ?? spikes/p1-arm-l-v2-sonnet.log          |
# ?? spikes/p7-mandatory.log                | 14 tee'd driver logs. 4 OLDER ones are already tracked
# ?? spikes/p7-optional.log                 | (spikes/s-typed.log, s-typed-oldschema.log, s-var.log,
# ?? spikes/p7-recommended.log              |  s-var-2400.log) and a .gitignore rule will not untrack them.
# ?? spikes/p8-0-delay-r1-issue-74-plan-briefs-dead-plan-anchor.log
# ?? spikes/p8-0-delay-r1-issue-79-plan-prds-verifier-false-pass.log
# ?? spikes/p8-0-delay-r1-issue-81-plan-discover-spec-gate-mismatch.log
# ?? spikes/p8-0-delay-r1.log               |
# ?? spikes/p8-0delay.log                   |
# ?? spikes/p8-resume-issue-79-plan-prds-verifier-false-pass.log
# ?? spikes/p8-resume-issue-81-plan-discover-spec-gate-mismatch.log
# ?? spikes/results/p8-state/              <-- suspended graph state
# ?? spikes/runner/target-dev/             <-- two extra build dirs, NOT gitignored
# ?? spikes/runner/target-p8/
```

**Count it, do not read it off this listing**: `git status --porcelain | awk '/^\?\?/{n++} END{print n+0}'` is the number — it always prints a count and exits 0, where `grep -c '^??'` prints `0` but exits **1** on a swept tree [hardened 2026-09-11] — and this
enumeration has been wrong before. The P2 artefacts, the F39/F40 amendments and the 2026-09-09 ` M` lines that
earlier revisions enumerated are all committed (`4519edc` … `009a4ec`); what Step 1 commits is the noise above and
nothing else. `references/adr/` holds **117 records + README.md + ERRATA.md**, so `ls references/adr/*.md | wc -l`
is **119** — Checkpoint 1 asserts that number. *(Corrected 2026-09-12: this read 112 / 114, which was already two
records behind the tree and contradicted the assertion it describes; ADR-0115/0116 then ADR-0090 landed that day. Moved 116 / 118 -> 117 / 119 on 2026-09-13, when Step 6.1 wrote ADR-0091.)* `e3b0bac` survives only as the **pre-design-pass reference point**
that Step 9's rollback and Step 1.3's fresh-clone branch both name.

Do not re-run `spikes/p9/COMMIT-2026-09-09.sh`. Step 1 commits what is left.

### 0.2 Toolchain

```bash
rustup run 1.98.1 rustc --version   # expect exactly: rustc 1.98.1 (48a229cea 2026-09-01)
rustup run 1.98.1 cargo --version   # expect exactly: cargo 1.98.1
rustc --version                     # whatever is on PATH — on this box 1.97.1 (8bab26f4f 2026-07-14); RECORD it
```

**The pin is `1.98.1`, exactly** (DECISIONS-2026-09-11 #7; ADR-0102 makes `rust-version` equal it). The spike
programme — every F-record, the spike runner's lockfile, S-PORTS's first run — ran on **1.97.1**; the build starts
one recorded instrument move away from every measurement (golden constraint 8), and S-PORTS was re-run on 1.98.1
with an identical result [measured, F41]. Stable is 1.98.1 as of 2026-09-03; the R-ASYNC read through 1.98.1 finds
no dyn-async-trait stabilisation (design §10.5, discharged 2026-09-10). Edition 2024 is already settled by
demonstration — `spikes/runner/Cargo.toml` carries `edition = "2024"` and compiles. There is **no
`rust-toolchain.toml` anywhere in this repository**; that is one of the four measured absences ADR-0083 exists to
close, and Step 5 writes it with `channel = "1.98.1"`. Until it exists the toolchain is whatever is on PATH, which
is why every Phase-1 `cargo` invocation in these files is written `cargo +1.98.1 …` until Checkpoint 5 asserts the
file.

### 0.3 Instrument, and the lineage rule

```bash
claude --version   # measured 2026-09-11 at Step 0: 2.1.269 (Claude Code) — the first measurement actually written
                   #   to spikes/results/instrument-checks/RUNBOOK-instrument.txt, which did not exist before then;
                   #   measured 2026-09-10: 2.1.268 (Claude Code); measured 2026-09-09: 2.1.266
```

**Record this before anything dispatches — to a file, never to a sentence in this runbook** (PIA-019: the
previous revision hard-coded "2.1.266" in a commit message that claimed to quote a file). `spikes/HANDOFF.md`
originally pinned **2.1.263** (DECISIONS-2026-09-09 #1, which moved the pin from 2.1.259 and noted the rig treats it
as a *floor*, not an exact pin); P7 then moved to **2.1.265** mid-run; 2026-09-09 measured **2.1.266**; 2026-09-10
measured **2.1.268**; **2026-09-11, at Step 0, measured 2.1.269 (Claude Code) — the move is recorded, not worked
around (Halt 2, golden constraint 8)**. Every later line that needs the version reads
`$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)`.

The rule (F20 / F37 §7): **a version move mid-run splits a comparison.** Two consequences here, both non-optional:

- Any P1 or P2 re-run commissioned by Step 4 is a **new** set, not an extension of the 2.1.263/2.1.265 records. It
  may not be pooled with them, and its write-up names its own version.
- The rig's `doctor` will pass, because it encodes a floor:

  ```bash
  cd D:/repos/redkiln-rs/spikes/runner && cargo build && ./target/debug/redkiln-spike-runner doctor
  # expect FIVE lines; the two below are verified verbatim against runner/src/main.rs:1753 and :1756 (the citation
  # is exact — re-checked 2026-09-11; "expect two lines" was wrong: doctor also prints `claude version`,
  # `API key absent` and `platform`) —
  #   version >= floor : true
  #   environment is fit for a measured run.
  ```

  **A passing `doctor` is not the same as a matched instrument.** Record the exact version in the commit message
  and in any results file this session writes.

### 0.4 Safety

Cut a branch. Nothing below runs on `main`.

```bash
git -C D:/repos/redkiln-rs checkout -b build/phase-0-to-1
```

**Checkpoint 0.** All six must hold before Step 1. Every line is a command with an expected result; the
instrument line is made checkable by writing the version to a file rather than by intending to remember it.

```bash
b=$(git -C D:/repos/redkiln-rs rev-parse --abbrev-ref HEAD); [ "$b" = "build/phase-0-to-1" ] || { echo "FAIL: HEAD is '$b', not build/phase-0-to-1 — §0.4: nothing below runs on main"; exit 1; }; echo "on $b"    # → on build/phase-0-to-1
ls D:/repos/redkiln-rs/references/adr/0114-*.md >/dev/null 2>&1 || { echo "FAIL: references/adr/0114-*.md absent"; exit 1; }; test -f D:/repos/redkiln-rs/runbook/INDEX.md || { echo "FAIL: runbook/INDEX.md absent"; exit 1; }; echo "remediation present"   # → the 2026-09-11 remediation is in the tree; the command globs ADR-**0114**, not ADR-0111 — both are present, the comment was wrong [corrected 2026-09-11]
v=$(rustup run 1.98.1 rustc --version); rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: rustup run 1.98.1 rustc exited $rc"; exit 1; }; printf '%s\n' "$v" | grep -q '^rustc 1\.98\.1 ' || { echo "FAIL: the pin is 1.98.1 exactly, got '$v'"; exit 1; }; echo "$v"    # → rustc 1.98.1 (48a229cea 2026-09-01); only the 1.98.1 pin is asserted, not the build hash — the PATH rustc is recorded, not asserted
v=$(claude --version); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: claude --version exited $rc — instrument NOT recorded, file untouched"; exit 1; }
printf '%s\n' "$v" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+' || { echo "FAIL: no version in '$v' — instrument NOT recorded, file untouched"; exit 1; }
printf '%s\n' "$v" > D:/repos/redkiln-rs/spikes/results/instrument-checks/RUNBOOK-instrument.txt
echo "$v"
                                                          # → validates the capture, THEN writes e.g. "2.1.269 (Claude Code)" (measured 2026-09-11) to a FILE; read it back, never retype it.
                                                          #   The old `claude --version | tee <file>` form put claude on the LEFT of a pipe (pipeline status is tee's) AND tee opens O_TRUNC before reading stdin: a failed instrument exited 0 and left a 0-byte file [hardened 2026-09-11].
f=D:/repos/redkiln-rs/spikes/results/instrument-checks/RUNBOOK-instrument.txt; test -s "$f" || { echo "FAIL: $f absent or empty"; exit 1; }; grep -qE '^[0-9]+\.[0-9]+\.[0-9]+' "$f" || { echo "FAIL: no version in $f: '$(cat "$f")'"; exit 1; }; echo "instrument recorded: $(cat "$f")"   # → instrument recorded: 2.1.269 (Claude Code); non-empty AND a version, so the downstream `grep -qF "$(cat …)"` in s1 Checkpoint 1 can never degrade to an empty pattern [hardened 2026-09-11]
( cd D:/repos/redkiln-rs/spikes/runner && cargo build ) || { echo "FAIL: the rig does not build — Halt 1"; exit 1; }
out=$(cd D:/repos/redkiln-rs/spikes/runner && ./target/debug/redkiln-spike-runner doctor); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: doctor exited $rc"; exit 1; }
printf '%s\n' "$out" | grep -qF 'version >= floor : true' || { echo "FAIL: doctor did not print 'version >= floor : true'"; exit 1; }
printf '%s\n' "$out" | grep -qF 'environment is fit for a measured run.' || { echo "FAIL: doctor did not print the fitness line"; exit 1; }
printf '%s\n' "$out"   # → exit 0, and the two lines §0.3 names are ASSERTED, not eyeballed (main.rs:1753, :1756). A passing floor is NOT a matched instrument (Halt 2).
```

`spikes/results/instrument-checks/` already exists. That file is what Step 1.3's commit message quotes, and what
Checkpoint 1 greps for — so "recorded in writing" stops being an intention and becomes a `test -s`.

The tree is **not** required to be clean at Checkpoint 0 — Step 1 is what makes it clean. What is required is that
you know exactly what is dirty and why, and that the instrument version is on disk.

---
