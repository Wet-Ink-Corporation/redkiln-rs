# W1 — Process health and velocity (status-quo audit of redkiln-rs)

Snapshot: commit `36dd8ac` (branch `build/phase-0-to-1`), audited 2026-10-05. All numbers are computed from `git log`
in the pinned worktree; times are the commits' own author times (UTC-7). "Week" = 7-day bucket from 2026-09-01
(W1 09-01..07, W2 09-08..14, W3 09-15..21, W4 09-22..28, W5 09-29..10-05). "RECORDS claim" and "CODE/GIT shows"
are kept apart throughout.

## 1. Verdict

The programme is moving fast and has been honest about its own failures, but its scope has more than doubled since the
runbook was first written, and the one thing Phase 1 actually exists to prove (a real, landed, self-hosted fix-lane run
and story-lane run) has not yet happened. 27 of the 30 runbook rows are closed; the 3 left are the slowest, most
operator-gated and highest-discovery-risk. Output peaked in W4 and is now falling (commits 141 -> 92, Rust added
81.6k -> 37.6k lines, D21 rulings 101 -> 42), and the last three days (s8i) have produced zero Rust commits. Scope is
converging in form (no new stage inserted for 11 days, no entry gate re-pointed since 09-24) but not yet in fact,
because each earlier expansion was triggered by the first contact with a real run, and the final real runs are still
ahead. Velocity-based projection for Checkpoint 8: central 2026-10-19 to 10-26; earliest credible 2026-10-13;
pessimistic early-to-mid November (section 9).

## 2. Commits per week (git log, 517 commits, 2026-09-01 .. 2026-10-05)

| week | commits | commits/day | note |
| --- | --- | --- | --- |
| W1 09-01..07 | 76 | 10.9 | spikes E0-E3, S-SILENT etc.; pre-runbook |
| W2 09-08..14 | 75 | 10.7 | design/audit remediation 09-10/11, runbook cut 09-11, Steps 0-7a closed, 7b opened |
| W3 09-15..21 | 133 | 19.0 | 7b, 7b-ii, 7c, 7d, 7e, 7f closed |
| W4 09-22..28 | 141 | 20.1 | peak: 7g .. 8e closed (9 stages) |
| W5 09-29..10-05 | 92 | 13.1 | 8f, 8g, 8h closed; 8i open |

Daily low points inside the busy weeks: 09-16 has no commits, 09-30 has 6, 10-01 has 1 (the s8g tail: Checkpoint 8g
needed 62.5 h). Source: `git log --format=%ad` bucketed by date.

## 3. Checkpoint pass dates and stages closed per week

Dates are the tracker commit that records the pass (git author time); hashes are the commits the PROGRESS rows cite.

| stage (INDEX row) | closed (tracker commit) | elapsed since previous close |
| --- | --- | --- |
| Step R remediation | 09-11 (merge `66d0ced` 21:52) | - |
| Step 0 | 09-11 22:21 (`bf5086a`) | - |
| Step 1 | 09-11 22:30 (`8a2b5a3`) | - |
| Step 3 | 09-12 07:17 (work `9b2c848`, tracker `54b61d8`) | 9 h |
| Step 4 / 4a | 09-12 16:09 (`b39581a`) / 16:35 (`db6b55c`) | 9 h / 0.5 h |
| Step 2 | 09-12 18:26 (`f6eba34`) | 2 h |
| Step 5 | 09-12 21:33 (`96ba9c2`; landing row `4885157`) | 3 h |
| Step 6 | 09-13 07:23 (`ea83eac`, M2) | 10 h |
| Step 7a | 09-13 11:22 (`3c8e880`) | 4 h |
| Step 7b | 09-15 20:20 (`cd2699e`, "passed at the new pin"); M3 needs 7b-ii | 2.4 d (opened 09-13 18:09) |
| Step 7b-ii | 09-17 17:00 (`6152219`, M3 reached; work `164e212`) | 1.9 d |
| Step 7c | 09-18 13:41 (`239e899`, work `2791f1d`) | 20.7 h (opened 09-17 22:34) |
| Step 7d | 09-21 15:07 (`c46c80b`, work `99e209c`) | 3.0 d (opened 09-18 21:04) |
| Step 7e | 09-21 17:35 (`0e82e86`) | 2.5 h |
| Step 7f | 09-21 22:38 (`745eca7`) | 5 h |
| Step 7g | 09-22 18:01 (`fc45eeb`) | 19.4 h |
| Step 7h | 09-23 21:38 (`76ae138`) | 27.6 h |
| Step 7i | 09-24 06:17 (tracker `4c282d7`, work `6907312`) | 8.6 h |
| s8a | 09-25 13:53 (`f81e5b9`; work `bf12a1c`) | 31.6 h |
| s8b | 09-26 00:46 (`e771730`) | 10.9 h |
| s8c | 09-26 16:12 (`d46b2eb`) | 15.4 h |
| s8d | 09-27 19:14 (`83cf836`) | 27.0 h |
| s8d-ii | 09-28 08:04 (`766854c`) | 12.8 h |
| s8e | 09-28 20:45 (`78e6922`) | 12.7 h |
| s8f | 09-29 15:32 (`084c10b`) | 18.8 h |
| s8g | 10-02 05:59 (`fcdd844`) | 62.5 h |
| s8h | 10-02 17:56 (`8db752c`; Record B flip re-run `09018dd` 21:00) | 12.0 h |
| s8i | OPEN since 10-02 21:24 (`285b659`); 74 h and counting at 10-05 20:21 | not closed |
| s8j, Step 8 | not started | - |

Stages closed per week (row count): W1 0 (pre-runbook spikes); W2 10 (R, 0, 1, 3, 4, 4a, 2, 5, 6, 7a);
W3 6 (7b, 7b-ii, 7c, 7d, 7e, 7f); W4 9 (7g, 7h, 7i, 8a, 8b, 8c, 8d, 8d-ii, 8e); W5 3 (8f, 8g, 8h).
Per-stage close-to-close time for the 12 stages 7g..8h: min 8.6 h, median 17.1 h, mean 21.5 h, max 62.5 h (8g). The
nine s8 sub-stages closed so far (8a..8h incl. 8d-ii) average 22.6 h each (sum 203.7 h / 9).

Caveat (RECORDS vs CODE): "closed" means the tracker row says checkpoint passed; the tracker itself corrected one of
these. `PROGRESS.md` row 40 / `D21-40` (`spikes/DECISIONS-2026-09-21.md:1561`) states "Checkpoint 8 has NOT passed, and M5
is not reached" - the 09-21 `d6160ff` "pass" was only the mechanical block. Checkpoints 5, 7b and 7f re-measured at
`4c282d7` were rc=1 (D21-40's table). So the stage-closed count is an upper bound on proven progress.

## 4. s8 sub-stage time (the stage family the operator is asking about)

From `3db4f5a` (re-open, 09-24 06:58) to `18b6dfa` (s8h close, 10-02 17:56): 153 commits in 8.4 days; 81 of them (53%)
touch no .rs/.toml/.yml/.json/.py/.sh file. Test counts reported in the close commits: 740 (09-24, `992d257`), 784
(s8b, `42a7f53`), 1034 (s8d-ii, `497266b`), 1119 (s8e, `abeb578`) - tests grew ~50% in four days. s8i (not closed) has
35 commits since stage open, +3,667 lines, all markdown (`git diff --numstat 285b659^ HEAD`: `s8i-s-bin.md` +1,737,
`METHOD-S.md` +1,374, `DECISIONS-2026-09-21.md` +535) and 0 lines of Rust: the S-BIN driver/audit are built in private
corpus copies (commit `5736b64` message) and the stage is dominated by operator-sat paid UE capture runs. s8i timeline:
open 10-02 21:24; refuting rounds 1-9 and 13 interims in 25 h of commits (10-03); B0 signed 10-04 20:06 (`a4c4700`);
B1 go-ahead 10-04 21:18 (`e430c95`); B1 complete 10-05 20:20 (`ee44bf1`, $5.561026 spent, one session stalled on
memory pressure and was retried: D21-150).

## 5. Insertions and re-pointed gates after the plan

Baseline plan: `runbook/INDEX.md` as first committed, `0e34384` (09-11 11:52): execution order
`0 -> 1 -> 3 -> 4 -> 4a -> 2 -> 5 -> 6 -> 7a -> 7b -> 7c -> 7d -> 7e -> 8`, 14 rows (`git show 0e34384:runbook/INDEX.md`).
HEAD has 30 rows (`runbook/INDEX.md:88` row 30 = Step 8). Net: +16 stages (+114%).

| date | insertion | trigger | evidence |
| --- | --- | --- | --- |
| 09-13 | 7b-ii (1 stage, the three gates) | "120 criteria carried inside s7b as one paragraph" | `INDEX.md:47`; commit `80a40da` |
| 09-21 | 7f, 7g, 7h, 7i (4 stages) | first real `redkiln run` showed the executor dispatches activations, not steps; 185/574 steps agent, `StepKind` 0 hits in runtime | `D21-3`, `spikes/DECISIONS-2026-09-21.md:200`; commit `8634de8` |
| 09-24 | 8a-8j (11 stages, incl. 8d-ii) | Checkpoint 8 re-measurement: "criterion (1) is still not satisfiable at HEAD", structural reasons | `D21-40`, `DECISIONS-2026-09-21.md:1561`; `INDEX.md:51`; `b8e7aa9` |

Entry gates re-pointed (RECORDS, citations are INDEX/commit text): (1) Step 7c gate Checkpoint 7b -> 7b-ii (09-13,
`INDEX.md:47-49`); (2) Step 8 gate 7e -> 7i (09-22, `0f68db3`; the INDEX cell itself records that the gate "was never
re-pointed off `7e`" for a day, `INDEX.md:88`); (3) Step 8 gate 7i -> 8j (09-24, `INDEX.md:88`, `3db4f5a`/`b8e7aa9`).
So 3 insertion events and 3 re-pointings, all within the 09-13 .. 09-24 window; none in the 11 days since. Related
correction events: the tracker's execution-order string omitted 7b-ii for two days (`PROGRESS.md:16`); Checkpoint 8
"passed" 09-21 then was un-passed 09-24 (`D21-40`).

Scale of the insertion: the runbook directory grew from 3,576 markdown lines (19 files, `0e34384`) to 50,208 lines at
HEAD (14x); the eleven s8 sub-stage files alone total 32,400 lines (`wc -l runbook/phase-1/s8*.md`; 2,187-3,589
lines each, vs the original single `s8` stage at 523).

## 6. Markdown vs Rust over time

Lines added (sum of `git log --numstat` insertions; .md vs .rs; "tests" = .rs under `/tests/`):

| week | md added | rs added | rs in tests | md:rs (added) |
| --- | --- | --- | --- | --- |
| W1 | 13,276 | 7,458 | 0 | 1.78 |
| W2 | 57,654 | 59,125 | 25,647 | 0.98 |
| W3 | 17,744 | 71,669 | 41,594 | 0.25 |
| W4 | 55,019 | 81,606 | 47,469 | 0.67 |
| W5 | 12,328 | 37,648 | 26,404 | 0.33 |

Standing size (`git grep -c ''` at each week's last commit): 09-07 rs 7,319 / md 12,301; 09-14 rs 55,844 / md 54,295;
09-21 rs 125,131 / md 70,808; 09-28 rs 202,144 / md 119,206; 10-05 rs 238,046 / md 130,756 (HEAD; matches the brief's
"~130k md / ~238k Rust"). The markdown-heavy phases are W1-W2 (spikes, design, audit) and again W4 (the s8
sub-stage files and decision ledger: +55k md that week). Rust is not starved: 238k lines in 4 weeks, ~58-70% of added
Rust per week is in `tests/` paths. A caution: added lines include churn and rewritten text; net growth is the
standing-size table.

## 7. Share of commits that are docs/decisions-only

Per week, commits classified by files touched (code = .rs/.toml/.yml/.lock/.json/.ts/.js/.py/.sh):

| week | commits | code+md | code only | md only | other |
| --- | --- | --- | --- | --- | --- |
| W1 | 76 | 36 | 11 | 25 (33%) | 4 |
| W2 | 75 | 23 | 5 | 39 (52%) | 8 |
| W3 | 133 | 51 | 24 | 56 (42%) | 2 |
| W4 | 141 | 62 | 3 | 72 (51%) | 4 |
| W5 | 92 | 27 | 1 | 64 (70%) | 0 |

Overall 256 of 517 commits (49.5%) are markdown-only; commits touching any .rs: W1 25, W2 17, W3 72, W4 65, W5 27.
The docs-only share is rising (42% -> 51% -> 70%) while Rust-touching commits fall (72 -> 65 -> 27). The W5 figure is
partly s8i, which is mostly operator-driven spend and decision writing rather than code, and partly the s8d-s8h
stage-file/ledger load. Code-only commits have all but vanished (1 in W5): essentially every code commit carries a
record change, which is the design's intent (golden constraints) but is also an overhead tax.

## 8. Operator rulings: accelerating or not

D21-nn headings first appearing, by date (git history of `spikes/DECISIONS-2026-09-21.md`): 09-21 8; 09-22 19; 09-23 11;
09-24 10; 09-25 12; 09-26 11; 09-27 17; 09-28 21; 09-29 18; 09-30 1; 10-02 14; 10-03 5; 10-04 1; 10-05 3. Total 151
(file's own header says D21-1 .. D21-151).

Per week: W3 8, W4 101, W5 42. So rulings peaked in W4 (14.4/day) and are decelerating (6.0/day in W5; 3 in the last
three days). Earlier series: D-1..D-14 (09-14), D17-x, D18-x (about 27 headings on 09-18), D20-1. ADR records added:
W2 122 (89 on 09-09: a batch), W3 11, W4 57, W5 8; 198 ADR files at HEAD (`git ls-tree`, `references/adr`).
Two process-health signals in the decisions themselves: (a) 38 entries contain "TAKEN PROVISIONALLY"
(`DECISIONS-2026-09-21.md`, `grep -c`), taken under the AFK standing rule D21-105 (`:4398`) pending operator review: a
real review debt that lands on the operator before Phase-1 exit; (b) D21-105 ruling 2 authorises paid spend only for
s8i's S-BIN dispatches (US$30 cap) and defers every other live/paid step (8h/8j probes, 8j paid runs, Checkpoint 7c's
paid re-run) "to the review packet", so the remaining stages cannot run AFK.

## 9. Convergence: still expanding, or settling?

Evidence of convergence (CODE/GIT shows): no new stage row since 09-24 (11 days); no gate re-pointed since 09-24;
the remaining stage list (8i, 8j, Step 8) is concrete with detailed stage files; Rust additions and rulings per week
are falling; test counts rose steadily (740 -> 1119 in four days) with checkpoint re-runs green in the tracker rows.

Evidence of continuing expansion / risk:
- Every expansion so far was set off by the first contact with a real artifact, not by planning: first real run on
  09-21 hit the AC-059 cycle lint, then a `RankTie`, then found 0 of 102 activities walkable (D21-1, D21-3, `b514cbe`);
  the 09-24 re-measurement found 4 more structural gaps. No complete real fix-lane or story-lane run has yet been
  executed against the shipped pack (s8 criterion (1) text; `PROGRESS.md` row 8 Step 8 cell: "(1) NOT SATISFIED").
  Those runs are the remaining 2 of the 3 rows and are the highest discovery-risk work.
- Instruments are not yet proven in CI: s8j.7 (`runbook/phase-1/s8j-run-preparation.md:1175-1180`) says the branch has
  never been pushed and `ci.yml` has never run. RECORDS are now stale on the first half: `origin/build/phase-0-to-1`
  exists (tip `7860e66`, 10-03, merge of PR #3; HEAD is 40 commits ahead of it); whether CI ran green on it is
  UNVERIFIED here. `main` is 416 commits behind this branch (`git rev-list --count main..HEAD`), `main` last moved
  09-11 (`353ea01`): the branch is a very large unlanded change.
- Process weight per stage is high and rising: eleven s8 stage files at 2.2k-3.6k lines; each checkpoint re-runs prior
  checkpoints whole (rows 5, 6, 7b, 7b-ii, 7d, 8b, 8d, 8d-ii, 8e, 8f re-run inside Checkpoint 8g per `fcdd844`); s8i
  consumed 9 adversarial refuting rounds in one day and ruled the driver patch "superseded" six times
  (`PROGRESS.md:49`). Adversarial-pass commits per day peaked at 10-11 (09-24, 09-26, 09-27, 10-03: 8).
- Each stage's self-found defects (e.g. `5853aa8`: 36 defects found in 7i's own green work; `b514cbe` s7g blocked,
  measured 0/102) show the instruments are being made honest, which lowers end-risk but is itself why throughput per
  stage is bounded.

Verdict on scope: the plan expanded from 14 to 30 rows in 24 days (+114%) and the in-flight stage s8i has not closed in
3 days; scope is converging only in the sense that the list is currently stable. It will not be demonstrably
converged until the first complete real run lands without an insertion.

## 10. Velocity-based projection to Checkpoint 8 (from 2026-10-05)

Remaining work, as the RECORDS state it: finish s8i (B2 and B3 = 10 Opus review passes, audit reading, finding, `Binary`
variant record and close; `s8i-s-bin.md:1771-1775`, `:3572-3589`), all of s8j (fixture proofs of both lanes, first
ever CI on the pushed branch, human roots, fix-item pick by operator, probe session, staged first run;
`s8j-run-preparation.md` headings), then Step 8's six exit checks with real paid runs of the fix lane (1), story lane
(1b), operational (3) and (7). Plus the debts: 38 provisional decisions to confirm, Record A (awaits s8j probes),
Checkpoints re-run whole.

Three estimation methods (all from git data):

A. Row-count method (naive, optimistic): 3 rows left. Close rate since 09-11 is 27 rows in 24 d = 1.1/day; the last
   three weeks 0.86/day; W5 0.43/day. 3 rows / 0.43 per day = 7 d -> 2026-10-12; at 0.86/day = 3.5 d -> 10-09. This
   method assumes the last rows resemble the earlier ones and is rejected as the headline.
B. Duration-weighted method: remaining-stage durations scaled from comparable stages. s8i is already 74 h and
   operator-paced; B2 and B3 are each a paid 5-session batch (B1's 6 sessions took ~23 h wall incl. one stall), plus
   analysis and a Rust/record close with refuting rounds: 3-5 d more. s8j (2.8k lines, first CI run, two paid probes,
   operator pick): 2-4 d. Step 8 (real runs, final checkpoint, M5): 2-5 d if clean. Sum 7-14 d -> 2026-10-12 to 10-19.
C. Scenario-weighted, with re-opening risk from history (3 insertion events in 24 d; first real runs have always found
   structural defects): 30% clean (7-11 d), 45% repairs inside the stages (12-20 d), 25% another insertion round
   comparable to D21-3 (4 stages / ~3 d) or D21-40 (11 stages / ~11 d) (21-35 d).

Projection for Checkpoint 8 (M5, Phase-1 exit): earliest credible 2026-10-13; P50 about 2026-10-21 (range 10-19 .. 10-26);
P90 about 2026-11-08. Assumptions: (1) the operator is available for the paid/go-ahead steps (D21-105 forbids AFK
spend beyond S-BIN), roughly one operator sitting per day; (2) the AFK-run throughput of W3-W4 is not restored (W5
pace, ~13 commits/day, is used); (3) CI on the pushed branch is green or fixed within a day; (4) no new stage is
inserted before the first real fix-lane run (the scenario C 25% case); (5) the account/spend caps hold and CLI
version churn (2.1.275 -> 2.1.289 in 4 weeks, repeated Halt 2) does not force another pin-move cycle (each cost 2-12 h
historically: `cee6587`, `97424cc`); (6) calendar days, not working days; the commit history shows weekend work.

## 11. completeness_pct

70. Reasoning (by remaining work, not stages): the build side is largely done (~90% of workspace code, tests, records
and instruments: 238k Rust lines, 1,119+ tests, 198 ADRs, 27/30 rows), but the proof side is about 5% done (s8's
criteria (1)/(1b)/(3)/(7) have never been satisfied end to end; the two live fix/story runs and the first CI run are
all ahead), and the proof side is the part that has historically generated the surprises. Weighting build 75% /
proof 25% gives ~69%, rounded to 70. By stage count it would read 90%, which overstates.

## 12. Unverified

- Whether `ci.yml` has run green on `origin/build/phase-0-to-1` (the branch appears pushed; the s8j text says never
  pushed). CI results are not in git.
- Uncommitted work in the live checkout (git status shows 4 modified files at session start) is deliberately not
  read; the numbers are as of commit `36dd8ac`.
- Actual calendar time lost to operator waiting vs agent time cannot be separated from commit timestamps; stage
  "elapsed" is close-to-close and includes idle time. Spend in total ($ actually used vs the US$30/US$90 caps in the
  s8i table) was not audited beyond B1's $5.561026 (`ee44bf1`).
- Test count after s8e (1,119) was not re-read from later tracker rows; 1,119+ is a floor from the commit messages.
- Step R's and Steps 0-5's per-stage elapsed times are not meaningful (they were closed in an 18-hour burst on
  09-11/12 after being hardened in the 09-10/11 remediation); I did not attribute effort to them.
- Dates for Step 5's checkpoint are taken from the landing commit `96ba9c2` (09-12 21:33); a later tracker re-pass was
  recorded in W3 (`ca61771`, `4f2ebd7`, `259e7f4`) after pin moves, so "closed" for rows 5/6/7a is revalidated
  repeatedly, not once.
