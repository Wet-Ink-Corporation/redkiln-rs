# Step 9 — Rollback (clean, total)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s9` | any | a decision to abandon the branch | Checkpoint 9 | — | — |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


Everything above is one branch plus one new repository plus one working-tree edit in the frozen predecessor. Each
reverses independently and completely.

**Do the salvage BEFORE the revert.** Four commits and two label files are worth more than the branch:

```bash
cd D:/repos/redkiln-rs
# Step 1's commit carries the instrument file; Checkpoint 4's commit carries ADR-0090 (PIA-018 — earlier revisions
# named a Step 4 commit that no checkpoint ever made); Checkpoint 4a's carries the registry edit; Checkpoint 2's
# carries CORPUS_PIN and the split; Step 6.1's carries ADR-0091 if it was reached. The design pass, ADR-0001–0089,
# the 2026-09-11 remediation (ADR-0092–0111, ERRATA.md, this runbook) are on main already and are not at risk.
git log --oneline main..build/phase-0-to-1
git checkout main
git cherry-pick <the Step 1 commit> <the Checkpoint 4 commit> <the Checkpoint 4a commit> <the Checkpoint 2 commit> [<the Step 6.1 commit>]
# Deleting the branch without this rolls back the decisions along with the code they were taken for:
# ADR-0090 (the context vector) and, if reached, ADR-0091 (the S-GIX result and git-version floor). Reverting
# ADR-0090 would leave 0079's `superseded_by` a one-sided link, which is a defect — cherry-pick before delete.
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

# 2b. THE EXTRACTED CORPUS REPO — do not delete it until you have taken the label files back.
#     They are Step 3b's output, they travelled into redkiln-corpus with results/, and NOTHING ELSE
#     HOLDS THEM unless Step 2's backup bundle exists. A blind panel cannot be regenerated blind: re-labelling
#     after seeing engine output produces a SIGHTED panel, which is a weaker result, and the timestamp guard
#     will refuse it. Two files travelled (arm D, P2); arm L is outstanding and, if it was labelled after Step 2,
#     exists ONLY in redkiln-corpus (DECISIONS-2026-09-11 #6) — it is restored by this sub-step and nothing else.
cp D:/repos/redkiln-corpus/results/p1-arm-d-labels-ryan.md \
   D:/repos/redkiln-corpus/results/p2-labels-ryan.md   D:/repos/redkiln-rs/spikes/results/
test -f D:/repos/redkiln-corpus/results/p1-arm-l-labels-ryan.md \
  && cp D:/repos/redkiln-corpus/results/p1-arm-l-labels-ryan.md D:/repos/redkiln-rs/spikes/results/ \
  || echo "arm L was never labelled — nothing to restore (expected)"
# The predecessor bundle (predecessor-47faec9f.bundle) is a copy of D:/repos/redkiln's refs; it needs no restore.
rm -rf D:/repos/redkiln-corpus                                  # or, better, keep it; it is self-contained

# 3. The one edit to the frozen predecessor. It was never committed there.
git -C D:/repos/redkiln checkout -- workflows/forge-fix.js
git -C D:/repos/redkiln rev-parse HEAD    # must still be 47faec9f46b8d983c2c75f0794164e68415ae706
git -C D:/repos/redkiln status --porcelain -- workflows/   # → EMPTY: the wrapper edit is gone
```

**Checkpoint 9.** Rollback is not done until these pass:

```bash
test "$(git -C D:/repos/redkiln-rs rev-parse --abbrev-ref HEAD)" = main \
  || { echo "FAIL: HEAD is $(git -C D:/repos/redkiln-rs rev-parse --abbrev-ref HEAD), not main"; exit 1; }   # → main
git -C D:/repos/redkiln-rs log --oneline -6 main                    # → the salvage cherry-picks (Step 1,
                                                                    #   Checkpoint 4, Checkpoint 4a, Checkpoint 2,
                                                                    #   Step 6.1 if reached) on top of main's HEAD —
                                                                    #   the 2026-09-11 remediation lineage, not
                                                                    #   `009a4ec` and not `e3b0bac`; `e3b0bac` is
                                                                    #   the pre-design-pass point (s0 §0.1)
subs=$(git -C D:/repos/redkiln-rs log --format=%s main)             # the eyeball above is not the check; this is
test -n "$subs" || { echo "FAIL: main's subject log is unreadable"; exit 1; }
for s in "Phase 0, part 1: run noise ignored, instrument recorded" "Extract redkiln-corpus at a pinned SHA" \
         "ADR-0090: the context-vector source" "Apply ADR-0089/0090/0093"; do   # verbatim from s1/s2/s4/s4a's own `git commit -m`
  printf '%s\n' "$subs" | grep -qF "$s" \
    || { echo "FAIL: \"$s\" is on no commit on main — cherry-pick it before the branch goes (if its step was never reached, record that first)"; exit 1; }
done
test -n "$(git -C D:/repos/redkiln-rs ls-tree --name-only main references/adr/ | grep '^references/adr/0091-')" \
  && echo "salvaged: ADR-0091 (Step 6.1)" \
  || echo "DEFERRED: ADR-0091 not on main — Step 6.1 was never reached (the salvage block marks it conditional); NOT a pass"
ls D:/repos/redkiln-rs/spikes/corpus D:/repos/redkiln-rs/spikes/results \
   D:/repos/redkiln-rs/spikes/runner >/dev/null && echo "spike tree restored"
ls D:/repos/redkiln-rs/spikes/results/p1-arm-d-labels-ryan.md \
   D:/repos/redkiln-rs/spikes/results/p2-labels-ryan.md             # → both: the panels survived (arm L only if it was ever labelled)
test "$(git -C D:/repos/redkiln rev-parse HEAD)" = 47faec9f46b8d983c2c75f0794164e68415ae706 \
  || { echo "FAIL: predecessor HEAD is not 47faec9f"; exit 1; }
git -C D:/repos/redkiln diff HEAD --exit-code --stat -- workflows/forge-fix.js   # → empty; a staged or unstaged edit now exits 1
test -f D:/repos/redkiln-rs/.gitignore || { echo "FAIL: .gitignore absent"; exit 1; }
test "$(grep -c 'spikes/results/ is deliberately NOT ignored' D:/repos/redkiln-rs/.gitignore)" = 1 \
  || { echo "FAIL: paragraph not restored exactly once"; exit 1; }
```

**What rollback cannot undo, and must not pretend to.** Four things, named rather than glossed:

1. **Model dispatches are spent.** Any P1/P2/S-GIX/D-FIXCONSUMER records written during the run are **evidence**
   and stay — a results file deleted because the build was rolled back is a measurement destroyed to tidy a
   branch. Keep the JSONL, keep the ADR, record the rollback as its own decision.
2. **`spikes/runner/target-dev` and `target-p8`** were `rm -rf`'d in Step 1.2. They are build output and
   regenerate from `cargo build`; they do not come back and do not need to.
3. **The blind panels are one-shot.** See 2b above. Blindness is destroyed by reading, not by deleting, and no
   revert restores it. **P2's is the sharpest case, because its blindness is a *timestamp* and a revert cannot
   move one**: `spikes/results/p2-labels-ryan.md` at `labelled_at 2026-09-10T15:58:08Z` precedes
   `p2-engine.jsonl`'s earliest record at `16:01:12Z`, and re-filling or re-stamping that file after the engine
   has been read makes the ordering false and the arm sighted [measured, F40]. Keep both files; `score-p2.py`'s
   `labels_at` cross-check is what would catch the mismatch, and catching it is not the same as repairing it.
4. **ADR-0079's status, and ADR-0089.** If Step 4 landed and you keep the cherry-pick, `0079` is `superseded` and
   `0090` exists. That is correct and is not rolled back — the decision was taken on evidence that is still true.
   If you *do* want it reverted, that is a new ADR superseding `0090`, never an edit to either file. **ADR-0089 is
   in a stronger position still**: it is `accepted`, it landed before this branch, and it is not this build's to
   revert. Reverting it would also mean reverting ADR-0030's `superseded_by` back-link, and a one-sided
   supersession link is a defect [documented, README §Supersessions]. If you want it gone, that is a new record
   superseding `0089` — and it would have to answer `auth` at 0.00 over 11 positives [measured, F39].

---
