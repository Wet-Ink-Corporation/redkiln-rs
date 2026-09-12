# Step 2 — Extract `redkiln-corpus` as its own repo, pinned by SHA (Phase 0, part 2)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s2` | Phase 0 | Checkpoint 4a (Step 2 runs SIXTH — see its ORDER banner) | Checkpoint 2 | `s4a` | `s5` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


> ## ⚠ ORDER: RUN THIS **AFTER** STEP 3, STEP 4 AND STEP 4a, NOT BEFORE THEM.
>
> **The canonical execution order in this file is 0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7 → 8.** Step 2 keeps this
> position in the document because it is Phase 0 part 2, but it is executed **sixth**.
>
> **Step 4a is inside the same constraint.** It edits `spikes/p9/facets.toml`, which Step 2 does **not** move —
> only `corpus/`, `results/` and `runner/` go to `redkiln-corpus` — but AC-520's scored-record clause points at a
> record that lives under `spikes/results/`, so run 4a while the results are still here and let the record travel
> into the corpus repo with everything else.
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
>
> **Arm L is not a precondition (DECISIONS-2026-09-11 #6).** Step 3b's outstanding act — the arm-L blind label
> file — has no scorer yet; Checkpoint 2 accepts it as outstanding, and it is labelled later in
> `redkiln-corpus/results/` and committed at a new pin. Two label files travel here, not three.

ADR-0013: the corpus becomes a separate `redkiln-corpus` repository — **pinned SHA, fixtures, JSONL and the spike
runner all live there**, and everything else depends on it by SHA. Today **`redkiln-corpus` does not exist**, and
`spikes/corpus/`, `spikes/results/*.jsonl` and `spikes/runner/` are all tracked in this repo. **AC-465 is violated
by construction today** and the corpus-residency lint cannot be switched on until this step runs.

History preservation is not required by ADR-0013 — what is required is a pinnable SHA. Take the plain route.

**But the substrate the Phase-1 tests read must be pinnable too (PIA-002, ADR-0100 / decision D-N).** Two things
are called "the corpus": `spikes/corpus/` holds only `PINNED_SHA`; the 1,312-file tree `redkiln-migrate` reads and
the 360-commit history AC-444 replays live in the predecessor clone `D:/repos/redkiln`, named in CORPUS_PIN by a
machine-local path. F12's 74 slices need **118 refs** (42 heads, 41 remote-tracking, 35 tags [measured, `git
for-each-ref | wc -l`, 2026-09-11]); from `47faec9f` alone only 46 of 74 slices and 231/348 trailer commits are
reachable. So the predecessor is **bundled with every ref** into redkiln-corpus, and the corpus-bound tests (AC-440,
444, 451, 452, 459, 460, 461, 469, 479; AC-165/184; AC-006/188's golden logs once Step 5 records them) run as an
explicit **local-only suite** behind a `corpus` cargo feature with a named runner — CI runs the rest (s5 §5.2).

```bash
mkdir -p D:/repos/redkiln-corpus && cd D:/repos/redkiln-corpus && git init

# The unit ADR-0013 names, and nothing else.
cp -r D:/repos/redkiln-rs/spikes/corpus   ./corpus
cp -r D:/repos/redkiln-rs/spikes/results  ./results
rm -rf ./results/p8-state                                       # suspended graph state Step 1.2 ignored; never corpus (PIA-021)
cp -r D:/repos/redkiln-rs/spikes/runner   ./runner
rm -rf ./runner/target ./runner/target-dev ./runner/target-p8   # build output, never corpus
cp -r D:/repos/redkiln-rs/spikes/schemas ./schemas              # p1-classify.schema.json — a runner input
cp -r D:/repos/redkiln-rs/spikes/prompts ./prompts              # the cards the records were produced under
# schemas/ and prompts/ are COPIED, not moved: the records cannot be replayed without the card and
# schema they were produced under, and METHOD-P/FINDINGS in this repo still cite them by path.
# Only corpus/, results/ and runner/ are removed from redkiln-rs below — that is ADR-0013's unit.
# spikes/fixtures/ (2.4 GB, e1/e3 fixture trees) is deliberately LOCAL and unversioned: the runner's
# build_fixture_at regenerates every one from the pinned predecessor. It is not copied and not moved (PIA-022).

# The predecessor, every ref, as one pinnable object (PIA-002): the F12 substrate travels with the corpus.
git -C D:/repos/redkiln bundle create D:/repos/redkiln-corpus/predecessor-47faec9f.bundle --all
git bundle verify ./predecessor-47faec9f.bundle                  # → "is okay"
git bundle list-heads ./predecessor-47faec9f.bundle | wc -l      # → 118 (re-derive: git -C D:/repos/redkiln for-each-ref | wc -l)
git bundle list-heads ./predecessor-47faec9f.bundle | wc -l > ./predecessor-refs.count

git add -A && git commit -m "redkiln-corpus v0: pinned corpus, spike results, spike runner, the predecessor bundled with all 118 refs (ADR-0013, ADR-0100)"
git rev-parse HEAD    # ← this is the pin. Record it.
```

**Before the `rm -rf` below, the corpus repo must exist somewhere other than this disk** (PIA-021: after the split,
the two filled blind panels exist in one copy). Push it to a remote, or at minimum `git -C D:/repos/redkiln-corpus
bundle create D:/backup/redkiln-corpus-$(date -u +%F).bundle --all` to a second volume, and record where.

Then record the pin in this repo and delete the originals:

```bash
cd D:/repos/redkiln-rs
cat > CORPUS_PIN <<'EOF'
# redkiln-corpus pin (ADR-0013). Every corpus read resolves through this SHA via the
# object store, never through a working tree (AC-464). A run against an unpinned
# reference, or against a corpus checkout with uncommitted changes, is REFUSED.
CORPUS_REPO=D:/repos/redkiln-corpus
CORPUS_SHA=<paste the rev-parse output above>
PINNED_AT=<paste the output of: date -u +%F>

# The frozen predecessor corpus this repo pins in turn, unchanged — and its bundle inside the corpus repo,
# which is what every corpus-bound test reads (never the machine-local clone; ADR-0100 / PIA-002):
REDKILN_REPO=D:/repos/redkiln
REDKILN_SHA=47faec9f46b8d983c2c75f0794164e68415ae706
PREDECESSOR_BUNDLE=predecessor-47faec9f.bundle
PREDECESSOR_REFS=118
EOF
# The placeholders are filled by hand on purpose; a heredoc that expands $(date) would also expand the SHA
# line's `<`, so this file is written, then checked by Checkpoint 2, never templated (golden constraint 7).
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

**Checkpoint 2.** All eleven checks — recounted on disk 2026-09-11; the block said "ten" and carries eleven, and
several now carry a one-line precondition guard above them, so the block runs longer than eleven lines:

```bash
git -C D:/repos/redkiln-corpus rev-parse HEAD                  # → a 40-hex SHA
grep CORPUS_SHA D:/repos/redkiln-rs/CORPUS_PIN | grep -qE '=[0-9a-f]{40}$' && echo "40-hex, unabbreviated"
h=$(git -C D:/repos/redkiln-corpus rev-parse HEAD) && test -n "$h" || { echo "FAIL: corpus HEAD unreadable"; exit 1; }
grep -q "$h" D:/repos/redkiln-rs/CORPUS_PIN && echo "pin matches HEAD" || { echo "FAIL: CORPUS_PIN missing, or it does not carry HEAD"; exit 1; }
out=$(git -C D:/repos/redkiln-corpus status --porcelain) || { echo "FAIL: corpus repo unreadable"; exit 1; }
test -z "$out" && echo "corpus clean" || { echo "FAIL: corpus dirty (AC-464)"; printf '%s\n' "$out"; exit 1; }   # → "corpus clean"
git -C D:/repos/redkiln-rs rev-parse --git-dir >/dev/null || { echo "FAIL: redkiln-rs is not a repo"; exit 1; }
n=$(git -C D:/repos/redkiln-rs ls-files | grep -E '^spikes/(corpus|results|runner)/' | wc -l)   # wc -l, not grep -c — see below
test "$n" -eq 0 && echo "no corpus paths tracked here" || { echo "FAIL: $n corpus paths still tracked here (AC-465)"; exit 1; }   # → 0
ls D:/repos/redkiln-corpus/results/p1-arm-d-labels-ryan.md \
   D:/repos/redkiln-corpus/results/p2-labels-ryan.md         # → both; Step 3b's work travelled
test -d D:/repos/redkiln-corpus/results || { echo "FAIL: corpus results/ absent — the arm-L check is not runnable"; exit 1; }
test -f D:/repos/redkiln-corpus/results/p1-arm-l-labels-ryan.md && echo "arm L travelled" || echo "DEFERRED: arm L outstanding — expected (DECISIONS-2026-09-11 #6)"
test -d D:/repos/redkiln-corpus/results || { echo "FAIL: corpus results/ absent — the p8-state exclusion is unverifiable"; exit 1; }
test -d D:/repos/redkiln-corpus/results/p8-state && { echo "FAIL: p8-state travelled (PIA-021)"; exit 1; } || echo "p8-state excluded"
git -C D:/repos/redkiln-corpus bundle verify predecessor-47faec9f.bundle >/dev/null && echo "predecessor bundle verifies"
c=$(cat D:/repos/redkiln-corpus/predecessor-refs.count); p=$(grep '^PREDECESSOR_REFS=' D:/repos/redkiln-rs/CORPUS_PIN | cut -d= -f2)
test -n "$c" && test "$c" = "$p" && echo "ref count pinned" || { echo "FAIL: refs.count='$c' CORPUS_PIN='$p'"; exit 1; }
out=$(git -C D:/repos/redkiln-rs status --porcelain) || { echo "FAIL: redkiln-rs unreadable"; exit 1; }
test -z "$out" && echo "redkiln-rs clean" || { echo "FAIL: redkiln-rs dirty — PIA-018 wants the split committed"; printf '%s\n' "$out"; exit 1; }   # → "redkiln-rs clean"
```

The label-file lines are the check that the previous ordering would have failed silently: the filled panels are
the most expensive artifact in Phase 0 and the only one no re-run can reproduce blind. The bundle lines are what
makes F12's 74 slices and AC-444's 348 trailer commits reachable from something pinned rather than from a clone on
one disk.

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
