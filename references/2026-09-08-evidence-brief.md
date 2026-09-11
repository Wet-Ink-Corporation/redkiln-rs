# Redkiln-RS — condensed evidence brief

**Date:** 2026-09-08, **amended 2026-09-09 with an F39 row** (§A, §B, §C.13, §E) and **again 2026-09-10 with an F40 row** (§A, §B, §C.14, §E) — the condensation itself covers F1–F38 as of 2026-09-08; F39 and F40 postdate it and were added here from `spikes/F39-DRAFT.md` and `spikes/F40-DRAFT.md`, so a document that describes this brief as "F1–F38 as condensed 2026-09-08, F39 and F40 read from the corpus" is still accurate. · **Status:** Working condensation, promoted into the repository so that the citations in the acceptance-criteria and success-measure registers resolve. **Not a source of truth** — `spikes/FINDINGS.md`, `spikes/F29-DRAFT.md`..`F40-DRAFT.md` and `spikes/DECISIONS-2026-09-09.md` remain authoritative, and where this brief and they disagree, they win. · **Purpose:** one citable digest so a dozen authoring agents cite the same figures instead of each re-deriving them from a 97 kB ledger. · **Evidence discipline:** every figure below is traceable to a named source file; nothing is rounded.

**Errata (2026-09-11):** the pre-implementation audit of 2026-09-10 (PIA-001…PIA-110; `references/audit/2026-09-10/MASTER.json`) corrected several restated figures. The rows and paragraphs below are the dated record and are not rewritten; each correction is a `**Erratum (2026-09-11, PIA-0NN).**` row or paragraph directly beneath the row it corrects, citing `references/adr/ERRATA.md` (E-NNNN-k) where an accepted record carries the same correction. The spike files still win over this brief and over its errata, per the precedence rule ADR-0092 records (accepted ADRs > the AC register > the SM register > spike files > the runbook; PIA-039).

---


Derived from `spikes/FINDINGS.md`, `spikes/F29-DRAFT.md`..`F40-DRAFT.md`, `spikes/DECISIONS-2026-09-09.md`,
`spikes/METHOD*.md`, `spikes/HANDOFF.md`. Every figure below is traceable to those files. **Do not quote a number
that is not here without reading its source file first.**

## A. Finding ledger (F1–F40; there is no F9 entry, and F30 was reserved for S-H1b/S-WT and never written. F1–F38 were condensed 2026-09-08; the **F39 row was appended 2026-09-09** from `spikes/F39-DRAFT.md` and the **F40 row 2026-09-10** from `spikes/F40-DRAFT.md`, after the areas were refuted against this brief — no area file was refuted against the F39 or F40 rows)

**Erratum (2026-09-11, PIA-019).** F30 was not "never written": it was **drafted 2026-09-06** (`spikes/FINDINGS-DRAFT-F30.md`, untracked in a linked worktree at `8b64201`, an ancestor of main; the run prompt `run-s-h1b-24h-read.md` :19/:98 instructed it be left uncommitted for review) and **promoted 2026-09-11 to `spikes/F30-DRAFT.md`**, the drafts' naming convention. Read the heading as "F1–F40; no F9 entry; F30 drafted 2026-09-06, promoted 2026-09-11".

| id | claim | number | status |
|---|---|---|---|
| F1 | Plan headroom is readable via an undocumented top-level `rate_limit_event`; `result.rate_limit` is null | `unifiedWindows.{five_hour,seven_day}.utilization`; **no Opus/Sonnet family sub-limit key** — the binding limit is invisible | measured |
| ↳ F1 | **Erratum (2026-09-11, PIA-065).** The row omits three keys F1 records: `isUsingOverage`, `overageStatus`, `overageDisabledReason` (`spikes/FINDINGS.md` F1 lines 42–48; every one of the 20 `spikes/results/e0-headroom.jsonl` and 60 `s-var.jsonl` records carries `is_using_overage: false, overage_status: "rejected"`). The AC register's "fabricated evidence" cut of the `isUsingOverage` citation is withdrawn — the citation was real and absent from this brief, not from the corpus; what is unrecorded is a **true-valued** instance only. Separately, the Agent SDK TypeScript reference now types the event (`rate_limit_event` with `rate_limit_info` status / resetsAt / utilization, fetched 2026-09-10) but not `unifiedWindows`, so the fields the instrument reads stay undocumented. ADR-0019's 0/156 was never in this brief (E-0019-1: proposal-only, not re-verified). | | |
| F2 | Settings-based hooks DO fire in `-p`, including user-level ones | 2 hook events per dispatch on a bare fixture | measured |
| F3 | `usage` uses plain cache naming and exposes no TTL split | `ephemeral_1h_*`/`ephemeral_5m_*` absent; trivial haiku dispatch $0.021–0.027 | measured; charter E1's split metric untestable from usage |
| ↳ F3 | **Erratum (2026-09-11, PIA-058).** "`ephemeral_1h_*`/`ephemeral_5m_*` absent" is false as stated: the flat keys are absent, but the split is carried as a **nested `cache_creation` object** (`ephemeral_1h_input_tokens` / `ephemeral_5m_input_tokens`) present on **954/954** run records [measured, audit recount 2026-09-11]. The TTL split is therefore readable from `usage`, which is what the evidence-augmentation pass's recommendation 1 already assumes; S-TTL1 stands as registered. | | |
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
| ↳ F14 | **Erratum (2026-09-11, PIA-077).** The "~1.9M" token figure does not reproduce from the batch records: **~1.0M** (854k cache-read, 157k cache-creation) across the 40 dispatches. The seven_day reading 0.100 → 0.100 and the STRUCK gate stand. | | |
| F15 | E2: reasoning effort IS applied per dispatch | thinking tokens low **0.0** / high **4.8** / xhigh **221.0**, complete separation; output 121/130/360 | measured; **low vs high did NOT separate** |
| F16 | E1: a commit costs 31% of the prefix; `--system-prompt` costs nothing | steady share: no-commit 1.000 · commit-each **0.689** · +`--exclude-dynamic` 0.846 · +`--system-prompt` **1.000** · worktree-each 0.685 | measured, 50 dispatches, within-arm range ≤0.001 |
| F17 | The E3 fixture was defeatable: the reviewer restored the fix from the object store | 42 turns, 22,853 output, 526 s, returned `approved` | measured as an instrument defect |
| F18 | Half of all human gates outlive the cache TTL | gate→verdict n=174 median **10.5 min**, 45% >1 h; stage→verdict n=191 median 51.6 min, p90 ~9 h, **max 226 h (9.4 days)**, 48% >1 h | measured, zero model cost; one operator |
| F19 | H2/H3: suspension is lossless; the world moves rarely and predictably | **174/174 lossless**; drift 12/174 = **7%**; ≤1h 3/95 = 3% vs >1h 9/79 = **11% (3.6×)** | measured; **limit: the FSM was never replayed**, so "same next action" is unproven |
| F20 | An agent card's `tools` allowlist silently disables `--json-schema` | `structured_output` absent, exit 0, `subtype: success`; adding `StructuredOutput` fixes it | measured; the mechanism behind a review consumer that fails open |
| F21 | E3 arm (b) complete | S-SILENT **23/23**, CLEAN FP 2/23, MCC **+0.917**, spend **$56.47**; five_hour +0.03 S-SILENT / ~+0.077 CLEAN | measured; FP provisional; the +0.077 figure is **superseded by F29's 0.0098, 7.8× lower** |
| ↳ F21 | **Erratum (2026-09-11, PIA-075; E-0001-1).** The row carries none of F21's platform-error shape, which exists in the tree: seven `spikes/results/e3.jsonl` records (six 529, one 500 — slugs fix-doctor-git-config-repair, issue-122 §2, issue-130, issue-131, issue-72 for 529; issue-82 for 500) are `subtype: "success"`, `is_error: true`, `step.exit_code` 1 — **exit 1 + is_error**, not the exit-0 cell the registers were written around. F21's count 7 + 1 = 8 disagrees with the file (7), and the 500 record's `void_reason` still reads "schema". The `is_error` / `api_error_status` fixture the registers said "may not exist" exists. | | |
| F22 | Background jobs die; a detached process does not | ~12 kills vs one **12,000 s** survivor under `Start-Process`; `issue-81` killed 3× as background, **116 s** in foreground | measured (operational); cause still unestablished |
| F23 | E3 RESULT: the kill criterion does not fire | detection **23/23 vs 23/23**; CLEAN FP 4/23 vs 2/23; MCC +0.839 vs +0.917; **McNemar p=0.625**; cost **$97.39 vs $56.47 = 1.72×**; `parent_inspected` **0/46** | measured; **primary metric saturated — no discriminating power** |
| ↳ F23 | **Erratum (2026-09-11, PIA-073; E-0001-1).** McNemar p = 0.625 is computed on the CLEAN false-positive discordant pairs (3 vs 1), not on detection; 23/23 vs 23/23 has no discordant pairs and admits no test. Where arm (a)'s MCC is restated elsewhere at the higher figure ADR-0080 and ADR-0074 printed, it is **+0.957** (tp 23 / fn 0 / fp 1 / tn 22; E-0074-3) — this row's +0.839 / +0.917 are the brief's own figures and stand. | | |
| F24 | ADJUDICATION: all six E3 rejections are factually TRUE | 6/6 confirmed, zero model cost; if all blocking both arms → FP 0/23, MCC +1.000 | measured for factual truth only; **severity unadjudicated (Ryan's)**; CLEAN ground truth wrong for ≥5/23 = 22% |
| F25 | ROOT CAUSE: Red→Green is evidenced at TEST granularity, but an AC is a claim about ONE ASSERTION | AC-015 probe: remove step 7 → RED; invert the ordering → **36/36 GREEN** | measured mechanism + a 4-step remediation package, proposed not adopted |
| F26 | E3-H: 20/20 across two harder tiers; the prediction was falsified twice; the line is closed | S-SUBTLE sighted 5/5 and 5/5, blind 5/5 and 5/5; spend $36.18; blind +18% median wall | measured |
| F27 | E0 VARIANCE: detection is stable, the verdict is not | detection **0/7 items flaky over 21 draws**; verdict **2/4 items flaky** | measured, n=2–3; a "stable" label here tolerates a true flip rate up to ~30% |
| ↳ F27 | **Erratum (2026-09-11, PIA-073).** "Up to ~30%" → a "stable" label at this n is **compatible with a true flip rate up to ~56% (n=3) / ~66% (n=2), Wilson 95%**. | | |
| F28 | H1/H4: a human gate costs a bounded ~18.6k tokens however long it lasts; the digest carries what is used | 0.9 min → share 1.000/creation 0; **65.2 min → 18,603**; **49.7 h → 18,603** (identical to the token); `--system-prompt` at 66.8 min → share **0.000**, creation **29,680**; H4 Jaccard FULL 1.000 = CONTROL 1.000, effect **+0.000** | measured, with a published correction: the flatness was **shared-prefix subsidy, not durability** |
| F29 | S-VAR: not one defect class is stable-reject; the verdict is a weighted coin, but every rejection is on target | 60 draws, 0 void, **$176.96**, 16.2 h; class means Reproduced **0.70** > MutationSurvived 0.37 > NarrowedPin 0.10 > control **0.00**; **19/19 rejections on-target**; per-draw five_hour **0.0098** | measured |
| ↳ F29 | **Erratum (2026-09-11, PIA-077).** The per-draw 0.0098 is the **four-row pooled** figure (60 draws); the three-row pooling is 56 draws / 0.0102. Downstream restatements use 60 / 0.0098. | | |
| F30 | **UNASSIGNED / NEVER WRITTEN.** Reserved for S-H1b/S-WT | S-H1b: 1.5 min share 1.000; **66.1 min share 0.000, creation 29,619**; 24 h read **not taken**. S-WT: w1 0.807, **w2–w10 all 1.000** | the numbers exist only in the design-recommendation and the proposal |
| ↳ F30 | **Erratum (2026-09-11, PIA-019).** Not "UNASSIGNED / NEVER WRITTEN": **drafted 2026-09-06** (16 haiku dispatches, 2026-09-06 18:45–20:02Z, pin `47faec9f`, claude 2.1.263, $0.19) and **promoted 2026-09-11 to `spikes/F30-DRAFT.md`**. The numbers do not "exist only in the design-recommendation and the proposal": the raw records are tracked at `spikes/results/s-wt.jsonl`, `spikes/results/s-h1b-1m.jsonl`, `s-h1b-65m.jsonl` and `s-h1b-24h.jsonl` (mark only; the 24 h read is still not taken — and the 2026-09-06 mark is spent, so a fresh mark is needed for it, PIA-109). | | |
| F31 | S-TYPED: the typed contract costs no detection and conforms perfectly | 51 valid, **$112.25**; S-SILENT 23/23, S-SUBTLE 5/5; **schema conformance 52/52 = 100%**; `repro.command` fill 147/147; CLEAN blocks strict 12/23, lenient 9/23; adjudication **14/14 confirmed**; **verified false positives 0/23**; cost **+74%** | measured; the severity call ("narrowed-pin advises") is Ryan's |
| ↳ F31 | **Erratum (2026-09-11, PIA-095; E-0074-2; E-0082-2).** The block counts (strict 12/23, lenient 9/23) and the sink seeds were produced by `policy_verdict` (`spikes/runner/src/e3.rs:254-256`) calling `policy.blocks(evidence)` on the **model-emitted** label, so they are measured under the model-label policy; the Rust-computed class is unmeasured until **S-CLASS-REPLAY** runs (ADR-0096 decision 3). The "≥ 13/23" CLEAN ground-truth figure restated in §E is F31's (F31-DRAFT :91), raising F24's ≥ 5/23 — never F24's. Of the four escapes, **two** survived fully green suites (issue-70 AC-001 1,890/1,890; issue-83 AC-002 1,907/1,907) and **two** (issue-131) survived with zero new failures against an already-red 54-failure baseline. | | |
| F32 | P9: the schema authors at a median 56 lines | median **56.5** non-blank lines (kill is >150); lint 13 real / 34 false; largest fragment **5 of 10** — exactly at the threshold | measured, zero model dispatches |
| F33 | P9b: a second author lands at a median 73 lines; the lint accepted 5 guesses silently | median **73** (+29%); 27 ambiguities, 20 new | measured; falsifies P9's "amendments will fall" at n=3 |
| F34 | P6 authoring: a third author wrote 90 activities at a median 53 lines | median **53**, sum 5,059; **0.11 first-run lint errors/file** (author read the lint first) vs P9b 1.3, P9 4.7; quality block in **42/90**; amendments 26–43 | measured; PC5 clears by **2.8×** |
| F35 | P6 lattice: unsatisfiability 23.5% → 11.4% → **0.3%** after decisions 1–10 | dead activities 15/101 → **6/102**; rank ties **0**; hard cycles **0**; lattice runtime 27 s | measured; the pre-registered reading fires the kill twice, the amended lattice clears all three criteria |
| ↳ F35 | **Erratum (2026-09-11, PIA-077).** The dead-activities figure reads "**6 of 102 after decisions 1–10 and Ryan's R3/R7** (§1.6 run9)". | | |
| F36 | **P1: the LLM-emitted context vector is not stable enough to drive deterministic selection, and the prescribed remedy does not repair it** | 400 dispatches, 0 void, $21.37; **modal `R` identical on 5/20, 3/20, 2/20, 2/20** vs an 80% line; alpha drift **−0.168 on byte-identical anchors**; `external_dep` became **constant** (200/200 false); `unknown` rates novelty 46.0%, size 39.5%, blast_radius 31.0%, criticality 27.0% | measured for the unlabelled half; **arm D precision/recall NOT COMPUTED at F36** — awaits Ryan's blind panel (F36 §0, "waits for the panel"); **scored later by F39** (panel-adjudicated under METHOD-P amendment 9, never human-panel; 140 truth cells at pin `47faec9f`) |
| ↳ F36 | **Erratum (2026-09-11, PIA-077; E-0079-1; E-0089-2).** The 2026-09-09 run scored **136** of the 140 truth cells (the panel-file key `resolve-open-questions` did not match the engine's `resolve-open-question`); after the key fix and `score-arm-d-panel.py` failing closed on a truth cell with no engine record, the 2026-09-11 re-run scores all **140** and `auth` reads 0/12 [0.00, 0.24] (F39 Correction). F36's cost restated elsewhere as $9.70 is **$10.71** (haiku $4.91 + sonnet $5.79, from the two log lines); this row's $21.37 is arm L's and stands. | | |
| F37 | **P7: the `adversary → policy` block costs 2.64× and changes the root cause on 1 item in 10; its policy check was false on 33 of 33** | 61 runs, 297 records, 0 void, **$131.62**; block $2.69 vs $1.02/run; adversary $1.68 / 676 s; **188 findings, 78% required running something**; deviation offered **71 times, used 0** | measured, **but the primary criterion is powerless at n=3** — every Wilson interval overlaps by construction |
| F38 | **P8 PARTIAL: mid-graph resume is lossless across a real process kill and costs less than a stage gate** | 3 of a pre-registered 27+3, $3.01; **0 changed, 0 lost, digests identical** across `taskkill /T /F`; resume **0.45× / 0.55× / 0.78×** of ~18.6k | PARTIAL; 65-min and 24-h arms and all 3 controls **unrun**; the second-arrival question is **not answerable** (arrivals forced, mark minted by the driver that implements the semantics) |
| ↳ F38 | **Erratum (2026-09-11, PIA-080; E-0087-1; ADR-0106).** The "costs-less-than-a-stage-gate" clause is withdrawn: the 0.45 / 0.55 / 0.78 ratios divide P8 **output tokens** by F28's **cache-creation** tokens — unlike counters. On cache-creation tokens the resumed dispatches created 84,960 / 49,468 / 46,961 = **4.57× / 2.66× / 2.52×** of 18,603, so on the like counter METHOD-P §5.4's > 1.5× kill fires against F28, subject to the cross-fixture caveat. Cost ordering is **UNMEASURED** (ADR-0106); the losslessness result (0 changed, 0 lost, digests identical across `taskkill /T /F`) is untouched. | | |
| F39 | **P1 arm D RESULT: no derived facet clears METHOD-P §1.4's 0.95 bar, and `auth` scores 0.00 over 11 positives** — every positive the lexical rule emits is false, because `auth` is a substring of *authored*, `oauth` of *coauthor*, and `token` matches a design token | 140 truth cells at pin `47faec9f`, **$0**; union source, 20 items, Wilson 95%: `ui` **1.00 [0.65, 1.00]** recall 0.78 · `toolchain` 0.73 [0.48, 0.89] recall 0.79 · `change` 0.72 [0.54, 0.85] recall 0.84 · `binary_change` 0.60 [0.23, 0.88] recall 1.00 · `flag_exists` 0.00 [0.00, 0.66] · **`auth` 0.00 [0.00, 0.26]**, 11 positives and 11 false · `data_migration` **not measurable** (0 positives on 20 items); `commits` is **never less precise than `docs`** and strictly better on `toolchain` (1.00 vs 0.50) and `change` (0.80 vs 0.50), while on `binary_change` `docs` emits **no positive at all** (precision undefined, recall 0.00, 3 missed) against `commits` 0.60 at recall 1.00 and on `ui` the two **tie at 1.00** and only recall separates them (0.67 vs 0.22) — and **`union` is worse than `commits` alone** on `toolchain` (0.73 vs 1.00) and `change` (0.72 vs 0.80) (F39 §3) | measured, **panel-adjudicated** — never human-panel — over 60 deterministic-oracle, 48 unanimous-panel and **32 human-adjudicated** cells (Ryan, blind; **15 of 32 differ from the script**); **the ≥0.95 clause is not reachable at n=20 for any facet however good** (`ui` is 7/7 and its lower bound is 0.65; ~60 positives are needed), and 4 items that changed no code produced **16 confident booleans** the panel marked `unknown` |
| ↳ F39 | **Erratum (2026-09-11, PIA-073, PIA-077; E-0089-2).** The 2026-09-09 scoring reached **136** of the 140 truth cells (panel-file key mismatch); the 2026-09-11 re-run with the key fix and a fail-closed scorer scores all **140**, and `auth` reads **0/12 [0.00, 0.24]**. "~60 … are needed" → **73** all-correct positives for a Wilson lower bound > 0.95 at the registered two-sided z = 1.96 (92 with continuity correction; 72 Clopper–Pearson). The other facet figures stand. | | |
| F40 | **P2 PERTURBATION RESULT: the corpus carries no staleness to detect — 0 of 41 rows are "should re-run" — so criterion 1's recall half and criterion 2 have no denominator, while criterion 1's precision half fires by construction; and on the rows both scopes score, the *fine* `derived_from` is three times worse than the coarse one**, because the reconstruction picks up the card the CLI rewrites on every stage advance (DECISIONS-2026-09-09 **#19** doubled the case count 6 → 12 by adding the card as a perturbation candidate, and cards are exactly what the CLI churns) | 41 `(case, downstream)` rows over 12 perturbation cases at pin `47faec9f`, **$0**; **11 of the 12 cases change nothing but YAML frontmatter** — `stage`, `status`, `updated`, `gate_open`, which the CLI rewrites on every advance — and the 12th (P01, the only non-card perturbation, on `discover.md`) changes **one body line** — a single rewritten line, which the unified diff shows as a `-`/`+` pair of wording; the labeller wrote *"only frontmatter properties (process/stage status), no new/updated facts of the system"* against **36 of the 41 rows** (35 the phrase exactly, one a shorter variant of it; four rows carry no reason at all and one is P01's) before any of this was computed. Wilson 95%: coarse **specificity 0.73 [0.58, 0.84]** (0 tp / 11 fp / 0 fn / 30 tn, 41 of 41 scored) vs fine **0.55 [0.38, 0.70]** (0 tp / 15 fp / 0 fn / 18 tn, 33 of 41 scored, **8 abstentions = 19.5% [0.10, 0.34]** — a reconstructed scope of `unknown`, excluded from scoring and never counted as a miss, DECISIONS #16). **Precision 0.00** (coarse [0.00, 0.26], fine [0.00, 0.20]) is a property of the label set, **not an independent reading of the engine**; **recall is not computable**. The two specificity intervals **overlap — not separated at this n**; what separates them is the paired comparison on the **33 rows both scopes scored**: coarse **5** `Stale`, fine **15**, **10 rows flip `Present` → `Stale` going coarse to fine and none flips the other way**. By downstream: `spec.md` coarse **11 Stale** / fine 5 Stale + 6 excluded · `implementation-report.md` and `report.md` each coarse 12 Present / fine **5 Stale**, 6 Present, 1 excluded · `_review.md` 6 Present under both — and the declared map connects neither report to the card at all | measured, **blind and timestamp-verified** — `labelled_at` 2026-09-10T15:58:08Z precedes the engine's earliest record at 16:01:12Z, checked by the scorer's own guard, and P2 is unaffected by METHOD-P amendment 8, so this is the **only arm in the programme whose blindness is checked rather than attested** (arm D is panel-adjudicated and never invoked amendment 8's attestation route; do not blur the two); **the perturbation half only** — the advisory half (A01–A06, criterion 3) is **unrun**, and nothing here settles `stale_policy`, because criterion 1's precision half fires while the remedy it prescribes is contraindicated by the same measurement |

## B. Kill criteria — outcome and threshold

**Filing rule (2026-09-11, PIA-106; DECISIONS-2026-09-11 #10; ADR-0111).** One rule, applied uniformly: a criterion is listed under *Fired* when it fires on its pre-registered reading, with a **"fires while powerless"** note where the registered n could not decide it. Under that rule **P7 §4.4(1)** — F37 :15, "KILL FIRES on the pre-registered reading", interval overlap on 10/10 items — is filed under *Fired* with the note "fires while powerless: at n=3 no two Wilson intervals can be disjoint ([0.439, 1.000] vs [0.000, 0.561]), so criterion 1 had zero power" (PIA-073), beside P1's α rule, which already carries the note. Its line under *Not answerable / powerless* below is the power statement, not a second filing, and S-P7B's re-run must change the criterion, not only n (ADR-0111). Counting what fired under this rule adds P7 §4.4(1) to the *Fired* list below; the lists themselves are the 2026-09-08 record and are not rewritten. Also under this rule: P8 §5.4(2)'s resume-cost criterion moves from *Did not fire* to **not measurable on the registered unit** — see the F38 erratum (E-0087-1).

**Fired.** P1 §1.4 modal-R (<80% identical) — **fires 4/4**, the one to act on. P1 §1.4 alpha<0.6 — fires twice but
**the predicate is not decidable on this corpus**. P1 §1.4's precision<0.95 clause — **fires on five facets**
(`auth` 0.00, `flag_exists` 0.00, `binary_change` 0.60, `change` 0.72, `toolchain` 0.73), each of which therefore stays
LLM-emitted (F39). P6 §3.4 unsatisfiability (>5%) — fires on the pre-registered reading (23.5%, 11.4%), clears
post-decision (0.3%). P6 dead activities (>20%) — fires by 0.8 points under the 8 pre-registered packs (20.8%), clears
under 12 (14.9%) and post-decision (5.9%). S-VAR §1.5 branch 2 and **branch 3 (architecture-changing: a reproduced
regression approved in ≥2 of 10 — measured 3 of 10)**. S-TYPED §2.6 branch 3, whose own remedy was executed (14/14
confirmed → lenient default). H1's ~25k threshold sits INSIDE the measured 18.6k–29.7k range.
**P2 §2.4 criterion 1's precision half** — fires on the coarse map at **0.00 [0.00, 0.26]** over 11 emitted positives, but
fires **by construction**: 0 of the 41 labelled rows are positive, so every emission is a false positive whatever the engine
does. Two things must travel with it. The recall half of the same criterion **cannot be evaluated at all** (below), and the
remedy the criterion prescribes — fine scope mandatory for `L`-class, `stale_policy = gate` — is **contraindicated by the
same measurement**: the fine scope is the worse of the two on this corpus (F40, §C.14). **P2 therefore does not settle
`stale_policy`**; the design consequence is open and needs a decision, not a default.

**Did not fire.** E3 §9 (23/23 vs 23/23, p=0.625). E3-H. E1 H1/kill. E2. S0.1. H2 (174/174). H4 (digest sufficient).
S-VAR branch 4 (control 0/10). S-TYPED branches 1, 2, 4 (conformance 100%). P7 §4.4(4) fixture-too-easy (7/10).
P8 §5.4(1) lost output (0 lost) and §5.4(2) resume cost (0.45–0.78×, kill was >1.5×). P9 §4 verbosity, three times.

**Not answerable / powerless.** E0 arm-sizing (E4–E6 never ran). E0 cache-cost gate (STRUCK at 0.01 resolution).
P7 §4.4(1) — establishes the criterion's **power, not its verdict**, at n=3. P7 §4.4(2) — n=1, arm dropped.
P7 §4.4(3) — **empty denominator**, 0 deviation requests in 71 offers. P8 §5.4(3) — **circular by construction**.
P2 §2.4 criterion 1's **recall half** and criterion 2 in full — **empty positive class**: 0 of the 41 labelled rows are
"should re-run", so recall has no denominator and is reported **NOT ANSWERABLE**, never 0.00, never 1.00, never n/a (F40;
the precision half of criterion 1 fires and is listed under *Fired* above). P2 §2.4 criterion 3 — **still unrun**: the
advisory half (A01–A06) is labelled *after* the engine in its own file (METHOD-P §2.3) and DECISIONS-2026-09-09 **#15**'s
`human_authored` remap must be settled first, so "P2 is entirely unrun" and "P2 is complete" are both false — the
perturbation half is scored, the advisory half is not (F40).
P1 §1.4's precision ≥0.95 clause — **not reachable at n=20 for any facet however good the method**: `ui` is 7/7 and its
lower bound is 0.65, and roughly 60 positives are needed for a lower bound above 0.95 (F39). The same clause on
`data_migration` — **empty denominator**, 0 positives on 20 items across all three sources, so precision is undefined and a
perfect method scores identically (F39). **That makes FOUR pre-registered criteria the programme's own
registered sample size cannot decide** — P1's alpha rule (F36; listed under *Fired* above, because it fires while being
undecidable), P7 §4.4(1) at n=3 (F37), METHOD-P §1.4's 0.95 bar (F39) and now P2 §2.4 criteria 1 and 2 (F40) — and the
pattern, not the individual row, is the finding: **four is a pattern in how this programme writes pre-registrations, not
four coincidences — each of the four fixed a threshold without checking that the registered corpus could produce the
denominator the threshold needs.** The defect is in the registration. It is reported in F39's and F40's Limits, not patched
into METHOD-P. (`data_migration`'s empty denominator in F39 and P2's missing recall denominator in F40 are the same shape;
their numbers are not interchangeable and must never be mixed.)
S-BIN and S-H1c — registered and **unrun**.

**Erratum (2026-09-11; E-0089-2; PIA-073).** The "roughly 60" positive count above → **73** (92 with continuity correction; 72 Clopper–Pearson); F37's criterion 1 had zero power at n=3 (no two Wilson intervals can be disjoint), which is the same class.

## C. Architecture-bearing implications

1. **Cache, split rule.** `--system-prompt` holds 1.000 across commits (F16) and across worktrees w2–w10 (S-WT), but
   **forfeits the account-wide subsidy across TIME**: 29,619–29,680 created at ~66 min, share 0.000 (F28 correction,
   S-H1b). The default prompt costs a flat 18,603 at both 65 min and 49.7 h. The two mitigations point in OPPOSITE
   directions. Forbids inferring "held across commits" implies "helps across time" — explicitly falsified.
   **Erratum (2026-09-11, PIA-058, PIA-103).** "Account-wide subsidy" → **shared-prefix subsidy**: the cache is scoped per machine and per directory (code.claude.com prompt-caching, fetched 2026-09-10; design §7.2), there is no account-wide shared head, and what F28 measured is the default prompt's static head being re-read on this machine. The split rule and the falsified inference stand.
2. **Gate cost.** Duration is free; frequency is marginally costly. Mid-graph gates are CHEAPER than stage gates
   (F38, 0.45–0.78×). Placement is a value decision.
   **Erratum (2026-09-11, PIA-080; ADR-0106).** "Mid-graph gates are CHEAPER than stage gates" is withdrawn — the 0.45–0.78 ratios are output tokens over cache-creation tokens (E-0087-1); on like counters 4.57 / 2.66 / 2.52; cost ordering **UNMEASURED**. Placement stays a value decision, not a cost one.
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
   **Erratum (2026-09-11, PIA-095; E-0082-2).** "Four adjudicated defects survived fully green suites" → four escaped the deterministic gate, **two** through fully green suites (issue-70 1,890/1,890; issue-83 1,907/1,907) and **two** (issue-131) with zero new failures against an already-red 54-failure baseline. The mechanism holds on two as on four; the block counts are model-label-policy figures until S-CLASS-REPLAY (E-0074-2).
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
13. **The derived half is not a rescue (F39).** Where F36 measures the LLM-emitted vector, arm D measures the derived one,
    and it fails differently rather than better: the lexical deriver carries **no signal** on `auth` — 11 positives, 11
    false, upper bound 0.26, because the pattern collides with the vocabulary of the instrument under test (*authored*,
    *coauthor*, design *tokens*) — and no facet clears METHOD-P §1.4's 0.95 bar. The registry compounds it: `facets.toml`
    gives the boolean facets the domain `[true, false]`, so the **4 items that changed no code at all** became **16
    confident booleans** on cells the panel returned `unknown` for. Forbids treating a lexical scan of free text as the
    "computed fact" PC8 rule 2 demands of a `require`-row input, and forbids a derived boolean resolving an absence of
    evidence to `false` — absence is `null`, routed to AC-079's `gate_opened { reason: unknown_facet }`, as ADR-0067's
    `recent_deploy` already does. ADR-0089 records both, and **does not** decide the source asymmetry — `commits` is never
    less precise than `docs` (strictly better on `toolchain` 1.00 vs 0.50 and `change` 0.80 vs 0.50; on `binary_change`
    `docs` emits no positive at all; on `ui` the two **tie** at 1.00 and only recall separates them, 0.67 vs 0.22), and
    `union` is worse than `commits` alone on `toolchain` and `change` (F39 §3; ADR-0089 "What this record does not decide",
    whose own summary of the same rows is looser) — which is an arm of S-FACET.
14. **A derivation inferred from a trace the instrument itself writes into is unsound, and the refinement imports the noise
    (F40).** P2's fine scope reconstructs `derived_from` as "which upstream changed before this artifact was last written"
    (METHOD-P §2.2), so it fires `Stale` on `implementation-report.md` and `report.md`, which **the declared map does not
    connect to the card at all** — not because the card feeds them, but because the CLI stamps `stage`, `status`, `updated`
    and `gate_open` on every advance and the card is therefore nearly always the most recent prior write. Measured: on the
    **33 rows both scopes scored**, coarse marks **5** `Stale` and fine **15**, with **10 rows flipping `Present` → `Stale`
    and none flipping back**; aggregate specificity is 0.73 [0.58, 0.84] coarse against 0.55 [0.38, 0.70] fine, and those
    intervals **overlap**, so the aggregate is **not separated at this n** and it is the paired comparison that separates
    them. This is **F39's failure class at the artifact level**: F39's `auth` regex collided with the vocabulary of a tool
    whose documents are *authored*, F40's `derived_from` reconstruction collides with the writes of a tool that stamps
    `stage` and `updated`, and in both the refinement meant to help is worse than the cruder thing it refines (F39 §3,
    `union` worse than `commits` alone on `toolchain` and `change`; §C.13). Forbids deriving a dependency edge from write
    order, recency or frequency in a history the tool under test writes into — a `derived_from` that matters must be
    **recorded**, not reconstructed (the old redkiln never stored it, F40 Limits 3) — and forbids adopting a finer scope on
    the argument that it is finer: a scope change is a measured claim, and here the finer scope is the worse one. Forbids
    equally any reading in which P2 settles `stale_policy`: criterion 1's precision half fires, its prescribed remedy (fine
    scope mandatory for `L`-class) is contraindicated by the same measurement, and the consequence is therefore **open**.
    What would decide it is a corpus carrying real content perturbations — **no item here carries a substantive upstream
    change**, so nothing in F40 says whether either scope catches real staleness.

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

Awaiting Ryan: **P2's advisory half only** — the perturbation half is **scored** (F40: 41 rows at pin `47faec9f`, $0,
blind and **timestamp-verified**), but A01–A06 are labelled after the engine in their own file (METHOD-P §2.3), criterion 3
is **unrun**, and DECISIONS-2026-09-09 #15's `human_authored` remap must be settled before it measures anything stable.
"P2 entirely unrun" is now false and "P2 complete" would also be false; every sentence must say which half ·
the E3 severity adjudication (facts settled 6/6, severity is not, so both FP rates and both MCCs are provisional) ·
the `bisect` ceiling question ·
the DECISIONS-2026-09-09 "reverse or confirm first" table (#15 `human_authored` remap changes what the advisory
fraction measures; #7 the mandatory arm prices an unreachable configuration; #5 20 items not 30).

Discharged 2026-09-09, not deleted: **P1 arm D precision/recall** is scored (F39) — 140 truth cells at pin `47faec9f`,
**panel-adjudicated**, never human-panel, with 32 of those cells adjudicated by Ryan blind and **15 of 32 differing from the
script**. That discharges the fourth of AC-071's five clauses (identity ≥0.80 · n ≥20 items · ≥10 draws per item ·
**arm-D panel scored** · corpus pinned). **The block still stands, on the first clause**: F36 measured modal required-set
identity at **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line, so ADR-0079 stays `proposed`. F39 measures a *derived*
context vector and 0079 asks about an *LLM-emitted* one: it narrows 0079's options without deciding among them, and no
facet is removed from the classification record, because none clears §1.4's bar and the bar is unreachable at this n.

**Erratum (2026-09-11; E-0089-2).** The 2026-09-09 scoring covered 136 of the 140 truth cells; the 2026-09-11 re-run scores all **140**; `auth` 0/12 [0.00, 0.24]. The clause structure of AC-071 and the standing block are unchanged.

Pre-registrations found broken: P1's alpha rule (not decidable) · **METHOD-P §1.4's precision ≥0.95 bar** (not reachable
at n=20 for any facet — `ui` is 7/7 with a lower bound of 0.65, and ~60 positives are needed; F39, reported in its
Limits) · **P2 §2.4 criteria 1 and 2** (empty positive class — 0 of 41 labelled rows are "should re-run", so recall has no
denominator and criterion 2 cannot fire either way; criterion 1's precision half fires **by construction** and its own
prescribed remedy is contraindicated by the measurement; F40, reported in its Limits) · P7 criteria 1–3
(powerless / n=1 / empty denominator) · P8 §5.2's fixture design ("remove the repro steps so
the gate fires" does not work against an agent with Read+Bash) · P8 §5.4(3) (circular) · METHOD-H §6 (vacuous, replaced
before data) · S0.2's registered primary (uncomputable at n=1) · charter E1's TTL-split metric (not in `usage`) ·
charter E4's oracle (empty ledger join).

**Erratum (2026-09-11; E-0089-2; PIA-073; ADR-0111).** The "~60" positive count → **73**. F37: at n=3 no two Wilson intervals can be disjoint — criterion 1 had zero power, and S-P7B's re-run must change the criterion, not only n. P8 §5.4(2)'s 0.45–0.78 ratios are not measurable on the registered unit (E-0087-1).

Reclassified, not absent — **spike P5**: replay identity + conformance was **reclassified** out of the pre-design P-series into a Phase 1 acceptance test because it needs the kernel's event log and step-level memo keys (`references/2026-09-07-process-catalog-proposal.md` §6 row P5, §6.1). Its criterion is live and belongs to the kernel: 100% replay identity of `dag_assembled` re-derived from logged inputs, violations attributed to pack versus kernel, and the kill — **any non-reproducible DAG means a hidden input, which means the memo key is unsound; fix before anything else.** Recorded here because an earlier condensation omitted it and a downstream refutation pass then struck it as nonexistent.

Unrun: **S-BIN** (gates Phase 1 for `ChangeSummary` type design) · **S-H1c** (the append route; D16 is conditional on it) ·
**S-FACET** (newly registered by ADR-0089 under ADR-0012: does a declared boundary map plus git metadata reach precision
≥0.95 against arm D's adjudicated labels, what does `surfaces.toml` cost to seed and to keep, and which source the engine
trusts — named consumer `facets.toml`'s `source`/`deriver` fields; until it reports, `surfaces` is unscored and may key no
`require` row) · the h1b 24 h read · **P8's 65-min and 24-h arms and all 3 controls** · P7's mandatory arm (dropped at
n=1) · E3 arm (c) · E4/E5/E6 · F28's `nocommit` arm (closed unmeasured).

**Erratum (2026-09-11, PIA-084; DECISIONS-2026-09-11 #12; ADR-0093 decision 3).** S-FACET as registered could not decide its own criterion on this corpus: `auth` has **0 true positives** in the reused labels (NOT ANSWERABLE for precision — every emission is a false positive whatever the method), `ui` has 9 (9/9 gives a Wilson lower bound of 0.70 — powerless against 0.95), and `surfaces` has **no label column** (NOT MEASURABLE). A dated pre-dispatch amendment under change-classification §6 fixes the estimator (the Wilson lower bound), states the positives per facet, and re-registers the kill on a criterion 20 items can decide — arm 3 vs arm 1 false-positive count, paired per item; the ≥ 0.95 bar stays recorded as powerless, never re-thresholded. Its gating scope is the pack rows keyed on scored derived facets, not Phase-1 entry. Also: **S-WT-REC** is discharged (METHOD-S 4b.2 pre-registered `--system-prompt`; all ten `s-wt.jsonl` records carry it) and **S-TTL1** stands as registered; **S-BIN** is bound to a real project and stays unrun; **S-DEFER**, **S-GIX/c**, **S-SUBTLE-SYM**, **S-CLASS-REPLAY** and **D4-SARIF** are new rows (design §10.5).

Contested: **CLEAN's ground truth is wrong for ≥13 of 23 (57%)** — every false-positive rate measured against it is an
UPPER BOUND. F37's mandatory cost ratio is reported as 3.66× in F37 and 4.4× in the decisions log — unreconciled.
`recent_deploy` is underivable at this pin · `binary_change` is now measured at precision **0.60 [0.23, 0.88]**, recall 1.00
on 5 positives (F39), and remains unregistered in `facets.toml`, so it can still feed no require row · `dynamism` is
pre-registered but has no registry entry. F37's `emit.corpus_sha` holds a per-item checkpoint sha, so the key is
misnamed. Working tree not clean as of 2026-09-09; HEAD was reported as `ab58733` and nothing from the AFK session
was committed.

**Erratum (2026-09-11, PIA-095, PIA-080; E-0067-1).** "≥13 of 23 (57%)" is F31's figure (F24 said ≥5/23). F37's 3.66× vs 4.4× is two denominators, not a contradiction: 3.66× = $9.857 / $2.693 (arm mean, n=30) and 4.4× = $9.857 / $2.222 (issue-77 r1); R-P7RATIO is discharged. `recent_deploy` is NOT MEASURABLE at the pin (`toolchain:deploy_log` absent; emitted `null` under `R-recent-deploy-feed-absent`), not merely "underivable".

**Corrected 2026-09-09:** the AFK session's output **was** committed, at `e3b0bac` ("P1 arm L x2 (400 dispatches, 0 void), P7 complete (61 runs), P8 partial, three rig subcommands"), which is HEAD. `spikes/HANDOFF.md` still says HEAD is `ab58733` with nothing committed; that statement is stale and this document does not inherit it. What actually remains untracked is 17 items: the raw P1/P7/P8 driver logs, `spikes/results/p8-state/`, and the `spikes/runner/target-dev/` and `target-p8/` build directories (`.gitignore` covers `target` but not those two names). The F36/F37/F38 drafts, `spikes/DECISIONS-2026-09-09.md` and every `spikes/results/*.jsonl` are tracked.

**Corrected again 2026-09-09, later the same day: `e3b0bac` is no longer HEAD.** Seven commits have landed on `design/2026-09-08-acceptance-criteria-adrs-architecture` since — `1cf2f0a`, `4519edc`, `20775ce`, `6736e48`, `e642a5b`, `767d87d`, `ce04058` — and `4519edc` carries the design pass, the 88 ADRs and `RUNBOOK.md`, so the untracked list above is now the run noise **only** [measured, `git log --oneline e3b0bac..HEAD` returns 7]. The F39 amendment of 2026-09-09 is in the working tree on top of that and is uncommitted at the time of writing; `RUNBOOK.md` §0.1 carries the current inventory and is the place to read it, because this paragraph has now been wrong twice and a status line in a condensation is the wrong home for a fact that moves.

**Corrected 2026-09-11.** HEAD at the audit of 2026-09-10 was `009a4ec` (0 ` M`, 17 `??` — run noise only); the F39 amendment landed at `a431c62` and the P2 labels at `a627547`. `claude --version` measured **2.1.268** on 2026-09-10 — the programme has straddled 2.1.263 → 2.1.266 → 2.1.268. The errata of 2026-09-11 land on branch `remediation/pia-2026-09-11`; `RUNBOOK.md` §0.1 remains the place to read the inventory (PIA-019).
