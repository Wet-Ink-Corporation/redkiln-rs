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

- **The third ARMED conformance lint is GREEN**: every trailer key in design §5's event table has a producer.
- **A lane runs intake → landed commit**, and the commit is on `rk-run/<item>`, not on the checkout.
- **Both trailer axes are present** on that run's branch, and both parse against design §5.4's grammar.
- **`Exit::FindingsBlock` has a producer.**
- **The user's checkout is unchanged across the run** — AC-518, asserted by `git status --porcelain` empty and HEAD
  unmoved.
- **No document claims the review gate is guarded against false approval** (ADR-0101 clause 5).

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
out=$(cargo test -p rk-kernel --test it -- conformance_every_trailer_key_has_a_producer 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: a trailer key in design §5's table still has no producer (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched — the lint was deleted rather than discharged"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item the lane ran — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
log=$(git log "rk-run/$FIX_ITEM" --format=%B 2>&1) || { echo "FAIL: no rk-run/$FIX_ITEM ref — the lane produced no run-owned branch"; exit 1; }
printf '%s\n' "$log" | grep -q '^Story:' || { echo "FAIL: no Story: trailer on the run branch — the stage boundary minted none"; exit 1; }
printf '%s\n' "$log" | grep -q '^Slice-Verdict:' || { echo "FAIL: no Slice-Verdict: trailer — the review stage sealed no slice"; exit 1; }
porcelain=$(git status --porcelain); test -z "$porcelain" || { echo "FAIL: the lane wrote to the user's checkout (AC-518):"; printf '%s\n' "$porcelain"; exit 1; }
prod=$(grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ --include=*.rs | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -cv 'exit.rs'); [ "$prod" -ge 1 ] || { echo "FAIL: Exit::FindingsBlock still has no producer outside its own declaration"; exit 1; }
claim=$(grep -rniE 'guarded against false approval' --include=*.md runbook/ references/ | grep -viE 'may not|never|until S-SUBTLE-CB|no document'); test -z "$claim" || { echo "FAIL: a document claims the review gate is guarded against false approval (ADR-0101 clause 5; S-SUBTLE-CB is UNRUN):"; printf '%s\n' "$claim"; exit 1; }
echo "CP7H-END-OF-BLOCK reached"
```

---
