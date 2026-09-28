# Step 7f — The conformance gate and the records it needs: the design's closed sets, asserted (Phase 1, part 4)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7f` | Phase 1 | Checkpoint 7e | Checkpoint 7f | `s7e` | `s7g` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** Step 8 ran the first `redkiln run` ever attempted on this tree and the shipped pack did
not load. Under the two pack defects sat the finding this step and the three after it exist to repair:
**`Runtime::run_activity` dispatches an *activation*, not an activity's *steps*.** There is no routing over
`StepKind` anywhere in the kernel — measured 2026-09-21, `grep -rn 'StepKind' crates/rk-kernel/src/runtime/` returns
**zero**. Across the shipped catalog that leaves **185 of 574 steps** executed as such: **113 `tool`** (including 28
`toolchain:test`), **102 `emit`**, **94 `check`** and **80 `gate`** have no executor, and **102 of 102** activities
carry at least one non-agent step.

**None of it is undecided.** design §4.5 fixes the five closed step kinds and says `tool` is *"a toolchain verb
through the port"*; design §5's event table names `stage_transition` tier-1 *"(trailer)"*, produced by *"the kernel's
FSM at a stage boundary"*, and `slice_sealed` by *"the lane's review stage"*; ADR-0105 decision 3 specifies the
trailer plumbing; `INDEX.md` puts *"the runtime"* in **Step 7b's** scope. This is execution against accepted records.

**Why the gate comes before the code.** Five stages of checkpoints passed over a runtime that does not implement its
own spec, and none of them was weak — each tested what was built. What no gate did was compare the **built surface**
against the **design's own closed sets**. This step authors that comparison first, so the three stages after it are
measured by something written before them rather than by their own account of themselves.

**This step takes one decision and it is recorded, not taken here**: `ImplReport` becomes the sixth toolchain-keyed
kind. The decision is `DECISIONS-2026-09-21` **D21-4**, taken before this stage opened; §7f.2 executes it, and
it executes it as a **SUPERSESSION IN PART** (`ADR-0132`), not an amendment — see §7f.2.

### 7f.1 The conformance gate — four assertions over closed sets, three of them ARMED

**The house pattern is `live.rs`'s: authored, ARMED, not claimed.** Each of the first three lints asserts the
**target** state and carries `#[ignore]` naming the debt and the stage that discharges it, **in the literal
form `discharged by s7g` / `discharged by s7h`** — the blocks below grep the reason for it, so the spelling
is load-bearing and is fixed here rather than left to each author. The ignore is removed by
the landing that makes it pass, **in the same commit**, so no stage can claim a capability the lint does not see. An
`#[ignore]`d test is coverage theatre unless something counts it, so Checkpoint 7f counts them **by name**.
*(Amended 2026-09-24, s8a §8a.5: the trailer lint's reason now ends `discharged by s8e`, D21-41 (e)'s stage. Its old
tail `discharged by s7h` was false, because `s7h` did not discharge it (ADR-0160). Checkpoint 7f's reason regex
follows; the table below and this section's work block record the arming as it stood.)*

The four, and what each pins:

| lint | closed set it reads | measured 2026-09-21 | discharged by |
| --- | --- | --- | --- |
| `conformance_every_step_kind_has_an_executor` | design §4.5's five step kinds | **0** `StepKind` references in `runtime/` | `s7g` |
| `conformance_every_port_has_a_call_site` | ADR-0003's port table | harness 1, store **1** *(read `store 4` until 2026-09-22; the 4 counted `store_guard()`, a kernel-private helper, not the port — `conformance.rs`’s own module doc said 1 all along)*, **toolchain 0, tracker 0, docs 0** | `s7g` (toolchain), later (tracker/docs) |
| `conformance_every_trailer_key_has_a_producer` | design §5's event table | `Gate` 2, **`Story` 0, `Slice-Verdict` 0** | `s7h` |
| `shipped_pack_loads` | the product's own `.redkiln/` | **RED** — `RankTie` over `ImplReport` | **this step**, §7f.2 |

**`shipped_pack_loads` is the one that would have caught all of this**, and its absence is the whole lesson: every
catalog test in this tree runs over a FIXTURE catalog. Nothing ever loaded the 102 activities the product ships. It
reads `.redkiln/` through `workspace_root()`, the route `tests/it/dag.rs` already uses for `spikes/p9/facets.toml`,
and it asserts the shipped pack passes **every load-time lint** — `requires_cycle_lint`, `excludes_lint`,
`dangling_requires_lint` and `rank_lint` — not merely that it parses.

🛑 **The first three lints scan CODE lines only.** A comment naming `StepKind`, a doc-comment naming a port, or a
grammar file naming a trailer key must not satisfy them — that is how a guard gets satisfied by the prose beside it,
which this programme has now been bitten by eight times. The source-lint precedents are `tests/it/retry.rs`'s
`ac028_policy_match_has_no_wildcard`, `tests/it/recorded_log.rs`'s `the_replay_still_calls_the_divergence_detection`
and `tests/it/projection.rs`'s `stage_action_still_filters_to_tier_one` (Step 8).

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# The three ARMED lints must exist, be IGNORED, and name their discharging stage in the reason.
# A PLAIN RUN, never `--list`. Measured 2026-09-21: `--list` prints `module::name: test` with NO
# ignored marker and NO reason text, so a listing can count the lints, can never tell an ARMED one
# from a DISCHARGED one, and can never carry the `discharged by` reason the last guard below greps.
# A plain run prints `test <name> ... ignored, <reason>` plus a result line carrying the counts.
# CORRECTED 2026-09-21, at this stage's own execution, and the correction is recorded rather than
# silently applied because the withdrawn sentence is what a later author would cite to revert to
# `--list`: this read "`--ignored --list` returns the IDENTICAL set — `--list` does not honour it".
# That INFERENCE IS FALSE. Measured on cargo 1.98.1 / rustc 1.98.1: `-p rk-kernel --test it --
# --list` is 200 tests and `-- --ignored --list` is 4; `-p rk-cli` is 101 and 0. `--list` DOES
# honour `--ignored`. The observation behind the claim was confounded by its filter — every test
# matching `child` is ignored, so both listings were trivially equal; with a filter that straddles
# the boundary they diverge (`-- ac0 --list` 34, `-- ac0 --ignored --list` 3). The OPERATIVE choice
# is unchanged and is correct for the reason stated above; only the false premise is withdrawn.
# TWO THINGS THE REHEARSAL OF THESE LINES FOUND, both measured 2026-09-21 against the four existing
# ignored child entrypoints: (a) `\b` does NOT anchor to a test name — `_` is a word character, so
# `^test .*\bchild` scored 4 by matching the word `child` in the REASON while
# `^test [A-Za-z0-9_:]*\bchild` scored 0; the pattern below anchors to the NAME position instead.
# (b) `grep A | grep -v B; [ rc -eq 1 ]` is GREEN when grep A matches nothing, so the armed set is
# counted before it is filtered. Both forms were checked in both directions.
out=$(cargo test -p rk-kernel --test it -- conformance_ 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the conformance run could not complete (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(printf '%s\n' "$out" | grep -cE '^test [A-Za-z0-9_:]*conformance_'); [ "$n" -eq 3 ] || { echo "FAIL: expected exactly 3 conformance lints, found $n — an armed lint that is deleted reads as a discharged one"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed; 3 ignored' || { echo "FAIL: the three conformance lints are not all ARMED at this stage — s7f arms them and s7g/s7h discharge them"; printf '%s\n' "$out" | grep -E '^test |^test result'; exit 1; }
armed=$(printf '%s\n' "$out" | grep -E '^test [A-Za-z0-9_:]*conformance_.*\.\.\. ignored,'); a=$(printf '%s\n' "$armed" | grep -c .); [ "$a" -eq 3 ] || { echo "FAIL: $a of 3 conformance lints are ARMED — a lint that RUNS here was discharged by a stage that has not happened"; exit 1; }
printf '%s\n' "$armed" | grep -vE 'discharged by (s7g|s7h)'; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: an armed lint's #[ignore] reason does not name its discharging stage (rc=$rc) — the reason IS the debt record"; exit 1; }
# And the one this step discharges must be GREEN, not ignored.
out=$(cargo test -p rk-kernel --test it -- shipped_pack_loads 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the shipped pack does not load (rc=$rc) — §7f.2 is not done"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched shipped_pack_loads — the product's own pack is asserted by nothing"; exit 1; }
```

### 7f.2 `ADR-0132` supersedes ADR-0073 in part — `ImplReport` is the sixth toolchain-keyed kind

**The decision, and its warrant.** `redkiln run` refuses the shipped catalog at `RankTie` over `ImplReport`:
`hotfix` and `implement-ue` both provide it at `method_rank = 10` and are simultaneously admissible. Measured
2026-09-21 over all 102 activities, there are **9 equal-rank producer sets**; **5** are on toolchain-keyed kinds and
step-2 keyed expansion resolves them. Of the remaining four, **two are `ImplReport` shadowing `Changeset` exactly** —
identical producer sets at both ranks, `dependency-update`/`hotfix`/`implement-ue`/`test-after` at 10 and
`bdd-outside-in`/`walking-skeleton` at 30 — because every activity that provides `Changeset` also provides
`ImplReport`, and `Changeset` is keyed while `ImplReport` is not.

**The other two are not ties at all** and no change is owed for them: `checklist-review` (`kind:chore`, `kind:docs`)
and `spike-decision-review` (`kind:spike`) are disjoint by tag, as are `fix-spec` (`kind:bug`, `kind:security-vuln`,
`kind:incident`) and `user-story-ac` (`kind:story`, `kind:project`, `kind:tech-debt`, `kind:docs`). No item's `kind`
is in both sets, so neither pair is ever simultaneously admissible [measured 2026-09-21]. **Keying `ImplReport`
therefore clears the entire remaining rank-tie surface**, which is why it is taken over narrowing a tag.

**`artifacts.toml` was not at fault** and this moves the RECORD, not the pack: the
registry carries precisely the **five** keyed kinds the design names — `ChangeSummary`, `Changeset`, `ProfileReport`,
`ReproCase`, `TestReport` — and `ImplReport`, the same activities' other output, is not among them with no record
stating why. ADR-0132 moves that count to **six**, and `spikes/p9/artifacts.toml` is **not** edited: §7b.12
forbids editing the spike registry to make a case go green, and `.redkiln/` is derived output, which is the half
this step may author.

**IT IS A SUPERSESSION, NOT AN AMENDMENT, AND THE ROUTE IS NOT THE RUNBOOK'S TO CHOOSE.** ADR-0073's five
keyed kinds are named at `:47-52`, **inside `## Decision`**. ADR-0092 rule 1: *"An erratum ... leaves every
sentence of `## Decision` ... standing as written. A correction that would move either is a supersession and
takes a new record."* Adding a sixth moves that sentence. So this step writes **`ADR-0132`, superseding
ADR-0073 IN PART** — the keyed-kinds sentence only — on ADR-0128→0107's form: 0073 stays `accepted` and gains
`superseded_by: ["0132"]`, every other clause of it stands (PC7(ii)'s withdrawn specificity tie-break, and its
explicit *"what replaces the tie-break for kinds that are not keyed is not stated in the source, and is not
asserted here"*), and `references/adr/README.md` takes its recount and index row in the same commit. The
record is committed `proposed` and flips on its own dated line — ADR-0126's route, which 0128, 0129 and 0130
each took.

**Not by `method_rank`.** ADR-0056 makes a tie a **validation error** precisely so that it is not resolved by
nudging a number; `method_rank` is a method-preference ordering and using it as a tie-break would encode a
preference nobody holds.

### 7f.3 The `rate_limit_reading` producer, named

**ADR-0110 defines headroom as a batch delta over *"readings sharing one `resetsAt`"*, and nothing journals a
`resetsAt`.** `EventKind::RateLimitReading` exists with the wire name `rate_limit_reading` and has **no producer
anywhere in the workspace** — its only three references are kernel match arms classifying it as tier-2. The row that
IS written on every dispatch, `DispatchRecord`, carries `headroom_before` / `headroom_after` as
`WindowReading { five_hour, seven_day }`, which has no such field.

**The value is already in hand and is discarded.** `rk-adapter-claude`'s `headroom::read_rate_limit_event` returns a
`RateLimitReading` carrying `resets_at`, `five_hour_resets_at` and `seven_day_resets_at`, and
`ClaudeHarness::last_reading()` exposes it. So the gap is a missing **producer**, not a missing measurement, and the
route taken is to journal the reading rather than to amend ADR-0110 toward what happens to be recorded — F6's
quantisation finding is what made the `resetsAt` grouping necessary, and a delta over "readings in one run" would be
a different measure wearing the same name.

This section authors the **record** naming the producer and its row's payload fields against design §5's event
table. `s7i` builds it and `redkiln cost` on top of it.

### 7f.4 RT-19 — the cost unit

`s7c` §7c.1 raised RT-19 and `s7d` §7d.8 routed it here: *"AC-395's `redkiln cost` and AC-372's renderers — `s7c`
ARMED them here and they are ARMED on to Step 8 with the cost unit record RT-19 still OWED."* Step 8 did not build
the verb, and recorded why. The renderer is what forces the unit question, so the unit is settled in the same pass
that names the producer and before `s7i` renders anything: `total_cost_usd` is `CostUsd(String)`, **the decimal
digits verbatim as the CLI printed them, never a float** (`s7c` §7c.1 as amended), and this section records what a
reader may and may not do with them.

### 7f.5 Checkpoint 7f

The following remain true, and the four assertions of §7f.1 are run rather than read:

- **The three ARMED conformance lints exist, by name, and each names the stage that discharges it.** A deleted
  armed lint reads exactly like a discharged one.
  *(Amended 2026-09-24, s8a §8a.5: D21-9 executed. The block below asserts the **discharged** state that is true now:
  exactly three `conformance_` lints; `conformance_every_step_kind_has_an_executor` and
  `conformance_every_port_has_a_call_site` each `... ok`, both discharged by `s7g` in the commit that made them
  pass (`fbf3c4c`; `s7h` removed no `#[ignore]` and only narrowed the trailer lint's reason — corrected 2026-09-25,
  s8a fix round 1: this said "discharged by `s7g` and `s7h` in the commits that made them pass"); `conformance_every_trailer_key_has_a_producer` ignored with
  `ARMED: Slice-Verdict has 0 producers`, its reason naming `discharged by s8e` (D21-41 (e)); and
  `test result: ok. 2 passed; 0 failed; 1 ignored`. The reason's old tail `discharged by s7h` was false:
  `s7h` did not discharge it (ADR-0160). The old text asserted `0 passed; 0 failed; 3 ignored`, stays
  non-re-runnable as ADR-0134 decision clause 5 says, and is not reversed here: D21-43 N27, *"Re-scoping 7f is
  execution, not supersession."* `s8e`'s Slice-Verdict commit moves this literal again.)*
- **`shipped_pack_loads` is GREEN** — the product's own `.redkiln/` passes every load-time lint, not merely parses.
- **`redkiln run` no longer refuses at catalog load**, which is this step's whole externally visible effect.
- **`artifacts.toml` carries six toolchain-keyed kinds** and `spikes/p9/artifacts.toml` is unchanged.
- **No `method_rank` moved** — the tie is resolved by keying, per ADR-0056.

*(Amended 2026-09-27, `s8d-ii` build, P6: `s8d-ii` §8d-ii.7's three amendments, in the commit that moves the run-tree
root to `.redkiln/worktrees/<item>` under the main worktree (D21-44; ADR-0180, proposed).)* Under that stage's binary
the run arm would create a second, full checkout inside this repository that nothing removes (D21-45 W2 never
force-removes), so:
1. **The run arm runs in a throwaway clone outside this repository, on a short path.** `c7f=$(mktemp -d)`, converted
   with `cygpath -m`; `clone="$c7f/c"`, made by `git clone --quiet --no-local`; `init` and then `run
   rk-fix-doctor-test-count --lane fix` run with the clone as the working directory and the binary named by its
   absolute path; the run-record reads name `$clone`; and the arm ends with `rm -rf "$c7f"`. The clone's `rk-run/`
   refs, worktrees and store are its own, so nothing in this repository moves. Its worst-case run-tree path is the
   `mktemp -d` root plus `/c` (50 characters measured at stage open) + 20 + 24 + 11 + 1 + the longest tracked path,
   246 of 259 (`s8d-ii` §8d-ii.0 item 2).
2. **The cleanup line is deleted.** It force-removed the legacy tree D21-44 keeps for `s8j` (*"stays where it is"*),
   and W2 forbids a forced removal.
3. **The prune line is deleted.** It ran in this repository, and a prune removes the registration of any tree whose
   directory is missing, which would be the registration D21-44 keeps for `s8j`.

The checkpoint's assertions about the run (it executes, it gets past catalog load, it opens a run record carrying
`dag_assembled`) are unchanged; they read the clone's run record. `s8e`'s literal for the discharged `Slice-Verdict`
lint stays `s8e`'s (§8e.13), and `s8j` §8j.8 amends this arm further, taking the clone as `$clone`.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
test -d .redkiln/catalog && test -f .redkiln/artifacts.toml || { echo "FAIL: .redkiln/ not derived — Checkpoint 7f NOT RUN (Halt 8)"; exit 1; }
keyed=$(grep -c '^keyed_by = "toolchain"' .redkiln/artifacts.toml); [ "$keyed" -eq 6 ] || { echo "FAIL: $keyed toolchain-keyed kinds, expected 6 (ADR-0132, superseding ADR-0073 in part, 2026-09-21)"; exit 1; }
grep -q '^\[artifact.ImplReport\]' .redkiln/artifacts.toml || { echo "FAIL: ImplReport has no registry entry at all"; exit 1; }
grep -q 'superseded_by: \["0132"\]' references/adr/0073-*.md || { echo "FAIL: ADR-0073 carries no back-link to 0132 — a one-sided link is a defect, not a shorthand (adr/README.md)"; exit 1; }
grep -q '^supersedes: \["0073"\]' references/adr/0132-*.md || { echo "FAIL: ADR-0132 does not declare what it supersedes"; exit 1; }
spike=$(grep -c '^keyed_by = "toolchain"' spikes/p9/artifacts.toml); [ "$spike" -eq 5 ] || { echo "FAIL: spikes/p9/artifacts.toml moved to $spike keyed kinds — §7b.12 forbids editing the spike registry"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- shipped_pack_loads 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: shipped_pack_loads exited $rc — the product's pack does not load"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched shipped_pack_loads"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- conformance_ 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the conformance run could not complete (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(printf '%s\n' "$out" | grep -cE '^test [A-Za-z0-9_:]*conformance_'); [ "$n" -eq 3 ] || { echo "FAIL: expected exactly 3 conformance lints, found $n"; exit 1; }
# [amended 2026-09-24, s8a §8a.5] D21-9 EXECUTED: re-scoped to the DISCHARGED state, so this block runs whole at
# s8a's exit (D21-40 decision 4). s7g removed both #[ignore]s in the commit that made the two lints pass (fbf3c4c;
# corrected 2026-09-25, s8a fix round 1: this said s7g and s7h each removed one; s7h removed none), which
# is this file's own convention, so the old literal "0 passed; 0 failed; 3 ignored" went false at s7g. The trailer
# lint is still ARMED on Slice-Verdict; its reason now names s8e (D21-41 (e)), because s7h did not discharge it
# (ADR-0160). The old text stays non-re-runnable (ADR-0134 decision clause 5; D21-43 N27). s8e's Slice-Verdict commit
# moves this literal again, by dated line, when it un-ignores the lint.
for t in conformance::conformance_every_step_kind_has_an_executor conformance::conformance_every_port_has_a_call_site; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: the discharged lint $t did not report '... ok' — it went red, was renamed, or was re-ignored"; printf '%s\n' "$out" | grep -E '^test |^test result'; exit 1; }
done
printf '%s\n' "$out" | grep -qF 'test conformance::conformance_every_trailer_key_has_a_producer ... ignored, ARMED: Slice-Verdict has 0 producers' || { echo "FAIL: the trailer lint is not ARMED naming Slice-Verdict — it was discharged, renamed or deleted, and s8e owns that move"; printf '%s\n' "$out" | grep -E '^test |^test result'; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 2 passed; 0 failed; 1 ignored' || { echo "FAIL: the conformance selection is not exactly 2 passed, 0 failed, 1 ignored — the discharged state moved"; printf '%s\n' "$out" | grep -E '^test |^test result'; exit 1; }
armed=$(printf '%s\n' "$out" | grep -E '^test [A-Za-z0-9_:]*conformance_.*\.\.\. ignored,'); a=$(printf '%s\n' "$armed" | grep -c .); [ "$a" -eq 1 ] || { echo "FAIL: $a conformance lints are ARMED, expected exactly 1 (the trailer lint, s8e's)"; exit 1; }
printf '%s\n' "$armed" | grep -vE 'discharged by s8e'; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: the armed lint's #[ignore] reason does not name s8e, the stage that discharges it (rc=$rc) — the reason IS the debt record"; exit 1; }
# THE BINARY MUST EXIST AND EXECUTE BEFORE ANY ASSERTION ABOUT WHAT IT PRINTS.
# CORRECTED 2026-09-21 at this stage's execution, and the hole was BLOCKING: `target/` is
# gitignored, 0 tracked files live under it, and this block builds nothing — so with the binary
# deleted the whole checkpoint ran to completion and printed CP7F-END-OF-BLOCK with rc=0
# [measured]. A fresh clone, a CI runner or anyone who has run `cargo clean` passed Checkpoint 7f
# with no product binary at all. The guard below captured `rc` and never tested it.
test -f ./target/debug/redkiln || cargo build -p rk-cli --bin redkiln >/dev/null 2>&1
test -f ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln is absent and could not be built — every guard below would be asserting over the empty string"; exit 1; }
./target/debug/redkiln --version >/dev/null 2>&1 || { echo "FAIL: ./target/debug/redkiln does not execute"; exit 1; }
# [amended 2026-09-27, s8d-ii build, P6; s8d-ii 8d-ii.7 amendment 1; ADR-0180, proposed] THE RUN ARM RUNS IN A
# THROWAWAY CLONE OUTSIDE THIS REPOSITORY. From s8d-ii on, `run` creates its tree at .redkiln/worktrees/<item> under the
# main worktree and never force-removes it (D21-45 W2), so a run here would leave a second full checkout registered
# inside the product repository. The clone's refs, worktrees and store are its own; `rm -rf` takes them all. The
# directory is short (mktemp -d, drive-letter form) because the run-tree length rule reads the actual item (8d-ii.6).
rk_bin="$(cygpath -m "$PWD")/target/debug/redkiln"
c7f=$(mktemp -d) || { echo "FAIL: mktemp -d failed — the run arm has no throwaway directory"; exit 1; }
c7f=$(cygpath -m "$c7f") || { echo "FAIL: cygpath -m could not convert the throwaway directory"; exit 1; }
clone="$c7f/c"
git clone --quiet --no-local . "$clone" || { echo "FAIL: git clone --no-local of this repository into $clone failed"; rm -rf "$c7f"; exit 1; }
out=$(cd "$clone" && "$rk_bin" init 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: init in the clone exited $rc"; printf '%s\n' "$out" | head -5; rm -rf "$c7f"; exit 1; }
runs_before=$(ls "$clone/.git/redkiln/runs" 2>/dev/null | wc -l)
out=$(cd "$clone" && "$rk_bin" run rk-fix-doctor-test-count --lane fix 2>&1); rc=$?
[ "$rc" -ne 127 ] || { echo "FAIL: the run did not execute (rc=127)"; rm -rf "$c7f"; exit 1; }
printf '%s\n' "$out" | grep -qE 'contains a cycle|RankTie' && { echo "FAIL: the run still refuses at catalog LOAD:"; printf '%s\n' "$out" | head -3; exit 1; }
# THE POSITIVE HALF. The guard above is a NEGATIVE assertion over a string that is EMPTY in the
# passing case (a successful run prints 0 bytes on both streams, rc=0), so on its own it is also
# satisfied by a nonexistent item id, an invalid --lane, and a missing .redkiln/ entirely — each
# measured 2026-09-21. `dag_assembled` is emitted by selection step 5, which is REACHED ONLY AFTER
# the catalog, the artifact registry and the facet registry have all loaded, so it is the smallest
# honest witness that this step's externally visible effect actually happened.
runs_after=$(ls "$clone/.git/redkiln/runs" 2>/dev/null | wc -l)
[ "$runs_after" -gt "$runs_before" ] || { echo "FAIL: the run opened no run record in the clone ($runs_before -> $runs_after) — it did not get as far as selection"; rm -rf "$c7f"; exit 1; }
newest=$(ls -1dt "$clone"/.git/redkiln/runs/*/ 2>/dev/null | head -1)
grep -q 'dag_assembled' "${newest}events.jsonl" || { echo "FAIL: $newest carries no dag_assembled — the pack did not load, whatever the refusal text said"; rm -rf "$c7f"; exit 1; }
printf '%s\n' "$out" | head -3   # → READ: the run gets past load. A later refusal (schemas, cards, roles) is s7g's and is not this checkpoint's subject.
# [amended 2026-09-27, s8d-ii build, P6; s8d-ii 8d-ii.7 amendments 2 and 3; ADR-0180, proposed] THE HAZARD IS
# DISARMED BY WHERE THE RUN RAN, NOT BY A REMOVAL HERE. The run tree is the clone's, so `rm -rf` of the throwaway
# directory takes it, its registration and the clone's store with it. The two lines that followed are deleted: the
# forced removal of the legacy tree at the old common-dir root (D21-44 keeps that tree and its registration for s8j,
# and W2 forbids a forced removal), and the prune of this repository's registrations (a prune drops the registration
# of any tree whose directory is missing, which would be the one D21-44 keeps).
rm -rf "$c7f" || { echo "FAIL: the throwaway clone $c7f could not be removed"; exit 1; }
echo "CP7F-END-OF-BLOCK reached"
```

---
