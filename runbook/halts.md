# Halt conditions — these apply at EVERY stage

*Part of `runbook/`; see `INDEX.md`. Stop, record in `PROGRESS.md`, and do not continue past any of these. None is discretionary.*

1. **The rig fails to build.** `cargo build` in `spikes/runner` (or `cargo build --workspace` from Step 5 on) does
   not exit 0. **Do not run the binary that is already in `target/`** — `cargo test` and `cargo clippy` do not
   rebuild it, and a stale binary is how this programme reached two false conclusions that code was broken.

2. **The instrument version differs from the pin.** As of 2026-09-10 it already does: measured **2.1.268** (stated
   here as read from `spikes/results/instrument-checks/RUNBOOK-instrument.txt`, never hard-coded — PIA-019;
   **corrected 2026-09-11: that file did not exist on this tree when this line was written — the directory held
   `README.md` and twelve `.jsonl` results and none by that name, and Checkpoint 0 line 4 created it for the first
   time at Step 0. The 2.1.268 figure was therefore hard-coded, which is the very PIA-019 defect this clause
   cites**) against HANDOFF's original
   **2.1.263**, with P7 having straddled **2.1.265** and 2026-09-09's measurement at **2.1.266**; **and, measured
   2026-09-11 at Step 0 and written to that file for the first time, 2.1.269 (Claude Code) — the move 2.1.268 →
   2.1.269 is recorded in `PROGRESS.md` §1/§3/§6 and in `bf5086a`, not worked around**; and the Rust
   toolchain moves from the spike programme's **1.97.1** to the pinned **1.98.1** (DECISIONS-2026-09-11 #7).
   **Record it; do not silently continue.** A version
   move mid-run splits a comparison (F20, F37 §7), and the rig's `doctor` printing `version >= floor : true` is a
   *floor* check, not a match. Any set that spans a version boundary is reported per-version or it is not reported.

3. **A platform error returns exit 0 / `subtype: success`.** This is **measured platform behaviour, not a
   theoretical risk** — F20: an agent card's `tools` allowlist silently disables `--json-schema`,
   `structured_output` is absent from the payload, and the process exits 0 reporting success. **Name it when it
   happens.** Any step that treats exit 0 as evidence of a completed dispatch, without checking that the structured
   payload is present, has measured nothing. The same shape is what makes `if (review && review.pass === false)`
   read absence as approval.

4. **A kill leaves a partial that resume does not restore losslessly.** Any step-output digest that differs across
   a `taskkill /T /F` boundary is a halt, not a retry. F38 is lossless 3/3 — at **one** step, inside the ~1 h TTL,
   with no within-experiment baseline because all three controls are unrun. A first divergence is a finding, and it
   is the finding Phase-1 exit criterion (3) exists to look for.

Two more that this tree has already produced and that cost time when unrecognised:

5. **A waiter that has been running longer than the job's timeout ceiling.** One `until grep` waiter spun 25
   minutes on a job that had already failed. If a wait has no `|exited with code` alternative, it is not a wait, it
   is a hang.

6. **A killed test run leaves a Windows handle no `Stop-Process` releases.** Parallel arms need **separate fixture
   trees**; one worktree per concurrent story is a requirement, not an optimisation. If a fixture tree will not
   delete, that is the handle, and reusing the tree corrupts the next arm.

Two more that this revision added, because the audit that produced it found both live in the previous one:

7. **A guard refuses and you are about to work around it.** The **thirteen** refusals in s4 §4.3 are the instrument
   (**recounted off the scripts on 2026-09-12 at Step 4's run: 6 in `score-p2.py`, 4 in `score-arm-d.py`, 3 label
   guards in `reconstruct-derived-from.py`. This line read "eleven" — five in the table plus six — and the two it
   missed are METHOD-P amendment 8's own guards, `score-arm-d.py:37` and `:81`, which is the mechanism Checkpoint
   4's 2026-09-12 repair turns on. That makes this the SECOND undercount of the same number, by the same route:
   not counting it off the script.** The first was **corrected 2026-09-11 by the runbook-wide checkpoint sweep**,
   when this line read "eight … three more" until the refusals were counted off the script, which is what s4 §4.3
   instructs: `grep -c 'sys.exit(' spikes/p2/score-p2.py` → 6, at :53, :72, :77, :79, :90 and :92, and that
   undercount silently dropped the BLINDNESS guard — the one s4 §4.3 calls "the one worth knowing" — out of the set
   Halt 7 declares to be the instrument), and
   s4 §4.3's list of what not to do is not exhaustive — the shape is. **Stop at any refusal and establish the
   guard's scope first** (golden constraint 9): is it too narrow, so the failure is passing through elsewhere, or
   too wide, so it is refusing valid work? Back-dating `labelled_at`, editing an `at` field, re-running an engine
   to move `first`, widening a facet regex, deleting a `--cases` guard, or disabling `score-p2.py`'s `labels_at`
   cross-check to score a re-stamped label file are all the same act. If the honest
   answer is that the guard is right and you were wrong, say so in the ADR and take the weaker result.

8. **A checkpoint cannot be run because a path it names does not exist.** That is a halt, not a skip, and it is
   *specifically* a halt when the checkpoint would go green anyway. `!`-negated `grep` over a missing directory
   exits 0. `grep -c` over a missing file exits 2. Both look like a pass in a terminal. **Those two are not the
   whole list. Added 2026-09-11 after the runbook-wide checkpoint sweep, which found six more shapes — five of
   them re-verified in a subshell on this tree that day, the sixth read off Step 0's own record rather than
   re-run, because re-running it truncates the file it is about:** (c) **a status-bearing command on the LEFT of
   a pipe** — a pipeline's status is its LAST stage's, so `{ printf 'test result: ok. 5 passed'; exit 101; } |
   grep -q 'ok'` exits **0** and the 101 is gone; (d) **an unguarded `grep -qF "$(cat X)"` with `X` absent** — the
   pattern is the empty string, which matches every line, so the check prints its success token and exits **0**
   having asserted nothing (this is s1 Checkpoint 1 line 6, the file's own PIA-019 regression check, which
   therefore cannot catch the defect it is named for); (e) **`git ls-files <path>` on an untracked path** — prints
   nothing and exits **0** (s1 Checkpoint 1 line 4); (f) **`… || echo "FAIL: …"` used as the guard** — `echo`
   succeeds, so the guard exits **0**, and a guard whose last statement is `false` rather than `exit 1` does not
   stop the lines after it either; (g) **any pass condition that is the ABSENCE of output** — `test -z "$(grep -r
   P missing/ 2>/dev/null)" && echo clean` prints `clean` and exits **0** over a tree that is not there; (h) **a
   command whose own output is the record** — `claude --version | tee <file>` opens its target `O_TRUNC` before
   reading stdin, so a failed re-run leaves a **0-byte** file and the line itself reports nothing (s0 Checkpoint 0
   line 4; line 5's `test -s` is what catches it, and it may not be dropped). **(c), (d) and (f) are the shapes a
   hardening pass itself introduces, so they are also the rule for writing one:** never put a status-bearing
   command left of a pipe — capture with `out=$(cmd 2>&1); rc=$?`, assert `rc` first, then grep the capture; prove
   a `$(…)` substitution non-empty before grepping with it; and always `|| { echo "FAIL: …"; exit 1; }`.
   **A checkpoint you could
   not run is a failed checkpoint** — record which one and why, and do not advance the step counter.
   **One carve-out, and it is narrow (PIA-003):** a checkpoint line marked **`[WILL EXIST]`** is **deferred, not
   failed** — record the deferral in `PROGRESS.md` against the stage the line names and re-run it there. The
   carve-out covers only lines so marked; a `[WILL EXIST]` line that exits non-zero because its binary is absent is
   not a pass either — its expectation is written as "exit != 0 AND stderr names the failing clause", and a missing
   binary satisfies neither half.

Three more that F39 produced, and that this revision added because it met all three in one scoring run:

9. **A criterion returns a result its registered n could not have decided.** Stop before you report it. METHOD-P
   §1.4's 0.95 precision bar is **not reachable at n=20 for any facet however good** — `ui` is 7/7 with a 95% lower
   bound of 0.65, and **73** all-correct positives are needed for a lower bound above 0.95 at z = 1.96 (92 with continuity correction) [measured, F39 Limits 1 as corrected 2026-09-11] —
   which is the third instance in the same pre-registration after P1's α rule (F36) and P7's criterion 1 (F37).
   **P2 §2.4's criteria 1 and 2 are the fourth**: **0 of 41** rows are labelled "should re-run", so recall has no
   denominator, criterion 1's recall half is **NOT ANSWERABLE** and criterion 2 **cannot fire either way**
   [measured, F40]. **Report it as powerless, and report a criterion with no positive instance as *not measurable*
   or *not answerable* — never as a pass or a failure, and never as `0.00`, `1.00` or `n/a`.** `data_migration`
   emits 0 positives on 20 items across all three sources; precision is undefined
   and a perfect method scores identically [measured, F39 §4]. **That and P2's recall are the same *shape* — a
   criterion with no denominator — and their numbers may never be mixed.** This is a result *about* the
   pre-registration and it
   is written into the finding's Limits. **It is not a licence to edit METHOD-P.** Golden constraint 10.

10. **A deriver's false positives cluster on the vocabulary of the thing being measured.** That is a correlated
    failure, not a tuning parameter, and it does not average out with more data. `auth` marks true on 14 of 20
    items and scores **0.00** — 0/11 [0.00, 0.26] on the 2026-09-09 scoring; 0/12 [0.00, 0.24] after the 2026-09-11 panel-file key fix scored all 140 cells — F39-DRAFT's Correction; ERRATA E-0089-2 — because `auth` is a substring of *authored*
    (40 hits), `oauth` of *coauthor* (8) and `token` matches a design token (36) [measured, F39 §2]. **Stop and
    read what the rule fired *inside* before you adjust the rule.** Adding word boundaries here removes the
    `coauthor` (8) and ARIA-`role` (2) hits and leaves the 40 *authored* and 36 design-`token` hits untouched — they
    are whole words at proper boundaries [measured, F39 §2; the registered prediction of S-FACET arm 2,
    `references/2026-09-09-change-classification.md` §6]. **Do not state that as a bare remainder**: an earlier
    revision wrote "removes 10 hits and leaves 76", and 76 is the surviving *authored* + design-`token` subset, not
    what is left of the 105 hits the table totals. If the collision is with the instrument's own vocabulary, the
    method is the finding.

11. **A scorer compares a value to the wrong shape and reports a confident number anyway.** The oracle emits
    `binary_change` as a JSON bool; the first scorer compared it to the string `"true"`, read every oracle positive
    as false, and scored the facet **0.00** where it is **0.60**. It was found and fixed **before** the result was
    reported [measured, F39 Limits 5], and the defect class — assuming a value's shape instead of checking it —
    occurred **three times** in one session's tooling. **A scorer that produces a clean-looking 0.00 is a scorer to
    read, not a result to publish.**

Two more that F40 produced, and that this revision added because the P2 scoring run met both:

12. **A criterion fires and prescribes a remedy the same measurement contraindicates.** Stop; do not apply the
    remedy, and do not record the firing without the contraindication. P2's criterion 1 fires on precision —
    coarse `derived_from` **0.00** [0.00, 0.26] over 11 emitted positives — and prescribes *"fine scope becomes
    mandatory for `L`-class activities"*, while the same run measures the **fine scope as the worse of the two**:
    on the 33 rows both scopes scored, coarse marks **5** `Stale` and fine **15**, with **10 rows flipping
    `Present` → `Stale` and none the other way** [measured, F40 §3]. **The design consequence is OPEN and needs a
    decision in its own record.** Golden constraint 11.

13. **A label set with no positive class, met on a corpus you did not build for the question.** Before you read a
    precision figure, count the positives. **0 of 41** P2 rows are "should re-run", so every `Stale` is a false
    positive **by construction** and `precision = 0.00` is a property of the label set, not a reading of the
    engine — **specificity** (coarse **0.73** [0.58, 0.84], fine **0.55** [0.38, 0.70]) is what the corpus
    measures. The cause is mechanical and recorded: DECISIONS #19 added the **card** as a perturbation candidate,
    and **11 of the 12 cases change nothing but the `stage`/`status`/`updated`/`gate_open` frontmatter the CLI
    rewrites on every advance** [measured, F40 §1]. **Report specificity, say the positives are absent, and do not
    lead with precision.** This is the artifact-level form of F39's failure class — an inference over a trace the
    instrument itself writes into (Halt 10) — and it is what makes the fine reconstruction fire on
    `implementation-report.md` and `report.md`, which the declared map does not connect to the card at all.

---
