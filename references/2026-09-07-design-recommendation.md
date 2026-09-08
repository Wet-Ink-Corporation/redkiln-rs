# Redkiln-RS — what the data says the specification must be

**Date:** 2026-09-07 · **Status:** Recommendation. Feeds proposal rev. 4 (`2026-09-05-architecture-proposal.md`) and the Phase 1 ADRs.
**Scope:** every spike run in this session — S-VAR, S-TYPED (with its mechanical adjudication), S-H1b, S-WT, S-MIG, and the H1 correction — read against the programme that preceded them (F1–F28) and against prior art (`research/2026-09-06-grounding-s-results.md`, plus the S-TYPED grounding pass in §6 here).
**Evidence discipline:** every number below is from a results file or a FINDINGS entry; every prior-art claim carries a source tag ([peer] / [preprint] / [1P] first-party / [blog]). Where a result is one draw, one repo, or one operator, it says so. Unrun spikes are listed as unrun, not assumed.

---

## 0. The verdict in one paragraph

The architecture survives the data with one part rebuilt and two parts repriced. **Rebuilt:** the review gate. A reviewer whose verdict flips on 3 of 10 draws for a reproduced regression (S-VAR) cannot hold a threshold, and when the verdict is removed and the reviewer is asked for typed, runnable evidence instead, it surfaces confirmed unpinned criteria on **12 of 23 shipped, human-approved fixes** (S-TYPED, adjudicated 14/14) at ceiling detection and ~2× the cost. The gate therefore becomes *reviewer proposes, machine confirms*: evidence class computed in code from what the reviewer ran, the flip criterion re-executed before anything blocks, and every block and override logged. **Repriced:** a human gate costs at most one prefix rewrite (18.6–29.7k tokens), not zero and not proportional to duration; and a static card prompt buys commit- and worktree-invariance at 1.000 but forfeits the account-wide cache subsidy across time. **Unchanged:** git as truth with a rebuildable event log, mark-and-resume gates, typed artifacts with markdown as a rendering (98% of spec sections lift), the five ports, and inversion itself. Prior art agrees with each of these moves and, in three places, had already solved a problem we had not yet noticed.

---

## 1. Scorecard — every spike in this session

| spike | question | result | registered prediction | status |
| --- | --- | --- | --- | --- |
| **S-VAR** (60 Opus draws, $177, 16.2 h) | per-item verdict flip rate on 6 CLEAN items, n=10 | reject rates 0.70 (Reproduced) · 0.60 / 0.30 / 0.20 (MutationSurvived) · 0.10 (NarrowedPin) · 0.00 (control); 19/19 rejections cited the F24 file; 0 off-target | Reproduced 9–10/10; 146-147 at 0.4–0.8; issue-63 at 0.3–0.7; control ≥9/10 | **ordering held, levels falsified** — §1.5's architecture-changing branch fired |
| **S-TYPED** (51 valid Opus dispatches, $112) | does a closed `Findings` schema with no verdict cost detection or raise FP? | S-SILENT 23/23, S-SUBTLE 5/5 (= prose); conformance 52/52; CLEAN blocks 12/23 strict, 9/23 lenient vs prose 2/23; contradiction secondary 4/5; repro fill 147/147 on blocking-grade findings (253/253 on all); cost +40% (S-SUBTLE), +71% (CLEAN), +101% (S-SILENT); wall 1.2× / 1.6× / 2.5× | detection ≥22/23 ✓; F24 re-found **and classified as F24 did** ≥4/5 ✗ (3/5 — `issue-74` came back advisory-only, `issue-63` as mutation-survived not narrowed-pin); contradiction secondary ≥4/5 ✓; strict FP 3–5/23 ✗; cost ±15% ✗; conformance ≥95% ✓ | **detection held; "false positives" were true findings (§2); classification and cost predictions wrong** |
| **S-TYPED adjudication** (zero model cost) | are the 8 newly blocked CLEAN items real? | **14/14 findings CONFIRMED** by applying the reviewer's own mutation and re-running the named suite; in 2 cases the full 1,890–1,907-test suite stays green | — | **CLEAN ground truth is wrong for ≥12/23 (52%; 13/23 counting F24's `issue-74`, which this draw reported advisory-only)**, up from F24's 5/23 |
| **S-H1b** (haiku) | does a static `--system-prompt` blunt time decay? | 1.5 min: share 1.000; **66.1 min: share 0.000, creation 29,619** — the whole prefix; 24 h read not taken | 65-min creation 4–10k | **falsified in the opposite direction**; replicates the F28 correction taken the day before (29,680 at 66.8 min) |
| **S-WT** (haiku) | is worktree fan-out cache-free under a static prompt? | w1 0.807 (first write), **w2–w10 all 1.000** | ≥ 0.95 | **held** |
| **S-MIG** (zero model cost, 1,312 files / 6,877 sections) | what lifts into the proposed types? | spec sections 98% lift; frontmatter 100% on 209 cards / 153 specs; ACs 88% strict (12% lack GWT or a verification); gates free-form in 25% of reports / 83% of reviews; TDD mutation columns in 0/156; `plan.md` unfilled 110/110 | ACs ≥95% ✗; anchors/boundary ≥90% ✓; TDD <80% ✗ (91%); kb bodies opaque ✓ | **2 of 4 held; the misses each name a type decision** |
| **H1 correction** (F28, 2026-09-05) | was the "flat 18.6k step" pure time decay? | The *loss* is real time decay (TTL expiry); the *flatness* was subsidy — the surviving ~17.5k of the default prompt's head was kept warm by other dispatches on the account, not durable on its own; `--system-prompt` at 66.8 min: 29,680 created | — | **mechanism corrected; practical bound stands: ≤ one prefix rewrite per resume** |
| S-H1c, S-BIN, h1b-24h read | append-prompt route; binary-artifact review; 24 h decay | **not run** | — | carried as open (§5) |

Two instrument notes that matter for reading the table. `audit-s.py`'s on-target column is 0-by-construction on CLEAN (no reverted files); the 19/19 figure is from `s-var-citations.py` against targets pre-registered before S-VAR. And ten S-VAR draws taken at a 2,400 s ceiling were quarantined (`s-var-2400.jsonl`) because the ceiling censored slow draws preferentially on exactly the items whose band the decision rule turns on; the 60 reported draws are all at 3,600 s.

---

## 2. The finding that rebuilds the review gate

Three measurements, in order, and what each rules out.

**S-VAR: the reviewer detects reliably and decides unreliably.** The one item with a live, end-to-end reproduced behavioural regression (`fix-advance-write-lock-gate-scope`, F24) was approved 3 times in 10 — and in those approving draws the reviewer's own rubric rationale still named the regression (a post-hoc keyword read of rubric rationale text, not a registered metric, and term-sensitive — 7–10 of 10 depending on the keyword; draw r8 discussed `basis`/memoized/`produces` and returned `pass: true`, `overall: 3`). The pattern held across classes: `issue-63`'s narrowed pin is mentioned in 10/10 draws and blocks in 1. **This is the pairwise–pointwise paradox** of *The Coin Flip Judge* (arXiv 2606.13685) [preprint] — the binarisation step carries the noise — and the criterion-dependent intra-rater reliability of *Rating Roulette* (EMNLP 2025) [peer]. *Are LLMs Reliable Code Reviewers?* (Automated Software Engineering journal, 2026) [peer] measured our exact regime: rationale-heavy prompts raise false rejection and produce rationale/verdict contradictions. n=10 resolves the class ordering (0.70 > 0.37 > 0.10 > 0.00) and not the levels; Coin Flip's curve says hard items need ~15 draws for 90% majority fidelity.

**S-TYPED: remove the verdict and the findings come out — and they are true.** With no `pass` field, detection stayed at ceiling on both seeded conditions and the reviewer produced blocking-grade findings on 12 of 23 CLEAN items. Under the old framing that is a 52% false-positive rate. Under adjudication it is not: every one of the 14 newly raised findings replicated mechanically — apply the reviewer's cited mutation, re-run the suite it named, watch it stay green — and two survived the *entire* suite. Add the four F24 items the typed arm also blocked (`fix-advance`, `146-147`, `135-129`, `issue-63`) and the 12 strict blockers are all confirmed. The typed arm reported the fifth F24 item, `issue-74`, as advisory only on this draw, so the union of *confirmed* unpinned items is **13 of 23 (57%)**, of which the typed reviewer blocked 12 in one draw — **at least 12 of 23 (52%) shipped, reviewed fix-lane items carry an acceptance criterion no test discriminates.** F24 had found 5. The prose reviewer, forced to emit a verdict, had reported 2. The registered contradiction secondary confirms the mechanism: on 4 of 5 F24 items the typed reviewer cited the known file with blocking-grade evidence, including two the prose arm had approved.

Prior art puts the 52% in range rather than out of it. Google's mutation testing at review time found a median of **one live mutant per changelist at review start** even after aggressive suppression (Petrović et al., ICSE 2021 [peer]; TSE 2021 [peer]), and Inozemtseva & Holmes (ICSE 2014) [peer] showed the coverage–effectiveness correlation drops to low or moderate once suite size is controlled — which is what a 1,907-test green suite under a live mutation looks like. Our number is criterion-directed (the reviewer chooses the mutation to attack the stated AC), so it should run higher than blind operator mutation; Meta's ACH (FSE 2025) [peer] is the closer analogue. No published study reports our exact metric; 52% is local to one repo and 23 items.

**The adjudication: the model's *labels* wobble at class boundaries; its *claims* do not.** F24's `narrowed-pin` on `issue-63` came back as `mutation-survived`, and `issue-74`'s confirmed mutation-survived finding came back advisory-only; the adjudication file records only the reviewer's label and CONFIRMED, so the label-vs-adjudicated-class disagreement is documented on those two items, not counted across the 14. The claim underneath was right 14/14. This is the adjacent-class confusion the ordinal-scale literature reports (C-SSRS study, arXiv 2505.13480 [preprint]) and the reason *When Policies Change Probabilities* (arXiv 2608.02677) [preprint] finds a 13.6–16.9 pp shift when a reviewer is asked to estimate *and* decide, recovered by a modular pipeline — model reports evidence, code applies the rule.

**What the gate must therefore be** (each rule traceable to the line above it):

1. **The reviewer returns evidence, not a class and not a verdict.** The schema keeps `evidence` as an *advisory* label for drift monitoring, but the **class is computed in Rust from the `repro` record**: `reproduced` iff a command was run and `observed ≠ expected_when_met`; `mutation-survived` iff a mutation was applied and the named suite stayed green; `narrowed-pin` iff only the discriminating assertion flips; otherwise `advisory`. S-TYPED's 147/147 repro fill rate is what makes this computable.
2. **Blocking requires re-execution, and re-execution requires a flip.** Before a `reproduced` or `mutation-survived` finding blocks, the deterministic gate re-runs the repro: it must fail on the candidate and pass on the reference, or (for a mutation) the mutant must survive the named suite when re-applied. "Both fail" or "cannot run" is `Unverifiable` → advisory. Agentless [peer] is the warning: 213 of 300 generated reproductions failed on base, only 94 flipped under the true fix. Jin & Chen's Fix-guided Verification Filter [peer] is the precedent for overriding the model with its own executed artefact (FNR 55–69% → 16–29%).
3. **The policy table is the *only* place a class becomes a block**, and it is a stage-weight parameter: `reproduced`/`mutation-survived` block; `narrowed-pin` blocks under a strict weight and is advisory under a lenient one (S-TYPED: 12 vs 9 of 23); `advisory` never blocks. S-VAR's class ordering sets the default order; its levels set nothing.
4. **A human override is a first-class state, not a deletion.** `Quarantined { reason, by, until }` stops a finding blocking and is not re-raised on re-review (GitLab flaky quarantine [1P]; Cloudflare "won't fix" [1P]).
5. **Every block and every override is an event**, and `false_block_rate` and per-card `pass^k` are reported telemetry. Cloudflare's 0.6% break-glass rate (288 of 48,095 MRs) [1P] is the only public comparator; nobody else measures it.
6. **A negative control ships with the gate.** S-SUBTLE-shaped fixtures (a defect the checklist does not name) stay in the corpus as the reviewer's regression test; *Reliability without Validity* (arXiv 2606.19544) [preprint] — a stable approval is consistent with a lenient judge.
7. **"Reviewed and green" stops being evidence that a criterion is pinned.** The per-AC mutation columns of the F25 remediation become the product's normal TDD evidence, and the reviewer's job on a fix is to *produce* the discriminating check the author did not.

---

## 3. Cost, and where to spend it

The typed reviewer costs **+71% on CLEAN and +101% on S-SILENT** against the prose reviewer, with median wall clock 1,071 s (~18 min) and median $3.03 per CLEAN review; the slowest CLEAN review ran 4,561 s. The prediction (±15%) was wrong because the reviewer does what the schema asks — it runs a repro for every finding. (The 4,561 s draw is `issues-135-129` CLEAN, a retry at `--timeout-secs 5400` after a 3,601 s void, disclosed in METHOD-S §7; every other S-TYPED CLEAN record ran under the 3,600 s ceiling.)

Against the field this is a middle tier, not an outlier: Cloudflare's read-only reviewer runs at a median $0.98 / 3 m 39 s with a $1.68 "full" tier (131,246 runs / 30 days) [1P]; Anthropic's verification-heavy Code Review is $15–25 and ~20 min per review [1P]. Ours is $3 with a runnable repro on every blocking finding, which neither publishes. The design consequence is **routing, not restraint**: Cloudflare's size-tiering cuts trivial-review cost ~8×; OpenCodeReview (arXiv 2608.09290) [preprint] reports 5–15× fewer tokens from deterministic dispatch with bounded tools. So:

- The **review stage weight** in a process pack selects the tier: `0.2` = read-only findings (no execution, advisory only), `1.0` = execution-grade with repro. `bug-standard` runs execution-grade because the 52% says the fix lane is where unpinned criteria live; a documentation-only or generated-code change does not.
- **Appetite is set at triage and the runner enforces it** (principle: three counters). A CLEAN review's 4,561 s tail says the timeout is a pack parameter per toolchain, not a constant — the S-VAR amendment already moved it to 3,600 s for this repo.
- **No cascade until its recall loss is measured.** A cheap-first/expensive-on-warrant design is well precedented for cost and unmeasured for detection; it is a Phase 2 spike, not a Phase 1 default.

---

## 4. Cache, prompts and the price of a human gate

Four measurements now describe the prefix completely, and they resolve into one rule with one open alternative.

| what | measured | source |
| --- | --- | --- |
| prefix shape | ~15.1k shared across *any* system prompt (tool block) + a text-dependent tail (~14.5k for the card, ~21k for the default) | S-H1b 1-min mark; F13; platform docs [1P]: tools → system → messages, one hash per breakpoint |
| commit between dispatches | default prompt 0.689; `--system-prompt` **1.000** | F16 |
| worktree per dispatch | default 0.685; `--system-prompt` **1.000** (w2–w10) | F16 arm d; S-WT |
| 65 min idle | default prompt 18,603 created (the head survived on subsidy); `--system-prompt` **29,619–29,680 created** (everything) | F28 correction; S-H1b |
| TTL | ~1 h, boundary 45–62 min, `ephemeral_1h` on every write | F10; Claude Code docs [1P] |

The docs explain the subsidy: the cache is **org-scoped**, a read refreshes the entry it hits, and the default prompt's static head is refreshed by any session on the account [1P]. That makes the subsidy real and also a hazard — a CLI upgrade anywhere on the account changes the tool block, and #46829 shows the TTL default has flipped server-side before.

**Rule for Phase 1:** deliver the card as a static `--system-prompt`, because the fix lane commits at every checkpoint and runs stories in worktrees — the two cases measured at 1.000 — and accept that a resume after a gate longer than the TTL costs one full prefix write (~30k at review scale). Pin `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and the CLI version; keep `--strict-mcp-config` (the shared tool block is half the prefix); keep model and effort constant per card. Gate *duration* is not a cost decision — the default-prompt write was identical at 66 minutes and 49.7 hours (F28), and a card-prompt prefix is already fully cold at 66 minutes (S-H1b; its 24 h read was not taken, so "two days" for the card is inferred from a prefix with nothing left to lose); gate *frequency* is, marginally.

**Open alternative, registered and unrun (S-H1c):** default prompt + `--exclude-dynamic-system-prompt-sections` + `--append-system-prompt`. If it keeps the subsidised head *and* holds ≥0.95 across commits, it dominates. The platform's own "pre-warm" pattern (`max_tokens: 0`) [1P] is API-only; on a subscription a keep-alive turn's quota cost is undocumented, so no heartbeat is built.

**Human gate, final shape.** F18/F19 (half of gates outlive the TTL; drift 3% under an hour, 11% over; suspension 174/174 lossless) plus the cost bound above give: gate = `(token, tree_hash, deadline)`; answering is a durable write; a stale answer (tree hash differs) is *void, not reconciled*; expiry has a terminal path; resume reconciles before acting. Every durable system surveyed — Temporal, Restate, Step Functions, Azure Durable, GitHub environments, Trigger.dev [1P] — mints a single-use token and most carry a deadline; only GitHub pins to a SHA. Binding to the tree hash is stronger than any of them.

---

## 5. Artifacts, store and the things the census found

S-MIG's numbers set the type decisions D5/D6 left open, and two of its misses are product findings rather than parse failures.

- **The AC type is `{ id, criterion: text, gwt: Option<Gwt>, verification: Option<text> }`.** 19 of 153 specs — all shipped and reviewed — write criteria without GIVEN/WHEN/THEN or without a verification note. The type accepts them; the *policy* decides whether a missing verification blocks a stage.
- **Gate results are emitted, never parsed.** Free-form in 25% of implementation reports and 83% of reviews. The toolchain adapter produces a typed result; the report renders it; migration fills `unknown`.
- **TDD evidence migrates with `mutation: unknown` on 142 of 156 rows** — the F25 columns exist in 0 reports. This is §2's 52% seen from the artifact side.
- **`plan.md` does not exist in Redkiln-RS.** 110 of 110 are the unfilled scaffold; planning lived in `_plan.md`, `_storymap.md` and the spec. Scott Logic's Spec Kit accounting (2,577 lines of markdown for 689 LOC) [blog] and Agent OS v3's retirement of orchestration phases [1P] are the same finding by intuition; the product version is an `artifact.read` event per activity input and a doctor rule that flags any stage whose artifact has zero downstream reads over N runs — which no surveyed system does.
- **Companions are sidecars with their own schemas.** `_integration`, `_grounding`, `_decomposition` are three shapes; 10 of 28 have no frontmatter at all. Layout C (D6) already allows this; the migration tool needs one schema per companion kind and a hand-fix path for the 2 `_ledger.md` files whose YAML does not parse.
- **Frontmatter is regular everywhere** (209/209 cards, 153/153 specs, 46/47 atom files with `id`), and the corpus's own key census — `archetype`/`slice` on 110 stories, `last_reviewed` on every atom that carries an id — is the type; the hand-listed keys in rev. 3 were incomplete on both.
- **SHA-keyed rows need a rewrite map.** Trailers survive rebase; the SHA an event references does not. git-branchless's `RewriteEvent(old→new)` [1P] and jj's conflict-as-value [1P] are the precedents; Beads and Backlog.md #558 [1P] show worktree confusion is the commonest field failure, so the kernel resolves the common dir at startup and refuses to write when the worktree's HEAD is not the branch the run claims.

---

## 6. What this does to the decisions

| decision | status after the data | change |
| --- | --- | --- |
| D1 inversion · D2 clean break · D3 five ports, no Postgres · D7 Codex validation-only · D9 CLI-only gates · D10 subscription-only · D11 two packs · D13 corpus repo · D14 pack composition | **unchanged** | — |
| **D4** findings-not-verdicts | **rebuilt** | class computed from the repro record in code; flip criterion re-executed before a block; `Unverifiable` and `Quarantined` states; policy table is a stage-weight parameter; block/override telemetry; negative-control fixtures (§2) |
| **D5/D6** human edits, layout C | **refined** | AC type with optional GWT/verification; gates emitted not parsed; sidecar schema per companion; `plan.md` removed; `artifact.read` instrumentation (§5) |
| **D8** UE fixture, S-BIN | **unchanged, unrun** | `ChangeSummary` still designed against the real project before the Unreal adapter |
| **D12** unknowns carried as spikes | applied | S-H1c and S-BIN remain registered and unrun |
| **D15** (new) | review tiering | review stage weight selects read-only vs execution-grade; `bug-standard` is execution-grade; no cascade until recall loss is measured (§3) |
| **D16** (new) | card delivery | static `--system-prompt` for Phase 1 with the cost accepted; append route as a config option pending S-H1c; TTL and CLI version pinned (§4) |
| **D17** (new) | gate record | `(token, tree_hash, deadline)`; stale answers void; terminal expiry path (§4) |
| **D18** (new) | rewrite events + worktree ownership check | `rewrite(old_sha → new_sha)` events on rebase/amend; kernel resolves the common dir at startup and refuses to write when the worktree's HEAD is not the run's branch (§5) |
| **D19** (new) | per-AC discriminating test is the product norm | "reviewed and green" is not evidence of pinning; the reviewer's job on a fix is to produce the discriminating check the author did not (§2) |

---

## 7. What remains unmeasured, and how the design hedges each

| open | why it matters | hedge |
| --- | --- | --- |
| S-H1c (append route) | could remove the ~30k resume cost | D16 keeps it a config option; default is the measured route |
| S-BIN (binary artifacts) | `ChangeSummary` for UE is designed, not tested | Unreal adapter stays Phase 2; type is designed against the real project before it |
| h1b-24h read | can only confirm a fully-cold prefix already seen at 66 min | none needed |
| n=10 on 6 items, one repo, one operator | S-VAR levels are ±30 pp; S-TYPED CLEAN is a single draw per item | policy takes the *ordering*; pass^k with intervals is telemetry from day one; borderline fixtures get 15–20 draws when a claim depends on them |
| CLEAN ground truth | "shipped and approved" is wrong for ≥52% of the corpus, so no false-positive rate against it is meaningful | false-block rate is measured against *adjudication* (mechanical re-execution), never against historical approval |
| evidence-class labels | model labels wobble at boundaries | class computed in code; label kept for drift monitoring |
| platform drift (TTL defaults, `--bare` default flip, tool block on upgrade, undocumented events) | every cache number is date-stamped | pinned TTL and CLI version; adoption/isolation asserted from `system/init`; raw events retained |
| Opus family sub-limit invisible (F1) | the binding budget cannot be throttled against | appetite per item; `five_hour` margins ≥2 ticks |
| the reviewer's own time (one CLEAN review voided at 3,601 s and completed at 4,561 s on a 5,400 s retry) | a review can outlast a worktree's usefulness | timeout is a toolchain-pack parameter; the runner records `basis: timeout` as a void, never a pass |

---

## 8. The specification, in one table

| component | rule | measured by |
| --- | --- | --- |
| Kernel | Rust holds every transition; agent classifies and may request typed deviation; process packs own structure, technology packs bind and contribute at slots | RFC §3.3; D14 |
| Harness adapter (Claude) | explicit card via `--agents`/`--agent`; static `--system-prompt`; `StructuredOutput` in tools; `--strict-mcp-config`; `--permission-prompts none`; never `--bare`; adoption/isolation asserted from `system/init`; TTL + CLI pinned; typed retry with durable `retry_at` marks | F7, F8, F16, F20, F21, S-WT, S-H1b |
| Review activity | typed `Findings` (advisory `evidence` label, `rationale`, structured `repro`); class computed in code; flip re-executed; policy per stage weight; `Quarantined`; negative-control fixtures; block/override events | S-VAR, S-TYPED + adjudication, F24/F25 |
| Verification gate | affected-area selection with `basis: whole-suite` fallback recorded; per-AC discriminating test required; mutation evidence columns | F11, F25, S-MIG |
| Human gate | `(token, tree_hash, deadline)` mark; process exits; answer is a durable write; stale answer void; reconcile on resume; ≤ one prefix rewrite per resume | F18, F19, F28, S-H1b |
| State | git trailers tier 1, SQLite tier 2 at the common dir, SHA-keyed with rewrite events, rebuildable, worktree ownership asserted | F12, F19, S-WT, prior art §5 |
| Artifacts | typed frontmatter + rendered read-only sections + opaque prose; sidecars per companion; AC type with optional GWT/verification; gates emitted; `plan.md` gone; `artifact.read` events | S-MIG |
| Telemetry | per-step cost and cache share; `pass^k` per card; false-block and override rates; gate latency; artifact reads | principle 11; Cloudflare comparator |

---

## 9. Sources

**Repository:** `spikes/FINDINGS.md` F1–F28 (F28 corrected 2026-09-05); `spikes/METHOD-S.md` and its §7 amendments; `spikes/results/s-var.jsonl`, `s-var-citations.py`, `s-var-citation-targets.json`, `s-var-2400.jsonl` (quarantined); `s-typed.jsonl`, `s-typed-adjudication.jsonl`, `s-typed-oldschema.jsonl` (pre-amendment, unused); `s-h1b-1m.jsonl`, `s-h1b-65m.jsonl`, `s-h1b-24h.jsonl` (mark only); `s-wt.jsonl`; `s-mig.jsonl`, `census-s-mig.py`, `s-mig-report.md`; `references/research/2026-09-06-grounding-s-results.md` (Parts A–C with full source lists).

**Prior art cited above (dated in the grounding briefs):** *The Coin Flip Judge?* arXiv 2606.13685 [preprint]; *Rating Roulette*, EMNLP 2025 Findings [peer]; *Are LLMs Reliable Code Reviewers?* Automated Software Engineering (journal), 2026 / arXiv 2603.00539 [peer]; *Reliability without Validity*, arXiv 2606.19544 [preprint]; *When Policies Change Probabilities*, arXiv 2608.02677 [preprint]; C-SSRS severity study, arXiv 2505.13480 [preprint]; Agentless, FSE 2025 / arXiv 2407.01489 [peer]; Meta ACH, FSE 2025 / arXiv 2501.12862 [peer]; Petrović et al., ICSE 2021 (arXiv 2103.07189) and TSE 2021 (arXiv 2102.11378) [peer]; Inozemtseva & Holmes, ICSE 2014 [peer]; *Capacity, Not Format*, arXiv 2606.09410 [preprint]; *Constraint Tax*, arXiv 2606.25605 [preprint]; OpenCodeReview, arXiv 2608.09290 [preprint]; Cloudflare, *Orchestrating AI Code Review at scale*, 2026 [1P]; Anthropic, *Code Review for Claude Code*, 2026-03 [1P]; Google AutoCommenter, FSE-Industry 2024 [peer]; Anthropic prompt-caching, structured-outputs, Claude Code caching/costs/CLI docs [1P]; claude-code #45756, #46829 [1P]; Temporal, Restate, AWS Step Functions, Azure Durable Functions, GitHub environments, Trigger.dev docs [1P]; GitLab quarantine [1P]; git-branchless, Jujutsu, Gerrit NoteDb, Beads, Backlog.md #558 [1P]; Scott Logic on Spec Kit, 2025-11-26 [blog]; Agent OS v3 discussion #310 [1P].
