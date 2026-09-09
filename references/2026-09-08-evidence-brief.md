# Redkiln-RS — condensed evidence brief

**Date:** 2026-09-08 · **Status:** Working condensation, promoted into the repository so that the citations in the acceptance-criteria and success-measure registers resolve. **Not a source of truth** — `spikes/FINDINGS.md`, `spikes/F29-DRAFT.md`..`F38-DRAFT.md` and `spikes/DECISIONS-2026-09-09.md` remain authoritative, and where this brief and they disagree, they win. · **Purpose:** one citable digest so a dozen authoring agents cite the same figures instead of each re-deriving them from a 97 kB ledger. · **Evidence discipline:** every figure below is traceable to a named source file; nothing is rounded.

---


Derived from `spikes/FINDINGS.md`, `spikes/F29-DRAFT.md`..`F38-DRAFT.md`, `spikes/DECISIONS-2026-09-09.md`,
`spikes/METHOD*.md`, `spikes/HANDOFF.md`. Every figure below is traceable to those files. **Do not quote a number
that is not here without reading its source file first.**

## A. Finding ledger (F1–F38; there is no F9 entry, and F30 was reserved for S-H1b/S-WT and never written)

| id | claim | number | status |
|---|---|---|---|
| F1 | Plan headroom is readable via an undocumented top-level `rate_limit_event`; `result.rate_limit` is null | `unifiedWindows.{five_hour,seven_day}.utilization`; **no Opus/Sonnet family sub-limit key** — the binding limit is invisible | measured |
| F2 | Settings-based hooks DO fire in `-p`, including user-level ones | 2 hook events per dispatch on a bare fixture | measured |
| F3 | `usage` uses plain cache naming and exposes no TTL split | `ephemeral_1h_*`/`ephemeral_5m_*` absent; trivial haiku dispatch $0.021–0.027 | measured; charter E1's split metric untestable from usage |
| F4 | Further undocumented `-p` stream events | `system/thinking_tokens` 4–6/dispatch | measured |
| F5 | Cache-read share is noisy at n=2 | 0.584/0.698/1.000/0.722 on an identical prompt | explicitly NOT a result |
| F6 | `utilization` is quantised to 0.01, so per-run cost is not measurable | five_hour and seven_day proven independent, not mirrors | measured; forces batch-delta reporting + a `resolution_limited` flag |
| F7 | `--agents` is INERT without `--agent` | canary: both → BANANA; `--agents` alone → 4; card's model+effort accepted | measured; retires RFC R2 |
| F8 | A default `claude -p` inherits the operator's whole environment | 12 MCP servers, ~280 tools; cache_creation 8,243 → 3,611 under `--strict-mcp-config`; MCP = 4,631 tokens = **56%** of the write; tools 280→34; TTFB ~1,480 of ~3,400 ms | measured |
| F9 | *(cited but has no ledger entry)* the charter's blocker-recall answer key does not exist | all 15 `_review.md` Required-Changes sections read "None" | struck; forced pre-registration amendment 1 |
| F10 | Prompt-cache TTL is ~1 h | full hit at 45.1 min; **61.6 min → share 0.690 (31% expired)**; boundary bracketed (45.1, 61.6), never located | measured, n=1 per gap; date-stamped platform behaviour |
| F11 | S0.1: the affected-area gate kills 9 of 10 test-reachable mutants | 15 mutants; 9 killed at test, **1 survived**; headline 93.3%, test-reachable **90.0%**; median wall 323 s | measured; CI on 1/10 is wide |
| F12 | S0.2: 1 slice in 5 passed through a rejected state | 360 trailer commits, 74 slices; **ever-rejected 15/74 = 20.3%**; sealed >once 21.6% | measured, re-scoped and disclosed (registered primary uncomputable: 1 of 101 runs recorded the field) |
| F13 | The prefix has TWO segments; 69% is shared across directories | ~17,439 shared + ~7,827 per-directory; 19/19 fresh repos identical | measured; reprices the worktree cost from 100% to ~31% |
| F14 | 40 dispatches / ~1.9M tokens stayed below the instrument | seven_day 0.100 → 0.100 | gate STRUCK, not answered |
| F15 | E2: reasoning effort IS applied per dispatch | thinking tokens low **0.0** / high **4.8** / xhigh **221.0**, complete separation; output 121/130/360 | measured; **low vs high did NOT separate** |
| F16 | E1: a commit costs 31% of the prefix; `--system-prompt` costs nothing | steady share: no-commit 1.000 · commit-each **0.689** · +`--exclude-dynamic` 0.846 · +`--system-prompt` **1.000** · worktree-each 0.685 | measured, 50 dispatches, within-arm range ≤0.001 |
| F17 | The E3 fixture was defeatable: the reviewer restored the fix from the object store | 42 turns, 22,853 output, 526 s, returned `approved` | measured as an instrument defect |
| F18 | Half of all human gates outlive the cache TTL | gate→verdict n=174 median **10.5 min**, 45% >1 h; stage→verdict n=191 median 51.6 min, p90 ~9 h, **max 226 h (9.4 days)**, 48% >1 h | measured, zero model cost; one operator |
| F19 | H2/H3: suspension is lossless; the world moves rarely and predictably | **174/174 lossless**; drift 12/174 = **7%**; ≤1h 3/95 = 3% vs >1h 9/79 = **11% (3.6×)** | measured; **limit: the FSM was never replayed**, so "same next action" is unproven |
| F20 | An agent card's `tools` allowlist silently disables `--json-schema` | `structured_output` absent, exit 0, `subtype: success`; adding `StructuredOutput` fixes it | measured; the mechanism behind a review consumer that fails open |
| F21 | E3 arm (b) complete | S-SILENT **23/23**, CLEAN FP 2/23, MCC **+0.917**, spend **$56.47**; five_hour +0.03 S-SILENT / ~+0.077 CLEAN | measured; FP provisional; the +0.077 figure is **superseded by F29's 0.0098, 7.8× lower** |
| F22 | Background jobs die; a detached process does not | ~12 kills vs one **12,000 s** survivor under `Start-Process`; `issue-81` killed 3× as background, **116 s** in foreground | measured (operational); cause still unestablished |
| F23 | E3 RESULT: the kill criterion does not fire | detection **23/23 vs 23/23**; CLEAN FP 4/23 vs 2/23; MCC +0.839 vs +0.917; **McNemar p=0.625**; cost **$97.39 vs $56.47 = 1.72×**; `parent_inspected` **0/46** | measured; **primary metric saturated — no discriminating power** |
| F24 | ADJUDICATION: all six E3 rejections are factually TRUE | 6/6 confirmed, zero model cost; if all blocking both arms → FP 0/23, MCC +1.000 | measured for factual truth only; **severity unadjudicated (Ryan's)**; CLEAN ground truth wrong for ≥5/23 = 22% |
| F25 | ROOT CAUSE: Red→Green is evidenced at TEST granularity, but an AC is a claim about ONE ASSERTION | AC-015 probe: remove step 7 → RED; invert the ordering → **36/36 GREEN** | measured mechanism + a 4-step remediation package, proposed not adopted |
| F26 | E3-H: 20/20 across two harder tiers; the prediction was falsified twice; the line is closed | S-SUBTLE sighted 5/5 and 5/5, blind 5/5 and 5/5; spend $36.18; blind +18% median wall | measured |
| F27 | E0 VARIANCE: detection is stable, the verdict is not | detection **0/7 items flaky over 21 draws**; verdict **2/4 items flaky** | measured, n=2–3; a "stable" label here tolerates a true flip rate up to ~30% |
| F28 | H1/H4: a human gate costs a bounded ~18.6k tokens however long it lasts; the digest carries what is used | 0.9 min → share 1.000/creation 0; **65.2 min → 18,603**; **49.7 h → 18,603** (identical to the token); `--system-prompt` at 66.8 min → share **0.000**, creation **29,680**; H4 Jaccard FULL 1.000 = CONTROL 1.000, effect **+0.000** | measured, with a published correction: the flatness was **shared-prefix subsidy, not durability** |
| F29 | S-VAR: not one defect class is stable-reject; the verdict is a weighted coin, but every rejection is on target | 60 draws, 0 void, **$176.96**, 16.2 h; class means Reproduced **0.70** > MutationSurvived 0.37 > NarrowedPin 0.10 > control **0.00**; **19/19 rejections on-target**; per-draw five_hour **0.0098** | measured |
| F30 | **UNASSIGNED / NEVER WRITTEN.** Reserved for S-H1b/S-WT | S-H1b: 1.5 min share 1.000; **66.1 min share 0.000, creation 29,619**; 24 h read **not taken**. S-WT: w1 0.807, **w2–w10 all 1.000** | the numbers exist only in the design-recommendation and the proposal |
| F31 | S-TYPED: the typed contract costs no detection and conforms perfectly | 51 valid, **$112.25**; S-SILENT 23/23, S-SUBTLE 5/5; **schema conformance 52/52 = 100%**; `repro.command` fill 147/147; CLEAN blocks strict 12/23, lenient 9/23; adjudication **14/14 confirmed**; **verified false positives 0/23**; cost **+74%** | measured; the severity call ("narrowed-pin advises") is Ryan's |
| F32 | P9: the schema authors at a median 56 lines | median **56.5** non-blank lines (kill is >150); lint 13 real / 34 false; largest fragment **5 of 10** — exactly at the threshold | measured, zero model dispatches |
| F33 | P9b: a second author lands at a median 73 lines; the lint accepted 5 guesses silently | median **73** (+29%); 27 ambiguities, 20 new | measured; falsifies P9's "amendments will fall" at n=3 |
| F34 | P6 authoring: a third author wrote 90 activities at a median 53 lines | median **53**, sum 5,059; **0.11 first-run lint errors/file** (author read the lint first) vs P9b 1.3, P9 4.7; quality block in **42/90**; amendments 26–43 | measured; PC5 clears by **2.8×** |
| F35 | P6 lattice: unsatisfiability 23.5% → 11.4% → **0.3%** after decisions 1–10 | dead activities 15/101 → **6/102**; rank ties **0**; hard cycles **0**; lattice runtime 27 s | measured; the pre-registered reading fires the kill twice, the amended lattice clears all three criteria |
| F36 | **P1: the LLM-emitted context vector is not stable enough to drive deterministic selection, and the prescribed remedy does not repair it** | 400 dispatches, 0 void, $21.37; **modal `R` identical on 5/20, 3/20, 2/20, 2/20** vs an 80% line; alpha drift **−0.168 on byte-identical anchors**; `external_dep` became **constant** (200/200 false); `unknown` rates novelty 46.0%, size 39.5%, blast_radius 31.0%, criticality 27.0% | measured for the unlabelled half; **arm D precision/recall NOT COMPUTED — awaits Ryan's blind panel** |
| F37 | **P7: the `adversary → policy` block costs 2.64× and changes the root cause on 1 item in 10; its policy check was false on 33 of 33** | 61 runs, 297 records, 0 void, **$131.62**; block $2.69 vs $1.02/run; adversary $1.68 / 676 s; **188 findings, 78% required running something**; deviation offered **71 times, used 0** | measured, **but the primary criterion is powerless at n=3** — every Wilson interval overlaps by construction |
| F38 | **P8 PARTIAL: mid-graph resume is lossless across a real process kill and costs less than a stage gate** | 3 of a pre-registered 27+3, $3.01; **0 changed, 0 lost, digests identical** across `taskkill /T /F`; resume **0.45× / 0.55× / 0.78×** of ~18.6k | PARTIAL; 65-min and 24-h arms and all 3 controls **unrun**; the second-arrival question is **not answerable** (arrivals forced, mark minted by the driver that implements the semantics) |

## B. Kill criteria — outcome and threshold

**Fired.** P1 §1.4 modal-R (<80% identical) — **fires 4/4**, the one to act on. P1 §1.4 alpha<0.6 — fires twice but
**the predicate is not decidable on this corpus**. P6 §3.4 unsatisfiability (>5%) — fires on the pre-registered reading
(23.5%, 11.4%), clears post-decision (0.3%). P6 dead activities (>20%) — fires by 0.8 points under the 8 pre-registered
packs (20.8%), clears under 12 (14.9%) and post-decision (5.9%). S-VAR §1.5 branch 2 and **branch 3 (architecture-changing:
a reproduced regression approved in ≥2 of 10 — measured 3 of 10)**. S-TYPED §2.6 branch 3, whose own remedy was executed
(14/14 confirmed → lenient default). H1's ~25k threshold sits INSIDE the measured 18.6k–29.7k range.

**Did not fire.** E3 §9 (23/23 vs 23/23, p=0.625). E3-H. E1 H1/kill. E2. S0.1. H2 (174/174). H4 (digest sufficient).
S-VAR branch 4 (control 0/10). S-TYPED branches 1, 2, 4 (conformance 100%). P7 §4.4(4) fixture-too-easy (7/10).
P8 §5.4(1) lost output (0 lost) and §5.4(2) resume cost (0.45–0.78×, kill was >1.5×). P9 §4 verbosity, three times.

**Not answerable / powerless.** E0 arm-sizing (E4–E6 never ran). E0 cache-cost gate (STRUCK at 0.01 resolution).
P7 §4.4(1) — establishes the criterion's **power, not its verdict**, at n=3. P7 §4.4(2) — n=1, arm dropped.
P7 §4.4(3) — **empty denominator**, 0 deviation requests in 71 offers. P8 §5.4(3) — **circular by construction**.
P2 all three criteria — unanswered, the engine phase refuses until Ryan's labels are timestamped.
S-BIN and S-H1c — registered and **unrun**.

## C. Architecture-bearing implications

1. **Cache, split rule.** `--system-prompt` holds 1.000 across commits (F16) and across worktrees w2–w10 (S-WT), but
   **forfeits the account-wide subsidy across TIME**: 29,619–29,680 created at ~66 min, share 0.000 (F28 correction,
   S-H1b). The default prompt costs a flat 18,603 at both 65 min and 49.7 h. The two mitigations point in OPPOSITE
   directions. Forbids inferring "held across commits" implies "helps across time" — explicitly falsified.
2. **Gate cost.** Duration is free; frequency is marginally costly. Mid-graph gates are CHEAPER than stage gates
   (F38, 0.45–0.78×). Placement is a value decision.
3. **Drift.** 7% overall, 3.6× more likely on gates over 1 h (F19); 45–48% of gates exceed 1 h (F18); median in-gap
   commits is 0. Forces `reconcile()` on every entry and forbids an optimisation that skips it.
4. **Resume.** Lossless at activity granularity (174/174) and at mid-graph step granularity (3/3 across a real kill).
   Forbids claiming more: the FSM was never replayed, and P8's state holds exactly one step.
5. **Structured output.** A tool-restricted card silently voids `--json-schema` at exit 0 (F20). Forces `StructuredOutput`
   in every card whose role demands a schema, checked at configuration time, and forbids parsing JSON out of prose.
6. **Process survival.** Nothing may depend on a process surviving (F22). Every wait is a durable mark plus a later read.
7. **Neither gate is sufficient alone.** A reproduced regression was approved 3/10 (F29) and four adjudicated defects
   survived fully green suites of 1890/1890 and 1907/1907 (F31). The review gate is high-precision, low-recall: use it to
   ADD findings, never to CLEAR an AC. Blocking is computed in code from re-executed evidence, never from the model's `pass`.
8. **The policy relaxation is free.** Lenient (`narrowed-pin` advises) gave 9/23 blocked, 0/23 verified FP, detection
   unchanged (F31). But the 7 adjudicated-true `narrowed-pin` findings still need a destination — no Advisory sink exists.
9. **Selection input is unstable (F36).** Per-facet alpha looks healthy on 4 of 8 axes while `R` almost never repeats:
   four axes each flipping on a minority of draws compose into an unstable required-artifact set. `unknown` is treated as
   `high` by position, so an axis oscillating unknown↔low moves `R` the full band width.
10. **P7's policy predicate is a constant function of the arm** — satisfiable only by a reviewer that finds nothing, which
    F23/F26/F31 prove this reviewer is not. And the rigor ladder is **not monotone in internal quality**: escalating to
    `mandatory` with `has_repro` selects `bisect`, which has no adversary and no policy step.
11. **Isolation is a measurement precondition.** 56% of a default cache write was unrelated MCP servers (F8), and the flag
    must be verified from the child's own `system/init` because an ignored flag looks identical to an honoured one.
12. **Method finding.** Five metrics saturated. Every discriminating result the programme produced came from **cost,
    stability or mechanism — never from success rates** (F28).

## D. Cost, token and latency figures

Spend: E3 $153.86 total (a $97.39 / b $56.47, **1.72×**) · E3-H $36.18 · F28 $18.85 · S-VAR $176.96 · S-TYPED $112.25
(prose comparator $64.61, **+74%**) · P1 arm L $21.37 · P7 $131.62 of a $200 cap · P8 $3.01 of a $40 cap ·
AFK session **$156.00 of $300**. P2, P6, P9, P9b, arm D, S-MIG, S0.1, S0.2, F24, F25 and the F31 adjudication: **$0**.

Per unit: trivial haiku dispatch $0.021–0.027 · E3 S-SILENT $0.45–0.64 (97–424 s) · E3 CLEAN $1.06–4.89 (317–2,055 s) ·
P7 graph run optional **$1.02** / recommended **$2.69** / mandatory (n=1) $9.86 · P7 adversary dispatch $1.68 / 676 s ·
S-TYPED CLEAN review median **$3.03**, median **1,071 s (~18 min)**, slowest 4,561 s.
External comparators: Cloudflare read-only median $0.98, full tier $1.68, **break-glass 288 of 48,095 MRs = 0.6%** ·
Anthropic Code Review $15–25 · Agentless $0.70/instance at 32.00% vs CodeR $3.34 at 28.33%.

Headroom: instrument resolution **0.01** · per CLEAN draw **0.0098 five_hour** (F29; F21's 0.077 is 7.8× too high) ·
seven_day 0.0017/draw · the binding constraint on a CLEAN set is **wall clock, not headroom** (16.2 h for 60 draws).

Tokens: default `-p` cache write 8,243 → 3,611 isolated · warm trivial prefix ~25,260 · prefix segments ~17,439 shared +
~7,827 per-directory · commit invalidation 7,853 (half recovered by `--exclude-dynamic`) · review-scale prefix ~36,160
default / ~29,680 static · **gate resume 18,603 default, 29,619–29,680 static — at most one prefix rewrite, flat in
duration** · mid-graph resume 8,408 / 10,314 / 14,579.

Latency: TTFB ~1,480 of ~3,400 ms · E3 median wall S-SILENT 242 (a) vs **128 s** (b), CLEAN 769 vs 682 s ·
S-TYPED CLEAN median 1,071 s vs prose 682 s (**1.6×**) · S0.1 mutant runs median 323 s · full gate has **no affected-area
selection** — every run is the whole 236-file suite (~250 s to >900 s, 1,890–1,927 tests) · human gate median 10.5 min,
p90 ~9 h, max 9.4 days.

Timeout history: 900 s default (**a default written for the fast condition** — cost a CLEAN record) → 2,400 → 3,600 →
one 5,400 s retry. `confirm` bug: `pnpm test -- --run <file>` ran the WHOLE suite for 26 of 68 invocations;
`npx vitest run <file>` is 1 file / 6 tests / **15.6 s** — a ~60× difference.

## E. Open items that must not disappear

Awaiting Ryan: P1 arm D precision/recall (blind panel unfilled) · P2 entirely (labels unwritten; the engine phase refuses) ·
the E3 severity adjudication (facts settled 6/6, severity is not, so both FP rates and both MCCs are provisional) ·
the `bisect` ceiling question · the DECISIONS-2026-09-09 "reverse or confirm first" table (#15 `human_authored` remap
changes what the advisory fraction measures; #7 the mandatory arm prices an unreachable configuration; #5 20 items not 30).

Pre-registrations found broken: P1's alpha rule (not decidable) · P7 criteria 1–3 (powerless / n=1 / empty denominator) ·
P8 §5.2's fixture design ("remove the repro steps so the gate fires" does not work against an agent with Read+Bash) ·
P8 §5.4(3) (circular) · METHOD-H §6 (vacuous, replaced before data) · S0.2's registered primary (uncomputable at n=1) ·
charter E1's TTL-split metric (not in `usage`) · charter E4's oracle (empty ledger join).

Reclassified, not absent — **spike P5**: replay identity + conformance was **reclassified** out of the pre-design P-series into a Phase 1 acceptance test because it needs the kernel's event log and step-level memo keys (`references/2026-09-07-process-catalog-proposal.md` §6 row P5, §6.1). Its criterion is live and belongs to the kernel: 100% replay identity of `dag_assembled` re-derived from logged inputs, violations attributed to pack versus kernel, and the kill — **any non-reproducible DAG means a hidden input, which means the memo key is unsound; fix before anything else.** Recorded here because an earlier condensation omitted it and a downstream refutation pass then struck it as nonexistent.

Unrun: **S-BIN** (gates Phase 1 for `ChangeSummary` type design) · **S-H1c** (the append route; D16 is conditional on it) ·
the h1b 24 h read · **P8's 65-min and 24-h arms and all 3 controls** · P7's mandatory arm (dropped at n=1) · E3 arm (c) ·
E4/E5/E6 · F28's `nocommit` arm (closed unmeasured).

Contested: **CLEAN's ground truth is wrong for ≥13 of 23 (57%)** — every false-positive rate measured against it is an
UPPER BOUND. F37's mandatory cost ratio is reported as 3.66× in F37 and 4.4× in the decisions log — unreconciled.
`recent_deploy` is underivable at this pin · `binary_change` is measured but unregistered in facets.toml so it can feed no
require row · `dynamism` is pre-registered but has no registry entry. F37's `emit.corpus_sha` holds a per-item checkpoint
sha, so the key is misnamed. Working tree not clean as of 2026-09-09; HEAD was reported as `ab58733` and nothing from the AFK session
was committed.

**Corrected 2026-09-09:** the AFK session's output **was** committed, at `e3b0bac` ("P1 arm L x2 (400 dispatches, 0 void), P7 complete (61 runs), P8 partial, three rig subcommands"), which is HEAD. `spikes/HANDOFF.md` still says HEAD is `ab58733` with nothing committed; that statement is stale and this document does not inherit it. What actually remains untracked is 17 items: the raw P1/P7/P8 driver logs, `spikes/results/p8-state/`, and the `spikes/runner/target-dev/` and `target-p8/` build directories (`.gitignore` covers `target` but not those two names). The F36/F37/F38 drafts, `spikes/DECISIONS-2026-09-09.md` and every `spikes/results/*.jsonl` are tracked.
