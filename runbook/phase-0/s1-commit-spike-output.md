# Step 1 — Commit the spike output (Phase 0, part 1)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s1` | Phase 0 | Checkpoint 0 | Checkpoint 1 | `s0` | `s3` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


### 1.1 The CRLF trap, first

`COMMIT-2026-09-09.sh` records it. **Measured 2026-09-11 at Step 1: NOT live on this tree — `git diff -w --stat`
and `git status --porcelain | grep -v '^??'` were both empty, not one tracked file modified, so the revert step
below had no work to do. Check anyway; the trap is real when it fires.** As recorded: `git status` shows whole-file `M` on `LICENSE`,
`.gitignore`, `references/2026-08-31-*.md`, `references/2026-09-0{1,2,3,4}-*.md`, `references/conformance-*.md`,
`references/research/2026-09-03-*.md`, `spikes/FINDINGS.md` and others, with **no content change** — line-ending
normalisation on this Windows checkout. Check before you stage anything:

```bash
git -C D:/repos/redkiln-rs diff -w --stat        # anything listed here with 0 real changes is CRLF churn
# Revert exactly those, or set core.autocrlf, BEFORE staging. Name the paths in CRLF_PATHS: a bare
# <file> is a bash redirect, not an argument (the hazard s7d and s8 (1) both flag).
CRLF_PATHS=""                                    # e.g. CRLF_PATHS="spikes/results/a.md spikes/results/b.md"
test -n "$CRLF_PATHS" && git -C D:/repos/redkiln-rs checkout -- $CRLF_PATHS
```

### 1.2 Remove the run noise

Two extra build directories exist because the main `target/` was locked by a live run. Neither is gitignored, so
both show as untracked noise. **This `rm -rf` is the one irreversible act in Phase 0** — it is build output and
regenerates from `cargo build`, which is why it is permitted, and Step 9 says so rather than pretending it rolls
back:

```bash
rm -rf D:/repos/redkiln-rs/spikes/runner/target-dev D:/repos/redkiln-rs/spikes/runner/target-p8
```

`spikes/*.log` are tee'd waiter logs, not evidence — the JSONL is the evidence. `spikes/results/p8-state/` is
suspended graph state, regenerable and meaningless without the run that made it; the digests that matter are in
`spikes/results/p8.jsonl`. Add both to `.gitignore` rather than committing them (stage `.gitignore` separately,
because it is CRLF-churned).

**Anchor assertion first (golden constraint 7).** `.gitignore` already carries a paragraph asserting that
`spikes/results/` is *deliberately not ignored*; appending a `spikes/results/p8-state/` rule beside it is a
narrowing, not a contradiction, and Step 2 is where that paragraph is actually rewritten. Assert you are appending
to the file you think you are:

```bash
grep -q 'spikes/results/ is deliberately NOT ignored' D:/repos/redkiln-rs/.gitignore \
  || { echo "FAIL: ANCHOR MISSING in .gitignore — stop and read it before appending"; exit 1; }
```

```bash
cat >> D:/repos/redkiln-rs/.gitignore <<'EOF'

# Waiter logs, tee'd for the `until grep` waiters. The JSONL in spikes/results/ is the evidence.
spikes/*.log

# Suspended graph state; regenerable, and meaningless without the run that made it.
# The digests that matter are in spikes/results/p8.jsonl.
spikes/results/p8-state/

# Extra build dirs cut when target/ was locked by a live run.
spikes/runner/target-*/
EOF
git -C D:/repos/redkiln-rs add .gitignore
git -C D:/repos/redkiln-rs commit -m "chore: ignore waiter logs, p8 state and extra target dirs"
```

**The `spikes/*.log` rule does not untrack the four logs already in the index** — `spikes/s-typed.log`,
`s-typed-oldschema.log`, `s-var.log`, `s-var-2400.log` (verified with `git ls-files 'spikes/*.log'`). A
`.gitignore` rule has no effect on a tracked path. Leave them tracked; they are already history. If you want them
gone, that is a separate `git rm --cached`, and it is not part of this step.

### 1.3 Commit the instrument file

**Everything the earlier revisions of this sub-step staged has landed** — the design pass (`4519edc`), the F39
amendment (`a431c62`), the P2 worksheet and labels (`ff414a1`, `5006dfe`, `a627547`), the F40 closeout (`009a4ec`)
and the 2026-09-11 remediation (the `remediation/pia-2026-09-11` merge) — so the list of files this sub-step used to
enumerate is history, not work (PIA-019). What Step 0 produced and nothing has committed is **one file**: the
instrument record. Stage it, and quote it in the commit message **from the file**, never retyped:

```bash
cd D:/repos/redkiln-rs
test -s spikes/results/instrument-checks/RUNBOOK-instrument.txt \
  || { echo "FAIL: instrument record absent or empty — Step 0 did not produce it; the commit below would quote nothing"; exit 1; }
git add spikes/results/instrument-checks/RUNBOOK-instrument.txt
# Re-derive rather than trusting this sentence — the tree is the truth:
git status --porcelain | grep -v '^??'                 # staged work — expected: the instrument record
u=$(git status --porcelain | grep '^??')               # untracked, expected EMPTY after 1.2
[ -z "$u" ] || { echo "FAIL: untracked paths left after 1.2:"; printf '%s\n' "$u"; exit 1; }
git commit -F - <<EOF
Phase 0, part 1: run noise ignored, instrument recorded

Instrument at time of commit: claude $(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)
(HANDOFF's original pin was 2.1.263; P7 straddled 2.1.265; 2026-09-09 measured 2.1.266; 2026-09-10 measured
2.1.268 — the F37 §7 lineage breaks are recorded, not repaired). Rust toolchain pin 1.98.1 (DECISIONS-2026-09-11 #7);
the spike programme ran on 1.97.1.

The set is 112 ADR records (0001–0114, 0090/0091 reserved): 108 accepted, 3 superseded, 1 proposed (0079, still
deliberately undecided until Step 4.4). It is NOT design final: S-BIN, S-H1c and P8's unrun arms gate that label.
EOF
```

If you are running this file against a tree where the remediation merge has *not* landed — a fresh clone at
`e3b0bac`, or a rollback per Step 9 — stop: Step 0's Checkpoint asserts the merge, and nothing below is executable
without the records it carries.

**Checkpoint 1.** All seven:

```bash
cd D:/repos/redkiln-rs
st=$(git status --porcelain)                           # → EMPTY (no output at all). Phase-0 exit (a).
[ -z "$st" ] || { echo "FAIL: tree not clean — Phase-0 exit (a) not met:"; printf '%s\n' "$st"; exit 1; }
n=$(ls references/adr/*.md | wc -l)                    # → 114 (112 records + README.md + ERRATA.md) — verified 2026-09-11
[ "$n" -eq 114 ] || { echo "FAIL: expected 114 ADR files, found $n"; exit 1; }
k=$(git ls-files references/adr/0114-*.md | wc -l)     # → 1  the last remediation record is tracked, not just on disk
[ "$k" -eq 1 ] || { echo "FAIL: ADR-0114 tracked-file count is $k, expected 1"; exit 1; }
# → runbook/INDEX.md  (the split runbook is tracked)
git ls-files --error-unmatch runbook/INDEX.md \
  || { echo "FAIL: runbook/INDEX.md is not tracked (bare ls-files prints nothing and exits 0)"; exit 1; }
s1c=$(git log -1 --format=%H --grep='^Phase 0, part 1: run noise ignored, instrument recorded$')
test -n "$s1c" || { echo "FAIL: this step's commit is not in history"; exit 1; }
git log --oneline -1 "$s1c"                            # → the commit this step wrote
test -s spikes/results/instrument-checks/RUNBOOK-instrument.txt \
  || { echo "FAIL: instrument record absent or empty — an empty grep -qF pattern matches every line"; exit 1; }
git log -1 --format=%B "$s1c" | grep -qF "$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)" \
  || { echo "FAIL: PIA-019 — the commit message does not quote the instrument file"; exit 1; }
echo "instrument quoted from the file"
git log -1 --format=%B "$s1c" | grep -qE 'claude 2\.1\.[0-9]+' \
  || { echo "FAIL: no 'claude 2.1.N' in the commit message"; exit 1; }
echo "instrument named in the commit message"
```

The sixth line is the PIA-019 regression: it fails if the commit message was typed rather than read from the file.
**Corrected 2026-09-11 — as originally written it did not, and this sentence is why the hole survived review.**
With the instrument file absent, `$(cat …)` is the empty string and `grep -qF ""` matches every line, so the line
printed its success token and exited 0 having asserted nothing (demonstrated on this tree). It is now guarded on
both sides: `test -s` proves the pattern non-empty first, and the message is read from the step's own commit.
This is Halt 8 shape (d).

A non-empty `status --porcelain` here is a failed checkpoint, not a nuisance. Design §10.2 makes it Phase-0 exit
condition (a) verbatim. The last line exists because "record the version in the commit message" was previously an
intention with no check behind it.

**The ADR file count, and why the number keeps moving.** **Superseded 2026-09-11:** this paragraph argued 89 → 90
(88 records + README, then ADR-0089) and contradicted the checkpoint five paragraphs above it, which asserts
**114**. The tree measures 114 — **112 records + `README.md` + `ERRATA.md`** — after the 2026-09-11 remediation
landed ADR-0092–0114 and the errata convention. The lesson the old paragraph was carrying still holds: anyone
reading a smaller number here on a tree that has 114 is reading a stale runbook, not a broken tree, and Step 4.4's
numbering moves with it (the next free number is **0115**).

---
