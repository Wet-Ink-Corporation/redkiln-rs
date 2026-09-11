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

`COMMIT-2026-09-09.sh` records it and it is still live: `git status` shows whole-file `M` on `LICENSE`,
`.gitignore`, `references/2026-08-31-*.md`, `references/2026-09-0{1,2,3,4}-*.md`, `references/conformance-*.md`,
`references/research/2026-09-03-*.md`, `spikes/FINDINGS.md` and others, with **no content change** — line-ending
normalisation on this Windows checkout. Check before you stage anything:

```bash
git -C D:/repos/redkiln-rs diff -w --stat        # anything listed here with 0 real changes is CRLF churn
git -C D:/repos/redkiln-rs checkout -- <file>    # revert those, or set core.autocrlf, BEFORE staging
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
  || echo "ANCHOR MISSING in .gitignore — stop and read it before appending"
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
git add spikes/results/instrument-checks/RUNBOOK-instrument.txt
# Re-derive rather than trusting this sentence — the tree is the truth:
git status --porcelain | grep -v '^??' ; echo "--- untracked, expected EMPTY after 1.2:" ; git status --porcelain | grep '^??'
git commit -F - <<EOF
Phase 0, part 1: run noise ignored, instrument recorded

Instrument at time of commit: claude $(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)
(HANDOFF's original pin was 2.1.263; P7 straddled 2.1.265; 2026-09-09 measured 2.1.266; 2026-09-10 measured
2.1.268 — the F37 §7 lineage breaks are recorded, not repaired). Rust toolchain pin 1.98.1 (DECISIONS-2026-09-11 #7);
the spike programme ran on 1.97.1.

The set is 110 ADR records (0001–0112, 0090/0091 reserved): 106 accepted, 3 superseded, 1 proposed (0079, still
deliberately undecided until Step 4.4). It is NOT design final: S-BIN, S-H1c and P8's unrun arms gate that label.
EOF
```

If you are running this file against a tree where the remediation merge has *not* landed — a fresh clone at
`e3b0bac`, or a rollback per Step 9 — stop: Step 0's Checkpoint asserts the merge, and nothing below is executable
without the records it carries.

**Checkpoint 1.** All seven:

```bash
cd D:/repos/redkiln-rs
git status --porcelain                                 # → EMPTY (no output at all). Phase-0 exit (a).
ls references/adr/*.md | wc -l                         # → 112 (110 records + README.md + ERRATA.md) — verified 2026-09-11
git ls-files references/adr/0112-*.md | wc -l          # → 1  the last remediation record is tracked, not just on disk
git ls-files runbook/INDEX.md                          # → runbook/INDEX.md  (the split runbook is tracked)
git log --oneline -1                                   # → the commit this step wrote
git log -1 --format=%B | grep -qF "$(cat spikes/results/instrument-checks/RUNBOOK-instrument.txt)" && echo "instrument quoted from the file"
git log -1 --format=%B | grep -qE 'claude 2\.1\.[0-9]+' && echo "instrument named in the commit message"
```

The sixth line is the PIA-019 regression: it fails if the commit message was typed rather than read from the file.

A non-empty `status --porcelain` here is a failed checkpoint, not a nuisance. Design §10.2 makes it Phase-0 exit
condition (a) verbatim. The last line exists because "record the version in the commit message" was previously an
intention with no check behind it.

**The count moved from 89 to 90 and the reason matters.** The previous revision expected 89 = 88 records + README;
the tree now holds 90 = **89** records + README, because ADR-0089 landed on 2026-09-09 [documented, ADR-0089].
Anyone reading 89 here on a tree that has 90 is reading a stale runbook, not a broken tree — and Step 4.4's
numbering has moved by the same one.

---
