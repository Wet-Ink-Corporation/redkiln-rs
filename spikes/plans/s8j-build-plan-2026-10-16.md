# s8j build plan — the batch from Checkpoint 8i to Checkpoint 8j (drafted 2026-10-16)

**What this is.** The execution plan for `runbook/phase-1/s8j-run-preparation.md` (Step 8j, row 29 of `runbook/INDEX.md`),
written the day after `s8i` closed (PR #81, merged 2026-10-16 as `0a86274` on `build/phase-0-to-1`; Checkpoint 8i passed
whole at `b3c746e`, D21-155). It decides nothing. Precedence is ADR-0092's: the ADRs, the AC and SM registers, the spike
files and the stage file all outrank this plan, and where this plan disagrees with the stage file, this plan is wrong.
It orders the stage's work into packages in the same shape `s8i` used (`S8I-C4-PORTS`, `S8I-D2-WIRE-GATE`, refuting
rounds, `R<n>FIX` packages), folds in the `pre-m5` GitHub issues the backlog contract sends here (`docs/BACKLOG.md`:41),
and lists what only Ryan can decide or do.

**Status: proposed, planning only.** No package below starts until Ryan approves this plan (thread request,
2026-10-16). Every push and every paid dispatch stays an explicit operator go-ahead, per act (`s8j`:289-292).

**Measured at** `0a86274` (this branch's base) unless a line says otherwise. Claims are tagged [measured], [documented]
or [inferred] in the runbook's legend.

---

## 1. Where things stand

- `s8i` is CLOSED; Checkpoint 8i passed whole (`runbook/PROGRESS.md` row 8i; D21-155). Row 8j reads *"PLANNED 2026-09-24
  … Not started."* [measured].
- The `s8j` stage file was last edited at `d24d15a` (2026-10-02, `s8h`'s R3FIX). Nothing `s8i` built or recorded since
  then has been read into it [measured: `git log -- runbook/phase-1/s8j-run-preparation.md`]. So the stage opens, as
  `s8i` did with its §8i.0, by measuring what its brief says that the tree does not.
- CI has run and is green: PR #81's head `6c8f99e` passed `gates (ubuntu-latest)`, `gates (windows-latest)`, `no egress
  (AC-498)`, `structural assertions` and `temper` [measured: PR #81 check runs, 2026-10-16]. §8j.7's premise, *"The branch
  has never been pushed … `ci.yml` has never run"* (`s8j`:1178-1180), and its *"known missing step"* (pnpm) are both stale:
  `ci.yml`:94-102 already installs pnpm 10.33.0 [measured]. This is issue #23's s8j half.
- Work now reaches `build/phase-0-to-1` through PRs (#1-#4, #35, #78, #81), not direct pushes. The stage file assumes
  direct pushes (§8j.7 loop step 1, `git push -u origin build/phase-0-to-1`).

## 2. Drift the opening package must record (S8J-A1's starting list)

Each item is a dated `8j.0` line in the stage file, in `s8i` §8i.0's form, measured again at stage open rather than trusted
from here.

| # | The stage file says | The tree says | Consequence |
| --- | --- | --- | --- |
| 1 | The pin moves to the installed CLI, and the adapter pin equals S-BIN's `cli_version` (`s8j`:977-981) | S-BIN's pins block reads `cli_version=2.1.289` (`s8i`:6997). `TESTED_CLI_VERSION = "2.1.278"` (`crates/rk-adapter-claude/src/config.rs`:124). `PERMISSION_MODE_ARGV` holds `2.1.278` and `2.1.287` rows and there is **no 2.1.289 `--help` capture** (`crates/rk-adapter-claude/src/command.rs`:132-135; `tests/fixtures/cli-help/` holds only `2.1.287.permission-mode.txt`) [measured] | The pin-move commit needs a fresh 2.1.289 capture taken by the operator first (`s8h` §8h.0 item 6's rule, which `s8j`:974-975 cites), and its row replaces both old rows |
| 2 | §8j.7: never pushed, CI never ran, the pnpm step is missing | CI green on PR #81; pnpm step present [measured] | §8j.7 is corrected by a dated line; the CI loop becomes "PR head run", see decision D9 |
| 3 | Checkpoint 8h's `MINTED_CLASSES` gains this stage's three classes | It already carries `s8i`'s fourth, `automated-verdict-refused` (`s8h`:2537) [measured] | The dated edit appends three to a four-entry literal; (8)'s expected row counts in 8h move from 55/57 again |
| 4 | Checkpoint 8j (11)'s floor is "CP8i's recorded count + 14", drafted 1263 + N + G | CP8i's recorded count is now a literal in row 8i [documented] | (11) is recomputed once from row 8i's literal, by dated edit |
| 5 | RT-57's record (`s8h`'s Record B) must be `accepted` before any paid step | ADR-0198 is `accepted` [measured] | The evidence file's `record-rt57` line names `0198` |
| 6 | The (g) record (`s8h`'s Record A) flips after both probes | ADR-0197 is `proposed`, and its decision 7 table also needs Ryan's signature (#73) [measured] | The flip needs probes 1 and 2 passed **and** #73 signed |
| 7 | `s8`'s amendments A1–A12 | D21-155 adds one owed to `s8`: its `S8_BIN` (`s8-exit-criterion.md`:438) misses the point-free `.map(BinaryChangeSummary::new)` and qualified `<BinaryChangeSummary>::new(` forms Checkpoint 8i (8)'s `BIN_RE` catches [documented] | One more amendment, A13, applied with A1–A12 |
| 8 | The lane proofs walk `review` | `s8i` put `review_input::route` and `kept_refusal` on `redkiln run`'s review path (`crates/rk-cli/src/review_input.rs`, `verbs/run/kept_refusal.rs`) [measured]; a `Binary` path takes the `automated-verdict-refused` gate, whose answered semantics are unbuilt (#80) | The fixture lanes must produce `Text` summaries only; `lane_proof` asserts the gate set, so a stray `Binary` route fails by name. #80 stays out of the batch |
| 9 | Pre-m5 code PRs merge "just before s8j re-runs its entry chain through Checkpoint 8i" (`docs/BACKLOG.md`:41) | None of #5, #6, #7, #11, #20 has a PR yet [measured: PR list] | They are batch 1 below, ahead of the chain run, if Ryan takes #74 |
| 10 | §8j.5's shortlist was read at `ca8f387` | `s8i` added `rk-ports` `binary`, `rk-store` `render/binary`, `rk-kernel` `workflow/gate`, `rk-cli` `review_input` and `kept_refusal` [measured] | Step 1's exclusion walk is re-run over the new call graph before any candidate is presented |
| 11 | The legacy checkout sits under `.git/redkiln/worktrees/` | Unmeasurable from a cloud container; reads 1-3 of §8j.1(d) are the operator's at stage open | Read on the box |

## 3. The batch, in order

Packages are named `S8J-<round><n>-<TOPIC>`. Every Rust package runs the repository's process: plan
(`temper:rust-architect` writes `.temper/plans/<package>.md`), red, green, refactor, self-review, gate
(`/temper:gate`), and `temper:rust-reviewer`'s validated verdict over the exact tree, then a Status block. Every test the
stage names is written red first and watched fail **for its named reason** before it is made green. Every mutant runs
from the Python driver, never the shell (`s8j`:810).

### Batch 1 — the pre-m5 run-path fixes (one PR each), only if Ryan takes #74

`docs/BACKLOG.md` makes each Task exactly one PR, and its merge hold says these merge just before `s8j` re-runs its entry
chain, so the chain covers them. They land **before** S8J-A2's neutralised chain run, because they touch the run path
the fixture proofs freeze.

| Package | Issue | Crate | Red-first test (from the issue) |
| --- | --- | --- | --- |
| S8J-P1-CANCEL | #5 | `rk-cli` (`main.rs`, `verbs/run.rs`:398) | `signal_during_dispatch_is_cancelled_by_signal_exit_13` driven through the real entry point, red while `run` builds its own token |
| S8J-P2-STATUS | #6 | `rk-cli` (`verbs/status.rs`) | `lane_e2e::two_items_in_one_store_do_not_fold_together` extended to `status`'s stage output |
| S8J-P3-RANK-TIE | #7 | `rk-kernel` (`select/step4.rs`, `rigor.rs`) | a rust-only rank-tie fixture that must be red at load |
| S8J-P4-ENV-SCRUB | #11 | `rk-cli` tests (`tests/it/main.rs`:270-292) | a parent-set `ANTHROPIC_*` variable that leaves the child's launch gate unaffected |
| S8J-P5-ZOMBIE | #20 | `rk-containment` (`posix.rs`) | an unreaped exited member reads `is_empty() == true` |

Each PR is `Closes #N`. The epic #41 closes when all four of its children merge. #20 is POSIX-only and on CI's ubuntu
leg; it is in the batch because it is `pre-m5` and on the CI path, not because #74 names it.

### Round A — open the stage

- **S8J-A1-DRIFT.** Write §8j.0 *"What this file's brief said that the tree does not"*, measured at the stage's entry
  commit, starting from §2's table. Docs only. It also writes the dated lines for drift 2, 3, 4, 5, 7 and 10 at their
  sites, and adds the three ledger entries it needs in `spikes/DECISIONS-2026-09-21.md` (the next free D21 ids at
  writing).
- **S8J-A2-NEUTRALISED-CHAIN (operator, plain terminal).** §8j.10's stage-open run: the twenty blocks of (10) extracted
  and run once each under `runbook/tools/nodispatch.sh` with `RK_CP_NESTED=1`; every failing line recorded, neutralised
  in a scratch copy only, and repeated until each block reaches its end marker. Checkpoints 8h and 8i refuse a
  Claude-spawned shell, so this is Ryan's run (the stage's "working day or more" budget, `s8j`:2067-2069). Its failure
  list is the stage's opening blocker list; the three known entries are in `s8j`:2084-2087. An AFK session can prepare
  the extraction script and read the output; it cannot run the blocks.
- **S8J-A3-SHORTLIST.** §8j.5's survey re-run at HEAD: the static caller walk over the new call graph (drift 10), the
  scheduled-work exclusion re-checked against `s8a`…`s8i` **and the open issues** (a candidate an open issue already
  schedules fails rule 2), and each surviving candidate (C1, C3, C4, C5 today) reproduced red in a throwaway `git archive`
  tree, never in the crate. Output: the presentation table for Ryan's pick, with file, quoted code, broken contract, red
  output and the out-of-run argument per candidate. No code commit.

### Round B — the records (at `proposed`)

- **S8J-B1-ROOT-QUESTIONS.** The human-root record (§8j.2): the question ids `root.bug-report` and
  `root.intake-record`, their one option each with `attaches`, the gate kind the step-4 gate binds under, and the
  `.redkiln/intake/` path convention. ADR-0085's `### Option — <name>` headings (checked by
  `runbook/tools/check-adr-options.py`). Supersedes nothing. Next free id at writing (0200 if nothing else lands first).
- **S8J-B2-HOLD.** The boundary-hold record (§8j.3): the item key `hold_before`, the gate cause, an answered hold never
  re-fires, and a hold naming a stage the pack does not order is refused at load by name. Supersedes nothing, and says
  it invites refutation.
- **S8J-B3-ROOT-ARM (conditional).** Only if S8J-A1 measures that a lane reads a root's **bytes** through an `artifact:`
  input (D21-110 ruling 2): the record superseding ADR-0173 in part, and the resolver's and reader's root arm in
  S8J-C2. Otherwise §8j.2 item 5's close line says the arm stays owed to the first consumer of a root's bytes.
- **S8J-B4-LEDGER.** The seven ledger entries `s8j`:314-322 lists (story candidate with its ADR-0011/E7 reading, Codex
  item retirement, fix-item reservation once picked, fixture-item note, legacy re-point, the two `tag-release` trials
  citing D21-46 R4, and the (g) flip, which is Ryan's line).

Each record's id goes into Checkpoint 8j (3)'s `S8J_Q_ADR=` / `S8J_HOLD_ADR=` line by dated edit when it is written.
Each flips to `accepted` only on Ryan's dated ledger line (ADR-0126's route).

### Round C — the Rust, test-first (one work commit at the stage close; packages are interim commits on the branch)

| Package | What | Tests red first | Mutants |
| --- | --- | --- | --- |
| S8J-C1-ITEMS | `.redkiln/items/rk-story-harden-spike-pack.toml`, `rk-cost-empty-case.toml`; `rk-story-codex-adapter.toml` deleted in the same commit as the 7i re-point; `#`-only note on `rk-fix-doctor-test-count.toml`; `.redkiln/intake/rk-story-harden-spike-pack.IntakeRecord.md`. `spike.toml` untouched | none of its own; the suite and Checkpoint 8j (3) read it | — |
| S8J-C2-ROOT-QUESTIONS | the two entries in `.redkiln/processes/questions.toml` (no second loader, no second file); the step-4 `Unsatisfiable` gate on a `source = "human"` kind bound through `s8d`'s `bind`; `human_root:BugReport` and `human_root:IntakeRecord` as `&str` constants on code lines; their fail-closed `[[gate_expiry]]` rows in both packs; `answer.rs`:706's stale RT-59 comment corrected | `processes::the_two_human_root_questions_load_and_each_attaches_its_kind`, `processes::every_human_kind_a_lane_demands_is_attached_by_exactly_one_question`, `processes::the_step_four_gate_on_a_human_kind_is_bound_to_its_question` | `M-attach-dropped` |
| S8J-C3-HOLD | `hold_before` on the item record; the lane loop opens the hold's gate instead of `AdvanceStage` into the held stage; an answered hold is spent and never re-fires; the load refusal; `hold:specification` constant and rows in both packs; if `gate_opened`'s reason set is closed in `rk-types`, the variant and its schema snapshot in the same change (ADR-0116 d5 by analogy) | `run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load`, and the three `lane_proof::boundary_hold_*` tests once C4's fixture exists | `M-hold-ignored`, `M-hold-spent-refires`, `M-hold-unordered-accepted` |
| S8J-C4-LANE-PROOF | `crates/rk-cli/tests/fixtures/lane/` (one-crate workspace, planted defect, no failing test); `tests/it/lane_proof.rs` with the byte copy of tracked `.redkiln/` minus `worktrees/` (asserting `registry.toml` arrived), `init` through the product path, the real Rust adapter, the scripted harness keyed by `(activity@version, step id, iteration path)` with **no default response**; the binary arm through `support::fixture_child()` and `s8h`'s one spawn helper | the seven non-hold `lane_proof` tests in `s8j`:794-800 | `M-seed-record`, `M-land-empty`, `M-fixture-toolchain`, `M-skip-empty-stage-trailer`, `M-spec-digest-pre-render`, `M-worktree-under-git` |
| S8J-C5-MUTANT-DRIVER | the Python driver that applies each of the ten mutants with sha256-verified restores and writes `<id><TAB>killed\|SURVIVED<TAB><HEAD>` | its own synthetic controls (a known survivor must read `SURVIVED`) | — |

Order: C1, then C2 and C3 (C3's `run::` test needs no fixture), then C4 (which turns C3's three `boundary_hold`
tests from unwritten to red to green), then C5 over everything. The stage's new-test count stays fourteen and its
mutants ten, with `s8h`'s `run::every_binary_spawn_withholds_or_supplies_the_launch_record` selected beside them as
fifteen (`s8j`:202-208, 226). A survivor blocks.

The proofs are slow by design: each builds the fixture crate in the run worktree's own `target/` (ADR-0137 d3). C4's
architect plan should measure one proof's wall time before the other six are written.

### Round D — the runbook amendments (docs and checkpoint lines)

- **S8J-D1-REPOINT.** Checkpoints 7h and 7i stop reading `$FIX_ITEM` (§8j.1(c)), each by one dated edit; rows 7h and
  7i marked `re-run pending (s8j re-point)`.
- **S8J-D2-S8-AMENDMENTS.** §8j.8's A1–A12 applied to `s8-exit-criterion.md` with dated comments, plus A13 (drift 7,
  `S8_BIN`'s two forms, with a synthetic control each way). A10's exemption file `runbook/phase-1/s8j-screen-exempt.txt`,
  re-measured over the blocks as they now stand (s8i's block is new since the last measure). A12's `-H` in `s7c` and
  `s7d`. `MINTED_CLASSES` gains the three classes (drift 3).
- **S8J-D3-RECOVERY.** Issues #61 and #25: a dated "failed first real run" section in `s8-exit-criterion.md` and/or
  `halts.md`, cross-referenced from `s8j`: archive the `rk-run/<item>` ref and the store, mint a fresh item by §8j.5's
  route, and re-run Checkpoint 8j's (3) and (12) against it. Draft only; #61's criteria require Ryan to approve the text
  before the freeze. No ADR decision sentence or ledger entry is edited by an agent.
- **S8J-D4-STALE-CI-RECORDS.** Issue #23's remainder: the "never pushed / ci.yml never ran" statements at
  `PROGRESS.md`:563, `ci.yml`:215, :353, :579 (the ARMED comments get dated corrections only once their job has run
  green, which PR #81's run now shows), ADR-0127:178 via ERRATA, and `crates/rk-containment/src/lib.rs`:38, plus the CI
  run ids and the temper baseline decision (6,606 findings within the baseline, PR #81's temper comment). Re-cite each
  line at the stage's entry commit first.
- **S8J-D5-R64 (AFK, read-only).** Issue #76: list each event-store tag, its source, and whether `doctor --rebuild` can
  re-derive it from git, as a comment on the issue. It feeds `s8`'s (2) rebuild differential, which A2 and A8 amend. No
  record edit.

### Refuting rounds

Adversarial `temper:rust-reviewer` rounds and the runbook's own refuters over the branch, each found item fixed in an
`R<n>FIX-8j` package, until a round moves no code and no guard (`s8j`:2814). The three runbook checkers (`check-refs.py`,
`check-checkpoints.py`, `check-adr-options.py`) and INDEX's validation checks run after every runbook change.

### Round E — the gated acts (Ryan's terminal, in the stage close's six-step order, `s8j`:2773-2812)

1. **The work commit** (everything in rounds C and D, the records at `proposed`, the evidence file opened with
   `record-rt57 0198`).
2. **The pin-move commit**: the 2.1.289 `--help` capture first (drift 1), then the constant, its prose, literal test and
   fixtures, the instrument file by `printf`, and X12's one-row table with `LIVE_WARRANTED` emptied and the golden argv
   moved once. Spend: none.
3. **§8j.6 step 1b**: the fifteen tests and ten mutants recorded over the moved pin (`fixture-proofs`, `mutant` lines).
   Then the probe session, each on its own go-ahead: 7c's suite portion (no spend), 7c's live portion (1 haiku),
   probe 0 (1 haiku), probe 1 (1 Sonnet), probe 2 (3 dispatches, one per pinned model). Six dispatches in all, plus
   #62's if taken. A failure stops the stage and nothing is re-spent before Ryan reads the report. The legacy checkout
   re-point (§8j.1(d)'s five reads, then `git worktree move`). **The evidence commit**, with the (g) flip if both probes
   passed and #73 is signed.
4. **CI** on the evidence commit (decision D9 for how it is pushed). A CI repair moves a code path and so re-opens step
   1b and the paid steps. **The frozen-record commit** appends `ci`, `fix-item`, `story-item`.
5. **Checkpoint 8j**, one-shot, over the frozen-record commit, from a plain terminal, on a clean tree.
6. **The tracker commit**: `PROGRESS.md` rows 7c, 7d, 7h, 7i and 8j, the checkpoint output, the ledger rows.

## 4. What only Ryan can decide or do

Decisions are numbered for the reply thread. Each has a recommendation; none is taken here.

| # | Decision | Recommendation | Blocks |
| --- | --- | --- | --- |
| D1 | Approve this plan | — | everything |
| D2 | #74: fold #5, #6, #7 and #11 (and #20) into the batch ahead of the chain run | **Yes**, as batch 1. They are on the run path the proofs freeze; landing them after the freeze re-opens step 1b and the paid steps | batch 1, A2 |
| D3 | #62: one real dispatch before the freeze. **Conflict found:** the audit names "the fixture item", but D21-42 18 says `rk-fix-doctor-test-count` and its ref are never run, and after §8j.1(d)'s re-point `s8d-ii` refuses it by design | **Yes, on a throwaway item in a scratch clone**, right after the pin move and before step 1b, with its own spend line and go-ahead; recorded as an `s8j` dated step | E2-E3 |
| D4 | #33: human-only closure of verify-stage ACs for the exit runs | **(c)**: accept human-only closure for `s8`'s runs now by a D21 entry and an M5 scope note; schedule the mutation-pin wiring for Phase 2 | `s8`, and A1's lane check text |
| D5 | The fix item: pick one of S8J-A3's presented candidates | your pick after A3 (C1, C3, C4, C5 today; C5 carries a lower-confidence caveat) | C1's fix item, B4's reservation |
| D6 | Accept the two records (and the conditional third) by dated ledger line | after the refuting rounds | E1 |
| D7 | Sign the three new `[[gate_expiry]]` rows (`human_root:BugReport`, `human_root:IntakeRecord`, `hold:specification`, all `fail`) row by row (AC-276) | sign as drafted | E1 |
| D8 | #73: sign ADR-0197's decision 7 table; and #28: confirm or reverse the 20 AFK-provisional rulings | one sitting for both, before the probe session | the (g) flip at E3 |
| D9 | How "CI green on the pushed branch" works now that work merges by PR | the `s8j` branch is pushed and a PR opened against `build/phase-0-to-1`; its head-commit run is the recorded `ci` run; **no other code PR merges into `build/phase-0-to-1` from the freeze until Checkpoint 8**, and the `s8j` PR merges after the tracker commit with a merge commit that changes no code path | E4, `prespend` |
| D10 | #30 (re-point 8j's entry gate to 8h) | close as not planned: 8i has passed, so the question is moot | — |
| D11 | #26 (what M5 certifies) | leave to `s8`'s close; not on `s8j`'s path | — |

**Acts only Ryan can perform:** S8J-A2's neutralised chain run; the 2.1.289 `--help` capture; the legacy checkout's
five reads and `git worktree move`; every push and every paid dispatch, each on its own go-ahead; the evidence lines
marked `pass`; Checkpoint 8j itself; #32 (an off-machine copy of `redkiln-corpus`), best done before the probe session
because the probes' evidence and `CORPUS_PIN` live there; deleting the `s8i` scratch folders under
`D:/repos/redkiln-sev/afk-tools/sp/s8i/tmp/` (D21-155); closing D21-143 when you choose to.

## 5. Out of this batch, and why

- **#80** (what an answered `automated-verdict-refused` gate does): Phase 2, owed to the first `Binary` producer
  (ADR-0199 decision 4). The fixture lanes stay `Text`-only (drift 8).
- **#77** (temper gate repro): `needs/repro`, parented under Post-M5 hardening. If a package's `/temper:gate` reproduces
  the failure, that reproduction is posted on #77, and the package's own gate must still be green.
- **#70 and #71** (splitting `verbs/run.rs`, 6,615 lines, and `rk-git`'s `worktree.rs`): post-M5. Moving the run path
  during `s8j` would churn every checkpoint that reads it.
- **AC-114's promotion and its `curation.decision` lint**: after Checkpoint 8, by §8j.8 A9.
- Every other open issue: not `pre-m5`, so it waits for M5 (`docs/BACKLOG.md`:41).

## 6. Done means

Checkpoint 8j passed once over the frozen-record commit and recorded in `PROGRESS.md` row 8j; the fourteen tests green
with every mutant killed; CI green on the recorded run with no code change since; the pre-m5 PRs merged and their issues
closed; and `s8` able to start with nothing left to decide except the go-ahead for each real run.
