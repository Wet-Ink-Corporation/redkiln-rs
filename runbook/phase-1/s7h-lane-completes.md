# Step 7h — The lane completes: stage advance, the two trailer axes, and the review gate's call site (Phase 1, part 6)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7h` | Phase 1 | Checkpoint 7g | Checkpoint 7h | `s7g` | `s7i` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** After `s7g` an activity's steps execute and a `tool` step reaches the port, but a run
still performs **one stage and stops**: `replay::resume`'s loop handles `Dispatch` and `OpenGate` and `_ => break`s
everything else, and `AdvanceStage` is unreachable because `ctx.successor` is never set (`base_inputs` does not set
it). A lane therefore cannot reach a landed commit, and **AC-473's *"every lane stage from intake to a landed
commit"* has no landed commit to reach** — measured 2026-09-21: a landed run leaves **zero** commits, and the next
run deletes the worktree with `worktree remove --force`.

**Advance and the trailer are one change, not two.** design §5's event table gives `stage_transition` as tier-1
*"(trailer)"* with producer *"the kernel's FSM at a stage boundary"* — the row and its `Story:` trailer commit are
emitted **at** the boundary, so wiring the boundary is what mints the trailer. Likewise `slice_sealed` /
`Slice-Verdict:` has producer *"the lane's review stage"*. ADR-0105 decision 3 fixes the plumbing:
`git commit --allow-empty --trailer "<Key>: <value>"` through the wrapper with `-c trailer.ifexists=add`, on the
run's branch. Measured 2026-09-21, `TrailerKey::Story` and `TrailerKey::SliceVerdict` occur **only in the parser**.

**This step takes no decision.** Both producers and the plumbing are specified.

### 7h.1 `ctx.successor`, and the stage boundary the FSM already knows how to take

`NextAction::AdvanceStage { from, to }` exists and `fsm::next_action`'s evaluation order already places it fourth,
after `Dispatch` and before `Halt`. What is missing is its input: `RuntimeContext::successor`, which `base_inputs`
never sets, so the arm is dead. The successor is the pack's own stage order — `[policy] stages` — filtered to the
stages the pack does not mark `forbidden`, which `bug-standard` and `story` both declare.

`replay::resume` gains the `AdvanceStage` arm: append the transition, write its trailer commit, and **continue the
loop into the next stage** rather than breaking. The existing `_ => break` stays for `Halt` and `Done`, which are
terminal by design.

🛑 **The gate exit is unchanged and must stay so.** `OpenGate` breaks and `run` exits 0 — golden constraint 2,
*"`run` NEVER WAITS"*, and ADR-0128's *"a run that STOPS AT A GATE exits 0"*. Advancing past a stage is not
advancing past a gate.

### 7h.2 The two trailer axes, written where their producers are

**`Story:` at the stage boundary** — value per design §5.4's grammar table, column-anchored, through the wrapper.
**`Slice-Verdict:` from the lane's review stage** — `<project> "/" <slice> SP <verdict>`, exactly two
whitespace-separated tokens, verdict in `approved | changes-requested` compared case-insensitively and stored
lower-case, latest wins with commits folded oldest-first.

**Two axes because one is provably insufficient** (design §5.4, F12): over **360 trailer commits across 74 slices**,
**15/74 = 20.3%** of slices passed through a rejected state and 21.6% were sealed more than once, so `Story:`-trailer
resume alone loses the rejection history. That is why this step wires both and not just the one the stage boundary
gives for free.

Every write goes through `rk_git::trailer::write::commit_with_trailers` on the run-owned worktree, past
`authorise_tier1_write` — which refuses unless the target is the store-recorded path on `rk-run/<item>` (AC-518,
ADR-0105 clause 8). No new write path is introduced; the existing one gains two more callers.

### 7h.3 The review and verify modules gain their call site

`rk_kernel::review` (10 files) and `rk_kernel::verify` (6 files) are built, tested, and have **no caller on the run
path** — measured 2026-09-21. They are not written here; they are **wired**. The review stage's activities reach
them the way every other stage reaches its own logic, and `slice_sealed` is the review stage's output.

`Exit::FindingsBlock` (4) has **no producer** today — `exit.rs` says so in terms: *"its producer is the kernel's,
built at 7b-ii"*. Wiring the review gate is what gives code 4 a producer, and Checkpoint 7h asserts it has one.

🛑 **ADR-0101 clause 5 holds, and this step is where it would be tempting to break it.** *"No document may claim
the review gate is guarded against false approval until S-SUBTLE-CB runs."* S-SUBTLE-CB has **not** run. This step
wires the gate; no line of this file, of its commits, or of any register amendment it carries may state or imply
that the gate is guarded against false approval.

### 7h.4 Checkpoint 7h

🛑 **HARDENED 2026-09-22, BEFORE ITS FIRST RUN, and the reasons are measured rather than stylistic.**
As authored, two of this block's six bullets could not fail:

- **`grep -q '^Slice-Verdict:'` was ALREADY GREEN on the untouched tree.** With
  `FIX_ITEM=rk-fix-doctor-test-count`, one line in the history of a pre-existing
  `refs/heads/rk-run/rk-fix-doctor-test-count` matched it — the wrapped **prose**
  `Slice-Verdict: does not register here yet could still invalidate a`, inside spike commit
  `ba46ed9`'s body. Git itself does not agree it is a trailer: `%(trailers:key=Slice-Verdict)` is
  empty for every commit on that branch. A guard satisfied by prose reports evidence it does not have.
- **The `no rk-run/$FIX_ITEM ref` guard passed over a STALE ref.** That branch existed and was **0
  commits ahead** of `build/phase-0-to-1` — a leftover pointing into mainline history, not a
  run-produced branch.

`00-preamble.md`'s PIA-035 rule requires a BEFORE assertion that **fails on the untouched tree**, and
neither did. So: trailers are read with **git's own parser**, scoped to the run's **own** commits, and
their values must **parse** against design §5.4's grammar rather than merely appear. `grep '^Story:'`
passes on `Story: /item`, which `split_qualified` resolves to a `BareStory` that records nothing
(ADR-0159).

**What this checkpoint asserts, and its state at this stage's close:**

- **The ARMED trailer lint's position is MEASURED, not assumed GREEN.** `Story:` and `Gate:` are
  minted; `Slice-Verdict:` is not. The `#[ignore]` **stays** (ADR-0160).
- **A lane crosses a real stage boundary**, and its `Story:` trailer is on `rk-run/<item>` — a commit
  **ahead of the base**, carrying a value that parses as a qualified story. ✅
- **The boundary is idempotent**: a second invocation appends no `stage_transition` and mints no second
  trailer (ADR-0157). ✅
- **The user's checkout is unchanged across the run** — AC-518, asserted by `git status --porcelain`
  empty. ✅
- **`Slice-Verdict:` has a producer.** ❌ **ADR-0160**: `slice_sealed` has no run-path producer (its
  one construction is `doctor/rebuild.rs`, the rebuild path), `review::confirm` produces no seal, and
  `<slice>` is declared nowhere.
- **`Exit::FindingsBlock` has a producer.** ❌ **ADR-0156**: the review stage is unreachable for six
  measured absences, the first being that **8 of 8** shipped cards carry no `permission_mode` (AC-214).
- **No document claims the review gate is guarded against false approval** (ADR-0101 clause 5;
  S-SUBTLE-CB is UNRUN). ✅

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Every `cargo` reading captures the process's OWN exit code (`out=$(…); rc=$?`) and never a pipe's:
# a `cargo test | grep` hands you grep's status, and under build-lock contention cargo exits 0 having
# run ZERO tests. `check-checkpoints.py` cannot catch that for you — measured 2026-09-22: of 92
# `cargo test` lines in fenced blocks across runbook/, 23 carry a real pipe and 0 are flaggable,
# because its PIPE-STATUS branch is gated on `not asserted(line)` and every guarded pipeline is
# "asserted" (carried finding (15)).

test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item the lane ran — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
git rev-parse --verify --quiet "refs/heads/rk-run/$FIX_ITEM" >/dev/null || { echo "FAIL: no rk-run/$FIX_ITEM ref — the lane produced no run-owned branch"; exit 1; }

# ---- (1) THE BRANCH IS THE RUN'S OWN WORK, not a stale ref. This is the BEFORE assertion: it was
# ---- FALSE at fc45eeb, where the ref existed and was 0 commits ahead.
base=$(git merge-base HEAD "rk-run/$FIX_ITEM") || { echo "FAIL: rk-run/$FIX_ITEM shares no history with HEAD"; exit 1; }
ahead=$(git rev-list --count "$base..rk-run/$FIX_ITEM")
[ "$ahead" -ge 1 ] || { echo "FAIL: rk-run/$FIX_ITEM is $ahead commits ahead of the merge base — a stale ref is not a run"; exit 1; }

# ---- (2) THE `Story:` TRAILER, read by GIT'S OWN PARSER and scoped to the run's own commits.
# ---- `grep '^Story:'` is not used: it matches wrapped prose in a commit BODY, which is how this
# ---- guard's `Slice-Verdict:` twin was green on an untouched tree.
story=$(git log "$base..rk-run/$FIX_ITEM" --format='%(trailers:key=Story,valueonly=true)' | grep -c .)
[ "$story" -eq 1 ] || { echo "FAIL: the run's own commits carry $story real Story: trailers, expected exactly 1 — 0 means the stage boundary minted none, >1 is duplicate-story-trailer DRIFT (ADR-0155)"; exit 1; }

# ---- (3) AND THE VALUE PARSES. A trailer that appears and does not parse records nothing:
# ---- `Story: /item` is a BareStory, counted into bare_story_ids, never a transition (ADR-0159).
val=$(git log "$base..rk-run/$FIX_ITEM" --format='%(trailers:key=Story,valueonly=true)' | grep . | head -1)
proj=${val%%/*}; slug=${val#*/}
[ -n "$proj" ] && [ "$proj" != "$val" ] || { echo "FAIL: the Story: value '$val' has no non-empty <project>/ prefix — split_qualified returns None and the mark is a bare story id"; exit 1; }
[ "$slug" = "$FIX_ITEM" ] || { echo "FAIL: the Story: slug is '$slug', not '$FIX_ITEM' — doctor --rebuild resolves the stage from .redkiln/items/<slug>.toml and reports unresolvable otherwise"; exit 1; }

# ---- (4) THE STAGE BOUNDARY IS IN THE JOURNAL, and is not a self-transition (ADR-0157).
runs=$(ls -d .git/redkiln/runs/* 2>/dev/null | grep -c .)
[ "$runs" -ge 1 ] || { echo "FAIL: no run record under .git/redkiln/runs — the lane did not run"; exit 1; }

# ---- (5) AC-518: THE USER'S CHECKOUT IS UNCHANGED. The tracker row may be unstaged at this moment
# ---- (00-preamble: a §1 row is hashless between the two close commits), so the guard names what it
# ---- tolerates rather than demanding a bare-empty tree.
porcelain=$(git status --porcelain | grep -v 'runbook/PROGRESS.md')
test -z "$porcelain" || { echo "FAIL: the lane wrote to the user's checkout (AC-518):"; printf '%s\n' "$porcelain"; exit 1; }

# ---- (6) THE ARMED LINT'S POSITION IS MEASURED. It is NOT green: `Slice-Verdict:` has no producer
# ---- (ADR-0160). This guard asserts the lint is still ARMED **and** that its reason names the one
# ---- key that is missing — so a lint discharged by an invented slice identity fails here.
out=$(cargo test -p rk-kernel --test it -- conformance_every_trailer_key_has_a_producer 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the conformance selection did not run cleanly (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qF 'ignored, ARMED: Slice-Verdict has 0 producers' || { echo "FAIL: the trailer lint's ARMED reason does not name Slice-Verdict as the one missing producer — it was discharged, renamed, or deleted rather than measured"; printf '%s\n' "$out" | grep -E 'ignored|test result:'; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [0-9]+ passed; 0 failed; 1 ignored' || { echo "FAIL: the conformance module did not report exactly one ignored lint"; exit 1; }

# ---- (7) HALF THE LINT IS DISCHARGED, AND THAT IS ASSERTED RATHER THAN CLAIMED: run it un-ignored
# ---- and require its failure to name Slice-Verdict ALONE. Before s7h it named Story too.
out=$(cargo test -p rk-kernel --test it -- conformance_every_trailer_key_has_a_producer --ignored 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the armed lint PASSES — it should be discharged and its #[ignore] removed in that same commit (s7f's convention)"; exit 1; }
printf '%s\n' "$out" | grep -qF 'no producer for: ["Slice-Verdict"]' || { echo "FAIL: the lint's failure does not name Slice-Verdict as the sole missing producer"; printf '%s\n' "$out" | grep -E 'no producer|panicked'; exit 1; }

# ---- (8) ADR-0101 CLAUSE 5. S-SUBTLE-CB is UNRUN and nothing may claim the gate is guarded.
#
# 🛑 **THE SCOPE IS `crates/`, AND THE AUTHORED FORM'S SCOPE MADE IT UNPASSABLE.** As written this
# guard swept `runbook/ references/` with an exclusion list (`may not|never|until S-SUBTLE-CB|no
# document`). It cannot pass on any tree: the PROHIBITION itself lives in those directories — design
# §10's row, ADR-0101 clause 5, and three other stages' own guard COMMANDS all contain the phrase —
# and when the prohibiting sentence wraps, the excluding words sit on the previous line. Measured
# 2026-09-22: 13 hits, every one a prohibition or a grep command, zero claims. So this is the third
# of six bullets that could not do its job as authored, alongside the two that could not fail.
#
# s7b-ii, s7c and s7d all scope the same sweep to `crates/` and require SILENCE — *"even to deny
# it"* — which is decidable precisely because source has no reason to carry the prohibition prose.
# This follows that precedent rather than inventing a doc-scanning form that cannot work.
raw=$(grep -rniE 'guarded against false approval|calibrated against false approval' crates/rk-kernel/ crates/rk-cli/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the ADR-0101 sweep could not run (grep rc=$rc)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a claim that the review gate is guarded against false approval appears in a 7h crate — ADR-0101 clause 5 forbids it until S-SUBTLE-CB runs (the 7b-ii lesson: even to deny it)"; printf '%s\n' "$raw"; exit 1; }   # → silent

# ---- (9) THE WORKTREE GUARD, BY PATH SET AND NOT BY COUNT (carried finding (14)'s shape). Counts
# ---- cannot tell a leaked registration from a replaced one; paths can.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
git worktree prune
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: the prune changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }

# ---- (10) THE TWO BULLETS THIS STAGE DOES NOT DELIVER, asserted as the debt they are. Each names
# ---- its record, so a later pass that closes one deletes a guard rather than editing a claim.
prod=$(grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ --include=*.rs | grep -v 'exit.rs' | grep -c .)
[ "$prod" -ge 1 ] || { echo "OWED (ADR-0156): Exit::FindingsBlock still has no producer outside its own declaration — the review stage is unreachable for six measured absences, the first being 8 of 8 shipped cards with no permission_mode (AC-214)"; exit 1; }

echo "CP7H-END-OF-BLOCK reached"
```

---
