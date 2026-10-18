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
⚠️ **Partly false, re-measured 2026-09-22:** true within `src/`, but `rk-cli/tests/it/doctor.rs:2058-2061`
and `rk-git/tests/it/trailers.rs:178` already construct both and push them through
`commit_with_trailers` — working test-side producers, and the template this stage's `Story:` producer
was built from.

🛑 **CORRECTED 2026-09-23 AT THIS STAGE'S OWN WORK. This read *"This step takes no decision. Both
producers and the plumbing are specified."* and it was wrong nine times over.** The stage took
**ADR-0152 … ADR-0160**. Re-deriving §7h's premise against HEAD rather than inheriting it found one
instruction that cannot be executed (§7h.1's *"continue the loop"*), one ordering that does not exist
(§7h.1's *"the pack's own stage order"*), one latent defect this stage ACTIVATES (a supplied successor
outranking a recorded halt), one contradiction between four accepted requirements (the `Story:`
cardinality), two free residuals nobody had declared (`<project>`, `<slice>`), and — by RUNNING the
binary rather than by reading — a self-transition appended once per invocation and a tier-1 store layer
with no writer on any axis. `s7g`'s file said the same sentence and took seventeen records; the lesson is
recorded here rather than left for a third stage to learn.

### 7h.1 `ctx.successor`, and the stage boundary the FSM already knows how to take

`NextAction::AdvanceStage { from, to }` exists and `fsm::next_action`'s evaluation order already places it fourth,
after `Dispatch` and before `Halt`. What is missing is its input: `RuntimeContext::successor`, which `base_inputs`
never sets, so the arm is dead. The successor is the pack's own stage order — `[policy] stages` — filtered to the
stages the pack does not mark `forbidden`, which `bug-standard` and `story` both declare.
🛑 **FALSE, and ADR-0153 replaces it.** `[policy] stages` has no order to read: a TOML table is
unordered by specification, `toml_subset::Table` is a `BTreeMap` so the twelve stages come back
**alphabetised**, and `parse_spans` cannot recover the textual order because the whole inline table sits
on one line and spans are at line grain. The order is now DECLARED, in a new `[policy] stage_order`
array, cross-checked against `policy.stages` at load.

`replay::resume` gains the `AdvanceStage` arm: append the transition, write its trailer commit, and **continue the
loop into the next stage** rather than breaking.
🛑 **NOT IMPLEMENTABLE, and ADR-0152 replaces it.** Everything the loop reads about *which stage* is
bound before it from `inputs.stage` — including, INSIDE the loop, `fsm::fold(item.id(), &inputs.stage,
&journal)`. A `continue` re-folds the same exhausted stage, reads the same `Some(successor)`, and fires
again. The arm steps and **breaks**, on the `OpenGate` model; the lane loop is `rk-cli`'s, one
`resume` per stage.

⚠️ **M7's reading is CORRECTED, 2026-09-23, by this stage's own adversarial pass.** Mutant M7 changes the arm's `break` to `continue`. Measured **before** ADR-0157's `to != state.stage` guard existed, it did not terminate and was killed at 150s. **Re-measured at the close, with the guard in place, it is GREEN in 11s**: the extra iteration re-folds, the arm declines to fire because the fold has already moved into the successor, and the loop reaches `Done`. So `break` and `continue` are now behaviourally EQUIVALENT at this seam, and ADR-0152's `break` rests on the run-record-grain argument — `run_id` is `format!("{}-{}", stage, &digest[..16])` — rather than on termination. The non-termination reading was true of the code it was measured against and is not true of the code that ships; the guard removed the symptom, not the reason. The existing `_ => break` stays for `Halt` and `Done`, which are
terminal by design.

🛑 **The gate exit is unchanged and must stay so.** `OpenGate` breaks and `run` exits 0 — golden constraint 2,
*"`run` NEVER WAITS"*, and ADR-0128's *"a run that STOPS AT A GATE exits 0"*. Advancing past a stage is not
advancing past a gate.

### 7h.2 The two trailer axes, written where their producers are

**`Story:` at the stage boundary** — value per design §5.4's grammar table, column-anchored, through the wrapper.
🛑 **ONE PER ITEM PER LANE, AT THE *FIRST* BOUNDARY — ADR-0155.** One per boundary is not available:
the value has no stage slot, `reconcile` makes a repeat `duplicate-story-trailer` DRIFT, and
`doctor --rebuild` already writes ONE `stage_transition` per item **and its own comment says why**. The
two axes have different cardinalities on purpose — `Slice-Verdict:` is many-per-slice (F12's rejection
history) — and this sentence flattened them.
*(Re-amended 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.5 and its records table; **ADR-0183**, proposed; D21-41 a.)* ~~**ONE PER ITEM PER LANE, AT THE *FIRST* BOUNDARY — ADR-0155.**~~ The ADR-0155 decision-6 erratum this paragraph carries — *"the first stage boundary"* — is overtaken: the `Story:` value widens to `<project>/<slug> <stage>`, so a boundary no longer repeats a value, and **a stage-bearing `Story:` mark is minted at every boundary** (and a terminal `<project>/<item> closed` mark at the lane's end), each through the kernel's boundary function (`s8e` §8e.3). ADR-0183 supersedes ADR-0155 decisions 1, 2 and 5(a) in part; ~~until the operator's dated ledger line flips it, ADR-0155 remains the accepted record and this re-amendment states the route `s8e` builds.~~ *(dated 2026-09-29, `s8f` build, package S8F-P8-DOCTOR-RULES-AND-NOTES; stale prose owed to `s8f` by D21-97.)* **That line is spent: ADR-0183 is `accepted`** since `s8e`'s flip commit `abeb578` (`spikes/DECISIONS-2026-09-21.md` D21-100, *"ADR-0183 flips to `accepted`"*), and ADR-0155 names it in `superseded_by`. ADR-0155 stays accepted for what ADR-0183 does not supersede; for decisions 1, 2 and 5(a) the stage-bearing mark at every boundary is the accepted rule, no longer a route awaiting its flip. The one legacy stage-less mark on the tree (`rk-run/rk-fix-doctor-test-count` at `73c087a`) is admitted read-only, so Checkpoint 7h's (2) and (3) keep reading what they read. The two axes still differ in cardinality on purpose: `Story:` is one per `(item, stage)`, `Slice-Verdict:` many per slice.
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
⚠️ **FALSE OF THE GUARD, re-measured 2026-09-22, and the distinction matters when choosing a caller
shape.** `authorise_tier1_write` (`rk-git/src/worktree.rs:55-88`) checks normalised PATH EQUALITY against
a **passed** `RunOwnership` and `HEAD == ownership.branch`. There is no `rk-run/` pattern anywhere in that
crate; the spelling and the store lookup are CALLER properties, and the two existing callers disagree —
`answer.rs` reads the pair from the store, `run.rs` reconstructs it by convention. **And it gains ONE
caller, not two**: the `Slice-Verdict:` axis is not delivered (ADR-0160).

### 7h.3 The review and verify modules gain their call site

🛑 **HALF DELIVERED, AND THE TWO HALVES HAVE DIFFERENT REASONS — ADR-0156, ADR-0160 and ADR-0162.**
⚠️ **Corrected 2026-09-23, when guard (10) was closed.** The `review`/`verify` **call site IS
delivered**, over the fixture corpus (ADR-0162, which supersedes ADR-0156 clause 3's *"No fixture
pack is authored"* in part — that sentence was reasoned from ADR-0160's slice-identity absences,
and ADR-0160 clause 5 says in terms that *"a review call site could exist without sealing a
slice"*). The **`Slice-Verdict:` axis is NOT delivered**, and Checkpoint 7h guards (6) and (7)
REQUIRE it to stay that way: they assert the ARMED lint still names `Slice-Verdict` as the sole
missing producer, so sealing a slice here would turn two green guards red. The rest of this
section stands as written. *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8 and §8e.13: `s8e`
sealed the slice — `ItemRecord.slice` declared, both review outcomes sealed through the kernel's tier-1
boundary — and in that commit guards (6) and (7) are amended to the discharged state, the Slice-Verdict
bullet below is dated ✅, and guard (10a)'s FAIL message loses its two executing backticks. Checkpoint 7h is
`passed 490c755, re-run pending` until Checkpoint 8e (10c) re-runs it whole.)* ***Previously: NOT DELIVERED AT THIS STAGE. ADR-0156 and ADR-0160
carry the reasons, and they are different reasons.*** The call site is blocked by ADR-0156's **six** measured absences — blocker (0), found by
running the binary, is that **8 of 8** shipped cards carry no `permission_mode` (AC-214), which is the
exit 3 a real run now stops at one stage after the boundary it takes. The `Slice-Verdict:` axis is blocked
by something else again (ADR-0160): `EventKind::SliceSealed` has exactly one construction in the
workspace and it is `doctor/rebuild.rs`, the REBUILD path; `review::confirm` produces five event kinds and
**no seal**; and `<slice>` is declared nowhere. Those are two tasks, not one done in sequence, and only
the second is blocked on an identity.

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
- **`Slice-Verdict:` has a producer.** ~~❌~~ **ADR-0160**: `slice_sealed` has no run-path producer (its
  one construction is `doctor/rebuild.rs`, the rebuild path), `review::confirm` produces no seal, and
  `<slice>` is declared nowhere. *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8 and §8e.13:
  ✅ — the item record declares `slice`, and the review pass seals both outcomes through the kernel's tier-1
  boundary, `approved` on `blocked.is_empty()` and `changes-requested` before `Exit::FindingsBlock` returns;
  `slice_verdict::approved_is_sealed_on_the_advance_path` and
  `slice_verdict::changes_requested_is_sealed_on_the_findings_block_path` read the mark off `rk-run/<item>`
  with git's own parser. The ❌ above is history. Guards (6) and (7) are amended in the same commit.)*
- **`Exit::FindingsBlock` has a producer.** ✅ **ADR-0162**, over the **fixture corpus**: a real
  `redkiln run review-1` exits `4`, driven by `exit/findings-block/` as AC-438's tenth row. The
  producer is `review_pass`, wired into `resume_and_read` on the return channel; the finding set is
  read from the journal off `activity_committed.result` (`Typed.value` verbatim, AC-009), so no
  kernel seam was owed. ⚠️ **This says nothing about the shipped tree**: ADR-0156 clause 4's
  blockers stand, the shipped review stage is still unreachable, and `0 of 102` shipped activities
  completing a committing walk is unchanged.
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

# *(dated 2026-10-18, s8j build, S8J-D1-REPOINT; s8j section 8j.1(c); s8e: "s8j re-points 7h and 7i when it mints the fresh item")* This checkpoint no longer reads the environment's FIX_ITEM, which s8 sets to the item s8j section 8j.5 reserves: it names the fixture item it was built over. Was: test -n on that variable, with a FAIL asking the operator to set it.
FIXTURE_ITEM=rk-fix-doctor-test-count
test -f ".redkiln/items/$FIXTURE_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIXTURE_ITEM.toml does not exist — the fixture item this checkpoint reads is gone (s8j section 8j.1(d) keeps it)"; exit 1; }
git rev-parse --verify --quiet "refs/heads/rk-run/$FIXTURE_ITEM" >/dev/null || { echo "FAIL: no rk-run/$FIXTURE_ITEM ref — the lane produced no run-owned branch"; exit 1; }

# ---- (1) THE BRANCH IS THE RUN'S OWN WORK, not a stale ref. This is the BEFORE assertion: it was
# ---- FALSE at fc45eeb, where the ref existed and was 0 commits ahead.
base=$(git merge-base HEAD "rk-run/$FIXTURE_ITEM") || { echo "FAIL: rk-run/$FIXTURE_ITEM shares no history with HEAD"; exit 1; }
ahead=$(git rev-list --count "$base..rk-run/$FIXTURE_ITEM")
[ "$ahead" -ge 1 ] || { echo "FAIL: rk-run/$FIXTURE_ITEM is $ahead commits ahead of the merge base — a stale ref is not a run"; exit 1; }

# ---- (2) THE `Story:` TRAILER, read by GIT'S OWN PARSER and scoped to the run's own commits.
# ---- `grep '^Story:'` is not used: it matches wrapped prose in a commit BODY, which is how this
# ---- guard's `Slice-Verdict:` twin was green on an untouched tree.
story=$(git log "$base..rk-run/$FIXTURE_ITEM" --format='%(trailers:key=Story,valueonly=true)' | grep -c .)
[ "$story" -eq 1 ] || { echo "FAIL: the run's own commits carry $story real Story: trailers, expected exactly 1 — 0 means the stage boundary minted none, >1 is duplicate-story-trailer DRIFT (ADR-0155)"; exit 1; }

# ---- (3) AND THE VALUE PARSES. A trailer that appears and does not parse records nothing:
# ---- `Story: /item` is a BareStory, counted into bare_story_ids, never a transition (ADR-0159).
val=$(git log "$base..rk-run/$FIXTURE_ITEM" --format='%(trailers:key=Story,valueonly=true)' | grep . | head -1)
proj=${val%%/*}; slug=${val#*/}
[ -n "$proj" ] && [ "$proj" != "$val" ] || { echo "FAIL: the Story: value '$val' has no non-empty <project>/ prefix — split_qualified returns None and the mark is a bare story id"; exit 1; }
[ "$slug" = "$FIXTURE_ITEM" ] || { echo "FAIL: the Story: slug is '$slug', not '$FIXTURE_ITEM' — doctor --rebuild resolves the stage from .redkiln/items/<slug>.toml and reports unresolvable otherwise"; exit 1; }

# ---- (4) THE STAGE BOUNDARY IS IN THE JOURNAL, and is not a self-transition (ADR-0157).
runs=$(ls -d .git/redkiln/runs/* 2>/dev/null | grep -c .)
[ "$runs" -ge 1 ] || { echo "FAIL: no run record under .git/redkiln/runs — the lane did not run"; exit 1; }

# ---- (5) AC-518: THE USER'S CHECKOUT IS UNCHANGED. The tracker row may be unstaged at this moment
# ---- (00-preamble: a §1 row is hashless between the two close commits), so the guard names what it
# ---- tolerates rather than demanding a bare-empty tree.
porcelain=$(git status --porcelain | grep -v 'runbook/PROGRESS.md')
test -z "$porcelain" || { echo "FAIL: the lane wrote to the user's checkout (AC-518):"; printf '%s\n' "$porcelain"; exit 1; }

# ---- (6) [amended 2026-09-28, s8e build, package S8E-P7; s8e section 8e.13; D21-47's time-bound rule] THE TRAILER LINT IS
# ---- DISCHARGED. s8e's Slice-Verdict commit seals both review outcomes through the kernel's boundary (seal_act) and deletes
# ---- the lint's #[ignore] in that commit. This guard asserted the lint ignored and ARMED with Slice-Verdict as its one
# ---- missing key, and one ignored; both went red by design there. The lines are REPLACED, not struck: a struck line in
# ---- this bash block would still carry the ARMED string, and Checkpoint 8e re-runs this block whole (its guard (10c)).
# ---- Now: the lint by its module-qualified name, its ok line, and exactly one passed with none ignored.
out=$(cargo test -p rk-kernel --test it -- conformance::conformance_every_trailer_key_has_a_producer --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the discharged trailer lint did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qxF 'test conformance::conformance_every_trailer_key_has_a_producer ... ok' || { echo "FAIL: the trailer lint did not report its ok line — it went red, was renamed, or was re-ignored"; printf '%s\n' "$out" | grep -E '^test |test result:'; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed; 0 ignored' || { echo "FAIL: the trailer lint's selection is not exactly 1 passed, 0 failed, 0 ignored"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }

# ---- (7) [amended 2026-09-28, s8e build, package S8E-P7; s8e section 8e.13] THE ARMED REASON IS GONE, NOT REWORDED. This
# ---- guard ran the lint with --ignored and required its failure to name Slice-Verdict alone; after the discharge the lint
# ---- passes, so it asserts instead that the ARMED string is absent from conformance.rs, with grep's rc exactly 1.
hits=$(grep -nF 'ARMED: Slice-Verdict has 0 producers' crates/rk-kernel/tests/it/conformance.rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the ARMED reason is still present in conformance.rs, or the file is unreadable (grep rc=$rc)"; printf '%s\n' "$hits"; exit 1; }

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

# ---- (10) `Exit::FindingsBlock` HAS A PRODUCER A RUN REACHES (ADR-0162 decision 7).
#
# 🛑 **THE AUTHORED FORM OF THIS GUARD COULD BE SATISFIED BY A DOC COMMENT, and measurement caught
# it in the act.** As written it was `grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ | grep -v
# exit.rs | grep -c .` with a `-ge 1` test. The moment the producer landed that grep returned
# **three** hits and **two were prose** — `review_pass.rs`'s module header and `run.rs`'s function
# doc comment [measured 2026-09-23]. So the authored guard would have gone green on the strength of
# two sentences about a producer, before any run reached one.
#
# That is the shape this programme has paid for twice and `0160` clause 4 forbids in terms: both
# `s7g` source lints went green while `execute_step` had no caller, and `0158` measured three
# declared tier-1 tables with zero writers. A grep over `src/` cannot tell a producer from a
# mention of one. So the assertion is now a **binary run that exits 4**, and the grep stays only
# as a floor — scoped to CODE lines — that names a deleted call site at its own line.

# (10a) THE BEHAVIOURAL HALF. The fixture must exist and must expect 4 before the run is read: a
# table-driven test over a missing directory asserts nothing (Halt 8's first shape).
test -d crates/rk-cli/tests/fixtures/exit/findings-block || { echo "FAIL: the findings-block exit fixture is absent — AC-438's table would walk past the row this guard is about"; exit 1; }
# 🛑 `tr -d` AND NOT `$(cat …)`. Command substitution strips trailing NEWLINES and NOT a trailing
# carriage return, so on a checkout this box's `core.autocrlf=true` smudged, the comparison would be
# `4\r` = `4` and this guard would go red for a reason unrelated to the producer. `.gitattributes`
# carries a `-text` row for this fixture; the guard does not depend on it.
#
# ⚠️ **`[:space:]` AND NOT `\r\n`, because the guard must read this file the way the TEST does.**
# `ac438`'s driver parses it with `.trim().parse()`. A first cut deleted CR and LF only, so an
# `expected` of `4 ` or `\t4` made this guard RED while the test it guards stayed GREEN — the same
# unrelated-reason red, one whitespace class over. Found by the adversarial pass.
exp=$(tr -d '[:space:]' < crates/rk-cli/tests/fixtures/exit/findings-block/expected)
[ "$exp" = "4" ] || { echo "FAIL: findings-block/expected reads '$exp', not 4"; exit 1; }
out=$(cargo test -p rk-cli --test it -- run::ac438_table_driven_exit_codes_over_fixtures_binary_arm --exact 2>&1); rc=$?
# [amended 2026-09-28, s8e build, package S8E-P7; s8e section 8e.13] the FAIL message below carried two backticks inside
# double quotes, which is command substitution: its FAIL path ran redkiln run before it echoed. The two backticks are
# removed and no other byte of the line changes.
[ "$rc" -eq 0 ] || { echo "FAIL: AC-438's binary arm did not pass (rc=$rc) — a real redkiln run over the review fixture did not exit 4"; printf '%s\n' "$out" | tail -25; exit 1; }
# 🛑 THE NAME IS MODULE-QUALIFIED, AND THAT IS NOT STYLE. `--exact` on the BARE function name runs
# ZERO tests and exits 0 — measured on this very line on 2026-09-23, which is trap (4) live.
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed' || { echo "FAIL: the exit-table selection did not run exactly one test — a filter that matches nothing is a green"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }

# (10b) THE SOURCE FLOOR, CODE LINES ONLY — and it is a FLOOR, not the assertion. (10a) is the
# assertion; this names a deleted call site at its own line instead of only by a changed exit code.
#
# 🛑 **THE FIRST CUT OF THIS LINE WAS STRICTLY WEAKER THAN THE REPO'S OWN `code_lines`, and the
# adversarial pass broke it four ways.** It read `grep -v 'exit.rs' | grep -vE ':[0-9]+:[[:space:]]*//'`
# and claimed *"prose cannot satisfy this"*. Measured 2026-09-23, all four hold:
#   · a `/* Exit::FindingsBlock */` line SURVIVED — only `//` was dropped, while
#     `crates/rk-cli/tests/it/main.rs`'s `code_lines` has always dropped `//`, `/*` AND `*`;
#   · `grep -v 'exit.rs'` is UNANCHORED and `.` is a wildcard, so it matches CONTENT as well as path
#     and would drop a real producer in `pre_exit.rs` or `verbs/exit.rs` — a false FAIL on a correct
#     tree, and `crates/rk-kernel/src/verify/exit.rs` proves a second `exit.rs` is not fanciful;
#   · the comment pattern was unanchored too, so a code line whose CONTENT held `:12: //` was dropped.
# Both patterns are now anchored to the `path:LINE:` prefix grep actually prints.
#
# ⚠️ **WHAT THIS STILL CANNOT SEE, said plainly rather than claimed away:** a mention inside an
# attribute or a string — `#[doc = "…"]`, or a wrapped `reason = "…"` continuation, of which this
# crate has eight. Those are not comment leaders and no grep of this shape excludes them. The floor
# is therefore NECESSARY-not-sufficient, and (10a)'s binary run is what makes the pair sound.
prod=$(grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ --include=*.rs | grep -v '^crates/rk-cli/src/exit\.rs:' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -c .)
[ "$prod" -ge 1 ] || { echo "FAIL: no CODE line under crates/rk-cli/src/ outside exit.rs names Exit::FindingsBlock — the call site was deleted and only its doc comments remain"; exit 1; }

echo "CP7H-END-OF-BLOCK reached"
```

---
