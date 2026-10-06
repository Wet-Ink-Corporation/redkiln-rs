# W1 — Plan status (redkiln-rs, snapshot 36dd8ac, branch build/phase-0-to-1, 2026-10-05)

Method: read `runbook/INDEX.md` (stage map :43-81, milestones :93-101), `runbook/PROGRESS.md` §1 (rows :20-52), stage-file headers, `spikes/DECISIONS-2026-09-21.md` (D21-2, D21-3, D21-40), and `git log` of the pinned worktree. Dates are `git show -s --format=%ad --date=short <hash>`. "RECORD" = what the tracker/INDEX says; "GIT" = what the commit graph shows. I did not run cargo, so every "checkpoint passed" is a RECORD claim whose commit I confirmed exists and is dated as stated; the pass itself (rc 0, 0 FAIL) is unverified by me.

## 1. Verdict

27 of 30 stages carry a recorded passed checkpoint (90 %). Stage 8i (S-BIN) is in progress (its B0 and B1 batches are done per git; the tracker row lags), 8j has not started, and Step 8 (the Phase-1 exit) is formally RE-OPENED and has not passed. M0-M4 are reached by the record, M5 is not. Only 14 stages existed in the 2026-09-11 baseline (`git show 0e34384:runbook/INDEX.md`, line 18) and 16 were inserted after, so by count the plan has more than doubled. Five of the 27 passes are currently marked provisional with a re-run owed (7c, 7f, 8a, 8c, 8d), mostly parked on `s8j`'s whole-chain re-run, and the 5-gate CI has never executed on a remote because the branch has never been pushed (`runbook/phase-1/s8j-run-preparation.md:1178-1179`; 416 commits ahead of main = 353ea01, 2026-09-11).

## 2. All 30 stages

Legend: "orig. cp" = the commit the tracker records for the first checkpoint pass; "latest" = the most recent whole re-run recorded at the head of the tracker row. Order is INDEX execution order (INDEX:41-43). Blocker = what the record names as outstanding.

| # | stage | status (RECORD) | orig. checkpoint hash | date | latest re-run / standing marks | blocker |
|---|---|---|---|---|---|---|
| 1 | 0 | checkpoint passed | bf5086a | 2026-09-11 | none | none |
| 2 | 1 | checkpoint passed (re-run 2026-09-12 at new ADR count) | 66276bf | 2026-09-11 | none | none |
| 3 | 3 | checkpoints 3a/3b/3c passed | 9b2c848 | 2026-09-12 | none | arm L labels DEFERRED by decision (reports DEFERRED, never a pass); D-FIXCONSUMER runtime probe was UNVERIFIED, countersigned 2026-09-13 (INDEX:107) |
| 4 | 4 | checkpoint 4 passed (ADR-0090) | b39581a | 2026-09-12 | none | `stale_policy` open at the time, closed by ADR-0117 (b66ba53, 2026-09-12) |
| 5 | 4a | checkpoint 4a passed | db6b55c | 2026-09-12 | none | `surfaces` unscored (S-FACET); AC-520 lexicality form owed (PROGRESS :25) |
| 6 | 2 | checkpoint 2 passed; Phase-0 exit (b) met | bbdaf35 | 2026-09-12 | none | arm L DEFERRED; no corpus remote (PROGRESS :26-27) |
| 7 | 5 | checkpoint passed (M1); re-run whole | 96ba9c2 | 2026-09-12 | passed at d24d15a (2026-10-02) | `ci.yml` authored, never run; AC-229 / ADR-0100 d4 errata owed (PROGRESS :28) |
| 8 | 6 | checkpoint passed (M2); re-run whole | ea83eac | 2026-09-13 | passed at 4777941 and d5bfc4f (2026-10-01/02) | R-SQLITE two-process contention half owed, reported not-measured |
| 9 | 7a | checkpoint passed; re-run whole | 3c8e880 | 2026-09-13 | passed at 492e516 and abeb578 (2026-09-28) | register §14 reconciliation owed; L3 poison recovery owed (PROGRESS :30) |
| 10 | 7b | checkpoint passed (needed the single CORPUS_PIN move) | ddc57be | 2026-09-15 | passed at d24d15a | none open |
| 11 | 7b-ii | checkpoint passed (M3) | 164e212 | 2026-09-17 | passed at d24d15a | ADR-0101 cl.5: review gate NOT claimed guarded against false approval until S-SUBTLE-CB runs (unrun) |
| 12 | 7c | passed, PROVISIONAL, re-run pending `s8j` | 2791f1d | 2026-09-18 | whole re-run needs a plain terminal and a paid haiku probe; guard lines amended by s8a/s8c/s8e | re-run owed (s8j §8j.6 step 2) |
| 13 | 7d | checkpoint passed | 99e209c | 2026-09-21 | passed at d24d15a | none open |
| 14 | 7e | CLOSED, checkpoint passed (M4 condition) | dddcdbd | 2026-09-21 | none recorded | none open |
| 15 | 7f | passed, re-run pending (`s8g .redkiln` mark open at head of row) | 79e0ef4 (first recorded as 0718741, corrected 2026-09-22) | 2026-09-21 | whole re-runs recorded at 497266b/be32cda and 492e516/abeb578; newest mark still open | next unnested replay clears it (PROGRESS :36) |
| 16 | 7g | CLOSED, checkpoint passed | 18eb4ae | 2026-09-22 | re-run whole at b183eb9 and 0b470ee (2026-09-27) | none open |
| 17 | 7h | CLOSED, checkpoint passed | 490c755 | 2026-09-23 | re-run whole at 492e516 and abeb578 | none open |
| 18 | 7i | CLOSED, checkpoint passed | 6907312 | 2026-09-24 | re-run whole at ae38553 and fff1112 | none open |
| 19 | 8a | passed, PROVISIONAL, re-run pending `s8j` | bf12a1c | 2026-09-25 | re-run whole at 492e516/abeb578 (09-28), then (11) card-mode guard amended by s8h | owed to s8j's entry-chain re-run |
| 20 | 8b | closed; checkpoint passed | 83d1268 (closed 42a7f53) | 2026-09-26 | re-run whole at d24d15a | none open |
| 21 | 8c | closed; PROVISIONAL, re-run pending `s8j` | 4ab7c67 (re-green after flip e8e4f68) | 2026-09-26 | (15) card-mode guard amended by s8h | owed to s8j |
| 22 | 8d | closed; PROVISIONAL, re-run pending `s8j` | b183eb9 (re-green after flip 0b470ee) | 2026-09-27 | (12a) card-mode guard amended by s8h | owed to s8j |
| 23 | 8d-ii | closed | 497266b (re-green after flip be32cda) | 2026-09-28 | re-run whole at 4777941 and d5bfc4f | none open |
| 24 | 8e | closed | 492e516 (re-green after flip abeb578) | 2026-09-28 | re-run whole at 4777941 and d5bfc4f | none open |
| 25 | 8f | closed | ae38553 (re-green after flip fff1112) | 2026-09-29 | re-run whole at 4777941 and d5bfc4f | none open |
| 26 | 8g | closed | 4777941 (re-green after flip d5bfc4f) | 2026-10-01 (flip 2026-10-02) | re-run whole at d24d15a | none open |
| 27 | 8h | closed (records held) | d24d15a | 2026-10-02 | re-green at b2ab1dc after Record B flip 9ff08fb | Record A (ADR-0197) awaits s8j's probes |
| 28 | 8i | IN PROGRESS, no checkpoint | none | stage open 285b659, 2026-10-02 | GIT: B0 complete (4a3d344, 2026-10-04); B1 complete (ee44bf1, 2026-10-05, $5.561026 spent of the US$90 cap); HEAD 36dd8ac is D21-151 (driver free-space check) | B2 (pass 1, 5 Opus) and B3 (pass 2, 5 Opus) need signed operator go-ahead lines; B4 conditional; tracker row text last dated 2026-10-04 (lags git) |
| 29 | 8j | PLANNED, not started (PROGRESS :50) | none | planned 2026-09-24 | n/a | gated on Checkpoint 8i; scope: fixture proof of both lanes, CI green on a pushed branch, whole-chain re-runs, run preparation |
| 30 | 8 | RE-OPENED 2026-09-24 (D21-40); Checkpoint 8 NOT passed | mechanical block only: d6160ff | 2026-09-21 | n/a | M5 not reached; of the six exit checks (1)(2)(3)(4)(6)(7): (2)(4)(6) pass, (3) half, (1) NOT SATISFIED and (7) NOT RUN at the time (PROGRESS :51; D21-2; D21-40 item 3) |

completeness = 27 passed / 30 = 90 %. Of the 27, five are provisional (7c, 7f, 8a, 8c, 8d); strictly clean passes are 22.

Record-hygiene notes (RECORD vs GIT): (a) PROGRESS §1's intro execution-order string (`PROGRESS.md:16`) still ends `7i → 8` and omits 8a-8j, while INDEX:41 has them; (b) PROGRESS :551 dates Checkpoint 5's execution 2026-09-13 while the work commit 96ba9c2 is dated 2026-09-12; (c) the 8i tracker row lags git (B0/B1 completion lives only in the stage file and commit messages).

## 3. Milestones

| milestone | status | evidence | caveat |
|---|---|---|---|
| M0 Phase 0 closed | REACHED (record) | Checkpoint 2 bbdaf35, 2026-09-12 (after 0, 1, 3, 4, 4a) | INDEX:95 row has no REACHED stamp; arm L still DEFERRED, no corpus remote |
| M1 foundation builds | REACHED (record) | Checkpoint 5 at 96ba9c2, 2026-09-12; re-run d24d15a | `ci.yml` never executed on a runner |
| M2 state is durable | REACHED (record) | Checkpoint 6 at ea83eac, 2026-09-13 ("M2", PROGRESS :29) | R-SQLITE contention half not measured |
| M3 kernel decides | REACHED 2026-09-17 (INDEX:98, stamped) | 7b ddc57be (2026-09-15), 7b-ii 164e212 (2026-09-17) | ADR-0101 cl.5: no false-approval-guard claim until S-SUBTLE-CB runs |
| M4 system runs | conditions met by the record, NOT stamped | Checkpoint 7e dddcdbd, 2026-09-21 (INDEX:99 names 7e after 7c, 7d) | INDEX row never updated to REACHED; 7c is currently provisional (re-run pending s8j) |
| M5 Phase 1 exits | NOT REACHED | Checkpoint 8 not passed; D21-40 item 3 corrects the earlier "passed d6160ff" (it was the mechanical block) | needs 8i, 8j, then six checks incl. a real fix-lane run and a story-lane run authoring a Phase-2 story |

## 4. Stages inserted after the original plan

Baseline: 14 stages at the 2026-09-11 runbook split (0 1 3 4 4a 2 5 6 7a 7b 7c 7d 7e 8; `git show 0e34384:runbook/INDEX.md:18`). Steps 4a and 7e were already in that baseline (0e34384, 2026-09-11, added by the audit remediation), so they are not later growth. Sixteen stages were inserted afterwards.

| stage(s) | inserted | commit | reason (as recorded) |
|---|---|---|---|
| 7b-ii | 2026-09-13 | 80a40da | Step 7b was sized at about 227 criteria in one file; 105 gate criteria (INDEX:36 says 120) rode inside s7b as one paragraph behind six assertions (17:1), so the three gates got their own stage and Checkpoint 7b-ii. 7c's entry gate moved 7b -> 7b-ii. |
| 7f, 7g, 7h, 7i | 2026-09-21 (plan 8634de8; each opened: 7f 62f795d 2026-09-21, 7g 085d1aa and 7h 00c6975 2026-09-22, 7i a811cea 2026-09-23) | 8634de8 (D21-3) | Step 8's first real run found the runtime dispatches an activation, not an activity's steps: 185 of 574 catalog steps are `agent`; 113 `tool`, 102 `emit`, 94 `check`, 80 `gate` had no executor; 102 of 102 activities carry a non-agent step; `ports.toolchain` had zero non-test call sites; `AdvanceStage` unreachable. 7f = conformance gate and records, 7g = step executor, 7h = lane completes (successor, two trailer axes), 7i = `redkiln cost`. Operator chose "build it" over narrowing the exit (DECISIONS-2026-09-21:200-238). |
| 8a-8j and 8d-ii (eleven) | 2026-09-24 | a1d35ac (files), b8e7aa9 (INDEX rows 19-29), c9e7b6e (8d-ii split from 8e, D21-46) | D21-40 re-opened Step 8: criterion (1) not satisfiable at HEAD for structural reasons (nothing stages or lands the harness's work, a lane cannot cross processes, the verify stage cannot walk, 0 of 8 cards carry `permission_mode`, no schemas, 0 model rows). Operator ruled "more sub-stages" on D21-3's precedent, real runs gated on fixture proof, corrected row 40's "passed", and ordered Checkpoints 5, 7b, 7f repaired before exit (DECISIONS-2026-09-21:1561-1590). Step 8's entry gate moved 7i -> 8j. |

Scale: the eleven `s8a`-`s8j` stage files total about 31,900 lines (32,400 with `s8-exit-criterion.md`); stages 8a-8h closed between 2026-09-25 and 2026-10-02 (eight stages, eight days).

## 5. What remains (RECORD)

1. Close 8i: B2 and B3 (5 Opus dispatches each; caps 18 Opus, 3 haiku, US$90, of which $5.561026 spent), optional B4, then Checkpoint 8i. Each batch needs a signed operator go-ahead line (D21-143).
2. 8j: fixture proof of both lanes, whole-chain re-run clearing the five provisional marks (7c, 7f, 8a, 8c, 8d), the pin-move probes (Record A), CI green on a pushed branch, run preparation. The branch has never been pushed (s8j:1178).
3. Step 8: six exit checks, including AC-473/AC-521 (fix lane end to end plus a story-lane run authoring a Phase-2 story) and (7) cost rows. Real, paid runs are authorised only after fixture proofs pass (D21-40 item 2).
4. Standing owed items: arm L labels (deferred), M-06 Part 2 (30 seeded P2 labels), S-SUBTLE-CB unrun (blocks any false-approval-guard claim), no corpus remote.

## 6. Unverified by me

- Any checkpoint's actual rc / 0-FAIL result (no cargo run, by instruction); I confirmed only that each recorded hash exists and its date.
- The live tree's uncommitted edits (runbook/PROGRESS.md, s8i-s-bin.md, DECISIONS-2026-09-21.md, METHOD-S.md were modified at session start) are outside the snapshot, so the true current tracker state of 8i may be ahead of what I report.
- INDEX's "120 criteria" vs the commit message's "105 criteria" for 7b-ii (both recorded; not recounted).
- M0/M1/M2 "reached" readings come from tracker Result cells, not INDEX stamps (INDEX stamps only M3).

HASHES: bf5086a 66276bf 9b2c848 b39581a db6b55c bbdaf35 96ba9c2 ea83eac 3c8e880 ddc57be 164e212 2791f1d 99e209c dddcdbd 0718741 79e0ef4 18eb4ae 490c755 6907312 bf12a1c 83d1268 42a7f53 4ab7c67 e8e4f68 b183eb9 0b470ee 497266b be32cda 492e516 abeb578 ae38553 fff1112 4777941 d5bfc4f d24d15a 9ff08fb b2ab1dc d6160ff 8634de8 80a40da a1d35ac b8e7aa9 c9e7b6e 62f795d 085d1aa 00c6975 a811cea 285b659 4a3d344 ee44bf1 36dd8ac b66ba53 0e34384 353ea01
