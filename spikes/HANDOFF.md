# Handoff — 2026-09-04

Written at the close of the session that first ran E3. Everything decided is
committed; nothing lives only in conversation. A fresh session on the S-series
needs this file, `FINDINGS.md`, `METHOD.md` and `METHOD-E3.md`. **Corrected
2026-09-09: "and nothing else" stopped short of the P-series, which did not
exist when this line was written.** A session touching P1–P9 also needs
`METHOD-P.md`, `F32-DRAFT.md`–`F39-DRAFT.md` and `DECISIONS-2026-09-09.md`.

**Branch:** `spike/rust-inversion-layer0` · **Corpus pin:** redkiln @ `47faec9f`
(frozen) · **Instrument:** `claude` **2.1.263** (was 2.1.259 through 2026-09-08, and
2.1.258 before 2026-09-03 — F20; the rig treats 2.1.259 as a floor, see
DECISIONS-2026-09-09 #1; it moved again to **2.1.265** partway through P7 —
F37 §7), Windows 11 · **135 tests**, 0 clippy warnings. **Corrected 2026-09-09:** the AFK session's output **was** committed, at `e3b0bac`. This paragraph previously read "Working tree is NOT clean as of 2026-09-09: the AFK session's output is staged by `p9/COMMIT-2026-09-09.sh` and nothing was committed" — that was true when written and is now stale. It has since misled three separate pieces of work into planning a commit that had already happened. What actually remains untracked is the raw P1/P7/P8 driver logs, `spikes/results/p8-state/`, and the `spikes/runner/target-dev/` and `target-p8/` build directories, which `.gitignore` does not cover.

---

## Where the programme actually stands

| | status | headline |
| --- | --- | --- |
| **Layer 0** | done | step runner, typed failure, resume, isolation, provenance |
| **S0.1** gate FPR | done | **90% killed** of test-reachable mutants; 1 survivor |
| **S0.2** reconcile | done | **20.3%** of slices were rejected mid-flight |
| **S0.3** manifest | done | 23 of 23 items eligible |
| **E1** incl. TTL | done | TTL **~1 h**; commits cost **31%** of prefix; `--system-prompt` costs **0** |
| **E2** effort | **closed** | effort pins per dispatch; RFC R2 falsified |
| **E0** headroom | partial | sizing bound only; cache-cost gate **struck** (disclosed) |
| **E0** sizing | **measured** | Opus/xhigh: **+0.03** five_hour per S-SILENT dispatch, **~+0.077** per CLEAN |
| **E3** arm (a) | complete | 23/23 detection, 4/23 FP; **0/46 records had the parent review for itself** |
| **E3** cost | measured | in-session costs **1.72×** headless for the same detection ($97.39 vs $56.47) |
| **E0** variance | not started | `--repeat N` on the E3 rig; no item run twice yet |
| **E3** | **ANSWERED** | **kill criterion does NOT fire.** Detection tied 23/23; arms statistically indistinguishable (McNemar p=0.625) |
| **E4 / E5 / E6** | gated | behind E3 |
| **H2** suspension | done | **174/174 lossless** across a real process boundary |
| **H3** world drift | done | 7% overall; **3.6× more likely** on gates > 1 h |
| **H1** cost of a gate | **done** | **flat 18.6k above the TTL, 0 below** — same at 65 min and 49.7 h |
| **H4** digest fidelity | **done** | prose adds **nothing**: FULL 1.000 = CONTROL 1.000 vs digest |

### The four results that changed the thesis

1. **The cache TTL is ~1 hour** (F10), boundary between 45.1 and 61.6 minutes.
   The biggest threat to the RFC's cost case — that Claude Code had silently
   moved to a 5-minute TTL — is refuted for this configuration on this date.
2. **`--system-prompt` is the best cache strategy, not the worst** (F16). I had
   predicted RFC R6 and R4 conflict. They do not: replacing the default system
   prompt deletes the volatile git snapshot rather than relocating it. Steady
   state 1.000 across commits, versus 0.689 unmitigated.
3. **69% of the prefix is shared across directories** (F13). Charter §6.2's
   worktree-fragmentation concern is real but about a third the size the docs
   imply — a worktree costs 31%, not 100%.
4. **Two of the three gated experiments had unusable oracles as chartered**
   (F9, C1). E3's answer key did not exist; E4's ledger join was empty. Both
   were replaced before anything was spent.

---

## Next action: adjudicate the five rejections, then decide what E3 licenses

E3 is answered and the programme does not stop. **But the result is weaker than
the headline reads**, and the next steps follow from that rather than from the
numbers being good.

**1. The κ adjudication (METHOD-E3 §7) — SEVERITY only; the facts are settled.**
All six rejection records were replicated mechanically (**F24**): every claim is
TRUE, six for six, none a hallucination. What remains is whether each warranted
`changes-requested`, which is a judgement and is Ryan's. Four of the six are
"the test is vacuous" rather than "the code is broken", and a reviewer may
legitimately treat those as advisory.

If all are accepted as blocking, both arms go to **0/23 FP and MCC +1.000** —
that is the upper bound, not the result; the reported 4/23 and 2/23 are the
lower. **Either way CLEAN's ground truth is wrong for at least 5 of 23 items
(22%)**, so both measured FP rates are upper bounds on the true rate.

The five items rejected by at least one arm:

| item | (a) | (b) |
| --- | --- | --- |
| `fix-advance-write-lock-gate-scope` | REJECT | REJECT |
| `issue-63-advance-gate-diagnostic-truncated` | REJECT | approve |
| `issue-74-plan-briefs-dead-plan-anchor` | REJECT | approve |
| `issues-135-129-declared-boundary-parse` | REJECT | approve |
| `issue-146-147-boundary-window` | approve | REJECT |

CLEAN's ground truth is "this shipped and a review approved it", which is not
"this is defect-free". Until these are adjudicated **both false-positive rates
are provisional**, and with them both MCCs. `fix-advance-write-lock-gate-scope`
is the one both arms rejected, which makes it the best candidate for a real
missed defect. This must not be settled by the model that built the rig.

**2. Detection is at ceiling, so the primary metric discriminated nothing.**
23/23 versus 23/23 is not a tie that favours anyone — it is a metric with no
power at this difficulty. §9 already treats an all-arms *floor* as grounds for
"inconclusive"; the mirror case deserves the same treatment. A harder seed set
is the fix: novel defects rather than reverted fixes, or the spec withheld, or
the fix's file list not named in the prompt. **That is the highest-value next
experiment**, and it is cheap to specify because the rig exists.

**3. E0 per-item stability is still unmeasured.** `--repeat N` on any item.
23/23 is one sample of a stochastic process, and a variance estimate is owed
before anyone quotes the rate.

**4. Arm (c)** — the mitigation arm — was correctly not run: §8's reduction
order drops it unless (b) loses, and (b) did not lose.

### How to run dispatches (F22), unchanged and still load-bearing

- **Foreground under the tool limit: never killed.** All S-SILENT fits.
- **Anything reaching the harness background: killed**, at any age.
- **`Start-Process` detached: survives.** One ran 12,000s through ~14 killed
  watchers. Use it for CLEAN, and wait on the PID from the foreground.
- Arm (a) runs **slower** than arm (b) — its CLEAN dispatches hit the 2400s
  ceiling once. Pass `--timeout-secs 3600` for arm (a) CLEAN.

```
cd spikes/runner && cargo build     # cargo test/clippy do NOT rebuild the bin
python results/audit-e3.py results/e3.jsonl
```

## What is genuinely unresolved

1. **Does a cache read draw down the subscription window?** Unanswerable at the
   platform's 0.01 resolution: 1.9M tokens moved it by less than one tick, and
   the assistant shares the account. Gate struck, not answered.
2. **E0's per-item stability** — no reviewer has been run twice on one item.
   The rig supports it (`--repeat N`) and E3's records serve both purposes.
3. **The in-session side of the cache asymmetry.** The RFC's claim that
   subagents get "5 minutes, always" is still only documentation. E3 arm (a) is
   the first chance to observe it.
4. **The human gate** — RFC §7 calls it the unoccupied market gap. **Now
   planned**: `spikes/METHOD-H.md` (pre-registration #3, not approved to run).
   H2 (resume determinism across a process boundary) and H3 (world drift during
   a gate) cost nothing in headroom and can run alongside E3; H1 and H4 wait.
   F18 already establishes the shape: ~half of all gates outlive the cache TTL.
5. **8 of 23 items sit outside S0.1** because their fixes touch only
   `workflows/`, not `src/`. A third of the corpus is unmeasured by the gate FPR.
6. **Arm (a) is unbuilt**, and it is the whole comparison. Everything E3 has
   produced so far is one arm against ground truth.
7. **Equivalent-seed triage (METHOD-E3 §10) has not been applied.** At 23/23 it
   cannot lower detection, but the exclusion list is still owed.
8. **The two CLEAN false positives are unadjudicated**, and the co-primary rate
   depends on how they land. Both are specific and mechanism-level:
   `fix-advance-write-lock-gate-scope` (two regressions claimed confirmed by
   differential probe) and `issue-146-147-boundary-window` (AC-015's test is
   vacuous — its regex passes on pre-existing prose). CLEAN's ground truth is
   "this shipped and was approved", which is not "this is defect-free". If
   either finding is real, the FPR is overstated. This needs **Ryan's blind
   annotation** (§7 κ) — it must not be settled by the model that built the rig.
9. **No item has been run twice.** E0's per-item stability (`--repeat N`) is
   still unmeasured, so 23/23 is one sample of a stochastic process.
10. **E3's primary metric saturated.** Both arms detect everything, so the
    design cannot rank them; the whole comparison fell to an underpowered
    co-primary FPR (McNemar p=0.625). E3 answers "does headless review
    degrade?" with "not detectably, on defects this easy" — and nothing
    stronger. E4/E5/E6 inherit that limit.

---

## Traps that cost time here — worth not repeating

- **`cargo test` and `cargo clippy` do not rebuild the binary.** I ran a stale
  binary four times and twice concluded code was broken when it had not
  compiled. Always `cargo build` before running.
- **Scripted edits need an assertion.** Two silent `str.replace` no-ops were
  taken for behaviour changes. Assert the anchor exists.
- **`until grep` waiters spin forever if the job dies** — every marker must have
  a failure alternative (`grep -qE "MARKER|exited with code"`). One waiter span
  25 minutes on a job that had already failed.
- **Background jobs get killed unpredictably** (~13 min survived, ~35 and ~55
  did not). Nothing may depend on one process surviving; resume everywhere.
- **Guards need the right scope.** Two guards I wrote to protect data were
  scoped wrong and blocked or voided valid work.
- **A rig that compiles and passes its unit tests has not been run.** Three E3
  defects survived "built and validated" (F20, and the abbreviated-SHA fetch):
  `--json-schema` silently inert under a tool-restricted card, a `\\?\` path
  handed to `git fetch`, and `git fetch <abbrev-sha>` failing where only the
  one path that happened to call `rev-parse` first worked. None was reachable
  from a unit test, because none of them touch the filesystem or the CLI.
  **Validate one item before every set**, and prefer a haiku probe to an Opus
  one when the question is "does this flag work at all".
- **A default was written for the fast condition.** The 900s timeout fitted
  S-SILENT and cost a CLEAN record. When two conditions differ in shape, check
  the ceiling against the slow one.

---

## P-series — the process-catalog spikes (added 2026-09-07)

A second spike lineage, independent of the S-series rig except where noted.
It serves `references/2026-09-07-process-catalog-proposal.md` (decisions
PC1–PC8) and `references/2026-09-07-activity-graph-schema.md` (§9a/§9b
amendments), both folded into the architecture proposal as D20–D22.

| spike | state | where |
| --- | --- | --- |
| **P9** authoring ergonomics | **run** — median 56.5 lines, 13 real lint catches / 34 false, fragment at threshold | `METHOD-P9.md`, `F32-DRAFT.md`, `p9/catalog/`, `p9/lint.py` (v1, frozen) |
| **P9b** second-author check | **run** — median 73 lines, 27 ambiguities (20 new), "amendments will fall" falsified at n=3 | `F33-DRAFT.md`, `p9/catalog-b/`, `p9/results-b-*.txt` |
| **P6 authoring** (third author, 90 activities) | **run** — median 53 lines, 0.11 first-run lint errors/file (author read the lint first), 30 ambiguities (25 new, catalog-level), amendments 26–43 proposed | `F34-DRAFT.md`, `p9/catalog/` (101), `p9/results-v2-batch-*.txt`, `p9/lines-v0.txt` |
| **P6** catalog completeness | **run** — 23.5% first run → 11.4% (pre-decision, two-band, run3) → **0.3% after decisions 1–10 (three-band, run8)**; mixed = pure Node; dead 7/102; ties 0; all §3.4 criteria clear; pack taggable after Ryan's review | `F35-DRAFT.md`, `p9/lattice.py`, `p9/packs/` (8 + 4 supplementary), `p9/results-p6-run*.txt`, `p9/reach-check.py` |
| **P1** classification stability / facet derivability | **arm D run** (640 records, 20 items x 4 path sources x 8 facets) and **arm L run twice** (200 + 200 dispatches, 0 void, $21.37). Modal `R` identical across 10 draws on 5/20 · 3/20 (v1) and 2/20 · 2/20 (v2) vs an 80% line — **kill fires 4/4**. §1.4's anchor rewrite did not repair it; alpha itself drifts up to -0.168 on **untouched** anchors, so the 0.6 threshold is not decidable here. **Arm L's human agreement panel is still unwritten** (§0 rule 1; `results/p1-arm-l-labels-RYAN-TODO.md` is a blank template) — **arm D's panel was replaced by METHOD-P amendment 9 and arm D is SCORED**, see the *P1 arm D* row below | `F36-DRAFT.md`, `METHOD-P.md` §1, `p1/analyse-arm-l.py` |
| **P2** short-circuit precision/recall | **SCORED, 2026-09-10** — 41 labels, blind and **timestamp-verified** (the one arm whose blindness is checked, not attested), $0. **0 of 41 rows are "should re-run"**, so recall has no denominator and **two of three kill criteria are not answerable**. What is measurable is the false-alarm rate: specificity **coarse 0.73** [0.58, 0.84], **fine 0.55** [0.38, 0.70]; on the 33 rows both scopes score, **10 flip Present→Stale from coarse to fine and none the other way**. Cause: **11 of 12 perturbations change only YAML frontmatter** (`stage`, `status`, `updated`, `gate_open`), so the fine reconstruction infers a dependency from the CLI's own bookkeeping. Criterion 1's precision half fires; its prescribed remedy (make fine scope mandatory) is contraindicated. Advisory half still unrun | `F40-DRAFT.md`, `p2/score-p2.py`, `results/p2-engine.jsonl`, `results/p2-labels-ryan.md` |
| **P7** encapsulated quality cost vs catch (absorbs P4) | **COMPLETE** — 61 graph runs, 0 void, $131.62 of $200; `optional` and `recommended` both at 10 items x 3 repeats (`mandatory` dropped, #33). Block costs **2.64x** and changes the root cause on **1 item in 10**; its policy check was **false on 33 of 33** runs where an adversary ran, so the verdict channel is a constant function of the arm. Deviation channel used **0** times. §4.4 criterion 1 fires but has **no power** at n=3 | `F37-DRAFT.md`, `p7/analyse-p7.py` |
| **P8** gate inside a graph | **PARTIAL** — 3 resume cycles of a pre-registered 27+3, $3.01. Resume is **lossless 3/3** across a real `taskkill /T /F`, and costs **0.45-0.78x** F28, so §5.4's first two criteria do not fire. The third is **not answerable**: the gate never fired on its own, so arrivals were forced (#36). §5.2's fixture design does not work — the investigator reproduces from the symptom. 65-min / 24-h arms and controls **not run** | `F38-DRAFT.md`, `plans/p8-24h-resume.md` |
| **P1 arm D** scored | **run, 2026-09-09** — panel-adjudicated oracle under METHOD-P amendment 9. **No facet clears §1.4's 0.95 bar.** `auth` **0.00** over 11 positives (every one false: `auth` ⊂ *authored*, `oauth` ⊂ *coauthor*, `token` = a design token); `data_migration` emits 0 positives and is **not measurable**; `ui` 1.00 on 7/7 but CI [0.65,1.00]. `commits` beats `docs` on every facet. 140 truth cells: 60 oracle, 48 unanimous panel, **32 Ryan-adjudicated** (15 of 32 against the script) | `F39-DRAFT.md`, `p1/score-arm-d-panel.py`, `p1/oracle-mechanical.py`, `references/2026-09-09-change-classification.md` |
| P3 | cut (PC1 decided on documented grounds) | proposal §6.1 |
| P5 | reclassified as Phase 1's acceptance test | proposal §6.1 |

**Rig order once the S-series releases it:** P1 arm L → P7 → P8. P7, P8 and P6
gate "design final". F30 is not assigned anywhere I can find; F32/F33 were
taken for P9/P9b to avoid a collision — reconcile if F30 is reserved elsewhere.
**As of 2026-09-09 that order has been run**: P1 arm L, then P7 (complete), then
P8 (partial). F36/F37/F38 are drafted, and **F39 scores P1 arm D** (offline, $0, 2026-09-09 —
it is not part of the rig order because it spends no model budget). Of the three that gate "design final",
P6 and P7 now have results and **P8 does not** — its 65-min and 24-h arms and
its 3 controls are unrun, and the one criterion that would reverse a schema
decision (§9b A8) is not answerable from what exists.

**Session 2026-09-08/09 (AFK) — numbers first:** 400 P1 arm-L dispatches across two instruments, 0 void, $21.37; 61 P7 graph runs, 0 void, $131.62; 3 P8 resume cycles, $3.01; 640 arm-D records and 77 P2 artifacts offline at $0. **Total $156.00 of the $300 cap.** Modal `R(stage, ctx)` identical across ten draws on **5/20, 3/20, 2/20, 2/20** items against an 80% line. P7: adversary block **2.64x** cost, changes the root cause on **1 item in 10**, policy check false on **33 of 33**. P8: resume **lossless 3/3**, **0.45-0.78x** F28. Deviation channel offered 71 times, used **0**. 38 decisions logged.

**What each lane says.** **F36 (P1 arm L)** — the LLM-emitted context vector is not stable enough to drive a deterministic selection engine, and §1.4's prescribed remedy does not repair it. The anchor rewrite left `blast_radius` unmoved (0.488 -> 0.473, so it becomes human-supplied on §1.4's own rule) and made `external_dep` **constant** rather than reliable (200/200 `false`) — unanimity by collapse, which looks healthy and is worse than noise. The finding nobody registered: three axes whose anchors were **byte-identical between the two runs** drifted by up to **-0.168**, which is larger than the effect the alpha<0.6 threshold is meant to detect, and carried `size` across the kill line without an anchor change. **"alpha < 0.6" is not a decidable predicate on 20 items x 10 draws.** What survives is the modal-`R` criterion: it fires 4 times out of 4, needs no inter-rater statistic, and is what the engine actually feels. Act on that one.

**F37 (P7)** — the encapsulated `adversary -> policy` block costs 2.64x a single dispatch and changes the answer on one item in ten. Its **policy check is a constant function of the arm**: 33 of 33 adversary dispatches produced at least one blocking-class finding, so the check was false every time it meaningfully ran and vacuously true otherwise, and in the `recommended` arm it is also inert (loop-back is `mandatory`-only, so 30 `false` verdicts produced 0 loop-backs). §4.4's first criterion fires — but at n=3 a Wilson interval is [0.44, 1.00] or [0.00, 0.56], so only a 3/3-vs-0/3 separation could ever fail to overlap. **The run establishes the criterion's power, not its verdict; do not set the `quality` shorthand's default from it.** The adversary itself works: 188 findings over 33 dispatches, 78% in classes requiring it to have run something, and self-evidencing even while `confirm` was broken.

**F38 (P8, partial)** — serialise/kill/reload is **lossless** at mid-graph granularity, 3/3, across a `taskkill /T /F` of a process holding state in memory, verified on both the step digests and the state files. A mid-graph resume costs **less** than a stage gate (0.45-0.78x F28), so the "plus the graph's replay" term §5.4 assumed does not exist. §5.4's third criterion is **not answerable**: the gate never fired on its own, so every arrival was forced, and the mark was minted by the driver that implements A8 by construction.

**F39 (P1 arm D)** — the *derived* half of the context vector is not a rescue for the LLM-emitted half F36 killed; it is measurably wrong in its own right. Over **140 truth cells** at pin `47faec9f`, **no derived facet clears §1.4's 0.95 bar**: `ui` 1.00 [0.65, 1.00], `toolchain` 0.73 [0.48, 0.89], `change` 0.72 [0.54, 0.85], `binary_change` 0.60 [0.23, 0.88], `flag_exists` 0.00 [0.00, 0.66], and **`auth` 0.00 [0.00, 0.26] over eleven positives every one of which is false** — the lexical rule carries no signal on this corpus, because `auth` is a substring of *authored* (40 of its 43 hits), `oauth` of *coauthor* (all 8), and `token` matches a design token (36 of 44). `data_migration` emits **0 positives on 20 items** and is **not measurable** — the criterion has no denominator, and it is reported in those words rather than as a failure. The finding nobody registered: **the ≥ 0.95 clause is not reachable at n = 20 for any facet however good** — `ui` is 7/7 and its lower bound is still 0.65, and roughly 60 positives are needed for a lower bound above 0.95, so a perfect deriver scores identically to this one. That is the third pre-registered criterion this programme's own sample size cannot decide, after P1's alpha rule (F36) and P7's criterion 1 (F37); it is reported in F39's Limits and **not** patched into METHOD-P. Two things fall out that are decisions, not measurements: `commits` beats `docs` on every facet and **`union` is worse than `commits` alone** (`toolchain` 1.00 vs 0.73, `change` 0.80 vs 0.72), because unioning a precise source with an imprecise one imports the imprecision — registered as an arm of **S-FACET**, not decided; and **4 of the 20 items changed no code at all**, on which the script emitted 16 confident booleans that the panel marked `unknown`. Oracle composition, so it is never quoted as a human panel: 60 cells a deterministic oracle over git metadata, 48 unanimous panel, **32 Ryan-adjudicated blind — 15 of the 32 differ from the script**; reported **panel-adjudicated** under METHOD-P amendment 9. Consumer: **ADR-0089** (accepted 2026-09-09) and `references/2026-09-09-change-classification.md` §6. **ADR-0079 is untouched and stays `proposed`** — F39 measures a *derived* vector and 0079 asks about an *LLM-emitted* one.

**Two findings that are about the pre-registration, not the rig.** §5.2's fixture — "3 items with the repro steps removed **so the gate fires**" — does not work: three redactions of increasing severity and two selection rules, and the investigator reproduced from the symptom every time. `issue-74` was localised **0 times in 6** Opus attempts in P7 and reproduced on the **first** haiku attempt in P8; reproducing a symptom and localising its cause are different tasks, and §5.2 assumed the harder gates the easier. And, from answering why P7's `mandatory` arm is unreachable: at `rigor = mandatory` with `has_repro = true` the engine selects **`bisect`**, which has **no adversary and no policy step** — escalating rigor **drops the quality block**. That is a candidate defect in F34 amendment 26's ceiling design, not in `root-cause`. Nothing was edited; the catalog is byte-identical to HEAD.

**Instrument, and a lineage break inside the primary comparison.** `claude` **2.1.263**, not the pinned 2.1.259 — the rig's own `doctor` treats 2.1.259 as a floor and prints `version >= floor : true` (#1), so the pin is moved here. It then moved **again to 2.1.265 during P7**, catching exactly `recommended`'s r3 draw on all ten items while `optional` ran wholly on 2.1.263. F37 §7 reports the per-repeat split so it can be inspected; no effect is detectable, and the arms are nonetheless not instrument-matched.

**Three defects worth carrying forward.** (1) `confirm` ran the **whole suite** for its first 26 of 68 invocations — `pnpm test -- --run <file>` expanded to `vitest run --run <file>` and the filter did not survive pnpm's forwarding; `npx vitest run <file>` is 1 file / 6 tests / 15.6 s against a >900 s full suite. Two earlier explanations (vitest fan-out, step timeout) were treating a symptom and are retired in F37 §8.2. (2) A P8 emit record claimed `second_arrival_minted_a_new_mark: false` where the gate had simply **not been re-reached** — read literally that reverses §9b's A8 on no evidence (#37). (3) Parallel arms need separate fixture trees, and a killed vitest leaves a Windows handle that no `Stop-Process` releases (#26, #30).

**What Ryan does next, in order.** (1) Read `spikes/DECISIONS-2026-09-09.md`, starting with its "decisions Ryan should reverse or confirm first" table — it leads on the `human_authored` remap because that changes what §2.3's advisory fraction measures. (2) Fill the **one** label file still owed — `spikes/results/p1-arm-l-labels-RYAN-TODO.md` — **P2's was filled on 2026-09-10 (41 of 41, labeller Ryan Britton) and scored (F40), so it is no longer owed and `spikes/results/p2-labels-ryan.md` must NOT be overwritten from its blank** — **from the item alone, before opening any engine output**, and stamp `labelled_at` in UTC `...Z`; the scorers compare timestamps lexicographically and a local-offset stamp mis-compares silently. **Corrected 2026-09-09: this said three, and arm D is no longer one of them.** METHOD-P amendment 9 replaced arm D's human-panel oracle, so `p1-arm-d-labels-RYAN-TODO.md` and the populated worksheet `p1-arm-d-labels-ryan.md` are **deliberately retained unfilled** as the record of the route not taken — do not fill either to "finish" it, and do not let a script overwrite the worksheet with its blank. (3) Run the **P2** scoring script, which refuses until (2) is done: `reconstruct-derived-from.py --phase engine --cases spikes/results/p2-cases.jsonl` (pass `--cases` explicitly). **Arm D is already scored** — by `spikes/p1/score-arm-d-panel.py`, which takes no arguments and reads five constant paths from the repo root, over 140 truth cells at pin `47faec9f` for $0 [measured, F39]. `spikes/p1/score-arm-d.py` is the pre-amendment scorer over the same engine records and is **not** what F39 reports; RUNBOOK §3b keeps it, and its `LEAVES THE MODEL (>=0.95)` string is a literal in the source, not an outcome — no facet printed it. **And a gap nothing registers: arm L's labels have no scorer.** `p1/analyse-arm-l.py` computes only §1.3's unlabelled half and says so in its own header — "`score-arm-d.py`'s counterpart for arm L does not exist yet and this script deliberately stops short of it". So filling `p1-arm-l-labels-RYAN-TODO.md` discharges §1.2's panel but currently feeds nothing; the consumer has to be written before the labels are worth the day they cost. That is an unknown with no named consumer (ADR-0012) and it is recorded here rather than left implicit. (4) `spikes/results/p7-deviation-labels-RYAN-TODO.md` has **no rows** — the channel was never used; nothing to adjudicate. (5) Decide the `bisect` question above. (6) P8's remaining arms: `spikes/plans/p8-24h-resume.md` carries the commands and the precondition to check first — and note that the 24 h arm has **not** been started, deliberately, because until #36 the gate could not be reached at all. Commit script: `p9/COMMIT-2026-09-09.sh` — **corrected 2026-09-09: it HAS been run; do not re-run it.** This line previously read "(nothing was committed; HEAD is still ab58733)", which was true when written and is the same stale claim the header paragraph records as having misled three separate pieces of work. `git log` shows the AFK session's output committed at **`e3b0bac`**, and seven commits since, HEAD at **`ce04058`** [measured, `git log --oneline e3b0bac..HEAD`; RUNBOOK.md §"HEAD is `ce04058`, not `e3b0bac`"]. What is genuinely uncommitted is listed in the header paragraph: the raw P1/P7/P8 driver logs, `spikes/results/p8-state/`, and the `spikes/runner/target-dev/` and `target-p8/` build directories.

**Session 2026-09-08, later (decisions applied — Ryan authorized the METHOD-P / schema edits):** ten decisions in `p9/DECISIONS-2026-09-08.md`; schema §9c (amendments 26–43) and METHOD-P amendments 6–7 applied; registries (`artifacts.toml` new, typed `[verb]`, `recent_deploy`), lint v3 (`lint-v2.py` frozen), 15 `family` keys + `max_rigor` on four families, 12 composing emits (`record` gone), `implement-ue` (102 activities), 46 version bumps, `lattice.py` v2 (`lattice-v1.py` frozen; `--validate`; 3 bands + unknown sweep; keyed instances). P6 run4→run9: **0.3% unsatisfiable (prereg-8, pack-default), 0.2% all-12, text slices 0.0%, mixed = node, dead 6/102 (all deviation-only), ties 0** — every §3.4 criterion clears (F35 §1.6). Ryan reviewed the 59 bumps, the four planning stages and R3/R7 (DECISIONS §Review) — the pack is taggable. Remaining: (1) tag the pack; (3) the `optional`-override mode is not a real reading (21%) — decide whether METHOD-P keeps it; (4) the `quality`/`green` shorthands wait for P7; (5) P1/P2 scripts still unrun. Commit script: `p9/COMMIT-2026-09-08.sh` (updated).

**Session 2026-09-08 (P6 authoring + lattice) — what to do next (superseded by the paragraph above where they overlap):** (1) decide F34 amendments 26 (rank semantics) and 27 (producer scope + roots) before anything else — F35's numbers depend on `lattice.py`'s interpretations I1/I2/I5 and move if those change; (2) the four remaining Node-slice gaps are one tag each (F35 §1.2); the binary-only UE slice needs an implement-stage UE producer that was out of scope; (3) `P6` used four supplementary packs (chore, tech-debt, security-vuln, docs) — accept or strike them in METHOD-P §3.2 (F35 §3.1); (4) smoke-run the P1/P2 scripts against `D:/repos/redkiln` @ `47faec9f` — they are untested; (5) seven one-line edits were made to P9 catalog files for coherence (F34 §0), all commented in place; (6) `catalog-b/` untouched; `lint-v1.py` untouched; `METHOD-P.md` untouched. Commit script: `p9/COMMIT-2026-09-08.sh`.

**Traps specific to P:** `p9/lint-v1.py` is frozen so `results-run*.txt` and
`results-b-*.txt` replay; `p9/lint.py` is v2 (§9 + §9a + §9b + `registry.toml`).
`p9/catalog-b/` is the second author's record and fails v2 by design
(`results-v2-catalog-b.txt`, 15 errors) — do not "fix" it. `p9/catalog/` is the
seed of the v0 pack and is v2-clean (`results-v2-catalog-run2.txt`). Human labels for P1/P2 are written
before unblinding (METHOD-P §0 rule 1) — do not run the engine first.

## Pre-registration status

`METHOD.md` covers Stage 0, E0, E1, E2; `METHOD-E3.md` covers E3/E6;
`METHOD-H.md` covers the human gate (planned, unapproved); `METHOD-P.md` covers
the P-series and owns amendments 6–9. **Nine** amendments, each disclosed in the
document and its commit. **Corrected 2026-09-09: this said "seven … none made
after seeing the data it would affect", which its own items 6 and 7 already
contradicted, and which stopped two amendments short of METHOD-P §6.**
Amendments 1–5 were made before the data they affect; 6 and 7 are post-data and
additive, changing what is *reported* and no kill criterion; 8 and 9 are
post-data and each **weakens an oracle**, disclosed as such, with §1.4's
thresholds untouched [documented, METHOD-P §6]:

1. E3/E6's primary metric replaced — the answer key did not exist (F9).
2. E3's input set 22 → 23 — my parser, not the criteria (S0.3).
3. E0's cache-cost gate struck — not settleable with this instrument (F14).
4. METHOD-E3 §6 — `StructuredOutput` added to the card's tool set. An
   **instrument** fix, not a metric change: without it `--json-schema` is
   silently inert and every record is void (F20). Made when the only E3
   dispatch in existence was void.
5. METHOD-E3 §5 — CLEAN and S-SILENT built to the same shape (one-commit
   shallow repo, one commit apart). Also instrument, not metric: CLEAN's
   false-positive rate is co-primary with S-SILENT's detection rate and the two
   are comparable only when measured in the same place. Made before any CLEAN
   dispatch.
6. METHOD-P §3.2 — packs: one per kind in `facets.toml` (12, not 8). **Post-data**, additive: the eight-pack
   reading stays primary in F35. Applied 2026-09-08 by Claude on Ryan's authorization.
7. METHOD-P §3.2 — lattice banded {low, med, high} over the 4 registered axes + an `unknown` sweep. **Post-data**,
   additive: the two-band run (`results-p6-run3.txt`) stays the pre-registered reading.
8. METHOD-P §0 rule 1 (2026-09-09) — the arm-D blindness check is **attested, not timestamped**. Made after the
   arm-D engine output existed and, on the labeller's own attestation, before it had been seen; it substitutes an
   attestation for an expired chronology check **on P1 arm D alone**, is strictly weaker than the check it
   replaces, and changes no kill criterion. Every result scored under it is reported **attested-blind**, never as
   timestamp-verified.
9. METHOD-P §0 rule 1 (2026-09-09) — arm D's **human-panel oracle is replaced** by a deterministic oracle on the
   mechanical facets plus an adversarially-verified model panel on the semantic ones, with Ryan adjudicating every
   contested cell. Weaker than the pre-registered oracle — panel and regex can share a lexical blind spot — and
   richer in what the panel may read. Results are reported **panel-adjudicated**, never as human-panel (F39).

**The RFC amendment is written**: `references/2026-09-02-rfc-amendment.md`
(charter §14.4). It records R2 falsified, the R4/R6 conflict dissolved and
inverted, §10's pivotal unknown resolved, the cache-TTL premise confirmed,
§6.2's worktree conflict repriced to ~⅓, the multi-agent citation corrected, and
three new risks the RFC did not anticipate. Its closing point is the one that
matters: **every result so far is cost and mechanism; the quality question the
RFC says is decisive has not been tested.**

**Charter §14 deliverables are COMPLETE.** §14.3 is
`references/conformance-2026-09-04-scorecard.md` (Run 3, measured — filed in the
spike repo because redkiln is frozen; move it to
`redkiln/references/conformance/` when the freeze lifts). §14.4 is the RFC
amendment, updated 2026-09-04 with the E3 result. §14.5 is
`references/2026-09-04-findings-for-product.md`.

**Design-implications note, published 2026-09-04:**
<https://claude.ai/code/artifact/dbe80d02-c642-4b53-98f0-257305c768e0>
("Bet 1, Measured"). Ties every finding back to the RFC's §2 component table —
six components, each with a typed state and a citation — plus the three design
requirements the RFC does not contain. **Private until shared from the page's
share menu.** It is a rendering of what is already in `FINDINGS.md`, the RFC
amendment and the Run 3 scorecard; those files remain the source of truth, and
the artifact must be republished from them if they change.

**Run 3's headline, because it needs acting on:** `forge-fix.js`'s review
consumer is `if (review && review.pass === false)`, which treats an ABSENT
verdict as approval — and F20 is a measured mechanism that produces exactly
that (a tool-restricted card plus a demanded schema yields prose, silently, on
the CLI path). Whether the Workflow runtime shares the defect is UNVERIFIED and
is one dispatch to settle. Fix the consumer either way:
`if (!review || review.pass !== true)`.

**§14.5 is DONE** — `references/2026-09-04-findings-for-product.md`. It carries
the four that were expected plus nine more, split by audience: Part A is Claude
Code platform behaviour (silent `--json-schema` failure under a tool-restricted
card; `--allowed-tools` not restricting the main turn; headroom readable only via
an undocumented event with the binding sub-limit invisible; platform errors
returning exit 0 / `subtype: success`; in-session subagents on a 1h not 5m cache;
background-process kills), Part B is redkiln (per-AC mutation evidence, with a
staged remediation package; a review gate that detects reliably but blocks
unreliably; no affected-area test selection; the empty ledger join; the S0.1
survivor the reviewer caught 4/4 times; a test helper that drops the field which
would have caught a real regression).
