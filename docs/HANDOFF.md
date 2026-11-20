# Handoff: where redkiln-rs v1 stands (2026-11-20)

This is the cold-start page for an agent or person picking up v1. It is a snapshot. The records win where they
disagree with it: `runbook/PROGRESS.md` (the tracker), `runbook/phase-1/s8-exit-criterion.md` (Step 8),
`spikes/DECISIONS-2026-09-21.md` (D21 entries) and GitHub Issues (`docs/BACKLOG.md`). Re-check the "Current state"
section against them before acting.

## 1. The goal

**v1 = M5 = Phase-1 exit = Checkpoint 8 passed** (Ryan, 2026-10-30). After Checkpoint 8 passes, v1 is tagged and
`build/phase-0-to-1` is merged to `main`. Post-M5 hardening issues (#37's children) are v1.x work and do not block
the tag.

Phase 1 ends when Redkiln-RS runs its own fix lane and authors a Phase-2 story (`runbook/INDEX.md`). Checkpoint 8
therefore needs one closed real fix-lane run (`rk-fix-doctor-margin-8`), one real story-lane run
(`rk-story-harden-spike-pack`, criterion (1b), AC-521) and the free check blocks.

## 2. Standing rules (Ryan's)

- **Branches.** All work merges into `build/phase-0-to-1` through a PR. The branch has a ruleset that requires the
  `temper` check. **Never push or merge to `main`.** `main` is stale at the 2026-09-11 remediation.
- **Decisions go on decision cards**, one question each, with a recommendation. Record each one in the weigh-in
  ledger.
- **Never write pass, evidence or go-ahead lines for Ryan.** The one exception was his typed instruction of
  2026-11-18 06:30Z ("Claude writes the -8 evidence and ledger lines"), and it covered only `-8`'s evidence and
  ledger lines.
- **Paid dispatches** run only on Ryan's PC (`D:/repos/redkiln-rs`, plain Git Bash or the desktop Terminal panel of
  the Remote Control session). They never run from a Claude-spawned Bash: the s8 scripts refuse a shell with
  `CLAUDECODE` set. One typed Go from Ryan per restart covers that restart's launches and gate relaunches.
  D21-40 decision 2 applies: after any failure there is no relaunch until Ryan decides again.
- **Gate answers are Ryan's attestations** (`--explain-sufficed`, `--toml-opened`). He types one plain line in the PC
  thread, for example `AC-2: discriminates, explain-sufficed no, toml-opened no`, and the PC session builds and runs
  the command. Never give him bash to paste there.
- **On Ryan's PC during Step 8:** `rust-src` stays removed (ruling R6). Paid scripts put the pinned CLI first on
  `PATH` (`~/s8j/claude-pin`, 2.1.303). The Terminal panel injects `GIT_CONFIG_PARAMETERS`, so every script unsets it
  first. Don't disable his everyday claude auto-updates.
- **Freeze.** Nothing under CODE_PATHS (`crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests
  tools .github .gitattributes .redkiln`, `s8-exit-criterion.md:54`) may merge until Checkpoint 8 passes. Each paid
  launch's `prespend` refuses any code change since the frozen sha.
- Give Ryan copy-paste commands with concrete values, never placeholders.

## 3. Current state (verified 2026-11-20 19:30Z)

| What | State |
|---|---|
| `build/phase-0-to-1` HEAD | `a6050cc` (#161, the `-8` evidence) |
| Frozen code sha | `a712595` (#160, D21-183: a stated 180 s bound for fixture `run` spawns, plus the `-8` item) |
| s8j | **Closed.** Checkpoint 8j passed at `a4b76e6` (D21-173) |
| Step 8 fix run | **`rk-fix-doctor-margin-8`, parked at the specification hold** since 2026-11-18 about 07:45Z. Run ref `1517b1f` ("hold:specification 1 opened"), gate token `f14d3f116025…`. No failures on the way: archive, step 1b (all mutants killed), evidence, CP8j recheck, preflight, and launches 1 to 3 all returned rc 0 |
| Hold review | Done. It recommends **approve**: the repro was placed exactly as committed and confirmed red, and RootCause covers all three cases (absent `[margins]`, missing window, non-integer value) back to `rules.rs` 845-868. Full text: `/mnt/project-files/s8-r9-out8.txt` |
| PC Remote Control session | **Offline** since 2026-11-18 08:52Z. Messages to it wait until Ryan reopens it |
| Story-lane run, check blocks, Checkpoint 8 record | Not started |

### Deadline: the hold expires around 2026-11-28 07:45Z

Gates are fail-closed with `gate_deadline_secs = 864000` (10 days, `.redkiln/packs/bug-standard.toml:7`). If the hold
is not answered by then, it expires, the run fails closed, and Step 8 needs restart `-9` (D21-163's route). Every
later gate in the run has the same 10-day window.

Earlier attempts and why they stopped are in PROGRESS row 8 (`-1` to `-7`, D21-174 to D21-183). Each failure was fixed
by a PR (#132 to #160) and then restarted.

## 4. The next concrete steps, in order

| # | Step | Who acts | Paid |
|---|---|---|---|
| 1 | Reopen the PC Remote Control session (thread "Let threads run s8 scripts on Ryan's PC") | **Ryan** | no |
| 2 | Type the hold answer in that thread: `Hold: approve, explain-sufficed no, toml-opened no` | **Ryan** | no |
| 3 | (3) kill: `S8_GO=rk-fix-doctor-margin-8 bash ~/s8j/s8-r11-kill.sh 2>&1 \| tee ~/s8j/s8-r11-kill-out.txt`. Run `s8-r11b-capture.sh` only if the kill did not save `target/s8-cp/s8-3-pre.txt` | PC session, under Ryan's `-8` Go of 2026-11-18 06:28Z | yes |
| 4 | Resume: `S8_GO=rk-fix-doctor-margin-8 bash ~/s8j/s8-r12-resume.sh` | PC session | yes |
| 5 | Verify: one `verify.ac-discriminates` gate per AC. After each gate, Ryan types a line such as `AC-1: discriminates, explain-sufficed no, toml-opened no`, and the PC session relaunches with `s8-r8-launch.sh` | **Ryan** answers each gate, then the PC session relaunches | yes |
| 6 | Review (maybe `review.blocking`), release, then closeout (`closure-record@2`, then the `closure.signoff` gate), until the last `Story:` trailer reads `closed` | **Ryan** answers the gates, the PC session relaunches | yes |
| 7 | Story-lane run (1b): `S8_GO=rk-story-harden-spike-pack bash ~/s8j/s8-c2-story-launch.sh 2>&1 \| tee ~/s8j/s8-c2-out.txt`. It starts only after the fix run has closed. It needs **its own Go from Ryan** and stops at each gate for his answer | **Ryan** Go and gates | yes |
| 8 | Check blocks: `bash ~/s8j/s8-c1-checks.sh 2>&1 \| tee ~/s8j/s8-c1-out.txt` | PC session | no |
| 9 | Checkpoint 8's record in PROGRESS and the DECISIONS entry | **Ryan** writes the pass lines | no |
| 10 | Decide what M5 certifies (#26) and how v1 lands (#24: merge form, version form, held-PR order) | **Ryan**, on cards | no |
| 11 | Tag v1, then merge `build/phase-0-to-1` to `main` | **Ryan** | no |
| 12 | Land the held PRs (§5), close #36, and re-parent what is left under it to #37 | agent PRs, Ryan's merges | no |

Copy `s8/cp8/` from the project files into `~/s8j` before step 7: the PC session refreshed only `restart8`. Copy each
step's output into the project files root as `s8-<step>-out8.txt` for review.

**On a failure** (a non-zero rc that is not a gate park, a `FAIL` line, or a `STOP` line): stop, copy the output, run
the free diagnostics `bash ~/s8j/s8-rdiag.sh` and `bash ~/s8j/s8-r18-transcripts.sh`, and bring a fix plan to Ryan.
A verify pass that runs the whole suite red (rc 14) leaves no halt, so a relaunch can re-run it, but only with Ryan's
decision. Run `bash ~/s8j/s8-r20-suite-diag.sh` first. A `NotIsolated` halt (rc 17, guard 3) can never clear in that
run, so the route is a restart (D21-182).

**How threads reach the PC session.** Other threads can't message it directly. Ask the project coordinator to relay.
Its auto mode needs Ryan's approval, typed in that thread, naming the action.

## 5. Open pull requests (all held until Checkpoint 8)

| PR | What | State |
|---|---|---|
| #147 | runbook: one go-ahead per restart covers launches and gate relaunches | Merge after CP8 |
| #148 | rk-store: `identity::read` propagates a failing probe (#17) | Green against `51134da`. Rebase and re-run CI first |
| #149 | rk-types: Digest deserialisation validates (#19) | Same |
| #150 | rk-kernel: a malformed usage charge fails closed (#47) | Same |
| #151 | rk-git: GitError gets Display (#16) | Same. #152 is its follow-up |
| #153 | rk-cli: drop unused anyhow and proptest (#12) | Same |
| #154 | rk-git: trailers only from a trailer paragraph (#18, ADR-0206 proposed) | **Not mergeable.** It breaks AC-444 on the corpus (F12 replay 74 → 4 slices). Waits on Ryan's choice of F (all-trailer paragraphs), B (revert and name the residual) or D (mode split) |

Before merging a PR whose base has moved since its CI ran, run the touched crates' scoped tests on base plus PR in a
detached worktree. `git merge-tree` alone is not enough: #98 and #99 were each green but turned the branch red
together.

## 6. Issues

**M5 initiative (#36), open children:**
- #23: mostly done. The undated "never run" comments in `crates/rk-containment` (`ffi.rs:37`, `:606`, `posix.rs:11`)
  are left for a comment-only PR after CP8.
- #24 (the landing step) and #26 (what M5 certifies): needed for v1. Ryan decides them on cards.
- #25, #33, #61, #62 and #76 (and #90, outside #36) were settled by D21-158, D21-159, D21-161 and D21-163, and closed
  as completed on 2026-11-20 on Ryan's card.

**v1.x issues filed during s8** (under #37): #155 (a guard-3 denial can't be read back; needs a decision), #162
(the margin floor test lacks an exactly-2 assertion), #163 (a `step_refused` halt is never cleared), and #164 (fixture
`run` spawns are slow under full-suite load on Windows; this also covers the flaky-suite history from `-7`).

**Labels fixed 2026-11-20:** #12 and #18 moved from `status/ready` to `status/in-progress`, since PRs #153 and #154
exist.

Everything else open belongs to Post-M5 hardening (#37, #31), Phases 2 to 4 (#38 to #40), or needs triage or a
decision. The AFK pick-up query is in `docs/BACKLOG.md`. Remember that any code PR is held until CP8.

## 7. Pointers

| What | Where |
|---|---|
| Step 8 text and its blocks | `runbook/phase-1/s8-exit-criterion.md` |
| Tracker | `runbook/PROGRESS.md` (row 8 for Step 8, §2 for the D21 register) |
| Decisions | `spikes/DECISIONS-2026-09-21.md` (the latest is D21-183, so the next free id is D21-184) |
| `-8` operator scripts and order | Project files `s8/restart8/README.md`; copies in `~/s8j` on Ryan's PC |
| Rest of CP8 (story run, checks) | Project files `s8/cp8/README.md`, `s8-c2-story-launch.sh`, `s8-c1-checks.sh` |
| `-8` outputs so far | Project files root: `s8-r1-out8.txt`, `s8-r3-out8.txt`, `s8-r4-out8.txt`, `s8-8-launches-paste.txt`, `s8-r9-out8.txt` |
| Audit and estimate (2026-10-30, recount 2026-11-02) | https://claude.ai/artifact/1siFhb4ynEGN6nG1cbehkK |
| Backlog contract | `docs/BACKLOG.md` |
| Project threads | "Take s8 through Checkpoint 8" (the s8 driver) and "Let threads run s8 scripts on Ryan's PC" (the Remote Control session) |
