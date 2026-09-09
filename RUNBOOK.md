# Runbook — Redkiln-RS from here to the end of Phase 1

**Status:** operator-facing build runbook. It is the execution order for
`references/2026-09-08-architecture-design.md` §0.1 and §10.2; where the two disagree the design governs and this
file is wrong. · **Scope:** Phase 0 (close the programme) and Phase 1 (kernel, store, fix and story lanes) only.
Phases 2–4 are out of scope. · **Inputs:** `references/2026-09-08-architecture-design.md` (§0.1, §5.3, §5.9, §9,
§10) · `references/2026-09-08-acceptance-criteria.md` (AC-001–AC-519, and **its** §12.10) · `references/adr/`
(88 records) · `spikes/HANDOFF.md` ("What Ryan does next", "Traps that cost time here") ·
`spikes/DECISIONS-2026-09-09.md` · `references/2026-09-08-evidence-brief.md` §E. · **Tree state:** every path and
every existing command below was re-verified against both working trees on **2026-09-08**. · **Commands that do not
exist yet** — because the code is not written — are marked **`[WILL EXIST]`** and are written in the form they will
take. Nothing else in this file is aspirational.

**What this file decides, stated honestly.** It is not decision-free. It takes **four** decisions and each one is
an ADR, not a runbook line:

| Decision | Where | Record |
| --- | --- | --- |
| The context-vector source | **Step 4** — the blocking gate | **ADR-0089**, superseding ADR-0079 in full |
| Whether S-GIX narrows the git write path | Step 6.1 | a **successor** to ADR-0077, never an edit to it |
| The minimum `git` executable version the wrapper asserts | Step 6.2 constraint 3 | named in that same successor record |
| `doctor`'s unread-artifact window `N` | Step 7d | the `doctor` rule table, cited by AC-472/AC-426 |

Everything else this file does is execution order. **Where a step is tempted to decide something else, it stops.**

**Invoking things.** All commands assume the repo root `D:/repos/redkiln-rs` unless the block opens with its own
`cd`; **every block that needs a working directory now states it**. The rig is
`cd D:/repos/redkiln-rs/spikes/runner && cargo build && ./target/debug/redkiln-spike-runner <cmd>`. Python tooling
runs from the repo root as `python spikes/<lane>/<script>.py`. The predecessor repo — the frozen corpus — is
`D:/repos/redkiln` at `47faec9f46b8d983c2c75f0794164e68415ae706`.

**Shell.** Every shell block in this file is **POSIX and is executed in Git Bash**, which is present on this box at
`/usr/bin/*`. This is not a stylistic preference: the file uses `rm -rf`, `cat <<'EOF'` heredocs, `sed -i`,
`printf`, `wc -l`, `grep -E/-c/-L`, `date -u`, `test -f/-s/-d` and leading `!` negation, **none of which work in
PowerShell** (`rm` there is `Remove-Item` and rejects `-rf`; there are no heredocs; `!` is `-not`). Open Git Bash,
`cd /d/repos/redkiln-rs`, and run them there. PowerShell equivalents are given only where a step has no POSIX
route. Drive paths (`D:/repos/...`) work unchanged in Git Bash.

**Tool preflight — checked 2026-09-08, and two are MISSING.** Run this before Step 0 and install what it names:

```bash
for t in sed wc grep date python cargo rustc git claude; do
  command -v "$t" >/dev/null || echo "MISSING: $t"
done
cargo audit --version  >/dev/null 2>&1 || echo "MISSING: cargo-audit  → cargo install cargo-audit --locked"
cargo insta --version  >/dev/null 2>&1 || echo "MISSING: cargo-insta  → cargo install cargo-insta --locked"
# Measured on this box 2026-09-08: everything above is present EXCEPT cargo-audit and cargo-insta,
# both of which Step 5 requires. Install them before Step 5 or Checkpoint 5 cannot be run at all.
```

---

## Preamble — the golden constraints

These hold for every step below. They are not advice; each one is a measured failure this programme already paid for.

1. **Nothing in the selection path over LLM-emitted facets starts before Step 4's gate clears.** ADR-0079 is
   `proposed`, is the only non-accepted record in the set other than two full supersessions, and **forbids being
   amended into a decision**. Until Step 4 closes, no `require` row may be keyed on an LLM-sourced `criticality`,
   `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro`. `kind` is out of scope of the
   block (human input, ADR-0030 rule 4). The engine's *semantics* are decided and may be built now; its *input* may
   not. [F36: modal `R` identical on 5/20, 3/20, 2/20, 2/20 against an 80% line; 400 dispatches, 0 void, $21.37]

2. **Nothing may depend on a process surviving.** Anything long-running is a durable mark plus a later read. F22
   measured ~12 background kills against one 12,000 s survivor under `Start-Process`; `issue-81` was killed three
   times as a background job and completed in **116 s** in the foreground. Waits are marks, not sleeps; the rig
   already splits `ttl-mark` / `ttl-read` for exactly this reason.

3. **`cargo test` and `cargo clippy` DO NOT rebuild the binary. Always `cargo build` first.** This cost the
   programme four wasted runs and two false conclusions that code was broken. The invocation is
   `cd spikes/runner && cargo build` — *then* run the binary from `target/debug/`.

4. **A rig that compiles and passes its unit tests has NOT been run.** Three E3 defects survived "built and
   validated": `--json-schema` silently inert under a tool-restricted card, a `\\?\` path handed to `git fetch`,
   and `git fetch <abbrev-sha>` failing on every path that did not happen to call `rev-parse` first. None was
   reachable from a unit test, because none touched the filesystem or the CLI. **Validate one item before every
   set**, and prefer a haiku probe to an Opus one when the question is "does this flag work at all".

5. **Every waiter needs a failure alternative.** A marker-only wait spins forever on a dead job — one `until grep`
   waiter spun 25 minutes on a job that had already failed. Every wait is `grep -qE "MARKER|exited with code"` or
   equivalent, never `grep -q "MARKER"`.

6. **A default written for the fast condition costs a record on the slow one.** The 900 s timeout fitted S-SILENT
   and lost a CLEAN record; the ladder ran 900 → 2,400 → 3,600 → one 5,400 s retry. When two conditions differ in
   shape, size the ceiling against the slow one.

7. **Scripted edits need an anchor assertion.** Two silent `str.replace` no-ops were read as behaviour changes.
   Every scripted edit asserts its anchor exists *before* it writes, and fails loudly if it does not.

8. **The instrument version is part of every record.** F20 and F37 §7: a version move mid-run splits a comparison.
   P7 straddled 2.1.263 → 2.1.265, catching exactly `recommended`'s r3 draw on all ten items while `optional` ran
   wholly on 2.1.263 — no effect is detectable, and **the arms are still not instrument-matched**. Record the
   version; never silently continue across a move.

9. **A guard scoped wrong blocks or voids valid work.** HANDOFF, "Traps that cost time here": *"Two guards I wrote
   to protect data were scoped wrong and blocked or voided valid work."* This is the constraint that cuts **both**
   ways, and it is the one this runbook most nearly lost. A guard too narrow lets the failure through
   (`score-arm-d.py`'s facet regex admits seven facets and silently ignores a `recent_deploy` label, §3b); a guard
   too wide refuses a valid run (a whole-second `labelled_at` refused against a microsecond engine stamp in the same
   second, §4.3 — deliberate and correct, but only because it was *chosen*). **When you meet a guard, establish its
   scope before you obey it and before you route around it.** The scope is the finding, not the verdict.

**Where each constraint actually bites.** They are restated at these steps because the preamble is not where an
operator meets them:

| # | Restated at |
| --- | --- |
| 1 — the LLM-facet block | Step 4 (the gate), **Step 5 banner**, **Step 6 banner**, **Step 7a/7b banners**, Step 8 |
| 2 — nothing depends on a process surviving | Step 3a (the dispatch), Step 7c, Step 8 (3), Halt 5 |
| 3 — `cargo build` first | Checkpoint 0, Step 5.3, Step 6.1, Halt 1 |
| 4 — a rig that compiles has not been run | Step 3a, Step 7c |
| 5 — every waiter needs a failure alternative | Step 3a (the dispatch), Halt 5 |
| 6 — a default written for the fast condition | Step 3a (the dispatch timeout) |
| 7 — scripted edits need an anchor assertion | Step 3a, **Step 4.4** (the two ADR edits), Step 1.2 |
| 8 — instrument version in every record | Step 0.3, Step 1.3, Step 3a, Step 4.1, Halt 2 |
| 9 — a guard scoped wrong | Step 3b, Step 4.3, Halt 7 |

---

## Step 0 — Preconditions and safety

### 0.1 What the tree actually is (this differs from what the documents say)

The evidence brief §E and design §10.2 both record "working tree not clean as of 2026-09-09; HEAD is `ab58733` and
nothing from the AFK session was committed". **That is now stale.** Re-verified 2026-09-08:

```bash
git -C D:/repos/redkiln-rs log --oneline -2
# e3b0bac P1 arm L x2 (400 dispatches, 0 void), P7 complete (61 runs), P8 partial, three rig subcommands
# ab58733 P6: v0 catalog (102 activities), lattice engine + 12 packs, F34/F35, decisions 1–10 applied, …
```

`spikes/p9/COMMIT-2026-09-09.sh` **has been run**; HEAD is `e3b0bac`. What remains uncommitted is the 2026-09-08
design pass and the run noise the commit script deliberately left out:

```bash
git -C D:/repos/redkiln-rs status --porcelain
# ?? RUNBOOK.md                            <-- THIS FILE. It is untracked and Step 1 must commit it.
# ?? references/2026-09-08-acceptance-criteria.md
# ?? references/2026-09-08-architecture-design.md
# ?? references/2026-09-08-evidence-augmentation.md
# ?? references/2026-09-08-evidence-brief.md
# ?? references/2026-09-08-success-measures.md
# ?? references/adr/                       (88 records + README.md; `ls references/adr/*.md | wc -l` = 89)
# ?? spikes/*.log                          (14 tee'd run logs; 4 older ones are already tracked)
# ?? spikes/results/p8-state/
# ?? spikes/runner/target-dev/  target-p8/ (two extra build dirs, NOT gitignored)
```

`RUNBOOK.md` is in that list because it is this file, and it was the defect that made Checkpoint 1 unpassable:
`git status --porcelain` cannot go empty while the runbook driving the commit is itself untracked and unlisted.
Step 1.3 stages it.

Do not re-run `COMMIT-2026-09-09.sh`. Step 1 commits what is left.

### 0.2 Toolchain

```bash
rustc --version   # expect 1.97.1 (8bab26f4f 2026-07-14) or later
cargo --version   # expect 1.97.1 (c980f4866 2026-06-30) or later
```

Edition 2024 is already settled by demonstration — `spikes/runner/Cargo.toml` carries `edition = "2024"` and
compiles. There is **no `rust-toolchain.toml` anywhere in this repository**; that is one of the four measured
absences ADR-0083 exists to close, and Step 5 writes it. Until it exists, the toolchain is whatever is on PATH, and
that is a recorded fact rather than a policy.

### 0.3 Instrument, and the lineage rule

```bash
claude --version   # measured 2026-09-09: 2.1.266 (Claude Code)
```

**Record this before anything dispatches.** `spikes/HANDOFF.md` pins **2.1.263** (DECISIONS-2026-09-09 #1, which
moved the pin from 2.1.259 and noted the rig treats it as a *floor*, not an exact pin). P7 then moved to **2.1.265**
mid-run. Today's instrument is **2.1.266** — a third version, and the second lineage break inside the P-series.

The rule (F20 / F37 §7): **a version move mid-run splits a comparison.** Two consequences here, both non-optional:

- Any P1 or P2 re-run commissioned by Step 4 is a **new** set, not an extension of the 2.1.263/2.1.265 records. It
  may not be pooled with them, and its write-up names its own version.
- The rig's `doctor` will pass, because it encodes a floor:

  ```bash
  cd D:/repos/redkiln-rs/spikes/runner && cargo build && ./target/debug/redkiln-spike-runner doctor
  # expect two lines, verified verbatim against runner/src/main.rs:1753 and :1756 —
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

**Checkpoint 0.** All five must hold before Step 1. Every line is a command with an expected result; the
instrument line is made checkable by writing the version to a file rather than by intending to remember it.

```bash
git -C D:/repos/redkiln-rs rev-parse --abbrev-ref HEAD    # → build/phase-0-to-1
rustc --version && cargo --version                        # → 1.97.1 (measured 2026-09-08), edition 2024 available
claude --version | tee D:/repos/redkiln-rs/spikes/results/instrument-checks/RUNBOOK-instrument.txt
                                                          # → writes e.g. "2.1.266 (Claude Code)" to a FILE
test -s D:/repos/redkiln-rs/spikes/results/instrument-checks/RUNBOOK-instrument.txt && echo "instrument recorded"
cd D:/repos/redkiln-rs/spikes/runner && cargo build && ./target/debug/redkiln-spike-runner doctor  # → exit 0
```

`spikes/results/instrument-checks/` already exists. That file is what Step 1.3's commit message quotes, and what
Checkpoint 1 greps for — so "recorded in writing" stops being an intention and becomes a `test -s`.

The tree is **not** required to be clean at Checkpoint 0 — Step 1 is what makes it clean. What is required is that
you know exactly what is dirty and why, and that the instrument version is on disk.

---

## Step 1 — Commit the spike output (Phase 0, part 1)

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

### 1.3 Commit the 2026-09-08 design pass

```bash
cd D:/repos/redkiln-rs
git add references/2026-09-08-architecture-design.md \
        references/2026-09-08-acceptance-criteria.md \
        references/2026-09-08-success-measures.md \
        references/2026-09-08-evidence-brief.md \
        references/2026-09-08-evidence-augmentation.md \
        references/adr/ \
        RUNBOOK.md \
        spikes/results/instrument-checks/RUNBOOK-instrument.txt
git commit -m "Architecture design 2026-09-08, AC/SM registers, ADR-0001–0088

The design supersedes the 2026-09-05 proposal at rev. 4 in full. It is NOT design
final: S-BIN, S-H1c and P8's unrun arms gate that label. 88 ADRs — 85 accepted,
2 superseded, 1 proposed (0079, deliberately undecided).

Instrument at time of commit: claude 2.1.266 (HANDOFF pins 2.1.263; P7 straddled
2.1.265 — F37 §7 lineage break recorded, not repaired)."
```

**Checkpoint 1.** All five:

```bash
cd D:/repos/redkiln-rs
git status --porcelain                                 # → EMPTY (no output at all). Phase-0 exit (a).
ls references/adr/*.md | wc -l                         # → 89 (88 records + README.md) — verified 2026-09-08
git ls-files RUNBOOK.md                                # → RUNBOOK.md  (this file is now tracked)
git log --oneline -1                                   # → the design-pass commit
git log -1 --format=%B | grep -qE 'claude 2\.1\.[0-9]+' && echo "instrument named in the commit message"
```

A non-empty `status --porcelain` here is a failed checkpoint, not a nuisance. Design §10.2 makes it Phase-0 exit
condition (a) verbatim. The last line exists because "record the version in the commit message" was previously an
intention with no check behind it.

---

## Step 2 — Extract `redkiln-corpus` as its own repo, pinned by SHA (Phase 0, part 2)

> ## ⚠ ORDER: RUN THIS **AFTER** STEP 3 AND STEP 4, NOT BEFORE THEM.
>
> **The canonical execution order in this file is 0 → 1 → 3 → 4 → 2 → 5 → 6 → 7 → 8.** Step 2 keeps this position
> in the document because it is Phase 0 part 2, but it is executed fifth.
>
> The previous revision said "AFTER Step 1 and BEFORE Step 4" and that was **wrong, and would have destroyed
> work**: Step 2 does `rm -rf spikes/results` and adds `spikes/results/` to `.gitignore`, while **Step 3b writes
> the three filled label files into `spikes/results/`** (`p1-arm-d-labels-ryan.md`, `p1-arm-l-labels-ryan.md`,
> `p2-labels-ryan.md`) and **Step 4 reads them from there**. Run Step 2 first and Step 3b writes into a directory
> that is about to be deleted, or into one that no longer exists.
>
> Running Step 2 last also removes the whole `--engine` / `--labels` / `--cases` re-pointing problem: by the time
> the split lands, the scorers have already run against `spikes/results/` in this repo, and the filled label files
> travel into `redkiln-corpus` as part of the corpus commit — which is where ADR-0013 wants them anyway.
>
> **What is not permitted is starting Step 5 with the split undone.** `AC-464` and `AC-465` are Phase-1 CI, and a
> product repo that still carries `s-mig.jsonl` cannot switch its own residency lint on.

ADR-0013: the corpus becomes a separate `redkiln-corpus` repository — **pinned SHA, fixtures, JSONL and the spike
runner all live there**, and everything else depends on it by SHA. Today **`redkiln-corpus` does not exist**, and
`spikes/corpus/`, `spikes/results/*.jsonl` and `spikes/runner/` are all tracked in this repo. **AC-465 is violated
by construction today** and the corpus-residency lint cannot be switched on until this step runs.

History preservation is not required by ADR-0013 — what is required is a pinnable SHA. Take the plain route.

```bash
mkdir -p D:/repos/redkiln-corpus && cd D:/repos/redkiln-corpus && git init

# The unit ADR-0013 names, and nothing else.
cp -r D:/repos/redkiln-rs/spikes/corpus   ./corpus
cp -r D:/repos/redkiln-rs/spikes/results  ./results
cp -r D:/repos/redkiln-rs/spikes/runner   ./runner
rm -rf ./runner/target ./runner/target-dev ./runner/target-p8   # build output, never corpus
cp -r D:/repos/redkiln-rs/spikes/schemas ./schemas              # p1-classify.schema.json — a runner input
cp -r D:/repos/redkiln-rs/spikes/prompts ./prompts              # the cards the records were produced under
# schemas/ and prompts/ are COPIED, not moved: the records cannot be replayed without the card and
# schema they were produced under, and METHOD-P/FINDINGS in this repo still cite them by path.
# Only corpus/, results/ and runner/ are removed from redkiln-rs below — that is ADR-0013's unit.

git add -A && git commit -m "redkiln-corpus v0: pinned corpus, spike results, spike runner (ADR-0013)"
git rev-parse HEAD    # ← this is the pin. Record it.
```

Then record the pin in this repo and delete the originals:

```bash
cd D:/repos/redkiln-rs
cat > CORPUS_PIN <<'EOF'
# redkiln-corpus pin (ADR-0013). Every corpus read resolves through this SHA via the
# object store, never through a working tree (AC-464). A run against an unpinned
# reference, or against a corpus checkout with uncommitted changes, is REFUSED.
CORPUS_REPO=D:/repos/redkiln-corpus
CORPUS_SHA=<paste the rev-parse output above>
PINNED_AT=2026-09-09

# The frozen predecessor corpus this repo pins in turn, unchanged:
REDKILN_REPO=D:/repos/redkiln
REDKILN_SHA=47faec9f46b8d983c2c75f0794164e68415ae706
EOF
git rm -r --cached spikes/corpus spikes/results spikes/runner
rm -rf spikes/corpus spikes/results spikes/runner
printf '\nspikes/corpus/\nspikes/results/\nspikes/runner/\n' >> .gitignore
git add CORPUS_PIN .gitignore && git commit -m "Extract redkiln-corpus at a pinned SHA (ADR-0013); AC-465 becomes enforceable"
```

**Reconcile the `.gitignore` paragraph you are contradicting.** `.gitignore` currently carries, verbatim:

> `# spikes/results/ is deliberately NOT ignored: charter §14.2 makes the raw`
> `# records a deliverable, and a results file that has to be force-added is a`
> `# results file that eventually goes missing.`

Adding `spikes/results/` to the ignore list makes that paragraph false, and a stale comment asserting the opposite
of the rule three lines below it is exactly the kind of thing that gets a results file force-added later. Rewrite
it in the same commit, with an anchor assertion first (golden constraint 7):

```bash
cd D:/repos/redkiln-rs
grep -q 'spikes/results/ is deliberately NOT ignored' .gitignore \
  && python - <<'PY' \
  || echo "ANCHOR MISSING — the paragraph is not there; read .gitignore and stop"
import io,sys
p=".gitignore"; t=io.open(p,encoding="utf-8",newline="").read()
old="""# spikes/results/ is deliberately NOT ignored: charter §14.2 makes the raw
# records a deliverable, and a results file that has to be force-added is a
# results file that eventually goes missing."""
assert t.count(old)==1, "anchor not unique — stop, do not write"
new="""# spikes/results/ moved to D:/repos/redkiln-corpus at the pin in CORPUS_PIN
# (ADR-0013). Charter §14.2 still makes the raw records a deliverable — they are
# a deliverable OF THE CORPUS REPO now, and AC-465 forbids them here."""
io.open(p,"w",encoding="utf-8",newline="").write(t.replace(old,new))
print("gitignore paragraph reconciled")
PY
git add .gitignore && git commit -m "chore: reconcile the .gitignore results paragraph with ADR-0013"
```

**Checkpoint 2.** All six:

```bash
git -C D:/repos/redkiln-corpus rev-parse HEAD                  # → a 40-hex SHA
grep CORPUS_SHA D:/repos/redkiln-rs/CORPUS_PIN | grep -qE '=[0-9a-f]{40}$' && echo "40-hex, unabbreviated"
grep -q "$(git -C D:/repos/redkiln-corpus rev-parse HEAD)" D:/repos/redkiln-rs/CORPUS_PIN && echo "pin matches HEAD"
git -C D:/repos/redkiln-corpus status --porcelain              # → EMPTY (AC-464 refuses a dirty checkout)
git -C D:/repos/redkiln-rs ls-files | grep -E '^spikes/(corpus|results|runner)/' | wc -l   # → 0
ls D:/repos/redkiln-corpus/results/p1-arm-d-labels-ryan.md \
   D:/repos/redkiln-corpus/results/p1-arm-l-labels-ryan.md \
   D:/repos/redkiln-corpus/results/p2-labels-ryan.md         # → all three; Step 3b's work travelled
```

The last line is the check that the previous ordering would have failed silently: the filled panels are the most
expensive artifact in Phase 0 and the only one no re-run can reproduce blind.

Note `wc -l` rather than `grep -c` on the residency line: `grep -c` **exits 1 when the count is 0**, so the check
you want to pass is the one that returns a failing exit status. Piping to `wc -l` keeps the number and drops the
trap.

The abbreviation clause is not pedantry: an abbreviated SHA handed to `git fetch` is one of the three defects that
survived "built and validated" in E3. Forty hex characters, everywhere.

**One thing to carry into Step 5.** Once `spikes/runner/` is gone from this tree, the root `Cargo.toml` Step 5.1
writes is safe. If for any reason you have **not** removed it, that root workspace manifest will break the rig:
cargo walks up from `spikes/runner/Cargo.toml`, finds a workspace root whose `members = ["crates/*"]` does not
include it, and errors with *"current package believes it's in a workspace when it's not"* — which is Halt
condition 1 arriving from a direction you did not expect. Step 5.1 carries the `exclude` line that prevents it.

---

## Step 3 — The review-consumer check, and the three blind label files (Phase 0, part 3)

Two independent pieces of work, each with its own checkpoint. Neither costs more than one dispatch.

### 3a — The one-dispatch check on the review consumer (D-FIXCONSUMER)

The defect is real and verifiable right now:

```bash
grep -n "review\.pass" D:/repos/redkiln/workflows/forge-fix.js
#  982:if (review && review.pass === false) {          <-- THE DEFECT
# 1019:if (!review || review.pass !== true) {           <-- already correct; do NOT touch it
# 1029, 1148, 1184, 1270, 1273: reads and renderings, not gates
```

**Read that second line before you do anything else.** `forge-fix.js` already contains one correctly-guarded site
at line 1019, in the exact form the fix below writes. That is why the previous revision's Checkpoint 3a
(`grep -c "review.pass !== true" → 1`) was **inverted**: it passed *before* the fix, on line 1019 alone, and would
have failed *after* the fix, at 2. A checkpoint that is green when you have done nothing is worse than none. The
counts below are the corrected ones, and both are given so you can tell the two states apart.

Line 982 reads an **ABSENT** verdict as approval. F20 is the measured mechanism that produces exactly that, silently:
an agent card's `tools` allowlist disables `--json-schema`, `structured_output` is absent, and the process exits
**0** with `subtype: success`. Whether the Workflow runtime shares the defect is **UNVERIFIED and is one dispatch
to settle**.

**Fix the consumer either way** — the fix does not depend on the answer:

```bash
F=D:/repos/redkiln/workflows/forge-fix.js

# Anchor assertion first (golden constraint 7): a silent no-op replace reads as a behaviour change.
# The anchor must be present EXACTLY ONCE, or the replace is not the edit you think it is.
n=$(grep -c 'if (review && review\.pass === false) {' "$F")
before=$(grep -c 'review\.pass !== true' "$F")          # → 1 (line 1019, pre-existing)
if [ "$n" -ne 1 ]; then
  echo "ANCHOR COUNT $n (expected 1) — DO NOT WRITE"
else
  sed -i 's/if (review && review\.pass === false) {/if (!review || review.pass !== true) {/' "$F"
  after=$(grep -c 'review\.pass !== true' "$F")         # → 2 (982 and 1019)
  echo "before=$before after=$after  (expected before=1 after=2)"
  grep -n 'review\.pass !== true' "$F"                  # → must print BOTH 982 and 1019
fi
```

Note the shape: no `exit 1`. The previous revision's `|| { echo …; exit 1; }` would have **closed the operator's
interactive shell** on a missed anchor, and would also have fired on a *sed* failure while blaming the anchor. An
`if` says which thing failed and leaves you in the shell to look at it.

The `sed` pattern does not escape the `&&`: inside single quotes the shell passes it through, and `&` is not
special in a sed *pattern* (only in the *replacement*, which contains none).

> **The corpus pin is not disturbed by this.** `D:/repos/redkiln` is frozen at `47faec9f…`, and every corpus read
> resolves at that SHA through the object store, never through the working tree (`spikes/corpus/PINNED_SHA` says so
> in terms; AC-464 makes it a refusal). A working-tree edit to `workflows/forge-fix.js` changes what the *old
> redkiln runs*; it changes nothing any measurement reads. Do not commit it into the frozen history.

The dispatch itself — does the **Workflow runtime** reproduce the absent-verdict path, as the CLI path does?
**`[WILL EXIST]`** There is no rig subcommand for this; `runner/src/main.rs` carries `doctor`, `probe`,
`ttl-probe`, `ttl-mark`, `ttl-read`, `corpus-manifest`, `gate-probe`, `reconcile-replay`, `review-probe`,
`digest-probe`, `human-gate`, `classify-probe`, `graph-probe`, `repro-probe` and `rehydrate`, and none of them
poses this question. The check to write:

```bash
# [WILL EXIST] — one haiku-scale dispatch through the Workflow runtime with a tool-restricted
# card that omits StructuredOutput, asserting on the runtime's own return value.
cd D:/repos/redkiln-rs/spikes/runner && cargo build     # golden constraint 3: test/clippy do not rebuild
./target/debug/redkiln-spike-runner fix-consumer-probe \
    --runtime workflow \
    --card-omits StructuredOutput \
    --timeout-secs 3600 \
    --out D:/repos/redkiln-rs/spikes/results/d-fixconsumer.jsonl
# PASS  = the runtime surfaces a typed absence (a StepError, or an explicit null the caller must handle)
# FAIL  = the runtime hands back a value indistinguishable from an approval — the CLI-path defect is shared
```

Four constraints land on this one command and each changed a character of it:

- **GC3** — the `cargo build` is not decoration; `cargo test`/`cargo clippy` do not rebuild the binary.
- **GC4** — validate this one item before any set, and prefer **haiku**: when the question is "does this flag work
  at all", a haiku probe answers it and an Opus one only costs more.
- **GC8** — stamp the instrument version into the record. Today that is **2.1.266**, a third version and the second
  lineage break in the P-series; this record may not be pooled with the 2.1.263/2.1.265 sets.
- **GC6** — the `--timeout-secs 3600` is deliberate and is **not** the fast-condition default. The 900 s ceiling
  fitted S-SILENT and cost a CLEAN record; a Workflow-runtime round trip is the slow shape here. Size against the
  slow one.
- **GC2 / GC5** — run it in the **foreground**. If you must wait on it, the wait is
  `grep -qE "MARKER|exited with code"`, never `grep -q "MARKER"`: one marker-only waiter spun 25 minutes on a job
  that had already failed. `issue-81` was killed three times as a background job and finished in 116 s in the
  foreground.

The `--out` path is absolute on purpose. The previous revision wrote `--out results/d-fixconsumer.jsonl` with no
`cd` anywhere in the block — the one silent working-directory dependency in the file.

**Checkpoint 3a.** Both:

```bash
grep -c 'review\.pass !== true'  D:/repos/redkiln/workflows/forge-fix.js   # → 2   (982 fixed + 1019 pre-existing)
grep -c 'review\.pass === false' D:/repos/redkiln/workflows/forge-fix.js   # → 0   (the defect is gone)
```

Two lines, not one, and the second is the one that actually discriminates: `=== false` reaching zero is only
possible if line 982 changed. Before the fix these read **1** and **1**.

and either a `d-fixconsumer.jsonl` record with a PASS/FAIL verdict, **or** a written line in
`spikes/DECISIONS-2026-09-09.md` recording that the dispatch was deferred and that the runtime's behaviour is
therefore still UNVERIFIED. Silence is not a checkpoint. The **principle** is already generalised into Phase 1
(AC-282, AC-439, §9.6's `spoke()` row: absence is a block, never a pass) — but generalising it does not discharge
the specific check, and this runbook does not pretend it does.

### 3b — Generate and hand over the three blind label files

All three exist and are already generated. Verify, do not regenerate:

```bash
cd D:/repos/redkiln-rs
grep -cE '^\| .+ \| (toolchain|change|binary_change|auth|data_migration|ui|flag_exists|recent_deploy) \|' \
     spikes/results/p1-arm-d-labels-RYAN-TODO.md    # → 160  (20 items x 8 facets)
grep -cE '^\| (_archive|support)' spikes/results/p1-arm-l-labels-RYAN-TODO.md
                                                    # →  20  (one per item: kind + 4 ordinal + 3 boolean)
grep -cE '^\| P[0-9]{2} \|'       spikes/results/p2-labels-RYAN-TODO.md
                                                    # →  41  (12 perturbation cases x their downstream sets)
```

`spikes/results/p7-deviation-labels-RYAN-TODO.md` carries **no data rows** — its only table row is the placeholder
"no deviation requests recorded yet". The channel was offered **71 times and used 0**. There is nothing to
adjudicate; do not wait on it. Note it is also the one file where METHOD-P §0 rule 1 does **not** apply and there
is **no timestamp guard** — it labels the *request*, which did not exist before the run, and that is disclosed in
the file itself. Do not generalise its looseness to the other three.

Copy each to a filled name, because the scorers read the filled copy and the `-RYAN-TODO` file stays as the blank
of record:

```bash
cp spikes/results/p1-arm-d-labels-RYAN-TODO.md spikes/results/p1-arm-d-labels-ryan.md
cp spikes/results/p1-arm-l-labels-RYAN-TODO.md spikes/results/p1-arm-l-labels-ryan.md
cp spikes/results/p2-labels-RYAN-TODO.md       spikes/results/p2-labels-ryan.md
```

**The rules for filling them, which are METHOD-P §0 rule 1 and are not negotiable:**

- Fill **from the item alone**, before opening any engine output. `spikes/results/p1-arm-d.jsonl` (640 records),
  `p1-arm-l.jsonl`, `p1-arm-l-v2.jsonl` and `p2-cases.jsonl` are all unblinding material. Do not open them.
- For P2, label **from the diff alone**:
  `git -C D:/repos/redkiln diff <replaced_with_rev> 47faec9f -- .bklg/<item>/<upstream>`. The advisory cases
  A01–A06 are labelled *after* the engine, in a separate file (METHOD-P §2.3).
- Leave `?` where the item alone cannot tell. `recent_deploy` is underivable at this pin (no deploy log) — label
  it `?` unless you know otherwise. `binary_change` is measured but **is not in `facets.toml`**, so it can feed no
  `require` row regardless of how you label it (DECISIONS-2026-09-09 #11; verified — `facets.toml` has no
  `[facet.binary_change]`).

**Guard scope, before you trust the row count (golden constraint 9).** The label file has **eight** facet columns;
`score-arm-d.py`'s parser regex admits **seven**:

```bash
grep -n 'toolchain|change|binary_change' D:/repos/redkiln-rs/spikes/p1/score-arm-d.py
# (toolchain|change|binary_change|auth|data_migration|ui|flag_exists)   <-- recent_deploy is NOT here
```

So a `recent_deploy` label is **read by nothing** — it is not scored, and its absence is not reported. That is
harmless *given* the instruction above (label it `?`, which the scorer would skip anyway), and it is stated here
so that a filled `recent_deploy` column is never mistaken for a scored one. Do not widen the regex to "fix" it: a
facet with no feed in the frozen corpus has no ground truth to score against, and widening the guard is how the
programme's two mis-scoped guards happened.
- For arm L, copy the ordinal anchors out of `spikes/prompts/p1-classify.md` **verbatim** before filling. A panel
  scored against remembered anchors is scored against different anchors.

### The UTC `...Z` timestamp trap — read this before you stamp anything

`labelled_at` is compared to the engine's earliest record **lexicographically, as a string**. The engine writes its
`at` field as a naive UTC ISO-8601 with a literal `Z` and microsecond precision (`2026-09-09T06:06:31.482913Z`).
`score-arm-d.py` then does, literally, `if ts >= first: raise SystemExit(...)`.

**A stamp carrying a local offset instead of `Z` mis-compares, and one direction of the mis-compare is SILENT:**

- A **negative** offset (`2026-09-08T23:00:00-05:00`, which is `2026-09-09T04:00Z`) sorts *before* an engine
  timestamp it actually follows. The guard passes. **A post-dated label is accepted as blind, silently, and the
  whole blind-panel discipline is void with nothing printed.**
- A **positive** offset (`2026-09-09T13:00:00+10:00`, which is `2026-09-09T03:00Z`) sorts *after* an engine
  timestamp it actually precedes. The guard refuses a valid label, loudly.

Only the loud one tells you. Produce the stamp with a command, never by hand:

```bash
date -u +%Y-%m-%dT%H:%M:%SZ                                              # → 2026-09-09T06:06:31Z
python -c "import datetime;print(datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'))"
```

```powershell
[DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ss") + "Z"                 # → 2026-09-09T06:06:34Z
```

Paste that into `labelled_at:` and set `labeller:`. Verify before moving on:

```bash
grep -h "^labelled_at:" spikes/results/p1-arm-d-labels-ryan.md \
                        spikes/results/p1-arm-l-labels-ryan.md \
                        spikes/results/p2-labels-ryan.md
# every line must end in Z, contain no '+' and no '-' after the 'T', and contain no "FILL"
```

**Checkpoint 3b.** All five. **The first line is an existence guard and is not optional** — the previous revision
opened with `! grep -l "FILL-ME" …`, and a `grep` over files that do not exist exits **2**, which `!` turns into a
pass. That checkpoint went green when Step 3b had not been done at all.

```bash
cd D:/repos/redkiln-rs
for f in spikes/results/p1-arm-d-labels-ryan.md \
         spikes/results/p1-arm-l-labels-ryan.md \
         spikes/results/p2-labels-ryan.md; do
  test -f "$f" || echo "MISSING: $f"; done                                                        # → no output
grep -l "FILL" spikes/results/p1-arm-d-labels-ryan.md spikes/results/p1-arm-l-labels-ryan.md \
               spikes/results/p2-labels-ryan.md | wc -l                                           # → 0
grep -h "^labelled_at:" spikes/results/*-labels-ryan.md | grep -cE 'Z$'                            # → 3
grep -cE '^\| .+ \| (toolchain|change|binary_change|auth|data_migration|ui|flag_exists|recent_deploy) \| *[^ |]' \
     spikes/results/p1-arm-d-labels-ryan.md                                                        # → > 0, i.e. rows carry values
grep -cE '^\| P[0-9]{2} \|.*\| *(yes|no) *\|' spikes/results/p2-labels-ryan.md                     # → 41 of 41 rows answered
```

`grep -l "FILL"`, not `"FILL-ME"`, and piped to `wc -l` rather than negated: `score-arm-d.py` tests for the
substring `FILL-ME` but `reconstruct-derived-from.py` tests for `FILL`, so the looser string is the one that
matches both guards. The blank row counts these are filled from — **160 / 20 / 41** — were re-verified against the
`-RYAN-TODO` originals on 2026-09-08 and are correct.

Empty cells and `?` cells are legitimate answers and are counted as unlabelled by the scorer, not as errors. What
is not legitimate is a file where `labelled_at` is set and the rows are untouched.

### 3c — Phase-0 exit (c) and (d): what this runbook cannot deliver, said plainly

Design §10.2 gives Phase 0 **four** exit conditions, and Phase 1's "Enter when" is "Phase 0 exit (a)–(d)". Steps 1
and 2 discharge (a) and (b). **This runbook cannot discharge (c) or (d) as written, and the previous revision did
not say so** — it went from Step 4 to Step 5 as though Phase 1 entry were unconditional. Two shortfalls, both
verified 2026-09-08:

| Exit | What §10.2 requires | State | What you must do |
| --- | --- | --- | --- |
| **(c)** | Every §10.5 register row marked "gates Phase 1" has a written finding with an F-number in `spikes/FINDINGS.md`, **or the row is explicitly re-scoped by an ADR** | **S-BIN is "Registered, unrun"** and is marked **YES, gates Phase 1**. It needs a real Unreal project, ~10 Opus dispatches and UE automation runs — it is the one Phase-0 row this runbook does not cost, schedule or contain | Take the **re-scope branch the design already provides**: record in ADR-0089, or a sibling record, that S-BIN's gate is scoped to the `ChangeSummary` type only, which Step 5 already refuses to build (**AC-475**, no synthesised value). That is a decision, so it is an ADR, not a runbook line |
| **(d)** | The blind panels are filled: **P1 arm D scored**, and `e3-severity-annotation-BLANK.md` **returned** | Step 3b delivers the P1/P2 half. The E3 half is **not delivered**: `spikes/results/e3-severity-annotation-ryan.md` exists but its own header says *"Filled by the model at Ryan's direction… NOT BLIND"* and that κ is **forfeited**; the pristine `e3-severity-annotation-BLANK.md` is still blank | Either annotate the BLANK from scratch without reading either filled sheet — which restores κ — or record E3-SEV as deferred. **E3-SEV does not gate Phase 1** (§10.5 "Gates Phase 1?" = No), so deferring is legitimate; leaving it unmentioned was not |

```bash
cd D:/repos/redkiln-rs
grep -n "NOT BLIND" spikes/results/e3-severity-annotation-ryan.md   # → the header's own disclosure
grep -c "FILL" spikes/results/e3-severity-annotation-BLANK.md       # → > 0: the pristine blank is still blank
```

**Checkpoint 3c.** One of the two, in writing, for each row — a check that a *sentence* exists, because there is
no command that can check a decision was taken:

```bash
cd D:/repos/redkiln-rs
grep -rn "S-BIN" references/adr/0089-*.md spikes/DECISIONS-2026-09-09.md   # → the re-scope, or the deferral
grep -rn "E3-SEV" spikes/DECISIONS-2026-09-09.md                           # → returned, or deferred, with a date
```

Silence on either is a failed checkpoint. Design §10.2 also names the price of getting this wrong: *"A Phase-1
commit that touches `ChangeSummary` or an `llm`-sourced facet before (c) is out of order."*

---

## Step 4 — THE BLOCKING GATE

**Nothing in the kernel's selection path proceeds until this step's checkpoint passes.** That is ADR-0079 and
AC-071, not a review convention.

### 4.1 State the decision rule BEFORE you look at any number

Write this into the ADR draft first, so the decision is not made post-hoc. Three clauses, each already fixed by a
pre-registration or by AC-071; nothing here is invented at scoring time.

| Clause | Source | The line | Where it is measured |
| --- | --- | --- | --- |
| **A. Modal-`R` identity** | METHOD-P §1.4 | ≥ **80%** of items show an identical modal required set `R(stage, ctx)` across the draws | `p1-arm-l-analysis.txt` / `p1-arm-l-v2-analysis.txt`. **Already measured: 5/20, 3/20, 2/20, 2/20 (0.25/0.15/0.10/0.10). Wilson 95% upper bounds 46.9 / 36.0 / 30.1 / 30.1 — every one sits far below the line.** |
| **B. Arm-D per-facet precision** | METHOD-P §1.4 / the scorer's own printed verdict | precision ≥ **0.95** → that facet **leaves the model** and becomes `source = "derived"`; below → it stays LLM-emitted, and therefore blocked | `score-arm-d.py`, which prints `LEAVES THE MODEL (>=0.95)` or `stays LLM-emitted` per facet per source |
| **C. The AC-071 five clauses, in full** | AC-071 | identity ≥ 0.80 **AND** n ≥ 20 items **AND** ≥ 10 draws/item **AND** arm-D panel `scored` **AND** `corpus_sha` equals the catalog corpus pin | the `redkiln lint --facet-stability` check, Phase 1 |

**The decision the rule produces, stated in advance so it is not chosen after the fact:**

- **If clause A fails** — which, on the records that already exist, it does, 4 times out of 4 — then **every
  ordinal axis is `source = "human"` for Phase 1**, supplied or confirmed at the triage gate. That is not a
  workaround; it is the hedge design §10.3 already ships, and it is what SM-024 measures the operator cost of.
- **Clause B is scored per facet and per source independently of A.** A *derived* facet (`toolchain`, `change`,
  `auth`, `data_migration`, `ui`, `flag_exists`) that clears 0.95 precision leaves the model on its own warrant —
  it is derived from paths and docs, not classified by a dispatch, so ADR-0079's block does not reach it. Record
  each such facet by name.
- **Clause C is the only thing that unblocks LLM-sourced ordinals**, and it needs a full P1 re-run at n ≥ 20 ×
  ≥ 10 draws in addition to the panel. That re-run is **not** part of Phase 1's critical path. Commission it or
  don't; Phase 1 proceeds either way, on human-sourced axes.
- **A temperature spike is explicitly not the answer.** The instability is schema-induced, corroborated across five
  2026 frontier models with Claude-Sonnet-4 measured at background temperature 0.

### 4.2 Run the scorers

**Precondition — confirm the three method decisions first.** HANDOFF's "What Ryan does next" opens with this, and
the previous revision of this runbook skipped it entirely. `spikes/DECISIONS-2026-09-09.md` leads with a table
titled *"Decisions Ryan should reverse or confirm first"*, and three of its rows change what the scorers below
**measure**:

| # | What it changes | Why it must be settled before you score |
| --- | --- | --- |
| **15** | `human_authored` remapped from a commit-authorship heuristic to the `artifacts.toml` registry | It **changes what METHOD-P §2.3's advisory fraction measures**. The old heuristic yields 0 advisory cases; the new one yields 6. The file's own note says *"confirm the mapping before scoring"* |
| **18** | `_intake-brief.md` and `discover.md` added to both P2 scripts' artifact lists; `_ledger.md` deliberately not | Reconstruct records go 72 → 77 and the advisory candidates become reachable at all |
| **27** | §1.4's anchor-rewrite re-run was taken as arm `L2` | If you would rather the axes went straight to human-supplied, **v1 stands alone as the pre-registered reading and `L2` can be discarded without touching it** — which is a different clause-A record set |

Confirm or reverse each in writing before the scorers run. A confirmation after the numbers are read is the same
defect §4.1 exists to prevent, one level down.

```bash
cd D:/repos/redkiln-rs
# Stay in redkiln-rs. Step 2 runs AFTER this step (see its ORDER banner), so spikes/results/ is still here
# and spikes/p1/, spikes/p2/ never move — only corpus/, results/ and runner/ go to redkiln-corpus.
# If you have taken Step 2 out of order, the scripts are still at spikes/p1/ and spikes/p2/ in THIS repo and
# only the --engine/--labels/--cases/--out paths move to D:/repos/redkiln-corpus/results/.

python spikes/p1/score-arm-d.py \
    --engine spikes/results/p1-arm-d.jsonl \
    --labels spikes/results/p1-arm-d-labels-ryan.md
# prints, per source (docs | commits | union) per facet:
#   precision=… recall=… tp=… fp=… fn=… unlabelled=…   LEAVES THE MODEL (>=0.95) | stays LLM-emitted

python spikes/p2/reconstruct-derived-from.py \
    --redkiln D:/repos/redkiln \
    --sha 47faec9f46b8d983c2c75f0794164e68415ae706 \
    --items spikes/plans/p2-items.txt \
    --phase engine \
    --cases spikes/results/p2-cases.jsonl \
    --labels spikes/results/p2-labels-ryan.md \
    --out   spikes/results/p2-engine.jsonl
```

Pass `--cases` **explicitly**. The engine phase used to derive it from `--out` by string replacement, which
mangled any path containing `engine` and — if `--out` lacked the substring — re-read the file it was about to
truncate, silently producing an empty result (DECISIONS-2026-09-09 #17). The script now refuses when `--cases`
resolves to `--out`, but pass it anyway.

Re-read the unlabelled half of arm L for clause A:

```bash
python spikes/p1/analyse-arm-l.py --records spikes/results/p1-arm-l.jsonl
python spikes/p1/analyse-arm-l.py --records spikes/results/p1-arm-l-v2.jsonl \
                                  --compare-to spikes/results/p1-arm-l.jsonl
```

**`[WILL EXIST]`** — there is **no arm-L panel scorer**. `analyse-arm-l.py` computes the unlabelled half only (its
arguments are `--records`, `--out`, `--compare-to`), and `score-arm-d.py`'s facet regex admits only the seven
derived facets. Scoring `p1-arm-l-labels-ryan.md` against the LLM-emitted ordinals needs a script that does not
exist:

```bash
# [WILL EXIST]
python spikes/p1/score-arm-l.py \
    --engine spikes/results/p1-arm-l.jsonl \
    --labels spikes/results/p1-arm-l-labels-ryan.md
```

Its absence does **not** block the decision. Clause A resolves from the modal-`R` figures already computed, and
F36's own conclusion is that modal-`R` "fires 4 times out of 4, needs no inter-rater statistic, and is what the
engine actually feels. Act on that one." Write the arm-L panel file, stamp it, and hold it for the re-run — it is
the ground truth that re-run will be scored against.

### 4.3 The refusal behaviour — and do not work around it

Both scorers refuse on an unfilled or post-dated label file. The exact messages:

| Message | Cause |
| --- | --- |
| `<path>: labelled_at missing — labels must be timestamped before unblinding` | `FILL-ME` still present, or no `labelled_at:` line |
| `<path>: labelled_at <ts> is not before engine first output <first> — METHOD-P §0 rule 1` | the stamp is at or after the engine's earliest record |
| `--labels required for the engine phase (METHOD-P §0 rule 1)` | P2 engine phase run without `--labels` |
| `labels are not timestamped — do not run the engine` | P2 label file still carries `FILL` |
| `--cases resolves to --out; pass --cases explicitly (#17)` | the #17 guard firing |

**These refusals are the instrument.** Do not back-date `labelled_at`. Do not edit the `at` field in
`p1-arm-d.jsonl`. Do not re-run the engine to move `first` past your stamp. If a refusal fires because you labelled
after opening the output, the honest repair is to say so in the ADR and treat the panel as sighted — a sighted
panel is a weaker result, and a silently unblinded one is no result at all.

Note the guard is deliberately strict on ties: a whole-second label stamp compares *after* a microsecond engine
stamp in the same second, so it refuses. That is fail-closed, and it is correct.

### 4.4 Take the decision and record it as a new ADR

The next free ADR number is **0089** (`references/adr/README.md` §Numbering: "`0074`–`0088` are allocated; the next
free number is `0089`"). Write `references/adr/0089-<slug>.md` carrying all eleven frontmatter keys in order, all
six mandatory sections, and a `**What this forbids**` block inside Consequences.

The supersession is **full** — ADR-0079 decides nothing and forbids being amended, so a successor replaces it
whole. Both sides carry the link; a one-sided link is a defect, not a shorthand:

- `0089` frontmatter: `supersedes: ["0079"]`, `status: accepted`, `date: 2026-09-09`, `legacy_id: none`,
  `consumers: []`.
- `0079` gains `superseded_by: ["0089"]` and its `status` moves `proposed → superseded`. **This is the one edit
  permitted to 0079**, and it is permitted because 0079 is `proposed`, not `accepted` — the immutability rule binds
  accepted records. Change nothing else in its body.

The eleven keys, in order, verified against `references/adr/README.md` §Format: `id`, `title`, `status`, `date`,
`legacy_id`, `source`, `supersedes`, `superseded_by`, `related`, `evidence`, `consumers`. The six sections:
`## Context and problem`, `## Decision drivers`, `## Considered options`, `## Decision`, `## Consequences`,
`## Evidence and confidence` (`Decision` may precede `Considered options`; both orders are valid). `0079`'s
current frontmatter carries `status: proposed` and `superseded_by: []` in exactly the literal form the checkpoint
greps for — verified 2026-09-08 against `0004`, the existing `superseded_by: ["0074"]` precedent.

**These two edits are scripted edits, so they get anchor assertions (golden constraint 7).** Two silent
`str.replace` no-ops were already read as behaviour changes once in this programme:

```bash
cd D:/repos/redkiln-rs
A=$(ls references/adr/0079-*.md)
grep -c '^status: proposed$'      "$A"   # → 1  BEFORE. If 0, stop: you are editing the wrong file or the wrong key.
grep -c '^superseded_by: \[\]$'   "$A"   # → 1  BEFORE.
# …make the two edits…
grep -c '^status: superseded$'          "$A"   # → 1  AFTER
grep -c '^superseded_by: \["0089"\]$'   "$A"   # → 1  AFTER
```

Then update `references/adr/README.md`, which needs **four** changes and each has its own anchor. The exact
strings, quoted from the file as it stands on 2026-09-08:

| # | Anchor now in README.md | Becomes |
| --- | --- | --- |
| 1 | `The set now holds 85 \`accepted\`, 2 \`superseded\` (0004, 0041) and 1 \`proposed\` (0079, deliberately undecided — see **Open items**).` | 86 accepted, 3 superseded (0004, 0041, 0079), **0 proposed** |
| 2 | `` `0074`–`0088` are allocated; the next free number is `0089`. `` | `0074`–`0089` allocated; next free is `0090` |
| 3 | The **Supersessions** table (§ at line 157) | gains the `0089 → 0079` row, full supersession |
| 4 | `` `0079` is the only `proposed` record: Phase 1 pack authoring over the LLM-emitted facets is blocked… `` | rewritten — the block is now discharged by a recorded decision |

Leave the four accepted records that carry named open items (0077/S-GIX, 0078/S-TTL1, 0080, 0088) exactly as they
are; this step settles one row, not the section. Note the README's own Open-items paragraph also asserts *"exactly
three records are not at `accepted`"* — after this step that sentence is still true but names a different three.

**Checkpoint 4.** All seven. This is the gate; treat a failure as a stop, not a note.

```bash
cd D:/repos/redkiln-rs
python spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl \
       --labels spikes/results/p1-arm-d-labels-ryan.md ; echo "exit=$?"   # → exit=0, per-facet table printed
test -s spikes/results/p2-engine.jsonl && echo "p2 engine produced output"
ls references/adr/0089-*.md | wc -l                                # → 1  (exactly one file)
grep -c '^superseded_by: \["0089"\]$' references/adr/0079-*.md     # → 1  back-link present
grep -c '^status: superseded$'        references/adr/0079-*.md     # → 1  status moved
grep -c '0079. is the only .proposed. record' references/adr/README.md   # → 0  the stale claim is GONE
grep -c 'the next free number is .0090.'       references/adr/README.md   # → 1  numbering advanced
```

The last two lines replaced `grep -c 'proposed' references/adr/README.md`, which was **not a check at all**: the
word `proposed` appears in the lifecycle line, the status enum table and the format table, so the count is nonzero
whatever you do and the comment beside it ("the Open-items claim must be gone") was an intention wearing a
command's clothes. These two match the specific sentences that must change, and their expected values differ, so
neither can be satisfied by accident.

Plus one that no command can check and that you must not skip: **the decision rule in §4.1 was written down before
the numbers were read.** If it was not, the ADR says so.

### 4.5 What "the gate is discharged" means — and what it does not

Passing Checkpoint 4 does **not** unblock selection over LLM-emitted facets. On the records that already exist,
clause A fails 4 times out of 4, and the decision ADR-0089 records is therefore expected to be *"every ordinal
axis is `source = "human"` for Phase 1"*. The gate is **discharged by a recorded decision**, not cleared by a
passing measurement. Those are different states and the difference is the whole point of Step 4.

Concretely, after Checkpoint 4 the following are still true and are still enforced at Steps 5–8:

```bash
cd D:/repos/redkiln-rs
# The gate token every later step checks. It asserts a DECISION exists, not that the block lifted.
ls references/adr/0089-*.md >/dev/null 2>&1 && echo "GATE DISCHARGED — ADR-0089 exists" \
                                            || echo "GATE OPEN — no selection-path work may start"
```

Run that line at the head of Steps 5, 6, 7a and 7b. It is the cheapest possible check and it is the one thing
between a reader who skimmed the preamble and a `require` row keyed on an LLM-emitted ordinal.

---

## Step 5 — Workspace, CI, and `rk-types` (Phase 1, part 1)

> ### 🚦 GATE CHECK — run this line before the first `cargo new`
> ```bash
> ls D:/repos/redkiln-rs/references/adr/0089-*.md >/dev/null 2>&1 \
>   && echo "GATE DISCHARGED — proceed" || echo "GATE OPEN — go back to Step 4"
> ```
> Phase 1 entry also needs **Step 3c** answered and **Step 2** landed. Nothing in *this* step touches the
> selection path, so a discharged gate is a formality here — which is exactly why it is checked here, where it is
> cheap, rather than first at Step 7b, where it is not.

The workspace and CI are **part of this step, not an afterthought**. There is **no `.github/workflows` directory in
this repository at all** — the single largest of the four measured absences, alongside no `rust-toolchain.toml`, no
`[lints]` table and no integration-test directory. ADR-0083 specifies CI as "a floor that must never again be
absent". It goes in with the first crate.

### 5.1 The workspace skeleton

```bash
cd D:/repos/redkiln-rs
ls crates/                       # exists and is EMPTY today — verified 2026-09-08
cargo new --lib crates/rk-types
```

Root `Cargo.toml` — workspace package fields inherited, lints in **one** place. **Write the root manifest before
you run `cargo build`, and keep the `exclude`:**

```toml
[workspace]
members = ["crates/*"]
resolver = "2"
# spikes/runner/ is its own package with its own Cargo.toml. If Step 2 has landed it is gone and this
# line is inert; if it has NOT, cargo walks up from spikes/runner/, finds this root, sees that
# members = ["crates/*"] does not cover it, and fails with
#   error: current package believes it's in a workspace when it's not
# — which reads as Halt condition 1 (the rig fails to build) from entirely the wrong direction.
exclude = ["spikes/runner"]

[workspace.package]
edition = "2024"

[workspace.lints.rust]
unsafe_code = "deny"

[workspace.lints.clippy]
all = "deny"
# An ENUMERATED pedantic subset, never the group: clippy::correctness is the only
# deny-by-default group, and nursery lints can change or be removed without notice,
# so a group deny across a dozen crates is a CI outage waiting on the next release.
semicolon_if_nothing_returned      = "deny"
explicit_iter_loop                 = "deny"
match_same_arms                    = "deny"
redundant_closure_for_method_calls = "deny"
items_after_statements             = "deny"
nursery = "warn"
```

Every member carries `[lints] workspace = true` **and nothing else**. No `#![deny]` / `#![allow]` in source as a
restatement of policy; a per-crate exception needs a comment naming why.

`rust-toolchain.toml` at the root — an **exact** channel, not `stable`:

```toml
[toolchain]
channel = "1.97.1"
components = ["rustfmt", "clippy"]
```

A pin and a floor answer different questions and are not substitutes. `package.rust-version` is not a substitute
either.

### 5.2 CI, from the first commit

`.github/workflows/ci.yml` runs on every push and PR, on the pinned toolchain, **with a Windows target** (the
`gix-*` advisory cluster is checkout and path handling, several of them Windows-specific, on the platform this is
being built on):

**This is a gate list, not a file.** The four lines below are the four gates ADR-0083 requires; they are not valid
GitHub Actions YAML and pasting them into `ci.yml` will not run. Author the real workflow around them — `on: [push,
pull_request]`, a matrix with a Windows runner, `dtolnay/rust-toolchain` reading `rust-toolchain.toml`, and
`cargo install cargo-audit --locked` in the job that needs it.

```yaml
# four gates, none optional — ADR-0083. A GATE LIST, not a workflow file.
- cargo fmt --all --check
- cargo clippy --workspace --all-targets
- cargo test --workspace
- cargo audit                      # against the committed Cargo.lock, watching gix-* specifically
```

**`cargo-audit` is not installed on this box** (measured 2026-09-08: `cargo audit` → `error: no such command`).
Neither is `cargo-insta`, which §5.3's snapshot workflow needs. Both are Checkpoint 5 lines, so install them
before you reach it, not when the checkpoint fails:

```bash
cargo install cargo-audit --locked
cargo install cargo-insta --locked
```

CI also carries the **structural assertions**, because a rule stated in prose is not a rule:

| Assertion | AC |
| --- | --- |
| `cargo metadata` shows no `rk-kernel` → `rk-adapter-*` edge | **AC-484** |
| no port-bound generic parameter in `rk-kernel`'s API surface | **AC-480** |
| port-inventory lint with a denylist on the git surface | **AC-483** |
| `traits.len() == fakes.len()` in `rk-ports` | **AC-481** |
| grep rule forbidding `unwrap_or(0)` / `unwrap_or_default()` on the usage struct's fields | **AC-362** |
| the golden schema snapshot (§5.3 below) | **AC-129**–**AC-133**, AC-157, AC-158 |
| corpus-residency lint: no frozen tree, fixture or spike JSONL tracked here | **AC-465** — enforceable only after Step 2 |

Two properties §9.1 calls "by construction, not by care" have **no criterion in any of the thirteen area files** —
`anyhow` in exactly one crate family, and the environment read in exactly one place. They are conventions with a
manifest-level shape, not checks. Author a manifest-level assertion for each here, or record in the commit that
they remain conventions. Do not let the phrase do the work.

### 5.3 `rk-types` and the golden schema snapshot

`rk-types` owns ids; the typed artifacts (`Spec`, `ImplReport`, `Findings`, `DocSet`, digests); schema derivation;
`StepError`; `Drift`; `GateBasis`. It may depend on **no async runtime** — no `tokio`, no `async-std`, no
`futures`, no `async-trait`.

**`ChangeSummary` is NOT built in this step.** S-BIN is registered and unrun, and it **gates Phase 1** for this
type specifically: no review evidence of any kind exists for items whose change is not a text diff, so the field
set cannot be fixed. The crate ships every other artifact type. Historical change material is carried as opaque
content with a migration-loss marker, and **no `ChangeSummary` value is synthesised** (AC-475). The five
toolchain-keyed kinds (ADR-0073) are not finalised either.

The closed-output rules are constraints on the **Rust types**, and `schemars` does not enforce them — data-carrying
enum variants derive to `oneOf` by default, matching serde's externally tagged representation, and
`#[serde(tag = "kind")]` is what renders a single discriminated object instead. So the rule gets a test:

```bash
cargo build -p rk-types                      # golden constraint 3 — build before you test
cargo test -p rk-types --  schema_snapshot   # insta golden snapshot over every exported schema
```

The snapshot asserts, on the generated JSON: no `oneOf` anywhere; `additionalProperties: false` at **every** object
level; `required` ordered first; ≤ 24 optional fields and ≤ 16 union fields per type. **Those two caps are this
project's own authoring budget, not a vendor constant** — they trace to an SDK issue's illustrative arithmetic and
the vendor's published limitations list no such cap. Do not cite them as a platform constraint.

A changed snapshot is a schema change and is reviewed as one. The residual ADR-0084 names about itself stands: a
reviewer can rubber-stamp `cargo insta accept`, and the guard against that is convention, not mechanism.

**Checkpoint 5.** All of:

```bash
cd D:/repos/redkiln-rs
cargo build --workspace                                  # builds, on the pinned toolchain
cargo fmt --all --check                                  # exit 0
cargo clippy --workspace --all-targets                   # exit 0, zero warnings
cargo test --workspace                                   # exit 0
cargo audit                                              # exit 0
test -f rust-toolchain.toml && grep -q 'channel = "1.97' rust-toolchain.toml
test -f .github/workflows/ci.yml
grep -L 'workspace = true' crates/*/Cargo.toml           # → empty: every member inherits lints
test -d crates/rk-types/tests/snapshots \
  && { grep -rn 'oneOf' crates/rk-types/tests/snapshots/ | wc -l; }   # → 0, and the dir MUST exist
grep -rn 'ChangeSummary' crates/rk-types/src/ | wc -l    # → 0. Blocked on S-BIN — must not exist yet
```

Two of those lines changed shape. `! grep -rn 'oneOf' <dir>` **passes when the directory does not exist** — grep
exits 2, `!` inverts it, and a checkpoint asserting "no `oneOf` survives into any exported schema" goes green on a
project with no snapshots at all. The `test -d` guard makes a missing snapshot directory the failure it is. The
`ChangeSummary` line is piped to `wc -l` for the same reason, though there the false pass was benign: the property
being asserted is absence, and a missing `src/` is a different failure the build already catches.

`grep -L` without `-r`: the arguments are an explicit glob of files, so `-r` did nothing.

**Acceptance set:** typed-artifact and store criteria **AC-120–AC-159**, of which the schema-design rules are
**AC-129, AC-130, AC-131, AC-132, AC-133, AC-157, AC-158**; the CI-borne structural assertions **AC-362, AC-480,
AC-481, AC-483, AC-484**; `AC-154` is explicitly **not** evaluable (S-BIN), and `AC-121`'s cross-OS determinism
half and `AC-158`'s property ordering are **unverifiable as specified** until CI fixes two OS targets and an
order-preserving schema export — mark them unverified, do not assume them.

---

## Step 6 — `rk-store` and `rk-git`, with S-GIX run FIRST (Phase 1, part 2)

> ### 🚦 GATE CHECK
> ```bash
> ls D:/repos/redkiln-rs/references/adr/0089-*.md >/dev/null 2>&1 \
>   && echo "GATE DISCHARGED — proceed" || echo "GATE OPEN — go back to Step 4"
> ```

### 6.1 Run S-GIX before a line of `rk-git` worktree code is written

Two arms, both **$0 model cost**, both a docs.rs read plus a small test against the exact pin `gix = "=0.87.1"`.
The research pass named the trailer arm **"the highest-value single check in the list"**.

| Arm | Question | Consumer |
| --- | --- | --- |
| **S-GIX/a** | Can a store rooted at the git **common dir** be opened and written correctly from a **linked worktree** — does the `GIT_COMMON_DIR`/`GIT_WORK_TREE` interplay behave, or is gitoxide's `crate-status.md` ("the delicate interplay … isn't implemented") still accurate? | `rk-git` worktree lifecycle |
| **S-GIX/b** | Does the pinned gix expose a first-class trailer-**writing** API, and does a five-line unit test round-trip a trailer **byte-identically** to `git interpret-trailers --in-place`? | `rk-git` trailer write path — **tier-1 state** |

**`[WILL EXIST]`** — as a `#[ignore]`d test in `rk-git` that runs on demand:

```bash
cargo build -p rk-git
cargo test -p rk-git -- --ignored s_gix_a_common_dir_from_linked_worktree
cargo test -p rk-git -- --ignored s_gix_b_trailer_roundtrip_matches_interpret_trailers
```

**Run these before the worktree code, not after.** A **go** on an arm may narrow ADR-0077 and nothing else may, and
the narrowing is written as a **successor record**, never as an edit to 0077 — ADR-0077 is `accepted`, and the
immutability rule binds accepted records absolutely. **This is the second of the four decisions this runbook
takes** (see the header table); it is an ADR, and the next free number after Step 4 is `0090`. A **no-go** costs
nothing, because the hybrid is what ships either way.

### 6.2 Build to the hybrid, whatever S-GIX says

| Direction | Owner | Operations |
| --- | --- | --- |
| **Read** | `gix`, exact pin `= "=0.87.1"` | `rev-parse` (including resolving the common dir AC-178 needs at startup), repository and worktree status, shallow clone by SHA for review checkouts, trailer **parsing** |
| **Write** | a **process-exec wrapper** invoking the `git` executable | `git worktree add` / `git worktree remove`; `git interpret-trailers --in-place` and/or `git commit --trailer` for **every** trailer write |

Four constraints that come with the hybrid and are easy to lose:

1. **All** shelling out to git goes through **one wrapper type inside `rk-git`**. No `std::process::Command` for
   git exists anywhere else in the workspace.
2. Git's stderr is diagnostic payload and is **never parsed for control flow**.
3. The wrapper asserts **at startup, not at first use**, that `git` is on `PATH` at or above a minimum version —
   and **that minimum is not yet established**, so the assertion is incomplete until you establish it. **This is
   the third of the four decisions this runbook takes.** Pick a version, record it in the same successor record
   6.1 produces, do not leave the assertion as a stub. (`git --version` on this box, 2026-09-08: the MSYS git at
   `/mingw64/bin/git`. Whatever you pick, the floor must be met by the CI runners too, not only by this machine.)
4. Two git implementations in one process can disagree — about config resolution and about Windows path handling.
   Anything read by gix and then written by the executable is **re-read after the write, never assumed**.

### 6.3 `rk-store`

One store per repository, at the common dir. Tier-1 / tier-2 split by the test *if the database were deleted, would
`redkiln run` take a different next action?* Trailers on **two** axes — `Story:` alone is insufficient: **20.3% of
slices (15/74) passed through a rejected state** over 360 trailer commits, and sealed-more-than-once was 21.6%, so
a `Slice-Verdict:` axis is required. Markdown is a **rendering**; nothing parses structure out of prose.
`reconcile()` runs at **every** entry — drift is 7% overall and **3.6× more likely** on gates over 1 h, while
45–48% of gates exceed 1 h — and no optimisation may skip it.

**Checkpoint 6.** All of:

```bash
cd D:/repos/redkiln-rs
cargo build --workspace && cargo clippy --workspace --all-targets && cargo test --workspace   # exit 0 each
grep -c 'gix = "=0.87.1"' crates/rk-git/Cargo.toml                       # → 1. EXACT pin, no caret
grep -rn 'std::process::Command' crates/ --include=*.rs \
  | grep -v 'crates/rk-git/src/git_exec.rs' | wc -l                       # → 0: one wrapper, one file
cargo audit                                                               # exit 0; gix-* clean at the pin
ls references/adr/0090-*.md | wc -l                                       # → 1: the S-GIX successor record
grep -cE 'S-GIX/(a|b).*(go|no-go)' references/adr/0090-*.md               # → 2: a verdict for EACH arm
```

The last two lines make "a **written S-GIX result**" checkable instead of aspirational. "We built the hybrid so we
didn't need to run it" is not a result; the spike is what licenses the *narrowing*, and an unrun spike leaves
ADR-0077 exactly where it is. If you record the result in `spikes/` rather than an ADR, point those two lines at
that file instead — but point them somewhere, and run them.

**Acceptance set:** git-as-truth **AC-160–AC-199** (notably **AC-178** common-dir resolution at startup, **AC-184**
`doctor --rebuild` from git alone with its 40-hex corpus-pin assertion, **AC-187** resume); store and typed-artifact
**AC-120–AC-159**; **AC-464** corpus reads through the object store, **AC-465** residency. **AC-013, AC-187,
AC-266, AC-267, AC-271, AC-401 are PROVISIONAL on P8** — multi-step resume is unmeasured; F38 held exactly one step
at n=3, and its 65-min arm, 24-h arm and all three controls are unrun. Build them; do not claim them.

---

## Step 7 — `rk-ports` and the fakes, `rk-kernel`, the adapters, `rk-cli` (Phase 1, part 3)

Strictly in this order. Each sub-step's checkpoint is its own; do not batch them.

### 7a — `rk-ports` and the fakes

Five named ports — event store, harness, tracker, docs site, toolchain — each a **sync** trait with one real
adapter and one fake. **Git is deliberately not a port**; it is a concrete crate the kernel names (AC-483).

Port traits are sync because native `async fn` in trait is **still not dyn-compatible** and `async_fn_in_dyn_trait`
remains nightly-only: "no generics leak into the core" and "dyn-dispatched port" are in direct tension the moment a
port method is async. Every `.await` lives in an adapter. **Cancellation is an explicit port operation on a
handle, never an implicit drop**, implemented as a real process-group kill.

Fake count equals trait count, and the equality is asserted **as a number** (AC-481), because a fake that quietly
stops tracking its trait is the failure this catches.

**Checkpoint 7a.** All four:

```bash
cd D:/repos/redkiln-rs
cargo build -p rk-ports && cargo test -p rk-ports         # exit 0 (build first — golden constraint 3)
grep -rn 'async fn' crates/rk-ports/src/ | wc -l          # → 0
grep -rn 'traits.len() == fakes.len()' crates/rk-ports/ | wc -l   # → ≥ 1: the assertion EXISTS
cargo test -p rk-ports -- port_fake_parity ; echo "exit=$?"        # → exit=0: and it PASSES
```

"the assertion present and passing" was an intention; those last two lines are it as commands. The number matters
(AC-481) because a fake that quietly stops tracking its trait is exactly what this catches, and a `traits.len()`
that is never evaluated catches nothing.

**Acceptance set: AC-480–AC-519** (ports, fakes; AC-239/AC-496/AC-502 are out of scope — S-CODEX is unrun and
Phase 4 is the earliest a second harness ships).

### 7b — `rk-kernel`

> # 🛑 STOP. THIS IS THE STEP STEP 4 EXISTS TO GATE.
>
> **`rk-kernel` is the crate that contains the selection engine.** Everything before this point could be built
> with the gate wide open and no harm done. This step cannot. Run both lines and read both answers:
>
> ```bash
> cd D:/repos/redkiln-rs
> ls references/adr/0089-*.md >/dev/null 2>&1 \
>   && echo "OK: ADR-0089 exists — the context-vector decision was TAKEN" \
>   || echo "STOP: no ADR-0089. Go to Step 4. Do not write selection code."
> grep -c '^status: superseded$' references/adr/0079-*.md    # → 1. ADR-0079 is discharged, not amended.
> ```
>
> **A discharged gate is not an unblocked one.** ADR-0089 is expected to record *human-sourced ordinal axes for
> Phase 1* — clause A fails 4/4 on the records that exist. So the thing you may now build is the engine's
> **semantics**; the thing you still may not build is any `require` row keyed on an LLM-emitted `criticality`,
> `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro`. `kind` is out of scope of the
> block (human input, ADR-0030 rule 4). That is golden constraint 1, ADR-0079, AC-071 and design §10.2's "may not"
> column, all saying the same sentence.
>
> The enforcement is `redkiln lint --facet-stability`, specified below. **Build the lint before the rows it
> polices**, so there is never a window in which a `require` row exists and nothing checks it.

Workflow/activity split, policy, memo, gates, budgets, retry, replay. **`tokio`, `async-std`, `futures` and
`async-trait` are forbidden here** — no dependency edge, direct or transitive, on any `rk-adapter-*` (AC-484), and
no generic parameter bound by a port trait (AC-480).

Four things this crate must get right, each because a measurement says so:

- **The FSM has never been replayed.** F19 proved suspension lossless 174/174 across a real process boundary but
  explicitly left "same next action" unproven. The kernel replay test is the *first* time it is replayed at all.
  That test is Phase-1 exit criterion (2), not a nice-to-have.
- **Determinism ban list fails the build** — `cargo clippy -D warnings` plus a `trybuild` must-fail fixture that
  reintroduces a clock read (AC-001–AC-004).
- **The `--stay`-class error is unrepresentable** — a `trybuild` must-fail fixture, not a runtime assertion.
- **Selection semantics only.** The seven-step pure function, rank, tie-is-an-error, unsatisfiable-is-a-typed-gate,
  deviation-never-edits-`R`, and the lint set all get built (AC-040–AC-047, AC-054–AC-056, AC-061–AC-064). What
  does not: any `require` row keyed on an LLM-emitted ordinal. `redkiln lint --facet-stability` (AC-071) must
  **fail closed** on `source = "llm"` without a scored stability record, and it must check **all five clauses** —
  identity, n, draws, `panel_verdict`, `corpus_sha`. A lint that reads identity and skips `panel_verdict` unblocks
  the entire provisional apparatus silently; the five-clause fixture set is the only defence.

Also here: the **rigor-monotonicity lint**, failing closed. The ladder is *not* monotone in internal quality today —
escalating to `mandatory` with `has_repro` selects `bisect`, which has **no adversary and no policy step**, so
escalating rigor *drops* the quality block. P7-CEIL settles whether `bisect` is a ceiling activity or gains the
missing steps; the lint ships in Phase 1 either way and fails on the violation.

And what may **not** be set: **no quality-shorthand default may be derived from P7.** Its primary criterion is
powerless at n=3 — every Wilson interval overlaps by construction — and the forbidden predicate
`findings.none(evidence in [Reproduced, MutationSurvived])` may not be used as a terminal condition. Each quality
block is authored explicitly, per activity.

**Checkpoint 7b.** All six:

```bash
cd D:/repos/redkiln-rs
cargo build -p rk-kernel && cargo test -p rk-kernel                   # exit 0
cargo test -p rk-kernel -- trybuild ; echo "exit=$?"                  # → exit=0: the must-fail fixtures
                                                                      #   FAIL TO COMPILE, which is the pass
grep -rnE 'tokio|async-trait|async-std|futures' crates/rk-kernel/Cargo.toml | wc -l   # → 0
cargo metadata --format-version 1 --no-deps >/dev/null && \
  cargo tree -p rk-kernel -e normal 2>/dev/null | grep -c 'rk-adapter-'  # → 0 (AC-484)
cargo test -p rk-kernel -- facet_stability_five_clause ; echo "exit=$?" # → exit=0: five clauses, four negatives
grep -rnE 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' \
     crates/ --include=*.toml --include=*.rs | wc -l                   # → 0. Golden constraint 1, as a command.
```

The last line is the standing prohibition made mechanical. It is coarse and it will occasionally flag a comment;
that is the right direction for this particular guard, and golden constraint 9 says to establish its scope rather
than route around it — read each hit, do not silence the grep.

**Acceptance set: AC-001–AC-039** (kernel), **AC-040–AC-079** (selection, minus the P1-blocked rows AC-069,
AC-070, AC-072, AC-076, AC-079), **AC-080–AC-119** (packs and catalog lint).

### 7c — The adapters

One crate per adapter, each owning its own runtime handle, constructed **once at the composition root and never per
call**. `rk-adapter-claude` lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs` from the spike
runner; that async body survives unchanged *inside* the adapter, wrapped by a sync port method that blocks on an
adapter-owned runtime. An adapter must never call `Handle::block_on` from a thread its own runtime is driving, and
the `-p` stream events must be consumed **inside** the adapter and surfaced as a completed value — the kernel
cannot iterate a stream.

Nine patterns transfer from `spikes/runner` **by construction**, each because a specific failure is otherwise
representable. The ones that bite hardest:

- **`spoke()`** — only `StepError::Schema` means the agent produced a judgement; everything else is silence.
  RK-B0013: one HTTP 529 produced a committed `changes-requested` seal whose entire surviving finding was "review
  failed to produce a digest". **Absence is a block, never a pass** (AC-229, AC-230, AC-231).
- **Prompt on stdin, never argv.** A Windows command line caps near 32 KB; the `redkiln-review` card alone is
  6.6 KB. Argv works in testing and fails on a real review.
- **A record is produced even when the step fails.** Cost only becomes visible if failures are priced too.
- **Raw payload retained beside the typed view** — otherwise a malformed digest and a refusal read identically.
- **`Option` over `0`** on usage fields, with **no `Default` impl**, plus the CI grep rule (AC-362). F3 measured
  `ephemeral_1h_*` / `ephemeral_5m_*` **absent**; a `Some(0)` there would have made a TTL claim look verified.
- **Provenance at dispatch time**, including a digest of the child's **effective** config read from its own
  `system/init` — an ignored flag looks identical to an honoured one — and the git snapshot captured at the instant
  of dispatch, because it cannot be retrofitted.
- **`StructuredOutput` in every card whose role demands a schema**, checked at **configuration time**. F20: a
  `tools` allowlist silently disables `--json-schema`, `structured_output` absent, **exit 0**, `subtype: success`.
  Nothing parses JSON out of prose.
- **Isolation:** `--strict-mcp-config`, verified from the child's own `system/init`. A default `claude -p` inherits
  12 MCP servers and ~280 tools; MCP alone was **4,631 tokens = 56%** of the cache write.
- **Subscription only.** `rk-adapter-claude` **refuses construction when `ANTHROPIC_API_KEY` is present** — a
  construction failure, not a warning (AC-488).

Cache and TTL: set **both** `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and** `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` in
the child's environment, and **report no run as 1 h without per-run `ephemeral_*` field evidence** (S-TTL1 is
unrun, and the fields were measured absent). Constructing the *child's* environment explicitly is required and is
the opposite of the workspace's own `set_var` ban — `std::env::set_var` / `remove_var` are forbidden **anywhere in
the workspace, tests included**, because they became `unsafe fn` in edition 2024 and race a concurrent `getenv`
under the default multi-threaded harness.

Card delivery: **static `--system-prompt` only.** The append flags are **absent from argv** (AC-211) — S-H1c is
unrun, and the route may not be enabled as anything but an unmeasured config option.

**Checkpoint 7c.** All six:

```bash
cd D:/repos/redkiln-rs
cargo build -p rk-adapter-claude -p rk-adapter-sqlite -p rk-adapter-node   # exit 0 — build BEFORE any dispatch
cargo test -p rk-adapter-claude -- api_key_refuses_construction ; echo "exit=$?"   # → exit=0 (AC-488)
cargo test -p rk-adapter-claude -- structured_output_required   ; echo "exit=$?"   # → exit=0: a card that
                                                                                   #   omits it is REFUSED
grep -rnE 'set_var|remove_var'      crates/ --include=*.rs | wc -l   # → 0 (edition 2024: these are unsafe fn)
grep -rn  'append-system-prompt'    crates/ --include=*.rs | wc -l   # → 0 (AC-211, S-H1c unrun)
```

plus, and this one no `cargo test` can stand in for: **one real dispatch validated before any set** (golden
constraint 4), in the foreground (golden constraint 2), with the instrument version written into the record
(golden constraint 8). Three E3 defects survived "built and validated" precisely because none of them was
reachable from a unit test.

**Acceptance set: AC-200–AC-239** (harness port and adapter),
**AC-360–AC-399** (event log, cost attribution, throttling), **AC-240–AC-279** (human gate), **AC-280–AC-319**
(review gate), **AC-320–AC-359** (verification gate).

Three of those sets carry standing prohibitions worth restating at the checkpoint: the review gate may **ADD**
findings and may **never CLEAR** an AC (a reproduced regression was approved on 3 of 10 draws; four adjudicated
defects survived fully green suites of 1,890/1,890 and 1,907/1,907); blocking is computed in Rust from re-executed
evidence, never from the model's `pass`; and **no section may claim the review gate is guarded against false
approval** — ADR-0088's confirmation-bias arm is registered and unrun.

### 7d — `rk-cli`, and `redkiln-migrate` v0

`rk-cli` is the **composition root**: a `clap` router, one idempotent `run` entry, plus `answer`, `status`,
`doctor`. The adapter set is resolved **once at startup** and is a run-scoped constant (AC-485); an adapter that is
not compiled in or is misconfigured **refuses the run before any spawn or store write** (AC-486). The process reads
its environment in exactly one place — here, while building `Config` — and configuration reaches the kernel **as a
value**.

`answer` is the **only** gate channel in Phase 1 (ADR-0009); tracker grammar and web view are Phase 3. `run` never
waits: a human gate is a `(token, tree_hash, deadline)` record, a `Gate:` trailer and a **process that exits 0**.
`doctor` is one rule table, all configuration-time, **zero model cost**.

One CLI behaviour has a measured platform cause and must be handled explicitly: **a platform error can return exit
0 with `subtype: success`** (F20). Exit codes are AC-400–AC-439's subject, and
`references/2026-09-08-acceptance-criteria.md` **§12.10** ("Exit codes, and a platform error that returns success",
line 5132) is named for exactly this. Note that section number belongs to the **acceptance-criteria register**, not
to the architecture design — the design has no §12, and the previous revision's bare "§12.10" sent the reader to
the wrong document.

`redkiln-migrate` v0 ships in Phase 1: one commit, one differential report. It is the **only** place an emitter of
an old redkiln file format may exist (AC-474), and **no configuration key, flag or environment variable enables a
dual-write or compatibility-write mode**.

**Checkpoint 7d:**

```bash
cd D:/repos/redkiln-rs
cargo build --workspace
./target/debug/redkiln doctor           # exit 0, zero model cost, one rule table printed
./target/debug/redkiln run --help       # the idempotent entry exists
./target/debug/redkiln answer --help    # the only Phase-1 gate channel
cargo test --workspace                  # trycmd/assert_cmd CLI-surface tests pass
```

```bash
grep -rnE 'unread[_-]?artifact[_-]?window|window_n' crates/rk-cli/src/ | wc -l   # → ≥ 1: N is DECLARED
grep -rn 'dual.write\|compat.write' crates/ --include=*.rs --include=*.toml | wc -l   # → 0 (AC-474)
```

**Acceptance set: AC-400–AC-439** (CLI and `doctor`), **AC-440–AC-479** (`redkiln-migrate` and the clean break),
**AC-485, AC-486, AC-488**. `AC-472`/`AC-426` cannot be evaluated until `doctor`'s unread-artifact window `N` is
**declared** — that is **the fourth and last of the four decisions this runbook takes**. Declare it in the rule
table in this step and record the number, or the criterion has no truth value and SM-139's target cannot be
missed.

---

## Step 8 — The exit criterion

Phase 1 exits on **seven runnable checks**, not on a narrative milestone. All seven, in order:

**(1) Self-hosting — AC-473.** Redkiln-RS runs its own fix lane end to end.

```bash
cd D:/repos/redkiln-rs
./target/debug/redkiln run --lane fix --item <a real defect in this repo>
./target/debug/redkiln explain --run <run-id> --stages
# PASS only when the event log contains EVERY lane stage from intake to a landed commit,
# with NO stage performed by the old redkiln. The claim is REFUSED if any stage is missing.
# The run id is the evidence; a statement is not.
```

**(2) Replay — AC-184 / SM-124, and the kernel replay test.**

```bash
./target/debug/redkiln doctor --rebuild
# reproduces the SAME next action from git alone, after dropping tier-2 entirely.
# This is the FIRST time the FSM is replayed at all — F19 proved suspension lossless
# 174/174 and explicitly left "same next action" unproven. Includes the adversarial-store
# variant and the 40-hex corpus-pin assertion.
```

**(3) Deterministic resume across a real kill, at MORE THAN ONE STEP.**

```bash
# taskkill /T /F the running process mid-graph, then:
./target/debug/redkiln run --resume <run-id>
# every step-output digest before the kill equals the digest after resume.
# F38 held exactly ONE step at n=3; this criterion is the multi-step case, which is unmeasured.
```

**(4) The determinism ban list fails the build.** `cargo clippy --workspace -D warnings` plus the `trybuild`
must-fail fixture reintroducing a clock read (AC-001–AC-004). The fixture **failing to compile** is the pass.

**(5) The `--stay`-class error is unrepresentable** — a `trybuild` must-fail fixture, not a runtime assertion.

**(6) `redkiln lint --facet-stability` (AC-071) passes**, which means one of exactly two things and nothing else:

```bash
# [WILL EXIST] — and so does its argument: there is NO .redkiln/ directory in redkiln-rs today
# (verified 2026-09-08). The facet registry that exists now is spikes/p9/facets.toml, which Step 7b's
# pack authoring derives .redkiln/facets.toml from. Both the binary and the file are Phase-1 output.
./target/debug/redkiln lint --facet-stability .redkiln/facets.toml \
                            --stability-record <path>
# exit 0 only when EITHER zero facets are declared source = "llm",
# OR a stability record satisfies ALL FIVE clauses:
#   modal_required_set_identity >= 0.80  AND  n_items >= 20  AND  draws_per_item >= 10
#   AND panel_verdict == "scored"        AND  corpus_sha == <catalog corpus pin>
```

On today's evidence the first branch is what passes. That is the expected outcome, not a shortfall.

**(7) Per-activity cost is recorded and carries `resolution_limited`** wherever the 0.01 quantisation binds
(SM-109). `utilization` is quantised to 0.01, so per-run cost is **not measurable**; batch-delta is the only honest
report, and a run that reports a per-run figure without the flag has laundered an unmeasurable into a measurement.

**Checkpoint 8.** All seven pass, and the following remain true and unviolated across the whole phase:

- No `require` row keyed on an LLM-emitted `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`,
  `external_dep` or `has_repro`.
- No finalised `ChangeSummary` type; no synthesised `ChangeSummary` value.
- No card append route enabled as anything but an unmeasured config option.
- No quality-shorthand default; no `findings.none(evidence in [Reproduced, MutationSurvived])` as a terminal
  condition.
- No run reported as 1 h TTL without per-run `ephemeral_*` field evidence.
- gix does not own the write path.
- **No document, section or commit message describes this design as final.** S-BIN, S-H1c and P8's unrun arms gate
  that label, and P8 has no result on its own question.

---

## Step 9 — Rollback (clean, total)

Everything above is one branch plus one new repository plus one working-tree edit in the frozen predecessor. Each
reverses independently and completely.

**Do the salvage BEFORE the revert.** Two commits and three files are worth more than the branch:

```bash
cd D:/repos/redkiln-rs
# Step 1's commit carries the design pass, the 88 ADRs and this runbook; Step 4's carries ADR-0089.
git log --oneline main..build/phase-0-to-1
git checkout main
git cherry-pick <the Step 1 commit> <the Step 4 commit>
# Deleting the branch without this rolls back a DECISION along with the code it was taken for.
```

```bash
# 1. The build. Nothing outside crates/, Cargo.toml, rust-toolchain.toml, .github/ and CORPUS_PIN was created.
git -C D:/repos/redkiln-rs checkout main
git -C D:/repos/redkiln-rs branch -D build/phase-0-to-1        # or keep it parked
git -C D:/repos/redkiln-rs clean -nd crates/ .github/          # DRY RUN first: untracked build residue
# then, only if the listing is what you expect:
git -C D:/repos/redkiln-rs clean -fd crates/ .github/
rm -rf D:/repos/redkiln-rs/target                              # workspace build output

# 2. The corpus split. Step 2 does `git rm -r --cached` AND `rm -rf` on the working tree, so the
#    working copies are gone whichever way you go. If the split commit lives only on the deleted
#    branch, `git checkout main` restores all three directories from main's tree — they were tracked.
#    Verify rather than assume:
ls D:/repos/redkiln-rs/spikes/corpus D:/repos/redkiln-rs/spikes/results D:/repos/redkiln-rs/spikes/runner
#    If any is missing, restore it explicitly from main:
git -C D:/repos/redkiln-rs checkout main -- spikes/corpus spikes/results spikes/runner
#    If the split commit landed on main, restore from the split repo instead:
cp -r D:/repos/redkiln-corpus/corpus  D:/repos/redkiln-rs/spikes/corpus
cp -r D:/repos/redkiln-corpus/results D:/repos/redkiln-rs/spikes/results
cp -r D:/repos/redkiln-corpus/runner  D:/repos/redkiln-rs/spikes/runner
#    and undo the two .gitignore edits (Step 1.2's rules and Step 2's results/ rule + paragraph rewrite):
git -C D:/repos/redkiln-rs checkout main -- .gitignore

# 2b. THE EXTRACTED CORPUS REPO — do not delete it until you have taken the three label files back.
#     They are Step 3b's output, they travelled into redkiln-corpus with results/, and NOTHING ELSE
#     HOLDS THEM. A blind panel cannot be regenerated blind: re-labelling after seeing engine output
#     produces a SIGHTED panel, which is a weaker result, and the timestamp guard will refuse it.
cp D:/repos/redkiln-corpus/results/p1-arm-d-labels-ryan.md \
   D:/repos/redkiln-corpus/results/p1-arm-l-labels-ryan.md \
   D:/repos/redkiln-corpus/results/p2-labels-ryan.md   D:/repos/redkiln-rs/spikes/results/
rm -rf D:/repos/redkiln-corpus                                  # or, better, keep it; it is self-contained

# 3. The one edit to the frozen predecessor. It was never committed there.
git -C D:/repos/redkiln checkout -- workflows/forge-fix.js
git -C D:/repos/redkiln rev-parse HEAD    # must still be 47faec9f46b8d983c2c75f0794164e68415ae706
git -C D:/repos/redkiln status --porcelain -- workflows/   # → EMPTY: the wrapper edit is gone
```

**Checkpoint 9.** Rollback is not done until these pass:

```bash
git -C D:/repos/redkiln-rs rev-parse --abbrev-ref HEAD              # → main
git -C D:/repos/redkiln-rs log --oneline -3                         # → the two cherry-picks on top of e3b0bac
ls D:/repos/redkiln-rs/spikes/corpus D:/repos/redkiln-rs/spikes/results \
   D:/repos/redkiln-rs/spikes/runner >/dev/null && echo "spike tree restored"
ls D:/repos/redkiln-rs/spikes/results/p1-arm-d-labels-ryan.md \
   D:/repos/redkiln-rs/spikes/results/p1-arm-l-labels-ryan.md \
   D:/repos/redkiln-rs/spikes/results/p2-labels-ryan.md             # → all three: the panels survived
git -C D:/repos/redkiln rev-parse HEAD                              # → 47faec9f46b8d983c2c75f0794164e68415ae706
git -C D:/repos/redkiln diff --stat -- workflows/forge-fix.js       # → empty
grep -c 'spikes/results/ is deliberately NOT ignored' D:/repos/redkiln-rs/.gitignore   # → 1: paragraph restored
```

**What rollback cannot undo, and must not pretend to.** Four things, named rather than glossed:

1. **Model dispatches are spent.** Any P1/P2/S-GIX/D-FIXCONSUMER records written during the run are **evidence**
   and stay — a results file deleted because the build was rolled back is a measurement destroyed to tidy a
   branch. Keep the JSONL, keep the ADR, record the rollback as its own decision.
2. **`spikes/runner/target-dev` and `target-p8`** were `rm -rf`'d in Step 1.2. They are build output and
   regenerate from `cargo build`; they do not come back and do not need to.
3. **The blind panels are one-shot.** See 2b above. Blindness is destroyed by reading, not by deleting, and no
   revert restores it.
4. **ADR-0079's status.** If Step 4 landed and you keep the cherry-pick, `0079` is `superseded` and `0089` exists.
   That is correct and is not rolled back — the decision was taken on evidence that is still true. If you *do*
   want it reverted, that is a new ADR superseding `0089`, never an edit to either file.

---

## Halt conditions — these apply at EVERY step

Stop, record, and do not continue past any of these. None is discretionary.

1. **The rig fails to build.** `cargo build` in `spikes/runner` (or `cargo build --workspace` from Step 5 on) does
   not exit 0. **Do not run the binary that is already in `target/`** — `cargo test` and `cargo clippy` do not
   rebuild it, and a stale binary is how this programme reached two false conclusions that code was broken.

2. **The instrument version differs from the pin.** As of 2026-09-09 it already does: measured **2.1.266** against
   HANDOFF's **2.1.263**, with P7 having straddled **2.1.265**. **Record it; do not silently continue.** A version
   move mid-run splits a comparison (F20, F37 §7), and the rig's `doctor` printing `version >= floor : true` is a
   *floor* check, not a match. Any set that spans a version boundary is reported per-version or it is not reported.

3. **A platform error returns exit 0 / `subtype: success`.** This is **measured platform behaviour, not a
   theoretical risk** — F20: an agent card's `tools` allowlist silently disables `--json-schema`,
   `structured_output` is absent from the payload, and the process exits 0 reporting success. **Name it when it
   happens.** Any step that treats exit 0 as evidence of a completed dispatch, without checking that the structured
   payload is present, has measured nothing. The same shape is what makes `if (review && review.pass === false)`
   read absence as approval.

4. **A kill leaves a partial that resume does not restore losslessly.** Any step-output digest that differs across
   a `taskkill /T /F` boundary is a halt, not a retry. F38 is lossless 3/3 — at **one** step, inside the ~1 h TTL,
   with no within-experiment baseline because all three controls are unrun. A first divergence is a finding, and it
   is the finding Phase-1 exit criterion (3) exists to look for.

Two more that this tree has already produced and that cost time when unrecognised:

5. **A waiter that has been running longer than the job's timeout ceiling.** One `until grep` waiter spun 25
   minutes on a job that had already failed. If a wait has no `|exited with code` alternative, it is not a wait, it
   is a hang.

6. **A killed test run leaves a Windows handle no `Stop-Process` releases.** Parallel arms need **separate fixture
   trees**; one worktree per concurrent story is a requirement, not an optimisation. If a fixture tree will not
   delete, that is the handle, and reusing the tree corrupts the next arm.

Two more that this revision added, because the audit that produced it found both live in the previous one:

7. **A guard refuses and you are about to work around it.** The five refusals in §4.3 are the instrument, and
   §4.3's list of what not to do is not exhaustive — the shape is. **Stop at any refusal and establish the
   guard's scope first** (golden constraint 9): is it too narrow, so the failure is passing through elsewhere, or
   too wide, so it is refusing valid work? Back-dating `labelled_at`, editing an `at` field, re-running an engine
   to move `first`, widening a facet regex, or deleting a `--cases` guard are all the same act. If the honest
   answer is that the guard is right and you were wrong, say so in the ADR and take the weaker result.

8. **A checkpoint cannot be run because a path it names does not exist.** That is a halt, not a skip, and it is
   *specifically* a halt when the checkpoint would go green anyway. `!`-negated `grep` over a missing directory
   exits 0. `grep -c` over a missing file exits 2. Both look like a pass in a terminal. **A checkpoint you could
   not run is a failed checkpoint** — record which one and why, and do not advance the step counter.

---

## One-glance checklist

**Execution order: 0 → 1 → 3 → 4 → 2 → 5 → 6 → 7 → 8.** Step 2 is fifth, not third — see its ORDER banner.

- [ ] **preflight** — `cargo install cargo-audit --locked` and `cargo install cargo-insta --locked` (both **MISSING** on this box, measured 2026-09-08); Git Bash open, not PowerShell
- [ ] **0** — branch cut; `rustc`/`cargo` 1.97.1, edition 2024; **`claude --version` written to `spikes/results/instrument-checks/RUNBOOK-instrument.txt`**; rig `doctor` exit 0
- [ ] **1** — CRLF churn checked with `git diff -w --stat`; `target-dev`/`target-p8` removed; logs and `p8-state` gitignored; design pass + 88 ADRs + **`RUNBOOK.md`** + the instrument file committed; **`git status --porcelain` EMPTY** (Phase-0 exit (a))
- [ ] **3a** — `forge-fix.js` line **982** fixed; `=== false` count **0** and `!== true` count **2** (line 1019 was already correct — do not read it as your own fix); D-FIXCONSUMER dispatch run **or** its deferral recorded
- [ ] **3b** — three label files copied to `-ryan.md`, filled **from the item alone**, `labelled_at` stamped **UTC `...Z` by command**, no `FILL` anywhere, **existence guarded before the negation**; `p7-deviation` correctly skipped (no rows); `recent_deploy` understood to be unscored
- [ ] **3c** — Phase-0 exit **(c)** S-BIN re-scoped by ADR **or** recorded as blocking; exit **(d)** E3-SEV returned **or** recorded as deferred. Neither may be silent
- [ ] **4** — DECISIONS #15 / #18 / #27 confirmed or reversed **before** scoring; **decision rule written down BEFORE the numbers were read**; `score-arm-d.py` exit 0; P2 engine phase produced output with `--cases` passed explicitly; refusals respected, nothing back-dated
- [ ] **4** — **ADR-0089 written** (11 keys in order, 6 sections, a `**What this forbids**` block); `0079` moved `proposed → superseded` with a reciprocal `superseded_by`, both anchors asserted; README tally → 86/3/0, numbering → `0090`, Supersessions row added, Open items rewritten. **The gate is now DISCHARGED — which is not the same as unblocked: the expected decision is human-sourced ordinal axes, and the block on LLM-sourced ones stands.**
- [ ] **2** — `redkiln-corpus` created; corpus + results + runner moved **with the three filled panels inside**; **40-hex** SHA in `CORPUS_PIN` matching corpus HEAD; corpus checkout clean; `spikes/{corpus,results,runner}` untracked here; the stale `.gitignore` results paragraph reconciled (AC-464, AC-465)
- [ ] **5** — 🚦 gate check run; workspace root `Cargo.toml` carries `exclude = ["spikes/runner"]`; `rust-toolchain.toml` (exact channel) + `[workspace.lints]` + **CI from the first commit** (the YAML in §5.2 is a **gate list, not a file**), four gates green, Windows target; structural assertions AC-362/480/481/483/484 wired
- [ ] **5** — `rk-types` built; golden schema snapshot passing, **with the snapshot directory's existence asserted** (no `oneOf`, `additionalProperties: false` everywhere); **`ChangeSummary` absent** (S-BIN)
- [ ] **6** — 🚦 gate check run; **S-GIX/a and S-GIX/b run and written up BEFORE the worktree code**, each arm's go/no-go in **ADR-0090** (a successor, never an edit to 0077); the **minimum `git` version chosen and recorded**; hybrid built; exact `gix = "=0.87.1"`; one git-exec wrapper, one file; `cargo audit` clean
- [ ] **6** — `rk-store` on the two trailer axes; `reconcile()` at every entry, no skip path
- [ ] **7a** — 🚦 gate check run; `rk-ports`: five sync traits, `traits.len() == fakes.len()` asserted as a number **and the test seen to pass**, no `async fn`
- [ ] **7b** — 🛑 **the STOP banner read**: ADR-0089 exists and `0079` is `superseded` before a line of selection code; `rk-kernel`: no async runtime, no adapter edge; `trybuild` must-fail fixtures **fail to compile**; **AC-071 lint fails closed on all five clauses**, built **before** the rows it polices; the LLM-ordinal grep returns 0; rigor-monotonicity lint ships; **no quality-shorthand default**
- [ ] **7c** — adapters: `spoke()`, stdin not argv, record-on-failure, `Option` over `0`, `StructuredOutput` checked at configuration time, `--strict-mcp-config` verified from the child's `system/init`, API-key refusal, both TTL vars set, **append flags absent from argv**
- [ ] **7d** — `rk-cli` composition root, adapters resolved once, refusal before any spawn or write; `answer` the only gate channel; `doctor`'s window `N` **declared as a number**; `redkiln-migrate` v0 with no dual-write key
- [ ] **8** — all seven exit checks pass: AC-473 self-hosting run id · `doctor --rebuild` same next action · multi-step resume across a real kill · ban list fails the build · `--stay` unrepresentable · `--facet-stability` exit 0 (on the **zero-`llm`-facets** branch, which is the expected outcome, not a shortfall) · `resolution_limited` recorded
- [ ] **8** — every standing prohibition still unviolated, including: **nothing anywhere calls this design final**
- [ ] **9** — rollback rehearsed at least once on paper; **the three filled panels recovered from `redkiln-corpus` BEFORE anything is deleted**; ADR-0089 and the design-pass commit cherry-picked before any branch delete; **Checkpoint 9 run** — `redkiln` back at `47faec9f…` with a clean `workflows/`, the spike tree restored, the `.gitignore` paragraph back
